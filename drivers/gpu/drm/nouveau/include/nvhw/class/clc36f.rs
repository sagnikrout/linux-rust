//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvhw/class/clc36f.h
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
//
// Copyright (c) 2025, NVIDIA CORPORATION. All rights reserved.
//

// Macro flag: #define _clc36f_h_

// NOTE - MEM_OP_A and MEM_OP_B have been replaced in gp100 with methods for
// specifying the page address for a targeted TLB invalidate and the uTLB for
// a targeted REPLAY_CANCEL for UVM.
// The previous MEM_OP_A/B functionality is in MEM_OP_C/D, with slightly
// rearranged fields.

pub const NVC36F_MEM_OP_A_TLB_INVALIDATE_SYSMEMBAR_EN: c_uint = 0x00000001;
pub const NVC36F_MEM_OP_A_TLB_INVALIDATE_SYSMEMBAR_DIS: c_uint = 0x00000000;

pub const NVC36F_MEM_OP_C_MEMBAR_TYPE_SYS_MEMBAR: c_uint = 0x00000000;
pub const NVC36F_MEM_OP_C_MEMBAR_TYPE_MEMBAR: c_uint = 0x00000001;

pub const NVC36F_MEM_OP_C_TLB_INVALIDATE_PDB_ONE: c_uint = 0x00000000;
pub const NVC36F_MEM_OP_C_TLB_INVALIDATE_PDB_ALL: c_uint = 0x00000001  // Probably nonsensical for MMU_TLB_INVALIDATE_TARGETED;

pub const NVC36F_MEM_OP_C_TLB_INVALIDATE_GPC_ENABLE: c_uint = 0x00000000;
pub const NVC36F_MEM_OP_C_TLB_INVALIDATE_GPC_DISABLE: c_uint = 0x00000001;

pub const NVC36F_MEM_OP_C_TLB_INVALIDATE_REPLAY_NONE: c_uint = 0x00000000;
pub const NVC36F_MEM_OP_C_TLB_INVALIDATE_REPLAY_START: c_uint = 0x00000001;
pub const NVC36F_MEM_OP_C_TLB_INVALIDATE_REPLAY_START_ACK_ALL: c_uint = 0x00000002;
pub const NVC36F_MEM_OP_C_TLB_INVALIDATE_REPLAY_CANCEL_TARGETED: c_uint = 0x00000003;
pub const NVC36F_MEM_OP_C_TLB_INVALIDATE_REPLAY_CANCEL_GLOBAL: c_uint = 0x00000004;
pub const NVC36F_MEM_OP_C_TLB_INVALIDATE_REPLAY_CANCEL_VA_GLOBAL: c_uint = 0x00000005;

pub const NVC36F_MEM_OP_C_TLB_INVALIDATE_ACK_TYPE_NONE: c_uint = 0x00000000;
pub const NVC36F_MEM_OP_C_TLB_INVALIDATE_ACK_TYPE_GLOBALLY: c_uint = 0x00000001;
pub const NVC36F_MEM_OP_C_TLB_INVALIDATE_ACK_TYPE_INTRANODE: c_uint = 0x00000002;

pub const NVC36F_MEM_OP_C_TLB_INVALIDATE_ACCESS_TYPE_VIRT_READ: c_int = 0;
pub const NVC36F_MEM_OP_C_TLB_INVALIDATE_ACCESS_TYPE_VIRT_WRITE: c_int = 1;
pub const NVC36F_MEM_OP_C_TLB_INVALIDATE_ACCESS_TYPE_VIRT_ATOMIC_STRONG: c_int = 2;
pub const NVC36F_MEM_OP_C_TLB_INVALIDATE_ACCESS_TYPE_VIRT_RSVRVD: c_int = 3;
pub const NVC36F_MEM_OP_C_TLB_INVALIDATE_ACCESS_TYPE_VIRT_ATOMIC_WEAK: c_int = 4;
pub const NVC36F_MEM_OP_C_TLB_INVALIDATE_ACCESS_TYPE_VIRT_ATOMIC_ALL: c_int = 5;
pub const NVC36F_MEM_OP_C_TLB_INVALIDATE_ACCESS_TYPE_VIRT_WRITE_AND_ATOMIC: c_int = 6;
pub const NVC36F_MEM_OP_C_TLB_INVALIDATE_ACCESS_TYPE_VIRT_ALL: c_int = 7;

