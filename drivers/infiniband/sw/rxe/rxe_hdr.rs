//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/sw/rxe/rxe_hdr.h
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
// extracted information about a packet carried in an sk_buff struct fits in
// the skbuff cb array. Must be at most 48 bytes. stored in control block of
// sk_buff for received packets.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxe_pkt_info {
    pub /: *mut *mut *mut rxe_dev rxe; / device that owns packet,
    pub /: *mut *mut *mut rxe_qp qp; / qp that owns packet,
    pub /: *mut *mut *mut rxe_send_wqe wqe; / send wqe,
    pub /: *mut *mut *mut u8 hdr; / points to bth,
    pub /: *mut *mut u32 mask; / useful info about pkt,
    pub /: *mut *mut u32 psn; / bth psn of packet,
    pub /: *mut *mut u16 pkey_index; / partition of pkt,
    pub /: *mut *mut u16 paylen; / length of bth - icrc,
    pub /: *mut *mut u8 port_num; / port pkt received on,
    pub /: *mut *mut u8 opcode; / bth opcode of packet,
}

// Macros should be used only for received skb
extern "C" {
    pub fn container_of()pkt: *mut (void, sk_buff: struct, _arg: cb) -> return;
}
//
// IBA header types and methods
//
// Some of these are for reference and completeness only since
// rxe does not currently support RD transport
// most of this could be moved into IB core. ib_pack.h has
// part of this but is incomplete
//
// Header specific routines to insert/extract values to/from headers
// the routines that are named __hhh_(set_)fff() take a pointer to a
// hhh header and get(set) the fff field. The routines named
// hhh_(set_)fff take a packet info struct and find the
// header and field based on the opcode in the packet.
// Conversion to/from network byte order from cpu order is also done.
//

//
// Base Transport Header
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxe_bth {
    pub opcode: u8,
    pub flags: u8,
    pub pkey: __be16,
    pub qpn: __be32,
    pub apsn: __be32,
}

extern "C" {
    pub fn be16_to_cpu(_arg: bth->pkey) -> return;
}
extern "C" {
    pub fn __bth_opcode(_arg: pkt->hdr) -> return;
}
extern "C" {
    pub fn __bth_se(_arg: pkt->hdr) -> return;
}
extern "C" {
    pub fn __bth_mig(_arg: pkt->hdr) -> return;
}
extern "C" {
    pub fn __bth_pad(_arg: pkt->hdr) -> return;
}
extern "C" {
    pub fn __bth_tver(_arg: pkt->hdr) -> return;
}
extern "C" {
    pub fn __bth_pkey(_arg: pkt->hdr) -> return;
}
extern "C" {
    pub fn __bth_qpn(_arg: pkt->hdr) -> return;
}
extern "C" {
    pub fn __bth_fecn(_arg: pkt->hdr) -> return;
}
extern "C" {
    pub fn __bth_becn(_arg: pkt->hdr) -> return;
}
extern "C" {
    pub fn __bth_resv6a(_arg: pkt->hdr) -> return;
}
extern "C" {
    pub fn __bth_ack(_arg: pkt->hdr) -> return;
}
extern "C" {
    pub fn __bth_psn(_arg: pkt->hdr) -> return;
}
//
// Reliable Datagram Extended Transport Header
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxe_rdeth {
    pub een: __be32,
}

//
// Datagram Extended Transport Header
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxe_deth {
    pub qkey: __be32,
    pub sqp: __be32,
}

extern "C" {
    pub fn be32_to_cpu(_arg: deth->qkey) -> return;
}
//
// RDMA Extended Transport Header
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxe_reth {
    pub va: __be64,
    pub rkey: __be32,
    pub len: __be32,
}

extern "C" {
    pub fn be64_to_cpu(_arg: reth->va) -> return;
}
extern "C" {
    pub fn be32_to_cpu(_arg: reth->rkey) -> return;
}
extern "C" {
    pub fn be32_to_cpu(_arg: reth->len) -> return;
}
//
// FLUSH Extended Transport Header
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxe_feth {
    pub bits: __be32,
}

