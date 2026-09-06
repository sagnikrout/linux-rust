//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/netpoll.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Common code for low-level network console, dump, and debugger code
//
// Derived from netconsole, kgdb-over-ethernet, and netdump patches
//

#[repr(C)]
#[derive(Copy, Clone)]
pub union inet_addr {
    pub ip: __be32,
    pub in6: in6_addr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netpoll {
    pub dev: *mut net_device,
    pub dev_tracker: netdevice_tracker,
//
// Either dev_name or dev_mac can be used to specify the local
// interface - dev_name is used if it is a nonempty string, else
// dev_mac is used.
//
    pub dev_name: [c_char; IFNAMSIZ],
    pub dev_mac: [u8; ETH_ALEN],
    pub name: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netpoll_info {
    pub refcnt: refcount_t,
    pub dev_lock: semaphore,
    pub txq: sk_buff_head,
    pub tx_work: delayed_work,
    pub rcu: rcu_head,
}

extern "C" {
    pub fn netpoll_poll_dev(dev: *mut net_device);
}
extern "C" {
    pub fn netpoll_poll_disable(dev: *mut net_device);
}
extern "C" {
    pub fn netpoll_poll_enable(dev: *mut net_device);
}

extern "C" {
    pub fn __netpoll_setup(np: *mut netpoll, ndev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn __netpoll_free(np: *mut netpoll);
}
extern "C" {
    pub fn netpoll_cleanup(np: *mut netpoll);
}
extern "C" {
    pub fn do_netpoll_cleanup(np: *mut netpoll);
}
extern "C" {
    pub fn netpoll_send_skb(np: *mut netpoll, skb: *mut sk_buff) -> netdev_tx_t;
}
extern "C" {
    pub fn netpoll_zap_completion_queue();
}
extern "C" {
    pub fn netpoll_get_carrier_timeout() -> c_uint;
}

extern "C" {
    pub fn irqs_disabled() -> return;
}

