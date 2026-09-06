//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/vexpress-sysreg.c
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
// Copyright (C) 2012 ARM Limited
//

pub const SYS_ID: c_uint = 0x000;
pub const SYS_SW: c_uint = 0x004;
pub const SYS_LED: c_uint = 0x008;
pub const SYS_100HZ: c_uint = 0x024;
pub const SYS_FLAGSSET: c_uint = 0x030;
pub const SYS_FLAGSCLR: c_uint = 0x034;
pub const SYS_NVFLAGS: c_uint = 0x038;
pub const SYS_NVFLAGSSET: c_uint = 0x038;
pub const SYS_NVFLAGSCLR: c_uint = 0x03c;
pub const SYS_MCI: c_uint = 0x048;
pub const SYS_FLASH: c_uint = 0x04c;
pub const SYS_CFGSW: c_uint = 0x058;
pub const SYS_24MHZ: c_uint = 0x05c;
pub const SYS_MISC: c_uint = 0x060;
pub const SYS_DMA: c_uint = 0x064;
pub const SYS_PROCID0: c_uint = 0x084;
pub const SYS_PROCID1: c_uint = 0x088;
pub const SYS_CFGDATA: c_uint = 0x0a0;
pub const SYS_CFGCTRL: c_uint = 0x0a4;
pub const SYS_CFGSTAT: c_uint = 0x0a8;
// The sysreg block is just a random collection of various functions...
    static const struct property_entry vexpress_sysreg_sys_led_props[] = {
    PROPERTY_ENTRY_STRING("label", "sys_led"),
    PROPERTY_ENTRY_U32("ngpios", 8),
    { }
    };
    static const struct software_node vexpress_sysreg_sys_led_swnode = {
    .properties = vexpress_sysreg_sys_led_props,
    };
    static const struct property_entry vexpress_sysreg_sys_mci_props[] = {
    PROPERTY_ENTRY_STRING("label", "sys_mci"),
    PROPERTY_ENTRY_U32("ngpios", 2),
    { }
    };
    static const struct software_node vexpress_sysreg_sys_mci_swnode = {
    .properties = vexpress_sysreg_sys_mci_props,
    };
    static const struct property_entry vexpress_sysreg_sys_flash_props[] = {
    PROPERTY_ENTRY_STRING("label", "sys_flash"),
    PROPERTY_ENTRY_U32("ngpios", 1),
    { }
    };
    static const struct software_node vexpress_sysreg_sys_flash_swnode = {
    .properties = vexpress_sysreg_sys_flash_props,
    };
    static struct mfd_cell vexpress_sysreg_cells[] = {
    {
    .name = "basic-mmio-gpio",
    .of_compatible = "arm,vexpress-sysreg,sys_led",
    .num_resources = 1,
    .resources = &DEFINE_RES_MEM_NAMED(SYS_LED, 0x4, "dat"),
    .swnode = &vexpress_sysreg_sys_led_swnode,
    }, {
    .name = "basic-mmio-gpio",
    .of_compatible = "arm,vexpress-sysreg,sys_mci",
    .num_resources = 1,
    .resources = &DEFINE_RES_MEM_NAMED(SYS_MCI, 0x4, "dat"),
    .swnode = &vexpress_sysreg_sys_mci_swnode,
    }, {
    .name = "basic-mmio-gpio",
    .of_compatible = "arm,vexpress-sysreg,sys_flash",
    .num_resources = 1,
    .resources = &DEFINE_RES_MEM_NAMED(SYS_FLASH, 0x4, "dat"),
    .swnode = &vexpress_sysreg_sys_flash_swnode,
    }, {
    .name = "vexpress-syscfg",
    .num_resources = 1,
    .resources = &DEFINE_RES_MEM(SYS_MISC, 0x4c),
    }
    };
#[no_mangle]
unsafe extern "C" fn vexpress_sysreg_probe(pdev: *mut platform_device) -> c_int {
    static int vexpress_sysreg_probe(struct platform_device *pdev)
    {
    struct gpio_generic_chip *mmc_gpio_chip;
    struct gpio_generic_chip_config config;
    struct resource *mem;
    void __iomem *base;
    int ret;
    mem = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!mem)
    return -EINVAL;
    base = devm_ioremap(&pdev.dev, mem.start, resource_size(mem));
    if (!base)
    return -ENOMEM;
//
// Duplicated SYS_MCI pseudo-GPIO controller for compatibility with
// older trees using sysreg node for MMC control lines.
//
    mmc_gpio_chip = devm_kzalloc(&pdev.dev, sizeof(*mmc_gpio_chip),
    GFP_KERNEL);
    if (!mmc_gpio_chip)
    return -ENOMEM;
    config = (struct gpio_generic_chip_config) {
    .dev = &pdev.dev,
    .sz = 4,
    .dat = base + SYS_MCI,
    };
    ret = gpio_generic_chip_init(mmc_gpio_chip, &config);
    if (ret)
    return ret;
    mmc_gpio_chip.gc.ngpio = 2;
    ret = devm_gpiochip_add_data(&pdev.dev, &mmc_gpio_chip.gc, core::ptr::null_mut());
    if (ret)
    return ret;
    return devm_mfd_add_devices(&pdev.dev, PLATFORM_DEVID_AUTO,
    vexpress_sysreg_cells,
    ARRAY_SIZE(vexpress_sysreg_cells), mem, 0, core::ptr::null_mut());
    }
    static const struct of_device_id vexpress_sysreg_match[] = {
    { .compatible = "arm,vexpress-sysreg", },
    {},
    };
    MODULE_DEVICE_TABLE(of, vexpress_sysreg_match);
    static struct platform_driver vexpress_sysreg_driver = {
    .driver = {
    .name = "vexpress-sysreg",
    .of_match_table = vexpress_sysreg_match,
    },
    .probe = vexpress_sysreg_probe,
    };
    module_platform_driver(vexpress_sysreg_driver);
    MODULE_DESCRIPTION("Versatile Express system registers driver");
    MODULE_LICENSE("GPL v2");
