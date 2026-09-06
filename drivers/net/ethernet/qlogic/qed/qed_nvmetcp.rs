//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/qlogic/qed/qed_nvmetcp.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
// Copyright 2021 Marvell. All rights reserved.

// tcp parameters
pub const QED_TCP_FLOW_LABEL: c_int = 0;
pub const QED_TCP_TWO_MSL_TIMER: c_int = 4000;
pub const QED_TCP_HALF_WAY_CLOSE_TIMEOUT: c_int = 10;
pub const QED_TCP_MAX_FIN_RT: c_int = 2;
pub const QED_TCP_SWS_TIMER: c_int = 5000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_nvmetcp_info {
    pub /: *mut *mut spinlock_t lock; / Connection resources.,
    pub free_list: list_head,
    pub max_num_outstanding_tasks: u16,
    pub event_context: *mut c_void,
    pub event_cb: nvmetcp_event_cb_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_hash_nvmetcp_con {
    pub node: hlist_node,
    pub con: *mut qed_nvmetcp_conn,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_nvmetcp_conn {
    pub list_entry: list_head,
    pub free_on_delete: bool,
    pub conn_id: u16,
    pub icid: u32,
    pub fw_cid: u32,
    pub layer_code: u8,
    pub offl_flags: u8,
    pub connect_mode: u8,
    pub sq_pbl_addr: dma_addr_t,
    pub r2tq: qed_chain,
    pub xhq: qed_chain,
    pub uhq: qed_chain,
    pub local_mac: [u8; 6],
    pub remote_mac: [u8; 6],
    pub ip_version: u8,
    pub ka_max_probe_cnt: u8,
    pub vlan_id: u16,
    pub tcp_flags: u16,
    pub remote_ip: [u32; 4],
    pub local_ip: [u32; 4],
    pub flow_label: u32,
    pub ka_timeout: u32,
    pub ka_interval: u32,
    pub max_rt_time: u32,
    pub ttl: u8,
    pub tos_or_tc: u8,
    pub remote_port: u16,
    pub local_port: u16,
    pub mss: u16,
    pub rcv_wnd_scale: u8,
    pub rcv_wnd: u32,
    pub cwnd: u32,
    pub update_flag: u8,
    pub default_cq: u8,
    pub abortive_dsconnect: u8,
    pub max_seq_size: u32,
    pub max_recv_pdu_length: u32,
    pub max_send_pdu_length: u32,
    pub first_seq_length: u32,
    pub physical_q0: u16,
    pub physical_q1: u16,
    pub nvmetcp_cccid_max_range: u16,
    pub nvmetcp_cccid_itid_table_addr: dma_addr_t,
}

extern "C" {
    pub fn qed_nvmetcp_alloc(p_hwfn: *mut qed_hwfn) -> c_int;
}
extern "C" {
    pub fn qed_nvmetcp_setup(p_hwfn: *mut qed_hwfn);
}
extern "C" {
    pub fn qed_nvmetcp_free(p_hwfn: *mut qed_hwfn);
}

