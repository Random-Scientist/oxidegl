use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use anyhow::Result;
use roxmltree::{Attribute, Node};
pub mod codegen;
pub mod doc_parse;
pub mod tasks;

pub fn remove_multi(s: &str, m: &[&str]) -> String {
    let mut out = String::with_capacity(s.len());
    for char in s.chars() {
        out.push(char);
        for filt in m {
            if out.ends_with(filt) {
                out.truncate(out.len() - filt.len());
            }
        }
    }
    out
}

trait NodeExt: Sized {
    fn find_named_child(&self, name: &str) -> Option<Self>;
    fn find_named_attribute<'a>(&'a self, name: &'a str) -> Option<Attribute<'a, 'a>>;
}
impl NodeExt for Node<'_, '_> {
    fn find_named_child(&self, name: &str) -> Option<Self> {
        self.children()
            .find(|child| child.tag_name().name() == name)
    }

    fn find_named_attribute<'b>(&'b self, name: &'b str) -> Option<Attribute<'b, 'b>> {
        let mut attrs = self.attributes();
        attrs.find(|attr| attr.name() == name)
    }
}
fn snake_case_from_title_case(src: &str) -> String {
    let mut out = String::new();
    let mut prev: Option<char> = None;
    for c in src.chars() {
        match prev {
            Some(prev) => {
                if !prev.is_numeric() && c.is_numeric() || c.is_uppercase() && !prev.is_numeric() {
                    out.push('_');
                }
                out.extend(c.to_lowercase());
            }
            None => out.extend(c.to_lowercase()),
        }
        prev = Some(c);
    }
    out
}
fn open_file_writer<P: AsRef<Path>>(path: P) -> Result<impl Write> {
    let _ = std::fs::remove_file(&path);
    let file = File::create(path)?;
    Ok(BufWriter::new(file))
}
