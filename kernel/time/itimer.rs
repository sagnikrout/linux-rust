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
    static struct timespec64 itimer_get_remtime(struct hrtimer *timer)
    {
    let mut rem: ktime_t = __hrtimer_get_remaining(timer, true);
//
// Racy but safe: if the itimer expires after the above
// hrtimer_get_remtime() call but before this condition
// then we return 0 - which is correct.
//
    if (hrtimer_active(timer)) {
    if (rem <= 0)
    rem = NSEC_PER_USEC;
    } else
    rem = 0;
    return ktime_to_timespec64(rem);
    }
    static void get_cpu_itimer(struct task_struct *tsk, unsigned int clock_id,
    struct itimerspec64 *const value)
    {
    u64 val, interval;
    struct cpu_itimer *it = &tsk.signal.it[clock_id];
    spin_lock_irq(&tsk.sighand.siglock);
    val = it.expires;
    interval = it.incr;
    if (val) {
    u64 t, samples[CPUCLOCK_MAX];
    thread_group_sample_cputime(tsk, samples);
    t = samples[clock_id];
    if (val < t)
// about to fire
    val = TICK_NSEC;
    else
    val -= t;
    }
    spin_unlock_irq(&tsk.sighand.siglock);
    value.it_value = ns_to_timespec64(val);
    value.it_interval = ns_to_timespec64(interval);
    }
