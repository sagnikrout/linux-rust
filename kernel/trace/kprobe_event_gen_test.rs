//! Automatically rewritten from C to Rust
//! Source: kernel/trace/kprobe_event_gen_test.c
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
// Test module for in-kernel kprobe event creation and generation.
//
// Copyright (C) 2019 Tom Zanussi <zanussi@kernel.org>
//

//
// This module is a simple test of basic functionality for in-kernel
// kprobe/kretprobe event creation.  The first test uses
// kprobe_event_gen_cmd_start(), kprobe_event_add_fields() and
// kprobe_event_gen_cmd_end() to create a kprobe event, which is then
// enabled in order to generate trace output.  The second creates a
// kretprobe event using kretprobe_event_gen_cmd_start() and
// kretprobe_event_gen_cmd_end(), and is also then enabled.
//
// To test, select CONFIG_KPROBE_EVENT_GEN_TEST and build the module.
// Then:
//
// # insmod kernel/trace/kprobe_event_gen_test.ko
// # cat /sys/kernel/tracing/trace
//
// You should see many instances of the "gen_kprobe_test" and
// "gen_kretprobe_test" events in the trace buffer.
//
// To remove the events, remove the module:
//
// # rmmod kprobe_event_gen_test
//
pub static mut gen_kprobe_test: *mut c_void = core::ptr::null_mut();
pub static mut gen_kretprobe_test: *mut c_void = core::ptr::null_mut();

// X86

// ARM64

// ARM

// RISCV

// others

#[no_mangle]
unsafe extern "C" fn trace_event_file_is_valid(input: *mut trace_event_file) -> bool {
    return input && !IS_ERR(input);
    }
//
// Test to make sure we can create a kprobe event, then add more
// fields.
//
#[no_mangle]
unsafe extern "C" fn test_gen_kprobe_cmd() -> c_int {
pub static mut cmd: usize = 0;
pub static mut buf: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
// Create a buffer to hold the generated command
    buf = kzalloc(MAX_DYNEVENT_CMD_LEN, GFP_KERNEL);
    if (!buf) {
    return -ENOMEM;
    }
// Before generating the command, initialize the cmd object
    kprobe_event_cmd_init(&cmd, buf, MAX_DYNEVENT_CMD_LEN);
//
// Define the gen_kprobe_test event with the first 2 kprobe
// fields.
//
    ret = kprobe_event_gen_cmd_start(&cmd, "gen_kprobe_test",
    KPROBE_GEN_TEST_FUNC,
    KPROBE_GEN_TEST_ARG0, KPROBE_GEN_TEST_ARG1);
    if (ret) {
// goto;
    }
// Use kprobe_event_add_fields to add the rest of the fields
    ret = kprobe_event_add_fields(&cmd, KPROBE_GEN_TEST_ARG2, KPROBE_GEN_TEST_ARG3);
    if (ret) {
// goto;
    }
//
// This actually creates the event.
//
    ret = kprobe_event_gen_cmd_end(&cmd);
    if (ret) {
// goto;
    }
//
// Now get the gen_kprobe_test event file.  We need to prevent
// the instance and event from disappearing from underneath
// us, which trace_get_event_file() does (though in this case
// we're using the top-level instance which never goes away).
//
    gen_kprobe_test = trace_get_event_file(core::ptr::null_mut(), "kprobes",
    "gen_kprobe_test");
    if (IS_ERR(gen_kprobe_test)) {
    ret = PTR_ERR(gen_kprobe_test);
// goto;
    }
// Enable the event or you won't see anything
    ret = trace_array_set_clr_event(gen_kprobe_test.tr,
    "kprobes", "gen_kprobe_test", true);
    if (ret) {
    trace_put_event_file(gen_kprobe_test);
// goto;
    }
// label;
    kfree(buf);
    return ret;
// label;
    if (trace_event_file_is_valid(gen_kprobe_test)) {
    gen_kprobe_test = core::ptr::null_mut();
    }
// We got an error after creating the event, delete it
    kprobe_event_delete("gen_kprobe_test");
// goto;
    }
