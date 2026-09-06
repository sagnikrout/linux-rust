//! Automatically rewritten from C to Rust
//! Source: kernel/trace/rv/rv_reactors.c
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
// Copyright (C) 2019-2022 Red Hat, Inc. Daniel Bristot de Oliveira <bristot@kernel.org>
//
// Runtime reactor interface.
//
// A runtime monitor can cause a reaction to the detection of an
// exception on the model's execution. By default, the monitors have
// tracing reactions, printing the monitor output via tracepoints.
// But other reactions can be added (on-demand) via this interface.
//
// == Registering reactors ==
//
// The struct rv_reactor defines a callback function to be executed
// in case of a model exception happens. The callback function
// receives a message to be (optionally) printed before executing
// the reaction.
//
// A RV reactor is registered via:
// int rv_register_reactor(rv_reactor *reactor)
// And unregistered via:
// int rv_unregister_reactor(rv_reactor *reactor)
//
// These functions are exported to modules, enabling reactors to be
// dynamically loaded.
//
// == User interface ==
//
// The user interface resembles the kernel tracing interface and
// presents these files:
//
// "available_reactors"
// - List the available reactors, one per line.
//
// For example:
// # cat available_reactors
// nop
// panic
// printk
//
// "reacting_on"
// - It is an on/off general switch for reactors, disabling
// all reactions.
//
// "monitors/MONITOR/reactors"
// - List available reactors, with the select reaction for the given
// MONITOR inside []. The default one is the nop (no operation)
// reactor.
// - Writing the name of an reactor enables it to the given
// MONITOR.
//
// For example:
// # cat monitors/wip/reactors
// [nop]
// panic
// printk
// # echo panic > monitors/wip/reactors
// # cat monitors/wip/reactors
// nop
// [panic]
// printk
//

//
// Interface for the reactor register.
//
pub static mut rv_reactors_list: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn get_reactor_rdef_by_name(name: *mut c_char) -> *mut c_void {
pub static mut r: *mut c_void = core::ptr::null_mut();
    list_for_each_entry(r, &rv_reactors_list, list) {
    if (strcmp(name, r.name) == 0) {
    return r;
    }
    }
    return core::ptr::null_mut();
    }
