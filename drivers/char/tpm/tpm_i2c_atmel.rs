//! Automatically rewritten from C to Rust
//! Source: drivers/char/tpm/tpm_i2c_atmel.c
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
// ATMEL I2C TPM AT97SC3204T
//
// Copyright (C) 2012 V Lab Technologies
// Teddy Reed <teddy@prosauce.org>
// Copyright (C) 2013, Obsidian Research Corp.
// Jason Gunthorpe <jgunthorpe@obsidianresearch.com>
// Device driver for ATMEL I2C TPMs.
//
// Teddy Reed determined the basic I2C command flow, unlike other I2C TPM
// devices the raw TCG formatted TPM command data is written via I2C and then
// raw TCG formatted TPM command data is returned via I2C.
//
// TGC status/locality/etc functions seen in the LPC implementation do not
// seem to be present.
//

pub const ATMEL_STS_OK: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct priv_data {
    pub len: usize,
//
// This is the amount we read on the first try. 25 was chosen to fit a
// fair number of read responses in the buffer so a 2nd retry can be
// avoided in small message cases.
//
    pub 25]: u8 buffer[sizeof(struct tpm_header) +,
}

    static int i2c_atmel_send(struct tpm_chip *chip, u8 *buf, size_t bufsiz,
    size_t len)
    {
    struct priv_data *priv = dev_get_drvdata(&chip.dev);
    struct i2c_client *client = to_i2c_client(chip.dev.parent);
    s32 status;
    priv.len = 0;
    if (len <= 2)
    return -EIO;
    status = i2c_master_send(client, buf, len);
    dev_dbg(&chip.dev,
    "%s(buf=%*ph len=%0zx) . sts=%d\n", __func__,
    (int)min_t(size_t, 64, len), buf, len, status);
    if (status < 0)
    return status;
//
// The upper layer does not support incomplete sends.
//
    if (status != len)
    return -E2BIG;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn i2c_atmel_recv(chip: *mut tpm_chip, buf: *mut u8, count: usize) -> c_int {
    static int i2c_atmel_recv(struct tpm_chip *chip, u8 *buf, size_t count)
    {
    struct priv_data *priv = dev_get_drvdata(&chip.dev);
    struct i2c_client *client = to_i2c_client(chip.dev.parent);
    struct tpm_header *hdr = (struct tpm_header *)priv.buffer;
    u32 expected_len;
    int rc;
    if (priv.len == 0)
    return -EIO;
//
// Get the message size from the message header, if we didn't get the
// whole message in read_status then we need to re-read the
// message.
//
    expected_len = be32_to_cpu(hdr.length);
    if (expected_len > count)
    return -ENOMEM;
    if (priv.len >= expected_len) {
    dev_dbg(&chip.dev,
    "%s early(buf=%*ph count=%0zx) . ret=%d\n", __func__,
    (int)min_t(size_t, 64, expected_len), buf, count,
    expected_len);
    memcpy(buf, priv.buffer, expected_len);
    return expected_len;
    }
    rc = i2c_master_recv(client, buf, expected_len);
    dev_dbg(&chip.dev,
    "%s reread(buf=%*ph count=%0zx) . ret=%d\n", __func__,
    (int)min_t(size_t, 64, expected_len), buf, count,
    expected_len);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn i2c_atmel_cancel(chip: *mut tpm_chip) {
    static void i2c_atmel_cancel(struct tpm_chip *chip)
    {
    dev_err(&chip.dev, "TPM operation cancellation was requested, but is not supported");
    }
#[no_mangle]
unsafe extern "C" fn i2c_atmel_read_status(chip: *mut tpm_chip) -> u8 {
    static u8 i2c_atmel_read_status(struct tpm_chip *chip)
    {
    struct priv_data *priv = dev_get_drvdata(&chip.dev);
    struct i2c_client *client = to_i2c_client(chip.dev.parent);
    int rc;
//
// The TPM fails the I2C read until it is ready, so we do the entire
// transfer here and buffer it locally. This way the common code can
// properly handle the timeouts.
//
    priv.len = 0;
    memset(priv.buffer, 0, sizeof(priv.buffer));
//
// Once the TPM has completed the command the command remains readable
// until another command is issued.
//
    rc = i2c_master_recv(client, priv.buffer, sizeof(priv.buffer));
    dev_dbg(&chip.dev,
    "%s: sts=%d", __func__, rc);
    if (rc <= 0)
    return 0;
    priv.len = rc;
    return ATMEL_STS_OK;
    }
#[no_mangle]
unsafe extern "C" fn i2c_atmel_req_canceled(chip: *mut tpm_chip, status: u8) -> bool {
    static bool i2c_atmel_req_canceled(struct tpm_chip *chip, u8 status)
    {
    return false;
    }
    static const struct tpm_class_ops i2c_atmel = {
    .flags = TPM_OPS_AUTO_STARTUP,
    .status = i2c_atmel_read_status,
    .recv = i2c_atmel_recv,
    .send = i2c_atmel_send,
    .cancel = i2c_atmel_cancel,
    .req_complete_mask = ATMEL_STS_OK,
    .req_complete_val = ATMEL_STS_OK,
    .req_canceled = i2c_atmel_req_canceled,
    };
#[no_mangle]
unsafe extern "C" fn i2c_atmel_probe(client: *mut i2c_client) -> c_int {
    static int i2c_atmel_probe(struct i2c_client *client)
    {
    struct tpm_chip *chip;
    struct device *dev = &client.dev;
    struct priv_data *priv;
    if (!i2c_check_functionality(client.adapter, I2C_FUNC_I2C))
    return -ENODEV;
    chip = tpmm_chip_alloc(dev, &i2c_atmel);
    if (IS_ERR(chip))
    return PTR_ERR(chip);
    priv = devm_kzalloc(dev, sizeof(struct priv_data), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
// Default timeouts
    chip.timeout_a = msecs_to_jiffies(TPM_I2C_SHORT_TIMEOUT);
    chip.timeout_b = msecs_to_jiffies(TPM_I2C_LONG_TIMEOUT);
    chip.timeout_c = msecs_to_jiffies(TPM_I2C_SHORT_TIMEOUT);
    chip.timeout_d = msecs_to_jiffies(TPM_I2C_SHORT_TIMEOUT);
    dev_set_drvdata(&chip.dev, priv);
//
// There is no known way to probe for this device, and all version
// information seems to be read via TPM commands. Thus we rely on the
// TPM startup process in the common code to detect the device.
//
    return tpm_chip_register(chip);
    }
#[no_mangle]
unsafe extern "C" fn i2c_atmel_remove(client: *mut i2c_client) {
    static void i2c_atmel_remove(struct i2c_client *client)
    {
    struct device *dev = &(client.dev);
    struct tpm_chip *chip = dev_get_drvdata(dev);
    tpm_chip_unregister(chip);
    }
    static const struct i2c_device_id i2c_atmel_id[] = {
    { I2C_DRIVER_NAME },
    {}
    };
    MODULE_DEVICE_TABLE(i2c, i2c_atmel_id);

    static const struct of_device_id i2c_atmel_of_match[] = {
    {.compatible = "atmel,at97sc3204t"},
    {},
    };
    MODULE_DEVICE_TABLE(of, i2c_atmel_of_match);

    static SIMPLE_DEV_PM_OPS(i2c_atmel_pm_ops, tpm_pm_suspend, tpm_pm_resume);
    static struct i2c_driver i2c_atmel_driver = {
    .id_table = i2c_atmel_id,
    .probe = i2c_atmel_probe,
    .remove = i2c_atmel_remove,
    .driver = {
    .name = I2C_DRIVER_NAME,
    .pm = &i2c_atmel_pm_ops,
    .of_match_table = of_match_ptr(i2c_atmel_of_match),
    },
    };
    module_i2c_driver(i2c_atmel_driver);
    MODULE_AUTHOR("Jason Gunthorpe <jgunthorpe@obsidianresearch.com>");
    MODULE_DESCRIPTION("Atmel TPM I2C Driver");
    MODULE_LICENSE("GPL");
