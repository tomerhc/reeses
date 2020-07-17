use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
type CatUrl = (String, Option<String>);

pub fn read_urls_file(
    path: &str,
) -> Result<HashMap<String, Vec<String>>, Box<dyn std::error::Error>> {
    let f = File::open(path)?;
    let lines = BufReader::new(f).lines();
    let mut urls: HashMap<String, Vec<String>> = HashMap::new();
    for line in lines {
        let l = line?;
        if l.len() > 0 {
            if "#" == &l[0..1] {
                continue;
            } else {
                match get_categories(l) {
                    (s, Some(c)) => {
                        if urls.contains_key(&c) {
                            urls.get_mut(&c).unwrap().push(s);
                        } else {
                            urls.insert(c, vec![s]);
                        }
                    }
                    (s, None) => {
                        if urls.contains_key("no category") {
                            urls.get_mut("no category").unwrap().push(s);
                        } else {
                            urls.insert(String::from("no category"), vec![s]);
                        }
                    }
                }
            }
        }
    }
    Ok(urls)
}

fn get_categories(line: String) -> CatUrl {
    let mut spl = line.split_whitespace().map(|s| String::from(s));
    (spl.next().unwrap(), spl.next())
}

pub fn filter_categories(
    urls: HashMap<String, Vec<String>>,
    categories: Vec<&str>,
) -> HashMap<String, Vec<String>> {
    urls.into_iter()
        .filter(|(c, _)| categories.contains(&c.as_str()))
        .collect()
}
