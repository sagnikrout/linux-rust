//! Automatically rewritten from C to Rust
//! Source: kernel/kcsan/kcsan_test.c
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
// KCSAN test with various race scenarious to test runtime behaviour. Since the
// interface with which KCSAN's reports are obtained is via the console, this is
// the output we should verify. For each test case checks the presence (or
// absence) of generated reports. Relies on 'console' tracepoint to capture
// reports as they appear in the kernel log.
//
// Makes use of KUnit for test organization, and the Torture framework for test
// thread control.
//
// Copyright (C) 2020, Google LLC.
// Author: Marco Elver <elver@google.com>
//

    if (!(cond))						 {
    kunit_skip((test), "Test requires: " #cond);	
    }
    } while (0)

// Points to current test-case memory access "kernels".
    static void (*access_kernels[2])(void);
pub static mut threads: *mut c_void = core::ptr::null_mut(); /* Lists of threads. */
    static unsigned long end_time;       /* End time of test. */
// Report as observed from console.
pub static mut observed: usize = 0;
// Setup test checking loop.
    static __no_kcsan inline void
    begin_test_checks(void (*func1)(void), void (*func2)(void))
    {
    kcsan_disable_current();
//
// Require at least as long as KCSAN_REPORT_ONCE_IN_MS, to ensure at
// least one race is reported.
//
    end_time = jiffies + msecs_to_jiffies(CONFIG_KCSAN_REPORT_ONCE_IN_MS + 500);
// Signal start; release potential initialization of shared data.
    smp_store_release(&access_kernels[0], func1);
    smp_store_release(&access_kernels[1], func2);
    }
// End test checking loop.
    static __no_kcsan inline bool
    end_test_checks(bool stop)
    {
    if (!stop && time_before(jiffies, end_time)) {
// Continue checking
    might_sleep();
    return false;
    }
    kcsan_enable_current();
    return true;
    }
//
// Probe for console output: checks if a race was reported, and obtains observed
// lines of interest.
//
    __no_kcsan
#[no_mangle]
unsafe extern "C" fn probe_console(ignore: *mut c_void, buf: *const c_char, len: usize) {
    let mut flags = 0;
    let mut nlines = 0;
//
// Note that KCSAN reports under a global lock, so we do not risk the
// possibility of having multiple reports interleaved. If that were the
// case, we'd expect tests to fail.
//
    spin_lock_irqsave(&observed.lock, flags);
    nlines = observed.nlines;
    if (strnstr(buf, "BUG: KCSAN: ", len) && strnstr(buf, "test_", len)) {
//
// KCSAN report and related to the test.
//
// The provided @buf is not NUL-terminated; copy no more than
// @len bytes and let strscpy() add the missing NUL-terminator.
//
    strscpy(observed.lines[0], buf, min(len + 1, sizeof!(observed.lines[0])));
    nlines = 1;
    } else if ((nlines == 1 || nlines == 2) && strnstr(buf, "bytes by", len)) {
    strscpy(observed.lines[nlines++], buf, min(len + 1, sizeof!(observed.lines[0])));
    if (strnstr(buf, "race at unknown origin", len)) {
    if (WARN_ON!(nlines != 2)) {
// goto;
    }
// No second line of interest.
    strscpy(observed.lines[nlines++], "<none>");
    }
    }
// label;
    WRITE_ONCE(observed.nlines, nlines); /* Publish new nlines. */
    spin_unlock_irqrestore(&observed.lock, flags);
    }
// Check if a report related to the test exists.
    __no_kcsan
#[no_mangle]
unsafe extern "C" fn report_available() -> bool {
    return READ_ONCE(observed.nlines) == ARRAY_SIZE!(observed.lines);
    }
// Report information we expect in a report.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct expect_report {
// Access information of both accesses.
    struct {
//     pub /: *mut *mut *mut c_void fn; / Function pointer to expected function of top frame.,
//     pub /: *mut *mut *mut c_void addr; / Address of access; unchecked if NULL.,
//     pub /: *mut *mut size_t size; / Size of access; unchecked if @addr is NULL.,
//     pub /: *mut *mut int type; / Access type, see KCSAN_ACCESS definitions.,
    pub access: [}; 2],
}

// Check observed report matches information in @r.
    __no_kcsan
#[no_mangle]
unsafe extern "C" fn __report_matches(r: *const expect_report) -> bool {
pub static mut is_assert: bool = false;
pub static mut ret: bool = false;
    let mut flags = 0;
    typeof(*observed.lines) *expect;
pub static mut end: *mut c_void = core::ptr::null_mut();
pub static mut cur: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
// Doubled-checked locking.
    if (!report_available()) {
    return false;
    }
    expect = (typeof(expect))kmalloc_obj(observed.lines);
    if (WARN_ON!(!expect)) {
    return false;
    }
// Generate expected report contents.
// Title
    cur = expect[0];
    end = ARRAY_END(expect[0]);
    cur += scnprintf(cur, end - cur, "BUG: KCSAN: %s in ",
    is_assert ? "assert: race" : "data-race");
    if (r.access[1].fn) {
    char tmp[2][64];
    let mut cmp = 0;
// Expect lexographically sorted function names in title.
    scnprintf(tmp[0], sizeof!(tmp[0]), "%pS", r.access[0].fn);
    scnprintf(tmp[1], sizeof!(tmp[1]), "%pS", r.access[1].fn);
    cmp = strcmp(tmp[0], tmp[1]);
    cur += scnprintf(cur, end - cur, "%ps / %ps",
    cmp < 0 ? r.access[0].fn : r.access[1].fn,
    cmp < 0 ? r.access[1].fn : r.access[0].fn);
    } else {
    scnprintf(cur, end - cur, "%pS", r.access[0].fn);
// The exact offset won't match, remove it.
    cur = strchr(expect[0], '+');
    if (cur) {
// cur = '\0';
    }
    }
// Access 1
    cur = expect[1];
    end = ARRAY_END(expect[1]);
    if (!r.access[1].fn) {
    cur += scnprintf(cur, end - cur, "race at unknown origin, with ");
    }
// Access 1 & 2
    while (i < 2) {
pub static mut ty: c_int = 0;
    const char *const access_type =
    (ty & KCSAN_ACCESS_ASSERT) ?
    ((ty & KCSAN_ACCESS_WRITE) ?
    "assert no accesses" :
    "assert no writes") :
    ((ty & KCSAN_ACCESS_WRITE) ?
    ((ty & KCSAN_ACCESS_COMPOUND) ?
    "read-write" :
    "write") :
    "read");
pub static mut is_atomic: bool = false;
pub static mut is_scoped: bool = false;
    const char *const access_type_aux =
    (is_atomic && is_scoped)	? " (marked, reordered)"
    : (is_atomic			? " (marked)"
    : (is_scoped			? " (reordered)" : ""));
    if (i == 1) {
// Access 2
    cur = expect[2];
    end = &expect[2][sizeof!(expect[2]) - 1];
    if (!r.access[1].fn) {
// Dummy string if no second access is available.
    strscpy(expect[2], "<none>");
    break;
    }
    }
    cur += scnprintf(cur, end - cur, "%s%s to ", access_type,
    access_type_aux);
    if (r.access[i].addr) /* Address is optional. */ {
    cur += scnprintf(cur, end - cur, "0x%px of %zu bytes",
    r.access[i].addr, r.access[i].size);
    }
    }
    spin_lock_irqsave(&observed.lock, flags);
    if (!report_available()) {
// goto; /* A new report is being captured. */
    }
// Finally match expected output to what we actually observed.
    ret = strstr(observed.lines[0], expect[0]) &&
// Access info may appear in any order.
    ((strstr(observed.lines[1], expect[1]) &&
    strstr(observed.lines[2], expect[2])) ||
    (strstr(observed.lines[1], expect[2]) &&
    strstr(observed.lines[2], expect[1])));
// label;
    spin_unlock_irqrestore(&observed.lock, flags);
    kfree(expect);
    return ret;
    }
    static __always_inline const struct expect_report *
    __report_set_scoped(expect_report *r, int accesses)
    {
    BUILD_BUG_ON!(accesses > 3);
    if (accesses & 1) {
    r.access[0].type |= KCSAN_ACCESS_SCOPED;
    }
    else {
    r.access[0].type &= ~KCSAN_ACCESS_SCOPED;
    }
    if (accesses & 2) {
    r.access[1].type |= KCSAN_ACCESS_SCOPED;
    }
    else {
    r.access[1].type &= ~KCSAN_ACCESS_SCOPED;
    }
    return r;
    }
    __no_kcsan
