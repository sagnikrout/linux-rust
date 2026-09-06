//! Automatically rewritten from C Header to Rust Module
//! Source: include/crypto/akcipher.h
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
// Public Key Encryption
//
// Copyright (c) 2015, Intel Corporation
// Authors: Tadeusz Struk <tadeusz.struk@intel.com>
//

//
// struct akcipher_request - public key cipher request
//
// @base:	Common attributes for async crypto requests
// @src:	Source data
// @dst:	Destination data
// @src_len:	Size of the input buffer
// @dst_len:	Size of @dst buffer
// It needs to be at least	as big as the expected result
// depending on the operation.
// After operation it will be updated with the actual size of the
// result.
// In case of error where the dst sgl size was insufficient,
// it will be updated to the size required for the operation.
// @__ctx:	Start of private context data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct akcipher_request {
    pub base: crypto_async_request,
    pub src: *mut scatterlist,
    pub dst: *mut scatterlist,
    pub src_len: c_uint,
    pub dst_len: c_uint,
    pub CRYPTO_MINALIGN_ATTR: *mut *mut void __ctx[],
}

//
// struct crypto_akcipher - user-instantiated objects which encapsulate
// algorithms and core processing logic
//
// @reqsize:	Request context size required by algorithm implementation
// @base:	Common crypto API algorithm data structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_akcipher {
    pub reqsize: c_uint,
    pub base: crypto_tfm,
}

//
// struct akcipher_alg - generic public key cipher algorithm
//
// @encrypt:	Function performs an encrypt operation as defined by public key
// algorithm. In case of error, where the dst_len was insufficient,
// the req->dst_len will be updated to the size required for the
// operation
// @decrypt:	Function performs a decrypt operation as defined by public key
// algorithm. In case of error, where the dst_len was insufficient,
// the req->dst_len will be updated to the size required for the
// operation
// @set_pub_key: Function invokes the algorithm specific set public key
// function, which knows how to decode and interpret
// the BER encoded public key and parameters
// @set_priv_key: Function invokes the algorithm specific set private key
// function, which knows how to decode and interpret
// the BER encoded private key and parameters
// @max_size:	Function returns dest buffer size required for a given key.
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
pub struct akcipher_alg {
    pub req): *mut *mut int (encrypt)(struct akcipher_request,
    pub req): *mut *mut int (decrypt)(struct akcipher_request,
    pub keylen): c_uint,
    pub keylen): c_uint,
    pub tfm): *mut *mut unsigned int (max_size)(struct crypto_akcipher,
    pub tfm): *mut *mut int (init)(struct crypto_akcipher,
    pub tfm): *mut *mut void (exit)(struct crypto_akcipher,
    pub base: crypto_alg,
}

//
// DOC: Generic Public Key Cipher API
//
// The Public Key Cipher API is used with the algorithms of type
// CRYPTO_ALG_TYPE_AKCIPHER (listed as type "akcipher" in /proc/crypto)
//
// crypto_alloc_akcipher() - allocate AKCIPHER tfm handle
// @alg_name: is the cra_name / name or cra_driver_name / driver name of the
// public key algorithm e.g. "rsa"
// @type: specifies the type of the algorithm
// @mask: specifies the mask for the algorithm
//
// Allocate a handle for public key algorithm. The returned struct
// crypto_akcipher is the handle that is required for any subsequent
// API invocation for the public key operations.
//
// Return: allocated handle in case of success; IS_ERR() is true in case
// of an error, PTR_ERR() returns the error code.
//
extern "C" {
    pub fn container_of(_arg: alg, akcipher_alg: struct, _arg: base) -> return;
}
extern "C" {
    pub fn container_of(_arg: tfm, crypto_akcipher: struct, _arg: base) -> return;
}
extern "C" {
    pub fn __crypto_akcipher_alg(_arg: crypto_akcipher_tfm(tfm)->__crt_alg) -> return;
}
extern "C" {
    pub fn __crypto_akcipher_tfm(_arg: req->base.tfm) -> return;
}
//
// crypto_free_akcipher() - free AKCIPHER tfm handle
//
// @tfm: AKCIPHER tfm handle allocated with crypto_alloc_akcipher()
//
// If @tfm is a NULL or error pointer, this function does nothing.
//
// akcipher_request_alloc() - allocates public key request
//
// @tfm:	AKCIPHER tfm handle allocated with crypto_alloc_akcipher()
// @gfp:	allocation flags
//
// Return: allocated handle in case of success or NULL in case of an error.
//
// akcipher_request_free() - zeroize and free public key request
//
// @req:	request to free
//
// akcipher_request_set_callback() - Sets an asynchronous callback.
//
// Callback will be called when an asynchronous operation on a given
// request is finished.
//
// @req:	request that the callback will be set for
// @flgs:	specify for instance if the operation may backlog
// @cmpl:	callback which will be called
// @data:	private data used by the caller
//
// akcipher_request_set_crypt() - Sets request parameters
//
// Sets parameters required by crypto operation
//
// @req:	public key request
// @src:	ptr to input scatter list
// @dst:	ptr to output scatter list
// @src_len:	size of the src input scatter list to be processed
// @dst_len:	size of the dst output scatter list
//
// crypto_akcipher_maxsize() - Get len for output buffer
//
// Function returns the dest buffer size required for a given key.
// Function assumes that the key is already set in the transformation. If this
// function is called without a setkey or with a failed setkey, you will end up
// in a NULL dereference.
//
// @tfm:	AKCIPHER tfm handle allocated with crypto_alloc_akcipher()
//
// crypto_akcipher_encrypt() - Invoke public key encrypt operation
//
// Function invokes the specific public key encrypt operation for a given
// public key algorithm
//
// @req:	asymmetric key request
//
// Return: zero on success; error code in case of error
//
extern "C" {
    pub fn crypto_akcipher_alg(_arg: tfm)->encrypt(req) -> return;
}
//
// crypto_akcipher_decrypt() - Invoke public key decrypt operation
//
// Function invokes the specific public key decrypt operation for a given
// public key algorithm
//
// @req:	asymmetric key request
//
// Return: zero on success; error code in case of error
//
extern "C" {
    pub fn crypto_akcipher_alg(_arg: tfm)->decrypt(req) -> return;
}
//
// crypto_akcipher_sync_encrypt() - Invoke public key encrypt operation
//
// Function invokes the specific public key encrypt operation for a given
// public key algorithm
//
// @tfm:	AKCIPHER tfm handle allocated with crypto_alloc_akcipher()
// @src:	source buffer
// @slen:	source length
// @dst:	destination obuffer
// @dlen:	destination length
//
// Return: zero on success; error code in case of error
//
// crypto_akcipher_sync_decrypt() - Invoke public key decrypt operation
//
// Function invokes the specific public key decrypt operation for a given
// public key algorithm
//
// @tfm:	AKCIPHER tfm handle allocated with crypto_alloc_akcipher()
// @src:	source buffer
// @slen:	source length
// @dst:	destination obuffer
// @dlen:	destination length
//
// Return: Output length on success; error code in case of error
//
// crypto_akcipher_set_pub_key() - Invoke set public key operation
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
// crypto_akcipher_set_priv_key() - Invoke set private key operation
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
