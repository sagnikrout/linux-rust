//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/edac/skx_common.h
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
// Common codes for both the skx_edac driver and Intel 10nm server EDAC driver.
// Originally split out from the skx_edac driver.
//
// Copyright (c) 2018, Intel Corporation.
//

pub const MSG_SIZE: c_int = 1024;
//
// Debug macros
//

//
// Get a bit field at register value <v>, from bit <lo> to bit <hi>
//

pub const I10NM_NUM_DDR_CHANNELS: c_int = 2;
pub const I10NM_NUM_DDR_DIMMS: c_int = 2;
pub const I10NM_NUM_HBM_CHANNELS: c_int = 2;
pub const I10NM_NUM_HBM_DIMMS: c_int = 1;

//
// According to Intel Architecture spec vol 3B,
// Table 15-10 "IA32_MCi_Status [15:0] Compound Error Code Encoding"
// memory errors should fit one of these masks:
// 000f 0000 1mmm cccc (binary)
// 000f 0010 1mmm cccc (binary)	[RAM used as cache]
// where:
// f = Correction Report Filtering Bit. If 1, subsequent errors
// won't be shown
// mmm = error type
// cccc = channel
//
pub const MCACOD_MEM_ERR_MASK: c_uint = 0xef80;
//
// Errors from either the memory of the 1-level memory system or the
// 2nd level memory (the slow "far" memory) of the 2-level memory system.
//
pub const MCACOD_MEM_CTL_ERR: c_uint = 0x80;
//
// Errors from the 1st level memory (the fast "near" memory as cache)
// of the 2-level memory system.
//
pub const MCACOD_EXT_MEM_ERR: c_uint = 0x280;
// Max RRL register sets per {,sub-,pseudo-}channel.
pub const NUM_RRL_SET: c_int = 4;
// Max RRL registers per set.
pub const NUM_RRL_REG: c_int = 7;
// Max correctable error count registers.
pub const NUM_CECNT_REG: c_int = 8;
// Error source from which the RRL registers log errors.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rrl_source_type {
// Last read error from patrol scrub.
    RRL_SRC_LRE_SCRUB,
// Last read error from demand.
    RRL_SRC_LRE_DEMAND,
// First read error from patrol scrub.
    RRL_SRC_FRE_SCRUB,
// First read error from demand.
    RRL_SRC_FRE_DEMAND,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rrl_ctrl_mode {
// Linux does not control RRL or reports values.
    RRL_CTRL_NONE,
// Firmware retains control. Linux only reports values.
    RRL_CTRL_BIOS,
// Linux takes control, resets mode bits, and clears valid/UC bits; reports values.
    RRL_CTRL_LINUX,
}

// RRL registers per {,sub-,pseudo-}channel.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg_rrl {
// RRL register parts.
    pub reg_num: int set_num,,
    pub sources: [rrl_source_type; NUM_RRL_SET],
    pub offsets: [u32; NUM_RRL_SET][NUM_RRL_REG],
// RRL register widths in byte per set.
    pub widths: [u8; NUM_RRL_REG],
// RRL control bits of the first register per set.
    pub v_mask: u32,
    pub uc_mask: u32,
    pub over_mask: u32,
    pub en_patspr_mask: u32,
    pub noover_mask: u32,
    pub en_mask: u32,
// CORRERRCNT register parts.
    pub cecnt_num: c_int,
    pub cecnt_offsets: [u32; NUM_CECNT_REG],
    pub cecnt_widths: [u8; NUM_CECNT_REG],
}

//
// Each cpu socket contains some pci devices that provide global
// information, and also some that are local to each of the two
// memory controllers on the die.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct skx_dev {
// {skx,i10nm}_edac
    pub bus: [u8; 4],
    pub seg: c_int,
    pub sad_all: *mut pci_dev,
    pub util_all: *mut pci_dev,
    pub uracu: *mut pci_dev,
    pub pcu_cr3: *mut pci_dev,
    pub mcroute: u32,
