//! Automatically rewritten from C Header to Rust Module
//! Source: include/keys/asymmetric-type.h
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
// Asymmetric Public-key cryptography key type interface
//
// See Documentation/crypto/asymmetric-keys.rst
//
// Copyright (C) 2012 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

//
// The key payload is four words.  The asymmetric-type key uses them as
// follows:
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum asymmetric_payload_bits {
    asym_crypto,		/* The data representing the key */
    asym_subtype,		/* Pointer to an asymmetric_key_subtype struct */
    asym_key_ids,		/* Pointer to an asymmetric_key_ids struct */
    asym_auth		/* The key's authorisation (signature, parent key ID) */
}

//
// Identifiers for an asymmetric key ID.  We have three ways of looking up a
// key derived from an X.509 certificate:
//
// (1) Serial Number & Issuer.  Non-optional.  This is the only valid way to
// map a PKCS#7 signature to an X.509 certificate.
//
// (2) Issuer & Subject Unique IDs.  Optional.  These were the original way to
// match X.509 certificates, but have fallen into disuse in favour of (3).
//
// (3) Auth & Subject Key Identifiers.  Optional.  SKIDs are only provided on
// CA keys that are intended to sign other keys, so don't appear in end
// user certificates unless forced.
//
// We could also support an PGP key identifier, which is just a SHA1 sum of the
// public key and certain parameters, but since we don't support PGP keys at
// the moment, we shall ignore those.
//
// What we actually do is provide a place where binary identifiers can be
// stashed and then compare against them when checking for an id match.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct asymmetric_key_id {
    pub len: c_ushort,
    pub __counted_by(len): unsigned char data[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct asymmetric_key_ids {
    pub id: [*mut c_void; 3],
}

//
// The payload is at the discretion of the subtype.
//
