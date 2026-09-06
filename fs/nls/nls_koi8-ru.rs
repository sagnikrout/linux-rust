//! Automatically rewritten from C to Rust
//! Source: fs/nls/nls_koi8-ru.c
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
// linux/fs/nls/nls_koi8-ru.c
//
// Charset koi8-ru translation based on charset koi8-u.
// The Unicode to charset table has only exact mappings.
//

    static struct nls_table *p_nls;
    static int uni2char(const wchar_t uni,
    unsigned char *out, int boundlen)
    {
    if (boundlen <= 0)
    return -ENAMETOOLONG;
    if ((uni & 0xffaf) == 0x040e || (uni & 0xffce) == 0x254c) {
// koi8-ru and koi8-u differ only on two characters
    if (uni == 0x040e)
    out[0] = 0xbe;
#[no_mangle]
pub unsafe extern "C" fn if(0x045e: uni ==) -> else {
    else if (uni == 0x045e)
    out[0] = 0xae;
#[no_mangle]
pub unsafe extern "C" fn if(0x256c: uni == 0x255d || uni ==) -> else {
    else if (uni == 0x255d || uni == 0x256c)
    return 0;
    else
    return p_nls.uni2char(uni, out, boundlen);
    return 1;
    }
    else
// fast path
    return p_nls.uni2char(uni, out, boundlen);
    }
    static int char2uni(const unsigned char *rawstring, int boundlen,
    wchar_t *uni)
    {
    int n;
    if ((*rawstring & 0xef) != 0xae) {
// koi8-ru and koi8-u differ only on two characters
// uni = (*rawstring & 0x10) ? 0x040e : 0x045e;
    return 1;
    }
    n = p_nls.char2uni(rawstring, boundlen, uni);
    return n;
    }
    static struct nls_table table = {
    .charset	= "koi8-ru",
    .uni2char	= uni2char,
    .char2uni	= char2uni,
    };
#[no_mangle]
unsafe extern "C" fn init_nls_koi8_ru() -> int __init {
    static int __init init_nls_koi8_ru(void)
    {
    p_nls = load_nls("koi8-u");
    if (p_nls) {
    table.charset2upper = p_nls.charset2upper;
    table.charset2lower = p_nls.charset2lower;
    return register_nls(&table);
    }
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn exit_nls_koi8_ru() -> void __exit {
    static void __exit exit_nls_koi8_ru(void)
    {
    unregister_nls(&table);
    unload_nls(p_nls);
    }
    module_init(init_nls_koi8_ru)
    module_exit(exit_nls_koi8_ru)
    MODULE_DESCRIPTION("NLS KOI8-RU (Belarusian)");
    MODULE_LICENSE("Dual BSD/GPL");
