//! Automatically rewritten from C to Rust
//! Source: kernel/events/hw_breakpoint_test.c
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
// KUnit test for hw_breakpoint constraints accounting logic.
//
// Copyright (C) 2022, Google LLC.
//

    do {										
    if ((slots) > get_test_bp_slots()) {					
    kunit_skip((test), "Requires breakpoint slots: %d > %d", slots,	
    get_test_bp_slots());				
    }									
    } while (0)

pub const MAX_TEST_BREAKPOINTS: c_int = 512;
    static char break_vars[MAX_TEST_BREAKPOINTS];
    static struct perf_event *test_bps[MAX_TEST_BREAKPOINTS];
pub static mut __other_task: *mut c_void = core::ptr::null_mut();
#[no_mangle]
pub unsafe extern "C" fn register_test_bp(cpu: c_int, tsk: *mut task_struct, idx: c_int) -> *mut c_void {
pub static mut attr: perf_event_attr = 0;
    if (WARN_ON!(idx < 0 || idx >= MAX_TEST_BREAKPOINTS)) {
    return core::ptr::null_mut();
    }
    hw_breakpoint_init(&attr);
    attr.bp_addr = (unsigned long)&break_vars[idx];
    attr.bp_len = HW_BREAKPOINT_LEN_1;
    attr.bp_type = HW_BREAKPOINT_RW;
    return perf_event_create_kernel_counter(&attr, cpu, tsk, core::ptr::null_mut(), core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn unregister_test_bp(bp: *mut perf_event) {
    if (WARN_ON!(IS_ERR(*bp))) {
    return;
    }
    if (WARN_ON!(!*bp)) {
    return;
    }
    unregister_hw_breakpoint(*bp);
// bp = NULL;
    }
#[no_mangle]
unsafe extern "C" fn get_test_bp_slots() -> c_int {
    static int slots;
    if (!slots) {
    slots = hw_breakpoint_slots(TYPE_DATA);
    }
    return slots;
    }
#[no_mangle]
unsafe extern "C" fn fill_one_bp_slot(test: *mut kunit, id: *mut c_int, cpu: c_int, tsk: *mut task_struct) {
    let mut bp = register_test_bp(cpu, tsk, *id);
    KUNIT_ASSERT_NOT_NULL(test, bp);
    KUNIT_ASSERT_FALSE(test, IS_ERR(bp));
    KUNIT_ASSERT_NULL(test, test_bps[*id]);
    test_bps[(*id)++] = bp;
    }
//
// Fills up the given @cpu/@tsk with breakpoints, only leaving @skip slots free.
//
// Returns true if this can be called again, continuing at @id.
//
#[no_mangle]
unsafe extern "C" fn fill_bp_slots(test: *mut kunit, id: *mut c_int, cpu: c_int, tsk: *mut task_struct, skip: c_int) -> bool {
    for (int i = 0; i < get_test_bp_slots() - skip; ++i) {
    fill_one_bp_slot(test, id, cpu, tsk);
    }
    return *id + get_test_bp_slots() <= MAX_TEST_BREAKPOINTS;
    }
#[no_mangle]
unsafe extern "C" fn dummy_kthread(arg: *mut c_void) -> c_int {
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn get_other_task(test: *mut kunit) -> *mut c_void {
pub static mut tsk: *mut c_void = core::ptr::null_mut();
    if (__other_task) {
    return __other_task;
    }
    tsk = kthread_create(dummy_kthread, core::ptr::null_mut(), "hw_breakpoint_dummy_task");
    KUNIT_ASSERT_FALSE(test, IS_ERR(tsk));
    __other_task = tsk;
    return __other_task;
    }
#[no_mangle]
unsafe extern "C" fn get_test_cpu(num: c_int) -> c_int {
    let mut cpu = 0;
    WARN_ON!(num < 0);
    for_each_online_cpu(cpu) {
    if (num-- <= 0) {
    break;
    }
    }
    return cpu;
    }
// ===== Test cases =====
#[no_mangle]
unsafe extern "C" fn test_one_cpu(test: *mut kunit) {
pub static mut idx: c_int = 0;
    fill_bp_slots(test, &idx, get_test_cpu(0), core::ptr::null_mut(), 0);
    TEST_EXPECT_NOSPC(register_test_bp(-1, current, idx));
    TEST_EXPECT_NOSPC(register_test_bp(get_test_cpu(0), core::ptr::null_mut(), idx));
    }
#[no_mangle]
unsafe extern "C" fn test_many_cpus(test: *mut kunit) {
pub static mut idx: c_int = 0;
    let mut cpu = 0;
// Test that CPUs are independent.
    for_each_online_cpu(cpu) {
pub static mut do_continue: bool = false;
    TEST_EXPECT_NOSPC(register_test_bp(cpu, core::ptr::null_mut(), idx));
    if (!do_continue) {
    break;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn test_one_task_on_all_cpus(test: *mut kunit) {
pub static mut idx: c_int = 0;
    fill_bp_slots(test, &idx, -1, current, 0);
    TEST_EXPECT_NOSPC(register_test_bp(-1, current, idx));
    TEST_EXPECT_NOSPC(register_test_bp(get_test_cpu(0), current, idx));
    TEST_EXPECT_NOSPC(register_test_bp(get_test_cpu(0), core::ptr::null_mut(), idx));
// Remove one and adding back CPU-target should work.
    unregister_test_bp(&test_bps[0]);
    fill_one_bp_slot(test, &idx, get_test_cpu(0), core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn test_two_tasks_on_all_cpus(test: *mut kunit) {
pub static mut idx: c_int = 0;
// Test that tasks are independent.
    fill_bp_slots(test, &idx, -1, current, 0);
    fill_bp_slots(test, &idx, -1, get_other_task(test), 0);
    TEST_EXPECT_NOSPC(register_test_bp(-1, current, idx));
    TEST_EXPECT_NOSPC(register_test_bp(-1, get_other_task(test), idx));
    TEST_EXPECT_NOSPC(register_test_bp(get_test_cpu(0), current, idx));
    TEST_EXPECT_NOSPC(register_test_bp(get_test_cpu(0), get_other_task(test), idx));
    TEST_EXPECT_NOSPC(register_test_bp(get_test_cpu(0), core::ptr::null_mut(), idx));
// Remove one from first task and adding back CPU-target should not work.
    unregister_test_bp(&test_bps[0]);
    TEST_EXPECT_NOSPC(register_test_bp(get_test_cpu(0), core::ptr::null_mut(), idx));
    }
#[no_mangle]
unsafe extern "C" fn test_one_task_on_one_cpu(test: *mut kunit) {
pub static mut idx: c_int = 0;
    fill_bp_slots(test, &idx, get_test_cpu(0), current, 0);
    TEST_EXPECT_NOSPC(register_test_bp(-1, current, idx));
    TEST_EXPECT_NOSPC(register_test_bp(get_test_cpu(0), current, idx));
    TEST_EXPECT_NOSPC(register_test_bp(get_test_cpu(0), core::ptr::null_mut(), idx));
//
// Remove one and adding back CPU-target should work; this case is
// special vs. above because the task's constraints are CPU-dependent.
//
    unregister_test_bp(&test_bps[0]);
    fill_one_bp_slot(test, &idx, get_test_cpu(0), core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn test_one_task_mixed(test: *mut kunit) {
pub static mut idx: c_int = 0;
    TEST_REQUIRES_BP_SLOTS(test, 3);
    fill_one_bp_slot(test, &idx, get_test_cpu(0), current);
    fill_bp_slots(test, &idx, -1, current, 1);
    TEST_EXPECT_NOSPC(register_test_bp(-1, current, idx));
    TEST_EXPECT_NOSPC(register_test_bp(get_test_cpu(0), current, idx));
    TEST_EXPECT_NOSPC(register_test_bp(get_test_cpu(0), core::ptr::null_mut(), idx));
// Transition from CPU-dependent pinned count to CPU-independent.
    unregister_test_bp(&test_bps[0]);
    unregister_test_bp(&test_bps[1]);
    fill_one_bp_slot(test, &idx, get_test_cpu(0), core::ptr::null_mut());
    fill_one_bp_slot(test, &idx, get_test_cpu(0), core::ptr::null_mut());
    TEST_EXPECT_NOSPC(register_test_bp(get_test_cpu(0), core::ptr::null_mut(), idx));
    }
#[no_mangle]
unsafe extern "C" fn test_two_tasks_on_one_cpu(test: *mut kunit) {
pub static mut idx: c_int = 0;
    fill_bp_slots(test, &idx, get_test_cpu(0), current, 0);
    fill_bp_slots(test, &idx, get_test_cpu(0), get_other_task(test), 0);
    TEST_EXPECT_NOSPC(register_test_bp(-1, current, idx));
    TEST_EXPECT_NOSPC(register_test_bp(-1, get_other_task(test), idx));
    TEST_EXPECT_NOSPC(register_test_bp(get_test_cpu(0), current, idx));
    TEST_EXPECT_NOSPC(register_test_bp(get_test_cpu(0), get_other_task(test), idx));
    TEST_EXPECT_NOSPC(register_test_bp(get_test_cpu(0), core::ptr::null_mut(), idx));
// Can still create breakpoints on some other CPU.
    fill_bp_slots(test, &idx, get_test_cpu(1), core::ptr::null_mut(), 0);
    }
#[no_mangle]
unsafe extern "C" fn test_two_tasks_on_one_all_cpus(test: *mut kunit) {
pub static mut idx: c_int = 0;
    fill_bp_slots(test, &idx, get_test_cpu(0), current, 0);
    fill_bp_slots(test, &idx, -1, get_other_task(test), 0);
    TEST_EXPECT_NOSPC(register_test_bp(-1, current, idx));
    TEST_EXPECT_NOSPC(register_test_bp(-1, get_other_task(test), idx));
    TEST_EXPECT_NOSPC(register_test_bp(get_test_cpu(0), current, idx));
    TEST_EXPECT_NOSPC(register_test_bp(get_test_cpu(0), get_other_task(test), idx));
    TEST_EXPECT_NOSPC(register_test_bp(get_test_cpu(0), core::ptr::null_mut(), idx));
// Cannot create breakpoints on some other CPU either.
    TEST_EXPECT_NOSPC(register_test_bp(get_test_cpu(1), core::ptr::null_mut(), idx));
    }
#[no_mangle]
unsafe extern "C" fn test_task_on_all_and_one_cpu(test: *mut kunit) {
    let mut tsk_on_cpu_idx = 0;
    let mut cpu_idx = 0;
pub static mut idx: c_int = 0;
    TEST_REQUIRES_BP_SLOTS(test, 3);
    fill_bp_slots(test, &idx, -1, current, 2);
// Transitioning from only all CPU breakpoints to mixed.
    tsk_on_cpu_idx = idx;
    fill_one_bp_slot(test, &idx, get_test_cpu(0), current);
    fill_one_bp_slot(test, &idx, -1, current);
    TEST_EXPECT_NOSPC(register_test_bp(-1, current, idx));
    TEST_EXPECT_NOSPC(register_test_bp(get_test_cpu(0), current, idx));
    TEST_EXPECT_NOSPC(register_test_bp(get_test_cpu(0), core::ptr::null_mut(), idx));
// We should still be able to use up another CPU's slots.
    cpu_idx = idx;
    fill_one_bp_slot(test, &idx, get_test_cpu(1), core::ptr::null_mut());
    TEST_EXPECT_NOSPC(register_test_bp(get_test_cpu(1), core::ptr::null_mut(), idx));
// Transitioning back to task target on all CPUs.
    unregister_test_bp(&test_bps[tsk_on_cpu_idx]);
// Still have a CPU target breakpoint in get_test_cpu(1).
    TEST_EXPECT_NOSPC(register_test_bp(-1, current, idx));
// Remove it and try again.
    unregister_test_bp(&test_bps[cpu_idx]);
    fill_one_bp_slot(test, &idx, -1, current);
    TEST_EXPECT_NOSPC(register_test_bp(-1, current, idx));
    TEST_EXPECT_NOSPC(register_test_bp(get_test_cpu(0), current, idx));
    TEST_EXPECT_NOSPC(register_test_bp(get_test_cpu(0), core::ptr::null_mut(), idx));
    TEST_EXPECT_NOSPC(register_test_bp(get_test_cpu(1), core::ptr::null_mut(), idx));
    }
pub static mut kunit_case: usize = 0;
#[no_mangle]
unsafe extern "C" fn test_init(test: *mut kunit) -> c_int {
// Most test cases want 2 distinct CPUs.
    if (num_online_cpus() < 2) {
    kunit_skip(test, "not enough cpus");
    }
// Want the system to not use breakpoints elsewhere.
    if (hw_breakpoint_is_used()) {
    kunit_skip(test, "hw breakpoint already in use");
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_exit(test: *mut kunit) {
    while (i < MAX_TEST_BREAKPOINTS) {
    if (test_bps[i]) {
    unregister_test_bp(&test_bps[i]);
    }
    }
    if (__other_task) {
    kthread_stop(__other_task);
    __other_task = core::ptr::null_mut();
    }
// Verify that internal state agrees that no breakpoints are in use.
    KUNIT_EXPECT_FALSE(test, hw_breakpoint_is_used());
    }
pub static mut kunit_suite: usize = 0;
    kunit_test_suites(&hw_breakpoint_test_suite);
    MODULE_AUTHOR("Marco Elver <elver@google.com>");