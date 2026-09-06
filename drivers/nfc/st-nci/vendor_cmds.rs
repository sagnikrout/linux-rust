//! Automatically rewritten from C to Rust
//! Source: drivers/nfc/st-nci/vendor_cmds.c
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
// Proprietary commands extension for STMicroelectronics NFC NCI Chip
//
// Copyright (C) 2014-2015  STMicroelectronics SAS. All rights reserved.
//

pub const ST_NCI_HCI_DM_GETDATA: c_uint = 0x10;
pub const ST_NCI_HCI_DM_PUTDATA: c_uint = 0x11;
pub const ST_NCI_HCI_DM_LOAD: c_uint = 0x12;
pub const ST_NCI_HCI_DM_GETINFO: c_uint = 0x13;
pub const ST_NCI_HCI_DM_FWUPD_START: c_uint = 0x14;
pub const ST_NCI_HCI_DM_FWUPD_STOP: c_uint = 0x15;
pub const ST_NCI_HCI_DM_UPDATE_AID: c_uint = 0x20;
pub const ST_NCI_HCI_DM_RESET: c_uint = 0x3e;
pub const ST_NCI_HCI_DM_FIELD_GENERATOR: c_uint = 0x32;
pub const ST_NCI_HCI_DM_VDC_MEASUREMENT_VALUE: c_uint = 0x33;
pub const ST_NCI_HCI_DM_VDC_VALUE_COMPARISON: c_uint = 0x34;
pub const ST_NCI_FACTORY_MODE_ON: c_int = 1;
pub const ST_NCI_FACTORY_MODE_OFF: c_int = 0;
pub const ST_NCI_EVT_POST_DATA: c_uint = 0x02;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct get_param_data {
    pub gate: u8,
    pub data: u8,
    pub __packed: },
    static int st_nci_factory_mode(struct nfc_dev *dev, void *data,
    size_t data_len)
    {
    pub nfc_get_drvdata(dev): *mut *mut nci_dev ndev =,
    pub nci_get_drvdata(ndev): *mut *mut st_nci_info info =,
    if (data_len != 1)
    pub -EINVAL: return,
    pub )data)[0]): *mut pr_debug("factory mode: %x\n", ((u8,
    switch (((u8 *)data)[0]) {
    case ST_NCI_FACTORY_MODE_ON:
    pub &info->flags): test_and_set_bit(ST_NCI_FACTORY_MODE,,
    case ST_NCI_FACTORY_MODE_OFF:
    pub &info->flags): clear_bit(ST_NCI_FACTORY_MODE,,
    default:
    pub -EINVAL: return,
    }
    pub 0: return,
    }
    static int st_nci_hci_clear_all_pipes(struct nfc_dev *dev, void *data,
    size_t data_len)
    {
    pub nfc_get_drvdata(dev): *mut *mut nci_dev ndev =,
    pub nci_hci_clear_all_pipes(ndev): return,
    }
    static int st_nci_hci_dm_put_data(struct nfc_dev *dev, void *data,
    size_t data_len)
    {
    pub nfc_get_drvdata(dev): *mut *mut nci_dev ndev =,
    return nci_hci_send_cmd(ndev, ST_NCI_DEVICE_MGNT_GATE,
    ST_NCI_HCI_DM_PUTDATA, data,
    pub NULL): data_len,,
    }
    static int st_nci_hci_dm_update_aid(struct nfc_dev *dev, void *data,
    size_t data_len)
    {
    pub nfc_get_drvdata(dev): *mut *mut nci_dev ndev =,
    return nci_hci_send_cmd(ndev, ST_NCI_DEVICE_MGNT_GATE,
    pub NULL): ST_NCI_HCI_DM_UPDATE_AID, data, data_len,,
    }
    static int st_nci_hci_dm_get_info(struct nfc_dev *dev, void *data,
    size_t data_len)
    {
    pub r: c_int,
    pub skb: *mut *mut sk_buff msg,,
    pub nfc_get_drvdata(dev): *mut *mut nci_dev ndev =,
    r = nci_hci_send_cmd(ndev, ST_NCI_DEVICE_MGNT_GATE, ST_NCI_HCI_DM_GETINFO,
    pub &skb): data, data_len,,
    if (r)
    pub r: return,
    msg = nfc_vendor_cmd_alloc_reply_skb(dev, ST_NCI_VENDOR_OUI,
    pub skb->len): HCI_DM_GET_INFO,,
    if (!msg) {
    pub -ENOMEM: r =,
    pub free_skb: goto,
    }
    if (nla_put(msg, NFC_ATTR_VENDOR_DATA, skb.len, skb.data)) {
    pub -ENOBUFS: r =,
    pub free_skb: goto,
    }
    pub nfc_vendor_cmd_reply(msg): r =,
    free_skb:
    pub r: return,
    }
    static int st_nci_hci_dm_get_data(struct nfc_dev *dev, void *data,
    size_t data_len)
    {
    pub r: c_int,
    pub skb: *mut *mut sk_buff msg,,
    pub nfc_get_drvdata(dev): *mut *mut nci_dev ndev =,
    r = nci_hci_send_cmd(ndev, ST_NCI_DEVICE_MGNT_GATE, ST_NCI_HCI_DM_GETDATA,
    pub &skb): data, data_len,,
    if (r)
    pub r: return,
    msg = nfc_vendor_cmd_alloc_reply_skb(dev, ST_NCI_VENDOR_OUI,
    pub skb->len): HCI_DM_GET_DATA,,
    if (!msg) {
    pub -ENOMEM: r =,
    pub free_skb: goto,
    }
    if (nla_put(msg, NFC_ATTR_VENDOR_DATA, skb.len, skb.data)) {
    pub -ENOBUFS: r =,
    pub free_skb: goto,
    }
    pub nfc_vendor_cmd_reply(msg): r =,
    free_skb:
    pub r: return,
    }
    static int st_nci_hci_dm_fwupd_start(struct nfc_dev *dev, void *data,
    size_t data_len)
    {
    pub r: c_int,
    pub nfc_get_drvdata(dev): *mut *mut nci_dev ndev =,
    pub true: dev->fw_download_in_progress =,
    r = nci_hci_send_cmd(ndev, ST_NCI_DEVICE_MGNT_GATE,
    pub NULL): ST_NCI_HCI_DM_FWUPD_START, data, data_len,,
    if (r)
    pub false: dev->fw_download_in_progress =,
    pub r: return,
    }
    static int st_nci_hci_dm_fwupd_end(struct nfc_dev *dev, void *data,
    size_t data_len)
    {
    pub nfc_get_drvdata(dev): *mut *mut nci_dev ndev =,
    return nci_hci_send_cmd(ndev, ST_NCI_DEVICE_MGNT_GATE,
    pub NULL): ST_NCI_HCI_DM_FWUPD_STOP, data, data_len,,
    }
    static int st_nci_hci_dm_direct_load(struct nfc_dev *dev, void *data,
    size_t data_len)
    {
    pub nfc_get_drvdata(dev): *mut *mut nci_dev ndev =,
    if (dev.fw_download_in_progress) {
    pub false: dev->fw_download_in_progress =,
    return nci_hci_send_cmd(ndev, ST_NCI_DEVICE_MGNT_GATE,
    pub NULL): ST_NCI_HCI_DM_LOAD, data, data_len,,
    }
    pub -EPROTO: return,
    }
    static int st_nci_hci_dm_reset(struct nfc_dev *dev, void *data,
    size_t data_len)
    {
    pub nfc_get_drvdata(dev): *mut *mut nci_dev ndev =,
    nci_hci_send_cmd(ndev, ST_NCI_DEVICE_MGNT_GATE,
    pub NULL): ST_NCI_HCI_DM_RESET, data, data_len,,
    pub 0: return,
    }
    static int st_nci_hci_get_param(struct nfc_dev *dev, void *data,
    size_t data_len)
    {
    pub r: c_int,
    pub skb: *mut *mut sk_buff msg,,
    pub nfc_get_drvdata(dev): *mut *mut nci_dev ndev =,
    pub )data: *mut *mut get_param_data param = (get_param_data,
    if (data_len < sizeof(struct get_param_data))
    pub -EPROTO: return,
    pub &skb): r = nci_hci_get_param(ndev, param->gate, param->data,,
    if (r)
    pub r: return,
    msg = nfc_vendor_cmd_alloc_reply_skb(dev, ST_NCI_VENDOR_OUI,
    pub skb->len): HCI_GET_PARAM,,
    if (!msg) {
    pub -ENOMEM: r =,
    pub free_skb: goto,
    }
    if (nla_put(msg, NFC_ATTR_VENDOR_DATA, skb.len, skb.data)) {
    pub -ENOBUFS: r =,
    pub free_skb: goto,
    }
    pub nfc_vendor_cmd_reply(msg): r =,
    free_skb:
    pub r: return,
    }
    static int st_nci_hci_dm_field_generator(struct nfc_dev *dev, void *data,
    size_t data_len)
    {
    pub nfc_get_drvdata(dev): *mut *mut nci_dev ndev =,
    return nci_hci_send_cmd(ndev, ST_NCI_DEVICE_MGNT_GATE,
    pub NULL): ST_NCI_HCI_DM_FIELD_GENERATOR, data, data_len,,
    }
    static int st_nci_hci_dm_vdc_measurement_value(struct nfc_dev *dev, void *data,
    size_t data_len)
    {
    pub r: c_int,
    pub skb: *mut *mut sk_buff msg,,
    pub nfc_get_drvdata(dev): *mut *mut nci_dev ndev =,
    if (data_len != 4)
    pub -EPROTO: return,
    r = nci_hci_send_cmd(ndev, ST_NCI_DEVICE_MGNT_GATE,
    ST_NCI_HCI_DM_VDC_MEASUREMENT_VALUE,
    pub &skb): data, data_len,,
    if (r)
    pub r: return,
    msg = nfc_vendor_cmd_alloc_reply_skb(dev, ST_NCI_VENDOR_OUI,
    pub skb->len): HCI_DM_VDC_MEASUREMENT_VALUE,,
    if (!msg) {
    pub -ENOMEM: r =,
    pub free_skb: goto,
    }
    if (nla_put(msg, NFC_ATTR_VENDOR_DATA, skb.len, skb.data)) {
    pub -ENOBUFS: r =,
    pub free_skb: goto,
    }
    pub nfc_vendor_cmd_reply(msg): r =,
    free_skb:
    pub r: return,
    }
    static int st_nci_hci_dm_vdc_value_comparison(struct nfc_dev *dev, void *data,
    size_t data_len)
    {
    pub r: c_int,
    pub skb: *mut *mut sk_buff msg,,
    pub nfc_get_drvdata(dev): *mut *mut nci_dev ndev =,
    if (data_len != 2)
    pub -EPROTO: return,
    r = nci_hci_send_cmd(ndev, ST_NCI_DEVICE_MGNT_GATE,
    ST_NCI_HCI_DM_VDC_VALUE_COMPARISON,
    pub &skb): data, data_len,,
    if (r)
    pub r: return,
    msg = nfc_vendor_cmd_alloc_reply_skb(dev, ST_NCI_VENDOR_OUI,
    pub skb->len): HCI_DM_VDC_VALUE_COMPARISON,,
    if (!msg) {
    pub -ENOMEM: r =,
    pub free_skb: goto,
    }
    if (nla_put(msg, NFC_ATTR_VENDOR_DATA, skb.len, skb.data)) {
    pub -ENOBUFS: r =,
    pub free_skb: goto,
    }
    pub nfc_vendor_cmd_reply(msg): r =,
    free_skb:
    pub r: return,
    }
    static int st_nci_loopback(struct nfc_dev *dev, void *data,
    size_t data_len)
    {
    pub r: c_int,
    pub skb: *mut *mut sk_buff msg,,
    pub nfc_get_drvdata(dev): *mut *mut nci_dev ndev =,
    if (data_len <= 0)
    pub -EPROTO: return,
    pub &skb): r = nci_nfcc_loopback(ndev, data, data_len,,
    if (r < 0)
    pub r: return,
    msg = nfc_vendor_cmd_alloc_reply_skb(dev, ST_NCI_VENDOR_OUI,
    pub skb->len): LOOPBACK,,
    if (!msg) {
    pub -ENOMEM: r =,
    pub free_skb: goto,
    }
    if (nla_put(msg, NFC_ATTR_VENDOR_DATA, skb.len, skb.data)) {
    pub -ENOBUFS: r =,
    pub free_skb: goto,
    }
    pub nfc_vendor_cmd_reply(msg): r =,
    free_skb:
    pub r: return,
    }
    static int st_nci_manufacturer_specific(struct nfc_dev *dev, void *data,
    size_t data_len)
    {
    pub msg: *mut sk_buff,
    pub nfc_get_drvdata(dev): *mut *mut nci_dev ndev =,
    msg = nfc_vendor_cmd_alloc_reply_skb(dev, ST_NCI_VENDOR_OUI,
    MANUFACTURER_SPECIFIC,
    if (!msg)
    pub -ENOMEM: return,
    if (nla_put(msg, NFC_ATTR_VENDOR_DATA, sizeof(ndev.manufact_specific_info),
    &ndev.manufact_specific_info)) {
    pub -ENOBUFS: return,
    }
    pub nfc_vendor_cmd_reply(msg): return,
    }
    static const struct nfc_vendor_cmd st_nci_vendor_cmds[] = {
    {
    .vendor_id = ST_NCI_VENDOR_OUI,
    .subcmd = FACTORY_MODE,
    .doit = st_nci_factory_mode,
    },
    {
    .vendor_id = ST_NCI_VENDOR_OUI,
    .subcmd = HCI_CLEAR_ALL_PIPES,
    .doit = st_nci_hci_clear_all_pipes,
    },
    {
    .vendor_id = ST_NCI_VENDOR_OUI,
    .subcmd = HCI_DM_PUT_DATA,
    .doit = st_nci_hci_dm_put_data,
    },
    {
    .vendor_id = ST_NCI_VENDOR_OUI,
    .subcmd = HCI_DM_UPDATE_AID,
    .doit = st_nci_hci_dm_update_aid,
    },
    {
    .vendor_id = ST_NCI_VENDOR_OUI,
    .subcmd = HCI_DM_GET_INFO,
    .doit = st_nci_hci_dm_get_info,
    },
    {
    .vendor_id = ST_NCI_VENDOR_OUI,
    .subcmd = HCI_DM_GET_DATA,
    .doit = st_nci_hci_dm_get_data,
    },
    {
    .vendor_id = ST_NCI_VENDOR_OUI,
    .subcmd = HCI_DM_DIRECT_LOAD,
    .doit = st_nci_hci_dm_direct_load,
    },
    {
    .vendor_id = ST_NCI_VENDOR_OUI,
    .subcmd = HCI_DM_RESET,
    .doit = st_nci_hci_dm_reset,
    },
    {
    .vendor_id = ST_NCI_VENDOR_OUI,
    .subcmd = HCI_GET_PARAM,
    .doit = st_nci_hci_get_param,
    },
    {
    .vendor_id = ST_NCI_VENDOR_OUI,
    .subcmd = HCI_DM_FIELD_GENERATOR,
    .doit = st_nci_hci_dm_field_generator,
    },
    {
    .vendor_id = ST_NCI_VENDOR_OUI,
    .subcmd = HCI_DM_FWUPD_START,
    .doit = st_nci_hci_dm_fwupd_start,
    },
    {
    .vendor_id = ST_NCI_VENDOR_OUI,
    .subcmd = HCI_DM_FWUPD_END,
    .doit = st_nci_hci_dm_fwupd_end,
    },
    {
    .vendor_id = ST_NCI_VENDOR_OUI,
    .subcmd = LOOPBACK,
    .doit = st_nci_loopback,
    },
    {
    .vendor_id = ST_NCI_VENDOR_OUI,
    .subcmd = HCI_DM_VDC_MEASUREMENT_VALUE,
    .doit = st_nci_hci_dm_vdc_measurement_value,
    },
    {
    .vendor_id = ST_NCI_VENDOR_OUI,
    .subcmd = HCI_DM_VDC_VALUE_COMPARISON,
    .doit = st_nci_hci_dm_vdc_value_comparison,
    },
    {
    .vendor_id = ST_NCI_VENDOR_OUI,
    .subcmd = MANUFACTURER_SPECIFIC,
    .doit = st_nci_manufacturer_specific,
    },
}

#[no_mangle]
pub unsafe extern "C" fn st_nci_vendor_cmds_init(ndev: *mut nci_dev) -> c_int {
    int st_nci_vendor_cmds_init(struct nci_dev *ndev)
    {
    return nci_set_vendor_cmds(ndev, st_nci_vendor_cmds,
    sizeof(st_nci_vendor_cmds));
    }
    EXPORT_SYMBOL(st_nci_vendor_cmds_init);