extern "C" {
    pub fn __feth_plt(rxe_opcode[pkt->opcode].offset[RXE_FETH]: pkt->hdr +) -> return;
}
extern "C" {
    pub fn __feth_sel(rxe_opcode[pkt->opcode].offset[RXE_FETH]: pkt->hdr +) -> return;
}
//
// Atomic Extended Transport Header
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxe_atmeth {
    pub va: __be64,
    pub rkey: __be32,
    pub swap_add: __be64,
    pub comp: __be64,
    pub __packed: },
    pub arg: *mut *mut rxe_atmeth atmeth =,
    pub be64_to_cpu(atmeth->va): return,
    pub arg: *mut *mut rxe_atmeth atmeth =,
    pub cpu_to_be64(va): atmeth->va =,
    pub arg: *mut *mut rxe_atmeth atmeth =,
    pub be32_to_cpu(atmeth->rkey): return,
    pub arg: *mut *mut rxe_atmeth atmeth =,
    pub cpu_to_be32(rkey): atmeth->rkey =,
    pub arg: *mut *mut rxe_atmeth atmeth =,
    pub be64_to_cpu(atmeth->swap_add): return,
    pub arg: *mut *mut rxe_atmeth atmeth =,
    pub cpu_to_be64(swap_add): atmeth->swap_add =,
    pub arg: *mut *mut rxe_atmeth atmeth =,
    pub be64_to_cpu(atmeth->comp): return,
    pub arg: *mut *mut rxe_atmeth atmeth =,
    pub cpu_to_be64(comp): atmeth->comp =,
    pub va): rxe_opcode[pkt->opcode].offset[RXE_ATMETH],,
    pub rkey): rxe_opcode[pkt->opcode].offset[RXE_ATMETH],,
    pub swap_add): rxe_opcode[pkt->opcode].offset[RXE_ATMETH],,
    pub comp): rxe_opcode[pkt->opcode].offset[RXE_ATMETH],,
//
// Ack Extended Transport Header
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxe_aeth {
    pub smsn: __be32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aeth_syndrome {
    AETH_TYPE_MASK		= 0xe0,
    AETH_ACK		= 0x00,
    AETH_RNR_NAK		= 0x20,
    AETH_RSVD		= 0x40,
    AETH_NAK		= 0x60,
    AETH_ACK_UNLIMITED	= 0x1f,
    AETH_NAK_PSN_SEQ_ERROR	= 0x60,
    AETH_NAK_INVALID_REQ	= 0x61,
    AETH_NAK_REM_ACC_ERR	= 0x62,
    AETH_NAK_REM_OP_ERR	= 0x63,
}

//
// Atomic Ack Extended Transport Header
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxe_atmack {
    pub orig: __be64,
}

extern "C" {
    pub fn be64_to_cpu(_arg: atmack->orig) -> return;
}
//
// Immediate Extended Transport Header
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxe_immdt {
    pub imm: __be32,
}

//
// Invalidate Extended Transport Header
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxe_ieth {
    pub rkey: __be32,
}

extern "C" {
    pub fn be32_to_cpu(_arg: ieth->rkey) -> return;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rxe_hdr_length {
    RXE_BTH_BYTES		= sizeof(struct rxe_bth),
    RXE_DETH_BYTES		= sizeof(struct rxe_deth),
    RXE_IMMDT_BYTES		= sizeof(struct rxe_immdt),
    RXE_RETH_BYTES		= sizeof(struct rxe_reth),
    RXE_AETH_BYTES		= sizeof(struct rxe_aeth),
    RXE_ATMACK_BYTES	= sizeof(struct rxe_atmack),
    RXE_ATMETH_BYTES	= sizeof(struct rxe_atmeth),
    RXE_IETH_BYTES		= sizeof(struct rxe_ieth),
    RXE_RDETH_BYTES		= sizeof(struct rxe_rdeth),
    RXE_FETH_BYTES		= sizeof(struct rxe_feth),
}
