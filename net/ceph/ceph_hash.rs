//! Automatically rewritten from C to Rust
//! Source: net/ceph/ceph_hash.c
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


//
// Robert Jenkin's hash function.
// https://burtleburtle.net/bob/hash/evahash.html
// This is in the public domain.
//

    do {							\
    a = a - b;  a = a - c;  a = a ^ (c >> 13);	\
    b = b - c;  b = b - a;  b = b ^ (a << 8);	\
    c = c - a;  c = c - b;  c = c ^ (b >> 13);	\
    a = a - b;  a = a - c;  a = a ^ (c >> 12);	\
    b = b - c;  b = b - a;  b = b ^ (a << 16);	\
    c = c - a;  c = c - b;  c = c ^ (b >> 5);	\
    a = a - b;  a = a - c;  a = a ^ (c >> 3);	\
    b = b - c;  b = b - a;  b = b ^ (a << 10);	\
    c = c - a;  c = c - b;  c = c ^ (b >> 15);	\
    } while (0)
#[no_mangle]
pub unsafe extern "C" fn ceph_str_hash_rjenkins(str: *const c_char, length: c_uint) -> c_uint {
    unsigned int ceph_str_hash_rjenkins(const char *str, unsigned int length)
    {
    const unsigned char *k = (const unsigned char *)str;
    __u32 a, b, c;  /* the internal state */
    __u32 len;      /* how many key bytes still need mixing */
// Set up the internal state
    len = length;
    a = 0x9e3779b9;      /* the golden ratio; an arbitrary value */
    b = a;
    c = 0;               /* variable initialization of internal state */
// handle most of the key
    while (len >= 12) {
    a = a + (k[0] + ((__u32)k[1] << 8) + ((__u32)k[2] << 16) +
    ((__u32)k[3] << 24));
    b = b + (k[4] + ((__u32)k[5] << 8) + ((__u32)k[6] << 16) +
    ((__u32)k[7] << 24));
    c = c + (k[8] + ((__u32)k[9] << 8) + ((__u32)k[10] << 16) +
    ((__u32)k[11] << 24));
    mix(a, b, c);
    k = k + 12;
    len = len - 12;
    }
// handle the last 11 bytes
    c = c + length;
    switch (len) {
    case 11:
    c = c + ((__u32)k[10] << 24);
    fallthrough;
    case 10:
    c = c + ((__u32)k[9] << 16);
    fallthrough;
    case 9:
    c = c + ((__u32)k[8] << 8);
// the first byte of c is reserved for the length
    fallthrough;
    case 8:
    b = b + ((__u32)k[7] << 24);
    fallthrough;
    case 7:
    b = b + ((__u32)k[6] << 16);
    fallthrough;
    case 6:
    b = b + ((__u32)k[5] << 8);
    fallthrough;
    case 5:
    b = b + k[4];
    fallthrough;
    case 4:
    a = a + ((__u32)k[3] << 24);
    fallthrough;
    case 3:
    a = a + ((__u32)k[2] << 16);
    fallthrough;
    case 2:
    a = a + ((__u32)k[1] << 8);
    fallthrough;
    case 1:
    a = a + k[0];
// case 0: nothing left to add
    }
    mix(a, b, c);
    return c;
    }
//
// linux dcache hash
//
#[no_mangle]
pub unsafe extern "C" fn ceph_str_hash_linux(str: *const c_char, length: c_uint) -> c_uint {
    unsigned int ceph_str_hash_linux(const char *str, unsigned int length)
    {
    let mut hash: c_ulong = 0;
    unsigned char c;
    while (length--) {
    c = *str++;
    hash = (hash + (c << 4) + (c >> 4)) * 11;
    }
    return hash;
    }
#[no_mangle]
pub unsafe extern "C" fn ceph_str_hash(type: c_int, s: *const c_char, len: c_uint) -> c_uint {
    unsigned int ceph_str_hash(int type, const char *s, unsigned int len)
    {
    switch (type) {
    case CEPH_STR_HASH_LINUX:
    return ceph_str_hash_linux(s, len);
    case CEPH_STR_HASH_RJENKINS:
    return ceph_str_hash_rjenkins(s, len);
    default:
    return -1;
    }
    }
    EXPORT_SYMBOL(ceph_str_hash);
    const char *ceph_str_hash_name(int type)
    {
    switch (type) {
    case CEPH_STR_HASH_LINUX:
    return "linux";
    case CEPH_STR_HASH_RJENKINS:
    return "rjenkins";
    default:
    return "unknown";
    }
    }
    EXPORT_SYMBOL(ceph_str_hash_name);
