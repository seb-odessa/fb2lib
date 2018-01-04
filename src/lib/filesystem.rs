use sal;
use result::Fb2Result;
use result::Fb2Error;
use crypto;
use crypto::digest::Digest;
use rustc_serialize::hex::ToHex;

use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::collections::HashMap;

pub fn sha1(input: &[u8]) -> Vec<u8> {
    let mut hasher = crypto::sha1::Sha1::new();
    hasher.input(input);
    let mut hash: Vec<u8> = vec![0; hasher.output_bytes()];
    hasher.result(&mut hash);
    return hash;
}

pub fn sha1_string(input: &[u8]) -> String {
    sha1(input).to_hex().to_uppercase()
}


fn get_file_name(file_name: &str) -> Result<String, Fb2Error> {
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
    if let Some(arch) = sal::get_archive_sizes(&conn, &get_file_name(archive_name)?)? {
        let mut file = File::open(&archive_name)?;
        let mut buffer = Vec::with_capacity(arch.piece_length);
        buffer.resize(arch.piece_length, 0u8);
        let mut idx = 0;
        let mut bytes = 0;
        let mut desc = HashMap::with_capacity(arch.pieces_count);
        print!("Calculating hashes.");
        loop {
            let size = file.read(&mut buffer)?;
            if 0 == size {
                break;
            }
            if size < arch.piece_length {
                buffer.resize(size, 0u8);
            }
            let hash = sha1_string(&buffer);
            println!(". {} -> {}", idx, &hash);

            desc.insert(idx, hash);
            bytes += size;
            idx += 1;
        }
        println!(". Done.");
        if bytes != arch.total_length {
            let err = Fb2Error::Custom(format!(
                "Archive is not complete. Expected {} bytes, but was readed {}",
                arch.total_length,
                bytes
            ));
            return Err(err);
        }
        if idx as usize != arch.pieces_count {
            let err = Fb2Error::Custom(format!(
                "Archive is not complete. Expected {} pieces, but found {}",
                arch.pieces_count,
                idx
            ));
            return Err(err);
        }
        let last_good_index = sal::validate_pieces(&conn, arch.id, &desc)?;
        if last_good_index != 0 {
            let err = Fb2Error::Custom(format!(
                "The hash of piece {} in archive {} is not valid: {}",
                last_good_index,
                &arch.id,
                desc[&last_good_index]
            ));
            return Err(err);
        }

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