//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/imagination/pvr_rogue_fwif_stream.h
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
//
// DOC: Streams
//
// Commands are submitted to the kernel driver in the form of streams.
//
// A command stream has the following layout :
// - A 64-bit header containing:
// * A u32 containing the length of the main stream inclusive of the length of the header.
// * A u32 for padding.
// - The main stream data.
// - The extension stream (optional), which is composed of:
// * One or more headers.
// * The extension stream data, corresponding to the extension headers.
//
// The main stream provides the base command data. This has a fixed layout based on the features
// supported by a given GPU.
//
// The extension stream provides the command parameters that are required for BRNs & ERNs for the
// current GPU. This stream is comprised of one or more headers, followed by data for each given
// BRN/ERN.
//
// Each header is a u32 containing a bitmask of quirks & enhancements in the extension stream, a
// "type" field determining the set of quirks & enhancements the bitmask represents, and a
// continuation bit determining whether any more headers are present. The headers are then followed
// by command data; this is specific to each quirk/enhancement. All unused / reserved bits in the
// header must be set to 0.
//
// All parameters and headers in the main and extension streams must be naturally aligned.
//
// If a parameter appears in both the main and extension streams, then the extension parameter is
// used.
//
// Stream extension header definition
//

//
// Stream extension header - Geometry 0
//

//
// Stream extension header - Fragment 0
//

//
// Stream extension header - Compute 0
//

