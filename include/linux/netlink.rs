//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/netlink.h
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

extern "C" {
    pub fn do_trace_netlink_extack(msg: *const c_char);
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum netlink_skb_flags {
    NETLINK_SKB_DST		= 0x8,	/* Dst set in sendto or sendmsg */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netlink_skb_parms {
    pub /: *mut *mut scm_creds creds; / Skb credentials,
    pub portid: __u32,
    pub dst_group: __u32,
    pub flags: __u32,
    pub sk: *mut sock,
    pub nsid_is_set: bool,
    pub nsid: c_int,
}

pub const NETLINK_CTX_SIZE: c_int = 48;
extern "C" {
    pub fn netlink_table_grab();
}
extern "C" {
    pub fn netlink_table_ungrab();
}

// optional Netlink kernel configuration parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netlink_kernel_cfg {
    pub groups: c_uint,
    pub flags: c_uint,
    pub skb): *mut *mut void (input)(struct sk_buff,
    pub group): *mut *mut *mut int (bind)(struct net net, int,
    pub group): *mut *mut *mut void (unbind)(struct net net, int,
    pub groups): *mut *mut *mut void (release) (struct sock sk, unsigned long,
}

extern "C" {
    pub fn __netlink_kernel_create(_arg: net, _arg: unit, _arg: THIS_MODULE, _arg: cfg) -> return;
}
// this can be increased when necessary - don't expose to userland
pub const NETLINK_MAX_COOKIE_LEN: c_int = 8;
pub const NETLINK_MAX_FMTMSG_LEN: c_int = 80;
//
// struct netlink_ext_ack - netlink extended ACK report struct
// @_msg: message string to report - don't access directly, use
// %NL_SET_ERR_MSG
// @bad_attr: attribute with error
// @policy: policy for a bad attribute
// @miss_type: attribute type which was missing
// @miss_nest: nest missing an attribute (%NULL if missing top level attr)
// @cookie: cookie data to return to userspace (for success)
// @cookie_len: actual cookie data length
// @_msg_buf: output buffer for formatted message strings - don't access
// directly, use %NL_SET_ERR_MSG_FMT
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netlink_ext_ack {
    pub _msg: *const c_char,
    pub bad_attr: *const nlattr,
    pub policy: *const nla_policy,
    pub miss_nest: *const nlattr,
    pub miss_type: u16,
    pub cookie: [u8; NETLINK_MAX_COOKIE_LEN],
    pub cookie_len: u8,
    pub _msg_buf: [c_char; NETLINK_MAX_FMTMSG_LEN],
}

// Always use this macro, this allows later putting the
// message into a separate section or such for things
// like translation or listing all possible messages.
// If string formatting is needed use NL_SET_ERR_MSG_FMT.
//

// We splice fmt with %s at each end even in the snprintf so that both calls
// can use the same string constant, avoiding its duplication in .ro
//

extern "C" {
    pub fn netlink_kernel_release(sk: *mut sock);
}
extern "C" {
    pub fn __netlink_change_ngroups(sk: *mut sock, groups: c_uint) -> c_int;
}
extern "C" {
    pub fn netlink_change_ngroups(sk: *mut sock, groups: c_uint) -> c_int;
}
extern "C" {
    pub fn __netlink_clear_multicast_users(sk: *mut sock, group: c_uint);
}
extern "C" {
    pub fn netlink_has_listeners(sk: *mut sock, group: c_uint) -> c_int;
}
extern "C" {
    pub fn netlink_strict_get_check(skb: *mut sk_buff) -> bool;
}
extern "C" {
    pub fn netlink_unicast(ssk: *mut sock, skb: *mut sk_buff, portid: __u32, nonblock: c_int) -> c_int;
}
extern "C" {
    pub fn int(dsk: *mut *mut netlink_filter_fn)(struct sock, skb: *mut sk_buff, data: *mut c_void) -> typedef;
}
extern "C" {
    pub fn netlink_set_err(ssk: *mut sock, portid: __u32, group: __u32, code: c_int) -> c_int;
}
extern "C" {
    pub fn netlink_register_notifier(nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn netlink_unregister_notifier(nb: *mut notifier_block) -> c_int;
}
// finegrained unicast helpers:
extern "C" {
    pub fn netlink_detachskb(sk: *mut sock, skb: *mut sk_buff);
}
extern "C" {
    pub fn netlink_sendskb(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
// This is a large skb, set destructor callback to release head
//
// skb should fit one page. This choice is good for headerless malloc.
// But we should limit to 8K so that userspace does not have to
// use enormous buffer sizes on recvmsg() calls just to avoid
// MSG_TRUNC when PAGE_SIZE is very large.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netlink_callback {
    pub skb: *mut sk_buff,
    pub nlh: *const nlmsghdr,
    pub cb): *mut netlink_callback,
    pub cb): *mut *mut int (done)(struct netlink_callback,
    pub data: *mut c_void,
// the module that dump function belong to
    pub module: *mut module,
    pub extack: *mut netlink_ext_ack,
    pub family: u16,
    pub answer_flags: u16,
    pub min_dump_alloc: u32,
    pub seq: unsigned int prev_seq,,
    pub flags: c_int,
    pub strict_check: bool,
    pub ctx: [u8; NETLINK_CTX_SIZE],
// args is deprecated. Cast a struct over ctx instead
// for proper type safety.
//
    pub args: [c_long; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netlink_notify {
    pub net: *mut net,
    pub portid: u32,
    pub protocol: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netlink_dump_control {
    pub ): *mut *mut int (start)(struct netlink_callback,
    pub ): *mut *mut *mut int (dump)(struct sk_buff skb, struct netlink_callback,
    pub ): *mut *mut int (done)(struct netlink_callback,
    pub extack: *mut netlink_ext_ack,
    pub data: *mut c_void,
    pub module: *mut module,
    pub min_dump_alloc: u32,
    pub flags: c_int,
}

extern "C" {
    pub fn __netlink_dump_start(_arg: ssk, _arg: skb, _arg: nlh, _arg: control) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netlink_tap {
    pub dev: *mut net_device,
    pub module: *mut module,
    pub list: list_head,
}

extern "C" {
    pub fn netlink_add_tap(nt: *mut netlink_tap) -> c_int;
}
extern "C" {
    pub fn netlink_remove_tap(nt: *mut netlink_tap) -> c_int;
}
extern "C" {
    pub fn netlink_capable(skb: *const sk_buff, cap: c_int) -> bool;
}
extern "C" {
    pub fn netlink_net_capable(skb: *const sk_buff, cap: c_int) -> bool;
}
