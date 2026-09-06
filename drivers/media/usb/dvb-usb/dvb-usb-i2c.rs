//! Automatically rewritten from C to Rust
//! Source: drivers/media/usb/dvb-usb/dvb-usb-i2c.c
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
// dvb-usb-i2c.c is part of the DVB USB library.
//
// Copyright (C) 2004-6 Patrick Boettcher (patrick.boettcher@posteo.de)
// see dvb-usb-init.c for copyright information.
//
// This file contains functions for (de-)initializing an I2C adapter.
//

#[no_mangle]
pub unsafe extern "C" fn dvb_usb_i2c_init(d: *mut dvb_usb_device) -> c_int {
    int dvb_usb_i2c_init(struct dvb_usb_device *d)
    {
    let mut ret: c_int = 0;
    if (!(d.props.caps & DVB_USB_IS_AN_I2C_ADAPTER))
    return 0;
    if (d.props.i2c_algo == core::ptr::null_mut()) {
    err("no i2c algorithm specified");
    ret = -EINVAL;
    goto err;
    }
    strscpy(d.i2c_adap.name, d.desc.name, sizeof(d.i2c_adap.name));
    d.i2c_adap.algo      = d.props.i2c_algo;
    d.i2c_adap.algo_data = core::ptr::null_mut();
    d.i2c_adap.dev.parent = &d.udev.dev;
    i2c_set_adapdata(&d.i2c_adap, d);
    ret = i2c_add_adapter(&d.i2c_adap);
    if (ret < 0) {
    err("could not add i2c adapter");
    goto err;
    }
    d.state |= DVB_USB_STATE_I2C;
    err:
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn dvb_usb_i2c_exit(d: *mut dvb_usb_device) -> c_int {
    int dvb_usb_i2c_exit(struct dvb_usb_device *d)
    {
    if (d.state & DVB_USB_STATE_I2C)
    i2c_del_adapter(&d.i2c_adap);
    d.state &= ~DVB_USB_STATE_I2C;
    return 0;
    }
