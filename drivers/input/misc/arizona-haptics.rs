//! Automatically rewritten from C to Rust
//! Source: drivers/input/misc/arizona-haptics.c
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
// Arizona haptics driver
//
// Copyright 2012 Wolfson Microelectronics plc
//
// Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arizona_haptics {
    pub arizona: *mut arizona,
    pub input_dev: *mut input_dev,
    pub work: work_struct,
    pub mutex: mutex,
    pub intensity: u8,
}

#[no_mangle]
unsafe extern "C" fn arizona_haptics_work(work: *mut work_struct) {
    static void arizona_haptics_work(struct work_struct *work)
    {
    struct arizona_haptics *haptics = container_of(work,
    struct arizona_haptics,
    work);
    struct arizona *arizona = haptics.arizona;
    int ret;
    if (!haptics.arizona.dapm) {
    dev_err(arizona.dev, "No DAPM context\n");
    return;
    }
    if (haptics.intensity) {
    ret = regmap_update_bits(arizona.regmap,
    ARIZONA_HAPTICS_PHASE_2_INTENSITY,
    ARIZONA_PHASE2_INTENSITY_MASK,
    haptics.intensity);
    if (ret != 0) {
    dev_err(arizona.dev, "Failed to set intensity: %d\n",
    ret);
    return;
    }
// This enable sequence will be a noop if already enabled
    ret = regmap_update_bits(arizona.regmap,
    ARIZONA_HAPTICS_CONTROL_1,
    ARIZONA_HAP_CTRL_MASK,
    1 << ARIZONA_HAP_CTRL_SHIFT);
    if (ret != 0) {
    dev_err(arizona.dev, "Failed to start haptics: %d\n",
    ret);
    return;
    }
    ret = snd_soc_dapm_enable_pin(arizona.dapm, "HAPTICS");
    if (ret != 0) {
    dev_err(arizona.dev, "Failed to start HAPTICS: %d\n",
    ret);
    return;
    }
    ret = snd_soc_dapm_sync(arizona.dapm);
    if (ret != 0) {
    dev_err(arizona.dev, "Failed to sync DAPM: %d\n",
    ret);
    return;
    }
    } else {
// This disable sequence will be a noop if already enabled
    ret = snd_soc_dapm_disable_pin(arizona.dapm, "HAPTICS");
    if (ret != 0) {
    dev_err(arizona.dev, "Failed to disable HAPTICS: %d\n",
    ret);
    return;
    }
    ret = snd_soc_dapm_sync(arizona.dapm);
    if (ret != 0) {
    dev_err(arizona.dev, "Failed to sync DAPM: %d\n",
    ret);
    return;
    }
    ret = regmap_update_bits(arizona.regmap,
    ARIZONA_HAPTICS_CONTROL_1,
    ARIZONA_HAP_CTRL_MASK, 0);
    if (ret != 0) {
    dev_err(arizona.dev, "Failed to stop haptics: %d\n",
    ret);
    return;
    }
    }
    }
    static int arizona_haptics_play(struct input_dev *input, void *data,
    struct ff_effect *effect)
    {
    struct arizona_haptics *haptics = input_get_drvdata(input);
    struct arizona *arizona = haptics.arizona;
    if (!arizona.dapm) {
    dev_err(arizona.dev, "No DAPM context\n");
    return -EBUSY;
    }
    if (effect.u.rumble.strong_magnitude) {
// Scale the magnitude into the range the device supports
    if (arizona.pdata.hap_act) {
    haptics.intensity =
    effect.u.rumble.strong_magnitude >> 9;
    if (effect.direction < 0x8000)
    haptics.intensity += 0x7f;
    } else {
    haptics.intensity =
    effect.u.rumble.strong_magnitude >> 8;
    }
    } else {
    haptics.intensity = 0;
    }
    schedule_work(&haptics.work);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn arizona_haptics_close(input: *mut input_dev) {
    static void arizona_haptics_close(struct input_dev *input)
    {
    struct arizona_haptics *haptics = input_get_drvdata(input);
    struct snd_soc_dapm_context *dapm = haptics.arizona.dapm;
    cancel_work_sync(&haptics.work);
    if (dapm)
    snd_soc_dapm_disable_pin(dapm, "HAPTICS");
    }
#[no_mangle]
unsafe extern "C" fn arizona_haptics_probe(pdev: *mut platform_device) -> c_int {
    static int arizona_haptics_probe(struct platform_device *pdev)
    {
    struct arizona *arizona = dev_get_drvdata(pdev.dev.parent);
    struct arizona_haptics *haptics;
    int ret;
    haptics = devm_kzalloc(&pdev.dev, sizeof(*haptics), GFP_KERNEL);
    if (!haptics)
    return -ENOMEM;
    haptics.arizona = arizona;
    ret = regmap_update_bits(arizona.regmap, ARIZONA_HAPTICS_CONTROL_1,
    ARIZONA_HAP_ACT, arizona.pdata.hap_act);
    if (ret != 0) {
    dev_err(arizona.dev, "Failed to set haptics actuator: %d\n",
    ret);
    return ret;
    }
    INIT_WORK(&haptics.work, arizona_haptics_work);
    haptics.input_dev = devm_input_allocate_device(&pdev.dev);
    if (!haptics.input_dev) {
    dev_err(arizona.dev, "Failed to allocate input device\n");
    return -ENOMEM;
    }
    input_set_drvdata(haptics.input_dev, haptics);
    haptics.input_dev.name = "arizona:haptics";
    haptics.input_dev.close = arizona_haptics_close;
    __set_bit(FF_RUMBLE, haptics.input_dev.ffbit);
    ret = input_ff_create_memless(haptics.input_dev, core::ptr::null_mut(),
    arizona_haptics_play);
    if (ret < 0) {
    dev_err(arizona.dev, "input_ff_create_memless() failed: %d\n",
    ret);
    return ret;
    }
    ret = input_register_device(haptics.input_dev);
    if (ret < 0) {
    dev_err(arizona.dev, "couldn't register input device: %d\n",
    ret);
    return ret;
    }
    return 0;
    }
    static struct platform_driver arizona_haptics_driver = {
    .probe		= arizona_haptics_probe,
    .driver		= {
    .name	= "arizona-haptics",
    },
    };
    module_platform_driver(arizona_haptics_driver);
    MODULE_ALIAS("platform:arizona-haptics");
    MODULE_DESCRIPTION("Arizona haptics driver");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Mark Brown <broonie@opensource.wolfsonmicro.com>");
