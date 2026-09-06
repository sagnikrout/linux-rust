//! Automatically rewritten from C to Rust
//! Source: mm/damon/core.c
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
// Data Access Monitor
//

// for damon_get_folio() used by node eligible memory metrics

// Macro flag: #define CREATE_TRACE_POINTS

pub static mut damon_lock: usize = 0;
    static int nr_running_ctxs;
    static bool running_exclusive_ctxs;
pub static mut damon_ops_lock: usize = 0;
    static struct damon_operations damon_registered_ops[NR_DAMON_OPS];
pub static mut damon_region_cache: *mut c_void = core::ptr::null_mut();
// Should be called under damon_ops_lock with id smaller than NR_DAMON_OPS
#[no_mangle]
unsafe extern "C" fn __damon_is_registered_ops(id: damon_ops_id) -> bool {
pub static mut empty_ops: damon_operations = 0;
    if (!memcmp(&empty_ops, &damon_registered_ops[id], sizeof!(empty_ops))) {
    return false;
    }
    return true;
    }
//
// damon_is_registered_ops() - Check if a given damon_operations is registered.
// @id:	Id of the damon_operations to check if registered.
//
// Return: true if the ops is set, false otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn damon_is_registered_ops(id: damon_ops_id) -> bool {
    let mut registered = 0;
    if (id >= NR_DAMON_OPS) {
    return false;
    }
    mutex_lock(&damon_ops_lock);
    registered = __damon_is_registered_ops(id);
    mutex_unlock(&damon_ops_lock);
    return registered;
    }
//
// damon_register_ops() - Register a monitoring operations set to DAMON.
// @ops:	monitoring operations set to register.
//
// This function registers a monitoring operations set of valid &struct
// damon_operations->id so that others can find and use them later.
//
// Return: 0 on success, negative error code otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn damon_register_ops(ops: *mut damon_operations) -> c_int {
pub static mut err: c_int = 0;
    if (ops.id >= NR_DAMON_OPS) {
    return -EINVAL;
    }
    mutex_lock(&damon_ops_lock);
// Fail for already registered ops
    if (__damon_is_registered_ops(ops.id)) {
    err = -EINVAL;
    }
    else {
    damon_registered_ops[ops.id] = *ops;
    }
    mutex_unlock(&damon_ops_lock);
    return err;
    }
//
// damon_select_ops() - Select a monitoring operations to use with the context.
// @ctx:	monitoring context to use the operations.
// @id:		id of the registered monitoring operations to select.
//
// This function finds registered monitoring operations set of @id and make
// @ctx to use it.
//
// Return: 0 on success, negative error code otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn damon_select_ops(ctx: *mut damon_ctx, id: damon_ops_id) -> c_int {
pub static mut err: c_int = 0;
    if (id >= NR_DAMON_OPS) {
    return -EINVAL;
    }
    mutex_lock(&damon_ops_lock);
    if (!__damon_is_registered_ops(id)) {
    err = -EINVAL;
    }
    else {
    ctx.ops = damon_registered_ops[id];
    }
    mutex_unlock(&damon_ops_lock);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn damon_new_filter(type: damon_filter_type, matching: bool, allow: bool) -> *mut c_void {
pub static mut filter: *mut c_void = core::ptr::null_mut();
    filter = kmalloc_obj(*filter);
    if (!filter) {
    return core::ptr::null_mut();
    }
    filter.type = type;
    filter.matching = matching;
    filter.allow = allow;
    INIT_LIST_HEAD(&filter.list);
    return filter;
    }
#[no_mangle]
pub unsafe extern "C" fn damon_add_filter(p: *mut damon_probe, f: *mut damon_filter) {
    list_add_tail(&f.list, &p.filters);
    }
#[no_mangle]
unsafe extern "C" fn damon_del_filter(f: *mut damon_filter) {
    list_del(&f.list);
    }
#[no_mangle]
unsafe extern "C" fn damon_free_filter(f: *mut damon_filter) {
    kfree(f);
    }
#[no_mangle]
pub unsafe extern "C" fn damon_destroy_filter(f: *mut damon_filter) {
    damon_del_filter(f);
    damon_free_filter(f);
    }
