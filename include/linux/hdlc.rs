//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/hdlc.h
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
// Generic HDLC support routines for Linux
//
// Copyright (C) 1999-2005 Krzysztof Halasa <khc@pm.waw.pl>
//

// This structure is a private property of HDLC protocols.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdlc_proto {
    pub dev): *mut *mut int (open)(struct net_device,
    pub dev): *mut *mut void (close)(struct net_device,
    pub /: *mut *mut *mut *mut void (start)(struct net_device dev); / if open & DCD,
    pub /: *mut *mut *mut *mut void (stop)(struct net_device dev); / if open & !DCD,
    pub dev): *mut *mut void (detach)(struct net_device,
    pub ifs): *mut *mut *mut int (ioctl)(struct net_device dev, struct if_settings,
    pub dev): *mut *mut *mut __be16 (type_trans)(struct sk_buff skb, struct net_device,
    pub skb): *mut *mut int (netif_rx)(struct sk_buff,
    pub dev): *mut *mut *mut netdev_tx_t (xmit)(struct sk_buff skb, struct net_device,
    pub module: *mut module,
    pub /: *mut *mut *mut hdlc_proto next; / next protocol in the list,
}

// Pointed to by netdev_priv(dev)
// used by HDLC layer to take control over HDLC device from hw driver
// hardware driver must handle this instead of dev->hard_start_xmit
// Things below are for HDLC layer internal use only
// Exported from hdlc module
// Called by hardware driver when a user requests HDLC service
extern "C" {
    pub fn hdlc_ioctl(dev: *mut net_device, ifs: *mut if_settings) -> c_int;
}
// Must be used by hardware driver on module startup/exit

extern "C" {
    pub fn unregister_hdlc_device(dev: *mut net_device);
}
extern "C" {
    pub fn register_hdlc_protocol(proto: *mut hdlc_proto);
}
extern "C" {
    pub fn unregister_hdlc_protocol(proto: *mut hdlc_proto);
}
extern "C" {
    pub fn netdev_priv(_arg: dev) -> return;
}
// Must be called by hardware driver when HDLC device is being opened
extern "C" {
    pub fn hdlc_open(dev: *mut net_device) -> c_int;
}
// Must be called by hardware driver when HDLC device is being closed
extern "C" {
    pub fn hdlc_close(dev: *mut net_device);
}
// Must be pointed to by hw driver's dev->netdev_ops->ndo_start_xmit
extern "C" {
    pub fn hdlc_start_xmit(skb: *mut sk_buff, dev: *mut net_device) -> netdev_tx_t;
}
// May be used by hardware driver to gain control over HDLC device
extern "C" {
    pub fn detach_hdlc_protocol(dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn htons(_arg: ETH_P_HDLC) -> return;
}
