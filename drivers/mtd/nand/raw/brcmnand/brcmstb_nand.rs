//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/nand/raw/brcmnand/brcmstb_nand.c
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
// Copyright © 2015 Broadcom Corporation
//

    static const struct of_device_id brcmstb_nand_of_match[] = {
    { .compatible = "brcm,brcmnand" },
    {},
    };
    MODULE_DEVICE_TABLE(of, brcmstb_nand_of_match);
#[no_mangle]
unsafe extern "C" fn brcmstb_nand_probe(pdev: *mut platform_device) -> c_int {
    static int brcmstb_nand_probe(struct platform_device *pdev)
    {
    return brcmnand_probe(pdev, core::ptr::null_mut());
    }
    static struct platform_driver brcmstb_nand_driver = {
    .probe			= brcmstb_nand_probe,
    .remove			= brcmnand_remove,
    .driver = {
    .name		= "brcmstb_nand",
    .pm		= &brcmnand_pm_ops,
    .of_match_table = brcmstb_nand_of_match,
    }
    };
    module_platform_driver(brcmstb_nand_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Brian Norris");
    MODULE_DESCRIPTION("NAND driver for Broadcom STB chips");
