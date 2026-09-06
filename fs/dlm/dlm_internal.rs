//! Automatically rewritten from C Header to Rust Module
//! Source: fs/dlm/dlm_internal.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) Sistina Software, Inc.  1997-2003  All rights reserved.
// Copyright (C) 2004-2011 Red Hat, Inc.  All rights reserved.
//
// This is the main header file to be included in each DLM source file.
//

//
// Lockspace member (per node in a ls)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dlm_member {
    pub list: list_head,
    pub nodeid: c_int,
    pub weight: c_int,
    pub slot: c_int,
    pub slot_prev: c_int,
    pub comm_seq: c_int,
    pub generation: u32,
}

//
// Save and manage recovery state for a lockspace.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dlm_recover {
    pub list: list_head,
    pub nodes: *mut dlm_config_node,
    pub nodes_count: c_int,
    pub seq: u64,
}

//
// Pass input args to second stage locking function.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dlm_args {
    pub flags: u32,
    pub astparam): *mut *mut void (astfn) (void,
    pub astparam: *mut c_void,
    pub mode): *mut *mut *mut void (bastfn) (void astparam, int,
    pub mode: c_int,
    pub lksb: *mut dlm_lksb,
}

//
// Lock block
//
// A lock can be one of three types:
//
// local copy      lock is mastered locally
// (lkb_nodeid is zero and DLM_LKF_MSTCPY is not set)
// process copy    lock is mastered on a remote node
// (lkb_nodeid is non-zero and DLM_LKF_MSTCPY is not set)
// master copy     master node's copy of a lock owned by remote node
// (lkb_nodeid is non-zero and DLM_LKF_MSTCPY is set)
//
// lkb_exflags: a copy of the most recent flags arg provided to dlm_lock or
// dlm_unlock.  The dlm does not modify these or use any private flags in
// this field; it only contains DLM_LKF_ flags from dlm.h.  These flags
// are sent as-is to the remote master when the lock is remote.
//
// lkb_flags: internal dlm flags (DLM_IFL_ prefix) from dlm_internal.h.
// Some internal flags are shared between the master and process nodes;
// these shared flags are kept in the lower two bytes.  One of these
// flags set on the master copy will be propagated to the process copy
// and v.v.  Other internal flags are private to the master or process
// node (e.g. DLM_IFL_MSTCPY).  These are kept in the high two bytes.
//
// lkb_sbflags: status block flags.  These flags are copied directly into
// the caller's lksb.sb_flags prior to the dlm_lock/dlm_unlock completion
// ast.  All defined in dlm.h with DLM_SBF_ prefix.
//
// lkb_status: the lock status indicates which rsb queue the lock is
// on, grant, convert, or wait.  DLM_LKSTS_ WAITING/GRANTED/CONVERT
//
// lkb_wait_type: the dlm message type (DLM_MSG_ prefix) for which a
// reply is needed.  Only set when the lkb is on the lockspace waiters
// list awaiting a reply from a remote node.
//
// lkb_nodeid: when the lkb is a local copy, nodeid is 0; when the lkb
// is a master copy, nodeid specifies the remote lock holder, when the
// lkb is a process copy, the nodeid specifies the lock master.
//
// lkb_status
pub const DLM_LKSTS_WAITING: c_int = 1;
pub const DLM_LKSTS_GRANTED: c_int = 2;
pub const DLM_LKSTS_CONVERT: c_int = 3;
// lkb_iflags
pub const DLM_IFL_MSTCPY_BIT: c_int = 16;

pub const DLM_IFL_RESEND_BIT: c_int = 17;
pub const DLM_IFL_DEAD_BIT: c_int = 18;
pub const DLM_IFL_OVERLAP_UNLOCK_BIT: c_int = 19;
pub const DLM_IFL_OVERLAP_CANCEL_BIT: c_int = 20;
pub const DLM_IFL_ENDOFLIFE_BIT: c_int = 21;
pub const DLM_IFL_DEADLOCK_CANCEL_BIT: c_int = 24;

