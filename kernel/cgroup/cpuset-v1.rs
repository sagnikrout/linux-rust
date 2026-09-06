//! Automatically rewritten from C to Rust
//! Source: kernel/cgroup/cpuset-v1.c
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


// SPDX-License-Identifier: GPL-2.0-or-later

//
// Legacy hierarchy call to cgroup_transfer_tasks() is handled asynchrously
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpuset_remove_tasks_struct {
    pub work: work_struct,
    pub cs: *mut cpuset,
}

//
// Frequency meter - How fast is some event occurring?
//
// These routines manage a digitally filtered, constant time based,
// event frequency meter.  There are four routines:
// fmeter_init() - initialize a frequency meter.
// fmeter_markevent() - called each time the event happens.
// fmeter_getrate() - returns the recent rate of such events.
// fmeter_update() - internal routine used to update fmeter.
//
// A common data structure is passed to each of these routines,
// which is used to keep track of the state required to manage the
// frequency meter and its digital filter.
//
// The filter works on the number of events marked per unit time.
// The filter is single-pole low-pass recursive (IIR).  The time unit
// is 1 second.  Arithmetic is done using 32-bit integers scaled to
// simulate 3 decimal digits of precision (multiplied by 1000).
//
// With an FM_COEF of 933, and a time base of 1 second, the filter
// has a half-life of 10 seconds, meaning that if the events quit
// happening, then the rate returned from the fmeter_getrate()
// will be cut in half each 10 seconds, until it converges to zero.
//
// It is not worth doing a real infinitely recursive filter.  If more
// than FM_MAXTICKS ticks have elapsed since the last filter event,
// just compute FM_MAXTICKS ticks worth, by which point the level
// will be stable.
//
// Limit the count of unprocessed events to FM_MAXCNT, so as to avoid
// arithmetic overflow in the fmeter_update() routine.
//
// Given the simple 32 bit integer arithmetic used, this meter works
// best for reporting rates between one per millisecond (msec) and
// one per 32 (approx) seconds.  At constant rates faster than one
// per msec it maxes out at values just under 1,000,000.  At constant
// rates between one per msec, and one per second it will stabilize
// to a value N*1000, where N is the rate of events per second.
// At constant rates between one per second and one per 32 seconds,
// it will be choppy, moving up on the seconds that have an event,
// and then decaying until the next event.  At rates slower than
// about one in 32 seconds, it decays all the way back to zero between
// each event.
//

// Initialize a frequency meter
#[no_mangle]
unsafe extern "C" fn fmeter_init(fmp: *mut fmeter) {
    fmp.cnt = 0;
    fmp.val = 0;
    fmp.time = 0;
    spin_lock_init(&fmp.lock);
    }
// Internal meter update - process cnt events and update value
#[no_mangle]
unsafe extern "C" fn fmeter_update(fmp: *mut fmeter) {
    let mut now;
    let mut ticks = 0;
    now = ktime_get_seconds();
    ticks = now - fmp.time;
    if (ticks == 0) {
    return;
    }
    ticks = min(FM_MAXTICKS, ticks);
    while (ticks-- > 0) {
    fmp.val = (FM_COEF * fmp.val) / FM_SCALE;
    }
    fmp.time = now;
    fmp.val += ((FM_SCALE - FM_COEF) * fmp.cnt) / FM_SCALE;
    fmp.cnt = 0;
    }
// Process any previous ticks, then bump cnt by one (times scale).
#[no_mangle]
unsafe extern "C" fn fmeter_markevent(fmp: *mut fmeter) {
    spin_lock(&fmp.lock);
    fmeter_update(fmp);
    fmp.cnt = min(FM_MAXCNT, fmp.cnt + FM_SCALE);
    spin_unlock(&fmp.lock);
    }
// Process any previous ticks, then return current value.
#[no_mangle]
unsafe extern "C" fn fmeter_getrate(fmp: *mut fmeter) -> c_int {
    let mut val = 0;
    spin_lock(&fmp.lock);
    fmeter_update(fmp);
    val = fmp.val;
    spin_unlock(&fmp.lock);
    return val;
    }
