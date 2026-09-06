//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/audit.h
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
pub struct ctl_table_header { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_root { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_set { pub _opaque: [u8; 0] }

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
macro_rules! DEFINE { ($($tt:tt)*) => {}; }

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
pub struct proc_dir_entry { pub _opaque: [u8; 0] }

pub type pid_type = c_int;
pub type cpu_pm_event = c_int;
pub type spinlock_t = u32;
pub type raw_spinlock_t = u32;
pub type kernel_cap_t = u64;
pub type cap_user_header_t = *mut c_void;
pub type cap_user_data_t = *mut c_void;
pub type async_cookie_t = u64;
pub type atomic_long_t = core::sync::atomic::AtomicI64;


























// SPDX-License-Identifier: GPL-2.0-or-later
// audit -- definition of audit_context structure and supporting types
//
// Copyright 2003-2004 Red Hat, Inc.
// Copyright 2005 Hewlett-Packard Development Company, L.P.
// Copyright 2005 IBM Corporation
//

// AUDIT_NAMES is the number of slots we reserve in the audit_context
// for saving names from getname().  If we get more names we will allocate
// a name dynamically and also add those to the list anchored by names_list.
pub const AUDIT_NAMES: c_int = 5;
// At task start time, the audit_state is set in the audit_context using
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum audit_state {
    AUDIT_STATE_DISABLED,	// Do not create per-task audit_context.
// No syscall-specific audit records can
// be generated.
    AUDIT_STATE_BUILD,	// Create the per-task audit_context,
// and fill it in at syscall
// entry time.  This makes a full
// syscall record available if some
// other part of the kernel decides it
// should be recorded.
    AUDIT_STATE_RECORD	// Create the per-task audit_context,
// always fill it in at syscall entry
// time, and always write out the audit
// record at syscall exit time.
}

// Rule lists
#[repr(C)]
#[derive(Copy, Clone)]
pub struct audit_entry {
    pub list: list_head,
    pub rcu: rcu_head,
    pub rule: audit_krule,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct audit_cap_data {
    pub permitted: kernel_cap_t,
    pub inheritable: kernel_cap_t,
//     pub /: *mut *mut unsigned int fE; / effective bit of file cap,
//     pub /: *mut *mut kernel_cap_t effective; / effective set of process,
}

// When fs/namei.c:getname() is called, we store the pointer in name and bump
// the refcnt in the associated filename struct.
//
// Further, in fs/namei.c:path_lookup() we store the inode and device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct audit_names {
//     pub /: *mut *mut list_head list; / audit_context->names_list,
    pub name: *mut filename,
//     pub /: *mut *mut int name_len; / number of chars to log,
//     pub /: *mut *mut bool hidden; / don't log this record,
    pub ino: u64,
    pub dev: dev_t,
    pub mode: umode_t,
    pub uid: kuid_t,
    pub gid: kgid_t,
    pub rdev: dev_t,
    pub oprop: lsm_prop,
    pub fcap: audit_cap_data,
    pub fcap_ver: c_uint,
//     pub /: *mut *mut unsigned char type; / record type,
//
// This was an allocated audit_names and not from the array of
// names allocated in the task audit context.  Thus this name
// should be freed on syscall exit.
//
    pub should_free: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct audit_proctitle {
//     pub /: *mut *mut int len; / length of the cmdline field.,
//     pub /: *mut *mut *mut char value; / the cmdline field,
}

// A timestamp/serial pair to identify an event
#[repr(C)]
#[derive(Copy, Clone)]
pub struct audit_stamp {
//     pub /: *mut *mut timespec64 ctime; / time of syscall entry,
//     pub /: *mut *mut unsigned int serial; / serial number for record,
}

// The per-task audit context.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct audit_context {
//     pub /: *mut *mut int dummy; / must be the first element,
    pub context: },
    pub current_state: audit_state state,,
//     pub /: *mut *mut audit_stamp stamp; / event identifier,
//     pub /: *mut *mut int major; / syscall number,
//     pub /: *mut *mut int uring_op; / uring operation,
//     pub /: *mut *mut unsigned long argv[4]; / syscall arguments,
//     pub /: *mut *mut long return_code;/ syscall return code,
    pub prio: u64,
//     pub /: *mut *mut int return_valid; / return code is valid,
//
// The names_list is the list of all audit_names collected during this
// syscall.  The first AUDIT_NAMES entries in the names_list will
// actually be from the preallocated_names array for performance
// reasons.  Except during allocation they should never be referenced
// through the preallocated_names array and should only be found/used
// by running the names_list.
//
    pub preallocated_names: [audit_names; AUDIT_NAMES],
//     pub /: *mut *mut int name_count; / total records in names_list,
//     pub /: *mut *mut list_head names_list; / audit_names->list anchor,
//     pub /: *mut *mut *mut char filterkey; / key for rule that triggered record,
    pub pwd: path,
    pub aux: *mut audit_aux_data,
    pub aux_pids: *mut audit_aux_data,
    pub sockaddr: *mut sockaddr_storage,
    pub sockaddr_len: usize,
// Save things to print about task_struct
    pub ppid: pid_t,
    pub fsuid: kuid_t uid, euid, suid,,
    pub fsgid: kgid_t gid, egid, sgid,,
    pub personality: c_ulong,
    pub arch: c_int,
    pub target_pid: pid_t,
    pub target_auid: kuid_t,
    pub target_uid: kuid_t,
    pub target_sessionid: c_uint,
    pub target_ref: lsm_prop,
    pub target_comm: [c_char; TASK_COMM_LEN],
    pub first_trees: *mut *mut audit_tree_refs trees,,
    pub killed_trees: list_head,
    pub tree_count: c_int,
    pub type: c_int,
    pub nargs: c_int,
    pub args: [c_long; 6],
    pub socketcall: },
    pub uid: kuid_t,
    pub gid: kgid_t,
    pub mode: umode_t,
    pub oprop: lsm_prop,
    pub has_perm: c_int,
    pub perm_uid: uid_t,
    pub perm_gid: gid_t,
    pub perm_mode: umode_t,
    pub qbytes: c_ulong,
    pub ipc: },
    pub mqdes: mqd_t,
    pub mqstat: mq_attr,
    pub mq_getsetattr: },
    pub mqdes: mqd_t,
    pub sigev_signo: c_int,
    pub mq_notify: },
    pub mqdes: mqd_t,
    pub msg_len: usize,
    pub msg_prio: c_uint,
    pub abs_timeout: timespec64,
    pub mq_sendrecv: },
    pub oflag: c_int,
    pub mode: umode_t,
    pub attr: mq_attr,
    pub mq_open: },
    pub pid: pid_t,
    pub cap: audit_cap_data,
    pub capset: },
    pub fd: c_int,
    pub flags: c_int,
    pub mmap: },
    pub openat2: open_how,
    pub argc: c_int,
    pub execve: },
    pub name: *const c_char,
    pub module: },
    pub ntp_data: audit_ntp_data,
    pub tk_injoffset: timespec64,
    pub time: },
}