// lkb_dflags
pub const DLM_DFL_USER_BIT: c_int = 0;

pub const DLM_DFL_ORPHAN_BIT: c_int = 1;

pub const DLM_CB_CAST: c_uint = 0x00000001;
pub const DLM_CB_BAST: c_uint = 0x00000002;
// much of this is just saving user space pointers associated with the
// lock that we pass back to the user lib with an ast
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dlm_user_args {
    pub lockspace: *mut *mut *mut dlm_user_proc proc; / each process that opens the,
// device has private data
// (dlm_user_proc) on the struct file,
// the process's locks point back to it
//
    pub lksb: dlm_lksb,
    pub user_lksb: *mut dlm_lksb __user,
    pub castparam: *mut void __user,
    pub castaddr: *mut void __user,
    pub bastparam: *mut void __user,
    pub bastaddr: *mut void __user,
    pub xid: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dlm_callback {
    pub /: *mut *mut uint32_t flags; / DLM_CBF_,
    pub /: *mut *mut int sb_status; / copy to lksb status,
    pub /: *mut *mut uint8_t sb_flags; / copy to lksb flags,
    pub /: *mut *mut int8_t mode; / rq mode of bast, gr mode of cast,
    pub copy_lvb: bool,
    pub lkb_lksb: *mut dlm_lksb,
    pub lvbptr: [c_uchar; DLM_USER_LVB_LEN],
    pub /: *mut *mut *mut void astparam; / caller's ast arg,
    pub ua: dlm_user_args,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dlm_lkb {
    pub /: *mut *mut *mut dlm_rsb lkb_resource; / the rsb,
    pub lkb_ref: kref,
    pub /: *mut *mut int lkb_nodeid; / copied from rsb,
    pub /: *mut *mut int lkb_ownpid; / pid of lock owner,
    pub /: *mut *mut uint32_t lkb_id; / our lock ID,
    pub /: *mut *mut uint32_t lkb_remid; / lock ID on remote partner,
    pub /: *mut *mut uint32_t lkb_exflags; / external flags from caller,
    pub /: *mut *mut unsigned long lkb_sbflags; / lksb flags,
    pub /: *mut *mut unsigned long lkb_dflags; / distributed flags,
    pub /: *mut *mut unsigned long lkb_iflags; / internal flags,
    pub /: *mut *mut uint32_t lkb_lvbseq; / lvb sequence number,
    pub /: *mut *mut int8_t lkb_status; / granted, waiting, convert,
    pub /: *mut *mut int8_t lkb_rqmode; / requested lock mode,
    pub /: *mut *mut int8_t lkb_grmode; / granted lock mode,
    pub /: *mut *mut int8_t lkb_highbast; / highest mode bast sent for,
    pub /: *mut *mut int8_t lkb_wait_type; / type of reply waiting for,
    pub lkb_wait_count: i8,
    pub /: *mut *mut int lkb_wait_nodeid; / for debugging,
    pub /: *mut *mut list_head lkb_statequeue; / rsb g/c/w list,
    pub /: *mut *mut list_head lkb_rsb_lookup; / waiting for rsb lookup,
    pub /: *mut *mut list_head lkb_wait_reply; / waiting for remote reply,
    pub /: *mut *mut list_head lkb_ownqueue; / list of locks for a process,
    pub lkb_timestamp: ktime_t,
    pub lkb_last_cast_cb_mode: i8,
    pub lkb_last_bast_cb_mode: i8,
    pub lkb_last_cb_mode: i8,
    pub lkb_last_cb_flags: u8,
    pub /: *mut *mut ktime_t lkb_last_cast_time; / for debugging,
    pub /: *mut *mut ktime_t lkb_last_bast_time; / for debugging,
    pub /: *mut *mut uint64_t lkb_recover_seq; / from ls_recover_seq,
    pub lkb_lvbptr: *mut c_char,
    pub /: *mut *mut *mut dlm_lksb lkb_lksb; / caller's status block,
    pub astparam): *mut *mut void (lkb_astfn) (void,
    pub mode): *mut *mut *mut void (lkb_bastfn) (void astparam, int,
    pub /: *mut *mut *mut void lkb_astparam; / caller's ast arg,
    pub lkb_ua: *mut dlm_user_args,
}

//
// res_master_nodeid is "normal": 0 is unset/invalid, non-zero is the real
// nodeid, even when nodeid is our_nodeid.
//
// res_nodeid is "odd": -1 is unset/invalid, zero means our_nodeid,
// greater than zero when another nodeid.
//
// (TODO: remove res_nodeid and only use res_master_nodeid)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dlm_rsb {
    pub /: *mut *mut *mut dlm_ls res_ls; / the lockspace,
    pub res_ref: kref,
    pub res_lock: spinlock_t,
    pub res_flags: c_ulong,
    pub /: *mut *mut int res_length; / length of rsb name,
    pub res_nodeid: c_int,
    pub res_master_nodeid: c_int,
    pub res_dir_nodeid: c_int,
    pub /: *mut *mut unsigned long res_id; / for ls_recover_xa,
    pub res_lvbseq: u32,
    pub res_hash: u32,
    pub res_toss_time: c_ulong,
    pub res_first_lkid: u32,
    pub /: *mut *mut list_head res_lookup; / lkbs waiting on first,
    pub /: *mut *mut rhash_head res_node; / rsbtbl,
    pub res_grantqueue: list_head,
    pub res_convertqueue: list_head,
    pub res_waitqueue: list_head,
    pub /: *mut *mut *mut list_head res_slow_list; / ls_slow_,
    pub res_scan_list: list_head,
    pub /: *mut *mut list_head res_root_list; / used for recovery,
    pub /: *mut *mut list_head res_masters_list; / used for recovery,
    pub /: *mut *mut list_head res_recover_list; / used for recovery,
    pub res_recover_locks_count: c_int,
    pub rcu: rcu_head,
    pub res_lvbptr: *mut c_char,
    pub res_name: [c_char; DLM_RESNAME_MAXLEN+1],
}

// dlm_master_lookup() flags
pub const DLM_LU_RECOVER_DIR: c_int = 1;
pub const DLM_LU_RECOVER_MASTER: c_int = 2;
// dlm_master_lookup() results
pub const DLM_LU_MATCH: c_int = 1;
pub const DLM_LU_ADD: c_int = 2;
// find_rsb() flags
pub const R_REQUEST: c_uint = 0x00000001;
pub const R_RECEIVE_REQUEST: c_uint = 0x00000002;
pub const R_RECEIVE_RECOVER: c_uint = 0x00000004;
// rsb_flags
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rsb_flags {
    RSB_MASTER_UNCERTAIN,
    RSB_VALNOTVALID,
    RSB_VALNOTVALID_PREV,
    RSB_NEW_MASTER,
    RSB_NEW_MASTER2,
    RSB_RECOVER_CONVERT,
    RSB_RECOVER_GRANT,
    RSB_RECOVER_LVB_INVAL,
    RSB_INACTIVE,
    RSB_HASHED, /* set while rsb is on ls_rsbtbl */
}

extern "C" {
    pub fn test_bit(_arg: flag, _arg: &r->res_flags) -> return;
}
// dlm_header is first element of all structs sent between nodes
pub const DLM_HEADER_MAJOR: c_uint = 0x00030000;
pub const DLM_HEADER_MINOR: c_uint = 0x00000002;
pub const DLM_VERSION_3_1: c_uint = 0x00030001;
pub const DLM_VERSION_3_2: c_uint = 0x00030002;
pub const DLM_HEADER_SLOTS: c_uint = 0x00000001;
pub const DLM_MSG: c_int = 1;
pub const DLM_RCOM: c_int = 2;
pub const DLM_OPTS: c_int = 3;
pub const DLM_ACK: c_int = 4;
pub const DLM_FIN: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dlm_header {
    pub h_version: __le32,
// for DLM_MSG and DLM_RCOM
    pub h_lockspace: __le32,
// for DLM_ACK and DLM_OPTS
    pub h_seq: __le32,
    pub u: },
    pub /: *mut *mut __le32 h_nodeid; / nodeid of sender,
    pub h_length: __le16,
    pub /: *mut *mut uint8_t h_cmd; / DLM_MSG, DLM_RCOM,
    pub h_pad: u8,
}

