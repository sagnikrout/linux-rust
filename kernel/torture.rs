//! Automatically rewritten from C to Rust
//! Source: kernel/torture.c
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



// SPDX-License-Identifier: GPL-2.0+
//
// Common functions for in-kernel torture tests.
//
// Copyright (C) IBM Corporation, 2014
//
// Author: Paul E. McKenney <paulmck@linux.ibm.com>
// Based on kernel/rcu/torture.c.
//

    MODULE_DESCRIPTION("Common functions for in-kernel torture tests");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Paul E. McKenney <paulmck@linux.ibm.com>");
    static bool disable_onoff_at_boot;
    module_param!(disable_onoff_at_boot, bool, 0444);
    static bool ftrace_dump_at_shutdown;
    module_param!(ftrace_dump_at_shutdown, bool, 0444);
    static int verbose_sleep_frequency;
    module_param!(verbose_sleep_frequency, int, 0444);
pub static mut verbose_sleep_duration: int = 1;
    module_param!(verbose_sleep_duration, int, 0444);
    static int random_shuffle;
    module_param!(random_shuffle, int, 0444);
pub static mut torture_type: *mut c_void = core::ptr::null_mut();
    static int verbose;
// Mediate rmmod and system shutdown.  Concurrent rmmod & shutdown illegal!

pub static mut fullstop: int = 0;
// static DEFINE_MUTEX(fullstop_mutex);
    static atomic_t verbose_sleep_counter;
//
// Sleep if needed from VERBOSE_TOROUT*().
//
#[no_mangle]
pub unsafe extern "C" fn verbose_torout_sleep() {
    if (verbose_sleep_frequency > 0 &&
    verbose_sleep_duration > 0 &&
    !(atomic_inc_return(&verbose_sleep_counter) % verbose_sleep_frequency)) {
    schedule_timeout_uninterruptible(verbose_sleep_duration);
    }
    }
    EXPORT_SYMBOL_GPL(verbose_torout_sleep);
//
// Schedule a high-resolution-timer sleep in nanoseconds, with a 32-bit
// nanosecond random fuzz.  This function and its friends desynchronize
// testing from the timer wheel.
//
#[no_mangle]
pub unsafe extern "C" fn torture_hrtimeout_ns(baset_ns: ktime_t, fuzzt_ns: u32, mode: hrtimer_mode, trsp: *mut torture_random_state) -> c_int {
pub static mut hto: ktime_t = 0;
    if (trsp && fuzzt_ns) {
    hto += torture_random(trsp) % fuzzt_ns;
    }
    set_current_state(TASK_IDLE);
    return schedule_hrtimeout(&hto, mode);
    }
    EXPORT_SYMBOL_GPL(torture_hrtimeout_ns);
//
// Schedule a high-resolution-timer sleep in microseconds, with a 32-bit
// nanosecond (not microsecond!) random fuzz.
//
#[no_mangle]
pub unsafe extern "C" fn torture_hrtimeout_us(baset_us: u32, fuzzt_ns: u32, trsp: *mut torture_random_state) -> c_int {
pub static mut baset_ns: ktime_t = 0;
    return torture_hrtimeout_ns(baset_ns, fuzzt_ns, HRTIMER_MODE_REL, trsp);
    }
    EXPORT_SYMBOL_GPL(torture_hrtimeout_us);
//
// Schedule a high-resolution-timer sleep in milliseconds, with a 32-bit
// microsecond (not millisecond!) random fuzz.
//
#[no_mangle]
pub unsafe extern "C" fn torture_hrtimeout_ms(baset_ms: u32, fuzzt_us: u32, trsp: *mut torture_random_state) -> c_int {
pub static mut baset_ns: ktime_t = 0;
    let mut fuzzt_ns = 0;
    if ((u32)~0U / NSEC_PER_USEC < fuzzt_us) {
    fuzzt_ns = (u32)~0U;
    }
    else {
    fuzzt_ns = fuzzt_us * NSEC_PER_USEC;
    }
    return torture_hrtimeout_ns(baset_ns, fuzzt_ns, HRTIMER_MODE_REL, trsp);
    }
    EXPORT_SYMBOL_GPL(torture_hrtimeout_ms);
//
// Schedule a high-resolution-timer sleep in jiffies, with an
// implied one-jiffy random fuzz.  This is intended to replace calls to
// schedule_timeout_interruptible() and friends.
//
#[no_mangle]
pub unsafe extern "C" fn torture_hrtimeout_jiffies(baset_j: u32, trsp: *mut torture_random_state) -> c_int {
pub static mut baset_ns: ktime_t = 0;
    return torture_hrtimeout_ns(baset_ns, jiffies_to_nsecs(1), HRTIMER_MODE_REL, trsp);
    }
    EXPORT_SYMBOL_GPL(torture_hrtimeout_jiffies);
