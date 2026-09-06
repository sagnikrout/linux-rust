//! Automatically rewritten from C Header to Rust Module
//! Source: security/integrity/ima/ima.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2005,2006,2007,2008 IBM Corporation
//
// Authors:
// Reiner Sailer <sailer@watson.ibm.com>
// Mimi Zohar <zohar@us.ibm.com>
//
// File: ima.h
// internal Integrity Measurement Architecture (IMA) definitions
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ima_show_type {
    IMA_SHOW_BINARY_OLD_STRING_FMT, IMA_SHOW_ASCII };
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tpm_pcrs {

//
// BINARY: current binary measurements list
// BINARY_STAGED: staged binary measurements list
// BINARY_FULL: binary measurements list since IMA init (lost after kexec)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum binary_lists {
    BINARY, BINARY_STAGED, BINARY_FULL, BINARY__LAST
}

// digest size for IMA, fits SHA1 or MD5

pub const IMA_EVENT_NAME_LEN_MAX: c_int = 255;
pub const IMA_HASH_BITS: c_int = 10;

pub const IMA_TEMPLATE_FIELD_ID_MAX_LEN: c_int = 16;
pub const IMA_TEMPLATE_NUM_FIELDS_MAX: c_int = 15;

// current content of the policy
// bitset of digests algorithms allowed in the setxattr hook
// IMA hash algorithm description
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ima_algo_desc {
    pub tfm: *mut crypto_shash,
    pub algo: hash_algo,
    pub digest_size: c_uint,
}

// set during initialization
// IMA event related data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ima_event_data {
    pub iint: *mut ima_iint_cache,
    pub file: *mut file,
    pub filename: *const c_uchar,
    pub xattr_value: *mut evm_ima_xattr_data,
    pub xattr_len: c_int,
    pub modsig: *const modsig,
    pub violation: *const c_char,
    pub buf: *const c_void,
    pub buf_len: c_int,
}

// IMA template field data definition
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ima_field_data {
    pub data: *mut u8,
    pub len: u32,
}

// IMA template field definition
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ima_template_field {
    pub field_id: [c_char; IMA_TEMPLATE_FIELD_ID_MAX_LEN],
    pub field_data): *mut ima_field_data,
    pub field_data): *mut ima_field_data,
}

