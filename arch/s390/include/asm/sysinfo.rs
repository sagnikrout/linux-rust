//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/sysinfo.h
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
// definition for store system information stsi
//
// Copyright IBM Corp. 2001, 2008
//
// Author(s): Ulrich Weigand <weigand@de.ibm.com>
// Christian Borntraeger <borntraeger@de.ibm.com>
//

//
// stsi - store system information
//
// Returns the current configuration level if function code 0 was specified.
// Otherwise returns 0 on success or a negative value on error.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sysinfo_1_1_1 {
    pub p:1: c_uchar,
    pub :6: c_uchar,
    pub t:1: c_uchar,
    pub :8: c_uchar,
    pub ccr: c_uchar,
    pub cai: c_uchar,
    pub reserved_0: [c_char; 20],
    pub lic: c_ulong,
    pub manufacturer: [c_char; 16],
    pub type: [c_char; 4],
    pub reserved_1: [c_char; 12],
    pub model_capacity: [c_char; 16],
    pub sequence: [c_char; 16],
    pub plant: [c_char; 4],
    pub model: [c_char; 16],
    pub model_perm_cap: [c_char; 16],
    pub model_temp_cap: [c_char; 16],
    pub model_cap_rating: c_uint,
    pub model_perm_cap_rating: c_uint,
    pub model_temp_cap_rating: c_uint,
    pub typepct: [c_uchar; 5],
    pub reserved_2: [c_uchar; 3],
    pub ncr: c_uint,
    pub npr: c_uint,
    pub ntr: c_uint,
    pub reserved_3: [c_char; 4],
    pub model_var_cap: [c_char; 16],
    pub model_var_cap_rating: c_uint,
    pub nvr: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sysinfo_1_2_1 {
    pub reserved_0: [c_char; 80],
    pub sequence: [c_char; 16],
    pub plant: [c_char; 4],
    pub reserved_1: [c_char; 2],
    pub cpu_address: c_ushort,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sysinfo_1_2_2 {
    pub format: c_char,
    pub reserved_0: [c_char; 1],
    pub acc_offset: c_ushort,
    pub :1: unsigned char mt_installed,
    pub :2: c_uchar,
    pub :5: unsigned char mt_stid,
    pub :3: c_uchar,
    pub :5: unsigned char mt_gtid,
    pub reserved_1: [c_char; 18],
    pub nominal_cap: c_uint,
    pub secondary_cap: c_uint,
    pub capability: c_uint,
    pub cpus_total: c_ushort,
    pub cpus_configured: c_ushort,
    pub cpus_standby: c_ushort,
    pub cpus_reserved: c_ushort,
    pub adjustment: [c_ushort; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sysinfo_1_2_2_extension {
    pub alt_capability: c_uint,
    pub alt_adjustment: [c_ushort; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sysinfo_2_2_1 {
    pub reserved_0: [c_char; 80],
    pub sequence: [c_char; 16],
    pub plant: [c_char; 4],
    pub cpu_id: c_ushort,
    pub cpu_address: c_ushort,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sysinfo_2_2_2 {
    pub reserved_0: [c_char; 32],
    pub lpar_number: c_ushort,
    pub reserved_1: c_char,
    pub characteristics: c_uchar,
    pub cpus_total: c_ushort,
    pub cpus_configured: c_ushort,
    pub cpus_standby: c_ushort,
    pub cpus_reserved: c_ushort,
    pub name: [c_char; 8],
    pub caf: c_uint,
    pub reserved_2: [c_char; 8],
    pub :1: unsigned char mt_installed,
    pub :2: c_uchar,
    pub :5: unsigned char mt_stid,
    pub :3: c_uchar,
    pub :5: unsigned char mt_gtid,
    pub :3: c_uchar,
    pub :5: unsigned char mt_psmtid,
    pub reserved_3: [c_char; 5],
    pub cpus_dedicated: c_ushort,
    pub cpus_shared: c_ushort,
    pub reserved_4: [c_char; 3],
    pub vsne: c_uchar,
    pub uuid: uuid_t,
    pub reserved_5: [c_char; 160],
    pub ext_name: [c_char; 256],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sysinfo_3_2_2 {
    pub reserved_0: [c_char; 31],
    pub :4: c_uchar,
    pub count:4: c_uchar,
    pub reserved_0: [c_char; 4],
    pub cpus_total: c_ushort,
    pub cpus_configured: c_ushort,
    pub cpus_standby: c_ushort,
    pub cpus_reserved: c_ushort,
    pub name: [c_char; 8],
    pub caf: c_uint,
    pub cpi: [c_char; 16],
    pub reserved_1: [c_char; 3],
    pub evmne: c_uchar,
    pub reserved_2: c_uint,
    pub uuid: uuid_t,
    pub vm: [}; 8],
    pub reserved_3: [c_char; 1504],
    pub ext_names: [c_char; 8][256],
}

//
// Returns the maximum nesting level supported by the cpu topology code.
// The current maximum level is 4 which is the drawer level.
//
extern "C" {
    pub fn min(_arg: topology_max_mnest, _arg: 4) -> return;
}
pub const TOPOLOGY_NR_MAG: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct topology_core {
    pub nl: c_uchar,
    pub reserved0: [c_uchar; 3],
    pub :5: c_uchar,
    pub d:1: c_uchar,
    pub pp:2: c_uchar,
    pub reserved1: c_uchar,
    pub origin: c_ushort,
    pub mask: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct topology_container {
    pub nl: c_uchar,
    pub reserved: [c_uchar; 6],
    pub id: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union topology_entry {
    pub nl: c_uchar,
    pub cpu: topology_core,
    pub container: topology_container,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sysinfo_15_1_x {
    pub reserved0: [c_uchar; 2],
    pub length: c_ushort,
    pub mag: [c_uchar; TOPOLOGY_NR_MAG],
    pub reserved1: c_uchar,
    pub mnest: c_uchar,
    pub reserved2: [c_uchar; 4],
    pub tle: [topology_entry; ],
}

extern "C" {
    pub fn stsi(sysinfo: *mut c_void, fc: c_int, sel1: c_int, sel2: c_int) -> c_int;
}
//
// Service level reporting interface.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct service_level {
    pub list: list_head,
    pub ): *mut *mut *mut void (seq_print)(struct seq_file , struct service_level,
}

extern "C" {
    pub fn register_service_level(: *mut service_level) -> c_int;
}
extern "C" {
    pub fn unregister_service_level(: *mut service_level) -> c_int;
}
extern "C" {
    pub fn sthyi_fill(dst: *mut c_void, rc: *mut u64) -> c_int;
}
