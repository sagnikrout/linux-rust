//! Automatically rewritten from C to Rust
//! Source: kernel/time/itimer.c
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
// Copyright (C) 1992 Darren Senn
//
// These are all the functions necessary to implement itimers

//
// itimer_get_remtime - get remaining time for the timer
//
// @timer: the timer to read
//
// Returns the delta between the expiry time and now, which can be
// less than zero or 1usec for an pending expired timer
//
#[no_mangle]
unsafe extern "C" fn itimer_get_remtime(timer: *mut hrtimer) -> timespec64 {
pub static mut rem: ktime_t = 0;
//
// Racy but safe: if the itimer expires after the above
// hrtimer_get_remtime() call but before this condition
// then we return 0 - which is correct.
//
    if (hrtimer_active(timer)) {
    if (rem <= 0) {
    rem = NSEC_PER_USEC;
    }
    } else {
    rem = 0;
    }
    return ktime_to_timespec64(rem);
    }
#[no_mangle]
pub unsafe extern "C" fn get_cpu_itimer(tsk: *mut task_struct, clock_id: c_uint, value: *mut itimerspec64) {
    u64 val, interval;
    let mut it = &tsk.signal.it[clock_id];
    spin_lock_irq(&tsk.sighand.siglock);
    val = it.expires;
    interval = it.incr;
    if (val) {
    u64 t, samples[CPUCLOCK_MAX];
    thread_group_sample_cputime(tsk, samples);
    t = samples[clock_id];
    if (val < t) {
// about to fire
    val = TICK_NSEC;
    }
    else {
    val -= t;
    }
    }
    spin_unlock_irq(&tsk.sighand.siglock);
    value.it_value = ns_to_timespec64(val);
    value.it_interval = ns_to_timespec64(interval);
    }
