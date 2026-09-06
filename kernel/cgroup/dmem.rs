//! Automatically rewritten from C to Rust
//! Source: kernel/cgroup/dmem.c
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
// === KERNEL_MACRO_PRELUDE_START ===
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
macro_rules! ARRAY_SIZE { ($($tt:tt)*) => { 1 }; }
macro_rules! container_of { ($($tt:tt)*) => { core::ptr::null_mut() }; }
macro_rules! sizeof { ($($tt:tt)*) => { 0usize }; }
macro_rules! IS_ENABLED { ($($tt:tt)*) => { false }; }
macro_rules! DECLARE_WORK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_WAKE_Q { ($($tt:tt)*) => {}; }
macro_rules! LLIST_HEAD { ($($tt:tt)*) => {}; }
macro_rules! LIST_HEAD { ($($tt:tt)*) => {}; }
macro_rules! SET_UID { ($($tt:tt)*) => {}; }
macro_rules! SET_GID { ($($tt:tt)*) => {}; }
macro_rules! list_for_each_entry { ($($tt:tt)*) => { if false }; }
macro_rules! list_for_each_entry_safe { ($($tt:tt)*) => { if false }; }
macro_rules! llist_for_each_entry_safe { ($($tt:tt)*) => { if false }; }
macro_rules! pr_info_once { ($($tt:tt)*) => {}; }
macro_rules! pr_info { ($($tt:tt)*) => {}; }
macro_rules! pr_warn { ($($tt:tt)*) => {}; }
macro_rules! pr_err { ($($tt:tt)*) => {}; }
macro_rules! pr_debug { ($($tt:tt)*) => {}; }
macro_rules! early_param { ($($tt:tt)*) => {}; }
macro_rules! BUILD_BUG_ON { ($($tt:tt)*) => {}; }
macro_rules! WARN_ON { ($($tt:tt)*) => { false }; }
macro_rules! WARN_ON_ONCE { ($($tt:tt)*) => { false }; }
macro_rules! BUG_ON { ($($tt:tt)*) => {}; }
macro_rules! BUG { () => {}; }
macro_rules! IS_ERR { ($($tt:tt)*) => { false }; }
macro_rules! PTR_ERR { ($($tt:tt)*) => { 0 }; }
macro_rules! ERR_PTR { ($($tt:tt)*) => { core::ptr::null_mut() }; }

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
pub struct ctl_table_header { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_root { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_set { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct proc_dir_entry { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_namespace { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ipc64_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ipc_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc64_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kern_ipc_perm { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_params { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_queue { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_msg { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_msgseg { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_sender { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_receiver { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sem { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sembuf { pub sem_num: u16, pub sem_op: i16, pub sem_flg: i16 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sem_array { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shmid_kernel { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shm_file_data { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wake_q_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct work_struct { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct llist_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct list_head { pub _opaque: [u8; 0] }

pub type pid_type = c_int;
pub type cpu_pm_event = c_int;
pub type spinlock_t = u32;
pub type raw_spinlock_t = u32;
pub type kernel_cap_t = u64;
pub type cap_user_header_t = *mut c_void;
pub type cap_user_data_t = *mut c_void;
pub type async_cookie_t = u64;
pub type atomic_long_t = core::sync::atomic::AtomicI64;
pub type key_t = i32;
pub type kuid_t = u32;
pub type kgid_t = u32;
pub type int = c_int;
pub type uint = c_uint;
pub type ulong = c_ulong;
pub type long = c_long;
pub type void = c_void;

// Standard Linux Error Codes
pub const EPERM: c_int = 1;
pub const ENOENT: c_int = 2;
pub const ESRCH: c_int = 3;
pub const EINTR: c_int = 4;
pub const EIO: c_int = 5;
pub const ENXIO: c_int = 6;
pub const E2BIG: c_int = 7;
pub const ENOEXEC: c_int = 8;
pub const EBADF: c_int = 9;
pub const ECHILD: c_int = 10;
pub const EAGAIN: c_int = 11;
pub const ENOMEM: c_int = 12;
pub const EACCES: c_int = 13;
pub const EFAULT: c_int = 14;
pub const EBUSY: c_int = 16;
pub const EEXIST: c_int = 17;
pub const EXDEV: c_int = 18;
pub const ENODEV: c_int = 19;
pub const ENOTDIR: c_int = 20;
pub const EISDIR: c_int = 21;
pub const EINVAL: c_int = 22;
pub const ENFILE: c_int = 23;
pub const EMFILE: c_int = 24;
pub const ENOSPC: c_int = 28;
pub const EROFS: c_int = 30;
pub const EIDRM: c_int = 43;
pub const EOPNOTSUPP: c_int = 95;
pub const ENOTSUPP: c_int = 524;

// Standard Memory Constants
pub const PAGE_SHIFT: usize = 12;
pub const PAGE_SIZE: usize = 1 << PAGE_SHIFT;
pub const GFP_KERNEL: c_uint = 0xcc0;
pub const GFP_ATOMIC: c_uint = 0x80000;
pub const GFP_NOWAIT: c_uint = 0;

// Standard Core Primitives
extern "C" {
    pub static current: *mut task_struct;
    pub fn printk(fmt: *const c_char, ...) -> c_int;
    pub fn rcu_read_lock();
    pub fn rcu_read_unlock();
    pub fn copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn copy_to_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn kmalloc(size: usize, flags: c_uint) -> *mut c_void;
    pub fn kfree(ptr: *mut c_void);
}
// === KERNEL_MACRO_PRELUDE_END ===


// SPDX-License-Identifier: GPL-2.0
//
// Copyright 2023-2024 Intel Corporation (Maarten Lankhorst <dev@lankhorst.se>)
// Copyright 2024 Red Hat (Maxime Ripard <mripard@kernel.org>)
// Partially based on the rdma and misc controllers, which bear the following copyrights:
//
// Copyright 2020 Google LLC
// Copyright (C) 2016 Parav Pandit <pandit.parav@gmail.com>
//

// Maximum reclaim attempts before giving up when lowering dmem.max.
pub const DMEM_MAX_RECLAIM_RETRIES: c_int = 16;
// SRCU domain serialising reclaim callbacks against region unregistration.
pub static mut dmemcg_srcu: usize = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmem_cgroup_region {
//
// @ref: References keeping the region alive.
// Keeps the region reference alive after a succesful RCU lookup.
//
    pub ref: kref,
// @rcu: RCU head for freeing
    pub rcu: rcu_head,
//
// @region_node: Linked into &dmem_cgroup_regions list.
// Protected by RCU and global spinlock.
//
    pub region_node: list_head,
//
// @pools: List of pools linked to this region.
// Protected by global spinlock only
//
    pub pools: list_head,
// @size: Size of region, in bytes
    pub size: u64,
// @name: Name describing the node, set by dmem_cgroup_register_region
    pub name: *mut c_char,
//
// @unregistered: Whether the region is unregistered by its caller.
// No new pools should be added to the region afterwards, and no new
// reclaim callbacks should be invoked.
//
    pub unregistered: bool,
//
// @ops: Optional driver operations for this region.
//
    pub ops: *const dmem_cgroup_ops,
// @reclaim_priv: Private data passed to @ops->reclaim.
    pub reclaim_priv: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmemcg_state {
    pub css: cgroup_subsys_state,
    pub pools: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmem_cgroup_pool_state {
    pub region: *mut dmem_cgroup_region,
    pub cs: *mut dmemcg_state,
// css node, RCU protected against region teardown
    pub css_node: list_head,
// dev node, no RCU protection required
    pub region_node: list_head,
    pub rcu: rcu_head,
    pub cnt: page_counter,
    pub parent: *mut dmem_cgroup_pool_state,
    pub ref: refcount_t,
    pub inited: bool,
}

//
// 3 operations require locking protection:
// - Registering and unregistering region to/from list, requires global lock.
// - Adding a dmem_cgroup_pool_state to a CSS, removing when CSS is freed.
// - Adding a dmem_cgroup_pool_state to a region list.
//
// Since for the most common operations RCU provides enough protection, I
// do not think more granular locking makes sense. Most protection is offered
// by RCU and the lockless operating page_counter.
//
pub static mut dmemcg_lock: usize = 0;
pub static mut dmem_cgroup_regions: usize = 0;
// forward_decl: dmemcg_free_region;
// forward_decl: dmemcg_pool_free_rcu;
#[no_mangle]
pub unsafe extern "C" fn css_to_dmemcs(css: *mut cgroup_subsys_state) -> *mut c_void {
    return container_of!(css, dmemcg_state, css);
    }
#[no_mangle]
pub unsafe extern "C" fn get_current_dmemcs() -> *mut c_void {
    return css_to_dmemcs(task_get_css(current, dmem_cgrp_id));
    }
#[no_mangle]
pub unsafe extern "C" fn parent_dmemcs(cg: *mut dmemcg_state) -> *mut c_void {
    return cg.css.parent ? css_to_dmemcs(cg.css.parent) : core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn dmemcg_pool_get(pool: *mut dmem_cgroup_pool_state) {
    refcount_inc(&pool.ref);
    }
#[no_mangle]
unsafe extern "C" fn dmemcg_pool_tryget(pool: *mut dmem_cgroup_pool_state) -> bool {
    return refcount_inc_not_zero(&pool.ref);
    }
#[no_mangle]
unsafe extern "C" fn dmemcg_pool_put(pool: *mut dmem_cgroup_pool_state) {
    if (!refcount_dec_and_test(&pool.ref)) {
    return;
    }
    call_rcu(&pool.rcu, dmemcg_pool_free_rcu);
    }
#[no_mangle]
unsafe extern "C" fn dmemcg_pool_free_rcu(rcu: *mut rcu_head) {
    let mut pool = container_of!(rcu, typeof(*pool), rcu);
    if (pool.parent) {
    dmemcg_pool_put(pool.parent);
    }
    kref_put(&pool.region.ref, dmemcg_free_region);
    kfree(pool);
    }
#[no_mangle]
unsafe extern "C" fn free_cg_pool(pool: *mut dmem_cgroup_pool_state) {
    list_del(&pool.region_node);
    dmemcg_pool_put(pool);
    }
#[no_mangle]
pub unsafe extern "C" fn set_resource_min(pool: *mut dmem_cgroup_pool_state, val: u64, nonblock: bool) {
    page_counter_set_min(&pool.cnt, val);
    }
#[no_mangle]
pub unsafe extern "C" fn set_resource_low(pool: *mut dmem_cgroup_pool_state, val: u64, nonblock: bool) {
    page_counter_set_low(&pool.cnt, val);
    }
#[no_mangle]
pub unsafe extern "C" fn set_resource_max(pool: *mut dmem_cgroup_pool_state, val: u64, nonblock: bool) {
    let mut region = pool.region;
pub static mut limit: c_ulong = 0;
// Apply the new limit immediately so concurrent allocations are throttled.
    xchg(&pool.cnt.max, limit);
    if (nonblock) {
    return;
    }
pub static mut srcu_idx: c_int = 0;
    if (!READ_ONCE(region.unregistered) && region.ops && region.ops.reclaim) {
    while ( ) {
pub static mut usage: u64 = 0;
    let mut ret = 0;
    if (usage <= limit) {
    break;
    }
    if (signal_pending(current)) {
    break;
    }
    ret = region.ops.reclaim(pool, usage - limit, region.reclaim_priv);
// -ENOSPC means no progress; other errors are fatal.
    if (ret && (ret != -ENOSPC || !retries--)) {
    break;
    }
    cond_resched();
    }
    }
    srcu_read_unlock(&dmemcg_srcu, srcu_idx);
    }
#[no_mangle]
unsafe extern "C" fn get_resource_low(pool: *mut dmem_cgroup_pool_state) -> u64 {
    return pool ? READ_ONCE(pool.cnt.low) : 0;
    }
#[no_mangle]
unsafe extern "C" fn get_resource_min(pool: *mut dmem_cgroup_pool_state) -> u64 {
    return pool ? READ_ONCE(pool.cnt.min) : 0;
    }
#[no_mangle]
unsafe extern "C" fn get_resource_max(pool: *mut dmem_cgroup_pool_state) -> u64 {
    return pool ? READ_ONCE(pool.cnt.max) : PAGE_COUNTER_MAX;
    }
#[no_mangle]
unsafe extern "C" fn get_resource_current(pool: *mut dmem_cgroup_pool_state) -> u64 {
    return pool ? page_counter_read(&pool.cnt) : 0;
    }
#[no_mangle]
unsafe extern "C" fn get_resource_peak(pool: *mut dmem_cgroup_pool_state) -> u64 {
    return pool ? READ_ONCE(pool.cnt.watermark) : 0;
    }
#[no_mangle]
unsafe extern "C" fn reset_all_resource_limits(rpool: *mut dmem_cgroup_pool_state) {
    set_resource_min(rpool, 0, false);
    set_resource_low(rpool, 0, false);
// nonblock: raising to max makes reclaim a no-op; sleeping is forbidden here.
    set_resource_max(rpool, PAGE_COUNTER_MAX, true);
    }
#[no_mangle]
unsafe extern "C" fn dmemcs_offline(css: *mut cgroup_subsys_state) {
    let mut dmemcs = css_to_dmemcs(css);
pub static mut pool: *mut c_void = core::ptr::null_mut();
    rcu_read_lock();
    list_for_each_entry_rcu(pool, &dmemcs.pools, css_node) {
    reset_all_resource_limits(pool);
    }
    rcu_read_unlock();
    }
#[no_mangle]
unsafe extern "C" fn dmemcs_free(css: *mut cgroup_subsys_state) {
    let mut dmemcs = css_to_dmemcs(css);
    let mut pool = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
    spin_lock(&dmemcg_lock);
    list_for_each_entry_safe(pool, next, &dmemcs.pools, css_node) {
//
// The pool is dead and all references are 0,
// no need for RCU protection with list_del_rcu or freeing.
//
    list_del(&pool.css_node);
    free_cg_pool(pool);
    }
    spin_unlock(&dmemcg_lock);
    kfree(dmemcs);
    }
#[no_mangle]
pub unsafe extern "C" fn dmemcs_alloc(parent_css: *mut cgroup_subsys_state) -> *mut c_void {
    let mut dmemcs = kzalloc_obj(*dmemcs);
    if (!dmemcs) {
    return ERR_PTR(-ENOMEM);
    }
    INIT_LIST_HEAD(&dmemcs.pools);
    return &dmemcs.css;
    }
#[no_mangle]
pub unsafe extern "C" fn find_cg_pool_locked(dmemcs: *mut dmemcg_state, region: *mut dmem_cgroup_region) -> *mut c_void {
pub static mut pool: *mut c_void = core::ptr::null_mut();
    list_for_each_entry_rcu(pool, &dmemcs.pools, css_node, spin_is_locked(&dmemcg_lock)) {
    if (pool.region == region)
    return pool;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn pool_parent(pool: *mut dmem_cgroup_pool_state) -> *mut c_void {
    if (!pool.cnt.parent) {
    return core::ptr::null_mut();
    }
    return container_of!(pool.cnt.parent, typeof(*pool), cnt);
    }
#[no_mangle]
pub unsafe extern "C" fn dmem_cgroup_calculate_protection(limit_pool: *mut dmem_cgroup_pool_state, test_pool: *mut dmem_cgroup_pool_state) {
pub static mut climit: *mut c_void = core::ptr::null_mut();
pub static mut css: *mut c_void = core::ptr::null_mut();
pub static mut dmemcg_iter: *mut c_void = core::ptr::null_mut();
    let mut pool = core::ptr::null_mut();
    let mut found_pool = core::ptr::null_mut();
    climit = &limit_pool.cnt;
    rcu_read_lock();
    css_for_each_descendant_pre(css, &limit_pool.cs.css) {
    dmemcg_iter = container_of!(css, dmemcg_state, css);
    found_pool = core::ptr::null_mut();
    list_for_each_entry_rcu(pool, &dmemcg_iter.pools, css_node) {
    if (pool.region == limit_pool.region) {
    found_pool = pool;
    break;
    }
    }
    if (!found_pool) {
    continue;
    }
    page_counter_calculate_protection(
    climit, &found_pool.cnt, true);
    if (found_pool == test_pool) {
    break;
    }
    }
    rcu_read_unlock();
    }
//
// dmem_cgroup_state_evict_valuable() - Check if we should evict from test_pool
// @limit_pool: The pool for which we hit limits
// @test_pool: The pool for which to test
// @ignore_low: Whether we have to respect low watermarks.
// @ret_hit_low: Pointer to whether it makes sense to consider low watermark.
//
// This function returns true if we can evict from @test_pool, false if not.
// When returning false and @ignore_low is false, @ret_hit_low may
// be set to true to indicate this function can be retried with @ignore_low
// set to true.
//
// Return: bool
//
#[no_mangle]
pub unsafe extern "C" fn dmem_cgroup_state_evict_valuable(limit_pool: *mut dmem_cgroup_pool_state, test_pool: *mut dmem_cgroup_pool_state, ignore_low: bool, ret_hit_low: *mut bool) -> bool {
    let mut pool = test_pool;
pub static mut ctest: *mut c_void = core::ptr::null_mut();
    u64 used, min, low;
// Can always evict from current pool, despite limits
    if (limit_pool == test_pool) {
    return true;
    }
    if (limit_pool) {
    if (!parent_dmemcs(limit_pool.cs)) {
    return true;
    }
    for (pool = test_pool; pool && limit_pool != pool; pool = pool_parent(pool)) {
    {}
    }
    if (!pool) {
    return false;
    }
    } else {
//
// If there is no cgroup limiting memory usage, use the root
// cgroup instead for limit calculations.
//
    for (limit_pool = test_pool; pool_parent(limit_pool); limit_pool = pool_parent(limit_pool)) {
    {}
    }
    }
    ctest = &test_pool.cnt;
    dmem_cgroup_calculate_protection(limit_pool, test_pool);
    used = page_counter_read(ctest);
    min = READ_ONCE(ctest.emin);
    if (used <= min) {
    return false;
    }
    if (!ignore_low) {
    low = READ_ONCE(ctest.elow);
    if (used > low) {
    return true;
    }
// ret_hit_low = true;
    return false;
    }
    return true;
    }
    EXPORT_SYMBOL_GPL(dmem_cgroup_state_evict_valuable);
#[no_mangle]
pub unsafe extern "C" fn alloc_pool_single(dmemcs: *mut dmemcg_state, region: *mut dmem_cgroup_region, allocpool: *mut *mut dmem_cgroup_pool_state) -> *mut c_void {
    let mut parent = parent_dmemcs(dmemcs);
    struct dmem_cgroup_pool_state *pool, *ppool = core::ptr::null_mut();
    if (!*allocpool) {
    pool = kzalloc_obj(*pool, GFP_NOWAIT);
    if (!pool) {
    return ERR_PTR(-ENOMEM);
    }
    } else {
    pool = *allocpool;
// allocpool = NULL;
    }
    pool.region = region;
    pool.cs = dmemcs;
    if (parent) {
    ppool = find_cg_pool_locked(parent, region);
    }
    page_counter_init(&pool.cnt,
    ppool ? &ppool.cnt : core::ptr::null_mut(), true);
    reset_all_resource_limits(pool);
    refcount_set(&pool.ref, 1);
    kref_get(&region.ref);
    if (ppool && !pool.parent) {
    pool.parent = ppool;
    dmemcg_pool_get(ppool);
    }
    list_add_tail_rcu(&pool.css_node, &dmemcs.pools);
    list_add_tail(&pool.region_node, &region.pools);
    if (!parent) {
    pool.inited = true;
    }
    else {
    pool.inited = ppool ? ppool.inited : false;
    }
    return pool;
    }
#[no_mangle]
pub unsafe extern "C" fn get_cg_pool_locked(dmemcs: *mut dmemcg_state, region: *mut dmem_cgroup_region, allocpool: *mut *mut dmem_cgroup_pool_state) -> *mut c_void {
    let mut pool = core::ptr::null_mut();
    let mut ppool = core::ptr::null_mut();
    let mut retpool = core::ptr::null_mut();
    let mut p = core::ptr::null_mut();
    let mut pp = core::ptr::null_mut();
//
// Recursively create pool, we may not initialize yet on
// recursion, this is done as a separate step.
//
    for (p = dmemcs; p; p = parent_dmemcs(p)) {
    pool = find_cg_pool_locked(p, region);
    if (!pool) {
    pool = alloc_pool_single(p, region, allocpool);
    }
    if (IS_ERR(pool)) {
    return pool;
    }
    if (p == dmemcs && pool.inited) {
    return pool;
    }
    if (pool.inited) {
    break;
    }
    }
    retpool = pool = find_cg_pool_locked(dmemcs, region);
    for (p = dmemcs, pp = parent_dmemcs(dmemcs); pp; p = pp, pp = parent_dmemcs(p)) {
    if (pool.inited) {
    break;
    }
// ppool was created if it didn't exist by above loop.
    ppool = find_cg_pool_locked(pp, region);
// Fix up parent links, mark as inited.
    pool.cnt.parent = &ppool.cnt;
    if (ppool && !pool.parent) {
    pool.parent = ppool;
    dmemcg_pool_get(ppool);
    }
    pool.inited = true;
    pool = ppool;
    }
    return retpool;
    }
#[no_mangle]
unsafe extern "C" fn dmemcg_free_rcu(rcu: *mut rcu_head) {
    let mut region = container_of!(rcu, typeof(*region), rcu);
    let mut pool = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
    list_for_each_entry_safe(pool, next, &region.pools, region_node) {
    free_cg_pool(pool);
    }
    kfree(region.name);
    kfree(region);
    }
#[no_mangle]
unsafe extern "C" fn dmemcg_free_region(ref: *mut kref) {
    let mut cgregion = container_of!(ref, typeof(*cgregion), ref);
    call_rcu(&cgregion.rcu, dmemcg_free_rcu);
    }
//
// dmem_cgroup_unregister_region() - Unregister a previously registered region.
// @region: The region to unregister.
//
// This function undoes dmem_cgroup_register_region.  It drains any
// in-flight reclaim callbacks before returning, so the caller may safely
// free the resources pointed to by the @reclaim_priv that was passed at
// registration time.
//
#[no_mangle]
pub unsafe extern "C" fn dmem_cgroup_unregister_region(region: *mut dmem_cgroup_region) {
    let mut pool = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
    if (!region) {
    return;
    }
    spin_lock(&dmemcg_lock);
// Remove from global region list
    list_del_rcu(&region.region_node);
    list_for_each_entry_safe(pool, next, &region.pools, region_node) {
    list_del_rcu(&pool.css_node);
    list_del(&pool.region_node);
    dmemcg_pool_put(pool);
    }
//
// Ensure any RCU based lookups fail. Additionally,
// no new pools should be added to the dead region
// by get_cg_pool_unlocked.
//
    WRITE_ONCE(region.unregistered, true);
    spin_unlock(&dmemcg_lock);
    synchronize_srcu(&dmemcg_srcu);
    kref_put(&region.ref, dmemcg_free_region);
    }
    EXPORT_SYMBOL_GPL(dmem_cgroup_unregister_region);
//
// dmem_cgroup_register_region() - Register a regions for dev cgroup.
// @init: Initialization parameters for the region.
// @fmt: Region parameters to register
//
// This function registers a node in the dmem cgroup with the
// name given. After calling this function, the region can be
// used for allocations.
//
// Return: NULL or a struct on success, PTR_ERR on failure.
//
#[no_mangle]
pub unsafe extern "C" fn dmem_cgroup_register_region(init: *mut dmem_cgroup_init, fmt: *mut c_char) -> *mut c_void {
pub static mut ret: *mut c_void = core::ptr::null_mut();
pub static mut region_name: *mut c_void = core::ptr::null_mut();
    let mut ap;
    if (!init || !init.size) {
    return core::ptr::null_mut();
    }
    va_start(ap, fmt);
    region_name = kvasprintf(GFP_KERNEL, fmt, ap);
    va_end(ap);
    if (!region_name) {
    return ERR_PTR(-ENOMEM);
    }
    ret = kzalloc_obj(*ret);
    if (!ret) {
    kfree(region_name);
    return ERR_PTR(-ENOMEM);
    }
    INIT_LIST_HEAD(&ret.pools);
    ret.name = region_name;
    ret.size = init.size;
    ret.ops = init.ops;
    ret.reclaim_priv = init.reclaim_priv;
    kref_init(&ret.ref);
    spin_lock(&dmemcg_lock);
    list_add_tail_rcu(&ret.region_node, &dmem_cgroup_regions);
    spin_unlock(&dmemcg_lock);
    return ret;
    }
    EXPORT_SYMBOL_GPL(dmem_cgroup_register_region);
#[no_mangle]
pub unsafe extern "C" fn dmemcg_get_region_by_name(name: *mut c_char) -> *mut c_void {
pub static mut region: *mut c_void = core::ptr::null_mut();
    list_for_each_entry_rcu(region, &dmem_cgroup_regions, region_node, spin_is_locked(&dmemcg_lock)) {
    if (!strcmp(name, region.name) &&
    kref_get_unless_zero(&region.ref))
    return region;
    }
    return core::ptr::null_mut();
    }
//
// dmem_cgroup_pool_state_put() - Drop a reference to a dmem_cgroup_pool_state
// @pool: &dmem_cgroup_pool_state
//
// Called to drop a reference to the limiting pool returned by
// dmem_cgroup_try_charge().
//
#[no_mangle]
pub unsafe extern "C" fn dmem_cgroup_pool_state_put(pool: *mut dmem_cgroup_pool_state) {
    if (pool) {
    css_put(&pool.cs.css);
    dmemcg_pool_put(pool);
    }
    }
    EXPORT_SYMBOL_GPL(dmem_cgroup_pool_state_put);
#[no_mangle]
pub unsafe extern "C" fn get_cg_pool_unlocked(cg: *mut dmemcg_state, region: *mut dmem_cgroup_region) -> *mut c_void {
    struct dmem_cgroup_pool_state *pool, *allocpool = core::ptr::null_mut();
// fastpath lookup?
    rcu_read_lock();
    pool = find_cg_pool_locked(cg, region);
    if (pool && !READ_ONCE(pool.inited)) {
    pool = core::ptr::null_mut();
    }
    if (pool && !dmemcg_pool_tryget(pool)) {
    pool = core::ptr::null_mut();
    }
    rcu_read_unlock();
    while (!pool) {
    spin_lock(&dmemcg_lock);
    if (!region.unregistered) {
    pool = get_cg_pool_locked(cg, region, &allocpool);
    }
    else {
    pool = ERR_PTR(-ENODEV);
    }
    if (!IS_ERR(pool)) {
    dmemcg_pool_get(pool);
    }
    spin_unlock(&dmemcg_lock);
    if (pool == ERR_PTR(-ENOMEM)) {
    pool = core::ptr::null_mut();
    if (WARN_ON!(allocpool)) {
    continue;
    }
    allocpool = kzalloc_obj(*allocpool);
    if (allocpool) {
    pool = core::ptr::null_mut();
    continue;
    }
    pool = ERR_PTR(-ENOMEM);
    }
    }
    kfree(allocpool);
    return pool;
    }
//
// dmem_cgroup_uncharge() - Uncharge a pool.
// @pool: Pool to uncharge.
// @size: Size to uncharge.
//
// Undoes the effects of dmem_cgroup_try_charge.
// Must be called with the returned pool as argument,
// and same @index and @size.
//
#[no_mangle]
pub unsafe extern "C" fn dmem_cgroup_uncharge(pool: *mut dmem_cgroup_pool_state, size: u64) {
    if (!pool) {
    return;
    }
    page_counter_uncharge(&pool.cnt, size);
    css_put(&pool.cs.css);
    dmemcg_pool_put(pool);
    }
    EXPORT_SYMBOL_GPL(dmem_cgroup_uncharge);
//
// dmem_cgroup_try_charge() - Try charging a new allocation to a region.
// @region: dmem region to charge
// @size: Size (in bytes) to charge.
// @ret_pool: On succesfull allocation, the pool that is charged.
// @ret_limit_pool: On a failed allocation, the limiting pool.
//
// This function charges the @region region for a size of @size bytes.
//
// If the function succeeds, @ret_pool is set, which must be passed to
// dmem_cgroup_uncharge() when undoing the allocation.
//
// When this function fails with -EAGAIN and @ret_limit_pool is non-null, it
// will be set to the pool for which the limit is hit. This can be used for
// eviction as argument to dmem_cgroup_evict_valuable(). This reference must be freed
// with @dmem_cgroup_pool_state_put().
//
// Return: 0 on success, -EAGAIN on hitting a limit, or a negative errno on failure.
//
#[no_mangle]
pub unsafe extern "C" fn dmem_cgroup_try_charge(region: *mut dmem_cgroup_region, size: u64, ret_pool: *mut *mut dmem_cgroup_pool_state, ret_limit_pool: *mut *mut dmem_cgroup_pool_state) -> c_int {
pub static mut cg: *mut c_void = core::ptr::null_mut();
pub static mut pool: *mut c_void = core::ptr::null_mut();
pub static mut fail: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
// ret_pool = NULL;
    if (ret_limit_pool) {
// ret_limit_pool = NULL;
    }
//
// hold on to css, as cgroup can be removed but resource
// accounting happens on css.
//
    cg = get_current_dmemcs();
    pool = get_cg_pool_unlocked(cg, region);
    if (IS_ERR(pool)) {
    ret = PTR_ERR(pool);
// goto;
    }
    if (!page_counter_try_charge(&pool.cnt, size, &fail)) {
    if (ret_limit_pool) {
// ret_limit_pool = container_of!(fail, dmem_cgroup_pool_state, cnt);
    css_get(&(*ret_limit_pool).cs.css);
    dmemcg_pool_get(*ret_limit_pool);
    }
    dmemcg_pool_put(pool);
    ret = -EAGAIN;
// goto;
    }
// On success, reference from get_current_dmemcs is transferred to *ret_pool
// ret_pool = pool;
    return 0;
// label;
    css_put(&cg.css);
    return ret;
    }
    EXPORT_SYMBOL_GPL(dmem_cgroup_try_charge);
//
// dmem_cgroup_below_min() - Tests whether current usage is within min limit.
//
// @root: Root of the subtree to calculate protection for, or NULL to calculate global protection.
// @test: The pool to test the usage/min limit of.
//
// Return: true if usage is below min and the cgroup is protected, false otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn dmem_cgroup_below_min(root: *mut dmem_cgroup_pool_state, test: *mut dmem_cgroup_pool_state) -> bool {
    if (root == test || !pool_parent(test)) {
    return false;
    }
    if (!root) {
    for (root = test; pool_parent(root); root = pool_parent(root)) {
    {}
    }
    }
//
// In mem_cgroup_below_min(), the memcg pendant, this call is missing.
// mem_cgroup_below_min() gets called during traversal of the cgroup tree, where
// protection is already calculated as part of the traversal. dmem cgroup eviction
// does not traverse the cgroup tree, so we need to recalculate effective protection
// here.
//
    dmem_cgroup_calculate_protection(root, test);
    return page_counter_read(&test.cnt) <= READ_ONCE(test.cnt.emin);
    }
    EXPORT_SYMBOL_GPL(dmem_cgroup_below_min);
//
// dmem_cgroup_below_low() - Tests whether current usage is within low limit.
//
// @root: Root of the subtree to calculate protection for, or NULL to calculate global protection.
// @test: The pool to test the usage/low limit of.
//
// Return: true if usage is below low and the cgroup is protected, false otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn dmem_cgroup_below_low(root: *mut dmem_cgroup_pool_state, test: *mut dmem_cgroup_pool_state) -> bool {
    if (root == test || !pool_parent(test)) {
    return false;
    }
    if (!root) {
    for (root = test; pool_parent(root); root = pool_parent(root)) {
    {}
    }
    }
//
// In mem_cgroup_below_low(), the memcg pendant, this call is missing.
// mem_cgroup_below_low() gets called during traversal of the cgroup tree, where
// protection is already calculated as part of the traversal. dmem cgroup eviction
// does not traverse the cgroup tree, so we need to recalculate effective protection
// here.
//
    dmem_cgroup_calculate_protection(root, test);
    return page_counter_read(&test.cnt) <= READ_ONCE(test.cnt.elow);
    }
    EXPORT_SYMBOL_GPL(dmem_cgroup_below_low);
//
// dmem_cgroup_get_common_ancestor(): Find the first common ancestor of two pools.
// @a: First pool to find the common ancestor of.
// @b: First pool to find the common ancestor of.
//
// Return: The first pool that is a parent of both @a and @b, or NULL if either @a or @b are NULL,
// or if such a pool does not exist. A reference to the returned pool is grabbed and must be
// released by the caller when it is done using the pool.
//
#[no_mangle]
pub unsafe extern "C" fn dmem_cgroup_get_common_ancestor(a: *mut dmem_cgroup_pool_state, b: *mut dmem_cgroup_pool_state) -> *mut c_void {
pub static mut ancestor_cgroup: *mut c_void = core::ptr::null_mut();
pub static mut ancestor_css: *mut c_void = core::ptr::null_mut();
    let mut ancestor_dmemcs = core::ptr::null_mut();
    let mut pool = core::ptr::null_mut();
    if (!a || !b) {
    return core::ptr::null_mut();
    }
    ancestor_cgroup = cgroup_common_ancestor(a.cs.css.cgroup, b.cs.css.cgroup);
    if (!ancestor_cgroup) {
    return core::ptr::null_mut();
    }
    rcu_read_lock();
    ancestor_css = cgroup_e_css(ancestor_cgroup, &dmem_cgrp_subsys);
    if (css_tryget(ancestor_css)) {
    ancestor_dmemcs = css_to_dmemcs(ancestor_css);
    }
    rcu_read_unlock();
    if (ancestor_dmemcs) {
    pool = get_cg_pool_unlocked(css_to_dmemcs(ancestor_css),
    a.region);
    if (WARN_ON!(IS_ERR(pool))) {
    pool = core::ptr::null_mut();
    css_put(ancestor_css);
    }
    }
    return pool;
    }
    EXPORT_SYMBOL_GPL(dmem_cgroup_get_common_ancestor);
#[no_mangle]
unsafe extern "C" fn dmem_cgroup_region_capacity_show(sf: *mut seq_file, v: *mut c_void) -> c_int {
pub static mut region: *mut c_void = core::ptr::null_mut();
    rcu_read_lock();
    list_for_each_entry_rcu(region, &dmem_cgroup_regions, region_node) {
    seq_puts(sf, region.name);
    seq_printf(sf, " %llu\n", region.size);
    }
    rcu_read_unlock();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dmemcg_parse_limit(options: *mut c_char, new_limit: *mut u64) -> c_int {
pub static mut end: *mut c_void = core::ptr::null_mut();
    if (!strcmp(options, "max")) {
// new_limit = PAGE_COUNTER_MAX;
    return 0;
    }
// new_limit = memparse(options, &end);
    if (*end != '\0') {
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn dmemcg_limit_write(of: *mut kernfs_open_file, buf: *mut c_char, nbytes: size_t, off: loff_t, u64: c_void) -> ssize_t {
    let mut dmemcs = css_to_dmemcs(of_css(of));
pub static mut pool: *mut c_void = core::ptr::null_mut();
pub static mut region: *mut c_void = core::ptr::null_mut();
pub static mut nonblock: bool = false;
pub static mut region_name: *mut c_void = core::ptr::null_mut();
    let mut new_limit = 0;
    let mut err = 0;
    buf = strstrip(buf);
    region_name = strsep(&buf, " \t");
    if (!buf || !region_name[0]) {
    return -EINVAL;
    }
    rcu_read_lock();
    region = dmemcg_get_region_by_name(region_name);
    rcu_read_unlock();
    if (!region) {
    return -EINVAL;
    }
    err = dmemcg_parse_limit(buf, &new_limit);
    if (err < 0) {
// goto;
    }
    pool = get_cg_pool_unlocked(dmemcs, region);
    if (IS_ERR(pool)) {
    err = PTR_ERR(pool);
// goto;
    }
    apply(pool, new_limit, nonblock);
    dmemcg_pool_put(pool);
// label;
    kref_put(&region.ref, dmemcg_free_region);
    return err ?: nbytes;
    }
#[no_mangle]
pub unsafe extern "C" fn dmemcg_limit_show(sf: *mut seq_file, v: *mut c_void) -> c_int {
    let mut dmemcs = css_to_dmemcs(seq_css(sf));
pub static mut region: *mut c_void = core::ptr::null_mut();
    rcu_read_lock();
    list_for_each_entry_rcu(region, &dmem_cgroup_regions, region_node) {
    let mut pool = find_cg_pool_locked(dmemcs, region);
    let mut val = 0;
    seq_puts(sf, region.name);
    val = fn(pool);
    if (val < PAGE_COUNTER_MAX) {
    seq_printf(sf, " %lld\n", val);
    }
    else {
    seq_puts(sf, " max\n");
    }
    }
    rcu_read_unlock();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dmem_cgroup_region_peak_show(sf: *mut seq_file, v: *mut c_void) -> c_int {
    return dmemcg_limit_show(sf, v, get_resource_peak);
    }
#[no_mangle]
unsafe extern "C" fn dmem_cgroup_region_current_show(sf: *mut seq_file, v: *mut c_void) -> c_int {
    return dmemcg_limit_show(sf, v, get_resource_current);
    }
#[no_mangle]
unsafe extern "C" fn dmem_cgroup_region_min_show(sf: *mut seq_file, v: *mut c_void) -> c_int {
    return dmemcg_limit_show(sf, v, get_resource_min);
    }
#[no_mangle]
pub unsafe extern "C" fn dmem_cgroup_region_min_write(of: *mut kernfs_open_file, buf: *mut c_char, nbytes: size_t, off: loff_t) -> ssize_t {
    return dmemcg_limit_write(of, buf, nbytes, off, set_resource_min);
    }
#[no_mangle]
unsafe extern "C" fn dmem_cgroup_region_low_show(sf: *mut seq_file, v: *mut c_void) -> c_int {
    return dmemcg_limit_show(sf, v, get_resource_low);
    }
#[no_mangle]
pub unsafe extern "C" fn dmem_cgroup_region_low_write(of: *mut kernfs_open_file, buf: *mut c_char, nbytes: size_t, off: loff_t) -> ssize_t {
    return dmemcg_limit_write(of, buf, nbytes, off, set_resource_low);
    }
#[no_mangle]
unsafe extern "C" fn dmem_cgroup_region_max_show(sf: *mut seq_file, v: *mut c_void) -> c_int {
    return dmemcg_limit_show(sf, v, get_resource_max);
    }
#[no_mangle]
pub unsafe extern "C" fn dmem_cgroup_region_max_write(of: *mut kernfs_open_file, buf: *mut c_char, nbytes: size_t, off: loff_t) -> ssize_t {
    return dmemcg_limit_write(of, buf, nbytes, off, set_resource_max);
    }
pub static mut cftype: usize = 0;
pub static mut cgroup_subsys: usize = 0;