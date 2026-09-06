//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/target/tcm_fc/tcm_fc.h
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
// Copyright (c) 2010 Cisco Systems, Inc.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ft_transport_id {
    pub format: __u8,
    pub __resvd1: [__u8; 7],
    pub wwpn: [__u8; 8],
    pub __resvd2: [__u8; 8],
    pub __attribute__((__packed__)): },
//
// Session (remote port).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ft_sess {
    pub /: *mut *mut u32 port_id; / for hash lookup use only,
    pub params: u32,
    pub /: *mut *mut u16 max_frame; / maximum frame size,
    pub /: *mut *mut u64 port_name; / port name for transport ID,
    pub tport: *mut ft_tport,
    pub se_sess: *mut se_session,
    pub /: *mut *mut hlist_node hash; / linkage in ft_sess_hash table,
    pub rcu: rcu_head,
    pub /: *mut *mut kref kref; / ref for hash and outstanding I/Os,
}

//
// Hash table of sessions per local port.
// Hash lookup by remote port FC_ID.
//
pub const FT_SESS_HASH_BITS: c_int = 6;

//
// Per local port data.
// This is created only after a TPG exists that allows target function
// for the local port.  If the TPG exists, this is allocated when
// we're notified that the local port has been created, or when
// the first PRLI provider callback is received.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ft_tport {
    pub lport: *mut fc_lport,
    pub /: *mut *mut *mut ft_tpg tpg; / NULL if TPG deleted before tport,
    pub /: *mut *mut u32 sess_count; / number of sessions in hash,
    pub rcu: rcu_head,
    pub /: *mut *mut hlist_head hash[FT_SESS_HASH_SIZE]; / list of sessions,
}

//
// Node ID and authentication.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ft_node_auth {
    pub port_name: u64,
    pub node_name: u64,
}

//
// Node ACL for FC remote port session.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ft_node_acl {
    pub se_node_acl: se_node_acl,
    pub node_auth: ft_node_auth,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ft_lun {
    pub index: u32,
    pub name: [c_char; FT_LUN_NAMELEN],
}

//
// Target portal group (local port).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ft_tpg {
    pub index: u32,
    pub lport_wwn: *mut ft_lport_wwn,
    pub /: *mut *mut *mut ft_tport tport; / active tport or NULL,
    pub /: *mut *mut list_head lun_list; / head of LUNs,
    pub se_tpg: se_portal_group,
    pub workqueue: *mut workqueue_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ft_lport_wwn {
    pub wwpn: u64,
    pub name: [c_char; FT_NAMELEN],
    pub ft_wwn_node: list_head,
    pub tpg: *mut ft_tpg,
    pub se_wwn: se_wwn,
}

//
// Commands
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ft_cmd {
    pub /: *mut *mut *mut ft_sess sess; / session held for cmd,
    pub /: *mut *mut *mut fc_seq seq; / sequence in exchange mgr,
    pub /: *mut *mut se_cmd se_cmd; / Local TCM I/O descriptor,
    pub req_frame: *mut fc_frame,
    pub /: *mut *mut u32 write_data_len; / data received on writes,
    pub work: work_struct,
// Local sense buffer
    pub ft_sense_buffer: [c_uchar; TRANSPORT_SENSE_BUFFER],
    pub /: *mut *mut u32 was_ddp_setup:1; / Set only if ddp is setup,
    pub /: *mut *mut u32 aborted:1; / Set if aborted by reset or timeout,
    pub /: *mut *mut *mut scatterlist sg; / Set only if DDP is setup,
    pub /: *mut *mut u32 sg_cnt; / No. of item in scatterlist,
}

//
// Fabric methods.
//
// Session ops.
//
extern "C" {
    pub fn ft_sess_put(: *mut ft_sess);
}
extern "C" {
    pub fn ft_sess_close(: *mut se_session);
}
extern "C" {
    pub fn ft_sess_get_index(: *mut se_session) -> u32;
}
extern "C" {
    pub fn ft_sess_get_port_name(: *mut se_session, : *mut c_uchar, _arg: u32) -> u32;
}
extern "C" {
    pub fn ft_lport_add(: *mut fc_lport, : *mut c_void);
}
extern "C" {
    pub fn ft_lport_del(: *mut fc_lport, : *mut c_void);
}
extern "C" {
    pub fn ft_lport_notify(: *mut notifier_block, long: unsigned, : *mut c_void) -> c_int;
}
//
// IO methods.
//
extern "C" {
    pub fn ft_check_stop_free(: *mut se_cmd) -> c_int;
}
extern "C" {
    pub fn ft_release_cmd(: *mut se_cmd);
}
extern "C" {
    pub fn ft_queue_status(: *mut se_cmd) -> c_int;
}
extern "C" {
    pub fn ft_queue_data_in(: *mut se_cmd) -> c_int;
}
extern "C" {
    pub fn ft_write_pending(: *mut se_cmd) -> c_int;
}
extern "C" {
    pub fn ft_queue_tm_resp(: *mut se_cmd);
}
extern "C" {
    pub fn ft_aborted_task(: *mut se_cmd);
}
//
// other internal functions.
//
extern "C" {
    pub fn ft_recv_req(: *mut ft_sess, : *mut fc_frame);
}
extern "C" {
    pub fn ft_recv_write_data(: *mut ft_cmd, : *mut fc_frame);
}
extern "C" {
    pub fn ft_dump_cmd(: *mut ft_cmd, caller: *const c_char);
}
extern "C" {
    pub fn ft_format_wwn(: *mut c_char, _arg: usize, _arg: u64) -> isize;
}
//
// Underlying HW specific helper function
//
extern "C" {
    pub fn ft_invl_hw_context(: *mut ft_cmd);
}
