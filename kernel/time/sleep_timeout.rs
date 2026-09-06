//! Automatically rewritten from C to Rust
//! Source: kernel/time/sleep_timeout.c
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
// Kernel internal schedule timeout and sleeping functions
//

//
// Since schedule_timeout()'s timer is defined on the stack, it must store
// the target task on the stack as well.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct process_timer {
    pub timer: timer_list,
    pub task: *mut task_struct,
}

#[no_mangle]
unsafe extern "C" fn process_timeout(t: *mut timer_list) {
    let mut timeout = timer_container_of(timeout, t, timer);
    wake_up_process(timeout.task);
    }
//
// schedule_timeout - sleep until timeout
// @timeout: timeout value in jiffies
//
// Make the current task sleep until @timeout jiffies have elapsed.
// The function behavior depends on the current task state
// (see also set_current_state() description):
//
// %TASK_RUNNING - the scheduler is called, but the task does not sleep
// at all. That happens because sched_submit_work() does nothing for
// tasks in %TASK_RUNNING state.
//
// %TASK_UNINTERRUPTIBLE - at least @timeout jiffies are guaranteed to
// pass before the routine returns unless the current task is explicitly
// woken up, (e.g. by wake_up_process()).
//
// %TASK_INTERRUPTIBLE - the routine may return early if a signal is
// delivered to the current task or the current task is explicitly woken
// up.
//
// The current task state is guaranteed to be %TASK_RUNNING when this
// routine returns.
//
// Specifying a @timeout value of %MAX_SCHEDULE_TIMEOUT will schedule
// the CPU away without a bound on the timeout. In this case the return
// value will be %MAX_SCHEDULE_TIMEOUT.
//
// Returns: 0 when the timer has expired otherwise the remaining time in
// jiffies will be returned. In all cases the return value is guaranteed
// to be non-negative.
//
#[no_mangle]
pub unsafe extern "C" fn schedule_timeout(timeout: signed long) -> signed long __sched {
pub static mut timer: usize = 0;
    let mut expire = 0;
    match (timeout) {
    MAX_SCHEDULE_TIMEOUT => {
//
// These two special cases are useful to be comfortable
// in the caller. Nothing more. We could take
// MAX_SCHEDULE_TIMEOUT from one of the negative value
// but I' d like to return a valid offset (>=0) to allow
// the caller to do everything it want with the retval.
//
    schedule();
// goto;
    }
    _ => {
//
// Another bit of PARANOID. Note that the retval will be
// 0 since no piece of kernel is supposed to do a check
// for a negative retval of schedule_timeout() (since it
// should never happens anyway). You just have the printk()
// that will tell you if something is gone wrong and where.
//
    if (timeout < 0) {
    pr_err!("%s: wrong timeout value %lx\n", __func__, timeout);
    dump_stack();
    __set_current_state(TASK_RUNNING);
// goto;
    }
    }
    }
    expire = timeout + jiffies;
    timer.task = current;
    timer_setup_on_stack(&timer.timer, process_timeout, 0);
    timer.timer.expires = expire;
    add_timer(&timer.timer);
    schedule();
    timer_delete_sync(&timer.timer);
// Remove the timer from the object tracker
    timer_destroy_on_stack(&timer.timer);
    timeout = expire - jiffies;
// label;
    return timeout < 0 ? 0 : timeout;
    }
    EXPORT_SYMBOL(schedule_timeout);
//
// __set_current_state() can be used in schedule_timeout_*() functions, because
// schedule_timeout() calls schedule() unconditionally.
//
// schedule_timeout_interruptible - sleep until timeout (interruptible)
// @timeout: timeout value in jiffies
//
// See schedule_timeout() for details.
//
// Task state is set to TASK_INTERRUPTIBLE before starting the timeout.
//
#[no_mangle]
pub unsafe extern "C" fn schedule_timeout_interruptible(timeout: signed long) -> signed long __sched {
    __set_current_state(TASK_INTERRUPTIBLE);
    return schedule_timeout(timeout);
    }
    EXPORT_SYMBOL(schedule_timeout_interruptible);
//
// schedule_timeout_killable - sleep until timeout (killable)
// @timeout: timeout value in jiffies
//
// See schedule_timeout() for details.
//
// Task state is set to TASK_KILLABLE before starting the timeout.
//
#[no_mangle]
pub unsafe extern "C" fn schedule_timeout_killable(timeout: signed long) -> signed long __sched {
    __set_current_state(TASK_KILLABLE);
    return schedule_timeout(timeout);
    }
    EXPORT_SYMBOL(schedule_timeout_killable);
