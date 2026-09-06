//! Automatically rewritten from C to Rust
//! Source: drivers/nfc/s3fwrn5/i2c.c
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
// I2C Link Layer for Samsung S3FWRN5 NCI based Driver
//
// Copyright (C) 2015 Samsung Electronics
// Robert Baldyga <r.baldyga@samsung.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct s3fwrn5_i2c_phy {
    pub common: phy_common,
    pub i2c_dev: *mut i2c_client,
    pub clk: *mut clk,
    pub irq_skip:1: c_uint,
}

#[no_mangle]
unsafe extern "C" fn s3fwrn5_i2c_set_mode(phy_id: *mut c_void, mode: enum s3fwrn5_mode) {
    static void s3fwrn5_i2c_set_mode(void *phy_id, enum s3fwrn5_mode mode)
    {
    struct s3fwrn5_i2c_phy *phy = phy_id;
    mutex_lock(&phy.common.mutex);
    if (s3fwrn5_phy_power_ctrl(&phy.common, mode) == false)
    goto out;
    phy.irq_skip = true;
    out:
    mutex_unlock(&phy.common.mutex);
    }
#[no_mangle]
unsafe extern "C" fn s3fwrn5_i2c_write(phy_id: *mut c_void, skb: *mut sk_buff) -> c_int {
    static int s3fwrn5_i2c_write(void *phy_id, struct sk_buff *skb)
    {
    struct s3fwrn5_i2c_phy *phy = phy_id;
    int ret;
    mutex_lock(&phy.common.mutex);
    phy.irq_skip = false;
    ret = i2c_master_send(phy.i2c_dev, skb.data, skb.len);
    if (ret == -EREMOTEIO) {
// Retry, chip was in standby
    usleep_range(110000, 120000);
    ret  = i2c_master_send(phy.i2c_dev, skb.data, skb.len);
    }
    mutex_unlock(&phy.common.mutex);
    if (ret < 0)
    return ret;
    if (ret != skb.len)
    return -EREMOTEIO;
    return 0;
    }
    static const struct s3fwrn5_phy_ops i2c_phy_ops = {
    .set_wake = s3fwrn5_phy_set_wake,
    .set_mode = s3fwrn5_i2c_set_mode,
    .get_mode = s3fwrn5_phy_get_mode,
    .write = s3fwrn5_i2c_write,
    };
#[no_mangle]
unsafe extern "C" fn s3fwrn5_i2c_read(phy: *mut s3fwrn5_i2c_phy) -> c_int {
    static int s3fwrn5_i2c_read(struct s3fwrn5_i2c_phy *phy)
    {
    struct sk_buff *skb;
    size_t hdr_size;
    size_t data_len;
    char hdr[4];
    int ret;
    hdr_size = (phy.common.mode == S3FWRN5_MODE_NCI) ?
    NCI_CTRL_HDR_SIZE : S3FWRN5_FW_HDR_SIZE;
    ret = i2c_master_recv(phy.i2c_dev, hdr, hdr_size);
    if (ret < 0)
    return ret;
    if (ret < hdr_size)
    return -EBADMSG;
    data_len = (phy.common.mode == S3FWRN5_MODE_NCI) ?
    ((struct nci_ctrl_hdr *)hdr).plen :
    ((struct s3fwrn5_fw_header *)hdr).len;
    skb = alloc_skb(hdr_size + data_len, GFP_KERNEL);
    if (!skb)
    return -ENOMEM;
    skb_put_data(skb, hdr, hdr_size);
    if (data_len == 0)
    goto out;
    ret = i2c_master_recv(phy.i2c_dev, skb_put(skb, data_len), data_len);
    if (ret != data_len) {
    kfree_skb(skb);
    return -EBADMSG;
    }
    out:
    return s3fwrn5_recv_frame(phy.common.ndev, skb, phy.common.mode);
    }
#[no_mangle]
unsafe extern "C" fn s3fwrn5_i2c_irq_thread_fn(irq: c_int, phy_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t s3fwrn5_i2c_irq_thread_fn(int irq, void *phy_id)
    {
    struct s3fwrn5_i2c_phy *phy = phy_id;
    if (!phy || !phy.common.ndev) {
    WARN_ON_ONCE(1);
    return IRQ_NONE;
    }
    mutex_lock(&phy.common.mutex);
    if (phy.irq_skip)
    goto out;
    switch (phy.common.mode) {
    case S3FWRN5_MODE_NCI:
    case S3FWRN5_MODE_FW:
    s3fwrn5_i2c_read(phy);
    break;
    case S3FWRN5_MODE_COLD:
    break;
    }
    out:
    mutex_unlock(&phy.common.mutex);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn s3fwrn5_i2c_probe(client: *mut i2c_client) -> c_int {
    static int s3fwrn5_i2c_probe(struct i2c_client *client)
    {
    struct s3fwrn5_i2c_phy *phy;
    int ret;
    phy = devm_kzalloc(&client.dev, sizeof(*phy), GFP_KERNEL);
    if (!phy)
    return -ENOMEM;
    mutex_init(&phy.common.mutex);
    phy.common.mode = S3FWRN5_MODE_COLD;
    phy.irq_skip = true;
    phy.i2c_dev = client;
    i2c_set_clientdata(client, phy);
    phy.common.gpio_en = devm_gpiod_get(&client.dev, "en", GPIOD_OUT_HIGH);
    if (IS_ERR(phy.common.gpio_en))
    return PTR_ERR(phy.common.gpio_en);
    phy.common.gpio_fw_wake = devm_gpiod_get(&client.dev, "wake", GPIOD_OUT_LOW);
    if (IS_ERR(phy.common.gpio_fw_wake))
    return PTR_ERR(phy.common.gpio_fw_wake);
//
// S3FWRN5 depends on a clock input ("XI" pin) to function properly.
// Depending on the hardware configuration this could be an always-on
// oscillator or some external clock that must be explicitly enabled.
// Make sure the clock is running before starting S3FWRN5.
//
    phy.clk = devm_clk_get_optional_enabled(&client.dev, core::ptr::null_mut());
    if (IS_ERR(phy.clk))
    return dev_err_probe(&client.dev, PTR_ERR(phy.clk),
    "failed to get clock\n");
    ret = s3fwrn5_probe(&phy.common.ndev, phy, &phy.i2c_dev.dev,
    &i2c_phy_ops);
    if (ret < 0)
    return ret;
    ret = devm_request_threaded_irq(&client.dev, phy.i2c_dev.irq, core::ptr::null_mut(),
    s3fwrn5_i2c_irq_thread_fn, IRQF_ONESHOT,
    S3FWRN5_I2C_DRIVER_NAME, phy);
    if (ret)
    goto s3fwrn5_remove;
    return 0;
    s3fwrn5_remove:
    s3fwrn5_remove(phy.common.ndev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn s3fwrn5_i2c_remove(client: *mut i2c_client) {
    static void s3fwrn5_i2c_remove(struct i2c_client *client)
    {
    struct s3fwrn5_i2c_phy *phy = i2c_get_clientdata(client);
    s3fwrn5_remove(phy.common.ndev);
    }
    static const struct i2c_device_id s3fwrn5_i2c_id_table[] = {
    { .name = S3FWRN5_I2C_DRIVER_NAME },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, s3fwrn5_i2c_id_table);
    static const struct of_device_id of_s3fwrn5_i2c_match[] = {
    { .compatible = "samsung,s3fwrn5-i2c" },
    { }
    };
    MODULE_DEVICE_TABLE(of, of_s3fwrn5_i2c_match);
    static struct i2c_driver s3fwrn5_i2c_driver = {
    .driver = {
    .name = S3FWRN5_I2C_DRIVER_NAME,
    .of_match_table = of_match_ptr(of_s3fwrn5_i2c_match),
    },
    .probe = s3fwrn5_i2c_probe,
    .remove = s3fwrn5_i2c_remove,
    .id_table = s3fwrn5_i2c_id_table,
    };
    module_i2c_driver(s3fwrn5_i2c_driver);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("I2C driver for Samsung S3FWRN5");
    MODULE_AUTHOR("Robert Baldyga <r.baldyga@samsung.com>");
