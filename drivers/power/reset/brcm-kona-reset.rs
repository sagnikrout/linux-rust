//! Automatically rewritten from C to Rust
//! Source: drivers/power/reset/brcm-kona-reset.c
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
// Copyright (C) 2016 Broadcom

pub const RSTMGR_REG_WR_ACCESS_OFFSET: c_int = 0;
pub const RSTMGR_REG_CHIP_SOFT_RST_OFFSET: c_int = 4;
pub const RSTMGR_WR_PASSWORD: c_uint = 0xa5a5;
pub const RSTMGR_WR_PASSWORD_SHIFT: c_int = 8;
pub const RSTMGR_WR_ACCESS_ENABLE: c_int = 1;
    static void __iomem *kona_reset_base;
#[no_mangle]
unsafe extern "C" fn kona_reset_handler(data: *mut sys_off_data) -> c_int {
    static int kona_reset_handler(struct sys_off_data *data)
    {
//
// A soft reset is triggered by writing a 0 to bit 0 of the soft reset
// register. To write to that register we must first write the password
// and the enable bit in the write access enable register.
//
    writel((RSTMGR_WR_PASSWORD << RSTMGR_WR_PASSWORD_SHIFT) |
    RSTMGR_WR_ACCESS_ENABLE,
    kona_reset_base + RSTMGR_REG_WR_ACCESS_OFFSET);
    writel(0, kona_reset_base + RSTMGR_REG_CHIP_SOFT_RST_OFFSET);
    return NOTIFY_DONE;
    }
#[no_mangle]
unsafe extern "C" fn kona_reset_probe(pdev: *mut platform_device) -> c_int {
    static int kona_reset_probe(struct platform_device *pdev)
    {
    kona_reset_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(kona_reset_base))
    return PTR_ERR(kona_reset_base);
    return devm_register_sys_off_handler(&pdev.dev, SYS_OFF_MODE_RESTART,
    128, kona_reset_handler, core::ptr::null_mut());
    }
    static const struct of_device_id of_match[] = {
    { .compatible = "brcm,bcm21664-resetmgr" },
    {},
    };
    static struct platform_driver bcm_kona_reset_driver = {
    .probe = kona_reset_probe,
    .driver = {
    .name = "brcm-kona-reset",
    .of_match_table = of_match,
    },
    };
    builtin_platform_driver(bcm_kona_reset_driver);
