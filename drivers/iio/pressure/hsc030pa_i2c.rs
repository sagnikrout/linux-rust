//! Automatically rewritten from C to Rust
//! Source: drivers/iio/pressure/hsc030pa_i2c.c
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
// Honeywell TruStability HSC Series pressure/temperature sensor
//
// Copyright (c) 2023 Petre Rodan <petre.rodan@subdimension.ro>
//
// Datasheet: https://prod-edam.honeywell.com/content/dam/honeywell-edam/sps/siot/en-us/products/sensors/pressure-sensors/board-mount-pressure-sensors/common/documents/sps-siot-i2c-comms-digital-output-pressure-sensors-tn-008201-3-en-ciid-45841.pdf
// Datasheet: https://prod-edam.honeywell.com/content/dam/honeywell-edam/sps/siot/en-us/products/sensors/pressure-sensors/common/documents/sps-siot-sleep-mode-technical-note-008286-1-en-ciid-155793.pdf
//

#[no_mangle]
unsafe extern "C" fn hsc_i2c_recv(data: *mut hsc_data) -> c_int {
    static int hsc_i2c_recv(struct hsc_data *data)
    {
    struct i2c_client *client = to_i2c_client(data.dev);
    struct i2c_msg msg;
    int ret;
    msleep_interruptible(HSC_RESP_TIME_MS);
    msg.addr = client.addr;
    msg.flags = client.flags | I2C_M_RD;
    msg.len = HSC_REG_MEASUREMENT_RD_SIZE;
    msg.buf = data.buffer;
    ret = i2c_transfer(client.adapter, &msg, 1);
    if (ret < 0)
    return ret;
    if (ret != 1)
    return -EIO;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hsc_i2c_probe(client: *mut i2c_client) -> c_int {
    static int hsc_i2c_probe(struct i2c_client *client)
    {
    if (!i2c_check_functionality(client.adapter, I2C_FUNC_I2C))
    return -EOPNOTSUPP;
    return hsc_common_probe(&client.dev, hsc_i2c_recv);
    }
    static const struct of_device_id hsc_i2c_match[] = {
    { .compatible = "honeywell,hsc030pa" },
    { }
    };
    MODULE_DEVICE_TABLE(of, hsc_i2c_match);
    static const struct i2c_device_id hsc_i2c_id[] = {
    { .name = "hsc030pa" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, hsc_i2c_id);
    static struct i2c_driver hsc_i2c_driver = {
    .driver = {
    .name = "hsc030pa",
    .of_match_table = hsc_i2c_match,
    },
    .probe = hsc_i2c_probe,
    .id_table = hsc_i2c_id,
    };
    module_i2c_driver(hsc_i2c_driver);
    MODULE_AUTHOR("Petre Rodan <petre.rodan@subdimension.ro>");
    MODULE_DESCRIPTION("Honeywell HSC and SSC pressure sensor i2c driver");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("IIO_HONEYWELL_HSC030PA");
