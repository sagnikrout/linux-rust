//! Automatically rewritten from C Header to Rust Module
//! Source: include/crypto/skcipher.h
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
// Symmetric key ciphers.
//
// Copyright (c) 2007-2015 Herbert Xu <herbert@gondor.apana.org.au>
//

// Set this bit if the lskcipher operation is a continuation.
pub const CRYPTO_LSKCIPHER_FLAG_CONT: c_uint = 0x00000001;
// Set this bit if the lskcipher operation is final.
pub const CRYPTO_LSKCIPHER_FLAG_FINAL: c_uint = 0x00000002;
// The bit CRYPTO_TFM_REQ_MAY_SLEEP can also be set if needed.
// Set this bit if the skcipher operation is a continuation.
pub const CRYPTO_SKCIPHER_REQ_CONT: c_uint = 0x00000001;
// Set this bit if the skcipher operation is not final.
pub const CRYPTO_SKCIPHER_REQ_NOTFINAL: c_uint = 0x00000002;
//
// struct skcipher_request - Symmetric key cipher request
// @cryptlen: Number of bytes to encrypt or decrypt
// @iv: Initialisation Vector
// @src: Source SG list
// @dst: Destination SG list
// @base: Underlying async request
// @__ctx: Start of private context data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct skcipher_request {
    pub cryptlen: c_uint,
    pub iv: *mut u8,
    pub src: *mut scatterlist,
    pub dst: *mut scatterlist,
    pub base: crypto_async_request,
    pub CRYPTO_MINALIGN_ATTR: *mut *mut void __ctx[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_skcipher {
    pub reqsize: c_uint,
    pub base: crypto_tfm,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_sync_skcipher {
    pub base: crypto_skcipher,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_lskcipher {
    pub base: crypto_tfm,
}

//
// struct skcipher_alg_common - common properties of skcipher_alg
// @min_keysize: Minimum key size supported by the transformation. This is the
// smallest key length supported by this transformation algorithm.
// This must be set to one of the pre-defined values as this is
// not hardware specific. Possible values for this field can be
// found via git grep "_MIN_KEY_SIZE" include/crypto
// @max_keysize: Maximum key size supported by the transformation. This is the
// largest key length supported by this transformation algorithm.
// This must be set to one of the pre-defined values as this is
// not hardware specific. Possible values for this field can be
// found via git grep "_MAX_KEY_SIZE" include/crypto
// @ivsize: IV size applicable for transformation. The consumer must provide an
// IV of exactly that size to perform the encrypt or decrypt operation.
// @chunksize: Equal to the block size except for stream ciphers such as
// CTR where it is set to the underlying block size.
// @statesize: Size of the internal state for the algorithm.
// @base: Definition of a generic crypto algorithm.
//

//
// struct skcipher_alg - symmetric key cipher definition
// @setkey: Set key for the transformation. This function is used to either
// program a supplied key into the hardware or store the key in the
// transformation context for programming it later. Note that this
// function does modify the transformation context. This function can
// be called multiple times during the existence of the transformation
// object, so one must make sure the key is properly reprogrammed into
// the hardware. This function is also responsible for checking the key
// length for validity. In case a software fallback was put in place in
// the @cra_init call, this function might need to use the fallback if
// the algorithm doesn't support all of the key sizes.
// @encrypt: Encrypt a scatterlist of blocks. This function is used to encrypt
// the supplied scatterlist containing the blocks of data. The crypto
// API consumer is responsible for aligning the entries of the
// scatterlist properly and making sure the chunks are correctly
// sized. In case a software fallback was put in place in the
// @cra_init call, this function might need to use the fallback if
// the algorithm doesn't support all of the key sizes. In case the
// key was stored in transformation context, the key might need to be
// re-programmed into the hardware in this function. This function
// shall not modify the transformation context, as this function may
// be called in parallel with the same transformation object.
// @decrypt: Decrypt a single block. This is a reverse counterpart to @encrypt
// and the conditions are exactly the same.
// @export: Export partial state of the transformation. This function dumps the
// entire state of the ongoing transformation into a provided block of
// data so it can be @import 'ed back later on. This is useful in case
// you want to save partial result of the transformation after
// processing certain amount of data and reload this partial result
// multiple times later on for multiple re-use. No data processing
// happens at this point.
// @import: Import partial state of the transformation. This function loads the
// entire state of the ongoing transformation from a provided block of
// data so the transformation can continue from this point onward. No
// data processing happens at this point.
// @init: Initialize the cryptographic transformation object. This function
// is used to initialize the cryptographic transformation object.
// This function is called only once at the instantiation time, right
// after the transformation context was allocated. In case the
// cryptographic hardware has some special requirements which need to
// be handled by software, this function shall check for the precise
// requirement of the transformation and put any software fallbacks
// in place.
// @exit: Deinitialize the cryptographic transformation object. This is a
// counterpart to @init, used to remove various changes set in
// @init.
// @walksize: Equal to the chunk size except in cases where the algorithm is
// considerably more efficient if it can operate on multiple chunks
// in parallel. Should be a multiple of chunksize.
// @co: see struct skcipher_alg_common
// @SKCIPHER_ALG_COMMON: see struct skcipher_alg_common
//
// All fields except @ivsize are mandatory and must be filled.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct skcipher_alg {
    pub keylen): c_uint,
    pub req): *mut *mut int (encrypt)(struct skcipher_request,
    pub req): *mut *mut int (decrypt)(struct skcipher_request,
    pub out): *mut *mut *mut int (export)(struct skcipher_request req, void,
    pub in): *const *const *const int (import)(struct skcipher_request req, void,
    pub tfm): *mut *mut int (init)(struct crypto_skcipher,
    pub tfm): *mut *mut void (exit)(struct crypto_skcipher,
    pub walksize: c_uint,
    pub SKCIPHER_ALG_COMMON: struct,
    pub co: skcipher_alg_common,
}

//
// struct lskcipher_alg - linear symmetric key cipher definition
// @setkey: Set key for the transformation. This function is used to either
// program a supplied key into the hardware or store the key in the
// transformation context for programming it later. Note that this
// function does modify the transformation context. This function can
// be called multiple times during the existence of the transformation
// object, so one must make sure the key is properly reprogrammed into
// the hardware. This function is also responsible for checking the key
// length for validity. In case a software fallback was put in place in
// the @cra_init call, this function might need to use the fallback if
// the algorithm doesn't support all of the key sizes.
// @encrypt: Encrypt a number of bytes. This function is used to encrypt
// the supplied data.  This function shall not modify
// the transformation context, as this function may be called
// in parallel with the same transformation object.  Data
// may be left over if length is not a multiple of blocks
// and there is more to come (final == false).  The number of
// left-over bytes should be returned in case of success.
// The siv field shall be as long as ivsize + statesize with
// the IV placed at the front.  The state will be used by the
// algorithm internally.
// @decrypt: Decrypt a number of bytes. This is a reverse counterpart to
// @encrypt and the conditions are exactly the same.
// @init: Initialize the cryptographic transformation object. This function
// is used to initialize the cryptographic transformation object.
// This function is called only once at the instantiation time, right
// after the transformation context was allocated.
// @exit: Deinitialize the cryptographic transformation object. This is a
// counterpart to @init, used to remove various changes set in
// @init.
// @co: see struct skcipher_alg_common
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lskcipher_alg {
    pub keylen): c_uint,
    pub flags): *mut *mut *mut u8 dst, unsigned len, u8 siv, u32,
    pub flags): *mut *mut *mut u8 dst, unsigned len, u8 siv, u32,
    pub tfm): *mut *mut int (init)(struct crypto_lskcipher,
    pub tfm): *mut *mut void (exit)(struct crypto_lskcipher,
    pub co: skcipher_alg_common,
}

