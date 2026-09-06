//! Automatically rewritten from C to Rust
//! Source: mm/kasan/kasan_test_c.c
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
// Copyright (c) 2014 Samsung Electronics Co., Ltd.
// Author: Andrey Ryabinin <a.ryabinin@samsung.com>
//

    MODULE_IMPORT_NS("EXPORTED_FOR_KUNIT_TESTING");
    static bool multishot;
// Fields set based on lines observed in the console.
    static struct {
    let mut report_found = 0;
    let mut async_fault = 0;
    } test_status;
//
// Some tests use these global variables to store return values from function
// calls that could otherwise be eliminated by the compiler as dead code.
//
    static void *volatile kasan_ptr_result;
    static volatile int kasan_int_result;
// Probe for console output: obtains test_status lines of interest.
#[no_mangle]
unsafe extern "C" fn probe_console(ignore: *mut c_void, buf: *const c_char, len: usize) {
    if (strnstr(buf, "BUG: KASAN: ", len)) {
    WRITE_ONCE(test_status.report_found, true);
    }

    else if (strnstr(buf, "Asynchronous fault: ", len)) {
    WRITE_ONCE(test_status.async_fault, true);
    }
    }
#[no_mangle]
unsafe extern "C" fn kasan_suite_init(suite: *mut kunit_suite) -> c_int {
    if (!kasan_enabled()) {
    pr_err!("Can't run KASAN tests with KASAN disabled");
    return -1;
    }
// Stop failing KUnit tests on KASAN reports.
    kasan_kunit_test_suite_start();
//
// Temporarily enable multi-shot mode. Otherwise, KASAN would only
// report the first detected bug and panic the kernel if panic_on_warn
// is enabled.
//
    multishot = kasan_save_enable_multi_shot();
    register_trace_console(probe_console, core::ptr::null_mut());
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn kasan_suite_exit(suite: *mut kunit_suite) {
    kasan_kunit_test_suite_end();
    kasan_restore_multi_shot(multishot);
    unregister_trace_console(probe_console, core::ptr::null_mut());
    tracepoint_synchronize_unregister();
    }
#[no_mangle]
unsafe extern "C" fn kasan_test_exit(test: *mut kunit) {
    KUNIT_EXPECT_FALSE(test, READ_ONCE(test_status.report_found));
    }
//
// KUNIT_EXPECT_KASAN_RESULT - checks whether the executed expression
// produces a KASAN report; causes a KUnit test failure when the result
// is different from @fail.
//
// @test: Currently executing KUnit test.
// @expr: Expression to be tested.
// @expr_str: Expression to be tested encoded as a string.
// @fail: Whether expression should produce a KASAN report.
//
// For hardware tag-based KASAN, when a synchronous tag fault happens, tag
// checking is auto-disabled. When this happens, this test handler reenables
// tag checking. As tag checking can be only disabled or enabled per CPU,
// this handler disables migration (preemption).
//
// Since the compiler doesn't see that the expression can change the test_status
// fields, it can reorder or optimize away the accesses to those fields.
// Use READ/WRITE_ONCE() for the accesses and compiler barriers around the
// expression to prevent that.
//
// In between KUNIT_EXPECT_KASAN_RESULT checks, test_status.report_found is kept
// as false. This allows detecting KASAN reports that happen outside of the
// checks by asserting !test_status.report_found at the start of
// KUNIT_EXPECT_KASAN_RESULT and in kasan_test_exit.
//

    do {									
    if (IS_ENABLED!(CONFIG_KASAN_HW_TAGS) &&				
    kasan_sync_fault_possible())				 {
    migrate_disable();					
    }
    KUNIT_EXPECT_FALSE(test, READ_ONCE(test_status.report_found));	
    barrier();							
    expr;								
    barrier();							
    if (kasan_async_fault_possible())				 {
    kasan_force_async_fault();				
    }
    if (READ_ONCE(test_status.report_found) != fail) {		
    KUNIT_FAIL(test, KUNIT_SUBTEST_INDENT "KASAN failure"	
    "%sexpected in \"" expr_str		
    "\", but %soccurred",			
    (fail ? " " : " not "),		
    (test_status.report_found ?		
    "" : "none "));			
    }								
    if (IS_ENABLED!(CONFIG_KASAN_HW_TAGS) &&				
    kasan_sync_fault_possible()) {				
    if (READ_ONCE(test_status.report_found) &&		
    !READ_ONCE(test_status.async_fault))		 {
    kasan_enable_hw_tags();				
    }
    migrate_enable();					
    }								
    WRITE_ONCE(test_status.report_found, false);			
    WRITE_ONCE(test_status.async_fault, false);			
    } while (0)
//
// KUNIT_EXPECT_KASAN_FAIL - check that the executed expression produces a
// KASAN report; causes a KUnit test failure otherwise.
//
// @test: Currently executing KUnit test.
// @expr: Expression that must produce a KASAN report.
//

    KUNIT_EXPECT_KASAN_RESULT(test, expr, #expr, true)
//
// KUNIT_EXPECT_KASAN_FAIL_READ - check that the executed expression
// produces a KASAN report when the write-only mode is not enabled;
// causes a KUnit test failure otherwise.
//
// Note: At the moment, this macro does not check whether the produced
// KASAN report is a report about a bad read access. It is only intended
// for checking the write-only KASAN mode functionality without failing
// KASAN tests.
//
// @test: Currently executing KUnit test.
// @expr: Expression that must only produce a KASAN report
// when the write-only mode is not enabled.
//

    KUNIT_EXPECT_KASAN_RESULT(test, expr, #expr,			
    !kasan_write_only_enabled())			

    if (!IS_ENABLED!(config))					 {
    kunit_skip((test), "Test requires " #config "=y");	
    }
    } while (0)

    if (IS_ENABLED!(config))						 {
    kunit_skip((test), "Test requires " #config "=n");	
    }
    } while (0)

    if (IS_ENABLED!(CONFIG_KASAN_HW_TAGS))				 {
    break;  /* No compiler instrumentation. */		
    }
    if (IS_ENABLED!(CONFIG_CC_HAS_KASAN_MEMINTRINSIC_PREFIX))	 {
    break;  /* Should always be instrumented! */		
    }
    if (IS_ENABLED!(CONFIG_GENERIC_ENTRY))				 {
    kunit_skip((test), "Test requires checked mem*()");	
    }
    } while (0)
#[no_mangle]
unsafe extern "C" fn kmalloc_oob_right(test: *mut kunit) {
pub static mut ptr: *mut c_void = core::ptr::null_mut();
pub static mut size: usize = 0;
    ptr = kmalloc(size, GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ptr);
    OPTIMIZER_HIDE_VAR(ptr);
//
// An unaligned access past the requested kmalloc size.
// Only generic KASAN can precisely detect these.
//
    if (IS_ENABLED!(CONFIG_KASAN_GENERIC)) {
    KUNIT_EXPECT_KASAN_FAIL(test, ptr[size] = 'x');
    }
//
// An aligned access into the first out-of-bounds granule that falls
// within the aligned kmalloc object.
//
    KUNIT_EXPECT_KASAN_FAIL(test, ptr[size + 5] = 'y');
// Out-of-bounds access past the aligned kmalloc object.
    KUNIT_EXPECT_KASAN_FAIL_READ(test, ptr[0] =
    ptr[size + KASAN_GRANULE_SIZE + 5]);
    kfree(ptr);
    }
#[no_mangle]
unsafe extern "C" fn kmalloc_oob_left(test: *mut kunit) {
pub static mut ptr: *mut c_void = core::ptr::null_mut();
pub static mut size: usize = 15;
    ptr = kmalloc(size, GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ptr);
    OPTIMIZER_HIDE_VAR(ptr);
    KUNIT_EXPECT_KASAN_FAIL_READ(test, *ptr = *(ptr - 1));
    kfree(ptr);
    }
#[no_mangle]
unsafe extern "C" fn kmalloc_node_oob_right(test: *mut kunit) {
pub static mut ptr: *mut c_void = core::ptr::null_mut();
pub static mut size: usize = 4096;
    ptr = kmalloc_node(size, GFP_KERNEL, 0);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ptr);
    OPTIMIZER_HIDE_VAR(ptr);
    KUNIT_EXPECT_KASAN_FAIL_READ(test, ptr[0] = ptr[size]);
    kfree(ptr);
    }
#[no_mangle]
unsafe extern "C" fn kmalloc_track_caller_oob_right(test: *mut kunit) {
pub static mut ptr: *mut c_void = core::ptr::null_mut();
pub static mut size: usize = 0;
//
// Check that KASAN detects out-of-bounds access for object allocated via
// kmalloc_track_caller().
//
    ptr = kmalloc_track_caller(size, GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ptr);
    OPTIMIZER_HIDE_VAR(ptr);
    KUNIT_EXPECT_KASAN_FAIL(test, ptr[size] = 'y');
    kfree(ptr);
//
// Check that KASAN detects out-of-bounds access for object allocated via
// kmalloc_node_track_caller().
//
    ptr = kmalloc_node_track_caller(size, GFP_KERNEL, 0);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ptr);
    OPTIMIZER_HIDE_VAR(ptr);
    KUNIT_EXPECT_KASAN_FAIL(test, ptr[size] = 'y');
    kfree(ptr);
    }
//
// Check that KASAN detects an out-of-bounds access for a big object allocated
// via kmalloc(). But not as big as to trigger the page_alloc fallback.
//
#[no_mangle]
unsafe extern "C" fn kmalloc_big_oob_right(test: *mut kunit) {
pub static mut ptr: *mut c_void = core::ptr::null_mut();
pub static mut size: usize = 0;
    ptr = kmalloc(size, GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ptr);
    OPTIMIZER_HIDE_VAR(ptr);
    KUNIT_EXPECT_KASAN_FAIL(test, ptr[size] = 0);
    kfree(ptr);
    }
//
// The kmalloc_large_* tests below use kmalloc() to allocate a memory chunk
// that does not fit into the largest slab cache and therefore is allocated via
// the page_alloc fallback.
//
#[no_mangle]
unsafe extern "C" fn kmalloc_large_oob_right(test: *mut kunit) {
pub static mut ptr: *mut c_void = core::ptr::null_mut();
pub static mut size: usize = 0;
    ptr = kmalloc(size, GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ptr);
    OPTIMIZER_HIDE_VAR(ptr);
    KUNIT_EXPECT_KASAN_FAIL(test, ptr[size + OOB_TAG_OFF] = 0);
    kfree(ptr);
    }
#[no_mangle]
unsafe extern "C" fn kmalloc_large_uaf(test: *mut kunit) {
pub static mut ptr: *mut c_void = core::ptr::null_mut();
pub static mut size: usize = 0;
    ptr = kmalloc(size, GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ptr);
    kfree(ptr);
    KUNIT_EXPECT_KASAN_FAIL_READ(test, (ptr)[0]);
    }
