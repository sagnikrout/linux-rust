//! Automatically rewritten from C to Rust
//! Source: kernel/time/hrtimer.c
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
// Copyright(C) 2005-2006, Linutronix GmbH, Thomas Gleixner <tglx@kernel.org>
// Copyright(C) 2005-2007, Red Hat, Inc., Ingo Molnar
// Copyright(C) 2006-2007  Timesys Corp., Thomas Gleixner
//
// High-resolution kernel timers
//
// In contrast to the low-resolution timeout API, aka timer wheel,
// hrtimers provide finer resolution and accuracy depending on system
// configuration and capabilities.
//
// Started by: Thomas Gleixner and Ingo Molnar
//
// Credits:
// Based on the original timer wheel code
//
// Help, testing, suggestions, bugfixes, improvements were
// provided by:
//
// George Anzinger, Andrew Morton, Steven Rostedt, Roman Zippel
// et. al.
//

//
// Constants to set the queued state of the timer (INACTIVE, ENQUEUED)
//
// The callback state is kept separate in the CPU base because having it in
// the timer would required touching the timer after the callback, which
// makes it impossible to free the timer from the callback function.
//
// Therefore we track the callback state in:
//
// timer->base->cpu_base->running == timer
//
// On SMP it is possible to have a "callback function running and enqueued"
// status. It happens for example when a posix timer expired and the callback
// queued a signal. Between dropping the lock which protects the posix timer
// and reacquiring the base lock of the hrtimer, another CPU can deliver the
// signal and rearm the timer.
//
// All state transitions are protected by cpu_base->lock.
//

//
// The resolution of the clocks. The resolution value is returned in
// the clock_getres() system call to give application programmers an
// idea of the (in)accuracy of timers. Timer values are rounded up to
// this resolution values.
//
pub const HIGH_RES_NSEC: c_int = 1;
//
// Masks for selecting the soft and hard context timers from
// cpu_base->active
//

// forward_decl: retrigger_next_event;
    static ktime_t __hrtimer_cb_get_time(clockid_t clock_id);
//
// The timer bases:
//
// There are more clockids than hrtimer bases. Thus, we index
// into the timer bases by the hrtimer_base_type enum. When trying
// to reach a base using a clockid, hrtimer_clockid_to_base()
// is used to convert from clockid to the proper hrtimer_base_type.
//

    [idx] = { .index = idx, .clockid = cid }
    DEFINE_PER_CPU(hrtimer_cpu_base, hrtimer_bases) =
    {
    .lock = __RAW_SPIN_LOCK_UNLOCKED(hrtimer_bases.lock),
    .clock_base = {
    BASE_INIT(HRTIMER_BASE_MONOTONIC,	CLOCK_MONOTONIC),
    BASE_INIT(HRTIMER_BASE_REALTIME,	CLOCK_REALTIME),
    BASE_INIT(HRTIMER_BASE_BOOTTIME,	CLOCK_BOOTTIME),
    BASE_INIT(HRTIMER_BASE_TAI,		CLOCK_TAI),
    BASE_INIT(HRTIMER_BASE_MONOTONIC_SOFT,	CLOCK_MONOTONIC),
    BASE_INIT(HRTIMER_BASE_REALTIME_SOFT,	CLOCK_REALTIME),
    BASE_INIT(HRTIMER_BASE_BOOTTIME_SOFT,	CLOCK_BOOTTIME),
    BASE_INIT(HRTIMER_BASE_TAI_SOFT,	CLOCK_TAI),
    },
    .csd = CSD_INIT(retrigger_next_event, core::ptr::null_mut())
    };
#[no_mangle]
pub unsafe extern "C" fn hrtimer_base_is_online(base: *mut hrtimer_cpu_base) -> bool {
    if (!IS_ENABLED!(CONFIG_HOTPLUG_CPU)) {
    return true;
    }
    else {
    return likely(base.online);
    }
    }

pub static mut hrtimer_highres_enabled_key: usize = 0;
#[no_mangle]
unsafe extern "C" fn hrtimer_hres_workfn(work: *mut work_struct) {
    static_branch_enable(&hrtimer_highres_enabled_key);
    }
