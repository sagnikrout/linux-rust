//! Automatically rewritten from C to Rust
//! Source: kernel/trace/synth_event_gen_test.c
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
// Test module for in-kernel synthetic event creation and generation.
//
// Copyright (C) 2019 Tom Zanussi <zanussi@kernel.org>
//

//
// This module is a simple test of basic functionality for in-kernel
// synthetic event creation and generation, the first and second tests
// using synth_event_gen_cmd_start() and synth_event_add_field(), the
// third uses synth_event_create() to do it all at once with a static
// field array.
//
// Following that are a few examples using the created events to test
// various ways of tracing a synthetic event.
//
// To test, select CONFIG_SYNTH_EVENT_GEN_TEST and build the module.
// Then:
//
// # insmod kernel/trace/synth_event_gen_test.ko
// # cat /sys/kernel/tracing/trace
//
// You should see several events in the trace buffer -
// "create_synth_test", "empty_synth_test", and several instances of
// "gen_synth_test".
//
// To remove the events, remove the module:
//
// # rmmod synth_event_gen_test
//
pub static mut create_synth_test: *mut c_void = core::ptr::null_mut();
pub static mut empty_synth_test: *mut c_void = core::ptr::null_mut();
pub static mut gen_synth_test: *mut c_void = core::ptr::null_mut();
//
// Test to make sure we can create a synthetic event, then add more
// fields.
//
#[no_mangle]
unsafe extern "C" fn test_gen_synth_cmd() -> c_int {
pub static mut cmd: usize = 0;
    u64 vals[7];
pub static mut buf: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
// Create a buffer to hold the generated command
    buf = kzalloc(MAX_DYNEVENT_CMD_LEN, GFP_KERNEL);
    if (!buf) {
    return -ENOMEM;
    }
// Before generating the command, initialize the cmd object
    synth_event_cmd_init(&cmd, buf, MAX_DYNEVENT_CMD_LEN);
//
// Create the empty gen_synth_test synthetic event with the
// first 4 fields.
//
    ret = synth_event_gen_cmd_start(&cmd, "gen_synth_test", THIS_MODULE,
    "pid_t", "next_pid_field",
    "char[16]", "next_comm_field",
    "u64", "ts_ns",
    "u64", "ts_ms");
    if (ret) {
// goto;
    }
// Use synth_event_add_field to add the rest of the fields
    ret = synth_event_add_field(&cmd, "unsigned int", "cpu");
    if (ret) {
// goto;
    }
    ret = synth_event_add_field(&cmd, "char[64]", "my_string_field");
    if (ret) {
// goto;
    }
    ret = synth_event_add_field(&cmd, "int", "my_int_field");
    if (ret) {
// goto;
    }
    ret = synth_event_gen_cmd_end(&cmd);
    if (ret) {
// goto;
    }
//
// Now get the gen_synth_test event file.  We need to prevent
// the instance and event from disappearing from underneath
// us, which trace_get_event_file() does (though in this case
// we're using the top-level instance which never goes away).
//
    gen_synth_test = trace_get_event_file(core::ptr::null_mut(), "synthetic",
    "gen_synth_test");
    if (IS_ERR(gen_synth_test)) {
    ret = PTR_ERR(gen_synth_test);
// goto;
    }
// Enable the event or you won't see anything
    ret = trace_array_set_clr_event(gen_synth_test.tr,
    "synthetic", "gen_synth_test", true);
    if (ret) {
    trace_put_event_file(gen_synth_test);
// goto;
    }
// Create some bogus values just for testing
    vals[0] = 777;			/* next_pid_field */
    vals[1] = (u64)(long)"hula hoops";	/* next_comm_field */
    vals[2] = 1000000;		/* ts_ns */
    vals[3] = 1000;			/* ts_ms */
    vals[4] = raw_smp_processor_id(); /* cpu */
    vals[5] = (u64)(long)"thneed";	/* my_string_field */
    vals[6] = 598;			/* my_int_field */
// Now generate a gen_synth_test event
    ret = synth_event_trace_array(gen_synth_test, vals, ARRAY_SIZE!(vals));
// label;
    kfree(buf);
    return ret;
// label;
// We got an error after creating the event, delete it
    synth_event_delete("gen_synth_test");
// goto;
    }