#[no_mangle]
unsafe extern "C" fn kmalloc_large_invalid_free(test: *mut kunit) {
pub static mut ptr: *mut c_void = core::ptr::null_mut();
pub static mut size: usize = 0;
    ptr = kmalloc(size, GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ptr);
    KUNIT_EXPECT_KASAN_FAIL(test, kfree(ptr + 1));
    }
#[no_mangle]
unsafe extern "C" fn page_alloc_oob_right(test: *mut kunit) {
pub static mut ptr: *mut c_void = core::ptr::null_mut();
pub static mut pages: *mut c_void = core::ptr::null_mut();
pub static mut order: usize = 4;
pub static mut size: usize = 0;
//
// With generic KASAN page allocations have no redzones, thus
// out-of-bounds detection is not guaranteed.
// See https://bugzilla.kernel.org/show_bug.cgi?id=210503.
//
    KASAN_TEST_NEEDS_CONFIG_OFF(test, CONFIG_KASAN_GENERIC);
    pages = alloc_pages(GFP_KERNEL, order);
    ptr = page_address(pages);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ptr);
    KUNIT_EXPECT_KASAN_FAIL_READ(test, ptr[0] = ptr[size]);
    free_pages((unsigned long)ptr, order);
    }
#[no_mangle]
unsafe extern "C" fn page_alloc_uaf(test: *mut kunit) {
pub static mut ptr: *mut c_void = core::ptr::null_mut();
pub static mut pages: *mut c_void = core::ptr::null_mut();
pub static mut order: usize = 4;
    pages = alloc_pages(GFP_KERNEL, order);
    ptr = page_address(pages);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ptr);
    free_pages((unsigned long)ptr, order);
    KUNIT_EXPECT_KASAN_FAIL_READ(test, (ptr)[0]);
    }
#[no_mangle]
pub unsafe extern "C" fn krealloc_more_oob_helper(test: *mut kunit, size1: size_t, size2: size_t) {
    let mut ptr1 = core::ptr::null_mut();
    let mut ptr2 = core::ptr::null_mut();
    let mut middle = 0;
    KUNIT_ASSERT_LT(test, size1, size2);
    middle = size1 + (size2 - size1) / 2;
    ptr1 = kmalloc(size1, GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ptr1);
    ptr2 = krealloc(ptr1, size2, GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ptr2);
// Suppress -Warray-bounds warnings.
    OPTIMIZER_HIDE_VAR(ptr2);
// All offsets up to size2 must be accessible.
    ptr2[size1 - 1] = 'x';
    ptr2[size1] = 'x';
    ptr2[middle] = 'x';
    ptr2[size2 - 1] = 'x';
// Generic mode is precise, so unaligned size2 must be inaccessible.
    if (IS_ENABLED!(CONFIG_KASAN_GENERIC)) {
    KUNIT_EXPECT_KASAN_FAIL(test, ptr2[size2] = 'x');
    }
// For all modes first aligned offset after size2 must be inaccessible.
    KUNIT_EXPECT_KASAN_FAIL(test,
    ptr2[round_up(size2, KASAN_GRANULE_SIZE)] = 'x');
    kfree(ptr2);
    }
#[no_mangle]
pub unsafe extern "C" fn krealloc_less_oob_helper(test: *mut kunit, size1: size_t, size2: size_t) {
    let mut ptr1 = core::ptr::null_mut();
    let mut ptr2 = core::ptr::null_mut();
    let mut middle = 0;
    KUNIT_ASSERT_LT(test, size2, size1);
    middle = size2 + (size1 - size2) / 2;
    ptr1 = kmalloc(size1, GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ptr1);
    ptr2 = krealloc(ptr1, size2, GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ptr2);
// Suppress -Warray-bounds warnings.
    OPTIMIZER_HIDE_VAR(ptr2);
// Must be accessible for all modes.
    ptr2[size2 - 1] = 'x';
// Generic mode is precise, so unaligned size2 must be inaccessible.
    if (IS_ENABLED!(CONFIG_KASAN_GENERIC)) {
    KUNIT_EXPECT_KASAN_FAIL(test, ptr2[size2] = 'x');
    }
// For all modes first aligned offset after size2 must be inaccessible.
    KUNIT_EXPECT_KASAN_FAIL(test,
    ptr2[round_up(size2, KASAN_GRANULE_SIZE)] = 'x');
//
// For all modes all size2, middle, and size1 should land in separate
// granules and thus the latter two offsets should be inaccessible.
//
    KUNIT_EXPECT_LE(test, round_up(size2, KASAN_GRANULE_SIZE),
    round_down(middle, KASAN_GRANULE_SIZE));
    KUNIT_EXPECT_LE(test, round_up(middle, KASAN_GRANULE_SIZE),
    round_down(size1, KASAN_GRANULE_SIZE));
    KUNIT_EXPECT_KASAN_FAIL(test, ptr2[middle] = 'x');
    KUNIT_EXPECT_KASAN_FAIL(test, ptr2[size1 - 1] = 'x');
    KUNIT_EXPECT_KASAN_FAIL(test, ptr2[size1] = 'x');
    kfree(ptr2);
    }
#[no_mangle]
unsafe extern "C" fn krealloc_more_oob(test: *mut kunit) {
    krealloc_more_oob_helper(test, 201, 235);
    }
#[no_mangle]
unsafe extern "C" fn krealloc_less_oob(test: *mut kunit) {
    krealloc_less_oob_helper(test, 235, 201);
    }
#[no_mangle]
unsafe extern "C" fn krealloc_large_more_oob(test: *mut kunit) {
    krealloc_more_oob_helper(test, KMALLOC_MAX_CACHE_SIZE + 201,
    KMALLOC_MAX_CACHE_SIZE + 235);
    }
#[no_mangle]
unsafe extern "C" fn krealloc_large_less_oob(test: *mut kunit) {
    krealloc_less_oob_helper(test, KMALLOC_MAX_CACHE_SIZE + 235,
    KMALLOC_MAX_CACHE_SIZE + 201);
    }
//
// Check that krealloc() detects a use-after-free, returns NULL,
// and doesn't unpoison the freed object.
//
#[no_mangle]
unsafe extern "C" fn krealloc_uaf(test: *mut kunit) {
    let mut ptr1 = core::ptr::null_mut();
    let mut ptr2 = core::ptr::null_mut();
pub static mut size1: c_int = 201;
pub static mut size2: c_int = 235;
    ptr1 = kmalloc(size1, GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ptr1);
    kfree(ptr1);
    KUNIT_EXPECT_KASAN_FAIL(test, ptr2 = krealloc(ptr1, size2, GFP_KERNEL));
    KUNIT_ASSERT_NULL(test, ptr2);
    KUNIT_EXPECT_KASAN_FAIL_READ(test, *ptr1);
    }
#[no_mangle]
unsafe extern "C" fn kmalloc_oob_16(test: *mut kunit) {
    struct {
    u64 words[2];
    } *ptr1, *ptr2;
    KASAN_TEST_NEEDS_CHECKED_MEMINTRINSICS(test);
// This test is specifically crafted for the generic mode.
    KASAN_TEST_NEEDS_CONFIG_ON(test, CONFIG_KASAN_GENERIC);
// RELOC_HIDE to prevent gcc from warning about short alloc
    ptr1 = RELOC_HIDE(kmalloc(sizeof!(*ptr1) - 3, GFP_KERNEL), 0);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ptr1);
    ptr2 = kmalloc_obj(*ptr2);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ptr2);
    OPTIMIZER_HIDE_VAR(ptr1);
    OPTIMIZER_HIDE_VAR(ptr2);
    KUNIT_EXPECT_KASAN_FAIL(test, *ptr1 = *ptr2);
    kfree(ptr1);
    kfree(ptr2);
    }
#[no_mangle]
unsafe extern "C" fn kmalloc_uaf_16(test: *mut kunit) {
    struct {
    u64 words[2];
    } *ptr1, *ptr2;
    KASAN_TEST_NEEDS_CHECKED_MEMINTRINSICS(test);
    ptr1 = kmalloc_obj(*ptr1);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ptr1);
    ptr2 = kmalloc_obj(*ptr2);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ptr2);
    kfree(ptr2);
    KUNIT_EXPECT_KASAN_FAIL_READ(test, *ptr1 = *ptr2);
    kfree(ptr1);
    }
//
// Note: in the memset tests below, the written range touches both valid and
// invalid memory. This makes sure that the instrumentation does not only check
// the starting address but the whole range.
//
#[no_mangle]
unsafe extern "C" fn kmalloc_oob_memset_2(test: *mut kunit) {
pub static mut ptr: *mut c_void = core::ptr::null_mut();
pub static mut size: usize = 0;
pub static mut memset_size: usize = 2;
    KASAN_TEST_NEEDS_CHECKED_MEMINTRINSICS(test);
    ptr = kmalloc(size, GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ptr);
    OPTIMIZER_HIDE_VAR(ptr);
    OPTIMIZER_HIDE_VAR(size);
    OPTIMIZER_HIDE_VAR(memset_size);
    KUNIT_EXPECT_KASAN_FAIL(test, memset(ptr + size - 1, 0, memset_size));
    kfree(ptr);
    }
#[no_mangle]
unsafe extern "C" fn kmalloc_oob_memset_4(test: *mut kunit) {
pub static mut ptr: *mut c_void = core::ptr::null_mut();
pub static mut size: usize = 0;
pub static mut memset_size: usize = 4;
    KASAN_TEST_NEEDS_CHECKED_MEMINTRINSICS(test);
    ptr = kmalloc(size, GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ptr);
    OPTIMIZER_HIDE_VAR(ptr);
    OPTIMIZER_HIDE_VAR(size);
    OPTIMIZER_HIDE_VAR(memset_size);
    KUNIT_EXPECT_KASAN_FAIL(test, memset(ptr + size - 3, 0, memset_size));
    kfree(ptr);
    }
#[no_mangle]
unsafe extern "C" fn kmalloc_oob_memset_8(test: *mut kunit) {
pub static mut ptr: *mut c_void = core::ptr::null_mut();
pub static mut size: usize = 0;
pub static mut memset_size: usize = 8;
    KASAN_TEST_NEEDS_CHECKED_MEMINTRINSICS(test);
    ptr = kmalloc(size, GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ptr);
    OPTIMIZER_HIDE_VAR(ptr);
    OPTIMIZER_HIDE_VAR(size);
    OPTIMIZER_HIDE_VAR(memset_size);
    KUNIT_EXPECT_KASAN_FAIL(test, memset(ptr + size - 7, 0, memset_size));
    kfree(ptr);
    }