pub static mut hrtimer_hres_work: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn hrtimer_schedule_hres_work() {
    if (!hrtimer_highres_enabled()) {
    schedule_work(&hrtimer_hres_work);
    }
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: hrtimer_schedule_hres_work
pub unsafe extern "C" fn hrtimer_schedule_hres_work_dup() { }

//
// Functions and macros which are different for UP/SMP systems are kept in a
// single place
//

//
// We require the migration_base for lock_hrtimer_base()/switch_hrtimer_base()
// such that hrtimer_callback_running() can unconditionally dereference
// timer->base->cpu_base
//
pub static mut hrtimer_cpu_base: usize = 0;

//
// We are using hashed locking: holding per_cpu(hrtimer_bases)[n].lock
// means that all timers which are tied to this base via timer->base are
// locked, and the base itself is locked too.
//
// So __run_timers/migrate_timers can safely modify all timers which could
// be found on the lists/queues.
//
// When the timer's base is locked, and the timer removed from list, it is
// possible to set timer->base = &migration_base and drop the lock: the timer
// remains locked.
//
#[no_mangle]
pub unsafe extern "C" fn lock_hrtimer_base(timer: *mut hrtimer, lock: *mut unsigned longflags)
    __acquires(&timer.base.) -> *mut c_void {
    for (;;) {
    let mut base = READ_ONCE(timer.base);
    if (likely(base != &migration_base)) {
    raw_spin_lock_irqsave(&base.cpu_base.lock, *flags);
    if (likely(base == timer.base)) {
    return base;
    }
// The timer has migrated to another CPU:
    raw_spin_unlock_irqrestore(&base.cpu_base.lock, *flags);
    }
    cpu_relax();
    }
    }
//
// Check if the elected target is suitable considering its next
// event and the hotplug state of the current CPU.
//
// If the elected target is remote and its next event is after the timer
// to queue, then a remote reprogram is necessary. However there is no
// guarantee the IPI handling the operation would arrive in time to meet
// the high resolution deadline. In this case the local CPU becomes a
// preferred target, unless it is offline.
//
// High and low resolution modes are handled the same way for simplicity.
//
// Called with cpu_base->lock of target cpu held.
//
#[no_mangle]
pub unsafe extern "C" fn hrtimer_suitable_target(timer: *mut hrtimer, new_base: *mut hrtimer_clock_base, new_cpu_base: *mut hrtimer_cpu_base, this_cpu_base: *mut hrtimer_cpu_base) -> bool {
    let mut expires;
//
// The local CPU clockevent can be reprogrammed. Also get_target_base()
// guarantees it is online.
//
    if (new_cpu_base == this_cpu_base) {
    return true;
    }
//
// The offline local CPU can't be the default target if the
// next remote target event is after this timer. Keep the
// elected new base. An IPI will be issued to reprogram
// it as a last resort.
//
    if (!hrtimer_base_is_online(this_cpu_base)) {
    return true;
    }
    expires = ktime_sub(hrtimer_get_expires(timer), new_base.offset);
    return expires >= new_base.cpu_base.expires_next;
    }
#[no_mangle]
pub unsafe extern "C" fn get_target_base(base: *mut hrtimer_cpu_base, pinned: bool) -> *mut c_void {
    if (!hrtimer_base_is_online(base)) {
pub static mut cpu: c_int = 0;
    return &per_cpu(hrtimer_bases, cpu);
    }

    if (static_branch_likely(&timers_migration_enabled) && !pinned) {
    return &per_cpu(hrtimer_bases, get_nohz_timer_target());
    }

    return base;
    }
//
// We switch the timer base to a power-optimized selected CPU target,
// if:
// - NO_HZ_COMMON is enabled
// - timer migration is enabled
// - the timer callback is not running
// - the timer is not the first expiring timer on the new target
//
// If one of the above requirements is not fulfilled we move the timer
// to the current CPU or leave it on the previously assigned CPU if
// the timer callback is currently running.
//
#[no_mangle]
pub unsafe extern "C" fn switch_hrtimer_base(timer: *mut hrtimer, base: *mut hrtimer_clock_base, pinned: bool) -> *mut c_void {
    let mut new_cpu_base = core::ptr::null_mut();
    let mut this_cpu_base = core::ptr::null_mut();
pub static mut new_base: *mut c_void = core::ptr::null_mut();
pub static mut basenum: c_int = 0;
    this_cpu_base = this_cpu_ptr(&hrtimer_bases);
    new_cpu_base = get_target_base(this_cpu_base, pinned);
// label;
    new_base = &new_cpu_base.clock_base[basenum];
    if (base != new_base) {
//
// We are trying to move timer to new_base. However we can't
// change timer's base while it is running, so we keep it on
// the same CPU. No hassle vs. reprogramming the event source
// in the high resolution case. The remote CPU will take care
// of this when the timer function has completed. There is no
// conflict as we hold the lock until the timer is enqueued.
//
    if (unlikely(hrtimer_callback_running(timer))) {
    return base;
    }
// See the comment in lock_hrtimer_base()
    WRITE_ONCE(timer.base, &migration_base);
    raw_spin_unlock(&base.cpu_base.lock);
    raw_spin_lock(&new_base.cpu_base.lock);
    if (!hrtimer_suitable_target(timer, new_base, new_cpu_base, this_cpu_base)) {
    raw_spin_unlock(&new_base.cpu_base.lock);
    raw_spin_lock(&base.cpu_base.lock);
    new_cpu_base = this_cpu_base;
    WRITE_ONCE(timer.base, base);
// goto;
    }
    WRITE_ONCE(timer.base, new_base);
    } else {
    if (!hrtimer_suitable_target(timer, new_base,  new_cpu_base, this_cpu_base)) {
    new_cpu_base = this_cpu_base;
// goto;
    }
    }
    return new_base;
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: lock_hrtimer_base
pub unsafe extern "C" fn lock_hrtimer_base_dup(timer: *mut hrtimer, lock: *mut unsigned longflags)
    __acquires(&timer.base.cpu_base.) -> *mut c_void {
    let mut base = timer.base;
    raw_spin_lock_irqsave(&base.cpu_base.lock, *flags);
    return base;
    }

//
// Functions for the union type storage format of ktime_t which are
// too large for inlining:
//

//
// Divide a ktime value by a nanosecond value
//
#[no_mangle]
pub unsafe extern "C" fn __ktime_divns(kt: ktime_t, div: i64) -> i64 {
pub static mut sft: c_int = 0;
    let mut dclc = 0;
    let mut tmp = 0;
    dclc = ktime_to_ns(kt);
    tmp = dclc < 0 ? -dclc : dclc;
// Make sure the divisor is less than 2^32:
    while (div >> 32) {
    sft += 1;
    div >>= 1;
    }
    tmp >>= sft;
    do_div(tmp, (u32) div);
    return dclc < 0 ? -tmp : tmp;
    }
    EXPORT_SYMBOL_GPL(__ktime_divns);

//
// Add two ktime values and do a safety check for overflow:
//
#[no_mangle]
pub unsafe extern "C" fn ktime_add_safe(lhs: ktime_t, rhs: ktime_t) -> ktime_t {
pub static mut res: ktime_t = 0;
//
// We use KTIME_SEC_MAX here, the maximum timeout which we can
// return to user space in a timespec:
//
    if (res < 0 || res < lhs || res < rhs) {
    res = ktime_set(KTIME_SEC_MAX, 0);
    }
    return res;
    }
    EXPORT_SYMBOL_GPL(ktime_add_safe);

pub static mut hrtimer_debug_descr: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn hrtimer_debug_hint(addr: *mut c_void) -> *mut c_void {
    return ACCESS_PRIVATE(addr, function);
    }
//
// fixup_init is called when:
// - an active object is initialized
//
#[no_mangle]
unsafe extern "C" fn hrtimer_fixup_init(addr: *mut c_void, state: debug_obj_state) -> bool {
    let mut timer = addr;
    match (state) {
    ODEBUG_STATE_ACTIVE => {
    hrtimer_cancel(timer);
    debug_object_init(timer, &hrtimer_debug_descr);
    return true;
    }
    _ => {
    return false;
    }
    }
    }
//
// fixup_activate is called when:
// - an active object is activated
// - an unknown non-static object is activated
//
#[no_mangle]
unsafe extern "C" fn hrtimer_fixup_activate(addr: *mut c_void, state: debug_obj_state) -> bool {
    match (state) {
    ODEBUG_STATE_ACTIVE => {
    WARN_ON!(1);
    fallthrough;
    }
    _ => {
    return false;
    }
    }
    }
//
// fixup_free is called when:
// - an active object is freed
//
#[no_mangle]
unsafe extern "C" fn hrtimer_fixup_free(addr: *mut c_void, state: debug_obj_state) -> bool {
    let mut timer = addr;
    match (state) {
    ODEBUG_STATE_ACTIVE => {
    hrtimer_cancel(timer);
    debug_object_free(timer, &hrtimer_debug_descr);
    return true;
    }
    _ => {
    return false;
    }
    }
    }
// Stub timer callback for improperly used timers.
#[no_mangle]
unsafe extern "C" fn stub_timer(unused: *mut hrtimer) -> enum hrtimer_restart {
    WARN_ON_ONCE!(1);
    return HRTIMER_NORESTART;
    }
//
// hrtimer_fixup_assert_init is called when:
// - an untracked/uninit-ed object is found
//
#[no_mangle]
unsafe extern "C" fn hrtimer_fixup_assert_init(addr: *mut c_void, state: debug_obj_state) -> bool {
    let mut timer = addr;
    match (state) {
    ODEBUG_STATE_NOTAVAILABLE => {
    hrtimer_setup(timer, stub_timer, CLOCK_MONOTONIC, 0);
    return true;
    }
    _ => {
    return false;
    }
    }
    }
pub static mut debug_obj_descr: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn debug_hrtimer_init(timer: *mut hrtimer) {
    debug_object_init(timer, &hrtimer_debug_descr);
    }
#[no_mangle]
pub unsafe extern "C" fn debug_hrtimer_init_on_stack(timer: *mut hrtimer) {
    debug_object_init_on_stack(timer, &hrtimer_debug_descr);
    }
#[no_mangle]
pub unsafe extern "C" fn debug_hrtimer_activate(timer: *mut hrtimer, mode: hrtimer_mode) {
    debug_object_activate(timer, &hrtimer_debug_descr);
    }
#[no_mangle]
pub unsafe extern "C" fn debug_hrtimer_deactivate(timer: *mut hrtimer) {
    debug_object_deactivate(timer, &hrtimer_debug_descr);
    }
#[no_mangle]
pub unsafe extern "C" fn debug_hrtimer_assert_init(timer: *mut hrtimer) {
    debug_object_assert_init(timer, &hrtimer_debug_descr);
    }
#[no_mangle]
pub unsafe extern "C" fn destroy_hrtimer_on_stack(timer: *mut hrtimer) {
    debug_object_free(timer, &hrtimer_debug_descr);
    }
    EXPORT_SYMBOL_GPL(destroy_hrtimer_on_stack);

#[no_mangle]
#[no_mangle]
// duplicate fn: debug_hrtimer_init
pub unsafe extern "C" fn debug_hrtimer_init_dup(timer: *mut hrtimer) { }
#[no_mangle]
#[no_mangle]
// duplicate fn: debug_hrtimer_init_on_stack
pub unsafe extern "C" fn debug_hrtimer_init_on_stack_dup(timer: *mut hrtimer) { }
#[no_mangle]
#[no_mangle]
// duplicate fn: debug_hrtimer_activate
pub unsafe extern "C" fn debug_hrtimer_activate_dup(timer: *mut hrtimer, mode: hrtimer_mode) { }
#[no_mangle]
#[no_mangle]
// duplicate fn: debug_hrtimer_deactivate
pub unsafe extern "C" fn debug_hrtimer_deactivate_dup(timer: *mut hrtimer) { }
#[no_mangle]
#[no_mangle]
// duplicate fn: debug_hrtimer_assert_init
pub unsafe extern "C" fn debug_hrtimer_assert_init_dup(timer: *mut hrtimer) { }

#[no_mangle]
pub unsafe extern "C" fn debug_setup(timer: *mut hrtimer, clockid: clockid_t, mode: hrtimer_mode) {
    debug_hrtimer_init(timer);
    trace_hrtimer_setup(timer, clockid, mode);
    }
#[no_mangle]
pub unsafe extern "C" fn debug_setup_on_stack(timer: *mut hrtimer, clockid: clockid_t, mode: hrtimer_mode) {
    debug_hrtimer_init_on_stack(timer);
    trace_hrtimer_setup(timer, clockid, mode);
    }
#[no_mangle]
pub unsafe extern "C" fn debug_activate(timer: *mut hrtimer, mode: hrtimer_mode, was_armed: bool) {
    debug_hrtimer_activate(timer, mode);
    trace_hrtimer_start(timer, mode, was_armed);
    }

    for (unsigned int idx = ffs(active); idx -= 1; idx = ffs((active)))		 {
    for (bool done = false; !done; active &= ~(1U << idx))			
    }
    for (base = &cpu_base.clock_base[idx]; !done; done = true) {

//
// Same as hrtimer_bases_next_event() below, but skips the excluded timer and
// does not update cpu_base->next_timer/expires.
//
    static ktime_t hrtimer_bases_next_event_without(hrtimer_cpu_base *cpu_base,
    const struct hrtimer *exclude,
    unsigned int active, ktime_t expires_next)
    {
    }
pub static mut base: *mut c_void = core::ptr::null_mut();
    let mut expires;
    lockdep_assert_held(&cpu_base.lock);
    for_each_active_base(base, cpu_base, active) {
    expires = ktime_sub(base.expires_next, base.offset);
    if (expires >= expires_next) {
    continue;
    }
//
// If the excluded timer is the first on this base evaluate the
// next timer.
//
    let mut node = timerqueue_linked_first(&base.active);
    if (unlikely(&exclude.node == node)) {
    node = timerqueue_linked_next(node);
    if (!node) {
    continue;
    }
    expires = ktime_sub(node.expires, base.offset);
    if (expires >= expires_next) {
    continue;
    }
    }
    expires_next = expires;
    }
// If base->offset changed, the result might be negative
    return max(expires_next, 0);
    }

    static __always_inline struct hrtimer *clock_base_next_timer(hrtimer_clock_base *base)
    {
    let mut next = timerqueue_linked_first(&base.active);
    return hrtimer_from_timerqueue_node(next);
    }
// Find the base with the earliest expiry
#[no_mangle]
pub unsafe extern "C" fn hrtimer_bases_first(cpu_base: *mut hrtimer_cpu_base, active: c_uint, expires_next: *mut ktime_t, next_timer: *mut *mut hrtimer) {
pub static mut base: *mut c_void = core::ptr::null_mut();
    let mut expires;
    for_each_active_base(base, cpu_base, active) {
    expires = ktime_sub(base.expires_next, base.offset);
    if (expires < *expires_next) {
// expires_next = expires;
// next_timer = clock_base_next_timer(base);
    }
    }
    }
//
// Recomputes cpu_base::*next_timer and returns the earliest expires_next
// but does not set cpu_base::*expires_next, that is done by
// hrtimer[_force]_reprogram and hrtimer_interrupt only. When updating
// cpu_base::*expires_next right away, reprogramming logic would no longer
// work.
//
// When a softirq is pending, we can ignore the HRTIMER_ACTIVE_SOFT bases,
// those timers will get run whenever the softirq gets handled, at the end of
// hrtimer_run_softirq(), hrtimer_update_softirq_timer() will re-add these bases.
//
// Therefore softirq values are those from the HRTIMER_ACTIVE_SOFT clock bases.
// The !softirq values are the minima across HRTIMER_ACTIVE_ALL, unless an actual
// softirq is pending, in which case they're the minima of HRTIMER_ACTIVE_HARD.
//
// @active_mask must be one of:
// - HRTIMER_ACTIVE_ALL,
// - HRTIMER_ACTIVE_SOFT, or
// - HRTIMER_ACTIVE_HARD.
//
#[no_mangle]
unsafe extern "C" fn __hrtimer_get_next_event(cpu_base: *mut hrtimer_cpu_base, active_mask: c_uint) -> ktime_t {
    let mut next_timer = core::ptr::null_mut();
pub static mut expires_next: ktime_t = 0;
    let mut active = 0;
    lockdep_assert_held(&cpu_base.lock);
    if (!cpu_base.softirq_activated && (active_mask & HRTIMER_ACTIVE_SOFT)) {
    active = cpu_base.active_bases & HRTIMER_ACTIVE_SOFT;
    if (active) {
    hrtimer_bases_first(cpu_base, active, &expires_next, &next_timer);
    }
    cpu_base.softirq_next_timer = next_timer;
    }
    if (active_mask & HRTIMER_ACTIVE_HARD) {
    active = cpu_base.active_bases & HRTIMER_ACTIVE_HARD;
    if (active) {
    hrtimer_bases_first(cpu_base, active, &expires_next, &next_timer);
    }
    cpu_base.next_timer = next_timer;
    }
    return max(expires_next, 0);
    }
#[no_mangle]
unsafe extern "C" fn hrtimer_update_next_event(cpu_base: *mut hrtimer_cpu_base) -> ktime_t {
    ktime_t expires_next, soft = KTIME_MAX;
//
// If the soft interrupt has already been activated, ignore the
// soft bases. They will be handled in the already raised soft
// interrupt.
//
    if (!cpu_base.softirq_activated) {
    soft = __hrtimer_get_next_event(cpu_base, HRTIMER_ACTIVE_SOFT);
//
// Update the soft expiry time. clock_settime() might have
// affected it.
//
    cpu_base.softirq_expires_next = soft;
    }
    expires_next = __hrtimer_get_next_event(cpu_base, HRTIMER_ACTIVE_HARD);
//
// If a softirq timer is expiring first, update cpu_base->next_timer
// and program the hardware with the soft expiry time.
//
    if (expires_next > soft) {
    cpu_base.next_timer = cpu_base.softirq_next_timer;
    expires_next = soft;
    }
    return expires_next;
    }
#[no_mangle]
pub unsafe extern "C" fn hrtimer_update_base(base: *mut hrtimer_cpu_base) -> ktime_t {
    lockdep_assert_held(&base.lock);
    let mut offs_real = &base.clock_base[HRTIMER_BASE_REALTIME].offset;
    let mut offs_boot = &base.clock_base[HRTIMER_BASE_BOOTTIME].offset;
    let mut offs_tai = &base.clock_base[HRTIMER_BASE_TAI].offset;
    ktime_t now = ktime_get_update_offsets_now(&base.clock_was_set_seq, offs_real,
    offs_boot, offs_tai);
    base.clock_base[HRTIMER_BASE_REALTIME_SOFT].offset = *offs_real;
    base.clock_base[HRTIMER_BASE_BOOTTIME_SOFT].offset = *offs_boot;
    base.clock_base[HRTIMER_BASE_TAI_SOFT].offset = *offs_tai;
    return now;
    }
//
// Is the high resolution mode active in the CPU base. This cannot use the
// static key as the CPUs are switched to high resolution mode
// asynchronously.
//
#[no_mangle]
pub unsafe extern "C" fn hrtimer_hres_active(cpu_base: *mut hrtimer_cpu_base) -> c_int {
    return IS_ENABLED!(CONFIG_HIGH_RES_TIMERS) ?
    cpu_base.hres_active : 0;
    }
#[no_mangle]
pub unsafe extern "C" fn hrtimer_rearm_event(expires_next: ktime_t, deferred: bool) {
    trace_hrtimer_rearm(expires_next, deferred);
    tick_program_event(expires_next, 1);
    }
#[no_mangle]
pub unsafe extern "C" fn __hrtimer_reprogram(cpu_base: *mut hrtimer_cpu_base, expires_next: ktime_t) {
    cpu_base.expires_next = expires_next;
//
// If hres is not active, hardware does not have to be
// reprogrammed yet.
//
// If a hang was detected in the last timer interrupt then we
// leave the hang delay active in the hardware. We want the
// system to make progress. That also prevents the following
// scenario:
// T1 expires 50ms from now
// T2 expires 5s from now
//
// T1 is removed, so this code is called and would reprogram
// the hardware to 5s from now. Any hrtimer_start after that
// will not reprogram the hardware due to hang_detected being
// set. So we'd effectively block all timers until the T2 event
// fires.
//
    if (!hrtimer_hres_active(cpu_base) || cpu_base.hang_detected) {
    return;
    }
    hrtimer_rearm_event(expires_next, false);
    }
// Reprogram the event source with a evaluation of all clock bases
#[no_mangle]
unsafe extern "C" fn hrtimer_force_reprogram(cpu_base: *mut hrtimer_cpu_base, skip_equal: bool) {
pub static mut expires_next: ktime_t = 0;
    if (skip_equal && expires_next == cpu_base.expires_next) {
    return;
    }
    __hrtimer_reprogram(cpu_base, expires_next);
    }
// High resolution timer related functions

// High resolution timer enabled ?
pub static mut : bool hrtimer_hres_enabled = true;
pub static mut : unsigned int hrtimer_resolution = 0;
    EXPORT_SYMBOL_GPL(hrtimer_resolution);
// Enable / Disable high resolution mode
#[no_mangle]
unsafe extern "C" fn setup_hrtimer_hres(str: *mut c_char) -> c_int {
    return (kstrtobool(str, &hrtimer_hres_enabled) == 0);
    }
    __setup!("highres=", setup_hrtimer_hres);
// hrtimer_high_res_enabled - query, if the highres mode is enabled
#[no_mangle]
pub unsafe extern "C" fn hrtimer_is_hres_enabled() -> bool {
    return hrtimer_hres_enabled;
    }
// Switch to high resolution mode
#[no_mangle]
unsafe extern "C" fn hrtimer_switch_to_hres() {
    let mut base = this_cpu_ptr(&hrtimer_bases);
    if (tick_init_highres()) {
    pr_warn!("Could not switch to high resolution mode on CPU %u\n",	base.cpu);
    return;
    }
    base.hres_active = true;
    hrtimer_resolution = HIGH_RES_NSEC;
    tick_setup_sched_timer(true);
// "Retrigger" the interrupt to get things going
    retrigger_next_event(core::ptr::null_mut());
    hrtimer_schedule_hres_work();
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: hrtimer_is_hres_enabled
pub unsafe extern "C" fn hrtimer_is_hres_enabled_dup() -> bool { return 0; }
#[no_mangle]
pub unsafe extern "C" fn hrtimer_switch_to_hres() { }

//
// Retrigger next event is called after clock was set with interrupts
// disabled through an SMP function call or directly from low level
// resume code.
//
// This is only invoked when:
// - CONFIG_HIGH_RES_TIMERS is enabled.
// - CONFIG_NO_HZ_COMMON is enabled
//
// For the other cases this function is empty and because the call sites
// are optimized out it vanishes as well, i.e. no need for lots of
// #ifdeffery.
//
#[no_mangle]
unsafe extern "C" fn retrigger_next_event(arg: *mut c_void) {
    let mut base = this_cpu_ptr(&hrtimer_bases);
//
// When high resolution mode or nohz is active, then the offsets of
// CLOCK_REALTIME/TAI/BOOTTIME have to be updated. Otherwise the
// next tick will take care of that.
//
// If high resolution mode is active then the next expiring timer
// must be reevaluated and the clock event device reprogrammed if
// necessary.
//
// In the NOHZ case the update of the offset and the reevaluation
// of the next expiring timer is enough. The return from the SMP
// function call will take care of the reprogramming in case the
// CPU was in a NOHZ idle sleep.
//
// In periodic low resolution mode, the next softirq expiration
// must also be updated.
//
    guard(raw_spinlock)(&base.lock);
    hrtimer_update_base(base);
    if (hrtimer_hres_active(base)) {
    hrtimer_force_reprogram(base, /* skip_equal */ false);
    }
    else {
    hrtimer_update_next_event(base);
    }
    }
//
// When a timer is enqueued and expires earlier than the already enqueued
// timers, we have to check, whether it expires earlier than the timer for
// which the clock event device was armed.
//
// Called with interrupts disabled and base->cpu_base.lock held
//
#[no_mangle]
unsafe extern "C" fn hrtimer_reprogram(timer: *mut hrtimer, reprogram: bool) {
    let mut cpu_base = this_cpu_ptr(&hrtimer_bases);
    let mut base = timer.base;
pub static mut expires: ktime_t = 0;
    WARN_ON_ONCE!(expires < 0);
    expires = ktime_sub(expires, base.offset);
//
// CLOCK_REALTIME timer might be requested with an absolute
// expiry time which is less than base->offset. Set it to 0.
//
    if (expires < 0) {
    expires = 0;
    }
    if (timer.is_soft) {
//
// soft hrtimer could be started on a remote CPU. In this
// case softirq_expires_next needs to be updated on the
// remote CPU. The soft hrtimer will not expire before the
// first hard hrtimer on the remote CPU -
// hrtimer_check_target() prevents this case.
//
    let mut timer_cpu_base = base.cpu_base;
    if (timer_cpu_base.softirq_activated) {
    return;
    }
    if (!ktime_before(expires, timer_cpu_base.softirq_expires_next)) {
    return;
    }
    timer_cpu_base.softirq_next_timer = timer;
    timer_cpu_base.softirq_expires_next = expires;
    if (!ktime_before(expires, timer_cpu_base.expires_next) || !reprogram) {
    return;
    }
    }
//
// If the timer is not on the current cpu, we cannot reprogram
// the other cpus clock event device.
//
    if (base.cpu_base != cpu_base) {
    return;
    }
    if (expires >= cpu_base.expires_next) {
    return;
    }
// If a deferred rearm is pending skip reprogramming the device
    if (cpu_base.deferred_rearm) {
    return;
    }
    cpu_base.next_timer = timer;
    __hrtimer_reprogram(cpu_base, expires);
    }
#[no_mangle]
unsafe extern "C" fn update_needs_ipi(cpu_base: *mut hrtimer_cpu_base, active: c_uint) -> bool {
pub static mut base: *mut c_void = core::ptr::null_mut();
    let mut expires;
    let mut seq = 0;
//
// Update the base offsets unconditionally so the following
// checks whether the SMP function call is required works.
//
// The update is safe even when the remote CPU is in the hrtimer
// interrupt or the hrtimer soft interrupt and expiring affected
// bases. Either it will see the update before handling a base or
// it will see it when it finishes the processing and reevaluates
// the next expiring timer.
//
    seq = cpu_base.clock_was_set_seq;
    hrtimer_update_base(cpu_base);
//
// If the sequence did not change over the update then the
// remote CPU already handled it.
//
    if (seq == cpu_base.clock_was_set_seq) {
    return false;
    }
// If a deferred rearm is pending the remote CPU will take care of it
    if (cpu_base.deferred_rearm) {
    cpu_base.deferred_needs_update = true;
    return false;
    }
//
// Walk the affected clock bases and check whether the first expiring
// timer in a clock base is moving ahead of the first expiring timer of
// @cpu_base. If so, the IPI must be invoked because per CPU clock
// event devices cannot be remotely reprogrammed.
//
    active &= cpu_base.active_bases;
    for_each_active_base(base, cpu_base, active) {
pub static mut next: *mut c_void = core::ptr::null_mut();
    next = timerqueue_linked_first(&base.active);
    expires = ktime_sub(next.expires, base.offset);
    if (expires < cpu_base.expires_next) {
    return true;
    }
// Extra check for softirq clock bases
    if (base.index < HRTIMER_BASE_MONOTONIC_SOFT) {
    continue;
    }
    if (cpu_base.softirq_activated) {
    continue;
    }
    if (expires < cpu_base.softirq_expires_next) {
    return true;
    }
    }
    return false;
    }
//
// Clock was set. This might affect CLOCK_REALTIME, CLOCK_TAI and
// CLOCK_BOOTTIME (for late sleep time injection).
//
// This requires to update the offsets for these clocks
// vs. CLOCK_MONOTONIC. When high resolution timers are enabled, then this
// also requires to eventually reprogram the per CPU clock event devices
// when the change moves an affected timer ahead of the first expiring
// timer on that CPU. Obviously remote per CPU clock event devices cannot
// be reprogrammed. The other reason why an IPI has to be sent is when the
// system is in !HIGH_RES and NOHZ mode. The NOHZ mode updates the offsets
// in the tick, which obviously might be stopped, so this has to bring out
// the remote CPU which might sleep in idle to get this sorted.
//
#[no_mangle]
pub unsafe extern "C" fn clock_was_set(bases: c_uint) {
    let mut mask;
    if (!hrtimer_highres_enabled() && !tick_nohz_is_active()) {
// goto;
    }
    if (!zalloc_cpumask_var(&mask, GFP_KERNEL)) {
    on_each_cpu(retrigger_next_event, core::ptr::null_mut(), 1);
// goto;
    }
// Avoid interrupting CPUs if possible
    scoped_guard(cpus_read_lock) {
    let mut cpu = 0;
    for_each_online_cpu(cpu) {
    let mut cpu_base = &per_cpu(hrtimer_bases, cpu);
    guard(raw_spinlock_irqsave)(&cpu_base.lock);
    if (update_needs_ipi(cpu_base, bases)) {
    cpumask_set_cpu(cpu, mask);
    }
    }
    scoped_guard(preempt)
    smp_call_function_many(mask, retrigger_next_event, core::ptr::null_mut(), 1);
    }
    free_cpumask_var(mask);
// label;
    timerfd_clock_was_set();
    }
#[no_mangle]
unsafe extern "C" fn clock_was_set_work(work: *mut work_struct) {
    clock_was_set(CLOCK_SET_WALL);
    }
pub static mut hrtimer_work: usize = 0;
//
// Called from timekeeping code to reprogram the hrtimer interrupt device
// on all cpus and to notify timerfd.
//
#[no_mangle]
pub unsafe extern "C" fn clock_was_set_delayed() {
    schedule_work(&hrtimer_work);
    }
//
// Called during resume either directly from via timekeeping_resume()
// or in the case of s2idle from tick_unfreeze() to ensure that the
// hrtimers are up to date.
//
#[no_mangle]
pub unsafe extern "C" fn hrtimers_resume_local() {
    lockdep_assert_irqs_disabled();
// Retrigger on the local CPU
    retrigger_next_event(core::ptr::null_mut());
    }
// Counterpart to lock_hrtimer_base above
#[no_mangle]
pub unsafe extern "C" fn unlock_hrtimer_base(timer: *const hrtimer, flags: *mut c_ulong) {
    raw_spin_unlock_irqrestore(&timer.base.cpu_base.lock, *flags);
    }
//
// hrtimer_update_function - Update the timer's callback function
// @timer:	Timer to update
// @function:	New callback function
//
// Only safe to call if the timer is not enqueued. Can be called in the callback function if the
// timer is not enqueued at the same time (see the comments above HRTIMER_STATE_ENQUEUED).
//
    void hrtimer_update_function(hrtimer *timer,
#[no_mangle]
pub unsafe extern "C" fn hrtimer_restart(): *mut *mut function)(hrtimer) -> enum {
    enum hrtimer_restart (*function))
    {

    guard(raw_spinlock_irqsave)(&timer.base.cpu_base.lock);
    if (WARN_ON_ONCE!(hrtimer_is_queued(timer))) {
    return;
    }
    if (WARN_ON_ONCE!(!function)) {
    return;
    }

    ACCESS_PRIVATE(timer, function) = function;
    }
    EXPORT_SYMBOL_GPL(hrtimer_update_function);
//
// hrtimer_forward() - forward the timer expiry
// @timer:	hrtimer to forward
// @now:	forward past this time
// @interval:	the interval to forward
//
// Forward the timer expiry so it will expire in the future.
//
// .. note::
// This only updates the timer expiry value and does not requeue the timer.
//
// There is also a variant of this function: hrtimer_forward_now().
//
// Context: Can be safely called from the callback function of @timer. If called
// from other contexts @timer must neither be enqueued nor running the
// callback and the caller needs to take care of serialization.
//
// Return: The number of overruns are returned.
//
#[no_mangle]
pub unsafe extern "C" fn hrtimer_forward(timer: *mut hrtimer, now: ktime_t, interval: ktime_t) -> u64 {
    let mut delta;
pub static mut orun: u64 = 1;
    delta = ktime_sub(now, hrtimer_get_expires(timer));
    if (delta < 0) {
    return 0;
    }
    if (WARN_ON!(timer.is_queued)) {
    return 0;
    }
    if (interval < hrtimer_resolution) {
    interval = hrtimer_resolution;
    }
    if (unlikely(delta >= interval)) {
pub static mut incr: i64 = 0;
    orun = ktime_divns(delta, incr);
    hrtimer_add_expires_ns(timer, incr * orun);
    if (hrtimer_get_expires(timer) > now) {
    return orun;
    }
//
// This (and the ktime_add() below) is the
// correction for exact:
//
    orun += 1;
    }
    hrtimer_add_expires(timer, interval);
    return orun;
    }
    EXPORT_SYMBOL_GPL(hrtimer_forward);
//
// enqueue_hrtimer - internal function to (re)start a timer
//
// The timer is inserted in expiry order. Insertion into the
// red black tree is O(log(n)).
//
// Returns true when the new timer is the leftmost timer in the tree.
//
#[no_mangle]
pub unsafe extern "C" fn enqueue_hrtimer(timer: *mut hrtimer, base: *mut hrtimer_clock_base, mode: hrtimer_mode, was_armed: bool) -> bool {
    lockdep_assert_held(&base.cpu_base.lock);
    debug_activate(timer, mode, was_armed);
    WARN_ON_ONCE!(!base.cpu_base.online);
    base.cpu_base.active_bases |= 1 << base.index;
// Pairs with the lockless read in hrtimer_is_queued()
    WRITE_ONCE(timer.is_queued, HRTIMER_STATE_ENQUEUED);
    if (!timerqueue_linked_add(&base.active, &timer.node)) {
    return false;
    }
    base.expires_next = hrtimer_get_expires(timer);
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn base_update_next_timer(base: *mut hrtimer_clock_base) {
    let mut next = timerqueue_linked_first(&base.active);
    base.expires_next = next ? next.expires : KTIME_MAX;
    }
//
// __remove_hrtimer - internal function to remove a timer
//
// High resolution timer mode reprograms the clock event device when the
// timer is the one which expires next. The caller can disable this by setting
// reprogram to zero. This is useful, when the context does a reprogramming
// anyway (e.g. timer interrupt)
//
#[no_mangle]
pub unsafe extern "C" fn __remove_hrtimer(timer: *mut hrtimer, base: *mut hrtimer_clock_base, newstate: bool, reprogram: bool) {
    let mut cpu_base = base.cpu_base;
    let mut was_first = 0;
    lockdep_assert_held(&cpu_base.lock);
    if (!timer.is_queued) {
    return;
    }
// Pairs with the lockless read in hrtimer_is_queued()
    WRITE_ONCE(timer.is_queued, newstate);
    was_first = !timerqueue_linked_prev(&timer.node);
    if (!timerqueue_linked_del(&base.active, &timer.node)) {
    cpu_base.active_bases &= ~(1 << base.index);
    }
// Nothing to update if this was not the first timer in the base
    if (!was_first) {
    return;
    }
    base_update_next_timer(base);
//
// If reprogram is false don't update cpu_base->next_timer and do not
// touch the clock event device.
//
// This happens when removing the first timer on a remote CPU, which
// will be handled by the remote CPU's interrupt. It also happens when
// a local timer is removed to be immediately restarted. That's handled
// at the call site.
//
    if (!reprogram || timer != cpu_base.next_timer || timer.is_lazy) {
    return;
    }
    if (cpu_base.deferred_rearm) {
    cpu_base.deferred_needs_update = true;
    }
    else {
    hrtimer_force_reprogram(cpu_base, /* skip_equal */ true);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn remove_hrtimer(timer: *mut hrtimer, base: *mut hrtimer_clock_base, newstate: bool) -> bool {
    lockdep_assert_held(&base.cpu_base.lock);
    if (timer.is_queued) {
    let mut reprogram = 0;
    debug_hrtimer_deactivate(timer);
//
// Remove the timer and force reprogramming when high
// resolution mode is active and the timer is on the current
// CPU. If we remove a timer on another CPU, reprogramming is
// skipped. The interrupt event on this CPU is fired and
// reprogramming happens in the interrupt handler. This is a
// rare case and less expensive than a smp call.
//
    reprogram = base.cpu_base == this_cpu_ptr(&hrtimer_bases);
    __remove_hrtimer(timer, base, newstate, reprogram);
    return true;
    }
    return false;
    }
//
// Update in place has to retrieve the expiry times of the neighbour nodes
// if they exist. That is cache line neutral because the dequeue/enqueue
// operation is going to need the same cache lines. But there is a big win
// when the dequeue/enqueue can be avoided because the RB tree does not
// have to be rebalanced twice.
//
#[no_mangle]
pub unsafe extern "C" fn hrtimer_can_update_in_place(timer: *mut hrtimer, base: *mut hrtimer_clock_base, expires: ktime_t) -> bool {
    let mut next = timerqueue_linked_next(&timer.node);
    let mut prev = timerqueue_linked_prev(&timer.node);
// If the new expiry goes behind the next timer, requeue is required
    if (next && expires > next.expires) {
    return false;
    }
// If this is the first timer, update in place
    if (!prev) {
    return true;
    }
// Update in place when it does not go ahead of the previous one
    return expires >= prev.expires;
    }
#[no_mangle]
pub unsafe extern "C" fn remove_and_enqueue_same_base(timer: *mut hrtimer, base: *mut hrtimer_clock_base, mode: hrtimer_mode, expires: ktime_t, delta_ns: u64) -> bool {
pub static mut was_first: bool = false;
// Remove it from the timer queue if active
    if (timer.is_queued) {
    was_first = !timerqueue_linked_prev(&timer.node);
// Try to update in place to avoid the de/enqueue dance
    if (hrtimer_can_update_in_place(timer, base, expires)) {
    hrtimer_set_expires_range_ns(timer, expires, delta_ns);
    trace_hrtimer_start(timer, mode, true);
    if (was_first) {
    base.expires_next = expires;
    }
    return was_first;
    }
    debug_hrtimer_deactivate(timer);
    timerqueue_linked_del(&base.active, &timer.node);
    }
// Set the new expiry time
    hrtimer_set_expires_range_ns(timer, expires, delta_ns);
    debug_activate(timer, mode, timer.is_queued);
    base.cpu_base.active_bases |= 1 << base.index;
// Pairs with the lockless read in hrtimer_is_queued()
    WRITE_ONCE(timer.is_queued, HRTIMER_STATE_ENQUEUED);
// If it's the first expiring timer now or again, update base
    if (timerqueue_linked_add(&base.active, &timer.node)) {
    base.expires_next = expires;
    return true;
    }
    if (was_first) {
    base_update_next_timer(base);
    }
    return false;
    }
    static inline ktime_t hrtimer_update_lowres(hrtimer *timer, ktime_t tim,
    const enum hrtimer_mode mode)
    {

//
// CONFIG_TIME_LOW_RES indicates that the system has no way to return
// granular time values. For relative timers we add hrtimer_resolution
// (i.e. one jiffy) to prevent short timeouts.
//
    timer.is_rel = mode & HRTIMER_MODE_REL;
    if (timer.is_rel) {
    tim = ktime_add_safe(tim, hrtimer_resolution);
    }

    return tim;
    }
#[no_mangle]
unsafe extern "C" fn hrtimer_update_softirq_timer(cpu_base: *mut hrtimer_cpu_base, reprogram: bool) {
pub static mut expires: ktime_t = 0;
//
// Reprogramming needs to be triggered, even if the next soft
// hrtimer expires at the same time as the next hard
// hrtimer. cpu_base->softirq_expires_next needs to be updated!
//
    if (expires == KTIME_MAX) {
    return;
    }
//
// cpu_base->next_timer is recomputed by __hrtimer_get_next_event()
// cpu_base->expires_next is only set by hrtimer_reprogram()
//
    hrtimer_reprogram(cpu_base.softirq_next_timer, reprogram);
    }

#[no_mangle]
unsafe extern "C" fn hrtimer_prefer_local(is_local: bool, is_first: bool, is_pinned: bool) -> __always_inline bool {
    if (static_branch_likely(&timers_migration_enabled)) {
//
// If it is local and the first expiring timer keep it on the local
// CPU to optimize reprogramming of the clockevent device. Also
// avoid switch_hrtimer_base() overhead when local and pinned.
//
    if (!is_local) {
    return false;
    }
    if (is_first || is_pinned) {
    return true;
    }
// Honour the NOHZ full restrictions
    if (!housekeeping_cpu(smp_processor_id(), HK_TYPE_KERNEL_NOISE)) {
    return false;
    }
//
// If the tick is not stopped or need_resched() is set, then
// there is no point in moving the timer somewhere else.
//
    return !tick_nohz_tick_stopped() || need_resched();
    }
    return is_local;
    }

#[no_mangle]
unsafe extern "C" fn hrtimer_prefer_local(is_local: bool, is_first: bool, is_pinned: bool) -> __always_inline bool {
    return is_local;
    }

#[no_mangle]
pub unsafe extern "C" fn hrtimer_keep_base(timer: *mut hrtimer, is_local: bool, is_first: bool, is_pinned: bool) -> bool {
// If the timer is running the callback it has to stay on its CPU base.
    if (unlikely(timer.base.running == timer)) {
    return true;
    }
    return hrtimer_prefer_local(is_local, is_first, is_pinned);
    }
    enum {
    HRTIMER_REPROGRAM_NONE,
    HRTIMER_REPROGRAM,
    HRTIMER_REPROGRAM_FORCE,
    };
#[no_mangle]
pub unsafe extern "C" fn __hrtimer_start_range_ns(timer: *mut hrtimer, tim: ktime_t, delta_ns: u64, mode: hrtimer_mode, base: *mut hrtimer_clock_base) -> c_int {
    let mut this_cpu_base = this_cpu_ptr(&hrtimer_bases);
    bool is_pinned, first, was_first, keep_base = false;
    let mut cpu_base = base.cpu_base;
    was_first = cpu_base.next_timer == timer;
    is_pinned = !!(mode & HRTIMER_MODE_PINNED);
//
// Don't keep it local if this enqueue happens on a unplugged CPU
// after hrtimer_cpu_dying() has been invoked.
//
    if (likely(this_cpu_base.online)) {
pub static mut is_local: bool = false;
    keep_base = hrtimer_keep_base(timer, is_local, was_first, is_pinned);
    }
// Calculate absolute expiry time for relative timers
    if (mode & HRTIMER_MODE_REL) {
    tim = ktime_add_safe(tim, __hrtimer_cb_get_time(base.clockid));
    }
// Compensate for low resolution granularity
    tim = hrtimer_update_lowres(timer, tim, mode);
//
// Remove an active timer from the queue. In case it is not queued
// on the current CPU, make sure that remove_hrtimer() updates the
// remote data correctly.
//
// If it's on the current CPU and the first expiring timer, then
// skip reprogramming, keep the timer local and enforce
// reprogramming later if it was the first expiring timer.  This
// avoids programming the underlying clock event twice (once at
// removal and once after enqueue).
//
// @keep_base is also true if the timer callback is running on a
// remote CPU and for local pinned timers.
//
    if (likely(keep_base)) {
    first = remove_and_enqueue_same_base(timer, base, mode, tim, delta_ns);
    } else {
// Keep the ENQUEUED state in case it is queued
pub static mut was_armed: bool = false;
    hrtimer_set_expires_range_ns(timer, tim, delta_ns);
// Switch the timer base, if necessary:
    base = switch_hrtimer_base(timer, base, is_pinned);
    cpu_base = base.cpu_base;
    first = enqueue_hrtimer(timer, base, mode, was_armed);
    }
// If a deferred rearm is pending skip reprogramming the device
    if (cpu_base.deferred_rearm) {
    cpu_base.deferred_needs_update = true;
    return HRTIMER_REPROGRAM_NONE;
    }
    if (!was_first || cpu_base != this_cpu_base) {
//
// If the current CPU base is online, then the timer is never
// queued on a remote CPU if it would be the first expiring
// timer there unless the timer callback is currently executed
// on the remote CPU. In the latter case the remote CPU will
// re-evaluate the first expiring timer after completing the
// callbacks.
//
    if (likely(hrtimer_base_is_online(this_cpu_base))) {
    return first ? HRTIMER_REPROGRAM : HRTIMER_REPROGRAM_NONE;
    }
//
// Timer was enqueued remote because the current base is
// already offline. If the timer is the first to expire,
// kick the remote CPU to reprogram the clock event.
//
    if (first) {
    smp_call_function_single_async(cpu_base.cpu, &cpu_base.csd);
    }
    return HRTIMER_REPROGRAM_NONE;
    }
//
// Special case for the HRTICK timer. It is frequently rearmed and most
// of the time moves the expiry into the future. That's expensive in
// virtual machines and it's better to take the pointless already armed
// interrupt than reprogramming the hardware on every context switch.
//
// If the new expiry is before the armed time, then reprogramming is
// required.
//
    if (timer.is_lazy) {
    if (cpu_base.expires_next <= hrtimer_get_expires(timer)) {
    return HRTIMER_REPROGRAM_NONE;
    }
    }
//
// Timer was the first expiring timer and forced to stay on the
// current CPU to avoid reprogramming on removal and enqueue. Force
// reprogram the hardware by evaluating the new first expiring
// timer.
//
    return HRTIMER_REPROGRAM_FORCE;
    }
#[no_mangle]
pub unsafe extern "C" fn hrtimer_start_range_ns_common(timer: *mut hrtimer, tim: ktime_t, delta_ns: u64, mode: hrtimer_mode, base: *mut hrtimer_clock_base) -> c_int {
//
// Check whether the HRTIMER_MODE_SOFT bit and hrtimer.is_soft
// match on CONFIG_PREEMPT_RT = n. With PREEMPT_RT check the hard
// expiry mode because unmarked timers are moved to softirq expiry.
//
    if (!IS_ENABLED!(CONFIG_PREEMPT_RT)) {
    WARN_ON_ONCE!(!(mode & HRTIMER_MODE_SOFT) ^ !timer.is_soft);
    }
    else {
    WARN_ON_ONCE!(!(mode & HRTIMER_MODE_HARD) ^ !timer.is_hard);
    }
    return __hrtimer_start_range_ns(timer, tim, delta_ns, mode, base);
    }
//
// hrtimer_start_range_ns - (re)start an hrtimer
// @timer:	the timer to be added
// @tim:	expiry time
// @delta_ns:	"slack" range for the timer
// @mode:	timer mode: absolute (HRTIMER_MODE_ABS) or
// relative (HRTIMER_MODE_REL), and pinned (HRTIMER_MODE_PINNED);
// softirq based mode is considered for debug purpose only!
//
#[no_mangle]
pub unsafe extern "C" fn hrtimer_start_range_ns(timer: *mut hrtimer, tim: ktime_t, delta_ns: u64, mode: hrtimer_mode) {
pub static mut base: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
    debug_hrtimer_assert_init(timer);
    base = lock_hrtimer_base(timer, &flags);
    switch (hrtimer_start_range_ns_common(timer, tim, delta_ns, mode, base)) {
    case HRTIMER_REPROGRAM:
    hrtimer_reprogram(timer, true);
    break;
    case HRTIMER_REPROGRAM_FORCE:
    hrtimer_force_reprogram(timer.base.cpu_base, 1);
    break;
    case HRTIMER_REPROGRAM_NONE:
    break;
    }
    unlock_hrtimer_base(timer, &flags);
    }
    EXPORT_SYMBOL_GPL(hrtimer_start_range_ns);
#[no_mangle]
pub unsafe extern "C" fn hrtimer_check_user_timer(timer: *mut hrtimer) -> bool {
    let mut cpu_base = timer.base.cpu_base;
    let mut expires;
//
// This uses soft expires because that's the user provided
// expiry time, while expires can be further in the past
// due to a slack value added to the user expiry time.
//
    expires = hrtimer_get_softexpires(timer);
// Convert to monotonic
    expires = ktime_sub(expires, timer.base.offset);
//
// Check whether this timer will end up as the first expiring timer in
// the CPU base. If not, no further checks required as it's then
// guaranteed to expire in the future.
//
    if (expires >= cpu_base.expires_next) {
    return true;
    }
// Validate that the expiry time is in the future.
    if (expires > ktime_get()) {
    return true;
    }
    debug_hrtimer_deactivate(timer);
    __remove_hrtimer(timer, timer.base, HRTIMER_STATE_INACTIVE, false);
    trace_hrtimer_start_expired(timer);
    return false;
    }
//
// hrtimer_start_range_ns_user - (re)start an user controlled hrtimer
// @timer:	the timer to be added
// @tim:	expiry time
// @delta_ns:	"slack" range for the timer
// @mode:	timer mode: absolute (HRTIMER_MODE_ABS) or
// relative (HRTIMER_MODE_REL), and pinned (HRTIMER_MODE_PINNED);
// softirq based mode is considered for debug purpose only!
//
// Returns: True when the timer was queued, false if it was already expired
//
// This function cannot invoke the timer callback for expired timers as it might
// be called under a lock which the timer callback needs to acquire. So the
// caller has to handle that case.
//
#[no_mangle]
pub unsafe extern "C" fn hrtimer_start_range_ns_user(timer: *mut hrtimer, tim: ktime_t, delta_ns: u64, mode: hrtimer_mode) -> bool {
pub static mut base: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
pub static mut ret: bool = true;
    debug_hrtimer_assert_init(timer);
    base = lock_hrtimer_base(timer, &flags);
    switch (hrtimer_start_range_ns_common(timer, tim, delta_ns, mode, base)) {
    case HRTIMER_REPROGRAM:
    ret = hrtimer_check_user_timer(timer);
    if (ret) {
    hrtimer_reprogram(timer, true);
    }
    break;
    case HRTIMER_REPROGRAM_FORCE:
    ret = hrtimer_check_user_timer(timer);
//
// The base must always be reevaluated, independent of the
// result above because the timer was the first pending timer.
//
    hrtimer_force_reprogram(timer.base.cpu_base, 1);
    break;
    case HRTIMER_REPROGRAM_NONE:
    break;
    }
    unlock_hrtimer_base(timer, &flags);
    return ret;
    }
    EXPORT_SYMBOL_GPL(hrtimer_start_range_ns_user);
//
// hrtimer_try_to_cancel - try to deactivate a timer
// @timer:	hrtimer to stop
//
// Returns:
//
// *  0 when the timer was not active
// *  1 when the timer was active
// * -1 when the timer is currently executing the callback function and
// cannot be stopped
//
#[no_mangle]
pub unsafe extern "C" fn hrtimer_try_to_cancel(timer: *mut hrtimer) -> c_int {
pub static mut base: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
pub static mut ret: c_int = 0;
//
// Check lockless first. If the timer is not active (neither
// enqueued nor running the callback, nothing to do here.  The
// base lock does not serialize against a concurrent enqueue,
// so we can avoid taking it.
//
    if (!hrtimer_active(timer)) {
    return 0;
    }
    base = lock_hrtimer_base(timer, &flags);
    if (!hrtimer_callback_running(timer)) {
    ret = remove_hrtimer(timer, base, HRTIMER_STATE_INACTIVE);
    if (ret) {
    trace_hrtimer_cancel(timer);
    }
    }
    unlock_hrtimer_base(timer, &flags);
    return ret;
    }
    EXPORT_SYMBOL_GPL(hrtimer_try_to_cancel);

#[no_mangle]
unsafe extern "C" fn hrtimer_cpu_base_init_expiry_lock(base: *mut hrtimer_cpu_base) {
    spin_lock_init(&base.softirq_expiry_lock);
    }
#[no_mangle]
unsafe extern "C" fn hrtimer_cpu_base_lock_expiry(base: *mut hrtimer_cpu_base) {
    spin_lock(&base.softirq_expiry_lock);
    }
#[no_mangle]
unsafe extern "C" fn hrtimer_cpu_base_unlock_expiry(base: *mut hrtimer_cpu_base) {
    spin_unlock(&base.softirq_expiry_lock);
    }
//
// The counterpart to hrtimer_cancel_wait_running().
//
// If there is a waiter for cpu_base->expiry_lock, then it was waiting for
// the timer callback to finish. Drop expiry_lock and reacquire it. That
// allows the waiter to acquire the lock and make progress.
//
#[no_mangle]
unsafe extern "C" fn hrtimer_sync_wait_running(cpu_base: *mut hrtimer_cpu_base, flags: c_ulong) {
    if (atomic_read(&cpu_base.timer_waiters)) {
    raw_spin_unlock_irqrestore(&cpu_base.lock, flags);
    spin_unlock(&cpu_base.softirq_expiry_lock);
    spin_lock(&cpu_base.softirq_expiry_lock);
    raw_spin_lock_irq(&cpu_base.lock);
    }
    }

#[no_mangle]
unsafe extern "C" fn is_migration_base(base: *mut hrtimer_clock_base) -> __always_inline bool {
pub static mut base: return = 0;
    }

#[no_mangle]
unsafe extern "C" fn is_migration_base(base: *mut hrtimer_clock_base) -> __always_inline bool {
    return false;
    }

//
// This function is called on PREEMPT_RT kernels when the fast path
// deletion of a timer failed because the timer callback function was
// running.
//
// This prevents priority inversion: if the soft irq thread is preempted
// in the middle of a timer callback, then calling hrtimer_cancel() can
// lead to two issues:
//
// - If the caller is on a remote CPU then it has to spin wait for the timer
// handler to complete. This can result in unbound priority inversion.
//
// - If the caller originates from the task which preempted the timer
// handler on the same CPU, then spin waiting for the timer handler to
// complete is never going to end.
//
#[no_mangle]
pub unsafe extern "C" fn hrtimer_cancel_wait_running(timer: *const hrtimer) {
// Lockless read. Prevent the compiler from reloading it below
    let mut base = READ_ONCE(timer.base);
//
// Just relax if the timer expires in hard interrupt context or if
// it is currently on the migration base.
//
    if (!timer.is_soft || is_migration_base(base)) {
    cpu_relax();
    return;
    }
//
// Mark the base as contended and grab the expiry lock, which is
// held by the softirq across the timer callback. Drop the lock
// immediately so the softirq can expire the next timer. In theory
// the timer could already be running again, but that's more than
// unlikely and just causes another wait loop.
//
    atomic_inc(&base.cpu_base.timer_waiters);
    spin_lock_bh(&base.cpu_base.softirq_expiry_lock);
    atomic_dec(&base.cpu_base.timer_waiters);
    spin_unlock_bh(&base.cpu_base.softirq_expiry_lock);
    }

#[no_mangle]
pub unsafe extern "C" fn hrtimer_cpu_base_init_expiry_lock(base: *mut hrtimer_cpu_base) { }
#[no_mangle]
pub unsafe extern "C" fn hrtimer_cpu_base_lock_expiry(base: *mut hrtimer_cpu_base) { }
#[no_mangle]
pub unsafe extern "C" fn hrtimer_cpu_base_unlock_expiry(base: *mut hrtimer_cpu_base) { }
#[no_mangle]
pub unsafe extern "C" fn hrtimer_sync_wait_running(base: *mut hrtimer_cpu_base, fl: c_ulong) { }

//
// hrtimer_cancel - cancel a timer and wait for the handler to finish.
// @timer:	the timer to be cancelled
//
// Returns:
// 0 when the timer was not active
// 1 when the timer was active
//
#[no_mangle]
pub unsafe extern "C" fn hrtimer_cancel(timer: *mut hrtimer) -> c_int {
    let mut ret = 0;
    do {
    ret = hrtimer_try_to_cancel(timer);
    if (ret < 0) {
    hrtimer_cancel_wait_running(timer);
    }
    } while (ret < 0);
    return ret;
    }
    EXPORT_SYMBOL_GPL(hrtimer_cancel);
//
// __hrtimer_get_remaining - get remaining time for the timer
// @timer:	the timer to read
// @adjust:	adjust relative timers when CONFIG_TIME_LOW_RES=y
//
#[no_mangle]
pub unsafe extern "C" fn __hrtimer_get_remaining(timer: *const hrtimer, adjust: bool) -> ktime_t {
    let mut flags = 0;
    let mut rem;
    lock_hrtimer_base(timer, &flags);
    if (IS_ENABLED!(CONFIG_TIME_LOW_RES) && adjust) {
    rem = hrtimer_expires_remaining_adjusted(timer);
    }
    else {
    rem = hrtimer_expires_remaining(timer);
    }
    unlock_hrtimer_base(timer, &flags);
    return rem;
    }
    EXPORT_SYMBOL_GPL(__hrtimer_get_remaining);

//
// hrtimer_get_next_event - get the time until next expiry event
//
// Returns the next expiry time or KTIME_MAX if no timer is pending.
//
#[no_mangle]
pub unsafe extern "C" fn hrtimer_get_next_event() -> ktime_t {
    let mut cpu_base = this_cpu_ptr(&hrtimer_bases);
//
// When HRES is active cmp_next_hrtimer_event() expects KTIME_MAX.
//
// cpu_base->hres_active is written only by the local CPU in
// hrtimer_switch_to_hres() from hard interrupt context and in
// hrtimers_cpu_starting() during CPU bring-up, and all callers reach
// this with interrupts disabled on the same CPU, so an unlocked read is
// stable without holding the lock.
//
    if (hrtimer_hres_active(cpu_base)) {
    return KTIME_MAX;
    }
    guard(raw_spinlock_irqsave)(&cpu_base.lock);
    return __hrtimer_get_next_event(cpu_base, HRTIMER_ACTIVE_ALL);
    }
//
// hrtimer_next_event_without - time until next expiry event w/o one timer
// @exclude:	timer to exclude
//
// Returns the next expiry time over all timers except for the @exclude one or
// KTIME_MAX if none of them is pending.
//
#[no_mangle]
pub unsafe extern "C" fn hrtimer_next_event_without(exclude: *const hrtimer) -> ktime_t {
    let mut cpu_base = this_cpu_ptr(&hrtimer_bases);
pub static mut expires: ktime_t = 0;
    let mut active = 0;
    guard(raw_spinlock_irqsave)(&cpu_base.lock);
    if (!hrtimer_hres_active(cpu_base)) {
    return expires;
    }
    active = cpu_base.active_bases & HRTIMER_ACTIVE_SOFT;
    if (active && !cpu_base.softirq_activated) {
    expires = hrtimer_bases_next_event_without(cpu_base, exclude, active, KTIME_MAX);
    }
    active = cpu_base.active_bases & HRTIMER_ACTIVE_HARD;
    if (!active) {
    return expires;
    }
    return hrtimer_bases_next_event_without(cpu_base, exclude, active, expires);
    }

#[no_mangle]
pub unsafe extern "C" fn hrtimer_clockid_to_base(clock_id: clockid_t) -> c_int {
    match (clock_id) {
    CLOCK_MONOTONIC => {
    return HRTIMER_BASE_MONOTONIC;
    }
    CLOCK_REALTIME => {
    return HRTIMER_BASE_REALTIME;
    }
    CLOCK_BOOTTIME => {
    return HRTIMER_BASE_BOOTTIME;
    }
    CLOCK_TAI => {
    return HRTIMER_BASE_TAI;
    }
    _ => {
    WARN(1, "Invalid clockid %d. Using MONOTONIC\n", clock_id);
    return HRTIMER_BASE_MONOTONIC;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn __hrtimer_cb_get_time(clock_id: clockid_t) -> ktime_t {
    match (clock_id) {
    CLOCK_MONOTONIC => {
    return ktime_get();
    }
    CLOCK_REALTIME => {
    return ktime_get_real();
    }
    CLOCK_BOOTTIME => {
    return ktime_get_boottime();
    }
    CLOCK_TAI => {
    return ktime_get_clocktai();
    }
    _ => {
    WARN(1, "Invalid clockid %d. Using MONOTONIC\n", clock_id);
    return ktime_get();
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn hrtimer_cb_get_time(timer: *const hrtimer) -> ktime_t {
    return __hrtimer_cb_get_time(timer.base.clockid);
    }
    EXPORT_SYMBOL_GPL(hrtimer_cb_get_time);
#[no_mangle]
pub unsafe extern "C" fn __hrtimer_setup(timer: *mut hrtimer, clock_id: clockid_t, mode: hrtimer_mode) {
pub static mut softtimer: bool = false;
pub static mut cpu_base: *mut c_void = core::ptr::null_mut();
    let mut base = 0;
//
// On PREEMPT_RT enabled kernels hrtimers which are not explicitly
// marked for hard interrupt expiry mode are moved into soft
// interrupt context for latency reasons and because the callbacks
// can invoke functions which might sleep on RT, e.g. spin_lock().
//
    if (IS_ENABLED!(CONFIG_PREEMPT_RT) && !(mode & HRTIMER_MODE_HARD)) {
    softtimer = true;
    }
    memset(timer, 0, sizeof!(hrtimer));
    cpu_base = raw_cpu_ptr(&hrtimer_bases);
//
// POSIX magic: Relative CLOCK_REALTIME timers are not affected by
// clock modifications, so they needs to become CLOCK_MONOTONIC to
// ensure POSIX compliance.
//
    if (clock_id == CLOCK_REALTIME && mode & HRTIMER_MODE_REL) {
    clock_id = CLOCK_MONOTONIC;
    }
    base = softtimer ? HRTIMER_MAX_CLOCK_BASES / 2 : 0;
    base += hrtimer_clockid_to_base(clock_id);
    timer.is_soft = softtimer;
    timer.is_hard = !!(mode & HRTIMER_MODE_HARD);
    timer.is_lazy = !!(mode & HRTIMER_MODE_LAZY_REARM);
    timer.base = &cpu_base.clock_base[base];
    timerqueue_linked_init(&timer.node);
    if (WARN_ON_ONCE!(!fn)) {
    ACCESS_PRIVATE(timer, function) = hrtimer_dummy_timeout;
    }
    else {
    ACCESS_PRIVATE(timer, function) = fn;
    }
    }
//
// hrtimer_setup - initialize a timer to the given clock
// @timer:	the timer to be initialized
// @function:	the callback function
// @clock_id:	the clock to be used
// @mode:       The modes which are relevant for initialization:
// HRTIMER_MODE_ABS, HRTIMER_MODE_REL, HRTIMER_MODE_ABS_SOFT,
// HRTIMER_MODE_REL_SOFT
//
// The PINNED variants of the above can be handed in,
// but the PINNED bit is ignored as pinning happens
// when the hrtimer is started
//
#[no_mangle]
pub unsafe extern "C" fn hrtimer_setup(timer: *mut hrtimer, clock_id: clockid_t, mode: hrtimer_mode) {
    debug_setup(timer, clock_id, mode);
    __hrtimer_setup(timer, function, clock_id, mode);
    }
    EXPORT_SYMBOL_GPL(hrtimer_setup);
//
// hrtimer_setup_on_stack - initialize a timer on stack memory
// @timer:	The timer to be initialized
// @function:	the callback function
// @clock_id:	The clock to be used
// @mode:       The timer mode
//
// Similar to hrtimer_setup(), except that this one must be used if struct hrtimer is in stack
// memory.
//
#[no_mangle]
pub unsafe extern "C" fn hrtimer_setup_on_stack(timer: *mut hrtimer, clock_id: clockid_t, mode: hrtimer_mode) {
    debug_setup_on_stack(timer, clock_id, mode);
    __hrtimer_setup(timer, function, clock_id, mode);
    }
    EXPORT_SYMBOL_GPL(hrtimer_setup_on_stack);
//
// A timer is active, when it is enqueued into the rbtree or the
// callback function is running or it's in the state of being migrated
// to another cpu.
//
// It is important for this function to not return a false negative.
//
#[no_mangle]
pub unsafe extern "C" fn hrtimer_active(timer: *const hrtimer) -> bool {
pub static mut base: *mut c_void = core::ptr::null_mut();
    let mut seq = 0;
    do {
    base = READ_ONCE(timer.base);
    seq = raw_read_seqcount_begin(&base.seq);
    if (timer.is_queued || base.running == timer) {
    return true;
    }
    } while (read_seqcount_retry(&base.seq, seq) || base != READ_ONCE(timer.base));
    return false;
    }
    EXPORT_SYMBOL_GPL(hrtimer_active);
//
// The write_seqcount_barrier()s in __run_hrtimer() split the thing into 3
// distinct sections:
//
// - queued:	the timer is queued
// - callback:	the timer is being ran
// - post:	the timer is inactive or (re)queued
//
// On the read side we ensure we observe timer->is_queued and cpu_base->running
// from the same section, if anything changed while we looked at it, we retry.
// This includes timer->base changing because sequence numbers alone are
// insufficient for that.
//
// The sequence numbers are required because otherwise we could still observe
// a false negative if the read side got smeared over multiple consecutive
// __run_hrtimer() invocations.
//
#[no_mangle]
pub unsafe extern "C" fn __run_hrtimer(cpu_base: *mut hrtimer_cpu_base, base: *mut hrtimer_clock_base, timer: *mut hrtimer, now: ktime_t, lock: unsigned long flags)
    __must_hold(&cpu_base.) {
    enum hrtimer_restart (*fn);
    let mut expires_in_hardirq = 0;
    let mut restart = 0;
    lockdep_assert_held(&cpu_base.lock);
    debug_hrtimer_deactivate(timer);
    base.running = timer;
//
// Separate the ->running assignment from the ->is_queued assignment.
//
// As with a regular write barrier, this ensures the read side in
// hrtimer_active() cannot observe base->running == NULL &&
// timer->is_queued == INACTIVE.
//
    raw_write_seqcount_barrier(&base.seq);
    __remove_hrtimer(timer, base, HRTIMER_STATE_INACTIVE, false);
    fn = ACCESS_PRIVATE(timer, function);
//
// Clear the 'is relative' flag for the TIME_LOW_RES case. If the
// timer is restarted with a period then it becomes an absolute
// timer. If its not restarted it does not matter.
//
    if (IS_ENABLED!(CONFIG_TIME_LOW_RES)) {
    timer.is_rel = false;
    }
//
// The timer is marked as running in the CPU base, so it is
// protected against migration to a different CPU even if the lock
// is dropped.
//
    raw_spin_unlock_irqrestore(&cpu_base.lock, flags);
    trace_hrtimer_expire_entry(timer, now);
    expires_in_hardirq = lockdep_hrtimer_enter(timer);
    restart = fn(timer);
    lockdep_hrtimer_exit(expires_in_hardirq);
    trace_hrtimer_expire_exit(timer);
    raw_spin_lock_irq(&cpu_base.lock);
//
// Note: We clear the running state after enqueue_hrtimer and
// we do not reprogram the event hardware. Happens either in
// hrtimer_start_range_ns() or in hrtimer_interrupt()
//
// Note: Because we dropped the cpu_base->lock above,
// hrtimer_start_range_ns() can have popped in and enqueued the timer
// for us already.
//
    if (restart == HRTIMER_RESTART && !timer.is_queued) {
    enqueue_hrtimer(timer, base, HRTIMER_MODE_ABS, false);
    }
//
// Separate the ->running assignment from the ->is_queued assignment.
//
// As with a regular write barrier, this ensures the read side in
// hrtimer_active() cannot observe base->running.timer == NULL &&
// timer->is_queued == INACTIVE.
//
    raw_write_seqcount_barrier(&base.seq);
    WARN_ON_ONCE!(base.running != timer);
    base.running = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn __hrtimer_run_queues(cpu_base: *mut hrtimer_cpu_base, now: ktime_t, flags: c_ulong, active_mask: c_uint) {
pub static mut active: c_uint = 0;
pub static mut base: *mut c_void = core::ptr::null_mut();
    for_each_active_base(base, cpu_base, active) {
pub static mut basenow: ktime_t = 0;
pub static mut timer: *mut c_void = core::ptr::null_mut();
    while ((timer = clock_base_next_timer(base))) {
//
// The immediate goal for using the softexpires is
// minimizing wakeups, not running timers at the
// earliest interrupt after their soft expiration.
// This allows us to avoid using a Priority Search
// Tree, which can answer a stabbing query for
// overlapping intervals and instead use the simple
// BST we already have.
// We don't add extra wakeups by delaying timers that
// are right-of a not yet expired timer, because that
// timer will have to trigger a wakeup anyway.
//
    if (basenow < hrtimer_get_softexpires(timer)) {
    break;
    }
    __run_hrtimer(cpu_base, base, timer, basenow, flags);
    if (active_mask == HRTIMER_ACTIVE_SOFT) {
    hrtimer_sync_wait_running(cpu_base, flags);
    }
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn hrtimer_run_softirq() -> __latent_entropy void {
    let mut cpu_base = this_cpu_ptr(&hrtimer_bases);
    let mut flags = 0;
    let mut now;
    hrtimer_cpu_base_lock_expiry(cpu_base);
    raw_spin_lock_irqsave(&cpu_base.lock, flags);
    now = hrtimer_update_base(cpu_base);
    __hrtimer_run_queues(cpu_base, now, flags, HRTIMER_ACTIVE_SOFT);
    cpu_base.softirq_activated = false;
    hrtimer_update_softirq_timer(cpu_base, true);
    raw_spin_unlock_irqrestore(&cpu_base.lock, flags);
    hrtimer_cpu_base_unlock_expiry(cpu_base);
    }

//
// Very similar to hrtimer_force_reprogram(), except it deals with
// deferred_rearm and hang_detected.
//
#[no_mangle]
unsafe extern "C" fn hrtimer_rearm(cpu_base: *mut hrtimer_cpu_base, expires_next: ktime_t, deferred: bool) {
    cpu_base.expires_next = expires_next;
    cpu_base.deferred_rearm = false;
    if (unlikely(cpu_base.hang_detected)) {
//
// Give the system a chance to do something else than looping
// on hrtimer interrupts.
//
    expires_next = ktime_add_ns(ktime_get(),
    min(100 * NSEC_PER_MSEC, cpu_base.max_hang_time));
    }
    hrtimer_rearm_event(expires_next, deferred);
    }

#[no_mangle]
pub unsafe extern "C" fn __hrtimer_rearm_deferred() {
    let mut cpu_base = this_cpu_ptr(&hrtimer_bases);
    let mut expires_next;
    if (!cpu_base.deferred_rearm) {
    return;
    }
    guard(raw_spinlock)(&cpu_base.lock);
    if (cpu_base.deferred_needs_update) {
    hrtimer_update_base(cpu_base);
    expires_next = hrtimer_update_next_event(cpu_base);
    } else {
// No timer added/removed. Use the cached value
    expires_next = cpu_base.deferred_expires_next;
    }
    hrtimer_rearm(cpu_base, expires_next, true);
    }
    static __always_inline void
    hrtimer_interrupt_rearm(hrtimer_cpu_base *cpu_base, ktime_t expires_next)
    {
// hrtimer_interrupt() just re-evaluated the first expiring timer
    cpu_base.deferred_needs_update = false;
// Cache the expiry time
    cpu_base.deferred_expires_next = expires_next;
    set_thread_flag(TIF_HRTIMER_REARM);
    }

    static __always_inline void
    hrtimer_interrupt_rearm(hrtimer_cpu_base *cpu_base, ktime_t expires_next)
    {
    hrtimer_rearm(cpu_base, expires_next, false);
    }

//
// High resolution timer interrupt
// Called with interrupts disabled
//
#[no_mangle]
pub unsafe extern "C" fn hrtimer_interrupt(dev: *mut clock_event_device) {
    let mut cpu_base = this_cpu_ptr(&hrtimer_bases);
    ktime_t expires_next, now, entry_time, delta;
    let mut flags = 0;
pub static mut retries: c_int = 0;
    BUG_ON!(!cpu_base.hres_active);
    cpu_base.nr_events += 1;
    dev.next_event = KTIME_MAX;
    dev.next_event_forced = 0;
    raw_spin_lock_irqsave(&cpu_base.lock, flags);
    entry_time = now = hrtimer_update_base(cpu_base);
// label;
    cpu_base.deferred_rearm = true;
//
// Set expires_next to KTIME_MAX, which prevents that remote CPUs queue
// timers while __hrtimer_run_queues() is expiring the clock bases.
// Timers which are re/enqueued on the local CPU are not affected by
// this.
//
    cpu_base.expires_next = KTIME_MAX;
    if (!ktime_before(now, cpu_base.softirq_expires_next)) {
    cpu_base.softirq_expires_next = KTIME_MAX;
    cpu_base.softirq_activated = true;
    raise_timer_softirq(HRTIMER_SOFTIRQ);
    }
    __hrtimer_run_queues(cpu_base, now, flags, HRTIMER_ACTIVE_HARD);
//
// The next timer was already expired due to:
// - tracing
// - long lasting callbacks
// - being scheduled away when running in a VM
//
// We need to prevent that we loop forever in the hrtiner interrupt
// routine. We give it 3 attempts to avoid overreacting on some
// spurious event.
//
    now = hrtimer_update_base(cpu_base);
    expires_next = hrtimer_update_next_event(cpu_base);
    cpu_base.hang_detected = false;
    if (expires_next < now) {
    if (++retries < 3) {
    cpu_base.nr_retries += 1;
// goto;
    }
    delta = ktime_sub(now, entry_time);
    cpu_base.max_hang_time = max_t(unsigned int, cpu_base.max_hang_time, delta);
    cpu_base.nr_hangs += 1;
    cpu_base.hang_detected = true;
    }
    hrtimer_interrupt_rearm(cpu_base, expires_next);
    raw_spin_unlock_irqrestore(&cpu_base.lock, flags);
    }

//
// Called from run_local_timers in hardirq context every jiffy
//
#[no_mangle]
pub unsafe extern "C" fn hrtimer_run_queues() {
    let mut cpu_base = this_cpu_ptr(&hrtimer_bases);
    let mut flags = 0;
    let mut now;
    if (hrtimer_hres_active(cpu_base)) {
    return;
    }
//
// This _is_ ugly: We have to check periodically, whether we
// can switch to highres and / or nohz mode. The clocksource
// switch happens with xtime_lock held. Notification from
// there only sets the check bit in the tick_oneshot code,
// otherwise we might deadlock vs. xtime_lock.
//
    if (tick_check_oneshot_change(!hrtimer_is_hres_enabled())) {
    hrtimer_switch_to_hres();
    return;
    }
    raw_spin_lock_irqsave(&cpu_base.lock, flags);
    now = hrtimer_update_base(cpu_base);
    if (!ktime_before(now, cpu_base.softirq_expires_next)) {
    cpu_base.softirq_expires_next = KTIME_MAX;
    cpu_base.softirq_activated = true;
    raise_timer_softirq(HRTIMER_SOFTIRQ);
    }
    __hrtimer_run_queues(cpu_base, now, flags, HRTIMER_ACTIVE_HARD);
    raw_spin_unlock_irqrestore(&cpu_base.lock, flags);
    }
//
// Sleep related functions:
//
#[no_mangle]
unsafe extern "C" fn hrtimer_wakeup(timer: *mut hrtimer) -> enum hrtimer_restart {
    let mut t = container_of!(timer, hrtimer_sleeper, timer);
    let mut task = t.task;
    t.task = core::ptr::null_mut();
    if (task) {
    wake_up_process(task);
    }
    return HRTIMER_NORESTART;
    }
//
// hrtimer_sleeper_start_expires - Start a hrtimer sleeper timer
// @sl:		sleeper to be started
// @mode:	timer mode abs/rel
//
// Wrapper around hrtimer_start_expires() for hrtimer_sleeper based timers
// to allow PREEMPT_RT to tweak the delivery mode (soft/hardirq context)
//
#[no_mangle]
pub unsafe extern "C" fn hrtimer_sleeper_start_expires(sl: *mut hrtimer_sleeper, mode: hrtimer_mode) {
//
// Make the enqueue delivery mode check work on RT. If the sleeper
// was initialized for hard interrupt delivery, force the mode bit.
// This is a special case for hrtimer_sleepers because
// __hrtimer_setup_sleeper() determines the delivery mode on RT so the
// fiddling with this decision is avoided at the call sites.
//
    if (IS_ENABLED!(CONFIG_PREEMPT_RT) && sl.timer.is_hard) {
    mode |= HRTIMER_MODE_HARD;
    }
// If already expired, clear the task pointer and set current state to running
    if (!hrtimer_start_expires_user(&sl.timer, mode)) {
    sl.task = core::ptr::null_mut();
    __set_current_state(TASK_RUNNING);
    }
    }
    EXPORT_SYMBOL_GPL(hrtimer_sleeper_start_expires);
#[no_mangle]
pub unsafe extern "C" fn __hrtimer_setup_sleeper(sl: *mut hrtimer_sleeper, clock_id: clockid_t, mode: hrtimer_mode) {
//
// On PREEMPT_RT enabled kernels hrtimers which are not explicitly
// marked for hard interrupt expiry mode are moved into soft
// interrupt context either for latency reasons or because the
// hrtimer callback takes regular spinlocks or invokes other
// functions which are not suitable for hard interrupt context on
// PREEMPT_RT.
//
// The hrtimer_sleeper callback is RT compatible in hard interrupt
// context, but there is a latency concern: Untrusted userspace can
// spawn many threads which arm timers for the same expiry time on
// the same CPU. That causes a latency spike due to the wakeup of
// a gazillion threads.
//
// OTOH, privileged real-time user space applications rely on the
// low latency of hard interrupt wakeups. If the current task is in
// a real-time scheduling class, mark the mode for hard interrupt
// expiry.
//
    if (IS_ENABLED!(CONFIG_PREEMPT_RT)) {
    if (rt_or_dl_task_policy(current) && !(mode & HRTIMER_MODE_SOFT)) {
    mode |= HRTIMER_MODE_HARD;
    }
    }
    __hrtimer_setup(&sl.timer, hrtimer_wakeup, clock_id, mode);
    sl.task = current;
    }
//
// hrtimer_setup_sleeper_on_stack - initialize a sleeper in stack memory
// @sl:		sleeper to be initialized
// @clock_id:	the clock to be used
// @mode:	timer mode abs/rel
//
#[no_mangle]
pub unsafe extern "C" fn hrtimer_setup_sleeper_on_stack(sl: *mut hrtimer_sleeper, clock_id: clockid_t, mode: hrtimer_mode) {
    debug_setup_on_stack(&sl.timer, clock_id, mode);
    __hrtimer_setup_sleeper(sl, clock_id, mode);
    }
    EXPORT_SYMBOL_GPL(hrtimer_setup_sleeper_on_stack);
#[no_mangle]
pub unsafe extern "C" fn nanosleep_copyout(restart: *mut restart_block, ts: *mut timespec64) -> c_int {
    match (restart.nanosleep.type) {

    TT_COMPAT => {
    if (put_old_timespec32(ts, restart.nanosleep.compat_rmtp)) {
    return -EFAULT;
    }
    // break;

    }
    TT_NATIVE => {
    if (put_timespec64(ts, restart.nanosleep.rmtp)) {
    return -EFAULT;
    }
    // break;
    }
    _ => {
    BUG();
    }
    }
    return -ERESTART_RESTARTBLOCK;
    }
#[no_mangle]
unsafe extern "C" fn do_nanosleep(t: *mut hrtimer_sleeper, mode: hrtimer_mode) -> int __sched {
pub static mut restart: *mut c_void = core::ptr::null_mut();
    do {
    set_current_state(TASK_INTERRUPTIBLE|TASK_FREEZABLE);
    hrtimer_sleeper_start_expires(t, mode);
    if (likely(t.task)) {
    schedule();
    }
    hrtimer_cancel(&t.timer);
    mode = HRTIMER_MODE_ABS;
    } while (t.task && !signal_pending(current));
    __set_current_state(TASK_RUNNING);
    if (!t.task) {
    return 0;
    }
    restart = &current.restart_block;
    if (restart.nanosleep.type != TT_NONE) {
pub static mut rem: ktime_t = 0;
pub static mut rmt: usize = 0;
    if (rem <= 0) {
    return 0;
    }
    rmt = ktime_to_timespec64(rem);
    return nanosleep_copyout(restart, &rmt);
    }
    return -ERESTART_RESTARTBLOCK;
    }
#[no_mangle]
unsafe extern "C" fn hrtimer_nanosleep_restart(restart: *mut restart_block) -> long __sched {
pub static mut t: usize = 0;
    let mut ret = 0;
    hrtimer_setup_sleeper_on_stack(&t, restart.nanosleep.clockid, HRTIMER_MODE_ABS);
    hrtimer_set_expires(&t.timer, restart.nanosleep.expires);
    ret = do_nanosleep(&t, HRTIMER_MODE_ABS);
    destroy_hrtimer_on_stack(&t.timer);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn hrtimer_nanosleep(rqtp: ktime_t, mode: hrtimer_mode, clockid: clockid_t) -> c_long {
pub static mut restart: *mut c_void = core::ptr::null_mut();
pub static mut t: usize = 0;
    let mut ret = 0;
    hrtimer_setup_sleeper_on_stack(&t, clockid, mode);
    hrtimer_set_expires_range_ns(&t.timer, rqtp, current.timer_slack_ns);
    ret = do_nanosleep(&t, mode);
    if (ret != -ERESTART_RESTARTBLOCK) {
// goto;
    }
// Absolute timers do not update the rmtp value and restart:
    if (mode == HRTIMER_MODE_ABS) {
    ret = -ERESTARTNOHAND;
// goto;
    }
    restart = &current.restart_block;
    restart.nanosleep.clockid = t.timer.base.clockid;
    restart.nanosleep.expires = hrtimer_get_expires(&t.timer);
    set_restart_fn(restart, hrtimer_nanosleep_restart);
// label;
    destroy_hrtimer_on_stack(&t.timer);
    return ret;
    }

#[no_mangle]
pub unsafe extern "C" fn sys_nanosleep(rqtp: usize, rmtp: usize) -> c_long {
pub static mut tu: usize = 0;
    if (get_timespec64(&tu, rqtp)) {
    return -EFAULT;
    }
    if (!timespec64_valid(&tu)) {
    return -EINVAL;
    }
    current.restart_block.fn = do_no_restart_syscall;
    current.restart_block.nanosleep.type = rmtp ? TT_NATIVE : TT_NONE;
    current.restart_block.nanosleep.rmtp = rmtp;
    return hrtimer_nanosleep(timespec64_to_ktime(tu), HRTIMER_MODE_REL, CLOCK_MONOTONIC);
    }

#[no_mangle]
pub unsafe extern "C" fn sys_nanosleep_time32(rqtp: usize, rmtp: usize) -> c_long {
pub static mut tu: usize = 0;
    if (get_old_timespec32(&tu, rqtp)) {
    return -EFAULT;
    }
    if (!timespec64_valid(&tu)) {
    return -EINVAL;
    }
    current.restart_block.fn = do_no_restart_syscall;
    current.restart_block.nanosleep.type = rmtp ? TT_COMPAT : TT_NONE;
    current.restart_block.nanosleep.compat_rmtp = rmtp;
    return hrtimer_nanosleep(timespec64_to_ktime(tu), HRTIMER_MODE_REL, CLOCK_MONOTONIC);
    }

//
// Functions related to boot-time initialization:
//
#[no_mangle]
pub unsafe extern "C" fn hrtimers_prepare_cpu(cpu: c_uint) -> c_int {
    let mut cpu_base = &per_cpu(hrtimer_bases, cpu);
    while (i < HRTIMER_MAX_CLOCK_BASES) {
    let mut clock_b = &cpu_base.clock_base[i];
    clock_b.cpu_base = cpu_base;
    seqcount_raw_spinlock_init(&clock_b.seq, &cpu_base.lock);
    timerqueue_linked_init_head(&clock_b.active);
    }
    cpu_base.cpu = cpu;
    hrtimer_cpu_base_init_expiry_lock(cpu_base);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn hrtimers_cpu_starting(cpu: c_uint) -> c_int {
    let mut cpu_base = this_cpu_ptr(&hrtimer_bases);
// Clear out any left over state from a CPU down operation
    cpu_base.active_bases = 0;
    cpu_base.hres_active = false;
    cpu_base.hang_detected = false;
    cpu_base.next_timer = core::ptr::null_mut();
    cpu_base.softirq_next_timer = core::ptr::null_mut();
    cpu_base.expires_next = KTIME_MAX;
    cpu_base.softirq_expires_next = KTIME_MAX;
    cpu_base.softirq_activated = false;
    cpu_base.online = true;
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn migrate_hrtimer_list(old_base: *mut hrtimer_clock_base, new_base: *mut hrtimer_clock_base) {
pub static mut node: *mut c_void = core::ptr::null_mut();
pub static mut timer: *mut c_void = core::ptr::null_mut();
    while ((node = timerqueue_linked_first(&old_base.active))) {
    timer = hrtimer_from_timerqueue_node(node);
    BUG_ON!(hrtimer_callback_running(timer));
    debug_hrtimer_deactivate(timer);
//
// Mark it as ENQUEUED not INACTIVE otherwise the
// timer could be seen as !active and just vanish away
// under us on another CPU
//
    __remove_hrtimer(timer, old_base, HRTIMER_STATE_ENQUEUED, false);
    timer.base = new_base;
//
// Enqueue the timers on the new cpu. This does not
// reprogram the event device in case the timer
// expires before the earliest on this CPU, but we run
// hrtimer_interrupt after we migrated everything to
// sort out already expired timers and reprogram the
// event device.
//
    enqueue_hrtimer(timer, new_base, HRTIMER_MODE_ABS, true);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn hrtimers_cpu_dying(dying_cpu: c_uint) -> c_int {
pub static mut ncpu: c_int = 0;
    let mut old_base = core::ptr::null_mut();
    let mut new_base = core::ptr::null_mut();
    old_base = this_cpu_ptr(&hrtimer_bases);
    new_base = &per_cpu(hrtimer_bases, ncpu);
//
// The caller is globally serialized and nobody else
// takes two locks at once, deadlock is not possible.
//
    raw_spin_lock(&old_base.lock);
    raw_spin_lock_nested(&new_base.lock, SINGLE_DEPTH_NESTING);
    for (int i = 0; i < HRTIMER_MAX_CLOCK_BASES; i++) {
    migrate_hrtimer_list(&old_base.clock_base[i], &new_base.clock_base[i]);
    }
// Tell the other CPU to retrigger the next event
    smp_call_function_single(ncpu, retrigger_next_event, core::ptr::null_mut(), 0);
    raw_spin_unlock(&new_base.lock);
    old_base.online = false;
    raw_spin_unlock(&old_base.lock);
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn hrtimers_init()  {
    hrtimers_prepare_cpu(smp_processor_id());
    hrtimers_cpu_starting(smp_processor_id());
    open_softirq(HRTIMER_SOFTIRQ, hrtimer_run_softirq);
    }
}
