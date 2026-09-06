//! Automatically rewritten from C to Rust
//! Source: drivers/bus/ts-nbus.c
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
// NBUS driver for TS-4600 based boards
//
// Copyright (c) 2016 - Savoir-faire Linux
// Author: Sebastien Bourdelin <sebastien.bourdelin@savoirfairelinux.com>
//
// This driver implements a GPIOs bit-banged bus, called the NBUS by Technologic
// Systems. It is used to communicate with the peripherals in the FPGA on the
// TS-4600 SoM.
//

pub const TS_NBUS_DIRECTION_IN: c_int = 0;
pub const TS_NBUS_DIRECTION_OUT: c_int = 1;
pub const TS_NBUS_WRITE_ADR: c_int = 0;
pub const TS_NBUS_WRITE_VAL: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ts_nbus {
    pub pwm: *mut pwm_device,
    pub data: *mut gpio_descs,
    pub csn: *mut gpio_desc,
    pub txrx: *mut gpio_desc,
    pub strobe: *mut gpio_desc,
    pub ale: *mut gpio_desc,
    pub rdy: *mut gpio_desc,
    pub lock: mutex,
}

//
// request all gpios required by the bus.
//
    static int ts_nbus_init_pdata(struct platform_device *pdev,
    struct ts_nbus *ts_nbus)
    {
    ts_nbus.data = devm_gpiod_get_array(&pdev.dev, "ts,data",
    GPIOD_OUT_HIGH);
    if (IS_ERR(ts_nbus.data))
    return dev_err_probe(&pdev.dev, PTR_ERR(ts_nbus.data),
    "failed to retrieve ts,data-gpio from dts\n");
    ts_nbus.csn = devm_gpiod_get(&pdev.dev, "ts,csn", GPIOD_OUT_HIGH);
    if (IS_ERR(ts_nbus.csn))
    return dev_err_probe(&pdev.dev, PTR_ERR(ts_nbus.csn),
    "failed to retrieve ts,csn-gpio from dts\n");
    ts_nbus.txrx = devm_gpiod_get(&pdev.dev, "ts,txrx", GPIOD_OUT_HIGH);
    if (IS_ERR(ts_nbus.txrx))
    return dev_err_probe(&pdev.dev, PTR_ERR(ts_nbus.txrx),
    "failed to retrieve ts,txrx-gpio from dts\n");
    ts_nbus.strobe = devm_gpiod_get(&pdev.dev, "ts,strobe", GPIOD_OUT_HIGH);
    if (IS_ERR(ts_nbus.strobe))
    return dev_err_probe(&pdev.dev, PTR_ERR(ts_nbus.strobe),
    "failed to retrieve ts,strobe-gpio from dts\n");
    ts_nbus.ale = devm_gpiod_get(&pdev.dev, "ts,ale", GPIOD_OUT_HIGH);
    if (IS_ERR(ts_nbus.ale))
    return dev_err_probe(&pdev.dev, PTR_ERR(ts_nbus.ale),
    "failed to retrieve ts,ale-gpio from dts\n");
    ts_nbus.rdy = devm_gpiod_get(&pdev.dev, "ts,rdy", GPIOD_IN);
    if (IS_ERR(ts_nbus.rdy))
    return dev_err_probe(&pdev.dev, PTR_ERR(ts_nbus.rdy),
    "failed to retrieve ts,rdy-gpio from dts\n");
    return 0;
    }
