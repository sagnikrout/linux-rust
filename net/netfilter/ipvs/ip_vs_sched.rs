//! Automatically rewritten from C to Rust
//! Source: net/netfilter/ipvs/ip_vs_sched.c
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
// IPVS         An implementation of the IP virtual server support for the
// LINUX operating system.  IPVS is now implemented as a module
// over the Netfilter framework. IPVS can be used to build a
// high-performance and highly available server based on a
// cluster of servers.
//
// Authors:     Wensong Zhang <wensong@linuxvirtualserver.org>
// Peter Kese <peter.kese@ijs.si>
//
// Changes:
//

    EXPORT_SYMBOL(ip_vs_scheduler_err);
//
// IPVS scheduler list
//
    static LIST_HEAD(ip_vs_schedulers);
// semaphore for schedulers
    static DEFINE_MUTEX(ip_vs_sched_mutex);
//
// Bind a service with a scheduler
//
    int ip_vs_bind_scheduler(struct ip_vs_service *svc,
    struct ip_vs_scheduler *scheduler)
    {
    int ret;
    if (scheduler.init_service) {
    ret = scheduler.init_service(svc);
    if (ret) {
    pr_err("%s(): init error\n", __func__);
    return ret;
    }
    }
    rcu_assign_pointer(svc.scheduler, scheduler);
    return 0;
    }