pub const NVC36F_MEM_OP_C_TLB_INVALIDATE_PAGE_TABLE_LEVEL_ALL: c_uint = 0x00000000  // Invalidate tlb caches at all levels of the page table;
pub const NVC36F_MEM_OP_C_TLB_INVALIDATE_PAGE_TABLE_LEVEL_PTE_ONLY: c_uint = 0x00000001;
pub const NVC36F_MEM_OP_C_TLB_INVALIDATE_PAGE_TABLE_LEVEL_UP_TO_PDE0: c_uint = 0x00000002;
pub const NVC36F_MEM_OP_C_TLB_INVALIDATE_PAGE_TABLE_LEVEL_UP_TO_PDE1: c_uint = 0x00000003;
pub const NVC36F_MEM_OP_C_TLB_INVALIDATE_PAGE_TABLE_LEVEL_UP_TO_PDE2: c_uint = 0x00000004;
pub const NVC36F_MEM_OP_C_TLB_INVALIDATE_PAGE_TABLE_LEVEL_UP_TO_PDE3: c_uint = 0x00000005;
pub const NVC36F_MEM_OP_C_TLB_INVALIDATE_PAGE_TABLE_LEVEL_UP_TO_PDE4: c_uint = 0x00000006;
pub const NVC36F_MEM_OP_C_TLB_INVALIDATE_PAGE_TABLE_LEVEL_UP_TO_PDE5: c_uint = 0x00000007;

pub const NVC36F_MEM_OP_C_TLB_INVALIDATE_PDB_APERTURE_VID_MEM: c_uint = 0x00000000;
pub const NVC36F_MEM_OP_C_TLB_INVALIDATE_PDB_APERTURE_SYS_MEM_COHERENT: c_uint = 0x00000002;
pub const NVC36F_MEM_OP_C_TLB_INVALIDATE_PDB_APERTURE_SYS_MEM_NONCOHERENT: c_uint = 0x00000003;

// MEM_OP_D MUST be preceded by MEM_OPs A-C.

pub const NVC36F_MEM_OP_D_OPERATION_MEMBAR: c_uint = 0x00000005;
pub const NVC36F_MEM_OP_D_OPERATION_MMU_TLB_INVALIDATE: c_uint = 0x00000009;
pub const NVC36F_MEM_OP_D_OPERATION_MMU_TLB_INVALIDATE_TARGETED: c_uint = 0x0000000a;
pub const NVC36F_MEM_OP_D_OPERATION_L2_PEERMEM_INVALIDATE: c_uint = 0x0000000d;
pub const NVC36F_MEM_OP_D_OPERATION_L2_SYSMEM_INVALIDATE: c_uint = 0x0000000e;
// CLEAN_LINES is an alias for Tegra/GPU IP usage
pub const NVC36F_MEM_OP_B_OPERATION_L2_INVALIDATE_CLEAN_LINES: c_uint = 0x0000000e;
pub const NVC36F_MEM_OP_D_OPERATION_L2_CLEAN_COMPTAGS: c_uint = 0x0000000f;
pub const NVC36F_MEM_OP_D_OPERATION_L2_FLUSH_DIRTY: c_uint = 0x00000010;
pub const NVC36F_MEM_OP_D_OPERATION_L2_WAIT_FOR_SYS_PENDING_READS: c_uint = 0x00000015;
pub const NVC36F_MEM_OP_D_OPERATION_ACCESS_COUNTER_CLR: c_uint = 0x00000016;

