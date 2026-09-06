//! Automatically rewritten from C Header to Rust Module
//! Source: include/crypto/aes.h
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
// Common values for AES algorithms
//

pub const AES_MIN_KEY_SIZE: c_int = 16;
pub const AES_MAX_KEY_SIZE: c_int = 32;
pub const AES_KEYSIZE_128: c_int = 16;
pub const AES_KEYSIZE_192: c_int = 24;
pub const AES_KEYSIZE_256: c_int = 32;
pub const AES_BLOCK_SIZE: c_int = 16;

//
// The POWER8 VSX optimized AES assembly code is borrowed from OpenSSL and
// inherits OpenSSL's AES_KEY format, which stores the number of rounds after
// the round keys.  That assembly code is difficult to change.  So for
// compatibility purposes we reserve space for the extra nrounds field on PPC64.
//
// Note: when prepared for decryption, the round keys are just the reversed
// standard round keys, not the round keys for the Equivalent Inverse Cipher.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p8_aes_key {
    pub rndkeys: [u32; AES_MAX_KEYLENGTH_U32],
    pub nrounds: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union aes_enckey_arch {
    pub rndkeys: [u32; AES_MAX_KEYLENGTH_U32],
// Used unconditionally (when SPE AES code is enabled in kconfig)
    pub __aligned(8): u32 spe_enc_key[AES_MAX_KEYLENGTH_U32],

//
// Kernels that include the POWER8 VSX optimized AES code use this field
// when that code is usable at key preparation time.  Otherwise they
// fall back to rndkeys.  In the latter case, p8.nrounds (which doesn't
// overlap rndkeys) is set to 0 to differentiate the two formats.
//
    pub p8: p8_aes_key,

// Used when the CPU supports CPACF AES for this key's length
    pub raw_key: [u8; AES_MAX_KEY_SIZE],
// Used when the CPU supports the SPARC64 AES opcodes
    pub sizeof(u64)]: u64 sparc_rndkeys[AES_MAX_KEYLENGTH /,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub union aes_invkey_arch {
    pub inv_rndkeys: [u32; AES_MAX_KEYLENGTH_U32],
// Used unconditionally (when SPE AES code is enabled in kconfig)
    pub __aligned(8): u32 spe_dec_key[AES_MAX_KEYLENGTH_U32],

// Used conditionally, analogous to aes_enckey_arch::p8
    pub p8: p8_aes_key,

}

//
// struct aes_enckey - An AES key prepared for encryption
// @len: Key length in bytes: 16 for AES-128, 24 for AES-192, 32 for AES-256.
// @nrounds: Number of rounds: 10 for AES-128, 12 for AES-192, 14 for AES-256.
// This is '6 + @len / 4' and is cached so that AES implementations
// that need it don't have to recompute it for each en/decryption.
// @padding: Padding to make offsetof(@k) be a multiple of 16, so that aligning
// this struct to a 16-byte boundary results in @k also being 16-byte
// aligned.  Users aren't required to align this struct to 16 bytes,
// but it may slightly improve performance.
// @k: This typically contains the AES round keys as an array of '@nrounds + 1'
// groups of four u32 words.  However, architecture-specific implementations
// of AES may store something else here, e.g. just the raw key if it's all
// they need.
//
// Note that this struct is about half the size of struct aes_key.  This is
// separate from struct aes_key so that modes that need only AES encryption
// (e.g. AES-GCM, AES-CTR, AES-CMAC, tweak key in AES-XTS) don't incur the time
// and space overhead of computing and caching the decryption round keys.
//
// Note that there's no decryption-only equivalent (i.e. "struct aes_deckey"),
// since (a) it's rare that modes need decryption-only, and (b) some AES
// implementations use the same @k for both encryption and decryption, either
// always or conditionally; in the latter case both @k and @inv_k are needed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aes_enckey {
    pub len: u32,
    pub nrounds: u32,
    pub padding: [u32; 2],
    pub k: aes_enckey_arch,
}

//
// struct aes_key - An AES key prepared for encryption and decryption
// @aes_enckey: Common fields and the key prepared for encryption
// @inv_k: This generally contains the round keys for the AES Equivalent
// Inverse Cipher, as an array of '@nrounds + 1' groups of four u32
// words.  However, architecture-specific implementations of AES may
// store something else here.  For example, they may leave this field
// uninitialized if they use @k for both encryption and decryption.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aes_key {
    pub /: *mut *mut aes_enckey; / Include all fields of aes_enckey.,
    pub inv_k: aes_invkey_arch,
}

//
// Please ensure that the first two fields are 16-byte aligned
// relative to the start of the structure, i.e., don't move them!
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_aes_ctx {
    pub key_enc: [u32; AES_MAX_KEYLENGTH_U32],
    pub key_dec: [u32; AES_MAX_KEYLENGTH_U32],
    pub key_length: u32,
}

