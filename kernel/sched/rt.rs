//! Automatically rewritten from C to Rust
//! Source: kernel/sched/rt.c
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
// Real-Time Scheduling Class (mapped to the SCHED_FIFO and SCHED_RR
// policies)
//

pub static mut sched_rr_timeslice: c_int = 0;
// More than 4 hours if BW_SHIFT equals 20.
pub static mut max_rt_runtime: u64 = 0;
//
// period over which we measure -rt task CPU usage in us.
// default: 1s
//
pub static mut sysctl_sched_rt_period: c_int = 1000000;
//
// part of the period that we allow rt tasks to run in us.
// default: 1s
//
pub static mut sysctl_sched_rt_runtime: c_int = 1000000;

pub static mut sysctl_sched_rr_timeslice: int = 0;
// forward_decl: sched_rt_handler;
// forward_decl: sched_rr_handler;
pub static mut ctl_table: usize = 0;
#[no_mangle]
unsafe extern "C" fn sched_rt_sysctl_init() -> c_int {
    register_sysctl_init("kernel", sched_rt_sysctls);
    return 0;
    }
    late_initcall!(sched_rt_sysctl_init);

#[no_mangle]
pub unsafe extern "C" fn init_rt_rq(rt_rq: *mut rt_rq) {
pub static mut array: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    array = &rt_rq.active;
    while (i < MAX_RT_PRIO) {
    INIT_LIST_HEAD(array.queue + i);
    __clear_bit(i, array.bitmap);
    }
// delimiter for bitsearch:
    __set_bit(MAX_RT_PRIO, array.bitmap);
    rt_rq.highest_prio.curr = MAX_RT_PRIO-1;
    rt_rq.highest_prio.next = MAX_RT_PRIO-1;
    rt_rq.overloaded = 0;
    plist_head_init(&rt_rq.pushable_tasks);
// We start is dequeued state, because no RT tasks are queued
    rt_rq.rt_queued = 0;

    rt_rq.rt_time = 0;
    rt_rq.rt_throttled = 0;
    rt_rq.rt_runtime = 0;
    raw_spin_lock_init(&rt_rq.rt_runtime_lock);
    rt_rq.tg = &root_task_group;

    }

// forward_decl: do_sched_rt_period_timer;
#[no_mangle]
unsafe extern "C" fn sched_rt_period_timer(timer: *mut hrtimer) -> enum hrtimer_restart {
    let mut rt_b = container_of!(timer, rt_bandwidth, rt_period_timer);
pub static mut idle: c_int = 0;
    let mut overrun = 0;
    raw_spin_lock(&rt_b.rt_runtime_lock);
    for (;;) {
    overrun = hrtimer_forward_now(timer, rt_b.rt_period);
    if (!overrun) {
    break;
    }
    raw_spin_unlock(&rt_b.rt_runtime_lock);
    idle = do_sched_rt_period_timer(rt_b, overrun);
    raw_spin_lock(&rt_b.rt_runtime_lock);
    }
    if (idle) {
    rt_b.rt_period_active = 0;
    }
    raw_spin_unlock(&rt_b.rt_runtime_lock);
    return idle ? HRTIMER_NORESTART : HRTIMER_RESTART;
    }
#[no_mangle]
pub unsafe extern "C" fn init_rt_bandwidth(rt_b: *mut rt_bandwidth, period: u64, runtime: u64) {
    rt_b.rt_period = ns_to_ktime(period);
    rt_b.rt_runtime = runtime;
    raw_spin_lock_init(&rt_b.rt_runtime_lock);
    hrtimer_setup(&rt_b.rt_period_timer, sched_rt_period_timer, CLOCK_MONOTONIC,
    HRTIMER_MODE_REL_HARD);
    }
#[no_mangle]
pub unsafe extern "C" fn do_start_rt_bandwidth(rt_b: *mut rt_bandwidth) {
    raw_spin_lock(&rt_b.rt_runtime_lock);
    if (!rt_b.rt_period_active) {
    rt_b.rt_period_active = 1;
//
// SCHED_DEADLINE updates the bandwidth, as a run away
// RT task with a DL task could hog a CPU. But DL does
// not reset the period. If a deadline task was running
// without an RT task running, it can cause RT tasks to
// throttle when they start up. Kick the timer right away
// to update the period.
//
    hrtimer_forward_now(&rt_b.rt_period_timer, ns_to_ktime(0));
    hrtimer_start_expires(&rt_b.rt_period_timer,
    HRTIMER_MODE_ABS_PINNED_HARD);
    }
    raw_spin_unlock(&rt_b.rt_runtime_lock);
    }
#[no_mangle]
unsafe extern "C" fn start_rt_bandwidth(rt_b: *mut rt_bandwidth) {
    if (!rt_bandwidth_enabled() || rt_b.rt_runtime == RUNTIME_INF) {
    return;
    }
    do_start_rt_bandwidth(rt_b);
    }
#[no_mangle]
unsafe extern "C" fn destroy_rt_bandwidth(rt_b: *mut rt_bandwidth) {
    hrtimer_cancel(&rt_b.rt_period_timer);
    }

#[no_mangle]
pub unsafe extern "C" fn rt_task_of(rt_se: *mut sched_rt_entity) -> *mut c_void {
    WARN_ON_ONCE!(!rt_entity_is_task(rt_se));
    return container_of!(rt_se, task_struct, rt);
    }
#[no_mangle]
pub unsafe extern "C" fn rq_of_rt_rq(rt_rq: *mut rt_rq) -> *mut c_void {
// Cannot fold with non-CONFIG_RT_GROUP_SCHED version, layout
    WARN_ON!(!rt_group_sched_enabled() && rt_rq.tg != &root_task_group);
    return rt_rq.rq;
    }
#[no_mangle]
pub unsafe extern "C" fn rt_rq_of_se(rt_se: *mut sched_rt_entity) -> *mut c_void {
    WARN_ON!(!rt_group_sched_enabled() && rt_se.rt_rq.tg != &root_task_group);
    return rt_se.rt_rq;
    }
#[no_mangle]
pub unsafe extern "C" fn rq_of_rt_se(rt_se: *mut sched_rt_entity) -> *mut c_void {
    let mut rt_rq = rt_se.rt_rq;
    WARN_ON!(!rt_group_sched_enabled() && rt_rq.tg != &root_task_group);
    return rt_rq.rq;
    }
