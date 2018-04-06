use sal;
use result::Fb2Result;
use result::Fb2Error;
use crypto::sha1::Sha1;
use crypto::digest::Digest;
use crossbeam;
use torrent::Metainfo;

use std::io;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn get_basename(file_name: &str) -> Fb2Result<String> {
    let path = Path::new(file_name);
    if path.is_file() {
        if let Some(name) = path.file_name() {
            if let Some(name) = name.to_str() {
                return Ok(name.to_string());
            }
        }
    }
    Err(Fb2Error::Custom(format!("Can't find file {}", file_name)))
}

pub fn check_integrity(db_file_name: &str, archive_name: &str) -> Fb2Result<()> {
    let conn = sal::get_connection(db_file_name)?;
    if let Some(arch) = sal::get_archive_sizes(&conn, &get_basename(archive_name)?)? {
        let mut bytes = 0;
        print!("Calculating hashes: ");
        let mut hasher = Sha1::new();
        let mut file = File::open(&archive_name)?;
        let mut jobs = Vec::with_capacity(arch.pieces_count);
        let mut buffer = Vec::with_capacity(arch.piece_length);
        let mut desc = sal::HashesByIdx::with_capacity(arch.pieces_count);

        buffer.resize(arch.piece_length, 0u8);
        crossbeam::scope(|scope| for index in 0..arch.pieces_count {
            if let Some(size) = file.read(&mut buffer).ok() {
                bytes += size;
                if size < arch.piece_length {
                    buffer.resize(size, 0u8);
                }
                let arg: Vec<u8> = buffer.clone();
                let job = scope.spawn(move || {
                    hasher.reset();
                    hasher.input(&arg);
                    return (index as i64, hasher.result_str());
                });
                jobs.push(job);
            }
        });
        if bytes != arch.total_length {
            let err = Fb2Error::Custom(format!(
                "Archive is not complete. Expected {} bytes, but was readed {}",
                arch.total_length,
                bytes
            ));
            return Err(err);
        }
        while let Some(job) = jobs.pop() {
            let (index, hash) = job.join();
            desc.insert(index, hash);
        }

        if let Some(index) = sal::validate(&conn, arch.id, &desc)? {
            let err = if let Some(expected) = sal::get_piece_hash(&conn, arch.id, index)? {
                Fb2Error::Custom(format!(
                    "The hash of piece {} in archive {} is not valid: expected {}, actual {}",
                    index,
                    &arch.id,
                    expected,
                    desc[&index]
                ))
            } else {
                Fb2Error::Custom(format!(
                    "The piece {} in archive {} was not found in DB",
                    index,
                    &arch.id
                ))
            };
            return Err(err);
        }
        println!(" Ok");
        return Ok(());
    }
    //
    let err = Fb2Error::Custom(format!(
        "Archive with name {} was not found in {}",
        archive_name,
        db_file_name
    ));
    Err(err)
}

pub fn load_torrent(torrent_file_name: &str) -> Fb2Result<Metainfo>{
    let mut buffer = Vec::new();
    // @todo Add file filter by magic prefix
    File::open(torrent_file_name)?.read_to_end(&mut buffer)?;
    let meta = Metainfo::from(&buffer).map_err(|e| io::Error::new(io::ErrorKind::Other, e.description()))?;
    Ok(meta)
}