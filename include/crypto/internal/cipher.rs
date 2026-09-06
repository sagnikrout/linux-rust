//! Automatically rewritten from C Header to Rust Module
//! Source: include/crypto/internal/cipher.h
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
// Copyright (c) 2002 James Morris <jmorris@intercode.com.au>
// Copyright (c) 2002 David S. Miller (davem@redhat.com)
// Copyright (c) 2005 Herbert Xu <herbert@gondor.apana.org.au>
//
// Portions derived from Cryptoapi, by Alexander Kjeldaas <astor@fast.no>
// and Nettle, by Niels Möller.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_cipher {
    pub base: crypto_tfm,
}

//
// DOC: Single Block Cipher API
//
// The single block cipher API is used with the ciphers of type
// CRYPTO_ALG_TYPE_CIPHER (listed as type "cipher" in /proc/crypto).
//
// Using the single block cipher API calls, operations with the basic cipher
// primitive can be implemented. These cipher primitives exclude any block
// chaining operations including IV handling.
//
// The purpose of this single block cipher API is to support the implementation
// of templates or other concepts that only need to perform the cipher operation
// on one block at a time. Templates invoke the underlying cipher primitive
// block-wise and process either the input or the output data of these cipher
// operations.
//
// crypto_alloc_cipher() - allocate single block cipher handle
// @alg_name: is the cra_name / name or cra_driver_name / driver name of the
// single block cipher
// @type: specifies the type of the cipher
// @mask: specifies the mask for the cipher
//
// Allocate a cipher handle for a single block cipher. The returned struct
// crypto_cipher is the cipher handle that is required for any subsequent API
// invocation for that single block cipher.
//
// Return: allocated cipher handle in case of success; IS_ERR() is true in case
// of an error, PTR_ERR() returns the error code.
//
extern "C" {
    pub fn __crypto_cipher_cast(_arg: crypto_alloc_base(alg_name, _arg: type, _arg: mask)) -> return;
}
//
// crypto_free_cipher() - zeroize and free the single block cipher handle
// @tfm: cipher handle to be freed
//
// crypto_has_cipher() - Search for the availability of a single block cipher
// @alg_name: is the cra_name / name or cra_driver_name / driver name of the
// single block cipher
// @type: specifies the type of the cipher
// @mask: specifies the mask for the cipher
//
// Return: true when the single block cipher is known to the kernel crypto API;
// false otherwise
//
extern "C" {
    pub fn crypto_has_alg(_arg: alg_name, _arg: type, _arg: mask) -> return;
}
//
// crypto_cipher_blocksize() - obtain block size for cipher
// @tfm: cipher handle
//
// The block size for the single block cipher referenced with the cipher handle
// tfm is returned. The caller may use that information to allocate appropriate
// memory for the data returned by the encryption or decryption operation
//
// Return: block size of cipher
//
extern "C" {
    pub fn crypto_tfm_alg_blocksize(_arg: crypto_cipher_tfm(tfm)) -> return;
}
extern "C" {
    pub fn crypto_tfm_alg_alignmask(_arg: crypto_cipher_tfm(tfm)) -> return;
}
extern "C" {
    pub fn crypto_tfm_get_flags(_arg: crypto_cipher_tfm(tfm)) -> return;
}
//
// crypto_cipher_setkey() - set key for cipher
// @tfm: cipher handle
// @key: buffer holding the key
// @keylen: length of the key in bytes
//
// The caller provided key is set for the single block cipher referenced by the
// cipher handle.
//
// Note, the key length determines the cipher type. Many block ciphers implement
// different cipher modes depending on the key size, such as AES-128 vs AES-192
// vs. AES-256. When providing a 16 byte key for an AES cipher handle, AES-128
// is performed.
//
// Return: 0 if the setting of the key was successful; < 0 if an error occurred
//
// crypto_cipher_encrypt_one() - encrypt one block of plaintext
// @tfm: cipher handle
// @dst: points to the buffer that will be filled with the ciphertext
// @src: buffer holding the plaintext to be encrypted
//
// Invoke the encryption operation of one block. The caller must ensure that
// the plaintext and ciphertext buffers are at least one block in size.
//
// crypto_cipher_decrypt_one() - decrypt one block of ciphertext
// @tfm: cipher handle
// @dst: points to the buffer that will be filled with the plaintext
// @src: buffer holding the ciphertext to be decrypted
//
// Invoke the decryption operation of one block. The caller must ensure that
// the plaintext and ciphertext buffers are at least one block in size.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_cipher_spawn {
    pub base: crypto_spawn,
}

extern "C" {
    pub fn crypto_grab_spawn(_arg: &spawn->base, _arg: inst, _arg: name, _arg: type, _arg: mask) -> return;
}
extern "C" {
    pub fn __crypto_cipher_cast(_arg: crypto_spawn_tfm(&spawn->base, _arg: type, _arg: mask)) -> return;
}
