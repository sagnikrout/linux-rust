//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/imagination/pvr_cccb.h
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


// SPDX-License-Identifier: GPL-2.0-only OR MIT
// Copyright (c) 2023 Imagination Technologies Ltd.

// Forward declaration from pvr_device.h.
// Forward declaration from pvr_gem.h.
// Forward declaration from pvr_hwrt.h.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvr_cccb {
// @ctrl_obj: FW object representing CCCB control structure.
    pub ctrl_obj: *mut pvr_fw_object,
// @ccb_obj: FW object representing CCCB.
    pub cccb_obj: *mut pvr_fw_object,
//
// @ctrl: Kernel mapping of CCCB control structure. @lock must be held
// when accessing.
//
    pub ctrl: *mut rogue_fwif_cccb_ctl,
// @cccb: Kernel mapping of CCCB. @lock must be held when accessing.
    pub cccb: *mut u8,
// @ctrl_fw_addr: FW virtual address of CCCB control structure.
    pub ctrl_fw_addr: u32,
// @ccb_fw_addr: FW virtual address of CCCB.
    pub cccb_fw_addr: u32,
// @size: Size of CCCB in bytes.
    pub size: usize,
// @write_offset: CCCB write offset.
    pub write_offset: u32,
// @wrap_mask: CCCB wrap mask.
    pub wrap_mask: u32,
}

extern "C" {
    pub fn pvr_cccb_fini(cccb: *mut pvr_cccb);
}
extern "C" {
    pub fn pvr_cccb_cmdseq_fits(pvr_cccb: *mut pvr_cccb, size: usize) -> bool;
}
//
// pvr_cccb_get_size_of_cmd_with_hdr() - Get the size of a command and its header.
// @cmd_size: Command size.
//
// Returns the size of the command and its header.
//
extern "C" {
    pub fn sizeof(ALIGN(cmd_size: rogue_fwif_ccb_cmd_header) +, _arg: 8) -> return;
}
//
// pvr_cccb_cmdseq_can_fit() - Check if a command sequence can fit in the CCCB.
// @pvr_cccb: Target Client CCB.
// @size: Command sequence size.
//
// Returns:
// * true it the CCCB is big enough to contain a command sequence, or
// * false otherwise.
//
// We divide the capacity by two to simplify our CCCB fencing logic:
// we want to be sure that, no matter what we had queued before, we
// are able to either queue our command sequence at the end or add a
// padding command and queue the command sequence at the beginning
// of the CCCB. If the command sequence size is bigger than half the
// CCCB capacity, we'd have to queue the padding command and make sure
// the FW is done processing it before queueing our command sequence.
//