//
// Test to make sure we can create an initially empty synthetic event,
// then add all the fields.
//
#[no_mangle]
unsafe extern "C" fn test_empty_synth_event() -> c_int {
pub static mut cmd: usize = 0;
    u64 vals[7];
pub static mut buf: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
// Create a buffer to hold the generated command
    buf = kzalloc(MAX_DYNEVENT_CMD_LEN, GFP_KERNEL);
    if (!buf) {
    return -ENOMEM;
    }
// Before generating the command, initialize the cmd object
    synth_event_cmd_init(&cmd, buf, MAX_DYNEVENT_CMD_LEN);
//
// Create the empty_synth_test synthetic event with no fields.
//
    ret = synth_event_gen_cmd_start(&cmd, "empty_synth_test", THIS_MODULE);
    if (ret) {
// goto;
    }
// Use synth_event_add_field to add all of the fields
    ret = synth_event_add_field(&cmd, "pid_t", "next_pid_field");
    if (ret) {
// goto;
    }
    ret = synth_event_add_field(&cmd, "char[16]", "next_comm_field");
    if (ret) {
// goto;
    }
    ret = synth_event_add_field(&cmd, "u64", "ts_ns");
    if (ret) {
// goto;
    }
    ret = synth_event_add_field(&cmd, "u64", "ts_ms");
    if (ret) {
// goto;
    }
    ret = synth_event_add_field(&cmd, "unsigned int", "cpu");
    if (ret) {
// goto;
    }
    ret = synth_event_add_field(&cmd, "char[64]", "my_string_field");
    if (ret) {
// goto;
    }
    ret = synth_event_add_field(&cmd, "int", "my_int_field");
    if (ret) {
// goto;
    }
// All fields have been added, close and register the synth event
    ret = synth_event_gen_cmd_end(&cmd);
    if (ret) {
// goto;
    }
//
// Now get the empty_synth_test event file.  We need to
// prevent the instance and event from disappearing from
// underneath us, which trace_get_event_file() does (though in
// this case we're using the top-level instance which never
// goes away).
//
    empty_synth_test = trace_get_event_file(core::ptr::null_mut(), "synthetic",
    "empty_synth_test");
    if (IS_ERR(empty_synth_test)) {
    ret = PTR_ERR(empty_synth_test);
// goto;
    }
// Enable the event or you won't see anything
    ret = trace_array_set_clr_event(empty_synth_test.tr,
    "synthetic", "empty_synth_test", true);
    if (ret) {
    trace_put_event_file(empty_synth_test);
// goto;
    }
// Create some bogus values just for testing
    vals[0] = 777;			/* next_pid_field */
    vals[1] = (u64)(long)"tiddlywinks";	/* next_comm_field */
    vals[2] = 1000000;		/* ts_ns */
    vals[3] = 1000;			/* ts_ms */
    vals[4] = raw_smp_processor_id(); /* cpu */
    vals[5] = (u64)(long)"thneed_2.0";	/* my_string_field */
    vals[6] = 399;			/* my_int_field */
// Now trace an empty_synth_test event
    ret = synth_event_trace_array(empty_synth_test, vals, ARRAY_SIZE!(vals));
// label;
    kfree(buf);
    return ret;
// label;
// We got an error after creating the event, delete it
    synth_event_delete("empty_synth_test");
// goto;
    }
