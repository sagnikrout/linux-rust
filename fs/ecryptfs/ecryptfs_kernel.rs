//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ecryptfs/ecryptfs_kernel.h
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
//
// eCryptfs: Linux filesystem encryption layer
// Kernel declarations.
//
// Copyright (C) 1997-2003 Erez Zadok
// Copyright (C) 2001-2003 Stony Brook University
// Copyright (C) 2004-2008 International Business Machines Corp.
// Author(s): Michael A. Halcrow <mahalcro@us.ibm.com>
// Trevor S. Highland <trevor.highland@gmail.com>
// Tyler Hicks <code@tyhicks.com>
//

pub const ECRYPTFS_DEFAULT_IV_BYTES: c_int = 16;
pub const ECRYPTFS_DEFAULT_EXTENT_SIZE: c_int = 4096;
pub const ECRYPTFS_MINIMUM_HEADER_EXTENT_SIZE: c_int = 8192;
pub const ECRYPTFS_DEFAULT_MSG_CTX_ELEMS: c_int = 32;

pub const ECRYPTFS_DEFAULT_NUM_USERS: c_int = 4;
pub const ECRYPTFS_MAX_NUM_USERS: c_int = 32768;

extern "C" {
    pub fn ecryptfs_dump_auth_tok(auth_tok: *mut ecryptfs_auth_tok);
}
// end = '\0';
extern "C" {
    pub fn ecryptfs_from_hex(dst: *mut c_char, src: *mut c_char, dst_size: c_int);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ecryptfs_key_record {
    pub type: c_uchar,
    pub enc_key_size: usize,
    pub sig: [c_uchar; ECRYPTFS_SIG_SIZE],
    pub enc_key: [c_uchar; ECRYPTFS_MAX_ENCRYPTED_KEY_BYTES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ecryptfs_auth_tok_list {
    pub auth_tok: *mut ecryptfs_auth_tok,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ecryptfs_page_crypt_context {
    pub page: *mut page,
pub const ECRYPTFS_PREPARE_COMMIT_MODE: c_int = 0;
pub const ECRYPTFS_WRITEPAGE_MODE: c_int = 1;
    pub mode: c_uint,
    pub lower_file: *mut file,
    pub wbc: *mut writeback_control,
    pub param: },
}

extern "C" {
    pub fn ERR_PTR(_arg: -EKEYREVOKED) -> return;
}
extern "C" {
    pub fn request_key(_arg: &key_type_encrypted, _arg: sig, _arg: NULL) -> return;
}

extern "C" {
    pub fn ERR_PTR(_arg: -ENOKEY) -> return;
}

extern "C" {
    pub fn ERR_PTR(_arg: -EKEYREVOKED) -> return;
}
pub const ECRYPTFS_MAX_KEYSET_SIZE: c_int = 1024;
pub const ECRYPTFS_MAX_CIPHER_NAME_SIZE: c_int = 31;
pub const ECRYPTFS_MAX_NUM_ENC_KEYS: c_int = 64;

pub const ECRYPTFS_SALT_BYTES: c_int = 2;
pub const MAGIC_ECRYPTFS_MARKER: c_uint = 0x3c81b7f5;

pub const ECRYPTFS_DEFAULT_KEY_BYTES: c_int = 16;
pub const ECRYPTFS_TAG_1_PACKET_TYPE: c_uint = 0x01;
pub const ECRYPTFS_TAG_3_PACKET_TYPE: c_uint = 0x8C;
pub const ECRYPTFS_TAG_11_PACKET_TYPE: c_uint = 0xED;
pub const ECRYPTFS_TAG_64_PACKET_TYPE: c_uint = 0x40;
pub const ECRYPTFS_TAG_65_PACKET_TYPE: c_uint = 0x41;
pub const ECRYPTFS_TAG_66_PACKET_TYPE: c_uint = 0x42;
pub const ECRYPTFS_TAG_67_PACKET_TYPE: c_uint = 0x43;
pub const ECRYPTFS_TAG_70_PACKET_TYPE: c_uint = 0x46 /* FNEK-encrypted filename;
// as dentry name
pub const ECRYPTFS_TAG_71_PACKET_TYPE: c_uint = 0x47 /* FNEK-encrypted filename in;
// metadata
pub const ECRYPTFS_TAG_72_PACKET_TYPE: c_uint = 0x48 /* FEK-encrypted filename as;
// dentry name
pub const ECRYPTFS_TAG_73_PACKET_TYPE: c_uint = 0x49 /* FEK-encrypted filename as;
// metadata

// ecryptfs_parse_packet_length() and
// ecryptfs_write_packet_length()
//
// Constraint: ECRYPTFS_FILENAME_MIN_RANDOM_PREPEND_BYTES >=
// ECRYPTFS_MAX_IV_BYTES
pub const ECRYPTFS_FILENAME_MIN_RANDOM_PREPEND_BYTES: c_int = 16;
pub const ECRYPTFS_NON_NULL: c_uint = 0x42 /* A reasonable substitute for NULL */;

pub const ECRYPTFS_FEK_ENCRYPTED_FILENAME_PREFIX_SIZE: c_int = 23;

pub const ECRYPTFS_FNEK_ENCRYPTED_FILENAME_PREFIX_SIZE: c_int = 24;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ecryptfs_key_sig {
    pub crypt_stat_list: list_head,
    pub 1]: char keysig[ECRYPTFS_SIG_SIZE_HEX +,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ecryptfs_filename {
    pub crypt_stat_list: list_head,
pub const ECRYPTFS_FILENAME_CONTAINS_DECRYPTED: c_uint = 0x00000001;
    pub flags: u32,
    pub seq_no: u32,
    pub filename: *mut c_char,
    pub encrypted_filename: *mut c_char,
    pub filename_size: usize,
    pub encrypted_filename_size: usize,
    pub fnek_sig: [c_char; ECRYPTFS_SIG_SIZE_HEX],
    pub 1]: char dentry_name[ECRYPTFS_ENCRYPTED_DENTRY_NAME_LEN +,
}

//
// This is the primary struct associated with each encrypted file.
//
// TODO: cache align/pack?
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ecryptfs_crypt_stat {
pub const ECRYPTFS_STRUCT_INITIALIZED: c_uint = 0x00000001;
pub const ECRYPTFS_POLICY_APPLIED: c_uint = 0x00000002;
pub const ECRYPTFS_ENCRYPTED: c_uint = 0x00000004;
pub const ECRYPTFS_SECURITY_WARNING: c_uint = 0x00000008;
pub const ECRYPTFS_ENABLE_HMAC: c_uint = 0x00000010;
pub const ECRYPTFS_ENCRYPT_IV_PAGES: c_uint = 0x00000020;
pub const ECRYPTFS_KEY_VALID: c_uint = 0x00000040;
pub const ECRYPTFS_METADATA_IN_XATTR: c_uint = 0x00000080;
pub const ECRYPTFS_VIEW_AS_ENCRYPTED: c_uint = 0x00000100;
pub const ECRYPTFS_KEY_SET: c_uint = 0x00000200;
pub const ECRYPTFS_ENCRYPT_FILENAMES: c_uint = 0x00000400;
pub const ECRYPTFS_ENCFN_USE_MOUNT_FNEK: c_uint = 0x00000800;
pub const ECRYPTFS_ENCFN_USE_FEK: c_uint = 0x00001000;
pub const ECRYPTFS_UNLINK_SIGS: c_uint = 0x00002000;
pub const ECRYPTFS_I_SIZE_INITIALIZED: c_uint = 0x00004000;
    pub flags: u32,
    pub file_version: c_uint,
    pub iv_bytes: usize,
    pub metadata_size: usize,
    pub /: *mut *mut size_t extent_size; / Data extent size; default is 4096,
    pub key_size: usize,
    pub extent_shift: usize,
    pub extent_mask: c_uint,
    pub mount_crypt_stat: *mut ecryptfs_mount_crypt_stat,
    pub tfm: *mut crypto_skcipher,
    pub 1]: unsigned char cipher[ECRYPTFS_MAX_CIPHER_NAME_SIZE +,
    pub key: [c_uchar; ECRYPTFS_MAX_KEY_BYTES],
    pub root_iv: [c_uchar; ECRYPTFS_MAX_IV_BYTES],
    pub keysig_list: list_head,
    pub keysig_list_mutex: mutex,
    pub cs_tfm_mutex: mutex,
    pub cs_mutex: mutex,
}

// inode private data.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ecryptfs_inode_info {
    pub vfs_inode: inode,
    pub wii_inode: *mut inode,
    pub lower_file_mutex: mutex,
    pub lower_file_count: core::sync::atomic::AtomicI32,
    pub lower_file: *mut file,
    pub crypt_stat: ecryptfs_crypt_stat,
}

//
// struct ecryptfs_global_auth_tok - A key used to encrypt all new files
// under the mountpoint
// @flags: Status flags
// @mount_crypt_stat_list: These auth_toks hang off the mount-wide
// cryptographic context. Every time a new
// inode comes into existence, eCryptfs copies
// the auth_toks on that list to the set of
// auth_toks on the inode's crypt_stat
// @global_auth_tok_key: The key from the user's keyring for the sig
// @sig: The key identifier
//
// ecryptfs_global_auth_tok structs refer to authentication token keys
// in the user keyring that apply to newly created files. A list of
// these objects hangs off of the mount_crypt_stat struct for any
// given eCryptfs mount. This struct maintains a reference to both the
// key contents and the key itself so that the key can be put on
// unmount.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ecryptfs_global_auth_tok {
pub const ECRYPTFS_AUTH_TOK_INVALID: c_uint = 0x00000001;
pub const ECRYPTFS_AUTH_TOK_FNEK: c_uint = 0x00000002;
    pub flags: u32,
    pub mount_crypt_stat_list: list_head,
    pub global_auth_tok_key: *mut key,
    pub 1]: unsigned char sig[ECRYPTFS_SIG_SIZE_HEX +,
}

//
// struct ecryptfs_key_tfm - Persistent key tfm
// @key_tfm: crypto API handle to the key
// @key_size: Key size in bytes
// @key_tfm_mutex: Mutex to ensure only one operation in eCryptfs is
// using the persistent TFM at any point in time
// @key_tfm_list: Handle to hang this off the module-wide TFM list
// @cipher_name: String name for the cipher for this TFM
//
// Typically, eCryptfs will use the same ciphers repeatedly throughout
// the course of its operations. In order to avoid unnecessarily
// destroying and initializing the same cipher repeatedly, eCryptfs
// keeps a list of crypto API contexts around to use when needed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ecryptfs_key_tfm {
    pub key_tfm: *mut crypto_skcipher,
    pub key_size: usize,
    pub key_tfm_mutex: mutex,
    pub key_tfm_list: list_head,
    pub 1]: unsigned char cipher_name[ECRYPTFS_MAX_CIPHER_NAME_SIZE +,
}

//
// This struct is to enable a mount-wide passphrase/salt combo. This
// is more or less a stopgap to provide similar functionality to other
// crypto filesystems like EncFS or CFS until full policy support is
// implemented in eCryptfs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ecryptfs_mount_crypt_stat {
// Pointers to memory we do not own, do not free these
pub const ECRYPTFS_PLAINTEXT_PASSTHROUGH_ENABLED: c_uint = 0x00000001;
pub const ECRYPTFS_XATTR_METADATA_ENABLED: c_uint = 0x00000002;
pub const ECRYPTFS_ENCRYPTED_VIEW_ENABLED: c_uint = 0x00000004;
pub const ECRYPTFS_MOUNT_CRYPT_STAT_INITIALIZED: c_uint = 0x00000008;
pub const ECRYPTFS_GLOBAL_ENCRYPT_FILENAMES: c_uint = 0x00000010;
pub const ECRYPTFS_GLOBAL_ENCFN_USE_MOUNT_FNEK: c_uint = 0x00000020;
pub const ECRYPTFS_GLOBAL_ENCFN_USE_FEK: c_uint = 0x00000040;
pub const ECRYPTFS_GLOBAL_MOUNT_AUTH_TOK_ONLY: c_uint = 0x00000080;
    pub flags: u32,
    pub global_auth_tok_list: list_head,
    pub global_auth_tok_list_mutex: mutex,
    pub global_default_cipher_key_size: usize,
    pub global_default_fn_cipher_key_bytes: usize,
    pub 1]: +,
    pub 1]: ECRYPTFS_MAX_CIPHER_NAME_SIZE +,
    pub 1]: char global_default_fnek_sig[ECRYPTFS_SIG_SIZE_HEX +,
}

