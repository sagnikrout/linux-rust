//! Automatically rewritten from C Header to Rust Module
//! Source: fs/afs/afs_vl.h
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
// AFS Volume Location Service client interface
//
// Copyright (C) 2002, 2007 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum AFSVL_Operations {
    VLGETENTRYBYID		= 503,	/* AFS Get VLDB entry by ID */
    VLGETENTRYBYNAME	= 504,	/* AFS Get VLDB entry by name */
    VLPROBE			= 514,	/* AFS probe VL service */
    VLGETENTRYBYIDU		= 526,	/* AFS Get VLDB entry by ID (UUID-variant) */
    VLGETENTRYBYNAMEU	= 527,	/* AFS Get VLDB entry by name (UUID-variant) */
    VLGETADDRSU		= 533,	/* AFS Get addrs for fileserver */
    YVLGETENDPOINTS		= 64002, /* YFS Get endpoints for file/volume server */
    YVLGETCELLNAME		= 64014, /* YFS Get actual cell name */
    VLGETCAPABILITIES	= 65537, /* AFS Get server capabilities */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum AFSVL_Errors {
    AFSVL_IDEXIST 		= 363520,	/* Volume Id entry exists in vl database */
    AFSVL_IO 		= 363521,	/* I/O related error */
    AFSVL_NAMEEXIST 	= 363522,	/* Volume name entry exists in vl database */
    AFSVL_CREATEFAIL 	= 363523,	/* Internal creation failure */
    AFSVL_NOENT 		= 363524,	/* No such entry */
    AFSVL_EMPTY 		= 363525,	/* Vl database is empty */
    AFSVL_ENTDELETED 	= 363526,	/* Entry is deleted (soft delete) */
    AFSVL_BADNAME 		= 363527,	/* Volume name is illegal */
    AFSVL_BADINDEX 		= 363528,	/* Index is out of range */
    AFSVL_BADVOLTYPE 	= 363529,	/* Bad volume type */
    AFSVL_BADSERVER 	= 363530,	/* Illegal server number (out of range) */
    AFSVL_BADPARTITION 	= 363531,	/* Bad partition number */
    AFSVL_REPSFULL 		= 363532,	/* Run out of space for Replication sites */
    AFSVL_NOREPSERVER 	= 363533,	/* No such Replication server site exists */
    AFSVL_DUPREPSERVER 	= 363534,	/* Replication site already exists */
    AFSVL_RWNOTFOUND 	= 363535,	/* Parent R/W entry not found */
    AFSVL_BADREFCOUNT 	= 363536,	/* Illegal Reference Count number */
    AFSVL_SIZEEXCEEDED 	= 363537,	/* Vl size for attributes exceeded */
    AFSVL_BADENTRY 		= 363538,	/* Bad incoming vl entry */
    AFSVL_BADVOLIDBUMP 	= 363539,	/* Illegal max volid increment */
    AFSVL_IDALREADYHASHED 	= 363540,	/* RO/BACK id already hashed */
    AFSVL_ENTRYLOCKED 	= 363541,	/* Vl entry is already locked */
    AFSVL_BADVOLOPER 	= 363542,	/* Bad volume operation code */
    AFSVL_BADRELLOCKTYPE 	= 363543,	/* Bad release lock type */
    AFSVL_RERELEASE 	= 363544,	/* Status report: last release was aborted */
    AFSVL_BADSERVERFLAG 	= 363545,	/* Invalid replication site server flag */
    AFSVL_PERM 		= 363546,	/* No permission access */
    AFSVL_NOMEM 		= 363547,	/* malloc/realloc failed to alloc enough memory */
}

pub const YFS_MAXENDPOINTS: c_int = 16;
//
// maps to "struct vldbentry" in vvl-spec.pdf
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct afs_vldbentry {
    pub /: *mut *mut char name[65]; / name of volume (with NUL char),
    pub /: *mut *mut afs_voltype_t type; / volume type,
    pub /: *mut *mut unsigned num_servers; / num servers that hold instances of this vol,
    pub /: *mut *mut unsigned clone_id; / cloning ID,
    pub flags: unsigned,
pub const AFS_VLF_RWEXISTS: c_uint = 0x1000		/* R/W volume exists */;
pub const AFS_VLF_ROEXISTS: c_uint = 0x2000		/* R/O volume exists */;
pub const AFS_VLF_BACKEXISTS: c_uint = 0x4000		/* backup volume exists */;
    pub /: *mut *mut afs_volid_t volume_ids[3]; / volume IDs,
    pub /: *mut *mut in_addr addr; / server address,
    pub /: *mut *mut unsigned partition; / partition ID on this server,
    pub /: *mut *mut unsigned flags; / server specific flags,
pub const AFS_VLSF_NEWREPSITE: c_uint = 0x0001	/* Ignore all 'non-new' servers */;
pub const AFS_VLSF_ROVOL: c_uint = 0x0002	/* this server holds a R/O instance of the volume */;
pub const AFS_VLSF_RWVOL: c_uint = 0x0004	/* this server holds a R/W instance of the volume */;
pub const AFS_VLSF_BACKVOL: c_uint = 0x0008	/* this server holds a backup instance of the volume */;
pub const AFS_VLSF_UUID: c_uint = 0x0010	/* This server is referred to by its UUID */;
pub const AFS_VLSF_DONTUSE: c_uint = 0x0020	/* This server ref should be ignored */;
    pub servers: [}; 8],
}

pub const AFS_VLDB_MAXNAMELEN: c_int = 65;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct afs_ListAddrByAttributes__xdr {
    pub Mask: __be32,
pub const AFS_VLADDR_IPADDR: c_uint = 0x1	/* Match by ->ipaddr */;
pub const AFS_VLADDR_INDEX: c_uint = 0x2	/* Match by ->index */;
pub const AFS_VLADDR_UUID: c_uint = 0x4	/* Match by ->uuid */;
    pub ipaddr: __be32,
    pub index: __be32,
    pub spare: __be32,
    pub uuid: afs_uuid__xdr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct afs_uvldbentry__xdr {
    pub name: [__be32; AFS_VLDB_MAXNAMELEN],
    pub nServers: __be32,
    pub serverNumber: [afs_uuid__xdr; AFS_NMAXNSERVERS],
    pub serverUnique: [__be32; AFS_NMAXNSERVERS],
    pub serverPartition: [__be32; AFS_NMAXNSERVERS],
    pub serverFlags: [__be32; AFS_NMAXNSERVERS],
    pub volumeId: [__be32; AFS_MAXTYPES],
    pub cloneId: __be32,
    pub flags: __be32,
    pub spares1: __be32,
    pub spares2: __be32,
    pub spares3: __be32,
    pub spares4: __be32,
    pub spares5: __be32,
    pub spares6: __be32,
    pub spares7: __be32,
    pub spares8: __be32,
    pub spares9: __be32,
}
