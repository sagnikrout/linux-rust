//! Automatically rewritten from C to Rust
//! Source: drivers/char/hw_random/powernv-rng.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright 2013 Michael Ellerman, Guo Chao, IBM Corp.
//

#[no_mangle]
unsafe extern "C" fn powernv_rng_read(rng: *mut hwrng, data: *mut c_void, max: usize, wait: bool) -> c_int {
    static int powernv_rng_read(struct hwrng *rng, void *data, size_t max, bool wait)
    {
    unsigned long *buf;
    int i, len;
// We rely on rng_buffer_size() being >= sizeof(unsigned long)
    len = max / sizeof(unsigned long);
    buf = (unsigned long *)data;
    for (i = 0; i < len; i++)
    pnv_get_random_long(buf++);
    return len * sizeof(unsigned long);
    }
    static struct hwrng powernv_hwrng = {
    .name = "powernv-rng",
    .read = powernv_rng_read,
    };
#[no_mangle]
unsafe extern "C" fn powernv_rng_probe(pdev: *mut platform_device) -> c_int {
    static int powernv_rng_probe(struct platform_device *pdev)
    {
    int rc;
    rc = devm_hwrng_register(&pdev.dev, &powernv_hwrng);
    if (rc) {
// We only register one device, ignore any others
    if (rc == -EEXIST)
    rc = -ENODEV;
    return rc;
    }
    pr_info("Registered powernv hwrng.\n");
    return 0;
    }
    static const struct of_device_id powernv_rng_match[] = {
    { .compatible	= "ibm,power-rng",},
    {},
    };
    MODULE_DEVICE_TABLE(of, powernv_rng_match);
    static struct platform_driver powernv_rng_driver = {
    .driver = {
    .name = "powernv_rng",
    .of_match_table = powernv_rng_match,
    },
    .probe	= powernv_rng_probe,
    };
    module_platform_driver(powernv_rng_driver);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Bare metal HWRNG driver for POWER7+ and above");
