//! Automatically rewritten from C to Rust
//! Source: kernel/sched/cpufreq_schedutil.c
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
// CPUFreq governor based on scheduler-provided CPU utilization data.
//
// Copyright (C) 2016, Intel Corporation
// Author: Rafael J. Wysocki <rafael.j.wysocki@intel.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sugov_tunables {
    pub attr_set: gov_attr_set,
    pub rate_limit_us: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sugov_policy {
    pub policy: *mut cpufreq_policy,
    pub tunables: *mut sugov_tunables,
    pub tunables_hook: list_head,
    pub update_lock: raw_spinlock_t,
    pub last_freq_update_time: u64,
    pub freq_update_delay_ns: i64,
    pub next_freq: c_uint,
    pub cached_raw_freq: c_uint,
// The next fields are only needed if fast switch cannot be used:
    pub irq_work: irq_work,
    pub work: kthread_work,
    pub work_lock: mutex,
    pub worker: kthread_worker,
    pub thread: *mut task_struct,
    pub work_in_progress: bool,
    pub limits_changed: bool,
    pub need_freq_update: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sugov_cpu {
    pub update_util: update_util_data,
    pub sg_policy: *mut sugov_policy,
    pub cpu: c_uint,
    pub iowait_boost_pending: bool,
    pub iowait_boost: c_uint,
    pub last_update: u64,
    pub util: c_ulong,
    pub bw_min: c_ulong,
    pub bw_max: c_ulong,
// The field below is for single-CPU policies only:

    pub saved_idle_calls: c_ulong,

}

pub static mut struct sugov_cpu: usize = 0;
// Governor internals
#[no_mangle]
unsafe extern "C" fn sugov_update_rate_limit_us(sg_policy: *mut sugov_policy) {
//
// Cast rate_limit_us before multiplication to force 64-bit arithmetic.
// Otherwise, on 32-bit platforms, both operands are converted to
// 32-bit unsigned long and the multiplication may overflow.
//
    sg_policy.freq_update_delay_ns =
    (s64)sg_policy.tunables.rate_limit_us * NSEC_PER_USEC;
    }
#[no_mangle]
unsafe extern "C" fn sugov_should_update_freq(sg_policy: *mut sugov_policy, time: u64) -> bool {
    let mut delta_ns = 0;
//
// Since cpufreq_update_util() is called with rq->lock held for
// the @target_cpu, our per-CPU data is fully serialized.
//
// However, drivers cannot in general deal with cross-CPU
// requests, so while get_next_freq() will work, our
// sugov_update_commit() call may not for the fast switching platforms.
//
// Hence stop here for remote requests if they aren't supported
// by the hardware, as calculating the frequency is pointless if
// we cannot in fact act on it.
//
// This is needed on the slow switching platforms too to prevent CPUs
// going offline from leaving stale IRQ work items behind.
//
    if (!cpufreq_this_cpu_can_update(sg_policy.policy)) {
    return false;
    }
    if (unlikely(READ_ONCE(sg_policy.limits_changed))) {
    WRITE_ONCE(sg_policy.limits_changed, false);
    sg_policy.need_freq_update = true;
//
// The above limits_changed update must occur before the reads
// of policy limits in cpufreq_driver_resolve_freq() or a policy
// limits update might be missed, so use a memory barrier to
// ensure it.
//
// This pairs with the write memory barrier in sugov_limits().
//
    smp_mb();
    return true;
    } else if (sg_policy.need_freq_update) {
// ignore_dl_rate_limit() wants a new frequency to be found.
    return true;
    }
    delta_ns = time - sg_policy.last_freq_update_time;
    return delta_ns >= sg_policy.freq_update_delay_ns;
    }
#[no_mangle]
pub unsafe extern "C" fn sugov_update_next_freq(sg_policy: *mut sugov_policy, time: u64, next_freq: c_uint) -> bool {
    if (sg_policy.need_freq_update) {
    sg_policy.need_freq_update = false;
//
// The policy limits have changed, but if the return value of
// cpufreq_driver_resolve_freq() after applying the new limits
// is still equal to the previously selected frequency, the
// driver callback need not be invoked unless the driver
// specifically wants that to happen on every update of the
// policy limits.
//
    if (sg_policy.next_freq == next_freq &&
    !cpufreq_driver_test_flags(CPUFREQ_NEED_UPDATE_LIMITS)) {
    return false;
    }
    } else if (sg_policy.next_freq == next_freq) {
    return false;
    }
    sg_policy.next_freq = next_freq;
    sg_policy.last_freq_update_time = time;
    return true;
    }
#[no_mangle]
unsafe extern "C" fn sugov_deferred_update(sg_policy: *mut sugov_policy) {
    if (!sg_policy.work_in_progress) {
    sg_policy.work_in_progress = true;
    irq_work_queue(&sg_policy.irq_work);
    }
    }
//
// get_capacity_ref_freq - get the reference frequency that has been used to
// correlate frequency and compute capacity for a given cpufreq policy. We use
// the CPU managing it for the arch_scale_freq_ref() call in the function.
// @policy: the cpufreq policy of the CPU in question.
//
// Return: the reference CPU frequency to compute a capacity.
//
    static __always_inline
#[no_mangle]
pub unsafe extern "C" fn get_capacity_ref_freq(policy: *mut cpufreq_policy) -> c_ulong {
pub static mut freq: c_uint = 0;
    if (freq) {
    return freq;
    }
    if (arch_scale_freq_invariant()) {
    return policy.cpuinfo.max_freq;
    }
//
// Apply a 25% margin so that we select a higher frequency than
// the current one before the CPU is fully busy:
//
    return policy.cur + (policy.cur >> 2);
    }
//
// get_next_freq - Compute a new frequency for a given cpufreq policy.
// @sg_policy: schedutil policy object to compute the new frequency for.
// @util: Current CPU utilization.
// @max: CPU capacity.
//
// If the utilization is frequency-invariant, choose the new frequency to be
// proportional to it, that is
//
// next_freq = C * max_freq * util / max
//
// Otherwise, approximate the would-be frequency-invariant utilization by
// util_raw * (curr_freq / max_freq) which leads to
//
// next_freq = C * curr_freq * util_raw / max
//
// Take C = 1.25 for the frequency tipping point at (util / max) = 0.8.
//
// The lowest driver-supported frequency which is equal or greater than the raw
// next_freq (as calculated above) is returned, subject to policy min/max and
// cpufreq driver limitations.
//
#[no_mangle]
pub unsafe extern "C" fn get_next_freq(sg_policy: *mut sugov_policy, util: c_ulong, max: c_ulong) -> c_uint {
    let mut policy = sg_policy.policy;
    let mut freq = 0;
    freq = get_capacity_ref_freq(policy);
    freq = map_util_freq(util, freq, max);
    if (freq == sg_policy.cached_raw_freq && !sg_policy.need_freq_update) {
    return sg_policy.next_freq;
    }
    sg_policy.cached_raw_freq = freq;
    return cpufreq_driver_resolve_freq(policy, freq);
    }
#[no_mangle]
pub unsafe extern "C" fn sugov_effective_cpu_perf(cpu: c_int, actual: c_ulong, min: c_ulong, max: c_ulong) -> c_ulong {
// Add dvfs headroom to actual utilization
    actual = map_util_perf(actual);
// Actually we don't need to target the max performance
    if (actual < max) {
    max = actual;
    }
//
// Ensure at least minimum performance while providing more compute
// capacity when possible.
//
    return max(min, max);
    }
#[no_mangle]
unsafe extern "C" fn sugov_get_util(sg_cpu: *mut sugov_cpu, boost: c_ulong) {
    unsigned long min, max, util = scx_cpuperf_target(sg_cpu.cpu);
    if (!scx_switched_all()) {
    util += cpu_util_cfs_boost(sg_cpu.cpu);
    }
    util = effective_cpu_util(sg_cpu.cpu, util, &min, &max);
    util = max(util, boost);
    sg_cpu.bw_min = min;
    sg_cpu.bw_max = max;
    sg_cpu.util = sugov_effective_cpu_perf(sg_cpu.cpu, util, min, max);
    }
//
// sugov_iowait_reset() - Reset the IO boost status of a CPU.
// @sg_cpu: the sugov data for the CPU to boost
// @time: the update time from the caller
// @set_iowait_boost: true if an IO boost has been requested
//
// The IO wait boost of a task is disabled after a tick since the last update
// of a CPU. If a new IO wait boost is requested after more then a tick, then
// we enable the boost starting from IOWAIT_BOOST_MIN, which improves energy
// efficiency by ignoring sporadic wakeups from IO.
//
#[no_mangle]
pub unsafe extern "C" fn sugov_iowait_reset(sg_cpu: *mut sugov_cpu, time: u64, set_iowait_boost: bool) -> bool {
pub static mut delta_ns: i64 = 0;
// Reset boost only if a tick has elapsed since last request
    if (delta_ns <= TICK_NSEC) {
    return false;
    }
    sg_cpu.iowait_boost = set_iowait_boost ? IOWAIT_BOOST_MIN : 0;
    sg_cpu.iowait_boost_pending = set_iowait_boost;
    return true;
    }
//
// sugov_iowait_boost() - Updates the IO boost status of a CPU.
// @sg_cpu: the sugov data for the CPU to boost
// @time: the update time from the caller
// @flags: SCHED_CPUFREQ_IOWAIT if the task is waking up after an IO wait
//
// Each time a task wakes up after an IO operation, the CPU utilization can be
// boosted to a certain utilization which doubles at each "frequent and
// successive" wakeup from IO, ranging from IOWAIT_BOOST_MIN to the utilization
// of the maximum OPP.
//
// To keep doubling, an IO boost has to be requested at least once per tick,
// otherwise we restart from the utilization of the minimum OPP.
//
#[no_mangle]
pub unsafe extern "C" fn sugov_iowait_boost(sg_cpu: *mut sugov_cpu, time: u64, flags: c_uint) {
pub static mut set_iowait_boost: bool = false;
// Reset boost if the CPU appears to have been idle enough
    if (sg_cpu.iowait_boost &&
    sugov_iowait_reset(sg_cpu, time, set_iowait_boost)) {
    return;
    }
// Boost only tasks waking up after IO
    if (!set_iowait_boost) {
    return;
    }
// Ensure boost doubles only one time at each request
    if (sg_cpu.iowait_boost_pending) {
    return;
    }
    sg_cpu.iowait_boost_pending = true;
// Double the boost at each request
    if (sg_cpu.iowait_boost) {
    sg_cpu.iowait_boost =
    min_t(unsigned int, sg_cpu.iowait_boost << 1, SCHED_CAPACITY_SCALE);
    return;
    }
// First wakeup after IO: start with minimum boost
    sg_cpu.iowait_boost = IOWAIT_BOOST_MIN;
    }
//
// sugov_iowait_apply() - Apply the IO boost to a CPU.
// @sg_cpu: the sugov data for the cpu to boost
// @time: the update time from the caller
// @max_cap: the max CPU capacity
//
// A CPU running a task which woken up after an IO operation can have its
// utilization boosted to speed up the completion of those IO operations.
// The IO boost value is increased each time a task wakes up from IO, in
// sugov_iowait_boost(), and it's instead decreased by this function,
// each time an increase has not been requested (!iowait_boost_pending).
//
// A CPU which also appears to have been idle for at least one tick has also
// its IO boost utilization reset.
//
// This mechanism is designed to boost high frequently IO waiting tasks, while
// being more conservative on tasks which does sporadic IO operations.
//
#[no_mangle]
pub unsafe extern "C" fn sugov_iowait_apply(sg_cpu: *mut sugov_cpu, time: u64, max_cap: c_ulong) -> c_ulong {
// No boost currently required
    if (!sg_cpu.iowait_boost) {
    return 0;
    }
// Reset boost if the CPU appears to have been idle enough
    if (sugov_iowait_reset(sg_cpu, time, false)) {
    return 0;
    }
    if (!sg_cpu.iowait_boost_pending) {
//
// No boost pending; reduce the boost value.
//
    sg_cpu.iowait_boost >>= 1;
    if (sg_cpu.iowait_boost < IOWAIT_BOOST_MIN) {
    sg_cpu.iowait_boost = 0;
    return 0;
    }
    }
    sg_cpu.iowait_boost_pending = false;
//
// sg_cpu->util is already in capacity scale; convert iowait_boost
// into the same scale so we can compare.
//
    return (sg_cpu.iowait_boost * max_cap) >> SCHED_CAPACITY_SHIFT;
    }

#[no_mangle]
unsafe extern "C" fn sugov_hold_freq(sg_cpu: *mut sugov_cpu) -> bool {
    let mut idle_calls = 0;
    let mut ret = 0;
//
// The heuristics in this function is for the fair class. For SCX, the
// performance target comes directly from the BPF scheduler. Let's just
// follow it.
//
    if (scx_switched_all()) {
    return false;
    }
// if capped by uclamp_max, always update to be in compliance
    if (uclamp_rq_is_capped(cpu_rq(sg_cpu.cpu))) {
    return false;
    }
//
// Maintain the frequency if the CPU has not been idle recently, as
// reduction is likely to be premature.
//
    idle_calls = tick_nohz_get_idle_calls_cpu(sg_cpu.cpu);
    ret = idle_calls == sg_cpu.saved_idle_calls;
    sg_cpu.saved_idle_calls = idle_calls;
    return ret;
    }

#[no_mangle]
pub unsafe extern "C" fn sugov_hold_freq(sg_cpu: *mut sugov_cpu) -> bool { return false; }

//
// Make sugov_should_update_freq() ignore the rate limit when DL
// has increased the utilization.
//
#[no_mangle]
pub unsafe extern "C" fn ignore_dl_rate_limit(sg_cpu: *mut sugov_cpu) {
    if (cpu_bw_dl(cpu_rq(sg_cpu.cpu)) > sg_cpu.bw_min) {
    sg_cpu.sg_policy.need_freq_update = true;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn sugov_update_single_common(sg_cpu: *mut sugov_cpu, time: u64, max_cap: c_ulong, flags: c_uint) -> bool {
    let mut boost = 0;
    sugov_iowait_boost(sg_cpu, time, flags);
    sg_cpu.last_update = time;
    ignore_dl_rate_limit(sg_cpu);
    if (!sugov_should_update_freq(sg_cpu.sg_policy, time)) {
    return false;
    }
    boost = sugov_iowait_apply(sg_cpu, time, max_cap);
    sugov_get_util(sg_cpu, boost);
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn sugov_update_single_freq(hook: *mut update_util_data, time: u64, flags: c_uint) {
    let mut sg_cpu = container_of!(hook, sugov_cpu, update_util);
    let mut sg_policy = sg_cpu.sg_policy;
pub static mut cached_freq: c_uint = 0;
    let mut max_cap = 0;
    let mut next_f = 0;
    max_cap = arch_scale_cpu_capacity(sg_cpu.cpu);
    if (!sugov_update_single_common(sg_cpu, time, max_cap, flags)) {
    return;
    }
    next_f = get_next_freq(sg_policy, sg_cpu.util, max_cap);
    if (sugov_hold_freq(sg_cpu) && next_f < sg_policy.next_freq &&
    !sg_policy.need_freq_update) {
    next_f = sg_policy.next_freq;
// Restore cached freq as next_freq has changed
    sg_policy.cached_raw_freq = cached_freq;
    }
    if (!sugov_update_next_freq(sg_policy, time, next_f)) {
    return;
    }
//
// This code runs under rq->lock for the target CPU, so it won't run
// concurrently on two different CPUs for the same target and it is not
// necessary to acquire the lock in the fast switch case.
//
    if (sg_policy.policy.fast_switch_enabled) {
    cpufreq_driver_fast_switch(sg_policy.policy, next_f);
    } else {
    raw_spin_lock(&sg_policy.update_lock);
    sugov_deferred_update(sg_policy);
    raw_spin_unlock(&sg_policy.update_lock);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn sugov_update_single_perf(hook: *mut update_util_data, time: u64, flags: c_uint) {
    let mut sg_cpu = container_of!(hook, sugov_cpu, update_util);
    let mut sg_policy = sg_cpu.sg_policy;
pub static mut prev_util: c_ulong = 0;
    let mut max_cap = 0;
//
// Fall back to the "frequency" path if frequency invariance is not
// supported, because the direct mapping between the utilization and
// the performance levels depends on the frequency invariance.
//
    if (!arch_scale_freq_invariant()) {
    sugov_update_single_freq(hook, time, flags);
    return;
    }
    max_cap = arch_scale_cpu_capacity(sg_cpu.cpu);
    if (!sugov_update_single_common(sg_cpu, time, max_cap, flags)) {
    return;
    }
    if (sugov_hold_freq(sg_cpu) && sg_cpu.util < prev_util) {
    sg_cpu.util = prev_util;
    }
    cpufreq_driver_adjust_perf(sg_policy.policy, sg_cpu.bw_min,
    sg_cpu.util, sg_cpu.bw_max, max_cap);
    sg_policy.need_freq_update = false;
    sg_policy.last_freq_update_time = time;
    }
#[no_mangle]
unsafe extern "C" fn sugov_next_freq_shared(sg_cpu: *mut sugov_cpu, time: u64) -> c_uint {
    let mut sg_policy = sg_cpu.sg_policy;
    let mut policy = sg_policy.policy;
pub static mut util: c_ulong = 0;
    let mut j = 0;
    max_cap = arch_scale_cpu_capacity(sg_cpu.cpu);
    for_each_cpu(j, policy.cpus) {
    let mut j_sg_cpu = &per_cpu(sugov_cpu, j);
    let mut boost = 0;
    boost = sugov_iowait_apply(j_sg_cpu, time, max_cap);
    sugov_get_util(j_sg_cpu, boost);
    util = max(j_sg_cpu.util, util);
    }
    return get_next_freq(sg_policy, util, max_cap);
    }
#[no_mangle]
pub unsafe extern "C" fn sugov_update_shared(hook: *mut update_util_data, time: u64, flags: c_uint) {
    let mut sg_cpu = container_of!(hook, sugov_cpu, update_util);
    let mut sg_policy = sg_cpu.sg_policy;
    let mut next_f = 0;
    raw_spin_lock(&sg_policy.update_lock);
    sugov_iowait_boost(sg_cpu, time, flags);
    sg_cpu.last_update = time;
    ignore_dl_rate_limit(sg_cpu);
    if (sugov_should_update_freq(sg_policy, time)) {
    next_f = sugov_next_freq_shared(sg_cpu, time);
    if (!sugov_update_next_freq(sg_policy, time, next_f)) {
// goto;
    }
    if (sg_policy.policy.fast_switch_enabled) {
    cpufreq_driver_fast_switch(sg_policy.policy, next_f);
    }
    else {
    sugov_deferred_update(sg_policy);
    }
    }
// label;
    raw_spin_unlock(&sg_policy.update_lock);
    }
#[no_mangle]
unsafe extern "C" fn sugov_work(work: *mut kthread_work) {
    let mut sg_policy = container_of!(work, sugov_policy, work);
    let mut freq = 0;
    let mut flags = 0;
//
// Hold sg_policy->update_lock shortly to handle the case where:
// in case sg_policy->next_freq is read here, and then updated by
// sugov_deferred_update() just before work_in_progress is set to false
// here, we may miss queueing the new update.
//
// Note: If a work was queued after the update_lock is released,
// sugov_work() will just be called again by kthread_work code; and the
// request will be proceed before the sugov thread sleeps.
//
    raw_spin_lock_irqsave(&sg_policy.update_lock, flags);
    freq = sg_policy.next_freq;
    sg_policy.work_in_progress = false;
    raw_spin_unlock_irqrestore(&sg_policy.update_lock, flags);
    mutex_lock(&sg_policy.work_lock);
    __cpufreq_driver_target(sg_policy.policy, freq, CPUFREQ_RELATION_L);
    mutex_unlock(&sg_policy.work_lock);
    }
#[no_mangle]
unsafe extern "C" fn sugov_irq_work(irq_work: *mut irq_work) {
pub static mut sg_policy: *mut c_void = core::ptr::null_mut();
    sg_policy = container_of!(irq_work, sugov_policy, irq_work);
    kthread_queue_work(&sg_policy.worker, &sg_policy.work);
    }
// sysfs interface
pub static mut global_tunables: *mut c_void = core::ptr::null_mut();
pub static mut global_tunables_lock: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn to_sugov_tunables(attr_set: *mut gov_attr_set) -> *mut c_void {
    return container_of!(attr_set, sugov_tunables, attr_set);
    }
#[no_mangle]
unsafe extern "C" fn rate_limit_us_show(attr_set: *mut gov_attr_set, buf: *mut c_char) -> isize {
    let mut tunables = to_sugov_tunables(attr_set);
    return sysfs_emit(buf, "%u\n", tunables.rate_limit_us);
    }
#[no_mangle]
pub unsafe extern "C" fn rate_limit_us_store(attr_set: *mut gov_attr_set, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut tunables = to_sugov_tunables(attr_set);
pub static mut sg_policy: *mut c_void = core::ptr::null_mut();
    let mut rate_limit_us = 0;
    if (kstrtouint(buf, 10, &rate_limit_us)) {
    return -EINVAL;
    }
    tunables.rate_limit_us = rate_limit_us;
    list_for_each_entry(sg_policy, &attr_set.policy_list, tunables_hook) {
    sugov_update_rate_limit_us(sg_policy);
    }
    return count;
    }
pub static mut rate_limit_us: governor_attr = 0;
    static struct attribute *sugov_attrs[] = {
    &rate_limit_us.attr,
    core::ptr::null_mut()
    };
    ATTRIBUTE_GROUPS(sugov);
#[no_mangle]
unsafe extern "C" fn sugov_tunables_free(kobj: *mut kobject) {
    let mut attr_set = to_gov_attr_set(kobj);
    kfree(to_sugov_tunables(attr_set));
    }
pub static mut kobj_type: usize = 0;
// cpufreq governor interface
pub static mut schedutil_gov: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn sugov_policy_alloc(policy: *mut cpufreq_policy) -> *mut c_void {
pub static mut sg_policy: *mut c_void = core::ptr::null_mut();
    sg_policy = kzalloc_obj(*sg_policy);
    if (!sg_policy) {
    return core::ptr::null_mut();
    }
    sg_policy.policy = policy;
    raw_spin_lock_init(&sg_policy.update_lock);
    return sg_policy;
    }
#[no_mangle]
unsafe extern "C" fn sugov_policy_free(sg_policy: *mut sugov_policy) {
    kfree(sg_policy);
    }
#[no_mangle]
unsafe extern "C" fn sugov_kthread_create(sg_policy: *mut sugov_policy) -> c_int {
pub static mut thread: *mut c_void = core::ptr::null_mut();
pub static mut sched_attr: usize = 0;
    let mut policy = sg_policy.policy;
    let mut ret = 0;
// kthread only required for slow path
    if (policy.fast_switch_enabled) {
    return 0;
    }
    kthread_init_work(&sg_policy.work, sugov_work);
    kthread_init_worker(&sg_policy.worker);
    thread = kthread_create(kthread_worker_fn, &sg_policy.worker,
    "sugov:%d",
    cpumask_first(policy.related_cpus));
    if (IS_ERR(thread)) {
    pr_err!("failed to create sugov thread: %pe\n", thread);
    return PTR_ERR(thread);
    }
    ret = sched_setattr_nocheck(thread, &attr);
    if (ret) {
    kthread_stop(thread);
    pr_warn!("%s: failed to set SCHED_DEADLINE\n", __func__);
    return ret;
    }
    sg_policy.thread = thread;
    if (policy.dvfs_possible_from_any_cpu) {
    set_cpus_allowed_ptr(thread, policy.related_cpus);
    }
    else {
    kthread_bind_mask(thread, policy.related_cpus);
    }
    init_irq_work(&sg_policy.irq_work, sugov_irq_work);
    mutex_init(&sg_policy.work_lock);
    wake_up_process(thread);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sugov_kthread_stop(sg_policy: *mut sugov_policy) {
// kthread only required for slow path
    if (sg_policy.policy.fast_switch_enabled) {
    return;
    }
    kthread_flush_worker(&sg_policy.worker);
    kthread_stop(sg_policy.thread);
    mutex_destroy(&sg_policy.work_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn sugov_tunables_alloc(sg_policy: *mut sugov_policy) -> *mut c_void {
pub static mut tunables: *mut c_void = core::ptr::null_mut();
    tunables = kzalloc_obj(*tunables);
    if (tunables) {
    gov_attr_set_init(&tunables.attr_set, &sg_policy.tunables_hook);
    if (!have_governor_per_policy()) {
    global_tunables = tunables;
    }
    }
    return tunables;
    }
#[no_mangle]
unsafe extern "C" fn sugov_clear_global_tunables() {
    if (!have_governor_per_policy()) {
    global_tunables = core::ptr::null_mut();
    }
    }
#[no_mangle]
unsafe extern "C" fn sugov_init(policy: *mut cpufreq_policy) -> c_int {
pub static mut sg_policy: *mut c_void = core::ptr::null_mut();
pub static mut tunables: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
// State should be equivalent to EXIT
    if (policy.governor_data) {
    return -EBUSY;
    }
    cpufreq_enable_fast_switch(policy);
    sg_policy = sugov_policy_alloc(policy);
    if (!sg_policy) {
    ret = -ENOMEM;
// goto;
    }
    ret = sugov_kthread_create(sg_policy);
    if (ret) {
// goto;
    }
    mutex_lock(&global_tunables_lock);
    if (global_tunables) {
    if (WARN_ON!(have_governor_per_policy())) {
    ret = -EINVAL;
// goto;
    }
    policy.governor_data = sg_policy;
    sg_policy.tunables = global_tunables;
    gov_attr_set_get(&global_tunables.attr_set, &sg_policy.tunables_hook);
// goto;
    }
    tunables = sugov_tunables_alloc(sg_policy);
    if (!tunables) {
    ret = -ENOMEM;
// goto;
    }
    tunables.rate_limit_us = cpufreq_policy_transition_delay_us(policy);
    policy.governor_data = sg_policy;
    sg_policy.tunables = tunables;
    ret = kobject_init_and_add(&tunables.attr_set.kobj, &sugov_tunables_ktype,
    get_governor_parent_kobj(policy), "%s",
    schedutil_gov.name);
    if (ret) {
// goto;
    }
// label;
//
// Schedutil is the preferred governor for EAS, so rebuild sched domains
// on governor changes to make sure the scheduler knows about them.
//
    em_rebuild_sched_domains();
    mutex_unlock(&global_tunables_lock);
    return 0;
// label;
    kobject_put(&tunables.attr_set.kobj);
    policy.governor_data = core::ptr::null_mut();
    sugov_clear_global_tunables();
// label;
    sugov_kthread_stop(sg_policy);
    mutex_unlock(&global_tunables_lock);
// label;
    sugov_policy_free(sg_policy);
// label;
    cpufreq_disable_fast_switch(policy);
    pr_err!("initialization failed (error %d)\n", ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sugov_exit(policy: *mut cpufreq_policy) {
    let mut sg_policy = policy.governor_data;
    let mut tunables = sg_policy.tunables;
    let mut count = 0;
    mutex_lock(&global_tunables_lock);
    count = gov_attr_set_put(&tunables.attr_set, &sg_policy.tunables_hook);
    policy.governor_data = core::ptr::null_mut();
    if (!count) {
    sugov_clear_global_tunables();
    }
    mutex_unlock(&global_tunables_lock);
    sugov_kthread_stop(sg_policy);
    sugov_policy_free(sg_policy);
    cpufreq_disable_fast_switch(policy);
    em_rebuild_sched_domains();
    }
#[no_mangle]
unsafe extern "C" fn sugov_start(policy: *mut cpufreq_policy) -> c_int {
    let mut sg_policy = policy.governor_data;
    void (*uu)(update_util_data *data, u64 time, unsigned int flags);
    let mut cpu = 0;
    sugov_update_rate_limit_us(sg_policy);
    sg_policy.last_freq_update_time	= 0;
    sg_policy.next_freq			= 0;
    sg_policy.work_in_progress		= false;
    sg_policy.limits_changed		= false;
    sg_policy.cached_raw_freq		= 0;
    sg_policy.need_freq_update = cpufreq_driver_test_flags(CPUFREQ_NEED_UPDATE_LIMITS);
    if (policy_is_shared(policy)) {
    uu = sugov_update_shared;
    }

    else if (policy.fast_switch_enabled && cpufreq_driver_has_adjust_perf()) {
    uu = sugov_update_single_perf;
    }
    else {
    uu = sugov_update_single_freq;
    }
    for_each_cpu(cpu, policy.cpus) {
    let mut sg_cpu = &per_cpu(sugov_cpu, cpu);
    memset(sg_cpu, 0, sizeof!(*sg_cpu));
    sg_cpu.cpu = cpu;
    sg_cpu.sg_policy = sg_policy;
    }
//
// Publish the hooks only after all per-CPU data is initialized, so a
// shared policy's sugov_update_shared() never reads an uninitialized
// sibling sugov_cpu.
//
    for_each_cpu(cpu, policy.cpus) {
    let mut sg_cpu = &per_cpu(sugov_cpu, cpu);
    cpufreq_add_update_util_hook(cpu, &sg_cpu.update_util, uu);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sugov_stop(policy: *mut cpufreq_policy) {
    let mut sg_policy = policy.governor_data;
    let mut cpu = 0;
    for_each_cpu(cpu, policy.cpus) {
    cpufreq_remove_update_util_hook(cpu);
    }
    synchronize_rcu();
    if (!policy.fast_switch_enabled) {
    irq_work_sync(&sg_policy.irq_work);
    kthread_cancel_work_sync(&sg_policy.work);
    }
    }
#[no_mangle]
unsafe extern "C" fn sugov_limits(policy: *mut cpufreq_policy) {
    let mut sg_policy = policy.governor_data;
    if (!policy.fast_switch_enabled) {
    mutex_lock(&sg_policy.work_lock);
    cpufreq_policy_apply_limits(policy);
    mutex_unlock(&sg_policy.work_lock);
    }
//
// The limits_changed update below must take place before the updates
// of policy limits in cpufreq_set_policy() or a policy limits update
// might be missed, so use a memory barrier to ensure it.
//
// This pairs with the memory barrier in sugov_should_update_freq().
//
    smp_wmb();
    WRITE_ONCE(sg_policy.limits_changed, true);
    }
pub static mut cpufreq_governor: usize = 0;

#[no_mangle]
pub unsafe extern "C" fn cpufreq_default_governor() -> *mut c_void {
    return &schedutil_gov;
    }

#[no_mangle]
pub unsafe extern "C" fn sugov_is_governor(policy: *mut cpufreq_policy) -> bool {
    return policy.governor == &schedutil_gov;
    }
    cpufreq_governor_init(schedutil_gov);