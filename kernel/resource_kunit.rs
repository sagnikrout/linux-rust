//! Automatically rewritten from C to Rust
//! Source: kernel/resource_kunit.c
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
// Test cases for API provided by resource.c and ioport.h
//

pub const R0_START: c_uint = 0x0000;
pub const R0_END: c_uint = 0xffff;
pub const R1_START: c_uint = 0x1234;
pub const R1_END: c_uint = 0x2345;
pub const R2_START: c_uint = 0x4567;
pub const R2_END: c_uint = 0x5678;
pub const R3_START: c_uint = 0x6789;
pub const R3_END: c_uint = 0x789a;
pub const R4_START: c_uint = 0x2000;
pub const R4_END: c_uint = 0x7000;
pub static mut r0: resource = 0;
pub static mut r1: resource = 0;
pub static mut r2: resource = 0;
pub static mut r3: resource = 0;
pub static mut r4: resource = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct result {
    pub r1: *mut resource,
    pub r2: *mut resource,
    pub r: resource,
    pub ret: bool,
}

pub static mut result: usize = 0;
pub static mut result: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn resource_do_test(test: *mut kunit, ret: bool, r: *mut resource, exp_ret: bool, exp_r: *mut resource, r1: *mut resource, r2: *mut resource) {
    KUNIT_EXPECT_EQ_MSG(test, ret, exp_ret, "Resources %pR %pR", r1, r2);
    KUNIT_EXPECT_EQ_MSG(test, r.start, exp_r.start, "Start elements are not equal");
    KUNIT_EXPECT_EQ_MSG(test, r.end, exp_r.end, "End elements are not equal");
    }
#[no_mangle]
unsafe extern "C" fn resource_do_union_test(test: *mut kunit, r: *mut result) {
pub static mut result: usize = 0;
    let mut ret = 0;
    memset(&result, 0, sizeof!(result));
    ret = resource_union(r.r1, r.r2, &result);
    resource_do_test(test, ret, &result, r.ret, &r.r, r.r1, r.r2);
    memset(&result, 0, sizeof!(result));
    ret = resource_union(r.r2, r.r1, &result);
    resource_do_test(test, ret, &result, r.ret, &r.r, r.r2, r.r1);
    }
#[no_mangle]
unsafe extern "C" fn resource_test_union(test: *mut kunit) {
    let mut r = results_for_union;
pub static mut i: c_uint = 0;
    do {
    resource_do_union_test(test, &r[i]);
    } while (++i < ARRAY_SIZE!(results_for_union));
    }
#[no_mangle]
unsafe extern "C" fn resource_do_intersection_test(test: *mut kunit, r: *mut result) {
pub static mut result: usize = 0;
    let mut ret = 0;
    memset(&result, 0, sizeof!(result));
    ret = resource_intersection(r.r1, r.r2, &result);
    resource_do_test(test, ret, &result, r.ret, &r.r, r.r1, r.r2);
    memset(&result, 0, sizeof!(result));
    ret = resource_intersection(r.r2, r.r1, &result);
    resource_do_test(test, ret, &result, r.ret, &r.r, r.r2, r.r1);
    }
#[no_mangle]
unsafe extern "C" fn resource_test_intersection(test: *mut kunit) {
    let mut r = results_for_intersection;
pub static mut i: c_uint = 0;
    do {
    resource_do_intersection_test(test, &r[i]);
    } while (++i < ARRAY_SIZE!(results_for_intersection));
    }
//
// The test resource tree for region_intersects() test:
//
// BASE-BASE+1M-1 : Test System RAM 0
// # hole 0 (BASE+1M-BASE+2M)
// BASE+2M-BASE+3M-1 : Test CXL Window 0
// BASE+3M-BASE+4M-1 : Test System RAM 1
// BASE+4M-BASE+7M-1 : Test CXL Window 1
// BASE+4M-BASE+5M-1 : Test System RAM 2
// BASE+4M+128K-BASE+4M+256K-1: Test Code
// BASE+5M-BASE+6M-1 : Test System RAM 3
//
pub const RES_TEST_RAM0_OFFSET: c_int = 0;

    KUNIT_DEFINE_ACTION_WRAPPER(kfree_wrapper, kfree, const void *);
#[no_mangle]
unsafe extern "C" fn remove_free_resource(ctx: *mut c_void) {
    let mut res = ctx;
    remove_resource(res);
    kfree(res);
    }
#[no_mangle]
pub unsafe extern "C" fn resource_test_add_action_or_abort(test: *mut kunit, ctx: *mut c_void) {
    KUNIT_ASSERT_EQ_MSG(test, 0,
    kunit_add_action_or_reset(test, action, ctx),
    "Fail to add action");
    }
#[no_mangle]
pub unsafe extern "C" fn resource_test_request_region(test: *mut kunit, parent: *mut resource, start: resource_size_t, size: resource_size_t, name: *mut c_char, flags: c_ulong) {
pub static mut res: *mut c_void = core::ptr::null_mut();
    res = __request_region(parent, start, size, name, flags);
    KUNIT_ASSERT_NOT_NULL(test, res);
    resource_test_add_action_or_abort(test, remove_free_resource, res);
    }
#[no_mangle]
pub unsafe extern "C" fn resource_test_insert_resource(test: *mut kunit, parent: *mut resource, start: resource_size_t, size: resource_size_t, name: *mut c_char, flags: c_ulong) {
pub static mut res: *mut c_void = core::ptr::null_mut();
    res = kzalloc_obj(*res);
    KUNIT_ASSERT_NOT_NULL(test, res);
    res.name = name;
    res.start = start;
    res.end = start + size - 1;
    res.flags = flags;
    if (insert_resource(parent, res)) {
    resource_test_add_action_or_abort(test, kfree_wrapper, res);
    KUNIT_FAIL_AND_ABORT(test, "Fail to insert resource %pR\n", res);
    }
    resource_test_add_action_or_abort(test, remove_free_resource, res);
    }