pub const MAX_SYNC_SKCIPHER_REQSIZE: c_int = 384;
//
// This performs a type-check against the "_tfm" argument to make sure
// all users have the correct skcipher tfm for doing on-stack requests.
//

//
// DOC: Symmetric Key Cipher API
//
// Symmetric key cipher API is used with the ciphers of type
// CRYPTO_ALG_TYPE_SKCIPHER (listed as type "skcipher" in /proc/crypto).
//
// Asynchronous cipher operations imply that the function invocation for a
// cipher request returns immediately before the completion of the operation.
// The cipher request is scheduled as a separate kernel thread and therefore
// load-balanced on the different CPUs via the process scheduler. To allow
// the kernel crypto API to inform the caller about the completion of a cipher
// request, the caller must provide a callback function. That function is
// invoked with the cipher handle when the request completes.
//
// To support the asynchronous operation, additional information than just the
// cipher handle must be supplied to the kernel crypto API. That additional
// information is given by filling in the skcipher_request data structure.
//
// For the symmetric key cipher API, the state is maintained with the tfm
// cipher handle. A single tfm can be used across multiple calls and in
// parallel. For asynchronous block cipher calls, context data supplied and
// only used by the caller can be referenced the request data structure in
// addition to the IV used for the cipher request. The maintenance of such
// state information would be important for a crypto driver implementer to
// have, because when calling the callback function upon completion of the
// cipher operation, that callback function may need some information about
// which operation just finished if it invoked multiple in parallel. This
// state information is unused by the kernel crypto API.
//
extern "C" {
    pub fn container_of(_arg: tfm, crypto_skcipher: struct, _arg: base) -> return;
}
//
// crypto_alloc_skcipher() - allocate symmetric key cipher handle
// @alg_name: is the cra_name / name or cra_driver_name / driver name of the
// skcipher cipher
// @type: specifies the type of the cipher
// @mask: specifies the mask for the cipher
//
// Allocate a cipher handle for an skcipher. The returned struct
// crypto_skcipher is the cipher handle that is required for any subsequent
// API invocation for that skcipher.
//
// Return: allocated cipher handle in case of success; IS_ERR() is true in case
// of an error, PTR_ERR() returns the error code.
//
// crypto_alloc_lskcipher() - allocate linear symmetric key cipher handle
// @alg_name: is the cra_name / name or cra_driver_name / driver name of the
// lskcipher
// @type: specifies the type of the cipher
// @mask: specifies the mask for the cipher
//
// Allocate a cipher handle for an lskcipher. The returned struct
// crypto_lskcipher is the cipher handle that is required for any subsequent
// API invocation for that lskcipher.
//
// Return: allocated cipher handle in case of success; IS_ERR() is true in case
// of an error, PTR_ERR() returns the error code.
//
extern "C" {
    pub fn crypto_skcipher_tfm(_arg: &tfm->base) -> return;
}
//
// crypto_free_skcipher() - zeroize and free cipher handle
// @tfm: cipher handle to be freed
//
// If @tfm is a NULL or error pointer, this function does nothing.
//
// crypto_free_lskcipher() - zeroize and free cipher handle
// @tfm: cipher handle to be freed
//
// If @tfm is a NULL or error pointer, this function does nothing.
//
// crypto_has_skcipher() - Search for the availability of an skcipher.
// @alg_name: is the cra_name / name or cra_driver_name / driver name of the
// skcipher
// @type: specifies the type of the skcipher
// @mask: specifies the mask for the skcipher
//
// Return: true when the skcipher is known to the kernel crypto API; false
// otherwise
//
extern "C" {
    pub fn crypto_has_skcipher(alg_name: *const c_char, type: u32, mask: u32) -> c_int;
}
extern "C" {
    pub fn crypto_tfm_alg_driver_name(_arg: crypto_skcipher_tfm(tfm)) -> return;
}
extern "C" {
    pub fn crypto_tfm_alg_driver_name(_arg: crypto_lskcipher_tfm(tfm)) -> return;
}
//
// crypto_skcipher_ivsize() - obtain IV size
// @tfm: cipher handle
//
// The size of the IV for the skcipher referenced by the cipher handle is
// returned. This IV size may be zero if the cipher does not need an IV.
//
// Return: IV size in bytes
//
extern "C" {
    pub fn crypto_skcipher_ivsize(_arg: &tfm->base) -> return;
}
//
// crypto_lskcipher_ivsize() - obtain IV size
// @tfm: cipher handle
//
// The size of the IV for the lskcipher referenced by the cipher handle is
// returned. This IV size may be zero if the cipher does not need an IV.
//
// Return: IV size in bytes
//
// crypto_skcipher_blocksize() - obtain block size of cipher
// @tfm: cipher handle
//
// The block size for the skcipher referenced with the cipher handle is
// returned. The caller may use that information to allocate appropriate
// memory for the data returned by the encryption or decryption operation
//
// Return: block size of cipher
//
extern "C" {
    pub fn crypto_tfm_alg_blocksize(_arg: crypto_skcipher_tfm(tfm)) -> return;
}
//
// crypto_lskcipher_blocksize() - obtain block size of cipher
// @tfm: cipher handle
//
// The block size for the lskcipher referenced with the cipher handle is
// returned. The caller may use that information to allocate appropriate
// memory for the data returned by the encryption or decryption operation
//
// Return: block size of cipher
//
extern "C" {
    pub fn crypto_tfm_alg_blocksize(_arg: crypto_lskcipher_tfm(tfm)) -> return;
}
//
// crypto_skcipher_chunksize() - obtain chunk size
// @tfm: cipher handle
//
// The block size is set to one for ciphers such as CTR.  However,
// you still need to provide incremental updates in multiples of
// the underlying block size as the IV does not have sub-block
// granularity.  This is known in this API as the chunk size.
//
// Return: chunk size in bytes
//
// crypto_lskcipher_chunksize() - obtain chunk size
// @tfm: cipher handle
//
// The block size is set to one for ciphers such as CTR.  However,
// you still need to provide incremental updates in multiples of
// the underlying block size as the IV does not have sub-block
// granularity.  This is known in this API as the chunk size.
//
// Return: chunk size in bytes
//
// crypto_skcipher_statesize() - obtain state size
// @tfm: cipher handle
//
// Some algorithms cannot be chained with the IV alone.  They carry
// internal state which must be replicated if data is to be processed
// incrementally.  The size of that state can be obtained with this
// function.
//
// Return: state size in bytes
//
// crypto_lskcipher_statesize() - obtain state size
// @tfm: cipher handle
//
// Some algorithms cannot be chained with the IV alone.  They carry
// internal state which must be replicated if data is to be processed
// incrementally.  The size of that state can be obtained with this
// function.
//
// Return: state size in bytes
//
extern "C" {
    pub fn crypto_skcipher_blocksize(_arg: &tfm->base) -> return;
}
extern "C" {
    pub fn crypto_tfm_alg_alignmask(_arg: crypto_skcipher_tfm(tfm)) -> return;
}
extern "C" {
    pub fn crypto_tfm_alg_alignmask(_arg: crypto_lskcipher_tfm(tfm)) -> return;
}
extern "C" {
    pub fn crypto_tfm_get_flags(_arg: crypto_skcipher_tfm(tfm)) -> return;
}
extern "C" {
    pub fn crypto_skcipher_get_flags(_arg: &tfm->base) -> return;
}
extern "C" {
    pub fn crypto_tfm_get_flags(_arg: crypto_lskcipher_tfm(tfm)) -> return;
}
//
// crypto_skcipher_setkey() - set key for cipher
// @tfm: cipher handle
// @key: buffer holding the key
// @keylen: length of the key in bytes
//
// The caller provided key is set for the skcipher referenced by the cipher
// handle.
//
// Note, the key length determines the cipher type. Many block ciphers implement
// different cipher modes depending on the key size, such as AES-128 vs AES-192
// vs. AES-256. When providing a 16 byte key for an AES cipher handle, AES-128
// is performed.
//
// Return: 0 if the setting of the key was successful; < 0 if an error occurred
//
extern "C" {
    pub fn crypto_skcipher_setkey(_arg: &tfm->base, _arg: key, _arg: keylen) -> return;
}
//
// crypto_lskcipher_setkey() - set key for cipher
// @tfm: cipher handle
// @key: buffer holding the key
// @keylen: length of the key in bytes
//
// The caller provided key is set for the lskcipher referenced by the cipher
// handle.
//
// Note, the key length determines the cipher type. Many block ciphers implement
// different cipher modes depending on the key size, such as AES-128 vs AES-192
// vs. AES-256. When providing a 16 byte key for an AES cipher handle, AES-128
// is performed.
//
// Return: 0 if the setting of the key was successful; < 0 if an error occurred
//
// crypto_skcipher_reqtfm() - obtain cipher handle from request
// @req: skcipher_request out of which the cipher handle is to be obtained
//
// Return the crypto_skcipher handle when furnishing an skcipher_request
// data structure.
//
// Return: crypto_skcipher handle
//
extern "C" {
    pub fn __crypto_skcipher_cast(_arg: req->base.tfm) -> return;
}
extern "C" {
    pub fn container_of(_arg: tfm, crypto_sync_skcipher: struct, _arg: base) -> return;
}
//
// crypto_skcipher_encrypt() - encrypt plaintext
// @req: reference to the skcipher_request handle that holds all information
// needed to perform the cipher operation
//
// Encrypt plaintext data using the skcipher_request handle. That data
// structure and how it is filled with data is discussed with the
// skcipher_request_* functions.
//
// Return: 0 if the cipher operation was successful; < 0 if an error occurred
//
extern "C" {
    pub fn crypto_skcipher_encrypt(req: *mut skcipher_request) -> c_int;
}
//
// crypto_skcipher_decrypt() - decrypt ciphertext
// @req: reference to the skcipher_request handle that holds all information
// needed to perform the cipher operation
//
// Decrypt ciphertext data using the skcipher_request handle. That data
// structure and how it is filled with data is discussed with the
// skcipher_request_* functions.
//
// Return: 0 if the cipher operation was successful; < 0 if an error occurred
//
extern "C" {
    pub fn crypto_skcipher_decrypt(req: *mut skcipher_request) -> c_int;
}
//
// crypto_skcipher_export() - export partial state
// @req: reference to the skcipher_request handle that holds all information
// needed to perform the operation
// @out: output buffer of sufficient size that can hold the state
//
// Export partial state of the transformation. This function dumps the
// entire state of the ongoing transformation into a provided block of
// data so it can be @import 'ed back later on. This is useful in case
// you want to save partial result of the transformation after
// processing certain amount of data and reload this partial result
// multiple times later on for multiple re-use. No data processing
// happens at this point.
//
// Return: 0 if the cipher operation was successful; < 0 if an error occurred
//
extern "C" {
    pub fn crypto_skcipher_export(req: *mut skcipher_request, out: *mut c_void) -> c_int;
}
//
// crypto_skcipher_import() - import partial state
// @req: reference to the skcipher_request handle that holds all information
// needed to perform the operation
// @in: buffer holding the state
//
// Import partial state of the transformation. This function loads the
// entire state of the ongoing transformation from a provided block of
// data so the transformation can continue from this point onward. No
// data processing happens at this point.
//
// Return: 0 if the cipher operation was successful; < 0 if an error occurred
//
extern "C" {
    pub fn crypto_skcipher_import(req: *mut skcipher_request, in: *const c_void) -> c_int;
}
//
// crypto_lskcipher_encrypt() - encrypt plaintext
// @tfm: lskcipher handle
// @src: source buffer
// @dst: destination buffer
// @len: number of bytes to process
// @siv: IV + state for the cipher operation.  The length of the IV must
// comply with the IV size defined by crypto_lskcipher_ivsize.  The
// IV is then followed with a buffer with the length as specified by
// crypto_lskcipher_statesize.
// Encrypt plaintext data using the lskcipher handle.
//
// Return: >=0 if the cipher operation was successful, if positive
// then this many bytes have been left unprocessed;
// < 0 if an error occurred
//
// crypto_lskcipher_decrypt() - decrypt ciphertext
// @tfm: lskcipher handle
// @src: source buffer
// @dst: destination buffer
// @len: number of bytes to process
// @siv: IV + state for the cipher operation.  The length of the IV must
// comply with the IV size defined by crypto_lskcipher_ivsize.  The
// IV is then followed with a buffer with the length as specified by
// crypto_lskcipher_statesize.
//
// Decrypt ciphertext data using the lskcipher handle.
//
// Return: >=0 if the cipher operation was successful, if positive
// then this many bytes have been left unprocessed;
// < 0 if an error occurred
//
// DOC: Symmetric Key Cipher Request Handle
//
// The skcipher_request data structure contains all pointers to data
// required for the symmetric key cipher operation. This includes the cipher
// handle (which can be used by multiple skcipher_request instances), pointer
// to plaintext and ciphertext, asynchronous callback function, etc. It acts
// as a handle to the skcipher_request_* API calls in a similar way as
// skcipher handle to the crypto_skcipher_* API calls.
//
// crypto_skcipher_reqsize() - obtain size of the request data structure
// @tfm: cipher handle
//
// Return: number of bytes
//
// skcipher_request_set_tfm() - update cipher handle reference in request
// @req: request handle to be modified
// @tfm: cipher handle that shall be added to the request handle
//
// Allow the caller to replace the existing skcipher handle in the request
// data structure with a different one.
//
extern "C" {
    pub fn container_of(_arg: req, skcipher_request: struct, _arg: base) -> return;
}
//
// skcipher_request_alloc() - allocate request data structure
// @tfm: cipher handle to be registered with the request
// @gfp: memory allocation flag that is handed to kmalloc by the API call.
//
// Allocate the request data structure that must be used with the skcipher
// encrypt and decrypt API calls. During the allocation, the provided skcipher
// handle is registered in the request data structure.
//
// Return: allocated request handle in case of success, or NULL if out of memory
//

