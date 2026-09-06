//! Automatically rewritten from C Header to Rust Module
//! Source: include/crypto/sha3.h
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
// Common values for SHA-3 algorithms
//
// See also Documentation/crypto/sha3.rst
//

//
// SHAKE128 and SHAKE256 actually have variable output size, but this is used to
// calculate the block size (rate) analogously to the above.
//

pub const SHA3_STATE_SIZE: c_int = 200;
//
// State for the Keccak-f[1600] permutation: 25 64-bit words.
//
// We usually keep the state words as little-endian, to make absorbing and
// squeezing easier.  (It means that absorbing and squeezing can just treat the
// state as a byte array.)  The state words are converted to native-endian only
// temporarily by implementations of the permutation that need native-endian
// words.  Of course, that conversion is a no-op on little-endian machines.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sha3_state {
    pub 8]: __le64 words[SHA3_STATE_SIZE /,
    pub bytes: [u8; SHA3_STATE_SIZE],
    pub /: *mut *mut u64 native_words[SHA3_STATE_SIZE / 8]; / see comment above,
}

// Internal context, shared by the digests (SHA3-*) and the XOFs (SHAKE*)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __sha3_ctx {
    pub state: sha3_state,
    pub /: *mut *mut u8 digest_size; / Digests only: the digest size in bytes,
    pub /: *mut *mut u8 block_size; / Block size in bytes,
    pub /: *mut *mut u8 absorb_offset; / Index of next state byte to absorb into,
    pub /: *mut *mut u8 squeeze_offset; / XOFs only: index of next state byte to extract,
}

extern "C" {
    pub fn __sha3_update(ctx: *mut __sha3_ctx, in: *const u8, in_len: usize);
}
//
// struct sha3_ctx - Context for SHA3-224, SHA3-256, SHA3-384, or SHA3-512
// @ctx: private
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sha3_ctx {
    pub ctx: __sha3_ctx,
}

//
// sha3_zeroize_ctx() - Zeroize a SHA-3 context
// @ctx: The context to zeroize
//
// This is already called by sha3_final().  Call this explicitly when abandoning
// a context without calling sha3_final().
//
// struct shake_ctx - Context for SHAKE128 or SHAKE256
// @ctx: private
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shake_ctx {
    pub ctx: __sha3_ctx,
}

