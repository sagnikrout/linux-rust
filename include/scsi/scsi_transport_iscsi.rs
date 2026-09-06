//! Automatically rewritten from C Header to Rust Module
//! Source: include/scsi/scsi_transport_iscsi.h
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
// iSCSI transport class definitions
//
// Copyright (C) IBM Corporation, 2004
// Copyright (C) Mike Christie, 2004 - 2006
// Copyright (C) Dmitry Yusupov, 2004 - 2005
// Copyright (C) Alex Aizman, 2004 - 2005
//

//
// struct iscsi_transport - iSCSI Transport template
//
// @name:		transport name
// @caps:		iSCSI Data-Path capabilities
// @create_session:	create new iSCSI session object
// @destroy_session:	destroy existing iSCSI session object
// @create_conn:	create new iSCSI connection
// @bind_conn:		associate this connection with existing iSCSI session
// and specified transport descriptor
// @destroy_conn:	destroy inactive iSCSI connection
// @set_param:		set iSCSI parameter. Return 0 on success, -ENODATA
// when param is not supported, and a -Exx value on other
// error.
// @get_param		get iSCSI parameter. Must return number of bytes
// copied to buffer on success, -ENODATA when param
// is not supported, and a -Exx value on other error
// @start_conn:		set connection to be operational
// @stop_conn:		suspend/recover/terminate connection
// @send_pdu:		send iSCSI PDU, Login, Logout, NOP-Out, Reject, Text.
// @session_recovery_timedout: notify LLD a block during recovery timed out
// @init_task:		Initialize a iscsi_task and any internal structs.
// When offloading the data path, this is called from
// queuecommand with the session lock, or from the
// iscsi_conn_send_pdu context with the session lock.
// When not offloading the data path, this is called
// from the scsi work queue without the session lock.
// @xmit_task		Requests LLD to transfer cmd task. Returns 0 or the
// number of bytes transferred on success, and -Exyz
// value on error. When offloading the data path, this
// is called from queuecommand with the session lock, or
// from the iscsi_conn_send_pdu context with the session
// lock. When not offloading the data path, this is called
// from the scsi work queue without the session lock.
// @cleanup_task:	requests LLD to fail task. Called with session lock
// and after the connection has been suspended and
// terminated during recovery. If called
// from abort task then connection is not suspended
// or terminated but sk_callback_lock is held
//
// Template API provided by iSCSI Transport
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_transport {
    pub owner: *mut module,
    pub name: *mut c_char,
    pub caps: c_uint,
    pub sn): u32,
    pub session): *mut *mut void (destroy_session) (struct iscsi_cls_session,
    pub cid): u32,
    pub is_active): *mut *mut *mut void (unbind_conn) (struct iscsi_cls_conn conn, bool,
    pub is_leading): uint64_t transport_eph, int,
    pub conn): *mut *mut int (start_conn) (struct iscsi_cls_conn,
    pub flag): *mut *mut *mut void (stop_conn) (struct iscsi_cls_conn conn, int,
    pub conn): *mut *mut void (destroy_conn) (struct iscsi_cls_conn,
    pub buflen): *mut *mut char buf, int,
    pub buf): *mut c_char,
    pub buf): *mut iscsi_param param, char,
    pub buf): *mut iscsi_param param, char,
    pub buf): *mut iscsi_host_param param, char,
    pub buflen): c_int,
    pub data_size): *mut *mut char data, uint32_t,
    pub stats): *mut iscsi_stats,
    pub task): *mut *mut int (init_task) (struct iscsi_task,
    pub task): *mut *mut int (xmit_task) (struct iscsi_task,
    pub task): *mut *mut void (cleanup_task) (struct iscsi_task,
    pub opcode): *mut *mut *mut int (alloc_pdu) (struct iscsi_task task, uint8_t,
    pub task): *mut *mut int (xmit_pdu) (struct iscsi_task,
    pub count): c_uint,
    pub age): *mut *mut int index, int,
    pub session): *mut *mut void (session_recovery_timedout) (struct iscsi_cls_session,
    pub non_blocking): c_int,
    pub timeout_ms): *mut *mut *mut int (ep_poll) (struct iscsi_endpoint ep, int,
    pub ep): *mut *mut void (ep_disconnect) (struct iscsi_endpoint,
    pub dst_addr): *mut uint32_t enable, struct sockaddr,
    pub params): *mut *mut *mut int (set_path) (struct Scsi_Host shost, struct iscsi_path,
    pub len): u32,
    pub buf): *mut int param, char,
    pub param): *mut *mut umode_t (attr_is_visible)(int param_type, int,
    pub job): *mut *mut int (bsg_request)(struct bsg_job,
    pub dst_addr): *mut uint32_t pid, struct sockaddr,
    pub buf): *mut *mut uint32_t num_entries, char,
    pub chap_tbl_idx): *mut *mut *mut int (delete_chap) (struct Scsi_Host shost, uint16_t,
    pub len): *mut *mut *mut *mut int (set_chap) (struct Scsi_Host shost, void data, int,
    pub buf): *mut int param, char,
    pub len): *mut *mut void data, int,
    pub len): c_int,
    pub fnode_sess): *mut *mut int (del_flashnode) (struct iscsi_bus_flash_session,
    pub fnode_conn): *mut iscsi_bus_flash_conn,
    pub fnode_conn): *mut iscsi_bus_flash_conn,
    pub cls_sess): *mut *mut int (logout_flashnode_sid) (struct iscsi_cls_session,
    pub len): *mut *mut *mut *mut int (get_host_stats) (struct Scsi_Host shost, char buf, int,
    pub sector): *mut *mut *mut u8 (check_protection)(struct iscsi_task task, sector_t,
}

