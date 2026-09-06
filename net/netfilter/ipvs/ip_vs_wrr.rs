//! Automatically rewritten from C to Rust
//! Source: net/netfilter/ipvs/ip_vs_wrr.c
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
// IPVS:        Weighted Round-Robin Scheduling module
//
// Authors:     Wensong Zhang <wensong@linuxvirtualserver.org>
//
// Changes:
// Wensong Zhang            :     changed the ip_vs_wrr_schedule to return dest
// Wensong Zhang            :     changed some comestics things for debugging
// Wensong Zhang            :     changed for the d-linked destination list
// Wensong Zhang            :     added the ip_vs_wrr_update_svc
// Julian Anastasov         :     fixed the bug of returning destination
// with weight 0 when all weights are zero
//

// The WRR algorithm depends on some caclulations:
// - mw: maximum weight
// - di: weight step, greatest common divisor from all weights
// - cw: current required weight
// As result, all weights are in the [di..mw] range with a step=di.
//
// First, we start with cw = mw and select dests with weight >= cw.
// Then cw is reduced with di and all dests are checked again.
// Last pass should be with cw = di. We have mw/di passes in total:
//
// pass 1: cw = max weight
// pass 2: cw = max weight - di
// pass 3: cw = max weight - 2 * di
// ...
// last pass: cw = di
//
// Weights are supposed to be >= di but we run in parallel with
// weight changes, it is possible some dest weight to be reduced
// below di, bad if it is the only available dest.
//
// So, we modify how mw is calculated, now it is reduced with (di - 1),
// so that last cw is 1 to catch such dests with weight below di:
// pass 1: cw = max weight - (di - 1)
// pass 2: cw = max weight - di - (di - 1)
// pass 3: cw = max weight - 2 * di - (di - 1)
// ...
// last pass: cw = 1
//
// current destination pointer for weighted round-robin scheduling
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_vs_wrr_mark {
    pub /: *mut *mut *mut ip_vs_dest cl; / current dest or head,
    pub /: *mut *mut int cw; / current weight,
    pub /: *mut *mut int mw; / maximum weight,
    pub /: *mut *mut int di; / decreasing interval,
    pub rcu_head: rcu_head,
}

