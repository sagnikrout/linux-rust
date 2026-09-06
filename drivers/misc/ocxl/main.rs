//! Automatically rewritten from C to Rust
//! Source: drivers/misc/ocxl/main.c
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
// Copyright 2017 IBM Corp.

#[no_mangle]
unsafe extern "C" fn init_ocxl() -> int __init {
    static int __init init_ocxl(void)
    {
    int rc;
    if (!tlbie_capable)
    return -EINVAL;
    rc = ocxl_file_init();
    if (rc)
    return rc;
    rc = pci_register_driver(&ocxl_pci_driver);
    if (rc) {
    ocxl_file_exit();
    return rc;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn exit_ocxl() {
    static void exit_ocxl(void)
    {
    pci_unregister_driver(&ocxl_pci_driver);
    ocxl_file_exit();
    }
    module_init(init_ocxl);
    module_exit(exit_ocxl);
    MODULE_DESCRIPTION("Open Coherent Accelerator");
    MODULE_LICENSE("GPL");
