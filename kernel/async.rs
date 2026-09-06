//! Automatically rewritten from C to Rust
//! Source: kernel/async.c
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

macro_rules! EXPORT_SYMBOL { ($($tt:tt)*) => {}; }
macro_rules! EXPORT_SYMBOL_GPL { ($($tt:tt)*) => {}; }
macro_rules! MODULE_LICENSE { ($($tt:tt)*) => {}; }
macro_rules! MODULE_AUTHOR { ($($tt:tt)*) => {}; }
macro_rules! MODULE_DESCRIPTION { ($($tt:tt)*) => {}; }
macro_rules! MODULE_ALIAS { ($($tt:tt)*) => {}; }
macro_rules! module_init { ($($tt:tt)*) => {}; }
macro_rules! module_exit { ($($tt:tt)*) => {}; }
macro_rules! early_initcall { ($($tt:tt)*) => {}; }
macro_rules! core_initcall { ($($tt:tt)*) => {}; }
macro_rules! postcore_initcall { ($($tt:tt)*) => {}; }
macro_rules! arch_initcall { ($($tt:tt)*) => {}; }
macro_rules! subsys_initcall { ($($tt:tt)*) => {}; }
macro_rules! fs_initcall { ($($tt:tt)*) => {}; }
macro_rules! device_initcall { ($($tt:tt)*) => {}; }
macro_rules! late_initcall { ($($tt:tt)*) => {}; }
macro_rules! __setup { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_MUTEX { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_SPINLOCK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DECLARE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DEFINE { ($($tt:tt)*) => {}; }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seq_file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_struct { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_namespace { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cred { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inode { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct notifier_block { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct raw_notifier_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct proc_dir_entry { pub _opaque: [u8; 0] }

pub type pid_type = c_int;
pub type cpu_pm_event = c_int;
pub type spinlock_t = u32;
pub type raw_spinlock_t = u32;
pub type kernel_cap_t = u64;
pub type cap_user_header_t = *mut c_void;
pub type cap_user_data_t = *mut c_void;
pub type async_cookie_t = u64;
pub type atomic_long_t = core::sync::atomic::AtomicI64;


























// SPDX-License-Identifier: GPL-2.0-only
//
// async.c: Asynchronous function calls for boot performance
//
// (C) Copyright 2009 Intel Corporation
// Author: Arjan van de Ven <arjan@linux.intel.com>
//
    Goals and Theory of Operation
    The primary goal of this feature is to reduce the kernel boot time,
    by doing various independent hardware delays and discovery operations
    decoupled and not strictly serialized.
    More specifically, the asynchronous function call concept allows
    certain operations (primarily during system boot) to happen
    asynchronously, out of order, while these operations still
    have their externally visible parts happen sequentially and in-order.
    (not unlike how out-of-order CPUs retire their instructions in order)
    Key to the asynchronous function call implementation is the concept of
    a "sequence cookie" (which, although it has an abstracted type, can be
    thought of as a monotonically incrementing number).
    The async core will assign each scheduled event such a sequence cookie and
    pass this to the called functions.
    The asynchronously called function should before doing a globally visible
    operation, such as registering device numbers, call the
    async_synchronize_cookie() function and pass in its own cookie. The
    async_synchronize_cookie() function will make sure that all asynchronous
    operations that were scheduled prior to the operation corresponding with the
    cookie have completed.
    Subsystem/driver initialization code that scheduled asynchronous probe
    functions, but which shares global resources with other drivers/subsystems
    that do not use the asynchronous call feature, need to do a full
    synchronization with the async_synchronize_full() function, before returning
    from their init function. This is to maintain strict ordering between the
    asynchronous and synchronous parts of the kernel.
//

pub static mut next_cookie: async_cookie_t = 1;
pub const MAX_WORK: c_int = 32768;
// static LIST_HEAD(async_global_pending);	/* pending from all registered doms */
// static ASYNC_DOMAIN(async_dfl_domain);
// static DEFINE_SPINLOCK(async_lock);
    static struct workqueue_struct *async_wq;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct async_entry {
    pub domain_list: list_head,
    pub global_list: list_head,
    pub work: work_struct,
    pub cookie: async_cookie_t,
    pub func: async_func_t,
    pub data: *mut c_void,
    pub domain: *mut async_domain,
}
// static DECLARE_WAIT_QUEUE_HEAD(async_done);
    static atomic_t entry_count;
#[no_mangle]
unsafe extern "C" fn microseconds_since(start: ktime_t) -> c_longlong {
pub static mut now: ktime_t = ktime_get();
    return ktime_to_ns(ktime_sub(now, start)) >> 10;
    }
#[no_mangle]
unsafe extern "C" fn lowest_in_progress(domain: *mut async_domain) -> async_cookie_t {
    struct async_entry *first = core::ptr::null_mut();
pub static mut ret: async_cookie_t = ASYNC_COOKIE_MAX;
    let mut flags = 0;
    spin_lock_irqsave(&async_lock, flags);
    if (domain) {
    if (!list_empty(&domain.pending)) {
    first = list_first_entry(&domain.pending,
    struct async_entry, domain_list);
    }
    } else {
    if (!list_empty(&async_global_pending)) {
    first = list_first_entry(&async_global_pending,
    struct async_entry, global_list);
    }
    }
    if (first) {
    ret = first.cookie;
    }
    spin_unlock_irqrestore(&async_lock, flags);
    return ret;
    }
//
// pick the first pending entry and run it
//
#[no_mangle]
unsafe extern "C" fn async_run_entry_fn(work: *mut work_struct) {
    struct async_entry *entry =
    container_of(work, struct async_entry, work);
    let mut flags = 0;
    let mut calltime;
// 1) run (and print duration)
    pr_debug("calling  %lli_%pS @ %i\n", (long long)entry.cookie,
    entry.func, task_pid_nr(current));
    calltime = ktime_get();
    entry.func(entry.data, entry.cookie);
    pr_debug("initcall %lli_%pS returned after %lld usecs\n",
    (long long)entry.cookie, entry.func,
    microseconds_since(calltime));
// 2) remove self from the pending queues
    spin_lock_irqsave(&async_lock, flags);
    list_del_init(&entry.domain_list);
    list_del_init(&entry.global_list);
// 3) free the entry
    kfree(entry);
    atomic_dec(&entry_count);
    spin_unlock_irqrestore(&async_lock, flags);
// 4) wake up any waiters
    wake_up(&async_done);
    }
#[no_mangle]
pub unsafe extern "C" fn __async_schedule_node_domain() {
    let mut newcookie;
    let mut flags = 0;
// INIT_LIST_HEAD;
// INIT_LIST_HEAD;
// INIT_WORK;
    entry.func = func;
    entry.data = data;
    entry.domain = domain;
    spin_lock_irqsave(&async_lock, flags);
// allocate cookie and queue
    newcookie = entry.cookie = next_cookie++;
    list_add_tail(&entry.domain_list, &domain.pending);
    if (domain.registered) {
    list_add_tail(&entry.global_list, &async_global_pending);
    }
    atomic_inc(&entry_count);
    spin_unlock_irqrestore(&async_lock, flags);
// schedule for execution
    queue_work_node(node, async_wq, &entry.work);
    return newcookie;
    }
//
// async_schedule_node_domain - NUMA specific version of async_schedule_domain
// @func: function to execute asynchronously
// @data: data pointer to pass to the function
// @node: NUMA node that we want to schedule this on or close to
// @domain: the domain
//
// Returns an async_cookie_t that may be used for checkpointing later.
// @domain may be used in the async_synchronize_*_domain() functions to
// wait within a certain synchronization domain rather than globally.
//
// Note: This function may be called from atomic or non-atomic contexts.
//
// The node requested will be honored on a best effort basis. If the node
// has no CPUs associated with it then the work is distributed among all
// available CPUs.
//
    async_cookie_t async_schedule_node_domain(async_func_t func, void *data,
    int node, struct async_domain *domain)
    {
    let mut entry = core::ptr::null_mut();
    let mut flags = 0;
    let mut newcookie;
// allow irq-off callers
    entry = kzalloc_obj(struct async_entry, GFP_ATOMIC);
//
// If we're out of memory or if there's too much work
// pending already, we execute synchronously.
//
    if (!entry || atomic_read(&entry_count) > MAX_WORK) {
    kfree(entry);
    spin_lock_irqsave(&async_lock, flags);
    newcookie = next_cookie++;
    spin_unlock_irqrestore(&async_lock, flags);
// low on memory.. run synchronously
    func(data, newcookie);
    return newcookie;
    }
    return __async_schedule_node_domain(func, data, node, domain, entry);
    }
// EXPORT_SYMBOL_GPL;
//
// async_schedule_node - NUMA specific version of async_schedule
// @func: function to execute asynchronously
// @data: data pointer to pass to the function
// @node: NUMA node that we want to schedule this on or close to
//
// Returns an async_cookie_t that may be used for checkpointing later.
// Note: This function may be called from atomic or non-atomic contexts.
//
// The node requested will be honored on a best effort basis. If the node
// has no CPUs associated with it then the work is distributed among all
// available CPUs.
//
#[no_mangle]
pub unsafe extern "C" fn async_schedule_node(func: async_func_t, data: *mut c_void, node: c_int) -> async_cookie_t {
    return async_schedule_node_domain(func, data, node, &async_dfl_domain);
    }
// EXPORT_SYMBOL_GPL;
//
// async_schedule_dev_nocall - A simplified variant of async_schedule_dev()
// @func: function to execute asynchronously
// @dev: device argument to be passed to function
//
// @dev is used as both the argument for the function and to provide NUMA
// context for where to run the function.
//
// If the asynchronous execution of @func is scheduled successfully, return
// true. Otherwise, do nothing and return false, unlike async_schedule_dev()
// that will run the function synchronously then.
//
#[no_mangle]
pub unsafe extern "C" fn async_schedule_dev_nocall(func: async_func_t, dev: *mut device) -> bool {
    let mut entry = core::ptr::null_mut();
    entry = kzalloc_obj(struct async_entry);
// Give up if there is no memory or too much work.
    if (!entry || atomic_read(&entry_count) > MAX_WORK) {
    kfree(entry);
    return false;
    }
    __async_schedule_node_domain(func, dev, dev_to_node(dev),
    &async_dfl_domain, entry);
    return true;
    }
//
// async_synchronize_full - synchronize all asynchronous function calls
//
// This function waits until all asynchronous function calls have been done.
//
#[no_mangle]
pub unsafe extern "C" fn async_synchronize_full() {
    async_synchronize_full_domain(core::ptr::null_mut());
    }
// EXPORT_SYMBOL_GPL;
//
// async_synchronize_full_domain - synchronize all asynchronous function within a certain domain
// @domain: the domain to synchronize
//
// This function waits until all asynchronous function calls for the
// synchronization domain specified by @domain have been done.
//
#[no_mangle]
pub unsafe extern "C" fn async_synchronize_full_domain(domain: *mut async_domain) {
    async_synchronize_cookie_domain(ASYNC_COOKIE_MAX, domain);
    }
// EXPORT_SYMBOL_GPL;
//
// async_synchronize_cookie_domain - synchronize asynchronous function calls within a certain domain with cookie checkpointing
// @cookie: async_cookie_t to use as checkpoint
// @domain: the domain to synchronize (%NULL for all registered domains)
//
// This function waits until all asynchronous function calls for the
// synchronization domain specified by @domain submitted prior to @cookie
// have been done.
//
#[no_mangle]
pub unsafe extern "C" fn async_synchronize_cookie_domain(cookie: async_cookie_t, domain: *mut async_domain) {
    let mut starttime;
    pr_debug("async_waiting @ %i\n", task_pid_nr(current));
    starttime = ktime_get();
    wait_event(async_done, lowest_in_progress(domain) >= cookie);
    pr_debug("async_continuing @ %i after %lli usec\n", task_pid_nr(current),
    microseconds_since(starttime));
    }
// EXPORT_SYMBOL_GPL;
//
// async_synchronize_cookie - synchronize asynchronous function calls with cookie checkpointing
// @cookie: async_cookie_t to use as checkpoint
//
// This function waits until all asynchronous function calls prior to @cookie
// have been done.
//
#[no_mangle]
pub unsafe extern "C" fn async_synchronize_cookie(cookie: async_cookie_t) {
    async_synchronize_cookie_domain(cookie, &async_dfl_domain);
    }
// EXPORT_SYMBOL_GPL;
//
// current_is_async - is %current an async worker task?
//
// Returns %true if %current is an async worker task.
//
#[no_mangle]
pub unsafe extern "C" fn current_is_async() -> bool {
    struct worker *worker = current_wq_worker();
    return worker && worker.current_func == async_run_entry_fn;
    }
// EXPORT_SYMBOL_GPL;
#[no_mangle]
pub unsafe extern "C" fn async_init() -> c_int {
//
// Async can schedule a number of interdependent work items. However,
// unbound workqueues can handle only upto min_active interdependent
// work items. The default min_active of 8 isn't sufficient for async
// and can lead to stalls. Let's use a dedicated workqueue with raised
// min_active.
//
    async_wq = alloc_workqueue("async", WQ_UNBOUND, 0);
// BUG_ON;
    workqueue_set_min_active(async_wq, WQ_DFL_ACTIVE);
    }