//
// validate key length for AES algorithms
//
// aes_expandkey - Expands the AES key as described in FIPS-197
// @ctx:	The location where the computed key will be stored.
// @in_key:	The supplied key.
// @key_len:	The length of the supplied key.
//
// Returns 0 on success. The function fails only if an invalid key size (or
// pointer) is supplied.
// The expanded key size is 240 bytes (max of 14 rounds with a unique 16 bytes
// key schedule plus a 16 bytes key which is used before the first round).
// The decryption key is prepared for the "Equivalent Inverse Cipher" as
// described in FIPS-197. The first slot (16 bytes) of each key (enc or dec) is
// for the initial combination, the second slot for the first round and so on.
//
// The following functions are temporarily exported for use by the AES mode
// implementations in arch/$(SRCARCH)/crypto/.  These exports will go away when
// that code is migrated into lib/crypto/.
//

extern "C" {
    pub fn ppc_expand_key_128(key_enc: *mut u32, key: *const u8);
}
extern "C" {
    pub fn ppc_expand_key_192(key_enc: *mut u32, key: *const u8);
}
extern "C" {
    pub fn ppc_expand_key_256(key_enc: *mut u32, key: *const u8);
}
extern "C" {
    pub fn ppc_generate_decrypt_key(key_dec: *mut u32, key_enc: *mut u32, key_len: c_uint);
}
extern "C" {
    pub fn aes_p8_encrypt(in: *const u8, out: *mut u8, key: *const p8_aes_key);
}
extern "C" {
    pub fn aes_p8_decrypt(in: *const u8, out: *mut u8, key: *const p8_aes_key);
}

extern "C" {
    pub fn aes_sparc64_load_encrypt_keys_128(key: *const u64);
}
extern "C" {
    pub fn aes_sparc64_load_encrypt_keys_192(key: *const u64);
}
extern "C" {
    pub fn aes_sparc64_load_encrypt_keys_256(key: *const u64);
}
extern "C" {
    pub fn aes_sparc64_load_decrypt_keys_128(key: *const u64);
}
extern "C" {
    pub fn aes_sparc64_load_decrypt_keys_192(key: *const u64);
}
extern "C" {
    pub fn aes_sparc64_load_decrypt_keys_256(key: *const u64);
}

//
// aes_preparekey() - Prepare an AES key for encryption and decryption
// @key: (output) The key structure to initialize
// @in_key: The raw AES key
// @key_len: Length of the raw key in bytes.  Should be either AES_KEYSIZE_128,
// AES_KEYSIZE_192, or AES_KEYSIZE_256.
//
// This prepares an AES key for both the encryption and decryption directions of
// the block cipher.  Typically this involves expanding the raw key into both
// the standard round keys and the Equivalent Inverse Cipher round keys, but
// some architecture-specific implementations don't do the full expansion here.
//
// The caller is responsible for zeroizing both the struct aes_key and the raw
// key once they are no longer needed.
//
// If you don't need decryption support, use aes_prepareenckey() instead.
//
// Return: 0 on success or -EINVAL if the given key length is invalid.  No other
// errors are possible, so callers that always pass a valid key length
// don't need to check for errors.
//
// Context: Any context.
//
extern "C" {
    pub fn aes_preparekey(key: *mut aes_key, in_key: *const u8, key_len: usize) -> c_int;
}
//
// aes_prepareenckey() - Prepare an AES key for encryption-only
// @key: (output) The key structure to initialize
// @in_key: The raw AES key
// @key_len: Length of the raw key in bytes.  Should be either AES_KEYSIZE_128,
// AES_KEYSIZE_192, or AES_KEYSIZE_256.
//
// This prepares an AES key for only the encryption direction of the block
// cipher.  Typically this involves expanding the raw key into only the standard
// round keys, resulting in a struct about half the size of struct aes_key.
//
// The caller is responsible for zeroizing both the struct aes_enckey and the
// raw key once they are no longer needed.
//
// Note that while the resulting prepared key supports only AES encryption, it
// can still be used for decrypting in a mode of operation that uses AES in only
// the encryption (forward) direction, for example counter mode.
//
// Return: 0 on success or -EINVAL if the given key length is invalid.  No other
// errors are possible, so callers that always pass a valid key length
// don't need to check for errors.
//
// Context: Any context.
//
extern "C" {
    pub fn aes_prepareenckey(key: *mut aes_enckey, in_key: *const u8, key_len: usize) -> c_int;
}
//
// aes_encrypt() - Encrypt a single AES block
// @key: The AES key, as a pointer to either an encryption-only key
// (struct aes_enckey) or a full, bidirectional key (struct aes_key).
// @out: Buffer to store the ciphertext block
// @in: Buffer containing the plaintext block
//
// Context: Any context.
//
// aes_decrypt() - Decrypt a single AES block
// @key: The AES key, previously initialized by aes_preparekey()
// @out: Buffer to store the plaintext block
// @in: Buffer containing the ciphertext block
//
// Context: Any context.
//
