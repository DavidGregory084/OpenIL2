pub mod finger;
use anyhow::{anyhow, bail, Context, Result};
use flate2::bufread::*;
use indicatif::{ProgressBar, ProgressStyle};
use memmap::Mmap;
use nom::bytes::complete::*;
use nom::multi::*;
use nom::number::complete::*;
use rayon::prelude::*;
use std::convert::TryInto;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug)]
pub struct SfsFile {
    pub mmap: Mmap,
    pub header: SfsHeader,
    pub toc: Vec<SfsTocItem>,
    pub chunk_boundaries: Vec<(usize, usize)>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct SfsHeader {
    pub magic: u32,
    pub version: u32,
    pub checksum: u32,
    pub toc_count: u32,
    pub header_end: u32,
    pub toc_end: u32,
    pub chunk_table_end: u32,
    pub uncompressed_size: u32,
    pub unknown: u16,
    pub comment: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct SfsTocItem {
    pub fingerprint: i64,
    pub index: u32,
    pub offset: u32,
    pub size: u32,
    pub unknown_1: u32,
    pub attributes: u32,
    pub unknown_2: u32,
}

pub fn parse_header<'a>(input: &'a [u8]) -> nom::IResult<&'a [u8], SfsHeader> {
    let (rem, magic) = le_u32(input)?;
    let (rem, version) = le_u32(rem)?;
    let (rem, checksum) = le_u32(rem)?;
    let (rem, toc_count) = le_u32(rem)?;
    let (rem, header_end) = le_u32(rem)?;
    let (rem, toc_end) = le_u32(rem)?;
    let (rem, chunk_table_end) = le_u32(rem)?;
    let (rem, uncompressed_size) = le_u32(rem)?;
    let (rem, unknown) = le_u16(rem)?;
    let (rem, rest_of_header) = take(222u8)(rem)?;

    let comment = String::from_utf8(rest_of_header.to_vec())
        .unwrap_or("".to_string())
        .trim_end_matches('\0')
        .to_string();

    Ok((
        rem,
        SfsHeader {
            magic,
            version,
            checksum,
            toc_count,
            header_end,
            toc_end,
            chunk_table_end,
            uncompressed_size,
            unknown,
            comment,
        },
    ))
}

pub fn parse_toc_item(input: &[u8]) -> nom::IResult<&[u8], SfsTocItem> {
    let (rem, fingerprint) = le_i64(input)?;
    let (rem, index) = le_u32(rem)?;
    let (rem, offset) = le_u32(rem)?;
    let (rem, size) = le_u32(rem)?;
    let (rem, unknown_1) = le_u32(rem)?;
    let (rem, attributes) = le_u32(rem)?;
    let (rem, unknown_2) = le_u32(rem)?;
    Ok((
        rem,
        SfsTocItem {
            fingerprint,
            index,
            offset,
            size,
            unknown_1,
            attributes,
            unknown_2,
        },
    ))
}

pub fn parse_toc<'a>(
    header: &SfsHeader,
    input: &'a [u8],
) -> nom::IResult<&'a [u8], Vec<SfsTocItem>> {
    let toc_count = header.toc_count as usize;
    many_m_n(toc_count, toc_count, parse_toc_item)(input)
}

pub fn sfs_decrypt(hash: i64, buf: &[u8]) -> Vec<u8> {
    let hash_bytes = hash.to_le_bytes();

    let new_buf = buf
        .iter()
        .enumerate()
        .map(|(idx, byte)| {
            // I think this a method of computing division using bitshifts, see:
            //   https://stackoverflow.com/a/436535
            let idx_64 = idx as u64;
            let hash_idx = (idx_64 * 0x2AAAAAAAB >> 0x21) & 7;
            return byte ^ hash_bytes[hash_idx as usize];
        })
        .collect();

    return new_buf;
}

pub fn sfs_decrypt2(hash: i64, buf: &[u8]) -> Vec<u8> {
    let hash_bytes = hash.to_le_bytes();

    let new_buf = buf
        .iter()
        .enumerate()
        .map(|(idx, byte)| {
            // I think this a method of computing division using bitshifts, see:
            //   https://stackoverflow.com/a/436535
            let idx_64 = idx as u64;
            let hash_idx = (idx_64 * 0x4CCCCCCCD >> 0x22) & 7;
            return byte ^ hash_bytes[hash_idx as usize];
        })
        .collect();

    return new_buf;
}

