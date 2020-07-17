use quick_xml::Reader;
use reqwest;
use std::error::Error;
use std::str;

mod xml_parser;
use self::xml_parser::{Channel, XmlObject};

type BoxResult<T> = Result<T, Box<dyn Error>>;

pub async fn load_channel(url: &str) -> BoxResult<Channel> {
    let response = reqwest::get(url).await?;
    if !response.status().is_success() {
        return Err(Box::from(format!(
            "get Error: {} return status code {}",
            url,
            response.status()
        )));
    }
    let resp_bytes = response.bytes().await?;
    let mut reader = Reader::from_str(str::from_utf8(&resp_bytes)?);
    reader.trim_text(true).expand_empty_elements(true);
    let mut buff: Vec<u8> = Vec::new();
    Ok(Channel::from_reader(&mut reader, &mut buff)?)
}
