//! Automatically rewritten from C Header to Rust Module
//! Source: crypto/testmgr.h
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
// Algorithm testing framework and tests.
//
// Copyright (c) 2002 James Morris <jmorris@intercode.com.au>
// Copyright (c) 2002 Jean-Francois Dive <jef@linuxbe.org>
// Copyright (c) 2007 Nokia Siemens Networks
// Copyright (c) 2008 Herbert Xu <herbert@gondor.apana.org.au>
// Copyright (c) 2019 Google LLC
//
// Updated RFC4106 AES-GCM testing. Some test vectors were taken from
// http://csrc.nist.gov/groups/ST/toolkit/BCM/documents/proposedmodes
// gcm/gcm-test-vectors.tar.gz
// Authors: Aidan O'Mahony (aidan.o.mahony@intel.com)
// Adrian Hoban <adrian.hoban@intel.com>
// Gabriele Paoloni <gabriele.paoloni@intel.com>
// Tadeusz Struk (tadeusz.struk@intel.com)
// Copyright (c) 2010, Intel Corporation.
//

pub const MAX_IVLEN: c_int = 32;
//
// hash_testvec:	structure to describe a hash (message digest) test
// @key:	Pointer to key (NULL if none)
// @plaintext:	Pointer to source data
// @digest:	Pointer to expected digest
// @psize:	Length of source data in bytes
// @ksize:	Length of @key in bytes (0 if no key)
// @setkey_error: Expected error from setkey()
// @digest_error: Expected error from digest()
// @fips_skip:	Skip the test vector in FIPS mode
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hash_testvec {
    pub key: *const c_char,
    pub plaintext: *const c_char,
    pub digest: *const c_char,
    pub psize: c_uint,
    pub ksize: c_ushort,
    pub setkey_error: c_int,
    pub digest_error: c_int,
    pub fips_skip: bool,
}

//
// cipher_testvec:	structure to describe a symmetric cipher test
// @key:	Pointer to key
// @klen:	Length of @key in bytes
// @iv:		Pointer to IV.  If NULL, an all-zeroes IV is used.
// @iv_out:	Pointer to output IV, if applicable for the cipher.
// @ptext:	Pointer to plaintext
// @ctext:	Pointer to ciphertext
// @len:	Length of @ptext and @ctext in bytes
// @wk:		Does the test need CRYPTO_TFM_REQ_FORBID_WEAK_KEYS?
// ( e.g. test needs to fail due to a weak key )
// @fips_skip:	Skip the test vector in FIPS mode
// @setkey_error: Expected error from setkey()
// @crypt_error: Expected error from encrypt() and decrypt()
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cipher_testvec {
    pub key: *const c_char,
    pub iv: *const c_char,
    pub iv_out: *const c_char,
    pub ptext: *const c_char,
    pub ctext: *const c_char,
    pub /: *mut *mut unsigned char wk; / weak key flag,
    pub klen: c_ushort,
    pub len: c_uint,
    pub fips_skip: bool,
    pub setkey_error: c_int,
    pub crypt_error: c_int,
}

