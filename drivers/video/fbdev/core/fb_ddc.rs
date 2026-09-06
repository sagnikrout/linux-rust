//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/core/fb_ddc.c
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


//
// drivers/video/fb_ddc.c - DDC/EDID read support.
//
// Copyright (C) 2006 Dennis Munsie <dmunsie@cecropia.com>
//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file COPYING in the main directory of this archive
// for more details.
//

pub const DDC_ADDR: c_uint = 0x50;
    static unsigned char *fb_do_probe_ddc_edid(struct i2c_adapter *adapter)
    {
    let mut start: c_uchar = 0x0;
    unsigned char *buf = kmalloc(EDID_LENGTH, GFP_KERNEL);
    struct i2c_msg msgs[] = {
    {
    .addr	= DDC_ADDR,
    .flags	= 0,
    .len	= 1,
    .buf	= &start,
    }, {
    .addr	= DDC_ADDR,
    .flags	= I2C_M_RD,
    .len	= EDID_LENGTH,
    .buf	= buf,
    }
    };
    if (!buf) {
    dev_warn(&adapter.dev, "unable to allocate memory for EDID "
    "block.\n");
    return core::ptr::null_mut();
    }
    if (i2c_transfer(adapter, msgs, 2) == 2)
    return buf;
    dev_warn(&adapter.dev, "unable to read EDID block.\n");
    kfree(buf);
    return core::ptr::null_mut();
    }
    unsigned char *fb_ddc_read(struct i2c_adapter *adapter)
    {
    struct i2c_algo_bit_data *algo_data = adapter.algo_data;
    unsigned char *edid = core::ptr::null_mut();
    int i, j;
    algo_data.setscl(algo_data.data, 1);
    for (i = 0; i < 3; i++) {
// For some old monitors we need the
// following process to initialize/stop DDC
//
    algo_data.setsda(algo_data.data, 1);
    msleep(13);
    algo_data.setscl(algo_data.data, 1);
    if (algo_data.getscl) {
    for (j = 0; j < 5; j++) {
    msleep(10);
    if (algo_data.getscl(algo_data.data))
    break;
    }
    if (j == 5)
    continue;
    } else {
    udelay(algo_data.udelay);
    }
    algo_data.setsda(algo_data.data, 0);
    msleep(15);
    algo_data.setscl(algo_data.data, 0);
    msleep(15);
    algo_data.setsda(algo_data.data, 1);
    msleep(15);
// Do the real work
    edid = fb_do_probe_ddc_edid(adapter);
    algo_data.setsda(algo_data.data, 0);
    algo_data.setscl(algo_data.data, 0);
    msleep(15);
    algo_data.setscl(algo_data.data, 1);
    if (algo_data.getscl) {
    for (j = 0; j < 10; j++) {
    msleep(10);
    if (algo_data.getscl(algo_data.data))
    break;
    }
    } else {
    udelay(algo_data.udelay);
    }
    algo_data.setsda(algo_data.data, 1);
    msleep(15);
    algo_data.setscl(algo_data.data, 0);
    algo_data.setsda(algo_data.data, 0);
    if (edid)
    break;
    }
// Release the DDC lines when done or the Apple Cinema HD display
// will switch off
//
    algo_data.setsda(algo_data.data, 1);
    algo_data.setscl(algo_data.data, 1);
    return edid;
    }
    EXPORT_SYMBOL_GPL(fb_ddc_read);
    MODULE_AUTHOR("Dennis Munsie <dmunsie@cecropia.com>");
    MODULE_DESCRIPTION("DDC/EDID reading support");
    MODULE_LICENSE("GPL");
