//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ppp_channel.h
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
// Definitions for the interface between the generic PPP code
// and a PPP channel.
//
// A PPP channel provides a way for the generic PPP code to send
// and receive packets over some sort of communications medium.
// Packets are stored in sk_buffs and have the 2-byte PPP protocol
// number at the start, but not the address and control bytes.
//
// Copyright 1999 Paul Mackerras.
//
// ==FILEVERSION 20000322==
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ppp_channel_ops {
// Send a packet (or multilink fragment) on this channel.
    pub ): *mut *mut *mut int (start_xmit)(struct ppp_channel , struct sk_buff,
// Handle an ioctl call that has come in via /dev/ppp.
    pub long): *mut *mut *mut int (ioctl)(struct ppp_channel , unsigned int, unsigned,
    pub ): *const ppp_channel,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ppp_channel {
    pub /: *mut *mut *mut void private; / channel private data,
    pub /: *const *const *const ppp_channel_ops ops; / operations for this channel,
    pub /: *mut *mut int mtu; / max transmit packet size,
    pub /: *mut *mut int hdrlen; / amount of headroom channel needs,
    pub /: *mut *mut *mut void ppp; / opaque to channel,
    pub /: *mut *mut int speed; / transfer rate (bytes/second),
    pub /: *mut *mut bool direct_xmit; / no qdisc, xmit directly,
}

// Called by the channel when it can send some more data.
extern "C" {
    pub fn ppp_output_wakeup(: *mut ppp_channel);
}
// Called by the channel to process a received PPP packet.
extern "C" {
    pub fn ppp_input(: *mut ppp_channel, : *mut sk_buff);
}
// Called by the channel when an input error occurs, indicating
extern "C" {
    pub fn ppp_input_error(: *mut ppp_channel);
}
// Attach a channel to a given PPP unit in specified net.
extern "C" {
    pub fn ppp_register_net_channel(: *mut net, : *mut ppp_channel) -> c_int;
}
// Attach a channel to a given PPP unit.
extern "C" {
    pub fn ppp_register_channel(: *mut ppp_channel) -> c_int;
}
// Detach a channel from its PPP unit (e.g. on hangup).
extern "C" {
    pub fn ppp_unregister_channel(: *mut ppp_channel);
}
// Get the channel number for a channel
extern "C" {
    pub fn ppp_channel_index(: *mut ppp_channel) -> c_int;
}
// Get the unit number associated with a channel, or -1 if none
extern "C" {
    pub fn ppp_unit_number(: *mut ppp_channel) -> c_int;
}
// Get the device name associated with a channel, or NULL if none.
// Caller must hold RCU read lock.
//
// SMP locking notes:
// The channel code must ensure that when it calls ppp_unregister_channel,
// nothing is executing in any of the procedures above, for that
// channel.  The generic layer will ensure that nothing is executing
// in the start_xmit and ioctl routines for the channel by the time
// that ppp_unregister_channel returns.
//

