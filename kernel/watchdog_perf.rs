//! Automatically rewritten from C to Rust
//! Source: kernel/watchdog_perf.c
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
// Detect hard lockups on a system using perf
//
// started by Don Zickus, Copyright (C) 2010 Red Hat, Inc.
//
// Note: Most of this code is borrowed heavily from the original softlockup
// detector, so thanks to Ingo for the initial implementation.
// Some chunks also taken from the old x86-specific nmi watchdog code, thanks
// to those contributors as well.
//

pub static mut struct perf_event *: usize = 0;
pub static mut watchdog_cpus: atomic_t = 0;

pub static mut ktime_t: usize = 0;
pub static mut unsigned int: usize = 0;
    static ktime_t watchdog_hrtimer_sample_threshold ;
#[no_mangle]
pub unsafe extern "C" fn watchdog_update_hrtimer_threshold(period: u64) {
//
// The hrtimer runs with a period of (watchdog_threshold * 2) / 5
//
// So it runs effectively with 2.5 times the rate of the NMI
// watchdog. That means the hrtimer should fire 2-3 times before
// the NMI watchdog expires. The NMI watchdog on x86 is based on
// unhalted CPU cycles, so if Turbo-Mode is enabled the CPU cycles
// might run way faster than expected and the NMI fires in a
// smaller period than the one deduced from the nominal CPU
// frequency. Depending on the Turbo-Mode factor this might be fast
// enough to get the NMI period smaller than the hrtimer watchdog
// period and trigger false positives.
//
// The sample threshold is used to check in the NMI handler whether
// the minimum time between two NMI samples has elapsed. That
// prevents false positives.
//
// Set this to 4/5 of the actual watchdog threshold period so the
// hrtimer is guaranteed to fire at least once within the real
// watchdog threshold.
//
    watchdog_hrtimer_sample_threshold = period * 2;
    }
#[no_mangle]
unsafe extern "C" fn watchdog_check_timestamp() -> bool {
    ktime_t delta, now = ktime_get_mono_fast_ns();
    delta = now - __this_cpu_read(last_timestamp);
    if (delta < watchdog_hrtimer_sample_threshold) {
//
// If ktime is jiffies based, a stalled timer would prevent
// jiffies from being incremented and the filter would look
// at a stale timestamp and never trigger.
//
    if (__this_cpu_inc_return(nmi_rearmed) < 10) {
    return false;
    }
    }
    __this_cpu_write(nmi_rearmed, 0);
    __this_cpu_write(last_timestamp, now);
    return true;
    }
#[no_mangle]
unsafe extern "C" fn watchdog_init_timestamp() {
    __this_cpu_write(nmi_rearmed, 0);
    __this_cpu_write(last_timestamp, ktime_get_mono_fast_ns());
    }

#[no_mangle]
pub unsafe extern "C" fn watchdog_check_timestamp() -> bool { return true; }
#[no_mangle]
pub unsafe extern "C" fn watchdog_init_timestamp() { }

pub static mut perf_event_attr: usize = 0;
pub static mut perf_event_attr: usize = 0;
// Callback function for perf event subsystem
#[no_mangle]
pub unsafe extern "C" fn watchdog_overflow_callback(event: *mut perf_event, data: *mut perf_sample_data, regs: *mut pt_regs) {
// Ensure the watchdog never gets throttled
    event.hw.interrupts = 0;
    if (panic_in_progress()) {
    return;
    }
    if (!watchdog_check_timestamp()) {
    return;
    }
    watchdog_hardlockup_check(smp_processor_id(), regs);
    }
#[no_mangle]
pub unsafe extern "C" fn hardlockup_detector_event_create(cpu: c_uint) -> *mut c_void {
pub static mut wd_attr: *mut c_void = core::ptr::null_mut();
pub static mut evt: *mut c_void = core::ptr::null_mut();
    wd_attr = &wd_hw_attr;
    wd_attr.sample_period = hw_nmi_get_sample_period(watchdog_thresh);
// Try to register using hardware perf events
    evt = perf_event_create_kernel_counter(wd_attr, cpu, core::ptr::null_mut(),
    watchdog_overflow_callback, core::ptr::null_mut());
    if (IS_ERR(evt)) {
    wd_attr = &fallback_wd_hw_attr;
    wd_attr.sample_period = hw_nmi_get_sample_period(watchdog_thresh);
    evt = perf_event_create_kernel_counter(wd_attr, cpu, core::ptr::null_mut(),
    watchdog_overflow_callback, core::ptr::null_mut());
    }
    return evt;
    }
