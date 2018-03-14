use sal;
use filesystem;
use result::into;
use result::Fb2Result;

pub fn load(db_file_name: &str, torrents: &Vec<&str>) -> Fb2Result<()> {
    for torrent in torrents {
        println!("Processing: {}", torrent);
        let metainfo = filesystem::load_torrent(torrent)?;
        println!("file name:     {}", &metainfo.get_file_name());
        println!("creation date: {}", &metainfo.get_creation_date());
        println!("info hash:     {}", &metainfo.get_info_hash());
        println!("total length:  {}", &metainfo.get_length());
        println!("piece length:  {}", &metainfo.get_piece_length());
        println!("piece count:   {}", &metainfo.get_piece_count());
        sal::register(db_file_name, metainfo).map_err(into)?;
    }
    Ok(())
}

pub fn check(db_file_name: &str, archive_name: &str) -> Fb2Result<()> {
    println!("torrent_check({}, {})", db_file_name, archive_name);
    filesystem::check_integrity(db_file_name, archive_name)
}
