//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/xdp_sock_drv.h
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
// Interface for implementing AF_XDP zero-copy support in drivers.
// Copyright(c) 2020 Intel Corporation.
//

pub const XDP_UMEM_MIN_CHUNK_SHIFT: c_int = 11;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xsk_cb_desc {
    pub src: *mut c_void,
    pub off: u8,
    pub bytes: u8,
}

extern "C" {
    pub fn xsk_tx_completed(pool: *mut xsk_buff_pool, nb_entries: u32);
}
extern "C" {
    pub fn xsk_tx_peek_desc(pool: *mut xsk_buff_pool, desc: *mut xdp_desc) -> bool;
}
extern "C" {
    pub fn xsk_tx_peek_release_desc_batch(pool: *mut xsk_buff_pool, max: u32) -> u32;
}
extern "C" {
    pub fn xsk_tx_release(pool: *mut xsk_buff_pool);
}
extern "C" {
    pub fn xsk_set_rx_need_wakeup(pool: *mut xsk_buff_pool);
}
extern "C" {
    pub fn xsk_set_tx_need_wakeup(pool: *mut xsk_buff_pool);
}
extern "C" {
    pub fn xsk_clear_rx_need_wakeup(pool: *mut xsk_buff_pool);
}
extern "C" {
    pub fn xsk_clear_tx_need_wakeup(pool: *mut xsk_buff_pool);
}
extern "C" {
    pub fn xsk_uses_need_wakeup(pool: *mut xsk_buff_pool) -> bool;
}
extern "C" {
    pub fn xsk_pool_get_chunk_size(xsk_pool_get_headroom(pool: pool) -) -> return;
}
// Reserve tailroom only for zero-copy pools that opted into
// multi-buffer. The reserved area is used for skb_shared_info,
// matching the XDP core's xdp_data_hard_end() layout.
//
extern "C" {
    pub fn ALIGN_DOWN(_arg: frame_size, _arg: 128) -> return;
}
extern "C" {
    pub fn xp_dma_map(_arg: pool, _arg: dev, _arg: attrs, _arg: umem->pgs, _arg: umem->npgs) -> return;
}
extern "C" {
    pub fn xp_get_dma(_arg: xskb) -> return;
}
extern "C" {
    pub fn xp_get_frame_dma(_arg: xskb) -> return;
}
extern "C" {
    pub fn xp_alloc(_arg: pool) -> return;
}
// Returns as many entries as possible up to max. 0 <= N <= max.
extern "C" {
    pub fn xp_alloc_batch(_arg: pool, _arg: xdp, _arg: max) -> return;
}
extern "C" {
    pub fn xp_can_alloc(_arg: pool, _arg: count) -> return;
}
extern "C" {
    pub fn xp_raw_get_dma(_arg: pool, _arg: addr) -> return;
}
extern "C" {
    pub fn xp_raw_get_data(_arg: pool, _arg: addr) -> return;
}
//
// xsk_buff_raw_get_ctx - get &xdp_desc context
// @pool: XSk buff pool desc address belongs to
// @addr: desc address (from userspace)
// @options: desc options (from userspace)
//
// Wrapper for xp_raw_get_ctx() to be used in drivers, see its kdoc for
// details.
//
// Return: new &xdp_desc_ctx struct containing desc's DMA address and metadata
// pointer, if it is present (initialized to %NULL otherwise).
//
extern "C" {
    pub fn xp_raw_get_ctx(_arg: pool, _arg: addr, _arg: options) -> return;
}

// flags = READ_ONCE(meta->flags);
//
// xsk_tx_metadata_request - Evaluate AF_XDP TX metadata at submission
// and call appropriate xsk_tx_metadata_ops operation.
// @pool: pointer to AF_XDP buffer pool, used to validate the metadata
// @pmeta: pointer to pointer to AF_XDP metadata area
// @ops: pointer to struct xsk_tx_metadata_ops
// @priv: pointer to driver-private area
//
// This function should be called by the networking device when
// it prepares AF_XDP egress packet.
//
// pmeta = NULL;

