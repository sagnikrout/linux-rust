//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/uv.h
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
// Ultravisor Interfaces
//
// Copyright IBM Corp. 2019, 2024
//
// Author(s):
// Vasily Gorbik <gor@linux.ibm.com>
// Janosch Frank <frankja@linux.ibm.com>
//

pub const UVC_CC_OK: c_int = 0;
pub const UVC_CC_ERROR: c_int = 1;
pub const UVC_CC_BUSY: c_int = 2;
pub const UVC_CC_PARTIAL: c_int = 3;
pub const UVC_RC_EXECUTED: c_uint = 0x0001;
pub const UVC_RC_INV_CMD: c_uint = 0x0002;
pub const UVC_RC_INV_STATE: c_uint = 0x0003;
pub const UVC_RC_INV_LEN: c_uint = 0x0005;
pub const UVC_RC_NO_RESUME: c_uint = 0x0007;
pub const UVC_RC_MORE_DATA: c_uint = 0x0100;
pub const UVC_RC_NEED_DESTROY: c_uint = 0x8000;
pub const UVC_CMD_QUI: c_uint = 0x0001;
pub const UVC_CMD_QUERY_KEYS: c_uint = 0x0002;
pub const UVC_CMD_INIT_UV: c_uint = 0x000f;
pub const UVC_CMD_CREATE_SEC_CONF: c_uint = 0x0100;
pub const UVC_CMD_DESTROY_SEC_CONF: c_uint = 0x0101;
pub const UVC_CMD_DESTROY_SEC_CONF_FAST: c_uint = 0x0102;
pub const UVC_CMD_CREATE_SEC_CPU: c_uint = 0x0120;
pub const UVC_CMD_DESTROY_SEC_CPU: c_uint = 0x0121;
pub const UVC_CMD_CONV_TO_SEC_STOR: c_uint = 0x0200;
pub const UVC_CMD_CONV_FROM_SEC_STOR: c_uint = 0x0201;
pub const UVC_CMD_DESTR_SEC_STOR: c_uint = 0x0202;
pub const UVC_CMD_SET_SEC_CONF_PARAMS: c_uint = 0x0300;
pub const UVC_CMD_UNPACK_IMG: c_uint = 0x0301;
pub const UVC_CMD_VERIFY_IMG: c_uint = 0x0302;
pub const UVC_CMD_CPU_RESET: c_uint = 0x0310;
pub const UVC_CMD_CPU_RESET_INITIAL: c_uint = 0x0311;
pub const UVC_CMD_PREPARE_RESET: c_uint = 0x0320;
pub const UVC_CMD_CPU_RESET_CLEAR: c_uint = 0x0321;
pub const UVC_CMD_CPU_SET_STATE: c_uint = 0x0330;
pub const UVC_CMD_SET_UNSHARE_ALL: c_uint = 0x0340;
pub const UVC_CMD_PIN_PAGE_SHARED: c_uint = 0x0341;
pub const UVC_CMD_UNPIN_PAGE_SHARED: c_uint = 0x0342;
pub const UVC_CMD_DUMP_INIT: c_uint = 0x0400;
pub const UVC_CMD_DUMP_CONF_STOR_STATE: c_uint = 0x0401;
pub const UVC_CMD_DUMP_CPU: c_uint = 0x0402;
pub const UVC_CMD_DUMP_COMPLETE: c_uint = 0x0403;
pub const UVC_CMD_SET_SHARED_ACCESS: c_uint = 0x1000;
pub const UVC_CMD_REMOVE_SHARED_ACCESS: c_uint = 0x1001;
pub const UVC_CMD_RETR_ATTEST: c_uint = 0x1020;
pub const UVC_CMD_ADD_SECRET: c_uint = 0x1031;
pub const UVC_CMD_LIST_SECRETS: c_uint = 0x1033;
pub const UVC_CMD_LOCK_SECRETS: c_uint = 0x1034;
pub const UVC_CMD_RETR_SECRET: c_uint = 0x1035;
// Bits in installed uv calls
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum uv_cmds_inst {
    BIT_UVC_CMD_QUI = 0,
    BIT_UVC_CMD_INIT_UV = 1,
    BIT_UVC_CMD_CREATE_SEC_CONF = 2,
    BIT_UVC_CMD_DESTROY_SEC_CONF = 3,
    BIT_UVC_CMD_CREATE_SEC_CPU = 4,
    BIT_UVC_CMD_DESTROY_SEC_CPU = 5,
    BIT_UVC_CMD_CONV_TO_SEC_STOR = 6,
    BIT_UVC_CMD_CONV_FROM_SEC_STOR = 7,
    BIT_UVC_CMD_SET_SHARED_ACCESS = 8,
    BIT_UVC_CMD_REMOVE_SHARED_ACCESS = 9,
    BIT_UVC_CMD_SET_SEC_PARMS = 11,
    BIT_UVC_CMD_UNPACK_IMG = 13,
    BIT_UVC_CMD_VERIFY_IMG = 14,
    BIT_UVC_CMD_CPU_RESET = 15,
    BIT_UVC_CMD_CPU_RESET_INITIAL = 16,
    BIT_UVC_CMD_CPU_SET_STATE = 17,
    BIT_UVC_CMD_PREPARE_RESET = 18,
    BIT_UVC_CMD_CPU_PERFORM_CLEAR_RESET = 19,
    BIT_UVC_CMD_UNSHARE_ALL = 20,
    BIT_UVC_CMD_PIN_PAGE_SHARED = 21,
    BIT_UVC_CMD_UNPIN_PAGE_SHARED = 22,
    BIT_UVC_CMD_DESTROY_SEC_CONF_FAST = 23,
    BIT_UVC_CMD_DUMP_INIT = 24,
    BIT_UVC_CMD_DUMP_CONFIG_STOR_STATE = 25,
    BIT_UVC_CMD_DUMP_CPU = 26,
    BIT_UVC_CMD_DUMP_COMPLETE = 27,
    BIT_UVC_CMD_RETR_ATTEST = 28,
    BIT_UVC_CMD_ADD_SECRET = 29,
    BIT_UVC_CMD_LIST_SECRETS = 30,
    BIT_UVC_CMD_LOCK_SECRETS = 31,
    BIT_UVC_CMD_RETR_SECRET = 33,
    BIT_UVC_CMD_QUERY_KEYS = 34,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum uv_feat_ind {
    BIT_UV_FEAT_MISC = 0,
    BIT_UV_FEAT_AIV = 1,
    BIT_UV_FEAT_AP = 4,
    BIT_UV_FEAT_AP_INTR = 5,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv_cb_header {
    pub len: u16,
    pub /: *mut *mut u16 cmd; / Command Code,
    pub /: *mut *mut u16 rc; / Response Code,
    pub /: *mut *mut u16 rrc; / Return Reason Code,
    pub __aligned(8): } __packed,
// Query Ultravisor Information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv_cb_qui {
    pub /: *mut *mut uv_cb_header header; / 0x0000,
    pub /: *mut *mut u64 reserved08; / 0x0008,
    pub /: *mut *mut u64 inst_calls_list[4]; / 0x0010,
    pub /: *mut *mut u64 reserved30[2]; / 0x0030,
    pub /: *mut *mut u64 uv_base_stor_len; / 0x0040,
    pub /: *mut *mut u64 reserved48; / 0x0048,
    pub /: *mut *mut u64 conf_base_phys_stor_len; / 0x0050,
    pub /: *mut *mut u64 conf_base_virt_stor_len; / 0x0058,
    pub /: *mut *mut u64 conf_virt_var_stor_len; / 0x0060,
    pub /: *mut *mut u64 cpu_stor_len; / 0x0068,
    pub /: *mut *mut u32 reserved70[3]; / 0x0070,
    pub /: *mut *mut u32 max_num_sec_conf; / 0x007c,
    pub /: *mut *mut u64 max_guest_stor_addr; / 0x0080,
    pub /: *mut *mut u8 reserved88[0x9e - 0x88]; / 0x0088,
    pub /: *mut *mut u16 max_guest_cpu_id; / 0x009e,
    pub /: *mut *mut u64 uv_feature_indications; / 0x00a0,
    pub /: *mut *mut u64 reserveda8; / 0x00a8,
    pub /: *mut *mut u64 supp_se_hdr_versions; / 0x00b0,
    pub /: *mut *mut u64 supp_se_hdr_pcf; / 0x00b8,
    pub /: *mut *mut u64 reservedc0; / 0x00c0,
    pub /: *mut *mut u64 conf_dump_storage_state_len; / 0x00c8,
    pub /: *mut *mut u64 conf_dump_finalize_len; / 0x00d0,
    pub /: *mut *mut u64 reservedd8; / 0x00d8,
    pub /: *mut *mut u64 supp_att_req_hdr_ver; / 0x00e0,
    pub /: *mut *mut u64 supp_att_pflags; / 0x00e8,
    pub /: *mut *mut u64 reservedf0; / 0x00f0,
    pub /: *mut *mut u64 supp_add_secret_req_ver; / 0x00f8,
    pub /: *mut *mut u64 supp_add_secret_pcf; / 0x0100,
    pub /: *mut *mut u64 supp_secret_types; / 0x0108,
    pub /: *mut *mut u16 max_assoc_secrets; / 0x0110,
    pub /: *mut *mut u16 max_retr_secrets; / 0x0112,
    pub /: *mut *mut u8 reserved114[0x120 - 0x114]; / 0x0114,
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv_key_hash {
    pub dword: [u64; 4],
    pub __aligned(8): } __packed,
pub const UVC_QUERY_KEYS_IDX_HK: c_int = 0;
pub const UVC_QUERY_KEYS_IDX_BACK_HK: c_int = 1;
// Query Ultravisor Keys
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv_cb_query_keys {
    pub /: *mut *mut uv_cb_header header; / 0x0000,
    pub /: *mut *mut u64 reserved08[3]; / 0x0008,
    pub /: *mut *mut uv_key_hash key_hashes[15]; / 0x0020,
    pub __aligned(8): } __packed,
    pub 0x200): static_assert(sizeof(struct uv_cb_query_keys) ==,
// Initialize Ultravisor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv_cb_init {
    pub header: uv_cb_header,
    pub reserved08: [u64; 2],
    pub stor_origin: u64,
    pub stor_len: u64,
    pub reserved28: [u64; 4],
    pub __aligned(8): } __packed,
// Create Guest Configuration
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv_cb_cgc {
    pub header: uv_cb_header,
    pub reserved08: [u64; 2],
    pub guest_handle: u64,
    pub conf_base_stor_origin: u64,
    pub conf_virt_stor_origin: u64,
    pub reserved30: [u8; 6],
    pub 14: u16 :,
    pub 1: u16 ap_instr_intr :,
    pub 1: u16 ap_allow_instr :,
}

// Create Secure CPU
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv_cb_csc {
    pub header: uv_cb_header,
    pub reserved08: [u64; 2],
    pub cpu_handle: u64,
    pub guest_handle: u64,
    pub stor_origin: u64,
    pub reserved30: [u8; 6],
    pub num: u16,
    pub state_origin: u64,
    pub reserved40: [u64; 4],
    pub __aligned(8): } __packed,
// Convert to Secure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv_cb_cts {
    pub header: uv_cb_header,
    pub reserved08: [u64; 2],
    pub guest_handle: u64,
    pub gaddr: u64,
    pub __aligned(8): } __packed,
