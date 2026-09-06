//! Automatically rewritten from C to Rust
//! Source: drivers/bluetooth/btqcomsmd.c
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
// Copyright (c) 2016, Linaro Ltd.
// Copyright (c) 2015, Sony Mobile Communications Inc.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btqcomsmd {
    pub hdev: *mut hci_dev,
    pub acl_channel: *mut rpmsg_endpoint,
    pub cmd_channel: *mut rpmsg_endpoint,
}

    static int btqcomsmd_recv(struct hci_dev *hdev, unsigned int type,
    const void *data, size_t count)
    {
    struct sk_buff *skb;
// Use GFP_ATOMIC as we're in IRQ context
    skb = bt_skb_alloc(count, GFP_ATOMIC);
    if (!skb) {
    hdev.stat.err_rx++;
    return -ENOMEM;
    }
    hci_skb_pkt_type(skb) = type;
    skb_put_data(skb, data, count);
    return hci_recv_frame(hdev, skb);
    }
    static int btqcomsmd_acl_callback(struct rpmsg_device *rpdev, void *data,
    int count, void *priv, u32 addr)
    {
    struct btqcomsmd *btq = priv;
    btq.hdev.stat.byte_rx += count;
    return btqcomsmd_recv(btq.hdev, HCI_ACLDATA_PKT, data, count);
    }
    static int btqcomsmd_cmd_callback(struct rpmsg_device *rpdev, void *data,
    int count, void *priv, u32 addr)
    {
    struct btqcomsmd *btq = priv;
    btq.hdev.stat.byte_rx += count;
    return btqcomsmd_recv(btq.hdev, HCI_EVENT_PKT, data, count);
    }
#[no_mangle]
unsafe extern "C" fn btqcomsmd_send(hdev: *mut hci_dev, skb: *mut sk_buff) -> c_int {
    static int btqcomsmd_send(struct hci_dev *hdev, struct sk_buff *skb)
    {
    struct btqcomsmd *btq = hci_get_drvdata(hdev);
    int ret;
    switch (hci_skb_pkt_type(skb)) {
    case HCI_ACLDATA_PKT:
    ret = rpmsg_send(btq.acl_channel, skb.data, skb.len);
    if (ret) {
    hdev.stat.err_tx++;
    break;
    }
    hdev.stat.acl_tx++;
    hdev.stat.byte_tx += skb.len;
    break;
    case HCI_COMMAND_PKT:
    ret = rpmsg_send(btq.cmd_channel, skb.data, skb.len);
    if (ret) {
    hdev.stat.err_tx++;
    break;
    }
    hdev.stat.cmd_tx++;
    hdev.stat.byte_tx += skb.len;
    break;
    default:
    ret = -EILSEQ;
    break;
    }
    if (!ret)
    kfree_skb(skb);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn btqcomsmd_open(hdev: *mut hci_dev) -> c_int {
    static int btqcomsmd_open(struct hci_dev *hdev)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn btqcomsmd_close(hdev: *mut hci_dev) -> c_int {
    static int btqcomsmd_close(struct hci_dev *hdev)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn btqcomsmd_setup(hdev: *mut hci_dev) -> c_int {
    static int btqcomsmd_setup(struct hci_dev *hdev)
    {
    struct sk_buff *skb;
    skb = __hci_cmd_sync(hdev, HCI_OP_RESET, 0, core::ptr::null_mut(), HCI_INIT_TIMEOUT);
    if (IS_ERR(skb))
    return PTR_ERR(skb);
    kfree_skb(skb);
// Devices do not have persistent storage for BD address. Retrieve
// it from the firmware node property.
//
    hci_set_quirk(hdev, HCI_QUIRK_USE_BDADDR_PROPERTY);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn btqcomsmd_set_bdaddr(hdev: *mut hci_dev, bdaddr: *const bdaddr_t) -> c_int {
    static int btqcomsmd_set_bdaddr(struct hci_dev *hdev, const bdaddr_t *bdaddr)
    {
    int ret;
    ret = qca_set_bdaddr_rome(hdev, bdaddr);
    if (ret)
    return ret;
// The firmware stops responding for a while after setting the bdaddr,
// causing timeouts for subsequent commands. Sleep a bit to avoid this.
//
    usleep_range(1000, 10000);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn btqcomsmd_probe(pdev: *mut platform_device) -> c_int {
    static int btqcomsmd_probe(struct platform_device *pdev)
    {
    struct btqcomsmd *btq;
    struct hci_dev *hdev;
    void *wcnss;
    int ret;
    btq = devm_kzalloc(&pdev.dev, sizeof(*btq), GFP_KERNEL);
    if (!btq)
    return -ENOMEM;
    wcnss = dev_get_drvdata(pdev.dev.parent);
    btq.acl_channel = qcom_wcnss_open_channel(wcnss, "APPS_RIVA_BT_ACL",
    btqcomsmd_acl_callback, btq);
    if (IS_ERR(btq.acl_channel))
    return PTR_ERR(btq.acl_channel);
    btq.cmd_channel = qcom_wcnss_open_channel(wcnss, "APPS_RIVA_BT_CMD",
    btqcomsmd_cmd_callback, btq);
    if (IS_ERR(btq.cmd_channel)) {
    ret = PTR_ERR(btq.cmd_channel);
    goto destroy_acl_channel;
    }
    hdev = hci_alloc_dev();
    if (!hdev) {
    ret = -ENOMEM;
    goto destroy_cmd_channel;
    }
    hci_set_drvdata(hdev, btq);
    btq.hdev = hdev;
    SET_HCIDEV_DEV(hdev, &pdev.dev);
    hdev.bus = HCI_SMD;
    hdev.open = btqcomsmd_open;
    hdev.close = btqcomsmd_close;
    hdev.send = btqcomsmd_send;
    hdev.setup = btqcomsmd_setup;
    hdev.set_bdaddr = btqcomsmd_set_bdaddr;
    ret = hci_register_dev(hdev);
    if (ret < 0)
    goto hci_free_dev;
    platform_set_drvdata(pdev, btq);
    return 0;
    hci_free_dev:
    hci_free_dev(hdev);
    destroy_cmd_channel:
    rpmsg_destroy_ept(btq.cmd_channel);
    destroy_acl_channel:
    rpmsg_destroy_ept(btq.acl_channel);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn btqcomsmd_remove(pdev: *mut platform_device) {
    static void btqcomsmd_remove(struct platform_device *pdev)
    {
    struct btqcomsmd *btq = platform_get_drvdata(pdev);
    hci_unregister_dev(btq.hdev);
    hci_free_dev(btq.hdev);
    rpmsg_destroy_ept(btq.cmd_channel);
    rpmsg_destroy_ept(btq.acl_channel);
    }
    static const struct of_device_id btqcomsmd_of_match[] = {
    { .compatible = "qcom,wcnss-bt", },
    { },
    };
    MODULE_DEVICE_TABLE(of, btqcomsmd_of_match);
    static struct platform_driver btqcomsmd_driver = {
    .probe = btqcomsmd_probe,
    .remove = btqcomsmd_remove,
    .driver  = {
    .name  = "btqcomsmd",
    .of_match_table = btqcomsmd_of_match,
    },
    };
    module_platform_driver(btqcomsmd_driver);
    MODULE_AUTHOR("Bjorn Andersson <bjorn.andersson@sonymobile.com>");
    MODULE_DESCRIPTION("Qualcomm SMD HCI driver");
    MODULE_LICENSE("GPL v2");
