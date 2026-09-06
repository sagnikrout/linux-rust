//! Automatically rewritten from C to Rust
//! Source: lib/parser.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// lib/parser.c - simple parser for mount, etc. options.
//

//
// max size needed by different bases to express U64
// HEX: "0xFFFFFFFFFFFFFFFF" --> 18
// DEC: "18446744073709551615" --> 20
// OCT: "01777777777777777777777" --> 23
// pick the max one to define NUMBER_BUF_LEN
//
pub const NUMBER_BUF_LEN: c_int = 24;
//
// match_one - Determines if a string matches a simple pattern
// @s: the string to examine for presence of the pattern
// @p: the string containing the pattern
// @args: array of %MAX_OPT_ARGS &substring_t elements. Used to return match
// locations.
//
// Description: Determines if the pattern @p is present in string @s. Can only
// match extremely simple token=arg style patterns. If the pattern is found,
// the location(s) of the arguments will be returned in the @args array.
//
#[no_mangle]
unsafe extern "C" fn match_one(s: *mut c_char, p: *const c_char, args[]: substring_t) -> c_int {
    static int match_one(char *s, const char *p, substring_t args[])
    {
    char *meta;
    let mut argc: c_int = 0;
    if (!p)
    return 1;
    while(1) {
    let mut len: c_int = -1;
    meta = strchr(p, '%');
    if (!meta)
    return strcmp(p, s) == 0;
    if (strncmp(p, s, meta-p))
    return 0;
    s += meta - p;
    p = meta + 1;
    if (isdigit(*p))
    len = simple_strtoul(p, (char **) &p, 10);
#[no_mangle]
pub unsafe extern "C" fn if('%': *mut *mut p ==) -> else {
    if (*s++ != '%')
    return 0;
    p++;
    continue;
    }
    if (argc >= MAX_OPT_ARGS)
    return 0;
    args[argc].from = s;
    switch (*p++) {
    case 's': {
    let mut str_len: usize = strlen(s);
    if (str_len == 0)
    return 0;
    if (len == -1 || len > str_len)
    len = str_len;
    args[argc].to = s + len;
    break;
    }
    case 'd':
    simple_strtol(s, &args[argc].to, 0);
    goto num;
    case 'u':
    simple_strtoul(s, &args[argc].to, 0);
    goto num;
    case 'o':
    simple_strtoul(s, &args[argc].to, 8);
    goto num;
    case 'x':
    simple_strtoul(s, &args[argc].to, 16);
    num:
    if (args[argc].to == args[argc].from)
    return 0;
    break;
    default:
    return 0;
    }
    s = args[argc].to;
    argc++;
    }
    }
//
// match_token - Find a token (and optional args) in a string
// @s: the string to examine for token/argument pairs
// @table: match_table_t describing the set of allowed option tokens and the
// arguments that may be associated with them. Must be terminated with a
// &struct match_token whose pattern is set to the NULL pointer.
// @args: array of %MAX_OPT_ARGS &substring_t elements. Used to return match
// locations.
//
// Description: Detects which if any of a set of token strings has been passed
// to it. Tokens can include up to %MAX_OPT_ARGS instances of basic c-style
// format identifiers which will be taken into account when matching the
// tokens, and whose locations will be returned in the @args array.
//
#[no_mangle]
pub unsafe extern "C" fn match_token(s: *mut c_char, table: match_table_t, args[]: substring_t) -> c_int {
    int match_token(char *s, const match_table_t table, substring_t args[])
    {
    const struct match_token *p;
    for (p = table; !match_one(s, p.pattern, args) ; p++)
    ;
    return p.token;
    }
    EXPORT_SYMBOL(match_token);
//
// match_number - scan a number in the given base from a substring_t
// @s: substring to be scanned
// @result: resulting integer on success
// @base: base to use when converting string
//
// Description: Given a &substring_t and a base, attempts to parse the substring
// as a number in that base.
//
// Return: On success, sets @result to the integer represented by the
// string and returns 0. Returns -EINVAL or -ERANGE on failure.
//
#[no_mangle]
unsafe extern "C" fn match_number(s: *mut substring_t, result: *mut c_int, base: c_int) -> c_int {
    static int match_number(substring_t *s, int *result, int base)
    {
    char *endp;
    char buf[NUMBER_BUF_LEN];
    int ret;
    long val;
    if (match_strlcpy(buf, s, NUMBER_BUF_LEN) >= NUMBER_BUF_LEN)
    return -ERANGE;
    ret = 0;
    val = simple_strtol(buf, &endp, base);
    if (endp == buf)
    ret = -EINVAL;
#[no_mangle]
pub unsafe extern "C" fn if((long)INT_MAX: val < (long)INT_MIN || val >) -> else {
    else if (val < (long)INT_MIN || val > (long)INT_MAX)
    ret = -ERANGE;
    else
// result = (int) val;
    return ret;
    }
//
// match_u64int - scan a number in the given base from a substring_t
// @s: substring to be scanned
// @result: resulting u64 on success
// @base: base to use when converting string
//
// Description: Given a &substring_t and a base, attempts to parse the substring
// as a number in that base.
//
// Return: On success, sets @result to the integer represented by the
// string and returns 0. Returns -EINVAL or -ERANGE on failure.
//
#[no_mangle]
unsafe extern "C" fn match_u64int(s: *mut substring_t, result: *mut u64, base: c_int) -> c_int {
    static int match_u64int(substring_t *s, u64 *result, int base)
    {
    char buf[NUMBER_BUF_LEN];
    int ret;
    u64 val;
    if (match_strlcpy(buf, s, NUMBER_BUF_LEN) >= NUMBER_BUF_LEN)
    return -ERANGE;
    ret = kstrtoull(buf, base, &val);
    if (!ret)
// result = val;
    return ret;
    }
//
// match_int - scan a decimal representation of an integer from a substring_t
// @s: substring_t to be scanned
// @result: resulting integer on success
//
// Description: Attempts to parse the &substring_t @s as a decimal integer.
//
// Return: On success, sets @result to the integer represented by the string
// and returns 0. Returns -EINVAL or -ERANGE on failure.
//
#[no_mangle]
pub unsafe extern "C" fn match_int(s: *mut substring_t, result: *mut c_int) -> c_int {
    int match_int(substring_t *s, int *result)
    {
    return match_number(s, result, 0);
    }
    EXPORT_SYMBOL(match_int);
//
// match_uint - scan a decimal representation of an integer from a substring_t
// @s: substring_t to be scanned
// @result: resulting integer on success
//
// Description: Attempts to parse the &substring_t @s as a decimal integer.
//
// Return: On success, sets @result to the integer represented by the string
// and returns 0. Returns -EINVAL or -ERANGE on failure.
//
#[no_mangle]
pub unsafe extern "C" fn match_uint(s: *mut substring_t, result: *mut c_uint) -> c_int {
    int match_uint(substring_t *s, unsigned int *result)
    {
    char buf[NUMBER_BUF_LEN];
    if (match_strlcpy(buf, s, NUMBER_BUF_LEN) >= NUMBER_BUF_LEN)
    return -ERANGE;
    return kstrtouint(buf, 10, result);
    }
    EXPORT_SYMBOL(match_uint);
//
// match_u64 - scan a decimal representation of a u64 from
// a substring_t
// @s: substring_t to be scanned
// @result: resulting unsigned long long on success
//
// Description: Attempts to parse the &substring_t @s as a long decimal
// integer.
//
// Return: On success, sets @result to the integer represented by the string
// and returns 0. Returns -EINVAL or -ERANGE on failure.
//
#[no_mangle]
pub unsafe extern "C" fn match_u64(s: *mut substring_t, result: *mut u64) -> c_int {
    int match_u64(substring_t *s, u64 *result)
    {
    return match_u64int(s, result, 0);
    }
    EXPORT_SYMBOL(match_u64);
//
// match_octal - scan an octal representation of an integer from a substring_t
// @s: substring_t to be scanned
// @result: resulting integer on success
//
// Description: Attempts to parse the &substring_t @s as an octal integer.
//
// Return: On success, sets @result to the integer represented by the string
// and returns 0. Returns -EINVAL or -ERANGE on failure.
//
#[no_mangle]
pub unsafe extern "C" fn match_octal(s: *mut substring_t, result: *mut c_int) -> c_int {
    int match_octal(substring_t *s, int *result)
    {
    return match_number(s, result, 8);
    }
    EXPORT_SYMBOL(match_octal);
//
// match_hex - scan a hex representation of an integer from a substring_t
// @s: substring_t to be scanned
// @result: resulting integer on success
//
// Description: Attempts to parse the &substring_t @s as a hexadecimal integer.
//
// Return: On success, sets @result to the integer represented by the string
// and returns 0. Returns -EINVAL or -ERANGE on failure.
//
#[no_mangle]
pub unsafe extern "C" fn match_hex(s: *mut substring_t, result: *mut c_int) -> c_int {
    int match_hex(substring_t *s, int *result)
    {
    return match_number(s, result, 16);
    }
    EXPORT_SYMBOL(match_hex);
//
// match_wildcard - parse if a string matches given wildcard pattern
// @pattern: wildcard pattern
// @str: the string to be parsed
//
// Description: Parse the string @str to check if matches wildcard
// pattern @pattern. The pattern may contain two types of wildcards:
//
// * '*' - matches zero or more characters
// * '?' - matches one character
//
// Return: If the @str matches the @pattern, return true, else return false.
//
#[no_mangle]
pub unsafe extern "C" fn match_wildcard(pattern: *const c_char, str: *const c_char) -> bool {
    bool match_wildcard(const char *pattern, const char *str)
    {
    const char *s = str;
    const char *p = pattern;
    let mut star: bool = false;
    while (*s) {
    switch (*p) {
    case '?':
    s++;
    p++;
    break;
    case '*':
    star = true;
    str = s;
    if (!*++p)
    return true;
    pattern = p;
    break;
    default:
    if (*s == *p) {
    s++;
    p++;
    } else {
    if (!star)
    return false;
    str++;
    s = str;
    p = pattern;
    }
    break;
    }
    }
    while (*p == '*')
    ++p;
    return !*p;
    }
    EXPORT_SYMBOL(match_wildcard);
//
// match_strlcpy - Copy the characters from a substring_t to a sized buffer
// @dest: where to copy to
// @src: &substring_t to copy
// @size: size of destination buffer
//
// Description: Copy the characters in &substring_t @src to the
// c-style string @dest.  Copy no more than @size - 1 characters, plus
// the terminating NUL.
//
// Return: length of @src.
//
#[no_mangle]
pub unsafe extern "C" fn match_strlcpy(dest: *mut c_char, src: *const substring_t, size: usize) -> usize {
    size_t match_strlcpy(char *dest, const substring_t *src, size_t size)
    {
    let mut ret: usize = src.to - src.from;
    if (size) {
    let mut len: usize = ret >= size ? size - 1 : ret;
    memcpy(dest, src.from, len);
    dest[len] = '\0';
    }
    return ret;
    }
    EXPORT_SYMBOL(match_strlcpy);
//
// match_strdup - allocate a new string with the contents of a substring_t
// @s: &substring_t to copy
//
// Description: Allocates and returns a string filled with the contents of
// the &substring_t @s. The caller is responsible for freeing the returned
// string with kfree().
//
// Return: the address of the newly allocated NUL-terminated string or
// %NULL on error.
//
    char *match_strdup(const substring_t *s)
    {
    return kmemdup_nul(s.from, s.to - s.from, GFP_KERNEL);
    }
    EXPORT_SYMBOL(match_strdup);