// Convert from Secure / Pin Page Shared
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv_cb_cfs {
    pub header: uv_cb_header,
    pub reserved08: [u64; 2],
    pub paddr: u64,
    pub __aligned(8): } __packed,
// Set Secure Config Parameter
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv_cb_ssc {
    pub header: uv_cb_header,
    pub reserved08: [u64; 2],
    pub guest_handle: u64,
    pub sec_header_origin: u64,
    pub sec_header_len: u32,
    pub reserved2c: u32,
    pub reserved30: [u64; 4],
    pub __aligned(8): } __packed,
// Unpack
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv_cb_unp {
    pub header: uv_cb_header,
    pub reserved08: [u64; 2],
    pub guest_handle: u64,
    pub gaddr: u64,
    pub tweak: [u64; 2],
    pub reserved38: [u64; 3],
    pub __aligned(8): } __packed,
pub const PV_CPU_STATE_OPR: c_int = 1;
pub const PV_CPU_STATE_STP: c_int = 2;
pub const PV_CPU_STATE_CHKSTP: c_int = 3;
pub const PV_CPU_STATE_OPR_LOAD: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv_cb_cpu_set_state {
    pub header: uv_cb_header,
    pub reserved08: [u64; 2],
    pub cpu_handle: u64,
    pub reserved20: [u8; 7],
    pub state: u8,
    pub reserved28: [u64; 5],
}