#[no_mangle]
unsafe extern "C" fn kmalloc_oob_memset_16(test: *mut kunit) {
pub static mut ptr: *mut c_void = core::ptr::null_mut();
pub static mut size: usize = 0;
pub static mut memset_size: usize = 16;
    KASAN_TEST_NEEDS_CHECKED_MEMINTRINSICS(test);
    ptr = kmalloc(size, GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ptr);
    OPTIMIZER_HIDE_VAR(ptr);
    OPTIMIZER_HIDE_VAR(size);
    OPTIMIZER_HIDE_VAR(memset_size);
    KUNIT_EXPECT_KASAN_FAIL(test, memset(ptr + size - 15, 0, memset_size));
    kfree(ptr);
    }
#[no_mangle]
unsafe extern "C" fn kmalloc_oob_in_memset(test: *mut kunit) {
pub static mut ptr: *mut c_void = core::ptr::null_mut();
pub static mut size: usize = 0;
    KASAN_TEST_NEEDS_CHECKED_MEMINTRINSICS(test);
    ptr = kmalloc(size, GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ptr);
    OPTIMIZER_HIDE_VAR(ptr);
    OPTIMIZER_HIDE_VAR(size);
    KUNIT_EXPECT_KASAN_FAIL(test,
    memset(ptr, 0, size + KASAN_GRANULE_SIZE));
    kfree(ptr);
    }
#[no_mangle]
unsafe extern "C" fn kmalloc_memmove_negative_size(test: *mut kunit) {
pub static mut ptr: *mut c_void = core::ptr::null_mut();
pub static mut size: usize = 64;
pub static mut invalid_size: usize = 0;
    KASAN_TEST_NEEDS_CHECKED_MEMINTRINSICS(test);
//
// Hardware tag-based mode doesn't check memmove for negative size.
// As a result, this test introduces a side-effect memory corruption,
// which can result in a crash.
//
    KASAN_TEST_NEEDS_CONFIG_OFF(test, CONFIG_KASAN_HW_TAGS);
    ptr = kmalloc(size, GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ptr);
    memset(ptr, 0, 64);
    OPTIMIZER_HIDE_VAR(ptr);
    OPTIMIZER_HIDE_VAR(invalid_size);
    KUNIT_EXPECT_KASAN_FAIL(test,
    memmove(ptr, ptr + 4, invalid_size));
    kfree(ptr);
    }
#[no_mangle]
unsafe extern "C" fn kmalloc_memmove_invalid_size(test: *mut kunit) {
pub static mut ptr: *mut c_void = core::ptr::null_mut();
pub static mut size: usize = 64;
pub static mut invalid_size: usize = 0;
    KASAN_TEST_NEEDS_CHECKED_MEMINTRINSICS(test);
    ptr = kmalloc(size, GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ptr);
    memset(ptr, 0, 64);
    OPTIMIZER_HIDE_VAR(ptr);
    OPTIMIZER_HIDE_VAR(invalid_size);
    KUNIT_EXPECT_KASAN_FAIL_READ(test,
    memmove(ptr, ptr + 4, invalid_size));
    kfree(ptr);
    }
#[no_mangle]
unsafe extern "C" fn kmalloc_uaf(test: *mut kunit) {
pub static mut ptr: *mut c_void = core::ptr::null_mut();
pub static mut size: usize = 10;
    ptr = kmalloc(size, GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ptr);
    kfree(ptr);
    KUNIT_EXPECT_KASAN_FAIL_READ(test, (ptr)[8]);
    }
#[no_mangle]
unsafe extern "C" fn kmalloc_uaf_memset(test: *mut kunit) {
pub static mut ptr: *mut c_void = core::ptr::null_mut();
pub static mut size: usize = 33;
    KASAN_TEST_NEEDS_CHECKED_MEMINTRINSICS(test);
//
// Only generic KASAN uses quarantine, which is required to avoid a
// kernel memory corruption this test causes.
//
    KASAN_TEST_NEEDS_CONFIG_ON(test, CONFIG_KASAN_GENERIC);
    ptr = kmalloc(size, GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ptr);
    kfree(ptr);
    KUNIT_EXPECT_KASAN_FAIL(test, memset(ptr, 0, size));
    }
#[no_mangle]
unsafe extern "C" fn kmalloc_uaf2(test: *mut kunit) {
    let mut ptr1 = core::ptr::null_mut();
    let mut ptr2 = core::ptr::null_mut();
pub static mut size: usize = 43;
pub static mut counter: c_int = 0;
// label;
    ptr1 = kmalloc(size, GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ptr1);
    kfree(ptr1);
    ptr2 = kmalloc(size, GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ptr2);
//
// For tag-based KASAN ptr1 and ptr2 tags might happen to be the same.
// Allow up to 16 attempts at generating different tags.
//
    if (!IS_ENABLED!(CONFIG_KASAN_GENERIC) && ptr1 == ptr2 && counter++ < 16) {
    kfree(ptr2);
// goto;
    }
    KUNIT_EXPECT_KASAN_FAIL_READ(test, (ptr1)[40]);
    KUNIT_EXPECT_PTR_NE(test, ptr1, ptr2);
    kfree(ptr2);
    }
//
// Check that KASAN detects use-after-free when another object was allocated in
// the same slot. Relevant for the tag-based modes, which do not use quarantine.
//
#[no_mangle]
unsafe extern "C" fn kmalloc_uaf3(test: *mut kunit) {
    let mut ptr1 = core::ptr::null_mut();
    let mut ptr2 = core::ptr::null_mut();
pub static mut size: usize = 100;
// This test is specifically crafted for tag-based modes.
    KASAN_TEST_NEEDS_CONFIG_OFF(test, CONFIG_KASAN_GENERIC);
    ptr1 = kmalloc(size, GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ptr1);
    kfree(ptr1);
    ptr2 = kmalloc(size, GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ptr2);
    kfree(ptr2);
    KUNIT_EXPECT_KASAN_FAIL_READ(test, (ptr1)[8]);
    }
#[no_mangle]
unsafe extern "C" fn kasan_atomics_helper(test: *mut kunit, unsafe: *mut c_void, safe: *mut c_void) {
    let mut i_unsafe = unsafe;
    KUNIT_EXPECT_KASAN_FAIL_READ(test, READ_ONCE(*i_unsafe));
    KUNIT_EXPECT_KASAN_FAIL(test, WRITE_ONCE(*i_unsafe, 42));
    KUNIT_EXPECT_KASAN_FAIL_READ(test, smp_load_acquire(i_unsafe));
    KUNIT_EXPECT_KASAN_FAIL(test, smp_store_release(i_unsafe, 42));
    KUNIT_EXPECT_KASAN_FAIL_READ(test, atomic_read(unsafe));
    KUNIT_EXPECT_KASAN_FAIL(test, atomic_set(unsafe, 42));
    KUNIT_EXPECT_KASAN_FAIL(test, atomic_add(42, unsafe));
    KUNIT_EXPECT_KASAN_FAIL(test, atomic_sub(42, unsafe));
    KUNIT_EXPECT_KASAN_FAIL(test, atomic_inc(unsafe));
    KUNIT_EXPECT_KASAN_FAIL(test, atomic_dec(unsafe));
    KUNIT_EXPECT_KASAN_FAIL(test, atomic_and(42, unsafe));
    KUNIT_EXPECT_KASAN_FAIL(test, atomic_andnot(42, unsafe));
    KUNIT_EXPECT_KASAN_FAIL(test, atomic_or(42, unsafe));
    KUNIT_EXPECT_KASAN_FAIL(test, atomic_xor(42, unsafe));
    KUNIT_EXPECT_KASAN_FAIL(test, atomic_xchg(unsafe, 42));
    KUNIT_EXPECT_KASAN_FAIL(test, atomic_cmpxchg(unsafe, 21, 42));
    KUNIT_EXPECT_KASAN_FAIL(test, atomic_try_cmpxchg(unsafe, safe, 42));
//
// The result of the test below may vary due to garbage values of
// unsafe in write-only mode.
// Therefore, skip this test when KASAN is configured in write-only mode.
//
    if (!kasan_write_only_enabled()) {
    KUNIT_EXPECT_KASAN_FAIL(test, atomic_try_cmpxchg(safe, unsafe, 42));
    }
    KUNIT_EXPECT_KASAN_FAIL(test, atomic_sub_and_test(42, unsafe));
    KUNIT_EXPECT_KASAN_FAIL(test, atomic_dec_and_test(unsafe));
    KUNIT_EXPECT_KASAN_FAIL(test, atomic_inc_and_test(unsafe));
    KUNIT_EXPECT_KASAN_FAIL(test, atomic_add_negative(42, unsafe));
//
// The result of the test below may vary due to garbage values of
// unsafe in write-only mode.
// Therefore, skip this test when KASAN is configured in write-only mode.
//
    if (!kasan_write_only_enabled()) {
    KUNIT_EXPECT_KASAN_FAIL(test, atomic_add_unless(unsafe, 21, 42));
    KUNIT_EXPECT_KASAN_FAIL(test, atomic_inc_not_zero(unsafe));
    KUNIT_EXPECT_KASAN_FAIL(test, atomic_inc_unless_negative(unsafe));
    KUNIT_EXPECT_KASAN_FAIL(test, atomic_dec_unless_positive(unsafe));
    KUNIT_EXPECT_KASAN_FAIL(test, atomic_dec_if_positive(unsafe));
    }
    KUNIT_EXPECT_KASAN_FAIL_READ(test, atomic_long_read(unsafe));
    KUNIT_EXPECT_KASAN_FAIL(test, atomic_long_set(unsafe, 42));
    KUNIT_EXPECT_KASAN_FAIL(test, atomic_long_add(42, unsafe));
    KUNIT_EXPECT_KASAN_FAIL(test, atomic_long_sub(42, unsafe));
    KUNIT_EXPECT_KASAN_FAIL(test, atomic_long_inc(unsafe));
    KUNIT_EXPECT_KASAN_FAIL(test, atomic_long_dec(unsafe));
    KUNIT_EXPECT_KASAN_FAIL(test, atomic_long_and(42, unsafe));
    KUNIT_EXPECT_KASAN_FAIL(test, atomic_long_andnot(42, unsafe));
    KUNIT_EXPECT_KASAN_FAIL(test, atomic_long_or(42, unsafe));
    KUNIT_EXPECT_KASAN_FAIL(test, atomic_long_xor(42, unsafe));
    KUNIT_EXPECT_KASAN_FAIL(test, atomic_long_xchg(unsafe, 42));
    KUNIT_EXPECT_KASAN_FAIL(test, atomic_long_cmpxchg(unsafe, 21, 42));
    KUNIT_EXPECT_KASAN_FAIL(test, atomic_long_try_cmpxchg(unsafe, safe, 42));
//
// The result of the test below may vary due to garbage values of
// unsafe in write-only mode.
// Therefore, skip this test when KASAN is configured in write-only mode.
//
    if (!kasan_write_only_enabled()) {
    KUNIT_EXPECT_KASAN_FAIL(test, atomic_long_try_cmpxchg(safe, unsafe, 42));
    }
    KUNIT_EXPECT_KASAN_FAIL(test, atomic_long_sub_and_test(42, unsafe));
    KUNIT_EXPECT_KASAN_FAIL(test, atomic_long_dec_and_test(unsafe));
    KUNIT_EXPECT_KASAN_FAIL(test, atomic_long_inc_and_test(unsafe));
    KUNIT_EXPECT_KASAN_FAIL(test, atomic_long_add_negative(42, unsafe));
//
// The result of the test below may vary due to garbage values of
// unsafe in write-only mode.
// Therefore, skip this test when KASAN is configured in write-only mode.
//
    if (!kasan_write_only_enabled()) {
    KUNIT_EXPECT_KASAN_FAIL(test, atomic_long_add_unless(unsafe, 21, 42));
    KUNIT_EXPECT_KASAN_FAIL(test, atomic_long_inc_not_zero(unsafe));
    KUNIT_EXPECT_KASAN_FAIL(test, atomic_long_inc_unless_negative(unsafe));
    KUNIT_EXPECT_KASAN_FAIL(test, atomic_long_dec_unless_positive(unsafe));
    KUNIT_EXPECT_KASAN_FAIL(test, atomic_long_dec_if_positive(unsafe));
    }
    }
