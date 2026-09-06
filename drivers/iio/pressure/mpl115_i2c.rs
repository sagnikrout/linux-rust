//! Automatically rewritten from C to Rust
//! Source: drivers/iio/pressure/mpl115_i2c.c
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
// Freescale MPL115A2 pressure/temperature sensor
//
// Copyright (c) 2014 Peter Meerwald <pmeerw@pmeerw.net>
//
// (7-bit I2C slave address 0x60)
//
// Datasheet: http://www.nxp.com/files/sensors/doc/data_sheet/MPL115A2.pdf
//

#[no_mangle]
unsafe extern "C" fn mpl115_i2c_init(dev: *mut device) -> c_int {
    static int mpl115_i2c_init(struct device *dev)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mpl115_i2c_read(dev: *mut device, address: u8) -> c_int {
    static int mpl115_i2c_read(struct device *dev, u8 address)
    {
    return i2c_smbus_read_word_swapped(to_i2c_client(dev), address);
    }
#[no_mangle]
unsafe extern "C" fn mpl115_i2c_write(dev: *mut device, address: u8, value: u8) -> c_int {
    static int mpl115_i2c_write(struct device *dev, u8 address, u8 value)
    {
    return i2c_smbus_write_byte_data(to_i2c_client(dev), address, value);
    }
    static const struct mpl115_ops mpl115_i2c_ops = {
    .init = mpl115_i2c_init,
    .read = mpl115_i2c_read,
    .write = mpl115_i2c_write,
    };
#[no_mangle]
unsafe extern "C" fn mpl115_i2c_probe(client: *mut i2c_client) -> c_int {
    static int mpl115_i2c_probe(struct i2c_client *client)
    {
    const struct i2c_device_id *id = i2c_client_get_device_id(client);
    if (!i2c_check_functionality(client.adapter, I2C_FUNC_SMBUS_WORD_DATA))
    return -EOPNOTSUPP;
    return mpl115_probe(&client.dev, id.name, &mpl115_i2c_ops);
    }
    static const struct i2c_device_id mpl115_i2c_id[] = {
    { .name = "mpl115" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, mpl115_i2c_id);
    static struct i2c_driver mpl115_i2c_driver = {
    .driver = {
    .name	= "mpl115",
    .pm = pm_ptr(&mpl115_dev_pm_ops),
    },
    .probe = mpl115_i2c_probe,
    .id_table = mpl115_i2c_id,
    };
    module_i2c_driver(mpl115_i2c_driver);
    MODULE_AUTHOR("Peter Meerwald <pmeerw@pmeerw.net>");
    MODULE_DESCRIPTION("Freescale MPL115A2 pressure/temperature driver");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("IIO_MPL115");
