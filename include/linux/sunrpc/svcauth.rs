//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sunrpc/svcauth.h
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
// linux/include/linux/sunrpc/svcauth.h
//
// RPC server-side authentication stuff.
//
// Copyright (C) 1995, 1996 Olaf Kirch <okir@monad.swb.de>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct svc_cred {
    pub cr_uid: kuid_t,
    pub cr_gid: kgid_t,
    pub cr_group_info: *mut group_info,
    pub /: *mut *mut u32 cr_flavor; / pseudoflavor,
// name of form servicetype/hostname@REALM, passed down by
// gss-proxy:
    pub cr_raw_principal: *mut c_char,
// name of form servicetype@hostname, passed down by
// rpc.svcgssd, or computed from the above:
    pub cr_principal: *mut c_char,
    pub cr_targ_princ: *mut c_char,
    pub cr_gss_mech: *mut gss_api_mech,
}

// Authentication is done in the context of a domain.
//
// Currently, the nfs server uses the auth_domain to stand
// for the "client" listed in /etc/exports.
//
// More generally, a domain might represent a group of clients using
// a common mechanism for authentication and having a common mapping
// between local identity (uid) and network identity.  All clients
// in a domain have similar general access rights.  Each domain can
// contain multiple principals which will have different specific right
// based on normal Discretionary Access Control.
//
// A domain is created by an authentication flavour module based on name
// only.  Userspace then fills in detail on demand.
//
// In the case of auth_unix and auth_null, the auth_domain is also
// associated with entries in another cache representing the mapping
// of ip addresses to the given client.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct auth_domain {
    pub ref: kref,
    pub hash: hlist_node,
    pub name: *mut c_char,
    pub flavour: *mut auth_ops,
    pub rcu_head: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum svc_auth_status {
    SVC_GARBAGE = 1,
    SVC_VALID,
    SVC_NEGATIVE,
    SVC_OK,
    SVC_DROP,
    SVC_CLOSE,
    SVC_DENIED,
    SVC_PENDING,
    SVC_COMPLETE,
}

//
// Each authentication flavour registers an auth_ops
// structure.
// name is simply the name.
// flavour gives the auth flavour. It determines where the flavour is registered
// accept() is given a request and should verify it.
// It should inspect the authenticator and verifier, and possibly the data.
// If there is a problem with the authentication *authp should be set.
// The return value of accept() can indicate:
// OK - authorised. client and credential are set in rqstp.
// reqbuf points to arguments
// resbuf points to good place for results.  verfier
// is (probably) already in place.  Certainly space is
// reserved for it.
// DROP - simply drop the request. It may have been deferred
// CLOSE - like SVC_DROP, but request is definitely lost.
// If there is a tcp connection, it should be closed.
// GARBAGE - rpc garbage_args error
// SYSERR - rpc system_err error
// DENIED - authp holds reason for denial.
// COMPLETE - the reply is encoded already and ready to be sent; no
// further processing is necessary.  (This is used for processing
// null procedure calls which are used to set up encryption
// contexts.)
//
// accept is passed the proc number so that it can accept NULL rpc requests
// even if it cannot authenticate the client (as is sometimes appropriate).
//
// release() is given a request after the procedure has been run.
// It should sign/encrypt the results if needed
//
// domain_release()
// This call releases a domain.
//
// set_client()
// Given a pending request (struct svc_rqst), finds and assigns
// an appropriate 'auth_domain' as the client.
//
// pseudoflavor()
// Returns RPC_AUTH pseudoflavor in use by @rqstp.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct auth_ops {
    pub name: *mut *mut c_char,
    pub owner: *mut module,
    pub flavour: c_int,
    pub rqstp): *mut *mut svc_auth_status (accept)(struct svc_rqst,
    pub rqstp): *mut *mut int (release)(struct svc_rqst,
    pub dom): *mut *mut void (domain_release)(struct auth_domain,
    pub rqstp): *mut *mut svc_auth_status (set_client)(struct svc_rqst,
    pub rqstp): *mut *mut rpc_authflavor_t (pseudoflavor)(struct svc_rqst,
}

extern "C" {
    pub fn svc_auth_flavor(rqstp: *mut svc_rqst) -> rpc_authflavor_t;
}
extern "C" {
    pub fn svc_authorise(rqstp: *mut svc_rqst) -> c_int;
}
extern "C" {
    pub fn svc_set_client(rqstp: *mut svc_rqst) -> svc_auth_status;
}
extern "C" {
    pub fn svc_auth_register(flavor: rpc_authflavor_t, aops: *mut auth_ops) -> c_int;
}
extern "C" {
    pub fn svc_auth_unregister(flavor: rpc_authflavor_t);
}
extern "C" {
    pub fn auth_domain_put(item: *mut auth_domain);
}
extern "C" {
    pub fn svcauth_unix_purge(net: *mut net);
}
extern "C" {
    pub fn svcauth_unix_info_release(xpt: *mut svc_xprt);
}
extern "C" {
    pub fn svcauth_unix_set_client(rqstp: *mut svc_rqst) -> svc_auth_status;
}
extern "C" {
    pub fn unix_gid_cache_create(net: *mut net) -> c_int;
}
extern "C" {
    pub fn unix_gid_cache_destroy(net: *mut net);
}
//
// The <stringhash.h> functions are good enough that we don't need to
// use hash_32() on them; just extracting the high bits is enough.
//
extern "C" {
    pub fn hashlen_hash(_arg: hashlen_string(NULL, bits: name)) >> (32 -) -> return;
}
extern "C" {
    pub fn full_name_hash(_arg: NULL, _arg: buf, bits: length) >> (32 -) -> return;
}
