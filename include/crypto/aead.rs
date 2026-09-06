//! Automatically rewritten from C Header to Rust Module
//! Source: include/crypto/aead.h
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
// AEAD: Authenticated Encryption with Associated Data
//
// Copyright (c) 2007-2015 Herbert Xu <herbert@gondor.apana.org.au>
//

//
// DOC: Authenticated Encryption With Associated Data (AEAD) Cipher API
//
// The AEAD cipher API is used with the ciphers of type CRYPTO_ALG_TYPE_AEAD
// (listed as type "aead" in /proc/crypto)
//
// The most prominent examples for this type of encryption is GCM and CCM.
// However, the kernel supports other types of AEAD ciphers which are defined
// with the following cipher string:
//
// authenc(keyed message digest, block cipher)
//
// For example: authenc(hmac(sha256), cbc(aes))
//
// The example code provided for the symmetric key cipher operation applies
// here as well. Naturally all *skcipher* symbols must be exchanged the *aead
// pendants discussed in the following. In addition, for the AEAD operation,
// the aead_request_set_ad function must be used to set the pointer to the
// associated data memory location before performing the encryption or
// decryption operation. Another deviation from the asynchronous block cipher
// operation is that the caller should explicitly check for -EBADMSG of the
// crypto_aead_decrypt. That error indicates an authentication error, i.e.
// a breach in the integrity of the message. In essence, that -EBADMSG error
// code is the key bonus an AEAD cipher has over "standard" block chaining
// modes.
//
// Memory Structure:
//
// The source scatterlist must contain the concatenation of
// associated data || plaintext or ciphertext.
//
// The destination scatterlist has the same layout, except that the plaintext
// (resp. ciphertext) will grow (resp. shrink) by the authentication tag size
// during encryption (resp. decryption). The authentication tag is generated
// during the encryption operation and appended to the ciphertext. During
// decryption, the authentication tag is consumed along with the ciphertext and
// used to verify the integrity of the plaintext and the associated data.
//
// In-place encryption/decryption is enabled by using the same scatterlist
// pointer for both the source and destination.
//
// Even in the out-of-place case, space must be reserved in the destination for
// the associated data, even though it won't be written to.  This makes the
// in-place and out-of-place cases more consistent.  It is permissible for the
// "destination" associated data to alias the "source" associated data.
//
// As with the other scatterlist crypto APIs, zero-length scatterlist elements
// are not allowed in the used part of the scatterlist.  Thus, if there is no
// associated data, the first element must point to the plaintext/ciphertext.
//
// To meet the needs of IPsec, a special quirk applies to rfc4106, rfc4309,
// rfc4543, and rfc7539esp ciphers.  For these ciphers, the final 'ivsize' bytes
// of the associated data buffer must contain a second copy of the IV.  This is
// in addition to the copy passed to aead_request_set_crypt().  These two IV
// copies must not differ; different implementations of the same algorithm may
// behave differently in that case.  Note that the algorithm might not actually
// treat the IV as associated data; nevertheless the length passed to
// aead_request_set_ad() must include it.
//
// struct aead_request - AEAD request
// @base: Common attributes for async crypto requests
// @assoclen: Length in bytes of associated data for authentication
// @cryptlen: Length of data to be encrypted or decrypted
// @iv: Initialisation vector
// @src: Source data
// @dst: Destination data
// @__ctx: Start of private context data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aead_request {
    pub base: crypto_async_request,
    pub assoclen: c_uint,
    pub cryptlen: c_uint,
    pub iv: *mut u8,
    pub src: *mut scatterlist,
    pub dst: *mut scatterlist,
    pub CRYPTO_MINALIGN_ATTR: *mut *mut void __ctx[],
}

