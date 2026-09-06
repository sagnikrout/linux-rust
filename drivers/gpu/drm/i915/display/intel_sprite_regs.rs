//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_sprite_regs.h
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
// Copyright © 2024 Intel Corporation

// g4x/ilk/snb video sprite
pub const _DVSACNTR: c_uint = 0x72180;
pub const _DVSBCNTR: c_uint = 0x73180;

pub const _DVSALINOFF: c_uint = 0x72184;
pub const _DVSBLINOFF: c_uint = 0x73184;

pub const _DVSASTRIDE: c_uint = 0x72188;
pub const _DVSBSTRIDE: c_uint = 0x73188;

pub const _DVSAPOS: c_uint = 0x7218c;
pub const _DVSBPOS: c_uint = 0x7318c;

pub const _DVSASIZE: c_uint = 0x72190;
pub const _DVSBSIZE: c_uint = 0x73190;

pub const _DVSAKEYVAL: c_uint = 0x72194;
pub const _DVSBKEYVAL: c_uint = 0x73194;

pub const _DVSAKEYMSK: c_uint = 0x72198;
pub const _DVSBKEYMSK: c_uint = 0x73198;

pub const _DVSASURF: c_uint = 0x7219c;
pub const _DVSBSURF: c_uint = 0x7319c;

pub const _DVSAKEYMAXVAL: c_uint = 0x721a0;
pub const _DVSBKEYMAXVAL: c_uint = 0x731a0;

pub const _DVSATILEOFF: c_uint = 0x721a4;
pub const _DVSBTILEOFF: c_uint = 0x731a4;

pub const _DVSASURFLIVE: c_uint = 0x721ac;
pub const _DVSBSURFLIVE: c_uint = 0x731ac;

pub const _DVSAGAMC_G4X: c_uint = 0x721e0 /* g4x */;
pub const _DVSBGAMC_G4X: c_uint = 0x731e0 /* g4x */;

pub const _DVSASCALE: c_uint = 0x72204;
pub const _DVSBSCALE: c_uint = 0x73204;

pub const _DVSAGAMC_ILK: c_uint = 0x72300 /* ilk/snb */;
pub const _DVSBGAMC_ILK: c_uint = 0x73300 /* ilk/snb */;

pub const _DVSAGAMCMAX_ILK: c_uint = 0x72340 /* ilk/snb */;
pub const _DVSBGAMCMAX_ILK: c_uint = 0x73340 /* ilk/snb */;

// ivb/hsw/bdw sprite
pub const _SPRA_CTL: c_uint = 0x70280;
pub const _SPRB_CTL: c_uint = 0x71280;

pub const _SPRA_LINOFF: c_uint = 0x70284 /* ivb */;
pub const _SPRB_LINOFF: c_uint = 0x71284 /* ivb */;

pub const _SPRA_STRIDE: c_uint = 0x70288;
pub const _SPRB_STRIDE: c_uint = 0x71288;

pub const _SPRA_POS: c_uint = 0x7028c;
pub const _SPRB_POS: c_uint = 0x7128c;

pub const _SPRA_SIZE: c_uint = 0x70290;
pub const _SPRB_SIZE: c_uint = 0x71290;

pub const _SPRA_KEYVAL: c_uint = 0x70294;
pub const _SPRB_KEYVAL: c_uint = 0x71294;

pub const _SPRA_KEYMSK: c_uint = 0x70298;
pub const _SPRB_KEYMSK: c_uint = 0x71298;

pub const _SPRA_SURF: c_uint = 0x7029c;
pub const _SPRB_SURF: c_uint = 0x7129c;

pub const _SPRA_KEYMAX: c_uint = 0x702a0;
pub const _SPRB_KEYMAX: c_uint = 0x712a0;

pub const _SPRA_TILEOFF: c_uint = 0x702a4 /* ivb */;
pub const _SPRB_TILEOFF: c_uint = 0x712a4 /* ivb */;

pub const _SPRA_OFFSET: c_uint = 0x702a4 /* hsw/bdw */;
pub const _SPRB_OFFSET: c_uint = 0x712a4 /* hsw/bdw */;

pub const _SPRA_SURFLIVE: c_uint = 0x702ac;
pub const _SPRB_SURFLIVE: c_uint = 0x712ac;

pub const _SPRA_SCALE: c_uint = 0x70304 /* ivb */;
pub const _SPRB_SCALE: c_uint = 0x71304 /* ivb */;

pub const _SPRA_GAMC: c_uint = 0x70400;
pub const _SPRB_GAMC: c_uint = 0x71400;

pub const _SPRA_GAMC16: c_uint = 0x70440;
pub const _SPRB_GAMC16: c_uint = 0x71440;

pub const _SPRA_GAMC17: c_uint = 0x7044c;
pub const _SPRB_GAMC17: c_uint = 0x7144c;

// vlv/chv sprite

//
// CHV pipe B sprite CSC
//
// |cr|   |c0 c1 c2|   |cr + cr_ioff|   |cr_ooff|
// |yg| = |c3 c4 c5| x |yg + yg_ioff| + |yg_ooff|
// |cb|   |c6 c7 c8|   |cb + cr_ioff|   |cb_ooff|
//

