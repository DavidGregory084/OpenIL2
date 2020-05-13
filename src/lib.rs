pub mod finger;

use flate2::bufread::*;
use memmap::Mmap;
use nom::bytes::complete::*;
use nom::multi::*;
use nom::number::complete::*;
use rayon::prelude::*;
use std::convert::TryInto;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::Result;
use std::path::Path;

#[derive(Debug)]
pub struct SfsFile {
    pub mmap: Mmap,
    pub header: SfsHeader,
    pub toc: Vec<SfsTocItem>,
    pub chunk_boundaries:  Vec<(usize, usize)>
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
    pub comment: String
}

#[derive(Debug, PartialEq, Eq)]
pub struct SfsTocItem {
    pub hash: u64,
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
            comment
        },
    ))
}

pub fn parse_toc_item(input: &[u8]) -> nom::IResult<&[u8], SfsTocItem> {
    let (rem, hash) = le_u64(input)?;
    let (rem, index) = le_u32(rem)?;
    let (rem, offset) = le_u32(rem)?;
    let (rem, size) = le_u32(rem)?;
    let (rem, unknown_1) = le_u32(rem)?;
    let (rem, attributes) = le_u32(rem)?;
    let (rem, unknown_2) = le_u32(rem)?;
    Ok((
        rem,
        SfsTocItem {
            hash,
            index,
            offset,
            size,
            unknown_1,
            attributes,
            unknown_2
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

pub fn sfs_hash(hash: u64, buf: &[u8]) -> u64 {
    // Extract the bottom and top bytes separately
    let bottom = (hash & 0xFFFFFFFF) as u32;
    let top = (hash >> 32 & 0xFFFFFFFF) as u32;

    // Update the hash using the input
    let (new_bottom, new_top) = buf.iter().fold((bottom, top), |(b, t), byte| {
        let bt = *byte as u32;
        let new_bottom = ((b << 8) | bt) ^ finger::BOTTOM_TABLE[(b >> 24) as usize];
        let new_top = ((t << 8) | bt) ^ finger::TOP_TABLE[(t >> 24) as usize];
        (new_bottom, new_top)
    });

    // Convert back to u64
    let bot_64 = new_bottom as u64;
    let top_64 = new_top as u64;

    // Combine back together
    return bot_64 & 0xFFFFFFFF | top_64 << 32;
}

pub fn sfs_decrypt(hash: u64, buf: &[u8]) -> Vec<u8> {
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

pub fn sfs_decrypt2(hash: u64, buf: &[u8]) -> Vec<u8> {
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
        sfs_hash(0, file_name.to_lowercase().as_bytes())
    } else {
        sfs_hash(0, header_slice)
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
        .iter().scan(chunk_table_end, |last, next| {
            let last_offset = *last;
            let next_offset = *next as usize;
            *last = next_offset;
            return Some((last_offset, next_offset));
        }).collect();

    println!("{:?}", decrypted_header);

    Ok(
        SfsFile {
            mmap,
            header: decrypted_header,
            toc,
            chunk_boundaries
        }
    )
}

pub fn decompress_chunk(sfs_file: &SfsFile, boundary: (usize, usize)) -> Vec<u8> {
    let (start, end) = boundary;
    let chunk = &sfs_file.mmap[start..end];

    if chunk.len() == 32768 {
        println!("Decompressing max size chunk at {:?},{:?}", start, end);
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
        decoder.read_to_end(&mut buffer).expect("Error decoding chunk");
        buffer
    } else {
        println!("Decompressing unknown chunk type {:?}", chunk[0..1].to_vec());
        chunk.to_vec()
    }
}

pub fn unpack_from_sfs_by_hash(sfs_file: &SfsFile, decompressed: &Vec<u8>, hash: u64) -> std::io::Result<Vec<u8>> {
    let toc_item = sfs_file.toc.iter().find(|item| {
        return item.hash == hash;
    }).expect("Couldn't find a matching entry in the SFS file");

    println!("Found entry at {:?}", toc_item);

    if toc_item.size == 0 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "The file is empty"));
    } else {
        let start_offset = toc_item.offset as usize;
        let end_offset = (toc_item.offset + toc_item.size) as usize;
        let data = &decompressed[start_offset..end_offset];
        Ok(data.to_vec())
    }
}

pub fn unpack_from_sfs_by_path(sfs_file: &SfsFile, decompressed: &Vec<u8>, path: &Path) -> std::io::Result<Vec<u8>> {
    if path.is_absolute() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "The path must be relative to the IL-2 directory"));
    } else {
        let file_path_str = path.as_os_str().to_str().unwrap();
        let file_path_hash = sfs_hash(0, file_path_str.as_bytes());
        return unpack_from_sfs_by_hash(sfs_file, decompressed, file_path_hash);
    }
}

pub fn decompress_sfs(sfs_file: &SfsFile) -> Result<Vec<u8>> {
    let decompressed: Vec<u8> = sfs_file.chunk_boundaries.par_iter().flat_map(|boundary| {
        return decompress_chunk(&sfs_file, *boundary);
    }).collect();

    return Ok(decompressed);
}

pub fn unpack_sfs(path: &Path) -> Result<()> {
    let sfs_file = read_sfs(&path)?;
    let decompressed = decompress_sfs(&sfs_file)?;

    sfs_file.toc.par_iter().for_each(|entry| {
        if entry.size > 0 {
            let start_offset = entry.offset as usize;
            let end_offset = (entry.offset + entry.size) as usize;
            let mut data = &decompressed[start_offset..end_offset];
            let file_name = format!("{:X}.DAT", entry.hash);
            let mut file = File::create(file_name).unwrap();
            file.write_all(&mut data).unwrap();
        }
    });

    return Ok(());
}
