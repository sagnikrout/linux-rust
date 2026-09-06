//! Automatically rewritten from C to Rust
//! Source: drivers/net/usb/gl620a.c
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
// GeneSys GL620USB-A based links
// Copyright (C) 2001 by Jiun-Jie Huang <huangjj@genesyslogic.com.tw>
// Copyright (C) 2001 by Stanislav Brabec <utx@penguin.cz>
//
// #define	DEBUG			// error path messages, extra info
// #define	VERBOSE			// more; success messages

//
// GeneSys GL620USB-A (www.genesyslogic.com.tw)
//
// ... should partially interop with the Win32 driver for this hardware.
// The GeneSys docs imply there's some NDIS issue motivating this framing.
//
// Some info from GeneSys:
// - GL620USB-A is full duplex; GL620USB is only half duplex for bulk.
// (Some cables, like the BAFO-100c, use the half duplex version.)
// - For the full duplex model, the low bit of the version code says
// which side is which ("left/right").
// - For the half duplex type, a control/interrupt handshake settles
// the transfer direction.  (That's disabled here, partially coded.)
// A control URB would block until other side writes an interrupt.
//
// Original code from Jiun-Jie Huang <huangjj@genesyslogic.com.tw>
// and merged into "usbnet" by Stanislav Brabec <utx@penguin.cz>.
//
// control msg write command
pub const GENELINK_CONNECT_WRITE: c_uint = 0xF0;
// interrupt pipe index
pub const GENELINK_INTERRUPT_PIPE: c_uint = 0x03;
// interrupt read buffer size
pub const INTERRUPT_BUFSIZE: c_uint = 0x08;
// interrupt pipe interval value
pub const GENELINK_INTERRUPT_INTERVAL: c_uint = 0x10;
// max transmit packet number per transmit
pub const GL_MAX_TRANSMIT_PACKETS: c_int = 32;
// max packet length
pub const GL_MAX_PACKET_LEN: c_int = 1514;
// max receive buffer size

    (((GL_MAX_PACKET_LEN + 4) * GL_MAX_TRANSMIT_PACKETS) + 4)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gl_packet {
    pub packet_length: __le32,
    pub packet_data: [c_char; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gl_header {
    pub packet_count: __le32,
    pub packets: gl_packet,
}

#[no_mangle]
unsafe extern "C" fn genelink_rx_fixup(dev: *mut usbnet, skb: *mut sk_buff) -> c_int {
    static int genelink_rx_fixup(struct usbnet *dev, struct sk_buff *skb)
    {
    struct gl_header	*header;
    struct gl_packet	*packet;
    struct sk_buff		*gl_skb;
    u32			size;
    u32			count;
// This check is no longer done by usbnet
    if (skb.len < dev.net.hard_header_len)
    return 0;
    header = (struct gl_header *) skb.data;
// get the packet count of the received skb
    count = le32_to_cpu(header.packet_count);
    if (count > GL_MAX_TRANSMIT_PACKETS) {
    netdev_dbg(dev.net,
    "genelink: invalid received packet count %u\n",
    count);
    return 0;
    }
// set the current packet pointer to the first packet
    packet = &header.packets;
// decrement the length for the packet count size 4 bytes
    skb_pull(skb, 4);
    while (count > 1) {
// get the packet length
    size = le32_to_cpu(packet.packet_length);
// this may be a broken packet
    if (size > GL_MAX_PACKET_LEN) {
    netdev_dbg(dev.net, "genelink: invalid rx length %d\n",
    size);
    return 0;
    }
    if (!skb_pull(skb, size + 4))
    return 0;
// allocate the skb for the individual packet
    gl_skb = alloc_skb(size, GFP_ATOMIC);
    if (gl_skb) {
// copy the packet data to the new skb
    skb_put_data(gl_skb, packet.packet_data, size);
    usbnet_skb_return(dev, gl_skb);
    }
// advance to the next packet
    packet = (struct gl_packet *)&packet.packet_data[size];
    count--;
    }
// skip the packet length field 4 bytes
    skb_pull(skb, 4);
    if (skb.len > GL_MAX_PACKET_LEN) {
    netdev_dbg(dev.net, "genelink: invalid rx length %d\n",
    skb.len);
    return 0;
    }
    return 1;
    }
    static struct sk_buff *
    genelink_tx_fixup(struct usbnet *dev, struct sk_buff *skb, gfp_t flags)
    {
    int 	padlen;
    let mut length: c_int = skb.len;
    let mut headroom: c_int = skb_headroom(skb);
    let mut tailroom: c_int = skb_tailroom(skb);
    __le32	*packet_count;
    __le32	*packet_len;
// FIXME:  magic numbers, bleech
    padlen = ((skb.len + (4 + 4*1)) % 64) ? 0 : 1;
    if ((!skb_cloned(skb))
    && ((headroom + tailroom) >= (padlen + (4 + 4*1)))) {
    if ((headroom < (4 + 4*1)) || (tailroom < padlen)) {
    skb.data = memmove(skb.head + (4 + 4*1),
    skb.data, skb.len);
    skb_set_tail_pointer(skb, skb.len);
    }
    } else {
    struct sk_buff	*skb2;
    skb2 = skb_copy_expand(skb, (4 + 4*1) , padlen, flags);
    dev_kfree_skb_any(skb);
    skb = skb2;
    if (!skb)
    return core::ptr::null_mut();
    }
// attach the packet count to the header
    packet_count = skb_push(skb, (4 + 4 * 1));
    packet_len = packet_count + 1;
// packet_count = cpu_to_le32(1);
// packet_len = cpu_to_le32(length);
// add padding byte
    if ((skb.len % dev.maxpacket) == 0)
    skb_put(skb, 1);
    return skb;
    }
#[no_mangle]
unsafe extern "C" fn genelink_bind(dev: *mut usbnet, intf: *mut usb_interface) -> c_int {
    static int genelink_bind(struct usbnet *dev, struct usb_interface *intf)
    {
    dev.hard_mtu = GL_RCV_BUF_SIZE;
    dev.net.hard_header_len += 4;
    return usbnet_get_endpoints(dev, intf);
    }
    static const struct driver_info	genelink_info = {
    .description =	"Genesys GeneLink",
    .flags =	FLAG_POINTTOPOINT | FLAG_FRAMING_GL | FLAG_NO_SETINT,
    .bind =		genelink_bind,
    .rx_fixup =	genelink_rx_fixup,
    .tx_fixup =	genelink_tx_fixup,
    .in = 1, .out = 2,

    .check_connect =genelink_check_connect,

    };
    static const struct usb_device_id	products [] = {
    {
    USB_DEVICE(0x05e3, 0x0502),	// GL620USB-A
    .driver_info =	(unsigned long) &genelink_info,
    },
// NOT: USB_DEVICE(0x05e3, 0x0501),	// GL620USB
// that's half duplex, not currently supported
//
    { },		// END
    };
    MODULE_DEVICE_TABLE(usb, products);
    static struct usb_driver gl620a_driver = {
    .name =		"gl620a",
    .id_table =	products,
    .probe =	usbnet_probe,
    .disconnect =	usbnet_disconnect,
    .suspend =	usbnet_suspend,
    .resume =	usbnet_resume,
    .disable_hub_initiated_lpm = 1,
    };
    module_usb_driver(gl620a_driver);
    MODULE_AUTHOR("Jiun-Jie Huang");
    MODULE_DESCRIPTION("GL620-USB-A Host-to-Host Link cables");
    MODULE_LICENSE("GPL");
