//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/spi/spi-cavium.h
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

pub const OCTEON_SPI_MAX_BYTES: c_int = 9;
pub const OCTEON_SPI_MAX_CLOCK_HZ: c_int = 16000000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octeon_spi_regs {
    pub config: c_int,
    pub status: c_int,
    pub tx: c_int,
    pub data: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct octeon_spi {
    pub register_base: *mut void __iomem,
    pub last_cfg: u64,
    pub cs_enax: u64,
    pub sys_freq: c_int,
    pub regs: octeon_spi_regs,
    pub clk: *mut clk,
}

// MPI register descriptions

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_mpi_cfg {
    pub u64: u64,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_mpi_cfg_s {

    pub reserved_29_63:35: u64,
    pub clkdiv:13: u64,
    pub csena3:1: u64,
    pub csena2:1: u64,
    pub csena1:1: u64,
    pub csena0:1: u64,
    pub cslate:1: u64,
    pub tritx:1: u64,
    pub idleclks:2: u64,
    pub cshi:1: u64,
    pub csena:1: u64,
    pub int_ena:1: u64,
    pub lsbfirst:1: u64,
    pub wireor:1: u64,
    pub clk_cont:1: u64,
    pub idlelo:1: u64,
    pub enable:1: u64,

    pub enable:1: u64,
    pub idlelo:1: u64,
    pub clk_cont:1: u64,
    pub wireor:1: u64,
    pub lsbfirst:1: u64,
    pub int_ena:1: u64,
    pub csena:1: u64,
    pub cshi:1: u64,
    pub idleclks:2: u64,
    pub tritx:1: u64,
    pub cslate:1: u64,
    pub csena0:1: u64,
    pub csena1:1: u64,
    pub csena2:1: u64,
    pub csena3:1: u64,
    pub clkdiv:13: u64,
    pub reserved_29_63:35: u64,

    pub s: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_mpi_cfg_cn30xx {

    pub reserved_29_63:35: u64,
    pub clkdiv:13: u64,
    pub reserved_12_15:4: u64,
    pub cslate:1: u64,
    pub tritx:1: u64,
    pub idleclks:2: u64,
    pub cshi:1: u64,
    pub csena:1: u64,
    pub int_ena:1: u64,
    pub lsbfirst:1: u64,
    pub wireor:1: u64,
    pub clk_cont:1: u64,
    pub idlelo:1: u64,
    pub enable:1: u64,

    pub enable:1: u64,
    pub idlelo:1: u64,
    pub clk_cont:1: u64,
    pub wireor:1: u64,
    pub lsbfirst:1: u64,
    pub int_ena:1: u64,
    pub csena:1: u64,
    pub cshi:1: u64,
    pub idleclks:2: u64,
    pub tritx:1: u64,
    pub cslate:1: u64,
    pub reserved_12_15:4: u64,
    pub clkdiv:13: u64,
    pub reserved_29_63:35: u64,

    pub cn30xx: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_mpi_cfg_cn31xx {

    pub reserved_29_63:35: u64,
    pub clkdiv:13: u64,
    pub reserved_11_15:5: u64,
    pub tritx:1: u64,
    pub idleclks:2: u64,
    pub cshi:1: u64,
    pub csena:1: u64,
    pub int_ena:1: u64,
    pub lsbfirst:1: u64,
    pub wireor:1: u64,
    pub clk_cont:1: u64,
    pub idlelo:1: u64,
    pub enable:1: u64,

    pub enable:1: u64,
    pub idlelo:1: u64,
    pub clk_cont:1: u64,
    pub wireor:1: u64,
    pub lsbfirst:1: u64,
    pub int_ena:1: u64,
    pub csena:1: u64,
    pub cshi:1: u64,
    pub idleclks:2: u64,
    pub tritx:1: u64,
    pub reserved_11_15:5: u64,
    pub clkdiv:13: u64,
    pub reserved_29_63:35: u64,

    pub cn31xx: },
    pub cn50xx: cvmx_mpi_cfg_cn30xx,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_mpi_cfg_cn61xx {

    pub reserved_29_63:35: u64,
    pub clkdiv:13: u64,
    pub reserved_14_15:2: u64,
    pub csena1:1: u64,
    pub csena0:1: u64,
    pub cslate:1: u64,
    pub tritx:1: u64,
    pub idleclks:2: u64,
    pub cshi:1: u64,
    pub reserved_6_6:1: u64,
    pub int_ena:1: u64,
    pub lsbfirst:1: u64,
    pub wireor:1: u64,
    pub clk_cont:1: u64,
    pub idlelo:1: u64,
    pub enable:1: u64,

    pub enable:1: u64,
    pub idlelo:1: u64,
    pub clk_cont:1: u64,
    pub wireor:1: u64,
    pub lsbfirst:1: u64,
    pub int_ena:1: u64,
    pub reserved_6_6:1: u64,
    pub cshi:1: u64,
    pub idleclks:2: u64,
    pub tritx:1: u64,
    pub cslate:1: u64,
    pub csena0:1: u64,
    pub csena1:1: u64,
    pub reserved_14_15:2: u64,
    pub clkdiv:13: u64,
    pub reserved_29_63:35: u64,

    pub cn61xx: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_mpi_cfg_cn66xx {

    pub reserved_29_63:35: u64,
    pub clkdiv:13: u64,
    pub csena3:1: u64,
    pub csena2:1: u64,
    pub reserved_12_13:2: u64,
    pub cslate:1: u64,
    pub tritx:1: u64,
    pub idleclks:2: u64,
    pub cshi:1: u64,
    pub reserved_6_6:1: u64,
    pub int_ena:1: u64,
    pub lsbfirst:1: u64,
    pub wireor:1: u64,
    pub clk_cont:1: u64,
    pub idlelo:1: u64,
    pub enable:1: u64,

    pub enable:1: u64,
    pub idlelo:1: u64,
    pub clk_cont:1: u64,
    pub wireor:1: u64,
    pub lsbfirst:1: u64,
    pub int_ena:1: u64,
    pub reserved_6_6:1: u64,
    pub cshi:1: u64,
    pub idleclks:2: u64,
    pub tritx:1: u64,
    pub cslate:1: u64,
    pub reserved_12_13:2: u64,
    pub csena2:1: u64,
    pub csena3:1: u64,
    pub clkdiv:13: u64,
    pub reserved_29_63:35: u64,

    pub cn66xx: },
    pub cnf71xx: cvmx_mpi_cfg_cn61xx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_mpi_datx {
    pub u64: u64,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_mpi_datx_s {

    pub reserved_8_63:56: u64,
    pub data:8: u64,

    pub data:8: u64,
    pub reserved_8_63:56: u64,

    pub s: },
    pub cn30xx: cvmx_mpi_datx_s,
    pub cn31xx: cvmx_mpi_datx_s,
    pub cn50xx: cvmx_mpi_datx_s,
    pub cn61xx: cvmx_mpi_datx_s,
    pub cn66xx: cvmx_mpi_datx_s,
    pub cnf71xx: cvmx_mpi_datx_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_mpi_sts {
    pub u64: u64,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_mpi_sts_s {

    pub reserved_13_63:51: u64,
    pub rxnum:5: u64,
    pub reserved_1_7:7: u64,
    pub busy:1: u64,

    pub busy:1: u64,
    pub reserved_1_7:7: u64,
    pub rxnum:5: u64,
    pub reserved_13_63:51: u64,

    pub s: },
    pub cn30xx: cvmx_mpi_sts_s,
    pub cn31xx: cvmx_mpi_sts_s,
    pub cn50xx: cvmx_mpi_sts_s,
    pub cn61xx: cvmx_mpi_sts_s,
    pub cn66xx: cvmx_mpi_sts_s,
    pub cnf71xx: cvmx_mpi_sts_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_mpi_tx {
    pub u64: u64,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_mpi_tx_s {

    pub reserved_22_63:42: u64,
    pub csid:2: u64,
    pub reserved_17_19:3: u64,
    pub leavecs:1: u64,
    pub reserved_13_15:3: u64,
    pub txnum:5: u64,
    pub reserved_5_7:3: u64,
    pub totnum:5: u64,

    pub totnum:5: u64,
    pub reserved_5_7:3: u64,
    pub txnum:5: u64,
    pub reserved_13_15:3: u64,
    pub leavecs:1: u64,
    pub reserved_17_19:3: u64,
    pub csid:2: u64,
    pub reserved_22_63:42: u64,

    pub s: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_mpi_tx_cn30xx {

    pub reserved_17_63:47: u64,
    pub leavecs:1: u64,
    pub reserved_13_15:3: u64,
    pub txnum:5: u64,
    pub reserved_5_7:3: u64,
    pub totnum:5: u64,

    pub totnum:5: u64,
    pub reserved_5_7:3: u64,
    pub txnum:5: u64,
    pub reserved_13_15:3: u64,
    pub leavecs:1: u64,
    pub reserved_17_63:47: u64,

    pub cn30xx: },
    pub cn31xx: cvmx_mpi_tx_cn30xx,
    pub cn50xx: cvmx_mpi_tx_cn30xx,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_mpi_tx_cn61xx {

    pub reserved_21_63:43: u64,
    pub csid:1: u64,
    pub reserved_17_19:3: u64,
    pub leavecs:1: u64,
    pub reserved_13_15:3: u64,
    pub txnum:5: u64,
    pub reserved_5_7:3: u64,
    pub totnum:5: u64,

    pub totnum:5: u64,
    pub reserved_5_7:3: u64,
    pub txnum:5: u64,
    pub reserved_13_15:3: u64,
    pub leavecs:1: u64,
    pub reserved_17_19:3: u64,
    pub csid:1: u64,
    pub reserved_21_63:43: u64,

    pub cn61xx: },
    pub cn66xx: cvmx_mpi_tx_s,
    pub cnf71xx: cvmx_mpi_tx_cn61xx,
}
