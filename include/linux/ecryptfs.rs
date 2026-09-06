//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ecryptfs.h
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
// Version verification for shared data structures w/ userspace
pub const ECRYPTFS_VERSION_MAJOR: c_uint = 0x00;
pub const ECRYPTFS_VERSION_MINOR: c_uint = 0x04;
pub const ECRYPTFS_SUPPORTED_FILE_VERSION: c_uint = 0x03;
// These flags indicate which features are supported by the kernel
// module; userspace tools such as the mount helper read the feature
// bits from a sysfs handle in order to determine how to behave.
pub const ECRYPTFS_VERSIONING_PASSPHRASE: c_uint = 0x00000001;
pub const ECRYPTFS_VERSIONING_PUBKEY: c_uint = 0x00000002;
pub const ECRYPTFS_VERSIONING_PLAINTEXT_PASSTHROUGH: c_uint = 0x00000004;
pub const ECRYPTFS_VERSIONING_POLICY: c_uint = 0x00000008;
pub const ECRYPTFS_VERSIONING_XATTR: c_uint = 0x00000010;
pub const ECRYPTFS_VERSIONING_MULTKEY: c_uint = 0x00000020;
pub const ECRYPTFS_VERSIONING_DEVMISC: c_uint = 0x00000040;
pub const ECRYPTFS_VERSIONING_HMAC: c_uint = 0x00000080;
pub const ECRYPTFS_VERSIONING_FILENAME_ENCRYPTION: c_uint = 0x00000100;
pub const ECRYPTFS_VERSIONING_GCM: c_uint = 0x00000200;
pub const ECRYPTFS_MAX_PASSWORD_LENGTH: c_int = 64;

pub const ECRYPTFS_SALT_SIZE: c_int = 8;

// The original signature size is only for what is stored on disk; all
// in-memory representations are expanded hex, so it better adapted to
// be passed around or referenced on the command line
pub const ECRYPTFS_SIG_SIZE: c_int = 8;

pub const ECRYPTFS_MAX_KEY_BYTES: c_int = 64;
pub const ECRYPTFS_MAX_ENCRYPTED_KEY_BYTES: c_int = 512;
pub const ECRYPTFS_FILE_VERSION: c_uint = 0x03;
pub const ECRYPTFS_MAX_PKI_NAME_BYTES: c_int = 16;
pub const RFC2440_CIPHER_DES3_EDE: c_uint = 0x02;
pub const RFC2440_CIPHER_CAST_5: c_uint = 0x03;
pub const RFC2440_CIPHER_BLOWFISH: c_uint = 0x04;
pub const RFC2440_CIPHER_AES_128: c_uint = 0x07;
pub const RFC2440_CIPHER_AES_192: c_uint = 0x08;
pub const RFC2440_CIPHER_AES_256: c_uint = 0x09;
pub const RFC2440_CIPHER_TWOFISH: c_uint = 0x0a;
pub const RFC2440_CIPHER_CAST_6: c_uint = 0x0b;
pub const RFC2440_CIPHER_RSA: c_uint = 0x01;
//
// For convenience, we may need to pass around the encrypted session
// key between kernel and userspace because the authentication token
// may not be extractable.  For example, the TPM may not release the
// private key, instead requiring the encrypted data and returning the
// decrypted data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ecryptfs_session_key {
pub const ECRYPTFS_USERSPACE_SHOULD_TRY_TO_DECRYPT: c_uint = 0x00000001;
pub const ECRYPTFS_USERSPACE_SHOULD_TRY_TO_ENCRYPT: c_uint = 0x00000002;
pub const ECRYPTFS_CONTAINS_DECRYPTED_KEY: c_uint = 0x00000004;
pub const ECRYPTFS_CONTAINS_ENCRYPTED_KEY: c_uint = 0x00000008;
    pub flags: u32,
    pub encrypted_key_size: u32,
    pub decrypted_key_size: u32,
    pub encrypted_key: [u8; ECRYPTFS_MAX_ENCRYPTED_KEY_BYTES],
    pub decrypted_key: [u8; ECRYPTFS_MAX_KEY_BYTES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ecryptfs_password {
    pub password_bytes: u32,
    pub hash_algo: i32,
    pub hash_iterations: u32,
    pub session_key_encryption_key_bytes: u32,
pub const ECRYPTFS_PERSISTENT_PASSWORD: c_uint = 0x01;
pub const ECRYPTFS_SESSION_KEY_ENCRYPTION_KEY_SET: c_uint = 0x02;
    pub flags: u32,
// Iterated-hash concatenation of salt and passphrase
    pub session_key_encryption_key: [u8; ECRYPTFS_MAX_KEY_BYTES],
    pub 1]: u8 signature[ECRYPTFS_PASSWORD_SIG_SIZE +,
// Always in expanded hex
    pub salt: [u8; ECRYPTFS_SALT_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ecryptfs_token_types {

    struct ecryptfs_private_key {
    u32 key_size;
    u32 data_len;
    u8 signature[ECRYPTFS_PASSWORD_SIG_SIZE + 1];
    char pki_type[ECRYPTFS_MAX_PKI_NAME_BYTES + 1];
    u8 data[];
}

// May be a password or a private key
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ecryptfs_auth_tok {
    pub /: *mut *mut u16 version; / 8-bit major and 8-bit minor,
    pub token_type: u16,
pub const ECRYPTFS_ENCRYPT_ONLY: c_uint = 0x00000001;
    pub flags: u32,
    pub session_key: ecryptfs_session_key,
    pub reserved: [u8; 32],
    pub password: ecryptfs_password,
    pub private_key: ecryptfs_private_key,
    pub token: },
// C attribute field omitted
