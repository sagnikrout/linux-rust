//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ceph/crypto.h
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
// Ceph fscrypt functionality
//

pub const CEPH_FSCRYPT_BLOCK_SHIFT: c_int = 12;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_fname {
    pub dir: *mut inode,
    pub hashed: *mut *mut char name; // b64 encoded, possibly,
    pub any): *mut *mut unsigned char ctext; // binary crypttext (if,
    pub buffer: u32 name_len; // length of name,
    pub crypttext: u32 ctext_len; // length of,
    pub no_copy: bool,
}

//
// Header for the encrypted file when truncating the size, this
// will be sent to MDS, and the MDS will update the encrypted
// last block and then truncate the size.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_fscrypt_truncate_size_header {
    pub ver: __u8,
    pub compat: __u8,
//
// It will be sizeof(assert_ver + file_offset + block_size)
// if the last block is empty when it's located in a file
// hole. Or the data_len will plus CEPH_FSCRYPT_BLOCK_SIZE.
//
    pub data_len: __le32,
    pub change_attr: __le64,
    pub file_offset: __le64,
    pub block_size: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_fscrypt_auth {
    pub cfa_version: __le32,
    pub cfa_blob_len: __le32,
    pub cfa_blob: [u8; FSCRYPT_SET_CONTEXT_MAX_SIZE],
    pub __packed: },
pub const CEPH_FSCRYPT_AUTH_VERSION: c_int = 1;
    pub le32_to_cpu(fa->cfa_blob_len): u32 ctxsize =,
    pub ctxsize: return offsetof(struct ceph_fscrypt_auth, cfa_blob) +,

//
// We want to encrypt filenames when creating them, but the encrypted
// versions of those names may have illegal characters in them. To mitigate
// that, we base64 encode them, but that gives us a result that can exceed
// NAME_MAX.
//
// Follow a similar scheme to fscrypt itself, and cap the filename to a
// smaller size. If the ciphertext name is longer than the value below, then
// sha256 hash the remaining bytes.
//
// For the fscrypt_nokey_name struct the dirhash[2] member is useless in ceph
// so the corresponding struct will be:
//
// struct fscrypt_ceph_nokey_name {
// u8 bytes[157];
// u8 sha256[SHA256_DIGEST_SIZE];
// }; // 180 bytes => 240 bytes base64-encoded, which is <= NAME_MAX (255)
//
// (240 bytes is the maximum size allowed for snapshot names to take into
// account the format: '_<SNAPSHOT-NAME>_<INODE-NUMBER>'.)
//
// Note that for long names that end up having their tail portion hashed, we
// must also store the full encrypted name (in the dentry's alternate_name
// field).
//

    pub sb): *mut void ceph_fscrypt_set_ops(struct super_block,
    pub fsc): *mut void ceph_fscrypt_free_dummy_policy(struct ceph_fs_client,
    pub as): *mut ceph_acl_sec_ctx,
    pub as): *mut ceph_acl_sec_ctx,
    pub len): *mut *mut *mut int ceph_encode_encrypted_dname(struct inode parent, char buf, int,
    pub 0: return,
    pub fname): return fscrypt_fname_alloc_buffer(NAME_MAX,,
    pub is_nokey): *mut *mut fscrypt_str oname, bool,
    pub dir): *mut int ceph_fscrypt_prepare_readdir(struct inode,
// crypto blocks cannot span more than one page
    pub PAGE_SHIFT): BUILD_BUG_ON(CEPH_FSCRYPT_BLOCK_SHIFT >,
    pub CEPH_FSCRYPT_BLOCK_SHIFT): (off >>,
//
// If we have an encrypted inode then we must adjust the offset and
// range of the on-the-wire read to cover an entire encryption block.
// The copy will be done using the original offset and length, after
// we've decrypted the result.
//
// len = ceph_fscrypt_blocks(*off, *len) * CEPH_FSCRYPT_BLOCK_SIZE;
// off &= CEPH_FSCRYPT_BLOCK_MASK;
    pub lblk_num): unsigned int offs, u64,
    pub lblk_num): unsigned int offs, u64,
    pub len): u64 off, int,
    pub ext_cnt): u32,
    pub len): c_int,
    pub page: return fscrypt_is_bounce_page(page) ? fscrypt_pagecache_page(page) :,

    pub -EOPNOTSUPP: return,
    pub 0: return,
    pub len: return,
    pub 0: return,
    pub fname->name: oname->name =,
    pub fname->name_len: oname->len =,
    pub 0: return,
    pub 0: return,
    pub 0: return,
    pub 0: return,
    pub 0: return,
    pub 0: return,
    pub 0: return,
    pub page: return,

    pub page_offset(ceph_fscrypt_pagecache_page(page)): return,