//
// schedule_timeout_uninterruptible - sleep until timeout (uninterruptible)
// @timeout: timeout value in jiffies
//
// See schedule_timeout() for details.
//
// Task state is set to TASK_UNINTERRUPTIBLE before starting the timeout.
//
#[no_mangle]
pub unsafe extern "C" fn schedule_timeout_uninterruptible(timeout: signed long) -> signed long __sched {
    __set_current_state(TASK_UNINTERRUPTIBLE);
    return schedule_timeout(timeout);
    }
    EXPORT_SYMBOL(schedule_timeout_uninterruptible);
//
// schedule_timeout_idle - sleep until timeout (idle)
// @timeout: timeout value in jiffies
//
// See schedule_timeout() for details.
//
// Task state is set to TASK_IDLE before starting the timeout. It is similar to
// schedule_timeout_uninterruptible(), except this task will not contribute to
// load average.
//
#[no_mangle]
pub unsafe extern "C" fn schedule_timeout_idle(timeout: signed long) -> signed long __sched {
    __set_current_state(TASK_IDLE);
    return schedule_timeout(timeout);
    }
    EXPORT_SYMBOL(schedule_timeout_idle);
//
// schedule_hrtimeout_range_clock - sleep until timeout
// @expires:	timeout value (ktime_t)
// @delta:	slack in expires timeout (ktime_t)
// @mode:	timer mode
// @clock_id:	timer clock to be used
//
// Details are explained in schedule_hrtimeout_range() function description as
// this function is commonly used.
//
    int __sched schedule_hrtimeout_range_clock(ktime_t *expires, u64 delta,
    const enum hrtimer_mode mode, clockid_t clock_id)
    {
pub static mut t: usize = 0;
//
// Optimize when a zero timeout value is given. It does not
// matter whether this is an absolute or a relative time.
//
    if (expires && *expires == 0) {
    __set_current_state(TASK_RUNNING);
    return 0;
    }
//
// A NULL parameter means "infinite"
//
    if (!expires) {
    schedule();
    return -EINTR;
    }
    hrtimer_setup_sleeper_on_stack(&t, clock_id, mode);
    hrtimer_set_expires_range_ns(&t.timer, *expires, delta);
    hrtimer_sleeper_start_expires(&t, mode);
    if (likely(t.task)) {
    schedule();
    }
    hrtimer_cancel(&t.timer);
    destroy_hrtimer_on_stack(&t.timer);
    __set_current_state(TASK_RUNNING);
    return !t.task ? 0 : -EINTR;
    }
    EXPORT_SYMBOL_GPL(schedule_hrtimeout_range_clock);
//
// schedule_hrtimeout_range - sleep until timeout
// @expires:	timeout value (ktime_t)
// @delta:	slack in expires timeout (ktime_t)
// @mode:	timer mode
//
// Make the current task sleep until the given expiry time has
// elapsed. The routine will return immediately unless
// the current task state has been set (see set_current_state()).
//
// The @delta argument gives the kernel the freedom to schedule the
// actual wakeup to a time that is both power and performance friendly
// for regular (non RT/DL) tasks.
// The kernel give the normal best effort behavior for "@expires+@delta",
// but may decide to fire the timer earlier, but no earlier than @expires.
//
// You can set the task state as follows -
//
// %TASK_UNINTERRUPTIBLE - at least @timeout time is guaranteed to
// pass before the routine returns unless the current task is explicitly
// woken up, (e.g. by wake_up_process()).
//
// %TASK_INTERRUPTIBLE - the routine may return early if a signal is
// delivered to the current task or the current task is explicitly woken
// up.
//
// The current task state is guaranteed to be TASK_RUNNING when this
// routine returns.
//
// Returns: 0 when the timer has expired. If the task was woken before the
// timer expired by a signal (only possible in state TASK_INTERRUPTIBLE) or
// by an explicit wakeup, it returns -EINTR.
//
    int __sched schedule_hrtimeout_range(ktime_t *expires, u64 delta,
    const enum hrtimer_mode mode)
    {
    return schedule_hrtimeout_range_clock(expires, delta, mode,
    CLOCK_MONOTONIC);
    }
    EXPORT_SYMBOL_GPL(schedule_hrtimeout_range);
