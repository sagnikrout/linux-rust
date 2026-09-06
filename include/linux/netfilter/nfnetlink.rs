//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/netfilter/nfnetlink.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfnl_info {
    pub net: *mut net,
    pub sk: *mut sock,
    pub nlh: *const nlmsghdr,
    pub nfmsg: *const nfgenmsg,
    pub extack: *mut netlink_ext_ack,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfnl_callback_type {
    NFNL_CB_UNSPEC	= 0,
    NFNL_CB_MUTEX,
    NFNL_CB_RCU,
    NFNL_CB_BATCH,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfnl_callback {
    pub cda[]): *const *const nlattr,
    pub policy: *const nla_policy,
    pub type: nfnl_callback_type,
    pub attr_count: __u16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfnl_abort_action {
    NFNL_ABORT_NONE		= 0,
    NFNL_ABORT_AUTOLOAD,
    NFNL_ABORT_VALIDATE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfnetlink_subsystem {
    pub name: *const c_char,
    pub /: *mut *mut __u8 subsys_id; / nfnetlink subsystem ID,
    pub /: *mut *mut __u8 cb_count; / number of callbacks,
    pub /: *const *const *const nfnl_callback cb; / callback for individual types,
    pub owner: *mut module,
    pub skb): *mut *mut *mut int (commit)(struct net net, struct sk_buff,
    pub action): nfnl_abort_action,
    pub genid): *mut *mut *mut bool (valid_genid)(struct net net, u32,
}

extern "C" {
    pub fn nfnetlink_subsys_register(n: *const nfnetlink_subsystem) -> c_int;
}
extern "C" {
    pub fn nfnetlink_subsys_unregister(n: *const nfnetlink_subsystem) -> c_int;
}
extern "C" {
    pub fn nfnetlink_has_listeners(net: *mut net, group: c_uint) -> c_int;
}
extern "C" {
    pub fn nfnetlink_set_err(net: *mut net, portid: u32, group: u32, error: c_int) -> c_int;
}
extern "C" {
    pub fn nfnetlink_unicast(skb: *mut sk_buff, net: *mut net, portid: u32) -> c_int;
}
extern "C" {
    pub fn nfnl_lock(subsys_id: __u8);
}
extern "C" {
    pub fn nfnl_unlock(subsys_id: __u8);
}

extern "C" {
    pub fn lockdep_nfnl_is_held(subsys_id: __u8) -> bool;
}

