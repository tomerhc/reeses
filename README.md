# reeses
A tiny, async rss reader and parser. 
Currently, I have not made a TUI for it, but maybe in the future..

To use it, call the `load_channel` function from `lib.rs`. it will return a vector of futures that can be executed, returning a `Channel` struct for each url. the `Channel` struct's `fmt::display` trait will print the channels title, description and all its items (its a bit lazy, for better handling you can access the actual fields of the struct). 

The XML parsing is very minimal, and is havily based on (basically a watered-down version of) [rust-syndication/rss](https://github.com/rust-syndication/rss). 
