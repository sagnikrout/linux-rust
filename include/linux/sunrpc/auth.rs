//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sunrpc/auth.h
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
// linux/include/linux/sunrpc/auth.h
//
// Declarations for the RPC client authentication machinery.
//
// Copyright (C) 1996, Olaf Kirch <okir@monad.swb.de>
//

//
// Maximum size of AUTH_NONE authentication information, in XDR words.
//

//
// Size of the nodename buffer. RFC1831 specifies a hard limit of 255 bytes,
// but Linux hostnames are actually limited to __NEW_UTS_LEN bytes.
//

pub const UNX_NGROUPS: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct auth_cred {
    pub cred: *const cred,
    pub /: *const *const *const char principal; / If present, this is a machine credential,
}

//
// Client user credentials
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpc_cred {
    pub /: *mut *mut hlist_node cr_hash; / hash chain,
    pub /: *mut *mut list_head cr_lru; / lru garbage collection,
    pub cr_rcu: rcu_head,
    pub cr_auth: *mut *mut rpc_auth,
    pub cr_ops: *const rpc_credops,
    pub /: *mut *mut unsigned long cr_expire; / when to gc,
    pub /: *mut *mut unsigned long cr_flags; / various flags,
    pub /: *mut *mut refcount_t cr_count; / ref count,
    pub cr_cred: *const cred,
// per-flavor data
}

pub const RPCAUTH_CRED_NEW: c_int = 0;
pub const RPCAUTH_CRED_UPTODATE: c_int = 1;
pub const RPCAUTH_CRED_HASHED: c_int = 2;
pub const RPCAUTH_CRED_NEGATIVE: c_int = 3;
//
// Client authentication handle
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpc_auth {
    pub /: *mut *mut unsigned int au_cslack; / call cred size estimate,
    pub /: *mut *mut unsigned int au_rslack; / reply cred size estimate,
    pub /: *mut *mut unsigned int au_verfsize; / size of reply verifier,
    pub /: *mut *mut unsigned int au_ralign; / words before UL header,
    pub au_flags: c_ulong,
    pub au_ops: *const rpc_authops,
    pub may: *mut *mut rpc_authflavor_t au_flavor; / pseudoflavor (note,
// differ from the flavor in
// au_ops->au_flavor in gss
// case)
    pub /: *mut *mut refcount_t au_count; / Reference counter,
    pub au_credcache: *mut *mut rpc_cred_cache,
// per-flavor data
}

// rpc_auth au_flags

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpc_auth_create_args {
    pub pseudoflavor: rpc_authflavor_t,
    pub target_name: *const c_char,
}

// Flags for rpcauth_lookupcred()
pub const RPCAUTH_LOOKUP_NEW: c_uint = 0x01	/* Accept an uninitialised cred */;
pub const RPCAUTH_LOOKUP_ASYNC: c_uint = 0x02	/* Don't block waiting for memory */;
//
// Client authentication ops
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpc_authops {
    pub owner: *mut module,
    pub /: *mut *mut *mut rpc_authflavor_t au_flavor; / flavor (RPC_AUTH_),
    pub au_name: *mut *mut c_char,
    pub ): *mut rpc_clnt,
    pub ): *mut *mut void (destroy)(struct rpc_auth,
    pub int): *mut *mut *mut int (hash_cred)(struct auth_cred , unsigned,
    pub int): *mut *mut *mut *mut *mut rpc_cred  (lookup_cred)(rpc_auth , auth_cred ,,
    pub gfp_t): *mut *mut *mut *mut *mut rpc_cred  (crcreate)(rpc_auth, auth_cred , int,,
    pub ): *mut *mut rpc_authflavor_t (info2flavor)(struct rpcsec_gss_info,
    pub ): *mut rpcsec_gss_info,
    pub ): *mut rpc_cred,
    pub clnt): *mut *mut int (ping)(struct rpc_clnt,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpc_credops {
    pub /: *const *const *const char  cr_name; / Name of the auth flavour,
    pub ): *mut *mut *mut int (cr_init)(struct rpc_auth , struct rpc_cred,
    pub ): *mut *mut void (crdestroy)(struct rpc_cred,
    pub int): *mut *mut *mut *mut int (crmatch)(struct auth_cred , struct rpc_cred ,,
    pub xdr): *mut xdr_stream,
    pub ): *mut *mut int (crrefresh)(struct rpc_task,
    pub xdr): *mut xdr_stream,
    pub xdr): *mut xdr_stream,
    pub xdr): *mut xdr_stream,
    pub ): *mut *mut int (crkey_timeout)(struct rpc_cred,
    pub ): *mut *mut *mut char  (crstringify_acceptor)(struct rpc_cred,
    pub ): *mut *mut bool (crneed_reencode)(struct rpc_task,
}

extern "C" {
    pub fn rpc_init_authunix() -> int __init;
}
extern "C" {
    pub fn rpcauth_init_module() -> int __init;
}
extern "C" {
    pub fn rpcauth_remove_module();
}
extern "C" {
    pub fn rpc_destroy_authunix();
}
extern "C" {
    pub fn rpcauth_register(: *const rpc_authops) -> c_int;
}
extern "C" {
    pub fn rpcauth_unregister(: *const rpc_authops) -> c_int;
}
extern "C" {
    pub fn rpcauth_release(: *mut rpc_auth);
}
extern "C" {
    pub fn rpcauth_lookup_credcache(: *mut rpc_auth, : *mut auth_cred, _arg: c_int, _arg: gfp_t) -> *mut rpc_cred;
}
extern "C" {
    pub fn rpcauth_init_cred(: *mut rpc_cred, : *const auth_cred, : *mut rpc_auth, : *const rpc_credops);
}
extern "C" {
    pub fn rpcauth_lookupcred(: *mut rpc_auth, _arg: c_int) -> *mut rpc_cred;
}
extern "C" {
    pub fn put_rpccred(: *mut rpc_cred);
}
extern "C" {
    pub fn rpcauth_xmit_need_reencode(task: *mut rpc_task) -> bool;
}
extern "C" {
    pub fn rpcauth_refreshcred(: *mut rpc_task) -> c_int;
}
extern "C" {
    pub fn rpcauth_invalcred(: *mut rpc_task);
}
extern "C" {
    pub fn rpcauth_uptodatecred(: *mut rpc_task) -> c_int;
}
extern "C" {
    pub fn rpcauth_init_credcache(: *mut rpc_auth) -> c_int;
}
extern "C" {
    pub fn rpcauth_destroy_credcache(: *mut rpc_auth);
}
extern "C" {
    pub fn rpcauth_clear_credcache(: *mut rpc_cred_cache);
}
extern "C" {
    pub fn rpcauth_stringify_acceptor(: *mut rpc_cred) -> *mut c_char;
}
