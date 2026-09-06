//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/inetdevice.h
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
pub struct ipv4_devconf {
    pub sysctl: *mut c_void,
    pub data: [c_int; IPV4_DEVCONF_MAX],
    pub IPV4_DEVCONF_MAX): DECLARE_BITMAP(state,,
}

pub const MC_HASH_SZ_LOG: c_int = 9;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct in_device {
    pub dev: *mut net_device,
    pub dev_tracker: netdevice_tracker,
    pub refcnt: refcount_t,
    pub dead: c_int,
    pub /: *mut *mut *mut in_ifaddr __rcu ifa_list;/ IP ifaddr chain,
    pub /: *mut *mut *mut ip_mc_list __rcu mc_list; / IP multicast filter chain,
    pub mc_hash: *mut *mut ip_mc_list __rcu  __rcu,
    pub /: *mut *mut int mc_count; / Number of installed mcasts,
    pub mc_tomb_lock: spinlock_t,
    pub mc_tomb: *mut ip_mc_list,
    pub mr_v1_seen: c_ulong,
    pub mr_v2_seen: c_ulong,
    pub /: *mut *mut unsigned long mr_qi; / Query Interval,
    pub /: *mut *mut unsigned long mr_qri; / Query Response Interval,
    pub /: *mut *mut unsigned char mr_qrv; / Query Robustness Variable,
    pub mr_gq_running: c_uchar,
    pub mr_maxdelay: u32,
    pub mr_ifc_count: u32,
    pub /: *mut *mut timer_list mr_gq_timer; / general query timer,
    pub /: *mut *mut timer_list mr_ifc_timer; / interface change timer,
    pub arp_parms: *mut neigh_parms,
    pub cnf: ipv4_devconf,
    pub rcu_head: rcu_head,
}

extern "C" {
    pub fn READ_ONCE(_arg: in_dev->cnf.data[index]) -> return;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct in_ifaddr {
    pub addr_lst: hlist_node,
    pub ifa_next: *mut in_ifaddr __rcu,
    pub ifa_dev: *mut in_device,
    pub rcu_head: rcu_head,
    pub ifa_local: __be32,
    pub ifa_address: __be32,
    pub ifa_mask: __be32,
    pub ifa_rt_priority: __u32,
    pub ifa_broadcast: __be32,
    pub ifa_scope: c_uchar,
    pub ifa_prefixlen: c_uchar,
    pub ifa_proto: c_uchar,
    pub ifa_flags: __u32,
    pub ifa_label: [c_char; IFNAMSIZ],
// In seconds, relative to tstamp. Expiry is at tstamp + HZ * lft.
    pub ifa_valid_lft: __u32,
    pub ifa_preferred_lft: __u32,
    pub /: *mut *mut unsigned long ifa_cstamp; / created timestamp,
    pub /: *mut *mut unsigned long ifa_tstamp; / updated timestamp,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct in_validator_info {
    pub ivi_addr: __be32,
    pub ivi_dev: *mut in_device,
    pub extack: *mut netlink_ext_ack,
}

extern "C" {
    pub fn register_inetaddr_notifier(nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn unregister_inetaddr_notifier(nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn register_inetaddr_validator_notifier(nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn unregister_inetaddr_validator_notifier(nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn __ip_dev_find(_arg: net, _arg: addr, _arg: true) -> return;
}
extern "C" {
    pub fn inet_addr_onlink(in_dev: *mut in_device, a: __be32, b: __be32) -> c_int;
}
extern "C" {
    pub fn devinet_ioctl(net: *mut net, cmd: c_uint, : *mut ifreq) -> c_int;
}

extern "C" {
    pub fn inet_gifconf(dev: *mut net_device, buf: *mut char __user, len: c_int, size: c_int) -> c_int;
}

extern "C" {
    pub fn devinet_init();
}
extern "C" {
    pub fn inet_select_addr(dev: *const net_device, dst: __be32, scope: c_int) -> __be32;
}
//
// Check if a mask is acceptable.
//

extern "C" {
    pub fn rcu_dereference(_arg: dev->ip_ptr) -> return;
}
extern "C" {
    pub fn rtnl_dereference(_arg: dev->ip_ptr) -> return;
}
extern "C" {
    pub fn rtnl_net_dereference(_arg: dev_net(dev), _arg: dev->ip_ptr) -> return;
}
// called with rcu_read_lock or rtnl held
extern "C" {
    pub fn in_dev_finish_destroy(idev: *mut in_device);
}

extern "C" {
    pub fn refcount_inc_not_zero(_arg: &idev->refcnt) -> return;
}

extern "C" {
    pub fn htonl(_arg: ~((1U<<(32-logmask))-1)) -> return;
}
