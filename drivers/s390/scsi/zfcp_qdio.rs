//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/s390/scsi/zfcp_qdio.h
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
// zfcp device driver
//
// Header file for zfcp qdio interface
//
// Copyright IBM Corp. 2010
//

// Max SBALS for chaining
pub const ZFCP_QDIO_MAX_SBALS_PER_REQ: c_int = 36;
//
// struct zfcp_qdio - basic qdio data structure
// @res_q: response queue
// @req_q: request queue
// @req_q_idx: index of next free buffer
// @req_q_free: number of free buffers in queue
// @stat_lock: lock to protect req_q_util and req_q_time
// @req_q_lock: lock to serialize access to request queue
// @req_q_time: time of last fill level change
// @req_q_util: used for accounting
// @req_q_full: queue full incidents
// @req_q_wq: used to wait for SBAL availability
// @irq_tasklet: used for QDIO interrupt processing
// @request_tasklet: used for Request Queue completion processing
// @request_timer: used to trigger the Request Queue completion processing
// @adapter: adapter used in conjunction with this qdio structure
// @max_sbale_per_sbal: qdio limit per sbal
// @max_sbale_per_req: qdio limit per request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zfcp_qdio {
    pub res_q: [*mut qdio_buffer; QDIO_MAX_BUFFERS_PER_Q],
    pub req_q: [*mut qdio_buffer; QDIO_MAX_BUFFERS_PER_Q],
    pub req_q_idx: u8,
    pub req_q_free: core::sync::atomic::AtomicI32,
    pub stat_lock: spinlock_t,
    pub req_q_lock: spinlock_t,
    pub req_q_time: c_ulonglong,
    pub req_q_util: u64,
    pub req_q_full: core::sync::atomic::AtomicI32,
    pub req_q_wq: wait_queue_head_t,
    pub irq_tasklet: tasklet_struct,
    pub request_tasklet: tasklet_struct,
    pub request_timer: timer_list,
    pub adapter: *mut zfcp_adapter,
    pub max_sbale_per_sbal: u16,
    pub max_sbale_per_req: u16,
}

//
// struct zfcp_qdio_req - qdio queue related values for a request
// @sbtype: sbal type flags for sbale 0
// @sbal_number: number of free sbals
// @sbal_first: first sbal for this request
// @sbal_last: last sbal for this request
// @sbal_limit: last possible sbal for this request
// @sbale_curr: current sbale at creation of this request
// @qdio_outb_usage: usage of outbound queue
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zfcp_qdio_req {
    pub sbtype: u8,
    pub sbal_number: u8,
    pub sbal_first: u8,
    pub sbal_last: u8,
    pub sbal_limit: u8,
    pub sbale_curr: u8,
    pub qdio_outb_usage: u16,
}

//
// zfcp_qdio_sbale_req - return pointer to sbale on req_q for a request
// @qdio: pointer to struct zfcp_qdio
// @q_req: pointer to struct zfcp_qdio_req
// Returns: pointer to qdio_buffer_element (sbale) structure
//
// zfcp_qdio_sbale_curr - return current sbale on req_q for a request
// @qdio: pointer to struct zfcp_qdio
// @q_req: pointer to struct zfcp_qdio_req
// Returns: pointer to qdio_buffer_element (sbale) structure
//
// zfcp_qdio_req_init - initialize qdio request
// @qdio: request queue where to start putting the request
// @q_req: the qdio request to start
// @req_id: The request id
// @sbtype: type flags to set for all sbals
// @data: First data block
// @len: Length of first data block
//
// This is the start of putting the request into the queue, the last
// step is passing the request to zfcp_qdio_send. The request queue
// lock must be held during the whole process from init to send.
//
// zfcp_qdio_fill_next - Fill next sbale, only for single sbal requests
// @qdio: pointer to struct zfcp_qdio
// @q_req: pointer to struct zfcp_queue_req
// @data: pointer to data
// @len: length of data
//
// This is only required for single sbal requests, calling it when
// wrapping around to the next sbal is a bug.
//
// zfcp_qdio_set_sbale_last - set last entry flag in current sbale
// @qdio: pointer to struct zfcp_qdio
// @q_req: pointer to struct zfcp_queue_req
//
// zfcp_qdio_sg_one_sbal - check if one sbale is enough for sg data
// @sg: The scatterlist where to check the data size
//
// Returns: 1 when one sbale is enough for the data in the scatterlist,
// 0 if not.
//
// zfcp_qdio_skip_to_last_sbale - skip to last sbale in sbal
// @qdio: pointer to struct zfcp_qdio
// @q_req: The current zfcp_qdio_req
//
// zfcp_qdio_sbal_limit - set the sbal limit for a request in q_req
// @qdio: pointer to struct zfcp_qdio
// @q_req: The current zfcp_qdio_req
// @max_sbals: maximum number of SBALs allowed
//
// zfcp_qdio_set_data_div - set data division count
// @qdio: pointer to struct zfcp_qdio
// @q_req: The current zfcp_qdio_req
// @count: The data division count
//
// zfcp_qdio_real_bytes - count bytes used
// @sg: pointer to struct scatterlist
//
// zfcp_qdio_set_scount - set SBAL count value
// @qdio: pointer to struct zfcp_qdio
// @q_req: The current zfcp_qdio_req
//
