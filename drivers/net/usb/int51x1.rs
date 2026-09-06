//! Automatically rewritten from C to Rust
//! Source: drivers/net/usb/int51x1.c
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
// Copyright (c) 2009 Peter Holik
//
// Intellon usb PLC (Powerline Communications) usb net driver
//
// https://web.archive.org/web/20101025091240id_/http://www.tandel.be/downloads/INT51X1_Datasheet.pdf
//
// Based on the work of Jan 'RedBully' Seiffert
//

pub const INT51X1_VENDOR_ID: c_uint = 0x09e1;
pub const INT51X1_PRODUCT_ID: c_uint = 0x5121;

#[no_mangle]
unsafe extern "C" fn int51x1_rx_fixup(dev: *mut usbnet, skb: *mut sk_buff) -> c_int {
    static int int51x1_rx_fixup(struct usbnet *dev, struct sk_buff *skb)
    {
    int len;
    if (!(pskb_may_pull(skb, INT51X1_HEADER_SIZE))) {
    netdev_err(dev.net, "unexpected tiny rx frame\n");
    return 0;
    }
    len = le16_to_cpu(*(__le16 *)&skb.data[skb.len - 2]);
    skb_trim(skb, len);
    return 1;
    }
    static struct sk_buff *int51x1_tx_fixup(struct usbnet *dev,
    struct sk_buff *skb, gfp_t flags)
    {
    let mut pack_len: c_int = skb.len;
    let mut pack_with_header_len: c_int = pack_len + INT51X1_HEADER_SIZE;
    let mut headroom: c_int = skb_headroom(skb);
    let mut tailroom: c_int = skb_tailroom(skb);
    let mut need_tail: c_int = 0;
    __le16 *len;
// if packet and our header is smaller than 64 pad to 64 (+ ZLP)
    if ((pack_with_header_len) < dev.maxpacket)
    need_tail = dev.maxpacket - pack_with_header_len + 1;
//
// usbnet would send a ZLP if packetlength mod urbsize == 0 for us,
// but we need to know ourself, because this would add to the length
// we send down to the device...
//
#[no_mangle]
pub unsafe extern "C" fn if(dev->maxpacket): !(pack_with_header_len %) -> else {
    else if (!(pack_with_header_len % dev.maxpacket))
    need_tail = 1;
    if (!skb_cloned(skb) &&
    (headroom + tailroom >= need_tail + INT51X1_HEADER_SIZE)) {
    if (headroom < INT51X1_HEADER_SIZE || tailroom < need_tail) {
    skb.data = memmove(skb.head + INT51X1_HEADER_SIZE,
    skb.data, skb.len);
    skb_set_tail_pointer(skb, skb.len);
    }
    } else {
    struct sk_buff *skb2;
    skb2 = skb_copy_expand(skb,
    INT51X1_HEADER_SIZE,
    need_tail,
    flags);
    dev_kfree_skb_any(skb);
    if (!skb2)
    return core::ptr::null_mut();
    skb = skb2;
    }
    pack_len += need_tail;
    pack_len &= 0x07ff;
    len = __skb_push(skb, INT51X1_HEADER_SIZE);
// len = cpu_to_le16(pack_len);
    if(need_tail)
    __skb_put_zero(skb, need_tail);
    return skb;
    }
    static const struct net_device_ops int51x1_netdev_ops = {
    .ndo_open		= usbnet_open,
    .ndo_stop		= usbnet_stop,
    .ndo_start_xmit		= usbnet_start_xmit,
    .ndo_tx_timeout		= usbnet_tx_timeout,
    .ndo_change_mtu		= usbnet_change_mtu,
    .ndo_get_stats64	= dev_get_tstats64,
    .ndo_set_mac_address	= eth_mac_addr,
    .ndo_validate_addr	= eth_validate_addr,
    .ndo_set_rx_mode	= usbnet_set_rx_mode,
    };
#[no_mangle]
unsafe extern "C" fn int51x1_bind(dev: *mut usbnet, intf: *mut usb_interface) -> c_int {
    static int int51x1_bind(struct usbnet *dev, struct usb_interface *intf)
    {
    let mut status: c_int = usbnet_get_ethernet_addr(dev, 3);
    if (status)
    return status;
    dev.net.hard_header_len += INT51X1_HEADER_SIZE;
    dev.hard_mtu = dev.net.mtu + dev.net.hard_header_len;
    dev.net.netdev_ops = &int51x1_netdev_ops;
    return usbnet_get_endpoints(dev, intf);
    }
    static const struct driver_info int51x1_info = {
    .description = "Intellon usb powerline adapter",
    .bind        = int51x1_bind,
    .rx_fixup    = int51x1_rx_fixup,
    .tx_fixup    = int51x1_tx_fixup,
    .set_rx_mode = usbnet_cdc_update_filter,
    .in          = 1,
    .out         = 2,
    .flags       = FLAG_ETHER,
    };
    static const struct usb_device_id products[] = {
    {
    USB_DEVICE(INT51X1_VENDOR_ID, INT51X1_PRODUCT_ID),
    .driver_info = (unsigned long) &int51x1_info,
    },
    {},
    };
    MODULE_DEVICE_TABLE(usb, products);
    static struct usb_driver int51x1_driver = {
    .name       = "int51x1",
    .id_table   = products,
    .probe      = usbnet_probe,
    .disconnect = usbnet_disconnect,
    .suspend    = usbnet_suspend,
    .resume     = usbnet_resume,
    .disable_hub_initiated_lpm = 1,
    };
    module_usb_driver(int51x1_driver);
    MODULE_AUTHOR("Peter Holik");
    MODULE_DESCRIPTION("Intellon usb powerline adapter");
    MODULE_LICENSE("GPL");
