//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath10k/ce.h
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


// SPDX-License-Identifier: ISC
//
// Copyright (c) 2005-2011 Atheros Communications Inc.
// Copyright (c) 2011-2017 Qualcomm Atheros, Inc.
// Copyright (c) 2018 The Linux Foundation. All rights reserved.
//

pub const CE_HTT_H2T_MSG_SRC_NENTRIES: c_int = 8192;
// Descriptor rings must be aligned to this boundary
pub const CE_DESC_RING_ALIGN: c_int = 8;
pub const CE_SEND_FLAG_GATHER: c_uint = 0x00010000;
//
// Copy Engine support: low-level Target-side Copy Engine API.
// This is a hardware access layer used by code that understands
// how to use copy engines.
//

// Following desc flags are used in QCA99X0

pub const CE_DDR_DRRI_SHIFT: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ce_desc {
    pub addr: __le32,
    pub nbytes: __le16,
    pub /: *mut *mut __le16 flags; / %CE_DESC_FLAGS_,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ce_desc_64 {
    pub addr: __le64,
    pub /: *mut *mut __le16 nbytes; / length in register map,
    pub /: *mut *mut __le16 flags; / fw_metadata_high,
    pub toeplitz_hash_result: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_ce_ring {
// Number of entries in this ring; must be power of 2
    pub nentries: c_uint,
    pub nentries_mask: c_uint,
//
// For dest ring, this is the next index to be processed
// by software after it was/is received into.
//
// For src ring, this is the last descriptor that was sent
// and completion processed by software.
//
// Regardless of src or dest ring, this is an invariant
// (modulo ring size):
// write index >= read index >= sw_index
//
    pub sw_index: c_uint,
// cached copy
    pub write_index: c_uint,
//
// For src ring, this is the next index not yet processed by HW.
// This is a cached copy of the real HW index (read index), used
// for avoiding reading the HW index register more often than
// necessary.
// This extends the invariant:
// write index >= read index >= hw_index >= sw_index
//
// For dest ring, this is currently unused.
//
// cached copy
    pub hw_index: c_uint,
// Start of DMA-coherent area reserved for descriptors
// Host address space
    pub base_addr_owner_space_unaligned: *mut c_void,
// CE address space
    pub base_addr_ce_space_unaligned: dma_addr_t,
//
// Actual start of descriptors.
// Aligned to descriptor-size boundary.
// Points into reserved DMA-coherent area, above.
//
// Host address space
    pub base_addr_owner_space: *mut c_void,
// CE address space
    pub base_addr_ce_space: dma_addr_t,
    pub shadow_base_unaligned: *mut c_char,
    pub shadow_base: *mut ce_desc_64,
// keep last
    pub __counted_by(nentries): *mut *mut void per_transfer_context[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_ce_pipe {
    pub ar: *mut ath10k,
    pub id: c_uint,
    pub attr_flags: c_uint,
    pub ctrl_addr: u32,
    pub ): *mut *mut void (send_cb)(struct ath10k_ce_pipe,
    pub ): *mut *mut void (recv_cb)(struct ath10k_ce_pipe,
    pub src_sz_max: c_uint,
    pub src_ring: *mut ath10k_ce_ring,
    pub dest_ring: *mut ath10k_ce_ring,
    pub ops: *const ath10k_ce_ops,
}

// Copy Engine settable attributes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_bus_ops {
    pub offset): *mut *mut *mut u32 (read32)(struct ath10k ar, u32,
    pub value): *mut *mut *mut void (write32)(struct ath10k ar, u32 offset, u32,
    pub ar): *mut *mut int (get_num_banks)(struct ath10k,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_ce {
// protects CE info
    pub ce_lock: spinlock_t,
    pub bus_ops: *const ath10k_bus_ops,
    pub ce_states: [ath10k_ce_pipe; CE_COUNT_MAX],
    pub vaddr_rri: *mut u32,
    pub paddr_rri: dma_addr_t,
}

// ==================Send====================
// ath10k_ce_send flags
pub const CE_SEND_FLAG_BYTE_SWAP: c_int = 1;
//
// Queue a source buffer to be sent to an anonymous destination buffer.
// ce         - which copy engine to use
// buffer          - address of buffer
// nbytes          - number of bytes to send
// transfer_id     - arbitrary ID; reflected to destination
// flags           - CE_SEND_FLAG_* values
// Returns 0 on success; otherwise an error status.
//
// Note: If no flags are specified, use CE's default data swap mode.
//
// Implementation note: pushes 1 buffer to Source ring
//
// 14 bits
extern "C" {
    pub fn __ath10k_ce_send_revert(pipe: *mut ath10k_ce_pipe);
}
extern "C" {
    pub fn ath10k_ce_num_free_src_entries(pipe: *mut ath10k_ce_pipe) -> c_int;
}
// ==================Recv=======================
extern "C" {
    pub fn __ath10k_ce_rx_num_free_bufs(pipe: *mut ath10k_ce_pipe) -> c_int;
}
extern "C" {
    pub fn ath10k_ce_rx_update_write_idx(pipe: *mut ath10k_ce_pipe, nentries: u32);
}
// recv flags
// Data is byte-swapped
pub const CE_RECV_FLAG_SWAPPED: c_int = 1;
//
// Supply data for the next completed unprocessed receive descriptor.
// Pops buffer from Dest ring.
//
// Supply data for the next completed unprocessed send descriptor.
// Pops 1 completed send buffer from Source ring.
//
// ==================CE Engine Initialization=======================
extern "C" {
    pub fn ath10k_ce_deinit_pipe(ar: *mut ath10k, ce_id: c_uint);
}
extern "C" {
    pub fn ath10k_ce_free_pipe(ar: *mut ath10k, ce_id: c_int);
}
// ==================CE Engine Shutdown=======================
//
// Support clean shutdown by allowing the caller to revoke
// receive buffers.  Target DMA must be stopped before using
// this API.
//
// Support clean shutdown by allowing the caller to cancel
// pending sends.  Target DMA must be stopped before using
// this API.
//
// ==================CE Interrupt Handlers====================
extern "C" {
    pub fn ath10k_ce_per_engine_service_any(ar: *mut ath10k);
}
extern "C" {
    pub fn ath10k_ce_per_engine_service(ar: *mut ath10k, ce_id: c_uint);
}
extern "C" {
    pub fn ath10k_ce_disable_interrupt(ar: *mut ath10k, ce_id: c_int);
}
extern "C" {
    pub fn ath10k_ce_disable_interrupts(ar: *mut ath10k);
}
extern "C" {
    pub fn ath10k_ce_enable_interrupt(ar: *mut ath10k, ce_id: c_int);
}
extern "C" {
    pub fn ath10k_ce_enable_interrupts(ar: *mut ath10k);
}
extern "C" {
    pub fn ath10k_ce_alloc_rri(ar: *mut ath10k);
}
extern "C" {
    pub fn ath10k_ce_free_rri(ar: *mut ath10k);
}
// ce_attr.flags values
// Use NonSnooping PCIe accesses?

// Byte swap data words

// Swizzle descriptors?

// no interrupt on copy completion

// no interrupt, only polling

// Attributes of an instance of a Copy Engine
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ce_attr {
// CE_ATTR_* values
    pub flags: c_uint,
// #entries in source ring - Must be a power of 2
    pub src_nentries: c_uint,
//
// Max source send size for this CE.
// This is also the minimum size of a destination buffer.
//
    pub src_sz_max: c_uint,
// #entries in destination ring - Must be a power of 2
    pub dest_nentries: c_uint,
    pub ): *mut *mut void (send_cb)(struct ath10k_ce_pipe,
    pub ): *mut *mut void (recv_cb)(struct ath10k_ce_pipe,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_ce_ops {
    pub attr): *const ce_attr,
    pub attr): *const ce_attr,
    pub paddr): dma_addr_t,
    pub nbytesp): *mut u32,
    pub nbytesp): *mut dma_addr_t,
    pub transfer_idp): *mut *mut u32 nbytesp, u32,
    pub ce_id): *mut *mut *mut void (ce_free_pipe)(struct ath10k ar, int,
    pub flags): u32 transfer_id, u32,
    pub addr): u64,
    pub addr): u64,
    pub per_transfer_contextp): *mut c_void,
}

