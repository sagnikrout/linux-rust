//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/rsi_cmds.h
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

pub const RSI_GRANULE_SHIFT: c_int = 12;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ripas {
    RSI_RIPAS_EMPTY = 0,
    RSI_RIPAS_RAM = 1,
    RSI_RIPAS_DESTROYED = 2,
    RSI_RIPAS_DEV = 3,
}

// out_lower = res.a1;
// out_higher = res.a2;
// top = res.a1;
// state = res.a2;
// top = res.a1;
pub const RSI_ATTEST_CHALLENGE_MIN_SIZE: c_int = 32;
pub const RSI_ATTEST_CHALLENGE_MAX_SIZE: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsi_attestation_token_init_args {
    pub fid: c_ulong,
    pub challenge: [u8; RSI_ATTEST_CHALLENGE_MAX_SIZE],
}

//
// rsi_attestation_token_init - Initialise the operation to retrieve an
// attestation token.
//
// @challenge:	The challenge data to be used in the attestation token
// generation.
// @size:	Size of the challenge data in bytes.
//
// Initialises the attestation token generation and returns an upper bound
// on the attestation token size that can be used to allocate an adequate
// buffer. The caller is expected to subsequently call
// rsi_attestation_token_continue() to retrieve the attestation token data on
// the same CPU.
//
// Returns:
// On success, returns the upper limit of the attestation report size.
// Otherwise, -EINVAL
//
// rsi_attestation_token_continue - Continue the operation to retrieve an
// attestation token.
//
// @granule: {I}PA of the Granule to which the token will be written.
// @offset:  Offset within Granule to start of buffer in bytes.
// @size:    The size of the buffer.
// @len:     The number of bytes written to the buffer.
//
// Retrieves up to a RSI_GRANULE_SIZE worth of token data per call. The caller
// is expected to call rsi_attestation_token_init() before calling this
// function to retrieve the attestation token.
//
// Return:
// * %RSI_SUCCESS     - Attestation token retrieved successfully.
// * %RSI_INCOMPLETE  - Token generation is not complete.
// * %RSI_ERROR_INPUT - A parameter was not valid.
// * %RSI_ERROR_STATE - Attestation not in progress.
//
// len = res.a1;