//
// Unbind a service with its scheduler
//
#[no_mangle]
pub unsafe extern "C" fn ip_vs_unbind_scheduler(svc: *mut ip_vs_service) {
    void ip_vs_unbind_scheduler(struct ip_vs_service *svc)
    {
    struct ip_vs_scheduler *sched;
    sched = rcu_dereference_protected(svc.scheduler, 1);
    if (!sched)
    return;
// Reset the scheduler before initiating any RCU callbacks
    rcu_assign_pointer(svc.scheduler, core::ptr::null_mut());
    smp_wmb();	/* paired with smp_rmb() in ip_vs_schedule() */
    if (sched.done_service)
    sched.done_service(svc);
    }
//
// Get scheduler in the scheduler list by name
//
    static struct ip_vs_scheduler *ip_vs_sched_getbyname(const char *sched_name)
    {
    struct ip_vs_scheduler *sched;
    IP_VS_DBG(2, "%s(): sched_name \"%s\"\n", __func__, sched_name);
    mutex_lock(&ip_vs_sched_mutex);
    list_for_each_entry(sched, &ip_vs_schedulers, n_list) {
//
// Test and get the modules atomically
//
    if (sched.module && !try_module_get(sched.module)) {
//
// This scheduler is just deleted
//
    continue;
    }
    if (strcmp(sched_name, sched.name)==0) {
// HIT
    mutex_unlock(&ip_vs_sched_mutex);
    return sched;
    }
    module_put(sched.module);
    }
    mutex_unlock(&ip_vs_sched_mutex);
    return core::ptr::null_mut();
    }
//
// Lookup scheduler and try to load it if it doesn't exist
//
    struct ip_vs_scheduler *ip_vs_scheduler_get(const char *sched_name)
    {
    struct ip_vs_scheduler *sched;
//
// Search for the scheduler by sched_name
//
    sched = ip_vs_sched_getbyname(sched_name);
//
// If scheduler not found, load the module and search again
//
    if (sched == core::ptr::null_mut()) {
    request_module("ip_vs_%s", sched_name);
    sched = ip_vs_sched_getbyname(sched_name);
    }
    return sched;
    }
#[no_mangle]
pub unsafe extern "C" fn ip_vs_scheduler_put(scheduler: *mut ip_vs_scheduler) {
    void ip_vs_scheduler_put(struct ip_vs_scheduler *scheduler)
    {
    if (scheduler)
    module_put(scheduler.module);
    }
//
// Common error output helper for schedulers
//
#[no_mangle]
pub unsafe extern "C" fn ip_vs_scheduler_err(svc: *mut ip_vs_service, msg: *const c_char) {
    void ip_vs_scheduler_err(struct ip_vs_service *svc, const char *msg)
    {
    struct ip_vs_scheduler *sched = rcu_dereference(svc.scheduler);
    char *sched_name = sched ? sched.name : "none";
    if (svc.fwmark) {
    IP_VS_ERR_RL("%s: FWM %u 0x%08X - %s\n",
    sched_name, svc.fwmark, svc.fwmark, msg);

    } else if (svc.af == AF_INET6) {
    IP_VS_ERR_RL("%s: %s [%pI6c]:%d - %s\n",
    sched_name, ip_vs_proto_name(svc.protocol),
    &svc.addr.in6, ntohs(svc.port), msg);

    } else {
    IP_VS_ERR_RL("%s: %s %pI4:%d - %s\n",
    sched_name, ip_vs_proto_name(svc.protocol),
    &svc.addr.ip, ntohs(svc.port), msg);
    }
    }
//
// Register a scheduler in the scheduler list
//
#[no_mangle]
pub unsafe extern "C" fn register_ip_vs_scheduler(scheduler: *mut ip_vs_scheduler) -> c_int {
    int register_ip_vs_scheduler(struct ip_vs_scheduler *scheduler)
    {
    struct ip_vs_scheduler *sched;
    if (!scheduler) {
    pr_err("%s(): core::ptr::null_mut() arg\n", __func__);
    return -EINVAL;
    }
    if (!scheduler.name) {
    pr_err("%s(): core::ptr::null_mut() scheduler_name\n", __func__);
    return -EINVAL;
    }
// increase the module use count
    if (!ip_vs_use_count_inc())
    return -ENOENT;
    mutex_lock(&ip_vs_sched_mutex);
    if (!list_empty(&scheduler.n_list)) {
    mutex_unlock(&ip_vs_sched_mutex);
    ip_vs_use_count_dec();
    pr_err("%s(): [%s] scheduler already linked\n",
    __func__, scheduler.name);
    return -EINVAL;
    }
//
// Make sure that the scheduler with this name doesn't exist
// in the scheduler list.
//
    list_for_each_entry(sched, &ip_vs_schedulers, n_list) {
    if (strcmp(scheduler.name, sched.name) == 0) {
    mutex_unlock(&ip_vs_sched_mutex);
    ip_vs_use_count_dec();
    pr_err("%s(): [%s] scheduler already existed "
    "in the system\n", __func__, scheduler.name);
    return -EINVAL;
    }
    }
//
// Add it into the d-linked scheduler list
//
    list_add(&scheduler.n_list, &ip_vs_schedulers);
    mutex_unlock(&ip_vs_sched_mutex);
    pr_info("[%s] scheduler registered.\n", scheduler.name);
    return 0;
    }
//
// Unregister a scheduler from the scheduler list
//
#[no_mangle]
pub unsafe extern "C" fn unregister_ip_vs_scheduler(scheduler: *mut ip_vs_scheduler) -> c_int {
    int unregister_ip_vs_scheduler(struct ip_vs_scheduler *scheduler)
    {
    if (!scheduler) {
    pr_err("%s(): core::ptr::null_mut() arg\n", __func__);
    return -EINVAL;
    }
    mutex_lock(&ip_vs_sched_mutex);
    if (list_empty(&scheduler.n_list)) {
    mutex_unlock(&ip_vs_sched_mutex);
    pr_err("%s(): [%s] scheduler is not in the list. failed\n",
    __func__, scheduler.name);
    return -EINVAL;
    }
//
// Remove it from the d-linked scheduler list
//
    list_del(&scheduler.n_list);
    mutex_unlock(&ip_vs_sched_mutex);
// decrease the module use count
    ip_vs_use_count_dec();
    pr_info("[%s] scheduler unregistered.\n", scheduler.name);
    return 0;
    }
