use wasm_bindgen::prelude::*;

use crate::{utils::process_lyrics, LyricLine, LyricWord};

use std::fmt::Write;
use std::{borrow::Cow, str::FromStr};

use nom::{bytes::complete::*, combinator::opt, multi::many1};
use nom::{character::complete::line_ending, IResult};

#[inline]
pub fn parse_time(src: &str) -> IResult<&str, usize> {
    let (src, _start) = tag("[")(src)?;

    let (src, min) = take_until1(":")(src)?;
    let (src, _) = tag(":")(src)?;
    let (src, sec) = take_until1(".")(src)?;
    let (src, _) = tag(".")(src)?;
    let (src, ms) = take_while_m_n(1, 3, |c: char| c.is_ascii_digit())(src)?;

    let min = u32::from_str(min).unwrap();
    let sec = u32::from_str(sec).unwrap();
    let mst = ms.trim_start_matches('0');
    let msl = mst.len();
    let ms = u32::from_str(ms).unwrap();

    let time = min as usize * 60 * 1000
        + sec as usize * 1000
        + ms as usize * (10usize).pow(3 - msl as u32);

    let (src, _) = tag("]")(src)?;
    Ok((src, time))
}

#[test]
fn time_test() {
    assert_eq!(parse_time("[00:01.12]"), Ok(("", 1120)));
    assert_eq!(parse_time("[00:10.254]"), Ok(("", 10254)));
    assert_eq!(parse_time("[01:10.1]"), Ok(("", 70100)));
    assert_eq!(parse_time("[168:10.254]"), Ok(("", 10090254)));
    assert!(parse_time("[168:10.254233]").is_err());
}

#[inline]
pub fn parse_line(src: &str) -> IResult<&str, Vec<LyricLine<'_>>> {
    let (src, times) = many1(parse_time)(src)?;
    match is_not("\r\n")(src) {
        Ok((src, line)) => {
            let (src, _) = opt(line_ending)(src)?;
            Ok((
                src,
                times
                    .into_iter()
                    .map(|t| LyricLine {
                        words: vec![LyricWord {
                            start_time: t,
                            end_time: 0,
                            word: Cow::Borrowed(line),
                        }],
                        ..Default::default()
                    })
                    .collect(),
            ))
        }
        Err(nom::Err::Error(nom::error::Error {
            input,
            code: nom::error::ErrorKind::IsNot,
        })) => Ok((
            src,
            times
                .into_iter()
                .map(|t| LyricLine {
                    words: vec![LyricWord {
                        start_time: t,
                        end_time: 0,
                        word: Cow::Borrowed(input),
                    }],
                    ..Default::default()
                })
                .collect(),
        )),
        Err(e) => Err(e),
    }
}

#[test]
fn lyric_line_test() {
    assert_eq!(
        parse_line("[00:01.12] test LyRiC"),
        Ok((
            "",
            vec![LyricLine {
                words: vec![LyricWord {
                    start_time: 1120,
                    end_time: 0,
                    word: Cow::Borrowed(" test LyRiC")
                }],
                ..Default::default()
            }]
        ))
    );
    assert_eq!(
        parse_line("[00:10.254][00:10.254] sssxxx\nrestline"),
        Ok((
            "restline",
            vec![
                LyricLine {
                    words: vec![LyricWord {
                        start_time: 10254,
                        end_time: 0,
                        word: Cow::Borrowed(" sssxxx")
                    }],
                    ..Default::default()
                },
                LyricLine {
                    words: vec![LyricWord {
                        start_time: 10254,
                        end_time: 0,
                        word: Cow::Borrowed(" sssxxx")
                    }],
                    ..Default::default()
                }
            ]
        ))
    );
    assert_eq!(
        parse_line("[01:10.1]"),
        Ok((
            "",
            vec![LyricLine {
                words: vec![LyricWord {
                    start_time: 70100,
                    end_time: 0,
                    word: Cow::Borrowed("")
                }],
                ..Default::default()
            }]
        ))
    );
}

#[inline]
pub fn parse_lrc(src: &str) -> Vec<LyricLine> {
    let lines = src.lines();
    let mut result = Vec::with_capacity(lines.size_hint().1.unwrap_or(1024).min(1024));

    for line in lines {
        if let Ok((_, line)) = parse_line(line) {
            result.extend_from_slice(&line);
        }
    }

    process_lyrics(&mut result);

    result
}

fn write_timestamp(result: &mut String, time: usize) {
    let ms = time % 1000;
    let sec = (time - ms) / 1000;
    let min = (sec - sec % 60) / 60;

    write!(result, "[{:02}:{:02}.{:03}]", min, sec % 60, ms).unwrap()
}

#[inline]
pub fn stringify_lrc(lines: &[LyricLine]) -> String {
    let capacity: usize = lines
        .iter()
        .map(|x| x.words.iter().map(|y| y.word.len()).sum::<usize>() + 13)
        .sum();
    let mut result = String::with_capacity(capacity);

    for line in lines {
        if !line.words.is_empty() {
            write_timestamp(&mut result, line.words[0].start_time);
            for word in line.words.iter() {
                result.push_str(&word.word);
            }
            result.push('\n');
        }
    }

    result
}

#[test]
fn stringify_lrc_test() {
    let lrc = parse_lrc("[00:01.12] test LyRiC\n[00:10.254] sssxxx");
    assert_eq!(
        stringify_lrc(&lrc),
        "[00:01.120] test LyRiC\n[00:10.254] sssxxx\n"
    );
}

#[wasm_bindgen(js_name = "parseLrc", skip_typescript)]
pub fn parse_lrc_js(src: &str) -> JsValue {
    serde_wasm_bindgen::to_value(&parse_lrc(src)).unwrap()
}

#[wasm_bindgen(js_name = "stringifyLrc", skip_typescript)]
pub fn stringify_lrc_js(lrc: JsValue) -> String {
    let lines: Vec<LyricLine> = serde_wasm_bindgen::from_value(lrc).unwrap();
    stringify_lrc(&lines)
}

#[test]
fn lrc_bench_test() {
    let mut times = Vec::with_capacity(1024);
    for _ in 0..1024 {
        let t = std::time::Instant::now();
        let _l = parse_lrc("[00:01.12] test LyRiC");
        times.push(t.elapsed());
    }
    let times = times.into_iter().map(|x| x.as_micros()).collect::<Vec<_>>();
    println!("used {} us", times.iter().sum::<u128>());
    println!(
        "average {} us",
        times.iter().sum::<u128>() / times.len() as u128
    );
}
