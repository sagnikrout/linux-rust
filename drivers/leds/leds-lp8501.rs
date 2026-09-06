//! Automatically rewritten from C to Rust
//! Source: drivers/leds/leds-lp8501.c
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
// TI LP8501 9 channel LED Driver
//
// Copyright (C) 2013 Texas Instruments
//
// Author: Milo(Woogyom) Kim <milo.kim@ti.com>
//

pub const LP8501_PAGES_PER_ENGINE: c_int = 1;
pub const LP8501_MAX_LEDS: c_int = 9;
// Registers
pub const LP8501_REG_ENABLE: c_uint = 0x00;

pub const LP8501_REG_OP_MODE: c_uint = 0x01;
pub const LP8501_REG_PWR_CONFIG: c_uint = 0x05;
pub const LP8501_PWR_CONFIG_M: c_uint = 0x03;
pub const LP8501_REG_LED_PWM_BASE: c_uint = 0x16;
pub const LP8501_REG_LED_CURRENT_BASE: c_uint = 0x26;
pub const LP8501_REG_CONFIG: c_uint = 0x36;

pub const LP8501_CP_MODE_MASK: c_uint = 0x18;
pub const LP8501_CP_MODE_SHIFT: c_int = 3;

pub const LP8501_REG_STATUS: c_uint = 0x3A;

pub const LP8501_REG_RESET: c_uint = 0x3D;
pub const LP8501_RESET: c_uint = 0xFF;
pub const LP8501_REG_PROG_MEM: c_uint = 0x50;
#[no_mangle]
unsafe extern "C" fn lp8501_post_init_device(chip: *mut lp55xx_chip) -> c_int {
    static int lp8501_post_init_device(struct lp55xx_chip *chip)
    {
    int ret;
    let mut val: u8 = LP8501_DEFAULT_CFG;
    ret = lp55xx_write(chip, LP8501_REG_ENABLE, LP8501_ENABLE);
    if (ret)
    return ret;
// Chip startup time is 500 us, 1 - 2 ms gives some margin
    usleep_range(1000, 2000);
    if (chip.pdata.clock_mode != LP55XX_CLOCK_EXT)
    val |= LP8501_INT_CLK;
    val |= (chip.pdata.charge_pump_mode << LP8501_CP_MODE_SHIFT) & LP8501_CP_MODE_MASK;
    ret = lp55xx_write(chip, LP8501_REG_CONFIG, val);
    if (ret)
    return ret;
// Power selection for each output
    return lp55xx_update_bits(chip, LP8501_REG_PWR_CONFIG,
    LP8501_PWR_CONFIG_M, chip.pdata.pwr_sel);
    }
#[no_mangle]
unsafe extern "C" fn lp8501_run_engine(chip: *mut lp55xx_chip, start: bool) {
    static void lp8501_run_engine(struct lp55xx_chip *chip, bool start)
    {
// stop engine
    if (!start) {
    lp55xx_stop_all_engine(chip);
    lp55xx_turn_off_channels(chip);
    return;
    }
    lp55xx_run_engine_common(chip);
    }
// Chip specific configurations
    static struct lp55xx_device_config lp8501_cfg = {
    .reg_op_mode = {
    .addr = LP8501_REG_OP_MODE,
    },
    .reg_exec = {
    .addr = LP8501_REG_ENABLE,
    },
    .engine_busy = {
    .addr = LP8501_REG_STATUS,
    .mask = LP8501_ENGINE_BUSY,
    },
    .reset = {
    .addr = LP8501_REG_RESET,
    .val  = LP8501_RESET,
    },
    .enable = {
    .addr = LP8501_REG_ENABLE,
    .val  = LP8501_ENABLE,
    },
    .prog_mem_base = {
    .addr = LP8501_REG_PROG_MEM,
    },
    .reg_led_pwm_base = {
    .addr = LP8501_REG_LED_PWM_BASE,
    },
    .reg_led_current_base = {
    .addr = LP8501_REG_LED_CURRENT_BASE,
    },
    .pages_per_engine   = LP8501_PAGES_PER_ENGINE,
    .max_channel  = LP8501_MAX_LEDS,
    .post_init_device   = lp8501_post_init_device,
    .brightness_fn      = lp55xx_led_brightness,
    .set_led_current    = lp55xx_set_led_current,
    .firmware_cb        = lp55xx_firmware_loaded_cb,
    .run_engine         = lp8501_run_engine,
    };
    static const struct i2c_device_id lp8501_id[] = {
    { .name = "lp8501", .driver_data = (kernel_ulong_t)&lp8501_cfg },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, lp8501_id);
    static const struct of_device_id of_lp8501_leds_match[] = {
    { .compatible = "ti,lp8501", .data = &lp8501_cfg, },
    {},
    };
    MODULE_DEVICE_TABLE(of, of_lp8501_leds_match);
    static struct i2c_driver lp8501_driver = {
    .driver = {
    .name	= "lp8501",
    .of_match_table = of_lp8501_leds_match,
    },
    .probe		= lp55xx_probe,
    .remove		= lp55xx_remove,
    .id_table	= lp8501_id,
    };
    module_i2c_driver(lp8501_driver);
    MODULE_DESCRIPTION("Texas Instruments LP8501 LED driver");
    MODULE_AUTHOR("Milo Kim");
    MODULE_LICENSE("GPL");
