// this file might chaotic, but it works

use std::io::Cursor;
use colored::Colorize;
use std::fs::File;
use flate2::read::GzDecoder;
use tar::Archive;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

async fn fetch_url(url: String, file_name: String) -> Result<()> {
    let response = reqwest::get(url).await?;
    let mut file = std::fs::File::create(file_name)?;
    let mut content =  Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;
    Ok(())
}
 
#[tokio::main]
pub async fn download(version: String) -> Result<()> {
    let mut version = version.clone().to_string();
    let mut file_name = String::from("lua-");
    let end = String::from(".tar.gz");
    file_name.push_str(&version);
    file_name.push_str(&end);
    let mut lua_url = "http://www.lua.org/ftp/lua-".to_string();
    version.push_str(end.clone().as_str());
    lua_url.push_str(&version);
    println!("{}", "downloading lua...".blue().bold() );
    fetch_url(lua_url, file_name.to_string()).await.unwrap();
    let file = File::open(file_name)?;
    let mut archive = Archive::new(GzDecoder::new(file));
    archive.unpack(".")?;
    Ok(())
}
