//! Automatically rewritten from C to Rust
//! Source: net/netfilter/ipvs/ip_vs_lc.c
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
// IPVS:        Least-Connection Scheduling module
//
// Authors:     Wensong Zhang <wensong@linuxvirtualserver.org>
//
// Changes:
// Wensong Zhang            :     added the ip_vs_lc_update_svc
// Wensong Zhang            :     added any dest with weight=0 is quiesced
//

//
// Least Connection scheduling
//
    static struct ip_vs_dest *
    ip_vs_lc_schedule(struct ip_vs_service *svc, const struct sk_buff *skb,
    struct ip_vs_iphdr *iph)
    {
    struct ip_vs_dest *dest, *least = core::ptr::null_mut();
    let mut loh: c_uint = 0, doh;
    IP_VS_DBG(6, "%s(): Scheduling...\n", __func__);
//
// Simply select the server with the least number of
// (activeconns*256) + totalconns
// Except whose weight is equal to zero.
// If the weight is equal to zero, it means that the server is
// quiesced, the existing connections to the server still get
// served, but no new connection is assigned to the server.
//
    list_for_each_entry_rcu(dest, &svc.destinations, n_list) {
    if ((dest.flags & IP_VS_DEST_F_OVERLOAD) ||
    atomic_read(&dest.weight) == 0)
    continue;
    doh = ip_vs_dest_conn_overhead(dest);
    if (!least || doh < loh) {
    least = dest;
    loh = doh;
    }
    }
    if (!least)
    ip_vs_scheduler_err(svc, "no destination available");
    else
    IP_VS_DBG_BUF(6, "LC: server %s:%u activeconns %d "
    "inactconns %d\n",
    IP_VS_DBG_ADDR(least.af, &least.addr),
    ntohs(least.port),
    atomic_read(&least.activeconns),
    ip_vs_dest_inactconns(least));
    return least;
    }
    static struct ip_vs_scheduler ip_vs_lc_scheduler = {
    .name =			"lc",
    .refcnt =		ATOMIC_INIT(0),
    .module =		THIS_MODULE,
    .n_list =		LIST_HEAD_INIT(ip_vs_lc_scheduler.n_list),
    .schedule =		ip_vs_lc_schedule,
    };
#[no_mangle]
unsafe extern "C" fn ip_vs_lc_init() -> int __init {
    static int __init ip_vs_lc_init(void)
    {
    return register_ip_vs_scheduler(&ip_vs_lc_scheduler) ;
    }
#[no_mangle]
unsafe extern "C" fn ip_vs_lc_cleanup() -> void __exit {
    static void __exit ip_vs_lc_cleanup(void)
    {
    unregister_ip_vs_scheduler(&ip_vs_lc_scheduler);
    synchronize_rcu();
    }
    module_init(ip_vs_lc_init);
    module_exit(ip_vs_lc_cleanup);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("ipvs least connection scheduler");
