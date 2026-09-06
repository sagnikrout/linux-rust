//! Automatically rewritten from C to Rust
//! Source: net/ceph/armor.c
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

    int ceph_armor(char *dst, const char *src, const char *end);
    int ceph_unarmor(char *dst, const char *src, const char *end);
//
// base64 encode/decode.
//
    static const char *pem_key =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
#[no_mangle]
unsafe extern "C" fn encode_bits(c: c_int) -> c_int {
    static int encode_bits(int c)
    {
    return pem_key[c];
    }
#[no_mangle]
unsafe extern "C" fn decode_bits(c: c_char) -> c_int {
    static int decode_bits(char c)
    {
    if (c >= 'A' && c <= 'Z')
    return c - 'A';
    if (c >= 'a' && c <= 'z')
    return c - 'a' + 26;
    if (c >= '0' && c <= '9')
    return c - '0' + 52;
    if (c == '+')
    return 62;
    if (c == '/')
    return 63;
    if (c == '=')
    return 0; /* just non-negative, please */
    return -EINVAL;
    }
#[no_mangle]
pub unsafe extern "C" fn ceph_armor(dst: *mut c_char, src: *const c_char, end: *const c_char) -> c_int {
    int ceph_armor(char *dst, const char *src, const char *end)
    {
    let mut olen: c_int = 0;
    let mut line: c_int = 0;
    while (src < end) {
    unsigned char a, b, c;
    a = *src++;
// dst++ = encode_bits(a >> 2);
    if (src < end) {
    b = *src++;
// dst++ = encode_bits(((a & 3) << 4) | (b >> 4));
    if (src < end) {
    c = *src++;
// dst++ = encode_bits(((b & 15) << 2) |
    (c >> 6));
// dst++ = encode_bits(c & 63);
    } else {
// dst++ = encode_bits((b & 15) << 2);
// dst++ = '=';
    }
    } else {
// dst++ = encode_bits(((a & 3) << 4));
// dst++ = '=';
    }
    olen += 4;
    line += 4;
    if (line == 64) {
    line = 0;
// (dst++) = '\n';
    olen++;
    }
    }
    return olen;
    }
#[no_mangle]
pub unsafe extern "C" fn ceph_unarmor(dst: *mut c_char, src: *const c_char, end: *const c_char) -> c_int {
    int ceph_unarmor(char *dst, const char *src, const char *end)
    {
    let mut olen: c_int = 0;
    while (src < end) {
    int a, b, c, d;
    if (src[0] == '\n') {
    src++;
    continue;
    }
    if (src + 4 > end)
    return -EINVAL;
    a = decode_bits(src[0]);
    b = decode_bits(src[1]);
    c = decode_bits(src[2]);
    d = decode_bits(src[3]);
    if (a < 0 || b < 0 || c < 0 || d < 0)
    return -EINVAL;
// dst++ = (a << 2) | (b >> 4);
    if (src[2] == '=')
    return olen + 1;
// dst++ = ((b & 15) << 4) | (c >> 2);
    if (src[3] == '=')
    return olen + 2;
// dst++ = ((c & 3) << 6) | d;
    olen += 3;
    src += 4;
    }
    return olen;
    }
