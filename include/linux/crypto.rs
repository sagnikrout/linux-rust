//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/crypto.h
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
// Scatterlist Cryptographic API.
//
// Copyright (c) 2002 James Morris <jmorris@intercode.com.au>
// Copyright (c) 2002 David S. Miller (davem@redhat.com)
// Copyright (c) 2005 Herbert Xu <herbert@gondor.apana.org.au>
//
// Portions derived from Cryptoapi, by Alexander Kjeldaas <astor@fast.no>
// and Nettle, by Niels Möller.
//

//
// Algorithm masks and types.
//
pub const CRYPTO_ALG_TYPE_MASK: c_uint = 0x0000000f;
pub const CRYPTO_ALG_TYPE_CIPHER: c_uint = 0x00000001;
pub const CRYPTO_ALG_TYPE_AEAD: c_uint = 0x00000003;
pub const CRYPTO_ALG_TYPE_LSKCIPHER: c_uint = 0x00000004;
pub const CRYPTO_ALG_TYPE_SKCIPHER: c_uint = 0x00000005;
pub const CRYPTO_ALG_TYPE_AKCIPHER: c_uint = 0x00000006;
pub const CRYPTO_ALG_TYPE_SIG: c_uint = 0x00000007;
pub const CRYPTO_ALG_TYPE_KPP: c_uint = 0x00000008;
pub const CRYPTO_ALG_TYPE_ACOMPRESS: c_uint = 0x0000000a;
pub const CRYPTO_ALG_TYPE_SCOMPRESS: c_uint = 0x0000000b;
pub const CRYPTO_ALG_TYPE_RNG: c_uint = 0x0000000c;
pub const CRYPTO_ALG_TYPE_HASH: c_uint = 0x0000000e;
pub const CRYPTO_ALG_TYPE_SHASH: c_uint = 0x0000000e;
pub const CRYPTO_ALG_TYPE_AHASH: c_uint = 0x0000000f;
pub const CRYPTO_ALG_TYPE_ACOMPRESS_MASK: c_uint = 0x0000000e;
pub const CRYPTO_ALG_LARVAL: c_uint = 0x00000010;
pub const CRYPTO_ALG_DEAD: c_uint = 0x00000020;
pub const CRYPTO_ALG_DYING: c_uint = 0x00000040;
pub const CRYPTO_ALG_ASYNC: c_uint = 0x00000080;
//
// Set if the algorithm (or an algorithm which it uses) requires another
// algorithm of the same type to handle corner cases.
//
pub const CRYPTO_ALG_NEED_FALLBACK: c_uint = 0x00000100;
//
// Set if the algorithm data structure should be duplicated into
// kmalloc memory before registration.  This is useful for hardware
// that can be disconnected at will.  Do not use this if the data
// structure is embedded into a bigger one.  Duplicate the overall
// data structure in the driver in that case.
//
pub const CRYPTO_ALG_DUP_FIRST: c_uint = 0x00000200;
//
// Set if the algorithm has passed automated run-time testing.  Note that
// if there is no run-time testing for a given algorithm it is considered
// to have passed.
//
pub const CRYPTO_ALG_TESTED: c_uint = 0x00000400;
//
// Set if the algorithm is an instance that is built from templates.
//
pub const CRYPTO_ALG_INSTANCE: c_uint = 0x00000800;
// Set this bit if the algorithm provided is hardware accelerated but
// not available to userspace via instruction set or so.
//
pub const CRYPTO_ALG_KERN_DRIVER_ONLY: c_uint = 0x00001000;
//
// Mark a cipher as a service implementation only usable by another
// cipher and never by a normal user of the kernel crypto API
//
pub const CRYPTO_ALG_INTERNAL: c_uint = 0x00002000;
//
// Set if the algorithm has a ->setkey() method but can be used without
// calling it first, i.e. there is a default key.
//
pub const CRYPTO_ALG_OPTIONAL_KEY: c_uint = 0x00004000;
//
// Don't trigger module loading
//
pub const CRYPTO_NOLOAD: c_uint = 0x00008000;
//
// The algorithm may allocate memory during request processing, i.e. during
// encryption, decryption, or hashing.  Users can request an algorithm with this
// flag unset if they can't handle memory allocation failures.
//
// This flag is currently only implemented for algorithms of type "skcipher",
// "aead", "ahash", "shash", and "cipher".  Algorithms of other types might not
// have this flag set even if they allocate memory.
//
// In some edge cases, algorithms can allocate memory regardless of this flag.
// To avoid these cases, users must obey the following usage constraints:
// skcipher:
// - The IV buffer and all scatterlist elements must be aligned to the
// algorithm's alignmask.
// - If the data were to be divided into chunks of size
// crypto_skcipher_walksize() (with any remainder going at the end), no
// chunk can cross a page boundary or a scatterlist element boundary.
// aead:
// - The IV buffer and all scatterlist elements must be aligned to the
// algorithm's alignmask.
// - The first scatterlist element must contain all the associated data,
// and its pages must be !PageHighMem.
// - If the plaintext/ciphertext were to be divided into chunks of size
// crypto_aead_walksize() (with the remainder going at the end), no chunk
// can cross a page boundary or a scatterlist element boundary.
// ahash:
// - crypto_ahash_finup() must not be used unless the algorithm implements
// ->finup() natively.
//
pub const CRYPTO_ALG_ALLOCATES_MEMORY: c_uint = 0x00010000;
//
// Mark an algorithm as a service implementation only usable by a
// template and never by a normal user of the kernel crypto API.
// This is intended to be used by algorithms that are themselves
// not FIPS-approved but may instead be used to implement parts of
// a FIPS-approved algorithm (e.g., dh vs. ffdhe2048(dh)).
//
pub const CRYPTO_ALG_FIPS_INTERNAL: c_uint = 0x00020000;
// Set if the algorithm supports virtual addresses.
pub const CRYPTO_ALG_REQ_VIRT: c_uint = 0x00040000;
// Set if the algorithm cannot have a fallback (e.g., phmac).
pub const CRYPTO_ALG_NO_FALLBACK: c_uint = 0x00080000;
// The high bits 0xff000000 are reserved for type-specific flags.
//
// Transform masks and values (for crt_flags).
//
pub const CRYPTO_TFM_NEED_KEY: c_uint = 0x00000001;
pub const CRYPTO_TFM_REQ_MASK: c_uint = 0x000fff00;
pub const CRYPTO_TFM_REQ_FORBID_WEAK_KEYS: c_uint = 0x00000100;
pub const CRYPTO_TFM_REQ_MAY_SLEEP: c_uint = 0x00000200;
pub const CRYPTO_TFM_REQ_MAY_BACKLOG: c_uint = 0x00000400;
pub const CRYPTO_TFM_REQ_ON_STACK: c_uint = 0x00000800;
//
// Miscellaneous stuff.
//
pub const CRYPTO_MAX_ALG_NAME: c_int = 128;
//
// The macro CRYPTO_MINALIGN_ATTR (along with the void * type in the actual
// declaration) is used to ensure that the crypto_tfm context structure is
// aligned correctly for the given architecture so that there are no alignment
// faults for C data types.  On architectures that support non-cache coherent
// DMA, such as ARM or arm64, it also takes into account the minimal alignment
// that is required to ensure that the context struct member does not share any
// cachelines with the rest of the struct. This is needed to ensure that cache
// maintenance for non-coherent DMA (cache invalidation in particular) does not
// affect data that may be accessed by the CPU concurrently.
//

