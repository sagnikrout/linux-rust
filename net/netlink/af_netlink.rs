//! Automatically rewritten from C Header to Rust Module
//! Source: net/netlink/af_netlink.h
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

// flags

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netlink_sock {
// struct sock has to be the first member of netlink_sock
    pub sk: sock,
    pub flags: c_ulong,
    pub portid: u32,
    pub dst_portid: u32,
    pub dst_group: u32,
    pub subscriptions: u32,
    pub ngroups: u32,
    pub groups: *mut c_ulong,
    pub state: c_ulong,
    pub max_recvmsg_len: usize,
    pub wait: wait_queue_head_t,
    pub bound: bool,
    pub cb_running: bool,
    pub dump_done_errno: c_int,
    pub cb: netlink_callback,
    pub nl_cb_mutex: mutex,
    pub skb): *mut *mut void (netlink_rcv)(struct sk_buff,
    pub group): *mut *mut *mut int (netlink_bind)(struct net net, int,
    pub group): *mut *mut *mut void (netlink_unbind)(struct net net, int,
    pub groups): *mut c_ulong,
    pub module: *mut module,
    pub node: rhash_head,
    pub rcu: rcu_head,
}

extern "C" {
    pub fn container_of(_arg: sk, netlink_sock: struct, _arg: sk) -> return;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netlink_table {
    pub hash: rhashtable,
    pub mc_list: hlist_head,
    pub listeners: *mut listeners __rcu,
    pub flags: c_uint,
    pub groups: c_uint,
    pub cb_mutex: *mut mutex,
    pub module: *mut module,
    pub group): *mut *mut *mut int (bind)(struct net net, int,
    pub group): *mut *mut *mut void (unbind)(struct net net, int,
    pub groups): *mut c_ulong,
    pub registered: c_int,
}