// Ring arithmetic (modulus number of entries in ring, which is a pwr of 2).

pub const CE_WRAPPER_INTERRUPT_SUMMARY_ADDRESS: c_uint = 0x0000;
// Host software's Copy Engine configuration.
pub const CE_ATTR_FLAGS: c_int = 0;
//
// Configuration information for a Copy Engine pipe.
// Passed from Host to Target during startup (one per CE).
//
// NOTE: Structure is shared between Host software and Target firmware!
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ce_pipe_config {
    pub pipenum: __le32,
    pub pipedir: __le32,
    pub nentries: __le32,
    pub nbytes_max: __le32,
    pub flags: __le32,
    pub reserved: __le32,
}

//
// Directions for interconnect pipe configuration.
// These definitions may be used during configuration and are shared
// between Host and Target.
//
// Pipe Directions are relative to the Host, so PIPEDIR_IN means
// "coming IN over air through Target to Host" as with a WiFi Rx operation.
// Conversely, PIPEDIR_OUT means "going OUT from Host through Target over air"
// as with a WiFi Tx operation. This is somewhat awkward for the "middle-man"
// Target since things that are "PIPEDIR_OUT" are coming IN to the Target
// over the interconnect.
//
pub const PIPEDIR_NONE: c_int = 0;

// Establish a mapping between a service/direction and a pipe.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ce_service_to_pipe {
    pub service_id: __le32,
    pub pipedir: __le32,
    pub pipenum: __le32,
}
