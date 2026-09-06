//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/cpacf.h
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
// CP Assist for Cryptographic Functions (CPACF)
//
// Copyright IBM Corp. 2003, 2023
// Author(s): Thomas Spatzier
// Jan Glauber
// Harald Freudenberger (freude@de.ibm.com)
// Martin Schwidefsky <schwidefsky@de.ibm.com>
//

//
// Instruction opcodes for the CPACF instructions
//
pub const CPACF_KMAC: c_uint = 0xb91e		/* MSA	*/;
pub const CPACF_KM: c_uint = 0xb92e		/* MSA	*/;
pub const CPACF_KMC: c_uint = 0xb92f		/* MSA	*/;
pub const CPACF_KIMD: c_uint = 0xb93e		/* MSA	*/;
pub const CPACF_KLMD: c_uint = 0xb93f		/* MSA	*/;
pub const CPACF_PCKMO: c_uint = 0xb928		/* MSA3 */;
pub const CPACF_KMF: c_uint = 0xb92a		/* MSA4 */;
pub const CPACF_KMO: c_uint = 0xb92b		/* MSA4 */;
pub const CPACF_PCC: c_uint = 0xb92c		/* MSA4 */;
pub const CPACF_KMCTR: c_uint = 0xb92d		/* MSA4 */;
pub const CPACF_PRNO: c_uint = 0xb93c		/* MSA5 */;
pub const CPACF_KMA: c_uint = 0xb929		/* MSA8 */;
pub const CPACF_KDSA: c_uint = 0xb93a		/* MSA9 */;
//
// En/decryption modifier bits
//
pub const CPACF_ENCRYPT: c_uint = 0x00;
pub const CPACF_DECRYPT: c_uint = 0x80;
//
// Function codes for the KM (CIPHER MESSAGE) instruction
//
pub const CPACF_KM_QUERY: c_uint = 0x00;
pub const CPACF_KM_DEA: c_uint = 0x01;
pub const CPACF_KM_TDEA_128: c_uint = 0x02;
pub const CPACF_KM_TDEA_192: c_uint = 0x03;
pub const CPACF_KM_AES_128: c_uint = 0x12;
pub const CPACF_KM_AES_192: c_uint = 0x13;
pub const CPACF_KM_AES_256: c_uint = 0x14;
pub const CPACF_KM_PAES_128: c_uint = 0x1a;
pub const CPACF_KM_PAES_192: c_uint = 0x1b;
pub const CPACF_KM_PAES_256: c_uint = 0x1c;
pub const CPACF_KM_XTS_128: c_uint = 0x32;
pub const CPACF_KM_XTS_256: c_uint = 0x34;
pub const CPACF_KM_PXTS_128: c_uint = 0x3a;
pub const CPACF_KM_PXTS_256: c_uint = 0x3c;
pub const CPACF_KM_XTS_128_FULL: c_uint = 0x52;
pub const CPACF_KM_XTS_256_FULL: c_uint = 0x54;
pub const CPACF_KM_PXTS_128_FULL: c_uint = 0x5a;
pub const CPACF_KM_PXTS_256_FULL: c_uint = 0x5c;
//
// Function codes for the KMC (CIPHER MESSAGE WITH CHAINING)
// instruction
//
pub const CPACF_KMC_QUERY: c_uint = 0x00;
pub const CPACF_KMC_DEA: c_uint = 0x01;
pub const CPACF_KMC_TDEA_128: c_uint = 0x02;
pub const CPACF_KMC_TDEA_192: c_uint = 0x03;
pub const CPACF_KMC_AES_128: c_uint = 0x12;
pub const CPACF_KMC_AES_192: c_uint = 0x13;
pub const CPACF_KMC_AES_256: c_uint = 0x14;
pub const CPACF_KMC_PAES_128: c_uint = 0x1a;
pub const CPACF_KMC_PAES_192: c_uint = 0x1b;
pub const CPACF_KMC_PAES_256: c_uint = 0x1c;
pub const CPACF_KMC_PRNG: c_uint = 0x43;
//
// Function codes for the KMCTR (CIPHER MESSAGE WITH COUNTER)
// instruction
//
pub const CPACF_KMCTR_QUERY: c_uint = 0x00;
pub const CPACF_KMCTR_DEA: c_uint = 0x01;
pub const CPACF_KMCTR_TDEA_128: c_uint = 0x02;
pub const CPACF_KMCTR_TDEA_192: c_uint = 0x03;
pub const CPACF_KMCTR_AES_128: c_uint = 0x12;
pub const CPACF_KMCTR_AES_192: c_uint = 0x13;
pub const CPACF_KMCTR_AES_256: c_uint = 0x14;
pub const CPACF_KMCTR_PAES_128: c_uint = 0x1a;
pub const CPACF_KMCTR_PAES_192: c_uint = 0x1b;
pub const CPACF_KMCTR_PAES_256: c_uint = 0x1c;
//
// Function codes for the KIMD (COMPUTE INTERMEDIATE MESSAGE DIGEST)
// instruction
//
pub const CPACF_KIMD_QUERY: c_uint = 0x00;
pub const CPACF_KIMD_SHA_1: c_uint = 0x01;
pub const CPACF_KIMD_SHA_256: c_uint = 0x02;
pub const CPACF_KIMD_SHA_512: c_uint = 0x03;
pub const CPACF_KIMD_SHA3_224: c_uint = 0x20;
pub const CPACF_KIMD_SHA3_256: c_uint = 0x21;
pub const CPACF_KIMD_SHA3_384: c_uint = 0x22;
pub const CPACF_KIMD_SHA3_512: c_uint = 0x23;
pub const CPACF_KIMD_GHASH: c_uint = 0x41;
//
// Function codes for the KLMD (COMPUTE LAST MESSAGE DIGEST)
// instruction
//
pub const CPACF_KLMD_QUERY: c_uint = 0x00;
pub const CPACF_KLMD_SHA_1: c_uint = 0x01;
pub const CPACF_KLMD_SHA_256: c_uint = 0x02;
pub const CPACF_KLMD_SHA_512: c_uint = 0x03;
pub const CPACF_KLMD_SHA3_224: c_uint = 0x20;
pub const CPACF_KLMD_SHA3_256: c_uint = 0x21;
pub const CPACF_KLMD_SHA3_384: c_uint = 0x22;
pub const CPACF_KLMD_SHA3_512: c_uint = 0x23;
//
// function codes for the KMAC (COMPUTE MESSAGE AUTHENTICATION CODE)
// instruction
//
pub const CPACF_KMAC_QUERY: c_uint = 0x00;
pub const CPACF_KMAC_DEA: c_uint = 0x01;
pub const CPACF_KMAC_TDEA_128: c_uint = 0x02;
pub const CPACF_KMAC_TDEA_192: c_uint = 0x03;
pub const CPACF_KMAC_HMAC_SHA_224: c_uint = 0x70;
pub const CPACF_KMAC_HMAC_SHA_256: c_uint = 0x71;
pub const CPACF_KMAC_HMAC_SHA_384: c_uint = 0x72;
pub const CPACF_KMAC_HMAC_SHA_512: c_uint = 0x73;
pub const CPACF_KMAC_PHMAC_SHA_224: c_uint = 0x78;
pub const CPACF_KMAC_PHMAC_SHA_256: c_uint = 0x79;
pub const CPACF_KMAC_PHMAC_SHA_384: c_uint = 0x7a;
pub const CPACF_KMAC_PHMAC_SHA_512: c_uint = 0x7b;
//
// Function codes for the PCKMO (PERFORM CRYPTOGRAPHIC KEY MANAGEMENT)
// instruction
//
pub const CPACF_PCKMO_QUERY: c_uint = 0x00;
pub const CPACF_PCKMO_ENC_DES_KEY: c_uint = 0x01;
pub const CPACF_PCKMO_ENC_TDES_128_KEY: c_uint = 0x02;
pub const CPACF_PCKMO_ENC_TDES_192_KEY: c_uint = 0x03;
pub const CPACF_PCKMO_ENC_AES_128_KEY: c_uint = 0x12;
pub const CPACF_PCKMO_ENC_AES_192_KEY: c_uint = 0x13;
pub const CPACF_PCKMO_ENC_AES_256_KEY: c_uint = 0x14;
pub const CPACF_PCKMO_ENC_AES_XTS_128_DOUBLE_KEY: c_uint = 0x15;
pub const CPACF_PCKMO_ENC_AES_XTS_256_DOUBLE_KEY: c_uint = 0x16;
pub const CPACF_PCKMO_ENC_ECC_P256_KEY: c_uint = 0x20;
pub const CPACF_PCKMO_ENC_ECC_P384_KEY: c_uint = 0x21;
pub const CPACF_PCKMO_ENC_ECC_P521_KEY: c_uint = 0x22;
pub const CPACF_PCKMO_ENC_ECC_ED25519_KEY: c_uint = 0x28;
pub const CPACF_PCKMO_ENC_ECC_ED448_KEY: c_uint = 0x29;
pub const CPACF_PCKMO_ENC_HMAC_512_KEY: c_uint = 0x76;
pub const CPACF_PCKMO_ENC_HMAC_1024_KEY: c_uint = 0x7a;
//
// Function codes for the PRNO (PERFORM RANDOM NUMBER OPERATION)
// instruction
//
pub const CPACF_PRNO_QUERY: c_uint = 0x00;
pub const CPACF_PRNO_SHA512_DRNG_GEN: c_uint = 0x03;
pub const CPACF_PRNO_SHA512_DRNG_SEED: c_uint = 0x83;
pub const CPACF_PRNO_TRNG_Q_R2C_RATIO: c_uint = 0x70;
pub const CPACF_PRNO_TRNG: c_uint = 0x72;
//
// Function codes for the KMA (CIPHER MESSAGE WITH AUTHENTICATION)
// instruction
//
pub const CPACF_KMA_QUERY: c_uint = 0x00;
pub const CPACF_KMA_GCM_AES_128: c_uint = 0x12;
pub const CPACF_KMA_GCM_AES_192: c_uint = 0x13;
pub const CPACF_KMA_GCM_AES_256: c_uint = 0x14;
//
// Flags for the KMA (CIPHER MESSAGE WITH AUTHENTICATION) instruction
//
pub const CPACF_KMA_LPC: c_uint = 0x100	/* Last-Plaintext/Ciphertext */;
pub const CPACF_KMA_LAAD: c_uint = 0x200	/* Last-AAD */;
pub const CPACF_KMA_HS: c_uint = 0x400	/* Hash-subkey Supplied */;
//
// Flags for the KIMD/KLMD (COMPUTE INTERMEDIATE/LAST MESSAGE DIGEST)
// instructions
//
pub const CPACF_KIMD_NIP: c_uint = 0x8000;
pub const CPACF_KLMD_DUFOP: c_uint = 0x4000;
pub const CPACF_KLMD_NIP: c_uint = 0x8000;
//
// Function codes for KDSA (COMPUTE DIGITAL SIGNATURE AUTHENTICATION)
// instruction
//
pub const CPACF_KDSA_QUERY: c_uint = 0x00;
pub const CPACF_KDSA_ECDSA_VERIFY_P256: c_uint = 0x01;
pub const CPACF_KDSA_ECDSA_VERIFY_P384: c_uint = 0x02;
pub const CPACF_KDSA_ECDSA_VERIFY_P521: c_uint = 0x03;
pub const CPACF_KDSA_ECDSA_SIGN_P256: c_uint = 0x09;
pub const CPACF_KDSA_ECDSA_SIGN_P384: c_uint = 0x0a;
pub const CPACF_KDSA_ECDSA_SIGN_P521: c_uint = 0x0b;
pub const CPACF_KDSA_ENC_ECDSA_SIGN_P256: c_uint = 0x11;
pub const CPACF_KDSA_ENC_ECDSA_SIGN_P384: c_uint = 0x12;
pub const CPACF_KDSA_ENC_ECDSA_SIGN_P521: c_uint = 0x13;
pub const CPACF_KDSA_EDDSA_VERIFY_ED25519: c_uint = 0x20;
pub const CPACF_KDSA_EDDSA_VERIFY_ED448: c_uint = 0x24;
pub const CPACF_KDSA_EDDSA_SIGN_ED25519: c_uint = 0x28;
pub const CPACF_KDSA_EDDSA_SIGN_ED448: c_uint = 0x2c;
pub const CPACF_KDSA_ENC_EDDSA_SIGN_ED25519: c_uint = 0x30;
pub const CPACF_KDSA_ENC_EDDSA_SIGN_ED448: c_uint = 0x34;
pub const CPACF_FC_QUERY: c_uint = 0x00;
pub const CPACF_FC_QUERY_AUTH_INFO: c_uint = 0x7F;
//
// Prototype for a not existing function to produce a link
// error if __cpacf_query() or __cpacf_check_opcode() is used
// with an invalid compile time const opcode.
//
extern "C" {
    pub fn __cpacf_bad_opcode();
}
//
// cpacf_query() - Query the function code mask for this CPACF opcode
// @opcode: the opcode of the crypto instruction
// @mask: ptr to struct cpacf_mask_t
//
// Executes the query function for the given crypto instruction @opcode
// and checks if @func is available
//
// On success 1 is returned and the mask is filled with the function
// code mask for this CPACF opcode, otherwise 0 is returned.
//
extern "C" {
    pub fn cpacf_test_func(_arg: &mask, _arg: func) -> return;
}
//
// cpacf_qai() - Get the query authentication information for a CPACF opcode
// @opcode: the opcode of the crypto instruction
// @mask: ptr to struct cpacf_qai_t
//
// Executes the query authentication information function for the given crypto
// instruction @opcode and checks if @func is available
//
// On success 1 is returned and the mask is filled with the query authentication
// information for this CPACF opcode, otherwise 0 is returned.
//
// cpacf_km() - executes the KM (CIPHER MESSAGE) instruction
// @func: the function code passed to KM; see CPACF_KM_xxx defines
// @param: address of parameter block; see POP for details on each func
// @dest: address of destination memory area
// @src: address of source memory area
// @src_len: length of src operand in bytes
//
// Returns 0 for the query func, number of processed bytes for
// encryption/decryption funcs
//
// cpacf_kmc() - executes the KMC (CIPHER MESSAGE WITH CHAINING) instruction
// @func: the function code passed to KM; see CPACF_KMC_xxx defines
// @param: address of parameter block; see POP for details on each func
// @dest: address of destination memory area
// @src: address of source memory area
// @src_len: length of src operand in bytes
//
// Returns 0 for the query func, number of processed bytes for
// encryption/decryption funcs
//
// cpacf_kimd() - executes the KIMD (COMPUTE INTERMEDIATE MESSAGE DIGEST)
// instruction
// @func: the function code passed to KM; see CPACF_KIMD_xxx defines
// @param: address of parameter block; see POP for details on each func
// @src: address of source memory area
// @src_len: length of src operand in bytes
//
// cpacf_klmd() - executes the KLMD (COMPUTE LAST MESSAGE DIGEST) instruction
// @func: the function code passed to KM; see CPACF_KLMD_xxx defines
// @param: address of parameter block; see POP for details on each func
// @src: address of source memory area
// @src_len: length of src operand in bytes
//
// _cpacf_kmac() - executes the KMAC (COMPUTE MESSAGE AUTHENTICATION CODE)
// instruction and updates flags in gr0
// @gr0: pointer to gr0 (fc and flags) passed to KMAC; see CPACF_KMAC_xxx defines
// @param: address of parameter block; see POP for details on each func
// @src: address of source memory area
// @src_len: length of src operand in bytes
//
// Returns 0 for the query func, number of processed bytes for digest funcs
//
// cpacf_kmac() - executes the KMAC (COMPUTE MESSAGE AUTHENTICATION CODE)
// instruction
// @func: function code passed to KMAC; see CPACF_KMAC_xxx defines
// @param: address of parameter block; see POP for details on each func
// @src: address of source memory area
// @src_len: length of src operand in bytes
//
// Returns 0 for the query func, number of processed bytes for digest funcs
//
extern "C" {
    pub fn _cpacf_kmac(_arg: &func, _arg: param, _arg: src, _arg: src_len) -> return;
}
//
// cpacf_kmctr() - executes the KMCTR (CIPHER MESSAGE WITH COUNTER) instruction
// @func: the function code passed to KMCTR; see CPACF_KMCTR_xxx defines
// @param: address of parameter block; see POP for details on each func
// @dest: address of destination memory area
// @src: address of source memory area
// @src_len: length of src operand in bytes
// @counter: address of counter value
//
// Returns 0 for the query func, number of processed bytes for
// encryption/decryption funcs
//
// cpacf_prno() - executes the PRNO (PERFORM RANDOM NUMBER OPERATION)
// instruction
// @func: the function code passed to PRNO; see CPACF_PRNO_xxx defines
// @param: address of parameter block; see POP for details on each func
// @dest: address of destination memory area
// @dest_len: size of destination memory area in bytes
// @seed: address of seed data
// @seed_len: size of seed data in bytes
//
// cpacf_trng() - executes the TRNG subfunction of the PRNO instruction
// @ucbuf: buffer for unconditioned data
// @ucbuf_len: amount of unconditioned data to fetch in bytes
// @cbuf: buffer for conditioned data
// @cbuf_len: amount of conditioned data to fetch in bytes
//
// cpacf_pcc() - executes the PCC (PERFORM CRYPTOGRAPHIC COMPUTATION)
// instruction
// @func: the function code passed to PCC; see CPACF_KM_xxx defines
// @param: address of parameter block; see POP for details on each func
//
// Returns the condition code, this is
// 0 - cc code 0 (normal completion)
// 1 - cc code 1 (protected key wkvp mismatch or src operand out of range)
// 2 - cc code 2 (something invalid, scalar multiply infinity, ...)
// Condition code 3 (partial completion) is handled within the asm code
// and never returned.
//
extern "C" {
    pub fn CC_TRANSFORM(_arg: cc) -> return;
}
//
// cpacf_pckmo() - executes the PCKMO (PERFORM CRYPTOGRAPHIC KEY
// MANAGEMENT) instruction
// @func: the function code passed to PCKMO; see CPACF_PCKMO_xxx defines
// @param: address of parameter block; see POP for details on each func
//
// Returns 0.
//
// cpacf_kma() - executes the KMA (CIPHER MESSAGE WITH AUTHENTICATION)
// instruction
// @func: the function code passed to KMA; see CPACF_KMA_xxx defines
// @param: address of parameter block; see POP for details on each func
// @dest: address of destination memory area
// @src: address of source memory area
// @src_len: length of src operand in bytes
// @aad: address of additional authenticated data memory area
// @aad_len: length of aad operand in bytes
//
