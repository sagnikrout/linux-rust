//! Automatically rewritten from C to Rust
//! Source: drivers/nfc/nfcmrvl/usb.c
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
// Marvell NFC-over-USB driver: USB interface related functions
//
// Copyright (C) 2014, Marvell International Ltd.
//

    static struct usb_device_id nfcmrvl_table[] = {
    { USB_DEVICE_AND_INTERFACE_INFO(0x1286, 0x2046,
    USB_CLASS_VENDOR_SPEC, 4, 1) },
    { }	/* Terminating entry */
    };
    MODULE_DEVICE_TABLE(usb, nfcmrvl_table);
pub const NFCMRVL_USB_BULK_RUNNING: c_int = 1;
pub const NFCMRVL_USB_SUSPENDING: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfcmrvl_usb_drv_data {
    pub udev: *mut usb_device,
    pub intf: *mut usb_interface,
    pub flags: c_ulong,
    pub waker: work_struct,
    pub tx_anchor: usb_anchor,
    pub bulk_anchor: usb_anchor,
    pub deferred: usb_anchor,
    pub tx_in_flight: c_int,
// protects tx_in_flight
    pub txlock: spinlock_t,
    pub bulk_tx_ep: *mut usb_endpoint_descriptor,
    pub bulk_rx_ep: *mut usb_endpoint_descriptor,
    pub suspend_count: c_int,
    pub priv: *mut nfcmrvl_private,
}

