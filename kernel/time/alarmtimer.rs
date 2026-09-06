//! Automatically rewritten from C to Rust
//! Source: kernel/time/alarmtimer.c
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
// Alarmtimer interface
//
// This interface provides a timer which is similar to hrtimers,
// but triggers a RTC alarm if the box is suspend.
//
// This interface is influenced by the Android RTC Alarm timer
// interface.
//
// Copyright (C) 2010 IBM Corporation
//
// Author: John Stultz <john.stultz@linaro.org>
//

// Macro flag: #define CREATE_TRACE_POINTS

//
// struct alarm_base - Alarm timer bases
// @lock:		Lock for synchronized access to the base
// @timerqueue:		Timerqueue head managing the list of events
// @get_ktime:		Function to read the time correlating to the base
// @get_timespec:	Function to read the namespace time correlating to the base
// @base_clockid:	clockid for the base
//
    static struct alarm_base {
    let mut lock;
pub static mut timerqueue: usize = 0;
    ktime_t			(*get_ktime)(void);
    void			(*get_timespec)(timespec64 *tp);
    let mut base_clockid;
    } alarm_bases[ALARM_NUMTYPE];

// freezer information to handle clock_nanosleep triggered wakeups
    static enum alarmtimer_type freezer_alarmtype;
    static ktime_t freezer_expires;
    static ktime_t freezer_delta;
pub static mut freezer_delta_lock: usize = 0;

// rtc timer and device for setting alarm wakeups at suspend
pub static mut rtctimer: usize = 0;
pub static mut rtcdev: *mut c_void = core::ptr::null_mut();
pub static mut rtcdev_lock: usize = 0;
//
// alarmtimer_get_rtcdev - Return selected rtcdevice
//
// This function returns the rtc device to use for wakealarms.
//
#[no_mangle]
pub unsafe extern "C" fn alarmtimer_get_rtcdev() -> *mut c_void {
pub static mut ret: *mut c_void = core::ptr::null_mut();
    guard(spinlock_irqsave)(&rtcdev_lock);
    ret = rtcdev;
    return ret;
    }
    EXPORT_SYMBOL_GPL(alarmtimer_get_rtcdev);
