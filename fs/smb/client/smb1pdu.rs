//! Automatically rewritten from C Header to Rust Module
//! Source: fs/smb/client/smb1pdu.h
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


// SPDX-License-Identifier: LGPL-2.1
//
// Copyright (c) International Business Machines  Corp., 2002,2009
// Author(s): Steve French (sfrench@us.ibm.com)
//

pub const CIFS_PROT: c_int = 0;

pub const BAD_PROT: c_uint = 0xFFFF;
// SMB command codes:
// See MS-CIFS 2.2.2.1
// Note some commands have minimal (wct=0,bcc=0), or uninteresting, responses
// (ie which include no useful data other than the SMB error code itself).
// This can allow us to avoid response buffer allocations and copy in some cases
//
pub const SMB_COM_CREATE_DIRECTORY: c_uint = 0x00 /* trivial response */;
pub const SMB_COM_DELETE_DIRECTORY: c_uint = 0x01 /* trivial response */;
pub const SMB_COM_CLOSE: c_uint = 0x04 /* triv req/rsp, timestamp ignored */;
pub const SMB_COM_FLUSH: c_uint = 0x05 /* triv req/rsp */;
pub const SMB_COM_DELETE: c_uint = 0x06 /* trivial response */;
pub const SMB_COM_RENAME: c_uint = 0x07 /* trivial response */;
pub const SMB_COM_QUERY_INFORMATION: c_uint = 0x08 /* aka getattr */;
pub const SMB_COM_SETATTR: c_uint = 0x09 /* trivial response */;
pub const SMB_COM_LOCKING_ANDX: c_uint = 0x24 /* trivial response */;
pub const SMB_COM_COPY: c_uint = 0x29 /* trivial rsp, fail filename ignrd*/;
pub const SMB_COM_ECHO: c_uint = 0x2B /* echo request */;
pub const SMB_COM_OPEN_ANDX: c_uint = 0x2D /* Legacy open for old servers */;
pub const SMB_COM_READ_ANDX: c_uint = 0x2E;
pub const SMB_COM_WRITE_ANDX: c_uint = 0x2F;
pub const SMB_COM_TRANSACTION2: c_uint = 0x32;
pub const SMB_COM_TRANSACTION2_SECONDARY: c_uint = 0x33;
pub const SMB_COM_FIND_CLOSE2: c_uint = 0x34 /* trivial response */;
pub const SMB_COM_TREE_DISCONNECT: c_uint = 0x71 /* trivial response */;
pub const SMB_COM_NEGOTIATE: c_uint = 0x72;
pub const SMB_COM_SESSION_SETUP_ANDX: c_uint = 0x73;
pub const SMB_COM_LOGOFF_ANDX: c_uint = 0x74 /* trivial response */;
pub const SMB_COM_TREE_CONNECT_ANDX: c_uint = 0x75;
pub const SMB_COM_NT_TRANSACT: c_uint = 0xA0;
pub const SMB_COM_NT_TRANSACT_SECONDARY: c_uint = 0xA1;
pub const SMB_COM_NT_CREATE_ANDX: c_uint = 0xA2;
pub const SMB_COM_NT_CANCEL: c_uint = 0xA4 /* no response */;
pub const SMB_COM_NT_RENAME: c_uint = 0xA5 /* trivial response */;
// Transact2 subcommand codes
pub const TRANS2_OPEN: c_uint = 0x00;
pub const TRANS2_FIND_FIRST: c_uint = 0x01;
pub const TRANS2_FIND_NEXT: c_uint = 0x02;
pub const TRANS2_QUERY_FS_INFORMATION: c_uint = 0x03;
pub const TRANS2_SET_FS_INFORMATION: c_uint = 0x04;
pub const TRANS2_QUERY_PATH_INFORMATION: c_uint = 0x05;
pub const TRANS2_SET_PATH_INFORMATION: c_uint = 0x06;
pub const TRANS2_QUERY_FILE_INFORMATION: c_uint = 0x07;
pub const TRANS2_SET_FILE_INFORMATION: c_uint = 0x08;
pub const TRANS2_GET_DFS_REFERRAL: c_uint = 0x10;
pub const TRANS2_REPORT_DFS_INCOSISTENCY: c_uint = 0x11;
// SMB Transact (Named Pipe) subcommand codes
pub const TRANS_SET_NMPIPE_STATE: c_uint = 0x0001;
pub const TRANS_RAW_READ_NMPIPE: c_uint = 0x0011;
pub const TRANS_QUERY_NMPIPE_STATE: c_uint = 0x0021;
pub const TRANS_QUERY_NMPIPE_INFO: c_uint = 0x0022;
pub const TRANS_PEEK_NMPIPE: c_uint = 0x0023;
pub const TRANS_TRANSACT_NMPIPE: c_uint = 0x0026;
pub const TRANS_RAW_WRITE_NMPIPE: c_uint = 0x0031;
pub const TRANS_READ_NMPIPE: c_uint = 0x0036;
pub const TRANS_WRITE_NMPIPE: c_uint = 0x0037;
pub const TRANS_WAIT_NMPIPE: c_uint = 0x0053;
pub const TRANS_CALL_NMPIPE: c_uint = 0x0054;
// NT Transact subcommand codes
pub const NT_TRANSACT_CREATE: c_uint = 0x01;
pub const NT_TRANSACT_IOCTL: c_uint = 0x02;
pub const NT_TRANSACT_SET_SECURITY_DESC: c_uint = 0x03;
pub const NT_TRANSACT_NOTIFY_CHANGE: c_uint = 0x04;
pub const NT_TRANSACT_RENAME: c_uint = 0x05;
pub const NT_TRANSACT_QUERY_SECURITY_DESC: c_uint = 0x06;
pub const NT_TRANSACT_GET_USER_QUOTA: c_uint = 0x07;
pub const NT_TRANSACT_SET_USER_QUOTA: c_uint = 0x08;
// future chained NTCreateXReadX bigger, but for time being NTCreateX biggest
// among the requests (NTCreateX response is bigger with wct of 34)
pub const MAX_CIFS_HDR_SIZE: c_uint = 0x54 /* 32 hdr + (2*24 wct) + 2 bct + 2 pad */;

// internal cifs vfs structures
//
// All constants go here
//
// Starting value for maximum SMB size negotiation
//

//
// Size of encrypted user password in bytes
//

//
// Size of the crypto key returned on the negotiate SMB in bytes
//

//
// Size of the ntlm client response
//

//
// Size of the session key (crypto key encrypted with the password
//

//
// Maximum user name length
//

//
// Flags on SMB open
//
pub const SMBOPEN_WRITE_THROUGH: c_uint = 0x4000;
pub const SMBOPEN_DENY_ALL: c_uint = 0x0010;
pub const SMBOPEN_DENY_WRITE: c_uint = 0x0020;
pub const SMBOPEN_DENY_READ: c_uint = 0x0030;
pub const SMBOPEN_DENY_NONE: c_uint = 0x0040;
pub const SMBOPEN_READ: c_uint = 0x0000;
pub const SMBOPEN_WRITE: c_uint = 0x0001;
pub const SMBOPEN_READWRITE: c_uint = 0x0002;
pub const SMBOPEN_EXECUTE: c_uint = 0x0003;
pub const SMBOPEN_OCREATE: c_uint = 0x0010;
pub const SMBOPEN_OTRUNC: c_uint = 0x0002;
pub const SMBOPEN_OAPPEND: c_uint = 0x0001;
//
// SMB flag definitions
// See MS-CIFS 2.2.3.1
//
pub const SMBFLG_EXTD_LOCK: c_uint = 0x01	/* server supports lock-read write-unlock smb */;
pub const SMBFLG_RCV_POSTED: c_uint = 0x02	/* obsolete */;
pub const SMBFLG_RSVD: c_uint = 0x04;
pub const SMBFLG_CASELESS: c_uint = 0x08	/* all pathnames treated as caseless (off;
pub const SMBFLG_CANONICAL_PATH_FORMAT: c_uint = 0x10	/* obsolete */;
pub const SMBFLG_OLD_OPLOCK: c_uint = 0x20	/* obsolete */;
pub const SMBFLG_OLD_OPLOCK_NOTIFY: c_uint = 0x40	/* obsolete */;
pub const SMBFLG_RESPONSE: c_uint = 0x80	/* this PDU is a response from server */;
//
// SMB flag2 definitions
// See MS-CIFS 2.2.3.1
// MS-SMB 2.2.3.1
//

// Combinations of file access permission bits

//
// Invalid readdir handle
//
pub const CIFS_NO_HANDLE: c_uint = 0xFFFF;
pub const NO_CHANGE_64: c_uint = 0xFFFFFFFFFFFFFFFFULL;
// IPC$ in ASCII

// IPC$ in Unicode

// Unicode Null terminate 2 bytes of 0

