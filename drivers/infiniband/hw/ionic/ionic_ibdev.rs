//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/ionic/ionic_ibdev.h
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


// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2018-2025, Advanced Micro Devices, Inc.

// Config knobs
pub const IONIC_EQ_DEPTH: c_int = 511;
pub const IONIC_EQ_COUNT: c_int = 32;
pub const IONIC_AQ_DEPTH: c_int = 63;
pub const IONIC_AQ_COUNT: c_int = 4;
pub const IONIC_EQ_ISR_BUDGET: c_int = 10;
pub const IONIC_EQ_WORK_BUDGET: c_int = 1000;
pub const IONIC_MAX_RD_ATOM: c_int = 16;
pub const IONIC_PKEY_TBL_LEN: c_int = 1;
pub const IONIC_GID_TBL_LEN: c_int = 256;
pub const IONIC_MAX_QPID: c_uint = 0xffffff;
pub const IONIC_SPEC_HIGH: c_int = 8;
pub const IONIC_MAX_PD: c_int = 1024;
pub const IONIC_SQCMB_ORDER: c_int = 5;
pub const IONIC_RQCMB_ORDER: c_int = 0;

pub const IONIC_CQ_GRACE: c_int = 100;
pub const IONIC_ROCE_UDP_SPORT: c_int = 28272;
pub const IONIC_DMA_LKEY: c_int = 0;

