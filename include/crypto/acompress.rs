//! Automatically rewritten from C Header to Rust Module
//! Source: include/crypto/acompress.h
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
// Asynchronous Compression operations
//
// Copyright (c) 2016, Intel Corporation
// Authors: Weigang Li <weigang.li@intel.com>
// Giovanni Cabiddu <giovanni.cabiddu@intel.com>
//

// Set this bit if source is virtual address instead of SG list.
pub const CRYPTO_ACOMP_REQ_SRC_VIRT: c_uint = 0x00000002;
// Set this bit for if virtual address source cannot be used for DMA.
pub const CRYPTO_ACOMP_REQ_SRC_NONDMA: c_uint = 0x00000004;
// Set this bit if destination is virtual address instead of SG list.
pub const CRYPTO_ACOMP_REQ_DST_VIRT: c_uint = 0x00000008;
// Set this bit for if virtual address destination cannot be used for DMA.
pub const CRYPTO_ACOMP_REQ_DST_NONDMA: c_uint = 0x00000010;
// Private flags that should not be touched by the user.

pub const CRYPTO_ACOMP_DST_MAX: c_int = 131072;
pub const MAX_SYNC_COMP_REQSIZE: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acomp_req_chain {
    pub compl: crypto_completion_t,
    pub data: *mut c_void,
    pub ssg: scatterlist,
    pub dsg: scatterlist,
    pub src: *const u8,
    pub sfolio: *mut folio,
}

//
// struct acomp_req - asynchronous (de)compression request
//
// @base:	Common attributes for asynchronous crypto requests
// @src:	Source scatterlist
// @dst:	Destination scatterlist
// @svirt:	Source virtual address
// @dvirt:	Destination virtual address
// @slen:	Size of the input buffer
// @dlen:	Size of the output buffer and number of bytes produced
// @chain:	Private API code data, do not use
// @__ctx:	Start of private context data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acomp_req {
    pub base: crypto_async_request,
    pub src: *mut scatterlist,
    pub svirt: *const u8,
}

//
// struct crypto_acomp - user-instantiated objects which encapsulate
// algorithms and core processing logic
//
// @compress:		Function performs a compress operation
// @decompress:		Function performs a de-compress operation
// @reqsize:		Context size for (de)compression requests
// @fb:			Synchronous fallback tfm
// @base:		Common crypto API algorithm data structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_acomp {
    pub req): *mut *mut int (compress)(struct acomp_req,
    pub req): *mut *mut int (decompress)(struct acomp_req,
    pub reqsize: c_uint,
    pub base: crypto_tfm,
}

//
// DOC: Asynchronous Compression API
//
// The Asynchronous Compression API is used with the algorithms of type
// CRYPTO_ALG_TYPE_ACOMPRESS (listed as type "acomp" in /proc/crypto)
//
// crypto_alloc_acomp() -- allocate ACOMPRESS tfm handle
// @alg_name:	is the cra_name / name or cra_driver_name / driver name of the
// compression algorithm e.g. "deflate"
// @type:	specifies the type of the algorithm
// @mask:	specifies the mask for the algorithm
//
// Allocate a handle for a compression algorithm. The returned struct
// crypto_acomp is the handle that is required for any subsequent
// API invocation for the compression operations.
//
// Return:	allocated handle in case of success; IS_ERR() is true in case
// of an error, PTR_ERR() returns the error code.
//
// crypto_alloc_acomp_node() -- allocate ACOMPRESS tfm handle with desired NUMA node
// @alg_name:	is the cra_name / name or cra_driver_name / driver name of the
// compression algorithm e.g. "deflate"
// @type:	specifies the type of the algorithm
// @mask:	specifies the mask for the algorithm
// @node:	specifies the NUMA node the ZIP hardware belongs to
//
// Allocate a handle for a compression algorithm. Drivers should try to use
// (de)compressors on the specified NUMA node.
// The returned struct crypto_acomp is the handle that is required for any
// subsequent API invocation for the compression operations.
//
// Return:	allocated handle in case of success; IS_ERR() is true in case
// of an error, PTR_ERR() returns the error code.
//
extern "C" {
    pub fn container_of(_arg: alg, comp_alg_common: struct, _arg: base) -> return;
}
extern "C" {
    pub fn container_of(_arg: tfm, crypto_acomp: struct, _arg: base) -> return;
}
extern "C" {
    pub fn __crypto_comp_alg_common(_arg: crypto_acomp_tfm(tfm)->__crt_alg) -> return;
}
extern "C" {
    pub fn __crypto_acomp_tfm(_arg: req->base.tfm) -> return;
}
//
// crypto_free_acomp() -- free ACOMPRESS tfm handle
//
// @tfm:	ACOMPRESS tfm handle allocated with crypto_alloc_acomp()
//
// If @tfm is a NULL or error pointer, this function does nothing.
//
extern "C" {
    pub fn crypto_has_alg(_arg: alg_name, _arg: type, _arg: mask) -> return;
}
extern "C" {
    pub fn crypto_tfm_alg_name(_arg: crypto_acomp_tfm(tfm)) -> return;
}
extern "C" {
    pub fn crypto_tfm_alg_driver_name(_arg: crypto_acomp_tfm(tfm)) -> return;
}
//
// acomp_request_alloc_extra() -- allocates asynchronous (de)compression request
//
// @tfm:	ACOMPRESS tfm handle allocated with crypto_alloc_acomp()
// @extra:	amount of extra memory
// @gfp:	gfp to pass to kzalloc (defaults to GFP_KERNEL)
//
// Return:	allocated handle in case of success or NULL in case of an error
//