pub const ASCII_NULL: c_uint = 0x00;
//
// Server type values (returned on EnumServer API
//
pub const CIFS_SV_TYPE_DC: c_uint = 0x00000008;
pub const CIFS_SV_TYPE_BACKDC: c_uint = 0x00000010;
//
// Alias type flags (From EnumAlias API call
//
pub const CIFS_ALIAS_TYPE_FILE: c_uint = 0x0001;
pub const CIFS_SHARE_TYPE_FILE: c_uint = 0x0000;
//
// File Attribute flags
//
pub const ATTR_READONLY: c_uint = 0x0001		/* See MS-CIFS 2.2.1.2.3 */;
pub const ATTR_HIDDEN: c_uint = 0x0002		/* See MS-CIFS 2.2.1.2.3 */;
pub const ATTR_SYSTEM: c_uint = 0x0004		/* See MS-CIFS 2.2.1.2.3 */;
pub const ATTR_VOLUME: c_uint = 0x0008;
pub const ATTR_DIRECTORY: c_uint = 0x0010		/* See MS-CIFS 2.2.1.2.3 */;
pub const ATTR_ARCHIVE: c_uint = 0x0020		/* See MS-CIFS 2.2.1.2.3 */;
pub const ATTR_DEVICE: c_uint = 0x0040;
pub const ATTR_NORMAL: c_uint = 0x0080		/* See MS-CIFS 2.2.1.2.3 */;
pub const ATTR_TEMPORARY: c_uint = 0x0100		/* See MS-CIFS 2.2.1.2.3 */;
pub const ATTR_SPARSE: c_uint = 0x0200		/* See MS-SMB 2.2.1.2.1 */;
pub const ATTR_REPARSE_POINT: c_uint = 0x0400		/* See MS-SMB 2.2.1.2.1 */;
pub const ATTR_COMPRESSED: c_uint = 0x0800		/* See MS-CIFS 2.2.1.2.3 */;
pub const ATTR_OFFLINE: c_uint = 0x1000		/* See MS-SMB 2.2.1.2.1;
pub const ATTR_NOT_CONTENT_INDEXED: c_uint = 0x2000		/* See MS-SMB 2.2.1.2.1 */;
pub const ATTR_ENCRYPTED: c_uint = 0x4000		/* See MS-SMB 2.2.1.2.1 */;
pub const ATTR_POSIX_SEMANTICS: c_uint = 0x0100000	/* See MS-CIFS 2.2.1.2.3 */;
pub const ATTR_BACKUP_SEMANTICS: c_uint = 0x0200000	/* See MS-CIFS 2.2.1.2.3 */;
pub const ATTR_DELETE_ON_CLOSE: c_uint = 0x0400000	/* See MS-CIFS 2.2.1.2.3 */;
pub const ATTR_SEQUENTIAL_SCAN: c_uint = 0x0800000	/* See MS-CIFS 2.2.1.2.3 */;
pub const ATTR_RANDOM_ACCESS: c_uint = 0x1000000	/* See MS-CIFS 2.2.1.2.3 */;
pub const ATTR_NO_BUFFERING: c_uint = 0x2000000	/* See MS-CIFS 2.2.1.2.3 */;
pub const ATTR_WRITE_THROUGH: c_uint = 0x8000000	/* See MS-CIFS 2.2.1.2.3 */;
// ShareAccess flags
pub const FILE_NO_SHARE: c_uint = 0x00000000;
pub const FILE_SHARE_READ: c_uint = 0x00000001;
pub const FILE_SHARE_WRITE: c_uint = 0x00000002;
pub const FILE_SHARE_DELETE: c_uint = 0x00000004;
pub const FILE_SHARE_ALL: c_uint = 0x00000007;
// CreateDisposition flags, similar to CreateAction as well
pub const FILE_SUPERSEDE: c_uint = 0x00000000;
pub const FILE_OPEN: c_uint = 0x00000001;
pub const FILE_CREATE: c_uint = 0x00000002;
pub const FILE_OPEN_IF: c_uint = 0x00000003;
pub const FILE_OVERWRITE: c_uint = 0x00000004;
pub const FILE_OVERWRITE_IF: c_uint = 0x00000005;
// CreateOptions
pub const CREATE_NOT_FILE: c_uint = 0x00000001	/* if set must not be file */;
pub const CREATE_WRITE_THROUGH: c_uint = 0x00000002;
pub const CREATE_SEQUENTIAL: c_uint = 0x00000004;
pub const CREATE_NO_BUFFER: c_uint = 0x00000008      /* should not buffer on srv */;
pub const CREATE_SYNC_ALERT: c_uint = 0x00000010	/* MBZ */;
pub const CREATE_ASYNC_ALERT: c_uint = 0x00000020	/* MBZ */;
pub const CREATE_NOT_DIR: c_uint = 0x00000040    /* if set must not be directory */;
pub const CREATE_TREE_CONNECTION: c_uint = 0x00000080	/* should be zero */;
pub const CREATE_COMPLETE_IF_OPLK: c_uint = 0x00000100	/* should be zero */;
pub const CREATE_NO_EA_KNOWLEDGE: c_uint = 0x00000200;
pub const CREATE_EIGHT_DOT_THREE: c_uint = 0x00000400	/* doc says this is obsolete;
pub const CREATE_OPEN_FOR_RECOVERY: c_uint = 0x00000400;
pub const CREATE_RANDOM_ACCESS: c_uint = 0x00000800;
pub const CREATE_DELETE_ON_CLOSE: c_uint = 0x00001000;
pub const CREATE_OPEN_BY_ID: c_uint = 0x00002000;
pub const CREATE_OPEN_BACKUP_INTENT: c_uint = 0x00004000;
pub const CREATE_NO_COMPRESSION: c_uint = 0x00008000;
pub const CREATE_RESERVE_OPFILTER: c_uint = 0x00100000	/* should be zero */;
pub const OPEN_REPARSE_POINT: c_uint = 0x00200000;
pub const OPEN_NO_RECALL: c_uint = 0x00400000;
pub const OPEN_FREE_SPACE_QUERY: c_uint = 0x00800000	/* should be zero */;
pub const CREATE_OPTIONS_MASK: c_uint = 0x007FFFFF;
pub const CREATE_OPTION_READONLY: c_uint = 0x10000000;
pub const CREATE_OPTION_SPECIAL: c_uint = 0x20000000   /* system. NB not sent over wire */;
// ImpersonationLevel flags
pub const SECURITY_ANONYMOUS: c_int = 0;
pub const SECURITY_IDENTIFICATION: c_int = 1;
pub const SECURITY_IMPERSONATION: c_int = 2;
pub const SECURITY_DELEGATION: c_int = 3;
// SecurityFlags
pub const SECURITY_CONTEXT_TRACKING: c_uint = 0x01;
pub const SECURITY_EFFECTIVE_ONLY: c_uint = 0x02;
//
// Default PID value, used in all SMBs where the PID is not important
//
pub const CIFS_DFT_PID: c_uint = 0x1234;
//
// We use the same routine for Copy and Move SMBs.  This flag is used to
// distinguish
//
pub const CIFS_COPY_OP: c_int = 1;
pub const CIFS_RENAME_OP: c_int = 2;
//
// Computer Name Length (since Netbios name was length 16 with last byte 0x20)
// No longer as important, now that TCP names are more commonly used to
// resolve hosts.
//
pub const CNLEN: c_int = 15;
//
// Share Name Length (SNLEN)
// Note:  This length was limited by the SMB used to get
// the Share info.   NetShareEnum only returned 13
// chars, including the null termination.
// This was removed because it no longer is limiting.
//
// Comment Length
//
pub const MAXCOMMENTLEN: c_int = 40;
//
// The OS/2 maximum path name
//
pub const MAX_PATHCONF: c_int = 256;
//
// SMB frame definitions  (following must be packed structs)
// See the SNIA CIFS Specification for details.
//
// The Naming convention is the lower case version of the
// smb command code name for the struct and this is typedef to the
// uppercase version of the same name with the prefix SMB_ removed
// for brevity.  Although typedefs are not commonly used for
// structure definitions in the Linux kernel, their use in the
// CIFS standards document, which this code is based on, may
// make this one of the cases where typedefs for structures make
// sense to improve readability for readers of the standards doc.
// Typedefs can always be removed later if they are too distracting
// and they are only used for the CIFSs PDUs themselves, not
// internal cifs vfs structures
//

pub const READ_RAW_ENABLE: c_int = 1;
pub const WRITE_RAW_ENABLE: c_int = 2;

// See MS-CIFS 2.2.4.52.2
// cap extended security off
// followed by Domain name - if extended security is off
// followed by 16 bytes of server GUID
// then security blob if cap_extended_security negotiated
// SecurityMode bits
pub const SECMODE_USER: c_uint = 0x01	/* off indicates share level security */;
pub const SECMODE_PW_ENCRYPT: c_uint = 0x02;
pub const SECMODE_SIGN_ENABLED: c_uint = 0x04	/* SMB security signatures enabled */;
pub const SECMODE_SIGN_REQUIRED: c_uint = 0x08	/* SMB security signatures required */;
// Negotiate response Capabilities
pub const CAP_RAW_MODE: c_uint = 0x00000001;
pub const CAP_MPX_MODE: c_uint = 0x00000002;
pub const CAP_UNICODE: c_uint = 0x00000004;
pub const CAP_LARGE_FILES: c_uint = 0x00000008;
pub const CAP_NT_SMBS: c_uint = 0x00000010	/* implies CAP_NT_FIND */;
pub const CAP_RPC_REMOTE_APIS: c_uint = 0x00000020;
pub const CAP_STATUS32: c_uint = 0x00000040;
pub const CAP_LEVEL_II_OPLOCKS: c_uint = 0x00000080;
pub const CAP_LOCK_AND_READ: c_uint = 0x00000100;
pub const CAP_NT_FIND: c_uint = 0x00000200;
pub const CAP_DFS: c_uint = 0x00001000;
pub const CAP_INFOLEVEL_PASSTHRU: c_uint = 0x00002000;
pub const CAP_LARGE_READ_X: c_uint = 0x00004000;
pub const CAP_LARGE_WRITE_X: c_uint = 0x00008000;
pub const CAP_LWIO: c_uint = 0x00010000 /* support fctl_srv_req_resume_key */;
pub const CAP_UNIX: c_uint = 0x00800000;
pub const CAP_COMPRESSED_DATA: c_uint = 0x02000000;
pub const CAP_DYNAMIC_REAUTH: c_uint = 0x20000000;
pub const CAP_PERSISTENT_HANDLES: c_uint = 0x40000000;
pub const CAP_EXTENDED_SECURITY: c_uint = 0x80000000;
// STRING NativeOS
// STRING NativeLanMan
// unsigned char * CaseSensitivePassword;
// STRING AccountName
// STRING PrimaryDomain
// STRING NativeOS
// STRING NativeLanMan
// unsigned char  * NativeOS;
// unsigned char  * NativeLanMan;
// unsigned char  * PrimaryDomain;
// STRING AccountName
// STRING PrimaryDomain
// STRING NativeOS
// STRING NativeLanMan
// unsigned char * NativeLanMan;
// unsigned char * PrimaryDomain;
// format of NLTMv2 Response ie "case sensitive password" hash when NTLMv2
pub const NTLMSSP_SERVER_TYPE: c_int = 1;
pub const NTLMSSP_DOMAIN_TYPE: c_int = 2;
pub const NTLMSSP_FQ_DOMAIN_TYPE: c_int = 3;
pub const NTLMSSP_DNS_DOMAIN_TYPE: c_int = 4;
pub const NTLMSSP_DNS_PARENT_TYPE: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ntlmssp2_name {
    pub type: __le16,
    pub length: __le16,
    pub data: [__u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ntlmv2_resp {
    pub ntlmv2_hash: [c_char; CIFS_ENCPWD_SIZE],
    pub reserved: [__u8; 8],
    pub key: [__u8; CIFS_SERVER_CHALLENGE_SIZE],
    pub challenge: } __packed,
    pub __packed: },
    pub blob_signature: __le32,
    pub reserved: __u32,
    pub time: __le64,
    pub /: *mut *mut __u64 client_chal; / random,
    pub reserved2: __u32,
// array of name entries could follow ending in minimum 4 byte struct
    pub __packed: },

