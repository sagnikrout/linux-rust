//! Automatically rewritten from C to Rust
//! Source: drivers/memory/da8xx-ddrctl.c
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
// TI da8xx DDR2/mDDR controller driver
//
// Copyright (C) 2016 BayLibre SAS
//
// Author:
// Bartosz Golaszewski <bgolaszewski@baylibre.com>
//

//
// REVISIT: Linux doesn't have a good framework for the kind of performance
// knobs this driver controls. We can't use device tree properties as it deals
// with hardware configuration rather than description. We also don't want to
// commit to maintaining some random sysfs attributes.
//
// For now we just hardcode the register values for the boards that need
// some changes (as is the case for the LCD controller on da850-lcdk - the
// first board we support here). When linux gets an appropriate framework,
// we'll easily convert the driver to it.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct da8xx_ddrctl_config_knob {
    pub name: *const c_char,
    pub reg: u32,
    pub mask: u32,
    pub shift: u32,
}

    static const struct da8xx_ddrctl_config_knob da8xx_ddrctl_knobs[] = {
    {
    .name = "da850-pbbpr",
    .reg = 0x20,
    .mask = 0xffffff00,
    .shift = 0,
    },
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct da8xx_ddrctl_setting {
    pub name: *const c_char,
    pub val: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct da8xx_ddrctl_board_settings {
    pub board: *const c_char,
    pub settings: *const da8xx_ddrctl_setting,
}

    static const struct da8xx_ddrctl_setting da850_lcdk_ddrctl_settings[] = {
    {
    .name = "da850-pbbpr",
    .val = 0x20,
    },
    { }
    };
    static const struct da8xx_ddrctl_board_settings da8xx_ddrctl_board_confs[] = {
    {
    .board = "ti,da850-lcdk",
    .settings = da850_lcdk_ddrctl_settings,
    },
    };
    static const struct da8xx_ddrctl_config_knob *
    da8xx_ddrctl_match_knob(const struct da8xx_ddrctl_setting *setting)
    {
    const struct da8xx_ddrctl_config_knob *knob;
    int i;
    for (i = 0; i < ARRAY_SIZE(da8xx_ddrctl_knobs); i++) {
    knob = &da8xx_ddrctl_knobs[i];
    if (strcmp(knob.name, setting.name) == 0)
    return knob;
    }
    return core::ptr::null_mut();
    }
    static const struct da8xx_ddrctl_setting *da8xx_ddrctl_get_board_settings(void)
    {
    const struct da8xx_ddrctl_board_settings *board_settings;
    int i;
    for (i = 0; i < ARRAY_SIZE(da8xx_ddrctl_board_confs); i++) {
    board_settings = &da8xx_ddrctl_board_confs[i];
    if (of_machine_is_compatible(board_settings.board))
    return board_settings.settings;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn da8xx_ddrctl_probe(pdev: *mut platform_device) -> c_int {
    static int da8xx_ddrctl_probe(struct platform_device *pdev)
    {
    const struct da8xx_ddrctl_config_knob *knob;
    const struct da8xx_ddrctl_setting *setting;
    struct resource *res;
    void __iomem *ddrctl;
    struct device *dev;
    u32 reg;
    dev = &pdev.dev;
    setting = da8xx_ddrctl_get_board_settings();
    if (!setting) {
    dev_err(dev, "no settings defined for this board\n");
    return -EINVAL;
    }
    ddrctl = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(ddrctl)) {
    dev_err(dev, "unable to map memory controller registers\n");
    return PTR_ERR(ddrctl);
    }
    for (; setting.name; setting++) {
    knob = da8xx_ddrctl_match_knob(setting);
    if (!knob) {
    dev_warn(dev,
    "no such config option: %s\n", setting.name);
    continue;
    }
    if (knob.reg + sizeof(u32) > resource_size(res)) {
    dev_warn(dev,
    "register offset of '%s' exceeds mapped memory size\n",
    knob.name);
    continue;
    }
    reg = readl(ddrctl + knob.reg);
    reg &= knob.mask;
    reg |= setting.val << knob.shift;
    dev_dbg(dev, "writing 0x%08x to %s\n", reg, setting.name);
    writel(reg, ddrctl + knob.reg);
    }
    return 0;
    }
    static const struct of_device_id da8xx_ddrctl_of_match[] = {
    { .compatible = "ti,da850-ddr-controller", },
    { },
    };
    static struct platform_driver da8xx_ddrctl_driver = {
    .probe = da8xx_ddrctl_probe,
    .driver = {
    .name = "da850-ddr-controller",
    .of_match_table = da8xx_ddrctl_of_match,
    },
    };
    module_platform_driver(da8xx_ddrctl_driver);
    MODULE_AUTHOR("Bartosz Golaszewski <bgolaszewski@baylibre.com>");
    MODULE_DESCRIPTION("TI da8xx DDR2/mDDR controller driver");
