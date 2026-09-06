//! Automatically rewritten from C to Rust
//! Source: drivers/iio/imu/smi330/smi330_i2c.c
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


// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
//
// Copyright (c) 2025 Robert Bosch GmbH.
//

pub const SMI330_NUM_DUMMY_BYTES: c_int = 2;

    (SMI330_NUM_DUMMY_BYTES + SMI330_SCAN_LEN * sizeof(s16))
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smi330_i2c_priv {
    pub i2c: *mut i2c_client,
    pub rx_buffer: [u8; SMI330_I2C_MAX_RX_BUFFER_SIZE],
}

    static int smi330_regmap_i2c_read(void *context, const void *reg_buf,
    size_t reg_size, void *val_buf,
    size_t val_size)
    {
    struct smi330_i2c_priv *priv = context;
    int ret;
    if (SMI330_NUM_DUMMY_BYTES + val_size > SMI330_I2C_MAX_RX_BUFFER_SIZE)
    return -EINVAL;
//
// SMI330 I2C read frame:
// <Slave address[6:0], RnW> <x, Register address[6:0]>
// <Slave address[6:0], RnW> <Dummy[7:0]> <Dummy[7:0]> <Data_0[7:0]> <Data_1[15:8]>...
// <Data_N[7:0]> <Data_N[15:8]>
// Remark: Slave address is not considered part of the frame in the following definitions
//
    struct i2c_msg msgs[] = {
    {
    .addr = priv.i2c.addr,
    .flags = priv.i2c.flags,
    .len = reg_size,
    .buf = (u8 *)reg_buf,
    },
    {
    .addr = priv.i2c.addr,
    .flags = priv.i2c.flags | I2C_M_RD,
    .len = SMI330_NUM_DUMMY_BYTES + val_size,
    .buf = priv.rx_buffer,
    },
    };
    ret = i2c_transfer(priv.i2c.adapter, msgs, ARRAY_SIZE(msgs));
    if (ret < 0)
    return ret;
    memcpy(val_buf, priv.rx_buffer + SMI330_NUM_DUMMY_BYTES, val_size);
    return 0;
    }
    static int smi330_regmap_i2c_write(void *context, const void *data,
    size_t count)
    {
    struct smi330_i2c_priv *priv = context;
    u8 reg;
//
// SMI330 I2C write frame:
// <Slave address[6:0], RnW> <x, Register address[6:0]> <Data_0[7:0]> <Data_1[15:8]>...
// <Data_N[7:0]> <Data_N[15:8]>
// Remark: Slave address is not considered part of the frame in the following definitions
//
    reg = *(u8 *)data;
    return i2c_smbus_write_i2c_block_data(priv.i2c, reg,
    count - sizeof(u8),
    data + sizeof(u8));
    }
    static const struct regmap_bus smi330_regmap_bus = {
    .read = smi330_regmap_i2c_read,
    .write = smi330_regmap_i2c_write,
    };
#[no_mangle]
unsafe extern "C" fn smi330_i2c_probe(i2c: *mut i2c_client) -> c_int {
    static int smi330_i2c_probe(struct i2c_client *i2c)
    {
    struct device *dev = &i2c.dev;
    struct smi330_i2c_priv *priv;
    struct regmap *regmap;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.i2c = i2c;
    regmap = devm_regmap_init(dev, &smi330_regmap_bus, priv,
    &smi330_regmap_config);
    if (IS_ERR(regmap))
    return dev_err_probe(dev, PTR_ERR(regmap),
    "Failed to initialize I2C Regmap\n");
    return smi330_core_probe(dev, regmap);
    }
    static const struct i2c_device_id smi330_i2c_device_id[] = {
    { .name = "smi330" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, smi330_i2c_device_id);
    static const struct of_device_id smi330_of_match[] = {
    { .compatible = "bosch,smi330" },
    { }
    };
    MODULE_DEVICE_TABLE(of, smi330_of_match);
    static struct i2c_driver smi330_i2c_driver = {
    .probe = smi330_i2c_probe,
    .id_table = smi330_i2c_device_id,
    .driver = {
    .of_match_table = smi330_of_match,
    .name = "smi330_i2c",
    },
    };
    module_i2c_driver(smi330_i2c_driver);
    MODULE_AUTHOR("Stefan Gutmann <stefan.gutmann@de.bosch.com>");
    MODULE_AUTHOR("Roman Huber <roman.huber@de.bosch.com>");
    MODULE_AUTHOR("Filip Andrei <Andrei.Filip@ro.bosch.com>");
    MODULE_AUTHOR("Drimbarean Avram Andrei <Avram-Andrei.Drimbarean@ro.bosch.com>");
    MODULE_DESCRIPTION("Bosch SMI330 I2C driver");
    MODULE_LICENSE("Dual BSD/GPL");
    MODULE_IMPORT_NS("IIO_SMI330");
