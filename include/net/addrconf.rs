//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/addrconf.h
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

// TEMP_VALID_LIFETIME default value as specified in RFC 8981 3.8

pub const IPV6_MAX_ADDRESSES: c_int = 16;

pub const ADDRCONF_NOTIFY_PRIORITY: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prefix_info {
    pub type: __u8,
    pub length: __u8,
    pub prefix_len: __u8,
#[repr(C)]
#[derive(Copy, Clone)]
pub union __packed {
    pub flags: __u8,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __packed {

    pub 4: reserved :,

    pub 1: onlink :,

}

// rfc4861 4.6.2: IPv6 PIO is 32 bytes in size

#[repr(C)]
#[derive(Copy, Clone)]
pub struct in6_validator_info {
    pub i6vi_addr: in6_addr,
    pub i6vi_dev: *mut inet6_dev,
    pub extack: *mut netlink_ext_ack,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ifa6_config {
    pub pfx: *const in6_addr,
    pub plen: c_uint,
    pub ifa_proto: u8,
    pub peer_pfx: *const in6_addr,
    pub rt_priority: u32,
    pub ifa_flags: u32,
    pub preferred_lft: u32,
    pub valid_lft: u32,
    pub scope: u16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum addr_type_t {
    UNICAST_ADDR,
    MULTICAST_ADDR,
    ANYCAST_ADDR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inet6_fill_args {
    pub portid: u32,
    pub seq: u32,
    pub event: c_int,
    pub flags: c_uint,
    pub netnsid: c_int,
    pub ifindex: c_int,
    pub type: addr_type_t,
    pub force_rt_scope_universe: bool,
}

extern "C" {
    pub fn addrconf_init() -> c_int;
}
extern "C" {
    pub fn addrconf_cleanup();
}
extern "C" {
    pub fn addrconf_add_ifaddr(net: *mut net, arg: *mut void __user) -> c_int;
}
extern "C" {
    pub fn addrconf_del_ifaddr(net: *mut net, arg: *mut void __user) -> c_int;
}
extern "C" {
    pub fn addrconf_set_dstaddr(net: *mut net, arg: *mut void __user) -> c_int;
}

extern "C" {
    pub fn ipv6_chk_home_addr(net: *mut net, addr: *const in6_addr) -> c_int;
}

extern "C" {
    pub fn ipv6_chk_prefix(addr: *const in6_addr, dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn inet_rcv_saddr_any(sk: *const sock) -> bool;
}
extern "C" {
    pub fn addrconf_join_solict(dev: *mut net_device, addr: *const in6_addr);
}
extern "C" {
    pub fn addrconf_leave_solict(idev: *mut inet6_dev, addr: *const in6_addr);
}
//
// The zSeries OSA network cards can be shared among various
// OS instances, but the OSA cards have only one MAC address.
// This leads to duplicate address conflicts in conjunction
// with IPv6 if more than one instance uses the same card.
//
// The driver for these cards can deliver a unique 16-bit
// identifier for each instance sharing the same card.  It is
// placed instead of 0xFFFE in the interface identifier.  The
// "u" bit of the interface identifier is not inverted in this
// case.  Hence the resulting interface identifier has local
// scope according to RFC2373.
//
pub const INFINITY_LIFE_TIME: c_uint = 0xFFFFFFFF;
//
// Avoid arithmetic overflow.
// Assuming unit is constant and non-zero, this "if" statement
// will go away on 64bit archs.
//
// IPv6 Address Label subsystem (addrlabel.c)
//
extern "C" {
    pub fn ipv6_addr_label_init() -> c_int;
}
extern "C" {
    pub fn ipv6_addr_label_cleanup();
}
extern "C" {
    pub fn ipv6_addr_label_rtnl_register() -> c_int;
}
//
// multicast prototypes (mcast.c)
//
extern "C" {
    pub fn pskb_may_pull(_arg: skb, _arg: len) -> return;
}
extern "C" {
    pub fn __ipv6_sock_mc_close(sk: *mut sock);
}
extern "C" {
    pub fn ipv6_sock_mc_close(sk: *mut sock);
}
extern "C" {
    pub fn ipv6_dev_mc_inc(dev: *mut net_device, addr: *const in6_addr) -> c_int;
}
extern "C" {
    pub fn __ipv6_dev_mc_dec(idev: *mut inet6_dev, addr: *const in6_addr) -> c_int;
}
extern "C" {
    pub fn ipv6_dev_mc_dec(dev: *mut net_device, addr: *const in6_addr) -> c_int;
}
extern "C" {
    pub fn ipv6_mc_up(idev: *mut inet6_dev);
}
extern "C" {
    pub fn ipv6_mc_down(idev: *mut inet6_dev);
}
extern "C" {
    pub fn ipv6_mc_unmap(idev: *mut inet6_dev);
}
extern "C" {
    pub fn ipv6_mc_remap(idev: *mut inet6_dev);
}
extern "C" {
    pub fn ipv6_mc_init_dev(idev: *mut inet6_dev);
}
extern "C" {
    pub fn ipv6_mc_destroy_dev(idev: *mut inet6_dev);
}
extern "C" {
    pub fn ipv6_mc_check_mld(skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn addrconf_dad_failure(skb: *mut sk_buff, ifp: *mut inet6_ifaddr);
}
extern "C" {
    pub fn ipv6_mc_dad_complete(idev: *mut inet6_dev);
}
//
// identify MLD packets for MLD filter exceptions
//
// anycast prototypes (anycast.c)
//
extern "C" {
    pub fn __ipv6_sock_ac_close(sk: *mut sock);
}
extern "C" {
    pub fn ipv6_sock_ac_close(sk: *mut sock);
}
extern "C" {
    pub fn __ipv6_dev_ac_inc(idev: *mut inet6_dev, addr: *const in6_addr) -> c_int;
}
extern "C" {
    pub fn __ipv6_dev_ac_dec(idev: *mut inet6_dev, addr: *const in6_addr) -> c_int;
}
extern "C" {
    pub fn ipv6_ac_destroy_dev(idev: *mut inet6_dev);
}
extern "C" {
    pub fn ipv6_anycast_init() -> c_int;
}
extern "C" {
    pub fn ipv6_anycast_cleanup();
}
// Device notifier
extern "C" {
    pub fn register_inet6addr_notifier(nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn unregister_inet6addr_notifier(nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn inet6addr_notifier_call_chain(val: c_ulong, v: *mut c_void) -> c_int;
}
extern "C" {
    pub fn register_inet6addr_validator_notifier(nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn unregister_inet6addr_validator_notifier(nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn inet6addr_validator_notifier_call_chain(val: c_ulong, v: *mut c_void) -> c_int;
}
//
// __in6_dev_get - get inet6_dev pointer from netdevice
// @dev: network device
//
// Caller must hold rcu_read_lock or RTNL, because this function
// does not take a reference on the inet6_dev.
//
extern "C" {
    pub fn rcu_dereference_rtnl(_arg: dev->ip6_ptr) -> return;
}
extern "C" {
    pub fn rcu_dereference(_arg: dev->ip6_ptr) -> return;
}
extern "C" {
    pub fn rtnl_net_dereference(_arg: dev_net(dev), _arg: dev->ip6_ptr) -> return;
}
//
// __in6_dev_stats_get - get inet6_dev pointer for stats
// @dev: network device
// @skb: skb for original incoming interface if needed
//
// Caller must hold rcu_read_lock or RTNL, because this function
// does not take a reference on the inet6_dev.
//
extern "C" {
    pub fn __in6_dev_get(_arg: dev) -> return;
}
//
// __in6_dev_get_safely - get inet6_dev pointer from netdevice
// @dev: network device
//
// This is a safer version of __in6_dev_get
//
extern "C" {
    pub fn rcu_dereference_rtnl(_arg: dev->ip6_ptr) -> return;
}
//
// in6_dev_get - get inet6_dev pointer from netdevice
// @dev: network device
//
// This version can be used in any context, and takes a reference
// on the inet6_dev. Callers must use in6_dev_put() later to
// release this reference.
//
extern "C" {
    pub fn in6_dev_finish_destroy(idev: *mut inet6_dev);
}
// pidev = NULL;
extern "C" {
    pub fn refcount_inc_not_zero(_arg: &idev->refcnt) -> return;
}
// called with rcu_read_lock held
extern "C" {
    pub fn inet6_ifa_finish_destroy(ifp: *mut inet6_ifaddr);
}
extern "C" {
    pub fn refcount_inc_not_zero(_arg: &ifp->refcnt) -> return;
}
//
// compute link-local solicited-node multicast address
//

extern "C" {
    pub fn if6_proc_init() -> c_int;
}
extern "C" {
    pub fn if6_proc_exit();
}