#[no_mangle]
unsafe extern "C" fn alarmtimer_rtc_add_device(dev: *mut device) -> c_int {
    let mut rtc = to_rtc_device(dev);
pub static mut pdev: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    if (rtcdev) {
    return -EBUSY;
    }
    if (!test_bit(RTC_FEATURE_ALARM, rtc.features)) {
    return -1;
    }
    if (!device_may_wakeup(rtc.dev.parent)) {
    return -1;
    }
    pdev = platform_device_register_data(dev, "alarmtimer",
    PLATFORM_DEVID_AUTO, core::ptr::null_mut(), 0);
    if (!IS_ERR(pdev)) {
    device_init_wakeup(&pdev.dev, true);
    }
    scoped_guard(spinlock_irqsave, &rtcdev_lock) {
    if (!IS_ERR(pdev) && !rtcdev && try_module_get(rtc.owner)) {
    rtcdev = rtc;
// hold a reference so it doesn't go away
    get_device(dev);
    pdev = core::ptr::null_mut();
    } else {
    ret = -1;
    }
    }
    platform_device_unregister(pdev);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn alarmtimer_rtc_timer_init() {
    rtc_timer_init(&rtctimer, core::ptr::null_mut(), core::ptr::null_mut());
    }
pub static mut class_interface: usize = 0;
#[no_mangle]
unsafe extern "C" fn alarmtimer_rtc_interface_setup() -> c_int {
    alarmtimer_rtc_interface.class = &rtc_class;
    return class_interface_register(&alarmtimer_rtc_interface);
    }
#[no_mangle]
unsafe extern "C" fn alarmtimer_rtc_interface_remove() {
    class_interface_unregister(&alarmtimer_rtc_interface);
    }

#[no_mangle]
pub unsafe extern "C" fn alarmtimer_rtc_interface_setup() -> c_int { return 0; }
#[no_mangle]
pub unsafe extern "C" fn alarmtimer_rtc_interface_remove() { }
#[no_mangle]
#[no_mangle]
// duplicate fn: alarmtimer_rtc_timer_init
pub unsafe extern "C" fn alarmtimer_rtc_timer_init_dup() { }

//
// alarmtimer_enqueue - Adds an alarm timer to an alarm_base timerqueue
// @base: pointer to the base where the timer is being run
// @alarm: pointer to alarm being enqueued.
//
// Adds alarm to a alarm_base timerqueue
//
// Must hold base->lock when calling.
//
#[no_mangle]
unsafe extern "C" fn alarmtimer_enqueue(base: *mut alarm_base, alarm: *mut alarm) {
    if (alarm.state & ALARMTIMER_STATE_ENQUEUED) {
    timerqueue_del(&base.timerqueue, &alarm.node);
    }
    timerqueue_add(&base.timerqueue, &alarm.node);
    alarm.state |= ALARMTIMER_STATE_ENQUEUED;
    }
//
// alarmtimer_dequeue - Removes an alarm timer from an alarm_base timerqueue
// @base: pointer to the base where the timer is running
// @alarm: pointer to alarm being removed
//
// Removes alarm to a alarm_base timerqueue
//
// Must hold base->lock when calling.
//
#[no_mangle]
unsafe extern "C" fn alarmtimer_dequeue(base: *mut alarm_base, alarm: *mut alarm) {
    if (!(alarm.state & ALARMTIMER_STATE_ENQUEUED)) {
    return;
    }
    timerqueue_del(&base.timerqueue, &alarm.node);
    alarm.state &= ~ALARMTIMER_STATE_ENQUEUED;
    }
//
// alarmtimer_fired - Handles alarm hrtimer being fired.
// @timer: pointer to hrtimer being run
//
// When a alarm timer fires, this runs through the timerqueue to
// see which alarms expired, and runs those. If there are more alarm
// timers queued for the future, we set the hrtimer to fire when
// the next future alarm timer expires.
//
#[no_mangle]
unsafe extern "C" fn alarmtimer_fired(timer: *mut hrtimer) -> enum hrtimer_restart {
    let mut alarm = container_of!(timer, alarm, timer);
    let mut base = &alarm_bases[alarm.type];
    scoped_guard(spinlock_irqsave, &base.lock)
    alarmtimer_dequeue(base, alarm);
    if (alarm.function) {
    alarm.function(alarm, base.get_ktime());
    }
    trace_alarmtimer_fired(alarm, base.get_ktime());
    return HRTIMER_NORESTART;
    }
#[no_mangle]
pub unsafe extern "C" fn alarm_expires_remaining(alarm: *const alarm) -> ktime_t {
    let mut base = &alarm_bases[alarm.type];
    return ktime_sub(alarm.node.expires, base.get_ktime());
    }
    EXPORT_SYMBOL_GPL(alarm_expires_remaining);

//
// alarmtimer_suspend - Suspend time callback
// @dev: unused
//
// When we are going into suspend, we look through the bases
// to see which is the soonest timer to expire. We then
// set an rtc timer to fire that far into the future, which
// will wake us from suspend.
//
#[no_mangle]
unsafe extern "C" fn alarmtimer_suspend(dev: *mut device) -> c_int {
    ktime_t min, now, expires;
pub static mut rtc: *mut c_void = core::ptr::null_mut();
pub static mut tm: usize = 0;
    let mut i = 0;
    let mut ret = 0;
    let mut type = 0;
    scoped_guard(spinlock_irqsave, &freezer_delta_lock) {
    min = freezer_delta;
    expires = freezer_expires;
    type = freezer_alarmtype;
    freezer_delta = 0;
    }
    rtc = alarmtimer_get_rtcdev();
// If we have no rtcdev, just return
    if (!rtc) {
    return 0;
    }
// Find the soonest timer to expire
    while (i < ALARM_NUMTYPE) {
    let mut base = &alarm_bases[i];
pub static mut next: *mut c_void = core::ptr::null_mut();
    let mut next_expires;
    let mut delta;
    scoped_guard(spinlock_irqsave, &base.lock) {
    next = timerqueue_getnext(&base.timerqueue);
    if (next) {
    next_expires = next.expires;
    }
    }
    if (!next) {
    continue;
    }
    delta = ktime_sub(next_expires, base.get_ktime());
    if (!min || (delta < min)) {
    expires = next_expires;
    min = delta;
    type = i;
    }
    }
    if (min == 0) {
    return 0;
    }
    if (ktime_to_ns(min) < 2 * NSEC_PER_SEC) {
    pm_wakeup_event(dev, 2 * MSEC_PER_SEC);
    return -EBUSY;
    }
    trace_alarmtimer_suspend(expires, type);
// Setup an rtc timer to fire that far in the future
    rtc_timer_cancel(rtc, &rtctimer);
    rtc_read_time(rtc, &tm);
    now = rtc_tm_to_ktime(tm);
//
// If the RTC alarm timer only supports a limited time offset, set the
// alarm time to the maximum supported value.
// The system may wake up earlier (possibly much earlier) than expected
// when the alarmtimer runs. This is the best the kernel can do if
// the alarmtimer exceeds the time that the rtc device can be programmed
// for.
//
    min = rtc_bound_alarmtime(rtc, min);
    now = ktime_add(now, min);
// Set alarm, if in the past reject suspend briefly to handle
    ret = rtc_timer_start(rtc, &rtctimer, now, 0);
    if (ret < 0) {
    pm_wakeup_event(dev, MSEC_PER_SEC);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn alarmtimer_resume(dev: *mut device) -> c_int {
pub static mut rtc: *mut c_void = core::ptr::null_mut();
    rtc = alarmtimer_get_rtcdev();
    if (rtc) {
    rtc_timer_cancel(rtc, &rtctimer);
    }
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn alarmtimer_suspend(dev: *mut device) -> c_int {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn alarmtimer_resume(dev: *mut device) -> c_int {
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn __alarm_init(alarm: *mut alarm, type: alarmtimer_type) {
    timerqueue_init(&alarm.node);
    alarm.function = function;
    alarm.type = type;
    alarm.state = ALARMTIMER_STATE_INACTIVE;
    }
//
// alarm_init - Initialize an alarm structure
// @alarm: ptr to alarm to be initialized
// @type: the type of the alarm
// @function: callback that is run when the alarm fires
//
#[no_mangle]
pub unsafe extern "C" fn alarm_init(alarm: *mut alarm, type: alarmtimer_type) {
    hrtimer_setup(&alarm.timer, alarmtimer_fired, alarm_bases[type].base_clockid,
    HRTIMER_MODE_ABS);
    __alarm_init(alarm, type, function);
    }
    EXPORT_SYMBOL_GPL(alarm_init);
//
// alarm_start_timer - Sets an alarm to fire
// @alarm:	Pointer to alarm to set
// @expires:	Expiry time
// @relative:	True if @expires is relative
//
// Returns: True if the alarm was queued. False if it already expired
//
#[no_mangle]
pub unsafe extern "C" fn alarm_start_timer(alarm: *mut alarm, expires: ktime_t, relative: bool) -> bool {
    let mut base = &alarm_bases[alarm.type];
    if (relative) {
    expires = ktime_add_safe(expires, base.get_ktime());
    }
    trace_alarmtimer_start(alarm, base.get_ktime());
    guard(spinlock_irqsave)(&base.lock);
    alarm.node.expires = expires;
    alarmtimer_enqueue(base, alarm);
    if (!hrtimer_start_range_ns_user(&alarm.timer, expires, 0, HRTIMER_MODE_ABS)) {
    alarmtimer_dequeue(base, alarm);
    return false;
    }
    return true;
    }
    EXPORT_SYMBOL_GPL(alarm_start_timer);
//
// alarm_try_to_cancel - Tries to cancel an alarm timer
// @alarm: ptr to alarm to be canceled
//
// Returns 1 if the timer was canceled, 0 if it was not running,
// and -1 if the callback was running
//
#[no_mangle]
pub unsafe extern "C" fn alarm_try_to_cancel(alarm: *mut alarm) -> c_int {
    let mut base = &alarm_bases[alarm.type];
    let mut ret = 0;
    scoped_guard(spinlock_irqsave, &base.lock) {
    ret = hrtimer_try_to_cancel(&alarm.timer);
    if (ret >= 0) {
    alarmtimer_dequeue(base, alarm);
    }
    }
    trace_alarmtimer_cancel(alarm, base.get_ktime());
    return ret;
    }
    EXPORT_SYMBOL_GPL(alarm_try_to_cancel);
//
// alarm_cancel - Spins trying to cancel an alarm timer until it is done
// @alarm: ptr to alarm to be canceled
//
// Returns 1 if the timer was canceled, 0 if it was not active.
//
#[no_mangle]
pub unsafe extern "C" fn alarm_cancel(alarm: *mut alarm) -> c_int {
    for (;;) {
pub static mut ret: c_int = 0;
    if (ret >= 0) {
    return ret;
    }
    hrtimer_cancel_wait_running(&alarm.timer);
    }
    }
    EXPORT_SYMBOL_GPL(alarm_cancel);
#[no_mangle]
pub unsafe extern "C" fn alarm_forward(alarm: *mut alarm, now: ktime_t, interval: ktime_t) -> u64 {
pub static mut overrun: u64 = 1;
    let mut delta;
    delta = ktime_sub(now, alarm.node.expires);
    if (delta < 0) {
    return 0;
    }
    if (unlikely(delta >= interval)) {
pub static mut incr: i64 = 0;
    overrun = ktime_divns(delta, incr);
    alarm.node.expires = ktime_add_ns(alarm.node.expires,
    incr*overrun);
    if (alarm.node.expires > now) {
    return overrun;
    }
//
// This (and the ktime_add() below) is the
// correction for exact:
//
    overrun += 1;
    }
    alarm.node.expires = ktime_add_safe(alarm.node.expires, interval);
    return overrun;
    }
    EXPORT_SYMBOL_GPL(alarm_forward);
#[no_mangle]
pub unsafe extern "C" fn alarm_forward_now(alarm: *mut alarm, interval: ktime_t) -> u64 {
    let mut base = &alarm_bases[alarm.type];
    return alarm_forward(alarm, base.get_ktime(), interval);
    }
    EXPORT_SYMBOL_GPL(alarm_forward_now);

#[no_mangle]
unsafe extern "C" fn alarmtimer_freezerset(absexp: ktime_t, type: alarmtimer_type) {
pub static mut base: *mut c_void = core::ptr::null_mut();
    let mut delta;
    match (type) {
    ALARM_REALTIME => {
    base = &alarm_bases[ALARM_REALTIME];
    type = ALARM_REALTIME_FREEZER;
    // break;
    }
    ALARM_BOOTTIME => {
    base = &alarm_bases[ALARM_BOOTTIME];
    type = ALARM_BOOTTIME_FREEZER;
    // break;
    }
    _ => {
    WARN_ONCE(1, "Invalid alarm type: %d\n", type);
    return;
    }
    }
    delta = ktime_sub(absexp, base.get_ktime());
    guard(spinlock_irqsave)(&freezer_delta_lock);
    if (!freezer_delta || (delta < freezer_delta)) {
    freezer_delta = delta;
    freezer_expires = absexp;
    freezer_alarmtype = type;
    }
    }
//
// clock2alarm - helper that converts from clockid to alarmtypes
// @clockid: clockid.
//
#[no_mangle]
unsafe extern "C" fn clock2alarm(clockid: clockid_t) -> enum alarmtimer_type {
    if (clockid == CLOCK_REALTIME_ALARM) {
    return ALARM_REALTIME;
    }
    WARN_ON_ONCE!(clockid != CLOCK_BOOTTIME_ALARM);
    return ALARM_BOOTTIME;
    }
//
// alarm_handle_timer - Callback for posix timers
// @alarm: alarm that fired
// @now: time at the timer expiration
//
// Posix timer callback for expired alarm timers.
//
#[no_mangle]
unsafe extern "C" fn alarm_handle_timer(alarm: *mut alarm, now: ktime_t) {
    let mut ptr = container_of!(alarm, k_itimer, it.alarm.alarmtimer);
    guard(spinlock_irqsave)(&ptr.it_lock);
    posix_timer_queue_signal(ptr);
    }
//
// alarm_timer_rearm - Posix timer callback for rearming timer
// @timr:	Pointer to the posixtimer data struct
//
#[no_mangle]
unsafe extern "C" fn alarm_timer_rearm(timr: *mut k_itimer) -> bool {
    let mut alarm = &timr.it.alarm.alarmtimer;
    timr.it_overrun += alarm_forward_now(alarm, timr.it_interval);
    return alarm_start_timer(alarm, alarm.node.expires, false);
    }
//
// alarm_timer_forward - Posix timer callback for forwarding timer
// @timr:	Pointer to the posixtimer data struct
// @now:	Current time to forward the timer against
//
#[no_mangle]
unsafe extern "C" fn alarm_timer_forward(timr: *mut k_itimer, now: ktime_t) -> i64 {
    let mut alarm = &timr.it.alarm.alarmtimer;
    return alarm_forward(alarm, now, timr.it_interval);
    }
//
// alarm_timer_remaining - Posix timer callback to retrieve remaining time
// @timr:	Pointer to the posixtimer data struct
// @now:	Current time to calculate against
//
#[no_mangle]
unsafe extern "C" fn alarm_timer_remaining(timr: *mut k_itimer, now: ktime_t) -> ktime_t {
    let mut alarm = &timr.it.alarm.alarmtimer;
    return ktime_sub(alarm.node.expires, now);
    }
//
// alarm_timer_try_to_cancel - Posix timer callback to cancel a timer
// @timr:	Pointer to the posixtimer data struct
//
#[no_mangle]
unsafe extern "C" fn alarm_timer_try_to_cancel(timr: *mut k_itimer) -> c_int {
    return alarm_try_to_cancel(&timr.it.alarm.alarmtimer);
    }
//
// alarm_timer_wait_running - Posix timer callback to wait for a timer
// @timr:	Pointer to the posixtimer data struct
//
// Called from the core code when timer cancel detected that the callback
// is running. @timr is unlocked and rcu read lock is held to prevent it
// from being freed.
//
#[no_mangle]
unsafe extern "C" fn alarm_timer_wait_running(timr: *mut k_itimer) {
    hrtimer_cancel_wait_running(&timr.it.alarm.alarmtimer.timer);
    }
//
// alarm_timer_arm - Posix timer callback to arm a timer
// @timr:	Pointer to the posixtimer data struct
// @expires:	The new expiry time
// @absolute:	Expiry value is absolute time
// @sigev_none:	Posix timer does not deliver signals
//
#[no_mangle]
pub unsafe extern "C" fn alarm_timer_arm(timr: *mut k_itimer, expires: ktime_t, absolute: bool, sigev_none: bool) -> bool {
    let mut alarm = &timr.it.alarm.alarmtimer;
    let mut base = &alarm_bases[alarm.type];
    if (!absolute) {
    expires = ktime_add_safe(expires, base.get_ktime());
    }
//
// sigev_none needs to update the expires value and pretend
// that the timer is queued
//
    if (sigev_none) {
    alarm.node.expires = expires;
    return true;
    }
    return alarm_start_timer(&timr.it.alarm.alarmtimer, expires, false);
    }
//
// alarm_clock_getres - posix getres interface
// @which_clock: clockid
// @tp: timespec to fill
//
// Returns the granularity of underlying alarm base clock
//
#[no_mangle]
unsafe extern "C" fn alarm_clock_getres(which_clock: clockid_t, tp: *mut timespec64) -> c_int {
    if (!alarmtimer_get_rtcdev()) {
    return -EINVAL;
    }
    tp.tv_sec = 0;
    tp.tv_nsec = hrtimer_resolution;
    return 0;
    }
//
// alarm_clock_get_timespec - posix clock_get_timespec interface
// @which_clock: clockid
// @tp: timespec to fill.
//
// Provides the underlying alarm base time in a tasks time namespace.
//
#[no_mangle]
unsafe extern "C" fn alarm_clock_get_timespec(which_clock: clockid_t, tp: *mut timespec64) -> c_int {
    let mut base = &alarm_bases[clock2alarm(which_clock)];
    if (!alarmtimer_get_rtcdev()) {
    return -EINVAL;
    }
    base.get_timespec(tp);
    return 0;
    }
//
// alarm_clock_get_ktime - posix clock_get_ktime interface
// @which_clock: clockid
//
// Provides the underlying alarm base time in the root namespace.
//
#[no_mangle]
unsafe extern "C" fn alarm_clock_get_ktime(which_clock: clockid_t) -> ktime_t {
    let mut base = &alarm_bases[clock2alarm(which_clock)];
    if (!alarmtimer_get_rtcdev()) {
    return -EINVAL;
    }
    return base.get_ktime();
    }
//
// alarm_timer_create - posix timer_create interface
// @new_timer: k_itimer pointer to manage
//
// Initializes the k_itimer structure.
//
#[no_mangle]
unsafe extern "C" fn alarm_timer_create(new_timer: *mut k_itimer) -> c_int {
    enum  alarmtimer_type type;
    if (!alarmtimer_get_rtcdev()) {
    return -EOPNOTSUPP;
    }
    if (!capable(CAP_WAKE_ALARM)) {
    return -EPERM;
    }
    type = clock2alarm(new_timer.it_clock);
    alarm_init(&new_timer.it.alarm.alarmtimer, type, alarm_handle_timer);
    return 0;
    }
//
// alarmtimer_nsleep_wakeup - Wakeup function for alarm_timer_nsleep
// @alarm: ptr to alarm that fired
// @now: time at the timer expiration
//
// Wakes up the task that set the alarmtimer
//
#[no_mangle]
unsafe extern "C" fn alarmtimer_nsleep_wakeup(alarm: *mut alarm, now: ktime_t) {
    let mut task = alarm.data;
    alarm.data = core::ptr::null_mut();
    if (task) {
    wake_up_process(task);
    }
    }
//
// alarmtimer_do_nsleep - Internal alarmtimer nsleep implementation
// @alarm: ptr to alarmtimer
// @absexp: absolute expiration time
// @type: alarm type (BOOTTIME/REALTIME).
//
// Sets the alarm timer and sleeps until it is fired or interrupted.
//
#[no_mangle]
pub unsafe extern "C" fn alarmtimer_do_nsleep(alarm: *mut alarm, absexp: ktime_t, type: alarmtimer_type) -> c_int {
pub static mut restart: *mut c_void = core::ptr::null_mut();
    alarm.data = current;
    do {
    set_current_state(TASK_INTERRUPTIBLE);
    if (!alarm_start_timer(alarm, absexp, false)) {
    alarm.data = core::ptr::null_mut();
    }
    if (likely(alarm.data)) {
    schedule();
    }
    alarm_cancel(alarm);
    } while (alarm.data && !signal_pending(current));
    __set_current_state(TASK_RUNNING);
    destroy_hrtimer_on_stack(&alarm.timer);
    if (!alarm.data) {
    return 0;
    }
    if (freezing(current)) {
    alarmtimer_freezerset(absexp, type);
    }
    restart = &current.restart_block;
    if (restart.nanosleep.type != TT_NONE) {
pub static mut rmt: usize = 0;
    let mut rem;
    rem = ktime_sub(absexp, alarm_bases[type].get_ktime());
    if (rem <= 0) {
    return 0;
    }
    rmt = ktime_to_timespec64(rem);
    return nanosleep_copyout(restart, &rmt);
    }
    return -ERESTART_RESTARTBLOCK;
    }
#[no_mangle]
pub unsafe extern "C" fn alarm_init_on_stack(alarm: *mut alarm, type: alarmtimer_type) {
    hrtimer_setup_on_stack(&alarm.timer, alarmtimer_fired, alarm_bases[type].base_clockid,
    HRTIMER_MODE_ABS);
    __alarm_init(alarm, type, function);
    }
//
// alarm_timer_nsleep_restart - restartblock alarmtimer nsleep
// @restart: ptr to restart block
//
// Handles restarted clock_nanosleep calls
//
#[no_mangle]
unsafe extern "C" fn alarm_timer_nsleep_restart(restart: *mut restart_block) -> long __sched {
pub static mut type: alarmtimer_type = 0;
pub static mut exp: ktime_t = 0;
pub static mut alarm: usize = 0;
    alarm_init_on_stack(&alarm, type, alarmtimer_nsleep_wakeup);
    return alarmtimer_do_nsleep(&alarm, exp, type);
    }
//
// alarm_timer_nsleep - alarmtimer nanosleep
// @which_clock: clockid
// @flags: determines abstime or relative
// @tsreq: requested sleep time (abs or rel)
//
// Handles clock_nanosleep calls against _ALARM clockids
//
#[no_mangle]
pub unsafe extern "C" fn alarm_timer_nsleep(which_clock: clockid_t, flags: c_int, tsreq: *mut timespec64) -> c_int {
pub static mut type: alarmtimer_type = 0;
    let mut restart = &current.restart_block;
pub static mut alarm: usize = 0;
    let mut exp;
    let mut ret = 0;
    if (!alarmtimer_get_rtcdev()) {
    return -EOPNOTSUPP;
    }
    if (flags & ~TIMER_ABSTIME) {
    return -EINVAL;
    }
    if (!capable(CAP_WAKE_ALARM)) {
    return -EPERM;
    }
    alarm_init_on_stack(&alarm, type, alarmtimer_nsleep_wakeup);
    exp = timespec64_to_ktime(*tsreq);
// Convert (if necessary) to absolute time
    if (flags != TIMER_ABSTIME) {
pub static mut now: ktime_t = 0;
    exp = ktime_add_safe(now, exp);
    } else {
    exp = timens_ktime_to_host(which_clock, exp);
    }
    ret = alarmtimer_do_nsleep(&alarm, exp, type);
    if (ret != -ERESTART_RESTARTBLOCK) {
    return ret;
    }
// abs timers don't set remaining time or restart
    if (flags == TIMER_ABSTIME) {
    return -ERESTARTNOHAND;
    }
    restart.nanosleep.clockid = type;
    restart.nanosleep.expires = exp;
    set_restart_fn(restart, alarm_timer_nsleep_restart);
    return ret;
    }
pub static mut k_clock: usize = 0;

// Suspend hook structures
pub static mut dev_pm_ops: usize = 0;
pub static mut platform_driver: usize = 0;
#[no_mangle]
unsafe extern "C" fn get_boottime_timespec(tp: *mut timespec64) {
    ktime_get_boottime_ts64(tp);
    timens_add_boottime(tp);
    }
//
// alarmtimer_init - Initialize alarm timer code
//
// This function initializes the alarm bases and registers
// the posix clock ids.
//
#[no_mangle]
unsafe extern "C" fn alarmtimer_init() -> c_int {
    let mut error = 0;
    let mut i = 0;
    alarmtimer_rtc_timer_init();
// Initialize alarm bases
    alarm_bases[ALARM_REALTIME].base_clockid = CLOCK_REALTIME;
    alarm_bases[ALARM_REALTIME].get_ktime = &ktime_get_real;
    alarm_bases[ALARM_REALTIME].get_timespec = ktime_get_real_ts64;
    alarm_bases[ALARM_BOOTTIME].base_clockid = CLOCK_BOOTTIME;
    alarm_bases[ALARM_BOOTTIME].get_ktime = &ktime_get_boottime;
    alarm_bases[ALARM_BOOTTIME].get_timespec = get_boottime_timespec;
    while (i < ALARM_NUMTYPE) {
    timerqueue_init_head(&alarm_bases[i].timerqueue);
    spin_lock_init(&alarm_bases[i].lock);
    }
    error = alarmtimer_rtc_interface_setup();
    if (error) {
    return error;
    }
    error = platform_driver_register(&alarmtimer_driver);
    if (error) {
// goto;
    }
    return 0;
// label;
    alarmtimer_rtc_interface_remove();
    return error;
    }
    device_initcall!(alarmtimer_init);