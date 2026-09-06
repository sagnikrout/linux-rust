//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/cavium/liquidio/octeon_iq.h
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


//
// Author: Cavium, Inc.
//
// Contact: support@cavium.com
// Please include "LiquidIO" in the subject.
//
// Copyright (c) 2003-2016 Cavium, Inc.
//
// This file is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License, Version 2, as
// published by the Free Software Foundation.
//
// This file is distributed in the hope that it will be useful, but
// AS-IS and WITHOUT ANY WARRANTY; without even the implied warranty
// of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE, TITLE, or
// NONINFRINGEMENT.  See the GNU General Public License for more details.
//
// !  \file  octeon_iq.h
// \brief Host Driver: Implementation of Octeon input queues. "Input" is
// with respect to the Octeon device on the NIC. From this driver's
// point of view they are egress queues.
//
pub const IQ_STATUS_RUNNING: c_int = 1;
pub const IQ_SEND_OK: c_int = 0;
pub const IQ_SEND_STOP: c_int = 1;

// -------------------------  INSTRUCTION QUEUE --------------------------
// \cond
pub const REQTYPE_NONE: c_int = 0;
pub const REQTYPE_NORESP_NET: c_int = 1;
pub const REQTYPE_NORESP_NET_SG: c_int = 2;
pub const REQTYPE_RESP_NET: c_int = 3;
pub const REQTYPE_RESP_NET_SG: c_int = 4;
pub const REQTYPE_SOFT_COMMAND: c_int = 5;
pub const REQTYPE_LAST: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octeon_request_list {
    pub reqtype: u32,
    pub buf: *mut c_void,
}

// \endcond
// Input Queue statistics. Each input queue has four stats fields.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct oct_iq_stats {
    pub /: *mut *mut *mut u64 instr_posted; /< Instructions posted to this queue.,
    pub /: *mut *mut *mut u64 instr_processed; /< Instructions processed in this queue.,
    pub /: *mut *mut *mut u64 instr_dropped; /< Instructions that could not be processed,
    pub /: *mut *mut *mut u64 bytes_sent; /< Bytes sent through this queue.,
    pub /: *mut *mut *mut u64 sgentry_sent;/< Gather entries sent through this queue.,
    pub /: *mut *mut *mut u64 tx_done;/< Num of packets sent to network.,
    pub /: *mut *mut *mut u64 tx_iq_busy;/< Numof times this iq was found to be full.,
    pub /: *mut *mut *mut u64 tx_dropped;/< Numof pkts dropped dueto xmitpath errors.,
    pub /: *mut *mut *mut u64 tx_tot_bytes;/< Total count of bytes sento to network.,
    pub /: *mut *mut u64 tx_gso; / count of tso,
    pub /: *mut *mut u64 tx_vxlan; / tunnel,
    pub /: *mut *mut u64 tx_dmamap_fail; / Number of times dma mapping failed,
    pub /: *mut *mut u64 tx_restart; / Number of times this queue restarted,
}

// The instruction (input) queue.
// The input queue is used to post raw (instruction) mode data or packet
// data to Octeon device from the host. Each input queue (upto 4) for
// a Octeon device has one such structure to represent it.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octeon_instr_queue {
    pub oct_dev: *mut octeon_device,
// A spinlock to protect access to the input ring.
    pub lock: spinlock_t,
// A spinlock to protect while posting on the ring.
    pub post_lock: spinlock_t,
// This flag indicates if the queue can be used for soft commands.
// If this flag is set, post_lock must be acquired before posting
// a command to the queue.
// If this flag is clear, post_lock is invalid for the queue.
// All control commands (soft commands) will go through only Queue 0
// (control and data queue). So only queue-0 needs post_lock,
// other queues are only data queues and does not need post_lock
//
    pub allow_soft_cmds: bool,
    pub pkt_in_done: u32,
    pub pkts_processed: u32,
// A spinlock to protect access to the input ring.
    pub iq_flush_running_lock: spinlock_t,
// Flag that indicates if the queue uses 64 byte commands.
    pub iqcmd_64B:1: u32,
// Queue info.
    pub txpciq: oct_txpciq,
    pub rsvd:17: u32,
// Controls whether extra flushing of IQ is done on Tx
    pub do_auto_flush:1: u32,
    pub status:8: u32,
// Maximum no. of instructions in this queue.
    pub max_count: u32,
// Index in input ring where the driver should write the next packet
    pub host_write_index: u32,
// Index in input ring where Octeon is expected to read the next
// packet.
//
    pub octeon_read_index: u32,
// This index aids in finding the window in the queue where Octeon
// has read the commands.
//
    pub flush_index: u32,
// This field keeps track of the instructions pending in this queue.
    pub instr_pending: core::sync::atomic::AtomicI32,
    pub reset_instr_cnt: u32,
// Pointer to the Virtual Base addr of the input ring.
    pub base_addr: *mut u8,
    pub request_list: *mut octeon_request_list,
// Octeon doorbell register for the ring.
    pub doorbell_reg: *mut void __iomem,
// Octeon instruction count register for this ring.
    pub inst_cnt_reg: *mut void __iomem,
// Number of instructions pending to be posted to Octeon.
    pub fill_cnt: u32,
// The max. number of instructions that can be held pending by the
// driver.
//
    pub fill_threshold: u32,