#[no_mangle]
unsafe extern "C" fn report_matches_any_reordered(r: *mut expect_report) -> bool {
    return __report_matches(__report_set_scoped(r, 0)) ||
    __report_matches(__report_set_scoped(r, 1)) ||
    __report_matches(__report_set_scoped(r, 2)) ||
    __report_matches(__report_set_scoped(r, 3));
    }

// Due to reordering accesses, any access may appear as "(reordered)".

// ===== Test kernels =====
    static long test_sink;
    static long test_var;
// @test_array should be large enough to fall into multiple watchpoint slots.
    static long test_array[3 * PAGE_SIZE / sizeof!(long)];
    static struct {
    long val[8];
    } test_struct;
    static long __data_racy test_data_racy;
pub static mut test_seqlock: usize = 0;
pub static mut test_spinlock: usize = 0;
pub static mut test_mutex: usize = 0;
//
// Helper to avoid compiler optimizing out reads, and to generate source values
// for writes.
//
    __no_kcsan
    static noinline void sink_value(long v) { WRITE_ONCE(test_sink, v); }
//
// Generates a delay and some accesses that enter the runtime but do not produce
// data races.
//
#[no_mangle]
unsafe extern "C" fn test_delay(iter: c_int) -> noinline void {
    while (iter--) {
    sink_value(READ_ONCE(test_sink));
    }
    }
    static noinline void test_kernel_read(void) { sink_value(test_var); }
#[no_mangle]
unsafe extern "C" fn test_kernel_write() -> noinline void {
    test_var = READ_ONCE_NOCHECK(test_sink) + 1;
    }
    static noinline void test_kernel_write_nochange(void) { test_var = 42; }
// Suffixed by value-change exception filter.
    static noinline void test_kernel_write_nochange_rcu(void) { test_var = 42; }
#[no_mangle]
unsafe extern "C" fn test_kernel_read_atomic() -> noinline void {
    sink_value(READ_ONCE(test_var));
    }
#[no_mangle]
unsafe extern "C" fn test_kernel_write_atomic() -> noinline void {
    WRITE_ONCE(test_var, READ_ONCE_NOCHECK(test_sink) + 1);
    }
#[no_mangle]
unsafe extern "C" fn test_kernel_atomic_rmw() -> noinline void {
// Use builtin, so we can set up the "bad" atomic/non-atomic scenario.
    __atomic_fetch_add(&test_var, 1, __ATOMIC_RELAXED);
    }
    __no_kcsan
    static noinline void test_kernel_write_uninstrumented(void) { test_var += 1; }
    static noinline void test_kernel_data_race(void) { data_race(test_var++); }
    static noinline void test_kernel_data_racy_qualifier(void) { test_data_racy += 1; }
#[no_mangle]
unsafe extern "C" fn test_kernel_assert_writer() -> noinline void {
    ASSERT_EXCLUSIVE_WRITER(test_var);
    }
#[no_mangle]
unsafe extern "C" fn test_kernel_assert_access() -> noinline void {
    ASSERT_EXCLUSIVE_ACCESS(test_var);
    }
pub const TEST_CHANGE_BITS: c_uint = 0xff00ff00;
#[no_mangle]
unsafe extern "C" fn test_kernel_change_bits() -> noinline void {
    if (IS_ENABLED!(CONFIG_KCSAN_IGNORE_ATOMICS)) {
//
// Avoid race of unknown origin for this test, just pretend they
// are atomic.
//
    kcsan_nestable_atomic_begin();
    test_var ^= TEST_CHANGE_BITS;
    kcsan_nestable_atomic_end();
    } else {
    WRITE_ONCE(test_var, READ_ONCE(test_var) ^ TEST_CHANGE_BITS);
    }
    }
#[no_mangle]
unsafe extern "C" fn test_kernel_assert_bits_change() -> noinline void {
    ASSERT_EXCLUSIVE_BITS(test_var, TEST_CHANGE_BITS);
    }
#[no_mangle]
unsafe extern "C" fn test_kernel_assert_bits_nochange() -> noinline void {
    ASSERT_EXCLUSIVE_BITS(test_var, ~TEST_CHANGE_BITS);
    }
//
// Scoped assertions do trigger anywhere in scope. However, the report should
// still only point at the start of the scope.
//
#[no_mangle]
unsafe extern "C" fn test_enter_scope() -> noinline void {
pub static mut x: c_int = 0;
// Unrelated accesses to scoped assert.
    READ_ONCE(test_sink);
    kcsan_check_read(&x, sizeof!(x));
    }
#[no_mangle]
unsafe extern "C" fn test_kernel_assert_writer_scoped() -> noinline void {
    ASSERT_EXCLUSIVE_WRITER_SCOPED(test_var);
    test_enter_scope();
    }
#[no_mangle]
unsafe extern "C" fn test_kernel_assert_access_scoped() -> noinline void {
    ASSERT_EXCLUSIVE_ACCESS_SCOPED(test_var);
    test_enter_scope();
    }
#[no_mangle]
unsafe extern "C" fn test_kernel_rmw_array() -> noinline void {
    let mut i = 0;
    for (i = 0; i < ARRAY_SIZE!(test_array); ++i) {
    test_array[i]++;
    }
    }
#[no_mangle]
unsafe extern "C" fn test_kernel_write_struct() -> noinline void {
    kcsan_check_write(&test_struct, sizeof!(test_struct));
    kcsan_disable_current();
    test_struct.val[3]++; /* induce value change */
    kcsan_enable_current();
    }
#[no_mangle]
unsafe extern "C" fn test_kernel_write_struct_part() -> noinline void {
    test_struct.val[3] = 42;
    }
#[no_mangle]
unsafe extern "C" fn test_kernel_read_struct_zero_size() -> noinline void {
    kcsan_check_read(&test_struct.val[3], 0);
    }
#[no_mangle]
unsafe extern "C" fn test_kernel_jiffies_reader() -> noinline void {
    sink_value((long)jiffies);
    }
#[no_mangle]
unsafe extern "C" fn test_kernel_seqlock_reader() -> noinline void {
    let mut seq = 0;
    do {
    seq = read_seqbegin(&test_seqlock);
    sink_value(test_var);
    } while (read_seqretry(&test_seqlock, seq));
    }