//
// Schedule a high-resolution-timer sleep in milliseconds, with a 32-bit
// millisecond (not second!) random fuzz.
//
#[no_mangle]
pub unsafe extern "C" fn torture_hrtimeout_s(baset_s: u32, fuzzt_ms: u32, trsp: *mut torture_random_state) -> c_int {
pub static mut baset_ns: ktime_t = 0;
    let mut fuzzt_ns = 0;
    if ((u32)~0U / NSEC_PER_MSEC < fuzzt_ms) {
    fuzzt_ns = (u32)~0U;
    }
    else {
    fuzzt_ns = fuzzt_ms * NSEC_PER_MSEC;
    }
    return torture_hrtimeout_ns(baset_ns, fuzzt_ns, HRTIMER_MODE_REL, trsp);
    }
    EXPORT_SYMBOL_GPL(torture_hrtimeout_s);

//
// Variables for online-offline handling.  Only present if CPU hotplug
// is enabled, otherwise does nothing.
//
pub static mut onoff_task: *mut c_void = core::ptr::null_mut();
    static long onoff_holdoff;
    static long onoff_interval;
pub static mut onoff_f: *mut c_void = core::ptr::null_mut();
    static long n_offline_attempts;
    static long n_offline_successes;
    static unsigned long sum_offline;
pub static mut min_offline: int = 0;
    static int max_offline;
    static long n_online_attempts;
    static long n_online_successes;
    static unsigned long sum_online;
pub static mut min_online: int = 0;
    static int max_online;
pub static mut torture_online_cpus: int = 0;
//
// Some torture testing leverages confusion as to the number of online
// CPUs.  This function returns the torture-testing view of this number,
// which allows torture tests to load-balance appropriately.
//
#[no_mangle]
pub unsafe extern "C" fn torture_num_online_cpus() -> c_int {
    return READ_ONCE(torture_online_cpus);
    }
    EXPORT_SYMBOL_GPL(torture_num_online_cpus);
//
// Attempt to take a CPU offline.  Return false if the CPU is already
// offline or if it is not subject to CPU-hotplug operations.  The
// caller can detect other failures by looking at the statistics.
//
#[no_mangle]
pub unsafe extern "C" fn torture_offline(cpu: c_int, n_offl_attempts: *mut c_long, n_offl_successes: *mut c_long, sum_offl: *mut c_ulong, min_offl: *mut c_int, max_offl: *mut c_int) -> bool {
    let mut delta = 0;
    let mut ret = 0;
pub static mut s: *mut c_void = core::ptr::null_mut();
    let mut starttime = 0;
    if (!cpu_online(cpu) || !cpu_is_hotpluggable(cpu)) {
    return false;
    }
    if (num_online_cpus() <= 1) {
    return false;  /* Can't offline the last CPU. */
    }
    if (verbose > 1) {
    pr_alert("%s" TORTURE_FLAG
    "torture_onoff task: offlining %d\n",
    torture_type, cpu);
    }
    starttime = jiffies;
    (*n_offl_attempts)++;
    ret = remove_cpu(cpu);
    if (ret) {
    s = "";
    if (!rcu_inkernel_boot_has_ended() && ret == -EBUSY) {
// PCI probe frequently disables hotplug during boot.
    (*n_offl_attempts)--;
    s = " (-EBUSY forgiven during boot)";
    }
    if (verbose) {
    pr_alert("%s" TORTURE_FLAG
    "torture_onoff task: offline %d failed%s: errno %d\n",
    torture_type, cpu, s, ret);
    }
    } else {
    if (verbose > 1) {
    pr_alert("%s" TORTURE_FLAG
    "torture_onoff task: offlined %d\n",
    torture_type, cpu);
    }
    if (onoff_f) {
    onoff_f();
    }
    (*n_offl_successes)++;
    delta = jiffies - starttime;
// sum_offl += delta;
    if (*min_offl < 0) {
// min_offl = delta;
// max_offl = delta;
    }
    if (*min_offl > delta) {
// min_offl = delta;
    }
    if (*max_offl < delta) {
// max_offl = delta;
    }
    WRITE_ONCE(torture_online_cpus, torture_online_cpus - 1);
    WARN_ON_ONCE!(torture_online_cpus <= 0);
    }
    return true;
    }
    EXPORT_SYMBOL_GPL(torture_offline);
