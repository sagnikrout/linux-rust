//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/s390/crypto/zcrypt_ccamisc.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright IBM Corp. 2019
// Author(s): Harald Freudenberger <freude@linux.ibm.com>
// Ingo Franzki <ifranzki@linux.ibm.com>
//
// Collection of CCA misc functions used by zcrypt and pkey
//

// Key token types
pub const TOKTYPE_NON_CCA: c_uint = 0x00 /* Non-CCA key token */;
pub const TOKTYPE_CCA_INTERNAL: c_uint = 0x01 /* CCA internal sym key token */;
pub const TOKTYPE_CCA_INTERNAL_PKA: c_uint = 0x1f /* CCA internal asym key token */;
// For TOKTYPE_NON_CCA:
pub const TOKVER_PROTECTED_KEY: c_uint = 0x01 /* Protected key token */;
pub const TOKVER_CLEAR_KEY: c_uint = 0x02 /* Clear key token */;
// For TOKTYPE_CCA_INTERNAL:
pub const TOKVER_CCA_AES: c_uint = 0x04 /* CCA AES key token */;
pub const TOKVER_CCA_VLSC: c_uint = 0x05 /* var length sym cipher key token */;
// Max size of a cca variable length cipher key token
pub const MAXCCAVLSCTOKENSIZE: c_int = 725;
// header part of a CCA key token
#[repr(C)]
#[derive(Copy, Clone)]
pub struct keytoken_header {
    pub /: *mut *mut u8 type; / one of the TOKTYPE values,
    pub res0: [u8; 1],
    pub /: *mut *mut u16 len; / vlsc token: total length in bytes,
    pub /: *mut *mut u8 version; / one of the TOKVER values,
    pub res1: [u8; 3],
    pub __packed: },
// inside view of a CCA secure key token (only type 0x01 version 0x04)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct secaeskeytoken {
    pub /: *mut *mut u8 type; / 0x01 for internal key token,
    pub res0: [u8; 3],
    pub /: *mut *mut u8 version; / should be 0x04,
    pub res1: [u8; 1],
    pub /: *mut *mut u8 flag; / key flags,
    pub res2: [u8; 1],
    pub /: *mut *mut u8 mkvp[8]; / master key verification pattern,
    pub /: *mut *mut u8 key[32]; / key value (encrypted),
    pub /: *mut *mut u8 cv[8]; / control vector,
    pub /: *mut *mut u16 bitsize; / key bit size,
    pub /: *mut *mut u16 keysize; / key byte size,
    pub /: *mut *mut u8 tvv[4]; / token validation value,
    pub __packed: },
// inside view of a variable length symmetric cipher AES key token
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cipherkeytoken {
    pub /: *mut *mut u8 type; / 0x01 for internal key token,
    pub res0: [u8; 1],
    pub /: *mut *mut u16 len; / total key token length in bytes,
    pub /: *mut *mut u8 version; / should be 0x05,
    pub res1: [u8; 3],
    pub /: *mut *mut u8 kms; / key material state, 0x03 means wrapped with MK,
    pub /: *mut *mut u8 kvpt; / key verification pattern type, should be 0x01,
    pub /: *mut *mut u8 mkvp0[8]; / master key verification pattern, lo part,
    pub /: *mut *mut u8 mkvp1[8]; / master key verification pattern, hi part (unused),
    pub /: *mut *mut u8 eskwm; / encrypted section key wrapping method,
    pub /: *mut *mut u8 hashalg; / hash algorithmus used for wrapping key,
    pub /: *mut *mut u8 plfver; / pay load format version,
    pub res2: [u8; 1],
    pub /: *mut *mut u8 adsver; / associated data section version,
    pub res3: [u8; 1],
    pub /: *mut *mut u16 adslen; / associated data section length,
    pub /: *mut *mut u8 kllen; / optional key label length,
    pub /: *mut *mut u8 ieaslen; / optional extended associated data length,
    pub /: *mut *mut u8 uadlen; / optional user definable associated data length,
    pub res4: [u8; 1],
    pub /: *mut *mut u16 wpllen; / wrapped payload length in bits:,
// plfver  0x00 0x01
// AES-128  512  640
// AES-192  576  640
// AES-256  640  640
    pub res5: [u8; 1],
    pub /: *mut *mut u8 algtype; / 0x02 for AES cipher,
    pub /: *mut *mut u16 keytype; / 0x0001 for 'cipher',
    pub /: *mut *mut u8 kufc; / key usage field count,
    pub /: *mut *mut u16 kuf1; / key usage field 1,
    pub /: *mut *mut u16 kuf2; / key usage field 2,
    pub /: *mut *mut u8 kmfc; / key management field count,
    pub /: *mut *mut u16 kmf1; / key management field 1,
    pub /: *mut *mut u16 kmf2; / key management field 2,
    pub /: *mut *mut u16 kmf3; / key management field 3,
    pub /: *mut *mut u8 vdata[]; / variable part data follows,
    pub __packed: },
