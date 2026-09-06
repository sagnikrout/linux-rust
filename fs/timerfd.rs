//! Automatically rewritten from C to Rust
//! Source: fs/timerfd.c
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
// fs/timerfd.c
//
// Copyright (C) 2007  Davide Libenzi <davidel@xmailserver.org>
//
// Thanks to Thomas Gleixner for code reviews and useful comments.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timerfd_ctx {
    union {
    pub tmr: hrtimer,
    pub alarm: alarm,
    pub t: },
    pub tintv: ktime_t,
    pub moffs: ktime_t,
    pub wqh: wait_queue_head_t,
    pub ticks: u64,
    pub clockid: c_int,
    pub expired: short unsigned,
    pub /: *mut *mut short unsigned settime_flags; / to show in fdinfo,
    pub rcu: rcu_head,
    pub clist: list_head,
    pub cancel_lock: spinlock_t,
    pub might_cancel: bool,
}

    static LIST_HEAD(cancel_list);
    static DEFINE_SPINLOCK(cancel_lock);
#[no_mangle]
pub unsafe extern "C" fn isalarm(ctx: *mut timerfd_ctx) -> bool {
    static inline bool isalarm(struct timerfd_ctx *ctx)
    {
    return ctx.clockid == CLOCK_REALTIME_ALARM ||
    ctx.clockid == CLOCK_BOOTTIME_ALARM;
    }
#[no_mangle]
unsafe extern "C" fn __timerfd_triggered(ctx: *mut timerfd_ctx) {
    static void __timerfd_triggered(struct timerfd_ctx *ctx)
    {
    lockdep_assert_held(&ctx.wqh.lock);
    ctx.expired = 1;
    ctx.ticks++;
    wake_up_locked_poll(&ctx.wqh, EPOLLIN);
    }
//
// This gets called when the timer event triggers. We set the "expired"
// flag, but we do not re-arm the timer (in case it's necessary,
// tintv != 0) until the timer is accessed.
//
#[no_mangle]
unsafe extern "C" fn timerfd_triggered(ctx: *mut timerfd_ctx) {
    static void timerfd_triggered(struct timerfd_ctx *ctx)
    {
    guard(spinlock_irqsave)(&ctx.wqh.lock);
    __timerfd_triggered(ctx);
    }
#[no_mangle]
unsafe extern "C" fn timerfd_tmrproc(htmr: *mut hrtimer) -> enum hrtimer_restart {
    static enum hrtimer_restart timerfd_tmrproc(struct hrtimer *htmr)
    {
    struct timerfd_ctx *ctx = container_of(htmr, struct timerfd_ctx,
    t.tmr);
    timerfd_triggered(ctx);
    return HRTIMER_NORESTART;
    }
#[no_mangle]
unsafe extern "C" fn timerfd_alarmproc(alarm: *mut alarm, now: ktime_t) {
    static void timerfd_alarmproc(struct alarm *alarm, ktime_t now)
    {
    struct timerfd_ctx *ctx = container_of(alarm, struct timerfd_ctx,
    t.alarm);
    timerfd_triggered(ctx);
    }
//
// Called when the clock was set to cancel the timers in the cancel
// list. This will wake up processes waiting on these timers. The
// wake-up requires ctx->ticks to be non zero, therefore we increment
// it before calling wake_up_locked().
//
#[no_mangle]
pub unsafe extern "C" fn timerfd_clock_was_set() {
    void timerfd_clock_was_set(void)
    {
    let mut moffs: ktime_t = ktime_mono_to_real(0);
    struct timerfd_ctx *ctx;
    unsigned long flags;
    rcu_read_lock();
    list_for_each_entry_rcu(ctx, &cancel_list, clist) {
    if (!ctx.might_cancel)
    continue;
    spin_lock_irqsave(&ctx.wqh.lock, flags);
    if (ctx.moffs != moffs) {
    ctx.moffs = KTIME_MAX;
    ctx.ticks++;
    wake_up_locked_poll(&ctx.wqh, EPOLLIN);
    }
    spin_unlock_irqrestore(&ctx.wqh.lock, flags);
    }
    rcu_read_unlock();
    }
#[no_mangle]
unsafe extern "C" fn timerfd_resume_work(work: *mut work_struct) {
    static void timerfd_resume_work(struct work_struct *work)
    {
    timerfd_clock_was_set();
    }
    static DECLARE_WORK(timerfd_work, timerfd_resume_work);
//
// Invoked from timekeeping_resume(). Defer the actual update to work so
// timerfd_clock_was_set() runs in task context.
//
#[no_mangle]
pub unsafe extern "C" fn timerfd_resume() {
    void timerfd_resume(void)
    {
    schedule_work(&timerfd_work);
    }
#[no_mangle]
unsafe extern "C" fn __timerfd_remove_cancel(ctx: *mut timerfd_ctx) {
    static void __timerfd_remove_cancel(struct timerfd_ctx *ctx)
    {
    if (ctx.might_cancel) {
    ctx.might_cancel = false;
    spin_lock(&cancel_lock);
    list_del_rcu(&ctx.clist);
    spin_unlock(&cancel_lock);
    }
    }
#[no_mangle]
unsafe extern "C" fn timerfd_remove_cancel(ctx: *mut timerfd_ctx) {
    static void timerfd_remove_cancel(struct timerfd_ctx *ctx)
    {
    spin_lock(&ctx.cancel_lock);
    __timerfd_remove_cancel(ctx);
    spin_unlock(&ctx.cancel_lock);
    }
#[no_mangle]
unsafe extern "C" fn timerfd_canceled(ctx: *mut timerfd_ctx) -> bool {
    static bool timerfd_canceled(struct timerfd_ctx *ctx)
    {
    if (!ctx.might_cancel || ctx.moffs != KTIME_MAX)
    return false;
    ctx.moffs = ktime_mono_to_real(0);
    return true;
    }
#[no_mangle]
unsafe extern "C" fn timerfd_setup_cancel(ctx: *mut timerfd_ctx, flags: c_int) {
    static void timerfd_setup_cancel(struct timerfd_ctx *ctx, int flags)
    {
    spin_lock(&ctx.cancel_lock);
    if ((ctx.clockid == CLOCK_REALTIME ||
    ctx.clockid == CLOCK_REALTIME_ALARM) &&
    (flags & TFD_TIMER_ABSTIME) && (flags & TFD_TIMER_CANCEL_ON_SET)) {
    if (!ctx.might_cancel) {
    ctx.might_cancel = true;
    spin_lock(&cancel_lock);
    list_add_rcu(&ctx.clist, &cancel_list);
    spin_unlock(&cancel_lock);
    }
    } else {
    __timerfd_remove_cancel(ctx);
    }
    spin_unlock(&ctx.cancel_lock);
    }
#[no_mangle]
unsafe extern "C" fn timerfd_get_remaining(ctx: *mut timerfd_ctx) -> ktime_t {
    static ktime_t timerfd_get_remaining(struct timerfd_ctx *ctx)
    {
    ktime_t remaining;
    if (isalarm(ctx))
    remaining = alarm_expires_remaining(&ctx.t.alarm);
    else
    remaining = hrtimer_expires_remaining_adjusted(&ctx.t.tmr);
    return remaining < 0 ? 0: remaining;
    }
#[no_mangle]
unsafe extern "C" fn timerfd_alarm_start(ctx: *mut timerfd_ctx, exp: ktime_t, relative: bool) {
    static void timerfd_alarm_start(struct timerfd_ctx *ctx, ktime_t exp, bool relative)
    {
// Start the timer. If it's expired already, handle the callback.
    if (!alarm_start_timer(&ctx.t.alarm, exp, relative))
    __timerfd_triggered(ctx);
    }
#[no_mangle]
unsafe extern "C" fn timerfd_alarm_restart(ctx: *mut timerfd_ctx) -> u64 {
    static u64 timerfd_alarm_restart(struct timerfd_ctx *ctx)
    {
// -1 to account for ctx->ticks++ in __timerfd_triggered()
    let mut ticks: u64 = alarm_forward_now(&ctx.t.alarm, ctx.tintv) - 1;
    timerfd_alarm_start(ctx, alarm_get_expires(&ctx.t.alarm), false);
    return ticks;
    }
    static void timerfd_hrtimer_start(struct timerfd_ctx *ctx, ktime_t exp,
    const enum hrtimer_mode mode)
    {
// Start the timer. If it's expired already, handle the callback.
    if (!hrtimer_start_range_ns_user(&ctx.t.tmr, exp, 0, mode))
    __timerfd_triggered(ctx);
    }
#[no_mangle]
unsafe extern "C" fn timerfd_hrtimer_restart(ctx: *mut timerfd_ctx) -> u64 {
    static u64 timerfd_hrtimer_restart(struct timerfd_ctx *ctx)
    {
// -1 to account for ctx->ticks++ in __timerfd_triggered()
    let mut ticks: u64 = hrtimer_forward_now(&ctx.t.tmr, ctx.tintv) - 1;
    timerfd_hrtimer_start(ctx, hrtimer_get_expires(&ctx.t.tmr), HRTIMER_MODE_ABS);
    return ticks;
    }
#[no_mangle]
unsafe extern "C" fn timerfd_restart(ctx: *mut timerfd_ctx) -> u64 {
    static u64 timerfd_restart(struct timerfd_ctx *ctx)
    {
    if (isalarm(ctx))
    return timerfd_alarm_restart(ctx);
    return timerfd_hrtimer_restart(ctx);
    }
    static int timerfd_setup(struct timerfd_ctx *ctx, int flags,
    const struct itimerspec64 *ktmr)
    {
    let mut clockid: c_int = ctx.clockid;
    enum hrtimer_mode htmode;
    ktime_t texp;
    htmode = (flags & TFD_TIMER_ABSTIME) ? HRTIMER_MODE_ABS: HRTIMER_MODE_REL;
    texp = timespec64_to_ktime(ktmr.it_value);
    ctx.expired = 0;
    ctx.ticks = 0;
    ctx.tintv = timespec64_to_ktime(ktmr.it_interval);
    if (isalarm(ctx)) {
    alarm_init(&ctx.t.alarm,
    ctx.clockid == CLOCK_REALTIME_ALARM ?
    ALARM_REALTIME : ALARM_BOOTTIME,
    timerfd_alarmproc);
    } else {
    hrtimer_setup(&ctx.t.tmr, timerfd_tmrproc, clockid, htmode);
    }
    if (texp != 0) {
    if (flags & TFD_TIMER_ABSTIME)
    texp = timens_ktime_to_host(clockid, texp);
    if (isalarm(ctx))
    timerfd_alarm_start(ctx, texp, !(flags & TFD_TIMER_ABSTIME));
    else
    timerfd_hrtimer_start(ctx, texp, htmode);
    if (timerfd_canceled(ctx))
    return -ECANCELED;
    }
    ctx.settime_flags = flags & TFD_SETTIME_FLAGS;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn timerfd_release(inode: *mut inode, file: *mut file) -> c_int {
    static int timerfd_release(struct inode *inode, struct file *file)
    {
    struct timerfd_ctx *ctx = file.private_data;
    timerfd_remove_cancel(ctx);
    if (isalarm(ctx))
    alarm_cancel(&ctx.t.alarm);
    else
    hrtimer_cancel(&ctx.t.tmr);
    kfree_rcu(ctx, rcu);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn timerfd_poll(file: *mut file, wait: *mut poll_table) -> __poll_t {
    static __poll_t timerfd_poll(struct file *file, poll_table *wait)
    {
    struct timerfd_ctx *ctx = file.private_data;
    let mut events: __poll_t = 0;
    unsigned long flags;
    poll_wait(file, &ctx.wqh, wait);
    spin_lock_irqsave(&ctx.wqh.lock, flags);
    if (ctx.ticks)
    events |= EPOLLIN;
    spin_unlock_irqrestore(&ctx.wqh.lock, flags);
    return events;
    }
#[no_mangle]
unsafe extern "C" fn timerfd_read_iter(iocb: *mut kiocb, to: *mut iov_iter) -> isize {
    static ssize_t timerfd_read_iter(struct kiocb *iocb, struct iov_iter *to)
    {
    struct file *file = iocb.ki_filp;
    struct timerfd_ctx *ctx = file.private_data;
    ssize_t res;
    let mut ticks: u64 = 0;
    if (iov_iter_count(to) < sizeof(ticks))
    return -EINVAL;
    spin_lock_irq(&ctx.wqh.lock);
    if (file.f_flags & O_NONBLOCK || iocb.ki_flags & IOCB_NOWAIT)
    res = -EAGAIN;
    else
    res = wait_event_interruptible_locked_irq(ctx.wqh, ctx.ticks);
//
// If clock has changed, we do not care about the
// ticks and we do not rearm the timer. Userspace must
// reevaluate anyway.
//
    if (timerfd_canceled(ctx)) {
    ctx.ticks = 0;
    ctx.expired = 0;
    res = -ECANCELED;
    }
    if (ctx.ticks) {
    let mut expired: c_uint = ctx.expired;
    ticks = ctx.ticks;
    ctx.expired = 0;
    ctx.ticks = 0;
//
// If tintv != 0, this is a periodic timer that needs to be
// re-armed. We avoid doing it in the timer callback to avoid
// DoS attacks specifying a very short timer period.
//
    if (expired && ctx.tintv)
    ticks += timerfd_restart(ctx);
    }
    spin_unlock_irq(&ctx.wqh.lock);
    if (ticks) {
    res = copy_to_iter(&ticks, sizeof(ticks), to);
    if (!res)
    res = -EFAULT;
    }
    return res;
    }

#[no_mangle]
unsafe extern "C" fn timerfd_show(m: *mut seq_file, file: *mut file) {
    static void timerfd_show(struct seq_file *m, struct file *file)
    {
    struct timerfd_ctx *ctx = file.private_data;
    struct timespec64 value, interval;
    spin_lock_irq(&ctx.wqh.lock);
    value = ktime_to_timespec64(timerfd_get_remaining(ctx));
    interval = ktime_to_timespec64(ctx.tintv);
    spin_unlock_irq(&ctx.wqh.lock);
    seq_printf(m,
    "clockid: %d\n"
    "ticks: %llu\n"
    "settime flags: 0%o\n"
    "it_value: (%llu, %llu)\n"
    "it_interval: (%llu, %llu)\n",
    ctx.clockid,
    (unsigned long long)ctx.ticks,
    ctx.settime_flags,
    (unsigned long long)value.tv_sec,
    (unsigned long long)value.tv_nsec,
    (unsigned long long)interval.tv_sec,
    (unsigned long long)interval.tv_nsec);
    }

#[no_mangle]
unsafe extern "C" fn timerfd_ioctl(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    static long timerfd_ioctl(struct file *file, unsigned int cmd, unsigned long arg)
    {
    struct timerfd_ctx *ctx = file.private_data;
    let mut ret: c_int = 0;
    switch (cmd) {
    case TFD_IOC_SET_TICKS: {
    u64 ticks;
    if (copy_from_user(&ticks, (u64 __user *)arg, sizeof(ticks)))
    return -EFAULT;
    if (!ticks)
    return -EINVAL;
    spin_lock_irq(&ctx.wqh.lock);
    if (!timerfd_canceled(ctx)) {
    ctx.ticks = ticks;
    wake_up_locked_poll(&ctx.wqh, EPOLLIN);
    } else
    ret = -ECANCELED;
    spin_unlock_irq(&ctx.wqh.lock);
    break;
    }
    default:
    ret = -ENOTTY;
    break;
    }
    return ret;
    }

    static const struct file_operations timerfd_fops = {
    .release	= timerfd_release,
    .poll		= timerfd_poll,
    .read_iter	= timerfd_read_iter,
    .llseek		= noop_llseek,
    .show_fdinfo	= timerfd_show,
    .unlocked_ioctl	= timerfd_ioctl,
    };
    SYSCALL_DEFINE2(timerfd_create, int, clockid, int, flags)
    {
    struct timerfd_ctx *ctx __free(kfree) = core::ptr::null_mut();
    int ret;
// Check the TFD_* constants for consistency.
    BUILD_BUG_ON(TFD_CLOEXEC != O_CLOEXEC);
    BUILD_BUG_ON(TFD_NONBLOCK != O_NONBLOCK);
    if ((flags & ~TFD_CREATE_FLAGS) ||
    (clockid != CLOCK_MONOTONIC &&
    clockid != CLOCK_REALTIME &&
    clockid != CLOCK_REALTIME_ALARM &&
    clockid != CLOCK_BOOTTIME &&
    clockid != CLOCK_BOOTTIME_ALARM))
    return -EINVAL;
    if ((clockid == CLOCK_REALTIME_ALARM ||
    clockid == CLOCK_BOOTTIME_ALARM) &&
    !capable(CAP_WAKE_ALARM))
    return -EPERM;
    ctx = kzalloc_obj(*ctx);
    if (!ctx)
    return -ENOMEM;
    init_waitqueue_head(&ctx.wqh);
    spin_lock_init(&ctx.cancel_lock);
    ctx.clockid = clockid;
    if (isalarm(ctx))
    alarm_init(&ctx.t.alarm,
    ctx.clockid == CLOCK_REALTIME_ALARM ?
    ALARM_REALTIME : ALARM_BOOTTIME,
    timerfd_alarmproc);
    else
    hrtimer_setup(&ctx.t.tmr, timerfd_tmrproc, clockid, HRTIMER_MODE_ABS);
    ctx.moffs = ktime_mono_to_real(0);
    ret = FD_ADD(flags & TFD_SHARED_FCNTL_FLAGS,
    anon_inode_getfile_fmode("[timerfd]", &timerfd_fops, ctx,
    O_RDWR | (flags & TFD_SHARED_FCNTL_FLAGS),
    FMODE_NOWAIT));
    if (ret >= 0)
    retain_and_null_ptr(ctx);
    return ret;
    }
    static int do_timerfd_settime(int ufd, int flags,
    const struct itimerspec64 *new,
    struct itimerspec64 *old)
    {
    struct timerfd_ctx *ctx;
    int ret;
    if ((flags & ~TFD_SETTIME_FLAGS) ||
    !itimerspec64_valid(new))
    return -EINVAL;
    CLASS(fd, f)(ufd);
    if (fd_empty(f))
    return -EBADF;
    if (fd_file(f).f_op != &timerfd_fops)
    return -EINVAL;
    ctx = fd_file(f).private_data;
    if (isalarm(ctx) && !capable(CAP_WAKE_ALARM))
    return -EPERM;
    timerfd_setup_cancel(ctx, flags);
//
// We need to stop the existing timer before reprogramming
// it to the new values.
//
    for (;;) {
    spin_lock_irq(&ctx.wqh.lock);
    if (isalarm(ctx)) {
    if (alarm_try_to_cancel(&ctx.t.alarm) >= 0)
    break;
    } else {
    if (hrtimer_try_to_cancel(&ctx.t.tmr) >= 0)
    break;
    }
    spin_unlock_irq(&ctx.wqh.lock);
    if (isalarm(ctx))
    hrtimer_cancel_wait_running(&ctx.t.alarm.timer);
    else
    hrtimer_cancel_wait_running(&ctx.t.tmr);
    }
//
// If the timer is expired and it's periodic, we need to advance it
// because the caller may want to know the previous expiration time.
// We do not update "ticks" and "expired" since the timer will be
// re-programmed again in the following timerfd_setup() call.
//
    if (ctx.expired && ctx.tintv) {
    if (isalarm(ctx))
    alarm_forward_now(&ctx.t.alarm, ctx.tintv);
    else
    hrtimer_forward_now(&ctx.t.tmr, ctx.tintv);
    }
    old.it_value = ktime_to_timespec64(timerfd_get_remaining(ctx));
    old.it_interval = ktime_to_timespec64(ctx.tintv);
//
// Re-program the timer to the new value ...
//
    ret = timerfd_setup(ctx, flags, new);
    spin_unlock_irq(&ctx.wqh.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn do_timerfd_gettime(ufd: c_int, t: *mut itimerspec64) -> c_int {
    static int do_timerfd_gettime(int ufd, struct itimerspec64 *t)
    {
    struct timerfd_ctx *ctx;
    CLASS(fd, f)(ufd);
    if (fd_empty(f))
    return -EBADF;
    if (fd_file(f).f_op != &timerfd_fops)
    return -EINVAL;
    ctx = fd_file(f).private_data;
    spin_lock_irq(&ctx.wqh.lock);
    if (ctx.expired && ctx.tintv) {
    ctx.expired = 0;
    ctx.ticks += timerfd_restart(ctx);
    }
    t.it_value = ktime_to_timespec64(timerfd_get_remaining(ctx));
    t.it_interval = ktime_to_timespec64(ctx.tintv);
    spin_unlock_irq(&ctx.wqh.lock);
    return 0;
    }
    SYSCALL_DEFINE4(timerfd_settime, int, ufd, int, flags,
    const struct __kernel_itimerspec __user *, utmr,
    struct __kernel_itimerspec __user *, otmr)
    {
    struct itimerspec64 new, old;
    int ret;
    if (get_itimerspec64(&new, utmr))
    return -EFAULT;
    ret = do_timerfd_settime(ufd, flags, &new, &old);
    if (ret)
    return ret;
    if (otmr && put_itimerspec64(&old, otmr))
    return -EFAULT;
    return ret;
    }
    SYSCALL_DEFINE2(timerfd_gettime, int, ufd, struct __kernel_itimerspec __user *, otmr)
    {
    struct itimerspec64 kotmr;
    let mut ret: c_int = do_timerfd_gettime(ufd, &kotmr);
    if (ret)
    return ret;
    return put_itimerspec64(&kotmr, otmr) ? -EFAULT : 0;
    }

    SYSCALL_DEFINE4(timerfd_settime32, int, ufd, int, flags,
    const struct old_itimerspec32 __user *, utmr,
    struct old_itimerspec32 __user *, otmr)
    {
    struct itimerspec64 new, old;
    int ret;
    if (get_old_itimerspec32(&new, utmr))
    return -EFAULT;
    ret = do_timerfd_settime(ufd, flags, &new, &old);
    if (ret)
    return ret;
    if (otmr && put_old_itimerspec32(&old, otmr))
    return -EFAULT;
    return ret;
    }
    SYSCALL_DEFINE2(timerfd_gettime32, int, ufd,
    struct old_itimerspec32 __user *, otmr)
    {
    struct itimerspec64 kotmr;
    let mut ret: c_int = do_timerfd_gettime(ufd, &kotmr);
    if (ret)
    return ret;
    return put_old_itimerspec32(&kotmr, otmr) ? -EFAULT : 0;
    }
