//! Automatically rewritten from C to Rust
//! Source: kernel/time/clocksource-wdtest.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Unit test for the clocksource watchdog.
//
// Copyright (C) 2021 Facebook, Inc.
// Copyright (C) 2026 Intel Corp.
//
// Author: Paul E. McKenney <paulmck@kernel.org>
// Author: Thomas Gleixner <tglx@kernel.org>
//

    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Clocksource watchdog unit test");
    MODULE_AUTHOR("Paul E. McKenney <paulmck@kernel.org>");
    MODULE_AUTHOR("Thomas Gleixner <tglx@kernel.org>");
    enum wdtest_states {
    WDTEST_INJECT_NONE,
    WDTEST_INJECT_DELAY,
    WDTEST_INJECT_POSITIVE,
    WDTEST_INJECT_NEGATIVE,
    WDTEST_INJECT_PERCPU	= 0x100,
    };
    static enum wdtest_states wdtest_state;
    static unsigned long wdtest_test_count;
    static ktime_t wdtest_last_ts, wdtest_offset;
pub const SHIFT_4000PPM: c_int = 8;
#[no_mangle]
unsafe extern "C" fn wdtest_get_offset(cs: *mut clocksource) -> ktime_t {
    if (wdtest_state < WDTEST_INJECT_PERCPU) {
    return wdtest_test_count & 0x1 ? 0 : wdtest_offset >> SHIFT_4000PPM;
    }
// Only affect the readout of the "remote" CPU
    return cs.wd_cpu == smp_processor_id() ? 0 : NSEC_PER_MSEC;
    }
#[no_mangle]
unsafe extern "C" fn wdtest_ktime_read(cs: *mut clocksource) -> u64 {
pub static mut now: ktime_t = 0;
pub static mut intv: ktime_t = 0;
//
// Only increment the test counter once per watchdog interval and
// store the interval for the offset calculation of this step. This
// guarantees a consistent behaviour even if the other side needs
// to repeat due to a watchdog read timeout.
//
    if (intv > (NSEC_PER_SEC / 4)) {
    WRITE_ONCE(wdtest_test_count, wdtest_test_count + 1);
    wdtest_last_ts = now;
    wdtest_offset = intv;
    }
    match (wdtest_state & ~WDTEST_INJECT_PERCPU) {
    WDTEST_INJECT_POSITIVE => {
    return now + wdtest_get_offset(cs);
    }
    WDTEST_INJECT_NEGATIVE => {
    return now - wdtest_get_offset(cs);
    }
    WDTEST_INJECT_DELAY => {
    udelay(500);
    return now;
    }
    _ => {
    return now;
    }
    }
    }

    CLOCK_SOURCE_CALIBRATED |		
    CLOCK_SOURCE_MUST_VERIFY |		
    CLOCK_SOURCE_WDTEST)
pub static mut clocksource: usize = 0;
#[no_mangle]
unsafe extern "C" fn wdtest_clocksource_reset(which: wdtest_states, percpu: bool) {
    clocksource_unregister(&clocksource_wdtest_ktime);
    pr_info!("Test: State %d percpu %d\n", which, percpu);
    wdtest_state = which;
    if (percpu) {
    wdtest_state |= WDTEST_INJECT_PERCPU;
    }
    wdtest_test_count = 0;
    wdtest_last_ts = 0;
    clocksource_wdtest_ktime.rating = 10;
    clocksource_wdtest_ktime.flags = KTIME_FLAGS;
    if (percpu) {
    clocksource_wdtest_ktime.flags |= CLOCK_SOURCE_WDTEST_PERCPU;
    }
    clocksource_register_khz(&clocksource_wdtest_ktime, 1000 * 1000);
    }
#[no_mangle]
pub unsafe extern "C" fn wdtest_execute(which: wdtest_states, percpu: bool, expect: c_uint, calls: c_ulong) -> bool {
    wdtest_clocksource_reset(which, percpu);
    for (; READ_ONCE(wdtest_test_count) < calls; msleep(100)) {
pub static mut flags: c_uint = 0;
    if (kthread_should_stop()) {
    return false;
    }
    if (flags & CLOCK_SOURCE_UNSTABLE) {
    if (expect & CLOCK_SOURCE_UNSTABLE) {
    return true;
    }
    pr_warn!("Fail: Unexpected unstable\n");
    return false;
    }
    if (flags & CLOCK_SOURCE_VALID_FOR_HRES) {
    if (expect & CLOCK_SOURCE_VALID_FOR_HRES) {
    return true;
    }
    pr_warn!("Fail: Unexpected valid for highres\n");
    return false;
    }
    }
    if (!expect) {
    return true;
    }
    pr_warn!("Fail: Timed out\n");
    return false;
    }
#[no_mangle]
unsafe extern "C" fn wdtest_run(percpu: bool) -> bool {
    if (!wdtest_execute(WDTEST_INJECT_NONE, percpu, CLOCK_SOURCE_VALID_FOR_HRES, 8)) {
    return false;
    }
    if (!wdtest_execute(WDTEST_INJECT_DELAY, percpu, 0, 4)) {
    return false;
    }
    if (!wdtest_execute(WDTEST_INJECT_POSITIVE, percpu, CLOCK_SOURCE_UNSTABLE, 8)) {
    return false;
    }
    if (!wdtest_execute(WDTEST_INJECT_NEGATIVE, percpu, CLOCK_SOURCE_UNSTABLE, 8)) {
    return false;
    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn wdtest_func(arg: *mut c_void) -> c_int {
    clocksource_register_khz(&clocksource_wdtest_ktime, 1000 * 1000);
    if (wdtest_run(false)) {
    if (wdtest_run(true)) {
    pr_info!("Success: All tests passed\n");
    }
    }
    clocksource_unregister(&clocksource_wdtest_ktime);
    if (!IS_MODULE(CONFIG_TEST_CLOCKSOURCE_WATCHDOG)) {
    return 0;
    }
    while (!kthread_should_stop()) {
    schedule_timeout_interruptible(3600 * HZ);
    }
    return 0;
    }
pub static mut wdtest_thread: *mut c_void = core::ptr::null_mut();
#[no_mangle]
unsafe extern "C" fn clocksource_wdtest_init() -> c_int {
    let mut t = kthread_run(wdtest_func, core::ptr::null_mut(), "wdtest");
    if (IS_ERR(t)) {
    pr_warn!("Failed to create wdtest kthread.\n");
    return PTR_ERR(t);
    }
    wdtest_thread = t;
    return 0;
    }
    module_init!(clocksource_wdtest_init);
#[no_mangle]
unsafe extern "C" fn clocksource_wdtest_cleanup() {
    if (wdtest_thread) {
    kthread_stop(wdtest_thread);
    }
    }
    module_exit!(clocksource_wdtest_cleanup);