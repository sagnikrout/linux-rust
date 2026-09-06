//! Automatically rewritten from C Header to Rust Module
//! Source: include/crypto/kpp.h
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
// Key-agreement Protocol Primitives (KPP)
//
// Copyright (c) 2016, Intel Corporation
// Authors: Salvatore Benedetto <salvatore.benedetto@intel.com>
//

//
// struct kpp_request
//
// @base:	Common attributes for async crypto requests
// @src:	Source data
// @dst:	Destination data
// @src_len:	Size of the input buffer
// @dst_len:	Size of the output buffer. It needs to be at least
// as big as the expected result depending	on the operation
// After operation it will be updated with the actual size of the
// result. In case of error where the dst sgl size was insufficient,
// it will be updated to the size required for the operation.
// @__ctx:	Start of private context data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kpp_request {
    pub base: crypto_async_request,
    pub src: *mut scatterlist,
    pub dst: *mut scatterlist,
    pub src_len: c_uint,
    pub dst_len: c_uint,
    pub CRYPTO_MINALIGN_ATTR: *mut *mut void __ctx[],
}

//
// struct crypto_kpp - user-instantiated object which encapsulate
// algorithms and core processing logic
//
// @reqsize:		Request context size required by algorithm
// implementation
// @base:	Common crypto API algorithm data structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_kpp {
    pub reqsize: c_uint,
    pub base: crypto_tfm,
}

//
// struct kpp_alg - generic key-agreement protocol primitives
//
// @set_secret:		Function invokes the protocol specific function to
// store the secret private key along with parameters.
// The implementation knows how to decode the buffer
// @generate_public_key: Function generate the public key to be sent to the
// counterpart. In case of error, where output is not big
// enough req->dst_len will be updated to the size
// required
// @compute_shared_secret: Function compute the shared secret as defined by
// the algorithm. The result is given back to the user.
// In case of error, where output is not big enough,
// req->dst_len will be updated to the size required
// @max_size:		Function returns the size of the output buffer
// @init:		Initialize the object. This is called only once at
// instantiation time. In case the cryptographic hardware
// needs to be initialized. Software fallback should be
// put in place here.
// @exit:		Undo everything @init did.
//
// @base:		Common crypto API algorithm data structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kpp_alg {
    pub len): c_uint,
    pub req): *mut *mut int (generate_public_key)(struct kpp_request,
    pub req): *mut *mut int (compute_shared_secret)(struct kpp_request,
    pub tfm): *mut *mut unsigned int (max_size)(struct crypto_kpp,
    pub tfm): *mut *mut int (init)(struct crypto_kpp,
    pub tfm): *mut *mut void (exit)(struct crypto_kpp,
    pub base: crypto_alg,
}

//
// DOC: Generic Key-agreement Protocol Primitives API
//
// The KPP API is used with the algorithm type
// CRYPTO_ALG_TYPE_KPP (listed as type "kpp" in /proc/crypto)
//
// crypto_alloc_kpp() - allocate KPP tfm handle
// @alg_name: is the name of the kpp algorithm (e.g. "dh", "ecdh")
// @type: specifies the type of the algorithm
// @mask: specifies the mask for the algorithm
//
// Allocate a handle for kpp algorithm. The returned struct crypto_kpp
// is required for any following API invocation
//
// Return: allocated handle in case of success; IS_ERR() is true in case of
// an error, PTR_ERR() returns the error code.
//
extern "C" {
    pub fn crypto_has_kpp(alg_name: *const c_char, type: u32, mask: u32) -> c_int;
}
extern "C" {
    pub fn container_of(_arg: alg, kpp_alg: struct, _arg: base) -> return;
}
extern "C" {
    pub fn container_of(_arg: tfm, crypto_kpp: struct, _arg: base) -> return;
}
extern "C" {
    pub fn __crypto_kpp_alg(_arg: crypto_kpp_tfm(tfm)->__crt_alg) -> return;
}
extern "C" {
    pub fn __crypto_kpp_tfm(_arg: req->base.tfm) -> return;
}
extern "C" {
    pub fn crypto_tfm_get_flags(_arg: crypto_kpp_tfm(tfm)) -> return;
}
//
// crypto_free_kpp() - free KPP tfm handle
//
// @tfm: KPP tfm handle allocated with crypto_alloc_kpp()
//
// If @tfm is a NULL or error pointer, this function does nothing.
//
// kpp_request_alloc() - allocates kpp request
//
// @tfm:	KPP tfm handle allocated with crypto_alloc_kpp()
// @gfp:	allocation flags
//
// Return: allocated handle in case of success or NULL in case of an error.
//
// kpp_request_free() - zeroize and free kpp request
//
// @req:	request to free
//
// kpp_request_set_callback() - Sets an asynchronous callback.
//
// Callback will be called when an asynchronous operation on a given
// request is finished.
//
// @req:	request that the callback will be set for
// @flgs:	specify for instance if the operation may backlog
// @cmpl:	callback which will be called
// @data:	private data used by the caller
//
// kpp_request_set_input() - Sets input buffer
//
// Sets parameters required by generate_public_key
//
// @req:	kpp request
// @input:	ptr to input scatter list
// @input_len:	size of the input scatter list
//
// kpp_request_set_output() - Sets output buffer
//
// Sets parameters required by kpp operation
//
// @req:	kpp request
// @output:	ptr to output scatter list
// @output_len:	size of the output scatter list
//
// struct kpp_secret - small header for packing secret buffer
//
// @type:	define type of secret. Each kpp type will define its own
// @len:	specify the len of the secret, include the header, that
// follows the struct
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kpp_secret {
    pub type: c_ushort,
    pub len: c_ushort,
}

//
// crypto_kpp_set_secret() - Invoke kpp operation
//
// Function invokes the specific kpp operation for a given alg.
//
// @tfm:	tfm handle
// @buffer:	Buffer holding the packet representation of the private
// key. The structure of the packet key depends on the particular
// KPP implementation. Packing and unpacking helpers are provided
// for ECDH and DH (see the respective header files for those
// implementations).
// @len:	Length of the packet private key buffer.
//
// Return: zero on success; error code in case of error
//
extern "C" {
    pub fn crypto_kpp_alg(_arg: tfm)->set_secret(tfm, _arg: buffer, _arg: len) -> return;
}
//
// crypto_kpp_generate_public_key() - Invoke kpp operation
//
// Function invokes the specific kpp operation for generating the public part
// for a given kpp algorithm.
//
// To generate a private key, the caller should use a random number generator.
// The output of the requested length serves as the private key.
//
// @req:	kpp key request
//
// Return: zero on success; error code in case of error
//
extern "C" {
    pub fn crypto_kpp_alg(_arg: tfm)->generate_public_key(req) -> return;
}
//
// crypto_kpp_compute_shared_secret() - Invoke kpp operation
//
// Function invokes the specific kpp operation for computing the shared secret
// for a given kpp algorithm.
//
// @req:	kpp key request
//
// Return: zero on success; error code in case of error
//
extern "C" {
    pub fn crypto_kpp_alg(_arg: tfm)->compute_shared_secret(req) -> return;
}
//
// crypto_kpp_maxsize() - Get len for output buffer
//
// Function returns the output buffer size required for a given key.
// Function assumes that the key is already set in the transformation. If this
// function is called without a setkey or with a failed setkey, you will end up
// in a NULL dereference.
//
// @tfm:	KPP tfm handle allocated with crypto_alloc_kpp()
//
