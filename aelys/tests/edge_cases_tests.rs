mod common;
use common::*;

// Unicode edge cases

#[test]
fn unicode_surrogate_pairs() {
    let code = r#"
needs std.string
let s = "𝕳𝖊𝖑𝖑𝖔"
string.char_len(s)
"#;
    assert_aelys_int(code, 5);
}

#[test]
fn unicode_rtl_text_handling() {
    let code = r#"
needs std.string
let arabic = "مرحبا بك"
let len = string.char_len(arabic)
if len > 0 { 1 } else { 0 }
"#;
    assert_aelys_int(code, 1);
}

#[test]
fn unicode_combining_characters() {
    let code = r#"
needs std.string
let s = "é"
string.len(s)
"#;
    // Should be 2 bytes for composed form
    let result = run_aelys(code);
    assert!(result.as_int().unwrap() > 0);
}

#[test]
fn unicode_zero_width_characters() {
    let code = r#"
needs std.string
let s = "a​b"
string.len(s)
"#;
    let result = run_aelys(code);
    assert!(result.as_int().unwrap() > 2);
}

#[test]
fn unicode_emoji_sequences() {
    let code = r#"
needs std.string
let emoji = "👨‍👩‍👧‍👦"
let byte_len = string.len(emoji)
let char_len = string.char_len(emoji)
if byte_len > char_len { 1 } else { 0 }
"#;
    assert_aelys_int(code, 1);
}

// String literal max length tests