#[no_mangle]
unsafe extern "C" fn kasan_atomics(test: *mut kunit) {
    let mut a1 = core::ptr::null_mut();
    let mut a2 = core::ptr::null_mut();
//
// Just as with kasan_bitops_tags(), we allocate 48 bytes of memory such
// that the following 16 bytes will make up the redzone.
//
    a1 = kzalloc(48, GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, a1);
    a2 = kzalloc_obj(atomic_long_t);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, a2);
// Use atomics to access the redzone.
    kasan_atomics_helper(test, a1 + 48, a2);
    kfree(a1);
    kfree(a2);
    }
#[no_mangle]
unsafe extern "C" fn kmalloc_double_kzfree(test: *mut kunit) {
pub static mut ptr: *mut c_void = core::ptr::null_mut();
pub static mut size: usize = 16;
//
// With the tag-based KASAN modes, if the memory happens to be
// reallocated between the two frees and the new allocation tag happens
// to match the old one, the second free will cause a memory corruption.
// Resolving https://bugzilla.kernel.org/show_bug.cgi?id=212177 would
// help to deal with this. With Generic KASAN, it's effectively
// impossible for the memory to get reallocated due to the quarantine.
//
    KASAN_TEST_NEEDS_CONFIG_ON(test, CONFIG_KASAN_GENERIC);
    ptr = kmalloc(size, GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ptr);
    kfree_sensitive(ptr);
    KUNIT_EXPECT_KASAN_FAIL(test, kfree_sensitive(ptr));
    }
// Check that ksize() does NOT unpoison whole object.
#[no_mangle]
unsafe extern "C" fn ksize_unpoisons_memory(test: *mut kunit) {
pub static mut ptr: *mut c_void = core::ptr::null_mut();
pub static mut size: usize = 0;
    let mut real_size = 0;
    ptr = kmalloc(size, GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ptr);
    real_size = ksize(ptr);
    KUNIT_EXPECT_GT(test, real_size, size);
    OPTIMIZER_HIDE_VAR(ptr);
// These accesses shouldn't trigger a KASAN report.
    ptr[0] = 'x';
    ptr[size - 1] = 'x';
// These must trigger a KASAN report.
    if (IS_ENABLED!(CONFIG_KASAN_GENERIC)) {
    KUNIT_EXPECT_KASAN_FAIL(test, (ptr)[size]);
    }
    KUNIT_EXPECT_KASAN_FAIL_READ(test, (ptr)[size + 5]);
    KUNIT_EXPECT_KASAN_FAIL_READ(test, (ptr)[real_size - 1]);
    kfree(ptr);
    }
//
// Check that a use-after-free is detected by ksize() and via normal accesses
// after it.
//
#[no_mangle]
unsafe extern "C" fn ksize_uaf(test: *mut kunit) {
pub static mut ptr: *mut c_void = core::ptr::null_mut();
pub static mut size: c_int = 0;
    ptr = kmalloc(size, GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ptr);
    kfree(ptr);
    OPTIMIZER_HIDE_VAR(ptr);
    KUNIT_EXPECT_KASAN_FAIL(test, ksize(ptr));
    KUNIT_EXPECT_KASAN_FAIL_READ(test, (ptr)[0]);
    KUNIT_EXPECT_KASAN_FAIL_READ(test, (ptr)[size]);
    }
//
// The two tests below check that Generic KASAN prints auxiliary stack traces
// for RCU callbacks and workqueues. The reports need to be inspected manually.
//
// These tests are still enabled for other KASAN modes to make sure that all
// modes report bad accesses in tested scenarios.
//
    static struct kasan_rcu_info {
    let mut i = 0;
pub static mut rcu: usize = 0;
    } *global_rcu_ptr;
#[no_mangle]
unsafe extern "C" fn rcu_uaf_reclaim(rp: *mut rcu_head) {
    let mut fp = container_of!(rp, kasan_rcu_info, rcu);
    kfree(fp);
    (fp).i;
    }
#[no_mangle]
unsafe extern "C" fn rcu_uaf(test: *mut kunit) {
pub static mut ptr: *mut c_void = core::ptr::null_mut();
    ptr = kmalloc_obj(kasan_rcu_info);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ptr);
    global_rcu_ptr = rcu_dereference_protected(
    ptr, core::ptr::null_mut());
    KUNIT_EXPECT_KASAN_FAIL_READ(test,
    call_rcu(&global_rcu_ptr.rcu, rcu_uaf_reclaim);
    rcu_barrier());
    }
#[no_mangle]
unsafe extern "C" fn workqueue_uaf_work(work: *mut work_struct) {
    kfree(work);
    }
#[no_mangle]
unsafe extern "C" fn workqueue_uaf(test: *mut kunit) {
pub static mut workqueue: *mut c_void = core::ptr::null_mut();
pub static mut work: *mut c_void = core::ptr::null_mut();
    workqueue = create_workqueue("kasan_workqueue_test");
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, workqueue);
    work = kmalloc_obj(work_struct);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, work);
    INIT_WORK(work, workqueue_uaf_work);
    queue_work(workqueue, work);
    destroy_workqueue(workqueue);
    KUNIT_EXPECT_KASAN_FAIL_READ(test,
    (work).data);
    }
#[no_mangle]
unsafe extern "C" fn kfree_via_page(test: *mut kunit) {
pub static mut ptr: *mut c_void = core::ptr::null_mut();
pub static mut size: usize = 8;
pub static mut page: *mut c_void = core::ptr::null_mut();
    let mut offset = 0;
    ptr = kmalloc(size, GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ptr);
    page = virt_to_page(ptr);
    offset = offset_in_page(ptr);
    kfree(page_address(page) + offset);
    }
#[no_mangle]
unsafe extern "C" fn kfree_via_phys(test: *mut kunit) {
pub static mut ptr: *mut c_void = core::ptr::null_mut();
pub static mut size: usize = 8;
    let mut phys;
    ptr = kmalloc(size, GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ptr);
    phys = virt_to_phys(ptr);
    kfree(phys_to_virt(phys));
    }
#[no_mangle]
unsafe extern "C" fn kmem_cache_oob(test: *mut kunit) {
pub static mut p: *mut c_void = core::ptr::null_mut();
pub static mut size: usize = 200;
pub static mut cache: *mut c_void = core::ptr::null_mut();
    cache = kmem_cache_create("test_cache", size, 0, 0, core::ptr::null_mut());
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, cache);
    p = kmem_cache_alloc(cache, GFP_KERNEL);
    if (!p) {
    kunit_err(test, "Allocation failed: %s\n", __func__);
    kmem_cache_destroy(cache);
    return;
    }
    KUNIT_EXPECT_KASAN_FAIL_READ(test, *p = p[size + OOB_TAG_OFF]);
    kmem_cache_free(cache, p);
    kmem_cache_destroy(cache);
    }
#[no_mangle]
unsafe extern "C" fn kmem_cache_double_free(test: *mut kunit) {
pub static mut p: *mut c_void = core::ptr::null_mut();
pub static mut size: usize = 200;
pub static mut cache: *mut c_void = core::ptr::null_mut();
    cache = kmem_cache_create("test_cache", size, 0, 0, core::ptr::null_mut());
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, cache);
    p = kmem_cache_alloc(cache, GFP_KERNEL);
    if (!p) {
    kunit_err(test, "Allocation failed: %s\n", __func__);
    kmem_cache_destroy(cache);
    return;
    }
    kmem_cache_free(cache, p);
    KUNIT_EXPECT_KASAN_FAIL(test, kmem_cache_free(cache, p));
    kmem_cache_destroy(cache);
    }
#[no_mangle]
unsafe extern "C" fn kmem_cache_invalid_free(test: *mut kunit) {
pub static mut p: *mut c_void = core::ptr::null_mut();
pub static mut size: usize = 200;
pub static mut cache: *mut c_void = core::ptr::null_mut();
    cache = kmem_cache_create("test_cache", size, 0, SLAB_TYPESAFE_BY_RCU,
    core::ptr::null_mut());
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, cache);
    p = kmem_cache_alloc(cache, GFP_KERNEL);
    if (!p) {
    kunit_err(test, "Allocation failed: %s\n", __func__);
    kmem_cache_destroy(cache);
    return;
    }
// Trigger invalid free, the object doesn't get freed.
    KUNIT_EXPECT_KASAN_FAIL(test, kmem_cache_free(cache, p + 1));
//
// Properly free the object to prevent the "Objects remaining in
// test_cache on __kmem_cache_shutdown" BUG failure.
//
    kmem_cache_free(cache, p);
    kmem_cache_destroy(cache);
    }
#[no_mangle]
unsafe extern "C" fn kmem_cache_rcu_uaf(test: *mut kunit) {
pub static mut p: *mut c_void = core::ptr::null_mut();
pub static mut size: usize = 200;
pub static mut cache: *mut c_void = core::ptr::null_mut();
    KASAN_TEST_NEEDS_CONFIG_ON(test, CONFIG_SLUB_RCU_DEBUG);
    cache = kmem_cache_create("test_cache", size, 0, SLAB_TYPESAFE_BY_RCU,
    core::ptr::null_mut());
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, cache);
    p = kmem_cache_alloc(cache, GFP_KERNEL);
    if (!p) {
    kunit_err(test, "Allocation failed: %s\n", __func__);
    kmem_cache_destroy(cache);
    return;
    }
