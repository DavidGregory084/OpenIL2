use anyhow::{anyhow, Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use io::BufWriter;
use rusqlite::{params, Connection, Statement};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::{
    fs::{DirEntry, File},
    process::{Command, Stdio},
};

struct SfsEntry {
    file_name: String,
    file_size: i32,
    fingerprint: i64,
    key_hash: Option<i32>,
    key_len_offset: Option<i32>,
    key_idx_offset: Option<i32>,
}

fn transform_class(game_dir: PathBuf, class_data: Vec<u8>) -> Result<Vec<u8>> {
    let mut cmd = Command::new("./class-transformer.exe");

    let process = cmd
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .current_dir(game_dir)
        .spawn()
        .context("Running Class Transformer")?;

    let mut transformed_data = Vec::new();
    process.stdin.unwrap().write_all(&class_data)?;
    process.stdout.unwrap().read_to_end(&mut transformed_data)?;

    Ok(transformed_data)
}

fn unpack_sfs(
    size_stmt: &mut Statement,
    count_stmt: &mut Statement,
    entries_stmt: &mut Statement,
    game_dir: PathBuf,
    entry: DirEntry,
) -> Result<()> {
    let file_name = entry.file_name().to_string_lossy().into_owned();
    let file_type = entry
        .file_type()
        .with_context(|| format!("Unable to get file type for directory entry {:?}", entry))?;

    if file_type.is_file() && file_name.ends_with(".SFS") {
        let file_stem = Path::new(&file_name)
            .file_stem()
            .ok_or(anyhow!(
                "Unable to get filename without extension for {}",
                file_name
            ))?
            .to_str()
            .with_context(|| anyhow!("Unable to convert file path to a valid UTF-8 string"))?;

        let zip_file_path = game_dir.join(format!("{}.zip", file_stem));
        let zip_file = File::create(zip_file_path.clone())
            .with_context(|| anyhow!("Unable to create file {}", zip_file_path.display()))?;
        let buffered_file = BufWriter::new(zip_file);
        let mut zip = zip::ZipWriter::new(buffered_file);
        let zip_options = zip::write::FileOptions::default();

        println!("Reading file header for {}", file_name);
        let sfs_file = sfs::read_sfs(entry.path().as_path())
            .with_context(|| format!("Unable to read SFS file {}", file_name))?;

        println!("Decompressing {}", file_name);
        let decompressed = sfs::decompress_sfs(&sfs_file)
            .with_context(|| format!("Unable to decompress SFS file {}", file_name))?;

        let entries_size =
            size_stmt.query_row(params![file_name.clone()], |row| row.get::<usize, i64>(0))?;

        let entries_count =
            count_stmt.query_row(params![file_name.clone()], |row| row.get::<usize, u32>(0))?;

        let sfs_entries = entries_stmt
            .query_map(params![file_name.clone()], |row| {
                Ok(SfsEntry {
                    fingerprint: row.get(0)?,
                    file_name: row.get(1)?,
                    file_size: row.get(2)?,
                    key_hash: row.get(3)?,
                    key_len_offset: row.get(4)?,
                    key_idx_offset: row.get(5)?,
                })
            })
            .context("Unable to query SFS database")?;

        let progress = ProgressBar::new(entries_size as u64);
        progress.set_style(
            ProgressStyle::default_bar()
                .template(
                    "[{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} {msg} ({eta})",
                )
                .progress_chars("#>-"),
        );

        println!(
            "Repacking {} as {}",
            entry.path().display(),
            zip_file_path.display()
        );

        for sfs_entry in sfs_entries.enumerate() {
            if let (entry_idx, Ok(sfs_entry)) = sfs_entry {
                let entry_name = sfs_entry.file_name.to_ascii_uppercase();

                if sfs_entry.file_size == 0 {
                    zip.add_directory(&entry_name, zip_options)
                        .with_context(|| {
                            anyhow!(
                                "Unable to create directory {} in {}",
                                sfs_entry.file_name,
                                zip_file_path.display()
                            )
                        })?;
                } else {
                    zip.start_file(&entry_name, zip_options).with_context(|| {
                        anyhow!(
                            "Unable to create file {} in {}",
                            sfs_entry.file_name,
                            zip_file_path.display()
                        )
                    })?;

                    let decrypted_data = if sfs_entry.file_name.ends_with(".class") {
                        let class_name =
                            sfs_entry.file_name.replace(".class", "").replace("/", ".");

                        let class_data = sfs::unpack_from_sfs_by_class_name(
                            &sfs_file,
                            &decompressed,
                            class_name.to_string(),
                        )
                        .with_context(|| {
                            format!("Unable to extract class {} from {}", class_name, file_name)
                        })?;

                        transform_class(game_dir.clone(), class_data)?
                    } else {
                        let raw_data = sfs::unpack_from_sfs_by_fingerprint(
                            &sfs_file,
                            &decompressed,
                            sfs_entry.fingerprint,
                        )
                        .with_context(|| {
                            format!(
                                "Unable to extract file {} from {}",
                                sfs_entry.file_name, file_name
                            )
                        })?;

                        match (
                            sfs_entry.key_hash,
                            sfs_entry.key_len_offset,
                            sfs_entry.key_idx_offset,
                        ) {
                            (Some(hash), Some(len_offset), Some(idx_offset)) => {
                                sfs::decrypt_data(raw_data, hash, len_offset, idx_offset)
                            }
                            _ => raw_data,
                        }
                    };

                    progress.set_message(&format!("#{}/{}", entry_idx, entries_count));

                    progress
                        .wrap_write(&mut zip)
                        .write_all(&decrypted_data)
                        .with_context(|| {
                            anyhow!(
                                "Unable to write entry {} to {}",
                                sfs_entry.file_name,
                                zip_file_path.display()
                            )
                        })?;
                }
            }
        }

        zip.flush().with_context(|| {
            anyhow!("Unable to zip content to file {}", zip_file_path.display())
        })?;

        zip.finish()
            .with_context(|| anyhow!("Unable to finish {}", zip_file_path.display()))?;
    }

    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let game_dir = Path::new(&args[1]).to_owned();
    let _tmp_dir = Path::new(&args[2]).to_owned();
    let sfs_file = game_dir.clone().join(&args[3]);
    let sfs_db = game_dir.clone().join("sfs_db.sqlite");
    let connection = Connection::open(sfs_db).context("Unable to connect to SFS database")?;

    let mut size_stmt = connection
        .prepare(
            "SELECT COALESCE(SUM(size), 0)
             FROM sfs_entry
             WHERE file_name IS NOT NULL
             AND sfs_file = ?1 COLLATE NOCASE;",
        )
        .context("Unable to prepare SFS database query")?;

    let mut count_stmt = connection
        .prepare(
            "SELECT COUNT(*)
             FROM sfs_entry
             WHERE file_name IS NOT NULL
             AND sfs_file = ?1 COLLATE NOCASE;",
        )
        .context("Unable to prepare SFS database query")?;

    let mut entries_stmt = connection
        .prepare(
            "SELECT fingerprint, file_name, size, key_hash, key_len_offset, key_idx_offset
             FROM sfs_entry
             WHERE file_name IS NOT NULL
             AND sfs_file = ?1 COLLATE NOCASE
             ORDER BY toc_index;",
        )
        .context("Unable to prepare SFS database query")?;

    for entry in std::fs::read_dir(game_dir.clone())
        .with_context(|| format!("Unable to list directory {}", game_dir.display()))?
    {
        if let Ok(entry) = entry {
            if entry.path() == sfs_file {
                unpack_sfs(
                    &mut size_stmt,
                    &mut count_stmt,
                    &mut entries_stmt,
                    game_dir.clone(),
                    entry,
                )?;
            }
        }
    }

    Ok(())
}
