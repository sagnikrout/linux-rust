//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/usb/usbnet.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// USB Networking Link Interface
//
// Copyright (C) 2000-2005 by David Brownell <dbrownell@users.sourceforge.net>
// Copyright (C) 2003-2005 David Hollis <dhollis@davehollis.com>
//

// interface from usbnet core to each USB networking link we handle
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbnet {
// housekeeping
    pub udev: *mut usb_device,
    pub intf: *mut usb_interface,
    pub driver_info: *const driver_info,
    pub driver_name: *const c_char,
    pub driver_priv: *mut c_void,
    pub wait: wait_queue_head_t,
    pub phy_mutex: mutex,
    pub suspend_count: c_uchar,
    pub pkt_err: unsigned char pkt_cnt,,
    pub tx_qlen: unsigned short rx_qlen,,
    pub can_dma_sg:1: unsigned,
// i/o info: pipes etc
    pub out: unsigned in,,
    pub status: *mut usb_host_endpoint,
    pub maxpacket: unsigned,
    pub delay: timer_list,
    pub padding_pkt: *const c_char,
// protocol/interface state
    pub net: *mut net_device,
    pub msg_enable: c_int,
    pub data: [c_ulong; 5],
    pub xid: u32,
    pub /: *mut *mut u32 hard_mtu; / count any extra framing,
    pub /: *mut *mut size_t rx_urb_size; / size for rx urbs,
    pub mii: mii_if_info,
    pub /: *mut *mut long rx_speed; / If MII not used,
    pub /: *mut *mut long tx_speed; / If MII not used,

// various kinds of pending driver work
    pub rxq: sk_buff_head,
    pub txq: sk_buff_head,
    pub done: sk_buff_head,
    pub rxq_pause: sk_buff_head,
    pub interrupt: *mut urb,
    pub interrupt_count: unsigned,
    pub interrupt_mutex: mutex,
    pub deferred: usb_anchor,
    pub bh_work: work_struct,
    pub bql_spinlock: spinlock_t,
    pub kevent: work_struct,
    pub flags: c_ulong,

// This one is special, as it indicates that the device is going away
// there are cyclic dependencies between tasklet, timer and bh
// that must be broken
//

}

