//! Automatically rewritten from C to Rust
//! Source: mm/vmstat.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// linux/mm/vmstat.c
//
// Manages VM statistics
// Copyright (C) 1991, 1992, 1993, 1994  Linus Torvalds
//
// zoned VM statistics
// Copyright (C) 2006 Silicon Graphics, Inc.,
// Christoph Lameter <cl@gentwo.org>
// Copyright (C) 2008-2014 Christoph Lameter
//

pub const ENABLE_NUMA_STAT: c_int = 1;
pub static mut sysctl_vm_numa_stat: int = 0;
// zero numa counters within a zone
#[no_mangle]
unsafe extern "C" fn zero_zone_numa_counters(zone: *mut zone) {
    let mut item = 0;
    let mut cpu = 0;
    while (item < NR_VM_NUMA_EVENT_ITEMS) {
    atomic_long_set(&zone.vm_numa_event[item], 0);
    for_each_online_cpu(cpu) {
    per_cpu_ptr(zone.per_cpu_zonestats, cpu).vm_numa_event[item]
    = 0;
    }
    }
    }
// zero numa counters of all the populated zones
#[no_mangle]
unsafe extern "C" fn zero_zones_numa_counters() {
pub static mut zone: *mut c_void = core::ptr::null_mut();
    for_each_populated_zone(zone) {
    zero_zone_numa_counters(zone);
    }
    }
// zero global numa counters
#[no_mangle]
unsafe extern "C" fn zero_global_numa_counters() {
    let mut item = 0;
    for (item = 0; item < NR_VM_NUMA_EVENT_ITEMS; item++) {
    atomic_long_set(&vm_numa_event[item], 0);
    }
    }
#[no_mangle]
unsafe extern "C" fn invalid_numa_statistics() {
    zero_zones_numa_counters();
    zero_global_numa_counters();
    }
pub static mut vm_numa_stat_lock: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn sysctl_vm_numa_stat_handler(table: *mut ctl_table, write: c_int, buffer: *mut c_void, length: *mut size_t, ppos: *mut loff_t) -> c_int {
    let mut ret = 0;
    let mut oldval = 0;
    mutex_lock(&vm_numa_stat_lock);
    if (write) {
    oldval = sysctl_vm_numa_stat;
    }
    ret = proc_dointvec_minmax(table, write, buffer, length, ppos);
    if (ret || !write) {
// goto;
    }
    if (oldval == sysctl_vm_numa_stat) {
// goto;
    }
if true {
    static_branch_enable(&vm_numa_stat_key);
    pr_info!("enable numa statistics\n");
    } else {
    static_branch_disable(&vm_numa_stat_key);
    invalid_numa_statistics();
    pr_info!("disable numa statistics, and clear numa counters\n");
    }
// label;
    mutex_unlock(&vm_numa_stat_lock);
    return ret;
    }

    DEFINE_PER_CPU(vm_event_state, vm_event_states) = {{0}};
    EXPORT_PER_CPU_SYMBOL(vm_event_states);
#[no_mangle]
unsafe extern "C" fn sum_vm_events(ret: *mut c_ulong) {
    let mut cpu = 0;
    let mut i = 0;
    memset(ret, 0, NR_VM_EVENT_ITEMS * sizeof!(unsigned long));
    for_each_online_cpu(cpu) {
    let mut this = &per_cpu(vm_event_states, cpu);
    for (i = 0; i < NR_VM_EVENT_ITEMS; i++) {
    ret[i] += this.event[i];
    }
    }
    }
//
// Accumulate the vm event counters across all CPUs.
// The result is unavoidably approximate - it can change
// during and after execution of this function.
//
#[no_mangle]
pub unsafe extern "C" fn all_vm_events(ret: *mut c_ulong) {
    cpus_read_lock();
    sum_vm_events(ret);
    cpus_read_unlock();
    }
    EXPORT_SYMBOL_GPL(all_vm_events);
//
// Fold the foreign cpu events into our own.
//
// This is adding to the events on one processor
// but keeps the global counts constant.
//
#[no_mangle]
pub unsafe extern "C" fn vm_events_fold_cpu(cpu: c_int) {
    let mut fold_state = &per_cpu(vm_event_states, cpu);
    let mut i = 0;
    while (i < NR_VM_EVENT_ITEMS) {
    count_vm_events(i, fold_state.event[i]);
    fold_state.event[i] = 0;
    }
    }

//
// Manage combined zone based / global counters
//
// vm_stat contains the global counters
//
    atomic_long_t vm_zone_stat[NR_VM_ZONE_STAT_ITEMS] __cacheline_aligned_in_smp;
    atomic_long_t vm_node_stat[NR_VM_NODE_STAT_ITEMS] __cacheline_aligned_in_smp;
    atomic_long_t vm_numa_event[NR_VM_NUMA_EVENT_ITEMS] __cacheline_aligned_in_smp;
    EXPORT_SYMBOL(vm_zone_stat);
    EXPORT_SYMBOL(vm_node_stat);

