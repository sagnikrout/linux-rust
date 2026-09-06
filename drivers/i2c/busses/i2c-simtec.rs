//! Automatically rewritten from C to Rust
//! Source: drivers/i2c/busses/i2c-simtec.c
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
// Copyright (C) 2005 Simtec Electronics
// Ben Dooks <ben@simtec.co.uk>
//
// Simtec Generic I2C Controller
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct simtec_i2c_data {
    pub ioarea: *mut resource,
    pub reg: *mut void __iomem,
    pub adap: i2c_adapter,
    pub bit: i2c_algo_bit_data,
}

// i2c bit-bus functions
#[no_mangle]
unsafe extern "C" fn simtec_i2c_setsda(pw: *mut c_void, state: c_int) {
    static void simtec_i2c_setsda(void *pw, int state)
    {
    struct simtec_i2c_data *pd = pw;
    writeb(CMD_SET_SDA | (state ? STATE_SDA : 0), pd.reg);
    }
#[no_mangle]
unsafe extern "C" fn simtec_i2c_setscl(pw: *mut c_void, state: c_int) {
    static void simtec_i2c_setscl(void *pw, int state)
    {
    struct simtec_i2c_data *pd = pw;
    writeb(CMD_SET_SCL | (state ? STATE_SCL : 0), pd.reg);
    }
#[no_mangle]
unsafe extern "C" fn simtec_i2c_getsda(pw: *mut c_void) -> c_int {
    static int simtec_i2c_getsda(void *pw)
    {
    struct simtec_i2c_data *pd = pw;
    return readb(pd.reg) & STATE_SDA ? 1 : 0;
    }
#[no_mangle]
unsafe extern "C" fn simtec_i2c_getscl(pw: *mut c_void) -> c_int {
    static int simtec_i2c_getscl(void *pw)
    {
    struct simtec_i2c_data *pd = pw;
    return readb(pd.reg) & STATE_SCL ? 1 : 0;
    }
// device registration
#[no_mangle]
unsafe extern "C" fn simtec_i2c_probe(dev: *mut platform_device) -> c_int {
    static int simtec_i2c_probe(struct platform_device *dev)
    {
    struct simtec_i2c_data *pd;
    struct resource *res;
    int size;
    int ret;
    pd = kzalloc_obj(struct simtec_i2c_data);
    if (pd == core::ptr::null_mut())
    return -ENOMEM;
    platform_set_drvdata(dev, pd);
    res = platform_get_resource(dev, IORESOURCE_MEM, 0);
    if (res == core::ptr::null_mut()) {
    dev_err(&dev.dev, "cannot find IO resource\n");
    ret = -ENOENT;
    goto err;
    }
    size = resource_size(res);
    pd.ioarea = request_mem_region(res.start, size, dev.name);
    if (pd.ioarea == core::ptr::null_mut()) {
    dev_err(&dev.dev, "cannot request IO\n");
    ret = -ENXIO;
    goto err;
    }
    pd.reg = ioremap(res.start, size);
    if (pd.reg == core::ptr::null_mut()) {
    dev_err(&dev.dev, "cannot map IO\n");
    ret = -ENXIO;
    goto err_res;
    }
// setup the private data
    pd.adap.owner = THIS_MODULE;
    pd.adap.algo_data = &pd.bit;
    pd.adap.dev.parent = &dev.dev;
    strscpy(pd.adap.name, "Simtec I2C", sizeof(pd.adap.name));
    pd.bit.data = pd;
    pd.bit.setsda = simtec_i2c_setsda;
    pd.bit.setscl = simtec_i2c_setscl;
    pd.bit.getsda = simtec_i2c_getsda;
    pd.bit.getscl = simtec_i2c_getscl;
    pd.bit.timeout = HZ;
    pd.bit.udelay = 20;
    ret = i2c_bit_add_bus(&pd.adap);
    if (ret)
    goto err_all;
    return 0;
    err_all:
    iounmap(pd.reg);
    err_res:
    release_mem_region(pd.ioarea.start, size);
    err:
    kfree(pd);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn simtec_i2c_remove(dev: *mut platform_device) {
    static void simtec_i2c_remove(struct platform_device *dev)
    {
    struct simtec_i2c_data *pd = platform_get_drvdata(dev);
    i2c_del_adapter(&pd.adap);
    iounmap(pd.reg);
    release_mem_region(pd.ioarea.start, resource_size(pd.ioarea));
    kfree(pd);
    }
// device driver
    static struct platform_driver simtec_i2c_driver = {
    .driver		= {
    .name		= "simtec-i2c",
    },
    .probe		= simtec_i2c_probe,
    .remove		= simtec_i2c_remove,
    };
    module_platform_driver(simtec_i2c_driver);
    MODULE_DESCRIPTION("Simtec Generic I2C Bus driver");
    MODULE_AUTHOR("Ben Dooks <ben@simtec.co.uk>");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:simtec-i2c");
