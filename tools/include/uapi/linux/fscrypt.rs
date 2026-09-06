//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/uapi/linux/fscrypt.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// fscrypt user API
//
// These ioctls can be used on filesystems that support fscrypt.  See the
// "User API" section of Documentation/filesystems/fscrypt.rst.
//

// Encryption policy flags
pub const FSCRYPT_POLICY_FLAGS_PAD_4: c_uint = 0x00;
pub const FSCRYPT_POLICY_FLAGS_PAD_8: c_uint = 0x01;
pub const FSCRYPT_POLICY_FLAGS_PAD_16: c_uint = 0x02;
pub const FSCRYPT_POLICY_FLAGS_PAD_32: c_uint = 0x03;
pub const FSCRYPT_POLICY_FLAGS_PAD_MASK: c_uint = 0x03;
pub const FSCRYPT_POLICY_FLAG_DIRECT_KEY: c_uint = 0x04;
pub const FSCRYPT_POLICY_FLAG_IV_INO_LBLK_64: c_uint = 0x08;
pub const FSCRYPT_POLICY_FLAG_IV_INO_LBLK_32: c_uint = 0x10;
// Encryption algorithms
pub const FSCRYPT_MODE_AES_256_XTS: c_int = 1;
pub const FSCRYPT_MODE_AES_256_CTS: c_int = 4;
pub const FSCRYPT_MODE_AES_128_CBC: c_int = 5;
pub const FSCRYPT_MODE_AES_128_CTS: c_int = 6;
pub const FSCRYPT_MODE_SM4_XTS: c_int = 7;
pub const FSCRYPT_MODE_SM4_CTS: c_int = 8;
pub const FSCRYPT_MODE_ADIANTUM: c_int = 9;
pub const FSCRYPT_MODE_AES_256_HCTR2: c_int = 10;
//
// Legacy policy version; ad-hoc KDF and no key verification.
// For new encrypted directories, use fscrypt_policy_v2 instead.
//
// Careful: the .version field for this is actually 0, not 1.
//
pub const FSCRYPT_POLICY_V1: c_int = 0;
pub const FSCRYPT_KEY_DESCRIPTOR_SIZE: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fscrypt_policy_v1 {
    pub version: __u8,
    pub contents_encryption_mode: __u8,
    pub filenames_encryption_mode: __u8,
    pub flags: __u8,
    pub master_key_descriptor: [__u8; FSCRYPT_KEY_DESCRIPTOR_SIZE],
}

//
// Process-subscribed "logon" key description prefix and payload format.
// Deprecated; prefer FS_IOC_ADD_ENCRYPTION_KEY instead.
//

pub const FSCRYPT_KEY_DESC_PREFIX_SIZE: c_int = 8;
pub const FSCRYPT_MAX_KEY_SIZE: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fscrypt_key {
    pub mode: __u32,
    pub raw: [__u8; FSCRYPT_MAX_KEY_SIZE],
    pub size: __u32,
}

//
// New policy version with HKDF and key verification (recommended).
//
pub const FSCRYPT_POLICY_V2: c_int = 2;
pub const FSCRYPT_KEY_IDENTIFIER_SIZE: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fscrypt_policy_v2 {
    pub version: __u8,
    pub contents_encryption_mode: __u8,
    pub filenames_encryption_mode: __u8,
    pub flags: __u8,
    pub log2_data_unit_size: __u8,
    pub __reserved: [__u8; 3],
    pub master_key_identifier: [__u8; FSCRYPT_KEY_IDENTIFIER_SIZE],
}

// Struct passed to FS_IOC_GET_ENCRYPTION_POLICY_EX
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fscrypt_get_policy_ex_arg {
    pub /: *mut *mut __u64 policy_size; / input/output,
    pub version: __u8,
    pub v1: fscrypt_policy_v1,
    pub v2: fscrypt_policy_v2,
    pub /: *mut *mut } policy; / output,
}

//
// v1 policy keys are specified by an arbitrary 8-byte key "descriptor",
// matching fscrypt_policy_v1::master_key_descriptor.
//
pub const FSCRYPT_KEY_SPEC_TYPE_DESCRIPTOR: c_int = 1;
//
// v2 policy keys are specified by a 16-byte key "identifier" which the kernel
// calculates as a cryptographic hash of the key itself,
// matching fscrypt_policy_v2::master_key_identifier.
//
pub const FSCRYPT_KEY_SPEC_TYPE_IDENTIFIER: c_int = 2;
//
// Specifies a key, either for v1 or v2 policies.  This doesn't contain the
// actual key itself; this is just the "name" of the key.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fscrypt_key_specifier {
    pub /: *mut *mut *mut __u32 type; / one of FSCRYPT_KEY_SPEC_TYPE_,
    pub __reserved: __u32,
    pub /: *mut *mut __u8 __reserved[32]; / reserve some extra space,
    pub descriptor: [__u8; FSCRYPT_KEY_DESCRIPTOR_SIZE],
    pub identifier: [__u8; FSCRYPT_KEY_IDENTIFIER_SIZE],
    pub u: },
}

//
// Payload of Linux keyring key of type "fscrypt-provisioning", referenced by
// fscrypt_add_key_arg::key_id as an alternative to fscrypt_add_key_arg::raw.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fscrypt_provisioning_key_payload {
    pub type: __u32,
    pub flags: __u32,
    pub raw: [__u8; ],
}

// Struct passed to FS_IOC_ADD_ENCRYPTION_KEY
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fscrypt_add_key_arg {
    pub key_spec: fscrypt_key_specifier,
    pub raw_size: __u32,
    pub key_id: __u32,
pub const FSCRYPT_ADD_KEY_FLAG_HW_WRAPPED: c_uint = 0x00000001;
    pub flags: __u32,
    pub __reserved: [__u32; 7],
    pub raw: [__u8; ],
}

// Struct passed to FS_IOC_REMOVE_ENCRYPTION_KEY
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fscrypt_remove_key_arg {
    pub key_spec: fscrypt_key_specifier,
pub const FSCRYPT_KEY_REMOVAL_STATUS_FLAG_FILES_BUSY: c_uint = 0x00000001;
pub const FSCRYPT_KEY_REMOVAL_STATUS_FLAG_OTHER_USERS: c_uint = 0x00000002;
    pub /: *mut *mut __u32 removal_status_flags; / output,
    pub __reserved: [__u32; 5],
}

// Struct passed to FS_IOC_GET_ENCRYPTION_KEY_STATUS
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fscrypt_get_key_status_arg {
// input
    pub key_spec: fscrypt_key_specifier,
    pub __reserved: [__u32; 6],
// output
pub const FSCRYPT_KEY_STATUS_ABSENT: c_int = 1;
pub const FSCRYPT_KEY_STATUS_PRESENT: c_int = 2;
pub const FSCRYPT_KEY_STATUS_INCOMPLETELY_REMOVED: c_int = 3;
    pub status: __u32,
pub const FSCRYPT_KEY_STATUS_FLAG_ADDED_BY_SELF: c_uint = 0x00000001;
    pub status_flags: __u32,
    pub user_count: __u32,
    pub __out_reserved: [__u32; 13],
}

//
// old names; don't add anything new here!

pub const FS_POLICY_FLAGS_VALID: c_uint = 0x07	/* contains old flags only */;

