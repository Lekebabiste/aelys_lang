mod common;
use common::*;

#[test]
fn string_len_basic() {
    let code = r#"
needs std.string
string.len("hello")
"#;
    assert_aelys_int(code, 5);
}

#[test]
fn string_len_empty() {
    let code = r#"
needs std.string
string.len("")
"#;
    assert_aelys_int(code, 0);
}

#[test]
fn string_char_len_ascii() {
    let code = r#"
needs std.string
string.char_len("hello")
"#;
    assert_aelys_int(code, 5);
}

#[test]
fn string_char_len_unicode() {
    let code = r#"
needs std.string
string.char_len("héllo")
"#;
    assert_aelys_int(code, 5);
}

#[test]
fn string_char_at_valid() {
    let code = r#"
needs std.string
let c = string.char_at("hello", 1)
42
"#;
    assert_aelys_int(code, 42);
}

#[test]
fn string_char_at_negative() {
    let code = r#"
needs std.string
let c = string.char_at("hello", -1)
42
"#;
    assert_aelys_int(code, 42);
}

#[test]
fn string_char_at_out_of_bounds() {
    let code = r#"
needs std.string
let c = string.char_at("hello", 100)
42
"#;
    assert_aelys_int(code, 42);
}

#[test]
fn string_byte_at_valid() {
    let code = r#"
needs std.string
string.byte_at("ABC", 0)
"#;
    assert_aelys_int(code, 65); // 'A'
}

#[test]
fn string_byte_at_out_of_bounds() {
    let code = r#"
needs std.string
string.byte_at("hi", 10)
"#;
    assert_aelys_int(code, -1);
}

#[test]
fn string_byte_at_negative() {
    let code = r#"
needs std.string
string.byte_at("test", -1)
"#;
    assert_aelys_int(code, -1);
}

#[test]
fn string_substr_basic() {
    let code = r#"
needs std.string
let s = string.substr("hello world", 0, 5)
42
"#;
    assert_aelys_int(code, 42);
}

#[test]
fn string_substr_negative_start() {
    let code = r#"
needs std.string
let s = string.substr("hello", -1, 3)
42
"#;
    assert_aelys_int(code, 42);
}

#[test]
fn string_substr_negative_len() {
    let code = r#"
needs std.string
let s = string.substr("hello", 0, -5)
42
"#;
    assert_aelys_int(code, 42);
}

#[test]
fn string_to_upper() {
    let code = r#"
needs std.string
let s = string.to_upper("hello")
42
"#;
    assert_aelys_int(code, 42);
}

#[test]
fn string_to_lower() {
    let code = r#"
needs std.string
let s = string.to_lower("HELLO")
42
"#;
    assert_aelys_int(code, 42);
}

#[test]
fn string_capitalize() {
    let code = r#"
needs std.string
let s = string.capitalize("hello")
42
"#;
    assert_aelys_int(code, 42);
}

#[test]
fn string_capitalize_empty() {
    let code = r#"
needs std.string
let s = string.capitalize("")
42
"#;
    assert_aelys_int(code, 42);
}

#[test]
fn string_contains_true() {
    let code = r#"
needs std.string
if string.contains("hello world", "wor") { 1 } else { 0 }
"#;
    assert_aelys_int(code, 1);
}

#[test]
fn string_contains_false() {
    let code = r#"
needs std.string
if string.contains("hello", "xyz") { 0 } else { 1 }
"#;
    assert_aelys_int(code, 1);
}

#[test]
fn string_starts_with_true() {
    let code = r#"
needs std.string
if string.starts_with("hello", "he") { 1 } else { 0 }
"#;
    assert_aelys_int(code, 1);
}

#[test]
fn string_starts_with_false() {
    let code = r#"
needs std.string
if string.starts_with("hello", "lo") { 0 } else { 1 }
"#;
    assert_aelys_int(code, 1);
}

#[test]
fn string_ends_with_true() {
    let code = r#"
needs std.string
if string.ends_with("hello", "lo") { 1 } else { 0 }
"#;
    assert_aelys_int(code, 1);
}

