//! Automatically rewritten from C to Rust
//! Source: drivers/input/misc/drv2667.c
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
// DRV2667 haptics driver family
//
// Author: Dan Murphy <dmurphy@ti.com>
//
// Copyright: (C) 2014 Texas Instruments, Inc.
//

// Control registers
pub const DRV2667_STATUS: c_uint = 0x00;
pub const DRV2667_CTRL_1: c_uint = 0x01;
pub const DRV2667_CTRL_2: c_uint = 0x02;
// Waveform sequencer
pub const DRV2667_WV_SEQ_0: c_uint = 0x03;
pub const DRV2667_WV_SEQ_1: c_uint = 0x04;
pub const DRV2667_WV_SEQ_2: c_uint = 0x05;
pub const DRV2667_WV_SEQ_3: c_uint = 0x06;
pub const DRV2667_WV_SEQ_4: c_uint = 0x07;
pub const DRV2667_WV_SEQ_5: c_uint = 0x08;
pub const DRV2667_WV_SEQ_6: c_uint = 0x09;
pub const DRV2667_WV_SEQ_7: c_uint = 0x0A;
pub const DRV2667_FIFO: c_uint = 0x0B;
pub const DRV2667_PAGE: c_uint = 0xFF;

pub const DRV2667_PAGE_0: c_uint = 0x00;
pub const DRV2667_PAGE_1: c_uint = 0x01;
pub const DRV2667_PAGE_2: c_uint = 0x02;
pub const DRV2667_PAGE_3: c_uint = 0x03;
pub const DRV2667_PAGE_4: c_uint = 0x04;
pub const DRV2667_PAGE_5: c_uint = 0x05;
pub const DRV2667_PAGE_6: c_uint = 0x06;
pub const DRV2667_PAGE_7: c_uint = 0x07;
pub const DRV2667_PAGE_8: c_uint = 0x08;
// RAM fields
pub const DRV2667_RAM_HDR_SZ: c_uint = 0x0;
// RAM Header addresses
pub const DRV2667_RAM_START_HI: c_uint = 0x01;
pub const DRV2667_RAM_START_LO: c_uint = 0x02;
pub const DRV2667_RAM_STOP_HI: c_uint = 0x03;
pub const DRV2667_RAM_STOP_LO: c_uint = 0x04;
pub const DRV2667_RAM_REPEAT_CT: c_uint = 0x05;
// RAM data addresses
pub const DRV2667_RAM_AMP: c_uint = 0x06;
pub const DRV2667_RAM_FREQ: c_uint = 0x07;
pub const DRV2667_RAM_DURATION: c_uint = 0x08;
pub const DRV2667_RAM_ENVELOPE: c_uint = 0x09;
// Control 1 Register
pub const DRV2667_25_VPP_GAIN: c_uint = 0x00;
pub const DRV2667_50_VPP_GAIN: c_uint = 0x01;
pub const DRV2667_75_VPP_GAIN: c_uint = 0x02;
pub const DRV2667_100_VPP_GAIN: c_uint = 0x03;
pub const DRV2667_DIGITAL_IN: c_uint = 0xfc;

// Control 2 Register

