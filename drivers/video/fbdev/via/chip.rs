//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/via/chip.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright 1998-2008 VIA Technologies, Inc. All Rights Reserved.
// Copyright 2001-2008 S3 Graphics, Inc. All Rights Reserved.
//

//
// Definition Graphic Chip Information
//
pub const PCI_VIA_VENDOR_ID: c_uint = 0x1106;
// Define VIA Graphic Chip Name
pub const UNICHROME_CLE266: c_int = 1;
pub const UNICHROME_CLE266_DID: c_uint = 0x3122;
pub const CLE266_REVISION_AX: c_uint = 0x0A;
pub const CLE266_REVISION_CX: c_uint = 0x0C;
pub const UNICHROME_K400: c_int = 2;
pub const UNICHROME_K400_DID: c_uint = 0x7205;
pub const UNICHROME_K800: c_int = 3;
pub const UNICHROME_K800_DID: c_uint = 0x3108;
pub const UNICHROME_PM800: c_int = 4;
pub const UNICHROME_PM800_DID: c_uint = 0x3118;
pub const UNICHROME_CN700: c_int = 5;
pub const UNICHROME_CN700_DID: c_uint = 0x3344;
pub const UNICHROME_CX700: c_int = 6;
pub const UNICHROME_CX700_DID: c_uint = 0x3157;
pub const CX700_REVISION_700: c_uint = 0x0;
pub const CX700_REVISION_700M: c_uint = 0x1;
pub const CX700_REVISION_700M2: c_uint = 0x2;
pub const UNICHROME_CN750: c_int = 7;
pub const UNICHROME_CN750_DID: c_uint = 0x3225;
pub const UNICHROME_K8M890: c_int = 8;
pub const UNICHROME_K8M890_DID: c_uint = 0x3230;
pub const UNICHROME_P4M890: c_int = 9;
pub const UNICHROME_P4M890_DID: c_uint = 0x3343;
pub const UNICHROME_P4M900: c_int = 10;
pub const UNICHROME_P4M900_DID: c_uint = 0x3371;
pub const UNICHROME_VX800: c_int = 11;
pub const UNICHROME_VX800_DID: c_uint = 0x1122;
pub const UNICHROME_VX855: c_int = 12;
pub const UNICHROME_VX855_DID: c_uint = 0x5122;
pub const UNICHROME_VX900: c_int = 13;
pub const UNICHROME_VX900_DID: c_uint = 0x7122;
//
// Definition TMDS Trasmitter Information
//
// Definition TMDS Trasmitter Index
pub const NON_TMDS_TRANSMITTER: c_uint = 0x00;
pub const VT1632_TMDS: c_uint = 0x01;
pub const INTEGRATED_TMDS: c_uint = 0x42;
// Definition TMDS Trasmitter I2C Target Address
pub const VT1632_TMDS_I2C_ADDR: c_uint = 0x10;
//
// Definition LVDS Trasmitter Information
//
// Definition LVDS Trasmitter Index
pub const NON_LVDS_TRANSMITTER: c_uint = 0x00;
pub const VT1631_LVDS: c_uint = 0x01;
pub const VT1636_LVDS: c_uint = 0x0E;
pub const INTEGRATED_LVDS: c_uint = 0x41;
// Definition Digital Transmitter Mode
pub const TX_DATA_12_BITS: c_uint = 0x01;
pub const TX_DATA_24_BITS: c_uint = 0x02;
pub const TX_DATA_DDR_MODE: c_uint = 0x04;
pub const TX_DATA_SDR_MODE: c_uint = 0x08;
// Definition LVDS Trasmitter I2C Target Address
pub const VT1631_LVDS_I2C_ADDR: c_uint = 0x70;
pub const VT3271_LVDS_I2C_ADDR: c_uint = 0x80;
pub const VT1636_LVDS_I2C_ADDR: c_uint = 0x80;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tmds_chip_information {
    pub tmds_chip_name: c_int,
    pub tmds_chip_target_addr: c_int,
    pub output_interface: c_int,
    pub i2c_port: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lvds_chip_information {
    pub lvds_chip_name: c_int,
    pub lvds_chip_target_addr: c_int,
    pub output_interface: c_int,
    pub i2c_port: c_int,
}

// The type of 2D engine
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum via_2d_engine {
    VIA_2D_ENG_H2,
    VIA_2D_ENG_H5,
    VIA_2D_ENG_M1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct chip_information {
    pub gfx_chip_name: c_int,
    pub gfx_chip_revision: c_int,
    pub twod_engine: via_2d_engine,
    pub tmds_chip_info: tmds_chip_information,
    pub lvds_chip_info: lvds_chip_information,
    pub lvds_chip_info2: lvds_chip_information,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tmds_setting_information {
    pub iga_path: c_int,
    pub h_active: c_int,
    pub v_active: c_int,
    pub max_pixel_clock: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lvds_setting_information {
    pub iga_path: c_int,
    pub lcd_panel_hres: c_int,
    pub lcd_panel_vres: c_int,
    pub display_method: c_int,
    pub device_lcd_dualedge: c_int,
    pub LCDDithering: c_int,
    pub lcd_mode: c_int,
    pub /: *mut *mut u32 vclk; /panel mode clock value,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GFX_DPA_SETTING {
    pub ClkRangeIndex: c_int,
    pub /: *mut *mut u8 DVP0; / CR96[3:0],
    pub /: *mut *mut u8 DVP0DataDri_S1; / SR2A[5],
    pub /: *mut *mut u8 DVP0DataDri_S; / SR1B[1],
    pub /: *mut *mut u8 DVP0ClockDri_S1; / SR2A[4],
    pub /: *mut *mut u8 DVP0ClockDri_S; / SR1E[2],
    pub /: *mut *mut u8 DVP1; / CR9B[3:0],
    pub /: *mut *mut u8 DVP1Driving; / SR65[3:0], Data and Clock driving,
    pub /: *mut *mut u8 DFPHigh; / CR97[3:0],
    pub /: *mut *mut u8 DFPLow; / CR99[3:0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VT1636_DPA_SETTING {
    pub CLK_SEL_ST1: u8,
    pub CLK_SEL_ST2: u8,
}
