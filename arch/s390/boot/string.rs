//! Automatically rewritten from C to Rust
//! Source: arch/s390/boot/string.c
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


// SPDX-License-Identifier: GPL-2.0
pub const IN_BOOT_STRING_C: c_int = 1;

//
// Duplicate some functions from the common lib/string.c
// instead of fully including it.
//
#[no_mangle]
pub unsafe extern "C" fn strncmp(cs: *const c_char, ct: *const c_char, count: usize) -> c_int {
    int strncmp(const char *cs, const char *ct, size_t count)
    {
    unsigned char c1, c2;
    while (count) {
    c1 = *cs++;
    c2 = *ct++;
    if (c1 != c2)
    return c1 < c2 ? -1 : 1;
    if (!c1)
    break;
    count--;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn sized_strscpy(dst: *mut c_char, src: *const c_char, count: usize) -> isize {
    ssize_t sized_strscpy(char *dst, const char *src, size_t count)
    {
    size_t len;
    if (count == 0)
    return -E2BIG;
    len = strnlen(src, count - 1);
    memcpy(dst, src, len);
    dst[len] = '\0';
    return src[len] ? -E2BIG : len;
    }
    void *memset64(uint64_t *s, uint64_t v, size_t count)
    {
    return __memset64(s, v, count * sizeof(v));
    }
    char *skip_spaces(const char *str)
    {
    while (isspace(*str))
    ++str;
    return (char *)str;
    }
    char *strim(char *s)
    {
    size_t size;
    char *end;
    size = strlen(s);
    if (!size)
    return s;
    end = s + size - 1;
    while (end >= s && isspace(*end))
    end--;
// (end + 1) = '\0';
    return skip_spaces(s);
    }
// Works only for digits and letters, but small and fast

#[no_mangle]
unsafe extern "C" fn simple_guess_base(cp: *const c_char) -> c_uint {
    static unsigned int simple_guess_base(const char *cp)
    {
    if (cp[0] == '0') {
    if (TOLOWER(cp[1]) == 'x' && isxdigit(cp[2]))
    return 16;
    else
    return 8;
    } else {
    return 10;
    }
    }
//
// simple_strtoull - convert a string to an unsigned long long
// @cp: The start of the string
// @endp: A pointer to the end of the parsed string will be placed here
// @base: The number base to use
//
    unsigned long long simple_strtoull(const char *cp, char **endp,
    unsigned int base)
    {
    let mut result: c_ulonglong = 0;
    if (!base)
    base = simple_guess_base(cp);
    if (base == 16 && cp[0] == '0' && TOLOWER(cp[1]) == 'x')
    cp += 2;
    while (isxdigit(*cp)) {
    unsigned int value;
    value = isdigit(*cp) ? *cp - '0' : TOLOWER(*cp) - 'a' + 10;
    if (value >= base)
    break;
    result = result * base + value;
    cp++;
    }
    if (endp)
// endp = (char *)cp;
    return result;
    }
#[no_mangle]
pub unsafe extern "C" fn simple_strtol(cp: *const c_char, endp: *mut c_char, base: c_uint) -> c_long {
    long simple_strtol(const char *cp, char **endp, unsigned int base)
    {
    if (*cp == '-')
    return -simple_strtoull(cp + 1, endp, base);
    return simple_strtoull(cp, endp, base);
    }
#[no_mangle]
pub unsafe extern "C" fn kstrtobool(s: *const c_char, res: *mut bool) -> c_int {
    int kstrtobool(const char *s, bool *res)
    {
    if (!s)
    return -EINVAL;
    switch (s[0]) {
    case 'y':
    case 'Y':
    case '1':
// res = true;
    return 0;
    case 'n':
    case 'N':
    case '0':
// res = false;
    return 0;
    case 'o':
    case 'O':
    switch (s[1]) {
    case 'n':
    case 'N':
// res = true;
    return 0;
    case 'f':
    case 'F':
// res = false;
    return 0;
    default:
    break;
    }
    default:
    break;
    }
    return -EINVAL;
    }
