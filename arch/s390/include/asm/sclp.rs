//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/sclp.h
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
// Copyright IBM Corp. 2007
//

pub const SCLP_CHP_INFO_MASK_SIZE: c_int = 32;

pub const SCLP_MAX_CORES: c_int = 512;
// 144 + 16 * SCLP_MAX_CORES + 2 * (SCLP_MAX_CORES - 1)

// 24 + 16 * SCLP_MAX_CORES

pub const SCLP_ERRNOTIFY_AQ_RESET: c_int = 0;
pub const SCLP_ERRNOTIFY_AQ_REPAIR: c_int = 1;
pub const SCLP_ERRNOTIFY_AQ_INFO_LOG: c_int = 2;
pub const SCLP_ERRNOTIFY_AQ_OPTICS_DATA: c_int = 3;
pub const SCLP_ERRNOTIFY_AQ_NVME_SMART_LOG: c_int = 4;
pub const SCLP_ERRNOTIFY_AQ_ADAPTER_INITIALIZED: c_int = 5;
pub const SCLP_ERRNOTIFY_AQ_RECOVERABLE_ERROR: c_int = 6;
pub const SCLP_ERRNOTIFY_AQ_TELEMETRY_DATA: c_int = 7;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sclp_chp_info {
    pub recognized: [u8; SCLP_CHP_INFO_MASK_SIZE],
    pub standby: [u8; SCLP_CHP_INFO_MASK_SIZE],
    pub configured: [u8; SCLP_CHP_INFO_MASK_SIZE],
}

