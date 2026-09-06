//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/au88x0/au88x0_a3d.h
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
// au88x0_a3d.h
//
// Fri Jul 18 14:16:03 2003
// Copyright  2003  mjander
// mjander@users.sourceforge.net
//
// #include <openal.h>
pub const HRTF_SZ: c_uint = 0x38;
pub const DLINE_SZ: c_uint = 0x28;
pub const CTRLID_HRTF: c_int = 1;
pub const CTRLID_ITD: c_int = 2;
pub const CTRLID_ILD: c_int = 4;
pub const CTRLID_FILTER: c_int = 8;
pub const CTRLID_GAINS: c_int = 16;
// 3D parameter structs
// First Register bank
pub const A3D_A_HrtfCurrent: c_uint = 0x18000	/* 56 ULONG */;
pub const A3D_A_GainCurrent: c_uint = 0x180E0;
pub const A3D_A_GainTarget: c_uint = 0x180E4;
pub const A3D_A_A12Current: c_uint = 0x180E8	/* Atmospheric current. */;
pub const A3D_A_A21Target: c_uint = 0x180EC	/* Atmospheric target */;
pub const A3D_A_B01Current: c_uint = 0x180F0	/* Atmospheric current */;
pub const A3D_A_B10Target: c_uint = 0x180F4	/* Atmospheric target */;
pub const A3D_A_B2Current: c_uint = 0x180F8	/* Atmospheric current */;
pub const A3D_A_B2Target: c_uint = 0x180FC	/* Atmospheric target */;
pub const A3D_A_HrtfTarget: c_uint = 0x18100	/* 56 ULONG */;
pub const A3D_A_ITDCurrent: c_uint = 0x181E0;
pub const A3D_A_ITDTarget: c_uint = 0x181E4;
pub const A3D_A_HrtfDelayLine: c_uint = 0x181E8	/* 56 ULONG */;
pub const A3D_A_ITDDelayLine: c_uint = 0x182C8	/* 40/45 ULONG */;
pub const A3D_A_HrtfTrackTC: c_uint = 0x1837C	/* Time Constants */;
pub const A3D_A_GainTrackTC: c_uint = 0x18380;
pub const A3D_A_CoeffTrackTC: c_uint = 0x18384;
pub const A3D_A_ITDTrackTC: c_uint = 0x18388;
pub const A3D_A_x1: c_uint = 0x1838C;
pub const A3D_A_x2: c_uint = 0x18390;
pub const A3D_A_y1: c_uint = 0x18394;
pub const A3D_A_y2: c_uint = 0x18398;
pub const A3D_A_HrtfOutL: c_uint = 0x1839C;
pub const A3D_A_HrtfOutR: c_uint = 0x183A0;
pub const A3D_A_TAIL: c_uint = 0x183A4;
// Second register bank
pub const A3D_B_HrtfCurrent: c_uint = 0x19000	/* 56 ULONG */;
pub const A3D_B_GainCurrent: c_uint = 0x190E0;
pub const A3D_B_GainTarget: c_uint = 0x190E4;
pub const A3D_B_A12Current: c_uint = 0x190E8;
pub const A3D_B_A21Target: c_uint = 0x190EC;
pub const A3D_B_B01Current: c_uint = 0x190F0;
pub const A3D_B_B10Target: c_uint = 0x190F4;
pub const A3D_B_B2Current: c_uint = 0x190F8;
pub const A3D_B_B2Target: c_uint = 0x190FC;
pub const A3D_B_HrtfTarget: c_uint = 0x19100	/* 56 ULONG */;
pub const A3D_B_ITDCurrent: c_uint = 0x191E0;
pub const A3D_B_ITDTarget: c_uint = 0x191E4;
pub const A3D_B_HrtfDelayLine: c_uint = 0x191E8	/* 56 ULONG */;
pub const A3D_B_TAIL: c_uint = 0x192C8;
// There are 4 slices, 4 a3d each = 16 a3d sources.
pub const A3D_SLICE_BANK_A: c_uint = 0x18000	/* 4 sources */;
pub const A3D_SLICE_BANK_B: c_uint = 0x19000	/* 4 sources */;
pub const A3D_SLICE_VDBDest: c_uint = 0x19C00	/* 8 ULONG */;
pub const A3D_SLICE_VDBSource: c_uint = 0x19C20	/* 4 ULONG */;
pub const A3D_SLICE_ABReg: c_uint = 0x19C30;
pub const A3D_SLICE_CReg: c_uint = 0x19C34;
pub const A3D_SLICE_Control: c_uint = 0x19C38;
pub const A3D_SLICE_DebugReserved: c_uint = 0x19C3c	/* Dangerous! */;
pub const A3D_SLICE_Pointers: c_uint = 0x19C40;
pub const A3D_SLICE_TAIL: c_uint = 0x1A000;
// Slice size: 0x2000
// Source size: 0x3A4, 0x2C8
// Address generator macro.

// #define a3d_addr(slice,source,reg) (((reg)>=0x19000) ? a3d_addr2((slice),(source),(reg)) : a3d_addr1((slice),(source),(reg)))
