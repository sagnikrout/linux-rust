//! Automatically rewritten from C to Rust
//! Source: kernel/sysctl-test.c
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



// SPDX-License-Identifier: GPL-2.0
//
// KUnit test of proc sysctl.
//

pub const KUNIT_PROC_READ: c_int = 0;
pub const KUNIT_PROC_WRITE: c_int = 1;
//
// Test that proc_dointvec will not try to use a NULL .data field even when the
// length is non-zero.
//
#[no_mangle]
unsafe extern "C" fn sysctl_test_api_dointvec_null_tbl_data(test: *mut kunit) {
pub static mut ctl_table: usize = 0;
//
// proc_dointvec expects a buffer in user space, so we allocate one. We
// also need to cast it to  so sparse doesn't get mad.
//
    let mut buffer = kunit_kzalloc(test, sizeof!(int),
    GFP_USER);
    let mut len = 0;
    let mut pos = 0;
//
// We don't care what the starting length is since proc_dointvec should
// not try to read because .data is NULL.
//
    len = 1234;
    KUNIT_EXPECT_EQ(test, 0, proc_dointvec(&null_data_table,
    KUNIT_PROC_READ, buffer, &len,
    &pos));
    KUNIT_EXPECT_EQ(test, 0, len);
//
// See above.
//
    len = 1234;
    KUNIT_EXPECT_EQ(test, 0, proc_dointvec(&null_data_table,
    KUNIT_PROC_WRITE, buffer, &len,
    &pos));
    KUNIT_EXPECT_EQ(test, 0, len);
    }
//
// Similar to the previous test, we create a struct ctrl_table that has a .data
// field that proc_dointvec cannot do anything with; however, this time it is
// because we tell proc_dointvec that the size is 0.
//
#[no_mangle]
unsafe extern "C" fn sysctl_test_api_dointvec_table_maxlen_unset(test: *mut kunit) {
pub static mut data: c_int = 0;
pub static mut ctl_table: usize = 0;
    let mut buffer = kunit_kzalloc(test, sizeof!(int),
    GFP_USER);
    let mut len = 0;
    let mut pos = 0;
//
// As before, we don't care what buffer length is because proc_dointvec
// cannot do anything because its internal .data buffer has zero length.
//
    len = 1234;
    KUNIT_EXPECT_EQ(test, 0, proc_dointvec(&data_maxlen_unset_table,
    KUNIT_PROC_READ, buffer, &len,
    &pos));
    KUNIT_EXPECT_EQ(test, 0, len);
//
// See previous comment.
//
    len = 1234;
    KUNIT_EXPECT_EQ(test, 0, proc_dointvec(&data_maxlen_unset_table,
    KUNIT_PROC_WRITE, buffer, &len,
    &pos));
    KUNIT_EXPECT_EQ(test, 0, len);
    }
//
// Here we provide a valid struct ctl_table, but we try to read and write from
// it using a buffer of zero length, so it should still fail in a similar way as
// before.
//
#[no_mangle]
unsafe extern "C" fn sysctl_test_api_dointvec_table_len_is_zero(test: *mut kunit) {
pub static mut data: c_int = 0;
// Good table.
pub static mut ctl_table: usize = 0;
    let mut buffer = kunit_kzalloc(test, sizeof!(int),
    GFP_USER);
//
// However, now our read/write buffer has zero length.
//
pub static mut len: usize = 0;
    let mut pos = 0;
    KUNIT_EXPECT_EQ(test, 0, proc_dointvec(&table, KUNIT_PROC_READ, buffer,
    &len, &pos));
    KUNIT_EXPECT_EQ(test, 0, len);
    KUNIT_EXPECT_EQ(test, 0, proc_dointvec(&table, KUNIT_PROC_WRITE, buffer,
    &len, &pos));
    KUNIT_EXPECT_EQ(test, 0, len);
    }
//
// Test that proc_dointvec refuses to read when the file position is non-zero.
//
#[no_mangle]
pub unsafe extern "C" fn sysctl_test_api_dointvec_table_read_but_position_set(test: *mut kunit) {
pub static mut data: c_int = 0;
// Good table.
pub static mut ctl_table: usize = 0;
    let mut buffer = kunit_kzalloc(test, sizeof!(int),
    GFP_USER);
//
// We don't care about our buffer length because we start off with a
// non-zero file position.
//
pub static mut len: usize = 1234;
//
// proc_dointvec should refuse to read into the buffer since the file
// pos is non-zero.
//
pub static mut pos: loff_t = 1;
    KUNIT_EXPECT_EQ(test, 0, proc_dointvec(&table, KUNIT_PROC_READ, buffer,
    &len, &pos));
    KUNIT_EXPECT_EQ(test, 0, len);
    }
//
// Test that we can read a two digit number in a sufficiently size buffer.
// Nothing fancy.
//
#[no_mangle]
unsafe extern "C" fn sysctl_test_dointvec_read_happy_single_positive(test: *mut kunit) {
pub static mut data: c_int = 0;
// Good table.
pub static mut ctl_table: usize = 0;
pub static mut len: usize = 4;
pub static mut pos: loff_t = 0;
    let mut buffer = kunit_kzalloc(test, len, GFP_USER);
    let mut user_buffer = buffer;
// Store 13 in the data field.
// (table.data) = 13;
    KUNIT_EXPECT_EQ(test, 0, proc_dointvec(&table, KUNIT_PROC_READ,
    user_buffer, &len, &pos));
    KUNIT_ASSERT_EQ(test, 3, len);
    buffer[len] = '\0';
// And we read 13 back out.
    KUNIT_EXPECT_STREQ(test, "13\n", buffer);
    }
