//! Automatically rewritten from C Header to Rust Module
//! Source: fs/smb/server/ntlmssp.h
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


// SPDX-License-Identifier: LGPL-2.1+
//
// Copyright (c) International Business Machines  Corp., 2002,2007
// Author(s): Steve French (sfrench@us.ibm.com)
//

// Security blob target info data

//
// Size of the crypto key returned on the negotiate SMB in bytes
//

//
// Size of encrypted user password in bytes
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
pub const NTLMSSP_NEGOTIATE_VERSION: c_uint = 0x2000000 /* we do not set */;
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct security_buffer {
    pub Length: __le16,
    pub MaximumLength: __le16,
    pub /: *mut *mut __le32 BufferOffset; / offset to buffer,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct target_info {
    pub Type: __le16,
    pub Length: __le16,
    pub Content: [__u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct negotiate_message {
    pub Signature: [__u8; sizeof(NTLMSSP_SIGNATURE)],
    pub /: *mut *mut __le32 MessageType; / NtLmNegotiate = 1,
    pub NegotiateFlags: __le32,
    pub /: *mut *mut security_buffer DomainName; / RFC 1001 style and ASCII,
    pub /: *mut *mut security_buffer WorkstationName; / RFC 1001 and ASCII,
//
// struct security_buffer for version info not present since we
// do not set the version is present flag
//
    pub DomainString: [c_char; ],
// followed by WorkstationString
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct challenge_message {
    pub Signature: [__u8; sizeof(NTLMSSP_SIGNATURE)],
    pub /: *mut *mut __le32 MessageType; / NtLmChallenge = 2,
    pub TargetName: security_buffer,
    pub NegotiateFlags: __le32,
    pub Challenge: [__u8; CIFS_CRYPTO_KEY_SIZE],
    pub Reserved: [__u8; 8],
    pub TargetInfoArray: security_buffer,
//
// struct security_buffer for version info not present since we
// do not set the version is present flag
//
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct authenticate_message {
    pub Signature: [__u8; sizeof(NTLMSSP_SIGNATURE)],
    pub /: *mut *mut __le32 MessageType; / NtLmsAuthenticate = 3,
    pub LmChallengeResponse: security_buffer,
    pub NtChallengeResponse: security_buffer,
    pub DomainName: security_buffer,
    pub UserName: security_buffer,
    pub WorkstationName: security_buffer,
    pub SessionKey: security_buffer,
    pub NegotiateFlags: __le32,
//
// struct security_buffer for version info not present since we
// do not set the version is present flag
//
    pub UserString: [c_char; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ntlmv2_resp {
    pub ntlmv2_hash: [c_char; CIFS_ENCPWD_SIZE],
    pub blob_signature: __le32,
    pub reserved: __u32,
    pub time: __le64,
    pub /: *mut *mut __u64 client_chal; / random,
    pub reserved2: __u32,
// array of name entries could follow ending in minimum 4 byte struct
    pub __packed: },
// per smb session structure/fields
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ntlmssp_auth {
// whether session key is per smb session
    pub sesskey_per_smbsess: bool,
// sent by client in type 1 ntlmsssp exchange
    pub client_flags: __u32,
// sent by server in type 2 ntlmssp exchange
    pub conn_flags: __u32,
// sent to server
    pub ciphertext: [c_uchar; CIFS_CPHTXT_SIZE],
// used by ntlmssp
    pub cryptkey: [c_char; CIFS_CRYPTO_KEY_SIZE],
}
