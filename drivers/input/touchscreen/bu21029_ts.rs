//! Automatically rewritten from C to Rust
//! Source: drivers/input/touchscreen/bu21029_ts.c
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
//
// Rohm BU21029 touchscreen controller driver
//
// Copyright (C) 2015-2018 Bosch Sicherheitssysteme GmbH
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License version 2 as
// published by the Free Software Foundation.
//

//
// HW_ID1 Register (PAGE=0, ADDR=0x0E, Reset value=0x02, Read only)
// +--------+--------+--------+--------+--------+--------+--------+--------+
// |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
// +--------+--------+--------+--------+--------+--------+--------+--------+
// |                                 HW_IDH                                |
// +--------+--------+--------+--------+--------+--------+--------+--------+
// HW_ID2 Register (PAGE=0, ADDR=0x0F, Reset value=0x29, Read only)
// +--------+--------+--------+--------+--------+--------+--------+--------+
// |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
// +--------+--------+--------+--------+--------+--------+--------+--------+
// |                                 HW_IDL                                |
// +--------+--------+--------+--------+--------+--------+--------+--------+
// HW_IDH: high 8bits of IC's ID
// HW_IDL: low  8bits of IC's ID
//

pub const SUPPORTED_HWID: c_uint = 0x0229;
//
// CFR0 Register (PAGE=0, ADDR=0x00, Reset value=0x20)
// +--------+--------+--------+--------+--------+--------+--------+--------+
// |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
// +--------+--------+--------+--------+--------+--------+--------+--------+
// |   0    |   0    |  CALIB |  INTRM |   0    |   0    |   0    |   0    |
// +--------+--------+--------+--------+--------+--------+--------+--------+
// CALIB: 0 = not to use calibration result (*)
// 1 = use calibration result
// INTRM: 0 = INT output depend on "pen down" (*)
// 1 = INT output always "0"
//

pub const CFR0_VALUE: c_uint = 0x00;
//
// CFR1 Register (PAGE=0, ADDR=0x01, Reset value=0xA6)
// +--------+--------+--------+--------+--------+--------+--------+--------+
// |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
// +--------+--------+--------+--------+--------+--------+--------+--------+
// |  MAV   |         AVE[2:0]         |   0    |         SMPL[2:0]        |
// +--------+--------+--------+--------+--------+--------+--------+--------+
// MAV:  0 = median average filter off
// 1 = median average filter on (*)
// AVE:  AVE+1 = number of average samples for MAV,
// if AVE>SMPL, then AVE=SMPL (=3)
// SMPL: SMPL+1 = number of conversion samples for MAV (=7)
//

pub const CFR1_VALUE: c_uint = 0xA6;
//
// CFR2 Register (PAGE=0, ADDR=0x02, Reset value=0x04)
// +--------+--------+--------+--------+--------+--------+--------+--------+
// |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
// +--------+--------+--------+--------+--------+--------+--------+--------+
// |          INTVL_TIME[3:0]          |          TIME_ST_ADC[3:0]         |
// +--------+--------+--------+--------+--------+--------+--------+--------+
// INTVL_TIME: waiting time between completion of conversion
// and start of next conversion, only usable in
// autoscan mode (=20.480ms)
// TIME_ST_ADC: waiting time between application of voltage
// to panel and start of A/D conversion (=100us)
//

pub const CFR2_VALUE: c_uint = 0xC9;
//
// CFR3 Register (PAGE=0, ADDR=0x0B, Reset value=0x72)
// +--------+--------+--------+--------+--------+--------+--------+--------+
// |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
// +--------+--------+--------+--------+--------+--------+--------+--------+
// |  RM8   | STRETCH|  PU90K |  DUAL  |           PIDAC_OFS[3:0]          |
// +--------+--------+--------+--------+--------+--------+--------+--------+
// RM8: 0 = coordinate resolution is 12bit (*)
// 1 = coordinate resolution is 8bit
// STRETCH: 0 = SCL_STRETCH function off
// 1 = SCL_STRETCH function on (*)
// PU90K: 0 = internal pull-up resistance for touch detection is ~50kohms (*)
// 1 = internal pull-up resistance for touch detection is ~90kohms
// DUAL: 0 = dual touch detection off (*)
// 1 = dual touch detection on
// PIDAC_OFS: dual touch detection circuit adjustment, it is not necessary
// to change this from initial value
//

