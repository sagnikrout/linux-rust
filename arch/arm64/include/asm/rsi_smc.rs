//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/rsi_smc.h
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
// Copyright (C) 2023 ARM Ltd.
//

//
// This file describes the Realm Services Interface (RSI) Application Binary
// Interface (ABI) for SMC calls made from within the Realm to the RMM and
// serviced by the RMM.
//
// The major version number of the RSI implementation.  This is increased when
// the binary format or semantics of the SMC calls change.
//

//
// The minor version number of the RSI implementation.  This is increased when
// a bug is fixed, or a feature is added without breaking binary compatibility.
//

//
// Returns RSI version.
//
// arg1 == Requested interface revision
// ret0 == Status / error
// ret1 == Lower implemented interface revision
// ret2 == Higher implemented interface revision
//

//
// Read feature register.
//
// arg1 == Feature register index
// ret0 == Status / error
// ret1 == Feature register value
//

//
// Read measurement for the current Realm.
//
// arg1 == Index, which measurements slot to read
// ret0 == Status / error
// ret1 == Measurement value, bytes:  0 -  7
// ret2 == Measurement value, bytes:  8 - 15
// ret3 == Measurement value, bytes: 16 - 23
// ret4 == Measurement value, bytes: 24 - 31
// ret5 == Measurement value, bytes: 32 - 39
// ret6 == Measurement value, bytes: 40 - 47
// ret7 == Measurement value, bytes: 48 - 55
// ret8 == Measurement value, bytes: 56 - 63
//

//
// Extend Realm Extensible Measurement (REM) value.
//
// arg1  == Index, which measurements slot to extend
// arg2  == Size of realm measurement in bytes, max 64 bytes
// arg3  == Measurement value, bytes:  0 -  7
// arg4  == Measurement value, bytes:  8 - 15
// arg5  == Measurement value, bytes: 16 - 23
// arg6  == Measurement value, bytes: 24 - 31
// arg7  == Measurement value, bytes: 32 - 39
// arg8  == Measurement value, bytes: 40 - 47
// arg9  == Measurement value, bytes: 48 - 55
// arg10 == Measurement value, bytes: 56 - 63
// ret0  == Status / error
//

//
// Initialize the operation to retrieve an attestation token.
//
// arg1 == Challenge value, bytes:  0 -  7
// arg2 == Challenge value, bytes:  8 - 15
// arg3 == Challenge value, bytes: 16 - 23
// arg4 == Challenge value, bytes: 24 - 31
// arg5 == Challenge value, bytes: 32 - 39
// arg6 == Challenge value, bytes: 40 - 47
// arg7 == Challenge value, bytes: 48 - 55
// arg8 == Challenge value, bytes: 56 - 63
// ret0 == Status / error
// ret1 == Upper bound of token size in bytes
//

//
// Continue the operation to retrieve an attestation token.
//
// arg1 == The IPA of token buffer
// arg2 == Offset within the granule of the token buffer
// arg3 == Size of the granule buffer
// ret0 == Status / error
// ret1 == Length of token bytes copied to the granule buffer
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct realm_config {
    pub /: *mut *mut unsigned long ipa_bits; / Width of IPA in bits,
    pub /: *mut *mut unsigned long hash_algo; / Hash algorithm,
}

//
// The RMM requires the configuration structure to be aligned to a 4k
// boundary, ensure this happens by aligning this structure.
//

//
// Read configuration for the current Realm.
//
// arg1 == struct realm_config addr
// ret0 == Status / error
//

//
// Request RIPAS of a target IPA range to be changed to a specified value.
//
// arg1 == Base IPA address of target region
// arg2 == Top of the region
// arg3 == RIPAS value
// arg4 == flags
// ret0 == Status / error
// ret1 == Top of modified IPA range
// ret2 == Whether the Host accepted or rejected the request
//

//
// Get RIPAS of a target IPA range.
//
// arg1 == Base IPA of target region
// arg2 == End of target IPA region
// ret0 == Status / error
// ret1 == Top of IPA region which has the reported RIPAS value
// ret2 == RIPAS value
//

//
// Make a Host call.
//
// arg1 == IPA of host call structure
// ret0 == Status / error
//