pub const LOADPARM_LEN: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sclp_ipl_info {
    pub is_valid: c_int,
    pub has_dump: c_int,
    pub loadparm: [c_char; LOADPARM_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sclp_core_entry {
    pub core_id: u8,
    pub reserved0: u8,
    pub 4: u8 :,
    pub 1: u8 sief2 :,
    pub 1: u8 skey :,
    pub 2: u8 :,
    pub 2: u8 :,
    pub 1: u8 gpere :,
    pub 1: u8 siif :,
    pub 1: u8 sigpif :,
    pub 3: u8 :,
    pub reserved2: [u8; 3],
    pub 2: u8 :,
    pub 1: u8 ib :,
    pub 1: u8 cei :,
    pub 4: u8 :,
    pub reserved3: [u8; 6],
    pub type: u8,
    pub reserved1: u8,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sclp_core_info {
    pub configured: c_uint,
    pub standby: c_uint,
    pub combined: c_uint,
    pub core: [sclp_core_entry; SCLP_MAX_CORES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sclp_info {
    pub 1: unsigned char has_linemode :,
    pub 1: unsigned char has_vt220 :,
    pub 1: unsigned char has_siif :,
    pub 1: unsigned char has_sigpif :,
    pub 1: unsigned char has_core_type :,
    pub 1: unsigned char has_sprp :,
    pub 1: unsigned char has_hvs :,
    pub 1: unsigned char has_wti :,
    pub 1: unsigned char has_esca :,
    pub 1: unsigned char has_sief2 :,
    pub 1: unsigned char has_64bscao :,
    pub 1: unsigned char has_gpere :,
    pub 1: unsigned char has_cmma :,
    pub 1: unsigned char has_gsls :,
    pub 1: unsigned char has_ib :,
    pub 1: unsigned char has_cei :,
    pub 1: unsigned char has_pfmfi :,
    pub 1: unsigned char has_ibs :,
    pub 1: unsigned char has_skey :,
    pub 1: unsigned char has_kss :,
    pub 1: unsigned char has_diag204_bif :,
    pub 1: unsigned char has_gisaf :,
    pub 1: unsigned char has_diag310 :,
    pub 1: unsigned char has_diag318 :,
    pub 1: unsigned char has_diag320 :,
    pub 1: unsigned char has_diag324 :,
    pub 1: unsigned char has_sipl :,
    pub 1: unsigned char has_sipl_eckd :,
    pub 1: unsigned char has_dirq :,
    pub 1: unsigned char has_iplcc :,
    pub 1: unsigned char has_zpci_lsi :,
    pub 1: unsigned char has_aisii :,
    pub 1: unsigned char has_aeni :,
    pub 1: unsigned char has_aisi :,
    pub 1: unsigned char has_astfleie2 :,
    pub ibc: c_uint,
    pub mtid: c_uint,
    pub mtid_cp: c_uint,
    pub mtid_prev: c_uint,
    pub rzm: c_ulong,
    pub rnmax: c_ulong,
    pub hamax: c_ulong,
    pub max_cores: c_uint,
    pub hsa_size: c_ulong,
    pub facilities: c_ulong,
    pub hmfai: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sccb_header {
    pub length: u16,
    pub function_code: u8,
    pub control_mask: [u8; 3],
    pub response_code: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct evbuf_header {
    pub length: u16,
    pub type: u8,
    pub flags: u8,
    pub _reserved: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct err_notify_evbuf {
    pub header: evbuf_header,
    pub action: u8,
    pub atype: u8,
    pub fh: u32,
    pub fid: u32,
    pub data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct err_notify_sccb {
    pub header: sccb_header,
    pub evbuf: err_notify_evbuf,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zpci_report_error_header {
    pub /: *mut *mut u8 version; / Interface version byte,
    pub byte: *mut *mut u8 action; / Action qualifier,
// 0: Adapter Reset Request
// 1: Deconfigure and repair action requested
// (OpenCrypto Problem Call Home)
// 2: Informational Report
// (OpenCrypto Successful Diagnostics Execution)
//
    pub /: *mut *mut u16 length; / Length of Subsequent Data (up to 4K – SCLP header,
    pub /: *mut *mut u8 data[]; / Subsequent Data passed verbatim to SCLP ET 24,
    pub __packed: },
    pub sclp_early_sccb: *mut extern char,
    pub sclp_early_adjust_va(void): c_void,
    pub sccb): *mut void sclp_early_set_buffer(void,
    pub sclp_early_read_info(void): c_int,
    pub sclp_early_read_storage_info(void): c_int,
    pub info): *mut int sclp_early_get_core_info(struct sclp_core_info,
    pub info): *mut void sclp_early_get_ipl_info(struct sclp_ipl_info,
    pub sclp_early_detect(void): c_void,
    pub sclp_early_detect_machine_features(void): c_void,
    pub s): *const void sclp_early_printk(char,
    pub len): *const *const void __sclp_early_printk(char s, unsigned int,
    pub s): *const void sclp_emergency_printk(char,
    pub sclp_init(void): c_int,
    pub mem): *mut int sclp_early_get_memsize(unsigned long,
    pub hsa_size): *mut int sclp_early_get_hsa_size(unsigned long,
    pub info): *mut int _sclp_get_core_info(struct sclp_core_info,
    pub core): int sclp_core_configure(u8,
    pub core): int sclp_core_deconfigure(u8,
    pub sclp_sdias_blk_count(void): c_int,
    pub nr_blks): *mut *mut int sclp_sdias_copy(void dest, int blk_num, int,
    pub chpid): int sclp_chp_configure(struct chp_id,
    pub chpid): int sclp_chp_deconfigure(struct chp_id,
    pub info): *mut int sclp_chp_read_info(struct sclp_chp_info,
    pub fid): int sclp_pci_configure(u32,
    pub fid): int sclp_pci_deconfigure(u32,
    pub apid): int sclp_ap_configure(u32,
    pub apid): int sclp_ap_deconfigure(u32,
    pub fid): *mut *mut int sclp_pci_report(struct zpci_report_error_header report, u32 fh, u32,
    pub count): *mut *mut size_t memcpy_hsa_iter(struct iov_iter iter, unsigned long src, size_t,
    pub dst): *mut void sclp_ocf_cpc_name_copy(char,
    pub sclp_early_get_core_info(info): return,
    pub _sclp_get_core_info(info): return,

