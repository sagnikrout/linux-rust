//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/aic94xx/aic94xx_reg.h
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
// Aic94xx SAS/SATA driver hardware registers definitions.
//
// Copyright (C) 2005 Adaptec, Inc.  All rights reserved.
// Copyright (C) 2005 Luben Tuikov <luben_tuikov@adaptec.com>
//

// Values
pub const AIC9410_DEV_REV_B0: c_uint = 0x8;
// MBAR0, SWA, SWB, SWC, internal memory space addresses
pub const REG_BASE_ADDR: c_uint = 0xB8000000;
pub const REG_BASE_ADDR_CSEQCIO: c_uint = 0xB8002000;
pub const REG_BASE_ADDR_EXSI: c_uint = 0xB8042800;
pub const MBAR0_SWA_SIZE: c_uint = 0x58;
pub const MBAR0_SWC_SIZE: c_uint = 0x8;
// MBAR1, points to On Chip Memory
pub const OCM_BASE_ADDR: c_uint = 0xA0000000;
pub const OCM_MAX_SIZE: c_uint = 0x20000;
// Smallest address possible to reference

// PCI configuration space registers
pub const PCI_IOBAR_OFFSET: c_int = 4;
pub const PCI_CONF_MBAR1: c_uint = 0x6C;
pub const PCI_CONF_MBAR0_SWA: c_uint = 0x70;
pub const PCI_CONF_MBAR0_SWB: c_uint = 0x74;
pub const PCI_CONF_MBAR0_SWC: c_uint = 0x78;
pub const PCI_CONF_MBAR_KEY: c_uint = 0x7C;
pub const PCI_CONF_FLSH_BAR: c_uint = 0xB8;

extern "C" {
    pub fn asd_read_reg_byte(asd_ha: *mut asd_ha_struct, reg: u32) -> u8;
}
extern "C" {
    pub fn asd_read_reg_word(asd_ha: *mut asd_ha_struct, reg: u32) -> u16;
}
extern "C" {
    pub fn asd_read_reg_dword(asd_ha: *mut asd_ha_struct, reg: u32) -> u32;
}
extern "C" {
    pub fn asd_write_reg_byte(asd_ha: *mut asd_ha_struct, reg: u32, val: u8);
}
extern "C" {
    pub fn asd_write_reg_word(asd_ha: *mut asd_ha_struct, reg: u32, val: u16);
}
extern "C" {
    pub fn asd_write_reg_dword(asd_ha: *mut asd_ha_struct, reg: u32, val: u32);
}

//
// asd_ddbsite_update_word -- atomically update a word in a ddb site
// @asd_ha: pointer to host adapter structure
// @ddb_site_no: the DDB site number
// @offs: the offset into the DDB
// @oldval: old value found in that offset
// @newval: the new value to replace it
//
// This function is used when the sequencers are running and we need to
// update a DDB site atomically without expensive pausing and upausing
// of the sequencers and accessing the DDB site through the CIO bus.
//
// Return 0 on success; -EFAULT on parity error; -EAGAIN if the old value
// is different than the current value at that offset.
//
extern "C" {
    pub fn asd_ddbsite_update_word(_arg: asd_ha, _arg: ddb_site_no, _arg: base, _arg: oval, _arg: nval) -> return;
}
// DCHREVISION returns 0, possibly broken
// Enable COM SAS interrupt on errors, COMSTAT
// Enable DCH SAS CFIFTOERR
// Enable Host Device interrupts
