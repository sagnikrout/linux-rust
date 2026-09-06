//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/dma/ste_dma40.h
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
// Maximum size for a single dma descriptor
// Size is limited to 16 bits.
// Size is in the units of addr-widths (1,2,4,8 bytes)
// Larger transfers will be split up to multiple linked desc
//
pub const STEDMA40_MAX_SEG_SIZE: c_uint = 0xFFFF;
// dev types for memcpy

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stedma40_mode {
    STEDMA40_MODE_LOGICAL = 0,
    STEDMA40_MODE_PHYSICAL,
    STEDMA40_MODE_OPERATION,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stedma40_mode_opt {
    STEDMA40_PCHAN_BASIC_MODE = 0,
    STEDMA40_LCHAN_SRC_LOG_DST_LOG = 0,
    STEDMA40_PCHAN_MODULO_MODE,
    STEDMA40_PCHAN_DOUBLE_DST_MODE,
    STEDMA40_LCHAN_SRC_PHY_DST_LOG,
    STEDMA40_LCHAN_SRC_LOG_DST_PHY,
}

pub const STEDMA40_ESIZE_8_BIT: c_uint = 0x0;
pub const STEDMA40_ESIZE_16_BIT: c_uint = 0x1;
pub const STEDMA40_ESIZE_32_BIT: c_uint = 0x2;
pub const STEDMA40_ESIZE_64_BIT: c_uint = 0x3;
// The value 4 indicates that PEN-reg shall be set to 0
pub const STEDMA40_PSIZE_PHY_1: c_uint = 0x4;
pub const STEDMA40_PSIZE_PHY_2: c_uint = 0x0;
pub const STEDMA40_PSIZE_PHY_4: c_uint = 0x1;
pub const STEDMA40_PSIZE_PHY_8: c_uint = 0x2;
pub const STEDMA40_PSIZE_PHY_16: c_uint = 0x3;
//
// The number of elements differ in logical and
// physical mode
//

// Maximum number of possible physical channels
pub const STEDMA40_MAX_PHYS: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stedma40_flow_ctrl {
    STEDMA40_NO_FLOW_CTRL,
    STEDMA40_FLOW_CTRL,
}

//
// struct stedma40_half_channel_info - dst/src channel configuration
//
// @big_endian: true if the src/dst should be read as big endian
// @data_width: Data width of the src/dst hardware
// @p_size: Burst size
// @flow_ctrl: Flow control on/off.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stedma40_half_channel_info {
    pub big_endian: bool,
    pub data_width: dma_slave_buswidth,
    pub psize: c_int,
    pub flow_ctrl: stedma40_flow_ctrl,
}

//
// struct stedma40_chan_cfg - Structure to be filled by client drivers.
//
// @dir: MEM 2 MEM, PERIPH 2 MEM , MEM 2 PERIPH, PERIPH 2 PERIPH
// @high_priority: true if high-priority
// @realtime: true if realtime mode is to be enabled.  Only available on DMA40
// version 3+, i.e DB8500v2+
// @mode: channel mode: physical, logical, or operation
// @mode_opt: options for the chosen channel mode
// @dev_type: src/dst device type (driver uses dir to figure out which)
// @src_info: Parameters for dst half channel
// @dst_info: Parameters for dst half channel
// @use_fixed_channel: if true, use physical channel specified by phy_channel
// @phy_channel: physical channel to use, only if use_fixed_channel is true
//
// This structure has to be filled by the client drivers.
// It is recommended to do all dma configurations for clients in the machine.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stedma40_chan_cfg {
    pub dir: dma_transfer_direction,
    pub high_priority: bool,
    pub realtime: bool,
    pub mode: stedma40_mode,
    pub mode_opt: stedma40_mode_opt,
    pub dev_type: c_int,
    pub src_info: stedma40_half_channel_info,
    pub dst_info: stedma40_half_channel_info,
    pub use_fixed_channel: bool,
    pub phy_channel: c_int,
}
