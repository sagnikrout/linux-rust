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
    AUDIT_STATE_DISABLED,	/* Do not create per-task audit_context.
// No syscall-specific audit records can
// be generated.
    AUDIT_STATE_BUILD,	/* Create the per-task audit_context,
// and fill it in at syscall
// entry time.  This makes a full
// syscall record available if some
// other part of the kernel decides it
// should be recorded.
    AUDIT_STATE_RECORD	/* Create the per-task audit_context,
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
    pub /: *mut *mut unsigned int fE; / effective bit of file cap,
    pub /: *mut *mut kernel_cap_t effective; / effective set of process,
}

// When fs/namei.c:getname() is called, we store the pointer in name and bump
// the refcnt in the associated filename struct.
//
// Further, in fs/namei.c:path_lookup() we store the inode and device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct audit_names {
    pub /: *mut *mut list_head list; / audit_context->names_list,
    pub name: *mut filename,
    pub /: *mut *mut int name_len; / number of chars to log,
    pub /: *mut *mut bool hidden; / don't log this record,
    pub ino: u64,
    pub dev: dev_t,
    pub mode: umode_t,
    pub uid: kuid_t,
    pub gid: kgid_t,
    pub rdev: dev_t,
    pub oprop: lsm_prop,
    pub fcap: audit_cap_data,
    pub fcap_ver: c_uint,
    pub /: *mut *mut unsigned char type; / record type,
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
    pub /: *mut *mut int len; / length of the cmdline field.,
    pub /: *mut *mut *mut char value; / the cmdline field,
}

// A timestamp/serial pair to identify an event
#[repr(C)]
#[derive(Copy, Clone)]
pub struct audit_stamp {
    pub /: *mut *mut timespec64 ctime; / time of syscall entry,
    pub /: *mut *mut unsigned int serial; / serial number for record,
}

// The per-task audit context.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct audit_context {
    pub /: *mut *mut int dummy; / must be the first element,
    pub context: },
    pub current_state: audit_state state,,
    pub /: *mut *mut audit_stamp stamp; / event identifier,
    pub /: *mut *mut int major; / syscall number,
    pub /: *mut *mut int uring_op; / uring operation,
    pub /: *mut *mut unsigned long argv[4]; / syscall arguments,
    pub /: *mut *mut long return_code;/ syscall return code,
    pub prio: u64,
    pub /: *mut *mut int return_valid; / return code is valid,
//
// The names_list is the list of all audit_names collected during this
// syscall.  The first AUDIT_NAMES entries in the names_list will
// actually be from the preallocated_names array for performance
// reasons.  Except during allocation they should never be referenced
// through the preallocated_names array and should only be found/used
// by running the names_list.
//
    pub preallocated_names: [audit_names; AUDIT_NAMES],
    pub /: *mut *mut int name_count; / total records in names_list,
    pub /: *mut *mut list_head names_list; / audit_names->list anchor,
    pub /: *mut *mut *mut char filterkey; / key for rule that triggered record,
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
}
