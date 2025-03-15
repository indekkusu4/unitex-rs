use phf::phf_map;

use crate::typeface::{bold, calligraphic, double_struck, italic, monospace, sans_serif, script};

pub static UNARY: phf::Map<&'static str, fn(&str) -> String> = phf_map! {
    "mathbb" => double_struck,
    "mathbf" => bold,
    "textbf" => bold,
    "mathit" => italic,
    "textit" => italic,
    "mathscr" => script,
    "mathcal" => calligraphic,
    "mathsf" => sans_serif, 
    "textsf" => sans_serif, 
    "mathtt" => monospace, 
    "texttt" => monospace, 
    "text" => text
};

pub fn text(s: &str) -> String {
    s.to_string()
}

pub fn subscript(s: &str) -> (String, bool) {
    render_if_exist_phf(s, &SUBSCRIPT)
}

pub fn supscript(s: &str) -> (String, bool) {
    render_if_exist_phf(s, &SUPSCRIPT)
}

pub fn render_if_exist_phf(s: &str, phf: &phf::Map<char, char>) -> (String, bool) {
    s.chars()
        .map(|c| phf.get(&c).map_or((c, false), |u| (*u, true)))
        .fold((String::new(), true), |s, t| {
            (s.0 + &t.0.to_string(), s.1 && t.1)
        })
}

pub static SUBSCRIPT: phf::Map<char, char> = phf_map! {
    '0' => '₀', 
    '1' => '₁', 
    '2' => '₂', 
    '3' => '₃', 
    '4' => '₄', 
    '5' => '₅', 
    '6' => '₆', 
    '7' => '₇', 
    '8' => '₈', 
    '9' => '₉', 
    'a' => 'ₐ', 
    'e' => 'ₑ', 
    'h' => 'ₕ', 
    'i' => 'ᵢ', 
    'j' => 'ⱼ', 
    'k' => 'ₖ', 
    'l' => 'ₗ', 
    'm' => 'ₘ', 
    'n' => 'ₙ', 
    'o' => 'ₒ', 
    'p' => 'ₚ', 
    'r' => 'ᵣ', 
    's' => 'ₛ', 
    't' => 'ₜ', 
    'u' => 'ᵤ', 
    'v' => 'ᵥ', 
    'x' => 'ₓ', 
    '+' => '₊', 
    '-' => '₋', 
    '=' => '₌', 
    '(' => '₍', 
    ')' => '₎', 
};

pub static SUPSCRIPT: phf::Map<char, char> = phf_map! {
    '0' => '⁰',
    '1' => '¹',
    '2' => '²',
    '3' => '³',
    '4' => '⁴',
    '5' => '⁵',
    '6' => '⁶',
    '7' => '⁷',
    '8' => '⁸',
    '9' => '⁹',
    'a' => 'ᵃ',
    'b' => 'ᵇ',
    'c' => 'ᶜ',
    'd' => 'ᵈ',
    'e' => 'ᵉ',
    'f' => 'ᶠ',
    'g' => 'ᵍ',
    'h' => 'ʰ',
    'j' => 'ʲ',
    'k' => 'ᵏ',
    'l' => 'ˡ',
    'm' => 'ᵐ',
    'n' => 'ⁿ',
    'o' => 'ᵒ',
    'p' => 'ᵖ',
    'r' => 'ʳ',
    's' => 'ˢ',
    't' => 'ᵗ',
    'u' => 'ᵘ',
    'v' => 'ᵛ',
    'w' => 'ʷ',
    'x' => 'ˣ',
    'y' => 'ʸ',
    'z' => 'ᶻ',
    '+' => '⁺',
    '-' => '⁻',
    '=' => '⁼',
    '(' => '⁽',
    ')' => '⁾',
    'A' => 'ᴬ',
    'B' => 'ᴮ',
    'D' => 'ᴰ',
    'E' => 'ᴱ',
    'G' => 'ᴳ',
    'H' => 'ᴴ',
    'I' => 'ᴵ',
    'J' => 'ᴶ',
    'K' => 'ᴷ',
    'L' => 'ᴸ',
    'M' => 'ᴹ',
    'N' => 'ᴺ',
    'α' => 'ᵅ',
    '′' => '′',
};

#[cfg(test)]
mod tests {
    use super::UNARY;

    #[test]
    fn test_unary_mathbb() {
        assert_eq!(
            UNARY.get("mathbb").unwrap()("Don't Lose Your Way"),
            "𝔻𝕠𝕟'𝕥 𝕃𝕠𝕤𝕖 𝕐𝕠𝕦𝕣 𝕎𝕒𝕪"
        )
    }
}
