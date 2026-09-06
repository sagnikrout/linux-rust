//! Automatically rewritten from C to Rust
//! Source: drivers/nfc/nxp-nci/core.c
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
// Generic driver for NXP NCI NFC chips
//
// Copyright (C) 2014  NXP Semiconductors  All rights reserved.
//
// Authors: Clément Perrochaud <clement.perrochaud@nxp.com>
//
// Derived from PN544 device driver:
// Copyright (C) 2012  Intel Corporation. All rights reserved.
//

pub const NXP_NCI_HDR_LEN: c_int = 4;

    NFC_PROTO_MIFARE_MASK | \
    NFC_PROTO_FELICA_MASK | \
    NFC_PROTO_ISO14443_MASK | \
    NFC_PROTO_ISO14443_B_MASK | \
    NFC_PROTO_ISO15693_MASK | \
    NFC_PROTO_NFC_DEP_MASK)

#[no_mangle]
unsafe extern "C" fn nxp_nci_open(ndev: *mut nci_dev) -> c_int {
    static int nxp_nci_open(struct nci_dev *ndev)
    {
    struct nxp_nci_info *info = nci_get_drvdata(ndev);
    let mut r: c_int = 0;
    mutex_lock(&info.info_lock);
    if (info.mode != NXP_NCI_MODE_COLD) {
    r = -EBUSY;
    goto open_exit;
    }
    if (info.phy_ops.set_mode)
    r = info.phy_ops.set_mode(info.phy_id, NXP_NCI_MODE_NCI);
    info.mode = NXP_NCI_MODE_NCI;
    open_exit:
    mutex_unlock(&info.info_lock);
    return r;
    }
#[no_mangle]
unsafe extern "C" fn nxp_nci_close(ndev: *mut nci_dev) -> c_int {
    static int nxp_nci_close(struct nci_dev *ndev)
    {
    struct nxp_nci_info *info = nci_get_drvdata(ndev);
    let mut r: c_int = 0;
    mutex_lock(&info.info_lock);
    if (info.phy_ops.set_mode)
    r = info.phy_ops.set_mode(info.phy_id, NXP_NCI_MODE_COLD);
    info.mode = NXP_NCI_MODE_COLD;
    mutex_unlock(&info.info_lock);
    return r;
    }
#[no_mangle]
unsafe extern "C" fn nxp_nci_send(ndev: *mut nci_dev, skb: *mut sk_buff) -> c_int {
    static int nxp_nci_send(struct nci_dev *ndev, struct sk_buff *skb)
    {
    struct nxp_nci_info *info = nci_get_drvdata(ndev);
    int r;
    if (!info.phy_ops.write) {
    kfree_skb(skb);
    return -EOPNOTSUPP;
    }
    if (info.mode != NXP_NCI_MODE_NCI) {
    kfree_skb(skb);
    return -EINVAL;
    }
    r = info.phy_ops.write(info.phy_id, skb);
    if (r < 0) {
    kfree_skb(skb);
    return r;
    }
    consume_skb(skb);
    return 0;
    }
    static int nxp_nci_rf_pll_unlocked_ntf(struct nci_dev *ndev,
    struct sk_buff *skb)
    {
    nfc_err(&ndev.nfc_dev.dev,
    "PLL didn't lock. Missing or unstable clock?\n");
    return 0;
    }
    static int nxp_nci_rf_txldo_error_ntf(struct nci_dev *ndev,
    struct sk_buff *skb)
    {
    nfc_err(&ndev.nfc_dev.dev,
    "RF transmitter couldn't start. Bad power and/or configuration?\n");
    return 0;
    }
    static const struct nci_driver_ops nxp_nci_core_ops[] = {
    {
    .opcode = NXP_NCI_RF_PLL_UNLOCKED_NTF,
    .ntf = nxp_nci_rf_pll_unlocked_ntf,
    },
    {
    .opcode = NXP_NCI_RF_TXLDO_ERROR_NTF,
    .ntf = nxp_nci_rf_txldo_error_ntf,
    },
    };
    static const struct nci_ops nxp_nci_ops = {
    .open = nxp_nci_open,
    .close = nxp_nci_close,
    .send = nxp_nci_send,
    .fw_download = nxp_nci_fw_download,
    .core_ops = nxp_nci_core_ops,
    .n_core_ops = ARRAY_SIZE(nxp_nci_core_ops),
    };
    int nxp_nci_probe(void *phy_id, struct device *pdev,
    const struct nxp_nci_phy_ops *phy_ops,
    unsigned int max_payload,
    struct nci_dev **ndev)
    {
    struct nxp_nci_info *info;
    int r;
    info = devm_kzalloc(pdev, sizeof(struct nxp_nci_info), GFP_KERNEL);
    if (!info)
    return -ENOMEM;
    info.phy_id = phy_id;
    info.pdev = pdev;
    info.phy_ops = phy_ops;
    info.max_payload = max_payload;
    INIT_WORK(&info.fw_info.work, nxp_nci_fw_work);
    init_completion(&info.fw_info.cmd_completion);
    mutex_init(&info.info_lock);
    if (info.phy_ops.set_mode) {
    r = info.phy_ops.set_mode(info.phy_id, NXP_NCI_MODE_COLD);
    if (r < 0)
    return r;
    }
    info.mode = NXP_NCI_MODE_COLD;
    info.ndev = nci_allocate_device(&nxp_nci_ops, NXP_NCI_NFC_PROTOCOLS,
    NXP_NCI_HDR_LEN, 0);
    if (!info.ndev)
    return -ENOMEM;
    nci_set_parent_dev(info.ndev, pdev);
    nci_set_drvdata(info.ndev, info);
    r = nci_register_device(info.ndev);
    if (r < 0) {
    nci_free_device(info.ndev);
    return r;
    }
// ndev = info->ndev;
    return r;
    }
    EXPORT_SYMBOL(nxp_nci_probe);
#[no_mangle]
pub unsafe extern "C" fn nxp_nci_remove(ndev: *mut nci_dev) {
    void nxp_nci_remove(struct nci_dev *ndev)
    {
    struct nxp_nci_info *info = nci_get_drvdata(ndev);
    if (info.mode == NXP_NCI_MODE_FW)
    nxp_nci_fw_work_complete(info, -ESHUTDOWN);
    cancel_work_sync(&info.fw_info.work);
    mutex_lock(&info.info_lock);
    if (info.phy_ops.set_mode)
    info.phy_ops.set_mode(info.phy_id, NXP_NCI_MODE_COLD);
    nci_unregister_device(ndev);
    nci_free_device(ndev);
    mutex_unlock(&info.info_lock);
    }
    EXPORT_SYMBOL(nxp_nci_remove);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("NXP NCI NFC driver");
    MODULE_AUTHOR("Clément Perrochaud <clement.perrochaud@nxp.com>");
