//! Automatically rewritten from C to Rust
//! Source: init/calibrate.c
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
macro_rules! printk { ($($tt:tt)*) => { 0 }; }
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
macro_rules! rootfs_initcall { ($($tt:tt)*) => {}; }
macro_rules! device_initcall { ($($tt:tt)*) => {}; }
macro_rules! late_initcall { ($($tt:tt)*) => {}; }
macro_rules! pure_initcall { ($($tt:tt)*) => {}; }
macro_rules! __setup { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_MUTEX { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_SPINLOCK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DECLARE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DEFINE { ($($tt:tt)*) => {}; }
macro_rules! ARRAY_SIZE { ($($tt:tt)*) => { 1 }; }
macro_rules! min_t { ($($tt:tt)*) => { 0 }; }
macro_rules! max_t { ($($tt:tt)*) => { 0 }; }
macro_rules! container_of { ($($tt:tt)*) => { core::ptr::null_mut() }; }
macro_rules! sizeof { ($($tt:tt)*) => { 0usize }; }
macro_rules! MKDEV { ($($tt:tt)*) => { 0u32 }; }
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
macro_rules! pr_warn_once { ($($tt:tt)*) => {}; }
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
pub struct ipc_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_ids { pub _opaque: [u8; 0] }

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
pub type compat_uptr_t = u32;
pub type compat_long_t = i32;
pub type compat_ulong_t = u32;
pub type compat_size_t = u32;
pub type __compat_uid_t = u32;
pub type __compat_gid_t = u32;
pub type compat_mode_t = u32;
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
pub const ENOSYS: c_int = 38;
pub const EIDRM: c_int = 43;
pub const EOPNOTSUPP: c_int = 95;
pub const ENOTSUPP: c_int = 524;
pub const SHMLBA: usize = 4096;
pub const COMPAT_SHMLBA: usize = 4096;

// Standard File Mode Constants
pub const S_IFCHR: u32 = 0x2000;
pub const S_IFDIR: u32 = 0x4000;
pub const S_IFREG: u32 = 0x8000;
pub const S_IFBLK: u32 = 0x6000;
pub const S_IFIFO: u32 = 0x1000;
pub const S_IFLNK: u32 = 0xa000;
pub const S_IFSOCK: u32 = 0xc000;
pub const S_IRWXU: u32 = 0x01c0;
pub const S_IRUSR: u32 = 0x0100;
pub const S_IWUSR: u32 = 0x0080;
pub const S_IXUSR: u32 = 0x0040;
pub const S_IRUGO: u32 = 0x0124;
pub const S_IWUGO: u32 = 0x0092;
pub const S_IXUGO: u32 = 0x0049;

// Standard Memory Constants
pub const PAGE_SHIFT: usize = 12;
pub const PAGE_SIZE: usize = 1 << PAGE_SHIFT;
pub const GFP_KERNEL: c_uint = 0xcc0;
pub const GFP_ATOMIC: c_uint = 0x80000;
pub const GFP_NOWAIT: c_uint = 0;

// Standard Core Primitives
extern "C" {
    pub static current: *mut task_struct;
    pub fn rcu_read_lock();
    pub fn rcu_read_unlock();
    pub fn copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn copy_to_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn kmalloc(size: usize, flags: c_uint) -> *mut c_void;
    pub fn kfree(ptr: *mut c_void);
    pub fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    pub fn usermodehelper_enable();
    pub fn new_encode_dev(dev: u32) -> u32;
}

pub unsafe fn init_mkdir<T>(_path: T, _mode: u32) -> c_int { 0 }
pub unsafe fn init_mknod<T>(_path: T, _mode: u32, _dev: u32) -> c_int { 0 }
// === KERNEL_MACRO_PRELUDE_END ===
macro_rules! kunit_test_init_section_suites { ($($tt:tt)*) => {}; }
macro_rules! kunit_test_suites { ($($tt:tt)*) => {}; }
macro_rules! do_trace_initcall_level { ($($tt:tt)*) => {}; }
macro_rules! do_one_initcall { ($($tt:tt)*) => {}; }
macro_rules! do_initcall_level { ($($tt:tt)*) => {}; }


macro_rules! kunit_test_init_section_suites { ($($tt:tt)*) => {}; }
macro_rules! kunit_test_suites { ($($tt:tt)*) => {}; }
macro_rules! do_trace_initcall_level { ($($tt:tt)*) => {}; }
macro_rules! do_one_initcall { ($($tt:tt)*) => {}; }
macro_rules! do_initcall_level { ($($tt:tt)*) => {}; }







// SPDX-License-Identifier: GPL-2.0
// calibrate.c: default delay calibration
//
// Excised from init/main.c
// Copyright (C) 1991, 1992  Linus Torvalds
//

    pub static mut lpj_fine: usize = 0;
    pub static mut preset_lpj: usize = 0;
#[no_mangle]
unsafe extern "C" fn lpj_setup(str: *mut c_char) -> c_int {
    return kstrtoul(str, 0, &preset_lpj) == 0;
    }
    __setup!("lpj=", lpj_setup);

// This routine uses the delay_read_timer() routine and gets the
// loops per jiffy directly, instead of guessing it using delay().
// Also, this code tries to handle non-maskable asynchronous events
// (like SMIs)
//

pub const MAX_DIRECT_CALIBRATION_RETRIES: c_int = 5;
#[no_mangle]
unsafe extern "C" fn calibrate_delay_direct() -> c_ulong {
pub static mut pre_start: usize = 0;
pub static mut start: usize = 0;
pub static mut post_start: usize = 0;
pub static mut pre_end: usize = 0;
pub static mut end: usize = 0;
pub static mut post_end: usize = 0;
    let mut start_jiffies = 0;
pub static mut timer_rate_min: usize = 0;
pub static mut timer_rate_max: usize = 0;
pub static mut good_timer_sum: c_ulong = 0;
pub static mut good_timer_count: c_ulong = 0;
    let mut measured_times = [0u8; 64];
    let mut max = -1; /* index of measured_times with max/min values or not set */
pub static mut min: c_int = 0;
    let mut i = 0;
    if (!delay_read_timer(&pre_start)) {
    return 0;
    }
//
// A simple loop like
// while ( jiffies < start_jiffies+1)
// start = delay_read_timer();
// will not do. As we don't really know whether jiffy switch
// happened first or timer_value was read first. And some asynchronous
// event can happen between these two events introducing errors in lpj.
//
// So, we do
// 1. pre_start <- When we are sure that jiffy switch hasn't happened
// 2. check jiffy switch
// 3. start <- timer value before or after jiffy switch
// 4. post_start <- When we are sure that jiffy switch has happened
//
// Note, we don't know anything about order of 2 and 3.
// Now, by looking at post_start and pre_start difference, we can
// check whether any asynchronous event happened or not
//
    while (i < MAX_DIRECT_CALIBRATION_RETRIES) {
    pre_start = 0;
    delay_read_timer(&start);
    start_jiffies = jiffies;
    while (time_before_eq(jiffies, start_jiffies + 1)) {
    pre_start = start;
    delay_read_timer(&start);
    }
    delay_read_timer(&post_start);
    pre_end = 0;
    end = post_start;
    while (time_before_eq(jiffies, start_jiffies + 1 +
    DELAY_CALIBRATION_TICKS)) {
    pre_end = end;
    delay_read_timer(&end);
    }
    delay_read_timer(&post_end);
    timer_rate_max = (post_end - pre_start) /
    DELAY_CALIBRATION_TICKS;
    timer_rate_min = (pre_end - post_start) /
    DELAY_CALIBRATION_TICKS;
//
// If the upper limit and lower limit of the timer_rate is
// >= 12.5% apart, redo calibration.
//
    if (start >= post_end) {
    printk!("calibrate_delay_direct() ignoring timer_rate as we had a TSC wrap around start=%lu >=post_end=%lu\n",
    start, post_end);
    }
    if (start < post_end && pre_start != 0 && pre_end != 0 &&
    (timer_rate_max - timer_rate_min) < (timer_rate_max >> 3)) {
    good_timer_count += 1;
    good_timer_sum += timer_rate_max;
    measured_times[i] = timer_rate_max;
    if (max < 0 || timer_rate_max > measured_times[max]) {
    max = i;
    }
    if (min < 0 || timer_rate_max < measured_times[min]) {
    min = i;
    }
    } else {
    measured_times[i] = 0;
    }
    }
//
// Find the maximum & minimum - if they differ too much throw out the
// one with the largest difference from the mean and try again...
//
    while (good_timer_count > 1) {
    let mut estimate = 0;
    let mut maxdiff = 0;
// compute the estimate
    estimate = (good_timer_sum/good_timer_count);
    maxdiff = estimate >> 3;
// if range is within 12% let's take it
    if ((measured_times[max] - measured_times[min]) < maxdiff) {
    return estimate;
    }
// ok - drop the worse value and try again...
    good_timer_sum = 0;
    good_timer_count = 0;
    if ((measured_times[max] - estimate) <
    (estimate - measured_times[min])) {
    printk!("calibrate_delay_direct() dropping min bogoMips estimate %d = %lu\n",
    min, measured_times[min]);
    measured_times[min] = 0;
    min = max;
    } else {
    printk!("calibrate_delay_direct() dropping max bogoMips estimate %d = %lu\n",
    max, measured_times[max]);
    measured_times[max] = 0;
    max = min;
    }
    while (i < MAX_DIRECT_CALIBRATION_RETRIES) {
    if (measured_times[i] == 0) {
    continue;
    }
    good_timer_count += 1;
    good_timer_sum += measured_times[i];
    if (measured_times[i] < measured_times[min]) {
    min = i;
    }
    if (measured_times[i] > measured_times[max]) {
    max = i;
    }
    }
    }
    printk!("calibrate_delay_direct() failed to get a good estimate for loops_per_jiffy.\nProbably due to long platform interrupts. Consider using \"lpj=\" boot option.\n");
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn calibrate_delay_direct() -> c_ulong {
    return 0;
    }

//
// This is the number of bits of precision for the loops_per_jiffy.  Each
// time we refine our estimate after the first takes 1.5/HZ seconds, so try
// to start with a good estimate.
// For the boot cpu we can skip the delay calibration and assign it a value
// calculated based on the timer frequency.
// For the rest of the CPUs we cannot assume that the timer frequency is same as
// the cpu frequency, hence do the calibration for those.
//
pub const LPS_PREC: c_int = 8;
#[no_mangle]
unsafe extern "C" fn calibrate_delay_converge() -> c_ulong {
// First stage - slowly accelerate to find initial bounds
pub static mut lpj: usize = 0;
pub static mut lpj_base: usize = 0;
pub static mut ticks: usize = 0;
pub static mut loopadd: usize = 0;
pub static mut loopadd_base: usize = 0;
pub static mut chop_limit: usize = 0;
pub static mut trials: c_int = 0;
    lpj = (1<<12);
// wait for "start of" clock tick
    ticks = jiffies;
    while (ticks == jiffies) {
    ; /* nothing */
    }
// Go ..
    ticks = jiffies;
    loop {
    if (trial_in_band == (1<<band)) {
    band += 1;
    trial_in_band = 0;
    }
    __delay(lpj * band);
    trials += band;
    break; }
//
// We overshot, so retreat to a clear underestimate. Then estimate
// the largest likely undershoot. This defines our chop bounds.
//
    trials -= band;
    loopadd_base = lpj * band;
    lpj_base = lpj * trials;
// label;
    lpj = lpj_base;
    loopadd = loopadd_base;
//
// Do a binary approximation to get lpj set to
// equal one clock (up to LPS_PREC bits)
//
    chop_limit = lpj >> LPS_PREC;
    while (loopadd > chop_limit) {
    lpj += loopadd;
    ticks = jiffies;
    while (ticks == jiffies) {
    ; /* nothing */
    }
    ticks = jiffies;
    __delay(lpj);
    if (jiffies != ticks)	/* longer than 1 tick */ {
    lpj -= loopadd;
    }
    loopadd >>= 1;
    }
//
// If we incremented every single time possible, presume we've
// massively underestimated initially, and retry with a higher
// start, and larger range. (Only seen on x86_64, due to SMIs)
//
    if (lpj + loopadd * 2 == lpj_base + loopadd_base * 2) {
    lpj_base = lpj;
    loopadd_base <<= 2;
// goto;
    }
    return lpj;
    }
pub static mut unsigned: usize = 0;
//
// Check if cpu calibration delay is already known. For example,
// some processors with multi-core sockets may have all cores
// with the same calibration delay.
//
// Architectures should override this function if a faster calibration
// method is available.
//

//
// Indicate the cpu delay calibration is done. This can be used by
// architectures to stop accepting delay timer registrations after this point.
//
#[no_mangle]
#[no_mangle]
// duplicate fn: __attribute__

#[no_mangle]
pub unsafe extern "C" fn calibrate_delay() {
    let mut lpj = 0;
pub static mut printed: usize = 0;
pub static mut this_cpu: c_int = 0;
    if (per_cpu(cpu_loops_per_jiffy, this_cpu)) {
    lpj = per_cpu(cpu_loops_per_jiffy, this_cpu);
    if (!printed) {
    pr_info!("Calibrating delay loop (skipped) already calibrated this CPU");
    }
    } else if (preset_lpj) {
    lpj = preset_lpj;
    if (!printed) {
    pr_info!("Calibrating delay loop (skipped) preset value.. ");
    }
    } else if ((!printed) && lpj_fine) {
    lpj = lpj_fine;
    pr_info!("Calibrating delay loop (skipped), value calculated using timer frequency.. ");
    } else if ((lpj = calibrate_delay_is_known())) {
    ;
    } else if ((lpj = calibrate_delay_direct()) != 0) {
    if (!printed) {
    pr_info!("Calibrating delay using timer specific routine.. ");
    }
    } else {
    if (!printed) {
    pr_info!("Calibrating delay loop... ");
    }
    lpj = calibrate_delay_converge();
    }
    per_cpu(cpu_loops_per_jiffy, this_cpu) = lpj;
    if (!printed) {
    pr_cont("%lu.%02lu BogoMIPS (lpj=%lu)\n",
    lpj/(500000/HZ),
    (lpj/(5000/HZ)) % 100, lpj);
    }
    loops_per_jiffy = lpj;
    printed = true;
    calibration_delay_done();
    }