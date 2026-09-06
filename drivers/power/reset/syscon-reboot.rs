//! Automatically rewritten from C to Rust
//! Source: drivers/power/reset/syscon-reboot.c
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
// Generic Syscon Reboot Driver
//
// Copyright (c) 2013, Applied Micro Circuits Corporation
// Author: Feng Kan <fkan@apm.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reboot_mode_bits {
    pub offset: u32,
    pub mask: u32,
    pub value: u32,
    pub valid: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reboot_data {
    pub 1]: reboot_mode_bits mode_bits[REBOOT_SOFT +,
    pub catchall: reboot_mode_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct syscon_reboot_context {
    pub map: *mut regmap,
    pub /: *const *const *const reboot_data rd; / from of match data, if any,
    pub /: *mut *mut reboot_mode_bits catchall; / from DT,
    pub restart_handler: notifier_block,
}

    static int syscon_restart_handle(struct notifier_block *this,
    unsigned long mode, void *cmd)
    {
    struct syscon_reboot_context *ctx =
    container_of(this, struct syscon_reboot_context,
    restart_handler);
    const struct reboot_mode_bits *mode_bits;
    if (ctx.rd) {
    if (mode < ARRAY_SIZE(ctx.rd.mode_bits) &&
    ctx.rd.mode_bits[mode].valid)
    mode_bits = &ctx.rd.mode_bits[mode];
    else
    mode_bits = &ctx.rd.catchall;
    } else {
    mode_bits = &ctx.catchall;
    }
// Issue the reboot
    regmap_update_bits(ctx.map, mode_bits.offset, mode_bits.mask,
    mode_bits.value);
    mdelay(1000);
    pr_emerg("Unable to restart system\n");
    return NOTIFY_DONE;
    }
#[no_mangle]
unsafe extern "C" fn syscon_reboot_probe(pdev: *mut platform_device) -> c_int {
    static int syscon_reboot_probe(struct platform_device *pdev)
    {
    struct syscon_reboot_context *ctx;
    struct device *dev = &pdev.dev;
    u32 priority;
    int err;
    ctx = devm_kzalloc(&pdev.dev, sizeof(*ctx), GFP_KERNEL);
    if (!ctx)
    return -ENOMEM;
    ctx.map = syscon_regmap_lookup_by_phandle(dev.of_node, "regmap");
    if (IS_ERR(ctx.map)) {
    ctx.map = syscon_node_to_regmap(dev.parent.of_node);
    if (IS_ERR(ctx.map))
    return PTR_ERR(ctx.map);
    }
    if (of_property_read_u32(pdev.dev.of_node, "priority", &priority))
    priority = 192;
    ctx.rd = of_device_get_match_data(dev);
    if (!ctx.rd) {
    int mask_err, value_err;
    if (of_property_read_u32(pdev.dev.of_node, "offset",
    &ctx.catchall.offset) &&
    of_property_read_u32(pdev.dev.of_node, "reg",
    &ctx.catchall.offset))
    return -EINVAL;
    value_err = of_property_read_u32(pdev.dev.of_node, "value",
    &ctx.catchall.value);
    mask_err = of_property_read_u32(pdev.dev.of_node, "mask",
    &ctx.catchall.mask);
    if (value_err && mask_err) {
    dev_err(dev, "unable to read 'value' and 'mask'");
    return -EINVAL;
    }
    if (value_err) {
// support old binding
    ctx.catchall.value = ctx.catchall.mask;
    ctx.catchall.mask = 0xFFFFFFFF;
    } else if (mask_err) {
// support value without mask
    ctx.catchall.mask = 0xFFFFFFFF;
    }
    }
    ctx.restart_handler.notifier_call = syscon_restart_handle;
    ctx.restart_handler.priority = priority;
    err = register_restart_handler(&ctx.restart_handler);
    if (err)
    dev_err(dev, "can't register restart notifier (err=%d)\n", err);
    return err;
    }
    static const struct reboot_data gs101_reboot_data = {
    .mode_bits = {
    [REBOOT_WARM] = {
    .offset = 0x3a00, /* SYSTEM_CONFIGURATION */
    .mask = 0x00000002, /* SWRESET_SYSTEM */
    .value = 0x00000002,
    .valid = true,
    },
    [REBOOT_SOFT] = {
    .offset = 0x3a00, /* SYSTEM_CONFIGURATION */
    .mask = 0x00000002, /* SWRESET_SYSTEM */
    .value = 0x00000002,
    .valid = true,
    },
    },
    .catchall = {
    .offset = 0x3e9c, /* PAD_CTRL_PWR_HOLD */
    .mask = 0x00000100,
    .value = 0x00000000,
    },
    };
    static const struct of_device_id syscon_reboot_of_match[] = {
    { .compatible = "google,gs101-reboot", .data = &gs101_reboot_data  },
    { .compatible = "syscon-reboot" },
    {}
    };
    static struct platform_driver syscon_reboot_driver = {
    .probe = syscon_reboot_probe,
    .driver = {
    .name = "syscon-reboot",
    .of_match_table = syscon_reboot_of_match,
    },
    };
    builtin_platform_driver(syscon_reboot_driver);
