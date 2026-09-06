//! Automatically rewritten from C to Rust
//! Source: drivers/media/i2c/uda1342.c
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

#[no_mangle]
unsafe extern "C" fn write_reg(client: *mut i2c_client, reg: c_int, value: c_int) -> c_int {
    static int write_reg(struct i2c_client *client, int reg, int value)
    {
// UDA1342 wants MSB first, but SMBus sends LSB first
    i2c_smbus_write_word_data(client, reg, swab16(value));
    return 0;
    }
    static int uda1342_s_routing(struct v4l2_subdev *sd,
    u32 input, u32 output, u32 config)
    {
    struct i2c_client *client = v4l2_get_subdevdata(sd);
    switch (input) {
    case UDA1342_IN1:
    write_reg(client, 0x00, 0x1241); /* select input 1 */
    break;
    case UDA1342_IN2:
    write_reg(client, 0x00, 0x1441); /* select input 2 */
    break;
    default:
    v4l2_err(sd, "input %d not supported\n", input);
    break;
    }
    return 0;
    }
    static const struct v4l2_subdev_audio_ops uda1342_audio_ops = {
    .s_routing = uda1342_s_routing,
    };
    static const struct v4l2_subdev_ops uda1342_ops = {
    .audio = &uda1342_audio_ops,
    };
#[no_mangle]
unsafe extern "C" fn uda1342_probe(client: *mut i2c_client) -> c_int {
    static int uda1342_probe(struct i2c_client *client)
    {
    struct i2c_adapter *adapter = client.adapter;
    struct v4l2_subdev *sd;
    if (!i2c_check_functionality(adapter, I2C_FUNC_SMBUS_WORD_DATA))
    return -ENODEV;
    dev_dbg(&client.dev, "initializing UDA1342 at address %d on %s\n",
    client.addr, adapter.name);
    sd = devm_kzalloc(&client.dev, sizeof(*sd), GFP_KERNEL);
    if (sd == core::ptr::null_mut())
    return -ENOMEM;
    v4l2_i2c_subdev_init(sd, client, &uda1342_ops);
    write_reg(client, 0x00, 0x8000); /* reset registers */
    write_reg(client, 0x00, 0x1241); /* select input 1 */
    v4l_info(client, "chip found @ 0x%02x (%s)\n",
    client.addr << 1, client.adapter.name);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn uda1342_remove(client: *mut i2c_client) {
    static void uda1342_remove(struct i2c_client *client)
    {
    struct v4l2_subdev *sd = i2c_get_clientdata(client);
    v4l2_device_unregister_subdev(sd);
    }
    static const struct i2c_device_id uda1342_id[] = {
    { .name = "uda1342" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, uda1342_id);
    static struct i2c_driver uda1342_driver = {
    .driver = {
    .name	= "uda1342",
    },
    .probe		= uda1342_probe,
    .remove		= uda1342_remove,
    .id_table	= uda1342_id,
    };
    module_i2c_driver(uda1342_driver);
    MODULE_DESCRIPTION("Philips UDA1342 audio codec driver");
    MODULE_LICENSE("GPL v2");
