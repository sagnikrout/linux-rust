//! Automatically rewritten from C to Rust
//! Source: lib/glob.c
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


// SPDX-License-Identifier: (GPL-2.0 OR MIT)

//
// The only reason this code can be compiled as a module is because the
// ATA code that depends on it can be as well.  In practice, they're
// both usually compiled in and the module overhead goes away.
//
    MODULE_DESCRIPTION("glob(7) matching");
    MODULE_LICENSE("Dual MIT/GPL");
    static bool __pure glob_match_str(char const *pat, char const *str,
    char const *str_end);
//
// glob_match - Shell-style pattern matching, like !fnmatch(pat, str, 0)
// @pat: Shell-style pattern to match, e.g. "*.[ch]".
// @str: String to match.  The pattern must match the entire string.
//
// Perform shell-style glob matching, returning true (1) if the match
// succeeds, or false (0) if it fails.  Equivalent to !fnmatch(@pat, @str, 0).
//
// Pattern metacharacters are ?, *, [ and \.
// (And, inside character classes, !, - and ].)
//
// This is a small and simple implementation intended for device denylists
// where a string is matched against a number of patterns.  Thus, it
// does not preprocess the patterns.  It is non-recursive, and run-time
// is at most quadratic: strlen(@str)*strlen(@pat).
//
// An example of the worst case is glob_match("*aaaaa", "aaaaaaaaaa");
// it takes 6 passes over the pattern before matching the string.
//
// Like !fnmatch(@pat, @str, 0) and unlike the shell, this does NOT
// treat / or leading . specially; it isn't actually used for pathnames.
//
// Note that according to glob(7) (and unlike bash), character classes
// are complemented by a leading !; this does not support the regex-style
// [^a-z] syntax.
//
// An opening bracket without a matching close is matched literally.
//
#[no_mangle]
pub unsafe extern "C" fn glob_match(pat: *const c_char, str: *const c_char) -> bool __pure {
    bool __pure glob_match(char const *pat, char const *str)
    {
    return glob_match_str(pat, str, core::ptr::null_mut());
    }
    EXPORT_SYMBOL(glob_match);
//
// glob_match_len - glob match against a length-bounded string
// @pat: Shell-style pattern to match.
// @str: String to match.  Need not be NUL-terminated.
// @len: Number of bytes of @str that may be read.
//
// Like glob_match(), but @str is only read up to @len bytes, so it can be
// used on buffers that are not NUL-terminated (e.g. trace event fields).
// A NUL byte within @len still terminates the string.
//
#[no_mangle]
pub unsafe extern "C" fn glob_match_len(pat: *const c_char, str: *const c_char, len: usize) -> bool __pure {
    bool __pure glob_match_len(char const *pat, char const *str, size_t len)
    {
    return glob_match_str(pat, str, str + len);
    }
    EXPORT_SYMBOL(glob_match_len);
    static bool __pure glob_match_str(char const *pat, char const *str,
    char const *str_end)
    {
//
// Backtrack to previous * on mismatch and retry starting one
// character later in the string.  Because * matches all characters
// (no exception for /), it can be easily proved that there's
// never a need to backtrack multiple levels.
//
    char const *back_pat = core::ptr::null_mut(), *back_str = core::ptr::null_mut();
//
// Loop over each token (character or class) in pat, matching
// it against the remaining unmatched tail of str.  Return false
// on mismatch, or true after matching the trailing nul bytes.
//
    for (;;) {
    let mut c: c_uchar = (str_end && str >= str_end) ? '\0' : *str;
    let mut d: c_uchar = *pat++;
    str++;
    switch (d) {
    case '?':	/* Wildcard: anything but nul */
    if (c == '\0')
    return false;
    break;
    case '*':	/* Any-length wildcard */
    if (*pat == '\0')	/* Optimize trailing * case */
    return true;
    back_pat = pat;
    back_str = --str;	/* Allow zero-length match */
    break;
    case '[': {	/* Character class */
    if (c == '\0')	/* No possible match */
    return false;
    let mut match: bool = false, inverted = (*pat == '!');
    char const *class = inverted ? pat + 1 : pat;
    let mut a: c_uchar = *class++;
//
// Iterate over each span in the character class.
// A span is either a single character a, or a
// range a-b.  The first span may begin with ']'.
//
    do {
    let mut b: c_uchar = a;
    if (a == '\0')	/* Malformed */
    goto literal;
    if (class[0] == '-' && class[1] != ']') {
    b = class[1];
    if (b == '\0')
    goto literal;
    class += 2;
// Any special action if a > b?
    }
    if (a <= c && c <= b)
    match = true;
    } while ((a = *class++) != ']');
    if (match == inverted)
    goto backtrack;
    pat = class;
    }
    break;
    case '\\':
    d = *pat++;
    fallthrough;
    default:	/* Literal character */
    literal:
    if (c == d) {
    if (d == '\0')
    return true;
    break;
    }
    backtrack:
    if (c == '\0' || !back_pat)
    return false;	/* No point continuing */
// Try again from last *, one character later in str.
    pat = back_pat;
    str = ++back_str;
    break;
    }
    }
    }
