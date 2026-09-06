//! Automatically rewritten from C to Rust
//! Source: security/integrity/platform_certs/platform_keyring.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Platform keyring for firmware/platform keys
//
// Copyright IBM Corporation, 2018
// Author(s): Nayna Jain <nayna@linux.ibm.com>
//

//
// add_to_platform_keyring - Add to platform keyring without validation.
// @source: Source of key
// @data: The blob holding the key
// @len: The length of the data blob
//
// Add a key to the platform keyring without checking its trust chain.  This
// is available only during kernel initialisation.
//
    void __init add_to_platform_keyring(const char *source, const void *data,
    size_t len)
    {
    key_perm_t perm;
    int rc;
    perm = (KEY_POS_ALL & ~KEY_POS_SETATTR) | KEY_USR_VIEW;
    rc = integrity_load_cert(INTEGRITY_KEYRING_PLATFORM, source, data, len,
    perm);
    if (rc)
    pr_info("Error adding keys to platform keyring %s\n", source);
    }
//
// Create the trusted keyrings.
//
#[no_mangle]
unsafe extern "C" fn platform_keyring_init() -> __init int {
    static __init int platform_keyring_init(void)
    {
    int rc;
    rc = integrity_init_keyring(INTEGRITY_KEYRING_PLATFORM);
    if (rc)
    return rc;
    pr_notice("Platform Keyring initialized\n");
    return 0;
    }
//
// Must be initialised before we try and load the keys into the keyring.
//
    device_initcall(platform_keyring_init);