// p = 1;
    rcu_read_lock();
// Free the object - this will internally schedule an RCU callback.
    kmem_cache_free(cache, p);
//
// We should still be allowed to access the object at this point because
// the cache is SLAB_TYPESAFE_BY_RCU and we've been in an RCU read-side
// critical section since before the kmem_cache_free().
//
    READ_ONCE(*p);
    rcu_read_unlock();
//
// Wait for the RCU callback to execute; after this, the object should
// have actually been freed from KASAN's perspective.
//
    rcu_barrier();
    KUNIT_EXPECT_KASAN_FAIL_READ(test, READ_ONCE(*p));
    kmem_cache_destroy(cache);
    }
//
// Check that SLAB_TYPESAFE_BY_RCU objects are immediately reused when
// CONFIG_SLUB_RCU_DEBUG is off, and stay at the same address.
// Without this, KASAN builds would be unable to trigger bugs caused by
// SLAB_TYPESAFE_BY_RCU users handling reycled objects improperly.
//
#[no_mangle]
unsafe extern "C" fn kmem_cache_rcu_reuse(test: *mut kunit) {
    let mut p = core::ptr::null_mut();
    let mut p2 = core::ptr::null_mut();
pub static mut cache: *mut c_void = core::ptr::null_mut();
    KASAN_TEST_NEEDS_CONFIG_OFF(test, CONFIG_SLUB_RCU_DEBUG);
    cache = kmem_cache_create("test_cache", 16, 0, SLAB_TYPESAFE_BY_RCU,
    core::ptr::null_mut());
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, cache);
    migrate_disable();
    p = kmem_cache_alloc(cache, GFP_KERNEL);
    if (!p) {
    kunit_err(test, "Allocation failed: %s\n", __func__);
// goto;
    }
    kmem_cache_free(cache, p);
    p2 = kmem_cache_alloc(cache, GFP_KERNEL);
    if (!p2) {
    kunit_err(test, "Allocation failed: %s\n", __func__);
// goto;
    }
    KUNIT_EXPECT_PTR_EQ(test, p, p2);
    kmem_cache_free(cache, p2);
// label;
    migrate_enable();
    kmem_cache_destroy(cache);
    }
#[no_mangle]
unsafe extern "C" fn kmem_cache_double_destroy(test: *mut kunit) {
pub static mut cache: *mut c_void = core::ptr::null_mut();
    cache = kmem_cache_create("test_cache", 200, 0, SLAB_NO_MERGE, core::ptr::null_mut());
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, cache);
    kmem_cache_destroy(cache);
    KUNIT_EXPECT_KASAN_FAIL(test, kmem_cache_destroy(cache));
    }
#[no_mangle]
unsafe extern "C" fn kmem_cache_accounted(test: *mut kunit) {
    let mut i = 0;
pub static mut p: *mut c_void = core::ptr::null_mut();
pub static mut size: usize = 200;
pub static mut cache: *mut c_void = core::ptr::null_mut();
    cache = kmem_cache_create("test_cache", size, 0, SLAB_ACCOUNT, core::ptr::null_mut());
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, cache);
//
// Several allocations with a delay to allow for lazy per memcg kmem
// cache creation.
//
    while (i < 5) {
    p = kmem_cache_alloc(cache, GFP_KERNEL);
    if (!p) {
// goto;
    }
    kmem_cache_free(cache, p);
    msleep(100);
    }
// label;
    kmem_cache_destroy(cache);
    }
#[no_mangle]
unsafe extern "C" fn kmem_cache_bulk(test: *mut kunit) {
pub static mut cache: *mut c_void = core::ptr::null_mut();
pub static mut size: usize = 200;
    char *p[10];
    let mut i = 0;
    cache = kmem_cache_create("test_cache", size, 0, 0, core::ptr::null_mut());
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, cache);
    if (!kmem_cache_alloc_bulk(cache, GFP_KERNEL, ARRAY_SIZE!(p),
    &p)) {
    kunit_err(test, "Allocation failed: %s\n", __func__);
    kmem_cache_destroy(cache);
    return;
    }
    for (i = 0; i < ARRAY_SIZE!(p); i++) {
    p[i][0] = p[i][size - 1] = 42;
    }
    kmem_cache_free_bulk(cache, ARRAY_SIZE!(p), &p);
    kmem_cache_destroy(cache);
    }
#[no_mangle]
pub unsafe extern "C" fn mempool_prepare_kmalloc(test: *mut kunit, pool: *mut mempool_t, size: size_t) -> *mut c_void {
pub static mut pool_size: c_int = 4;
    let mut ret = 0;
pub static mut elem: *mut c_void = core::ptr::null_mut();
    memset(pool, 0, sizeof!(*pool));
    ret = mempool_init_kmalloc_pool(pool, pool_size, size);
    KUNIT_ASSERT_EQ(test, ret, 0);
//
// Allocate one element to prevent mempool from freeing elements to the
// underlying allocator and instead make it add them to the element
// list when the tests trigger double-free and invalid-free bugs.
// This allows testing KASAN annotations in add_element().
//
    elem = mempool_alloc_preallocated(pool);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, elem);
    return elem;
    }
#[no_mangle]
pub unsafe extern "C" fn mempool_prepare_slab(test: *mut kunit, pool: *mut mempool_t, size: size_t) -> *mut c_void {
pub static mut cache: *mut c_void = core::ptr::null_mut();
pub static mut pool_size: c_int = 4;
    let mut ret = 0;
    cache = kmem_cache_create("test_cache", size, 0, 0, core::ptr::null_mut());
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, cache);
    memset(pool, 0, sizeof!(*pool));
    ret = mempool_init_slab_pool(pool, pool_size, cache);
    KUNIT_ASSERT_EQ(test, ret, 0);
//
// Do not allocate one preallocated element, as we skip the double-free
// and invalid-free tests for slab mempool for simplicity.
//
    return cache;
    }
#[no_mangle]
pub unsafe extern "C" fn mempool_prepare_page(test: *mut kunit, pool: *mut mempool_t, order: c_int) -> *mut c_void {
pub static mut pool_size: c_int = 4;
    let mut ret = 0;
pub static mut elem: *mut c_void = core::ptr::null_mut();
    memset(pool, 0, sizeof!(*pool));
    ret = mempool_init_page_pool(pool, pool_size, order);
    KUNIT_ASSERT_EQ(test, ret, 0);
    elem = mempool_alloc_preallocated(pool);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, elem);
    return elem;
    }
#[no_mangle]
unsafe extern "C" fn mempool_oob_right_helper(test: *mut kunit, pool: *mut mempool_t, size: usize) {
pub static mut elem: *mut c_void = core::ptr::null_mut();
    elem = mempool_alloc_preallocated(pool);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, elem);
    OPTIMIZER_HIDE_VAR(elem);
    if (IS_ENABLED!(CONFIG_KASAN_GENERIC)) {
    KUNIT_EXPECT_KASAN_FAIL(test,
    (&elem[size])[0]);
    }
    else {
    KUNIT_EXPECT_KASAN_FAIL_READ(test,
    (&elem[round_up(size, KASAN_GRANULE_SIZE)])[0]);
    }
    mempool_free(elem, pool);
    }
#[no_mangle]
unsafe extern "C" fn mempool_kmalloc_oob_right(test: *mut kunit) {
    let mut pool;
pub static mut size: usize = 0;
pub static mut extra_elem: *mut c_void = core::ptr::null_mut();
    extra_elem = mempool_prepare_kmalloc(test, &pool, size);
    mempool_oob_right_helper(test, &pool, size);
    mempool_free(extra_elem, &pool);
    mempool_exit(&pool);
    }
#[no_mangle]
unsafe extern "C" fn mempool_kmalloc_large_oob_right(test: *mut kunit) {
    let mut pool;
pub static mut size: usize = 0;
pub static mut extra_elem: *mut c_void = core::ptr::null_mut();
    extra_elem = mempool_prepare_kmalloc(test, &pool, size);
    mempool_oob_right_helper(test, &pool, size);
    mempool_free(extra_elem, &pool);
    mempool_exit(&pool);
    }
#[no_mangle]
unsafe extern "C" fn mempool_slab_oob_right(test: *mut kunit) {
    let mut pool;
pub static mut size: usize = 123;
pub static mut cache: *mut c_void = core::ptr::null_mut();
    cache = mempool_prepare_slab(test, &pool, size);
    mempool_oob_right_helper(test, &pool, size);
    mempool_exit(&pool);
    kmem_cache_destroy(cache);
    }
//
// Skip the out-of-bounds test for page mempool. With Generic KASAN, page
// allocations have no redzones, and thus the out-of-bounds detection is not
// guaranteed; see https://bugzilla.kernel.org/show_bug.cgi?id=210503. With
// the tag-based KASAN modes, the neighboring allocation might have the same
// tag; see https://bugzilla.kernel.org/show_bug.cgi?id=203505.
//
#[no_mangle]
unsafe extern "C" fn mempool_uaf_helper(test: *mut kunit, pool: *mut mempool_t, page: bool) {
    let mut elem = core::ptr::null_mut();
    let mut ptr = core::ptr::null_mut();
    elem = mempool_alloc_preallocated(pool);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, elem);
    mempool_free(elem, pool);
    ptr = page ? page_address(elem) : elem;
    KUNIT_EXPECT_KASAN_FAIL_READ(test, (ptr)[0]);
    }
#[no_mangle]
unsafe extern "C" fn mempool_kmalloc_uaf(test: *mut kunit) {
    let mut pool;
pub static mut size: usize = 128;
pub static mut extra_elem: *mut c_void = core::ptr::null_mut();
    extra_elem = mempool_prepare_kmalloc(test, &pool, size);
    mempool_uaf_helper(test, &pool, false);
    mempool_free(extra_elem, &pool);
    mempool_exit(&pool);
    }
#[no_mangle]
unsafe extern "C" fn mempool_kmalloc_large_uaf(test: *mut kunit) {
    let mut pool;
pub static mut size: usize = 0;
pub static mut extra_elem: *mut c_void = core::ptr::null_mut();
    extra_elem = mempool_prepare_kmalloc(test, &pool, size);
    mempool_uaf_helper(test, &pool, false);
    mempool_free(extra_elem, &pool);
    mempool_exit(&pool);
    }
