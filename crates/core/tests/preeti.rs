//! Corpus and rule tests for the Preeti converter.
//!
//! Every row in `data/preeti-corpus.tsv` was cross-checked against an
//! independent MIT-licensed converter before it was committed; see
//! `data/SOURCES.md`.

use npt_core::preeti::{detect, preeti_to_unicode, unicode_to_preeti, Direction};

const CORPUS: &str = include_str!("../../../data/preeti-corpus.tsv");

#[test]
fn corpus_round_trips() {
    let mut failures = Vec::new();
    let mut rows = 0;
    for line in CORPUS.lines() {
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }
        let mut parts = line.splitn(3, '\t');
        let (Some(preeti), Some(unicode)) = (parts.next(), parts.next()) else {
            panic!("bad row: {line:?}");
        };
        let mode = parts.next().unwrap_or("both");
        rows += 1;

        if mode != "unicode" {
            let got = preeti_to_unicode(preeti);
            if got.text != unicode {
                failures.push(format!("Preeti to Unicode {preeti:?}: got {:?}, want {unicode:?}", got.text));
            }
        }
        if mode != "preeti" {
            let got = unicode_to_preeti(unicode);
            if got.text != preeti {
                failures.push(format!("Unicode to Preeti {unicode:?}: got {:?}, want {preeti:?}", got.text));
            }
            // Rows marked `unicode` (mixed English) are lossy in the reverse
            // direction by nature: Latin letters are Preeti keys.
            if mode == "both" {
                let back = preeti_to_unicode(&got.text);
                if back.text != unicode {
                    failures.push(format!("round trip {unicode:?}: got {:?} through {:?}", back.text, got.text));
                }
            }
        }
    }
    assert!(rows >= 80, "corpus shrank to {rows} rows");
    assert!(failures.is_empty(), "{} failures:\n{}", failures.len(), failures.join("\n"));
}

fn pair(preeti: &str, unicode: &str) {
    let forward = preeti_to_unicode(preeti);
    assert_eq!(forward.text, unicode, "Preeti to Unicode for {preeti:?}");
    let backward = unicode_to_preeti(unicode);
    assert_eq!(backward.text, preeti, "Unicode to Preeti for {unicode:?}");
    assert_eq!(
        preeti_to_unicode(&backward.text).text,
        unicode,
        "round trip for {unicode:?}"
    );
}

#[test]
fn standalone_letters() {
    for (key, letter) in [
        ("s", "क"),
        ("g", "न"),
        ("k", "प"),
        ("/", "र"),
        ("a", "ब"),
        ("z", "श"),
        ("x", "ह"),
        ("O", "इ"),
        ("P", "ए"),
        ("p", "उ"),
        ("C", "ऋ"),
        ("cf", "आ"),
        ("cf]", "ओ"),
        ("cf}", "औ"),
    ] {
        let got = preeti_to_unicode(key);
        assert_eq!(got.text, letter, "key {key:?}");
    }
}

#[test]
fn every_matra() {
    let cases = [
        ("sf", "का"),
        ("ls", "कि"),
        ("sL", "की"),
        ("s'", "कु"),
        ("s\"", "कू"),
        ("s[", "कृ"),
        ("s]", "के"),
        ("s}", "कै"),
        ("sf]", "को"),
        ("sf}", "कौ"),
        ("s+", "कं"),
        ("sF", "कँ"),
        ("sM", "कः"),
    ];
    for (preeti, unicode) in cases {
        assert_eq!(preeti_to_unicode(preeti).text, unicode, "matra {preeti:?}");
        assert_eq!(unicode_to_preeti(unicode).text, preeti, "matra back {unicode:?}");
    }
}

#[test]
fn short_i_moves_after_the_cluster() {
    pair("ls", "कि");
    pair("lGt", "न्ति");
    pair("sflGtk'/", "कान्तिपुर");
    pair("lzIfs", "शिक्षक");
    // `sl` is the same word typed before the consonant; Unicode only has one order.
    assert_eq!(preeti_to_unicode("sl").text, "कि");
}

#[test]
fn reph_moves_before_its_consonant() {
    pair("wd{", "धर्म");
    pair("s{", "र्क");
    pair("sfo{", "कार्य");
    pair("sfo{sf", "कार्यका");
}

#[test]
fn halant_and_half_forms() {
    pair("S", "क्");
    pair("T", "त्");
    pair("kf7\\ofq", "पाठ्यात्र");
    pair("x'G", "हुन्");
    pair("cfgGb", "आनन्द");
}

#[test]
fn conjuncts() {
    pair("If", "क्ष");
    pair("If]q", "क्षेत्र");
    pair("q", "त्र");
    pair("1", "ज्ञ");
    pair("2", "द्द");
    pair("4", "द्ध");
    pair("Q", "त्त");
}

#[test]
fn digits_and_punctuation() {
    pair("!@#$%^&*()", "१२३४५६७८९०");
    pair("g]kfn.", "नेपाल।");
    pair("g]kfn, sf7df08\"", "नेपाल, काठमाण्डू");
}

