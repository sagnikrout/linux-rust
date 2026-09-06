//! Automatically rewritten from C to Rust
//! Source: mm/kfence/kfence_test.c
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
// Test cases for KFENCE memory safety error detector. Since the interface with
// which KFENCE's reports are obtained is via the console, this is the output we
// should verify. For each test case checks the presence (or absence) of
// generated reports. Relies on 'console' tracepoint to capture reports as they
// appear in the kernel log.
//
// Copyright (C) 2020, Google LLC.
// Author: Alexander Potapenko <glider@google.com>
// Marco Elver <elver@google.com>
//

// May be overridden by <asm/kfence.h>.

    if (!(cond))						 {
    kunit_skip((test), "Test requires: " #cond);	
    }
    } while (0)
// Report as observed from console.
pub static mut observed: usize = 0;
// Probe for console output: obtains observed lines of interest.
#[no_mangle]
unsafe extern "C" fn probe_console(ignore: *mut c_void, buf: *const c_char, len: usize) {
    let mut flags = 0;
    let mut nlines = 0;
    spin_lock_irqsave(&observed.lock, flags);
    nlines = observed.nlines;
    if (strnstr(buf, "BUG: KFENCE: ", len) && strnstr(buf, "test_", len)) {
//
// KFENCE report and related to the test.
//
// The provided @buf is not NUL-terminated; copy no more than
// @len bytes and let strscpy() add the missing NUL-terminator.
//
    strscpy(observed.lines[0], buf, min(len + 1, sizeof!(observed.lines[0])));
    nlines = 1;
    } else if (nlines == 1 && (strnstr(buf, "at 0x", len) || strnstr(buf, "of 0x", len))) {
    strscpy(observed.lines[nlines++], buf, min(len + 1, sizeof!(observed.lines[0])));
    }
    WRITE_ONCE(observed.nlines, nlines); /* Publish new nlines. */
    spin_unlock_irqrestore(&observed.lock, flags);
    }
// Check if a report related to the test exists.
#[no_mangle]
unsafe extern "C" fn report_available() -> bool {
    return READ_ONCE(observed.nlines) == ARRAY_SIZE!(observed.lines);
    }
// Information we expect in a report.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct expect_report {
//     pub /: *mut *mut enum kfence_error_type type; / The type or error.,
//     pub /: *mut *mut *mut c_void fn; / Function pointer to expected function where access occurred.,
//     pub /: *mut *mut *mut char addr; / Address at which the bad access occurred.,
//     pub /: *mut *mut bool is_write; / Is access a write.,
}

    static const char *get_access_type(const struct expect_report *r)
    {
    return str_write_read(r.is_write);
    }
// Check observed report matches information in @r.
#[no_mangle]
unsafe extern "C" fn report_matches(r: *const expect_report) -> bool {
pub static mut addr: c_ulong = 0;
pub static mut ret: bool = false;
    let mut flags = 0;
    typeof(observed.lines) expect;
pub static mut end: *mut c_void = core::ptr::null_mut();
pub static mut cur: *mut c_void = core::ptr::null_mut();
// Doubled-checked locking.
    if (!report_available()) {
    return false;
    }
// Generate expected report contents.
// Title
    cur = expect[0];
    end = ARRAY_END(expect[0]);
    match (r.type) {
    KFENCE_ERROR_OOB => {
    cur += scnprintf(cur, end - cur, "BUG: KFENCE: out-of-bounds %s",
    get_access_type(r));
    // break;
    }
    KFENCE_ERROR_UAF => {
    cur += scnprintf(cur, end - cur, "BUG: KFENCE: use-after-free %s",
    get_access_type(r));
    // break;
    }
    KFENCE_ERROR_CORRUPTION => {
    cur += scnprintf(cur, end - cur, "BUG: KFENCE: memory corruption");
    // break;
    }
    KFENCE_ERROR_INVALID => {
    cur += scnprintf(cur, end - cur, "BUG: KFENCE: invalid %s",
    get_access_type(r));
    // break;
    }
    KFENCE_ERROR_INVALID_FREE => {
    cur += scnprintf(cur, end - cur, "BUG: KFENCE: invalid free");
    // break;
    }
    }
    scnprintf(cur, end - cur, " in %pS", r.fn);
// The exact offset won't match, remove it; also strip module name.
    cur = strchr(expect[0], '+');
    if (cur) {
// cur = '\0';
    }
// Access information
    cur = expect[1];
    end = ARRAY_END(expect[1]);
    match (r.type) {
    KFENCE_ERROR_OOB => {
    cur += scnprintf(cur, end - cur, "Out-of-bounds %s at", get_access_type(r));
    addr = arch_kfence_test_address(addr);
    // break;
    }
    KFENCE_ERROR_UAF => {
    cur += scnprintf(cur, end - cur, "Use-after-free %s at", get_access_type(r));
    addr = arch_kfence_test_address(addr);
    // break;
    }
    KFENCE_ERROR_CORRUPTION => {
    cur += scnprintf(cur, end - cur, "Corrupted memory at");
    // break;
    }
    KFENCE_ERROR_INVALID => {
    cur += scnprintf(cur, end - cur, "Invalid %s at", get_access_type(r));
    addr = arch_kfence_test_address(addr);
    // break;
    }
    KFENCE_ERROR_INVALID_FREE => {
    cur += scnprintf(cur, end - cur, "Invalid free of");
    // break;
    }
    }
    cur += scnprintf(cur, end - cur, " 0x%p", addr);
    spin_lock_irqsave(&observed.lock, flags);
    if (!report_available()) {
// goto; /* A new report is being captured. */
    }
// Finally match expected output to what we actually observed.
    ret = strstr(observed.lines[0], expect[0]) && strstr(observed.lines[1], expect[1]);
// label;
    spin_unlock_irqrestore(&observed.lock, flags);
    return ret;
    }
