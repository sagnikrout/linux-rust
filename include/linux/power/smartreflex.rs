//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/power/smartreflex.h
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
// OMAP Smartreflex Defines and Routines
//
// Author: Thara Gopinath	<thara@ti.com>
//
// Copyright (C) 2010 Texas Instruments, Inc.
// Thara Gopinath <thara@ti.com>
//
// Copyright (C) 2008 Nokia Corporation
// Kalle Jokiniemi
//
// Copyright (C) 2007 Texas Instruments, Inc.
// Lesly A M <x0080970@ti.com>
//

//
// Different Smartreflex IPs version. The v1 is the 65nm version used in
// OMAP3430. The v2 is the update for the 45nm version of the IP
// used in OMAP3630 and OMAP4430
//
pub const SR_TYPE_V1: c_int = 1;
pub const SR_TYPE_V2: c_int = 2;
// SMART REFLEX REG ADDRESS OFFSET
pub const SRCONFIG: c_uint = 0x00;
pub const SRSTATUS: c_uint = 0x04;
pub const SENVAL: c_uint = 0x08;
pub const SENMIN: c_uint = 0x0C;
pub const SENMAX: c_uint = 0x10;
pub const SENAVG: c_uint = 0x14;
pub const AVGWEIGHT: c_uint = 0x18;
pub const NVALUERECIPROCAL: c_uint = 0x1c;
pub const SENERROR_V1: c_uint = 0x20;
pub const ERRCONFIG_V1: c_uint = 0x24;
pub const IRQ_EOI: c_uint = 0x20;
pub const IRQSTATUS_RAW: c_uint = 0x24;
pub const IRQSTATUS: c_uint = 0x28;
pub const IRQENABLE_SET: c_uint = 0x2C;
pub const IRQENABLE_CLR: c_uint = 0x30;
pub const SENERROR_V2: c_uint = 0x34;
pub const ERRCONFIG_V2: c_uint = 0x38;
// Bit/Shift Positions
// SRCONFIG
pub const SRCONFIG_ACCUMDATA_SHIFT: c_int = 22;
pub const SRCONFIG_SRCLKLENGTH_SHIFT: c_int = 12;
pub const SRCONFIG_SENNENABLE_V1_SHIFT: c_int = 5;
pub const SRCONFIG_SENPENABLE_V1_SHIFT: c_int = 3;
pub const SRCONFIG_SENNENABLE_V2_SHIFT: c_int = 1;
pub const SRCONFIG_SENPENABLE_V2_SHIFT: c_int = 0;
pub const SRCONFIG_CLKCTRL_SHIFT: c_int = 0;

// AVGWEIGHT
pub const AVGWEIGHT_SENPAVGWEIGHT_SHIFT: c_int = 2;
pub const AVGWEIGHT_SENNAVGWEIGHT_SHIFT: c_int = 0;
// NVALUERECIPROCAL
pub const NVALUERECIPROCAL_SENPGAIN_SHIFT: c_int = 20;
pub const NVALUERECIPROCAL_SENNGAIN_SHIFT: c_int = 16;
pub const NVALUERECIPROCAL_RNSENP_SHIFT: c_int = 8;
pub const NVALUERECIPROCAL_RNSENN_SHIFT: c_int = 0;
// ERRCONFIG
pub const ERRCONFIG_ERRWEIGHT_SHIFT: c_int = 16;
pub const ERRCONFIG_ERRMAXLIMIT_SHIFT: c_int = 8;
pub const ERRCONFIG_ERRMINLIMIT_SHIFT: c_int = 0;

// IRQSTATUS

// IRQENABLE_SET and IRQENABLE_CLEAR

// Common Bit values
pub const SRCLKLENGTH_12MHZ_SYSCLK: c_uint = 0x3c;
pub const SRCLKLENGTH_13MHZ_SYSCLK: c_uint = 0x41;
pub const SRCLKLENGTH_19MHZ_SYSCLK: c_uint = 0x60;
pub const SRCLKLENGTH_26MHZ_SYSCLK: c_uint = 0x82;
pub const SRCLKLENGTH_38MHZ_SYSCLK: c_uint = 0xC0;
//
// 3430 specific values. Maybe these should be passed from board file or
// pmic structures.
//
pub const OMAP3430_SR_ACCUMDATA: c_uint = 0x1f4;
pub const OMAP3430_SR1_SENPAVGWEIGHT: c_uint = 0x03;
pub const OMAP3430_SR1_SENNAVGWEIGHT: c_uint = 0x03;
pub const OMAP3430_SR2_SENPAVGWEIGHT: c_uint = 0x01;
pub const OMAP3430_SR2_SENNAVGWEIGHT: c_uint = 0x01;
pub const OMAP3430_SR_ERRWEIGHT: c_uint = 0x04;
pub const OMAP3430_SR_ERRMAXLIMIT: c_uint = 0x02;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sr_instance {
    OMAP_SR_MPU,			/* shared with iva on omap3 */
    OMAP_SR_CORE,
    OMAP_SR_IVA,
    OMAP_SR_NR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_sr {
    pub name: *mut c_char,
    pub node: list_head,
    pub pdev: *mut platform_device,
    pub nvalue_table: *mut omap_sr_nvalue_table,
    pub voltdm: *mut voltagedomain,
    pub dbg_dir: *mut dentry,
    pub irq: c_uint,
    pub fck: *mut clk,
    pub srid: c_int,
    pub ip_type: c_int,
    pub nvalue_count: c_int,
    pub autocomp_active: bool,
    pub clk_length: u32,
    pub err_weight: u32,
    pub err_minlimit: u32,
    pub err_maxlimit: u32,
    pub accum_data: u32,
    pub senn_avgweight: u32,
    pub senp_avgweight: u32,
    pub senp_mod: u32,
    pub senn_mod: u32,
    pub base: *mut void __iomem,
    pub enabled:1: c_ulong,
}

//
// test_cond_timeout - busy-loop, testing a condition
// @cond: condition to test until it evaluates to true
// @timeout: maximum number of microseconds in the timeout
// @index: loop index (integer)
//
// Loop waiting for @cond to become true or until at least @timeout
// microseconds have passed.  To use, define some integer @index in the
// calling code.  After running, if @index == @timeout, then the loop has
// timed out.
//
// Copied from omap_test_timeout

//
// struct omap_sr_pmic_data - Strucutre to be populated by pmic code to pass
// pmic specific info to smartreflex driver
//
// @sr_pmic_init:	API to initialize smartreflex on the PMIC side.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_sr_pmic_data {
    pub (void): *mut *mut void (sr_pmic_init),
}