pub const DLM_MSG_REQUEST: c_int = 1;
pub const DLM_MSG_CONVERT: c_int = 2;
pub const DLM_MSG_UNLOCK: c_int = 3;
pub const DLM_MSG_CANCEL: c_int = 4;
pub const DLM_MSG_REQUEST_REPLY: c_int = 5;
pub const DLM_MSG_CONVERT_REPLY: c_int = 6;
pub const DLM_MSG_UNLOCK_REPLY: c_int = 7;
pub const DLM_MSG_CANCEL_REPLY: c_int = 8;
pub const DLM_MSG_GRANT: c_int = 9;
pub const DLM_MSG_BAST: c_int = 10;
pub const DLM_MSG_LOOKUP: c_int = 11;
pub const DLM_MSG_REMOVE: c_int = 12;
pub const DLM_MSG_LOOKUP_REPLY: c_int = 13;
pub const DLM_MSG_PURGE: c_int = 14;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dlm_message {
    pub m_header: dlm_header,
    pub /: *mut *mut __le32 m_type; / DLM_MSG_,
    pub m_nodeid: __le32,
    pub m_pid: __le32,
    pub /: *mut *mut __le32 m_lkid; / lkid on sender,
    pub /: *mut *mut __le32 m_remid; / lkid on receiver,
    pub m_parent_lkid: __le32,
    pub m_parent_remid: __le32,
    pub m_exflags: __le32,
    pub m_sbflags: __le32,
    pub m_flags: __le32,
    pub m_lvbseq: __le32,
    pub m_hash: __le32,
    pub m_status: __le32,
    pub m_grmode: __le32,
    pub m_rqmode: __le32,
    pub m_bastmode: __le32,
    pub m_asts: __le32,
    pub /: *mut *mut __le32 m_result; / 0 or -EXXX,
    pub /: *mut *mut char m_extra[]; / name or lvb,
}

