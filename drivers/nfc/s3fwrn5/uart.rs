//! Automatically rewritten from C to Rust
//! Source: drivers/nfc/s3fwrn5/uart.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// UART Link Layer for S3FWRN82 NCI based Driver
//
// Copyright (C) 2015 Samsung Electronics
// Robert Baldyga <r.baldyga@samsung.com>
// Copyright (C) 2020 Samsung Electronics
// Bongsu Jeon <bongsu.jeon@samsung.com>
//

pub const S3FWRN82_NCI_HEADER: c_int = 3;
pub const S3FWRN82_NCI_IDX: c_int = 2;
pub const NCI_SKB_BUFF_LEN: c_int = 258;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct s3fwrn82_uart_phy {
    pub common: phy_common,
    pub ser_dev: *mut serdev_device,
    pub recv_skb: *mut sk_buff,
}

#[no_mangle]
unsafe extern "C" fn s3fwrn82_uart_write(phy_id: *mut c_void, out: *mut sk_buff) -> c_int {
    static int s3fwrn82_uart_write(void *phy_id, struct sk_buff *out)
    {
    struct s3fwrn82_uart_phy *phy = phy_id;
    int err;
    err = serdev_device_write(phy.ser_dev,
    out.data, out.len,
    MAX_SCHEDULE_TIMEOUT);
    if (err < 0)
    return err;
    return 0;
    }
    static const struct s3fwrn5_phy_ops uart_phy_ops = {
    .set_wake = s3fwrn5_phy_set_wake,
    .set_mode = s3fwrn5_phy_set_mode,
    .get_mode = s3fwrn5_phy_get_mode,
    .write = s3fwrn82_uart_write,
    };
    static size_t s3fwrn82_uart_read(struct serdev_device *serdev,
    const u8 *data, size_t count)
    {
    struct s3fwrn82_uart_phy *phy = serdev_device_get_drvdata(serdev);
    size_t i;
    for (i = 0; i < count; i++) {
    if (!phy.recv_skb) {
    phy.recv_skb = alloc_skb(NCI_SKB_BUFF_LEN, GFP_KERNEL);
    if (!phy.recv_skb)
    return i;
    }
    skb_put_u8(phy.recv_skb, *data++);
    if (phy.recv_skb.len < S3FWRN82_NCI_HEADER)
    continue;
    if ((phy.recv_skb.len - S3FWRN82_NCI_HEADER)
    < phy.recv_skb.data[S3FWRN82_NCI_IDX])
    continue;
    s3fwrn5_recv_frame(phy.common.ndev, phy.recv_skb,
    phy.common.mode);
    phy.recv_skb = core::ptr::null_mut();
    }
    return i;
    }
    static const struct serdev_device_ops s3fwrn82_serdev_ops = {
    .receive_buf = s3fwrn82_uart_read,
    .write_wakeup = serdev_device_write_wakeup,
    };
    static const struct of_device_id s3fwrn82_uart_of_match[] = {
    { .compatible = "samsung,s3fwrn82" },
    { }
    };
    MODULE_DEVICE_TABLE(of, s3fwrn82_uart_of_match);
#[no_mangle]
unsafe extern "C" fn s3fwrn82_uart_probe(serdev: *mut serdev_device) -> c_int {
    static int s3fwrn82_uart_probe(struct serdev_device *serdev)
    {
    struct s3fwrn82_uart_phy *phy;
    let mut ret: c_int = -ENOMEM;
    phy = devm_kzalloc(&serdev.dev, sizeof(*phy), GFP_KERNEL);
    if (!phy)
    goto err_exit;
    phy.recv_skb = alloc_skb(NCI_SKB_BUFF_LEN, GFP_KERNEL);
    if (!phy.recv_skb)
    goto err_exit;
    mutex_init(&phy.common.mutex);
    phy.common.mode = S3FWRN5_MODE_COLD;
    phy.ser_dev = serdev;
    serdev_device_set_drvdata(serdev, phy);
    serdev_device_set_client_ops(serdev, &s3fwrn82_serdev_ops);
    ret = serdev_device_open(serdev);
    if (ret) {
    dev_err(&serdev.dev, "Unable to open device\n");
    goto err_skb;
    }
    ret = serdev_device_set_baudrate(serdev, 115200);
    if (ret != 115200) {
    ret = -EINVAL;
    goto err_serdev;
    }
    serdev_device_set_flow_control(serdev, false);
    phy.common.gpio_en = devm_gpiod_get(&serdev.dev, "en", GPIOD_OUT_HIGH);
    if (IS_ERR(phy.common.gpio_en)) {
    ret = PTR_ERR(phy.common.gpio_en);
    goto err_serdev;
    }
    phy.common.gpio_fw_wake = devm_gpiod_get(&serdev.dev, "wake", GPIOD_OUT_LOW);
    if (IS_ERR(phy.common.gpio_fw_wake)) {
    ret = PTR_ERR(phy.common.gpio_fw_wake);
    goto err_serdev;
    }
    ret = s3fwrn5_probe(&phy.common.ndev, phy, &phy.ser_dev.dev,
    &uart_phy_ops);
    if (ret < 0)
    goto err_serdev;
    return ret;
    err_serdev:
    serdev_device_close(serdev);
    err_skb:
    kfree_skb(phy.recv_skb);
    err_exit:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn s3fwrn82_uart_remove(serdev: *mut serdev_device) {
    static void s3fwrn82_uart_remove(struct serdev_device *serdev)
    {
    struct s3fwrn82_uart_phy *phy = serdev_device_get_drvdata(serdev);
    s3fwrn5_remove(phy.common.ndev);
    serdev_device_close(serdev);
    kfree_skb(phy.recv_skb);
    }
    static struct serdev_device_driver s3fwrn82_uart_driver = {
    .probe = s3fwrn82_uart_probe,
    .remove = s3fwrn82_uart_remove,
    .driver = {
    .name = "s3fwrn82_uart",
    .of_match_table = s3fwrn82_uart_of_match,
    },
    };
    module_serdev_device_driver(s3fwrn82_uart_driver);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("UART driver for Samsung NFC");
    MODULE_AUTHOR("Bongsu Jeon <bongsu.jeon@samsung.com>");
