//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/nfs4.h
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
// include/linux/nfs4.h
//
// NFSv4 protocol definitions.
//
// Copyright (c) 2002 The Regents of the University of Michigan.
// All rights reserved.
//
// Kendrick Smith <kmsmith@umich.edu>
// Andy Adamson   <andros@umich.edu>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfs4_acl_whotype {
    NFS4_ACL_WHO_NAMED = 0,
    NFS4_ACL_WHO_OWNER,
    NFS4_ACL_WHO_GROUP,
    NFS4_ACL_WHO_EVERYONE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_ace {
    pub type: u32,
    pub flag: u32,
    pub access_mask: u32,
    pub whotype: c_int,
    pub who_uid: kuid_t,
    pub who_gid: kgid_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_acl {
    pub naces: u32,
    pub aces: [nfs4_ace; ],
}

pub const NFS4_MAXLABELLEN: c_int = 2048;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_label {
    pub lfs: u32,
    pub pi: u32,
    pub lsmid: u32,
    pub len: u32,
    pub label: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_stateid_struct {
    pub data: [c_char; NFS4_STATEID_SIZE],
    pub seqid: __be32,
    pub other: [c_char; NFS4_STATEID_OTHER_SIZE],
// C attribute field omitted
}

pub type nfs4_stateid = nfs4_stateid_struct;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfs_opnum4 {
    OP_ACCESS = 3,
    OP_CLOSE = 4,
    OP_COMMIT = 5,
    OP_CREATE = 6,
    OP_DELEGPURGE = 7,
    OP_DELEGRETURN = 8,
    OP_GETATTR = 9,
    OP_GETFH = 10,
    OP_LINK = 11,
    OP_LOCK = 12,
    OP_LOCKT = 13,
    OP_LOCKU = 14,
    OP_LOOKUP = 15,
    OP_LOOKUPP = 16,
    OP_NVERIFY = 17,
    OP_OPEN = 18,
    OP_OPENATTR = 19,
    OP_OPEN_CONFIRM = 20,
    OP_OPEN_DOWNGRADE = 21,
    OP_PUTFH = 22,
    OP_PUTPUBFH = 23,
    OP_PUTROOTFH = 24,
    OP_READ = 25,
    OP_READDIR = 26,
    OP_READLINK = 27,
    OP_REMOVE = 28,
    OP_RENAME = 29,
    OP_RENEW = 30,
    OP_RESTOREFH = 31,
    OP_SAVEFH = 32,
    OP_SECINFO = 33,
    OP_SETATTR = 34,
    OP_SETCLIENTID = 35,
    OP_SETCLIENTID_CONFIRM = 36,
    OP_VERIFY = 37,
    OP_WRITE = 38,
    OP_RELEASE_LOCKOWNER = 39,

// nfs41
    OP_BACKCHANNEL_CTL = 40,
    OP_BIND_CONN_TO_SESSION = 41,
    OP_EXCHANGE_ID = 42,
    OP_CREATE_SESSION = 43,
    OP_DESTROY_SESSION = 44,
    OP_FREE_STATEID = 45,
    OP_GET_DIR_DELEGATION = 46,
    OP_GETDEVICEINFO = 47,
    OP_GETDEVICELIST = 48,
    OP_LAYOUTCOMMIT = 49,
    OP_LAYOUTGET = 50,
    OP_LAYOUTRETURN = 51,
    OP_SECINFO_NO_NAME = 52,
    OP_SEQUENCE = 53,
    OP_SET_SSV = 54,
    OP_TEST_STATEID = 55,
    OP_WANT_DELEGATION = 56,
    OP_DESTROY_CLIENTID = 57,
    OP_RECLAIM_COMPLETE = 58,

// nfs42
    OP_ALLOCATE = 59,
    OP_COPY = 60,
    OP_COPY_NOTIFY = 61,
    OP_DEALLOCATE = 62,
    OP_IO_ADVISE = 63,
    OP_LAYOUTERROR = 64,
    OP_LAYOUTSTATS = 65,
    OP_OFFLOAD_CANCEL = 66,
    OP_OFFLOAD_STATUS = 67,
    OP_READ_PLUS = 68,
    OP_SEEK = 69,
    OP_WRITE_SAME = 70,
    OP_CLONE = 71,

// xattr support (RFC8276)
    OP_GETXATTR                = 72,
    OP_SETXATTR                = 73,
    OP_LISTXATTRS              = 74,
    OP_REMOVEXATTR             = 75,

    OP_ILLEGAL = 10044,
}

// Defining first and last NFS4 operations implemented.

// error codes for internal client use
pub const NFS4ERR_RESET_TO_MDS: c_int = 12001;
pub const NFS4ERR_RESET_TO_PNFS: c_int = 12002;
pub const NFS4ERR_FATAL_IOERROR: c_int = 12003;
// See RFC 7530, section 9.1.7
//
// Note: NF4BAD is not actually part of the protocol; it is just used
// internally by nfsd.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfs_ftype4 {
    NF4BAD		= 0,
    NF4REG          = 1,    /* Regular File */
    NF4DIR          = 2,    /* Directory */
    NF4BLK          = 3,    /* Special File - block device */
    NF4CHR          = 4,    /* Special File - character device */
    NF4LNK          = 5,    /* Symbolic Link */
    NF4SOCK         = 6,    /* Special File - socket */
    NF4FIFO         = 7,    /* Special File - fifo */
    NF4ATTRDIR      = 8,    /* Attribute Directory */
    NF4NAMEDATTR    = 9     /* Named Attribute */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum open_claim_type4 {
    NFS4_OPEN_CLAIM_NULL = 0,
    NFS4_OPEN_CLAIM_PREVIOUS = 1,
    NFS4_OPEN_CLAIM_DELEGATE_CUR = 2,
    NFS4_OPEN_CLAIM_DELEGATE_PREV = 3,
    NFS4_OPEN_CLAIM_FH = 4, /* 4.1 */
    NFS4_OPEN_CLAIM_DELEG_CUR_FH = 5, /* 4.1 */
    NFS4_OPEN_CLAIM_DELEG_PREV_FH = 6, /* 4.1 */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum opentype4 {
    NFS4_OPEN_NOCREATE = 0,
    NFS4_OPEN_CREATE = 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum createmode4 {
    NFS4_CREATE_UNCHECKED = 0,
    NFS4_CREATE_GUARDED = 1,
    NFS4_CREATE_EXCLUSIVE = 2,
//
// New to NFSv4.1. If session is persistent,
// GUARDED4 MUST be used. Otherwise, use
// EXCLUSIVE4_1 instead of EXCLUSIVE4.
//
    NFS4_CREATE_EXCLUSIVE4_1 = 3
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum limit_by4 {
    NFS4_LIMIT_SIZE = 1,
    NFS4_LIMIT_BLOCKS = 2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfs4_open_delegation_type4 {
    NFS4_OPEN_DELEGATE_NONE = 0,
    NFS4_OPEN_DELEGATE_READ = 1,
    NFS4_OPEN_DELEGATE_WRITE = 2,
    NFS4_OPEN_DELEGATE_NONE_EXT = 3, /* 4.1 */
    NFS4_OPEN_DELEGATE_READ_ATTRS_DELEG = 4,
    NFS4_OPEN_DELEGATE_WRITE_ATTRS_DELEG = 5,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum why_no_delegation4 {
    WND4_NOT_WANTED = 0,
    WND4_CONTENTION = 1,
    WND4_RESOURCE = 2,
    WND4_NOT_SUPP_FTYPE = 3,
    WND4_WRITE_DELEG_NOT_SUPP_FTYPE = 4,
    WND4_NOT_SUPP_UPGRADE = 5,
    WND4_NOT_SUPP_DOWNGRADE = 6,
    WND4_CANCELLED = 7,
    WND4_IS_DIR = 8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lock_type4 {
    NFS4_UNLOCK_LT = 0,
    NFS4_READ_LT = 1,
    NFS4_WRITE_LT = 2,
    NFS4_READW_LT = 3,
    NFS4_WRITEW_LT = 4
}

//
// Symbol names and values are from RFC 7531 Section 2.
// "XDR Description of NFSv4.0"
//
// Symbol names and values are from RFC 5662 Section 2.
// "XDR Description of NFSv4.1"
//
// Symbol names and values are from RFC 7863 Section 2.
// "XDR Description of NFSv4.2"
//
// Symbol names and values are from RFC 8275 Section 5.
// "The mode_umask Attribute"
//
// Symbol names and values are from RFC 8276 Section 8.6.
// "Numeric Values Assigned to Protocol Extensions"
//
// Symbol name and value are from draft-ietf-nfsv4-uncacheable-files
// Section 7.  "XDR for Uncacheable Attribute"
//
// The following internal definitions enable processing the above
// attribute bits within 32-bit word boundaries.
//
// Mandatory Attributes

// Mandatory in NFSv4.1

// Recommended Attributes

// MDS threshold bitmap bits

pub const NFSPROC4_NULL: c_int = 0;
pub const NFSPROC4_COMPOUND: c_int = 1;
pub const NFS4_VERSION: c_int = 4;
pub const NFS4_MINOR_VERSION: c_int = 0;
pub const NFS4_DEBUG: c_int = 1;
//
// Index of predefined Linux client operations
//
// To ensure that /proc/net/rpc/nfs remains correctly ordered, please
// append only to this enum when adding new client operations.
//
// nfs41 types
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_sessionid {
    pub data: [c_uchar; NFS4_MAX_SESSIONID_LEN],
}

// Create Session Flags
pub const SESSION4_PERSIST: c_uint = 0x001;
pub const SESSION4_BACK_CHAN: c_uint = 0x002;
pub const SESSION4_RDMA: c_uint = 0x004;
pub const SESSION4_FLAG_MASK_A: c_uint = 0x007;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum state_protect_how4 {
    SP4_NONE	= 0,
    SP4_MACH_CRED	= 1,
    SP4_SSV		= 2
}

// GET_DIR_DELEGATION non-fatal status codes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gddrnf4_status {
    GDD4_OK		= 0,
    GDD4_UNAVAIL	= 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pnfs_layouttype {
    LAYOUT_NFSV4_1_FILES  = 1,
    LAYOUT_OSD2_OBJECTS = 2,
    LAYOUT_BLOCK_VOLUME = 3,
    LAYOUT_FLEX_FILES = 4,
    LAYOUT_SCSI = 5,
    LAYOUT_TYPE_MAX
}

// used for both layout return and recall
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pnfs_layoutreturn_type {
    RETURN_FILE = 1,
    RETURN_FSID = 2,
    RETURN_ALL  = 3
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pnfs_iomode {
    IOMODE_READ = 1,
    IOMODE_RW = 2,
    IOMODE_ANY = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pnfs_notify_deviceid_type4 {
    NOTIFY_DEVICEID4_CHANGE = 1 << 1,
    NOTIFY_DEVICEID4_DELETE = 1 << 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pnfs_block_volume_type {
    PNFS_BLOCK_VOLUME_SIMPLE	= 0,
    PNFS_BLOCK_VOLUME_SLICE		= 1,
    PNFS_BLOCK_VOLUME_CONCAT	= 2,
    PNFS_BLOCK_VOLUME_STRIPE	= 3,
    PNFS_BLOCK_VOLUME_SCSI		= 4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pnfs_block_extent_state {
    PNFS_BLOCK_READWRITE_DATA	= 0,
    PNFS_BLOCK_READ_DATA		= 1,
    PNFS_BLOCK_INVALID_DATA		= 2,
    PNFS_BLOCK_NONE_DATA		= 3,
}

// on the wire size of a block layout extent

// on the wire size of a scsi commit range

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scsi_code_set {
    PS_CODE_SET_BINARY	= 1,
    PS_CODE_SET_ASCII	= 2,
    PS_CODE_SET_UTF8	= 3
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scsi_designator_type {
    PS_DESIGNATOR_T10	= 1,
    PS_DESIGNATOR_EUI64	= 2,
    PS_DESIGNATOR_NAA	= 3,
    PS_DESIGNATOR_NAME	= 8
}

pub const NFL4_UFLG_MASK: c_uint = 0x0000003F;
pub const NFL4_UFLG_DENSE: c_uint = 0x00000001;
pub const NFL4_UFLG_COMMIT_THRU_MDS: c_uint = 0x00000002;
pub const NFL4_UFLG_STRIPE_UNIT_SIZE_MASK: c_uint = 0xFFFFFFC0;
// Encoded in the loh_body field of type layouthint4
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum filelayout_hint_care4 {
    NFLH4_CARE_DENSE		= NFL4_UFLG_DENSE,
    NFLH4_CARE_COMMIT_THRU_MDS	= NFL4_UFLG_COMMIT_THRU_MDS,
    NFLH4_CARE_STRIPE_UNIT_SIZE	= 0x00000040,
    NFLH4_CARE_STRIPE_COUNT		= 0x00000080
}

pub const NFS4_DEVICEID4_SIZE: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_deviceid {
    pub data: [c_char; NFS4_DEVICEID4_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum data_content4 {
    NFS4_CONTENT_DATA		= 0,
    NFS4_CONTENT_HOLE		= 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pnfs_update_layout_reason {
    PNFS_UPDATE_LAYOUT_UNKNOWN = 0,
    PNFS_UPDATE_LAYOUT_NO_PNFS,
    PNFS_UPDATE_LAYOUT_RD_ZEROLEN,
    PNFS_UPDATE_LAYOUT_MDSTHRESH,
    PNFS_UPDATE_LAYOUT_NOMEM,
    PNFS_UPDATE_LAYOUT_BULK_RECALL,
    PNFS_UPDATE_LAYOUT_IO_TEST_FAIL,
    PNFS_UPDATE_LAYOUT_FOUND_CACHED,
    PNFS_UPDATE_LAYOUT_RETURN,
    PNFS_UPDATE_LAYOUT_RETRY,
    PNFS_UPDATE_LAYOUT_BLOCKED,
    PNFS_UPDATE_LAYOUT_INVALID_OPEN,
    PNFS_UPDATE_LAYOUT_SEND_LAYOUTGET,
    PNFS_UPDATE_LAYOUT_EXIT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_op_map {
    pub longs: [c_ulong; NFS4_OP_MAP_NUM_LONGS],
    pub words: [u32; NFS4_OP_MAP_NUM_WORDS],
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs42_netaddr {
    pub netid: [c_char; RPCBIND_MAXNETIDLEN],
    pub 1]: char addr[RPCBIND_MAXUADDRLEN +,
    pub netid_len: u32,
    pub addr_len: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum netloc_type4 {
    NL4_NAME		= 1,
    NL4_URL			= 2,
    NL4_NETADDR		= 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nl4_server {
    pub nl4_type: netloc_type4,
    pub nl4_str_sz: c_int,
    pub 1]: char nl4_str[NFS4_OPAQUE_LIMIT +,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfs4_change_attr_type {
    NFS4_CHANGE_TYPE_IS_MONOTONIC_INCR = 0,
    NFS4_CHANGE_TYPE_IS_VERSION_COUNTER = 1,
    NFS4_CHANGE_TYPE_IS_VERSION_COUNTER_NOPNFS = 2,
    NFS4_CHANGE_TYPE_IS_TIME_METADATA = 3,
    NFS4_CHANGE_TYPE_IS_UNDEFINED = 4,
}

//
// Options for setxattr. These match the flags for setxattr(2).
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfs4_setxattr_options {
    SETXATTR4_EITHER	= 0,
    SETXATTR4_CREATE	= 1,
    SETXATTR4_REPLACE	= 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfs_cb_opnum4 {
    OP_CB_GETATTR = 3,
    OP_CB_RECALL  = 4,

// Callback operations new to NFSv4.1
    OP_CB_LAYOUTRECALL  = 5,
    OP_CB_NOTIFY        = 6,
    OP_CB_PUSH_DELEG    = 7,
    OP_CB_RECALL_ANY    = 8,
    OP_CB_RECALLABLE_OBJ_AVAIL = 9,
    OP_CB_RECALL_SLOT   = 10,
    OP_CB_SEQUENCE      = 11,
    OP_CB_WANTS_CANCELLED = 12,
    OP_CB_NOTIFY_LOCK   = 13,
    OP_CB_NOTIFY_DEVICEID = 14,

// Callback operations new to NFSv4.2
    OP_CB_OFFLOAD = 15,

    OP_CB_ILLEGAL = 10044,
}
