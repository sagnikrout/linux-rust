//! Automatically rewritten from C to Rust
//! Source: kernel/trace/remote_test.c
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
// Copyright (C) 2025 - Google LLC
// Author: Vincent Donnefort <vdonnefort@google.com>
//

pub static mut struct simple_rb_per_cpu *: usize = 0;
pub static mut remote_test_buffer_desc: *mut c_void = core::ptr::null_mut();
//
// The trace_remote lock already serializes accesses from the trace_remote_callbacks.
// However write_event can still race with load/unload.
//
pub static mut simple_rbs_lock: usize = 0;
#[no_mangle]
unsafe extern "C" fn remote_test_load_simple_rb(cpu: c_int, rb_desc: *mut ring_buffer_desc) -> c_int {
pub static mut cpu_buffer: *mut c_void = core::ptr::null_mut();
pub static mut bpages: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    cpu_buffer = kmalloc_obj(*cpu_buffer);
    if (!cpu_buffer) {
    return ret;
    }
    bpages = kmalloc_objs(*bpages, rb_desc.nr_page_va);
    if (!bpages) {
// goto;
    }
    ret = simple_ring_buffer_init(cpu_buffer, bpages, rb_desc);
    if (ret) {
// goto;
    }
    scoped_guard(mutex, &simple_rbs_lock) {
    WARN_ON!(*per_cpu_ptr(&simple_rbs, cpu));
// per_cpu_ptr(&simple_rbs, cpu) = cpu_buffer;
    }
    return 0;
// label;
    kfree(bpages);
// label;
    kfree(cpu_buffer);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn remote_test_unload_simple_rb(cpu: c_int) {
    let mut cpu_buffer = *per_cpu_ptr(&simple_rbs, cpu);
pub static mut bpages: *mut c_void = core::ptr::null_mut();
    if (!cpu_buffer) {
    return;
    }
    guard(mutex)(&simple_rbs_lock);
    bpages = cpu_buffer.bpages;
    simple_ring_buffer_unload(cpu_buffer);
    kfree(bpages);
    kfree(cpu_buffer);
// per_cpu_ptr(&simple_rbs, cpu) = NULL;
    }
#[no_mangle]
pub unsafe extern "C" fn remote_test_load(size: c_ulong, unused: *mut c_void) -> *mut c_void {
pub static mut rb_desc: *mut c_void = core::ptr::null_mut();
pub static mut desc: *mut c_void = core::ptr::null_mut();
    let mut desc_size = 0;
    let mut cpu = 0;
    let mut ret = 0;
    if (WARN_ON!(remote_test_buffer_desc)) {
    return ERR_PTR(-EINVAL);
    }
    desc_size = trace_buffer_desc_size(size, num_possible_cpus());
    if (desc_size == SIZE_MAX) {
    ret = -E2BIG;
// goto;
    }
    desc = kmalloc(desc_size, GFP_KERNEL);
    if (!desc) {
    ret = -ENOMEM;
// goto;
    }
    ret = trace_remote_alloc_buffer(desc, desc_size, size, cpu_possible_mask);
    if (ret) {
// goto;
    }
    for_each_ring_buffer_desc(rb_desc, cpu, desc) {
    ret = remote_test_load_simple_rb(rb_desc.cpu, rb_desc);
    if (ret) {
// goto;
    }
    }
    remote_test_buffer_desc = desc;
    return remote_test_buffer_desc;
// label;
    for_each_ring_buffer_desc(rb_desc, cpu, desc) {
    remote_test_unload_simple_rb(rb_desc.cpu);
    }
    trace_remote_free_buffer(desc);
// label;
    kfree(desc);
// label;
    return ERR_PTR(ret);
    }
#[no_mangle]
unsafe extern "C" fn remote_test_unload(desc: *mut trace_buffer_desc, unused: *mut c_void) {
pub static mut rb_desc: *mut c_void = core::ptr::null_mut();
    let mut cpu = 0;
    if (WARN_ON!(desc != remote_test_buffer_desc)) {
    return;
    }
    for_each_ring_buffer_desc(rb_desc, cpu, desc) {
    remote_test_unload_simple_rb(rb_desc.cpu);
    }
    remote_test_buffer_desc = core::ptr::null_mut();
    trace_remote_free_buffer(desc);
    kfree(desc);
    }
#[no_mangle]
unsafe extern "C" fn remote_test_enable_tracing(enable: bool, unused: *mut c_void) -> c_int {
pub static mut rb_desc: *mut c_void = core::ptr::null_mut();
    let mut cpu = 0;
    if (!remote_test_buffer_desc) {
    return -ENODEV;
    }
    for_each_ring_buffer_desc(rb_desc, cpu, remote_test_buffer_desc) {
    WARN_ON!(simple_ring_buffer_enable_tracing(*per_cpu_ptr(&simple_rbs, rb_desc.cpu),
    enable));
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn remote_test_swap_reader_page(cpu: c_uint, unused: *mut c_void) -> c_int {
pub static mut cpu_buffer: *mut c_void = core::ptr::null_mut();
    if (cpu >= NR_CPUS) {
    return -EINVAL;
    }
    cpu_buffer = *per_cpu_ptr(&simple_rbs, cpu);
    if (!cpu_buffer) {
    return -EINVAL;
    }
    return simple_ring_buffer_swap_reader_page(cpu_buffer);
    }
#[no_mangle]
unsafe extern "C" fn remote_test_reset(cpu: c_uint, unused: *mut c_void) -> c_int {
pub static mut cpu_buffer: *mut c_void = core::ptr::null_mut();
    if (cpu >= NR_CPUS) {
    return -EINVAL;
    }
    cpu_buffer = *per_cpu_ptr(&simple_rbs, cpu);
    if (!cpu_buffer) {
    return -EINVAL;
    }
    return simple_ring_buffer_reset(cpu_buffer);
    }
#[no_mangle]
unsafe extern "C" fn remote_test_enable_event(id: c_ushort, enable: bool, unused: *mut c_void) -> c_int {
    if (id != REMOTE_TEST_EVENT_ID) {
    return -EINVAL;
    }
//
// Let's just use the struct remote_event enabled field that is turned on and off by
// trace_remote. This is a bit racy but good enough for a simple test module.
//
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn write_event_write(filp: *mut file, ubuf: *mut c_char, cnt: size_t, pos: *mut loff_t) -> ssize_t {
pub static mut evt_test: *mut c_void = core::ptr::null_mut();
pub static mut cpu_buffer: *mut c_void = core::ptr::null_mut();
    let mut val = 0;
    let mut ret = 0;
    ret = kstrtoul_from_user(ubuf, cnt, 10, &val);
    if (ret) {
    return ret;
    }
    guard(mutex)(&simple_rbs_lock);
    if (!remote_event_selftest.enabled) {
    return -ENODEV;
    }
    guard(preempt)();
    cpu_buffer = *this_cpu_ptr(&simple_rbs);
    if (!cpu_buffer) {
    return -ENODEV;
    }
    evt_test = simple_ring_buffer_reserve(cpu_buffer,
    sizeof!(remote_event_format_selftest),
    trace_clock_global());
    if (!evt_test) {
    return -ENODEV;
    }
    evt_test.hdr.id = REMOTE_TEST_EVENT_ID;
    evt_test.id = val;
    simple_ring_buffer_commit(cpu_buffer);
    return cnt;
    }
pub static mut file_operations: usize = 0;
#[no_mangle]
unsafe extern "C" fn remote_test_init_tracefs(d: *mut dentry, unused: *mut c_void) -> c_int {
    return tracefs_create_file("write_event", 0200, d, core::ptr::null_mut(), &write_event_fops) ?
    0 : -ENOMEM;
    }
pub static mut trace_remote_callbacks: usize = 0;
#[no_mangle]
unsafe extern "C" fn remote_test_init() -> c_int {
    return trace_remote_register("test", &trace_remote_callbacks, core::ptr::null_mut(),
    &remote_event_selftest, 1);
    }
    module_init!(remote_test_init);
    MODULE_DESCRIPTION("Test module for the trace remote interface");
    MODULE_AUTHOR("Vincent Donnefort");
    MODULE_LICENSE("GPL");