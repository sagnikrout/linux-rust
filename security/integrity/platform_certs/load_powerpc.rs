//! Automatically rewritten from C to Rust
//! Source: security/integrity/platform_certs/load_powerpc.c
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
// Copyright (C) 2019 IBM Corporation
// Author: Nayna Jain
//
// - loads keys and hashes stored and controlled by the firmware.
//

    do { db = data + offset; size = size - offset; } while (0)
//
// Get a certificate list blob from the named secure variable.
//
// Returns:
// - a pointer to a kmalloc'd buffer containing the cert list on success
// - NULL if the key does not exist
// - an ERR_PTR on error
//
    static __init void *get_cert_list(u8 *key, unsigned long keylen, u64 *size)
    {
    int rc;
    void *db;
    rc = secvar_ops.get(key, keylen, core::ptr::null_mut(), size);
    if (rc) {
    if (rc == -ENOENT)
    return core::ptr::null_mut();
    return ERR_PTR(rc);
    }
    db = kmalloc(*size, GFP_KERNEL);
    if (!db)
    return ERR_PTR(-ENOMEM);
    rc = secvar_ops.get(key, keylen, db, size);
    if (rc) {
    kfree(db);
    return ERR_PTR(rc);
    }
    return db;
    }
//
// Load the certs contained in the keys databases into the platform trusted
// keyring and the blacklisted X.509 cert SHA256 hashes into the blacklist
// keyring.
//
#[no_mangle]
unsafe extern "C" fn load_powerpc_certs() -> int __init {
    static int __init load_powerpc_certs(void)
    {
    void *db = core::ptr::null_mut(), *dbx = core::ptr::null_mut(), *data = core::ptr::null_mut();
    void *trustedca;
    void *moduledb;
    let mut dsize: u64 = 0;
    let mut offset: u64 = 0;
    let mut rc: c_int = 0;
    ssize_t len;
    char buf[32];
    if (!secvar_ops)
    return -ENODEV;
    len = secvar_ops.format(buf, sizeof(buf));
    if (len <= 0)
    return -ENODEV;
// Check for known secure boot implementations from OPAL or PLPKS
    if (strcmp("ibm,edk2-compat-v1", buf) && strcmp("ibm,plpks-sb-v1", buf) &&
    strcmp("ibm,plpks-sb-v0", buf)) {
    pr_err("Unsupported secvar implementation \"%s\", not loading certs\n", buf);
    return -ENODEV;
    }
    if (strcmp("ibm,plpks-sb-v1", buf) == 0 || strcmp("ibm,plpks-sb-v0", buf) == 0)
// PLPKS authenticated variables ESL data is prefixed with 8 bytes of timestamp
    offset = 8;
//
// Get db, and dbx. They might not exist, so it isn't an error if we
// can't get them.
//
    data = get_cert_list("db", 3, &dsize);
    if (!data) {
    pr_info("Couldn't get db list from firmware\n");
    } else if (IS_ERR(data)) {
    rc = PTR_ERR(data);
    pr_err("Error reading db from firmware: %d\n", rc);
    return rc;
    } else {
    extract_esl(db, data, dsize, offset);
    rc = parse_efi_signature_list("powerpc:db", db, dsize,
    get_handler_for_db);
    if (rc)
    pr_err("Couldn't parse db signatures: %d\n", rc);
    kfree(data);
    }
    data = get_cert_list("dbx", 4,  &dsize);
    if (!data) {
    pr_info("Couldn't get dbx list from firmware\n");
    } else if (IS_ERR(data)) {
    rc = PTR_ERR(data);
    pr_err("Error reading dbx from firmware: %d\n", rc);
    return rc;
    } else {
    extract_esl(dbx, data, dsize, offset);
    rc = parse_efi_signature_list("powerpc:dbx", dbx, dsize,
    get_handler_for_dbx);
    if (rc)
    pr_err("Couldn't parse dbx signatures: %d\n", rc);
    kfree(data);
    }
    data = get_cert_list("trustedcadb", 12,  &dsize);
    if (!data) {
    pr_info("Couldn't get trustedcadb list from firmware\n");
    } else if (IS_ERR(data)) {
    rc = PTR_ERR(data);
    pr_err("Error reading trustedcadb from firmware: %d\n", rc);
    } else {
    extract_esl(trustedca, data, dsize, offset);
    rc = parse_efi_signature_list("powerpc:trustedca", trustedca, dsize,
    get_handler_for_ca_keys);
    if (rc)
    pr_err("Couldn't parse trustedcadb signatures: %d\n", rc);
    kfree(data);
    }
    data = get_cert_list("moduledb", 9,  &dsize);
    if (!data) {
    pr_info("Couldn't get moduledb list from firmware\n");
    } else if (IS_ERR(data)) {
    rc = PTR_ERR(data);
    pr_err("Error reading moduledb from firmware: %d\n", rc);
    } else {
    extract_esl(moduledb, data, dsize, offset);
    rc = parse_efi_signature_list("powerpc:moduledb", moduledb, dsize,
    get_handler_for_code_signing_keys);
    if (rc)
    pr_err("Couldn't parse moduledb signatures: %d\n", rc);
    kfree(data);
    }
    return rc;
    }
    late_initcall(load_powerpc_certs);
