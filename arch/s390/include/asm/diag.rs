//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/diag.h
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
// s390 diagnose functions
//
// Copyright IBM Corp. 2007
// Author(s): Michael Holzheu <holzheu@de.ibm.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum diag_stat_enum {
    DIAG_STAT_X008,
    DIAG_STAT_X00C,
    DIAG_STAT_X010,
    DIAG_STAT_X014,
    DIAG_STAT_X044,
    DIAG_STAT_X064,
    DIAG_STAT_X08C,
    DIAG_STAT_X09C,
    DIAG_STAT_X0DC,
    DIAG_STAT_X204,
    DIAG_STAT_X210,
    DIAG_STAT_X224,
    DIAG_STAT_X250,
    DIAG_STAT_X258,
    DIAG_STAT_X26C,
    DIAG_STAT_X288,
    DIAG_STAT_X2C4,
    DIAG_STAT_X2FC,
    DIAG_STAT_X304,
    DIAG_STAT_X308,
    DIAG_STAT_X310,
    DIAG_STAT_X318,
    DIAG_STAT_X320,
    DIAG_STAT_X324,
    DIAG_STAT_X49C,
    DIAG_STAT_X500,
    NR_DIAG_STAT
}

extern "C" {
    pub fn diag_stat_inc(nr: diag_stat_enum);
}
extern "C" {
    pub fn diag_stat_inc_norecursion(nr: diag_stat_enum);
}
//
// Diagnose 0c: Pseudo Timer
//
extern "C" {
    pub fn diag0c(data: *mut hypfs_diag0c_entry);
}
//
// Diagnose 10: Release page range
//
// Diagnose 14: Input spool file manipulation
//
extern "C" {
    pub fn diag14(rx: c_ulong, ry1: c_ulong, subcode: c_ulong) -> c_int;
}
//
// Diagnose 210: Get information about a virtual device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct diag210 {
    pub /: *mut *mut u16 vrdcdvno; / device number (input),
    pub /: *mut *mut u16 vrdclen; / data block length (input),
    pub /: *mut *mut u8 vrdcvcla; / virtual device class (output),
    pub /: *mut *mut u8 vrdcvtyp; / virtual device type (output),
    pub /: *mut *mut u8 vrdcvsta; / virtual device status (output),
    pub /: *mut *mut u8 vrdcvfla; / virtual device flags (output),
    pub /: *mut *mut u8 vrdcrccl; / real device class (output),
    pub /: *mut *mut u8 vrdccrty; / real device type (output),
    pub /: *mut *mut u8 vrdccrmd; / real device model (output),
    pub /: *mut *mut u8 vrdccrft; / real device feature (output),
    pub __aligned(4): } __packed,
    pub addr): *mut extern int diag210(struct diag210,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct diag8c {
    pub flags: u8,
    pub num_partitions: u8,
    pub width: u16,
    pub height: u16,
    pub data: [u8; ],
    pub __aligned(4): } __packed,
    pub devno): *mut *mut extern int diag8c(struct diag8c out, struct ccw_dev_id,
// bit is set in flags, when physical cpu info is included in diag 204 data
pub const DIAG204_LPAR_PHYS_FLG: c_uint = 0x80;

// diag 204 subcodes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum diag204_sc {
    DIAG204_SUBC_STIB4 = 4,
    DIAG204_SUBC_RSI = 5,
    DIAG204_SUBC_STIB6 = 6,
    DIAG204_SUBC_STIB7 = 7
}

pub const DIAG204_SUBCODE_MASK: c_uint = 0xffff;
pub const DIAG204_BIF_BIT: c_uint = 0x80000000;

