//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/sw/rxe/rxe_loc.h
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
//
// Copyright (c) 2016 Mellanox Technologies Ltd. All rights reserved.
// Copyright (c) 2015 System Fabric Works, Inc. All rights reserved.
//
// rxe_av.c
extern "C" {
    pub fn rxe_init_av(attr: *mut rdma_ah_attr, av: *mut rxe_av);
}
extern "C" {
    pub fn rxe_av_chk_attr(qp: *mut rxe_qp, attr: *mut rdma_ah_attr) -> c_int;
}
extern "C" {
    pub fn rxe_ah_chk_attr(ah: *mut rxe_ah, attr: *mut rdma_ah_attr) -> c_int;
}
extern "C" {
    pub fn rxe_av_to_attr(av: *mut rxe_av, attr: *mut rdma_ah_attr);
}
extern "C" {
    pub fn rxe_av_fill_ip_info(av: *mut rxe_av, attr: *mut rdma_ah_attr);
}
// rxe_cq.c
extern "C" {
    pub fn rxe_cq_post(cq: *mut rxe_cq, cqe: *mut rxe_cqe, solicited: c_int) -> c_int;
}
extern "C" {
    pub fn rxe_cq_cleanup(elem: *mut rxe_pool_elem);
}
// rxe_mcast.c
extern "C" {
    pub fn rxe_attach_mcast(ibqp: *mut ib_qp, mgid: *mut ib_gid, mlid: u16) -> c_int;
}
extern "C" {
    pub fn rxe_detach_mcast(ibqp: *mut ib_qp, mgid: *mut ib_gid, mlid: u16) -> c_int;
}
extern "C" {
    pub fn rxe_cleanup_mcg(kref: *mut kref);
}
// rxe_mmap.c
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxe_mmap_info {
    pub pending_mmaps: list_head,
    pub context: *mut ib_ucontext,
    pub ref: kref,
    pub obj: *mut c_void,
    pub info: mminfo,
}

extern "C" {
    pub fn rxe_mmap_release(ref: *mut kref);
}
extern "C" {
    pub fn rxe_mmap(context: *mut ib_ucontext, vma: *mut vm_area_struct) -> c_int;
}
// rxe_mr.c
extern "C" {
    pub fn rxe_get_next_key(last_key: u32) -> u8;
}
extern "C" {
    pub fn rxe_mr_init(access: c_int, mr: *mut rxe_mr);
}
extern "C" {
    pub fn rxe_mr_init_dma(access: c_int, mr: *mut rxe_mr);
}
extern "C" {
    pub fn rxe_mr_init_fast(max_pages: c_int, mr: *mut rxe_mr) -> c_int;
}
extern "C" {
    pub fn rxe_flush_pmem_iova(mr: *mut rxe_mr, iova: u64, length: c_uint) -> c_int;
}
extern "C" {
    pub fn rxe_mr_do_atomic_write(mr: *mut rxe_mr, iova: u64, value: u64) -> resp_states;
}
extern "C" {
    pub fn mr_check_range(mr: *mut rxe_mr, iova: u64, length: usize) -> c_int;
}
extern "C" {
    pub fn advance_dma_data(dma: *mut rxe_dma_info, length: c_uint) -> c_int;
}
extern "C" {
    pub fn rxe_invalidate_mr(qp: *mut rxe_qp, key: u32) -> c_int;
}
extern "C" {
    pub fn rxe_reg_fast_mr(qp: *mut rxe_qp, wqe: *mut rxe_send_wqe) -> c_int;
}
extern "C" {
    pub fn rxe_mr_cleanup(elem: *mut rxe_pool_elem);
}
// defined in rxe_mr.c; used in rxe_mr.c and rxe_odp.c
// rxe_mw.c
extern "C" {
    pub fn rxe_alloc_mw(ibmw: *mut ib_mw, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn rxe_dealloc_mw(ibmw: *mut ib_mw) -> c_int;
}
extern "C" {
    pub fn rxe_bind_mw(qp: *mut rxe_qp, wqe: *mut rxe_send_wqe) -> c_int;
}
extern "C" {
    pub fn rxe_invalidate_mw(qp: *mut rxe_qp, rkey: u32) -> c_int;
}
extern "C" {
    pub fn rxe_mw_cleanup(elem: *mut rxe_pool_elem);
}
// rxe_net.c
// rxe_qp.c
extern "C" {
    pub fn rxe_qp_chk_init(rxe: *mut rxe_dev, init: *mut ib_qp_init_attr) -> c_int;
}
extern "C" {
    pub fn rxe_qp_to_init(qp: *mut rxe_qp, init: *mut ib_qp_init_attr) -> c_int;
}
extern "C" {
    pub fn rxe_qp_to_attr(qp: *mut rxe_qp, attr: *mut ib_qp_attr, mask: c_int) -> c_int;
}
extern "C" {
    pub fn rxe_qp_error(qp: *mut rxe_qp);
}
extern "C" {
    pub fn rxe_qp_chk_destroy(qp: *mut rxe_qp) -> c_int;
}
extern "C" {
    pub fn rxe_qp_cleanup(elem: *mut rxe_pool_elem);
}
extern "C" {
    pub fn free_rd_atomic_resource(res: *mut resp_res);
}
extern "C" {
    pub fn retransmit_timer(t: *mut timer_list);
}
extern "C" {
    pub fn rnr_nak_timer(t: *mut timer_list);
}
// rxe_srq.c
extern "C" {
    pub fn rxe_srq_chk_init(rxe: *mut rxe_dev, init: *mut ib_srq_init_attr) -> c_int;
}
extern "C" {
    pub fn rxe_srq_cleanup(elem: *mut rxe_pool_elem);
}
extern "C" {
    pub fn rxe_dealloc(ib_dev: *mut ib_device);
}
extern "C" {
    pub fn rxe_completer(qp: *mut rxe_qp) -> c_int;
}
extern "C" {
    pub fn rxe_requester(qp: *mut rxe_qp) -> c_int;
}
extern "C" {
    pub fn rxe_sender(qp: *mut rxe_qp) -> c_int;
}
extern "C" {
    pub fn rxe_receiver(qp: *mut rxe_qp) -> c_int;
}
// rxe_icrc.c
extern "C" {
    pub fn rxe_icrc_check(skb: *mut sk_buff, pkt: *mut rxe_pkt_info) -> c_int;
}
extern "C" {
    pub fn rxe_icrc_generate(skb: *mut sk_buff, pkt: *mut rxe_pkt_info);
}
extern "C" {
    pub fn rxe_resp_queue_pkt(qp: *mut rxe_qp, skb: *mut sk_buff);
}
extern "C" {
    pub fn rxe_comp_queue_pkt(qp: *mut rxe_qp, skb: *mut sk_buff);
}
// rxe_odp.c

extern "C" {
    pub fn rxe_odp_do_atomic_write(mr: *mut rxe_mr, iova: u64, value: u64) -> resp_states;
}

// rxe-mad.c