// The last time that the doorbell was rung.
    pub last_db_time: u64,
// The doorbell timeout. If the doorbell was not rung for this time and
// fill_cnt is non-zero, ring the doorbell again.
//
    pub db_timeout: u32,
// Statistics for this input queue.
    pub stats: oct_iq_stats,
// DMA mapped base address of the input descriptor ring.
    pub base_addr_dma: dma_addr_t,
// Application context
    pub app_ctx: *mut c_void,
// network stack queue index
    pub q_index: c_int,
// os ifidx associated with this queue
    pub ifidx: c_int,
}

// ----------------------  INSTRUCTION FORMAT ----------------------------
// 32-byte instruction format.
// Format of instruction for a 32-byte mode input queue.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octeon_instr_32B {
// Pointer where the input data is available.
    pub dptr: u64,
// Instruction Header.
    pub ih: u64,
// Pointer where the response for a RAW mode packet will be written
// by Octeon.
//
    pub rptr: u64,
// Input Request Header. Additional info about the input.
    pub irh: u64,
}

// 64-byte instruction format.
// Format of instruction for a 64-byte mode input queue.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octeon_instr2_64B {
// Pointer where the input data is available.
    pub dptr: u64,
// Instruction Header.
    pub ih2: u64,
// Input Request Header.
    pub irh: u64,
// opcode/subcode specific parameters
    pub ossp: [u64; 2],
// Return Data Parameters
    pub rdp: u64,
// Pointer where the response for a RAW mode packet will be written
// by Octeon.
//
    pub rptr: u64,
    pub reserved: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct octeon_instr3_64B {
// Pointer where the input data is available.
    pub dptr: u64,
// Instruction Header.
    pub ih3: u64,
// Instruction Header.
    pub pki_ih3: u64,
// Input Request Header.
    pub irh: u64,
// opcode/subcode specific parameters
    pub ossp: [u64; 2],
// Return Data Parameters
    pub rdp: u64,
// Pointer where the response for a RAW mode packet will be written
// by Octeon.
//
    pub rptr: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union octeon_instr_64B {
    pub cmd2: octeon_instr2_64B,
    pub cmd3: octeon_instr3_64B,
}

// The size of each buffer in soft command buffer pool
//
pub const SOFT_COMMAND_BUFFER_SIZE: c_int = 2048;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octeon_soft_command {
// Soft command buffer info.
    pub node: list_head,
    pub dma_addr: u64,
    pub size: u32,
// Command and return status
    pub cmd: octeon_instr_64B,
pub const COMPLETION_WORD_INIT: c_uint = 0xffffffffffffffffULL;
    pub status_word: *mut u64,
// Data buffer info
    pub virtdptr: *mut c_void,
    pub dmadptr: u64,
    pub datasize: u32,
// Return buffer info
    pub virtrptr: *mut c_void,
    pub dmarptr: u64,
    pub rdatasize: u32,
// Context buffer info
    pub ctxptr: *mut c_void,
    pub ctxsize: u32,
// Time out and callback
    pub expiry_time: usize,
    pub iq_no: u32,
    pub ): *mut *mut *mut void (callback)(struct octeon_device , u32, void,
    pub callback_arg: *mut c_void,
    pub caller_is_done: c_int,
    pub sc_status: u32,
    pub complete: completion,
}

// max timeout (in milli sec) for soft request
pub const LIO_SC_MAX_TMO_MS: c_int = 60000;
// Maximum number of buffers to allocate into soft command buffer pool
//
pub const MAX_SOFT_COMMAND_BUFFERS: c_int = 256;
// Head of a soft command buffer pool.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octeon_sc_buffer_pool {
// List structure to add delete pending entries to
    pub head: list_head,
// A lock for this response list
    pub lock: spinlock_t,
    pub alloc_buf_count: core::sync::atomic::AtomicI32,
}

extern "C" {
    pub fn octeon_setup_sc_buffer_pool(oct: *mut octeon_device) -> c_int;
}
extern "C" {
    pub fn octeon_free_sc_done_list(oct: *mut octeon_device) -> c_int;
}
extern "C" {
    pub fn octeon_free_sc_zombie_list(oct: *mut octeon_device) -> c_int;
}
extern "C" {
    pub fn octeon_free_sc_buffer_pool(oct: *mut octeon_device) -> c_int;
}
//
// octeon_init_instr_queue()
// @param octeon_dev      - pointer to the octeon device structure.
// @param txpciq          - queue to be initialized (0 <= q_no <= 3).
//
// Called at driver init time for each input queue. iq_conf has the
// configuration parameters for the queue.
//
// @return  Success: 0   Failure: 1
//
// octeon_delete_instr_queue()
// @param octeon_dev      - pointer to the octeon device structure.
// @param iq_no           - queue to be deleted (0 <= q_no <= 3).
//
// Called at driver unload time for each input queue. Deletes all
// allocated resources for the input queue.
//
// @return  Success: 0   Failure: 1
//
extern "C" {
    pub fn octeon_delete_instr_queue(octeon_dev: *mut octeon_device, iq_no: u32) -> c_int;
}
extern "C" {
    pub fn lio_wait_for_instr_fetch(oct: *mut octeon_device) -> c_int;
}
