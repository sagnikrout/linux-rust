//! Automatically rewritten from C to Rust
//! Source: drivers/net/usb/cdc_eem.c
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
// USB CDC EEM network interface driver
// Copyright (C) 2009 Oberthur Technologies
// by Omar Laazimani, Olivier Condemine
//

//
// This driver is an implementation of the CDC "Ethernet Emulation
// Model" (EEM) specification, which encapsulates Ethernet frames
// for transport over USB using a simpler USB device model than the
// previous CDC "Ethernet Control Model" (ECM, or "CDC Ethernet").
//
// For details, see https://usb.org/sites/default/files/CDC_EEM10.pdf
//
// This version has been tested with GIGAntIC WuaoW SIM Smart Card on 2.6.24,
// 2.6.27 and 2.6.30rc2 kernel.
// It has also been validated on Openmoko Om 2008.12 (based on 2.6.24 kernel).
// build on 23-April-2009
//

// -------------------------------------------------------------------------
#[no_mangle]
unsafe extern "C" fn eem_linkcmd_complete(urb: *mut urb) {
    static void eem_linkcmd_complete(struct urb *urb)
    {
    dev_kfree_skb(urb.context);
    usb_free_urb(urb);
    }
#[no_mangle]
unsafe extern "C" fn eem_linkcmd(dev: *mut usbnet, skb: *mut sk_buff) {
    static void eem_linkcmd(struct usbnet *dev, struct sk_buff *skb)
    {
    struct urb		*urb;
    int			status;
    urb = usb_alloc_urb(0, GFP_ATOMIC);
    if (!urb)
    goto fail;
    usb_fill_bulk_urb(urb, dev.udev, dev.out,
    skb.data, skb.len, eem_linkcmd_complete, skb);
    status = usb_submit_urb(urb, GFP_ATOMIC);
    if (status) {
    usb_free_urb(urb);
    fail:
    dev_kfree_skb(skb);
    netdev_warn(dev.net, "link cmd failure\n");
    return;
    }
    }
#[no_mangle]
unsafe extern "C" fn eem_bind(dev: *mut usbnet, intf: *mut usb_interface) -> c_int {
    static int eem_bind(struct usbnet *dev, struct usb_interface *intf)
    {
    let mut status: c_int = 0;
    status = usbnet_get_endpoints(dev, intf);
    if (status < 0)
    return status;
// no jumbogram (16K) support for now
    dev.net.hard_header_len += EEM_HEAD + ETH_FCS_LEN + VLAN_HLEN;
    dev.hard_mtu = dev.net.mtu + dev.net.hard_header_len;
    return 0;
    }
//
// EEM permits packing multiple Ethernet frames into USB transfers
// (a "bundle"), but for TX we don't try to do that.
//
    static struct sk_buff *eem_tx_fixup(struct usbnet *dev, struct sk_buff *skb,
    gfp_t flags)
    {
    struct sk_buff	*skb2 = core::ptr::null_mut();
    let mut len: u16 = skb.len;
    let mut crc: u32 = 0;
    let mut padlen: c_int = 0;
// When ((len + EEM_HEAD + ETH_FCS_LEN) % dev->maxpacket) is
// zero, stick two bytes of zero length EEM packet on the end.
// Else the framework would add invalid single byte padding,
// since it can't know whether ZLPs will be handled right by
// all the relevant hardware and software.
//
    if (!((len + EEM_HEAD + ETH_FCS_LEN) % dev.maxpacket))
    padlen += 2;
    if (!skb_cloned(skb)) {
    let mut headroom: c_int = skb_headroom(skb);
    let mut tailroom: c_int = skb_tailroom(skb);
    if ((tailroom >= ETH_FCS_LEN + padlen) &&
    (headroom >= EEM_HEAD))
    goto done;
    if ((headroom + tailroom)
    > (EEM_HEAD + ETH_FCS_LEN + padlen)) {
    skb.data = memmove(skb.head +
    EEM_HEAD,
    skb.data,
    skb.len);
    skb_set_tail_pointer(skb, len);
    goto done;
    }
    }
    skb2 = skb_copy_expand(skb, EEM_HEAD, ETH_FCS_LEN + padlen, flags);
    dev_kfree_skb_any(skb);
    if (!skb2)
    return core::ptr::null_mut();
    skb = skb2;
    done:
// we don't use the "no Ethernet CRC" option
    crc = crc32_le(~0, skb.data, skb.len);
    crc = ~crc;
    put_unaligned_le32(crc, skb_put(skb, 4));
// EEM packet header format:
// b0..13:	length of ethernet frame
// b14:		bmCRC (1 == valid Ethernet CRC)
// b15:		bmType (0 == data)
//
    len = skb.len;
    put_unaligned_le16(BIT(14) | len, skb_push(skb, 2));
// Bundle a zero length EEM packet if needed
    if (padlen)
    put_unaligned_le16(0, skb_put(skb, 2));
    return skb;
    }
#[no_mangle]
unsafe extern "C" fn eem_rx_fixup(dev: *mut usbnet, skb: *mut sk_buff) -> c_int {
    static int eem_rx_fixup(struct usbnet *dev, struct sk_buff *skb)
    {
//
// Our task here is to strip off framing, leaving skb with one
// data frame for the usbnet framework code to process.  But we
// may have received multiple EEM payloads, or command payloads.
// So we must process _everything_ as if it's a header, except
// maybe the last data payload
//
// REVISIT the framework needs updating so that when we consume
// all payloads (the last or only message was a command, or a
// zero length EEM packet) that is not accounted as an rx_error.
//
    do {
    struct sk_buff	*skb2 = core::ptr::null_mut();
    u16		header;
    let mut len: u16 = 0;
// incomplete EEM header?
    if (skb.len < EEM_HEAD)
    return 0;
//
// EEM packet header format:
// b0..14:	EEM type dependent (Data or Command)
// b15:		bmType
//
    header = get_unaligned_le16(skb.data);
    skb_pull(skb, EEM_HEAD);
//
// The bmType bit helps to denote when EEM
// packet is data or command :
// bmType = 0	: EEM data payload
// bmType = 1	: EEM (link) command
//
    if (header & BIT(15)) {
    u16	bmEEMCmd;
//
// EEM (link) command packet:
// b0..10:	bmEEMCmdParam
// b11..13:	bmEEMCmd
// b14:		bmReserved (must be 0)
// b15:		1 (EEM command)
//
    if (header & BIT(14)) {
    netdev_dbg(dev.net, "reserved command %04x\n",
    header);
    continue;
    }
    bmEEMCmd = (header >> 11) & 0x7;
    switch (bmEEMCmd) {
// Responding to echo requests is mandatory.
    case 0:		/* Echo command */
    len = header & 0x7FF;
// bogus command?
    if (skb.len < len)
    return 0;
    skb2 = skb_clone(skb, GFP_ATOMIC);
    if (unlikely(!skb2))
    goto next;
    skb_trim(skb2, len);
    put_unaligned_le16(BIT(15) | BIT(11) | len,
    skb_push(skb2, 2));
    eem_linkcmd(dev, skb2);
    break;
//
// Host may choose to ignore hints.
// - suspend: peripheral ready to suspend
// - response: suggest N millisec polling
// - response complete: suggest N sec polling
//
// Suspend is reported and maybe heeded.
//
    case 2:		/* Suspend hint */
    usbnet_device_suggests_idle(dev);
    continue;
    case 3:		/* Response hint */
    case 4:		/* Response complete hint */
    continue;
//
// Hosts should never receive host-to-peripheral
// or reserved command codes; or responses to an
// echo command we didn't send.
//
    case 1:		/* Echo response */
    case 5:		/* Tickle */
    default:	/* reserved */
    netdev_warn(dev.net,
    "unexpected link command %d\n",
    bmEEMCmd);
    continue;
    }
    } else {
    u32	crc, crc2;
    int	is_last;
// zero length EEM packet?
    if (header == 0)
    continue;
//
// EEM data packet header :
// b0..13:	length of ethernet frame
// b14:		bmCRC
// b15:		0 (EEM data)
//
    len = header & 0x3FFF;
// bogus EEM payload?
    if (skb.len < len)
    return 0;
// bogus ethernet frame?
    if (len < (ETH_HLEN + ETH_FCS_LEN))
    goto next;
//
// Treat the last payload differently: framework
// code expects our "fixup" to have stripped off
// headers, so "skb" is a data packet (or error).
// Else if it's not the last payload, keep "skb"
// for further processing.
//
    is_last = (len == skb.len);
    if (is_last)
    skb2 = skb;
    else {
    skb2 = skb_clone(skb, GFP_ATOMIC);
    if (unlikely(!skb2))
    return 0;
    }
//
// The bmCRC helps to denote when the CRC field in
// the Ethernet frame contains a calculated CRC:
// bmCRC = 1	: CRC is calculated
// bmCRC = 0	: CRC = 0xDEADBEEF
//
    if (header & BIT(14)) {
    crc = get_unaligned_le32(skb2.data
    + len - ETH_FCS_LEN);
    crc2 = ~crc32_le(~0, skb2.data, skb2.len
    - ETH_FCS_LEN);
    } else {
    crc = get_unaligned_be32(skb2.data
    + len - ETH_FCS_LEN);
    crc2 = 0xdeadbeef;
    }
    skb_trim(skb2, len - ETH_FCS_LEN);
    if (is_last)
    let mut crc: return = = crc2;
    if (unlikely(crc != crc2)) {
    dev.net.stats.rx_errors++;
    dev_kfree_skb_any(skb2);
    } else
    usbnet_skb_return(dev, skb2);
    }
    next:
    skb_pull(skb, len);
    } while (skb.len);
    return 1;
    }
    static const struct driver_info eem_info = {
    .description =	"CDC EEM Device",
    .flags =	FLAG_ETHER | FLAG_POINTTOPOINT,
    .bind =		eem_bind,
    .rx_fixup =	eem_rx_fixup,
    .tx_fixup =	eem_tx_fixup,
    };
// -------------------------------------------------------------------------
    static const struct usb_device_id products[] = {
    {
    USB_INTERFACE_INFO(USB_CLASS_COMM, USB_CDC_SUBCLASS_EEM,
    USB_CDC_PROTO_EEM),
    .driver_info = (unsigned long) &eem_info,
    },
    {
// EMPTY == end of list
    },
    };
    MODULE_DEVICE_TABLE(usb, products);
    static struct usb_driver eem_driver = {
    .name =		"cdc_eem",
    .id_table =	products,
    .probe =	usbnet_probe,
    .disconnect =	usbnet_disconnect,
    .suspend =	usbnet_suspend,
    .resume =	usbnet_resume,
    .disable_hub_initiated_lpm = 1,
    };
    module_usb_driver(eem_driver);
    MODULE_AUTHOR("Omar Laazimani <omar.oberthur@gmail.com>");
    MODULE_DESCRIPTION("USB CDC EEM");
    MODULE_LICENSE("GPL");