#[no_mangle]
unsafe extern "C" fn do_getitimer(which: c_int, value: *mut itimerspec64) -> c_int {
    static int do_getitimer(int which, struct itimerspec64 *value)
    {
    struct task_struct *tsk = current;
    switch (which) {
    case ITIMER_REAL:
    spin_lock_irq(&tsk.sighand.siglock);
    value.it_value = itimer_get_remtime(&tsk.signal.real_timer);
    value.it_interval =
    ktime_to_timespec64(tsk.signal.it_real_incr);
    spin_unlock_irq(&tsk.sighand.siglock);
    break;
    case ITIMER_VIRTUAL:
    get_cpu_itimer(tsk, CPUCLOCK_VIRT, value);
    break;
    case ITIMER_PROF:
    get_cpu_itimer(tsk, CPUCLOCK_PROF, value);
    break;
    default:
    return(-EINVAL);
    }
    return 0;
    }
    static int put_itimerval(struct __kernel_old_itimerval __user *o,
    const struct itimerspec64 *i)
    {
    let mut v: __kernel_old_itimerval = {};
    v.it_interval.tv_sec = i.it_interval.tv_sec;
    v.it_interval.tv_usec = i.it_interval.tv_nsec / NSEC_PER_USEC;
    v.it_value.tv_sec = i.it_value.tv_sec;
    v.it_value.tv_usec = i.it_value.tv_nsec / NSEC_PER_USEC;
    return copy_to_user(o, &v, sizeof(struct __kernel_old_itimerval)) ? -EFAULT : 0;
    }
    SYSCALL_DEFINE2(getitimer, int, which, struct __kernel_old_itimerval __user *, value)
    {
    struct itimerspec64 get_buffer;
    let mut error: c_int = do_getitimer(which, &get_buffer);
    if (!error && put_itimerval(value, &get_buffer))
    error = -EFAULT;
    return error;
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct old_itimerval32 {
    pub it_interval: old_timeval32,
    pub it_value: old_timeval32,
}

    static int put_old_itimerval32(struct old_itimerval32 __user *o,
    const struct itimerspec64 *i)
    {
    struct old_itimerval32 v32;
    v32.it_interval.tv_sec = i.it_interval.tv_sec;
    v32.it_interval.tv_usec = i.it_interval.tv_nsec / NSEC_PER_USEC;
    v32.it_value.tv_sec = i.it_value.tv_sec;
    v32.it_value.tv_usec = i.it_value.tv_nsec / NSEC_PER_USEC;
    return copy_to_user(o, &v32, sizeof(struct old_itimerval32)) ? -EFAULT : 0;
    }
    COMPAT_SYSCALL_DEFINE2(getitimer, int, which,
    struct old_itimerval32 __user *, value)
    {
    struct itimerspec64 get_buffer;
    let mut error: c_int = do_getitimer(which, &get_buffer);
    if (!error && put_old_itimerval32(value, &get_buffer))
    error = -EFAULT;
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
    void posixtimer_rearm_itimer(struct task_struct *tsk)
    {
    struct hrtimer *tmr = &tsk.signal.real_timer;
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
    enum hrtimer_restart it_real_fn(struct hrtimer *timer)
    {
    struct signal_struct *sig =
    container_of(timer, struct signal_struct, real_timer);
    struct pid *leader_pid = sig.pids[PIDTYPE_TGID];
    trace_itimer_expire(ITIMER_REAL, leader_pid, 0);
    kill_pid_info(SIGALRM, SEND_SIG_PRIV, leader_pid);
    return HRTIMER_NORESTART;
    }
    static void set_cpu_itimer(struct task_struct *tsk, unsigned int clock_id,
    const struct itimerspec64 *const value,
    struct itimerspec64 *const ovalue)
    {
    u64 oval, nval, ointerval, ninterval;
    struct cpu_itimer *it = &tsk.signal.it[clock_id];
    nval = timespec64_to_ns(&value.it_value);
    ninterval = timespec64_to_ns(&value.it_interval);
    spin_lock_irq(&tsk.sighand.siglock);
    oval = it.expires;
    ointerval = it.incr;
    if (oval || nval) {
    if (nval > 0)
    nval += TICK_NSEC;
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
    static int do_setitimer(int which, struct itimerspec64 *value,
    struct itimerspec64 *ovalue)
    {
    struct task_struct *tsk = current;
    struct hrtimer *timer;
    ktime_t expires;
    switch (which) {
    case ITIMER_REAL:
    again:
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
    goto again;
    }
    expires = timespec64_to_ktime(value.it_value);
    if (expires != 0) {
    tsk.signal.it_real_incr =
    timespec64_to_ktime(value.it_interval);
    hrtimer_start(timer, expires, HRTIMER_MODE_REL);
    } else
    tsk.signal.it_real_incr = 0;
    trace_itimer_state(ITIMER_REAL, value, 0);
    spin_unlock_irq(&tsk.sighand.siglock);
    break;
    case ITIMER_VIRTUAL:
    set_cpu_itimer(tsk, CPUCLOCK_VIRT, value, ovalue);
    break;
    case ITIMER_PROF:
    set_cpu_itimer(tsk, CPUCLOCK_PROF, value, ovalue);
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn clear_itimer() {
    void clear_itimer(void)
    {
    let mut v: itimerspec64 = {};
    int i;
    for (i = 0; i < 3; i++)
    do_setitimer(i, &v, core::ptr::null_mut());
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
    static unsigned int alarm_setitimer(unsigned int seconds)
    {
    struct itimerspec64 it_new, it_old;

    if (seconds > INT_MAX)
    seconds = INT_MAX;

    it_new.it_value.tv_sec = seconds;
    it_new.it_value.tv_nsec = 0;
    it_new.it_interval.tv_sec = it_new.it_interval.tv_nsec = 0;
    do_setitimer(ITIMER_REAL, &it_new, &it_old);
//
// We can't return 0 if we have an alarm pending ...  And we'd
// better return too much than too little anyway
//
    if ((!it_old.it_value.tv_sec && it_old.it_value.tv_nsec) ||
    it_old.it_value.tv_nsec >= (NSEC_PER_SEC / 2))
    it_old.it_value.tv_sec++;
    return it_old.it_value.tv_sec;
    }
//
// For backwards compatibility?  This can be done in libc so Alpha
// and all newer ports shouldn't need it.
//
    SYSCALL_DEFINE1(alarm, unsigned int, seconds)
    {
    return alarm_setitimer(seconds);
    }

#[no_mangle]
unsafe extern "C" fn get_itimerval(o: *mut itimerspec64, i: *const __kernel_old_itimerval __user) -> c_int {
    static int get_itimerval(struct itimerspec64 *o, const struct __kernel_old_itimerval __user *i)
    {
    struct __kernel_old_itimerval v;
    if (copy_from_user(&v, i, sizeof(struct __kernel_old_itimerval)))
    return -EFAULT;
// Validate the timevals in value.
    if (!timeval_valid(&v.it_value) ||
    !timeval_valid(&v.it_interval))
    return -EINVAL;
    o.it_interval.tv_sec = v.it_interval.tv_sec;
    o.it_interval.tv_nsec = v.it_interval.tv_usec * NSEC_PER_USEC;
    o.it_value.tv_sec = v.it_value.tv_sec;
    o.it_value.tv_nsec = v.it_value.tv_usec * NSEC_PER_USEC;
    return 0;
    }
    SYSCALL_DEFINE3(setitimer, int, which, struct __kernel_old_itimerval __user *, value,
    struct __kernel_old_itimerval __user *, ovalue)
    {
    struct itimerspec64 set_buffer, get_buffer;
    int error;
    if (value) {
    error = get_itimerval(&set_buffer, value);
    if (error)
    return error;
    } else {
    memset(&set_buffer, 0, sizeof(set_buffer));
    printk_once(KERN_WARNING "%s calls setitimer() with new_value core::ptr::null_mut() pointer."
    " Misfeature support will be removed\n",
    current.comm);
    }
    error = do_setitimer(which, &set_buffer, ovalue ? &get_buffer : core::ptr::null_mut());
    if (error || !ovalue)
    return error;
    if (put_itimerval(ovalue, &get_buffer))
    return -EFAULT;
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn get_old_itimerval32(o: *mut itimerspec64, i: *const old_itimerval32 __user) -> c_int {
    static int get_old_itimerval32(struct itimerspec64 *o, const struct old_itimerval32 __user *i)
    {
    struct old_itimerval32 v32;
    if (copy_from_user(&v32, i, sizeof(struct old_itimerval32)))
    return -EFAULT;
// Validate the timevals in value.
    if (!timeval_valid(&v32.it_value) ||
    !timeval_valid(&v32.it_interval))
    return -EINVAL;
    o.it_interval.tv_sec = v32.it_interval.tv_sec;
    o.it_interval.tv_nsec = v32.it_interval.tv_usec * NSEC_PER_USEC;
    o.it_value.tv_sec = v32.it_value.tv_sec;
    o.it_value.tv_nsec = v32.it_value.tv_usec * NSEC_PER_USEC;
    return 0;
    }
    COMPAT_SYSCALL_DEFINE3(setitimer, int, which,
    struct old_itimerval32 __user *, value,
    struct old_itimerval32 __user *, ovalue)
    {
    struct itimerspec64 set_buffer, get_buffer;
    int error;
    if (value) {
    error = get_old_itimerval32(&set_buffer, value);
    if (error)
    return error;
    } else {
    memset(&set_buffer, 0, sizeof(set_buffer));
    printk_once(KERN_WARNING "%s calls setitimer() with new_value core::ptr::null_mut() pointer."
    " Misfeature support will be removed\n",
    current.comm);
    }
    error = do_setitimer(which, &set_buffer, ovalue ? &get_buffer : core::ptr::null_mut());
    if (error || !ovalue)
    return error;
    if (put_old_itimerval32(ovalue, &get_buffer))
    return -EFAULT;
    return 0;
    }