//
// Capabilities bits (for NTLM SessSetup request)
// See MS-CIFS 2.2.4.52.2
// MS-SMB 2.2.4.5.2.1
//
pub const CAP_UNICODE: c_uint = 0x00000004;
pub const CAP_LARGE_FILES: c_uint = 0x00000008;
pub const CAP_NT_SMBS: c_uint = 0x00000010;
pub const CAP_STATUS32: c_uint = 0x00000040;
pub const CAP_LEVEL_II_OPLOCKS: c_uint = 0x00000080;
pub const CAP_NT_FIND: c_uint = 0x00000200	/* reserved should be zero;
pub const CAP_BULK_TRANSFER: c_uint = 0x00000400;
pub const CAP_EXTENDED_SECURITY: c_uint = 0x80000000;
// Action bits
pub const GUEST_LOGIN: c_int = 1;
    pub /: *mut *mut smb_hdr hdr; / wct = 4,
    pub AndXCommand: __u8,
    pub AndXReserved: __u8,
    pub AndXOffset: __le16,
    pub /: *mut *mut __le16 Flags; / see below,
    pub PasswordLength: __le16,
    pub ByteCount: __le16,
    pub /: *mut *mut unsigned char Password[]; / followed by,
// STRING Path    *//* \\server\share name
// STRING Service
    pub TCONX_REQ: } __packed,
    pub /: *mut *mut smb_hdr hdr; / wct = 3 , not extended response,
    pub AndXCommand: __u8,
    pub AndXReserved: __u8,
    pub AndXOffset: __le16,
    pub /: *mut *mut __le16 OptionalSupport; / see below,
    pub ByteCount: __u16,
    pub /: *mut *mut unsigned char Service[]; / always ASCII, not Unicode,
// STRING NativeFileSystem
    pub TCONX_RSP: } __packed,
    pub /: *mut *mut smb_hdr hdr; / wct = 7, extended response,
    pub AndXCommand: __u8,
    pub AndXReserved: __u8,
    pub AndXOffset: __le16,
    pub /: *mut *mut __le16 OptionalSupport; / see below,
    pub MaximalShareAccessRights: __le32,
    pub GuestMaximalShareAccessRights: __le32,
    pub ByteCount: __u16,
    pub /: *mut *mut unsigned char Service[]; / always ASCII, not Unicode,
// STRING NativeFileSystem
    pub TCONX_RSP_EXT: } __packed,
// tree connect Flags
pub const DISCONNECT_TID: c_uint = 0x0001;
pub const TCON_EXTENDED_SIGNATURES: c_uint = 0x0004;
pub const TCON_EXTENDED_SECINFO: c_uint = 0x0008;
// OptionalSupport bits
pub const SMB_SUPPORT_SEARCH_BITS: c_uint = 0x0001	/* "must have" directory search bits;
pub const SMB_SHARE_IS_IN_DFS: c_uint = 0x0002;
pub const SMB_CSC_MASK: c_uint = 0x000C;
// CSC flags defined as follows
pub const SMB_CSC_CACHE_MANUAL_REINT: c_uint = 0x0000;
pub const SMB_CSC_CACHE_AUTO_REINT: c_uint = 0x0004;
pub const SMB_CSC_CACHE_VDO: c_uint = 0x0008;
pub const SMB_CSC_NO_CACHING: c_uint = 0x000C;
pub const SMB_UNIQUE_FILE_NAME: c_uint = 0x0010;
pub const SMB_EXTENDED_SIGNATURES: c_uint = 0x0020;
// services
//
// A:       ie disk
// LPT1:    ie printer
// IPC      ie named pipe
// COMM
// ?????    ie any type
//
    pub hdr: smb_hdr,
    pub EchoCount: __le16,
    pub ByteCount: __le16,
    pub Data: [c_char; ],
    pub ECHO_REQ: } __packed,
    pub hdr: smb_hdr,
    pub SequenceNumber: __le16,
    pub ByteCount: __le16,
    pub Data: [c_char; ],
    pub ECHO_RSP: } __packed,
    pub /: *mut *mut smb_hdr hdr; / wct = 2,
    pub AndXCommand: __u8,
    pub AndXReserved: __u8,
    pub AndXOffset: __u16,
    pub ByteCount: __u16,
    pub LOGOFF_ANDX_REQ: } __packed,
    pub /: *mut *mut smb_hdr hdr; / wct = 2,
    pub AndXCommand: __u8,
    pub AndXReserved: __u8,
    pub AndXOffset: __u16,
    pub ByteCount: __u16,
    pub LOGOFF_ANDX_RSP: } __packed,
// tdis is probably simplest SMB PDU
    pub /: *mut *mut smb_hdr hdr; / wct = 0,
    pub /: *mut *mut __u16 ByteCount; / bcc = 0,
    pub req: } __packed,
    pub /: *mut *mut smb_hdr hdr; / wct = 0,
    pub /: *mut *mut __u16 ByteCount; / bcc = 0,
    pub resp: } __packed,
    pub TREE_DISCONNECT: } __packed,
    pub /: *mut *mut smb_hdr hdr; / wct = 3,
    pub FileID: __u16,
    pub /: *mut *mut __u32 LastWriteTime; / should be zero or -1,
    pub /: *mut *mut __u16 ByteCount; / 0,
    pub CLOSE_REQ: } __packed,
    pub /: *mut *mut smb_hdr hdr; / wct = 0,
    pub /: *mut *mut __u16 ByteCount; / bct = 0,
    pub CLOSE_RSP: } __packed,
    pub /: *mut *mut smb_hdr hdr; / wct = 1,
    pub FileID: __u16,
    pub /: *mut *mut __u16 ByteCount; / 0,
    pub FLUSH_REQ: } __packed,
    pub /: *mut *mut smb_hdr hdr; / wct = 1,
    pub FileID: __u16,
    pub /: *mut *mut __u16 ByteCount; / 0,
    pub FINDCLOSE_REQ: } __packed,
// OpenFlags
pub const REQ_MORE_INFO: c_uint = 0x00000001  /* legacy (OPEN_AND_X) only */;
pub const REQ_OPLOCK: c_uint = 0x00000002;
pub const REQ_BATCHOPLOCK: c_uint = 0x00000004;
pub const REQ_OPENDIRONLY: c_uint = 0x00000008;
pub const REQ_EXTENDED_INFO: c_uint = 0x00000010;
// File type
pub const DISK_TYPE: c_uint = 0x0000;
pub const BYTE_PIPE_TYPE: c_uint = 0x0001;
pub const MESSAGE_PIPE_TYPE: c_uint = 0x0002;
pub const PRINTER_TYPE: c_uint = 0x0003;
pub const COMM_DEV_TYPE: c_uint = 0x0004;
pub const UNKNOWN_TYPE: c_uint = 0xFFFF;
// Device Type or File Status Flags
pub const NO_EAS: c_uint = 0x0001;
pub const NO_SUBSTREAMS: c_uint = 0x0002;
pub const NO_REPARSETAG: c_uint = 0x0004;
// following flags can apply if pipe
pub const ICOUNT_MASK: c_uint = 0x00FF;
pub const PIPE_READ_MODE: c_uint = 0x0100;
pub const NAMED_PIPE_TYPE: c_uint = 0x0400;
pub const PIPE_END_POINT: c_uint = 0x4000;
pub const BLOCKING_NAMED_PIPE: c_uint = 0x8000;
    pub /: *mut *mut smb_hdr hdr; / wct = 24,
    pub AndXCommand: __u8,
    pub AndXReserved: __u8,
    pub AndXOffset: __le16,
    pub /: *mut *mut __u8 Reserved; / Must Be Zero,
    pub NameLength: __le16,
    pub OpenFlags: __le32,
    pub RootDirectoryFid: __u32,
    pub DesiredAccess: __le32,
    pub AllocationSize: __le64,
    pub FileAttributes: __le32,
    pub ShareAccess: __le32,
    pub CreateDisposition: __le32,
    pub CreateOptions: __le32,
    pub ImpersonationLevel: __le32,
    pub SecurityFlags: __u8,
    pub ByteCount: __le16,
    pub fileName: [c_char; ],
    pub OPEN_REQ: } __packed,
// open response: oplock levels
pub const OPLOCK_NONE: c_int = 0;
pub const OPLOCK_EXCLUSIVE: c_int = 1;
pub const OPLOCK_BATCH: c_int = 2;

// open response for CreateAction shifted left
pub const CIFS_CREATE_ACTION: c_uint = 0x20000 /* file created */;
    pub /: *mut *mut smb_hdr hdr; / wct = 34 BB,
    pub AndXCommand: __u8,
    pub AndXReserved: __u8,
    pub AndXOffset: __le16,
    pub OplockLevel: __u8,
    pub Fid: __u16,
    pub CreateAction: __le32,
    pub CreationTime: __le64,
    pub LastAccessTime: __le64,
    pub LastWriteTime: __le64,
    pub ChangeTime: __le64,
    pub FileAttributes: __le32,
    pub AllocationSize: __le64,
    pub EndOfFile: __le64,
    pub FileType: __le16,
    pub DeviceState: __le16,
    pub DirectoryFlag: __u8,
    pub /: *mut *mut __u16 ByteCount; / bct = 0,
    pub OPEN_RSP: } __packed,
    pub /: *mut *mut smb_hdr hdr; / wct = 42 but meaningless due to MS bug?,
    pub AndXCommand: __u8,
    pub AndXReserved: __u8,
    pub AndXOffset: __le16,
    pub OplockLevel: __u8,
    pub Fid: __u16,
    pub CreateAction: __le32,
    pub CreationTime: __le64,
    pub LastAccessTime: __le64,
    pub LastWriteTime: __le64,
    pub ChangeTime: __le64,
    pub FileAttributes: __le32,
    pub AllocationSize: __le64,
    pub EndOfFile: __le64,
    pub FileType: __le16,
    pub DeviceState: __le16,
    pub DirectoryFlag: __u8,
    pub VolumeGUID: [__u8; 16],
    pub /: *mut *mut __u64 FileId; / note no endian conversion - is opaque UniqueID,
    pub MaximalAccessRights: __le32,
    pub GuestMaximalAccessRights: __le32,
    pub /: *mut *mut __u16 ByteCount; / bct = 0,
    pub OPEN_RSP_EXT: } __packed,
// format of legacy open request
    pub /: *mut *mut smb_hdr hdr; / wct = 15,
    pub AndXCommand: __u8,
    pub AndXReserved: __u8,
    pub AndXOffset: __le16,
    pub OpenFlags: __le16,
    pub Mode: __le16,
    pub /: *mut *mut __le16 Sattr; / search attributes,
    pub /: *mut *mut __le16 FileAttributes; / dos attrs,
    pub /: *mut *mut __le32 CreateTime; / os2 format,
    pub OpenFunction: __le16,
    pub EndOfFile: __le32,
    pub Timeout: __le32,
    pub Reserved: __le32,
    pub /: *mut *mut __le16 ByteCount; / file name follows,
    pub fileName: [c_char; ],
    pub OPENX_REQ: } __packed,
    pub /: *mut *mut smb_hdr hdr; / wct = 15,
    pub AndXCommand: __u8,
    pub AndXReserved: __u8,
    pub AndXOffset: __le16,
    pub Fid: __u16,
    pub FileAttributes: __le16,
    pub /: *mut *mut __le32 LastWriteTime; / os2 format,
    pub EndOfFile: __le32,
    pub Access: __le16,
    pub FileType: __le16,
    pub IPCState: __le16,
    pub Action: __le16,
    pub FileId: __u32,
    pub Reserved: __u16,
    pub ByteCount: __u16,
    pub OPENX_RSP: } __packed,
// For encoding of POSIX Open Request - see trans2 function 0x209 data struct
// Legacy write request for older servers
    pub /: *mut *mut smb_hdr hdr; / wct = 12,
    pub AndXCommand: __u8,
    pub AndXReserved: __u8,
    pub AndXOffset: __le16,
    pub Fid: __u16,
    pub OffsetLow: __le32,
    pub /: *mut *mut __u32 Reserved; / Timeout,
    pub /: *mut *mut __le16 WriteMode; / 1 = write through,
    pub Remaining: __le16,
    pub Reserved2: __le16,
    pub DataLengthLow: __le16,
    pub DataOffset: __le16,
    pub ByteCount: __le16,
    pub DWORD: *mut *mut __u8 Pad; / BB check for whether padded to,
    pub Data: [c_char; ],
    pub WRITEX_REQ: } __packed,
    pub /: *mut *mut smb_hdr hdr; / wct = 14,
    pub AndXCommand: __u8,
    pub AndXReserved: __u8,
    pub AndXOffset: __le16,
    pub Fid: __u16,
    pub OffsetLow: __le32,
    pub Reserved: __u32,
    pub WriteMode: __le16,
    pub Remaining: __le16,
    pub DataLengthHigh: __le16,
    pub DataLengthLow: __le16,
    pub DataOffset: __le16,
    pub OffsetHigh: __le32,
    pub ByteCount: __le16,
    pub DWORD: *mut *mut __u8 Pad; / BB check for whether padded to,
    pub Data: [c_char; ],
    pub WRITE_REQ: } __packed,
    pub /: *mut *mut smb_hdr hdr; / wct = 6,
    pub AndXCommand: __u8,
    pub AndXReserved: __u8,
    pub AndXOffset: __le16,
    pub Count: __le16,
    pub Remaining: __le16,
    pub CountHigh: __le16,
    pub Reserved: __u16,
    pub ByteCount: __u16,
    pub WRITE_RSP: } __packed,
