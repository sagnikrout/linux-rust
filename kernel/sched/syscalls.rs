//! Automatically rewritten from C to Rust
//! Source: kernel/sched/syscalls.c
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
// kernel/sched/syscalls.c
//
// Core kernel scheduler syscalls related code
//
// Copyright (C) 1991-2002  Linus Torvalds
// Copyright (C) 1998-2024  Ingo Molnar, Red Hat
//

#[no_mangle]
pub unsafe extern "C" fn __normal_prio(policy: c_int, rt_prio: c_int, nice: c_int) -> c_int {
    let mut prio = 0;
    if (dl_policy(policy)) {
    prio = MAX_DL_PRIO - 1;
    }

    else if (rt_policy(policy)) {
    prio = MAX_RT_PRIO - 1 - rt_prio;
    }
    else {
    prio = NICE_TO_PRIO(nice);
    }
    return prio;
    }
//
// Calculate the expected normal priority: i.e. priority
// without taking RT-inheritance into account. Might be
// boosted by interactivity modifiers. Changes upon fork,
// setprio syscalls, and whenever the interactivity
// estimator recalculates.
//
#[no_mangle]
pub unsafe extern "C" fn normal_prio(p: *mut task_struct) -> c_int {
    return __normal_prio(p.policy, p.rt_priority, PRIO_TO_NICE(p.static_prio));
    }
//
// Calculate the current priority, i.e. the priority
// taken into account by the scheduler. This value might
// be boosted by RT tasks, or might be boosted by
// interactivity modifiers. Will be RT if the task got
// RT-boosted. If not then it returns p->normal_prio.
//
#[no_mangle]
unsafe extern "C" fn effective_prio(p: *mut task_struct) -> c_int {
    p.normal_prio = normal_prio(p);
//
// If we are RT tasks or we were boosted to RT priority,
// keep the priority unchanged. Otherwise, update priority
// to the normal priority:
//
    if (!rt_or_dl_prio(p.prio)) {
    return p.normal_prio;
    }
    return p.prio;
    }
#[no_mangle]
pub unsafe extern "C" fn set_user_nice(p: *mut task_struct, nice: c_long) {
    let mut old_prio = 0;
    if (task_nice(p) == nice || nice < MIN_NICE || nice > MAX_NICE) {
    return;
    }
//
// We have to be careful, if called from sys_setpriority(),
// the task might be in the middle of scheduling on another CPU.
//
    guard(task_rq_lock)(p);
//
// The RT priorities are set via sched_setscheduler(), but we still
// allow the 'normal' nice value to be set - but as expected
// it won't have any effect on scheduling until the task is
// SCHED_DEADLINE, SCHED_FIFO or SCHED_RR:
//
    if (task_has_dl_policy(p) || task_has_rt_policy(p)) {
    p.static_prio = NICE_TO_PRIO(nice);
    return;
    }
    scoped_guard (sched_change, p, DEQUEUE_SAVE) {
    p.static_prio = NICE_TO_PRIO(nice);
    set_load_weight(p, true);
    old_prio = p.prio;
    p.prio = effective_prio(p);
    }
    }
    EXPORT_SYMBOL(set_user_nice);
//
// is_nice_reduction - check if nice value is an actual reduction
//
// Similar to can_nice() but does not perform a capability check.
//
// @p: task
// @nice: nice value
//
#[no_mangle]
unsafe extern "C" fn is_nice_reduction(p: *const task_struct, nice: c_int) -> bool {
// Convert nice value [19,-20] to rlimit style value [1,40]:
pub static mut nice_rlim: c_int = 0;
    return (nice_rlim <= task_rlimit(p, RLIMIT_NICE));
    }
//
// can_nice - check if a task can reduce its nice value
// @p: task
// @nice: nice value
//
#[no_mangle]
pub unsafe extern "C" fn can_nice(p: *const task_struct, nice: c_int) -> c_int {
    return is_nice_reduction(p, nice) || capable(CAP_SYS_NICE);
    }

//
// sys_nice - change the priority of the current process.
// @increment: priority increment
//
// sys_setpriority is a more generic, but much slower function that
// does similar things.
//
#[no_mangle]
pub unsafe extern "C" fn sys_nice(increment: usize) -> c_long {
    let mut nice = 0;
    let mut retval = 0;
//
// Setpriority might change our priority at the same moment.
// We don't have to worry. Conceptually one call occurs first
// and we have a single winner.
//
    increment = clamp(increment, -NICE_WIDTH, NICE_WIDTH);
    nice = task_nice(current) + increment;
    nice = clamp_val(nice, MIN_NICE, MAX_NICE);
    if (increment < 0 && !can_nice(current, nice)) {
    return -EPERM;
    }
    retval = security_task_setnice(current, nice);
    if (retval) {
    return retval;
    }
    set_user_nice(current, nice);
    return 0;
    }

//
// task_prio - return the priority value of a given task.
// @p: the task in question.
//
// Return: The priority value as seen by users in /proc.
//
// sched policy         return value   kernel prio    user prio/nice
//
// normal, batch, idle     [0 ... 39]  [100 ... 139]          0/[-20 ... 19]
// fifo, rr             [-2 ... -100]     [98 ... 0]  [1 ... 99]
// deadline                     -101             -1           0
//
#[no_mangle]
pub unsafe extern "C" fn task_prio(p: *const task_struct) -> c_int {
    return p.prio - MAX_RT_PRIO;
    }
//
// idle_cpu - is a given CPU idle currently?
// @cpu: the processor in question.
//
// Return: 1 if the CPU is currently idle. 0 otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn idle_cpu(cpu: c_int) -> c_int {
    return idle_rq(cpu_rq(cpu));
    }
//
// idle_task - return the idle task for a given CPU.
// @cpu: the processor in question.
//
// Return: The idle task for the CPU @cpu.
//
#[no_mangle]
pub unsafe extern "C" fn idle_task(cpu: c_int) -> *mut c_void {
    return cpu_rq(cpu).idle;
    }

#[no_mangle]
pub unsafe extern "C" fn sched_core_idle_cpu(cpu: c_int) -> c_int {
    let mut rq = cpu_rq(cpu);
    if (sched_core_enabled(rq) && rq.curr == rq.idle) {
    return 1;
    }
    return idle_cpu(cpu);
    }