// inside view of an CCA secure ECC private key
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eccprivkeytoken {
    pub /: *mut *mut u8 type; / 0x1f for internal asym key token,
    pub /: *mut *mut u8 version; / should be 0x00,
    pub /: *mut *mut u16 len; / total key token length in bytes,
    pub res1: [u8; 4],
    pub /: *mut *mut u8 secid; / 0x20 for ECC priv key section marker,
    pub /: *mut *mut u8 secver; / section version,
    pub /: *mut *mut u16 seclen; / section length,
    pub /: *mut *mut u8 wtype; / wrapping method, 0x00 clear, 0x01 AES,
    pub /: *mut *mut u8 htype; / hash method, 0x02 for SHA-256,
    pub res2: [u8; 2],
    pub /: *mut *mut u8 kutc; / key usage and translation control,
    pub /: *mut *mut u8 ctype; / curve type,
    pub /: *mut *mut u8 kfs; / key format and security,
    pub /: *mut *mut u8 ksrc; / key source,
    pub /: *mut *mut u16 pbitlen; / length of prime p in bits,
    pub /: *mut *mut u16 ibmadlen; / IBM associated data length in bytes,
    pub /: *mut *mut u8 mkvp[8]; / master key verification pattern,
    pub /: *mut *mut u8 opk[48]; / encrypted object protection key data,
    pub /: *mut *mut u16 adatalen; / associated data length in bytes,
    pub /: *mut *mut u16 fseclen; / formatted section length in bytes,
    pub /: *mut *mut u8 more_data[]; / more data follows,
    pub __packed: },
// Some defines for the CCA AES cipherkeytoken kmf1 field
pub const KMF1_XPRT_SYM: c_uint = 0x8000;
pub const KMF1_XPRT_UASY: c_uint = 0x4000;
pub const KMF1_XPRT_AASY: c_uint = 0x2000;
pub const KMF1_XPRT_RAW: c_uint = 0x1000;
pub const KMF1_XPRT_CPAC: c_uint = 0x0800;
pub const KMF1_XPRT_DES: c_uint = 0x0080;
pub const KMF1_XPRT_AES: c_uint = 0x0040;
pub const KMF1_XPRT_RSA: c_uint = 0x0008;
//
// Simple check if the token is a valid CCA secure AES data key
// token. If keybitsize is given, the bitsize of the key is
// also checked. Returns 0 on success or errno value on failure.
//
    pub keybitsize): *const *const u8 token, u32 keysize, int,
//
// Simple check if the token is a valid CCA secure AES cipher key
// token. If keybitsize is given, the bitsize of the key is
// also checked. If checkcpacfexport is enabled, the key is also
// checked for the export flag to allow CPACF export.
// Returns 0 on success or errno value on failure.
//
    pub checkcpacfexport): int keybitsize, int,
//
// Simple check if the token is a valid CCA secure ECC private
// key token. Returns 0 on success or errno value on failure.
//
    pub checkcpacfexport): c_int,
//
// Generate (random) CCA AES DATA secure key.
//
    pub xflags): u32,
//
// Generate CCA AES DATA secure key with given clear key value.
//
    pub xflags): *const *const *const u8 clrkey, u8 seckey, u32,
//
// Derive proteced key from an CCA AES DATA secure key.
//
    pub xflags): *mut *mut u32 protkeytype, u32,
