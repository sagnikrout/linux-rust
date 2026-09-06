//! Automatically rewritten from C Header to Rust Module
//! Source: include/crypto/sha2.h
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
// Common values for SHA-2 algorithms
//

pub const SHA224_DIGEST_SIZE: c_int = 28;
pub const SHA224_BLOCK_SIZE: c_int = 64;
pub const SHA256_DIGEST_SIZE: c_int = 32;
pub const SHA256_BLOCK_SIZE: c_int = 64;
pub const SHA256_STATE_WORDS: c_int = 8;
pub const SHA384_DIGEST_SIZE: c_int = 48;
pub const SHA384_BLOCK_SIZE: c_int = 128;
pub const SHA512_DIGEST_SIZE: c_int = 64;
pub const SHA512_BLOCK_SIZE: c_int = 128;
pub const SHA512_STATE_SIZE: c_int = 80;
pub const SHA224_H0: c_uint = 0xc1059ed8UL;
pub const SHA224_H1: c_uint = 0x367cd507UL;
pub const SHA224_H2: c_uint = 0x3070dd17UL;
pub const SHA224_H3: c_uint = 0xf70e5939UL;
pub const SHA224_H4: c_uint = 0xffc00b31UL;
pub const SHA224_H5: c_uint = 0x68581511UL;
pub const SHA224_H6: c_uint = 0x64f98fa7UL;
pub const SHA224_H7: c_uint = 0xbefa4fa4UL;
pub const SHA256_H0: c_uint = 0x6a09e667UL;
pub const SHA256_H1: c_uint = 0xbb67ae85UL;
pub const SHA256_H2: c_uint = 0x3c6ef372UL;
pub const SHA256_H3: c_uint = 0xa54ff53aUL;
pub const SHA256_H4: c_uint = 0x510e527fUL;
pub const SHA256_H5: c_uint = 0x9b05688cUL;
pub const SHA256_H6: c_uint = 0x1f83d9abUL;
pub const SHA256_H7: c_uint = 0x5be0cd19UL;
pub const SHA384_H0: c_uint = 0xcbbb9d5dc1059ed8ULL;
pub const SHA384_H1: c_uint = 0x629a292a367cd507ULL;
pub const SHA384_H2: c_uint = 0x9159015a3070dd17ULL;
pub const SHA384_H3: c_uint = 0x152fecd8f70e5939ULL;
pub const SHA384_H4: c_uint = 0x67332667ffc00b31ULL;
pub const SHA384_H5: c_uint = 0x8eb44a8768581511ULL;
pub const SHA384_H6: c_uint = 0xdb0c2e0d64f98fa7ULL;
pub const SHA384_H7: c_uint = 0x47b5481dbefa4fa4ULL;
pub const SHA512_H0: c_uint = 0x6a09e667f3bcc908ULL;
pub const SHA512_H1: c_uint = 0xbb67ae8584caa73bULL;
pub const SHA512_H2: c_uint = 0x3c6ef372fe94f82bULL;
pub const SHA512_H3: c_uint = 0xa54ff53a5f1d36f1ULL;
pub const SHA512_H4: c_uint = 0x510e527fade682d1ULL;
pub const SHA512_H5: c_uint = 0x9b05688c2b3e6c1fULL;
pub const SHA512_H6: c_uint = 0x1f83d9abfb41bd6bULL;
pub const SHA512_H7: c_uint = 0x5be0cd19137e2179ULL;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_sha256_state {
    pub state: [u32; SHA256_STATE_WORDS],
    pub count: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sha256_state {
    pub ctx: crypto_sha256_state,
    pub state: [u32; SHA256_STATE_WORDS],
    pub count: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sha512_state {
    pub 8]: u64 state[SHA512_DIGEST_SIZE /,
    pub count: [u64; 2],
    pub buf: [u8; SHA512_BLOCK_SIZE],
}

// State for the SHA-256 (and SHA-224) compression function
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sha256_block_state {
    pub h: [u32; SHA256_STATE_WORDS],
}

//
// Context structure, shared by SHA-224 and SHA-256.  The sha224_ctx and
// sha256_ctx structs wrap this one so that the API has proper typing and
// doesn't allow mixing the SHA-224 and SHA-256 functions arbitrarily.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __sha256_ctx {
    pub state: sha256_block_state,
    pub bytecount: u64,
    pub __aligned(__alignof__(__be64)): u8 buf[SHA256_BLOCK_SIZE],
}

extern "C" {
    pub fn __sha256_update(ctx: *mut __sha256_ctx, data: *const u8, len: usize);
}
//
// HMAC key and message context structs, shared by HMAC-SHA224 and HMAC-SHA256.
// The hmac_sha224_* and hmac_sha256_* structs wrap this one so that the API has
// proper typing and doesn't allow mixing the functions arbitrarily.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __hmac_sha256_key {
    pub istate: sha256_block_state,
    pub ostate: sha256_block_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __hmac_sha256_ctx {
    pub sha_ctx: __sha256_ctx,
    pub ostate: sha256_block_state,
}

//
// struct sha224_ctx - Context for hashing a message with SHA-224
// @ctx: private
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sha224_ctx {
    pub ctx: __sha256_ctx,
}

//
// sha224_init() - Initialize a SHA-224 context for a new message
// @ctx: the context to initialize
//
// If you don't need incremental computation, consider sha224() instead.
//
// Context: Any context.
//
extern "C" {
    pub fn sha224_init(ctx: *mut sha224_ctx);
}
//
// sha224_update() - Update a SHA-224 context with message data
// @ctx: the context to update; must have been initialized
// @data: the message data
// @len: the data length in bytes
//
// This can be called any number of times.
//
// Context: Any context.
//
// sha224_final() - Finish computing a SHA-224 message digest
// @ctx: the context to finalize; must have been initialized
// @out: (output) the resulting SHA-224 message digest
//
// After finishing, this zeroizes @ctx.  So the caller does not need to do it.
//
// Context: Any context.
//
extern "C" {
    pub fn sha224_final(ctx: *mut sha224_ctx, SHA224_DIGEST_SIZE]: u8 out[at_least);
}
//
// sha224() - Compute SHA-224 message digest in one shot
// @data: the message data
// @len: the data length in bytes
// @out: (output) the resulting SHA-224 message digest
//
// Context: Any context.
//
extern "C" {
    pub fn sha224(data: *const u8, len: usize, SHA224_DIGEST_SIZE]: u8 out[at_least);
}
//
// struct hmac_sha224_key - Prepared key for HMAC-SHA224
// @key: private
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hmac_sha224_key {
    pub key: __hmac_sha256_key,
}

//
// struct hmac_sha224_ctx - Context for computing HMAC-SHA224 of a message
// @ctx: private
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hmac_sha224_ctx {
    pub ctx: __hmac_sha256_ctx,
}

//
// hmac_sha224_preparekey() - Prepare a key for HMAC-SHA224
// @key: (output) the key structure to initialize
// @raw_key: the raw HMAC-SHA224 key
// @raw_key_len: the key length in bytes.  All key lengths are supported.
//
// Note: the caller is responsible for zeroizing both the struct hmac_sha224_key
// and the raw key once they are no longer needed.
//
// Context: Any context.
//
// hmac_sha224_init() - Initialize an HMAC-SHA224 context for a new message
// @ctx: (output) the HMAC context to initialize
// @key: the prepared HMAC key
//
// If you don't need incremental computation, consider hmac_sha224() instead.
//
// Context: Any context.
//
// hmac_sha224_init_usingrawkey() - Initialize an HMAC-SHA224 context for a new
// message, using a raw key
// @ctx: (output) the HMAC context to initialize
// @raw_key: the raw HMAC-SHA224 key
// @raw_key_len: the key length in bytes.  All key lengths are supported.
//
// If you don't need incremental computation, consider hmac_sha224_usingrawkey()
// instead.
//
// Context: Any context.
//
// hmac_sha224_update() - Update an HMAC-SHA224 context with message data
// @ctx: the HMAC context to update; must have been initialized
// @data: the message data
// @data_len: the data length in bytes
//
// This can be called any number of times.
//
// Context: Any context.
//
// hmac_sha224_final() - Finish computing an HMAC-SHA224 value
// @ctx: the HMAC context to finalize; must have been initialized
// @out: (output) the resulting HMAC-SHA224 value
//
// After finishing, this zeroizes @ctx.  So the caller does not need to do it.
//
// Context: Any context.
//
// hmac_sha224() - Compute HMAC-SHA224 in one shot, using a prepared key
// @key: the prepared HMAC key
// @data: the message data
// @data_len: the data length in bytes
// @out: (output) the resulting HMAC-SHA224 value
//
// If you're using the key only once, consider using hmac_sha224_usingrawkey().
//
// Context: Any context.
//
// hmac_sha224_usingrawkey() - Compute HMAC-SHA224 in one shot, using a raw key
// @raw_key: the raw HMAC-SHA224 key
// @raw_key_len: the key length in bytes.  All key lengths are supported.
// @data: the message data
// @data_len: the data length in bytes
// @out: (output) the resulting HMAC-SHA224 value
//
// If you're using the key multiple times, prefer to use
// hmac_sha224_preparekey() followed by multiple calls to hmac_sha224() instead.
//
// Context: Any context.
//
// struct sha256_ctx - Context for hashing a message with SHA-256
// @ctx: private
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sha256_ctx {
    pub ctx: __sha256_ctx,
}

//
// sha256_init() - Initialize a SHA-256 context for a new message
// @ctx: the context to initialize
//
// If you don't need incremental computation, consider sha256() instead.
//
// Context: Any context.
//
extern "C" {
    pub fn sha256_init(ctx: *mut sha256_ctx);
}
//
// sha256_update() - Update a SHA-256 context with message data
// @ctx: the context to update; must have been initialized
// @data: the message data
// @len: the data length in bytes
//
// This can be called any number of times.
//
// Context: Any context.
//
// sha256_final() - Finish computing a SHA-256 message digest
// @ctx: the context to finalize; must have been initialized
// @out: (output) the resulting SHA-256 message digest
//
// After finishing, this zeroizes @ctx.  So the caller does not need to do it.
//
// Context: Any context.
//
extern "C" {
    pub fn sha256_final(ctx: *mut sha256_ctx, SHA256_DIGEST_SIZE]: u8 out[at_least);
}
//
// sha256() - Compute SHA-256 message digest in one shot
// @data: the message data
// @len: the data length in bytes
// @out: (output) the resulting SHA-256 message digest
//
// Context: Any context.
//
extern "C" {
    pub fn sha256(data: *const u8, len: usize, SHA256_DIGEST_SIZE]: u8 out[at_least);
}
//
// sha256_finup_2x() - Compute two SHA-256 digests from a common initial
// context.  On some CPUs, this is faster than sequentially
// computing each digest.
// @ctx: an optional initial context, which may have already processed data.  If
// NULL, a default initial context is used (equivalent to sha256_init()).
// @data1: data for the first message
// @data2: data for the second message
// @len: the length of each of @data1 and @data2, in bytes
// @out1: (output) the first SHA-256 message digest
// @out2: (output) the second SHA-256 message digest
//
// Context: Any context.
//
// sha256_finup_2x_is_optimized() - Check if sha256_finup_2x() is using a real
// interleaved implementation, as opposed to a
// sequential fallback
// @return: true if optimized
//
// Context: Any context.
//
extern "C" {
    pub fn sha256_finup_2x_is_optimized() -> bool;
}
//
// struct hmac_sha256_key - Prepared key for HMAC-SHA256
// @key: private
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hmac_sha256_key {
    pub key: __hmac_sha256_key,
}

//
// struct hmac_sha256_ctx - Context for computing HMAC-SHA256 of a message
// @ctx: private
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hmac_sha256_ctx {
    pub ctx: __hmac_sha256_ctx,
}

//
// hmac_sha256_preparekey() - Prepare a key for HMAC-SHA256
// @key: (output) the key structure to initialize
// @raw_key: the raw HMAC-SHA256 key
// @raw_key_len: the key length in bytes.  All key lengths are supported.
//
// Note: the caller is responsible for zeroizing both the struct hmac_sha256_key
// and the raw key once they are no longer needed.
//
// Context: Any context.
//
// hmac_sha256_init() - Initialize an HMAC-SHA256 context for a new message
// @ctx: (output) the HMAC context to initialize
// @key: the prepared HMAC key
//
// If you don't need incremental computation, consider hmac_sha256() instead.
//
// Context: Any context.
//
// hmac_sha256_init_usingrawkey() - Initialize an HMAC-SHA256 context for a new
// message, using a raw key
// @ctx: (output) the HMAC context to initialize
// @raw_key: the raw HMAC-SHA256 key
// @raw_key_len: the key length in bytes.  All key lengths are supported.
//
// If you don't need incremental computation, consider hmac_sha256_usingrawkey()
// instead.
//
// Context: Any context.
//
// hmac_sha256_update() - Update an HMAC-SHA256 context with message data
// @ctx: the HMAC context to update; must have been initialized
// @data: the message data
// @data_len: the data length in bytes
//
// This can be called any number of times.
//
// Context: Any context.
//
// hmac_sha256_final() - Finish computing an HMAC-SHA256 value
// @ctx: the HMAC context to finalize; must have been initialized
// @out: (output) the resulting HMAC-SHA256 value
//
// After finishing, this zeroizes @ctx.  So the caller does not need to do it.
//
// Context: Any context.
//
// hmac_sha256() - Compute HMAC-SHA256 in one shot, using a prepared key
// @key: the prepared HMAC key
// @data: the message data
// @data_len: the data length in bytes
// @out: (output) the resulting HMAC-SHA256 value
//
// If you're using the key only once, consider using hmac_sha256_usingrawkey().
//
// Context: Any context.
//
// hmac_sha256_usingrawkey() - Compute HMAC-SHA256 in one shot, using a raw key
// @raw_key: the raw HMAC-SHA256 key
// @raw_key_len: the key length in bytes.  All key lengths are supported.
// @data: the message data
// @data_len: the data length in bytes
// @out: (output) the resulting HMAC-SHA256 value
//
// If you're using the key multiple times, prefer to use
// hmac_sha256_preparekey() followed by multiple calls to hmac_sha256() instead.
//
// Context: Any context.
//
// State for the SHA-512 (and SHA-384) compression function
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sha512_block_state {
    pub h: [u64; 8],
}

//
// Context structure, shared by SHA-384 and SHA-512.  The sha384_ctx and
// sha512_ctx structs wrap this one so that the API has proper typing and
// doesn't allow mixing the SHA-384 and SHA-512 functions arbitrarily.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __sha512_ctx {
    pub state: sha512_block_state,
    pub bytecount_lo: u64,
    pub bytecount_hi: u64,
    pub __aligned(__alignof__(__be64)): u8 buf[SHA512_BLOCK_SIZE],
}

