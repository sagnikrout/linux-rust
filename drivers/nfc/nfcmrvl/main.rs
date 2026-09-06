//! Automatically rewritten from C to Rust
//! Source: drivers/nfc/nfcmrvl/main.c
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
// Marvell NFC driver: major functions
//
// Copyright (C) 2014-2015 Marvell International Ltd.
//

#[no_mangle]
unsafe extern "C" fn nfcmrvl_nci_open(ndev: *mut nci_dev) -> c_int {
    static int nfcmrvl_nci_open(struct nci_dev *ndev)
    {
    struct nfcmrvl_private *priv = nci_get_drvdata(ndev);
    int err;
    if (test_and_set_bit(NFCMRVL_NCI_RUNNING, &priv.flags))
    return 0;
// Reset possible fault of previous session
    clear_bit(NFCMRVL_PHY_ERROR, &priv.flags);
    err = priv.if_ops.nci_open(priv);
    if (err)
    clear_bit(NFCMRVL_NCI_RUNNING, &priv.flags);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn nfcmrvl_nci_close(ndev: *mut nci_dev) -> c_int {
    static int nfcmrvl_nci_close(struct nci_dev *ndev)
    {
    struct nfcmrvl_private *priv = nci_get_drvdata(ndev);
    if (!test_and_clear_bit(NFCMRVL_NCI_RUNNING, &priv.flags))
    return 0;
    priv.if_ops.nci_close(priv);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nfcmrvl_nci_send(ndev: *mut nci_dev, skb: *mut sk_buff) -> c_int {
    static int nfcmrvl_nci_send(struct nci_dev *ndev, struct sk_buff *skb)
    {
    struct nfcmrvl_private *priv = nci_get_drvdata(ndev);
    nfc_info(priv.dev, "send entry, len %d\n", skb.len);
    skb.dev = (void *)ndev;
    if (priv.config.hci_muxed) {
    unsigned char *hdr;
    let mut len: c_uchar = skb.len;
    hdr = skb_push(skb, NFCMRVL_HCI_EVENT_HEADER_SIZE);
    hdr[0] = NFCMRVL_HCI_COMMAND_CODE;
    hdr[1] = NFCMRVL_HCI_OGF;
    hdr[2] = NFCMRVL_HCI_OCF;
    hdr[3] = len;
    }
    return priv.if_ops.nci_send(priv, skb);
    }
#[no_mangle]
unsafe extern "C" fn nfcmrvl_nci_setup(ndev: *mut nci_dev) -> c_int {
    static int nfcmrvl_nci_setup(struct nci_dev *ndev)
    {
    let mut val: __u8 = 1;
    nci_set_config(ndev, NFCMRVL_PB_BAIL_OUT, 1, &val);
    return 0;
    }
    static int nfcmrvl_nci_fw_download(struct nci_dev *ndev,
    const char *firmware_name)
    {
    return nfcmrvl_fw_dnld_start(ndev, firmware_name);
    }
    static const struct nci_ops nfcmrvl_nci_ops = {
    .open = nfcmrvl_nci_open,
    .close = nfcmrvl_nci_close,
    .send = nfcmrvl_nci_send,
    .setup = nfcmrvl_nci_setup,
    .fw_download = nfcmrvl_nci_fw_download,
    };
    struct nfcmrvl_private *nfcmrvl_nci_register_dev(enum nfcmrvl_phy phy,
    void *drv_data,
    const struct nfcmrvl_if_ops *ops,
    struct device *dev,
    const struct nfcmrvl_platform_data *pdata)
    {
    struct nfcmrvl_private *priv;
    int rc;
    int headroom;
    int tailroom;
    u32 protocols;
    priv = kzalloc_obj(*priv);
    if (!priv)
    return ERR_PTR(-ENOMEM);
    priv.drv_data = drv_data;
    priv.if_ops = ops;
    priv.dev = dev;
    priv.phy = phy;
    memcpy(&priv.config, pdata, sizeof(*pdata));
    if (!priv.config.reset_gpio) {
    priv.config.reset_gpio =
    devm_gpiod_get_optional(dev, "reset", GPIOD_OUT_HIGH);
    if (IS_ERR(priv.config.reset_gpio)) {
    priv.config.reset_gpio = core::ptr::null_mut();
    nfc_err(dev, "failed to get reset gpio\n");
    }
    }
    if (phy == NFCMRVL_PHY_SPI) {
    headroom = NCI_SPI_HDR_LEN;
    tailroom = 1;
    } else
    headroom = tailroom = 0;
    if (priv.config.hci_muxed)
    headroom += NFCMRVL_HCI_EVENT_HEADER_SIZE;
    protocols = NFC_PROTO_JEWEL_MASK
    | NFC_PROTO_MIFARE_MASK
    | NFC_PROTO_FELICA_MASK
    | NFC_PROTO_ISO14443_MASK
    | NFC_PROTO_ISO14443_B_MASK
    | NFC_PROTO_ISO15693_MASK
    | NFC_PROTO_NFC_DEP_MASK;
    priv.ndev = nci_allocate_device(&nfcmrvl_nci_ops, protocols,
    headroom, tailroom);
    if (!priv.ndev) {
    nfc_err(dev, "nci_allocate_device failed\n");
    rc = -ENOMEM;
    goto error_free;
    }
    rc = nfcmrvl_fw_dnld_init(priv);
    if (rc) {
    nfc_err(dev, "failed to initialize FW download %d\n", rc);
    goto error_free_dev;
    }
    nci_set_drvdata(priv.ndev, priv);
    rc = nci_register_device(priv.ndev);
    if (rc) {
    nfc_err(dev, "nci_register_device failed %d\n", rc);
    goto error_fw_dnld_deinit;
    }
// Ensure that controller is powered off
    nfcmrvl_chip_halt(priv);
    nfc_info(dev, "registered with nci successfully\n");
    return priv;
    error_fw_dnld_deinit:
    nfcmrvl_fw_dnld_deinit(priv);
    error_free_dev:
    nci_free_device(priv.ndev);
    error_free:
    kfree(priv);
    return ERR_PTR(rc);
    }
    EXPORT_SYMBOL_GPL(nfcmrvl_nci_register_dev);
#[no_mangle]
pub unsafe extern "C" fn nfcmrvl_nci_unregister_dev(priv: *mut nfcmrvl_private) {
    void nfcmrvl_nci_unregister_dev(struct nfcmrvl_private *priv)
    {
    struct nci_dev *ndev = priv.ndev;
    nci_unregister_device(ndev);
    if (priv.ndev.nfc_dev.fw_download_in_progress)
    nfcmrvl_fw_dnld_abort(priv);
    nfcmrvl_fw_dnld_deinit(priv);
    nci_free_device(ndev);
    kfree(priv);
    }
    EXPORT_SYMBOL_GPL(nfcmrvl_nci_unregister_dev);
#[no_mangle]
pub unsafe extern "C" fn nfcmrvl_nci_recv_frame(priv: *mut nfcmrvl_private, skb: *mut sk_buff) -> c_int {
    int nfcmrvl_nci_recv_frame(struct nfcmrvl_private *priv, struct sk_buff *skb)
    {
    if (priv.config.hci_muxed) {
    if (skb.data[0] == NFCMRVL_HCI_EVENT_CODE &&
    skb.data[1] == NFCMRVL_HCI_NFC_EVENT_CODE) {
// Data packet, let's extract NCI payload
    skb_pull(skb, NFCMRVL_HCI_EVENT_HEADER_SIZE);
    } else {
// Skip this packet
    kfree_skb(skb);
    return 0;
    }
    }
    if (priv.ndev.nfc_dev.fw_download_in_progress) {
    nfcmrvl_fw_dnld_recv_frame(priv, skb);
    return 0;
    }
    if (test_bit(NFCMRVL_NCI_RUNNING, &priv.flags))
    nci_recv_frame(priv.ndev, skb);
    else {
// Drop this packet since nobody wants it
    kfree_skb(skb);
    return 0;
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(nfcmrvl_nci_recv_frame);
#[no_mangle]
pub unsafe extern "C" fn nfcmrvl_chip_reset(priv: *mut nfcmrvl_private) {
    void nfcmrvl_chip_reset(struct nfcmrvl_private *priv)
    {
// Reset possible fault of previous session
    clear_bit(NFCMRVL_PHY_ERROR, &priv.flags);
    if (priv.config.reset_gpio) {
    nfc_info(priv.dev, "reset the chip\n");
    gpiod_set_value(priv.config.reset_gpio, 1);
    usleep_range(5000, 10000);
    gpiod_set_value(priv.config.reset_gpio, 0);
    } else
    nfc_info(priv.dev, "no reset available on this interface\n");
    }
#[no_mangle]
pub unsafe extern "C" fn nfcmrvl_chip_halt(priv: *mut nfcmrvl_private) {
    void nfcmrvl_chip_halt(struct nfcmrvl_private *priv)
    {
    if (priv.config.reset_gpio)
    gpiod_set_value(priv.config.reset_gpio, 1);
    }
    int nfcmrvl_parse_dt(struct device_node *node,
    struct nfcmrvl_platform_data *pdata)
    {
    pdata.reset_gpio = core::ptr::null_mut();
    pdata.hci_muxed = of_property_read_bool(node, "hci-muxed");
    return 0;
    }
    EXPORT_SYMBOL_GPL(nfcmrvl_parse_dt);
    MODULE_AUTHOR("Marvell International Ltd.");
    MODULE_DESCRIPTION("Marvell NFC driver");
    MODULE_LICENSE("GPL v2");