#[no_mangle]
unsafe extern "C" fn do_getitimer(which: c_int, value: *mut itimerspec64) -> c_int {
    let mut tsk = current;
    match (which) {
    ITIMER_REAL => {
    spin_lock_irq(&tsk.sighand.siglock);
    value.it_value = itimer_get_remtime(&tsk.signal.real_timer);
    value.it_interval =
    ktime_to_timespec64(tsk.signal.it_real_incr);
    spin_unlock_irq(&tsk.sighand.siglock);
    // break;
    }
    ITIMER_VIRTUAL => {
    get_cpu_itimer(tsk, CPUCLOCK_VIRT, value);
    // break;
    }
    ITIMER_PROF => {
    get_cpu_itimer(tsk, CPUCLOCK_PROF, value);
    // break;
    }
    _ => {
    return(-EINVAL);
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn put_itimerval(o: *mut __kernel_old_itimerval, i: *mut itimerspec64) -> c_int {
pub static mut v: __kernel_old_itimerval = 0;
    v.it_interval.tv_sec = i.it_interval.tv_sec;
    v.it_interval.tv_usec = i.it_interval.tv_nsec / NSEC_PER_USEC;
    v.it_value.tv_sec = i.it_value.tv_sec;
    v.it_value.tv_usec = i.it_value.tv_nsec / NSEC_PER_USEC;
    return copy_to_user(o, &v, sizeof!(__kernel_old_itimerval)) ? -EFAULT : 0;
    }
#[no_mangle]
pub unsafe extern "C" fn sys_getitimer(which: usize, value: usize) -> c_long {
pub static mut get_buffer: usize = 0;
pub static mut error: c_int = 0;
    if (!error && put_itimerval(value, &get_buffer)) {
    error = -EFAULT;
    }
    return error;
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct old_itimerval32 {
    pub it_interval: old_timeval32,
    pub it_value: old_timeval32,
}

#[no_mangle]
pub unsafe extern "C" fn put_old_itimerval32(o: *mut old_itimerval32, i: *mut itimerspec64) -> c_int {
pub static mut v32: usize = 0;
    v32.it_interval.tv_sec = i.it_interval.tv_sec;
    v32.it_interval.tv_usec = i.it_interval.tv_nsec / NSEC_PER_USEC;
    v32.it_value.tv_sec = i.it_value.tv_sec;
    v32.it_value.tv_usec = i.it_value.tv_nsec / NSEC_PER_USEC;
    return copy_to_user(o, &v32, sizeof!(old_itimerval32)) ? -EFAULT : 0;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: sys_getitimer
pub unsafe extern "C" fn sys_getitimer_dup(which: usize, value: usize) -> c_long {
pub static mut get_buffer: usize = 0;
pub static mut error: c_int = 0;
    if (!error && put_old_itimerval32(value, &get_buffer)) {
    error = -EFAULT;
    }
    return error;
    }

//
// Invoked from dequeue_signal() when SIG_ALRM is delivered.
//
// Restart the ITIMER_REAL timer if it is armed as periodic timer.  Doing
// this in the signal delivery path instead of self rearming prevents a DoS
// with small increments in the high reolution timer case and reduces timer
// noise in general.
//
#[no_mangle]
pub unsafe extern "C" fn posixtimer_rearm_itimer(tsk: *mut task_struct) {
    let mut tmr = &tsk.signal.real_timer;
    if (!hrtimer_is_queued(tmr) && tsk.signal.it_real_incr != 0) {
    hrtimer_forward_now(tmr, tsk.signal.it_real_incr);
    hrtimer_restart(tmr);
    }
    }
//
// Interval timers are restarted in the signal delivery path.  See
// posixtimer_rearm_itimer().
//
#[no_mangle]
pub unsafe extern "C" fn it_real_fn(timer: *mut hrtimer) -> enum hrtimer_restart {
    let mut sig = container_of!(timer, signal_struct, real_timer);
    let mut leader_pid = sig.pids[PIDTYPE_TGID];
    trace_itimer_expire(ITIMER_REAL, leader_pid, 0);
    kill_pid_info(SIGALRM, SEND_SIG_PRIV, leader_pid);
    return HRTIMER_NORESTART;
    }
#[no_mangle]
pub unsafe extern "C" fn set_cpu_itimer(tsk: *mut task_struct, clock_id: c_uint, value: *mut itimerspec64, ovalue: *mut itimerspec64) {
    u64 oval, nval, ointerval, ninterval;
    let mut it = &tsk.signal.it[clock_id];
    nval = timespec64_to_ns(&value.it_value);
    ninterval = timespec64_to_ns(&value.it_interval);
    spin_lock_irq(&tsk.sighand.siglock);
    oval = it.expires;
    ointerval = it.incr;
    if (oval || nval) {
    if (nval > 0) {
    nval += TICK_NSEC;
    }
    set_process_cpu_timer(tsk, clock_id, &nval, &oval);
    }
    it.expires = nval;
    it.incr = ninterval;
    trace_itimer_state(clock_id == CPUCLOCK_VIRT ?
    ITIMER_VIRTUAL : ITIMER_PROF, value, nval);
    spin_unlock_irq(&tsk.sighand.siglock);
    if (ovalue) {
    ovalue.it_value = ns_to_timespec64(oval);
    ovalue.it_interval = ns_to_timespec64(ointerval);
    }
    }
//
// Returns true if the timeval is in canonical form
//

    (((t).tv_sec >= 0) && (((unsigned long) (t).tv_usec) < USEC_PER_SEC))
#[no_mangle]
pub unsafe extern "C" fn do_setitimer(which: c_int, value: *mut itimerspec64, ovalue: *mut itimerspec64) -> c_int {
    let mut tsk = current;
pub static mut timer: *mut c_void = core::ptr::null_mut();
    let mut expires;
    match (which) {
    ITIMER_REAL => {
// label;
    spin_lock_irq(&tsk.sighand.siglock);
    timer = &tsk.signal.real_timer;
    if (ovalue) {
    ovalue.it_value = itimer_get_remtime(timer);
    ovalue.it_interval
    = ktime_to_timespec64(tsk.signal.it_real_incr);
    }
// We are sharing ->siglock with it_real_fn()
    if (hrtimer_try_to_cancel(timer) < 0) {
    spin_unlock_irq(&tsk.sighand.siglock);
    hrtimer_cancel_wait_running(timer);
// goto;
    }
    expires = timespec64_to_ktime(value.it_value);
    if (expires != 0) {
    tsk.signal.it_real_incr =
    timespec64_to_ktime(value.it_interval);
    hrtimer_start(timer, expires, HRTIMER_MODE_REL);
    } else {
    tsk.signal.it_real_incr = 0;
    }
    trace_itimer_state(ITIMER_REAL, value, 0);
    spin_unlock_irq(&tsk.sighand.siglock);
    // break;
    }
    ITIMER_VIRTUAL => {
    set_cpu_itimer(tsk, CPUCLOCK_VIRT, value, ovalue);
    // break;
    }
    ITIMER_PROF => {
    set_cpu_itimer(tsk, CPUCLOCK_PROF, value, ovalue);
    // break;
    }
    _ => {
    return -EINVAL;
    }
    }
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn clear_itimer() {
pub static mut v: itimerspec64 = 0;
    let mut i = 0;
    for (i = 0; i < 3; i++) {
    do_setitimer(i, &v, core::ptr::null_mut());
    }
    }

//
// alarm_setitimer - set alarm in seconds
//
// @seconds:	number of seconds until alarm
// 0 disables the alarm
//
// Returns the remaining time in seconds of a pending timer or 0 when
// the timer is not active.
//
// On 32 bit machines the seconds value is limited to (INT_MAX/2) to avoid
// negative timeval settings which would cause immediate expiry.
//
#[no_mangle]
unsafe extern "C" fn alarm_setitimer(seconds: c_uint) -> c_uint {
    struct itimerspec64 it_new, it_old;

    if (seconds > INT_MAX) {
    seconds = INT_MAX;
    }

    it_new.it_value.tv_sec = seconds;
    it_new.it_value.tv_nsec = 0;
    it_new.it_interval.tv_sec = it_new.it_interval.tv_nsec = 0;
    do_setitimer(ITIMER_REAL, &it_new, &it_old);
//
// We can't return 0 if we have an alarm pending ...  And we'd
// better return too much than too little anyway
//
    if ((!it_old.it_value.tv_sec && it_old.it_value.tv_nsec) ||
    it_old.it_value.tv_nsec >= (NSEC_PER_SEC / 2)) {
    it_old.it_value.tv_sec += 1;
    }
    return it_old.it_value.tv_sec;
    }
//
// For backwards compatibility?  This can be done in libc so Alpha
// and all newer ports shouldn't need it.
//
#[no_mangle]
pub unsafe extern "C" fn sys_alarm(seconds: usize) -> c_long {
    return alarm_setitimer(seconds);
    }

#[no_mangle]
unsafe extern "C" fn get_itimerval(o: *mut itimerspec64, i: *const __kernel_old_itimerval ) -> c_int {
pub static mut v: usize = 0;
    if (copy_from_user(&v, i, sizeof!(__kernel_old_itimerval))) {
    return -EFAULT;
    }
// Validate the timevals in value.
    if (!timeval_valid(&v.it_value) ||
    !timeval_valid(&v.it_interval)) {
    return -EINVAL;
    }
    o.it_interval.tv_sec = v.it_interval.tv_sec;
    o.it_interval.tv_nsec = v.it_interval.tv_usec * NSEC_PER_USEC;
    o.it_value.tv_sec = v.it_value.tv_sec;
    o.it_value.tv_nsec = v.it_value.tv_usec * NSEC_PER_USEC;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn sys_setitimer(which: usize, value: usize, ovalue: usize) -> c_long {
    struct itimerspec64 set_buffer, get_buffer;
    let mut error = 0;
    if (value) {
    error = get_itimerval(&set_buffer, value);
    if (error) {
    return error;
    }
    } else {
    memset(&set_buffer, 0, sizeof!(set_buffer));
    printk_once("%s calls setitimer() with new_value core::ptr::null_mut() pointer."
    " Misfeature support will be removed\n",
    current.comm);
    }
    error = do_setitimer(which, &set_buffer, ovalue ? &get_buffer : core::ptr::null_mut());
    if (error || !ovalue) {
    return error;
    }
    if (put_itimerval(ovalue, &get_buffer)) {
    return -EFAULT;
    }
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn get_old_itimerval32(o: *mut itimerspec64, i: *const old_itimerval32 ) -> c_int {
pub static mut v32: usize = 0;
    if (copy_from_user(&v32, i, sizeof!(old_itimerval32))) {
    return -EFAULT;
    }
// Validate the timevals in value.
    if (!timeval_valid(&v32.it_value) ||
    !timeval_valid(&v32.it_interval)) {
    return -EINVAL;
    }
    o.it_interval.tv_sec = v32.it_interval.tv_sec;
    o.it_interval.tv_nsec = v32.it_interval.tv_usec * NSEC_PER_USEC;
    o.it_value.tv_sec = v32.it_value.tv_sec;
    o.it_value.tv_nsec = v32.it_value.tv_usec * NSEC_PER_USEC;
    return 0;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: sys_setitimer
pub unsafe extern "C" fn sys_setitimer_dup(which: usize, value: usize, ovalue: usize) -> c_long {
    struct itimerspec64 set_buffer, get_buffer;
    let mut error = 0;
    if (value) {
    error = get_old_itimerval32(&set_buffer, value);
    if (error) {
    return error;
    }
    } else {
    memset(&set_buffer, 0, sizeof!(set_buffer));
    printk_once("%s calls setitimer() with new_value core::ptr::null_mut() pointer."
    " Misfeature support will be removed\n",
    current.comm);
    }
    error = do_setitimer(which, &set_buffer, ovalue ? &get_buffer : core::ptr::null_mut());
    if (error || !ovalue) {
    return error;
    }
    if (put_old_itimerval32(ovalue, &get_buffer)) {
    return -EFAULT;
    }
    return 0;
    }