extern "C" {
    pub fn __sha512_update(ctx: *mut __sha512_ctx, data: *const u8, len: usize);
}
//
// HMAC key and message context structs, shared by HMAC-SHA384 and HMAC-SHA512.
// The hmac_sha384_* and hmac_sha512_* structs wrap this one so that the API has
// proper typing and doesn't allow mixing the functions arbitrarily.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __hmac_sha512_key {
    pub istate: sha512_block_state,
    pub ostate: sha512_block_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __hmac_sha512_ctx {
    pub sha_ctx: __sha512_ctx,
    pub ostate: sha512_block_state,
}

//
// struct sha384_ctx - Context for hashing a message with SHA-384
// @ctx: private
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sha384_ctx {
    pub ctx: __sha512_ctx,
}

//
// sha384_init() - Initialize a SHA-384 context for a new message
// @ctx: the context to initialize
//
// If you don't need incremental computation, consider sha384() instead.
//
// Context: Any context.
//
extern "C" {
    pub fn sha384_init(ctx: *mut sha384_ctx);
}
//
// sha384_update() - Update a SHA-384 context with message data
// @ctx: the context to update; must have been initialized
// @data: the message data
// @len: the data length in bytes
//
// This can be called any number of times.
//
// Context: Any context.
//
// sha384_final() - Finish computing a SHA-384 message digest
// @ctx: the context to finalize; must have been initialized
// @out: (output) the resulting SHA-384 message digest
//
// After finishing, this zeroizes @ctx.  So the caller does not need to do it.
//
// Context: Any context.
//
extern "C" {
    pub fn sha384_final(ctx: *mut sha384_ctx, SHA384_DIGEST_SIZE]: u8 out[at_least);
}
//
// sha384() - Compute SHA-384 message digest in one shot
// @data: the message data
// @len: the data length in bytes
// @out: (output) the resulting SHA-384 message digest
//
// Context: Any context.
//
extern "C" {
    pub fn sha384(data: *const u8, len: usize, SHA384_DIGEST_SIZE]: u8 out[at_least);
}
//
// struct hmac_sha384_key - Prepared key for HMAC-SHA384
// @key: private
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hmac_sha384_key {
    pub key: __hmac_sha512_key,
}