//
// Test to make sure we can create a kretprobe event.
//
#[no_mangle]
unsafe extern "C" fn test_gen_kretprobe_cmd() -> c_int {
pub static mut cmd: usize = 0;
pub static mut buf: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
// Create a buffer to hold the generated command
    buf = kzalloc(MAX_DYNEVENT_CMD_LEN, GFP_KERNEL);
    if (!buf) {
    return -ENOMEM;
    }
// Before generating the command, initialize the cmd object
    kprobe_event_cmd_init(&cmd, buf, MAX_DYNEVENT_CMD_LEN);
//
// Define the kretprobe event.
//
    ret = kretprobe_event_gen_cmd_start(&cmd, "gen_kretprobe_test",
    KPROBE_GEN_TEST_FUNC,
    "$retval");
    if (ret) {
// goto;
    }
//
// This actually creates the event.
//
    ret = kretprobe_event_gen_cmd_end(&cmd);
    if (ret) {
// goto;
    }
//
// Now get the gen_kretprobe_test event file.  We need to
// prevent the instance and event from disappearing from
// underneath us, which trace_get_event_file() does (though in
// this case we're using the top-level instance which never
// goes away).
//
    gen_kretprobe_test = trace_get_event_file(core::ptr::null_mut(), "kprobes",
    "gen_kretprobe_test");
    if (IS_ERR(gen_kretprobe_test)) {
    ret = PTR_ERR(gen_kretprobe_test);
// goto;
    }
// Enable the event or you won't see anything
    ret = trace_array_set_clr_event(gen_kretprobe_test.tr,
    "kprobes", "gen_kretprobe_test", true);
    if (ret) {
    trace_put_event_file(gen_kretprobe_test);
// goto;
    }
// label;
    kfree(buf);
    return ret;
// label;
    if (trace_event_file_is_valid(gen_kretprobe_test)) {
    gen_kretprobe_test = core::ptr::null_mut();
    }
// We got an error after creating the event, delete it
    kprobe_event_delete("gen_kretprobe_test");
// goto;
    }
#[no_mangle]
unsafe extern "C" fn kprobe_event_gen_test_init() -> c_int {
    let mut ret = 0;
    ret = test_gen_kprobe_cmd();
    if (ret) {
    return ret;
    }
    ret = test_gen_kretprobe_cmd();
    if (ret) {
    if (trace_event_file_is_valid(gen_kretprobe_test)) {
    WARN_ON!(trace_array_set_clr_event(gen_kretprobe_test.tr,
    "kprobes",
    "gen_kretprobe_test", false));
    trace_put_event_file(gen_kretprobe_test);
    }
    WARN_ON!(kprobe_event_delete("gen_kretprobe_test"));
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn kprobe_event_gen_test_exit()  {
    if (trace_event_file_is_valid(gen_kprobe_test)) {
// Disable the event or you can't remove it
    WARN_ON!(trace_array_set_clr_event(gen_kprobe_test.tr,
    "kprobes",
    "gen_kprobe_test", false));
// Now give the file and instance back
    trace_put_event_file(gen_kprobe_test);
    }
// Now unregister and free the event
    WARN_ON!(kprobe_event_delete("gen_kprobe_test"));
    if (trace_event_file_is_valid(gen_kretprobe_test)) {
// Disable the event or you can't remove it
    WARN_ON!(trace_array_set_clr_event(gen_kretprobe_test.tr,
    "kprobes",
    "gen_kretprobe_test", false));
// Now give the file and instance back
    trace_put_event_file(gen_kretprobe_test);
    }
// Now unregister and free the event
    WARN_ON!(kprobe_event_delete("gen_kretprobe_test"));
    }
    module_init!(kprobe_event_gen_test_init)
    module_exit!(kprobe_event_gen_test_exit)
    MODULE_AUTHOR("Tom Zanussi");
    MODULE_DESCRIPTION("kprobe event generation test");
    MODULE_LICENSE("GPL v2");