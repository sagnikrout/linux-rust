//! Automatically rewritten from C to Rust
//! Source: mm/kmsan/kmsan_test.c
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
// Test cases for KMSAN.
// For each test case checks the presence (or absence) of generated reports.
// Relies on 'console' tracepoint to capture reports as they appear in the
// kernel log.
//
// Copyright (C) 2021-2022, Google LLC.
// Author: Alexander Potapenko <glider@google.com>
//

pub static mut int: usize = 0;
// Report as observed from console.
pub static mut observed: usize = 0;
// Probe for console output: obtains observed lines of interest.
#[no_mangle]
unsafe extern "C" fn probe_console(ignore: *mut c_void, buf: *const c_char, len: usize) {
    let mut flags = 0;
    if (observed.ignore) {
    return;
    }
    spin_lock_irqsave(&observed.lock, flags);
    if (strnstr(buf, "BUG: KMSAN: ", len)) {
//
// KMSAN report and related to the test.
//
// The provided @buf is not NUL-terminated; copy no more than
// @len bytes and let strscpy() add the missing NUL-terminator.
//
    strscpy(observed.header, buf,
    min(len + 1, sizeof!(observed.header)));
    WRITE_ONCE(observed.available, true);
    observed.ignore = true;
    }
    spin_unlock_irqrestore(&observed.lock, flags);
    }
// Check if a report related to the test exists.
#[no_mangle]
unsafe extern "C" fn report_available() -> bool {
    return READ_ONCE(observed.available);
    }
// Reset observed.available, so that the test can trigger another report.
#[no_mangle]
unsafe extern "C" fn report_reset() {
    let mut flags = 0;
    spin_lock_irqsave(&observed.lock, flags);
    WRITE_ONCE(observed.available, false);
    observed.ignore = false;
    spin_unlock_irqrestore(&observed.lock, flags);
    }
// Information we expect in a report.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct expect_report {
//     pub /: *const *const *const char error_type; / Error type.,
//
// Kernel symbol from the error header, or NULL if no report is
// expected.
//
    pub symbol: *const c_char,
}

// Check observed report matches information in @r.
#[no_mangle]
unsafe extern "C" fn report_matches(r: *const expect_report) -> bool {
    typeof(observed.header) expected_header;
    let mut flags = 0;
pub static mut ret: bool = false;
pub static mut end: *mut c_void = core::ptr::null_mut();
pub static mut cur: *mut c_void = core::ptr::null_mut();
// Doubled-checked locking.
    if (!report_available() || !r.symbol) {
    return (!report_available() && !r.symbol);
    }
// Generate expected report contents.
// Title
    cur = expected_header;
    end = ARRAY_END(expected_header);
    cur += scnprintf(cur, end - cur, "BUG: KMSAN: %s", r.error_type);
    scnprintf(cur, end - cur, " in %s", r.symbol);
// The exact offset won't match, remove it; also strip module name.
    cur = strchr(expected_header, '+');
    if (cur) {
// cur = '\0';
    }
    spin_lock_irqsave(&observed.lock, flags);
    if (!report_available()) {
// goto; /* A new report is being captured. */
    }
// Finally match expected output to what we actually observed.
    ret = strstr(observed.header, expected_header);
// label;
    spin_unlock_irqrestore(&observed.lock, flags);
    return ret;
    }
// ===== Test cases =====
// Prevent replacing branch with select in LLVM.
#[no_mangle]
unsafe extern "C" fn check_true(arg: *mut c_char) -> noinline void {
    pr_info!("%s is true\n", arg);
    }
