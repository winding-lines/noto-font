/// Handle fonts for Conrod.
extern crate hyper;

use std::path::{PathBuf, Path};
use std::fs;
use std::io;
use hyper::Client;
use hyper::header::Connection;
use std::process::Command;
use std::env;

pub const INFO_URL: &'static str = "http://www.google.com/get/noto/";

pub const DOWNLOAD_URL: &'static str = "https://noto-website-2.storage.googleapis.\
                                        com/pkgs/NotoSans-unhinted.zip";
pub const RELATIVE_TO_HOME: &'static str = ".fonts";
pub const ZIP_NAME: &'static str = "NotoSans-unhinted.zip";
pub const FONT_NAME: &'static str = "NotoSans-Regular.ttf";


/// Return the name of the user's home folder.
pub fn home_dir() -> String {
    env::home_dir().unwrap().to_str().unwrap().to_owned()
}

/// Locate a local font or download it.                                    
pub fn find_or_download_font() -> Result<PathBuf, io::Error> {
    let mut base = PathBuf::from(home_dir());
    base.push(RELATIVE_TO_HOME);
    if !base.is_dir() {
        try!(fs::create_dir(&base));
    }

    let mut font_path = base.clone();
    font_path.push(FONT_NAME);
    if font_path.is_file() {
        return Ok(font_path);
    }

    let zip_path = try!(download_zip(&base));
    try!(unzip(&zip_path, &base));
    Ok(font_path)
}


/// Download the zipped font file in a wellknown location.
fn download_zip(base: &Path) -> Result<PathBuf, io::Error> {
    let mut zip_path = base.to_owned();
    zip_path.push(ZIP_NAME);
    if zip_path.is_file() {
        return Ok(zip_path);
    }
    let http = Client::new();
    let mut zip_file = try!(fs::File::create(&zip_path));
    match http.get(DOWNLOAD_URL)
              .header(Connection::close())
              .send() {
        Ok(mut res) => {
            try!(io::copy(&mut res, &mut zip_file));

            Ok(zip_path)
        }
        Err(e) => {
            Err(io::Error::new(io::ErrorKind::Other,
                               format!("Could not download the font: {}", e)))
        }
    }
}

fn unzip(zip: &Path, dest: &Path) -> Result<(), io::Error> {
    // Unzip with external command for now,
    // flate2 only provides lower level primitives.
    match Command::new("unzip").arg(zip).arg("-d").arg(dest).status() {
        Ok(exit_status) if exit_status.success() => Ok(()),
        Ok(exit_status) => {
            let msg = format!("Command `unzip {:?} -d {:?}` exited with {}",
                              zip.as_os_str(),
                              dest.as_os_str(),
                              exit_status.code().unwrap_or(99));
            Err(io::Error::new(io::ErrorKind::Other, msg))

        }
        Err(e) => {
            let msg = format!("Command `unzip {:?} -d {:?}` failed with {:?}",
                              zip.as_os_str(),
                              dest.as_os_str(),
                              e);
            Err(io::Error::new(io::ErrorKind::Other, msg))
        }
    }
}
