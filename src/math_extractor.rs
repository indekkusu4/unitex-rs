
use pest::{iterators::{Pair, Pairs}, Parser};
use pest_derive::Parser;

use crate::parser::latex_unicodify;

#[derive(Parser)]
#[grammar = "math-extractor.pest"]
pub struct MathExtractorParser;

pub fn pair_to_unicode(pair: Pair<'_, Rule>) -> String {
    match pair.as_rule() {
        Rule::inline_math => latex_unicodify(pair.as_str()),
        Rule::block_math => latex_unicodify(pair.as_str()),
        Rule::content => pairs_to_unicode(pair.into_inner()),
        _ => pair.as_str().to_string(), 
    }
}

pub fn pairs_to_unicode(iter: Pairs<'_, Rule>) -> String {
    iter.map(pair_to_unicode)
        .reduce(|s, t| s + &t)
        .unwrap_or_default()
}

pub fn unicodify(s: &str) -> String {
    let s = s.replace("\\$", "\\dollar");
    match MathExtractorParser::parse(Rule::content, &s) {
        Ok(pairs) => pairs_to_unicode(pairs),
        Err(e) => e.to_string(),
    }
}
