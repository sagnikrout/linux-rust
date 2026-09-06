//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hid/intel-ish-hid/ishtp/client.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// ISHTP client logic
//
// Copyright (c) 2003-2016, Intel Corporation.
//

// Tx and Rx ring size
pub const CL_DEF_RX_RING_SIZE: c_int = 2;
pub const CL_DEF_TX_RING_SIZE: c_int = 2;
pub const CL_MAX_RX_RING_SIZE: c_int = 32;
pub const CL_MAX_TX_RING_SIZE: c_int = 32;
pub const DMA_SLOT_SIZE: c_int = 4096;
// Number of IPC fragments after which it's worth sending via DMA
pub const DMA_WORTH_THRESHOLD: c_int = 3;
// DMA/IPC Tx paths. Other the default means enforcement
pub const CL_TX_PATH_DEFAULT: c_int = 0;
pub const CL_TX_PATH_IPC: c_int = 1;
pub const CL_TX_PATH_DMA: c_int = 2;
// Client Tx buffer list entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ishtp_cl_tx_ring {
    pub list: list_head,
    pub send_buf: ishtp_msg_data,
}

// ISHTP client instance
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ishtp_cl {
    pub link: list_head,
    pub dev: *mut ishtp_device,
    pub state: cl_state,
    pub status: c_int,
// Link to ISHTP bus device
    pub device: *mut ishtp_cl_device,
// ID of client connected
    pub host_client_id: u8,
    pub fw_client_id: u8,
    pub ishtp_flow_ctrl_creds: u8,
    pub out_flow_ctrl_creds: u8,
// dma
    pub last_tx_path: c_int,
// 0: ack wasn't received,1:ack was received
    pub last_dma_acked: c_int,
    pub last_dma_addr: *mut c_uchar,
// 0: ack wasn't received,1:ack was received
    pub last_ipc_acked: c_int,
// Rx ring buffer pool
    pub rx_ring_size: c_uint,
    pub free_rb_list: ishtp_cl_rb,
    pub free_list_spinlock: spinlock_t,
// Rx in-process list
    pub in_process_list: ishtp_cl_rb,
    pub in_process_spinlock: spinlock_t,
// Client Tx buffers list
    pub tx_ring_size: c_uint,
    pub tx_free_list: ishtp_cl_tx_ring tx_list,,
    pub tx_ring_free_size: c_int,
    pub tx_list_spinlock: spinlock_t,
    pub tx_free_list_spinlock: spinlock_t,
    pub /: *mut *mut size_t tx_offs; / Offset in buffer at head of 'tx_list',
//
// if we get a FC, and the list is not empty, we must know whether we
// are at the middle of sending.
// if so -need to increase FC counter, otherwise, need to start sending
// the first msg in list
// (!)This is for counting-FC implementation only. Within single-FC the
// other party may NOT send FC until it receives complete message
//
    pub sending: c_int,
// Send FC spinlock
    pub fc_spinlock: spinlock_t,
// wait queue for connect and disconnect response from FW
    pub wait_ctrl_res: wait_queue_head_t,
// Error stats
    pub err_send_msg: c_uint,
    pub err_send_fc: c_uint,
// Send/recv stats
    pub send_msg_cnt_ipc: c_uint,
    pub send_msg_cnt_dma: c_uint,
    pub recv_msg_cnt_ipc: c_uint,
    pub recv_msg_cnt_dma: c_uint,
    pub recv_msg_num_frags: c_uint,
    pub ishtp_flow_ctrl_cnt: c_uint,
    pub out_flow_ctrl_cnt: c_uint,
// Rx msg ... out FC timing
    pub ts_rx: ktime_t,
    pub ts_out_fc: ktime_t,
    pub ts_max_fc_delay: ktime_t,
    pub client_data: *mut c_void,
}

// Client connection managenment internal functions
extern "C" {
    pub fn ishtp_fw_cl_by_id(dev: *mut ishtp_device, client_id: u8) -> c_int;
}
extern "C" {
    pub fn ishtp_cl_send_msg(dev: *mut ishtp_device, cl: *mut ishtp_cl);
}
extern "C" {
    pub fn ishtp_cl_read_start(cl: *mut ishtp_cl) -> c_int;
}
// Ring Buffer I/F
extern "C" {
    pub fn ishtp_cl_alloc_rx_ring(cl: *mut ishtp_cl) -> c_int;
}
extern "C" {
    pub fn ishtp_cl_alloc_tx_ring(cl: *mut ishtp_cl) -> c_int;
}
extern "C" {
    pub fn ishtp_cl_free_rx_ring(cl: *mut ishtp_cl);
}
extern "C" {
    pub fn ishtp_cl_free_tx_ring(cl: *mut ishtp_cl);
}
// DMA I/F functions
extern "C" {
    pub fn ishtp_cl_alloc_dma_buf(dev: *mut ishtp_device);
}
extern "C" {
    pub fn ishtp_cl_free_dma_buf(dev: *mut ishtp_device);
}
// Request blocks alloc/free I/F
extern "C" {
    pub fn ishtp_io_rb_free(priv_rb: *mut ishtp_cl_rb);
}
extern "C" {
    pub fn ishtp_io_rb_alloc_buf(rb: *mut ishtp_cl_rb, length: usize) -> c_int;
}
//
// ishtp_cl_cmp_id - tells if file private data have same id
// returns true  - if ids are the same and not NULL
//
