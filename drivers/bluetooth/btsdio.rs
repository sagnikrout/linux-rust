//! Automatically rewritten from C to Rust
//! Source: drivers/bluetooth/btsdio.c
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
// Generic Bluetooth SDIO driver
//
// Copyright (C) 2007  Cambridge Silicon Radio Ltd.
// Copyright (C) 2007  Marcel Holtmann <marcel@holtmann.org>
//

    static const struct sdio_device_id btsdio_table[] = {
// Generic Bluetooth Type-A SDIO device
    { SDIO_DEVICE_CLASS(SDIO_CLASS_BT_A) },
// Generic Bluetooth Type-B SDIO device
    { SDIO_DEVICE_CLASS(SDIO_CLASS_BT_B) },
    { }	/* Terminating entry */
    };
    MODULE_DEVICE_TABLE(sdio, btsdio_table);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btsdio_data {
    pub hdev: *mut hci_dev,
    pub func: *mut sdio_func,
    pub work: work_struct,
    pub txq: sk_buff_head,
}

pub const REG_RDAT: c_uint = 0x00	/* Receiver Data */;
pub const REG_TDAT: c_uint = 0x00	/* Transmitter Data */;
pub const REG_PC_RRT: c_uint = 0x10	/* Read Packet Control */;
pub const REG_PC_WRT: c_uint = 0x11	/* Write Packet Control */;
pub const REG_RTC_STAT: c_uint = 0x12	/* Retry Control Status */;
pub const REG_RTC_SET: c_uint = 0x12	/* Retry Control Set */;
pub const REG_INTRD: c_uint = 0x13	/* Interrupt Indication */;
pub const REG_CL_INTRD: c_uint = 0x13	/* Interrupt Clear */;
pub const REG_EN_INTRD: c_uint = 0x14	/* Interrupt Enable */;
pub const REG_MD_STAT: c_uint = 0x20	/* Bluetooth Mode Status */;
pub const REG_MD_SET: c_uint = 0x20	/* Bluetooth Mode Set */;
#[no_mangle]
unsafe extern "C" fn btsdio_tx_packet(data: *mut btsdio_data, skb: *mut sk_buff) -> c_int {
    static int btsdio_tx_packet(struct btsdio_data *data, struct sk_buff *skb)
    {
    int err;
    BT_DBG("%s", data.hdev.name);
// Prepend Type-A header
    skb_push(skb, 4);
    skb.data[0] = (skb.len & 0x0000ff);
    skb.data[1] = (skb.len & 0x00ff00) >> 8;
    skb.data[2] = (skb.len & 0xff0000) >> 16;
    skb.data[3] = hci_skb_pkt_type(skb);
    err = sdio_writesb(data.func, REG_TDAT, skb.data, skb.len);
    if (err < 0) {
    skb_pull(skb, 4);
    sdio_writeb(data.func, 0x01, REG_PC_WRT, core::ptr::null_mut());
    return err;
    }
    data.hdev.stat.byte_tx += skb.len;
    kfree_skb(skb);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn btsdio_work(work: *mut work_struct) {
    static void btsdio_work(struct work_struct *work)
    {
    struct btsdio_data *data = container_of(work, struct btsdio_data, work);
    struct sk_buff *skb;
    int err;
    BT_DBG("%s", data.hdev.name);
    sdio_claim_host(data.func);
    while ((skb = skb_dequeue(&data.txq))) {
    err = btsdio_tx_packet(data, skb);
    if (err < 0) {
    data.hdev.stat.err_tx++;
    skb_queue_head(&data.txq, skb);
    break;
    }
    }
    sdio_release_host(data.func);
    }
#[no_mangle]
unsafe extern "C" fn btsdio_rx_packet(data: *mut btsdio_data) -> c_int {
    static int btsdio_rx_packet(struct btsdio_data *data)
    {
    u8 hdr[4] __attribute__ ((aligned(4)));
    struct sk_buff *skb;
    int err, len;
    BT_DBG("%s", data.hdev.name);
    err = sdio_readsb(data.func, hdr, REG_RDAT, 4);
    if (err < 0)
    return err;
    len = hdr[0] | (hdr[1] << 8) | (hdr[2] << 16);
    if (len < 4 || len > 65543)
    return -EILSEQ;
    skb = bt_skb_alloc(len - 4, GFP_KERNEL);
    if (!skb) {
// Out of memory. Prepare a read retry and just
// return with the expectation that the next time
// we're called we'll have more memory.
//
    return -ENOMEM;
    }
    skb_put(skb, len - 4);
    err = sdio_readsb(data.func, skb.data, REG_RDAT, len - 4);
    if (err < 0) {
    kfree_skb(skb);
    return err;
    }
    data.hdev.stat.byte_rx += len;
    switch (hdr[3]) {
    case HCI_EVENT_PKT:
    case HCI_ACLDATA_PKT:
    case HCI_SCODATA_PKT:
    case HCI_ISODATA_PKT:
    hci_skb_pkt_type(skb) = hdr[3];
    err = hci_recv_frame(data.hdev, skb);
    if (err < 0)
    return err;
    break;
    default:
    kfree_skb(skb);
    return -EINVAL;
    }
    sdio_writeb(data.func, 0x00, REG_PC_RRT, core::ptr::null_mut());
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn btsdio_interrupt(func: *mut sdio_func) {
    static void btsdio_interrupt(struct sdio_func *func)
    {
    struct btsdio_data *data = sdio_get_drvdata(func);
    int intrd;
    BT_DBG("%s", data.hdev.name);
    intrd = sdio_readb(func, REG_INTRD, core::ptr::null_mut());
    if (intrd & 0x01) {
    sdio_writeb(func, 0x01, REG_CL_INTRD, core::ptr::null_mut());
    if (btsdio_rx_packet(data) < 0) {
    data.hdev.stat.err_rx++;
    sdio_writeb(data.func, 0x01, REG_PC_RRT, core::ptr::null_mut());
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn btsdio_open(hdev: *mut hci_dev) -> c_int {
    static int btsdio_open(struct hci_dev *hdev)
    {
    struct btsdio_data *data = hci_get_drvdata(hdev);
    int err;
    BT_DBG("%s", hdev.name);
    sdio_claim_host(data.func);
    err = sdio_enable_func(data.func);
    if (err < 0)
    goto release;
    err = sdio_claim_irq(data.func, btsdio_interrupt);
    if (err < 0) {
    sdio_disable_func(data.func);
    goto release;
    }
    if (data.func.class == SDIO_CLASS_BT_B)
    sdio_writeb(data.func, 0x00, REG_MD_SET, core::ptr::null_mut());
    sdio_writeb(data.func, 0x01, REG_EN_INTRD, core::ptr::null_mut());
    release:
    sdio_release_host(data.func);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn btsdio_close(hdev: *mut hci_dev) -> c_int {
    static int btsdio_close(struct hci_dev *hdev)
    {
    struct btsdio_data *data = hci_get_drvdata(hdev);
    BT_DBG("%s", hdev.name);
    sdio_claim_host(data.func);
    sdio_writeb(data.func, 0x00, REG_EN_INTRD, core::ptr::null_mut());
    sdio_release_irq(data.func);
    sdio_disable_func(data.func);
    sdio_release_host(data.func);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn btsdio_flush(hdev: *mut hci_dev) -> c_int {
    static int btsdio_flush(struct hci_dev *hdev)
    {
    struct btsdio_data *data = hci_get_drvdata(hdev);
    BT_DBG("%s", hdev.name);
    skb_queue_purge(&data.txq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn btsdio_send_frame(hdev: *mut hci_dev, skb: *mut sk_buff) -> c_int {
    static int btsdio_send_frame(struct hci_dev *hdev, struct sk_buff *skb)
    {
    struct btsdio_data *data = hci_get_drvdata(hdev);
    BT_DBG("%s", hdev.name);
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
    default:
    return -EILSEQ;
    }
    skb_queue_tail(&data.txq, skb);
    schedule_work(&data.work);
    return 0;
    }
    static int btsdio_probe(struct sdio_func *func,
    const struct sdio_device_id *id)
    {
    struct btsdio_data *data;
    struct hci_dev *hdev;
    struct sdio_func_tuple *tuple = func.tuples;
    int err;
    BT_DBG("func %p id %p class 0x%04x", func, id, func.class);
    while (tuple) {
    BT_DBG("code 0x%x size %d", tuple.code, tuple.size);
    tuple = tuple.next;
    }
// Broadcom devices soldered onto the PCB (non-removable) use an
// UART connection for Bluetooth, ignore the BT SDIO interface.
//
    if (func.vendor == SDIO_VENDOR_ID_BROADCOM &&
    !mmc_card_is_removable(func.card.host)) {
    switch (func.device) {
    case SDIO_DEVICE_ID_BROADCOM_43341:
    case SDIO_DEVICE_ID_BROADCOM_43430:
    case SDIO_DEVICE_ID_BROADCOM_4345:
    case SDIO_DEVICE_ID_BROADCOM_43455:
    case SDIO_DEVICE_ID_BROADCOM_4356:
    case SDIO_DEVICE_ID_BROADCOM_CYPRESS_4373:
    return -ENODEV;
    }
    }
    data = devm_kzalloc(&func.dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.func = func;
    INIT_WORK(&data.work, btsdio_work);
    skb_queue_head_init(&data.txq);
    hdev = hci_alloc_dev();
    if (!hdev)
    return -ENOMEM;
    hdev.bus = HCI_SDIO;
    hci_set_drvdata(hdev, data);
    data.hdev = hdev;
    SET_HCIDEV_DEV(hdev, &func.dev);
    hdev.open     = btsdio_open;
    hdev.close    = btsdio_close;
    hdev.flush    = btsdio_flush;
    hdev.send     = btsdio_send_frame;
    if (func.vendor == 0x0104 && func.device == 0x00c5)
    hci_set_quirk(hdev, HCI_QUIRK_RESET_ON_CLOSE);
    err = hci_register_dev(hdev);
    if (err < 0) {
    hci_free_dev(hdev);
    return err;
    }
    sdio_set_drvdata(func, data);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn btsdio_remove(func: *mut sdio_func) {
    static void btsdio_remove(struct sdio_func *func)
    {
    struct btsdio_data *data = sdio_get_drvdata(func);
    struct hci_dev *hdev;
    BT_DBG("func %p", func);
    if (!data)
    return;
    cancel_work_sync(&data.work);
    hdev = data.hdev;
    sdio_set_drvdata(func, core::ptr::null_mut());
    hci_unregister_dev(hdev);
    hci_free_dev(hdev);
    }
    static struct sdio_driver btsdio_driver = {
    .name		= "btsdio",
    .probe		= btsdio_probe,
    .remove		= btsdio_remove,
    .id_table	= btsdio_table,
    };
    module_sdio_driver(btsdio_driver);
    MODULE_AUTHOR("Marcel Holtmann <marcel@holtmann.org>");
    MODULE_DESCRIPTION("Generic Bluetooth SDIO driver ver " VERSION);
    MODULE_VERSION(VERSION);
    MODULE_LICENSE("GPL");