//
// Attempt to bring a CPU online.  Return false if the CPU is already
// online or if it is not subject to CPU-hotplug operations.  The
// caller can detect other failures by looking at the statistics.
//
#[no_mangle]
pub unsafe extern "C" fn torture_online(cpu: c_int, n_onl_attempts: *mut c_long, n_onl_successes: *mut c_long, sum_onl: *mut c_ulong, min_onl: *mut c_int, max_onl: *mut c_int) -> bool {
    let mut delta = 0;
    let mut ret = 0;
pub static mut s: *mut c_void = core::ptr::null_mut();
    let mut starttime = 0;
    if (cpu_online(cpu) || !cpu_is_hotpluggable(cpu)) {
    return false;
    }
    if (verbose > 1) {
    pr_alert("%s" TORTURE_FLAG
    "torture_onoff task: onlining %d\n",
    torture_type, cpu);
    }
    starttime = jiffies;
    (*n_onl_attempts)++;
    ret = add_cpu(cpu);
    if (ret) {
    s = "";
    if (!rcu_inkernel_boot_has_ended() && ret == -EBUSY) {
// PCI probe frequently disables hotplug during boot.
    (*n_onl_attempts)--;
    s = " (-EBUSY forgiven during boot)";
    }
    if (verbose) {
    pr_alert("%s" TORTURE_FLAG
    "torture_onoff task: online %d failed%s: errno %d\n",
    torture_type, cpu, s, ret);
    }
    } else {
    if (verbose > 1) {
    pr_alert("%s" TORTURE_FLAG
    "torture_onoff task: onlined %d\n",
    torture_type, cpu);
    }
    (*n_onl_successes)++;
    delta = jiffies - starttime;
// sum_onl += delta;
    if (*min_onl < 0) {
// min_onl = delta;
// max_onl = delta;
    }
    if (*min_onl > delta) {
// min_onl = delta;
    }
    if (*max_onl < delta) {
// max_onl = delta;
    }
    WRITE_ONCE(torture_online_cpus, torture_online_cpus + 1);
    }
    return true;
    }
    EXPORT_SYMBOL_GPL(torture_online);
//
// Get everything online at the beginning and ends of tests.
//
#[no_mangle]
unsafe extern "C" fn torture_online_all(phase: *mut c_char) {
    let mut cpu = 0;
    let mut ret = 0;
    for_each_possible_cpu(cpu) {
    if (cpu_online(cpu)) {
    continue;
    }
    ret = add_cpu(cpu);
    if (ret && verbose) {
    pr_alert("%s" TORTURE_FLAG
    "%s: %s online %d: errno %d\n",
    __func__, phase, torture_type, cpu, ret);
    }
    }
    }
//
// Execute random CPU-hotplug operations at the interval specified
// by the onoff_interval.
//
#[no_mangle]
pub unsafe extern "C" fn torture_onoff(arg: *mut c_void) -> c_int {
    let mut cpu = 0;
pub static mut maxcpu: c_int = 0;
pub static mut rand: usize = 0;
    VERBOSE_TOROUT_STRING("torture_onoff task started");
    for_each_online_cpu(cpu) {
    maxcpu = cpu;
    }
    WARN_ON!(maxcpu < 0);
    torture_online_all("Initial");
    if (maxcpu == 0) {
    VERBOSE_TOROUT_STRING("Only one CPU, so CPU-hotplug testing is disabled");
// goto;
    }
    if (onoff_holdoff > 0) {
    VERBOSE_TOROUT_STRING("torture_onoff begin holdoff");
    torture_hrtimeout_jiffies(onoff_holdoff, &rand);
    VERBOSE_TOROUT_STRING("torture_onoff end holdoff");
    }
    while (!rcu_inkernel_boot_has_ended()) {
    schedule_timeout_interruptible(HZ / 10);
    }
    while (!torture_must_stop()) {
    if (disable_onoff_at_boot && !rcu_inkernel_boot_has_ended()) {
    torture_hrtimeout_jiffies(HZ / 10, &rand);
    continue;
    }
    cpu = torture_random(&rand) % (maxcpu + 1);
    if (!torture_offline(cpu,
    &n_offline_attempts, &n_offline_successes,
    &sum_offline, &min_offline, &max_offline)) {
    torture_online(cpu,
    &n_online_attempts, &n_online_successes,
    &sum_online, &min_online, &max_online);
    }
    torture_hrtimeout_jiffies(onoff_interval, &rand);
    }
// label;
    torture_kthread_stopping("torture_onoff");
    torture_online_all("Final");
    return 0;
    }

