//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/s390/cio/chsc.h
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

pub const CHSC_SDA_OC_MSS: c_uint = 0x2;
pub const NR_MEASUREMENT_CHARS: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmg_chars {
    pub values: [u32; NR_MEASUREMENT_CHARS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmg_cmcb {
    pub 1: u32 not_valid :,
    pub 1: u32 shared :,
    pub 1: u32 extended :,
    pub 21: u32 :,
    pub 8: u32 chpid :,
    pub 5: u32 cmcv :,
    pub 7: u32 :,
    pub 4: u32 cmgp :,
    pub 8: u32 cmgq :,
    pub 8: u32 cmg :,
    pub 16: u32 :,
    pub 16: u32 cmgs :,
    pub data: [u32; NR_MEASUREMENT_CHARS],
}

pub const NR_MEASUREMENT_ENTRIES: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmg_entry {
    pub values: [u32; NR_MEASUREMENT_ENTRIES],
}

pub const NR_EXT_MEASUREMENT_ENTRIES: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmg_ext_entry {
    pub values: [u32; NR_EXT_MEASUREMENT_ENTRIES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct channel_path_desc_fmt1 {
    pub flags: u8,
    pub lsn: u8,
    pub desc: u8,
    pub chpid: u8,
    pub esc: u8,
    pub chpp: u8,
    pub unused: [u32; 2],
    pub chid: u16,
    pub mdc: u16,
    pub r:1: u8,
    pub s:1: u8,
    pub f:1: u8,
    pub zeros: [u32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct channel_path_desc_fmt3 {
    pub fmt1_desc: channel_path_desc_fmt1,
    pub util_str: [u8; 64],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct css_chsc_char {
    pub res: u64,
    pub 20: u64 :,
    pub /: *mut *mut u32 secm : 1; / bit 84,
    pub 1: u32 :,
    pub /: *mut *mut u32 scmc : 1; / bit 86,
    pub 20: u32 :,
    pub /: *mut *mut u32 scssc : 1; / bit 107,
    pub /: *mut *mut u32 scsscf : 1; / bit 108,
    pub /: *mut *mut u32 pnso:1; / bit 116,
    pub __packed: },
    pub css_chsc_characteristics: extern struct css_chsc_char,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct chsc_ssd_info {
    pub path_mask: u8,
    pub fla_valid_mask: u8,
    pub chpid: [chp_id; 8],
    pub fla: [u16; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct chsc_ssqd_area {
    pub request: chsc_header,
    pub ssid:2: u8,
    pub fmt:4: u8,
    pub first_sch: u16,
    pub last_sch: u16,
    pub response: chsc_header,
    pub qdio_ssqd: qdio_ssqd_desc,
    pub __aligned(PAGE_SIZE): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct chsc_scssc_area {
    pub request: chsc_header,
    pub operation_code: u16,
    pub summary_indicator_addr: dma64_t,
    pub subchannel_indicator_addr: dma64_t,
    pub ks:4: u32,
    pub kc:4: u32,
    pub isc:3: u32,
    pub word_with_d_bit: u32,
    pub schid: subchannel_id,
    pub reserved: [u32; 1004],
    pub response: chsc_header,
    pub __aligned(PAGE_SIZE): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct chsc_scpd {
    pub request: chsc_header,
    pub m:1: u32,
    pub c:1: u32,
    pub fmt:4: u32,
    pub cssid:8: u32,
    pub rfmt:4: u32,
    pub first_chpid:8: u32,
    pub last_chpid:8: u32,
    pub zeroes1: u32,
    pub response: chsc_header,
    pub data: [u8; ],
    pub __aligned(PAGE_SIZE): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct chsc_sda_area {
    pub request: chsc_header,
    pub :4: u8,
    pub format:4: u8,
    pub :8: u8,
    pub operation_code: u16,
    pub :32: u32,
    pub :32: u32,
    pub operation_data_area: [u32; 252],
    pub response: chsc_header,
    pub :4: u32,
    pub format2:4: u32,
    pub :24: u32,
    pub __aligned(PAGE_SIZE): } __packed,
    pub ssd): *mut chsc_ssd_info,
    pub chsc_determine_css_characteristics(void): extern int,
    pub chsc_init(void): extern int,
    pub chsc_init_cleanup(void): extern void,
    pub operation_code): *mut *mut int __chsc_enable_facility(struct chsc_sda_area sda_area, int,
    pub chsc_enable_facility(int): extern int,
    pub channel_subsystem: struct,
    pub int): *mut *mut extern int chsc_secm(struct channel_subsystem ,,
    pub enable): *mut *mut int __chsc_do_secm(struct channel_subsystem css, int,
    pub on): int chsc_chp_vary(struct chp_id chpid, int,
    pub page): *mut int c, int m, void,
    pub desc): *mut channel_path_desc_fmt0,
    pub desc): *mut channel_path_desc_fmt1,
    pub desc): *mut channel_path_desc_fmt3,
    pub chpid): void chsc_chp_online(struct chp_id,
    pub chpid): void chsc_chp_offline(struct chp_id,
    pub chp): *mut int chsc_get_channel_measurement_chars(struct channel_path,
    pub ssqd): *mut int chsc_ssqd(struct subchannel_id schid, struct chsc_ssqd_area,
    pub isc): u8,
    pub origin): int chsc_sgib(u32,
    pub response): int chsc_error_from_response(int,
    pub schid): int chsc_siosl(struct subchannel_id,
// Functions and definitions to query storage-class memory.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sale {
    pub sa: u64,
    pub p:4: u32,
    pub op_state:4: u32,
    pub data_state:4: u32,
    pub rank:4: u32,
    pub r:1: u32,
    pub rid:8: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct chsc_scm_info {
    pub request: chsc_header,
    pub reqtok: u64,
    pub reserved1: [u32; 4],
    pub response: chsc_header,
    pub rq: u8,
    pub mbc: u32,
    pub msa: u64,
    pub is: u16,
    pub mmc: u16,
    pub mci: u32,
    pub nr_scm_ini: u64,
    pub nr_scm_unini: u64,
    pub reserved2: [u32; 10],
    pub restok: u64,
    pub scmal: [sale; 248],
    pub __aligned(PAGE_SIZE): } __packed,
    pub token): *mut *mut int chsc_scm_info(struct chsc_scm_info scm_area, u64,
    pub cnc): u8 oc, struct chsc_pnso_resume_token resume_token, int,
    pub iid): *mut *mut int __init chsc_get_cssid_iid(int idx, u8 cssid, u8,

    pub scm_update_information(void): c_int,
    pub scm_process_availability_information(void): c_int,

    pub }: static inline int scm_update_information(void) { return 0;,
    pub }: static inline int scm_process_availability_information(void) { return 0;,