extern "C" {
    pub fn test_bit(_arg: EVENT_UNPLUG, _arg: &ubn->flags) -> return;
}
extern "C" {
    pub fn to_usb_driver(_arg: intf->dev.driver) -> return;
}
// interface from the device/framing level "minidriver" to core
#[repr(C)]
#[derive(Copy, Clone)]
pub struct driver_info {
    pub description: *mut c_char,
    pub flags: c_int,
// framing is CDC Ethernet, not writing ZLPs (hw issues), or optionally:
pub const FLAG_FRAMING_NC: c_uint = 0x0001		/* guard against device dropouts */;
pub const FLAG_FRAMING_GL: c_uint = 0x0002		/* genelink batches packets */;
pub const FLAG_FRAMING_Z: c_uint = 0x0004		/* zaurus adds a trailer */;
pub const FLAG_FRAMING_RN: c_uint = 0x0008		/* RNDIS batches, plus huge header */;
pub const FLAG_NO_SETINT: c_uint = 0x0010		/* device can't set_interface() */;
pub const FLAG_ETHER: c_uint = 0x0020		/* maybe use "eth%d" names */;
pub const FLAG_FRAMING_AX: c_uint = 0x0040		/* AX88772/178 packets */;
pub const FLAG_WLAN: c_uint = 0x0080		/* use "wlan%d" names */;
pub const FLAG_AVOID_UNLINK_URBS: c_uint = 0x0100	/* don't unlink urbs at usbnet_stop() */;
pub const FLAG_SEND_ZLP: c_uint = 0x0200		/* hw requires ZLPs are sent */;
pub const FLAG_WWAN: c_uint = 0x0400		/* use "wwan%d" names */;
pub const FLAG_LINK_INTR: c_uint = 0x0800		/* updates link (carrier) status */;
pub const FLAG_POINTTOPOINT: c_uint = 0x1000	/* possibly use "usb%d" names */;
//
// Indicates to usbnet, that USB driver accumulates multiple IP packets.
// Affects statistic (counters) and short packet handling.
//
pub const FLAG_MULTI_PACKET: c_uint = 0x2000;
pub const FLAG_RX_ASSEMBLE: c_uint = 0x4000	/* rx packets may span >1 frames */;
pub const FLAG_NOARP: c_uint = 0x8000	/* device can't do ARP */;
pub const FLAG_NOMAXMTU: c_uint = 0x10000	/* allow max_mtu above hard_mtu */;
// init device ... can sleep, or cause probe() failure
    pub ): *mut *mut *mut int (bind)(struct usbnet , struct usb_interface,
// cleanup device ... can sleep, but can't fail
    pub ): *mut *mut *mut void (unbind)(struct usbnet , struct usb_interface,
// reset device ... can sleep
    pub ): *mut *mut int (reset)(struct usbnet,
// stop device ... can sleep
    pub ): *mut *mut int (stop)(struct usbnet,
// see if peer is connected ... can sleep
    pub ): *mut *mut int (check_connect)(struct usbnet,
// (dis)activate runtime power management
    pub int): *mut *mut *mut int (manage_power)(struct usbnet ,,
// for status polling
    pub ): *mut *mut *mut void (status)(struct usbnet , struct urb,
// link reset handling, called from defer_kevent
    pub ): *mut *mut int (link_reset)(struct usbnet,
// fixup rx packet (strip framing)
    pub skb): *mut *mut *mut int (rx_fixup)(struct usbnet dev, struct sk_buff,
// fixup tx packet (add framing)
    pub flags): *mut *mut sk_buff skb, gfp_t,
// recover from timeout
    pub dev): *mut *mut void (recover)(struct usbnet,
// early initialization code, can sleep. This is for minidrivers
// having 'subminidrivers' that need to do extra initialization
// right after minidriver have initialized hardware.
    pub dev): *mut *mut int (early_init)(struct usbnet,
// called by minidriver when receiving indication
    pub indlen): *mut *mut *mut *mut void (indication)(struct usbnet dev, void ind, int,
// rx mode change (device changes address list filtering)
    pub dev): *mut *mut void (set_rx_mode)(struct usbnet,
// for new devices, use the descriptor-reading code instead
    pub /: *mut *mut int in; / rx endpoint,
    pub /: *mut *mut int out; / tx endpoint,
    pub /: *mut *mut unsigned long data; / Misc driver specific data,
}

// Minidrivers are just drivers using the "usbnet" core as a powerful
// network-specific subroutine library ... that happens to do pretty
// much everything except custom framing and chip-specific stuff.
//
extern "C" {
    pub fn usbnet_probe(: *mut usb_interface, : *const usb_device_id) -> c_int;
}
extern "C" {
    pub fn usbnet_suspend(: *mut usb_interface, _arg: pm_message_t) -> c_int;
}
extern "C" {
    pub fn usbnet_resume(: *mut usb_interface) -> c_int;
}
extern "C" {
    pub fn usbnet_disconnect(: *mut usb_interface);
}
extern "C" {
    pub fn usbnet_device_suggests_idle(dev: *mut usbnet);
}
// Drivers that reuse some of the standard USB CDC infrastructure
// (notably, using multiple interfaces according to the CDC
// union descriptor) get some helper code.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdc_state {
    pub header: *mut usb_cdc_header_desc,
    pub u: *mut usb_cdc_union_desc,
    pub ether: *mut usb_cdc_ether_desc,
    pub control: *mut usb_interface,
    pub data: *mut usb_interface,
}

