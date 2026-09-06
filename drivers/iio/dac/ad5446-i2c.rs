//! Automatically rewritten from C to Rust
//! Source: drivers/iio/dac/ad5446-i2c.c
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


// SPDX-License-Identifier: GPL-2.0
//
// AD5446 SPI I2C driver
//
// Copyright 2025 Analog Devices Inc.
//

#[no_mangle]
unsafe extern "C" fn ad5622_write(st: *mut ad5446_state, val: c_uint) -> c_int {
    static int ad5622_write(struct ad5446_state *st, unsigned int val)
    {
    struct i2c_client *client = to_i2c_client(st.dev);
    int ret;
    st.d16 = cpu_to_be16(val);
    ret = i2c_master_send_dmasafe(client, (char *)&st.d16, sizeof(st.d16));
    if (ret < 0)
    return ret;
    if (ret != sizeof(st.d16))
    return -EIO;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ad5446_i2c_probe(i2c: *mut i2c_client) -> c_int {
    static int ad5446_i2c_probe(struct i2c_client *i2c)
    {
    const struct i2c_device_id *id = i2c_client_get_device_id(i2c);
    const struct ad5446_chip_info *chip_info;
    chip_info = i2c_get_match_data(i2c);
    if (!chip_info)
    return -ENODEV;
    return ad5446_probe(&i2c.dev, id.name, chip_info);
    }
//
// ad5446_supported_i2c_device_ids:
// The AD5620/40/60 parts are available in different fixed internal reference
// voltage options. The actual part numbers may look differently
// (and a bit cryptic), however this style is used to make clear which
// parts are supported here.
//
    static const struct ad5446_chip_info ad5602_chip_info = {
    .channel = AD5446_CHANNEL_POWERDOWN(8, 16, 4),
    .write = ad5622_write,
    };
    static const struct ad5446_chip_info ad5612_chip_info = {
    .channel = AD5446_CHANNEL_POWERDOWN(10, 16, 2),
    .write = ad5622_write,
    };
    static const struct ad5446_chip_info ad5622_chip_info = {
    .channel = AD5446_CHANNEL_POWERDOWN(12, 16, 0),
    .write = ad5622_write,
    };
    static const struct i2c_device_id ad5446_i2c_ids[] = {
    { .name = "ad5301", .driver_data = (kernel_ulong_t)&ad5602_chip_info },
    { .name = "ad5311", .driver_data = (kernel_ulong_t)&ad5612_chip_info },
    { .name = "ad5321", .driver_data = (kernel_ulong_t)&ad5622_chip_info },
    { .name = "ad5602", .driver_data = (kernel_ulong_t)&ad5602_chip_info },
    { .name = "ad5612", .driver_data = (kernel_ulong_t)&ad5612_chip_info },
    { .name = "ad5622", .driver_data = (kernel_ulong_t)&ad5622_chip_info },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, ad5446_i2c_ids);
    static const struct of_device_id ad5446_i2c_of_ids[] = {
    { .compatible = "adi,ad5301", .data = &ad5602_chip_info },
    { .compatible = "adi,ad5311", .data = &ad5612_chip_info },
    { .compatible = "adi,ad5321", .data = &ad5622_chip_info },
    { .compatible = "adi,ad5602", .data = &ad5602_chip_info },
    { .compatible = "adi,ad5612", .data = &ad5612_chip_info },
    { .compatible = "adi,ad5622", .data = &ad5622_chip_info },
    { }
    };
    MODULE_DEVICE_TABLE(of, ad5446_i2c_of_ids);
    static struct i2c_driver ad5446_i2c_driver = {
    .driver = {
    .name	= "ad5446",
    .of_match_table = ad5446_i2c_of_ids,
    },
    .probe = ad5446_i2c_probe,
    .id_table = ad5446_i2c_ids,
    };
    module_i2c_driver(ad5446_i2c_driver);
    MODULE_AUTHOR("Nuno Sá <nuno.sa@analog.com>");
    MODULE_DESCRIPTION("Analog Devices AD5622 and similar I2C DACs");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("IIO_AD5446");