//
// A common UV call struct for calls that take no payload
// Examples:
// Destroy cpu/config
// Verify
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv_cb_nodata {
    pub header: uv_cb_header,
    pub reserved08: [u64; 2],
    pub handle: u64,
    pub reserved20: [u64; 4],
    pub __aligned(8): } __packed,
// Destroy Configuration Fast
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv_cb_destroy_fast {
    pub header: uv_cb_header,
    pub reserved08: [u64; 2],
    pub handle: u64,
    pub reserved20: [u64; 5],
    pub __aligned(8): } __packed,
// Set Shared Access
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv_cb_share {
    pub header: uv_cb_header,
    pub reserved08: [u64; 3],
    pub paddr: u64,
    pub reserved28: u64,
    pub __aligned(8): } __packed,
// Retrieve Attestation Measurement
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv_cb_attest {
    pub /: *mut *mut uv_cb_header header; / 0x0000,
    pub /: *mut *mut u64 reserved08[2]; / 0x0008,
    pub /: *mut *mut u64 arcb_addr; / 0x0018,
    pub /: *mut *mut u64 cont_token; / 0x0020,
    pub /: *mut *mut u8 reserved28[6]; / 0x0028,
    pub /: *mut *mut u16 user_data_len; / 0x002e,
    pub /: *mut *mut u8 user_data[256]; / 0x0030,
    pub /: *mut *mut u32 reserved130[3]; / 0x0130,
    pub /: *mut *mut u32 meas_len; / 0x013c,
    pub /: *mut *mut u64 meas_addr; / 0x0140,
    pub /: *mut *mut u8 config_uid[16]; / 0x0148,
    pub /: *mut *mut u32 reserved158; / 0x0158,
    pub /: *mut *mut u32 add_data_len; / 0x015c,
    pub /: *mut *mut u64 add_data_addr; / 0x0160,
    pub /: *mut *mut u64 reserved168[4]; / 0x0168,
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv_cb_dump_cpu {
    pub header: uv_cb_header,
    pub reserved08: [u64; 2],
    pub cpu_handle: u64,
    pub dump_area_origin: u64,
    pub reserved28: [u64; 5],
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv_cb_dump_stor_state {
    pub header: uv_cb_header,
    pub reserved08: [u64; 2],
    pub config_handle: u64,
    pub dump_area_origin: u64,
    pub gaddr: u64,
    pub reserved28: [u64; 4],
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv_cb_dump_complete {
    pub header: uv_cb_header,
    pub reserved08: [u64; 2],
    pub config_handle: u64,
    pub dump_area_origin: u64,
    pub reserved30: [u64; 5],
    pub __aligned(8): } __packed,
//
// A common UV call struct for pv guests that contains a single address
// Examples:
// Add Secret
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv_cb_guest_addr {
    pub header: uv_cb_header,
    pub reserved08: [u64; 3],
    pub addr: u64,
    pub reserved28: [u64; 4],
    pub __aligned(8): } __packed,
pub const UVC_RC_RETR_SECR_BUF_SMALL: c_uint = 0x0109;
pub const UVC_RC_RETR_SECR_STORE_EMPTY: c_uint = 0x010f;
pub const UVC_RC_RETR_SECR_INV_IDX: c_uint = 0x0110;
pub const UVC_RC_RETR_SECR_INV_SECRET: c_uint = 0x0111;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv_cb_retr_secr {
    pub header: uv_cb_header,
    pub reserved08: [u64; 2],
    pub secret_idx: u16,
    pub reserved1a: u16,
    pub buf_size: u32,
    pub buf_addr: u64,
    pub reserved28: [u64; 4],
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv_cb_list_secrets {
    pub header: uv_cb_header,
    pub reserved08: [u64; 2],
    pub reserved18: [u8; 6],
    pub start_idx: u16,
    pub list_addr: u64,
    pub reserved28: [u64; 4],
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum uv_secret_types {
    UV_SECRET_INVAL = 0x0,
    UV_SECRET_NULL = 0x1,
    UV_SECRET_ASSOCIATION = 0x2,
    UV_SECRET_PLAIN = 0x3,
    UV_SECRET_AES_128 = 0x4,
    UV_SECRET_AES_192 = 0x5,
    UV_SECRET_AES_256 = 0x6,
    UV_SECRET_AES_XTS_128 = 0x7,
    UV_SECRET_AES_XTS_256 = 0x8,
    UV_SECRET_HMAC_SHA_256 = 0x9,
    UV_SECRET_HMAC_SHA_512 = 0xa,
// 0x0b - 0x10 reserved
    UV_SECRET_ECDSA_P256 = 0x11,
    UV_SECRET_ECDSA_P384 = 0x12,
    UV_SECRET_ECDSA_P521 = 0x13,
    UV_SECRET_ECDSA_ED25519 = 0x14,
    UV_SECRET_ECDSA_ED448 = 0x15,
}

//
// uv_secret_list_item_hdr - UV secret metadata.
// @index: Index of the secret in the secret list.
// @type: Type of the secret. See `enum uv_secret_types`.
// @length: Length of the stored secret.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv_secret_list_item_hdr {
    pub index: u16,
    pub type: u16,
    pub length: u32,
    pub __aligned(8): } __packed,
pub const UV_SECRET_ID_LEN: c_int = 32;
//
// uv_secret_list_item - UV secret entry.
// @hdr: The metadata of this secret.
// @id: The ID of this secret, not the secret itself.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv_secret_list_item {
    pub hdr: uv_secret_list_item_hdr,
    pub reserverd08: u64,
    pub id: [u8; UV_SECRET_ID_LEN],
    pub __aligned(8): } __packed,
//
// uv_secret_list - UV secret-metadata list.
// @num_secr_stored: Number of secrets stored in this list.
// @total_num_secrets: Number of secrets stored in the UV for this guest.
// @next_secret_idx: positive number if there are more secrets available or zero.
// @secrets: Up to 85 UV-secret metadata entries.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv_secret_list {
    pub num_secr_stored: u16,
    pub total_num_secrets: u16,
    pub next_secret_idx: u16,
    pub reserved_06: u16,
    pub reserved_08: u64,
    pub secrets: [uv_secret_list_item; 85],
    pub __aligned(8): } __packed,
    pub PAGE_SIZE): static_assert(sizeof(struct uv_secret_list) ==,
    pub cc: c_int,
    pub CC_CLOBBER_LIST("memory")): :,
    pub CC_TRANSFORM(cc): return,
    pub cc: c_int,
    pub r2): cc = __uv_call(r1,,
    pub 1): } while (cc >,
    pub cc: return,
//
// special variant of uv_call that only transports the cpu or guest
// handle and the command, like destroy or verify.
//
}