// legacy read request for older servers
    pub /: *mut *mut smb_hdr hdr; / wct = 10,
    pub AndXCommand: __u8,
    pub AndXReserved: __u8,
    pub AndXOffset: __le16,
    pub Fid: __u16,
    pub OffsetLow: __le32,
    pub MaxCount: __le16,
    pub /: *mut *mut __le16 MinCount; / obsolete,
    pub Reserved: __le32,
    pub Remaining: __le16,
    pub ByteCount: __le16,
    pub READX_REQ: } __packed,
    pub /: *mut *mut smb_hdr hdr; / wct = 12,
    pub AndXCommand: __u8,
    pub AndXReserved: __u8,
    pub AndXOffset: __le16,
    pub Fid: __u16,
    pub OffsetLow: __le32,
    pub MaxCount: __le16,
    pub /: *mut *mut __le16 MinCount; / obsolete,
    pub MaxCountHigh: __le32,
    pub Remaining: __le16,
    pub OffsetHigh: __le32,
    pub ByteCount: __le16,
    pub READ_REQ: } __packed,
    pub /: *mut *mut smb_hdr hdr; / wct = 12,
    pub AndXCommand: __u8,
    pub AndXReserved: __u8,
    pub AndXOffset: __le16,
    pub Remaining: __le16,
    pub DataCompactionMode: __le16,
    pub Reserved: __le16,
    pub DataLength: __le16,
    pub DataOffset: __le16,
    pub DataLengthHigh: __le16,
    pub Reserved2: __u64,
    pub ByteCount: __u16,
// read response data immediately follows
    pub READ_RSP: } __packed,
    pub Pid: __le16,
    pub Pad: __le16,
    pub OffsetHigh: __le32,
    pub OffsetLow: __le32,
    pub LengthHigh: __le32,
    pub LengthLow: __le32,
    pub LOCKING_ANDX_RANGE: } __packed,
pub const LOCKING_ANDX_SHARED_LOCK: c_uint = 0x01;
pub const LOCKING_ANDX_OPLOCK_RELEASE: c_uint = 0x02;
pub const LOCKING_ANDX_CHANGE_LOCKTYPE: c_uint = 0x04;
pub const LOCKING_ANDX_CANCEL_LOCK: c_uint = 0x08;
pub const LOCKING_ANDX_LARGE_FILES: c_uint = 0x10	/* always on for us */;
    pub /: *mut *mut smb_hdr hdr; / wct = 8,
    pub AndXCommand: __u8,
    pub AndXReserved: __u8,
    pub AndXOffset: __le16,
    pub Fid: __u16,
    pub LockType: __u8,
    pub OplockLevel: __u8,
    pub Timeout: __le32,
    pub NumberOfUnlocks: __le16,
    pub NumberOfLocks: __le16,
    pub ByteCount: __le16,
    pub Locks: [LOCKING_ANDX_RANGE; ],
    pub LOCK_REQ: } __packed,
// lock type
pub const CIFS_RDLCK: c_int = 0;
pub const CIFS_WRLCK: c_int = 1;
pub const CIFS_UNLCK: c_int = 2;
    pub /: *mut *mut __le16 lock_type; / 0 = Read, 1 = Write, 2 = Unlock,
    pub /: *mut *mut __le16 lock_flags; / 1 = Wait (only valid for setlock),
    pub pid: __le32,
    pub start: __le64,
    pub length: __le64,
// BB what about additional owner info to identify network client
    pub CIFS_POSIX_LOCK: } __packed,
    pub /: *mut *mut smb_hdr hdr; / wct = 2,
    pub AndXCommand: __u8,
    pub AndXReserved: __u8,
    pub AndXOffset: __le16,
    pub ByteCount: __u16,
    pub LOCK_RSP: } __packed,
    pub /: *mut *mut smb_hdr hdr; / wct = 1,
    pub /: *mut *mut __le16 SearchAttributes; / target file attributes,
    pub ByteCount: __le16,
    pub /: *mut *mut __u8 BufferFormat; / 4 = ASCII or Unicode,
    pub OldFileName: [c_uchar; ],
// followed by __u8 BufferFormat2
// followed by NewFileName
    pub RENAME_REQ: } __packed,
// copy request flags
pub const COPY_MUST_BE_FILE: c_uint = 0x0001;
pub const COPY_MUST_BE_DIR: c_uint = 0x0002;
pub const COPY_TARGET_MODE_ASCII: c_uint = 0x0004 /* if not set, binary */;
pub const COPY_SOURCE_MODE_ASCII: c_uint = 0x0008 /* if not set, binary */;
pub const COPY_VERIFY_WRITES: c_uint = 0x0010;
pub const COPY_TREE: c_uint = 0x0020;
    pub /: *mut *mut smb_hdr hdr; / wct = 3,
    pub Tid2: __u16,
    pub OpenFunction: __le16,
    pub Flags: __le16,
    pub ByteCount: __le16,
    pub /: *mut *mut __u8 BufferFormat; / 4 = ASCII or Unicode,
    pub OldFileName: [c_uchar; ],
// followed by __u8 BufferFormat2
// followed by NewFileName string
    pub COPY_REQ: } __packed,
    pub /: *mut *mut smb_hdr hdr; / wct = 1,
    pub /: *mut *mut __le16 CopyCount; / number of files copied,
    pub /: *mut *mut __u16 ByteCount; / may be zero,
    pub /: *mut *mut __u8 BufferFormat; / 0x04 - only present if errored file follows,
    pub /: *mut *mut unsigned char ErrorFileName[]; / only present if error in copy,
    pub COPY_RSP: } __packed,
pub const CREATE_HARD_LINK: c_uint = 0x103;
pub const MOVEFILE_COPY_ALLOWED: c_uint = 0x0002;
pub const MOVEFILE_REPLACE_EXISTING: c_uint = 0x0001;
    pub /: *mut *mut smb_hdr hdr; / wct = 4,
    pub /: *mut *mut __le16 SearchAttributes; / target file attributes,
    pub /: *mut *mut __le16 Flags; / spec says Information Level,
    pub ClusterCount: __le32,
    pub ByteCount: __le16,
    pub /: *mut *mut __u8 BufferFormat; / 4 = ASCII or Unicode,
    pub OldFileName: [c_uchar; ],
// followed by __u8 BufferFormat2
// followed by NewFileName
    pub NT_RENAME_REQ: } __packed,
    pub /: *mut *mut smb_hdr hdr; / wct = 0,
    pub /: *mut *mut __u16 ByteCount; / bct = 0,
    pub RENAME_RSP: } __packed,
    pub /: *mut *mut smb_hdr hdr; / wct = 1,
    pub SearchAttributes: __le16,
    pub ByteCount: __le16,
    pub /: *mut *mut __u8 BufferFormat; / 4 = ASCII,
    pub fileName: [c_uchar; ],
    pub DELETE_FILE_REQ: } __packed,
    pub /: *mut *mut smb_hdr hdr; / wct = 0,
    pub /: *mut *mut __u16 ByteCount; / bct = 0,
    pub DELETE_FILE_RSP: } __packed,
    pub /: *mut *mut smb_hdr hdr; / wct = 0,
    pub ByteCount: __le16,
    pub /: *mut *mut __u8 BufferFormat; / 4 = ASCII,
    pub DirName: [c_uchar; ],
    pub DELETE_DIRECTORY_REQ: } __packed,
    pub /: *mut *mut smb_hdr hdr; / wct = 0,
    pub /: *mut *mut __u16 ByteCount; / bct = 0,
    pub DELETE_DIRECTORY_RSP: } __packed,
    pub /: *mut *mut smb_hdr hdr; / wct = 0,
    pub ByteCount: __le16,
    pub /: *mut *mut __u8 BufferFormat; / 4 = ASCII,
    pub DirName: [c_uchar; ],
    pub CREATE_DIRECTORY_REQ: } __packed,
    pub /: *mut *mut smb_hdr hdr; / wct = 0,
    pub /: *mut *mut __u16 ByteCount; / bct = 0,
    pub CREATE_DIRECTORY_RSP: } __packed,
    pub /: *mut *mut smb_hdr hdr; / wct = 0,
    pub /: *mut *mut __le16 ByteCount; / 1 + namelen + 1,
    pub /: *mut *mut __u8 BufferFormat; / 4 = ASCII,
    pub FileName: [c_uchar; ],
    pub QUERY_INFORMATION_REQ: } __packed,
    pub /: *mut *mut smb_hdr hdr; / wct = 10,
    pub attr: __le16,
    pub last_write_time: __le32,
    pub size: __le32,
    pub reserved: [__u16; 5],
    pub /: *mut *mut __le16 ByteCount; / bcc = 0,
    pub QUERY_INFORMATION_RSP: } __packed,
    pub /: *mut *mut smb_hdr hdr; / wct = 8,
    pub attr: __le16,
    pub last_write_time: __le32,
    pub /: *mut *mut __le16 reserved[5]; / must be zero,
    pub ByteCount: __le16,
    pub /: *mut *mut __u8 BufferFormat; / 4 = ASCII,
    pub fileName: [c_uchar; ],
    pub SETATTR_REQ: } __packed,
    pub /: *mut *mut smb_hdr hdr; / wct = 0,
    pub /: *mut *mut __u16 ByteCount; / bct = 0,
    pub SETATTR_RSP: } __packed,
// empty wct response to setattr
//
// NT Transact structure definitions follow
// Currently only ioctl, acl (get security descriptor)
// and notify are implemented
//
    pub /: *mut *mut smb_hdr hdr; / wct >= 19,
    pub MaxSetupCount: __u8,
    pub Reserved: __u16,
    pub TotalParameterCount: __le32,
    pub TotalDataCount: __le32,
    pub MaxParameterCount: __le32,
    pub MaxDataCount: __le32,
    pub ParameterCount: __le32,
    pub ParameterOffset: __le32,
    pub DataCount: __le32,
    pub DataOffset: __le32,
    pub /: *mut *mut __u8 SetupCount; / four setup words follow subcommand,
// SNIA spec incorrectly included spurious pad here
    pub /: *mut *mut __le16 SubCommand; / 2 = IOCTL/FSCTL,
