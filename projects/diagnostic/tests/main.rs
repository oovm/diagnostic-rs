#![allow(unused, dead_code)]
use diagnostic::{Color, Config, Console, Diagnostic, FileCache, Label, Palette, ReportKind, Source, SourceID};
use std::{iter::zip, ops::Range};

mod multi_file;
mod multi_line;
mod simple;
mod source;
mod stress_test;

#[test]
fn ready() {
    println!("it works!")
}
