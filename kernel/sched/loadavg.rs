//! Automatically rewritten from C to Rust
//! Source: kernel/sched/loadavg.c
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
// kernel/sched/loadavg.c
//
// This file contains the magic bits required to compute the global loadavg
// figure. Its a silly number but people think its important. We go through
// great pains to make it work on big machines and tickless kernels.
//

//
// Global load-average calculations
//
// We take a distributed and async approach to calculating the global load-avg
// in order to minimize overhead.
//
// The global load average is an exponentially decaying average of nr_running +
// nr_uninterruptible.
//
// Once every LOAD_FREQ:
//
// nr_active = 0;
// for_each_possible_cpu(cpu)
// nr_active += cpu_of(cpu)->nr_running + cpu_of(cpu)->nr_uninterruptible;
//
// avenrun[n] = avenrun[0] * exp_n + nr_active * (1 - exp_n)
//
// Due to a number of reasons the above turns in the mess below:
//
// - for_each_possible_cpu() is prohibitively expensive on machines with
// serious number of CPUs, therefore we need to take a distributed approach
// to calculating nr_active.
//
// \Sum_i x_i(t) = \Sum_i x_i(t) - x_i(t_0) | x_i(t_0) := 0
// = \Sum_i { \Sum_j=1 x_i(t_j) - x_i(t_j-1) }
//
// So assuming nr_active := 0 when we start out -- true per definition, we
// can simply take per-CPU deltas and fold those into a global accumulate
// to obtain the same result. See calc_load_fold_active().
//
// Furthermore, in order to avoid synchronizing all per-CPU delta folding
// across the machine, we assume 10 ticks is sufficient time for every
// CPU to have completed this task.
//
// This places an upper-bound on the IRQ-off latency of the machine. Then
// again, being late doesn't loose the delta, just wrecks the sample.
//
// - cpu_rq()->nr_uninterruptible isn't accurately tracked per-CPU because
// this would add another cross-CPU cache-line miss and atomic operation
// to the wakeup path. Instead we increment on whatever CPU the task ran
// when it went into uninterruptible state and decrement on whatever CPU
// did the wakeup. This means that only the sum of nr_uninterruptible over
// all CPUs yields the correct result.
//
// This covers the NO_HZ=n code, for extra head-aches, see the comment below.
//
// Variables and functions for calc_load
    let mut calc_load_tasks;
    let mut calc_load_update = 0;
    unsigned long avenrun[3];
    EXPORT_SYMBOL(avenrun); /* should be removed */
//
// get_avenrun - get the load average array
// @loads:	pointer to destination load array
// @offset:	offset to add
// @shift:	shift count to shift the result left
//
// These values are estimates at best, so no need for locking.
//
#[no_mangle]
pub unsafe extern "C" fn get_avenrun(loads: *mut c_ulong, offset: c_ulong, shift: c_int) {
    loads[0] = (avenrun[0] + offset) << shift;
    loads[1] = (avenrun[1] + offset) << shift;
    loads[2] = (avenrun[2] + offset) << shift;
    }
#[no_mangle]
pub unsafe extern "C" fn calc_load_fold_active(this_rq: *mut rq, adjust: c_long) -> c_long {
    long nr_active, delta = 0;
    nr_active = this_rq.nr_running - adjust;
    nr_active += (long)this_rq.nr_uninterruptible;
    if (nr_active != this_rq.calc_load_active) {
    delta = nr_active - this_rq.calc_load_active;
    this_rq.calc_load_active = nr_active;
    }
    return delta;
    }
//
// fixed_power_int - compute: x^n, in O(log n) time
//
// @x:         base of the power
// @frac_bits: fractional bits of @x
// @n:         power to raise @x to.
//
// By exploiting the relation between the definition of the natural power
// function: x^n := x*x*...*x (x multiplied by itself for n times), and
// the binary encoding of numbers used by computers: n := \Sum n_i * 2^i,
// (where: n_i \elem {0, 1}, the binary vector representing n),
// we find: x^n := x^(\Sum n_i * 2^i) := \Prod x^(n_i * 2^i), which is
// of course trivially computable in O(log_2 n), the length of our binary
// vector.
//
#[no_mangle]
pub unsafe extern "C" fn fixed_power_int(x: c_ulong, frac_bits: c_uint, n: c_uint) -> c_ulong {
pub static mut result: c_ulong = 0;
    if (n) {
    for (;;) {
    if (n & 1) {
    result *= x;
    result += 1UL << (frac_bits - 1);
    result >>= frac_bits;
    }
    n >>= 1;
    if (!n) {
    break;
    }
    x *= x;
    x += 1UL << (frac_bits - 1);
    x >>= frac_bits;
    }
    }
    return result;
    }