//
// struct hmac_sha384_ctx - Context for computing HMAC-SHA384 of a message
// @ctx: private
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hmac_sha384_ctx {
    pub ctx: __hmac_sha512_ctx,
}

//
// hmac_sha384_preparekey() - Prepare a key for HMAC-SHA384
// @key: (output) the key structure to initialize
// @raw_key: the raw HMAC-SHA384 key
// @raw_key_len: the key length in bytes.  All key lengths are supported.
//
// Note: the caller is responsible for zeroizing both the struct hmac_sha384_key
// and the raw key once they are no longer needed.
//
// Context: Any context.
//
// hmac_sha384_init() - Initialize an HMAC-SHA384 context for a new message
// @ctx: (output) the HMAC context to initialize
// @key: the prepared HMAC key
//
// If you don't need incremental computation, consider hmac_sha384() instead.
//
// Context: Any context.
//
// hmac_sha384_init_usingrawkey() - Initialize an HMAC-SHA384 context for a new
// message, using a raw key
// @ctx: (output) the HMAC context to initialize
// @raw_key: the raw HMAC-SHA384 key
// @raw_key_len: the key length in bytes.  All key lengths are supported.
//
// If you don't need incremental computation, consider hmac_sha384_usingrawkey()
// instead.
//
// Context: Any context.
//
// hmac_sha384_update() - Update an HMAC-SHA384 context with message data
// @ctx: the HMAC context to update; must have been initialized
// @data: the message data
// @data_len: the data length in bytes
//
// This can be called any number of times.
//
// Context: Any context.
//
// hmac_sha384_final() - Finish computing an HMAC-SHA384 value
// @ctx: the HMAC context to finalize; must have been initialized
// @out: (output) the resulting HMAC-SHA384 value
//
// After finishing, this zeroizes @ctx.  So the caller does not need to do it.
//
// Context: Any context.
//
// hmac_sha384() - Compute HMAC-SHA384 in one shot, using a prepared key
// @key: the prepared HMAC key
// @data: the message data
// @data_len: the data length in bytes
// @out: (output) the resulting HMAC-SHA384 value
//
// If you're using the key only once, consider using hmac_sha384_usingrawkey().
//
// Context: Any context.
//
// hmac_sha384_usingrawkey() - Compute HMAC-SHA384 in one shot, using a raw key
// @raw_key: the raw HMAC-SHA384 key
// @raw_key_len: the key length in bytes.  All key lengths are supported.
// @data: the message data
// @data_len: the data length in bytes
// @out: (output) the resulting HMAC-SHA384 value
//
// If you're using the key multiple times, prefer to use
// hmac_sha384_preparekey() followed by multiple calls to hmac_sha384() instead.
//
// Context: Any context.
//
// struct sha512_ctx - Context for hashing a message with SHA-512
// @ctx: private
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sha512_ctx {
    pub ctx: __sha512_ctx,
}