//
// Generate (random) CCA AES CIPHER secure key.
//
    pub xflags): *mut *mut *mut u8 keybuf, u32 keybufsize, u32,
//
// Derive proteced key from CCA AES cipher secure key.
//
    pub xflags): u32,
//
// Build CCA AES CIPHER secure key with a given clear key value.
//
    pub xflags): u32,
//
// Derive proteced key from CCA ECC secure private key.
//
    pub xflags): *mut *mut *mut *mut u8 protkey, u32 protkeylen, u32 protkeytype, u32,
//
// Query cryptographic facility from CCA adapter
//
    pub xflags): u32,
//
// Build a list of cca apqns meeting the following constrains:
// - apqn is online and is in fact a CCA apqn
// - if cardnr is not FFFF only apqns with this cardnr
// - if domain is not FFFF only apqns with this domainnr
// - if minhwtype > 0 only apqns with hwtype >= minhwtype
// - if cur_mkvp != 0 only apqns where cur_mkvp == mkvp
// - if old_mkvp != 0 only apqns where old_mkvp == mkvp
// The mktype determines which set of master keys to use:
// 0 = AES_MK_SET - AES MK set, 1 = APKA MK_SET - APKA MK set
// The caller should set *nr_apqns to the nr of elements available in *apqns.
// On return *nr_apqns is then updated with the nr of apqns filled into *apqns.
// The return value is either 0 for success or a negative errno value.
// If no apqn meeting the criteria is found, -ENODEV is returned.
//
    pub xflags): *const *const *const u8 cur_mkvp, u8 old_mkvp, u32,
pub const AES_MK_SET: c_int = 0;
pub const APKA_MK_SET: c_int = 1;
// struct to hold info for each CCA queue
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cca_info {
    pub /: *mut *mut u8 new_asym_mkvp[16]; / verify pattern of new asym master key,
    pub /: *mut *mut u8 cur_asym_mkvp[16]; / verify pattern of current asym master key,
    pub /: *mut *mut u8 old_asym_mkvp[16]; / verify pattern of old asym master key,
    pub /: *mut *mut u8 new_aes_mkvp[8]; / truncated sha256 of new aes master key,
    pub /: *mut *mut u8 cur_aes_mkvp[8]; / truncated sha256 of current aes master key,
    pub /: *mut *mut u8 old_aes_mkvp[8]; / truncated sha256 of old aes master key,
    pub /: *mut *mut u8 new_apka_mkvp[8]; / truncated sha256 of new apka master key,
    pub /: *mut *mut u8 cur_apka_mkvp[8]; / truncated sha256 of current apka mk,
    pub /: *mut *mut u8 old_apka_mkvp[8]; / truncated sha256 of old apka mk,
    pub /: *mut *mut char serial[9]; / serial number (8 ascii numbers + 0x00),
    pub /: *mut *mut char new_aes_mk_state; / '1' empty, '2' partially full, '3' full,
    pub /: *mut *mut char cur_aes_mk_state; / '1' invalid, '2' valid,
    pub /: *mut *mut char old_aes_mk_state; / '1' invalid, '2' valid,
    pub /: *mut *mut char new_apka_mk_state; / '1' empty, '2' partially full, '3' full,
    pub /: *mut *mut char cur_apka_mk_state; / '1' invalid, '2' valid,
    pub /: *mut *mut char old_apka_mk_state; / '1' invalid, '2' valid,
    pub /: *mut *mut char new_asym_mk_state; / '1' empty, '2' partially full, '3' full,
    pub /: *mut *mut char cur_asym_mk_state; / '1' invalid, '2' valid,
    pub /: *mut *mut char old_asym_mk_state; / '1' invalid, '2' valid,
    pub /: *mut *mut *mut int hwtype; / one of the defined AP_DEVICE_TYPE_,
}

//
// Fetch cca information about an CCA queue.
//
extern "C" {
    pub fn cca_get_info(card: u16, dom: u16, ci: *mut cca_info, xflags: u32) -> c_int;
}
extern "C" {
    pub fn zcrypt_ccamisc_init() -> c_int;
}
extern "C" {
    pub fn zcrypt_ccamisc_exit();
}
