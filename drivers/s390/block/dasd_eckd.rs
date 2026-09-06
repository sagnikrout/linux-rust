//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/s390/block/dasd_eckd.h
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
// Author(s)......: Holger Smolinski <Holger.Smolinski@de.ibm.com>
// Horst Hummel <Horst.Hummel@de.ibm.com>
// Bugreports.to..: <Linux390@de.ibm.com>
// Copyright IBM Corp. 1999, 2000
//
// SECTION: CCW Definitions
//
pub const DASD_ECKD_CCW_NOP: c_uint = 0x03;
pub const DASD_ECKD_CCW_WRITE: c_uint = 0x05;
pub const DASD_ECKD_CCW_READ: c_uint = 0x06;
pub const DASD_ECKD_CCW_WRITE_HOME_ADDRESS: c_uint = 0x09;
pub const DASD_ECKD_CCW_READ_HOME_ADDRESS: c_uint = 0x0a;
pub const DASD_ECKD_CCW_WRITE_KD: c_uint = 0x0d;
pub const DASD_ECKD_CCW_READ_KD: c_uint = 0x0e;
pub const DASD_ECKD_CCW_ERASE: c_uint = 0x11;
pub const DASD_ECKD_CCW_READ_COUNT: c_uint = 0x12;
pub const DASD_ECKD_CCW_SLCK: c_uint = 0x14;
pub const DASD_ECKD_CCW_WRITE_RECORD_ZERO: c_uint = 0x15;
pub const DASD_ECKD_CCW_READ_RECORD_ZERO: c_uint = 0x16;
pub const DASD_ECKD_CCW_WRITE_CKD: c_uint = 0x1d;
pub const DASD_ECKD_CCW_READ_CKD: c_uint = 0x1e;
pub const DASD_ECKD_CCW_PSF: c_uint = 0x27;
pub const DASD_ECKD_CCW_SNID: c_uint = 0x34;
pub const DASD_ECKD_CCW_RSSD: c_uint = 0x3e;
pub const DASD_ECKD_CCW_LOCATE_RECORD: c_uint = 0x47;
pub const DASD_ECKD_CCW_LOCATE_RECORD_EXT: c_uint = 0x4b;
pub const DASD_ECKD_CCW_SNSS: c_uint = 0x54;
pub const DASD_ECKD_CCW_DEFINE_EXTENT: c_uint = 0x63;
pub const DASD_ECKD_CCW_WRITE_MT: c_uint = 0x85;
pub const DASD_ECKD_CCW_READ_MT: c_uint = 0x86;
pub const DASD_ECKD_CCW_WRITE_KD_MT: c_uint = 0x8d;
pub const DASD_ECKD_CCW_READ_KD_MT: c_uint = 0x8e;
pub const DASD_ECKD_CCW_READ_COUNT_MT: c_uint = 0x92;
pub const DASD_ECKD_CCW_RELEASE: c_uint = 0x94;
pub const DASD_ECKD_CCW_WRITE_FULL_TRACK: c_uint = 0x95;
pub const DASD_ECKD_CCW_READ_CKD_MT: c_uint = 0x9e;
pub const DASD_ECKD_CCW_WRITE_CKD_MT: c_uint = 0x9d;
pub const DASD_ECKD_CCW_WRITE_TRACK_DATA: c_uint = 0xA5;
pub const DASD_ECKD_CCW_READ_TRACK_DATA: c_uint = 0xA6;
pub const DASD_ECKD_CCW_RESERVE: c_uint = 0xB4;
pub const DASD_ECKD_CCW_READ_TRACK: c_uint = 0xDE;
pub const DASD_ECKD_CCW_PFX: c_uint = 0xE7;
pub const DASD_ECKD_CCW_PFX_READ: c_uint = 0xEA;
pub const DASD_ECKD_CCW_RSCK: c_uint = 0xF9;
pub const DASD_ECKD_CCW_RCD: c_uint = 0xFA;
pub const DASD_ECKD_CCW_DSO: c_uint = 0xF7;
// Define Subsystem Function / Orders
pub const DSO_ORDER_RAS: c_uint = 0x81;
//
// Perform Subsystem Function / Orders
//
pub const PSF_ORDER_PRSSD: c_uint = 0x18;
pub const PSF_ORDER_CUIR_RESPONSE: c_uint = 0x1A;
pub const PSF_ORDER_SSC: c_uint = 0x1D;
//
// Perform Subsystem Function / Sub-Orders
//
pub const PSF_SUBORDER_QHA: c_uint = 0x1C /* Query Host Access */;
pub const PSF_SUBORDER_PPRCEQ: c_uint = 0x50 /* PPRC Extended Query */;
pub const PSF_SUBORDER_VSQ: c_uint = 0x52 /* Volume Storage Query */;
pub const PSF_SUBORDER_LCQ: c_uint = 0x53 /* Logical Configuration Query */;
//
// PPRC Extended Query Scopes
//
pub const PPRCEQ_SCOPE_4: c_uint = 0x04 /* Scope 4 for PPRC Extended Query */;
//
// CUIR response condition codes
//
pub const PSF_CUIR_INVALID: c_uint = 0x00;
pub const PSF_CUIR_COMPLETED: c_uint = 0x01;
pub const PSF_CUIR_NOT_SUPPORTED: c_uint = 0x02;
pub const PSF_CUIR_ERROR_IN_REQ: c_uint = 0x03;
pub const PSF_CUIR_DENIED: c_uint = 0x04;
pub const PSF_CUIR_LAST_PATH: c_uint = 0x05;
pub const PSF_CUIR_DEVICE_ONLINE: c_uint = 0x06;
pub const PSF_CUIR_VARY_FAILURE: c_uint = 0x07;
pub const PSF_CUIR_SOFTWARE_FAILURE: c_uint = 0x08;
pub const PSF_CUIR_NOT_RECOGNIZED: c_uint = 0x09;
//
// CUIR codes
//
pub const CUIR_QUIESCE: c_uint = 0x01;
pub const CUIR_RESUME: c_uint = 0x02;
//
// Out-of-space (OOS) Codes
//
pub const REPO_WARN: c_uint = 0x01;
pub const REPO_EXHAUST: c_uint = 0x02;
pub const POOL_WARN: c_uint = 0x03;
pub const POOL_EXHAUST: c_uint = 0x04;
pub const REPO_RELIEVE: c_uint = 0x05;
pub const POOL_RELIEVE: c_uint = 0x06;
//
// attention message definitions
//
pub const ATTENTION_LENGTH_CUIR: c_uint = 0x0e;
pub const ATTENTION_FORMAT_CUIR: c_uint = 0x01;
pub const ATTENTION_LENGTH_OOS: c_uint = 0x10;
pub const ATTENTION_FORMAT_OOS: c_uint = 0x06;
pub const DASD_ECKD_PG_GROUPED: c_uint = 0x10;
//
// Size that is reported for large volumes in the old 16-bit no_cyl field
//
pub const LV_COMPAT_CYL: c_uint = 0xFFFE;
pub const FCX_MAX_DATA_FACTOR: c_int = 65536;
pub const DASD_ECKD_RCD_DATA_SIZE: c_int = 256;
pub const DASD_ECKD_PATH_THRHLD: c_int = 256;
pub const DASD_ECKD_PATH_INTERVAL: c_int = 300;
//
// Maximum number of blocks to be chained
//
pub const DASD_ECKD_MAX_BLOCKS: c_int = 180;
pub const DASD_ECKD_MAX_BLOCKS_RAW: c_int = 256;
//
// SECTION: Type Definitions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eckd_count {
    pub cyl: __u16,
    pub head: __u16,
    pub record: __u8,
    pub kl: __u8,
    pub dl: __u16,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eckd_r0 {
    pub count: eckd_count,
    pub data: [__u8; 8],
    pub __packed: },