//
// Initiate online-offline handling.
//
#[no_mangle]
pub unsafe extern "C" fn torture_onoff_init(ooholdoff: c_long, oointerval: c_long, f: *mut torture_ofl_func) -> c_int {

    onoff_holdoff = ooholdoff;
    onoff_interval = oointerval;
    onoff_f = f;
    if (onoff_interval <= 0) {
    return 0;
    }
    return torture_create_kthread(torture_onoff, core::ptr::null_mut(), onoff_task);

    return 0;

    }
    EXPORT_SYMBOL_GPL(torture_onoff_init);
//
// Clean up after online/offline testing.
//
#[no_mangle]
unsafe extern "C" fn torture_onoff_cleanup() {

    if (onoff_task == core::ptr::null_mut()) {
    return;
    }
    VERBOSE_TOROUT_STRING("Stopping torture_onoff task");
    kthread_stop(onoff_task);
    onoff_task = core::ptr::null_mut();

    }
//
// Print online/offline testing statistics.
//
#[no_mangle]
pub unsafe extern "C" fn torture_onoff_stats() {

    pr_cont("onoff: %ld/%ld:%ld/%ld %d,%d:%d,%d %lu:%lu (HZ=%d) ",
    n_online_successes, n_online_attempts,
    n_offline_successes, n_offline_attempts,
    min_online, max_online,
    min_offline, max_offline,
    sum_online, sum_offline, HZ);

    }
    EXPORT_SYMBOL_GPL(torture_onoff_stats);
//
// Were all the online/offline operations successful?
//
#[no_mangle]
pub unsafe extern "C" fn torture_onoff_failures() -> bool {

    return n_online_successes != n_online_attempts ||
    n_offline_successes != n_offline_attempts;

    return false;

    }
    EXPORT_SYMBOL_GPL(torture_onoff_failures);

pub const TORTURE_RANDOM_REFRESH: c_int = 10000;
//
// Crude but fast random-number generator.  Uses a linear congruential
// generator, with occasional help from cpu_clock().
//
#[no_mangle]
pub unsafe extern "C" fn torture_random(trsp: *mut torture_random_state) -> c_ulong {
    if (--trsp.trs_count < 0) {
    trsp.trs_state += (unsigned long)local_clock() + raw_smp_processor_id();
    trsp.trs_count = TORTURE_RANDOM_REFRESH;
    }
    trsp.trs_state = trsp.trs_state * TORTURE_RANDOM_MULT +
    TORTURE_RANDOM_ADD;
    return swahw32(trsp.trs_state);
    }
    EXPORT_SYMBOL_GPL(torture_random);
//
// Variables for shuffling.  The idea is to ensure that each CPU stays
// idle for an extended period to test interactions with dyntick idle,
// as well as interactions with any per-CPU variables.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shuffle_task {
    pub st_l: list_head,
    pub st_t: *mut task_struct,
}

    static long shuffle_interval;	/* In jiffies. */
pub static mut shuffler_task: *mut c_void = core::ptr::null_mut();
    static cpumask_var_t shuffle_tmp_mask;
    static int shuffle_idle_cpu;	/* Force all torture tasks off this CPU */
pub static mut shuffle_task_list: list_head = 0;
// static DEFINE_MUTEX(shuffle_task_mutex);
//
// Register a task to be shuffled.  If there is no memory, just splat
// and don't bother registering.
//
#[no_mangle]
pub unsafe extern "C" fn torture_shuffle_task_register(tp: *mut task_struct) {
pub static mut stp: *mut c_void = core::ptr::null_mut();
    if (WARN_ON_ONCE!(tp == core::ptr::null_mut())) {
    return;
    }
    stp = kmalloc_obj(*stp);
    if (WARN_ON_ONCE!(stp == core::ptr::null_mut())) {
    return;
    }
    stp.st_t = tp;
    mutex_lock(&shuffle_task_mutex);
    list_add(&stp.st_l, &shuffle_task_list);
    mutex_unlock(&shuffle_task_mutex);
    }
    EXPORT_SYMBOL_GPL(torture_shuffle_task_register);
//
// Unregister all tasks, for example, at the end of the torture run.
//
#[no_mangle]
unsafe extern "C" fn torture_shuffle_task_unregister_all() {
pub static mut stp: *mut c_void = core::ptr::null_mut();
pub static mut p: *mut c_void = core::ptr::null_mut();
    mutex_lock(&shuffle_task_mutex);
    list_for_each_entry_safe(stp, p, &shuffle_task_list, st_l) {
    list_del(&stp.st_l);
    kfree(stp);
    }
    mutex_unlock(&shuffle_task_mutex);
    }
