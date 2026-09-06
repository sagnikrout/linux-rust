//! Automatically rewritten from C to Rust
//! Source: kernel/sched/clock.c
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
// sched_clock() for unstable CPU clocks
//
// Copyright (C) 2008 Red Hat, Inc., Peter Zijlstra
//
// Updates and enhancements:
// Copyright (C) 2008 Red Hat, Inc. Steven Rostedt <srostedt@redhat.com>
//
// Based on code by:
// Ingo Molnar <mingo@redhat.com>
// Guillaume Chazarain <guichaz@gmail.com>
//
// What this file implements:
//
// cpu_clock(i) provides a fast (execution time) high resolution
// clock with bounded drift between CPUs. The value of cpu_clock(i)
// is monotonic for constant i. The timestamp returned is in nanoseconds.
//
// ######################### BIG FAT WARNING ##########################
// # when comparing cpu_clock(i) to cpu_clock(j) for i != j, time can #
// # go backwards !!                                                  #
// ####################################################################
//
// There is no strict promise about the base, although it tends to start
// at 0 on boot (but people really shouldn't rely on that).
//
// cpu_clock(i)       -- can be used from any context, including NMI.
// local_clock()      -- is cpu_clock() on the current CPU.
//
// sched_clock_cpu(i)
//
// How it is implemented:
//
// The implementation either uses sched_clock() when
// !CONFIG_HAVE_UNSTABLE_SCHED_CLOCK, which means in that case the
// sched_clock() is assumed to provide these properties (mostly it means
// the architecture provides a globally synchronized highres time source).
//
// Otherwise it tries to create a semi stable clock from a mixture of other
// clocks, including:
//
// - GTOD (clock monotonic)
// - sched_clock()
// - explicit idle events
//
// We use GTOD as base and use sched_clock() deltas to improve resolution. The
// deltas are filtered to provide monotonicity and keeping it within an
// expected window.
//
// Furthermore, explicit sleep and wakeup hooks allow us to account for time
// that is otherwise invisible (TSC gets stopped).
//

//
// Scheduler clock - returns current time in nanosec units.
// This is default implementation.
// Architectures and sub-architectures can override this.
//
#[no_mangle]
pub unsafe extern "C" fn sched_clock() -> notrace unsigned long long __weak {
    return (unsigned long long)(jiffies - INITIAL_JIFFIES)
// (NSEC_PER_SEC / HZ);
    }
    EXPORT_SYMBOL_GPL(sched_clock);
pub static mut sched_clock_running: usize = 0;

//
// We must start with !__sched_clock_stable because the unstable -> stable
// transition is accurate, while the stable -> unstable transition is not.
//
// Similarly we start with __sched_clock_stable_early, thereby assuming we
// will become stable, such that there's only a single 1 -> 0 transition.
//
pub static mut __sched_clock_stable: usize = 0;
pub static mut __sched_clock_stable_early: int = 1;
//
// We want: ktime_get_ns() + __gtod_offset == sched_clock() + __sched_clock_offset
//
    let mut __sched_clock_offset = 0;
    static  u64 __gtod_offset;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sched_clock_data {
    pub tick_raw: u64,
    pub tick_gtod: u64,
    pub clock: u64,
}

pub static mut struct sched_clock_data: usize = 0;
    static __always_inline struct sched_clock_data *this_scd(void)
    {
    return this_cpu_ptr(&sched_clock_data);
    }
    notrace static inline struct sched_clock_data *cpu_sdc(int cpu)
    {
    return &per_cpu(sched_clock_data, cpu);
    }
#[no_mangle]
pub unsafe extern "C" fn sched_clock_stable() -> notrace int {
    return static_branch_likely(&__sched_clock_stable);
    }
#[no_mangle]
pub unsafe extern "C" fn __scd_stamp(scd: *mut sched_clock_data) -> notrace static void {
    scd.tick_gtod = ktime_get_ns();
    scd.tick_raw = sched_clock();
    }
