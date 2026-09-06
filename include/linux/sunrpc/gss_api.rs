//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sunrpc/gss_api.h
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
// linux/include/linux/sunrpc/gss_api.h
//
// Somewhat simplified version of the gss api.
//
// Dug Song <dugsong@monkey.org>
// Andy Adamson <andros@umich.edu>
// Bruce Fields <bfields@umich.edu>
// Copyright (c) 2000 The Regents of the University of Michigan
//

// The mechanism-independent gss-api context:
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gss_ctx {
    pub mech_type: *mut gss_api_mech,
    pub internal_ctx_id: *mut c_void,
    pub align: unsigned int slack,,
}

// XXX  arbitrary length - is this set somewhere?
pub const GSS_OID_MAX_LEN: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpcsec_gss_oid {
    pub len: c_uint,
    pub data: [u8; GSS_OID_MAX_LEN],
}

// From RFC 3530
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpcsec_gss_info {
    pub oid: rpcsec_gss_oid,
    pub qop: u32,
    pub service: u32,
}

// gss-api prototypes; note that these are somewhat simplified versions of
// the prototypes specified in RFC 2744.
extern "C" {
    pub fn gss_pseudoflavor_to_service(: *mut gss_api_mech, pseudoflavor: u32) -> u32;
}
extern "C" {
    pub fn gss_pseudoflavor_to_datatouch(: *mut gss_api_mech, pseudoflavor: u32) -> bool;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pf_desc {
    pub pseudoflavor: u32,
    pub qop: u32,
    pub service: u32,
    pub name: *mut c_char,
    pub auth_domain_name: *mut c_char,
    pub domain: *mut auth_domain,
    pub datatouch: bool,
}

// Different mechanisms (e.g., krb5 or spkm3) may implement gss-api, and
// mechanisms may be dynamically registered or unregistered by modules.
// Each mechanism is described by the following struct:
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gss_api_mech {
    pub gm_list: list_head,
    pub gm_owner: *mut module,
    pub gm_oid: rpcsec_gss_oid,
    pub gm_name: *mut c_char,
    pub gm_ops: *const gss_api_ops,
// pseudoflavors supported by this mechanism:
    pub gm_pf_num: c_int,
    pub gm_pfs: *mut *mut pf_desc,
// Should the following be a callback operation instead?
    pub gm_upcall_enctypes: *const c_char,
}

// and must provide the following operations:
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gss_api_ops {
    pub gfp_mask): gfp_t,
    pub mic_token): *mut xdr_netobj,
    pub mic_token): *mut xdr_netobj,
    pub inpages): *mut page,
    pub buf): *mut xdr_buf,
    pub internal_ctx_id): *mut c_void,
}

extern "C" {
    pub fn gss_mech_register(: *mut gss_api_mech) -> c_int;
}
extern "C" {
    pub fn gss_mech_unregister(: *mut gss_api_mech);
}
// returns a mechanism descriptor given an OID, and increments the mechanism's
// reference count.
extern "C" {
    pub fn gss_mech_get_by_OID(: *mut rpcsec_gss_oid) -> *mut gss_api_mech;
}
// Given a GSS security tuple, look up a pseudoflavor
extern "C" {
    pub fn gss_mech_info2flavor(: *mut rpcsec_gss_info) -> rpc_authflavor_t;
}
// Given a pseudoflavor, look up a GSS security tuple
extern "C" {
    pub fn gss_mech_flavor2info(_arg: rpc_authflavor_t, : *mut rpcsec_gss_info) -> c_int;
}
// Returns a reference to a mechanism, given a name like "krb5" etc.
// Similar, but get by pseudoflavor.
extern "C" {
    pub fn gss_mech_get(: *mut gss_api_mech) -> *mut gss_api_mech;
}
// For every successful gss_mech_get or gss_mech_get_by_* call there must be a
// corresponding call to gss_mech_put.
extern "C" {
    pub fn gss_mech_put(: *mut gss_api_mech);
}