#[test]
fn mixed_text_keeps_english() {
    let got = unicode_to_preeti("नेपाल Nepal 123");
    assert_eq!(got.text, "g]kfn Nepal 123");
    // Latin letters are Preeti keys, so the reverse direction reads them as
    // Nepali. That is the encoding, not a bug, and the panel says which way it
    // is converting.
    assert!(preeti_to_unicode(&got.text).text.starts_with("नेपाल"));
}

#[test]
fn names_round_trip() {
    // The reference converter writes the reph as `{`; both `/\o` and `o{` are
    // valid Preeti for आचार्य, and the reph form is the one this app emits.
    assert_eq!(preeti_to_unicode(";lag cfrf/\\o").text, "सबिन आचार्य");
    pair(";lag cfrfo{", "सबिन आचार्य");
    pair("cfo'if", "आयुष");
    pair("d];f]", "मेसो");
}

#[test]
fn ra_foot_and_ru_glyphs() {
    // `K/` and `/'` map correctly but the font draws them with a visible
    // halant or a detached matra, so the canonical keys are the only ones that
    // look right.
    pair("k|", "प्र");
    pair("s|d", "क्रम");
    pair("u|x", "ग्रह");
    pair("a|Xd", "ब्रह्म");
    pair("/fi6«", "राष्ट्र");
    pair("?", "रु");
    pair("¿", "रू");
    pair("?k}ofF", "रुपैयाँ");
    pair("lk|", "प्रि");
    pair("l6«", "ट्रि");
}

#[test]
fn colon_visarga_and_visual_matra_order() {
    // A bare M is a colon; an M closing a word is a visarga.
    assert_eq!(preeti_to_unicode("M").text, ":");
    assert_eq!(preeti_to_unicode("g]kfn M").text, "नेपाल:");
    assert_eq!(preeti_to_unicode("k'gM").text, "पुनः");
    // The bindu must move behind the matra before the vowels combine.
    assert_eq!(preeti_to_unicode("cf+}").text, "औं");
    assert_eq!(preeti_to_unicode(";ftf+}").text, "सातौं");
    // Matras typed in visual order still make the right vowel.
    assert_eq!(preeti_to_unicode("ad]flhd").text, "बमोजिम");
}

#[test]
fn punctuation_uses_preeti_keys() {
    assert_eq!(unicode_to_preeti("नेपाल/काठमाडौं").text, "g]kfn÷sf7df8f}+");
    assert_eq!(unicode_to_preeti("२०%").text, "@)Ü");
    assert_eq!(unicode_to_preeti("(क)").text, "-s_");
    assert_eq!(unicode_to_preeti("१+२").text, "!±@");
    assert_eq!(unicode_to_preeti("नमस्ते।").text, "gd:t].");
    assert_eq!(unicode_to_preeti("२०८३").text, "@)*#");
    assert_eq!(preeti_to_unicode("-s_").text, "(क)");
}

#[test]
fn half_pha_uses_the_joined_glyph() {
    // The fonts draw `km\` with a visible halant; their own documents use the
    // private-use half glyph, which joins into आफ्नो.
    assert_eq!(unicode_to_preeti("आफ्नो").text, "cf\u{f000}gf]");
    assert_eq!(unicode_to_preeti("आफ्नै").text, "cf\u{f000}g}");
    pair("cf\u{f000}gf]", "आफ्नो");
}

#[test]
fn legacy_variants_from_a_real_document() {    for (preeti, unicode) in [
        (";}¢flGts", "सैद्धान्तिक"),
        ("sÝf", "कठ्ठा"),
        ("gl;{·", "नर्सिङ"),
        ("gl;{∙", "नर्सिङ"),
        (":sÒn", "स्कुल"),
        ("s]G›", "केन्द्र"),
        ("dxŒj", "महत्व"),
        ("6ØfS6/", "ट्याक्टर"),
        ("cfˆgf]", "आफ्नो"),
        ("k'¥ofpg", "पुर्याउन"),
        ("k|mfO{", "फ्राई"),
    ] {
        assert_eq!(preeti_to_unicode(preeti).text, unicode, "variant {preeti:?}");
    }
}

#[test]
fn unrepresentable_characters_are_reported() {
    // ॲ has no Preeti key.
    let got = unicode_to_preeti("नेपाल ॲ");
    assert!(got.text.contains('ॲ'), "character must survive: {:?}", got.text);
    assert_eq!(got.unconverted, vec!['ॲ']);

    // A currency sign is not Preeti either.
    let got = preeti_to_unicode("g]kfn €");
    assert!(got.text.contains('€'), "character must survive: {:?}", got.text);
    assert_eq!(got.unconverted, vec!['€']);
}

#[test]
fn direction_is_chosen_by_script() {
    assert_eq!(detect("g]kfn"), Direction::ToUnicode);
    assert_eq!(detect("नेपाल"), Direction::ToPreeti);
    assert_eq!(detect("Nepal 123"), Direction::ToUnicode);
    assert_eq!(detect("नेपाल Nepal"), Direction::ToPreeti);
}
