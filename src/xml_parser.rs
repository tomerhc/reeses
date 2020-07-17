use quick_xml::events::Event;
use quick_xml::Reader;
use std::io::BufRead;
use std::{error, fmt};

pub trait XmlObject<S> {
    fn from_reader<T: BufRead>(
        reader: &mut Reader<T>,
        buff: &mut Vec<u8>,
    ) -> Result<S, Box<dyn error::Error>>;
}

#[derive(Debug)]
pub struct Channel {
    pub title: Option<String>,
    pub description: Option<String>,
    pub pub_date: Option<String>,
    pub items: Vec<Item>,
}

impl XmlObject<Channel> for Channel {
    fn from_reader<T: BufRead>(
        reader: &mut Reader<T>,
        buff: &mut Vec<u8>,
    ) -> Result<Self, Box<dyn error::Error>> {
        let mut channel = Channel {
            title: None,
            description: None,
            pub_date: None,
            items: Vec::new(),
        };

        // get to first channel start event
        loop {
            match reader.read_event(buff)? {
                Event::Start(elem) => match elem.name() {
                    b"channel" => {
                        break;
                    }
                    _ => continue,
                },
                Event::Eof => {
                    return Err(Box::from(std::io::Error::new(
                        std::io::ErrorKind::UnexpectedEof,
                        "eof error",
                    )))
                }
                _ => (),
            }
        }

        let mut skip_buf: Vec<u8> = Vec::new();
        loop {
            match reader.read_event(buff)? {
                Event::Start(elem) => match elem.name() {
                    b"title" => channel.title = element_text(reader)?,
                    b"description" => channel.description = element_text(reader)?,
                    b"pubDate" => channel.pub_date = element_text(reader)?,
                    b"item" => {
                        let item = Item::from_reader(reader, buff)?;
                        channel.items.push(item);
                    }
                    _ => {
                        reader.read_to_end(elem.name(), &mut skip_buf)?;
                        skip_buf.clear();
                    }
                },
                Event::End(_) => break,
                Event::Eof => {
                    return Err(Box::from(std::io::Error::new(
                        std::io::ErrorKind::UnexpectedEof,
                        "eof error",
                    )))
                }
                _ => (),
            }
            buff.clear();
        }
        Ok(channel)
    }
}

impl fmt::Display for Channel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buff = String::new();
        if let Some(t) = &self.title {
            let s = format!(
                "Channel: {}\n=============================================================\n",
                t
            );
            buff.push_str(&s[..]);
        } else {
            let s = format!(
                "Channel: No Title Found\n=============================================================\n"
            );
            buff.push_str(&s[..]);
        }

        if let Some(t) = &self.description {
            let s = format!("{}\n", t);
            buff.push_str(&s[..]);
        }

        if let Some(t) = &self.pub_date {
            let s = format!("Published: {}\n", t);
            buff.push_str(&s[..]);
        }

        for item in self.items.clone().iter_mut() {
            let s = format!("\n{}", item);
            buff.push_str(&s[..]);
        }
        write!(f, "{}", buff)
    }
}

#[derive(Debug, Clone)]
pub struct Item {
    pub title: Option<String>,
    pub description: Option<String>,
    pub content_encoded: Option<String>,
    pub link: Option<String>,
    pub author: Option<String>,
    pub category: Option<String>,
    pub pub_date: Option<String>,
    pub guid: Option<String>,
}

impl XmlObject<Item> for Item {
    fn from_reader<T: BufRead>(
        reader: &mut Reader<T>,
        buff: &mut Vec<u8>,
    ) -> Result<Self, Box<dyn error::Error>> {
        let mut item = Item {
            title: None,
            description: None,
            content_encoded: None,
            link: None,
            author: None,
            category: None,
            pub_date: None,
            guid: None,
        };

        let mut skip_buf: Vec<u8> = Vec::new();
        loop {
            match reader.read_event(buff)? {
                Event::Start(elem) => match elem.name() {
                    b"title" => item.title = element_text(reader)?,
                    b"description" => item.description = element_text(reader)?,
                    b"content:encoded" => item.content_encoded = element_text(reader)?,
                    b"link" => item.link = element_text(reader)?,
                    b"author" => item.author = element_text(reader)?,
                    b"category" => item.category = element_text(reader)?,
                    b"pubDate" => item.pub_date = element_text(reader)?,
                    b"guid" => item.guid = element_text(reader)?,
                    _ => {
                        reader.read_to_end(elem.name(), &mut skip_buf)?;
                        skip_buf.clear();
                    }
                },
                Event::End(_) => break,
                Event::Eof => {
                    return Err(Box::from(std::io::Error::new(
                        std::io::ErrorKind::UnexpectedEof,
                        "eof error",
                    )))
                }
                _ => (),
            }
            buff.clear();
        }
        Ok(item)
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buff = String::new();
        if let Some(t) = &self.title {
            let s = format!(
                "\n{}\n-----------------------------------------------------------\n",
                t
            );
            buff.push_str(&s[..]);
        } else {
            let s = format!(
                "No Item Title Found\n-----------------------------------------------------------\n",
            );
            buff.push_str(&s[..]);
        }

        if let Some(t) = &self.author {
            let s = format!("By: {}\n", t);
            buff.push_str(&s[..]);
        }
        if let Some(t) = &self.category {
            let s = format!("Category: {}\n", t);
            buff.push_str(&s[..]);
        }

        if let Some(t) = &self.pub_date {
            let s = format!("Published: {}\n", t);
            buff.push_str(&s[..]);
        }

        if let Some(t) = &self.description {
            let s = format!("{}\n", t);
            buff.push_str(&s[..]);
        }

        if let Some(t) = &self.content_encoded {
            let s = format!("{}\n", t);
            buff.push_str(&s[..]);
        }

        if let Some(t) = &self.link {
            let s = format!("Link: {}\n", t);
            buff.push_str(&s[..]);
        }

        write!(f, "{}", buff)
    }
}

pub fn element_text<T: BufRead>(
    reader: &mut Reader<T>,
) -> Result<Option<String>, Box<dyn error::Error>> {
    let mut content: Option<String> = None;
    let mut buf = Vec::new();
    let mut skip_buf = Vec::new();

    loop {
        match reader.read_event(&mut buf)? {
            Event::Start(elem) => {
                reader.read_to_end(elem.name(), &mut skip_buf)?;
            }
            Event::CData(elem) => {
                //                let unesc = elem.unescaped()?;
                let text: String = reader.decode(&elem)?.into();
                if let Some(mut c) = content {
                    c.push_str(text.as_str());
                }
                content = Some(text);
            }
            Event::Text(elem) => {
                let text = elem.unescape_and_decode(reader)?;
                if let Some(mut c) = content {
                    c.push_str(text.as_str());
                }
                content = Some(text);
            }
            Event::End(_) | Event::Eof => break,
            _ => (),
        }
        buf.clear();
    }

    Ok(content)
}