// SetupCount words follow then
    pub ByteCount: __le16,
    pub Pad: [__u8; 3],
    pub Parms: [__u8; ],
    pub NTRANSACT_REQ: } __packed,
    pub /: *mut *mut smb_hdr hdr; / wct = 18,
    pub Reserved: [__u8; 3],
    pub TotalParameterCount: __le32,
    pub TotalDataCount: __le32,
    pub ParameterCount: __le32,
    pub ParameterOffset: __le32,
    pub ParameterDisplacement: __le32,
    pub DataCount: __le32,
    pub DataOffset: __le32,
    pub DataDisplacement: __le32,
    pub /: *mut *mut __u8 SetupCount; / 0,
    pub ByteCount: __u16,
// __u8 Pad[3];
// parms and data follow
    pub NTRANSACT_RSP: } __packed,
    pub /: *mut *mut smb_hdr hdr; / wct = 23,
    pub MaxSetupCount: __u8,
    pub Reserved: __u16,
    pub TotalParameterCount: __le32,
    pub TotalDataCount: __le32,
    pub MaxParameterCount: __le32,
    pub MaxDataCount: __le32,
    pub ParameterCount: __le32,
    pub ParameterOffset: __le32,
    pub DataCount: __le32,
    pub DataOffset: __le32,
    pub /: *mut *mut __u8 SetupCount; / four setup words follow subcommand,
// SNIA spec incorrectly included spurious pad here
    pub /: *mut *mut __le16 SubCommand; / 2 = IOCTL/FSCTL,
    pub FunctionCode: __le32,
    pub Fid: __u16,
    pub /: *mut *mut __u8 IsFsctl; / 1 = File System Control 0 = device control (IOCTL),
    pub /: *mut *mut __u8 IsRootFlag; / 1 = apply command to root of share (must be DFS),
    pub ByteCount: __le16,
    pub Pad: [__u8; 3],
    pub Data: [__u8; ],
    pub TRANSACT_IOCTL_REQ: } __packed,
    pub /: *mut *mut smb_hdr hdr; / wct = 23,
    pub MaxSetupCount: __u8,
    pub Reserved: __u16,
    pub TotalParameterCount: __le32,
    pub TotalDataCount: __le32,
    pub MaxParameterCount: __le32,
    pub MaxDataCount: __le32,
    pub ParameterCount: __le32,
    pub ParameterOffset: __le32,
    pub DataCount: __le32,
    pub DataOffset: __le32,
    pub /: *mut *mut __u8 SetupCount; / four setup words follow subcommand,
// SNIA spec incorrectly included spurious pad here
    pub /: *mut *mut __le16 SubCommand; / 2 = IOCTL/FSCTL,
    pub FunctionCode: __le32,
    pub Fid: __u16,
    pub /: *mut *mut __u8 IsFsctl; / 1 = File System Control 0 = device control (IOCTL),
    pub /: *mut *mut __u8 IsRootFlag; / 1 = apply command to root of share (must be DFS),
    pub ByteCount: __le16,
    pub Pad: [__u8; 3],
    pub /: *mut *mut __le16 compression_state; / See below for valid flags,
    pub TRANSACT_COMPR_IOCTL_REQ: } __packed,
    pub /: *mut *mut smb_hdr hdr; / wct = 19,
    pub Reserved: [__u8; 3],
    pub TotalParameterCount: __le32,
    pub TotalDataCount: __le32,
    pub ParameterCount: __le32,
    pub ParameterOffset: __le32,
    pub ParameterDisplacement: __le32,
    pub DataCount: __le32,
    pub DataOffset: __le32,
    pub DataDisplacement: __le32,
    pub /: *mut *mut __u8 SetupCount; / 1,
    pub ReturnedDataLen: __le16,
    pub ByteCount: __le16,
    pub TRANSACT_IOCTL_RSP: } __packed,
pub const CIFS_ACL_OWNER: c_int = 1;
pub const CIFS_ACL_GROUP: c_int = 2;
pub const CIFS_ACL_DACL: c_int = 4;
pub const CIFS_ACL_SACL: c_int = 8;
    pub /: *mut *mut smb_hdr hdr; / wct = 19,
    pub MaxSetupCount: __u8,
    pub Reserved: __u16,
    pub TotalParameterCount: __le32,
    pub TotalDataCount: __le32,
    pub MaxParameterCount: __le32,
    pub MaxDataCount: __le32,
    pub ParameterCount: __le32,
    pub ParameterOffset: __le32,
    pub DataCount: __le32,
    pub DataOffset: __le32,
    pub /: *mut *mut __u8 SetupCount; / no setup words follow subcommand,
// SNIA spec incorrectly included spurious pad here
    pub /: *mut *mut __le16 SubCommand; / 6 = QUERY_SECURITY_DESC,
    pub /: *mut *mut __le16 ByteCount; / bcc = 3 + 8,
    pub Pad: [__u8; 3],
    pub Fid: __u16,
    pub Reserved2: __u16,
    pub AclFlags: __le32,
    pub QUERY_SEC_DESC_REQ: } __packed,
    pub /: *mut *mut smb_hdr hdr; / wct = 19,
    pub MaxSetupCount: __u8,
    pub Reserved: __u16,
    pub TotalParameterCount: __le32,
    pub TotalDataCount: __le32,
    pub MaxParameterCount: __le32,
    pub MaxDataCount: __le32,
    pub ParameterCount: __le32,
    pub ParameterOffset: __le32,
    pub DataCount: __le32,
    pub DataOffset: __le32,
    pub /: *mut *mut __u8 SetupCount; / no setup words follow subcommand,
// SNIA spec incorrectly included spurious pad here
    pub /: *mut *mut __le16 SubCommand; / 3 = SET_SECURITY_DESC,
    pub /: *mut *mut __le16 ByteCount; / bcc = 3 + 8,
    pub Pad: [__u8; 3],
    pub Fid: __u16,
    pub Reserved2: __u16,
    pub AclFlags: __le32,
    pub SET_SEC_DESC_REQ: } __packed,
    pub /: *mut *mut smb_hdr hdr; / wct = 23,
    pub MaxSetupCount: __u8,
    pub Reserved: __u16,
    pub TotalParameterCount: __le32,
    pub TotalDataCount: __le32,
    pub MaxParameterCount: __le32,
    pub MaxDataCount: __le32,
    pub ParameterCount: __le32,
    pub ParameterOffset: __le32,
    pub DataCount: __le32,
    pub DataOffset: __le32,
    pub /: *mut *mut __u8 SetupCount; / four setup words follow subcommand,
// SNIA spec incorrectly included spurious pad here
    pub /: *mut *mut __le16 SubCommand;/ 4 = Change Notify,
    pub /: *mut *mut __le32 CompletionFilter; / operation to monitor,
    pub Fid: __u16,
    pub /: *mut *mut __u8 WatchTree; / 1 = Monitor subdirectories,
    pub Reserved2: __u8,
    pub ByteCount: __le16,
// __u8 Pad[3];
// __u8 Data[];
    pub TRANSACT_CHANGE_NOTIFY_REQ: } __packed,
// BB eventually change to use generic ntransact rsp struct
    pub /: *mut *mut smb_hdr hdr; / wct = 18,
    pub Reserved: [__u8; 3],
    pub TotalParameterCount: __le32,
    pub TotalDataCount: __le32,
    pub ParameterCount: __le32,
    pub ParameterOffset: __le32,
    pub ParameterDisplacement: __le32,
    pub DataCount: __le32,
    pub DataOffset: __le32,
    pub DataDisplacement: __le32,
    pub /: *mut *mut __u8 SetupCount; / 0,
    pub ByteCount: __u16,
// __u8 Pad[3];
    pub TRANSACT_CHANGE_NOTIFY_RSP: } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cifs_quota_data {
    pub /: *mut *mut __u32 rsrvd1; / 0,
    pub sid_size: __u32,
    pub /: *mut *mut __u64 rsrvd2; / 0,
    pub space_used: __u64,
    pub soft_limit: __u64,
    pub hard_limit: __u64,
    pub /: *mut *mut char sid[]; / variable size?,
    pub __packed: },
