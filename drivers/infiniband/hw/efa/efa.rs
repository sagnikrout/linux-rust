//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/efa/efa.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-2-Clause
//
// Copyright 2018-2026 Amazon.com, Inc. or its affiliates. All rights reserved.
//

pub const EFA_IRQNAME_SIZE: c_int = 40;
pub const EFA_MGMNT_MSIX_VEC_IDX: c_int = 0;
pub const EFA_COMP_EQS_VEC_BASE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_irq {
    pub handler: irq_handler_t,
    pub data: *mut c_void,
    pub irqn: u32,
    pub vector: u32,
    pub affinity_hint_mask: cpumask_t,
    pub name: [c_char; EFA_IRQNAME_SIZE],
}

// Don't use anything other than atomic64
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_stats {
    pub alloc_pd_err: core::sync::atomic::AtomicI64,
    pub create_qp_err: core::sync::atomic::AtomicI64,
    pub create_cq_err: core::sync::atomic::AtomicI64,
    pub reg_mr_err: core::sync::atomic::AtomicI64,
    pub alloc_ucontext_err: core::sync::atomic::AtomicI64,
    pub create_ah_err: core::sync::atomic::AtomicI64,
    pub mmap_err: core::sync::atomic::AtomicI64,
    pub keep_alive_rcvd: core::sync::atomic::AtomicI64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_dev {
    pub ibdev: ib_device,
    pub edev: efa_com_dev,
    pub pdev: *mut pci_dev,
    pub dev_attr: efa_com_get_device_attr_result,
    pub reg_bar_addr: u64,
    pub reg_bar_len: u64,
    pub mem_bar_addr: u64,
    pub mem_bar_len: u64,
    pub db_bar_addr: u64,
    pub db_bar_len: u64,
    pub num_irq_vectors: u32,
    pub admin_msix_vector_idx: u32,
    pub admin_irq: efa_irq,
    pub stats: efa_stats,
// Array of completion EQs
    pub eqs: *mut efa_eq,
    pub neqs: u32,
// Only stores CQs with interrupts enabled
    pub cqs_xa: xarray,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_ucontext {
    pub ibucontext: ib_ucontext,
    pub uarn: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_pd {
    pub ibpd: ib_pd,
    pub pdn: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_mr_interconnect_info {
    pub recv_ic_id: u16,
    pub rdma_read_ic_id: u16,
    pub rdma_recv_ic_id: u16,
    pub 1: u8 recv_ic_id_valid :,
    pub 1: u8 rdma_read_ic_id_valid :,
    pub 1: u8 rdma_recv_ic_id_valid :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_mr {
    pub ibmr: ib_mr,
    pub umem: *mut ib_umem,
    pub ic_info: efa_mr_interconnect_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_cq {
    pub ibcq: ib_cq,
    pub ucontext: *mut efa_ucontext,
    pub dma_addr: dma_addr_t,
    pub cpu_addr: *mut c_void,
    pub mmap_entry: *mut rdma_user_mmap_entry,
    pub db_mmap_entry: *mut rdma_user_mmap_entry,
    pub size: usize,
    pub cq_idx: u16,
// NULL when no interrupts requested
    pub eq: *mut efa_eq,
    pub umem: *mut ib_umem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_comp_cntr {
    pub ibcc: ib_comp_cntr,
    pub comp_umem: *mut ib_umem,
    pub err_umem: *mut ib_umem,
    pub comp_handle: u32,
    pub err_handle: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_qp {
    pub ibqp: ib_qp,
    pub rq_dma_addr: dma_addr_t,
    pub rq_cpu_addr: *mut c_void,
    pub rq_size: usize,
    pub state: ib_qp_state,
// Used for saving mmap_xa entries
    pub sq_db_mmap_entry: *mut rdma_user_mmap_entry,
    pub llq_desc_mmap_entry: *mut rdma_user_mmap_entry,
    pub rq_db_mmap_entry: *mut rdma_user_mmap_entry,
    pub rq_mmap_entry: *mut rdma_user_mmap_entry,
    pub qp_handle: u32,
    pub max_send_wr: u32,
    pub max_recv_wr: u32,
    pub max_send_sge: u32,
    pub max_recv_sge: u32,
    pub max_inline_data: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_ah {
    pub ibah: ib_ah,
    pub ah: u16,
// dest_addr
    pub id: [u8; EFA_GID_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_eq {
    pub eeq: efa_com_eq,
    pub irq: efa_irq,
}

extern "C" {
    pub fn efa_query_port_speed(ibdev: *mut ib_device, port_num: u32, speed: *mut u64) -> c_int;
}
extern "C" {
    pub fn efa_alloc_pd(ibpd: *mut ib_pd, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn efa_dealloc_pd(ibpd: *mut ib_pd, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn efa_destroy_qp(ibqp: *mut ib_qp, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn efa_destroy_cq(ibcq: *mut ib_cq, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn efa_destroy_comp_cntr(ibcc: *mut ib_comp_cntr) -> c_int;
}
extern "C" {
    pub fn efa_dereg_mr(ibmr: *mut ib_mr, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn efa_alloc_ucontext(ibucontext: *mut ib_ucontext, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn efa_dealloc_ucontext(ibucontext: *mut ib_ucontext);
}
extern "C" {
    pub fn efa_mmap_free(rdma_entry: *mut rdma_user_mmap_entry);
}
extern "C" {
    pub fn efa_destroy_ah(ibah: *mut ib_ah, flags: u32) -> c_int;
}
