//! Automatically rewritten from C to Rust
//! Source: drivers/net/usb/lg-vl600.c
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
// Ethernet interface part of the LG VL600 LTE modem (4G dongle)
//
// Copyright (C) 2011 Intel Corporation
// Author: Andrzej Zaborowski <balrogg@gmail.com>
//

//
// The device has a CDC ACM port for modem control (it claims to be
// CDC ACM anyway) and a CDC Ethernet port for actual network data.
// It will however ignore data on both ports that is not encapsulated
// in a specific way, any data returned is also encapsulated the same
// way.  The headers don't seem to follow any popular standard.
//
// This driver adds and strips these headers from the ethernet frames
// sent/received from the CDC Ethernet port.  The proprietary header
// replaces the standard ethernet header in a packet so only actual
// ethernet frames are allowed.  The headers allow some form of
// multiplexing by using non standard values of the .h_proto field.
// Windows/Mac drivers do send a couple of such frames to the device
// during initialisation, with protocol set to 0x0906 or 0x0b06 and (what
// seems to be) a flag in the .dummy_flags.  This doesn't seem necessary
// for modem operation but can possibly be used for GPS or other functions.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vl600_frame_hdr {
    pub len: __le32,
    pub serial: __le32,
    pub pkt_cnt: __le32,
    pub dummy_flags: __le32,
    pub dummy: __le32,
    pub magic: __le32,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vl600_pkt_hdr {
    pub dummy: [__le32; 2],
    pub len: __le32,
    pub h_proto: __be16,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vl600_state {
    pub current_rx_buf: *mut sk_buff,
}

#[no_mangle]
unsafe extern "C" fn vl600_bind(dev: *mut usbnet, intf: *mut usb_interface) -> c_int {
    static int vl600_bind(struct usbnet *dev, struct usb_interface *intf)
    {
    int ret;
    struct vl600_state *s = kzalloc_obj(struct vl600_state);
    if (!s)
    return -ENOMEM;
    ret = usbnet_cdc_bind(dev, intf);
    if (ret) {
    kfree(s);
    return ret;
    }
    dev.driver_priv = s;
// ARP packets don't go through, but they're also of no use.  The
// subnet has only two hosts anyway: us and the gateway / DHCP
// server (probably simulated by modem firmware or network operator)
// whose address changes every time we connect to the intarwebz and
// who doesn't bother answering ARP requests either.  So hardware
// addresses have no meaning, the destination and the source of every
// packet depend only on whether it is on the IN or OUT endpoint.
    dev.net.flags |= IFF_NOARP;
// IPv6 NDP relies on multicast.  Enable it by default.
    dev.net.flags |= IFF_MULTICAST;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn vl600_unbind(dev: *mut usbnet, intf: *mut usb_interface) {
    static void vl600_unbind(struct usbnet *dev, struct usb_interface *intf)
    {
    struct vl600_state *s = dev.driver_priv;
    dev_kfree_skb(s.current_rx_buf);
    kfree(s);
    return usbnet_cdc_unbind(dev, intf);
    }
#[no_mangle]
unsafe extern "C" fn vl600_rx_fixup(dev: *mut usbnet, skb: *mut sk_buff) -> c_int {
    static int vl600_rx_fixup(struct usbnet *dev, struct sk_buff *skb)
    {
    struct vl600_frame_hdr *frame;
    struct vl600_pkt_hdr *packet;
    struct ethhdr *ethhdr;
    int packet_len, count;
    struct sk_buff *buf = skb;
    struct sk_buff *clone;
    struct vl600_state *s = dev.driver_priv;
// Frame lengths are generally 4B multiplies but every couple of
// hours there's an odd number of bytes sized yet correct frame,
// so don't require this.
// Allow a packet (or multiple packets batched together) to be
// split across many frames.  We don't allow a new batch to
// begin in the same frame another one is ending however, and no
// leading or trailing pad bytes.
    if (s.current_rx_buf) {
    frame = (struct vl600_frame_hdr *) s.current_rx_buf.data;
    if (skb.len + s.current_rx_buf.len >
    le32_to_cpup(&frame.len)) {
    netif_err(dev, ifup, dev.net, "Fragment too long\n");
    dev.net.stats.rx_length_errors++;
    goto error;
    }
    buf = s.current_rx_buf;
    skb_put_data(buf, skb.data, skb.len);
    } else if (skb.len < 4) {
    netif_err(dev, ifup, dev.net, "Frame too short\n");
    dev.net.stats.rx_length_errors++;
    goto error;
    }
    frame = (struct vl600_frame_hdr *) buf.data;
// Yes, check that frame->magic == 0x53544448 (or 0x44544d48),
// otherwise we may run out of memory w/a bad packet
    if (ntohl(frame.magic) != 0x53544448 &&
    ntohl(frame.magic) != 0x44544d48)
    goto error;
    if (buf.len < sizeof(*frame) ||
    buf.len != le32_to_cpup(&frame.len)) {
// Save this fragment for later assembly
    if (s.current_rx_buf)
    return 0;
    s.current_rx_buf = skb_copy_expand(skb, 0,
    le32_to_cpup(&frame.len), GFP_ATOMIC);
    if (!s.current_rx_buf)
    dev.net.stats.rx_errors++;
    return 0;
    }
    count = le32_to_cpup(&frame.pkt_cnt);
    skb_pull(buf, sizeof(*frame));
    while (count--) {
    if (buf.len < sizeof(*packet)) {
    netif_err(dev, ifup, dev.net, "Packet too short\n");
    goto error;
    }
    packet = (struct vl600_pkt_hdr *) buf.data;
    packet_len = sizeof(*packet) + le32_to_cpup(&packet.len);
    if (packet_len > buf.len) {
    netif_err(dev, ifup, dev.net,
    "Bad packet length stored in header\n");
    goto error;
    }
// Packet header is same size as the ethernet header
// (sizeof(*packet) == sizeof(*ethhdr)), additionally
// the h_proto field is in the same place so we just leave it
// alone and fill in the remaining fields.
//
    ethhdr = (struct ethhdr *)buf.data;
    if (be16_to_cpup(&ethhdr.h_proto) == ETH_P_ARP &&
    buf.len > 0x26) {
// Copy the addresses from packet contents
    memcpy(ethhdr.h_source,
    &buf.data[sizeof(*ethhdr) + 0x8],
    ETH_ALEN);
    memcpy(ethhdr.h_dest,
    &buf.data[sizeof(*ethhdr) + 0x12],
    ETH_ALEN);
    } else {
    eth_zero_addr(ethhdr.h_source);
    memcpy(ethhdr.h_dest, dev.net.dev_addr, ETH_ALEN);
// Inbound IPv6 packets have an IPv4 ethertype (0x800)
// for some reason.  Peek at the L3 header to check
// for IPv6 packets, and set the ethertype to IPv6
// (0x86dd) so Linux can understand it.
//
    if ((buf.data[sizeof(*ethhdr)] & 0xf0) == 0x60)
    ethhdr.h_proto = htons(ETH_P_IPV6);
    }
    if (count) {
// Not the last packet in this batch
    clone = skb_clone(buf, GFP_ATOMIC);
    if (!clone)
    goto error;
    skb_trim(clone, packet_len);
    usbnet_skb_return(dev, clone);
    skb_pull(buf, (packet_len + 3) & ~3);
    } else {
    skb_trim(buf, packet_len);
    if (s.current_rx_buf) {
    usbnet_skb_return(dev, buf);
    s.current_rx_buf = core::ptr::null_mut();
    return 0;
    }
    return 1;
    }
    }
    error:
    if (s.current_rx_buf) {
    dev_kfree_skb_any(s.current_rx_buf);
    s.current_rx_buf = core::ptr::null_mut();
    }
    dev.net.stats.rx_errors++;
    return 0;
    }
    static struct sk_buff *vl600_tx_fixup(struct usbnet *dev,
    struct sk_buff *skb, gfp_t flags)
    {
    struct sk_buff *ret;
    struct vl600_frame_hdr *frame;
    struct vl600_pkt_hdr *packet;
    let mut serial: static uint32_t = 1;
    let mut orig_len: c_int = skb.len - sizeof(struct ethhdr);
    let mut full_len: c_int = (skb.len + sizeof(struct vl600_frame_hdr) + 3) & ~3;
    frame = (struct vl600_frame_hdr *) skb.data;
    if (skb.len > sizeof(*frame) && skb.len == le32_to_cpup(&frame.len))
    return skb; /* Already encapsulated? */
    if (skb.len < sizeof(struct ethhdr))
// Drop, device can only deal with ethernet packets
    return core::ptr::null_mut();
    if (!skb_cloned(skb)) {
    let mut headroom: c_int = skb_headroom(skb);
    let mut tailroom: c_int = skb_tailroom(skb);
    if (tailroom >= full_len - skb.len - sizeof(*frame) &&
    headroom >= sizeof(*frame))
// There's enough head and tail room
    goto encapsulate;
    if (headroom + tailroom + skb.len >= full_len) {
// There's enough total room, just readjust
    skb.data = memmove(skb.head + sizeof(*frame),
    skb.data, skb.len);
    skb_set_tail_pointer(skb, skb.len);
    goto encapsulate;
    }
    }
// Alloc a new skb with the required size
    ret = skb_copy_expand(skb, sizeof(struct vl600_frame_hdr), full_len -
    skb.len - sizeof(struct vl600_frame_hdr), flags);
    dev_kfree_skb_any(skb);
    if (!ret)
    return ret;
    skb = ret;
    encapsulate:
// Packet header is same size as ethernet packet header
// (sizeof(*packet) == sizeof(struct ethhdr)), additionally the
// h_proto field is in the same place so we just leave it alone and
// overwrite the remaining fields.
//
    packet = (struct vl600_pkt_hdr *) skb.data;
// The VL600 wants IPv6 packets to have an IPv4 ethertype
// Since this modem only supports IPv4 and IPv6, just set all
// frames to 0x0800 (ETH_P_IP)
//
    packet.h_proto = htons(ETH_P_IP);
    memset(&packet.dummy, 0, sizeof(packet.dummy));
    packet.len = cpu_to_le32(orig_len);
    frame = skb_push(skb, sizeof(*frame));
    memset(frame, 0, sizeof(*frame));
    frame.len = cpu_to_le32(full_len);
    frame.serial = cpu_to_le32(serial++);
    frame.pkt_cnt = cpu_to_le32(1);
    if (skb.len < full_len) /* Pad */
    skb_put(skb, full_len - skb.len);
    return skb;
    }
    static const struct driver_info	vl600_info = {
    .description	= "LG VL600 modem",
    .flags		= FLAG_RX_ASSEMBLE | FLAG_WWAN,
    .bind		= vl600_bind,
    .unbind		= vl600_unbind,
    .status		= usbnet_cdc_status,
    .rx_fixup	= vl600_rx_fixup,
    .tx_fixup	= vl600_tx_fixup,
    };
    static const struct usb_device_id products[] = {
    {
    USB_DEVICE_AND_INTERFACE_INFO(0x1004, 0x61aa, USB_CLASS_COMM,
    USB_CDC_SUBCLASS_ETHERNET, USB_CDC_PROTO_NONE),
    .driver_info	= (unsigned long) &vl600_info,
    },
    {},	/* End */
    };
    MODULE_DEVICE_TABLE(usb, products);
    static struct usb_driver lg_vl600_driver = {
    .name		= "lg-vl600",
    .id_table	= products,
    .probe		= usbnet_probe,
    .disconnect	= usbnet_disconnect,
    .suspend	= usbnet_suspend,
    .resume		= usbnet_resume,
    .disable_hub_initiated_lpm = 1,
    };
    module_usb_driver(lg_vl600_driver);
    MODULE_AUTHOR("Anrzej Zaborowski");
    MODULE_DESCRIPTION("LG-VL600 modem's ethernet link");
    MODULE_LICENSE("GPL");
