//! Automatically rewritten from C to Rust
//! Source: drivers/bluetooth/hci_ag6xx.c
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
// Bluetooth HCI UART driver for Intel/AG6xx devices
//
// Copyright (C) 2016  Intel Corporation
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ag6xx_data {
    pub rx_skb: *mut sk_buff,
    pub txq: sk_buff_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pbn_entry {
    pub addr: __le32,
    pub plen: __le32,
    pub data: [__u8; ],
    pub __packed: },
#[no_mangle]
unsafe extern "C" fn ag6xx_open(hu: *mut hci_uart) -> c_int {
    static int ag6xx_open(struct hci_uart *hu)
    {
    pub ag6xx: *mut ag6xx_data,
    pub hu): BT_DBG("hu %p",,
    pub kzalloc_obj(*ag6xx): *mut ag6xx =,
    if (!ag6xx)
    pub -ENOMEM: return,
    pub ag6xx: hu->priv =,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn ag6xx_close(hu: *mut hci_uart) -> c_int {
    static int ag6xx_close(struct hci_uart *hu)
    {
    pub hu->priv: *mut *mut ag6xx_data ag6xx =,
    pub hu): BT_DBG("hu %p",,
    pub NULL: hu->priv =,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn ag6xx_flush(hu: *mut hci_uart) -> c_int {
    static int ag6xx_flush(struct hci_uart *hu)
    {
    pub hu->priv: *mut *mut ag6xx_data ag6xx =,
    pub hu): BT_DBG("hu %p",,
    pub 0: return,
    }
    static struct sk_buff *ag6xx_dequeue(struct hci_uart *hu)
    {
    pub hu->priv: *mut *mut ag6xx_data ag6xx =,
    pub skb: *mut sk_buff,
    pub skb_dequeue(&ag6xx->txq): skb =,
    if (!skb)
    pub skb: return,
// Prepend skb with frame type
    pub 1): memcpy(skb_push(skb, 1), &bt_cb(skb)->pkt_type,,
    pub skb: return,
    }
#[no_mangle]
unsafe extern "C" fn ag6xx_enqueue(hu: *mut hci_uart, skb: *mut sk_buff) -> c_int {
    static int ag6xx_enqueue(struct hci_uart *hu, struct sk_buff *skb)
    {
    pub hu->priv: *mut *mut ag6xx_data ag6xx =,
    pub skb): skb_queue_tail(&ag6xx->txq,,
    pub 0: return,
    }
    static const struct h4_recv_pkt ag6xx_recv_pkts[] = {
    { H4_RECV_ACL,    .recv = hci_recv_frame   },
    { H4_RECV_SCO,    .recv = hci_recv_frame   },
    { H4_RECV_EVENT,  .recv = hci_recv_frame   },
}

#[no_mangle]
unsafe extern "C" fn ag6xx_recv(hu: *mut hci_uart, data: *const c_void, count: c_int) -> c_int {
    static int ag6xx_recv(struct hci_uart *hu, const void *data, int count)
    {
    struct ag6xx_data *ag6xx = hu.priv;
    if (!test_bit(HCI_UART_REGISTERED, &hu.flags))
    return -EUNATCH;
    ag6xx.rx_skb = h4_recv_buf(hu, ag6xx.rx_skb, data, count,
    ag6xx_recv_pkts,
    ARRAY_SIZE(ag6xx_recv_pkts));
    if (IS_ERR(ag6xx.rx_skb)) {
    let mut err: c_int = PTR_ERR(ag6xx.rx_skb);
    bt_dev_err(hu.hdev, "Frame reassembly failed (%d)", err);
    ag6xx.rx_skb = core::ptr::null_mut();
    return err;
    }
    return count;
    }
    static int intel_mem_write(struct hci_dev *hdev, u32 addr, u32 plen,
    const void *data)
    {
// Can write a maximum of 247 bytes per HCI command.
// HCI cmd Header (3), Intel mem write header (6), data (247).
//
    while (plen > 0) {
    struct sk_buff *skb;
    u8 cmd_param[253], fragment_len = (plen > 247) ? 247 : plen;
    let mut leaddr: __le32 = cpu_to_le32(addr);
    memcpy(cmd_param, &leaddr, 4);
    cmd_param[4] = 0;
    cmd_param[5] = fragment_len;
    memcpy(cmd_param + 6, data, fragment_len);
    skb = __hci_cmd_sync(hdev, 0xfc8e, fragment_len + 6, cmd_param,
    HCI_INIT_TIMEOUT);
    if (IS_ERR(skb))
    return PTR_ERR(skb);
    kfree_skb(skb);
    plen -= fragment_len;
    data += fragment_len;
    addr += fragment_len;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ag6xx_setup(hu: *mut hci_uart) -> c_int {
    static int ag6xx_setup(struct hci_uart *hu)
    {
    struct hci_dev *hdev = hu.hdev;
    struct sk_buff *skb;
    struct intel_version ver;
    const struct firmware *fw;
    const u8 *fw_ptr;
    char fwname[64];
    let mut patched: bool = false;
    int err;
    hu.hdev.set_diag = btintel_set_diag;
    hu.hdev.set_bdaddr = btintel_set_bdaddr;
    err = btintel_enter_mfg(hdev);
    if (err)
    return err;
    err = btintel_read_version(hdev, &ver);
    if (err)
    return err;
    btintel_version_info(hdev, &ver);
// The hardware platform number has a fixed value of 0x37 and
// for now only accept this single value.
//
    if (ver.hw_platform != 0x37) {
    bt_dev_err(hdev, "Unsupported Intel hardware platform: 0x%X",
    ver.hw_platform);
    return -EINVAL;
    }
// Only the hardware variant iBT 2.1 (AG6XX) is supported by this
// firmware setup method.
//
    if (ver.hw_variant != 0x0a) {
    bt_dev_err(hdev, "Unsupported Intel hardware variant: 0x%x",
    ver.hw_variant);
    return -EINVAL;
    }
    snprintf(fwname, sizeof(fwname), "intel/ibt-hw-%x.%x.bddata",
    ver.hw_platform, ver.hw_variant);
    err = request_firmware(&fw, fwname, &hdev.dev);
    if (err < 0) {
    bt_dev_err(hdev, "Failed to open Intel bddata file: %s (%d)",
    fwname, err);
    goto patch;
    }
    bt_dev_info(hdev, "Applying bddata (%s)", fwname);
    skb = __hci_cmd_sync_ev(hdev, 0xfc2f, fw.size, fw.data,
    HCI_EV_CMD_STATUS, HCI_CMD_TIMEOUT);
    if (IS_ERR(skb)) {
    bt_dev_err(hdev, "Applying bddata failed (%ld)", PTR_ERR(skb));
    release_firmware(fw);
    return PTR_ERR(skb);
    }
    kfree_skb(skb);
    release_firmware(fw);
    patch:
// If there is no applied patch, fw_patch_num is always 0x00. In other
// cases, current firmware is already patched. No need to patch it.
//
    if (ver.fw_patch_num) {
    bt_dev_info(hdev, "Device is already patched. patch num: %02x",
    ver.fw_patch_num);
    patched = true;
    goto complete;
    }
    snprintf(fwname, sizeof(fwname),
    "intel/ibt-hw-%x.%x.%x-fw-%x.%x.%x.%x.%x.pbn",
    ver.hw_platform, ver.hw_variant, ver.hw_revision,
    ver.fw_variant,  ver.fw_revision, ver.fw_build_num,
    ver.fw_build_ww, ver.fw_build_yy);
    err = request_firmware(&fw, fwname, &hdev.dev);
    if (err < 0) {
    bt_dev_err(hdev, "Failed to open Intel patch file: %s(%d)",
    fwname, err);
    goto complete;
    }
    fw_ptr = fw.data;
    bt_dev_info(hdev, "Patching firmware file (%s)", fwname);
// PBN patch file contains a list of binary patches to be applied on top
// of the embedded firmware. Each patch entry header contains the target
// address and patch size.
//
// Patch entry:
// | addr(le) | patch_len(le) | patch_data |
// | 4 Bytes  |    4 Bytes    |   n Bytes  |
//
// PBN file is terminated by a patch entry whose address is 0xffffffff.
//
    while (fw.size > fw_ptr - fw.data) {
    struct pbn_entry *pbn = (void *)fw_ptr;
    u32 addr, plen;
    if (pbn.addr == 0xffffffff) {
    bt_dev_info(hdev, "Patching complete");
    patched = true;
    break;
    }
    addr = le32_to_cpu(pbn.addr);
    plen = le32_to_cpu(pbn.plen);
    if (fw.data + fw.size <= pbn.data + plen) {
    bt_dev_info(hdev, "Invalid patch len (%d)", plen);
    break;
    }
    bt_dev_info(hdev, "Patching %td/%zu", (fw_ptr - fw.data),
    fw.size);
    err = intel_mem_write(hdev, addr, plen, pbn.data);
    if (err) {
    bt_dev_err(hdev, "Patching failed");
    break;
    }
    fw_ptr = pbn.data + plen;
    }
    release_firmware(fw);
    complete:
// Exit manufacturing mode and reset
    err = btintel_exit_mfg(hdev, true, patched);
    if (err)
    return err;
// Set the event mask for Intel specific vendor events. This enables
// a few extra events that are useful during general operation.
//
    btintel_set_event_mask_mfg(hdev, false);
    btintel_check_bdaddr(hdev);
    return 0;
    }
    static const struct hci_uart_proto ag6xx_proto = {
    .id		= HCI_UART_AG6XX,
    .name		= "AG6XX",
    .manufacturer	= 2,
    .open		= ag6xx_open,
    .close		= ag6xx_close,
    .flush		= ag6xx_flush,
    .setup		= ag6xx_setup,
    .recv		= ag6xx_recv,
    .enqueue	= ag6xx_enqueue,
    .dequeue	= ag6xx_dequeue,
    };
#[no_mangle]
pub unsafe extern "C" fn ag6xx_init() -> int __init {
    int __init ag6xx_init(void)
    {
    return hci_uart_register_proto(&ag6xx_proto);
    }
#[no_mangle]
pub unsafe extern "C" fn ag6xx_deinit() -> int __exit {
    int __exit ag6xx_deinit(void)
    {
    return hci_uart_unregister_proto(&ag6xx_proto);
    }
