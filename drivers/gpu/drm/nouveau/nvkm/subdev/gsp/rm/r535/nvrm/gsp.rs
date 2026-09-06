//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/gsp/rm/r535/nvrm/gsp.h
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


// SPDX-License-Identifier: MIT
// Copyright (c) 2025, NVIDIA CORPORATION. All rights reserved.

// Excerpt of RM headers from https://github.com/NVIDIA/open-gpu-kernel-modules/tree/535.113.01

pub const NV0080_CTRL_GR_CAPS_TBL_SIZE: c_int = 23;

pub const MAX_GPC_COUNT: c_int = 32;

pub const NVGPU_ENGINE_CAPS_MASK_BITS: c_int = 32;

// Client handle for internal RMAPI control.
// Device handle for internal RMAPI control.
// Subdevice handle for internal RMAPI control.

// Magic
// BL to use for verification (i.e. Booter locked it in WPR2)
// Revision number of Booter-BL-Sequencer handoff interface
// Bumped up when we change this interface so it is not backward compatible.
// Bumped up when we revoke GSP-RM ucode
// ---- Members regarding data in SYSMEM ----------------------------
// Consumed by Booter for DMA
// Offsets inside bootloader image needed by Booter
// Used only at initial boot
//
// Used at suspend/resume to read GspFwHeapFreeList
// Offset relative to GspFwWprMeta FBMEM PA (gspFwWprStart)
//
// ---- Members describing FB layout --------------------------------
// GSP-RM to use to setup heap.
// BL to use to find ELF for jump
// Size is sizeOfRadix3Elf above.
// Size is sizeOfBootloader above.
// GSP-RM to use for fbRegionInfo?
// ---- Other members -----------------------------------------------
// GSP-RM to use for fbRegionInfo?
// Boot count.  Used to determine whether to load the firmware image.
// This union is organized the way it is to start at an 8-byte boundary and achieve natural
// packing of the internal struct fields.
// TODO: the partitionRpc* fields below do not really belong in this
// structure. The values are patched in by the partition bootstrapper
// when GSP-RM is booted in a partition, and this structure was a
// convenient place for the bootstrapper to access them. These should
// be moved to a different comm. mechanism between the bootstrapper
// and the GSP-RM tasks.
// Shared partition RPC memory (physical address)
// Offsets relative to partitionRpcAddr
// Code section and dataSection offset and size.
// Used during GSP-RM resume to check for revocation
// Pad for the partitionRpc* fields, plus 4 bytes
// CrashCat (contiguous) buffer size/location - occupies same bytes as the
// elf(Code|Data)(Offset|Size) fields above.
// TODO: move to GSP_FMC_INIT_PARAMS
// Pad for the lsUcodeVersion field
// Number of VF partitions allocating sub-heaps from the WPR heap
// Used during boot to ensure the heap is adequately sized
// Pad structure to exactly 256 bytes.  Can replace padding with additional
// fields without incrementing revision.  Padding initialized to 0.
// BL to use for verification (i.e. Booter says OK to boot)
pub const GSP_FW_WPR_META_MAGIC: c_uint = 0xdc3aae21371a60b3ULL;
pub const GSP_FW_WPR_META_REVISION: c_int = 1;

pub type LibosAddress = NvU64;
//
// Magic
// Use for verification by Booter
//
// Revision number
// Bumped up when we change this interface so it is not backward compatible.
// Bumped up when we revoke GSP-RM ucode
//
// ---- Members regarding data in SYSMEM ----------------------------
// Consumed by Booter for DMA
//
// ---- Members for crypto ops across S/R ---------------------------
//
// HMAC over the entire GspFwSRMeta structure (including padding)
// with the hmac field itself zeroed.
//
// Hash over GspFwWprMeta structure
// Hash over GspFwHeapFreeList structure. All zeros signifies no free list.
// Hash over data in WPR2 (skipping over free heap chunks; see Booter for details)
//
// Pad structure to exactly 256 bytes (1 DMA chunk).
// Padding initialized to zero.
//
pub const GSP_FW_SR_META_MAGIC: c_uint = 0x8a3bb9e6c6c39d93ULL;
pub const GSP_FW_SR_META_REVISION: c_int = 2;

// GSP_SEQ_BUF_OPCODE_CORE_RESET */                                 \
// GSP_SEQ_BUF_OPCODE_CORE_START */                                 \
// GSP_SEQ_BUF_OPCODE_CORE_WAIT_FOR_HALT */                         \
// GSP_SEQ_BUF_OPCODE_CORE_RESUME */                                \
//
// Version 1
// Version 2
// Version 3 = for Partition boot
// Version 4 = for eb riscv boot
// Version 5 = Support signing entire RISC-V image as "code" in code section for hopper and later.
//
// Manifest contains information about Monitor and it is
// input to BR
//
// Monitor Data offset within RISCV image and size
//
// Monitor Code offset withtin RISCV image and size
//
// Swbrom Code offset within RISCV image and size
//
// Swbrom Data offset within RISCV image and size
//
// Total size of FB carveout (image and reserved space).
//
// Indicates whether the entire RISC-V image is signed as "code" in code section.
//
pub const NV2080_CTRL_INTERNAL_INTR_MAX_TABLE_SIZE: c_int = 128;

pub type rpc_message_rpc_union_field_v = rpc_message_rpc_union_field_v03_00;
pub type rpc_message_header_v = rpc_message_header_v03_00;