pub const CFR3_VALUE: c_uint = 0x42;
//
// LDO Register (PAGE=0, ADDR=0x0C, Reset value=0x00)
// +--------+--------+--------+--------+--------+--------+--------+--------+
// |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
// +--------+--------+--------+--------+--------+--------+--------+--------+
// |   0    |         PVDD[2:0]        |   0    |         AVDD[2:0]        |
// +--------+--------+--------+--------+--------+--------+--------+--------+
// PVDD: output voltage of panel output regulator (=2.000V)
// AVDD: output voltage of analog circuit regulator (=2.000V)
//

pub const LDO_VALUE: c_uint = 0x77;
//
// Serial Interface Command Byte 1 (CID=1)
// +--------+--------+--------+--------+--------+--------+--------+--------+
// |   D7   |   D6   |   D5   |   D4   |   D3   |   D2   |   D1   |   D0   |
// +--------+--------+--------+--------+--------+--------+--------+--------+
// |   1    |                 CF                |  CMSK  |  PDM   |  STP   |
// +--------+--------+--------+--------+--------+--------+--------+--------+
// CF: conversion function, see table 3 in datasheet p6 (=0000, automatic scan)
// CMSK: 0 = executes convert function (*)
// 1 = reads the convert result
// PDM: 0 = power down after convert function stops (*)
// 1 = keep power on after convert function stops
// STP: 1 = abort current conversion and power down, set to "0" automatically
//
pub const BU21029_AUTOSCAN: c_uint = 0x80;
//
// The timeout value needs to be larger than INTVL_TIME + tConv4 (sample and
// conversion time), where tConv4 is calculated by formula:
// tPON + tDLY1 + (tTIME_ST_ADC + (tADC * tSMPL) * 2 + tDLY2) * 3
// see figure 8 in datasheet p15 for details of each field.
//
pub const PEN_UP_TIMEOUT_MS: c_int = 50;
pub const STOP_DELAY_MIN_US: c_int = 50;
pub const STOP_DELAY_MAX_US: c_int = 1000;
pub const START_DELAY_MS: c_int = 2;
pub const BUF_LEN: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bu21029_ts_data {
    pub client: *mut i2c_client,
    pub in_dev: *mut input_dev,
    pub timer: timer_list,
    pub vdd: *mut regulator,
    pub reset_gpios: *mut gpio_desc,
    pub x_plate_ohms: u32,
    pub prop: touchscreen_properties,
}