// Shuffle tasks such that we allow shuffle_idle_cpu to become idle.
// A special case is when shuffle_idle_cpu = -1, in which case we allow
// the tasks to run on all CPUs.
//
#[no_mangle]
unsafe extern "C" fn torture_shuffle_tasks(trp: *mut torture_random_state) {
pub static mut stp: *mut c_void = core::ptr::null_mut();
    cpumask_setall(shuffle_tmp_mask);
    cpus_read_lock();
// No point in shuffling if there is only one online CPU (ex: UP)
    if (num_online_cpus() == 1) {
    cpus_read_unlock();
    return;
    }
// Advance to the next CPU.  Upon overflow, don't idle any CPUs.
    shuffle_idle_cpu = cpumask_next(shuffle_idle_cpu, shuffle_tmp_mask);
    if (shuffle_idle_cpu >= nr_cpu_ids) {
    shuffle_idle_cpu = -1;
    }
    else {
    cpumask_clear_cpu(shuffle_idle_cpu, shuffle_tmp_mask);
    }
    mutex_lock(&shuffle_task_mutex);
    list_for_each_entry(stp, &shuffle_task_list, st_l) {
    if (!random_shuffle || torture_random(trp) & 0x1) {
    set_cpus_allowed_ptr(stp.st_t, shuffle_tmp_mask);
    }
    }
    mutex_unlock(&shuffle_task_mutex);
    cpus_read_unlock();
    }
// Shuffle tasks across CPUs, with the intent of allowing each CPU in the
// system to become idle at a time and cut off its timer ticks. This is meant
// to test the support for such tickless idle CPU in RCU.
//
#[no_mangle]
unsafe extern "C" fn torture_shuffle(arg: *mut c_void) -> c_int {
pub static mut rand: usize = 0;
    VERBOSE_TOROUT_STRING("torture_shuffle task started");
    do {
    torture_hrtimeout_jiffies(shuffle_interval, &rand);
    torture_shuffle_tasks(&rand);
    torture_shutdown_absorb("torture_shuffle");
    } while (!torture_must_stop());
    torture_kthread_stopping("torture_shuffle");
    return 0;
    }
//
// Start the shuffler, with shuffint in jiffies.
//
#[no_mangle]
pub unsafe extern "C" fn torture_shuffle_init(shuffint: c_long) -> c_int {
    let mut ret = 0;
    shuffle_interval = shuffint;
    shuffle_idle_cpu = -1;
    if (!alloc_cpumask_var(&shuffle_tmp_mask, GFP_KERNEL)) {
    TOROUT_ERRSTRING("Failed to alloc mask");
    return -ENOMEM;
    }
// Create the shuffler thread
    ret = torture_create_kthread(torture_shuffle, core::ptr::null_mut(), shuffler_task);
    if (ret) {
    free_cpumask_var(shuffle_tmp_mask);
    }
    return ret;
    }
    EXPORT_SYMBOL_GPL(torture_shuffle_init);
//
// Stop the shuffling.
//
#[no_mangle]
unsafe extern "C" fn torture_shuffle_cleanup() {
    torture_shuffle_task_unregister_all();
    if (shuffler_task) {
    VERBOSE_TOROUT_STRING("Stopping torture_shuffle task");
    kthread_stop(shuffler_task);
    free_cpumask_var(shuffle_tmp_mask);
    }
    shuffler_task = core::ptr::null_mut();
    }
//
// Variables for auto-shutdown.  This allows "lights out" torture runs
// to be fully scripted.
//
pub static mut shutdown_task: *mut c_void = core::ptr::null_mut();
    static ktime_t shutdown_time;		/* time to system shutdown. */
    static void (*torture_shutdown_hook)(void);
//
// Absorb kthreads into a kernel function that won't return, so that
// they won't ever access module text or data again.
//
#[no_mangle]
pub unsafe extern "C" fn torture_shutdown_absorb(title: *const c_char) {
    while (READ_ONCE(fullstop) == FULLSTOP_SHUTDOWN) {
    pr_notice("torture thread %s parking due to system shutdown\n",
    title);
    schedule_timeout_uninterruptible(MAX_SCHEDULE_TIMEOUT);
    }
    }
    EXPORT_SYMBOL_GPL(torture_shutdown_absorb);
