//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/ti-sysc.h
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
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ti_sysc_module_type {
    TI_SYSC_OMAP2,
    TI_SYSC_OMAP2_TIMER,
    TI_SYSC_OMAP3_SHAM,
    TI_SYSC_OMAP3_AES,
    TI_SYSC_OMAP4,
    TI_SYSC_OMAP4_TIMER,
    TI_SYSC_OMAP4_SIMPLE,
    TI_SYSC_OMAP34XX_SR,
    TI_SYSC_OMAP36XX_SR,
    TI_SYSC_OMAP4_SR,
    TI_SYSC_OMAP4_MCASP,
    TI_SYSC_OMAP4_USB_HOST_FS,
    TI_SYSC_DRA7_MCAN,
    TI_SYSC_PRUSS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_sysc_cookie {
    pub data: *mut c_void,
    pub clkdm: *mut c_void,
}

//
// struct sysc_regbits - TI OCP_SYSCONFIG register field offsets
// @midle_shift: Offset of the midle bit
// @clkact_shift: Offset of the clockactivity bit
// @sidle_shift: Offset of the sidle bit
// @enwkup_shift: Offset of the enawakeup bit
// @srst_shift: Offset of the softreset bit
// @autoidle_shift: Offset of the autoidle bit
// @dmadisable_shift: Offset of the dmadisable bit
// @emufree_shift; Offset of the emufree bit
//
// Note that 0 is a valid shift, and for ti-sysc.c -ENODEV can be used if a
// feature is not available.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sysc_regbits {
    pub midle_shift: i8,
    pub clkact_shift: i8,
    pub sidle_shift: i8,
    pub enwkup_shift: i8,
    pub srst_shift: i8,
    pub autoidle_shift: i8,
    pub dmadisable_shift: i8,
    pub emufree_shift: i8,
}

pub const SYSC_NR_IDLEMODES: c_int = 4;
//
// struct sysc_capabilities - capabilities for an interconnect target module
// @type: sysc type identifier for the module
// @sysc_mask: bitmask of supported SYSCONFIG register bits
// @regbits: bitmask of SYSCONFIG register bits
// @mod_quirks: bitmask of module specific quirks
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sysc_capabilities {
    pub type: ti_sysc_module_type,
    pub sysc_mask: u32,
    pub regbits: *const sysc_regbits,
    pub mod_quirks: u32,
}

//
// struct sysc_config - configuration for an interconnect target module
// @sysc_val: configured value for sysc register
// @syss_mask: configured mask value for SYSSTATUS register
// @midlemodes: bitmask of supported master idle modes
// @sidlemodes: bitmask of supported slave idle modes
// @srst_udelay: optional delay needed after OCP soft reset
// @quirks: bitmask of enabled quirks
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sysc_config {
    pub sysc_val: u32,
    pub syss_mask: u32,
    pub midlemodes: u8,
    pub sidlemodes: u8,
    pub srst_udelay: u8,
    pub quirks: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sysc_registers {
    SYSC_REVISION,
    SYSC_SYSCONFIG,
    SYSC_SYSSTATUS,
    SYSC_MAX_REGS,
}

//
// struct ti_sysc_module_data - ti-sysc to hwmod translation data for a module
// @name: legacy "ti,hwmods" module name
// @module_pa: physical address of the interconnect target module
// @module_size: size of the interconnect target module
// @offsets: array of register offsets as listed in enum sysc_registers
// @nr_offsets: number of registers
// @cap: interconnect target module capabilities
// @cfg: interconnect target module configuration
//
// This data is enough to allocate a new struct omap_hwmod_class_sysconfig
// based on device tree data parsed by ti-sysc driver.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_sysc_module_data {
    pub name: *const c_char,
    pub module_pa: u64,
    pub module_size: u32,
    pub offsets: *mut c_int,
    pub nr_offsets: c_int,
    pub cap: *const sysc_capabilities,
    pub cfg: *mut sysc_config,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_sysc_platform_data {
    pub auxdata: *mut of_dev_auxdata,
    pub (*soc_type_gp)(void): *mut bool,
    pub cookie): *mut *mut clk ick, ti_sysc_cookie,
    pub cookie): *const ti_sysc_cookie,
    pub cookie): *const ti_sysc_cookie,
    pub cookie): *mut ti_sysc_cookie,
    pub cookie): *const ti_sysc_cookie,
    pub cookie): *const ti_sysc_cookie,
    pub cookie): *const ti_sysc_cookie,
}
