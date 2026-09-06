//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/wm831x-auxadc.c
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
// wm831x-auxadc.c  --  AUXADC for Wolfson WM831x PMICs
//
// Copyright 2009-2011 Wolfson Microelectronics PLC.
//
// Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm831x_auxadc_req {
    pub list: list_head,
    pub input: enum wm831x_auxadc,
    pub val: c_int,
    pub done: completion,
}

    static int wm831x_auxadc_read_irq(struct wm831x *wm831x,
    enum wm831x_auxadc input)
    {
    struct wm831x_auxadc_req *req;
    int ret;
    let mut ena: bool = false;
    req = kzalloc_obj(*req);
    if (!req)
    return -ENOMEM;
    init_completion(&req.done);
    req.input = input;
    req.val = -ETIMEDOUT;
    mutex_lock(&wm831x.auxadc_lock);
// Enqueue the request
    list_add(&req.list, &wm831x.auxadc_pending);
    ena = !wm831x.auxadc_active;
    if (ena) {
    ret = wm831x_set_bits(wm831x, WM831X_AUXADC_CONTROL,
    WM831X_AUX_ENA, WM831X_AUX_ENA);
    if (ret != 0) {
    dev_err(wm831x.dev, "Failed to enable AUXADC: %d\n",
    ret);
    goto out;
    }
    }
// Enable the conversion if not already running
    if (!(wm831x.auxadc_active & (1 << input))) {
    ret = wm831x_set_bits(wm831x, WM831X_AUXADC_SOURCE,
    1 << input, 1 << input);
    if (ret != 0) {
    dev_err(wm831x.dev,
    "Failed to set AUXADC source: %d\n", ret);
    goto out;
    }
    wm831x.auxadc_active |= 1 << input;
    }
// We convert at the fastest rate possible
    if (ena) {
    ret = wm831x_set_bits(wm831x, WM831X_AUXADC_CONTROL,
    WM831X_AUX_CVT_ENA |
    WM831X_AUX_RATE_MASK,
    WM831X_AUX_CVT_ENA |
    WM831X_AUX_RATE_MASK);
    if (ret != 0) {
    dev_err(wm831x.dev, "Failed to start AUXADC: %d\n",
    ret);
    goto out;
    }
    }
    mutex_unlock(&wm831x.auxadc_lock);
// Wait for an interrupt
    wait_for_completion_timeout(&req.done, msecs_to_jiffies(500));
    mutex_lock(&wm831x.auxadc_lock);
    ret = req.val;
    out:
    list_del(&req.list);
    mutex_unlock(&wm831x.auxadc_lock);
    kfree(req);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn wm831x_auxadc_irq(irq: c_int, irq_data: *mut c_void) -> irqreturn_t {
    static irqreturn_t wm831x_auxadc_irq(int irq, void *irq_data)
    {
    struct wm831x *wm831x = irq_data;
    struct wm831x_auxadc_req *req;
    int ret, input, val;
    ret = wm831x_reg_read(wm831x, WM831X_AUXADC_DATA);
    if (ret < 0) {
    dev_err(wm831x.dev,
    "Failed to read AUXADC data: %d\n", ret);
    return IRQ_NONE;
    }
    input = ((ret & WM831X_AUX_DATA_SRC_MASK)
    >> WM831X_AUX_DATA_SRC_SHIFT) - 1;
    if (input == 14)
    input = WM831X_AUX_CAL;
    val = ret & WM831X_AUX_DATA_MASK;
    mutex_lock(&wm831x.auxadc_lock);
// Disable this conversion, we're about to complete all users
    wm831x_set_bits(wm831x, WM831X_AUXADC_SOURCE,
    1 << input, 0);
    wm831x.auxadc_active &= ~(1 << input);
// Turn off the entire convertor if idle
    if (!wm831x.auxadc_active)
    wm831x_reg_write(wm831x, WM831X_AUXADC_CONTROL, 0);
// Wake up any threads waiting for this request
    list_for_each_entry(req, &wm831x.auxadc_pending, list) {
    if (req.input == input) {
    req.val = val;
    complete(&req.done);
    }
    }
    mutex_unlock(&wm831x.auxadc_lock);
    return IRQ_HANDLED;
    }
    static int wm831x_auxadc_read_polled(struct wm831x *wm831x,
    enum wm831x_auxadc input)
    {
    int ret, src;
    mutex_lock(&wm831x.auxadc_lock);
    ret = wm831x_set_bits(wm831x, WM831X_AUXADC_CONTROL,
    WM831X_AUX_ENA, WM831X_AUX_ENA);
    if (ret < 0) {
    dev_err(wm831x.dev, "Failed to enable AUXADC: %d\n", ret);
    goto out;
    }
// We force a single source at present
    src = input;
    ret = wm831x_reg_write(wm831x, WM831X_AUXADC_SOURCE,
    1 << src);
    if (ret < 0) {
    dev_err(wm831x.dev, "Failed to set AUXADC source: %d\n", ret);
    goto out;
    }
    ret = wm831x_set_bits(wm831x, WM831X_AUXADC_CONTROL,
    WM831X_AUX_CVT_ENA, WM831X_AUX_CVT_ENA);
    if (ret < 0) {
    dev_err(wm831x.dev, "Failed to start AUXADC: %d\n", ret);
    goto disable;
    }
// If we're not using interrupts then read the interrupt status register
    msleep(20);
    ret = wm831x_reg_read(wm831x, WM831X_INTERRUPT_STATUS_1);
    if (ret < 0) {
    dev_err(wm831x.dev,
    "ISR 1 read failed: %d\n", ret);
    goto disable;
    }
// Did it complete?
    if (ret & WM831X_AUXADC_DATA_EINT) {
    wm831x_reg_write(wm831x, WM831X_INTERRUPT_STATUS_1,
    WM831X_AUXADC_DATA_EINT);
    } else {
    dev_err(wm831x.dev,
    "AUXADC conversion timeout\n");
    ret = -EBUSY;
    goto disable;
    }
    ret = wm831x_reg_read(wm831x, WM831X_AUXADC_DATA);
    if (ret < 0) {
    dev_err(wm831x.dev,
    "Failed to read AUXADC data: %d\n", ret);
    goto disable;
    }
    src = ((ret & WM831X_AUX_DATA_SRC_MASK)
    >> WM831X_AUX_DATA_SRC_SHIFT) - 1;
    if (src == 14)
    src = WM831X_AUX_CAL;
    if (src != input) {
    dev_err(wm831x.dev, "Data from source %d not %d\n",
    src, input);
    ret = -EINVAL;
    } else {
    ret &= WM831X_AUX_DATA_MASK;
    }
    disable:
    wm831x_set_bits(wm831x, WM831X_AUXADC_CONTROL, WM831X_AUX_ENA, 0);
    out:
    mutex_unlock(&wm831x.auxadc_lock);
    return ret;
    }
//
// wm831x_auxadc_read: Read a value from the WM831x AUXADC
//
// @wm831x: Device to read from.
// @input: AUXADC input to read.
//
#[no_mangle]
pub unsafe extern "C" fn wm831x_auxadc_read(wm831x: *mut wm831x, input: enum wm831x_auxadc) -> c_int {
    int wm831x_auxadc_read(struct wm831x *wm831x, enum wm831x_auxadc input)
    {
    return wm831x.auxadc_read(wm831x, input);
    }
    EXPORT_SYMBOL_GPL(wm831x_auxadc_read);
//
// wm831x_auxadc_read_uv: Read a voltage from the WM831x AUXADC
//
// @wm831x: Device to read from.
// @input: AUXADC input to read.
//
#[no_mangle]
pub unsafe extern "C" fn wm831x_auxadc_read_uv(wm831x: *mut wm831x, input: enum wm831x_auxadc) -> c_int {
    int wm831x_auxadc_read_uv(struct wm831x *wm831x, enum wm831x_auxadc input)
    {
    int ret;
    ret = wm831x_auxadc_read(wm831x, input);
    if (ret < 0)
    return ret;
    ret *= 1465;
    return ret;
    }
    EXPORT_SYMBOL_GPL(wm831x_auxadc_read_uv);
#[no_mangle]
pub unsafe extern "C" fn wm831x_auxadc_init(wm831x: *mut wm831x) {
    void wm831x_auxadc_init(struct wm831x *wm831x)
    {
    int ret;
    mutex_init(&wm831x.auxadc_lock);
    INIT_LIST_HEAD(&wm831x.auxadc_pending);
    if (wm831x.irq) {
    wm831x.auxadc_read = wm831x_auxadc_read_irq;
    ret = request_threaded_irq(wm831x_irq(wm831x,
    WM831X_IRQ_AUXADC_DATA),
    core::ptr::null_mut(), wm831x_auxadc_irq,
    IRQF_ONESHOT,
    "auxadc", wm831x);
    if (ret < 0) {
    dev_err(wm831x.dev, "AUXADC IRQ request failed: %d\n",
    ret);
    wm831x.auxadc_read = core::ptr::null_mut();
    }
    }
    if (!wm831x.auxadc_read)
    wm831x.auxadc_read = wm831x_auxadc_read_polled;
    }
