//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/pruss_driver.h
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
// PRU-ICSS sub-system specific definitions
//
// Copyright (C) 2014-2020 Texas Instruments Incorporated - http://www.ti.com
// Suman Anna <s-anna@ti.com>
//

//
// enum pruss_gp_mux_sel - PRUSS GPI/O Mux modes for the
// PRUSS_GPCFG0/1 registers
//
// NOTE: The below defines are the most common values, but there
// are some exceptions like on 66AK2G, where the RESERVED and MII2
// values are interchanged. Also, this bit-field does not exist on
// AM335x SoCs
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pruss_gp_mux_sel {
    PRUSS_GP_MUX_SEL_GP,
    PRUSS_GP_MUX_SEL_ENDAT,
    PRUSS_GP_MUX_SEL_RESERVED,
    PRUSS_GP_MUX_SEL_SD,
    PRUSS_GP_MUX_SEL_MII2,
    PRUSS_GP_MUX_SEL_MAX,
}

//
// enum pruss_gpi_mode - PRUSS GPI configuration modes, used
// to program the PRUSS_GPCFG0/1 registers
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pruss_gpi_mode {
    PRUSS_GPI_MODE_DIRECT,
    PRUSS_GPI_MODE_PARALLEL,
    PRUSS_GPI_MODE_28BIT_SHIFT,
    PRUSS_GPI_MODE_MII,
    PRUSS_GPI_MODE_MAX,
}

//
// enum pru_type - PRU core type identifier
//
// @PRU_TYPE_PRU: Programmable Real-time Unit
// @PRU_TYPE_RTU: Auxiliary Programmable Real-Time Unit
// @PRU_TYPE_TX_PRU: Transmit Programmable Real-Time Unit
// @PRU_TYPE_MAX: just keep this one at the end
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pru_type {
    PRU_TYPE_PRU,
    PRU_TYPE_RTU,
    PRU_TYPE_TX_PRU,
    PRU_TYPE_MAX,
}

//
// enum pruss_mem - PRUSS memory range identifiers
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pruss_mem {
    PRUSS_MEM_DRAM0 = 0,
    PRUSS_MEM_DRAM1,
    PRUSS_MEM_SHRD_RAM2,
    PRUSS_MEM_MAX,
}

//
// struct pruss_mem_region - PRUSS memory region structure
// @va: kernel virtual address of the PRUSS memory region
// @pa: physical (bus) address of the PRUSS memory region
// @size: size of the PRUSS memory region
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pruss_mem_region {
    pub va: *mut void __iomem,
    pub pa: phys_addr_t,
    pub size: usize,
}

//
// struct pruss - PRUSS parent structure
// @dev: pruss device pointer
// @cfg_base: base iomap for CFG region
// @cfg_regmap: regmap for config region
// @mem_regions: data for each of the PRUSS memory regions
// @mem_in_use: to indicate if memory resource is in use
// @lock: mutex to serialize access to resources
// @core_clk_mux: clk handle for PRUSS CORE_CLK_MUX
// @iep_clk_mux: clk handle for PRUSS IEP_CLK_MUX
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pruss {
    pub dev: *mut device,
    pub cfg_base: *mut void __iomem,
    pub cfg_regmap: *mut regmap,
    pub mem_regions: [pruss_mem_region; PRUSS_MEM_MAX],
    pub mem_in_use: [*mut pruss_mem_region; PRUSS_MEM_MAX],
    pub /: *mut *mut mutex lock; / PRU resource lock,
    pub core_clk_mux: *mut clk,
    pub iep_clk_mux: *mut clk,
}

extern "C" {
    pub fn pruss_put(pruss: *mut pruss);
}
extern "C" {
    pub fn pruss_cfg_get_gpmux(pruss: *mut pruss, pru_id: pruss_pru_id, mux: *mut u8) -> c_int;
}
extern "C" {
    pub fn pruss_cfg_set_gpmux(pruss: *mut pruss, pru_id: pruss_pru_id, mux: u8) -> c_int;
}
extern "C" {
    pub fn pruss_cfg_miirt_enable(pruss: *mut pruss, enable: bool) -> c_int;
}

extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}