//
// struct aead_alg - AEAD cipher definition
// @maxauthsize: Set the maximum authentication tag size supported by the
// transformation. A transformation may support smaller tag sizes.
// As the authentication tag is a message digest to ensure the
// integrity of the encrypted data, a consumer typically wants the
// largest authentication tag possible as defined by this
// variable.
// @setauthsize: Set authentication size for the AEAD transformation. This
// function is used to specify the consumer requested size of the
// authentication tag to be either generated by the transformation
// during encryption or the size of the authentication tag to be
// supplied during the decryption operation. This function is also
// responsible for checking the authentication tag size for
// validity.
// @setkey: see struct skcipher_alg
// @encrypt: see struct skcipher_alg
// @decrypt: see struct skcipher_alg
// @ivsize: see struct skcipher_alg
// @chunksize: see struct skcipher_alg
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
// @base: Definition of a generic crypto cipher algorithm.
//
// All fields except @ivsize is mandatory and must be filled.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aead_alg {
    pub keylen): c_uint,
    pub authsize): *mut *mut *mut int (setauthsize)(struct crypto_aead tfm, unsigned int,
    pub req): *mut *mut int (encrypt)(struct aead_request,
    pub req): *mut *mut int (decrypt)(struct aead_request,
    pub tfm): *mut *mut int (init)(struct crypto_aead,
    pub tfm): *mut *mut void (exit)(struct crypto_aead,
    pub ivsize: c_uint,
    pub maxauthsize: c_uint,
    pub chunksize: c_uint,
    pub base: crypto_alg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_aead {
    pub authsize: c_uint,
    pub reqsize: c_uint,
    pub base: crypto_tfm,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_sync_aead {
    pub base: crypto_aead,
}

pub const MAX_SYNC_AEAD_REQSIZE: c_int = 384;

extern "C" {
    pub fn container_of(_arg: tfm, crypto_aead: struct, _arg: base) -> return;
}
//
// crypto_alloc_aead() - allocate AEAD cipher handle
// @alg_name: is the cra_name / name or cra_driver_name / driver name of the
// AEAD cipher
// @type: specifies the type of the cipher
// @mask: specifies the mask for the cipher
//
// Allocate a cipher handle for an AEAD. The returned struct
// crypto_aead is the cipher handle that is required for any subsequent
// API invocation for that AEAD.
//
// Return: allocated cipher handle in case of success; IS_ERR() is true in case
// of an error, PTR_ERR() returns the error code.
//
extern "C" {
    pub fn crypto_aead_tfm(_arg: &tfm->base) -> return;
}
//
// crypto_free_aead() - zeroize and free aead handle
// @tfm: cipher handle to be freed
//
// If @tfm is a NULL or error pointer, this function does nothing.
//
// crypto_has_aead() - Search for the availability of an aead.
// @alg_name: is the cra_name / name or cra_driver_name / driver name of the
// aead
// @type: specifies the type of the aead
// @mask: specifies the mask for the aead
//
// Return: true when the aead is known to the kernel crypto API; false
// otherwise
//
extern "C" {
    pub fn crypto_has_aead(alg_name: *const c_char, type: u32, mask: u32) -> c_int;
}
extern "C" {
    pub fn crypto_tfm_alg_driver_name(_arg: crypto_aead_tfm(tfm)) -> return;
}
//
// crypto_aead_ivsize() - obtain IV size
// @tfm: cipher handle
//
// The size of the IV for the aead referenced by the cipher handle is
// returned. This IV size may be zero if the cipher does not need an IV.
//
// Return: IV size in bytes
//
extern "C" {
    pub fn crypto_aead_alg_ivsize(_arg: crypto_aead_alg(tfm)) -> return;
}
extern "C" {
    pub fn crypto_aead_ivsize(_arg: &tfm->base) -> return;
}
//
// crypto_aead_authsize() - obtain maximum authentication data size
// @tfm: cipher handle
//
// The maximum size of the authentication data for the AEAD cipher referenced
// by the AEAD cipher handle is returned. The authentication data size may be
// zero if the cipher implements a hard-coded maximum.
//
// The authentication data may also be known as "tag value".
//
// Return: authentication data size / tag size in bytes
//
extern "C" {
    pub fn crypto_aead_authsize(_arg: &tfm->base) -> return;
}
extern "C" {
    pub fn crypto_aead_alg_maxauthsize(_arg: crypto_aead_alg(aead)) -> return;
}
extern "C" {
    pub fn crypto_aead_maxauthsize(_arg: &tfm->base) -> return;
}
//
// crypto_aead_blocksize() - obtain block size of cipher
// @tfm: cipher handle
//
// The block size for the AEAD referenced with the cipher handle is returned.
// The caller may use that information to allocate appropriate memory for the
// data returned by the encryption or decryption operation
//
// Return: block size of cipher
//
extern "C" {
    pub fn crypto_tfm_alg_blocksize(_arg: crypto_aead_tfm(tfm)) -> return;
}
extern "C" {
    pub fn crypto_aead_blocksize(_arg: &tfm->base) -> return;
}
extern "C" {
    pub fn crypto_tfm_alg_alignmask(_arg: crypto_aead_tfm(tfm)) -> return;
}
extern "C" {
    pub fn crypto_tfm_get_flags(_arg: crypto_aead_tfm(tfm)) -> return;
}
extern "C" {
    pub fn crypto_aead_get_flags(_arg: &tfm->base) -> return;
}
//
// crypto_aead_setkey() - set key for cipher
// @tfm: cipher handle
// @key: buffer holding the key
// @keylen: length of the key in bytes
//
// The caller provided key is set for the AEAD referenced by the cipher
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
    pub fn crypto_aead_setkey(_arg: &tfm->base, _arg: key, _arg: keylen) -> return;
}
//
// crypto_aead_setauthsize() - set authentication data size
// @tfm: cipher handle
// @authsize: size of the authentication data / tag in bytes
//
// Set the authentication data size / tag size. AEAD requires an authentication
// tag (or MAC) in addition to the associated data.
//
// Return: 0 if the setting of the key was successful; < 0 if an error occurred
//
extern "C" {
    pub fn crypto_aead_setauthsize(tfm: *mut crypto_aead, authsize: c_uint) -> c_int;
}
extern "C" {
    pub fn crypto_aead_setauthsize(_arg: &tfm->base, _arg: authsize) -> return;
}
extern "C" {
    pub fn __crypto_aead_cast(_arg: req->base.tfm) -> return;
}
extern "C" {
    pub fn container_of(_arg: tfm, crypto_sync_aead: struct, _arg: base) -> return;
}
//
// crypto_aead_encrypt() - encrypt plaintext
// @req: reference to the aead_request handle that holds all information
// needed to perform the cipher operation
//
// Encrypt plaintext data using the aead_request handle. That data structure
// and how it is filled with data is discussed with the aead_request_
// functions.
//
// IMPORTANT NOTE The encryption operation creates the authentication data
// tag. That data is concatenated with the created ciphertext.
// The ciphertext memory size is therefore the given number of
// block cipher blocks + the size defined by the
// crypto_aead_setauthsize invocation. The caller must ensure
// that sufficient memory is available for the ciphertext and
// the authentication tag.
//
// Return: 0 if the cipher operation was successful; < 0 if an error occurred
//
extern "C" {
    pub fn crypto_aead_encrypt(req: *mut aead_request) -> c_int;
}
//
// crypto_aead_decrypt() - decrypt ciphertext
// @req: reference to the aead_request handle that holds all information
// needed to perform the cipher operation
//
// Decrypt ciphertext data using the aead_request handle. That data structure
// and how it is filled with data is discussed with the aead_request_
// functions.
//
// IMPORTANT NOTE The caller must concatenate the ciphertext followed by the
// authentication data / tag. That authentication data / tag
// must have the size defined by the crypto_aead_setauthsize
// invocation.
//
// Return: 0 if the cipher operation was successful; -EBADMSG: The AEAD
// cipher operation performs the authentication of the data during the
// decryption operation. Therefore, the function returns this error if
// the authentication of the ciphertext was unsuccessful (i.e. the
// integrity of the ciphertext or the associated data was violated);
// < 0 if an error occurred.
//
extern "C" {
    pub fn crypto_aead_decrypt(req: *mut aead_request) -> c_int;
}
//
// DOC: Asynchronous AEAD Request Handle
//
// The aead_request data structure contains all pointers to data required for
// the AEAD cipher operation. This includes the cipher handle (which can be
// used by multiple aead_request instances), pointer to plaintext and
// ciphertext, asynchronous callback function, etc. It acts as a handle to the
// aead_request_* API calls in a similar way as AEAD handle to the
// crypto_aead_* API calls.
//
// crypto_aead_reqsize() - obtain size of the request data structure
// @tfm: cipher handle
//
// Return: number of bytes
//
// aead_request_set_tfm() - update cipher handle reference in request
// @req: request handle to be modified
// @tfm: cipher handle that shall be added to the request handle
//
// Allow the caller to replace the existing aead handle in the request
// data structure with a different one.
//
// aead_request_alloc() - allocate request data structure
// @tfm: cipher handle to be registered with the request
// @gfp: memory allocation flag that is handed to kmalloc by the API call.
//
// Allocate the request data structure that must be used with the AEAD
// encrypt and decrypt API calls. During the allocation, the provided aead
// handle is registered in the request data structure.
//
// Return: allocated request handle in case of success, or NULL if out of memory
//
// aead_request_free() - zeroize and free request data structure
// @req: request data structure cipher handle to be freed
//
// aead_request_set_callback() - set asynchronous callback function
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
// Setting the callback function that is triggered once the cipher operation
// completes
//
// The callback function is registered with the aead_request handle and
// must comply with the following template::
//
// void callback_function(struct crypto_async_request *req, int error)
//
// aead_request_set_crypt - set data buffers
// @req: request handle
// @src: source scatter / gather list
// @dst: destination scatter / gather list
// @cryptlen: number of bytes to process from @src
// @iv: IV for the cipher operation which must comply with the IV size defined
// by crypto_aead_ivsize()
//
// Setting the source data and destination data scatter / gather lists which
// hold the associated data concatenated with the plaintext or ciphertext. See
// below for the authentication tag.
//
// For encryption, the source is treated as the plaintext and the
// destination is the ciphertext. For a decryption operation, the use is
// reversed - the source is the ciphertext and the destination is the plaintext.
//
// The memory structure for cipher operation has the following structure:
//
// - AEAD encryption input:  assoc data || plaintext
// - AEAD encryption output: assoc data || ciphertext || auth tag
// - AEAD decryption input:  assoc data || ciphertext || auth tag
// - AEAD decryption output: assoc data || plaintext
//
// Albeit the kernel requires the presence of the AAD buffer, however,
// the kernel does not fill the AAD buffer in the output case. If the
// caller wants to have that data buffer filled, the caller must either
// use an in-place cipher operation (i.e. same memory location for
// input/output memory location).
//
// aead_request_set_ad - set associated data information
// @req: request handle
// @assoclen: number of bytes in associated data
//
// Setting the AD information.  This function sets the length of
// the associated data.
//
