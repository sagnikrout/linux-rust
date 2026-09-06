//! Automatically rewritten from C to Rust
//! Source: drivers/input/misc/ad714x-i2c.c
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
// AD714X CapTouch Programmable Controller driver (I2C bus)
//
// Copyright 2009-2011 Analog Devices Inc.
//

    static int ad714x_i2c_write(struct ad714x_chip *chip,
    unsigned short reg, unsigned short data)
    {
    struct i2c_client *client = to_i2c_client(chip.dev);
    int error;
    chip.xfer_buf[0] = cpu_to_be16(reg);
    chip.xfer_buf[1] = cpu_to_be16(data);
    error = i2c_master_send(client, (u8 *)chip.xfer_buf,
    2 * sizeof(*chip.xfer_buf));
    if (unlikely(error < 0)) {
    dev_err(&client.dev, "I2C write error: %d\n", error);
    return error;
    }
    return 0;
    }
    static int ad714x_i2c_read(struct ad714x_chip *chip,
    unsigned short reg, unsigned short *data, size_t len)
    {
    struct i2c_client *client = to_i2c_client(chip.dev);
    int i;
    int error;
    chip.xfer_buf[0] = cpu_to_be16(reg);
    error = i2c_master_send(client, (u8 *)chip.xfer_buf,
    sizeof(*chip.xfer_buf));
    if (error >= 0)
    error = i2c_master_recv(client, (u8 *)chip.xfer_buf,
    len * sizeof(*chip.xfer_buf));
    if (unlikely(error < 0)) {
    dev_err(&client.dev, "I2C read error: %d\n", error);
    return error;
    }
    for (i = 0; i < len; i++)
    data[i] = be16_to_cpu(chip.xfer_buf[i]);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ad714x_i2c_probe(client: *mut i2c_client) -> c_int {
    static int ad714x_i2c_probe(struct i2c_client *client)
    {
    struct ad714x_chip *chip;
    chip = ad714x_probe(&client.dev, BUS_I2C, client.irq,
    ad714x_i2c_read, ad714x_i2c_write);
    if (IS_ERR(chip))
    return PTR_ERR(chip);
    i2c_set_clientdata(client, chip);
    return 0;
    }
    static const struct i2c_device_id ad714x_id[] = {
    { .name = "ad7142_captouch" },
    { .name = "ad7143_captouch" },
    { .name = "ad7147_captouch" },
    { .name = "ad7147a_captouch" },
    { .name = "ad7148_captouch" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, ad714x_id);
    static struct i2c_driver ad714x_i2c_driver = {
    .driver = {
    .name = "ad714x_captouch",
    .pm   = pm_sleep_ptr(&ad714x_pm),
    },
    .probe = ad714x_i2c_probe,
    .id_table = ad714x_id,
    };
    module_i2c_driver(ad714x_i2c_driver);
    MODULE_DESCRIPTION("Analog Devices AD714X Capacitance Touch Sensor I2C Bus Driver");
    MODULE_AUTHOR("Barry Song <21cnbao@gmail.com>");
    MODULE_LICENSE("GPL");
