//! Automatically rewritten from C to Rust
//! Source: mm/damon/reclaim.c
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
// DAMON-based page reclamation
//

//
// Enable or disable DAMON_RECLAIM.
//
// You can enable DAMON_RCLAIM by setting the value of this parameter as ``Y``.
// Setting it as ``N`` disables DAMON_RECLAIM.  Note that DAMON_RECLAIM could
// do no real monitoring and reclamation due to the watermarks-based activation
// condition.  Refer to below descriptions for the watermarks parameter for
// this.
//
    static bool enabled ;
//
// Make DAMON_RECLAIM reads the input parameters again, except ``enabled``.
//
// Input parameters that updated while DAMON_RECLAIM is running are not applied
// by default.  Once this parameter is set as ``Y``, DAMON_RECLAIM reads values
// of parameters except ``enabled`` again.  Once the re-reading is done, this
// parameter is set as ``N``.  If invalid parameters are found while the
// re-reading, DAMON_RECLAIM will be disabled.
//
    static bool commit_inputs ;
//
// Time threshold for cold memory regions identification in microseconds.
//
// If a memory region is not accessed for this or longer time, DAMON_RECLAIM
// identifies the region as cold, and reclaims.  120 seconds by default.
//
pub static mut : unsigned long min_age = 120000000;
    module_param!(min_age, ulong, 0600);
pub static mut damos_quota: usize = 0;
pub static mut damon_reclaim_quota: usize = 0;
//
// Desired level of memory pressure-stall time in microseconds.
//
// While keeping the caps that set by other quotas, DAMON_RECLAIM automatically
// increases and decreases the effective level of the quota aiming this level of
// memory pressure is incurred.  System-wide ``some`` memory PSI in microseconds
// per quota reset interval (``quota_reset_interval_ms``) is collected and
// compared to this value to see if the aim is satisfied.  Value zero means
// disabling this auto-tuning feature.
//
// Disabled by default.
//
    static unsigned long quota_mem_pressure_us ;
    module_param!(quota_mem_pressure_us, ulong, 0600);
//
// User-specifiable feedback for auto-tuning of the effective quota.
//
// While keeping the caps that set by other quotas, DAMON_RECLAIM automatically
// increases and decreases the effective level of the quota aiming receiving this
// feedback of value ``10,000`` from the user.  DAMON_RECLAIM assumes the feedback
// value and the quota are positively proportional.  Value zero means disabling
// this auto-tuning feature.
//
// Disabled by default.
//
    static unsigned long quota_autotune_feedback ;
    module_param!(quota_autotune_feedback, ulong, 0600);
//
// Auto-tune monitoring intervals.
//
// If this parameter is set as ``Y``, DAMON_RECLAIM automatically tunes DAMON's
// sampling and aggregation intervals.  The auto-tuning aims to capture
// meaningful amount of access events in each DAMON-snapshot, while keeping the
// sampling intervals 5 milliseconds in minimum, and 10 seconds in maximum.
// Setting this as ``N`` disables the auto-tuning.
//
// Disabled by default.
//
    static bool autotune_monitoring_intervals ;
    module_param!(autotune_monitoring_intervals, bool, 0600);
pub static mut damos_watermarks: usize = 0;
pub static mut damon_reclaim_wmarks: usize = 0;
pub static mut damon_attrs: usize = 0;
pub static mut damon_reclaim_mon_attrs: usize = 0;
//
// Start of the target memory region in physical address.
//
// The start physical address of memory region that DAMON_RECLAIM will do work
// against.  By default, the system's entire physical memory is used as the
// region.
//
    static unsigned long monitor_region_start ;
    module_param!(monitor_region_start, ulong, 0600);
//
// End of the target memory region in physical address.
//
// The end physical address of memory region that DAMON_RECLAIM will do work
// against.  By default, the system's entire physical memory is used as the
// region.
//
    static unsigned long monitor_region_end ;
    module_param!(monitor_region_end, ulong, 0600);
//
// Scale factor for DAMON_RECLAIM to ops address conversion.
//
// This parameter must not be set to 0.
//
pub static mut : unsigned long addr_unit = 1;
//
// Skip anonymous pages reclamation.
//
// If this parameter is set as ``Y``, DAMON_RECLAIM does not reclaim anonymous
// pages.  By default, ``N``.
//
    static bool skip_anon ;
    module_param!(skip_anon, bool, 0600);
