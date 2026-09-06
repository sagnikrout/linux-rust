//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/pseries/plpks_sed_ops.c
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
// POWER Platform specific code for non-volatile SED key access
// Copyright (C) 2022 IBM Corporation
//
// Define operations for SED Opal to read/write keys
// from POWER LPAR Platform KeyStore(PLPKS).
//
// Self Encrypting Drives(SED) key storage using PLPKS
//

    let mut plpks_sed_initialized: static bool = false;
    let mut plpks_sed_available: static bool = false;
//
// structure that contains all SED data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct plpks_sed_object_data {
    pub version: u_char,
    pub pad1: [u_char; 7],
    pub authority: u_long,
    pub range: u_long,
    pub key_len: u_int,
    pub key: [u_char; 32],
}

pub const PLPKS_SED_OBJECT_DATA_V0: c_int = 0;

//
// authority is admin1 and range is global
//
pub const PLPKS_SED_AUTHORITY: c_uint = 0x0000000900010001;
pub const PLPKS_SED_RANGE: c_uint = 0x0000080200000001;
#[no_mangle]
unsafe extern "C" fn plpks_init_var(var: *mut plpks_var, keyname: *mut c_char) {
    static void plpks_init_var(struct plpks_var *var, char *keyname)
    {
    if (!plpks_sed_initialized) {
    plpks_sed_initialized = true;
    plpks_sed_available = plpks_is_available();
    if (!plpks_sed_available)
    pr_err("SED: plpks not available\n");
    }
    var.name = keyname;
    var.namelen = strlen(keyname);
    if (strcmp(PLPKS_SED_KEY, keyname) == 0) {
    var.name = PLPKS_SED_MANGLED_LABEL;
    var.namelen = strlen(keyname);
    }
    var.policy = PLPKS_WORLDREADABLE;
    var.os = PLPKS_VAR_COMMON;
    var.data = core::ptr::null_mut();
    var.datalen = 0;
    var.component = PLPKS_SED_COMPONENT;
    }
//
// Read the SED Opal key from PLPKS given the label
//
#[no_mangle]
pub unsafe extern "C" fn sed_read_key(keyname: *mut c_char, key: *mut c_char, keylen: *mut u_int) -> c_int {
    int sed_read_key(char *keyname, char *key, u_int *keylen)
    {
    struct plpks_var var;
    struct plpks_sed_object_data data;
    int ret;
    u_int len;
    plpks_init_var(&var, keyname);
    if (!plpks_sed_available)
    return -EOPNOTSUPP;
    var.data = (u8 *)&data;
    var.datalen = sizeof(data);
    ret = plpks_read_os_var(&var);
    if (ret != 0)
    return ret;
    len = min_t(u16, be32_to_cpu(data.key_len), var.datalen);
    memcpy(key, data.key, len);
    key[len] = '\0';
// keylen = len;
    return 0;
    }
//
// Write the SED Opal key to PLPKS given the label
//
#[no_mangle]
pub unsafe extern "C" fn sed_write_key(keyname: *mut c_char, key: *mut c_char, keylen: u_int) -> c_int {
    int sed_write_key(char *keyname, char *key, u_int keylen)
    {
    struct plpks_var var;
    struct plpks_sed_object_data data;
    struct plpks_var_name vname;
    plpks_init_var(&var, keyname);
    if (!plpks_sed_available)
    return -EOPNOTSUPP;
    var.datalen = sizeof(struct plpks_sed_object_data);
    var.data = (u8 *)&data;
// initialize SED object
    data.version = PLPKS_SED_OBJECT_DATA_V0;
    data.authority = cpu_to_be64(PLPKS_SED_AUTHORITY);
    data.range = cpu_to_be64(PLPKS_SED_RANGE);
    memset(&data.pad1, '\0', sizeof(data.pad1));
    data.key_len = cpu_to_be32(keylen);
    memcpy(data.key, (char *)key, keylen);
//
// Key update requires remove first. The return value
// is ignored since it's okay if the key doesn't exist.
//
    vname.namelen = var.namelen;
    vname.name = var.name;
    plpks_remove_var(var.component, var.os, vname);
    return plpks_write_var(var);
    }
