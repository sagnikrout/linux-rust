//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/mediatek/vcodec/decoder/vdec_msg_queue.h
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
//
// Copyright (c) 2021 MediaTek Inc.
// Author: Yunfei Dong <yunfei.dong@mediatek.com>
//

pub const NUM_BUFFER_COUNT: c_int = 3;
extern "C" {
    pub fn int(lat_buf: *mut *mut core_decode_cb_t)(struct vdec_lat_buf) -> typedef;
}
//
// enum core_ctx_status - Context decode status for core hardwre.
// @CONTEXT_LIST_EMPTY: No buffer queued on core hardware(must always be 0)
// @CONTEXT_LIST_QUEUED: Buffer queued to core work list
// @CONTEXT_LIST_DEC_DONE: context decode done
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum core_ctx_status {
    CONTEXT_LIST_EMPTY = 0,
    CONTEXT_LIST_QUEUED,
    CONTEXT_LIST_DEC_DONE,
}

//
// struct vdec_msg_queue_ctx - represents a queue for buffers ready to be processed
// @ready_to_use: ready used queue used to signalize when get a job queue
// @ready_queue: list of ready lat buffer queues
// @ready_lock: spin lock to protect the lat buffer usage
// @ready_num: number of buffers ready to be processed
// @hardware_index: hardware id that this queue is used for
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdec_msg_queue_ctx {
    pub ready_to_use: wait_queue_head_t,
    pub ready_queue: list_head,
// protect lat buffer
    pub ready_lock: spinlock_t,
    pub ready_num: c_int,
    pub hardware_index: c_int,
}

//
// struct vdec_lat_buf - lat buffer message used to store lat info for core decode
// @wdma_err_addr: wdma error address used for lat hardware
// @slice_bc_addr: slice bc address used for lat hardware
// @rd_mv_addr:	mv addr for av1 lat hardware output, core hardware input
// @tile_addr:	tile buffer for av1 core input
// @ts_info: need to set timestamp from output to capture
// @src_buf_req: output buffer media request object
//
// @private_data: shared information used to lat and core hardware
// @ctx: mtk vcodec context information
// @core_decode: different codec use different decode callback function
// @lat_list: add lat buffer to lat head list
// @core_list: add lat buffer to core head list
//
// @is_last_frame: meaning this buffer is the last frame
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdec_lat_buf {
    pub wdma_err_addr: mtk_vcodec_mem,
    pub slice_bc_addr: mtk_vcodec_mem,
    pub rd_mv_addr: mtk_vcodec_mem,
    pub tile_addr: mtk_vcodec_mem,
    pub ts_info: vb2_v4l2_buffer,
    pub src_buf_req: *mut media_request,
    pub private_data: *mut c_void,
    pub ctx: *mut mtk_vcodec_dec_ctx,
    pub core_decode: core_decode_cb_t,
    pub lat_list: list_head,
    pub core_list: list_head,
    pub is_last_frame: bool,
}

//
// struct vdec_msg_queue - used to store lat buffer message
// @lat_buf: lat buffer used to store lat buffer information
// @wdma_addr: wdma address used for ube
// @wdma_rptr_addr: ube read point
// @wdma_wptr_addr: ube write point
// @core_work: core hardware work
// @lat_ctx: used to store lat buffer list
// @core_ctx: used to store core buffer list
//
// @lat_list_cnt: used to record each instance lat list count
// @core_list_cnt: used to record each instance core list count
// @flush_done: core flush done status
// @empty_lat_buf: the last lat buf used to flush decode
// @core_dec_done: core work queue decode done event
// @status: current context decode status for core hardware
// @ctx: mtk vcodec context information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdec_msg_queue {
    pub lat_buf: [vdec_lat_buf; NUM_BUFFER_COUNT],
    pub wdma_addr: mtk_vcodec_mem,
    pub wdma_rptr_addr: u64,
    pub wdma_wptr_addr: u64,
    pub core_work: work_struct,
    pub lat_ctx: vdec_msg_queue_ctx,
    pub core_ctx: vdec_msg_queue_ctx,
    pub lat_list_cnt: core::sync::atomic::AtomicI32,
    pub core_list_cnt: core::sync::atomic::AtomicI32,
    pub flush_done: bool,
    pub empty_lat_buf: vdec_lat_buf,
    pub core_dec_done: wait_queue_head_t,
    pub status: c_int,
    pub ctx: *mut mtk_vcodec_dec_ctx,
}

//
// vdec_msg_queue_init - init lat buffer information.
// @msg_queue: used to store the lat buffer information
// @ctx: v4l2 ctx
// @core_decode: core decode callback for each codec
// @private_size: the private data size used to share with core
//
// Return: returns 0 if init successfully, or fail.
//
// vdec_msg_queue_init_ctx - used to init msg queue context information.
// @ctx: message queue context
// @hardware_index: hardware index
//
extern "C" {
    pub fn vdec_msg_queue_init_ctx(ctx: *mut vdec_msg_queue_ctx, hardware_index: c_int);
}
//
// vdec_msg_queue_qbuf - enqueue lat buffer to queue list.
// @ctx: message queue context
// @buf: current lat buffer
//
// Return: returns 0 if qbuf successfully, or fail.
//
extern "C" {
    pub fn vdec_msg_queue_qbuf(ctx: *mut vdec_msg_queue_ctx, buf: *mut vdec_lat_buf) -> c_int;
}
//
// vdec_msg_queue_dqbuf - dequeue lat buffer from queue list.
// @ctx: message queue context
//
// Return: returns not null if dq successfully, or fail.
//
// vdec_msg_queue_update_ube_rptr - used to update the ube read point.
// @msg_queue: used to store the lat buffer information
// @ube_rptr: current ube read point
//
extern "C" {
    pub fn vdec_msg_queue_update_ube_rptr(msg_queue: *mut vdec_msg_queue, ube_rptr: u64);
}
//
// vdec_msg_queue_update_ube_wptr - used to update the ube write point.
// @msg_queue: used to store the lat buffer information
// @ube_wptr: current ube write point
//
extern "C" {
    pub fn vdec_msg_queue_update_ube_wptr(msg_queue: *mut vdec_msg_queue, ube_wptr: u64);
}
//
// vdec_msg_queue_wait_lat_buf_full - used to check whether all lat buffer
// in lat list.
// @msg_queue: used to store the lat buffer information
//
// Return: returns true if successfully, or fail.
//
extern "C" {
    pub fn vdec_msg_queue_wait_lat_buf_full(msg_queue: *mut vdec_msg_queue) -> bool;
}
//
// vdec_msg_queue_deinit - deinit lat buffer information.
// @msg_queue: used to store the lat buffer information
// @ctx: v4l2 ctx
//
