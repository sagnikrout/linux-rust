//! Automatically rewritten from C to Rust
//! Source: drivers/iio/pressure/mprls0025pa_i2c.c
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
// MPRLS0025PA - Honeywell MicroPressure pressure sensor series driver
//
// Copyright (c) Andreas Klinger <ak@it-klinger.de>
//
// Data sheet:
// https://prod-edam.honeywell.com/content/dam/honeywell-edam/sps/siot/en-us/products/sensors/pressure-sensors/board-mount-pressure-sensors/micropressure-mpr-series/documents/sps-siot-mpr-series-datasheet-32332628-ciid-172626.pdf
//

#[no_mangle]
unsafe extern "C" fn mpr_i2c_read(data: *mut mpr_data, unused: u8, cnt: u8) -> c_int {
    static int mpr_i2c_read(struct mpr_data *data, const u8 unused, const u8 cnt)
    {
    int ret;
    struct i2c_client *client = to_i2c_client(data.dev);
    if (cnt > MPR_MEASUREMENT_RD_SIZE)
    return -EOVERFLOW;
    ret = i2c_master_recv(client, data.rx_buf, cnt);
    if (ret < 0)
    return ret;
#[no_mangle]
pub unsafe extern "C" fn if(cnt: ret !=) -> else {
    else if (ret != cnt)
    return -EIO;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mpr_i2c_write(data: *mut mpr_data, cmd: u8, unused: u8) -> c_int {
    static int mpr_i2c_write(struct mpr_data *data, const u8 cmd, const u8 unused)
    {
    int ret;
    struct i2c_client *client = to_i2c_client(data.dev);
    data.tx_buf[0] = cmd;
    ret = i2c_master_send(client, data.tx_buf, MPR_PKT_SYNC_LEN);
    if (ret < 0)
    return ret;
#[no_mangle]
pub unsafe extern "C" fn if(MPR_PKT_SYNC_LEN: ret !=) -> else {
    else if (ret != MPR_PKT_SYNC_LEN)
    return -EIO;
    return 0;
    }
    static const struct mpr_ops mpr_i2c_ops = {
    .read = mpr_i2c_read,
    .write = mpr_i2c_write,
    };
#[no_mangle]
unsafe extern "C" fn mpr_i2c_probe(client: *mut i2c_client) -> c_int {
    static int mpr_i2c_probe(struct i2c_client *client)
    {
    if (!i2c_check_functionality(client.adapter, I2C_FUNC_SMBUS_READ_BYTE))
    return -EOPNOTSUPP;
    return mpr_common_probe(&client.dev, &mpr_i2c_ops, client.irq);
    }
    static const struct of_device_id mpr_i2c_match[] = {
    { .compatible = "honeywell,mprls0025pa" },
    { }
    };
    MODULE_DEVICE_TABLE(of, mpr_i2c_match);
    static const struct i2c_device_id mpr_i2c_id[] = {
    { .name = "mprls0025pa" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, mpr_i2c_id);
    static struct i2c_driver mpr_i2c_driver = {
    .probe = mpr_i2c_probe,
    .id_table = mpr_i2c_id,
    .driver = {
    .name = "mprls0025pa",
    .of_match_table = mpr_i2c_match,
    },
    };
    module_i2c_driver(mpr_i2c_driver);
    MODULE_AUTHOR("Andreas Klinger <ak@it-klinger.de>");
    MODULE_DESCRIPTION("Honeywell MPR pressure sensor i2c driver");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("IIO_HONEYWELL_MPRLS0025PA");
