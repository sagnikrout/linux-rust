//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/intel/qat/qat_common/icp_qat_uclo.h
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


// SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only)
// Copyright(c) 2014 - 2020 Intel Corporation
pub const ICP_QAT_AC_895XCC_DEV_TYPE: c_uint = 0x00400000;
pub const ICP_QAT_AC_C62X_DEV_TYPE: c_uint = 0x01000000;
pub const ICP_QAT_AC_C3XXX_DEV_TYPE: c_uint = 0x02000000;
pub const ICP_QAT_AC_4XXX_A_DEV_TYPE: c_uint = 0x08000000;
pub const ICP_QAT_AC_6XXX_DEV_TYPE: c_uint = 0x80000000;
pub const ICP_QAT_UCLO_MAX_AE: c_int = 17;
pub const ICP_QAT_UCLO_MAX_CTX: c_int = 8;

pub const ICP_QAT_UCLO_MAX_USTORE: c_uint = 0x4000;
pub const ICP_QAT_UCLO_MAX_XFER_REG: c_int = 128;
pub const ICP_QAT_UCLO_MAX_GPR_REG: c_int = 128;
pub const ICP_QAT_UCLO_MAX_LMEM_REG: c_int = 1024;
pub const ICP_QAT_UCLO_MAX_LMEM_REG_2X: c_int = 1280;
pub const ICP_QAT_UCLO_AE_ALL_CTX: c_uint = 0xff;
pub const ICP_QAT_UOF_OBJID_LEN: c_int = 8;
pub const ICP_QAT_UOF_FID: c_uint = 0xc6c2;
pub const ICP_QAT_UOF_MAJVER: c_uint = 0x4;
pub const ICP_QAT_UOF_MINVER: c_uint = 0x11;

pub const ICP_QAT_UOF_LOCAL_SCOPE: c_int = 1;
pub const ICP_QAT_UOF_INIT_EXPR: c_int = 0;
pub const ICP_QAT_UOF_INIT_REG: c_int = 1;
pub const ICP_QAT_UOF_INIT_REG_CTX: c_int = 2;
pub const ICP_QAT_UOF_INIT_EXPR_ENDIAN_SWAP: c_int = 3;
pub const ICP_QAT_SUOF_OBJ_ID_LEN: c_int = 8;
pub const ICP_QAT_SUOF_FID: c_uint = 0x53554f46;
pub const ICP_QAT_SUOF_MAJVER: c_uint = 0x0;
pub const ICP_QAT_SUOF_MINVER: c_uint = 0x1;
pub const ICP_QAT_SUOF_OBJ_NAME_LEN: c_int = 128;
pub const ICP_QAT_MOF_OBJ_ID_LEN: c_int = 8;
pub const ICP_QAT_MOF_OBJ_CHUNKID_LEN: c_int = 8;
pub const ICP_QAT_MOF_FID: c_uint = 0x00666f6d;
pub const ICP_QAT_MOF_MAJVER: c_uint = 0x0;
pub const ICP_QAT_MOF_MINVER: c_uint = 0x1;

pub const DSS_FWSK_EXPONENT_LEN: c_int = 4;
pub const DSS_FWSK_PADDING_LEN: c_int = 380;

pub const CSS_FWSK_EXPONENT_LEN: c_int = 4;
pub const CSS_FWSK_PADDING_LEN: c_int = 252;

