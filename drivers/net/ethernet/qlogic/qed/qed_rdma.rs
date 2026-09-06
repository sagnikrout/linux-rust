//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/qlogic/qed/qed_rdma.h
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

// Add 1 for header element

// Up to 2^16 XRC Domains are supported, but the actual number of supported XRC
// SRQs is much smaller so there's no need to have that many domains.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_rdma_toggle_bit {
    QED_RDMA_TOGGLE_BIT_CLEAR = 0,
    QED_RDMA_TOGGLE_BIT_SET = 1
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_bmap {
    pub bitmap: *mut c_ulong,
    pub max_count: u32,
    pub name: [c_char; QED_RDMA_MAX_BMAP_NAME],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_rdma_info {
// spin lock to protect bitmaps
    pub lock: spinlock_t,
    pub cq_map: qed_bmap,
    pub pd_map: qed_bmap,
    pub xrcd_map: qed_bmap,
    pub tid_map: qed_bmap,
    pub qp_map: qed_bmap,
    pub srq_map: qed_bmap,
    pub xrc_srq_map: qed_bmap,
    pub cid_map: qed_bmap,
    pub tcp_cid_map: qed_bmap,
    pub real_cid_map: qed_bmap,
    pub dpi_map: qed_bmap,
    pub toggle_bits: qed_bmap,
    pub events: qed_rdma_events,
    pub dev: *mut qed_rdma_device,
    pub port: *mut qed_rdma_port,
    pub last_tid: u32,
    pub num_cnqs: u8,
    pub num_qps: u32,
    pub num_mrs: u32,
    pub num_srqs: u32,
    pub srq_id_offset: u16,
    pub queue_zone_base: u16,
    pub max_queue_zones: u16,
    pub proto: protocol_type,
    pub iwarp: qed_iwarp_info,
    pub active:1: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_rdma_qp {
    pub qp_handle: regpair,
    pub qp_handle_async: regpair,
    pub qpid: u32,
    pub icid: u16,
    pub cur_state: qed_roce_qp_state,
    pub qp_type: qed_rdma_qp_type,
    pub iwarp_state: qed_iwarp_qp_state,
    pub use_srq: bool,
    pub signal_all: bool,
    pub fmr_and_reserved_lkey: bool,
    pub incoming_rdma_read_en: bool,
    pub incoming_rdma_write_en: bool,
    pub incoming_atomic_en: bool,
    pub e2e_flow_control_en: bool,
    pub pd: u16,
    pub pkey: u16,
    pub dest_qp: u32,
    pub mtu: u16,
    pub srq_id: u16,
    pub traffic_class_tos: u8,
    pub hop_limit_ttl: u8,
    pub dpi: u16,
    pub flow_label: u32,
    pub lb_indication: bool,
    pub vlan_id: u16,
    pub ack_timeout: u32,
    pub retry_cnt: u8,
    pub rnr_retry_cnt: u8,
    pub min_rnr_nak_timer: u8,
    pub sqd_async: bool,
    pub sgid: qed_gid,
    pub dgid: qed_gid,
    pub roce_mode: roce_mode,
    pub udp_src_port: u16,
    pub stats_queue: u8,
// requeseter
    pub max_rd_atomic_req: u8,
    pub sq_psn: u32,
    pub sq_cq_id: u16,
    pub sq_num_pages: u16,
    pub sq_pbl_ptr: dma_addr_t,
    pub orq: *mut c_void,
    pub orq_phys_addr: dma_addr_t,
    pub orq_num_pages: u8,
    pub req_offloaded: bool,
    pub has_req: bool,
// responder
    pub max_rd_atomic_resp: u8,
    pub rq_psn: u32,
    pub rq_cq_id: u16,
    pub rq_num_pages: u16,
    pub xrcd_id: u16,
    pub rq_pbl_ptr: dma_addr_t,
    pub irq: *mut c_void,
    pub irq_phys_addr: dma_addr_t,
    pub irq_num_pages: u8,
    pub resp_offloaded: bool,
    pub cq_prod: u32,
    pub has_resp: bool,
    pub remote_mac_addr: [u8; 6],
    pub local_mac_addr: [u8; 6],
    pub shared_queue: *mut c_void,
    pub shared_queue_phys_addr: dma_addr_t,
    pub ep: *mut qed_iwarp_ep,
    pub edpm_mode: u8,
}

extern "C" {
    pub fn qed_rdma_dpm_bar(p_hwfn: *mut qed_hwfn, p_ptt: *mut qed_ptt);
}
extern "C" {
    pub fn qed_rdma_dpm_conf(p_hwfn: *mut qed_hwfn, p_ptt: *mut qed_ptt);
}
extern "C" {
    pub fn qed_rdma_info_alloc(p_hwfn: *mut qed_hwfn) -> c_int;
}
extern "C" {
    pub fn qed_rdma_info_free(p_hwfn: *mut qed_hwfn);
}

extern "C" {
    pub fn qed_rdma_set_fw_mac(p_fw_mac: *mut __le16, p_qed_mac: *const u8);
}
extern "C" {
    pub fn qed_rdma_allocated_qps(p_hwfn: *mut qed_hwfn) -> bool;
}
