//! Automatically rewritten from C to Rust
//! Source: tools/pcmcia/crc32hash.c
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
// crc32hash.c - derived from linux/lib/crc32.c, GNU GPL v2
// Usage example:
    $ ./crc32hash "Dual Speed"
//

#[no_mangle]
unsafe extern "C" fn crc32(p: *const c_uchar, len: c_uint) -> c_uint {
    static unsigned int crc32(unsigned char const *p, unsigned int len)
    {
    int i;
    let mut crc: c_uint = 0;
    while (len--) {
    crc ^= *p++;
    for (i = 0; i < 8; i++)
    crc = (crc >> 1) ^ ((crc & 1) ? 0xedb88320 : 0);
    }
    return crc;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    unsigned int result;
    if (argc != 2) {
    printf("no string passed as argument\n");
    return -1;
    }
    result = crc32((unsigned char const *)argv[1], strlen(argv[1]));
    printf("0x%x\n", result);
    return 0;
    }