pub static mut synth_field_desc: usize = 0;
//
// Test synthetic event creation all at once from array of field
// descriptors.
//
#[no_mangle]
unsafe extern "C" fn test_create_synth_event() -> c_int {
    u64 vals[9];
    let mut ret = 0;
// Create the create_synth_test event with the fields above
    ret = synth_event_create("create_synth_test",
    create_synth_test_fields,
    ARRAY_SIZE!(create_synth_test_fields),
    THIS_MODULE);
    if (ret) {
// goto;
    }
//
// Now get the create_synth_test event file.  We need to
// prevent the instance and event from disappearing from
// underneath us, which trace_get_event_file() does (though in
// this case we're using the top-level instance which never
// goes away).
//
    create_synth_test = trace_get_event_file(core::ptr::null_mut(), "synthetic",
    "create_synth_test");
    if (IS_ERR(create_synth_test)) {
    ret = PTR_ERR(create_synth_test);
// goto;
    }
// Enable the event or you won't see anything
    ret = trace_array_set_clr_event(create_synth_test.tr,
    "synthetic", "create_synth_test", true);
    if (ret) {
    trace_put_event_file(create_synth_test);
// goto;
    }
// Create some bogus values just for testing
    vals[0] = 777;			/* next_pid_field */
    vals[1] = (u64)(long)"tiddlywinks";	/* next_comm_field */
    vals[2] = 1000000;		/* ts_ns */
    vals[3] = (u64)(long)"xrayspecs";	/* dynstring_field_1 */
    vals[4] = 1000;			/* ts_ms */
    vals[5] = raw_smp_processor_id(); /* cpu */
    vals[6] = (u64)(long)"thneed";	/* my_string_field */
    vals[7] = (u64)(long)"kerplunk";	/* dynstring_field_2 */
    vals[8] = 398;			/* my_int_field */
// Now generate a create_synth_test event
    ret = synth_event_trace_array(create_synth_test, vals, ARRAY_SIZE!(vals));
// label;
    return ret;
// label;
// We got an error after creating the event, delete it
    synth_event_delete("create_synth_test");
// goto;
    }
//
// Test tracing a synthetic event by reserving trace buffer space,
// then filling in fields one after another.
//
#[no_mangle]
unsafe extern "C" fn test_add_next_synth_val() -> c_int {
pub static mut trace_state: usize = 0;
    let mut ret = 0;
// Start by reserving space in the trace buffer
    ret = synth_event_trace_start(gen_synth_test, &trace_state);
    if (ret) {
    return ret;
    }
// Write some bogus values into the trace buffer, one after another
// next_pid_field
    ret = synth_event_add_next_val(777, &trace_state);
    if (ret) {
// goto;
    }
// next_comm_field
    ret = synth_event_add_next_val((u64)(long)"slinky", &trace_state);
    if (ret) {
// goto;
    }
// ts_ns
    ret = synth_event_add_next_val(1000000, &trace_state);
    if (ret) {
// goto;
    }
// ts_ms
    ret = synth_event_add_next_val(1000, &trace_state);
    if (ret) {
// goto;
    }
// cpu
    ret = synth_event_add_next_val(raw_smp_processor_id(), &trace_state);
    if (ret) {
// goto;
    }
// my_string_field
    ret = synth_event_add_next_val((u64)(long)"thneed_2.01", &trace_state);
    if (ret) {
// goto;
    }
// my_int_field
    ret = synth_event_add_next_val(395, &trace_state);
// label;
// Finally, commit the event
    ret = synth_event_trace_end(&trace_state);
    return ret;
    }
//
// Test tracing a synthetic event by reserving trace buffer space,
// then filling in fields using field names, which can be done in any
// order.
//
#[no_mangle]
unsafe extern "C" fn test_add_synth_val() -> c_int {
pub static mut trace_state: usize = 0;
    let mut ret = 0;
// Start by reserving space in the trace buffer
    ret = synth_event_trace_start(gen_synth_test, &trace_state);
    if (ret) {
    return ret;
    }
// Write some bogus values into the trace buffer, using field names
    ret = synth_event_add_val("ts_ns", 1000000, &trace_state);
    if (ret) {
// goto;
    }
    ret = synth_event_add_val("ts_ms", 1000, &trace_state);
    if (ret) {
// goto;
    }
    ret = synth_event_add_val("cpu", raw_smp_processor_id(), &trace_state);
    if (ret) {
// goto;
    }
    ret = synth_event_add_val("next_pid_field", 777, &trace_state);
    if (ret) {
// goto;
    }
    ret = synth_event_add_val("next_comm_field", (u64)(long)"silly putty",
    &trace_state);
    if (ret) {
// goto;
    }
    ret = synth_event_add_val("my_string_field", (u64)(long)"thneed_9",
    &trace_state);
    if (ret) {
// goto;
    }
    ret = synth_event_add_val("my_int_field", 3999, &trace_state);
// label;
// Finally, commit the event
    ret = synth_event_trace_end(&trace_state);
    return ret;
    }
