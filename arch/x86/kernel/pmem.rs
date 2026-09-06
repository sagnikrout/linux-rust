//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/pmem.c
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
// Copyright (c) 2015, Christoph Hellwig.
// Copyright (c) 2015, Intel Corporation.
//

#[no_mangle]
unsafe extern "C" fn found(res: *mut resource, data: *mut c_void) -> c_int {
    static int found(struct resource *res, void *data)
    {
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn register_e820_pmem() -> __init int {
    static __init int register_e820_pmem(void)
    {
    struct platform_device *pdev;
    int rc;
    rc = walk_iomem_res_desc(IORES_DESC_PERSISTENT_MEMORY_LEGACY,
    IORESOURCE_MEM, 0, -1, core::ptr::null_mut(), found);
    if (rc <= 0)
    return 0;
//
// See drivers/nvdimm/e820.c for the implementation, this is
// simply here to trigger the module to load on demand.
//
    pdev = platform_device_alloc("e820_pmem", -1);
    if (!pdev)
    return -ENOMEM;
    rc = platform_device_add(pdev);
    if (rc)
    platform_device_put(pdev);
    return rc;
    }
    device_initcall(register_e820_pmem);