//
// sha512_init() - Initialize a SHA-512 context for a new message
// @ctx: the context to initialize
//
// If you don't need incremental computation, consider sha512() instead.
//
// Context: Any context.
//
extern "C" {
    pub fn sha512_init(ctx: *mut sha512_ctx);
}
//
// sha512_update() - Update a SHA-512 context with message data
// @ctx: the context to update; must have been initialized
// @data: the message data
// @len: the data length in bytes
//
// This can be called any number of times.
//
// Context: Any context.
//
// sha512_final() - Finish computing a SHA-512 message digest
// @ctx: the context to finalize; must have been initialized
// @out: (output) the resulting SHA-512 message digest
//
// After finishing, this zeroizes @ctx.  So the caller does not need to do it.
//
// Context: Any context.
//
extern "C" {
    pub fn sha512_final(ctx: *mut sha512_ctx, SHA512_DIGEST_SIZE]: u8 out[at_least);
}
//
// sha512() - Compute SHA-512 message digest in one shot
// @data: the message data
// @len: the data length in bytes
// @out: (output) the resulting SHA-512 message digest
//
// Context: Any context.
//
extern "C" {
    pub fn sha512(data: *const u8, len: usize, SHA512_DIGEST_SIZE]: u8 out[at_least);
}
//
// struct hmac_sha512_key - Prepared key for HMAC-SHA512
// @key: private
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hmac_sha512_key {
    pub key: __hmac_sha512_key,
}