extern "C" {
    pub fn void(req: *mut *mut crypto_completion_t)(void, err: c_int) -> typedef;
}
//
// DOC: Block Cipher Context Data Structures
//
// These data structures define the operating context for each block cipher
// type.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_async_request {
    pub list: list_head,
    pub complete: crypto_completion_t,
    pub data: *mut c_void,
    pub tfm: *mut crypto_tfm,
    pub flags: u32,
}

//
// DOC: Block Cipher Algorithm Definitions
//
// These data structures define modular crypto algorithm implementations,
// managed via crypto_register_alg() and crypto_unregister_alg().
//
// struct cipher_alg - single-block symmetric ciphers definition
// @cia_min_keysize: Minimum key size supported by the transformation. This is
// the smallest key length supported by this transformation
// algorithm. This must be set to one of the pre-defined
// values as this is not hardware specific. Possible values
// for this field can be found via git grep "_MIN_KEY_SIZE"
// include/crypto
// @cia_max_keysize: Maximum key size supported by the transformation. This is
// the largest key length supported by this transformation
// algorithm. This must be set to one of the pre-defined values
// as this is not hardware specific. Possible values for this
// field can be found via git grep "_MAX_KEY_SIZE"
// include/crypto
// @cia_setkey: Set key for the transformation. This function is used to either
// program a supplied key into the hardware or store the key in the
// transformation context for programming it later. Note that this
// function does modify the transformation context. This function
// can be called multiple times during the existence of the
// transformation object, so one must make sure the key is properly
// reprogrammed into the hardware. This function is also
// responsible for checking the key length for validity.
// @cia_encrypt: Encrypt a single block. This function is used to encrypt a
// single block of data, which must be @cra_blocksize big. This
// always operates on a full @cra_blocksize and it is not possible
// to encrypt a block of smaller size. The supplied buffers must
// therefore also be at least of @cra_blocksize size. Both the
// input and output buffers are always aligned to @cra_alignmask.
// In case either of the input or output buffer supplied by user
// of the crypto API is not aligned to @cra_alignmask, the crypto
// API will re-align the buffers. The re-alignment means that a
// new buffer will be allocated, the data will be copied into the
// new buffer, then the processing will happen on the new buffer,
// then the data will be copied back into the original buffer and
// finally the new buffer will be freed. In case a software
// fallback was put in place in the @cra_init call, this function
// might need to use the fallback if the algorithm doesn't support
// all of the key sizes. In case the key was stored in
// transformation context, the key might need to be re-programmed
// into the hardware in this function. This function shall not
// modify the transformation context, as this function may be
// called in parallel with the same transformation object.
// @cia_decrypt: Decrypt a single block. This is a reverse counterpart to
// @cia_encrypt, and the conditions are exactly the same.
//
// All fields are mandatory and must be filled.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cipher_alg {
    pub cia_min_keysize: c_uint,
    pub cia_max_keysize: c_uint,
    pub keylen): c_uint,
    pub src): *const *const *const *const void (cia_encrypt)(struct crypto_tfm tfm, u8 dst, u8,
    pub src): *const *const *const *const void (cia_decrypt)(struct crypto_tfm tfm, u8 dst, u8,
}