// RAM Envelope settings
pub const DRV2667_NO_ENV: c_uint = 0x00;
pub const DRV2667_32_MS_ENV: c_uint = 0x01;
pub const DRV2667_64_MS_ENV: c_uint = 0x02;
pub const DRV2667_96_MS_ENV: c_uint = 0x03;
pub const DRV2667_128_MS_ENV: c_uint = 0x04;
pub const DRV2667_160_MS_ENV: c_uint = 0x05;
pub const DRV2667_192_MS_ENV: c_uint = 0x06;
pub const DRV2667_224_MS_ENV: c_uint = 0x07;
pub const DRV2667_256_MS_ENV: c_uint = 0x08;
pub const DRV2667_512_MS_ENV: c_uint = 0x09;
pub const DRV2667_768_MS_ENV: c_uint = 0x0a;
pub const DRV2667_1024_MS_ENV: c_uint = 0x0b;
pub const DRV2667_1280_MS_ENV: c_uint = 0x0c;
pub const DRV2667_1536_MS_ENV: c_uint = 0x0d;
pub const DRV2667_1792_MS_ENV: c_uint = 0x0e;
pub const DRV2667_2048_MS_ENV: c_uint = 0x0f;
//
// struct drv2667_data -
// @input_dev: Pointer to the input device
// @client: Pointer to the I2C client
// @regmap: Register map of the device
// @work: Work item used to off load the enable/disable of the vibration
// @regulator: Pointer to the regulator for the IC
// @page: Page number
// @magnitude: Magnitude of the vibration event
// @frequency: Frequency of the vibration event
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drv2667_data {
    pub input_dev: *mut input_dev,
    pub client: *mut i2c_client,
    pub regmap: *mut regmap,
    pub work: work_struct,
    pub regulator: *mut regulator,
    pub page: u32,
    pub magnitude: u32,
    pub frequency: u32,
}

    static const struct reg_default drv2667_reg_defs[] = {
    { DRV2667_STATUS, 0x02 },
    { DRV2667_CTRL_1, 0x28 },
    { DRV2667_CTRL_2, 0x40 },
    { DRV2667_WV_SEQ_0, 0x00 },
    { DRV2667_WV_SEQ_1, 0x00 },
    { DRV2667_WV_SEQ_2, 0x00 },
    { DRV2667_WV_SEQ_3, 0x00 },
    { DRV2667_WV_SEQ_4, 0x00 },
    { DRV2667_WV_SEQ_5, 0x00 },
    { DRV2667_WV_SEQ_6, 0x00 },
    { DRV2667_WV_SEQ_7, 0x00 },
    { DRV2667_FIFO, 0x00 },
    { DRV2667_PAGE, 0x00 },
    };
