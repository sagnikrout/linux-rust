//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/tps6507x.c
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
// tps6507x.c  --  TPS6507x chip family multi-function driver
//
// Copyright (c) 2010 RidgeRun (todd.fischer@ridgerun.com)
//
// Author: Todd Fischer
// todd.fischer@ridgerun.com
//
// Credits:
//
// Using code from wm831x-*.c, wm8400-core, Wolfson Microelectronics PLC.
//
// For licencing details see kernel-base/COPYING
//

    static const struct mfd_cell tps6507x_devs[] = {
    {
    .name = "tps6507x-pmic",
    },
    {
    .name = "tps6507x-ts",
    },
    };
    static int tps6507x_i2c_read_device(struct tps6507x_dev *tps6507x, char reg,
    int bytes, void *dest)
    {
    struct i2c_client *i2c = tps6507x.i2c_client;
    struct i2c_msg xfer[2];
    int ret;
// Write register
    xfer[0].addr = i2c.addr;
    xfer[0].flags = 0;
    xfer[0].len = 1;
    xfer[0].buf = &reg;
// Read data
    xfer[1].addr = i2c.addr;
    xfer[1].flags = I2C_M_RD;
    xfer[1].len = bytes;
    xfer[1].buf = dest;
    ret = i2c_transfer(i2c.adapter, xfer, 2);
    if (ret == 2)
    ret = 0;
#[no_mangle]
pub unsafe extern "C" fn if(0: ret >=) -> else {
    else if (ret >= 0)
    ret = -EIO;
    return ret;
    }
    static int tps6507x_i2c_write_device(struct tps6507x_dev *tps6507x, char reg,
    int bytes, void *src)
    {
    struct i2c_client *i2c = tps6507x.i2c_client;
// we add 1 byte for device register
    u8 msg[TPS6507X_MAX_REGISTER + 1];
    int ret;
    if (bytes > TPS6507X_MAX_REGISTER)
    return -EINVAL;
    msg[0] = reg;
    memcpy(&msg[1], src, bytes);
    ret = i2c_master_send(i2c, msg, bytes + 1);
    if (ret < 0)
    return ret;
    if (ret != bytes + 1)
    return -EIO;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tps6507x_i2c_probe(i2c: *mut i2c_client) -> c_int {
    static int tps6507x_i2c_probe(struct i2c_client *i2c)
    {
    struct tps6507x_dev *tps6507x;
    tps6507x = devm_kzalloc(&i2c.dev, sizeof(struct tps6507x_dev),
    GFP_KERNEL);
    if (tps6507x == core::ptr::null_mut())
    return -ENOMEM;
    i2c_set_clientdata(i2c, tps6507x);
    tps6507x.dev = &i2c.dev;
    tps6507x.i2c_client = i2c;
    tps6507x.read_dev = tps6507x_i2c_read_device;
    tps6507x.write_dev = tps6507x_i2c_write_device;
    return devm_mfd_add_devices(tps6507x.dev, -1, tps6507x_devs,
    ARRAY_SIZE(tps6507x_devs), core::ptr::null_mut(), 0, core::ptr::null_mut());
    }
    static const struct i2c_device_id tps6507x_i2c_id[] = {
    { "tps6507x" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, tps6507x_i2c_id);

    static const struct of_device_id tps6507x_of_match[] = {
    {.compatible = "ti,tps6507x", },
    {},
    };
    MODULE_DEVICE_TABLE(of, tps6507x_of_match);

    static struct i2c_driver tps6507x_i2c_driver = {
    .driver = {
    .name = "tps6507x",
    .of_match_table = of_match_ptr(tps6507x_of_match),
    },
    .probe = tps6507x_i2c_probe,
    .id_table = tps6507x_i2c_id,
    };
#[no_mangle]
unsafe extern "C" fn tps6507x_i2c_init() -> int __init {
    static int __init tps6507x_i2c_init(void)
    {
    return i2c_add_driver(&tps6507x_i2c_driver);
    }
// init early so consumer devices can complete system boot
    subsys_initcall(tps6507x_i2c_init);
#[no_mangle]
unsafe extern "C" fn tps6507x_i2c_exit() -> void __exit {
    static void __exit tps6507x_i2c_exit(void)
    {
    i2c_del_driver(&tps6507x_i2c_driver);
    }
    module_exit(tps6507x_i2c_exit);
    MODULE_DESCRIPTION("TPS6507x chip family multi-function driver");
    MODULE_LICENSE("GPL");