//
// the data gpios are used for reading and writing values, their directions
// should be adjusted accordingly.
//
#[no_mangle]
unsafe extern "C" fn ts_nbus_set_direction(ts_nbus: *mut ts_nbus, direction: c_int) {
    static void ts_nbus_set_direction(struct ts_nbus *ts_nbus, int direction)
    {
    int i;
    for (i = 0; i < 8; i++) {
    if (direction == TS_NBUS_DIRECTION_IN)
    gpiod_direction_input(ts_nbus.data.desc[i]);
    else
// when used as output the default state of the data
// lines are set to high
    gpiod_direction_output(ts_nbus.data.desc[i], 1);
    }
    }
//
// reset the bus in its initial state.
// The data, csn, strobe and ale lines must be zero'ed to let the FPGA knows a
// new transaction can be process.
//
#[no_mangle]
unsafe extern "C" fn ts_nbus_reset_bus(ts_nbus: *mut ts_nbus) {
    static void ts_nbus_reset_bus(struct ts_nbus *ts_nbus)
    {
    DECLARE_BITMAP(values, 8);
    values[0] = 0;
    gpiod_set_array_value_cansleep(8, ts_nbus.data.desc,
    ts_nbus.data.info, values);
    gpiod_set_value_cansleep(ts_nbus.csn, 0);
    gpiod_set_value_cansleep(ts_nbus.strobe, 0);
    gpiod_set_value_cansleep(ts_nbus.ale, 0);
    }
//
// let the FPGA knows it can process.
//
#[no_mangle]
unsafe extern "C" fn ts_nbus_start_transaction(ts_nbus: *mut ts_nbus) {
    static void ts_nbus_start_transaction(struct ts_nbus *ts_nbus)
    {
    gpiod_set_value_cansleep(ts_nbus.strobe, 1);
    }
//
// read a byte value from the data gpios.
// return 0 on success or negative errno on failure.
//
#[no_mangle]
unsafe extern "C" fn ts_nbus_read_byte(ts_nbus: *mut ts_nbus, val: *mut u8) -> c_int {
    static int ts_nbus_read_byte(struct ts_nbus *ts_nbus, u8 *val)
    {
    struct gpio_descs *gpios = ts_nbus.data;
    int ret, i;
// val = 0;
    for (i = 0; i < 8; i++) {
    ret = gpiod_get_value_cansleep(gpios.desc[i]);
    if (ret < 0)
    return ret;
    if (ret)
// val |= BIT(i);
    }
    return 0;
    }
//
// set the data gpios accordingly to the byte value.
//
#[no_mangle]
unsafe extern "C" fn ts_nbus_write_byte(ts_nbus: *mut ts_nbus, byte: u8) {
    static void ts_nbus_write_byte(struct ts_nbus *ts_nbus, u8 byte)
    {
    struct gpio_descs *gpios = ts_nbus.data;
    DECLARE_BITMAP(values, 8);
    values[0] = byte;
    gpiod_set_array_value_cansleep(8, gpios.desc, gpios.info, values);
    }
//
// reading the bus consists of resetting the bus, then notifying the FPGA to
// send the data in the data gpios and return the read value.
// return 0 on success or negative errno on failure.
//
#[no_mangle]
unsafe extern "C" fn ts_nbus_read_bus(ts_nbus: *mut ts_nbus, val: *mut u8) -> c_int {
    static int ts_nbus_read_bus(struct ts_nbus *ts_nbus, u8 *val)
    {
    ts_nbus_reset_bus(ts_nbus);
    ts_nbus_start_transaction(ts_nbus);
    return ts_nbus_read_byte(ts_nbus, val);
    }
//
// writing to the bus consists of resetting the bus, then define the type of
// command (address/value), write the data and notify the FPGA to retrieve the
// value in the data gpios.
//
#[no_mangle]
unsafe extern "C" fn ts_nbus_write_bus(ts_nbus: *mut ts_nbus, cmd: c_int, val: u8) {
    static void ts_nbus_write_bus(struct ts_nbus *ts_nbus, int cmd, u8 val)
    {
    ts_nbus_reset_bus(ts_nbus);
    if (cmd == TS_NBUS_WRITE_ADR)
    gpiod_set_value_cansleep(ts_nbus.ale, 1);
    ts_nbus_write_byte(ts_nbus, val);
    ts_nbus_start_transaction(ts_nbus);
    }
//
// read the value in the FPGA register at the given address.
// return 0 on success or negative errno on failure.
//
#[no_mangle]
pub unsafe extern "C" fn ts_nbus_read(ts_nbus: *mut ts_nbus, adr: u8, val: *mut u16) -> c_int {
    int ts_nbus_read(struct ts_nbus *ts_nbus, u8 adr, u16 *val)
    {
    int ret, i;
    u8 byte;
// bus access must be atomic
    mutex_lock(&ts_nbus.lock);
// set the bus in read mode
    gpiod_set_value_cansleep(ts_nbus.txrx, 0);
// write address
    ts_nbus_write_bus(ts_nbus, TS_NBUS_WRITE_ADR, adr);
// set the data gpios direction as input before reading
    ts_nbus_set_direction(ts_nbus, TS_NBUS_DIRECTION_IN);
// reading value MSB first
    do {
// val = 0;
    byte = 0;
    for (i = 1; i >= 0; i--) {
// read a byte from the bus, leave on error
    ret = ts_nbus_read_bus(ts_nbus, &byte);
    if (ret < 0)
    goto err;
// append the byte read to the final value
// val |= byte << (i * 8);
    }
    gpiod_set_value_cansleep(ts_nbus.csn, 1);
    ret = gpiod_get_value_cansleep(ts_nbus.rdy);
    } while (ret);
    err:
// restore the data gpios direction as output after reading
    ts_nbus_set_direction(ts_nbus, TS_NBUS_DIRECTION_OUT);
    mutex_unlock(&ts_nbus.lock);
    return ret;
    }
    EXPORT_SYMBOL_GPL(ts_nbus_read);
//
// write the desired value in the FPGA register at the given address.
//
#[no_mangle]
pub unsafe extern "C" fn ts_nbus_write(ts_nbus: *mut ts_nbus, adr: u8, val: u16) -> c_int {
    int ts_nbus_write(struct ts_nbus *ts_nbus, u8 adr, u16 val)
    {
    int i;
// bus access must be atomic
    mutex_lock(&ts_nbus.lock);
// set the bus in write mode
    gpiod_set_value_cansleep(ts_nbus.txrx, 1);
// write address
    ts_nbus_write_bus(ts_nbus, TS_NBUS_WRITE_ADR, adr);
// writing value MSB first
    for (i = 1; i >= 0; i--)
    ts_nbus_write_bus(ts_nbus, TS_NBUS_WRITE_VAL, (u8)(val >> (i * 8)));
// wait for completion
    gpiod_set_value_cansleep(ts_nbus.csn, 1);
    while (gpiod_get_value_cansleep(ts_nbus.rdy) != 0) {
    gpiod_set_value_cansleep(ts_nbus.csn, 0);
    gpiod_set_value_cansleep(ts_nbus.csn, 1);
    }
    mutex_unlock(&ts_nbus.lock);
    return 0;
    }
    EXPORT_SYMBOL_GPL(ts_nbus_write);
#[no_mangle]
unsafe extern "C" fn ts_nbus_probe(pdev: *mut platform_device) -> c_int {
    static int ts_nbus_probe(struct platform_device *pdev)
    {
    struct pwm_device *pwm;
    struct pwm_state state;
    struct device *dev = &pdev.dev;
    struct ts_nbus *ts_nbus;
    int ret;
    ts_nbus = devm_kzalloc(dev, sizeof(*ts_nbus), GFP_KERNEL);
    if (!ts_nbus)
    return -ENOMEM;
    mutex_init(&ts_nbus.lock);
    ret = ts_nbus_init_pdata(pdev, ts_nbus);
    if (ret < 0)
    return ret;
    pwm = devm_pwm_get(dev, core::ptr::null_mut());
    if (IS_ERR(pwm))
    return dev_err_probe(dev, PTR_ERR(pwm),
    "unable to request PWM\n");
    pwm_init_state(pwm, &state);
    if (!state.period)
    return dev_err_probe(dev, -EINVAL, "invalid PWM period\n");
    state.duty_cycle = state.period;
    state.enabled = true;
    ret = pwm_apply_might_sleep(pwm, &state);
    if (ret < 0)
    return dev_err_probe(dev, ret, "failed to configure PWM\n");
//
// we can now start the FPGA and populate the peripherals.
//
    ts_nbus.pwm = pwm;
//
// let the child nodes retrieve this instance of the ts-nbus.
//
    dev_set_drvdata(dev, ts_nbus);
    ret = of_platform_populate(dev.of_node, core::ptr::null_mut(), core::ptr::null_mut(), dev);
    if (ret < 0)
    return dev_err_probe(dev, ret,
    "failed to populate platform devices on bus\n");
    dev_info(dev, "initialized\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ts_nbus_remove(pdev: *mut platform_device) {
    static void ts_nbus_remove(struct platform_device *pdev)
    {
    struct ts_nbus *ts_nbus = dev_get_drvdata(&pdev.dev);
// shutdown the FPGA
    mutex_lock(&ts_nbus.lock);
    pwm_disable(ts_nbus.pwm);
    mutex_unlock(&ts_nbus.lock);
    }
    static const struct of_device_id ts_nbus_of_match[] = {
    { .compatible = "technologic,ts-nbus", },
    { },
    };
    MODULE_DEVICE_TABLE(of, ts_nbus_of_match);
    static struct platform_driver ts_nbus_driver = {
    .probe		= ts_nbus_probe,
    .remove		= ts_nbus_remove,
    .driver		= {
    .name	= "ts_nbus",
    .of_match_table = ts_nbus_of_match,
    },
    };
    module_platform_driver(ts_nbus_driver);
    MODULE_ALIAS("platform:ts_nbus");
    MODULE_AUTHOR("Sebastien Bourdelin <sebastien.bourdelin@savoirfairelinux.com>");
    MODULE_DESCRIPTION("Technologic Systems NBUS");
    MODULE_LICENSE("GPL v2");
