pub const fn char_to_ascii_offset(c: char) -> Option<u32> {
    if c.is_ascii_uppercase() {
        return Some(c as u32 - 'A' as u32);
    } else if c.is_ascii_lowercase() {
        return Some((26 as u32 + c as u32) - 'a' as u32);
    } else if c.is_ascii_digit() {
        return Some((52 as u32 + c as u32) - '0' as u32);
    }
    return None;
}

// The following unicode data is exported from UniTeX.ts.
// Length information from:
//  - 52 = 26 + 26
//  - 62 = 26 + 26 + 10

pub const DOUBLE_STRUCK: [char; 62] = [
    '𝔸', '𝔹', 'ℂ', '𝔻', '𝔼', '𝔽', '𝔾', 'ℍ', '𝕀', '𝕁', '𝕂', '𝕃', '𝕄', 'ℕ', '𝕆', 'ℙ', 'ℚ', 'ℝ', '𝕊',
    '𝕋', '𝕌', '𝕍', '𝕎', '𝕏', '𝕐', 'ℤ', '𝕒', '𝕓', '𝕔', '𝕕', '𝕖', '𝕗', '𝕘', '𝕙', '𝕚', '𝕛', '𝕜', '𝕝',
    '𝕞', '𝕟', '𝕠', '𝕡', '𝕢', '𝕣', '𝕤', '𝕥', '𝕦', '𝕧', '𝕨', '𝕩', '𝕪', '𝕫', '𝟘', '𝟙', '𝟚', '𝟛', '𝟜',
    '𝟝', '𝟞', '𝟟', '𝟠', '𝟡',
];

pub const BOLD: [char; 52] = [
    '𝐀', '𝐁', '𝐂', '𝐃', '𝐄', '𝐅', '𝐆', '𝐇', '𝐈', '𝐉', '𝐊', '𝐋', '𝐌', '𝐍', '𝐎', '𝐏', '𝐐', '𝐑', '𝐒',
    '𝐓', '𝐔', '𝐕', '𝐖', '𝐗', '𝐘', '𝐙', '𝐚', '𝐛', '𝐜', '𝐝', '𝐞', '𝐟', '𝐠', '𝐡', '𝐢', '𝐣', '𝐤', '𝐥',
    '𝐦', '𝐧', '𝐨', '𝐩', '𝐪', '𝐫', '𝐬', '𝐭', '𝐮', '𝐯', '𝐰', '𝐱', '𝐲', '𝐳',
];

pub const ITALIC: [char; 52] = [
    '𝐴', '𝐵', '𝐶', '𝐷', '𝐸', '𝐹', '𝐺', '𝐻', '𝐼', '𝐽', '𝐾', '𝐿', '𝑀', '𝑁', '𝑂', '𝑃', '𝑄', '𝑅', '𝑆',
    '𝑇', '𝑈', '𝑉', '𝑊', '𝑋', '𝑌', '𝑍', '𝑎', '𝑏', '𝑐', '𝑑', '𝑒', '𝑓', '𝑔', 'h', '𝑖', '𝑗', '𝑘', '𝑙',
    '𝑚', '𝑛', '𝑜', '𝑝', '𝑞', '𝑟', '𝑠', '𝑡', '𝑢', '𝑣', '𝑤', '𝑥', '𝑦', '𝑧',
];

pub const SCRIPT: [char; 52] = [
    '𝒜', 'ℬ', '𝒞', '𝒟', 'ℰ', 'ℱ', '𝒢', 'ℋ', 'ℐ', '𝒥', '𝒦', 'ℒ', 'ℳ', '𝒩', '𝒪', '𝒫', '𝒬', 'ℛ', '𝒮',
    '𝒯', '𝒰', '𝒱', '𝒲', '𝒳', '𝒴', '𝒵', '𝒶', '𝒷', '𝒸', '𝒹', 'ℯ', '𝒻', 'g', '𝒽', '𝒾', '𝒿', '𝓀', '𝓁',
    '𝓂', '𝓃', 'ℴ', '𝓅', '𝓆', '𝓇', '𝓈', '𝓉', '𝓊', '𝓋', '𝓌', '𝓍', '𝓎', '𝓏',
];

