//! Automatically rewritten from C to Rust
//! Source: drivers/net/dsa/xrs700x/xrs700x_i2c.c
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
// Copyright (C) 2020 NovaTech LLC
// George McCollister <george.mccollister@gmail.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xrs700x_i2c_cmd {
    pub reg: __be32,
    pub val: __be16,
    pub __packed: },
    static int xrs700x_i2c_reg_read(void *context, unsigned int reg,
    unsigned int *val)
    {
    pub context: *mut *mut device dev =,
    pub to_i2c_client(dev): *mut *mut i2c_client i2c =,
    pub cmd: xrs700x_i2c_cmd,
    pub ret: c_int,
    pub 1): cmd.reg = cpu_to_be32(reg |,
    pub sizeof(cmd.reg)): *mut *mut ret = i2c_master_send(i2c, (char )&cmd.reg,,
    if (ret < 0) {
    pub ret): dev_err(dev, "xrs i2c_master_send returned %d\n",,
    pub ret: return,
    }
    pub sizeof(cmd.val)): *mut *mut ret = i2c_master_recv(i2c, (char )&cmd.val,,
    if (ret < 0) {
    pub ret): dev_err(dev, "xrs i2c_master_recv returned %d\n",,
    pub ret: return,
    }
// val = be16_to_cpu(cmd.val);
    pub 0: return,
    }
    static int xrs700x_i2c_reg_write(void *context, unsigned int reg,
    unsigned int val)
    {
    pub context: *mut *mut device dev =,
    pub to_i2c_client(dev): *mut *mut i2c_client i2c =,
    pub cmd: xrs700x_i2c_cmd,
    pub ret: c_int,
    pub cpu_to_be32(reg): cmd.reg =,
    pub cpu_to_be16(val): cmd.val =,
    pub sizeof(cmd)): *mut *mut ret = i2c_master_send(i2c, (char )&cmd,,
    if (ret < 0) {
    pub ret): dev_err(dev, "xrs i2c_master_send returned %d\n",,
    pub ret: return,
    }
    pub 0: return,
    }
    static const struct regmap_config xrs700x_i2c_regmap_config = {
    .val_bits = 16,
    .reg_stride = 2,
    .reg_bits = 32,
    .pad_bits = 0,
    .write_flag_mask = 0,
    .read_flag_mask = 0,
    .reg_read = xrs700x_i2c_reg_read,
    .reg_write = xrs700x_i2c_reg_write,
    .max_register = 0,
    .cache_type = REGCACHE_NONE,
    .reg_format_endian = REGMAP_ENDIAN_BIG,
    .val_format_endian = REGMAP_ENDIAN_BIG
}

#[no_mangle]
unsafe extern "C" fn xrs700x_i2c_probe(i2c: *mut i2c_client) -> c_int {
    static int xrs700x_i2c_probe(struct i2c_client *i2c)
    {
    struct xrs700x *priv;
    int ret;
    priv = xrs700x_switch_alloc(&i2c.dev, i2c);
    if (!priv)
    return -ENOMEM;
    priv.regmap = devm_regmap_init(&i2c.dev, core::ptr::null_mut(), &i2c.dev,
    &xrs700x_i2c_regmap_config);
    if (IS_ERR(priv.regmap)) {
    ret = PTR_ERR(priv.regmap);
    dev_err(&i2c.dev, "Failed to initialize regmap: %d\n", ret);
    return ret;
    }
    i2c_set_clientdata(i2c, priv);
    ret = xrs700x_switch_register(priv);
// Main DSA driver may not be started yet.
    if (ret)
    return ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xrs700x_i2c_remove(i2c: *mut i2c_client) {
    static void xrs700x_i2c_remove(struct i2c_client *i2c)
    {
    struct xrs700x *priv = i2c_get_clientdata(i2c);
    if (!priv)
    return;
    xrs700x_switch_remove(priv);
    }
#[no_mangle]
unsafe extern "C" fn xrs700x_i2c_shutdown(i2c: *mut i2c_client) {
    static void xrs700x_i2c_shutdown(struct i2c_client *i2c)
    {
    struct xrs700x *priv = i2c_get_clientdata(i2c);
    if (!priv)
    return;
    xrs700x_switch_shutdown(priv);
    i2c_set_clientdata(i2c, core::ptr::null_mut());
    }
    static const struct i2c_device_id xrs700x_i2c_id[] = {
    { .name = "xrs700x-switch" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, xrs700x_i2c_id);
    static const struct of_device_id __maybe_unused xrs700x_i2c_dt_ids[] = {
    { .compatible = "arrow,xrs7003e", .data = &xrs7003e_info },
    { .compatible = "arrow,xrs7003f", .data = &xrs7003f_info },
    { .compatible = "arrow,xrs7004e", .data = &xrs7004e_info },
    { .compatible = "arrow,xrs7004f", .data = &xrs7004f_info },
    {},
    };
    MODULE_DEVICE_TABLE(of, xrs700x_i2c_dt_ids);
    static struct i2c_driver xrs700x_i2c_driver = {
    .driver = {
    .name	= "xrs700x-i2c",
    .of_match_table = of_match_ptr(xrs700x_i2c_dt_ids),
    },
    .probe = xrs700x_i2c_probe,
    .remove	= xrs700x_i2c_remove,
    .shutdown = xrs700x_i2c_shutdown,
    .id_table = xrs700x_i2c_id,
    };
    module_i2c_driver(xrs700x_i2c_driver);
    MODULE_AUTHOR("George McCollister <george.mccollister@gmail.com>");
    MODULE_DESCRIPTION("Arrow SpeedChips XRS700x DSA I2C driver");
    MODULE_LICENSE("GPL v2");
