//! Automatically rewritten from C to Rust
//! Source: drivers/video/backlight/apple_dwi_bl.c
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


// SPDX-License-Identifier: GPL-2.0 OR MIT
//
// Driver for backlight controllers attached via Apple DWI 2-wire interface
//
// Copyright (c) 2024 Nick Chan <towinchenmi@gmail.com>
//

pub const DWI_BL_CTL: c_uint = 0x0;

// Only used on Apple A9 and later

pub const DWI_BL_CMD: c_uint = 0x4;

pub const DWI_BL_CMD_TYPE_SET_BRIGHTNESS: c_uint = 0xa;

    DWI_BL_CTL_SEND2 | \
    DWI_BL_CTL_SEND3 | \
    DWI_BL_CTL_LE_DATA | \
    DWI_BL_CTL_SEND4)
pub const DWI_BL_MAX_BRIGHTNESS: c_int = 2047;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apple_dwi_bl {
    pub base: *mut void __iomem,
}

#[no_mangle]
unsafe extern "C" fn dwi_bl_update_status(bl: *mut backlight_device) -> c_int {
    static int dwi_bl_update_status(struct backlight_device *bl)
    {
    struct apple_dwi_bl *dwi_bl = bl_get_data(bl);
    let mut brightness: c_int = backlight_get_brightness(bl);
    let mut cmd: u32 = 0;
    cmd |= FIELD_PREP(DWI_BL_CMD_DATA, brightness);
    cmd |= FIELD_PREP(DWI_BL_CMD_TYPE, DWI_BL_CMD_TYPE_SET_BRIGHTNESS);
    writel(cmd, dwi_bl.base + DWI_BL_CMD);
    writel(DWI_BL_CTL_SEND, dwi_bl.base + DWI_BL_CTL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dwi_bl_get_brightness(bl: *mut backlight_device) -> c_int {
    static int dwi_bl_get_brightness(struct backlight_device *bl)
    {
    struct apple_dwi_bl *dwi_bl = bl_get_data(bl);
    let mut cmd: u32 = readl(dwi_bl.base + DWI_BL_CMD);
    return FIELD_GET(DWI_BL_CMD_DATA, cmd);
    }
    static const struct backlight_ops dwi_bl_ops = {
    .options = BL_CORE_SUSPENDRESUME,
    .get_brightness = dwi_bl_get_brightness,
    .update_status	= dwi_bl_update_status
    };
#[no_mangle]
unsafe extern "C" fn dwi_bl_probe(dev: *mut platform_device) -> c_int {
    static int dwi_bl_probe(struct platform_device *dev)
    {
    struct apple_dwi_bl *dwi_bl;
    struct backlight_device *bl;
    struct backlight_properties props;
    struct resource *res;
    dwi_bl = devm_kzalloc(&dev.dev, sizeof(*dwi_bl), GFP_KERNEL);
    if (!dwi_bl)
    return -ENOMEM;
    dwi_bl.base = devm_platform_get_and_ioremap_resource(dev, 0, &res);
    if (IS_ERR(dwi_bl.base))
    return PTR_ERR(dwi_bl.base);
    memset(&props, 0, sizeof(struct backlight_properties));
    props.type = BACKLIGHT_PLATFORM;
    props.max_brightness = DWI_BL_MAX_BRIGHTNESS;
    props.scale = BACKLIGHT_SCALE_LINEAR;
    bl = devm_backlight_device_register(&dev.dev, dev.name, &dev.dev,
    dwi_bl, &dwi_bl_ops, &props);
    if (IS_ERR(bl))
    return PTR_ERR(bl);
    platform_set_drvdata(dev, dwi_bl);
    bl.props.brightness = dwi_bl_get_brightness(bl);
    return 0;
    }
    static const struct of_device_id dwi_bl_of_match[] = {
    { .compatible = "apple,dwi-bl" },
    {},
    };
    MODULE_DEVICE_TABLE(of, dwi_bl_of_match);
    static struct platform_driver dwi_bl_driver = {
    .driver		= {
    .name	= "apple-dwi-bl",
    .of_match_table = dwi_bl_of_match
    },
    .probe		= dwi_bl_probe,
    };
    module_platform_driver(dwi_bl_driver);
    MODULE_DESCRIPTION("Apple DWI Backlight Driver");
    MODULE_AUTHOR("Nick Chan <towinchenmi@gmail.com>");
    MODULE_LICENSE("Dual MIT/GPL");