#[no_mangle]
pub unsafe extern "C" fn damon_nth_filter(n: c_int, p: *mut damon_probe) -> *mut c_void {
pub static mut f: *mut c_void = core::ptr::null_mut();
pub static mut i: c_int = 0;
    damon_for_each_filter(f, p) {
    if (i++ == n) {
    return f;
    }
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn damon_new_probe() -> *mut c_void {
pub static mut p: *mut c_void = core::ptr::null_mut();
    p = kmalloc_obj(*p);
    if (!p) {
    return core::ptr::null_mut();
    }
    p.weight = 0;
    INIT_LIST_HEAD(&p.filters);
    INIT_LIST_HEAD(&p.list);
    return p;
    }
#[no_mangle]
pub unsafe extern "C" fn damon_add_probe(ctx: *mut damon_ctx, probe: *mut damon_probe) {
    list_add_tail(&probe.list, &ctx.probes);
    }
#[no_mangle]
unsafe extern "C" fn damon_del_probe(p: *mut damon_probe) {
    list_del(&p.list);
    }
#[no_mangle]
unsafe extern "C" fn damon_free_probe(p: *mut damon_probe) {
    let mut f = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
    damon_for_each_filter_safe(f, next, p)
    damon_free_filter(f);
    kfree(p);
    }
#[no_mangle]
unsafe extern "C" fn damon_destroy_probe(p: *mut damon_probe) {
    damon_del_probe(p);
    damon_free_probe(p);
    }
#[no_mangle]
pub unsafe extern "C" fn damon_nth_probe(n: c_int, ctx: *mut damon_ctx) -> *mut c_void {
pub static mut p: *mut c_void = core::ptr::null_mut();
pub static mut i: c_int = 0;
    damon_for_each_probe(p, ctx) {
    if (i++ == n) {
    return p;
    }
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn damon_has_probe_weights(c: *mut damon_ctx) -> bool {
pub static mut p: *mut c_void = core::ptr::null_mut();
    damon_for_each_probe(p, c) {
    if (p.weight) {
    return true;
    }
    }
    return false;
    }
//
// damon_mvsum() - Returns pseudo moving sum value for a time window.
// @current_nr:		The value of the current aggregation window.
// @last_nr:		The value of the last aggregation window.
// @left_window_bp:	Left time of the current aggregation window.
//
// This function calculates a pseudo moving sum value of a counter that is
// aggregated for each time window.  @current_nr is the value of the counter
// that aggregated so far (maybe not yet complete), from the beginning of the
// current aggregation time window.  @last_nr is the value of the counter that
// has completely aggregated in the last aggregation time window.
// @left_window_bp represents how much time is left for the current aggregation
// time window in bp (1/10,000).  For example, the aggregation time window is
// for every 10 seconds and 7 seconds has passed since the beginning of the
// current window, this parameter will be 3000 ((10 - 7) / 10 * 10000).
//
// The logic assumes the aggregation in the last phase was made in a single
// speed.  Based on the assumption, the value from the last window that needs
// to be added to the current value is calculated as a portion of the last
// value based on the remaining time window.
//
#[no_mangle]
pub unsafe extern "C" fn damon_mvsum(current_nr: c_ulong, last_nr: c_ulong, left_window_bp: c_ulong) -> c_ulong {
    return current_nr + mult_frac(last_nr, left_window_bp, 10000);
    }
//
// damon_nr_accesses_mvsum() - Returns moving sum access frequency score.
// @r:		Region to get the access frequency of.
// @ctx:	DAMON context of @r.
//
// This function returns for how many sampling iterations in the last
// aggregation interval (&damon_attrs->aggr_interval) the region was found to
// be accessed.  Hence the value can be interpreted as the relative access
// frequency score of the region (@r).  The value is calculated as a pseudo
// moving sum, and hence it is not an exact value but just a best-effort
// reasonable estimation.
//
// Return: the pseudo moving sum access frequency score.
//
#[no_mangle]
pub unsafe extern "C" fn damon_nr_accesses_mvsum(r: *mut damon_region, ctx: *mut damon_ctx) -> c_uint {
    unsigned long sample_interval, aggr_interval;
    unsigned long window_len, left_window, left_window_bp;
    sample_interval = ctx.attrs.sample_interval ? : 1;
    aggr_interval = ctx.attrs.aggr_interval ? : 1;
    window_len = aggr_interval / sample_interval;
    if (time_after_eq(ctx.passed_sample_intervals,
    ctx.next_aggregation_sis)) {
    left_window = 0;
    }
    else {
    left_window = ctx.next_aggregation_sis -
    ctx.passed_sample_intervals;
    }
    left_window_bp = mult_frac(left_window, 10000, window_len);
    if (left_window_bp == 10000) {
    return r.last_nr_accesses;
    }
    return damon_mvsum(r.nr_accesses, r.last_nr_accesses,
    left_window_bp);
    }
    unsigned char damon_probe_hits_mvsum(int probe_idx, damon_region *r, damon_ctx *ctx)
    {
    unsigned long sample_interval, aggr_interval;
    unsigned long window_len, left_window, left_window_bp;
    sample_interval = ctx.attrs.sample_interval ? : 1;
    aggr_interval =  ctx.attrs.aggr_interval ? : 1;
    window_len = aggr_interval / sample_interval;
    if (time_after_eq(ctx.passed_sample_intervals,
    ctx.next_aggregation_sis)) {
    left_window = 0;
    }
    else {
    left_window = ctx.next_aggregation_sis -
    ctx.passed_sample_intervals;
    }
    left_window_bp = mult_frac(left_window, 10000, window_len);
    if (left_window_bp == 10000) {
    return r.last_probe_hits[probe_idx];
    }
    return damon_mvsum(r.probe_hits[probe_idx],
    r.last_probe_hits[probe_idx], left_window_bp);
    }

#[no_mangle]
unsafe extern "C" fn damon_verify_new_region(start: c_ulong, end: c_ulong) {
    WARN_ONCE(start >= end, "start %lu >= end %lu\n", start, end);
    }

#[no_mangle]
unsafe extern "C" fn damon_verify_new_region(start: c_ulong, end: c_ulong) {
    }

//
// Construct a damon_region struct
//
// Returns the pointer to the new struct if success, or NULL otherwise
//
#[no_mangle]
pub unsafe extern "C" fn damon_new_region(start: c_ulong, end: c_ulong) -> *mut c_void {
pub static mut region: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    damon_verify_new_region(start, end);
    region = kmem_cache_alloc(damon_region_cache, GFP_KERNEL);
    if (!region) {
    return core::ptr::null_mut();
    }
    region.ar.start = start;
    region.ar.end = end;
    region.nr_accesses = 0;
    while (i < DAMON_MAX_PROBES) {
    region.probe_hits[i] = 0;
    region.last_probe_hits[i] = 0;
    }
    INIT_LIST_HEAD(&region.list);
    region.age = 0;
    region.last_nr_accesses = 0;
    return region;
    }
#[no_mangle]
unsafe extern "C" fn damon_add_region(r: *mut damon_region, t: *mut damon_target) {
    list_add_tail(&r.list, &t.regions_list);
    t.nr_regions += 1;
    }
//
// Add a region between two other regions
//
#[no_mangle]
pub unsafe extern "C" fn damon_insert_region(r: *mut damon_region, prev: *mut damon_region, next: *mut damon_region, t: *mut damon_target) {
    __list_add(&r.list, &prev.list, &next.list);
    t.nr_regions += 1;
    }

#[no_mangle]
unsafe extern "C" fn damon_verify_del_region(t: *mut damon_target) {
    WARN_ONCE(t.nr_regions == 0, "t.nr_regions == 0\n");
    }

#[no_mangle]
unsafe extern "C" fn damon_verify_del_region(t: *mut damon_target) {
    }

#[no_mangle]
unsafe extern "C" fn damon_del_region(r: *mut damon_region, t: *mut damon_target) {
    damon_verify_del_region(t);
    list_del(&r.list);
    t.nr_regions -= 1;
    }
#[no_mangle]
unsafe extern "C" fn damon_free_region(r: *mut damon_region) {
    kmem_cache_free(damon_region_cache, r);
    }
#[no_mangle]
pub unsafe extern "C" fn damon_destroy_region(r: *mut damon_region, t: *mut damon_target) {
    damon_del_region(r, t);
    damon_free_region(r);
    }
#[no_mangle]
pub unsafe extern "C" fn damon_is_last_region(r: *mut damon_region, t: *mut damon_target) -> bool {
    return list_is_last(&r.list, &t.regions_list);
    }
//
// damon_probe_hits_wsum() - Returns probe hits weighted sum of a region.
// @r:		region to get the weighted sum of.
// @last:	if the request is for last-window aggregated probe hits.
// @ctx:	context of &r.
//
// Return: the weighted sum of probe hits of the region.
//
#[no_mangle]
pub unsafe extern "C" fn damon_probe_hits_wsum(r: *mut damon_region, last: bool, ctx: *mut damon_ctx) -> c_uint {
pub static mut probe: *mut c_void = core::ptr::null_mut();
pub static mut sum: c_uint = 0;
pub static mut i: c_int = 0;
    damon_for_each_probe(probe, ctx) {
    if (last) {
    sum += r.last_probe_hits[i++] * probe.weight;
    }
    else {
    sum += r.probe_hits[i++] * probe.weight;
    }
    }
    return sum;
    }
//
// Check whether a region is intersecting an address range
//
// Returns true if it is.
//
#[no_mangle]
pub unsafe extern "C" fn damon_intersect(r: *mut damon_region, re: *mut damon_addr_range) -> bool {
    return !(r.ar.end <= re.start || re.end <= r.ar.start);
    }
//
// Fill holes in regions with new regions.
//
#[no_mangle]
pub unsafe extern "C" fn damon_fill_regions_holes(first: *mut damon_region, last: *mut damon_region, t: *mut damon_target) -> c_int {
    let mut r = first;
    damon_for_each_region_from(r, t) {
    let mut next = core::ptr::null_mut();
    let mut newr = core::ptr::null_mut();
    if (r == last) {
    break;
    }
    next = damon_next_region(r);
    if (r.ar.end != next.ar.start) {
    newr = damon_new_region(r.ar.end, next.ar.start);
    if (!newr) {
    return -ENOMEM;
    }
    damon_insert_region(newr, r, next, t);
    }
    }
    return 0;
    }
//
// damon_set_regions() - Set regions of a target for given address ranges.
// @t:		the given target.
// @ranges:	array of new monitoring target ranges.
// @nr_ranges:	length of @ranges.
// @min_region_sz:	minimum region size.
//
// This function adds new regions to, or modify existing regions of a
// monitoring target to fit in specific ranges.
//
// Return: 0 if success, or negative error code otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn damon_set_regions(t: *mut damon_target, ranges: *mut damon_addr_range, nr_ranges: c_uint, min_region_sz: c_ulong) -> c_int {
    let mut r = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
    let mut i = 0;
    let mut last_end = 0;
    let mut err = 0;
    while (i < nr_ranges) {
    unsigned long start, end;
    start = ALIGN_DOWN(ranges[i].start, min_region_sz);
    end = ALIGN(ranges[i].end, min_region_sz);
    if (start >= end) {
    return -EINVAL;
    }
    if (i > 0 && last_end > start) {
    return -EINVAL;
    }
    last_end = end;
    }
// Remove regions which are not in the new ranges
    damon_for_each_region_safe(r, next, t) {
    while (i < nr_ranges) {
    if (damon_intersect(r, &ranges[i])) {
    break;
    }
    }
    if (i == nr_ranges) {
    damon_destroy_region(r, t);
    }
    }
    if (!damon_nr_regions(t)) {
    while (i < nr_ranges) {
    r = damon_new_region(
    ALIGN_DOWN(ranges[i].start,
    min_region_sz),
    ALIGN(ranges[i].end, min_region_sz));
    if (!r) {
    return -ENOMEM;
    }
    damon_add_region(r, t);
    }
    return 0;
    }
    r = damon_first_region(t);
// Add new regions or resize existing regions to fit in the ranges
    while (i < nr_ranges) {
    let mut first = core::ptr::null_mut(), *last, *newr;
pub static mut range: *mut c_void = core::ptr::null_mut();
pub static mut insert_before_r: bool = false;
    range = &ranges[i];
// Get the first/last regions intersecting with the range
    damon_for_each_region_from(r, t) {
    if (damon_intersect(r, range)) {
    if (!first) {
    first = r;
    }
    last = r;
    }
    if (r.ar.start >= range.end) {
    insert_before_r = true;
    break;
    }
    }
    if (!first) {
// no region intersects with this range
    newr = damon_new_region(
    ALIGN_DOWN(range.start,
    min_region_sz),
    ALIGN(range.end, min_region_sz));
    if (!newr) {
    return -ENOMEM;
    }
    if (insert_before_r) {
    damon_insert_region(newr, damon_prev_region(r),
    r, t);
    }
    else {
    damon_add_region(newr, t);
    }
    } else {
// resize intersecting regions to fit in this range
    first.ar.start = ALIGN_DOWN(range.start,
    min_region_sz);
    last.ar.end = ALIGN(range.end, min_region_sz);
// fill possible holes in the range
    err = damon_fill_regions_holes(first, last, t);
    if (err) {
    return err;
    }
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn damos_new_filter(type: damos_filter_type, matching: bool, allow: bool) -> *mut c_void {
pub static mut filter: *mut c_void = core::ptr::null_mut();
    filter = kmalloc_obj(*filter);
    if (!filter) {
    return core::ptr::null_mut();
    }
    filter.type = type;
    filter.matching = matching;
    filter.allow = allow;
    INIT_LIST_HEAD(&filter.list);
    return filter;
    }
//
// damos_filter_for_ops() - Return if the filter is ops-handled one.
// @type:	type of the filter.
//
// Return: true if the filter of @type needs to be handled by ops layer, false
// otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn damos_filter_for_ops(type: damos_filter_type) -> bool {
    match (type) {
    DAMOS_FILTER_TYPE_ADDR => {
    }
    DAMOS_FILTER_TYPE_TARGET => {
    return false;
    }
    _ => {
    // break;
    }
    }
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn damos_add_filter(s: *mut damos, f: *mut damos_filter) {
    if (damos_filter_for_ops(f.type)) {
    list_add_tail(&f.list, &s.ops_filters);
    }
    else {
    list_add_tail(&f.list, &s.core_filters);
    }
    }
#[no_mangle]
unsafe extern "C" fn damos_del_filter(f: *mut damos_filter) {
    list_del(&f.list);
    }
#[no_mangle]
unsafe extern "C" fn damos_free_filter(f: *mut damos_filter) {
    kfree(f);
    }
#[no_mangle]
pub unsafe extern "C" fn damos_destroy_filter(f: *mut damos_filter) {
    damos_del_filter(f);
    damos_free_filter(f);
    }
#[no_mangle]
pub unsafe extern "C" fn damos_new_quota_goal(metric: damos_quota_goal_metric, target_value: c_ulong) -> *mut c_void {
pub static mut goal: *mut c_void = core::ptr::null_mut();
    goal = kmalloc_obj(*goal);
    if (!goal) {
    return core::ptr::null_mut();
    }
    goal.metric = metric;
    goal.target_value = target_value;
    INIT_LIST_HEAD(&goal.list);
    return goal;
    }
#[no_mangle]
pub unsafe extern "C" fn damos_add_quota_goal(q: *mut damos_quota, g: *mut damos_quota_goal) {
    list_add_tail(&g.list, &q.goals);
    }
#[no_mangle]
unsafe extern "C" fn damos_del_quota_goal(g: *mut damos_quota_goal) {
    list_del(&g.list);
    }
#[no_mangle]
unsafe extern "C" fn damos_free_quota_goal(g: *mut damos_quota_goal) {
    kfree(g);
    }
#[no_mangle]
pub unsafe extern "C" fn damos_destroy_quota_goal(g: *mut damos_quota_goal) {
    damos_del_quota_goal(g);
    damos_free_quota_goal(g);
    }
#[no_mangle]
unsafe extern "C" fn damos_quota_goals_empty(q: *mut damos_quota) -> bool {
    return list_empty(&q.goals);
    }
// initialize fields of @quota that normally API users wouldn't set
#[no_mangle]
pub unsafe extern "C" fn damos_quota_init(quota: *mut damos_quota) -> *mut c_void {
    quota.esz = 0;
    quota.total_charged_sz = 0;
    quota.total_charged_ns = 0;
    quota.charged_sz = 0;
    quota.charged_from = 0;
    quota.charge_target_from = core::ptr::null_mut();
    quota.charge_addr_from = 0;
    quota.esz_bp = 0;
    return quota;
    }
#[no_mangle]
pub unsafe extern "C" fn damon_new_scheme(pattern: *mut damos_access_pattern, action: damos_action, apply_interval_us: c_ulong, quota: *mut damos_quota, wmarks: *mut damos_watermarks, target_nid: c_int) -> *mut c_void {
pub static mut scheme: *mut c_void = core::ptr::null_mut();
    scheme = kmalloc_obj(*scheme);
    if (!scheme) {
    return core::ptr::null_mut();
    }
    scheme.pattern = *pattern;
    scheme.action = action;
    scheme.apply_interval_us = apply_interval_us;
//
// next_apply_sis will be set when kdamond starts.  While kdamond is
// running, it will also updated when it is added to the DAMON context,
// or damon_attrs are updated.
//
    scheme.next_apply_sis = 0;
    scheme.walk_completed = false;
    INIT_LIST_HEAD(&scheme.core_filters);
    INIT_LIST_HEAD(&scheme.ops_filters);
    scheme.stat = (damos_stat){};
    scheme.max_nr_snapshots = 0;
    scheme.last_applied = core::ptr::null_mut();
    INIT_LIST_HEAD(&scheme.list);
    scheme.quota = *(damos_quota_init(quota));
// quota.goals should be separately set by caller
    INIT_LIST_HEAD(&scheme.quota.goals);
    scheme.wmarks = *wmarks;
    scheme.wmarks.activated = true;
    scheme.migrate_dests = (damos_migrate_dests){};
    scheme.target_nid = target_nid;
    return scheme;
    }
#[no_mangle]
unsafe extern "C" fn damos_set_next_apply_sis(s: *mut damos, ctx: *mut damon_ctx) {
    let mut sample_interval = ctx.attrs.sample_interval ?
    ctx.attrs.sample_interval : 1;
    let mut apply_interval = s.apply_interval_us ?
    s.apply_interval_us : ctx.attrs.aggr_interval;
    s.next_apply_sis = ctx.passed_sample_intervals +
    apply_interval / sample_interval;
    }
#[no_mangle]
pub unsafe extern "C" fn damon_add_scheme(ctx: *mut damon_ctx, s: *mut damos) {
    list_add_tail(&s.list, &ctx.schemes);
    damos_set_next_apply_sis(s, ctx);
    }
#[no_mangle]
unsafe extern "C" fn damon_del_scheme(s: *mut damos) {
    list_del(&s.list);
    }
#[no_mangle]
unsafe extern "C" fn damon_free_scheme(s: *mut damos) {
    kfree(s);
    }
#[no_mangle]
pub unsafe extern "C" fn damon_destroy_scheme(s: *mut damos) {
    let mut g = core::ptr::null_mut();
    let mut g_next = core::ptr::null_mut();
    let mut f = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
    damos_for_each_quota_goal_safe(g, g_next, &s.quota)
    damos_destroy_quota_goal(g);
    damos_for_each_core_filter_safe(f, next, s)
    damos_destroy_filter(f);
    damos_for_each_ops_filter_safe(f, next, s)
    damos_destroy_filter(f);
    kfree(s.migrate_dests.node_id_arr);
    kfree(s.migrate_dests.weight_arr);
    damon_del_scheme(s);
    damon_free_scheme(s);
    }
//
// Construct a damon_target struct
//
// Returns the pointer to the new struct if success, or NULL otherwise
//
#[no_mangle]
pub unsafe extern "C" fn damon_new_target() -> *mut c_void {
pub static mut t: *mut c_void = core::ptr::null_mut();
    t = kmalloc_obj(*t);
    if (!t) {
    return core::ptr::null_mut();
    }
    t.pid = core::ptr::null_mut();
    t.nr_regions = 0;
    INIT_LIST_HEAD(&t.regions_list);
    INIT_LIST_HEAD(&t.list);
    t.obsolete = false;
    return t;
    }
#[no_mangle]
pub unsafe extern "C" fn damon_add_target(ctx: *mut damon_ctx, t: *mut damon_target) {
    list_add_tail(&t.list, &ctx.adaptive_targets);
    }
#[no_mangle]
pub unsafe extern "C" fn damon_targets_empty(ctx: *mut damon_ctx) -> bool {
    return list_empty(&ctx.adaptive_targets);
    }
#[no_mangle]
unsafe extern "C" fn damon_del_target(t: *mut damon_target) {
    list_del(&t.list);
    }
#[no_mangle]
pub unsafe extern "C" fn damon_free_target(t: *mut damon_target) {
    let mut r = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
    damon_for_each_region_safe(r, next, t)
    damon_free_region(r);
    kfree(t);
    }
#[no_mangle]
pub unsafe extern "C" fn damon_destroy_target(t: *mut damon_target, ctx: *mut damon_ctx) {
    if (ctx && ctx.ops.cleanup_target) {
    ctx.ops.cleanup_target(t);
    }
    damon_del_target(t);
    damon_free_target(t);
    }
#[no_mangle]
pub unsafe extern "C" fn damon_nr_regions(t: *mut damon_target) -> c_uint {
    return t.nr_regions;
    }
#[no_mangle]
pub unsafe extern "C" fn damon_new_ctx() -> *mut c_void {
pub static mut ctx: *mut c_void = core::ptr::null_mut();
    ctx = kzalloc_obj(*ctx);
    if (!ctx) {
    return core::ptr::null_mut();
    }
    init_completion(&ctx.kdamond_started);
    ctx.attrs.sample_interval = 5 * 1000;
    ctx.attrs.aggr_interval = 100 * 1000;
    ctx.attrs.ops_update_interval = 60 * 1000 * 1000;
    ctx.passed_sample_intervals = 0;
// These will be set from kdamond_init_ctx()
    ctx.next_aggregation_sis = 0;
    ctx.next_ops_update_sis = 0;
    mutex_init(&ctx.kdamond_lock);
    INIT_LIST_HEAD(&ctx.call_controls);
    mutex_init(&ctx.call_controls_lock);
    mutex_init(&ctx.walk_control_lock);
    ctx.attrs.min_nr_regions = 10;
    ctx.attrs.max_nr_regions = 1000;
    INIT_LIST_HEAD(&ctx.probes);
    ctx.addr_unit = 1;
    ctx.min_region_sz = DAMON_MIN_REGION_SZ;
    INIT_LIST_HEAD(&ctx.adaptive_targets);
    INIT_LIST_HEAD(&ctx.schemes);
    prandom_seed_state(&ctx.rnd_state, get_random_u64());
    return ctx;
    }
#[no_mangle]
unsafe extern "C" fn damon_destroy_targets(ctx: *mut damon_ctx) {
    let mut t = core::ptr::null_mut();
    let mut next_t = core::ptr::null_mut();
    damon_for_each_target_safe(t, next_t, ctx)
    damon_destroy_target(t, ctx);
    }
#[no_mangle]
pub unsafe extern "C" fn damon_destroy_ctx(ctx: *mut damon_ctx) {
    let mut s = core::ptr::null_mut();
    let mut next_s = core::ptr::null_mut();
    let mut p = core::ptr::null_mut();
    let mut next_p = core::ptr::null_mut();
    damon_destroy_targets(ctx);
    damon_for_each_scheme_safe(s, next_s, ctx)
    damon_destroy_scheme(s);
    damon_for_each_probe_safe(p, next_p, ctx)
    damon_destroy_probe(p);
    kfree(ctx);
    }
#[no_mangle]
pub unsafe extern "C" fn damon_attrs_equals(attrs1: *mut damon_attrs, attrs2: *mut damon_attrs) -> bool {
    let mut ig1 = &attrs1.intervals_goal;
    let mut ig2 = &attrs2.intervals_goal;
    return attrs1.sample_interval == attrs2.sample_interval &&
    attrs1.aggr_interval == attrs2.aggr_interval &&
    attrs1.ops_update_interval == attrs2.ops_update_interval &&
    attrs1.min_nr_regions == attrs2.min_nr_regions &&
    attrs1.max_nr_regions == attrs2.max_nr_regions &&
    ig1.access_bp == ig2.access_bp &&
    ig1.aggrs == ig2.aggrs &&
    ig1.min_sample_us == ig2.min_sample_us &&
    ig1.max_sample_us == ig2.max_sample_us;
    }
#[no_mangle]
pub unsafe extern "C" fn damon_age_for_new_attrs(age: c_uint, old_attrs: *mut damon_attrs, new_attrs: *mut damon_attrs) -> c_uint {
    return age * old_attrs.aggr_interval / new_attrs.aggr_interval;
    }
// convert sample ratio in bp (per 10,000) to count
#[no_mangle]
pub unsafe extern "C" fn damon_sample_bp_to_count(bp: c_uint, attrs: *mut damon_attrs) -> c_uint {
    return bp * damon_nr_samples_per_aggr(attrs) / 10000;
    }
// convert sample count to ratio in bp (per 10,000)
#[no_mangle]
pub unsafe extern "C" fn damon_sample_count_to_bp(count: c_uint, attrs: *mut damon_attrs) -> c_uint {
    return mult_frac(count, 10000, damon_nr_samples_per_aggr(attrs));
    }
#[no_mangle]
pub unsafe extern "C" fn damon_nr_samples_for_new_attrs(nr: c_uint, old_attrs: *mut damon_attrs, new_attrs: *mut damon_attrs) -> c_uint {
    return damon_sample_bp_to_count(
    damon_sample_count_to_bp(nr, old_attrs), new_attrs);
    }
#[no_mangle]
pub unsafe extern "C" fn damon_update_probe_hits(r: *mut damon_region, old_attrs: *mut damon_attrs, new_attrs: *mut damon_attrs, aggregating: bool, ctx: *mut damon_ctx) {
pub static mut p: *mut c_void = core::ptr::null_mut();
pub static mut i: c_int = 0;
    damon_for_each_probe(p, ctx) {
    r.last_probe_hits[i] = damon_nr_samples_for_new_attrs(
    r.last_probe_hits[i], old_attrs, new_attrs);
    if (!aggregating) {
    r.probe_hits[i] = damon_nr_samples_for_new_attrs(
    r.probe_hits[i], old_attrs,
    new_attrs);
    }
    else {
    r.probe_hits[i] = 0;
    }
    i += 1;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn damon_update_monitoring_result(r: *mut damon_region, old_attrs: *mut damon_attrs, new_attrs: *mut damon_attrs, aggregating: bool, ctx: *mut damon_ctx) {
    damon_update_probe_hits(r, old_attrs, new_attrs, aggregating, ctx);
    r.last_nr_accesses = damon_nr_samples_for_new_attrs(
    r.last_nr_accesses, old_attrs, new_attrs);
    if (!aggregating) {
    r.nr_accesses = damon_nr_samples_for_new_attrs(
    r.nr_accesses, old_attrs, new_attrs);
    }
    else {
//
// if this is called in the middle of the aggregation, reset
// the aggregations we made so far for this aggregation
// interval.  In other words, make the status like
// kdamond_reset_aggregated() is called.
//
    r.nr_accesses = 0;
    }
    r.age = damon_age_for_new_attrs(r.age, old_attrs, new_attrs);
    }
//
// region->nr_accesses is the number of sampling intervals in the last
// aggregation interval that access to the region has found, and region->age is
// the number of aggregation intervals that its access pattern has maintained.
// For the reason, the real meaning of the two fields depend on current
// sampling interval and aggregation interval.  This function updates
// ->nr_accesses and ->age of given damon_ctx's regions for new damon_attrs.
//
#[no_mangle]
pub unsafe extern "C" fn damon_update_monitoring_results(ctx: *mut damon_ctx, new_attrs: *mut damon_attrs, aggregating: bool) {
    let mut old_attrs = &ctx.attrs;
pub static mut t: *mut c_void = core::ptr::null_mut();
pub static mut r: *mut c_void = core::ptr::null_mut();
// if any interval is zero, simply forgive conversion
    if (!old_attrs.sample_interval || !old_attrs.aggr_interval ||
    !new_attrs.sample_interval ||
    !new_attrs.aggr_interval) {
    return;
    }
    damon_for_each_target(t, ctx)
    damon_for_each_region(r, t)
    damon_update_monitoring_result(r, old_attrs, new_attrs,
    aggregating, ctx);
    }
//
// damon_valid_intervals_goal() - return if the intervals goal of @attrs is
// valid.
//
#[no_mangle]
unsafe extern "C" fn damon_valid_intervals_goal(attrs: *mut damon_attrs) -> bool {
    let mut goal = &attrs.intervals_goal;
// tuning is disabled
    if (!goal.aggrs) {
    return true;
    }
    if (goal.min_sample_us > goal.max_sample_us) {
    return false;
    }
    if (attrs.sample_interval < goal.min_sample_us ||
    goal.max_sample_us < attrs.sample_interval) {
    return false;
    }
    return true;
    }
//
// damon_set_attrs() - Set attributes for the monitoring.
// @ctx:		monitoring context
// @attrs:		monitoring attributes
//
// This function updates monitoring results and next monitoring/damos operation
// schedules.  Because those are periodically updated by kdamond, this should
// be called from a safe contexts.  Such contexts include damon_ctx setup time
// while the kdamond is not yet started, and inside of kdamond_fn().
//
// In detail, all DAMON API callers directly call this function for initial
// setup of damon_ctx before calling damon_start().  Some of the API callers
// also indirectly call this function via damon_call() -> damon_commit() for
// online parameters updates.  Finally, kdamond_fn() itself use this for
// applying auto-tuned monitoring intervals.
//
// Every time interval is in micro-seconds.
//
// Return: 0 on success, negative error code otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn damon_set_attrs(ctx: *mut damon_ctx, attrs: *mut damon_attrs) -> c_int {
    let mut sample_interval = attrs.sample_interval ?
    attrs.sample_interval : 1;
pub static mut s: *mut c_void = core::ptr::null_mut();
    let mut aggregating = ctx.passed_sample_intervals <
    ctx.next_aggregation_sis;
    if (!damon_valid_intervals_goal(attrs)) {
    return -EINVAL;
    }
    if (attrs.min_nr_regions < 3) {
    return -EINVAL;
    }
    if (attrs.min_nr_regions > attrs.max_nr_regions) {
    return -EINVAL;
    }
    if (attrs.sample_interval > attrs.aggr_interval) {
    return -EINVAL;
    }
// calls from core-external doesn't set this.
    if (!attrs.aggr_samples) {
    attrs.aggr_samples = attrs.aggr_interval / sample_interval;
    }
    ctx.next_aggregation_sis = ctx.passed_sample_intervals +
    attrs.aggr_interval / sample_interval;
    ctx.next_ops_update_sis = ctx.passed_sample_intervals +
    attrs.ops_update_interval / sample_interval;
//
// next_intervals_tune_sis will be updated inside kdamond_fn().
//
    damon_update_monitoring_results(ctx, attrs, aggregating);
    ctx.attrs = *attrs;
    damon_for_each_scheme(s, ctx)
    damos_set_next_apply_sis(s, ctx);
    return 0;
    }
//
// damon_set_schemes() - Set data access monitoring based operation schemes.
// @ctx:	monitoring context
// @schemes:	array of the schemes
// @nr_schemes:	number of entries in @schemes
//
// This function should not be called while the kdamond of the context is
// running.
//
#[no_mangle]
pub unsafe extern "C" fn damon_set_schemes(ctx: *mut damon_ctx, schemes: *mut *mut damos, nr_schemes: ssize_t) {
    let mut s = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
    let mut i = 0;
    damon_for_each_scheme_safe(s, next, ctx)
    damon_destroy_scheme(s);
    for (i = 0; i < nr_schemes; i++) {
    damon_add_scheme(ctx, schemes[i]);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn damos_nth_quota_goal(n: c_int, q: *mut damos_quota) -> *mut c_void {
pub static mut goal: *mut c_void = core::ptr::null_mut();
pub static mut i: c_int = 0;
    damos_for_each_quota_goal(goal, q) {
    if (i++ == n) {
    return goal;
    }
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn damos_commit_quota_goal_union(dst: *mut damos_quota_goal, src: *mut damos_quota_goal) {
    match (dst.metric) {
    DAMOS_QUOTA_NODE_MEM_USED_BP => {
    }
    DAMOS_QUOTA_NODE_MEM_FREE_BP => {
    dst.nid = src.nid;
    // break;
    }
    DAMOS_QUOTA_NODE_MEMCG_USED_BP => {
    }
    DAMOS_QUOTA_NODE_MEMCG_FREE_BP => {
    dst.nid = src.nid;
    dst.memcg_id = src.memcg_id;
    // break;
    }
    _ => {
    // break;
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn damos_commit_quota_goal(dst: *mut damos_quota_goal, src: *mut damos_quota_goal) {
    dst.metric = src.metric;
    dst.target_value = src.target_value;
    if (dst.metric == DAMOS_QUOTA_USER_INPUT) {
    dst.current_value = src.current_value;
    }
// keep last_psi_total as is, since it will be updated in next cycle
    damos_commit_quota_goal_union(dst, src);
    }
//
// damos_commit_quota_goals() - Commit DAMOS quota goals to another quota.
// @dst:	The commit destination DAMOS quota.
// @src:	The commit source DAMOS quota.
//
// Copies user-specified parameters for quota goals from @src to @dst.  Users
// should use this function for quota goals-level parameters update of running
// DAMON contexts, instead of manual in-place updates.
//
// This function should be called from parameters-update safe context, like
// damon_call().
//
#[no_mangle]
pub unsafe extern "C" fn damos_commit_quota_goals(dst: *mut damos_quota, src: *mut damos_quota) -> c_int {
    let mut dst_goal = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
    let mut src_goal = core::ptr::null_mut();
    let mut new_goal = core::ptr::null_mut();
pub static mut i: c_int = 0;
    damos_for_each_quota_goal_safe(dst_goal, next, dst) {
    src_goal = damos_nth_quota_goal(i++, src);
    if (src_goal) {
    damos_commit_quota_goal(dst_goal, src_goal);
    }
    else {
    damos_destroy_quota_goal(dst_goal);
    }
    }
    damos_for_each_quota_goal_safe(src_goal, next, src) {
    if (j++ < i) {
    continue;
    }
    new_goal = damos_new_quota_goal(
    src_goal.metric, src_goal.target_value);
    if (!new_goal) {
    return -ENOMEM;
    }
    damos_commit_quota_goal(new_goal, src_goal);
    damos_add_quota_goal(dst, new_goal);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn damos_commit_quota(dst: *mut damos_quota, src: *mut damos_quota) -> c_int {
    let mut err = 0;
    dst.reset_interval = src.reset_interval;
    dst.ms = src.ms;
    dst.sz = src.sz;
    err = damos_commit_quota_goals(dst, src);
    if (err) {
    return err;
    }
    dst.goal_tuner = src.goal_tuner;
    dst.fail_charge_num = src.fail_charge_num;
    dst.fail_charge_denom = src.fail_charge_denom;
    dst.weight_sz = src.weight_sz;
    dst.weight_nr_accesses = src.weight_nr_accesses;
    dst.weight_age = src.weight_age;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn damos_nth_core_filter(n: c_int, s: *mut damos) -> *mut c_void {
pub static mut filter: *mut c_void = core::ptr::null_mut();
pub static mut i: c_int = 0;
    damos_for_each_core_filter(filter, s) {
    if (i++ == n) {
    return filter;
    }
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn damos_nth_ops_filter(n: c_int, s: *mut damos) -> *mut c_void {
pub static mut filter: *mut c_void = core::ptr::null_mut();
pub static mut i: c_int = 0;
    damos_for_each_ops_filter(filter, s) {
    if (i++ == n) {
    return filter;
    }
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn damos_commit_filter_arg(dst: *mut damos_filter, src: *mut damos_filter) {
    match (dst.type) {
    DAMOS_FILTER_TYPE_MEMCG => {
    dst.memcg_id = src.memcg_id;
    // break;
    }
    DAMOS_FILTER_TYPE_ADDR => {
    dst.addr_range = src.addr_range;
    // break;
    }
    DAMOS_FILTER_TYPE_TARGET => {
    dst.target_idx = src.target_idx;
    // break;
    }
    DAMOS_FILTER_TYPE_HUGEPAGE_SIZE => {
    dst.sz_range = src.sz_range;
    // break;
    }
    _ => {
    // break;
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn damos_commit_filter(dst: *mut damos_filter, src: *mut damos_filter) {
    dst.type = src.type;
    dst.matching = src.matching;
    dst.allow = src.allow;
    damos_commit_filter_arg(dst, src);
    }
#[no_mangle]
unsafe extern "C" fn damos_commit_core_filters(dst: *mut damos, src: *mut damos) -> c_int {
    let mut dst_filter = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
    let mut src_filter = core::ptr::null_mut();
    let mut new_filter = core::ptr::null_mut();
pub static mut i: c_int = 0;
    damos_for_each_core_filter_safe(dst_filter, next, dst) {
    src_filter = damos_nth_core_filter(i++, src);
    if (src_filter) {
    damos_commit_filter(dst_filter, src_filter);
    }
    else {
    damos_destroy_filter(dst_filter);
    }
    }
    damos_for_each_core_filter_safe(src_filter, next, src) {
    if (j++ < i) {
    continue;
    }
    new_filter = damos_new_filter(
    src_filter.type, src_filter.matching,
    src_filter.allow);
    if (!new_filter) {
    return -ENOMEM;
    }
    damos_commit_filter_arg(new_filter, src_filter);
    damos_add_filter(dst, new_filter);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn damos_commit_ops_filters(dst: *mut damos, src: *mut damos) -> c_int {
    let mut dst_filter = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
    let mut src_filter = core::ptr::null_mut();
    let mut new_filter = core::ptr::null_mut();
pub static mut i: c_int = 0;
    damos_for_each_ops_filter_safe(dst_filter, next, dst) {
    src_filter = damos_nth_ops_filter(i++, src);
    if (src_filter) {
    damos_commit_filter(dst_filter, src_filter);
    }
    else {
    damos_destroy_filter(dst_filter);
    }
    }
    damos_for_each_ops_filter_safe(src_filter, next, src) {
    if (j++ < i) {
    continue;
    }
    new_filter = damos_new_filter(
    src_filter.type, src_filter.matching,
    src_filter.allow);
    if (!new_filter) {
    return -ENOMEM;
    }
    damos_commit_filter_arg(new_filter, src_filter);
    damos_add_filter(dst, new_filter);
    }
    return 0;
    }
//
// damos_filters_default_reject() - decide whether to reject memory that didn't
// match with any given filter.
// @filters:	Given DAMOS filters of a group.
//
#[no_mangle]
unsafe extern "C" fn damos_filters_default_reject(filters: *mut list_head) -> bool {
pub static mut last_filter: *mut c_void = core::ptr::null_mut();
    if (list_empty(filters)) {
    return false;
    }
    last_filter = list_last_entry(filters, damos_filter, list);
    return last_filter.allow;
    }
#[no_mangle]
unsafe extern "C" fn damos_set_filters_default_reject(s: *mut damos) {
    if (!list_empty(&s.ops_filters)) {
    s.core_filters_default_reject = false;
    }
    else {
    s.core_filters_default_reject =
    damos_filters_default_reject(&s.core_filters);
    }
    s.ops_filters_default_reject =
    damos_filters_default_reject(&s.ops_filters);
    }
#[no_mangle]
unsafe extern "C" fn damon_valid_probe_params(ctx: *mut damon_ctx) -> bool {
    let mut sample_interval = 0;
    let mut max_probe_hits = 0;
pub static mut probe: *mut c_void = core::ptr::null_mut();
    let mut wsum = 0;
    let mut wsum_to_add = 0;
    if (!damon_has_probe_weights(ctx)) {
    return true;
    }
    sample_interval = ctx.attrs.sample_interval ? : 1;
    if (ctx.attrs.aggr_interval / sample_interval > U8_MAX) {
    return false;
    }
// invalid if probe hits weighted sum can overflow
    max_probe_hits = damon_nr_samples_per_aggr(&ctx.attrs);
    wsum = 0;
    damon_for_each_probe(probe, ctx) {
    if (probe.weight > UINT_MAX / max_probe_hits) {
    return false;
    }
    wsum_to_add = probe.weight * max_probe_hits;
    if (UINT_MAX - wsum < wsum_to_add) {
    return false;
    }
    wsum += wsum_to_add;
    }
    return true;
    }
//
// damos_commit_dests() - Copy migration destinations from @src to @dst.
// @dst:	Destination structure to update.
// @src:	Source structure to copy from.
//
// If the number of destinations has changed, the old arrays in @dst are freed
// and new ones are allocated.  On success, @dst contains a full copy of
// @src's arrays and count.
//
// On allocation failure, @dst is left in a partially torn-down state: its
// arrays may be NULL and @nr_dests may not reflect the actual allocation
// sizes.  The structure remains safe to deallocate via damon_destroy_scheme(),
// but callers must not reuse @dst for further commits — it should be
// discarded.
//
// Return: 0 on success, -ENOMEM on allocation failure.
//
#[no_mangle]
pub unsafe extern "C" fn damos_commit_dests(dst: *mut damos_migrate_dests, src: *mut damos_migrate_dests) -> c_int {
    if (dst.nr_dests != src.nr_dests) {
    kfree(dst.node_id_arr);
    kfree(dst.weight_arr);
    dst.node_id_arr = kmalloc_array(src.nr_dests,
    sizeof!(*dst.node_id_arr), GFP_KERNEL);
    if (!dst.node_id_arr) {
    dst.weight_arr = core::ptr::null_mut();
    return -ENOMEM;
    }
    dst.weight_arr = kmalloc_array(src.nr_dests,
    sizeof!(*dst.weight_arr), GFP_KERNEL);
    if (!dst.weight_arr) {
// ->node_id_arr will be freed by scheme destruction
    return -ENOMEM;
    }
    }
    dst.nr_dests = src.nr_dests;
    while (i < src.nr_dests) {
    dst.node_id_arr[i] = src.node_id_arr[i];
    dst.weight_arr[i] = src.weight_arr[i];
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn damos_commit_filters(dst: *mut damos, src: *mut damos) -> c_int {
    let mut err = 0;
    err = damos_commit_core_filters(dst, src);
    if (err) {
    return err;
    }
    err = damos_commit_ops_filters(dst, src);
    if (err) {
    return err;
    }
    damos_set_filters_default_reject(dst);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn damon_nth_scheme(n: c_int, ctx: *mut damon_ctx) -> *mut c_void {
pub static mut s: *mut c_void = core::ptr::null_mut();
pub static mut i: c_int = 0;
    damon_for_each_scheme(s, ctx) {
    if (i++ == n) {
    return s;
    }
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn damos_commit(dst: *mut damos, src: *mut damos) -> c_int {
    let mut err = 0;
    dst.pattern = src.pattern;
    dst.action = src.action;
    dst.apply_interval_us = src.apply_interval_us;
    err = damos_commit_quota(&dst.quota, &src.quota);
    if (err) {
    return err;
    }
    dst.wmarks = src.wmarks;
    dst.target_nid = src.target_nid;
    err = damos_commit_dests(&dst.migrate_dests, &src.migrate_dests);
    if (err) {
    return err;
    }
    err = damos_commit_filters(dst, src);
    if (err) {
    return err;
    }
    dst.max_nr_snapshots = src.max_nr_snapshots;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn damon_commit_schemes(dst: *mut damon_ctx, src: *mut damon_ctx) -> c_int {
    let mut dst_scheme = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
    let mut src_scheme = core::ptr::null_mut();
    let mut new_scheme = core::ptr::null_mut();
pub static mut i: c_int = 0;
    damon_for_each_scheme_safe(dst_scheme, next, dst) {
    src_scheme = damon_nth_scheme(i++, src);
    if (src_scheme) {
    err = damos_commit(dst_scheme, src_scheme);
    if (err) {
    return err;
    }
    } else {
    damon_destroy_scheme(dst_scheme);
    }
    }
    damon_for_each_scheme_safe(src_scheme, next, src) {
    if (j++ < i) {
    continue;
    }
    new_scheme = damon_new_scheme(&src_scheme.pattern,
    src_scheme.action,
    src_scheme.apply_interval_us,
    &src_scheme.quota, &src_scheme.wmarks,
    NUMA_NO_NODE);
    if (!new_scheme) {
    return -ENOMEM;
    }
    err = damos_commit(new_scheme, src_scheme);
    if (err) {
    damon_destroy_scheme(new_scheme);
    return err;
    }
    damon_add_scheme(dst, new_scheme);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn damon_nth_target(n: c_int, ctx: *mut damon_ctx) -> *mut c_void {
pub static mut t: *mut c_void = core::ptr::null_mut();
pub static mut i: c_int = 0;
    damon_for_each_target(t, ctx) {
    if (i++ == n) {
    return t;
    }
    }
    return core::ptr::null_mut();
    }
//
// The caller should ensure the regions of @src are
// 1. valid (end >= src) and
// 2. sorted by starting address.
//
// If @src has no region, @dst keeps current regions.
//
#[no_mangle]
pub unsafe extern "C" fn damon_commit_target_regions(dst: *mut damon_target, src: *mut damon_target, src_min_region_sz: c_ulong) -> c_int {
    struct damon_region *src_region, *prev = core::ptr::null_mut();
pub static mut ranges: *mut c_void = core::ptr::null_mut();
pub static mut i: c_int = 0;
    damon_for_each_region(src_region, src) {
    if (!prev || prev.ar.end != src_region.ar.start) {
    i += 1;
    }
    prev = src_region;
    }
    if (!i) {
    return 0;
    }
    ranges = kvmalloc_objs(*ranges, i, GFP_KERNEL | __GFP_NOWARN);
    if (!ranges) {
    return -ENOMEM;
    }
    prev = core::ptr::null_mut();
    i = 0;
    damon_for_each_region(src_region, src) {
    if (!prev) {
    ranges[i].start = src_region.ar.start;
    } else if (prev.ar.end != src_region.ar.start) {
    ranges[i++].end = prev.ar.end;
    ranges[i].start = src_region.ar.start;
    }
    prev = src_region;
    }
    ranges[i++].end = damon_last_region(src).ar.end;
    err = damon_set_regions(dst, ranges, i, src_min_region_sz);
    kvfree(ranges);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn damon_commit_target(dst: *mut damon_target, dst_has_pid: bool, src: *mut damon_target, src_has_pid: bool, src_min_region_sz: c_ulong) -> c_int {
    let mut err = 0;
    err = damon_commit_target_regions(dst, src, src_min_region_sz);
    if (err) {
    return err;
    }
    if (dst_has_pid) {
    put_pid(dst.pid);
    }
    if (src_has_pid) {
    get_pid(src.pid);
    }
    dst.pid = src.pid;
    return 0;
    }
//
// damon_revert_target_commits() - revert unsuccessful target commits.
// @dst:	Commit destination context
// @failed:	Commit failed destination target
// @src:	Commit source context
//
// Revert target states that changed by damon_commit_target(), and cannot be
// cleaned up by the destination context's ops.cleanup_target().
//
#[no_mangle]
pub unsafe extern "C" fn damon_revert_target_commits(dst: *mut damon_ctx, failed: *mut damon_target, src: *mut damon_ctx) {
pub static mut target: *mut c_void = core::ptr::null_mut();
    if (!damon_target_has_pid(src)) {
    return;
    }
    if (dst.ops.cleanup_target) {
    return;
    }
    damon_for_each_target(target, dst) {
    if (target == failed) {
    return;
    }
    put_pid(target.pid);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn damon_commit_targets(dst: *mut damon_ctx, src: *mut damon_ctx) -> c_int {
    let mut dst_target = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
    let mut src_target = core::ptr::null_mut();
    let mut new_target = core::ptr::null_mut();
pub static mut failed: *mut c_void = core::ptr::null_mut();
pub static mut i: c_int = 0;
    damon_for_each_target_safe(dst_target, next, dst) {
    src_target = damon_nth_target(i++, src);
//
// If src target is obsolete, do not commit the parameters to
// the dst target, and further remove the dst target.
//
    if (src_target && !src_target.obsolete) {
    err = damon_commit_target(
    dst_target, damon_target_has_pid(dst),
    src_target, damon_target_has_pid(src),
    src.min_region_sz);
    if (err) {
    failed = dst_target;
// goto;
    }
    } else {
pub static mut s: *mut c_void = core::ptr::null_mut();
    damon_destroy_target(dst_target, dst);
    damon_for_each_scheme(s, dst) {
    if (s.quota.charge_target_from == dst_target) {
    s.quota.charge_target_from = core::ptr::null_mut();
    s.quota.charge_addr_from = 0;
    }
    }
    }
    }
    failed = core::ptr::null_mut();
    damon_for_each_target_safe(src_target, next, src) {
    if (j++ < i) {
    continue;
    }
// target to remove has no matching dst
    if (src_target.obsolete) {
    err = -EINVAL;
// goto;
    }
    new_target = damon_new_target();
    if (!new_target) {
    err = -ENOMEM;
// goto;
    }
    err = damon_commit_target(new_target, false,
    src_target, damon_target_has_pid(src),
    src.min_region_sz);
    if (err) {
    damon_destroy_target(new_target, core::ptr::null_mut());
// goto;
    }
    damon_add_target(dst, new_target);
    }
    return 0;
// label;
    damon_revert_target_commits(dst, failed, src);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn damon_commit_filter(dst: *mut damon_filter, src: *mut damon_filter) {
    dst.type = src.type;
    dst.matching = src.matching;
    dst.allow = src.allow;
    match (dst.type) {
    DAMON_FILTER_TYPE_MEMCG => {
    dst.memcg_id = src.memcg_id;
    // break;
    }
    _ => {
    // break;
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn damon_commit_filters(dst: *mut damon_probe, src: *mut damon_probe) -> c_int {
    let mut dst_filter = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
    let mut src_filter = core::ptr::null_mut();
    let mut new_filter = core::ptr::null_mut();
pub static mut i: c_int = 0;
    damon_for_each_filter_safe(dst_filter, next, dst) {
    src_filter = damon_nth_filter(i++, src);
    if (src_filter) {
    damon_commit_filter(dst_filter, src_filter);
    }
    else {
    damon_destroy_filter(dst_filter);
    }
    }
    damon_for_each_filter_safe(src_filter, next, src) {
    if (j++ < i) {
    continue;
    }
    new_filter = damon_new_filter(src_filter.type,
    src_filter.matching, src_filter.allow);
    if (!new_filter) {
    return -ENOMEM;
    }
    match (src_filter.type) {
    DAMON_FILTER_TYPE_MEMCG => {
    new_filter.memcg_id = src_filter.memcg_id;
    // break;
    }
    _ => {
    // break;
    }
    }
    damon_add_filter(dst, new_filter);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn damon_commit_probes(dst: *mut damon_ctx, src: *mut damon_ctx) -> c_int {
    let mut dst_probe = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
    let mut src_probe = core::ptr::null_mut();
    let mut new_probe = core::ptr::null_mut();
pub static mut i: c_int = 0;
    damon_for_each_probe_safe(dst_probe, next, dst) {
    src_probe = damon_nth_probe(i++, src);
    if (src_probe) {
    dst_probe.weight = src_probe.weight;
    err = damon_commit_filters(dst_probe, src_probe);
    if (err) {
    return err;
    }
    } else {
    damon_destroy_probe(dst_probe);
    }
    }
    damon_for_each_probe_safe(src_probe, next, src) {
    if (j++ < i) {
    continue;
    }
    new_probe = damon_new_probe();
    if (!new_probe) {
    return -ENOMEM;
    }
    damon_add_probe(dst, new_probe);
    new_probe.weight = src_probe.weight;
    err = damon_commit_filters(new_probe, src_probe);
    if (err) {
    return err;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn __damon_commit_ctx(dst: *mut damon_ctx, src: *mut damon_ctx) -> c_int {
    let mut err = 0;
pub static mut scheme: *mut c_void = core::ptr::null_mut();
pub static mut goal: *mut c_void = core::ptr::null_mut();
    dst.maybe_corrupted = true;
    if (!is_power_of_2(src.min_region_sz)) {
    return -EINVAL;
    }
// node_eligible_mem_bp metric requires PADDR ops
    if (src.ops.id != DAMON_OPS_PADDR) {
    damon_for_each_scheme(scheme, src) {
    let mut quota = &scheme.quota;
    damos_for_each_quota_goal(goal, quota) {
    if (goal.metric ==
    DAMOS_QUOTA_NODE_ELIGIBLE_MEM_BP) {
    return -EINVAL;
    }
    }
    }
    }
    if (!damon_valid_probe_params(src)) {
    return -EINVAL;
    }
    err = damon_commit_schemes(dst, src);
    if (err) {
    return err;
    }
    err = damon_commit_targets(dst, src);
    if (err) {
    return err;
    }
//
// schemes and targets should be updated first, since
// 1. damon_set_attrs() updates monitoring results of targets and
// next_apply_sis of schemes, and
// 2. ops update should be done after pid handling is done (target
// committing require putting pids).
//
    if (!damon_attrs_equals(&dst.attrs, &src.attrs)) {
    err = damon_set_attrs(dst, &src.attrs);
    if (err) {
    damon_revert_target_commits(dst, core::ptr::null_mut(), src);
    return err;
    }
    }
    dst.pause = src.pause;
    dst.ops = src.ops;
    err = damon_commit_probes(dst, src);
    if (err) {
    return err;
    }
    dst.addr_unit = src.addr_unit;
    dst.min_region_sz = src.min_region_sz;
    dst.maybe_corrupted = false;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn damon_new_test_ctx(dst: *mut damon_ctx) -> *mut c_void {
pub static mut test_ctx: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    test_ctx = damon_new_ctx();
    if (!test_ctx) {
    return core::ptr::null_mut();
    }
    err = __damon_commit_ctx(test_ctx, dst);
    if (err) {
    damon_destroy_ctx(test_ctx);
    return core::ptr::null_mut();
    }
    return test_ctx;
    }
//
// damon_commit_ctx() - Commit parameters of a DAMON context to another.
// @dst:	The commit destination DAMON context.
// @src:	The commit source DAMON context.
//
// This function copies user-specified parameters from @src to @dst and update
// the internal status and results accordingly.  Users should use this function
// for context-level parameters update of running context, instead of manual
// in-place updates.
//
// This function should be called from parameters-update safe context, like
// damon_call().
//
#[no_mangle]
pub unsafe extern "C" fn damon_commit_ctx(dst: *mut damon_ctx, src: *mut damon_ctx) -> c_int {
pub static mut test_ctx: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    test_ctx = damon_new_test_ctx(dst);
    if (!test_ctx) {
    return -ENOMEM;
    }
    err = __damon_commit_ctx(test_ctx, src);
    if (err) {
// goto;
    }
    err = __damon_commit_ctx(dst, src);
// label;
    damon_destroy_ctx(test_ctx);
    return err;
    }
//
// damon_nr_running_ctxs() - Return number of currently running contexts.
//
#[no_mangle]
pub unsafe extern "C" fn damon_nr_running_ctxs() -> c_int {
    let mut nr_ctxs = 0;
    mutex_lock(&damon_lock);
    nr_ctxs = nr_running_ctxs;
    mutex_unlock(&damon_lock);
    return nr_ctxs;
    }
// Returns the size upper limit for each monitoring region
#[no_mangle]
unsafe extern "C" fn damon_region_sz_limit(ctx: *mut damon_ctx) -> c_ulong {
pub static mut t: *mut c_void = core::ptr::null_mut();
pub static mut r: *mut c_void = core::ptr::null_mut();
pub static mut sz: c_ulong = 0;
    damon_for_each_target(t, ctx) {
    damon_for_each_region(r, t)
    sz += damon_sz_region(r);
    }
    if (ctx.attrs.min_nr_regions) {
    sz /= ctx.attrs.min_nr_regions;
    }
    if (sz < ctx.min_region_sz) {
    sz = ctx.min_region_sz;
    }
    return sz;
    }
// forward_decl: damon_split_region_at;
//
// damon_apply_min_nr_regions() - Make effect of min_nr_regions parameter.
// @ctx:	monitoring context.
//
// This function implement min_nr_regions (minimum number of damon_region
// objects in the given monitoring context) behavior.  It first calculates
// maximum size of each region for enforcing the min_nr_regions as total size
// of the regions divided by the min_nr_regions.  After that, this function
// splits regions to ensure all regions are equal to or smaller than the size
// limit.  Finally, this function returns the maximum size limit.
//
// Returns: maximum size of each region for convincing min_nr_regions.
//
#[no_mangle]
unsafe extern "C" fn damon_apply_min_nr_regions(ctx: *mut damon_ctx) -> c_ulong {
pub static mut max_region_sz: c_ulong = 0;
pub static mut t: *mut c_void = core::ptr::null_mut();
    let mut r = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
    max_region_sz = ALIGN(max_region_sz, ctx.min_region_sz);
    damon_for_each_target(t, ctx) {
    damon_for_each_region_safe(r, next, t) {
    while (damon_sz_region(r) > max_region_sz) {
    if (damon_split_region_at(t, r, max_region_sz)) {
// goto;
    }
    r = damon_next_region(r);
    }
    }
    }
// label;
    return max_region_sz;
    }
// forward_decl: kdamond_fn;
//
// __damon_start() - Starts monitoring with given context.
// @ctx:	monitoring context
//
// This function should be called while damon_lock is hold.
//
// Return: 0 on success, negative error code otherwise.
//
#[no_mangle]
unsafe extern "C" fn __damon_start(ctx: *mut damon_ctx) -> c_int {
pub static mut err: c_int = 0;
    mutex_lock(&ctx.kdamond_lock);
    if (!ctx.kdamond) {
    err = 0;
    reinit_completion(&ctx.kdamond_started);
    ctx.kdamond = kthread_run(kdamond_fn, ctx, "kdamond.%d",
    nr_running_ctxs);
    if (IS_ERR(ctx.kdamond)) {
    err = PTR_ERR(ctx.kdamond);
    ctx.kdamond = core::ptr::null_mut();
    } else {
    wait_for_completion(&ctx.kdamond_started);
    }
    }
    mutex_unlock(&ctx.kdamond_lock);
    return err;
    }
// forward_decl: __damon_commit_ctx;
//
// damon_start() - Starts the monitorings for a given group of contexts.
// @ctxs:	an array of the pointers for contexts to start monitoring
// @nr_ctxs:	size of @ctxs
// @exclusive:	exclusiveness of this contexts group
//
// This function starts a group of monitoring threads for a group of monitoring
// contexts.  One thread per each context is created and run in parallel.  The
// caller should handle synchronization between the threads by itself.  If
// @exclusive is true and a group of threads that created by other
// 'damon_start()' call is currently running, this function does nothing but
// returns -EBUSY.
//
// Return: 0 on success, negative error code otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn damon_start(ctxs: *mut damon_ctx, nr_ctxs: c_int, exclusive: bool) -> c_int {
    let mut i = 0;
pub static mut err: c_int = 0;
    while (i < nr_ctxs) {
pub static mut test_ctx: *mut c_void = core::ptr::null_mut();
    test_ctx = damon_new_ctx();
    if (!test_ctx) {
    return -ENOMEM;
    }
    err = __damon_commit_ctx(test_ctx, ctxs[i]);
    damon_destroy_ctx(test_ctx);
    if (err) {
    return err;
    }
    }
    mutex_lock(&damon_lock);
    if ((exclusive && nr_running_ctxs) ||
    (!exclusive && running_exclusive_ctxs)) {
    mutex_unlock(&damon_lock);
    return -EBUSY;
    }
    while (i < nr_ctxs) {
    err = __damon_start(ctxs[i]);
    if (err) {
    break;
    }
    nr_running_ctxs += 1;
    }
    if (exclusive && nr_running_ctxs) {
    running_exclusive_ctxs = true;
    }
    mutex_unlock(&damon_lock);
    if (i != nr_ctxs) {
    damon_stop(ctxs, i);
    }
    return err;
    }
//
// __damon_stop() - Stops monitoring of a given context.
// @ctx:	monitoring context
//
#[no_mangle]
unsafe extern "C" fn __damon_stop(ctx: *mut damon_ctx) {
pub static mut tsk: *mut c_void = core::ptr::null_mut();
    mutex_lock(&ctx.kdamond_lock);
    tsk = ctx.kdamond;
    if (tsk) {
    get_task_struct(tsk);
    mutex_unlock(&ctx.kdamond_lock);
    kthread_stop_put(tsk);
    return;
    }
    mutex_unlock(&ctx.kdamond_lock);
    }
//
// damon_stop() - Stops the monitorings for a given group of contexts.
// @ctxs:	an array of the pointers for contexts to stop monitoring
// @nr_ctxs:	size of @ctxs
//
#[no_mangle]
pub unsafe extern "C" fn damon_stop(ctxs: *mut damon_ctx, nr_ctxs: c_int) {
    let mut i = 0;
    for (i = 0; i < nr_ctxs; i++) {
// nr_running_ctxs is decremented in kdamond_fn
    __damon_stop(ctxs[i]);
    }
    }
//
// damon_is_running() - Returns if a given DAMON context is running.
// @ctx:	The DAMON context to see if running.
//
// Return: true if @ctx is running, false otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn damon_is_running(ctx: *mut damon_ctx) -> bool {
    let mut running = 0;
    mutex_lock(&ctx.kdamond_lock);
    running = ctx.kdamond != core::ptr::null_mut();
    mutex_unlock(&ctx.kdamond_lock);
    return running;
    }
//
// damon_kdamond_pid() - Return pid of a given DAMON context's worker thread.
// @ctx:	The DAMON context of the question.
//
// Return: pid if @ctx is running, negative error code otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn damon_kdamond_pid(ctx: *mut damon_ctx) -> c_int {
pub static mut pid: c_int = 0;
    mutex_lock(&ctx.kdamond_lock);
    if (ctx.kdamond) {
    pid = ctx.kdamond.pid;
    }
    mutex_unlock(&ctx.kdamond_lock);
    return pid;
    }
//
// damon_call() - Invoke a given function on DAMON worker thread (kdamond).
// @ctx:	DAMON context to call the function for.
// @control:	Control variable of the call request.
//
// Ask DAMON worker thread (kdamond) of @ctx to call a function with an
// argument data that respectively passed via &damon_call_control->fn and
// &damon_call_control->data of @control.  If &damon_call_control->repeat of
// @control is unset, further wait until the kdamond finishes handling of the
// request.  Otherwise, return as soon as the request is made.
//
// The kdamond executes the function with the argument in the main loop, just
// after a sampling of the iteration is finished.  The function can hence
// safely access the internal data of the &struct damon_ctx without additional
// synchronization.  The return value of the function will be saved in
// &damon_call_control->return_code.
//
// Note that this function should be called only after damon_start() with the
// @ctx has succeeded.  Otherwise, this function could fall into an indefinite
// wait.
//
// When this function is failed, the @ctx is guaranteed to be stopped.
//
// Return: 0 on success, negative error code otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn damon_call(ctx: *mut damon_ctx, control: *mut damon_call_control) -> c_int {
    if (!control.repeat) {
    init_completion(&control.completion);
    }
    control.canceled = false;
    INIT_LIST_HEAD(&control.list);
    mutex_lock(&ctx.call_controls_lock);
    if (ctx.call_controls_obsolete) {
    mutex_unlock(&ctx.call_controls_lock);
// goto;
    }
    list_add_tail(&control.list, &ctx.call_controls);
    mutex_unlock(&ctx.call_controls_lock);
    if (control.repeat) {
    return 0;
    }
    wait_for_completion(&control.completion);
    if (control.canceled) {
// goto;
    }
    return 0;
// label;
    while (damon_is_running(ctx)) {
    schedule_timeout_idle(msecs_to_jiffies(100));
    }
    return -ECANCELED;
    }
//
// damos_walk() - Invoke a given functions while DAMOS walk regions.
// @ctx:	DAMON context to call the functions for.
// @control:	Control variable of the walk request.
//
// Ask DAMON worker thread (kdamond) of @ctx to call a function for each region
// that the kdamond will apply DAMOS action to, and wait until the kdamond
// finishes handling of the request.
//
// The kdamond executes the given function in the main loop, for each region
// just after it applied any DAMOS actions of @ctx to it.  The invocation is
// made only within one &damos->apply_interval_us since damos_walk()
// invocation, for each scheme.  The given callback function can hence safely
// access the internal data of &struct damon_ctx and &struct damon_region that
// each of the scheme will apply the action for next interval, without
// additional synchronizations against the kdamond.  If every scheme of @ctx
// passed at least one &damos->apply_interval_us, kdamond marks the request as
// completed so that damos_walk() can wakeup and return.
//
// Note that this function should be called only after damon_start() with the
// @ctx has succeeded.  Otherwise, this function could fall into an indefinite
// wait.
//
// Return: 0 on success, negative error code otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn damos_walk(ctx: *mut damon_ctx, control: *mut damos_walk_control) -> c_int {
    init_completion(&control.completion);
    control.canceled = false;
    mutex_lock(&ctx.walk_control_lock);
    if (ctx.walk_control_obsolete) {
    mutex_unlock(&ctx.walk_control_lock);
    return -ECANCELED;
    }
    if (ctx.walk_control) {
    mutex_unlock(&ctx.walk_control_lock);
    return -EBUSY;
    }
    ctx.walk_control = control;
    mutex_unlock(&ctx.walk_control_lock);
    wait_for_completion(&control.completion);
    if (control.canceled) {
    return -ECANCELED;
    }
    return 0;
    }
//
// Reset the aggregated monitoring results ('nr_accesses' of each region).
//
#[no_mangle]
unsafe extern "C" fn kdamond_reset_aggregated(c: *mut damon_ctx) {
pub static mut t: *mut c_void = core::ptr::null_mut();
    let mut ti = 0;	/* target's index */
pub static mut nr_probes: c_uint = 0;
pub static mut probe: *mut c_void = core::ptr::null_mut();
    if (trace_damon_region_aggregated_enabled()) {
    damon_for_each_probe(probe, c)
    nr_probes += 1;
    }
    damon_for_each_target(t, c) {
pub static mut r: *mut c_void = core::ptr::null_mut();
    damon_for_each_region(r, t) {
    let mut i = 0;
    trace_damon_aggregated(ti, r, damon_nr_regions(t));
    trace_damon_region_aggregated(ti, r,
    damon_nr_regions(t), nr_probes);
    r.last_nr_accesses = r.nr_accesses;
    r.nr_accesses = 0;
    while (i < DAMON_MAX_PROBES) {
    r.last_probe_hits[i] = r.probe_hits[i];
    r.probe_hits[i] = 0;
    }
    }
    ti += 1;
    }
    }
#[no_mangle]
unsafe extern "C" fn damon_get_intervals_score(c: *mut damon_ctx) -> c_ulong {
pub static mut t: *mut c_void = core::ptr::null_mut();
pub static mut r: *mut c_void = core::ptr::null_mut();
    unsigned long sz_region, max_access_events = 0, access_events = 0;
    let mut target_access_events = 0;
pub static mut goal_bp: c_ulong = 0;
    damon_for_each_target(t, c) {
    damon_for_each_region(r, t) {
    sz_region = damon_sz_region(r);
    max_access_events += sz_region * c.attrs.aggr_samples;
    access_events += sz_region * r.nr_accesses;
    }
    }
    target_access_events = max_access_events * goal_bp / 10000;
    target_access_events = target_access_events ? : 1;
    return mult_frac(access_events, 10000, target_access_events);
    }
// forward_decl: damon_feed_loop_next_input;
#[no_mangle]
unsafe extern "C" fn damon_get_intervals_adaptation_bp(c: *mut damon_ctx) -> c_ulong {
    unsigned long score_bp, adaptation_bp;
    score_bp = damon_get_intervals_score(c);
    adaptation_bp = damon_feed_loop_next_input(100000000, score_bp) /
    10000;
//
// adaptation_bp ranges from 1 to 20,000.  Avoid too rapid reduction of
// the intervals by rescaling [1,10,000] to [5000, 10,000].
//
    if (adaptation_bp <= 10000) {
    adaptation_bp = 5000 + adaptation_bp / 2;
    }
    return adaptation_bp;
    }
#[no_mangle]
unsafe extern "C" fn kdamond_tune_intervals(c: *mut damon_ctx) -> noinline_for_stack void {
    let mut adaptation_bp = 0;
pub static mut new_attrs: usize = 0;
pub static mut goal: *mut c_void = core::ptr::null_mut();
    adaptation_bp = damon_get_intervals_adaptation_bp(c);
    if (adaptation_bp == 10000) {
    return;
    }
    new_attrs = c.attrs;
    goal = &c.attrs.intervals_goal;
    new_attrs.sample_interval = min(goal.max_sample_us,
    c.attrs.sample_interval * adaptation_bp / 10000);
    new_attrs.sample_interval = max(goal.min_sample_us,
    new_attrs.sample_interval);
    new_attrs.aggr_interval = new_attrs.sample_interval *
    c.attrs.aggr_samples;
    trace_damon_monitor_intervals_tune(new_attrs.sample_interval);
    damon_set_attrs(c, &new_attrs);
    }
#[no_mangle]
pub unsafe extern "C" fn __damos_valid_target(r: *mut damon_region, s: *mut damos, c: *mut damon_ctx) -> bool {
    let mut sz = 0;
pub static mut nr_accesses: c_uint = 0;
    sz = damon_sz_region(r);
    return s.pattern.min_sz_region <= sz &&
    sz <= s.pattern.max_sz_region &&
    s.pattern.min_nr_accesses <= nr_accesses &&
    nr_accesses <= s.pattern.max_nr_accesses &&
    s.pattern.min_age_region <= r.age &&
    r.age <= s.pattern.max_age_region;
    }
//
// damos_quota_is_set() - Return if the given quota is actually set.
// @quota:	The quota to check.
//
// Returns true if the quota is set, false otherwise.
//
#[no_mangle]
unsafe extern "C" fn damos_quota_is_set(quota: *mut damos_quota) -> bool {
    return quota.esz || quota.sz || quota.ms ||
    !damos_quota_goals_empty(quota);
    }
#[no_mangle]
pub unsafe extern "C" fn damos_valid_target(c: *mut damon_ctx, r: *mut damon_region, s: *mut damos) -> bool {
pub static mut ret: bool = false;
    if (!ret || !damos_quota_is_set(&s.quota) || !c.ops.get_scheme_score) {
    return ret;
    }
    return c.ops.get_scheme_score(c, r, s) >= s.quota.min_score;
    }
//
// damos_skip_charged_region() - Check if the given region or starting part of
// it is already charged for the DAMOS quota.
// @t:	The target of the region.
// @rp:	The pointer to the region.
// @s:	The scheme to be applied.
// @min_region_sz:	minimum region size.
//
// If a quota of a scheme has exceeded in a quota charge window, the scheme's
// action would applied to only a part of the target access pattern fulfilling
// regions.  To avoid applying the scheme action to only already applied
// regions, DAMON skips applying the scheme action to the regions that charged
// in the previous charge window.
//
// This function checks if a given region should be skipped or not for the
// reason.  If only the starting part of the region has previously charged,
// this function splits the region into two so that the second one covers the
// area that not charged in the previous charge widnow, and return true.  The
// caller can see the second one on the next iteration of the region walk.
// Note that this means the caller should use damon_for_each_region() instead
// of damon_for_each_region_safe().  If damon_for_each_region_safe() is used,
// the second region will just be ignored.
//
// Return: true if the region should be skipped, false otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn damos_skip_charged_region(t: *mut damon_target, r: *mut damon_region, s: *mut damos, min_region_sz: c_ulong) -> bool {
    let mut quota = &s.quota;
    let mut sz_to_skip = 0;
// Skip previously charged regions
    if (quota.charge_target_from) {
    if (t != quota.charge_target_from) {
    return true;
    }
    if (r == damon_last_region(t)) {
    quota.charge_target_from = core::ptr::null_mut();
    quota.charge_addr_from = 0;
    return true;
    }
    if (quota.charge_addr_from &&
    r.ar.end <= quota.charge_addr_from) {
    return true;
    }
    if (quota.charge_addr_from && r.ar.start <
    quota.charge_addr_from) {
    sz_to_skip = ALIGN_DOWN(quota.charge_addr_from -
    r.ar.start, min_region_sz);
    if (!sz_to_skip) {
    if (damon_sz_region(r) <= min_region_sz) {
    return true;
    }
    sz_to_skip = min_region_sz;
    }
    damon_split_region_at(t, r, sz_to_skip);
    return true;
    }
    quota.charge_target_from = core::ptr::null_mut();
    quota.charge_addr_from = 0;
    }
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn damos_update_stat(s: *mut damos, sz_tried: c_ulong, sz_applied: c_ulong, sz_ops_filter_passed: c_ulong) {
    s.stat.nr_tried += 1;
    s.stat.sz_tried += sz_tried;
    if (sz_applied) {
    s.stat.nr_applied += 1;
    }
    s.stat.sz_applied += sz_applied;
    s.stat.sz_ops_filter_passed += sz_ops_filter_passed;
    }
#[no_mangle]
pub unsafe extern "C" fn damos_filter_match(ctx: *mut damon_ctx, t: *mut damon_target, r: *mut damon_region, filter: *mut damos_filter, min_region_sz: c_ulong) -> bool {
pub static mut matched: bool = false;
pub static mut ti: *mut c_void = core::ptr::null_mut();
pub static mut target_idx: c_int = 0;
    unsigned long start, end;
    match (filter.type) {
    DAMOS_FILTER_TYPE_TARGET => {
    damon_for_each_target(ti, ctx) {
    if (ti == t) {
    // break;
    }
    target_idx += 1;
    }
    matched = target_idx == filter.target_idx;
    // break;
    }
    DAMOS_FILTER_TYPE_ADDR => {
    start = ALIGN_DOWN(filter.addr_range.start, min_region_sz);
    end = ALIGN_DOWN(filter.addr_range.end, min_region_sz);
// inside the range
    if (start <= r.ar.start && r.ar.end <= end) {
    matched = true;
    // break;
    }
// outside of the range
    if (r.ar.end <= start || end <= r.ar.start) {
    matched = false;
    // break;
    }
// start before the range and overlap
    if (r.ar.start < start) {
    damon_split_region_at(t, r, start - r.ar.start);
    matched = false;
    // break;
    }
// start inside the range
    damon_split_region_at(t, r, end - r.ar.start);
    matched = true;
    // break;
    }
    _ => {
    return false;
    }
    }
pub static mut matched: return = 0;
    }
#[no_mangle]
pub unsafe extern "C" fn damos_core_filter_out(ctx: *mut damon_ctx, t: *mut damon_target, r: *mut damon_region, s: *mut damos) -> bool {
pub static mut filter: *mut c_void = core::ptr::null_mut();
    s.core_filters_allowed = false;
    damos_for_each_core_filter(filter, s) {
    if (damos_filter_match(ctx, t, r, filter, ctx.min_region_sz)) {
    if (filter.allow) {
    s.core_filters_allowed = true;
    }
    return !filter.allow;
    }
    }
    return s.core_filters_default_reject;
    }
//
// damos_walk_call_walk() - Call &damos_walk_control->walk_fn.
// @ctx:	The context of &damon_ctx->walk_control.
// @t:		The monitoring target of @r that @s will be applied.
// @r:		The region of @t that @s will be applied.
// @s:		The scheme of @ctx that will be applied to @r.
//
// This function is called from kdamond whenever it asked the operation set to
// apply a DAMOS scheme action to a region.  If a DAMOS walk request is
// installed by damos_walk() and not yet uninstalled, invoke it.
//
#[no_mangle]
pub unsafe extern "C" fn damos_walk_call_walk(ctx: *mut damon_ctx, t: *mut damon_target, r: *mut damon_region, s: *mut damos, sz_filter_passed: c_ulong) {
pub static mut control: *mut c_void = core::ptr::null_mut();
    if (s.walk_completed) {
    return;
    }
    control = ctx.walk_control;
    if (!control) {
    return;
    }
    control.walk_fn(control.data, ctx, t, r, s, sz_filter_passed);
    }
//
// damos_walk_complete() - Complete DAMOS walk request if all walks are done.
// @ctx:	The context of &damon_ctx->walk_control.
// @s:		A scheme of @ctx that all walks are now done.
//
// This function is called when kdamond finished applying the action of a DAMOS
// scheme to all regions that eligible for the given &damos->apply_interval_us.
// If every scheme of @ctx including @s now finished walking for at least one
// &damos->apply_interval_us, this function makrs the handling of the given
// DAMOS walk request is done, so that damos_walk() can wake up and return.
//
#[no_mangle]
unsafe extern "C" fn damos_walk_complete(ctx: *mut damon_ctx, s: *mut damos) {
pub static mut siter: *mut c_void = core::ptr::null_mut();
pub static mut control: *mut c_void = core::ptr::null_mut();
    control = ctx.walk_control;
    if (!control) {
    return;
    }
    s.walk_completed = true;
// if all schemes completed, signal completion to walker
    damon_for_each_scheme(siter, ctx) {
    if (!siter.walk_completed) {
    return;
    }
    }
    damon_for_each_scheme(siter, ctx)
    siter.walk_completed = false;
    complete(&control.completion);
    ctx.walk_control = core::ptr::null_mut();
    }
//
// damos_walk_cancel() - Cancel the current DAMOS walk request.
// @ctx:	The context of &damon_ctx->walk_control.
//
// This function is called when @ctx is deactivated by DAMOS watermarks, DAMOS
// walk is requested but there is no DAMOS scheme to walk for, or the kdamond
// is already out of the main loop and therefore gonna be terminated, and hence
// cannot continue the walks.  This function therefore marks the walk request
// as canceled, so that damos_walk() can wake up and return.
//
#[no_mangle]
unsafe extern "C" fn damos_walk_cancel(ctx: *mut damon_ctx) {
pub static mut control: *mut c_void = core::ptr::null_mut();
    mutex_lock(&ctx.walk_control_lock);
    control = ctx.walk_control;
    mutex_unlock(&ctx.walk_control_lock);
    if (!control) {
    return;
    }
    control.canceled = true;
    complete(&control.completion);
    mutex_lock(&ctx.walk_control_lock);
    ctx.walk_control = core::ptr::null_mut();
    mutex_unlock(&ctx.walk_control_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn damos_charge_quota(quota: *mut damos_quota, sz_region: c_ulong, sz_applied: c_ulong) {
//
// sz_applied could be bigger than sz_region, depending on ops
// implementation of the action, e.g., damos_pa_pageout().  Charge only
// the region size in the case.
//
    if (!quota.fail_charge_denom || sz_applied > sz_region) {
    quota.charged_sz += sz_region;
    }
    else {
    quota.charged_sz += sz_applied + mult_frac(
    (sz_region - sz_applied),
    quota.fail_charge_num,
    quota.fail_charge_denom);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn damos_quota_is_full(quota: *mut damos_quota, min_region_sz: c_ulong) -> bool {
    if (!damos_quota_is_set(quota)) {
    return false;
    }
    if (quota.charged_sz >= quota.esz) {
    return true;
    }
//
// DAMOS action is applied per region, so <min_region_sz remaining
// quota means the quota is effectively full.
//
    return quota.esz - quota.charged_sz < min_region_sz;
    }
#[no_mangle]
pub unsafe extern "C" fn damos_apply_scheme(c: *mut damon_ctx, t: *mut damon_target, r: *mut damon_region, s: *mut damos) {
    let mut quota = &s.quota;
pub static mut sz: c_ulong = 0;
    struct timespec64 begin, end;
pub static mut sz_applied: c_ulong = 0;
pub static mut sz_ops_filter_passed: c_ulong = 0;
//
// We plan to support multiple context per kdamond, as DAMON sysfs
// implies with 'nr_contexts' file.  Nevertheless, only single context
// per kdamond is supported for now.  So, we can simply use '0' context
// index here.
//
pub static mut cidx: c_uint = 0;
pub static mut siter: *mut c_void = core::ptr::null_mut();		/* schemes iterator */
pub static mut sidx: c_uint = 0;
pub static mut titer: *mut c_void = core::ptr::null_mut();	/* targets iterator */
pub static mut tidx: c_uint = 0;
pub static mut do_trace: bool = false;
// get indices for trace_damos_before_apply()
    if (trace_damos_before_apply_enabled()) {
    damon_for_each_scheme(siter, c) {
    if (siter == s) {
    break;
    }
    sidx += 1;
    }
    damon_for_each_target(titer, c) {
    if (titer == t) {
    break;
    }
    tidx += 1;
    }
    nr_accesses = damon_nr_accesses_mvsum(r, c);
    do_trace = true;
    }
    if (c.ops.apply_scheme) {
    if (damos_quota_is_set(quota) &&
    quota.charged_sz + sz > quota.esz) {
    sz = ALIGN_DOWN(quota.esz - quota.charged_sz,
    c.min_region_sz);
    if (!sz) {
// goto;
    }
    damon_split_region_at(t, r, sz);
    }
    if (damos_core_filter_out(c, t, r, s)) {
    return;
    }
    ktime_get_coarse_ts64(&begin);
    trace_damos_before_apply(cidx, sidx, tidx, r, nr_accesses,
    damon_nr_regions(t), do_trace);
    sz_applied = c.ops.apply_scheme(c, t, r, s,
    &sz_ops_filter_passed);
    damos_walk_call_walk(c, t, r, s, sz_ops_filter_passed);
    ktime_get_coarse_ts64(&end);
    quota.total_charged_ns += timespec64_to_ns(&end) -
    timespec64_to_ns(&begin);
    damos_charge_quota(quota, sz, sz_applied);
    if (damos_quota_is_full(quota, c.min_region_sz)) {
    quota.charge_target_from = t;
    quota.charge_addr_from = r.ar.end;
    }
    }
    if (s.action != DAMOS_STAT) {
    r.age = 0;
    }
// label;
    damos_update_stat(s, sz, sz_applied, sz_ops_filter_passed);
    }
#[no_mangle]
pub unsafe extern "C" fn damon_do_apply_schemes(c: *mut damon_ctx, t: *mut damon_target, r: *mut damon_region) {
pub static mut s: *mut c_void = core::ptr::null_mut();
    damon_for_each_scheme(s, c) {
    let mut quota = &s.quota;
    if (time_before(c.passed_sample_intervals, s.next_apply_sis)) {
    continue;
    }
    if (!s.wmarks.activated) {
    continue;
    }
// Check the quota
    if (damos_quota_is_full(quota, c.min_region_sz)) {
    continue;
    }
    if (damos_skip_charged_region(t, r, s, c.min_region_sz)) {
    continue;
    }
    if (s.max_nr_snapshots &&
    s.max_nr_snapshots <= s.stat.nr_snapshots) {
    continue;
    }
    if (damos_valid_target(c, r, s)) {
    damos_apply_scheme(c, t, r, s);
    }
    if (damon_is_last_region(r, t)) {
    s.stat.nr_snapshots += 1;
    }
    }
    }
//
// damos_apply_target() - Apply DAMOS schemes to a given target.
// @c:			monitoring context to apply its DAMOS schemes to..
// @t:			monitoring target to apply the schemes to.
// @max_region_sz:	maximum region size for @c.
//
// This function could split regions for keeping the quota.  To minimize
// overhead from the split operations increased number of regions, this
// function will also merge regions after the schemes applying attempt is done,
// for each region.  The merge operation is made only when it doesn't lose the
// monitoring information and not violating @max_region_sz.
//
// Hence, after this function is called, the total number of regions could
// be increased or reduced.  The increase could make max_nr_regions temporarily
// be violated, until the next per-aggregation interval regions merge operation
// is executed.  The decrease will not violate min_nr_regions though, since it
// keeps @max_region_sz.
//
#[no_mangle]
pub unsafe extern "C" fn damos_apply_target(c: *mut damon_ctx, t: *mut damon_target, max_region_sz: c_ulong) {
pub static mut r: *mut c_void = core::ptr::null_mut();
    damon_for_each_region(r, t) {
pub static mut prev_r: *mut c_void = core::ptr::null_mut();
    damon_do_apply_schemes(c, t, r);
//
// damon_do_apply_scheems() could split the region for the
// quota.  Keeping the new slices is an overhead.  Merge back
// the slices into the previous region if it doesn't lose any
// information and not violating the max_region_sz.
//
    if (damon_first_region(t) == r) {
    continue;
    }
    prev_r = damon_prev_region(r);
    if (prev_r.ar.end != r.ar.start) {
    continue;
    }
    if (prev_r.age != r.age) {
    continue;
    }
    if (prev_r.last_nr_accesses != r.last_nr_accesses) {
    continue;
    }
    if (prev_r.nr_accesses != r.nr_accesses) {
    continue;
    }
    if (r.ar.end - prev_r.ar.start > max_region_sz) {
    continue;
    }
    prev_r.ar.end = r.ar.end;
    damon_destroy_region(r, t);
    r = prev_r;
    }
    }
//
// damon_feed_loop_next_input() - get next input to achieve a target score.
// @last_input	The last input.
// @score	Current score that made with @last_input.
//
// Calculate next input to achieve the target score, based on the last input
// and current score.  Assuming the input and the score are positively
// proportional, calculate how much compensation should be added to or
// subtracted from the last input as a proportion of the last input.  Avoid
// next input always being zero by setting it non-zero always.  In short form
// (assuming support of float and signed calculations), the algorithm is as
// below.
//
// next_input = max(last_input * ((goal - current) / goal + 1), 1)
//
// For simple implementation, we assume the target score is always 10,000.  The
// caller should adjust @score for this.
//
// Returns next input that assumed to achieve the target score.
//
#[no_mangle]
pub unsafe extern "C" fn damon_feed_loop_next_input(last_input: c_ulong, score: c_ulong) -> c_ulong {
pub static mut goal: c_ulong = 10000;
// Set minimum input as 10000 to avoid compensation be zero
pub static mut min_input: c_ulong = 10000;
    unsigned long score_goal_diff, compensation;
pub static mut over_achieving: bool = false;
    if (score == goal) {
    return last_input;
    }
    if (score >= goal * 2) {
    return min_input;
    }
    if (over_achieving) {
    score_goal_diff = score - goal;
    }
    else {
    score_goal_diff = goal - score;
    }
    if (last_input < ULONG_MAX / score_goal_diff) {
    compensation = last_input * score_goal_diff / goal;
    }
    else {
    compensation = last_input / goal * score_goal_diff;
    }
    if (over_achieving) {
    return max(last_input - compensation, min_input);
    }
    if (last_input < ULONG_MAX - compensation) {
    return last_input + compensation;
    }
    return ULONG_MAX;
    }

#[no_mangle]
unsafe extern "C" fn damos_get_some_mem_psi_total() -> u64 {
    if (static_branch_likely(&psi_disabled)) {
    return 0;
    }
    return div_u64(psi_system.total[PSI_AVGS][PSI_MEM * 2],
    NSEC_PER_USEC);
    }

#[no_mangle]
pub unsafe extern "C" fn damos_get_some_mem_psi_total() -> u64 {
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn invalid_mem_node(nid: c_int) -> bool {
    return nid < 0 || nid >= MAX_NUMNODES || !node_state(nid, N_MEMORY);
    }
    static __kernel_ulong_t damos_get_node_mem_bp(damos_quota_goal *goal)
    {
pub static mut i: usize = 0;
    let mut numerator;
    if (invalid_mem_node(goal.nid)) {
    if (goal.metric == DAMOS_QUOTA_NODE_MEM_USED_BP) {
    return 0;
    }
    else	/* DAMOS_QUOTA_NODE_MEM_FREE_BP */
    return 10000;
    }
    si_meminfo_node(&i, goal.nid);
    if (goal.metric == DAMOS_QUOTA_NODE_MEM_USED_BP) {
    numerator = i.totalram - i.freeram;
    }
    else	/* DAMOS_QUOTA_NODE_MEM_FREE_BP */
    numerator = i.freeram;
    return mult_frac(numerator, 10000, i.totalram);
    }
#[no_mangle]
pub unsafe extern "C" fn damos_get_node_memcg_used_bp(goal: *mut damos_quota_goal) -> c_ulong {
pub static mut memcg: *mut c_void = core::ptr::null_mut();
pub static mut lruvec: *mut c_void = core::ptr::null_mut();
    unsigned long used_pages, numerator;
pub static mut i: usize = 0;
    if (invalid_mem_node(goal.nid)) {
    if (goal.metric == DAMOS_QUOTA_NODE_MEMCG_USED_BP) {
    return 0;
    }
    else	/* DAMOS_QUOTA_NODE_MEMCG_FREE_BP */
    return 10000;
    }
    memcg = mem_cgroup_get_from_id(goal.memcg_id);
    if (!memcg) {
    if (goal.metric == DAMOS_QUOTA_NODE_MEMCG_USED_BP) {
    return 0;
    }
    else	/* DAMOS_QUOTA_NODE_MEMCG_FREE_BP */
    return 10000;
    }
    mem_cgroup_flush_stats(memcg);
    lruvec = mem_cgroup_lruvec(memcg, NODE_DATA(goal.nid));
    used_pages = lruvec_page_state(lruvec, NR_ACTIVE_ANON);
    used_pages += lruvec_page_state(lruvec, NR_INACTIVE_ANON);
    used_pages += lruvec_page_state(lruvec, NR_ACTIVE_FILE);
    used_pages += lruvec_page_state(lruvec, NR_INACTIVE_FILE);
    mem_cgroup_put(memcg);
    si_meminfo_node(&i, goal.nid);
    if (goal.metric == DAMOS_QUOTA_NODE_MEMCG_USED_BP) {
    numerator = used_pages;
    }
    else	/* DAMOS_QUOTA_NODE_MEMCG_FREE_BP */
    numerator = i.totalram - used_pages;
    return mult_frac(numerator, 10000, i.totalram);
    }

//
// damos_calc_eligible_bytes() - Calculate raw eligible bytes per node.
// @c:		The DAMON context.
// @s:		The scheme.
// @nid:	The target NUMA node id.
// @total:	Output for total eligible bytes across all nodes.
//
// Iterates through each folio in eligible regions to accurately determine
// which node the memory resides on. Returns eligible bytes on the specified
// node and sets *total to the sum across all nodes.
//
// Note: This function requires damon_get_folio() from ops-common.c, which is
// only available when CONFIG_DAMON_PADDR is enabled. It also requires the
// context to be using PADDR operations for meaningful results.
//
    static phys_addr_t damos_calc_eligible_bytes(damon_ctx *c, damos *s, int nid, phys_addr_t *total)
    {
pub static mut t: *mut c_void = core::ptr::null_mut();
pub static mut r: *mut c_void = core::ptr::null_mut();
pub static mut total_eligible: phys_addr_t = 0;
pub static mut node_eligible: phys_addr_t = 0;
    damon_for_each_target(t, c) {
    damon_for_each_region(r, t) {
    phys_addr_t addr, end_addr;
    if (!__damos_valid_target(r, s, c)) {
    continue;
    }
// Convert from core address units to physical bytes
    addr = (phys_addr_t)r.ar.start * c.addr_unit;
    end_addr = (phys_addr_t)r.ar.end * c.addr_unit;
    while (addr < end_addr) {
pub static mut folio: *mut c_void = core::ptr::null_mut();
    phys_addr_t folio_start, folio_end;
    phys_addr_t overlap_start, overlap_end;
    let mut counted;
    folio = damon_get_folio(PHYS_PFN(addr));
    if (!folio) {
    addr = PAGE_ALIGN_DOWN(addr +
    PAGE_SIZE);
    if (!addr) {
    break;
    }
    continue;
    }
//
// Calculate exact overlap between the region
// [addr, end_addr) and the folio range.
// The folio may start before addr if addr is
// in the middle of a large folio.
//
    folio_start = PFN_PHYS(folio_pfn(folio));
    folio_end = folio_start + folio_size(folio);
    overlap_start = max(addr, folio_start);
    overlap_end = min(end_addr, folio_end);
    if (overlap_end > overlap_start) {
    counted = overlap_end - overlap_start;
    total_eligible += counted;
    if (folio_nid(folio) == nid) {
    node_eligible += counted;
    }
    }
// Advance past the entire folio
    addr = folio_end;
    folio_put(folio);
    }
    cond_resched();
    }
    }
// total = total_eligible;
    return node_eligible;
    }
#[no_mangle]
pub unsafe extern "C" fn damos_get_node_eligible_mem_bp(c: *mut damon_ctx, s: *mut damos, nid: c_int) -> c_ulong {
pub static mut total_eligible: phys_addr_t = 0;
    let mut node_eligible;
    if (c.ops.id != DAMON_OPS_PADDR) {
    return 0;
    }
    if (nid < 0 || nid >= MAX_NUMNODES || !node_online(nid)) {
    return 0;
    }
    node_eligible = damos_calc_eligible_bytes(c, s, nid, &total_eligible);
    if (!(unsigned long)total_eligible) {
    return 0;
    }
    return mult_frac((unsigned long)node_eligible, 10000,
    (unsigned long)total_eligible);
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: damos_get_node_eligible_mem_bp
pub unsafe extern "C" fn damos_get_node_eligible_mem_bp_dup(c: *mut damon_ctx, s: *mut damos, nid: c_int) -> c_ulong {
    return 0;
    }

    static __kernel_ulong_t damos_get_node_mem_bp(damos_quota_goal *goal)
    {
    return 0;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: damos_get_node_memcg_used_bp
pub unsafe extern "C" fn damos_get_node_memcg_used_bp_dup(goal: *mut damos_quota_goal) -> c_ulong {
    return 0;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: damos_get_node_eligible_mem_bp
pub unsafe extern "C" fn damos_get_node_eligible_mem_bp_dup(c: *mut damon_ctx, s: *mut damos, nid: c_int) -> c_ulong {
    return 0;
    }

//
// Returns LRU-active or inactive memory to total LRU memory size ratio.
//
#[no_mangle]
unsafe extern "C" fn damos_get_in_active_mem_bp(active_ratio: bool) -> c_uint {
    unsigned long active, inactive, total;
// This should align with /proc/meminfo output
    active = global_node_page_state(NR_LRU_BASE + LRU_ACTIVE_ANON) +
    global_node_page_state(NR_LRU_BASE + LRU_ACTIVE_FILE);
    inactive = global_node_page_state(NR_LRU_BASE + LRU_INACTIVE_ANON) +
    global_node_page_state(NR_LRU_BASE + LRU_INACTIVE_FILE);
    total = active + inactive;
    if (active_ratio) {
    return mult_frac(active, 10000, total);
    }
    return mult_frac(inactive, 10000, total);
    }
#[no_mangle]
pub unsafe extern "C" fn damos_set_quota_goal_current_value(c: *mut damon_ctx, s: *mut damos, goal: *mut damos_quota_goal) {
    let mut now_psi_total = 0;
    match (goal.metric) {
    DAMOS_QUOTA_USER_INPUT => {
// User should already set goal->current_value
    // break;
    }
    DAMOS_QUOTA_SOME_MEM_PSI_US => {
    now_psi_total = damos_get_some_mem_psi_total();
    goal.current_value = now_psi_total - goal.last_psi_total;
    goal.last_psi_total = now_psi_total;
    // break;
    }
    DAMOS_QUOTA_NODE_MEM_USED_BP => {
    }
    DAMOS_QUOTA_NODE_MEM_FREE_BP => {
    goal.current_value = damos_get_node_mem_bp(goal);
    // break;
    }
    DAMOS_QUOTA_NODE_MEMCG_USED_BP => {
    }
    DAMOS_QUOTA_NODE_MEMCG_FREE_BP => {
    goal.current_value = damos_get_node_memcg_used_bp(goal);
    // break;
    }
    DAMOS_QUOTA_ACTIVE_MEM_BP => {
    }
    DAMOS_QUOTA_INACTIVE_MEM_BP => {
    goal.current_value = damos_get_in_active_mem_bp(
    goal.metric == DAMOS_QUOTA_ACTIVE_MEM_BP);
    // break;
    }
    DAMOS_QUOTA_NODE_ELIGIBLE_MEM_BP => {
    goal.current_value = damos_get_node_eligible_mem_bp(c, s,
    goal.nid);
    // break;
    }
    _ => {
    // break;
    }
    }
    }
// Return the highest score since it makes schemes least aggressive
#[no_mangle]
unsafe extern "C" fn damos_quota_score(c: *mut damon_ctx, s: *mut damos) -> c_ulong {
pub static mut goal: *mut c_void = core::ptr::null_mut();
    let mut quota = &s.quota;
pub static mut highest_score: c_ulong = 0;
    damos_for_each_quota_goal(goal, quota) {
    damos_set_quota_goal_current_value(c, s, goal);
    highest_score = max(highest_score,
    mult_frac(goal.current_value, 10000,
    goal.target_value));
    }
    return highest_score;
    }
#[no_mangle]
unsafe extern "C" fn damos_goal_tune_esz_bp_consist(c: *mut damon_ctx, s: *mut damos) {
    let mut quota = &s.quota;
pub static mut score: c_ulong = 0;
    quota.esz_bp = damon_feed_loop_next_input(
    max(quota.esz_bp, 10000UL), score);
    }
#[no_mangle]
pub unsafe extern "C" fn damos_goal_tune_esz_bp_temporal(c: *mut damon_ctx, s: *mut damos) {
    let mut quota = &s.quota;
pub static mut score: c_ulong = 0;
    if (score >= 10000) {
    quota.esz_bp = 0;
    }

    else if (quota.sz) {
    quota.esz_bp = quota.sz * 10000;
    }
    else {
    quota.esz_bp = ULONG_MAX;
    }
    }
//
// Called only if quota->ms, or quota->sz are set, or quota->goals is not empty
//
#[no_mangle]
unsafe extern "C" fn damos_set_effective_quota(ctx: *mut damon_ctx, s: *mut damos) {
    let mut quota = &s.quota;
    let mut throughput = 0;
pub static mut esz: c_ulong = 0;
    if (!quota.ms && list_empty(&quota.goals)) {
    quota.esz = quota.sz;
    return;
    }
    if (!list_empty(&quota.goals)) {
    if (quota.goal_tuner == DAMOS_QUOTA_GOAL_TUNER_CONSIST) {
    damos_goal_tune_esz_bp_consist(ctx, s);
    }

    else if (quota.goal_tuner == DAMOS_QUOTA_GOAL_TUNER_TEMPORAL) {
    damos_goal_tune_esz_bp_temporal(ctx, s);
    }
    esz = quota.esz_bp / 10000;
    }
    if (quota.ms) {
    if (quota.total_charged_ns) {
    throughput = mult_frac(quota.total_charged_sz,
    1000000, quota.total_charged_ns);
    }
    else {
    throughput = PAGE_SIZE * 1024;
    }
    esz = min(throughput * quota.ms, esz);
    esz = max(ctx.min_region_sz, esz);
    }
    if (quota.sz && quota.sz < esz) {
    esz = quota.sz;
    }
    quota.esz = esz;
    }
#[no_mangle]
pub unsafe extern "C" fn damos_trace_esz(c: *mut damon_ctx, s: *mut damos, quota: *mut damos_quota) {
pub static mut cidx: c_uint = 0;
pub static mut siter: *mut c_void = core::ptr::null_mut();
    damon_for_each_scheme(siter, c) {
    if (siter == s) {
    break;
    }
    sidx += 1;
    }
    trace_damos_esz(cidx, sidx, quota.esz);
    }
#[no_mangle]
unsafe extern "C" fn damos_adjust_quota(c: *mut damon_ctx, s: *mut damos) {
    let mut quota = &s.quota;
pub static mut t: *mut c_void = core::ptr::null_mut();
pub static mut r: *mut c_void = core::ptr::null_mut();
    unsigned long cumulated_sz, cached_esz;
    unsigned int score, max_score = 0;
    if (!quota.ms && !quota.sz && list_empty(&quota.goals)) {
    return;
    }
// First charge window
    if (!quota.total_charged_sz && !quota.charged_from) {
    quota.charged_from = jiffies;
    damos_set_effective_quota(c, s);
    if (trace_damos_esz_enabled()) {
    damos_trace_esz(c, s, quota);
    }
    }
// New charge window starts
    if (!time_in_range_open(jiffies, quota.charged_from,
    quota.charged_from +
    msecs_to_jiffies(quota.reset_interval))) {
    if (damos_quota_is_full(quota, c.min_region_sz)) {
    s.stat.qt_exceeds += 1;
    }
    quota.total_charged_sz += quota.charged_sz;
    quota.charged_from = jiffies;
    quota.charged_sz = 0;
    if (trace_damos_esz_enabled()) {
    cached_esz = quota.esz;
    }
    damos_set_effective_quota(c, s);
    if (trace_damos_esz_enabled() && quota.esz != cached_esz) {
    damos_trace_esz(c, s, quota);
    }
    }
    if (!c.ops.get_scheme_score) {
    return;
    }
// Fill up the score histogram
    memset(c.regions_score_histogram, 0,
    sizeof!(*c.regions_score_histogram) *
    (DAMOS_MAX_SCORE + 1));
    damon_for_each_target(t, c) {
    damon_for_each_region(r, t) {
    if (!__damos_valid_target(r, s, c)) {
    continue;
    }
    if (damos_core_filter_out(c, t, r, s)) {
    continue;
    }
    score = c.ops.get_scheme_score(c, r, s);
    c.regions_score_histogram[score] +=
    damon_sz_region(r);
    if (score > max_score) {
    max_score = score;
    }
    }
    }
// Set the min score limit
    while ( ) {
    cumulated_sz += c.regions_score_histogram[score];
    if (cumulated_sz >= quota.esz || !score) {
    break;
    }
    }
    quota.min_score = score;
    }
#[no_mangle]
unsafe extern "C" fn damos_trace_stat(c: *mut damon_ctx, s: *mut damos) {
pub static mut cidx: c_uint = 0;
pub static mut siter: *mut c_void = core::ptr::null_mut();
    if (!trace_damos_stat_after_apply_interval_enabled()) {
    return;
    }
    damon_for_each_scheme(siter, c) {
    if (siter == s) {
    break;
    }
    sidx += 1;
    }
    trace_call__damos_stat_after_apply_interval(cidx, sidx, &s.stat);
    }
#[no_mangle]
unsafe extern "C" fn kdamond_apply_schemes(c: *mut damon_ctx) {
pub static mut t: *mut c_void = core::ptr::null_mut();
pub static mut s: *mut c_void = core::ptr::null_mut();
pub static mut has_schemes_to_apply: bool = false;
    let mut max_region_sz = 0;
    damon_for_each_scheme(s, c) {
    if (time_before(c.passed_sample_intervals, s.next_apply_sis)) {
    continue;
    }
    if (!s.wmarks.activated) {
    continue;
    }
    has_schemes_to_apply = true;
    damos_adjust_quota(c, s);
    }
    if (!has_schemes_to_apply) {
    return;
    }
    max_region_sz = damon_region_sz_limit(c);
    mutex_lock(&c.walk_control_lock);
    damon_for_each_target(t, c) {
    if (c.ops.target_valid && c.ops.target_valid(t) == false) {
    continue;
    }
    damos_apply_target(c, t, max_region_sz);
    }
    damon_for_each_scheme(s, c) {
    if (time_before(c.passed_sample_intervals, s.next_apply_sis)) {
    continue;
    }
    damos_walk_complete(c, s);
    damos_set_next_apply_sis(s, c);
    s.last_applied = core::ptr::null_mut();
    damos_trace_stat(c, s);
    }
    mutex_unlock(&c.walk_control_lock);
    }

#[no_mangle]
pub unsafe extern "C" fn damon_verify_merge_two_regions(l: *mut damon_region, r: *mut damon_region) {
// damon_merge_two_regions() may created incorrect left region
    WARN_ONCE(l.ar.start >= l.ar.end, "l: %lu-%lu, r: %lu-%lu\n",
    l.ar.start, l.ar.end, r.ar.start, r.ar.end);
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: damon_verify_merge_two_regions
pub unsafe extern "C" fn damon_verify_merge_two_regions_dup(l: *mut damon_region, r: *mut damon_region) {
    }

//
// Merge two adjacent regions into one region
//
#[no_mangle]
pub unsafe extern "C" fn damon_merge_two_regions(t: *mut damon_target, l: *mut damon_region, r: *mut damon_region) {
pub static mut sz_l: c_ulong = 0;
    let mut i = 0;
    l.nr_accesses = (l.nr_accesses * sz_l + r.nr_accesses * sz_r) /
    (sz_l + sz_r);
    l.age = (l.age * sz_l + r.age * sz_r) / (sz_l + sz_r);
    l.ar.end = r.ar.end;
// todo: do this for only installed probes
    for (i = 0; i < DAMON_MAX_PROBES; i++) {
    l.probe_hits[i] = (l.probe_hits[i] * sz_l + r.probe_hits[i]
// sz_r) / (sz_l + sz_r);
    }
    damon_verify_merge_two_regions(l, r);
    damon_destroy_region(r, t);
    }
#[no_mangle]
pub unsafe extern "C" fn damon_merge_score(r: *mut damon_region, last: bool, ctx: *mut damon_ctx, use_probe_hits: bool) -> c_uint {
    if (use_probe_hits) {
    return damon_probe_hits_wsum(r, last, ctx);
    }
    if (last) {
    return r.last_nr_accesses;
    }
    return r.nr_accesses;
    }
//
// Merge adjacent regions having similar access frequencies
//
// t		target affected by this merge operation
// thres	'->nr_accesses' diff threshold for the merge
// sz_limit	size upper limit of each region
//
#[no_mangle]
pub unsafe extern "C" fn damon_merge_regions_of(t: *mut damon_target, thres: c_uint, sz_limit: c_ulong, ctx: *mut damon_ctx, count_age: bool) {
    struct damon_region *r, *prev = core::ptr::null_mut(), *next;
pub static mut use_probe_hits: bool = false;
    damon_for_each_region_safe(r, next, t) {
    let mut score = 0;
    let mut last_score = 0;
    let mut diff = 0;
    score = damon_merge_score(r, false, ctx, use_probe_hits);
    last_score = damon_merge_score(r, true, ctx, use_probe_hits);
    if (count_age) {
    if (abs_diff(score, last_score) > thres) {
    r.age = 0;
    }

    else if ((score == 0) != (last_score == 0)) {
    r.age = 0;
    }
    else {
    r.age += 1;
    }
    }
    if (!prev) {
// goto;
    }
    if (prev.ar.end != r.ar.start) {
// goto;
    }
    diff = abs_diff(score, damon_merge_score(prev, false, ctx,
    use_probe_hits));
    if (diff > thres) {
// goto;
    }
    if (damon_sz_region(prev) + damon_sz_region(r) > sz_limit) {
// goto;
    }
    damon_merge_two_regions(t, prev, r);
    continue;
// label;
    prev = r;
    }
    }
//
// Merge adjacent regions having similar access frequencies
//
// threshold	'->nr_accesses' diff threshold for the merge
// sz_limit	size upper limit of each region
//
// This function merges monitoring target regions which are adjacent and their
// access frequencies are similar.  This is for minimizing the monitoring
// overhead under the dynamically changeable access pattern.  If a merge was
// unnecessarily made, later 'kdamond_split_regions()' will revert it.
//
// The total number of regions could be higher than the user-defined limit,
// max_nr_regions for some cases.  For example, the user can update
// max_nr_regions to a number that lower than the current number of regions
// while DAMON is running.  For such a case, repeat merging until the limit is
// met while increasing @threshold up to possible maximum level.
//
#[no_mangle]
pub unsafe extern "C" fn kdamond_merge_regions(c: *mut damon_ctx, threshold: c_uint, sz_limit: c_ulong) {
pub static mut t: *mut c_void = core::ptr::null_mut();
    let mut nr_regions = 0;
    let mut max_thres = 0;
pub static mut count_age: bool = true;
    max_thres = c.attrs.aggr_interval /
    (c.attrs.sample_interval ?  c.attrs.sample_interval : 1);
    while (true) {
    nr_regions = 0;
    damon_for_each_target(t, c) {
    damon_merge_regions_of(t, threshold, sz_limit, c,
    count_age);
    nr_regions += damon_nr_regions(t);
    }
    count_age = false;
    if (nr_regions <= c.attrs.max_nr_regions ||
    max_thres <= threshold) {
    break;
    }
    if (threshold < max_thres / 2) {
    threshold = max(1, threshold * 2);
    }
    else {
    threshold = max_thres;
    }
    }
    }

#[no_mangle]
pub unsafe extern "C" fn damon_verify_split_region_at(r: *mut damon_region, sz_r: c_ulong) {
    WARN_ONCE(sz_r == 0 || sz_r >= damon_sz_region(r),
    "sz_r: %lu r: %lu-%lu (%lu)\n",
    sz_r, r.ar.start, r.ar.end, damon_sz_region(r));
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: damon_verify_split_region_at
pub unsafe extern "C" fn damon_verify_split_region_at_dup(r: *mut damon_region, sz_r: c_ulong) {
    }

//
// Split a region in two
//
// r		the region to be split
// sz_r		size of the first sub-region that will be made
//
// Return: 0 on success, negative error code otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn damon_split_region_at(t: *mut damon_target, r: *mut damon_region, sz_r: c_ulong) -> c_int {
pub static mut new: *mut c_void = core::ptr::null_mut();
    damon_verify_split_region_at(r, sz_r);
    new = damon_new_region(r.ar.start + sz_r, r.ar.end);
    if (!new) {
    return -ENOMEM;
    }
    r.ar.end = new.ar.start;
    new.age = r.age;
    new.last_nr_accesses = r.last_nr_accesses;
    new.nr_accesses = r.nr_accesses;
// todo: do this for only installed probes
    memcpy(new.probe_hits, r.probe_hits, sizeof!(r.probe_hits));
    memcpy(new.last_probe_hits, r.last_probe_hits,
    sizeof!(r.last_probe_hits));
    damon_insert_region(new, r, damon_next_region(r), t);
    return 0;
    }
// Split every region in the given target into 'nr_subs' regions
#[no_mangle]
pub unsafe extern "C" fn damon_split_regions_of(ctx: *mut damon_ctx, t: *mut damon_target, nr_subs: c_int, min_region_sz: c_ulong) {
    let mut r = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
    unsigned long sz_region, sz_sub = 0;
    let mut i = 0;
    damon_for_each_region_safe(r, next, t) {
    sz_region = damon_sz_region(r);
    while (i < nr_subs - 1 &&
    sz_region > 2 * min_region_sz) {
//
// Randomly select size of left sub-region to be at
// least 10 percent and at most 90% of original region
//
    sz_sub = ALIGN_DOWN(damon_rand(ctx, 1, 10) *
    sz_region / 10, min_region_sz);
// Do not allow blank region
    if (sz_sub == 0 || sz_sub >= sz_region) {
    continue;
    }
    damon_split_region_at(t, r, sz_sub);
    sz_region = sz_sub;
    }
    }
    }
// Split one in every @split_step regions into two, from a rotating offset
#[no_mangle]
pub unsafe extern "C" fn damon_split_some_regions(ctx: *mut damon_ctx, split_step: c_ulong) {
    static unsigned long rotation;
pub static mut t: *mut c_void = core::ptr::null_mut();
    let mut r = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
pub static mut offset: c_ulong = 0;
pub static mut idx: c_ulong = 0;
    damon_for_each_target(t, ctx) {
    damon_for_each_region_safe(r, next, t) {
    unsigned long sz_region, sz_sub;
    if (idx++ % split_step != offset) {
    continue;
    }
    sz_region = damon_sz_region(r);
    if (sz_region < 2 * ctx.min_region_sz) {
    continue;
    }
    sz_sub = ALIGN_DOWN(damon_rand(ctx, 1, 10) *
    sz_region / 10, ctx.min_region_sz);
// Do not allow blank region
    if (sz_sub == 0 || sz_sub >= sz_region) {
    continue;
    }
    damon_split_region_at(t, r, sz_sub);
    }
    }
    }
//
// Split every target region into randomly-sized small regions
//
// This function splits every target region into random-sized small regions if
// current total number of the regions is equal or smaller than half of the
// user-specified maximum number of regions.  This is for maximizing the
// monitoring accuracy under the dynamically changeable access patterns.  If a
// split was unnecessarily made, later 'kdamond_merge_regions()' will revert
// it.
//
#[no_mangle]
unsafe extern "C" fn kdamond_split_regions(ctx: *mut damon_ctx) {
pub static mut t: *mut c_void = core::ptr::null_mut();
pub static mut nr_regions: c_ulong = 0;
pub static mut max_nr_regions: c_ulong = 0;
    static unsigned long last_nr_regions;
pub static mut nr_subregions: c_int = 2;
    damon_for_each_target(t, ctx)
    nr_regions += damon_nr_regions(t);
    if (nr_regions >= max_nr_regions) {
// goto;
    }
    if (nr_regions > max_nr_regions / 2) {
    damon_split_some_regions(ctx,
    max_nr_regions / (max_nr_regions - nr_regions));
// goto;
    }
// Maybe the middle of the region has different access frequency
    if (last_nr_regions == nr_regions &&
    nr_regions < max_nr_regions / 3) {
    nr_subregions = 3;
    }
    damon_for_each_target(t, ctx)
    damon_split_regions_of(ctx, t, nr_subregions,
    ctx.min_region_sz);
// label;
    last_nr_regions = nr_regions;
    }
//
// Check whether current monitoring should be stopped
//
// The monitoring is stopped when either the user requested to stop, or all
// monitoring targets are invalid.
//
// Returns true if need to stop current monitoring.
//
#[no_mangle]
unsafe extern "C" fn kdamond_need_stop(ctx: *mut damon_ctx) -> bool {
pub static mut t: *mut c_void = core::ptr::null_mut();
    if (kthread_should_stop()) {
    return true;
    }
    if (!ctx.ops.target_valid) {
    return false;
    }
    damon_for_each_target(t, ctx) {
    if (ctx.ops.target_valid(t)) {
    return false;
    }
    }
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn damos_get_wmark_metric_value(metric: damos_wmark_metric, metric_value: *mut c_ulong) -> c_int {
    match (metric) {
    DAMOS_WMARK_FREE_MEM_RATE => {
// metric_value = global_zone_page_state(NR_FREE_PAGES) * 1000
    totalram_pages();
    return 0;
    }
    _ => {
    // break;
    }
    }
    return -EINVAL;
    }
//
// Returns zero if the scheme is active.  Else, returns time to wait for next
// watermark check in micro-seconds.
//
#[no_mangle]
unsafe extern "C" fn damos_wmark_wait_us(scheme: *mut damos) -> c_ulong {
    let mut metric = 0;
    if (damos_get_wmark_metric_value(scheme.wmarks.metric, &metric)) {
    return 0;
    }
// higher than high watermark or lower than low watermark
    if (metric > scheme.wmarks.high || scheme.wmarks.low > metric) {
    if (scheme.wmarks.activated) {
    pr_debug!("deactivate a scheme (%d) for %s wmark\n",
    scheme.action,
    str_high_low(metric > scheme.wmarks.high));
    }
    scheme.wmarks.activated = false;
    return scheme.wmarks.interval;
    }
// inactive and higher than middle watermark
    if ((scheme.wmarks.high >= metric && metric >= scheme.wmarks.mid) &&
    !scheme.wmarks.activated) {
    return scheme.wmarks.interval;
    }
    if (!scheme.wmarks.activated) {
    pr_debug!("activate a scheme (%d)\n", scheme.action);
    }
    scheme.wmarks.activated = true;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn kdamond_usleep(usecs: c_ulong) {
    if (usecs >= USLEEP_RANGE_UPPER_BOUND) {
    schedule_timeout_idle(usecs_to_jiffies(usecs));
    }
    else {
    usleep_range_idle(usecs, usecs + 1);
    }
    }

#[no_mangle]
unsafe extern "C" fn damon_verify_ctx(c: *mut damon_ctx) {
pub static mut t: *mut c_void = core::ptr::null_mut();
pub static mut r: *mut c_void = core::ptr::null_mut();
    damon_for_each_target(t, c) {
    let mut prev_r = core::ptr::null_mut();
pub static mut nr_regions: c_uint = 0;
    damon_for_each_region(r, t) {
    WARN_ONCE(r.ar.start >= r.ar.end,
    "region start (%lu) >= end (%lu)\n",
    r.ar.start, r.ar.end);
    WARN_ONCE(prev_r && prev_r.ar.end > r.ar.start,
    "region overlap (%lu > %lu)\n",
    prev_r.ar.end, r.ar.start);
    prev_r = r;
    nr_regions += 1;
    }
    WARN_ONCE(damon_nr_regions(t) != nr_regions,
    "nr_regions mismatch: %u != %u\n",
    damon_nr_regions(t), nr_regions);
    }
    }

#[no_mangle]
unsafe extern "C" fn damon_verify_ctx(c: *mut damon_ctx) {
    }

//
// kdamond_call() - handle damon_call_control objects.
// @ctx:	The &struct damon_ctx of the kdamond.
// @cancel:	Whether to cancel the invocation of the function.
//
// If there are &struct damon_call_control requests that registered via
// &damon_call() on @ctx, do or cancel the invocation of the function depending
// on @cancel.  @cancel is set when the kdamond is already out of the main loop
// and therefore will be terminated.
//
#[no_mangle]
unsafe extern "C" fn kdamond_call(ctx: *mut damon_ctx, cancel: bool) {
    let mut control = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
pub static mut controls: usize = 0;
    damon_verify_ctx(ctx);
    mutex_lock(&ctx.call_controls_lock);
    list_splice_tail_init(&ctx.call_controls, &controls);
    mutex_unlock(&ctx.call_controls_lock);
    list_for_each_entry_safe(control, next, &controls, list) {
    if (!control.repeat || cancel) {
    list_del(&control.list);
    }
    if (cancel) {
    control.canceled = true;
    }
    else {
    control.return_code = control.fn(control.data);
    }
    if (!control.repeat) {
    complete(&control.completion);
    }

    else if (control.canceled && control.dealloc_on_cancel) {
    kfree(control);
    }
    if (!cancel && ctx.maybe_corrupted) {
    break;
    }
    }
    mutex_lock(&ctx.call_controls_lock);
    list_splice_tail(&controls, &ctx.call_controls);
    mutex_unlock(&ctx.call_controls_lock);
    }
// Returns negative error code if it's not activated but should return
#[no_mangle]
unsafe extern "C" fn kdamond_wait_activation(ctx: *mut damon_ctx) -> c_int {
pub static mut s: *mut c_void = core::ptr::null_mut();
    let mut wait_time = 0;
pub static mut min_wait_time: c_ulong = 0;
pub static mut init_wait_time: bool = false;
    while (!kdamond_need_stop(ctx)) {
    damon_for_each_scheme(s, ctx) {
    wait_time = damos_wmark_wait_us(s);
    if (!init_wait_time || wait_time < min_wait_time) {
    init_wait_time = true;
    min_wait_time = wait_time;
    }
    }
    if (!min_wait_time) {
    return 0;
    }
    kdamond_usleep(min_wait_time);
    kdamond_call(ctx, false);
    if (ctx.maybe_corrupted) {
    return -EINVAL;
    }
    damos_walk_cancel(ctx);
    }
    return -EBUSY;
    }
#[no_mangle]
unsafe extern "C" fn kdamond_init_ctx(ctx: *mut damon_ctx) {
    let mut sample_interval = ctx.attrs.sample_interval ?
    ctx.attrs.sample_interval : 1;
pub static mut scheme: *mut c_void = core::ptr::null_mut();
    ctx.passed_sample_intervals = 0;
    ctx.next_aggregation_sis = ctx.attrs.aggr_interval / sample_interval;
    ctx.next_ops_update_sis = ctx.attrs.ops_update_interval /
    sample_interval;
    ctx.next_intervals_tune_sis = ctx.next_aggregation_sis *
    ctx.attrs.intervals_goal.aggrs;
    damon_for_each_scheme(scheme, ctx) {
    damos_set_next_apply_sis(scheme, ctx);
    damos_set_filters_default_reject(scheme);
    }
    }
//
// The monitoring daemon that runs as a kernel thread
//
#[no_mangle]
unsafe extern "C" fn kdamond_fn(data: *mut c_void) -> c_int {
    let mut ctx = data;
pub static mut sz_limit: c_ulong = 0;
    pr_debug!("kdamond (%d) starts\n", current.pid);
    mutex_lock(&ctx.call_controls_lock);
    ctx.call_controls_obsolete = false;
    mutex_unlock(&ctx.call_controls_lock);
    mutex_lock(&ctx.walk_control_lock);
    ctx.walk_control_obsolete = false;
    mutex_unlock(&ctx.walk_control_lock);
    complete(&ctx.kdamond_started);
    kdamond_init_ctx(ctx);
    if (ctx.ops.init) {
    ctx.ops.init(ctx);
    }
    ctx.regions_score_histogram = kmalloc_array(DAMOS_MAX_SCORE + 1,
    sizeof!(*ctx.regions_score_histogram), GFP_KERNEL);
    if (!ctx.regions_score_histogram) {
// goto;
    }
    sz_limit = damon_apply_min_nr_regions(ctx);
    while (!kdamond_need_stop(ctx)) {
//
// ctx->attrs and ctx->next_{aggregation,ops_update}_sis could
// be changed from kdamond_call().  Read the values here, and
// use those for this iteration.  That is, damon_set_attrs()
// updated new values are respected from next iteration.
//
pub static mut next_aggregation_sis: c_ulong = 0;
pub static mut next_ops_update_sis: c_ulong = 0;
pub static mut sample_interval: c_ulong = 0;
pub static mut access_check_disabled: bool = false;
pub static mut max_merge_score: c_uint = 0;
    let mut get_max_wsum = 0;
    if (kdamond_wait_activation(ctx)) {
    break;
    }
    if (!access_check_disabled && ctx.ops.prepare_access_checks) {
    ctx.ops.prepare_access_checks(ctx);
    }
    kdamond_usleep(sample_interval);
    ctx.passed_sample_intervals += 1;
    if (!access_check_disabled && ctx.ops.check_accesses) {
    max_merge_score = ctx.ops.check_accesses(ctx);
    }
    if (ctx.ops.apply_probes) {
    if (time_after_eq(ctx.passed_sample_intervals,
    next_aggregation_sis) &&
    access_check_disabled) {
    get_max_wsum = true;
    }
    else {
    get_max_wsum = false;
    }
    max_wsum = ctx.ops.apply_probes(ctx,
    access_check_disabled, get_max_wsum);
    if (get_max_wsum) {
    max_merge_score = max_wsum;
    }
    }
    if (time_after_eq(ctx.passed_sample_intervals,
    next_aggregation_sis)) {
    kdamond_merge_regions(ctx,
    max_merge_score / 10,
    sz_limit);
// online updates might be made
    sz_limit = damon_apply_min_nr_regions(ctx);
    }
//
// do kdamond_call() and kdamond_apply_schemes() after
// kdamond_merge_regions() if possible, to reduce overhead
//
    kdamond_call(ctx, false);
    if (ctx.maybe_corrupted) {
    break;
    }
    while (ctx.pause) {
    damos_walk_cancel(ctx);
    kdamond_usleep(ctx.attrs.sample_interval);
// allow caller unset pause via damon_call()
    kdamond_call(ctx, false);
    if (kdamond_need_stop(ctx) || ctx.maybe_corrupted) {
// goto;
    }
    }
    if (!list_empty(&ctx.schemes)) {
    kdamond_apply_schemes(ctx);
    }
    else {
    damos_walk_cancel(ctx);
    }
    sample_interval = ctx.attrs.sample_interval ?
    ctx.attrs.sample_interval : 1;
    if (time_after_eq(ctx.passed_sample_intervals,
    next_aggregation_sis)) {
    if (ctx.attrs.intervals_goal.aggrs &&
    time_after_eq(
    ctx.passed_sample_intervals,
    ctx.next_intervals_tune_sis)) {
//
// ctx->next_aggregation_sis might be updated
// from kdamond_call().  In the case,
// damon_set_attrs() which will be called from
// kdamond_tune_interval() may wrongly think
// this is in the middle of the current
// aggregation, and make aggregation
// information reset for all regions.  Then,
// following kdamond_reset_aggregated() call
// will make the region information invalid.
//
// Reset ->next_aggregation_sis to avoid that.
// It will anyway correctly updated after this
// if clause.
//
    ctx.next_aggregation_sis =
    next_aggregation_sis;
    ctx.next_intervals_tune_sis +=
    ctx.attrs.aggr_samples *
    ctx.attrs.intervals_goal.aggrs;
    kdamond_tune_intervals(ctx);
    sample_interval = ctx.attrs.sample_interval ?
    ctx.attrs.sample_interval : 1;
    }
    ctx.next_aggregation_sis = next_aggregation_sis +
    ctx.attrs.aggr_interval / sample_interval;
    kdamond_reset_aggregated(ctx);
    kdamond_split_regions(ctx);
    }
    if (time_after_eq(ctx.passed_sample_intervals,
    next_ops_update_sis)) {
    ctx.next_ops_update_sis = next_ops_update_sis +
    ctx.attrs.ops_update_interval /
    sample_interval;
    if (ctx.ops.update) {
    ctx.ops.update(ctx);
    }
    }
    }
// label;
    damon_destroy_targets(ctx);
    kfree(ctx.regions_score_histogram);
    mutex_lock(&ctx.call_controls_lock);
    ctx.call_controls_obsolete = true;
    mutex_unlock(&ctx.call_controls_lock);
    kdamond_call(ctx, true);
    mutex_lock(&ctx.walk_control_lock);
    ctx.walk_control_obsolete = true;
    mutex_unlock(&ctx.walk_control_lock);
    damos_walk_cancel(ctx);
    pr_debug!("kdamond (%d) finishes\n", current.pid);
    mutex_lock(&ctx.kdamond_lock);
    ctx.kdamond = core::ptr::null_mut();
    mutex_unlock(&ctx.kdamond_lock);
    mutex_lock(&damon_lock);
    nr_running_ctxs -= 1;
    if (!nr_running_ctxs && running_exclusive_ctxs) {
    running_exclusive_ctxs = false;
    }
    mutex_unlock(&damon_lock);
    return 0;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct damon_system_ram_range_walk_arg {
    pub walked: bool,
    pub res: resource,
}

#[no_mangle]
unsafe extern "C" fn damon_system_ram_walk_fn(res: *mut resource, arg: *mut c_void) -> c_int {
    let mut a = arg;
    if (!a.walked) {
    a.walked = true;
    a.res.start = res.start;
    }
    a.res.end = res.end;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn damon_res_to_core_addr(ra: resource_size_t, addr_unit: c_ulong) -> c_ulong {
//
// Use div_u64() for avoiding linking errors related with __udivdi3,
// __aeabi_uldivmod, or similar problems.  This should also improve the
// performance optimization (read div_u64() comment for the detail).
//
    if (sizeof!(ra) == 8 && sizeof!(addr_unit) == 4) {
    return div_u64(ra, addr_unit);
    }
    return ra / addr_unit;
    }
#[no_mangle]
pub unsafe extern "C" fn damon_find_system_rams_range(start: *mut c_ulong, end: *mut c_ulong, addr_unit: c_ulong) -> bool {
pub static mut arg: damon_system_ram_range_walk_arg = 0;
    walk_system_ram_res(0, -1, &arg, damon_system_ram_walk_fn);
    if (!arg.walked) {
    return false;
    }
// start = damon_res_to_core_addr(arg.res.start, addr_unit);
// end = damon_res_to_core_addr(arg.res.end + 1, addr_unit);
    if (*end <= *start) {
    return false;
    }
    return true;
    }
//
// damon_set_region_system_rams_default() - Set the region of the given
// monitoring target as requested, or to cover all 'System RAM' resources.
// @t:		The monitoring target to set the region.
// @start:	The pointer to the start address of the region.
// @end:	The pointer to the end address of the region.
// @addr_unit:	The address unit for the damon_ctx of @t.
// @min_region_sz:	Minimum region size.
//
// This function sets the region of @t as requested by @start and @end.  If the
// values of @start and @end are zero, however, this function finds 'System
// RAM' resources and sets the region to cover all the resource.  In the latter
// case, this function saves the start and the end addresseses of the first and
// the last resources in @start and @end, respectively.
//
// Return: 0 on success, negative error code otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn damon_set_region_system_rams_default(t: *mut damon_target, start: *mut c_ulong, end: *mut c_ulong, addr_unit: c_ulong, min_region_sz: c_ulong) -> c_int {
pub static mut addr_range: usize = 0;
    if (!*start && !*end &&
    !damon_find_system_rams_range(start, end, addr_unit)) {
    return -EINVAL;
    }
    addr_range.start = *start;
    addr_range.end = *end;
    return damon_set_regions(t, &addr_range, 1, min_region_sz);
    }
//
// damon_update_region_access_rate() - Update the access rate of a region.
// @r:		The DAMON region to update for its access check result.
// @accessed:	Whether the region has accessed during last sampling interval.
//
// Update the access rate of a region with the region's last sampling interval
// access check result.
//
// Usually this will be called by &damon_operations->check_accesses callback.
//
#[no_mangle]
pub unsafe extern "C" fn damon_update_region_access_rate(r: *mut damon_region, accessed: bool) {
    if (accessed) {
    r.nr_accesses += 1;
    }
    }
//
// damon_initialized() - Return if DAMON is ready to be used.
//
// Return: true if DAMON is ready to be used, false otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn damon_initialized() -> bool {
    return damon_region_cache != core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn damon_init() -> c_int {
    damon_region_cache = KMEM_CACHE(damon_region, 0);
    if (unlikely(!damon_region_cache)) {
    pr_err!("creating damon_region_cache fails\n");
    return -ENOMEM;
    }
    return 0;
    }
    subsys_initcall!(damon_init);