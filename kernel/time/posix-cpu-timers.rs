//! Automatically rewritten from C to Rust
//! Source: kernel/time/posix-cpu-timers.c
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
// Implement CPU time clocks for the POSIX clock interface.
//

// forward_decl: posix_cpu_timer_rearm;
#[no_mangle]
pub unsafe extern "C" fn posix_cputimers_group_init(pct: *mut posix_cputimers, cpu_limit: u64) {
    posix_cputimers_init(pct);
    if (cpu_limit != RLIM_INFINITY) {
    pct.bases[CPUCLOCK_PROF].nextevt = cpu_limit * NSEC_PER_SEC;
    pct.timers_active = true;
    }
    }
//
// Called after updating RLIMIT_CPU to run cpu timer and update
// tsk->signal->posix_cputimers.bases[clock].nextevt expiration cache if
// necessary. Needs siglock protection since other code may update the
// expiration cache as well.
//
// Returns 0 on success, -ESRCH on failure.  Can fail if the task is exiting and
// we cannot lock_task_sighand.  Cannot fail if task is current.
//
#[no_mangle]
pub unsafe extern "C" fn update_rlimit_cpu(task: *mut task_struct, rlim_new: c_ulong) -> c_int {
pub static mut nsecs: u64 = 0;
    let mut irq_fl = 0;
    if (!lock_task_sighand(task, &irq_fl)) {
    return -ESRCH;
    }
    set_process_cpu_timer(task, CPUCLOCK_PROF, &nsecs, core::ptr::null_mut());
    unlock_task_sighand(task, &irq_fl);
    return 0;
    }