extern "C" {
    pub fn usbnet_cdc_update_filter(dev: *mut usbnet);
}
extern "C" {
    pub fn usbnet_generic_cdc_bind(: *mut usbnet, : *mut usb_interface) -> c_int;
}
extern "C" {
    pub fn usbnet_ether_cdc_bind(dev: *mut usbnet, intf: *mut usb_interface) -> c_int;
}
extern "C" {
    pub fn usbnet_cdc_bind(: *mut usbnet, : *mut usb_interface) -> c_int;
}
extern "C" {
    pub fn usbnet_cdc_unbind(: *mut usbnet, : *mut usb_interface);
}
extern "C" {
    pub fn usbnet_cdc_status(: *mut usbnet, : *mut urb);
}
extern "C" {
    pub fn usbnet_cdc_zte_rx_fixup(dev: *mut usbnet, skb: *mut sk_buff) -> c_int;
}
// CDC and RNDIS support the same host-chosen packet filters for IN transfers

// we record the state for each of our queued skbs
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum skb_state {
    illegal = 0,
    tx_start, tx_done,
    rx_start, rx_done, rx_cleanup,
    unlink_start
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct skb_data {
    pub urb: *mut urb,
    pub dev: *mut usbnet,
    pub state: skb_state,
    pub length: c_long,
    pub packets: c_ulong,
}

// Drivers that set FLAG_MULTI_PACKET must call this in their
// tx_fixup method before returning an skb.
//
extern "C" {
    pub fn usbnet_open(net: *mut net_device) -> c_int;
}
extern "C" {
    pub fn usbnet_stop(net: *mut net_device) -> c_int;
}
extern "C" {
    pub fn usbnet_tx_timeout(net: *mut net_device, txqueue: c_uint);
}
extern "C" {
    pub fn usbnet_change_mtu(net: *mut net_device, new_mtu: c_int) -> c_int;
}
extern "C" {
    pub fn usbnet_get_endpoints(: *mut usbnet, : *mut usb_interface) -> c_int;
}
extern "C" {
    pub fn usbnet_get_ethernet_addr(: *mut usbnet, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn usbnet_defer_kevent(: *mut usbnet, _arg: c_int);
}
extern "C" {
    pub fn usbnet_skb_return(: *mut usbnet, : *mut sk_buff);
}
extern "C" {
    pub fn usbnet_unlink_rx_urbs(: *mut usbnet);
}
extern "C" {
    pub fn usbnet_pause_rx(: *mut usbnet);
}
extern "C" {
    pub fn usbnet_resume_rx(: *mut usbnet);
}
extern "C" {
    pub fn usbnet_purge_paused_rxq(: *mut usbnet);
}
extern "C" {
    pub fn usbnet_get_link(net: *mut net_device) -> u32;
}
extern "C" {
    pub fn usbnet_get_msglevel(: *mut net_device) -> u32;
}
extern "C" {
    pub fn usbnet_set_msglevel(: *mut net_device, _arg: u32);
}
extern "C" {
    pub fn usbnet_set_rx_mode(net: *mut net_device);
}
extern "C" {
    pub fn usbnet_get_drvinfo(: *mut net_device, : *mut ethtool_drvinfo);
}
extern "C" {
    pub fn usbnet_mii_ioctl(net: *mut net_device, rq: *mut ifreq, cmd: c_int) -> c_int;
}
extern "C" {
    pub fn usbnet_nway_reset(net: *mut net_device) -> c_int;
}
extern "C" {
    pub fn usbnet_manage_power(: *mut usbnet, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn usbnet_link_change(: *mut usbnet, _arg: bool, _arg: bool);
}
extern "C" {
    pub fn usbnet_status_start(dev: *mut usbnet, mem_flags: gfp_t) -> c_int;
}
extern "C" {
    pub fn usbnet_status_stop(dev: *mut usbnet);
}
extern "C" {
    pub fn usbnet_update_max_qlen(dev: *mut usbnet);
}