//
// shake_zeroize_ctx() - Zeroize a SHAKE context
// @ctx: The context to zeroize
//
// Call this after the last squeeze.
//
// sha3_224_init() - Initialize a context for SHA3-224
// @ctx: The context to initialize
//
// This begins a new SHA3-224 message digest computation.
//
// Context: Any context.
//
// ctx = (struct sha3_ctx){
//
// sha3_256_init() - Initialize a context for SHA3-256
// @ctx: The context to initialize
//
// This begins a new SHA3-256 message digest computation.
//
// Context: Any context.
//
// ctx = (struct sha3_ctx){
//
// sha3_384_init() - Initialize a context for SHA3-384
// @ctx: The context to initialize
//
// This begins a new SHA3-384 message digest computation.
//
// Context: Any context.
//
// ctx = (struct sha3_ctx){
//
// sha3_512_init() - Initialize a context for SHA3-512
// @ctx: The context to initialize
//
// This begins a new SHA3-512 message digest computation.
//
// Context: Any context.
//
// ctx = (struct sha3_ctx){
//
// sha3_update() - Update a SHA-3 digest context with input data
// @ctx: The context to update; must have been initialized
// @in: The input data
// @in_len: Length of the input data in bytes
//
// This can be called any number of times to add data to a SHA3-224, SHA3-256,
// SHA3-384, or SHA3-512 digest (depending on which init function was called).
//
// Context: Any context.
//
// sha3_final() - Finish computing a SHA-3 message digest
// @ctx: The context to finalize; must have been initialized
// @out: (output) The resulting SHA3-224, SHA3-256, SHA3-384, or SHA3-512
// message digest, matching the init function that was called.  Note that
// the size differs for each one; see SHA3_*_DIGEST_SIZE.
//
// After finishing, this zeroizes @ctx.  So the caller does not need to do it.
//
// Context: Any context.
//
extern "C" {
    pub fn sha3_final(ctx: *mut sha3_ctx, out: *mut u8);
}
//
// shake128_init() - Initialize a context for SHAKE128
// @ctx: The context to initialize
//
// This begins a new SHAKE128 extendable-output function (XOF) computation.
//
// Context: Any context.
//
// ctx = (struct shake_ctx){
//
// shake256_init() - Initialize a context for SHAKE256
// @ctx: The context to initialize
//
// This begins a new SHAKE256 extendable-output function (XOF) computation.
//
// Context: Any context.
//
// ctx = (struct shake_ctx){
//
// shake_update() - Update a SHAKE context with input data
// @ctx: The context to update; must have been initialized
// @in: The input data
// @in_len: Length of the input data in bytes
//
// This can be called any number of times to add more input data to SHAKE128 or
// SHAKE256.  This cannot be called after squeezing has begun.
//
// Context: Any context.
//
// shake_squeeze() - Generate output from SHAKE128 or SHAKE256
// @ctx: The context to squeeze; must have been initialized
// @out: Where to write the resulting output data
// @out_len: The amount of data to extract to @out in bytes
//
// This may be called multiple times.  A number of consecutive squeezes laid
// end-to-end will yield the same output as one big squeeze generating the same
// total amount of output.  More input cannot be provided after squeezing has
// begun.  After the last squeeze, call shake_zeroize_ctx().
//
// Context: Any context.
//
extern "C" {
    pub fn shake_squeeze(ctx: *mut shake_ctx, out: *mut u8, out_len: usize);
}
//
// sha3_224() - Compute SHA3-224 digest in one shot
// @in: The input data to be digested
// @in_len: Length of the input data in bytes
// @out: The buffer into which the digest will be stored
//
// Convenience function that computes a SHA3-224 digest.  Use this instead of
// the incremental API if you're able to provide all the input at once.
//
// Context: Any context.
//
extern "C" {
    pub fn sha3_224(in: *const u8, in_len: usize, out[SHA3_224_DIGEST_SIZE]: u8);
}
//
// sha3_256() - Compute SHA3-256 digest in one shot
// @in: The input data to be digested
// @in_len: Length of the input data in bytes
// @out: The buffer into which the digest will be stored
//
// Convenience function that computes a SHA3-256 digest.  Use this instead of
// the incremental API if you're able to provide all the input at once.
//
// Context: Any context.
//
extern "C" {
    pub fn sha3_256(in: *const u8, in_len: usize, out[SHA3_256_DIGEST_SIZE]: u8);
}
//
// sha3_384() - Compute SHA3-384 digest in one shot
// @in: The input data to be digested
// @in_len: Length of the input data in bytes
// @out: The buffer into which the digest will be stored
//
// Convenience function that computes a SHA3-384 digest.  Use this instead of
// the incremental API if you're able to provide all the input at once.
//
// Context: Any context.
//
extern "C" {
    pub fn sha3_384(in: *const u8, in_len: usize, out[SHA3_384_DIGEST_SIZE]: u8);
}
//
// sha3_512() - Compute SHA3-512 digest in one shot
// @in: The input data to be digested
// @in_len: Length of the input data in bytes
// @out: The buffer into which the digest will be stored
//
// Convenience function that computes a SHA3-512 digest.  Use this instead of
// the incremental API if you're able to provide all the input at once.
//
// Context: Any context.
//
extern "C" {
    pub fn sha3_512(in: *const u8, in_len: usize, out[SHA3_512_DIGEST_SIZE]: u8);
}
//
// shake128() - Compute SHAKE128 in one shot
// @in: The input data to be used
// @in_len: Length of the input data in bytes
// @out: The buffer into which the output will be stored
// @out_len: Length of the output to produce in bytes
//
// Convenience function that computes SHAKE128 in one shot.  Use this instead of
// the incremental API if you're able to provide all the input at once as well
// as receive all the output at once.  All output lengths are supported.
//
// Context: Any context.
//
extern "C" {
    pub fn shake128(in: *const u8, in_len: usize, out: *mut u8, out_len: usize);
}
//
// shake256() - Compute SHAKE256 in one shot
// @in: The input data to be used
// @in_len: Length of the input data in bytes
// @out: The buffer into which the output will be stored
// @out_len: Length of the output to produce in bytes
//
// Convenience function that computes SHAKE256 in one shot.  Use this instead of
// the incremental API if you're able to provide all the input at once as well
// as receive all the output at once.  All output lengths are supported.
//
// Context: Any context.
//
extern "C" {
    pub fn shake256(in: *const u8, in_len: usize, out: *mut u8, out_len: usize);
}