pub const DLM_RS_NODES: c_uint = 0x00000001;
pub const DLM_RS_NODES_ALL: c_uint = 0x00000002;
pub const DLM_RS_DIR: c_uint = 0x00000004;
pub const DLM_RS_DIR_ALL: c_uint = 0x00000008;
pub const DLM_RS_LOCKS: c_uint = 0x00000010;
pub const DLM_RS_LOCKS_ALL: c_uint = 0x00000020;
pub const DLM_RS_DONE: c_uint = 0x00000040;
pub const DLM_RS_DONE_ALL: c_uint = 0x00000080;
pub const DLM_RCOM_STATUS: c_int = 1;
pub const DLM_RCOM_NAMES: c_int = 2;
pub const DLM_RCOM_LOOKUP: c_int = 3;
pub const DLM_RCOM_LOCK: c_int = 4;
pub const DLM_RCOM_STATUS_REPLY: c_int = 5;
pub const DLM_RCOM_NAMES_REPLY: c_int = 6;
pub const DLM_RCOM_LOOKUP_REPLY: c_int = 7;
pub const DLM_RCOM_LOCK_REPLY: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dlm_rcom {
    pub rc_header: dlm_header,
    pub /: *mut *mut __le32 rc_type; / DLM_RCOM_,
    pub /: *mut *mut __le32 rc_result; / multi-purpose,
    pub /: *mut *mut __le64 rc_id; / match reply with request,
    pub /: *mut *mut __le64 rc_seq; / sender's ls_recover_seq,
    pub /: *mut *mut __le64 rc_seq_reply; / remote ls_recover_seq,
    pub rc_buf: [c_char; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dlm_opt_header {
    pub t_type: __le16,
    pub t_length: __le16,
    pub t_pad: __le32,
// need to be 8 byte aligned
    pub t_value: [c_char; ],
}

// encapsulation header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dlm_opts {
    pub o_header: dlm_header,
    pub o_nextcmd: u8,
    pub o_pad: u8,
    pub o_optlen: __le16,
    pub o_pad2: __le32,
    pub o_opts: [c_char; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dlm_packet {
    pub /: *mut *mut dlm_header header; / common to other two,
    pub message: dlm_message,
    pub rcom: dlm_rcom,
    pub opts: dlm_opts,
}

pub const DLM_RSF_NEED_SLOTS: c_uint = 0x00000001;
// RCOM_STATUS data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcom_status {
    pub rs_flags: __le32,
    pub rs_unused1: __le32,
    pub rs_unused2: __le64,
}

// RCOM_STATUS_REPLY data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcom_config {
    pub rf_lvblen: __le32,
    pub rf_lsflags: __le32,
// DLM_HEADER_SLOTS adds:
    pub rf_flags: __le32,
    pub rf_our_slot: __le16,
    pub rf_num_slots: __le16,
    pub rf_generation: __le32,
    pub rf_unused1: __le32,
    pub rf_unused2: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcom_slot {
    pub ro_nodeid: __le32,
    pub ro_slot: __le16,
    pub ro_unused1: __le16,
    pub ro_unused2: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcom_lock {
    pub rl_ownpid: __le32,
    pub rl_lkid: __le32,
    pub rl_remid: __le32,
    pub rl_parent_lkid: __le32,
    pub rl_parent_remid: __le32,
    pub rl_exflags: __le32,
    pub rl_flags: __le32,
    pub rl_lvbseq: __le32,
    pub rl_result: __le32,
    pub rl_rqmode: i8,
    pub rl_grmode: i8,
    pub rl_status: i8,
    pub rl_asts: i8,
    pub rl_wait_type: __le16,
    pub rl_namelen: __le16,
    pub rl_name: [c_char; DLM_RESNAME_MAXLEN],
    pub rl_lvb: [c_char; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dlm_ls {
    pub /: *mut *mut list_head ls_list; / list of lockspaces,
    pub /: *mut *mut uint32_t ls_global_id; / global unique lockspace ID,
    pub ls_generation: u32,
    pub ls_exflags: u32,
    pub ls_lvblen: c_int,
    pub in: *mut *mut atomic_t ls_count; / refcount of processes,
    pub ls_count_wait: wait_queue_head_t,
    pub /: *mut *mut int ls_create_count; / create/release refcount,
    pub /: *mut *mut unsigned long ls_flags; / LSFL_,
    pub ls_kobj: kobject,
    pub ls_lkbxa: xarray,
    pub ls_lkbxa_lock: rwlock_t,
// an rsb is on rsbtl for primary locking functions,
    pub ls_rsbtbl: rhashtable,
    pub /: *mut *mut rwlock_t ls_rsbtbl_lock; / for ls_rsbtbl and ls_slow,
    pub /: *mut *mut list_head ls_slow_inactive; / to iterate rsbtbl,
    pub /: *mut *mut list_head ls_slow_active; / to iterate rsbtbl,
    pub /: *mut *mut timer_list ls_scan_timer; / based on first scan_list rsb toss_time,
    pub /: *mut *mut list_head ls_scan_list; / rsbs ordered by res_toss_time,
    pub ls_scan_lock: spinlock_t,
    pub ls_waiters_lock: spinlock_t,
    pub /: *mut *mut list_head ls_waiters; / lkbs needing a reply,
    pub ls_orphans_lock: spinlock_t,
    pub ls_orphans: list_head,
    pub /: *mut *mut list_head ls_nodes; / current nodes in ls,
    pub /: *mut *mut list_head ls_nodes_gone; / dead node list, recovery,
    pub /: *mut *mut int ls_num_nodes; / number of nodes in ls,
    pub ls_low_nodeid: c_int,
    pub ls_total_weight: c_int,
    pub ls_node_array: *mut c_int,
    pub ls_slot: c_int,
    pub ls_num_slots: c_int,
    pub ls_slots_size: c_int,
    pub ls_slots: *mut dlm_slot,
    pub /: *mut *mut dlm_rsb ls_local_rsb; / for returning errors,
    pub /: *mut *mut dlm_lkb ls_local_lkb; / for returning errors,
    pub /: *mut *mut *mut dentry ls_debug_rsb_dentry; / debugfs,
    pub /: *mut *mut *mut dentry ls_debug_waiters_dentry; / debugfs,
    pub /: *mut *mut *mut dentry ls_debug_locks_dentry; / debugfs,
    pub /: *mut *mut *mut dentry ls_debug_all_dentry; / debugfs,
    pub /: *mut *mut *mut dentry ls_debug_toss_dentry; / debugfs,
    pub /: *mut *mut *mut dentry ls_debug_queued_asts_dentry; / debugfs,
    pub /: *mut *mut wait_queue_head_t ls_uevent_wait; / user part of join/leave,
    pub ls_uevent_result: c_int,
    pub ls_recovery_done: completion,
    pub ls_recovery_result: c_int,
    pub ls_device: miscdevice,
    pub ls_callback_wq: *mut workqueue_struct,
// recovery related
    pub ls_cb_lock: spinlock_t,
    pub /: *mut *mut list_head ls_cb_delay; / save for queue_work later,
    pub ls_recoverd_task: *mut task_struct,
    pub ls_recoverd_active: mutex,
    pub ls_recover_lock: spinlock_t,
    pub /: *mut *mut unsigned long ls_recover_begin; / jiffies timestamp,
    pub /: *mut *mut uint32_t ls_recover_status; / DLM_RS_,
    pub ls_recover_seq: u64,
    pub ls_recover_args: *mut dlm_recover,
    pub /: *mut *mut rw_semaphore ls_in_recovery; / block local requests,
    pub /: *mut *mut rwlock_t ls_recv_active; / block dlm_recv,
    pub /: *mut *mut list_head ls_requestqueue;/ queue remote requests,
    pub ls_requestqueue_lock: rwlock_t,
    pub ls_recover_buf: *mut dlm_rcom,
    pub /: *mut *mut int ls_recover_nodeid; / for debugging,
    pub /: *mut *mut unsigned int ls_recover_locks_in; / for log info,
    pub ls_rcom_seq: u64,
    pub ls_rcom_spin: spinlock_t,
    pub ls_recover_list: list_head,
    pub ls_recover_list_lock: spinlock_t,
    pub ls_recover_list_count: c_int,
    pub ls_recover_xa: xarray,
    pub ls_recover_xa_lock: spinlock_t,
    pub ls_wait_general: wait_queue_head_t,
    pub ls_recover_lock_wait: wait_queue_head_t,
    pub ls_clear_proc_locks: spinlock_t,
    pub /: *mut *mut list_head ls_masters_list; / root resources,
    pub /: *mut *mut rwlock_t ls_masters_lock; / protect root_list,
    pub /: *mut *mut list_head ls_dir_dump_list; / root resources,
    pub /: *mut *mut rwlock_t ls_dir_dump_lock; / protect root_list,
    pub ls_ops: *const dlm_lockspace_ops,
    pub ls_ops_arg: *mut c_void,
    pub ls_free_work: work_struct,
    pub ls_namelen: c_int,
    pub 1]: char ls_name[DLM_LOCKSPACE_LEN +,
// Must be last --ends in a flexible-array member.
    pub /: *mut *mut dlm_message ls_local_ms; / for faking a reply,
}

//
// LSFL_RECOVER_STOP - dlm_ls_stop() sets this to tell dlm recovery routines
// that they should abort what they're doing so new recovery can be started.
//
// LSFL_RECOVER_DOWN - dlm_ls_stop() sets this to tell dlm_recoverd that it
// should do down_write() on the in_recovery rw_semaphore. (doing down_write
// within dlm_ls_stop causes complaints about the lock acquired/released
// in different contexts.)
//
// LSFL_RECOVER_LOCK - dlm_recoverd holds the in_recovery rw_semaphore.
// It sets this after it is done with down_write() on the in_recovery
// rw_semaphore and clears it after it has released the rw_semaphore.
//
// LSFL_RECOVER_WORK - dlm_ls_start() sets this to tell dlm_recoverd that it
// should begin recovery of the lockspace.
//
// LSFL_RUNNING - set when normal locking activity is enabled.
// dlm_ls_stop() clears this to tell dlm locking routines that they should
// quit what they are doing so recovery can run.  dlm_recoverd sets
// this after recovery is finished.
//
pub const LSFL_RECOVER_STOP: c_int = 0;
pub const LSFL_RECOVER_DOWN: c_int = 1;
pub const LSFL_RECOVER_LOCK: c_int = 2;
pub const LSFL_RECOVER_WORK: c_int = 3;
pub const LSFL_RUNNING: c_int = 4;
pub const LSFL_RCOM_READY: c_int = 5;
pub const LSFL_RCOM_WAIT: c_int = 6;
pub const LSFL_UEVENT_WAIT: c_int = 7;
pub const LSFL_CB_DELAY: c_int = 9;
pub const LSFL_NODIR: c_int = 10;
pub const LSFL_RECV_MSG_BLOCKED: c_int = 11;
pub const LSFL_FS: c_int = 12;
pub const LSFL_SOFTIRQ: c_int = 13;
pub const DLM_PROC_FLAGS_CLOSING: c_int = 1;
pub const DLM_PROC_FLAGS_COMPAT: c_int = 2;
// locks list is kept so we can remove all a process's locks when it
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dlm_user_proc {
    pub lockspace: *mut dlm_lockspace_t,
    pub /: *mut *mut unsigned long flags; / DLM_PROC_FLAGS,
    pub asts: list_head,
    pub asts_spin: spinlock_t,
    pub locks: list_head,
    pub locks_spin: spinlock_t,
    pub unlocking: list_head,
    pub wait: wait_queue_head_t,
}

extern "C" {
    pub fn test_bit(_arg: LSFL_RECOVER_STOP, _arg: &ls->ls_flags) -> return;
}
extern "C" {
    pub fn test_bit(_arg: LSFL_NODIR, _arg: &ls->ls_flags) -> return;
}
// takes a snapshot from dlm atomic flags
// coming from UAPI header
//
// TODO:
// Move this to UAPI header and let other values point to them and use BIT()
//
pub const DLM_SBF_DEMOTED_BIT: c_int = 0;

pub const DLM_SBF_VALNOTVALID_BIT: c_int = 1;
pub const DLM_SBF_ALTMODE_BIT: c_int = 2;

// be sure the next person updates this
extern "C" {
    pub fn dlm_plock_init() -> c_int;
}
extern "C" {
    pub fn dlm_plock_exit();
}

extern "C" {
    pub fn dlm_register_debugfs();
}
extern "C" {
    pub fn dlm_unregister_debugfs();
}
extern "C" {
    pub fn dlm_create_debug_file(ls: *mut dlm_ls);
}
extern "C" {
    pub fn dlm_delete_debug_file(ls: *mut dlm_ls);
}
extern "C" {
    pub fn dlm_delete_debug_comms_file(ctx: *mut c_void);
}