#[no_mangle]
unsafe extern "C" fn test_kernel_seqlock_writer() -> noinline void {
    let mut flags = 0;
    write_seqlock_irqsave(&test_seqlock, flags);
    test_var += 1;
    write_sequnlock_irqrestore(&test_seqlock, flags);
    }
#[no_mangle]
unsafe extern "C" fn test_kernel_atomic_builtins() -> noinline void {
//
// Generate concurrent accesses, expecting no reports, ensuring KCSAN
// treats builtin atomics as actually atomic.
//
    __atomic_load_n(&test_var, __ATOMIC_RELAXED);
    }
#[no_mangle]
unsafe extern "C" fn test_kernel_xor_1bit() -> noinline void {
// Do not report data races between the read-writes.
    kcsan_nestable_atomic_begin();
    test_var ^= 0x10000;
    kcsan_nestable_atomic_end();
    }

    static noinline void test_kernel_##name(void)		
    {							
    let mut flag = &test_struct.val[0];		
    let mut v = 0;					
    if (!(acquire))					 {
    return;					
    }
    while (v++ < 100) {				
    test_var += 1;				
    barrier();				
    }						
    release;					
    test_delay(10);					
    }
    TEST_KERNEL_LOCKED(with_memorder,
    cmpxchg_acquire(flag, 0, 1) == 0,
    smp_store_release(flag, 0));
    TEST_KERNEL_LOCKED(wrong_memorder,
    cmpxchg_relaxed(flag, 0, 1) == 0,
    WRITE_ONCE(*flag, 0));
    TEST_KERNEL_LOCKED(atomic_builtin_with_memorder,
    __atomic_compare_exchange_n(flag, &v, 1, 0, __ATOMIC_ACQUIRE, __ATOMIC_RELAXED),
    __atomic_store_n(flag, 0, __ATOMIC_RELEASE));
    TEST_KERNEL_LOCKED(atomic_builtin_wrong_memorder,
    __atomic_compare_exchange_n(flag, &v, 1, 0, __ATOMIC_RELAXED, __ATOMIC_RELAXED),
    __atomic_store_n(flag, 0, __ATOMIC_RELAXED));