//
// watchdog_hardlockup_enable - Enable the local event
// @cpu: The CPU to enable hard lockup on.
//
#[no_mangle]
pub unsafe extern "C" fn watchdog_hardlockup_enable(cpu: c_uint) {
pub static mut evt: *mut c_void = core::ptr::null_mut();
    WARN_ON_ONCE!(cpu != smp_processor_id());
    evt = hardlockup_detector_event_create(cpu);
    if (IS_ERR(evt)) {
    pr_debug!("Perf event create on CPU %d failed with %ld\n", cpu,
    PTR_ERR(evt));
    return;
    }
// use original value for check
    if (!atomic_fetch_inc(&watchdog_cpus)) {
    pr_info!("Enabled. Permanently consumes one hw-PMU counter.\n");
    }
    WARN_ONCE(this_cpu_read(watchdog_ev), "unexpected watchdog_ev leak");
    this_cpu_write(watchdog_ev, evt);
    watchdog_init_timestamp();
    perf_event_enable(evt);
    }
//
// watchdog_hardlockup_disable - Disable the local event
// @cpu: The CPU to enable hard lockup on.
//
#[no_mangle]
pub unsafe extern "C" fn watchdog_hardlockup_disable(cpu: c_uint) {
    let mut event = this_cpu_read(watchdog_ev);
    WARN_ON_ONCE!(cpu != smp_processor_id());
    if (event) {
    perf_event_disable(event);
    perf_event_release_kernel(event);
    this_cpu_write(watchdog_ev, core::ptr::null_mut());
    atomic_dec(&watchdog_cpus);
    }
    }
//
// hardlockup_detector_perf_adjust_period - Adjust the event period due
// to current cpu frequency change
// @period: The target period to be set
//
#[no_mangle]
pub unsafe extern "C" fn hardlockup_detector_perf_adjust_period(period: u64) {
    let mut event = this_cpu_read(watchdog_ev);
    if (!(watchdog_enabled & WATCHDOG_HARDLOCKUP_ENABLED)) {
    return;
    }
    if (!event) {
    return;
    }
    if (event.attr.sample_period == period) {
    return;
    }
    if (perf_event_period(event, period)) {
    pr_err!("failed to change period to %llu\n", period);
    }
    }
//
// hardlockup_detector_perf_stop - Globally stop watchdog events
//
// Special interface for x86 to handle the perf HT bug.
//
#[no_mangle]
pub unsafe extern "C" fn hardlockup_detector_perf_stop()  {
    let mut cpu = 0;
    lockdep_assert_cpus_held();
    for_each_online_cpu(cpu) {
    let mut event = per_cpu(watchdog_ev, cpu);
    if (event) {
    perf_event_disable(event);
    }
    }
    }
//
// hardlockup_detector_perf_restart - Globally restart watchdog events
//
// Special interface for x86 to handle the perf HT bug.
//
#[no_mangle]
pub unsafe extern "C" fn hardlockup_detector_perf_restart()  {
    let mut cpu = 0;
    lockdep_assert_cpus_held();
    if (!(watchdog_enabled & WATCHDOG_HARDLOCKUP_ENABLED)) {
    return;
    }
    for_each_online_cpu(cpu) {
    let mut event = per_cpu(watchdog_ev, cpu);
    if (event) {
    perf_event_enable(event);
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn arch_perf_nmi_is_available() -> bool __weak __init {
    return true;
    }
//
// watchdog_hardlockup_probe - Probe whether NMI event is available at all
//
#[no_mangle]
pub unsafe extern "C" fn watchdog_hardlockup_probe() -> c_int {
pub static mut evt: *mut c_void = core::ptr::null_mut();
    let mut cpu = 0;
    let mut ret = 0;
    if (!arch_perf_nmi_is_available()) {
    return -ENODEV;
    }
    if (!hw_nmi_get_sample_period(watchdog_thresh)) {
    return -EINVAL;
    }
//
// Test hardware PMU availability by creating a temporary perf event.
// The event is released immediately.
//
    cpu = raw_smp_processor_id();
    evt = hardlockup_detector_event_create(cpu);
    if (IS_ERR(evt)) {
    pr_info!("Perf NMI watchdog permanently disabled\n");
    ret = PTR_ERR(evt);
    } else {
    perf_event_release_kernel(evt);
    ret = 0;
    }
    return ret;
    }
//
// hardlockup_config_perf_event - Overwrite config of wd_hw_attr.
// @str: number which identifies the raw perf event to use
//
#[no_mangle]
pub unsafe extern "C" fn hardlockup_config_perf_event(str: *const c_char)  {
    let mut config = 0;
    char buf[24];
    let mut comma = strchr(str, ',');
    if (!comma) {
    if (kstrtoull(str, 16, &config)) {
    return;
    }
    } else {
pub static mut len: c_uint = 0;
    if (len > sizeof!(buf)) {
    return;
    }
    strscpy(buf, str, len);
    if (kstrtoull(buf, 16, &config)) {
    return;
    }
    }
    wd_hw_attr.type = PERF_TYPE_RAW;
    wd_hw_attr.config = config;
    }