#[test]
fn string_ends_with_false() {
    let code = r#"
needs std.string
if string.ends_with("hello", "he") { 0 } else { 1 }
"#;
    assert_aelys_int(code, 1);
}

#[test]
fn string_find_exists() {
    let code = r#"
needs std.string
let pos = string.find("hello world", "wor")
if pos >= 0 { 1 } else { 0 }
"#;
    assert_aelys_int(code, 1);
}

#[test]
fn string_find_not_found() {
    let code = r#"
needs std.string
string.find("hello", "xyz")
"#;
    assert_aelys_int(code, -1);
}

#[test]
fn string_rfind_exists() {
    let code = r#"
needs std.string
let pos = string.rfind("hello hello", "hello")
if pos > 0 { 1 } else { 0 }
"#;
    assert_aelys_int(code, 1);
}

#[test]
fn string_rfind_not_found() {
    let code = r#"
needs std.string
string.rfind("hello", "xyz")
"#;
    assert_aelys_int(code, -1);
}

#[test]
fn string_count_occurrences() {
    let code = r#"
needs std.string
string.count("hello hello hello", "hello")
"#;
    assert_aelys_int(code, 3);
}

#[test]
fn string_count_zero() {
    let code = r#"
needs std.string
string.count("hello", "xyz")
"#;
    assert_aelys_int(code, 0);
}

#[test]
fn string_replace_all() {
    let code = r#"
needs std.string
let s = string.replace("hello hello", "hello", "hi")
42
"#;
    assert_aelys_int(code, 42);
}

#[test]
fn string_replace_first() {
    let code = r#"
needs std.string
let s = string.replace_first("hello hello", "hello", "hi")
42
"#;
    assert_aelys_int(code, 42);
}

#[test]
fn string_split_basic() {
    let code = r#"
needs std.string
let parts = string.split("a,b,c", ",")
42
"#;
    assert_aelys_int(code, 42);
}

#[test]
fn string_split_empty_separator() {
    let code = r#"
needs std.string
let parts = string.split("abc", "")
42
"#;
    assert_aelys_int(code, 42);
}

#[test]
fn string_join_basic() {
    let code = r#"
needs std.string
let parts = "a\nb\nc"
let s = string.join(parts, ",")
42
"#;
    assert_aelys_int(code, 42);
}

#[test]
fn string_repeat_positive() {
    let code = r#"
needs std.string
let s = string.repeat("ab", 3)
42
"#;
    assert_aelys_int(code, 42);
}

#[test]
fn string_repeat_zero() {
    let code = r#"
needs std.string
let s = string.repeat("abc", 0)
42
"#;
    assert_aelys_int(code, 42);
}

#[test]
fn string_repeat_negative() {
    let code = r#"
needs std.string
let s = string.repeat("abc", -5)
42
"#;
    assert_aelys_int(code, 42);
}

#[test]
fn string_reverse_basic() {
    let code = r#"
needs std.string
let s = string.reverse("abc")
42
"#;
    assert_aelys_int(code, 42);
}

#[test]
fn string_reverse_unicode() {
    let code = r#"
needs std.string
let s = string.reverse("héllo")
42
"#;
    assert_aelys_int(code, 42);
}

#[test]
fn string_concat_basic() {
    let code = r#"
needs std.string
let s = string.concat("hello", " world")
42
"#;
    assert_aelys_int(code, 42);
}

#[test]
fn string_trim_whitespace() {
    let code = r#"
needs std.string
let s = string.trim("  hello  ")
42
"#;
    assert_aelys_int(code, 42);
}

#[test]
fn string_trim_start() {
    let code = r#"
needs std.string
let s = string.trim_start("  hello")
42
"#;
    assert_aelys_int(code, 42);
}

#[test]
fn string_trim_end() {
    let code = r#"
needs std.string
let s = string.trim_end("hello  ")
42
"#;
    assert_aelys_int(code, 42);
}

#[test]
fn string_pad_left_basic() {
    let code = r#"
needs std.string
let s = string.pad_left("5", 3, "0")
42
"#;
    assert_aelys_int(code, 42);
}