#[no_mangle]
unsafe extern "C" fn nfcmrvl_inc_tx(drv_data: *mut nfcmrvl_usb_drv_data) -> c_int {
    static int nfcmrvl_inc_tx(struct nfcmrvl_usb_drv_data *drv_data)
    {
    unsigned long flags;
    int rv;
    spin_lock_irqsave(&drv_data.txlock, flags);
    rv = test_bit(NFCMRVL_USB_SUSPENDING, &drv_data.flags);
    if (!rv)
    drv_data.tx_in_flight++;
    spin_unlock_irqrestore(&drv_data.txlock, flags);
    return rv;
    }
#[no_mangle]
unsafe extern "C" fn nfcmrvl_bulk_complete(urb: *mut urb) {
    static void nfcmrvl_bulk_complete(struct urb *urb)
    {
    struct nfcmrvl_usb_drv_data *drv_data = urb.context;
    int err;
    dev_dbg(&drv_data.udev.dev, "urb %p status %d count %d\n",
    urb, urb.status, urb.actual_length);
    if (!test_bit(NFCMRVL_NCI_RUNNING, &drv_data.flags))
    return;
    if (!urb.status) {
    struct sk_buff *skb;
    skb = nci_skb_alloc(drv_data.priv.ndev, urb.actual_length,
    GFP_ATOMIC);
    if (!skb) {
    nfc_err(&drv_data.udev.dev, "failed to alloc mem\n");
    } else {
    skb_put_data(skb, urb.transfer_buffer,
    urb.actual_length);
    if (nfcmrvl_nci_recv_frame(drv_data.priv, skb) < 0)
    nfc_err(&drv_data.udev.dev,
    "corrupted Rx packet\n");
    }
    }
    if (!test_bit(NFCMRVL_USB_BULK_RUNNING, &drv_data.flags))
    return;
    usb_anchor_urb(urb, &drv_data.bulk_anchor);
    usb_mark_last_busy(drv_data.udev);
    err = usb_submit_urb(urb, GFP_ATOMIC);
    if (err) {
// -EPERM: urb is being killed;
// -ENODEV: device got disconnected
//
    if (err != -EPERM && err != -ENODEV)
    nfc_err(&drv_data.udev.dev,
    "urb %p failed to resubmit (%d)\n", urb, -err);
    usb_unanchor_urb(urb);
    }
    }
    static int
    nfcmrvl_submit_bulk_urb(struct nfcmrvl_usb_drv_data *drv_data, gfp_t mem_flags)
    {
    struct urb *urb;
    unsigned char *buf;
    unsigned int pipe;
    int err, size = NFCMRVL_NCI_MAX_EVENT_SIZE;
    if (!drv_data.bulk_rx_ep)
    return -ENODEV;
    urb = usb_alloc_urb(0, mem_flags);
    if (!urb)
    return -ENOMEM;
    buf = kmalloc(size, mem_flags);
    if (!buf) {
    usb_free_urb(urb);
    return -ENOMEM;
    }
    pipe = usb_rcvbulkpipe(drv_data.udev,
    drv_data.bulk_rx_ep.bEndpointAddress);
    usb_fill_bulk_urb(urb, drv_data.udev, pipe, buf, size,
    nfcmrvl_bulk_complete, drv_data);
    urb.transfer_flags |= URB_FREE_BUFFER;
    usb_mark_last_busy(drv_data.udev);
    usb_anchor_urb(urb, &drv_data.bulk_anchor);
    err = usb_submit_urb(urb, mem_flags);
    if (err) {
    if (err != -EPERM && err != -ENODEV)
    nfc_err(&drv_data.udev.dev,
    "urb %p submission failed (%d)\n", urb, -err);
    usb_unanchor_urb(urb);
    }
    usb_free_urb(urb);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn nfcmrvl_tx_complete(urb: *mut urb) {
    static void nfcmrvl_tx_complete(struct urb *urb)
    {
    struct sk_buff *skb = urb.context;
    struct nci_dev *ndev = (struct nci_dev *)skb.dev;
    struct nfcmrvl_private *priv = nci_get_drvdata(ndev);
    struct nfcmrvl_usb_drv_data *drv_data = priv.drv_data;
    unsigned long flags;
    nfc_info(priv.dev, "urb %p status %d count %d\n",
    urb, urb.status, urb.actual_length);
    spin_lock_irqsave(&drv_data.txlock, flags);
    drv_data.tx_in_flight--;
    spin_unlock_irqrestore(&drv_data.txlock, flags);
    kfree(urb.setup_packet);
    kfree_skb(skb);
    }
#[no_mangle]
unsafe extern "C" fn nfcmrvl_usb_nci_open(priv: *mut nfcmrvl_private) -> c_int {
    static int nfcmrvl_usb_nci_open(struct nfcmrvl_private *priv)
    {
    struct nfcmrvl_usb_drv_data *drv_data = priv.drv_data;
    int err;
    err = usb_autopm_get_interface(drv_data.intf);
    if (err)
    return err;
    drv_data.intf.needs_remote_wakeup = 1;
    err = nfcmrvl_submit_bulk_urb(drv_data, GFP_KERNEL);
    if (err)
    goto failed;
    set_bit(NFCMRVL_USB_BULK_RUNNING, &drv_data.flags);
    nfcmrvl_submit_bulk_urb(drv_data, GFP_KERNEL);
    usb_autopm_put_interface(drv_data.intf);
    return 0;
    failed:
    usb_autopm_put_interface(drv_data.intf);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn nfcmrvl_usb_stop_traffic(drv_data: *mut nfcmrvl_usb_drv_data) {
    static void nfcmrvl_usb_stop_traffic(struct nfcmrvl_usb_drv_data *drv_data)
    {
    usb_kill_anchored_urbs(&drv_data.bulk_anchor);
    }
#[no_mangle]
unsafe extern "C" fn nfcmrvl_usb_nci_close(priv: *mut nfcmrvl_private) -> c_int {
    static int nfcmrvl_usb_nci_close(struct nfcmrvl_private *priv)
    {
    struct nfcmrvl_usb_drv_data *drv_data = priv.drv_data;
    int err;
    cancel_work_sync(&drv_data.waker);
    clear_bit(NFCMRVL_USB_BULK_RUNNING, &drv_data.flags);
    nfcmrvl_usb_stop_traffic(drv_data);
    usb_kill_anchored_urbs(&drv_data.tx_anchor);
    err = usb_autopm_get_interface(drv_data.intf);
    if (err)
    goto failed;
    drv_data.intf.needs_remote_wakeup = 0;
    usb_autopm_put_interface(drv_data.intf);
    failed:
    usb_scuttle_anchored_urbs(&drv_data.deferred);
    return 0;
    }
    static int nfcmrvl_usb_nci_send(struct nfcmrvl_private *priv,
    struct sk_buff *skb)
    {
    struct nfcmrvl_usb_drv_data *drv_data = priv.drv_data;
    struct urb *urb;
    unsigned int pipe;
    int err;
    if (!drv_data.bulk_tx_ep)
    return -ENODEV;
    urb = usb_alloc_urb(0, GFP_ATOMIC);
    if (!urb)
    return -ENOMEM;
    pipe = usb_sndbulkpipe(drv_data.udev,
    drv_data.bulk_tx_ep.bEndpointAddress);
    usb_fill_bulk_urb(urb, drv_data.udev, pipe, skb.data, skb.len,
    nfcmrvl_tx_complete, skb);
    err = nfcmrvl_inc_tx(drv_data);
    if (err) {
    usb_anchor_urb(urb, &drv_data.deferred);
    schedule_work(&drv_data.waker);
    err = 0;
    goto done;
    }
    usb_anchor_urb(urb, &drv_data.tx_anchor);
    err = usb_submit_urb(urb, GFP_ATOMIC);
    if (err) {
    if (err != -EPERM && err != -ENODEV)
    nfc_err(&drv_data.udev.dev,
    "urb %p submission failed (%d)\n", urb, -err);
    kfree(urb.setup_packet);
    usb_unanchor_urb(urb);
    } else {
    usb_mark_last_busy(drv_data.udev);
    }
    done:
    usb_free_urb(urb);
    return err;
    }
    static const struct nfcmrvl_if_ops usb_ops = {
    .nci_open = nfcmrvl_usb_nci_open,
    .nci_close = nfcmrvl_usb_nci_close,
    .nci_send = nfcmrvl_usb_nci_send,
    };
#[no_mangle]
unsafe extern "C" fn nfcmrvl_waker(work: *mut work_struct) {
    static void nfcmrvl_waker(struct work_struct *work)
    {
    struct nfcmrvl_usb_drv_data *drv_data =
    container_of(work, struct nfcmrvl_usb_drv_data, waker);
    int err;
    err = usb_autopm_get_interface(drv_data.intf);
    if (err)
    return;
    usb_autopm_put_interface(drv_data.intf);
    }
    static int nfcmrvl_probe(struct usb_interface *intf,
    const struct usb_device_id *id)
    {
    struct nfcmrvl_usb_drv_data *drv_data;
    struct nfcmrvl_private *priv;
    struct usb_device *udev = interface_to_usbdev(intf);
    struct nfcmrvl_platform_data config;
    int ret;
// No configuration for USB
    memset(&config, 0, sizeof(config));
    config.reset_gpio = core::ptr::null_mut();
    nfc_info(&udev.dev, "intf %p id %p\n", intf, id);
    drv_data = devm_kzalloc(&intf.dev, sizeof(*drv_data), GFP_KERNEL);
    if (!drv_data)
    return -ENOMEM;
    ret = usb_find_common_endpoints(intf.cur_altsetting, &drv_data.bulk_rx_ep,
    &drv_data.bulk_tx_ep, core::ptr::null_mut(), core::ptr::null_mut());
    if (ret)
    return -ENODEV;
    drv_data.udev = udev;
    drv_data.intf = intf;
    INIT_WORK(&drv_data.waker, nfcmrvl_waker);
    spin_lock_init(&drv_data.txlock);
    init_usb_anchor(&drv_data.tx_anchor);
    init_usb_anchor(&drv_data.bulk_anchor);
    init_usb_anchor(&drv_data.deferred);
    priv = nfcmrvl_nci_register_dev(NFCMRVL_PHY_USB, drv_data, &usb_ops,
    &intf.dev, &config);
    if (IS_ERR(priv))
    return PTR_ERR(priv);
    drv_data.priv = priv;
    drv_data.priv.support_fw_dnld = false;
    usb_set_intfdata(intf, drv_data);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nfcmrvl_disconnect(intf: *mut usb_interface) {
    static void nfcmrvl_disconnect(struct usb_interface *intf)
    {
    struct nfcmrvl_usb_drv_data *drv_data = usb_get_intfdata(intf);
    if (!drv_data)
    return;
    nfc_info(&drv_data.udev.dev, "intf %p\n", intf);
    nfcmrvl_nci_unregister_dev(drv_data.priv);
    usb_set_intfdata(drv_data.intf, core::ptr::null_mut());
    }

#[no_mangle]
unsafe extern "C" fn nfcmrvl_suspend(intf: *mut usb_interface, message: pm_message_t) -> c_int {
    static int nfcmrvl_suspend(struct usb_interface *intf, pm_message_t message)
    {
    struct nfcmrvl_usb_drv_data *drv_data = usb_get_intfdata(intf);
    nfc_info(&drv_data.udev.dev, "intf %p\n", intf);
    if (drv_data.suspend_count++)
    return 0;
    spin_lock_irq(&drv_data.txlock);
    if (!(PMSG_IS_AUTO(message) && drv_data.tx_in_flight)) {
    set_bit(NFCMRVL_USB_SUSPENDING, &drv_data.flags);
    spin_unlock_irq(&drv_data.txlock);
    } else {
    spin_unlock_irq(&drv_data.txlock);
    drv_data.suspend_count--;
    return -EBUSY;
    }
    nfcmrvl_usb_stop_traffic(drv_data);
    usb_kill_anchored_urbs(&drv_data.tx_anchor);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nfcmrvl_play_deferred(drv_data: *mut nfcmrvl_usb_drv_data) {
    static void nfcmrvl_play_deferred(struct nfcmrvl_usb_drv_data *drv_data)
    {
    struct urb *urb;
    int err;
    while ((urb = usb_get_from_anchor(&drv_data.deferred))) {
    usb_anchor_urb(urb, &drv_data.tx_anchor);
    err = usb_submit_urb(urb, GFP_ATOMIC);
    if (err) {
    kfree(urb.setup_packet);
    usb_unanchor_urb(urb);
    usb_free_urb(urb);
    break;
    }
    drv_data.tx_in_flight++;
    usb_free_urb(urb);
    }
// Cleanup the rest deferred urbs.
    while ((urb = usb_get_from_anchor(&drv_data.deferred))) {
    kfree(urb.setup_packet);
    usb_free_urb(urb);
    }
    }
#[no_mangle]
unsafe extern "C" fn nfcmrvl_resume(intf: *mut usb_interface) -> c_int {
    static int nfcmrvl_resume(struct usb_interface *intf)
    {
    struct nfcmrvl_usb_drv_data *drv_data = usb_get_intfdata(intf);
    let mut err: c_int = 0;
    nfc_info(&drv_data.udev.dev, "intf %p\n", intf);
    if (--drv_data.suspend_count)
    return 0;
    if (!test_bit(NFCMRVL_NCI_RUNNING, &drv_data.flags))
    goto done;
    if (test_bit(NFCMRVL_USB_BULK_RUNNING, &drv_data.flags)) {
    err = nfcmrvl_submit_bulk_urb(drv_data, GFP_NOIO);
    if (err) {
    clear_bit(NFCMRVL_USB_BULK_RUNNING, &drv_data.flags);
    goto failed;
    }
    nfcmrvl_submit_bulk_urb(drv_data, GFP_NOIO);
    }
    spin_lock_irq(&drv_data.txlock);
    nfcmrvl_play_deferred(drv_data);
    clear_bit(NFCMRVL_USB_SUSPENDING, &drv_data.flags);
    spin_unlock_irq(&drv_data.txlock);
    return 0;
    failed:
    usb_scuttle_anchored_urbs(&drv_data.deferred);
    done:
    spin_lock_irq(&drv_data.txlock);
    clear_bit(NFCMRVL_USB_SUSPENDING, &drv_data.flags);
    spin_unlock_irq(&drv_data.txlock);
    return err;
    }

    static struct usb_driver nfcmrvl_usb_driver = {
    .name		= "nfcmrvl",
    .probe		= nfcmrvl_probe,
    .disconnect	= nfcmrvl_disconnect,

    .suspend	= nfcmrvl_suspend,
    .resume		= nfcmrvl_resume,
    .reset_resume	= nfcmrvl_resume,

    .id_table	= nfcmrvl_table,
    .supports_autosuspend = 1,
    .disable_hub_initiated_lpm = 1,
    .soft_unbind = 1,
    };
    module_usb_driver(nfcmrvl_usb_driver);
    MODULE_AUTHOR("Marvell International Ltd.");
    MODULE_DESCRIPTION("Marvell NFC-over-USB driver");
    MODULE_LICENSE("GPL v2");
