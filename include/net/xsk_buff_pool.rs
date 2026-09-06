//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/xsk_buff_pool.h
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
// Copyright(c) 2020 Intel Corporation.

pub const XSK_PRIV_MAX: c_int = 24;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xdp_buff_xsk {
    pub xdp: xdp_buff,
    pub cb: [u8; XSK_PRIV_MAX],
    pub dma: dma_addr_t,
    pub frame_dma: dma_addr_t,
    pub pool: *mut xsk_buff_pool,
    pub list_node: list_head,
    pub __aligned_largest: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xsk_dma_map {
    pub dma_pages: *mut dma_addr_t,
    pub dev: *mut device,
    pub netdev: *mut net_device,
    pub users: refcount_t,
    pub /: *mut *mut list_head list; / Protected by the RTNL_LOCK,
    pub dma_pages_cnt: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xsk_buff_pool {
// Members only used in the control path first.
    pub dev: *mut device,
    pub netdev: *mut net_device,
    pub xsk_tx_list: list_head,
// Protects modifications to the xsk_tx_list
    pub xsk_tx_list_lock: spinlock_t,
    pub users: refcount_t,
    pub umem: *mut xdp_umem,
    pub work: work_struct,
// Protects generic receive in shared and non-shared umem mode.
    pub rx_lock: spinlock_t,
    pub free_list: list_head,
    pub xskb_list: list_head,
    pub heads_cnt: u32,
    pub queue_id: u16,
// Data path members as close to free_heads at the end as possible.
    pub ____cacheline_aligned_in_smp: *mut *mut xsk_queue fq,
    pub cq: *mut xsk_queue,
// For performance reasons, each buff pool has its own array of dma_pages
// even when they are identical.
//
    pub dma_pages: *mut dma_addr_t,
    pub heads: *mut xdp_buff_xsk,
    pub tx_descs: *mut xdp_desc,
    pub chunk_mask: u64,
    pub addrs_cnt: u64,
    pub free_list_cnt: u32,
    pub dma_pages_cnt: u32,
    pub free_heads_cnt: u32,
    pub headroom: u32,
    pub chunk_size: u32,
    pub chunk_shift: u32,
    pub frame_len: u32,
    pub tx_descs_nentries: u32,
    pub reclaim_descs: u32,
    pub tx_zc_pending_descs: u32,
    pub xdp_zc_max_segs: u32,
    pub /: *mut *mut u8 tx_metadata_len; / inherited from umem,
    pub cached_need_wakeup: u8,
    pub uses_need_wakeup: bool,
    pub unaligned: bool,
    pub tx_sw_csum: bool,
    pub addrs: *mut c_void,
// Mutual exclusion of the completion ring in the SKB mode.
// Protect: NAPI TX thread and sendmsg error paths in the SKB
// destructor callback.
//
    pub cq_prod_lock: spinlock_t,
    pub free_heads: [*mut xdp_buff_xsk; ],
}

// Masks for xdp_umem_page flags.
// The low 12-bits of the addr will be 0 since this is the page address, so we
// can use them for flags.
//
pub const XSK_NEXT_PG_CONTIG_SHIFT: c_int = 0;

// AF_XDP core.
extern "C" {
    pub fn xp_destroy(pool: *mut xsk_buff_pool);
}
extern "C" {
    pub fn xp_get_pool(pool: *mut xsk_buff_pool);
}
extern "C" {
    pub fn xp_put_pool(pool: *mut xsk_buff_pool) -> bool;
}
extern "C" {
    pub fn xp_clear_dev(pool: *mut xsk_buff_pool);
}
extern "C" {
    pub fn xp_add_xsk(pool: *mut xsk_buff_pool, xs: *mut xdp_sock);
}
extern "C" {
    pub fn xp_del_xsk(pool: *mut xsk_buff_pool, xs: *mut xdp_sock);
}
// AF_XDP, and XDP core.
extern "C" {
    pub fn xp_free(xskb: *mut xdp_buff_xsk);
}
// AF_XDP ZC drivers, via xdp_sock_buff.h
extern "C" {
    pub fn xp_set_rxq_info(pool: *mut xsk_buff_pool, rxq: *mut xdp_rxq_info);
}
extern "C" {
    pub fn xp_fill_cb(pool: *mut xsk_buff_pool, desc: *mut xsk_cb_desc);
}
extern "C" {
    pub fn xp_dma_unmap(pool: *mut xsk_buff_pool, attrs: c_ulong);
}
extern "C" {
    pub fn xp_alloc_batch(pool: *mut xsk_buff_pool, xdp: *mut xdp_buff, max: u32) -> u32;
}
extern "C" {
    pub fn xp_can_alloc(pool: *mut xsk_buff_pool, count: u32) -> bool;
}
extern "C" {
    pub fn xp_raw_get_dma(pool: *mut xsk_buff_pool, addr: u64) -> dma_addr_t;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xdp_desc_ctx {
    pub dma: dma_addr_t,
    pub meta: *mut xsk_tx_metadata,
}