//
// find_process_by_pid - find a process with a matching PID value.
// @pid: the pid in question.
//
// The task of @pid, if found. %NULL otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn find_process_by_pid(pid: pid_t) -> *mut c_void {
    return pid ? find_task_by_vpid(pid) : current;
    }
#[no_mangle]
pub unsafe extern "C" fn find_get_task(pid: pid_t) -> *mut c_void {
pub static mut p: *mut c_void = core::ptr::null_mut();
    guard(rcu)();
    p = find_process_by_pid(pid);
    if (likely(p)) {
    get_task_struct(p);
    }
    return p;
    }
    DEFINE_CLASS(find_get_task, task_struct *, if (_T) put_task_struct(_T),
    find_get_task(pid), pid_t pid)
//
// sched_setparam() passes in -1 for its policy, to let the functions
// it calls know not to change it.
//

#[no_mangle]
pub unsafe extern "C" fn __setscheduler_params(p: *mut task_struct, attr: *mut sched_attr) {
pub static mut policy: c_int = 0;
    if (policy == SETPARAM_POLICY) {
    policy = p.policy;
    }
    p.policy = policy;
    if (dl_policy(policy)) {
    __setparam_dl(p, attr);
    }

    else if (fair_policy(policy)) {
    __setparam_fair(p, attr);
    }
// rt-policy tasks do not have a timerslack
    if (rt_or_dl_task_policy(p)) {
    p.timer_slack_ns = 0;
    } else if (p.timer_slack_ns == 0) {
// when switching back to non-rt policy, restore timerslack
    p.timer_slack_ns = p.default_timer_slack_ns;
    }
//
// __sched_setscheduler() ensures attr->sched_priority == 0 when
// !rt_policy. Always setting this ensures that things like
// getparam()/getattr() don't report silly values for !rt tasks.
//
    p.rt_priority = attr.sched_priority;
    p.normal_prio = normal_prio(p);
    set_load_weight(p, true);
    }
//
// Check the target process has a UID that matches the current process's:
//
#[no_mangle]
unsafe extern "C" fn check_same_owner(p: *mut task_struct) -> bool {
    let mut cred = current_cred(), *pcred;
    guard(rcu)();
    pcred = __task_cred(p);
    return (uid_eq(cred.euid, pcred.euid) ||
    uid_eq(cred.euid, pcred.uid));
    }

