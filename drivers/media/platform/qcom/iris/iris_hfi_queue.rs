//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/qcom/iris/iris_hfi_queue.h
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
// Copyright (c) 2022-2024 Qualcomm Innovation Center, Inc. All rights reserved.
//
// Max 64 Buffers ( 32 input buffers and 32 output buffers)
// can be queued by v4l2 framework at any given time.
//
pub const IFACEQ_MAX_BUF_COUNT: c_int = 64;
//
// Max session supported are 16.
// this value is used to calcualte the size of
// individual shared queue.
//
pub const IFACE_MAX_PARALLEL_SESSIONS: c_int = 16;
pub const IFACEQ_DFLT_QHDR: c_uint = 0x0101;

//
// SFR: Subsystem Failure Reason
// when hardware goes into bad state/failure, firmware fills this memory
// and driver will get to know the actual failure reason from this SFR buffer.
//

//
// Memory layout of the shared queues:
//
// ||=================||  ^        ^         ^
// ||                 ||  |        |         |
// ||    Queue Table  || 288 Bytes |         |
// ||      Header     ||  |        |         |
// ||                 ||  |        |         |
// ||-----------------||  V        |         |
// ||-----------------||  ^        |         |
// ||                 ||  |        |         |
// ||  Command Queue  || 56 Bytes  |         |
// ||     Header      ||  |        |         |
// ||                 ||  |        |         |
// ||-----------------||  V       456 Bytes  |
// ||-----------------||  ^        |         |
// ||                 ||  |        |         |
// ||  Message Queue  || 56 Bytes  |         |
// ||     Header      ||  |        |         |
// ||                 ||  |        |         |
// ||-----------------||  V        |         Buffer size aligned to 4k
// ||-----------------||  ^        |         Overall Queue Size = 2,404 KB
// ||                 ||  |        |         |
// ||   Debug Queue   || 56 Bytes  |         |
// ||     Header      ||  |        |         |
// ||                 ||  |        |         |
// ||=================||  V        V         |
// ||=================||           ^         |
// ||                 ||           |         |
// ||     Command     ||         800 KB      |
// ||      Queue      ||           |         |
// ||                 ||           |         |
// ||=================||           V         |
// ||=================||           ^         |
// ||                 ||           |         |
// ||     Message     ||         800 KB      |
// ||      Queue      ||           |         |
// ||                 ||           |         |
// ||=================||           V         |
// ||=================||           ^         |
// ||                 ||           |         |
// ||      Debug      ||         800 KB      |
// ||      Queue      ||           |         |
// ||                 ||           |         |
// ||=================||           V         |
// ||                 ||                     |
// ||=================||                     V
//
// Shared queues are used for communication between driver and firmware.
// There are 3 types of queues:
// Command queue - driver to write any command to firmware.
// Message queue - firmware to send any response to driver.
// Debug queue - firmware to write debug message.
//
// Host-firmware shared queue ids
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iris_iface_queue {
    IFACEQ_CMDQ_ID,
    IFACEQ_MSGQ_ID,
    IFACEQ_DBGQ_ID,
    IFACEQ_NUMQ, /* not an index */
}

//
// struct iris_hfi_queue_header
//
// @status: Queue status, bits (7:0), 0x1 - active, 0x0 - inactive
// @start_addr: Queue start address in non cached memory
// @queue_type: Queue ID
// @header_type: Default queue header
// @q_size: Queue size
// Number of queue packets if pkt_size is non-zero
// Queue size in bytes if pkt_size is zero
// @pkt_size: Size of queue packet entries
// 0x0: variable queue packet size
// non zero: size of queue packet entry, fixed
// @pkt_drop_cnt: Number of packets dropped by sender
// @rx_wm: Receiver watermark, applicable in event driven mode
// @tx_wm: Sender watermark, applicable in event driven mode
// @rx_req: Receiver sets this bit if queue is empty
// @tx_req: Sender sets this bit if queue is full
// @rx_irq_status: Receiver sets this bit and triggers an interrupt to
// the sender after packets are dequeued. Sender clears this bit
// @tx_irq_status: Sender sets this bit and triggers an interrupt to
// the receiver after packets are queued. Receiver clears this bit
// @read_idx: Index till where receiver has consumed the packets from the queue.
// @write_idx: Index till where sender has written the packets into the queue.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iris_hfi_queue_header {
    pub status: u32,
    pub start_addr: u32,
    pub queue_type: u16,
    pub header_type: u16,
    pub q_size: u32,
    pub pkt_size: u32,
    pub pkt_drop_cnt: u32,
    pub rx_wm: u32,
    pub tx_wm: u32,
    pub rx_req: u32,
    pub tx_req: u32,
    pub rx_irq_status: u32,
    pub tx_irq_status: u32,
    pub read_idx: u32,
    pub write_idx: u32,
}

//
// struct iris_hfi_queue_table_header
//
// @version: Queue table version number
// @size: Queue table size from version to last parametr in qhdr entry
// @qhdr0_offset: Offset to the start of first qhdr
// @qhdr_size: Queue header size in bytes
// @num_q: Total number of queues in Queue table
// @num_active_q: Total number of active queues
// @device_addr: Device address of the queue
// @name: Queue name in characters
// @q_hdr: Array of queue headers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iris_hfi_queue_table_header {
    pub version: u32,
    pub size: u32,
    pub qhdr0_offset: u32,
    pub qhdr_size: u32,
    pub num_q: u32,
    pub num_active_q: u32,
    pub device_addr: *mut c_void,
    pub /: *mut *mut char name[256]; / NUL-terminated array of characters,
    pub q_hdr: [iris_hfi_queue_header; IFACEQ_NUMQ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iris_iface_q_info {
    pub qhdr: *mut iris_hfi_queue_header,
    pub device_addr: dma_addr_t,
    pub kernel_vaddr: *mut c_void,
}

extern "C" {
    pub fn iris_hfi_queues_init(core: *mut iris_core) -> c_int;
}
extern "C" {
    pub fn iris_hfi_queues_deinit(core: *mut iris_core);
}
extern "C" {
    pub fn iris_hfi_queue_cmd_write_locked(core: *mut iris_core, pkt: *mut c_void, pkt_size: u32) -> c_int;
}
extern "C" {
    pub fn iris_hfi_queue_cmd_write(core: *mut iris_core, pkt: *mut c_void, pkt_size: u32) -> c_int;
}
extern "C" {
    pub fn iris_hfi_queue_msg_read(core: *mut iris_core, pkt: *mut c_void) -> c_int;
}
extern "C" {
    pub fn iris_hfi_queue_dbg_read(core: *mut iris_core, pkt: *mut c_void) -> c_int;
}
