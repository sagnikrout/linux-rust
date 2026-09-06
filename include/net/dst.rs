//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/dst.h
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
// net/dst.h	Protocol independent destination cache definitions.
//
// Authors:	Alexey Kuznetsov, <kuznet@ms2.inr.ac.ru>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dst_entry {
    pub dev: *mut net_device,
    pub dev_rcu: *mut net_device __rcu,
}

pub const DST_NOXFRM: c_uint = 0x0002;
pub const DST_NOPOLICY: c_uint = 0x0004;
pub const DST_NOCOUNT: c_uint = 0x0008;
pub const DST_FAKE_RTABLE: c_uint = 0x0010;
pub const DST_XFRM_TUNNEL: c_uint = 0x0020;
pub const DST_XFRM_QUEUE: c_uint = 0x0040;
pub const DST_METADATA: c_uint = 0x0080;
// A non-zero value of dst->obsolete forces by-hand validation
// of the route entry.  Positive values are set by the generic
// dst layer to indicate that the entry has been forcefully
// destroyed.
//
// Negative values are used by the implementation layer code to
// force invocation of the dst_ops->check() method.
//
pub const DST_OBSOLETE_NONE: c_int = 0;
pub const DST_OBSOLETE_DEAD: c_int = 2;

//
// __rcuref wants to be on a different cache line from
// input/output/ops or performance tanks badly
//

//
// Used by rtable and rt6_info. Moves lwtstate into the next cache
// line on 64bit so that lwtstate does not cause false sharing with
// __rcuref under contention of __rcuref. This also puts the
// frequently accessed members of rtable and rt6_info out of the
// __rcuref cache line.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dst_metrics {
    pub metrics: [u32; RTAX_MAX],
    pub refcnt: refcount_t,
    pub /: *mut *mut } __aligned(4); / Low pointer bits contain DST_METRICS_FLAGS,
    pub dst_default_metrics: extern struct dst_metrics,
    pub old): *mut *mut *mut u32 dst_cow_metrics_generic(struct dst_entry dst, unsigned long,
pub const DST_METRICS_READ_ONLY: c_uint = 0x1UL;
pub const DST_METRICS_REFCOUNTED: c_uint = 0x2UL;
pub const DST_METRICS_FLAGS: c_uint = 0x3UL;

    pub DST_METRICS_READ_ONLY: return dst->_metrics &,
    pub old): *mut *mut void __dst_destroy_metrics_generic(struct dst_entry dst, unsigned long,
    pub dst->_metrics: unsigned long val =,
    pub val): __dst_destroy_metrics_generic(dst,,
    pub dst->_metrics: unsigned long p =,
    pub p): return dst->ops->cow_metrics(dst,,
    pub __DST_METRICS_PTR(p): return,
// This may only be invoked before the entry has reached global
// visibility.
//
    pub 0): (read_only ? DST_METRICS_READ_ONLY :,
    pub dst_metrics_write_ptr(dest): *mut *mut u32 dst_metrics =,
    pub DST_METRICS_PTR(src): *mut *mut u32 src_metrics =,
    pub sizeof(u32)): *mut *mut memcpy(dst_metrics, src_metrics, RTAX_MAX,
    pub DST_METRICS_PTR(dst): return,
    pub DST_METRICS_PTR(dst): *mut *mut u32 p =,
    pub p: [return; metric-1],
    pub RTAX_MTU): metric ==,
    pub metric): return dst_metric_raw(dst,,
    pub RTAX_ADVMSS): u32 advmss = dst_metric_raw(dst,,
    pub dst->ops->default_advmss(dst): advmss =,
    pub advmss: return,
    pub dst_metrics_write_ptr(dst): *mut *mut u32 p =,
    pub val: p[metric-1] =,
// Kernel-internal feature bits that are unallocated in user space.

    pub feature: return dst_metric(dst, RTAX_FEATURES) &,
    pub )): *const INDIRECT_CALLABLE_DECLARE(unsigned int ip6_mtu(struct dst_entry,
    pub )): *const INDIRECT_CALLABLE_DECLARE(unsigned int ipv4_mtu(struct dst_entry,
    pub dst): return INDIRECT_CALL_INET(dst->ops->mtu, ip6_mtu, ipv4_mtu,,
// Variant of dst_mtu() for IPv4 users.
    pub dst): return INDIRECT_CALL_1(dst->ops->mtu, ipv4_mtu,,
// RTT metrics are stored in milliseconds for user ABI, but used as jiffies
    pub metric)): return msecs_to_jiffies(dst_metric(dst,,
    pub metric): return dst_metric(dst, RTAX_LOCK) & (1 <<,
//
// If your kernel compilation stops here, please check
// the placement of __rcuref in struct dst_entry
//
    pub 63): BUILD_BUG_ON(offsetof(struct dst_entry, __rcuref) &,
    pub time): WRITE_ONCE(dst->lastuse,,
    pub dst: return,
    pub dst): *mut void dst_release(struct dst_entry,
    pub dst): *mut void dst_release_immediate(struct dst_entry,
    pub SKB_DST_PTRMASK)): *mut *mut dst_release((struct dst_entry )(refdst &,
//
// skb_dst_drop - drops skb dst
// @skb: buffer
//
// Drops dst reference count if a reference was taken.
//
    pub 0UL: skb->_skb_refdst =,
    pub !!refdst: nskb->slow_gro |=,
    pub refdst: nskb->_skb_refdst =,
    pub oskb->_skb_refdst): __skb_dst_copy(nskb,,
//
// dst_hold_safe - Take a reference on a dst if possible
// @dst: pointer to dst entry
//
// This helper returns false if it could not safely
// take a reference on a dst.
//
    pub rcuref_get(&dst->__rcuref): return,
//
// skb_dst_force - makes sure skb dst is refcounted
// @skb: buffer
//
// If dst is not yet refcounted and not destroyed, grab a ref on it.
// Returns: true if dst is refcounted.
//
    pub skb_dst(skb): *mut *mut dst_entry dst =,
    pub NULL: dst =,
    pub long)dst: skb->_skb_refdst = (unsigned,
    pub !!dst: skb->slow_gro |=,
    pub 0UL: return skb->_skb_refdst !=,
//
// __skb_tunnel_rx - prepare skb for rx reinsert
// @skb: buffer
// @dev: tunnel device
// @net: netns for packet i/o
//
// After decapsulation, packet is going to re-enter (netif_rx()) our stack,
// so make some cleanups. (no accounting done)
//
    pub dev: skb->dev =,
//
// Clear hash so that we can recalculate the hash for the
// encapsulated packet, unless we have already determine the hash
// over the L4 4-tuple.
//
    pub 0): skb_set_queue_mapping(skb,,
    pub dev_net(dev))): skb_scrub_packet(skb, !net_eq(net,,
//
// skb_tunnel_rx - prepare skb for rx reinsert
// @skb: buffer
// @dev: tunnel device
// @net: netns for packet i/o
//
// After decapsulation, packet is going to re-enter (netif_rx()) our stack,
// so make some cleanups, and perform accounting.
// Note: this accounting is not SMP safe.
//
    pub rx_packets): DEV_STATS_INC(dev,,
    pub skb->len): DEV_STATS_ADD(dev, rx_bytes,,
    pub net): __skb_tunnel_rx(skb, dev,,

    pub dst: *const dst_entry,
    pub skb_dst(skb): dst =,
    pub dst->tclassid: return,

    pub 0: return,
    pub skb): *mut *mut *mut int dst_discard_out(struct net net, struct sock sk, struct sk_buff,
    pub skb): return dst_discard_out(&init_net, skb->sk,,
    pub flags): int initial_obsolete, unsigned short,
    pub flags): c_ushort,
    pub dst): *mut void dst_dev_put(struct dst_entry,
    pub daddr): *mut *mut neighbour n = dst->ops->neigh_lookup(dst, NULL,,
    pub n: return IS_ERR(n) ? NULL :,
    pub n: *mut neighbour,
    pub NULL: return,
    pub NULL): n = dst->ops->neigh_lookup(dst, skb,,
    pub n: return IS_ERR(n) ? NULL :,
    pub daddr): dst->ops->confirm_neigh(dst,,
    pub skb_dst(skb): *mut *mut dst_entry dst =,
    pub timeout: unsigned long old, expires = jiffies +,
    pub 1: expires =,
    pub READ_ONCE(dst->expires): old =,
    pub expires): WRITE_ONCE(dst->expires,,
    pub LL_RESERVED_SPACE(dst->dev): return,
    pub skb->mac_len: return,
    pub )): *mut sk_buff,
    pub )): *mut sk_buff,
