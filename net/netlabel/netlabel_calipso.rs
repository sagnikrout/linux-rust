//! Automatically rewritten from C Header to Rust Module
//! Source: net/netlabel/netlabel_calipso.h
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
// NetLabel CALIPSO Support
//
// This file defines the CALIPSO functions for the NetLabel system.  The
// NetLabel system manages static and dynamic label mappings for network
// protocols such as CIPSO and RIPSO.
//
// Authors: Paul Moore <paul@paul-moore.com>
// Huw Davies <huw@codeweavers.com>
//
// (c) Copyright Hewlett-Packard Development Company, L.P., 2006
// (c) Copyright Huw Davies <huw@codeweavers.com>, 2015
//

// The following NetLabel payloads are supported by the CALIPSO subsystem.
//
// o ADD:
// Sent by an application to add a new DOI mapping table.
//
// Required attributes:
//
// NLBL_CALIPSO_A_DOI
// NLBL_CALIPSO_A_MTYPE
//
// If using CALIPSO_MAP_PASS no additional attributes are required.
//
// o REMOVE:
// Sent by an application to remove a specific DOI mapping table from the
// CALIPSO system.
//
// Required attributes:
//
// NLBL_CALIPSO_A_DOI
//
// o LIST:
// Sent by an application to list the details of a DOI definition.  On
// success the kernel should send a response using the following format.
//
// Required attributes:
//
// NLBL_CALIPSO_A_DOI
//
// The valid response message format depends on the type of the DOI mapping,
// the defined formats are shown below.
//
// Required attributes:
//
// NLBL_CALIPSO_A_MTYPE
//
// If using CALIPSO_MAP_PASS no additional attributes are required.
//
// o LISTALL:
// This message is sent by an application to list the valid DOIs on the
// system.  When sent by an application there is no payload and the
// NLM_F_DUMP flag should be set.  The kernel should respond with a series of
// the following messages.
//
// Required attributes:
//
// NLBL_CALIPSO_A_DOI
// NLBL_CALIPSO_A_MTYPE
//
// NetLabel CALIPSO commands
// NetLabel CALIPSO attributes
// (NLA_U32)
// the DOI value
// (NLA_U32)
// the mapping table type (defined in the calipso.h header as
// CALIPSO_MAP_*)

// NetLabel protocol functions

extern "C" {
    pub fn netlbl_calipso_genl_init() -> c_int;
}

extern "C" {
    pub fn calipso_doi_free(doi_def: *mut calipso_doi);
}
extern "C" {
    pub fn calipso_doi_remove(doi: u32, audit_info: *mut netlbl_audit) -> c_int;
}
extern "C" {
    pub fn calipso_doi_putdef(doi_def: *mut calipso_doi);
}
extern "C" {
    pub fn calipso_sock_getattr(sk: *mut sock, secattr: *mut netlbl_lsm_secattr) -> c_int;
}
extern "C" {
    pub fn calipso_sock_delattr(sk: *mut sock);
}
extern "C" {
    pub fn calipso_req_delattr(req: *mut request_sock);
}
extern "C" {
    pub fn calipso_skbuff_delattr(skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn calipso_cache_invalidate();
}
