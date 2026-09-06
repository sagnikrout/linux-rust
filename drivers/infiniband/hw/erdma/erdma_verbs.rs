//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/erdma/erdma_verbs.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
// Authors: Cheng Xu <chengyou@linux.alibaba.com>
// Kai Shen <kaishen@linux.alibaba.com>
// Copyright (c) 2020-2022, Alibaba Group.

// RDMA Capability.

pub const ERDMA_MAX_SEND_WR: c_int = 8192;
pub const ERDMA_MAX_ORD: c_int = 128;
pub const ERDMA_MAX_IRD: c_int = 128;
pub const ERDMA_MAX_SGE_RD: c_int = 1;

pub const ERDMA_MAX_SEND_SGE: c_int = 6;
pub const ERDMA_MAX_RECV_SGE: c_int = 1;

pub const ERDMA_MAX_FRMR_PA: c_int = 512;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_user_mmap_entry {
    pub rdma_entry: rdma_user_mmap_entry,
    pub address: u64,
    pub mmap_flag: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_ext_db_info {
    pub enable: bool,
    pub sdb_off: u16,
    pub rdb_off: u16,
    pub cdb_off: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_ucontext {
    pub ibucontext: ib_ucontext,
    pub ext_db: erdma_ext_db_info,
    pub sdb: u64,
    pub rdb: u64,
    pub cdb: u64,
    pub sq_db_mmap_entry: *mut rdma_user_mmap_entry,
    pub rq_db_mmap_entry: *mut rdma_user_mmap_entry,
    pub cq_db_mmap_entry: *mut rdma_user_mmap_entry,
// doorbell records
    pub dbrecords_page_list: list_head,
    pub dbrecords_page_mutex: mutex,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_pd {
    pub ibpd: ib_pd,
    pub pdn: u32,
}

//
// MemoryRegion definition.
//
pub const ERDMA_MAX_INLINE_MTT_ENTRIES: c_int = 4;

pub const ERDMA_MR_MAX_MTT_CNT: c_int = 524288;
pub const ERDMA_MTT_ENTRY_SIZE: c_int = 8;
pub const ERDMA_MR_TYPE_NORMAL: c_int = 0;
pub const ERDMA_MR_TYPE_FRMR: c_int = 1;
pub const ERDMA_MR_TYPE_DMA: c_int = 2;
pub const ERDMA_MR_MTT_0LEVEL: c_int = 0;
pub const ERDMA_MR_MTT_1LEVEL: c_int = 1;

// Hierarchical storage structure for MTT entries
#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_mtt {
    pub buf: *mut u64,
    pub size: usize,
    pub continuous: bool,
    pub buf_dma: dma_addr_t,
    pub dma_addrs: *mut dma_addr_t,
    pub npages: u32,
    pub level: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_mem {
    pub umem: *mut ib_umem,
    pub mtt: *mut erdma_mtt,
    pub page_size: u32,
    pub page_offset: u32,
    pub page_cnt: u32,
    pub mtt_nents: u32,
    pub va: u64,
    pub len: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_mr {
    pub ibmr: ib_mr,
    pub mem: erdma_mem,
    pub type: u8,
    pub access: u8,
    pub valid: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_user_dbrecords_page {
    pub list: list_head,
    pub umem: *mut ib_umem,
    pub va: u64,
    pub refcnt: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_av {
    pub port: u8,
    pub hop_limit: u8,
    pub traffic_class: u8,
    pub sl: u8,
    pub sgid_index: u8,
    pub udp_sport: u16,
    pub flow_label: u32,
    pub dmac: [u8; ETH_ALEN],
    pub dgid: [u8; ERDMA_ROCEV2_GID_SIZE],
    pub ntype: erdma_network_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_ah {
    pub ibah: ib_ah,
    pub av: erdma_av,
    pub ahn: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_uqp {
    pub sq_mem: erdma_mem,
    pub rq_mem: erdma_mem,
    pub sq_dbrec_dma: dma_addr_t,
    pub rq_dbrec_dma: dma_addr_t,
    pub user_dbr_page: *mut erdma_user_dbrecords_page,
    pub rq_offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_kqp {
    pub sq_pi: u16,
    pub sq_ci: u16,
    pub rq_pi: u16,
    pub rq_ci: u16,
    pub swr_tbl: *mut u64,
    pub rwr_tbl: *mut u64,
    pub hw_sq_db: *mut void __iomem,
    pub hw_rq_db: *mut void __iomem,
    pub sq_buf: *mut c_void,
    pub sq_buf_dma_addr: dma_addr_t,
    pub rq_buf: *mut c_void,
    pub rq_buf_dma_addr: dma_addr_t,
    pub sq_dbrec: *mut c_void,
    pub rq_dbrec: *mut c_void,
    pub sq_dbrec_dma: dma_addr_t,
    pub rq_dbrec_dma: dma_addr_t,
    pub sig_all: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum erdma_qps_iwarp {
    ERDMA_QPS_IWARP_IDLE = 0,
    ERDMA_QPS_IWARP_RTR = 1,
    ERDMA_QPS_IWARP_RTS = 2,
    ERDMA_QPS_IWARP_CLOSING = 3,
    ERDMA_QPS_IWARP_TERMINATE = 4,
    ERDMA_QPS_IWARP_ERROR = 5,
    ERDMA_QPS_IWARP_UNDEF = 6,
    ERDMA_QPS_IWARP_COUNT = 7,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum erdma_qpa_mask_iwarp {
    ERDMA_QPA_IWARP_STATE = (1 << 0),
    ERDMA_QPA_IWARP_LLP_HANDLE = (1 << 2),
    ERDMA_QPA_IWARP_ORD = (1 << 3),
    ERDMA_QPA_IWARP_IRD = (1 << 4),
    ERDMA_QPA_IWARP_SQ_SIZE = (1 << 5),
    ERDMA_QPA_IWARP_RQ_SIZE = (1 << 6),
    ERDMA_QPA_IWARP_MPA = (1 << 7),
    ERDMA_QPA_IWARP_CC = (1 << 8),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum erdma_qps_rocev2 {
    ERDMA_QPS_ROCEV2_RESET = 0,
    ERDMA_QPS_ROCEV2_INIT = 1,
    ERDMA_QPS_ROCEV2_RTR = 2,
    ERDMA_QPS_ROCEV2_RTS = 3,
    ERDMA_QPS_ROCEV2_SQD = 4,
    ERDMA_QPS_ROCEV2_SQE = 5,
    ERDMA_QPS_ROCEV2_ERROR = 6,
    ERDMA_QPS_ROCEV2_COUNT = 7,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum erdma_qpa_mask_rocev2 {
    ERDMA_QPA_ROCEV2_STATE = (1 << 0),
    ERDMA_QPA_ROCEV2_QKEY = (1 << 1),
    ERDMA_QPA_ROCEV2_AV = (1 << 2),
    ERDMA_QPA_ROCEV2_SQ_PSN = (1 << 3),
    ERDMA_QPA_ROCEV2_RQ_PSN = (1 << 4),
    ERDMA_QPA_ROCEV2_DST_QPN = (1 << 5),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum erdma_qp_flags {
    ERDMA_QP_IN_FLUSHING = (1 << 0),
}

pub const ERDMA_QP_ACTIVE: c_int = 0;
pub const ERDMA_QP_PASSIVE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_mod_qp_params_iwarp {
    pub state: erdma_qps_iwarp,
    pub cc: erdma_cc_alg,
    pub qp_type: u8,
    pub pd_len: u8,
    pub irq_size: u32,
    pub orq_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_qp_attrs_iwarp {
    pub state: erdma_qps_iwarp,
    pub cookie: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_mod_qp_params_rocev2 {
    pub state: erdma_qps_rocev2,
    pub qkey: u32,
    pub sq_psn: u32,
    pub rq_psn: u32,
    pub dst_qpn: u32,
    pub av: erdma_av,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union erdma_mod_qp_params {
    pub iwarp: erdma_mod_qp_params_iwarp,
    pub rocev2: erdma_mod_qp_params_rocev2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_qp_attrs_rocev2 {
    pub state: erdma_qps_rocev2,
    pub qkey: u32,
    pub dst_qpn: u32,
    pub av: erdma_av,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_qp_attrs {
    pub /: *mut *mut erdma_cc_alg cc; / Congestion control algorithm,
    pub sq_size: u32,
    pub rq_size: u32,
    pub orq_size: u32,
    pub irq_size: u32,
    pub max_send_sge: u32,
    pub max_recv_sge: u32,
    pub iwarp: erdma_qp_attrs_iwarp,
    pub rocev2: erdma_qp_attrs_rocev2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_qp {
    pub ibqp: ib_qp,
    pub ref: kref,
    pub safe_free: completion,
    pub dev: *mut erdma_dev,
    pub cep: *mut erdma_cep,
    pub state_lock: rw_semaphore,
    pub flags: c_ulong,
    pub reflush_dwork: delayed_work,
    pub kern_qp: erdma_kqp,
    pub user_qp: erdma_uqp,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_kcq_info {
    pub qbuf: *mut c_void,
    pub qbuf_dma_addr: dma_addr_t,
    pub ci: u32,
    pub cmdsn: u32,
    pub notify_cnt: u32,
    pub lock: spinlock_t,
    pub db: *mut u8 __iomem,
    pub dbrec: *mut u64,
    pub dbrec_dma: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_ucq_info {
    pub qbuf_mem: erdma_mem,
    pub user_dbr_page: *mut erdma_user_dbrecords_page,
    pub dbrec_dma: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_cq {
    pub ibcq: ib_cq,
    pub cqn: u32,
    pub depth: u32,
    pub assoc_eqn: u32,
    pub refcount: refcount_t,
    pub free: completion,
    pub kern_cq: erdma_kcq_info,
    pub user_cq: erdma_ucq_info,
}

extern "C" {
    pub fn erdma_qp_get(qp: *mut erdma_qp);
}
extern "C" {
    pub fn erdma_qp_put(qp: *mut erdma_qp);
}
extern "C" {
    pub fn erdma_qp_llp_close(qp: *mut erdma_qp);
}
extern "C" {
    pub fn erdma_qp_cm_drop(qp: *mut erdma_qp);
}
extern "C" {
    pub fn container_of(_arg: ibctx, erdma_ucontext: struct, _arg: ibucontext) -> return;
}
extern "C" {
    pub fn container_of(_arg: pd, erdma_pd: struct, _arg: ibpd) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibmr, erdma_mr: struct, _arg: ibmr) -> return;
}
extern "C" {
    pub fn container_of(_arg: qp, erdma_qp: struct, _arg: ibqp) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibcq, erdma_cq: struct, _arg: ibcq) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibah, erdma_ah: struct, _arg: ibah) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibmmap, erdma_user_mmap_entry: struct, _arg: rdma_entry) -> return;
}
extern "C" {
    pub fn erdma_alloc_ucontext(ibctx: *mut ib_ucontext, data: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn erdma_dealloc_ucontext(ibctx: *mut ib_ucontext);
}
extern "C" {
    pub fn erdma_alloc_pd(ibpd: *mut ib_pd, data: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn erdma_dealloc_pd(ibpd: *mut ib_pd, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn erdma_destroy_qp(ibqp: *mut ib_qp, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn erdma_destroy_cq(ibcq: *mut ib_cq, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn erdma_disassociate_ucontext(ibcontext: *mut ib_ucontext);
}
extern "C" {
    pub fn erdma_req_notify_cq(ibcq: *mut ib_cq, flags: ib_cq_notify_flags) -> c_int;
}
extern "C" {
    pub fn erdma_dereg_mr(ibmr: *mut ib_mr, data: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn erdma_mmap(ctx: *mut ib_ucontext, vma: *mut vm_area_struct) -> c_int;
}
extern "C" {
    pub fn erdma_mmap_free(rdma_entry: *mut rdma_user_mmap_entry);
}
extern "C" {
    pub fn erdma_qp_get_ref(ibqp: *mut ib_qp);
}
extern "C" {
    pub fn erdma_qp_put_ref(ibqp: *mut ib_qp);
}
extern "C" {
    pub fn erdma_poll_cq(ibcq: *mut ib_cq, num_entries: c_int, wc: *mut ib_wc) -> c_int;
}
extern "C" {
    pub fn erdma_remove_cqes_of_qp(ibcq: *mut ib_cq, qpn: u32);
}
extern "C" {
    pub fn erdma_port_event(dev: *mut erdma_dev, reason: ib_event_type);
}
extern "C" {
    pub fn erdma_set_mtu(dev: *mut erdma_dev, mtu: u32);
}
extern "C" {
    pub fn erdma_add_gid(attr: *const ib_gid_attr, context: *mut c_void) -> c_int;
}
extern "C" {
    pub fn erdma_del_gid(attr: *const ib_gid_attr, context: *mut c_void) -> c_int;
}
extern "C" {
    pub fn erdma_query_pkey(ibdev: *mut ib_device, port: u32, index: u16, pkey: *mut u16) -> c_int;
}
extern "C" {
    pub fn erdma_set_av_cfg(av_cfg: *mut erdma_av_cfg, av: *mut erdma_av);
}
extern "C" {
    pub fn erdma_destroy_ah(ibah: *mut ib_ah, flags: u32) -> c_int;
}
extern "C" {
    pub fn erdma_query_ah(ibah: *mut ib_ah, ah_attr: *mut rdma_ah_attr) -> c_int;
}
