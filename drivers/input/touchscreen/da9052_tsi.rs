//! Automatically rewritten from C to Rust
//! Source: drivers/input/touchscreen/da9052_tsi.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// TSI driver for Dialog DA9052
//
// Copyright(c) 2012 Dialog Semiconductor Ltd.
//
// Author: David Dajun Chen <dchen@diasemi.com>
//

pub const TSI_PEN_DOWN_STATUS: c_uint = 0x40;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct da9052_tsi {
    pub da9052: *mut da9052,
    pub dev: *mut input_dev,
    pub ts_pen_work: delayed_work,
    pub stopped: bool,
    pub adc_on: bool,
}

#[no_mangle]
unsafe extern "C" fn da9052_ts_adc_toggle(tsi: *mut da9052_tsi, on: bool) {
    static void da9052_ts_adc_toggle(struct da9052_tsi *tsi, bool on)
    {
    da9052_reg_update(tsi.da9052, DA9052_TSI_CONT_A_REG, 1 << 0, on);
    tsi.adc_on = on;
    }
#[no_mangle]
unsafe extern "C" fn da9052_ts_pendwn_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t da9052_ts_pendwn_irq(int irq, void *data)
    {
    struct da9052_tsi *tsi = data;
    if (!tsi.stopped) {
// Mask PEN_DOWN event and unmask TSI_READY event
    da9052_disable_irq_nosync(tsi.da9052, DA9052_IRQ_PENDOWN);
    da9052_enable_irq(tsi.da9052, DA9052_IRQ_TSIREADY);
    da9052_ts_adc_toggle(tsi, true);
    schedule_delayed_work(&tsi.ts_pen_work, HZ / 50);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn da9052_ts_read(tsi: *mut da9052_tsi) {
    static void da9052_ts_read(struct da9052_tsi *tsi)
    {
    struct input_dev *input = tsi.dev;
    int ret;
    u16 x, y, z;
    u8 v;
    ret = da9052_reg_read(tsi.da9052, DA9052_TSI_X_MSB_REG);
    if (ret < 0)
    return;
    x = (u16) ret;
    ret = da9052_reg_read(tsi.da9052, DA9052_TSI_Y_MSB_REG);
    if (ret < 0)
    return;
    y = (u16) ret;
    ret = da9052_reg_read(tsi.da9052, DA9052_TSI_Z_MSB_REG);
    if (ret < 0)
    return;
    z = (u16) ret;
    ret = da9052_reg_read(tsi.da9052, DA9052_TSI_LSB_REG);
    if (ret < 0)
    return;
    v = (u8) ret;
    x = ((x << 2) & 0x3fc) | (v & 0x3);
    y = ((y << 2) & 0x3fc) | ((v & 0xc) >> 2);
    z = ((z << 2) & 0x3fc) | ((v & 0x30) >> 4);
    input_report_key(input, BTN_TOUCH, 1);
    input_report_abs(input, ABS_X, x);
    input_report_abs(input, ABS_Y, y);
    input_report_abs(input, ABS_PRESSURE, z);
    input_sync(input);
    }
#[no_mangle]
unsafe extern "C" fn da9052_ts_datardy_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t da9052_ts_datardy_irq(int irq, void *data)
    {
    struct da9052_tsi *tsi = data;
    da9052_ts_read(tsi);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn da9052_ts_pen_work(work: *mut work_struct) {
    static void da9052_ts_pen_work(struct work_struct *work)
    {
    struct da9052_tsi *tsi = container_of(work, struct da9052_tsi,
    ts_pen_work.work);
    if (!tsi.stopped) {
    let mut ret: c_int = da9052_reg_read(tsi.da9052, DA9052_TSI_LSB_REG);
    if (ret < 0 || (ret & TSI_PEN_DOWN_STATUS)) {
// Pen is still DOWN (or read error)
    schedule_delayed_work(&tsi.ts_pen_work, HZ / 50);
    } else {
    struct input_dev *input = tsi.dev;
// Pen UP
    da9052_ts_adc_toggle(tsi, false);
// Report Pen UP
    input_report_key(input, BTN_TOUCH, 0);
    input_report_abs(input, ABS_PRESSURE, 0);
    input_sync(input);
//
// FIXME: Fixes the unhandled irq issue when quick
// pen down and pen up events occurs
//
    ret = da9052_reg_update(tsi.da9052,
    DA9052_EVENT_B_REG, 0xC0, 0xC0);
    if (ret < 0)
    return;
// Mask TSI_READY event and unmask PEN_DOWN event
    da9052_disable_irq(tsi.da9052, DA9052_IRQ_TSIREADY);
    da9052_enable_irq(tsi.da9052, DA9052_IRQ_PENDOWN);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn da9052_ts_configure_gpio(da9052: *mut da9052) -> c_int {
    static int da9052_ts_configure_gpio(struct da9052 *da9052)
    {
    int error;
    error = da9052_reg_update(da9052, DA9052_GPIO_2_3_REG, 0x30, 0);
    if (error < 0)
    return error;
    error = da9052_reg_update(da9052, DA9052_GPIO_4_5_REG, 0x33, 0);
    if (error < 0)
    return error;
    error = da9052_reg_update(da9052, DA9052_GPIO_6_7_REG, 0x33, 0);
    if (error < 0)
    return error;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn da9052_configure_tsi(tsi: *mut da9052_tsi) -> c_int {
    static int da9052_configure_tsi(struct da9052_tsi *tsi)
    {
    int error;
    error = da9052_ts_configure_gpio(tsi.da9052);
    if (error)
    return error;
// Measure TSI sample every 1ms
    error = da9052_reg_update(tsi.da9052, DA9052_ADC_CONT_REG,
    1 << 6, 1 << 6);
    if (error < 0)
    return error;
// TSI_DELAY: 3 slots, TSI_SKIP: 0 slots, TSI_MODE: XYZP
    error = da9052_reg_update(tsi.da9052, DA9052_TSI_CONT_A_REG, 0xFC, 0xC0);
    if (error < 0)
    return error;
// Supply TSIRef through LD09
    error = da9052_reg_write(tsi.da9052, DA9052_LDO9_REG, 0x59);
    if (error < 0)
    return error;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn da9052_ts_input_open(input_dev: *mut input_dev) -> c_int {
    static int da9052_ts_input_open(struct input_dev *input_dev)
    {
    struct da9052_tsi *tsi = input_get_drvdata(input_dev);
    tsi.stopped = false;
    mb();
// Unmask PEN_DOWN event
    da9052_enable_irq(tsi.da9052, DA9052_IRQ_PENDOWN);
// Enable Pen Detect Circuit
    return da9052_reg_update(tsi.da9052, DA9052_TSI_CONT_A_REG,
    1 << 1, 1 << 1);
    }
#[no_mangle]
unsafe extern "C" fn da9052_ts_input_close(input_dev: *mut input_dev) {
    static void da9052_ts_input_close(struct input_dev *input_dev)
    {
    struct da9052_tsi *tsi = input_get_drvdata(input_dev);
    tsi.stopped = true;
    mb();
    da9052_disable_irq(tsi.da9052, DA9052_IRQ_PENDOWN);
    cancel_delayed_work_sync(&tsi.ts_pen_work);
    if (tsi.adc_on) {
    da9052_disable_irq(tsi.da9052, DA9052_IRQ_TSIREADY);
    da9052_ts_adc_toggle(tsi, false);
//
// If ADC was on that means that pendwn IRQ was disabled
// twice and we need to enable it to keep enable/disable
// counter balanced. IRQ is still off though.
//
    da9052_enable_irq(tsi.da9052, DA9052_IRQ_PENDOWN);
    }
// Disable Pen Detect Circuit
    da9052_reg_update(tsi.da9052, DA9052_TSI_CONT_A_REG, 1 << 1, 0);
    }
#[no_mangle]
unsafe extern "C" fn da9052_ts_probe(pdev: *mut platform_device) -> c_int {
    static int da9052_ts_probe(struct platform_device *pdev)
    {
    struct da9052 *da9052;
    struct da9052_tsi *tsi;
    struct input_dev *input_dev;
    int error;
    da9052 = dev_get_drvdata(pdev.dev.parent);
    if (!da9052)
    return -EINVAL;
    tsi = kzalloc_obj(*tsi);
    input_dev = input_allocate_device();
    if (!tsi || !input_dev) {
    error = -ENOMEM;
    goto err_free_mem;
    }
    tsi.da9052 = da9052;
    tsi.dev = input_dev;
    tsi.stopped = true;
    INIT_DELAYED_WORK(&tsi.ts_pen_work, da9052_ts_pen_work);
    input_dev.id.version = 0x0101;
    input_dev.id.vendor = 0x15B6;
    input_dev.id.product = 0x9052;
    input_dev.name = "Dialog DA9052 TouchScreen Driver";
    input_dev.dev.parent = &pdev.dev;
    input_dev.open = da9052_ts_input_open;
    input_dev.close = da9052_ts_input_close;
    __set_bit(EV_ABS, input_dev.evbit);
    __set_bit(EV_KEY, input_dev.evbit);
    __set_bit(BTN_TOUCH, input_dev.keybit);
    input_set_abs_params(input_dev, ABS_X, 0, 1023, 0, 0);
    input_set_abs_params(input_dev, ABS_Y, 0, 1023, 0, 0);
    input_set_abs_params(input_dev, ABS_PRESSURE, 0, 1023, 0, 0);
    input_set_drvdata(input_dev, tsi);
// Disable Pen Detect Circuit
    da9052_reg_update(tsi.da9052, DA9052_TSI_CONT_A_REG, 1 << 1, 0);
// Disable ADC
    da9052_ts_adc_toggle(tsi, false);
    error = da9052_request_irq(tsi.da9052, DA9052_IRQ_PENDOWN,
    "pendown-irq", da9052_ts_pendwn_irq, tsi);
    if (error) {
    dev_err(tsi.da9052.dev,
    "Failed to register PENDWN IRQ: %d\n", error);
    goto err_free_mem;
    }
    error = da9052_request_irq(tsi.da9052, DA9052_IRQ_TSIREADY,
    "tsiready-irq", da9052_ts_datardy_irq, tsi);
    if (error) {
    dev_err(tsi.da9052.dev,
    "Failed to register TSIRDY IRQ :%d\n", error);
    goto err_free_pendwn_irq;
    }
// Mask PEN_DOWN and TSI_READY events
    da9052_disable_irq(tsi.da9052, DA9052_IRQ_PENDOWN);
    da9052_disable_irq(tsi.da9052, DA9052_IRQ_TSIREADY);
    error = da9052_configure_tsi(tsi);
    if (error)
    goto err_free_datardy_irq;
    error = input_register_device(tsi.dev);
    if (error)
    goto err_free_datardy_irq;
    platform_set_drvdata(pdev, tsi);
    return 0;
    err_free_datardy_irq:
    da9052_free_irq(tsi.da9052, DA9052_IRQ_TSIREADY, tsi);
    err_free_pendwn_irq:
    da9052_free_irq(tsi.da9052, DA9052_IRQ_PENDOWN, tsi);
    err_free_mem:
    kfree(tsi);
    input_free_device(input_dev);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn da9052_ts_remove(pdev: *mut platform_device) {
    static void da9052_ts_remove(struct platform_device *pdev)
    {
    struct da9052_tsi *tsi = platform_get_drvdata(pdev);
    da9052_reg_write(tsi.da9052, DA9052_LDO9_REG, 0x19);
    da9052_free_irq(tsi.da9052, DA9052_IRQ_TSIREADY, tsi);
    da9052_free_irq(tsi.da9052, DA9052_IRQ_PENDOWN, tsi);
    input_unregister_device(tsi.dev);
    kfree(tsi);
    }
    static struct platform_driver da9052_tsi_driver = {
    .probe	= da9052_ts_probe,
    .remove	= da9052_ts_remove,
    .driver	= {
    .name	= "da9052-tsi",
    },
    };
    module_platform_driver(da9052_tsi_driver);
    MODULE_DESCRIPTION("Touchscreen driver for Dialog Semiconductor DA9052");
    MODULE_AUTHOR("Anthony Olech <Anthony.Olech@diasemi.com>");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:da9052-tsi");
