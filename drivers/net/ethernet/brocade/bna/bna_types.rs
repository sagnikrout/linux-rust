//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/brocade/bna/bna_types.h
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
// Linux network driver for QLogic BR-series Converged Network Adapter.
//
// Copyright (c) 2005-2014 Brocade Communications Systems, Inc.
// Copyright (c) 2014-2015 QLogic Corporation
// All rights reserved
// www.qlogic.com
//

// Forward declarations
// Enums, primitive data types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bna_status {
    BNA_STATUS_T_DISABLED	= 0,
    BNA_STATUS_T_ENABLED	= 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bna_cleanup_type {
    BNA_HARD_CLEANUP	= 0,
    BNA_SOFT_CLEANUP	= 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bna_cb_status {
    BNA_CB_SUCCESS		= 0,
    BNA_CB_FAIL		= 1,
    BNA_CB_INTERRUPT	= 2,
    BNA_CB_BUSY		= 3,
    BNA_CB_INVALID_MAC	= 4,
    BNA_CB_MCAST_LIST_FULL	= 5,
    BNA_CB_UCAST_CAM_FULL	= 6,
    BNA_CB_WAITING		= 7,
    BNA_CB_NOT_EXEC		= 8
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bna_res_type {
    BNA_RES_T_MEM		= 1,
    BNA_RES_T_INTR		= 2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bna_mem_type {
    BNA_MEM_T_KVA		= 1,
    BNA_MEM_T_DMA		= 2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bna_intr_type {
    BNA_INTR_T_INTX		= 1,
    BNA_INTR_T_MSIX		= 2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bna_res_req_type {
    BNA_RES_MEM_T_COM		= 0,
    BNA_RES_MEM_T_ATTR		= 1,
    BNA_RES_MEM_T_FWTRC		= 2,
    BNA_RES_MEM_T_STATS		= 3,
    BNA_RES_T_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bna_mod_res_req_type {
    BNA_MOD_RES_MEM_T_TX_ARRAY	= 0,
    BNA_MOD_RES_MEM_T_TXQ_ARRAY	= 1,
    BNA_MOD_RES_MEM_T_RX_ARRAY	= 2,
    BNA_MOD_RES_MEM_T_RXP_ARRAY	= 3,
    BNA_MOD_RES_MEM_T_RXQ_ARRAY	= 4,
    BNA_MOD_RES_MEM_T_UCMAC_ARRAY	= 5,
    BNA_MOD_RES_MEM_T_MCMAC_ARRAY	= 6,
    BNA_MOD_RES_MEM_T_MCHANDLE_ARRAY = 7,
    BNA_MOD_RES_T_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bna_tx_res_req_type {
    BNA_TX_RES_MEM_T_TCB	= 0,
    BNA_TX_RES_MEM_T_UNMAPQ	= 1,
    BNA_TX_RES_MEM_T_QPT	= 2,
    BNA_TX_RES_MEM_T_SWQPT	= 3,
    BNA_TX_RES_MEM_T_PAGE	= 4,
    BNA_TX_RES_MEM_T_IBIDX	= 5,
    BNA_TX_RES_INTR_T_TXCMPL = 6,
    BNA_TX_RES_T_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bna_rx_mem_type {
    BNA_RX_RES_MEM_T_CCB		= 0,	/* CQ context */
    BNA_RX_RES_MEM_T_RCB		= 1,	/* CQ context */
    BNA_RX_RES_MEM_T_UNMAPHQ	= 2,
    BNA_RX_RES_MEM_T_UNMAPDQ	= 3,
    BNA_RX_RES_MEM_T_CQPT		= 4,
    BNA_RX_RES_MEM_T_CSWQPT		= 5,
    BNA_RX_RES_MEM_T_CQPT_PAGE	= 6,
    BNA_RX_RES_MEM_T_HQPT		= 7,
    BNA_RX_RES_MEM_T_DQPT		= 8,
    BNA_RX_RES_MEM_T_HSWQPT		= 9,
    BNA_RX_RES_MEM_T_DSWQPT		= 10,
    BNA_RX_RES_MEM_T_DPAGE		= 11,
    BNA_RX_RES_MEM_T_HPAGE		= 12,
    BNA_RX_RES_MEM_T_IBIDX		= 13,
    BNA_RX_RES_MEM_T_RIT		= 14,
    BNA_RX_RES_T_INTR		= 15,
    BNA_RX_RES_T_MAX		= 16
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bna_tx_type {
    BNA_TX_T_REGULAR	= 0,
    BNA_TX_T_LOOPBACK	= 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bna_tx_flags {
    BNA_TX_F_ENET_STARTED	= 1,
    BNA_TX_F_ENABLED	= 2,
    BNA_TX_F_BW_UPDATED	= 8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bna_tx_mod_flags {
    BNA_TX_MOD_F_ENET_STARTED	= 1,
    BNA_TX_MOD_F_ENET_LOOPBACK	= 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bna_rx_type {
    BNA_RX_T_REGULAR	= 0,
    BNA_RX_T_LOOPBACK	= 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bna_rxp_type {
    BNA_RXP_SINGLE		= 1,
    BNA_RXP_SLR		= 2,
    BNA_RXP_HDS		= 3
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bna_rxmode {
    BNA_RXMODE_PROMISC	= 1,
    BNA_RXMODE_DEFAULT	= 2,
    BNA_RXMODE_ALLMULTI	= 4
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bna_rx_event {
    RX_E_START			= 1,
    RX_E_STOP			= 2,
    RX_E_FAIL			= 3,
    RX_E_STARTED			= 4,
    RX_E_STOPPED			= 5,
    RX_E_RXF_STARTED		= 6,
    RX_E_RXF_STOPPED		= 7,
    RX_E_CLEANUP_DONE		= 8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bna_rx_flags {
    BNA_RX_F_ENET_STARTED	= 1,
    BNA_RX_F_ENABLED	= 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bna_rx_mod_flags {
    BNA_RX_MOD_F_ENET_STARTED	= 1,
    BNA_RX_MOD_F_ENET_LOOPBACK	= 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bna_rxf_event {
    RXF_E_START			= 1,
    RXF_E_STOP			= 2,
    RXF_E_FAIL			= 3,
    RXF_E_CONFIG			= 4,
    RXF_E_FW_RESP			= 7,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bna_enet_type {
    BNA_ENET_T_REGULAR		= 0,
    BNA_ENET_T_LOOPBACK_INTERNAL	= 1,
    BNA_ENET_T_LOOPBACK_EXTERNAL	= 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bna_link_status {
    BNA_LINK_DOWN		= 0,
    BNA_LINK_UP		= 1,
    BNA_CEE_UP		= 2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bna_ethport_flags {
    BNA_ETHPORT_F_ADMIN_UP		= 1,
    BNA_ETHPORT_F_PORT_ENABLED	= 2,
    BNA_ETHPORT_F_RX_STARTED	= 4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bna_enet_flags {
    BNA_ENET_F_IOCETH_READY		= 1,
    BNA_ENET_F_ENABLED		= 2,
    BNA_ENET_F_PAUSE_CHANGED	= 4,
    BNA_ENET_F_MTU_CHANGED		= 8
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bna_rss_flags {
    BNA_RSS_F_RIT_PENDING		= 1,
    BNA_RSS_F_CFG_PENDING		= 2,
    BNA_RSS_F_STATUS_PENDING	= 4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bna_mod_flags {
    BNA_MOD_F_INIT_DONE		= 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bna_pkt_rates {
    BNA_PKT_RATE_10K		= 10000,
    BNA_PKT_RATE_20K		= 20000,
    BNA_PKT_RATE_30K		= 30000,
    BNA_PKT_RATE_40K		= 40000,
    BNA_PKT_RATE_50K		= 50000,
    BNA_PKT_RATE_60K		= 60000,
    BNA_PKT_RATE_70K		= 70000,
    BNA_PKT_RATE_80K		= 80000,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bna_dim_load_types {
    BNA_LOAD_T_HIGH_4		= 0, /* 80K <= r */
    BNA_LOAD_T_HIGH_3		= 1, /* 60K <= r < 80K */
    BNA_LOAD_T_HIGH_2		= 2, /* 50K <= r < 60K */
    BNA_LOAD_T_HIGH_1		= 3, /* 40K <= r < 50K */
    BNA_LOAD_T_LOW_1		= 4, /* 30K <= r < 40K */
    BNA_LOAD_T_LOW_2		= 5, /* 20K <= r < 30K */
    BNA_LOAD_T_LOW_3		= 6, /* 10K <= r < 20K */
    BNA_LOAD_T_LOW_4		= 7, /* r < 10K */
    BNA_LOAD_T_MAX			= 8
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bna_dim_bias_types {
    BNA_BIAS_T_SMALL		= 0, /* small pkts > (large pkts * 2) */
    BNA_BIAS_T_LARGE		= 1, /* Not BNA_BIAS_T_SMALL */
    BNA_BIAS_T_MAX			= 2
}

pub const BNA_MAX_NAME_SIZE: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bna_ident {
    pub id: c_int,
    pub name: [c_char; BNA_MAX_NAME_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bna_mac {
// This should be the first one
    pub qe: list_head,
    pub addr: [u8; ETH_ALEN],
    pub handle: *mut bna_mcam_handle,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bna_mem_descr {
    pub len: u32,
    pub kva: *mut c_void,
    pub dma: bna_dma_addr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bna_mem_info {
    pub mem_type: bna_mem_type,
    pub len: u32,
    pub num: u32,
    pub /: *mut *mut u32 align_sz; / 0/1 = no alignment,
    pub mdl: *mut bna_mem_descr,
    pub /: *mut *mut *mut void cookie; / For bnad to unmap dma later,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bna_intr_descr {
    pub vector: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bna_intr_info {
    pub intr_type: bna_intr_type,
    pub num: c_int,
    pub idl: *mut bna_intr_descr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union bna_res_u {
    pub mem_info: bna_mem_info,
    pub intr_info: bna_intr_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bna_res_info {
    pub res_type: bna_res_type,
    pub res_u: bna_res_u,
}

// HW QPT
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bna_qpt {
    pub hw_qpt_ptr: bna_dma_addr,
    pub kv_qpt_ptr: *mut c_void,
    pub page_count: u32,
    pub page_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bna_attr {
    pub fw_query_complete: bool,
    pub num_txq: c_int,
    pub num_rxp: c_int,
    pub num_ucmac: c_int,
    pub num_mcmac: c_int,
    pub max_rit_size: c_int,
}

// IOCEth
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bna_ioceth {
    pub e): *mut *mut *mut void (fsm)(struct bna_ioceth s, enum bna_ioceth_event,
    pub ioc: bfa_ioc,
    pub attr: bna_attr,
    pub msgq_cmd: bfa_msgq_cmd_entry,
    pub attr_req: bfi_enet_attr_req,
    pub bnad): *mut *mut void (stop_cbfn)(struct bnad,
    pub stop_cbarg: *mut bnad,
    pub bna: *mut bna,
}

// Enet
// Pause configuration
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bna_pause_config {
    pub tx_pause: bna_status,
    pub rx_pause: bna_status,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bna_enet {
    pub e): *mut *mut *mut void (fsm)(struct bna_enet s, enum bna_enet_event,
    pub flags: bna_enet_flags,
    pub type: bna_enet_type,
    pub pause_config: bna_pause_config,
    pub mtu: c_int,
// Callback for bna_enet_disable(), enet_stop()
    pub ): *mut *mut void (stop_cbfn)(void,
    pub stop_cbarg: *mut c_void,
// Callback for bna_enet_mtu_set()
    pub ): *mut *mut void (mtu_cbfn)(struct bnad,
    pub chld_stop_wc: bfa_wc,
    pub msgq_cmd: bfa_msgq_cmd_entry,
    pub pause_req: bfi_enet_set_pause_req,
    pub bna: *mut bna,
}

// Ethport
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bna_ethport {
    pub e): *mut *mut *mut void (fsm)(struct bna_ethport s, enum bna_ethport_event,
    pub flags: bna_ethport_flags,
    pub link_status: bna_link_status,
    pub rx_started_count: c_int,
    pub ): *mut *mut void (stop_cbfn)(struct bna_enet,
    pub bna_cb_status): *mut *mut *mut void (adminup_cbfn)(struct bnad , enum,
    pub bna_link_status): *mut *mut *mut void (link_cbfn)(struct bnad , enum,
    pub msgq_cmd: bfa_msgq_cmd_entry,
    pub admin_req: bfi_enet_enable_req,
    pub lpbk_req: bfi_enet_diag_lb_req,
    pub bfi_enet_cmd: },
    pub bna: *mut bna,
}

// Interrupt Block
// Doorbell structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bna_ib_dbell {
    pub doorbell_addr: *mut void __iomem,
    pub doorbell_ack: u32,
}

// IB structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bna_ib {
    pub ib_seg_host_addr: bna_dma_addr,
    pub ib_seg_host_addr_kva: *mut c_void,
    pub door_bell: bna_ib_dbell,
    pub intr_type: bna_intr_type,
    pub intr_vector: c_int,
    pub /: *mut *mut u8 coalescing_timeo; / Unit is 5usec.,
    pub interpkt_count: c_int,
    pub interpkt_timeo: c_int,
}

// Tx object
// Tx datapath control structure

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bna_tcb {
// Fast path
    pub sw_qpt: *mut c_void,
    pub sw_q: *mut c_void,
    pub unmap_q: *mut c_void,
    pub producer_index: u32,
    pub consumer_index: u32,
    pub hw_consumer_index: *mut volatile u32,
    pub q_depth: u32,
    pub q_dbell: *mut void __iomem,
    pub i_dbell: *mut bna_ib_dbell,
// Control path
    pub txq: *mut bna_txq,
    pub bnad: *mut bnad,
    pub /: *mut *mut *mut void priv; / BNAD's cookie,
    pub intr_type: bna_intr_type,
    pub intr_vector: c_int,
    pub /: *mut *mut u8 priority; / Current priority,
    pub /: *mut *mut unsigned long flags; / Used by bnad as required,
    pub id: c_int,
    pub name: [c_char; BNA_Q_NAME_SIZE],
}

// TxQ QPT and configuration
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bna_txq {
// This should be the first one
    pub qe: list_head,
    pub priority: u8,
    pub qpt: bna_qpt,
    pub tcb: *mut bna_tcb,
    pub ib: bna_ib,
    pub tx: *mut bna_tx,
    pub hw_id: c_int,
    pub tx_packets: u64,
    pub tx_bytes: u64,
}

// Tx object
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bna_tx {
// This should be the first one
    pub qe: list_head,
    pub rid: c_int,
    pub hw_id: c_int,
    pub e): *mut *mut *mut void (fsm)(struct bna_tx s, enum bna_tx_event,
    pub flags: bna_tx_flags,
    pub type: bna_tx_type,
    pub num_txq: c_int,
    pub txq_q: list_head,
    pub txf_vlan_id: u16,
// Tx event handlers
    pub ): *mut *mut *mut void (tcb_setup_cbfn)(struct bnad , struct bna_tcb,
    pub ): *mut *mut *mut void (tcb_destroy_cbfn)(struct bnad , struct bna_tcb,
    pub ): *mut *mut *mut void (tx_stall_cbfn)(struct bnad , struct bna_tx,
    pub ): *mut *mut *mut void (tx_resume_cbfn)(struct bnad , struct bna_tx,
    pub ): *mut *mut *mut void (tx_cleanup_cbfn)(struct bnad , struct bna_tx,
// callback for bna_tx_disable(), bna_tx_stop()
    pub tx): *mut *mut *mut void (stop_cbfn)(void arg, struct bna_tx,
    pub stop_cbarg: *mut c_void,
    pub msgq_cmd: bfa_msgq_cmd_entry,
    pub cfg_req: bfi_enet_tx_cfg_req,
    pub req: bfi_enet_req,
    pub cfg_rsp: bfi_enet_tx_cfg_rsp,
    pub bfi_enet_cmd: },
    pub bna: *mut bna,
    pub /: *mut *mut *mut void priv; / bnad's cookie,
}

// Tx object configuration used during creation
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bna_tx_config {
    pub num_txq: c_int,
    pub txq_depth: c_int,
    pub coalescing_timeo: c_int,
    pub tx_type: bna_tx_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bna_tx_event_cbfn {
// Optional
    pub ): *mut *mut *mut void (tcb_setup_cbfn)(struct bnad , struct bna_tcb,
    pub ): *mut *mut *mut void (tcb_destroy_cbfn)(struct bnad , struct bna_tcb,
// Mandatory
    pub ): *mut *mut *mut void (tx_stall_cbfn)(struct bnad , struct bna_tx,
    pub ): *mut *mut *mut void (tx_resume_cbfn)(struct bnad , struct bna_tx,
    pub ): *mut *mut *mut void (tx_cleanup_cbfn)(struct bnad , struct bna_tx,
}

// Tx module - keeps track of free, active tx objects
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bna_tx_mod {
    pub /: *mut *mut *mut bna_tx tx; / BFI_MAX_TXQ entries,
    pub /: *mut *mut *mut bna_txq txq; / BFI_MAX_TXQ entries,
    pub tx_free_q: list_head,
    pub tx_active_q: list_head,
    pub txq_free_q: list_head,
// callback for bna_tx_mod_stop()
    pub enet): *mut *mut void (stop_cbfn)(struct bna_enet,
    pub tx_stop_wc: bfa_wc,
    pub flags: bna_tx_mod_flags,
    pub prio_map: u8,
    pub default_prio: c_int,
    pub iscsi_over_cee: c_int,
    pub iscsi_prio: c_int,
    pub prio_reconfigured: c_int,
    pub rid_mask: u32,
    pub bna: *mut bna,
}

// Rx object
// Rx datapath control structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bna_rcb {
// Fast path
    pub sw_qpt: *mut c_void,
    pub sw_q: *mut c_void,
    pub unmap_q: *mut c_void,
    pub producer_index: u32,
    pub consumer_index: u32,
    pub q_depth: u32,
    pub q_dbell: *mut void __iomem,
// Control path
    pub rxq: *mut bna_rxq,
    pub ccb: *mut bna_ccb,
    pub bnad: *mut bnad,
    pub /: *mut *mut *mut void priv; / BNAD's cookie,
    pub flags: c_ulong,
    pub id: c_int,
}

// RxQ structure - QPT, configuration
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bna_rxq {
    pub qe: list_head,
    pub buffer_size: c_int,
    pub q_depth: c_int,
    pub num_vecs: u32,
    pub multi_buffer: bna_status,
    pub qpt: bna_qpt,
    pub rcb: *mut bna_rcb,
    pub rxp: *mut bna_rxp,
    pub rx: *mut bna_rx,
    pub hw_id: c_int,
    pub rx_packets: u64,
    pub rx_bytes: u64,
    pub rx_packets_with_error: u64,
    pub rxbuf_alloc_failed: u64,
    pub rxbuf_map_failed: u64,
}

// RxQ pair
#[repr(C)]
#[derive(Copy, Clone)]
pub union bna_rxq_u {
    pub hdr: *mut bna_rxq,
    pub data: *mut bna_rxq,
    pub hds: },
    pub small: *mut bna_rxq,
    pub large: *mut bna_rxq,
    pub slr: },
    pub only: *mut bna_rxq,
    pub reserved: *mut bna_rxq,
    pub single: },
}

// Packet rate for Dynamic Interrupt Moderation
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bna_pkt_rate {
    pub small_pkt_cnt: u32,
    pub large_pkt_cnt: u32,
}

// Completion control structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bna_ccb {
// Fast path
    pub sw_qpt: *mut c_void,
    pub sw_q: *mut c_void,
    pub producer_index: u32,
    pub hw_producer_index: *mut volatile u32,
    pub q_depth: u32,
    pub i_dbell: *mut bna_ib_dbell,
    pub rcb: [*mut bna_rcb; 2],
    pub /: *mut *mut *mut void ctrl; / For bnad,
    pub pkt_rate: bna_pkt_rate,
    pub pkts_una: u32,
    pub bytes_per_intr: u32,
// Control path
    pub cq: *mut bna_cq,
    pub bnad: *mut bnad,
    pub /: *mut *mut *mut void priv; / BNAD's cookie,
    pub intr_type: bna_intr_type,
    pub intr_vector: c_int,
    pub /: *mut *mut u8 rx_coalescing_timeo; / For NAPI,
    pub id: c_int,
    pub name: [c_char; BNA_Q_NAME_SIZE],
}

// CQ QPT, configuration
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bna_cq {
    pub qpt: bna_qpt,
    pub ccb: *mut bna_ccb,
    pub ib: bna_ib,
    pub rx: *mut bna_rx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bna_rss_config {
    pub hash_type: bfi_enet_rss_type,
    pub hash_mask: u8,
    pub toeplitz_hash_key: [u32; BFI_ENET_RSS_KEY_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bna_hds_config {
    pub hdr_type: bfi_enet_hds_type,
    pub forced_offset: c_int,
}

// Rx object configuration used during creation
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bna_rx_config {
    pub rx_type: bna_rx_type,
    pub num_paths: c_int,
    pub rxp_type: bna_rxp_type,
    pub coalescing_timeo: c_int,
//
// Small/Large (or Header/Data) buffer size to be configured
// for SLR and HDS queue type.
//
    pub frame_size: u32,
// header or small queue
    pub q1_depth: u32,
    pub q1_buf_size: u32,
// data or large queue
    pub q0_depth: u32,
    pub q0_buf_size: u32,
    pub q0_num_vecs: u32,
    pub q0_multi_buf: bna_status,
    pub rss_status: bna_status,
    pub rss_config: bna_rss_config,
    pub hds_config: bna_hds_config,
    pub vlan_strip_status: bna_status,
}

// Rx Path structure - one per MSIX vector/CPU
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bna_rxp {
// This should be the first one
    pub qe: list_head,
    pub type: bna_rxp_type,
    pub rxq: bna_rxq_u,
    pub cq: bna_cq,
    pub rx: *mut bna_rx,
// MSI-x vector number for configuring RSS
    pub vector: c_int,
    pub hw_id: c_int,
}

// RxF structure (hardware Rx Function)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bna_rxf {
    pub e): *mut *mut *mut void (fsm)(struct bna_rxf s, enum bna_rxf_event,
    pub msgq_cmd: bfa_msgq_cmd_entry,
    pub req: bfi_enet_enable_req,
    pub rss_req: bfi_enet_rss_cfg_req,
    pub rit_req: bfi_enet_rit_req,
    pub vlan_req: bfi_enet_rx_vlan_req,
    pub mcast_add_req: bfi_enet_mcast_add_req,
    pub mcast_del_req: bfi_enet_mcast_del_req,
    pub ucast_req: bfi_enet_ucast_req,
    pub bfi_enet_cmd: },
// callback for bna_rxf_start()
    pub rx): *mut *mut void (start_cbfn) (struct bna_rx,
    pub start_cbarg: *mut bna_rx,
// callback for bna_rxf_stop()
    pub rx): *mut *mut void (stop_cbfn) (struct bna_rx,
    pub stop_cbarg: *mut bna_rx,
//
// callback for:
// bna_rxf_ucast_set()
// bna_rxf_{ucast/mcast}_add(),
// bna_rxf_{ucast/mcast}_del(),
// bna_rxf_mode_set()
//
    pub rx): *mut *mut *mut void (cam_fltr_cbfn)(struct bnad bnad, struct bna_rx,
    pub cam_fltr_cbarg: *mut bnad,
// List of unicast addresses yet to be applied to h/w
    pub ucast_pending_add_q: list_head,
    pub ucast_pending_del_q: list_head,
    pub ucast_pending_mac: *mut bna_mac,
    pub ucast_pending_set: c_int,
// ucast addresses applied to the h/w
    pub ucast_active_q: list_head,
    pub ucast_active_mac: bna_mac,
    pub ucast_active_set: c_int,
// List of multicast addresses yet to be applied to h/w
    pub mcast_pending_add_q: list_head,
    pub mcast_pending_del_q: list_head,
// multicast addresses applied to the h/w
    pub mcast_active_q: list_head,
    pub mcast_handle_q: list_head,
// Rx modes yet to be applied to h/w
    pub rxmode_pending: bna_rxmode,
    pub rxmode_pending_bitmask: bna_rxmode,
// Rx modes applied to h/w
    pub rxmode_active: bna_rxmode,
    pub vlan_pending_bitmask: u8,
    pub vlan_filter_status: bna_status,
    pub 32]: u32 vlan_filter_table[(BFI_ENET_VLAN_ID_MAX) /,
    pub vlan_strip_pending: bool,
    pub vlan_strip_status: bna_status,
    pub rss_pending: bna_rss_flags,
    pub rss_status: bna_status,
    pub rss_cfg: bna_rss_config,
    pub rit: *mut u8,
    pub rit_size: c_int,
    pub rx: *mut bna_rx,
}

// Rx object
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bna_rx {
// This should be the first one
    pub qe: list_head,
    pub rid: c_int,
    pub hw_id: c_int,
    pub e): *mut *mut *mut void (fsm)(struct bna_rx s, enum bna_rx_event,
    pub type: bna_rx_type,
    pub num_paths: c_int,
    pub rxp_q: list_head,
    pub hds_cfg: bna_hds_config,
    pub rxf: bna_rxf,
    pub rx_flags: bna_rx_flags,
    pub msgq_cmd: bfa_msgq_cmd_entry,
    pub cfg_req: bfi_enet_rx_cfg_req,
    pub req: bfi_enet_req,
    pub cfg_rsp: bfi_enet_rx_cfg_rsp,
    pub bfi_enet_cmd: },
// Rx event handlers
    pub ): *mut *mut *mut void (rcb_setup_cbfn)(struct bnad , struct bna_rcb,
    pub ): *mut *mut *mut void (rcb_destroy_cbfn)(struct bnad , struct bna_rcb,
    pub ): *mut *mut *mut void (ccb_setup_cbfn)(struct bnad , struct bna_ccb,
    pub ): *mut *mut *mut void (ccb_destroy_cbfn)(struct bnad , struct bna_ccb,
    pub ): *mut *mut *mut void (rx_stall_cbfn)(struct bnad , struct bna_rx,
    pub ): *mut *mut *mut void (rx_cleanup_cbfn)(struct bnad , struct bna_rx,
    pub ): *mut *mut *mut void (rx_post_cbfn)(struct bnad , struct bna_rx,
// callback for bna_rx_disable(), bna_rx_stop()
    pub rx): *mut *mut *mut void (stop_cbfn)(void arg, struct bna_rx,
    pub stop_cbarg: *mut c_void,
    pub bna: *mut bna,
    pub /: *mut *mut *mut void priv; / bnad's cookie,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bna_rx_event_cbfn {
// Optional
    pub ): *mut *mut *mut void (rcb_setup_cbfn)(struct bnad , struct bna_rcb,
    pub ): *mut *mut *mut void (rcb_destroy_cbfn)(struct bnad , struct bna_rcb,
    pub ): *mut *mut *mut void (ccb_setup_cbfn)(struct bnad , struct bna_ccb,
    pub ): *mut *mut *mut void (ccb_destroy_cbfn)(struct bnad , struct bna_ccb,
    pub ): *mut *mut *mut void (rx_stall_cbfn)(struct bnad , struct bna_rx,
// Mandatory
    pub ): *mut *mut *mut void (rx_cleanup_cbfn)(struct bnad , struct bna_rx,
    pub ): *mut *mut *mut void (rx_post_cbfn)(struct bnad , struct bna_rx,
}

// Rx module - keeps track of free, active rx objects
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bna_rx_mod {
    pub /: *mut *mut *mut bna bna; / back pointer to parent,
    pub /: *mut *mut *mut bna_rx rx; / BFI_MAX_RXQ entries,
    pub /: *mut *mut *mut bna_rxp rxp; / BFI_MAX_RXQ entries,
    pub /: *mut *mut *mut bna_rxq rxq; / BFI_MAX_RXQ entries,
    pub rx_free_q: list_head,
    pub rx_active_q: list_head,
    pub rx_free_count: c_int,
    pub rxp_free_q: list_head,
    pub rxp_free_count: c_int,
    pub rxq_free_q: list_head,
    pub rxq_free_count: c_int,
    pub flags: bna_rx_mod_flags,
// callback for bna_rx_mod_stop()
    pub enet): *mut *mut void (stop_cbfn)(struct bna_enet,
    pub rx_stop_wc: bfa_wc,
    pub dim_vector: [u32; BNA_LOAD_T_MAX][BNA_BIAS_T_MAX],
    pub rid_mask: u32,
}

// CAM
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bna_ucam_mod {
    pub /: *mut *mut *mut *mut bna_mac ucmac; / num_ucmac  2 entries,
    pub free_q: list_head,
    pub del_q: list_head,
    pub bna: *mut bna,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bna_mcam_handle {
// This should be the first one
    pub qe: list_head,
    pub handle: c_int,
    pub refcnt: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bna_mcam_mod {
    pub /: *mut *mut *mut *mut bna_mac mcmac; / num_mcmac  2 entries,
    pub /: *mut *mut *mut bna_mcam_handle mchandle; / num_mcmac entries,
    pub free_q: list_head,
    pub del_q: list_head,
    pub free_handle_q: list_head,
    pub bna: *mut bna,
}

// Statistics
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bna_stats {
    pub hw_stats_dma: bna_dma_addr,
    pub hw_stats_kva: *mut bfi_enet_stats,
    pub hw_stats: bfi_enet_stats,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bna_stats_mod {
    pub ioc_ready: bool,
    pub stats_get_busy: bool,
    pub stats_clr_busy: bool,
    pub stats_get_cmd: bfa_msgq_cmd_entry,
    pub stats_clr_cmd: bfa_msgq_cmd_entry,
    pub stats_get: bfi_enet_stats_req,
    pub stats_clr: bfi_enet_stats_req,
}

// BNA
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bna {
    pub ident: bna_ident,
    pub pcidev: bfa_pcidev,
    pub regs: bna_reg,
    pub bits: bna_bit_defn,
    pub stats: bna_stats,
    pub ioceth: bna_ioceth,
    pub cee: bfa_cee,
    pub flash: bfa_flash,
    pub msgq: bfa_msgq,
    pub ethport: bna_ethport,
    pub enet: bna_enet,
    pub stats_mod: bna_stats_mod,
    pub tx_mod: bna_tx_mod,
    pub rx_mod: bna_rx_mod,
    pub ucam_mod: bna_ucam_mod,
    pub mcam_mod: bna_mcam_mod,
    pub mod_flags: bna_mod_flags,
    pub default_mode_rid: c_int,
    pub promisc_rid: c_int,
    pub bnad: *mut bnad,
}
