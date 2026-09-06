//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/adi-axi-common.h
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
// Analog Devices AXI common registers & definitions
//
// Copyright 2019 Analog Devices Inc.
//
// https://wiki.analog.com/resources/fpga/docs/axi_ip
// https://wiki.analog.com/resources/fpga/docs/hdl/regmap
//

pub const ADI_AXI_REG_VERSION: c_uint = 0x0000;
pub const ADI_AXI_REG_FPGA_INFO: c_uint = 0x001C;

//
// adi_axi_pcore_ver_gteq() - check if a version is satisfied
// @version: the full version read from the hardware
// @major: the major version to compare against
// @minor: the minor version to compare against
//
// ADI AXI IP Cores use semantic versioning, so this can be used to check for
// feature availability.
//
// Return: true if the version is greater than or equal to the specified
// major and minor version, false otherwise.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum adi_axi_fpga_technology {
    ADI_AXI_FPGA_TECH_UNKNOWN = 0,
    ADI_AXI_FPGA_TECH_SERIES7,
    ADI_AXI_FPGA_TECH_ULTRASCALE,
    ADI_AXI_FPGA_TECH_ULTRASCALE_PLUS,
    ADI_AXI_FPGA_TECH_VERSAL,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum adi_axi_fpga_family {
    ADI_AXI_FPGA_FAMILY_UNKNOWN = 0,
    ADI_AXI_FPGA_FAMILY_ARTIX,
    ADI_AXI_FPGA_FAMILY_KINTEX,
    ADI_AXI_FPGA_FAMILY_VIRTEX,
    ADI_AXI_FPGA_FAMILY_ZYNQ,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum adi_axi_fpga_speed_grade {
    ADI_AXI_FPGA_SPEED_UNKNOWN      = 0,
    ADI_AXI_FPGA_SPEED_1    = 10,
    ADI_AXI_FPGA_SPEED_1L   = 11,
    ADI_AXI_FPGA_SPEED_1H   = 12,
    ADI_AXI_FPGA_SPEED_1HV  = 13,
    ADI_AXI_FPGA_SPEED_1LV  = 14,
    ADI_AXI_FPGA_SPEED_2    = 20,
    ADI_AXI_FPGA_SPEED_2L   = 21,
    ADI_AXI_FPGA_SPEED_2LV  = 22,
    ADI_AXI_FPGA_SPEED_2MP  = 23,
    ADI_AXI_FPGA_SPEED_3    = 30,
}
