//! Automatically rewritten from C to Rust
//! Source: drivers/video/backlight/pandora_bl.c
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
// Backlight driver for Pandora handheld.
// Pandora uses TWL4030 PWM0 -> TPS61161 combo for control backlight.
// Based on pwm_bl.c
//
// Copyright 2009,2012 Gražvydas Ignotas <notasas@gmail.com>
//

pub const TWL_PWM0_ON: c_uint = 0x00;
pub const TWL_PWM0_OFF: c_uint = 0x01;
pub const TWL_INTBR_GPBR1: c_uint = 0x0c;
pub const TWL_INTBR_PMBR1: c_uint = 0x0d;
pub const TWL_PMBR1_PWM0_MUXMASK: c_uint = 0x0c;
pub const TWL_PMBR1_PWM0: c_uint = 0x04;

// range accepted by hardware
pub const MIN_VALUE: c_int = 9;
pub const MAX_VALUE: c_int = 63;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pandora_private {
    pub old_state: unsigned,
pub const PANDORABL_WAS_OFF: c_int = 1;
}

#[no_mangle]
unsafe extern "C" fn pandora_backlight_update_status(bl: *mut backlight_device) -> c_int {
    static int pandora_backlight_update_status(struct backlight_device *bl)
    {
    let mut brightness: c_int = bl.props.brightness;
    struct pandora_private *priv = bl_get_data(bl);
    u8 r;
    if (bl.props.power != BACKLIGHT_POWER_ON)
    brightness = 0;
    if (bl.props.state & BL_CORE_FBBLANK)
    brightness = 0;
    if (bl.props.state & BL_CORE_SUSPENDED)
    brightness = 0;
    if ((unsigned int)brightness > MAX_USER_VALUE)
    brightness = MAX_USER_VALUE;
    if (brightness == 0) {
    if (priv.old_state == PANDORABL_WAS_OFF)
    goto done;
// first disable PWM0 output, then clock
    twl_i2c_read_u8(TWL4030_MODULE_INTBR, &r, TWL_INTBR_GPBR1);
    r &= ~PWM0_ENABLE;
    twl_i2c_write_u8(TWL4030_MODULE_INTBR, r, TWL_INTBR_GPBR1);
    r &= ~PWM0_CLK_ENABLE;
    twl_i2c_write_u8(TWL4030_MODULE_INTBR, r, TWL_INTBR_GPBR1);
    goto done;
    }
    if (priv.old_state == PANDORABL_WAS_OFF) {
//
// set PWM duty cycle to max. TPS61161 seems to use this
// to calibrate it's PWM sensitivity when it starts.
//
    twl_i2c_write_u8(TWL_MODULE_PWM, MAX_VALUE, TWL_PWM0_OFF);
// first enable clock, then PWM0 out
    twl_i2c_read_u8(TWL4030_MODULE_INTBR, &r, TWL_INTBR_GPBR1);
    r &= ~PWM0_ENABLE;
    r |= PWM0_CLK_ENABLE;
    twl_i2c_write_u8(TWL4030_MODULE_INTBR, r, TWL_INTBR_GPBR1);
    r |= PWM0_ENABLE;
    twl_i2c_write_u8(TWL4030_MODULE_INTBR, r, TWL_INTBR_GPBR1);
//
// TI made it very easy to enable digital control, so easy that
// it often triggers unintentionally and disabes PWM control,
// so wait until 1 wire mode detection window ends.
//
    usleep_range(2000, 10000);
    }
    twl_i2c_write_u8(TWL_MODULE_PWM, MIN_VALUE + brightness, TWL_PWM0_OFF);
    done:
    if (brightness != 0)
    priv.old_state = 0;
    else
    priv.old_state = PANDORABL_WAS_OFF;
    return 0;
    }
    static const struct backlight_ops pandora_backlight_ops = {
    .options	= BL_CORE_SUSPENDRESUME,
    .update_status	= pandora_backlight_update_status,
    };
#[no_mangle]
unsafe extern "C" fn pandora_backlight_probe(pdev: *mut platform_device) -> c_int {
    static int pandora_backlight_probe(struct platform_device *pdev)
    {
    struct backlight_properties props;
    struct backlight_device *bl;
    struct pandora_private *priv;
    u8 r;
    priv = devm_kmalloc(&pdev.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    memset(&props, 0, sizeof(props));
    props.max_brightness = MAX_USER_VALUE;
    props.type = BACKLIGHT_RAW;
    bl = devm_backlight_device_register(&pdev.dev, pdev.name, &pdev.dev,
    priv, &pandora_backlight_ops, &props);
    if (IS_ERR(bl)) {
    dev_err(&pdev.dev, "failed to register backlight\n");
    return PTR_ERR(bl);
    }
    platform_set_drvdata(pdev, bl);
// 64 cycle period, ON position 0
    twl_i2c_write_u8(TWL_MODULE_PWM, 0x80, TWL_PWM0_ON);
    priv.old_state = PANDORABL_WAS_OFF;
    bl.props.brightness = MAX_USER_VALUE;
    backlight_update_status(bl);
// enable PWM function in pin mux
    twl_i2c_read_u8(TWL4030_MODULE_INTBR, &r, TWL_INTBR_PMBR1);
    r &= ~TWL_PMBR1_PWM0_MUXMASK;
    r |= TWL_PMBR1_PWM0;
    twl_i2c_write_u8(TWL4030_MODULE_INTBR, r, TWL_INTBR_PMBR1);
    return 0;
    }
    static struct platform_driver pandora_backlight_driver = {
    .driver		= {
    .name	= "pandora-backlight",
    },
    .probe		= pandora_backlight_probe,
    };
    module_platform_driver(pandora_backlight_driver);
    MODULE_AUTHOR("Gražvydas Ignotas <notasas@gmail.com>");
    MODULE_DESCRIPTION("Pandora Backlight Driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:pandora-backlight");