//
// struct crypto_alg - definition of a cryptograpic cipher algorithm
// @cra_flags: Flags describing this transformation. See include/linux/crypto.h
// CRYPTO_ALG_* flags for the flags which go in here. Those are
// used for fine-tuning the description of the transformation
// algorithm.
// @cra_blocksize: Minimum block size of this transformation. The size in bytes
// of the smallest possible unit which can be transformed with
// this algorithm. The users must respect this value.
// In case of HASH transformation, it is possible for a smaller
// block than @cra_blocksize to be passed to the crypto API for
// transformation, in case of any other transformation type, an
// error will be returned upon any attempt to transform smaller
// than @cra_blocksize chunks.
// @cra_ctxsize: Size of the operational context of the transformation. This
// value informs the kernel crypto API about the memory size
// needed to be allocated for the transformation context.
// @cra_alignmask: For cipher, skcipher, lskcipher, and aead algorithms this is
// 1 less than the alignment, in bytes, that the algorithm
// implementation requires for input and output buffers.  When
// the crypto API is invoked with buffers that are not aligned
// to this alignment, the crypto API automatically utilizes
// appropriately aligned temporary buffers to comply with what
// the algorithm needs.  (For scatterlists this happens only if
// the algorithm uses the skcipher_walk helper functions.)  This
// misalignment handling carries a performance penalty, so it is
// preferred that algorithms do not set a nonzero alignmask.
// Also, crypto API users may wish to allocate buffers aligned
// to the alignmask of the algorithm being used, in order to
// avoid the API having to realign them.  Note: the alignmask is
// not supported for hash algorithms and is always 0 for them.
// @cra_reqsize: Size of the request context for this algorithm.
// @cra_priority: Priority of this transformation implementation. In case
// multiple transformations with same @cra_name are available to
// the Crypto API, the kernel will use the one with highest
// @cra_priority.
// @cra_name: Generic name (usable by multiple implementations) of the
// transformation algorithm. This is the name of the transformation
// itself. This field is used by the kernel when looking up the
// providers of particular transformation.
// @cra_driver_name: Unique name of the transformation provider. This is the
// name of the provider of the transformation. This can be any
// arbitrary value, but in the usual case, this contains the
// name of the chip or provider and the name of the
// transformation algorithm.
// @cra_type: Type of the cryptographic transformation. This is a pointer to
// struct crypto_type, which implements callbacks common for all
// transformation types. There are multiple options, such as
// &crypto_skcipher_type, &crypto_ahash_type, &crypto_rng_type.
// This field might be empty. In that case, there are no common
// callbacks. This is the case for: cipher.
// @cra_u: Callbacks implementing the transformation. This is a union of
// multiple structures. Depending on the type of transformation selected
// by @cra_type and @cra_flags above, the associated structure must be
// filled with callbacks. This field might be empty. This is the case
// for ahash, shash.
// @cra_init: Deprecated, do not use.
// @cra_exit: Deprecated, do not use.
// @cra_u.cipher: Union member which contains a single-block symmetric cipher
// definition. See @struct @cipher_alg.
// @cra_module: Owner of this transformation implementation. Set to THIS_MODULE
// @cra_list: internally used
// @cra_users: internally used
// @cra_refcnt: internally used
// @cra_destroy: internally used
//
// The struct crypto_alg describes a generic Crypto API algorithm and is common
// for all of the transformations. Any variable not documented here shall not
// be used by a cipher implementation as it is internal to the Crypto API.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_alg {
    pub cra_list: list_head,
    pub cra_users: list_head,
    pub cra_flags: u32,
    pub cra_blocksize: c_uint,
    pub cra_ctxsize: c_uint,
    pub cra_alignmask: c_uint,
    pub cra_reqsize: c_uint,
    pub cra_priority: c_int,
    pub cra_refcnt: refcount_t,
    pub cra_name: [c_char; CRYPTO_MAX_ALG_NAME],
    pub cra_driver_name: [c_char; CRYPTO_MAX_ALG_NAME],
    pub cra_type: *const crypto_type,
    pub cipher: cipher_alg,
    pub cra_u: },
    pub tfm): *mut *mut int (cra_init)(struct crypto_tfm,
    pub tfm): *mut *mut void (cra_exit)(struct crypto_tfm,
    pub alg): *mut *mut void (cra_destroy)(struct crypto_alg,
    pub cra_module: *mut module,
    pub CRYPTO_MINALIGN_ATTR: },
