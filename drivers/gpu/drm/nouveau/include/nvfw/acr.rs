//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvfw/acr.h
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


#[repr(C)]
#[derive(Copy, Clone)]
pub struct wpr_header {
pub const WPR_HEADER_V0_FALCON_ID_INVALID: c_uint = 0xffffffff;
    pub falcon_id: u32,
    pub lsb_offset: u32,
    pub bootstrap_owner: u32,
    pub lazy_bootstrap: u32,
pub const WPR_HEADER_V0_STATUS_NONE: c_int = 0;
pub const WPR_HEADER_V0_STATUS_COPY: c_int = 1;
pub const WPR_HEADER_V0_STATUS_VALIDATION_CODE_FAILED: c_int = 2;
pub const WPR_HEADER_V0_STATUS_VALIDATION_DATA_FAILED: c_int = 3;
pub const WPR_HEADER_V0_STATUS_VALIDATION_DONE: c_int = 4;
pub const WPR_HEADER_V0_STATUS_VALIDATION_SKIPPED: c_int = 5;
pub const WPR_HEADER_V0_STATUS_BOOTSTRAP_READY: c_int = 6;
    pub status: u32,
}

extern "C" {
    pub fn wpr_header_dump(: *mut nvkm_subdev, : *const wpr_header);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wpr_header_v1 {
pub const WPR_HEADER_V1_FALCON_ID_INVALID: c_uint = 0xffffffff;
    pub falcon_id: u32,
    pub lsb_offset: u32,
    pub bootstrap_owner: u32,
    pub lazy_bootstrap: u32,
    pub bin_version: u32,
pub const WPR_HEADER_V1_STATUS_NONE: c_int = 0;
pub const WPR_HEADER_V1_STATUS_COPY: c_int = 1;
pub const WPR_HEADER_V1_STATUS_VALIDATION_CODE_FAILED: c_int = 2;
pub const WPR_HEADER_V1_STATUS_VALIDATION_DATA_FAILED: c_int = 3;
pub const WPR_HEADER_V1_STATUS_VALIDATION_DONE: c_int = 4;
pub const WPR_HEADER_V1_STATUS_VALIDATION_SKIPPED: c_int = 5;
pub const WPR_HEADER_V1_STATUS_BOOTSTRAP_READY: c_int = 6;
pub const WPR_HEADER_V1_STATUS_REVOCATION_CHECK_FAILED: c_int = 7;
    pub status: u32,
}

extern "C" {
    pub fn wpr_header_v1_dump(: *mut nvkm_subdev, : *const wpr_header_v1);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wpr_generic_header {
pub const WPR_GENERIC_HEADER_ID_LSF_UCODE_DESC: c_int = 1;
pub const WPR_GENERIC_HEADER_ID_LSF_WPR_HEADER: c_int = 2;
pub const WPR_GENERIC_HEADER_ID_LSF_SHARED_SUB_WPR: c_int = 3;
pub const WPR_GENERIC_HEADER_ID_LSF_LSB_HEADER: c_int = 4;
    pub identifier: u16,
    pub version: u16,
    pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wpr_header_v2 {
    pub hdr: wpr_generic_header,
    pub wpr: wpr_header_v1,
}

extern "C" {
    pub fn wpr_header_v2_dump(: *mut nvkm_subdev, : *const wpr_header_v2);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lsf_signature {
    pub prd_keys: [u8; 2][16],
    pub dbg_keys: [u8; 2][16],
    pub b_prd_present: u32,
    pub b_dbg_present: u32,
    pub falcon_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lsf_signature_v1 {
    pub prd_keys: [u8; 2][16],
    pub dbg_keys: [u8; 2][16],
    pub b_prd_present: u32,
    pub b_dbg_present: u32,
    pub falcon_id: u32,
    pub supports_versioning: u32,
    pub version: u32,
    pub depmap_count: u32,
    pub 4]: *mut *mut *mut *mut *mut u8 depmap[11/LSF_LSB_DEPMAP_SIZE/  2,
    pub kdf: [u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lsb_header_tail {
    pub ucode_off: u32,
    pub ucode_size: u32,
    pub data_size: u32,
    pub bl_code_size: u32,
    pub bl_imem_off: u32,
    pub bl_data_off: u32,
    pub bl_data_size: u32,
    pub app_code_off: u32,
    pub app_code_size: u32,
    pub app_data_off: u32,
    pub app_data_size: u32,
    pub flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lsb_header {
    pub signature: lsf_signature,
    pub tail: lsb_header_tail,
}

extern "C" {
    pub fn lsb_header_dump(: *mut nvkm_subdev, : *mut lsb_header);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lsb_header_v1 {
    pub signature: lsf_signature_v1,
    pub tail: lsb_header_tail,
}

extern "C" {
    pub fn lsb_header_v1_dump(: *mut nvkm_subdev, : *mut lsb_header_v1);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lsb_header_v2 {
    pub hdr: wpr_generic_header,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lsf_signature_v2 {
    pub hdr: wpr_generic_header,
    pub falcon_id: u32,
    pub prd_present: u8,
    pub dbg_present: u8,
    pub reserved: u16,
    pub sig_size: u32,
    pub 128]: u8 prod_sig[2][384 +,
    pub 128]: u8 debug_sig[2][384 +,
    pub sig_algo_ver: u16,
    pub sig_algo: u16,
    pub hash_algo_ver: u16,
    pub hash_algo: u16,
    pub sig_algo_padding_type: u32,
    pub 4]: *mut *mut *mut u8 depmap[11  2,
    pub depmap_count: u32,
    pub supports_versioning: u8,
    pub pad: [u8; 3],
    pub ls_ucode_version: u32,
    pub ls_ucode_id: u32,
    pub ucode_ls_encrypted: u32,
    pub ls_eng_algo_type: u32,
    pub ls_eng_algo_ver: u32,
    pub ls_enc_iv: [u8; 16],
    pub rsvd: [u8; 36],
    pub signature: },
    pub ucode_off: u32,
    pub ucode_size: u32,
    pub data_size: u32,
    pub bl_code_size: u32,
    pub bl_imem_off: u32,
    pub bl_data_off: u32,
    pub bl_data_size: u32,
    pub rsvd0: u32,
    pub app_code_off: u32,
    pub app_code_size: u32,
    pub app_data_off: u32,
    pub app_data_size: u32,
    pub app_imem_offset: u32,
    pub app_dmem_offset: u32,
    pub flags: u32,
    pub monitor_code_offset: u32,
    pub monitor_data_offset: u32,
    pub manifest_offset: u32,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hs_fmc_params {
    pub hs_fmc: u8,
    pub padding: [u8; 3],
    pub pkc_algo: u16,
    pub pkc_algo_version: u16,
    pub engid_mask: u32,
    pub ucode_id: u32,
    pub fuse_ver: u32,
    pub 128]: u8 pkc_signature[384 +,
    pub pkc_key: [u8; 2048],
    pub rsvd: [u8; 4],
    pub hs_fmc_params: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hs_ovl_sig_blob_params {
    pub hs_ovl_sig_blob_present: u8,
    pub hs_ovl_sig_blob_offset: u32,
    pub hs_ovl_sig_blob_size: u32,
    pub hs_ovl_sig_blob_params: },
    pub rsvd: [u8; 20],
}

extern "C" {
    pub fn lsb_header_v2_dump(: *mut nvkm_subdev, : *mut lsb_header_v2);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flcn_acr_desc {
    pub reserved_dmem: [u8; 0x200],
    pub signatures: [u32; 4],
    pub ucode_reserved_space: },
    pub wpr_region_id: u32,
    pub wpr_offset: u32,
    pub mmu_mem_range: u32,
    pub no_regions: u32,
    pub start_addr: u32,
    pub end_addr: u32,
    pub region_id: u32,
    pub read_mask: u32,
    pub write_mask: u32,
    pub client_mask: u32,
    pub region_props: [}; 2],
    pub regions: },
    pub ucode_blob_size: u32,
    pub __aligned(8): u64 ucode_blob_base,
    pub vpr_enabled: u32,
    pub vpr_start: u32,
    pub vpr_end: u32,
    pub hdcp_policies: u32,
    pub vpr_desc: },
}

extern "C" {
    pub fn flcn_acr_desc_dump(: *mut nvkm_subdev, : *mut flcn_acr_desc);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flcn_acr_desc_v1 {
    pub reserved_dmem: [u8; 0x200],
    pub signatures: [u32; 4],
    pub wpr_region_id: u32,
    pub wpr_offset: u32,
    pub mmu_memory_range: u32,
    pub no_regions: u32,
    pub start_addr: u32,
    pub end_addr: u32,
    pub region_id: u32,
    pub read_mask: u32,
    pub write_mask: u32,
    pub client_mask: u32,
    pub shadow_mem_start_addr: u32,
    pub region_props: [}; 2],
    pub regions: },
    pub ucode_blob_size: u32,
    pub __aligned(8): u64 ucode_blob_base,
    pub vpr_enabled: u32,
    pub vpr_start: u32,
    pub vpr_end: u32,
    pub hdcp_policies: u32,
    pub vpr_desc: },
}

extern "C" {
    pub fn flcn_acr_desc_v1_dump(: *mut nvkm_subdev, : *mut flcn_acr_desc_v1);
}