pub static mut damon_reclaim_stat: usize = 0;
pub static mut damon_reclaim_stat: usize = 0;
pub static mut ctx: *mut c_void = core::ptr::null_mut();
pub static mut target: *mut c_void = core::ptr::null_mut();
#[no_mangle]
pub unsafe extern "C" fn damon_reclaim_new_scheme(aggr_interval: c_ulong) -> *mut c_void {
pub static mut damos_access_pattern: usize = 0;
    return damon_new_scheme(
    &pattern,
// page out those, as soon as found
    DAMOS_PAGEOUT,
// for each aggregation interval
    0,
// under the quota.
    &damon_reclaim_quota,
// (De)activate this according to the watermarks.
    &damon_reclaim_wmarks,
    NUMA_NO_NODE);
    }
#[no_mangle]
unsafe extern "C" fn damon_reclaim_apply_parameters() -> c_int {
pub static mut param_ctx: *mut c_void = core::ptr::null_mut();
pub static mut param_target: *mut c_void = core::ptr::null_mut();
pub static mut attrs: usize = 0;
pub static mut scheme: *mut c_void = core::ptr::null_mut();
pub static mut goal: *mut c_void = core::ptr::null_mut();
pub static mut filter: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    err = damon_modules_new_paddr_ctx_target(&param_ctx, &param_target);
    if (err) {
    return err;
    }
    param_ctx.addr_unit = addr_unit;
    param_ctx.min_region_sz = max(DAMON_MIN_REGION_SZ / addr_unit, 1);
    if (!damon_reclaim_mon_attrs.aggr_interval) {
    err = -EINVAL;
// goto;
    }
    attrs = damon_reclaim_mon_attrs;
    if (autotune_monitoring_intervals) {
    attrs.sample_interval = 5000;
    attrs.aggr_interval = 100000;
    attrs.intervals_goal.access_bp = 40;
    attrs.intervals_goal.aggrs = 3;
    attrs.intervals_goal.min_sample_us = 5000;
    attrs.intervals_goal.max_sample_us = 10 * 1000 * 1000;
    }
    err = damon_set_attrs(param_ctx, &attrs);
    if (err) {
// goto;
    }
    err = -ENOMEM;
    scheme = damon_reclaim_new_scheme(attrs.aggr_interval);
    if (!scheme) {
// goto;
    }
    damon_set_schemes(param_ctx, &scheme, 1);
    if (quota_mem_pressure_us) {
    goal = damos_new_quota_goal(DAMOS_QUOTA_SOME_MEM_PSI_US,
    quota_mem_pressure_us);
    if (!goal) {
// goto;
    }
    damos_add_quota_goal(&scheme.quota, goal);
    }
    if (quota_autotune_feedback) {
    goal = damos_new_quota_goal(DAMOS_QUOTA_USER_INPUT, 10000);
    if (!goal) {
// goto;
    }
    goal.current_value = quota_autotune_feedback;
    damos_add_quota_goal(&scheme.quota, goal);
    }
    if (skip_anon) {
    filter = damos_new_filter(DAMOS_FILTER_TYPE_ANON, true, false);
    if (!filter) {
// goto;
    }
    damos_add_filter(scheme, filter);
    }
    err = damon_set_region_system_rams_default(param_target,
    &monitor_region_start, &monitor_region_end,
    param_ctx.addr_unit, param_ctx.min_region_sz);
    if (err) {
// goto;
    }
    err = damon_commit_ctx(ctx, param_ctx);
// label;
    damon_destroy_ctx(param_ctx);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn damon_reclaim_commit_inputs_fn(arg: *mut c_void) -> c_int {
    return damon_reclaim_apply_parameters();
    }
    static bool damon_reclaim_damon_has_started;
#[no_mangle]
pub unsafe extern "C" fn damon_reclaim_commit_inputs_store(val: *mut c_char, kp: *mut kernel_param) -> c_int {
    let mut commit_inputs_request = 0;
    let mut err = 0;
pub static mut damon_call_control: usize = 0;
    if (!val) {
    commit_inputs_request = true;
    } else {
    err = kstrtobool(val, &commit_inputs_request);
    if (err) {
    return err;
    }
    }
    if (!commit_inputs_request) {
    return 0;
    }
// Skip damon_call() if ctx has not successfully started.
    if (!damon_reclaim_damon_has_started) {
    return -EINVAL;
    }
    err = damon_call(ctx, &control);
    return err ? err : control.return_code;
    }
pub static mut kernel_param_ops: usize = 0;
    module_param_cb!(commit_inputs, &commit_inputs_param_ops, &commit_inputs, 0600);
#[no_mangle]
unsafe extern "C" fn damon_reclaim_damon_call_fn(arg: *mut c_void) -> c_int {
    let mut c = arg;
pub static mut s: *mut c_void = core::ptr::null_mut();
// update the stats parameter
    damon_for_each_scheme(s, c)
    damon_reclaim_stat = s.stat;
    return 0;
    }
pub static mut damon_call_control: usize = 0;
#[no_mangle]
unsafe extern "C" fn damon_reclaim_turn(on: bool) -> c_int {
    let mut err = 0;
    if (!on) {
    damon_stop(&ctx, 1);
    return 0;
    }
    err = damon_reclaim_apply_parameters();
    if (err) {
    return err;
    }
    err = damon_start(&ctx, 1, true);
    if (err) {
    return err;
    }
    if (!damon_reclaim_damon_has_started) {
    damon_reclaim_damon_has_started = true;
    }
    return damon_call(ctx, &call_control);
    }
#[no_mangle]
pub unsafe extern "C" fn damon_reclaim_addr_unit_store(val: *mut c_char, kp: *mut kernel_param) -> c_int {
    let mut input_addr_unit = 0;
pub static mut err: c_int = 0;
    if (err) {
    return err;
    }
    if (!input_addr_unit) {
    return -EINVAL;
    }
    addr_unit = input_addr_unit;
    return 0;
    }
pub static mut kernel_param_ops: usize = 0;
    module_param_cb!(addr_unit, &addr_unit_param_ops, &addr_unit, 0600);
    MODULE_PARM_DESC(addr_unit,
    "Scale factor for DAMON_RECLAIM to ops address conversion (default: 1)");
#[no_mangle]
unsafe extern "C" fn damon_reclaim_enabled() -> bool {
    if (!ctx) {
    return false;
    }
    return damon_is_running(ctx);
    }
#[no_mangle]
pub unsafe extern "C" fn damon_reclaim_enabled_store(val: *mut c_char, kp: *mut kernel_param) -> c_int {
    let mut err = 0;
    err = kstrtobool(val, &enabled);
    if (err) {
    return err;
    }
    if (damon_reclaim_enabled() == enabled) {
    return 0;
    }
// Called before init function.  The function will handle this.
    if (!damon_initialized()) {
    return 0;
    }
// damon_modules_new_paddr_ctx_target() in the init function failed.
    if (!ctx) {
    return -ENOMEM;
    }
    return damon_reclaim_turn(enabled);
    }
#[no_mangle]
pub unsafe extern "C" fn damon_reclaim_enabled_load(buffer: *mut c_char, kp: *mut kernel_param) -> c_int {
    return sprintf(buffer, "%c\n", damon_reclaim_enabled() ? 'Y' : 'N');
    }
pub static mut kernel_param_ops: usize = 0;
    module_param_cb!(enabled, &enabled_param_ops, &enabled, 0600);
    MODULE_PARM_DESC(enabled,
    "Enable or disable DAMON_RECLAIM (default: disabled)");
#[no_mangle]
pub unsafe extern "C" fn damon_reclaim_kdamond_pid_store(val: *mut c_char, kp: *mut kernel_param) -> c_int {
//
// kdamond_pid is read-only, but kernel command line could write it.
// Do nothing here.
//
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn damon_reclaim_kdamond_pid_load(buffer: *mut c_char, kp: *mut kernel_param) -> c_int {
pub static mut kdamond_pid: c_int = 0;
    if (ctx) {
    kdamond_pid = damon_kdamond_pid(ctx);
    if (kdamond_pid < 0) {
    kdamond_pid = -1;
    }
    }
    return sprintf(buffer, "%d\n", kdamond_pid);
    }
pub static mut kernel_param_ops: usize = 0;
//
// PID of the DAMON thread
//
// If DAMON_RECLAIM is enabled, this becomes the PID of the worker thread.
// Else, -1.
//
    module_param_cb!(kdamond_pid, &kdamond_pid_param_ops, core::ptr::null_mut(), 0400);
#[no_mangle]
unsafe extern "C" fn damon_reclaim_init() -> c_int {
    let mut err = 0;
    if (!damon_initialized()) {
    err = -ENOMEM;
// goto;
    }
    err = damon_modules_new_paddr_ctx_target(&ctx, &target);
    if (err) {
// goto;
    }
    call_control.data = ctx;
// 'enabled' has set before this function, probably via command line
    if (enabled) {
    err = damon_reclaim_turn(true);
    }
// label;
    if (err && enabled) {
    enabled = false;
    }
    return err;
    }
    module_init!(damon_reclaim_init);