#[no_mangle]
unsafe extern "C" fn ip_vs_wrr_gcd_weight(svc: *mut ip_vs_service) -> c_int {
    static int ip_vs_wrr_gcd_weight(struct ip_vs_service *svc)
    {
    struct ip_vs_dest *dest;
    int weight;
    let mut g: c_int = 0;
    list_for_each_entry(dest, &svc.destinations, n_list) {
    weight = atomic_read(&dest.weight);
    if (weight > 0) {
    if (g > 0)
    g = gcd(weight, g);
    else
    g = weight;
    }
    }
    return g ? g : 1;
    }
//
// Get the maximum weight of the service destinations.
//
#[no_mangle]
unsafe extern "C" fn ip_vs_wrr_max_weight(svc: *mut ip_vs_service) -> c_int {
    static int ip_vs_wrr_max_weight(struct ip_vs_service *svc)
    {
    struct ip_vs_dest *dest;
    int new_weight, weight = 0;
    list_for_each_entry(dest, &svc.destinations, n_list) {
    new_weight = atomic_read(&dest.weight);
    if (new_weight > weight)
    weight = new_weight;
    }
    return weight;
    }
#[no_mangle]
unsafe extern "C" fn ip_vs_wrr_init_svc(svc: *mut ip_vs_service) -> c_int {
    static int ip_vs_wrr_init_svc(struct ip_vs_service *svc)
    {
    struct ip_vs_wrr_mark *mark;
//
// Allocate the mark variable for WRR scheduling
//
    mark = kmalloc_obj(struct ip_vs_wrr_mark);
    if (mark == core::ptr::null_mut())
    return -ENOMEM;
    mark.cl = list_entry(&svc.destinations, struct ip_vs_dest, n_list);
    mark.di = ip_vs_wrr_gcd_weight(svc);
    mark.mw = ip_vs_wrr_max_weight(svc) - (mark.di - 1);
    mark.cw = mark.mw;
    svc.sched_data = mark;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ip_vs_wrr_done_svc(svc: *mut ip_vs_service) {
    static void ip_vs_wrr_done_svc(struct ip_vs_service *svc)
    {
    struct ip_vs_wrr_mark *mark = svc.sched_data;
//
// Release the mark variable
//
    kfree_rcu(mark, rcu_head);
    }
    static int ip_vs_wrr_dest_changed(struct ip_vs_service *svc,
    struct ip_vs_dest *dest)
    {
    struct ip_vs_wrr_mark *mark = svc.sched_data;
    spin_lock_bh(&svc.sched_lock);
    mark.cl = list_entry(&svc.destinations, struct ip_vs_dest, n_list);
    mark.di = ip_vs_wrr_gcd_weight(svc);
    mark.mw = ip_vs_wrr_max_weight(svc) - (mark.di - 1);
    if (mark.cw > mark.mw || !mark.cw)
    mark.cw = mark.mw;
#[no_mangle]
pub unsafe extern "C" fn if(1: mark->di >) -> else {
    else if (mark.di > 1)
    mark.cw = (mark.cw / mark.di) * mark.di + 1;
    spin_unlock_bh(&svc.sched_lock);
    return 0;
    }
//
// Weighted Round-Robin Scheduling
//
    static struct ip_vs_dest *
    ip_vs_wrr_schedule(struct ip_vs_service *svc, const struct sk_buff *skb,
    struct ip_vs_iphdr *iph)
    {
    struct ip_vs_dest *dest, *last, *stop = core::ptr::null_mut();
    struct ip_vs_wrr_mark *mark = svc.sched_data;
    let mut last_pass: bool = false, restarted = false;
    IP_VS_DBG(6, "%s(): Scheduling...\n", __func__);
    spin_lock_bh(&svc.sched_lock);
    dest = mark.cl;
// No available dests?
    if (mark.mw == 0)
    goto err_noavail;
    last = dest;
// Stop only after all dests were checked for weight >= 1 (last pass)
    while (1) {
    list_for_each_entry_continue_rcu(dest,
    &svc.destinations,
    n_list) {
    if (!(dest.flags & IP_VS_DEST_F_OVERLOAD) &&
    atomic_read(&dest.weight) >= mark.cw)
    goto found;
    if (dest == stop)
    goto err_over;
    }
    mark.cw -= mark.di;
    if (mark.cw <= 0) {
    mark.cw = mark.mw;
// Stop if we tried last pass from first dest:
// 1. last_pass: we started checks when cw > di but
// then all dests were checked for w >= 1
// 2. last was head: the first and only traversal
// was for weight >= 1, for all dests.
//
    if (last_pass ||
    &last.n_list == &svc.destinations)
    goto err_over;
    restarted = true;
    }
    last_pass = mark.cw <= mark.di;
    if (last_pass && restarted &&
    &last.n_list != &svc.destinations) {
// First traversal was for w >= 1 but only
// for dests after 'last', now do the same
// for all dests up to 'last'.
//
    stop = last;
    }
    }
    found:
    IP_VS_DBG_BUF(6, "WRR: server %s:%u "
    "activeconns %d refcnt %d weight %d\n",
    IP_VS_DBG_ADDR(dest.af, &dest.addr), ntohs(dest.port),
    atomic_read(&dest.activeconns),
    refcount_read(&dest.refcnt),
    atomic_read(&dest.weight));
    mark.cl = dest;
    out:
    spin_unlock_bh(&svc.sched_lock);
    return dest;
    err_noavail:
    mark.cl = dest;
    dest = core::ptr::null_mut();
    ip_vs_scheduler_err(svc, "no destination available");
    goto out;
    err_over:
    mark.cl = dest;
    dest = core::ptr::null_mut();
    ip_vs_scheduler_err(svc, "no destination available: "
    "all destinations are overloaded");
    goto out;
    }
    static struct ip_vs_scheduler ip_vs_wrr_scheduler = {
    .name =			"wrr",
    .refcnt =		ATOMIC_INIT(0),
    .module =		THIS_MODULE,
    .n_list =		LIST_HEAD_INIT(ip_vs_wrr_scheduler.n_list),
    .init_service =		ip_vs_wrr_init_svc,
    .done_service =		ip_vs_wrr_done_svc,
    .add_dest =		ip_vs_wrr_dest_changed,
    .del_dest =		ip_vs_wrr_dest_changed,
    .upd_dest =		ip_vs_wrr_dest_changed,
    .schedule =		ip_vs_wrr_schedule,
    };
#[no_mangle]
unsafe extern "C" fn ip_vs_wrr_init() -> int __init {
    static int __init ip_vs_wrr_init(void)
    {
    return register_ip_vs_scheduler(&ip_vs_wrr_scheduler) ;
    }
#[no_mangle]
unsafe extern "C" fn ip_vs_wrr_cleanup() -> void __exit {
    static void __exit ip_vs_wrr_cleanup(void)
    {
    unregister_ip_vs_scheduler(&ip_vs_wrr_scheduler);
    synchronize_rcu();
    }
    module_init(ip_vs_wrr_init);
    module_exit(ip_vs_wrr_cleanup);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("ipvs weighted round-robin scheduler");
