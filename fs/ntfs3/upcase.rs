//! Automatically rewritten from C to Rust
//! Source: fs/ntfs3/upcase.c
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
//
// Copyright (C) 2019-2021 Paragon Software GmbH, All rights reserved.
//

#[no_mangle]
pub unsafe extern "C" fn upcase_unicode_char(upcase: *const u16, chr: u16) -> u16 {
    static inline u16 upcase_unicode_char(const u16 *upcase, u16 chr)
    {
    if (chr < 'a')
    return chr;
    if (chr <= 'z')
    return chr - ('a' - 'A');
    return upcase[chr];
    }
//
// ntfs_cmp_names
//
// Thanks Kari Argillander <kari.argillander@gmail.com> for idea and implementation 'bothcase'
//
// Straight way to compare names:
// - Case insensitive
// - If name equals and 'bothcases' then
// - Case sensitive
// 'Straight way' code scans input names twice in worst case.
// Optimized code scans input names only once.
//
    int ntfs_cmp_names(const __le16 *s1, size_t l1, const __le16 *s2, size_t l2,
    const u16 *upcase, bool bothcase)
    {
    let mut diff1: c_int = 0;
    int diff2;
    let mut len: usize = min(l1, l2);
    if (!bothcase && upcase)
    goto case_insentive;
    for (; len; s1++, s2++, len--) {
    diff1 = le16_to_cpu(*s1) - le16_to_cpu(*s2);
    if (diff1) {
    if (bothcase && upcase)
    goto case_insentive;
    return diff1;
    }
    }
    return l1 - l2;
    case_insentive:
    for (; len; s1++, s2++, len--) {
    diff2 = upcase_unicode_char(upcase, le16_to_cpu(*s1)) -
    upcase_unicode_char(upcase, le16_to_cpu(*s2));
    if (diff2)
    return diff2;
    }
    diff2 = l1 - l2;
    return diff2 ? diff2 : diff1;
    }
    int ntfs_cmp_names_cpu(const struct cpu_str *uni1, const struct le_str *uni2,
    const u16 *upcase, bool bothcase)
    {
    const u16 *s1 = uni1.name;
    const __le16 *s2 = uni2.name;
    let mut l1: usize = uni1.len;
    let mut l2: usize = uni2.len;
    let mut len: usize = min(l1, l2);
    let mut diff1: c_int = 0;
    int diff2;
    if (!bothcase && upcase)
    goto case_insentive;
    for (; len; s1++, s2++, len--) {
    diff1 = *s1 - le16_to_cpu(*s2);
    if (diff1) {
    if (bothcase && upcase)
    goto case_insentive;
    return diff1;
    }
    }
    return l1 - l2;
    case_insentive:
    for (; len; s1++, s2++, len--) {
    diff2 = upcase_unicode_char(upcase, *s1) -
    upcase_unicode_char(upcase, le16_to_cpu(*s2));
    if (diff2)
    return diff2;
    }
    diff2 = l1 - l2;
    return diff2 ? diff2 : diff1;
    }
// Helper function for ntfs_d_hash.
    unsigned long ntfs_names_hash(const u16 *name, size_t len, const u16 *upcase,
    unsigned long hash)
    {
    while (len--) {
    let mut c: c_uint = upcase_unicode_char(upcase, *name++);
    hash = partial_name_hash(c, hash);
    }
    return hash;
    }
