//! Automatically rewritten from C to Rust
//! Source: drivers/iio/chemical/ags02ma.c
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
// Copyright (C) 2023 Anshul Dalal <anshulusr@gmail.com>
//
// Driver for Aosong AGS02MA
//
// Datasheet:
// https://asairsensors.com/wp-content/uploads/2021/09/AGS02MA.pdf
// Product Page:
// http://www.aosong.com/m/en/products-33.html
//

pub const AGS02MA_TVOC_READ_REG: c_uint = 0x00;
pub const AGS02MA_VERSION_REG: c_uint = 0x11;
pub const AGS02MA_VERSION_PROCESSING_DELAY: c_int = 30;
pub const AGS02MA_TVOC_READ_PROCESSING_DELAY: c_int = 1500;
pub const AGS02MA_CRC8_INIT: c_uint = 0xff;
pub const AGS02MA_CRC8_POLYNOMIAL: c_uint = 0x31;
    DECLARE_CRC8_TABLE(ags02ma_crc8_table);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ags02ma_data {
    pub client: *mut i2c_client,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ags02ma_reading {
    pub data: __be32,
    pub crc: u8,
    pub __packed: },
    static int ags02ma_register_read(struct i2c_client *client, u8 reg, u16 delay,
    u32 *val)
    {
    pub ret: c_int,
    pub crc: u8,
    pub read_buffer: ags02ma_reading,
    pub sizeof(reg)): ret = i2c_master_send(client, &reg,,
    if (ret < 0) {
    dev_err(&client.dev,
    pub ret): "Failed to send data to register 0x%x: %d", reg,,
    pub ret: return,
    }
// Processing Delay, Check Table 7.7 in the datasheet
    pub sizeof(read_buffer)): *mut *mut ret = i2c_master_recv(client, (u8 )&read_buffer,,
    if (ret < 0) {
    dev_err(&client.dev,
    pub ret): "Failed to receive from register 0x%x: %d", reg,,
    pub ret: return,
    }
    crc = crc8(ags02ma_crc8_table, (u8 *)&read_buffer.data,
    pub AGS02MA_CRC8_INIT): sizeof(read_buffer.data),,
    if (crc != read_buffer.crc) {
    pub error\n"): dev_err(&client->dev, "CRC,
    pub -EIO: return,
    }
// val = be32_to_cpu(read_buffer.data);
    pub 0: return,
    }
    static int ags02ma_read_raw(struct iio_dev *iio_device,
    struct iio_chan_spec const *chan, int *val,
    int *val2, long mask)
    {
    pub ret: c_int,
    pub iio_priv(iio_device): *mut *mut ags02ma_data data =,
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    ret = ags02ma_register_read(data.client, AGS02MA_TVOC_READ_REG,
    AGS02MA_TVOC_READ_PROCESSING_DELAY,
    if (ret < 0)
    pub ret: return,
    pub IIO_VAL_INT: return,
    case IIO_CHAN_INFO_SCALE:
// The sensor reads data as ppb
// val = 0;
// val2 = 100;
    pub IIO_VAL_INT_PLUS_NANO: return,
    default:
    pub -EINVAL: return,
    }
    }
    static const struct iio_info ags02ma_info = {
    .read_raw = ags02ma_read_raw,
}

    static const struct iio_chan_spec ags02ma_channel = {
    .type = IIO_CONCENTRATION,
    .channel2 = IIO_MOD_VOC,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |
    BIT(IIO_CHAN_INFO_SCALE),
    };
#[no_mangle]
unsafe extern "C" fn ags02ma_probe(client: *mut i2c_client) -> c_int {
    static int ags02ma_probe(struct i2c_client *client)
    {
    int ret;
    struct ags02ma_data *data;
    struct iio_dev *indio_dev;
    u32 version;
    indio_dev = devm_iio_device_alloc(&client.dev, sizeof(*data));
    if (!indio_dev)
    return -ENOMEM;
    crc8_populate_msb(ags02ma_crc8_table, AGS02MA_CRC8_POLYNOMIAL);
    ret = ags02ma_register_read(client, AGS02MA_VERSION_REG,
    AGS02MA_VERSION_PROCESSING_DELAY, &version);
    if (ret < 0)
    return dev_err_probe(&client.dev, ret,
    "Failed to read device version\n");
    dev_dbg(&client.dev, "Aosong AGS02MA, Version: 0x%x", version);
    data = iio_priv(indio_dev);
    data.client = client;
    indio_dev.info = &ags02ma_info;
    indio_dev.channels = &ags02ma_channel;
    indio_dev.num_channels = 1;
    indio_dev.name = "ags02ma";
    return devm_iio_device_register(&client.dev, indio_dev);
    }
    static const struct i2c_device_id ags02ma_id_table[] = {
    { .name = "ags02ma" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, ags02ma_id_table);
    static const struct of_device_id ags02ma_of_table[] = {
    { .compatible = "aosong,ags02ma" },
    { }
    };
    MODULE_DEVICE_TABLE(of, ags02ma_of_table);
    static struct i2c_driver ags02ma_driver = {
    .driver = {
    .name = "ags02ma",
    .of_match_table = ags02ma_of_table,
    },
    .id_table = ags02ma_id_table,
    .probe = ags02ma_probe,
    };
    module_i2c_driver(ags02ma_driver);
    MODULE_AUTHOR("Anshul Dalal <anshulusr@gmail.com>");
    MODULE_DESCRIPTION("Aosong AGS02MA TVOC Driver");
    MODULE_LICENSE("GPL");
