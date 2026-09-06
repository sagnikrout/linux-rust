//! Automatically rewritten from C to Rust
//! Source: kernel/time/timer.c
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
// Kernel internal timers
//
// Copyright (C) 1991, 1992  Linus Torvalds
//
// 1997-01-28  Modified by Finn Arne Gangstad to make timers scale better.
//
// 1997-09-10  Updated NTP code according to technical memorandum Jan '96
// "A Kernel Model for Precision Timekeeping" by Dave Mills
// 1998-12-24  Fixed a xtime SMP race (we need the xtime_lock rw spinlock to
// serialize accesses to xtime/lost_ticks).
// Copyright (C) 1998  Andrea Arcangeli
// 1999-03-10  Improved NTP compatibility by Ulrich Windl
// 2002-05-31	Move sys_sysinfo here and make its locking sane, Robert Love
// 2000-10-05  Implemented scalable SMP per-CPU timer handling.
// Copyright (C) 2000, 2001, 2002  Ingo Molnar
// Designed by David S. Miller, Alexey Kuznetsov and Ingo Molnar
//

// Macro flag: #define CREATE_TRACE_POINTS

pub static mut __cacheline_aligned_in_smp: __visible u64 jiffies_64 = 0;
    EXPORT_SYMBOL(jiffies_64);
//
// The timer wheel has LVL_DEPTH array levels. Each level provides an array of
// LVL_SIZE buckets. Each level is driven by its own clock and therefore each
// level has a different granularity.
//
// The level granularity is:		LVL_CLK_DIV ^ level
// The level clock frequency is:	HZ / (LVL_CLK_DIV ^ level)
//
// The array level of a newly armed timer depends on the relative expiry
// time. The farther the expiry time is away the higher the array level and
// therefore the granularity becomes.
//
// Contrary to the original timer wheel implementation, which aims for 'exact'
// expiry of the timers, this implementation removes the need for recascading
// the timers into the lower array levels. The previous 'classic' timer wheel
// implementation of the kernel already violated the 'exact' expiry by adding
// slack to the expiry time to provide batched expiration. The granularity
// levels provide implicit batching.
//
// This is an optimization of the original timer wheel implementation for the
// majority of the timer wheel use cases: timeouts. The vast majority of
// timeout timers (networking, disk I/O ...) are canceled before expiry. If
// the timeout expires it indicates that normal operation is disturbed, so it
// does not matter much whether the timeout comes with a slight delay.
//
// The only exception to this are networking timers with a small expiry
// time. They rely on the granularity. Those fit into the first wheel level,
// which has HZ granularity.
//
// We don't have cascading anymore. timers with a expiry time above the
// capacity of the last wheel level are force expired at the maximum timeout
// value of the last wheel level. From data sampling we know that the maximum
// value observed is 5 days (network connection tracking), so this should not
// be an issue.
//
// The currently chosen array constants values are a good compromise between
// array size and granularity.
//
// This results in the following granularity and range levels:
//
// HZ 1000 steps
// Level Offset  Granularity            Range
// 0      0         1 ms                0 ms -         63 ms
// 1     64         8 ms               64 ms -        511 ms
// 2    128        64 ms              512 ms -       4095 ms (512ms - ~4s)
// 3    192       512 ms             4096 ms -      32767 ms (~4s - ~32s)
// 4    256      4096 ms (~4s)      32768 ms -     262143 ms (~32s - ~4m)
// 5    320     32768 ms (~32s)    262144 ms -    2097151 ms (~4m - ~34m)
// 6    384    262144 ms (~4m)    2097152 ms -   16777215 ms (~34m - ~4h)
// 7    448   2097152 ms (~34m)  16777216 ms -  134217727 ms (~4h - ~1d)
// 8    512  16777216 ms (~4h)  134217728 ms - 1073741822 ms (~1d - ~12d)
//
// HZ  300
// Level Offset  Granularity            Range
// 0	   0         3 ms                0 ms -        210 ms
// 1	  64        26 ms              213 ms -       1703 ms (213ms - ~1s)
// 2	 128       213 ms             1706 ms -      13650 ms (~1s - ~13s)
// 3	 192      1706 ms (~1s)      13653 ms -     109223 ms (~13s - ~1m)
// 4	 256     13653 ms (~13s)    109226 ms -     873810 ms (~1m - ~14m)
// 5	 320    109226 ms (~1m)     873813 ms -    6990503 ms (~14m - ~1h)
// 6	 384    873813 ms (~14m)   6990506 ms -   55924050 ms (~1h - ~15h)
// 7	 448   6990506 ms (~1h)   55924053 ms -  447392423 ms (~15h - ~5d)
// 8    512  55924053 ms (~15h) 447392426 ms - 3579139406 ms (~5d - ~41d)
//
// HZ  250
// Level Offset  Granularity            Range
// 0	   0         4 ms                0 ms -        255 ms
// 1	  64        32 ms              256 ms -       2047 ms (256ms - ~2s)
// 2	 128       256 ms             2048 ms -      16383 ms (~2s - ~16s)
// 3	 192      2048 ms (~2s)      16384 ms -     131071 ms (~16s - ~2m)
// 4	 256     16384 ms (~16s)    131072 ms -    1048575 ms (~2m - ~17m)
// 5	 320    131072 ms (~2m)    1048576 ms -    8388607 ms (~17m - ~2h)
// 6	 384   1048576 ms (~17m)   8388608 ms -   67108863 ms (~2h - ~18h)
// 7	 448   8388608 ms (~2h)   67108864 ms -  536870911 ms (~18h - ~6d)
// 8    512  67108864 ms (~18h) 536870912 ms - 4294967288 ms (~6d - ~49d)
//
// HZ  100
// Level Offset  Granularity            Range
// 0	   0         10 ms               0 ms -        630 ms
// 1	  64         80 ms             640 ms -       5110 ms (640ms - ~5s)
// 2	 128        640 ms            5120 ms -      40950 ms (~5s - ~40s)
// 3	 192       5120 ms (~5s)     40960 ms -     327670 ms (~40s - ~5m)
// 4	 256      40960 ms (~40s)   327680 ms -    2621430 ms (~5m - ~43m)
// 5	 320     327680 ms (~5m)   2621440 ms -   20971510 ms (~43m - ~5h)
// 6	 384    2621440 ms (~43m) 20971520 ms -  167772150 ms (~5h - ~1d)
// 7	 448   20971520 ms (~5h) 167772160 ms - 1342177270 ms (~1d - ~15d)
//
// Clock divisor for the next level
pub const LVL_CLK_SHIFT: c_int = 3;

//
// The time start value for each level to select the bucket at enqueue
// time. We start from the last possible delta of the previous level
// so that we can later add an extra LVL_GRAN(n) to n (see calc_index()).
//

// Size of each clock level
pub const LVL_BITS: c_int = 6;

// Level depth

// The cutoff (max. capacity of the wheel)

//
// The resulting wheel size. If NOHZ is configured we allocate two
// wheels so we have a separate storage for the deferrable timers.
//

//
// If multiple bases need to be locked, use the base ordering for lock
// nesting, i.e. lowest number first.
//

