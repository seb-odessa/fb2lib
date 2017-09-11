

fn ls(filename: &str) -> ZipResult<()> {
    let file = std::fs::File::open(&std::path::Path::new(filename))?;
    let mut archive = zip::ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let zip_file = archive.by_index(i)?;
        println!(
            "Filename: {}, {} / {}",
            zip_file.name(),
            zip_file.compressed_size(),
            zip_file.size()
        );
    }

    Ok(())
}