//
// transport registration upcalls
//
extern "C" {
    pub fn iscsi_unregister_transport(tt: *mut iscsi_transport);
}
//
// control plane upcalls
//
// iscsi class connection state
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iscsi_connection_state {
    ISCSI_CONN_UP = 0,
    ISCSI_CONN_DOWN,
    ISCSI_CONN_FAILED,
    ISCSI_CONN_BOUND,
}

pub const ISCSI_CLS_CONN_BIT_CLEANUP: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_cls_conn {
    pub /: *mut *mut list_head conn_list; / item in connlist,
    pub /: *mut *mut *mut void dd_data; / LLD private data,
    pub transport: *mut iscsi_transport,
    pub /: *mut *mut uint32_t cid; / connection id,
//
// This protects the conn startup and binding/unbinding of the ep to
// the conn. Unbinding includes ep_disconnect and stop_conn.
//
    pub ep_mutex: mutex,
    pub ep: *mut iscsi_endpoint,
// Used when accessing flags and queueing work.
    pub lock: spinlock_t,
    pub flags: c_ulong,
    pub cleanup_work: work_struct,
    pub /: *mut *mut device dev; / sysfs transport/container device,
    pub state: iscsi_connection_state,
}

// iscsi class session state

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_cls_session {
    pub /: *mut *mut list_head sess_list; / item in session_list,
    pub transport: *mut iscsi_transport,
    pub lock: spinlock_t,
    pub block_work: work_struct,
    pub unblock_work: work_struct,
    pub scan_work: work_struct,
    pub unbind_work: work_struct,
    pub destroy_work: work_struct,
// recovery fields
    pub recovery_tmo: c_int,
    pub recovery_tmo_sysfs_override: bool,
    pub recovery_work: delayed_work,
    pub workq: *mut workqueue_struct,
    pub target_id: c_uint,
    pub ida_used: bool,
//
// pid of userspace process that created session or -1 if
// created by the kernel.
//
    pub creator: pid_t,
    pub state: c_int,
    pub /: *mut *mut int target_state; / session target bind state,
    pub /: *mut *mut int sid; / session id,
    pub /: *mut *mut *mut void dd_data; / LLD private data,
    pub /: *mut *mut device dev; / sysfs transport/container device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_cls_host {
    pub mutex: mutex,
    pub bsg_q: *mut request_queue,
    pub port_speed: u32,
    pub port_state: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_endpoint {
    pub /: *mut *mut *mut void dd_data; / LLD private data,
    pub dev: device,
    pub id: c_int,
    pub conn: *mut iscsi_cls_conn,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_iface {
    pub dev: device,
    pub transport: *mut iscsi_transport,
    pub /: *mut *mut uint32_t iface_type; / IPv4 or IPv6,
    pub /: *mut *mut uint32_t iface_num; / iface number, 0 - n,
    pub /: *mut *mut *mut void dd_data; / LLD private data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_bus_flash_conn {
    pub /: *mut *mut list_head conn_list; / item in connlist,
    pub /: *mut *mut *mut void dd_data; / LLD private data,
    pub transport: *mut iscsi_transport,
    pub /: *mut *mut device dev; / sysfs transport/container device,
// iscsi connection parameters
    pub exp_statsn: u32,
    pub statsn: u32,
    pub initiator_max_recv_dsl*/: *mut *mut unsigned max_recv_dlength; /,
    pub /: *mut *mut unsigned max_xmit_dlength; / target_max_recv_dsl,
    pub max_segment_size: unsigned,
    pub tcp_xmit_wsf: unsigned,
    pub tcp_recv_wsf: unsigned,
    pub hdrdgst_en: c_int,
    pub datadgst_en: c_int,
    pub port: c_int,
    pub ipaddress: *mut c_char,
    pub link_local_ipv6_addr: *mut c_char,
    pub redirect_ipaddr: *mut c_char,
    pub keepalive_timeout: u16,
    pub local_port: u16,
    pub snack_req_en: u8,
// tcp timestamp negotiation status
    pub tcp_timestamp_stat: u8,
    pub tcp_nagle_disable: u8,
// tcp window scale factor
    pub tcp_wsf_disable: u8,
    pub tcp_timer_scale: u8,
    pub tcp_timestamp_en: u8,
    pub ipv4_tos: u8,
    pub ipv6_traffic_class: u8,
    pub ipv6_flow_label: u8,
    pub fragment_disable: u8,
// Link local IPv6 address is assigned by firmware or driver
    pub is_fw_assigned_ipv6: u8,
}

pub const ISID_SIZE: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_bus_flash_session {
    pub /: *mut *mut list_head sess_list; / item in session_list,
    pub transport: *mut iscsi_transport,
    pub target_id: c_uint,
    pub /: *mut *mut int flash_state; / persistent or non-persistent,
    pub /: *mut *mut *mut void dd_data; / LLD private data,
    pub /: *mut *mut device dev; / sysfs transport/container device,
// iscsi session parameters
    pub first_burst: unsigned,
    pub max_burst: unsigned,
    pub max_r2t: c_ushort,
    pub default_taskmgmt_timeout: c_int,
    pub initial_r2t_en: c_int,
    pub imm_data_en: c_int,
    pub time2wait: c_int,
    pub time2retain: c_int,
    pub pdu_inorder_en: c_int,
    pub dataseq_inorder_en: c_int,
    pub erl: c_int,
    pub tpgt: c_int,
    pub username: *mut c_char,
    pub username_in: *mut c_char,
    pub password: *mut c_char,
    pub password_in: *mut c_char,
    pub targetname: *mut c_char,
    pub targetalias: *mut c_char,
    pub portal_type: *mut c_char,
    pub tsid: u16,
    pub chap_in_idx: u16,
    pub chap_out_idx: u16,
// index of iSCSI discovery session if the entry is
// discovered by iSCSI discovery session
//
    pub discovery_parent_idx: u16,
// indicates if discovery was done through iSNS discovery service
// or through sendTarget
    pub discovery_parent_type: u16,
// Firmware auto sendtarget discovery disable
    pub auto_snd_tgt_disable: u8,
    pub discovery_sess: u8,
// indicates if this flashnode entry is enabled or disabled
    pub entry_state: u8,
    pub chap_auth_en: u8,
// enables firmware to auto logout the discovery session on discovery
// completion
//
    pub discovery_logout_en: u8,
    pub bidi_chap_en: u8,
// makes authentication for discovery session optional
    pub discovery_auth_optional: u8,
    pub isid: [u8; ISID_SIZE],
    pub is_boot_target: u8,
}

//
// session and connection functions that can be used by HW iSCSI LLDs
//

extern "C" {
    pub fn iscsi_session_chkready(session: *mut iscsi_cls_session) -> c_int;
}
extern "C" {
    pub fn iscsi_is_session_online(session: *mut iscsi_cls_session) -> c_int;
}
extern "C" {
    pub fn iscsi_force_destroy_session(session: *mut iscsi_cls_session);
}
extern "C" {
    pub fn iscsi_remove_session(session: *mut iscsi_cls_session);
}
extern "C" {
    pub fn iscsi_free_session(session: *mut iscsi_cls_session);
}
extern "C" {
    pub fn iscsi_add_conn(conn: *mut iscsi_cls_conn) -> c_int;
}
extern "C" {
    pub fn iscsi_remove_conn(conn: *mut iscsi_cls_conn);
}
extern "C" {
    pub fn iscsi_put_conn(conn: *mut iscsi_cls_conn);
}
extern "C" {
    pub fn iscsi_get_conn(conn: *mut iscsi_cls_conn);
}
extern "C" {
    pub fn iscsi_unblock_session(session: *mut iscsi_cls_session);
}
extern "C" {
    pub fn iscsi_block_session(session: *mut iscsi_cls_session);
}
extern "C" {
    pub fn iscsi_destroy_endpoint(ep: *mut iscsi_endpoint);
}
extern "C" {
    pub fn iscsi_put_endpoint(ep: *mut iscsi_endpoint);
}
extern "C" {
    pub fn iscsi_block_scsi_eh(cmd: *mut scsi_cmnd) -> c_int;
}
extern "C" {
    pub fn iscsi_destroy_iface(iface: *mut iscsi_iface);
}
extern "C" {
    pub fn iscsi_is_session_dev(dev: *const device) -> c_int;
}
extern "C" {
    pub fn iscsi_destroy_all_flashnode(shost: *mut Scsi_Host);
}
