//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/lm3533-ctrlbank.c
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
// lm3533-ctrlbank.c -- LM3533 Generic Control Bank interface
//
// Copyright (C) 2011-2012 Texas Instruments
//
// Author: Johan Hovold <jhovold@gmail.com>
//

pub const LM3533_MAX_CURRENT_MIN: c_int = 5000;
pub const LM3533_MAX_CURRENT_MAX: c_int = 29800;
pub const LM3533_MAX_CURRENT_STEP: c_int = 800;
pub const LM3533_PWM_MAX: c_uint = 0x3f;
pub const LM3533_REG_PWM_BASE: c_uint = 0x14;
pub const LM3533_REG_MAX_CURRENT_BASE: c_uint = 0x1f;
pub const LM3533_REG_CTRLBANK_ENABLE: c_uint = 0x27;
pub const LM3533_REG_BRIGHTNESS_BASE: c_uint = 0x40;
#[no_mangle]
pub unsafe extern "C" fn lm3533_ctrlbank_get_reg(cb: *mut lm3533_ctrlbank, base: u8) -> u8 {
    static inline u8 lm3533_ctrlbank_get_reg(struct lm3533_ctrlbank *cb, u8 base)
    {
    return base + cb.id;
    }
#[no_mangle]
pub unsafe extern "C" fn lm3533_ctrlbank_enable(cb: *mut lm3533_ctrlbank) -> c_int {
    int lm3533_ctrlbank_enable(struct lm3533_ctrlbank *cb)
    {
    u8 mask;
    int ret;
    dev_dbg(cb.dev, "%s - %d\n", __func__, cb.id);
    mask = 1 << cb.id;
    ret = lm3533_update(cb.lm3533, LM3533_REG_CTRLBANK_ENABLE,
    mask, mask);
    if (ret)
    dev_err(cb.dev, "failed to enable ctrlbank %d\n", cb.id);
    return ret;
    }
    EXPORT_SYMBOL_GPL(lm3533_ctrlbank_enable);
#[no_mangle]
pub unsafe extern "C" fn lm3533_ctrlbank_disable(cb: *mut lm3533_ctrlbank) -> c_int {
    int lm3533_ctrlbank_disable(struct lm3533_ctrlbank *cb)
    {
    u8 mask;
    int ret;
    dev_dbg(cb.dev, "%s - %d\n", __func__, cb.id);
    mask = 1 << cb.id;
    ret = lm3533_update(cb.lm3533, LM3533_REG_CTRLBANK_ENABLE, 0, mask);
    if (ret)
    dev_err(cb.dev, "failed to disable ctrlbank %d\n", cb.id);
    return ret;
    }
    EXPORT_SYMBOL_GPL(lm3533_ctrlbank_disable);
//
// Full-scale current.
//
// imax		5000 - 29800 uA (800 uA step)
//
#[no_mangle]
pub unsafe extern "C" fn lm3533_ctrlbank_set_max_current(cb: *mut lm3533_ctrlbank, imax: u16) -> c_int {
    int lm3533_ctrlbank_set_max_current(struct lm3533_ctrlbank *cb, u16 imax)
    {
    u8 reg;
    u8 val;
    int ret;
    if (imax < LM3533_MAX_CURRENT_MIN || imax > LM3533_MAX_CURRENT_MAX)
    return -EINVAL;
    val = (imax - LM3533_MAX_CURRENT_MIN) / LM3533_MAX_CURRENT_STEP;
    reg = lm3533_ctrlbank_get_reg(cb, LM3533_REG_MAX_CURRENT_BASE);
    ret = lm3533_write(cb.lm3533, reg, val);
    if (ret)
    dev_err(cb.dev, "failed to set max current\n");
    return ret;
    }
    EXPORT_SYMBOL_GPL(lm3533_ctrlbank_set_max_current);
#[no_mangle]
pub unsafe extern "C" fn lm3533_ctrlbank_set_brightness(cb: *mut lm3533_ctrlbank, val: u8) -> c_int {
    int lm3533_ctrlbank_set_brightness(struct lm3533_ctrlbank *cb, u8 val)
    {
    u8 reg;
    int ret;
    reg = lm3533_ctrlbank_get_reg(cb, LM3533_REG_BRIGHTNESS_BASE);
    ret = lm3533_write(cb.lm3533, reg, val);
    if (ret)
    dev_err(cb.dev, "failed to set brightness\n");
    return ret;
    }
    EXPORT_SYMBOL_GPL(lm3533_ctrlbank_set_brightness);
#[no_mangle]
pub unsafe extern "C" fn lm3533_ctrlbank_get_brightness(cb: *mut lm3533_ctrlbank, val: *mut u8) -> c_int {
    int lm3533_ctrlbank_get_brightness(struct lm3533_ctrlbank *cb, u8 *val)
    {
    u8 reg;
    int ret;
    reg = lm3533_ctrlbank_get_reg(cb, LM3533_REG_BRIGHTNESS_BASE);
    ret = lm3533_read(cb.lm3533, reg, val);
    if (ret)
    dev_err(cb.dev, "failed to get brightness\n");
    return ret;
    }
    EXPORT_SYMBOL_GPL(lm3533_ctrlbank_get_brightness);
//
// PWM-input control mask:
//
// bit 5 - PWM-input enabled in Zone 4
// bit 4 - PWM-input enabled in Zone 3
// bit 3 - PWM-input enabled in Zone 2
// bit 2 - PWM-input enabled in Zone 1
// bit 1 - PWM-input enabled in Zone 0
// bit 0 - PWM-input enabled
//
#[no_mangle]
pub unsafe extern "C" fn lm3533_ctrlbank_set_pwm(cb: *mut lm3533_ctrlbank, val: u8) -> c_int {
    int lm3533_ctrlbank_set_pwm(struct lm3533_ctrlbank *cb, u8 val)
    {
    u8 reg;
    int ret;
    if (val > LM3533_PWM_MAX)
    return -EINVAL;
    reg = lm3533_ctrlbank_get_reg(cb, LM3533_REG_PWM_BASE);
    ret = lm3533_write(cb.lm3533, reg, val);
    if (ret)
    dev_err(cb.dev, "failed to set PWM mask\n");
    return ret;
    }
    EXPORT_SYMBOL_GPL(lm3533_ctrlbank_set_pwm);
#[no_mangle]
pub unsafe extern "C" fn lm3533_ctrlbank_get_pwm(cb: *mut lm3533_ctrlbank, val: *mut u8) -> c_int {
    int lm3533_ctrlbank_get_pwm(struct lm3533_ctrlbank *cb, u8 *val)
    {
    u8 reg;
    int ret;
    reg = lm3533_ctrlbank_get_reg(cb, LM3533_REG_PWM_BASE);
    ret = lm3533_read(cb.lm3533, reg, val);
    if (ret)
    dev_err(cb.dev, "failed to get PWM mask\n");
    return ret;
    }
    EXPORT_SYMBOL_GPL(lm3533_ctrlbank_get_pwm);
    MODULE_AUTHOR("Johan Hovold <jhovold@gmail.com>");
    MODULE_DESCRIPTION("LM3533 Control Bank interface");
    MODULE_LICENSE("GPL");