// superblock private data.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ecryptfs_sb_info {
    pub wsi_sb: *mut super_block,
    pub lower_mnt: *mut vfsmount,
    pub mount_crypt_stat: ecryptfs_mount_crypt_stat,
}

// file private data.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ecryptfs_file_info {
    pub wfi_file: *mut file,
    pub crypt_stat: *mut ecryptfs_crypt_stat,
}

// auth_tok <=> encrypted_session_key mappings
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ecryptfs_auth_tok_list_item {
    pub encrypted_session_key: [c_uchar; ECRYPTFS_MAX_KEY_BYTES],
    pub list: list_head,
    pub auth_tok: ecryptfs_auth_tok,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ecryptfs_message {
// Can never be greater than ecryptfs_message_buf_len
// Used to find the parent msg_ctx
// Inherits from msg_ctx->index
    pub index: u32,
    pub data_len: u32,
    pub __counted_by(data_len): u8 data[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ecryptfs_msg_ctx {
pub const ECRYPTFS_MSG_CTX_STATE_FREE: c_uint = 0x01;
pub const ECRYPTFS_MSG_CTX_STATE_PENDING: c_uint = 0x02;
pub const ECRYPTFS_MSG_CTX_STATE_DONE: c_uint = 0x03;
pub const ECRYPTFS_MSG_CTX_STATE_NO_REPLY: c_uint = 0x04;
    pub state: u8,
pub const ECRYPTFS_MSG_HELO: c_int = 100;
pub const ECRYPTFS_MSG_QUIT: c_int = 101;
pub const ECRYPTFS_MSG_REQUEST: c_int = 102;
pub const ECRYPTFS_MSG_RESPONSE: c_int = 103;
    pub type: u8,
    pub index: u32,
// Counter converts to a sequence number. Each message sent
// out for which we expect a response has an associated
// sequence number. The response must have the same sequence
// number as the counter for the msg_stc for the message to be
// valid.
    pub counter: u32,
    pub msg_size: usize,
    pub msg: *mut ecryptfs_message,
    pub task: *mut task_struct,
    pub node: list_head,
    pub daemon_out_list: list_head,
    pub mux: mutex,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ecryptfs_daemon {
pub const ECRYPTFS_DAEMON_IN_READ: c_uint = 0x00000001;
pub const ECRYPTFS_DAEMON_IN_POLL: c_uint = 0x00000002;
pub const ECRYPTFS_DAEMON_ZOMBIE: c_uint = 0x00000004;
pub const ECRYPTFS_DAEMON_MISCDEV_OPEN: c_uint = 0x00000008;
    pub flags: u32,
    pub num_queued_msg_ctx: u32,
    pub file: *mut file,
    pub mux: mutex,
    pub msg_ctx_out_queue: list_head,
    pub wait: wait_queue_head_t,
    pub euid_chain: hlist_node,
}

extern "C" {
    pub fn container_of(_arg: inode, ecryptfs_inode_info: struct, _arg: vfs_inode) -> return;
}

extern "C" {
    pub fn __ecryptfs_printk(fmt: *const c_char, ...);
}
extern "C" {
    pub fn ecryptfs_i_size_init(page_virt: *const c_char, inode: *mut inode);
}
extern "C" {
    pub fn ecryptfs_dump_hex(data: *mut c_char, bytes: c_int);
}
extern "C" {
    pub fn ecryptfs_compute_root_iv(crypt_stat: *mut ecryptfs_crypt_stat) -> c_int;
}
extern "C" {
    pub fn ecryptfs_rotate_iv(iv: *mut c_uchar);
}
extern "C" {
    pub fn ecryptfs_init_crypt_stat(crypt_stat: *mut ecryptfs_crypt_stat);
}
extern "C" {
    pub fn ecryptfs_destroy_crypt_stat(crypt_stat: *mut ecryptfs_crypt_stat);
}
extern "C" {
    pub fn ecryptfs_init_crypt_ctx(crypt_stat: *mut ecryptfs_crypt_stat) -> c_int;
}
extern "C" {
    pub fn ecryptfs_write_inode_size_to_metadata(ecryptfs_inode: *mut inode) -> c_int;
}
extern "C" {
    pub fn ecryptfs_encrypt_page(folio: *mut folio) -> c_int;
}
extern "C" {
    pub fn ecryptfs_decrypt_page(folio: *mut folio) -> c_int;
}
extern "C" {
    pub fn ecryptfs_read_metadata(ecryptfs_dentry: *mut dentry) -> c_int;
}
extern "C" {
    pub fn ecryptfs_new_file_context(ecryptfs_inode: *mut inode) -> c_int;
}
extern "C" {
    pub fn ecryptfs_read_and_validate_header_region(inode: *mut inode) -> c_int;
}
extern "C" {
    pub fn ecryptfs_code_for_cipher_string(cipher_name: *mut c_char, key_bytes: usize) -> u8;
}
extern "C" {
    pub fn ecryptfs_cipher_code_to_string(str: *mut c_char, size: usize, cipher_code: u8) -> c_int;
}
extern "C" {
    pub fn ecryptfs_set_default_sizes(crypt_stat: *mut ecryptfs_crypt_stat);
}
extern "C" {
    pub fn ecryptfs_truncate(dentry: *mut dentry, new_length: loff_t) -> c_int;
}
extern "C" {
    pub fn ecryptfs_read_xattr_region(page_virt: *mut c_char, ecryptfs_inode: *mut inode) -> c_int;
}

extern "C" {
    pub fn ecryptfs_init_messaging() -> c_int;
}
extern "C" {
    pub fn ecryptfs_release_messaging();
}

extern "C" {
    pub fn ecryptfs_add_keysig(crypt_stat: *mut ecryptfs_crypt_stat, sig: *mut c_char) -> c_int;
}
extern "C" {
    pub fn ecryptfs_init_crypto() -> c_int;
}
extern "C" {
    pub fn ecryptfs_destroy_crypto() -> c_int;
}
extern "C" {
    pub fn ecryptfs_tfm_exists(cipher_name: *mut c_char, key_tfm: *mut ecryptfs_key_tfm) -> c_int;
}
extern "C" {
    pub fn ecryptfs_write(inode: *mut inode, data: *mut c_char, offset: loff_t, size: usize) -> c_int;
}

extern "C" {
    pub fn ecryptfs_init_ecryptfs_miscdev() -> c_int;
}
extern "C" {
    pub fn ecryptfs_destroy_ecryptfs_miscdev();
}
extern "C" {
    pub fn ecryptfs_msg_ctx_alloc_to_free(msg_ctx: *mut ecryptfs_msg_ctx);
}
extern "C" {
    pub fn ecryptfs_exorcise_daemon(daemon: *mut ecryptfs_daemon) -> c_int;
}
extern "C" {
    pub fn ecryptfs_find_daemon_by_euid(daemon: *mut ecryptfs_daemon) -> c_int;
}

extern "C" {
    pub fn ecryptfs_init_kthread() -> c_int;
}
extern "C" {
    pub fn ecryptfs_destroy_kthread();
}
extern "C" {
    pub fn ecryptfs_get_lower_file(dentry: *mut dentry, inode: *mut inode) -> c_int;
}
extern "C" {
    pub fn ecryptfs_put_lower_file(inode: *mut inode);
}