// rc = uvcb.header.rc;
// rrc = uvcb.header.rrc;
//
// uv_list_secrets() - Do a List Secrets UVC.
//
// @buf: Buffer to write list into; size of one page.
// @start_idx: The smallest index that should be included in the list.
// For the fist invocation use 0.
// @rc: Pointer to store the return code or NULL.
// @rrc: Pointer to store the return reason code or NULL.
//
// This function calls the List Secrets UVC. The result is written into `buf`,
// that needs to be at least one page of writable memory.
// `buf` consists of:
// * %struct uv_secret_list_hdr
// * %struct uv_secret_list_item (multiple)
//
// For `start_idx` use _0_ for the first call. If there are more secrets available
// but could not fit into the page then `rc` is `UVC_RC_MORE_DATA`.
// In this case use `uv_secret_list_hdr.next_secret_idx` for `start_idx`.
//
// Context: might sleep.
//
// Return: The UVC condition code.
//
// rc = uvcb.header.rc;
// rrc = uvcb.header.rrc;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv_info {
    pub inst_calls_list: [c_ulong; 4],
    pub uv_base_stor_len: c_ulong,
    pub guest_base_stor_len: c_ulong,
    pub guest_virt_base_stor_len: c_ulong,
    pub guest_virt_var_stor_len: c_ulong,
    pub guest_cpu_stor_len: c_ulong,
    pub max_sec_stor_addr: c_ulong,
    pub max_num_sec_conf: c_uint,
    pub max_guest_cpu_id: c_ushort,
    pub uv_feature_indications: c_ulong,
    pub supp_se_hdr_ver: c_ulong,
    pub supp_se_hdr_pcf: c_ulong,
    pub conf_dump_storage_state_len: c_ulong,
    pub conf_dump_finalize_len: c_ulong,
    pub supp_att_req_hdr_ver: c_ulong,
    pub supp_att_pflags: c_ulong,
    pub supp_add_secret_req_ver: c_ulong,
    pub supp_add_secret_pcf: c_ulong,
    pub supp_secret_types: c_ulong,
    pub max_assoc_secrets: c_ushort,
    pub max_retr_secrets: c_ushort,
}

