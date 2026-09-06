//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/fib_rules.h
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
pub struct fib_kuid_range {
    pub start: kuid_t,
    pub end: kuid_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fib_rule {
    pub list: list_head,
    pub iifindex: c_int,
    pub oifindex: c_int,
    pub mark: u32,
    pub mark_mask: u32,
    pub flags: u32,
    pub table: u32,
    pub action: u8,
    pub l3mdev: u8,
    pub proto: u8,
    pub ip_proto: u8,
    pub target: u32,
    pub tun_id: __be64,
    pub ctarget: *mut fib_rule __rcu,
    pub fr_net: *mut net,
    pub refcnt: refcount_t,
    pub pref: u32,
    pub suppress_ifgroup: c_int,
    pub suppress_prefixlen: c_int,
    pub iifname: [c_char; IFNAMSIZ],
    pub oifname: [c_char; IFNAMSIZ],
    pub uid_range: fib_kuid_range,
    pub sport_range: fib_rule_port_range,
    pub dport_range: fib_rule_port_range,
    pub sport_mask: u16,
    pub dport_mask: u16,
    pub iif_is_l3_master: u8,
    pub oif_is_l3_master: u8,
    pub rcu: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fib_lookup_arg {
    pub lookup_ptr: *mut c_void,
    pub lookup_data: *const c_void,
    pub result: *mut c_void,
    pub rule: *mut fib_rule,
    pub table: u32,
    pub flags: c_int,
pub const FIB_LOOKUP_NOREF: c_int = 1;
pub const FIB_LOOKUP_IGNORE_LINKSTATE: c_int = 2;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fib_rules_ops {
    pub family: c_int,
    pub list: list_head,
    pub rule_size: c_int,
    pub addr_size: c_int,
    pub unresolved_rules: c_int,
    pub nr_goto_rules: c_int,
    pub fib_rules_seq: c_uint,
    pub ): *mut fib_lookup_arg,
    pub ): *mut fib_lookup_arg,
    pub int): *mut *mut flowi ,,
    pub ): *mut netlink_ext_ack,
    pub ): *mut *mut void (delete)(struct fib_rule,
    pub ): *mut nlattr,
    pub ): *mut fib_rule_hdr,
    pub ): *mut *mut size_t (nlmsg_payload)(struct fib_rule,
// Called after modifications to the rules set, must flush
// the route cache if one exists.
    pub ops): *mut *mut void (flush_cache)(struct fib_rules_ops,
    pub net): *mut *mut bool (need_rtnl)(struct net,
    pub nlgroup: c_int,
    pub rules_list: list_head,
    pub owner: *mut module,
    pub fro_net: *mut net,
    pub lock: mutex,
    pub rcu: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fib_rule_notifier_info {
    pub /: *mut *mut fib_notifier_info info; / must be first,
    pub rule: *mut fib_rule,
}

extern "C" {
    pub fn refcount_inc_not_zero(_arg: &rule->refcnt) -> return;
}

extern "C" {
    pub fn nla_get_u32(_arg: nla[FRA_TABLE]) -> return;
}
extern "C" {
    pub fn fib_rules_unregister(: *mut fib_rules_ops);
}
extern "C" {
    pub fn fib_default_rule_add(: *mut fib_rules_ops, pref: u32, table: u32) -> c_int;
}
extern "C" {
    pub fn fib_rule_matchall(rule: *const fib_rule) -> bool;
}
extern "C" {
    pub fn fib_rules_seq_read(net: *const net, family: c_int) -> c_uint;
}
