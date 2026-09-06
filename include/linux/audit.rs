//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/audit.h
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
// audit.h -- Auditing support
//
// Copyright 2003-2004 Red Hat Inc., Durham, North Carolina.
// All Rights Reserved.
//
// Written by Rickard E. (Rik) Faith <faith@redhat.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct audit_sig_info {
    pub uid: uid_t,
    pub pid: pid_t,
    pub ctx: [c_char; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct audit_krule {
    pub pflags: u32,
    pub flags: u32,
    pub listnr: u32,
    pub action: u32,
    pub mask: [u32; AUDIT_BITMASK_SIZE],
    pub /: *mut *mut u32 buflen; / for data alloc on list rules,
    pub field_count: u32,
    pub /: *mut *mut *mut char filterkey; / ties events to rules,
    pub fields: *mut audit_field,
    pub /: *mut *mut *mut audit_field arch_f; / quick access to arch field,
    pub /: *mut *mut *mut audit_field inode_f; / quick access to an inode field,
    pub /: *mut *mut *mut audit_watch watch; / associated watch,
    pub /: *mut *mut *mut audit_tree tree; / associated watched tree,
    pub exe: *mut audit_fsnotify_mark,
    pub /: *mut *mut list_head rlist; / entry in audit_{watch,tree}.rules list,
    pub /: *mut *mut *mut list_head list; / for AUDIT_LIST purposes only,
    pub prio: u64,
}

// Flag to indicate legacy AUDIT_LOGINUID unset usage
pub const AUDIT_LOGINUID_LEGACY: c_uint = 0x1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct audit_field {
    pub type: u32,
    pub val: u32,
    pub uid: kuid_t,
    pub gid: kgid_t,
    pub lsm_str: *mut c_char,
    pub lsm_rule: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum audit_ntp_type {
    AUDIT_NTP_OFFSET,
    AUDIT_NTP_FREQ,
    AUDIT_NTP_STATUS,
    AUDIT_NTP_TAI,
    AUDIT_NTP_TICK,
    AUDIT_NTP_ADJUST,

    AUDIT_NTP_NVALS /* count */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct audit_ntp_val {
    pub newval: long long oldval,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct audit_ntp_data {
    pub vals: [audit_ntp_val; AUDIT_NTP_NVALS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct audit_ntp_data {

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum audit_nfcfgop {
    AUDIT_XT_OP_REGISTER,
    AUDIT_XT_OP_REPLACE,
    AUDIT_XT_OP_UNREGISTER,
    AUDIT_NFT_OP_TABLE_REGISTER,
    AUDIT_NFT_OP_TABLE_UNREGISTER,
    AUDIT_NFT_OP_CHAIN_REGISTER,
    AUDIT_NFT_OP_CHAIN_UNREGISTER,
    AUDIT_NFT_OP_RULE_REGISTER,
    AUDIT_NFT_OP_RULE_UNREGISTER,
    AUDIT_NFT_OP_SET_REGISTER,
    AUDIT_NFT_OP_SET_UNREGISTER,
    AUDIT_NFT_OP_SETELEM_REGISTER,
    AUDIT_NFT_OP_SETELEM_UNREGISTER,
    AUDIT_NFT_OP_GEN_REGISTER,
    AUDIT_NFT_OP_OBJ_REGISTER,
    AUDIT_NFT_OP_OBJ_UNREGISTER,
    AUDIT_NFT_OP_OBJ_RESET,
    AUDIT_NFT_OP_FLOWTABLE_REGISTER,
    AUDIT_NFT_OP_FLOWTABLE_UNREGISTER,
    AUDIT_NFT_OP_SETELEM_RESET,
    AUDIT_NFT_OP_RULE_RESET,
    AUDIT_NFT_OP_INVALID,
}

    pub list): *mut extern int __init audit_register_class(int class, unsigned int,
    pub syscall): extern int audit_classify_syscall(int abi, unsigned int,
    pub arch): extern int audit_classify_arch(int,
// audit_names->type values

// maximized args number that audit_socketcall can process
pub const AUDITSC_ARGS: c_int = 6;
// bit values for ->signal->audit_tty

// bit values for audit_cfg_lsm

    pub filename: struct,
pub const AUDIT_OFF: c_int = 0;
pub const AUDIT_ON: c_int = 1;
pub const AUDIT_LOCKED: c_int = 2;

// These are defined in audit.c
// Public API
    pub ...): *const *const char fmt,,
    pub type): *mut *mut *mut extern struct audit_buffer audit_log_start(struct audit_context ctx, gfp_t gfp_mask, int,
    pub ...): *const *const *const void audit_log_format(struct audit_buffer ab, char fmt,,
    pub ab): *mut extern void audit_log_end(struct audit_buffer,
    pub len): usize,
    pub len): usize,
    pub n): usize,
    pub n): usize,
    pub string): *const c_char,
    pub path): *const path,
    pub key): *mut c_char,
    pub operation): *const c_char,
    pub message): *const extern void audit_log_lost(char,
    pub prop): *mut *mut extern int audit_log_subj_ctx(struct audit_buffer ab, struct lsm_prop,
    pub prop): *mut *mut extern int audit_log_obj_ctx(struct audit_buffer ab, struct lsm_prop,
    pub ab): *mut extern int audit_log_task_context(struct audit_buffer,
    pub ab): *mut extern void audit_log_task_info(struct audit_buffer,
    pub nfproto): *const *const sk_buff skb, u8,
    pub audit_update_lsm_rules(void): extern int,