//
// Available reactors seq functions.
//
#[no_mangle]
unsafe extern "C" fn reactors_show(m: *mut seq_file, p: *mut c_void) -> c_int {
    let mut reactor = container_of!(p, rv_reactor, list);
    seq_printf(m, "%s\n", reactor.name);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn reactors_stop(m: *mut seq_file, p: *mut c_void) {
    mutex_unlock(&rv_interface_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn reactors_start(m: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
    mutex_lock(&rv_interface_lock);
    return seq_list_start(&rv_reactors_list, *pos);
    }
#[no_mangle]
pub unsafe extern "C" fn reactors_next(m: *mut seq_file, p: *mut c_void, pos: *mut loff_t) -> *mut c_void {
    return seq_list_next(p, &rv_reactors_list, pos);
    }
//
// available_reactors seq definition.
//
pub static mut seq_operations: usize = 0;
//
// available_reactors interface.
//
#[no_mangle]
unsafe extern "C" fn available_reactors_open(inode: *mut inode, file: *mut file) -> c_int {
    return seq_open(file, &available_reactors_seq_ops);
    };
pub static mut file_operations: usize = 0;
//
// Monitor's reactor file.
//
#[no_mangle]
unsafe extern "C" fn monitor_reactor_show(m: *mut seq_file, p: *mut c_void) -> c_int {
    let mut mon = m.private;
    let mut reactor = container_of!(p, rv_reactor, list);
    if (mon.reactor == reactor) {
    seq_printf(m, "[%s]\n", reactor.name);
    }
    else {
    seq_printf(m, "%s\n", reactor.name);
    }
    return 0;
    }
//
// available_reactors seq definition.
//
pub static mut seq_operations: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn monitor_swap_reactors_single(mon: *mut rv_monitor, reactor: *mut rv_reactor, nested: bool) {
    let mut monitor_enabled = 0;
// nothing to do
    if (mon.reactor == reactor) {
    return;
    }
    monitor_enabled = mon.enabled;
    if (monitor_enabled) {
    rv_disable_monitor(mon);
    }
    mon.reactor = reactor;
    mon.react = reactor.react;
// enable only once if iterating through a container
    if (monitor_enabled && !nested) {
    rv_enable_monitor(mon);
    }
    }
#[no_mangle]
unsafe extern "C" fn monitor_swap_reactors(mon: *mut rv_monitor, reactor: *mut rv_reactor) {
    let mut p = mon;
    if (rv_is_container_monitor(mon)) {
    list_for_each_entry_continue(p, &rv_monitors_list, list) {
    }
    if (p.parent != mon) {
    break;
    }
    monitor_swap_reactors_single(p, reactor, true);
    }
//
// This call enables and disables the monitor if they were active.
// In case of a container, we already disabled all and will enable all.
// All nested monitors are enabled also if they were off, we may refine
// this logic in the future.
//
    monitor_swap_reactors_single(mon, reactor, false);
    }
#[no_mangle]
pub unsafe extern "C" fn monitor_reactors_write(file: *mut file, user_buf: *mut c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    char buff[MAX_RV_REACTOR_NAME_SIZE + 2];
pub static mut mon: *mut c_void = core::ptr::null_mut();
pub static mut reactor: *mut c_void = core::ptr::null_mut();
pub static mut seq_f: *mut c_void = core::ptr::null_mut();
pub static mut retval: c_int = 0;
pub static mut ptr: *mut c_void = core::ptr::null_mut();
    let mut len = 0;
    if (count < 1 || count > MAX_RV_REACTOR_NAME_SIZE + 1) {
    return -EINVAL;
    }
    memset(buff, 0, sizeof!(buff));
    retval = simple_write_to_buffer(buff, sizeof!(buff) - 1, ppos, user_buf, count);
    if (retval < 0) {
    return -EFAULT;
    }
    ptr = strim(buff);
    len = strlen(ptr);
    if (!len) {
    return count;
    }
//
// See monitor_reactors_open()
//
    seq_f = file.private_data;
    mon = seq_f.private;
    guard(mutex)(&rv_interface_lock);
    list_for_each_entry(reactor, &rv_reactors_list, list) {
    if (strcmp(ptr, reactor.name) != 0) {
    continue;
    }
    monitor_swap_reactors(mon, reactor);
    return count;
    }
    return -EINVAL;
    }
//
// available_reactors interface.
//
#[no_mangle]
unsafe extern "C" fn monitor_reactors_open(inode: *mut inode, file: *mut file) -> c_int {
    let mut mon = inode.i_private;
pub static mut seq_f: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    ret = seq_open(file, &monitor_reactors_seq_ops);
    if (ret < 0) {
    return ret;
    }
//
// seq_open stores the seq_file on the file->private data.
//
    seq_f = file.private_data;
//
// Copy the create file "private" data to the seq_file private data.
//
    seq_f.private = mon;
    return 0;
    };
pub static mut file_operations: usize = 0;
#[no_mangle]
unsafe extern "C" fn __rv_register_reactor(reactor: *mut rv_reactor) -> c_int {
pub static mut r: *mut c_void = core::ptr::null_mut();
    list_for_each_entry(r, &rv_reactors_list, list) {
    if (strcmp(reactor.name, r.name) == 0) {
    pr_info!("Reactor %s is already registered\n", reactor.name);
    return -EINVAL;
    }
    }
    list_add_tail(&reactor.list, &rv_reactors_list);
    return 0;
    }
//
// rv_register_reactor - register a rv reactor.
// @reactor:	The rv_reactor to be registered.
//
// Returns 0 if successful, error otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn rv_register_reactor(reactor: *mut rv_reactor) -> c_int {
    if (strlen(reactor.name) >= MAX_RV_REACTOR_NAME_SIZE) {
    pr_info!("Reactor %s has a name longer than %d\n",
    reactor.name, MAX_RV_MONITOR_NAME_SIZE);
    return -EINVAL;
    }
    guard(mutex)(&rv_interface_lock);
    return __rv_register_reactor(reactor);
    }
//
// rv_unregister_reactor - unregister a rv reactor.
// @reactor:	The rv_reactor to be unregistered.
//
// Returns 0 if successful, error otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn rv_unregister_reactor(reactor: *mut rv_reactor) -> c_int {
    guard(mutex)(&rv_interface_lock);
    list_del(&reactor.list);
    return 0;
    }
//
// reacting_on interface.
//
    static bool  reacting_on;
//
// rv_reacting_on - checks if reacting is on
//
// Returns 1 if on, 0 otherwise.
//
#[no_mangle]
unsafe extern "C" fn rv_reacting_on() -> bool {
// Ensures that concurrent monitors read consistent reacting_on
    smp_rmb();
    return READ_ONCE(reacting_on);
    }
#[no_mangle]
pub unsafe extern "C" fn reacting_on_read_data(filp: *mut file, user_buf: *mut c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
pub static mut buff: *mut c_void = core::ptr::null_mut();
    buff = rv_reacting_on() ? "1\n" : "0\n";
    return simple_read_from_buffer(user_buf, count, ppos, buff, strlen(buff)+1);
    }
#[no_mangle]
unsafe extern "C" fn turn_reacting_off() {
    WRITE_ONCE(reacting_on, false);
// Ensures that concurrent monitors read consistent reacting_on
    smp_wmb();
    }
#[no_mangle]
unsafe extern "C" fn turn_reacting_on() {
    WRITE_ONCE(reacting_on, true);
// Ensures that concurrent monitors read consistent reacting_on
    smp_wmb();
    }
#[no_mangle]
pub unsafe extern "C" fn reacting_on_write_data(filp: *mut file, user_buf: *mut c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    let mut retval = 0;
    let mut val = 0;
    retval = kstrtobool_from_user(user_buf, count, &val);
    if (retval) {
    return retval;
    }
    guard(mutex)(&rv_interface_lock);
    if (val) {
    turn_reacting_on();
    }
    else {
    turn_reacting_off();
    }
//
// Wait for the execution of all events to finish
// before returning to user-space.
//
    tracepoint_synchronize_unregister();
    return count;
    }
pub static mut file_operations: usize = 0;
//
// reactor_populate_monitor - creates per monitor reactors file
// @mon:	The monitor.
// @root:	The directory of the monitor.
//
// Returns 0 if successful, error otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn reactor_populate_monitor(mon: *mut rv_monitor, root: *mut dentry) -> c_int {
pub static mut tmp: *mut c_void = core::ptr::null_mut();
    tmp = rv_create_file("reactors", RV_MODE_WRITE, root, mon, &monitor_reactors_ops);
    if (!tmp) {
    return -ENOMEM;
    }
//
// Configure as the rv_nop reactor.
//
    mon.reactor = get_reactor_rdef_by_name("nop");
    return 0;
    }
