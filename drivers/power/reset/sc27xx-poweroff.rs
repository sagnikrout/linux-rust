//! Automatically rewritten from C to Rust
//! Source: drivers/power/reset/sc27xx-poweroff.c
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
// Copyright (C) 2018 Spreadtrum Communications Inc.
// Copyright (C) 2018 Linaro Ltd.
//

pub const SC27XX_PWR_PD_HW: c_uint = 0xc2c;

pub const SC27XX_SLP_CTRL: c_uint = 0xdf0;

    static struct regmap *regmap;
//
// On Spreadtrum platform, we need power off system through external SC27xx
// series PMICs, and it is one similar SPI bus mapped by regmap to access PMIC,
// which is not fast io access.
//
// So before stopping other cores, we need release other cores' resource by
// taking cpus down to avoid racing regmap or spi mutex lock when poweroff
// system through PMIC.
//
#[no_mangle]
unsafe extern "C" fn sc27xx_poweroff_shutdown(data: *mut c_void) {
    static void sc27xx_poweroff_shutdown(void *data)
    {

    int cpu;
    for_each_online_cpu(cpu) {
    if (cpu != smp_processor_id())
    remove_cpu(cpu);
    }

    }
    static const struct syscore_ops poweroff_syscore_ops = {
    .shutdown = sc27xx_poweroff_shutdown,
    };
    static struct syscore poweroff_syscore = {
    .ops = &poweroff_syscore_ops,
    };
#[no_mangle]
unsafe extern "C" fn sc27xx_poweroff_do_poweroff() {
    static void sc27xx_poweroff_do_poweroff(void)
    {
// Disable the external subsys connection's power firstly
    regmap_write(regmap, SC27XX_SLP_CTRL, SC27XX_LDO_XTL_EN);
    regmap_write(regmap, SC27XX_PWR_PD_HW, SC27XX_PWR_OFF_EN);
    }
#[no_mangle]
unsafe extern "C" fn sc27xx_poweroff_probe(pdev: *mut platform_device) -> c_int {
    static int sc27xx_poweroff_probe(struct platform_device *pdev)
    {
    if (regmap)
    return -EINVAL;
    regmap = dev_get_regmap(pdev.dev.parent, core::ptr::null_mut());
    if (!regmap)
    return -ENODEV;
    pm_power_off = sc27xx_poweroff_do_poweroff;
    register_syscore(&poweroff_syscore);
    return 0;
    }
    static const struct platform_device_id sc27xx_poweroff_id_table[] = {
    { "sc2731-poweroff" },
    { }
    };
    MODULE_DEVICE_TABLE(platform, sc27xx_poweroff_id_table);
    static struct platform_driver sc27xx_poweroff_driver = {
    .probe = sc27xx_poweroff_probe,
    .driver = {
    .name = "sc27xx-poweroff",
    },
    .id_table = sc27xx_poweroff_id_table,
    };
    module_platform_driver(sc27xx_poweroff_driver);
    MODULE_DESCRIPTION("Power off driver for SC27XX PMIC Device");
    MODULE_AUTHOR("Baolin Wang <baolin.wang@unisoc.com>");
    MODULE_LICENSE("GPL v2");
