//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/sw/rxe/rxe_opcode.h
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
// contains header bit mask definitions and header lengths
// declaration of the rxe_opcode_info struct and
// rxe_wr_opcode_info struct
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rxe_wr_mask {
    WR_INLINE_MASK			= BIT(0),
    WR_ATOMIC_MASK			= BIT(1),
    WR_SEND_MASK			= BIT(2),
    WR_READ_MASK			= BIT(3),
    WR_WRITE_MASK			= BIT(4),
    WR_LOCAL_OP_MASK		= BIT(5),
    WR_FLUSH_MASK			= BIT(6),
    WR_ATOMIC_WRITE_MASK		= BIT(7),

    WR_READ_OR_WRITE_MASK		= WR_READ_MASK | WR_WRITE_MASK,
    WR_WRITE_OR_SEND_MASK		= WR_WRITE_MASK | WR_SEND_MASK,
    WR_ATOMIC_OR_READ_MASK		= WR_ATOMIC_MASK | WR_READ_MASK,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxe_wr_opcode_info {
    pub name: *mut c_char,
    pub mask: [rxe_wr_mask; WR_MAX_QPT],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rxe_hdr_type {
    RXE_LRH,
    RXE_GRH,
    RXE_BTH,
    RXE_RETH,
    RXE_AETH,
    RXE_ATMETH,
    RXE_ATMACK,
    RXE_IETH,
    RXE_RDETH,
    RXE_DETH,
    RXE_IMMDT,
    RXE_FETH,
    RXE_PAYLOAD,
    NUM_HDR_TYPES
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rxe_hdr_mask {
    RXE_LRH_MASK		= BIT(RXE_LRH),
    RXE_GRH_MASK		= BIT(RXE_GRH),
    RXE_BTH_MASK		= BIT(RXE_BTH),
    RXE_IMMDT_MASK		= BIT(RXE_IMMDT),
    RXE_RETH_MASK		= BIT(RXE_RETH),
    RXE_AETH_MASK		= BIT(RXE_AETH),
    RXE_ATMETH_MASK		= BIT(RXE_ATMETH),
    RXE_ATMACK_MASK		= BIT(RXE_ATMACK),
    RXE_IETH_MASK		= BIT(RXE_IETH),
    RXE_RDETH_MASK		= BIT(RXE_RDETH),
    RXE_DETH_MASK		= BIT(RXE_DETH),
    RXE_FETH_MASK		= BIT(RXE_FETH),
    RXE_PAYLOAD_MASK	= BIT(RXE_PAYLOAD),

    RXE_REQ_MASK		= BIT(NUM_HDR_TYPES + 0),
    RXE_ACK_MASK		= BIT(NUM_HDR_TYPES + 1),
    RXE_SEND_MASK		= BIT(NUM_HDR_TYPES + 2),
    RXE_WRITE_MASK		= BIT(NUM_HDR_TYPES + 3),
    RXE_READ_MASK		= BIT(NUM_HDR_TYPES + 4),
    RXE_ATOMIC_MASK		= BIT(NUM_HDR_TYPES + 5),
    RXE_FLUSH_MASK		= BIT(NUM_HDR_TYPES + 6),

    RXE_RWR_MASK		= BIT(NUM_HDR_TYPES + 7),
    RXE_COMP_MASK		= BIT(NUM_HDR_TYPES + 8),

    RXE_START_MASK		= BIT(NUM_HDR_TYPES + 9),
    RXE_MIDDLE_MASK		= BIT(NUM_HDR_TYPES + 10),
    RXE_END_MASK		= BIT(NUM_HDR_TYPES + 11),

    RXE_LOOPBACK_MASK	= BIT(NUM_HDR_TYPES + 12),

    RXE_ATOMIC_WRITE_MASK   = BIT(NUM_HDR_TYPES + 14),

    RXE_READ_OR_ATOMIC_MASK	= (RXE_READ_MASK | RXE_ATOMIC_MASK),
    RXE_WRITE_OR_SEND_MASK	= (RXE_WRITE_MASK | RXE_SEND_MASK),
    RXE_READ_OR_WRITE_MASK	= (RXE_READ_MASK | RXE_WRITE_MASK),
    RXE_RDMA_OP_MASK	= (RXE_READ_MASK | RXE_WRITE_MASK |
    RXE_ATOMIC_WRITE_MASK | RXE_FLUSH_MASK |
    RXE_ATOMIC_MASK),
}

pub const RXE_NUM_OPCODE: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxe_opcode_info {
    pub name: *mut c_char,
    pub mask: rxe_hdr_mask,
    pub length: c_int,
    pub offset: [c_int; NUM_HDR_TYPES],
}
