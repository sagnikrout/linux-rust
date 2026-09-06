//! Automatically rewritten from C to Rust
//! Source: net/netfilter/ipvs/ip_vs_wlc.c
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
// IPVS:        Weighted Least-Connection Scheduling module
//
// Authors:     Wensong Zhang <wensong@linuxvirtualserver.org>
// Peter Kese <peter.kese@ijs.si>
//
// Changes:
// Wensong Zhang            :     changed the ip_vs_wlc_schedule to return dest
// Wensong Zhang            :     changed to use the inactconns in scheduling
// Wensong Zhang            :     changed some comestics things for debugging
// Wensong Zhang            :     changed for the d-linked destination list
// Wensong Zhang            :     added the ip_vs_wlc_update_svc
// Wensong Zhang            :     added any dest with weight=0 is quiesced
//

//
// Weighted Least Connection scheduling
//
    static struct ip_vs_dest *
    ip_vs_wlc_schedule(struct ip_vs_service *svc, const struct sk_buff *skb,
    struct ip_vs_iphdr *iph)
    {
    struct ip_vs_dest *dest, *least;
    int loh, doh;
    IP_VS_DBG(6, "ip_vs_wlc_schedule(): Scheduling...\n");
//
// We calculate the load of each dest server as follows:
// (dest overhead) / dest->weight
//
// Remember -- no floats in kernel mode!!!
// The comparison of h1*w2 > h2*w1 is equivalent to that of
// h1/w1 > h2/w2
// if every weight is larger than zero.
//
// The server with weight=0 is quiesced and will not receive any
// new connections.
//
    list_for_each_entry_rcu(dest, &svc.destinations, n_list) {
    if (!(dest.flags & IP_VS_DEST_F_OVERLOAD) &&
    atomic_read(&dest.weight) > 0) {
    least = dest;
    loh = ip_vs_dest_conn_overhead(least);
    goto nextstage;
    }
    }
    ip_vs_scheduler_err(svc, "no destination available");
    return core::ptr::null_mut();
//
// Find the destination with the least load.
//
    nextstage:
    list_for_each_entry_continue_rcu(dest, &svc.destinations, n_list) {
    if (dest.flags & IP_VS_DEST_F_OVERLOAD)
    continue;
    doh = ip_vs_dest_conn_overhead(dest);
    if ((__s64)loh * atomic_read(&dest.weight) >
    (__s64)doh * atomic_read(&least.weight)) {
    least = dest;
    loh = doh;
    }
    }
    IP_VS_DBG_BUF(6, "WLC: server %s:%u "
    "activeconns %d refcnt %d weight %d overhead %d\n",
    IP_VS_DBG_ADDR(least.af, &least.addr),
    ntohs(least.port),
    atomic_read(&least.activeconns),
    refcount_read(&least.refcnt),
    atomic_read(&least.weight), loh);
    return least;
    }
    static struct ip_vs_scheduler ip_vs_wlc_scheduler =
    {
    .name =			"wlc",
    .refcnt =		ATOMIC_INIT(0),
    .module =		THIS_MODULE,
    .n_list =		LIST_HEAD_INIT(ip_vs_wlc_scheduler.n_list),
    .schedule =		ip_vs_wlc_schedule,
    };
#[no_mangle]
unsafe extern "C" fn ip_vs_wlc_init() -> int __init {
    static int __init ip_vs_wlc_init(void)
    {
    return register_ip_vs_scheduler(&ip_vs_wlc_scheduler);
    }
#[no_mangle]
unsafe extern "C" fn ip_vs_wlc_cleanup() -> void __exit {
    static void __exit ip_vs_wlc_cleanup(void)
    {
    unregister_ip_vs_scheduler(&ip_vs_wlc_scheduler);
    synchronize_rcu();
    }
    module_init(ip_vs_wlc_init);
    module_exit(ip_vs_wlc_cleanup);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("ipvs weighted least connection scheduler");