#[no_mangle]
unsafe extern "C" fn fold_vm_zone_numa_events(zone: *mut zone) {
    unsigned long zone_numa_events[NR_VM_NUMA_EVENT_ITEMS] = { 0, };
    let mut cpu = 0;
    enum numa_stat_item item;
    for_each_online_cpu(cpu) {
pub static mut pzstats: *mut c_void = core::ptr::null_mut();
    pzstats = per_cpu_ptr(zone.per_cpu_zonestats, cpu);
    for (item = 0; item < NR_VM_NUMA_EVENT_ITEMS; item++) {
    zone_numa_events[item] += xchg(&pzstats.vm_numa_event[item], 0);
    }
    }
    for (item = 0; item < NR_VM_NUMA_EVENT_ITEMS; item++) {
    zone_numa_event_add(zone_numa_events[item], zone, item);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn fold_vm_numa_events() {
pub static mut zone: *mut c_void = core::ptr::null_mut();
    for_each_populated_zone(zone) {
    fold_vm_zone_numa_events(zone);
    }
    }

#[no_mangle]
pub unsafe extern "C" fn calculate_pressure_threshold(zone: *mut zone) -> c_int {
    let mut threshold = 0;
    let mut watermark_distance = 0;
//
// As vmstats are not up to date, there is drift between the estimated
// and real values. For high thresholds and a high number of CPUs, it
// is possible for the min watermark to be breached while the estimated
// value looks fine. The pressure threshold is a reduced value such
// that even the maximum amount of drift will not accidentally breach
// the min watermark
//
    watermark_distance = low_wmark_pages(zone) - min_wmark_pages(zone);
    threshold = max(1, (int)(watermark_distance / num_online_cpus()));
//
// Maximum threshold is 125
//
    threshold = min(125, threshold);
    return threshold;
    }
#[no_mangle]
pub unsafe extern "C" fn calculate_normal_threshold(zone: *mut zone) -> c_int {
    let mut threshold = 0;
    let mut mem = 0;	/* memory in 128 MB units */
//
// The threshold scales with the number of processors and the amount
// of memory per zone. More memory means that we can defer updates for
// longer, more processors could lead to more contention.
// fls() is used to have a cheap way of logarithmic scaling.
//
// Some sample thresholds:
//
// Threshold	Processors	(fls)	Zonesize	fls(mem)+1
// ------------------------------------------------------------------
// 8		1		1	0.9-1 GB	4
// 16		2		2	0.9-1 GB	4
// 20 		2		2	1-2 GB		5
// 24		2		2	2-4 GB		6
// 28		2		2	4-8 GB		7
// 32		2		2	8-16 GB		8
// 4		2		2	<128M		1
// 30		4		3	2-4 GB		5
// 48		4		3	8-16 GB		8
// 32		8		4	1-2 GB		4
// 32		8		4	0.9-1GB		4
// 10		16		5	<128M		1
// 40		16		5	900M		4
// 70		64		7	2-4 GB		5
// 84		64		7	4-8 GB		6
// 108		512		9	4-8 GB		6
// 125		1024		10	8-16 GB		8
// 125		1024		10	16-32 GB	9
//
    mem = zone_managed_pages(zone) >> (27 - PAGE_SHIFT);
    threshold = 2 * fls(num_online_cpus()) * (1 + fls(mem));
//
// Maximum threshold is 125
//
    threshold = min(125, threshold);
    return threshold;
    }
//
// Refresh the thresholds for each zone.
//
#[no_mangle]
pub unsafe extern "C" fn refresh_zone_stat_thresholds() {
pub static mut pgdat: *mut c_void = core::ptr::null_mut();
pub static mut zone: *mut c_void = core::ptr::null_mut();
    let mut cpu = 0;
    let mut threshold = 0;
// Zero current pgdat thresholds
    for_each_online_pgdat(pgdat) {
    for_each_online_cpu(cpu) {
    per_cpu_ptr(pgdat.per_cpu_nodestats, cpu).stat_threshold = 0;
    }
    }
    for_each_populated_zone(zone) {
    let mut pgdat = zone.zone_pgdat;
    unsigned long max_drift, tolerate_drift;
    threshold = calculate_normal_threshold(zone);
    for_each_online_cpu(cpu) {
    let mut pgdat_threshold = 0;
    per_cpu_ptr(zone.per_cpu_zonestats, cpu).stat_threshold
    = threshold;
// Base nodestat threshold on the largest populated zone.
    pgdat_threshold = per_cpu_ptr(pgdat.per_cpu_nodestats, cpu).stat_threshold;
    per_cpu_ptr(pgdat.per_cpu_nodestats, cpu).stat_threshold
    = max(threshold, pgdat_threshold);
    }
//
// Only set percpu_drift_mark if there is a danger that
// NR_FREE_PAGES reports the low watermark is ok when in fact
// the min watermark could be breached by an allocation
//
    tolerate_drift = low_wmark_pages(zone) - min_wmark_pages(zone);
    max_drift = num_online_cpus() * threshold;
    if (max_drift > tolerate_drift) {
    zone.percpu_drift_mark = high_wmark_pages(zone) +
    max_drift;
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn set_pgdat_percpu_threshold(pgdat: *mut pg_data_t) {
pub static mut zone: *mut c_void = core::ptr::null_mut();
    let mut cpu = 0;
    let mut threshold = 0;
    let mut i = 0;
    while (i < pgdat.nr_zones) {
    zone = &pgdat.node_zones[i];
    if (!zone.percpu_drift_mark) {
    continue;
    }
    threshold = (*calculate_pressure)(zone);
    for_each_online_cpu(cpu) {
    per_cpu_ptr(zone.per_cpu_zonestats, cpu).stat_threshold
    = threshold;
    }
    }
    }
//
// For use when we know that interrupts are disabled,
// or when we know that preemption is disabled and that
// particular counter cannot be updated from interrupt context.
//
#[no_mangle]
pub unsafe extern "C" fn __mod_zone_page_state(zone: *mut zone, item: zone_stat_item, delta: c_long) {
    let mut pcp = zone.per_cpu_zonestats;
    let mut p = pcp.vm_stat_diff + item;
    let mut x = 0;
    let mut t = 0;
//
// Accurate vmstat updates require a RMW. On !PREEMPT_RT kernels,
// atomicity is provided by IRQs being disabled -- either explicitly
// or via local_lock_irq. On PREEMPT_RT, local_lock_irq only disables
// CPU migrations and preemption potentially corrupts a counter so
// disable preemption.
//
    preempt_disable_nested();
    x = delta + __this_cpu_read(*p);
    t = __this_cpu_read(pcp.stat_threshold);
    if (unlikely(abs(x) > t)) {
    zone_page_state_add(x, zone, item);
    x = 0;
    }
    __this_cpu_write(*p, x);
    preempt_enable_nested();
    }
    EXPORT_SYMBOL(__mod_zone_page_state);
#[no_mangle]
pub unsafe extern "C" fn __mod_node_page_state(pgdat: *mut pglist_data, item: node_stat_item, delta: c_long) {
    let mut pcp = pgdat.per_cpu_nodestats;
    let mut p = pcp.vm_node_stat_diff + item;
    let mut x = 0;
    let mut t = 0;
    if (vmstat_item_in_bytes(item)) {
//
// Only cgroups use subpage accounting right now; at
// the global level, these items still change in
// multiples of whole pages. Store them as pages
// internally to keep the per-cpu counters compact.
//
    VM_WARN_ON_ONCE(delta & (PAGE_SIZE - 1));
    delta >>= PAGE_SHIFT;
    }
// See __mod_zone_page_state()
    preempt_disable_nested();
    x = delta + __this_cpu_read(*p);
    t = __this_cpu_read(pcp.stat_threshold);
    if (unlikely(abs(x) > t)) {
    node_page_state_add(x, pgdat, item);
    x = 0;
    }
    __this_cpu_write(*p, x);
    preempt_enable_nested();
    }
    EXPORT_SYMBOL(__mod_node_page_state);
//
// Optimized increment and decrement functions.
//
// These are only for a single page and therefore can take a struct page
// argument instead of struct zone *. This allows the inclusion of the code
// generated for page_zone(page) into the optimized functions.
//
// No overflow check is necessary and therefore the differential can be
// incremented or decremented in place which may allow the compilers to
// generate better code.
// The increment or decrement is known and therefore one boundary check can
// be omitted.
//
// NOTE: These functions are very performance sensitive. Change only
// with care.
//
// Some processors have inc/dec instructions that are atomic vs an interrupt.
// However, the code must first determine the differential location in a zone
// based on the processor number and then inc/dec the counter. There is no
// guarantee without disabling preemption that the processor will not change
// in between and therefore the atomicity vs. interrupt cannot be exploited
// in a useful way here.
//
#[no_mangle]
pub unsafe extern "C" fn __inc_zone_state(zone: *mut zone, item: zone_stat_item) {
    let mut pcp = zone.per_cpu_zonestats;
    let mut p = pcp.vm_stat_diff + item;
    s8 v, t;
// See __mod_zone_page_state()
    preempt_disable_nested();
    v = __this_cpu_inc_return(*p);
    t = __this_cpu_read(pcp.stat_threshold);
    if (unlikely(v > t)) {
pub static mut overstep: i8 = 0;
    zone_page_state_add(v + overstep, zone, item);
    __this_cpu_write(*p, -overstep);
    }
    preempt_enable_nested();
    }
#[no_mangle]
pub unsafe extern "C" fn __inc_node_state(pgdat: *mut pglist_data, item: node_stat_item) {
    let mut pcp = pgdat.per_cpu_nodestats;
    let mut p = pcp.vm_node_stat_diff + item;
    s8 v, t;
    VM_WARN_ON_ONCE(vmstat_item_in_bytes(item));
// See __mod_zone_page_state()
    preempt_disable_nested();
    v = __this_cpu_inc_return(*p);
    t = __this_cpu_read(pcp.stat_threshold);
    if (unlikely(v > t)) {
pub static mut overstep: i8 = 0;
    node_page_state_add(v + overstep, pgdat, item);
    __this_cpu_write(*p, -overstep);
    }
    preempt_enable_nested();
    }
#[no_mangle]
pub unsafe extern "C" fn __inc_zone_page_state(page: *mut page, item: zone_stat_item) {
    __inc_zone_state(page_zone(page), item);
    }
    EXPORT_SYMBOL(__inc_zone_page_state);
#[no_mangle]
pub unsafe extern "C" fn __inc_node_page_state(page: *mut page, item: node_stat_item) {
    __inc_node_state(page_pgdat(page), item);
    }
    EXPORT_SYMBOL(__inc_node_page_state);
#[no_mangle]
pub unsafe extern "C" fn __dec_zone_state(zone: *mut zone, item: zone_stat_item) {
    let mut pcp = zone.per_cpu_zonestats;
    let mut p = pcp.vm_stat_diff + item;
    s8 v, t;
// See __mod_zone_page_state()
    preempt_disable_nested();
    v = __this_cpu_dec_return(*p);
    t = __this_cpu_read(pcp.stat_threshold);
    if (unlikely(v < - t)) {
pub static mut overstep: i8 = 0;
    zone_page_state_add(v - overstep, zone, item);
    __this_cpu_write(*p, overstep);
    }
    preempt_enable_nested();
    }
#[no_mangle]
pub unsafe extern "C" fn __dec_node_state(pgdat: *mut pglist_data, item: node_stat_item) {
    let mut pcp = pgdat.per_cpu_nodestats;
    let mut p = pcp.vm_node_stat_diff + item;
    s8 v, t;
    VM_WARN_ON_ONCE(vmstat_item_in_bytes(item));
// See __mod_zone_page_state()
    preempt_disable_nested();
    v = __this_cpu_dec_return(*p);
    t = __this_cpu_read(pcp.stat_threshold);
    if (unlikely(v < - t)) {
pub static mut overstep: i8 = 0;
    node_page_state_add(v - overstep, pgdat, item);
    __this_cpu_write(*p, overstep);
    }
    preempt_enable_nested();
    }
#[no_mangle]
pub unsafe extern "C" fn __dec_zone_page_state(page: *mut page, item: zone_stat_item) {
    __dec_zone_state(page_zone(page), item);
    }
    EXPORT_SYMBOL(__dec_zone_page_state);
#[no_mangle]
pub unsafe extern "C" fn __dec_node_page_state(page: *mut page, item: node_stat_item) {
    __dec_node_state(page_pgdat(page), item);
    }
    EXPORT_SYMBOL(__dec_node_page_state);

//
// If we have cmpxchg_local support then we do not need to incur the overhead
// that comes with local_irq_save/restore if we use this_cpu_try_cmpxchg().
//
// mod_state() modifies the zone counter state through atomic per cpu
// operations.
//
// Overstep mode specifies how overstep should handled:
// 0       No overstepping
// 1       Overstepping half of threshold
// -1      Overstepping minus half of threshold
//
#[no_mangle]
pub unsafe extern "C" fn mod_zone_state(zone: *mut zone, item: zone_stat_item, delta: c_long, overstep_mode: c_int) {
    let mut pcp = zone.per_cpu_zonestats;
    let mut p = pcp.vm_stat_diff + item;
    let mut n = 0;
    let mut t = 0;
    let mut z = 0;
    let mut o = 0;
    o = this_cpu_read(*p);
    do {
    z = 0;  /* overflow to zone counters */
//
// The fetching of the stat_threshold is racy. We may apply
// a counter threshold to the wrong the cpu if we get
// rescheduled while executing here. However, the next
// counter update will apply the threshold again and
// therefore bring the counter under the threshold again.
//
// Most of the time the thresholds are the same anyways
// for all cpus in a zone.
//
    t = this_cpu_read(pcp.stat_threshold);
    n = delta + (long)o;
    if (abs(n) > t) {
pub static mut os: c_int = 0;
// Overflow must be added to zone counters
    z = n + os;
    n = -os;
    }
    } while (!this_cpu_try_cmpxchg(*p, &o, n));
    if (z) {
    zone_page_state_add(z, zone, item);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn mod_zone_page_state(zone: *mut zone, item: zone_stat_item, delta: c_long) {
    mod_zone_state(zone, item, delta, 0);
    }
    EXPORT_SYMBOL(mod_zone_page_state);
#[no_mangle]
pub unsafe extern "C" fn inc_zone_page_state(page: *mut page, item: zone_stat_item) {
    mod_zone_state(page_zone(page), item, 1, 1);
    }
    EXPORT_SYMBOL(inc_zone_page_state);
#[no_mangle]
pub unsafe extern "C" fn dec_zone_page_state(page: *mut page, item: zone_stat_item) {
    mod_zone_state(page_zone(page), item, -1, -1);
    }
    EXPORT_SYMBOL(dec_zone_page_state);
#[no_mangle]
pub unsafe extern "C" fn mod_node_state(pgdat: *mut pglist_data, item: node_stat_item, delta: c_int, overstep_mode: c_int) {
    let mut pcp = pgdat.per_cpu_nodestats;
    let mut p = pcp.vm_node_stat_diff + item;
    let mut n = 0;
    let mut t = 0;
    let mut z = 0;
    let mut o = 0;
    if (vmstat_item_in_bytes(item)) {
//
// Only cgroups use subpage accounting right now; at
// the global level, these items still change in
// multiples of whole pages. Store them as pages
// internally to keep the per-cpu counters compact.
//
    VM_WARN_ON_ONCE(delta & (PAGE_SIZE - 1));
    delta >>= PAGE_SHIFT;
    }
    o = this_cpu_read(*p);
    do {
    z = 0;  /* overflow to node counters */
//
// The fetching of the stat_threshold is racy. We may apply
// a counter threshold to the wrong the cpu if we get
// rescheduled while executing here. However, the next
// counter update will apply the threshold again and
// therefore bring the counter under the threshold again.
//
// Most of the time the thresholds are the same anyways
// for all cpus in a node.
//
    t = this_cpu_read(pcp.stat_threshold);
    n = delta + (long)o;
    if (abs(n) > t) {
pub static mut os: c_int = 0;
// Overflow must be added to node counters
    z = n + os;
    n = -os;
    }
    } while (!this_cpu_try_cmpxchg(*p, &o, n));
    if (z) {
    node_page_state_add(z, pgdat, item);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn mod_node_page_state(pgdat: *mut pglist_data, item: node_stat_item, delta: c_long) {
    mod_node_state(pgdat, item, delta, 0);
    }
    EXPORT_SYMBOL(mod_node_page_state);
#[no_mangle]
pub unsafe extern "C" fn inc_node_page_state(page: *mut page, item: node_stat_item) {
    mod_node_state(page_pgdat(page), item, 1, 1);
    }
    EXPORT_SYMBOL(inc_node_page_state);
#[no_mangle]
pub unsafe extern "C" fn dec_node_page_state(page: *mut page, item: node_stat_item) {
    mod_node_state(page_pgdat(page), item, -1, -1);
    }
    EXPORT_SYMBOL(dec_node_page_state);

//
// Use interrupt disable to serialize counter updates
//
#[no_mangle]
#[no_mangle]
// duplicate fn: mod_zone_page_state
pub unsafe extern "C" fn mod_zone_page_state_dup(zone: *mut zone, item: zone_stat_item, delta: c_long) {
    let mut flags = 0;
    local_irq_save(flags);
    __mod_zone_page_state(zone, item, delta);
    local_irq_restore(flags);
    }
    EXPORT_SYMBOL(mod_zone_page_state);
#[no_mangle]
#[no_mangle]
// duplicate fn: inc_zone_page_state
pub unsafe extern "C" fn inc_zone_page_state_dup(page: *mut page, item: zone_stat_item) {
    let mut flags = 0;
pub static mut zone: *mut c_void = core::ptr::null_mut();
    zone = page_zone(page);
    local_irq_save(flags);
    __inc_zone_state(zone, item);
    local_irq_restore(flags);
    }
    EXPORT_SYMBOL(inc_zone_page_state);
#[no_mangle]
#[no_mangle]
// duplicate fn: dec_zone_page_state
pub unsafe extern "C" fn dec_zone_page_state_dup(page: *mut page, item: zone_stat_item) {
    let mut flags = 0;
    local_irq_save(flags);
    __dec_zone_page_state(page, item);
    local_irq_restore(flags);
    }
    EXPORT_SYMBOL(dec_zone_page_state);
#[no_mangle]
#[no_mangle]
// duplicate fn: mod_node_page_state
pub unsafe extern "C" fn mod_node_page_state_dup(pgdat: *mut pglist_data, item: node_stat_item, delta: c_long) {
    let mut flags = 0;
    local_irq_save(flags);
    __mod_node_page_state(pgdat, item, delta);
    local_irq_restore(flags);
    }
    EXPORT_SYMBOL(mod_node_page_state);
#[no_mangle]
#[no_mangle]
// duplicate fn: inc_node_page_state
pub unsafe extern "C" fn inc_node_page_state_dup(page: *mut page, item: node_stat_item) {
    let mut flags = 0;
pub static mut pgdat: *mut c_void = core::ptr::null_mut();
    pgdat = page_pgdat(page);
    local_irq_save(flags);
    __inc_node_state(pgdat, item);
    local_irq_restore(flags);
    }
    EXPORT_SYMBOL(inc_node_page_state);
#[no_mangle]
#[no_mangle]
// duplicate fn: dec_node_page_state
pub unsafe extern "C" fn dec_node_page_state_dup(page: *mut page, item: node_stat_item) {
    let mut flags = 0;
    local_irq_save(flags);
    __dec_node_page_state(page, item);
    local_irq_restore(flags);
    }
    EXPORT_SYMBOL(dec_node_page_state);

//
// Fold a differential into the global counters.
// Returns whether counters were updated.
//
#[no_mangle]
unsafe extern "C" fn fold_diff(zone_diff: *mut c_int, node_diff: *mut c_int) -> c_int {
    let mut i = 0;
pub static mut changed: bool = false;
    while (i < NR_VM_ZONE_STAT_ITEMS) {
    if (zone_diff[i]) {
    atomic_long_add(zone_diff[i], &vm_zone_stat[i]);
    changed = true;
    }
    }
    while (i < NR_VM_NODE_STAT_ITEMS) {
    if (node_diff[i]) {
    atomic_long_add(node_diff[i], &vm_node_stat[i]);
    changed = true;
    }
    }
    return changed;
    }
//
// Update the zone counters for the current cpu.
//
// Note that refresh_cpu_vm_stats strives to only access
// node local memory. The per cpu pagesets on remote zones are placed
// in the memory local to the processor using that pageset. So the
// loop over all zones will access a series of cachelines local to
// the processor.
//
// The call to zone_page_state_add updates the cachelines with the
// statistics in the remote zone struct as well as the global cachelines
// with the global counters. These could cause remote node cache line
// bouncing and will have to be only done when necessary.
//
// The function returns whether global counters were updated.
//
#[no_mangle]
unsafe extern "C" fn refresh_cpu_vm_stats(do_pagesets: bool) -> bool {
pub static mut pgdat: *mut c_void = core::ptr::null_mut();
pub static mut zone: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    int global_zone_diff[NR_VM_ZONE_STAT_ITEMS] = { 0, };
    int global_node_diff[NR_VM_NODE_STAT_ITEMS] = { 0, };
pub static mut changed: bool = false;
    for_each_populated_zone(zone) {
    let mut pzstats = zone.per_cpu_zonestats;
    let mut pcp = zone.per_cpu_pageset;
    while (i < NR_VM_ZONE_STAT_ITEMS) {
    let mut v = 0;
    v = this_cpu_xchg(pzstats.vm_stat_diff[i], 0);
    if (v) {
    atomic_long_add(v, &zone.vm_stat[i]);
    global_zone_diff[i] += v;

// 3 seconds idle till flush
    __this_cpu_write(pcp.expire, 3);

    }
    }
    if (do_pagesets) {
    cond_resched();
    if (decay_pcp_high(zone, this_cpu_ptr(pcp))) {
    changed = true;
    }

//
// Deal with draining the remote pageset of this
// processor
//
// Check if there are pages remaining in this pageset
// if not then there is nothing to expire.
//
    if (!__this_cpu_read(pcp.expire) ||
    !__this_cpu_read(pcp.count)) {
    continue;
    }
//
// We never drain zones local to this processor.
//
    if (zone_to_nid(zone) == numa_node_id()) {
    __this_cpu_write(pcp.expire, 0);
    continue;
    }
    if (__this_cpu_dec_return(pcp.expire)) {
    changed = true;
    continue;
    }
    if (__this_cpu_read(pcp.count)) {
    drain_zone_pages(zone, this_cpu_ptr(pcp));
    changed = true;
    }

    }
    }
    for_each_online_pgdat(pgdat) {
    let mut p = pgdat.per_cpu_nodestats;
    while (i < NR_VM_NODE_STAT_ITEMS) {
    let mut v = 0;
    v = this_cpu_xchg(p.vm_node_stat_diff[i], 0);
    if (v) {
    atomic_long_add(v, &pgdat.vm_stat[i]);
    global_node_diff[i] += v;
    }
    }
    }
    if (fold_diff(global_zone_diff, global_node_diff)) {
    changed = true;
    }
    return changed;
    }
//
// Fold the data for an offline cpu into the global array.
// There cannot be any access by the offline cpu and therefore
// synchronization is simplified.
//
#[no_mangle]
pub unsafe extern "C" fn cpu_vm_stats_fold(cpu: c_int) {
pub static mut pgdat: *mut c_void = core::ptr::null_mut();
pub static mut zone: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    int global_zone_diff[NR_VM_ZONE_STAT_ITEMS] = { 0, };
    int global_node_diff[NR_VM_NODE_STAT_ITEMS] = { 0, };
    for_each_populated_zone(zone) {
pub static mut pzstats: *mut c_void = core::ptr::null_mut();
    pzstats = per_cpu_ptr(zone.per_cpu_zonestats, cpu);
    while (i < NR_VM_ZONE_STAT_ITEMS) {
    if (pzstats.vm_stat_diff[i]) {
    let mut v = 0;
    v = pzstats.vm_stat_diff[i];
    pzstats.vm_stat_diff[i] = 0;
    atomic_long_add(v, &zone.vm_stat[i]);
    global_zone_diff[i] += v;
    }
    }

    while (i < NR_VM_NUMA_EVENT_ITEMS) {
    if (pzstats.vm_numa_event[i]) {
    let mut v = 0;
    v = pzstats.vm_numa_event[i];
    pzstats.vm_numa_event[i] = 0;
    zone_numa_event_add(v, zone, i);
    }
    }

    }
    for_each_online_pgdat(pgdat) {
pub static mut p: *mut c_void = core::ptr::null_mut();
    p = per_cpu_ptr(pgdat.per_cpu_nodestats, cpu);
    for (i = 0; i < NR_VM_NODE_STAT_ITEMS; i++) {
    if (p.vm_node_stat_diff[i]) {
    }
    let mut v = 0;
    v = p.vm_node_stat_diff[i];
    p.vm_node_stat_diff[i] = 0;
    atomic_long_add(v, &pgdat.vm_stat[i]);
    global_node_diff[i] += v;
    }
    }
    fold_diff(global_zone_diff, global_node_diff);
    }
//
// this is only called if !populated_zone(zone), which implies no other users of
// pset->vm_stat_diff[] exist.
//
#[no_mangle]
pub unsafe extern "C" fn drain_zonestat(zone: *mut zone, pzstats: *mut per_cpu_zonestat) {
    let mut v = 0;
    let mut i = 0;
    while (i < NR_VM_ZONE_STAT_ITEMS) {
    if (pzstats.vm_stat_diff[i]) {
    v = pzstats.vm_stat_diff[i];
    pzstats.vm_stat_diff[i] = 0;
    zone_page_state_add(v, zone, i);
    }
    }

    while (i < NR_VM_NUMA_EVENT_ITEMS) {
    if (pzstats.vm_numa_event[i]) {
    v = pzstats.vm_numa_event[i];
    pzstats.vm_numa_event[i] = 0;
    zone_numa_event_add(v, zone, i);
    }
    }

    }

//
// Determine the per node value of a stat item. This function
// is called frequently in a NUMA machine, so try to be as
// frugal as possible.
//
#[no_mangle]
pub unsafe extern "C" fn sum_zone_node_page_state(node: c_int, item: zone_stat_item) -> c_ulong {
    let mut zones = NODE_DATA(node).node_zones;
    let mut i = 0;
pub static mut count: c_ulong = 0;
    for (i = 0; i < MAX_NR_ZONES; i++) {
    count += zone_page_state(zones + i, item);
    }
    return count;
    }
// Determine the per node value of a numa stat item.
#[no_mangle]
pub unsafe extern "C" fn sum_zone_numa_event_state(node: c_int, item: numa_stat_item) -> c_ulong {
    let mut zones = NODE_DATA(node).node_zones;
pub static mut count: c_ulong = 0;
    let mut i = 0;
    for (i = 0; i < MAX_NR_ZONES; i++) {
    count += zone_numa_event_state(zones + i, item);
    }
    return count;
    }
//
// Determine the per node value of a stat item.
//
#[no_mangle]
pub unsafe extern "C" fn node_page_state_pages(pgdat: *mut pglist_data, item: node_stat_item) -> c_ulong {
pub static mut x: c_long = 0;

    if (x < 0) {
    x = 0;
    }

    return x;
    }
#[no_mangle]
pub unsafe extern "C" fn node_page_state(pgdat: *mut pglist_data, item: node_stat_item) -> c_ulong {
    VM_WARN_ON_ONCE(vmstat_item_in_bytes(item));
    return node_page_state_pages(pgdat, item);
    }
//
// Non-clamping variant of node_page_state() intended for callers that
// snapshot a monotonically-incremented counter and subtract two samples.
// See global_node_page_state_monotonic() for the rationale.
//
#[no_mangle]
pub unsafe extern "C" fn node_page_state_monotonic(pgdat: *mut pglist_data, item: node_stat_item) -> c_ulong {
    return (unsigned long)atomic_long_read(&pgdat.vm_stat[item]);
    }

//
// Count number of pages "struct page" and "struct page_ext" consume.
// nr_memmap_boot_pages: # of pages allocated by boot allocator
// nr_memmap_pages: # of pages that were allocated by buddy allocator
//
pub static mut nr_memmap_boot_pages: atomic_long_t = 0;
pub static mut nr_memmap_pages: atomic_long_t = 0;
#[no_mangle]
pub unsafe extern "C" fn memmap_boot_pages_add(delta: c_long) {
    atomic_long_add(delta, &nr_memmap_boot_pages);
    }
#[no_mangle]
pub unsafe extern "C" fn memmap_pages_add(delta: c_long) {
    atomic_long_add(delta, &nr_memmap_pages);
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct contig_page_info {
    pub free_pages: c_ulong,
    pub free_blocks_total: c_ulong,
    pub free_blocks_suitable: c_ulong,
}

//
// Calculate the number of free pages in a zone, how many contiguous
// pages are free and how many are large enough to satisfy an allocation of
// the target size. Note that this function makes no attempt to estimate
// how many suitable free blocks there *might* be if MOVABLE pages were
// migrated. Calculating that is possible, but expensive and can be
// figured out from userspace
//
#[no_mangle]
pub unsafe extern "C" fn fill_contig_page_info(zone: *mut zone, suitable_order: c_uint, info: *mut contig_page_info) {
    let mut order = 0;
    info.free_pages = 0;
    info.free_blocks_total = 0;
    info.free_blocks_suitable = 0;
    while (order < NR_PAGE_ORDERS) {
    let mut blocks = 0;
//
// Count number of free blocks.
//
// Access to nr_free is lockless as nr_free is used only for
// diagnostic purposes. Use data_race to avoid KCSAN warning.
//
    blocks = data_race(zone.free_area[order].nr_free);
    info.free_blocks_total += blocks;
// Count free base pages
    info.free_pages += blocks << order;
// Count the suitable free blocks
    if (order >= suitable_order) {
    info.free_blocks_suitable += blocks <<
    (order - suitable_order);
    }
    }
    }
//
// A fragmentation index only makes sense if an allocation of a requested
// size would fail. If that is true, the fragmentation index indicates
// whether external fragmentation or a lack of memory was the problem.
// The value can be used to determine if page reclaim or compaction
// should be used
//
#[no_mangle]
unsafe extern "C" fn __fragmentation_index(order: c_uint, info: *mut contig_page_info) -> c_int {
pub static mut requested: c_ulong = 0;
    if (WARN_ON_ONCE!(order > MAX_PAGE_ORDER)) {
    return 0;
    }
    if (!info.free_blocks_total) {
    return 0;
    }
// Fragmentation index only makes sense when a request would fail
    if (info.free_blocks_suitable) {
    return -1000;
    }
//
// Index is between 0 and 1 so return within 3 decimal places
//
// 0 => allocation would fail due to lack of memory
// 1 => allocation would fail due to fragmentation
//
    return 1000 - div_u64( (1000+(div_u64(info.free_pages * 1000ULL, requested))), info.free_blocks_total);
    }
//
// Calculates external fragmentation within a zone wrt the given order.
// It is defined as the percentage of pages found in blocks of size
// less than 1 << order. It returns values in range [0, 100].
//
#[no_mangle]
pub unsafe extern "C" fn extfrag_for_order(zone: *mut zone, order: c_uint) -> c_uint {
pub static mut info: usize = 0;
    fill_contig_page_info(zone, order, &info);
    if (info.free_pages == 0) {
    return 0;
    }
    return div_u64((info.free_pages -
    (info.free_blocks_suitable << order)) * 100,
    info.free_pages);
    }
// Same as __fragmentation index but allocs contig_page_info on stack
#[no_mangle]
pub unsafe extern "C" fn fragmentation_index(zone: *mut zone, order: c_uint) -> c_int {
pub static mut info: usize = 0;
    fill_contig_page_info(zone, order, &info);
    return __fragmentation_index(order, &info);
    }

    defined(CONFIG_NUMA) || defined(CONFIG_MEMCG)

    TEXT_FOR_DMA(xx, yy)			
    TEXT_FOR_DMA32(xx, yy)			
    [xx##_NORMAL] = yy "_normal",		
    TEXT_FOR_HIGHMEM(xx, yy)		
    [xx##_MOVABLE] = yy "_movable",		
    TEXT_FOR_DEVICE(xx, yy)
    const char * const vmstat_text[] = {
// enum zone_stat_item counters

    [I(NR_FREE_PAGES)]			= "nr_free_pages",
    [I(NR_FREE_PAGES_BLOCKS)]		= "nr_free_pages_blocks",
    [I(NR_ZONE_INACTIVE_ANON)]		= "nr_zone_inactive_anon",
    [I(NR_ZONE_ACTIVE_ANON)]		= "nr_zone_active_anon",
    [I(NR_ZONE_INACTIVE_FILE)]		= "nr_zone_inactive_file",
    [I(NR_ZONE_ACTIVE_FILE)]		= "nr_zone_active_file",
    [I(NR_ZONE_UNEVICTABLE)]		= "nr_zone_unevictable",
    [I(NR_ZONE_WRITE_PENDING)]		= "nr_zone_write_pending",
    [I(NR_MLOCK)]				= "nr_mlock",

    [I(NR_ZSPAGES)]				= "nr_zspages",

    [I(NR_FREE_CMA_PAGES)]			= "nr_free_cma",

    [I(NR_UNACCEPTED)]			= "nr_unaccepted",

// enum numa_stat_item counters

    [I(NUMA_HIT)]				= "numa_hit",
    [I(NUMA_MISS)]				= "numa_miss",
    [I(NUMA_FOREIGN)]			= "numa_foreign",
    [I(NUMA_INTERLEAVE_HIT)]		= "numa_interleave",
    [I(NUMA_LOCAL)]				= "numa_local",
    [I(NUMA_OTHER)]				= "numa_other",

// enum node_stat_item counters

    [I(NR_INACTIVE_ANON)]			= "nr_inactive_anon",
    [I(NR_ACTIVE_ANON)]			= "nr_active_anon",
    [I(NR_INACTIVE_FILE)]			= "nr_inactive_file",
    [I(NR_ACTIVE_FILE)]			= "nr_active_file",
    [I(NR_UNEVICTABLE)]			= "nr_unevictable",
    [I(NR_SLAB_RECLAIMABLE_B)]		= "nr_slab_reclaimable",
    [I(NR_SLAB_UNRECLAIMABLE_B)]		= "nr_slab_unreclaimable",
    [I(NR_ISOLATED_ANON)]			= "nr_isolated_anon",
    [I(NR_ISOLATED_FILE)]			= "nr_isolated_file",
    [I(WORKINGSET_NODES)]			= "workingset_nodes",
    [I(WORKINGSET_REFAULT_ANON)]		= "workingset_refault_anon",
    [I(WORKINGSET_REFAULT_FILE)]		= "workingset_refault_file",
    [I(WORKINGSET_ACTIVATE_ANON)]		= "workingset_activate_anon",
    [I(WORKINGSET_ACTIVATE_FILE)]		= "workingset_activate_file",
    [I(WORKINGSET_RESTORE_ANON)]		= "workingset_restore_anon",
    [I(WORKINGSET_RESTORE_FILE)]		= "workingset_restore_file",
    [I(WORKINGSET_NODERECLAIM)]		= "workingset_nodereclaim",
    [I(NR_ANON_MAPPED)]			= "nr_anon_pages",
    [I(NR_FILE_MAPPED)]			= "nr_mapped",
    [I(NR_FILE_PAGES)]			= "nr_file_pages",
    [I(NR_FILE_DIRTY)]			= "nr_dirty",
    [I(NR_WRITEBACK)]			= "nr_writeback",
    [I(NR_SHMEM)]				= "nr_shmem",
    [I(NR_SHMEM_THPS)]			= "nr_shmem_hugepages",
    [I(NR_SHMEM_PMDMAPPED)]			= "nr_shmem_pmdmapped",
    [I(NR_FILE_THPS)]			= "nr_file_hugepages",
    [I(NR_FILE_PMDMAPPED)]			= "nr_file_pmdmapped",
    [I(NR_ANON_THPS)]			= "nr_anon_transparent_hugepages",
    [I(NR_VMSCAN_WRITE)]			= "nr_vmscan_write",
    [I(NR_VMSCAN_IMMEDIATE)]		= "nr_vmscan_immediate_reclaim",
    [I(NR_DIRTIED)]				= "nr_dirtied",
    [I(NR_WRITTEN)]				= "nr_written",
    [I(NR_THROTTLED_WRITTEN)]		= "nr_throttled_written",
    [I(NR_KERNEL_MISC_RECLAIMABLE)]		= "nr_kernel_misc_reclaimable",
    [I(NR_FOLL_PIN_ACQUIRED)]		= "nr_foll_pin_acquired",
    [I(NR_FOLL_PIN_RELEASED)]		= "nr_foll_pin_released",
    [I(NR_VMALLOC)]				= "nr_vmalloc",
    [I(NR_KERNEL_STACK_KB)]			= "nr_kernel_stack",

    [I(NR_KERNEL_SCS_KB)]			= "nr_shadow_call_stack",

    [I(NR_PAGETABLE)]			= "nr_page_table_pages",
    [I(NR_SECONDARY_PAGETABLE)]		= "nr_sec_page_table_pages",

    [I(NR_IOMMU_PAGES)]			= "nr_iommu_pages",

    [I(NR_SWAPCACHE)]			= "nr_swapcached",

    [I(PGPROMOTE_SUCCESS)]			= "pgpromote_success",
    [I(PGPROMOTE_CANDIDATE)]		= "pgpromote_candidate",
    [I(PGPROMOTE_CANDIDATE_NRL)]		= "pgpromote_candidate_nrl",

    [I(PGDEMOTE_KSWAPD)]			= "pgdemote_kswapd",
    [I(PGDEMOTE_DIRECT)]			= "pgdemote_direct",
    [I(PGDEMOTE_KHUGEPAGED)]		= "pgdemote_khugepaged",
    [I(PGDEMOTE_PROACTIVE)]			= "pgdemote_proactive",
    [I(PGSTEAL_KSWAPD)]			= "pgsteal_kswapd",
    [I(PGSTEAL_DIRECT)]			= "pgsteal_direct",
    [I(PGSTEAL_KHUGEPAGED)]			= "pgsteal_khugepaged",
    [I(PGSTEAL_PROACTIVE)]			= "pgsteal_proactive",
    [I(PGSTEAL_ANON)]			= "pgsteal_anon",
    [I(PGSTEAL_FILE)]			= "pgsteal_file",
    [I(PGSCAN_KSWAPD)]			= "pgscan_kswapd",
    [I(PGSCAN_DIRECT)]			= "pgscan_direct",
    [I(PGSCAN_KHUGEPAGED)]			= "pgscan_khugepaged",
    [I(PGSCAN_PROACTIVE)]			= "pgscan_proactive",
    [I(PGSCAN_ANON)]			= "pgscan_anon",
    [I(PGSCAN_FILE)]			= "pgscan_file",
    [I(PGROTATE_ANON)]			= "pgrotate_anon",
    [I(PGROTATE_FILE)]			= "pgrotate_file",
    [I(PGREFILL)]				= "pgrefill",

    [I(NR_HUGETLB)]				= "nr_hugetlb",

    [I(NR_BALLOON_PAGES)]			= "nr_balloon_pages",
    [I(NR_KERNEL_FILE_PAGES)]		= "nr_kernel_file_pages",
    [I(NR_GPU_ACTIVE)]			= "nr_gpu_active",
    [I(NR_GPU_RECLAIM)]			= "nr_gpu_reclaim",

// system-wide enum vm_stat_item counters

    NR_VM_NODE_STAT_ITEMS + x)
    [I(NR_DIRTY_THRESHOLD)]			= "nr_dirty_threshold",
    [I(NR_DIRTY_BG_THRESHOLD)]		= "nr_dirty_background_threshold",
    [I(NR_MEMMAP_PAGES)]			= "nr_memmap_pages",
    [I(NR_MEMMAP_BOOT_PAGES)]		= "nr_memmap_boot_pages",

// enum vm_event_item counters

    NR_VM_NODE_STAT_ITEMS + NR_VM_STAT_ITEMS + x)
    [I(PGPGIN)]				= "pgpgin",
    [I(PGPGOUT)]				= "pgpgout",
    [I(PSWPIN)]				= "pswpin",
    [I(PSWPOUT)]				= "pswpout",

    NR_VM_NODE_STAT_ITEMS + NR_VM_STAT_ITEMS)
    TEXTS_FOR_ZONES(OFF+PGALLOC, "pgalloc")
    TEXTS_FOR_ZONES(OFF+ALLOCSTALL, "allocstall")
    TEXTS_FOR_ZONES(OFF+PGSCAN_SKIP, "pgskip")

    [I(PGFREE)]				= "pgfree",
    [I(PGACTIVATE)]				= "pgactivate",
    [I(PGDEACTIVATE)]			= "pgdeactivate",
    [I(PGLAZYFREE)]				= "pglazyfree",
    [I(PGFAULT)]				= "pgfault",
    [I(PGMAJFAULT)]				= "pgmajfault",
    [I(PGLAZYFREED)]			= "pglazyfreed",
    [I(PGREUSE)]				= "pgreuse",
    [I(PGSCAN_DIRECT_THROTTLE)]		= "pgscan_direct_throttle",

    [I(PGSCAN_ZONE_RECLAIM_SUCCESS)]	= "zone_reclaim_success",
    [I(PGSCAN_ZONE_RECLAIM_FAILED)]		= "zone_reclaim_failed",

    [I(PGINODESTEAL)]			= "pginodesteal",
    [I(SLABS_SCANNED)]			= "slabs_scanned",
    [I(KSWAPD_INODESTEAL)]			= "kswapd_inodesteal",
    [I(KSWAPD_LOW_WMARK_HIT_QUICKLY)]	= "kswapd_low_wmark_hit_quickly",
    [I(KSWAPD_HIGH_WMARK_HIT_QUICKLY)]	= "kswapd_high_wmark_hit_quickly",
    [I(PAGEOUTRUN)]				= "pageoutrun",
    [I(PGROTATED)]				= "pgrotated",
    [I(DROP_PAGECACHE)]			= "drop_pagecache",
    [I(DROP_SLAB)]				= "drop_slab",
    [I(OOM_KILL)]				= "oom_kill",

    [I(NUMA_PTE_UPDATES)]			= "numa_pte_updates",
    [I(NUMA_HUGE_PTE_UPDATES)]		= "numa_huge_pte_updates",
    [I(NUMA_HINT_FAULTS)]			= "numa_hint_faults",
    [I(NUMA_HINT_FAULTS_LOCAL)]		= "numa_hint_faults_local",
    [I(NUMA_PAGE_MIGRATE)]			= "numa_pages_migrated",

    [I(PGMIGRATE_SUCCESS)]			= "pgmigrate_success",
    [I(PGMIGRATE_FAIL)]			= "pgmigrate_fail",
    [I(THP_MIGRATION_SUCCESS)]		= "thp_migration_success",
    [I(THP_MIGRATION_FAIL)]			= "thp_migration_fail",
    [I(THP_MIGRATION_SPLIT)]		= "thp_migration_split",

    [I(COMPACTMIGRATE_SCANNED)]		= "compact_migrate_scanned",
    [I(COMPACTFREE_SCANNED)]		= "compact_free_scanned",
    [I(COMPACTISOLATED)]			= "compact_isolated",
    [I(COMPACTSTALL)]			= "compact_stall",
    [I(COMPACTFAIL)]			= "compact_fail",
    [I(COMPACTSUCCESS)]			= "compact_success",
    [I(KCOMPACTD_WAKE)]			= "compact_daemon_wake",
    [I(KCOMPACTD_MIGRATE_SCANNED)]		= "compact_daemon_migrate_scanned",
    [I(KCOMPACTD_FREE_SCANNED)]		= "compact_daemon_free_scanned",

    [I(HTLB_BUDDY_PGALLOC)]			= "htlb_buddy_alloc_success",
    [I(HTLB_BUDDY_PGALLOC_FAIL)]		= "htlb_buddy_alloc_fail",

    [I(CMA_ALLOC_SUCCESS)]			= "cma_alloc_success",
    [I(CMA_ALLOC_FAIL)]			= "cma_alloc_fail",

    [I(UNEVICTABLE_PGCULLED)]		= "unevictable_pgs_culled",
    [I(UNEVICTABLE_PGSCANNED)]		= "unevictable_pgs_scanned",
    [I(UNEVICTABLE_PGRESCUED)]		= "unevictable_pgs_rescued",
    [I(UNEVICTABLE_PGMLOCKED)]		= "unevictable_pgs_mlocked",
    [I(UNEVICTABLE_PGMUNLOCKED)]		= "unevictable_pgs_munlocked",
    [I(UNEVICTABLE_PGCLEARED)]		= "unevictable_pgs_cleared",
    [I(UNEVICTABLE_PGSTRANDED)]		= "unevictable_pgs_stranded",

    [I(THP_FAULT_ALLOC)]			= "thp_fault_alloc",
    [I(THP_FAULT_FALLBACK)]			= "thp_fault_fallback",
    [I(THP_FAULT_FALLBACK_CHARGE)]		= "thp_fault_fallback_charge",
    [I(THP_COLLAPSE_ALLOC)]			= "thp_collapse_alloc",
    [I(THP_COLLAPSE_ALLOC_FAILED)]		= "thp_collapse_alloc_failed",
    [I(THP_FILE_ALLOC)]			= "thp_file_alloc",
    [I(THP_FILE_FALLBACK)]			= "thp_file_fallback",
    [I(THP_FILE_FALLBACK_CHARGE)]		= "thp_file_fallback_charge",
    [I(THP_FILE_MAPPED)]			= "thp_file_mapped",
    [I(THP_SPLIT_PAGE)]			= "thp_split_page",
    [I(THP_SPLIT_PAGE_FAILED)]		= "thp_split_page_failed",
    [I(THP_DEFERRED_SPLIT_PAGE)]		= "thp_deferred_split_page",
    [I(THP_UNDERUSED_SPLIT_PAGE)]		= "thp_underused_split_page",
    [I(THP_SPLIT_PMD)]			= "thp_split_pmd",
    [I(THP_SCAN_EXCEED_NONE_PTE)]		= "thp_scan_exceed_none_pte",
    [I(THP_SCAN_EXCEED_SWAP_PTE)]		= "thp_scan_exceed_swap_pte",
    [I(THP_SCAN_EXCEED_SHARED_PTE)]		= "thp_scan_exceed_share_pte",

    [I(THP_SPLIT_PUD)]			= "thp_split_pud",

    [I(THP_ZERO_PAGE_ALLOC)]		= "thp_zero_page_alloc",
    [I(THP_ZERO_PAGE_ALLOC_FAILED)]		= "thp_zero_page_alloc_failed",
    [I(THP_SWPOUT)]				= "thp_swpout",
    [I(THP_SWPOUT_FALLBACK)]		= "thp_swpout_fallback",

    [I(BALLOON_INFLATE)]			= "balloon_inflate",
    [I(BALLOON_DEFLATE)]			= "balloon_deflate",

    [I(BALLOON_MIGRATE)]			= "balloon_migrate",

    [I(NR_TLB_REMOTE_FLUSH)]		= "nr_tlb_remote_flush",
    [I(NR_TLB_REMOTE_FLUSH_RECEIVED)]	= "nr_tlb_remote_flush_received",
    [I(NR_TLB_LOCAL_FLUSH_ALL)]		= "nr_tlb_local_flush_all",
    [I(NR_TLB_LOCAL_FLUSH_ONE)]		= "nr_tlb_local_flush_one",

    [I(SWAP_RA)]				= "swap_ra",
    [I(SWAP_RA_HIT)]			= "swap_ra_hit",
    [I(SWPIN_ZERO)]				= "swpin_zero",
    [I(SWPOUT_ZERO)]			= "swpout_zero",

    [I(KSM_SWPIN_COPY)]			= "ksm_swpin_copy",

    [I(COW_KSM)]				= "cow_ksm",

    [I(ZSWPIN)]				= "zswpin",
    [I(ZSWPOUT)]				= "zswpout",
    [I(ZSWPWB)]				= "zswpwb",

    [I(DIRECT_MAP_LEVEL2_SPLIT)]		= "direct_map_level2_splits",
    [I(DIRECT_MAP_LEVEL3_SPLIT)]		= "direct_map_level3_splits",
    [I(DIRECT_MAP_LEVEL2_COLLAPSE)]		= "direct_map_level2_collapses",
    [I(DIRECT_MAP_LEVEL3_COLLAPSE)]		= "direct_map_level3_collapses",

    [I(VMA_LOCK_SUCCESS)]			= "vma_lock_success",
    [I(VMA_LOCK_ABORT)]			= "vma_lock_abort",
    [I(VMA_LOCK_RETRY)]			= "vma_lock_retry",
    [I(VMA_LOCK_MISS)]			= "vma_lock_miss",

    [I(KSTACK_1K)]				= "kstack_1k",

    [I(KSTACK_2K)]				= "kstack_2k",

    [I(KSTACK_4K)]				= "kstack_4k",

    [I(KSTACK_8K)]				= "kstack_8k",

    [I(KSTACK_16K)]				= "kstack_16k",

    [I(KSTACK_32K)]				= "kstack_32k",

    [I(KSTACK_64K)]				= "kstack_64k",

    [I(KSTACK_REST)]			= "kstack_rest",

    [I(NRSWPIN)]				= "nrswpin",
    [I(NRSWPOUT)]				= "nrswpout",

    };

    defined(CONFIG_PROC_FS)
#[no_mangle]
pub unsafe extern "C" fn frag_start(m: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
pub static mut pgdat: *mut c_void = core::ptr::null_mut();
pub static mut node: loff_t = 0;
    for (pgdat = first_online_pgdat();
    pgdat && node;
    pgdat = next_online_pgdat(pgdat)) {
    node -= 1;
    }
    return pgdat;
    }
#[no_mangle]
pub unsafe extern "C" fn frag_next(m: *mut seq_file, arg: *mut c_void, pos: *mut loff_t) -> *mut c_void {
    let mut pgdat = arg;
    (*pos)++;
    return next_online_pgdat(pgdat);
    }
#[no_mangle]
unsafe extern "C" fn frag_stop(m: *mut seq_file, arg: *mut c_void) {
    }
//
// Walk zones in a node and print using a callback.
// If @assert_populated is true, only use callback for zones that are populated.
//
#[no_mangle]
pub unsafe extern "C" fn walk_zones_in_node(m: *mut seq_file, pgdat: *mut pg_data_t, assert_populated: bool, nolock: bool, m: *mut *mut c_void (print)( seq_file) {
pub static mut zone: *mut c_void = core::ptr::null_mut();
    let mut node_zones = pgdat.node_zones;
    let mut flags = 0;
    while (zone - node_zones < MAX_NR_ZONES) {
    if (assert_populated && !populated_zone(zone)) {
    continue;
    }
    if (!nolock) {
    spin_lock_irqsave(&zone.lock, flags);
    }
    print(m, pgdat, zone);
    if (!nolock) {
    spin_unlock_irqrestore(&zone.lock, flags);
    }
    }
    }

#[no_mangle]
pub unsafe extern "C" fn frag_show_print(m: *mut seq_file, pgdat: *mut pg_data_t, zone: *mut zone) {
    let mut order = 0;
    seq_printf(m, "Node %d, zone %8s ", pgdat.node_id, zone.name);
    for (order = 0; order < NR_PAGE_ORDERS; ++order) {
//
// Access to nr_free is lockless as nr_free is used only for
// printing purposes. Use data_race to avoid KCSAN warning.
//
    seq_printf(m, "%6lu ", data_race(zone.free_area[order].nr_free));
    }
    seq_putc(m, '\n');
    }
//
// This walks the free areas for each zone.
//
#[no_mangle]
unsafe extern "C" fn frag_show(m: *mut seq_file, arg: *mut c_void) -> c_int {
    let mut pgdat = arg;
    walk_zones_in_node(m, pgdat, true, true, frag_show_print);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn pagetypeinfo_showfree_print(m: *mut seq_file, pgdat: *mut pg_data_t, zone: *mut zone) {
    let mut order = 0;
    let mut mtype = 0;
    while (mtype < MIGRATE_TYPES) {
    seq_printf(m, "Node %4d, zone %8s, type %12s ",
    pgdat.node_id,
    zone.name,
    migratetype_names[mtype]);
    while (order < NR_PAGE_ORDERS) {
pub static mut freecount: c_ulong = 0;
pub static mut area: *mut c_void = core::ptr::null_mut();
pub static mut curr: *mut c_void = core::ptr::null_mut();
pub static mut overflow: bool = false;
    area = &(zone.free_area[order]);
    list_for_each(curr, &area.free_list[mtype]) {
//
// Cap the free_list iteration because it might
// be really large and we are under a spinlock
// so a long time spent here could trigger a
// hard lockup detector. Anyway this is a
// debugging tool so knowing there is a handful
// of pages of this order should be more than
// sufficient.
//
    if (++freecount >= 100000) {
    overflow = true;
    break;
    }
    }
    seq_printf(m, "%s%6lu ", overflow ? ">" : "", freecount);
    spin_unlock_irq(&zone.lock);
    cond_resched();
    spin_lock_irq(&zone.lock);
    }
    seq_putc(m, '\n');
    }
    }
// Print out the free pages at each order for each migratetype
#[no_mangle]
unsafe extern "C" fn pagetypeinfo_showfree(m: *mut seq_file, arg: *mut c_void) {
    let mut order = 0;
    let mut pgdat = arg;
// Print header
    seq_printf(m, "%-43s ", "Free pages count per migrate type at order");
    for (order = 0; order < NR_PAGE_ORDERS; ++order) {
    seq_printf(m, "%6d ", order);
    }
    seq_putc(m, '\n');
    walk_zones_in_node(m, pgdat, true, false, pagetypeinfo_showfree_print);
    }
#[no_mangle]
pub unsafe extern "C" fn pagetypeinfo_showblockcount_print(m: *mut seq_file, pgdat: *mut pg_data_t, zone: *mut zone) {
    let mut mtype = 0;
    let mut pfn = 0;
pub static mut start_pfn: c_ulong = 0;
pub static mut end_pfn: c_ulong = 0;
    unsigned long count[MIGRATE_TYPES] = { 0, };
    while (pfn < end_pfn) {
pub static mut page: *mut c_void = core::ptr::null_mut();
    page = pfn_to_online_page(pfn);
    if (!page) {
    continue;
    }
    if (page_zone(page) != zone) {
    continue;
    }
    mtype = get_pageblock_migratetype(page);
    if (mtype < MIGRATE_TYPES) {
    count[mtype]++;
    }
    }
// Print counts
    seq_printf(m, "Node %d, zone %8s ", pgdat.node_id, zone.name);
    for (mtype = 0; mtype < MIGRATE_TYPES; mtype++) {
    seq_printf(m, "%12lu ", count[mtype]);
    }
    seq_putc(m, '\n');
    }
// Print out the number of pageblocks for each migratetype
#[no_mangle]
unsafe extern "C" fn pagetypeinfo_showblockcount(m: *mut seq_file, arg: *mut c_void) {
    let mut mtype = 0;
    let mut pgdat = arg;
    seq_printf(m, "\n%-23s", "Number of blocks type ");
    for (mtype = 0; mtype < MIGRATE_TYPES; mtype++) {
    seq_printf(m, "%12s ", migratetype_names[mtype]);
    }
    seq_putc(m, '\n');
    walk_zones_in_node(m, pgdat, true, false,
    pagetypeinfo_showblockcount_print);
    }
//
// Print out the number of pageblocks for each migratetype that contain pages
// of other types. This gives an indication of how well fallbacks are being
// contained by rmqueue_fallback(). It requires information from PAGE_OWNER
// to determine what is going on
//
#[no_mangle]
unsafe extern "C" fn pagetypeinfo_showmixedcount(m: *mut seq_file, pgdat: *mut pg_data_t) {

    let mut mtype = 0;
    if (!static_branch_unlikely(&page_owner_inited)) {
    return;
    }
    drain_all_pages(core::ptr::null_mut());
    seq_printf(m, "\n%-23s", "Number of mixed blocks ");
    for (mtype = 0; mtype < MIGRATE_TYPES; mtype++) {
    seq_printf(m, "%12s ", migratetype_names[mtype]);
    }
    seq_putc(m, '\n');
    walk_zones_in_node(m, pgdat, true, true,
    pagetypeinfo_showmixedcount_print);

    }
//
// This prints out statistics in relation to grouping pages by mobility.
// It is expensive to collect so do not constantly read the file.
//
#[no_mangle]
unsafe extern "C" fn pagetypeinfo_show(m: *mut seq_file, arg: *mut c_void) -> c_int {
    let mut pgdat = arg;
// check memoryless node
    if (!node_state(pgdat.node_id, N_MEMORY)) {
    return 0;
    }
    seq_printf(m, "Page block order: %d\n", pageblock_order);
    seq_printf(m, "Pages per block:  %lu\n", pageblock_nr_pages);
    seq_putc(m, '\n');
    pagetypeinfo_showfree(m, pgdat);
    pagetypeinfo_showblockcount(m, pgdat);
    pagetypeinfo_showmixedcount(m, pgdat);
    return 0;
    }
pub static mut seq_operations: usize = 0;
pub static mut seq_operations: usize = 0;
#[no_mangle]
unsafe extern "C" fn is_zone_first_populated(pgdat: *mut pg_data_t, zone: *mut zone) -> bool {
    let mut zid = 0;
    while (zid < MAX_NR_ZONES) {
    let mut compare = &pgdat.node_zones[zid];
    if (populated_zone(compare)) {
pub static mut zone: return = 0;
    }
    }
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn zoneinfo_show_print(m: *mut seq_file, pgdat: *mut pg_data_t, zone: *mut zone) {
    let mut i = 0;
    seq_printf(m, "Node %d, zone %8s", pgdat.node_id, zone.name);
    if (is_zone_first_populated(pgdat, zone)) {
    seq_printf(m, "\n  per-node stats");
    while (i < NR_VM_NODE_STAT_ITEMS) {
pub static mut pages: c_ulong = 0;
    if (vmstat_item_print_in_thp(i)) {
    pages /= HPAGE_PMD_NR;
    }
    seq_printf(m, "\n      %-12s %lu", node_stat_name(i),
    pages);
    }
    }
    seq_printf(m,
    "\n  pages free     %lu"
    "\n        boost    %lu"
    "\n        min      %lu"
    "\n        low      %lu"
    "\n        high     %lu"
    "\n        promo    %lu"
    "\n        spanned  %lu"
    "\n        present  %lu"
    "\n        managed  %lu"
    "\n        cma      %lu",
    zone_page_state(zone, NR_FREE_PAGES),
    zone.watermark_boost,
    min_wmark_pages(zone),
    low_wmark_pages(zone),
    high_wmark_pages(zone),
    promo_wmark_pages(zone),
    zone.spanned_pages,
    zone.present_pages,
    zone_managed_pages(zone),
    zone_cma_pages(zone));
    seq_printf(m,
    "\n        protection: (%ld",
    zone.lowmem_reserve[0]);
    for (i = 1; i < ARRAY_SIZE!(zone.lowmem_reserve); i++) {
    seq_printf(m, ", %ld", zone.lowmem_reserve[i]);
    }
    seq_putc(m, ')');
// If unpopulated, no other information is useful
    if (!populated_zone(zone)) {
    seq_putc(m, '\n');
    return;
    }
    for (i = 0; i < NR_VM_ZONE_STAT_ITEMS; i++) {
    seq_printf(m, "\n      %-12s %lu", zone_stat_name(i),
    zone_page_state(zone, i));
    }

    fold_vm_zone_numa_events(zone);
    for (i = 0; i < NR_VM_NUMA_EVENT_ITEMS; i++) {
    seq_printf(m, "\n      %-12s %lu", numa_stat_name(i),
    zone_numa_event_state(zone, i));
    }

    seq_printf(m, "\n  pagesets");
    for_each_online_cpu(i) {
pub static mut pcp: *mut c_void = core::ptr::null_mut();
    struct per_cpu_zonestat __maybe_unused *pzstats;
    pcp = per_cpu_ptr(zone.per_cpu_pageset, i);
    seq_printf(m,
    "\n    cpu: %i"
    "\n              count:    %i"
    "\n              high:     %i"
    "\n              batch:    %i"
    "\n              high_min: %i"
    "\n              high_max: %i",
    i,
    pcp.count,
    pcp.high,
    pcp.batch,
    pcp.high_min,
    pcp.high_max);

    pzstats = per_cpu_ptr(zone.per_cpu_zonestats, i);
    seq_printf(m, "\n  vm stats threshold: %d",
    pzstats.stat_threshold);

    }
    seq_printf(m,
    "\n  node_unreclaimable:  %u"
    "\n  start_pfn:           %lu"
    "\n  reserved_highatomic: %lu"
    "\n  free_highatomic:     %lu",
    kswapd_test_hopeless(pgdat),
    zone.zone_start_pfn,
    zone.nr_reserved_highatomic,
    zone.nr_free_highatomic);
    seq_putc(m, '\n');
    }
//
// Output information about zones in @pgdat.  All zones are printed regardless
// of whether they are populated or not: lowmem_reserve_ratio operates on the
// set of all zones and userspace would not be aware of such zones if they are
// suppressed here (zoneinfo displays the effect of lowmem_reserve_ratio).
//
#[no_mangle]
unsafe extern "C" fn zoneinfo_show(m: *mut seq_file, arg: *mut c_void) -> c_int {
    let mut pgdat = arg;
    walk_zones_in_node(m, pgdat, false, false, zoneinfo_show_print);
    return 0;
    }
pub static mut seq_operations: usize = 0;

    NR_VM_NUMA_EVENT_ITEMS + 
    NR_VM_NODE_STAT_ITEMS + 
    NR_VM_STAT_ITEMS + 
    (IS_ENABLED!(CONFIG_VM_EVENT_COUNTERS) ? 
    NR_VM_EVENT_ITEMS : 0))
#[no_mangle]
pub unsafe extern "C" fn vmstat_start(m: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
pub static mut v: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    if (*pos >= NR_VMSTAT_ITEMS) {
    return core::ptr::null_mut();
    }
    BUILD_BUG_ON!(ARRAY_SIZE!(vmstat_text) != NR_VMSTAT_ITEMS);
    fold_vm_numa_events();
    v = kmalloc_array(NR_VMSTAT_ITEMS, sizeof!(unsigned long), GFP_KERNEL);
    m.private = v;
    if (!v) {
    return ERR_PTR(-ENOMEM);
    }
    for (i = 0; i < NR_VM_ZONE_STAT_ITEMS; i++) {
    v[i] = global_zone_page_state(i);
    }
    v += NR_VM_ZONE_STAT_ITEMS;

    for (i = 0; i < NR_VM_NUMA_EVENT_ITEMS; i++) {
    v[i] = global_numa_event_state(i);
    }
    v += NR_VM_NUMA_EVENT_ITEMS;

    while (i < NR_VM_NODE_STAT_ITEMS) {
    v[i] = global_node_page_state_pages(i);
    if (vmstat_item_print_in_thp(i)) {
    v[i] /= HPAGE_PMD_NR;
    }
    }
    v += NR_VM_NODE_STAT_ITEMS;
    global_dirty_limits(v + NR_DIRTY_BG_THRESHOLD,
    v + NR_DIRTY_THRESHOLD);
    v[NR_MEMMAP_PAGES] = atomic_long_read(&nr_memmap_pages);
    v[NR_MEMMAP_BOOT_PAGES] = atomic_long_read(&nr_memmap_boot_pages);
    v += NR_VM_STAT_ITEMS;

    all_vm_events(v);
    v[PGPGIN] /= 2;		/* sectors . kbytes */
    v[PGPGOUT] /= 2;

    return m.private + *pos;
    }
#[no_mangle]
pub unsafe extern "C" fn vmstat_next(m: *mut seq_file, arg: *mut c_void, pos: *mut loff_t) -> *mut c_void {
    (*pos)++;
    if (*pos >= NR_VMSTAT_ITEMS) {
    return core::ptr::null_mut();
    }
    return m.private + *pos;
    }
#[no_mangle]
unsafe extern "C" fn vmstat_show(m: *mut seq_file, arg: *mut c_void) -> c_int {
    let mut l = arg;
pub static mut off: c_ulong = 0;
    seq_puts(m, vmstat_text[off]);
    seq_put_decimal_ull(m, " ", *l);
    seq_putc(m, '\n');
    if (off == NR_VMSTAT_ITEMS - 1) {
//
// We've come to the end - add any deprecated counters to avoid
// breaking userspace which might depend on them being present.
//
    seq_puts(m, "nr_unstable 0\n");
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vmstat_stop(m: *mut seq_file, arg: *mut c_void) {
    kfree(m.private);
    m.private = core::ptr::null_mut();
    }
pub static mut seq_operations: usize = 0;

pub static mut struct delayed_work: usize = 0;
pub static mut : int sysctl_stat_interval = 0;
    static int vmstat_late_init_done;

#[no_mangle]
unsafe extern "C" fn refresh_vm_stats(work: *mut work_struct) {
    refresh_cpu_vm_stats(true);
    }
#[no_mangle]
pub unsafe extern "C" fn vmstat_refresh(table: *mut ctl_table, write: c_int, buffer: *mut c_void, lenp: *mut size_t, ppos: *mut loff_t) -> c_int {
    let mut val = 0;
    let mut err = 0;
    let mut i = 0;
//
// The regular update, every sysctl_stat_interval, may come later
// than expected: leaving a significant amount in per_cpu buckets.
// This is particularly misleading when checking a quantity of HUGE
// pages, immediately after running a test.  /proc/sys/vm/stat_refresh,
// which can equally be echo'ed to or cat'ted from (by root),
// can be used to update the stats just before reading them.
//
// Oh, and since global_zone_page_state() etc. are so careful to hide
// transiently negative values, report an error here if any of
// the stats is negative, so we know to go looking for imbalance.
//
    err = schedule_on_each_cpu(refresh_vm_stats);
    if (err) {
    return err;
    }
    while (i < NR_VM_ZONE_STAT_ITEMS) {
//
// Skip checking stats known to go negative occasionally.
//
    match (i) {
    NR_ZONE_WRITE_PENDING => {
    }
    NR_FREE_CMA_PAGES => {
    continue;
    }
    }
    val = atomic_long_read(&vm_zone_stat[i]);
    if (val < 0) {
    pr_warn!("%s: %s %ld\n",
    __func__, zone_stat_name(i), val);
    }
    }
    while (i < NR_VM_NODE_STAT_ITEMS) {
//
// Skip checking stats known to go negative occasionally.
//
    match (i) {
    NR_WRITEBACK => {
    continue;
    }
    }
    val = atomic_long_read(&vm_node_stat[i]);
    if (val < 0) {
    pr_warn!("%s: %s %ld\n",
    __func__, node_stat_name(i), val);
    }
    }
    if (write) {
// ppos += *lenp;
    }
    else {
// lenp = 0;
    }
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn vmstat_update(w: *mut work_struct) {
    if (refresh_cpu_vm_stats(true)) {
//
// Counters were updated so we expect more updates
// to occur in the future. Keep on running the
// update worker thread.
//
    queue_delayed_work_on(smp_processor_id(), mm_percpu_wq,
    this_cpu_ptr(&vmstat_work),
    round_jiffies_relative(sysctl_stat_interval));
    }
    }
//
// Check if the diffs for a certain cpu indicate that
// an update is needed.
//
#[no_mangle]
unsafe extern "C" fn need_update(cpu: c_int) -> bool {
    let mut last_pgdat = core::ptr::null_mut();
pub static mut zone: *mut c_void = core::ptr::null_mut();
    for_each_populated_zone(zone) {
    let mut pzstats = per_cpu_ptr(zone.per_cpu_zonestats, cpu);
pub static mut n: *mut c_void = core::ptr::null_mut();
//
// The fast way of checking if there are any vmstat diffs.
//
    if (memchr_inv(pzstats.vm_stat_diff, 0, sizeof!(pzstats.vm_stat_diff))) {
    return true;
    }
    if (last_pgdat == zone.zone_pgdat) {
    continue;
    }
    last_pgdat = zone.zone_pgdat;
    n = per_cpu_ptr(zone.zone_pgdat.per_cpu_nodestats, cpu);
    if (memchr_inv(n.vm_node_stat_diff, 0, sizeof!(n.vm_node_stat_diff))) {
    return true;
    }
    }
    return false;
    }
//
// Switch off vmstat processing and then fold all the remaining differentials
// until the diffs stay at zero. The function is used by NOHZ and can only be
// invoked when tick processing is not active.
//
#[no_mangle]
pub unsafe extern "C" fn quiet_vmstat() {
    if (system_state != SYSTEM_RUNNING) {
    return;
    }
    if (!delayed_work_pending(this_cpu_ptr(&vmstat_work))) {
    return;
    }
    if (!need_update(smp_processor_id())) {
    return;
    }
//
// Just refresh counters and do not care about the pending delayed
// vmstat_update. It doesn't fire that often to matter and canceling
// it would be too expensive from this path.
// vmstat_shepherd will take care about that for us.
//
    refresh_cpu_vm_stats(false);
    }
//
// Shepherd worker thread that checks the
// differentials of processors that have their worker
// threads for vm statistics updates disabled because of
// inactivity.
//
// forward_decl: vmstat_shepherd;
pub static mut shepherd: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn vmstat_flush_workqueue() {
    flush_workqueue(mm_percpu_wq);
    }
#[no_mangle]
unsafe extern "C" fn vmstat_shepherd(w: *mut work_struct) {
    let mut cpu = 0;
    cpus_read_lock();
// Check processors whose vmstat worker threads have been disabled
    for_each_online_cpu(cpu) {
    let mut dw = &per_cpu(vmstat_work, cpu);
//
// In kernel users of vmstat counters either require the precise value and
// they are using zone_page_state_snapshot interface or they can live with
// an imprecision as the regular flushing can happen at arbitrary time and
// cumulative error can grow (see calculate_normal_threshold).
//
// From that POV the regular flushing can be postponed for CPUs that have
// been isolated from the kernel interference without critical
// infrastructure ever noticing. Skip regular flushing from vmstat_shepherd
// for all isolated CPUs to avoid interference with the isolated workload.
//
    scoped_guard(rcu) {
    if (cpu_is_isolated(cpu)) {
    continue;
    }
    if (!work_busy(&dw.work) && need_update(cpu)) {
    queue_delayed_work_on(cpu, mm_percpu_wq, dw, 0);
    }
    }
    cond_resched();
    }
    cpus_read_unlock();
    schedule_delayed_work(&shepherd,
    round_jiffies_relative(sysctl_stat_interval));
    }
#[no_mangle]
unsafe extern "C" fn start_shepherd_timer()  {
    let mut cpu = 0;
    for_each_possible_cpu(cpu) {
    INIT_DEFERRABLE_WORK(per_cpu_ptr(&vmstat_work, cpu),
    vmstat_update);
//
// For secondary CPUs during CPU hotplug scenarios,
// vmstat_cpu_online() will enable the work.
// mm/vmstat:online enables and disables vmstat_work
// symmetrically during CPU hotplug events.
//
    if (!cpu_online(cpu)) {
    disable_delayed_work_sync(&per_cpu(vmstat_work, cpu));
    }
    }
    schedule_delayed_work(&shepherd,
    round_jiffies_relative(sysctl_stat_interval));
    }
#[no_mangle]
unsafe extern "C" fn init_cpu_node_state()  {
    let mut node = 0;
    for_each_online_node(node) {
    if (!cpumask_empty(cpumask_of_node(node))) {
    node_set_state(node, N_CPU);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn vmstat_cpu_online(cpu: c_uint) -> c_int {
    if (vmstat_late_init_done) {
    refresh_zone_stat_thresholds();
    }
    if (!node_state(cpu_to_node(cpu), N_CPU)) {
    node_set_state(cpu_to_node(cpu), N_CPU);
    }
    enable_delayed_work(&per_cpu(vmstat_work, cpu));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vmstat_cpu_down_prep(cpu: c_uint) -> c_int {
    disable_delayed_work_sync(&per_cpu(vmstat_work, cpu));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vmstat_cpu_dead(cpu: c_uint) -> c_int {
pub static mut node_cpus: *mut c_void = core::ptr::null_mut();
    let mut node = 0;
    node = cpu_to_node(cpu);
    refresh_zone_stat_thresholds();
    node_cpus = cpumask_of_node(node);
    if (!cpumask_empty(node_cpus)) {
    return 0;
    }
    node_clear_state(node, N_CPU);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vmstat_late_init() -> c_int {
    refresh_zone_stat_thresholds();
    vmstat_late_init_done = 1;
    return 0;
    }
    late_initcall!(vmstat_late_init);

pub static mut ctl_table: usize = 0;

pub static mut mm_percpu_wq: *mut c_void = core::ptr::null_mut();
#[no_mangle]
pub unsafe extern "C" fn init_mm_internals()  {
    int ret __maybe_unused;
    mm_percpu_wq = alloc_workqueue("mm_percpu_wq",
    WQ_MEM_RECLAIM | WQ_PERCPU, 0);

    ret = cpuhp_setup_state_nocalls(CPUHP_MM_VMSTAT_DEAD, "mm/vmstat:dead",
    core::ptr::null_mut(), vmstat_cpu_dead);
    if (ret < 0) {
    pr_err!("vmstat: failed to register 'dead' hotplug state\n");
    }
    ret = cpuhp_setup_state_nocalls(CPUHP_AP_ONLINE_DYN, "mm/vmstat:online",
    vmstat_cpu_online,
    vmstat_cpu_down_prep);
    if (ret < 0) {
    pr_err!("vmstat: failed to register 'online' hotplug state\n");
    }
    cpus_read_lock();
    init_cpu_node_state();
    cpus_read_unlock();
    start_shepherd_timer();

    proc_create_seq("buddyinfo", 0444, core::ptr::null_mut(), &fragmentation_op);
    proc_create_seq("pagetypeinfo", 0400, core::ptr::null_mut(), &pagetypeinfo_op);
    proc_create_seq("vmstat", 0444, core::ptr::null_mut(), &vmstat_op);
    proc_create_seq("zoneinfo", 0444, core::ptr::null_mut(), &zoneinfo_op);
    register_sysctl_init("vm", vmstat_table);

    }

//
// Return an index indicating how much of the available free memory is
// unusable for an allocation of the requested size.
//
#[no_mangle]
pub unsafe extern "C" fn unusable_free_index(order: c_uint, info: *mut contig_page_info) -> c_int {
// No free memory is interpreted as all free memory is unusable
    if (info.free_pages == 0) {
    return 1000;
    }
//
// Index should be a value between 0 and 1. Return a value to 3
// decimal places.
//
// 0 => no fragmentation
// 1 => high fragmentation
//
    return div_u64((info.free_pages - (info.free_blocks_suitable << order)) * 1000ULL, info.free_pages);
    }
#[no_mangle]
pub unsafe extern "C" fn unusable_show_print(m: *mut seq_file, pgdat: *mut pg_data_t, zone: *mut zone) {
    let mut order = 0;
    let mut index = 0;
pub static mut info: usize = 0;
    seq_printf(m, "Node %d, zone %8s ",
    pgdat.node_id,
    zone.name);
    while (order < NR_PAGE_ORDERS) {
    fill_contig_page_info(zone, order, &info);
    index = unusable_free_index(order, &info);
    seq_printf(m, "%d.%03d ", index / 1000, index % 1000);
    }
    seq_putc(m, '\n');
    }
//
// Display unusable free space index
//
// The unusable free space index measures how much of the available free
// memory cannot be used to satisfy an allocation of a given size and is a
// value between 0 and 1. The higher the value, the more of free memory is
// unusable and by implication, the worse the external fragmentation is. This
// can be expressed as a percentage by multiplying by 100.
//
#[no_mangle]
unsafe extern "C" fn unusable_show(m: *mut seq_file, arg: *mut c_void) -> c_int {
    let mut pgdat = arg;
// check memoryless node
    if (!node_state(pgdat.node_id, N_MEMORY)) {
    return 0;
    }
    walk_zones_in_node(m, pgdat, true, false, unusable_show_print);
    return 0;
    }
pub static mut seq_operations: usize = 0;
pub static mut unusable: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn extfrag_show_print(m: *mut seq_file, pgdat: *mut pg_data_t, zone: *mut zone) {
    let mut order = 0;
    let mut index = 0;
// Alloc on stack as interrupts are disabled for zone walk
pub static mut info: usize = 0;
    seq_printf(m, "Node %d, zone %8s ",
    pgdat.node_id,
    zone.name);
    while (order < NR_PAGE_ORDERS) {
    fill_contig_page_info(zone, order, &info);
    index = __fragmentation_index(order, &info);
    seq_printf(m, "%2d.%03d ", index / 1000, index % 1000);
    }
    seq_putc(m, '\n');
    }
//
// Display fragmentation index for orders that allocations would fail for
//
#[no_mangle]
unsafe extern "C" fn extfrag_show(m: *mut seq_file, arg: *mut c_void) -> c_int {
    let mut pgdat = arg;
    walk_zones_in_node(m, pgdat, true, false, extfrag_show_print);
    return 0;
    }
pub static mut seq_operations: usize = 0;
pub static mut extfrag: usize = 0;
#[no_mangle]
unsafe extern "C" fn extfrag_debug_init() -> c_int {
pub static mut extfrag_debug_root: *mut c_void = core::ptr::null_mut();
    extfrag_debug_root = debugfs_create_dir("extfrag", core::ptr::null_mut());
    debugfs_create_file("unusable_index", 0444, extfrag_debug_root, core::ptr::null_mut(),
    &unusable_fops);
    debugfs_create_file("extfrag_index", 0444, extfrag_debug_root, core::ptr::null_mut(),
    &extfrag_fops);
    return 0;
    }
    module_init!(extfrag_debug_init);