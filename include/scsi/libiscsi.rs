//! Automatically rewritten from C Header to Rust Module
//! Source: include/scsi/libiscsi.h
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
//
// iSCSI lib definitions
//
// Copyright (C) 2006 Red Hat, Inc.  All rights reserved.
// Copyright (C) 2004 - 2006 Mike Christie
// Copyright (C) 2004 - 2005 Dmitry Yusupov
// Copyright (C) 2004 - 2005 Alex Aizman
//

pub const ISCSI_MGMT_CMDS_MAX: c_int = 15;
pub const ISCSI_DEF_CMD_PER_LUN: c_int = 32;
// Task Mgmt states
pub const ISID_SIZE: c_int = 6;
// Connection flags
pub const ISCSI_CONN_FLAG_SUSPEND_TX: c_int = 0;
pub const ISCSI_CONN_FLAG_SUSPEND_RX: c_int = 1;
pub const ISCSI_CONN_FLAG_BOUND: c_int = 2;
pub const ISCSI_ITT_MASK: c_uint = 0x1fff;
pub const ISCSI_TOTAL_CMDS_MAX: c_int = 4096;
// this must be a power of two greater than ISCSI_MGMT_CMDS_MAX
pub const ISCSI_TOTAL_CMDS_MIN: c_int = 16;
pub const ISCSI_AGE_SHIFT: c_int = 28;
pub const ISCSI_AGE_MASK: c_uint = 0xf;
pub const ISCSI_ADDRESS_BUF_LEN: c_int = 64;
// this is the maximum possible storage for AHSs
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_r2t_info {
    pub /: *mut *mut __be32 ttt; / copied from R2T,
    pub /: *mut *mut __be32 exp_statsn; / copied from R2T,
    pub /: *mut *mut uint32_t data_length; / copied from R2T,
    pub /: *mut *mut uint32_t data_offset; / copied from R2T,
    pub /: *mut *mut int data_count; / DATA-Out payload progress,
    pub datasn: c_int,
// LLDs should set/update these values
    pub /: *mut *mut int sent; / R2T sequence progress,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_task {
//
// Because LLDs allocate their hdr differently, this is a pointer
// and length to that storage. It must be setup at session
// creation time.
//
    pub hdr: *mut iscsi_hdr,
    pub hdr_max: c_ushort,
    pub /: *mut *mut unsigned short hdr_len; / accumulated size of hdr used,
// copied values in case we need to send tmfs
    pub hdr_itt: itt_t,
    pub cmdsn: __be32,
    pub lun: scsi_lun,
    pub /: *mut *mut int itt; / this ITT,
    pub /: *mut *mut unsigned imm_count; / imm-data (bytes),
// offset in unsolicited stream (bytes);
    pub unsol_r2t: iscsi_r2t_info,
    pub /: *mut *mut *mut char data; / mgmt payload,
    pub data_count: unsigned,
    pub cmd*/: *mut *mut *mut scsi_cmnd sc; / associated SCSI,
    pub /: *mut *mut *mut iscsi_conn conn; / used connection,
// data processing tracking
    pub last_xfer: c_ulong,
    pub last_timeout: c_ulong,
    pub have_checked_conn: bool,
// T10 protection information
    pub protected: bool,
// state set/tested under session->lock
    pub state: c_int,
    pub refcount: refcount_t,
    pub /: *mut *mut list_head running; / running cmd list,
    pub /: *mut *mut *mut void dd_data; / driver/transport data,
}

// Private data associated with struct scsi_cmnd.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_cmd {
    pub task: *mut iscsi_task,
    pub age: c_int,
}

