//! Automatically rewritten from C to Rust
//! Source: kernel/power/suspend_test.c
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
// kernel/power/suspend_test.c - Suspend to RAM and standby test facility.
//
// Copyright (c) 2009 Pavel Machek <pavel@ucw.cz>
//

//
// We test the system suspend code by setting an RTC wakealarm a short
// time in the future, then suspending.  Suspending the devices won't
// normally take long ... some systems only need a few milliseconds.
//
// The time it takes is system-specific though, so when we test this
// during system bootup we allow a LOT of time.
//
pub const TEST_SUSPEND_SECONDS: c_int = 10;
    static unsigned long suspend_test_start_time;
pub static mut test_repeat_count_max: u32 = 1;
    static u32 test_repeat_count_current;
#[no_mangle]
pub unsafe extern "C" fn suspend_test_start() {
// FIXME Use better timebase than "jiffies", ideally a clocksource.
// What we want is a hardware counter that will work correctly even
// during the irqs-are-off stages of the suspend/resume cycle...
//
    suspend_test_start_time = jiffies;
    }
#[no_mangle]
pub unsafe extern "C" fn suspend_test_finish(label: *const c_char) {
pub static mut nj: c_long = 0;
    let mut msec: c_uint = 0;
    msec = jiffies_to_msecs(abs(nj));
    pr_info!("PM: %s took %d.%03d seconds\n", label,
    msec / 1000, msec % 1000);
// Warning on suspend means the RTC alarm period needs to be
// larger -- the system was sooo slooowwww to suspend that the
// alarm (should have) fired before the system went to sleep!
//
// Warning on either suspend or resume also means the system
// has some performance issues.  The stack dump of a WARN_ON
// is more likely to get the right attention than a printk...
//
    WARN(msec > (TEST_SUSPEND_SECONDS * 1000),
    "Component: %s, time: %u\n", label, msec);
    }
//
// To test system suspend, we need a hands-off mechanism to resume the
// system.  RTCs wake alarms are a common self-contained mechanism.
//
#[no_mangle]
unsafe extern "C" fn test_wakealarm(rtc: *mut rtc_device, state: suspend_state_t)  {
    static char err_readtime[] __initdata =
    "PM: can't read %s time, err %d\n";
    static char err_wakealarm [] __initdata =
    "PM: can't set %s wakealarm, err %d\n";
    static char err_suspend[] __initdata =
    "PM: suspend test failed, error %d\n";
    static char info_test[] __initdata =
    "PM: test RTC wakeup from '%s' suspend\n";
    let mut now;
pub static mut alm: usize = 0;
    let mut status = 0;
// this may fail if the RTC hasn't been initialized
// label;
    status = rtc_read_time(rtc, &alm.time);
    if (status < 0) {
    printk(err_readtime, dev_name(&rtc.dev), status);
    return;
    }
    now = rtc_tm_to_time64(&alm.time);
    memset(&alm, 0, sizeof alm);
    rtc_time64_to_tm(now + TEST_SUSPEND_SECONDS, &alm.time);
    alm.enabled = true;
    status = rtc_set_alarm(rtc, &alm);
    if (status < 0) {
    printk(err_wakealarm, dev_name(&rtc.dev), status);
    return;
    }
    if (state == PM_SUSPEND_MEM) {
    printk(info_test, pm_states[state]);
    status = pm_suspend(state);
    if (status == -ENODEV) {
    state = PM_SUSPEND_STANDBY;
    }
    }
    if (state == PM_SUSPEND_STANDBY) {
    printk(info_test, pm_states[state]);
    status = pm_suspend(state);
    if (status < 0) {
    state = PM_SUSPEND_TO_IDLE;
    }
    }
    if (state == PM_SUSPEND_TO_IDLE) {
    printk(info_test, pm_states[state]);
    status = pm_suspend(state);
    }
    if (status < 0) {
    printk(err_suspend, status);
    }
    test_repeat_count_current += 1;
    if (test_repeat_count_current < test_repeat_count_max) {
// goto;
    }
// Some platforms can't detect that the alarm triggered the
// wakeup, or (accordingly) disable it after it afterwards.
// It's supposed to give oneshot behavior; cope.
//
    alm.enabled = false;
    rtc_set_alarm(rtc, &alm);
    }
#[no_mangle]
unsafe extern "C" fn has_wakealarm(dev: *mut device, data: *const c_void) -> c_int {
    let mut candidate = to_rtc_device(dev);
    if (!test_bit(RTC_FEATURE_ALARM, candidate.features)) {
    return 0;
    }
    if (!device_may_wakeup(candidate.dev.parent)) {
    return 0;
    }
    return 1;
    }
//
// Kernel options like "test_suspend=mem" force suspend/resume sanity tests
// at startup time.  They're normally disabled, for faster boot and because
// we can't know which states really work on this particular system.
//
pub static mut test_state_label: *mut c_void = core::ptr::null_mut();
    static char warn_bad_state[] __initdata =
    "PM: can't test '%s' suspend state\n";
#[no_mangle]
unsafe extern "C" fn setup_test_suspend(value: *mut c_char) -> c_int {
    let mut i = 0;
pub static mut repeat: *mut c_void = core::ptr::null_mut();
pub static mut suspend_type: *mut c_void = core::ptr::null_mut();
// example : "=mem[,N]" ==> "mem[,N]"
    value += 1;
    suspend_type = strsep(&value, ",");
    if (!suspend_type) {
    return 1;
    }
    repeat = strsep(&value, ",");
    if (repeat) {
    if (kstrtou32(repeat, 0, &test_repeat_count_max)) {
    return 1;
    }
    }
    for (i = PM_SUSPEND_MIN; i < PM_SUSPEND_MAX; i++) {
    if (!strcmp(pm_labels[i], suspend_type)) {
    }
    test_state_label = pm_labels[i];
    return 1;
    }
    printk(warn_bad_state, suspend_type);
    return 1;
    }
    __setup!("test_suspend", setup_test_suspend);
#[no_mangle]
unsafe extern "C" fn test_suspend() -> c_int {
    static char		warn_no_rtc[] __initdata =
    "PM: no wakealarm-capable RTC driver is ready\n";
    let mut rtc = core::ptr::null_mut();
pub static mut dev: *mut c_void = core::ptr::null_mut();
    let mut test_state;
// PM is initialized by now; is that state testable?
    if (!test_state_label) {
    return 0;
    }
    while (test_state < PM_SUSPEND_MAX) {
    let mut state_label = pm_states[test_state];
    if (state_label && !strcmp(test_state_label, state_label)) {
    break;
    }
    }
    if (test_state == PM_SUSPEND_MAX) {
    printk(warn_bad_state, test_state_label);
    return 0;
    }
// RTCs have initialized by now too ... can we use one?
    dev = class_find_device(&rtc_class, core::ptr::null_mut(), core::ptr::null_mut(), has_wakealarm);
    if (dev) {
    rtc = rtc_class_open(dev_name(dev));
    put_device(dev);
    }
    if (!rtc) {
    printk(warn_no_rtc);
    return 0;
    }
// go for it
    test_wakealarm(rtc, test_state);
    rtc_class_close(rtc);
    return 0;
    }
    late_initcall!(test_suspend);