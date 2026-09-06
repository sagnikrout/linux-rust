//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/qla2xxx/qla_inline.h
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
// QLogic Fibre Channel HBA Driver
// Copyright (c)  2003-2014 QLogic Corporation
//

//
// qla24xx_calc_iocbs() - Determine number of Command Type 3 and
// Continuation Type 1 IOCBs to allocate.
//
// @vha: HA context
// @dsds: number of data segment descriptors needed
//
// Returns the number of IOCB entries needed to store @dsds.
//
// qla2x00_debounce_register
// Debounce register.
//
// Input:
// port = register address.
//
// Returns:
// register value.
//
// qla29xx_calc_iocbs() - Determine number of Command-Type and Continuation
// IOCBs to allocate for the 29xx extended (128-byte) IOCB ring.
// @vha: HA context
// @dsds: number of data segment descriptors needed
// @iocb_dsds: number of DSDs embedded in the first (command) IOCB.  The
// remaining DSDs ride on Continuation Type 1 Ext IOCBs which hold
// NUM_CONT1_DSDS (10) each.
//
// Returns the total number of IOCB entries needed to carry @dsds.
//
// qla_req_entry_size() - request-ring entry stride.
// @ha: HBA pointer
//
// Returns sizeof(struct request_ext) (128) on 29xx, sizeof(request_t) (64)
// everywhere else.
//
extern "C" {
    pub fn IS_QLA29XX(sizeof(request_t: ha) ? sizeof(struct request_ext) :) -> return;
}
//
// qla_rsp_entry_size() - response-ring entry stride.
// @ha: HBA pointer
//
// Counterpart of qla_req_entry_size() for the response ring.
//
extern "C" {
    pub fn IS_QLA29XX(sizeof(response_t: ha) ? sizeof(struct response_ext) :) -> return;
}
//
// qla_sts_cont_data_size() - status-continuation IOCB data payload size.
// @ha: HBA pointer
//
// sts_cont_entry_t and struct sts_cont_entry_ext share the same header and
// data offset; only the trailing data[] size differs (60 vs 124 bytes).
// Returns that size so callers need not branch on the adapter type.
//
// qla_logio_set_vp_index() - write vp_index into a login/logout IOCB.
// @ha: HBA pointer
// @pkt: logio IOCB (logio_entry_24xx or logio_entry_24xx_ext)
// @vp_idx: virtual port index
//
// vp_index widens from u8 (logio_entry_24xx) to __le16
// (logio_entry_24xx_ext) on 29xx; write the field at the right width.
//
// Request/response queues are bounded by the MSI-X vector count less
// the mailbox vector.  These counters are u8, so a board advertising
// e.g. 257 vectors would truncate msix_count - 1 (256) to 0 and hand
// kzalloc_objs() a zero count (ZERO_SIZE_PTR), faulting on the first
// ha->req_q_map[0] store.  Clamp into [1, QLA_MAX_QUEUES - 1].
//
extern "C" {
    pub fn clamp_t(_arg: u16, 1: msix_count -, _arg: 1, 1: QLA_MAX_QUEUES -) -> return;
}
// ofcp++ = swab32(*ifcp++);
// odest++ = cpu_to_le32(*isrc);
// clean up allocated prev pool
// This will have to change when the max no. of states > 16
//
// Uncomment when corresponding SCSI changes are done.
//
// Test appropriate base-vha and vha flags.
// ref : INIT - normal flow
extern "C" {
    pub fn qla2xxx_rel_done_warning(sp: *mut srb_t, res: c_int);
}
extern "C" {
    pub fn qla2xxx_rel_free_warning(sp: *mut srb_t);
}
pub const SQ_SCOPE_MASK: c_uint = 0xc000 /* SAM-6 rev5 5.3.2 */;
pub const SQ_SCOPE_SHIFT: c_int = 14;
pub const SQ_QUAL_MASK: c_uint = 0x3fff;

