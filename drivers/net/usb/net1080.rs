//! Automatically rewritten from C to Rust
//! Source: drivers/net/usb/net1080.c
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
// Net1080 based USB host-to-host cables
// Copyright (C) 2000-2005 by David Brownell
//
// #define	DEBUG			// error path messages, extra info
// #define	VERBOSE			// more; success messages

//
// Netchip 1080 driver ... http://www.netchip.com
// (Sept 2004:  End-of-life announcement has been sent.)
// Used in (some) LapLink cables
//

//
// NetChip framing of ethernet packets, supporting additional error
// checks for links that may drop bulk packets from inside messages.
// Odd USB length == always short read for last usb packet.
// - nc_header
// - Ethernet header (14 bytes)
// - payload
// - (optional padding byte, if needed so length becomes odd)
// - nc_trailer
//
// This framing is to be avoided for non-NetChip devices.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nc_header {
    pub all): __le16 hdr_len; // sizeof nc_header (LE,,
    pub ethhdr): __le16 packet_len; // payload size (including,
    pub packets: __le16 packet_id; // detects dropped,
pub const MIN_HEADER: c_int = 6;
// all else is optional, and must start with:
// __le16	vendorId;	// from usb-if
// __le16	productId;
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nc_trailer {
    pub packet_id: __le16,
    pub __packed: },
// packets may use FLAG_FRAMING_NC and optional pad

    + sizeof (struct ethhdr) \
    + (mtu) \
    + 1 \
    + sizeof (struct nc_trailer))

// packets _could_ be up to 64KB...
pub const NC_MAX_PACKET: c_int = 32767;
//
// Zero means no timeout; else, how long a 64 byte bulk packet may be queued
// before the hardware drops it.  If that's done, the driver will need to
// frame network packets to guard against the dropped USB packets.  The win32
// driver sets this for both sides of the link.
//

//
// We ignore most registers and EEPROM contents.
//

//
// Vendor specific requests to read/write data
//

    static int
    nc_vendor_read(struct usbnet *dev, u8 req, u8 regnum, u16 *retval_ptr)
    {
    int status = usbnet_read_cmd(dev, req,
    USB_DIR_IN | USB_TYPE_VENDOR |
    USB_RECIP_DEVICE,
    0, regnum, retval_ptr,
    pub retval_ptr): *mut sizeof,
    if (status > 0)
    pub 0: status =,
    if (!status)
    pub status: return,
    }
    static inline int
    nc_register_read(struct usbnet *dev, u8 regnum, u16 *retval_ptr)
    {
    pub retval_ptr): return nc_vendor_read(dev, REQUEST_REGISTER, regnum,,
    }
    static void
    nc_vendor_write(struct usbnet *dev, u8 req, u8 regnum, u16 value)
    {
    usbnet_write_cmd(dev, req,
    USB_DIR_OUT | USB_TYPE_VENDOR | USB_RECIP_DEVICE,
    pub 0): value, regnum, NULL,,
    }
    static inline void
    nc_register_write(struct usbnet *dev, u8 regnum, u16 value)
    {
    pub value): nc_vendor_write(dev, REQUEST_REGISTER, regnum,,
    }