pub const MONOSPACE: [char; 62] = [
    '𝙰', '𝙱', '𝙲', '𝙳', '𝙴', '𝙵', '𝙶', '𝙷', '𝙸', '𝙹', '𝙺', '𝙻', '𝙼', '𝙽', '𝙾', '𝙿', '𝚀', '𝚁', '𝚂',
    '𝚃', '𝚄', '𝚅', '𝚆', '𝚇', '𝚈', '𝚉', '𝚊', '𝚋', '𝚌', '𝚍', '𝚎', '𝚏', '𝚐', '𝚑', '𝚒', '𝚓', '𝚔', '𝚕',
    '𝚖', '𝚗', '𝚘', '𝚙', '𝚚', '𝚛', '𝚜', '𝚝', '𝚞', '𝚟', '𝚠', '𝚡', '𝚢', '𝚣', '𝟶', '𝟷', '𝟸', '𝟹', '𝟺',
    '𝟻', '𝟼', '𝟽', '𝟾', '𝟿',
];

pub const SANS_SERIF: [char; 62] = [
    '𝖠', '𝖡', '𝖢', '𝖣', '𝖤', '𝖥', '𝖦', '𝖧', '𝖨', '𝖩', '𝖪', '𝖫', '𝖬', '𝖭', '𝖮', '𝖯', '𝖰', '𝖱', '𝖲',
    '𝖳', '𝖴', '𝖵', '𝖶', '𝖷', '𝖸', '𝖹', '𝖺', '𝖻', '𝖼', '𝖽', '𝖾', '𝖿', '𝗀', '𝗁', '𝗂', '𝗃', '𝗄', '𝗅',
    '𝗆', '𝗇', '𝗈', '𝗉', '𝗊', '𝗋', '𝗌', '𝗍', '𝗎', '𝗏', '𝗐', '𝗑', '𝗒', '𝗓', '𝟢', '𝟣', '𝟤', '𝟥', '𝟦',
    '𝟧', '𝟨', '𝟩', '𝟪', '𝟫',
];

pub fn double_struck(s: &str) -> String {
    render_if_exist_62(s, DOUBLE_STRUCK)
}

pub fn bold(s: &str) -> String {
    render_if_exist_52(s, BOLD)
}

pub fn italic(s: &str) -> String {
    render_if_exist_52(s, ITALIC)
}

pub fn script(s: &str) -> String {
    render_if_exist_52(s, SCRIPT)
}

pub fn calligraphic(s: &str) -> String {
    render_if_exist_52(s, SCRIPT)
}

pub fn monospace(s: &str) -> String {
    render_if_exist_62(s, MONOSPACE)
}

pub fn sans_serif(s: &str) -> String {
    render_if_exist_62(s, SANS_SERIF)
}

pub fn render_if_exist_52(s: &str, alphabet: [char; 52]) -> String {
    s.chars()
        .map(|c| char_to_ascii_offset(c).map_or(c, |i| alphabet[i as usize]))
        .collect()
}

pub fn render_if_exist_62(s: &str, alphabet: [char; 62]) -> String {
    s.chars()
        .map(|c| char_to_ascii_offset(c).map_or(c, |i| alphabet[i as usize]))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::typeface::{DOUBLE_STRUCK, char_to_ascii_offset};

    #[test]
    fn test_char_to_ascii_offset() {
        assert_eq!(char_to_ascii_offset('K'), Some(10));
        assert_eq!(char_to_ascii_offset('k'), Some(36));
        assert_eq!(char_to_ascii_offset('7'), Some(59));
        assert_eq!(char_to_ascii_offset('?'), None);
    }

    #[test]
    fn test_double_struck() {
        assert_eq!(
            DOUBLE_STRUCK[char_to_ascii_offset('K').unwrap() as usize],
            '𝕂'
        );
        assert_eq!(
            DOUBLE_STRUCK[char_to_ascii_offset('k').unwrap() as usize],
            '𝕜'
        );
        assert_eq!(
            DOUBLE_STRUCK[char_to_ascii_offset('7').unwrap() as usize],
            '𝟟'
        );
    }
}
