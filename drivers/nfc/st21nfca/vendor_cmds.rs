//! Automatically rewritten from C to Rust
//! Source: drivers/nfc/st21nfca/vendor_cmds.c
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
// Proprietary commands extension for STMicroelectronics NFC Chip
//
// Copyright (C) 2014-2015  STMicroelectronics SAS. All rights reserved.
//

pub const ST21NFCA_HCI_DM_GETDATA: c_uint = 0x10;
pub const ST21NFCA_HCI_DM_PUTDATA: c_uint = 0x11;
pub const ST21NFCA_HCI_DM_LOAD: c_uint = 0x12;
pub const ST21NFCA_HCI_DM_GETINFO: c_uint = 0x13;
pub const ST21NFCA_HCI_DM_UPDATE_AID: c_uint = 0x20;
pub const ST21NFCA_HCI_DM_RESET: c_uint = 0x3e;
pub const ST21NFCA_HCI_DM_FIELD_GENERATOR: c_uint = 0x32;
pub const ST21NFCA_FACTORY_MODE_ON: c_int = 1;
pub const ST21NFCA_FACTORY_MODE_OFF: c_int = 0;
pub const ST21NFCA_EVT_POST_DATA: c_uint = 0x02;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct get_param_data {
    pub gate: u8,
    pub data: u8,
    pub __packed: },
    static int st21nfca_factory_mode(struct nfc_dev *dev, void *data,
    size_t data_len)
    {
    pub nfc_get_drvdata(dev): *mut *mut nfc_hci_dev hdev =,
    if (data_len != 1)
    pub -EINVAL: return,
    pub )data)[0]): *mut pr_debug("factory mode: %x\n", ((u8,
    switch (((u8 *)data)[0]) {
    case ST21NFCA_FACTORY_MODE_ON:
    pub &hdev->quirks): test_and_set_bit(ST21NFCA_FACTORY_MODE,,
    case ST21NFCA_FACTORY_MODE_OFF:
    pub &hdev->quirks): clear_bit(ST21NFCA_FACTORY_MODE,,
    default:
    pub -EINVAL: return,
    }
    pub 0: return,
    }
    static int st21nfca_hci_clear_all_pipes(struct nfc_dev *dev, void *data,
    size_t data_len)
    {
    pub nfc_get_drvdata(dev): *mut *mut nfc_hci_dev hdev =,
    pub nfc_hci_disconnect_all_gates(hdev): return,
    }
    static int st21nfca_hci_dm_put_data(struct nfc_dev *dev, void *data,
    size_t data_len)
    {
    pub nfc_get_drvdata(dev): *mut *mut nfc_hci_dev hdev =,
    return nfc_hci_send_cmd(hdev, ST21NFCA_DEVICE_MGNT_GATE,
    ST21NFCA_HCI_DM_PUTDATA, data,
    pub NULL): data_len,,
    }
    static int st21nfca_hci_dm_update_aid(struct nfc_dev *dev, void *data,
    size_t data_len)
    {
    pub nfc_get_drvdata(dev): *mut *mut nfc_hci_dev hdev =,
    return nfc_hci_send_cmd(hdev, ST21NFCA_DEVICE_MGNT_GATE,
    pub NULL): ST21NFCA_HCI_DM_UPDATE_AID, data, data_len,,
    }
    static int st21nfca_hci_dm_get_info(struct nfc_dev *dev, void *data,
    size_t data_len)
    {
    pub r: c_int,
    pub skb: *mut *mut sk_buff msg,,
    pub nfc_get_drvdata(dev): *mut *mut nfc_hci_dev hdev =,
    r = nfc_hci_send_cmd(hdev,
    ST21NFCA_DEVICE_MGNT_GATE,
    ST21NFCA_HCI_DM_GETINFO,
    pub &skb): data, data_len,,
    if (r)
    pub exit: goto,
    msg = nfc_vendor_cmd_alloc_reply_skb(dev, ST21NFCA_VENDOR_OUI,
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
    exit:
    pub r: return,
    }
    static int st21nfca_hci_dm_get_data(struct nfc_dev *dev, void *data,
    size_t data_len)
    {
    pub r: c_int,
    pub skb: *mut *mut sk_buff msg,,
    pub nfc_get_drvdata(dev): *mut *mut nfc_hci_dev hdev =,
    r = nfc_hci_send_cmd(hdev,
    ST21NFCA_DEVICE_MGNT_GATE,
    ST21NFCA_HCI_DM_GETDATA,
    pub &skb): data, data_len,,
    if (r)
    pub exit: goto,
    msg = nfc_vendor_cmd_alloc_reply_skb(dev, ST21NFCA_VENDOR_OUI,
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
    exit:
    pub r: return,
    }
    static int st21nfca_hci_dm_load(struct nfc_dev *dev, void *data,
    size_t data_len)
    {
    pub nfc_get_drvdata(dev): *mut *mut nfc_hci_dev hdev =,
    return nfc_hci_send_cmd(hdev, ST21NFCA_DEVICE_MGNT_GATE,
    pub NULL): ST21NFCA_HCI_DM_LOAD, data, data_len,,
    }
    static int st21nfca_hci_dm_reset(struct nfc_dev *dev, void *data,
    size_t data_len)
    {
    pub r: c_int,
    pub nfc_get_drvdata(dev): *mut *mut nfc_hci_dev hdev =,
    r = nfc_hci_send_cmd_async(hdev, ST21NFCA_DEVICE_MGNT_GATE,
    pub NULL): ST21NFCA_HCI_DM_RESET, data, data_len, NULL,,
    if (r < 0)
    pub r: return,
    pub nfc_llc_stop(hdev->llc): r =,
    if (r < 0)
    pub r: return,
    pub nfc_llc_start(hdev->llc): return,
    }
    static int st21nfca_hci_get_param(struct nfc_dev *dev, void *data,
    size_t data_len)
    {
    pub r: c_int,
    pub skb: *mut *mut sk_buff msg,,
    pub nfc_get_drvdata(dev): *mut *mut nfc_hci_dev hdev =,
    pub )data: *mut *mut get_param_data param = (get_param_data,
    if (data_len < sizeof(struct get_param_data))
    pub -EPROTO: return,
    pub &skb): r = nfc_hci_get_param(hdev, param->gate, param->data,,
    if (r)
    pub exit: goto,
    msg = nfc_vendor_cmd_alloc_reply_skb(dev, ST21NFCA_VENDOR_OUI,
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
    exit:
    pub r: return,
    }
    static int st21nfca_hci_dm_field_generator(struct nfc_dev *dev, void *data,
    size_t data_len)
    {
    pub nfc_get_drvdata(dev): *mut *mut nfc_hci_dev hdev =,
    return nfc_hci_send_cmd(hdev,
    ST21NFCA_DEVICE_MGNT_GATE,
    ST21NFCA_HCI_DM_FIELD_GENERATOR,
    pub NULL): data, data_len,,
    }
    int st21nfca_hci_loopback_event_received(struct nfc_hci_dev *hdev, u8 event,
    struct sk_buff *skb)
    {
    pub nfc_hci_get_clientdata(hdev): *mut *mut st21nfca_hci_info info =,
    switch (event) {
    case ST21NFCA_EVT_POST_DATA:
    pub skb: info->vendor_info.rx_skb =,
    default:
    pub gate\n"): nfc_err(&hdev->ndev->dev, "Unexpected event on loopback,
    }
    pub 0: return,
    }
    static int st21nfca_hci_loopback(struct nfc_dev *dev, void *data,
    size_t data_len)
    {
    pub r: c_int,
    pub msg: *mut sk_buff,
    pub nfc_get_drvdata(dev): *mut *mut nfc_hci_dev hdev =,
    pub nfc_hci_get_clientdata(hdev): *mut *mut st21nfca_hci_info info =,
    if (data_len <= 0)
    pub -EPROTO: return,
    pub NULL: info->vendor_info.rx_skb =,
    r = nfc_hci_send_event(hdev, NFC_HCI_LOOPBACK_GATE,
    pub data_len): ST21NFCA_EVT_POST_DATA, data,,
    if (r < 0) {
    pub -EPROTO: r =,
    pub exit: goto,
    }
    if (!info.vendor_info.rx_skb ||
    info.vendor_info.rx_skb.len != data_len) {
    pub -EPROTO: r =,
    pub exit: goto,
    }
    msg = nfc_vendor_cmd_alloc_reply_skb(hdev.ndev,
    ST21NFCA_VENDOR_OUI,
    HCI_LOOPBACK,
    if (!msg) {
    pub -ENOMEM: r =,
    pub free_skb: goto,
    }
    if (nla_put(msg, NFC_ATTR_VENDOR_DATA, info.vendor_info.rx_skb.len,
    info.vendor_info.rx_skb.data)) {
    pub -ENOBUFS: r =,
    pub free_skb: goto,
    }
    pub nfc_vendor_cmd_reply(msg): r =,
    free_skb:
    exit:
    pub r: return,
    }
    static const struct nfc_vendor_cmd st21nfca_vendor_cmds[] = {
    {
    .vendor_id = ST21NFCA_VENDOR_OUI,
    .subcmd = FACTORY_MODE,
    .doit = st21nfca_factory_mode,
    },
    {
    .vendor_id = ST21NFCA_VENDOR_OUI,
    .subcmd = HCI_CLEAR_ALL_PIPES,
    .doit = st21nfca_hci_clear_all_pipes,
    },
    {
    .vendor_id = ST21NFCA_VENDOR_OUI,
    .subcmd = HCI_DM_PUT_DATA,
    .doit = st21nfca_hci_dm_put_data,
    },
    {
    .vendor_id = ST21NFCA_VENDOR_OUI,
    .subcmd = HCI_DM_UPDATE_AID,
    .doit = st21nfca_hci_dm_update_aid,
    },
    {
    .vendor_id = ST21NFCA_VENDOR_OUI,
    .subcmd = HCI_DM_GET_INFO,
    .doit = st21nfca_hci_dm_get_info,
    },
    {
    .vendor_id = ST21NFCA_VENDOR_OUI,
    .subcmd = HCI_DM_GET_DATA,
    .doit = st21nfca_hci_dm_get_data,
    },
    {
    .vendor_id = ST21NFCA_VENDOR_OUI,
    .subcmd = HCI_DM_LOAD,
    .doit = st21nfca_hci_dm_load,
    },
    {
    .vendor_id = ST21NFCA_VENDOR_OUI,
    .subcmd = HCI_DM_RESET,
    .doit = st21nfca_hci_dm_reset,
    },
    {
    .vendor_id = ST21NFCA_VENDOR_OUI,
    .subcmd = HCI_GET_PARAM,
    .doit = st21nfca_hci_get_param,
    },
    {
    .vendor_id = ST21NFCA_VENDOR_OUI,
    .subcmd = HCI_DM_FIELD_GENERATOR,
    .doit = st21nfca_hci_dm_field_generator,
    },
    {
    .vendor_id = ST21NFCA_VENDOR_OUI,
    .subcmd = HCI_LOOPBACK,
    .doit = st21nfca_hci_loopback,
    },
}

#[no_mangle]
pub unsafe extern "C" fn st21nfca_vendor_cmds_init(hdev: *mut nfc_hci_dev) -> c_int {
    int st21nfca_vendor_cmds_init(struct nfc_hci_dev *hdev)
    {
    struct st21nfca_hci_info *info = nfc_hci_get_clientdata(hdev);
    init_completion(&info.vendor_info.req_completion);
    return nfc_hci_set_vendor_cmds(hdev, st21nfca_vendor_cmds,
    sizeof(st21nfca_vendor_cmds));
    }
    EXPORT_SYMBOL(st21nfca_vendor_cmds_init);
