//! Automatically rewritten from C Header to Rust Module
//! Source: fs/nfsd/xdr3.h
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
// XDR types for NFSv3 in nfsd.
//
// Copyright (C) 1996-1998, Olaf Kirch <okir@monad.swb.de>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd3_sattrargs {
    pub fh: svc_fh,
    pub attrs: iattr,
    pub check_guard: c_int,
    pub guardtime: timespec64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd3_diropargs {
    pub fh: svc_fh,
    pub name: *mut *mut c_char,
    pub len: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd3_accessargs {
    pub fh: svc_fh,
    pub access: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd3_readargs {
    pub fh: svc_fh,
    pub offset: __u64,
    pub count: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd3_writeargs {
    pub fh: svc_fh,
    pub offset: __u64,
    pub count: __u32,
    pub stable: c_int,
    pub len: __u32,
    pub payload: xdr_buf,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd3_createargs {
    pub fh: svc_fh,
    pub name: *mut *mut c_char,
    pub len: c_uint,
    pub createmode: c_int,
    pub attrs: iattr,
    pub verf: *mut *mut __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd3_mknodargs {
    pub fh: svc_fh,
    pub name: *mut *mut c_char,
    pub len: c_uint,
    pub ftype: __u32,
    pub minor: __u32 major,,
    pub attrs: iattr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd3_renameargs {
    pub ffh: svc_fh,
    pub fname: *mut *mut c_char,
    pub flen: c_uint,
    pub tfh: svc_fh,
    pub tname: *mut *mut c_char,
    pub tlen: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd3_linkargs {
    pub ffh: svc_fh,
    pub tfh: svc_fh,
    pub tname: *mut *mut c_char,
    pub tlen: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd3_symlinkargs {
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
pub struct nfsd3_readdirargs {
    pub fh: svc_fh,
    pub cookie: __u64,
    pub count: __u32,
    pub verf: *mut *mut __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd3_commitargs {
    pub fh: svc_fh,
    pub offset: __u64,
    pub count: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd3_getaclargs {
    pub fh: svc_fh,
    pub mask: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd3_setaclargs {
    pub fh: svc_fh,
    pub mask: __u32,
    pub acl_access: *mut posix_acl,
    pub acl_default: *mut posix_acl,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd3_attrstat {
    pub status: __be32,
    pub fh: svc_fh,
    pub stat: kstat,
}

// LOOKUP, CREATE, MKDIR, SYMLINK, MKNOD
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd3_diropres {
    pub status: __be32,
    pub dirfh: svc_fh,
    pub fh: svc_fh,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd3_accessres {
    pub status: __be32,
    pub fh: svc_fh,
    pub access: __u32,
    pub stat: kstat,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd3_readlinkres {
    pub status: __be32,
    pub fh: svc_fh,
    pub len: __u32,
    pub pages: *mut page,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd3_readres {
    pub status: __be32,
    pub fh: svc_fh,
    pub count: c_ulong,
    pub eof: __u32,
    pub pages: *mut page,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd3_writeres {
    pub status: __be32,
    pub fh: svc_fh,
    pub count: c_ulong,
    pub committed: c_int,
    pub verf: [__be32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd3_renameres {
    pub status: __be32,
    pub ffh: svc_fh,
    pub tfh: svc_fh,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd3_linkres {
    pub status: __be32,
    pub tfh: svc_fh,
    pub fh: svc_fh,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd3_readdirres {
// Components of the reply
    pub status: __be32,
    pub fh: svc_fh,
    pub verf: [__be32; 2],
// Used to encode the reply's entry list
    pub xdr: xdr_stream,
    pub dirlist: xdr_buf,
    pub scratch: svc_fh,
    pub common: readdir_cd,
    pub cookie_offset: c_uint,
    pub rqstp: *mut *mut svc_rqst,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd3_fsstatres {
    pub status: __be32,
    pub stats: kstatfs,
    pub invarsec: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd3_fsinfores {
    pub status: __be32,
    pub f_rtmax: __u32,
    pub f_rtpref: __u32,
    pub f_rtmult: __u32,
    pub f_wtmax: __u32,
    pub f_wtpref: __u32,
    pub f_wtmult: __u32,
    pub f_dtpref: __u32,
    pub f_maxfilesize: __u64,
    pub f_properties: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd3_pathconfres {
    pub status: __be32,
    pub p_link_max: __u32,
    pub p_name_max: __u32,
    pub p_no_trunc: __u32,
    pub p_chown_restricted: __u32,
    pub p_case_insensitive: bool,
    pub p_case_preserving: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd3_commitres {
    pub status: __be32,
    pub fh: svc_fh,
    pub verf: [__be32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd3_getaclres {
    pub status: __be32,
    pub fh: svc_fh,
    pub mask: c_int,
    pub acl_access: *mut posix_acl,
    pub acl_default: *mut posix_acl,
    pub stat: kstat,
}

// dummy type for release
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd3_fhandle_pair {
    pub dummy: __u32,
    pub fh1: svc_fh,
    pub fh2: svc_fh,
}

//
// Storage requirements for XDR arguments and results.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union nfsd3_xdrstore {
    pub sattrargs: nfsd3_sattrargs,
    pub diropargs: nfsd3_diropargs,
    pub readargs: nfsd3_readargs,
    pub writeargs: nfsd3_writeargs,
    pub createargs: nfsd3_createargs,
    pub renameargs: nfsd3_renameargs,
    pub linkargs: nfsd3_linkargs,
    pub symlinkargs: nfsd3_symlinkargs,
    pub readdirargs: nfsd3_readdirargs,
    pub diropres: nfsd3_diropres,
    pub accessres: nfsd3_accessres,
    pub readlinkres: nfsd3_readlinkres,
    pub readres: nfsd3_readres,
    pub writeres: nfsd3_writeres,
    pub renameres: nfsd3_renameres,
    pub linkres: nfsd3_linkres,
    pub readdirres: nfsd3_readdirres,
    pub fsstatres: nfsd3_fsstatres,
    pub fsinfores: nfsd3_fsinfores,
    pub pathconfres: nfsd3_pathconfres,
    pub commitres: nfsd3_commitres,
    pub getaclres: nfsd3_getaclres,
}

extern "C" {
    pub fn nfs3svc_decode_fhandleargs(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
}
extern "C" {
    pub fn nfs3svc_decode_sattrargs(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
}
extern "C" {
    pub fn nfs3svc_decode_diropargs(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
}
extern "C" {
    pub fn nfs3svc_decode_accessargs(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
}
extern "C" {
    pub fn nfs3svc_decode_readargs(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
}
extern "C" {
    pub fn nfs3svc_decode_writeargs(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
}
extern "C" {
    pub fn nfs3svc_decode_createargs(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
}
extern "C" {
    pub fn nfs3svc_decode_mkdirargs(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
}
extern "C" {
    pub fn nfs3svc_decode_mknodargs(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
}
extern "C" {
    pub fn nfs3svc_decode_renameargs(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
}
extern "C" {
    pub fn nfs3svc_decode_linkargs(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
}
extern "C" {
    pub fn nfs3svc_decode_symlinkargs(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
}
extern "C" {
    pub fn nfs3svc_decode_readdirargs(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
}
extern "C" {
    pub fn nfs3svc_decode_readdirplusargs(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
}
extern "C" {
    pub fn nfs3svc_decode_commitargs(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
}
extern "C" {
    pub fn nfs3svc_encode_getattrres(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
}
extern "C" {
    pub fn nfs3svc_encode_wccstat(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
}
extern "C" {
    pub fn nfs3svc_encode_lookupres(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
}
extern "C" {
    pub fn nfs3svc_encode_accessres(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
}
extern "C" {
    pub fn nfs3svc_encode_readlinkres(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
}
extern "C" {
    pub fn nfs3svc_encode_readres(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
}
extern "C" {
    pub fn nfs3svc_encode_writeres(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
}
extern "C" {
    pub fn nfs3svc_encode_createres(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
}
extern "C" {
    pub fn nfs3svc_encode_renameres(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
}
extern "C" {
    pub fn nfs3svc_encode_linkres(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
}
extern "C" {
    pub fn nfs3svc_encode_readdirres(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
}
extern "C" {
    pub fn nfs3svc_encode_fsstatres(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
}
extern "C" {
    pub fn nfs3svc_encode_fsinfores(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
}
extern "C" {
    pub fn nfs3svc_encode_pathconfres(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
}
extern "C" {
    pub fn nfs3svc_encode_commitres(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool;
}
extern "C" {
    pub fn nfs3svc_release_fhandle(: *mut svc_rqst);
}
extern "C" {
    pub fn nfs3svc_release_fhandle2(: *mut svc_rqst);
}
extern "C" {
    pub fn nfs3svc_encode_cookie3(resp: *mut nfsd3_readdirres, offset: u64);
}
// Helper functions for NFSv3 ACL code
extern "C" {
    pub fn svcxdr_decode_nfs_fh3(xdr: *mut xdr_stream, fhp: *mut svc_fh) -> bool;
}
extern "C" {
    pub fn svcxdr_encode_nfsstat3(xdr: *mut xdr_stream, status: __be32) -> bool;
}
