//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/amazon/ena/ena_eth_com.h
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
// Copyright 2015-2020 Amazon.com, Inc. or its affiliates. All rights reserved.
//

// we allow 2 DMA descriptors per LLQ entry

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_com_tx_ctx {
    pub ena_meta: ena_com_tx_meta,
    pub ena_bufs: *mut ena_com_buf,
// For LLQ, header buffer - pushed to the device mem space
    pub push_header: *mut c_void,
    pub l3_proto: ena_eth_io_l3_proto_index,
    pub l4_proto: ena_eth_io_l4_proto_index,
    pub num_bufs: u16,
    pub req_id: u16,
// For regular queue, indicate the size of the header
// For LLQ, indicate the size of the pushed buffer
//
    pub header_len: u16,
    pub meta_valid: u8,
    pub tso_enable: u8,
    pub l3_csum_enable: u8,
    pub l4_csum_enable: u8,
    pub l4_csum_partial: u8,
    pub /: *mut *mut u8 df; / Don't fragment,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_com_rx_ctx {
    pub ena_bufs: *mut ena_com_rx_buf_info,
    pub l3_proto: ena_eth_io_l3_proto_index,
    pub l4_proto: ena_eth_io_l4_proto_index,
    pub l3_csum_err: bool,
    pub l4_csum_err: bool,
    pub l4_csum_checked: u8,
// fragmented packet
    pub frag: bool,
    pub hash: u32,
    pub descs: u16,
    pub max_bufs: u16,
    pub pkt_offset: u8,
}

extern "C" {
    pub fn ena_com_cq_empty(io_cq: *mut ena_com_io_cq) -> bool;
}
// Check if the submission queue has enough space to hold required_buffers
// This calculation doesn't need to be 100% accurate. So to reduce
// the calculation overhead just Subtract 2 lines from the free descs
// (one for the header line and one to compensate the devision
// down calculation.
//
// Switch phase bit in case of wrap around
// When the current completion descriptor phase isn't the same as the
// expected, it mean that the device still didn't update
// this completion.
//
// req_id = READ_ONCE(cdesc->req_id);
