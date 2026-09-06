//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/irdma/verbs.h
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
// Copyright (c) 2015 - 2021 Intel Corporation
pub const IRDMA_MAX_SAVED_PHY_PGADDR: c_int = 4;
pub const IRDMA_FLUSH_DELAY_MS: c_int = 20;
pub const IRDMA_PKEY_TBL_SZ: c_int = 1;
pub const IRDMA_DEFAULT_PKEY: c_uint = 0xFFFF;
pub const IRDMA_SHADOW_PGCNT: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_ucontext {
    pub ibucontext: ib_ucontext,
    pub iwdev: *mut irdma_device,
    pub db_mmap_entry: *mut rdma_user_mmap_entry,
    pub cq_reg_mem_list: list_head,
    pub /: *mut *mut spinlock_t cq_reg_mem_list_lock; / protect CQ memory list,
    pub qp_reg_mem_list: list_head,
    pub /: *mut *mut spinlock_t qp_reg_mem_list_lock; / protect QP memory list,
    pub srq_reg_mem_list: list_head,
    pub /: *mut *mut spinlock_t srq_reg_mem_list_lock; / protect SRQ memory list,
    pub abi_ver: c_int,
    pub 1: u8 legacy_mode :,
    pub 1: u8 use_raw_attrs :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_pd {
    pub ibpd: ib_pd,
    pub sc_pd: irdma_sc_pd,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union irdma_sockaddr {
    pub saddr_in: sockaddr_in,
    pub saddr_in6: sockaddr_in6,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_av {
    pub macaddr: [u8; 16],
    pub attrs: rdma_ah_attr,
    pub sgid_addr: irdma_sockaddr,
    pub dgid_addr: irdma_sockaddr,
    pub net_type: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_ah {
    pub ibah: ib_ah,
    pub sc_ah: irdma_sc_ah,
    pub pd: *mut irdma_pd,
    pub av: irdma_av,
    pub sgid_index: u8,
    pub dgid: ib_gid,
    pub list: hlist_node,
    pub refcnt: refcount_t,
    pub /: *mut *mut *mut irdma_ah parent_ah; / AH from cached list,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_hmc_pble {
    pub idx: u32,
    pub addr: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_cq_mr {
    pub cq_pbl: irdma_hmc_pble,
    pub shadow: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_srq_mr {
    pub srq_pbl: irdma_hmc_pble,
    pub shadow: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_qp_mr {
    pub sq_pbl: irdma_hmc_pble,
    pub rq_pbl: irdma_hmc_pble,
    pub shadow: dma_addr_t,
    pub rq_pa: dma_addr_t,
    pub sq_page: *mut page,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_cq_buf {
    pub kmem_buf: irdma_dma_mem,
    pub cq_uk: irdma_cq_uk,
    pub hw: *mut irdma_hw,
    pub list: list_head,
    pub work: work_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_pbl {
    pub list: list_head,
    pub qp_mr: irdma_qp_mr,
    pub cq_mr: irdma_cq_mr,
    pub srq_mr: irdma_srq_mr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_mr {
    pub ibmr: ib_mr,
    pub ibmw: ib_mw,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_srq {
    pub ibsrq: ib_srq,
    pub __aligned(64): irdma_sc_srq sc_srq,
    pub kmem: irdma_dma_mem,
    pub srq_wrid_mem: *mut u64,
    pub refcnt: refcount_t,
    pub /: *mut *mut spinlock_t lock; / for poll srq,
    pub iwpbl: *mut irdma_pbl,
    pub sg_list: *mut irdma_sge,
    pub srq_head: u16,
    pub srq_num: u32,
    pub max_wr: u32,
    pub user_mode:1: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_cq {
    pub ibcq: ib_cq,
    pub sc_cq: irdma_sc_cq,
    pub cq_num: u32,
    pub user_mode: bool,
    pub armed: core::sync::atomic::AtomicI32,
    pub last_notify: irdma_cmpl_notify,
    pub kmem: irdma_dma_mem,
    pub kmem_shadow: irdma_dma_mem,
    pub free_cq: completion,
    pub refcnt: refcount_t,
    pub /: *mut *mut spinlock_t lock; / for poll cq,
    pub resize_list: list_head,
    pub cur_cqe: irdma_cq_poll_info,
    pub cmpl_generated: list_head,
    pub iwpbl: *mut irdma_pbl,
    pub iwpbl_shadow: *mut irdma_pbl,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_cmpl_gen {
    pub list: list_head,
    pub cpi: irdma_cq_poll_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct disconn_work {
    pub work: work_struct,
    pub iwqp: *mut irdma_qp,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_qp_kmode {
    pub dma_mem: irdma_dma_mem,
    pub sq_wrid_mem: *mut irdma_sq_uk_wr_trk_info,
    pub rq_wrid_mem: *mut u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_qp {
    pub ibqp: ib_qp,
    pub sc_qp: irdma_sc_qp,
    pub iwdev: *mut irdma_device,
    pub iwscq: *mut irdma_cq,
    pub iwrcq: *mut irdma_cq,
    pub iwpd: *mut irdma_pd,
    pub push_wqe_mmap_entry: *mut rdma_user_mmap_entry,
    pub push_db_mmap_entry: *mut rdma_user_mmap_entry,
    pub ctx_info: irdma_qp_host_ctx_info,
    pub iwarp_info: irdma_iwarp_offload_info,
    pub roce_info: irdma_roce_offload_info,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum irdma_mmap_flag {
    IRDMA_MMAP_IO_NC,
    IRDMA_MMAP_IO_WC,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_user_mmap_entry {
    pub rdma_entry: rdma_user_mmap_entry,
    pub bar_offset: u64,
    pub mmap_flag: u8,
}

//
// iWARP does not support sendImm, so the presence of Imm data
// must be WriteImm.
//
extern "C" {
    pub fn irdma_mcast_mac(ip_addr: *mut u32, mac: *mut u8, ipv4: bool);
}
extern "C" {
    pub fn irdma_ib_register_device(iwdev: *mut irdma_device) -> c_int;
}
extern "C" {
    pub fn irdma_ib_unregister_device(iwdev: *mut irdma_device);
}
extern "C" {
    pub fn irdma_ib_dealloc_device(ibdev: *mut ib_device);
}
extern "C" {
    pub fn irdma_ib_qp_event(iwqp: *mut irdma_qp, event: irdma_qp_event_type);
}
extern "C" {
    pub fn irdma_generate_flush_completions(iwqp: *mut irdma_qp);
}
extern "C" {
    pub fn irdma_remove_cmpls_list(iwcq: *mut irdma_cq);
}
extern "C" {
    pub fn irdma_generated_cmpls(iwcq: *mut irdma_cq, cq_poll_info: *mut irdma_cq_poll_info) -> c_int;
}