// ===== Test cases =====
//
// Tests that various barriers have the expected effect on internal state. Not
// exhaustive on atomic_t operations. Unlike the selftest, also checks for
// too-strict barrier instrumentation; these can be tolerated, because it does
// not cause false positives, but at least we should be aware of such cases.
//
#[no_mangle]
unsafe extern "C" fn test_barrier_nothreads(test: *mut kunit) {

    let mut reorder_access = &current.kcsan_ctx.reorder_access;

    let mut reorder_access = core::ptr::null_mut();

pub static mut arch_spinlock: arch_spinlock_t = 0;
pub static mut dummy: core::sync::atomic::AtomicI32 = 0;
    KCSAN_TEST_REQUIRES(test, reorder_access != core::ptr::null_mut());
    KCSAN_TEST_REQUIRES(test, IS_ENABLED!(CONFIG_SMP));

    do {											
    reorder_access.type = (access_type) | KCSAN_ACCESS_SCOPED;			
    reorder_access.size = sizeof!(test_var);					
    barrier;									
    KUNIT_EXPECT_EQ_MSG(test, reorder_access.size,					
    order_before ? 0 : sizeof!(test_var),			
    "improperly instrumented type=(" #access_type "): " name);	
    } while (0)

//
// Lockdep initialization can strengthen certain locking operations due
// to calling into instrumented files; "warm up" our locks.
//
    spin_lock(&test_spinlock);
    spin_unlock(&test_spinlock);
    mutex_lock(&test_mutex);
    mutex_unlock(&test_mutex);
// Force creating a valid entry in reorder_access first.
    test_var = 0;
    while (test_var++ < 1000000 && reorder_access.size != sizeof!(test_var)) {
    __kcsan_check_read(&test_var, sizeof!(test_var));
    }
    KUNIT_ASSERT_EQ(test, reorder_access.size, sizeof!(test_var));
    kcsan_nestable_atomic_begin(); /* No watchpoints in called functions. */
    KCSAN_EXPECT_READ_BARRIER(mb(), true);
    KCSAN_EXPECT_READ_BARRIER(wmb(), false);
    KCSAN_EXPECT_READ_BARRIER(rmb(), true);
    KCSAN_EXPECT_READ_BARRIER(smp_mb(), true);
    KCSAN_EXPECT_READ_BARRIER(smp_wmb(), false);
    KCSAN_EXPECT_READ_BARRIER(smp_rmb(), true);
    KCSAN_EXPECT_READ_BARRIER(dma_wmb(), false);
    KCSAN_EXPECT_READ_BARRIER(dma_rmb(), true);
    KCSAN_EXPECT_READ_BARRIER(smp_mb__before_atomic(), true);
    KCSAN_EXPECT_READ_BARRIER(smp_mb__after_atomic(), true);
    KCSAN_EXPECT_READ_BARRIER(smp_mb__after_spinlock(), true);
    KCSAN_EXPECT_READ_BARRIER(smp_store_mb(test_var, 0), true);
    KCSAN_EXPECT_READ_BARRIER(smp_load_acquire(&test_var), false);
    KCSAN_EXPECT_READ_BARRIER(smp_store_release(&test_var, 0), true);
    KCSAN_EXPECT_READ_BARRIER(xchg(&test_var, 0), true);
    KCSAN_EXPECT_READ_BARRIER(xchg_release(&test_var, 0), true);
    KCSAN_EXPECT_READ_BARRIER(xchg_relaxed(&test_var, 0), false);
    KCSAN_EXPECT_READ_BARRIER(cmpxchg(&test_var, 0,  0), true);
    KCSAN_EXPECT_READ_BARRIER(cmpxchg_release(&test_var, 0,  0), true);
    KCSAN_EXPECT_READ_BARRIER(cmpxchg_relaxed(&test_var, 0,  0), false);
    KCSAN_EXPECT_READ_BARRIER(atomic_read(&dummy), false);
    KCSAN_EXPECT_READ_BARRIER(atomic_read_acquire(&dummy), false);
    KCSAN_EXPECT_READ_BARRIER(atomic_set(&dummy, 0), false);
    KCSAN_EXPECT_READ_BARRIER(atomic_set_release(&dummy, 0), true);
    KCSAN_EXPECT_READ_BARRIER(atomic_add(1, &dummy), false);
    KCSAN_EXPECT_READ_BARRIER(atomic_add_return(1, &dummy), true);
    KCSAN_EXPECT_READ_BARRIER(atomic_add_return_acquire(1, &dummy), false);
    KCSAN_EXPECT_READ_BARRIER(atomic_add_return_release(1, &dummy), true);
    KCSAN_EXPECT_READ_BARRIER(atomic_add_return_relaxed(1, &dummy), false);
    KCSAN_EXPECT_READ_BARRIER(atomic_fetch_add(1, &dummy), true);
    KCSAN_EXPECT_READ_BARRIER(atomic_fetch_add_acquire(1, &dummy), false);
    KCSAN_EXPECT_READ_BARRIER(atomic_fetch_add_release(1, &dummy), true);
    KCSAN_EXPECT_READ_BARRIER(atomic_fetch_add_relaxed(1, &dummy), false);
    KCSAN_EXPECT_READ_BARRIER(test_and_set_bit(0, &test_var), true);
    KCSAN_EXPECT_READ_BARRIER(test_and_clear_bit(0, &test_var), true);
    KCSAN_EXPECT_READ_BARRIER(test_and_change_bit(0, &test_var), true);
    KCSAN_EXPECT_READ_BARRIER(clear_bit_unlock(0, &test_var), true);
    KCSAN_EXPECT_READ_BARRIER(__clear_bit_unlock(0, &test_var), true);
    KCSAN_EXPECT_READ_BARRIER(arch_spin_lock(&arch_spinlock), false);
    KCSAN_EXPECT_READ_BARRIER(arch_spin_unlock(&arch_spinlock), true);
    KCSAN_EXPECT_READ_BARRIER(spin_lock(&test_spinlock), false);
    KCSAN_EXPECT_READ_BARRIER(spin_unlock(&test_spinlock), true);
    KCSAN_EXPECT_READ_BARRIER(mutex_lock(&test_mutex), false);
    KCSAN_EXPECT_READ_BARRIER(mutex_unlock(&test_mutex), true);
    KCSAN_EXPECT_WRITE_BARRIER(mb(), true);
    KCSAN_EXPECT_WRITE_BARRIER(wmb(), true);
    KCSAN_EXPECT_WRITE_BARRIER(rmb(), false);
    KCSAN_EXPECT_WRITE_BARRIER(smp_mb(), true);
    KCSAN_EXPECT_WRITE_BARRIER(smp_wmb(), true);
    KCSAN_EXPECT_WRITE_BARRIER(smp_rmb(), false);
    KCSAN_EXPECT_WRITE_BARRIER(dma_wmb(), true);
    KCSAN_EXPECT_WRITE_BARRIER(dma_rmb(), false);
    KCSAN_EXPECT_WRITE_BARRIER(smp_mb__before_atomic(), true);
    KCSAN_EXPECT_WRITE_BARRIER(smp_mb__after_atomic(), true);
    KCSAN_EXPECT_WRITE_BARRIER(smp_mb__after_spinlock(), true);
    KCSAN_EXPECT_WRITE_BARRIER(smp_store_mb(test_var, 0), true);
    KCSAN_EXPECT_WRITE_BARRIER(smp_load_acquire(&test_var), false);
    KCSAN_EXPECT_WRITE_BARRIER(smp_store_release(&test_var, 0), true);
    KCSAN_EXPECT_WRITE_BARRIER(xchg(&test_var, 0), true);
    KCSAN_EXPECT_WRITE_BARRIER(xchg_release(&test_var, 0), true);
    KCSAN_EXPECT_WRITE_BARRIER(xchg_relaxed(&test_var, 0), false);
    KCSAN_EXPECT_WRITE_BARRIER(cmpxchg(&test_var, 0,  0), true);
    KCSAN_EXPECT_WRITE_BARRIER(cmpxchg_release(&test_var, 0,  0), true);
    KCSAN_EXPECT_WRITE_BARRIER(cmpxchg_relaxed(&test_var, 0,  0), false);
    KCSAN_EXPECT_WRITE_BARRIER(atomic_read(&dummy), false);
    KCSAN_EXPECT_WRITE_BARRIER(atomic_read_acquire(&dummy), false);
    KCSAN_EXPECT_WRITE_BARRIER(atomic_set(&dummy, 0), false);
    KCSAN_EXPECT_WRITE_BARRIER(atomic_set_release(&dummy, 0), true);
    KCSAN_EXPECT_WRITE_BARRIER(atomic_add(1, &dummy), false);
    KCSAN_EXPECT_WRITE_BARRIER(atomic_add_return(1, &dummy), true);
    KCSAN_EXPECT_WRITE_BARRIER(atomic_add_return_acquire(1, &dummy), false);
    KCSAN_EXPECT_WRITE_BARRIER(atomic_add_return_release(1, &dummy), true);
    KCSAN_EXPECT_WRITE_BARRIER(atomic_add_return_relaxed(1, &dummy), false);
    KCSAN_EXPECT_WRITE_BARRIER(atomic_fetch_add(1, &dummy), true);
    KCSAN_EXPECT_WRITE_BARRIER(atomic_fetch_add_acquire(1, &dummy), false);
    KCSAN_EXPECT_WRITE_BARRIER(atomic_fetch_add_release(1, &dummy), true);
    KCSAN_EXPECT_WRITE_BARRIER(atomic_fetch_add_relaxed(1, &dummy), false);
    KCSAN_EXPECT_WRITE_BARRIER(test_and_set_bit(0, &test_var), true);
    KCSAN_EXPECT_WRITE_BARRIER(test_and_clear_bit(0, &test_var), true);
    KCSAN_EXPECT_WRITE_BARRIER(test_and_change_bit(0, &test_var), true);
    KCSAN_EXPECT_WRITE_BARRIER(clear_bit_unlock(0, &test_var), true);
    KCSAN_EXPECT_WRITE_BARRIER(__clear_bit_unlock(0, &test_var), true);
    KCSAN_EXPECT_WRITE_BARRIER(arch_spin_lock(&arch_spinlock), false);
    KCSAN_EXPECT_WRITE_BARRIER(arch_spin_unlock(&arch_spinlock), true);
    KCSAN_EXPECT_WRITE_BARRIER(spin_lock(&test_spinlock), false);
    KCSAN_EXPECT_WRITE_BARRIER(spin_unlock(&test_spinlock), true);
    KCSAN_EXPECT_WRITE_BARRIER(mutex_lock(&test_mutex), false);
    KCSAN_EXPECT_WRITE_BARRIER(mutex_unlock(&test_mutex), true);
    KCSAN_EXPECT_RW_BARRIER(mb(), true);
    KCSAN_EXPECT_RW_BARRIER(wmb(), true);
    KCSAN_EXPECT_RW_BARRIER(rmb(), true);
    KCSAN_EXPECT_RW_BARRIER(smp_mb(), true);
    KCSAN_EXPECT_RW_BARRIER(smp_wmb(), true);
    KCSAN_EXPECT_RW_BARRIER(smp_rmb(), true);
    KCSAN_EXPECT_RW_BARRIER(dma_wmb(), true);
    KCSAN_EXPECT_RW_BARRIER(dma_rmb(), true);
    KCSAN_EXPECT_RW_BARRIER(smp_mb__before_atomic(), true);
    KCSAN_EXPECT_RW_BARRIER(smp_mb__after_atomic(), true);
    KCSAN_EXPECT_RW_BARRIER(smp_mb__after_spinlock(), true);
    KCSAN_EXPECT_RW_BARRIER(smp_store_mb(test_var, 0), true);
    KCSAN_EXPECT_RW_BARRIER(smp_load_acquire(&test_var), false);
    KCSAN_EXPECT_RW_BARRIER(smp_store_release(&test_var, 0), true);
    KCSAN_EXPECT_RW_BARRIER(xchg(&test_var, 0), true);
    KCSAN_EXPECT_RW_BARRIER(xchg_release(&test_var, 0), true);
    KCSAN_EXPECT_RW_BARRIER(xchg_relaxed(&test_var, 0), false);
    KCSAN_EXPECT_RW_BARRIER(cmpxchg(&test_var, 0,  0), true);
    KCSAN_EXPECT_RW_BARRIER(cmpxchg_release(&test_var, 0,  0), true);
    KCSAN_EXPECT_RW_BARRIER(cmpxchg_relaxed(&test_var, 0,  0), false);
    KCSAN_EXPECT_RW_BARRIER(atomic_read(&dummy), false);
    KCSAN_EXPECT_RW_BARRIER(atomic_read_acquire(&dummy), false);
    KCSAN_EXPECT_RW_BARRIER(atomic_set(&dummy, 0), false);
    KCSAN_EXPECT_RW_BARRIER(atomic_set_release(&dummy, 0), true);
    KCSAN_EXPECT_RW_BARRIER(atomic_add(1, &dummy), false);
    KCSAN_EXPECT_RW_BARRIER(atomic_add_return(1, &dummy), true);
    KCSAN_EXPECT_RW_BARRIER(atomic_add_return_acquire(1, &dummy), false);
    KCSAN_EXPECT_RW_BARRIER(atomic_add_return_release(1, &dummy), true);
    KCSAN_EXPECT_RW_BARRIER(atomic_add_return_relaxed(1, &dummy), false);
    KCSAN_EXPECT_RW_BARRIER(atomic_fetch_add(1, &dummy), true);
    KCSAN_EXPECT_RW_BARRIER(atomic_fetch_add_acquire(1, &dummy), false);
    KCSAN_EXPECT_RW_BARRIER(atomic_fetch_add_release(1, &dummy), true);
    KCSAN_EXPECT_RW_BARRIER(atomic_fetch_add_relaxed(1, &dummy), false);
    KCSAN_EXPECT_RW_BARRIER(test_and_set_bit(0, &test_var), true);
    KCSAN_EXPECT_RW_BARRIER(test_and_clear_bit(0, &test_var), true);
    KCSAN_EXPECT_RW_BARRIER(test_and_change_bit(0, &test_var), true);
    KCSAN_EXPECT_RW_BARRIER(clear_bit_unlock(0, &test_var), true);
    KCSAN_EXPECT_RW_BARRIER(__clear_bit_unlock(0, &test_var), true);
    KCSAN_EXPECT_RW_BARRIER(arch_spin_lock(&arch_spinlock), false);
    KCSAN_EXPECT_RW_BARRIER(arch_spin_unlock(&arch_spinlock), true);
    KCSAN_EXPECT_RW_BARRIER(spin_lock(&test_spinlock), false);
    KCSAN_EXPECT_RW_BARRIER(spin_unlock(&test_spinlock), true);
    KCSAN_EXPECT_RW_BARRIER(mutex_lock(&test_mutex), false);
    KCSAN_EXPECT_RW_BARRIER(mutex_unlock(&test_mutex), true);
    KCSAN_EXPECT_READ_BARRIER(xor_unlock_is_negative_byte(1, &test_var), true);
    KCSAN_EXPECT_WRITE_BARRIER(xor_unlock_is_negative_byte(1, &test_var), true);
    KCSAN_EXPECT_RW_BARRIER(xor_unlock_is_negative_byte(1, &test_var), true);
    kcsan_nestable_atomic_end();
    }
// Simple test with normal data race.
    __no_kcsan
#[no_mangle]
unsafe extern "C" fn test_basic(test: *mut kunit) {
pub static mut expect_report: usize = 0;
pub static mut expect_report: usize = 0;
pub static mut match_expect: bool = false;
pub static mut match_never: bool = false;
    begin_test_checks(test_kernel_write, test_kernel_read);
    do {
    match_expect |= report_matches(&expect);
    match_never = report_matches(&never);
    } while (!end_test_checks(match_never));
    KUNIT_EXPECT_TRUE(test, match_expect);
    KUNIT_EXPECT_FALSE(test, match_never);
    }
//
// Stress KCSAN with lots of concurrent races on different addresses until
// timeout.
//
    __no_kcsan
#[no_mangle]
unsafe extern "C" fn test_concurrent_races(test: *mut kunit) {
pub static mut expect_report: usize = 0;
pub static mut expect_report: usize = 0;
pub static mut match_expect: bool = false;
pub static mut match_never: bool = false;
    begin_test_checks(test_kernel_rmw_array, test_kernel_rmw_array);
    do {
    match_expect |= report_matches(&expect);
    match_never |= report_matches(&never);
    } while (!end_test_checks(false));
    KUNIT_EXPECT_TRUE(test, match_expect); /* Sanity check matches exist. */
    KUNIT_EXPECT_FALSE(test, match_never);
    }
// Test the KCSAN_REPORT_VALUE_CHANGE_ONLY option.
    __no_kcsan
#[no_mangle]
unsafe extern "C" fn test_novalue_change(test: *mut kunit) {
pub static mut expect_report: usize = 0;
pub static mut expect_report: usize = 0;
pub static mut match_expect: bool = false;
    test_kernel_write_nochange(); /* Reset value. */
    begin_test_checks(test_kernel_write_nochange, test_kernel_read);
    do {
    match_expect = report_matches(&expect_rw) || report_matches(&expect_ww);
    } while (!end_test_checks(match_expect));
    if (IS_ENABLED!(CONFIG_KCSAN_REPORT_VALUE_CHANGE_ONLY)) {
    KUNIT_EXPECT_FALSE(test, match_expect);
    }
    else {
    KUNIT_EXPECT_TRUE(test, match_expect);
    }
    }
//
// Test that the rules where the KCSAN_REPORT_VALUE_CHANGE_ONLY option should
// never apply work.
//
    __no_kcsan
#[no_mangle]
unsafe extern "C" fn test_novalue_change_exception(test: *mut kunit) {
pub static mut expect_report: usize = 0;
pub static mut expect_report: usize = 0;
pub static mut match_expect: bool = false;
    test_kernel_write_nochange_rcu(); /* Reset value. */
    begin_test_checks(test_kernel_write_nochange_rcu, test_kernel_read);
    do {
    match_expect = report_matches(&expect_rw) || report_matches(&expect_ww);
    } while (!end_test_checks(match_expect));
    KUNIT_EXPECT_TRUE(test, match_expect);
    }
// Test that data races of unknown origin are reported.
    __no_kcsan
#[no_mangle]
unsafe extern "C" fn test_unknown_origin(test: *mut kunit) {
pub static mut expect_report: usize = 0;
pub static mut match_expect: bool = false;
    begin_test_checks(test_kernel_write_uninstrumented, test_kernel_read);
    do {
    match_expect = report_matches(&expect);
    } while (!end_test_checks(match_expect));
    if (IS_ENABLED!(CONFIG_KCSAN_REPORT_RACE_UNKNOWN_ORIGIN)) {
    KUNIT_EXPECT_TRUE(test, match_expect);
    }
    else {
    KUNIT_EXPECT_FALSE(test, match_expect);
    }
    }
// Test KCSAN_ASSUME_PLAIN_WRITES_ATOMIC if it is selected.
    __no_kcsan
#[no_mangle]
unsafe extern "C" fn test_write_write_assume_atomic(test: *mut kunit) {
pub static mut expect_report: usize = 0;
pub static mut match_expect: bool = false;
    begin_test_checks(test_kernel_write, test_kernel_write);
    do {
    sink_value(READ_ONCE(test_var)); /* induce value-change */
    match_expect = report_matches(&expect);
    } while (!end_test_checks(match_expect));
    if (IS_ENABLED!(CONFIG_KCSAN_ASSUME_PLAIN_WRITES_ATOMIC)) {
    KUNIT_EXPECT_FALSE(test, match_expect);
    }
    else {
    KUNIT_EXPECT_TRUE(test, match_expect);
    }
    }
//
// Test that data races with writes larger than word-size are always reported,
// even if KCSAN_ASSUME_PLAIN_WRITES_ATOMIC is selected.
//
    __no_kcsan
#[no_mangle]
unsafe extern "C" fn test_write_write_struct(test: *mut kunit) {
pub static mut expect_report: usize = 0;
pub static mut match_expect: bool = false;
    begin_test_checks(test_kernel_write_struct, test_kernel_write_struct);
    do {
    match_expect = report_matches(&expect);
    } while (!end_test_checks(match_expect));
    KUNIT_EXPECT_TRUE(test, match_expect);
    }
//
// Test that data races where only one write is larger than word-size are always
// reported, even if KCSAN_ASSUME_PLAIN_WRITES_ATOMIC is selected.
//
    __no_kcsan
#[no_mangle]
unsafe extern "C" fn test_write_write_struct_part(test: *mut kunit) {
pub static mut expect_report: usize = 0;
pub static mut match_expect: bool = false;
    begin_test_checks(test_kernel_write_struct, test_kernel_write_struct_part);
    do {
    match_expect = report_matches(&expect);
    } while (!end_test_checks(match_expect));
    KUNIT_EXPECT_TRUE(test, match_expect);
    }
// Test that races with atomic accesses never result in reports.
    __no_kcsan
#[no_mangle]
unsafe extern "C" fn test_read_atomic_write_atomic(test: *mut kunit) {
pub static mut match_never: bool = false;
    begin_test_checks(test_kernel_read_atomic, test_kernel_write_atomic);
    do {
    match_never = report_available();
    } while (!end_test_checks(match_never));
    KUNIT_EXPECT_FALSE(test, match_never);
    }
// Test that a race with an atomic and plain access result in reports.
    __no_kcsan
#[no_mangle]
unsafe extern "C" fn test_read_plain_atomic_write(test: *mut kunit) {
pub static mut expect_report: usize = 0;
pub static mut match_expect: bool = false;
    KCSAN_TEST_REQUIRES(test, !IS_ENABLED!(CONFIG_KCSAN_IGNORE_ATOMICS));
    begin_test_checks(test_kernel_read, test_kernel_write_atomic);
    do {
    match_expect = report_matches(&expect);
    } while (!end_test_checks(match_expect));
    KUNIT_EXPECT_TRUE(test, match_expect);
    }
// Test that atomic RMWs generate correct report.
    __no_kcsan
#[no_mangle]
unsafe extern "C" fn test_read_plain_atomic_rmw(test: *mut kunit) {
pub static mut expect_report: usize = 0;
pub static mut match_expect: bool = false;
    KCSAN_TEST_REQUIRES(test, !IS_ENABLED!(CONFIG_KCSAN_IGNORE_ATOMICS));
    begin_test_checks(test_kernel_read, test_kernel_atomic_rmw);
    do {
    match_expect = report_matches(&expect);
    } while (!end_test_checks(match_expect));
    KUNIT_EXPECT_TRUE(test, match_expect);
    }
// Zero-sized accesses should never cause data race reports.
    __no_kcsan
#[no_mangle]
unsafe extern "C" fn test_zero_size_access(test: *mut kunit) {
pub static mut expect_report: usize = 0;
pub static mut expect_report: usize = 0;
pub static mut match_expect: bool = false;
pub static mut match_never: bool = false;
    begin_test_checks(test_kernel_write_struct, test_kernel_read_struct_zero_size);
    do {
    match_expect |= report_matches(&expect);
    match_never = report_matches(&never);
    } while (!end_test_checks(match_never));
    KUNIT_EXPECT_TRUE(test, match_expect); /* Sanity check. */
    KUNIT_EXPECT_FALSE(test, match_never);
    }
// Test the data_race() macro.
    __no_kcsan
#[no_mangle]
unsafe extern "C" fn test_data_race(test: *mut kunit) {
pub static mut match_never: bool = false;
    begin_test_checks(test_kernel_data_race, test_kernel_data_race);
    do {
    match_never = report_available();
    } while (!end_test_checks(match_never));
    KUNIT_EXPECT_FALSE(test, match_never);
    }
// Test the __data_racy type qualifier.
    __no_kcsan
#[no_mangle]
unsafe extern "C" fn test_data_racy_qualifier(test: *mut kunit) {
pub static mut match_never: bool = false;
    begin_test_checks(test_kernel_data_racy_qualifier, test_kernel_data_racy_qualifier);
    do {
    match_never = report_available();
    } while (!end_test_checks(match_never));
    KUNIT_EXPECT_FALSE(test, match_never);
    }
    __no_kcsan
#[no_mangle]
unsafe extern "C" fn test_assert_exclusive_writer(test: *mut kunit) {
pub static mut expect_report: usize = 0;
pub static mut match_expect: bool = false;
    begin_test_checks(test_kernel_assert_writer, test_kernel_write_nochange);
    do {
    match_expect = report_matches(&expect);
    } while (!end_test_checks(match_expect));
    KUNIT_EXPECT_TRUE(test, match_expect);
    }
    __no_kcsan
#[no_mangle]
unsafe extern "C" fn test_assert_exclusive_access(test: *mut kunit) {
pub static mut expect_report: usize = 0;
pub static mut match_expect: bool = false;
    begin_test_checks(test_kernel_assert_access, test_kernel_read);
    do {
    match_expect = report_matches(&expect);
    } while (!end_test_checks(match_expect));
    KUNIT_EXPECT_TRUE(test, match_expect);
    }
    __no_kcsan
#[no_mangle]
unsafe extern "C" fn test_assert_exclusive_access_writer(test: *mut kunit) {
pub static mut expect_report: usize = 0;
pub static mut expect_report: usize = 0;
pub static mut expect_report: usize = 0;
pub static mut match_expect_access_writer: bool = false;
pub static mut match_expect_access_access: bool = false;
pub static mut match_never: bool = false;
    begin_test_checks(test_kernel_assert_access, test_kernel_assert_writer);
    do {
    match_expect_access_writer |= report_matches(&expect_access_writer);
    match_expect_access_access |= report_matches(&expect_access_access);
    match_never |= report_matches(&never);
    } while (!end_test_checks(match_never));
    KUNIT_EXPECT_TRUE(test, match_expect_access_writer);
    KUNIT_EXPECT_TRUE(test, match_expect_access_access);
    KUNIT_EXPECT_FALSE(test, match_never);
    }
    __no_kcsan
#[no_mangle]
unsafe extern "C" fn test_assert_exclusive_bits_change(test: *mut kunit) {
pub static mut expect_report: usize = 0;
pub static mut match_expect: bool = false;
    begin_test_checks(test_kernel_assert_bits_change, test_kernel_change_bits);
    do {
    match_expect = report_matches(&expect);
    } while (!end_test_checks(match_expect));
    KUNIT_EXPECT_TRUE(test, match_expect);
    }
    __no_kcsan
#[no_mangle]
unsafe extern "C" fn test_assert_exclusive_bits_nochange(test: *mut kunit) {
pub static mut match_never: bool = false;
    begin_test_checks(test_kernel_assert_bits_nochange, test_kernel_change_bits);
    do {
    match_never = report_available();
    } while (!end_test_checks(match_never));
    KUNIT_EXPECT_FALSE(test, match_never);
    }
    __no_kcsan
#[no_mangle]
unsafe extern "C" fn test_assert_exclusive_writer_scoped(test: *mut kunit) {
pub static mut expect_report: usize = 0;
pub static mut expect_report: usize = 0;
pub static mut match_expect_start: bool = false;
pub static mut match_expect_inscope: bool = false;
    begin_test_checks(test_kernel_assert_writer_scoped, test_kernel_write_nochange);
    do {
    match_expect_start |= report_matches(&expect_start);
    match_expect_inscope |= report_matches(&expect_inscope);
    } while (!end_test_checks(match_expect_inscope));
    KUNIT_EXPECT_TRUE(test, match_expect_start);
    KUNIT_EXPECT_FALSE(test, match_expect_inscope);
    }
    __no_kcsan
#[no_mangle]
unsafe extern "C" fn test_assert_exclusive_access_scoped(test: *mut kunit) {
pub static mut expect_report: usize = 0;
pub static mut expect_report: usize = 0;
pub static mut expect_report: usize = 0;
pub static mut match_expect_start: bool = false;
pub static mut match_expect_inscope: bool = false;
    begin_test_checks(test_kernel_assert_access_scoped, test_kernel_read);
    end_time += msecs_to_jiffies(1000); /* This test requires a bit more time. */
    do {
    match_expect_start |= report_matches(&expect_start1) || report_matches(&expect_start2);
    match_expect_inscope |= report_matches(&expect_inscope);
    } while (!end_test_checks(match_expect_inscope));
    KUNIT_EXPECT_TRUE(test, match_expect_start);
    KUNIT_EXPECT_FALSE(test, match_expect_inscope);
    }
//
// jiffies is special (declared to be volatile) and its accesses are typically
// not marked; this test ensures that the compiler nor KCSAN gets confused about
// jiffies's declaration on different architectures.
//
    __no_kcsan
#[no_mangle]
unsafe extern "C" fn test_jiffies_noreport(test: *mut kunit) {
pub static mut match_never: bool = false;
    begin_test_checks(test_kernel_jiffies_reader, test_kernel_jiffies_reader);
    do {
    match_never = report_available();
    } while (!end_test_checks(match_never));
    KUNIT_EXPECT_FALSE(test, match_never);
    }
// Test that racing accesses in seqlock critical sections are not reported.
    __no_kcsan
#[no_mangle]
unsafe extern "C" fn test_seqlock_noreport(test: *mut kunit) {
pub static mut match_never: bool = false;
    begin_test_checks(test_kernel_seqlock_reader, test_kernel_seqlock_writer);
    do {
    match_never = report_available();
    } while (!end_test_checks(match_never));
    KUNIT_EXPECT_FALSE(test, match_never);
    }
//
// Test atomic builtins work and required instrumentation functions exist. We
// also test that KCSAN understands they're atomic by racing with them via
// test_kernel_atomic_builtins(), and expect no reports.
//
// The atomic builtins _SHOULD NOT_ be used in normal kernel code!
//
#[no_mangle]
unsafe extern "C" fn test_atomic_builtins(test: *mut kunit) {
pub static mut match_never: bool = false;
    begin_test_checks(test_kernel_atomic_builtins, test_kernel_atomic_builtins);
    do {
    let mut tmp = 0;
    kcsan_enable_current();
    __atomic_store_n(&test_var, 42L, __ATOMIC_RELAXED);
    KUNIT_EXPECT_EQ(test, 42L, __atomic_load_n(&test_var, __ATOMIC_RELAXED));
    KUNIT_EXPECT_EQ(test, 42L, __atomic_exchange_n(&test_var, 20, __ATOMIC_RELAXED));
    KUNIT_EXPECT_EQ(test, 20L, test_var);
    tmp = 20L;
    KUNIT_EXPECT_TRUE(test, __atomic_compare_exchange_n(&test_var, &tmp, 30L,
    0, __ATOMIC_RELAXED,
    __ATOMIC_RELAXED));
    KUNIT_EXPECT_EQ(test, tmp, 20L);
    KUNIT_EXPECT_EQ(test, test_var, 30L);
    KUNIT_EXPECT_FALSE(test, __atomic_compare_exchange_n(&test_var, &tmp, 40L,
    1, __ATOMIC_RELAXED,
    __ATOMIC_RELAXED));
    KUNIT_EXPECT_EQ(test, tmp, 30L);
    KUNIT_EXPECT_EQ(test, test_var, 30L);
    KUNIT_EXPECT_EQ(test, 30L, __atomic_fetch_add(&test_var, 1, __ATOMIC_RELAXED));
    KUNIT_EXPECT_EQ(test, 31L, __atomic_fetch_sub(&test_var, 1, __ATOMIC_RELAXED));
    KUNIT_EXPECT_EQ(test, 30L, __atomic_fetch_and(&test_var, 0xf, __ATOMIC_RELAXED));
    KUNIT_EXPECT_EQ(test, 14L, __atomic_fetch_xor(&test_var, 0xf, __ATOMIC_RELAXED));
    KUNIT_EXPECT_EQ(test, 1L, __atomic_fetch_or(&test_var, 0xf0, __ATOMIC_RELAXED));
    KUNIT_EXPECT_EQ(test, 241L, __atomic_fetch_nand(&test_var, 0xf, __ATOMIC_RELAXED));
    KUNIT_EXPECT_EQ(test, -2L, test_var);
    __atomic_thread_fence(__ATOMIC_SEQ_CST);
    __atomic_signal_fence(__ATOMIC_SEQ_CST);
    kcsan_disable_current();
    match_never = report_available();
    } while (!end_test_checks(match_never));
    KUNIT_EXPECT_FALSE(test, match_never);
    }
    __no_kcsan
#[no_mangle]
unsafe extern "C" fn test_1bit_value_change(test: *mut kunit) {
pub static mut expect_report: usize = 0;
pub static mut match: bool = false;
    begin_test_checks(test_kernel_read, test_kernel_xor_1bit);
    do {
    match = IS_ENABLED!(CONFIG_KCSAN_PERMISSIVE)
    ? report_available()
    : report_matches(&expect);
    } while (!end_test_checks(match));
    if (IS_ENABLED!(CONFIG_KCSAN_PERMISSIVE)) {
    KUNIT_EXPECT_FALSE(test, match);
    }
    else {
    KUNIT_EXPECT_TRUE(test, match);
    }
    }
    __no_kcsan
#[no_mangle]
unsafe extern "C" fn test_correct_barrier(test: *mut kunit) {
pub static mut expect_report: usize = 0;
pub static mut match_expect: bool = false;
    test_struct.val[0] = 0; /* init unlocked */
    begin_test_checks(test_kernel_with_memorder, test_kernel_with_memorder);
    do {
    match_expect = report_matches_any_reordered(&expect);
    } while (!end_test_checks(match_expect));
    KUNIT_EXPECT_FALSE(test, match_expect);
    }
    __no_kcsan
#[no_mangle]
unsafe extern "C" fn test_missing_barrier(test: *mut kunit) {
pub static mut expect_report: usize = 0;
pub static mut match_expect: bool = false;
    test_struct.val[0] = 0; /* init unlocked */
    begin_test_checks(test_kernel_wrong_memorder, test_kernel_wrong_memorder);
    do {
    match_expect = report_matches_any_reordered(&expect);
    } while (!end_test_checks(match_expect));
    if (IS_ENABLED!(CONFIG_KCSAN_WEAK_MEMORY)) {
    KUNIT_EXPECT_TRUE(test, match_expect);
    }
    else {
    KUNIT_EXPECT_FALSE(test, match_expect);
    }
    }
    __no_kcsan
#[no_mangle]
unsafe extern "C" fn test_atomic_builtins_correct_barrier(test: *mut kunit) {
pub static mut expect_report: usize = 0;
pub static mut match_expect: bool = false;
    test_struct.val[0] = 0; /* init unlocked */
    begin_test_checks(test_kernel_atomic_builtin_with_memorder,
    test_kernel_atomic_builtin_with_memorder);
    do {
    match_expect = report_matches_any_reordered(&expect);
    } while (!end_test_checks(match_expect));
    KUNIT_EXPECT_FALSE(test, match_expect);
    }
    __no_kcsan
#[no_mangle]
unsafe extern "C" fn test_atomic_builtins_missing_barrier(test: *mut kunit) {
pub static mut expect_report: usize = 0;
pub static mut match_expect: bool = false;
    test_struct.val[0] = 0; /* init unlocked */
    begin_test_checks(test_kernel_atomic_builtin_wrong_memorder,
    test_kernel_atomic_builtin_wrong_memorder);
    do {
    match_expect = report_matches_any_reordered(&expect);
    } while (!end_test_checks(match_expect));
    if (IS_ENABLED!(CONFIG_KCSAN_WEAK_MEMORY)) {
    KUNIT_EXPECT_TRUE(test, match_expect);
    }
    else {
    KUNIT_EXPECT_FALSE(test, match_expect);
    }
    }
//
// Generate thread counts for all test cases. Values generated are in interval
// [2, 5] followed by exponentially increasing thread counts from 8 to 32.
//
// The thread counts are chosen to cover potentially interesting boundaries and
// corner cases (2 to 5), and then stress the system with larger counts.
//
    static const void *nthreads_gen_params(kunit *test, const void *prev, char *desc)
    {
pub static mut nthreads: c_long = 0;
    if (nthreads < 0 || nthreads >= 32) {
    nthreads = 0; /* stop */
    }

    else if (!nthreads) {
    nthreads = 2; /* initial value */
    }

    else if (nthreads < 5) {
    nthreads += 1;
    }

    else if (nthreads == 5) {
    nthreads = 8;
    }
    else {
    nthreads *= 2;
    }
    if (!preempt_model_preemptible() ||
    !IS_ENABLED!(CONFIG_KCSAN_INTERRUPT_WATCHER)) {
//
// Without any preemption, keep 2 CPUs free for other tasks, one
// of which is the main test case function checking for
// completion or failure.
//
pub static mut min_unused_cpus: c_long = 0;
pub static mut min_required_cpus: c_long = 0;
    if (num_online_cpus() < min_required_cpus) {
    pr_err_once("Too few online CPUs (%u < %ld) for test\n",
    num_online_cpus(), min_required_cpus);
    nthreads = 0;
    } else if (nthreads >= num_online_cpus() - min_unused_cpus) {
// Use negative value to indicate last param.
    nthreads = -(num_online_cpus() - min_unused_cpus);
    pr_warn_once("Limiting number of threads to %ld (only %d online CPUs)\n",
    -nthreads, num_online_cpus());
    }
    }
    snprintf(desc, KUNIT_PARAM_DESC_SIZE, "threads=%ld", abs(nthreads));
    return nthreads;
    }

pub static mut kunit_case: usize = 0;
// ===== End test cases =====
// Concurrent accesses from interrupts.
    __no_kcsan
#[no_mangle]
unsafe extern "C" fn access_thread_timer(timer: *mut timer_list) {
pub static mut cnt: atomic_t = 0;
    let mut idx = 0;
    void (*func)(void);
    idx = (unsigned int)atomic_inc_return(&cnt) % ARRAY_SIZE!(access_kernels);
// Acquire potential initialization.
    func = smp_load_acquire(&access_kernels[idx]);
    if (func) {
    func();
    }
    }
// The main loop for each thread.
    __no_kcsan
#[no_mangle]
unsafe extern "C" fn access_thread(arg: *mut c_void) -> c_int {
pub static mut timer: usize = 0;
pub static mut cnt: c_uint = 0;
    let mut idx = 0;
    void (*func)(void);
    timer_setup_on_stack(&timer, access_thread_timer, 0);
    do {
    might_sleep();
    if (!timer_pending(&timer)) {
    mod_timer(&timer, jiffies + 1);
    }
    else {
// Iterate through all kernels.
    idx = cnt++ % ARRAY_SIZE!(access_kernels);
// Acquire potential initialization.
    func = smp_load_acquire(&access_kernels[idx]);
    if (func) {
    func();
    }
    }
    } while (!torture_must_stop());
    timer_delete_sync(&timer);
    timer_destroy_on_stack(&timer);
    torture_kthread_stopping("access_thread");
    return 0;
    }
    __no_kcsan
#[no_mangle]
unsafe extern "C" fn test_init(test: *mut kunit) -> c_int {
    let mut flags = 0;
    let mut nthreads = 0;
    let mut i = 0;
    spin_lock_irqsave(&observed.lock, flags);
    for (i = 0; i < ARRAY_SIZE!(observed.lines); ++i) {
    observed.lines[i][0] = '\0';
    }
    observed.nlines = 0;
    spin_unlock_irqrestore(&observed.lock, flags);
    if (strstr(test.name, "nothreads")) {
    return 0;
    }
    if (!torture_init_begin(test.name, 1)) {
    return -EBUSY;
    }
    if (WARN_ON!(threads)) {
// goto;
    }
    while (i < ARRAY_SIZE!(access_kernels)) {
    if (WARN_ON!(access_kernels[i])) {
// goto;
    }
    }
    nthreads = abs((long)test.param_value);
    if (WARN_ON!(!nthreads)) {
// goto;
    }
    threads = kzalloc_objs(task_struct *, nthreads + 1);
    if (WARN_ON!(!threads)) {
// goto;
    }
    threads[nthreads] = core::ptr::null_mut();
    while (i < nthreads) {
    if (torture_create_kthread(access_thread, core::ptr::null_mut(), threads[i])) {
// goto;
    }
    }
    torture_init_end();
    return 0;
// label;
    kfree(threads);
    threads = core::ptr::null_mut();
    torture_init_end();
    return -EINVAL;
    }
    __no_kcsan
#[no_mangle]
unsafe extern "C" fn test_exit(test: *mut kunit) {
pub static mut stop_thread: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    if (strstr(test.name, "nothreads")) {
    return;
    }
    if (torture_cleanup_begin()) {
    return;
    }
    for (i = 0; i < ARRAY_SIZE!(access_kernels); ++i) {
    WRITE_ONCE(access_kernels[i], core::ptr::null_mut());
    }
    if (threads) {
    for (stop_thread = threads; *stop_thread; stop_thread++) {
    torture_stop_kthread(reader_thread, *stop_thread);
    }
    kfree(threads);
    threads = core::ptr::null_mut();
    }
    torture_cleanup_end();
    }
    __no_kcsan
#[no_mangle]
unsafe extern "C" fn register_tracepoints() {
    register_trace_console(probe_console, core::ptr::null_mut());
    }
    __no_kcsan
#[no_mangle]
unsafe extern "C" fn unregister_tracepoints() {
    unregister_trace_console(probe_console, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn kcsan_suite_init(suite: *mut kunit_suite) -> c_int {
    register_tracepoints();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn kcsan_suite_exit(suite: *mut kunit_suite) {
    unregister_tracepoints();
    tracepoint_synchronize_unregister();
    }
pub static mut kunit_suite: usize = 0;
    kunit_test_suites(&kcsan_test_suite);
    MODULE_DESCRIPTION("KCSAN test suite");
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Marco Elver <elver@google.com>");