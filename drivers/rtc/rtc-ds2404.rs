//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-ds2404.c
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
// Copyright (C) 2012 Sven Schnelle <svens@stackframe.org>

pub const DS2404_STATUS_REG: c_uint = 0x200;
pub const DS2404_CONTROL_REG: c_uint = 0x201;
pub const DS2404_RTC_REG: c_uint = 0x202;
pub const DS2404_WRITE_SCRATCHPAD_CMD: c_uint = 0x0f;
pub const DS2404_READ_SCRATCHPAD_CMD: c_uint = 0xaa;
pub const DS2404_COPY_SCRATCHPAD_CMD: c_uint = 0x55;
pub const DS2404_READ_MEMORY_CMD: c_uint = 0xf0;
pub const DS2404_RST: c_int = 0;
pub const DS2404_CLK: c_int = 1;
pub const DS2404_DQ: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ds2404 {
    pub dev: *mut device,
    pub rst_gpiod: *mut gpio_desc,
    pub clk_gpiod: *mut gpio_desc,
    pub dq_gpiod: *mut gpio_desc,
}

#[no_mangle]
unsafe extern "C" fn ds2404_gpio_map(chip: *mut ds2404, pdev: *mut platform_device) -> c_int {
    static int ds2404_gpio_map(struct ds2404 *chip, struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
// This will de-assert RESET, declare this GPIO as GPIOD_ACTIVE_LOW
    chip.rst_gpiod = devm_gpiod_get(dev, "rst", GPIOD_OUT_LOW);
    if (IS_ERR(chip.rst_gpiod))
    return PTR_ERR(chip.rst_gpiod);
    chip.clk_gpiod = devm_gpiod_get(dev, "clk", GPIOD_OUT_HIGH);
    if (IS_ERR(chip.clk_gpiod))
    return PTR_ERR(chip.clk_gpiod);
    chip.dq_gpiod = devm_gpiod_get(dev, "dq", GPIOD_ASIS);
    if (IS_ERR(chip.dq_gpiod))
    return PTR_ERR(chip.dq_gpiod);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ds2404_reset(chip: *mut ds2404) {
    static void ds2404_reset(struct ds2404 *chip)
    {
    gpiod_set_value(chip.rst_gpiod, 1);
    udelay(1000);
    gpiod_set_value(chip.rst_gpiod, 0);
    gpiod_set_value(chip.clk_gpiod, 0);
    gpiod_direction_output(chip.dq_gpiod, 0);
    udelay(10);
    }
#[no_mangle]
unsafe extern "C" fn ds2404_write_byte(chip: *mut ds2404, byte: u8) {
    static void ds2404_write_byte(struct ds2404 *chip, u8 byte)
    {
    int i;
    gpiod_direction_output(chip.dq_gpiod, 1);
    for (i = 0; i < 8; i++) {
    gpiod_set_value(chip.dq_gpiod, byte & (1 << i));
    udelay(10);
    gpiod_set_value(chip.clk_gpiod, 1);
    udelay(10);
    gpiod_set_value(chip.clk_gpiod, 0);
    udelay(10);
    }
    }
#[no_mangle]
unsafe extern "C" fn ds2404_read_byte(chip: *mut ds2404) -> u8 {
    static u8 ds2404_read_byte(struct ds2404 *chip)
    {
    int i;
    let mut ret: u8 = 0;
    gpiod_direction_input(chip.dq_gpiod);
    for (i = 0; i < 8; i++) {
    gpiod_set_value(chip.clk_gpiod, 0);
    udelay(10);
    if (gpiod_get_value(chip.dq_gpiod))
    ret |= 1 << i;
    gpiod_set_value(chip.clk_gpiod, 1);
    udelay(10);
    }
    return ret;
    }
    static void ds2404_read_memory(struct ds2404 *chip, u16 offset,
    int length, u8 *out)
    {
    ds2404_reset(chip);
    ds2404_write_byte(chip, DS2404_READ_MEMORY_CMD);
    ds2404_write_byte(chip, offset & 0xff);
    ds2404_write_byte(chip, (offset >> 8) & 0xff);
    while (length--)
// out++ = ds2404_read_byte(chip);
    }
    static void ds2404_write_memory(struct ds2404 *chip, u16 offset,
    int length, u8 *out)
    {
    int i;
    u8 ta01, ta02, es;
    ds2404_reset(chip);
    ds2404_write_byte(chip, DS2404_WRITE_SCRATCHPAD_CMD);
    ds2404_write_byte(chip, offset & 0xff);
    ds2404_write_byte(chip, (offset >> 8) & 0xff);
    for (i = 0; i < length; i++)
    ds2404_write_byte(chip, out[i]);
    ds2404_reset(chip);
    ds2404_write_byte(chip, DS2404_READ_SCRATCHPAD_CMD);
    ta01 = ds2404_read_byte(chip);
    ta02 = ds2404_read_byte(chip);
    es = ds2404_read_byte(chip);
    for (i = 0; i < length; i++) {
    if (out[i] != ds2404_read_byte(chip)) {
    dev_err(chip.dev, "read invalid data\n");
    return;
    }
    }
    ds2404_reset(chip);
    ds2404_write_byte(chip, DS2404_COPY_SCRATCHPAD_CMD);
    ds2404_write_byte(chip, ta01);
    ds2404_write_byte(chip, ta02);
    ds2404_write_byte(chip, es);
    while (gpiod_get_value(chip.dq_gpiod))
    ;
    }
#[no_mangle]
unsafe extern "C" fn ds2404_enable_osc(chip: *mut ds2404) {
    static void ds2404_enable_osc(struct ds2404 *chip)
    {
    u8 in[1] = { 0x10 }; /* enable oscillator */
    ds2404_write_memory(chip, 0x201, 1, in);
    }
#[no_mangle]
unsafe extern "C" fn ds2404_read_time(dev: *mut device, dt: *mut rtc_time) -> c_int {
    static int ds2404_read_time(struct device *dev, struct rtc_time *dt)
    {
    struct ds2404 *chip = dev_get_drvdata(dev);
    let mut time: c_ulong = 0;
    let mut hw_time: __le32 = 0;
    ds2404_read_memory(chip, 0x203, 4, (u8 *)&hw_time);
    time = le32_to_cpu(hw_time);
    rtc_time64_to_tm(time, dt);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ds2404_set_time(dev: *mut device, dt: *mut rtc_time) -> c_int {
    static int ds2404_set_time(struct device *dev, struct rtc_time *dt)
    {
    struct ds2404 *chip = dev_get_drvdata(dev);
    let mut time: u32 = cpu_to_le32(rtc_tm_to_time64(dt));
    ds2404_write_memory(chip, 0x203, 4, (u8 *)&time);
    return 0;
    }
    static const struct rtc_class_ops ds2404_rtc_ops = {
    .read_time	= ds2404_read_time,
    .set_time	= ds2404_set_time,
    };
#[no_mangle]
unsafe extern "C" fn rtc_probe(pdev: *mut platform_device) -> c_int {
    static int rtc_probe(struct platform_device *pdev)
    {
    struct ds2404 *chip;
    struct rtc_device *rtc;
    let mut retval: c_int = -EBUSY;
    chip = devm_kzalloc(&pdev.dev, sizeof(struct ds2404), GFP_KERNEL);
    if (!chip)
    return -ENOMEM;
    chip.dev = &pdev.dev;
    rtc = devm_rtc_allocate_device(&pdev.dev);
    if (IS_ERR(rtc))
    return PTR_ERR(rtc);
    retval = ds2404_gpio_map(chip, pdev);
    if (retval)
    return retval;
    platform_set_drvdata(pdev, chip);
    rtc.ops = &ds2404_rtc_ops;
    rtc.range_max = U32_MAX;
    retval = devm_rtc_register_device(rtc);
    if (retval)
    return retval;
    ds2404_enable_osc(chip);
    return 0;
    }
    static struct platform_driver rtc_device_driver = {
    .probe	= rtc_probe,
    .driver = {
    .name	= "ds2404",
    },
    };
    module_platform_driver(rtc_device_driver);
    MODULE_DESCRIPTION("DS2404 RTC");
    MODULE_AUTHOR("Sven Schnelle");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:ds2404");
