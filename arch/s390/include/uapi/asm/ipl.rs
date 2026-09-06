//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/uapi/asm/ipl.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note

// IPL Parameter List header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipl_pl_hdr {
    pub len: __u32,
    pub flags: __u8,
    pub reserved1: [__u8; 2],
    pub version: __u8,
    pub __packed: },
pub const IPL_PL_FLAG_IPLPS: c_uint = 0x80;
pub const IPL_PL_FLAG_SIPL: c_uint = 0x40;
pub const IPL_PL_FLAG_IPLSR: c_uint = 0x20;
pub const IPL_PL_FLAG_SBP: c_uint = 0x10;
// IPL Parameter Block header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipl_pb_hdr {
    pub len: __u32,
    pub pbt: __u8,
    pub __packed: },
// IPL Parameter Block types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipl_pbt {
    IPL_PBT_FCP = 0,
    IPL_PBT_SCP_DATA = 1,
    IPL_PBT_CCW = 2,
    IPL_PBT_ECKD = 3,
    IPL_PBT_NVME = 4,
}

// IPL Parameter Block 0 with common fields
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipl_pb0_common {
    pub len: __u32,
    pub pbt: __u8,
    pub flags: __u8,
    pub reserved1: [__u8; 2],
    pub loadparm: [__u8; 8],
    pub reserved2: [__u8; 84],
    pub __packed: },
pub const IPL_PB0_FLAG_LOADPARM: c_uint = 0x80;
// IPL Parameter Block 0 for FCP
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipl_pb0_fcp {
    pub len: __u32,
    pub pbt: __u8,
    pub reserved1: [__u8; 3],
    pub loadparm: [__u8; 8],
    pub reserved2: [__u8; 304],
    pub opt: __u8,
    pub reserved3: [__u8; 3],
    pub cssid: __u8,
    pub reserved4: [__u8; 1],
    pub devno: __u16,
    pub reserved5: [__u8; 4],
    pub wwpn: __u64,
    pub lun: __u64,
    pub bootprog: __u32,
    pub reserved6: [__u8; 12],
    pub br_lba: __u64,
    pub scp_data_len: __u32,
    pub reserved7: [__u8; 260],
    pub scp_data: [__u8; ],
    pub __packed: },
pub const IPL_PB0_FCP_OPT_IPL: c_uint = 0x10;
pub const IPL_PB0_FCP_OPT_DUMP: c_uint = 0x20;
// IPL Parameter Block 0 for NVMe
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipl_pb0_nvme {
    pub len: __u32,
    pub pbt: __u8,
    pub reserved1: [__u8; 3],
    pub loadparm: [__u8; 8],
    pub reserved2: [__u8; 304],
    pub opt: __u8,
    pub reserved3: [__u8; 3],
    pub fid: __u32,
    pub reserved4: [__u8; 12],
    pub nsid: __u32,
    pub reserved5: [__u8; 4],
    pub bootprog: __u32,
    pub reserved6: [__u8; 12],
    pub br_lba: __u64,
    pub scp_data_len: __u32,
    pub reserved7: [__u8; 260],
    pub scp_data: [__u8; ],
    pub __packed: },
pub const IPL_PB0_NVME_OPT_IPL: c_uint = 0x10;
pub const IPL_PB0_NVME_OPT_DUMP: c_uint = 0x20;
// IPL Parameter Block 0 for CCW
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipl_pb0_ccw {
    pub len: __u32,
    pub pbt: __u8,
    pub flags: __u8,
    pub reserved1: [__u8; 2],
    pub loadparm: [__u8; 8],
    pub reserved2: [__u8; 84],
    pub 13: __u16 reserved3 :,
    pub 3: __u8 ssid :,
    pub devno: __u16,
    pub vm_flags: __u8,
    pub reserved4: [__u8; 3],
    pub vm_parm_len: __u32,
    pub nss_name: [__u8; 8],
    pub vm_parm: [__u8; 64],
    pub reserved5: [__u8; 8],
    pub __packed: },
// IPL Parameter Block 0 for ECKD
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipl_pb0_eckd {
    pub len: __u32,
    pub pbt: __u8,
    pub reserved1: [__u8; 3],
    pub reserved2: [__u32; 78],
    pub opt: __u8,
    pub reserved4: [__u8; 4],
    pub reserved5:5: __u8,
    pub ssid:3: __u8,
    pub devno: __u16,
    pub reserved6: [__u32; 5],
    pub bootprog: __u32,
    pub reserved7: [__u8; 12],
    pub cyl: __u16,
    pub head: __u8,
    pub record: __u8,
    pub reserved: __u32,
    pub __packed: } br_chr,
    pub scp_data_len: __u32,
    pub reserved8: [__u8; 260],
    pub scp_data: [__u8; ],
    pub __packed: },
pub const IPL_PB0_ECKD_OPT_IPL: c_uint = 0x10;
pub const IPL_PB0_ECKD_OPT_DUMP: c_uint = 0x20;
pub const IPL_PB0_CCW_VM_FLAG_NSS: c_uint = 0x80;
pub const IPL_PB0_CCW_VM_FLAG_VP: c_uint = 0x40;
// IPL Parameter Block 1 for additional SCP data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipl_pb1_scp_data {
    pub len: __u32,
    pub pbt: __u8,
    pub scp_data: [__u8; ],
    pub __packed: },
// IPL Report List header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipl_rl_hdr {
    pub len: __u32,
    pub flags: __u8,
    pub reserved1: [__u8; 2],
    pub version: __u8,
    pub reserved2: [__u8; 8],
    pub __packed: },
// IPL Report Block header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipl_rb_hdr {
    pub len: __u32,
    pub rbt: __u8,
    pub reserved1: [__u8; 11],
    pub __packed: },
// IPL Report Block types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipl_rbt {
    IPL_RBT_CERTIFICATES = 1,
    IPL_RBT_COMPONENTS = 2,
}

// IPL Report Block for the certificate list
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipl_rb_certificate_entry {
    pub addr: __u64,
    pub len: __u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipl_rb_certificates {
    pub len: __u32,
    pub rbt: __u8,
    pub reserved1: [__u8; 11],
    pub entries: [ipl_rb_certificate_entry; ],
    pub __packed: },
// IPL Report Block for the component list
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipl_rb_component_entry {
    pub addr: __u64,
    pub len: __u64,
    pub flags: __u8,
    pub reserved1: [__u8; 5],
    pub certificate_index: __u16,
    pub reserved2: [__u8; 8],
}

pub const IPL_RB_COMPONENT_FLAG_SIGNED: c_uint = 0x80;
pub const IPL_RB_COMPONENT_FLAG_VERIFIED: c_uint = 0x40;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipl_rb_components {
    pub len: __u32,
    pub rbt: __u8,
    pub reserved1: [__u8; 11],
    pub entries: [ipl_rb_component_entry; ],
    pub __packed: },