//
// aead_testvec:	structure to describe an AEAD test
// @key:	Pointer to key
// @iv:		Pointer to IV.  If NULL, an all-zeroes IV is used.
// @ptext:	Pointer to plaintext
// @assoc:	Pointer to associated data
// @ctext:	Pointer to the full authenticated ciphertext.  For AEADs that
// produce a separate "ciphertext" and "authentication tag", these
// two parts are concatenated: ciphertext || tag.
// @novrfy:	If set, this is an inauthentic input test: only decryption is
// tested, and it is expected to fail with either -EBADMSG or
// @crypt_error if it is nonzero.
// @wk:		Does the test need CRYPTO_TFM_REQ_FORBID_WEAK_KEYS?
// (e.g. setkey() needs to fail due to a weak key)
// @klen:	Length of @key in bytes
// @plen:	Length of @ptext in bytes
// @alen:	Length of @assoc in bytes
// @clen:	Length of @ctext in bytes
// @setkey_error: Expected error from setkey().  If set, neither encryption nor
// decryption is tested.
// @setauthsize_error: Expected error from setauthsize().  If set, neither
// encryption nor decryption is tested.
// @crypt_error: When @novrfy=0, the expected error from encrypt().  When
// @novrfy=1, an optional alternate error code that is acceptable
// for decrypt() to return besides -EBADMSG.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aead_testvec {
    pub key: *const c_char,
    pub iv: *const c_char,
    pub ptext: *const c_char,
    pub assoc: *const c_char,
    pub ctext: *const c_char,
    pub novrfy: c_uchar,
    pub wk: c_uchar,
    pub klen: c_uchar,
    pub plen: c_uint,
    pub clen: c_uint,
    pub alen: c_uint,
    pub setkey_error: c_int,
    pub setauthsize_error: c_int,
    pub crypt_error: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drbg_testvec {
// Instantiate
    pub entropy: *const c_uchar,
    pub entropylen: usize,
    pub pers: *const c_uchar,
    pub perslen: usize,
// Reseed (optional)
    pub ent_reseed: *const c_uchar,
    pub ent_reseed_len: usize,
    pub addtl_reseed: *const c_uchar,
    pub addtl_reseed_len: usize,
// Generate (twice)
    pub addtla: *const c_uchar,
    pub addtlb: *const c_uchar,
    pub addtllen: usize,
// Expected output from last call to Generate
    pub expected: *const c_uchar,
    pub expectedlen: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct akcipher_testvec {
    pub key: *const c_uchar,
    pub m: *const c_uchar,
    pub c: *const c_uchar,
    pub key_len: c_uint,
    pub m_size: c_uint,
    pub c_size: c_uint,
    pub public_key_vec: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sig_testvec {
    pub key: *const c_uchar,
    pub params: *const c_uchar,
    pub m: *const c_uchar,
    pub c: *const c_uchar,
    pub key_len: c_uint,
    pub param_len: c_uint,
    pub m_size: c_uint,
    pub c_size: c_uint,
    pub public_key_vec: bool,
    pub algo: OID,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kpp_testvec {
    pub secret: *const c_uchar,
    pub b_secret: *const c_uchar,
    pub b_public: *const c_uchar,
    pub expected_a_public: *const c_uchar,
    pub expected_ss: *const c_uchar,
    pub secret_size: c_ushort,
    pub b_secret_size: c_ushort,
    pub b_public_size: c_ushort,
    pub expected_a_public_size: c_ushort,
    pub expected_ss_size: c_ushort,
    pub genkey: bool,
}

//
// RSA test vectors. Borrowed from openSSL.
//

//
// ECDSA test vectors.
//
// ECDSA X9.62 test vectors.
//
// Identical to ECDSA test vectors, except signature in "c" is X9.62 encoded.
//
// ECDSA P1363 test vectors.
//
// Identical to ECDSA test vectors, except signature in "c" is P1363 encoded.
//
// EC-RDSA test vectors are generated by gost-engine.
//
// PKCS#1 RSA test vectors for hash algorithm "none"
// (i.e. the hash in "m" is not prepended by a Full Hash Prefix)
//
// Obtained from:
// https://vcsjones.dev/sometimes-valid-rsa-dotnet
// https://gist.github.com/vcsjones/ab4c2327b53ed018eada76b75ef4fd99
//
// PKCS#1 RSA test vectors. Obtained from CAVS testing.
//
// m is SHA256 hash of following message:
// "\x49\x41\xbe\x0a\x0c\xc9\xf6\x35\x51\xe4\x27\x56\x13\x71\x4b\xd0"
// "\x36\x92\x84\x89\x1b\xf8\x56\x4a\x72\x61\x14\x69\x4f\x5e\x98\xa5"
// "\x80\x5a\x37\x51\x1f\xd8\xf5\xb5\x63\xfc\xf4\xb1\xbb\x4d\x33\xa3"
// "\x1e\xb9\x75\x8b\x9c\xda\x7e\x6d\x3a\x77\x85\xf7\xfc\x4e\xe7\x64"
// "\x43\x10\x19\xa0\x59\xae\xe0\xad\x4b\xd3\xc4\x45\xf7\xb1\xc2\xc1"
// "\x65\x01\x41\x39\x5b\x45\x47\xed\x2b\x51\xed\xe3\xd0\x09\x10\xd2"
// "\x39\x6c\x4a\x3f\xe5\xd2\x20\xe6\xb0\x71\x7d\x5b\xed\x26\x60\xf1"
// "\xb4\x73\xd1\xdb\x7d\xc4\x19\x91\xee\xf6\x32\x76\xf2\x19\x7d\xb7"
//

// xa
// p
// g

// xa
// p
// g

// xa

// xa

// xa

// xa

// xa

// xa

// xa

// xa

// xa

// xa

//
// NIST P384 test vectors from RFC5903
//

//
// MD4 test vectors from RFC1320
//
// MD5 test vectors from RFC1321
//
// RIPEMD-160 test vectors from ISO/IEC 10118-3:2004(E)
//
// Streebog test vectors from RFC 6986 and GOST R 34.11-2012
//
// Two HMAC-Streebog test vectors from RFC 7836 and R 50.1.113-2016 A
//
// Example vectors below taken from
// http://www.oscca.gov.cn/UpFile/20101222141857786.pdf
//
// The rest taken from
// https://github.com/adamws/oscca-sm3
//
// A.1. Example 1
// A.1. Example 2
// Example vectors below taken from
// GM/T 0042-2015 Appendix D.3
//
// SHA1 test vectors from FIPS PUB 180-1
// Long vector from CAVS 5.0
//
// SHA224 test vectors from FIPS PUB 180-2
//
// SHA256 test vectors from NIST
//
// SHA384 test vectors from NIST and kerneli
//
// SHA512 test vectors from NIST and kerneli
//
// WHIRLPOOL test vectors from Whirlpool package
// by Vincent Rijmen and Paulo S. L. M. Barreto as part of the NESSIE
// submission
//
// HMAC-MD5 test vectors from RFC2202
// (These need to be fixed to not use strlen).
//
// HMAC-RIPEMD160 test vectors from RFC2286
//
// HMAC-SHA1 test vectors from RFC2202
//
// SHA224 HMAC test vectors from RFC4231
//
// ("Hi There")
// ("what do ya want for nothing?")
// ("Test Using Larger Than Block-Size Key - Hash Key First")
// ("This is a test using a larger than block-size key and a")
//
// HMAC-SHA256 test vectors from
// draft-ietf-ipsec-ciph-sha-256-01.txt
//
// From NIST Special Publication 800-38B, Three Key TDEA
// Corrected test vectors from:
// http://csrc.nist.gov/publications/nistpubs/800-38B/Updated_CMAC_Examples.pdf
//
// SHA384 HMAC test vectors from RFC4231
//
// SHA512 HMAC test vectors from RFC4231
//
// DES test vectors.
//
// Blowfish test vectors.
//
// Twofish test vectors.
//
// Generated from AES-LRW test vectors
// Generated from AES-XTS test vectors
//
// Serpent test vectors.  These are backwards because Serpent writes
// octet sequences in right-to-left mode.
//
// Generated from AES-LRW test vectors
// Generated from AES-XTS test vectors
//
// SM4 test vectors taken from the "The SM4 Blockcipher Algorithm And Its
// Modes Of Operations" draft RFC
// https://datatracker.ietf.org/doc/draft-ribose-cfrg-sm4
//
// Generated from AES-CTS test vectors
// Generated from AES-XTS test vectors
// Cast6 test vectors from RFC 2612
//
// AES test vectors.
//

// from http://grouper.ieee.org/groups/1619/email/pdf00017.pdf
// http://www.mail-archive.com/stds-p1619@listserv.ieee.org/msg00173.html
// http://grouper.ieee.org/groups/1619/email/pdf00086.pdf
// generated using Crypto++
// This is taken from FIPS CAVS.
//
// rfc4309 refers to section 8 of rfc3610 for test vectors, but they all
// use a 13-byte nonce, we only support an 11-byte nonce.  Worse,
// they use AD lengths which are not valid ESP header lengths.
//
// These vectors are copied/generated from the ones for rfc4106 with
// the key truncated by one byte..
//
// ChaCha20-Poly1305 AEAD test vectors from RFC7539 2.8.2./A.5.
//
// draft-irtf-cfrg-chacha20-poly1305
//
// AEGIS-128 test vectors - generated via reference implementation from
// SUPERCOP (https://bench.cr.yp.to/supercop.html):
//
// https://bench.cr.yp.to/supercop/supercop-20170228.tar.xz
// (see crypto_aead/aegis128/)
//
// Borrowed from the first applicable test vector from ACVP:
// https://github.com/usnistgov/ACVP-Server/blob/v1.1.0.33/gen-val/json-files/hmacDRBG-1.0/prompt.json#L4596
// https://github.com/usnistgov/ACVP-Server/blob/v1.1.0.33/gen-val/json-files/hmacDRBG-1.0/expectedResults.json#L986
//
// .entropy = ACVP entropyInput || nonce
// Cast5 test vectors from RFC 2144
//
// ARC4 test vectors from OpenSSL
//
// TEA test vectors
//
// XTEA test vectors
//
// KHAZAD test vectors.
//
// Anubis test vectors.
//
// XETA test vectors
//
// CAMELLIA test vectors.
//
// Generated from AES-LRW test vectors
// Generated from AES-XTS test vectors
//
// SEED test vectors
//
// ARIA test vectors
//
// Same as XChaCha20 test vectors above, but recomputed the ciphertext with
// XChaCha12, using a modified libsodium.
//
// Adiantum test vectors from https://github.com/google/adiantum
// Adiantum with XChaCha20 instead of XChaCha12
// Test vectors from https://github.com/google/adiantum
//
// CTS (Cipher Text Stealing) mode tests
//
// Compression stuff.
//
pub const COMP_BUF_SIZE: c_int = 512;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct comp_testvec {
    pub outlen: int inlen,,
    pub input: [c_char; COMP_BUF_SIZE],
    pub output: [c_char; COMP_BUF_SIZE],
}

//
// Deflate test vectors (null-terminated strings).
// Params: winbits=-11, Z_DEFAULT_COMPRESSION, MAX_MEM_LEVEL.
//
// LZO test vectors (null-terminated strings).
//
// CRC32 test vectors
//
// CRC32C test vectors
//
// based on aes_cbc_tv_template
// based on hmac_sha256_aes_cbc_tv_temp

//
// Test vectors generated using https://github.com/google/hctr2
//
// Test vectors generated using https://github.com/google/hctr2
//

// rfc8009 Appendix A
// "enc no plain"
// "enc plain<block"
// "enc plain==block"
// "enc plain>block"
// rfc8009 Appendix A
// "enc no plain"
// "enc plain<block"
// "enc plain==block"
// "enc plain>block"
// rfc6803 sec 10
// "enc no plain"
// "enc 1 plain",
// "enc 9 plain",
// "enc 13 plain",
// "enc 30 plain",
// "enc no plain",
// "enc 1 plain",
// "enc 9 plain",
// "enc 13 plain",
// "enc 30 plain",
