//! Automatically rewritten from C to Rust
//! Source: drivers/media/pci/saa7164/saa7164-i2c.c
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
// Driver for the NXP SAA7164 PCIe bridge
//
// Copyright (c) 2010-2015 Steven Toth <stoth@kernellabs.com>
//

#[no_mangle]
unsafe extern "C" fn i2c_xfer(i2c_adap: *mut i2c_adapter, msgs: *mut i2c_msg, num: c_int) -> c_int {
    static int i2c_xfer(struct i2c_adapter *i2c_adap, struct i2c_msg *msgs, int num)
    {
    struct saa7164_i2c *bus = i2c_adap.algo_data;
    struct saa7164_dev *dev = bus.dev;
    int i, retval = 0;
    dprintk(DBGLVL_I2C, "%s(num = %d)\n", __func__, num);
    for (i = 0 ; i < num; i++) {
    dprintk(DBGLVL_I2C, "%s(num = %d) addr = 0x%02x  len = 0x%x\n",
    __func__, num, msgs[i].addr, msgs[i].len);
    if (msgs[i].flags & I2C_M_RD) {
    retval = saa7164_api_i2c_read(bus,
    msgs[i].addr,
    0 /* reglen */,
    core::ptr::null_mut() /* reg */, msgs[i].len, msgs[i].buf);
    } else if (i + 1 < num && (msgs[i + 1].flags & I2C_M_RD) &&
    msgs[i].addr == msgs[i + 1].addr) {
// write then read from same address
    retval = saa7164_api_i2c_read(bus, msgs[i].addr,
    msgs[i].len, msgs[i].buf,
    msgs[i+1].len, msgs[i+1].buf
    );
    i++;
    if (retval < 0)
    goto err;
    } else {
// write
    retval = saa7164_api_i2c_write(bus, msgs[i].addr,
    msgs[i].len, msgs[i].buf);
    }
    if (retval < 0)
    goto err;
    }
    return num;
    err:
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn saa7164_functionality(adap: *mut i2c_adapter) -> u32 {
    static u32 saa7164_functionality(struct i2c_adapter *adap)
    {
    return I2C_FUNC_I2C;
    }
    static const struct i2c_algorithm saa7164_i2c_algo_template = {
    .master_xfer	= i2c_xfer,
    .functionality	= saa7164_functionality,
    };
// -----------------------------------------------------------------------
    static const struct i2c_adapter saa7164_i2c_adap_template = {
    .name              = "saa7164",
    .owner             = THIS_MODULE,
    .algo              = &saa7164_i2c_algo_template,
    };
    static const struct i2c_client saa7164_i2c_client_template = {
    .name	= "saa7164 internal",
    };
#[no_mangle]
pub unsafe extern "C" fn saa7164_i2c_register(bus: *mut saa7164_i2c) -> c_int {
    int saa7164_i2c_register(struct saa7164_i2c *bus)
    {
    struct saa7164_dev *dev = bus.dev;
    dprintk(DBGLVL_I2C, "%s(bus = %d)\n", __func__, bus.nr);
    bus.i2c_adap = saa7164_i2c_adap_template;
    bus.i2c_client = saa7164_i2c_client_template;
    bus.i2c_adap.dev.parent = &dev.pci.dev;
    strscpy(bus.i2c_adap.name, bus.dev.name,
    sizeof(bus.i2c_adap.name));
    bus.i2c_adap.algo_data = bus;
    i2c_set_adapdata(&bus.i2c_adap, bus);
    i2c_add_adapter(&bus.i2c_adap);
    bus.i2c_client.adapter = &bus.i2c_adap;
    if (0 != bus.i2c_rc)
    printk(KERN_ERR "%s: i2c bus %d register FAILED\n",
    dev.name, bus.nr);
    return bus.i2c_rc;
    }
#[no_mangle]
pub unsafe extern "C" fn saa7164_i2c_unregister(bus: *mut saa7164_i2c) -> c_int {
    int saa7164_i2c_unregister(struct saa7164_i2c *bus)
    {
    i2c_del_adapter(&bus.i2c_adap);
    return 0;
    }