// Private API (for audit.c only)
    pub datasz): *mut *mut extern int audit_rule_change(int type, int seq, void data, size_t,
    pub seq): *mut *mut extern int audit_list_rules_send(struct sk_buff request_skb, int,
    pub loginuid): extern int audit_set_loginuid(kuid_t,
    pub tsk->loginuid: return,
    pub tsk->sessionid: return,
    pub audit_enabled: extern u32,
    pub t): *mut extern int audit_signal_info(int sig, struct task_struct,
    pub flags): *const *const extern void audit_cfg_lsm(struct lsm_id lsmid, int,

    pub NULL: return,
    pub 0: return,
    pub 0: return,
    pub 0: return,
    pub 0: return,
    pub INVALID_UID: return,
    pub AUDIT_SID_UNSET: return,

    pub 0: return,

// These are defined in auditsc.c
// Public API
    pub task): *mut extern int audit_alloc(struct task_struct,
    pub task): *mut extern void __audit_free(struct task_struct,
    pub op): extern void __audit_uring_entry(u8,
    pub code): extern void __audit_uring_exit(int success, long,
    pub a3): unsigned long a2, unsigned long,
    pub ret_value): extern void __audit_syscall_exit(int ret_success, long,
    pub name): *mut extern void __audit_getname(struct filename,
    pub flags): c_uint,
    pub ): *const extern void __audit_file(struct file,
    pub type): c_uchar,
    pub code): extern void audit_seccomp(unsigned long syscall, long signr, int,
    pub res): *const *const char old_names, int,
    pub t): *mut extern void __audit_ptrace(struct task_struct,
    pub ctx: task->audit_context =,
    pub current->audit_context: return,
    pub audit_context(): *mut *mut void p =,
    pub )p: *mut *mut return !p || (int,
//
// We intentionally check audit_context() before audit_enabled as most
// Linux systems (as of ~2021) rely on systemd which forces audit to
// be enabled regardless of the user's audit configuration.
//
    pub code): __audit_uring_exit(success,,
    pub a3): __audit_syscall_entry(major, a0, a1, a2,,
    pub is_syscall_success(pt_regs): int success =,
    pub regs_return_value(pt_regs): long return_code =,
    pub return_code): __audit_syscall_exit(success,,
    pub aflags): __audit_inode(name, dentry,,
    pub AUDIT_INODE_HIDDEN): AUDIT_INODE_PARENT |,
    pub type): __audit_inode_child(parent, dentry,,
    pub signr): void audit_core_dumps(long,
// Private API (for audit.c only)
    pub ipcp): *mut extern void __audit_ipc_obj(struct kern_ipc_perm,
    pub mode): extern void __audit_ipc_set_perm(unsigned long qbytes, uid_t uid, gid_t gid, umode_t,
    pub bprm): *mut extern void __audit_bprm(struct linux_binprm,
    pub args): *mut extern int __audit_socketcall(int nargs, unsigned long,
    pub addr): *mut extern int __audit_sockaddr(int len, void,
    pub fd2): extern void __audit_fd_pair(int fd1, int,
    pub attr): *mut extern void __audit_mq_open(int oflag, umode_t mode, struct mq_attr,
    pub abs_timeout): *const extern void __audit_mq_sendrecv(mqd_t mqdes, size_t msg_len, unsigned int msg_prio, struct timespec64,
    pub notification): *const extern void __audit_mq_notify(mqd_t mqdes, struct sigevent,
    pub mqstat): *mut extern void __audit_mq_getsetattr(mqd_t mqdes, struct mq_attr,
    pub old): *const cred,
    pub old): *const *const extern void __audit_log_capset(struct cred new, struct cred,
    pub flags): extern void __audit_mmap_fd(int fd, int,
    pub how): *mut extern void __audit_openat2_how(struct open_how,
    pub name): *const extern void __audit_log_kern_module(char,
    pub friar): *mut extern void __audit_fanotify(u32 response, struct fanotify_response_info_audit_rule,
    pub offset): extern void __audit_tk_injoffset(struct timespec64,
    pub ad): *const extern void __audit_ntp_log(struct audit_ntp_data,
    pub gfp): audit_nfcfgop op, gfp_t,
    pub fd2): __audit_fd_pair(fd1,,
    pub mode): __audit_ipc_set_perm(qbytes, uid, gid,,
    pub args): return __audit_socketcall(nargs,,
    pub 0: return,
    pub a: [c_ulong; AUDITSC_ARGS],
    pub i: c_int,
    pub 0: return,
    pub i++): for (i = 0; i < nargs;,
    pub long)args[i]: a[i] = (unsigned,
    pub a): return __audit_socketcall(nargs,,
    pub addr): return __audit_sockaddr(len,,
    pub 0: return,
    pub attr): __audit_mq_open(oflag, mode,,
    pub abs_timeout): __audit_mq_sendrecv(mqdes, msg_len, msg_prio,,
    pub notification): __audit_mq_notify(mqdes,,
    pub mqstat): __audit_mq_getsetattr(mqdes,,
    pub old): return __audit_log_bprm_fcaps(bprm, new,,
    pub 0: return,
    pub old): __audit_log_capset(new,,
    pub flags): __audit_mmap_fd(fd,,
    pub friar): __audit_fanotify(response,,
// ignore no-op events
    pub sizeof(*ad)): *mut memset(ad, 0,,
    pub val: ad->vals[type].oldval =,
    pub val: ad->vals[type].newval =,
    pub gfp): __audit_log_nfcfg(name, af, nentries, op,,
    pub audit_n_rules: extern int,
    pub audit_signals: extern int,

    pub 0: return,
    pub true: return,
    pub NULL: return,
    pub 0: return,
    pub 0: return,
    pub 0: return,
    pub 0: return,
pub const audit_n_rules: c_int = 0;
pub const audit_signals: c_int = 0;

    pub uid_valid(audit_get_loginuid(tsk)): return,