//
// A helper struct for waiting for completion of async crypto ops
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_wait {
    pub completion: completion,
    pub err: c_int,
}

//
// Macro for declaring a crypto op async wait object on stack
//

//
// Async ops completion helper functioons
//
extern "C" {
    pub fn crypto_req_done(req: *mut c_void, err: c_int);
}
//
// Algorithm query interface.
//
extern "C" {
    pub fn crypto_has_alg(name: *const c_char, type: u32, mask: u32) -> c_int;
}
//
// Transforms: user-instantiated objects which encapsulate algorithms
// and core processing logic.  Managed via crypto_alloc_*() and
// crypto_free_*(), as well as the various helpers below.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_tfm {
    pub crt_flags: u32,
    pub node: c_int,
    pub fb: *mut crypto_tfm,
    pub tfm): *mut *mut void (exit)(struct crypto_tfm,
    pub __crt_alg: *mut crypto_alg,
    pub CRYPTO_MINALIGN_ATTR: *mut *mut void __crt_ctx[],
}

//
// Transform user interface.
//
extern "C" {
    pub fn crypto_destroy_tfm(mem: *mut c_void, tfm: *mut crypto_tfm);
}
extern "C" {
    pub fn crypto_destroy_tfm(_arg: tfm, _arg: tfm) -> return;
}
//
// Transform helpers which query the underlying algorithm.
//
extern "C" {
    pub fn __alignof__(_arg: tfm->__crt_ctx) -> return;
}