#[no_mangle]
unsafe extern "C" fn mempool_slab_uaf(test: *mut kunit) {
    let mut pool;
pub static mut size: usize = 123;
pub static mut cache: *mut c_void = core::ptr::null_mut();
    cache = mempool_prepare_slab(test, &pool, size);
    mempool_uaf_helper(test, &pool, false);
    mempool_exit(&pool);
    kmem_cache_destroy(cache);
    }
#[no_mangle]
unsafe extern "C" fn mempool_page_alloc_uaf(test: *mut kunit) {
    let mut pool;
pub static mut order: c_int = 2;
pub static mut extra_elem: *mut c_void = core::ptr::null_mut();
    extra_elem = mempool_prepare_page(test, &pool, order);
    mempool_uaf_helper(test, &pool, true);
    mempool_free(extra_elem, &pool);
    mempool_exit(&pool);
    }
#[no_mangle]
unsafe extern "C" fn mempool_double_free_helper(test: *mut kunit, pool: *mut mempool_t) {
pub static mut elem: *mut c_void = core::ptr::null_mut();
    elem = mempool_alloc_preallocated(pool);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, elem);
    mempool_free(elem, pool);
    KUNIT_EXPECT_KASAN_FAIL(test, mempool_free(elem, pool));
    }
#[no_mangle]
unsafe extern "C" fn mempool_kmalloc_double_free(test: *mut kunit) {
    let mut pool;
pub static mut size: usize = 128;
pub static mut extra_elem: *mut c_void = core::ptr::null_mut();
    extra_elem = mempool_prepare_kmalloc(test, &pool, size);
    mempool_double_free_helper(test, &pool);
    mempool_free(extra_elem, &pool);
    mempool_exit(&pool);
    }
#[no_mangle]
unsafe extern "C" fn mempool_kmalloc_large_double_free(test: *mut kunit) {
    let mut pool;
pub static mut size: usize = 0;
pub static mut extra_elem: *mut c_void = core::ptr::null_mut();
    extra_elem = mempool_prepare_kmalloc(test, &pool, size);
    mempool_double_free_helper(test, &pool);
    mempool_free(extra_elem, &pool);
    mempool_exit(&pool);
    }
#[no_mangle]
unsafe extern "C" fn mempool_page_alloc_double_free(test: *mut kunit) {
    let mut pool;
pub static mut order: c_int = 2;
pub static mut extra_elem: *mut c_void = core::ptr::null_mut();
    extra_elem = mempool_prepare_page(test, &pool, order);
    mempool_double_free_helper(test, &pool);
    mempool_free(extra_elem, &pool);
    mempool_exit(&pool);
    }
#[no_mangle]
unsafe extern "C" fn mempool_kmalloc_invalid_free_helper(test: *mut kunit, pool: *mut mempool_t) {
pub static mut elem: *mut c_void = core::ptr::null_mut();
    elem = mempool_alloc_preallocated(pool);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, elem);
    KUNIT_EXPECT_KASAN_FAIL(test, mempool_free(elem + 1, pool));
    mempool_free(elem, pool);
    }
#[no_mangle]
unsafe extern "C" fn mempool_kmalloc_invalid_free(test: *mut kunit) {
    let mut pool;
pub static mut size: usize = 128;
pub static mut extra_elem: *mut c_void = core::ptr::null_mut();
    extra_elem = mempool_prepare_kmalloc(test, &pool, size);
    mempool_kmalloc_invalid_free_helper(test, &pool);
    mempool_free(extra_elem, &pool);
    mempool_exit(&pool);
    }
#[no_mangle]
unsafe extern "C" fn mempool_kmalloc_large_invalid_free(test: *mut kunit) {
    let mut pool;
pub static mut size: usize = 0;
pub static mut extra_elem: *mut c_void = core::ptr::null_mut();
    extra_elem = mempool_prepare_kmalloc(test, &pool, size);
    mempool_kmalloc_invalid_free_helper(test, &pool);
    mempool_free(extra_elem, &pool);
    mempool_exit(&pool);
    }
//
// Skip the invalid-free test for page mempool. The invalid-free detection only
// works for compound pages and mempool preallocates all page elements without
// the __GFP_COMP flag.
//
    static char global_array[10];
#[no_mangle]
unsafe extern "C" fn kasan_global_oob_right(test: *mut kunit) {
//
// Deliberate out-of-bounds access. To prevent CONFIG_UBSAN_LOCAL_BOUNDS
// from failing here and panicking the kernel, access the array via a
// volatile pointer, which will prevent the compiler from being able to
// determine the array bounds.
//
// This access uses a volatile pointer to char (char *volatile) rather
// than the more conventional pointer to volatile char 
// because we want to prevent the compiler from making inferences about
// the pointer itself (i.e. its array bounds), not the data that it
// refers to.
//
pub static mut array: *mut char volatile = core::ptr::null_mut();
    let mut p = &array[ARRAY_SIZE!(global_array) + 3];
// Only generic mode instruments globals.
    KASAN_TEST_NEEDS_CONFIG_ON(test, CONFIG_KASAN_GENERIC);
    KUNIT_EXPECT_KASAN_FAIL(test, *p);
    }
#[no_mangle]
unsafe extern "C" fn kasan_global_oob_left(test: *mut kunit) {
pub static mut array: *mut char volatile = core::ptr::null_mut();
    let mut p = array - 3;
//
// GCC is known to fail this test, skip it.
// See https://bugzilla.kernel.org/show_bug.cgi?id=215051.
//
    KASAN_TEST_NEEDS_CONFIG_ON(test, CONFIG_CC_IS_CLANG);
    KASAN_TEST_NEEDS_CONFIG_ON(test, CONFIG_KASAN_GENERIC);
    KUNIT_EXPECT_KASAN_FAIL(test, *p);
    }
#[no_mangle]
unsafe extern "C" fn kasan_stack_oob(test: *mut kunit) {
    char stack_array[10];
// See comment in kasan_global_oob_right.
pub static mut array: *mut char volatile = core::ptr::null_mut();
    let mut p = &array[ARRAY_SIZE!(stack_array) + OOB_TAG_OFF];
    KASAN_TEST_NEEDS_CONFIG_ON(test, CONFIG_KASAN_STACK);
    KUNIT_EXPECT_KASAN_FAIL(test, *p);
    }
#[no_mangle]
unsafe extern "C" fn kasan_alloca_oob_left(test: *mut kunit) {
pub static mut i: volatile int = 10;
    char alloca_array[i];
// See comment in kasan_global_oob_right.
pub static mut array: *mut char volatile = core::ptr::null_mut();
    let mut p = array - 1;
// Only generic mode instruments dynamic allocas.
    KASAN_TEST_NEEDS_CONFIG_ON(test, CONFIG_KASAN_GENERIC);
    KASAN_TEST_NEEDS_CONFIG_ON(test, CONFIG_KASAN_STACK);
    KUNIT_EXPECT_KASAN_FAIL(test, *p);
    }
#[no_mangle]
unsafe extern "C" fn kasan_alloca_oob_right(test: *mut kunit) {
pub static mut i: volatile int = 10;
    char alloca_array[i];
// See comment in kasan_global_oob_right.
pub static mut array: *mut char volatile = core::ptr::null_mut();
    let mut p = array + i;
// Only generic mode instruments dynamic allocas.
    KASAN_TEST_NEEDS_CONFIG_ON(test, CONFIG_KASAN_GENERIC);
    KASAN_TEST_NEEDS_CONFIG_ON(test, CONFIG_KASAN_STACK);
    KUNIT_EXPECT_KASAN_FAIL(test, *p);
    }
#[no_mangle]
unsafe extern "C" fn kasan_memchr(test: *mut kunit) {
pub static mut ptr: *mut c_void = core::ptr::null_mut();
pub static mut size: usize = 24;
//
// str* functions are not instrumented with CONFIG_AMD_MEM_ENCRYPT.
// See https://bugzilla.kernel.org/show_bug.cgi?id=206337 for details.
//
    KASAN_TEST_NEEDS_CONFIG_OFF(test, CONFIG_AMD_MEM_ENCRYPT);
    if (OOB_TAG_OFF) {
    size = round_up(size, OOB_TAG_OFF);
    }
    ptr = kmalloc(size, GFP_KERNEL | __GFP_ZERO);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ptr);
    OPTIMIZER_HIDE_VAR(ptr);
    OPTIMIZER_HIDE_VAR(size);
    KUNIT_EXPECT_KASAN_FAIL_READ(test,
    kasan_ptr_result = memchr(ptr, '1', size + 1));
    kfree(ptr);
    }
#[no_mangle]
unsafe extern "C" fn kasan_memcmp(test: *mut kunit) {
pub static mut ptr: *mut c_void = core::ptr::null_mut();
pub static mut size: usize = 24;
    int arr[9];
//
// str* functions are not instrumented with CONFIG_AMD_MEM_ENCRYPT.
// See https://bugzilla.kernel.org/show_bug.cgi?id=206337 for details.
//
    KASAN_TEST_NEEDS_CONFIG_OFF(test, CONFIG_AMD_MEM_ENCRYPT);
    if (OOB_TAG_OFF) {
    size = round_up(size, OOB_TAG_OFF);
    }
    ptr = kmalloc(size, GFP_KERNEL | __GFP_ZERO);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ptr);
    memset(arr, 0, sizeof!(arr));
    OPTIMIZER_HIDE_VAR(ptr);
    OPTIMIZER_HIDE_VAR(size);
    KUNIT_EXPECT_KASAN_FAIL_READ(test,
    kasan_int_result = memcmp(ptr, arr, size+1));
    kfree(ptr);
    }
#[no_mangle]
unsafe extern "C" fn kasan_strings(test: *mut kunit) {
pub static mut ptr: *mut c_void = core::ptr::null_mut();
pub static mut src: *mut c_void = core::ptr::null_mut();
pub static mut size: usize = 24;
//
// str* functions are not instrumented with CONFIG_AMD_MEM_ENCRYPT.
// See https://bugzilla.kernel.org/show_bug.cgi?id=206337 for details.
//
    KASAN_TEST_NEEDS_CONFIG_OFF(test, CONFIG_AMD_MEM_ENCRYPT);
    ptr = kmalloc(size, GFP_KERNEL | __GFP_ZERO);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ptr);
    OPTIMIZER_HIDE_VAR(ptr);
    src = kmalloc(KASAN_GRANULE_SIZE, GFP_KERNEL | __GFP_ZERO);
    strscpy(src, "f0cacc1a0000000", KASAN_GRANULE_SIZE);
    OPTIMIZER_HIDE_VAR(src);