pub fn read_sfs(path: &Path) -> Result<SfsFile> {
    // Map the file into memory
    let file = File::open(&path)?;
    // This is unsafe because another process could modify the file contents
    // while it's mapped into memory, resulting in UB
    let mmap = unsafe { Mmap::map(&file) }?;

    let file_name = path.file_name().unwrap().to_str().unwrap();

    // Read the header
    let (_, header) = parse_header(&mmap[..]).unwrap();
    let header_slice = &mmap[0..256];

    let header_hash = if header.version == 0xCA {
        finger::bytes(0, file_name.to_lowercase().as_bytes())
    } else {
        finger::bytes(0, header_slice)
    };

    // On version 202 (0xCA) we have to do some additional decryption
    let decrypted_vec = if header.version == 0xCA {
        let decrypted = sfs_decrypt(header_hash, &mmap[8..32]);

        // Decrypt all of the u32 fields from checksum onwards
        let decrypted_vec: Vec<u8> = mmap[0..8]
            .iter()
            .chain(&decrypted)
            .chain(&mmap[32..256])
            .map(|b| *b)
            .collect();

        decrypted_vec
    } else {
        header_slice.to_vec()
    };

    let (_, decrypted_header) = parse_header(&decrypted_vec).unwrap();
    let header_end = decrypted_header.header_end as usize;

    // Check that the file header has the right signature
    let magic_slice = &decrypted_vec[0..4];
    assert_eq!(magic_slice, [0x53, 0x46, 0x53, 0x0]);

    // Check that the checksum is valid
    //
    // We have to zero out the checksum then add the rest of the header up
    //
    let header_sum = &decrypted_vec[0..8]
        .iter()
        .chain(&[0u8, 0u8, 0u8, 0u8])
        .chain(&decrypted_vec[12..header_end])
        .map(|b| *b as u32)
        .sum();

    assert_eq!(decrypted_header.checksum, *header_sum);

    // Read the table of contents
    let toc_end = decrypted_header.toc_end as usize;
    let toc_slice = &mmap[header_end..toc_end];
    let decrypted_toc = sfs_decrypt(header_hash, toc_slice);
    let (_, mut toc) = parse_toc(&decrypted_header, &decrypted_toc[..]).unwrap();

    toc.sort_by(|a, b| a.index.cmp(&b.index));

    let chunk_table_end = {
        // Divide by 32768
        let bytes_to_read = decrypted_header.uncompressed_size >> 15;

        // Check if the value is divisible by 32768
        let bytes_to_read = if (decrypted_header.uncompressed_size & 0x7FFF) != 0 {
            // If not add one to allow for a remainder
            bytes_to_read + 1
        } else {
            bytes_to_read
        };

        let bytes_to_read = (bytes_to_read * 4 + 4) as usize;

        toc_end + bytes_to_read
    };

    let chunk_slice = &mmap[toc_end..chunk_table_end];

    let decrypted_table = sfs_decrypt2(header_hash, chunk_slice);

    let chunk_offsets: Vec<u32> = decrypted_table
        .chunks(4)
        .map(|c| {
            return u32::from_le_bytes(c.try_into().unwrap());
        })
        .collect();

    let chunk_boundaries: Vec<(usize, usize)> = chunk_offsets[1..]
        .iter()
        .scan(chunk_table_end, |last, next| {
            let last_offset = *last;
            let next_offset = *next as usize;
            *last = next_offset;
            return Some((last_offset, next_offset));
        })
        .collect();

    Ok(SfsFile {
        mmap,
        header: decrypted_header,
        toc,
        chunk_boundaries,
    })
}

pub fn decompress_chunk(sfs_file: &SfsFile, boundary: (usize, usize)) -> Vec<u8> {
    let (start, end) = boundary;
    let chunk = &sfs_file.mmap[start..end];

    if chunk.len() == 32768 {
        chunk.to_vec()
    } else if chunk[0] == 1 {
        panic!("Can't do LZSS yet!")
    // let mut decoder = ZlibDecoder::new(&chunk[1..]);
    // let mut buffer = Vec::new();
    // decoder.read_to_end(&mut buffer).expect("Error decoding chunk");
    // buffer
    } else if chunk[1] == 8 {
        let mut decoder = DeflateDecoder::new(&chunk[3..]);
        let mut buffer = Vec::new();
        decoder
            .read_to_end(&mut buffer)
            .expect("Error decoding chunk");
        buffer
    } else {
        println!(
            "Decompressing unknown chunk type {:?}",
            chunk[0..1].to_vec()
        );
        chunk.to_vec()
    }
}