// quota sub commands
pub const QUOTA_LIST_CONTINUE: c_int = 0;
pub const QUOTA_LIST_START: c_uint = 0x100;
pub const QUOTA_FOR_SID: c_uint = 0x101;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trans2_req {
// struct smb_hdr hdr precedes. Set wct = 14+
    pub TotalParameterCount: __le16,
    pub TotalDataCount: __le16,
    pub MaxParameterCount: __le16,
    pub MaxDataCount: __le16,
    pub MaxSetupCount: __u8,
    pub Reserved: __u8,
    pub Flags: __le16,
    pub Timeout: __le32,
    pub Reserved2: __u16,
    pub ParameterCount: __le16,
    pub ParameterOffset: __le16,
    pub DataCount: __le16,
    pub DataOffset: __le16,
    pub SetupCount: __u8,
    pub Reserved3: __u8,
    pub /: *mut *mut __le16 SubCommand; / 1st setup word - SetupCount words follow,
    pub ByteCount: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb_t2_req {
    pub hdr: smb_hdr,
    pub t2_req: trans2_req,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trans2_resp {
// struct smb_hdr hdr precedes. Note wct = 10 + setup count
    pub TotalParameterCount: __le16,
    pub TotalDataCount: __le16,
    pub Reserved: __u16,
    pub ParameterCount: __le16,
    pub ParameterOffset: __le16,
    pub ParameterDisplacement: __le16,
    pub DataCount: __le16,
    pub DataOffset: __le16,
    pub DataDisplacement: __le16,
    pub SetupCount: __u8,
    pub Reserved1: __u8,
// SetupWords[SetupCount];
    pub ByteCount: __u16,
    pub Reserved2;*/: *mut __u16,
// data area follows
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb_t2_rsp {
    pub hdr: smb_hdr,
    pub t2_rsp: trans2_resp,
    pub __packed: },
// PathInfo/FileInfo infolevels
pub const SMB_INFO_STANDARD: c_int = 1;
pub const SMB_SET_FILE_EA: c_int = 2;
pub const SMB_QUERY_FILE_EA_SIZE: c_int = 2;
pub const SMB_INFO_QUERY_EAS_FROM_LIST: c_int = 3;
pub const SMB_INFO_QUERY_ALL_EAS: c_int = 4;
pub const SMB_INFO_IS_NAME_VALID: c_int = 6;
pub const SMB_QUERY_FILE_BASIC_INFO: c_uint = 0x101;
pub const SMB_QUERY_FILE_STANDARD_INFO: c_uint = 0x102;
pub const SMB_QUERY_FILE_EA_INFO: c_uint = 0x103;
pub const SMB_QUERY_FILE_NAME_INFO: c_uint = 0x104;
pub const SMB_QUERY_FILE_ALLOCATION_INFO: c_uint = 0x105;
pub const SMB_QUERY_FILE_END_OF_FILEINFO: c_uint = 0x106;
pub const SMB_QUERY_FILE_ALL_INFO: c_uint = 0x107;
pub const SMB_QUERY_ALT_NAME_INFO: c_uint = 0x108;
pub const SMB_QUERY_FILE_STREAM_INFO: c_uint = 0x109;
pub const SMB_QUERY_FILE_COMPRESSION_INFO: c_uint = 0x10B;
pub const SMB_QUERY_FILE_UNIX_BASIC: c_uint = 0x200;
pub const SMB_QUERY_FILE_UNIX_LINK: c_uint = 0x201;
pub const SMB_QUERY_POSIX_ACL: c_uint = 0x204;
pub const SMB_QUERY_XATTR: c_uint = 0x205  /* e.g. system EA name space */;
pub const SMB_QUERY_ATTR_FLAGS: c_uint = 0x206  /* append,immutable etc. */;
pub const SMB_QUERY_POSIX_PERMISSION: c_uint = 0x207;
pub const SMB_QUERY_POSIX_LOCK: c_uint = 0x208;
// #define SMB_POSIX_OPEN               0x209
// #define SMB_POSIX_UNLINK             0x20a
pub const SMB_QUERY_FILE__UNIX_INFO2: c_uint = 0x20b;
pub const SMB_QUERY_FILE_INTERNAL_INFO: c_uint = 0x3ee;
pub const SMB_QUERY_FILE_ACCESS_INFO: c_uint = 0x3f0;
pub const SMB_QUERY_FILE_NAME_INFO2: c_uint = 0x3f1 /* 0x30 bytes */;
pub const SMB_QUERY_FILE_POSITION_INFO: c_uint = 0x3f6;
pub const SMB_QUERY_FILE_MODE_INFO: c_uint = 0x3f8;
pub const SMB_QUERY_FILE_ALGN_INFO: c_uint = 0x3f9;
pub const SMB_SET_FILE_BASIC_INFO: c_uint = 0x101;
pub const SMB_SET_FILE_DISPOSITION_INFO: c_uint = 0x102;
pub const SMB_SET_FILE_ALLOCATION_INFO: c_uint = 0x103;
pub const SMB_SET_FILE_END_OF_FILE_INFO: c_uint = 0x104;
pub const SMB_SET_FILE_UNIX_BASIC: c_uint = 0x200;
pub const SMB_SET_FILE_UNIX_LINK: c_uint = 0x201;
pub const SMB_SET_FILE_UNIX_HLINK: c_uint = 0x203;
pub const SMB_SET_POSIX_ACL: c_uint = 0x204;
pub const SMB_SET_XATTR: c_uint = 0x205;
pub const SMB_SET_ATTR_FLAGS: c_uint = 0x206  /* append, immutable etc. */;
pub const SMB_SET_POSIX_LOCK: c_uint = 0x208;
pub const SMB_POSIX_OPEN: c_uint = 0x209;
pub const SMB_POSIX_UNLINK: c_uint = 0x20a;
pub const SMB_SET_FILE_UNIX_INFO2: c_uint = 0x20b;
pub const SMB_SET_FILE_BASIC_INFO2: c_uint = 0x3ec;
pub const SMB_SET_FILE_RENAME_INFORMATION: c_uint = 0x3f2 /* BB check if qpathinfo too */;
pub const SMB_FILE_ALL_INFO2: c_uint = 0x3fa;
pub const SMB_SET_FILE_ALLOCATION_INFO2: c_uint = 0x3fb;
pub const SMB_SET_FILE_END_OF_FILE_INFO2: c_uint = 0x3fc;
pub const SMB_FILE_MOVE_CLUSTER_INFO: c_uint = 0x407;
pub const SMB_FILE_QUOTA_INFO: c_uint = 0x408;
pub const SMB_FILE_REPARSEPOINT_INFO: c_uint = 0x409;
pub const SMB_FILE_MAXIMUM_INFO: c_uint = 0x40d;
// Find File infolevels
pub const SMB_FIND_FILE_INFO_STANDARD: c_uint = 0x001;
pub const SMB_FIND_FILE_QUERY_EA_SIZE: c_uint = 0x002;
pub const SMB_FIND_FILE_QUERY_EAS_FROM_LIST: c_uint = 0x003;
pub const SMB_FIND_FILE_DIRECTORY_INFO: c_uint = 0x101;
pub const SMB_FIND_FILE_FULL_DIRECTORY_INFO: c_uint = 0x102;
pub const SMB_FIND_FILE_NAMES_INFO: c_uint = 0x103;
pub const SMB_FIND_FILE_BOTH_DIRECTORY_INFO: c_uint = 0x104;
pub const SMB_FIND_FILE_ID_FULL_DIR_INFO: c_uint = 0x105;
pub const SMB_FIND_FILE_ID_BOTH_DIR_INFO: c_uint = 0x106;
pub const SMB_FIND_FILE_UNIX: c_uint = 0x202;
// #define SMB_FIND_FILE_POSIX_INFO          0x064
    pub /: *mut *mut smb_hdr hdr; / wct = 14+,
    pub TotalParameterCount: __le16,
    pub TotalDataCount: __le16,
    pub MaxParameterCount: __le16,
    pub MaxDataCount: __le16,
    pub MaxSetupCount: __u8,
    pub Reserved: __u8,
    pub Flags: __le16,
    pub Timeout: __le32,
    pub Reserved2: __u16,
    pub ParameterCount: __le16,
    pub ParameterOffset: __le16,
    pub DataCount: __le16,
    pub DataOffset: __le16,
    pub SetupCount: __u8,
    pub Reserved3: __u8,
    pub /: *mut *mut __le16 SubCommand; / one setup word,
    pub ByteCount: __le16,
    pub Pad: __u8,
    pub InformationLevel: __le16,
    pub Reserved4: __u32,
    pub FileName: [c_char; ],
    pub TRANSACTION2_QPI_REQ: } __packed,
    pub /: *mut *mut smb_hdr hdr; / wct = 10 + SetupCount,
    pub t2: trans2_resp,
    pub ByteCount: __u16,
    pub /: *mut *mut __u16 Reserved2; / parameter word is present for infolevels > 100,
    pub TRANSACTION2_QPI_RSP: } __packed,
    pub /: *mut *mut smb_hdr hdr; / wct = 15,
    pub TotalParameterCount: __le16,
    pub TotalDataCount: __le16,
    pub MaxParameterCount: __le16,
    pub MaxDataCount: __le16,
    pub MaxSetupCount: __u8,
    pub Reserved: __u8,
    pub Flags: __le16,
    pub Timeout: __le32,
    pub Reserved2: __u16,
    pub ParameterCount: __le16,
    pub ParameterOffset: __le16,
    pub DataCount: __le16,
    pub DataOffset: __le16,
    pub SetupCount: __u8,
    pub Reserved3: __u8,
    pub /: *mut *mut __le16 SubCommand; / one setup word,
    pub ByteCount: __le16,
    pub Pad: __u8,
    pub Pad1: __u16,
    pub InformationLevel: __le16,
    pub Reserved4: __u32,
    pub FileName: [c_char; ],
    pub TRANSACTION2_SPI_REQ: } __packed,
    pub /: *mut *mut smb_hdr hdr; / wct = 10 + SetupCount,
    pub t2: trans2_resp,
    pub ByteCount: __u16,
    pub /: *mut *mut __u16 Reserved2; / parameter word is present for infolevels > 100,
    pub TRANSACTION2_SPI_RSP: } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct set_file_rename {
    pub /: *mut *mut __le32 overwrite; / 1 = overwrite dest,
    pub /: *mut *mut __u32 root_fid; / zero,
    pub target_name_len: __le32,
    pub /: *mut *mut char target_name[]; / Must be unicode,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb_com_transaction2_sfi_req {
    pub /: *mut *mut smb_hdr hdr; / wct = 15,
    pub TotalParameterCount: __le16,
    pub TotalDataCount: __le16,
    pub MaxParameterCount: __le16,
    pub MaxDataCount: __le16,
    pub MaxSetupCount: __u8,
    pub Reserved: __u8,
    pub Flags: __le16,
    pub Timeout: __le32,
    pub Reserved2: __u16,
    pub ParameterCount: __le16,
    pub ParameterOffset: __le16,
    pub DataCount: __le16,
    pub DataOffset: __le16,
    pub SetupCount: __u8,
    pub Reserved3: __u8,
    pub /: *mut *mut __le16 SubCommand; / one setup word,
    pub ByteCount: __le16,
    pub Pad: __u8,
    pub Pad1: __u16,
    pub Fid: __u16,
    pub InformationLevel: __le16,
    pub Reserved4: __u16,
    pub payload: [__u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb_com_transaction2_sfi_rsp {
    pub /: *mut *mut smb_hdr hdr; / wct = 10 + SetupCount,
    pub t2: trans2_resp,
    pub ByteCount: __u16,
    pub /: *mut *mut __u16 Reserved2; / parameter word reserved - present for infolevels > 100,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb_t2_qfi_req {
    pub hdr: smb_hdr,
    pub t2: trans2_req,
    pub Pad: __u8,
    pub Fid: __u16,
    pub InformationLevel: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb_t2_qfi_rsp {
    pub /: *mut *mut smb_hdr hdr; / wct = 10 + SetupCount,
    pub t2: trans2_resp,
    pub ByteCount: __u16,
    pub /: *mut *mut __u16 Reserved2; / parameter word reserved - present for infolevels > 100,
    pub __packed: },
//
// Flags on T2 FINDFIRST and FINDNEXT
//
pub const CIFS_SEARCH_CLOSE_ALWAYS: c_uint = 0x0001;
pub const CIFS_SEARCH_CLOSE_AT_END: c_uint = 0x0002;
pub const CIFS_SEARCH_RETURN_RESUME: c_uint = 0x0004;
pub const CIFS_SEARCH_CONTINUE_FROM_LAST: c_uint = 0x0008;
pub const CIFS_SEARCH_BACKUP_SEARCH: c_uint = 0x0010;
//
// Size of the resume key on FINDFIRST and FINDNEXT calls
//
pub const CIFS_SMB_RESUME_KEY_SIZE: c_int = 4;
    pub /: *mut *mut smb_hdr hdr; / wct = 15,
    pub TotalParameterCount: __le16,
    pub TotalDataCount: __le16,
    pub MaxParameterCount: __le16,
    pub MaxDataCount: __le16,
    pub MaxSetupCount: __u8,
    pub Reserved: __u8,
    pub Flags: __le16,
    pub Timeout: __le32,
    pub Reserved2: __u16,
    pub ParameterCount: __le16,
    pub ParameterOffset: __le16,
    pub DataCount: __le16,
    pub DataOffset: __le16,
    pub /: *mut *mut __u8 SetupCount; / one,
    pub Reserved3: __u8,
    pub /: *mut *mut __le16 SubCommand; / TRANS2_FIND_FIRST,
    pub ByteCount: __le16,
    pub Pad: __u8,
    pub SearchAttributes: __le16,
    pub SearchCount: __le16,
    pub SearchFlags: __le16,
    pub InformationLevel: __le16,
    pub SearchStorageType: __le32,
    pub FileName: [c_char; ],
    pub TRANSACTION2_FFIRST_REQ: } __packed,
    pub /: *mut *mut smb_hdr hdr; / wct = 10,
    pub t2: trans2_resp,
    pub ByteCount: __u16,
    pub TRANSACTION2_FFIRST_RSP: } __packed,
    pub SearchHandle: __u16,
    pub SearchCount: __le16,
    pub EndofSearch: __le16,
    pub EAErrorOffset: __le16,
    pub LastNameOffset: __le16,
    pub T2_FFIRST_RSP_PARMS: } __packed,
    pub /: *mut *mut smb_hdr hdr; / wct = 15,
    pub TotalParameterCount: __le16,
    pub TotalDataCount: __le16,
    pub MaxParameterCount: __le16,
    pub MaxDataCount: __le16,
    pub MaxSetupCount: __u8,
    pub Reserved: __u8,
    pub Flags: __le16,
    pub Timeout: __le32,
    pub Reserved2: __u16,
    pub ParameterCount: __le16,
    pub ParameterOffset: __le16,
    pub DataCount: __le16,
    pub DataOffset: __le16,
    pub /: *mut *mut __u8 SetupCount; / one,
    pub Reserved3: __u8,
    pub /: *mut *mut __le16 SubCommand; / TRANS2_FIND_NEXT,
    pub ByteCount: __le16,
    pub Pad: __u8,
    pub SearchHandle: __u16,
    pub SearchCount: __le16,
    pub InformationLevel: __le16,
    pub ResumeKey: __u32,
    pub SearchFlags: __le16,
    pub ResumeFileName: [c_char; ],
    pub TRANSACTION2_FNEXT_REQ: } __packed,
    pub /: *mut *mut smb_hdr hdr; / wct = 10,
    pub t2: trans2_resp,
    pub ByteCount: __u16,
    pub TRANSACTION2_FNEXT_RSP: } __packed,
    pub SearchCount: __le16,
    pub EndofSearch: __le16,
    pub EAErrorOffset: __le16,
    pub LastNameOffset: __le16,
    pub T2_FNEXT_RSP_PARMS: } __packed,