// ===== Test cases =====

// Cache used by tests; if NULL, allocate from kmalloc instead.
pub static mut test_cache: *mut c_void = core::ptr::null_mut();
#[no_mangle]
pub unsafe extern "C" fn setup_test_cache(test: *mut kunit, size: size_t, flags: slab_flags_t) -> size_t {
    if (test.priv != TEST_PRIV_WANT_MEMCACHE) {
    return size;
    }
    kunit_info(test, "%s: size=%zu, ctor=%ps\n", __func__, size, ctor);
//
// Use SLAB_NO_MERGE to prevent merging with existing caches.
// Use SLAB_ACCOUNT to allocate via memcg, if enabled.
//
    flags |= SLAB_NO_MERGE | SLAB_ACCOUNT;
    test_cache = kmem_cache_create("test", size, 1, flags, ctor);
    KUNIT_ASSERT_TRUE_MSG(test, test_cache, "could not create cache");
    return size;
    }
#[no_mangle]
unsafe extern "C" fn test_cache_destroy() {
    if (!test_cache) {
    return;
    }
    kmem_cache_destroy(test_cache);
    test_cache = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn kmalloc_cache_alignment(size: usize) -> usize {
// just to get ->align so no need to pass in the real caller
pub static mut type: kmalloc_cache_type = 0;
    return kmalloc_caches[type][__kmalloc_index(size, false)].align;
    }
// Must always inline to match stack trace against caller.
#[no_mangle]
unsafe extern "C" fn test_free(ptr: *mut c_void) -> __always_inline void {
    if (test_cache) {
    kmem_cache_free(test_cache, ptr);
    }
    else {
    kfree(ptr);
    }
    }
//
// If this should be a KFENCE allocation, and on which side the allocation and
// the closest guard page should be.
//
    enum allocation_policy {
    ALLOCATE_ANY, /* KFENCE, any side. */
    ALLOCATE_LEFT, /* KFENCE, left side of page. */
    ALLOCATE_RIGHT, /* KFENCE, right side of page. */
    ALLOCATE_NONE, /* No KFENCE allocation. */
    };
//
// Try to get a guarded allocation from KFENCE. Uses either kmalloc() or the
// current test_cache if set up.
//
#[no_mangle]
pub unsafe extern "C" fn test_alloc(test: *mut kunit, size: size_t, gfp: gfp_t, policy: allocation_policy) -> *mut c_void {
pub static mut alloc: *mut c_void = core::ptr::null_mut();
    unsigned long timeout, resched_after;
pub static mut policy_name: *mut c_void = core::ptr::null_mut();
    match (policy) {
    ALLOCATE_ANY => {
    policy_name = "any";
    // break;
    }
    ALLOCATE_LEFT => {
    policy_name = "left";
    // break;
    }
    ALLOCATE_RIGHT => {
    policy_name = "right";
    // break;
    }
    ALLOCATE_NONE => {
    policy_name = "none";
    // break;
    }
    }
    kunit_info(test, "%s: size=%zu, gfp=%pGg, policy=%s, cache=%i\n", __func__, size, &gfp,
    policy_name, !!test_cache);
//
// 100x the sample interval should be more than enough to ensure we get
// a KFENCE allocation eventually.
//
    timeout = jiffies + msecs_to_jiffies(100 * kfence_sample_interval);
//
// Especially for non-preemption kernels, ensure the allocation-gate
// timer can catch up: after @resched_after, every failed allocation
// attempt yields, to ensure the allocation-gate timer is scheduled.
//
    resched_after = jiffies + msecs_to_jiffies(kfence_sample_interval);
    do {
    if (test_cache) {
    alloc = kmem_cache_alloc(test_cache, gfp);
    }
    else {
    alloc = kmalloc(size, gfp);
    }
    if (is_kfence_address(alloc)) {
    let mut slab = virt_to_slab(alloc);
pub static mut type: kmalloc_cache_type = 0;
    let mut s = test_cache ?:
    kmalloc_caches[type][__kmalloc_index(size, false)];
//
// Verify that various helpers return the right values
// even for KFENCE objects; these are required so that
// memcg accounting works correctly.
//
    KUNIT_EXPECT_EQ(test, obj_to_index(s, slab, alloc), 0U);
    KUNIT_EXPECT_EQ(test, ((unsigned int)slab.objects), 1);
    if (policy == ALLOCATE_ANY) {
    return alloc;
    }
    if (policy == ALLOCATE_LEFT && PAGE_ALIGNED(alloc)) {
    return alloc;
    }
    if (policy == ALLOCATE_RIGHT && !PAGE_ALIGNED(alloc)) {
    return alloc;
    }
    } else if (policy == ALLOCATE_NONE) {
    return alloc;
    }
    test_free(alloc);
    if (time_after(jiffies, resched_after)) {
    cond_resched();
    }
    } while (time_before(jiffies, timeout));
    KUNIT_ASSERT_TRUE_MSG(test, false, "failed to allocate from KFENCE");
    return core::ptr::null_mut(); /* Unreachable. */
    }
#[no_mangle]
unsafe extern "C" fn test_out_of_bounds_read(test: *mut kunit) {
pub static mut size: usize = 32;
pub static mut expect_report: usize = 0;
pub static mut buf: *mut c_void = core::ptr::null_mut();
    setup_test_cache(test, size, 0, core::ptr::null_mut());
//
// If we don't have our own cache, adjust based on alignment, so that we
// actually access guard pages on either side.
//
    if (!test_cache) {
    size = kmalloc_cache_alignment(size);
    }
// Test both sides.
    buf = test_alloc(test, size, GFP_KERNEL, ALLOCATE_LEFT);
    expect.addr = buf - 1;
    READ_ONCE(*expect.addr);
    KUNIT_EXPECT_TRUE(test, report_matches(&expect));
    test_free(buf);
    buf = test_alloc(test, size, GFP_KERNEL, ALLOCATE_RIGHT);
    expect.addr = buf + size;
    READ_ONCE(*expect.addr);
    KUNIT_EXPECT_TRUE(test, report_matches(&expect));
    test_free(buf);
    }
#[no_mangle]
unsafe extern "C" fn test_out_of_bounds_write(test: *mut kunit) {
pub static mut size: usize = 32;
pub static mut expect_report: usize = 0;
pub static mut buf: *mut c_void = core::ptr::null_mut();
    setup_test_cache(test, size, 0, core::ptr::null_mut());
    buf = test_alloc(test, size, GFP_KERNEL, ALLOCATE_LEFT);
    expect.addr = buf - 1;
    WRITE_ONCE(*expect.addr, 42);
    KUNIT_EXPECT_TRUE(test, report_matches(&expect));
    test_free(buf);
    }
#[no_mangle]
unsafe extern "C" fn test_use_after_free_read(test: *mut kunit) {
pub static mut size: usize = 32;
pub static mut expect_report: usize = 0;
    setup_test_cache(test, size, 0, core::ptr::null_mut());
    expect.addr = test_alloc(test, size, GFP_KERNEL, ALLOCATE_ANY);
    test_free(expect.addr);
    READ_ONCE(*expect.addr);
    KUNIT_EXPECT_TRUE(test, report_matches(&expect));
    }
#[no_mangle]
unsafe extern "C" fn test_use_after_free_read_nofault(test: *mut kunit) {
pub static mut size: usize = 32;
pub static mut addr: *mut c_void = core::ptr::null_mut();
    let mut dst = 0;
    let mut ret = 0;
    setup_test_cache(test, size, 0, core::ptr::null_mut());
    addr = test_alloc(test, size, GFP_KERNEL, ALLOCATE_ANY);
    test_free(addr);
// Use after free with *_nofault()
    ret = copy_from_kernel_nofault(&dst, addr, 1);
    KUNIT_EXPECT_EQ(test, ret, -EFAULT);
    KUNIT_EXPECT_FALSE(test, report_available());
    }
#[no_mangle]
unsafe extern "C" fn test_double_free(test: *mut kunit) {
pub static mut size: usize = 32;
pub static mut expect_report: usize = 0;
    setup_test_cache(test, size, 0, core::ptr::null_mut());
    expect.addr = test_alloc(test, size, GFP_KERNEL, ALLOCATE_ANY);
    test_free(expect.addr);
    test_free(expect.addr); /* Double-free. */
    KUNIT_EXPECT_TRUE(test, report_matches(&expect));
    }
#[no_mangle]
unsafe extern "C" fn test_invalid_addr_free(test: *mut kunit) {
pub static mut size: usize = 32;
pub static mut expect_report: usize = 0;
pub static mut buf: *mut c_void = core::ptr::null_mut();
    setup_test_cache(test, size, 0, core::ptr::null_mut());
    buf = test_alloc(test, size, GFP_KERNEL, ALLOCATE_ANY);
    expect.addr = buf + 1; /* Free on invalid address. */
    test_free(expect.addr); /* Invalid address free. */
    test_free(buf); /* No error. */
    KUNIT_EXPECT_TRUE(test, report_matches(&expect));
    }
#[no_mangle]
unsafe extern "C" fn test_corruption(test: *mut kunit) {
pub static mut size: usize = 32;
pub static mut expect_report: usize = 0;
pub static mut buf: *mut c_void = core::ptr::null_mut();
    setup_test_cache(test, size, 0, core::ptr::null_mut());
// Test both sides.
    buf = test_alloc(test, size, GFP_KERNEL, ALLOCATE_LEFT);
    expect.addr = buf + size;
    WRITE_ONCE(*expect.addr, 42);
    test_free(buf);
    KUNIT_EXPECT_TRUE(test, report_matches(&expect));
    buf = test_alloc(test, size, GFP_KERNEL, ALLOCATE_RIGHT);
    expect.addr = buf - 1;
    WRITE_ONCE(*expect.addr, 42);
    test_free(buf);
    KUNIT_EXPECT_TRUE(test, report_matches(&expect));
    }
//
// KFENCE is unable to detect an OOB if the allocation's alignment requirements
// leave a gap between the object and the guard page. Specifically, an
// allocation of e.g. 73 bytes is aligned on 8 and 128 bytes for SLUB or SLAB
// respectively. Therefore it is impossible for the allocated object to
// contiguously line up with the right guard page.
//
// However, we test that an access to memory beyond the gap results in KFENCE
// detecting an OOB access.
//
#[no_mangle]
unsafe extern "C" fn test_kmalloc_aligned_oob_read(test: *mut kunit) {
pub static mut size: usize = 73;
pub static mut align: usize = 0;
pub static mut expect_report: usize = 0;
pub static mut buf: *mut c_void = core::ptr::null_mut();
    buf = test_alloc(test, size, GFP_KERNEL, ALLOCATE_RIGHT);
//
// The object is offset to the right, so there won't be an OOB to the
// left of it.
//
    READ_ONCE(*(buf - 1));
    KUNIT_EXPECT_FALSE(test, report_available());
//
// @buf must be aligned on @align, therefore buf + size belongs to the
// same page -> no OOB.
//
    READ_ONCE(*(buf + size));
    KUNIT_EXPECT_FALSE(test, report_available());
// Overflowing by @align bytes will result in an OOB.
    expect.addr = buf + size + align;
    READ_ONCE(*expect.addr);
    KUNIT_EXPECT_TRUE(test, report_matches(&expect));
    test_free(buf);
    }
#[no_mangle]
unsafe extern "C" fn test_kmalloc_aligned_oob_write(test: *mut kunit) {
pub static mut size: usize = 73;
pub static mut expect_report: usize = 0;
pub static mut buf: *mut c_void = core::ptr::null_mut();
    buf = test_alloc(test, size, GFP_KERNEL, ALLOCATE_RIGHT);
//
// The object is offset to the right, so we won't get a page
// fault immediately after it.
//
    expect.addr = buf + size;
    WRITE_ONCE(*expect.addr, READ_ONCE(*expect.addr) + 1);
    KUNIT_EXPECT_FALSE(test, report_available());
    test_free(buf);
    KUNIT_EXPECT_TRUE(test, report_matches(&expect));
    }
// Test cache shrinking and destroying with KFENCE.
#[no_mangle]
unsafe extern "C" fn test_shrink_memcache(test: *mut kunit) {
pub static mut size: usize = 32;
pub static mut buf: *mut c_void = core::ptr::null_mut();
    setup_test_cache(test, size, 0, core::ptr::null_mut());
    KUNIT_EXPECT_TRUE(test, test_cache);
    buf = test_alloc(test, size, GFP_KERNEL, ALLOCATE_ANY);
    kmem_cache_shrink(test_cache);
    test_free(buf);
    KUNIT_EXPECT_FALSE(test, report_available());
    }
#[no_mangle]
unsafe extern "C" fn ctor_set_x(obj: *mut c_void) {
// Every object has at least 8 bytes.
    memset(obj, 'x', 8);
    }
// Ensure that SL*B does not modify KFENCE objects on bulk free.
#[no_mangle]
unsafe extern "C" fn test_free_bulk(test: *mut kunit) {
    let mut iter = 0;
    while (iter < 5) {
    const size_t size = setup_test_cache(test, get_random_u32_inclusive(8, 307),
    0, (iter & 1) ? ctor_set_x : core::ptr::null_mut());
    void *objects[] = {
    test_alloc(test, size, GFP_KERNEL, ALLOCATE_RIGHT),
    test_alloc(test, size, GFP_KERNEL, ALLOCATE_NONE),
    test_alloc(test, size, GFP_KERNEL, ALLOCATE_LEFT),
    test_alloc(test, size, GFP_KERNEL, ALLOCATE_NONE),
    test_alloc(test, size, GFP_KERNEL, ALLOCATE_NONE),
    };
    kmem_cache_free_bulk(test_cache, ARRAY_SIZE!(objects), objects);
    KUNIT_ASSERT_FALSE(test, report_available());
    test_cache_destroy();
    }
    }
// Test init-on-free works.
#[no_mangle]
unsafe extern "C" fn test_init_on_free(test: *mut kunit) {
pub static mut size: usize = 32;
pub static mut expect_report: usize = 0;
    let mut i = 0;
    KFENCE_TEST_REQUIRES(test, IS_ENABLED!(CONFIG_INIT_ON_FREE_DEFAULT_ON));
// Assume it hasn't been disabled on command line.
    setup_test_cache(test, size, 0, core::ptr::null_mut());
    expect.addr = test_alloc(test, size, GFP_KERNEL, ALLOCATE_ANY);
    for (i = 0; i < size; i++) {
    expect.addr[i] = i + 1;
    }
    test_free(expect.addr);
    while (i < size) {
//
// This may fail if the page was recycled by KFENCE and then
// written to again -- this however, is near impossible with a
// default config.
//
    KUNIT_EXPECT_EQ(test, expect.addr[i], (char)0);
    if (!i) /* Only check first access to not fail test if page is ever re-protected. */ {
    KUNIT_EXPECT_TRUE(test, report_matches(&expect));
    }
    }
    }
// Ensure that constructors work properly.
#[no_mangle]
unsafe extern "C" fn test_memcache_ctor(test: *mut kunit) {
pub static mut size: usize = 32;
pub static mut buf: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    setup_test_cache(test, size, 0, ctor_set_x);
    buf = test_alloc(test, size, GFP_KERNEL, ALLOCATE_ANY);
    for (i = 0; i < 8; i++) {
    KUNIT_EXPECT_EQ(test, buf[i], (char)'x');
    }
    test_free(buf);
    KUNIT_EXPECT_FALSE(test, report_available());
    }
// Test that memory is zeroed if requested.
#[no_mangle]
unsafe extern "C" fn test_gfpzero(test: *mut kunit) {
    const size_t size = PAGE_SIZE; /* PAGE_SIZE so we can use ALLOCATE_ANY. */
    let mut buf1 = core::ptr::null_mut();
    let mut buf2 = core::ptr::null_mut();
    let mut i = 0;
// Skip if we think it'd take too long.
    KFENCE_TEST_REQUIRES(test, kfence_sample_interval <= 100);
    setup_test_cache(test, size, 0, core::ptr::null_mut());
    buf1 = test_alloc(test, size, GFP_KERNEL, ALLOCATE_ANY);
    for (i = 0; i < size; i++) {
    buf1[i] = i + 1;
    }
    test_free(buf1);
// Try to get same address again -- this can take a while.
    for (i = 0;; i++) {
    buf2 = test_alloc(test, size, GFP_KERNEL | __GFP_ZERO, ALLOCATE_ANY);
    if (buf1 == buf2) {
    break;
    }
    test_free(buf2);
    if (kthread_should_stop() || (i == CONFIG_KFENCE_NUM_OBJECTS)) {
    kunit_warn(test, "giving up ... cannot get same object back\n");
    return;
    }
    cond_resched();
    }
    for (i = 0; i < size; i++) {
    KUNIT_EXPECT_EQ(test, buf2[i], (char)0);
    }
    test_free(buf2);
    KUNIT_EXPECT_FALSE(test, report_available());
    }
#[no_mangle]
unsafe extern "C" fn test_invalid_access(test: *mut kunit) {
pub static mut expect_report: usize = 0;
    READ_ONCE(__kfence_pool[10]);
    KUNIT_EXPECT_TRUE(test, report_matches(&expect));
    }
// Test SLAB_TYPESAFE_BY_RCU works.
#[no_mangle]
unsafe extern "C" fn test_memcache_typesafe_by_rcu(test: *mut kunit) {
pub static mut size: usize = 32;
pub static mut expect_report: usize = 0;
    setup_test_cache(test, size, SLAB_TYPESAFE_BY_RCU, core::ptr::null_mut());
    KUNIT_EXPECT_TRUE(test, test_cache); /* Want memcache. */
    expect.addr = test_alloc(test, size, GFP_KERNEL, ALLOCATE_ANY);
// expect.addr = 42;
    rcu_read_lock();
    test_free(expect.addr);
    KUNIT_EXPECT_EQ(test, *expect.addr, (char)42);
//
// Up to this point, memory should not have been freed yet, and
// therefore there should be no KFENCE report from the above access.
//
    rcu_read_unlock();
// Above access to @expect.addr should not have generated a report!
    KUNIT_EXPECT_FALSE(test, report_available());
// Only after rcu_barrier() is the memory guaranteed to be freed.
    rcu_barrier();
// Expect use-after-free.
    KUNIT_EXPECT_EQ(test, *expect.addr, (char)42);
    KUNIT_EXPECT_TRUE(test, report_matches(&expect));
    }
// Test krealloc().
#[no_mangle]
unsafe extern "C" fn test_krealloc(test: *mut kunit) {
pub static mut size: usize = 32;
pub static mut expect_report: usize = 0;
    let mut buf = expect.addr;
    let mut i = 0;
    KUNIT_EXPECT_FALSE(test, test_cache);
    KUNIT_EXPECT_EQ(test, ksize(buf), size); /* Precise size match after KFENCE alloc. */
    for (i = 0; i < size; i++) {
    buf[i] = i + 1;
    }
// Check that we successfully change the size.
    buf = krealloc(buf, size * 3, GFP_KERNEL); /* Grow. */
// Note: Might no longer be a KFENCE alloc.
    KUNIT_EXPECT_GE(test, ksize(buf), size * 3);
    for (i = 0; i < size; i++) {
    KUNIT_EXPECT_EQ(test, buf[i], (char)(i + 1));
    }
    for (; i < size * 3; i++) /* Fill to extra bytes. */ {
    buf[i] = i + 1;
    }
    buf = krealloc(buf, size * 2, GFP_KERNEL); /* Shrink. */
    KUNIT_EXPECT_GE(test, ksize(buf), size * 2);
    for (i = 0; i < size * 2; i++) {
    KUNIT_EXPECT_EQ(test, buf[i], (char)(i + 1));
    }
    buf = krealloc(buf, 0, GFP_KERNEL); /* Free. */
    KUNIT_EXPECT_EQ(test, (unsigned long)buf, (unsigned long)ZERO_SIZE_PTR);
    KUNIT_ASSERT_FALSE(test, report_available()); /* No reports yet! */
    READ_ONCE(*expect.addr); /* Ensure krealloc() actually freed earlier KFENCE object. */
    KUNIT_ASSERT_TRUE(test, report_matches(&expect));
    }
// Test that some objects from a bulk allocation belong to KFENCE pool.
#[no_mangle]
unsafe extern "C" fn test_memcache_alloc_bulk(test: *mut kunit) {
pub static mut size: usize = 32;
pub static mut pass: bool = false;
    let mut timeout = 0;
    setup_test_cache(test, size, 0, core::ptr::null_mut());
    KUNIT_EXPECT_TRUE(test, test_cache); /* Want memcache. */
//
// 100x the sample interval should be more than enough to ensure we get
// a KFENCE allocation eventually.
//
    timeout = jiffies + msecs_to_jiffies(100 * kfence_sample_interval);
    do {
    void *objects[100];
    let mut i = 0;
    if (!kmem_cache_alloc_bulk(test_cache, GFP_ATOMIC,
    ARRAY_SIZE!(objects), objects)) {
    continue;
    }
    while (i < ARRAY_SIZE!(objects)) {
    if (is_kfence_address(objects[i])) {
    pass = true;
    break;
    }
    }
    kmem_cache_free_bulk(test_cache, ARRAY_SIZE!(objects), objects);
//
// kmem_cache_alloc_bulk() disables interrupts, and calling it
// in a tight loop may not give KFENCE a chance to switch the
// static branch. Call cond_resched() to let KFENCE chime in.
//
    cond_resched();
    } while (!pass && time_before(jiffies, timeout));
    KUNIT_EXPECT_TRUE(test, pass);
    KUNIT_EXPECT_FALSE(test, report_available());
    }
//
// KUnit does not provide a way to provide arguments to tests, and we encode
// additional info in the name. Set up 2 tests per test case, one using the
// default allocator, and another using a custom memcache (suffix '-memcache').
//

    { .run_case = test_name, .name = #test_name },				
    { .run_case = test_name, .name = #test_name "-memcache" }
pub static mut kunit_case: usize = 0;
// ===== End test cases =====
#[no_mangle]
unsafe extern "C" fn test_init(test: *mut kunit) -> c_int {
    let mut flags = 0;
    let mut i = 0;
    if (!__kfence_pool) {
    return -EINVAL;
    }
    spin_lock_irqsave(&observed.lock, flags);
    for (i = 0; i < ARRAY_SIZE!(observed.lines); i++) {
    observed.lines[i][0] = '\0';
    }
    observed.nlines = 0;
    spin_unlock_irqrestore(&observed.lock, flags);
// Any test with 'memcache' in its name will want a memcache.
    if (strstr(test.name, "memcache")) {
    test.priv = TEST_PRIV_WANT_MEMCACHE;
    }
    else {
    test.priv = core::ptr::null_mut();
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_exit(test: *mut kunit) {
    test_cache_destroy();
    }
#[no_mangle]
unsafe extern "C" fn kfence_suite_init(suite: *mut kunit_suite) -> c_int {
    register_trace_console(probe_console, core::ptr::null_mut());
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn kfence_suite_exit(suite: *mut kunit_suite) {
    unregister_trace_console(probe_console, core::ptr::null_mut());
    tracepoint_synchronize_unregister();
    }
pub static mut kunit_suite: usize = 0;
    kunit_test_suites(&kfence_test_suite);
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Alexander Potapenko <glider@google.com>, Marco Elver <elver@google.com>");
    MODULE_DESCRIPTION("kfence unit test suite");