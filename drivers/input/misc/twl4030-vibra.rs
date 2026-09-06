//! Automatically rewritten from C to Rust
//! Source: drivers/input/misc/twl4030-vibra.c
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
// twl4030-vibra.c - TWL4030 Vibrator driver
//
// Copyright (C) 2008-2010 Nokia Corporation
//
// Written by Henrik Saari <henrik.saari@nokia.com>
// Updates by Felipe Balbi <felipe.balbi@nokia.com>
// Input by Jari Vanhala <ext-jari.vanhala@nokia.com>
//

// MODULE ID2
pub const LEDEN: c_uint = 0x00;
// ForceFeedback
pub const EFFECT_DIR_180_DEG: c_uint = 0x8000 /* range is 0 - 0xFFFF */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vibra_info {
    pub dev: *mut device,
    pub input_dev: *mut input_dev,
    pub play_work: work_struct,
    pub enabled: bool,
    pub speed: c_int,
    pub direction: c_int,
    pub coexist: bool,
}

#[no_mangle]
unsafe extern "C" fn vibra_disable_leds() {
    static void vibra_disable_leds(void)
    {
    u8 reg;
// Disable LEDA & LEDB, cannot be used with vibra (PWM)
    twl_i2c_read_u8(TWL4030_MODULE_LED, &reg, LEDEN);
    reg &= ~0x03;
    twl_i2c_write_u8(TWL4030_MODULE_LED, LEDEN, reg);
    }
// Powers H-Bridge and enables audio clk
#[no_mangle]
unsafe extern "C" fn vibra_enable(info: *mut vibra_info) {
    static void vibra_enable(struct vibra_info *info)
    {
    u8 reg;
    twl4030_audio_enable_resource(TWL4030_AUDIO_RES_POWER);
// turn H-Bridge on
    twl_i2c_read_u8(TWL4030_MODULE_AUDIO_VOICE,
    &reg, TWL4030_REG_VIBRA_CTL);
    twl_i2c_write_u8(TWL4030_MODULE_AUDIO_VOICE,
    (reg | TWL4030_VIBRA_EN), TWL4030_REG_VIBRA_CTL);
    twl4030_audio_enable_resource(TWL4030_AUDIO_RES_APLL);
    info.enabled = true;
    }
#[no_mangle]
unsafe extern "C" fn vibra_disable(info: *mut vibra_info) {
    static void vibra_disable(struct vibra_info *info)
    {
    u8 reg;
// Power down H-Bridge
    twl_i2c_read_u8(TWL4030_MODULE_AUDIO_VOICE,
    &reg, TWL4030_REG_VIBRA_CTL);
    twl_i2c_write_u8(TWL4030_MODULE_AUDIO_VOICE,
    (reg & ~TWL4030_VIBRA_EN), TWL4030_REG_VIBRA_CTL);
    twl4030_audio_disable_resource(TWL4030_AUDIO_RES_APLL);
    twl4030_audio_disable_resource(TWL4030_AUDIO_RES_POWER);
    info.enabled = false;
    }
#[no_mangle]
unsafe extern "C" fn vibra_play_work(work: *mut work_struct) {
    static void vibra_play_work(struct work_struct *work)
    {
    struct vibra_info *info = container_of(work,
    struct vibra_info, play_work);
    int dir;
    int pwm;
    u8 reg;
    dir = info.direction;
    pwm = info.speed;
    twl_i2c_read_u8(TWL4030_MODULE_AUDIO_VOICE,
    &reg, TWL4030_REG_VIBRA_CTL);
    if (pwm && (!info.coexist || !(reg & TWL4030_VIBRA_SEL))) {
    if (!info.enabled)
    vibra_enable(info);
// set vibra rotation direction
    twl_i2c_read_u8(TWL4030_MODULE_AUDIO_VOICE,
    &reg, TWL4030_REG_VIBRA_CTL);
    reg = (dir) ? (reg | TWL4030_VIBRA_DIR) :
    (reg & ~TWL4030_VIBRA_DIR);
    twl_i2c_write_u8(TWL4030_MODULE_AUDIO_VOICE,
    reg, TWL4030_REG_VIBRA_CTL);
// set PWM, 1 = max, 255 = min
    twl_i2c_write_u8(TWL4030_MODULE_AUDIO_VOICE,
    256 - pwm, TWL4030_REG_VIBRA_SET);
    } else {
    if (info.enabled)
    vibra_disable(info);
    }
    }
// Input/ForceFeedback
    static int vibra_play(struct input_dev *input, void *data,
    struct ff_effect *effect)
    {
    struct vibra_info *info = input_get_drvdata(input);
    info.speed = effect.u.rumble.strong_magnitude >> 8;
    if (!info.speed)
    info.speed = effect.u.rumble.weak_magnitude >> 9;
    info.direction = effect.direction < EFFECT_DIR_180_DEG ? 0 : 1;
    schedule_work(&info.play_work);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn twl4030_vibra_close(input: *mut input_dev) {
    static void twl4030_vibra_close(struct input_dev *input)
    {
    struct vibra_info *info = input_get_drvdata(input);
    cancel_work_sync(&info.play_work);
    if (info.enabled)
    vibra_disable(info);
    }
// Module
#[no_mangle]
unsafe extern "C" fn twl4030_vibra_suspend(dev: *mut device) -> c_int {
    static int twl4030_vibra_suspend(struct device *dev)
    {
    struct platform_device *pdev = to_platform_device(dev);
    struct vibra_info *info = platform_get_drvdata(pdev);
    if (info.enabled)
    vibra_disable(info);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn twl4030_vibra_resume(dev: *mut device) -> c_int {
    static int twl4030_vibra_resume(struct device *dev)
    {
    vibra_disable_leds();
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(twl4030_vibra_pm_ops,
    twl4030_vibra_suspend, twl4030_vibra_resume);
#[no_mangle]
unsafe extern "C" fn twl4030_vibra_check_coexist(parent: *mut device_node) -> bool {
    static bool twl4030_vibra_check_coexist(struct device_node *parent)
    {
    struct device_node *node __free(device_node) =
    of_get_child_by_name(parent, "codec");
    return node != core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn twl4030_vibra_probe(pdev: *mut platform_device) -> c_int {
    static int twl4030_vibra_probe(struct platform_device *pdev)
    {
    struct device_node *twl4030_core_node = pdev.dev.parent.of_node;
    struct vibra_info *info;
    int ret;
    if (!twl4030_core_node) {
    dev_dbg(&pdev.dev, "twl4030 OF node is missing\n");
    return -EINVAL;
    }
    info = devm_kzalloc(&pdev.dev, sizeof(*info), GFP_KERNEL);
    if (!info)
    return -ENOMEM;
    info.dev = &pdev.dev;
    info.coexist = twl4030_vibra_check_coexist(twl4030_core_node);
    INIT_WORK(&info.play_work, vibra_play_work);
    info.input_dev = devm_input_allocate_device(&pdev.dev);
    if (info.input_dev == core::ptr::null_mut()) {
    dev_err(&pdev.dev, "couldn't allocate input device\n");
    return -ENOMEM;
    }
    input_set_drvdata(info.input_dev, info);
    info.input_dev.name = "twl4030:vibrator";
    info.input_dev.id.version = 1;
    info.input_dev.close = twl4030_vibra_close;
    __set_bit(FF_RUMBLE, info.input_dev.ffbit);
    ret = input_ff_create_memless(info.input_dev, core::ptr::null_mut(), vibra_play);
    if (ret < 0) {
    dev_dbg(&pdev.dev, "couldn't register vibrator to FF\n");
    return ret;
    }
    ret = input_register_device(info.input_dev);
    if (ret < 0) {
    dev_dbg(&pdev.dev, "couldn't register input device\n");
    goto err_iff;
    }
    vibra_disable_leds();
    platform_set_drvdata(pdev, info);
    return 0;
    err_iff:
    input_ff_destroy(info.input_dev);
    return ret;
    }
    static struct platform_driver twl4030_vibra_driver = {
    .probe		= twl4030_vibra_probe,
    .driver		= {
    .name	= "twl4030-vibra",
    .pm	= pm_sleep_ptr(&twl4030_vibra_pm_ops),
    },
    };
    module_platform_driver(twl4030_vibra_driver);
    MODULE_ALIAS("platform:twl4030-vibra");
    MODULE_DESCRIPTION("TWL4030 Vibra driver");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Nokia Corporation");