// IMA template descriptor definition
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ima_template_desc {
    pub list: list_head,
    pub name: *mut c_char,
    pub fmt: *mut c_char,
    pub num_fields: c_int,
    pub fields: *const ima_template_field,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ima_template_entry {
    pub pcr: c_int,
    pub digests: *mut tpm_digest,
    pub /: *mut *mut *mut ima_template_desc template_desc; / template descriptor,
    pub template_data_len: u32,
    pub /: *mut *mut ima_field_data template_data[]; / template related data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ima_queue_entry {
    pub /: *mut *mut hlist_node hnext; / place in hash collision list,
    pub /: *mut *mut list_head later; / place in ima_measurements list,
    pub entry: *mut ima_template_entry,
}

// Some details preceding the binary serialized measurement list
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ima_kexec_hdr {
    pub version: u16,
    pub _reserved0: u16,
    pub _reserved1: u32,
    pub buffer_size: u64,
    pub count: u64,
}

// IMA iint action cache flags
pub const IMA_MEASURE: c_uint = 0x00000001;
pub const IMA_MEASURED: c_uint = 0x00000002;
pub const IMA_APPRAISE: c_uint = 0x00000004;
pub const IMA_APPRAISED: c_uint = 0x00000008;
// #define IMA_COLLECT		0x00000010  do not use this flag
pub const IMA_COLLECTED: c_uint = 0x00000020;
pub const IMA_AUDIT: c_uint = 0x00000040;
pub const IMA_AUDITED: c_uint = 0x00000080;
pub const IMA_HASH: c_uint = 0x00000100;
pub const IMA_HASHED: c_uint = 0x00000200;
// IMA iint policy rule cache flags
pub const IMA_NONACTION_FLAGS: c_uint = 0xff000000;
pub const IMA_DIGSIG_REQUIRED: c_uint = 0x01000000;
pub const IMA_PERMIT_DIRECTIO: c_uint = 0x02000000;
pub const IMA_NEW_FILE: c_uint = 0x04000000;
pub const IMA_SIGV3_REQUIRED: c_uint = 0x08000000;
pub const IMA_FAIL_UNVERIFIABLE_SIGS: c_uint = 0x10000000;
pub const IMA_MODSIG_ALLOWED: c_uint = 0x20000000;
pub const IMA_CHECK_BLACKLIST: c_uint = 0x40000000;
pub const IMA_VERITY_REQUIRED: c_uint = 0x80000000;
// Exclude non-action flags which are not rule-specific.

// IMA iint subaction appraise cache flags
pub const IMA_FILE_APPRAISE: c_uint = 0x00001000;
pub const IMA_FILE_APPRAISED: c_uint = 0x00002000;
pub const IMA_MMAP_APPRAISE: c_uint = 0x00004000;
pub const IMA_MMAP_APPRAISED: c_uint = 0x00008000;
pub const IMA_BPRM_APPRAISE: c_uint = 0x00010000;
pub const IMA_BPRM_APPRAISED: c_uint = 0x00020000;
pub const IMA_READ_APPRAISE: c_uint = 0x00040000;
pub const IMA_READ_APPRAISED: c_uint = 0x00080000;
pub const IMA_CREDS_APPRAISE: c_uint = 0x00100000;
pub const IMA_CREDS_APPRAISED: c_uint = 0x00200000;

//
// IMA iint cache atomic_flags
//
// IMA_CHANGE_ATTR - indicates that chATTR() was called (chmod, chown, chgrp)
// and file attributes have changed. On file open, it causes IMA to clear
// iint->flags to re-evaluate policy and perform IMA functions again.
//
// IMA_CHANGE_XATTR - indicates that setxattr or removexattr was called and
// extended attributes have changed. On file open, it causes IMA to clear
// iint->flags IMA_DONE_MASK to re-appraise.
//
// IMA_UPDATE_XATTR - indicates that security.ima needs to be updated. It is
// cleared if file policy changes and no update is needed.
//
// IMA_DIGSIG - indicates that file security.ima has signature and file
// security.ima must not update on file close.
//
// IMA_MAY_EMIT_TOMTOU - indicates to add Time-of-Measure-Time-of-Use (ToMToU)
// integrity violation (a file that is already opened for read is opened for
// write) to the measurement list and to also emit an audit message.
//
// IMA_EMITTED_OPENWRITERS - indicates to add open-writers integrity violation
// (a file that is already opened for write is opened for read) to the
// measurement list and to also emit an audit message.
//
pub const IMA_CHANGE_XATTR: c_int = 0;
pub const IMA_UPDATE_XATTR: c_int = 1;
pub const IMA_CHANGE_ATTR: c_int = 2;
pub const IMA_DIGSIG: c_int = 3;
pub const IMA_MAY_EMIT_TOMTOU: c_int = 4;
pub const IMA_EMITTED_OPENWRITERS: c_int = 5;
// IMA integrity metadata associated with an inode
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ima_iint_cache {
    pub /: *mut *mut mutex mutex; / protects: version, flags, digest,
    pub real_inode: integrity_inode_attributes,
    pub flags: c_ulong,
    pub measured_pcrs: c_ulong,
    pub atomic_flags: c_ulong,
    pub ima_file_status:4: integrity_status,
    pub ima_mmap_status:4: integrity_status,
    pub ima_bprm_status:4: integrity_status,
    pub ima_read_status:4: integrity_status,
    pub ima_creds_status:4: integrity_status,
    pub ima_hash: *mut ima_digest_data,
}

// iint_sec = iint;
extern "C" {
    pub fn ima_inode_free_rcu(inode_security: *mut c_void);
}
extern "C" {
    pub fn ima_iintcache_init() -> void __init;
}

extern "C" {
    pub fn ima_load_kexec_buffer();
}

extern "C" {
    pub fn ima_measure_kexec_event(event_name: *const c_char);
}

//
// The default binary_runtime_measurements list format is defined as the
// platform native format.  The canonical format is defined as little-endian.
//
// Internal IMA function definitions
extern "C" {
    pub fn ima_init() -> c_int;
}
extern "C" {
    pub fn ima_fs_init() -> c_int;
}
extern "C" {
    pub fn ima_calc_file_hash(file: *mut file, hash: *mut ima_digest_data) -> c_int;
}
extern "C" {
    pub fn ima_calc_boot_aggregate(hash: *mut ima_digest_data) -> c_int;
}
extern "C" {
    pub fn ima_init_crypto() -> c_int;
}
extern "C" {
    pub fn ima_putc(m: *mut seq_file, data: *mut c_void, datalen: c_int);
}
extern "C" {
    pub fn ima_print_digest(m: *mut seq_file, digest: *mut u8, size: u32);
}
extern "C" {
    pub fn ima_template_has_modsig(ima_template: *const ima_template_desc) -> bool;
}
extern "C" {
    pub fn ima_queue_stage() -> c_int;
}
extern "C" {
    pub fn ima_queue_staged_delete_all() -> c_int;
}
extern "C" {
    pub fn ima_queue_delete_partial(req_value: c_ulong) -> c_int;
}
extern "C" {
    pub fn ima_restore_measurement_entry(entry: *mut ima_template_entry) -> c_int;
}
extern "C" {
    pub fn ima_restore_measurement_list(bufsize: loff_t, buf: *mut c_void) -> c_int;
}
extern "C" {
    pub fn ima_measurements_show(m: *mut seq_file, v: *mut c_void) -> c_int;
}
extern "C" {
    pub fn ima_init_htable() -> int __init;
}
extern "C" {
    pub fn ima_get_binary_runtime_size(binary_list: binary_lists) -> c_ulong;
}
extern "C" {
    pub fn ima_init_template() -> c_int;
}
extern "C" {
    pub fn ima_init_template_list();
}
extern "C" {
    pub fn ima_init_digests() -> int __init;
}
extern "C" {
    pub fn ima_init_reboot_notifier() -> void __init;
}
//
// used to protect h_table and sha_table
//
// Total number of measurement list records since hard boot.
// Total number of violations since hard boot.
// there is no point in taking a hash of part of a digest

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ima_hooks {
    __ima_hooks(__ima_hook_enumify)
}

//
// To track keys that need to be measured.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ima_key_entry {
    pub list: list_head,
    pub payload: *mut c_void,
    pub payload_len: usize,
    pub keyring_name: *mut c_char,
}

extern "C" {
    pub fn ima_init_key_queue();
}
extern "C" {
    pub fn ima_should_queue_key() -> bool;
}
extern "C" {
    pub fn ima_process_queued_keys();
}

// LIM API function definitions
extern "C" {
    pub fn ima_must_measure(inode: *mut inode, mask: c_int, func: ima_hooks) -> c_int;
}
extern "C" {
    pub fn ima_free_template_entry(entry: *mut ima_template_entry);
}
// IMA policy related functions
extern "C" {
    pub fn ima_init_policy();
}
extern "C" {
    pub fn ima_update_policy();
}
extern "C" {
    pub fn ima_update_policy_flags();
}
extern "C" {
    pub fn ima_parse_add_rule(: *mut c_char) -> isize;
}
extern "C" {
    pub fn ima_delete_rules();
}
extern "C" {
    pub fn ima_check_policy() -> c_int;
}
extern "C" {
    pub fn ima_policy_stop(m: *mut seq_file, v: *mut c_void);
}
extern "C" {
    pub fn ima_policy_show(m: *mut seq_file, v: *mut c_void) -> c_int;
}
extern "C" {
    pub fn ima_measure_loaded_policy();
}
extern "C" {
    pub fn ima_measure_raw_policy(buf: *const c_char, buf_len: usize) -> c_int;
}
// Appraise integrity measurements
pub const IMA_APPRAISE_ENFORCE: c_uint = 0x01;
pub const IMA_APPRAISE_FIX: c_uint = 0x02;
pub const IMA_APPRAISE_LOG: c_uint = 0x04;
pub const IMA_APPRAISE_MODULES: c_uint = 0x08;
pub const IMA_APPRAISE_FIRMWARE: c_uint = 0x10;
pub const IMA_APPRAISE_POLICY: c_uint = 0x20;
pub const IMA_APPRAISE_KEXEC: c_uint = 0x40;

extern "C" {
    pub fn ima_update_xattr(iint: *mut ima_iint_cache, file: *mut file);
}
extern "C" {
    pub fn init_ima_appraise_lsm(lsmid: *const lsm_id) -> void __init;
}

extern "C" {
    pub fn ima_collect_modsig(modsig: *mut modsig, buf: *const c_void, size: loff_t);
}
extern "C" {
    pub fn ima_free_modsig(modsig: *mut modsig);
}

// LSM based policy rules require audit

