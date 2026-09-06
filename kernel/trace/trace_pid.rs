//! Automatically rewritten from C to Rust
//! Source: kernel/trace/trace_pid.c
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
// trace_find_filtered_pid - check if a pid exists in a filtered_pid list
// @filtered_pids: The list of pids to check
// @search_pid: The PID to find in @filtered_pids
//
// Returns true if @search_pid is found in @filtered_pids, and false otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn trace_find_filtered_pid(filtered_pids: *mut trace_pid_list, search_pid: pid_t) -> bool {
    return trace_pid_list_is_set(filtered_pids, search_pid);
    }
//
// trace_ignore_this_task - should a task be ignored for tracing
// @filtered_pids: The list of pids to check
// @filtered_no_pids: The list of pids not to be traced
// @task: The task that should be ignored if not filtered
//
// Checks if @task should be traced or not from @filtered_pids.
// Returns true if @task should *NOT* be traced.
// Returns false if @task should be traced.
//
#[no_mangle]
pub unsafe extern "C" fn trace_ignore_this_task(filtered_pids: *mut trace_pid_list, filtered_no_pids: *mut trace_pid_list, task: *mut task_struct) -> bool {
//
// If filtered_no_pids is not empty, and the task's pid is listed
// in filtered_no_pids, then return true.
// Otherwise, if filtered_pids is empty, that means we can
// trace all tasks. If it has content, then only trace pids
// within filtered_pids.
//
    return (filtered_pids &&
    !trace_find_filtered_pid(filtered_pids, task.pid)) ||
    (filtered_no_pids &&
    trace_find_filtered_pid(filtered_no_pids, task.pid));
    }
//
// trace_filter_add_remove_task - Add or remove a task from a pid_list
// @pid_list: The list to modify
// @self: The current task for fork or NULL for exit
// @task: The task to add or remove
//
// If adding a task, if @self is defined, the task is only added if @self
// is also included in @pid_list. This happens on fork and tasks should
// only be added when the parent is listed. If @self is NULL, then the
// @task pid will be removed from the list, which would happen on exit
// of a task.
//
#[no_mangle]
pub unsafe extern "C" fn trace_filter_add_remove_task(pid_list: *mut trace_pid_list, self: *mut task_struct, task: *mut task_struct) {
    if (!pid_list) {
    return;
    }
// For forks, we only add if the forking task is listed
    if (self) {
    if (!trace_find_filtered_pid(pid_list, self.pid)) {
    return;
    }
    }
// "self" is set for forks, and NULL for exits
    if (self) {
    trace_pid_list_set(pid_list, task.pid);
    }
    else {
    trace_pid_list_clear(pid_list, task.pid);
    }
    }
//
// trace_pid_next - Used for seq_file to get to the next pid of a pid_list
// @pid_list: The pid list to show
// @v: The last pid that was shown (+1 the actual pid to let zero be displayed)
// @pos: The position of the file
//
// This is used by the seq_file "next" operation to iterate the pids
// listed in a trace_pid_list structure.
//
// Returns the pid+1 as we want to display pid of zero, but NULL would
// stop the iteration.
//
#[no_mangle]
pub unsafe extern "C" fn trace_pid_next(pid_list: *mut trace_pid_list, v: *mut c_void, pos: *mut loff_t) -> *mut c_void {
pub static mut pid: c_long = 0;
    let mut next = 0;
    (*pos)++;
// pid already is +1 of the actual previous bit
    if (trace_pid_list_next(pid_list, pid, &next) < 0) {
    return core::ptr::null_mut();
    }
    pid = next;
// Return pid + 1 to allow zero to be represented
    return (pid + 1);
    }
//
// trace_pid_start - Used for seq_file to start reading pid lists
// @pid_list: The pid list to show
// @pos: The position of the file
//
// This is used by seq_file "start" operation to start the iteration
// of listing pids.
//
// Returns the pid+1 as we want to display pid of zero, but NULL would
// stop the iteration.
//
#[no_mangle]
pub unsafe extern "C" fn trace_pid_start(pid_list: *mut trace_pid_list, pos: *mut loff_t) -> *mut c_void {
    let mut pid = 0;
    let mut first = 0;
pub static mut l: loff_t = 0;
    if (trace_pid_list_first(pid_list, &first) < 0) {
    return core::ptr::null_mut();
    }
    pid = first;
// Return pid + 1 so that zero can be the exit value
    for (pid += 1; pid && l < *pos;
    pid = (unsigned long)trace_pid_next(pid_list, pid, &l)) {
    ;
    }
    return pid;
    }
//
// trace_pid_show - show the current pid in seq_file processing
// @m: The seq_file structure to write into
// @v: A void pointer of the pid (+1) value to display
//
// Can be directly used by seq_file operations to display the current
// pid value.
//
#[no_mangle]
pub unsafe extern "C" fn trace_pid_show(m: *mut seq_file, v: *mut c_void) -> c_int {
pub static mut pid: c_ulong = 0;
    seq_printf(m, "%lu\n", pid);
    return 0;
    }
// 128 should be much more than enough
pub const PID_BUF_SIZE: c_int = 127;
#[no_mangle]
pub unsafe extern "C" fn trace_pid_write(filtered_pids: *mut trace_pid_list, new_pid_list: *mut *mut trace_pid_list, ubuf: *mut c_char, cnt: size_t) -> c_int {
pub static mut pid_list: *mut c_void = core::ptr::null_mut();
pub static mut parser: usize = 0;
    let mut val = 0;
pub static mut nr_pids: c_int = 0;
pub static mut read: isize = 0;
    let mut ret = 0;
    let mut pos = 0;
    let mut pid = 0;
    if (trace_parser_get_init(&parser, PID_BUF_SIZE + 1)) {
    return -ENOMEM;
    }
//
// Always recreate a new array. The write is an all or nothing
// operation. Always create a new array when adding new pids by
// the user. If the operation fails, then the current list is
// not modified.
//
    pid_list = trace_pid_list_alloc();
    if (!pid_list) {
    trace_parser_put(&parser);
    return -ENOMEM;
    }
    if (filtered_pids) {
// copy the current bits to the new max
    ret = trace_pid_list_first(filtered_pids, &pid);
    while (!ret) {
    ret = trace_pid_list_set(pid_list, pid);
    if (ret < 0) {
// goto;
    }
    ret = trace_pid_list_next(filtered_pids, pid + 1, &pid);
    nr_pids += 1;
    }
    }
    ret = 0;
    while (cnt > 0) {
    pos = 0;
    ret = trace_get_user(&parser, ubuf, cnt, &pos);
    if (ret < 0) {
    break;
    }
    read += ret;
    ubuf += ret;
    cnt -= ret;
    if (!trace_parser_loaded(&parser)) {
    break;
    }
    ret = -EINVAL;
    if (kstrtoul(parser.buffer, 0, &val)) {
    break;
    }
    pid = (pid_t)val;
    if (trace_pid_list_set(pid_list, pid) < 0) {
    ret = -1;
    break;
    }
    nr_pids += 1;
    trace_parser_clear(&parser);
    ret = 0;
    }
// label;
    trace_parser_put(&parser);
    if (ret < 0) {
    trace_pid_list_free(pid_list);
    return ret;
    }
    if (!nr_pids) {
// Cleared the list of pids
    trace_pid_list_free(pid_list);
    pid_list = core::ptr::null_mut();
    }
// new_pid_list = pid_list;
    return read;
    }