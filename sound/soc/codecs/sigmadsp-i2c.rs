//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/sigmadsp-i2c.c
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
// Load Analog Devices SigmaStudio firmware files
//
// Copyright 2009-2011 Analog Devices Inc.
//

    static int sigmadsp_write_i2c(void *control_data,
    unsigned int addr, const uint8_t data[], size_t len)
    {
    uint8_t *buf;
    int ret;
    buf = kzalloc(2 + len, GFP_KERNEL | GFP_DMA);
    if (!buf)
    return -ENOMEM;
    put_unaligned_be16(addr, buf);
    memcpy(buf + 2, data, len);
    ret = i2c_master_send(control_data, buf, len + 2);
    kfree(buf);
    if (ret < 0)
    return ret;
    return 0;
    }
    static int sigmadsp_read_i2c(void *control_data,
    unsigned int addr, uint8_t data[], size_t len)
    {
    struct i2c_client *client = control_data;
    struct i2c_msg msgs[2];
    uint8_t buf[2];
    int ret;
    put_unaligned_be16(addr, buf);
    msgs[0].addr = client.addr;
    msgs[0].len = sizeof(buf);
    msgs[0].buf = buf;
    msgs[0].flags = 0;
    msgs[1].addr = client.addr;
    msgs[1].len = len;
    msgs[1].buf = data;
    msgs[1].flags = I2C_M_RD;
    ret = i2c_transfer(client.adapter, msgs, ARRAY_SIZE(msgs));
    if (ret < 0)
    return ret;
#[no_mangle]
pub unsafe extern "C" fn if(ARRAY_SIZE(msgs): ret !=) -> else {
    else if (ret != ARRAY_SIZE(msgs))
    return -EIO;
    return 0;
    }
//
// devm_sigmadsp_init_i2c() - Initialize SigmaDSP instance
// @client: The parent I2C device
// @ops: The sigmadsp_ops to use for this instance
// @firmware_name: Name of the firmware file to load
//
// Allocates a SigmaDSP instance and loads the specified firmware file.
//
// Returns a pointer to a struct sigmadsp on success, or a PTR_ERR() on error.
//
    struct sigmadsp *devm_sigmadsp_init_i2c(struct i2c_client *client,
    const struct sigmadsp_ops *ops,	const char *firmware_name)
    {
    struct sigmadsp *sigmadsp;
    sigmadsp = devm_sigmadsp_init(&client.dev, ops, firmware_name);
    if (IS_ERR(sigmadsp))
    return sigmadsp;
    sigmadsp.control_data = client;
    sigmadsp.write = sigmadsp_write_i2c;
    sigmadsp.read = sigmadsp_read_i2c;
    return sigmadsp;
    }
    EXPORT_SYMBOL_GPL(devm_sigmadsp_init_i2c);
    MODULE_AUTHOR("Lars-Peter Clausen <lars@metafoo.de>");
    MODULE_DESCRIPTION("SigmaDSP I2C firmware loader");
    MODULE_LICENSE("GPL");
