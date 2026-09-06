//! Automatically rewritten from C Header to Rust Module
//! Source: include/crypto/sig.h
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
// Public Key Signature Algorithm
//
// Copyright (c) 2023 Herbert Xu <herbert@gondor.apana.org.au>
//

//
// struct crypto_sig - user-instantiated objects which encapsulate
// algorithms and core processing logic
//
// @base:	Common crypto API algorithm data structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_sig {
    pub base: crypto_tfm,
}

//
// struct sig_alg - generic public key signature algorithm
//
// @sign:	Function performs a sign operation as defined by public key
// algorithm. On success, the signature size is returned.
// Optional.
// @verify:	Function performs a complete verify operation as defined by
// public key algorithm, returning verification status. Optional.
// @set_pub_key: Function invokes the algorithm specific set public key
// function, which knows how to decode and interpret
// the BER encoded public key and parameters. Mandatory.
// @set_priv_key: Function invokes the algorithm specific set private key
// function, which knows how to decode and interpret
// the BER encoded private key and parameters. Optional.
// @key_size:	Function returns key size. Mandatory.
// @digest_size: Function returns maximum digest size. Optional.
// @max_size:	Function returns maximum signature size. Optional.
// @init:	Initialize the cryptographic transformation object.
// This function is used to initialize the cryptographic
// transformation object. This function is called only once at
// the instantiation time, right after the transformation context
// was allocated. In case the cryptographic hardware has some
// special requirements which need to be handled by software, this
// function shall check for the precise requirement of the
// transformation and put any software fallbacks in place.
// @exit:	Deinitialize the cryptographic transformation object. This is a
// counterpart to @init, used to remove various changes set in
// @init.
//
// @base:	Common crypto API algorithm data structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sig_alg {
    pub dlen): *mut *mut void dst, unsigned int,
    pub dlen): *const *const void digest, unsigned int,
    pub keylen): *const *const void key, unsigned int,
    pub keylen): *const *const void key, unsigned int,
    pub tfm): *mut *mut unsigned int (key_size)(struct crypto_sig,
    pub tfm): *mut *mut unsigned int (digest_size)(struct crypto_sig,
    pub tfm): *mut *mut unsigned int (max_size)(struct crypto_sig,
    pub tfm): *mut *mut int (init)(struct crypto_sig,
    pub tfm): *mut *mut void (exit)(struct crypto_sig,
    pub base: crypto_alg,
}

//
// DOC: Generic Public Key Signature API
//
// The Public Key Signature API is used with the algorithms of type
// CRYPTO_ALG_TYPE_SIG (listed as type "sig" in /proc/crypto)
//
// crypto_alloc_sig() - allocate signature tfm handle
// @alg_name: is the cra_name / name or cra_driver_name / driver name of the
// signing algorithm e.g. "ecdsa"
// @type: specifies the type of the algorithm
// @mask: specifies the mask for the algorithm
//
// Allocate a handle for public key signature algorithm. The returned struct
// crypto_sig is the handle that is required for any subsequent
// API invocation for signature operations.
//
// Return: allocated handle in case of success; IS_ERR() is true in case
// of an error, PTR_ERR() returns the error code.
//
extern "C" {
    pub fn container_of(_arg: tfm, crypto_sig: struct, _arg: base) -> return;
}
extern "C" {
    pub fn container_of(_arg: alg, sig_alg: struct, _arg: base) -> return;
}
extern "C" {
    pub fn __crypto_sig_alg(_arg: crypto_sig_tfm(tfm)->__crt_alg) -> return;
}
//
// crypto_free_sig() - free signature tfm handle
//
// @tfm: signature tfm handle allocated with crypto_alloc_sig()
//
// If @tfm is a NULL or error pointer, this function does nothing.
//
// crypto_sig_keysize() - Get key size
//
// Function returns the key size in bits.
// Function assumes that the key is already set in the transformation. If this
// function is called without a setkey or with a failed setkey, you may end up
// in a NULL dereference.
//
// @tfm:	signature tfm handle allocated with crypto_alloc_sig()
//
// crypto_sig_digestsize() - Get maximum digest size
//
// Function returns the maximum digest size in bytes.
// Function assumes that the key is already set in the transformation. If this
// function is called without a setkey or with a failed setkey, you may end up
// in a NULL dereference.
//
// @tfm:	signature tfm handle allocated with crypto_alloc_sig()
//
// crypto_sig_maxsize() - Get maximum signature size
//
// Function returns the maximum signature size in bytes.
// Function assumes that the key is already set in the transformation. If this
// function is called without a setkey or with a failed setkey, you may end up
// in a NULL dereference.
//
// @tfm:	signature tfm handle allocated with crypto_alloc_sig()
//
// crypto_sig_sign() - Invoke signing operation
//
// Function invokes the specific signing operation for a given algorithm
//
// @tfm:	signature tfm handle allocated with crypto_alloc_sig()
// @src:	source buffer
// @slen:	source length
// @dst:	destination obuffer
// @dlen:	destination length
//
// Return: signature size on success; error code in case of error
//
// crypto_sig_verify() - Invoke signature verification
//
// Function invokes the specific signature verification operation
// for a given algorithm.
//
// @tfm:	signature tfm handle allocated with crypto_alloc_sig()
// @src:	source buffer
// @slen:	source length
// @digest:	digest
// @dlen:	digest length
//
// Return: zero on verification success; error code in case of error.
//
// crypto_sig_set_pubkey() - Invoke set public key operation
//
// Function invokes the algorithm specific set key function, which knows
// how to decode and interpret the encoded key and parameters
//
// @tfm:	tfm handle
// @key:	BER encoded public key, algo OID, paramlen, BER encoded
// parameters
// @keylen:	length of the key (not including other data)
//
// Return: zero on success; error code in case of error
//
// crypto_sig_set_privkey() - Invoke set private key operation
//
// Function invokes the algorithm specific set key function, which knows
// how to decode and interpret the encoded key and parameters
//
// @tfm:	tfm handle
// @key:	BER encoded private key, algo OID, paramlen, BER encoded
// parameters
// @keylen:	length of the key (not including other data)
//
// Return: zero on success; error code in case of error
//