extern "C" {
    pub fn audit_log_session_info(ab: *mut audit_buffer);
}
extern "C" {
    pub fn auditd_test_task(task: *mut task_struct) -> c_int;
}
pub const AUDIT_INODE_BUCKETS: c_int = 32;
// Indicates that audit should log the full pathname.

extern "C" {
    pub fn audit_match_class(class: c_int, syscall: c_uint) -> c_int;
}
extern "C" {
    pub fn audit_comparator(left: u32, op: u32, right: u32) -> c_int;
}
extern "C" {
    pub fn audit_uid_comparator(left: kuid_t, op: u32, right: kuid_t) -> c_int;
}
extern "C" {
    pub fn audit_gid_comparator(left: kgid_t, op: u32, right: kgid_t) -> c_int;
}
extern "C" {
    pub fn parent_len(path: *const c_char) -> c_int;
}
extern "C" {
    pub fn audit_compare_dname_path(dname: *const qstr, path: *const c_char, plen: c_int) -> c_int;
}
extern "C" {
    pub fn audit_panic(message: *const c_char);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct audit_netlink_list {
    pub portid: __u32,
    pub net: *mut net,
    pub q: sk_buff_head,
}

extern "C" {
    pub fn audit_send_list_thread(_dest: *mut c_void) -> c_int;
}
extern "C" {
    pub fn audit_del_rule(entry: *mut audit_entry) -> c_int;
}
extern "C" {
    pub fn audit_free_rule_rcu(head: *mut rcu_head);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct audit_watch_ctx {
    pub dir: *mut inode,
    pub child: *mut inode,
}

extern "C" {
    pub fn audit_put_tty(tty: *mut tty_struct);
}
// audit watch/mark/tree functions
extern "C" {
    pub fn audit_serial() -> c_uint;
}

extern "C" {
    pub fn audit_put_watch(watch: *mut audit_watch);
}
extern "C" {
    pub fn audit_get_watch(watch: *mut audit_watch);
}
extern "C" {
    pub fn audit_add_watch(krule: *mut audit_krule, list: *mut list_head) -> c_int;
}
extern "C" {
    pub fn audit_remove_watch_rule(krule: *mut audit_krule);
}
extern "C" {
    pub fn audit_watch_compare(watch: *mut audit_watch, ino: u64, dev: dev_t) -> c_int;
}
extern "C" {
    pub fn audit_remove_mark(audit_mark: *mut audit_fsnotify_mark);
}
extern "C" {
    pub fn audit_remove_mark_rule(krule: *mut audit_krule);
}
extern "C" {
    pub fn audit_put_chunk(chunk: *mut audit_chunk);
}
extern "C" {
    pub fn audit_make_tree(rule: *mut audit_krule, pathname: *mut c_char, op: u32) -> c_int;
}
extern "C" {
    pub fn audit_add_tree_rule(rule: *mut audit_krule) -> c_int;
}
extern "C" {
    pub fn audit_remove_tree_rule(rule: *mut audit_krule) -> c_int;
}
extern "C" {
    pub fn audit_trim_trees();
}
extern "C" {
    pub fn audit_tag_tree(old: *mut c_char, new: *mut c_char) -> c_int;
}
extern "C" {
    pub fn audit_put_tree(tree: *mut audit_tree);
}
extern "C" {
    pub fn audit_kill_trees(context: *mut audit_context);
}
extern "C" {
    pub fn audit_signal_info_syscall(t: *mut task_struct) -> c_int;
}

extern "C" {
    pub fn audit_filter(msgtype: c_int, listtype: c_uint) -> c_int;
}
extern "C" {
    pub fn audit_ctl_lock();
}
extern "C" {
    pub fn audit_ctl_unlock();