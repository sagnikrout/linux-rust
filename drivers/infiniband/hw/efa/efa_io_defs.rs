//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/efa/efa_io_defs.h
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
pub const EFA_IO_TX_DESC_NUM_BUFS: c_int = 2;
pub const EFA_IO_TX_DESC_NUM_RDMA_BUFS: c_int = 1;
pub const EFA_IO_TX_DESC_INLINE_MAX_SIZE: c_int = 32;
pub const EFA_IO_TX_DESC_IMM_DATA_SIZE: c_int = 4;
pub const EFA_IO_TX_DESC_INLINE_PBL_SIZE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efa_io_queue_type {
// send queue (of a QP)
    EFA_IO_SEND_QUEUE                           = 1,
// recv queue (of a QP)
    EFA_IO_RECV_QUEUE                           = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efa_io_send_op_type {
// send message
    EFA_IO_SEND                                 = 0,
// RDMA read
    EFA_IO_RDMA_READ                            = 1,
// RDMA write
    EFA_IO_RDMA_WRITE                           = 2,
// Fast MR registration
    EFA_IO_FAST_REG                             = 3,
// Fast MR invalidation
    EFA_IO_FAST_INV                             = 4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efa_io_comp_status {
// Successful completion
    EFA_IO_COMP_STATUS_OK                       = 0,
// Flushed during QP destroy
    EFA_IO_COMP_STATUS_FLUSHED                  = 1,
// Internal QP error
    EFA_IO_COMP_STATUS_LOCAL_ERROR_QP_INTERNAL_ERROR = 2,
// Unsupported operation
    EFA_IO_COMP_STATUS_LOCAL_ERROR_UNSUPPORTED_OP = 3,
// Bad AH
    EFA_IO_COMP_STATUS_LOCAL_ERROR_INVALID_AH   = 4,
// LKEY not registered or does not match IOVA
    EFA_IO_COMP_STATUS_LOCAL_ERROR_INVALID_LKEY = 5,
// Message too long
    EFA_IO_COMP_STATUS_LOCAL_ERROR_BAD_LENGTH   = 6,
// RKEY not registered or does not match remote IOVA
    EFA_IO_COMP_STATUS_REMOTE_ERROR_BAD_ADDRESS = 7,
// Connection was reset by remote side
    EFA_IO_COMP_STATUS_REMOTE_ERROR_ABORT       = 8,
// Bad dest QP number (QP does not exist or is in error state)
    EFA_IO_COMP_STATUS_REMOTE_ERROR_BAD_DEST_QPN = 9,
// Destination resource not ready (no WQEs posted on RQ)
    EFA_IO_COMP_STATUS_REMOTE_ERROR_RNR         = 10,
// Receiver SGL too short
    EFA_IO_COMP_STATUS_REMOTE_ERROR_BAD_LENGTH  = 11,
// Unexpected status returned by responder
    EFA_IO_COMP_STATUS_REMOTE_ERROR_BAD_STATUS  = 12,
// Unresponsive remote - was previously responsive
    EFA_IO_COMP_STATUS_LOCAL_ERROR_UNRESP_REMOTE = 13,
// No valid AH at remote side (required for RDMA operations)
    EFA_IO_COMP_STATUS_REMOTE_ERROR_UNKNOWN_PEER = 14,
// Unreachable remote - never received a response
    EFA_IO_COMP_STATUS_LOCAL_ERROR_UNREACH_REMOTE = 15,
// Remote feature mismatch
    EFA_IO_COMP_STATUS_REMOTE_ERROR_FEATURE_MISMATCH = 18,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efa_io_frwr_pbl_mode {
    EFA_IO_FRWR_INLINE_PBL                      = 0,
    EFA_IO_FRWR_DIRECT_PBL                      = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efa_io_processing_hint {
// Optimize for throughput
    EFA_IO_PROCESSING_HINT_BURST_PPS_SENSITIVE  = 1 << 0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_io_req_id_ex {
    pub w: [u16; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_io_tx_meta_desc {
// Verbs-generated Request ID
    pub req_id: u16,
//
// control flags
// 3:0 : op_type - enum efa_io_send_op_type
// 4 : has_imm - immediate_data field carries valid
// data.
// 5 : inline_msg - inline mode - inline message data
// follows this descriptor (no buffer descriptors).
// Note that it is different from immediate data
// 6 : meta_extension - Extended metadata. MBZ
// 7 : meta_desc - Indicates metadata descriptor.
// Must be set.
//
    pub ctrl1: u8,
//
// control flags
// 0 : phase
// 1 : reserved25 - MBZ
// 2 : first - Indicates first descriptor in
// transaction. Must be set.
// 3 : last - Indicates last descriptor in
// transaction. Must be set.
// 4 : comp_req - Indicates whether completion should
// be posted, after packet is transmitted. Valid only
// for the first descriptor
// 7:5 : reserved29 - MBZ
//
    pub ctrl2: u8,
    pub dest_qp_num: u16,
//
// If inline_msg bit is set, length of inline message in bytes,
// otherwise length of SGL (number of buffers).
//
    pub length: u16,
//
// immediate data: if has_imm is set, then this field is included within
// Tx message and reported in remote Rx completion.
//
    pub immediate_data: u32,
    pub ah: u16,
//
// control flags
// 1:0 : processing_hints - Bitmask of enum
// efa_io_processing_hint
// 7:2 : reserved - MBZ
//
    pub ctrl3: u8,
    pub reserved: u8,
// Queue key
    pub qkey: u32,
    pub reserved2: [u8; 6],
    pub req_id_ex: efa_io_req_id_ex,
}

//
// Tx queue buffer descriptor, for any transport type. Preceded by metadata
// descriptor.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_io_tx_buf_desc {
// length in bytes
    pub length: u32,
//
// 23:0 : lkey - local memory translation key
// 31:24 : reserved - MBZ
//
    pub lkey: u32,
// Buffer address bits[31:0]
    pub buf_addr_lo: u32,
// Buffer address bits[63:32]
    pub buf_addr_hi: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_io_remote_mem_addr {
// length in bytes
    pub length: u32,
// remote memory translation key
    pub rkey: u32,
// Buffer address bits[31:0]
    pub buf_addr_lo: u32,
// Buffer address bits[63:32]
    pub buf_addr_hi: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_io_rdma_req {
// Remote memory address
    pub remote_mem: efa_io_remote_mem_addr,
// Local memory address
    pub local_mem: [efa_io_tx_buf_desc; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_io_fast_mr_reg_req {
// Updated local key of the MR after lkey/rkey increment
    pub lkey: u32,
//
// permissions
// 0 : local_write_enable - Local write permissions:
// must be set for RQ buffers and buffers posted for
// RDMA Read requests
// 1 : remote_write_enable - Remote write
// permissions: must be set to enable RDMA write to
// the region
// 2 : remote_read_enable - Remote read permissions:
// must be set to enable RDMA read from the region
// 7:3 : reserved2 - MBZ
//
    pub permissions: u8,
//
// control flags
// 4:0 : phys_page_size_shift - page size is (1 <<
// phys_page_size_shift)
// 6:5 : pbl_mode - enum efa_io_frwr_pbl_mode
// 7 : reserved - MBZ
//
    pub flags: u8,
// MBZ
    pub reserved: [u8; 2],
// IO Virtual Address associated with this MR
    pub iova: u64,
// Memory region length, in bytes
    pub mr_length: u64,
// Physical Buffer List, each element is page-aligned.
//
// Inline array of physical page addresses (optimization
// for short region activation).
//
    pub inline_array: [u64; 1],
// points to PBL (Currently only direct)
    pub dma_addr: u64,
    pub pbl: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_io_fast_mr_inv_req {
// Local key of the MR to invalidate
    pub lkey: u32,
// MBZ
    pub reserved: [u8; 28],
}

//
// Tx WQE, composed of tx meta descriptors followed by either tx buffer
// descriptors or inline data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_io_tx_wqe {
// TX meta
    pub meta: efa_io_tx_meta_desc,
// Send buffer descriptors
    pub sgl: [efa_io_tx_buf_desc; 2],
    pub inline_data: [u8; 32],
// RDMA local and remote memory addresses
    pub rdma_req: efa_io_rdma_req,
// Fast registration
    pub reg_mr_req: efa_io_fast_mr_reg_req,
// Fast invalidation
    pub inv_mr_req: efa_io_fast_mr_inv_req,
    pub data: },
}

//
// Rx buffer descriptor; RX WQE is composed of one or more RX buffer
// descriptors.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_io_rx_desc {
// Buffer address bits[31:0]
    pub buf_addr_lo: u32,
// Buffer Pointer[63:32]
    pub buf_addr_hi: u32,
// Verbs-generated request id.
    pub req_id: u16,
// Length in bytes.
    pub length: u16,
//
// LKey and control flags
// 23:0 : lkey
// 29:24 : reserved - MBZ
// 30 : first - Indicates first descriptor in WQE
// 31 : last - Indicates last descriptor in WQE
//
    pub lkey_ctrl: u32,
}

// Common IO completion descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_io_cdesc_common {
//
// verbs-generated request ID, as provided in the completed tx or rx
// descriptor.
//
    pub req_id: u16,
    pub status: u8,
//
// flags
// 0 : phase - Phase bit
// 2:1 : q_type - enum efa_io_queue_type: send/recv
// 3 : has_imm - indicates that immediate data is
// present - for RX completions only
// 6:4 : op_type - enum efa_io_send_op_type
// 7 : unsolicited - indicates that there is no
// matching request - for RDMA with imm. RX only
//
    pub flags: u8,
// local QP number
    pub qp_num: u16,
}

// Tx completion descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_io_tx_cdesc {
// Common completion info
    pub common: efa_io_cdesc_common,
    pub req_id_ex: efa_io_req_id_ex,
// MBZ
    pub reserved: [u8; 4],
}

// Rx Completion Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_io_rx_cdesc {
// Common completion info
    pub common: efa_io_cdesc_common,
// Transferred length bits[15:0]
    pub length: u16,
// Remote Address Handle FW index, 0xFFFF indicates invalid ah
    pub ah: u16,
    pub src_qp_num: u16,
// Immediate data
    pub imm: u32,
}

// Rx Completion Descriptor RDMA write info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_io_rx_cdesc_rdma_write {
// Transferred length bits[31:16]
    pub length_hi: u16,
}

// Extended Rx Completion Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_io_rx_cdesc_ex {
// Base RX completion info
    pub base: efa_io_rx_cdesc,
    pub rdma_write: efa_io_rx_cdesc_rdma_write,
//
// Valid only in case of unknown AH (0xFFFF) and CQ
// set_src_addr is enabled.
//
    pub src_addr: [u8; 16],
    pub u: },
}

// tx_meta_desc

// tx_buf_desc

// fast_mr_reg_req

// rx_desc

// cdesc_common