//
// Extended Address Volume track address: the head field carries the actual
// head in its low-order 4 bits; the cylinder bits that do not fit the 16-bit
// cyl field are shifted in just above them.
//

//
// On-disk DASD format label.
//
// Written into track 0, head 0, record 4 (R4 - the first non-special CDL
// record) as part of the same channel program that formats track 0, so it is
// stored atomically with the track: either both the track format and the label
// make it to disk or neither does. Its presence with a valid magic therefore
// marks a completed format and can be used for format detection.
//
// The structure is exactly the smallest supported block size (512 bytes) so it
// always fits into a single record.
// For larger block sizes the rest of the record is zero padded.
// The magic together with the version is used to recognise a valid label.
//
pub const DASD_ESE_LABEL_MAGIC: c_uint = 0xC4C1E2C4C6D4E3F1ULL	/* EBCDIC "DASDFMT1" */;
pub const DASD_ESE_LABEL_VERSION: c_int = 1;
// dasd_format_label.flags
pub const DASD_ESE_LABEL_F_ESE: c_uint = 0x00000001	/* volume is extent space efficient */;
pub const DASD_ESE_LABEL_F_QUICK: c_uint = 0x00000002	/* quick (space released) format */;
pub const DASD_ESE_LABEL_F_FULL: c_uint = 0x00000004	/* full format */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dasd_format_label {
    pub /: *mut *mut __u64 magic; / DASD_ESE_LABEL_MAGIC,
    pub /: *mut *mut __u32 version; / DASD_ESE_LABEL_VERSION,
    pub /: *mut *mut *mut __u32 flags; / DASD_ESE_LABEL_F_,
    pub /: *mut *mut __u32 blksize; / block size the volume was formatted with,
    pub reserved0: __u32,
    pub /: *mut *mut __u64 format_tod; / TOD clock at format time,
    pub /: *mut *mut __u8 kernel_version[64]; / NUL terminated kernel release (uname -r),
    pub /: *mut *mut __u8 reserved[416]; / pad the struct to 512 bytes,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ch_t {
    pub cyl: __u16,
    pub head: __u16,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct chr_t {
    pub cyl: __u16,
    pub head: __u16,
    pub record: __u8,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DE_eckd_data {
    pub /: *mut *mut unsigned char perm:2; / Permissions on this extent,
    pub reserved:1: c_uchar,
    pub /: *mut *mut unsigned char seek:2; / Seek control,
    pub /: *mut *mut unsigned char auth:2; / Access authorization,
    pub /: *mut *mut unsigned char pci:1; / PCI Fetch mode,
// C attribute field omitted
    pub /: *mut *mut unsigned char mode:2; / Architecture mode,
    pub /: *mut *mut unsigned char ckd:1; / CKD Conversion,
    pub /: *mut *mut unsigned char operation:3; / Operation mode,
    pub /: *mut *mut unsigned char cfw:1; / Cache fast write,
    pub /: *mut *mut unsigned char dfw:1; / DASD fast write,
// C attribute field omitted
    pub /: *mut *mut __u16 blk_size; / Blocksize,
    pub fast_write_id: __u16,
    pub /: *mut *mut __u8 ga_additional; / Global Attributes Additional,
    pub /: *mut *mut __u8 ga_extended; / Global Attributes Extended,
    pub beg_ext: ch_t,
    pub end_ext: ch_t,
    pub /: *mut *mut unsigned long ep_sys_time; / Ext Parameter - System Time Stamp,
    pub /: *mut *mut __u8 ep_format; / Extended Parameter format byte,
    pub /: *mut *mut __u8 ep_prio; / Extended Parameter priority I/O byte,
    pub /: *mut *mut __u8 ep_reserved1; / Extended Parameter Reserved,
    pub /: *mut *mut __u8 ep_rec_per_track; / Number of records on a track,
    pub /: *mut *mut __u8 ep_reserved[4]; / Extended Parameter Reserved,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct LO_eckd_data {
    pub orientation:2: c_uchar,
    pub operation:6: c_uchar,
// C attribute field omitted
    pub last_bytes_used:1: c_uchar,
    pub reserved:6: c_uchar,
    pub read_count_suffix:1: c_uchar,
// C attribute field omitted
    pub unused: __u8,
    pub count: __u8,
    pub seek_addr: ch_t,
    pub search_arg: chr_t,
    pub sector: __u8,
    pub length: __u16,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct LRE_eckd_data {
    pub orientation:2: c_uchar,
    pub operation:6: c_uchar,
// C attribute field omitted
    pub length_valid:1: c_uchar,
    pub length_scope:1: c_uchar,
    pub imbedded_ccw_valid:1: c_uchar,
    pub check_bytes:2: c_uchar,
    pub imbedded_count_valid:1: c_uchar,
    pub reserved:1: c_uchar,
    pub read_count_suffix:1: c_uchar,
// C attribute field omitted
    pub imbedded_ccw: __u8,
    pub count: __u8,
    pub seek_addr: ch_t,
    pub search_arg: chr_t,
    pub sector: __u8,
    pub length: __u16,
    pub imbedded_count: __u8,
    pub extended_operation: __u8,
    pub extended_parameter_length: __u16,
    pub extended_parameter: [__u8; ],
// C attribute field omitted
// Prefix data for format 0x00 and 0x01
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PFX_eckd_data {
    pub format: c_uchar,
    pub define_extent:1: c_uchar,
    pub time_stamp:1: c_uchar,
    pub verify_base:1: c_uchar,
    pub hyper_pav:1: c_uchar,
    pub reserved:4: c_uchar,
// C attribute field omitted
    pub base_address: __u8,
    pub aux: __u8,
    pub base_lss: __u8,
    pub reserved: [__u8; 7],
    pub define_extent: DE_eckd_data,
    pub locate_record: LRE_eckd_data,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dasd_eckd_characteristics {
    pub cu_type: __u16,
    pub support:2: c_uchar,
    pub async:1: c_uchar,
    pub reserved:1: c_uchar,
    pub cache_info:1: c_uchar,
    pub model:3: c_uchar,
// C attribute field omitted
    pub dev_type: __u16,
    pub dev_model: __u8,
    pub mult_burst:1: c_uchar,
    pub RT_in_LR:1: c_uchar,
    pub reserved1:1: c_uchar,
    pub RD_IN_LR:1: c_uchar,
    pub reserved2:4: c_uchar,
    pub reserved3:8: c_uchar,
    pub defect_wr:1: c_uchar,
    pub XRC_supported:1: c_uchar,
    pub PPRC_enabled:1: c_uchar,
    pub striping:1: c_uchar,
    pub reserved5:4: c_uchar,
    pub cfw:1: c_uchar,
    pub reserved6:2: c_uchar,
    pub cache:1: c_uchar,
    pub dual_copy:1: c_uchar,
    pub dfw:1: c_uchar,
    pub reset_alleg:1: c_uchar,
    pub sense_down:1: c_uchar,
// C attribute field omitted
    pub dev_class: __u8,
    pub unit_type: __u8,
    pub no_cyl: __u16,
    pub trk_per_cyl: __u16,
    pub sec_per_trk: __u8,
    pub byte_per_track: [__u8; 3],
    pub home_bytes: __u16,
    pub formula: __u8,
    pub f1: __u8,
    pub f2: __u16,
    pub f3: __u16,
// C attribute field omitted
    pub f1: __u8,
    pub f2: __u8,
    pub f3: __u8,
    pub f4: __u8,
    pub f5: __u8,
// C attribute field omitted
// C attribute field omitted
    pub first_alt_trk: __u16,
    pub no_alt_trk: __u16,
    pub first_dia_trk: __u16,
    pub no_dia_trk: __u16,
    pub first_sup_trk: __u16,
    pub no_sup_trk: __u16,
    pub MDR_ID: __u8,
    pub OBR_ID: __u8,
    pub director: __u8,
    pub rd_trk_set: __u8,
    pub max_rec_zero: __u16,
    pub reserved1: __u8,
    pub RWANY_in_LR: __u8,
    pub factor6: __u8,
    pub factor7: __u8,
    pub factor8: __u8,
    pub reserved2: [__u8; 3],
    pub reserved3: [__u8; 6],
    pub long_no_cyl: __u32,
// C attribute field omitted
// elements of the configuration data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dasd_ned {
    pub identifier:2: __u8,
    pub token_id:1: __u8,
    pub sno_valid:1: __u8,
    pub subst_sno:1: __u8,
    pub recNED:1: __u8,
    pub emuNED:1: __u8,
    pub reserved:1: __u8,
// C attribute field omitted
    pub descriptor: __u8,
    pub dev_class: __u8,
    pub reserved: __u8,
    pub dev_type: [__u8; 6],
    pub dev_model: [__u8; 3],
    pub HDA_manufacturer: [__u8; 3],
    pub HDA_location: [__u8; 2],
    pub HDA_seqno: [__u8; 12],
    pub serial: },
    pub ID: __u8,
    pub unit_addr: __u8,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dasd_sneq {
    pub identifier:2: __u8,
    pub reserved:6: __u8,
// C attribute field omitted
    pub res1: __u8,
    pub format: __u16,
    pub /: *mut *mut __u8 res2[4]; / byte 4- 7,
    pub /: *mut *mut __u8 sua_flags; / byte 8,
    pub /: *mut *mut __u8 base_unit_addr; / byte 9,
    pub /: *mut *mut __u8 res3[22]; / byte 10-31,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vd_sneq {
    pub identifier:2: __u8,
    pub reserved:6: __u8,
// C attribute field omitted
    pub res1: __u8,
    pub format: __u16,
    pub /: *mut *mut __u8 res2[4]; / byte 4- 7,
    pub /: *mut *mut __u8 uit[16]; / byte 8-23,
    pub /: *mut *mut __u8 res3[8]; / byte 24-31,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dasd_gneq {
    pub identifier:2: __u8,
    pub reserved:6: __u8,
// C attribute field omitted
    pub record_selector: __u8,
    pub reserved: [__u8; 4],
    pub value:2: __u8,
    pub number:6: __u8,
// C attribute field omitted
    pub reserved3: __u8,
    pub subsystemID: __u16,
    pub reserved2: [__u8; 22],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dasd_rssd_features {
    pub feature: [c_char; 256],
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dasd_rssd_messages {
    pub length: __u16,
    pub format: __u8,
    pub code: __u8,
    pub message_id: __u32,
    pub flags: __u8,
    pub messages: [c_char; 4087],
    pub __packed: },
//
// Read Subsystem Data - Volume Storage Query
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dasd_rssd_vsq {
    pub tse:1: __u8,
    pub space_not_available:1: __u8,
    pub ese:1: __u8,
    pub unused:5: __u8,
    pub vol_info: } __packed,
    pub unused1: __u8,
    pub extent_pool_id: __u16,
    pub warn_cap_limit: __u8,
    pub warn_cap_guaranteed: __u8,
    pub unused2: __u16,
    pub limit_capacity: __u32,
    pub guaranteed_capacity: __u32,
    pub space_allocated: __u32,
    pub space_configured: __u32,
    pub logical_capacity: __u32,
    pub __packed: },
//
// Extent Pool Summary
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dasd_ext_pool_sum {
    pub pool_id: __u16,
    pub repo_warn_thrshld: __u8,
    pub warn_thrshld: __u8,
    pub /: *mut *mut __u8 type:1; / 0 - CKD / 1 - FB,
    pub track_space_efficient:1: __u8,
    pub extent_space_efficient:1: __u8,
    pub standard_volume:1: __u8,
    pub extent_size_valid:1: __u8,
    pub capacity_at_warnlevel:1: __u8,
    pub pool_oos:1: __u8,
    pub unused0:1: __u8,
    pub unused1: __u8,
    pub flags: } __packed,
    pub reserved0:1: __u8,
    pub size_1G:1: __u8,
    pub reserved1:5: __u8,
    pub size_16M:1: __u8,
    pub extent_size: } __packed,
    pub unused: __u8,
    pub __packed: },
//
// Read Subsystem Data-Response - Logical Configuration Query - Header
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dasd_rssd_lcq {
    pub /: *mut *mut __u16 data_length; / Length of data returned,
    pub /: *mut *mut __u16 pool_count; / Count of extent pools returned - Max: 448,
    pub /: *mut *mut __u8 pool_info_valid:1; / Detailed Information valid,
    pub pool_id_volume:1: __u8,
    pub pool_id_cec:1: __u8,
    pub unused0:5: __u8,
    pub unused1: __u8,
    pub header_flags: } __packed,
    pub /: *mut *mut char sfi_type[6]; / Storage Facility Image Type (EBCDIC),
    pub /: *mut *mut char sfi_model[3]; / Storage Facility Image Model (EBCDIC),
    pub /: *mut *mut __u8 sfi_seq_num[10]; / Storage Facility Image Sequence Number,
    pub reserved: [__u8; 7],
    pub ext_pool_sum: [dasd_ext_pool_sum; 448],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dasd_oos_message {
    pub length: __u16,
    pub format: __u8,
    pub code: __u8,
    pub percentage_empty: __u8,
    pub reserved: __u8,
    pub ext_pool_id: __u16,
    pub token: __u16,
    pub unused: [__u8; 6],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dasd_cuir_message {
    pub length: __u16,
    pub format: __u8,
    pub code: __u8,
    pub message_id: __u32,
    pub flags: __u8,
    pub neq_map: [__u8; 3],
    pub ned_map: __u8,
    pub record_selector: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dasd_psf_cuir_response {
    pub order: __u8,
    pub flags: __u8,
    pub cc: __u8,
    pub chpid: __u8,
    pub device_nr: __u16,
    pub reserved: __u16,
    pub message_id: __u32,
    pub system_id: __u64,
    pub cssid: __u8,
    pub ssid: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dasd_ckd_path_group_entry {
    pub status_flags: __u8,
    pub pgid: [__u8; 11],
    pub sysplex_name: [__u8; 8],
    pub timestamp: __u32,
    pub cylinder: __u32,
    pub reserved: [__u8; 4],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dasd_ckd_host_information {
    pub access_flags: __u8,
    pub entry_size: __u8,
    pub entry_count: __u16,
    pub entry: [__u8; 16390],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dasd_psf_query_host_access {
    pub access_flag: __u8,
    pub version: __u8,
    pub CKD_length: __u16,
    pub SCSI_length: __u16,
    pub unused: [__u8; 10],
    pub host_access_information: [__u8; 16394],
    pub __packed: },
//
// Perform Subsystem Function - Prepare for Read Subsystem Data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dasd_psf_prssd_data {
    pub order: c_uchar,
    pub flags: c_uchar,
    pub reserved1: c_uchar,
    pub reserved2: c_uchar,
    pub lss: c_uchar,
    pub volume: c_uchar,
    pub suborder: c_uchar,
    pub varies: [c_uchar; 5],
// C attribute field omitted
//
// Perform Subsystem Function - Set Subsystem Characteristics
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dasd_psf_ssc_data {
    pub order: c_uchar,
    pub flags: c_uchar,
    pub cu_type: [c_uchar; 4],
    pub suborder: c_uchar,
    pub reserved: [c_uchar; 59],
    pub __attribute__((packed)): },
// Maximum number of extents for a single Release Allocated Space command

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dasd_dso_ras_ext_range {
    pub beg_ext: ch_t,
    pub end_ext: ch_t,
    pub __packed: },
//
// Define Subsystem Operation - Release Allocated Space
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dasd_dso_ras_data {
    pub order: __u8,
    pub /: *mut *mut __u8 message:1; / Must be zero,
    pub reserved1:2: __u8,
    pub /: *mut *mut __u8 vol_type:1; / 0 - CKD/FBA, 1 - FB,
    pub reserved2:4: __u8,
    pub flags: } __packed,
// Operation Flags to specify scope
    pub reserved1:2: __u8,
// Release Space by Extent
    pub /: *mut *mut __u8 by_extent:1; / 0 - entire volume, 1 - specified extents,
    pub guarantee_init:1: __u8,
    pub /: *mut *mut __u8 force_release:1; / Internal - will be ignored,
    pub reserved2:11: __u16,
    pub op_flags: } __packed,
    pub lss: __u8,
    pub dev_addr: __u8,
    pub reserved1: __u32,
    pub reserved2: [__u8; 10],
    pub /: *mut *mut __u16 nr_exts; / Defines number of ext_scope - max 110,
    pub reserved3: __u16,
    pub __packed: },
//
// some structures and definitions for alias handling
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dasd_unit_address_configuration {
    pub ua_type: c_char,
    pub base_ua: c_char,
    pub unit: [}; 256],
    pub __attribute__((packed)): },
pub const MAX_DEVICES_PER_LCU: c_int = 256;
// flags on the LCU
pub const NEED_UAC_UPDATE: c_uint = 0x01;
pub const UPDATE_PENDING: c_uint = 0x02;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pavtype {


    struct alias_root {
    struct list_head serverlist;
    spinlock_t lock;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct alias_server {
    pub server: list_head,
    pub uid: dasd_uid,
    pub lculist: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct summary_unit_check_work_data {
    pub reason: c_char,
    pub device: *mut dasd_device,
    pub worker: work_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct read_uac_work_data {
    pub device: *mut dasd_device,
    pub dwork: delayed_work,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct alias_lcu {
    pub lcu: list_head,
    pub uid: dasd_uid,
    pub pav: pavtype,
    pub flags: c_char,
    pub lock: spinlock_t,
    pub grouplist: list_head,
    pub active_devices: list_head,
    pub inactive_devices: list_head,
    pub uac: *mut dasd_unit_address_configuration,
    pub suc_data: summary_unit_check_work_data,
    pub ruac_data: read_uac_work_data,
    pub rsu_cqr: *mut dasd_ccw_req,
    pub lcu_setup: completion,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct alias_pav_group {
    pub group: list_head,
    pub uid: dasd_uid,
    pub lcu: *mut alias_lcu,
    pub baselist: list_head,
    pub aliaslist: list_head,
    pub next: *mut dasd_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dasd_conf_data {
    pub neds: [dasd_ned; 5],
    pub reserved: [u8; 64],
    pub gneq: dasd_gneq,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dasd_conf {
    pub data: *mut u8,
    pub len: c_int,
// pointers to specific parts in the conf_data
    pub ned: *mut dasd_ned,
    pub sneq: *mut dasd_sneq,
    pub vdsneq: *mut vd_sneq,
    pub gneq: *mut dasd_gneq,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dasd_eckd_private {
    pub rdc_data: dasd_eckd_characteristics,
    pub conf: dasd_conf,
    pub count_area: [eckd_count; 5],
    pub init_cqr_status: c_int,
    pub uses_cdl: c_int,
    pub /: *mut *mut attrib_data_t attrib; / e.g. cache operations,
    pub features: dasd_rssd_features,
    pub vsq: dasd_rssd_vsq,
    pub eps: dasd_ext_pool_sum,
    pub real_cyl: u32,
// alias management
    pub uid: dasd_uid,
//
// Cached copies of conf.ned->ID (the LSS) and conf.ned->unit_addr,
// refreshed under ccwdev_lock. Kept outside uid because create_uid()
// memsets uid before repopulating it, which would expose a transient
// zero to the lockless CCW-build readers.
//
    pub ned_lss: __u8,
    pub ned_ua: __u8,
    pub pavgroup: *mut alias_pav_group,
    pub lcu: *mut alias_lcu,
    pub count: c_int,
    pub fcx_max_data: u32,
    pub suc_reason: c_char,
//
// Set when the whole volume's space was released (full RAS); consumed by
// the next format to mark the on-disk label as a quick (vs full) format.
//
    pub ese_format_quick: c_int,
//
// Cached on-disk format label (R4), read at online and refreshed on
// format. When valid, is_ese() is derived from it; otherwise it falls
// back to the hardware ESE field (vsq.vol_info.ese).
//
    pub ese_label: dasd_format_label,
    pub ese_label_valid: bool,
}

extern "C" {
    pub fn dasd_alias_make_device_known_to_lcu(: *mut dasd_device) -> c_int;
}
extern "C" {
    pub fn dasd_alias_disconnect_device_from_lcu(: *mut dasd_device);
}
extern "C" {
    pub fn dasd_alias_add_device(: *mut dasd_device) -> c_int;
}
extern "C" {
    pub fn dasd_alias_remove_device(: *mut dasd_device) -> c_int;
}
extern "C" {
    pub fn dasd_alias_handle_summary_unit_check(: *mut work_struct);
}
extern "C" {
    pub fn dasd_eckd_reset_ccw_to_base_io(: *mut dasd_ccw_req);
}
extern "C" {
    pub fn dasd_alias_update_add_device(: *mut dasd_device) -> c_int;
}