//
// Test tracing a synthetic event all at once from array of values.
//
#[no_mangle]
unsafe extern "C" fn test_trace_synth_event() -> c_int {
    let mut ret = 0;
// Trace some bogus values just for testing
    ret = synth_event_trace(create_synth_test, 9,	/* number of values */
    (u64)444,		/* next_pid_field */
    (u64)(long)"clackers",	/* next_comm_field */
    (u64)1000000,		/* ts_ns */
    (u64)(long)"viewmaster",/* dynstring_field_1 */
    (u64)1000,		/* ts_ms */
    (u64)raw_smp_processor_id(), /* cpu */
    (u64)(long)"Thneed",	/* my_string_field */
    (u64)(long)"yoyos",	/* dynstring_field_2 */
    (u64)999);		/* my_int_field */
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn synth_event_gen_test_init() -> c_int {
    let mut ret = 0;
    ret = test_gen_synth_cmd();
    if (ret) {
    return ret;
    }
    ret = test_empty_synth_event();
    if (ret) {
    WARN_ON!(trace_array_set_clr_event(gen_synth_test.tr,
    "synthetic",
    "gen_synth_test", false));
    trace_put_event_file(gen_synth_test);
    WARN_ON!(synth_event_delete("gen_synth_test"));
// goto;
    }
    ret = test_create_synth_event();
    if (ret) {
    WARN_ON!(trace_array_set_clr_event(gen_synth_test.tr,
    "synthetic",
    "gen_synth_test", false));
    trace_put_event_file(gen_synth_test);
    WARN_ON!(synth_event_delete("gen_synth_test"));
    WARN_ON!(trace_array_set_clr_event(empty_synth_test.tr,
    "synthetic",
    "empty_synth_test", false));
    trace_put_event_file(empty_synth_test);
    WARN_ON!(synth_event_delete("empty_synth_test"));
// goto;
    }
    ret = test_add_next_synth_val();
    WARN_ON!(ret);
    ret = test_add_synth_val();
    WARN_ON!(ret);
    ret = test_trace_synth_event();
    WARN_ON!(ret);
// Disable when done
    trace_array_set_clr_event(gen_synth_test.tr,
    "synthetic",
    "gen_synth_test", false);
    trace_array_set_clr_event(empty_synth_test.tr,
    "synthetic",
    "empty_synth_test", false);
    trace_array_set_clr_event(create_synth_test.tr,
    "synthetic",
    "create_synth_test", false);
// label;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn synth_event_gen_test_exit()  {
// Disable the event or you can't remove it
    WARN_ON!(trace_array_set_clr_event(gen_synth_test.tr,
    "synthetic",
    "gen_synth_test", false));
// Now give the file and instance back
    trace_put_event_file(gen_synth_test);
// Now unregister and free the synthetic event
    WARN_ON!(synth_event_delete("gen_synth_test"));
// Disable the event or you can't remove it
    WARN_ON!(trace_array_set_clr_event(empty_synth_test.tr,
    "synthetic",
    "empty_synth_test", false));
// Now give the file and instance back
    trace_put_event_file(empty_synth_test);
// Now unregister and free the synthetic event
    WARN_ON!(synth_event_delete("empty_synth_test"));
// Disable the event or you can't remove it
    WARN_ON!(trace_array_set_clr_event(create_synth_test.tr,
    "synthetic",
    "create_synth_test", false));
// Now give the file and instance back
    trace_put_event_file(create_synth_test);
// Now unregister and free the synthetic event
    WARN_ON!(synth_event_delete("create_synth_test"));
    }
    module_init!(synth_event_gen_test_init)
    module_exit!(synth_event_gen_test_exit)
    MODULE_AUTHOR("Tom Zanussi");
    MODULE_DESCRIPTION("synthetic event generation test");
    MODULE_LICENSE("GPL v2");