#[no_mangle]
pub unsafe extern "C" fn __setscheduler_dl_pi(newprio: c_int, policy: c_int, p: *mut task_struct, scope: *mut sched_change_ctx) {
//
// In case a DEADLINE task (either proper or boosted) gets
// setscheduled to a lower priority class, check if it neeeds to
// inherit parameters from a potential pi_task. In that case make
// sure replenishment happens with the next enqueue.
//
    if (dl_prio(newprio) && !dl_policy(policy)) {
    let mut pi_task = rt_mutex_get_top_task(p);
    if (pi_task) {
    p.dl.pi_se = pi_task.dl.pi_se;
    scope.flags |= ENQUEUE_REPLENISH;
    }
    }
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: __setscheduler_dl_pi
pub unsafe extern "C" fn __setscheduler_dl_pi_dup(newprio: c_int, policy: c_int, p: *mut task_struct, scope: *mut sched_change_ctx) {
    }

#[no_mangle]
pub unsafe extern "C" fn uclamp_validate(p: *mut task_struct, attr: *mut sched_attr) -> c_int {
pub static mut util_min: c_int = 0;
pub static mut util_max: c_int = 0;
    if (attr.sched_flags & SCHED_FLAG_UTIL_CLAMP_MIN) {
    util_min = attr.sched_util_min;
    if (util_min + 1 > SCHED_CAPACITY_SCALE + 1) {
    return -EINVAL;
    }
    }
    if (attr.sched_flags & SCHED_FLAG_UTIL_CLAMP_MAX) {
    util_max = attr.sched_util_max;
    if (util_max + 1 > SCHED_CAPACITY_SCALE + 1) {
    return -EINVAL;
    }
    }
    if (util_min != -1 && util_max != -1 && util_min > util_max) {
    return -EINVAL;
    }
//
// We have valid uclamp attributes; make sure uclamp is enabled.
//
// We need to do that here, because enabling static branches is a
// blocking operation which obviously cannot be done while holding
// scheduler locks.
//
    sched_uclamp_enable();
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn uclamp_reset(attr: *mut sched_attr, clamp_id: uclamp_id, uc_se: *mut uclamp_se) -> bool {
// Reset on sched class change for a non user-defined clamp value.
    if (likely(!(attr.sched_flags & SCHED_FLAG_UTIL_CLAMP)) &&
    !uc_se.user_defined) {
    return true;
    }
// Reset on sched_util_{min,max} == -1.
    if (clamp_id == UCLAMP_MIN &&
    attr.sched_flags & SCHED_FLAG_UTIL_CLAMP_MIN &&
    attr.sched_util_min == -1) {
    return true;
    }
    if (clamp_id == UCLAMP_MAX &&
    attr.sched_flags & SCHED_FLAG_UTIL_CLAMP_MAX &&
    attr.sched_util_max == -1) {
    return true;
    }
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn __setscheduler_uclamp(p: *mut task_struct, attr: *mut sched_attr) {
    enum uclamp_id clamp_id;
    for_each_clamp_id(clamp_id) {
    let mut uc_se = &p.uclamp_req[clamp_id];
    let mut value = 0;
    if (!uclamp_reset(attr, clamp_id, uc_se)) {
    continue;
    }
//
// RT by default have a 100% boost value that could be modified
// at runtime.
//
    if (unlikely(rt_task(p) && clamp_id == UCLAMP_MIN)) {
    value = sysctl_sched_uclamp_util_min_rt_default;
    }
    else {
    value = uclamp_none(clamp_id);
    }
    uclamp_se_set(uc_se, value, false);
    }
    if (likely(!(attr.sched_flags & SCHED_FLAG_UTIL_CLAMP))) {
    return;
    }
    if (attr.sched_flags & SCHED_FLAG_UTIL_CLAMP_MIN &&
    attr.sched_util_min != -1) {
    uclamp_se_set(&p.uclamp_req[UCLAMP_MIN],
    attr.sched_util_min, true);
    }
    if (attr.sched_flags & SCHED_FLAG_UTIL_CLAMP_MAX &&
    attr.sched_util_max != -1) {
    uclamp_se_set(&p.uclamp_req[UCLAMP_MAX],
    attr.sched_util_max, true);
    }
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: uclamp_validate
pub unsafe extern "C" fn uclamp_validate_dup(p: *mut task_struct, attr: *mut sched_attr) -> c_int {
    return -EOPNOTSUPP;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: __setscheduler_uclamp
pub unsafe extern "C" fn __setscheduler_uclamp_dup(p: *mut task_struct, attr: *mut sched_attr) { }

//
// Allow unprivileged RT tasks to decrease priority.
// Only issue a capable test if needed and only once to avoid an audit
// event on permitted non-privileged operations:
//
#[no_mangle]
pub unsafe extern "C" fn user_check_sched_setscheduler(p: *mut task_struct, attr: *mut sched_attr, policy: c_int, reset_on_fork: c_int) -> c_int {
    if (fair_policy(policy)) {
    if (attr.sched_nice < task_nice(p) &&
    !is_nice_reduction(p, attr.sched_nice)) {
// goto;
    }
    }
    if (rt_policy(policy)) {
pub static mut rlim_rtprio: c_ulong = 0;
// Can't set/change the rt policy:
    if (policy != p.policy && !rlim_rtprio) {
// goto;
    }
// Can't increase priority:
    if (attr.sched_priority > p.rt_priority &&
    attr.sched_priority > rlim_rtprio) {
// goto;
    }
    }
//
// Can't set/change SCHED_DEADLINE policy at all for now
// (safest behavior); in the future we would like to allow
// unprivileged DL tasks to increase their relative deadline
// or reduce their runtime (both ways reducing utilization)
//
    if (dl_policy(policy)) {
// goto;
    }
//
// Treat SCHED_IDLE as nice 20. Only allow a switch to
// SCHED_NORMAL if the RLIMIT_NICE would normally permit it.
//
    if (task_has_idle_policy(p) && !idle_policy(policy)) {
    if (!is_nice_reduction(p, task_nice(p))) {
// goto;
    }
    }
// Can't change other user's priorities:
    if (!check_same_owner(p)) {
// goto;
    }
// Normal users shall not reset the sched_reset_on_fork flag:
    if (p.sched_reset_on_fork && !reset_on_fork) {
// goto;
    }
    return 0;
// label;
    if (!capable(CAP_SYS_NICE)) {
    return -EPERM;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn __sched_setscheduler(p: *mut task_struct, attr: *mut sched_attr, user: bool, pi: bool) -> c_int {
pub static mut oldpolicy: c_int = 0;
    let mut retval = 0;
    let mut oldprio = 0;
    let mut newprio = 0;
    let mut prev_class = core::ptr::null_mut();
    let mut next_class = core::ptr::null_mut();
pub static mut head: *mut c_void = core::ptr::null_mut();
pub static mut rf: usize = 0;
    let mut reset_on_fork = 0;
pub static mut queue_flags: c_int = 0;
pub static mut rq: *mut c_void = core::ptr::null_mut();
pub static mut cpuset_locked: bool = false;
// The pi code expects interrupts enabled
    BUG_ON!(pi && in_interrupt());
// label;
// Double check policy once rq lock held:
    if (policy < 0) {
    reset_on_fork = p.sched_reset_on_fork;
    policy = oldpolicy = p.policy;
    } else {
    reset_on_fork = !!(attr.sched_flags & SCHED_FLAG_RESET_ON_FORK);
    if (!valid_policy(policy)) {
    return -EINVAL;
    }
    }
    if (attr.sched_flags & ~(SCHED_FLAG_ALL | SCHED_FLAG_SUGOV)) {
    return -EINVAL;
    }
//
// Valid priorities for SCHED_FIFO and SCHED_RR are
// 1..MAX_RT_PRIO-1, valid priority for SCHED_NORMAL,
// SCHED_BATCH and SCHED_IDLE is 0.
//
    if (attr.sched_priority > MAX_RT_PRIO-1) {
    return -EINVAL;
    }
    if ((dl_policy(policy) && !__checkparam_dl(attr)) ||
    (rt_policy(policy) != (attr.sched_priority != 0))) {
    return -EINVAL;
    }
    if (user) {
    retval = user_check_sched_setscheduler(p, attr, policy, reset_on_fork);
    if (retval) {
    return retval;
    }
    if (attr.sched_flags & SCHED_FLAG_SUGOV) {
    return -EINVAL;
    }
    retval = security_task_setscheduler(p);
    if (retval) {
    return retval;
    }
    }
// Update task specific "requested" clamps
    if (attr.sched_flags & SCHED_FLAG_UTIL_CLAMP) {
    retval = uclamp_validate(p, attr);
    if (retval) {
    return retval;
    }
    }
//
// SCHED_DEADLINE bandwidth accounting relies on stable cpusets
// information.
//
    if (dl_policy(policy) || dl_policy(p.policy)) {
    cpuset_locked = true;
    cpuset_lock();
    }
//
// Make sure no PI-waiters arrive (or leave) while we are
// changing the priority of the task:
//
// To be able to change p->policy safely, the appropriate
// runqueue lock must be held.
//
    rq = task_rq_lock(p, &rf);
    update_rq_clock(rq);
//
// Changing the policy of the stop threads its a very bad idea:
//
    if (p == rq.stop) {
    retval = -EINVAL;
// goto;
    }
    retval = scx_check_setscheduler(p, policy);
    if (retval) {
// goto;
    }
//
// If not changing anything there's no need to proceed further,
// but store a possible modification of reset_on_fork.
//
    if (unlikely(policy == p.policy)) {
    if (fair_policy(policy) &&
    (attr.sched_nice != task_nice(p) ||
    (attr.sched_runtime != p.se.slice))) {
// goto;
    }
    if (rt_policy(policy) && attr.sched_priority != p.rt_priority) {
// goto;
    }
    if (dl_policy(policy) && dl_param_changed(p, attr)) {
// goto;
    }
    if (attr.sched_flags & SCHED_FLAG_UTIL_CLAMP) {
// goto;
    }
    p.sched_reset_on_fork = reset_on_fork;
    retval = 0;
// goto;
    }
// label;
    if (user) {

//
// Do not allow real-time tasks into groups that have no runtime
// assigned.
//
    if (rt_group_sched_enabled() &&
    rt_bandwidth_enabled() && rt_policy(policy) &&
    task_group(p).rt_bandwidth.rt_runtime == 0 &&
    !task_group_is_autogroup(task_group(p))) {
    retval = -EPERM;
// goto;
    }

    if (dl_bandwidth_enabled() && dl_policy(policy) &&
    !(attr.sched_flags & SCHED_FLAG_SUGOV)) {
    let mut span = rq.rd.span;
//
// Don't allow tasks with an affinity mask smaller than
// the entire root_domain to become SCHED_DEADLINE. We
// will also fail if there's no bandwidth available.
//
    if (!cpumask_subset(span, p.cpus_ptr) ||
    rq.rd.dl_bw.bw == 0) {
    retval = -EPERM;
// goto;
    }
    }
    }
// Re-check policy now with rq lock held:
    if (unlikely(oldpolicy != -1 && oldpolicy != p.policy)) {
    policy = oldpolicy = -1;
    task_rq_unlock(rq, p, &rf);
    if (cpuset_locked) {
    cpuset_unlock();
    }
// goto;
    }
//
// If setscheduling to SCHED_DEADLINE (or changing the parameters
// of a SCHED_DEADLINE task) we need to check if enough bandwidth
// is available.
//
    if ((dl_policy(policy) || dl_task(p)) && sched_dl_overflow(p, policy, attr)) {
    retval = -EBUSY;
// goto;
    }
    p.sched_reset_on_fork = reset_on_fork;
    oldprio = p.prio;
    newprio = __normal_prio(policy, attr.sched_priority, attr.sched_nice);
    if (pi) {
//
// Take priority boosted tasks into account. If the new
// effective priority is unchanged, we just store the new
// normal parameters and do not touch the scheduler class and
// the runqueue. This will be done when the task deboost
// itself.
//
    newprio = rt_effective_prio(p, newprio);
    if (newprio == oldprio && !dl_prio(newprio)) {
    queue_flags &= ~DEQUEUE_MOVE;
    }
    }
    prev_class = p.sched_class;
    next_class = __setscheduler_class(policy, newprio);
    if (prev_class != next_class) {
    queue_flags |= DEQUEUE_CLASS;
    }
    scoped_guard (sched_change, p, queue_flags) {
    if (!(attr.sched_flags & SCHED_FLAG_KEEP_PARAMS)) {
    __setscheduler_params(p, attr);
    p.sched_class = next_class;
    p.prio = newprio;
    __setscheduler_dl_pi(newprio, policy, p, scope);
    }
    __setscheduler_uclamp(p, attr);
    if (scope.queued) {
//
// We enqueue to tail when the priority of a task is
// increased (user space view).
//
    if (oldprio < p.prio) {
    scope.flags |= ENQUEUE_HEAD;
    }
    }
    }
// Avoid rq from going away on us:
    preempt_disable();
    head = splice_balance_callbacks(rq);
    task_rq_unlock(rq, p, &rf);
    if (pi) {
    if (cpuset_locked) {
    cpuset_unlock();
    }
    rt_mutex_adjust_pi(p);
    }
// Run balance callbacks after we've adjusted the PI chain:
    balance_callbacks(rq, head);
    preempt_enable();
    return 0;
// label;
    task_rq_unlock(rq, p, &rf);
    if (cpuset_locked) {
    cpuset_unlock();
    }
    return retval;
    }
#[no_mangle]
pub unsafe extern "C" fn _sched_setscheduler(p: *mut task_struct, policy: c_int, param: *mut sched_param, check: bool) -> c_int {
pub static mut sched_attr: usize = 0;
    if (p.se.custom_slice) {
    attr.sched_runtime = p.se.slice;
    }
// Fixup the legacy SCHED_RESET_ON_FORK hack.
    if ((policy != SETPARAM_POLICY) && (policy & SCHED_RESET_ON_FORK)) {
    attr.sched_flags |= SCHED_FLAG_RESET_ON_FORK;
    policy &= ~SCHED_RESET_ON_FORK;
    attr.sched_policy = policy;
    }
    return __sched_setscheduler(p, &attr, check, true);
    }
//
// sched_setscheduler - change the scheduling policy and/or RT priority of a thread.
// @p: the task in question.
// @policy: new policy.
// @param: structure containing the new RT priority.
//
// Use sched_set_fifo(), read its comment.
//
// Return: 0 on success. An error code otherwise.
//
// NOTE that the task may be already dead.
//
#[no_mangle]
pub unsafe extern "C" fn sched_setscheduler(p: *mut task_struct, policy: c_int, param: *mut sched_param) -> c_int {
    return _sched_setscheduler(p, policy, param, true);
    }
#[no_mangle]
pub unsafe extern "C" fn sched_setattr(p: *mut task_struct, attr: *const sched_attr) -> c_int {
    return __sched_setscheduler(p, attr, true, true);
    }
#[no_mangle]
pub unsafe extern "C" fn sched_setattr_nocheck(p: *mut task_struct, attr: *const sched_attr) -> c_int {
    return __sched_setscheduler(p, attr, false, true);
    }
    EXPORT_SYMBOL_GPL(sched_setattr_nocheck);
//
// sched_setscheduler_nocheck - change the scheduling policy and/or RT priority of a thread from kernel-space.
// @p: the task in question.
// @policy: new policy.
// @param: structure containing the new RT priority.
//
// Just like sched_setscheduler, only don't bother checking if the
// current context has permission.  For example, this is needed in
// stop_machine(): we create temporary high priority worker threads,
// but our caller might not have that capability.
//
// Return: 0 on success. An error code otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn sched_setscheduler_nocheck(p: *mut task_struct, policy: c_int, param: *mut sched_param) -> c_int {
    return _sched_setscheduler(p, policy, param, false);
    }
//
// SCHED_FIFO is a broken scheduler model; that is, it is fundamentally
// incapable of resource management, which is the one thing an OS really should
// be doing.
//
// This is of course the reason it is limited to privileged users only.
//
// Worse still; it is fundamentally impossible to compose static priority
// workloads. You cannot take two correctly working static prio workloads
// and smash them together and still expect them to work.
//
// For this reason 'all' FIFO tasks the kernel creates are basically at:
//
// MAX_RT_PRIO / 2
//
// The administrator _MUST_ configure the system, the kernel simply doesn't
// know enough information to make a sensible choice.
//
#[no_mangle]
pub unsafe extern "C" fn sched_set_fifo(p: *mut task_struct) {
pub static mut sp: sched_param = 0;
    WARN_ON_ONCE!(sched_setscheduler_nocheck(p, SCHED_FIFO, &sp) != 0);
    }
    EXPORT_SYMBOL_GPL(sched_set_fifo);
//
// For when you don't much care about FIFO, but want to be above SCHED_NORMAL.
//
#[no_mangle]
pub unsafe extern "C" fn sched_set_fifo_low(p: *mut task_struct) {
pub static mut sp: sched_param = 0;
    WARN_ON_ONCE!(sched_setscheduler_nocheck(p, SCHED_FIFO, &sp) != 0);
    }
    EXPORT_SYMBOL_GPL(sched_set_fifo_low);
//
// Used when the primary interrupt handler is forced into a thread, in addition
// to the (always threaded) secondary handler.  The secondary handler gets a
// slightly lower priority so that the primary handler can preempt it, thereby
// emulating the behavior of a non-PREEMPT_RT system where the primary handler
// runs in hard interrupt context.
//
#[no_mangle]
pub unsafe extern "C" fn sched_set_fifo_secondary(p: *mut task_struct) {
pub static mut sp: sched_param = 0;
    WARN_ON_ONCE!(sched_setscheduler_nocheck(p, SCHED_FIFO, &sp) != 0);
    }
#[no_mangle]
pub unsafe extern "C" fn sched_set_normal(p: *mut task_struct, nice: c_int) {
pub static mut sched_attr: usize = 0;
    WARN_ON_ONCE!(sched_setattr_nocheck(p, &attr) != 0);
    }
    EXPORT_SYMBOL_GPL(sched_set_normal);
#[no_mangle]
pub unsafe extern "C" fn do_sched_setscheduler(pid: pid_t, policy: c_int, param: *mut sched_param) -> c_int {
pub static mut lparam: usize = 0;
    if (unlikely(!param || pid < 0)) {
    return -EINVAL;
    }
    if (copy_from_user(&lparam, param, sizeof!(sched_param))) {
    return -EFAULT;
    }
    CLASS(find_get_task, p)(pid);
    if (!p) {
    return -ESRCH;
    }
    return sched_setscheduler(p, policy, &lparam);
    }
//
// Mimics kernel/events/core.c perf_copy_attr().
//
#[no_mangle]
unsafe extern "C" fn sched_copy_attr(uattr: *mut sched_attr , attr: *mut sched_attr) -> c_int {
    let mut size = 0;
    let mut ret = 0;
// Zero the full structure, so that a short copy will be nice:
    memset(attr, 0, sizeof!(*attr));
    ret = get_user(size, &uattr.size);
    if (ret) {
    return ret;
    }
// ABI compatibility quirk:
    if (!size) {
    size = SCHED_ATTR_SIZE_VER0;
    }
    if (size < SCHED_ATTR_SIZE_VER0 || size > PAGE_SIZE) {
// goto;
    }
    ret = copy_struct_from_user(attr, sizeof!(*attr), uattr, size);
    if (ret) {
    if (ret == -E2BIG) {
// goto;
    }
    return ret;
    }
    if ((attr.sched_flags & SCHED_FLAG_UTIL_CLAMP) &&
    size < SCHED_ATTR_SIZE_VER1) {
    return -EINVAL;
    }
//
// XXX: Do we want to be lenient like existing syscalls; or do we want
// to be strict and return an error on out-of-bounds values?
//
    attr.sched_nice = clamp(attr.sched_nice, MIN_NICE, MAX_NICE);
    return 0;
// label;
    put_user(sizeof!(*attr), &uattr.size);
    return -E2BIG;
    }
#[no_mangle]
unsafe extern "C" fn get_params(p: *mut task_struct, attr: *mut sched_attr, flags: c_uint) {
    if (task_has_dl_policy(p)) {
    __getparam_dl(p, attr, flags);
    } else if (task_has_rt_policy(p)) {
    attr.sched_priority = p.rt_priority;
    } else {
    attr.sched_nice = task_nice(p);
    attr.sched_runtime = p.se.slice;
    }
    }
//
// sys_sched_setscheduler - set/change the scheduler policy and RT priority
// @pid: the pid in question.
// @policy: new policy.
// @param: structure containing the new RT priority.
//
// Return: 0 on success. An error code otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn sys_sched_setscheduler(pid: usize, policy: usize, param: usize) -> c_long {
    if (policy < 0) {
    return -EINVAL;
    }
    return do_sched_setscheduler(pid, policy, param);
    }
//
// sys_sched_setparam - set/change the RT priority of a thread
// @pid: the pid in question.
// @param: structure containing the new RT priority.
//
// Return: 0 on success. An error code otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn sys_sched_setparam(pid: usize, param: usize) -> c_long {
    return do_sched_setscheduler(pid, SETPARAM_POLICY, param);
    }
//
// sys_sched_setattr - same as above, but with extended sched_attr
// @pid: the pid in question.
// @uattr: structure containing the extended parameters.
// @flags: for future extension.
//
#[no_mangle]
pub unsafe extern "C" fn sys_sched_setattr(pid: usize, uattr: usize, flags: usize) -> c_long {
pub static mut attr: usize = 0;
    let mut retval = 0;
    if (unlikely(!uattr || pid < 0 || flags)) {
    return -EINVAL;
    }
    retval = sched_copy_attr(uattr, &attr);
    if (retval) {
    return retval;
    }
    if ((int)attr.sched_policy < 0) {
    return -EINVAL;
    }
    if (attr.sched_flags & SCHED_FLAG_KEEP_POLICY) {
    attr.sched_policy = SETPARAM_POLICY;
    }
    CLASS(find_get_task, p)(pid);
    if (!p) {
    return -ESRCH;
    }
    if (attr.sched_flags & SCHED_FLAG_KEEP_PARAMS) {
    get_params(p, &attr, 0);
    }
    return sched_setattr(p, &attr);
    }
//
// sys_sched_getscheduler - get the policy (scheduling class) of a thread
// @pid: the pid in question.
//
// Return: On success, the policy of the thread. Otherwise, a negative error
// code.
//
#[no_mangle]
pub unsafe extern "C" fn sys_sched_getscheduler(pid: usize) -> c_long {
pub static mut p: *mut c_void = core::ptr::null_mut();
    let mut retval = 0;
    if (pid < 0) {
    return -EINVAL;
    }
    guard(rcu)();
    p = find_process_by_pid(pid);
    if (!p) {
    return -ESRCH;
    }
    retval = security_task_getscheduler(p);
    if (!retval) {
    retval = p.policy;
    if (p.sched_reset_on_fork) {
    retval |= SCHED_RESET_ON_FORK;
    }
    }
    return retval;
    }
//
// sys_sched_getparam - get the RT priority of a thread
// @pid: the pid in question.
// @param: structure containing the RT priority.
//
// Return: On success, 0 and the RT priority is in @param. Otherwise, an error
// code.
//
#[no_mangle]
pub unsafe extern "C" fn sys_sched_getparam(pid: usize, param: usize) -> c_long {
pub static mut lp: sched_param = 0;
pub static mut p: *mut c_void = core::ptr::null_mut();
    let mut retval = 0;
    if (unlikely(!param || pid < 0)) {
    return -EINVAL;
    }
    scoped_guard (rcu) {
    p = find_process_by_pid(pid);
    if (!p) {
    return -ESRCH;
    }
    retval = security_task_getscheduler(p);
    if (retval) {
    return retval;
    }
    if (task_has_rt_policy(p)) {
    lp.sched_priority = p.rt_priority;
    }
    }
//
// This one might sleep, we cannot do it with a spinlock held ...
//
    return copy_to_user(param, &lp, sizeof!(*param)) ? -EFAULT : 0;
    }
//
// sys_sched_getattr - similar to sched_getparam, but with sched_attr
// @pid: the pid in question.
// @uattr: structure containing the extended parameters.
// @usize: sizeof!(attr) for fwd/bwd comp.
// @flags: for future extension.
//
#[no_mangle]
pub unsafe extern "C" fn sys_sched_getattr(pid: usize, uattr: usize, usize: usize, flags: usize) -> c_long {
pub static mut kattr: sched_attr = 0;
pub static mut p: *mut c_void = core::ptr::null_mut();
    let mut retval = 0;
    if (unlikely(!uattr || pid < 0 || usize > PAGE_SIZE ||
    usize < SCHED_ATTR_SIZE_VER0)) {
    return -EINVAL;
    }
    scoped_guard (rcu) {
    p = find_process_by_pid(pid);
    if (!p) {
    return -ESRCH;
    }
    if (flags) {
    if (!task_has_dl_policy(p) ||
    flags != SCHED_GETATTR_FLAG_DL_DYNAMIC) {
    return -EINVAL;
    }
    }
    retval = security_task_getscheduler(p);
    if (retval) {
    return retval;
    }
    kattr.sched_policy = p.policy;
    if (p.sched_reset_on_fork) {
    kattr.sched_flags |= SCHED_FLAG_RESET_ON_FORK;
    }
    get_params(p, &kattr, flags);
    kattr.sched_flags &= SCHED_FLAG_ALL;

//
// This could race with another potential updater, but this is fine
// because it'll correctly read the old or the new value. We don't need
// to guarantee who wins the race as long as it doesn't return garbage.
//
    kattr.sched_util_min = p.uclamp_req[UCLAMP_MIN].value;
    kattr.sched_util_max = p.uclamp_req[UCLAMP_MAX].value;

    }
    kattr.size = min(usize, sizeof!(kattr));
    return copy_struct_to_user(uattr, usize, &kattr, sizeof!(kattr), core::ptr::null_mut());
    }
#[no_mangle]
pub unsafe extern "C" fn dl_task_check_affinity(p: *mut task_struct, mask: *const cpumask) -> c_int {
//
// If the task isn't a deadline task or admission control is
// disabled then we don't care about affinity changes.
//
    if (!task_has_dl_policy(p) || !dl_bandwidth_enabled()) {
    return 0;
    }
//
// The special/sugov task isn't part of regular bandwidth/admission
// control so let userspace change affinities.
//
    if (dl_entity_is_special(&p.dl)) {
    return 0;
    }
//
// Since bandwidth control happens on root_domain basis,
// if admission test is enabled, we only admit -deadline
// tasks allowed to run on all the CPUs in the task's
// root_domain.
//
    guard(rcu)();
    if (!cpumask_subset(task_rq(p).rd.span, mask)) {
    return -EBUSY;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn __sched_setaffinity(p: *mut task_struct, ctx: *mut affinity_context) -> c_int {
    let mut retval = 0;
    cpumask_var_t cpus_allowed, new_mask;
    if (!alloc_cpumask_var(&cpus_allowed, GFP_KERNEL)) {
    return -ENOMEM;
    }
    if (!alloc_cpumask_var(&new_mask, GFP_KERNEL)) {
    retval = -ENOMEM;
// goto;
    }
    cpuset_cpus_allowed(p, cpus_allowed);
    cpumask_and(new_mask, ctx.new_mask, cpus_allowed);
    ctx.new_mask = new_mask;
    ctx.flags |= SCA_CHECK;
    retval = dl_task_check_affinity(p, new_mask);
    if (retval) {
// goto;
    }
    retval = __set_cpus_allowed_ptr(p, ctx);
    if (retval) {
// goto;
    }
    cpuset_cpus_allowed(p, cpus_allowed);
    if (!cpumask_subset(new_mask, cpus_allowed)) {
//
// We must have raced with a concurrent cpuset update.
// Just reset the cpumask to the cpuset's cpus_allowed.
//
    cpumask_copy(new_mask, cpus_allowed);
//
// If SCA_USER is set, a 2nd call to __set_cpus_allowed_ptr()
// will restore the previous user_cpus_ptr value.
//
// In the unlikely event a previous user_cpus_ptr exists,
// we need to further restrict the mask to what is allowed
// by that old user_cpus_ptr.
//
    if (unlikely((ctx.flags & SCA_USER) && ctx.user_mask)) {
    let mut empty = !cpumask_and(new_mask, new_mask,
    ctx.user_mask);
    if (empty) {
    cpumask_copy(new_mask, cpus_allowed);
    }
    }
    __set_cpus_allowed_ptr(p, ctx);
    retval = -EINVAL;
    }
// label;
    free_cpumask_var(new_mask);
// label;
    free_cpumask_var(cpus_allowed);
    return retval;
    }
#[no_mangle]
pub unsafe extern "C" fn sched_setaffinity(pid: pid_t, in_mask: *const cpumask) -> c_long {
pub static mut ac: usize = 0;
pub static mut user_mask: *mut c_void = core::ptr::null_mut();
    let mut retval = 0;
    CLASS(find_get_task, p)(pid);
    if (!p) {
    return -ESRCH;
    }
    if (p.flags & PF_NO_SETAFFINITY) {
    return -EINVAL;
    }
    if (!check_same_owner(p)) {
    guard(rcu)();
    if (!ns_capable(__task_cred(p).user_ns, CAP_SYS_NICE)) {
    return -EPERM;
    }
    }
    retval = security_task_setscheduler(p);
    if (retval) {
    return retval;
    }
//
// With non-SMP configs, user_cpus_ptr/user_mask isn't used and
// alloc_user_cpus_ptr() returns NULL.
//
    user_mask = alloc_user_cpus_ptr(NUMA_NO_NODE);
    if (user_mask) {
    cpumask_copy(user_mask, in_mask);
    } else {
    return -ENOMEM;
    }
    ac = (affinity_context){
    .new_mask  = in_mask,
    .user_mask = user_mask,
    .flags     = SCA_USER,
    };
    retval = __sched_setaffinity(p, &ac);
    kfree(ac.user_mask);
    return retval;
    }
#[no_mangle]
pub unsafe extern "C" fn get_user_cpu_mask(user_mask_ptr: *mut c_ulong, len: c_uint, new_mask: *mut cpumask) -> c_int {
    if (len < cpumask_size()) {
    cpumask_clear(new_mask);
    }

    else if (len > cpumask_size()) {
    len = cpumask_size();
    }
    return copy_from_user(new_mask, user_mask_ptr, len) ? -EFAULT : 0;
    }
//
// sys_sched_setaffinity - set the CPU affinity of a process
// @pid: pid of the process
// @len: length in bytes of the bitmask pointed to by user_mask_ptr
// @user_mask_ptr: user-space pointer to the new CPU mask
//
// Return: 0 on success. An error code otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn sys_sched_setaffinity(pid: usize, len: usize, user_mask_ptr: usize) -> c_long {
    let mut new_mask;
    let mut retval = 0;
    if (!alloc_cpumask_var(&new_mask, GFP_KERNEL)) {
    return -ENOMEM;
    }
    retval = get_user_cpu_mask(user_mask_ptr, len, new_mask);
    if (retval == 0) {
    retval = sched_setaffinity(pid, new_mask);
    }
    free_cpumask_var(new_mask);
    return retval;
    }
#[no_mangle]
pub unsafe extern "C" fn sched_getaffinity(pid: pid_t, mask: *mut cpumask) -> c_long {
pub static mut p: *mut c_void = core::ptr::null_mut();
    let mut retval = 0;
    guard(rcu)();
    p = find_process_by_pid(pid);
    if (!p) {
    return -ESRCH;
    }
    retval = security_task_getscheduler(p);
    if (retval) {
    return retval;
    }
    guard(raw_spinlock_irqsave)(&p.pi_lock);
    cpumask_and(mask, &p.cpus_mask, cpu_active_mask);
    return 0;
    }
//
// sys_sched_getaffinity - get the CPU affinity of a process
// @pid: pid of the process
// @len: length in bytes of the bitmask pointed to by user_mask_ptr
// @user_mask_ptr: user-space pointer to hold the current CPU mask
//
// Return: size of CPU mask copied to user_mask_ptr on success. An
// error code otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn sys_sched_getaffinity(pid: usize, len: usize, user_mask_ptr: usize) -> c_long {
    let mut ret = 0;
    let mut mask;
    if ((len * BITS_PER_BYTE) < nr_cpu_ids) {
    return -EINVAL;
    }
    if (len & (sizeof!(unsigned long)-1)) {
    return -EINVAL;
    }
    if (!zalloc_cpumask_var(&mask, GFP_KERNEL)) {
    return -ENOMEM;
    }
    ret = sched_getaffinity(pid, mask);
    if (ret == 0) {
pub static mut retlen: c_uint = 0;
    if (copy_to_user(user_mask_ptr, cpumask_bits(mask), retlen)) {
    ret = -EFAULT;
    }
    else {
    ret = retlen;
    }
    }
    free_cpumask_var(mask);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn do_sched_yield() {
pub static mut rf: usize = 0;
pub static mut rq: *mut c_void = core::ptr::null_mut();
    rq = this_rq_lock_irq(&rf);
    schedstat_inc(rq.yld_count);
    rq.donor.sched_class.yield_task(rq);
    preempt_disable();
    rq_unlock_irq(rq, &rf);
    sched_preempt_enable_no_resched();
    schedule();
    }
//
// sys_sched_yield - yield the current processor to other threads.
//
// This function yields the current CPU to other tasks. If there are no
// other threads running on this CPU then this function will return.
//
// Return: 0.
//
#[no_mangle]
pub unsafe extern "C" fn sys_sched_yield() -> c_long {
    do_sched_yield();
    return 0;
    }
//
// yield - yield the current processor to other threads.
//
// Do not ever use this function, there's a 99% chance you're doing it wrong.
//
// The scheduler is at all times free to pick the calling task as the most
// eligible task to run, if removing the yield() call from your code breaks
// it, it's already broken.
//
// Typical broken usage is:
//
// while (!event)
// yield();
//
// where one assumes that yield() will let 'the other' process run that will
// make event true. If the current task is a SCHED_FIFO task that will never
// happen. Never use yield() as a progress guarantee!!
//
// If you want to use yield() to wait for something, use wait_event().
// If you want to use yield() to be 'nice' for others, use cond_resched().
// If you still want to use yield(), do not!
//
#[no_mangle]
pub unsafe extern "C" fn yield() -> void __sched {
    set_current_state(TASK_RUNNING);
    do_sched_yield();
    }
    EXPORT_SYMBOL(yield);
//
// yield_to - yield the current processor to another thread in
// your thread group, or accelerate that thread toward the
// processor it's on.
// @p: target task
// @preempt: whether task preemption is allowed or not
//
// It's the caller's job to ensure that the target task struct
// can't go away on us before we can do any checks.
//
// Return:
// true (>0) if we indeed boosted the target task.
// false (0) if we failed to boost the target.
// -ESRCH if there's no task to yield to.
//
#[no_mangle]
pub unsafe extern "C" fn yield_to(p: *mut task_struct, preempt: bool) -> int __sched {
pub static mut curr: *mut c_void = core::ptr::null_mut();
    let mut rq = core::ptr::null_mut();
    let mut p_rq = core::ptr::null_mut();
pub static mut yielded: c_int = 0;
    scoped_guard (raw_spinlock_irqsave, &p.pi_lock) {
    rq = this_rq();
    curr = rq.donor;
// label;
    p_rq = task_rq(p);
//
// If we're the only runnable task on the rq and target rq also
// has only one task, there's absolutely no point in yielding.
//
    if (rq.nr_running == 1 && p_rq.nr_running == 1) {
    return -ESRCH;
    }
    guard(double_rq_lock)(rq, p_rq);
    if (task_rq(p) != p_rq) {
// goto;
    }
    if (!curr.sched_class.yield_to_task) {
    return 0;
    }
    if (curr.sched_class != p.sched_class) {
    return 0;
    }
    if (task_on_cpu(p_rq, p) || !task_is_running(p)) {
    return 0;
    }
    yielded = curr.sched_class.yield_to_task(rq, p);
    if (yielded) {
    schedstat_inc(rq.yld_count);
//
// Make p's CPU reschedule; pick_next_entity
// takes care of fairness.
//
    if (preempt && rq != p_rq) {
    resched_curr(p_rq);
    }
    }
    }
    if (yielded) {
    schedule();
    }
    return yielded;
    }
    EXPORT_SYMBOL_GPL(yield_to);
//
// sys_sched_get_priority_max - return maximum RT priority.
// @policy: scheduling class.
//
// Return: On success, this syscall returns the maximum
// rt_priority that can be used by a given scheduling class.
// On failure, a negative error code is returned.
//
#[no_mangle]
pub unsafe extern "C" fn sys_sched_get_priority_max(policy: usize) -> c_long {
pub static mut ret: c_int = 0;
    match (policy) {
    SCHED_FIFO => {
    }
    SCHED_RR => {
    ret = MAX_RT_PRIO-1;
    // break;
    }
    SCHED_DEADLINE => {
    }
    SCHED_NORMAL => {
    }
    SCHED_BATCH => {
    }
    SCHED_IDLE => {
    }
    SCHED_EXT => {
    ret = 0;
    // break;
    }
    }
    return ret;
    }
//
// sys_sched_get_priority_min - return minimum RT priority.
// @policy: scheduling class.
//
// Return: On success, this syscall returns the minimum
// rt_priority that can be used by a given scheduling class.
// On failure, a negative error code is returned.
//
#[no_mangle]
pub unsafe extern "C" fn sys_sched_get_priority_min(policy: usize) -> c_long {
pub static mut ret: c_int = 0;
    match (policy) {
    SCHED_FIFO => {
    }
    SCHED_RR => {
    ret = 1;
    // break;
    }
    SCHED_DEADLINE => {
    }
    SCHED_NORMAL => {
    }
    SCHED_BATCH => {
    }
    SCHED_IDLE => {
    }
    SCHED_EXT => {
    ret = 0;
    }
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sched_rr_get_interval(pid: pid_t, t: *mut timespec64) -> c_int {
pub static mut time_slice: c_uint = 0;
    let mut retval = 0;
    if (pid < 0) {
    return -EINVAL;
    }
    scoped_guard (rcu) {
    let mut p = find_process_by_pid(pid);
    if (!p) {
    return -ESRCH;
    }
    retval = security_task_getscheduler(p);
    if (retval) {
    return retval;
    }
    scoped_guard (task_rq_lock, p) {
    let mut rq = scope.rq;
    if (p.sched_class.get_rr_interval) {
    time_slice = p.sched_class.get_rr_interval(rq, p);
    }
    }
    }
    jiffies_to_timespec64(time_slice, t);
    return 0;
    }
//
// sys_sched_rr_get_interval - return the default time-slice of a process.
// @pid: pid of the process.
// @interval: userspace pointer to the time-slice value.
//
// this syscall writes the default time-slice value of a given process
// into the user-space timespec buffer. A value of '0' means infinity.
//
// Return: On success, 0 and the time-slice is in @interval. Otherwise,
// an error code.
//
#[no_mangle]
pub unsafe extern "C" fn sys_sched_rr_get_interval(pid: usize, interval: usize) -> c_long {
pub static mut t: usize = 0;
pub static mut retval: c_int = 0;
    if (retval == 0) {
    retval = put_timespec64(&t, interval);
    }
    return retval;
    }

#[no_mangle]
pub unsafe extern "C" fn sys_sched_rr_get_interval_time32(pid: usize, interval: usize) -> c_long {
pub static mut t: usize = 0;
pub static mut retval: c_int = 0;
    if (retval == 0) {
    retval = put_old_timespec32(&t, interval);
    }
    return retval;
    }