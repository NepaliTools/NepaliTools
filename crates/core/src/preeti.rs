//! Preeti (legacy ASCII font encoding) and Unicode Devanagari.
//!
//! Preeti stores Devanagari glyphs on ordinary ASCII keys, in visual order. The
//! conversion is therefore not a character swap: the short-i sign is typed before
//! its consonant, the reph is typed after its consonant, independent vowels are
//! built from `c` plus a matra, and a handful of keys stand for whole conjuncts.
//!
//! The table is factual key-layout data, vendored in `data/preeti-map.tsv` and
//! provenance recorded in `data/SOURCES.md`. Contextual rules live here.

use std::collections::HashMap;
use std::sync::OnceLock;

const MAP: &str = include_str!("../../../data/preeti-map.tsv");

/// Text produced by a conversion, plus any characters the target encoding
/// cannot represent. Those characters are left untouched in `text`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Conversion {
    pub text: String,
    pub unconverted: Vec<char>,
}

impl Conversion {
    fn new(text: String, unconverted: Vec<char>) -> Self {
        Conversion { text, unconverted }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    /// Input has Devanagari, so it is converted to Preeti keys.
    ToPreeti,
    /// Input has no Devanagari, so it is read as Preeti keys.
    ToUnicode,
}

/// A selection with any Devanagari in it is Unicode text. Preeti is ASCII, so
/// this never mistakes one for the other.
pub fn detect(input: &str) -> Direction {
    if input.chars().any(is_devanagari) {
        Direction::ToPreeti
    } else {
        Direction::ToUnicode
    }
}

pub fn is_devanagari(ch: char) -> bool {
    ('\u{0900}'..='\u{097f}').contains(&ch)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    Consonant,
    Matra,
    Vowel,
    Other,
}

#[derive(Debug, Clone)]
enum Atom {
    Text(&'static str, Kind),
    /// A character with no mapping, kept as it arrived.
    Unknown(char),
    /// `m`, which turns a few preceding letters into another glyph.
    Modifier,
    /// `{`, the reph, typed after its consonant.
    Reph,
}

fn table() -> &'static HashMap<char, &'static str> {
    static TABLE: OnceLock<HashMap<char, &'static str>> = OnceLock::new();
    TABLE.get_or_init(|| {
        let mut map = HashMap::new();
        for line in MAP.lines() {
            if is_comment(line) || line.trim().is_empty() {
                continue;
            }
            let mut parts = line.splitn(2, '\t');
            let Some(key) = parts.next() else { continue };
            let Some(value) = parts.next() else { continue };
            let mut chars = key.chars();
            let (Some(ch), None) = (chars.next(), chars.next()) else {
                continue;
            };
            map.insert(ch, value);
        }
        map
    })
}

/// A row is data unless it is the header. `#` is itself a Preeti key, so a
/// bare `#` check would drop the digit ३.
fn is_comment(line: &str) -> bool {
    line.starts_with("# ") || line.starts_with("##")
}

fn kind_of(value: &str) -> Kind {    let mut has_matras = false;
    let mut has_consonant = false;
    for ch in value.chars() {
        if ch == '\u{094d}' {
            continue;
        }
        if ('\u{093e}'..='\u{094c}').contains(&ch) || ('\u{0901}'..='\u{0903}').contains(&ch) {
            has_matras = true;
        }
        if ('\u{0915}'..='\u{0939}').contains(&ch) {
            has_consonant = true;
        }
    }
    if has_matras {
        Kind::Matra
    } else if has_consonant {
        Kind::Consonant
    } else if ('\u{0904}'..='\u{0914}').contains(&value.chars().next().unwrap_or(' ')) {
        Kind::Vowel
    } else {
        Kind::Other
    }
}

// ---------------------------------------------------------------- Preeti to Unicode

pub fn preeti_to_unicode(input: &str) -> Conversion {
    let mut atoms = Vec::with_capacity(input.chars().count());
    let mut unconverted = Vec::new();
    for ch in input.chars() {
        match ch {
            'm' => atoms.push(Atom::Modifier),
            '{' => atoms.push(Atom::Reph),
            _ => match table().get(&ch) {
                Some(value) => atoms.push(Atom::Text(value, kind_of(value))),
                None => {
                    atoms.push(Atom::Unknown(ch));
                    if !ch.is_whitespace() && !ch.is_ascii() {
                        unconverted.push(ch);
                    }
                }
            },
        }
    }

    resolve_modifiers(&mut atoms);
    let with_reph = resolve_reph(atoms);
    let reordered = resolve_short_i(with_reph);

    let mut text: Vec<char> = Vec::with_capacity(input.len());
    for atom in reordered {
        match atom {
            Atom::Text(value, _) => text.extend(value.chars()),
            Atom::Unknown(ch) => text.push(ch),
            Atom::Modifier => {
                text.push('m');
                unconverted.push('m');
            }
            Atom::Reph => text.extend("र्".chars()),
        }
    }

    let text = clean(text);
    Conversion::new(text, unconverted)
}

/// `m` turns the preceding letter into another glyph: `km` is फ, `pm` is ऊ,
/// `em` is झ, `qm` is क्र and `Qm` is क्त. On any other letter it has no
/// meaning, so it stays a literal m.
fn resolve_modifiers(atoms: &mut Vec<Atom>) {
    let mut index = 0;
    while index < atoms.len() {
        if !matches!(atoms[index], Atom::Modifier) {
            index += 1;
            continue;
        }
        let previous = (0..index).rev().find(|&j| {
            matches!(
                atoms[j],
                Atom::Text(_, Kind::Consonant) | Atom::Text(_, Kind::Vowel)
            )
        });
        let replacement = previous.and_then(|j| match atoms[j] {
            Atom::Text(text, _) => match text {
                "प" => Some("फ"),
                "भ" => Some("झ"),
                "उ" => Some("ऊ"),
                "त्र" => Some("क्र"),
                "त्त" => Some("क्त"),
                _ => None,
            },
            _ => None,
        });
        match (previous, replacement) {
            (Some(j), Some(text)) => {
                atoms[j] = Atom::Text(text, Kind::Consonant);
                atoms.remove(index);
            }
            _ => {
                atoms[index] = Atom::Unknown('m');
                index += 1;
            }
        }
    }
}

/// `{` is the reph, typed after the consonant it sits above, and written before
/// that consonant in Unicode. `O{` is the one special case: it is ई.
fn resolve_reph(atoms: Vec<Atom>) -> Vec<Atom> {
    let mut out = Vec::with_capacity(atoms.len() + 1);
    for atom in atoms {
        if matches!(atom, Atom::Reph) {
            let position = out.iter().rposition(|item| {
                matches!(
                    item,
                    Atom::Text(_, Kind::Consonant) | Atom::Text(_, Kind::Vowel)
                )
            });
            if let Some(position) = position {
                if matches!(out[position], Atom::Text("इ", Kind::Vowel)) {
                    out[position] = Atom::Text("ई", Kind::Vowel);
                    continue;
                }
                out.insert(position, Atom::Text("र्", Kind::Matra));
            } else {
                out.push(Atom::Text("र्", Kind::Matra));
            }
        } else {
            out.push(atom);
        }
    }
    out
}

/// The short-i is typed before its consonant but belongs after the whole cluster,
/// so `lGt` is न्ति, not िन्त.
fn resolve_short_i(atoms: Vec<Atom>) -> Vec<Atom> {
    let mut out = Vec::with_capacity(atoms.len());
    let mut index = 0;
    while index < atoms.len() {
        if !matches!(atoms[index], Atom::Text("ि", Kind::Matra)) {
            out.push(atoms[index].clone());
            index += 1;
            continue;
        }
        let next = (index + 1..atoms.len())
            .find(|&j| matches!(atoms[j], Atom::Text(_, Kind::Consonant)));
        let Some(mut last) = next else {
            out.push(Atom::Text("ि", Kind::Matra));
            index += 1;
            continue;
        };
        for item in atoms.iter().take(last + 1).skip(index + 1) {
            out.push(item.clone());
        }
        // Keep consuming the halant chain: न् + त is one cluster.
        while ends_with_halant(&out) {
            match atoms.get(last + 1) {
                Some(Atom::Text(_, Kind::Consonant)) => {
                    last += 1;
                    out.push(atoms[last].clone());
                }
                _ => break,
            }
        }
        // The ra-foot belongs to the cluster, so प्रि keeps ि after प्र.
        if matches!(atoms.get(last + 1), Some(Atom::Text("\u{094d}\u{0930}", _))) {
            last += 1;
            out.push(atoms[last].clone());
        }
        // A half form plus आ is the full letter (क्ष् + ा is क्ष), and the
        // short-i belongs after that letter, not between the two keys.
        if ends_with_halant(&out) {
            if let Some(Atom::Text("ा", _)) = atoms.get(last + 1) {
                last += 1;
                out.push(atoms[last].clone());
            }
        }
        out.push(Atom::Text("ि", Kind::Matra));
        index = last + 1;
    }
    out
}

fn ends_with_halant(atoms: &[Atom]) -> bool {
    matches!(atoms.last(), Some(Atom::Text(text, _)) if text.ends_with('\u{094d}'))
}

fn clean(chars: Vec<char>) -> String {
    let chars = drop_dangling_aa(chars);
    let chars = merge_vowels(chars);
    let chars = move_bindu(chars);
    let chars = dedupe(chars);
    chars.into_iter().collect()
}

/// `\f` is a half letter followed by ā. Together they are the full letter, so
/// both keys collapse: ण् + ा becomes ण, and the same trick carries ष and क्ष.
fn drop_dangling_aa(chars: Vec<char>) -> Vec<char> {
    let mut out = Vec::with_capacity(chars.len());
    let mut index = 0;
    while index < chars.len() {
        if chars[index] == '\u{094d}' && chars.get(index + 1) == Some(&'\u{093e}') {
            index += 2;
        } else {
            out.push(chars[index]);
            index += 1;
        }
    }
    out
}

fn merge_vowels(chars: Vec<char>) -> Vec<char> {
    const RULES: [(&str, char); 7] = [
        ("अाे", 'ओ'),
        ("अाै", 'औ'),
        ("अा", 'आ'),
        ("एै", 'ऐ'),
        ("एे", 'ऐ'),
        ("ाे", 'ो'),
        ("ाै", 'ौ'),
    ];
    let mut out = chars;
    for (from, to) in RULES {
        out = replace(&out, from, to);
    }
    out
}

/// ं and ँ sit after the vowel sign in Unicode even when they were typed before it.
fn move_bindu(chars: Vec<char>) -> Vec<char> {
    const MATRAS: [char; 10] = [
        '\u{093e}', '\u{093f}', '\u{0940}', '\u{0941}', '\u{0942}', '\u{0943}', '\u{0947}',
        '\u{0948}', '\u{094b}', '\u{094c}',
    ];
    let mut out = Vec::with_capacity(chars.len());
    let mut index = 0;
    while index < chars.len() {
        let ch = chars[index];
        if (ch == '\u{0902}' || ch == '\u{0901}') && chars.get(index + 1).is_some_and(|next| MATRAS.contains(next)) {
            out.push(chars[index + 1]);
            out.push(ch);
            index += 2;
        } else {
            out.push(ch);
            index += 1;
        }
    }
    out
}

fn dedupe(chars: Vec<char>) -> Vec<char> {
    let mut out: Vec<char> = Vec::with_capacity(chars.len());
    for ch in chars {
        if matches!(ch, '\u{0901}' | '\u{0902}' | '\u{0947}' | '\u{0948}' | '\u{0941}' | '\u{0942}')
            && out.last() == Some(&ch)
        {
            continue;
        }
        out.push(ch);
    }
    out
}

fn replace(chars: &[char], from: &str, to: char) -> Vec<char> {
    let from: Vec<char> = from.chars().collect();
    let mut out = Vec::with_capacity(chars.len());
    let mut index = 0;
    while index < chars.len() {
        if chars[index..].starts_with(&from) {
            out.push(to);
            index += from.len();
        } else {
            out.push(chars[index]);
            index += 1;
        }
    }
    out
}

// ---------------------------------------------------------------- Unicode to Preeti

fn reverse_table() -> &'static HashMap<&'static str, char> {
    static TABLE: OnceLock<HashMap<&'static str, char>> = OnceLock::new();
    TABLE.get_or_init(|| {
        let mut map = HashMap::new();
        for line in MAP.lines() {
            if is_comment(line) || line.trim().is_empty() {
                continue;
            }
            let mut parts = line.splitn(2, '\t');
            let Some(key) = parts.next() else { continue };
            let Some(value) = parts.next() else { continue };
            let Some(ch) = key.chars().next() else { continue };
            // First row wins: the table lists canonical keys before extras.
            map.entry(value).or_insert(ch);
        }
        map
    })
}

/// Whole conjuncts that have a single dedicated key.
fn ligatures() -> &'static Vec<(&'static str, char)> {
    static LIGATURES: OnceLock<Vec<(&'static str, char)>> = OnceLock::new();
    LIGATURES.get_or_init(|| {
        let mut list: Vec<(&'static str, char)> = Vec::new();
        list.extend(
            reverse_table()
                .iter()
                .filter(|(value, _)| {
                    let count = value.chars().count();
                    count > 1 && kind_of(value) == Kind::Consonant && !value.ends_with('\u{094d}')
                })
                .map(|(value, key)| (*value, *key)),
        );
        list.sort_by_key(|(value, _)| std::cmp::Reverse(value.chars().count()));
        list
    })
}

fn full_of(ch: char) -> Option<String> {
    match ch {
        // Letters Preeti only has as half forms; आ folds the half back to full.
        'ण' => return Some("0f".into()),
        'ष' => return Some("if".into()),
        _ => {}
    }
    reverse_table()
        .get(ch.to_string().as_str())
        .map(|key| key.to_string())
}

fn half_of(ch: char) -> String {
    let halant = format!("{ch}\u{094d}");
    if let Some(key) = reverse_table().get(halant.as_str()) {
        return key.to_string();
    }
    match full_of(ch) {
        Some(full) => format!("{full}\\"),
        None => String::new(),
    }
}

pub fn unicode_to_preeti(input: &str) -> Conversion {
    let chars: Vec<char> = input.chars().collect();
    let mut out = String::with_capacity(input.len());
    let mut unconverted = Vec::new();
    let mut index = 0;

    while index < chars.len() {
        let ch = chars[index];

        // रु and रू have their own glyphs in Preeti; र plus a matra does not
        // join into them, so `/'` renders as two pieces.
        if ch == '\u{0930}' {
            match chars.get(index + 1) {
                Some('\u{0941}') => {
                    out.push('?');
                    index += 2;
                    continue;
                }
                Some('\u{0942}') => {
                    out.push('\u{00bf}');
                    index += 2;
                    continue;
                }
                _ => {}
            }
        }

        // Reph: र + ् + consonant becomes consonant + `{`.
        if ch == '\u{0930}' && chars.get(index + 1) == Some(&'\u{094d}') {            match chars.get(index + 2) {
                Some(&next) if is_consonant(next) => {
                    let (cluster, next_index) = read_cluster(&chars, index + 2);
                    out.push_str(&cluster);
                    out.push('{');
                    index = next_index;
                    continue;
                }
                _ => {
                    out.push('{');
                    index += 2;
                    continue;
                }
            }
        }

        if is_consonant(ch) {
            let (cluster, next_index) = read_cluster(&chars, index);
            // The short-i is typed before the cluster it follows.
            if chars.get(next_index) == Some(&'\u{093f}') {
                out.push('l');
                out.push_str(&cluster);
                index = next_index + 1;
            } else {
                out.push_str(&cluster);
                index = next_index;
            }
            continue;
        }

        match ch {
            '\u{093e}' => out.push('f'),
            '\u{093f}' => out.push('l'),
            '\u{0940}' => out.push('L'),
            '\u{0941}' => out.push('\''),
            '\u{0942}' => out.push('"'),
            '\u{0943}' => out.push('['),
            '\u{0947}' => out.push(']'),
            '\u{0948}' => out.push('}'),
            '\u{094b}' => out.push_str("f]"),
            '\u{094c}' => out.push_str("f}"),
            '\u{0902}' => out.push('+'),
            '\u{0901}' => out.push('F'),
            '\u{0903}' => out.push('M'),
            _ => match ch.to_string().as_str() {
                "आ" => out.push_str("cf"),
                "इ" => out.push('O'),
                "ई" => out.push_str("O{"),
                "उ" => out.push('p'),
                "ऊ" => out.push_str("pm"),
                "ऋ" => out.push('C'),
                "ए" => out.push('P'),
                "ऐ" => out.push_str("P}"),
                "ओ" => out.push_str("cf]"),
                "औ" => out.push_str("cf}"),
                _ => match reverse_table().get(ch.to_string().as_str()) {
                    Some(key) => out.push(*key),
                    None => {
                        out.push(ch);
                        if is_devanagari(ch) {
                            unconverted.push(ch);
                        }
                    }
                },
            },
        }
        index += 1;
    }

    Conversion::new(out, unconverted)
}

fn is_consonant(ch: char) -> bool {
    ('\u{0915}'..='\u{0939}').contains(&ch)
}

/// Reads a consonant cluster: a consonant, then any `् + consonant` chain, plus
/// the same for a dedicated conjunct key (`क्ष`, `त्र`).
fn read_cluster(chars: &[char], start: usize) -> (String, usize) {
    // क्ष has no full-letter key: `I` is only the half form, and `If` is the
    // full one, so the आ comes along.
    if chars[start] == 'क' && chars.get(start + 1) == Some(&'\u{094d}') && chars.get(start + 2) == Some(&'ष') {
        return ("If".to_string(), start + 3);
    }
    for (value, key) in ligatures() {
        let length = value.chars().count();
        if chars.len() >= start + length
            && chars[start..start + length].iter().copied().eq(value.chars())
        {
            return (key.to_string(), start + length);
        }
    }
    let ch = chars[start];
    if chars.get(start + 1) == Some(&'\u{094d}') {
        // A consonant plus र is the ra-foot, not a half form followed by a
        // separate र. Retroflex letters take their own foot glyph; the plain
        // `K/` form leaves a visible halant in the font.
        if chars.get(start + 2) == Some(&'\u{0930}') {
            let foot = if matches!(ch, 'ट' | 'ठ' | 'ड' | 'ढ') { '\u{00ab}' } else { '|' };
            let base = full_of(ch).unwrap_or_else(|| ch.to_string());
            return (format!("{base}{foot}"), start + 3);
        }
        if let Some(&next) = chars.get(start + 2) {
            if is_consonant(next) {
                let (rest, end) = read_cluster(chars, start + 2);
                return (format!("{}{}", half_of(ch), rest), end);
            }
        }
        // A halant with nothing to join is still a half form: हुन् ends in न्.
        return (half_of(ch), start + 2);
    }
    match full_of(ch) {
        Some(key) => (key, start + 1),
        None => (ch.to_string(), start + 1),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_every_table_row() {
        for line in MAP.lines() {
            if is_comment(line) || line.trim().is_empty() {
                continue;
            }
            let (key, value) = line.split_once('\t').expect("row has two columns");
            let ch = key.chars().next().expect("row has a key");
            assert_eq!(table().get(&ch), Some(&value), "row {line:?}");
        }
    }

    #[test]
    fn direction_follows_script() {
        assert_eq!(detect("g]kfn"), Direction::ToUnicode);
        assert_eq!(detect("नेपाल"), Direction::ToPreeti);
        assert_eq!(detect("Nepal 123"), Direction::ToUnicode);
    }
}
