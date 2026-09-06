//! Automatically rewritten from C to Rust
//! Source: drivers/power/reset/xgene-reboot.c
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
// AppliedMicro X-Gene SoC Reboot Driver
//
// Copyright (c) 2013, Applied Micro Circuits Corporation
// Author: Feng Kan <fkan@apm.com>
// Author: Loc Ho <lho@apm.com>
//
// This driver provides system reboot functionality for APM X-Gene SoC.
// For system shutdown, this is board specify. If a board designer
// implements GPIO shutdown, use the gpio-poweroff.c driver.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xgene_reboot_context {
    pub dev: *mut device,
    pub csr: *mut void __iomem,
    pub mask: u32,
}

#[no_mangle]
unsafe extern "C" fn xgene_restart_handler(data: *mut sys_off_data) -> c_int {
    static int xgene_restart_handler(struct sys_off_data *data)
    {
    struct xgene_reboot_context *ctx = data.cb_data;
// Issue the reboot
    writel(ctx.mask, ctx.csr);
    mdelay(1000);
    dev_emerg(ctx.dev, "Unable to restart system\n");
    return NOTIFY_DONE;
    }
#[no_mangle]
unsafe extern "C" fn xgene_reboot_probe(pdev: *mut platform_device) -> c_int {
    static int xgene_reboot_probe(struct platform_device *pdev)
    {
    struct xgene_reboot_context *ctx;
    struct device *dev = &pdev.dev;
    int err;
    ctx = devm_kzalloc(dev, sizeof(*ctx), GFP_KERNEL);
    if (!ctx)
    return -ENOMEM;
    ctx.csr = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(ctx.csr)) {
    dev_err(dev, "can not map resource\n");
    return PTR_ERR(ctx.csr);
    }
    if (of_property_read_u32(dev.of_node, "mask", &ctx.mask))
    ctx.mask = 0xFFFFFFFF;
    ctx.dev = dev;
    err = devm_register_sys_off_handler(dev, SYS_OFF_MODE_RESTART, 128,
    xgene_restart_handler, ctx);
    if (err)
    dev_err(dev, "cannot register restart handler (err=%d)\n", err);
    return err;
    }
    static const struct of_device_id xgene_reboot_of_match[] = {
    { .compatible = "apm,xgene-reboot" },
    {}
    };
    static struct platform_driver xgene_reboot_driver = {
    .probe = xgene_reboot_probe,
    .driver = {
    .name = "xgene-reboot",
    .of_match_table = xgene_reboot_of_match,
    },
    };
    builtin_platform_driver(xgene_reboot_driver);
