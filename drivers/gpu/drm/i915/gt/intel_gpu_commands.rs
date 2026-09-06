//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gt/intel_gpu_commands.h
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
// Copyright © 2003-2018 Intel Corporation
//

//
// Target address alignments required for GPU access e.g.
// MI_STORE_DWORD_IMM.
//
pub const alignof_dword: c_int = 4;
pub const alignof_qword: c_int = 8;
//
// Instruction field definitions used by the command parser
//
pub const INSTR_CLIENT_SHIFT: c_int = 29;
pub const INSTR_MI_CLIENT: c_uint = 0x0;
pub const INSTR_BC_CLIENT: c_uint = 0x2;
pub const INSTR_GSC_CLIENT: c_uint = 0x2 /* MTL+ */;
pub const INSTR_RC_CLIENT: c_uint = 0x3;
pub const INSTR_SUBCLIENT_SHIFT: c_int = 27;
pub const INSTR_SUBCLIENT_MASK: c_uint = 0x18000000;
pub const INSTR_MEDIA_SUBCLIENT: c_uint = 0x2;
pub const INSTR_26_TO_24_MASK: c_uint = 0x7000000;
pub const INSTR_26_TO_24_SHIFT: c_int = 24;

//
// Memory interface instructions used by the kernel
//

// Many MI commands use bit 22 of the header dword for GGTT vs PPGTT

// IVB has funny definitions for which plane to flip.

// SKL ones

pub const MI_SEMAPHORE_TOKEN_SHIFT: c_int = 5;

//
// Official intel docs are somewhat sloppy concerning MI_LOAD_REGISTER_IMM:
// - Always issue a MI_NOOP _before_ the MI_LOAD_REGISTER_IMM - otherwise hw
// simply ignores the register load under certain conditions.
// - One can actually load arbitrary many arbitrary registers: Simply issue x
// address/value pairs. Don't overdue it, though, x <= 2^4 must hold!
//

// Gen11+. addr = base + (ctx_restore ? offset & GENMASK(12,2) : offset)

// for snb/ivb/vlv this also means "batch in ppgtt" when ppgtt is enabled.

//
// 3D instructions used by the kernel
//

pub const XY_CTRL_SURF_INSTR_SIZE: c_int = 5;
pub const MI_FLUSH_DW_SIZE: c_int = 3;

pub const SRC_ACCESS_TYPE_SHIFT: c_int = 21;
pub const DST_ACCESS_TYPE_SHIFT: c_int = 20;
pub const CCS_SIZE_MASK: c_uint = 0x3FF;
pub const CCS_SIZE_SHIFT: c_int = 8;

pub const NUM_CCS_BYTES_PER_BLOCK: c_int = 256;
pub const NUM_BYTES_PER_CCS_BYTE: c_int = 256;
pub const NUM_CCS_BLKS_PER_XFER: c_int = 1024;
pub const INDIRECT_ACCESS: c_int = 0;
pub const DIRECT_ACCESS: c_int = 1;

pub const XY_FAST_COLOR_BLT_DW: c_int = 16;

pub const XY_FAST_COLOR_BLT_MEM_TYPE_SHIFT: c_int = 31;

pub const LINEAR: c_int = 0;
pub const TILE_X: c_uint = 0x1;
pub const XMAJOR: c_uint = 0x1;
pub const YMAJOR: c_uint = 0x2;
pub const TILE_64: c_uint = 0x3;

// Note:  MOCS value = (index << 1)

//
// 3D-related flags that can't be set on _engines_ that lack access to the 3D
// pipeline (i.e., CCS engines).
//

// 3D-related flags that can't be set on _platforms_ that lack a 3D pipeline

// Opcodes for MI_MATH_INSTR

// Registers used as operands in MI_MATH_INSTR

pub const MI_MATH_REG_SRCA: c_uint = 0x20;
pub const MI_MATH_REG_SRCB: c_uint = 0x21;
pub const MI_MATH_REG_ACCU: c_uint = 0x31;
pub const MI_MATH_REG_ZF: c_uint = 0x32;
pub const MI_MATH_REG_CF: c_uint = 0x33;
//
// Media instructions used by the kernel
//

//
// Commands used only by the command parser
//

//
// Used to convert any address to canonical form.
// Starting from gen8, some commands (e.g. STATE_BASE_ADDRESS,
// MI_LOAD_REGISTER_MEM and others, see Broadwell PRM Vol2a) require the
// addresses to be in a canonical form:
// "GraphicsAddress[63:48] are ignored by the HW and assumed to be in correct
// canonical form [63:48] == [47]."
//
pub const GEN8_HIGH_ADDRESS_BIT: c_int = 47;
extern "C" {
    pub fn sign_extend64(_arg: address, _arg: GEN8_HIGH_ADDRESS_BIT) -> return;
}
// cs++ = MI_BATCH_BUFFER_START | flags;
// cs++ = addr;
