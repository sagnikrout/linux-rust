//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/s390/crypto/zcrypt_ep11misc.h
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
//
// Collection of EP11 misc functions used by zcrypt and pkey
//

pub const EP11_STRUCT_MAGIC: c_uint = 0x1234;
pub const EP11_BLOB_PKEY_EXTRACTABLE: c_uint = 0x00200000;
//
// Internal used values for the version field of the key header.
// Should match to the enum pkey_key_type in pkey.h.
//
pub const TOKVER_EP11_AES: c_uint = 0x03  /* EP11 AES key blob (old style) */;
pub const TOKVER_EP11_AES_WITH_HEADER: c_uint = 0x06 /* EP11 AES key blob with header */;
pub const TOKVER_EP11_ECC_WITH_HEADER: c_uint = 0x07 /* EP11 ECC key blob with header */;
// inside view of an EP11 secure key blob
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ep11keyblob {
    pub session: [u8; 32],
// only used for PKEY_TYPE_EP11:
    pub head: ep11kblob_header,
}

// check ep11 key magic to find out if this is an ep11 key blob
//
// For valid ep11 keyblobs, returns a reference to the wrappingkey verification
// pattern. Otherwise NULL.
//
// Simple check if the key blob is a valid EP11 AES key blob with header.
// If checkcpacfexport is enabled, the key is also checked for the
// attributes needed to export this key for CPACF use.
// Returns 0 on success or errno value on failure.
//
// Simple check if the key blob is a valid EP11 ECC key blob with header.
// If checkcpacfexport is enabled, the key is also checked for the
// attributes needed to export this key for CPACF use.
// Returns 0 on success or errno value on failure.
//
// Simple check if the key blob is a valid EP11 AES key blob with
// the header in the session field (old style EP11 AES key).
// If checkcpacfexport is enabled, the key is also checked for the
// attributes needed to export this key for CPACF use.
// Returns 0 on success or errno value on failure.
//
// EP11 card info struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ep11_card_info {
    pub /: *mut *mut u64 op_mode; / card operational mode(s),
    pub /: *mut *mut char serial[16]; / serial number string (16 ascii, no 0x00 !),
    pub /: *mut *mut u32 API_ord_nr; / API ordinal number,
    pub /: *mut *mut u16 FW_version; / Firmware major and minor version,
}

// EP11 domain info struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ep11_domain_info {
    pub /: *mut *mut u64 op_mode; / domain operational mode(s),
    pub /: *mut *mut u8 cur_wkvp[32]; / current wrapping key verification pattern,
    pub /: *mut *mut u8 new_wkvp[32]; / new wrapping key verification pattern,
    pub /: *mut *mut char cur_wk_state; / '0' invalid, '1' valid,
    pub /: *mut *mut char new_wk_state; / '0' empty, '1' uncommitted, '2' committed,
}

//
// Provide information about an EP11 card.
//
extern "C" {
    pub fn ep11_get_card_info(card: u16, info: *mut ep11_card_info, xflags: u32) -> c_int;
}
//
// Provide information about a domain within an EP11 card.
//
// Generate (random) EP11 AES secure key.
//
// Generate EP11 AES secure key with given clear key value.
//
// Build a list of ep11 apqns meeting the following constrains:
// - apqn is online and is in fact an EP11 apqn
// - if cardnr is not FFFF only apqns with this cardnr
// - if domain is not FFFF only apqns with this domainnr
// - if minhwtype > 0 only apqns with hwtype >= minhwtype
// - if minapi > 0 only apqns with API_ord_nr >= minapi
// - if wkvp != NULL only apqns where the wkvp (EP11_WKVPLEN bytes) matches
// to the first EP11_WKVPLEN bytes of the wkvp of the current wrapping
// key for this domain. When a wkvp is given there will always be a re-fetch
// of the domain info for the potential apqn - so this triggers an request
// reply to each apqn eligible.
// The caller should set *nr_apqns to the nr of elements available in *apqns.
// On return *nr_apqns is then updated with the nr of apqns filled into *apqns.
// The return value is either 0 for success or a negative errno value.
// If no apqn meeting the criteria is found, -ENODEV is returned.
//
// Derive proteced key from EP11 key blob (AES and ECC keys).
//
extern "C" {
    pub fn zcrypt_ep11misc_init() -> c_int;
}
extern "C" {
    pub fn zcrypt_ep11misc_exit();
}