#[test]
fn string_pad_left_already_wide() {
    let code = r#"
needs std.string
let s = string.pad_left("hello", 2, "x")
42
"#;
    assert_aelys_int(code, 42);
}

#[test]
fn string_pad_right_basic() {
    let code = r#"
needs std.string
let s = string.pad_right("hi", 5, ".")
42
"#;
    assert_aelys_int(code, 42);
}

#[test]
fn string_is_empty_true() {
    let code = r#"
needs std.string
if string.is_empty("") { 1 } else { 0 }
"#;
    assert_aelys_int(code, 1);
}

#[test]
fn string_is_empty_false() {
    let code = r#"
needs std.string
if string.is_empty("x") { 0 } else { 1 }
"#;
    assert_aelys_int(code, 1);
}

#[test]
fn string_is_whitespace_true() {
    let code = r#"
needs std.string
if string.is_whitespace("   ") { 1 } else { 0 }
"#;
    assert_aelys_int(code, 1);
}

#[test]
fn string_is_whitespace_false() {
    let code = r#"
needs std.string
if string.is_whitespace(" a ") { 0 } else { 1 }
"#;
    assert_aelys_int(code, 1);
}

#[test]
fn string_is_whitespace_empty() {
    let code = r#"
needs std.string
if string.is_whitespace("") { 0 } else { 1 }
"#;
    assert_aelys_int(code, 1);
}

#[test]
fn string_is_numeric_true() {
    let code = r#"
needs std.string
if string.is_numeric("12345") { 1 } else { 0 }
"#;
    assert_aelys_int(code, 1);
}

#[test]
fn string_is_numeric_false() {
    let code = r#"
needs std.string
if string.is_numeric("12a34") { 0 } else { 1 }
"#;
    assert_aelys_int(code, 1);
}

#[test]
fn string_is_alphabetic_true() {
    let code = r#"
needs std.string
if string.is_alphabetic("hello") { 1 } else { 0 }
"#;
    assert_aelys_int(code, 1);
}

#[test]
fn string_is_alphabetic_false() {
    let code = r#"
needs std.string
if string.is_alphabetic("hello123") { 0 } else { 1 }
"#;
    assert_aelys_int(code, 1);
}

#[test]
fn string_is_alphanumeric_true() {
    let code = r#"
needs std.string
if string.is_alphanumeric("hello123") { 1 } else { 0 }
"#;
    assert_aelys_int(code, 1);
}

#[test]
fn string_is_alphanumeric_false() {
    let code = r#"
needs std.string
if string.is_alphanumeric("hello-123") { 0 } else { 1 }
"#;
    assert_aelys_int(code, 1);
}

#[test]
fn string_lines_basic() {
    let code = r#"
needs std.string
let s = string.lines("a\nb\nc")
42
"#;
    assert_aelys_int(code, 42);
}

#[test]
fn string_line_count() {
    let code = r#"
needs std.string
string.line_count("a\nb\nc")
"#;
    assert_aelys_int(code, 3);
}

#[test]
fn string_line_count_empty() {
    let code = r#"
needs std.string
string.line_count("")
"#;
    assert_aelys_int(code, 0);
}

#[test]
fn string_bytes_basic() {
    let code = r#"
needs std.string
let b = string.bytes("AB")
42
"#;
    assert_aelys_int(code, 42);
}

#[test]
fn string_chars_basic() {
    let code = r#"
needs std.string
let c = string.chars("abc")
42
"#;
    assert_aelys_int(code, 42);
}

#[test]
fn string_unicode_length_mismatch() {
    let code = r#"
needs std.string
let byte_len = string.len("😀")
let char_len = string.char_len("😀")
if byte_len > char_len { 1 } else { 0 }
"#;
    assert_aelys_int(code, 1);
}

#[test]
fn string_emoji_reverse() {
    let code = r#"
needs std.string
let s = string.reverse("😀😁")
42
"#;
    assert_aelys_int(code, 42);
}

#[test]
fn string_rtl_text() {
    let code = r#"
needs std.string
let s = string.reverse("مرحبا")
42
"#;
    assert_aelys_int(code, 42);
}
