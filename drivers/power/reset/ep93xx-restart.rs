//! Automatically rewritten from C to Rust
//! Source: drivers/power/reset/ep93xx-restart.c
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
//
// Cirrus EP93xx SoC reset driver
//
// Copyright (C) 2021 Nikita Shubin <nikita.shubin@maquefel.me>
//

pub const EP93XX_SYSCON_DEVCFG: c_uint = 0x80;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ep93xx_restart {
    pub aux_dev: *mut ep93xx_regmap_adev,
    pub restart_handler: notifier_block,
}

    static int ep93xx_restart_handle(struct notifier_block *this,
    unsigned long mode, void *cmd)
    {
    struct ep93xx_restart *priv =
    container_of(this, struct ep93xx_restart, restart_handler);
    struct ep93xx_regmap_adev *aux = priv.aux_dev;
// Issue the reboot
    aux.update_bits(aux.map, aux.lock, EP93XX_SYSCON_DEVCFG,
    EP93XX_SYSCON_DEVCFG_SWRST, EP93XX_SYSCON_DEVCFG_SWRST);
    aux.update_bits(aux.map, aux.lock, EP93XX_SYSCON_DEVCFG,
    EP93XX_SYSCON_DEVCFG_SWRST, 0);
    return NOTIFY_DONE;
    }
    static int ep93xx_reboot_probe(struct auxiliary_device *adev,
    const struct auxiliary_device_id *id)
    {
    struct ep93xx_regmap_adev *rdev = to_ep93xx_regmap_adev(adev);
    struct device *dev = &adev.dev;
    struct ep93xx_restart *priv;
    int err;
    if (!rdev.update_bits)
    return -ENODEV;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.aux_dev = rdev;
    priv.restart_handler.notifier_call = ep93xx_restart_handle;
    priv.restart_handler.priority = 128;
    err = register_restart_handler(&priv.restart_handler);
    if (err)
    return dev_err_probe(dev, err, "can't register restart notifier\n");
    return 0;
    }
    static const struct auxiliary_device_id ep93xx_reboot_ids[] = {
    {
    .name = "soc_ep93xx.reset-ep93xx",
    },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(auxiliary, ep93xx_reboot_ids);
    static struct auxiliary_driver ep93xx_reboot_driver = {
    .probe		= ep93xx_reboot_probe,
    .id_table	= ep93xx_reboot_ids,
    };
    module_auxiliary_driver(ep93xx_reboot_driver);
