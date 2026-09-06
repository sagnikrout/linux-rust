//! Automatically rewritten from C to Rust
//! Source: drivers/bluetooth/btrsi.c
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


// SPDX-License-Identifier: ISC
//
// Copyright (c) 2017 Redpine Signals Inc.
//

pub const RSI_DMA_ALIGN: c_int = 8;
pub const RSI_FRAME_DESC_SIZE: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsi_hci_adapter {
    pub priv: *mut c_void,
    pub proto_ops: *mut rsi_proto_ops,
    pub hdev: *mut hci_dev,
}

#[no_mangle]
unsafe extern "C" fn rsi_hci_open(hdev: *mut hci_dev) -> c_int {
    static int rsi_hci_open(struct hci_dev *hdev)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rsi_hci_close(hdev: *mut hci_dev) -> c_int {
    static int rsi_hci_close(struct hci_dev *hdev)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rsi_hci_flush(hdev: *mut hci_dev) -> c_int {
    static int rsi_hci_flush(struct hci_dev *hdev)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rsi_hci_send_pkt(hdev: *mut hci_dev, skb: *mut sk_buff) -> c_int {
    static int rsi_hci_send_pkt(struct hci_dev *hdev, struct sk_buff *skb)
    {
    struct rsi_hci_adapter *h_adapter = hci_get_drvdata(hdev);
    struct sk_buff *new_skb = core::ptr::null_mut();
    switch (hci_skb_pkt_type(skb)) {
    case HCI_COMMAND_PKT:
    hdev.stat.cmd_tx++;
    break;
    case HCI_ACLDATA_PKT:
    hdev.stat.acl_tx++;
    break;
    case HCI_SCODATA_PKT:
    hdev.stat.sco_tx++;
    break;
    }
    if (skb_headroom(skb) < RSI_HEADROOM_FOR_BT_HAL) {
// Insufficient skb headroom - allocate a new skb
    new_skb = skb_realloc_headroom(skb, RSI_HEADROOM_FOR_BT_HAL);
    if (unlikely(!new_skb))
    return -ENOMEM;
    bt_cb(new_skb).pkt_type = hci_skb_pkt_type(skb);
    kfree_skb(skb);
    skb = new_skb;
    if (!IS_ALIGNED((unsigned long)skb.data, RSI_DMA_ALIGN)) {
    u8 *skb_data = skb.data;
    let mut skb_len: c_int = skb.len;
    skb_push(skb, RSI_DMA_ALIGN);
    skb_pull(skb, PTR_ALIGN(skb.data,
    RSI_DMA_ALIGN) - skb.data);
    memmove(skb.data, skb_data, skb_len);
    skb_trim(skb, skb_len);
    }
    }
    return h_adapter.proto_ops.coex_send_pkt(h_adapter.priv, skb,
    RSI_BT_Q);
    }
#[no_mangle]
unsafe extern "C" fn rsi_hci_recv_pkt(priv: *mut c_void, pkt: *const u8) -> c_int {
    static int rsi_hci_recv_pkt(void *priv, const u8 *pkt)
    {
    struct rsi_hci_adapter *h_adapter = priv;
    struct hci_dev *hdev = h_adapter.hdev;
    struct sk_buff *skb;
    let mut pkt_len: c_int = get_unaligned_le16(pkt) & 0x0fff;
    skb = dev_alloc_skb(pkt_len);
    if (!skb)
    return -ENOMEM;
    memcpy(skb.data, pkt + RSI_FRAME_DESC_SIZE, pkt_len);
    skb_put(skb, pkt_len);
    h_adapter.hdev.stat.byte_rx += skb.len;
    hci_skb_pkt_type(skb) = pkt[14];
    return hci_recv_frame(hdev, skb);
    }
#[no_mangle]
unsafe extern "C" fn rsi_hci_attach(priv: *mut c_void, ops: *mut rsi_proto_ops) -> c_int {
    static int rsi_hci_attach(void *priv, struct rsi_proto_ops *ops)
    {
    struct rsi_hci_adapter *h_adapter = core::ptr::null_mut();
    struct hci_dev *hdev;
    let mut err: c_int = 0;
    h_adapter = kzalloc_obj(*h_adapter);
    if (!h_adapter)
    return -ENOMEM;
    h_adapter.priv = priv;
    h_adapter.proto_ops = ops;
    hdev = hci_alloc_dev();
    if (!hdev) {
    BT_ERR("Failed to alloc HCI device");
    goto err;
    }
    h_adapter.hdev = hdev;
    if (ops.get_host_intf(priv) == RSI_HOST_INTF_SDIO)
    hdev.bus = HCI_SDIO;
    else
    hdev.bus = HCI_USB;
    hci_set_drvdata(hdev, h_adapter);
    hdev.open = rsi_hci_open;
    hdev.close = rsi_hci_close;
    hdev.flush = rsi_hci_flush;
    hdev.send = rsi_hci_send_pkt;
    err = hci_register_dev(hdev);
    if (err < 0) {
    BT_ERR("HCI registration failed with errcode %d", err);
    hci_free_dev(hdev);
    goto err;
    }
    ops.set_bt_context(priv, h_adapter);
    return 0;
    err:
    h_adapter.hdev = core::ptr::null_mut();
    kfree(h_adapter);
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn rsi_hci_detach(priv: *mut c_void) {
    static void rsi_hci_detach(void *priv)
    {
    struct rsi_hci_adapter *h_adapter = priv;
    struct hci_dev *hdev;
    if (!h_adapter)
    return;
    hdev = h_adapter.hdev;
    if (hdev) {
    hci_unregister_dev(hdev);
    hci_free_dev(hdev);
    h_adapter.hdev = core::ptr::null_mut();
    }
    kfree(h_adapter);
    }
    const struct rsi_mod_ops rsi_bt_ops = {
    .attach	= rsi_hci_attach,
    .detach	= rsi_hci_detach,
    .recv_pkt = rsi_hci_recv_pkt,
    };
    EXPORT_SYMBOL(rsi_bt_ops);
#[no_mangle]
unsafe extern "C" fn rsi_91x_bt_module_init() -> c_int {
    static int rsi_91x_bt_module_init(void)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rsi_91x_bt_module_exit() {
    static void rsi_91x_bt_module_exit(void)
    {
    return;
    }
    module_init(rsi_91x_bt_module_init);
    module_exit(rsi_91x_bt_module_exit);
    MODULE_AUTHOR("Redpine Signals Inc");
    MODULE_DESCRIPTION("RSI BT driver");
    MODULE_LICENSE("Dual BSD/GPL");