// resource is not reserved on the device, indicated in tbl_order

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_admin_state {
    IONIC_ADMIN_ACTIVE, /* submitting admin commands to queue */
    IONIC_ADMIN_PAUSED, /* not submitting, but may complete normally */
    IONIC_ADMIN_KILLED, /* not submitting, locally completed */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_admin_flags {
    IONIC_ADMIN_F_BUSYWAIT  = BIT(0),	/* Don't sleep */
    IONIC_ADMIN_F_TEARDOWN  = BIT(1),	/* In destroy path */
    IONIC_ADMIN_F_INTERRUPT = BIT(2),	/* Interruptible w/timeout */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_mmap_flag {
    IONIC_MMAP_WC = BIT(0),
    IONIC_MMAP_PHC = BIT(1),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_mmap_entry {
    pub rdma_entry: rdma_user_mmap_entry,
    pub size: c_ulong,
    pub pfn: c_ulong,
    pub mmap_flags: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_ibdev {
    pub ibdev: ib_device,
    pub lif_cfg: ionic_lif_cfg,
    pub qp_tbl: xarray,
    pub cq_tbl: xarray,
    pub inuse_dbid: ionic_resid_bits,
    pub inuse_pdid: ionic_resid_bits,
    pub inuse_ahid: ionic_resid_bits,
    pub inuse_mrid: ionic_resid_bits,
    pub inuse_qpid: ionic_resid_bits,
    pub inuse_cqid: ionic_resid_bits,
    pub half_cqid_udma_shift: u8,
    pub half_qpid_udma_shift: u8,
    pub next_qpid_udma_idx: u8,
    pub next_mrkey: u8,
    pub reset_work: work_struct,
    pub reset_posted: bool,
    pub reset_cnt: u32,
    pub admin_dwork: delayed_work,
    pub aq_vec: *mut ionic_aq,
    pub admin_state: core::sync::atomic::AtomicI32,
    pub eq_vec: *mut ionic_eq,
    pub hw_stats: *mut ionic_v1_stat,
    pub hw_stats_buf: *mut c_void,
    pub hw_stats_hdrs: *mut rdma_stat_desc,
    pub counter_stats: *mut ionic_counter_stats,
    pub hw_stats_count: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_eq {
    pub dev: *mut ionic_ibdev,
    pub eqid: u32,
    pub intr: u32,
    pub q: ionic_queue,
    pub armed: c_int,
    pub enable: bool,
    pub work: work_struct,
    pub irq: c_int,
    pub name: [c_char; 32],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_admin_wr {
    pub work: completion,
    pub aq_ent: list_head,
    pub wqe: ionic_v1_admin_wqe,
    pub cqe: ionic_v1_cqe,
    pub aq: *mut ionic_aq,
    pub status: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_admin_wr_q {
    pub wr: *mut ionic_admin_wr,
    pub wqe_strides: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_aq {
    pub dev: *mut ionic_ibdev,
    pub vcq: *mut ionic_vcq,
    pub work: work_struct,
    pub admin_state: core::sync::atomic::AtomicI32,
    pub stamp: c_ulong,
    pub armed: bool,
    pub aqid: u32,
    pub cqid: u32,
    pub /: *mut *mut spinlock_t lock; / for posting,
    pub q: ionic_queue,
    pub q_wr: *mut ionic_admin_wr_q,
    pub wr_prod: list_head,
    pub wr_post: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_ctx {
    pub ibctx: ib_ucontext,
    pub dbid: u32,
    pub mmap_dbell: *mut rdma_user_mmap_entry,
    pub mmap_phc: *mut rdma_user_mmap_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_tbl_buf {
    pub tbl_limit: u32,
    pub tbl_pages: u32,
    pub tbl_size: usize,
    pub tbl_buf: *mut __le64,
    pub tbl_dma: dma_addr_t,
    pub page_size_log2: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_pd {
    pub ibpd: ib_pd,
    pub pdid: u32,
    pub flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_cq {
    pub vcq: *mut ionic_vcq,
    pub cqid: u32,
    pub eqid: u32,
    pub /: *mut *mut spinlock_t lock; / for polling,
    pub poll_sq: list_head,
    pub flush: bool,
    pub flush_sq: list_head,
    pub flush_rq: list_head,
    pub ibkill_flush_ent: list_head,
    pub q: ionic_queue,
    pub color: bool,
    pub credit: c_int,
    pub arm_any_prod: u16,
    pub arm_sol_prod: u16,
    pub cq_kref: kref,
    pub cq_rel_comp: completion,
// infrequently accessed, keep at end
    pub umem: *mut ib_umem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_vcq {
    pub ibcq: ib_cq,
    pub cq: [ionic_cq; 2],
    pub udma_mask: u8,
    pub poll_idx: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_sq_meta {
    pub wrid: u64,
    pub len: u32,
    pub seq: u16,
    pub ibop: u8,
    pub ibsts: u8,
    pub remote:1: u8,
    pub signal:1: u8,
    pub local_comp:1: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_rq_meta {
    pub next: *mut ionic_rq_meta,
    pub wrid: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_qp {
    pub ibqp: ib_qp,
    pub state: ib_qp_state,
    pub qpid: u32,
    pub ahid: u32,
    pub sq_cqid: u32,
    pub rq_cqid: u32,
    pub udma_idx: u8,
    pub has_ah:1: u8,
    pub has_sq:1: u8,
    pub has_rq:1: u8,
    pub sig_all:1: u8,
    pub qp_list_counter: list_head,
    pub cq_poll_sq: list_head,
    pub cq_flush_sq: list_head,
    pub cq_flush_rq: list_head,
    pub ibkill_flush_ent: list_head,
    pub /: *mut *mut spinlock_t sq_lock; / for posting and polling,
    pub sq: ionic_queue,
    pub sq_meta: *mut ionic_sq_meta,
    pub sq_msn_idx: *mut u16,
    pub sq_spec: c_int,
    pub sq_old_prod: u16,
    pub sq_msn_prod: u16,
    pub sq_msn_cons: u16,
    pub sq_cmb: u8,
    pub sq_flush: bool,
    pub sq_flush_rcvd: bool,
    pub /: *mut *mut spinlock_t rq_lock; / for posting and polling,
    pub rq: ionic_queue,
    pub rq_meta: *mut ionic_rq_meta,
    pub rq_meta_head: *mut ionic_rq_meta,
    pub rq_spec: c_int,
    pub rq_old_prod: u16,
    pub rq_cmb: u8,
    pub rq_flush: bool,
    pub qp_kref: kref,
    pub qp_rel_comp: completion,
// infrequently accessed, keep at end
    pub sgid_index: c_int,
    pub sq_cmb_order: c_int,
    pub sq_cmb_pgid: u32,
    pub sq_cmb_addr: phys_addr_t,
    pub mmap_sq_cmb: *mut rdma_user_mmap_entry,
    pub sq_umem: *mut ib_umem,
    pub rq_cmb_order: c_int,
    pub rq_cmb_pgid: u32,
    pub rq_cmb_addr: phys_addr_t,
    pub mmap_rq_cmb: *mut rdma_user_mmap_entry,
    pub rq_umem: *mut ib_umem,
    pub dcqcn_profile: c_int,
    pub hdr: *mut ib_ud_header,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_ah {
    pub ibah: ib_ah,
    pub ahid: u32,
    pub sgid_index: c_int,
    pub hdr: ib_ud_header,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_mr {
    pub ibmr: ib_mr,
    pub ibmw: ib_mw,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_counter_stats {
    pub queue_stats_count: c_int,
    pub hdr: *mut ionic_v1_stat,
    pub stats_hdrs: *mut rdma_stat_desc,
    pub counter_ida: ida,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_rdma_counter {
    pub rdma_counter: rdma_counter,
    pub vals: *mut c_void,
    pub qp_list: list_head,
}

extern "C" {
    pub fn container_of(_arg: counter, ionic_rdma_counter: struct, _arg: rdma_counter) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibdev, ionic_ibdev: struct, _arg: ibdev) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibctx, ionic_ctx: struct, _arg: ibctx) -> return;
}
extern "C" {
    pub fn to_ionic_ctx(_arg: uobj->context) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibpd, ionic_pd: struct, _arg: ibpd) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibmr, ionic_mr: struct, _arg: ibmr) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibmw, ionic_mr: struct, _arg: ibmw) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibcq, ionic_vcq: struct, _arg: ibcq) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibqp, ionic_qp: struct, _arg: ibqp) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibah, ionic_ah: struct, _arg: ibah) -> return;
}
extern "C" {
    pub fn ionic_ctx_dbid(_arg: dev, _arg: to_ionic_ctx_uobj(uobj)) -> return;
}
// ionic_admin.c
extern "C" {
    pub fn ionic_admin_post(dev: *mut ionic_ibdev, wr: *mut ionic_admin_wr);
}
extern "C" {
    pub fn ionic_rdma_reset_devcmd(dev: *mut ionic_ibdev) -> c_int;
}
extern "C" {
    pub fn ionic_create_rdma_admin(dev: *mut ionic_ibdev) -> c_int;
}
extern "C" {
    pub fn ionic_destroy_rdma_admin(dev: *mut ionic_ibdev);
}
extern "C" {
    pub fn ionic_kill_rdma_admin(dev: *mut ionic_ibdev, fatal_path: bool);
}
// ionic_controlpath.c
extern "C" {
    pub fn ionic_destroy_cq_common(dev: *mut ionic_ibdev, cq: *mut ionic_cq);
}
extern "C" {
    pub fn ionic_flush_qp(dev: *mut ionic_ibdev, qp: *mut ionic_qp);
}
extern "C" {
    pub fn ionic_notify_flush_cq(cq: *mut ionic_cq);
}
extern "C" {
    pub fn ionic_alloc_ucontext(ibctx: *mut ib_ucontext, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn ionic_dealloc_ucontext(ibctx: *mut ib_ucontext);
}
extern "C" {
    pub fn ionic_mmap(ibctx: *mut ib_ucontext, vma: *mut vm_area_struct) -> c_int;
}
extern "C" {
    pub fn ionic_mmap_free(rdma_entry: *mut rdma_user_mmap_entry);
}
extern "C" {
    pub fn ionic_alloc_pd(ibpd: *mut ib_pd, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn ionic_dealloc_pd(ibpd: *mut ib_pd, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn ionic_query_ah(ibah: *mut ib_ah, ah_attr: *mut rdma_ah_attr) -> c_int;
}
extern "C" {
    pub fn ionic_destroy_ah(ibah: *mut ib_ah, flags: u32) -> c_int;
}
extern "C" {
    pub fn ionic_dereg_mr(ibmr: *mut ib_mr, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn ionic_alloc_mw(ibmw: *mut ib_mw, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn ionic_dealloc_mw(ibmw: *mut ib_mw) -> c_int;
}
extern "C" {
    pub fn ionic_destroy_cq(ibcq: *mut ib_cq, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn ionic_destroy_qp(ibqp: *mut ib_qp, udata: *mut ib_udata) -> c_int;
}
// ionic_datapath.c
extern "C" {
    pub fn ionic_poll_cq(ibcq: *mut ib_cq, nwc: c_int, wc: *mut ib_wc) -> c_int;
}
extern "C" {
    pub fn ionic_req_notify_cq(ibcq: *mut ib_cq, flags: ib_cq_notify_flags) -> c_int;
}
// ionic_hw_stats.c
extern "C" {
    pub fn ionic_stats_init(dev: *mut ionic_ibdev);
}
extern "C" {
    pub fn ionic_stats_cleanup(dev: *mut ionic_ibdev);
}
// ionic_pgtbl.c
extern "C" {
    pub fn ionic_pgtbl_dma(buf: *mut ionic_tbl_buf, va: u64) -> __le64;
}
extern "C" {
    pub fn ionic_pgtbl_off(buf: *mut ionic_tbl_buf, va: u64) -> __be64;
}
extern "C" {
    pub fn ionic_pgtbl_page(buf: *mut ionic_tbl_buf, dma: u64) -> c_int;
}
extern "C" {
    pub fn ionic_pgtbl_unbuf(dev: *mut ionic_ibdev, buf: *mut ionic_tbl_buf);
}