#[no_mangle]
unsafe extern "C" fn bu21029_touch_report(bu21029: *mut bu21029_ts_data, buf: *const u8) {
    static void bu21029_touch_report(struct bu21029_ts_data *bu21029, const u8 *buf)
    {
    u16 x, y, z1, z2;
    u32 rz;
    let mut max_pressure: i32 = input_abs_get_max(bu21029.in_dev, ABS_PRESSURE);
//
// compose upper 8 and lower 4 bits into a 12bit value:
// +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
// |            ByteH              |            ByteL              |
// +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
// |b07|b06|b05|b04|b03|b02|b01|b00|b07|b06|b05|b04|b03|b02|b01|b00|
// +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
// |v11|v10|v09|v08|v07|v06|v05|v04|v03|v02|v01|v00| 0 | 0 | 0 | 0 |
// +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
//
    x  = (buf[0] << 4) | (buf[1] >> 4);
    y  = (buf[2] << 4) | (buf[3] >> 4);
    z1 = (buf[4] << 4) | (buf[5] >> 4);
    z2 = (buf[6] << 4) | (buf[7] >> 4);
    if (z1 && z2) {
//
// calculate Rz (pressure resistance value) by equation:
// Rz = Rx * (x/Q) * ((z2/z1) - 1), where
// Rx is x-plate resistance,
// Q  is the touch screen resolution (8bit = 256, 12bit = 4096)
// x, z1, z2 are the measured positions.
//
    rz  = z2 - z1;
    rz *= x;
    rz *= bu21029.x_plate_ohms;
    rz /= z1;
    rz  = DIV_ROUND_CLOSEST(rz, SCALE_12BIT);
    if (rz <= max_pressure) {
    touchscreen_report_pos(bu21029.in_dev, &bu21029.prop,
    x, y, false);
    input_report_abs(bu21029.in_dev, ABS_PRESSURE,
    max_pressure - rz);
    input_report_key(bu21029.in_dev, BTN_TOUCH, 1);
    input_sync(bu21029.in_dev);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn bu21029_touch_release(t: *mut timer_list) {
    static void bu21029_touch_release(struct timer_list *t)
    {
    struct bu21029_ts_data *bu21029 = timer_container_of(bu21029, t,
    timer);
    input_report_abs(bu21029.in_dev, ABS_PRESSURE, 0);
    input_report_key(bu21029.in_dev, BTN_TOUCH, 0);
    input_sync(bu21029.in_dev);
    }
#[no_mangle]
unsafe extern "C" fn bu21029_touch_soft_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t bu21029_touch_soft_irq(int irq, void *data)
    {
    struct bu21029_ts_data *bu21029 = data;
    u8 buf[BUF_LEN];
    int error;
//
// Read touch data and deassert interrupt (will assert again after
// INTVL_TIME + tConv4 for continuous touch)
//
    error = i2c_smbus_read_i2c_block_data(bu21029.client, BU21029_AUTOSCAN,
    sizeof(buf), buf);
    if (error < 0)
    goto out;
    bu21029_touch_report(bu21029, buf);
// reset timer for pen up detection
    mod_timer(&bu21029.timer,
    jiffies + msecs_to_jiffies(PEN_UP_TIMEOUT_MS));
    out:
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn bu21029_put_chip_in_reset(bu21029: *mut bu21029_ts_data) {
    static void bu21029_put_chip_in_reset(struct bu21029_ts_data *bu21029)
    {
    if (bu21029.reset_gpios) {
    gpiod_set_value_cansleep(bu21029.reset_gpios, 1);
    usleep_range(STOP_DELAY_MIN_US, STOP_DELAY_MAX_US);
    }
    }
#[no_mangle]
unsafe extern "C" fn bu21029_start_chip(dev: *mut input_dev) -> c_int {
    static int bu21029_start_chip(struct input_dev *dev)
    {
    struct bu21029_ts_data *bu21029 = input_get_drvdata(dev);
    struct i2c_client *i2c = bu21029.client;
    struct {
    u8 reg;
    u8 value;
    } init_table[] = {
    {BU21029_CFR0_REG, CFR0_VALUE},
    {BU21029_CFR1_REG, CFR1_VALUE},
    {BU21029_CFR2_REG, CFR2_VALUE},
    {BU21029_CFR3_REG, CFR3_VALUE},
    {BU21029_LDO_REG,  LDO_VALUE}
    };
    int error, i;
    __be16 hwid;
    error = regulator_enable(bu21029.vdd);
    if (error) {
    dev_err(&i2c.dev, "failed to power up chip: %d", error);
    return error;
    }
// take chip out of reset
    if (bu21029.reset_gpios) {
    gpiod_set_value_cansleep(bu21029.reset_gpios, 0);
    msleep(START_DELAY_MS);
    }
    error = i2c_smbus_read_i2c_block_data(i2c, BU21029_HWID_REG,
    sizeof(hwid), (u8 *)&hwid);
    if (error < 0) {
    dev_err(&i2c.dev, "failed to read HW ID\n");
    goto err_out;
    }
    if (be16_to_cpu(hwid) != SUPPORTED_HWID) {
    dev_err(&i2c.dev,
    "unsupported HW ID 0x%x\n", be16_to_cpu(hwid));
    error = -ENODEV;
    goto err_out;
    }
    for (i = 0; i < ARRAY_SIZE(init_table); ++i) {
    error = i2c_smbus_write_byte_data(i2c,
    init_table[i].reg,
    init_table[i].value);
    if (error < 0) {
    dev_err(&i2c.dev,
    "failed to write %#02x to register %#02x: %d\n",
    init_table[i].value, init_table[i].reg,
    error);
    goto err_out;
    }
    }
    error = i2c_smbus_write_byte(i2c, BU21029_AUTOSCAN);
    if (error < 0) {
    dev_err(&i2c.dev, "failed to start autoscan\n");
    goto err_out;
    }
    enable_irq(bu21029.client.irq);
    return 0;
    err_out:
    bu21029_put_chip_in_reset(bu21029);
    regulator_disable(bu21029.vdd);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn bu21029_stop_chip(dev: *mut input_dev) {
    static void bu21029_stop_chip(struct input_dev *dev)
    {
    struct bu21029_ts_data *bu21029 = input_get_drvdata(dev);
    disable_irq(bu21029.client.irq);
    timer_delete_sync(&bu21029.timer);
    bu21029_put_chip_in_reset(bu21029);
    regulator_disable(bu21029.vdd);
    }
#[no_mangle]
unsafe extern "C" fn bu21029_probe(client: *mut i2c_client) -> c_int {
    static int bu21029_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct bu21029_ts_data *bu21029;
    struct input_dev *in_dev;
    int error;
    if (!i2c_check_functionality(client.adapter,
    I2C_FUNC_SMBUS_WRITE_BYTE |
    I2C_FUNC_SMBUS_WRITE_BYTE_DATA |
    I2C_FUNC_SMBUS_READ_I2C_BLOCK)) {
    dev_err(dev, "i2c functionality support is not sufficient\n");
    return -EIO;
    }
    bu21029 = devm_kzalloc(dev, sizeof(*bu21029), GFP_KERNEL);
    if (!bu21029)
    return -ENOMEM;
    error = device_property_read_u32(dev, "rohm,x-plate-ohms", &bu21029.x_plate_ohms);
    if (error) {
    dev_err(dev, "invalid 'x-plate-ohms' supplied: %d\n", error);
    return error;
    }
    bu21029.vdd = devm_regulator_get(dev, "vdd");
    if (IS_ERR(bu21029.vdd))
    return dev_err_probe(dev, PTR_ERR(bu21029.vdd),
    "failed to acquire 'vdd' supply\n");
    bu21029.reset_gpios = devm_gpiod_get_optional(dev, "reset", GPIOD_OUT_HIGH);
    if (IS_ERR(bu21029.reset_gpios))
    return dev_err_probe(dev, PTR_ERR(bu21029.reset_gpios),
    "failed to acquire 'reset' gpio\n");
    in_dev = devm_input_allocate_device(dev);
    if (!in_dev) {
    dev_err(dev, "unable to allocate input device\n");
    return -ENOMEM;
    }
    bu21029.client = client;
    bu21029.in_dev = in_dev;
    timer_setup(&bu21029.timer, bu21029_touch_release, 0);
    in_dev.name		= DRIVER_NAME;
    in_dev.id.bustype	= BUS_I2C;
    in_dev.open		= bu21029_start_chip;
    in_dev.close		= bu21029_stop_chip;
    input_set_capability(in_dev, EV_KEY, BTN_TOUCH);
    input_set_abs_params(in_dev, ABS_X, 0, MAX_12BIT, 0, 0);
    input_set_abs_params(in_dev, ABS_Y, 0, MAX_12BIT, 0, 0);
    input_set_abs_params(in_dev, ABS_PRESSURE, 0, MAX_12BIT, 0, 0);
    touchscreen_parse_properties(in_dev, false, &bu21029.prop);
    input_set_drvdata(in_dev, bu21029);
    error = devm_request_threaded_irq(dev, client.irq, core::ptr::null_mut(),
    bu21029_touch_soft_irq,
    IRQF_ONESHOT | IRQF_NO_AUTOEN,
    DRIVER_NAME, bu21029);
    if (error) {
    dev_err(dev, "unable to request touch irq: %d\n", error);
    return error;
    }
    error = input_register_device(in_dev);
    if (error) {
    dev_err(dev, "unable to register input device: %d\n", error);
    return error;
    }
    i2c_set_clientdata(client, bu21029);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bu21029_suspend(dev: *mut device) -> c_int {
    static int bu21029_suspend(struct device *dev)
    {
    struct i2c_client *i2c = to_i2c_client(dev);
    struct bu21029_ts_data *bu21029 = i2c_get_clientdata(i2c);
    if (!device_may_wakeup(dev)) {
    guard(mutex)(&bu21029.in_dev.mutex);
    if (input_device_enabled(bu21029.in_dev))
    bu21029_stop_chip(bu21029.in_dev);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bu21029_resume(dev: *mut device) -> c_int {
    static int bu21029_resume(struct device *dev)
    {
    struct i2c_client *i2c = to_i2c_client(dev);
    struct bu21029_ts_data *bu21029 = i2c_get_clientdata(i2c);
    if (!device_may_wakeup(dev)) {
    guard(mutex)(&bu21029.in_dev.mutex);
    if (input_device_enabled(bu21029.in_dev))
    bu21029_start_chip(bu21029.in_dev);
    }
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(bu21029_pm_ops, bu21029_suspend, bu21029_resume);
    static const struct i2c_device_id bu21029_ids[] = {
    { .name = DRIVER_NAME },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(i2c, bu21029_ids);

    static const struct of_device_id bu21029_of_ids[] = {
    { .compatible = "rohm,bu21029" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, bu21029_of_ids);

    static struct i2c_driver bu21029_driver = {
    .driver	= {
    .name		= DRIVER_NAME,
    .of_match_table	= of_match_ptr(bu21029_of_ids),
    .pm		= pm_sleep_ptr(&bu21029_pm_ops),
    },
    .id_table	= bu21029_ids,
    .probe		= bu21029_probe,
    };
    module_i2c_driver(bu21029_driver);
    MODULE_AUTHOR("Zhu Yi <yi.zhu5@cn.bosch.com>");
    MODULE_DESCRIPTION("Rohm BU21029 touchscreen controller driver");
    MODULE_LICENSE("GPL v2");