//
// skcipher_request_free() - zeroize and free request data structure
// @req: request data structure cipher handle to be freed
//
// skcipher_request_set_callback() - set asynchronous callback function
// @req: request handle
// @flags: specify zero or an ORing of the flags
// CRYPTO_TFM_REQ_MAY_BACKLOG the request queue may back log and
// increase the wait queue beyond the initial maximum size;
// CRYPTO_TFM_REQ_MAY_SLEEP the request processing may sleep
// @compl: callback function pointer to be registered with the request handle
// @data: The data pointer refers to memory that is not used by the kernel
// crypto API, but provided to the callback function for it to use. Here,
// the caller can provide a reference to memory the callback function can
// operate on. As the callback function is invoked asynchronously to the
// related functionality, it may need to access data structures of the
// related functionality which can be referenced using this pointer. The
// callback function can access the memory via the "data" field in the
// crypto_async_request data structure provided to the callback function.
//
// This function allows setting the callback function that is triggered once the
// cipher operation completes.
//
// The callback function is registered with the skcipher_request handle and
// must comply with the following template::
//
// void callback_function(struct crypto_async_request *req, int error)
//
// skcipher_request_set_crypt() - set data buffers
// @req: request handle
// @src: source scatter / gather list
// @dst: destination scatter / gather list
// @cryptlen: number of bytes to process from @src
// @iv: IV for the cipher operation which must comply with the IV size defined
// by crypto_skcipher_ivsize
//
// This function allows setting of the source data and destination data
// scatter / gather lists.
//
// For encryption, the source is treated as the plaintext and the
// destination is the ciphertext. For a decryption operation, the use is
// reversed - the source is the ciphertext and the destination is the plaintext.
//
