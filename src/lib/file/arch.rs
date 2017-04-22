extern crate zip;
extern crate std;

use std::io;
use std::fs;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

pub fn open_zip(name: &str) -> zip::result::ZipResult<zip::ZipArchive<std::fs::File>>
{
    let fname = std::path::Path::new(name);
    let file = try!(fs::File::open(&fname));
    zip::ZipArchive::new(file)
}

#[cfg(unix)]
pub fn convert_permissions(mode: Option<u32>) -> Option<fs::Permissions>
{
    match mode {
        Some(mode) => Some(fs::Permissions::from_mode(mode)),
        None => None,
    }
}

#[cfg(not(unix))]
pub fn convert_permissions(_mode: Option<u32>) -> Option<fs::Permissions>
{
    None
}

pub fn write_file(file: &mut zip::read::ZipFile, outpath: &std::path::Path, perms: Option<fs::Permissions>)
{
    let mut outfile = fs::File::create(&outpath).unwrap();
    io::copy(file, &mut outfile).unwrap();
    if let Some(perms) = perms {
        fs::set_permissions(outpath, perms).unwrap();
    }
}

pub fn create_directory(outpath: &std::path::Path, perms: Option<fs::Permissions>)
{
    fs::create_dir_all(&outpath).unwrap();
    if let Some(perms) = perms {
        fs::set_permissions(outpath, perms).unwrap();
    }
}

pub fn sanitize_filename(filename: &str) -> std::path::PathBuf
{
    let no_null_filename = match filename.find('\0') {
        Some(index) => &filename[0..index],
        None => filename,
    };

    std::path::Path::new(no_null_filename)
        .components()
        .filter(|component| match *component {
            std::path::Component::Normal(..) => true,
            _ => false
        })
        .fold(std::path::PathBuf::new(), |mut path, ref cur| {
            path.push(cur.as_os_str());
            path
        })
}
