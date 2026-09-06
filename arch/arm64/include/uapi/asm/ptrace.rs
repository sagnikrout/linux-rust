//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/uapi/asm/ptrace.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// Based on arch/arm/include/asm/ptrace.h
//
// Copyright (C) 1996-2003 Russell King
// Copyright (C) 2012 ARM Ltd.
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License version 2 as
// published by the Free Software Foundation.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
//

//
// PSR bits
//
pub const PSR_MODE_EL0t: c_uint = 0x00000000;
pub const PSR_MODE_EL1t: c_uint = 0x00000004;
pub const PSR_MODE_EL1h: c_uint = 0x00000005;
pub const PSR_MODE_EL2t: c_uint = 0x00000008;
pub const PSR_MODE_EL2h: c_uint = 0x00000009;
pub const PSR_MODE_EL3t: c_uint = 0x0000000c;
pub const PSR_MODE_EL3h: c_uint = 0x0000000d;
pub const PSR_MODE_MASK: c_uint = 0x0000000f;
// AArch32 CPSR bits
pub const PSR_MODE32_BIT: c_uint = 0x00000010;
// AArch64 SPSR bits
pub const PSR_F_BIT: c_uint = 0x00000040;
pub const PSR_I_BIT: c_uint = 0x00000080;
pub const PSR_A_BIT: c_uint = 0x00000100;
pub const PSR_D_BIT: c_uint = 0x00000200;
pub const PSR_BTYPE_MASK: c_uint = 0x00000c00;
pub const PSR_SSBS_BIT: c_uint = 0x00001000;
pub const PSR_PAN_BIT: c_uint = 0x00400000;
pub const PSR_UAO_BIT: c_uint = 0x00800000;
pub const PSR_DIT_BIT: c_uint = 0x01000000;
pub const PSR_TCO_BIT: c_uint = 0x02000000;
pub const PSR_V_BIT: c_uint = 0x10000000;
pub const PSR_C_BIT: c_uint = 0x20000000;
pub const PSR_Z_BIT: c_uint = 0x40000000;
pub const PSR_N_BIT: c_uint = 0x80000000;
pub const PSR_BTYPE_SHIFT: c_int = 10;
//
// Groups of PSR bits
//
pub const PSR_f: c_uint = 0xff000000	/* Flags		*/;
pub const PSR_s: c_uint = 0x00ff0000	/* Status		*/;
pub const PSR_x: c_uint = 0x0000ff00	/* Extension		*/;
pub const PSR_c: c_uint = 0x000000ff	/* Control		*/;
// Convenience names for the values of PSTATE.BTYPE

// syscall emulation path in ptrace
pub const PTRACE_SYSEMU: c_int = 31;
pub const PTRACE_SYSEMU_SINGLESTEP: c_int = 32;
// MTE allocation tag access
pub const PTRACE_PEEKMTETAGS: c_int = 33;
pub const PTRACE_POKEMTETAGS: c_int = 34;
//
// User structures for general purpose, floating point and debug registers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_pt_regs {
    pub regs: [__u64; 31],
    pub sp: __u64,
    pub pc: __u64,
    pub pstate: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_fpsimd_state {
    pub vregs: [__u128; 32],
    pub fpsr: __u32,
    pub fpcr: __u32,
    pub __reserved: [__u32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_hwdebug_state {
    pub dbg_info: __u32,
    pub pad: __u32,
    pub addr: __u64,
    pub ctrl: __u32,
    pub pad: __u32,
    pub dbg_regs: [}; 16],
}

// SVE/FP/SIMD state (NT_ARM_SVE & NT_ARM_SSVE)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_sve_header {
    pub /: *mut *mut __u32 size; / total meaningful regset content in bytes,
    pub /: *mut *mut __u32 max_size; / maxmium possible size for this thread,
    pub /: *mut *mut __u16 vl; / current vector length,
    pub /: *mut *mut __u16 max_vl; / maximum possible vector length,
    pub flags: __u16,
    pub __reserved: __u16,
}

// Definitions for user_sve_header.flags:

pub const SVE_PT_REGS_FPSIMD: c_int = 0;

//
// Common SVE_PT_* flags:
// These must be kept in sync with prctl interface in <linux/prctl.h>
//

