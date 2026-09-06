//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/imagination/pvr_rogue_meta.h
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
// The META HW register definitions in the file are updated manually

//
// META registers and MACROS
//

// Poll for done.

// Set for read.

// Internal ctrl regs.

// Data unit regs.

// Data unit regs.

// Address unit regs.

// Address unit regs.

// PC registers.

// Macros to calculate register access values.

//
// META LDR Format
//
// Block header structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_meta_ldr_block_hdr {
    pub dev_id: u32,
    pub sl_code: u32,
    pub sl_data: u32,
    pub pc_ctrl: u16,
    pub crc: u16,
}

// High level data stream block structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_meta_ldr_l1_data_blk {
    pub cmd: u16,
    pub length: u16,
    pub next: u32,
    pub cmd_data: [u32; 4],
}

// High level data stream block structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_meta_ldr_l2_data_blk {
    pub tag: u16,
    pub length: u16,
    pub block_data: [u32; 4],
}

// Config command structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_meta_ldr_cfg_blk {
    pub type: u32,
    pub block_data: [u32; 4],
}

// Block type definitions

//
// Command definitions
// Value   Name            Description
// 0       LoadMem         Load memory with binary data.
// 1       LoadCore        Load a set of core registers.
// 2       LoadMMReg       Load a set of memory mapped registers.
// 3       StartThreads    Set each thread PC and SP, then enable threads.
// 4       ZeroMem         Zeros a memory region.
// 5       Config          Perform a configuration command.
//

//
// Config Command definitions
// Value   Name        Description
// 0       Pause       Pause for x times 100 instructions
// 1       Read        Read a value from register - No value return needed.
// Utilises effects of issuing reads to certain registers
// 2       Write       Write to mem location
// 3       MemSet      Set mem to value
// 4       MemCheck    check mem for specific value.
//

//
// ROGUE FW segmented MMU definitions
//
// All threads can access the segment.

// Writable.

// All threads can access and writable.

// Direct map region 10 used for mapping GPU memory - max 8MB.

// Segment IDs.

//
// SLC caching strategy in S7 and volcanic is emitted through the segment MMU.
// All the segments configured through the macro ROGUE_FW_SEGMMU_OUTADDR_TOP are
// CACHED in the SLC.
// The interface has been kept the same to simplify the code changes.
// The bifdm argument is ignored (no longer relevant) in S7 and volcanic.
//

//
// To configure the Page Catalog and BIF-DM fed into the BIF for Garten
// accesses through this segment.
//

// META segments have 4kB minimum size.

// Segmented MMU registers (n = segment id).

//
// The following defines must be recalculated if the Meta MMU segments used
// to access Host-FW data are changed
// Current combinations are:
// - SLC uncached, META cached,   FW base address 0x70000000
// - SLC uncached, META uncached, FW base address 0xF0000000
// - SLC cached,   META cached,   FW base address 0x10000000
// - SLC cached,   META uncached, FW base address 0x90000000
//

//
// For non-VIVT SLCs the cacheability of the FW data in the SLC is selected in
// the PTEs for the FW data, not in the Meta Segment MMU, which means these
// defines have no real effect in those cases.
//

//
// ROGUE FW Bootloader defaults
//

// Bootloader configuration offset is in dwords (512 bytes)

//
// ROGUE META Stack
//

//
// ROGUE META Core memory
//
// Code and data both map to the same physical memory.

//
// 2nd thread
//

//
// META compatibility
//

pub const ROGUE_CR_META_MTP218_CORE_ID_VALUE: c_uint = 0x19;
pub const ROGUE_CR_META_MTP219_CORE_ID_VALUE: c_uint = 0x1E;
pub const ROGUE_CR_META_LTP218_CORE_ID_VALUE: c_uint = 0x1C;
pub const ROGUE_CR_META_LTP217_CORE_ID_VALUE: c_uint = 0x1F;