#[no_mangle]
unsafe extern "C" fn resource_test_region_intersects(test: *mut kunit) {
pub static mut flags: c_ulong = 0;
pub static mut parent: *mut c_void = core::ptr::null_mut();
    let mut start;
// Find an iomem_resource hole to hold test resources
    parent = alloc_free_mem_region(&iomem_resource, RES_TEST_TOTAL_SIZE, SZ_1M,
    "test resources");
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, parent);
    start = parent.start;
    resource_test_add_action_or_abort(test, remove_free_resource, parent);
    resource_test_request_region(test, parent, start + RES_TEST_RAM0_OFFSET,
    RES_TEST_RAM0_SIZE, "Test System RAM 0", flags);
    resource_test_insert_resource(test, parent, start + RES_TEST_WIN0_OFFSET,
    RES_TEST_WIN0_SIZE, "Test CXL Window 0",
    IORESOURCE_MEM);
    resource_test_request_region(test, parent, start + RES_TEST_RAM1_OFFSET,
    RES_TEST_RAM1_SIZE, "Test System RAM 1", flags);
    resource_test_insert_resource(test, parent, start + RES_TEST_WIN1_OFFSET,
    RES_TEST_WIN1_SIZE, "Test CXL Window 1",
    IORESOURCE_MEM);
    resource_test_request_region(test, parent, start + RES_TEST_RAM2_OFFSET,
    RES_TEST_RAM2_SIZE, "Test System RAM 2", flags);
    resource_test_insert_resource(test, parent, start + RES_TEST_CODE_OFFSET,
    RES_TEST_CODE_SIZE, "Test Code", flags);
    resource_test_request_region(test, parent, start + RES_TEST_RAM3_OFFSET,
    RES_TEST_RAM3_SIZE, "Test System RAM 3", flags);
    kunit_release_action(test, remove_free_resource, parent);
    KUNIT_EXPECT_EQ(test, REGION_INTERSECTS,
    region_intersects(start + RES_TEST_RAM0_OFFSET, PAGE_SIZE,
    IORESOURCE_SYSTEM_RAM, IORES_DESC_NONE));
    KUNIT_EXPECT_EQ(test, REGION_INTERSECTS,
    region_intersects(start + RES_TEST_RAM0_OFFSET +
    RES_TEST_RAM0_SIZE - PAGE_SIZE, 2 * PAGE_SIZE,
    IORESOURCE_SYSTEM_RAM, IORES_DESC_NONE));
    KUNIT_EXPECT_EQ(test, REGION_DISJOINT,
    region_intersects(start + RES_TEST_HOLE0_OFFSET, PAGE_SIZE,
    IORESOURCE_SYSTEM_RAM, IORES_DESC_NONE));
    KUNIT_EXPECT_EQ(test, REGION_DISJOINT,
    region_intersects(start + RES_TEST_HOLE0_OFFSET +
    RES_TEST_HOLE0_SIZE - PAGE_SIZE, 2 * PAGE_SIZE,
    IORESOURCE_SYSTEM_RAM, IORES_DESC_NONE));
    KUNIT_EXPECT_EQ(test, REGION_MIXED,
    region_intersects(start + RES_TEST_WIN0_OFFSET +
    RES_TEST_WIN0_SIZE - PAGE_SIZE, 2 * PAGE_SIZE,
    IORESOURCE_SYSTEM_RAM, IORES_DESC_NONE));
    KUNIT_EXPECT_EQ(test, REGION_INTERSECTS,
    region_intersects(start + RES_TEST_RAM1_OFFSET +
    RES_TEST_RAM1_SIZE - PAGE_SIZE, 2 * PAGE_SIZE,
    IORESOURCE_SYSTEM_RAM, IORES_DESC_NONE));
    KUNIT_EXPECT_EQ(test, REGION_INTERSECTS,
    region_intersects(start + RES_TEST_RAM2_OFFSET +
    RES_TEST_RAM2_SIZE - PAGE_SIZE, 2 * PAGE_SIZE,
    IORESOURCE_SYSTEM_RAM, IORES_DESC_NONE));
    KUNIT_EXPECT_EQ(test, REGION_INTERSECTS,
    region_intersects(start + RES_TEST_CODE_OFFSET, PAGE_SIZE,
    IORESOURCE_SYSTEM_RAM, IORES_DESC_NONE));
    KUNIT_EXPECT_EQ(test, REGION_INTERSECTS,
    region_intersects(start + RES_TEST_RAM2_OFFSET,
    RES_TEST_RAM2_SIZE + PAGE_SIZE,
    IORESOURCE_SYSTEM_RAM, IORES_DESC_NONE));
    KUNIT_EXPECT_EQ(test, REGION_MIXED,
    region_intersects(start + RES_TEST_RAM3_OFFSET,
    RES_TEST_RAM3_SIZE + PAGE_SIZE,
    IORESOURCE_SYSTEM_RAM, IORES_DESC_NONE));
    }
pub static mut kunit_case: usize = 0;
pub static mut kunit_suite: usize = 0;
    kunit_test_suite(resource_test_suite);
    MODULE_DESCRIPTION("I/O Port & Memory Resource manager unit tests");
    MODULE_LICENSE("GPL");