//
// Make sure that strscpy() does not trigger KASAN if it overreads into
// poisoned memory.
//
// The expected size does not include the terminator '\0'
// so it is (KASAN_GRANULE_SIZE - 2) ==
// KASAN_GRANULE_SIZE - ("initial removed character" + "\0").
//
    KUNIT_EXPECT_EQ(test, KASAN_GRANULE_SIZE - 2,
    strscpy(ptr, src + 1, KASAN_GRANULE_SIZE));
// strscpy should fail if the first byte is unreadable.
    KUNIT_EXPECT_KASAN_FAIL_READ(test, strscpy(ptr, src + KASAN_GRANULE_SIZE,
    KASAN_GRANULE_SIZE));
    kfree(src);
    kfree(ptr);
//
// Try to cause only 1 invalid access (less spam in dmesg).
// For that we need ptr to point to zeroed byte.
// Skip metadata that could be stored in freed object so ptr
// will likely point to zeroed byte.
//
    ptr += 16;
    KUNIT_EXPECT_KASAN_FAIL_READ(test, kasan_ptr_result = strchr(ptr, '1'));
    KUNIT_EXPECT_KASAN_FAIL_READ(test, kasan_ptr_result = strrchr(ptr, '1'));
    KUNIT_EXPECT_KASAN_FAIL_READ(test, kasan_int_result = strcmp(ptr, "2"));
    KUNIT_EXPECT_KASAN_FAIL_READ(test, kasan_int_result = strncmp(ptr, "2", 1));
    KUNIT_EXPECT_KASAN_FAIL_READ(test, kasan_int_result = strlen(ptr));
    KUNIT_EXPECT_KASAN_FAIL_READ(test, kasan_int_result = strnlen(ptr, 1));
    }
#[no_mangle]
unsafe extern "C" fn kasan_bitops_modify(test: *mut kunit, nr: c_int, addr: *mut c_void) {
    KUNIT_EXPECT_KASAN_FAIL(test, set_bit(nr, addr));
    KUNIT_EXPECT_KASAN_FAIL(test, __set_bit(nr, addr));
    KUNIT_EXPECT_KASAN_FAIL(test, clear_bit(nr, addr));
    KUNIT_EXPECT_KASAN_FAIL(test, __clear_bit(nr, addr));
    KUNIT_EXPECT_KASAN_FAIL(test, clear_bit_unlock(nr, addr));
    KUNIT_EXPECT_KASAN_FAIL(test, __clear_bit_unlock(nr, addr));
    KUNIT_EXPECT_KASAN_FAIL(test, change_bit(nr, addr));
    KUNIT_EXPECT_KASAN_FAIL(test, __change_bit(nr, addr));
    }
#[no_mangle]
unsafe extern "C" fn kasan_bitops_test_and_modify(test: *mut kunit, nr: c_int, addr: *mut c_void) {
    KUNIT_EXPECT_KASAN_FAIL(test, test_and_set_bit(nr, addr));
    KUNIT_EXPECT_KASAN_FAIL(test, __test_and_set_bit(nr, addr));
//
// When KASAN is running in write-only mode,
// a fault won't occur when the bit is set.
// Therefore, skip the test_and_set_bit_lock test in write-only mode.
//
    if (!kasan_write_only_enabled()) {
    KUNIT_EXPECT_KASAN_FAIL(test, test_and_set_bit_lock(nr, addr));
    }
    KUNIT_EXPECT_KASAN_FAIL(test, test_and_clear_bit(nr, addr));
    KUNIT_EXPECT_KASAN_FAIL(test, __test_and_clear_bit(nr, addr));
    KUNIT_EXPECT_KASAN_FAIL(test, test_and_change_bit(nr, addr));
    KUNIT_EXPECT_KASAN_FAIL(test, __test_and_change_bit(nr, addr));
    KUNIT_EXPECT_KASAN_FAIL_READ(test, kasan_int_result = test_bit(nr, addr));
    if (nr < 7) {
    KUNIT_EXPECT_KASAN_FAIL(test, kasan_int_result =
    xor_unlock_is_negative_byte(1 << nr, addr));
    }
    }
#[no_mangle]
unsafe extern "C" fn kasan_bitops_generic(test: *mut kunit) {
pub static mut bits: *mut c_void = core::ptr::null_mut();
// This test is specifically crafted for the generic mode.
    KASAN_TEST_NEEDS_CONFIG_ON(test, CONFIG_KASAN_GENERIC);
//
// Allocate 1 more byte, which causes kzalloc to round up to 16 bytes;
// this way we do not actually corrupt other memory.
//
    bits = kzalloc(sizeof!(*bits) + 1, GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, bits);
//
// Below calls try to access bit within allocated memory; however, the
// below accesses are still out-of-bounds, since bitops are defined to
// operate on the whole long the bit is in.
//
    kasan_bitops_modify(test, BITS_PER_LONG, bits);
//
// Below calls try to access bit beyond allocated memory.
//
    kasan_bitops_test_and_modify(test, BITS_PER_LONG + BITS_PER_BYTE, bits);
    kfree(bits);
    }
#[no_mangle]
unsafe extern "C" fn kasan_bitops_tags(test: *mut kunit) {
pub static mut bits: *mut c_void = core::ptr::null_mut();
// This test is specifically crafted for tag-based modes.
    KASAN_TEST_NEEDS_CONFIG_OFF(test, CONFIG_KASAN_GENERIC);
// kmalloc-64 cache will be used and the last 16 bytes will be the redzone.
    bits = kzalloc(48, GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, bits);
// Do the accesses past the 48 allocated bytes, but within the redone.
    kasan_bitops_modify(test, BITS_PER_LONG, bits + 48);
    kasan_bitops_test_and_modify(test, BITS_PER_LONG + BITS_PER_BYTE, bits + 48);
    kfree(bits);
    }
#[no_mangle]
unsafe extern "C" fn vmalloc_helpers_tags(test: *mut kunit) {
pub static mut ptr: *mut c_void = core::ptr::null_mut();
// This test is intended for tag-based modes.
    KASAN_TEST_NEEDS_CONFIG_OFF(test, CONFIG_KASAN_GENERIC);
    KASAN_TEST_NEEDS_CONFIG_ON(test, CONFIG_KASAN_VMALLOC);
    if (!kasan_vmalloc_enabled()) {
    kunit_skip(test, "Test requires kasan.vmalloc=on");
    }
    ptr = vmalloc(PAGE_SIZE);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ptr);
// Check that the returned pointer is tagged.
    KUNIT_EXPECT_GE(test, (u8)get_tag(ptr), (u8)KASAN_TAG_MIN);
    KUNIT_EXPECT_LT(test, (u8)get_tag(ptr), (u8)KASAN_TAG_KERNEL);
// Make sure exported vmalloc helpers handle tagged pointers.
    KUNIT_ASSERT_TRUE(test, is_vmalloc_addr(ptr));
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, vmalloc_to_page(ptr));

    {
    let mut rv = 0;
// Make sure vmalloc'ed memory permissions can be changed.
    rv = set_memory_ro((unsigned long)ptr, 1);
    KUNIT_ASSERT_GE(test, rv, 0);
    rv = set_memory_rw((unsigned long)ptr, 1);
    KUNIT_ASSERT_GE(test, rv, 0);
    }

    vfree(ptr);
    }
#[no_mangle]
unsafe extern "C" fn vmalloc_oob_helper(test: *mut kunit, v_ptr: *mut c_char, size: usize) {
//
// We have to be careful not to hit the guard page in vmalloc tests.
// The MMU will catch that and crash us.
//
// Make sure in-bounds accesses are valid.
    v_ptr[0] = 0;
    v_ptr[size - 1] = 0;
//
// An unaligned access past the requested vmalloc size.
// Only generic KASAN can precisely detect these.
//
    if (IS_ENABLED!(CONFIG_KASAN_GENERIC)) {
    KUNIT_EXPECT_KASAN_FAIL(test, (v_ptr)[size]);
    }
// An aligned access into the first out-of-bounds granule.
    size = round_up(size, KASAN_GRANULE_SIZE);
    KUNIT_EXPECT_KASAN_FAIL_READ(test, (v_ptr)[size]);
    }
#[no_mangle]
unsafe extern "C" fn vmalloc_oob(test: *mut kunit) {
    let mut v_ptr = core::ptr::null_mut();
    let mut p_ptr = core::ptr::null_mut();
pub static mut page: *mut c_void = core::ptr::null_mut();
pub static mut size: usize = 0;
    KASAN_TEST_NEEDS_CONFIG_ON(test, CONFIG_KASAN_VMALLOC);
    if (!kasan_vmalloc_enabled()) {
    kunit_skip(test, "Test requires kasan.vmalloc=on");
    }
    v_ptr = vmalloc(size);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, v_ptr);
    OPTIMIZER_HIDE_VAR(v_ptr);
    vmalloc_oob_helper(test, v_ptr, size);
    size -= KASAN_GRANULE_SIZE + 1;
    v_ptr = vrealloc(v_ptr, size, GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, v_ptr);
    OPTIMIZER_HIDE_VAR(v_ptr);
    vmalloc_oob_helper(test, v_ptr, size);
    size += 2 * KASAN_GRANULE_SIZE + 2;
    v_ptr = vrealloc(v_ptr, size, GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, v_ptr);
    vmalloc_oob_helper(test, v_ptr, size);
// Check that in-bounds accesses to the physical page are valid.
    page = vmalloc_to_page(v_ptr);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, page);
    p_ptr = page_address(page);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, p_ptr);
    p_ptr[0] = 0;
    vfree(v_ptr);
//
// We can't check for use-after-unmap bugs in this nor in the following
// vmalloc tests, as the page might be fully unmapped and accessing it
// will crash the kernel.
//
    }
#[no_mangle]
unsafe extern "C" fn vmap_tags(test: *mut kunit) {
    let mut p_ptr = core::ptr::null_mut();
    let mut v_ptr = core::ptr::null_mut();
    let mut p_page = core::ptr::null_mut();
    let mut v_page = core::ptr::null_mut();
//
// This test is specifically crafted for the software tag-based mode,
// the only tag-based mode that poisons vmap mappings.
//
    KASAN_TEST_NEEDS_CONFIG_ON(test, CONFIG_KASAN_SW_TAGS);
    KASAN_TEST_NEEDS_CONFIG_ON(test, CONFIG_KASAN_VMALLOC);
    if (!kasan_vmalloc_enabled()) {
    kunit_skip(test, "Test requires kasan.vmalloc=on");
    }
    p_page = alloc_pages(GFP_KERNEL, 1);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, p_page);
    p_ptr = page_address(p_page);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, p_ptr);
    v_ptr = vmap(&p_page, 1, VM_MAP, PAGE_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, v_ptr);
