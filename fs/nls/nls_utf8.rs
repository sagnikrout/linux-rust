//! Automatically rewritten from C to Rust
//! Source: fs/nls/nls_utf8.c
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
// Module for handling utf8 just like any other charset.
// By Urban Widmark 2000
//

    static unsigned char identity[256];
#[no_mangle]
unsafe extern "C" fn uni2char(uni: wchar_t, out: *mut c_uchar, boundlen: c_int) -> c_int {
    static int uni2char(wchar_t uni, unsigned char *out, int boundlen)
    {
    int n;
    if (boundlen <= 0)
    return -ENAMETOOLONG;
    n = utf32_to_utf8(uni, out, boundlen);
    if (n < 0) {
// out = '?';
    return -EINVAL;
    }
    return n;
    }
#[no_mangle]
unsafe extern "C" fn char2uni(rawstring: *const c_uchar, boundlen: c_int, uni: *mut wchar_t) -> c_int {
    static int char2uni(const unsigned char *rawstring, int boundlen, wchar_t *uni)
    {
    int n;
    unicode_t u;
    n = utf8_to_utf32(rawstring, boundlen, &u);
    if (n < 0 || u > MAX_WCHAR_T) {
// uni = 0x003f;	/* ?
    return -EINVAL;
    }
// uni = (wchar_t) u;
    return n;
    }
    static struct nls_table table = {
    .charset	= "utf8",
    .uni2char	= uni2char,
    .char2uni	= char2uni,
    .charset2lower	= identity,	/* no conversion */
    .charset2upper	= identity,
    };
#[no_mangle]
unsafe extern "C" fn init_nls_utf8() -> int __init {
    static int __init init_nls_utf8(void)
    {
    int i;
    for (i=0; i<256; i++)
    identity[i] = i;
    return register_nls(&table);
    }
#[no_mangle]
unsafe extern "C" fn exit_nls_utf8() -> void __exit {
    static void __exit exit_nls_utf8(void)
    {
    unregister_nls(&table);
    }
    module_init(init_nls_utf8)
    module_exit(exit_nls_utf8)
    MODULE_DESCRIPTION("NLS UTF-8");
    MODULE_LICENSE("Dual BSD/GPL");