#[no_mangle]
unsafe extern "C" fn drv2667_set_waveform_freq(haptics: *mut drv2667_data) -> c_int {
    static int drv2667_set_waveform_freq(struct drv2667_data *haptics)
    {
    unsigned int read_buf;
    int freq;
    int error;
// Per the data sheet:
// Sinusoid Frequency (Hz) = 7.8125 x Frequency
//
    freq = (haptics.frequency * 1000) / 78125;
    if (freq <= 0) {
    dev_err(&haptics.client.dev,
    "ERROR: Frequency calculated to %i\n", freq);
    return -EINVAL;
    }
    error = regmap_read(haptics.regmap, DRV2667_PAGE, &read_buf);
    if (error) {
    dev_err(&haptics.client.dev,
    "Failed to read the page number: %d\n", error);
    return -EIO;
    }
    if (read_buf == DRV2667_PAGE_0 ||
    haptics.page != read_buf) {
    error = regmap_write(haptics.regmap,
    DRV2667_PAGE, haptics.page);
    if (error) {
    dev_err(&haptics.client.dev,
    "Failed to set the page: %d\n", error);
    return -EIO;
    }
    }
    error = regmap_write(haptics.regmap, DRV2667_RAM_FREQ,	freq);
    if (error)
    dev_err(&haptics.client.dev,
    "Failed to set the frequency: %d\n", error);
// Reset back to original page
    if (read_buf == DRV2667_PAGE_0 ||
    haptics.page != read_buf) {
    error = regmap_write(haptics.regmap, DRV2667_PAGE, read_buf);
    if (error) {
    dev_err(&haptics.client.dev,
    "Failed to set the page: %d\n", error);
    return -EIO;
    }
    }
    return error;
    }
#[no_mangle]
unsafe extern "C" fn drv2667_worker(work: *mut work_struct) {
    static void drv2667_worker(struct work_struct *work)
    {
    struct drv2667_data *haptics = container_of(work, struct drv2667_data, work);
    int error;
    if (haptics.magnitude) {
    error = regmap_write(haptics.regmap,
    DRV2667_PAGE, haptics.page);
    if (error) {
    dev_err(&haptics.client.dev,
    "Failed to set the page: %d\n", error);
    return;
    }
    error = regmap_write(haptics.regmap, DRV2667_RAM_AMP,
    haptics.magnitude);
    if (error) {
    dev_err(&haptics.client.dev,
    "Failed to set the amplitude: %d\n", error);
    return;
    }
    error = regmap_write(haptics.regmap,
    DRV2667_PAGE, DRV2667_PAGE_0);
    if (error) {
    dev_err(&haptics.client.dev,
    "Failed to set the page: %d\n", error);
    return;
    }
    error = regmap_write(haptics.regmap,
    DRV2667_CTRL_2, DRV2667_GO);
    if (error) {
    dev_err(&haptics.client.dev,
    "Failed to set the GO bit: %d\n", error);
    }
    } else {
    error = regmap_update_bits(haptics.regmap, DRV2667_CTRL_2,
    DRV2667_GO, 0);
    if (error) {
    dev_err(&haptics.client.dev,
    "Failed to unset the GO bit: %d\n", error);
    }
    }
    }
    static int drv2667_haptics_play(struct input_dev *input, void *data,
    struct ff_effect *effect)
    {
    struct drv2667_data *haptics = input_get_drvdata(input);
    if (effect.u.rumble.strong_magnitude > 0)
    haptics.magnitude = effect.u.rumble.strong_magnitude;
#[no_mangle]
pub unsafe extern "C" fn if(0: effect->u.rumble.weak_magnitude >) -> else {
    else if (effect.u.rumble.weak_magnitude > 0)
    haptics.magnitude = effect.u.rumble.weak_magnitude;
    else
    haptics.magnitude = 0;
    schedule_work(&haptics.work);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn drv2667_close(input: *mut input_dev) {
    static void drv2667_close(struct input_dev *input)
    {
    struct drv2667_data *haptics = input_get_drvdata(input);
    int error;
    cancel_work_sync(&haptics.work);
    error = regmap_update_bits(haptics.regmap, DRV2667_CTRL_2,
    DRV2667_STANDBY, DRV2667_STANDBY);
    if (error)
    dev_err(&haptics.client.dev,
    "Failed to enter standby mode: %d\n", error);
    }
    static const struct reg_sequence drv2667_init_regs[] = {
    { DRV2667_CTRL_2, 0 },
    { DRV2667_CTRL_1, DRV2667_25_VPP_GAIN },
    { DRV2667_WV_SEQ_0, 1 },
    { DRV2667_WV_SEQ_1, 0 }
    };
    static const struct reg_sequence drv2667_page1_init[] = {
    { DRV2667_RAM_HDR_SZ, 0x05 },
    { DRV2667_RAM_START_HI, 0x80 },
    { DRV2667_RAM_START_LO, 0x06 },
    { DRV2667_RAM_STOP_HI, 0x00 },
    { DRV2667_RAM_STOP_LO, 0x09 },
    { DRV2667_RAM_REPEAT_CT, 0 },
    { DRV2667_RAM_DURATION, 0x05 },
    { DRV2667_RAM_ENVELOPE, DRV2667_NO_ENV },
    { DRV2667_RAM_AMP, 0x60 },
    };
#[no_mangle]
unsafe extern "C" fn drv2667_init(haptics: *mut drv2667_data) -> c_int {
    static int drv2667_init(struct drv2667_data *haptics)
    {
    int error;
// Set default haptic frequency to 195Hz on Page 1
    haptics.frequency = 195;
    haptics.page = DRV2667_PAGE_1;
    error = regmap_register_patch(haptics.regmap,
    drv2667_init_regs,
    ARRAY_SIZE(drv2667_init_regs));
    if (error) {
    dev_err(&haptics.client.dev,
    "Failed to write init registers: %d\n",
    error);
    return error;
    }
    error = regmap_write(haptics.regmap, DRV2667_PAGE, haptics.page);
    if (error) {
    dev_err(&haptics.client.dev, "Failed to set page: %d\n",
    error);
    goto error_out;
    }
    error = drv2667_set_waveform_freq(haptics);
    if (error)
    goto error_page;
    error = regmap_register_patch(haptics.regmap,
    drv2667_page1_init,
    ARRAY_SIZE(drv2667_page1_init));
    if (error) {
    dev_err(&haptics.client.dev,
    "Failed to write page registers: %d\n",
    error);
    return error;
    }
    error = regmap_write(haptics.regmap, DRV2667_PAGE, DRV2667_PAGE_0);
    return error;
    error_page:
    regmap_write(haptics.regmap, DRV2667_PAGE, DRV2667_PAGE_0);
    error_out:
    return error;
    }
    static const struct regmap_config drv2667_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = DRV2667_MAX_REG,
    .reg_defaults = drv2667_reg_defs,
    .num_reg_defaults = ARRAY_SIZE(drv2667_reg_defs),
    .cache_type = REGCACHE_NONE,
    };
#[no_mangle]
unsafe extern "C" fn drv2667_probe(client: *mut i2c_client) -> c_int {
    static int drv2667_probe(struct i2c_client *client)
    {
    struct drv2667_data *haptics;
    int error;
    haptics = devm_kzalloc(&client.dev, sizeof(*haptics), GFP_KERNEL);
    if (!haptics)
    return -ENOMEM;
    haptics.regulator = devm_regulator_get(&client.dev, "vbat");
    if (IS_ERR(haptics.regulator)) {
    error = PTR_ERR(haptics.regulator);
    dev_err(&client.dev,
    "unable to get regulator, error: %d\n", error);
    return error;
    }
    haptics.input_dev = devm_input_allocate_device(&client.dev);
    if (!haptics.input_dev) {
    dev_err(&client.dev, "Failed to allocate input device\n");
    return -ENOMEM;
    }
    haptics.input_dev.name = "drv2667:haptics";
    haptics.input_dev.dev.parent = client.dev.parent;
    haptics.input_dev.close = drv2667_close;
    input_set_drvdata(haptics.input_dev, haptics);
    input_set_capability(haptics.input_dev, EV_FF, FF_RUMBLE);
    error = input_ff_create_memless(haptics.input_dev, core::ptr::null_mut(),
    drv2667_haptics_play);
    if (error) {
    dev_err(&client.dev, "input_ff_create() failed: %d\n",
    error);
    return error;
    }
    INIT_WORK(&haptics.work, drv2667_worker);
    haptics.client = client;
    i2c_set_clientdata(client, haptics);
    haptics.regmap = devm_regmap_init_i2c(client, &drv2667_regmap_config);
    if (IS_ERR(haptics.regmap)) {
    error = PTR_ERR(haptics.regmap);
    dev_err(&client.dev, "Failed to allocate register map: %d\n",
    error);
    return error;
    }
    error = drv2667_init(haptics);
    if (error) {
    dev_err(&client.dev, "Device init failed: %d\n", error);
    return error;
    }
    error = input_register_device(haptics.input_dev);
    if (error) {
    dev_err(&client.dev, "couldn't register input device: %d\n",
    error);
    return error;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn drv2667_suspend(dev: *mut device) -> c_int {
    static int drv2667_suspend(struct device *dev)
    {
    struct drv2667_data *haptics = dev_get_drvdata(dev);
    int error;
    guard(mutex)(&haptics.input_dev.mutex);
    if (input_device_enabled(haptics.input_dev)) {
    error = regmap_update_bits(haptics.regmap, DRV2667_CTRL_2,
    DRV2667_STANDBY, DRV2667_STANDBY);
    if (error) {
    dev_err(dev, "Failed to set standby mode\n");
    regulator_disable(haptics.regulator);
    return error;
    }
    error = regulator_disable(haptics.regulator);
    if (error) {
    dev_err(dev, "Failed to disable regulator\n");
    regmap_update_bits(haptics.regmap,
    DRV2667_CTRL_2,
    DRV2667_STANDBY, 0);
    return error;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn drv2667_resume(dev: *mut device) -> c_int {
    static int drv2667_resume(struct device *dev)
    {
    struct drv2667_data *haptics = dev_get_drvdata(dev);
    int error;
    guard(mutex)(&haptics.input_dev.mutex);
    if (input_device_enabled(haptics.input_dev)) {
    error = regulator_enable(haptics.regulator);
    if (error) {
    dev_err(dev, "Failed to enable regulator\n");
    return error;
    }
    error = regmap_update_bits(haptics.regmap, DRV2667_CTRL_2,
    DRV2667_STANDBY, 0);
    if (error) {
    dev_err(dev, "Failed to unset standby mode\n");
    regulator_disable(haptics.regulator);
    return error;
    }
    }
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(drv2667_pm_ops, drv2667_suspend, drv2667_resume);
    static const struct i2c_device_id drv2667_id[] = {
    { .name = "drv2667" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, drv2667_id);

    static const struct of_device_id drv2667_of_match[] = {
    { .compatible = "ti,drv2667", },
    { }
    };
    MODULE_DEVICE_TABLE(of, drv2667_of_match);

    static struct i2c_driver drv2667_driver = {
    .probe		= drv2667_probe,
    .driver		= {
    .name	= "drv2667-haptics",
    .of_match_table = of_match_ptr(drv2667_of_match),
    .pm	= pm_sleep_ptr(&drv2667_pm_ops),
    },
    .id_table = drv2667_id,
    };
    module_i2c_driver(drv2667_driver);
    MODULE_DESCRIPTION("TI DRV2667 haptics driver");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Dan Murphy <dmurphy@ti.com>");
