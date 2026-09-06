//! Automatically rewritten from C to Rust
//! Source: drivers/media/i2c/ov7640.c
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
// Copyright (C) 2005-2006 Micronas USA Inc.
//

    MODULE_DESCRIPTION("OmniVision ov7640 sensor driver");
    MODULE_LICENSE("GPL v2");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg_val {
    pub reg: u8,
    pub val: u8,
}

    static const struct reg_val regval_init[] = {
    {0x12, 0x80},
    {0x12, 0x54},
    {0x14, 0x24},
    {0x15, 0x01},
    {0x28, 0x20},
    {0x75, 0x82},
    };
    static int write_regs(struct i2c_client *client,
    const struct reg_val *rv, int len)
    {
    while (--len >= 0) {
    if (i2c_smbus_write_byte_data(client, rv.reg, rv.val) < 0)
    return -1;
    rv++;
    }
    return 0;
    }
// -----------------------------------------------------------------------
    static const struct v4l2_subdev_ops ov7640_ops;
#[no_mangle]
unsafe extern "C" fn ov7640_probe(client: *mut i2c_client) -> c_int {
    static int ov7640_probe(struct i2c_client *client)
    {
    struct i2c_adapter *adapter = client.adapter;
    struct v4l2_subdev *sd;
    if (!i2c_check_functionality(adapter, I2C_FUNC_SMBUS_BYTE_DATA))
    return -ENODEV;
    sd = devm_kzalloc(&client.dev, sizeof(*sd), GFP_KERNEL);
    if (sd == core::ptr::null_mut())
    return -ENOMEM;
    v4l2_i2c_subdev_init(sd, client, &ov7640_ops);
    client.flags = I2C_CLIENT_SCCB;
    v4l_info(client, "chip found @ 0x%02x (%s)\n",
    client.addr << 1, client.adapter.name);
    if (write_regs(client, regval_init, ARRAY_SIZE(regval_init)) < 0) {
    v4l_err(client, "error initializing OV7640\n");
    return -ENODEV;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ov7640_remove(client: *mut i2c_client) {
    static void ov7640_remove(struct i2c_client *client)
    {
    struct v4l2_subdev *sd = i2c_get_clientdata(client);
    v4l2_device_unregister_subdev(sd);
    }
    static const struct i2c_device_id ov7640_id[] = {
    { .name = "ov7640" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, ov7640_id);
    static struct i2c_driver ov7640_driver = {
    .driver = {
    .name	= "ov7640",
    },
    .probe = ov7640_probe,
    .remove = ov7640_remove,
    .id_table = ov7640_id,
    };
    module_i2c_driver(ov7640_driver);
