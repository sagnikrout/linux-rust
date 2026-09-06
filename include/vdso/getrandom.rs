//! Automatically rewritten from C Header to Rust Module
//! Source: include/vdso/getrandom.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright (C) 2022-2024 Jason A. Donenfeld <Jason@zx2c4.com>. All Rights Reserved.
//

pub const CHACHA_KEY_SIZE: c_int = 32;
pub const CHACHA_BLOCK_SIZE: c_int = 64;
//
// struct vgetrandom_state - State used by vDSO getrandom().
//
// @batch:	One and a half ChaCha20 blocks of buffered RNG output.
//
// @key:	Key to be used for generating next batch.
//
// @batch_key:	Union of the prior two members, which is exactly two full
// ChaCha20 blocks in size, so that @batch and @key can be filled
// together.
//
// @generation:	Snapshot of @rng_info->generation in the vDSO data page at
// the time @key was generated.
//
// @pos:	Offset into @batch of the next available random byte.
//
// @in_use:	Reentrancy guard for reusing a state within the same thread
// due to signal handlers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vgetrandom_state {
    pub 2]: *mut *mut u8 batch[CHACHA_BLOCK_SIZE  3 /,
    pub sizeof(u32)]: u32 key[CHACHA_KEY_SIZE /,
}

//
// __arch_chacha20_blocks_nostack - Generate ChaCha20 stream without using the stack.
// @dst_bytes:	Destination buffer to hold @nblocks * 64 bytes of output.
// @key:	32-byte input key.
// @counter:	8-byte counter, read on input and updated on return.
// @nblocks:	Number of blocks to generate.
//
// Generates a given positive number of blocks of ChaCha20 output with nonce=0, and does not write
// to any stack or memory outside of the parameters passed to it, in order to mitigate stack data
// leaking into forked child processes.
//
extern "C" {
    pub fn __arch_chacha20_blocks_nostack(dst_bytes: *mut u8, key: *const u32, counter: *mut u32, nblocks: usize);
}
//
// __vdso_getrandom - Architecture-specific vDSO implementation of getrandom() syscall.
// @buffer:		Passed to __cvdso_getrandom().
// @len:		Passed to __cvdso_getrandom().
// @flags:		Passed to __cvdso_getrandom().
// @opaque_state:	Passed to __cvdso_getrandom().
// @opaque_len:		Passed to __cvdso_getrandom();
//
// This function is implemented by making a single call to to __cvdso_getrandom(), whose
// documentation may be consulted for more information.
//
// Returns:	The return value of __cvdso_getrandom().
//
extern "C" {
    pub fn __vdso_getrandom(buffer: *mut c_void, len: usize, flags: c_uint, opaque_state: *mut c_void, opaque_len: usize) -> isize;
}