//
// Nop reactor register
//
    __printf(1, 0) static void rv_nop_reaction(const char *msg, va_list args)
    {
    }
pub static mut rv_reactor: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn init_rv_reactors(root_dir: *mut dentry) -> c_int {
    let mut retval = 0;
    struct dentry *available __free(rv_remove) =
    rv_create_file("available_reactors", RV_MODE_READ, root_dir,
    core::ptr::null_mut(), &available_reactors_ops);
    struct dentry *reacting __free(rv_remove) =
    rv_create_file("reacting_on", RV_MODE_WRITE, root_dir, core::ptr::null_mut(), &reacting_on_fops);
    if (!reacting || !available) {
    return -ENOMEM;
    }
    retval = __rv_register_reactor(&rv_nop);
    if (retval) {
    return retval;
    }
    turn_reacting_on();
    retain_and_null_ptr(available);
    retain_and_null_ptr(reacting);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn rv_react(monitor: *mut rv_monitor, msg: *const c_char, ...) {
pub static mut rv_react_map: usize = 0;
    let mut args;
    if (!rv_reacting_on() || !monitor.react) {
    return;
    }
    va_start(args, msg);
    lock_map_acquire_try(&rv_react_map);
    monitor.react(msg, args);
    lock_map_release(&rv_react_map);
    va_end(args);
    }
    EXPORT_SYMBOL_GPL(rv_react);