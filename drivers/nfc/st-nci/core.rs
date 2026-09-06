//! Automatically rewritten from C to Rust
//! Source: drivers/nfc/st-nci/core.c
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
// NCI based Driver for STMicroelectronics NFC Chip
//
// Copyright (C) 2014-2015  STMicroelectronics SAS. All rights reserved.
//

pub const ST_NCI1_X_PROPRIETARY_ISO15693: c_uint = 0x83;
#[no_mangle]
unsafe extern "C" fn st_nci_init(ndev: *mut nci_dev) -> c_int {
    static int st_nci_init(struct nci_dev *ndev)
    {
    struct nci_mode_set_cmd cmd;
    cmd.cmd_type = ST_NCI_SET_NFC_MODE;
    cmd.mode = 1;
    return nci_prop_cmd(ndev, ST_NCI_CORE_PROP,
    sizeof(struct nci_mode_set_cmd), (__u8 *)&cmd);
    }
#[no_mangle]
unsafe extern "C" fn st_nci_open(ndev: *mut nci_dev) -> c_int {
    static int st_nci_open(struct nci_dev *ndev)
    {
    struct st_nci_info *info = nci_get_drvdata(ndev);
    int r;
    if (test_and_set_bit(ST_NCI_RUNNING, &info.flags))
    return 0;
    r = ndlc_open(info.ndlc);
    if (r)
    clear_bit(ST_NCI_RUNNING, &info.flags);
    return r;
    }
#[no_mangle]
unsafe extern "C" fn st_nci_close(ndev: *mut nci_dev) -> c_int {
    static int st_nci_close(struct nci_dev *ndev)
    {
    struct st_nci_info *info = nci_get_drvdata(ndev);
    if (!test_bit(ST_NCI_RUNNING, &info.flags))
    return 0;
    ndlc_close(info.ndlc);
    clear_bit(ST_NCI_RUNNING, &info.flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn st_nci_send(ndev: *mut nci_dev, skb: *mut sk_buff) -> c_int {
    static int st_nci_send(struct nci_dev *ndev, struct sk_buff *skb)
    {
    struct st_nci_info *info = nci_get_drvdata(ndev);
    skb.dev = (void *)ndev;
    if (!test_bit(ST_NCI_RUNNING, &info.flags))
    return -EBUSY;
    return ndlc_send(info.ndlc, skb);
    }
    static __u32 st_nci_get_rfprotocol(struct nci_dev *ndev,
    __u8 rf_protocol)
    {
    return rf_protocol == ST_NCI1_X_PROPRIETARY_ISO15693 ?
    NFC_PROTO_ISO15693_MASK : 0;
    }
    static int st_nci_prop_rsp_packet(struct nci_dev *ndev,
    struct sk_buff *skb)
    {
    let mut status: __u8 = skb.data[0];
    nci_req_complete(ndev, status);
    return 0;
    }
    static const struct nci_driver_ops st_nci_prop_ops[] = {
    {
    .opcode = nci_opcode_pack(NCI_GID_PROPRIETARY,
    ST_NCI_CORE_PROP),
    .rsp = st_nci_prop_rsp_packet,
    },
    };
    static const struct nci_ops st_nci_ops = {
    .init = st_nci_init,
    .open = st_nci_open,
    .close = st_nci_close,
    .send = st_nci_send,
    .get_rfprotocol = st_nci_get_rfprotocol,
    .discover_se = st_nci_discover_se,
    .enable_se = st_nci_enable_se,
    .disable_se = st_nci_disable_se,
    .se_io = st_nci_se_io,
    .hci_load_session = st_nci_hci_load_session,
    .hci_event_received = st_nci_hci_event_received,
    .hci_cmd_received = st_nci_hci_cmd_received,
    .prop_ops = st_nci_prop_ops,
    .n_prop_ops = ARRAY_SIZE(st_nci_prop_ops),
    };
    int st_nci_probe(struct llt_ndlc *ndlc, int phy_headroom,
    int phy_tailroom, struct st_nci_se_status *se_status)
    {
    struct st_nci_info *info;
    int r;
    u32 protocols;
    info = devm_kzalloc(ndlc.dev,
    sizeof(struct st_nci_info), GFP_KERNEL);
    if (!info)
    return -ENOMEM;
    protocols = NFC_PROTO_JEWEL_MASK
    | NFC_PROTO_MIFARE_MASK
    | NFC_PROTO_FELICA_MASK
    | NFC_PROTO_ISO14443_MASK
    | NFC_PROTO_ISO14443_B_MASK
    | NFC_PROTO_ISO15693_MASK
    | NFC_PROTO_NFC_DEP_MASK;
    BUILD_BUG_ON(ARRAY_SIZE(st_nci_prop_ops) > NCI_MAX_PROPRIETARY_CMD);
    ndlc.ndev = nci_allocate_device(&st_nci_ops, protocols,
    phy_headroom, phy_tailroom);
    if (!ndlc.ndev) {
    pr_err("Cannot allocate nfc ndev\n");
    return -ENOMEM;
    }
    info.ndlc = ndlc;
    nci_set_drvdata(ndlc.ndev, info);
    r = st_nci_vendor_cmds_init(ndlc.ndev);
    if (r) {
    pr_err("Cannot register proprietary vendor cmds\n");
    goto err_reg_dev;
    }
    r = nci_register_device(ndlc.ndev);
    if (r) {
    pr_err("Cannot register nfc device to nci core\n");
    goto err_reg_dev;
    }
    return st_nci_se_init(ndlc.ndev, se_status);
    err_reg_dev:
    nci_free_device(ndlc.ndev);
    return r;
    }
    EXPORT_SYMBOL_GPL(st_nci_probe);
#[no_mangle]
pub unsafe extern "C" fn st_nci_remove(ndev: *mut nci_dev) {
    void st_nci_remove(struct nci_dev *ndev)
    {
    struct st_nci_info *info = nci_get_drvdata(ndev);
    ndlc_close(info.ndlc);
    nci_unregister_device(ndev);
    nci_free_device(ndev);
    }
    EXPORT_SYMBOL_GPL(st_nci_remove);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION(DRIVER_DESC);
