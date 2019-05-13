
use std::io::{Read, Result, stdin};
use pulldown_cmark::{Parser, Event, Tag};

/* Prints the first (h1) title of a markdown document */

fn main() -> Result<()> {
    let mut markdown_input: String = String::from("");
    stdin().lock().read_to_string(&mut markdown_input)?;

    let mut parser = Parser::new(&markdown_input);
    let title = parse_title(&mut parser).unwrap_or(String::from(""));
    println!("{}", title);
    Ok(())
}

fn parse_title(parser: &mut Parser) -> Option<String> {
    let mut ready_for_header = false;
    for event in parser {
        use Event::*;
        match event {
            Start(Tag::Header(1)) => { ready_for_header = true; }
            Text(x) => {
                if ready_for_header {
                    return Some(x.to_string());
                }
            },
            End(Tag::Header(1)) => {
                break;
            },
            _ => ()
        };
    }
    None
}
