use futures::future::join_all;
use reeses;
mod urls;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let paths = urls::read_urls_file("/home/tomerh/.config/reeses/urls")?;
    let paths = urls::filter_categories(paths, vec!["infosec", "linux"]);

    let channels: Vec<String> = paths.into_iter().map(|(_, u)| u).flatten().collect();

    let futs: Vec<_> = channels.iter().map(|u| reeses::load_channel(u)).collect();
    for res in join_all(futs).await {
        println!("{}", res?);
    }
    Ok(())
}