// The two available diag 204 data formats
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum diag204_format {
    DIAG204_INFO_SIMPLE = 0,
    DIAG204_INFO_EXT = 0x00010000
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum diag204_cpu_flags {
    DIAG204_CPU_ONLINE = 0x20,
    DIAG204_CPU_CAPPED = 0x40,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct diag204_info_blk_hdr {
    pub npar: __u8,
    pub flags: __u8,
    pub tslice: __u16,
    pub phys_cpus: __u16,
    pub this_part: __u16,
    pub curtod: __u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct diag204_x_info_blk_hdr {
    pub npar: __u8,
    pub flags: __u8,
    pub tslice: __u16,
    pub phys_cpus: __u16,
    pub this_part: __u16,
    pub curtod1: __u64,
    pub curtod2: __u64,
    pub reserved: [c_char; 40],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct diag204_part_hdr {
    pub pn: __u8,
    pub cpus: __u8,
    pub reserved: [c_char; 6],
    pub part_name: [c_char; DIAG204_LPAR_NAME_LEN],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct diag204_x_part_hdr {
    pub pn: __u8,
    pub cpus: __u8,
    pub rcpus: __u8,
    pub pflag: __u8,
    pub mlu: __u32,
    pub part_name: [c_char; DIAG204_LPAR_NAME_LEN],
    pub lpc_name: [c_char; 8],
    pub os_name: [c_char; 8],
    pub online_cs: __u64,
    pub online_es: __u64,
    pub upid: __u8,
    pub reserved:3: __u8,
    pub mtid:5: __u8,
    pub reserved1: [c_char; 2],
    pub group_mlu: __u32,
    pub group_name: [c_char; 8],
    pub hardware_group_name: [c_char; 8],
    pub reserved2: [c_char; 24],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct diag204_cpu_info {
    pub cpu_addr: __u16,
    pub reserved1: [c_char; 2],
    pub ctidx: __u8,
    pub cflag: __u8,
    pub weight: __u16,
    pub acc_time: __u64,
    pub lp_time: __u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct diag204_x_cpu_info {
    pub cpu_addr: __u16,
    pub reserved1: [c_char; 2],
    pub ctidx: __u8,
    pub cflag: __u8,
    pub weight: __u16,
    pub acc_time: __u64,
    pub lp_time: __u64,
    pub min_weight: __u16,
    pub cur_weight: __u16,
    pub max_weight: __u16,
    pub reseved2: [c_char; 2],
    pub online_time: __u64,
    pub wait_time: __u64,
    pub pma_weight: __u32,
    pub polar_weight: __u32,
    pub cpu_type_cap: __u32,
    pub group_cpu_type_cap: __u32,
    pub reserved3: [c_char; 32],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct diag204_phys_hdr {
    pub reserved1: [c_char; 1],
    pub cpus: __u8,
    pub reserved2: [c_char; 6],
    pub mgm_name: [c_char; 8],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct diag204_x_phys_hdr {
    pub reserved1: [c_char; 1],
    pub cpus: __u8,
    pub reserved2: [c_char; 6],
    pub mgm_name: [c_char; 8],
    pub reserved3: [c_char; 80],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct diag204_phys_cpu {
    pub cpu_addr: __u16,
    pub reserved1: [c_char; 2],
    pub ctidx: __u8,
    pub reserved2: [c_char; 3],
    pub mgm_time: __u64,
    pub reserved3: [c_char; 8],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct diag204_x_phys_cpu {
    pub cpu_addr: __u16,
    pub reserved1: [c_char; 2],
    pub ctidx: __u8,
    pub reserved2: [c_char; 1],
    pub weight: __u16,
    pub mgm_time: __u64,
    pub reserved3: [c_char; 80],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct diag204_x_part_block {
    pub hdr: diag204_x_part_hdr,
    pub cpus: [diag204_x_cpu_info; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct diag204_x_phys_block {
    pub hdr: diag204_x_phys_hdr,
    pub cpus: [diag204_x_phys_cpu; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum diag26c_sc {
    DIAG26C_PORT_VNIC    = 0x00000024,
    DIAG26C_MAC_SERVICES = 0x00000030
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum diag26c_version {
    DIAG26C_VERSION2	 = 0x00000002,	/* z/VM 5.4.0 */
    DIAG26C_VERSION6_VM65918 = 0x00020006	/* z/VM 6.4.0 + VM65918 */
}

pub const DIAG26C_VNIC_INFO: c_uint = 0x0002;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct diag26c_vnic_req {
    pub resp_buf_len: u32,
    pub resp_version: u32,
    pub req_format: u16,
    pub vlan_id: u16,
    pub sys_name: u64,
    pub res: [u8; 2],
    pub devno: u16,
    pub __aligned(8): } __packed,
pub const VNIC_INFO_PROT_L3: c_int = 1;
pub const VNIC_INFO_PROT_L2: c_int = 2;
// Note: this is the bare minimum, use it for uninitialized VNICs only.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct diag26c_vnic_resp {
    pub version: u32,
    pub entry_cnt: u32,
// VNIC info:
    pub next_entry: u32,
    pub owner: u64,
    pub devno: u16,
    pub status: u8,
    pub type: u8,
    pub lan_owner: u64,
    pub lan_name: u64,
    pub port_name: u64,
    pub port_type: u8,
    pub ext_status:6: u8,
    pub protocol:2: u8,
    pub base_devno: u16,
    pub port_num: u32,
    pub ifindex: u32,
    pub maxinfo: u32,
    pub dev_count: u32,
// 3x device info:
    pub dev_info1: [u8; 28],
    pub dev_info2: [u8; 28],
    pub dev_info3: [u8; 28],
    pub __aligned(8): } __packed,
pub const DIAG26C_GET_MAC: c_uint = 0x0000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct diag26c_mac_req {
    pub resp_buf_len: u32,
    pub resp_version: u32,
    pub op_code: u16,
    pub devno: u16,
    pub res: [u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct diag26c_mac_resp {
    pub version: u32,
    pub mac: [u8; ETH_ALEN],
    pub res: [u8; 2],
    pub __aligned(8): },
pub const CPNC_LINUX: c_uint = 0x4;
#[repr(C)]
#[derive(Copy, Clone)]
pub union diag318_info {
    pub val: c_ulong,
    pub 8: unsigned long cpnc :,
    pub 56: unsigned long cpvc :,
}

extern "C" {
    pub fn diag204(subcode: c_ulong, size: c_ulong, addr: *mut c_void) -> c_int;
}
extern "C" {
    pub fn diag224(ptr: *mut c_void) -> c_int;
}
extern "C" {
    pub fn diag26c(req: *mut c_void, resp: *mut c_void, subcode: diag26c_sc) -> c_int;
}
//
// This structure must contain only pointers/references into
// the AMODE31 text section.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct diag_ops {
    pub addr): *mut *mut int (diag210)(struct diag210,
    pub subcode): *mut *mut int (diag26c)(unsigned long rx, unsigned long rx1, enum diag26c_sc,
    pub subcode): *mut *mut int (diag14)(unsigned long rx, unsigned long ry1, unsigned long,
    pub len): *mut *mut *mut *mut int (diag8c)(struct diag8c addr, struct ccw_dev_id devno, size_t,
    pub rx): *mut *mut void (diag0c)(unsigned long,
    pub (*diag308_reset)(void): *mut c_void,
}

extern "C" {
    pub fn _diag210_amode31(addr: *mut diag210) -> c_int;
}
extern "C" {
    pub fn _diag26c_amode31(rx: c_ulong, rx1: c_ulong, subcode: diag26c_sc) -> c_int;
}
extern "C" {
    pub fn _diag14_amode31(rx: c_ulong, ry1: c_ulong, subcode: c_ulong) -> c_int;
}
extern "C" {
    pub fn _diag0c_amode31(rx: c_ulong);
}
extern "C" {
    pub fn _diag308_reset_amode31();
}
extern "C" {
    pub fn _diag8c_amode31(addr: *mut diag8c, devno: *mut ccw_dev_id, len: usize) -> c_int;
}
// diag 49c subcodes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum diag49c_sc {
    DIAG49C_SUBC_ACK = 0,
    DIAG49C_SUBC_REG = 1
}

extern "C" {
    pub fn diag49c(subcode: c_ulong) -> c_int;
}
