//! Automatically rewritten from C to Rust
//! Source: drivers/nfc/microread/i2c.c
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
// HCI based Driver for Inside Secure microread NFC Chip - i2c layer
//
// Copyright (C) 2013 Intel Corporation. All rights reserved.
//

pub const MICROREAD_I2C_FRAME_HEADROOM: c_int = 1;
pub const MICROREAD_I2C_FRAME_TAILROOM: c_int = 1;
// framing in HCI mode
pub const MICROREAD_I2C_LLC_LEN: c_int = 1;
pub const MICROREAD_I2C_LLC_CRC: c_int = 1;

    MICROREAD_I2C_LLC_CRC)

pub const MICROREAD_I2C_LLC_MAX_PAYLOAD: c_int = 29;

    MICROREAD_I2C_LLC_MAX_PAYLOAD)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct microread_i2c_phy {
    pub i2c_dev: *mut i2c_client,
    pub hdev: *mut nfc_hci_dev,
    pub /*: *mut int hard_fault;,
// < 0 if hardware error occured (e.g. i2c err)
// and prevents normal operation.
//
}

    do {								\
    pr_debug("%s:\n", info);				\
    print_hex_dump(KERN_DEBUG, "i2c: ", DUMP_PREFIX_OFFSET,	\
    16, 1, (skb).data, (skb).len, 0);	\
    } while (0)
#[no_mangle]
unsafe extern "C" fn microread_i2c_add_len_crc(skb: *mut sk_buff) {
    static void microread_i2c_add_len_crc(struct sk_buff *skb)
    {
    int i;
    let mut crc: u8 = 0;
    int len;
    len = skb.len;
// (u8 *)skb_push(skb, 1) = len;
    for (i = 0; i < skb.len; i++)
    crc = crc ^ skb.data[i];
    skb_put_u8(skb, crc);
    }
#[no_mangle]
unsafe extern "C" fn microread_i2c_remove_len_crc(skb: *mut sk_buff) {
    static void microread_i2c_remove_len_crc(struct sk_buff *skb)
    {
    skb_pull(skb, MICROREAD_I2C_FRAME_HEADROOM);
    skb_trim(skb, MICROREAD_I2C_FRAME_TAILROOM);
    }
#[no_mangle]
unsafe extern "C" fn check_crc(skb: *const sk_buff) -> c_int {
    static int check_crc(const struct sk_buff *skb)
    {
    int i;
    let mut crc: u8 = 0;
    for (i = 0; i < skb.len - 1; i++)
    crc = crc ^ skb.data[i];
    if (crc != skb.data[skb.len-1]) {
    pr_err("CRC error 0x%x != 0x%x\n", crc, skb.data[skb.len-1]);
    pr_info("%s: BAD CRC\n", __func__);
    return -EPERM;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn microread_i2c_enable(phy_id: *mut c_void) -> c_int {
    static int microread_i2c_enable(void *phy_id)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn microread_i2c_disable(phy_id: *mut c_void) {
    static void microread_i2c_disable(void *phy_id)
    {
    return;
    }
#[no_mangle]
unsafe extern "C" fn microread_i2c_write(phy_id: *mut c_void, skb: *mut sk_buff) -> c_int {
    static int microread_i2c_write(void *phy_id, struct sk_buff *skb)
    {
    int r;
    struct microread_i2c_phy *phy = phy_id;
    struct i2c_client *client = phy.i2c_dev;
    if (phy.hard_fault != 0)
    return phy.hard_fault;
    usleep_range(3000, 6000);
    microread_i2c_add_len_crc(skb);
    I2C_DUMP_SKB("i2c frame written", skb);
    r = i2c_master_send(client, skb.data, skb.len);
    if (r == -EREMOTEIO) {	/* Retry, chip was in standby */
    usleep_range(6000, 10000);
    r = i2c_master_send(client, skb.data, skb.len);
    }
    if (r >= 0) {
    if (r != skb.len)
    r = -EREMOTEIO;
    else
    r = 0;
    }
    microread_i2c_remove_len_crc(skb);
    return r;
    }
    static int microread_i2c_read(struct microread_i2c_phy *phy,
    struct sk_buff **skb)
    {
    int r;
    u8 len;
    u8 tmp[MICROREAD_I2C_LLC_MAX_SIZE - 1];
    struct i2c_client *client = phy.i2c_dev;
    r = i2c_master_recv(client, &len, 1);
    if (r != 1) {
    nfc_err(&client.dev, "cannot read len byte\n");
    return -EREMOTEIO;
    }
    if ((len < MICROREAD_I2C_LLC_MIN_SIZE) ||
    (len > MICROREAD_I2C_LLC_MAX_SIZE)) {
    nfc_err(&client.dev, "invalid len byte\n");
    r = -EBADMSG;
    goto flush;
    }
// skb = alloc_skb(1 + len, GFP_KERNEL);
    if (*skb == core::ptr::null_mut()) {
    r = -ENOMEM;
    goto flush;
    }
    skb_put_u8(*skb, len);
    r = i2c_master_recv(client, skb_put(*skb, len), len);
    if (r != len) {
    kfree_skb(*skb);
    return -EREMOTEIO;
    }
    I2C_DUMP_SKB("cc frame read", *skb);
    r = check_crc(*skb);
    if (r != 0) {
    kfree_skb(*skb);
    r = -EBADMSG;
    goto flush;
    }
    skb_pull(*skb, 1);
    skb_trim(*skb, (*skb).len - MICROREAD_I2C_FRAME_TAILROOM);
    usleep_range(3000, 6000);
    return 0;
    flush:
    if (i2c_master_recv(client, tmp, sizeof(tmp)) < 0)
    r = -EREMOTEIO;
    usleep_range(3000, 6000);
    return r;
    }
