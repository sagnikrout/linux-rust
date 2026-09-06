//! Automatically rewritten from C to Rust
//! Source: drivers/net/usb/cdc-phonet.c
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
// phonet.c -- USB CDC Phonet host driver
//
// Copyright (C) 2008-2009 Nokia Corporation. All rights reserved.
//
// Author: Rémi Denis-Courmont
//

pub const PN_MEDIA_USB: c_uint = 0x1B;
    let mut rxq_size: static unsigned = 17;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbpn_dev {
    pub dev: *mut net_device,
    pub data_intf: *mut *mut usb_interface intf,,
    pub usb: *mut usb_device,
    pub rx_pipe: unsigned int tx_pipe,,
    pub active_setting: u8,
    pub disconnected: u8,
    pub tx_queue: unsigned,
    pub tx_lock: spinlock_t,
    pub rx_lock: spinlock_t,
    pub rx_skb: *mut sk_buff,
    pub urbs: [*mut urb; ],
}

    static void tx_complete(struct urb *req);
    static void rx_complete(struct urb *req);
//
// Network device callbacks
//
#[no_mangle]
unsafe extern "C" fn usbpn_xmit(skb: *mut sk_buff, dev: *mut net_device) -> netdev_tx_t {
    static netdev_tx_t usbpn_xmit(struct sk_buff *skb, struct net_device *dev)
    {
    struct usbpn_dev *pnd = netdev_priv(dev);
    struct urb *req = core::ptr::null_mut();
    unsigned long flags;
    int err;
    if (skb.protocol != htons(ETH_P_PHONET))
    goto drop;
    req = usb_alloc_urb(0, GFP_ATOMIC);
    if (!req)
    goto drop;
    usb_fill_bulk_urb(req, pnd.usb, pnd.tx_pipe, skb.data, skb.len,
    tx_complete, skb);
    req.transfer_flags = URB_ZERO_PACKET;
    err = usb_submit_urb(req, GFP_ATOMIC);
    if (err) {
    usb_free_urb(req);
    goto drop;
    }
    spin_lock_irqsave(&pnd.tx_lock, flags);
    pnd.tx_queue++;
    if (pnd.tx_queue >= dev.tx_queue_len)
    netif_stop_queue(dev);
    spin_unlock_irqrestore(&pnd.tx_lock, flags);
    return NETDEV_TX_OK;
    drop:
    dev_kfree_skb(skb);
    dev.stats.tx_dropped++;
    return NETDEV_TX_OK;
    }
#[no_mangle]
unsafe extern "C" fn tx_complete(req: *mut urb) {
    static void tx_complete(struct urb *req)
    {
    struct sk_buff *skb = req.context;
    struct net_device *dev = skb.dev;
    struct usbpn_dev *pnd = netdev_priv(dev);
    let mut status: c_int = req.status;
    unsigned long flags;
    switch (status) {
    case 0:
    dev.stats.tx_bytes += skb.len;
    break;
    case -ENOENT:
    case -ECONNRESET:
    case -ESHUTDOWN:
    dev.stats.tx_aborted_errors++;
    fallthrough;
    default:
    dev.stats.tx_errors++;
    dev_dbg(&dev.dev, "TX error (%d)\n", status);
    }
    dev.stats.tx_packets++;
    spin_lock_irqsave(&pnd.tx_lock, flags);
    pnd.tx_queue--;
    netif_wake_queue(dev);
    spin_unlock_irqrestore(&pnd.tx_lock, flags);
    dev_kfree_skb_any(skb);
    usb_free_urb(req);
    }
#[no_mangle]
unsafe extern "C" fn rx_submit(pnd: *mut usbpn_dev, req: *mut urb, gfp_flags: gfp_t) -> c_int {
    static int rx_submit(struct usbpn_dev *pnd, struct urb *req, gfp_t gfp_flags)
    {
    struct net_device *dev = pnd.dev;
    struct page *page;
    int err;
    page = __dev_alloc_page(gfp_flags | __GFP_NOMEMALLOC);
    if (!page)
    return -ENOMEM;
    usb_fill_bulk_urb(req, pnd.usb, pnd.rx_pipe, page_address(page),
    PAGE_SIZE, rx_complete, dev);
    req.transfer_flags = 0;
    err = usb_submit_urb(req, gfp_flags);
    if (unlikely(err)) {
    dev_dbg(&dev.dev, "RX submit error (%d)\n", err);
    put_page(page);
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn rx_complete(req: *mut urb) {
    static void rx_complete(struct urb *req)
    {
    struct net_device *dev = req.context;
    struct usbpn_dev *pnd = netdev_priv(dev);
    struct page *page = virt_to_page(req.transfer_buffer);
    struct sk_buff *skb;
    unsigned long flags;
    let mut status: c_int = req.status;
    switch (status) {
    case 0:
    spin_lock_irqsave(&pnd.rx_lock, flags);
    skb = pnd.rx_skb;
    if (!skb) {
    skb = pnd.rx_skb = netdev_alloc_skb(dev, 12);
    if (likely(skb)) {
// Can't use pskb_pull() on page in IRQ
    skb_put_data(skb, page_address(page), 1);
    skb_add_rx_frag(skb, skb_shinfo(skb).nr_frags,
    page, 1, req.actual_length,
    PAGE_SIZE);
    page = core::ptr::null_mut();
    }
    } else if (skb_shinfo(skb).nr_frags < MAX_SKB_FRAGS) {
    skb_add_rx_frag(skb, skb_shinfo(skb).nr_frags,
    page, 0, req.actual_length,
    PAGE_SIZE);
    page = core::ptr::null_mut();
    } else {
    dev_kfree_skb_any(skb);
    pnd.rx_skb = core::ptr::null_mut();
    skb = core::ptr::null_mut();
    dev.stats.rx_length_errors++;
    }
    if (req.actual_length < PAGE_SIZE)
    pnd.rx_skb = core::ptr::null_mut(); /* Last fragment */
    else
    skb = core::ptr::null_mut();
    spin_unlock_irqrestore(&pnd.rx_lock, flags);
    if (skb) {
    skb.protocol = htons(ETH_P_PHONET);
    skb_reset_mac_header(skb);
    __skb_pull(skb, 1);
    skb.dev = dev;
    dev.stats.rx_packets++;
    dev.stats.rx_bytes += skb.len;
    netif_rx(skb);
    }
    goto resubmit;
    case -ENOENT:
    case -ECONNRESET:
    case -ESHUTDOWN:
    req = core::ptr::null_mut();
    break;
    case -EOVERFLOW:
    dev.stats.rx_over_errors++;
    dev_dbg(&dev.dev, "RX overflow\n");
    break;
    case -EILSEQ:
    dev.stats.rx_crc_errors++;
    break;
    }
    dev.stats.rx_errors++;
    resubmit:
    if (page)
    put_page(page);
    if (req)
    rx_submit(pnd, req, GFP_ATOMIC);
    }
    static int usbpn_close(struct net_device *dev);
#[no_mangle]
unsafe extern "C" fn usbpn_open(dev: *mut net_device) -> c_int {
    static int usbpn_open(struct net_device *dev)
    {
    struct usbpn_dev *pnd = netdev_priv(dev);
    int err;
    unsigned i;
    let mut num: unsigned = pnd.data_intf.cur_altsetting.desc.bInterfaceNumber;
    err = usb_set_interface(pnd.usb, num, pnd.active_setting);
    if (err)
    return err;
    for (i = 0; i < rxq_size; i++) {
    struct urb *req = usb_alloc_urb(0, GFP_KERNEL);
    if (!req || rx_submit(pnd, req, GFP_KERNEL)) {
    usb_free_urb(req);
    usbpn_close(dev);
    return -ENOMEM;
    }
    pnd.urbs[i] = req;
    }
    netif_wake_queue(dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn usbpn_close(dev: *mut net_device) -> c_int {
    static int usbpn_close(struct net_device *dev)
    {
    struct usbpn_dev *pnd = netdev_priv(dev);
    unsigned i;
    let mut num: unsigned = pnd.data_intf.cur_altsetting.desc.bInterfaceNumber;
    netif_stop_queue(dev);
    for (i = 0; i < rxq_size; i++) {
    struct urb *req = pnd.urbs[i];
    if (!req)
    continue;
    usb_kill_urb(req);
    usb_free_urb(req);
    pnd.urbs[i] = core::ptr::null_mut();
    }
    return usb_set_interface(pnd.usb, num, !pnd.active_setting);
    }
    static int usbpn_siocdevprivate(struct net_device *dev, struct ifreq *ifr,
    void __user *data, int cmd)
    {
    struct if_phonet_req *req = (struct if_phonet_req *)ifr;
    switch (cmd) {
    case SIOCPNGAUTOCONF:
    req.ifr_phonet_autoconf.device = PN_DEV_PC;
    return 0;
    }
    return -ENOIOCTLCMD;
    }
    static const struct net_device_ops usbpn_ops = {
    .ndo_open	= usbpn_open,
    .ndo_stop	= usbpn_close,
    .ndo_start_xmit = usbpn_xmit,
    .ndo_siocdevprivate = usbpn_siocdevprivate,
    };
#[no_mangle]
unsafe extern "C" fn usbpn_setup(dev: *mut net_device) {
    static void usbpn_setup(struct net_device *dev)
    {
    let mut addr: u8 = PN_MEDIA_USB;
    dev.features		= 0;
    dev.netdev_ops		= &usbpn_ops;
    dev.header_ops		= &phonet_header_ops;
    dev.type		= ARPHRD_PHONET;
    dev.flags		= IFF_POINTOPOINT | IFF_NOARP;
    dev.mtu		= PHONET_MAX_MTU;
    dev.min_mtu		= PHONET_MIN_MTU;
    dev.max_mtu		= PHONET_MAX_MTU;
    dev.hard_header_len	= 1;
    dev.addr_len		= 1;
    dev_addr_set(dev, &addr);
    dev.tx_queue_len	= 3;
    dev.needs_free_netdev	= true;
    }
//
// USB driver callbacks
//
    static const struct usb_device_id usbpn_ids[] = {
    {
    .match_flags = USB_DEVICE_ID_MATCH_VENDOR
    | USB_DEVICE_ID_MATCH_INT_CLASS
    | USB_DEVICE_ID_MATCH_INT_SUBCLASS,
    .idVendor = 0x0421, /* Nokia */
    .bInterfaceClass = USB_CLASS_COMM,
    .bInterfaceSubClass = 0xFE,
    },
    { },
    };
    MODULE_DEVICE_TABLE(usb, usbpn_ids);
    static struct usb_driver usbpn_driver;
#[no_mangle]
unsafe extern "C" fn usbpn_probe(intf: *mut usb_interface, id: *const usb_device_id) -> c_int {
    static int usbpn_probe(struct usb_interface *intf, const struct usb_device_id *id)
    {
    static const char ifname[] = "usbpn%d";
    const struct usb_cdc_union_desc *union_header = core::ptr::null_mut();
    const struct usb_host_interface *data_desc;
    struct usb_interface *data_intf;
    struct usb_device *usbdev = interface_to_usbdev(intf);
    struct net_device *dev;
    struct usbpn_dev *pnd;
    u8 *data;
    let mut phonet: c_int = 0;
    int len, err;
    struct usb_cdc_parsed_header hdr;
    data = intf.altsetting.extra;
    len = intf.altsetting.extralen;
    cdc_parse_cdc_header(&hdr, intf, data, len);
    union_header = hdr.usb_cdc_union_desc;
    phonet = hdr.phonet_magic_present;
    if (!union_header || !phonet)
    return -EINVAL;
    data_intf = usb_ifnum_to_if(usbdev, union_header.bSlaveInterface0);
    if (data_intf == core::ptr::null_mut())
    return -ENODEV;
// Data interface has one inactive and one active setting
    if (data_intf.num_altsetting != 2)
    return -EINVAL;
    if (data_intf.altsetting[0].desc.bNumEndpoints == 0 &&
    data_intf.altsetting[1].desc.bNumEndpoints == 2)
    data_desc = data_intf.altsetting + 1;
    else
    if (data_intf.altsetting[0].desc.bNumEndpoints == 2 &&
    data_intf.altsetting[1].desc.bNumEndpoints == 0)
    data_desc = data_intf.altsetting;
    else
    return -EINVAL;
    dev = alloc_netdev(struct_size(pnd, urbs, rxq_size), ifname,
    NET_NAME_UNKNOWN, usbpn_setup);
    if (!dev)
    return -ENOMEM;
    pnd = netdev_priv(dev);
    SET_NETDEV_DEV(dev, &intf.dev);
    pnd.dev = dev;
    pnd.usb = usbdev;
    pnd.intf = intf;
    pnd.data_intf = data_intf;
    spin_lock_init(&pnd.tx_lock);
    spin_lock_init(&pnd.rx_lock);
// Endpoints
    if (usb_pipein(data_desc.endpoint[0].desc.bEndpointAddress)) {
    pnd.rx_pipe = usb_rcvbulkpipe(usbdev,
    data_desc.endpoint[0].desc.bEndpointAddress);
    pnd.tx_pipe = usb_sndbulkpipe(usbdev,
    data_desc.endpoint[1].desc.bEndpointAddress);
    } else {
    pnd.rx_pipe = usb_rcvbulkpipe(usbdev,
    data_desc.endpoint[1].desc.bEndpointAddress);
    pnd.tx_pipe = usb_sndbulkpipe(usbdev,
    data_desc.endpoint[0].desc.bEndpointAddress);
    }
    pnd.active_setting = data_desc - data_intf.altsetting;
    err = usb_driver_claim_interface(&usbpn_driver, data_intf, pnd);
    if (err)
    goto out;
// Force inactive mode until the network device is brought UP
    usb_set_interface(usbdev, union_header.bSlaveInterface0,
    !pnd.active_setting);
    usb_set_intfdata(intf, pnd);
    err = register_netdev(dev);
    if (err) {
// Set disconnected flag so that disconnect() returns early.
    pnd.disconnected = 1;
    usb_driver_release_interface(&usbpn_driver, data_intf);
    goto out;
    }
    dev_dbg(&dev.dev, "USB CDC Phonet device found\n");
    return 0;
    out:
    usb_set_intfdata(intf, core::ptr::null_mut());
    free_netdev(dev);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn usbpn_disconnect(intf: *mut usb_interface) {
    static void usbpn_disconnect(struct usb_interface *intf)
    {
    struct usbpn_dev *pnd = usb_get_intfdata(intf);
    if (pnd.disconnected)
    return;
    pnd.disconnected = 1;
    usb_driver_release_interface(&usbpn_driver,
    (pnd.intf == intf) ? pnd.data_intf : pnd.intf);
    unregister_netdev(pnd.dev);
    }
    static struct usb_driver usbpn_driver = {
    .name =		"cdc_phonet",
    .probe =	usbpn_probe,
    .disconnect =	usbpn_disconnect,
    .id_table =	usbpn_ids,
    .disable_hub_initiated_lpm = 1,
    };
    module_usb_driver(usbpn_driver);
    MODULE_AUTHOR("Remi Denis-Courmont");
    MODULE_DESCRIPTION("USB CDC Phonet host interface");
    MODULE_LICENSE("GPL");
