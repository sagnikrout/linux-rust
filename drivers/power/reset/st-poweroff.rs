//! Automatically rewritten from C to Rust
//! Source: drivers/power/reset/st-poweroff.c
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
// Copyright (C) 2014 STMicroelectronics
//
// Power off Restart driver, used in STMicroelectronics devices.
//
// Author: Christophe Kerello <christophe.kerello@st.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reset_syscfg {
    pub regmap: *mut regmap,
// syscfg used for reset
    pub offset_rst: c_uint,
    pub mask_rst: c_uint,
// syscfg used for unmask the reset
    pub offset_rst_msk: c_uint,
    pub mask_rst_msk: c_uint,
}

// STiH407
pub const STIH407_SYSCFG_4000: c_uint = 0x0;
pub const STIH407_SYSCFG_4008: c_uint = 0x20;
    static struct reset_syscfg stih407_reset = {
    .offset_rst = STIH407_SYSCFG_4000,
    .mask_rst = BIT(0),
    .offset_rst_msk = STIH407_SYSCFG_4008,
    .mask_rst_msk = BIT(0)
    };
    static struct reset_syscfg *st_restart_syscfg;
    static int st_restart(struct notifier_block *this, unsigned long mode,
    void *cmd)
    {
// reset syscfg updated
    regmap_update_bits(st_restart_syscfg.regmap,
    st_restart_syscfg.offset_rst,
    st_restart_syscfg.mask_rst,
    0);
// unmask the reset
    regmap_update_bits(st_restart_syscfg.regmap,
    st_restart_syscfg.offset_rst_msk,
    st_restart_syscfg.mask_rst_msk,
    0);
    return NOTIFY_DONE;
    }
    static struct notifier_block st_restart_nb = {
    .notifier_call = st_restart,
    .priority = 192,
    };
    static const struct of_device_id st_reset_of_match[] = {
    {
    .compatible = "st,stih407-restart",
    .data = (void *)&stih407_reset,
    },
    {}
    };
#[no_mangle]
unsafe extern "C" fn st_reset_probe(pdev: *mut platform_device) -> c_int {
    static int st_reset_probe(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    struct device *dev = &pdev.dev;
    st_restart_syscfg = (struct reset_syscfg *)of_device_get_match_data(dev);
    if (!st_restart_syscfg)
    return -ENODEV;
    st_restart_syscfg.regmap =
    syscon_regmap_lookup_by_phandle(np, "st,syscfg");
    if (IS_ERR(st_restart_syscfg.regmap)) {
    dev_err(dev, "No syscfg phandle specified\n");
    return PTR_ERR(st_restart_syscfg.regmap);
    }
    return register_restart_handler(&st_restart_nb);
    }
    static struct platform_driver st_reset_driver = {
    .probe = st_reset_probe,
    .driver = {
    .name = "st_reset",
    .of_match_table = st_reset_of_match,
    },
    };
    builtin_platform_driver(st_reset_driver);
    MODULE_AUTHOR("Christophe Kerello <christophe.kerello@st.com>");
    MODULE_DESCRIPTION("STMicroelectronics Power off Restart driver");
    MODULE_LICENSE("GPL v2");
