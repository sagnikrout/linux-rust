//! Automatically rewritten from C Header to Rust Module
//! Source: fs/smb/client/ntlmssp.h
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
// Copyright (c) International Business Machines  Corp., 2002,2007
// Author(s): Steve French (sfrench@us.ibm.com)
//

// Message Types

// Negotiate Flags
pub const NTLMSSP_NEGOTIATE_UNICODE: c_uint = 0x01 /* Text strings are unicode */;
pub const NTLMSSP_NEGOTIATE_OEM: c_uint = 0x02 /* Text strings are in OEM */;
pub const NTLMSSP_REQUEST_TARGET: c_uint = 0x04 /* Srv returns its auth realm */;
// define reserved9                       0x08
pub const NTLMSSP_NEGOTIATE_SIGN: c_uint = 0x0010 /* Request signing capability */;
pub const NTLMSSP_NEGOTIATE_SEAL: c_uint = 0x0020 /* Request confidentiality */;
pub const NTLMSSP_NEGOTIATE_DGRAM: c_uint = 0x0040;
pub const NTLMSSP_NEGOTIATE_LM_KEY: c_uint = 0x0080 /* Use LM session key */;
// defined reserved 8                   0x0100
pub const NTLMSSP_NEGOTIATE_NTLM: c_uint = 0x0200 /* NTLM authentication */;
pub const NTLMSSP_NEGOTIATE_NT_ONLY: c_uint = 0x0400 /* Lanman not allowed */;
pub const NTLMSSP_ANONYMOUS: c_uint = 0x0800;
pub const NTLMSSP_NEGOTIATE_DOMAIN_SUPPLIED: c_uint = 0x1000 /* reserved6 */;
pub const NTLMSSP_NEGOTIATE_WORKSTATION_SUPPLIED: c_uint = 0x2000;
pub const NTLMSSP_NEGOTIATE_LOCAL_CALL: c_uint = 0x4000 /* client/server same machine */;
pub const NTLMSSP_NEGOTIATE_ALWAYS_SIGN: c_uint = 0x8000 /* Sign. All security levels  */;
pub const NTLMSSP_TARGET_TYPE_DOMAIN: c_uint = 0x10000;
pub const NTLMSSP_TARGET_TYPE_SERVER: c_uint = 0x20000;
pub const NTLMSSP_TARGET_TYPE_SHARE: c_uint = 0x40000;
pub const NTLMSSP_NEGOTIATE_EXTENDED_SEC: c_uint = 0x80000 /* NB:not related to NTLMv2 pwd*/;
// #define NTLMSSP_REQUEST_INIT_RESP     0x100000
pub const NTLMSSP_NEGOTIATE_IDENTIFY: c_uint = 0x100000;
pub const NTLMSSP_REQUEST_ACCEPT_RESP: c_uint = 0x200000 /* reserved5 */;
pub const NTLMSSP_REQUEST_NON_NT_KEY: c_uint = 0x400000;
pub const NTLMSSP_NEGOTIATE_TARGET_INFO: c_uint = 0x800000;
// #define reserved4                 0x1000000
pub const NTLMSSP_NEGOTIATE_VERSION: c_uint = 0x2000000 /* we only set for SMB2+ */;
// #define reserved3                 0x4000000
// #define reserved2                 0x8000000
// #define reserved1                0x10000000
pub const NTLMSSP_NEGOTIATE_128: c_uint = 0x20000000;
pub const NTLMSSP_NEGOTIATE_KEY_XCH: c_uint = 0x40000000;
pub const NTLMSSP_NEGOTIATE_56: c_uint = 0x80000000;
// Define AV Pair Field IDs
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum av_field_type {
    NTLMSSP_AV_EOL = 0,
    NTLMSSP_AV_NB_COMPUTER_NAME,
    NTLMSSP_AV_NB_DOMAIN_NAME,
    NTLMSSP_AV_DNS_COMPUTER_NAME,
    NTLMSSP_AV_DNS_DOMAIN_NAME,
    NTLMSSP_AV_DNS_TREE_NAME,
    NTLMSSP_AV_FLAGS,
    NTLMSSP_AV_TIMESTAMP,
    NTLMSSP_AV_RESTRICTION,
    NTLMSSP_AV_TARGET_NAME,
    NTLMSSP_AV_CHANNEL_BINDINGS
}

// Although typedefs are not commonly used for structure definitions
// in the Linux kernel, in this particular case they are useful
// to more closely match the standards document for NTLMSSP from
// OpenGroup and to make the code more closely match the standard in
// appearance
// SECURITY_BUFFER for version info not present since we
// followed by WorkstationString
pub const NTLMSSP_REVISION_W2K3: c_uint = 0x0F;
// See MS-NLMP section 2.2.2.10
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ntlmssp_version {
    pub ProductMajorVersion: __u8,
    pub ProductMinorVersion: __u8,
    pub /: *mut *mut __le16 ProductBuild; / we send the cifs.ko module version here,
    pub Reserved: [__u8; 3],
    pub /: *mut *mut __u8 NTLMRevisionCurrent; / currently 0x0F,
    pub __packed: },
// see MS-NLMP section 2.2.1.1
#[repr(C)]
#[derive(Copy, Clone)]
pub struct negotiate_message {
    pub Signature: [__u8; sizeof(NTLMSSP_SIGNATURE)],
    pub /: *mut *mut __le32 MessageType; / NtLmNegotiate = 1,
    pub NegotiateFlags: __le32,
    pub /: *mut *mut SECURITY_BUFFER DomainName; / RFC 1001 style and ASCII,
    pub /: *mut *mut SECURITY_BUFFER WorkstationName; / RFC 1001 and ASCII,
    pub Version: ntlmssp_version,
// SECURITY_BUFFER
    pub DomainString: [c_char; ],
// followed by WorkstationString
    pub __packed: },
    pub Signature: [__u8; sizeof(NTLMSSP_SIGNATURE)],
    pub /: *mut *mut __le32 MessageType; / NtLmChallenge = 2,
    pub TargetName: SECURITY_BUFFER,
    pub NegotiateFlags: __le32,
    pub Challenge: [__u8; CIFS_CRYPTO_KEY_SIZE],
    pub Reserved: [__u8; 8],
    pub TargetInfoArray: SECURITY_BUFFER,
// SECURITY_BUFFER for version info not present since we
    pub PCHALLENGE_MESSAGE: *mut } __packed CHALLENGE_MESSAGE,,
    pub Signature: [__u8; sizeof(NTLMSSP_SIGNATURE)],
    pub /: *mut *mut __le32 MessageType; / NtLmsAuthenticate = 3,
    pub LmChallengeResponse: SECURITY_BUFFER,
    pub NtChallengeResponse: SECURITY_BUFFER,
    pub DomainName: SECURITY_BUFFER,
    pub UserName: SECURITY_BUFFER,
    pub WorkstationName: SECURITY_BUFFER,
    pub SessionKey: SECURITY_BUFFER,
    pub NegotiateFlags: __le32,
    pub Version: ntlmssp_version,
// SECURITY_BUFFER
    pub UserString: [c_char; ],
    pub PAUTHENTICATE_MESSAGE: *mut } __packed AUTHENTICATE_MESSAGE,,
//
// Size of the session key (crypto key encrypted with the password
//
    pub ses): *mut cifs_ses,
    pub nls_cp): *const nls_table,
    pub nls_cp): *const nls_table,
    pub nls_cp): *const nls_table,