//
// Cause the torture test to shutdown the system after the test has
// run for the time specified by the shutdown_secs parameter.
//
#[no_mangle]
unsafe extern "C" fn torture_shutdown(arg: *mut c_void) -> c_int {
    let mut ktime_snap;
    VERBOSE_TOROUT_STRING("torture_shutdown task started");
    ktime_snap = ktime_get();
    while (ktime_before(ktime_snap, shutdown_time) &&
    !torture_must_stop()) {
    if (verbose) {
    pr_alert("%s" TORTURE_FLAG
    "torture_shutdown task: %llu ms remaining\n",
    torture_type,
    ktime_ms_delta(shutdown_time, ktime_snap));
    }
    set_current_state(TASK_INTERRUPTIBLE);
    schedule_hrtimeout(&shutdown_time, HRTIMER_MODE_ABS);
    ktime_snap = ktime_get();
    }
    if (torture_must_stop()) {
    torture_kthread_stopping("torture_shutdown");
    return 0;
    }
// OK, shut down the system.
    VERBOSE_TOROUT_STRING("torture_shutdown task shutting down system");
    shutdown_task = core::ptr::null_mut();	/* Avoid self-kill deadlock. */
    if (torture_shutdown_hook) {
    torture_shutdown_hook();
    }
    else {
    VERBOSE_TOROUT_STRING("No torture_shutdown_hook(), skipping.");
    }
    if (ftrace_dump_at_shutdown) {
    rcu_ftrace_dump(DUMP_ALL);
    }
    kernel_power_off();	/* Shut down the system. */
    return 0;
    }
