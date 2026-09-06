//! Automatically rewritten from C to Rust
//! Source: kernel/debug/kdb/kdb_debugger.c
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
// Created by: Jason Wessel <jason.wessel@windriver.com>
//
// Copyright (c) 2009 Wind River Systems, Inc.  All Rights Reserved.
//
// This file is licensed under the terms of the GNU General Public
// License version 2. This program is licensed "as is" without any
// warranty of any kind, whether express or implied.
//

//
// KDB interface to KGDB internals
//
    get_char_func kdb_poll_funcs[] = {
    dbg_io_get_char,
    core::ptr::null_mut(),
    core::ptr::null_mut(),
    core::ptr::null_mut(),
    core::ptr::null_mut(),
    core::ptr::null_mut(),
    };
    EXPORT_SYMBOL_GPL(kdb_poll_funcs);
pub static mut kdb_poll_idx: c_int = 1;
    EXPORT_SYMBOL_GPL(kdb_poll_idx);
pub static mut kdb_ks: *mut c_void = core::ptr::null_mut();
#[no_mangle]
pub unsafe extern "C" fn kdb_common_init_state(ks: *mut kgdb_state) -> c_int {
    kdb_initial_cpu = atomic_read(&kgdb_active);
    kdb_current_task = kgdb_info[ks.cpu].task;
    kdb_current_regs = kgdb_info[ks.cpu].debuggerinfo;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn kdb_common_deinit_state() -> c_int {
    kdb_initial_cpu = -1;
    kdb_current_task = core::ptr::null_mut();
    kdb_current_regs = core::ptr::null_mut();
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn kdb_stub(ks: *mut kgdb_state) -> c_int {
pub static mut error: c_int = 0;
pub static mut bp: *mut c_void = core::ptr::null_mut();
pub static mut addr: c_ulong = 0;
pub static mut reason: kdb_reason_t = 0;
pub static mut db_result: kdb_dbtrap_t = 0;
    let mut i = 0;
    kdb_ks = ks;
    if (KDB_STATE(REENTRY)) {
    reason = KDB_REASON_SWITCH;
    KDB_STATE_CLEAR(REENTRY);
    addr = instruction_pointer(ks.linux_regs);
    }
    ks.pass_exception = 0;
    if (atomic_read(&kgdb_setting_breakpoint)) {
    reason = KDB_REASON_KEYBOARD;
    }
    if (ks.err_code == KDB_REASON_SYSTEM_NMI && ks.signo == SIGTRAP) {
    reason = KDB_REASON_SYSTEM_NMI;
    }

    else if (in_nmi()) {
    reason = KDB_REASON_NMI;
    }
    while (i < KDB_MAXBPT) {
    if ((bp.bp_enabled) && (bp.bp_addr == addr)) {
    reason = KDB_REASON_BREAK;
    db_result = KDB_DB_BPT;
    if (addr != instruction_pointer(ks.linux_regs)) {
    kgdb_arch_set_pc(ks.linux_regs, addr);
    }
    break;
    }
    }
    if (reason == KDB_REASON_BREAK || reason == KDB_REASON_SWITCH) {
    while (i < KDB_MAXBPT) {
    if (bp.bp_free) {
    continue;
    }
    if (bp.bp_addr == addr) {
    bp.bp_delay = 1;
    bp.bp_delayed = 1;
//
// SSBPT is set when the kernel debugger must single step a
// task in order to re-establish an instruction breakpoint
// which uses the instruction replacement mechanism.  It is
// cleared by any action that removes the need to single-step
// the breakpoint.
//
    reason = KDB_REASON_BREAK;
    db_result = KDB_DB_BPT;
    KDB_STATE_SET(SSBPT);
    break;
    }
    }
    }
    if (reason != KDB_REASON_BREAK && ks.ex_vector == 0 &&
    ks.signo == SIGTRAP) {
    reason = KDB_REASON_SSTEP;
    db_result = KDB_DB_BPT;
    }
// Set initial kdb state variables
    KDB_STATE_CLEAR(KGDB_TRANS);
    kdb_common_init_state(ks);
// Remove any breakpoints as needed by kdb and clear single step
    kdb_bp_remove();
    KDB_STATE_CLEAR(DOING_SS);
    KDB_STATE_SET(PAGER);
    if (ks.err_code == DIE_OOPS || reason == KDB_REASON_OOPS) {
    ks.pass_exception = 1;
    KDB_FLAG_SET(CATASTROPHIC);
    }
// set CATASTROPHIC if the system contains unresponsive processors
    for_each_online_cpu(i) {
    if (!kgdb_info[i].enter_kgdb)
    KDB_FLAG_SET(CATASTROPHIC);
    }
    if (KDB_STATE(SSBPT) && reason == KDB_REASON_SSTEP) {
    KDB_STATE_CLEAR(SSBPT);
    KDB_STATE_CLEAR(DOING_SS);
    } else {
// Start kdb main loop
    error = kdb_main_loop(KDB_REASON_ENTER, reason,
    ks.err_code, db_result, ks.linux_regs);
    }
//
// Upon exit from the kdb main loop setup break points and restart
// the system based on the requested continue state
//
    kdb_common_deinit_state();
    KDB_STATE_CLEAR(PAGER);
    if (error == KDB_CMD_KGDB) {
    if (KDB_STATE(DOING_KGDB)) {
    KDB_STATE_CLEAR(DOING_KGDB);
    }
    return DBG_PASS_EVENT;
    }
    kdb_bp_install(ks.linux_regs);
// Set the exit state to a single step or a continue
    if (KDB_STATE(DOING_SS)) {
    gdbstub_state(ks, "s");
    }
    else {
    gdbstub_state(ks, "c");
    }
    KDB_FLAG_CLEAR(CATASTROPHIC);
// Invoke arch specific exception handling prior to system resume
    kgdb_info[ks.cpu].ret_state = gdbstub_state(ks, "e");
    if (ks.pass_exception) {
    kgdb_info[ks.cpu].ret_state = 1;
    }
    if (error == KDB_CMD_CPU) {
    KDB_STATE_SET(REENTRY);
//
// Force clear the single step bit because kdb emulates this
// differently vs the gdbstub
//
    kgdb_single_step = 0;
    return DBG_SWITCH_CPU_EVENT;
    }
    return kgdb_info[ks.cpu].ret_state;
    }
#[no_mangle]
pub unsafe extern "C" fn kdb_gdb_state_pass(buf: *mut c_char) {
    gdbstub_state(kdb_ks, buf);
    }