pub fn decrypt_data(
    raw_data: Vec<u8>,
    key_hash: i32,
    key_len_offset: i32,
    key_idx_offset: i32,
) -> Vec<u8> {
    let xor_table = finger::key_table(key_hash, key_len_offset);

    let decrypted_data = raw_data
        .iter()
        .enumerate()
        .map(|(i, b)| {
            let table_idx = (i + key_idx_offset as usize) % xor_table.len();
            let xor_val = xor_table[table_idx];
            let result = xor_val ^ b;
            return result;
        })
        .collect();

    decrypted_data
}

pub fn unpack_from_sfs_by_fingerprint(
    sfs_file: &SfsFile,
    decompressed: &Vec<u8>,
    fingerprint: i64,
) -> Result<Vec<u8>> {
    let toc_item = sfs_file
        .toc
        .iter()
        .find(|item| {
            return item.fingerprint == fingerprint;
        })
        .context("Couldn't find a matching entry in the SFS file")?;

    let start_offset = toc_item.offset as usize;
    let end_offset = (toc_item.offset + toc_item.size) as usize;
    let data = &decompressed[start_offset..end_offset];

    Ok(data.to_vec())
}

pub fn unpack_from_sfs_by_path(
    sfs_file: &SfsFile,
    decompressed: &Vec<u8>,
    path: &Path,
) -> Result<Vec<u8>> {
    if path.is_absolute() {
        bail!("The path must be relative to the game directory");
    } else {
        let file_path_str = path
            .to_str()
            .ok_or(anyhow!("Unable to convert path to valid UTF-8 string"))?;
        let file_path_fingerprint = finger::string(0, file_path_str);
        unpack_from_sfs_by_fingerprint(sfs_file, decompressed, file_path_fingerprint)
    }
}

const CLASS_MAGIC: [u8; 4] = [0xCA, 0xFE, 0xBA, 0xBE];

pub fn unpack_from_sfs_by_class_name(
    sfs_file: &SfsFile,
    decompressed: &Vec<u8>,
    class_name: String,
) -> Result<Vec<u8>> {
    let obfuscated_name = format!("sdw{}cwc2w9e", class_name);
    let obfuscated_chars: Vec<i32> = obfuscated_name.chars().map(|c| return c as i32).collect();
    let class_hash = finger::int(&obfuscated_chars);
    let class_fingerprint = finger::string(0, &format!("cod/{}", class_hash));
    let raw_class_data = unpack_from_sfs_by_fingerprint(sfs_file, decompressed, class_fingerprint)?;

    if raw_class_data.starts_with(&CLASS_MAGIC) {
        Ok(raw_class_data)
    } else {
        let decrypted_class_data: Vec<u8> = decrypt_data(raw_class_data, class_hash, 14, 0);

        let decrypted_class_data_with_header: Vec<u8> =
            [0xCA, 0xFE, 0xBA, 0xBE, 0x00, 0x00, 0x00, 0x2F]
                .iter()
                .cloned()
                .chain(decrypted_class_data)
                .collect();

        Ok(decrypted_class_data_with_header)
    }
}

pub fn decompress_sfs(sfs_file: &SfsFile) -> Result<Vec<u8>> {
    let progress = ProgressBar::new(sfs_file.header.uncompressed_size as u64);
    progress.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .progress_chars("#>-"),
    );

    let decompressed: Vec<u8> = sfs_file
        .chunk_boundaries
        .iter()
        .flat_map(|boundary| {
            let decompressed = decompress_chunk(&sfs_file, *boundary);
            progress.inc(decompressed.len() as u64);
            decompressed
        })
        .collect();

    Ok(decompressed)
}

pub fn unpack_sfs(path: &Path) -> Result<()> {
    let sfs_file = read_sfs(&path)?;
    let decompressed = decompress_sfs(&sfs_file)?;

    sfs_file.toc.par_iter().for_each(|entry| {
        if entry.size > 0 {
            let start_offset = entry.offset as usize;
            let end_offset = (entry.offset + entry.size) as usize;
            let mut data = &decompressed[start_offset..end_offset];
            let file_name = format!("{:X}.DAT", entry.fingerprint);
            let mut file = File::create(file_name).unwrap();
            file.write_all(&mut data).unwrap();
        }
    });

    return Ok(());
}
