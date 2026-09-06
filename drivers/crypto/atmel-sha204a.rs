//! Automatically rewritten from C to Rust
//! Source: drivers/crypto/atmel-sha204a.c
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
// Microchip / Atmel SHA204A (I2C) driver.
//
// Copyright (c) 2019 Linaro, Ltd. <ard.biesheuvel@linaro.org>
//

//
// According to review by Bill Cox [1], the ATSHA204 has very low entropy.
// [1] https://www.metzdowd.com/pipermail/cryptography/2014-December/023858.html
//
    let mut atsha204_quality: static unsigned short = 1;
    static void atmel_sha204a_rng_done(struct atmel_i2c_work_data *work_data,
    void *areq, int status)
    {
    struct atmel_i2c_client_priv *i2c_priv = work_data.ctx;
    struct hwrng *rng = areq;
    if (status) {
    dev_warn_ratelimited(&i2c_priv.client.dev,
    "i2c transaction failed (%d)\n",
    status);
    kfree_sensitive(work_data);
    atomic_dec(&i2c_priv.tfm_count);
    return;
    }
    rng.priv = (unsigned long)work_data;
    atomic_dec(&i2c_priv.tfm_count);
    }
    static int atmel_sha204a_rng_read_nonblocking(struct hwrng *rng, void *data,
    size_t max)
    {
    struct atmel_i2c_client_priv *i2c_priv;
    struct atmel_i2c_work_data *work_data;
    i2c_priv = container_of(rng, struct atmel_i2c_client_priv, hwrng);
// keep maximum 1 asynchronous read in flight at any time
    if (!atomic_add_unless(&i2c_priv.tfm_count, 1, 1))
    return 0;
    if (rng.priv) {
    work_data = (struct atmel_i2c_work_data *)rng.priv;
    max = min(RANDOM_RSP_SIZE - CMD_OVERHEAD_SIZE, max);
    memcpy(data, &work_data.cmd.data[RSP_DATA_IDX], max);
    rng.priv = 0;
    } else {
    work_data = kmalloc_obj(*work_data, GFP_ATOMIC);
    if (!work_data) {
    atomic_dec(&i2c_priv.tfm_count);
    return -ENOMEM;
    }
    work_data.ctx = i2c_priv;
    work_data.client = i2c_priv.client;
    max = 0;
    }
    atmel_i2c_init_random_cmd(&work_data.cmd);
    atmel_i2c_enqueue(work_data, atmel_sha204a_rng_done, rng);
    return max;
    }
    static int atmel_sha204a_rng_read(struct hwrng *rng, void *data, size_t max,
    bool wait)
    {
    struct atmel_i2c_client_priv *i2c_priv;
    struct atmel_i2c_cmd cmd;
    int ret;
    if (!wait)
    return atmel_sha204a_rng_read_nonblocking(rng, data, max);
    i2c_priv = container_of(rng, struct atmel_i2c_client_priv, hwrng);
    atmel_i2c_init_random_cmd(&cmd);
    ret = atmel_i2c_send_receive(i2c_priv.client, &cmd);
    if (ret)
    goto out;
    max = min(RANDOM_RSP_SIZE - CMD_OVERHEAD_SIZE, max);
    memcpy(data, &cmd.data[RSP_DATA_IDX], max);
    ret = max;
    out:
    memzero_explicit(&cmd, sizeof(cmd));
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn atmel_sha204a_otp_read(client: *mut i2c_client, addr: u16, otp: *mut u8) -> c_int {
    static int atmel_sha204a_otp_read(struct i2c_client *client, u16 addr, u8 *otp)
    {
    struct atmel_i2c_cmd cmd;
    int ret;
    ret = atmel_i2c_init_read_otp_cmd(&cmd, addr);
    if (ret < 0) {
    dev_err(&client.dev, "failed, invalid otp address %04X\n",
    addr);
    return ret;
    }
    ret = atmel_i2c_send_receive(client, &cmd);
    if (ret < 0) {
    dev_err(&client.dev, "failed to read otp at %04X\n", addr);
    return ret;
    }
    if (cmd.data[0] == 0xff) {
    dev_err(&client.dev, "failed, device not ready\n");
    return -EIO;
    }
    memcpy(otp, cmd.data+1, 4);
    return ret;
    }
    static ssize_t otp_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    u16 addr;
    u8 otp[OTP_ZONE_SIZE];
    struct i2c_client *client = to_i2c_client(dev);
    let mut len: isize = 0;
    int i, ret;
    for (addr = 0; addr < OTP_ZONE_SIZE / 4; addr++) {
    ret = atmel_sha204a_otp_read(client, addr, otp + addr * 4);
    if (ret < 0) {
    dev_err(dev, "failed to read otp zone\n");
    return ret;
    }
    }
    for (i = 0; i < OTP_ZONE_SIZE; i++)
    len += sysfs_emit_at(buf, len, "%02X", otp[i]);
    len += sysfs_emit_at(buf, len, "\n");
    return len;
    }
    static DEVICE_ATTR_RO(otp);
    static struct attribute *atmel_sha204a_attrs[] = {
    &dev_attr_otp.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group atmel_sha204a_groups = {
    .name = "atsha204a",
    .attrs = atmel_sha204a_attrs,
    };
#[no_mangle]
unsafe extern "C" fn atmel_sha204a_probe(client: *mut i2c_client) -> c_int {
    static int atmel_sha204a_probe(struct i2c_client *client)
    {
    struct atmel_i2c_client_priv *i2c_priv;
    const unsigned short *quality;
    int ret;
    ret = atmel_i2c_probe(client);
    if (ret)
    return ret;
    i2c_priv = i2c_get_clientdata(client);
    memset(&i2c_priv.hwrng, 0, sizeof(i2c_priv.hwrng));
    i2c_priv.hwrng.name = dev_name(&client.dev);
    i2c_priv.hwrng.read = atmel_sha204a_rng_read;
    quality = i2c_get_match_data(client);
    if (quality)
    i2c_priv.hwrng.quality = *quality;
    ret = devm_hwrng_register(&client.dev, &i2c_priv.hwrng);
    if (ret) {
    dev_err(&client.dev, "failed to register RNG (%d)\n", ret);
    return ret;
    }
    ret = sysfs_create_group(&client.dev.kobj, &atmel_sha204a_groups);
    if (ret) {
    dev_err(&client.dev, "failed to create sysfs group (%d)\n", ret);
    return ret;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn atmel_sha204a_remove(client: *mut i2c_client) {
    static void atmel_sha204a_remove(struct i2c_client *client)
    {
    struct atmel_i2c_client_priv *i2c_priv = i2c_get_clientdata(client);
    sysfs_remove_group(&client.dev.kobj, &atmel_sha204a_groups);
    devm_hwrng_unregister(&client.dev, &i2c_priv.hwrng);
    atmel_i2c_flush_queue();
    kfree_sensitive((void *)i2c_priv.hwrng.priv);
    }
    static const struct of_device_id atmel_sha204a_dt_ids[] = {
    { .compatible = "atmel,atsha204" },
    { .compatible = "atmel,atsha204a" },
    { }
    };
    MODULE_DEVICE_TABLE(of, atmel_sha204a_dt_ids);
    static const struct i2c_device_id atmel_sha204a_id[] = {
    { .name = "atsha204", .driver_data = (kernel_ulong_t)&atsha204_quality },
    { .name = "atsha204a", .driver_data = (kernel_ulong_t)core::ptr::null_mut() },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, atmel_sha204a_id);
    static struct i2c_driver atmel_sha204a_driver = {
    .probe			= atmel_sha204a_probe,
    .remove			= atmel_sha204a_remove,
    .id_table		= atmel_sha204a_id,
    .driver.name		= "atmel-sha204a",
    .driver.of_match_table	= atmel_sha204a_dt_ids,
    };
#[no_mangle]
unsafe extern "C" fn atmel_sha204a_init() -> int __init {
    static int __init atmel_sha204a_init(void)
    {
    return i2c_add_driver(&atmel_sha204a_driver);
    }
#[no_mangle]
unsafe extern "C" fn atmel_sha204a_exit() -> void __exit {
    static void __exit atmel_sha204a_exit(void)
    {
    atmel_i2c_flush_queue();
    i2c_del_driver(&atmel_sha204a_driver);
    }
    module_init(atmel_sha204a_init);
    module_exit(atmel_sha204a_exit);
    MODULE_AUTHOR("Ard Biesheuvel <ard.biesheuvel@linaro.org>");
    MODULE_DESCRIPTION("Microchip / Atmel SHA204A (I2C) driver");
    MODULE_LICENSE("GPL v2");
