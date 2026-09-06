//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/qlogic/qed/qed_sp.h
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
pub enum spq_mode {
    QED_SPQ_MODE_BLOCK,     /* Client will poll a designated mem. address */
    QED_SPQ_MODE_CB,        /* Client supplies a callback */
    QED_SPQ_MODE_EBLOCK,    /* QED should block until completion */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_spq_comp_cb {
    pub fw_return_code): u8,
    pub cookie: *mut c_void,
}

//
// qed_eth_cqe_completion(): handles the completion of a
// ramrod on the cqe ring.
//
// @p_hwfn: HW device data.
// @cqe: CQE.
//
// Return: Int.
//
// QED Slow-hwfn queue interface
#[repr(C)]
#[derive(Copy, Clone)]
pub union ramrod_data {
    pub pf_start: pf_start_ramrod_data,
    pub pf_update: pf_update_ramrod_data,
    pub rx_queue_start: rx_queue_start_ramrod_data,
    pub rx_queue_update: rx_queue_update_ramrod_data,
    pub rx_queue_stop: rx_queue_stop_ramrod_data,
    pub tx_queue_start: tx_queue_start_ramrod_data,
    pub tx_queue_stop: tx_queue_stop_ramrod_data,
    pub vport_start: vport_start_ramrod_data,
    pub vport_stop: vport_stop_ramrod_data,
    pub rx_update_gft: rx_update_gft_filter_ramrod_data,
    pub vport_update: vport_update_ramrod_data,
    pub core_rx_queue_start: core_rx_start_ramrod_data,
    pub core_rx_queue_stop: core_rx_stop_ramrod_data,
    pub core_tx_queue_start: core_tx_start_ramrod_data,
    pub core_tx_queue_stop: core_tx_stop_ramrod_data,
    pub vport_filter_update: vport_filter_update_ramrod_data,
    pub rdma_init_func: rdma_init_func_ramrod_data,
    pub rdma_close_func: rdma_close_func_ramrod_data,
    pub rdma_register_tid: rdma_register_tid_ramrod_data,
    pub rdma_deregister_tid: rdma_deregister_tid_ramrod_data,
    pub roce_create_qp_resp: roce_create_qp_resp_ramrod_data,
    pub roce_create_qp_req: roce_create_qp_req_ramrod_data,
    pub roce_modify_qp_resp: roce_modify_qp_resp_ramrod_data,
    pub roce_modify_qp_req: roce_modify_qp_req_ramrod_data,
    pub roce_query_qp_resp: roce_query_qp_resp_ramrod_data,
    pub roce_query_qp_req: roce_query_qp_req_ramrod_data,
    pub roce_destroy_qp_resp: roce_destroy_qp_resp_ramrod_data,
    pub roce_destroy_qp_req: roce_destroy_qp_req_ramrod_data,
    pub roce_init_func: roce_init_func_ramrod_data,
    pub rdma_create_cq: rdma_create_cq_ramrod_data,
    pub rdma_destroy_cq: rdma_destroy_cq_ramrod_data,
    pub rdma_create_srq: rdma_srq_create_ramrod_data,
    pub rdma_destroy_srq: rdma_srq_destroy_ramrod_data,
    pub rdma_modify_srq: rdma_srq_modify_ramrod_data,
    pub iwarp_create_qp: iwarp_create_qp_ramrod_data,
    pub iwarp_tcp_offload: iwarp_tcp_offload_ramrod_data,
    pub iwarp_mpa_offload: iwarp_mpa_offload_ramrod_data,
    pub iwarp_modify_qp: iwarp_modify_qp_ramrod_data,
    pub iwarp_init_func: iwarp_init_func_ramrod_data,
    pub fcoe_init: fcoe_init_ramrod_params,
    pub fcoe_conn_ofld: fcoe_conn_offload_ramrod_params,
    pub fcoe_conn_terminate: fcoe_conn_terminate_ramrod_params,
    pub fcoe_stat: fcoe_stat_ramrod_params,
    pub iscsi_init: iscsi_init_ramrod_params,
    pub iscsi_conn_offload: iscsi_spe_conn_offload,
    pub iscsi_conn_update: iscsi_conn_update_ramrod_params,
    pub iscsi_conn_mac_update: iscsi_spe_conn_mac_update,
    pub iscsi_conn_terminate: iscsi_spe_conn_termination,
    pub nvmetcp_init: nvmetcp_init_ramrod_params,
    pub nvmetcp_conn_offload: nvmetcp_spe_conn_offload,
    pub nvmetcp_conn_update: nvmetcp_conn_update_ramrod_params,
    pub nvmetcp_conn_terminate: nvmetcp_spe_conn_termination,
    pub vf_start: vf_start_ramrod_data,
    pub vf_stop: vf_stop_ramrod_data,
}