//
// a1 = a0 * e + a * (1 - e)
//
// a2 = a1 * e + a * (1 - e)
// = (a0 * e + a * (1 - e)) * e + a * (1 - e)
// = a0 * e^2 + a * (1 - e) * (1 + e)
//
// a3 = a2 * e + a * (1 - e)
// = (a0 * e^2 + a * (1 - e) * (1 + e)) * e + a * (1 - e)
// = a0 * e^3 + a * (1 - e) * (1 + e + e^2)
//
// ...
//
// an = a0 * e^n + a * (1 - e) * (1 + e + ... + e^n-1) [1]
// = a0 * e^n + a * (1 - e) * (1 - e^n)/(1 - e)
// = a0 * e^n + a * (1 - e^n)
//
// [1] application of the geometric series:
//
// n         1 - x^(n+1)
// S_n := \Sum x^i = -------------
// i=0          1 - x
//
#[no_mangle]
pub unsafe extern "C" fn calc_load_n(load: c_ulong, exp: c_ulong, active: c_ulong, n: c_uint) -> c_ulong {
    return calc_load(load, fixed_power_int(exp, FSHIFT, n), active);
    }

//
// Handle NO_HZ for the global load-average.
//
// Since the above described distributed algorithm to compute the global
// load-average relies on per-CPU sampling from the tick, it is affected by
// NO_HZ.
//
// The basic idea is to fold the nr_active delta into a global NO_HZ-delta upon
// entering NO_HZ state such that we can include this as an 'extra' CPU delta
// when we read the global state.
//
// Obviously reality has to ruin such a delightfully simple scheme:
//
// - When we go NO_HZ idle during the window, we can negate our sample
// contribution, causing under-accounting.
//
// We avoid this by keeping two NO_HZ-delta counters and flipping them
// when the window starts, thus separating old and new NO_HZ load.
//
// The only trick is the slight shift in index flip for read vs write.
//
// 0s            5s            10s           15s
// +10           +10           +10           +10
// |-|-----------|-|-----------|-|-----------|-|
// r:0 0 1           1 0           0 1           1 0
// w:0 1 1           0 0           1 1           0 0
//
// This ensures we'll fold the old NO_HZ contribution in this window while
// accumulating the new one.
//
// - When we wake up from NO_HZ during the window, we push up our
// contribution, since we effectively move our sample point to a known
// busy state.
//
// This is solved by pushing the window forward, and thus skipping the
// sample, for this CPU (effectively using the NO_HZ-delta for this CPU which
// was in effect at the time the window opened). This also solves the issue
// of having to deal with a CPU having been in NO_HZ for multiple LOAD_FREQ
// intervals.
//
// When making the ILB scale, we should try to pull this in as well.
//
    static atomic_long_t calc_load_nohz[2];
    static int calc_load_idx;
#[no_mangle]
pub unsafe extern "C" fn calc_load_write_idx() -> c_int {
pub static mut idx: c_int = 0;
//
// See calc_global_nohz(), if we observe the new index, we also
// need to observe the new update time.
//
    smp_rmb();
//
// If the folding window started, make sure we start writing in the
// next NO_HZ-delta.
//
    if (!time_before(jiffies, READ_ONCE(calc_load_update))) {
    idx += 1;
    }
    return idx & 1;
    }
#[no_mangle]
pub unsafe extern "C" fn calc_load_read_idx() -> c_int {
    return calc_load_idx & 1;
    }