//
// The remainder of the SVE state follows struct user_sve_header.  The
// total size of the SVE state (including header) depends on the
// metadata in the header:  SVE_PT_SIZE(vq, flags) gives the total size
// of the state in bytes, including the header.
//
// Refer to <asm/sigcontext.h> for details of how to pass the correct
// "vq" argument to these macros.
//
// Offset from the start of struct user_sve_header to the register data

//
// The register data content and layout depends on the value of the
// flags field.
//
// (flags & SVE_PT_REGS_MASK) == SVE_PT_REGS_FPSIMD case:
//
// The payload starts at offset SVE_PT_FPSIMD_OFFSET, and is of type
// struct user_fpsimd_state.  Additional data might be appended in the
// future: use SVE_PT_FPSIMD_SIZE(vq, flags) to compute the total size.
// SVE_PT_FPSIMD_SIZE(vq, flags) will never be less than
// sizeof(struct user_fpsimd_state).
//

//
// (flags & SVE_PT_REGS_MASK) == SVE_PT_REGS_SVE case:
//
// The payload starts at offset SVE_PT_SVE_OFFSET, and is of size
// SVE_PT_SVE_SIZE(vq, flags).
//
// Additional macros describe the contents and layout of the payload.
// For each, SVE_PT_SVE_x_OFFSET(args) is the start offset relative to
// the start of struct user_sve_header, and SVE_PT_SVE_x_SIZE(args) is
// the size in bytes:
//
// x	type				description
// -	----				-----------
// ZREGS		\
// ZREG		|
// PREGS		| refer to <asm/sigcontext.h>
// PREG		|
// FFR
//
// FPSR	uint32_t			FPSR
// FPCR	uint32_t			FPCR
//
// Additional data might be appended in the future.
//
// The Z-, P- and FFR registers are represented in memory in an endianness-
// invariant layout which differs from the layout used for the FPSIMD
// V-registers on big-endian systems: see sigcontext.h for more explanation.
//

// For streaming mode SVE (SSVE) FFR must be read and written as zero

//
// Any future extension appended after FPCR must be aligned to the next
// 128-bit boundary.
//

// pointer authentication masks (NT_ARM_PAC_MASK)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_pac_mask {
    pub data_mask: __u64,
    pub insn_mask: __u64,
}

// pointer authentication keys (NT_ARM_PACA_KEYS, NT_ARM_PACG_KEYS)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_pac_address_keys {
    pub apiakey: __u128,
    pub apibkey: __u128,
    pub apdakey: __u128,
    pub apdbkey: __u128,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_pac_generic_keys {
    pub apgakey: __u128,
}

// ZA state (NT_ARM_ZA)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_za_header {
    pub /: *mut *mut __u32 size; / total meaningful regset content in bytes,
    pub /: *mut *mut __u32 max_size; / maxmium possible size for this thread,
    pub /: *mut *mut __u16 vl; / current vector length,
    pub /: *mut *mut __u16 max_vl; / maximum possible vector length,
    pub flags: __u16,
    pub __reserved: __u16,
}

//
// Common ZA_PT_* flags:
// These must be kept in sync with prctl interface in <linux/prctl.h>
//

//
// The remainder of the ZA state follows struct user_za_header.  The
// total size of the ZA state (including header) depends on the
// metadata in the header:  ZA_PT_SIZE(vq, flags) gives the total size
// of the state in bytes, including the header.
//
// Refer to <asm/sigcontext.h> for details of how to pass the correct
// "vq" argument to these macros.
//
// Offset from the start of struct user_za_header to the register data

//
// The payload starts at offset ZA_PT_ZA_OFFSET, and is of size
// ZA_PT_ZA_SIZE(vq, flags).
//
// The ZA array is stored as a sequence of horizontal vectors ZAV of SVL/8
// bytes each, starting from vector 0.
//
// Additional data might be appended in the future.
//
// The ZA matrix is represented in memory in an endianness-invariant layout
// which differs from the layout used for the FPSIMD V-registers on big-endian
// systems: see sigcontext.h for more explanation.
//

// GCS state (NT_ARM_GCS)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_gcs {
    pub features_enabled: __u64,
    pub features_locked: __u64,
    pub gcspr_el0: __u64,
}