pub const ICP_QAT_CSS_RSA4K_MAX_IMAGE_LEN: c_uint = 0x40000;
pub const ICP_QAT_CSS_RSA3K_MAX_IMAGE_LEN: c_uint = 0x30000;
// All lengths below are in bytes
pub const ICP_QAT_DUALSIGN_OPAQUE_HDR_LEN: c_int = 12;
pub const ICP_QAT_DUALSIGN_OPAQUE_HDR_ALIGN_LEN: c_int = 16;
pub const ICP_QAT_DUALSIGN_OPAQUE_DATA_LEN: c_int = 3540;
pub const ICP_QAT_DUALSIGN_XMSS_PUBKEY_LEN: c_int = 64;
pub const ICP_QAT_DUALSIGN_XMSS_SIG_LEN: c_int = 2692;
pub const ICP_QAT_DUALSIGN_XMSS_SIG_ALIGN_LEN: c_int = 2696;
pub const ICP_QAT_DUALSIGN_MISC_INFO_LEN: c_int = 16;
pub const ICP_QAT_DUALSIGN_FW_TYPE_LEN: c_int = 7;
pub const ICP_QAT_DUALSIGN_MODULE_TYPE: c_uint = 0x14;
pub const ICP_QAT_DUALSIGN_HDR_LEN: c_uint = 0x375;
pub const ICP_QAT_DUALSIGN_HDR_VER: c_uint = 0x40001;
pub const ICP_QAT_DUALSIGN_HDR_LEN_OFFSET: c_int = 4;
pub const ICP_QAT_DUALSIGN_HDR_VER_OFFSET: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icp_qat_uof_mem_region {
    ICP_QAT_UOF_SRAM_REGION = 0x0,
    ICP_QAT_UOF_LMEM_REGION = 0x3,
    ICP_QAT_UOF_UMEM_REGION = 0x5
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icp_qat_uof_regtype {
    ICP_NO_DEST	= 0,
    ICP_GPA_REL	= 1,
    ICP_GPA_ABS	= 2,
    ICP_GPB_REL	= 3,
    ICP_GPB_ABS	= 4,
    ICP_SR_REL	= 5,
    ICP_SR_RD_REL	= 6,
    ICP_SR_WR_REL	= 7,
    ICP_SR_ABS	= 8,
    ICP_SR_RD_ABS	= 9,
    ICP_SR_WR_ABS	= 10,
    ICP_DR_REL	= 19,
    ICP_DR_RD_REL	= 20,
    ICP_DR_WR_REL	= 21,
    ICP_DR_ABS	= 22,
    ICP_DR_RD_ABS	= 23,
    ICP_DR_WR_ABS	= 24,
    ICP_LMEM	= 26,
    ICP_LMEM0	= 27,
    ICP_LMEM1	= 28,
    ICP_NEIGH_REL	= 31,
    ICP_LMEM2	= 61,
    ICP_LMEM3	= 62,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icp_qat_css_fwtype {
    CSS_AE_FIRMWARE = 0,
    CSS_MMP_FIRMWARE = 1
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_uclo_page {
    pub encap_page: *mut icp_qat_uclo_encap_page,
    pub region: *mut icp_qat_uclo_region,
    pub flags: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_uclo_region {
    pub loaded: *mut icp_qat_uclo_page,
    pub page: *mut icp_qat_uclo_page,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_uclo_aeslice {
    pub region: *mut icp_qat_uclo_region,
    pub page: *mut icp_qat_uclo_page,
    pub cur_page: [*mut icp_qat_uclo_page; ICP_QAT_UCLO_MAX_CTX],
    pub encap_image: *mut icp_qat_uclo_encapme,
    pub ctx_mask_assigned: c_uint,
    pub new_uaddr: [c_uint; ICP_QAT_UCLO_MAX_CTX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_uclo_aedata {
    pub slice_num: c_uint,
    pub eff_ustore_size: c_uint,
    pub ae_slices: [icp_qat_uclo_aeslice; ICP_QAT_UCLO_MAX_CTX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_uof_encap_obj {
    pub beg_uof: *mut c_char,
    pub obj_hdr: *mut icp_qat_uof_objhdr,
    pub chunk_hdr: *mut icp_qat_uof_chunkhdr,
    pub var_mem_seg: *mut icp_qat_uof_varmem_seg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_uclo_encap_uwblock {
    pub start_addr: c_uint,
    pub words_num: c_uint,
    pub micro_words: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_uclo_encap_page {
    pub def_page: c_uint,
    pub page_region: c_uint,
    pub beg_addr_v: c_uint,
    pub beg_addr_p: c_uint,
    pub micro_words_num: c_uint,
    pub uwblock_num: c_uint,
    pub uwblock: *mut icp_qat_uclo_encap_uwblock,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_uclo_encapme {
    pub img_ptr: *mut icp_qat_uof_image,
    pub page: *mut icp_qat_uclo_encap_page,
    pub ae_reg_num: c_uint,
    pub ae_reg: *mut icp_qat_uof_ae_reg,
    pub init_regsym_num: c_uint,
    pub init_regsym: *mut icp_qat_uof_init_regsym,
    pub sbreak_num: c_uint,
    pub sbreak: *mut icp_qat_uof_sbreak,
    pub uwords_num: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_uclo_init_mem_table {
    pub entry_num: c_uint,
    pub init_mem: *mut icp_qat_uof_initmem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_uclo_objhdr {
    pub file_buff: *mut c_char,
    pub checksum: c_uint,
    pub size: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_uof_strtable {
    pub table_len: c_uint,
    pub reserved: c_uint,
    pub strings: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_uclo_objhandle {
    pub prod_type: c_uint,
    pub prod_rev: c_uint,
    pub obj_hdr: *mut icp_qat_uclo_objhdr,
    pub encap_uof_obj: icp_qat_uof_encap_obj,
    pub str_table: icp_qat_uof_strtable,
    pub ae_uimage: [icp_qat_uclo_encapme; ICP_QAT_UCLO_MAX_UIMAGE],
    pub ae_data: [icp_qat_uclo_aedata; ICP_QAT_UCLO_MAX_AE],
    pub init_mem_tab: icp_qat_uclo_init_mem_table,
    pub lm_init_tab: [*mut icp_qat_uof_batch_init; ICP_QAT_UCLO_MAX_AE],
    pub umem_init_tab: [*mut icp_qat_uof_batch_init; ICP_QAT_UCLO_MAX_AE],
    pub uimage_num: c_int,
    pub uword_in_bytes: c_int,
    pub global_inited: c_int,
    pub ae_num: c_uint,
    pub ustore_phy_size: c_uint,
    pub obj_buf: *mut c_void,
    pub uword_buf: *mut u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_uof_uword_block {
    pub start_addr: c_uint,
    pub words_num: c_uint,
    pub uword_offset: c_uint,
    pub reserved: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_uof_filehdr {
    pub file_id: c_ushort,
    pub reserved1: c_ushort,
    pub min_ver: c_char,
    pub maj_ver: c_char,
    pub reserved2: c_ushort,
    pub max_chunks: c_ushort,
    pub num_chunks: c_ushort,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_uof_filechunkhdr {
    pub chunk_id: [c_char; ICP_QAT_UOF_OBJID_LEN],
    pub checksum: c_uint,
    pub offset: c_uint,
    pub size: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_uof_objhdr {
    pub ac_dev_type: c_uint,
    pub min_cpu_ver: c_ushort,
    pub max_cpu_ver: c_ushort,
    pub max_chunks: c_short,
    pub num_chunks: c_short,
    pub reserved1: c_uint,
    pub reserved2: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_uof_chunkhdr {
    pub chunk_id: [c_char; ICP_QAT_UOF_OBJID_LEN],
    pub offset: c_uint,
    pub size: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_uof_memvar_attr {
    pub offset_in_byte: c_uint,
    pub value: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_uof_initmem {
    pub sym_name: c_uint,
    pub region: c_char,
    pub scope: c_char,
    pub reserved1: c_ushort,
    pub addr: c_uint,
    pub num_in_bytes: c_uint,
    pub val_attr_num: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_uof_init_regsym {
    pub sym_name: c_uint,
    pub init_type: c_char,
    pub value_type: c_char,
    pub reg_type: c_char,
    pub ctx: c_uchar,
    pub reg_addr: c_uint,
    pub value: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_uof_varmem_seg {
    pub sram_base: c_uint,
    pub sram_size: c_uint,
    pub sram_alignment: c_uint,
    pub sdram_base: c_uint,
    pub sdram_size: c_uint,
    pub sdram_alignment: c_uint,
    pub sdram1_base: c_uint,
    pub sdram1_size: c_uint,
    pub sdram1_alignment: c_uint,
    pub scratch_base: c_uint,
    pub scratch_size: c_uint,
    pub scratch_alignment: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_uof_gtid {
    pub tool_id: [c_char; ICP_QAT_UOF_OBJID_LEN],
    pub tool_ver: c_int,
    pub reserved1: c_uint,
    pub reserved2: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_uof_sbreak {
    pub page_num: c_uint,
    pub virt_uaddr: c_uint,
    pub sbreak_type: c_uchar,
    pub reg_type: c_uchar,
    pub reserved1: c_ushort,
    pub addr_offset: c_uint,
    pub reg_addr: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_uof_code_page {
    pub page_region: c_uint,
    pub page_num: c_uint,
    pub def_page: c_uchar,
    pub reserved2: c_uchar,
    pub reserved1: c_ushort,
    pub beg_addr_v: c_uint,
    pub beg_addr_p: c_uint,
    pub neigh_reg_tab_offset: c_uint,
    pub uc_var_tab_offset: c_uint,
    pub imp_var_tab_offset: c_uint,
    pub imp_expr_tab_offset: c_uint,
    pub code_area_offset: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_uof_image {
    pub img_name: c_uint,
    pub ae_assigned: c_uint,
    pub ctx_assigned: c_uint,
    pub ac_dev_type: c_uint,
    pub entry_address: c_uint,
    pub fill_pattern: [c_uint; 2],
    pub reloadable_size: c_uint,
    pub sensitivity: c_uchar,
    pub reserved: c_uchar,
    pub ae_mode: c_ushort,
    pub max_ver: c_ushort,
    pub min_ver: c_ushort,
    pub image_attrib: c_ushort,
    pub reserved2: c_ushort,
    pub page_region_num: c_ushort,
    pub numpages: c_ushort,
    pub reg_tab_offset: c_uint,
    pub init_reg_sym_tab: c_uint,
    pub sbreak_tab: c_uint,
    pub app_metadata: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_uof_objtable {
    pub entry_num: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_uof_ae_reg {
    pub name: c_uint,
    pub vis_name: c_uint,
    pub type: c_ushort,
    pub addr: c_ushort,
    pub access_mode: c_ushort,
    pub visible: c_uchar,
    pub reserved1: c_uchar,
    pub ref_count: c_ushort,
    pub reserved2: c_ushort,
    pub xo_id: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_uof_code_area {
    pub micro_words_num: c_uint,
    pub uword_block_tab: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_uof_batch_init {
    pub ae: c_uint,
    pub addr: c_uint,
    pub value: *mut c_uint,
    pub size: c_uint,
    pub next: *mut icp_qat_uof_batch_init,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_suof_img_hdr {
    pub simg_buf: *mut c_char,
    pub simg_len: c_ulong,
    pub css_header: *mut c_char,
    pub css_simg: *mut c_char,
    pub simg_size: c_ulong,
    pub ae_num: c_uint,
    pub ae_mask: c_uint,
    pub fw_type: c_uint,
    pub simg_name: c_ulong,
    pub appmeta_data: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_suof_img_tbl {
    pub num_simgs: c_uint,
    pub simg_hdr: *mut icp_qat_suof_img_hdr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_suof_handle {
    pub file_id: c_uint,
    pub check_sum: c_uint,
    pub min_ver: c_char,
    pub maj_ver: c_char,
    pub fw_type: c_char,
    pub suof_buf: *mut c_char,
    pub suof_size: c_uint,
    pub sym_str: *mut c_char,
    pub sym_size: c_uint,
    pub img_table: icp_qat_suof_img_tbl,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_fw_auth_desc {
    pub img_len: c_uint,
    pub ae_mask: c_uint,
    pub css_hdr_high: c_uint,
    pub css_hdr_low: c_uint,
    pub img_high: c_uint,
    pub img_low: c_uint,
    pub signature_high: c_uint,
    pub signature_low: c_uint,
    pub fwsk_pub_high: c_uint,
    pub fwsk_pub_low: c_uint,
    pub img_ae_mode_data_high: c_uint,
    pub img_ae_mode_data_low: c_uint,
    pub img_ae_init_data_high: c_uint,
    pub img_ae_init_data_low: c_uint,
    pub img_ae_insts_high: c_uint,
    pub img_ae_insts_low: c_uint,
    pub cpp_mask: c_uint,
    pub reserved: c_uint,
    pub xmss_pubkey_high: c_uint,
    pub xmss_pubkey_low: c_uint,
    pub xmss_sig_high: c_uint,
    pub xmss_sig_low: c_uint,
    pub reserved2: [c_uint; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_auth_chunk {
    pub fw_auth_desc: icp_qat_fw_auth_desc,
    pub chunk_size: u64,
    pub chunk_bus_addr: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_css_hdr {
    pub module_type: c_uint,
    pub header_len: c_uint,
    pub header_ver: c_uint,
    pub module_id: c_uint,
    pub module_vendor: c_uint,
    pub date: c_uint,
    pub size: c_uint,
    pub key_size: c_uint,
    pub module_size: c_uint,
    pub exponent_size: c_uint,
    pub fw_type: c_uint,
    pub reserved: [c_uint; 21],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_simg_ae_mode {
    pub file_id: c_uint,
    pub maj_ver: c_ushort,
    pub min_ver: c_ushort,
    pub dev_type: c_uint,
    pub devmax_ver: c_ushort,
    pub devmin_ver: c_ushort,
    pub ae_mask: c_uint,
    pub ctx_enables: c_uint,
    pub fw_type: c_char,
    pub ctx_mode: c_char,
    pub nn_mode: c_char,
    pub lm0_mode: c_char,
    pub lm1_mode: c_char,
    pub scs_mode: c_char,
    pub lm2_mode: c_char,
    pub lm3_mode: c_char,
    pub tindex_mode: c_char,
    pub reserved: [c_uchar; 7],
    pub simg_name: [c_char; 256],
    pub appmeta_data: [c_char; 256],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_suof_filehdr {
    pub file_id: c_uint,
    pub check_sum: c_uint,
    pub min_ver: c_char,
    pub maj_ver: c_char,
    pub fw_type: c_char,
    pub reserved: c_char,
    pub max_chunks: c_ushort,
    pub num_chunks: c_ushort,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_suof_chunk_hdr {
    pub chunk_id: [c_char; ICP_QAT_SUOF_OBJ_ID_LEN],
    pub offset: u64,
    pub size: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_suof_strtable {
    pub tab_length: c_uint,
    pub strings: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_suof_objhdr {
    pub img_length: c_uint,
    pub reserved: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_mof_file_hdr {
    pub file_id: c_uint,
    pub checksum: c_uint,
    pub min_ver: c_char,
    pub maj_ver: c_char,
    pub reserved: c_ushort,
    pub max_chunks: c_ushort,
    pub num_chunks: c_ushort,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_mof_chunkhdr {
    pub chunk_id: [c_char; ICP_QAT_MOF_OBJ_ID_LEN],
    pub offset: u64,
    pub size: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_mof_str_table {
    pub tab_len: c_uint,
    pub strings: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_mof_obj_hdr {
    pub max_chunks: c_ushort,
    pub num_chunks: c_ushort,
    pub reserved: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_mof_obj_chunkhdr {
    pub chunk_id: [c_char; ICP_QAT_MOF_OBJ_CHUNKID_LEN],
    pub offset: u64,
    pub size: u64,
    pub name: c_uint,
    pub reserved: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_mof_objhdr {
    pub obj_name: *mut c_char,
    pub obj_buf: *mut c_char,
    pub obj_size: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_mof_table {
    pub num_objs: c_uint,
    pub obj_hdr: *mut icp_qat_mof_objhdr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_mof_handle {
    pub file_id: c_uint,
    pub checksum: c_uint,
    pub min_ver: c_char,
    pub maj_ver: c_char,
    pub mof_buf: *mut c_char,
    pub mof_size: u32,
    pub sym_str: *mut c_char,
    pub sym_size: c_uint,
    pub uobjs_hdr: *mut c_char,
    pub sobjs_hdr: *mut c_char,
    pub obj_table: icp_qat_mof_table,
}