#[no_mangle]
pub unsafe extern "C" fn unregister_rt_sched_group(tg: *mut task_group) {
    if (!rt_group_sched_enabled()) {
    return;
    }
    if (tg.rt_se) {
    destroy_rt_bandwidth(&tg.rt_bandwidth);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn free_rt_sched_group(tg: *mut task_group) {
    let mut i = 0;
    if (!rt_group_sched_enabled()) {
    return;
    }
    for_each_possible_cpu(i) {
    if (tg.rt_rq) {
    kfree(tg.rt_rq[i]);
    }
    if (tg.rt_se) {
    kfree(tg.rt_se[i]);
    }
    }
    kfree(tg.rt_rq);
    kfree(tg.rt_se);
    }
#[no_mangle]
pub unsafe extern "C" fn init_tg_rt_entry(tg: *mut task_group, rt_rq: *mut rt_rq, rt_se: *mut sched_rt_entity, cpu: c_int, parent: *mut sched_rt_entity) {
    let mut rq = cpu_rq(cpu);
    rt_rq.highest_prio.curr = MAX_RT_PRIO-1;
    rt_rq.rt_nr_boosted = 0;
    rt_rq.rq = rq;
    rt_rq.tg = tg;
    tg.rt_rq[cpu] = rt_rq;
    tg.rt_se[cpu] = rt_se;
    if (!rt_se) {
    return;
    }
    if (!parent) {
    rt_se.rt_rq = &rq.rt;
    }
    else {
    rt_se.rt_rq = parent.my_q;
    }
    rt_se.my_q = rt_rq;
    rt_se.parent = parent;
    INIT_LIST_HEAD(&rt_se.run_list);
    }
#[no_mangle]
pub unsafe extern "C" fn alloc_rt_sched_group(tg: *mut task_group, parent: *mut task_group) -> c_int {
pub static mut rt_rq: *mut c_void = core::ptr::null_mut();
pub static mut rt_se: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    if (!rt_group_sched_enabled()) {
    return 1;
    }
    tg.rt_rq = kzalloc_objs(rt_rq, nr_cpu_ids);
    if (!tg.rt_rq) {
// goto;
    }
    tg.rt_se = kzalloc_objs(rt_se, nr_cpu_ids);
    if (!tg.rt_se) {
// goto;
    }
    init_rt_bandwidth(&tg.rt_bandwidth, ktime_to_ns(global_rt_period()), 0);
    for_each_possible_cpu(i) {
    rt_rq = kzalloc_node(sizeof!(rt_rq),
    GFP_KERNEL, cpu_to_node(i));
    if (!rt_rq) {
// goto;
    }
    rt_se = kzalloc_node(sizeof!(sched_rt_entity),
    GFP_KERNEL, cpu_to_node(i));
    if (!rt_se) {
// goto;
    }
    init_rt_rq(rt_rq);
    rt_rq.rt_runtime = tg.rt_bandwidth.rt_runtime;
    init_tg_rt_entry(tg, rt_rq, rt_se, i, parent.rt_se[i]);
    }
    return 1;
// label;
    kfree(rt_rq);
// label;
    return 0;
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: rt_task_of
pub unsafe extern "C" fn rt_task_of_dup(rt_se: *mut sched_rt_entity) -> *mut c_void {
    return container_of!(rt_se, task_struct, rt);
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: rq_of_rt_rq
pub unsafe extern "C" fn rq_of_rt_rq_dup(rt_rq: *mut rt_rq) -> *mut c_void {
    return container_of!(rt_rq, rq, rt);
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: rq_of_rt_se
pub unsafe extern "C" fn rq_of_rt_se_dup(rt_se: *mut sched_rt_entity) -> *mut c_void {
    let mut p = rt_task_of(rt_se);
    return task_rq(p);
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: rt_rq_of_se
pub unsafe extern "C" fn rt_rq_of_se_dup(rt_se: *mut sched_rt_entity) -> *mut c_void {
    let mut rq = rq_of_rt_se(rt_se);
    return &rq.rt;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: unregister_rt_sched_group
pub unsafe extern "C" fn unregister_rt_sched_group_dup(tg: *mut task_group) { }
#[no_mangle]
#[no_mangle]
// duplicate fn: free_rt_sched_group
pub unsafe extern "C" fn free_rt_sched_group_dup(tg: *mut task_group) { }
#[no_mangle]
#[no_mangle]
// duplicate fn: alloc_rt_sched_group
pub unsafe extern "C" fn alloc_rt_sched_group_dup(tg: *mut task_group, parent: *mut task_group) -> c_int {
    return 1;
    }

#[no_mangle]
pub unsafe extern "C" fn need_pull_rt_task(rq: *mut rq, prev: *mut task_struct) -> bool {
// Try to pull RT tasks here if we lower this rq's prio
    return rq.online && rq.rt.highest_prio.curr > prev.prio;
    }
#[no_mangle]
pub unsafe extern "C" fn rt_overloaded(rq: *mut rq) -> c_int {
    return atomic_read(&rq.rd.rto_count);
    }
#[no_mangle]
pub unsafe extern "C" fn rt_set_overload(rq: *mut rq) {
    if (!rq.online) {
    return;
    }
    cpumask_set_cpu(rq.cpu, rq.rd.rto_mask);
//
// Make sure the mask is visible before we set
// the overload count. That is checked to determine
// if we should look at the mask. It would be a shame
// if we looked at the mask, but the mask was not
// updated yet.
//
// Matched by the barrier in pull_rt_task().
//
    smp_wmb();
    atomic_inc(&rq.rd.rto_count);
    }
#[no_mangle]
pub unsafe extern "C" fn rt_clear_overload(rq: *mut rq) {
    if (!rq.online) {
    return;
    }
// the order here really doesn't matter
    atomic_dec(&rq.rd.rto_count);
    cpumask_clear_cpu(rq.cpu, rq.rd.rto_mask);
    }
#[no_mangle]
pub unsafe extern "C" fn has_pushable_tasks(rq: *mut rq) -> c_int {
    return !plist_head_empty(&rq.rt.pushable_tasks);
    }
pub static mut struct balance_callback: usize = 0;
pub static mut struct balance_callback: usize = 0;
// forward_decl: push_rt_tasks;
// forward_decl: pull_rt_task;
#[no_mangle]
pub unsafe extern "C" fn rt_queue_push_tasks(rq: *mut rq) {
    if (!has_pushable_tasks(rq)) {
    return;
    }
    queue_balance_callback(rq, &per_cpu(rt_push_head, rq.cpu), push_rt_tasks);
    }
#[no_mangle]
pub unsafe extern "C" fn rt_queue_pull_task(rq: *mut rq) {
    queue_balance_callback(rq, &per_cpu(rt_pull_head, rq.cpu), pull_rt_task);
    }
#[no_mangle]
unsafe extern "C" fn enqueue_pushable_task(rq: *mut rq, p: *mut task_struct) {
    plist_del(&p.pushable_tasks, &rq.rt.pushable_tasks);
    plist_node_init(&p.pushable_tasks, p.prio);
    plist_add(&p.pushable_tasks, &rq.rt.pushable_tasks);
// Update the highest prio pushable task
    if (p.prio < rq.rt.highest_prio.next) {
    rq.rt.highest_prio.next = p.prio;
    }
    if (!rq.rt.overloaded) {
    rt_set_overload(rq);
    rq.rt.overloaded = 1;
    }
    }
#[no_mangle]
unsafe extern "C" fn dequeue_pushable_task(rq: *mut rq, p: *mut task_struct) {
    plist_del(&p.pushable_tasks, &rq.rt.pushable_tasks);
// Update the new highest prio pushable task
    if (has_pushable_tasks(rq)) {
    p = plist_first_entry(&rq.rt.pushable_tasks, task_struct, pushable_tasks);
    rq.rt.highest_prio.next = p.prio;
    } else {
    rq.rt.highest_prio.next = MAX_RT_PRIO-1;
    if (rq.rt.overloaded) {
    rt_clear_overload(rq);
    rq.rt.overloaded = 0;
    }
    }
    }
// forward_decl: enqueue_top_rt_rq;
// forward_decl: dequeue_top_rt_rq;
#[no_mangle]
pub unsafe extern "C" fn on_rt_rq(rt_se: *mut sched_rt_entity) -> c_int {
    return rt_se.on_rq;
    }

//
// Verify the fitness of task @p to run on @cpu taking into account the uclamp
// settings.
//
// This check is only important for heterogeneous systems where uclamp_min value
// is higher than the capacity of a @cpu. For non-heterogeneous system this
// function will always return true.
//
// The function will return true if the capacity of the @cpu is >= the
// uclamp_min and false otherwise.
//
// Note that uclamp_min will be clamped to uclamp_max if uclamp_min
// > uclamp_max.
//
#[no_mangle]
pub unsafe extern "C" fn rt_task_fits_capacity(p: *mut task_struct, cpu: c_int) -> bool {
    let mut min_cap = 0;
    let mut max_cap = 0;
    let mut cpu_cap = 0;
// Only heterogeneous systems can benefit from this check
    if (!sched_asym_cpucap_active()) {
    return true;
    }
    min_cap = uclamp_eff_value(p, UCLAMP_MIN);
    max_cap = uclamp_eff_value(p, UCLAMP_MAX);
    cpu_cap = arch_scale_cpu_capacity(cpu);
    return cpu_cap >= min(min_cap, max_cap);
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: rt_task_fits_capacity
pub unsafe extern "C" fn rt_task_fits_capacity_dup(p: *mut task_struct, cpu: c_int) -> bool {
    return true;
    }

#[no_mangle]
pub unsafe extern "C" fn sched_rt_runtime(rt_rq: *mut rt_rq) -> u64 {
    return rt_rq.rt_runtime;
    }
#[no_mangle]
pub unsafe extern "C" fn sched_rt_period(rt_rq: *mut rt_rq) -> u64 {
    return ktime_to_ns(rt_rq.tg.rt_bandwidth.rt_period);
    }
    typedef struct task_group *rt_rq_iter_t;
#[no_mangle]
pub unsafe extern "C" fn next_task_group(tg: *mut task_group) -> *mut c_void {
    if (!rt_group_sched_enabled()) {
    WARN_ON!(tg != &root_task_group);
    return core::ptr::null_mut();
    }
    do {
    tg = list_entry_rcu(tg.list.next,
    typeof(task_group), list);
    } while (&tg.list != &task_groups && task_group_is_autogroup(tg));
    if (&tg.list == &task_groups) {
    tg = core::ptr::null_mut();
    }
    return tg;
    }

    for (iter = &root_task_group;					
    iter && (rt_rq = iter.rt_rq[cpu_of(rq)]);		
    iter = next_task_group(iter)) {

    for (; rt_se; rt_se = rt_se.parent)
    }
#[no_mangle]
pub unsafe extern "C" fn group_rt_rq(rt_se: *mut sched_rt_entity) -> *mut c_void {
    return rt_se.my_q;
    }
// forward_decl: enqueue_rt_entity;
// forward_decl: dequeue_rt_entity;
#[no_mangle]
unsafe extern "C" fn sched_rt_rq_enqueue(rt_rq: *mut rt_rq) {
    let mut donor = rq_of_rt_rq(rt_rq).donor;
    let mut rq = rq_of_rt_rq(rt_rq);
pub static mut rt_se: *mut c_void = core::ptr::null_mut();
pub static mut cpu: c_int = 0;
    rt_se = rt_rq.tg.rt_se[cpu];
    if (rt_rq.rt_nr_running) {
    if (!rt_se) {
    enqueue_top_rt_rq(rt_rq);
    }

    else if (!on_rt_rq(rt_se)) {
    enqueue_rt_entity(rt_se, 0);
    }
    if (rt_rq.highest_prio.curr < donor.prio) {
    resched_curr(rq);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn sched_rt_rq_dequeue(rt_rq: *mut rt_rq) {
pub static mut rt_se: *mut c_void = core::ptr::null_mut();
pub static mut cpu: c_int = 0;
    rt_se = rt_rq.tg.rt_se[cpu];
    if (!rt_se) {
    dequeue_top_rt_rq(rt_rq, rt_rq.rt_nr_running);
// Kick cpufreq (see the comment in kernel/sched/sched.h).
    cpufreq_update_util(rq_of_rt_rq(rt_rq), 0);
    }

    else if (on_rt_rq(rt_se)) {
    dequeue_rt_entity(rt_se, 0);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn rt_rq_throttled(rt_rq: *mut rt_rq) -> c_int {
    return rt_rq.rt_throttled && !rt_rq.rt_nr_boosted;
    }
#[no_mangle]
unsafe extern "C" fn rt_se_boosted(rt_se: *mut sched_rt_entity) -> c_int {
    let mut rt_rq = group_rt_rq(rt_se);
pub static mut p: *mut c_void = core::ptr::null_mut();
    if (rt_rq) {
    return !!rt_rq.rt_nr_boosted;
    }
    p = rt_task_of(rt_se);
    return p.prio != p.normal_prio;
    }
    static inline const struct cpumask *sched_rt_period_mask(void)
    {
    return this_rq().rd.span;
    }
#[no_mangle]
pub unsafe extern "C" fn sched_rt_period_rt_rq(rt_b: *mut rt_bandwidth, cpu: c_int) -> *mut c_void {
    return container_of!(rt_b, task_group, rt_bandwidth).rt_rq[cpu];
    }
#[no_mangle]
pub unsafe extern "C" fn sched_rt_bandwidth(rt_rq: *mut rt_rq) -> *mut c_void {
    return &rt_rq.tg.rt_bandwidth;
    }
#[no_mangle]
pub unsafe extern "C" fn sched_rt_bandwidth_account(rt_rq: *mut rt_rq) -> bool {
    let mut rt_b = sched_rt_bandwidth(rt_rq);
    return (hrtimer_active(&rt_b.rt_period_timer) ||
    rt_rq.rt_time < rt_b.rt_runtime);
    }
//
// We ran out of runtime, see if we can borrow some from our neighbours.
//
#[no_mangle]
unsafe extern "C" fn do_balance_runtime(rt_rq: *mut rt_rq) {
    let mut rt_b = sched_rt_bandwidth(rt_rq);
    let mut rd = rq_of_rt_rq(rt_rq).rd;
    let mut i = 0;
    let mut weight = 0;
    let mut rt_period = 0;
    weight = cpumask_weight(rd.span);
    raw_spin_lock(&rt_b.rt_runtime_lock);
    rt_period = ktime_to_ns(rt_b.rt_period);
    for_each_cpu(i, rd.span) {
    let mut iter = sched_rt_period_rt_rq(rt_b, i);
    let mut diff = 0;
    if (iter == rt_rq) {
    continue;
    }
    raw_spin_lock(&iter.rt_runtime_lock);
//
// Either all rqs have inf runtime and there's nothing to steal
// or __disable_runtime() below sets a specific rq to inf to
// indicate its been disabled and disallow stealing.
//
    if (iter.rt_runtime == RUNTIME_INF) {
// goto;
    }
//
// From runqueues with spare time, take 1/n part of their
// spare time, but no more than our period.
//
    diff = iter.rt_runtime - iter.rt_time;
    if (diff > 0) {
    diff = div_u64((u64)diff, weight);
    if (rt_rq.rt_runtime + diff > rt_period) {
    diff = rt_period - rt_rq.rt_runtime;
    }
    iter.rt_runtime -= diff;
    rt_rq.rt_runtime += diff;
    if (rt_rq.rt_runtime == rt_period) {
    raw_spin_unlock(&iter.rt_runtime_lock);
    break;
    }
    }
// label;
    raw_spin_unlock(&iter.rt_runtime_lock);
    }
    raw_spin_unlock(&rt_b.rt_runtime_lock);
    }
//
// Ensure this RQ takes back all the runtime it lend to its neighbours.
//
#[no_mangle]
unsafe extern "C" fn __disable_runtime(rq: *mut rq) {
    let mut rd = rq.rd;
    let mut iter;
pub static mut rt_rq: *mut c_void = core::ptr::null_mut();
    if (unlikely(!scheduler_running)) {
    return;
    }
    for_each_rt_rq(rt_rq, iter, rq) {
    let mut rt_b = sched_rt_bandwidth(rt_rq);
    let mut want = 0;
    let mut i = 0;
    raw_spin_lock(&rt_b.rt_runtime_lock);
    raw_spin_lock(&rt_rq.rt_runtime_lock);
//
// Either we're all inf and nobody needs to borrow, or we're
// already disabled and thus have nothing to do, or we have
// exactly the right amount of runtime to take out.
//
    if (rt_rq.rt_runtime == RUNTIME_INF ||
    rt_rq.rt_runtime == rt_b.rt_runtime) {
// goto;
    }
    raw_spin_unlock(&rt_rq.rt_runtime_lock);
//
// Calculate the difference between what we started out with
// and what we current have, that's the amount of runtime
// we lend and now have to reclaim.
//
    want = rt_b.rt_runtime - rt_rq.rt_runtime;
//
// Greedy reclaim, take back as much as we can.
//
    for_each_cpu(i, rd.span) {
    let mut iter = sched_rt_period_rt_rq(rt_b, i);
    let mut diff = 0;
//
// Can't reclaim from ourselves or disabled runqueues.
//
    if (iter == rt_rq || iter.rt_runtime == RUNTIME_INF) {
    continue;
    }
    raw_spin_lock(&iter.rt_runtime_lock);
    if (want > 0) {
    diff = min_t(s64, iter.rt_runtime, want);
    iter.rt_runtime -= diff;
    want -= diff;
    } else {
    iter.rt_runtime -= want;
    want -= want;
    }
    raw_spin_unlock(&iter.rt_runtime_lock);
    if (!want) {
    break;
    }
    }
    raw_spin_lock(&rt_rq.rt_runtime_lock);
//
// We cannot be left wanting - that would mean some runtime
// leaked out of the system.
//
    WARN_ON_ONCE!(want);
// label;
//
// Disable all the borrow logic by pretending we have inf
// runtime - in which case borrowing doesn't make sense.
//
    rt_rq.rt_runtime = RUNTIME_INF;
    rt_rq.rt_throttled = 0;
    raw_spin_unlock(&rt_rq.rt_runtime_lock);
    raw_spin_unlock(&rt_b.rt_runtime_lock);
// Make rt_rq available for pick_next_task()
    sched_rt_rq_enqueue(rt_rq);
    }
    }
#[no_mangle]
unsafe extern "C" fn __enable_runtime(rq: *mut rq) {
    let mut iter;
pub static mut rt_rq: *mut c_void = core::ptr::null_mut();
    if (unlikely(!scheduler_running)) {
    return;
    }
//
// Reset each runqueue's bandwidth settings
//
    for_each_rt_rq(rt_rq, iter, rq) {
    let mut rt_b = sched_rt_bandwidth(rt_rq);
    raw_spin_lock(&rt_b.rt_runtime_lock);
    raw_spin_lock(&rt_rq.rt_runtime_lock);
    rt_rq.rt_runtime = rt_b.rt_runtime;
    rt_rq.rt_time = 0;
    rt_rq.rt_throttled = 0;
    raw_spin_unlock(&rt_rq.rt_runtime_lock);
    raw_spin_unlock(&rt_b.rt_runtime_lock);
    }
    }
#[no_mangle]
unsafe extern "C" fn balance_runtime(rt_rq: *mut rt_rq) {
    if (!sched_feat(RT_RUNTIME_SHARE)) {
    return;
    }
    if (rt_rq.rt_time > rt_rq.rt_runtime) {
    raw_spin_unlock(&rt_rq.rt_runtime_lock);
    do_balance_runtime(rt_rq);
    raw_spin_lock(&rt_rq.rt_runtime_lock);
    }
    }
#[no_mangle]
unsafe extern "C" fn do_sched_rt_period_timer(rt_b: *mut rt_bandwidth, overrun: c_int) -> c_int {
    int i, idle = 1, throttled = 0;
pub static mut span: *mut c_void = core::ptr::null_mut();
    span = sched_rt_period_mask();
//
// FIXME: isolated CPUs should really leave the root task group,
// whether they are isolcpus or were isolated via cpusets, lest
// the timer run on a CPU which does not service all runqueues,
// potentially leaving other CPUs indefinitely throttled.  If
// isolation is really required, the user will turn the throttle
// off to kill the perturbations it causes anyway.  Meanwhile,
// this maintains functionality for boot and/or troubleshooting.
//
    if (rt_b == &root_task_group.rt_bandwidth) {
    span = cpu_online_mask;
    }
    for_each_cpu(i, span) {
pub static mut enqueue: c_int = 0;
    let mut rt_rq = sched_rt_period_rt_rq(rt_b, i);
    let mut rq = rq_of_rt_rq(rt_rq);
pub static mut rf: usize = 0;
    let mut skip = 0;
//
// When span == cpu_online_mask, taking each rq->lock
// can be time-consuming. Try to avoid it when possible.
//
    raw_spin_lock(&rt_rq.rt_runtime_lock);
    if (!sched_feat(RT_RUNTIME_SHARE) && rt_rq.rt_runtime != RUNTIME_INF) {
    rt_rq.rt_runtime = rt_b.rt_runtime;
    }
    skip = !rt_rq.rt_time && !rt_rq.rt_nr_running;
    raw_spin_unlock(&rt_rq.rt_runtime_lock);
    if (skip) {
    continue;
    }
    rq_lock(rq, &rf);
    update_rq_clock(rq);
    if (rt_rq.rt_time) {
    let mut runtime = 0;
    raw_spin_lock(&rt_rq.rt_runtime_lock);
    if (rt_rq.rt_throttled) {
    balance_runtime(rt_rq);
    }
    runtime = rt_rq.rt_runtime;
    rt_rq.rt_time -= min(rt_rq.rt_time, overrun*runtime);
    if (rt_rq.rt_throttled && rt_rq.rt_time < runtime) {
    rt_rq.rt_throttled = 0;
    enqueue = 1;
//
// When we're idle and a woken (rt) task is
// throttled wakeup_preempt() will set
// skip_update and the time between the wakeup
// and this unthrottle will get accounted as
// 'runtime'.
//
    if (rt_rq.rt_nr_running && rq.curr == rq.idle) {
    rq_clock_cancel_skipupdate(rq);
    }
    }
    if (rt_rq.rt_time || rt_rq.rt_nr_running) {
    idle = 0;
    }
    raw_spin_unlock(&rt_rq.rt_runtime_lock);
    } else if (rt_rq.rt_nr_running) {
    idle = 0;
    if (!rt_rq_throttled(rt_rq)) {
    enqueue = 1;
    }
    }
    if (rt_rq.rt_throttled) {
    throttled = 1;
    }
    if (enqueue) {
    sched_rt_rq_enqueue(rt_rq);
    }
    rq_unlock(rq, &rf);
    }
    if (!throttled && (!rt_bandwidth_enabled() || rt_b.rt_runtime == RUNTIME_INF)) {
    return 1;
    }
    return idle;
    }
#[no_mangle]
unsafe extern "C" fn sched_rt_runtime_exceeded(rt_rq: *mut rt_rq) -> c_int {
pub static mut runtime: u64 = 0;
    if (rt_rq.rt_throttled) {
    return rt_rq_throttled(rt_rq);
    }
    if (runtime >= sched_rt_period(rt_rq)) {
    return 0;
    }
    balance_runtime(rt_rq);
    runtime = sched_rt_runtime(rt_rq);
    if (runtime == RUNTIME_INF) {
    return 0;
    }
    if (rt_rq.rt_time > runtime) {
    let mut rt_b = sched_rt_bandwidth(rt_rq);
//
// Don't actually throttle groups that have no runtime assigned
// but accrue some time due to boosting.
//
    if (likely(rt_b.rt_runtime)) {
    rt_rq.rt_throttled = 1;
    printk_deferred_once("sched: RT throttling activated\n");
    } else {
//
// In case we did anyway, make it go away,
// replenishment is a joke, since it will replenish us
// with exactly 0 ns.
//
    rt_rq.rt_time = 0;
    }
    if (rt_rq_throttled(rt_rq)) {
    sched_rt_rq_dequeue(rt_rq);
    return 1;
    }
    }
    return 0;
    }

    typedef struct rt_rq *rt_rq_iter_t;

    for ((void) iter, rt_rq = &rq.rt; rt_rq; rt_rq = core::ptr::null_mut()) {

    for (; rt_se; rt_se = core::ptr::null_mut())
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: group_rt_rq
pub unsafe extern "C" fn group_rt_rq_dup(rt_se: *mut sched_rt_entity) -> *mut c_void {
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn sched_rt_rq_enqueue(rt_rq: *mut rt_rq) {
    let mut rq = rq_of_rt_rq(rt_rq);
    if (!rt_rq.rt_nr_running) {
    return;
    }
    enqueue_top_rt_rq(rt_rq);
    resched_curr(rq);
    }
#[no_mangle]
pub unsafe extern "C" fn sched_rt_rq_dequeue(rt_rq: *mut rt_rq) {
    dequeue_top_rt_rq(rt_rq, rt_rq.rt_nr_running);
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: rt_rq_throttled
pub unsafe extern "C" fn rt_rq_throttled_dup(rt_rq: *mut rt_rq) -> c_int {
    return false;
    }
    static inline const struct cpumask *sched_rt_period_mask(void)
    {
    return cpu_online_mask;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: sched_rt_period_rt_rq
pub unsafe extern "C" fn sched_rt_period_rt_rq_dup(rt_b: *mut rt_bandwidth, cpu: c_int) -> *mut c_void {
    return &cpu_rq(cpu).rt;
    }
#[no_mangle]
pub unsafe extern "C" fn __enable_runtime(rq: *mut rq) { }
#[no_mangle]
pub unsafe extern "C" fn __disable_runtime(rq: *mut rq) { }

#[no_mangle]
pub unsafe extern "C" fn rt_se_prio(rt_se: *mut sched_rt_entity) -> c_int {

    let mut rt_rq = group_rt_rq(rt_se);
    if (rt_rq) {
    return rt_rq.highest_prio.curr;
    }

    return rt_task_of(rt_se).prio;
    }
//
// Update the current task's runtime statistics. Skip current tasks that
// are not in our scheduling class.
//
#[no_mangle]
unsafe extern "C" fn update_curr_rt(rq: *mut rq) {
    let mut donor = rq.donor;
    let mut delta_exec = 0;
    if (donor.sched_class != &rt_sched_class) {
    return;
    }
    delta_exec = update_curr_common(rq);
    if (unlikely(delta_exec <= 0)) {
    return;
    }

    let mut rt_se = &donor.rt;
    if (!rt_bandwidth_enabled()) {
    return;
    }
    for_each_sched_rt_entity(rt_se) {
    let mut rt_rq = rt_rq_of_se(rt_se);
    let mut exceeded = 0;
    if (sched_rt_runtime(rt_rq) != RUNTIME_INF) {
    raw_spin_lock(&rt_rq.rt_runtime_lock);
    rt_rq.rt_time += delta_exec;
    exceeded = sched_rt_runtime_exceeded(rt_rq);
    if (exceeded) {
    resched_curr(rq);
    }
    raw_spin_unlock(&rt_rq.rt_runtime_lock);
    if (exceeded) {
    do_start_rt_bandwidth(sched_rt_bandwidth(rt_rq));
    }
    }
    }

    }
#[no_mangle]
pub unsafe extern "C" fn dequeue_top_rt_rq(rt_rq: *mut rt_rq, count: c_uint) {
    let mut rq = rq_of_rt_rq(rt_rq);
    BUG_ON!(&rq.rt != rt_rq);
    if (!rt_rq.rt_queued) {
    return;
    }
    BUG_ON!(!rq.nr_running);
    sub_nr_running(rq, count);
    rt_rq.rt_queued = 0;
    }
#[no_mangle]
pub unsafe extern "C" fn enqueue_top_rt_rq(rt_rq: *mut rt_rq) {
    let mut rq = rq_of_rt_rq(rt_rq);
    BUG_ON!(&rq.rt != rt_rq);
    if (rt_rq.rt_queued) {
    return;
    }
    if (rt_rq_throttled(rt_rq)) {
    return;
    }
    if (rt_rq.rt_nr_running) {
    add_nr_running(rq, rt_rq.rt_nr_running);
    rt_rq.rt_queued = 1;
    }
// Kick cpufreq (see the comment in kernel/sched/sched.h).
    cpufreq_update_util(rq, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn inc_rt_prio_smp(rt_rq: *mut rt_rq, prio: c_int, prev_prio: c_int) {
    let mut rq = rq_of_rt_rq(rt_rq);
//
// Change rq's cpupri only if rt_rq is the top queue.
//
    if (IS_ENABLED!(CONFIG_RT_GROUP_SCHED) && &rq.rt != rt_rq) {
    return;
    }
    if (rq.online && prio < prev_prio) {
    cpupri_set(&rq.rd.cpupri, rq.cpu, prio);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn dec_rt_prio_smp(rt_rq: *mut rt_rq, prio: c_int, prev_prio: c_int) {
    let mut rq = rq_of_rt_rq(rt_rq);
//
// Change rq's cpupri only if rt_rq is the top queue.
//
    if (IS_ENABLED!(CONFIG_RT_GROUP_SCHED) && &rq.rt != rt_rq) {
    return;
    }
    if (rq.online && rt_rq.highest_prio.curr != prev_prio) {
    cpupri_set(&rq.rd.cpupri, rq.cpu, rt_rq.highest_prio.curr);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn inc_rt_prio(rt_rq: *mut rt_rq, prio: c_int) {
pub static mut prev_prio: c_int = 0;
    if (prio < prev_prio) {
    rt_rq.highest_prio.curr = prio;
    }
    inc_rt_prio_smp(rt_rq, prio, prev_prio);
    }
#[no_mangle]
pub unsafe extern "C" fn dec_rt_prio(rt_rq: *mut rt_rq, prio: c_int) {
pub static mut prev_prio: c_int = 0;
    if (rt_rq.rt_nr_running) {
    WARN_ON!(prio < prev_prio);
//
// This may have been our highest task, and therefore
// we may have some re-computation to do
//
    if (prio == prev_prio) {
    let mut array = &rt_rq.active;
    rt_rq.highest_prio.curr =
    sched_find_first_bit(array.bitmap);
    }
    } else {
    rt_rq.highest_prio.curr = MAX_RT_PRIO-1;
    }
    dec_rt_prio_smp(rt_rq, prio, prev_prio);
    }

#[no_mangle]
pub unsafe extern "C" fn inc_rt_group(rt_se: *mut sched_rt_entity, rt_rq: *mut rt_rq) {
    if (rt_se_boosted(rt_se)) {
    rt_rq.rt_nr_boosted += 1;
    }
    start_rt_bandwidth(&rt_rq.tg.rt_bandwidth);
    }
#[no_mangle]
pub unsafe extern "C" fn dec_rt_group(rt_se: *mut sched_rt_entity, rt_rq: *mut rt_rq) {
    if (rt_se_boosted(rt_se)) {
    rt_rq.rt_nr_boosted -= 1;
    }
    WARN_ON!(!rt_rq.rt_nr_running && rt_rq.rt_nr_boosted);
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: inc_rt_group
pub unsafe extern "C" fn inc_rt_group_dup(rt_se: *mut sched_rt_entity, rt_rq: *mut rt_rq) {
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: dec_rt_group
pub unsafe extern "C" fn dec_rt_group_dup(rt_se: *mut sched_rt_entity, rt_rq: *mut rt_rq) {}

    static inline
#[no_mangle]
pub unsafe extern "C" fn rt_se_nr_running(rt_se: *mut sched_rt_entity) -> c_uint {
    let mut group_rq = group_rt_rq(rt_se);
    if (group_rq) {
    return group_rq.rt_nr_running;
    }
    else {
    return 1;
    }
    }
    static inline
#[no_mangle]
pub unsafe extern "C" fn rt_se_rr_nr_running(rt_se: *mut sched_rt_entity) -> c_uint {
    let mut group_rq = group_rt_rq(rt_se);
pub static mut tsk: *mut c_void = core::ptr::null_mut();
    if (group_rq) {
    return group_rq.rr_nr_running;
    }
    tsk = rt_task_of(rt_se);
    return (tsk.policy == SCHED_RR) ? 1 : 0;
    }
    static inline
#[no_mangle]
pub unsafe extern "C" fn inc_rt_tasks(rt_se: *mut sched_rt_entity, rt_rq: *mut rt_rq) {
pub static mut prio: c_int = 0;
    WARN_ON!(!rt_prio(prio));
    rt_rq.rt_nr_running += rt_se_nr_running(rt_se);
    rt_rq.rr_nr_running += rt_se_rr_nr_running(rt_se);
    inc_rt_prio(rt_rq, prio);
    inc_rt_group(rt_se, rt_rq);
    }
    static inline
#[no_mangle]
pub unsafe extern "C" fn dec_rt_tasks(rt_se: *mut sched_rt_entity, rt_rq: *mut rt_rq) {
    WARN_ON!(!rt_prio(rt_se_prio(rt_se)));
    WARN_ON!(!rt_rq.rt_nr_running);
    rt_rq.rt_nr_running -= rt_se_nr_running(rt_se);
    rt_rq.rr_nr_running -= rt_se_rr_nr_running(rt_se);
    dec_rt_prio(rt_rq, rt_se_prio(rt_se));
    dec_rt_group(rt_se, rt_rq);
    }
//
// Change rt_se->run_list location unless SAVE && !MOVE
//
// assumes ENQUEUE/DEQUEUE flags match
//
#[no_mangle]
pub unsafe extern "C" fn move_entity(flags: c_uint) -> bool {
    if ((flags & (DEQUEUE_SAVE | DEQUEUE_MOVE)) == DEQUEUE_SAVE) {
    return false;
    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn __delist_rt_entity(rt_se: *mut sched_rt_entity, array: *mut rt_prio_array) {
    list_del_init(&rt_se.run_list);
    if (list_empty(array.queue + rt_se_prio(rt_se))) {
    __clear_bit(rt_se_prio(rt_se), array.bitmap);
    }
    rt_se.on_list = 0;
    }
#[no_mangle]
pub unsafe extern "C" fn __schedstats_from_rt_se(rt_se: *mut sched_rt_entity) -> *mut c_void {
// schedstats is not supported for rt group.
    if (!rt_entity_is_task(rt_se)) {
    return core::ptr::null_mut();
    }
    return &rt_task_of(rt_se).stats;
    }
#[no_mangle]
pub unsafe extern "C" fn update_stats_wait_start_rt(rt_rq: *mut rt_rq, rt_se: *mut sched_rt_entity) {
pub static mut stats: *mut c_void = core::ptr::null_mut();
    let mut p = core::ptr::null_mut();
    if (!schedstat_enabled()) {
    return;
    }
    if (rt_entity_is_task(rt_se)) {
    p = rt_task_of(rt_se);
    }
    stats = __schedstats_from_rt_se(rt_se);
    if (!stats) {
    return;
    }
    __update_stats_wait_start(rq_of_rt_rq(rt_rq), p, stats);
    }
#[no_mangle]
pub unsafe extern "C" fn update_stats_enqueue_sleeper_rt(rt_rq: *mut rt_rq, rt_se: *mut sched_rt_entity) {
pub static mut stats: *mut c_void = core::ptr::null_mut();
    let mut p = core::ptr::null_mut();
    if (!schedstat_enabled()) {
    return;
    }
    if (rt_entity_is_task(rt_se)) {
    p = rt_task_of(rt_se);
    }
    stats = __schedstats_from_rt_se(rt_se);
    if (!stats) {
    return;
    }
    __update_stats_enqueue_sleeper(rq_of_rt_rq(rt_rq), p, stats);
    }
#[no_mangle]
pub unsafe extern "C" fn update_stats_enqueue_rt(rt_rq: *mut rt_rq, rt_se: *mut sched_rt_entity, flags: c_int) {
    if (!schedstat_enabled()) {
    return;
    }
    if (flags & ENQUEUE_WAKEUP) {
    update_stats_enqueue_sleeper_rt(rt_rq, rt_se);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn update_stats_wait_end_rt(rt_rq: *mut rt_rq, rt_se: *mut sched_rt_entity) {
pub static mut stats: *mut c_void = core::ptr::null_mut();
    let mut p = core::ptr::null_mut();
    if (!schedstat_enabled()) {
    return;
    }
    if (rt_entity_is_task(rt_se)) {
    p = rt_task_of(rt_se);
    }
    stats = __schedstats_from_rt_se(rt_se);
    if (!stats) {
    return;
    }
    __update_stats_wait_end(rq_of_rt_rq(rt_rq), p, stats);
    }
#[no_mangle]
pub unsafe extern "C" fn update_stats_dequeue_rt(rt_rq: *mut rt_rq, rt_se: *mut sched_rt_entity, flags: c_int) {
    let mut p = core::ptr::null_mut();
    let mut rq = rq_of_rt_rq(rt_rq);
    if (!schedstat_enabled()) {
    return;
    }
    if (rt_entity_is_task(rt_se)) {
    p = rt_task_of(rt_se);
    if (p != rq.curr) {
    update_stats_wait_end_rt(rt_rq, rt_se);
    }
    }
    if ((flags & DEQUEUE_SLEEP) && p) {
    let mut state = 0;
    state = READ_ONCE(p.__state);
    if (state & TASK_INTERRUPTIBLE) {
    __schedstat_set(p.stats.sleep_start,
    rq_clock(rq_of_rt_rq(rt_rq)));
    }
    if (state & TASK_UNINTERRUPTIBLE) {
    __schedstat_set(p.stats.block_start,
    rq_clock(rq_of_rt_rq(rt_rq)));
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn __enqueue_rt_entity(rt_se: *mut sched_rt_entity, flags: c_uint) {
    let mut rt_rq = rt_rq_of_se(rt_se);
    let mut array = &rt_rq.active;
    let mut group_rq = group_rt_rq(rt_se);
    let mut queue = array.queue + rt_se_prio(rt_se);
//
// Don't enqueue the group if its throttled, or when empty.
// The latter is a consequence of the former when a child group
// get throttled and the current group doesn't have any other
// active members.
//
    if (group_rq && (rt_rq_throttled(group_rq) || !group_rq.rt_nr_running)) {
    if (rt_se.on_list) {
    __delist_rt_entity(rt_se, array);
    }
    return;
    }
    if (move_entity(flags)) {
    WARN_ON_ONCE!(rt_se.on_list);
    if (flags & ENQUEUE_HEAD) {
    list_add(&rt_se.run_list, queue);
    }
    else {
    list_add_tail(&rt_se.run_list, queue);
    }
    __set_bit(rt_se_prio(rt_se), array.bitmap);
    rt_se.on_list = 1;
    }
    rt_se.on_rq = 1;
    inc_rt_tasks(rt_se, rt_rq);
    }
#[no_mangle]
unsafe extern "C" fn __dequeue_rt_entity(rt_se: *mut sched_rt_entity, flags: c_uint) {
    let mut rt_rq = rt_rq_of_se(rt_se);
    let mut array = &rt_rq.active;
    if (move_entity(flags)) {
    WARN_ON_ONCE!(!rt_se.on_list);
    __delist_rt_entity(rt_se, array);
    }
    rt_se.on_rq = 0;
    dec_rt_tasks(rt_se, rt_rq);
    }
//
// Because the prio of an upper entry depends on the lower
// entries, we must remove entries top - down.
//
#[no_mangle]
unsafe extern "C" fn dequeue_rt_stack(rt_se: *mut sched_rt_entity, flags: c_uint) {
    let mut back = core::ptr::null_mut();
    let mut rt_nr_running = 0;
    for_each_sched_rt_entity(rt_se) {
    rt_se.back = back;
    back = rt_se;
    }
    rt_nr_running = rt_rq_of_se(back).rt_nr_running;
    while (rt_se) {
    if (on_rt_rq(rt_se)) {
    __dequeue_rt_entity(rt_se, flags);
    }
    }
    dequeue_top_rt_rq(rt_rq_of_se(back), rt_nr_running);
    }
#[no_mangle]
unsafe extern "C" fn enqueue_rt_entity(rt_se: *mut sched_rt_entity, flags: c_uint) {
    let mut rq = rq_of_rt_se(rt_se);
    update_stats_enqueue_rt(rt_rq_of_se(rt_se), rt_se, flags);
    dequeue_rt_stack(rt_se, flags);
    for_each_sched_rt_entity(rt_se) {
    __enqueue_rt_entity(rt_se, flags);
    }
    enqueue_top_rt_rq(&rq.rt);
    }
#[no_mangle]
unsafe extern "C" fn dequeue_rt_entity(rt_se: *mut sched_rt_entity, flags: c_uint) {
    let mut rq = rq_of_rt_se(rt_se);
    update_stats_dequeue_rt(rt_rq_of_se(rt_se), rt_se, flags);
    dequeue_rt_stack(rt_se, flags);
    for_each_sched_rt_entity(rt_se) {
    let mut rt_rq = group_rt_rq(rt_se);
    if (rt_rq && rt_rq.rt_nr_running) {
    __enqueue_rt_entity(rt_se, flags);
    }
    }
    enqueue_top_rt_rq(&rq.rt);
    }
//
// Adding/removing a task to/from a priority array:
//
#[no_mangle]
pub unsafe extern "C" fn enqueue_task_rt(rq: *mut rq, p: *mut task_struct, flags: c_int) {
    let mut rt_se = &p.rt;
    if (flags & ENQUEUE_WAKEUP) {
    rt_se.timeout = 0;
    }
    check_schedstat_required();
    update_stats_wait_start_rt(rt_rq_of_se(rt_se), rt_se);
    enqueue_rt_entity(rt_se, flags);
    if (task_is_blocked(p)) {
    return;
    }
    if (!task_current(rq, p) && p.nr_cpus_allowed > 1) {
    enqueue_pushable_task(rq, p);
    }
    }
#[no_mangle]
unsafe extern "C" fn dequeue_task_rt(rq: *mut rq, p: *mut task_struct, flags: c_int) -> bool {
    let mut rt_se = &p.rt;
    update_curr_rt(rq);
    dequeue_rt_entity(rt_se, flags);
    dequeue_pushable_task(rq, p);
    return true;
    }
//
// Put task to the head or the end of the run list without the overhead of
// dequeue followed by enqueue.
//
#[no_mangle]
pub unsafe extern "C" fn requeue_rt_entity(rt_rq: *mut rt_rq, rt_se: *mut sched_rt_entity, head: c_int) {
    if (on_rt_rq(rt_se)) {
    let mut array = &rt_rq.active;
    let mut queue = array.queue + rt_se_prio(rt_se);
    if (head) {
    list_move(&rt_se.run_list, queue);
    }
    else {
    list_move_tail(&rt_se.run_list, queue);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn requeue_task_rt(rq: *mut rq, p: *mut task_struct, head: c_int) {
    let mut rt_se = &p.rt;
pub static mut rt_rq: *mut c_void = core::ptr::null_mut();
    for_each_sched_rt_entity(rt_se) {
    rt_rq = rt_rq_of_se(rt_se);
    requeue_rt_entity(rt_rq, rt_se, head);
    }
    }
#[no_mangle]
unsafe extern "C" fn yield_task_rt(rq: *mut rq) {
    requeue_task_rt(rq, rq.donor, 0);
    }
// forward_decl: find_lowest_rq;
#[no_mangle]
pub unsafe extern "C" fn select_task_rq_rt(p: *mut task_struct, cpu: c_int, flags: c_int) -> c_int {
    let mut curr = core::ptr::null_mut();
    let mut donor = core::ptr::null_mut();
pub static mut rq: *mut c_void = core::ptr::null_mut();
    let mut test = 0;
// For anything but wake ups, just return the task_cpu
    if (!(flags & (WF_TTWU | WF_FORK))) {
// goto;
    }
    rq = cpu_rq(cpu);
    rcu_read_lock();
    curr = READ_ONCE(rq.curr); /* unlocked access */
    donor = READ_ONCE(rq.donor);
//
// If the current task on @p's runqueue is an RT task, then
// try to see if we can wake this RT task up on another
// runqueue. Otherwise simply start this RT task
// on its current runqueue.
//
// We want to avoid overloading runqueues. If the woken
// task is a higher priority, then it will stay on this CPU
// and the lower prio task should be moved to another CPU.
// Even though this will probably make the lower prio task
// lose its cache, we do not want to bounce a higher task
// around just because it gave up its CPU, perhaps for a
// lock?
//
// For equal prio tasks, we just let the scheduler sort it out.
//
// Otherwise, just let it ride on the affine RQ and the
// post-schedule router will push the preempted task away
//
// This test is optimistic, if we get it wrong the load-balancer
// will have to sort it out.
//
// We take into account the capacity of the CPU to ensure it fits the
// requirement of the task - which is only important on heterogeneous
// systems like big.LITTLE.
//
    test = curr &&
    unlikely(rt_task(donor)) &&
    (curr.nr_cpus_allowed < 2 || donor.prio <= p.prio);
    if (test || !rt_task_fits_capacity(p, cpu)) {
pub static mut target: c_int = 0;
//
// Bail out if we were forcing a migration to find a better
// fitting CPU but our search failed.
//
    if (!test && target != -1 && !rt_task_fits_capacity(p, target)) {
// goto;
    }
//
// Don't bother moving it if the destination CPU is
// not running a lower priority task.
//
    if (target != -1 &&
    p.prio < cpu_rq(target).rt.highest_prio.curr) {
    cpu = target;
    }
    }
// label;
    rcu_read_unlock();
// label;
    return cpu;
    }
#[no_mangle]
unsafe extern "C" fn check_preempt_equal_prio(rq: *mut rq, p: *mut task_struct) {
    if (rq.curr.nr_cpus_allowed == 1 ||
    !cpupri_find(&rq.rd.cpupri, rq.donor, core::ptr::null_mut())) {
    return;
    }
//
// p is migratable, so let's not schedule it and
// see if it is pushed or pulled somewhere else.
//
    if (p.nr_cpus_allowed != 1 &&
    cpupri_find(&rq.rd.cpupri, p, core::ptr::null_mut())) {
    return;
    }
//
// There appear to be other CPUs that can accept
// the current task but none can run 'p', so lets reschedule
// to try and push the current task away:
//
    requeue_task_rt(rq, p, 1);
    resched_curr(rq);
    }
#[no_mangle]
unsafe extern "C" fn balance_rt(rq: *mut rq, rf: *mut rq_flags) -> c_int {
//
// Note, rq->donor may change during rq lock drops,
// so don't re-use p across lock drops
//
    let mut p = rq.donor;
    if (!on_rt_rq(&p.rt) && need_pull_rt_task(rq, p)) {
//
// This is OK, because current is on_cpu, which avoids it being
// picked for load-balance and preemption/IRQs are still
// disabled avoiding further scheduler activity on it and we've
// not yet started the picking loop.
//
    rq_unpin_lock(rq, rf);
    pull_rt_task(rq);
    rq_repin_lock(rq, rf);
    }
    return sched_stop_runnable(rq) || sched_dl_runnable(rq) || sched_rt_runnable(rq);
    }
//
// Preempt the current task with a newly woken task if needed:
//
#[no_mangle]
unsafe extern "C" fn wakeup_preempt_rt(rq: *mut rq, p: *mut task_struct, flags: c_int) {
    let mut donor = rq.donor;
//
// XXX If we're preempted by DL, queue a push?
//
    if (p.sched_class != &rt_sched_class ||
    donor.sched_class != &rt_sched_class) {
    return;
    }
    if (p.prio < donor.prio) {
    resched_curr(rq);
    return;
    }
//
// If:
//
// - the newly woken task is of equal priority to the current task
// - the newly woken task is non-migratable while current is migratable
// - current will be preempted on the next reschedule
//
// we should check to see if current can readily move to a different
// cpu.  If so, we will reschedule to allow the push logic to try
// to move current somewhere else, making room for our non-migratable
// task.
//
    if (p.prio == donor.prio && !test_tsk_need_resched(rq.curr)) {
    check_preempt_equal_prio(rq, p);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn set_next_task_rt(rq: *mut rq, p: *mut task_struct, first: bool) {
    let mut rt_se = &p.rt;
    let mut rt_rq = &rq.rt;
    p.se.exec_start = rq_clock_task(rq);
    if (on_rt_rq(&p.rt)) {
    update_stats_wait_end_rt(rt_rq, rt_se);
    }
// The running task is never eligible for pushing
    dequeue_pushable_task(rq, p);
    if (!first) {
    return;
    }
//
// If prev task was rt, put_prev_task() has already updated the
// utilization. We only care of the case where we start to schedule a
// rt task
//
    if (rq.donor.sched_class != &rt_sched_class) {
    update_rt_rq_load_avg(rq_clock_pelt(rq), rq, 0);
    }
    rt_queue_push_tasks(rq);
    }
#[no_mangle]
pub unsafe extern "C" fn pick_next_rt_entity(rt_rq: *mut rt_rq) -> *mut c_void {
    let mut array = &rt_rq.active;
    let mut next = core::ptr::null_mut();
pub static mut queue: *mut c_void = core::ptr::null_mut();
    let mut idx = 0;
    idx = sched_find_first_bit(array.bitmap);
    BUG_ON!(idx >= MAX_RT_PRIO);
    queue = array.queue + idx;
    if (WARN_ON_ONCE!(list_empty(queue))) {
    return core::ptr::null_mut();
    }
    next = list_entry(queue.next, sched_rt_entity, run_list);
    return next;
    }
#[no_mangle]
pub unsafe extern "C" fn _pick_next_task_rt(rq: *mut rq) -> *mut c_void {
pub static mut rt_se: *mut c_void = core::ptr::null_mut();
    let mut rt_rq = &rq.rt;
    do {
    rt_se = pick_next_rt_entity(rt_rq);
    if (unlikely(!rt_se)) {
    return core::ptr::null_mut();
    }
    rt_rq = group_rt_rq(rt_se);
    } while (rt_rq);
    return rt_task_of(rt_se);
    }
#[no_mangle]
pub unsafe extern "C" fn pick_task_rt(rq: *mut rq, rf: *mut rq_flags) -> *mut c_void {
pub static mut p: *mut c_void = core::ptr::null_mut();
    if (!sched_rt_runnable(rq)) {
    return core::ptr::null_mut();
    }
    p = _pick_next_task_rt(rq);
    return p;
    }
#[no_mangle]
unsafe extern "C" fn put_prev_task_rt(rq: *mut rq, p: *mut task_struct, next: *mut task_struct) {
    let mut rt_se = &p.rt;
    let mut rt_rq = &rq.rt;
    if (on_rt_rq(&p.rt)) {
    update_stats_wait_start_rt(rt_rq, rt_se);
    }
    update_curr_rt(rq);
    update_rt_rq_load_avg(rq_clock_pelt(rq), rq, 1);
    if (task_is_blocked(p)) {
    return;
    }
//
// The previous task needs to be made eligible for pushing
// if it is still active
//
    if (on_rt_rq(&p.rt) && p.nr_cpus_allowed > 1) {
    enqueue_pushable_task(rq, p);
    }
    }
// Only try algorithms three times
pub const RT_MAX_TRIES: c_int = 3;
//
// Return the highest pushable rq's task, which is suitable to be executed
// on the CPU, NULL otherwise
//
#[no_mangle]
pub unsafe extern "C" fn pick_highest_pushable_task(rq: *mut rq, cpu: c_int) -> *mut c_void {
    let mut head = &rq.rt.pushable_tasks;
pub static mut p: *mut c_void = core::ptr::null_mut();
    if (!has_pushable_tasks(rq)) {
    return core::ptr::null_mut();
    }
    plist_for_each_entry(p, head, pushable_tasks) {
    if (task_is_pushable(rq, p, cpu)) {
    return p;
    }
    }
    return core::ptr::null_mut();
    }
pub static mut cpumask_var_t: usize = 0;
#[no_mangle]
unsafe extern "C" fn find_lowest_rq(task: *mut task_struct) -> c_int {
pub static mut sd: *mut c_void = core::ptr::null_mut();
    let mut lowest_mask = this_cpu_cpumask_var_ptr(local_cpu_mask);
pub static mut this_cpu: c_int = 0;
pub static mut cpu: c_int = 0;
    let mut ret = 0;
// Make sure the mask is initialized first
    if (unlikely(!lowest_mask)) {
    return -1;
    }
    if (task.nr_cpus_allowed == 1) {
    return -1; /* No other targets possible */
    }
//
// If we're on asym system ensure we consider the different capacities
// of the CPUs when searching for the lowest_mask.
//
    if (sched_asym_cpucap_active()) {
    ret = cpupri_find_fitness(&task_rq(task).rd.cpupri,
    task, lowest_mask,
    rt_task_fits_capacity);
    } else {
    ret = cpupri_find(&task_rq(task).rd.cpupri,
    task, lowest_mask);
    }
    if (!ret) {
    return -1; /* No targets found */
    }
//
// At this point we have built a mask of CPUs representing the
// lowest priority tasks in the system.  Now we want to elect
// the best one based on our affinity and topology.
//
// We prioritize the last CPU that the task executed on since
// it is most likely cache-hot in that location.
//
    if (cpumask_test_cpu(cpu, lowest_mask)) {
    return cpu;
    }
//
// Otherwise, we consult the sched_domains span maps to figure
// out which CPU is logically closest to our hot cache data.
//
    if (!cpumask_test_cpu(this_cpu, lowest_mask)) {
    this_cpu = -1; /* Skip this_cpu opt if not among lowest */
    }
    rcu_read_lock();
    for_each_domain(cpu, sd) {
    if (sd.flags & SD_WAKE_AFFINE) {
    let mut best_cpu = 0;
//
// "this_cpu" is cheaper to preempt than a
// remote processor.
//
    if (this_cpu != -1 &&
    cpumask_test_cpu(this_cpu, sched_domain_span(sd))) {
    rcu_read_unlock();
    return this_cpu;
    }
    best_cpu = cpumask_any_and_distribute(lowest_mask,
    sched_domain_span(sd));
    if (best_cpu < nr_cpu_ids) {
    rcu_read_unlock();
    return best_cpu;
    }
    }
    }
    rcu_read_unlock();
//
// And finally, if there were no matches within the domains
// just give the caller *something* to work with from the compatible
// locations.
//
    if (this_cpu != -1) {
    return this_cpu;
    }
    cpu = cpumask_any_distribute(lowest_mask);
    if (cpu < nr_cpu_ids) {
    return cpu;
    }
    return -1;
    }
#[no_mangle]
pub unsafe extern "C" fn pick_next_pushable_task(rq: *mut rq) -> *mut c_void {
    let mut head = &rq.rt.pushable_tasks;
    struct task_struct *i, *p = core::ptr::null_mut();
    if (!has_pushable_tasks(rq)) {
    return core::ptr::null_mut();
    }
    plist_for_each_entry(i, head, pushable_tasks) {
// make sure task isn't on_cpu (possible with proxy-exec)
    if (!task_on_cpu(rq, i)) {
    p = i;
    break;
    }
    }
    if (!p) {
    return core::ptr::null_mut();
    }
    BUG_ON!(rq.cpu != task_cpu(p));
    BUG_ON!(task_current(rq, p));
    BUG_ON!(task_current_donor(rq, p));
    BUG_ON!(p.nr_cpus_allowed <= 1);
    BUG_ON!(!task_on_rq_queued(p));
    BUG_ON!(!rt_task(p));
    return p;
    }
// Will lock the rq it finds
#[no_mangle]
pub unsafe extern "C" fn find_lock_lowest_rq(task: *mut task_struct, rq: *mut rq) -> *mut c_void {
    let mut lowest_rq = core::ptr::null_mut();
    let mut tries = 0;
    let mut cpu = 0;
    while (tries < RT_MAX_TRIES) {
    cpu = find_lowest_rq(task);
    if ((cpu == -1) || (cpu == rq.cpu)) {
    break;
    }
    lowest_rq = cpu_rq(cpu);
    if (lowest_rq.rt.highest_prio.curr <= task.prio) {
//
// Target rq has tasks of equal or higher priority,
// retrying does not release any lock and is unlikely
// to yield a different result.
//
    lowest_rq = core::ptr::null_mut();
    break;
    }
// if the prio of this runqueue changed, try again
    if (double_lock_balance(rq, lowest_rq)) {
//
// We had to unlock the run queue. In
// the mean time, task could have
// migrated already or had its affinity changed,
// therefore check if the task is still at the
// head of the pushable tasks list.
// It is possible the task was scheduled, set
// "migrate_disabled" and then got preempted, so we must
// check the task migration disable flag here too.
//
    if (unlikely(is_migration_disabled(task) ||
    !cpumask_test_cpu(lowest_rq.cpu, &task.cpus_mask) ||
    task != pick_next_pushable_task(rq))) {
    double_unlock_balance(rq, lowest_rq);
    lowest_rq = core::ptr::null_mut();
    break;
    }
    }
// If this rq is still suitable use it.
    if (lowest_rq.rt.highest_prio.curr > task.prio) {
    break;
    }
// try again
    double_unlock_balance(rq, lowest_rq);
    lowest_rq = core::ptr::null_mut();
    }
    return lowest_rq;
    }
//
// If the current CPU has more than one RT task, see if the non
// running task can migrate over to a CPU that is running a task
// of lesser priority.
//
#[no_mangle]
unsafe extern "C" fn push_rt_task(rq: *mut rq, pull: bool) -> c_int {
pub static mut next_task: *mut c_void = core::ptr::null_mut();
pub static mut lowest_rq: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    if (!rq.rt.overloaded) {
    return 0;
    }
    next_task = pick_next_pushable_task(rq);
    if (!next_task) {
    return 0;
    }
// label;
//
// It's possible that the next_task slipped in of
// higher priority than current. If that's the case
// just reschedule current.
//
    if (unlikely(next_task.prio < rq.donor.prio)) {
    resched_curr(rq);
    return 0;
    }
    if (is_migration_disabled(next_task)) {
    let mut push_task = core::ptr::null_mut();
    let mut cpu = 0;
    if (!pull || rq.push_busy) {
    return 0;
    }
//
// Invoking find_lowest_rq() on anything but an RT task doesn't
// make sense. Per the above priority check, curr has to
// be of higher priority than next_task, so no need to
// reschedule when bailing out.
//
// Note that the stoppers are masqueraded as SCHED_FIFO
// (cf. sched_set_stop_task()), so we can't rely on rt_task().
//
    if (rq.donor.sched_class != &rt_sched_class) {
    return 0;
    }
    cpu = find_lowest_rq(rq.curr);
    if (cpu == -1 || cpu == rq.cpu) {
    return 0;
    }
//
// Given we found a CPU with lower priority than @next_task,
// therefore it should be running. However we cannot migrate it
// to this other CPU, instead attempt to push the current
// running task on this CPU away.
//
    push_task = get_push_task(rq);
    if (push_task) {
    preempt_disable();
    raw_spin_rq_unlock(rq);
    stop_one_cpu_nowait(rq.cpu, push_cpu_stop,
    push_task, &rq.push_work);
    preempt_enable();
    raw_spin_rq_lock(rq);
    }
    return 0;
    }
    if (WARN_ON!(next_task == rq.curr)) {
    return 0;
    }
// We might release rq lock
    get_task_struct(next_task);
// find_lock_lowest_rq locks the rq if found
    lowest_rq = find_lock_lowest_rq(next_task, rq);
    if (!lowest_rq) {
pub static mut task: *mut c_void = core::ptr::null_mut();
//
// find_lock_lowest_rq releases rq->lock
// so it is possible that next_task has migrated.
//
// We need to make sure that the task is still on the same
// run-queue and is also still the next task eligible for
// pushing.
//
    task = pick_next_pushable_task(rq);
    if (task == next_task) {
//
// The task hasn't migrated, and is still the next
// eligible task, but we failed to find a run-queue
// to push it to.  Do not retry in this case, since
// other CPUs will pull from us when ready.
//
// goto;
    }
    if (!task) {
// No more tasks, just exit
// goto;
    }
//
// Something has shifted, try again.
//
    put_task_struct(next_task);
    next_task = task;
// goto;
    }
    move_queued_task_locked(rq, lowest_rq, next_task);
    resched_curr(lowest_rq);
    ret = 1;
    double_unlock_balance(rq, lowest_rq);
// label;
    put_task_struct(next_task);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn push_rt_tasks(rq: *mut rq) {
// push_rt_task will return true if it moved an RT
    while (push_rt_task(rq, false)) {
    ;
    }
    }

//
// When a high priority task schedules out from a CPU and a lower priority
// task is scheduled in, a check is made to see if there's any RT tasks
// on other CPUs that are waiting to run because a higher priority RT task
// is currently running on its CPU. In this case, the CPU with multiple RT
// tasks queued on it (overloaded) needs to be notified that a CPU has opened
// up that may be able to run one of its non-running queued RT tasks.
//
// All CPUs with overloaded RT tasks need to be notified as there is currently
// no way to know which of these CPUs have the highest priority task waiting
// to run. Instead of trying to take a spinlock on each of these CPUs,
// which has shown to cause large latency when done on machines with many
// CPUs, sending an IPI to the CPUs to have them push off the overloaded
// RT tasks waiting to run.
//
// Just sending an IPI to each of the CPUs is also an issue, as on large
// count CPU machines, this can cause an IPI storm on a CPU, especially
// if its the only CPU with multiple RT tasks queued, and a large number
// of CPUs scheduling a lower priority task at the same time.
//
// Each root domain has its own IRQ work function that can iterate over
// all CPUs with RT overloaded tasks. Since all CPUs with overloaded RT
// task must be checked if there's one or many CPUs that are lowering
// their priority, there's a single IRQ work iterator that will try to
// push off RT tasks that are waiting to run.
//
// When a CPU schedules a lower priority task, it will kick off the
// IRQ work iterator that will jump to each CPU with overloaded RT tasks.
// As it only takes the first CPU that schedules a lower priority task
// to start the process, the rto_start variable is incremented and if
// the atomic result is one, then that CPU will try to take the rto_lock.
// This prevents high contention on the lock as the process handles all
// CPUs scheduling lower priority tasks.
//
// All CPUs that are scheduling a lower priority task will increment the
// rt_loop_next variable. This will make sure that the IRQ work iterator
// checks all RT overloaded CPUs whenever a CPU schedules a new lower
// priority task, even if the iterator is in the middle of a scan. Incrementing
// the rt_loop_next will cause the iterator to perform another scan.
//
#[no_mangle]
unsafe extern "C" fn rto_next_cpu(rd: *mut root_domain) -> c_int {
pub static mut this_cpu: c_int = 0;
    let mut next = 0;
    let mut cpu = 0;
//
// When starting the IPI RT pushing, the rto_cpu is set to -1,
// rt_next_cpu() will simply return the first CPU found in
// the rto_mask.
//
// If rto_next_cpu() is called with rto_cpu is a valid CPU, it
// will return the next CPU found in the rto_mask.
//
// If there are no more CPUs left in the rto_mask, then a check is made
// against rto_loop and rto_loop_next. rto_loop is only updated with
// the rto_lock held, but any CPU may increment the rto_loop_next
// without any locking.
//
    for (;;) {
// When rto_cpu is -1 this acts like cpumask_first()
    cpu = cpumask_next(rd.rto_cpu, rd.rto_mask);
    rd.rto_cpu = cpu;
// Do not send IPI to self
    if (cpu == this_cpu) {
    continue;
    }
    if (cpu < nr_cpu_ids) {
    return cpu;
    }
    rd.rto_cpu = -1;
//
// ACQUIRE ensures we see the @rto_mask changes
// made prior to the @next value observed.
//
// Matches WMB in rt_set_overload().
//
    next = atomic_read_acquire(&rd.rto_loop_next);
    if (rd.rto_loop == next) {
    break;
    }
    rd.rto_loop = next;
    }
    return -1;
    }
#[no_mangle]
pub unsafe extern "C" fn rto_start_trylock(v: *mut core::sync::atomic::AtomicI32) -> bool {
    return !atomic_cmpxchg_acquire(v, 0, 1);
    }
#[no_mangle]
pub unsafe extern "C" fn rto_start_unlock(v: *mut core::sync::atomic::AtomicI32) {
    atomic_set_release(v, 0);
    }
#[no_mangle]
unsafe extern "C" fn tell_cpu_to_push(rq: *mut rq) {
pub static mut cpu: c_int = 0;
// Keep the loop going if the IPI is currently active
    atomic_inc(&rq.rd.rto_loop_next);
// Only one CPU can initiate a loop at a time
    if (!rto_start_trylock(&rq.rd.rto_loop_start)) {
    return;
    }
    raw_spin_lock(&rq.rd.rto_lock);
//
// The rto_cpu is updated under the lock, if it has a valid CPU
// then the IPI is still running and will continue due to the
// update to loop_next, and nothing needs to be done here.
// Otherwise it is finishing up and an IPI needs to be sent.
//
    if (rq.rd.rto_cpu < 0) {
    cpu = rto_next_cpu(rq.rd);
    }
    raw_spin_unlock(&rq.rd.rto_lock);
    rto_start_unlock(&rq.rd.rto_loop_start);
    if (cpu >= 0) {
// Make sure the rd does not get freed while pushing
    sched_get_rd(rq.rd);
    irq_work_queue_on(&rq.rd.rto_push_work, cpu);
    }
    }
// Called from hardirq context
#[no_mangle]
pub unsafe extern "C" fn rto_push_irq_work_func(work: *mut irq_work) {
    let mut rd = container_of!(work, root_domain, rto_push_work);
pub static mut rq: *mut c_void = core::ptr::null_mut();
    let mut cpu = 0;
    rq = this_rq();
//
// We do not need to grab the lock to check for has_pushable_tasks.
// When it gets updated, a check is made if a push is possible.
//
    if (has_pushable_tasks(rq)) {
    raw_spin_rq_lock(rq);
    while (push_rt_task(rq, true)) {
    ;
    }
    raw_spin_rq_unlock(rq);
    }
    raw_spin_lock(&rd.rto_lock);
// Pass the IPI to the next rt overloaded queue
    cpu = rto_next_cpu(rd);
    raw_spin_unlock(&rd.rto_lock);
    if (cpu < 0) {
    sched_put_rd(rd);
    return;
    }
// Try the next RT overloaded CPU
    irq_work_queue_on(&rd.rto_push_work, cpu);
    }

#[no_mangle]
unsafe extern "C" fn pull_rt_task(this_rq: *mut rq) {
pub static mut this_cpu: c_int = 0;
pub static mut resched: bool = false;
    let mut p = core::ptr::null_mut();
    let mut push_task = core::ptr::null_mut();
pub static mut src_rq: *mut c_void = core::ptr::null_mut();
pub static mut rt_overload_count: c_int = 0;
    if (likely(!rt_overload_count)) {
    return;
    }
//
// Match the barrier from rt_set_overloaded; this guarantees that if we
// see overloaded we must also see the rto_mask bit.
//
    smp_rmb();
// If we are the only overloaded CPU do nothing
    if (rt_overload_count == 1 &&
    cpumask_test_cpu(this_rq.cpu, this_rq.rd.rto_mask)) {
    return;
    }

    if (sched_feat(RT_PUSH_IPI)) {
    tell_cpu_to_push(this_rq);
    return;
    }

    for_each_cpu(cpu, this_rq.rd.rto_mask) {
    if (this_cpu == cpu) {
    continue;
    }
    src_rq = cpu_rq(cpu);
//
// Don't bother taking the src_rq->lock if the next highest
// task is known to be lower-priority than our current task.
// This may look racy, but if this value is about to go
// logically higher, the src_rq will push this task away.
// And if its going logically lower, we do not care
//
    if (src_rq.rt.highest_prio.next >=
    this_rq.rt.highest_prio.curr) {
    continue;
    }
//
// We can potentially drop this_rq's lock in
// double_lock_balance, and another CPU could
// alter this_rq
//
    push_task = core::ptr::null_mut();
    double_lock_balance(this_rq, src_rq);
//
// We can pull only a task, which is pushable
// on its rq, and no others.
//
    p = pick_highest_pushable_task(src_rq, this_cpu);
//
// Do we have an RT task that preempts
// the to-be-scheduled task?
//
    if (p && (p.prio < this_rq.rt.highest_prio.curr)) {
    WARN_ON!(p == src_rq.curr);
    WARN_ON!(!task_on_rq_queued(p));
//
// There's a chance that p is higher in priority
// than what's currently running on its CPU.
// This is just that p is waking up and hasn't
// had a chance to schedule. We only pull
// p if it is lower in priority than the
// current task on the run queue
//
    if (p.prio < src_rq.donor.prio) {
// goto;
    }
    if (is_migration_disabled(p)) {
    push_task = get_push_task(src_rq);
    } else {
    move_queued_task_locked(src_rq, this_rq, p);
    resched = true;
    }
//
// We continue with the search, just in
// case there's an even higher prio task
// in another runqueue. (low likelihood
// but possible)
//
    }
// label;
    double_unlock_balance(this_rq, src_rq);
    if (push_task) {
    preempt_disable();
    raw_spin_rq_unlock(this_rq);
    stop_one_cpu_nowait(src_rq.cpu, push_cpu_stop,
    push_task, &src_rq.push_work);
    preempt_enable();
    raw_spin_rq_lock(this_rq);
    }
    }
    if (resched) {
    resched_curr(this_rq);
    }
    }
//
// If we are not running and we are not going to reschedule soon, we should
// try to push tasks away now
//
#[no_mangle]
unsafe extern "C" fn task_woken_rt(rq: *mut rq, p: *mut task_struct) {
    let mut need_to_push = !task_on_cpu(rq, p) &&
    !test_tsk_need_resched(rq.curr) &&
    p.nr_cpus_allowed > 1 &&
    (dl_task(rq.donor) || rt_task(rq.donor)) &&
    (rq.curr.nr_cpus_allowed < 2 ||
    rq.donor.prio <= p.prio);
    if (need_to_push) {
    push_rt_tasks(rq);
    }
    }
// Assumes rq->lock is held
#[no_mangle]
unsafe extern "C" fn rq_online_rt(rq: *mut rq) {
    if (rq.rt.overloaded) {
    rt_set_overload(rq);
    }
    __enable_runtime(rq);
    cpupri_set(&rq.rd.cpupri, rq.cpu, rq.rt.highest_prio.curr);
    }
// Assumes rq->lock is held
#[no_mangle]
unsafe extern "C" fn rq_offline_rt(rq: *mut rq) {
    if (rq.rt.overloaded) {
    rt_clear_overload(rq);
    }
    __disable_runtime(rq);
    cpupri_set(&rq.rd.cpupri, rq.cpu, CPUPRI_INVALID);
    }
//
// When switch from the rt queue, we bring ourselves to a position
// that we might want to pull RT tasks from other runqueues.
//
#[no_mangle]
unsafe extern "C" fn switched_from_rt(rq: *mut rq, p: *mut task_struct) {
//
// If there are other RT tasks then we will reschedule
// and the scheduling of the other RT tasks will handle
// the balancing. But if we are the last RT task
// we may need to handle the pulling of RT tasks
// now.
//
    if (!task_on_rq_queued(p) || rq.rt.rt_nr_running) {
    return;
    }
    rt_queue_pull_task(rq);
    }
#[no_mangle]
pub unsafe extern "C" fn init_sched_rt_class()  {
    let mut i = 0;
    for_each_possible_cpu(i) {
    zalloc_cpumask_var_node(&per_cpu(local_cpu_mask, i),
    GFP_KERNEL, cpu_to_node(i));
    }
    }
//
// When switching a task to RT, we may overload the runqueue
// with RT tasks. In this case we try to push them off to
// other runqueues.
//
#[no_mangle]
unsafe extern "C" fn switched_to_rt(rq: *mut rq, p: *mut task_struct) {
//
// If we are running, update the avg_rt tracking, as the running time
// will now on be accounted into the latter.
//
    if (task_current(rq, p)) {
    update_rt_rq_load_avg(rq_clock_pelt(rq), rq, 0);
    return;
    }
//
// If we are not running we may need to preempt the current
// running task. If that current running task is also an RT task
// then see if we can move to another run queue.
//
    if (task_on_rq_queued(p)) {
    if (p.nr_cpus_allowed > 1 && rq.rt.overloaded) {
    rt_queue_push_tasks(rq);
    }
    if (p.prio < rq.donor.prio && cpu_online(cpu_of(rq))) {
    resched_curr(rq);
    }
    }
    }
//
// Priority of the task has changed. This may cause
// us to initiate a push or pull.
//
#[no_mangle]
pub unsafe extern "C" fn prio_changed_rt(rq: *mut rq, p: *mut task_struct, oldprio: u64) {
    if (!task_on_rq_queued(p)) {
    return;
    }
    if (p.prio == oldprio) {
    return;
    }
    if (task_current_donor(rq, p)) {
//
// If our priority decreases while running, we
// may need to pull tasks to this runqueue.
//
    if (oldprio < p.prio) {
    rt_queue_pull_task(rq);
    }
//
// If there's a higher priority task waiting to run
// then reschedule.
//
    if (p.prio > rq.rt.highest_prio.curr) {
    resched_curr(rq);
    }
    } else {
//
// This task is not running, but if it is
// greater than the current running task
// then reschedule.
//
    if (p.prio < rq.donor.prio) {
    resched_curr(rq);
    }
    }
    }

#[no_mangle]
unsafe extern "C" fn watchdog(rq: *mut rq, p: *mut task_struct) {
    unsigned long soft, hard;
// max may change after cur was read, this will be fixed next tick
    soft = task_rlimit(p, RLIMIT_RTTIME);
    hard = task_rlimit_max(p, RLIMIT_RTTIME);
    if (soft != RLIM_INFINITY) {
    let mut next = 0;
    if (p.rt.watchdog_stamp != jiffies) {
    p.rt.timeout += 1;
    p.rt.watchdog_stamp = jiffies;
    }
    next = DIV_ROUND_UP(min(soft, hard), USEC_PER_SEC/HZ);
    if (p.rt.timeout > next) {
    posix_cputimers_rt_watchdog(&p.posix_cputimers,
    p.se.sum_exec_runtime);
    }
    }
    }

#[no_mangle]
pub unsafe extern "C" fn watchdog(rq: *mut rq, p: *mut task_struct) { }

//
// scheduler tick hitting a task of our scheduling class.
//
// NOTE: This function can be called remotely by the tick offload that
// goes along full dynticks. Therefore no local assumption can be made
// and everything must be accessed through the @rq and @curr passed in
// parameters.
//
#[no_mangle]
unsafe extern "C" fn task_tick_rt(rq: *mut rq, p: *mut task_struct, queued: c_int) {
    let mut rt_se = &p.rt;
    update_curr_rt(rq);
    update_rt_rq_load_avg(rq_clock_pelt(rq), rq, 1);
    watchdog(rq, p);
//
// RR tasks need a special form of time-slice management.
// FIFO tasks have no timeslices.
//
    if (p.policy != SCHED_RR) {
    return;
    }
    if (--p.rt.time_slice) {
    return;
    }
    p.rt.time_slice = sched_rr_timeslice;
//
// Requeue to the end of queue if we (and all of our ancestors) are not
// the only element on the queue
//
    for_each_sched_rt_entity(rt_se) {
    if (rt_se.run_list.prev != rt_se.run_list.next) {
    requeue_task_rt(rq, p, 0);
    resched_curr(rq);
    return;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn get_rr_interval_rt(rq: *mut rq, task: *mut task_struct) -> c_uint {
//
// Time slice is 0 for SCHED_FIFO tasks
//
    if (task.policy == SCHED_RR) {
    return sched_rr_timeslice;
    }
    else {
    return 0;
    }
    }

#[no_mangle]
unsafe extern "C" fn task_is_throttled_rt(p: *mut task_struct, cpu: c_int) -> c_int {
pub static mut rt_rq: *mut c_void = core::ptr::null_mut();

    rt_rq = task_group(p).rt_rq[cpu];
    WARN_ON!(!rt_group_sched_enabled() && rt_rq.tg != &root_task_group);

    rt_rq = &cpu_rq(cpu).rt;

    return rt_rq_throttled(rt_rq);
    }

    DEFINE_SCHED_CLASS(rt) = {
    .enqueue_task		= enqueue_task_rt,
    .dequeue_task		= dequeue_task_rt,
    .yield_task		= yield_task_rt,
    .wakeup_preempt		= wakeup_preempt_rt,
    .pick_task		= pick_task_rt,
    .put_prev_task		= put_prev_task_rt,
    .set_next_task          = set_next_task_rt,
    .balance		= balance_rt,
    .select_task_rq		= select_task_rq_rt,
    .set_cpus_allowed       = set_cpus_allowed_common,
    .rq_online              = rq_online_rt,
    .rq_offline             = rq_offline_rt,
    .task_woken		= task_woken_rt,
    .switched_from		= switched_from_rt,
    .find_lock_rq		= find_lock_lowest_rq,
    .task_tick		= task_tick_rt,
    .get_rr_interval	= get_rr_interval_rt,
    .switched_to		= switched_to_rt,
    .prio_changed		= prio_changed_rt,
    .update_curr		= update_curr_rt,

    .task_is_throttled	= task_is_throttled_rt,

    .uclamp_enabled		= 1,

    };

//
// Ensure that the real time constraints are schedulable.
//
pub static mut rt_constraints_mutex: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn tg_has_rt_tasks(tg: *mut task_group) -> c_int {
pub static mut task: *mut c_void = core::ptr::null_mut();
pub static mut it: usize = 0;
pub static mut ret: c_int = 0;
//
// Autogroups do not have RT tasks; see autogroup_create().
//
    if (task_group_is_autogroup(tg)) {
    return 0;
    }
    css_task_iter_start(&tg.css, 0, &it);
    while (!ret && (task = css_task_iter_next(&it))) {
    ret |= rt_task(task);
    }
    css_task_iter_end(&it);
    return ret;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt_schedulable_data {
    pub tg: *mut task_group,
    pub rt_period: u64,
    pub rt_runtime: u64,
}

#[no_mangle]
unsafe extern "C" fn tg_rt_schedulable(tg: *mut task_group, data: *mut c_void) -> c_int {
    let mut d = data;
pub static mut child: *mut c_void = core::ptr::null_mut();
    u64 total, sum = 0;
    u64 period, runtime;
    period = ktime_to_ns(tg.rt_bandwidth.rt_period);
    runtime = tg.rt_bandwidth.rt_runtime;
    if (tg == d.tg) {
    period = d.rt_period;
    runtime = d.rt_runtime;
    }
//
// Cannot have more runtime than the period.
//
    if (runtime > period && runtime != RUNTIME_INF) {
    return -EINVAL;
    }
//
// Ensure we don't starve existing RT tasks if runtime turns zero.
//
    if (rt_bandwidth_enabled() && !runtime &&
    tg.rt_bandwidth.rt_runtime && tg_has_rt_tasks(tg)) {
    return -EBUSY;
    }
    total = to_ratio(period, runtime);
//
// Nobody can have more than the global setting allows.
//
    if (total > to_ratio(global_rt_period(), global_rt_runtime())) {
    return -EINVAL;
    }
//
// The sum of our children's runtime should not exceed our own.
//
    list_for_each_entry_rcu(child, &tg.children, siblings) {
    period = ktime_to_ns(child.rt_bandwidth.rt_period);
    runtime = child.rt_bandwidth.rt_runtime;
    if (child == d.tg) {
    period = d.rt_period;
    runtime = d.rt_runtime;
    }
    sum += to_ratio(period, runtime);
    }
    if (sum > total) {
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn __rt_schedulable(tg: *mut task_group, period: u64, runtime: u64) -> c_int {
    let mut ret = 0;
pub static mut rt_schedulable_data: usize = 0;
    rcu_read_lock();
    ret = walk_tg_tree(tg_rt_schedulable, tg_nop, &data);
    rcu_read_unlock();
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn tg_set_rt_bandwidth(tg: *mut task_group, rt_period: u64, rt_runtime: u64) -> c_int {
    int i, err = 0;
//
// Disallowing the root group RT runtime is BAD, it would disallow the
// kernel creating (and or operating) RT threads.
//
    if (tg == &root_task_group && rt_runtime == 0) {
    return -EINVAL;
    }
// No period doesn't make any sense.
    if (rt_period == 0) {
    return -EINVAL;
    }
//
// Bound quota to defend quota against overflow during bandwidth shift.
//
    if (rt_runtime != RUNTIME_INF && rt_runtime > max_rt_runtime) {
    return -EINVAL;
    }
    mutex_lock(&rt_constraints_mutex);
    err = __rt_schedulable(tg, rt_period, rt_runtime);
    if (err) {
// goto;
    }
    raw_spin_lock_irq(&tg.rt_bandwidth.rt_runtime_lock);
    tg.rt_bandwidth.rt_period = ns_to_ktime(rt_period);
    tg.rt_bandwidth.rt_runtime = rt_runtime;
    for_each_possible_cpu(i) {
    let mut rt_rq = tg.rt_rq[i];
    raw_spin_lock(&rt_rq.rt_runtime_lock);
    rt_rq.rt_runtime = rt_runtime;
    raw_spin_unlock(&rt_rq.rt_runtime_lock);
    }
    raw_spin_unlock_irq(&tg.rt_bandwidth.rt_runtime_lock);
// label;
    mutex_unlock(&rt_constraints_mutex);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn sched_group_set_rt_runtime(tg: *mut task_group, rt_runtime_us: c_long) -> c_int {
    u64 rt_runtime, rt_period;
    rt_period = ktime_to_ns(tg.rt_bandwidth.rt_period);
    rt_runtime = (u64)rt_runtime_us * NSEC_PER_USEC;
    if (rt_runtime_us < 0) {
    rt_runtime = RUNTIME_INF;
    }

    else if ((u64)rt_runtime_us > U64_MAX / NSEC_PER_USEC) {
    return -EINVAL;
    }
    return tg_set_rt_bandwidth(tg, rt_period, rt_runtime);
    }
#[no_mangle]
pub unsafe extern "C" fn sched_group_rt_runtime(tg: *mut task_group) -> c_long {
    let mut rt_runtime_us = 0;
    if (tg.rt_bandwidth.rt_runtime == RUNTIME_INF) {
    return -1;
    }
    rt_runtime_us = tg.rt_bandwidth.rt_runtime;
    do_div(rt_runtime_us, NSEC_PER_USEC);
    return rt_runtime_us;
    }
#[no_mangle]
pub unsafe extern "C" fn sched_group_set_rt_period(tg: *mut task_group, rt_period_us: u64) -> c_int {
    u64 rt_runtime, rt_period;
    if (rt_period_us > U64_MAX / NSEC_PER_USEC) {
    return -EINVAL;
    }
    rt_period = rt_period_us * NSEC_PER_USEC;
    rt_runtime = tg.rt_bandwidth.rt_runtime;
    return tg_set_rt_bandwidth(tg, rt_period, rt_runtime);
    }
#[no_mangle]
pub unsafe extern "C" fn sched_group_rt_period(tg: *mut task_group) -> c_long {
    let mut rt_period_us = 0;
    rt_period_us = ktime_to_ns(tg.rt_bandwidth.rt_period);
    do_div(rt_period_us, NSEC_PER_USEC);
    return rt_period_us;
    }
#[no_mangle]
pub unsafe extern "C" fn sched_rt_can_attach(tg: *mut task_group, tsk: *mut task_struct) -> c_int {
// Don't accept real-time tasks when there is no way for them to run
    if (rt_group_sched_enabled() && rt_task(tsk) && tg.rt_bandwidth.rt_runtime == 0) {
    return 0;
    }
    return 1;
    }

#[no_mangle]
unsafe extern "C" fn sched_rt_global_validate() -> c_int {
    if ((sysctl_sched_rt_runtime != RUNTIME_INF) &&
    ((sysctl_sched_rt_runtime > sysctl_sched_rt_period) ||
    ((u64)sysctl_sched_rt_runtime *
    NSEC_PER_USEC > max_rt_runtime))) {
    return -EINVAL;
    }

    if (!rt_group_sched_enabled()) {
    return 0;
    }
    scoped_guard(mutex, &rt_constraints_mutex)
    return __rt_schedulable(core::ptr::null_mut(), 0, 0);

    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn sched_rt_handler(table: *mut ctl_table, write: c_int, buffer: *mut c_void, lenp: *mut size_t, ppos: *mut loff_t) -> c_int {
    let mut old_period = 0;
    let mut old_runtime = 0;
pub static mut mutex: usize = 0;
    let mut ret = 0;
    mutex_lock(&mutex);
    sched_domains_mutex_lock();
    old_period = sysctl_sched_rt_period;
    old_runtime = sysctl_sched_rt_runtime;
    ret = proc_dointvec_minmax(table, write, buffer, lenp, ppos);
    if (!ret && write) {
    ret = sched_rt_global_validate();
    if (ret) {
// goto;
    }
    ret = sched_dl_global_validate();
    if (ret) {
// goto;
    }
    sched_dl_do_global();
    }
    if (0) {
// label;
    sysctl_sched_rt_period = old_period;
    sysctl_sched_rt_runtime = old_runtime;
    }
    sched_domains_mutex_unlock();
    mutex_unlock(&mutex);
//
// After changing maximum available bandwidth for DEADLINE, we need to
// recompute per root domain and per cpus variables accordingly.
//
    rebuild_sched_domains();
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn sched_rr_handler(table: *mut ctl_table, write: c_int, buffer: *mut c_void, lenp: *mut size_t, ppos: *mut loff_t) -> c_int {
    let mut ret = 0;
pub static mut mutex: usize = 0;
    mutex_lock(&mutex);
    ret = proc_dointvec(table, write, buffer, lenp, ppos);
//
// Make sure that internally we keep jiffies.
// Also, writing zero resets the time-slice to default:
//
    if (!ret && write) {
    sched_rr_timeslice =
    sysctl_sched_rr_timeslice <= 0 ? RR_TIMESLICE :
    msecs_to_jiffies(sysctl_sched_rr_timeslice);
    if (sysctl_sched_rr_timeslice <= 0) {
    sysctl_sched_rr_timeslice = jiffies_to_msecs(RR_TIMESLICE);
    }
    }
    mutex_unlock(&mutex);
    return ret;
    }

#[no_mangle]
pub unsafe extern "C" fn print_rt_stats(m: *mut seq_file, cpu: c_int) {
    let mut iter;
pub static mut rt_rq: *mut c_void = core::ptr::null_mut();
    rcu_read_lock();
    for_each_rt_rq(rt_rq, iter, cpu_rq(cpu)) {
    print_rt_rq(m, cpu, rt_rq);
    }
    rcu_read_unlock();
    }