pub const NVC36F_MEM_OP_D_ACCESS_COUNTER_CLR_TYPE_MIMC: c_uint = 0x00000000;
pub const NVC36F_MEM_OP_D_ACCESS_COUNTER_CLR_TYPE_MOMC: c_uint = 0x00000001;
pub const NVC36F_MEM_OP_D_ACCESS_COUNTER_CLR_TYPE_ALL: c_uint = 0x00000002;
pub const NVC36F_MEM_OP_D_ACCESS_COUNTER_CLR_TYPE_TARGETED: c_uint = 0x00000003;

pub const NVC36F_MEM_OP_D_ACCESS_COUNTER_CLR_TARGETED_TYPE_MIMC: c_uint = 0x00000000;
pub const NVC36F_MEM_OP_D_ACCESS_COUNTER_CLR_TARGETED_TYPE_MOMC: c_uint = 0x00000001;

pub const NVC36F_SEM_EXECUTE_OPERATION_ACQUIRE: c_uint = 0x00000000;
pub const NVC36F_SEM_EXECUTE_OPERATION_RELEASE: c_uint = 0x00000001;
pub const NVC36F_SEM_EXECUTE_OPERATION_ACQ_STRICT_GEQ: c_uint = 0x00000002;
pub const NVC36F_SEM_EXECUTE_OPERATION_ACQ_CIRC_GEQ: c_uint = 0x00000003;
pub const NVC36F_SEM_EXECUTE_OPERATION_ACQ_AND: c_uint = 0x00000004;
pub const NVC36F_SEM_EXECUTE_OPERATION_ACQ_NOR: c_uint = 0x00000005;
pub const NVC36F_SEM_EXECUTE_OPERATION_REDUCTION: c_uint = 0x00000006;

pub const NVC36F_SEM_EXECUTE_ACQUIRE_SWITCH_TSG_DIS: c_uint = 0x00000000;
pub const NVC36F_SEM_EXECUTE_ACQUIRE_SWITCH_TSG_EN: c_uint = 0x00000001;

pub const NVC36F_SEM_EXECUTE_RELEASE_WFI_DIS: c_uint = 0x00000000;
pub const NVC36F_SEM_EXECUTE_RELEASE_WFI_EN: c_uint = 0x00000001;

pub const NVC36F_SEM_EXECUTE_PAYLOAD_SIZE_32BIT: c_uint = 0x00000000;
pub const NVC36F_SEM_EXECUTE_PAYLOAD_SIZE_64BIT: c_uint = 0x00000001;

pub const NVC36F_SEM_EXECUTE_RELEASE_TIMESTAMP_DIS: c_uint = 0x00000000;
pub const NVC36F_SEM_EXECUTE_RELEASE_TIMESTAMP_EN: c_uint = 0x00000001;

pub const NVC36F_SEM_EXECUTE_REDUCTION_IMIN: c_uint = 0x00000000;
pub const NVC36F_SEM_EXECUTE_REDUCTION_IMAX: c_uint = 0x00000001;
pub const NVC36F_SEM_EXECUTE_REDUCTION_IXOR: c_uint = 0x00000002;
pub const NVC36F_SEM_EXECUTE_REDUCTION_IAND: c_uint = 0x00000003;
pub const NVC36F_SEM_EXECUTE_REDUCTION_IOR: c_uint = 0x00000004;
pub const NVC36F_SEM_EXECUTE_REDUCTION_IADD: c_uint = 0x00000005;
pub const NVC36F_SEM_EXECUTE_REDUCTION_INC: c_uint = 0x00000006;
pub const NVC36F_SEM_EXECUTE_REDUCTION_DEC: c_uint = 0x00000007;

pub const NVC36F_SEM_EXECUTE_REDUCTION_FORMAT_SIGNED: c_uint = 0x00000000;
pub const NVC36F_SEM_EXECUTE_REDUCTION_FORMAT_UNSIGNED: c_uint = 0x00000001;
