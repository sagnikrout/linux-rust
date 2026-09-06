//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/powernv/opal-secvar.c
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
// PowerNV code for secure variables
//
// Copyright (C) 2019 IBM Corporation
// Author: Claudio Carvalho
// Nayna Jain
//
// APIs to access secure variables managed by OPAL.
//

#[no_mangle]
unsafe extern "C" fn opal_status_to_err(rc: c_int) -> c_int {
    static int opal_status_to_err(int rc)
    {
    int err;
    switch (rc) {
    case OPAL_SUCCESS:
    err = 0;
    break;
    case OPAL_UNSUPPORTED:
    err = -ENXIO;
    break;
    case OPAL_PARAMETER:
    err = -EINVAL;
    break;
    case OPAL_RESOURCE:
    err = -ENOSPC;
    break;
    case OPAL_HARDWARE:
    err = -EIO;
    break;
    case OPAL_NO_MEM:
    err = -ENOMEM;
    break;
    case OPAL_EMPTY:
    err = -ENOENT;
    break;
    case OPAL_PARTIAL:
    err = -EFBIG;
    break;
    default:
    err = -EINVAL;
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn opal_get_variable(key: *const c_char, ksize: u64, data: *mut u8, dsize: *mut u64) -> c_int {
    static int opal_get_variable(const char *key, u64 ksize, u8 *data, u64 *dsize)
    {
    int rc;
    if (!key || !dsize)
    return -EINVAL;
// dsize = cpu_to_be64(*dsize);
    rc = opal_secvar_get(key, ksize, data, dsize);
// dsize = be64_to_cpu(*dsize);
    return opal_status_to_err(rc);
    }
#[no_mangle]
unsafe extern "C" fn opal_get_next_variable(key: *const c_char, keylen: *mut u64, keybufsize: u64) -> c_int {
    static int opal_get_next_variable(const char *key, u64 *keylen, u64 keybufsize)
    {
    int rc;
    if (!key || !keylen)
    return -EINVAL;
// keylen = cpu_to_be64(*keylen);
    rc = opal_secvar_get_next(key, keylen, keybufsize);
// keylen = be64_to_cpu(*keylen);
    return opal_status_to_err(rc);
    }
#[no_mangle]
unsafe extern "C" fn opal_set_variable(key: *const c_char, ksize: u64, data: *mut u8, dsize: u64) -> c_int {
    static int opal_set_variable(const char *key, u64 ksize, u8 *data, u64 dsize)
    {
    int rc;
    if (!key || !data)
    return -EINVAL;
    rc = opal_secvar_enqueue_update(key, ksize, data, dsize);
    return opal_status_to_err(rc);
    }
#[no_mangle]
unsafe extern "C" fn opal_secvar_format(buf: *mut c_char, bufsize: usize) -> isize {
    static ssize_t opal_secvar_format(char *buf, size_t bufsize)
    {
    let mut rc: isize = 0;
    struct device_node *node;
    const char *format;
    node = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "ibm,secvar-backend");
    if (!of_device_is_available(node)) {
    rc = -ENODEV;
    goto out;
    }
    rc = of_property_read_string(node, "format", &format);
    if (rc)
    goto out;
    rc = snprintf(buf, bufsize, "%s", format);
    out:
    of_node_put(node);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn opal_secvar_max_size(max_size: *mut u64) -> c_int {
    static int opal_secvar_max_size(u64 *max_size)
    {
    int rc;
    struct device_node *node;
    node = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "ibm,secvar-backend");
    if (!node)
    return -ENODEV;
    if (!of_device_is_available(node)) {
    rc = -ENODEV;
    goto out;
    }
    rc = of_property_read_u64(node, "max-var-size", max_size);
    out:
    of_node_put(node);
    return rc;
    }
    static const struct secvar_operations opal_secvar_ops = {
    .get = opal_get_variable,
    .get_next = opal_get_next_variable,
    .set = opal_set_variable,
    .format = opal_secvar_format,
    .max_size = opal_secvar_max_size,
    };
#[no_mangle]
unsafe extern "C" fn opal_secvar_probe(pdev: *mut platform_device) -> c_int {
    static int opal_secvar_probe(struct platform_device *pdev)
    {
    if (!opal_check_token(OPAL_SECVAR_GET)
    || !opal_check_token(OPAL_SECVAR_GET_NEXT)
    || !opal_check_token(OPAL_SECVAR_ENQUEUE_UPDATE)) {
    pr_err("OPAL doesn't support secure variables\n");
    return -ENODEV;
    }
    return set_secvar_ops(&opal_secvar_ops);
    }
    static const struct of_device_id opal_secvar_match[] = {
    { .compatible = "ibm,secvar-backend",},
    {},
    };
    static struct platform_driver opal_secvar_driver = {
    .driver = {
    .name = "secvar",
    .of_match_table = opal_secvar_match,
    },
    };
#[no_mangle]
unsafe extern "C" fn opal_secvar_init() -> int __init {
    static int __init opal_secvar_init(void)
    {
    return platform_driver_probe(&opal_secvar_driver, opal_secvar_probe);
    }
    device_initcall(opal_secvar_init);