// QFSInfo Levels
pub const SMB_INFO_ALLOCATION: c_int = 1;
pub const SMB_INFO_VOLUME: c_int = 2;
pub const SMB_QUERY_FS_VOLUME_INFO: c_uint = 0x102;
pub const SMB_QUERY_FS_SIZE_INFO: c_uint = 0x103;
pub const SMB_QUERY_FS_DEVICE_INFO: c_uint = 0x104;
pub const SMB_QUERY_FS_ATTRIBUTE_INFO: c_uint = 0x105;
pub const SMB_QUERY_CIFS_UNIX_INFO: c_uint = 0x200;
pub const SMB_QUERY_POSIX_FS_INFO: c_uint = 0x201;
pub const SMB_QUERY_POSIX_WHO_AM_I: c_uint = 0x202;
pub const SMB_REQUEST_TRANSPORT_ENCRYPTION: c_uint = 0x203;
pub const SMB_QUERY_FS_PROXY: c_uint = 0x204 /* WAFS enabled. Returns structure;
pub const SMB_QUERY_LABEL_INFO: c_uint = 0x3ea;
pub const SMB_QUERY_FS_QUOTA_INFO: c_uint = 0x3ee;
pub const SMB_QUERY_FS_FULL_SIZE_INFO: c_uint = 0x3ef;
pub const SMB_QUERY_OBJECTID_INFO: c_uint = 0x3f0;
    pub /: *mut *mut smb_hdr hdr; / wct = 14+,
    pub TotalParameterCount: __le16,
    pub TotalDataCount: __le16,
    pub MaxParameterCount: __le16,
    pub MaxDataCount: __le16,
    pub MaxSetupCount: __u8,
    pub Reserved: __u8,
    pub Flags: __le16,
    pub Timeout: __le32,
    pub Reserved2: __u16,
    pub ParameterCount: __le16,
    pub ParameterOffset: __le16,
    pub DataCount: __le16,
    pub DataOffset: __le16,
    pub SetupCount: __u8,
    pub Reserved3: __u8,
    pub /: *mut *mut __le16 SubCommand; / one setup word,
    pub ByteCount: __le16,
    pub Pad: __u8,
    pub InformationLevel: __le16,
    pub TRANSACTION2_QFSI_REQ: } __packed,
    pub /: *mut *mut smb_hdr hdr; / wct = 10 + SetupCount,
    pub t2: trans2_resp,
    pub ByteCount: __u16,
    pub /: *mut *mut *mut *mut __u8 Pad; / may be three bytes? // followed by data area,
    pub TRANSACTION2_QFSI_RSP: } __packed,
    pub /: *mut *mut __u32 flags; / 0 = Authenticated user 1 = GUEST,
    pub /: *mut *mut __u32 mask; / which flags bits server understands ie 0x0001,
    pub unix_user_id: __u64,
    pub unix_user_gid: __u64,
    pub /: *mut *mut __u32 number_of_supplementary_gids; / may be zero,
    pub /: *mut *mut __u32 number_of_sids; / may be zero,
    pub /: *mut *mut __u32 length_of_sid_array; / in bytes - may be zero,
    pub /: *mut *mut __u32 pad; / reserved - MBZ,
// __u64 gid_array[0]; */  /* may be empty
// __u8 * psid_list */  /* may be empty
    pub WHOAMI_RSP_DATA: } __packed,
// SETFSInfo Levels
pub const SMB_SET_CIFS_UNIX_INFO: c_uint = 0x200;
// level 0x203 is defined above in list of QFS info levels
// #define SMB_REQUEST_TRANSPORT_ENCRYPTION 0x203
// Level 0x200 request structure follows
    pub /: *mut *mut smb_hdr hdr; / wct = 15,
    pub TotalParameterCount: __le16,
    pub TotalDataCount: __le16,
    pub MaxParameterCount: __le16,
    pub MaxDataCount: __le16,
    pub MaxSetupCount: __u8,
    pub Reserved: __u8,
    pub Flags: __le16,
    pub Timeout: __le32,
    pub Reserved2: __u16,
    pub /: *mut *mut __le16 ParameterCount; / 4,
    pub ParameterOffset: __le16,
    pub /: *mut *mut __le16 DataCount; / 12,
    pub DataOffset: __le16,
    pub /: *mut *mut __u8 SetupCount; / one,
    pub Reserved3: __u8,
    pub /: *mut *mut __le16 SubCommand; / TRANS2_SET_FS_INFORMATION,
    pub ByteCount: __le16,
    pub Pad: __u8,
    pub /: *mut *mut __u16 FileNum; / Parameters start.,
    pub /: *mut *mut __le16 InformationLevel;/ Parameters end.,
    pub /: *mut *mut __le16 ClientUnixMajor; / Data start.,
    pub ClientUnixMinor: __le16,
    pub /: *mut *mut __le64 ClientUnixCap; / Data end,
    pub TRANSACTION2_SETFSI_REQ: } __packed,
// level 0x203 request structure follows
    pub /: *mut *mut smb_hdr hdr; / wct = 15,
    pub TotalParameterCount: __le16,
    pub TotalDataCount: __le16,
    pub MaxParameterCount: __le16,
    pub MaxDataCount: __le16,
    pub MaxSetupCount: __u8,
    pub Reserved: __u8,
    pub Flags: __le16,
    pub Timeout: __le32,
    pub Reserved2: __u16,
    pub /: *mut *mut __le16 ParameterCount; / 4,
    pub ParameterOffset: __le16,
    pub /: *mut *mut __le16 DataCount; / 12,
    pub DataOffset: __le16,
    pub /: *mut *mut __u8 SetupCount; / one,
    pub Reserved3: __u8,
    pub /: *mut *mut __le16 SubCommand; / TRANS2_SET_FS_INFORMATION,
    pub ByteCount: __le16,
    pub Pad: __u8,
    pub /: *mut *mut __u16 Reserved4; / Parameters start.,
    pub /: *mut *mut __le16 InformationLevel;/ Parameters end.,
// NTLMSSP Blob, Data start.
    pub TRANSACTION2_SETFSI_ENC_REQ: } __packed,
// response for setfsinfo levels 0x200 and 0x203
    pub /: *mut *mut smb_hdr hdr; / wct = 10,
    pub t2: trans2_resp,
    pub ByteCount: __u16,
    pub TRANSACTION2_SETFSI_RSP: } __packed,
    pub /: *mut *mut smb_hdr hdr; / wct = 15,
    pub TotalParameterCount: __le16,
    pub TotalDataCount: __le16,
    pub MaxParameterCount: __le16,
    pub MaxDataCount: __le16,
    pub MaxSetupCount: __u8,
    pub Reserved: __u8,
    pub Flags: __le16,
    pub Timeout: __le32,
    pub Reserved2: __u16,
    pub ParameterCount: __le16,
    pub ParameterOffset: __le16,
    pub DataCount: __le16,
    pub DataOffset: __le16,
    pub SetupCount: __u8,
    pub Reserved3: __u8,
    pub /: *mut *mut __le16 SubCommand; / one setup word,
    pub ByteCount: __le16,
    pub length: *mut *mut __u8 Pad[3]; / Win2K has sent 0x0F01 (max response,
    pub MaxReferralLevel: __le16,
    pub RequestFileName: [c_char; ],
    pub TRANSACTION2_GET_DFS_REFER_REQ: } __packed,

// DFS server target type
pub const DFS_TYPE_LINK: c_uint = 0x0000  /* also for sysvol targets */;
pub const DFS_TYPE_ROOT: c_uint = 0x0001;
// Referral Entry Flags
pub const DFS_NAME_LIST_REF: c_uint = 0x0200 /* set for domain or DC referral responses */;
pub const DFS_TARGET_SET_BOUNDARY: c_uint = 0x0400 /* only valid with version 4 dfs req */;
    pub /: *mut *mut __le16 VersionNumber; / must be 3 or 4,
    pub Size: __le16,
    pub /: *mut *mut __le16 ServerType; / 0x0001 = root targets; 0x0000 = link targets,
    pub ReferralEntryFlags: __le16,
    pub TimeToLive: __le32,
    pub DfsPathOffset: __le16,
    pub DfsAlternatePathOffset: __le16,
    pub /: *mut *mut __le16 NetworkAddressOffset; / offset of the link target,
    pub /: *mut *mut __u8 ServiceSiteGuid[16]; / MBZ, ignored,
    pub REFERRAL3: } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct get_dfs_referral_rsp {
    pub PathConsumed: __le16,
    pub NumberOfReferrals: __le16,
    pub DFSFlags: __le32,
    pub /: *mut *mut REFERRAL3 referrals[]; / array of level 3 dfs_referral structures,
// followed by the strings pointed to by the referral structures
    pub __packed: },
    pub /: *mut *mut smb_hdr hdr; / wct = 10,
    pub t2: trans2_resp,
    pub ByteCount: __u16,
    pub Pad: __u8,
    pub dfs_data: get_dfs_referral_rsp,
    pub TRANSACTION2_GET_DFS_REFER_RSP: } __packed,
// DFS Flags
pub const DFSREF_REFERRAL_SERVER: c_uint = 0x00000001 /* all targets are DFS roots */;
pub const DFSREF_STORAGE_SERVER: c_uint = 0x00000002 /* no further ref requests needed */;
pub const DFSREF_TARGET_FAILBACK: c_uint = 0x00000004 /* only for DFS referral version 4 */;
//
// All structs for everything above the SMB PDUs themselves
// (such as the T2 level specific data) go here
//
// Information on a server
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct serverInfo {
    pub name: [c_char; 16],
    pub versionMajor: c_uchar,
    pub versionMinor: c_uchar,
    pub type: c_ulong,
    pub commentOffset: c_uint,
    pub __packed: },