extern "C" {
    pub fn test_bit_inv(_arg: feature_bit, _arg: &uv_info.uv_feature_indications) -> return;
}
//
// Sharing is page wise, if we encounter addresses that are
// not page aligned, we assume something went wrong. If
// malloced structs are passed to this function, we could leak
// data to the hypervisor.
//
// Guest 2 request to the Ultravisor to make a page shared with the
// hypervisor for IO.
//
// @addr: Real or absolute address of the page to be shared
//
extern "C" {
    pub fn share(_arg: addr, _arg: UVC_CMD_SET_SHARED_ACCESS) -> return;
}
//
// Guest 2 request to the Ultravisor to make a page unshared.
//
// @addr: Real or absolute address of the page to be unshared
//
extern "C" {
    pub fn share(_arg: addr, _arg: UVC_CMD_REMOVE_SHARED_ACCESS) -> return;
}
extern "C" {
    pub fn uv_retrieve_secret(secret_idx: u16, buf: *mut u8, buf_size: usize) -> c_int;
}
extern "C" {
    pub fn uv_pin_shared(paddr: c_ulong) -> c_int;
}
extern "C" {
    pub fn uv_destroy_folio(folio: *mut folio) -> c_int;
}
extern "C" {
    pub fn uv_destroy_pte(pte: pte_t) -> c_int;
}
extern "C" {
    pub fn uv_convert_from_secure_pte(pte: pte_t) -> c_int;
}
extern "C" {
    pub fn s390_wiggle_split_folio(mm: *mut mm_struct, folio: *mut folio) -> c_int;
}
extern "C" {
    pub fn __make_folio_secure(folio: *mut folio, uvcb: *mut uv_cb_header) -> c_int;
}
extern "C" {
    pub fn uv_convert_from_secure(paddr: c_ulong) -> c_int;
}
extern "C" {
    pub fn uv_convert_from_secure_folio(folio: *mut folio) -> c_int;
}
extern "C" {
    pub fn uv_free_stor_var(stor_var: *mut c_void);
}
extern "C" {
    pub fn setup_uv();
}
