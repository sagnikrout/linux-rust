//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/lwtunnel.h
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
pub const __NET_LWTUNNEL_H: c_int = 1;

pub const LWTUNNEL_HASH_BITS: c_int = 7;

// lw tunnel state flags

// LWTUNNEL_XMIT_CONTINUE should be distinguishable from dst_output return
// values (NET_XMIT_xxx and NETDEV_TX_xxx in linux/netdevice.h) for safety.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lwtunnel_state {
    pub type: __u16,
    pub flags: __u16,
    pub headroom: __u16,
    pub refcnt: core::sync::atomic::AtomicI32,
    pub skb): *mut *mut *mut *mut int (orig_output)(struct net net, struct sock sk, struct sk_buff,
    pub ): *mut *mut int (orig_input)(struct sk_buff,
    pub rcu: rcu_head,
    pub data: [__u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lwtunnel_encap_ops {
    pub extack): *mut netlink_ext_ack,
    pub lws): *mut *mut void (destroy_state)(struct lwtunnel_state,
    pub skb): *mut *mut *mut *mut int (output)(struct net net, struct sock sk, struct sk_buff,
    pub skb): *mut *mut int (input)(struct sk_buff,
    pub lwtstate): *mut lwtunnel_state,
    pub lwtstate): *mut *mut int (get_encap_size)(struct lwtunnel_state,
    pub b): *mut *mut *mut int (cmp_encap)(struct lwtunnel_state a, struct lwtunnel_state,
    pub skb): *mut *mut int (xmit)(struct sk_buff,
    pub owner: *mut module,
}

extern "C" {
    pub fn lwtstate_free(lws: *mut lwtunnel_state);
}
extern "C" {
    pub fn lwtunnel_get_encap_size(lwtstate: *mut lwtunnel_state) -> c_int;
}
extern "C" {
    pub fn lwtunnel_cmp_encap(a: *mut lwtunnel_state, b: *mut lwtunnel_state) -> c_int;
}
extern "C" {
    pub fn lwtunnel_output(net: *mut net, sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn lwtunnel_input(skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn lwtunnel_xmit(skb: *mut sk_buff) -> c_int;
}

// return 0 since we are not walking attr looking for
// RTA_ENCAP_TYPE attribute on nexthops.
//

