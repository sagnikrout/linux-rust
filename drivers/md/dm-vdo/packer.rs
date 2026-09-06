//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/dm-vdo/packer.h
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
// Copyright 2023 Red Hat
//

// The header of a compressed block.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct compressed_block_header {
// Unsigned 32-bit major and minor versions, little-endian
    pub version: packed_version_number,
// List of unsigned 16-bit compressed block sizes, little-endian
    pub sizes: [__le16; VDO_MAX_COMPRESSION_SLOTS],
    pub __packed: },
//
// A compressed block is only written if we can pack at least two fragments into it, so a
// fragment which fills the entire data portion of a compressed block is too big.
//
}

// * The compressed block overlay.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct compressed_block {
    pub header: compressed_block_header,
    pub data: [c_char; VDO_COMPRESSED_BLOCK_DATA_SIZE],
    pub __packed: },
//
// Each packer_bin holds an incomplete batch of data_vios that only partially fill a compressed
// block. The bins are kept in a list sorted by the amount of unused space so the first bin with
// enough space to hold a newly-compressed data_vio can easily be found. When the bin fills up or
// is flushed, the first uncanceled data_vio in the bin is selected to be the agent for that bin.
// Upon entering the packer, each data_vio already has its compressed data in the first slot of the
// data_vio's compressed_block (overlaid on the data_vio's scratch_block). So the agent's fragment
// is already in place. The fragments for the other uncanceled data_vios in the bin are packed into
// the agent's compressed block. The agent then writes out the compressed block. If the write is
// successful, the agent shares its pbn lock which each of the other data_vios in its compressed
// block and sends each on its way. Finally the agent itself continues on the write path as before.
//
// There is one special bin which is used to hold data_vios which have been canceled and removed
// from their bin by the packer. These data_vios need to wait for the canceller to rendezvous with
// them and so they sit in this special bin.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct packer_bin {
// List links for packer.packer_bins
    pub list: list_head,
// The number of items in the bin
    pub slots_used: slot_number_t,
// The number of compressed block bytes remaining in the current batch
    pub free_space: usize,
// The current partial batch of data_vios, waiting for more
    pub incoming: [*mut data_vio; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct packer {
// The ID of the packer's callback thread
    pub thread_id: thread_id_t,
// The number of bins
    pub size: block_count_t,
// A list of all packer_bins, kept sorted by free_space
    pub bins: list_head,
//
// A bin to hold data_vios which were canceled out of the packer and are waiting to
// rendezvous with the canceling data_vio.
//
    pub canceled_bin: *mut packer_bin,
// The current flush generation
    pub flush_generation: sequence_number_t,
// The administrative state of the packer
    pub state: admin_state,
// Statistics are only updated on the packer thread, but are accessed from other threads
    pub statistics: packer_statistics,
}

extern "C" {
    pub fn vdo_free_packer(packer: *mut packer);
}
extern "C" {
    pub fn vdo_get_packer_statistics(packer: *const packer) -> packer_statistics __must_check;
}
extern "C" {
    pub fn vdo_attempt_packing(data_vio: *mut data_vio);
}
extern "C" {
    pub fn vdo_flush_packer(packer: *mut packer);
}
extern "C" {
    pub fn vdo_remove_lock_holder_from_packer(completion: *mut vdo_completion);
}
extern "C" {
    pub fn vdo_increment_packer_flush_generation(packer: *mut packer);
}
extern "C" {
    pub fn vdo_drain_packer(packer: *mut packer, completion: *mut vdo_completion);
}
extern "C" {
    pub fn vdo_resume_packer(packer: *mut packer, parent: *mut vdo_completion);
}
extern "C" {
    pub fn vdo_dump_packer(packer: *const packer);
}
