//! Automatically rewritten from C to Rust
//! Source: drivers/media/usb/dvb-usb-v2/dvb_usb_urb.c
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
// DVB USB framework
//
// Copyright (C) 2004-6 Patrick Boettcher <patrick.boettcher@posteo.de>
// Copyright (C) 2012 Antti Palosaari <crope@iki.fi>
//

    static int dvb_usb_v2_generic_io(struct dvb_usb_device *d,
    u8 *wbuf, u16 wlen, u8 *rbuf, u16 rlen)
    {
    int ret, actual_length;
    if (!wbuf || !wlen || !d.props.generic_bulk_ctrl_endpoint ||
    !d.props.generic_bulk_ctrl_endpoint_response) {
    dev_dbg(&d.udev.dev, "%s: failed=%d\n", __func__, -EINVAL);
    return -EINVAL;
    }
    dev_dbg(&d.udev.dev, "%s: >>> %*ph\n", __func__, wlen, wbuf);
    ret = usb_bulk_msg(d.udev, usb_sndbulkpipe(d.udev,
    d.props.generic_bulk_ctrl_endpoint), wbuf, wlen,
    &actual_length, 2000);
    if (ret) {
    dev_err(&d.udev.dev, "%s: usb_bulk_msg() failed=%d\n",
    KBUILD_MODNAME, ret);
    return ret;
    }
    if (actual_length != wlen) {
    dev_err(&d.udev.dev, "%s: usb_bulk_msg() write length=%d, actual=%d\n",
    KBUILD_MODNAME, wlen, actual_length);
    return -EIO;
    }
// an answer is expected
    if (rbuf && rlen) {
    if (d.props.generic_bulk_ctrl_delay)
    usleep_range(d.props.generic_bulk_ctrl_delay,
    d.props.generic_bulk_ctrl_delay
    + 20000);
    ret = usb_bulk_msg(d.udev, usb_rcvbulkpipe(d.udev,
    d.props.generic_bulk_ctrl_endpoint_response),
    rbuf, rlen, &actual_length, 2000);
    if (ret)
    dev_err(&d.udev.dev,
    "%s: 2nd usb_bulk_msg() failed=%d\n",
    KBUILD_MODNAME, ret);
    dev_dbg(&d.udev.dev, "%s: <<< %*ph\n", __func__,
    actual_length, rbuf);
    }
    return ret;
    }
    int dvb_usbv2_generic_rw(struct dvb_usb_device *d,
    u8 *wbuf, u16 wlen, u8 *rbuf, u16 rlen)
    {
    int ret;
    mutex_lock(&d.usb_mutex);
    ret = dvb_usb_v2_generic_io(d, wbuf, wlen, rbuf, rlen);
    mutex_unlock(&d.usb_mutex);
    return ret;
    }
    EXPORT_SYMBOL(dvb_usbv2_generic_rw);
#[no_mangle]
pub unsafe extern "C" fn dvb_usbv2_generic_write(d: *mut dvb_usb_device, buf: *mut u8, len: u16) -> c_int {
    int dvb_usbv2_generic_write(struct dvb_usb_device *d, u8 *buf, u16 len)
    {
    int ret;
    mutex_lock(&d.usb_mutex);
    ret = dvb_usb_v2_generic_io(d, buf, len, core::ptr::null_mut(), 0);
    mutex_unlock(&d.usb_mutex);
    return ret;
    }
    EXPORT_SYMBOL(dvb_usbv2_generic_write);
    int dvb_usbv2_generic_rw_locked(struct dvb_usb_device *d,
    u8 *wbuf, u16 wlen, u8 *rbuf, u16 rlen)
    {
    return dvb_usb_v2_generic_io(d, wbuf, wlen, rbuf, rlen);
    }
    EXPORT_SYMBOL(dvb_usbv2_generic_rw_locked);
#[no_mangle]
pub unsafe extern "C" fn dvb_usbv2_generic_write_locked(d: *mut dvb_usb_device, buf: *mut u8, len: u16) -> c_int {
    int dvb_usbv2_generic_write_locked(struct dvb_usb_device *d, u8 *buf, u16 len)
    {
    return dvb_usb_v2_generic_io(d, buf, len, core::ptr::null_mut(), 0);
    }
    EXPORT_SYMBOL(dvb_usbv2_generic_write_locked);
