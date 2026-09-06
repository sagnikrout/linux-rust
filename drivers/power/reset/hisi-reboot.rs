//! Automatically rewritten from C to Rust
//! Source: drivers/power/reset/hisi-reboot.c
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
// HiSilicon SoC reset code
//
// Copyright (c) 2014 HiSilicon Ltd.
// Copyright (c) 2014 Linaro Ltd.
//
// Author: Haojian Zhuang <haojian.zhuang@linaro.org>
//

    static void __iomem *base;
    static u32 reboot_offset;
    static int hisi_restart_handler(struct notifier_block *this,
    unsigned long mode, void *cmd)
    {
    writel_relaxed(0xdeadbeef, base + reboot_offset);
    while (1)
    cpu_do_idle();
    return NOTIFY_DONE;
    }
    static struct notifier_block hisi_restart_nb = {
    .notifier_call = hisi_restart_handler,
    .priority = 128,
    };
#[no_mangle]
unsafe extern "C" fn hisi_reboot_probe(pdev: *mut platform_device) -> c_int {
    static int hisi_reboot_probe(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    int err;
    base = of_iomap(np, 0);
    if (!base) {
    WARN(1, "failed to map base address");
    return -ENODEV;
    }
    if (of_property_read_u32(np, "reboot-offset", &reboot_offset) < 0) {
    pr_err("failed to find reboot-offset property\n");
    iounmap(base);
    return -EINVAL;
    }
    err = register_restart_handler(&hisi_restart_nb);
    if (err) {
    dev_err(&pdev.dev, "cannot register restart handler (err=%d)\n",
    err);
    iounmap(base);
    }
    return err;
    }
    static const struct of_device_id hisi_reboot_of_match[] = {
    { .compatible = "hisilicon,sysctrl" },
    {}
    };
    MODULE_DEVICE_TABLE(of, hisi_reboot_of_match);
    static struct platform_driver hisi_reboot_driver = {
    .probe = hisi_reboot_probe,
    .driver = {
    .name = "hisi-reboot",
    .of_match_table = hisi_reboot_of_match,
    },
    };
    module_platform_driver(hisi_reboot_driver);