#[no_mangle]
unsafe extern "C" fn check_false(arg: *mut c_char) -> noinline void {
    pr_info!("%s is false\n", arg);
    }

    do {                             
    if (x)                    {
    check_true(#x);  
    }
    else {
    check_false(#x); 
    }
    } while (0)

pub static mut expect_report: usize = 0;
    kunit_info(
    test,
    "memcpy()ing aligned uninit src to unaligned dst (UMR report)\n");
    kmsan_check_memory(&uninit_src, sizeof!(uninit_src));
    memcpy_noinline(&dst[1], &uninit_src,
    sizeof!(uninit_src));
    kmsan_check_memory(dst, 4);
    KUNIT_EXPECT_TRUE(test, report_matches(&expect));
    report_reset();
    kmsan_check_memory(&dst[4], sizeof!(uninit_src));
    KUNIT_EXPECT_TRUE(test, report_matches(&expect));
    }
//
// Test case: ensure that origin slots do not accidentally get overwritten with
// zeroes during memcpy().
//
// Previously, when copying memory from an aligned buffer to an unaligned one,
// if there were zero origins corresponding to zero shadow values in the source
// buffer, they could have ended up being copied to nonzero shadow values in the
// destination buffer:
//
// memcpy(0xffff888080a00000, 0xffff888080900002, 8)
//
// src (0xffff888080900002): ..xx .... xx..
// src origins:              o111 0000 o222
// dst (0xffff888080a00000): xx.. ..xx
// dst origins:              o111 0000
// (or 0000 o222)
//
// (here . stands for an initialized byte, and x for an uninitialized one.
//
// Ensure that this does not happen anymore, and for both destination bytes
// the origin is nonzero (i.e. KMSAN reports an error).
//
#[no_mangle]
unsafe extern "C" fn test_memcpy_initialized_gap(test: *mut kunit) {
    EXPECTATION_UNINIT_VALUE_FN(expect, "test_memcpy_initialized_gap");
    volatile char uninit_src[12];
    volatile char dst[8] = { 0 };
    kunit_info(
    test,
    "unaligned 4-byte initialized value gets a nonzero origin after memcpy() - (2 UMR reports)\n");
    uninit_src[0] = 42;
    uninit_src[1] = 42;
    uninit_src[4] = 42;
    uninit_src[5] = 42;
    uninit_src[6] = 42;
    uninit_src[7] = 42;
    uninit_src[10] = 42;
    uninit_src[11] = 42;
    memcpy_noinline(&dst[0], &uninit_src[2], 8);
    kmsan_check_memory(&dst[0], 4);
    KUNIT_EXPECT_TRUE(test, report_matches(&expect));
    report_reset();
    kmsan_check_memory(&dst[2], 4);
    KUNIT_EXPECT_FALSE(test, report_matches(&expect));
    report_reset();
    kmsan_check_memory(&dst[4], 4);
    KUNIT_EXPECT_TRUE(test, report_matches(&expect));
    }
// Generate test cases for memset16(), memset32(), memset64().

    static void test_memset##size(kunit *test)                   
    {                                                                   
    EXPECTATION_NO_REPORT(expect);                              
    volatile uint##size##_t uninit;                             
    
    kunit_info(test,                                            
    "memset" #size "() should initialize memory\n"); 
    memset##size((uint##size##_t *)&uninit, 0, 1);              
    kmsan_check_memory(&uninit, sizeof!(uninit));        
    KUNIT_EXPECT_TRUE(test, report_matches(&expect));           
    }
    DEFINE_TEST_MEMSETXX(16)
    DEFINE_TEST_MEMSETXX(32)
    DEFINE_TEST_MEMSETXX(64)
// Test case: ensure that KMSAN does not access shadow memory out of bounds.
#[no_mangle]
unsafe extern "C" fn test_memset_on_guarded_buffer(test: *mut kunit) {
    let mut buf = vmalloc(PAGE_SIZE);
    kunit_info(test,
    "memset() on ends of guarded buffer should not crash\n");
    while (size <= 128) {
    memset(buf, 0xff, size);
    memset(buf + PAGE_SIZE - size, 0xff, size);
    }
    vfree(buf);
    }
#[no_mangle]
unsafe extern "C" fn fibonacci(array: *mut c_int, size: c_int, start: c_int) -> noinline void {
    if (start < 2 || (start == size)) {
    return;
    }
    array[start] = array[start - 1] + array[start - 2];
    fibonacci(array, size, start + 1);
    }
#[no_mangle]
unsafe extern "C" fn test_long_origin_chain(test: *mut kunit) {
    EXPECTATION_UNINIT_VALUE_FN(expect, "test_long_origin_chain");
// (KMSAN_MAX_ORIGIN_DEPTH * 2) recursive calls to fibonacci().
    volatile int accum[KMSAN_MAX_ORIGIN_DEPTH * 2 + 2];
pub static mut last: c_int = 0;
    kunit_info(
    test,
    "origin chain exceeding KMSAN_MAX_ORIGIN_DEPTH (UMR report)\n");
//
// We do not set accum[1] to 0, so the uninitializedness will be carried
// over to accum[2..last].
//
    accum[0] = 1;
    fibonacci(accum, ARRAY_SIZE!(accum), 2);
    kmsan_check_memory(&accum[last], sizeof!(int));
    KUNIT_EXPECT_TRUE(test, report_matches(&expect));
    }
//
// Test case: ensure that saving/restoring/printing stacks to/from stackdepot
// does not trigger errors.
//
// KMSAN uses stackdepot to store origin stack traces, that's why we do not
// instrument lib/stackdepot.c. Yet it must properly mark its outputs as
// initialized because other kernel features (e.g. netdev tracker) may also
// access stackdepot from instrumented code.
//
#[no_mangle]
unsafe extern "C" fn test_stackdepot_roundtrip(test: *mut kunit) {
    unsigned long src_entries[16], *dst_entries;
    let mut src_nentries = 0;
    let mut dst_nentries = 0;
    EXPECTATION_NO_REPORT(expect);
    let mut handle;
    kunit_info(test, "testing stackdepot roundtrip (no reports)\n");
    src_nentries =
    stack_trace_save(src_entries, ARRAY_SIZE!(src_entries), 1);
    handle = stack_depot_save(src_entries, src_nentries, GFP_KERNEL);
    stack_depot_print(handle);
    dst_nentries = stack_depot_fetch(handle, &dst_entries);
    KUNIT_EXPECT_TRUE(test, src_nentries == dst_nentries);
    kmsan_check_memory(dst_entries,
    sizeof!(*dst_entries) * dst_nentries);
    KUNIT_EXPECT_TRUE(test, report_matches(&expect));
    }
//
// Test case: ensure that kmsan_unpoison_memory() and the instrumentation work
// the same.
//
#[no_mangle]
unsafe extern "C" fn test_unpoison_memory(test: *mut kunit) {
    EXPECTATION_UNINIT_VALUE_FN(expect, "test_unpoison_memory");
    volatile char a[4], b[4];
    kunit_info(
    test,
    "unpoisoning via the instrumentation vs. kmsan_unpoison_memory() (2 UMR reports)\n");
// Initialize a[0] and check a[1]--a[3].
    a[0] = 0;
    kmsan_check_memory(&a[1], 3);
    KUNIT_EXPECT_TRUE(test, report_matches(&expect));
    report_reset();
// Initialize b[0] and check b[1]--b[3].
    kmsan_unpoison_memory(&b[0], 1);
    kmsan_check_memory(&b[1], 3);
    KUNIT_EXPECT_TRUE(test, report_matches(&expect));
    }
#[no_mangle]
unsafe extern "C" fn test_copy_from_kernel_nofault(test: *mut kunit) {
    let mut ret = 0;
    char buf[4], src[4];
pub static mut size: usize = 0;
    EXPECTATION_UNINIT_VALUE_FN(expect, "copy_from_kernel_nofault");
    kunit_info(
    test,
    "testing copy_from_kernel_nofault with uninitialized memory\n");
    ret = copy_from_kernel_nofault(&buf[0], &src[0], size);
    USE(ret);
    KUNIT_EXPECT_TRUE(test, report_matches(&expect));
    }
pub static mut kunit_case: usize = 0;
// ===== End test cases =====
#[no_mangle]
unsafe extern "C" fn test_init(test: *mut kunit) -> c_int {
    let mut flags = 0;
    spin_lock_irqsave(&observed.lock, flags);
    observed.header[0] = '\0';
    observed.ignore = false;
    observed.available = false;
    spin_unlock_irqrestore(&observed.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_exit(test: *mut kunit) {
    }
    static int orig_panic_on_kmsan;
#[no_mangle]
unsafe extern "C" fn kmsan_suite_init(suite: *mut kunit_suite) -> c_int {
    register_trace_console(probe_console, core::ptr::null_mut());
    orig_panic_on_kmsan = panic_on_kmsan;
    panic_on_kmsan = 0;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn kmsan_suite_exit(suite: *mut kunit_suite) {
    unregister_trace_console(probe_console, core::ptr::null_mut());
    tracepoint_synchronize_unregister();
    panic_on_kmsan = orig_panic_on_kmsan;
    }
pub static mut kunit_suite: usize = 0;
    kunit_test_suites(&kmsan_test_suite);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Alexander Potapenko <glider@google.com>");
    MODULE_DESCRIPTION("Test cases for KMSAN");