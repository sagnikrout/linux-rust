//! Automatically rewritten from C to Rust
//! Source: lib/net_utils.c
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

#[no_mangle]
pub unsafe extern "C" fn mac_pton(s: *const c_char, mac: *mut u8) -> bool {
    bool mac_pton(const char *s, u8 *mac)
    {
    int i;
    if (strnlen(s, MAC_ADDR_STR_LEN) < MAC_ADDR_STR_LEN)
    return false;
// Don't dirty result unless string is valid MAC.
    for (i = 0; i < ETH_ALEN; i++) {
    if (!isxdigit(s[i * 3]) || !isxdigit(s[i * 3 + 1]))
    return false;
    if (i != ETH_ALEN - 1 && s[i * 3 + 2] != ':')
    return false;
    }
    for (i = 0; i < ETH_ALEN; i++) {
    mac[i] = (hex_to_bin(s[i * 3]) << 4) | hex_to_bin(s[i * 3 + 1]);
    }
    return true;
    }
    EXPORT_SYMBOL(mac_pton);
