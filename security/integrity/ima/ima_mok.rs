//! Automatically rewritten from C to Rust
//! Source: security/integrity/ima/ima_mok.c
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
// Copyright (C) 2015 Juniper Networks, Inc.
//
// Author:
// Petko Manolov <petko.manolov@konsulko.com>
//

    struct key *ima_blacklist_keyring;
//
// Allocate the IMA blacklist keyring
//
#[no_mangle]
unsafe extern "C" fn ima_mok_init() -> __init int {
    static __init int ima_mok_init(void)
    {
    struct key_restriction *restriction;
    pr_notice("Allocating IMA blacklist keyring.\n");
    restriction = kzalloc_obj(struct key_restriction);
    if (!restriction)
    panic("Can't allocate IMA blacklist restriction.");
    restriction.check = restrict_link_by_builtin_trusted;
    ima_blacklist_keyring = keyring_alloc(".ima_blacklist",
    KUIDT_INIT(0), KGIDT_INIT(0), current_cred(),
    (KEY_POS_ALL & ~KEY_POS_SETATTR) |
    KEY_USR_VIEW | KEY_USR_READ |
    KEY_USR_WRITE | KEY_USR_SEARCH,
    KEY_ALLOC_NOT_IN_QUOTA |
    KEY_ALLOC_SET_KEEP,
    restriction, core::ptr::null_mut());
    if (IS_ERR(ima_blacklist_keyring))
    panic("Can't allocate IMA blacklist keyring.");
    return 0;
    }
    device_initcall(ima_mok_init);
