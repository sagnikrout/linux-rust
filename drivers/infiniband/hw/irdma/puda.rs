//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/irdma/puda.h
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


// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
// Copyright (c) 2015 - 2020 Intel Corporation
pub const IRDMA_IEQ_MPA_FRAMING: c_int = 6;
pub const IRDMA_TCP_OFFSET: c_int = 40;
pub const IRDMA_IPV4_PAD: c_int = 20;
pub const IRDMA_MRK_BLK_SZ: c_int = 512;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum puda_rsrc_type {
    IRDMA_PUDA_RSRC_TYPE_ILQ = 1,
    IRDMA_PUDA_RSRC_TYPE_IEQ,
    IRDMA_PUDA_RSRC_TYPE_MAX, /* Must be last entry */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum puda_rsrc_complete {
    PUDA_CQ_CREATED = 1,
    PUDA_QP_CREATED,
    PUDA_TX_COMPLETE,
    PUDA_RX_COMPLETE,
    PUDA_HASH_CRC_COMPLETE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_puda_cmpl_info {
    pub qp: *mut irdma_qp_uk,
    pub q_type: u8,
    pub l3proto: u8,
    pub l4proto: u8,
    pub vlan: u16,
    pub payload_len: u32,
    pub /: *mut *mut u32 compl_error; / No_err=0, else major and minor err code,
    pub qp_id: u32,
    pub wqe_idx: u32,
    pub ipv4:1: bool,
    pub smac_valid:1: bool,
    pub vlan_valid:1: bool,
    pub smac: [u8; ETH_ALEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_puda_send_info {
    pub /: *mut *mut u64 paddr; / Physical address,
    pub len: u32,
    pub ah_id: u32,
    pub tcplen: u8,
    pub maclen: u8,
    pub ipv4:1: bool,
    pub do_lpb:1: bool,
    pub scratch: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_puda_buf {
    pub /: *mut *mut list_head list; / MUST be first entry,
    pub /: *mut *mut irdma_dma_mem mem; / DMA memory for the buffer,
    pub /: *mut *mut *mut irdma_puda_buf next; / for alloclist in rsrc struct,
    pub /: *mut *mut irdma_virt_mem buf_mem; / Buffer memory for this buffer,
    pub scratch: *mut c_void,
    pub iph: *mut u8,
    pub tcph: *mut u8,
    pub data: *mut u8,
    pub datalen: u16,
    pub vlan_id: u16,
    pub /: *mut *mut u8 tcphlen; / tcp length in bytes,
    pub /: *mut *mut u8 maclen; / mac length in bytes,
    pub /: *mut *mut u32 totallen; / machlen+iphlen+tcphlen+datalen,
    pub refcount: refcount_t,
    pub hdrlen: u8,
    pub ipv4:1: bool,
    pub vlan_valid:1: bool,
    pub /: *mut *mut bool do_lpb:1; / Loopback buffer,
    pub smac_valid:1: bool,
    pub seqnum: u32,
    pub ah_id: u32,
    pub smac: [u8; ETH_ALEN],
    pub vsi: *mut irdma_sc_vsi,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_puda_rsrc_info {
    pub buf): *mut *mut *mut void (receive)(struct irdma_sc_vsi vsi, struct irdma_puda_buf,
    pub sqwrid): *mut *mut *mut void (xmit_complete)(struct irdma_sc_vsi vsi, void,
    pub /: *mut *mut puda_rsrc_type type; / ILQ or IEQ,
    pub count: u32,
    pub pd_id: u32,
    pub cq_id: u32,
    pub qp_id: u32,
    pub sq_size: u32,
    pub rq_size: u32,
    pub /: *mut *mut u32 tx_buf_cnt; / total bufs allocated will be rq_size + tx_buf_cnt,
    pub buf_size: u16,
    pub stats_idx: u16,
    pub stats_idx_valid:1: bool,
    pub abi_ver: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_puda_rsrc {
    pub cq: irdma_sc_cq,
    pub qp: irdma_sc_qp,
    pub sc_pd: irdma_sc_pd,
    pub dev: *mut irdma_sc_dev,
    pub vsi: *mut irdma_sc_vsi,
    pub cqmem: irdma_dma_mem,
    pub qpmem: irdma_dma_mem,
    pub ilq_mem: irdma_virt_mem,
    pub cmpl: puda_rsrc_complete,
    pub type: puda_rsrc_type,
    pub /: *mut *mut u16 buf_size; /buf must be max datalen + tcpip hdr + mac,
    pub cq_id: u32,
    pub qp_id: u32,
    pub sq_size: u32,
    pub rq_size: u32,
    pub cq_size: u32,
    pub sq_wrtrk_array: *mut irdma_sq_uk_wr_trk_info,
    pub rq_wrid_array: *mut u64,
    pub compl_rxwqe_idx: u32,
    pub rx_wqe_idx: u32,
    pub rxq_invalid_cnt: u32,
    pub tx_wqe_avail_cnt: u32,
    pub txpend: list_head,
    pub /: *mut *mut list_head bufpool; / free buffers pool list for recv and xmit,
    pub alloc_buf_count: u32,
    pub /: *mut *mut u32 avail_buf_count; / snapshot of currently available buffers,
    pub bufpool_lock: spinlock_t,
    pub alloclist: *mut irdma_puda_buf,
    pub buf): *mut *mut *mut void (receive)(struct irdma_sc_vsi vsi, struct irdma_puda_buf,
    pub sqwrid): *mut *mut *mut void (xmit_complete)(struct irdma_sc_vsi vsi, void,
// puda stats
    pub stats_buf_alloc_fail: u64,
    pub stats_pkt_rcvd: u64,
    pub stats_pkt_sent: u64,
    pub stats_rcvd_pkt_err: u64,
    pub stats_sent_pkt_q: u64,
    pub stats_bad_qp_id: u64,
// IEQ stats
    pub fpdu_processed: u64,
    pub bad_seq_num: u64,
    pub crc_err: u64,
    pub pmode_count: u64,
    pub partials_handled: u64,
    pub stats_idx: u16,
    pub check_crc:1: bool,
    pub stats_idx_valid:1: bool,
}

extern "C" {
    pub fn irdma_puda_send(qp: *mut irdma_sc_qp, info: *mut irdma_puda_send_info) -> c_int;
}
extern "C" {
    pub fn irdma_ieq_check_mpacrc(addr: *const c_void, len: u32, val: u32) -> c_int;
}
extern "C" {
    pub fn irdma_ieq_mpa_crc_ae(dev: *mut irdma_sc_dev, qp: *mut irdma_sc_qp);
}
extern "C" {
    pub fn irdma_ieq_update_tcpip_info(buf: *mut irdma_puda_buf, len: u16, seqnum: u32);
}
extern "C" {
    pub fn irdma_cqp_qp_create_cmd(dev: *mut irdma_sc_dev, qp: *mut irdma_sc_qp) -> c_int;
}
extern "C" {
    pub fn irdma_cqp_cq_create_cmd(dev: *mut irdma_sc_dev, cq: *mut irdma_sc_cq) -> c_int;
}
extern "C" {
    pub fn irdma_cqp_qp_destroy_cmd(dev: *mut irdma_sc_dev, qp: *mut irdma_sc_qp) -> c_int;
}
extern "C" {
    pub fn irdma_cqp_cq_destroy_cmd(dev: *mut irdma_sc_dev, cq: *mut irdma_sc_cq);
}
extern "C" {
    pub fn irdma_puda_free_ah(dev: *mut irdma_sc_dev, ah: *mut irdma_sc_ah);
}
extern "C" {
    pub fn irdma_ieq_cleanup_qp(ieq: *mut irdma_puda_rsrc, qp: *mut irdma_sc_qp);
}
