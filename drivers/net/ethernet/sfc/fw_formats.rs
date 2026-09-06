//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/sfc/fw_formats.h
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
// Driver for AMD network controllers and boards
// Copyright (C) 2025, Advanced Micro Devices, Inc.
//
// This program is free software; you can redistribute it and/or modify it
// under the terms of the GNU General Public License version 2 as published
// by the Free Software Foundation, incorporated herein by reference.
//
// Header layouts of firmware update images recognised by Efx NICs.
// The sources-of-truth for these layouts are AMD internal documents
// and sfregistry headers, neither of which are available externally
// nor usable directly by the driver.
//
// While each format includes a 'magic number', these are at different
// offsets in the various formats, and a legal header for one format
// could have the right value in whichever field occupies that offset
// to match another format's magic.
// Besides, some packaging formats (such as CMS/PKCS#7 signed images)
// prepend a header for which finding the size is a non-trivial task;
// rather than trying to parse those headers, we search byte-by-byte
// through the provided firmware image looking for a valid header.
// Thus, format recognition has to include validation of the checksum
// field, even though the firmware will validate that itself before
// applying the image.
//
// EF10 (Medford2, X2) "reflash" header format.  Defined in SF-121352-AN
pub const EFX_REFLASH_HEADER_MAGIC_OFST: c_int = 0;
pub const EFX_REFLASH_HEADER_MAGIC_LEN: c_int = 4;
pub const EFX_REFLASH_HEADER_MAGIC_VALUE: c_uint = 0x106F1A5;
pub const EFX_REFLASH_HEADER_VERSION_OFST: c_int = 4;
pub const EFX_REFLASH_HEADER_VERSION_LEN: c_int = 4;
pub const EFX_REFLASH_HEADER_VERSION_VALUE: c_int = 4;
pub const EFX_REFLASH_HEADER_FIRMWARE_TYPE_OFST: c_int = 8;
pub const EFX_REFLASH_HEADER_FIRMWARE_TYPE_LEN: c_int = 4;
pub const EFX_REFLASH_FIRMWARE_TYPE_BOOTROM: c_uint = 0x2;
pub const EFX_REFLASH_FIRMWARE_TYPE_BUNDLE: c_uint = 0xd;
pub const EFX_REFLASH_HEADER_FIRMWARE_SUBTYPE_OFST: c_int = 12;
pub const EFX_REFLASH_HEADER_FIRMWARE_SUBTYPE_LEN: c_int = 4;
pub const EFX_REFLASH_HEADER_PAYLOAD_SIZE_OFST: c_int = 16;
pub const EFX_REFLASH_HEADER_PAYLOAD_SIZE_LEN: c_int = 4;
pub const EFX_REFLASH_HEADER_LENGTH_OFST: c_int = 20;
pub const EFX_REFLASH_HEADER_LENGTH_LEN: c_int = 4;
// Reflash trailer
pub const EFX_REFLASH_TRAILER_CRC_OFST: c_int = 0;
pub const EFX_REFLASH_TRAILER_CRC_LEN: c_int = 4;

// EF100 "SmartNIC image" header format.
// Defined in sfregistry "src/layout/snic_image_hdr.h".
//
pub const EFX_SNICIMAGE_HEADER_MAGIC_OFST: c_int = 16;
pub const EFX_SNICIMAGE_HEADER_MAGIC_LEN: c_int = 4;
pub const EFX_SNICIMAGE_HEADER_MAGIC_VALUE: c_uint = 0x541C057A;
pub const EFX_SNICIMAGE_HEADER_VERSION_OFST: c_int = 20;
pub const EFX_SNICIMAGE_HEADER_VERSION_LEN: c_int = 4;
pub const EFX_SNICIMAGE_HEADER_VERSION_VALUE: c_int = 1;
pub const EFX_SNICIMAGE_HEADER_LENGTH_OFST: c_int = 24;
pub const EFX_SNICIMAGE_HEADER_LENGTH_LEN: c_int = 4;
pub const EFX_SNICIMAGE_HEADER_PARTITION_TYPE_OFST: c_int = 36;
pub const EFX_SNICIMAGE_HEADER_PARTITION_TYPE_LEN: c_int = 4;
pub const EFX_SNICIMAGE_HEADER_PARTITION_SUBTYPE_OFST: c_int = 40;
pub const EFX_SNICIMAGE_HEADER_PARTITION_SUBTYPE_LEN: c_int = 4;
pub const EFX_SNICIMAGE_HEADER_PAYLOAD_SIZE_OFST: c_int = 60;
pub const EFX_SNICIMAGE_HEADER_PAYLOAD_SIZE_LEN: c_int = 4;
pub const EFX_SNICIMAGE_HEADER_CRC_OFST: c_int = 64;
pub const EFX_SNICIMAGE_HEADER_CRC_LEN: c_int = 4;
pub const EFX_SNICIMAGE_HEADER_MINLEN: c_int = 256;
// EF100 "SmartNIC bundle" header format.  Defined in SF-122606-TC
pub const EFX_SNICBUNDLE_HEADER_MAGIC_OFST: c_int = 0;
pub const EFX_SNICBUNDLE_HEADER_MAGIC_LEN: c_int = 4;
pub const EFX_SNICBUNDLE_HEADER_MAGIC_VALUE: c_uint = 0xB1001001;
pub const EFX_SNICBUNDLE_HEADER_VERSION_OFST: c_int = 4;
pub const EFX_SNICBUNDLE_HEADER_VERSION_LEN: c_int = 4;
pub const EFX_SNICBUNDLE_HEADER_VERSION_VALUE: c_int = 1;
pub const EFX_SNICBUNDLE_HEADER_BUNDLE_TYPE_OFST: c_int = 8;
pub const EFX_SNICBUNDLE_HEADER_BUNDLE_TYPE_LEN: c_int = 4;
pub const EFX_SNICBUNDLE_HEADER_BUNDLE_SUBTYPE_OFST: c_int = 12;
pub const EFX_SNICBUNDLE_HEADER_BUNDLE_SUBTYPE_LEN: c_int = 4;
pub const EFX_SNICBUNDLE_HEADER_LENGTH_OFST: c_int = 20;
pub const EFX_SNICBUNDLE_HEADER_LENGTH_LEN: c_int = 4;
pub const EFX_SNICBUNDLE_HEADER_CRC_OFST: c_int = 224;
pub const EFX_SNICBUNDLE_HEADER_CRC_LEN: c_int = 4;

