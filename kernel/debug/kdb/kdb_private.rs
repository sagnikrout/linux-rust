//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/debug/kdb/kdb_private.h
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
// Kernel Debugger Architecture Independent Private Headers
//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file "COPYING" in the main directory of this archive
// for more details.
//
// Copyright (c) 2000-2004 Silicon Graphics, Inc.  All Rights Reserved.
// Copyright (c) 2009 Wind River Systems, Inc.  All Rights Reserved.
//

// Kernel Debugger Command codes.  Must not overlap with error codes.

// Internal debug flags
pub const KDB_DEBUG_FLAG_BP: c_uint = 0x0002	/* Breakpoint subsystem debug */;
pub const KDB_DEBUG_FLAG_BB_SUMM: c_uint = 0x0004	/* Basic block analysis, summary only */;
pub const KDB_DEBUG_FLAG_AR: c_uint = 0x0008	/* Activation record, generic */;
pub const KDB_DEBUG_FLAG_ARA: c_uint = 0x0010	/* Activation record, arch specific */;
pub const KDB_DEBUG_FLAG_BB: c_uint = 0x0020	/* All basic block analysis */;
pub const KDB_DEBUG_FLAG_STATE: c_uint = 0x0040	/* State flags */;
pub const KDB_DEBUG_FLAG_MASK: c_uint = 0xffff	/* All debug flags */;

//
// KDB_MAXBPT describes the total number of breakpoints
// supported by this architecture.
//
pub const KDB_MAXBPT: c_int = 16;
// Symbol table format returned by kallsyms.
// "kernel"
// any version
extern "C" {
    pub fn kallsyms_symbol_next(prefix_name: *mut c_char, flag: c_int, buf_size: c_int) -> c_int;
}
extern "C" {
    pub fn kallsyms_symbol_complete(prefix_name: *mut c_char, max_len: c_int) -> c_int;
}
// Exported Symbols for kernel loadable modules to use.
extern "C" {
    pub fn kdb_getarea_size(: *mut c_void, long: unsigned, _arg: usize) -> c_int;
}
extern "C" {
    pub fn kdb_putarea_size(long: unsigned, : *mut c_void, _arg: usize) -> c_int;
}
//
// Like get_user and put_user, kdb_getarea and kdb_putarea take variable
// names, not pointers.  The underlying *_size functions take pointers.
//

extern "C" {
    pub fn kdb_getword(: *mut c_ulong, long: unsigned, _arg: usize) -> c_int;
}
extern "C" {
    pub fn kdb_putword(long: unsigned, long: unsigned, _arg: usize) -> c_int;
}
extern "C" {
    pub fn kdbgetularg(: *const c_char, : *mut c_ulong) -> c_int;
}
extern "C" {
    pub fn kdbgetu64arg(: *const c_char, : *mut u64) -> c_int;
}
extern "C" {
    pub fn kdbgetsymval(: *const c_char, : *mut kdb_symtab_t) -> c_int;
}
extern "C" {
    pub fn kdbnearsym(long: unsigned, : *mut kdb_symtab_t) -> c_int;
}
extern "C" {
    pub fn kdb_symbol_print(long: unsigned, : *const kdb_symtab_t, int: unsigned);
}
// Routine for debugging the debugger state.
extern "C" {
    pub fn kdb_print_state(: *const c_char, _arg: c_int);
}
pub const KDB_STATE_KDB: c_uint = 0x00000001	/* Cpu is inside kdb */;
pub const KDB_STATE_LEAVING: c_uint = 0x00000002	/* Cpu is leaving kdb */;
pub const KDB_STATE_CMD: c_uint = 0x00000004	/* Running a kdb command */;
pub const KDB_STATE_KDB_CONTROL: c_uint = 0x00000008	// This cpu is under;
// kdb control
pub const KDB_STATE_HOLD_CPU: c_uint = 0x00000010	/* Hold this cpu inside kdb */;
pub const KDB_STATE_DOING_SS: c_uint = 0x00000020	/* Doing ss command */;
pub const KDB_STATE_SSBPT: c_uint = 0x00000080	// Install breakpoint;
// after one ss, independent of
// DOING_SS
pub const KDB_STATE_REENTRY: c_uint = 0x00000100	/* Valid re-entry into kdb */;
pub const KDB_STATE_SUPPRESS: c_uint = 0x00000200	/* Suppress error messages */;
pub const KDB_STATE_PAGER: c_uint = 0x00000400	/* pager is available */;
pub const KDB_STATE_GO_SWITCH: c_uint = 0x00000800	// go is switching;
// back to initial cpu
pub const KDB_STATE_WAIT_IPI: c_uint = 0x00002000	/* Waiting for kdb_ipi() NMI */;
pub const KDB_STATE_RECURSE: c_uint = 0x00004000	/* Recursive entry to kdb */;
pub const KDB_STATE_IP_ADJUSTED: c_uint = 0x00008000	// Restart IP has been;
// adjusted
pub const KDB_STATE_GO1: c_uint = 0x00010000	/* go only releases one cpu */;
pub const KDB_STATE_KEYBOARD: c_uint = 0x00020000	// kdb entered via;
// keyboard on this cpu
pub const KDB_STATE_KEXEC: c_uint = 0x00040000	/* kexec issued */;
pub const KDB_STATE_DOING_KGDB: c_uint = 0x00080000	/* kgdb enter now issued */;
pub const KDB_STATE_KGDB_TRANS: c_uint = 0x00200000	/* Transition to kgdb */;
pub const KDB_STATE_ARCH: c_uint = 0xff000000	// Reserved for arch;
// specific use

extern "C" {
    pub fn kdb_register_table(kp: *mut kdbtab_t, len: usize);
}
// KDB breakpoint management functions
extern "C" {
    pub fn kdb_initbptab();
}
extern "C" {
    pub fn kdb_bp_install(: *mut pt_regs);
}
extern "C" {
    pub fn kdb_bp_remove();
}
// Miscellaneous functions and data areas
pub const KDB_GREPPING_FLAG_SEARCH: c_uint = 0x8000;
pub const KDB_GREP_STRLEN: c_int = 256;
extern "C" {
    pub fn kdb_task_state_char(: *const task_struct) -> c_char;
}
extern "C" {
    pub fn kdb_task_state(p: *const task_struct, mask: *const c_char) -> bool;
}
extern "C" {
    pub fn kdb_ps_suppressed();
}
extern "C" {
    pub fn kdb_ps1(p: *const task_struct);
}
extern "C" {
    pub fn kdb_getchar() -> c_char;
}
extern "C" {
    pub fn kdb_gdb_state_pass(buf: *mut c_char);
}
// Defines for kdb_symbol_print
pub const KDB_SP_SPACEB: c_uint = 0x0001		/* Space before string */;
pub const KDB_SP_SPACEA: c_uint = 0x0002		/* Space after string */;
pub const KDB_SP_PAREN: c_uint = 0x0004		/* Parenthesis around string */;
pub const KDB_SP_VALUE: c_uint = 0x0008		/* Print the value of the address */;
pub const KDB_SP_SYMSIZE: c_uint = 0x0010		/* Print the size of the symbol */;
pub const KDB_SP_NEWLINE: c_uint = 0x0020		/* Newline after string */;

extern "C" {
    pub fn kdb_kbd_cleanup_state();
}

// Macro flag: #define kdb_kbd_cleanup_state()