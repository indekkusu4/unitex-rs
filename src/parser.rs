use pest::Parser;
use pest::iterators::{Pair, Pairs};
use pest_derive::Parser;

use crate::fixed_macro::get_fixed_value;
use crate::unary_macro::{subscript, supscript, UNARY};

#[derive(Parser)]
#[grammar = "latex-math.pest"]
pub struct LatexParser;

pub fn pair_to_unicode(pair: Pair<'_, Rule>) -> String {
    match pair.as_rule() {
        Rule::unary => {
            let mut inner = pair.into_inner();
            let name = inner.next().unwrap().as_str();
            let first = inner.next().unwrap();
            let first_str = pair_to_unicode(first);
            UNARY
                .get(name)
                .map_or(format!("\\{}{{{}}}", name, first_str), |f| f(&first_str))
        }
        Rule::fixed_name => {
            let name = pair.as_str();
            get_fixed_value(name).unwrap_or(format!("\\{}", name))
        }
        Rule::supscript => {
            let mut inner = pair.into_inner();
            let base = pair_to_unicode(inner.next().unwrap());

            let raw_exponent = pair_to_unicode(inner.next().unwrap());
            let raw_exponent = trim_all_spaces(&raw_exponent);
            
            let (exponent, success) = supscript(&raw_exponent);
            let exponent = adjust_attach_content(&raw_exponent, &exponent, success, "^");
            format!("{}{}", base, exponent)
        }
        Rule::subscript => {
            let mut inner = pair.into_inner();
            let base = pair_to_unicode(inner.next().unwrap());
            
            let raw_index = pair_to_unicode(inner.next().unwrap());
            let raw_index = trim_all_spaces(&raw_index);

            let (index, success) = subscript(&raw_index);
            let index = adjust_attach_content(&raw_index, &index, success, "_");
            format!("{}{}", base, index)
        }
        Rule::ascii_infix => format!(" {} ", pair.as_str()),
        Rule::formula => pairs_to_unicode(pair.into_inner()),
        Rule::braced_mono => pairs_to_unicode(pair.into_inner()),
        Rule::text_braced_mono => pairs_to_unicode(pair.into_inner()),
        Rule::spaces => " ".to_string(),
        Rule::tie => " ".to_string(),
        _ => pair.as_str().to_string(),
    }
}

pub fn pairs_to_unicode(iter: Pairs<'_, Rule>) -> String {
    iter.map(pair_to_unicode)
        .reduce(|s, t| s + &t)
        .unwrap_or_default()
}

pub fn trim_all_spaces(s: &str) -> String {
    s.replace(|c| c == ' ' || c == '\t', "")
}

pub fn adjust_attach_content(raw: &str, s: &str, success: bool, mark: &str) -> String {
    match (s.chars().count(), success) {
        (1, true) => s.to_string(),
        (1, _) => format!("{}{}", mark, raw),
        (_, true) => format!("{}{{{}}}", mark, s),
        (_, _) => format!("{}{{{}}}", mark, raw),
    }
}

pub fn adjust_literal_content(s: &str) -> String {
    let trimmed = s.trim();
    match trimmed.is_empty() {
        true => " ".to_string(),
        _ => trimmed.to_string(),
    }
}

pub fn latex_unicodify(s: &str) -> String {
    match LatexParser::parse(Rule::formula, s) {
        Ok(pairs) => pairs_to_unicode(pairs),
        Err(e) => e.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use pest::Parser;

    use crate::parser::{LatexParser, Rule, latex_unicodify, pairs_to_unicode};

    #[test]
    fn parse_text() {
        assert_eq!(
            latex_unicodify(r#"\text{Before my body  is \texttt{dry}}"#),
            "Before my body is 𝚍𝚛𝚢"
        );
    }

    #[test]
    fn parse_text_math_fonts() {
        let raw = r#"\textbf{Such your blood} ~ \mathbb{Blumen kranz}"#;
        let result = LatexParser::parse(Rule::formula, raw);
        assert_eq!(
            pairs_to_unicode(result.unwrap()),
            "𝐒𝐮𝐜𝐡 𝐲𝐨𝐮𝐫 𝐛𝐥𝐨𝐨𝐝 𝔹𝕝𝕦𝕞𝕖𝕟𝕜𝕣𝕒𝕟𝕫"
        );
    }

    #[test]
    fn parse_supscript() {
        assert_eq!(latex_unicodify(r#"a^2+b^2 \ge 2ab"#), "a² + b² ≥ 2ab");
        assert_eq!(latex_unicodify(r#"e^{-x^2}, ~ \pi_4(\mathbb{S}^3)"#), "e^{-x²}, π₄(𝕊³)")
    }
}