#[no_mangle]
unsafe extern "C" fn microread_i2c_irq_thread_fn(irq: c_int, phy_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t microread_i2c_irq_thread_fn(int irq, void *phy_id)
    {
    struct microread_i2c_phy *phy = phy_id;
    struct sk_buff *skb = core::ptr::null_mut();
    int r;
    if (!phy || irq != phy.i2c_dev.irq) {
    WARN_ON_ONCE(1);
    return IRQ_NONE;
    }
    if (phy.hard_fault != 0)
    return IRQ_HANDLED;
    r = microread_i2c_read(phy, &skb);
    if (r == -EREMOTEIO) {
    phy.hard_fault = r;
    nfc_hci_recv_frame(phy.hdev, core::ptr::null_mut());
    return IRQ_HANDLED;
    } else if ((r == -ENOMEM) || (r == -EBADMSG)) {
    return IRQ_HANDLED;
    }
    nfc_hci_recv_frame(phy.hdev, skb);
    return IRQ_HANDLED;
    }
    static const struct nfc_phy_ops i2c_phy_ops = {
    .write = microread_i2c_write,
    .enable = microread_i2c_enable,
    .disable = microread_i2c_disable,
    };
#[no_mangle]
unsafe extern "C" fn microread_i2c_probe(client: *mut i2c_client) -> c_int {
    static int microread_i2c_probe(struct i2c_client *client)
    {
    struct microread_i2c_phy *phy;
    int r;
    phy = devm_kzalloc(&client.dev, sizeof(struct microread_i2c_phy),
    GFP_KERNEL);
    if (!phy)
    return -ENOMEM;
    i2c_set_clientdata(client, phy);
    phy.i2c_dev = client;
    r = request_threaded_irq(client.irq, core::ptr::null_mut(), microread_i2c_irq_thread_fn,
    IRQF_TRIGGER_RISING | IRQF_ONESHOT,
    MICROREAD_I2C_DRIVER_NAME, phy);
    if (r) {
    nfc_err(&client.dev, "Unable to register IRQ handler\n");
    return r;
    }
    r = microread_probe(phy, &i2c_phy_ops, LLC_SHDLC_NAME,
    MICROREAD_I2C_FRAME_HEADROOM,
    MICROREAD_I2C_FRAME_TAILROOM,
    MICROREAD_I2C_LLC_MAX_PAYLOAD, &phy.hdev);
    if (r < 0)
    goto err_irq;
    return 0;
    err_irq:
    free_irq(client.irq, phy);
    return r;
    }
#[no_mangle]
unsafe extern "C" fn microread_i2c_remove(client: *mut i2c_client) {
    static void microread_i2c_remove(struct i2c_client *client)
    {
    struct microread_i2c_phy *phy = i2c_get_clientdata(client);
    microread_remove(phy.hdev);
    free_irq(client.irq, phy);
    }
    static const struct i2c_device_id microread_i2c_id[] = {
    { .name = MICROREAD_I2C_DRIVER_NAME },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, microread_i2c_id);
    static struct i2c_driver microread_i2c_driver = {
    .driver = {
    .name = MICROREAD_I2C_DRIVER_NAME,
    },
    .probe		= microread_i2c_probe,
    .remove		= microread_i2c_remove,
    .id_table	= microread_i2c_id,
    };
    module_i2c_driver(microread_i2c_driver);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION(DRIVER_DESC);