//
// struct omap_smartreflex_dev_attr - Smartreflex Device attribute.
//
// @sensor_voltdm_name:       Name of voltdomain of SR instance
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_smartreflex_dev_attr {
    pub sensor_voltdm_name: *const c_char,
}

//
// The smart reflex driver supports CLASS1 CLASS2 and CLASS3 SR.
// The smartreflex class driver should pass the class type.
// Should be used to populate the class_type field of the
// omap_smartreflex_class_data structure.
//
pub const SR_CLASS1: c_uint = 0x1;
pub const SR_CLASS2: c_uint = 0x2;
pub const SR_CLASS3: c_uint = 0x3;
//
// struct omap_sr_class_data - Smartreflex class driver info
//
// @enable:		API to enable a particular class smaartreflex.
// @disable:		API to disable a particular class smartreflex.
// @configure:		API to configure a particular class smartreflex.
// @notify:		API to notify the class driver about an event in SR.
// Not needed for class3.
// @notify_flags:	specify the events to be notified to the class driver
// @class_type:		specify which smartreflex class.
// Can be used by the SR driver to take any class
// based decisions.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_sr_class_data {
    pub sr): *mut *mut int (enable)(struct omap_sr,
    pub is_volt_reset): *mut *mut *mut int (disable)(struct omap_sr sr, int,
    pub sr): *mut *mut int (configure)(struct omap_sr,
    pub status): *mut *mut *mut int (notify)(struct omap_sr sr, u32,
    pub notify_flags: u8,
    pub class_type: u8,
}

//
// struct omap_sr_nvalue_table	- Smartreflex n-target value info
//
// @efuse_offs:	  The offset of the efuse where n-target values are stored.
// @nvalue:	  The n-target value.
// @errminlimit:  The value of the ERRMINLIMIT bitfield for this n-target
// @volt_nominal: microvolts DC that the VDD is initially programmed to
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_sr_nvalue_table {
    pub efuse_offs: u32,
    pub nvalue: u32,
    pub errminlimit: u32,
    pub volt_nominal: c_ulong,
}

//
// struct omap_sr_data - Smartreflex platform data.
//
// @name:		instance name
// @ip_type:		Smartreflex IP type.
// @senp_mod:		SENPENABLE value of the sr CONFIG register
// @senn_mod:		SENNENABLE value for sr CONFIG register
// @err_weight		ERRWEIGHT value of the sr ERRCONFIG register
// @err_maxlimit	ERRMAXLIMIT value of the sr ERRCONFIG register
// @accum_data		ACCUMDATA value of the sr CONFIG register
// @senn_avgweight	SENNAVGWEIGHT value of the sr AVGWEIGHT register
// @senp_avgweight	SENPAVGWEIGHT value of the sr AVGWEIGHT register
// @nvalue_count:	Number of distinct nvalues in the nvalue table
// @nvalue_table:	table containing the  efuse offsets and nvalues
// corresponding to them.
// @voltdm:		Pointer to the voltage domain associated with the SR
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_sr_data {
    pub name: *const c_char,
    pub ip_type: c_int,
    pub senp_mod: u32,
    pub senn_mod: u32,
    pub err_weight: u32,
    pub err_maxlimit: u32,
    pub accum_data: u32,
    pub senn_avgweight: u32,
    pub senp_avgweight: u32,
    pub nvalue_count: c_int,
    pub nvalue_table: *mut omap_sr_nvalue_table,
    pub voltdm: *mut voltagedomain,
}

// Smartreflex module enable/disable interface
extern "C" {
    pub fn omap_sr_enable(voltdm: *mut voltagedomain);
}
extern "C" {
    pub fn omap_sr_disable(voltdm: *mut voltagedomain);
}
extern "C" {
    pub fn omap_sr_disable_reset_volt(voltdm: *mut voltagedomain);
}
// Smartreflex driver hooks to be called from Smartreflex class driver
extern "C" {
    pub fn sr_enable(sr: *mut omap_sr, volt: c_ulong) -> c_int;
}
extern "C" {
    pub fn sr_disable(sr: *mut omap_sr);
}
extern "C" {
    pub fn sr_configure_errgen(sr: *mut omap_sr) -> c_int;
}
extern "C" {
    pub fn sr_disable_errgen(sr: *mut omap_sr) -> c_int;
}
extern "C" {
    pub fn sr_configure_minmax(sr: *mut omap_sr) -> c_int;
}
// API to register the smartreflex class driver with the smartreflex driver
extern "C" {
    pub fn sr_register_class(class_data: *mut omap_sr_class_data) -> c_int;
}

