//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/platforms/cell/spufs/spu_utils.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// utils.h: Utilities for SPU-side of the context switch operation.
//
// (C) Copyright IBM 2005
//
// 64-bit safe EA.
//
// 128-bit register template.
//
// DMA list structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_list_elem {
    pub size: c_uint,
    pub ea_low: c_uint,
}

//
// Declare storage for 8-byte aligned DMA list.
//
// External definition for storage
// declared in crt0.
//
// Compute LSCSA byte offset for a given field.
//

// Save, Step 4:
// Restore, Step 1:
// Set the SPU_RdEventMsk channel to zero to mask
// all events.
//
// Save, Step 5:
// Restore, Step 2:
// Set the SPU_WrTagMsk channel to '01' to unmask
// only tag group 0.
//
// Save, Step 6:
// Restore, Step 3:
// Update the effective address for the CSA in the
// pre-canned DMA-list in local storage.
//
// Save, Step 12:
// Restore, Step 7:
// Send a PUTLLC (tag 0) command to the MFC using
// an effective address in the CSA in order to
// remove any possible lock-line reservation.
//
// Save, Step 15:
// Restore, Step 8:
// Write the MFC_TagUpdate channel with '01'.
//
// Save, Step 16:
// Restore, Step 9:
// Read the MFC_TagStat channel data.
//
// Save, Step 17:
// Restore, Step 10:
// Read the MFC_AtomicStat channel data.
//