#[no_mangle]
unsafe extern "C" fn nc_dump_registers(dev: *mut usbnet) {
    static void nc_dump_registers(struct usbnet *dev)
    {
    pub reg: u8,
    pub (u16)): *mut *mut u16 vp = kmalloc(sizeof,
    if (!vp)
    pub "registers:\n"): netdev_dbg(dev->net,,
    pub {: for (reg = 0; reg < 0x20; reg++),
    pub retval: c_int,
// reading some registers is trouble
    if (reg >= 0x08 && reg <= 0xf)
    if (reg >= 0x12 && reg <= 0x1e)
    pub vp): retval = nc_register_read(dev, reg,,
    if (retval < 0)
    netdev_dbg(dev.net, "reg [0x%x] ==> error %d\n",
    pub retval): reg,,
    else
    pub vp): *mut netdev_dbg(dev->net, "reg [0x%x] = 0x%x\n", reg,,
    }
    }

// -------------------------------------------------------------------------
//
// Control register
//
pub const USBCTL_WRITABLE_MASK: c_uint = 0x1f0f;
// bits 15-13 reserved, r/o

// bits 7-4 reserved, r/o

#[no_mangle]
pub unsafe extern "C" fn nc_dump_usbctl(dev: *mut usbnet, usbctl: u16) {
    static inline void nc_dump_usbctl(struct usbnet *dev, u16 usbctl)
    {
    netif_dbg(dev, link, dev.net,
    pub 0x%x\n",: "net1080 %s-%s usbctl 0x%x:%s%s%s%s%s; this%s%s; other%s%s; r/o,
    dev.udev.bus.bus_name, dev.udev.devpath,
    usbctl,
    (usbctl & USBCTL_ENABLE_LANG) ? " lang" : "",
    (usbctl & USBCTL_ENABLE_MFGR) ? " mfgr" : "",
    (usbctl & USBCTL_ENABLE_PROD) ? " prod" : "",
    (usbctl & USBCTL_ENABLE_SERIAL) ? " serial" : "",
    (usbctl & USBCTL_ENABLE_DEFAULTS) ? " defaults" : "",
    (usbctl & USBCTL_FLUSH_THIS) ? " FLUSH" : "",
    (usbctl & USBCTL_DISCONN_THIS) ? " DIS" : "",
    (usbctl & USBCTL_FLUSH_OTHER) ? " FLUSH" : "",
    (usbctl & USBCTL_DISCONN_OTHER) ? " DIS" : "",
    pub ~USBCTL_WRITABLE_MASK): usbctl &,
    }
// -------------------------------------------------------------------------
//
// Status register
//

pub const STATUS_UNSPEC_MASK: c_uint = 0x0c8c;

#[no_mangle]
pub unsafe extern "C" fn nc_dump_status(dev: *mut usbnet, status: u16) {
    static inline void nc_dump_status(struct usbnet *dev, u16 status)
    {
    netif_dbg(dev, link, dev.net,
    pub 0x%x\n",: "net1080 %s-%s status 0x%x: this (%c) PKT=%d%s%s%s; other PKT=%d%s%s%s; unspec,
    dev.udev.bus.bus_name, dev.udev.devpath,
    status,
// XXX the packet counts don't seem right
// (1 at reset, not 0); maybe UNSPEC too
    (status & STATUS_PORT_A) ? 'A' : 'B',
    STATUS_PACKETS_THIS(status),
    (status & STATUS_CONN_THIS) ? " CON" : "",
    (status & STATUS_SUSPEND_THIS) ? " SUS" : "",
    (status & STATUS_MAILBOX_THIS) ? " MBOX" : "",
    STATUS_PACKETS_OTHER(status),
    (status & STATUS_CONN_OTHER) ? " CON" : "",
    (status & STATUS_SUSPEND_OTHER) ? " SUS" : "",
    (status & STATUS_MAILBOX_OTHER) ? " MBOX" : "",
    pub STATUS_UNSPEC_MASK): status &,
    }
// -------------------------------------------------------------------------
//
// TTL register
//

// -------------------------------------------------------------------------
#[no_mangle]
unsafe extern "C" fn net1080_reset(dev: *mut usbnet) -> c_int {
    static int net1080_reset(struct usbnet *dev)
    {
    pub ttl: u16 usbctl, status,,
    pub vp: u16,
    pub retval: c_int,
// nc_dump_registers(dev);
    if ((retval = nc_register_read(dev, REG_STATUS, &vp)) < 0) {
    netdev_dbg(dev.net, "can't read %s-%s status: %d\n",
    pub retval): dev->udev->bus->bus_name, dev->udev->devpath,,
    pub done: goto,
    }
    pub vp: status =,
    pub status): nc_dump_status(dev,,
    if ((retval = nc_register_read(dev, REG_USBCTL, &vp)) < 0) {
    pub retval): netdev_dbg(dev->net, "can't read USBCTL, %d\n",,
    pub done: goto,
    }
    pub vp: usbctl =,
    pub usbctl): nc_dump_usbctl(dev,,
    nc_register_write(dev, REG_USBCTL,
    pub USBCTL_FLUSH_OTHER): USBCTL_FLUSH_THIS |,
    if ((retval = nc_register_read(dev, REG_TTL, &vp)) < 0) {
    pub retval): netdev_dbg(dev->net, "can't read TTL, %d\n",,
    pub done: goto,
    }
    pub vp: ttl =,
    nc_register_write(dev, REG_TTL,
    pub ): MK_TTL(NC_READ_TTL_MS, TTL_OTHER(ttl)),
    pub NC_READ_TTL_MS): netdev_dbg(dev->net, "assigned TTL, %d ms\n",,
    netif_info(dev, link, dev.net, "port %c, peer %sconnected\n",
    (status & STATUS_PORT_A) ? 'A' : 'B',
    pub "dis"): (status & STATUS_CONN_OTHER) ? "" :,
    pub 0: retval =,
    done:
    pub retval: return,
    }
#[no_mangle]
unsafe extern "C" fn net1080_check_connect(dev: *mut usbnet) -> c_int {
    static int net1080_check_connect(struct usbnet *dev)
    {
    pub retval: c_int,
    pub status: u16,
    pub vp: u16,
    pub &vp): retval = nc_register_read(dev, REG_STATUS,,
    pub vp: status =,
    if (retval != 0) {
    pub retval): netdev_dbg(dev->net, "net1080_check_conn read - %d\n",,
    pub retval: return,
    }
    if ((status & STATUS_CONN_OTHER) != STATUS_CONN_OTHER)
    pub -ENOLINK: return,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn nc_ensure_sync(dev: *mut usbnet) {
    static void nc_ensure_sync(struct usbnet *dev)
    {
    if (++dev.frame_errors <= 5)
    if (usbnet_write_cmd_async(dev, REQUEST_REGISTER,
    USB_DIR_OUT | USB_TYPE_VENDOR |
    USB_RECIP_DEVICE,
    USBCTL_FLUSH_THIS |
    USBCTL_FLUSH_OTHER,
    REG_USBCTL, core::ptr::null_mut(), 0))
    netif_dbg(dev, rx_err, dev.net,
    pub errors\n"): "flush net1080; too many framing,
    pub 0: dev->frame_errors =,
    }
#[no_mangle]
unsafe extern "C" fn net1080_rx_fixup(dev: *mut usbnet, skb: *mut sk_buff) -> c_int {
    static int net1080_rx_fixup(struct usbnet *dev, struct sk_buff *skb)
    {
    pub header: *mut nc_header,
    pub trailer: *mut nc_trailer,
    pub packet_len: u16 hdr_len,,
// This check is no longer done by usbnet
    if (skb.len < dev.net.hard_header_len)
    pub 0: return,
    if (!(skb.len & 0x01)) {
    netdev_dbg(dev.net, "rx framesize %d range %d..%d mtu %d\n",
    skb.len, dev.net.hard_header_len, dev.hard_mtu,
    pub 0: return,
    }
    pub skb->data: *mut *mut header = (struct nc_header ),
    pub le16_to_cpup(&header->hdr_len): hdr_len =,
    pub le16_to_cpup(&header->packet_len): packet_len =,
    if (FRAMED_SIZE(packet_len) > NC_MAX_PACKET) {
    pub packet_len): netdev_dbg(dev->net, "packet too big, %d\n",,
    pub 0: return,
    } else if (hdr_len < MIN_HEADER) {
    pub hdr_len): netdev_dbg(dev->net, "header too short, %d\n",,
    pub 0: return,
    } else if (hdr_len > MIN_HEADER) {
// out of band data for us?
    pub MIN_HEADER): netdev_dbg(dev->net, "header OOB, %d bytes\n", hdr_len -,
// switch (vendor/product ids) { ... }
    }
    pub hdr_len): skb_pull(skb,,
    trailer = (struct nc_trailer *)
    pub trailer): *mut (skb->data + skb->len - sizeof,
    pub trailer): *mut skb_trim(skb, skb->len - sizeof,
    if ((packet_len & 0x01) == 0) {
    if (packet_len >= skb.len || skb.data[packet_len] != PAD_BYTE) {
    pub pad\n"): netdev_dbg(dev->net, "bad,
    pub 0: return,
    }
    pub 1): skb_trim(skb, skb->len -,
    }
    if (skb.len != packet_len) {
    netdev_dbg(dev.net, "bad packet len %d (expected %d)\n",
    pub packet_len): skb->len,,
    pub 0: return,
    }
    if (header.packet_id != get_unaligned(&trailer.packet_id)) {
    netdev_dbg(dev.net, "(2+ dropped) rx packet_id mismatch 0x%x 0x%x\n",
    le16_to_cpu(header.packet_id),
    pub 0: return,
    }

    netdev_dbg(dev.net, "frame <rx h %d p %d id %d\n", header.hdr_len,
    pub header->packet_id): header->packet_len,,

    pub 0: dev->frame_errors =,
    pub 1: return,
    }
    static struct sk_buff *
    net1080_tx_fixup(struct usbnet *dev, struct sk_buff *skb, gfp_t flags)
    {
    pub skb2: *mut sk_buff,
    pub NULL: *mut *mut nc_header header =,
    pub NULL: *mut *mut nc_trailer trailer =,
    pub nc_trailer): int padlen = sizeof (struct,
    pub skb->len: int len =,
    if (!((len + padlen + sizeof (struct nc_header)) & 0x01))
    if (!skb_cloned(skb)) {
    pub skb_headroom(skb): int headroom =,
    pub skb_tailroom(skb): int tailroom =,
    if (padlen <= tailroom &&
    sizeof(struct nc_header) <= headroom)
// There's enough head and tail room
    pub encapsulate: goto,
    if ((sizeof (struct nc_header) + padlen) <
    (headroom + tailroom)) {
// There's enough total room, so just readjust
    skb.data = memmove(skb.head
    + sizeof (struct nc_header),
    pub skb->len): skb->data,,
    pub len): skb_set_tail_pointer(skb,,
    pub encapsulate: goto,
    }
    }
// Create a new skb to use with the correct size
    skb2 = skb_copy_expand(skb,
    sizeof (struct nc_header),
    padlen,
    if (!skb2)
    pub skb2: return,
    pub skb2: skb =,
    encapsulate:
// header first
    pub header): *mut header = skb_push(skb, sizeof,
    pub (*header)): *mut header->hdr_len = cpu_to_le16(sizeof,
    pub cpu_to_le16(len): header->packet_len =,
    pub cpu_to_le16((u16)dev->xid++): header->packet_id =,
// maybe pad; then trailer
    if (!((skb.len + sizeof *trailer) & 0x01))
    pub PAD_BYTE): skb_put_u8(skb,,
    pub trailer): *mut trailer = skb_put(skb, sizeof,
    pub &trailer->packet_id): put_unaligned(header->packet_id,,

    netdev_dbg(dev.net, "frame >tx h %d p %d id %d\n",
    header.hdr_len, header.packet_len,

    pub skb: return,
    }
#[no_mangle]
unsafe extern "C" fn net1080_bind(dev: *mut usbnet, intf: *mut usb_interface) -> c_int {
    static int net1080_bind(struct usbnet *dev, struct usb_interface *intf)
    {
    unsigned	extra = sizeof (struct nc_header)
    + 1
    pub nc_trailer): + sizeof (struct,
    pub extra: dev->net->hard_header_len +=,
    pub dev->net->mtu: dev->rx_urb_size = dev->net->hard_header_len +,
    pub NC_MAX_PACKET: dev->hard_mtu =,
    pub intf): return usbnet_get_endpoints (dev,,
    }
    static const struct driver_info	net1080_info = {
    .description =	"NetChip TurboCONNECT",
    .flags =	FLAG_POINTTOPOINT | FLAG_FRAMING_NC,
    .bind =		net1080_bind,
    .reset =	net1080_reset,
    .check_connect = net1080_check_connect,
    .rx_fixup =	net1080_rx_fixup,
    .tx_fixup =	net1080_tx_fixup,
}

    static const struct usb_device_id	products [] = {
    {
    USB_DEVICE(0x0525, 0x1080),	// NetChip ref design
    .driver_info =	(unsigned long) &net1080_info,
    }, {
    USB_DEVICE(0x06D0, 0x0622),	// Laplink Gold
    .driver_info =	(unsigned long) &net1080_info,
    },
    { },		// END
    };
    MODULE_DEVICE_TABLE(usb, products);
    static struct usb_driver net1080_driver = {
    .name =		"net1080",
    .id_table =	products,
    .probe =	usbnet_probe,
    .disconnect =	usbnet_disconnect,
    .suspend =	usbnet_suspend,
    .resume =	usbnet_resume,
    .disable_hub_initiated_lpm = 1,
    };
    module_usb_driver(net1080_driver);
    MODULE_AUTHOR("David Brownell");
    MODULE_DESCRIPTION("NetChip 1080 based USB Host-to-Host Links");
    MODULE_LICENSE("GPL");