//
// Collection of memory_pressure is suppressed unless
// this flag is enabled by writing "1" to the special
// cpuset file 'memory_pressure_enabled' in the root cpuset.
//
    let mut cpuset_memory_pressure_enabled = 0;
//
// __cpuset_memory_pressure_bump - keep stats of per-cpuset reclaims.
//
// Keep a running average of the rate of synchronous (direct)
// page reclaim efforts initiated by tasks in each cpuset.
//
// This represents the rate at which some task in the cpuset
// ran low on memory on all nodes it was allowed to use, and
// had to enter the kernels page reclaim code in an effort to
// create more free memory by tossing clean pages or swapping
// or writing dirty pages.
//
// Display to user space in the per-cpuset read-only file
// "memory_pressure".  Value displayed is an integer
// representing the recent rate of entry into the synchronous
// (direct) page reclaim by any task attached to the cpuset.
//
#[no_mangle]
pub unsafe extern "C" fn __cpuset_memory_pressure_bump() {
    rcu_read_lock();
    fmeter_markevent(&task_cs(current).fmeter);
    rcu_read_unlock();
    }
#[no_mangle]
unsafe extern "C" fn update_relax_domain_level(cs: *mut cpuset, val: i64) -> c_int {

    if (val < -1 || val > sched_domain_level_max + 1) {
    return -EINVAL;
    }

    if (val != cs.relax_domain_level) {
    cs.relax_domain_level = val;
    if (!cpumask_empty(cs.cpus_allowed) &&
    is_sched_load_balance(cs)) {
    rebuild_sched_domains_locked();
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn cpuset_write_s64(css: *mut cgroup_subsys_state, cft: *mut cftype, val: s64) -> c_int {
    let mut cs = css_cs(css);
pub static mut type: cpuset_filetype_t = 0;
pub static mut retval: c_int = 0;
    cpuset_full_lock();
    if (!is_cpuset_online(cs)) {
// goto;
    }
    match (type) {
    FILE_SCHED_RELAX_DOMAIN_LEVEL => {
    pr_info_once!("cpuset.%s is deprecated\n", cft.name);
    retval = update_relax_domain_level(cs, val);
    // break;
    }
    _ => {
    retval = -EINVAL;
    // break;
    }
    }
// label;
    cpuset_full_unlock();
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn cpuset_read_s64(css: *mut cgroup_subsys_state, cft: *mut cftype) -> i64 {
    let mut cs = css_cs(css);
pub static mut type: cpuset_filetype_t = 0;
    match (type) {
    FILE_SCHED_RELAX_DOMAIN_LEVEL => {
    return cs.relax_domain_level;
    }
    _ => {
    BUG();
    }
    }
// Unreachable but makes gcc happy
    return 0;
    }
//
// Update a task's spread flag if the cpuset's page spread flag is set.
//
// Call with callback_lock or cpuset_mutex held. The check can be skipped
// if on default hierarchy.
//
#[no_mangle]
pub unsafe extern "C" fn cpuset1_update_task_spread_flags(cs: *mut cpuset, tsk: *mut task_struct) {
    if (cgroup_subsys_on_dfl(cpuset_cgrp_subsys)) {
    return;
    }
    if (is_spread_page(cs)) {
    task_set_spread_page(tsk);
    }
    else {
    task_clear_spread_page(tsk);
    }
    }
//
// cpuset1_update_tasks_flags - update the page spread flag of cpuset tasks
// @cs: the cpuset whose tasks need their page spread flag updated
//
// Iterate through each task of @cs updating its page spread flag.  As this
// function is called with cpuset_mutex held, cpuset membership stays
// stable.
//
#[no_mangle]
pub unsafe extern "C" fn cpuset1_update_tasks_flags(cs: *mut cpuset) {
pub static mut it: usize = 0;
pub static mut task: *mut c_void = core::ptr::null_mut();
    css_task_iter_start(&cs.css, 0, &it);
    while ((task = css_task_iter_next(&it))) {
    cpuset1_update_task_spread_flags(cs, task);
    }
    css_task_iter_end(&it);
    }
//
// If CPU and/or memory hotplug handlers, below, unplug any CPUs
// or memory nodes, we need to walk over the cpuset hierarchy,
// removing that CPU or node from all cpusets.  If this removes the
// last CPU or node from a cpuset, then move the tasks in the empty
// cpuset to its next-highest non-empty parent.
//
#[no_mangle]
unsafe extern "C" fn remove_tasks_in_empty_cpuset(cs: *mut cpuset) {
pub static mut parent: *mut c_void = core::ptr::null_mut();
//
// Find its next-highest non-empty parent, (top cpuset
// has online cpus, so can't be empty).
//
    parent = parent_cs(cs);
    while (cpumask_empty(parent.cpus_allowed) ||
    nodes_empty(parent.mems_allowed)) {
    parent = parent_cs(parent);
    }
    if (cgroup_transfer_tasks(parent.css.cgroup, cs.css.cgroup)) {
    pr_err!("cpuset: failed to transfer tasks out of empty cpuset ");
    pr_cont_cgroup_name(cs.css.cgroup);
    pr_cont("\n");
    }
    }
#[no_mangle]
unsafe extern "C" fn cpuset_migrate_tasks_workfn(work: *mut work_struct) {
pub static mut s: *mut c_void = core::ptr::null_mut();
    s = container_of!(work, cpuset_remove_tasks_struct, work);
    remove_tasks_in_empty_cpuset(s.cs);
    css_put(&s.cs.css);
    kfree(s);
    }
#[no_mangle]
pub unsafe extern "C" fn cpuset1_hotplug_update_tasks(cs: *mut cpuset, new_cpus: *mut cpumask, new_mems: *mut nodemask_t, cpus_updated: bool, mems_updated: bool) {
    let mut is_empty = 0;
    cpuset_callback_lock_irq();
    cpumask_copy(cs.cpus_allowed, new_cpus);
    cpumask_copy(cs.effective_cpus, new_cpus);
    cs.mems_allowed = *new_mems;
    cs.effective_mems = *new_mems;
    cpuset_callback_unlock_irq();
//
// Don't call cpuset_update_tasks_cpumask() if the cpuset becomes empty,
// as the tasks will be migrated to an ancestor.
//
    if (cpus_updated && !cpumask_empty(cs.cpus_allowed)) {
    cpuset_update_tasks_cpumask(cs, new_cpus);
    }
    if (mems_updated && !nodes_empty(cs.mems_allowed)) {
    cpuset_update_tasks_nodemask(cs);
    }
    is_empty = cpumask_empty(cs.cpus_allowed) ||
    nodes_empty(cs.mems_allowed);
//
// Move tasks to the nearest ancestor with execution resources,
// This is full cgroup operation which will also call back into
// cpuset. Execute it asynchronously using workqueue.
//
    if (is_empty && cgroup_has_tasks(cs.css.cgroup) &&
    css_tryget_online(&cs.css)) {
pub static mut s: *mut c_void = core::ptr::null_mut();
    s = kzalloc_obj(*s);
    if (WARN_ON_ONCE!(!s)) {
    css_put(&cs.css);
    return;
    }
    s.cs = cs;
    INIT_WORK(&s.work, cpuset_migrate_tasks_workfn);
    schedule_work(&s.work);
    }
    }
//
// is_cpuset_subset(p, q) - Is cpuset p a subset of cpuset q?
//
// One cpuset is a subset of another if all its allowed CPUs and
// Memory Nodes are a subset of the other, and its exclusive flags
// are only set if the other's are set.  Call holding cpuset_mutex.
//
#[no_mangle]
unsafe extern "C" fn is_cpuset_subset(p: *const cpuset, q: *const cpuset) -> c_int {
    return	cpumask_subset(p.cpus_allowed, q.cpus_allowed) &&
    nodes_subset(p.mems_allowed, q.mems_allowed) &&
    is_cpu_exclusive(p) <= is_cpu_exclusive(q) &&
    is_mem_exclusive(p) <= is_mem_exclusive(q);
    }
//
// cpuset1_validate_change() - Validate conditions specific to legacy (v1)
// behavior.
//
#[no_mangle]
pub unsafe extern "C" fn cpuset1_validate_change(cur: *mut cpuset, trial: *mut cpuset) -> c_int {
pub static mut css: *mut c_void = core::ptr::null_mut();
    let mut c = core::ptr::null_mut();
    let mut par = core::ptr::null_mut();
    let mut ret = 0;
    WARN_ON_ONCE!(!rcu_read_lock_held());
// Each of our child cpusets must be a subset of us
    ret = -EBUSY;
    cpuset_for_each_child(c, css, cur)
    if (!is_cpuset_subset(c, trial)) {
// goto;
    }
// On legacy hierarchy, we must be a subset of our parent cpuset.
    ret = -EACCES;
    par = parent_cs(cur);
    if (par && !is_cpuset_subset(trial, par)) {
// goto;
    }
//
// Cpusets with tasks - existing or newly being attached - can't
// be changed to have empty cpus_allowed or mems_allowed.
//
    ret = -ENOSPC;
    if (cpuset_is_populated(cur)) {
    if (!cpumask_empty(cur.cpus_allowed) &&
    cpumask_empty(trial.cpus_allowed)) {
// goto;
    }
    if (!nodes_empty(cur.mems_allowed) &&
    nodes_empty(trial.mems_allowed)) {
// goto;
    }
    }
    ret = 0;
// label;
    return ret;
    }
//
// cpuset1_cpus_excl_conflict() - Check if two cpusets have exclusive CPU conflicts
// to legacy (v1)
// @cs1: first cpuset to check
// @cs2: second cpuset to check
//
// Returns: true if CPU exclusivity conflict exists, false otherwise
//
// If either cpuset is CPU exclusive, their allowed CPUs cannot intersect.
//
#[no_mangle]
pub unsafe extern "C" fn cpuset1_cpus_excl_conflict(cs1: *mut cpuset, cs2: *mut cpuset) -> bool {
    if (is_cpu_exclusive(cs1) || is_cpu_exclusive(cs2)) {
    return cpumask_intersects(cs1.cpus_allowed,
    cs2.cpus_allowed);
    }
    return false;
    }

//
// proc_cpuset_show()
// - Print tasks cpuset path into seq_file.
// - Used for /proc/<pid>/cpuset.
//
#[no_mangle]
pub unsafe extern "C" fn proc_cpuset_show(m: *mut seq_file, ns: *mut pid_namespace, pid: *mut pid, tsk: *mut task_struct) -> c_int {
pub static mut buf: *mut c_void = core::ptr::null_mut();
pub static mut css: *mut c_void = core::ptr::null_mut();
    let mut retval = 0;
    retval = -ENOMEM;
    buf = kmalloc(PATH_MAX, GFP_KERNEL);
    if (!buf) {
// goto;
    }
    rcu_read_lock();
    spin_lock_irq(&css_set_lock);
    css = task_css(tsk, cpuset_cgrp_id);
    retval = cgroup_path_ns_locked(css.cgroup, buf, PATH_MAX,
    current.nsproxy.cgroup_ns);
    spin_unlock_irq(&css_set_lock);
    rcu_read_unlock();
    if (retval == -E2BIG) {
    retval = -ENAMETOOLONG;
    }
    if (retval < 0) {
// goto;
    }
    seq_puts(m, buf);
    seq_putc(m, '\n');
    retval = 0;
// label;
    kfree(buf);
// label;
    return retval;
    }

#[no_mangle]
unsafe extern "C" fn cpuset_read_u64(css: *mut cgroup_subsys_state, cft: *mut cftype) -> u64 {
    let mut cs = css_cs(css);
pub static mut type: cpuset_filetype_t = 0;
    match (type) {
    FILE_CPU_EXCLUSIVE => {
    return is_cpu_exclusive(cs);
    }
    FILE_MEM_EXCLUSIVE => {
    return is_mem_exclusive(cs);
    }
    FILE_MEM_HARDWALL => {
    return is_mem_hardwall(cs);
    }
    FILE_SCHED_LOAD_BALANCE => {
    return is_sched_load_balance(cs);
    }
    FILE_MEMORY_MIGRATE => {
    return is_memory_migrate(cs);
    }
    FILE_MEMORY_PRESSURE_ENABLED => {
    return cpuset_memory_pressure_enabled;
    }
    FILE_MEMORY_PRESSURE => {
    return fmeter_getrate(&cs.fmeter);
    }
    FILE_SPREAD_PAGE => {
    return is_spread_page(cs);
    }
    FILE_SPREAD_SLAB => {
    return is_spread_slab(cs);
    }
    _ => {
    BUG();
    }
    }
// Unreachable but makes gcc happy
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn cpuset_write_u64(css: *mut cgroup_subsys_state, cft: *mut cftype, val: u64) -> c_int {
    let mut cs = css_cs(css);
pub static mut type: cpuset_filetype_t = 0;
pub static mut retval: c_int = 0;
    cpuset_full_lock();
    if (!is_cpuset_online(cs)) {
    retval = -ENODEV;
// goto;
    }
    match (type) {
    FILE_CPU_EXCLUSIVE => {
    retval = cpuset_update_flag(CS_CPU_EXCLUSIVE, cs, val);
    // break;
    }
    FILE_MEM_EXCLUSIVE => {
    pr_info_once!("cpuset.%s is deprecated\n", cft.name);
    retval = cpuset_update_flag(CS_MEM_EXCLUSIVE, cs, val);
    // break;
    }
    FILE_MEM_HARDWALL => {
    pr_info_once!("cpuset.%s is deprecated\n", cft.name);
    retval = cpuset_update_flag(CS_MEM_HARDWALL, cs, val);
    // break;
    }
    FILE_SCHED_LOAD_BALANCE => {
    pr_info_once!("cpuset.%s is deprecated, use cpuset.cpus.partition instead\n", cft.name);
    retval = cpuset_update_flag(CS_SCHED_LOAD_BALANCE, cs, val);
    // break;
    }
    FILE_MEMORY_MIGRATE => {
    pr_info_once!("cpuset.%s is deprecated\n", cft.name);
    retval = cpuset_update_flag(CS_MEMORY_MIGRATE, cs, val);
    // break;
    }
    FILE_MEMORY_PRESSURE_ENABLED => {
    pr_info_once!("cpuset.%s is deprecated, use memory.pressure with CONFIG_PSI instead\n", cft.name);
    cpuset_memory_pressure_enabled = !!val;
    // break;
    }
    FILE_SPREAD_PAGE => {
    pr_info_once!("cpuset.%s is deprecated\n", cft.name);
    retval = cpuset_update_flag(CS_SPREAD_PAGE, cs, val);
    // break;
    }
    FILE_SPREAD_SLAB => {
    pr_warn_once("cpuset.%s is deprecated\n", cft.name);
    retval = cpuset_update_flag(CS_SPREAD_SLAB, cs, val);
    // break;
    }
    _ => {
    retval = -EINVAL;
    // break;
    }
    }
// label;
    cpuset_full_unlock();
    return retval;
    }
#[no_mangle]
pub unsafe extern "C" fn cpuset1_init(cs: *mut cpuset) {
    fmeter_init(&cs.fmeter);
    cs.relax_domain_level = -1;
    }
#[no_mangle]
pub unsafe extern "C" fn cpuset1_online_css(css: *mut cgroup_subsys_state) {
pub static mut tmp_cs: *mut c_void = core::ptr::null_mut();
pub static mut pos_css: *mut c_void = core::ptr::null_mut();
    let mut cs = css_cs(css);
    let mut parent = parent_cs(cs);
    lockdep_assert_cpus_held();
    lockdep_assert_cpuset_lock_held();
    if (is_spread_page(parent)) {
    set_bit(CS_SPREAD_PAGE, &cs.flags);
    }
    if (is_spread_slab(parent)) {
    set_bit(CS_SPREAD_SLAB, &cs.flags);
    }
    if (!test_bit(CGRP_CPUSET_CLONE_CHILDREN, &css.cgroup.flags)) {
    return;
    }
//
// Clone @parent's configuration if CGRP_CPUSET_CLONE_CHILDREN is
// set.  This flag handling is implemented in cgroup core for
// historical reasons - the flag may be specified during mount.
//
// Currently, if any sibling cpusets have exclusive cpus or mem, we
// refuse to clone the configuration - thereby refusing the task to
// be entered, and as a result refusing the sys_unshare() or
// clone() which initiated it.  If this becomes a problem for some
// users who wish to allow that scenario, then this could be
// changed to grant parent->cpus_allowed-sibling_cpus_exclusive
// (and likewise for mems) to the new cgroup.
//
    rcu_read_lock();
    cpuset_for_each_child(tmp_cs, pos_css, parent) {
    if (is_mem_exclusive(tmp_cs) || is_cpu_exclusive(tmp_cs)) {
    rcu_read_unlock();
    return;
    }
    }
    rcu_read_unlock();
    cpuset_callback_lock_irq();
    cs.mems_allowed = parent.mems_allowed;
    cs.effective_mems = parent.mems_allowed;
    cpumask_copy(cs.cpus_allowed, parent.cpus_allowed);
    cpumask_copy(cs.effective_cpus, parent.cpus_allowed);
    cpuset_callback_unlock_irq();
    }
#[no_mangle]
pub unsafe extern "C" fn update_domain_attr(dattr: *mut sched_domain_attr, c: *mut cpuset) {
    if (dattr.relax_domain_level < c.relax_domain_level) {
    dattr.relax_domain_level = c.relax_domain_level;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn update_domain_attr_tree(dattr: *mut sched_domain_attr, root_cs: *mut cpuset) {
pub static mut cp: *mut c_void = core::ptr::null_mut();
pub static mut pos_css: *mut c_void = core::ptr::null_mut();
    rcu_read_lock();
    cpuset_for_each_descendant_pre(cp, pos_css, root_cs) {
// skip the whole subtree if @cp doesn't have any CPU
    if (cpumask_empty(cp.cpus_allowed)) {
    pos_css = css_rightmost_descendant(pos_css);
    continue;
    }
    if (is_sched_load_balance(cp)) {
    update_domain_attr(dattr, cp);
    }
    }
    rcu_read_unlock();
    }
//
// cpuset1_generate_sched_domains()
//
// Finding the best partition (set of domains):
// The double nested loops below over i, j scan over the load
// balanced cpusets (using the array of cpuset pointers in csa[])
// looking for pairs of cpusets that have overlapping cpus_allowed
// and merging them using a union-find algorithm.
//
// The union of the cpus_allowed masks from the set of all cpusets
// having the same root then form the one element of the partition
// (one sched domain) to be passed to partition_sched_domains().
//
#[no_mangle]
pub unsafe extern "C" fn cpuset1_generate_sched_domains(domains: *mut *mut cpumask_var_t, attributes: *mut *mut sched_domain_attr) -> c_int {
pub static mut cp: *mut c_void = core::ptr::null_mut();	/* top-down scan of cpusets */
pub static mut csa: *mut c_void = core::ptr::null_mut();	/* array of all cpuset ptrs */
    let mut csn = 0;		/* how many cpuset ptrs in csa so far */
    let mut i = 0;
    let mut j = 0;		/* indices for partition finding loops */
pub static mut doms: *mut c_void = core::ptr::null_mut();	/* resulting partition; i.e. sched domains */
pub static mut dattr: *mut c_void = core::ptr::null_mut();  /* attributes for custom domains */
    let mut ndoms = 0;		/* number of sched domains in result */
    let mut nslot = 0;		/* next empty doms[] struct cpumask slot */
pub static mut pos_css: *mut c_void = core::ptr::null_mut();
    let mut nslot_update = 0;
    lockdep_assert_cpuset_lock_held();
    doms = core::ptr::null_mut();
    dattr = core::ptr::null_mut();
    csa = core::ptr::null_mut();
// Special case for the 99% of systems with one, full, sched domain
    if (is_sched_load_balance(&top_cpuset)) {
    ndoms = 1;
    doms = alloc_sched_domains(ndoms);
    if (!doms) {
// goto;
    }
    dattr = kmalloc_obj(sched_domain_attr);
    if (dattr) {
// dattr = SD_ATTR_INIT;
    update_domain_attr_tree(dattr, &top_cpuset);
    }
    cpumask_and(doms[0], top_cpuset.effective_cpus,
    housekeeping_cpumask(HK_TYPE_DOMAIN));
// goto;
    }
    csa = kmalloc_objs(cp, nr_cpusets());
    if (!csa) {
// goto;
    }
    csn = 0;
    rcu_read_lock();
    cpuset_for_each_descendant_pre(cp, pos_css, &top_cpuset) {
    if (cp == &top_cpuset) {
    continue;
    }
//
// Continue traversing beyond @cp iff @cp has some CPUs and
// isn't load balancing.  The former is obvious.  The
// latter: All child cpusets contain a subset of the
// parent's cpus, so just skip them, and then we call
// update_domain_attr_tree() to calc relax_domain_level of
// the corresponding sched domain.
//
    if (!cpumask_empty(cp.cpus_allowed) &&
    !(is_sched_load_balance(cp) &&
    cpumask_intersects(cp.cpus_allowed,
    housekeeping_cpumask(HK_TYPE_DOMAIN)))) {
    continue;
    }
    if (is_sched_load_balance(cp) &&
    !cpumask_empty(cp.effective_cpus)) {
    csa[csn++] = cp;
    }
// skip @cp's subtree
    pos_css = css_rightmost_descendant(pos_css);
    continue;
    }
    rcu_read_unlock();
    for (i = 0; i < csn; i++) {
    uf_node_init(&csa[i].node);
    }
// Merge overlapping cpusets
    while (i < csn) {
    while (j < csn) {
    if (cpusets_overlap(csa[i], csa[j])) {
    uf_union(&csa[i].node, &csa[j].node);
    }
    }
    }
// Count the total number of domains
    while (i < csn) {
    if (uf_find(&csa[i].node) == &csa[i].node) {
    ndoms += 1;
    }
    }
//
// Now we know how many domains to create.
// Convert <csn, csa> to <ndoms, doms> and populate cpu masks.
//
    doms = alloc_sched_domains(ndoms);
    if (!doms) {
// goto;
    }
//
// The rest of the code, including the scheduler, can deal with
// dattr==NULL case. No need to abort if alloc fails.
//
    dattr = kmalloc_objs(sched_domain_attr, ndoms);
    while (i < csn) {
    nslot_update = 0;
    while (j < csn) {
    if (uf_find(&csa[j].node) == &csa[i].node) {
    let mut dp = doms[nslot];
    if (i == j) {
    nslot_update = 1;
    cpumask_clear(dp);
    if (dattr) {
// (dattr + nslot) = SD_ATTR_INIT;
    }
    }
    cpumask_or(dp, dp, csa[j].effective_cpus);
    cpumask_and(dp, dp, housekeeping_cpumask(HK_TYPE_DOMAIN));
    if (dattr) {
    update_domain_attr_tree(dattr + nslot, csa[j]);
    }
    }
    }
    if (nslot_update) {
    nslot += 1;
    }
    }
    BUG_ON!(nslot != ndoms);
// label;
    kfree(csa);
//
// Fallback to the default domain if kmalloc() failed.
// See comments in partition_sched_domains().
//
    if (doms == core::ptr::null_mut()) {
    ndoms = 1;
    }
// domains    = doms;
// attributes = dattr;
    return ndoms;
    }
//
// for the common functions, 'private' gives the type of file
//
pub static mut cftype: usize = 0;