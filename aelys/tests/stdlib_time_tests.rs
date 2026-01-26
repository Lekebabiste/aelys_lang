mod common;
use common::*;

#[test]
fn time_now_returns_reasonable_value() {
    let code = r#"
needs std.time
let t = time.now()
if t > 1600000000.0 and t < 2000000000.0 { 1 } else { 0 }
"#;
    assert_aelys_int(code, 1);
}

#[test]
fn time_now_ms_positive() {
    let code = r#"
needs std.time
let t = time.now_ms()
if t > 0 { 1 } else { 0 }
"#;
    assert_aelys_int(code, 1);
}

#[test]
fn time_now_us_greater_than_ms() {
    let code = r#"
needs std.time
let ms = time.now_ms()
let us = time.now_us()
if us >= ms { 1 } else { 0 }
"#;
    assert_aelys_int(code, 1);
}

#[test]
fn timer_and_elapsed() {
    let code = r#"
needs std.time
let t = time.timer()
let e = time.elapsed(t)
if e >= 0.0 { 1 } else { 0 }
"#;
    assert_aelys_int(code, 1);
}

#[test]
fn timer_elapsed_ms() {
    let code = r#"
needs std.time
let t = time.timer()
time.sleep(10)
let e = time.elapsed_ms(t)
if e >= 5 { 1 } else { 0 }
"#;
    assert_aelys_int(code, 1);
}

#[test]
fn timer_elapsed_us() {
    let code = r#"
needs std.time
let t = time.timer()
let e = time.elapsed_us(t)
if e >= 0 { 1 } else { 0 }
"#;
    assert_aelys_int(code, 1);
}

#[test]
fn timer_reset() {
    let code = r#"
needs std.time
let t = time.timer()
time.sleep(20)
time.reset(t)
let e = time.elapsed_ms(t)
if e < 15 { 1 } else { 0 }
"#;
    assert_aelys_int(code, 1);
}

#[test]
fn timer_invalid_handle() {
    let code = r#"
needs std.time
time.elapsed(999)
"#;
    assert_aelys_error_contains(code, "invalid");
}

#[test]
fn reset_invalid_handle() {
    let code = r#"
needs std.time
time.reset(777)
"#;
    assert_aelys_error_contains(code, "invalid");
}

#[test]
fn sleep_zero_ms() {
    let code = r#"
needs std.time
time.sleep(0)
42
"#;
    assert_aelys_int(code, 42);
}

#[test]
fn sleep_negative_ignored() {
    let code = r#"
needs std.time
time.sleep(-100)
42
"#;
    assert_aelys_int(code, 42);
}

#[test]
fn sleep_us_works() {
    let code = r#"
needs std.time
time.sleep_us(1000)
1
"#;
    assert_aelys_int(code, 1);
}

#[test]
fn year_is_current() {
    let code = r#"
needs std.time
let y = time.year()
if y >= 2024 and y <= 2030 { 1 } else { 0 }
"#;
    assert_aelys_int(code, 1);
}

#[test]
fn month_in_range() {
    let code = r#"
needs std.time
let m = time.month()
if m >= 1 and m <= 12 { 1 } else { 0 }
"#;
    assert_aelys_int(code, 1);
}

#[test]
fn day_in_range() {
    let code = r#"
needs std.time
let d = time.day()
if d >= 1 and d <= 31 { 1 } else { 0 }
"#;
    assert_aelys_int(code, 1);
}

#[test]
fn hour_in_range() {
    let code = r#"
needs std.time
let h = time.hour()
if h >= 0 and h < 24 { 1 } else { 0 }
"#;
    assert_aelys_int(code, 1);
}

#[test]
fn minute_in_range() {
    let code = r#"
needs std.time
let m = time.minute()
if m >= 0 and m < 60 { 1 } else { 0 }
"#;
    assert_aelys_int(code, 1);
}

#[test]
fn second_in_range() {
    let code = r#"
needs std.time
let s = time.second()
if s >= 0 and s < 60 { 1 } else { 0 }
"#;
    assert_aelys_int(code, 1);
}

#[test]
fn weekday_in_range() {
    let code = r#"
needs std.time
let w = time.weekday()
if w >= 0 and w < 7 { 1 } else { 0 }
"#;
    assert_aelys_int(code, 1);
}

#[test]
fn yearday_in_range() {
    let code = r#"
needs std.time
let yd = time.yearday()
if yd >= 1 and yd <= 366 { 1 } else { 0 }
"#;
    assert_aelys_int(code, 1);
}

#[test]
fn format_year() {
    let code = r#"
needs std.time
let s = time.format("%Y")
42
"#;
    assert_aelys_int(code, 42);
}

#[test]
fn format_complex() {
    let code = r#"
needs std.time
let s = time.format("%Y-%m-%d %H:%M:%S")
42
"#;
    assert_aelys_int(code, 42);
}

#[test]
fn iso_format() {
    let code = r#"
needs std.time
let s = time.iso()
42
"#;
    assert_aelys_int(code, 42);
}

#[test]
fn date_format() {
    let code = r#"
needs std.time
let s = time.date()
42
"#;
    assert_aelys_int(code, 42);
}

#[test]
fn time_str_format() {
    let code = r#"
needs std.time
let s = time.time_str()
42
"#;
    assert_aelys_int(code, 42);
}

#[test]
fn elapsed_ms_invalid_handle() {
    let code = r#"
needs std.time
time.elapsed_ms(12345)
"#;
    assert_aelys_error_contains(code, "invalid");
}

#[test]
fn elapsed_us_invalid_handle() {
    let code = r#"
needs std.time
time.elapsed_us(54321)
"#;
    let err = run_aelys_err(code);
    assert!(err.contains("invalid") || err.contains("handle"));
}

#[test]
fn sleep_precise() {
    let code = r#"
needs std.time
let t = time.timer()
time.sleep(50)
let e = time.elapsed_ms(t)
if e >= 45 and e < 200 { 1 } else { 0 }
"#;
    assert_aelys_int(code, 1);
}

#[test]
fn multiple_timers() {
    let code = r#"
needs std.time
let t1 = time.timer()
time.sleep(10)
let t2 = time.timer()
let e1 = time.elapsed_ms(t1)
let e2 = time.elapsed_ms(t2)
if e1 > e2 { 1 } else { 0 }
"#;
    assert_aelys_int(code, 1);
}

#[test]
fn format_percent_escape() {
    let code = r#"
needs std.time
let s = time.format("100%%")
42
"#;
    assert_aelys_int(code, 42);
}

#[test]
fn format_weekday_name() {
    let code = r#"
needs std.time
let s = time.format("%a")
42
"#;
    assert_aelys_int(code, 42);
}

#[test]
fn format_month_name() {
    let code = r#"
needs std.time
let s = time.format("%b")
42
"#;
    assert_aelys_int(code, 42);
}