//
// Start up the shutdown task.
//
#[no_mangle]
pub unsafe extern "C" fn torture_shutdown_init(ssecs: c_int, (*cleanup)(void): *mut c_void) -> c_int {
#[no_mangle]
#[no_mangle]
// duplicate fn: torture_shutdown_init
pub unsafe extern "C" fn torture_shutdown_init_dup(ssecs: c_int) -> c_int {
    torture_shutdown_hook = cleanup;
    if (ssecs > 0) {
    shutdown_time = ktime_add(ktime_get(), ktime_set(ssecs, 0));
    return torture_create_kthread(torture_shutdown, core::ptr::null_mut(),
    shutdown_task);
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(torture_shutdown_init);
//
// Detect and respond to a system shutdown.
//
#[no_mangle]
pub unsafe extern "C" fn torture_shutdown_notify(unused1: *mut notifier_block, unused2: c_ulong, unused3: *mut c_void) -> c_int {
    mutex_lock(&fullstop_mutex);
    if (READ_ONCE(fullstop) == FULLSTOP_DONTSTOP) {
    VERBOSE_TOROUT_STRING("Unscheduled system shutdown detected");
    WRITE_ONCE(fullstop, FULLSTOP_SHUTDOWN);
    } else {
    pr_warn!("Concurrent rmmod and shutdown illegal!\n");
    }
    mutex_unlock(&fullstop_mutex);
    return NOTIFY_DONE;
    }
pub static mut notifier_block: usize = 0;
//
// Shut down the shutdown task.  Say what???  Heh!  This can happen if
// the torture module gets an rmmod before the shutdown time arrives.  ;-)
//
#[no_mangle]
unsafe extern "C" fn torture_shutdown_cleanup() {
    unregister_reboot_notifier(&torture_shutdown_nb);
    if (shutdown_task != core::ptr::null_mut()) {
    VERBOSE_TOROUT_STRING("Stopping torture_shutdown task");
    kthread_stop(shutdown_task);
    }
    shutdown_task = core::ptr::null_mut();
    }
//
// Variables for stuttering, which means to periodically pause and
// restart testing in order to catch bugs that appear when load is
// suddenly applied to or removed from the system.
//
pub static mut stutter_task: *mut c_void = core::ptr::null_mut();
    static ktime_t stutter_till_abs_time;
    static int stutter;
    static int stutter_gap;
//
// Block until the stutter interval ends.  This must be called periodically
// by all running kthreads that need to be subject to stuttering.
//
#[no_mangle]
pub unsafe extern "C" fn stutter_wait(title: *const c_char) -> bool {
pub static mut ret: bool = false;
    let mut till_ns;
    cond_resched_tasks_rcu_qs();
    till_ns = READ_ONCE(stutter_till_abs_time);
    if (till_ns && ktime_before(ktime_get(), till_ns)) {
    torture_hrtimeout_ns(till_ns, 0, HRTIMER_MODE_ABS, core::ptr::null_mut());
    ret = true;
    }
    torture_shutdown_absorb(title);
    return ret;
    }
    EXPORT_SYMBOL_GPL(stutter_wait);
//
// Cause the torture test to "stutter", starting and stopping all
// threads periodically.
//
#[no_mangle]
unsafe extern "C" fn torture_stutter(arg: *mut c_void) -> c_int {
    let mut till_ns;
    VERBOSE_TOROUT_STRING("torture_stutter task started");
    do {
    if (!torture_must_stop() && stutter > 1) {
    till_ns = ktime_add_ns(ktime_get(),
    jiffies_to_nsecs(stutter));
    WRITE_ONCE(stutter_till_abs_time, till_ns);
    torture_hrtimeout_jiffies(stutter - 1, core::ptr::null_mut());
    }
    if (!torture_must_stop()) {
    torture_hrtimeout_jiffies(stutter_gap, core::ptr::null_mut());
    }
    torture_shutdown_absorb("torture_stutter");
    } while (!torture_must_stop());
    torture_kthread_stopping("torture_stutter");
    return 0;
    }
//
// Initialize and kick off the torture_stutter kthread.
//
#[no_mangle]
pub unsafe extern "C" fn torture_stutter_init(s: c_int, sgap: c_int) -> c_int {
    stutter = s;
    stutter_gap = sgap;
    return torture_create_kthread(torture_stutter, core::ptr::null_mut(), stutter_task);
    }
    EXPORT_SYMBOL_GPL(torture_stutter_init);
//
// Cleanup after the torture_stutter kthread.
//
#[no_mangle]
unsafe extern "C" fn torture_stutter_cleanup() {
    if (!stutter_task) {
    return;
    }
    VERBOSE_TOROUT_STRING("Stopping torture_stutter task");
    kthread_stop(stutter_task);
    stutter_task = core::ptr::null_mut();
    }
    static unsigned long torture_init_jiffies;
#[no_mangle]
pub unsafe extern "C" fn torture_print_module_parms() {
    pr_alert("torture module --- %s:  disable_onoff_at_boot=%d ftrace_dump_at_shutdown=%d verbose_sleep_frequency=%d verbose_sleep_duration=%d random_shuffle=%d%s\n",
    torture_type, disable_onoff_at_boot, ftrace_dump_at_shutdown, verbose_sleep_frequency, verbose_sleep_duration, random_shuffle,
    rcu_inkernel_boot_has_ended() ? "" : " still booting");
    }
//
// Initialize torture module.  Please note that this is -not- invoked via
// the usual module_init!() mechanism, but rather by an explicit call from
// the client torture module.  This call must be paired with a later
// torture_init_end().
//
// The runnable parameter points to a flag that controls whether or not
// the test is currently runnable.  If there is no such flag, pass in NULL.
//
#[no_mangle]
pub unsafe extern "C" fn torture_init_begin(ttype: *mut c_char, v: c_int) -> bool {
    mutex_lock(&fullstop_mutex);
    if (torture_type != core::ptr::null_mut()) {
    pr_alert("%s: Refusing %s init: %s running.\n",
    __func__, ttype, torture_type);
    pr_alert("%s: One torture test at a time!\n", __func__);
    mutex_unlock(&fullstop_mutex);
    return false;
    }
    torture_type = ttype;
    verbose = v;
    fullstop = FULLSTOP_DONTSTOP;
    WRITE_ONCE(torture_init_jiffies, jiffies); // Lockless reads.
    torture_print_module_parms();
    return true;
    }
    EXPORT_SYMBOL_GPL(torture_init_begin);
//
// Tell the torture module that initialization is complete.
//
#[no_mangle]
pub unsafe extern "C" fn torture_init_end() {
    mutex_unlock(&fullstop_mutex);
    register_reboot_notifier(&torture_shutdown_nb);
    }
    EXPORT_SYMBOL_GPL(torture_init_end);
//
// Get the torture_init_begin()-time value of the jiffies counter.
//
#[no_mangle]
pub unsafe extern "C" fn get_torture_init_jiffies() -> c_ulong {
    return READ_ONCE(torture_init_jiffies);
    }
    EXPORT_SYMBOL_GPL(get_torture_init_jiffies);
//
// Clean up torture module.  Please note that this is -not- invoked via
// the usual module_exit!() mechanism, but rather by an explicit call from
// the client torture module.  Returns true if a race with system shutdown
// is detected, otherwise, all kthreads started by functions in this file
// will be shut down.
//
// This must be called before the caller starts shutting down its own
// kthreads.
//
// Both torture_cleanup_begin() and torture_cleanup_end() must be paired,
// in order to correctly perform the cleanup. They are separated because
// threads can still need to reference the torture_type type, thus nullify
// only after completing all other relevant calls.
//
#[no_mangle]
pub unsafe extern "C" fn torture_cleanup_begin() -> bool {
    mutex_lock(&fullstop_mutex);
    if (READ_ONCE(fullstop) == FULLSTOP_SHUTDOWN) {
    pr_warn!("Concurrent rmmod and shutdown illegal!\n");
    mutex_unlock(&fullstop_mutex);
    schedule_timeout_uninterruptible(10);
    return true;
    }
    WRITE_ONCE(fullstop, FULLSTOP_RMMOD);
    mutex_unlock(&fullstop_mutex);
    torture_shutdown_cleanup();
    torture_shuffle_cleanup();
    torture_stutter_cleanup();
    torture_onoff_cleanup();
    return false;
    }
    EXPORT_SYMBOL_GPL(torture_cleanup_begin);
#[no_mangle]
pub unsafe extern "C" fn torture_cleanup_end() {
    mutex_lock(&fullstop_mutex);
    torture_type = core::ptr::null_mut();
    mutex_unlock(&fullstop_mutex);
    }
    EXPORT_SYMBOL_GPL(torture_cleanup_end);
//
// Is it time for the current torture test to stop?
//
#[no_mangle]
pub unsafe extern "C" fn torture_must_stop() -> bool {
    return torture_must_stop_irq() || kthread_should_stop();
    }
    EXPORT_SYMBOL_GPL(torture_must_stop);
//
// Is it time for the current torture test to stop?  This is the irq-safe
// version, hence no check for kthread_should_stop().
//
#[no_mangle]
pub unsafe extern "C" fn torture_must_stop_irq() -> bool {
    return READ_ONCE(fullstop) != FULLSTOP_DONTSTOP;
    }
    EXPORT_SYMBOL_GPL(torture_must_stop_irq);
//
// Each kthread must wait for kthread_should_stop() before returning from
// its top-level function, otherwise segfaults ensue.  This function
// prints a "stopping" message and waits for kthread_should_stop(), and
// should be called from all torture kthreads immediately prior to
// returning.
//
#[no_mangle]
pub unsafe extern "C" fn torture_kthread_stopping(title: *mut c_char) {
    char buf[128];
    snprintf(buf, sizeof!(buf), "%s is stopping", title);
    VERBOSE_TOROUT_STRING(buf);
    while (!kthread_should_stop()) {
    torture_shutdown_absorb(title);
    schedule_timeout_uninterruptible(HZ / 20);
    }
    }
    EXPORT_SYMBOL_GPL(torture_kthread_stopping);
//
// Create a generic torture kthread that is immediately runnable.  If you
// need the kthread to be stopped so that you can do something to it before
// it starts, you will need to open-code your own.
//
#[no_mangle]
pub unsafe extern "C" fn _torture_create_kthread(arg: *mut c_void, s: *mut c_char, m: *mut c_char, f: *mut c_char, tp: *mut *mut task_struct) -> c_int {
pub static mut ret: c_int = 0;
    VERBOSE_TOROUT_STRING(m);
// tp = kthread_create(fn, arg, "%s", s);
    if (IS_ERR(*tp)) {
    ret = PTR_ERR(*tp);
    TOROUT_ERRSTRING(f);
// tp = NULL;
    return ret;
    }
    if (cbf) {
    cbf(*tp);
    }
    wake_up_process(*tp);  // Process is sleeping, so ordering provided.
    torture_shuffle_task_register(*tp);
    return ret;
    }
    EXPORT_SYMBOL_GPL(_torture_create_kthread);
//
// Stop a generic kthread, emitting a message.
//
#[no_mangle]
pub unsafe extern "C" fn _torture_stop_kthread(m: *mut c_char, tp: *mut task_struct) {
    if (*tp == core::ptr::null_mut()) {
    return;
    }
    VERBOSE_TOROUT_STRING(m);
    kthread_stop(*tp);
// tp = NULL;
    }
    EXPORT_SYMBOL_GPL(_torture_stop_kthread);
//
// Set the specified task's niceness value, saturating at limits.
// Saturating noisily, but saturating.
//
#[no_mangle]
pub unsafe extern "C" fn torture_sched_set_normal(t: *mut task_struct, nice: c_int) {
pub static mut realnice: c_int = 0;
    if (WARN_ON_ONCE!(realnice > MAX_NICE)) {
    realnice = MAX_NICE;
    }
    if (WARN_ON_ONCE!(realnice < MIN_NICE)) {
    realnice = MIN_NICE;
    }
    sched_set_normal(t, realnice);
    }
    EXPORT_SYMBOL_GPL(torture_sched_set_normal);

}