//
// Same as previous test, just now with negative numbers.
//
#[no_mangle]
unsafe extern "C" fn sysctl_test_dointvec_read_happy_single_negative(test: *mut kunit) {
pub static mut data: c_int = 0;
// Good table.
pub static mut ctl_table: usize = 0;
pub static mut len: usize = 5;
pub static mut pos: loff_t = 0;
    let mut buffer = kunit_kzalloc(test, len, GFP_USER);
    let mut user_buffer = buffer;
// (table.data) = -16;
    KUNIT_EXPECT_EQ(test, 0, proc_dointvec(&table, KUNIT_PROC_READ,
    user_buffer, &len, &pos));
    KUNIT_ASSERT_EQ(test, 4, len);
    buffer[len] = '\0';
    KUNIT_EXPECT_STREQ(test, "-16\n", buffer);
    }
//
// Test that a simple positive write works.
//
#[no_mangle]
unsafe extern "C" fn sysctl_test_dointvec_write_happy_single_positive(test: *mut kunit) {
pub static mut data: c_int = 0;
// Good table.
pub static mut ctl_table: usize = 0;
    char input[] = "9";
pub static mut len: usize = 0;
pub static mut pos: loff_t = 0;
    let mut buffer = kunit_kzalloc(test, len, GFP_USER);
    let mut user_buffer = buffer;
    memcpy(buffer, input, len);
    KUNIT_EXPECT_EQ(test, 0, proc_dointvec(&table, KUNIT_PROC_WRITE,
    user_buffer, &len, &pos));
    KUNIT_EXPECT_EQ(test, sizeof!(input) - 1, len);
    KUNIT_EXPECT_EQ(test, sizeof!(input) - 1, pos);
    KUNIT_EXPECT_EQ(test, 9, *(table.data));
    }
//
// Same as previous test, but now with negative numbers.
//
#[no_mangle]
unsafe extern "C" fn sysctl_test_dointvec_write_happy_single_negative(test: *mut kunit) {
pub static mut data: c_int = 0;
pub static mut ctl_table: usize = 0;
    char input[] = "-9";
pub static mut len: usize = 0;
pub static mut pos: loff_t = 0;
    let mut buffer = kunit_kzalloc(test, len, GFP_USER);
    let mut user_buffer = buffer;
    memcpy(buffer, input, len);
    KUNIT_EXPECT_EQ(test, 0, proc_dointvec(&table, KUNIT_PROC_WRITE,
    user_buffer, &len, &pos));
    KUNIT_EXPECT_EQ(test, sizeof!(input) - 1, len);
    KUNIT_EXPECT_EQ(test, sizeof!(input) - 1, pos);
    KUNIT_EXPECT_EQ(test, -9, *(table.data));
    }
//
// Test that writing a value smaller than the minimum possible value is not
// allowed.
//
#[no_mangle]
pub unsafe extern "C" fn sysctl_test_api_dointvec_write_single_less_int_min(test: *mut kunit) {
pub static mut data: c_int = 0;
pub static mut ctl_table: usize = 0;
pub static mut max_len: usize = 0;
pub static mut pos: loff_t = 0;
    let mut buffer = kunit_kzalloc(test, max_len, GFP_USER);
    let mut user_buffer = buffer;
    let mut abs_of_less_than_min = (unsigned long)INT_MAX
    - (INT_MAX + INT_MIN) + 1;
//
// We use this rigmarole to create a string that contains a value one
// less than the minimum accepted value.
//
    KUNIT_ASSERT_LT(test,
    (size_t)snprintf(buffer, max_len, "-%lu",
    abs_of_less_than_min),
    max_len);
    KUNIT_EXPECT_EQ(test, -EINVAL, proc_dointvec(&table, KUNIT_PROC_WRITE,
    user_buffer, &len, &pos));
    KUNIT_EXPECT_EQ(test, max_len, len);
    KUNIT_EXPECT_EQ(test, 0, *(table.data));
    }
//
// Test that writing the maximum possible value works.
//
#[no_mangle]
pub unsafe extern "C" fn sysctl_test_api_dointvec_write_single_greater_int_max(test: *mut kunit) {
pub static mut data: c_int = 0;
pub static mut ctl_table: usize = 0;
pub static mut max_len: usize = 0;
pub static mut pos: loff_t = 0;
    let mut buffer = kunit_kzalloc(test, max_len, GFP_USER);
    let mut user_buffer = buffer;
pub static mut greater_than_max: c_ulong = 0;
    KUNIT_ASSERT_GT(test, greater_than_max, (unsigned long)INT_MAX);
    KUNIT_ASSERT_LT(test, (size_t)snprintf(buffer, max_len, "%lu",
    greater_than_max),
    max_len);
    KUNIT_EXPECT_EQ(test, -EINVAL, proc_dointvec(&table, KUNIT_PROC_WRITE,
    user_buffer, &len, &pos));
    KUNIT_ASSERT_EQ(test, max_len, len);
    KUNIT_EXPECT_EQ(test, 0, *(table.data));
    }
pub static mut kunit_case: usize = 0;
pub static mut kunit_suite: usize = 0;
    kunit_test_suites(&sysctl_test_suite);
    MODULE_DESCRIPTION("KUnit test of proc sysctl");
    MODULE_LICENSE("GPL v2");