//
// struct hmac_sha512_ctx - Context for computing HMAC-SHA512 of a message
// @ctx: private
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hmac_sha512_ctx {
    pub ctx: __hmac_sha512_ctx,
}

//
// hmac_sha512_preparekey() - Prepare a key for HMAC-SHA512
// @key: (output) the key structure to initialize
// @raw_key: the raw HMAC-SHA512 key
// @raw_key_len: the key length in bytes.  All key lengths are supported.
//
// Note: the caller is responsible for zeroizing both the struct hmac_sha512_key
// and the raw key once they are no longer needed.
//
// Context: Any context.
//
// hmac_sha512_init() - Initialize an HMAC-SHA512 context for a new message
// @ctx: (output) the HMAC context to initialize
// @key: the prepared HMAC key
//
// If you don't need incremental computation, consider hmac_sha512() instead.
//
// Context: Any context.
//
// hmac_sha512_init_usingrawkey() - Initialize an HMAC-SHA512 context for a new
// message, using a raw key
// @ctx: (output) the HMAC context to initialize
// @raw_key: the raw HMAC-SHA512 key
// @raw_key_len: the key length in bytes.  All key lengths are supported.
//
// If you don't need incremental computation, consider hmac_sha512_usingrawkey()
// instead.
//
// Context: Any context.
//
// hmac_sha512_update() - Update an HMAC-SHA512 context with message data
// @ctx: the HMAC context to update; must have been initialized
// @data: the message data
// @data_len: the data length in bytes
//
// This can be called any number of times.
//
// Context: Any context.
//
// hmac_sha512_final() - Finish computing an HMAC-SHA512 value
// @ctx: the HMAC context to finalize; must have been initialized
// @out: (output) the resulting HMAC-SHA512 value
//
// After finishing, this zeroizes @ctx.  So the caller does not need to do it.
//
// Context: Any context.
//
// hmac_sha512() - Compute HMAC-SHA512 in one shot, using a prepared key
// @key: the prepared HMAC key
// @data: the message data
// @data_len: the data length in bytes
// @out: (output) the resulting HMAC-SHA512 value
//
// If you're using the key only once, consider using hmac_sha512_usingrawkey().
//
// Context: Any context.
//
// hmac_sha512_usingrawkey() - Compute HMAC-SHA512 in one shot, using a raw key
// @raw_key: the raw HMAC-SHA512 key
// @raw_key_len: the key length in bytes.  All key lengths are supported.
// @data: the message data
// @data_len: the data length in bytes
// @out: (output) the resulting HMAC-SHA512 value
//
// If you're using the key multiple times, prefer to use
// hmac_sha512_preparekey() followed by multiple calls to hmac_sha512() instead.
//
// Context: Any context.
//