//
// acomp_request_alloc_extra() -- allocate acomp request with extra memory
//
// @tfm:	ACOMPRESS tfm handle allocated with crypto_alloc_acomp()
// @extra:	amount of extra memory
// @gfp:	gfp to pass to kzalloc
//
// Return:	allocated handle in case of success or NULL in case of an error
//

extern "C" {
    pub fn crypto_req_on_stack(_arg: &req->base) -> return;
}
//
// acomp_request_free() -- zeroize and free asynchronous (de)compression
// request as well as the output buffer if allocated
// inside the algorithm
//
// @req:	request to free
//
// acomp_request_set_callback() -- Sets an asynchronous callback
//
// Callback will be called when an asynchronous operation on a given
// request is finished.
//
// @req:	request that the callback will be set for
// @flgs:	specify for instance if the operation may backlog
// @cmpl:	callback which will be called
// @data:	private data used by the caller
//
// acomp_request_set_params() -- Sets request parameters
//
// Sets parameters required by an acomp operation
//
// @req:	asynchronous compress request
// @src:	pointer to input buffer scatterlist
// @dst:	pointer to output buffer scatterlist. If this is NULL, the
// acomp layer will allocate the output memory
// @slen:	size of the input buffer
// @dlen:	size of the output buffer. If dst is NULL, this can be used by
// the user to specify the maximum amount of memory to allocate
//
// acomp_request_set_src_sg() -- Sets source scatterlist
//
// Sets source scatterlist required by an acomp operation.
//
// @req:	asynchronous compress request
// @src:	pointer to input buffer scatterlist
// @slen:	size of the input buffer
//
// acomp_request_set_src_dma() -- Sets DMA source virtual address
//
// Sets source virtual address required by an acomp operation.
// The address must be usable for DMA.
//
// @req:	asynchronous compress request
// @src:	virtual address pointer to input buffer
// @slen:	size of the input buffer
//
// acomp_request_set_src_nondma() -- Sets non-DMA source virtual address
//
// Sets source virtual address required by an acomp operation.
// The address can not be used for DMA.
//
// @req:	asynchronous compress request
// @src:	virtual address pointer to input buffer
// @slen:	size of the input buffer
//
// acomp_request_set_src_folio() -- Sets source folio
//
// Sets source folio required by an acomp operation.
//
// @req:	asynchronous compress request
// @folio:	pointer to input folio
// @off:	input folio offset
// @len:	size of the input buffer
//
// acomp_request_set_dst_sg() -- Sets destination scatterlist
//
// Sets destination scatterlist required by an acomp operation.
//
// @req:	asynchronous compress request
// @dst:	pointer to output buffer scatterlist
// @dlen:	size of the output buffer
//
// acomp_request_set_dst_dma() -- Sets DMA destination virtual address
//
// Sets destination virtual address required by an acomp operation.
// The address must be usable for DMA.
//
// @req:	asynchronous compress request
// @dst:	virtual address pointer to output buffer
// @dlen:	size of the output buffer
//
// acomp_request_set_dst_nondma() -- Sets non-DMA destination virtual address
//
// Sets destination virtual address required by an acomp operation.
// The address can not be used for DMA.
//
// @req:	asynchronous compress request
// @dst:	virtual address pointer to output buffer
// @dlen:	size of the output buffer
//
// acomp_request_set_dst_folio() -- Sets destination folio
//
// Sets destination folio required by an acomp operation.
//
// @req:	asynchronous compress request
// @folio:	pointer to input folio
// @off:	input folio offset
// @len:	size of the input buffer
//
// crypto_acomp_compress() -- Invoke asynchronous compress operation
//
// Function invokes the asynchronous compress operation
//
// @req:	asynchronous compress request
//
// Return:	zero on success; error code in case of error
//
extern "C" {
    pub fn crypto_acomp_compress(req: *mut acomp_req) -> c_int;
}
//
// crypto_acomp_decompress() -- Invoke asynchronous decompress operation
//
// Function invokes the asynchronous decompress operation
//
// @req:	asynchronous compress request
//
// Return:	zero on success; error code in case of error
//
extern "C" {
    pub fn crypto_acomp_decompress(req: *mut acomp_req) -> c_int;
}
