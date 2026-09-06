//! Automatically rewritten from C to Rust
//! Source: net/bridge/br_nf_core.c
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
// Handle firewalling core
// Linux ethernet bridge
//
// Authors:
// Lennert Buytenhek		<buytenh@gnu.org>
// Bart De Schuymer		<bdschuym@pandora.be>
//
// Lennert dedicates this file to Kerstin Wurdinger.
//

    static void fake_update_pmtu(struct dst_entry *dst, struct sock *sk,
    struct sk_buff *skb, u32 mtu,
    bool confirm_neigh)
    {
    }
    static void fake_redirect(struct dst_entry *dst, struct sock *sk,
    struct sk_buff *skb)
    {
    }
    static u32 *fake_cow_metrics(struct dst_entry *dst, unsigned long old)
    {
    return core::ptr::null_mut();
    }
    static struct neighbour *fake_neigh_lookup(const struct dst_entry *dst,
    struct sk_buff *skb,
    const void *daddr)
    {
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn fake_mtu(dst: *const dst_entry) -> c_uint {
    static unsigned int fake_mtu(const struct dst_entry *dst)
    {
    return dst.dev.mtu;
    }
    static struct dst_ops fake_dst_ops = {
    .family		= AF_INET,
    .update_pmtu	= fake_update_pmtu,
    .redirect	= fake_redirect,
    .cow_metrics	= fake_cow_metrics,
    .neigh_lookup	= fake_neigh_lookup,
    .mtu		= fake_mtu,
    };
//
// Initialize bogus route table used to keep netfilter happy.
// Currently, we fill in the PMTU entry because netfilter
// refragmentation needs it, and the rt_flags entry because
// ipt_REJECT needs it.  Future netfilter modules might
// require us to fill additional fields.
//
#[no_mangle]
pub unsafe extern "C" fn br_netfilter_rtable_init(br: *mut net_bridge) {
    void br_netfilter_rtable_init(struct net_bridge *br)
    {
    struct rtable *rt = &br.fake_rtable;
    rcuref_init(&rt.dst.__rcuref, 1);
    rt.dst.dev = br.dev;
    dst_init_metrics(&rt.dst, br.metrics, false);
    dst_metric_set(&rt.dst, RTAX_MTU, br.dev.mtu);
    rt.dst.flags	= DST_NOXFRM | DST_FAKE_RTABLE;
    rt.dst.ops = &fake_dst_ops;
    }
#[no_mangle]
pub unsafe extern "C" fn br_nf_core_init() -> int __init {
    int __init br_nf_core_init(void)
    {
    return dst_entries_init(&fake_dst_ops);
    }
#[no_mangle]
pub unsafe extern "C" fn br_nf_core_fini() {
    void br_nf_core_fini(void)
    {
    dst_entries_destroy(&fake_dst_ops);
    }
