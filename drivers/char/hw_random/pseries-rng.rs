//! Automatically rewritten from C to Rust
//! Source: drivers/char/hw_random/pseries-rng.c
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
// Copyright (C) 2010 Michael Neuling IBM Corporation
//
// Driver for the pseries hardware RNG for POWER7+ and above
//

#[no_mangle]
unsafe extern "C" fn pseries_rng_read(rng: *mut hwrng, data: *mut c_void, max: usize, wait: bool) -> c_int {
    static int pseries_rng_read(struct hwrng *rng, void *data, size_t max, bool wait)
    {
    u64 buffer[PLPAR_HCALL_BUFSIZE];
    int rc;
    rc = plpar_hcall(H_RANDOM, (unsigned long *)buffer);
    if (rc != H_SUCCESS) {
    pr_err_ratelimited("H_RANDOM call failed %d\n", rc);
    return -EIO;
    }
    memcpy(data, buffer, 8);
// The hypervisor interface returns 64 bits
    return 8;
    }
//
// pseries_rng_get_desired_dma - Return desired DMA allocate for CMO operations
//
// This is a required function for a driver to operate in a CMO environment
// but this device does not make use of DMA allocations, return 0.
//
// Return value:
// Number of bytes of IO data the driver will need to perform well -> 0
//
#[no_mangle]
unsafe extern "C" fn pseries_rng_get_desired_dma(vdev: *mut vio_dev) -> c_ulong {
    static unsigned long pseries_rng_get_desired_dma(struct vio_dev *vdev)
    {
    return 0;
    };
    static struct hwrng pseries_rng = {
    .name		= KBUILD_MODNAME,
    .read		= pseries_rng_read,
    };
    static int pseries_rng_probe(struct vio_dev *dev,
    const struct vio_device_id *id)
    {
    return hwrng_register(&pseries_rng);
    }
#[no_mangle]
unsafe extern "C" fn pseries_rng_remove(dev: *mut vio_dev) {
    static void pseries_rng_remove(struct vio_dev *dev)
    {
    hwrng_unregister(&pseries_rng);
    }
    static const struct vio_device_id pseries_rng_driver_ids[] = {
    { "ibm,random-v1", "ibm,random"},
    { "", "" }
    };
    MODULE_DEVICE_TABLE(vio, pseries_rng_driver_ids);
    static struct vio_driver pseries_rng_driver = {
    .name = KBUILD_MODNAME,
    .probe = pseries_rng_probe,
    .remove = pseries_rng_remove,
    .get_desired_dma = pseries_rng_get_desired_dma,
    .id_table = pseries_rng_driver_ids
    };
#[no_mangle]
unsafe extern "C" fn rng_init() -> int __init {
    static int __init rng_init(void)
    {
    pr_info("Registering IBM pSeries RNG driver\n");
    return vio_register_driver(&pseries_rng_driver);
    }
    module_init(rng_init);
#[no_mangle]
unsafe extern "C" fn rng_exit() -> void __exit {
    static void __exit rng_exit(void)
    {
    vio_unregister_driver(&pseries_rng_driver);
    }
    module_exit(rng_exit);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Michael Neuling <mikey@neuling.org>");
    MODULE_DESCRIPTION("H/W RNG driver for IBM pSeries processors");
