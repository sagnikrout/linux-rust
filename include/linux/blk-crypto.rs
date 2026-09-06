//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/blk-crypto.h
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
// Copyright 2019 Google LLC
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum blk_crypto_mode_num {
    BLK_ENCRYPTION_MODE_INVALID,
    BLK_ENCRYPTION_MODE_AES_256_XTS,
    BLK_ENCRYPTION_MODE_AES_128_CBC_ESSIV,
    BLK_ENCRYPTION_MODE_ADIANTUM,
    BLK_ENCRYPTION_MODE_SM4_XTS,
    BLK_ENCRYPTION_MODE_MAX,
}

//
// Supported types of keys.  Must be bitflags due to their use in
// blk_crypto_profile::key_types_supported.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum blk_crypto_key_type {
//
// Raw keys (i.e. "software keys").  These keys are simply kept in raw,
// plaintext form in kernel memory.
//
    BLK_CRYPTO_KEY_TYPE_RAW = 0x1,

//
// Hardware-wrapped keys.  These keys are only present in kernel memory
// in ephemerally-wrapped form, and they can only be unwrapped by
// dedicated hardware.  For details, see the "Hardware-wrapped keys"
// section of Documentation/block/inline-encryption.rst.
//
    BLK_CRYPTO_KEY_TYPE_HW_WRAPPED = 0x2,
}

//
// Currently the maximum raw key size is 64 bytes, as that is the key size of
// BLK_ENCRYPTION_MODE_AES_256_XTS which takes the longest key.
//
// The maximum hardware-wrapped key size depends on the hardware's key wrapping
// algorithm, which is a hardware implementation detail, so it isn't precisely
// specified.  But currently 128 bytes is plenty in practice.  Implementations
// are recommended to wrap a 32-byte key for the hardware KDF with AES-256-GCM,
// which should result in a size closer to 64 bytes than 128.
//
// Both of these values can trivially be increased if ever needed.
//
pub const BLK_CRYPTO_MAX_RAW_KEY_SIZE: c_int = 64;
pub const BLK_CRYPTO_MAX_HW_WRAPPED_KEY_SIZE: c_int = 128;

//
// Size of the "software secret" which can be derived from a hardware-wrapped
// key.  This is currently always 32 bytes.  Note, the choice of 32 bytes
// assumes that the software secret is only used directly for algorithms that
// don't require more than a 256-bit key to get the desired security strength.
// If it were to be used e.g. directly as an AES-256-XTS key, then this would
// need to be increased (which is possible if hardware supports it, but care
// would need to be taken to avoid breaking users who need exactly 32 bytes).
//
pub const BLK_CRYPTO_SW_SECRET_SIZE: c_int = 32;
// Flags for blk_crypto_config::flags:
//
// If set, inline encryption hardware will be used if available.
// If unset, CPU-based encryption will always be used (requires
// CONFIG_BLK_INLINE_ENCRYPTION_FALLBACK)
//

//
// struct blk_crypto_config - an inline encryption key's crypto configuration
// @crypto_mode: encryption algorithm this key is for
// @data_unit_size: the data unit size for all encryption/decryptions with this
// key.  This is the size in bytes of each individual plaintext and
// ciphertext.  This is always a power of 2.  It might be e.g. the
// filesystem block size or the disk sector size.
// @dun_bytes: the maximum number of bytes of DUN used when using this key
// @key_type: the type of this key -- either raw or hardware-wrapped
// @flags: BLK_CRYPTO_CFG_* flags
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blk_crypto_config {
    pub crypto_mode: blk_crypto_mode_num,
    pub data_unit_size: c_uint,
    pub dun_bytes: c_uint,
    pub key_type: blk_crypto_key_type,
    pub flags: c_int,
}

//
// struct blk_crypto_key - an inline encryption key
// @crypto_cfg: the crypto mode, data unit size, key type, and other
// characteristics of this key and how it will be used
// @data_unit_size_bits: log2 of data_unit_size
// @size: size of this key in bytes.  The size of a raw key is fixed for a given
// crypto mode, but the size of a hardware-wrapped key can vary.
// @bytes: the bytes of this key.  Only the first @size bytes are significant.
//
// A blk_crypto_key is immutable once created, and many bios can reference it at
// the same time.  It must not be freed until all bios using it have completed
// and it has been evicted from all devices on which it may have been used.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blk_crypto_key {
    pub crypto_cfg: blk_crypto_config,
    pub data_unit_size_bits: c_uint,
    pub size: c_uint,
    pub bytes: [u8; BLK_CRYPTO_MAX_ANY_KEY_SIZE],
}

pub const BLK_CRYPTO_MAX_IV_SIZE: c_int = 32;

//
// struct bio_crypt_ctx - an inline encryption context
// @bc_key: the key, algorithm, and data unit size to use
// @bc_dun: the data unit number (starting IV) to use
//
// A bio_crypt_ctx specifies that the contents of the bio will be encrypted (for
// write requests) or decrypted (for read requests) inline by the storage device
// or controller, or by the crypto API fallback.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bio_crypt_ctx {
    pub bc_key: *const blk_crypto_key,
    pub bc_dun: [u64; BLK_CRYPTO_DUN_ARRAY_SIZE],
}

extern "C" {
    pub fn __blk_crypto_submit_bio(bio: *mut bio) -> bool;
}
//
// blk_crypto_submit_bio - Submit a bio that may have a crypto context
// @bio: bio to submit
//
// If @bio has no crypto context, or the crypt context attached to @bio is
// supported by the underlying device's inline encryption hardware, just submit
// @bio.
//
// Otherwise, try to perform en/decryption for this bio by falling back to the
// kernel crypto API. For encryption this means submitting newly allocated
// bios for the encrypted payload while keeping back the source bio until they
// complete, while for reads the decryption happens in-place by a hooked in
// completion handler.
//
extern "C" {
    pub fn __bio_crypt_clone(dst: *mut bio, src: *mut bio, gfp_mask: gfp_t) -> c_int;
}
//
// bio_crypt_clone - clone bio encryption context
// @dst: destination bio
// @src: source bio
// @gfp_mask: memory allocation flags
//
// If @src has an encryption context, clone it to @dst.
//
// Return: 0 on success, -ENOMEM if out of memory.  -ENOMEM is only possible if
// @gfp_mask doesn't include %__GFP_DIRECT_RECLAIM.
//
extern "C" {
    pub fn __bio_crypt_clone(_arg: dst, _arg: src, _arg: gfp_mask) -> return;
}
