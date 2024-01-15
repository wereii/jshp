use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::multispace0;
use nom::error::context;
use nom::sequence::{preceded, terminated};
use nom::IResult;

use nom::combinator::peek;
use nom_locate::{position, LocatedSpan};

use std::fs::read_to_string;
use std::path::PathBuf;

type Span<'a> = LocatedSpan<&'a str>;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum CodeSpanKind {
    Jshp,
    JshpEcho,
}

/// A tuple of line and offset
type PositionTuple = (u32, usize);

#[derive(Clone, Debug)]
pub struct CodeSpan {
    pub start_position: PositionTuple,
    pub stop_position: PositionTuple,
    pub code: String,
    pub kind: CodeSpanKind,
}

#[derive(Clone, Debug)]
pub struct PreprocessedFile {
    pub code_spans: Vec<CodeSpan>,
    pub raw: String,
    pub file_path: PathBuf,
}

fn parse_code_span(s: Span) -> IResult<Span, CodeSpan> {
    // TODO - use VerboseError
    // https://github.com/rust-bakery/nom/blob/main/doc/error_management.md

    let (s, _) = context("Missing starting tag", take_until("<?"))(s)?;
    let (s, start_span) = position(s)?;

    let (s, opening_tag) = terminated(alt((tag("<?jshp"), tag("<?="))), multispace0)(s)?;
    let kind = match *opening_tag.fragment() {
        "<?jshp" => CodeSpanKind::Jshp,
        "<?=" => CodeSpanKind::JshpEcho,
        _ => panic!("Unexpected fragment"),
    };

    let (s, code) = context(
        "Missing closing tag",
        preceded(multispace0, take_until("?>")),
    )(s)?;

    let (s, _) = preceded(multispace0, tag("?>"))(s)?;
    let (s, stop_span) = position(s)?;

    return Ok((
        s,
        CodeSpan {
            start_position: (start_span.location_line(), start_span.location_offset()),
            stop_position: (stop_span.location_line(), stop_span.location_offset()),
            code: code.fragment().trim().to_owned(),
            kind,
        },
    ));
}

fn parse_data(content: &str) -> Result<Vec<CodeSpan>, String> {
    let mut span = Span::new(&content);
    if span.len() == 0 {
        panic!("Empty data")
    }

    let mut code_spans = Vec::new();
    loop {
        // A hack/extraneous read, I couldn't find a way to extract enough information from
        // the nom error (in `take_until` in `fn parse_code_span` ) to distinguish exhausted input from a "syntax" error
        // TODO: move this into the err branch of `parse_code_span`
        let peek_res: IResult<Span, Span> = peek(take_until("<?"))(span);
        if peek_res.is_err() {
            // no more fragments
            break;
        }

        let res = parse_code_span(span);
        match res {
            Ok((result_span, code_span)) => {
                code_spans.push(code_span.clone());
                span = result_span;
            }
            Err(e) => {
                return Err(format!("Failed parsing javascript code span, error {}", e));
            }
        }
    }

    Ok(code_spans)
}

pub fn process_file<'a>(file_path: PathBuf) -> Result<Box<PreprocessedFile>, String> {
    let mut processed_file = {
        PreprocessedFile {
            code_spans: vec![],
            raw: read_to_string(&file_path).map_err(|err| err.to_string())?,
            file_path,
        }
    };

    parse_data(&processed_file.raw)?
        .into_iter()
        .for_each(|code_span| {
            processed_file.code_spans.push(code_span);
        });
    Ok(Box::new(processed_file))
}

#[cfg(test)]
mod tests {
    use crate::javascript::parse::{parse_code_span, parse_data, Span};

    #[test]
    fn test_buffer_parser_jshp() {
        let res = parse_code_span(Span::new(
            "<html>\
                bla bla bla\
                <?jshp \
                echo(\"Hello World\") \
                ?> \
                bla bla bla\
                </html>",
        ));

        match res {
            Ok((_, code_span)) => {
                assert_eq!(code_span.code, "echo(\"Hello World\")");
                assert_eq!(code_span.start_position.0, 1);
                assert_eq!(code_span.start_position.1, 17);
                assert_eq!(code_span.stop_position.0, 1);
                assert_eq!(code_span.stop_position.1, 46);
            }
            Err(e) => {
                eprintln!("Error: {:?}", e);
                assert!(false);
            }
        }
    }

    #[test]
    fn test_parse_file() {
        let file_text = String::from(
            "<html>
                bla bla bla
                <?jshp
                echo(\"Hello World\")
                ?> bla bla bla
                <?= \"Hello\" + \" \" + \"World.\" ?>
                </html>",
        );

        let res = parse_data(&file_text);
        match res {
            Ok(_res) => {
                // println!("{:?}", res);
                // TODO
            }
            Err(err) => {
                println!("{}", err.escape_debug());
                assert!(false);
            }
        }
    }
}