pub const EQ_MAX_CREDIT: c_uint = 0xffffffff;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum spq_priority {
    QED_SPQ_PRIORITY_NORMAL,
    QED_SPQ_PRIORITY_HIGH,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union qed_spq_req_comp {
    pub cb: qed_spq_comp_cb,
    pub done_addr: *mut u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_spq_comp_done {
    pub done: c_uint,
    pub fw_return_code: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_spq_entry {
    pub list: list_head,
    pub flags: u8,
// HSI slow path element
    pub elem: slow_path_element,
    pub ramrod: ramrod_data,
    pub priority: spq_priority,
// pending queue for this entry
    pub queue: *mut list_head,
    pub comp_mode: spq_mode,
    pub comp_cb: qed_spq_comp_cb,
    pub /: *mut *mut qed_spq_comp_done comp_done; / SPQ_MODE_EBLOCK,
// Posted entry for unlimited list entry in EBLOCK mode
    pub post_ent: *mut qed_spq_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_eq {
    pub chain: qed_chain,
    pub /: *mut *mut u8 eq_sb_index; / index within the SB,
    pub /: *mut *mut *mut __le16 p_fw_cons; / ptr to index value,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_consq {
    pub chain: qed_chain,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_spq {
    pub /: *mut *mut spinlock_t lock; / SPQ lock,
    pub unlimited_pending: list_head,
    pub pending: list_head,
    pub completion_pending: list_head,
    pub free_pool: list_head,
    pub chain: qed_chain,
// allocated dma-able memory for spq entries (+ramrod data)
    pub p_phys: dma_addr_t,
    pub p_virt: *mut qed_spq_entry,

// Bitmap for handling out-of-order completions
    pub SPQ_RING_SIZE): DECLARE_BITMAP(p_comp_bitmap,,
    pub comp_bitmap_idx: u8,
// Statistics
    pub unlimited_pending_count: u32,
    pub normal_count: u32,
    pub high_count: u32,
    pub comp_sent_count: u32,
    pub comp_count: u32,
    pub cid: u32,
    pub db_addr_offset: u32,
    pub db_data: core_db_data,
    pub async_comp_cb: [qed_spq_async_comp_cb; MAX_PROTOCOL_TYPE],
}

//
// qed_spq_post(): Posts a Slow hwfn request to FW, or lacking that
// Pends it to the future list.
//
// @p_hwfn: HW device data.
// @p_ent: Ent.
// @fw_return_code: Return code from firmware.
//
// Return: Int.
//
// qed_spq_alloc(): Alloocates & initializes the SPQ and EQ.
//
// @p_hwfn: HW device data.
//
// Return: Int.
//
extern "C" {
    pub fn qed_spq_alloc(p_hwfn: *mut qed_hwfn) -> c_int;
}
//
// qed_spq_setup(): Reset the SPQ to its start state.
//
// @p_hwfn: HW device data.
//
// Return: Void.
//
extern "C" {
    pub fn qed_spq_setup(p_hwfn: *mut qed_hwfn);
}
//
// qed_spq_free(): Deallocates the given SPQ struct.
//
// @p_hwfn: HW device data.
//
// Return: Void.
//
extern "C" {
    pub fn qed_spq_free(p_hwfn: *mut qed_hwfn);
}
//
// qed_spq_get_entry(): Obtain an entrry from the spq
// free pool list.
//
// @p_hwfn: HW device data.
// @pp_ent: PP ENT.
//
// Return: Int.
//
// qed_spq_return_entry(): Return an entry to spq free pool list.
//
// @p_hwfn: HW device data.
// @p_ent: P ENT.
//
// Return: Void.
//
// qed_eq_alloc(): Allocates & initializes an EQ struct.
//
// @p_hwfn: HW device data.
// @num_elem: number of elements in the eq.
//
// Return: Int.
//
extern "C" {
    pub fn qed_eq_alloc(p_hwfn: *mut qed_hwfn, num_elem: u16) -> c_int;
}
//
// qed_eq_setup(): Reset the EQ to its start state.
//
// @p_hwfn: HW device data.
//
// Return: Void.
//
extern "C" {
    pub fn qed_eq_setup(p_hwfn: *mut qed_hwfn);
}
//
// qed_eq_free(): deallocates the given EQ struct.
//
// @p_hwfn: HW device data.
//
// Return: Void.
//
extern "C" {
    pub fn qed_eq_free(p_hwfn: *mut qed_hwfn);
}
//
// qed_eq_prod_update(): update the FW with default EQ producer.
//
// @p_hwfn: HW device data.
// @prod: Prod.
//
// Return: Void.
//
// qed_eq_completion(): Completes currently pending EQ elements.
//
// @p_hwfn: HW device data.
// @cookie: Cookie.
//
// Return: Int.
//
// qed_spq_completion(): Completes a single event.
//
// @p_hwfn: HW device data.
// @echo: echo value from cookie (used for determining completion).
// @fw_return_code: FW return code.
// @p_data: data from cookie (used in callback function if applicable).
//
// Return: Int.
//
// qed_spq_get_cid(): Given p_hwfn, return cid for the hwfn's SPQ.
//
// @p_hwfn: HW device data.
//
// Return: u32 - SPQ CID.
//
extern "C" {
    pub fn qed_spq_get_cid(p_hwfn: *mut qed_hwfn) -> u32;
}
//
// qed_consq_alloc(): Allocates & initializes an ConsQ struct.
//
// @p_hwfn: HW device data.
//
// Return: Int.
//
extern "C" {
    pub fn qed_consq_alloc(p_hwfn: *mut qed_hwfn) -> c_int;
}
//
// qed_consq_setup(): Reset the ConsQ to its start state.
//
// @p_hwfn: HW device data.
//
// Return Void.
//
extern "C" {
    pub fn qed_consq_setup(p_hwfn: *mut qed_hwfn);
}
//
// qed_consq_free(): deallocates the given ConsQ struct.
//
// @p_hwfn: HW device data.
//
// Return Void.
//
extern "C" {
    pub fn qed_consq_free(p_hwfn: *mut qed_hwfn);
}
extern "C" {
    pub fn qed_spq_pend_post(p_hwfn: *mut qed_hwfn) -> c_int;
}
// Slow-hwfn low-level commands (Ramrods) function definitions.
pub const QED_SP_EQ_COMPLETION: c_uint = 0x01;
pub const QED_SP_CQE_COMPLETION: c_uint = 0x02;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_sp_init_data {
    pub cid: u32,
    pub opaque_fid: u16,
// Information regarding operation upon sending & completion
    pub comp_mode: spq_mode,
    pub p_comp_data: *mut qed_spq_comp_cb,
}

//
// qed_sp_destroy_request(): Returns a SPQ entry to the pool / frees the
// entry if allocated. Should be called on in error
// flows after initializing the SPQ entry
// and before posting it.
//
// @p_hwfn: HW device data.
// @p_ent: Ent.
//
// Return: Void.
//
// qed_sp_pf_start(): PF Function Start Ramrod.
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
// @p_tunn: P_tunn.
// @allow_npar_tx_switch: Allow NPAR TX Switch.
//
// Return: Int.
//
// This ramrod is sent to initialize a physical function (PF). It will
// configure the function related parameters and write its completion to the
// event ring specified in the parameters.
//
// Ramrods complete on the common event ring for the PF. This ring is
// allocated by the driver on host memory and its parameters are written
// to the internal RAM of the UStorm by the Function Start Ramrod.
//
// qed_sp_pf_update(): PF Function Update Ramrod.
//
// @p_hwfn: HW device data.
//
// Return: Int.
//
// This ramrod updates function-related parameters. Every parameter can be
// updated independently, according to configuration flags.
//
extern "C" {
    pub fn qed_sp_pf_update(p_hwfn: *mut qed_hwfn) -> c_int;
}
//
// qed_sp_pf_update_stag(): Update firmware of new outer tag.
//
// @p_hwfn: HW device data.
//
// Return: Int.
//
extern "C" {
    pub fn qed_sp_pf_update_stag(p_hwfn: *mut qed_hwfn) -> c_int;
}
//
// qed_sp_pf_update_ufp(): PF ufp update Ramrod.
//
// @p_hwfn: HW device data.
//
// Return: Int.
//
extern "C" {
    pub fn qed_sp_pf_update_ufp(p_hwfn: *mut qed_hwfn) -> c_int;
}
extern "C" {
    pub fn qed_sp_pf_stop(p_hwfn: *mut qed_hwfn) -> c_int;
}
//
// qed_sp_heartbeat_ramrod(): Send empty Ramrod.
//
// @p_hwfn: HW device data.
//
// Return: Int.
//
extern "C" {
    pub fn qed_sp_heartbeat_ramrod(p_hwfn: *mut qed_hwfn) -> c_int;
}