extern "C" {
    pub fn scsi_cmd_priv(_arg: cmd) -> return;
}
// Connection's states
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_conn {
    pub /: *mut *mut *mut iscsi_cls_conn cls_conn; / ptr to class connection,
    pub /: *mut *mut *mut void dd_data; / iscsi_transport data,
    pub /: *mut *mut *mut iscsi_session session; / parent session,
//
// conn_stop() flag: stop to recover, stop to terminate
//
    pub stop_stage: c_int,
    pub transport_timer: timer_list,
    pub last_recv: c_ulong,
    pub last_ping: c_ulong,
    pub ping_timeout: c_int,
    pub recv_timeout: c_int,
    pub ping_task: *mut iscsi_task,
// iSCSI connection-wide sequencing
    pub exp_statsn: u32,
    pub statsn: u32,
// control data
    pub /: *mut *mut int id; / CID,
    pub /: *mut *mut int c_stage; / connection state,
//
// Preallocated buffer for pdus that have data but do not
// originate from scsi-ml. We never have two pdus using the
// buffer at the same time. It is only allocated to
// the default max recv size because the pdus we support
// should always fit in this buffer
//
    pub data: *mut c_char,
    pub /: *mut *mut *mut iscsi_task login_task; / mtask used for login/text,
    pub /: *mut *mut *mut iscsi_task task; / xmit task in progress,
// xmit
// items must be added/deleted under frwd lock
    pub /: *mut *mut list_head mgmtqueue; / mgmt (control) xmit queue,
    pub /: *mut *mut list_head cmdqueue; / data-path cmd queue,
    pub /: *mut *mut list_head requeue; / tasks needing another run,
    pub /: *mut *mut work_xmitwork; / per-conn. xmit workqueue,
// recv
    pub recvwork: work_struct,
    pub /: *mut *mut unsigned long flags; / ISCSI_CONN_FLAGs,
// negotiated params
    pub initiator_max_recv_dsl*/: *mut *mut unsigned max_recv_dlength; /,
    pub /: *mut *mut unsigned max_xmit_dlength; / target_max_recv_dsl,
    pub hdrdgst_en: c_int,
    pub datadgst_en: c_int,
    pub ifmarker_en: c_int,
    pub ofmarker_en: c_int,
// values userspace uses to id a conn
    pub persistent_port: c_int,
    pub persistent_address: *mut c_char,
    pub max_segment_size: unsigned,
    pub tcp_xmit_wsf: unsigned,
    pub tcp_recv_wsf: unsigned,
    pub keepalive_tmo: u16,
    pub local_port: u16,
    pub tcp_timestamp_stat: u8,
    pub tcp_nagle_disable: u8,
    pub tcp_wsf_disable: u8,
    pub tcp_timer_scale: u8,
    pub tcp_timestamp_en: u8,
    pub fragment_disable: u8,
    pub ipv4_tos: u8,
    pub ipv6_traffic_class: u8,
    pub ipv6_flow_label: u8,
    pub is_fw_assigned_ipv6: u8,
    pub local_ipaddr: *mut c_char,
// MIB-statistics
    pub txdata_octets: u64,
    pub rxdata_octets: u64,
    pub scsicmd_pdus_cnt: u32,
    pub dataout_pdus_cnt: u32,
    pub scsirsp_pdus_cnt: u32,
    pub datain_pdus_cnt: u32,
    pub r2t_pdus_cnt: u32,
    pub tmfcmd_pdus_cnt: u32,
    pub tmfrsp_pdus_cnt: i32,
// custom statistics
    pub eh_abort_cnt: u32,
    pub fmr_unalign_cnt: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_pool {
    pub /: *mut *mut kfifo queue; / FIFO Queue,
    pub /: *mut *mut *mut *mut void pool; / Pool of elements,
    pub /: *mut *mut int max; / Max number of elements,
}

// Session's states
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_session {
    pub cls_session: *mut iscsi_cls_session,
//
// Syncs up the scsi eh thread with the iscsi eh thread when sending
// task management functions. This must be taken before the session
// and recv lock.
//
    pub eh_mutex: mutex,
// abort
    pub /: *mut *mut wait_queue_head_t ehwait; / used in eh_abort(),
    pub tmhdr: iscsi_tm,
    pub tmf_timer: timer_list,
    pub etc.*/: *mut *mut int tmf_state; / see TMF_INITIAL,,
    pub running_aborted_task: *mut iscsi_task,
// iSCSI session-wide sequencing
    pub cmdsn: u32,
    pub exp_cmdsn: u32,
    pub max_cmdsn: u32,
// This tracks the reqs queued into the initiator
    pub queued_cmdsn: u32,
// configuration
    pub abort_timeout: c_int,
    pub lu_reset_timeout: c_int,
    pub tgt_reset_timeout: c_int,
    pub initial_r2t_en: c_int,
    pub max_r2t: c_ushort,
    pub imm_data_en: c_int,
    pub first_burst: unsigned,
    pub max_burst: unsigned,
    pub time2wait: c_int,
    pub time2retain: c_int,
    pub pdu_inorder_en: c_int,
    pub dataseq_inorder_en: c_int,
    pub erl: c_int,
    pub fast_abort: c_int,
    pub tpgt: c_int,
    pub username: *mut c_char,
    pub username_in: *mut c_char,
    pub password: *mut c_char,
    pub password_in: *mut c_char,
    pub targetname: *mut c_char,
    pub targetalias: *mut c_char,
    pub ifacename: *mut c_char,
    pub initiatorname: *mut c_char,
    pub boot_root: *mut c_char,
    pub boot_nic: *mut c_char,
    pub boot_target: *mut c_char,
    pub portal_type: *mut c_char,
    pub discovery_parent_type: *mut c_char,
    pub discovery_parent_idx: u16,
    pub def_taskmgmt_tmo: u16,
    pub tsid: u16,
    pub auto_snd_tgt_disable: u8,
    pub discovery_sess: u8,
    pub chap_auth_en: u8,
    pub discovery_logout_en: u8,
    pub bidi_chap_en: u8,
    pub discovery_auth_optional: u8,
    pub isid: [u8; ISID_SIZE],
// control data
    pub tt: *mut iscsi_transport,
    pub host: *mut Scsi_Host,
    pub /: *mut *mut *mut iscsi_conn leadconn; / leading connection,
// Between the forward and the backward locks exists a strict locking
// hierarchy. The mutual exclusion zone protected by the forward lock
// can enclose the mutual exclusion zone protected by the backward lock
// but not vice versa.
//
    pub : *mut *mut spinlock_t frwd_lock; / protects session state,,
// cmdsn, queued_cmdsn
// session resources:
// - cmdpool kfifo_out ,
// - mgmtpool, queues
    pub : *mut *mut spinlock_t back_lock; / protects cmdsn_exp,
// cmdsn_max,
// cmdpool kfifo_in
    pub /: *mut *mut int state; / session state,
    pub /: *mut *mut int age; / counts session re-opens,
    pub /: *mut *mut int scsi_cmds_max; / max scsi commands,
    pub /: *mut *mut int cmds_max; / size of cmds array,
    pub /: *mut *mut *mut *mut iscsi_task cmds; / Original Cmds arr,
    pub /: *mut *mut iscsi_pool cmdpool; / PDU's pool,
    pub /: *mut *mut *mut void dd_data; / LLD private data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_host {
    pub initiatorname: *mut c_char,
// hw address or netdev iscsi connection is bound to
    pub hwaddress: *mut c_char,
    pub netdev: *mut c_char,
    pub session_removal_wq: wait_queue_head_t,
// protects sessions and state
    pub lock: spinlock_t,
    pub num_sessions: c_int,
    pub state: c_int,
    pub workq: *mut workqueue_struct,
}

//
// scsi host template
//
extern "C" {
    pub fn iscsi_eh_abort(sc: *mut scsi_cmnd) -> c_int;
}
extern "C" {
    pub fn iscsi_eh_recover_target(sc: *mut scsi_cmnd) -> c_int;
}
extern "C" {
    pub fn iscsi_eh_session_reset(sc: *mut scsi_cmnd) -> c_int;
}
extern "C" {
    pub fn iscsi_eh_device_reset(sc: *mut scsi_cmnd) -> c_int;
}
extern "C" {
    pub fn iscsi_eh_cmd_timed_out(sc: *mut scsi_cmnd) -> scsi_timeout_action;
}
//
// iSCSI host helpers.
//

extern "C" {
    pub fn iscsi_host_add(shost: *mut Scsi_Host, pdev: *mut device) -> c_int;
}
extern "C" {
    pub fn iscsi_host_remove(shost: *mut Scsi_Host, is_shutdown: bool);
}
extern "C" {
    pub fn iscsi_host_free(shost: *mut Scsi_Host);
}
extern "C" {
    pub fn iscsi_target_alloc(starget: *mut scsi_target) -> c_int;
}
//
// session management
//
extern "C" {
    pub fn iscsi_session_remove(cls_session: *mut iscsi_cls_session);
}
extern "C" {
    pub fn iscsi_session_free(cls_session: *mut iscsi_cls_session);
}
extern "C" {
    pub fn iscsi_session_teardown(: *mut iscsi_cls_session);
}
extern "C" {
    pub fn iscsi_session_recovery_timedout(: *mut iscsi_cls_session);
}

//
// connection management
//
extern "C" {
    pub fn iscsi_conn_teardown(: *mut iscsi_cls_conn);
}
extern "C" {
    pub fn iscsi_conn_start(: *mut iscsi_cls_conn) -> c_int;
}
extern "C" {
    pub fn iscsi_conn_stop(: *mut iscsi_cls_conn, _arg: c_int);
}
extern "C" {
    pub fn iscsi_conn_unbind(cls_conn: *mut iscsi_cls_conn, is_active: bool);
}
extern "C" {
    pub fn iscsi_conn_failure(conn: *mut iscsi_conn, err: iscsi_err);
}
extern "C" {
    pub fn iscsi_suspend_tx(conn: *mut iscsi_conn);
}
extern "C" {
    pub fn iscsi_suspend_rx(conn: *mut iscsi_conn);
}
extern "C" {
    pub fn iscsi_suspend_queue(conn: *mut iscsi_conn);
}
extern "C" {
    pub fn iscsi_conn_queue_xmit(conn: *mut iscsi_conn);
}
extern "C" {
    pub fn iscsi_conn_queue_recv(conn: *mut iscsi_conn);
}

//
// pdu and task processing
//
extern "C" {
    pub fn iscsi_update_cmdsn(: *mut iscsi_session, : *mut iscsi_nopin);
}
extern "C" {
    pub fn iscsi_verify_itt(: *mut iscsi_conn, _arg: itt_t) -> c_int;
}
extern "C" {
    pub fn iscsi_requeue_task(task: *mut iscsi_task);
}
extern "C" {
    pub fn iscsi_put_task(task: *mut iscsi_task);
}
extern "C" {
    pub fn __iscsi_put_task(task: *mut iscsi_task);
}
extern "C" {
    pub fn iscsi_get_task(task: *mut iscsi_task) -> bool;
}
//
// generic helpers
//
extern "C" {
    pub fn iscsi_pool_free(: *mut iscsi_pool);
}
extern "C" {
    pub fn iscsi_pool_init(: *mut iscsi_pool, _arg: c_int, : *mut c_void, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn iscsi_switch_str_param(: *mut c_char, : *mut c_char) -> c_int;
}
//
// inline functions to deal with padding.
//