#[test]
fn very_long_string_literal() {
    let long_str = "x".repeat(10000);
    let code = format!(r#"
needs std.string
let s = "{}"
string.len(s)
"#, long_str);
    assert_aelys_int(&code, 10000);
}

#[test]
fn extremely_long_string_concat() {
    let code = r#"
needs std.string
let mut s = ""
let mut i = 0
while i < 1000 {
    s = s + "x"
    i = i + 1
}
string.len(s)
"#;
    assert_aelys_int(code, 1000);
}

// Recursion at MAX_FRAMES

#[test]
fn recursion_near_max_frames() {
    let code = r#"
fn recurse(n) {
    if n <= 0 {
        return 1
    }
    return recurse(n - 1)
}
recurse(500)
"#;
    assert_aelys_int(code, 1);
}

#[test]
fn mutual_recursion_deep() {
    let code = r#"
fn even(n) {
    if n == 0 { return true }
    if n == 1 { return false }
    return odd(n - 1)
}
fn odd(n) {
    if n == 0 { return false }
    if n == 1 { return true }
    return even(n - 1)
}
if even(200) { 1 } else { 0 }
"#;
    assert_aelys_int(code, 1);
}

// GC during allocation

#[test]
fn gc_stress_many_allocations() {
    let code = r#"
let mut i = 0
while i < 10000 {
    let s = "test string number " + "more text"
    i = i + 1
}
42
"#;
    assert_aelys_int(code, 42);
}

#[test]
fn gc_with_circular_references() {
    // Aelys doesn't have mutable data structures that can create cycles
    // But we can stress GC with many allocations
    let code = r#"
fn make_strings(n) {
    if n <= 0 { return 0 }
    let s = "string " + "concat"
    return make_strings(n - 1) + 1
}
make_strings(1000)
"#;
    assert_aelys_int(code, 1000);
}

// Type system edge cases

#[test]
fn deeply_nested_function_types() {
    let code = r#"
fn f1() { return fn() { return fn() { return fn() { return 42 } } } }
let result = f1()()()()
result
"#;
    assert_aelys_int(code, 42);
}

#[test]
fn closure_capturing_many_variables() {
    let code = r#"
fn make_closure() {
    let a = 1
    let b = 2
    let c = 3
    let d = 4
    let e = 5
    let f = 6
    let g = 7
    let h = 8
    return fn() { return a + b + c + d + e + f + g + h }
}
let closure = make_closure()
closure()
"#;
    assert_aelys_int(code, 36);
}

// Integer boundary values

#[test]
fn int_max_value() {
    let code = r#"
let max = 140737488355327
if max > 0 { 1 } else { 0 }
"#;
    assert_aelys_int(code, 1);
}

#[test]
fn int_overflow_handling() {
    let code = r#"
needs std.math
let large = 140737488355327
let result = math.pow(large, 2)
if result > 0.0 { 1 } else { 0 }
"#;
    assert_aelys_int(code, 1);
}

// Float special values

#[test]
fn float_infinity_operations() {
    let code = r#"
needs std.math
let inf = math.INF
let result = inf + 1.0
if math.is_inf(result) { 1 } else { 0 }
"#;
    assert_aelys_int(code, 1);
}

#[test]
fn float_nan_propagation() {
    let code = r#"
needs std.math
let nan = math.sqrt(-1.0)
let result = nan + 5.0
if math.is_nan(result) { 1 } else { 0 }
"#;
    assert_aelys_int(code, 1);
}

#[test]
fn division_by_very_small() {
    let code = r#"
let result = 1.0 / 0.0000000001
if result > 1000000000.0 { 1 } else { 0 }
"#;
    assert_aelys_int(code, 1);
}

// Empty collections and edge cases

#[test]
fn empty_string_operations() {
    let code = r#"
needs std.string
let s = ""
let rev = string.reverse(s)
let upper = string.to_upper(s)
if string.is_empty(s) { 1 } else { 0 }
"#;
    assert_aelys_int(code, 1);
}

#[test]
fn string_split_on_empty() {
    let code = r#"
needs std.string
let parts = string.split("", ",")
42
"#;
    assert_aelys_int(code, 42);
}

// Boundary conditions for loops

#[test]
fn loop_zero_iterations() {
    let code = r#"
let mut sum = 0
let mut i = 0
while i < 0 {
    sum = sum + i
    i = i + 1
}
sum
"#;
    assert_aelys_int(code, 0);
}

#[test]
fn loop_one_iteration() {
    let code = r#"
let mut sum = 0
let mut i = 0
while i < 1 {
    sum = sum + i
    i = i + 1
}
sum
"#;
    assert_aelys_int(code, 0);
}

#[test]
fn nested_loops_deep() {
    let code = r#"
let mut count = 0
let mut i = 0
while i < 10 {
    let mut j = 0
    while j < 10 {
        let mut k = 0
        while k < 10 {
            count = count + 1
            k = k + 1
        }
        j = j + 1
    }
    i = i + 1
}
count
"#;
    assert_aelys_int(code, 1000);
}

// Variable shadowing edge cases

#[test]
fn extreme_variable_shadowing() {
    let code = r#"
let x = 1
{
    let x = 2
    {
        let x = 3
        {
            let x = 4
            {
                let x = 5
            }
        }
    }
}
x
"#;
    assert_aelys_int(code, 1);
}

// Function parameter edge cases

#[test]
fn function_with_zero_params() {
    let code = r#"
fn f() { return 42 }
f()
"#;
    assert_aelys_int(code, 42);
}

#[test]
fn function_with_many_params() {
    let code = r#"
fn add10(a, b, c, d, e, f, g, h, i, j) {
    return a + b + c + d + e + f + g + h + i + j
}
add10(1, 2, 3, 4, 5, 6, 7, 8, 9, 10)
"#;
    assert_aelys_int(code, 55);
}

// Whitespace and formatting edge cases

#[test]
fn code_with_excessive_whitespace() {
    let code = r#"
let     x     =     42


x
"#;
    assert_aelys_int(code, 42);
}

#[test]
fn code_with_tabs() {
    let code = "let\tx\t=\t42\nx";
    assert_aelys_int(code, 42);
}

// Math edge cases

#[test]
fn sqrt_zero() {
    let code = r#"
needs std.math
let r = math.sqrt(0.0)
if r == 0.0 { 1 } else { 0 }
"#;
    assert_aelys_int(code, 1);
}

#[test]
fn log_one() {
    let code = r#"
needs std.math
let r = math.log(1.0)
if r > -0.01 and r < 0.01 { 1 } else { 0 }
"#;
    assert_aelys_int(code, 1);
}

#[test]
fn pow_zero_exponent() {
    let code = r#"
needs std.math
let r = math.pow(123.456, 0.0)
if r > 0.99 and r < 1.01 { 1 } else { 0 }
"#;
    assert_aelys_int(code, 1);
}

#[test]
fn trig_large_angles() {
    let code = r#"
needs std.math
let r = math.sin(1000000.0)
if r >= -1.0 and r <= 1.0 { 1 } else { 0 }
"#;
    assert_aelys_int(code, 1);
}