#[no_mangle]
pub unsafe extern "C" fn __set_sched_clock_stable() -> notrace static void {
pub static mut scd: *mut c_void = core::ptr::null_mut();
//
// Since we're still unstable and the tick is already running, we have
// to disable IRQs in order to get a consistent scd->tick* reading.
//
    local_irq_disable();
    scd = this_scd();
//
// Attempt to make the (initial) unstable->stable transition continuous.
//
    __sched_clock_offset = (scd.tick_gtod + __gtod_offset) - (scd.tick_raw);
    local_irq_enable();
    printk("sched_clock: Marking stable (%lld, %lld).(%lld, %lld)\n",
    scd.tick_gtod, __gtod_offset,
    scd.tick_raw,  __sched_clock_offset);
    static_branch_enable(&__sched_clock_stable);
    tick_dep_clear(TICK_DEP_BIT_CLOCK_UNSTABLE);
    }
//
// If we ever get here, we're screwed, because we found out -- typically after
// the fact -- that TSC wasn't good. This means all our clocksources (including
// ktime) could have reported wrong values.
//
// What we do here is an attempt to fix up and continue sort of where we left
// off in a coherent manner.
//
// The only way to fully avoid random clock jumps is to boot with:
// "tsc=unstable".
//
#[no_mangle]
pub unsafe extern "C" fn __sched_clock_work(work: *mut work_struct) -> notrace static void {
pub static mut scd: *mut c_void = core::ptr::null_mut();
    let mut cpu = 0;
// take a current timestamp and set 'now'
    preempt_disable();
    scd = this_scd();
    __scd_stamp(scd);
    scd.clock = scd.tick_gtod + __gtod_offset;
    preempt_enable();
// clone to all CPUs
    for_each_possible_cpu(cpu) {
    per_cpu(sched_clock_data, cpu) = *scd;
    }
    printk("TSC found unstable after boot, most likely due to broken BIOS. Use 'tsc=unstable'.\n");
    printk("sched_clock: Marking unstable (%lld, %lld)<-(%lld, %lld)\n",
    scd.tick_gtod, __gtod_offset,
    scd.tick_raw,  __sched_clock_offset);
    disable_sched_clock_irqtime();
    static_branch_disable(&__sched_clock_stable);
    }
pub static mut sched_clock_work: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn __clear_sched_clock_stable() -> notrace static void {
    if (!sched_clock_stable()) {
    return;
    }
    tick_dep_set(TICK_DEP_BIT_CLOCK_UNSTABLE);
    schedule_work(&sched_clock_work);
    }