// Output packet to network from transport.
    pub skb): net, sk,,
    pub )): *mut INDIRECT_CALLABLE_DECLARE(int ip6_input(struct sk_buff,
    pub )): *mut INDIRECT_CALLABLE_DECLARE(int ip_local_deliver(struct sk_buff,
// Input packet from network to transport.
    pub skb): ip6_input, ip_local_deliver,,
    pub cookie): ipv4_dst_check, dst,,
    pub dst: return,
// Flags for xfrm_lookup flags argument.
}

// skb attached with this dst needs transformation if dst->xfrm is valid

// update dst pmtu but not do neighbor confirm
extern "C" {
    pub fn READ_ONCE(_arg: dst->dev) -> return;
}
extern "C" {
    pub fn rcu_dereference(_arg: dst->dev_rcu) -> return;
}
extern "C" {
    pub fn dev_net_rcu(_arg: dst_dev_rcu(dst)) -> return;
}
extern "C" {
    pub fn dst_dev(_arg: skb_dst(skb)) -> return;
}
extern "C" {
    pub fn dst_dev_rcu(_arg: skb_dst(skb)) -> return;
}
extern "C" {
    pub fn dev_net(_arg: skb_dst_dev(skb)) -> return;
}
extern "C" {
    pub fn dev_net_rcu(_arg: skb_dst_dev_rcu(skb)) -> return;
}
extern "C" {
    pub fn dst_blackhole_mtu(dst: *const dst_entry) -> c_uint;
}
