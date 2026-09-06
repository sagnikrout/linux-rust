//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/qlogic/qed/qed_iwarp.h
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
// QLogic qed NIC Driver
// Copyright (c) 2015-2017  QLogic Corporation
// Copyright (c) 2019-2020 Marvell International Ltd.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_iwarp_qp_state {
    QED_IWARP_QP_STATE_IDLE,
    QED_IWARP_QP_STATE_RTS,
    QED_IWARP_QP_STATE_TERMINATE,
    QED_IWARP_QP_STATE_CLOSING,
    QED_IWARP_QP_STATE_ERROR,
}

extern "C" {
    pub fn qed_roce2iwarp_state(state: qed_roce_qp_state) -> qed_iwarp_qp_state;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_iwarp_ll2_buff {
    pub piggy_buf: *mut qed_iwarp_ll2_buff,
    pub data: *mut c_void,
    pub data_phys_addr: dma_addr_t,
    pub buff_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_iwarp_ll2_mpa_buf {
    pub list_entry: list_head,
    pub ll2_buf: *mut qed_iwarp_ll2_buff,
    pub data: unaligned_opaque_data,
    pub tcp_payload_len: u16,
    pub placement_offset: u8,
}

// In some cases a fpdu will arrive with only one byte of the header, in this
// case the fpdu_length will be partial (contain only higher byte and
// incomplete bytes will contain the invalid value
//
pub const QED_IWARP_INVALID_INCOMPLETE_BYTES: c_uint = 0xffff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_iwarp_fpdu {
    pub mpa_buf: *mut qed_iwarp_ll2_buff,
    pub mpa_frag_virt: *mut c_void,
    pub mpa_frag: dma_addr_t,
    pub pkt_hdr: dma_addr_t,
    pub mpa_frag_len: u16,
    pub fpdu_length: u16,
    pub incomplete_bytes: u16,
    pub pkt_hdr_size: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_iwarp_info {
    pub /: *mut *mut list_head listen_list; / qed_iwarp_listener,
    pub /: *mut *mut list_head ep_list; / qed_iwarp_ep,
    pub /: *mut *mut list_head ep_free_list; / pre-allocated ep's,
    pub /: *mut *mut list_head mpa_buf_list; / list of mpa_bufs,
    pub mpa_buf_pending_list: list_head,
    pub /: *mut *mut spinlock_t iw_lock; / for iwarp resources,
    pub /: *mut *mut spinlock_t qp_lock; / for teardown races,
    pub rcv_wnd_scale: u32,
    pub rcv_wnd_size: u16,
    pub max_mtu: u16,
    pub mac_addr: [u8; ETH_ALEN],
    pub crc_needed: u8,
    pub tcp_flags: u8,
    pub ll2_syn_handle: u8,
    pub ll2_ooo_handle: u8,
    pub ll2_mpa_handle: u8,
    pub peer2peer: u8,
    pub mpa_rev: mpa_negotiation_mode,
    pub rtr_type: mpa_rtr_type,
    pub partial_fpdus: *mut qed_iwarp_fpdu,
    pub mpa_bufs: *mut qed_iwarp_ll2_mpa_buf,
    pub mpa_intermediate_buf: *mut u8,
    pub max_num_partial_fpdus: u16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_iwarp_ep_state {
    QED_IWARP_EP_INIT,
    QED_IWARP_EP_MPA_REQ_RCVD,
    QED_IWARP_EP_MPA_OFFLOADED,
    QED_IWARP_EP_ESTABLISHED,
    QED_IWARP_EP_CLOSED
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union async_output {
    pub mpa_response: iwarp_eqe_data_mpa_async_completion,
    pub mpa_request: iwarp_eqe_data_tcp_async_completion,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_iwarp_ep_memory {
    pub in_pdata: [u8; QED_MAX_PRIV_DATA_LEN],
    pub out_pdata: [u8; QED_MAX_PRIV_DATA_LEN],
    pub async_output: async_output,
}

// Endpoint structure represents a TCP connection. This connection can be
// associated with a QP or not (in which case QP==NULL)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_iwarp_ep {
    pub list_entry: list_head,
    pub qp: *mut qed_rdma_qp,
    pub ep_buffer_virt: *mut qed_iwarp_ep_memory,
    pub ep_buffer_phys: dma_addr_t,
    pub state: qed_iwarp_ep_state,
    pub sig: c_int,
    pub cm_info: qed_iwarp_cm_info,
    pub connect_mode: tcp_connect_mode,
    pub rtr_type: mpa_rtr_type,
    pub mpa_rev: mpa_negotiation_mode,
    pub tcp_cid: u32,
    pub cid: u32,
    pub mss: u16,
    pub remote_mac_addr: [u8; 6],
    pub local_mac_addr: [u8; 6],
    pub mpa_reply_processed: bool,
// For Passive side - syn packet related data
    pub syn_ip_payload_length: u16,
    pub syn: *mut qed_iwarp_ll2_buff,
    pub syn_phy_addr: dma_addr_t,
// The event_cb function is called for asynchrounous events associated
// with the ep. It is initialized at different entry points depending
// on whether the ep is the tcp connection active side or passive side
// The cb_context is passed to the event_cb function.
//
    pub event_cb: iwarp_event_handler,
    pub cb_context: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_iwarp_listener {
    pub list_entry: list_head,
// The event_cb function is called for connection requests.
// The cb_context is passed to the event_cb function.
//
    pub event_cb: iwarp_event_handler,
    pub cb_context: *mut c_void,
    pub max_backlog: u32,
    pub ip_addr: [u32; 4],
    pub port: u16,
    pub vlan: u16,
    pub ip_version: u8,
}

extern "C" {
    pub fn qed_iwarp_alloc(p_hwfn: *mut qed_hwfn) -> c_int;
}
extern "C" {
    pub fn qed_iwarp_stop(p_hwfn: *mut qed_hwfn) -> c_int;
}
extern "C" {
    pub fn qed_iwarp_resc_free(p_hwfn: *mut qed_hwfn);
}
extern "C" {
    pub fn qed_iwarp_init_devinfo(p_hwfn: *mut qed_hwfn);
}
extern "C" {
    pub fn qed_iwarp_init_hw(p_hwfn: *mut qed_hwfn, p_ptt: *mut qed_ptt);
}
extern "C" {
    pub fn qed_iwarp_destroy_qp(p_hwfn: *mut qed_hwfn, qp: *mut qed_rdma_qp) -> c_int;
}
extern "C" {
    pub fn qed_iwarp_fw_destroy(p_hwfn: *mut qed_hwfn, qp: *mut qed_rdma_qp) -> c_int;
}
extern "C" {
    pub fn qed_iwarp_accept(rdma_cxt: *mut c_void, iparams: *mut qed_iwarp_accept_in) -> c_int;
}
extern "C" {
    pub fn qed_iwarp_reject(rdma_cxt: *mut c_void, iparams: *mut qed_iwarp_reject_in) -> c_int;
}
extern "C" {
    pub fn qed_iwarp_destroy_listen(rdma_cxt: *mut c_void, handle: *mut c_void) -> c_int;
}
extern "C" {
    pub fn qed_iwarp_send_rtr(rdma_cxt: *mut c_void, iparams: *mut qed_iwarp_send_rtr_in) -> c_int;
}