//
// The following structure is the format of the data returned on a NetShareEnum
// with level "90" (x5A)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shareInfo {
    pub shareName: [c_char; 13],
    pub pad: c_char,
    pub type: c_ushort,
    pub commentOffset: c_uint,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aliasInfo {
    pub aliasName: [c_char; 9],
    pub pad: c_char,
    pub commentOffset: c_uint,
    pub type: [c_uchar; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aliasInfo92 {
    pub aliasNameOffset: c_int,
    pub serverNameOffset: c_int,
    pub shareNameOffset: c_int,
    pub __packed: },
    pub fsid: __le32,
    pub SectorsPerAllocationUnit: __le32,
    pub TotalAllocationUnits: __le32,
    pub FreeAllocationUnits: __le32,
    pub BytesPerSector: __le16,
    pub FILE_SYSTEM_ALLOC_INFO: } __packed,
    pub MajorVersionNumber: __le16,
    pub MinorVersionNumber: __le16,
    pub Capability: __le64,
    pub 0x200*/: *mut *mut } __packed FILE_SYSTEM_UNIX_INFO; / Unix extension level,
// Version numbers for CIFS UNIX major and minor.
pub const CIFS_UNIX_MAJOR_VERSION: c_int = 1;
pub const CIFS_UNIX_MINOR_VERSION: c_int = 0;
// Linux/Unix extensions capability flags
pub const CIFS_UNIX_FCNTL_CAP: c_uint = 0x00000001 /* support for fcntl locks */;
pub const CIFS_UNIX_POSIX_ACL_CAP: c_uint = 0x00000002 /* support getfacl/setfacl */;
pub const CIFS_UNIX_XATTR_CAP: c_uint = 0x00000004 /* support new namespace   */;
pub const CIFS_UNIX_EXTATTR_CAP: c_uint = 0x00000008 /* support chattr/chflag   */;
pub const CIFS_UNIX_POSIX_PATHNAMES_CAP: c_uint = 0x00000010 /* Allow POSIX path chars  */;
pub const CIFS_UNIX_POSIX_PATH_OPS_CAP: c_uint = 0x00000020 /* Allow new POSIX path based;
pub const CIFS_UNIX_LARGE_READ_CAP: c_uint = 0x00000040 /* support reads >128K (up to 0xFFFF00 */;
pub const CIFS_UNIX_LARGE_WRITE_CAP: c_uint = 0x00000080;
pub const CIFS_UNIX_TRANSPORT_ENCRYPTION_CAP: c_uint = 0x00000100 /* can do SPNEGO crypt */;
pub const CIFS_UNIX_TRANSPORT_ENCRYPTION_MANDATORY_CAP: c_uint = 0x00000200 /* must do  */;
pub const CIFS_UNIX_PROXY_CAP: c_uint = 0x00000400 /* Proxy cap: 0xACE ioctl and QFS PROXY call */;

// presumably don't need the 0x20 POSIX_PATH_OPS_CAP since we never send
// #define CIFS_UNIX_CAP_MASK              0x000000fb
pub const CIFS_UNIX_CAP_MASK: c_uint = 0x000003db;

pub const CIFS_UNIX_CAP_MASK: c_uint = 0x00000013;

pub const CIFS_POSIX_EXTENSIONS: c_uint = 0x00000010 /* support for new QFSInfo */;
//
// QueryFileInfo/QueryPathinfo (also for SetPath/SetFile) data buffer formats
//
    pub CreationTime: __le64,
    pub LastAccessTime: __le64,
    pub LastWriteTime: __le64,
    pub ChangeTime: __le64,
    pub Attributes: __le32,
    pub Pad1: __u32,
    pub AllocationSize: __le64,
    pub /: *mut *mut __le64 EndOfFile; / size ie offset to first free byte in file,
    pub /: *mut *mut __le32 NumberOfLinks; / hard links,
    pub DeletePending: __u8,
    pub Directory: __u8,
    pub Pad2: __u16,
    pub EASize: __le32,
    pub FileNameLength: __le32,
    pub __pad: c_char,
    pub FileName): DECLARE_FLEX_ARRAY(char,,
}

// defines for enumerating possible values of the Unix type field below
pub const UNIX_FILE: c_int = 0;
pub const UNIX_DIR: c_int = 1;
pub const UNIX_SYMLINK: c_int = 2;
pub const UNIX_CHARDEV: c_int = 3;
pub const UNIX_BLOCKDEV: c_int = 4;
pub const UNIX_FIFO: c_int = 5;
pub const UNIX_SOCKET: c_int = 6;
// The following three structures are needed only for
#[repr(C)]
#[derive(Copy, Clone)]
pub struct file_allocation_info {
    pub /: *mut *mut __le64 AllocationSize; / Note old Samba srvr rounds this up too much,
    pub /: *mut *mut } __packed; / size used on disk, for level 0x103 for set, 0x105 for query,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct file_end_of_file_info {
    pub /: *mut *mut __le64 FileSize; / offset to end of file,
    pub /: *mut *mut } __packed; / size info, level 0x104 for set, 0x106 for query,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct file_alt_name_info {
    pub alt_name): DECLARE_FLEX_ARRAY(__u8,,
    pub /: *mut *mut } __packed; / level 0x0108,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct file_stream_info {
    pub /: *mut *mut __le32 number_of_streams; / BB check sizes and verify location,
// followed by info on streams themselves
    pub size: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct file_compression_info {
    pub compressed_size: __le64,
    pub format: __le16,
    pub unit_shift: __u8,
    pub ch_shift: __u8,
    pub cl_shift: __u8,
    pub pad: [__u8; 3],
    pub /: *mut *mut } __packed; / level 0x10b,
// POSIX ACL set/query path info structures
pub const CIFS_ACL_VERSION: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cifs_posix_ace {
    pub cifs_e_tag: __u8,
    pub cifs_e_perm: __u8,
    pub /: *mut *mut __le64 cifs_uid; / or gid,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cifs_posix_acl {
    pub version: __le16,
    pub /: *mut *mut __le16 access_entry_count; / access ACL - count of entries,
    pub /: *mut *mut __le16 default_entry_count; / default ACL - count of entries,
    pub ace_array: [cifs_posix_ace; ],
// followed by struct cifs_posix_ace default_ace_array[]
    pub /: *mut *mut } __packed; / level 0x204,
// types of access control entries already defined in posix_acl.h
// #define CIFS_POSIX_ACL_USER_OBJ	 0x01
pub const CIFS_POSIX_ACL_USER: c_uint = 0x02;
pub const CIFS_POSIX_ACL_GROUP_OBJ: c_uint = 0x04;
pub const CIFS_POSIX_ACL_GROUP: c_uint = 0x08;
pub const CIFS_POSIX_ACL_MASK: c_uint = 0x10;
pub const CIFS_POSIX_ACL_OTHER: c_uint = 0x20 */;
// types of perms
// #define CIFS_POSIX_ACL_EXECUTE   0x01
pub const CIFS_POSIX_ACL_WRITE: c_uint = 0x02;
pub const CIFS_POSIX_ACL_READ: c_uint = 0x04 */;
// end of POSIX ACL definitions
// POSIX Open Flags
pub const SMB_O_RDONLY: c_uint = 0x1;
pub const SMB_O_WRONLY: c_uint = 0x2;
pub const SMB_O_RDWR: c_uint = 0x4;
pub const SMB_O_CREAT: c_uint = 0x10;
pub const SMB_O_EXCL: c_uint = 0x20;
pub const SMB_O_TRUNC: c_uint = 0x40;
pub const SMB_O_APPEND: c_uint = 0x80;
pub const SMB_O_SYNC: c_uint = 0x100;
pub const SMB_O_DIRECTORY: c_uint = 0x200;
pub const SMB_O_NOFOLLOW: c_uint = 0x400;
pub const SMB_O_DIRECT: c_uint = 0x800;
    pub /: *mut *mut __le32 OpenFlags; / same as NT CreateX,
    pub PosixOpenFlags: __le32,
    pub Permissions: __le64,
    pub /: *mut *mut __le16 Level; / reply level requested (see QPathInfo levels),
    pub /: *mut *mut } __packed OPEN_PSX_REQ; / level 0x209 SetPathInfo data,
    pub OplockFlags: __le16,
    pub Fid: __u16,
    pub CreateAction: __le32,
    pub ReturnedLevel: __le16,
    pub Pad: __le16,
// struct following varies based on requested level
    pub /: *mut *mut } __packed OPEN_PSX_RSP; / level 0x209 SetPathInfo data,
pub const SMB_POSIX_UNLINK_FILE_TARGET: c_int = 0;
pub const SMB_POSIX_UNLINK_DIRECTORY_TARGET: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct unlink_psx_rq {
    pub type: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct file_internal_info {
    pub /: *mut *mut __le64 UniqueId; / inode number,
    pub /: *mut *mut } __packed; / level 0x3ee,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct file_mode_info {
    pub Mode: __le32,
    pub /: *mut *mut } __packed; / level 0x3f8,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct file_attrib_tag {
    pub Attribute: __le32,
    pub ReparseTag: __le32,
    pub /: *mut *mut } __packed; / level 0x40b,
//
// FindFirst/FindNext transact2 data buffer formats
//
    pub NextEntryOffset: __le32,
    pub /: *mut *mut __u32 ResumeKey; / as with FileIndex - no need to convert,
    pub basic: FILE_UNIX_BASIC_INFO,
    pub __pad: c_char,
    pub FileName): DECLARE_FLEX_ARRAY(char,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fea {
    pub EA_flags: c_uchar,
    pub name_len: __u8,
    pub value_len: __le16,
    pub name: [c_char; ],
// optionally followed by value
    pub __packed: },
// flags for _FEA.fEA
pub const FEA_NEEDEA: c_uint = 0x80	/* need EA bit */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fealist {
    pub list_len: __le32,
    pub list: fea,
    pub __packed: },
// used to hold an arbitrary blob of data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct data_blob {
    pub data: *mut __u8,
    pub length: usize,
    pub data_blob): *mut *mut void (free) (struct data_blob,
    pub __packed: },

//
// xsymlink is a symlink format (used by MacOS) that can be used
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xsymlink {
// 1067 bytes
    pub /: *mut *mut *mut *mut char signature[4]; / XSym / / not null terminated,
    pub /: *mut *mut char cr0; / \n,
// ASCII representation of length (4 bytes decimal) terminated by \n not null
    pub length: [c_char; 4],
    pub /: *mut *mut char cr1; / \n,
// md5 of valid subset of path ie path[0] through path[length-1]
    pub md5: [__u8; 32],
    pub /: *mut *mut char cr2; / \n,
// if room left, then end with \n then 0x20s by convention but not required
    pub path: [c_char; 1024],
    pub __packed: },
// BB do we need another field for flags? BB
    pub xattr_name_len: __u32,
    pub xattr_value_len: __u32,
    pub xattr_name: [c_char; ],
// followed by xattr_value[xattr_value_len], no pad
    pub /: *mut *mut } __packed FILE_XATTR_INFO; / extended attribute info level 0x205,
// flags for lsattr and chflags commands removed arein uapi/linux/fs.h
    pub /: *mut *mut __le64 mask; / list of all possible attribute bits,
    pub /: *mut *mut __le64 mode; / list of actual attribute bits on this inode,
    pub /: *mut *mut } __packed FILE_CHATTR_INFO; / ext attributes (chattr, chflags) level 0x206,