#[no_mangle]
unsafe extern "C" fn calc_load_nohz_fold(rq: *mut rq) {
    let mut delta = 0;
    delta = calc_load_fold_active(rq, 0);
    if (delta) {
pub static mut idx: c_int = 0;
    atomic_long_add(delta, &calc_load_nohz[idx]);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn calc_load_nohz_start() {
//
// We're going into NO_HZ mode, if there's any pending delta, fold it
// into the pending NO_HZ delta.
//
    calc_load_nohz_fold(this_rq());
    }
//
// Keep track of the load for NOHZ_FULL, must be called between
// calc_load_nohz_{start,stop}().
//
#[no_mangle]
pub unsafe extern "C" fn calc_load_nohz_remote(rq: *mut rq) {
    calc_load_nohz_fold(rq);
    }
#[no_mangle]
pub unsafe extern "C" fn calc_load_nohz_stop() {
    let mut this_rq = this_rq();
//
// If we're still before the pending sample window, we're done.
//
    this_rq.calc_load_update = READ_ONCE(calc_load_update);
    if (time_before(jiffies, this_rq.calc_load_update)) {
    return;
    }
//
// We woke inside or after the sample window, this means we're already
// accounted through the nohz accounting, so skip the entire deal and
// sync up for the next window.
//
    if (time_before(jiffies, this_rq.calc_load_update + 10)) {
    this_rq.calc_load_update += LOAD_FREQ;
    }
    }
#[no_mangle]
unsafe extern "C" fn calc_load_nohz_read() -> c_long {
pub static mut idx: c_int = 0;
pub static mut delta: c_long = 0;
    if (atomic_long_read(&calc_load_nohz[idx])) {
    delta = atomic_long_xchg(&calc_load_nohz[idx], 0);
    }
    return delta;
    }
//
// NO_HZ can leave us missing all per-CPU ticks calling
// calc_load_fold_active(), but since a NO_HZ CPU folds its delta into
// calc_load_nohz per calc_load_nohz_start(), all we need to do is fold
// in the pending NO_HZ delta if our NO_HZ period crossed a load cycle boundary.
//
// Once we've updated the global active value, we need to apply the exponential
// weights adjusted to the number of cycles missed.
//
#[no_mangle]
unsafe extern "C" fn calc_global_nohz() {
    let mut sample_window = 0;
    let mut delta = 0;
    let mut active = 0;
    let mut n = 0;
    sample_window = READ_ONCE(calc_load_update);
    if (!time_before(jiffies, sample_window + 10)) {
//
// Catch-up, fold however many we are behind still
//
    delta = jiffies - sample_window - 10;
    n = 1 + (delta / LOAD_FREQ);
    active = atomic_long_read(&calc_load_tasks);
    active = active > 0 ? active * FIXED_1 : 0;
    avenrun[0] = calc_load_n(avenrun[0], EXP_1, active, n);
    avenrun[1] = calc_load_n(avenrun[1], EXP_5, active, n);
    avenrun[2] = calc_load_n(avenrun[2], EXP_15, active, n);
    WRITE_ONCE(calc_load_update, sample_window + n * LOAD_FREQ);
    }
//
// Flip the NO_HZ index...
//
// Make sure we first write the new time then flip the index, so that
// calc_load_write_idx() will see the new time when it reads the new
// index, this avoids a double flip messing things up.
//
    smp_wmb();
    calc_load_idx += 1;
    }

#[no_mangle]
pub unsafe extern "C" fn calc_load_nohz_read() -> c_long { return 0; }
#[no_mangle]
pub unsafe extern "C" fn calc_global_nohz() { }

//
// calc_load - update the avenrun load estimates 10 ticks after the
// CPUs have updated calc_load_tasks.
//
// Called from the global timer code.
//
#[no_mangle]
pub unsafe extern "C" fn calc_global_load() {
    let mut sample_window = 0;
    let mut active = 0;
    let mut delta = 0;
    sample_window = READ_ONCE(calc_load_update);
    if (time_before(jiffies, sample_window + 10)) {
    return;
    }
//
// Fold the 'old' NO_HZ-delta to include all NO_HZ CPUs.
//
    delta = calc_load_nohz_read();
    if (delta) {
    atomic_long_add(delta, &calc_load_tasks);
    }
    active = atomic_long_read(&calc_load_tasks);
    active = active > 0 ? active * FIXED_1 : 0;
    avenrun[0] = calc_load(avenrun[0], EXP_1, active);
    avenrun[1] = calc_load(avenrun[1], EXP_5, active);
    avenrun[2] = calc_load(avenrun[2], EXP_15, active);
    WRITE_ONCE(calc_load_update, sample_window + LOAD_FREQ);
//
// In case we went to NO_HZ for multiple LOAD_FREQ intervals
// catch up in bulk.
//
    calc_global_nohz();
    }
//
// Called from sched_tick() to periodically update this CPU's
// active count.
//
#[no_mangle]
pub unsafe extern "C" fn calc_global_load_tick(this_rq: *mut rq) {
    let mut delta = 0;
    if (time_before(jiffies, this_rq.calc_load_update)) {
    return;
    }
    delta  = calc_load_fold_active(this_rq, 0);
    if (delta) {
    atomic_long_add(delta, &calc_load_tasks);
    }
    this_rq.calc_load_update += LOAD_FREQ;
    }