#[no_mangle]
pub unsafe extern "C" fn clear_sched_clock_stable() -> notrace void {
    __sched_clock_stable_early = 0;
    smp_mb(); /* matches sched_clock_init_late() */
    if (static_key_count(&sched_clock_running.key) == 2) {
    __clear_sched_clock_stable();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn __sched_clock_gtod_offset() -> notrace static void {
    let mut scd = this_scd();
    __scd_stamp(scd);
    __gtod_offset = (scd.tick_raw + __sched_clock_offset) - scd.tick_gtod;
    }
#[no_mangle]
pub unsafe extern "C" fn sched_clock_init()  {
//
// Set __gtod_offset such that once we mark sched_clock_running,
// sched_clock_tick() continues where sched_clock() left off.
//
// Even if TSC is buggered, we're still UP at this point so it
// can't really be out of sync.
//
    local_irq_disable();
    __sched_clock_gtod_offset();
    local_irq_enable();
    static_branch_inc(&sched_clock_running);
    }
//
// We run this as late_initcall!() such that it runs after all built-in drivers,
// notably: acpi_processor and intel_idle, which can mark the TSC as unstable.
//
#[no_mangle]
unsafe extern "C" fn sched_clock_init_late() -> c_int {
    static_branch_inc(&sched_clock_running);
//
// Ensure that it is impossible to not do a static_key update.
//
// Either {set,clear}_sched_clock_stable() must see sched_clock_running
// and do the update, or we must see their __sched_clock_stable_early
// and do the update, or both.
//
    smp_mb(); /* matches {set,clear}_sched_clock_stable() */
    if (__sched_clock_stable_early) {
    __set_sched_clock_stable();
    }
    else {
    disable_sched_clock_irqtime();  /* disable if clock unstable. */
    }
    return 0;
    }
    late_initcall!(sched_clock_init_late);
//
// min, max except they take wrapping into account
//
#[no_mangle]
unsafe extern "C" fn wrap_min(x: u64, y: u64) -> __always_inline u64 {
    return (s64)(x - y) < 0 ? x : y;
    }
#[no_mangle]
unsafe extern "C" fn wrap_max(x: u64, y: u64) -> __always_inline u64 {
    return (s64)(x - y) > 0 ? x : y;
    }
//
// update the percpu scd from the raw @now value
//
// - filter out backward motion
// - use the GTOD tick value to create a window to filter crazy TSC values
//
#[no_mangle]
unsafe extern "C" fn sched_clock_local(scd: *mut sched_clock_data) -> __always_inline u64 {
    u64 now, clock, old_clock, min_clock, max_clock, gtod;
    let mut delta = 0;
// label;
    now = sched_clock_noinstr();
    delta = now - scd.tick_raw;
    if (unlikely(delta < 0)) {
    delta = 0;
    }
    old_clock = scd.clock;
//
// scd->clock = clamp(scd->tick_gtod + delta,
// max(scd->tick_gtod, scd->clock),
// scd->tick_gtod + TICK_NSEC);
//
    gtod = scd.tick_gtod + __gtod_offset;
    clock = gtod + delta;
    min_clock = wrap_max(gtod, old_clock);
    max_clock = wrap_max(old_clock, gtod + TICK_NSEC);
    clock = wrap_max(clock, min_clock);
    clock = wrap_min(clock, max_clock);
    if (!raw_try_cmpxchg64(&scd.clock, &old_clock, clock)) {
// goto;
    }
    return clock;
    }
#[no_mangle]
pub unsafe extern "C" fn local_clock_noinstr() -> noinstr u64 {
    let mut clock = 0;
    if (static_branch_likely(&__sched_clock_stable)) {
    return sched_clock_noinstr() + __sched_clock_offset;
    }
    if (!static_branch_likely(&sched_clock_running)) {
    return sched_clock_noinstr();
    }
    clock = sched_clock_local(this_scd());
    return clock;
    }
#[no_mangle]
pub unsafe extern "C" fn local_clock() -> u64 {
    let mut now = 0;
    preempt_disable_notrace();
    now = local_clock_noinstr();
    preempt_enable_notrace();
    return now;
    }
    EXPORT_SYMBOL_GPL(local_clock);
#[no_mangle]
unsafe extern "C" fn sched_clock_remote(scd: *mut sched_clock_data) -> notrace u64 {
    let mut my_scd = this_scd();
    u64 this_clock, remote_clock;
    u64 *ptr, old_val, val;

// label;
//
// Careful here: The local and the remote clock values need to
// be read out atomic as we need to compare the values and
// then update either the local or the remote side. So the
// cmpxchg64 below only protects one readout.
//
// We must reread via sched_clock_local() in the retry case on
// 32-bit kernels as an NMI could use sched_clock_local() via the
// tracer and hit between the readout of
// the low 32-bit and the high 32-bit portion.
//
    this_clock = sched_clock_local(my_scd);
//
// We must enforce atomic readout on 32-bit, otherwise the
// update on the remote CPU can hit in between the readout of
// the low 32-bit and the high 32-bit portion.
//
    remote_clock = cmpxchg64(&scd.clock, 0, 0);

//
// On 64-bit kernels the read of [my]scd->clock is atomic versus the
// update, so we can avoid the above 32-bit dance.
//
    sched_clock_local(my_scd);
// label;
    this_clock = my_scd.clock;
    remote_clock = scd.clock;

//
// Use the opportunity that we have both locks
// taken to couple the two clocks: we take the
// larger time as the latest time for both
// runqueues. (this creates monotonic movement)
//
    if (likely((s64)(remote_clock - this_clock) < 0)) {
    ptr = &scd.clock;
    old_val = remote_clock;
    val = this_clock;
    } else {
//
// Should be rare, but possible:
//
    ptr = &my_scd.clock;
    old_val = this_clock;
    val = remote_clock;
    }
    if (!try_cmpxchg64(ptr, &old_val, val)) {
// goto;
    }
    return val;
    }
//
// Similar to cpu_clock(), but requires local IRQs to be disabled.
//
// See cpu_clock().
//
#[no_mangle]
pub unsafe extern "C" fn sched_clock_cpu(cpu: c_int) -> notrace u64 {
pub static mut scd: *mut c_void = core::ptr::null_mut();
    let mut clock = 0;
    if (sched_clock_stable()) {
    return sched_clock() + __sched_clock_offset;
    }
    if (!static_branch_likely(&sched_clock_running)) {
    return sched_clock();
    }
    preempt_disable_notrace();
    scd = cpu_sdc(cpu);
    if (cpu != smp_processor_id()) {
    clock = sched_clock_remote(scd);
    }
    else {
    clock = sched_clock_local(scd);
    }
    preempt_enable_notrace();
    return clock;
    }
    EXPORT_SYMBOL_GPL(sched_clock_cpu);
#[no_mangle]
pub unsafe extern "C" fn sched_clock_tick() -> notrace void {
pub static mut scd: *mut c_void = core::ptr::null_mut();
    if (sched_clock_stable()) {
    return;
    }
    if (!static_branch_likely(&sched_clock_running)) {
    return;
    }
    lockdep_assert_irqs_disabled();
    scd = this_scd();
    __scd_stamp(scd);
    sched_clock_local(scd);
    }
#[no_mangle]
pub unsafe extern "C" fn sched_clock_tick_stable() -> notrace void {
    if (!sched_clock_stable()) {
    return;
    }
//
// Called under watchdog_lock.
//
// The watchdog just found this TSC to (still) be stable, so now is a
// good moment to update our __gtod_offset. Because once we find the
// TSC to be unstable, any computation will be computing crap.
//
    local_irq_disable();
    __sched_clock_gtod_offset();
    local_irq_enable();
    }
//
// We are going deep-idle (IRQs are disabled):
//
#[no_mangle]
pub unsafe extern "C" fn sched_clock_idle_sleep_event() -> notrace void {
    sched_clock_cpu(smp_processor_id());
    }
    EXPORT_SYMBOL_GPL(sched_clock_idle_sleep_event);
//
// We just idled; resync with ktime.
//
#[no_mangle]
pub unsafe extern "C" fn sched_clock_idle_wakeup_event() -> notrace void {
    let mut flags = 0;
    if (sched_clock_stable()) {
    return;
    }
    if (unlikely(timekeeping_suspended)) {
    return;
    }
    local_irq_save(flags);
    sched_clock_tick();
    local_irq_restore(flags);
    }
    EXPORT_SYMBOL_GPL(sched_clock_idle_wakeup_event);

#[no_mangle]
#[no_mangle]
// duplicate fn: sched_clock_init
pub unsafe extern "C" fn sched_clock_init_dup()  {
    static_branch_inc(&sched_clock_running);
    local_irq_disable();
    generic_sched_clock_init();
    local_irq_enable();
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: sched_clock_cpu
pub unsafe extern "C" fn sched_clock_cpu_dup(cpu: c_int) -> notrace u64 {
    if (!static_branch_likely(&sched_clock_running)) {
    return 0;
    }
    return sched_clock();
    }

//
// Running clock - returns the time that has elapsed while a guest has been
// running.
// On a guest this value should be local_clock minus the time the guest was
// suspended by the hypervisor (for any reason).
// On bare metal this function should return the same as local_clock.
// Architectures and sub-architectures can override this.
//
#[no_mangle]
pub unsafe extern "C" fn running_clock() -> notrace u64 __weak {
    return local_clock();
    }