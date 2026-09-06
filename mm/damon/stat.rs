//! Automatically rewritten from C to Rust
//! Source: mm/damon/stat.c
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
// Shows data access monitoring results in simple metrics.
//

// forward_decl: damon_stat_enabled_store;
// forward_decl: damon_stat_enabled_load;
pub static mut kernel_param_ops: usize = 0;
    static bool enabled  = IS_ENABLED!(
    CONFIG_DAMON_STAT_ENABLED_DEFAULT);
    module_param_cb!(enabled, &enabled_param_ops, core::ptr::null_mut(), 0600);
    MODULE_PARM_DESC(enabled, "Enable of disable DAMON_STAT");
    static unsigned long estimated_memory_bandwidth ;
    module_param!(estimated_memory_bandwidth, ulong, 0400);
    MODULE_PARM_DESC(estimated_memory_bandwidth,
    "Estimated memory bandwidth usage in bytes per second");
pub static mut memory_idle_ms_percentiles: [usize; 101] = [0; 101];
    module_param_array!(memory_idle_ms_percentiles, long, core::ptr::null_mut(), 0400);
    MODULE_PARM_DESC(memory_idle_ms_percentiles,
    "Memory idle time percentiles in milliseconds");
    static unsigned long aggr_interval_us;
    module_param!(aggr_interval_us, ulong, 0400);
    MODULE_PARM_DESC(aggr_interval_us,
    "Current tuned aggregation interval in microseconds");
pub static mut damon_stat_context: *mut c_void = core::ptr::null_mut();
    static unsigned long damon_stat_last_refresh_jiffies;
#[no_mangle]
unsafe extern "C" fn damon_stat_set_estimated_memory_bandwidth(c: *mut damon_ctx) {
pub static mut t: *mut c_void = core::ptr::null_mut();
pub static mut r: *mut c_void = core::ptr::null_mut();
pub static mut access_bytes: c_ulong = 0;
    damon_for_each_target(t, c) {
    damon_for_each_region(r, t)
    access_bytes += (r.ar.end - r.ar.start) *
    r.nr_accesses;
    }
    estimated_memory_bandwidth = access_bytes * USEC_PER_MSEC *
    MSEC_PER_SEC / c.attrs.aggr_interval;
    }
#[no_mangle]
unsafe extern "C" fn damon_stat_idletime(r: *const damon_region) -> c_int {
    if (r.nr_accesses) {
    return -1 * (r.age + 1);
    }
    return r.age + 1;
    }
#[no_mangle]
unsafe extern "C" fn damon_stat_cmp_regions(a: *const c_void, b: *const c_void) -> c_int {
    let mut ra = *a;
    let mut rb = *b;
    return damon_stat_idletime(ra) - damon_stat_idletime(rb);
    }
#[no_mangle]
pub unsafe extern "C" fn damon_stat_sort_regions(c: *mut damon_ctx, sorted_ptr: *mut *mut *mut damon_region, nr_regions_ptr: *mut c_int, total_sz_ptr: *mut c_ulong) -> c_int {
pub static mut t: *mut c_void = core::ptr::null_mut();
pub static mut r: *mut c_void = core::ptr::null_mut();
pub static mut region_pointers: *mut c_void = core::ptr::null_mut();
pub static mut nr_regions: c_uint = 0;
pub static mut total_sz: c_ulong = 0;
    damon_for_each_target(t, c) {
// there is only one target
    region_pointers = kmalloc_objs(*region_pointers,
    damon_nr_regions(t));
    if (!region_pointers) {
    return -ENOMEM;
    }
    damon_for_each_region(r, t) {
    region_pointers[nr_regions++] = r;
    total_sz += r.ar.end - r.ar.start;
    }
    }
    sort(region_pointers, nr_regions, sizeof!(*region_pointers),
    damon_stat_cmp_regions, core::ptr::null_mut());
// sorted_ptr = region_pointers;
// nr_regions_ptr = nr_regions;
// total_sz_ptr = total_sz;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn damon_stat_set_idletime_percentiles(c: *mut damon_ctx) {
    let mut sorted_regions = core::ptr::null_mut();
    let mut region = core::ptr::null_mut();
    let mut nr_regions = 0;
    unsigned long total_sz, accounted_bytes = 0;
    int err, i, next_percentile = 0;
    err = damon_stat_sort_regions(c, &sorted_regions, &nr_regions,
    &total_sz);
    if (err) {
    return;
    }
    while (i < nr_regions) {
    region = sorted_regions[i];
    accounted_bytes += region.ar.end - region.ar.start;
    while (next_percentile <= accounted_bytes * 100 / total_sz) {
    memory_idle_ms_percentiles[next_percentile++] =
    damon_stat_idletime(region) *
    (long)c.attrs.aggr_interval / USEC_PER_MSEC;
    }
    }
    kfree(sorted_regions);
    }
