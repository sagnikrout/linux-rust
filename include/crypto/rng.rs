//! Automatically rewritten from C Header to Rust Module
//! Source: include/crypto/rng.h
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
// RNG: Random Number Generator  algorithms under the crypto API
//
// Copyright (c) 2008 Neil Horman <nhorman@tuxdriver.com>
// Copyright (c) 2015 Herbert Xu <herbert@gondor.apana.org.au>
//

//
// struct rng_alg - random number generator definition
//
// @generate:	The function defined by this variable obtains a
// random number. The random number generator transform
// must generate the random number out of the context
// provided with this call, plus any additional data
// if provided to the call.
// @seed:	Seed or reseed the random number generator.  With the
// invocation of this function call, the random number
// generator shall become ready for generation.  If the
// random number generator requires a seed for setting
// up a new state, the seed must be provided by the
// consumer while invoking this function. The required
// size of the seed is defined with @seedsize .
// @set_ent:	Set entropy that would otherwise be obtained from
// entropy source.  Internal use only.
// @seedsize:	The seed size required for a random number generator
// initialization defined with this variable. Some
// random number generators does not require a seed
// as the seeding is implemented internally without
// the need of support by the consumer. In this case,
// the seed size is set to zero.
// @base:	Common crypto API algorithm data structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rng_alg {
    pub dlen): *mut *mut u8 dst, unsigned int,
    pub slen): *const *const *const *const int (seed)(struct crypto_rng tfm, u8 seed, unsigned int,
    pub len): c_uint,
    pub seedsize: c_uint,
    pub base: crypto_alg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_rng {
    pub base: crypto_tfm,
}

extern "C" {
    pub fn __crypto_stdrng_get_bytes(buf: *mut c_void, len: c_uint) -> c_int;
}
//
// crypto_stdrng_get_bytes() - get cryptographically secure random bytes
// @buf: output buffer holding the random numbers
// @len: length of the output buffer
//
// This function fills the caller-allocated buffer with random numbers using the
// normal Linux RNG if fips_enabled=0, or the highest-priority "stdrng"
// algorithm in the crypto_rng subsystem if fips_enabled=1.
//
// Context: May sleep
// Return: 0 function was successful; < 0 if an error occurred
//
extern "C" {
    pub fn __crypto_stdrng_get_bytes(_arg: buf, _arg: len) -> return;
}
extern "C" {
    pub fn get_random_bytes_wait(_arg: buf, _arg: len) -> return;
}
//
// DOC: Random number generator API
//
// The random number generator API is used with the ciphers of type
// CRYPTO_ALG_TYPE_RNG (listed as type "rng" in /proc/crypto)
//
// crypto_alloc_rng() -- allocate RNG handle
// @alg_name: is the cra_name / name or cra_driver_name / driver name of the
// message digest cipher
// @type: specifies the type of the cipher
// @mask: specifies the mask for the cipher
//
// Allocate a cipher handle for a random number generator. The returned struct
// crypto_rng is the cipher handle that is required for any subsequent
// API invocation for that random number generator.
//
// For all random number generators, this call creates a new private copy of
// the random number generator that does not share a state with other
// instances. The only exception is the "krng" random number generator which
// is a kernel crypto API use case for the get_random_bytes() function of the
// /dev/random driver.
//
// Return: allocated cipher handle in case of success; IS_ERR() is true in case
// of an error, PTR_ERR() returns the error code.
//
extern "C" {
    pub fn container_of(_arg: alg, rng_alg: struct, _arg: base) -> return;
}
//
// crypto_rng_alg() - obtain 'struct rng_alg' pointer from RNG handle
// @tfm: RNG handle
//
// Return: Pointer to 'struct rng_alg', derived from @tfm RNG handle
//
extern "C" {
    pub fn __crypto_rng_alg(_arg: crypto_rng_tfm(tfm)->__crt_alg) -> return;
}
//
// crypto_free_rng() - zeroize and free RNG handle
// @tfm: cipher handle to be freed
//
// If @tfm is a NULL or error pointer, this function does nothing.
//
// crypto_rng_generate() - get random number
// @tfm: cipher handle
// @src: Input buffer holding additional data, may be NULL
// @slen: Length of additional data
// @dst: output buffer holding the random numbers
// @dlen: length of the output buffer
//
// This function fills the caller-allocated buffer with random
// numbers using the random number generator referenced by the
// cipher handle.
//
// Return: 0 function was successful; < 0 if an error occurred
//
extern "C" {
    pub fn crypto_rng_alg(_arg: tfm)->generate(tfm, _arg: src, _arg: slen, _arg: dst, _arg: dlen) -> return;
}
//
// crypto_rng_get_bytes() - get random number
// @tfm: cipher handle
// @rdata: output buffer holding the random numbers
// @dlen: length of the output buffer
//
// This function fills the caller-allocated buffer with random numbers using the
// random number generator referenced by the cipher handle.
//
// Return: 0 function was successful; < 0 if an error occurred
//
extern "C" {
    pub fn crypto_rng_generate(_arg: tfm, _arg: NULL, _arg: 0, _arg: rdata, _arg: dlen) -> return;
}
//
// crypto_rng_reset() - re-initialize the RNG
// @tfm: cipher handle
// @seed: seed input data
// @slen: length of the seed input data
//
// The reset function completely re-initializes the random number generator
// referenced by the cipher handle by clearing the current state. The new state
// is initialized with the caller provided seed or automatically, depending on
// the random number generator type. (The SP800-90A DRBGs perform an automatic
// seeding.) The seed is provided as a parameter to this function call. The
// provided seed should have the length of the seed size defined for the random
// number generator as defined by crypto_rng_seedsize.
//
// Return: 0 if the setting of the key was successful; < 0 if an error occurred
//
// crypto_rng_seedsize() - obtain seed size of RNG
// @tfm: cipher handle
//
// The function returns the seed size for the random number generator
// referenced by the cipher handle. This value may be zero if the random
// number generator does not implement or require a reseeding. For example,
// the SP800-90A DRBGs implement an automated reseeding after reaching a
// pre-defined threshold.
//
// Return: seed size for the random number generator
//