//
// Functions for validating access to tasks.
//
#[no_mangle]
pub unsafe extern "C" fn pid_for_clock(clock: clockid_t, gettime: bool) -> *mut c_void {
pub static mut thread: bool = false;
pub static mut upid: pid_t = 0;
pub static mut pid: *mut c_void = core::ptr::null_mut();
    if (CPUCLOCK_WHICH(clock) >= CPUCLOCK_MAX) {
    return core::ptr::null_mut();
    }
//
// If the encoded PID is 0, then the timer is targeted at current
// or the process to which current belongs.
//
    if (upid == 0) {
    return thread ? task_pid(current) : task_tgid(current);
    }
    pid = find_vpid(upid);
    if (!pid) {
    return core::ptr::null_mut();
    }
    if (thread) {
    let mut tsk = pid_task(pid, PIDTYPE_PID);
    return (tsk && same_thread_group(tsk, current)) ? pid : core::ptr::null_mut();
    }
//
// For clock_gettime(PROCESS) allow finding the process by
// with the pid of the current task.  The code needs the tgid
// of the process so that pid_task(pid, PIDTYPE_TGID) can be
// used to find the process.
//
    if (gettime && (pid == task_pid(current))) {
    return task_tgid(current);
    }
//
// For processes require that pid identifies a process.
//
    return pid_has_task(pid, PIDTYPE_TGID) ? pid : core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn validate_clock_permissions(clock: clockid_t) -> c_int {
    let mut ret = 0;
    rcu_read_lock();
    ret = pid_for_clock(clock, false) ? 0 : -EINVAL;
    rcu_read_unlock();
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn clock_pid_type(clock: clockid_t) -> enum pid_type {
    return CPUCLOCK_PERTHREAD(clock) ? PIDTYPE_PID : PIDTYPE_TGID;
    }
#[no_mangle]
pub unsafe extern "C" fn cpu_timer_task_rcu(timer: *mut k_itimer) -> *mut c_void {
    return pid_task(timer.it.cpu.pid, clock_pid_type(timer.it_clock));
    }
//
// Update expiry time from increment, and increase overrun count,
// given the current clock sample.
//
#[no_mangle]
unsafe extern "C" fn bump_cpu_timer(timer: *mut k_itimer, now: u64) -> u64 {
    u64 delta, incr, expires = timer.it.cpu.node.expires;
    let mut i = 0;
    if (!timer.it_interval) {
    return expires;
    }
    if (now < expires) {
    return expires;
    }
    incr = timer.it_interval;
    delta = now + incr - expires;
// Don't use (incr*2 < delta), incr*2 might overflow.
    for (i = 0; incr < delta - incr; i++) {
    incr = incr << 1;
    }
    while (i >= 0) {
    if (delta < incr) {
    continue;
    }
    timer.it.cpu.node.expires += incr;
    timer.it_overrun += 1LL << i;
    delta -= incr;
    }
    return timer.it.cpu.node.expires;
    }
// Check whether all cache entries contain U64_MAX, i.e. eternal expiry time
#[no_mangle]
pub unsafe extern "C" fn expiry_cache_is_inactive(pct: *const posix_cputimers) -> bool {
    return !(~pct.bases[CPUCLOCK_PROF].nextevt |
    ~pct.bases[CPUCLOCK_VIRT].nextevt |
    ~pct.bases[CPUCLOCK_SCHED].nextevt);
    }
#[no_mangle]
pub unsafe extern "C" fn posix_cpu_clock_getres(which_clock: clockid_t, tp: *mut timespec64) -> c_int {
pub static mut error: c_int = 0;
    if (!error) {
    tp.tv_sec = 0;
    tp.tv_nsec = ((NSEC_PER_SEC + HZ - 1) / HZ);
    if (CPUCLOCK_WHICH(which_clock) == CPUCLOCK_SCHED) {
//
// If sched_clock is using a cycle counter, we
// don't have any idea of its true resolution
// exported, but it is much more than 1s/HZ.
//
    tp.tv_nsec = 1;
    }
    }
    return error;
    }
#[no_mangle]
pub unsafe extern "C" fn posix_cpu_clock_set(clock: clockid_t, tp: *mut timespec64) -> c_int {
pub static mut error: c_int = 0;
//
// You can never reset a CPU clock, but we check for other errors
// in the call before failing with EPERM.
//
    return error ? : -EPERM;
    }
//
// Sample a per-thread clock for the given task. clkid is validated.
//
#[no_mangle]
unsafe extern "C" fn cpu_clock_sample(clkid: clockid_t, p: *mut task_struct) -> u64 {
    u64 utime, stime;
    if (clkid == CPUCLOCK_SCHED) {
    return task_sched_runtime(p);
    }
    task_cputime(p, &utime, &stime);
    match (clkid) {
    CPUCLOCK_PROF => {
    return utime + stime;
    }
    CPUCLOCK_VIRT => {
    return utime;
    }
    _ => {
    WARN_ON_ONCE!(1);
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn store_samples(samples: *mut u64, stime: u64, utime: u64, rtime: u64) {
    samples[CPUCLOCK_PROF] = stime + utime;
    samples[CPUCLOCK_VIRT] = utime;
    samples[CPUCLOCK_SCHED] = rtime;
    }
#[no_mangle]
unsafe extern "C" fn task_sample_cputime(p: *mut task_struct, samples: *mut u64) {
    u64 stime, utime;
    task_cputime(p, &utime, &stime);
    store_samples(samples, stime, utime, p.se.sum_exec_runtime);
    }
#[no_mangle]
pub unsafe extern "C" fn proc_sample_cputime_atomic(at: *mut task_cputime_atomic, samples: *mut u64) {
    u64 stime, utime, rtime;
    utime = atomic64_read(&at.utime);
    stime = atomic64_read(&at.stime);
    rtime = atomic64_read(&at.sum_exec_runtime);
    store_samples(samples, stime, utime, rtime);
    }
//
// Set cputime to sum_cputime if sum_cputime > cputime. Use cmpxchg
// to avoid race conditions with concurrent updates to cputime.
//
#[no_mangle]
pub unsafe extern "C" fn __update_gt_cputime(cputime: *mut core::sync::atomic::AtomicI64, sum_cputime: u64) {
pub static mut curr_cputime: u64 = 0;
    do {
    if (sum_cputime <= curr_cputime) {
    return;
    }
    } while (!atomic64_try_cmpxchg(cputime, &curr_cputime, sum_cputime));
    }
#[no_mangle]
pub unsafe extern "C" fn update_gt_cputime(cputime_atomic: *mut task_cputime_atomic, sum: *mut task_cputime) {
    __update_gt_cputime(&cputime_atomic.utime, sum.utime);
    __update_gt_cputime(&cputime_atomic.stime, sum.stime);
    __update_gt_cputime(&cputime_atomic.sum_exec_runtime, sum.sum_exec_runtime);
    }
//
// thread_group_sample_cputime - Sample cputime for a given task
// @tsk:	Task for which cputime needs to be started
// @samples:	Storage for time samples
//
// Called from sys_getitimer() to calculate the expiry time of an active
// timer. That means group cputime accounting is already active. Called
// with task sighand lock held.
//
// Updates @times with an uptodate sample of the thread group cputimes.
//
#[no_mangle]
pub unsafe extern "C" fn thread_group_sample_cputime(tsk: *mut task_struct, samples: *mut u64) {
    let mut cputimer = &tsk.signal.cputimer;
    let mut pct = &tsk.signal.posix_cputimers;
    WARN_ON_ONCE!(!pct.timers_active);
    proc_sample_cputime_atomic(&cputimer.cputime_atomic, samples);
    }
//
// thread_group_start_cputime - Start cputime and return a sample
// @tsk:	Task for which cputime needs to be started
// @samples:	Storage for time samples
//
// The thread group cputime accounting is avoided when there are no posix
// CPU timers armed. Before starting a timer it's required to check whether
// the time accounting is active. If not, a full update of the atomic
// accounting store needs to be done and the accounting enabled.
//
// Updates @times with an uptodate sample of the thread group cputimes.
//
#[no_mangle]
unsafe extern "C" fn thread_group_start_cputime(tsk: *mut task_struct, samples: *mut u64) {
    let mut cputimer = &tsk.signal.cputimer;
    let mut pct = &tsk.signal.posix_cputimers;
    lockdep_assert_task_sighand_held(tsk);
// Check if cputimer isn't running. This is accessed without locking.
    if (!READ_ONCE(pct.timers_active)) {
pub static mut sum: usize = 0;
//
// The POSIX timer interface allows for absolute time expiry
// values through the TIMER_ABSTIME flag, therefore we have
// to synchronize the timer to the clock every time we start it.
//
    thread_group_cputime(tsk, &sum);
    update_gt_cputime(&cputimer.cputime_atomic, &sum);
//
// We're setting timers_active without a lock. Ensure this
// only gets written to in one operation. We set it after
// update_gt_cputime() as a small optimization, but
// barriers are not required because update_gt_cputime()
// can handle concurrent updates.
//
    WRITE_ONCE(pct.timers_active, true);
    }
    proc_sample_cputime_atomic(&cputimer.cputime_atomic, samples);
    }
#[no_mangle]
unsafe extern "C" fn __thread_group_cputime(tsk: *mut task_struct, samples: *mut u64) {
pub static mut ct: usize = 0;
    thread_group_cputime(tsk, &ct);
    store_samples(samples, ct.stime, ct.utime, ct.sum_exec_runtime);
    }
//
// Sample a process (thread group) clock for the given task clkid. If the
// group's cputime accounting is already enabled, read the atomic
// store. Otherwise a full update is required.  clkid is already validated.
//
#[no_mangle]
pub unsafe extern "C" fn cpu_clock_sample_group(clkid: clockid_t, p: *mut task_struct, start: bool) -> u64 {
    let mut cputimer = &p.signal.cputimer;
    let mut pct = &p.signal.posix_cputimers;
    u64 samples[CPUCLOCK_MAX];
    if (!READ_ONCE(pct.timers_active)) {
    if (start) {
    thread_group_start_cputime(p, samples);
    }
    else {
    __thread_group_cputime(p, samples);
    }
    } else {
    proc_sample_cputime_atomic(&cputimer.cputime_atomic, samples);
    }
    return samples[clkid];
    }
#[no_mangle]
unsafe extern "C" fn posix_cpu_clock_get(clock: clockid_t, tp: *mut timespec64) -> c_int {
pub static mut clkid: clockid_t = 0;
pub static mut tsk: *mut c_void = core::ptr::null_mut();
    let mut t = 0;
    rcu_read_lock();
    tsk = pid_task(pid_for_clock(clock, true), clock_pid_type(clock));
    if (!tsk) {
    rcu_read_unlock();
    return -EINVAL;
    }
    if (CPUCLOCK_PERTHREAD(clock)) {
    t = cpu_clock_sample(clkid, tsk);
    }
    else {
    t = cpu_clock_sample_group(clkid, tsk, false);
    }
    rcu_read_unlock();
// tp = ns_to_timespec64(t);
    return 0;
    }
//
// Validate the clockid_t for a new CPU-clock timer, and initialize the timer.
// This is called from sys_timer_create() and do_cpu_nanosleep() with the
// new timer already all-zeros initialized.
//
#[no_mangle]
unsafe extern "C" fn posix_cpu_timer_create(new_timer: *mut k_itimer) -> c_int {
pub static mut posix_cpu_timers_key: usize = 0;
pub static mut pid: *mut c_void = core::ptr::null_mut();
    rcu_read_lock();
    pid = pid_for_clock(new_timer.it_clock, false);
    if (!pid) {
    rcu_read_unlock();
    return -EINVAL;
    }
//
// If posix timer expiry is handled in task work context then
// timer::it_lock can be taken without disabling interrupts as all
// other locking happens in task context. This requires a separate
// lock class key otherwise regular posix timer expiry would record
// the lock class being taken in interrupt context and generate a
// false positive warning.
//
    if (IS_ENABLED!(CONFIG_POSIX_CPU_TIMERS_TASK_WORK)) {
    lockdep_set_class(&new_timer.it_lock, &posix_cpu_timers_key);
    }
    new_timer.kclock = &clock_posix_cpu;
    timerqueue_init(&new_timer.it.cpu.node);
    new_timer.it.cpu.pid = get_pid(pid);
    rcu_read_unlock();
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn timer_base(timer: *mut k_itimer, tsk: *mut task_struct) -> *mut c_void {
pub static mut clkidx: c_int = 0;
    if (CPUCLOCK_PERTHREAD(timer.it_clock)) {
    return tsk.posix_cputimers.bases + clkidx;
    }
    else {
    return tsk.signal.posix_cputimers.bases + clkidx;
    }
    }
//
// Force recalculating the base earliest expiration on the next tick.
// This will also re-evaluate the need to keep around the process wide
// cputime counter and tick dependency and eventually shut these down
// if necessary.
//
#[no_mangle]
pub unsafe extern "C" fn trigger_base_recalc_expires(timer: *mut k_itimer, tsk: *mut task_struct) {
    let mut base = timer_base(timer, tsk);
    base.nextevt = 0;
    }
//
// Dequeue the timer and reset the base if it was its earliest expiration.
// It makes sure the next tick recalculates the base next expiration so we
// don't keep the costly process wide cputime counter around for a random
// amount of time, along with the tick dependency.
//
// If another timer gets queued between this and the next tick, its
// expiration will update the base next event if necessary on the next
// tick.
//
#[no_mangle]
unsafe extern "C" fn disarm_timer(timer: *mut k_itimer, p: *mut task_struct) {
    let mut ctmr = &timer.it.cpu;
pub static mut base: *mut c_void = core::ptr::null_mut();
    if (!cpu_timer_dequeue(ctmr)) {
    return;
    }
    base = timer_base(timer, p);
    if (cpu_timer_getexpires(ctmr) == base.nextevt) {
    trigger_base_recalc_expires(timer, p);
    }
    }
//
// Lookup the task via timer->it.cpu.pid and attempt to lock the task's sighand.
//
// This can race with the reaping of the task:
//
// CPU0					CPU1
//
// // Finds task
// p = pid_task(pid, pid_type);		__exit_signal(p)
// lock(p, sighand);
// posix_cpu_timers*_exit();
// sighand = lock_task_sighand(p);	  unhash_task(p);
// p->sighand = NULL;
// unlock(sighand);
//
// In this case sighand is NULL, which means the task and the associated timer
// queue cannot be longer accessed safely.
//
// __exit_signal() invokes posix_cpu_timers_exit() and if the thread group is
// dead it also invokes posix_cpu_timers_group_exit(). These functions delete
// all pending timers from the related timer queues. The POSIX timers (k_itimer)
// themself are still accessible, but not longer connected to the task.
//
// exec() works slightly differently. The task which exec()'s terminates all
// other threads in the thread group and runs __exit_signal() on them. As the
// thread group is not dead they only clean up the per task timers via
// posix_cpu_timers_exit().
//
// As the TGID on exec() stays the same per process timers stay queued, if they
// are armed. This works without a problem when exec() is done by the thread
// group leader. If a non-leader thread exec()'s this can end up in the
// following scenario:
//
// CPU0					CPU1
// // Returns old leader
// p = pid_task(pid, pid_type);		de_thread()
// switch_leader()
// release_task(old leader)
// __exit_signal()
// old_leader->sighand = NULL;
// // Returns NULL
// sighand = lock_task_sighand(p)
//
// That's problematic for several functions:
//
// - posix_cpu_timer_del(): If the timer is still enqueued on the task the
// underlying k_itimer will be freed which results in a UAF in
// run_posix_cpu_timers() or on timerqueue related add/delete operations.
// If the timer is not enqueued, the failure is harmless
//
// - posix_cpu_timer_set(): Independent of the enqueued state that results in a
// transient failure which is user space visible (-ESRCH) for regular posix
// timers. But for the use case in do_cpu_nanosleep() it's the same UAF
// problem just that the timer is allocated on the stack.
//
// - posix_cpu_timer_rearm(): Timer is not enqueued at that point, but this
// silently ignores the rearm request, which is a functional problem as the
// timer wont expire anymore.
//
#[no_mangle]
pub unsafe extern "C" fn timer_lock_sighand(timer: *mut k_itimer, flags: *mut c_ulong) -> *mut c_void {
pub static mut type: pid_type = 0;
    let mut ctmr = &timer.it.cpu;
    guard(rcu)();
    for (;;) {
    let mut t = pid_task(timer.it.cpu.pid, type);
// Fail if the task cannot be found.
    if (!t) {
    break;
    }
// Try to lock the task's sighand
    if (lock_task_sighand(t, flags)) {
    return t;
    }
//
// The next PID lookup might either fail or return the new
// leader. This is correct for both exit() and exec().
//
    }
//
// If the timer is still enqueued, warn. There is nothing safe to do
// here as there might be two timers in there which are removed in
// parallel and that will cause more damage than good. This should never
// happen!
//
// Ensure that the stores to the timer and timerqueue are visible:
//
// __exit_signal()
// posix_cpu_timers*_exit()
// write_seqlock(seqlock)
// smp_wmb(); <-------
// __unhash_process()	  |	!pid_task()
// ---->	smp_rmb();
// WARN_ON_ONCE!(...)
//
    smp_rmb();
    WARN_ON_ONCE!(ctmr.head || timerqueue_node_queued(&ctmr.node));
    return core::ptr::null_mut();
    }
//
// Clean up a CPU-clock timer that is about to be destroyed.
// This is called from timer deletion with the timer already locked.
// If we return TIMER_RETRY, it's necessary to release the timer's lock
// and try again.  (This happens when the timer is in the middle of firing.)
//
#[no_mangle]
unsafe extern "C" fn posix_cpu_timer_del(timer: *mut k_itimer) -> c_int {
pub static mut p: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
pub static mut ret: c_int = 0;
    p = timer_lock_sighand(timer, &flags);
    if (likely(p)) {
    if (timer.it.cpu.firing) {
//
// Prevent signal delivery. The timer cannot be dequeued
// because it is on the firing list which is not protected
// by sighand->lock. The delivery path is waiting for
// the timer lock. So go back, unlock and retry.
//
    timer.it.cpu.firing = false;
    ret = TIMER_RETRY;
    } else {
    disarm_timer(timer, p);
    }
    unlock_task_sighand(p, &flags);
    }
    if (!ret) {
    put_pid(timer.it.cpu.pid);
    timer.it_status = POSIX_TIMER_DISARMED;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cleanup_timerqueue(head: *mut timerqueue_head) {
pub static mut node: *mut c_void = core::ptr::null_mut();
pub static mut ctmr: *mut c_void = core::ptr::null_mut();
    while ((node = timerqueue_getnext(head))) {
    timerqueue_del(head, node);
    ctmr = container_of!(node, cpu_timer, node);
    ctmr.head = core::ptr::null_mut();
    }
    }
//
// Clean out CPU timers which are still armed when a thread exits. The
// timers are only removed from the list. No other updates are done. The
// corresponding posix timers are still accessible, but cannot be rearmed.
//
// This must be called with the siglock held.
//
#[no_mangle]
unsafe extern "C" fn cleanup_timers(pct: *mut posix_cputimers) {
    cleanup_timerqueue(&pct.bases[CPUCLOCK_PROF].tqhead);
    cleanup_timerqueue(&pct.bases[CPUCLOCK_VIRT].tqhead);
    cleanup_timerqueue(&pct.bases[CPUCLOCK_SCHED].tqhead);
    }
//
// These are both called with the siglock held, when the current thread
// is being reaped.  When the final (leader) thread in the group is reaped,
// posix_cpu_timers_exit_group will be called after posix_cpu_timers_exit.
//
#[no_mangle]
pub unsafe extern "C" fn posix_cpu_timers_exit(tsk: *mut task_struct) {
    cleanup_timers(&tsk.posix_cputimers);
    }
#[no_mangle]
pub unsafe extern "C" fn posix_cpu_timers_exit_group(tsk: *mut task_struct) {
    cleanup_timers(&tsk.signal.posix_cputimers);
    }
//
// Insert the timer on the appropriate list before any timers that
// expire later.  This must be called with the sighand lock held.
//
#[no_mangle]
unsafe extern "C" fn arm_timer(timer: *mut k_itimer, p: *mut task_struct) {
    let mut base = timer_base(timer, p);
    let mut ctmr = &timer.it.cpu;
pub static mut newexp: u64 = 0;
    timer.it_status = POSIX_TIMER_ARMED;
    if (!cpu_timer_enqueue(&base.tqhead, ctmr)) {
    return;
    }
//
// We are the new earliest-expiring POSIX 1.b timer, hence
// need to update expiration cache. Take into account that
// for process timers we share expiration cache with itimers
// and RLIMIT_CPU and for thread timers with RLIMIT_RTTIME.
//
    if (newexp < base.nextevt) {
    base.nextevt = newexp;
    }
    if (CPUCLOCK_PERTHREAD(timer.it_clock)) {
    tick_dep_set_task(p, TICK_DEP_BIT_POSIX_TIMER);
    }
    else {
    tick_dep_set_signal(p, TICK_DEP_BIT_POSIX_TIMER);
    }
    }
//
// The timer is locked, fire it and arrange for its reload.
//
#[no_mangle]
unsafe extern "C" fn cpu_timer_fire(timer: *mut k_itimer) {
    let mut ctmr = &timer.it.cpu;
    timer.it_status = POSIX_TIMER_DISARMED;
    if (unlikely(ctmr.nanosleep)) {
//
// This a special case for clock_nanosleep,
// not a normal timer from sys_timer_create.
//
    wake_up_process(timer.it_process);
    cpu_timer_setexpires(ctmr, 0);
    } else {
    posix_timer_queue_signal(timer);
// Disable oneshot timers
    if (!timer.it_interval) {
    cpu_timer_setexpires(ctmr, 0);
    }
    }
    }
// forward_decl: __posix_cpu_timer_get;
//
// Guts of sys_timer_settime for CPU timers.
// This is called with the timer locked and interrupts disabled.
// If we return TIMER_RETRY, it's necessary to release the timer's lock
// and try again.  (This happens when the timer is in the middle of firing.)
//
#[no_mangle]
pub unsafe extern "C" fn posix_cpu_timer_set(timer: *mut k_itimer, timer_flags: c_int, new: *mut itimerspec64, old: *mut itimerspec64) -> c_int {
pub static mut sigev_none: bool = false;
pub static mut clkid: clockid_t = 0;
    let mut ctmr = &timer.it.cpu;
    u64 old_expires, new_expires, now;
pub static mut p: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
pub static mut ret: c_int = 0;
    p = timer_lock_sighand(timer, &flags);
//
// If p has just been reaped, we can no longer get any information about
// it at all.
//
    if (!p) {
    return -ESRCH;
    }
//
// Use the to_ktime conversion because that clamps the maximum
// value to KTIME_MAX and avoid multiplication overflows.
//
    new_expires = ktime_to_ns(timespec64_to_ktime(new.it_value));
// Retrieve the current expiry time before disarming the timer
    old_expires = cpu_timer_getexpires(ctmr);
    if (unlikely(timer.it.cpu.firing)) {
//
// Prevent signal delivery. The timer cannot be dequeued
// because it is on the firing list which is not protected
// by sighand->lock. The delivery path is waiting for
// the timer lock. So go back, unlock and retry.
//
    timer.it.cpu.firing = false;
    ret = TIMER_RETRY;
    } else {
    cpu_timer_dequeue(ctmr);
    timer.it_status = POSIX_TIMER_DISARMED;
    }
//
// Sample the current clock for saving the previous setting
// and for rearming the timer.
//
    if (CPUCLOCK_PERTHREAD(timer.it_clock)) {
    now = cpu_clock_sample(clkid, p);
    }
    else {
    now = cpu_clock_sample_group(clkid, p, !sigev_none);
    }
// Retrieve the previous expiry value if requested.
    if (old) {
    old.it_value = (timespec64){ };
    if (old_expires) {
    __posix_cpu_timer_get(timer, old, now);
    }
    }
// Retry if the timer expiry is running concurrently
    if (unlikely(ret)) {
    unlock_task_sighand(p, &flags);
    return ret;
    }
// Convert relative expiry time to absolute
    if (new_expires && !(timer_flags & TIMER_ABSTIME)) {
    new_expires += now;
    }
// Set the new expiry time (might be 0)
    cpu_timer_setexpires(ctmr, new_expires);
//
// Arm the timer if it is not disabled, the new expiry value has
// not yet expired and the timer requires signal delivery.
// SIGEV_NONE timers are never armed. In case the timer is not
// armed, enforce the reevaluation of the timer base so that the
// process wide cputime counter can be disabled eventually.
//
    if (likely(!sigev_none)) {
    if (new_expires && now < new_expires) {
    arm_timer(timer, p);
    }
    else {
    trigger_base_recalc_expires(timer, p);
    }
    }
    unlock_task_sighand(p, &flags);
    posix_timer_set_common(timer, new);
//
// If the new expiry time was already in the past the timer was not
// queued. Fire it immediately even if the thread never runs to
// accumulate more time on this clock.
//
    if (!sigev_none && new_expires && now >= new_expires) {
    cpu_timer_fire(timer);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn __posix_cpu_timer_get(timer: *mut k_itimer, itp: *mut itimerspec64, now: u64) {
pub static mut sigev_none: bool = false;
    u64 expires, iv = timer.it_interval;
//
// Make sure that interval timers are moved forward for the
// following cases:
// - SIGEV_NONE timers which are never armed
// - Timers which expired, but the signal has not yet been
// delivered
//
    if (iv && timer.it_status != POSIX_TIMER_ARMED) {
    expires = bump_cpu_timer(timer, now);
    }
    else {
    expires = cpu_timer_getexpires(&timer.it.cpu);
    }
//
// Expired interval timers cannot have a remaining time <= 0.
// The kernel has to move them forward so that the next
// timer expiry is > @now.
//
    if (now < expires) {
    itp.it_value = ns_to_timespec64(expires - now);
    } else {
//
// A single shot SIGEV_NONE timer must return 0, when it is
// expired! Timers which have a real signal delivery mode
// must return a remaining time greater than 0 because the
// signal has not yet been delivered.
//
    if (!sigev_none) {
    itp.it_value.tv_nsec = 1;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn posix_cpu_timer_get(timer: *mut k_itimer, itp: *mut itimerspec64) {
pub static mut clkid: clockid_t = 0;
pub static mut p: *mut c_void = core::ptr::null_mut();
    let mut now = 0;
    rcu_read_lock();
    p = cpu_timer_task_rcu(timer);
    if (p && cpu_timer_getexpires(&timer.it.cpu)) {
    itp.it_interval = ktime_to_timespec64(timer.it_interval);
    if (CPUCLOCK_PERTHREAD(timer.it_clock)) {
    now = cpu_clock_sample(clkid, p);
    }
    else {
    now = cpu_clock_sample_group(clkid, p, false);
    }
    __posix_cpu_timer_get(timer, itp, now);
    }
    rcu_read_unlock();
    }
pub const MAX_COLLECTED: c_int = 20;
#[no_mangle]
pub unsafe extern "C" fn collect_timerqueue(head: *mut timerqueue_head, firing: *mut list_head, now: u64) -> u64 {
pub static mut next: *mut c_void = core::ptr::null_mut();
pub static mut i: c_int = 0;
    while ((next = timerqueue_getnext(head))) {
pub static mut ctmr: *mut c_void = core::ptr::null_mut();
    let mut expires = 0;
    ctmr = container_of!(next, cpu_timer, node);
    expires = cpu_timer_getexpires(ctmr);
// Limit the number of timers to expire at once
    if (++i == MAX_COLLECTED || now < expires) {
    return expires;
    }
    ctmr.firing = true;
// See posix_cpu_timer_wait_running()
    rcu_assign_pointer(ctmr.handling, current);
    cpu_timer_dequeue(ctmr);
    list_add_tail(&ctmr.elist, firing);
    }
    return U64_MAX;
    }
#[no_mangle]
pub unsafe extern "C" fn collect_posix_cputimers(pct: *mut posix_cputimers, samples: *mut u64, firing: *mut list_head) {
    let mut base = pct.bases;
    let mut i = 0;
    while (i < CPUCLOCK_MAX) {
    base.nextevt = collect_timerqueue(&base.tqhead, firing,
    samples[i]);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn check_dl_overrun(tsk: *mut task_struct) {
    if (tsk.dl.dl_overrun) {
    tsk.dl.dl_overrun = 0;
    send_signal_locked(SIGXCPU, SEND_SIG_PRIV, tsk, PIDTYPE_TGID);
    }
    }
#[no_mangle]
unsafe extern "C" fn check_rlimit(time: u64, limit: u64, signo: c_int, rt: bool, hard: bool) -> bool {
    if (time < limit) {
    return false;
    }
    if (print_fatal_signals) {
    pr_info!("%s Watchdog Timeout (%s): %s[%d]\n",
    rt ? "RT" : "CPU", hard ? "hard" : "soft",
    current.comm, task_pid_nr(current));
    }
    send_signal_locked(signo, SEND_SIG_PRIV, current, PIDTYPE_TGID);
    return true;
    }
//
// Check for any per-thread CPU timers that have fired and move them off
// the tsk->cpu_timers[N] list onto the firing list.  Here we update the
// tsk->it_*_expires values to reflect the remaining thread CPU timers.
//
#[no_mangle]
pub unsafe extern "C" fn check_thread_timers(tsk: *mut task_struct, firing: *mut list_head) {
    let mut pct = &tsk.posix_cputimers;
    u64 samples[CPUCLOCK_MAX];
    let mut soft = 0;
    if (dl_task(tsk)) {
    check_dl_overrun(tsk);
    }
    if (expiry_cache_is_inactive(pct)) {
    return;
    }
    task_sample_cputime(tsk, samples);
    collect_posix_cputimers(pct, samples, firing);
//
// Check for the special case thread timers.
//
    soft = task_rlimit(tsk, RLIMIT_RTTIME);
    if (soft != RLIM_INFINITY) {
// Task RT timeout is accounted in jiffies. RTTIME is usec
pub static mut rttime: c_ulong = 0;
pub static mut hard: c_ulong = 0;
// At the hard limit, send SIGKILL. No further action.
    if (hard != RLIM_INFINITY &&
    check_rlimit(rttime, hard, SIGKILL, true, true)) {
    return;
    }
// At the soft limit, send a SIGXCPU every second
    if (check_rlimit(rttime, soft, SIGXCPU, true, false)) {
    soft += USEC_PER_SEC;
    tsk.signal.rlim[RLIMIT_RTTIME].rlim_cur = soft;
    }
    }
    if (expiry_cache_is_inactive(pct)) {
    tick_dep_clear_task(tsk, TICK_DEP_BIT_POSIX_TIMER);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn stop_process_timers(sig: *mut signal_struct) {
    let mut pct = &sig.posix_cputimers;
// Turn off the active flag. This is done without locking.
    WRITE_ONCE(pct.timers_active, false);
    tick_dep_clear_signal(sig, TICK_DEP_BIT_POSIX_TIMER);
    }
#[no_mangle]
pub unsafe extern "C" fn check_cpu_itimer(tsk: *mut task_struct, it: *mut cpu_itimer, expires: *mut u64, cur_time: u64, signo: c_int) {
    if (!it.expires) {
    return;
    }
    if (cur_time >= it.expires) {
    if (it.incr) {
    it.expires += it.incr;
    }
    else {
    it.expires = 0;
    }
    trace_itimer_expire(signo == SIGPROF ?
    ITIMER_PROF : ITIMER_VIRTUAL,
    task_tgid(tsk), cur_time);
    send_signal_locked(signo, SEND_SIG_PRIV, tsk, PIDTYPE_TGID);
    }
    if (it.expires && it.expires < *expires) {
// expires = it->expires;
    }
    }
//
// Check for any per-thread CPU timers that have fired and move them
// off the tsk->*_timers list onto the firing list.  Per-thread timers
// have already been taken off.
//
#[no_mangle]
pub unsafe extern "C" fn check_process_timers(tsk: *mut task_struct, firing: *mut list_head) {
pub static mut sig: *mut signal_const = core::ptr::null_mut();
    let mut pct = &sig.posix_cputimers;
    u64 samples[CPUCLOCK_MAX];
    let mut soft = 0;
//
// If there are no active process wide timers (POSIX 1.b, itimers,
// RLIMIT_CPU) nothing to check. Also skip the process wide timer
// processing when there is already another task handling them.
//
    if (!READ_ONCE(pct.timers_active) || pct.expiry_active) {
    return;
    }
//
// Signify that a thread is checking for process timers.
// Write access to this field is protected by the sighand lock.
//
    pct.expiry_active = true;
//
// Collect the current process totals. Group accounting is active
// so the sample can be taken directly.
//
    proc_sample_cputime_atomic(&sig.cputimer.cputime_atomic, samples);
    collect_posix_cputimers(pct, samples, firing);
//
// Check for the special case process timers.
//
    check_cpu_itimer(tsk, &sig.it[CPUCLOCK_PROF],
    &pct.bases[CPUCLOCK_PROF].nextevt,
    samples[CPUCLOCK_PROF], SIGPROF);
    check_cpu_itimer(tsk, &sig.it[CPUCLOCK_VIRT],
    &pct.bases[CPUCLOCK_VIRT].nextevt,
    samples[CPUCLOCK_VIRT], SIGVTALRM);
    soft = task_rlimit(tsk, RLIMIT_CPU);
    if (soft != RLIM_INFINITY) {
// RLIMIT_CPU is in seconds. Samples are nanoseconds
pub static mut hard: c_ulong = 0;
pub static mut ptime: u64 = 0;
pub static mut softns: u64 = 0;
pub static mut hardns: u64 = 0;
// At the hard limit, send SIGKILL. No further action.
    if (hard != RLIM_INFINITY &&
    check_rlimit(ptime, hardns, SIGKILL, false, true)) {
    return;
    }
// At the soft limit, send a SIGXCPU every second
    if (check_rlimit(ptime, softns, SIGXCPU, false, false)) {
    sig.rlim[RLIMIT_CPU].rlim_cur = soft + 1;
    softns += NSEC_PER_SEC;
    }
// Update the expiry cache
    if (softns < pct.bases[CPUCLOCK_PROF].nextevt) {
    pct.bases[CPUCLOCK_PROF].nextevt = softns;
    }
    }
    if (expiry_cache_is_inactive(pct)) {
    stop_process_timers(sig);
    }
    pct.expiry_active = false;
    }
//
// This is called from the signal code (via posixtimer_rearm)
// when the last timer signal was delivered and we have to reload the timer.
//
// Return true unconditionally so the core code assumes the timer to be
// armed. Otherwise it would requeue the signal.
//
#[no_mangle]
unsafe extern "C" fn posix_cpu_timer_rearm(timer: *mut k_itimer) -> bool {
pub static mut clkid: clockid_t = 0;
pub static mut p: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
    let mut now = 0;
    p = timer_lock_sighand(timer, &flags);
    if (unlikely(!p)) {
    return true;
    }
//
// Fetch the current sample and update the timer's expiry time.
//
    if (CPUCLOCK_PERTHREAD(timer.it_clock)) {
    now = cpu_clock_sample(clkid, p);
    }
    else {
    now = cpu_clock_sample_group(clkid, p, true);
    }
    bump_cpu_timer(timer, now);
//
// Now re-arm for the new expiry time.
//
    arm_timer(timer, p);
    unlock_task_sighand(p, &flags);
    return true;
    }
//
// task_cputimers_expired - Check whether posix CPU timers are expired
//
// @samples:	Array of current samples for the CPUCLOCK clocks
// @pct:	Pointer to a posix_cputimers container
//
// Returns true if any member of @samples is greater than the corresponding
// member of @pct->bases[CLK].nextevt. False otherwise
//
#[no_mangle]
pub unsafe extern "C" fn task_cputimers_expired(samples: *mut u64, pct: *mut posix_cputimers) -> bool {
    let mut i = 0;
    while (i < CPUCLOCK_MAX) {
    if (samples[i] >= pct.bases[i].nextevt) {
    return true;
    }
    }
    return false;
    }
//
// fastpath_timer_check - POSIX CPU timers fast path.
//
// @tsk:	The task (thread) being checked.
//
// Check the task and thread group timers.  If both are zero (there are no
// timers set) return false.  Otherwise snapshot the task and thread group
// timers and compare them with the corresponding expiration times.  Return
// true if a timer has expired, else return false.
//
#[no_mangle]
pub unsafe extern "C" fn fastpath_timer_check(tsk: *mut task_struct) -> bool {
    let mut pct = &tsk.posix_cputimers;
pub static mut sig: *mut c_void = core::ptr::null_mut();
    if (!expiry_cache_is_inactive(pct)) {
    u64 samples[CPUCLOCK_MAX];
    task_sample_cputime(tsk, samples);
    if (task_cputimers_expired(samples, pct)) {
    return true;
    }
    }
    sig = tsk.signal;
    pct = &sig.posix_cputimers;
//
// Check if thread group timers expired when timers are active and
// no other thread in the group is already handling expiry for
// thread group cputimers. These fields are read without the
// sighand lock. However, this is fine because this is meant to be
// a fastpath heuristic to determine whether we should try to
// acquire the sighand lock to handle timer expiry.
//
// In the worst case scenario, if concurrently timers_active is set
// or expiry_active is cleared, but the current thread doesn't see
// the change yet, the timer checks are delayed until the next
// thread in the group gets a scheduler interrupt to handle the
// timer. This isn't an issue in practice because these types of
// delays with signals actually getting sent are expected.
//
    if (READ_ONCE(pct.timers_active) && !READ_ONCE(pct.expiry_active)) {
    u64 samples[CPUCLOCK_MAX];
    proc_sample_cputime_atomic(&sig.cputimer.cputime_atomic,
    samples);
    if (task_cputimers_expired(samples, pct)) {
    return true;
    }
    }
    if (dl_task(tsk) && tsk.dl.dl_overrun) {
    return true;
    }
    return false;
    }
// forward_decl: handle_posix_cpu_timers;

#[no_mangle]
unsafe extern "C" fn posix_cpu_timers_work(work: *mut callback_head) {
    let mut cw = container_of!(work, typeof(*cw), work);
    mutex_lock(&cw.mutex);
    handle_posix_cpu_timers(current);
    mutex_unlock(&cw.mutex);
    }
//
// Invoked from the posix-timer core when a cancel operation failed because
// the timer is marked firing. The caller holds rcu_read_lock(), which
// protects the timer and the task which is expiring it from being freed.
//
#[no_mangle]
unsafe extern "C" fn posix_cpu_timer_wait_running(timr: *mut k_itimer) {
    let mut tsk = rcu_dereference(timr.it.cpu.handling);
// Has the handling task completed expiry already?
    if (!tsk) {
    return;
    }
// Ensure that the task cannot go away
    get_task_struct(tsk);
// Now drop the RCU protection so the mutex can be locked
    rcu_read_unlock();
// Wait on the expiry mutex
    mutex_lock(&tsk.posix_cputimers_work.mutex);
// Release it immediately again.
    mutex_unlock(&tsk.posix_cputimers_work.mutex);
// Drop the task reference.
    put_task_struct(tsk);
// Relock RCU so the callsite is balanced
    rcu_read_lock();
    }
#[no_mangle]
unsafe extern "C" fn posix_cpu_timer_wait_running_nsleep(timr: *mut k_itimer) {
// Ensure that timr->it.cpu.handling task cannot go away
    rcu_read_lock();
    spin_unlock_irq(&timr.it_lock);
    posix_cpu_timer_wait_running(timr);
    rcu_read_unlock();
// @timr is on stack and is valid
    spin_lock_irq(&timr.it_lock);
    }
//
// Clear existing posix CPU timers task work.
//
#[no_mangle]
pub unsafe extern "C" fn clear_posix_cputimers_work(p: *mut task_struct) {
//
// A copied work entry from the old task is not meaningful, clear it.
// N.B. init_task_work will not do this.
//
    memset(&p.posix_cputimers_work.work, 0,
    sizeof!(p.posix_cputimers_work.work));
    init_task_work(&p.posix_cputimers_work.work,
    posix_cpu_timers_work);
    mutex_init(&p.posix_cputimers_work.mutex);
    p.posix_cputimers_work.scheduled = false;
    }
//
// Initialize posix CPU timers task work in init task. Out of line to
// keep the callback static and to avoid header recursion hell.
//
#[no_mangle]
pub unsafe extern "C" fn posix_cputimers_init_work()  {
    clear_posix_cputimers_work(current);
    }
//
// Note: All operations on tsk->posix_cputimer_work.scheduled happen either
// in hard interrupt context or in task context with interrupts
// disabled. Aside of that the writer/reader interaction is always in the
// context of the current task, which means they are strict per CPU.
//
#[no_mangle]
pub unsafe extern "C" fn posix_cpu_timers_work_scheduled(tsk: *mut task_struct) -> bool {
    return tsk.posix_cputimers_work.scheduled;
    }
#[no_mangle]
pub unsafe extern "C" fn __run_posix_cpu_timers(tsk: *mut task_struct) {
    if (WARN_ON_ONCE!(tsk.posix_cputimers_work.scheduled)) {
    return;
    }
// Schedule task work to actually expire the timers
    tsk.posix_cputimers_work.scheduled = true;
    task_work_add(tsk, &tsk.posix_cputimers_work.work, TWA_RESUME);
    }
#[no_mangle]
pub unsafe extern "C" fn posix_cpu_timers_enable_work(tsk: *mut task_struct, start: c_ulong) -> bool {
pub static mut ret: bool = true;
//
// On !RT kernels interrupts are disabled while collecting expired
// timers, so no tick can happen and the fast path check can be
// reenabled without further checks.
//
    if (!IS_ENABLED!(CONFIG_PREEMPT_RT)) {
    tsk.posix_cputimers_work.scheduled = false;
    return true;
    }
//
// On RT enabled kernels ticks can happen while the expired timers
// are collected under sighand lock. But any tick which observes
// the CPUTIMERS_WORK_SCHEDULED bit set, does not run the fastpath
// checks. So reenabling the tick work has do be done carefully:
//
// Disable interrupts and run the fast path check if jiffies have
// advanced since the collecting of expired timers started. If
// jiffies have not advanced or the fast path check did not find
// newly expired timers, reenable the fast path check in the timer
// interrupt. If there are newly expired timers, return false and
// let the collection loop repeat.
//
    local_irq_disable();
    if (start != jiffies && fastpath_timer_check(tsk)) {
    ret = false;
    }
    else {
    tsk.posix_cputimers_work.scheduled = false;
    }
    local_irq_enable();
    return ret;
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: __run_posix_cpu_timers
pub unsafe extern "C" fn __run_posix_cpu_timers_dup(tsk: *mut task_struct) {
    lockdep_posixtimer_enter();
    handle_posix_cpu_timers(tsk);
    lockdep_posixtimer_exit();
    }
#[no_mangle]
unsafe extern "C" fn posix_cpu_timer_wait_running(timr: *mut k_itimer) {
    cpu_relax();
    }
#[no_mangle]
unsafe extern "C" fn posix_cpu_timer_wait_running_nsleep(timr: *mut k_itimer) {
    spin_unlock_irq(&timr.it_lock);
    cpu_relax();
    spin_lock_irq(&timr.it_lock);
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: posix_cpu_timers_work_scheduled
pub unsafe extern "C" fn posix_cpu_timers_work_scheduled_dup(tsk: *mut task_struct) -> bool {
    return false;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: posix_cpu_timers_enable_work
pub unsafe extern "C" fn posix_cpu_timers_enable_work_dup(tsk: *mut task_struct, start: c_ulong) -> bool {
    return true;
    }

#[no_mangle]
unsafe extern "C" fn handle_posix_cpu_timers(tsk: *mut task_struct) {
    let mut timer = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
    unsigned long flags, start;
pub static mut firing: usize = 0;
//
// tsk is current and ->sighand is stable, see the
// tsk->exit_state check in run_posix_cpu_timers()
//
    spin_lock_irqsave(&tsk.sighand.siglock, flags);
    do {
//
// On RT locking sighand lock does not disable interrupts,
// so this needs to be careful vs. ticks. Store the current
// jiffies value.
//
    start = READ_ONCE(jiffies);
    barrier();
//
// Here we take off tsk->signal->cpu_timers[N] and
// tsk->cpu_timers[N] all the timers that are firing, and
// put them on the firing list.
//
    check_thread_timers(tsk, &firing);
    check_process_timers(tsk, &firing);
//
// The above timer checks have updated the expiry cache and
// because nothing can have queued or modified timers after
// sighand lock was taken above it is guaranteed to be
// consistent. So the next timer interrupt fastpath check
// will find valid data.
//
// If timer expiry runs in the timer interrupt context then
// the loop is not relevant as timers will be directly
// expired in interrupt context. The stub function below
// returns always true which allows the compiler to
// optimize the loop out.
//
// If timer expiry is deferred to task work context then
// the following rules apply:
//
// - On !RT kernels no tick can have happened on this CPU
// after sighand lock was acquired because interrupts are
// disabled. So reenabling task work before dropping
// sighand lock and reenabling interrupts is race free.
//
// - On RT kernels ticks might have happened but the tick
// work ignored posix CPU timer handling because the
// CPUTIMERS_WORK_SCHEDULED bit is set. Reenabling work
// must be done very carefully including a check whether
// ticks have happened since the start of the timer
// expiry checks. posix_cpu_timers_enable_work() takes
// care of that and eventually lets the expiry checks
// run again.
//
    } while (!posix_cpu_timers_enable_work(tsk, start));
//
// We must release sighand lock before taking any timer's lock.
// There is a potential race with timer deletion here, as the
// siglock now protects our private firing list.  We have set
// the firing flag in each timer, so that a deletion attempt
// that gets the timer lock before we do will give it up and
// spin until we've taken care of that timer below.
//
    spin_unlock_irqrestore(&tsk.sighand.siglock, flags);
//
// Now that all the timers on our list have the firing flag,
// no one will touch their list entries but us.  We'll take
// each timer's lock before clearing its firing flag, so no
// timer call will interfere.
//
    list_for_each_entry_safe(timer, next, &firing, it.cpu.elist) {
    let mut cpu_firing = 0;
//
// spin_lock() is sufficient here even independent of the
// expiry context. If expiry happens in hard interrupt
// context it's obvious. For task work context it's safe
// because all other operations on timer::it_lock happen in
// task context (syscall or exit).
//
    spin_lock(&timer.it_lock);
    list_del_init(&timer.it.cpu.elist);
    cpu_firing = timer.it.cpu.firing;
    timer.it.cpu.firing = false;
//
// If the firing flag is cleared then this raced with a
// timer rearm/delete operation. So don't generate an
// event.
//
    if (likely(cpu_firing)) {
    cpu_timer_fire(timer);
    }
// See posix_cpu_timer_wait_running()
    rcu_assign_pointer(timer.it.cpu.handling, core::ptr::null_mut());
    spin_unlock(&timer.it_lock);
    }
    }
//
// This is called from the timer interrupt handler.  The irq handler has
// already updated our counts.  We need to check if any timers fire now.
// Interrupts are disabled.
//
#[no_mangle]
pub unsafe extern "C" fn run_posix_cpu_timers() {
    let mut tsk = current;
    lockdep_assert_irqs_disabled();
//
// Ensure that release_task(tsk) can't happen while
// handle_posix_cpu_timers() is running. Otherwise, a concurrent
// posix_cpu_timer_del() may fail to lock_task_sighand(tsk) and
// miss timer->it.cpu.firing != 0.
//
    if (tsk.exit_state) {
    return;
    }
//
// If the actual expiry is deferred to task work context and the
// work is already scheduled there is no point to do anything here.
//
    if (posix_cpu_timers_work_scheduled(tsk)) {
    return;
    }
//
// The fast path checks that there are no expired thread or thread
// group timers.  If that's so, just return.
//
    if (!fastpath_timer_check(tsk)) {
    return;
    }
    __run_posix_cpu_timers(tsk);
    }
//
// Set one of the process-wide special case CPU timers or RLIMIT_CPU.
// The tsk->sighand->siglock must be held by the caller.
//
#[no_mangle]
pub unsafe extern "C" fn set_process_cpu_timer(tsk: *mut task_struct, clkid: c_uint, newval: *mut u64, oldval: *mut u64) {
    u64 now, *nextevt;
    if (WARN_ON_ONCE!(clkid >= CPUCLOCK_SCHED)) {
    return;
    }
    nextevt = &tsk.signal.posix_cputimers.bases[clkid].nextevt;
    now = cpu_clock_sample_group(clkid, tsk, true);
    if (oldval) {
//
// We are setting itimer. The *oldval is absolute and we update
// it to be relative, *newval argument is relative and we update
// it to be absolute.
//
    if (*oldval) {
    if (*oldval <= now) {
// Just about to fire.
// oldval = TICK_NSEC;
    } else {
// oldval -= now;
    }
    }
    if (*newval) {
// newval += now;
    }
    }
//
// Update expiration cache if this is the earliest timer. CPUCLOCK_PROF
// expiry cache is also used by RLIMIT_CPU!.
//
    if (*newval < *nextevt) {
// nextevt = *newval;
    }
    tick_dep_set_signal(tsk, TICK_DEP_BIT_POSIX_TIMER);
    }
#[no_mangle]
pub unsafe extern "C" fn do_cpu_nanosleep(which_clock: clockid_t, flags: c_int, rqtp: *mut timespec64) -> c_int {
pub static mut it: usize = 0;
pub static mut timer: usize = 0;
    let mut expires = 0;
    let mut error = 0;
//
// Set up a temporary timer and then wait for it to go off.
//
    memset(&timer, 0, sizeof timer);
    spin_lock_init(&timer.it_lock);
    timer.it_clock = which_clock;
    timer.it_overrun = -1;
    error = posix_cpu_timer_create(&timer);
    timer.it_process = current;
    timer.it.cpu.nanosleep = true;
    if (!error) {
pub static mut zero_it: usize = 0;
pub static mut restart: *mut c_void = core::ptr::null_mut();
    memset(&it, 0, sizeof!(it));
    it.it_value = *rqtp;
    spin_lock_irq(&timer.it_lock);
    error = posix_cpu_timer_set(&timer, flags, &it, core::ptr::null_mut());
    if (error) {
    posix_cpu_timer_del(&timer);
    spin_unlock_irq(&timer.it_lock);
    return error;
    }
    while (!signal_pending(current)) {
    if (!cpu_timer_getexpires(&timer.it.cpu)) {
//
// Our timer fired and was reset, below
// deletion can not fail.
//
    posix_cpu_timer_del(&timer);
    spin_unlock_irq(&timer.it_lock);
    return 0;
    }
//
// Block until cpu_timer_fire (or a signal) wakes us.
//
    __set_current_state(TASK_INTERRUPTIBLE);
    spin_unlock_irq(&timer.it_lock);
    schedule();
    spin_lock_irq(&timer.it_lock);
    }
//
// We were interrupted by a signal.
//
    expires = cpu_timer_getexpires(&timer.it.cpu);
    error = posix_cpu_timer_set(&timer, 0, &zero_it, &it);
    if (!error) {
// Timer is now unarmed, deletion can not fail.
    posix_cpu_timer_del(&timer);
    } else {
    while (error == TIMER_RETRY) {
    posix_cpu_timer_wait_running_nsleep(&timer);
    error = posix_cpu_timer_del(&timer);
    }
    }
    spin_unlock_irq(&timer.it_lock);
    if ((it.it_value.tv_sec | it.it_value.tv_nsec) == 0) {
//
// It actually did fire already.
//
    return 0;
    }
    error = -ERESTART_RESTARTBLOCK;
//
// Report back to the user the time still remaining.
//
    restart = &current.restart_block;
    restart.nanosleep.expires = ns_to_ktime(expires);
    if (restart.nanosleep.type != TT_NONE) {
    error = nanosleep_copyout(restart, &it.it_value);
    }
    }
    return error;
    }
// forward_decl: posix_cpu_nsleep_restart;
#[no_mangle]
pub unsafe extern "C" fn posix_cpu_nsleep(which_clock: clockid_t, flags: c_int, rqtp: *mut timespec64) -> c_int {
    let mut restart_block = &current.restart_block;
    let mut error = 0;
//
// Diagnose required errors first.
//
    if (CPUCLOCK_PERTHREAD(which_clock) &&
    (CPUCLOCK_PID(which_clock) == 0 ||
    CPUCLOCK_PID(which_clock) == task_pid_vnr(current))) {
    return -EINVAL;
    }
    error = do_cpu_nanosleep(which_clock, flags, rqtp);
    if (error == -ERESTART_RESTARTBLOCK) {
    if (flags & TIMER_ABSTIME) {
    return -ERESTARTNOHAND;
    }
    restart_block.nanosleep.clockid = which_clock;
    set_restart_fn(restart_block, posix_cpu_nsleep_restart);
    }
    return error;
    }
#[no_mangle]
unsafe extern "C" fn posix_cpu_nsleep_restart(restart_block: *mut restart_block) -> c_long {
pub static mut which_clock: clockid_t = 0;
pub static mut t: usize = 0;
    t = ktime_to_timespec64(restart_block.nanosleep.expires);
    return do_cpu_nanosleep(which_clock, TIMER_ABSTIME, &t);
    }

#[no_mangle]
pub unsafe extern "C" fn process_cpu_clock_getres(which_clock: clockid_t, tp: *mut timespec64) -> c_int {
    return posix_cpu_clock_getres(PROCESS_CLOCK, tp);
    }
#[no_mangle]
pub unsafe extern "C" fn process_cpu_clock_get(which_clock: clockid_t, tp: *mut timespec64) -> c_int {
    return posix_cpu_clock_get(PROCESS_CLOCK, tp);
    }
#[no_mangle]
unsafe extern "C" fn process_cpu_timer_create(timer: *mut k_itimer) -> c_int {
    timer.it_clock = PROCESS_CLOCK;
    return posix_cpu_timer_create(timer);
    }
#[no_mangle]
pub unsafe extern "C" fn process_cpu_nsleep(which_clock: clockid_t, flags: c_int, rqtp: *mut timespec64) -> c_int {
    return posix_cpu_nsleep(PROCESS_CLOCK, flags, rqtp);
    }
#[no_mangle]
pub unsafe extern "C" fn thread_cpu_clock_getres(which_clock: clockid_t, tp: *mut timespec64) -> c_int {
    return posix_cpu_clock_getres(THREAD_CLOCK, tp);
    }
#[no_mangle]
pub unsafe extern "C" fn thread_cpu_clock_get(which_clock: clockid_t, tp: *mut timespec64) -> c_int {
    return posix_cpu_clock_get(THREAD_CLOCK, tp);
    }
#[no_mangle]
unsafe extern "C" fn thread_cpu_timer_create(timer: *mut k_itimer) -> c_int {
    timer.it_clock = THREAD_CLOCK;
    return posix_cpu_timer_create(timer);
    }
pub static mut k_clock: usize = 0;
pub static mut k_clock: usize = 0;
pub static mut k_clock: usize = 0;