#[no_mangle]
unsafe extern "C" fn damon_stat_damon_call_fn(data: *mut c_void) -> c_int {
    let mut c = data;
// avoid unnecessarily frequent stat update
    if (time_before_eq(jiffies, damon_stat_last_refresh_jiffies +
    secs_to_jiffies(5))) {
    return 0;
    }
    damon_stat_last_refresh_jiffies = jiffies;
    aggr_interval_us = c.attrs.aggr_interval;
    damon_stat_set_estimated_memory_bandwidth(c);
    damon_stat_set_idletime_percentiles(c);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn damon_stat_build_ctx() -> *mut c_void {
pub static mut ctx: *mut c_void = core::ptr::null_mut();
pub static mut attrs: usize = 0;
pub static mut target: *mut c_void = core::ptr::null_mut();
pub static mut start: c_ulong = 0;
    ctx = damon_new_ctx();
    if (!ctx) {
    return core::ptr::null_mut();
    }
    attrs = (damon_attrs) {
    .sample_interval = 5 * USEC_PER_MSEC,
    .aggr_interval = 100 * USEC_PER_MSEC,
    .ops_update_interval = 60 * USEC_PER_MSEC * MSEC_PER_SEC,
    .min_nr_regions = 10,
    .max_nr_regions = 1000,
    };
//
// auto-tune sampling and aggregation interval aiming 4% DAMON-observed
// accesses ratio, keeping sampling interval in [5ms, 10s] range.
//
    attrs.intervals_goal = (damon_intervals_goal) {
    .access_bp = 400, .aggrs = 3,
    .min_sample_us = 5000, .max_sample_us = 10000000,
    };
    if (damon_set_attrs(ctx, &attrs)) {
// goto;
    }
    if (damon_select_ops(ctx, DAMON_OPS_PADDR)) {
// goto;
    }
    target = damon_new_target();
    if (!target) {
// goto;
    }
    damon_add_target(ctx, target);
    if (damon_set_region_system_rams_default(target, &start, &end,
    ctx.addr_unit, ctx.min_region_sz)) {
// goto;
    }
    return ctx;
// label;
    damon_destroy_ctx(ctx);
    return core::ptr::null_mut();
    }
pub static mut damon_call_control: usize = 0;
#[no_mangle]
unsafe extern "C" fn damon_stat_start() -> c_int {
    let mut err = 0;
    if (damon_stat_context) {
    if (damon_is_running(damon_stat_context)) {
    return -EAGAIN;
    }
    damon_destroy_ctx(damon_stat_context);
    }
    damon_stat_context = damon_stat_build_ctx();
    if (!damon_stat_context) {
    return -ENOMEM;
    }
    err = damon_start(&damon_stat_context, 1, true);
    if (err) {
    damon_destroy_ctx(damon_stat_context);
    damon_stat_context = core::ptr::null_mut();
    return err;
    }
    damon_stat_last_refresh_jiffies = jiffies;
    call_control.data = damon_stat_context;
    return damon_call(damon_stat_context, &call_control);
    }
#[no_mangle]
unsafe extern "C" fn damon_stat_stop() {
    damon_stop(&damon_stat_context, 1);
    damon_destroy_ctx(damon_stat_context);
    damon_stat_context = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn damon_stat_enabled() -> bool {
    if (!damon_stat_context) {
    return false;
    }
    return damon_is_running(damon_stat_context);
    }
#[no_mangle]
pub unsafe extern "C" fn damon_stat_enabled_store(val: *mut c_char, kp: *mut kernel_param) -> c_int {
    let mut err = 0;
    err = kstrtobool(val, &enabled);
    if (err) {
    return err;
    }
    if (damon_stat_enabled() == enabled) {
    return 0;
    }
    if (!damon_initialized()) {
//
// probably called from command line parsing (parse_args()).
// Cannot call damon_new_ctx().  Let damon_stat_init() handle.
//
    return 0;
    }
    if (enabled) {
    return damon_stat_start();
    }
    damon_stat_stop();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn damon_stat_enabled_load(buffer: *mut c_char, kp: *const kernel_param) -> c_int {
    return sprintf(buffer, "%c\n", damon_stat_enabled() ? 'Y' : 'N');
    }
#[no_mangle]
pub unsafe extern "C" fn damon_stat_kdamond_pid_store(val: *mut c_char, kp: *mut kernel_param) -> c_int {
//
// kdamond_pid is read-only, but kernel command line could write it.
// Do nothing here.
//
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn damon_stat_kdamond_pid_load(buffer: *mut c_char, kp: *mut kernel_param) -> c_int {
    let mut pid = 0;
    if (!damon_stat_context) {
    pid = -1;
    } else {
    pid = damon_kdamond_pid(damon_stat_context);
    if (pid < 1) {
    pid = -1;
    }
    }
    return sprintf(buffer, "%d\n", pid);
    }
pub static mut kernel_param_ops: usize = 0;
//
// PID of the DAMON thread
//
// If DAMON_STAT is enabled, this becomes the PID of the worker thread.
// Else, -1.
//
    module_param_cb!(kdamond_pid, &kdamond_pid_param_ops, core::ptr::null_mut(), 0400);
    MODULE_PARM_DESC(kdamond_pid, "pid of the kdamond");
#[no_mangle]
unsafe extern "C" fn damon_stat_init() -> c_int {
pub static mut err: c_int = 0;
    if (!damon_initialized()) {
    err = -ENOMEM;
// goto;
    }
// probably set via command line
    if (enabled) {
    err = damon_stat_start();
    }
// label;
    if (err && enabled) {
    enabled = false;
    }
    return err;
    }
    module_init!(damon_stat_init);