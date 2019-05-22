
use std::io::{Read, Result, stdin};
use pulldown_cmark::{Parser, Event, Tag};
use comrak::{parse_document, Arena, ComrakOptions};

/* Prints the first (h1) title of a markdown document */

#[derive(Debug)]
enum RunMode {
    Title,
    Dump
}

fn main() -> Result<()> {
    let query_str = std::env::args().nth(1).unwrap_or(String::new());
    let mode =
        match query_str.as_ref() {
            "title" => RunMode::Title,
            _ => RunMode::Dump
        };
    dbg!(&mode);
    let mut markdown_input: String = String::from("");
    stdin().lock().read_to_string(&mut markdown_input)?;

    match mode {
        RunMode::Title => {
            let mut parser = Parser::new(&markdown_input);
            let title = parse_title(&mut parser).unwrap_or(String::from(""));
            println!("{}", title);
        },
        RunMode::Dump => {
            let arena = Arena::new();
            let foo = comrak::parse_document(&arena, &markdown_input, &ComrakOptions::default());
            dbg!(foo);
        }
    }
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