//
// We can't check for out-of-bounds bugs in this nor in the following
// vmalloc tests, as allocations have page granularity and accessing
// the guard page will crash the kernel.
//
    KUNIT_EXPECT_GE(test, (u8)get_tag(v_ptr), (u8)KASAN_TAG_MIN);
    KUNIT_EXPECT_LT(test, (u8)get_tag(v_ptr), (u8)KASAN_TAG_KERNEL);
// Make sure that in-bounds accesses through both pointers work.
// p_ptr = 0;
// v_ptr = 0;
// Make sure vmalloc_to_page() correctly recovers the page pointer.
    v_page = vmalloc_to_page(v_ptr);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, v_page);
    KUNIT_EXPECT_PTR_EQ(test, p_page, v_page);
    vunmap(v_ptr);
    free_pages((unsigned long)p_ptr, 1);
    }
#[no_mangle]
unsafe extern "C" fn vm_map_ram_tags(test: *mut kunit) {
    let mut p_ptr = core::ptr::null_mut();
    let mut v_ptr = core::ptr::null_mut();
pub static mut page: *mut c_void = core::ptr::null_mut();
//
// This test is specifically crafted for the software tag-based mode,
// the only tag-based mode that poisons vm_map_ram mappings.
//
    KASAN_TEST_NEEDS_CONFIG_ON(test, CONFIG_KASAN_SW_TAGS);
    page = alloc_pages(GFP_KERNEL, 1);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, page);
    p_ptr = page_address(page);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, p_ptr);
    v_ptr = vm_map_ram(&page, 1, -1);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, v_ptr);
    KUNIT_EXPECT_GE(test, (u8)get_tag(v_ptr), (u8)KASAN_TAG_MIN);
    KUNIT_EXPECT_LT(test, (u8)get_tag(v_ptr), (u8)KASAN_TAG_KERNEL);
// Make sure that in-bounds accesses through both pointers work.
// p_ptr = 0;
// v_ptr = 0;
    vm_unmap_ram(v_ptr, 1);
    free_pages((unsigned long)p_ptr, 1);
    }
//
// Check that the assigned pointer tag falls within the [KASAN_TAG_MIN,
// KASAN_TAG_KERNEL) range (note: excluding the match-all tag) for tag-based
// modes.
//
#[no_mangle]
unsafe extern "C" fn match_all_not_assigned(test: *mut kunit) {
pub static mut ptr: *mut c_void = core::ptr::null_mut();
pub static mut pages: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    let mut size = 0;
    let mut order = 0;
    KASAN_TEST_NEEDS_CONFIG_OFF(test, CONFIG_KASAN_GENERIC);
    while (i < 256) {
    size = get_random_u32_inclusive(1, 1024);
    ptr = kmalloc(size, GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ptr);
    KUNIT_EXPECT_GE(test, (u8)get_tag(ptr), (u8)KASAN_TAG_MIN);
    KUNIT_EXPECT_LT(test, (u8)get_tag(ptr), (u8)KASAN_TAG_KERNEL);
    kfree(ptr);
    }
    while (i < 256) {
    order = get_random_u32_inclusive(1, 4);
    pages = alloc_pages(GFP_KERNEL, order);
    ptr = page_address(pages);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ptr);
    KUNIT_EXPECT_GE(test, (u8)get_tag(ptr), (u8)KASAN_TAG_MIN);
    KUNIT_EXPECT_LT(test, (u8)get_tag(ptr), (u8)KASAN_TAG_KERNEL);
    free_pages((unsigned long)ptr, order);
    }
    if (!kasan_vmalloc_enabled()) {
    return;
    }
    while (i < 256) {
    size = get_random_u32_inclusive(1, 1024);
    ptr = vmalloc(size);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ptr);
    KUNIT_EXPECT_GE(test, (u8)get_tag(ptr), (u8)KASAN_TAG_MIN);
    KUNIT_EXPECT_LT(test, (u8)get_tag(ptr), (u8)KASAN_TAG_KERNEL);
    vfree(ptr);
    }
    }
// Check that 0xff works as a match-all pointer tag for tag-based modes.
#[no_mangle]
unsafe extern "C" fn match_all_ptr_tag(test: *mut kunit) {
pub static mut ptr: *mut c_void = core::ptr::null_mut();
    let mut tag = 0;
    KASAN_TEST_NEEDS_CONFIG_OFF(test, CONFIG_KASAN_GENERIC);
    ptr = kmalloc(128, GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ptr);
// Backup the assigned tag.
    tag = get_tag(ptr);
    KUNIT_EXPECT_NE(test, tag, (u8)KASAN_TAG_KERNEL);
// Reset the tag to 0xff.
    ptr = set_tag(ptr, KASAN_TAG_KERNEL);
// This access shouldn't trigger a KASAN report.
// ptr = 0;
// Recover the pointer tag and free.
    ptr = set_tag(ptr, tag);
    kfree(ptr);
    }
// Check that there are no match-all memory tags for tag-based modes.
#[no_mangle]
unsafe extern "C" fn match_all_mem_tag(test: *mut kunit) {
pub static mut ptr: *mut c_void = core::ptr::null_mut();
    let mut tag = 0;
    KASAN_TEST_NEEDS_CONFIG_OFF(test, CONFIG_KASAN_GENERIC);
    ptr = kmalloc(128, GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ptr);
    KUNIT_EXPECT_NE(test, (u8)get_tag(ptr), (u8)KASAN_TAG_KERNEL);
// For each possible tag value not matching the pointer tag.
    while (tag <= KASAN_TAG_KERNEL) {
//
// For Software Tag-Based KASAN, skip the majority of tag
// values to avoid the test printing too many reports.
//
    if (IS_ENABLED!(CONFIG_KASAN_SW_TAGS) &&
    tag >= KASAN_TAG_MIN + 8 && tag <= KASAN_TAG_KERNEL - 8) {
    continue;
    }
    if (tag == get_tag(ptr)) {
    continue;
    }
// Mark the first memory granule with the chosen memory tag.
    kasan_poison(ptr, KASAN_GRANULE_SIZE, (u8)tag, false);
// This access must cause a KASAN report.
    KUNIT_EXPECT_KASAN_FAIL(test, *ptr = 0);
    }
// Recover the memory tag and free.
    kasan_poison(ptr, KASAN_GRANULE_SIZE, get_tag(ptr), false);
    kfree(ptr);
    }
//
// Check that Rust performing a use-after-free using `unsafe` is detected.
// This is a smoke test to make sure that Rust is being sanitized properly.
//
#[no_mangle]
unsafe extern "C" fn rust_uaf(test: *mut kunit) {
    KASAN_TEST_NEEDS_CONFIG_ON(test, CONFIG_RUST);
    KUNIT_EXPECT_KASAN_FAIL(test, kasan_test_rust_uaf());
    }
//
// copy_to_kernel_nofault() is an internal helper available when
// kasan_test is built-in, so it must not be visible to loadable modules.
//

#[no_mangle]
unsafe extern "C" fn copy_to_kernel_nofault_oob(test: *mut kunit) {
pub static mut ptr: *mut c_void = core::ptr::null_mut();
    char buf[128];
pub static mut size: usize = 0;
//
// This test currently fails with the HW_TAGS mode. The reason is
// unknown and needs to be investigated.
//
    KASAN_TEST_NEEDS_CONFIG_OFF(test, CONFIG_KASAN_HW_TAGS);
    ptr = kmalloc(size - KASAN_GRANULE_SIZE, GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ptr);
    OPTIMIZER_HIDE_VAR(ptr);
//
// We test copy_to_kernel_nofault() to detect corrupted memory that is
// being written into the kernel. In contrast,
// copy_from_kernel_nofault() is primarily used in kernel helper
// functions where the source address might be random or uninitialized.
// Applying KASAN instrumentation to copy_from_kernel_nofault() could
// lead to false positives.  By focusing KASAN checks only on
// copy_to_kernel_nofault(), we ensure that only valid memory is
// written to the kernel, minimizing the risk of kernel corruption
// while avoiding false positives in the reverse case.
//
    KUNIT_EXPECT_KASAN_FAIL(test,
    copy_to_kernel_nofault(&buf[0], ptr, size));
    KUNIT_EXPECT_KASAN_FAIL(test,
    copy_to_kernel_nofault(ptr, &buf[0], size));
    kfree(ptr);
    }

#[no_mangle]
unsafe extern "C" fn copy_user_test_oob(test: *mut kunit) {
pub static mut kmem: *mut c_void = core::ptr::null_mut();
    let mut usermem = core::ptr::null_mut();
    let mut useraddr = 0;
pub static mut size: usize = 0;
    int __maybe_unused unused;
    kmem = kunit_kmalloc(test, size, GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, kmem);
    useraddr = kunit_vm_mmap(test, core::ptr::null_mut(), 0, PAGE_SIZE,
    PROT_READ | PROT_WRITE | PROT_EXEC,
    MAP_ANONYMOUS | MAP_PRIVATE, 0);
    KUNIT_ASSERT_NE_MSG(test, useraddr, 0,
    "Could not create userspace mm");
    KUNIT_ASSERT_LT_MSG(test, useraddr, (unsigned long)TASK_SIZE,
    "Failed to allocate user memory");
    OPTIMIZER_HIDE_VAR(size);
    usermem = useraddr;
    KUNIT_EXPECT_KASAN_FAIL(test,
    unused = copy_from_user(kmem, usermem, size + 1));
    KUNIT_EXPECT_KASAN_FAIL_READ(test,
    unused = copy_to_user(usermem, kmem, size + 1));
    KUNIT_EXPECT_KASAN_FAIL(test,
    unused = __copy_from_user(kmem, usermem, size + 1));
    KUNIT_EXPECT_KASAN_FAIL_READ(test,
    unused = __copy_to_user(usermem, kmem, size + 1));
    KUNIT_EXPECT_KASAN_FAIL(test,
    unused = __copy_from_user_inatomic(kmem, usermem, size + 1));
    KUNIT_EXPECT_KASAN_FAIL_READ(test,
    unused = __copy_to_user_inatomic(usermem, kmem, size + 1));
//
// Prepare a long string in usermem to avoid the strncpy_from_user test
// bailing out on '\0' before it reaches out-of-bounds.
//
    memset(kmem, 'a', size);
    KUNIT_EXPECT_EQ(test, copy_to_user(usermem, kmem, size), 0);
    KUNIT_EXPECT_KASAN_FAIL(test,
    unused = strncpy_from_user(kmem, usermem, size + 1));
    }
pub static mut kunit_case: usize = 0;
pub static mut kunit_suite: usize = 0;
    kunit_test_suite(kasan_kunit_test_suite);
    MODULE_DESCRIPTION("KUnit tests for checking KASAN bug-detection capabilities");
    MODULE_LICENSE("GPL");