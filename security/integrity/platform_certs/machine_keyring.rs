//! Automatically rewritten from C to Rust
//! Source: security/integrity/platform_certs/machine_keyring.c
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
// Machine keyring routines.
//
// Copyright (c) 2021, Oracle and/or its affiliates.
//

#[no_mangle]
unsafe extern "C" fn machine_keyring_init() -> __init int {
    static __init int machine_keyring_init(void)
    {
    int rc;
    rc = integrity_init_keyring(INTEGRITY_KEYRING_MACHINE);
    if (rc)
    return rc;
    pr_notice("Machine keyring initialized\n");
    return 0;
    }
    device_initcall(machine_keyring_init);
#[no_mangle]
pub unsafe extern "C" fn add_to_machine_keyring(source: *const c_char, data: *const c_void, len: usize) -> void __init {
    void __init add_to_machine_keyring(const char *source, const void *data, size_t len)
    {
    key_perm_t perm;
    int rc;
    perm = (KEY_POS_ALL & ~KEY_POS_SETATTR) | KEY_USR_VIEW;
    rc = integrity_load_cert(INTEGRITY_KEYRING_MACHINE, source, data, len, perm);
//
// Some MOKList keys may not pass the machine keyring restrictions.
// If the restriction check does not pass and the platform keyring
// is configured, try to add it into that keyring instead.
//
    if (rc && efi_enabled(EFI_BOOT) &&
    IS_ENABLED(CONFIG_INTEGRITY_PLATFORM_KEYRING))
    rc = integrity_load_cert(INTEGRITY_KEYRING_PLATFORM, source,
    data, len, perm);
    if (rc)
    pr_info("Error adding keys to machine keyring %s\n", source);
    }
//
// Try to load the MokListTrustedRT MOK variable to see if we should trust
// the MOK keys within the kernel. It is not an error if this variable
// does not exist.  If it does not exist, MOK keys should not be trusted
// within the machine keyring.
//
#[no_mangle]
unsafe extern "C" fn uefi_check_trust_mok_keys() -> __init bool {
    static __init bool uefi_check_trust_mok_keys(void)
    {
    struct efi_mokvar_table_entry *mokvar_entry;
    mokvar_entry = efi_mokvar_entry_find("MokListTrustedRT");
    if (mokvar_entry)
    return true;
    return false;
    }
#[no_mangle]
unsafe extern "C" fn trust_moklist() -> bool __init {
    static bool __init trust_moklist(void)
    {
    static bool initialized;
    static bool trust_mok;
    if (!initialized) {
    initialized = true;
    trust_mok = false;
    if (uefi_check_trust_mok_keys())
    trust_mok = true;
    }
    return trust_mok;
    }
//
// Provides platform specific check for trusting imputed keys before loading
// on .machine keyring. UEFI systems enable this trust based on a variable,
// and for other platforms, it is always enabled.
//
#[no_mangle]
pub unsafe extern "C" fn imputed_trust_enabled() -> bool __init {
    bool __init imputed_trust_enabled(void)
    {
    if (efi_enabled(EFI_BOOT))
    return trust_moklist();
    return true;
    }
