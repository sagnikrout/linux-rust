//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/x86/xstate.h
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

pub const XSAVE_HDR_OFFSET: c_int = 512;
pub const XSAVE_HDR_SIZE: c_int = 64;
//
// List of XSAVE features Linux knows about. Copied from
// arch/x86/include/asm/fpu/types.h
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xfeature {
    XFEATURE_FP,
    XFEATURE_SSE,
    XFEATURE_YMM,
    XFEATURE_BNDREGS,
    XFEATURE_BNDCSR,
    XFEATURE_OPMASK,
    XFEATURE_ZMM_Hi256,
    XFEATURE_Hi16_ZMM,
    XFEATURE_PT_UNIMPLEMENTED_SO_FAR,
    XFEATURE_PKRU,
    XFEATURE_PASID,
    XFEATURE_CET_USER,
    XFEATURE_CET_KERNEL_UNUSED,
    XFEATURE_RSRVD_COMP_13,
    XFEATURE_RSRVD_COMP_14,
    XFEATURE_LBR,
    XFEATURE_RSRVD_COMP_16,
    XFEATURE_XTILECFG,
    XFEATURE_XTILEDATA,
    XFEATURE_APX,

    XFEATURE_MAX,
}

// Copied from arch/x86/kernel/fpu/xstate.c
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xsave_buffer {
    pub legacy: [c_char; XSAVE_HDR_OFFSET],
    pub header: [c_char; XSAVE_HDR_SIZE],
    pub extended: [c_char; 0],
}

pub const CPUID_LEAF_XSTATE: c_uint = 0xd;
pub const CPUID_SUBLEAF_XSTATE_USER: c_uint = 0x0;
//
// EBX enumerates the size (in bytes) required by the XSAVE
// instruction for an XSAVE area containing all the user state
// components corresponding to bits currently set in XCR0.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xstate_info {
    pub name: *const c_char,
    pub num: u32,
    pub mask: u32,
    pub xbuf_offset: u32,
    pub size: u32,
}

// XSAVE buffer should be 64B-aligned.
extern "C" {
    pub fn aligned_alloc(_arg: 64, _arg: xbuf_size) -> return;
}
// XSTATE_BV is at the beginning of the header:
// (uint64_t *)(&xbuf->header) = bv;
// See 'struct _fpx_sw_bytes' at sigcontext.h
pub const SW_BYTES_OFFSET: c_int = 464;
// N.B. The struct's field name varies so read from the offset.

//
// Ensure that 'data' is never 0.  This ensures that
// the registers are never in their initial configuration
// and thus never tracked as being in the init state.
//
// ptr = data;
// Testing kernel's context switching and ABI support for the xstate.
extern "C" {
    pub fn test_xstate(feature_num: u32);
}