//
// schedule_hrtimeout - sleep until timeout
// @expires:	timeout value (ktime_t)
// @mode:	timer mode
//
// See schedule_hrtimeout_range() for details. @delta argument of
// schedule_hrtimeout_range() is set to 0 and has therefore no impact.
//
#[no_mangle]
pub unsafe extern "C" fn schedule_hrtimeout(expires: *mut ktime_t, mode: hrtimer_mode) -> int __sched {
    return schedule_hrtimeout_range(expires, 0, mode);
    }
    EXPORT_SYMBOL_GPL(schedule_hrtimeout);
//
// msleep - sleep safely even with waitqueue interruptions
// @msecs:	Requested sleep duration in milliseconds
//
// msleep() uses jiffy based timeouts for the sleep duration. Because of the
// design of the timer wheel, the maximum additional percentage delay (slack) is
// 12.5%. This is only valid for timers which will end up in level 1 or a higher
// level of the timer wheel. For explanation of those 12.5% please check the
// detailed description about the basics of the timer wheel.
//
// The slack of timers which will end up in level 0 depends on sleep duration
// (msecs) and HZ configuration and can be calculated in the following way (with
// the timer wheel design restriction that the slack is not less than 12.5%):
//
// ``slack = MSECS_PER_TICK / msecs``
//
// When the allowed slack of the callsite is known, the calculation could be
// turned around to find the minimal allowed sleep duration to meet the
// constraints. For example:
//
// * ``HZ=1000`` with ``slack=25%``: ``MSECS_PER_TICK / slack = 1 / (1/4) = 4``:
// all sleep durations greater or equal 4ms will meet the constraints.
// * ``HZ=1000`` with ``slack=12.5%``: ``MSECS_PER_TICK / slack = 1 / (1/8) = 8``:
// all sleep durations greater or equal 8ms will meet the constraints.
// * ``HZ=250`` with ``slack=25%``: ``MSECS_PER_TICK / slack = 4 / (1/4) = 16``:
// all sleep durations greater or equal 16ms will meet the constraints.
// * ``HZ=250`` with ``slack=12.5%``: ``MSECS_PER_TICK / slack = 4 / (1/8) = 32``:
// all sleep durations greater or equal 32ms will meet the constraints.
//
// See also the signal aware variant msleep_interruptible().
//
#[no_mangle]
pub unsafe extern "C" fn msleep(msecs: c_uint) {
pub static mut timeout: c_ulong = 0;
    while (timeout) {
    timeout = schedule_timeout_uninterruptible(timeout);
    }
    }
    EXPORT_SYMBOL(msleep);
//
// msleep_interruptible - sleep waiting for signals
// @msecs:	Requested sleep duration in milliseconds
//
// See msleep() for some basic information.
//
// The difference between msleep() and msleep_interruptible() is that the sleep
// could be interrupted by a signal delivery and then returns early.
//
// Returns: The remaining time of the sleep duration transformed to msecs (see
// schedule_timeout() for details).
//
#[no_mangle]
pub unsafe extern "C" fn msleep_interruptible(msecs: c_uint) -> c_ulong {
pub static mut timeout: c_ulong = 0;
    while (timeout && !signal_pending(current)) {
    timeout = schedule_timeout_interruptible(timeout);
    }
    return jiffies_to_msecs(timeout);
    }
    EXPORT_SYMBOL(msleep_interruptible);
//
// usleep_range_state - Sleep for an approximate time in a given state
// @min:	Minimum time in usecs to sleep
// @max:	Maximum time in usecs to sleep
// @state:	State of the current task that will be while sleeping
//
// usleep_range_state() sleeps at least for the minimum specified time but not
// longer than the maximum specified amount of time. The range might reduce
// power usage by allowing hrtimers to coalesce an already scheduled interrupt
// with this hrtimer. In the worst case, an interrupt is scheduled for the upper
// bound.
//
// The sleeping task is set to the specified state before starting the sleep.
//
// In non-atomic context where the exact wakeup time is flexible, use
// usleep_range() or its variants instead of udelay(). The sleep improves
// responsiveness by avoiding the CPU-hogging busy-wait of udelay().
//
#[no_mangle]
pub unsafe extern "C" fn usleep_range_state(min: c_ulong, max: c_ulong, state: c_uint) -> void __sched {
pub static mut exp: ktime_t = 0;
pub static mut delta: u64 = 0;
    if (WARN_ON_ONCE!(max < min)) {
    delta = 0;
    }
    for (;;) {
    __set_current_state(state);
// Do not return before the requested sleep time has elapsed
    if (!schedule_hrtimeout_range(&exp, delta, HRTIMER_MODE_ABS)) {
    break;
    }
    }
    }
    EXPORT_SYMBOL(usleep_range_state);