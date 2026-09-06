//! Automatically rewritten from C to Rust
//! Source: net/netfilter/ipvs/ip_vs_fo.c
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
// IPVS:        Weighted Fail Over module
//
// Authors:     Kenny Mathis <kmathis@chokepoint.net>
//
// Changes:
// Kenny Mathis            :     added initial functionality based on weight
//

// Weighted Fail Over Module
    static struct ip_vs_dest *
    ip_vs_fo_schedule(struct ip_vs_service *svc, const struct sk_buff *skb,
    struct ip_vs_iphdr *iph)
    {
    struct ip_vs_dest *dest, *hweight = core::ptr::null_mut();
    int hw = 0; /* Track highest weight */
    IP_VS_DBG(6, "ip_vs_fo_schedule(): Scheduling...\n");
// Basic failover functionality
// Find virtual server with highest weight and send it traffic
//
    list_for_each_entry_rcu(dest, &svc.destinations, n_list) {
    if (!(dest.flags & IP_VS_DEST_F_OVERLOAD) &&
    atomic_read(&dest.weight) > hw) {
    hweight = dest;
    hw = atomic_read(&dest.weight);
    }
    }
    if (hweight) {
    IP_VS_DBG_BUF(6, "FO: server %s:%u activeconns %d weight %d\n",
    IP_VS_DBG_ADDR(hweight.af, &hweight.addr),
    ntohs(hweight.port),
    atomic_read(&hweight.activeconns),
    atomic_read(&hweight.weight));
    return hweight;
    }
    ip_vs_scheduler_err(svc, "no destination available");
    return core::ptr::null_mut();
    }
    static struct ip_vs_scheduler ip_vs_fo_scheduler = {
    .name =			"fo",
    .refcnt =		ATOMIC_INIT(0),
    .module =		THIS_MODULE,
    .n_list =		LIST_HEAD_INIT(ip_vs_fo_scheduler.n_list),
    .schedule =		ip_vs_fo_schedule,
    };
#[no_mangle]
unsafe extern "C" fn ip_vs_fo_init() -> int __init {
    static int __init ip_vs_fo_init(void)
    {
    return register_ip_vs_scheduler(&ip_vs_fo_scheduler);
    }
#[no_mangle]
unsafe extern "C" fn ip_vs_fo_cleanup() -> void __exit {
    static void __exit ip_vs_fo_cleanup(void)
    {
    unregister_ip_vs_scheduler(&ip_vs_fo_scheduler);
    synchronize_rcu();
    }
    module_init(ip_vs_fo_init);
    module_exit(ip_vs_fo_cleanup);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("ipvs weighted failover scheduler");