//
// struct timer_base - Per CPU timer base (number of base depends on config)
// @lock:		Lock protecting the timer_base
// @running_timer:	When expiring timers, the lock is dropped. To make
// sure not to race against deleting/modifying a
// currently running timer, the pointer is set to the
// timer, which expires at the moment. If no timer is
// running, the pointer is NULL.
// @expiry_lock:	PREEMPT_RT only: Lock is taken in softirq around
// timer expiry callback execution and when trying to
// delete a running timer and it wasn't successful in
// the first glance. It prevents priority inversion
// when callback was preempted on a remote CPU and a
// caller tries to delete the running timer. It also
// prevents a life lock, when the task which tries to
// delete a timer preempted the softirq thread which
// is running the timer callback function.
// @timer_waiters:	PREEMPT_RT only: Tells, if there is a waiter
// waiting for the end of the timer callback function
// execution.
// @clk:		clock of the timer base; is updated before enqueue
// of a timer; during expiry, it is 1 offset ahead of
// jiffies to avoid endless requeuing to current
// jiffies
// @next_expiry:	expiry value of the first timer; it is updated when
// finding the next timer and during enqueue; the
// value is not valid, when next_expiry_recalc is set
// @cpu:		Number of CPU the timer base belongs to
// @next_expiry_recalc: States, whether a recalculation of next_expiry is
// required. Value is set true, when a timer was
// deleted.
// @is_idle:		Is set, when timer_base is idle. It is triggered by NOHZ
// code. This state is only used in standard
// base. Deferrable timers, which are enqueued remotely
// never wake up an idle CPU. So no matter of supporting it
// for this base.
// @timers_pending:	Is set, when a timer is pending in the base. It is only
// reliable when next_expiry_recalc is not set.
// @pending_map:	bitmap of the timer wheel; each bit reflects a
// bucket of the wheel. When a bit is set, at least a
// single timer is enqueued in the related bucket.
// @vectors:		Array of lists; Each array member reflects a bucket
// of the timer wheel. The list contains all timers
// which are enqueued into a specific bucket.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct timer_base {
    pub lock: raw_spinlock_t,
    pub running_timer: *mut timer_list,

    pub expiry_lock: spinlock_t,
    pub timer_waiters: core::sync::atomic::AtomicI32,

    pub clk: c_ulong,
    pub next_expiry: c_ulong,
    pub cpu: c_uint,
    pub next_expiry_recalc: bool,
    pub is_idle: bool,
    pub timers_pending: bool,
    pub WHEEL_SIZE): DECLARE_BITMAP(pending_map,,
    pub vectors: [hlist_head; WHEEL_SIZE],
}
    pub timer_bases[NR_BASES]): DEFINE_PER_CPU(timer_base,,

    pub DEFINE_STATIC_KEY_FALSE(timers_nohz_active): static,
    pub DEFINE_MUTEX(timer_keys_mutex): static,
    pub work): *mut static void timer_update_keys(work_struct,
    pub timer_update_keys): DECLARE_WORK(timer_update_work,,

    pub 1: unsigned int sysctl_timer_migration =,
#[no_mangle]
unsafe extern "C" fn timers_update_migration() {
    if (sysctl_timer_migration && tick_nohz_is_active()) {
    else
    }

#[no_mangle]
pub unsafe extern "C" fn timer_migration_handler(table: *mut ctl_table, write: c_int, buffer: *mut c_void, lenp: *mut size_t, ppos: *mut loff_t) -> c_int {
    }
    pub ret: c_int,
    pub ppos): ret = proc_dointvec_minmax(table, write, buffer, lenp,,
    if (!ret && write) {
    pub ret: return,
    }
pub static mut ctl_table: usize = 0;
    }
#[no_mangle]
pub unsafe extern "C" fn timer_debug_hint(addr: *mut c_void) -> *mut c_void {
    let mut timer = addr;
    let mut i = 0;
    while (i < ARRAY_SIZE!(timer_hints)) {
    if (timer_hints[i].function == timer.function) {
    void (**fn)(void) = addr + timer_hints[i].offset;
pub static mut fn: *mut c_void = core::ptr::null_mut();
    }
    }
    return timer.function;
    }
#[no_mangle]
unsafe extern "C" fn timer_is_static_object(addr: *mut c_void) -> bool {
    let mut timer = addr;
    return (timer.entry.pprev == core::ptr::null_mut() &&
    timer.entry.next == TIMER_ENTRY_STATIC);
    }
//
// timer_fixup_init is called when:
// - an active object is initialized
//
#[no_mangle]
unsafe extern "C" fn timer_fixup_init(addr: *mut c_void, state: debug_obj_state) -> bool {
    let mut timer = addr;
    match (state) {
    ODEBUG_STATE_ACTIVE => {
    timer_delete_sync(timer);
    debug_object_init(timer, &timer_debug_descr);
    return true;
    }
    _ => {
    return false;
    }
    }
    }
// Stub timer callback for improperly used timers.
#[no_mangle]
unsafe extern "C" fn stub_timer(unused: *mut timer_list) {
    WARN_ON!(1);
    }
//
// timer_fixup_activate is called when:
// - an active object is activated
// - an unknown non-static object is activated
//
#[no_mangle]
unsafe extern "C" fn timer_fixup_activate(addr: *mut c_void, state: debug_obj_state) -> bool {
    let mut timer = addr;
    match (state) {
    ODEBUG_STATE_NOTAVAILABLE => {
    timer_setup(timer, stub_timer, 0);
    return true;
    }
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
// timer_fixup_free is called when:
// - an active object is freed
//
#[no_mangle]
unsafe extern "C" fn timer_fixup_free(addr: *mut c_void, state: debug_obj_state) -> bool {
    let mut timer = addr;
    match (state) {
    ODEBUG_STATE_ACTIVE => {
    timer_delete_sync(timer);
    debug_object_free(timer, &timer_debug_descr);
    return true;
    }
    _ => {
    return false;
    }
    }
    }
//
// timer_fixup_assert_init is called when:
// - an untracked/uninit-ed object is found
//
#[no_mangle]
unsafe extern "C" fn timer_fixup_assert_init(addr: *mut c_void, state: debug_obj_state) -> bool {
    let mut timer = addr;
    match (state) {
    ODEBUG_STATE_NOTAVAILABLE => {
    timer_setup(timer, stub_timer, 0);
    return true;
    }
    _ => {
    return false;
    }
    }
    }
pub static mut debug_obj_descr: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn debug_timer_init(timer: *mut timer_list) {
    debug_object_init(timer, &timer_debug_descr);
    }
#[no_mangle]
pub unsafe extern "C" fn debug_timer_activate(timer: *mut timer_list) {
    debug_object_activate(timer, &timer_debug_descr);
    }
#[no_mangle]
pub unsafe extern "C" fn debug_timer_deactivate(timer: *mut timer_list) {
    debug_object_deactivate(timer, &timer_debug_descr);
    }
#[no_mangle]
pub unsafe extern "C" fn debug_timer_assert_init(timer: *mut timer_list) {
    debug_object_assert_init(timer, &timer_debug_descr);
    }
// forward_decl: do_init_timer;
#[no_mangle]
pub unsafe extern "C" fn timer_init_key_on_stack(timer: *mut timer_list, flags: c_uint, name: *mut c_char, key: *mut lock_class_key) {
    debug_object_init_on_stack(timer, &timer_debug_descr);
    do_init_timer(timer, func, flags, name, key);
    }
    EXPORT_SYMBOL_GPL(timer_init_key_on_stack);
#[no_mangle]
pub unsafe extern "C" fn timer_destroy_on_stack(timer: *mut timer_list) {
    debug_object_free(timer, &timer_debug_descr);
    }
    EXPORT_SYMBOL_GPL(timer_destroy_on_stack);

#[no_mangle]
#[no_mangle]
// duplicate fn: debug_timer_init
pub unsafe extern "C" fn debug_timer_init_dup(timer: *mut timer_list) { }
#[no_mangle]
#[no_mangle]
// duplicate fn: debug_timer_activate
pub unsafe extern "C" fn debug_timer_activate_dup(timer: *mut timer_list) { }
#[no_mangle]
#[no_mangle]
// duplicate fn: debug_timer_deactivate
pub unsafe extern "C" fn debug_timer_deactivate_dup(timer: *mut timer_list) { }
#[no_mangle]
#[no_mangle]
// duplicate fn: debug_timer_assert_init
pub unsafe extern "C" fn debug_timer_assert_init_dup(timer: *mut timer_list) { }

#[no_mangle]
pub unsafe extern "C" fn debug_init(timer: *mut timer_list) {
    debug_timer_init(timer);
    trace_timer_init(timer);
    }
#[no_mangle]
pub unsafe extern "C" fn debug_deactivate(timer: *mut timer_list) {
    debug_timer_deactivate(timer);
    trace_timer_cancel(timer);
    }
#[no_mangle]
pub unsafe extern "C" fn debug_assert_init(timer: *mut timer_list) {
    debug_timer_assert_init(timer);
    }
#[no_mangle]
pub unsafe extern "C" fn do_init_timer(timer: *mut timer_list, flags: c_uint, name: *mut c_char, key: *mut lock_class_key) {
    timer.entry.pprev = core::ptr::null_mut();
    timer.function = func;
    if (WARN_ON_ONCE!(flags & ~TIMER_INIT_FLAGS)) {
    flags &= TIMER_INIT_FLAGS;
    }
    timer.flags = flags | raw_smp_processor_id();
    lockdep_init_map(&timer.lockdep_map, name, key, 0);
    }
//
// timer_init_key - initialize a timer
// @timer: the timer to be initialized
// @func: timer callback function
// @flags: timer flags
// @name: name of the timer
// @key: lockdep class key of the fake lock used for tracking timer
// sync lock dependencies
//
// timer_init_key() must be done to a timer prior to calling *any* of the
// other timer functions.
//
#[no_mangle]
pub unsafe extern "C" fn timer_init_key(timer: *mut timer_list, flags: c_uint, name: *mut c_char, key: *mut lock_class_key) {
    debug_init(timer);
    do_init_timer(timer, func, flags, name, key);
    }
    EXPORT_SYMBOL(timer_init_key);
#[no_mangle]
pub unsafe extern "C" fn detach_timer(timer: *mut timer_list, clear_pending: bool) {
    let mut entry = &timer.entry;
    debug_deactivate(timer);
    __hlist_del(entry);
    if (clear_pending) {
    entry.pprev = core::ptr::null_mut();
    }
    entry.next = LIST_POISON2;
    }
#[no_mangle]
pub unsafe extern "C" fn detach_if_pending(timer: *mut timer_list, base: *mut timer_base, clear_pending: bool) -> c_int {
pub static mut idx: unsigned = 0;
    if (!timer_pending(timer)) {
    return 0;
    }
    if (hlist_is_singular_node(&timer.entry, base.vectors + idx)) {
    __clear_bit(idx, base.pending_map);
    base.next_expiry_recalc = true;
    }
    detach_timer(timer, clear_pending);
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn get_timer_cpu_base(tflags: u32, cpu: u32) -> *mut c_void {
pub static mut index: c_int = 0;
//
// If the timer is deferrable and NO_HZ_COMMON is set then we need
// to use the deferrable base.
//
    if (IS_ENABLED!(CONFIG_NO_HZ_COMMON) && (tflags & TIMER_DEFERRABLE)) {
    index = BASE_DEF;
    }
    return per_cpu_ptr(&timer_bases[index], cpu);
    }
#[no_mangle]
pub unsafe extern "C" fn get_timer_this_cpu_base(tflags: u32) -> *mut c_void {
pub static mut index: c_int = 0;
//
// If the timer is deferrable and NO_HZ_COMMON is set then we need
// to use the deferrable base.
//
    if (IS_ENABLED!(CONFIG_NO_HZ_COMMON) && (tflags & TIMER_DEFERRABLE)) {
    index = BASE_DEF;
    }
    return this_cpu_ptr(&timer_bases[index]);
    }
#[no_mangle]
pub unsafe extern "C" fn get_timer_base(tflags: u32) -> *mut c_void {
    return get_timer_cpu_base(tflags, tflags & TIMER_CPUMASK);
    }
#[no_mangle]
pub unsafe extern "C" fn __forward_timer_base(base: *mut timer_base, basej: c_ulong) {
//
// Check whether we can forward the base. We can only do that when
// @basej is past base->clk otherwise we might rewind base->clk.
//
    if (time_before_eq(basej, base.clk)) {
    return;
    }
//
// If the next expiry value is > jiffies, then we fast forward to
// jiffies otherwise we forward to the next expiry value.
//
    if (time_after(base.next_expiry, basej)) {
    base.clk = basej;
    } else {
    if (WARN_ON_ONCE!(time_before(base.next_expiry, base.clk))) {
    return;
    }
    base.clk = base.next_expiry;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn forward_timer_base(base: *mut timer_base) {
    __forward_timer_base(base, READ_ONCE(jiffies));
    }
//
// We are using hashed locking: Holding per_cpu(timer_bases[x]).lock means
// that all timers which are tied to this base are locked, and the base itself
// is locked too.
//
// So __run_timers/migrate_timers can safely modify all timers which could
// be found in the base->vectors array.
//
// When a timer is migrating then the TIMER_MIGRATING flag is set and we need
// to wait until the migration is done.
//
#[no_mangle]
pub unsafe extern "C" fn lock_timer_base(timer: *mut timer_list, lock: *mut unsigned longflags)
    __acquires(timer.base.) -> *mut c_void {
    for (;;) {
pub static mut base: *mut c_void = core::ptr::null_mut();
    let mut tf = 0;
//
// We need to use READ_ONCE() here, otherwise the compiler
// might re-read @tf between the check for TIMER_MIGRATING
// and spin_lock().
//
    tf = READ_ONCE(timer.flags);
    if (!(tf & TIMER_MIGRATING)) {
    base = get_timer_base(tf);
    raw_spin_lock_irqsave(&base.lock, *flags);
    if (timer.flags == tf) {
    return base;
    }
    raw_spin_unlock_irqrestore(&base.lock, *flags);
    }
    cpu_relax();
    }
    }
pub const MOD_TIMER_PENDING_ONLY: c_uint = 0x01;
pub const MOD_TIMER_REDUCE: c_uint = 0x02;
pub const MOD_TIMER_NOTPENDING: c_uint = 0x04;
#[no_mangle]
pub unsafe extern "C" fn __mod_timer(timer: *mut timer_list, expires: c_ulong, options: c_uint) -> c_int {
pub static mut clk: c_ulong = 0;
    let mut base = core::ptr::null_mut();
    let mut new_base = core::ptr::null_mut();
pub static mut idx: c_uint = 0;
pub static mut ret: c_int = 0;
    debug_assert_init(timer);
//
// This is a common optimization triggered by the networking code - if
// the timer is re-modified to have the same timeout or ends up in the
// same array bucket then just return:
//
    if (!(options & MOD_TIMER_NOTPENDING) && timer_pending(timer)) {
//
// The downside of this optimization is that it can result in
// larger granularity than you would get from adding a new
// timer with this expiry.
//
pub static mut diff: c_long = 0;
    if (!diff) {
    return 1;
    }
    if (options & MOD_TIMER_REDUCE && diff <= 0) {
    return 1;
    }
//
// We lock timer base and calculate the bucket index right
// here. If the timer ends up in the same bucket, then we
// just update the expiry time and avoid the whole
// dequeue/enqueue dance.
//
    base = lock_timer_base(timer, &flags);
//
// Has @timer been shutdown? This needs to be evaluated
// while holding base lock to prevent a race against the
// shutdown code.
//
    if (!timer.function) {
// goto;
    }
    forward_timer_base(base);
    if (timer_pending(timer) && (options & MOD_TIMER_REDUCE) &&
    time_before_eq(timer.expires, expires)) {
    ret = 1;
// goto;
    }
    clk = base.clk;
    idx = calc_wheel_index(expires, clk, &bucket_expiry);
//
// Retrieve and compare the array index of the pending
// timer. If it matches set the expiry to the new value so a
// subsequent call will exit in the expires check above.
//
    if (idx == timer_get_idx(timer)) {
    if (!(options & MOD_TIMER_REDUCE)) {
    timer.expires = expires;
    }

    else if (time_after(timer.expires, expires)) {
    timer.expires = expires;
    }
    ret = 1;
// goto;
    }
    } else {
    base = lock_timer_base(timer, &flags);
//
// Has @timer been shutdown? This needs to be evaluated
// while holding base lock to prevent a race against the
// shutdown code.
//
    if (!timer.function) {
// goto;
    }
    forward_timer_base(base);
    }
    ret = detach_if_pending(timer, base, false);
    if (!ret && (options & MOD_TIMER_PENDING_ONLY)) {
// goto;
    }
    new_base = get_timer_this_cpu_base(timer.flags);
    if (base != new_base) {
//
// We are trying to schedule the timer on the new base.
// However we can't change timer's base while it is running,
// otherwise timer_delete_sync() can't detect that the timer's
// handler yet has not finished. This also guarantees that the
// timer is serialized wrt itself.
//
    if (likely(base.running_timer != timer)) {
// See the comment in lock_timer_base()
    timer.flags |= TIMER_MIGRATING;
    raw_spin_unlock(&base.lock);
    base = new_base;
    raw_spin_lock(&base.lock);
    WRITE_ONCE(timer.flags,
    (timer.flags & ~TIMER_BASEMASK) | base.cpu);
    forward_timer_base(base);
    }
    }
    debug_timer_activate(timer);
    timer.expires = expires;
//
// If 'idx' was calculated above and the base time did not advance
// between calculating 'idx' and possibly switching the base, only
// enqueue_timer() is required. Otherwise we need to (re)calculate
// the wheel index via internal_add_timer().
//
    if (idx != UINT_MAX && clk == base.clk) {
    enqueue_timer(base, timer, idx, bucket_expiry);
    }
    else {
// forward_decl: ernal_add_timer;
    }
// label;
    raw_spin_unlock_irqrestore(&base.lock, flags);
    return ret;
    }
//
// mod_timer_pending - Modify a pending timer's timeout
// @timer:	The pending timer to be modified
// @expires:	New absolute timeout in jiffies
//
// mod_timer_pending() is the same for pending timers as mod_timer(), but
// will not activate inactive timers.
//
// If @timer->function == NULL then the start operation is silently
// discarded.
//
// Return:
// * %0 - The timer was inactive and not modified or was in
// shutdown state and the operation was discarded
// * %1 - The timer was active and requeued to expire at @expires
//
#[no_mangle]
pub unsafe extern "C" fn mod_timer_pending(timer: *mut timer_list, expires: c_ulong) -> c_int {
    return __mod_timer(timer, expires, MOD_TIMER_PENDING_ONLY);
    }
    EXPORT_SYMBOL(mod_timer_pending);
//
// mod_timer - Modify a timer's timeout
// @timer:	The timer to be modified
// @expires:	New absolute timeout in jiffies
//
// mod_timer(timer, expires) is equivalent to:
//
// timer_delete(timer); timer->expires = expires; add_timer(timer);
//
// mod_timer() is more efficient than the above open coded sequence. In
// case that the timer is inactive, the timer_delete() part is a NOP. The
// timer is in any case activated with the new expiry time @expires.
//
// Note that if there are multiple unserialized concurrent users of the
// same timer, then mod_timer() is the only safe way to modify the timeout,
// since add_timer() cannot modify an already running timer.
//
// If @timer->function == NULL then the start operation is silently
// discarded. In this case the return value is 0 and meaningless.
//
// Return:
// * %0 - The timer was inactive and started or was in shutdown
// state and the operation was discarded
// * %1 - The timer was active and requeued to expire at @expires or
// the timer was active and not modified because @expires did
// not change the effective expiry time
//
#[no_mangle]
pub unsafe extern "C" fn mod_timer(timer: *mut timer_list, expires: c_ulong) -> c_int {
    return __mod_timer(timer, expires, 0);
    }
    EXPORT_SYMBOL(mod_timer);
//
// timer_reduce - Modify a timer's timeout if it would reduce the timeout
// @timer:	The timer to be modified
// @expires:	New absolute timeout in jiffies
//
// timer_reduce() is very similar to mod_timer(), except that it will only
// modify an enqueued timer if that would reduce the expiration time. If
// @timer is not enqueued it starts the timer.
//
// If @timer->function == NULL then the start operation is silently
// discarded.
//
// Return:
// * %0 - The timer was inactive and started or was in shutdown
// state and the operation was discarded
// * %1 - The timer was active and requeued to expire at @expires or
// the timer was active and not modified because @expires
// did not change the effective expiry time such that the
// timer would expire earlier than already scheduled
//
#[no_mangle]
pub unsafe extern "C" fn timer_reduce(timer: *mut timer_list, expires: c_ulong) -> c_int {
    return __mod_timer(timer, expires, MOD_TIMER_REDUCE);
    }
    EXPORT_SYMBOL(timer_reduce);
//
// add_timer - Start a timer
// @timer:	The timer to be started
//
// Start @timer to expire at @timer->expires in the future. @timer->expires
// is the absolute expiry time measured in 'jiffies'. When the timer expires
// timer->function(timer) will be invoked from soft interrupt context.
//
// The @timer->expires and @timer->function fields must be set prior
// to calling this function.
//
// If @timer->function == NULL then the start operation is silently
// discarded.
//
// If @timer->expires is already in the past @timer will be queued to
// expire at the next timer tick.
//
// This can only operate on an inactive timer. Attempts to invoke this on
// an active timer are rejected with a warning.
//
#[no_mangle]
pub unsafe extern "C" fn add_timer(timer: *mut timer_list) {
    if (WARN_ON_ONCE!(timer_pending(timer))) {
    return;
    }
    __mod_timer(timer, timer.expires, MOD_TIMER_NOTPENDING);
    }
    EXPORT_SYMBOL(add_timer);
//
// add_timer_local() - Start a timer on the local CPU
// @timer:	The timer to be started
//
// Same as add_timer() except that the timer flag TIMER_PINNED is set.
//
// See add_timer() for further details.
//
#[no_mangle]
pub unsafe extern "C" fn add_timer_local(timer: *mut timer_list) {
    if (WARN_ON_ONCE!(timer_pending(timer))) {
    return;
    }
    timer.flags |= TIMER_PINNED;
    __mod_timer(timer, timer.expires, MOD_TIMER_NOTPENDING);
    }
    EXPORT_SYMBOL(add_timer_local);
//
// add_timer_global() - Start a timer without TIMER_PINNED flag set
// @timer:	The timer to be started
//
// Same as add_timer() except that the timer flag TIMER_PINNED is unset.
//
// See add_timer() for further details.
//
#[no_mangle]
pub unsafe extern "C" fn add_timer_global(timer: *mut timer_list) {
    if (WARN_ON_ONCE!(timer_pending(timer))) {
    return;
    }
    timer.flags &= ~TIMER_PINNED;
    __mod_timer(timer, timer.expires, MOD_TIMER_NOTPENDING);
    }
    EXPORT_SYMBOL(add_timer_global);
//
// add_timer_on - Start a timer on a particular CPU
// @timer:	The timer to be started
// @cpu:	The CPU to start it on
//
// Same as add_timer() except that it starts the timer on the given CPU and
// the TIMER_PINNED flag is set. When timer shouldn't be a pinned timer in
// the next round, add_timer_global() should be used instead as it unsets
// the TIMER_PINNED flag.
//
// See add_timer() for further details.
//
#[no_mangle]
pub unsafe extern "C" fn add_timer_on(timer: *mut timer_list, cpu: c_int) {
    let mut new_base = core::ptr::null_mut();
    let mut base = core::ptr::null_mut();
    let mut flags = 0;
    debug_assert_init(timer);
    if (WARN_ON_ONCE!(timer_pending(timer))) {
    return;
    }
// Make sure timer flags have TIMER_PINNED flag set
    timer.flags |= TIMER_PINNED;
    new_base = get_timer_cpu_base(timer.flags, cpu);
//
// If @timer was on a different CPU, it should be migrated with the
// old base locked to prevent other operations proceeding with the
// wrong base locked.  See lock_timer_base().
//
    base = lock_timer_base(timer, &flags);
//
// Has @timer been shutdown? This needs to be evaluated while
// holding base lock to prevent a race against the shutdown code.
//
    if (!timer.function) {
// goto;
    }
    if (base != new_base) {
    timer.flags |= TIMER_MIGRATING;
    raw_spin_unlock(&base.lock);
    base = new_base;
    raw_spin_lock(&base.lock);
    WRITE_ONCE(timer.flags,
    (timer.flags & ~TIMER_BASEMASK) | cpu);
    }
    forward_timer_base(base);
    debug_timer_activate(timer);
// forward_decl: ernal_add_timer;
// label;
    raw_spin_unlock_irqrestore(&base.lock, flags);
    }
    EXPORT_SYMBOL_GPL(add_timer_on);
//
// __timer_delete - Internal function: Deactivate a timer
// @timer:	The timer to be deactivated
// @shutdown:	If true, this indicates that the timer is about to be
// shutdown permanently.
//
// If @shutdown is true then @timer->function is set to NULL under the
// timer base lock which prevents further rearming of the time. In that
// case any attempt to rearm @timer after this function returns will be
// silently ignored.
//
// Return:
// * %0 - The timer was not pending
// * %1 - The timer was pending and deactivated
//
#[no_mangle]
unsafe extern "C" fn __timer_delete(timer: *mut timer_list, shutdown: bool) -> c_int {
pub static mut base: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
pub static mut ret: c_int = 0;
    debug_assert_init(timer);
//
// If @shutdown is set then the lock has to be taken whether the
// timer is pending or not to protect against a concurrent rearm
// which might hit between the lockless pending check and the lock
// acquisition. By taking the lock it is ensured that such a newly
// enqueued timer is dequeued and cannot end up with
// timer->function == NULL in the expiry code.
//
// If timer->function is currently executed, then this makes sure
// that the callback cannot requeue the timer.
//
    if (timer_pending(timer) || shutdown) {
    base = lock_timer_base(timer, &flags);
    ret = detach_if_pending(timer, base, true);
    if (shutdown) {
    timer.function = core::ptr::null_mut();
    }
    raw_spin_unlock_irqrestore(&base.lock, flags);
    }
    return ret;
    }
//
// timer_delete - Deactivate a timer
// @timer:	The timer to be deactivated
//
// The function only deactivates a pending timer, but contrary to
// timer_delete_sync() it does not take into account whether the timer's
// callback function is concurrently executed on a different CPU or not.
// It neither prevents rearming of the timer.  If @timer can be rearmed
// concurrently then the return value of this function is meaningless.
//
// Return:
// * %0 - The timer was not pending
// * %1 - The timer was pending and deactivated
//
#[no_mangle]
pub unsafe extern "C" fn timer_delete(timer: *mut timer_list) -> c_int {
    return __timer_delete(timer, false);
    }
    EXPORT_SYMBOL(timer_delete);
//
// timer_shutdown - Deactivate a timer and prevent rearming
// @timer:	The timer to be deactivated
//
// The function does not wait for an eventually running timer callback on a
// different CPU but it prevents rearming of the timer. Any attempt to arm
// @timer after this function returns will be silently ignored.
//
// This function is useful for teardown code and should only be used when
// timer_shutdown_sync() cannot be invoked due to locking or context constraints.
//
// Return:
// * %0 - The timer was not pending
// * %1 - The timer was pending
//
#[no_mangle]
pub unsafe extern "C" fn timer_shutdown(timer: *mut timer_list) -> c_int {
    return __timer_delete(timer, true);
    }
    EXPORT_SYMBOL_GPL(timer_shutdown);
//
// __try_to_del_timer_sync - Internal function: Try to deactivate a timer
// @timer:	Timer to deactivate
// @shutdown:	If true, this indicates that the timer is about to be
// shutdown permanently.
//
// If @shutdown is true then @timer->function is set to NULL under the
// timer base lock which prevents further rearming of the timer. Any
// attempt to rearm @timer after this function returns will be silently
// ignored.
//
// This function cannot guarantee that the timer cannot be rearmed
// right after dropping the base lock if @shutdown is false. That
// needs to be prevented by the calling code if necessary.
//
// Return:
// * %0  - The timer was not pending
// * %1  - The timer was pending and deactivated
// * %-1 - The timer callback function is running on a different CPU
//
#[no_mangle]
unsafe extern "C" fn __try_to_del_timer_sync(timer: *mut timer_list, shutdown: bool) -> c_int {
pub static mut base: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
pub static mut ret: c_int = 0;
    debug_assert_init(timer);
    base = lock_timer_base(timer, &flags);
    if (base.running_timer != timer) {
    ret = detach_if_pending(timer, base, true);
    if (shutdown) {
    timer.function = core::ptr::null_mut();
    }
    }
    raw_spin_unlock_irqrestore(&base.lock, flags);
    return ret;
    }
//
// timer_delete_sync_try - Try to deactivate a timer
// @timer:	Timer to deactivate
//
// This function tries to deactivate a timer. On success the timer is not
// queued and the timer callback function is not running on any CPU.
//
// This function does not guarantee that the timer cannot be rearmed right
// after dropping the base lock. That needs to be prevented by the calling
// code if necessary.
//
// Return:
// * %0  - The timer was not pending
// * %1  - The timer was pending and deactivated
// * %-1 - The timer callback function is running on a different CPU
//
#[no_mangle]
pub unsafe extern "C" fn timer_delete_sync_try(timer: *mut timer_list) -> c_int {
    return __try_to_del_timer_sync(timer, false);
    }
    EXPORT_SYMBOL(timer_delete_sync_try);

#[no_mangle]
unsafe extern "C" fn timer_base_init_expiry_lock(base: *mut timer_base) -> __init void {
    spin_lock_init(&base.expiry_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn timer_base_lock_expiry(base: *mut timer_base) {
    spin_lock(&base.expiry_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn timer_base_unlock_expiry(base: *mut timer_base) {
    spin_unlock(&base.expiry_lock);
    }
//
// The counterpart to del_timer_wait_running().
//
// If there is a waiter for base->expiry_lock, then it was waiting for the
// timer callback to finish. Drop expiry_lock and reacquire it. That allows
// the waiter to acquire the lock and make progress.
//
#[no_mangle]
unsafe extern "C" fn timer_sync_wait_running(base: *mut timer_base) {
    if (atomic_read(&base.timer_waiters)) {
    raw_spin_unlock_irq(&base.lock);
    spin_unlock(&base.expiry_lock);
    spin_lock(&base.expiry_lock);
    raw_spin_lock_irq(&base.lock);
    }
    }
//
// This function is called on PREEMPT_RT kernels when the fast path
// deletion of a timer failed because the timer callback function was
// running.
//
// This prevents priority inversion, if the softirq thread on a remote CPU
// got preempted, and it prevents a life lock when the task which tries to
// delete a timer preempted the softirq thread running the timer callback
// function.
//
#[no_mangle]
unsafe extern "C" fn del_timer_wait_running(timer: *mut timer_list) {
    let mut tf = 0;
    tf = READ_ONCE(timer.flags);
    if (!(tf & (TIMER_MIGRATING | TIMER_IRQSAFE))) {
    let mut base = get_timer_base(tf);
//
// Mark the base as contended and grab the expiry lock,
// which is held by the softirq across the timer
// callback. Drop the lock immediately so the softirq can
// expire the next timer. In theory the timer could already
// be running again, but that's more than unlikely and just
// causes another wait loop.
//
    atomic_inc(&base.timer_waiters);
    spin_lock_bh(&base.expiry_lock);
    atomic_dec(&base.timer_waiters);
    spin_unlock_bh(&base.expiry_lock);
    }
    }

#[no_mangle]
pub unsafe extern "C" fn timer_base_init_expiry_lock(base: *mut timer_base) { }
#[no_mangle]
#[no_mangle]
// duplicate fn: timer_base_lock_expiry
pub unsafe extern "C" fn timer_base_lock_expiry_dup(base: *mut timer_base) { }
#[no_mangle]
#[no_mangle]
// duplicate fn: timer_base_unlock_expiry
pub unsafe extern "C" fn timer_base_unlock_expiry_dup(base: *mut timer_base) { }
#[no_mangle]
pub unsafe extern "C" fn timer_sync_wait_running(base: *mut timer_base) { }
#[no_mangle]
pub unsafe extern "C" fn del_timer_wait_running(timer: *mut timer_list) { }

//
// __timer_delete_sync - Internal function: Deactivate a timer and wait
// for the handler to finish.
// @timer:	The timer to be deactivated
// @shutdown:	If true, @timer->function will be set to NULL under the
// timer base lock which prevents rearming of @timer
//
// If @shutdown is not set the timer can be rearmed later. If the timer can
// be rearmed concurrently, i.e. after dropping the base lock then the
// return value is meaningless.
//
// If @shutdown is set then @timer->function is set to NULL under timer
// base lock which prevents rearming of the timer. Any attempt to rearm
// a shutdown timer is silently ignored.
//
// If the timer should be reused after shutdown it has to be initialized
// again.
//
// Return:
// * %0	- The timer was not pending
// * %1	- The timer was pending and deactivated
//
#[no_mangle]
unsafe extern "C" fn __timer_delete_sync(timer: *mut timer_list, shutdown: bool) -> c_int {
    let mut ret = 0;

    let mut flags = 0;
//
// If lockdep gives a backtrace here, please reference
// the synchronization rules above.
//
    local_irq_save(flags);
    lock_map_acquire(&timer.lockdep_map);
    lock_map_release(&timer.lockdep_map);
    local_irq_restore(flags);

//
// don't use it in hardirq context, because it
// could lead to deadlock.
//
    WARN_ON!(in_hardirq() && !(timer.flags & TIMER_IRQSAFE));
//
// Must be able to sleep on PREEMPT_RT because of the slowpath in
// del_timer_wait_running().
//
    if (IS_ENABLED!(CONFIG_PREEMPT_RT) && !(timer.flags & TIMER_IRQSAFE)) {
    lockdep_assert_preemption_enabled();
    }
    do {
    ret = __try_to_del_timer_sync(timer, shutdown);
    if (unlikely(ret < 0)) {
    del_timer_wait_running(timer);
    cpu_relax();
    }
    } while (ret < 0);
    return ret;
    }
//
// timer_delete_sync - Deactivate a timer and wait for the handler to finish.
// @timer:	The timer to be deactivated
//
// Synchronization rules: Callers must prevent restarting of the timer,
// otherwise this function is meaningless. It must not be called from
// interrupt contexts unless the timer is an irqsafe one. The caller must
// not hold locks which would prevent completion of the timer's callback
// function. The timer's handler must not call add_timer_on(). Upon exit
// the timer is not queued and the handler is not running on any CPU.
//
// For !irqsafe timers, the caller must not hold locks that are held in
// interrupt context. Even if the lock has nothing to do with the timer in
// question.  Here's why::
//
// CPU0                             CPU1
// ----                             ----
// <SOFTIRQ>
// call_timer_fn();
// base->running_timer = mytimer;
// spin_lock_irq(somelock);
// <IRQ>
// spin_lock(somelock);
// timer_delete_sync(mytimer);
// while (base->running_timer == mytimer);
//
// Now timer_delete_sync() will never return and never release somelock.
// The interrupt on the other CPU is waiting to grab somelock but it has
// interrupted the softirq that CPU0 is waiting to finish.
//
// This function cannot guarantee that the timer is not rearmed again by
// some concurrent or preempting code, right after it dropped the base
// lock. If there is the possibility of a concurrent rearm then the return
// value of the function is meaningless.
//
// If such a guarantee is needed, e.g. for teardown situations then use
// timer_shutdown_sync() instead.
//
// Return:
// * %0	- The timer was not pending
// * %1	- The timer was pending and deactivated
//
#[no_mangle]
pub unsafe extern "C" fn timer_delete_sync(timer: *mut timer_list) -> c_int {
    return __timer_delete_sync(timer, false);
    }
    EXPORT_SYMBOL(timer_delete_sync);
//
// timer_shutdown_sync - Shutdown a timer and prevent rearming
// @timer: The timer to be shutdown
//
// When the function returns it is guaranteed that:
// - @timer is not queued
// - The callback function of @timer is not running
// - @timer cannot be enqueued again. Any attempt to rearm
// @timer is silently ignored.
//
// See timer_delete_sync() for synchronization rules.
//
// This function is useful for final teardown of an infrastructure where
// the timer is subject to a circular dependency problem.
//
// A common pattern for this is a timer and a workqueue where the timer can
// schedule work and work can arm the timer. On shutdown the workqueue must
// be destroyed and the timer must be prevented from rearming. Unless the
// code has conditionals like 'if (mything->in_shutdown)' to prevent that
// there is no way to get this correct with timer_delete_sync().
//
// timer_shutdown_sync() is solving the problem. The correct ordering of
// calls in this case is:
//
// timer_shutdown_sync(&mything->timer);
// workqueue_destroy(&mything->workqueue);
//
// After this 'mything' can be safely freed.
//
// This obviously implies that the timer is not required to be functional
// for the rest of the shutdown operation.
//
// Return:
// * %0 - The timer was not pending
// * %1 - The timer was pending
//
#[no_mangle]
pub unsafe extern "C" fn timer_shutdown_sync(timer: *mut timer_list) -> c_int {
    return __timer_delete_sync(timer, true);
    }
    EXPORT_SYMBOL_GPL(timer_shutdown_sync);
#[no_mangle]
pub unsafe extern "C" fn call_timer_fn(timer: *mut timer_list, baseclk: c_ulong) {
pub static mut count: c_int = 0;

//
// It is permissible to free the timer from inside the
// function that is called from it, this we need to take into
// account for lockdep too. To avoid bogus "held lock freed"
// warnings as well as problems when looking into
// timer->lockdep_map, make a copy and use that here.
//
pub static mut lockdep_map: usize = 0;
    lockdep_copy_map(&lockdep_map, &timer.lockdep_map);

//
// Couple the lock chain with the lock chain at
// timer_delete_sync() by acquiring the lock_map around the fn()
// call here and in timer_delete_sync().
//
    lock_map_acquire(&lockdep_map);
    trace_timer_expire_entry(timer, baseclk);
    fn(timer);
    trace_timer_expire_exit(timer);
    lock_map_release(&lockdep_map);
    if (count != preempt_count()) {
    WARN_ONCE(1, "timer: %pS preempt leak: %08x . %08x\n",
    fn, count, preempt_count());
//
// Restore the preempt count. That gives us a decent
// chance to survive and extract information. If the
// callback kept a lock held, bad luck, but not worse
// than the BUG() we had.
//
    preempt_count_set(count);
    }
    }
#[no_mangle]
unsafe extern "C" fn expire_timers(base: *mut timer_base, head: *mut hlist_head) {
//
// This value is required only for tracing. base->clk was
// incremented directly before expire_timers was called. But expiry
// is related to the old base->clk value.
//
pub static mut baseclk: c_ulong = 0;
    while (!hlist_empty(head)) {
pub static mut timer: *mut c_void = core::ptr::null_mut();
    void (*fn);
    timer = hlist_entry(head.first, timer_list, entry);
    base.running_timer = timer;
    detach_timer(timer, true);
    fn = timer.function;
    if (WARN_ON_ONCE!(!fn)) {
// Should never happen. Emphasis on should!
    base.running_timer = core::ptr::null_mut();
    continue;
    }
    if (timer.flags & TIMER_IRQSAFE) {
    raw_spin_unlock(&base.lock);
    call_timer_fn(timer, fn, baseclk);
    raw_spin_lock(&base.lock);
    base.running_timer = core::ptr::null_mut();
    } else {
    raw_spin_unlock_irq(&base.lock);
    call_timer_fn(timer, fn, baseclk);
    raw_spin_lock_irq(&base.lock);
    base.running_timer = core::ptr::null_mut();
    timer_sync_wait_running(base);
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn collect_expired_timers(base: *mut timer_base, heads: *mut hlist_head) -> c_int {
pub static mut clk: c_ulong = 0;
pub static mut vec: *mut c_void = core::ptr::null_mut();
    int i, levels = 0;
    let mut idx = 0;
    while (i < LVL_DEPTH) {
    idx = (clk & LVL_MASK) + i * LVL_SIZE;
    if (__test_and_clear_bit(idx, base.pending_map)) {
    vec = base.vectors + idx;
    hlist_move_list(vec, heads++);
    levels += 1;
    }
// Is it time to look at the next level?
    if (clk & LVL_CLK_MASK) {
    break;
    }
// Shift clock for the next level granularity
    clk >>= LVL_CLK_SHIFT;
    }
    return levels;
    }
//
// Find the next pending bucket of a level. Search from level start (@offset)
// + @clk upwards and if nothing there, search from start of the level
// (@offset) up to @offset + clk.
//
#[no_mangle]
pub unsafe extern "C" fn next_pending_bucket(base: *mut timer_base, offset: c_uint, clk: c_uint) -> c_int {
    unsigned pos, start = offset + clk;
pub static mut end: unsigned = 0;
    pos = find_next_bit(base.pending_map, end, start);
    if (pos < end) {
    return pos - start;
    }
    pos = find_next_bit(base.pending_map, start, offset);
    return pos < start ? pos + LVL_SIZE - start : -1;
    }
//
// Search the first expiring timer in the various clock levels. Caller must
// hold base->lock.
//
// Store next expiry time in base->next_expiry.
//
#[no_mangle]
unsafe extern "C" fn timer_recalc_next_expiry(base: *mut timer_base) {
    unsigned long clk, next, adj;
    unsigned lvl, offset = 0;
    next = base.clk + TIMER_NEXT_MAX_DELTA;
    clk = base.clk;
    while (lvl < LVL_DEPTH) {
pub static mut pos: c_int = 0;
pub static mut lvl_clk: c_ulong = 0;
    if (pos >= 0) {
pub static mut tmp: c_ulong = 0;
    tmp <<= LVL_SHIFT(lvl);
    if (time_before(tmp, next)) {
    next = tmp;
    }
//
// If the next expiration happens before we reach
// the next level, no need to check further.
//
    if (pos <= ((LVL_CLK_DIV - lvl_clk) & LVL_CLK_MASK)) {
    break;
    }
    }
//
// Clock for the next level. If the current level clock lower
// bits are zero, we look at the next level as is. If not we
// need to advance it by one because that's going to be the
// next expiring bucket in that level. base->clk is the next
// expiring jiffy. So in case of:
//
// LVL5 LVL4 LVL3 LVL2 LVL1 LVL0
// 0    0    0    0    0    0
//
// we have to look at all levels @index 0. With
//
// LVL5 LVL4 LVL3 LVL2 LVL1 LVL0
// 0    0    0    0    0    2
//
// LVL0 has the next expiring bucket @index 2. The upper
// levels have the next expiring bucket @index 1.
//
// In case that the propagation wraps the next level the same
// rules apply:
//
// LVL5 LVL4 LVL3 LVL2 LVL1 LVL0
// 0    0    0    0    F    2
//
// So after looking at LVL0 we get:
//
// LVL5 LVL4 LVL3 LVL2 LVL1
// 0    0    0    1    0
//
// So no propagation from LVL1 to LVL2 because that happened
// with the add already, but then we need to propagate further
// from LVL2 to LVL3.
//
// So the simple check whether the lower bits of the current
// level are 0 or not is sufficient for all cases.
//
    adj = lvl_clk ? 1 : 0;
    clk >>= LVL_CLK_SHIFT;
    clk += adj;
    }
    WRITE_ONCE(base.next_expiry, next);
    base.next_expiry_recalc = false;
    base.timers_pending = !(next == base.clk + TIMER_NEXT_MAX_DELTA);
    }

//
// Check, if the next hrtimer event is before the next timer wheel
// event:
//
#[no_mangle]
unsafe extern "C" fn cmp_next_hrtimer_event(basem: u64, expires: u64) -> u64 {
pub static mut nextevt: u64 = 0;
//
// If high resolution timers are enabled
// hrtimer_get_next_event() returns KTIME_MAX.
//
    if (expires <= nextevt) {
    return expires;
    }
//
// If the next timer is already expired, return the tick base
// time so the tick is fired immediately.
//
    if (nextevt <= basem) {
    return basem;
    }
//
// Round up to the next jiffy. High resolution timers are
// off, so the hrtimers are expired in the tick and we need to
// make sure that this tick really expires the timer to avoid
// a ping pong of the nohz stop code.
//
// Use DIV_ROUND_UP_ULL to prevent gcc calling __divdi3
//
    return DIV_ROUND_UP_ULL(nextevt, TICK_NSEC) * TICK_NSEC;
    }
#[no_mangle]
pub unsafe extern "C" fn next_timer_interrupt(base: *mut timer_base, basej: c_ulong) -> c_ulong {
    if (base.next_expiry_recalc) {
    timer_recalc_next_expiry(base);
    }
//
// Move next_expiry for the empty base into the future to prevent an
// unnecessary raise of the timer softirq when the next_expiry value
// will be reached even if there is no timer pending.
//
// This update is also required to make timer_base::next_expiry values
// easy comparable to find out which base holds the first pending timer.
//
    if (!base.timers_pending) {
    WRITE_ONCE(base.next_expiry, basej + TIMER_NEXT_MAX_DELTA);
    }
    return base.next_expiry;
    }
#[no_mangle]
pub unsafe extern "C" fn fetch_next_timer_interrupt(basej: c_ulong, basem: u64, base_local: *mut timer_base, base_global: *mut timer_base, tevt: *mut timer_events) -> c_ulong {
    unsigned long nextevt, nextevt_local, nextevt_global;
    let mut local_first = 0;
    nextevt_local = next_timer_interrupt(base_local, basej);
    nextevt_global = next_timer_interrupt(base_global, basej);
    local_first = time_before_eq(nextevt_local, nextevt_global);
    nextevt = local_first ? nextevt_local : nextevt_global;
//
// If the @nextevt is at max. one tick away, use @nextevt and store
// it in the local expiry value. The next global event is irrelevant in
// this case and can be left as KTIME_MAX.
//
    if (time_before_eq(nextevt, basej + 1)) {
// If we missed a tick already, force 0 delta
    if (time_before(nextevt, basej)) {
    nextevt = basej;
    }
    tevt.local = basem + (u64)(nextevt - basej) * TICK_NSEC;
//
// This is required for the remote check only but it doesn't
// hurt, when it is done for both call sites:
//
// * The remote callers will only take care of the global timers
// as local timers will be handled by CPU itself. When not
// updating tevt->global with the already missed first global
// timer, it is possible that it will be missed completely.
//
// * The local callers will ignore the tevt->global anyway, when
// nextevt is max. one tick away.
//
    if (!local_first) {
    tevt.global = tevt.local;
    }
    return nextevt;
    }
//
// Update tevt.* values:
//
// If the local queue expires first, then the global event can be
// ignored. If the global queue is empty, nothing to do either.
//
    if (!local_first && base_global.timers_pending) {
    tevt.global = basem + (u64)(nextevt_global - basej) * TICK_NSEC;
    }
    if (base_local.timers_pending) {
    tevt.local = basem + (u64)(nextevt_local - basej) * TICK_NSEC;
    }
    return nextevt;
    }

//
// fetch_next_timer_interrupt_remote() - Store next timers into @tevt
// @basej:	base time jiffies
// @basem:	base time clock monotonic
// @tevt:	Pointer to the storage for the expiry values
// @cpu:	Remote CPU
//
// Stores the next pending local and global timer expiry values in the
// struct pointed to by @tevt. If a queue is empty the corresponding
// field is set to KTIME_MAX. If local event expires before global
// event, global event is set to KTIME_MAX as well.
//
// Caller needs to make sure timer base locks are held (use
// timer_lock_remote_bases() for this purpose).
//
#[no_mangle]
pub unsafe extern "C" fn fetch_next_timer_interrupt_remote(basej: c_ulong, basem: u64, tevt: *mut timer_events, cpu: c_uint) {
    let mut base_local = core::ptr::null_mut();
    let mut base_global = core::ptr::null_mut();
// Preset local / global events
    tevt.local = tevt.global = KTIME_MAX;
    base_local = per_cpu_ptr(&timer_bases[BASE_LOCAL], cpu);
    base_global = per_cpu_ptr(&timer_bases[BASE_GLOBAL], cpu);
    lockdep_assert_held(&base_local.lock);
    lockdep_assert_held(&base_global.lock);
    fetch_next_timer_interrupt(basej, basem, base_local, base_global, tevt);
    }
//
// timer_unlock_remote_bases - unlock timer bases of cpu
// @cpu:	Remote CPU
//
// Unlocks the remote timer bases.
//
#[no_mangle]
pub unsafe extern "C" fn timer_unlock_remote_bases(cpu: c_uint) {
    let mut base_local = core::ptr::null_mut();
    let mut base_global = core::ptr::null_mut();
    base_local = per_cpu_ptr(&timer_bases[BASE_LOCAL], cpu);
    base_global = per_cpu_ptr(&timer_bases[BASE_GLOBAL], cpu);
    raw_spin_unlock(&base_global.lock);
    raw_spin_unlock(&base_local.lock);
    }
//
// timer_lock_remote_bases - lock timer bases of cpu
// @cpu:	Remote CPU
//
// Locks the remote timer bases.
//
#[no_mangle]
pub unsafe extern "C" fn timer_lock_remote_bases(cpu: c_uint) {
    let mut base_local = core::ptr::null_mut();
    let mut base_global = core::ptr::null_mut();
    base_local = per_cpu_ptr(&timer_bases[BASE_LOCAL], cpu);
    base_global = per_cpu_ptr(&timer_bases[BASE_GLOBAL], cpu);
    lockdep_assert_irqs_disabled();
    raw_spin_lock(&base_local.lock);
    raw_spin_lock_nested(&base_global.lock, SINGLE_DEPTH_NESTING);
    }
//
// timer_base_is_idle() - Return whether timer base is set idle
//
// Returns value of local timer base is_idle value.
//
#[no_mangle]
pub unsafe extern "C" fn timer_base_is_idle() -> bool {
    return __this_cpu_read(timer_bases[BASE_LOCAL].is_idle);
    }
// forward_decl: __run_timer_base;
//
// timer_expire_remote() - expire global timers of cpu
// @cpu:	Remote CPU
//
// Expire timers of global base of remote CPU.
//
#[no_mangle]
pub unsafe extern "C" fn timer_expire_remote(cpu: c_uint) {
    let mut base = per_cpu_ptr(&timer_bases[BASE_GLOBAL], cpu);
    __run_timer_base(base);
    }
#[no_mangle]
pub unsafe extern "C" fn timer_use_tmigr(basej: c_ulong, basem: u64, nextevt: *mut c_ulong, tick_stop_path: *mut bool, timer_base_idle: bool, tevt: *mut timer_events) {
    let mut next_tmigr = 0;
    if (timer_base_idle) {
    next_tmigr = tmigr_cpu_new_timer(tevt.global);
    }

    else if (tick_stop_path) {
    next_tmigr = tmigr_cpu_deactivate(tevt.global);
    }
    else {
    next_tmigr = tmigr_quick_check(tevt.global);
    }
//
// If the CPU is the last going idle in timer migration hierarchy, make
// sure the CPU will wake up in time to handle remote timers.
// next_tmigr == KTIME_MAX if other CPUs are still active.
//
    if (next_tmigr < tevt.local) {
    let mut tmp = 0;
// If we missed a tick already, force 0 delta
    if (next_tmigr < basem) {
    next_tmigr = basem;
    }
    tmp = div_u64(next_tmigr - basem, TICK_NSEC);
// nextevt = basej + (unsigned long)tmp;
    tevt.local = next_tmigr;
    }
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: timer_use_tmigr
pub unsafe extern "C" fn timer_use_tmigr_dup(basej: c_ulong, basem: u64, nextevt: *mut c_ulong, tick_stop_path: *mut bool, timer_base_idle: bool, tevt: *mut timer_events) {
//
// Make sure first event is written into tevt->local to not miss a
// timer on !SMP systems.
//
    tevt.local = min_t(u64, tevt.local, tevt.global);
    }

#[no_mangle]
pub unsafe extern "C" fn __get_next_timer_interrupt(basej: c_ulong, basem: u64, idle: *mut bool) -> u64 {
pub static mut tevt: timer_events = 0;
    let mut base_local = core::ptr::null_mut();
    let mut base_global = core::ptr::null_mut();
    let mut nextevt = 0;
    let mut idle_is_possible = 0;
//
// When the CPU is offline, the tick is cancelled and nothing is supposed
// to try to stop it.
//
    if (WARN_ON_ONCE!(cpu_is_offline(smp_processor_id()))) {
    if (idle) {
// idle = true;
    }
    return tevt.local;
    }
    base_local = this_cpu_ptr(&timer_bases[BASE_LOCAL]);
    base_global = this_cpu_ptr(&timer_bases[BASE_GLOBAL]);
    raw_spin_lock(&base_local.lock);
    raw_spin_lock_nested(&base_global.lock, SINGLE_DEPTH_NESTING);
    nextevt = fetch_next_timer_interrupt(basej, basem, base_local,
    base_global, &tevt);
//
// If the next event is only one jiffy ahead there is no need to call
// timer migration hierarchy related functions. The value for the next
// global timer in @tevt struct equals then KTIME_MAX. This is also
// true, when the timer base is idle.
//
// The proper timer migration hierarchy function depends on the callsite
// and whether timer base is idle or not. @nextevt will be updated when
// this CPU needs to handle the first timer migration hierarchy
// event. See timer_use_tmigr() for detailed information.
//
    idle_is_possible = time_after(nextevt, basej + 1);
    if (idle_is_possible) {
    timer_use_tmigr(basej, basem, &nextevt, idle,
    base_local.is_idle, &tevt);
    }
//
// We have a fresh next event. Check whether we can forward the
// base.
//
    __forward_timer_base(base_local, basej);
    __forward_timer_base(base_global, basej);
//
// Set base->is_idle only when caller is timer_base_try_to_set_idle()
//
    if (idle) {
//
// Bases are idle if the next event is more than a tick
// away. Caution: @nextevt could have changed by enqueueing a
// global timer into timer migration hierarchy. Therefore a new
// check is required here.
//
// If the base is marked idle then any timer add operation must
// forward the base clk itself to keep granularity small. This
// idle logic is only maintained for the BASE_LOCAL and
// BASE_GLOBAL base, deferrable timers may still see large
// granularity skew (by design).
//
    if (!base_local.is_idle && time_after(nextevt, basej + 1)) {
    base_local.is_idle = true;
//
// Global timers queued locally while running in a task
// in nohz_full mode need a self-IPI to kick reprogramming
// in IRQ tail.
//
    if (tick_nohz_full_cpu(base_local.cpu)) {
    base_global.is_idle = true;
    }
    trace_timer_base_idle(true, base_local.cpu);
    }
// idle = base_local->is_idle;
//
// When timer base is not set idle, undo the effect of
// tmigr_cpu_deactivate() to prevent inconsistent states - active
// timer base but inactive timer migration hierarchy.
//
// When timer base was already marked idle, nothing will be
// changed here.
//
    if (!base_local.is_idle && idle_is_possible) {
    tmigr_cpu_activate();
    }
    }
    raw_spin_unlock(&base_global.lock);
    raw_spin_unlock(&base_local.lock);
    return cmp_next_hrtimer_event(basem, tevt.local);
    }
//
// get_next_timer_interrupt() - return the time (clock mono) of the next timer
// @basej:	base time jiffies
// @basem:	base time clock monotonic
//
// Returns the tick aligned clock monotonic time of the next pending timer or
// KTIME_MAX if no timer is pending. If timer of global base was queued into
// timer migration hierarchy, first global timer is not taken into account. If
// it was the last CPU of timer migration hierarchy going idle, first global
// event is taken into account.
//
#[no_mangle]
pub unsafe extern "C" fn get_next_timer_interrupt(basej: c_ulong, basem: u64) -> u64 {
    return __get_next_timer_interrupt(basej, basem, core::ptr::null_mut());
    }
//
// timer_base_try_to_set_idle() - Try to set the idle state of the timer bases
// @basej:	base time jiffies
// @basem:	base time clock monotonic
// @idle:	pointer to store the value of timer_base->is_idle on return;
// *idle contains the information whether tick was already stopped
//
// Returns the tick aligned clock monotonic time of the next pending timer or
// KTIME_MAX if no timer is pending. When tick was already stopped KTIME_MAX is
// returned as well.
//
#[no_mangle]
pub unsafe extern "C" fn timer_base_try_to_set_idle(basej: c_ulong, basem: u64, idle: *mut bool) -> u64 {
    if (*idle) {
    return KTIME_MAX;
    }
    return __get_next_timer_interrupt(basej, basem, idle);
    }
//
// timer_clear_idle - Clear the idle state of the timer base
//
// Called with interrupts disabled
//
#[no_mangle]
pub unsafe extern "C" fn timer_clear_idle() {
pub static mut this_cpu: c_int = 0;
//
// We do this unlocked. The worst outcome is a remote pinned timer
// enqueue sending a pointless IPI, but taking the lock would just
// make the window for sending the IPI a few instructions smaller
// for the cost of taking the lock in the exit from idle
// path. Required for BASE_LOCAL only.
//
    __this_cpu_write(timer_bases[BASE_LOCAL].is_idle, false);
    if (tick_nohz_full_cpu(this_cpu)) {
    __this_cpu_write(timer_bases[BASE_GLOBAL].is_idle, false);
    }
    trace_timer_base_idle(false, this_cpu);
// Activate without holding the timer_base->lock
    tmigr_cpu_activate();
    }

//
// __run_timers - run all expired timers (if any) on this CPU.
// @base: the timer vector to be processed.
//
#[no_mangle]
pub unsafe extern "C" fn __run_timers(base: *mut timer_base) {
    struct hlist_head heads[LVL_DEPTH];
    let mut levels = 0;
    lockdep_assert_held(&base.lock);
    if (base.running_timer) {
    return;
    }
    while (time_after_eq(jiffies, base.clk) &&
    time_after_eq(jiffies, base.next_expiry)) {
    levels = collect_expired_timers(base, heads);
//
// The two possible reasons for not finding any expired
// timer at this clk are that all matching timers have been
// dequeued or no timer has been queued since
// base::next_expiry was set to base::clk +
// TIMER_NEXT_MAX_DELTA.
//
    WARN_ON_ONCE!(!levels && !base.next_expiry_recalc
    && base.timers_pending);
//
// While executing timers, base->clk is set 1 offset ahead of
// jiffies to avoid endless requeuing to current jiffies.
//
    base.clk += 1;
    timer_recalc_next_expiry(base);
    while (levels--) {
    expire_timers(base, heads + levels);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn __run_timer_base(base: *mut timer_base) {
// Can race against a remote CPU updating next_expiry under the lock
    if (time_before(jiffies, READ_ONCE(base.next_expiry))) {
    return;
    }
    timer_base_lock_expiry(base);
    raw_spin_lock_irq(&base.lock);
    __run_timers(base);
    raw_spin_unlock_irq(&base.lock);
    timer_base_unlock_expiry(base);
    }
#[no_mangle]
unsafe extern "C" fn run_timer_base(index: c_int) {
    let mut base = this_cpu_ptr(&timer_bases[index]);
    __run_timer_base(base);
    }
//
// This function runs timers and the timer-tq in bottom half context.
//
#[no_mangle]
unsafe extern "C" fn run_timer_softirq() -> __latent_entropy void {
    run_timer_base(BASE_LOCAL);
    if (IS_ENABLED!(CONFIG_NO_HZ_COMMON)) {
    run_timer_base(BASE_GLOBAL);
    run_timer_base(BASE_DEF);
    if (is_timers_nohz_active()) {
    tmigr_handle_remote();
    }
    }
    }
//
// Called by the local, per-CPU timer interrupt on SMP.
//
#[no_mangle]
unsafe extern "C" fn run_local_timers() {
    let mut base = this_cpu_ptr(&timer_bases[BASE_LOCAL]);
    hrtimer_run_queues();
    while (i < NR_BASES) {
//
// Raise the softirq only if required.
//
// timer_base::next_expiry can be written by a remote CPU while
// holding the lock. If this write happens at the same time than
// the lockless local read, sanity checker could complain about
// data corruption.
//
// There are two possible situations where
// timer_base::next_expiry is written by a remote CPU:
//
// 1. Remote CPU expires global timers of this CPU and updates
// timer_base::next_expiry of BASE_GLOBAL afterwards in
// next_timer_interrupt() or timer_recalc_next_expiry(). The
// worst outcome is a superfluous raise of the timer softirq
// when the not yet updated value is read.
//
// 2. A new first pinned timer is enqueued by a remote CPU
// and therefore timer_base::next_expiry of BASE_LOCAL is
// updated. When this update is missed, this isn't a
// problem, as an IPI is executed nevertheless when the CPU
// was idle before. When the CPU wasn't idle but the update
// is missed, then the timer would expire one jiffy late -
// bad luck.
//
// Those unlikely corner cases where the worst outcome is only a
// one jiffy delay or a superfluous raise of the softirq are
// not that expensive as doing the check always while holding
// the lock.
//
// Possible remote writers are using WRITE_ONCE(). Local reader
// uses therefore READ_ONCE().
//
    if (time_after_eq(jiffies, READ_ONCE(base.next_expiry)) ||
    (i == BASE_DEF && tmigr_requires_handle_remote())) {
    raise_timer_softirq(TIMER_SOFTIRQ);
    return;
    }
    }
    }
//
// Called from the timer interrupt handler to charge one tick to the current
// process.  user_tick is 1 if the tick is user time, 0 for system.
//
#[no_mangle]
pub unsafe extern "C" fn update_process_times(user_tick: c_int) {
    let mut p = current;
// Note: this timer irq context must be accounted for as well.
    account_process_tick(p, user_tick);
    run_local_timers();
    rcu_sched_clock_irq(user_tick);

    if (in_hardirq()) {
    irq_work_tick();
    }

    sched_tick();
    if (IS_ENABLED!(CONFIG_POSIX_TIMERS)) {
    run_posix_cpu_timers();
    }
    }

#[no_mangle]
unsafe extern "C" fn migrate_timer_list(new_base: *mut timer_base, head: *mut hlist_head) {
pub static mut timer: *mut c_void = core::ptr::null_mut();
pub static mut cpu: c_int = 0;
    while (!hlist_empty(head)) {
    timer = hlist_entry(head.first, timer_list, entry);
    detach_timer(timer, false);
    timer.flags = (timer.flags & ~TIMER_BASEMASK) | cpu;
    debug_timer_activate(timer);
// forward_decl: ernal_add_timer;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn timers_prepare_cpu(cpu: c_uint) -> c_int {
pub static mut base: *mut c_void = core::ptr::null_mut();
    let mut b = 0;
    while (b < NR_BASES) {
    base = per_cpu_ptr(&timer_bases[b], cpu);
    base.clk = jiffies;
    base.next_expiry = base.clk + TIMER_NEXT_MAX_DELTA;
    base.next_expiry_recalc = false;
    base.timers_pending = false;
    base.is_idle = false;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn timers_dead_cpu(cpu: c_uint) -> c_int {
pub static mut old_base: *mut c_void = core::ptr::null_mut();
pub static mut new_base: *mut c_void = core::ptr::null_mut();
    let mut b = 0;
    let mut i = 0;
    while (b < NR_BASES) {
    old_base = per_cpu_ptr(&timer_bases[b], cpu);
    new_base = get_cpu_ptr(&timer_bases[b]);
//
// The caller is globally serialized and nobody else
// takes two locks at once, deadlock is not possible.
//
    raw_spin_lock_irq(&new_base.lock);
    raw_spin_lock_nested(&old_base.lock, SINGLE_DEPTH_NESTING);
//
// The current CPUs base clock might be stale. Update it
// before moving the timers over.
//
    forward_timer_base(new_base);
    WARN_ON_ONCE!(old_base.running_timer);
    old_base.running_timer = core::ptr::null_mut();
    for (i = 0; i < WHEEL_SIZE; i++) {
    migrate_timer_list(new_base, old_base.vectors + i);
    }
    raw_spin_unlock(&old_base.lock);
    raw_spin_unlock_irq(&new_base.lock);
    put_cpu_ptr(&timer_bases);
    }
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn init_timer_cpu(cpu: c_int)  {
pub static mut base: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    while (i < NR_BASES) {
    base = per_cpu_ptr(&timer_bases[i], cpu);
    base.cpu = cpu;
    raw_spin_lock_init(&base.lock);
    base.clk = jiffies;
    base.next_expiry = base.clk + TIMER_NEXT_MAX_DELTA;
    timer_base_init_expiry_lock(base);
    }
    }
#[no_mangle]
unsafe extern "C" fn init_timer_cpus()  {
    let mut cpu = 0;
    for_each_possible_cpu(cpu) {
    init_timer_cpu(cpu);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn timers_init()  {
    init_timer_cpus();
    posix_cputimers_init_work();
    open_softirq(TIMER_SOFTIRQ, run_timer_softirq);
    }