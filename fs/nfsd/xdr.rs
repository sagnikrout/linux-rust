//! Automatically rewritten from C Header to Rust Module
//! Source: fs/nfsd/xdr.h
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
// XDR types for nfsd. This is mainly a typing exercise.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd_fhandle {
    pub fh: svc_fh,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd_sattrargs {
    pub fh: svc_fh,
    pub attrs: iattr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd_diropargs {
    pub fh: svc_fh,
    pub name: *mut *mut c_char,
    pub len: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd_readargs {
    pub fh: svc_fh,
    pub offset: __u32,
    pub count: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd_writeargs {
    pub fh: svc_fh,
    pub offset: __u32,
    pub len: __u32,
    pub payload: xdr_buf,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd_createargs {
    pub fh: svc_fh,
    pub name: *mut *mut c_char,
    pub len: c_uint,
    pub attrs: iattr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd_renameargs {
    pub ffh: svc_fh,
    pub fname: *mut *mut c_char,
    pub flen: c_uint,
    pub tfh: svc_fh,
    pub tname: *mut *mut c_char,
    pub tlen: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd_linkargs {
    pub ffh: svc_fh,
    pub tfh: svc_fh,
    pub tname: *mut *mut c_char,
    pub tlen: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd_symlinkargs {
    pub ffh: svc_fh,
    pub fname: *mut *mut c_char,
    pub flen: c_uint,
    pub tname: *mut *mut c_char,
    pub tlen: c_uint,
    pub attrs: iattr,
    pub first: kvec,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd_readdirargs {
    pub fh: svc_fh,
    pub cookie: __u32,
    pub count: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd_stat {
    pub status: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd_attrstat {
    pub status: __be32,
    pub fh: svc_fh,
    pub stat: kstat,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd_diropres {
    pub status: __be32,
    pub fh: svc_fh,
    pub stat: kstat,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd_readlinkres {
    pub status: __be32,
    pub len: c_int,
    pub page: *mut page,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd_readres {
    pub status: __be32,
    pub fh: svc_fh,
    pub count: c_ulong,
    pub stat: kstat,
    pub pages: *mut page,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd_readdirres {
// Components of the reply
    pub status: __be32,
    pub count: c_int,
// Used to encode the reply's entry list
    pub xdr: xdr_stream,
    pub dirlist: xdr_buf,
    pub common: readdir_cd,
    pub cookie_offset: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd_statfsres {
    pub status: __be32,
    pub stats: kstatfs,
}

//
// Storage requirements for XDR arguments and results.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union nfsd_xdrstore {
    pub sattr: nfsd_sattrargs,
    pub dirop: nfsd_diropargs,
    pub read: nfsd_readargs,
    pub write: nfsd_writeargs,
    pub create: nfsd_createargs,
    pub rename: nfsd_renameargs,
    pub link: nfsd_linkargs,
    pub symlink: nfsd_symlinkargs,
    pub readdir: nfsd_readdirargs,
}

extern "C" {
    pub fn nfssvc_decode_fhandleargs(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
}
extern "C" {
    pub fn nfssvc_decode_sattrargs(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
}
extern "C" {
    pub fn nfssvc_decode_diropargs(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
}
extern "C" {
    pub fn nfssvc_decode_readargs(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
}
extern "C" {
    pub fn nfssvc_decode_writeargs(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
}
extern "C" {
    pub fn nfssvc_decode_createargs(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
}
extern "C" {
    pub fn nfssvc_decode_renameargs(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
}
extern "C" {
    pub fn nfssvc_decode_linkargs(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
}
extern "C" {
    pub fn nfssvc_decode_symlinkargs(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
}
extern "C" {
    pub fn nfssvc_decode_readdirargs(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
}
extern "C" {
    pub fn nfssvc_encode_statres(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
}
extern "C" {
    pub fn nfssvc_encode_attrstatres(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
}
extern "C" {
    pub fn nfssvc_encode_diropres(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
}
extern "C" {
    pub fn nfssvc_encode_readlinkres(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
}
extern "C" {
    pub fn nfssvc_encode_readres(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
}
extern "C" {
    pub fn nfssvc_encode_statfsres(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
}
extern "C" {
    pub fn nfssvc_encode_readdirres(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
}
extern "C" {
    pub fn nfssvc_encode_nfscookie(resp: *mut nfsd_readdirres, offset: u32);
}
extern "C" {
    pub fn nfssvc_release_attrstat(rqstp: *mut svc_rqst);
}
extern "C" {
    pub fn nfssvc_release_diropres(rqstp: *mut svc_rqst);
}
extern "C" {
    pub fn nfssvc_release_readres(rqstp: *mut svc_rqst);
}
// Helper functions for NFSv2 ACL code
extern "C" {
    pub fn svcxdr_decode_fhandle(xdr: *mut xdr_stream, fhp: *mut svc_fh) -> bool;
}
extern "C" {
    pub fn svcxdr_encode_stat(xdr: *mut xdr_stream, status: __be32) -> bool;
}