// Handle only scope 1 or 2, which is for I-T nexus.
// Skip processing, if retry delay timer is already in effect.
// qual is expressed in 100ms increments.
//
// 29xx uses the 128-byte-strided extended request ring; advance the
// matching ring_ext_ptr so the next IOCB allocator sees the correct
// slot.  All other 83xx-family generations (83xx/27xx/28xx) keep the
// 64-byte ring_ptr.
//
// qla_rsp_ring_advance() - Advance the response queue consumer pointer
// to the next IOCB slot, handling both 24xx (64-byte) and 29xx (128-byte)
// ring strides.
//
// On 29xx, ring_ext_ptr is the authoritative slot pointer (correct 128-byte
// pitch) and ring_ptr is kept in sync as a response_t view of the same slot
// so existing 24xx-shaped reads (rsp->ring_ptr->signature,
// (struct sts_entry_24xx *)rsp->ring_ptr, etc.) keep working unchanged; the
// first 64 bytes of struct response_ext are layout-compatible with response_t.
//
// qla_req_ring_slot() - return the current request-ring producer slot.
// @ha: HBA pointer
// @req: request queue
//
// On 29xx the firmware-visible ring uses 128-byte-strided entries
// referenced by ring_ext_ptr; on earlier adapters the 64-byte ring
// referenced by ring_ptr is used.  The returned pointer is
// layout-compatible with request_t for common header writes; callers
// needing 29xx-specific fields should cast to struct request_ext.
//
// qla_req_ring_advance() - advance request-ring producer pointer.
// @ha: HBA pointer
// @req: request queue
//
// Mirrors qla_rsp_ring_advance().  Does NOT publish the new producer
// index to firmware; callers that need to do so should follow with a
// wrt_reg_dword or qla_83xx_start_iocbs().
//
// qla_rsp_ring_rewind_to() - Restore the response queue consumer pointer
// to a previously-observed slot (used when we need to defer processing an
// IOCB whose continuation entries have not yet arrived).
// @rsp: response queue
// @pkt: 64-byte view of the slot to rewind to (captured from a prior read
// of rsp->ring_ptr)
// @idx: matching ring_index value (also captured before the advance)
//
// On 29xx, pkt was originally obtained as (response_t *)rsp->ring_ext_ptr,
// so casting back to struct response_ext * recovers the 128-byte-stride slot
// pointer.
//
// no need to acquire qpair lock. It's just rough calculation
//
// decrement to zero.  This routine will not decrement below zero
// @v:  pointer of type atomic_t
// @amount: amount to decrement from v
//
// should not happen
pub const ISP_REG_DISCONNECT: c_uint = 0xffffffffU;
//
// qla2x00_isp_reg_stat
//
// Description:
// Read the host status register of ISP before aborting the command.
//
// Input:
// ha = pointer to host adapter structure.
//
// Returns:
// Either true or false.
//
// Note: Return true if there is register disconnect.
//
// Common fields extracted from FWI2 status IOCBs.  Populated once so
// callers avoid duplicated IS_QLA29XX() branches for every field access.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_sts_fwi2 {
    pub data: *mut u8,
    pub data_sz: u32,
    pub scsi_status: u16,
    pub sts_qual: u16,
    pub sense_len: u32,
    pub rsp_data_len: u32,
    pub rsp_residual_count: u32,
}

//
// qla_els_set_vp_sof() - write the vp_index / sof_type pair into an ELS
// pass-through IOCB (els_entry_24xx{,_ext}).
//
// Both layouts have the same 16-bit slot at offset 14, but it is encoded
// differently:
// - 24xx: separate u8 vp_index + u8 sof_type with EST_SOFI3 (1 << 4)
// - 29xx: __le16 vp_index_sof with bits [8:0]=VP index, [15:12]=SOF type
// and ELS_EXT_EST_SOFI3
// so this is the single point in the driver that knows about that
// encoding split.
//