// imh_edac
// System-view MMIO base physical addresses.
    pub mmio_base_h_north: u64,
    pub mmio_base_h_south: u64,
    pub pkg: c_int,
    pub num_imc: c_int,
    pub list: list_head,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct skx_imc {
// i10nm_edac
    pub mdev: *mut pci_dev,
// imh_edac
    pub dev: *mut device,
    pub mci: *mut mem_ctl_info,
    pub mbase: *mut void __iomem,
    pub chan_mmio_sz: c_int,
    pub /: *mut *mut int num_channels; / channels per memory controller,
    pub /: *mut *mut int num_dimms; / dimms per channel,
    pub hbm_mc: bool,
    pub /: *mut *mut u8 mc; / system wide mc#,
    pub /: *mut *mut u8 lmc; / socket relative mc#,
    pub src_id: u8,
//
// Some server BIOS may hide certain memory controllers, and the
// EDAC driver skips those hidden memory controllers. However, the
// ADXL still decodes memory error address using physical memory
// controller indices. The mapping table is used to convert the
// physical indices (reported by ADXL) to the logical indices
// (used the EDAC driver) of present memory controllers during the
// error handling process.
//
    pub mc_mapping: u8,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct skx_channel {
    pub cdev: *mut pci_dev,
    pub edev: *mut pci_dev,
//
// Two groups of RRL control registers per channel to save default RRL
// settings of two {sub-,pseudo-}channels in Linux RRL control mode.
//
    pub rrl_ctl: [u32; 2][NUM_RRL_SET],
#[repr(C)]
#[derive(Copy, Clone)]
pub struct skx_dimm {
    pub close_pg: u8,
    pub bank_xor_enable: u8,
    pub fine_grain_bank: u8,
    pub rowbits: u8,
    pub colbits: u8,
    pub dimms: [}; NUM_DIMMS],
    pub chan: [}; NUM_CHANNELS],
    pub imc: [}; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct skx_pvt {
    pub imc: *mut skx_imc,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum type {
    SKX,
    I10NM,
    SPR,
    GNR,
    DMR,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum error_source {
    ERR_SRC_1LM,
    ERR_SRC_2LM_NM,
    ERR_SRC_2LM_FM,
    ERR_SRC_NOT_MEMORY,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct decoded_addr {
    pub mce: *mut mce,
    pub dev: *mut skx_dev,
    pub addr: u64,
    pub socket: c_int,
    pub imc: c_int,
    pub channel: c_int,
    pub chan_addr: u64,
    pub sktways: c_int,
    pub chanways: c_int,
    pub dimm: c_int,
    pub cs: c_int,
    pub subch: c_int,
    pub rank: c_int,
    pub channel_rank: c_int,
    pub rank_address: u64,
    pub row: c_int,
    pub column: c_int,
    pub bank_address: c_int,
    pub bank_group: c_int,
    pub decoded_by_adxl: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_bdf {
    pub 8: u32 bus :,
    pub 5: u32 dev :,
    pub 3: u32 fun :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct res_config {
    pub type: type,
// DDR memory controllers per socket
    pub ddr_imc_num: c_int,
// DDR channels per DDR memory controller
    pub ddr_chan_num: c_int,
// DDR DIMMs per DDR memory channel
    pub ddr_dimm_num: c_int,
// Per DDR channel memory-mapped I/O size
    pub ddr_chan_mmio_sz: c_int,
// HBM memory controllers per socket
    pub hbm_imc_num: c_int,
// HBM channels per HBM memory controller
    pub hbm_chan_num: c_int,
// HBM DIMMs per HBM memory channel
    pub hbm_dimm_num: c_int,
// Per HBM channel memory-mapped I/O size
    pub hbm_chan_mmio_sz: c_int,
    pub support_ddr5: bool,
// RRL register sets per DDR channel
    pub reg_rrl_ddr: [*mut reg_rrl; 2],
// RRL register sets per HBM channel
    pub reg_rrl_hbm: [*mut reg_rrl; 2],
// RRL control mode
    pub rrl_ctrl_mode: rrl_ctrl_mode,
// {skx,i10nm}_edac
// Configuration agent device ID
    pub decs_did: c_uint,
// Default bus number configuration register offset
    pub busno_cfg_offset: c_int,
    pub sad_all_bdf: pci_bdf,
    pub pcu_cr3_bdf: pci_bdf,
    pub util_all_bdf: pci_bdf,
    pub uracu_bdf: pci_bdf,
    pub ddr_mdev_bdf: pci_bdf,
    pub hbm_mdev_bdf: pci_bdf,
    pub sad_all_offset: c_int,
}

// imh_edac
// MMIO base physical address in local package view
extern "C" {
    pub fn bool(res: *mut *mut skx_decode_f)(struct decoded_addr) -> typedef;
}
extern "C" {
    pub fn void(res: *mut *mut skx_show_rrl_f)(struct decoded_addr, msg: *mut c_char, len: c_int, scrub_err: bool) -> typedef;
}
extern "C" {
    pub fn skx_readx(addr: *mut void __iomem, width: u8) -> u64;
}
extern "C" {
    pub fn skx_read_imc_reg(imc: *mut skx_imc, chan: c_int, offset: u32, width: u8) -> u64;
}
extern "C" {
    pub fn skx_write_imc_reg(imc: *mut skx_imc, chan: c_int, offset: u32, width: u8, val: u64);
}
extern "C" {
    pub fn skx_adxl_get() -> c_int;
}
extern "C" {
    pub fn skx_adxl_put();
}
extern "C" {
    pub fn skx_set_decode(decode: skx_decode_f);
}
extern "C" {
    pub fn skx_set_show_rrl(rrl: skx_show_rrl_f);
}
extern "C" {
    pub fn skx_show_rrl(res: *mut decoded_addr, msg: *mut c_char, len: c_int, scrub_err: bool);
}
extern "C" {
    pub fn skx_enable_rrl(enable: bool);
}
extern "C" {
    pub fn skx_set_mem_cfg(mem_cfg_2lm: bool);
}
extern "C" {
    pub fn skx_set_res_cfg(cfg: *mut res_config);
}
extern "C" {
    pub fn skx_init_mc_mapping(d: *mut skx_dev);
}
extern "C" {
    pub fn skx_set_mc_mapping(d: *mut skx_dev, pmc: u8, lmc: u8);
}
extern "C" {
    pub fn skx_get_src_id(d: *mut skx_dev, off: c_int, id: *mut u8) -> c_int;
}
extern "C" {
    pub fn skx_get_all_bus_mappings(cfg: *mut res_config, list: *mut list_head) -> c_int;
}
extern "C" {
    pub fn skx_get_hi_lo(did: c_uint, off[]: c_int, tolm: *mut u64, tohm: *mut u64) -> c_int;
}
extern "C" {
    pub fn skx_set_hi_lo(tolm: u64, tohm: u64);
}
extern "C" {
    pub fn skx_remove();
}

extern "C" {
    pub fn skx_setup_debug(name: *const c_char);
}
extern "C" {
    pub fn skx_teardown_debug();
}

