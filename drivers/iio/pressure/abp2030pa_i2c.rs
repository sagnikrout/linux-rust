//! Automatically rewritten from C to Rust
//! Source: drivers/iio/pressure/abp2030pa_i2c.c
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
// Honeywell ABP2 series pressure sensor driver
//
// Copyright (c) 2025 Petre Rodan <petre.rodan@subdimension.ro>
//

#[no_mangle]
unsafe extern "C" fn abp2_i2c_read(data: *mut abp2_data, unused: u8, nbytes: u8) -> c_int {
    static int abp2_i2c_read(struct abp2_data *data, u8 unused, u8 nbytes)
    {
    struct i2c_client *client = to_i2c_client(data.dev);
    int ret;
    if (nbytes > ABP2_MEASUREMENT_RD_SIZE)
    return -EOVERFLOW;
    ret = i2c_master_recv(client, data.rx_buf, nbytes);
    if (ret < 0)
    return ret;
    if (ret != nbytes)
    return -EIO;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn abp2_i2c_write(data: *mut abp2_data, cmd: u8, nbytes: u8) -> c_int {
    static int abp2_i2c_write(struct abp2_data *data, u8 cmd, u8 nbytes)
    {
    struct i2c_client *client = to_i2c_client(data.dev);
    int ret;
    if (nbytes > ABP2_MEASUREMENT_RD_SIZE)
    return -EOVERFLOW;
    data.tx_buf[0] = cmd;
    ret = i2c_master_send(client, data.tx_buf, nbytes);
    if (ret < 0)
    return ret;
    if (ret != nbytes)
    return -EIO;
    return 0;
    }
    static const struct abp2_ops abp2_i2c_ops = {
    .read = abp2_i2c_read,
    .write = abp2_i2c_write,
    };
#[no_mangle]
unsafe extern "C" fn abp2_i2c_probe(client: *mut i2c_client) -> c_int {
    static int abp2_i2c_probe(struct i2c_client *client)
    {
    if (!i2c_check_functionality(client.adapter, I2C_FUNC_I2C))
    return -EOPNOTSUPP;
    return abp2_common_probe(&client.dev, &abp2_i2c_ops, client.irq);
    }
    static const struct of_device_id abp2_i2c_match[] = {
    { .compatible = "honeywell,abp2030pa" },
    { }
    };
    MODULE_DEVICE_TABLE(of, abp2_i2c_match);
    static const struct i2c_device_id abp2_i2c_id[] = {
    { .name = "abp2030pa" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, abp2_i2c_id);
    static struct i2c_driver abp2_i2c_driver = {
    .driver = {
    .name = "abp2030pa",
    .of_match_table = abp2_i2c_match,
    },
    .probe = abp2_i2c_probe,
    .id_table = abp2_i2c_id,
    };
    module_i2c_driver(abp2_i2c_driver);
    MODULE_AUTHOR("Petre Rodan <petre.rodan@subdimension.ro>");
    MODULE_DESCRIPTION("Honeywell ABP2 pressure sensor i2c driver");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("IIO_HONEYWELL_ABP2030PA");
