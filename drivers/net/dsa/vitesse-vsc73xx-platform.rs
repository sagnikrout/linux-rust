//! Automatically rewritten from C to Rust
//! Source: drivers/net/dsa/vitesse-vsc73xx-platform.c
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
// DSA driver for:
// Vitesse VSC7385 SparX-G5 5+1-port Integrated Gigabit Ethernet Switch
// Vitesse VSC7388 SparX-G8 8-port Integrated Gigabit Ethernet Switch
// Vitesse VSC7395 SparX-G5e 5+1-port Integrated Gigabit Ethernet Switch
// Vitesse VSC7398 SparX-G8e 8-port Integrated Gigabit Ethernet Switch
//
// This driver takes control of the switch chip connected over CPU-attached
// address bus and configures it to route packages around when connected to
// a CPU port.
//
// Copyright (C) 2019 Pawel Dembicki <paweldembicki@gmail.com>
// Based on vitesse-vsc-spi.c by:
// Copyright (C) 2018 Linus Wallej <linus.walleij@linaro.org>
// Includes portions of code from the firmware uploader by:
// Copyright (C) 2009 Gabor Juhos <juhosg@openwrt.org>
//

pub const VSC73XX_CMD_PLATFORM_BLOCK_SHIFT: c_int = 14;
pub const VSC73XX_CMD_PLATFORM_BLOCK_MASK: c_uint = 0x7;
pub const VSC73XX_CMD_PLATFORM_SUBBLOCK_SHIFT: c_int = 10;
pub const VSC73XX_CMD_PLATFORM_SUBBLOCK_MASK: c_uint = 0xf;
pub const VSC73XX_CMD_PLATFORM_REGISTER_SHIFT: c_int = 2;
//
// struct vsc73xx_platform - VSC73xx Platform state container
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vsc73xx_platform {
    pub pdev: *mut platform_device,
    pub base_addr: *mut void __iomem,
    pub vsc: vsc73xx,
}

    static const struct vsc73xx_ops vsc73xx_platform_ops;
#[no_mangle]
unsafe extern "C" fn vsc73xx_make_addr(block: u8, subblock: u8, reg: u8) -> u32 {
    static u32 vsc73xx_make_addr(u8 block, u8 subblock, u8 reg)
    {
    u32 ret;
    ret = (block & VSC73XX_CMD_PLATFORM_BLOCK_MASK)
    << VSC73XX_CMD_PLATFORM_BLOCK_SHIFT;
    ret |= (subblock & VSC73XX_CMD_PLATFORM_SUBBLOCK_MASK)
    << VSC73XX_CMD_PLATFORM_SUBBLOCK_SHIFT;
    ret |= reg << VSC73XX_CMD_PLATFORM_REGISTER_SHIFT;
    return ret;
    }
    static int vsc73xx_platform_read(struct vsc73xx *vsc, u8 block, u8 subblock,
    u8 reg, u32 *val)
    {
    struct vsc73xx_platform *vsc_platform = vsc.priv;
    u32 offset;
    if (!vsc73xx_is_addr_valid(block, subblock))
    return -EINVAL;
    offset = vsc73xx_make_addr(block, subblock, reg);
// By default vsc73xx running in big-endian mode.
// (See "Register Addressing" section 5.5.3 in the VSC7385 manual.)
//
// val = ioread32be(vsc_platform->base_addr + offset);
    return 0;
    }
    static int vsc73xx_platform_write(struct vsc73xx *vsc, u8 block, u8 subblock,
    u8 reg, u32 val)
    {
    struct vsc73xx_platform *vsc_platform = vsc.priv;
    u32 offset;
    if (!vsc73xx_is_addr_valid(block, subblock))
    return -EINVAL;
    offset = vsc73xx_make_addr(block, subblock, reg);
    iowrite32be(val, vsc_platform.base_addr + offset);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vsc73xx_platform_probe(pdev: *mut platform_device) -> c_int {
    static int vsc73xx_platform_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct vsc73xx_platform *vsc_platform;
    int ret;
    vsc_platform = devm_kzalloc(dev, sizeof(*vsc_platform), GFP_KERNEL);
    if (!vsc_platform)
    return -ENOMEM;
    platform_set_drvdata(pdev, vsc_platform);
    vsc_platform.pdev = pdev;
    vsc_platform.vsc.dev = dev;
    vsc_platform.vsc.priv = vsc_platform;
    vsc_platform.vsc.ops = &vsc73xx_platform_ops;
// obtain I/O memory space
    vsc_platform.base_addr = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(vsc_platform.base_addr)) {
    dev_err(&pdev.dev, "cannot request I/O memory space\n");
    ret = -ENXIO;
    return ret;
    }
    return vsc73xx_probe(&vsc_platform.vsc);
    }
#[no_mangle]
unsafe extern "C" fn vsc73xx_platform_remove(pdev: *mut platform_device) {
    static void vsc73xx_platform_remove(struct platform_device *pdev)
    {
    struct vsc73xx_platform *vsc_platform = platform_get_drvdata(pdev);
    if (!vsc_platform)
    return;
    vsc73xx_remove(&vsc_platform.vsc);
    }
#[no_mangle]
unsafe extern "C" fn vsc73xx_platform_shutdown(pdev: *mut platform_device) {
    static void vsc73xx_platform_shutdown(struct platform_device *pdev)
    {
    struct vsc73xx_platform *vsc_platform = platform_get_drvdata(pdev);
    if (!vsc_platform)
    return;
    vsc73xx_shutdown(&vsc_platform.vsc);
    platform_set_drvdata(pdev, core::ptr::null_mut());
    }
    static const struct vsc73xx_ops vsc73xx_platform_ops = {
    .read = vsc73xx_platform_read,
    .write = vsc73xx_platform_write,
    };
    static const struct of_device_id vsc73xx_of_match[] = {
    {
    .compatible = "vitesse,vsc7385",
    },
    {
    .compatible = "vitesse,vsc7388",
    },
    {
    .compatible = "vitesse,vsc7395",
    },
    {
    .compatible = "vitesse,vsc7398",
    },
    { },
    };
    MODULE_DEVICE_TABLE(of, vsc73xx_of_match);
    static struct platform_driver vsc73xx_platform_driver = {
    .probe = vsc73xx_platform_probe,
    .remove = vsc73xx_platform_remove,
    .shutdown = vsc73xx_platform_shutdown,
    .driver = {
    .name = "vsc73xx-platform",
    .of_match_table = vsc73xx_of_match,
    },
    };
    module_platform_driver(vsc73xx_platform_driver);
    MODULE_AUTHOR("Pawel Dembicki <paweldembicki@gmail.com>");
    MODULE_DESCRIPTION("Vitesse VSC7385/7388/7395/7398 Platform driver");
    MODULE_LICENSE("GPL v2");
