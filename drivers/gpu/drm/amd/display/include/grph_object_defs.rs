//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/include/grph_object_defs.h
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


//
// Copyright 2012-15 Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
// Authors: AMD
//

//
// These defines shared between All Graphics Objects
//

// HPD unit id - HW direct translation
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hpd_source_id {
    HPD_SOURCEID1 = 0,
    HPD_SOURCEID2,
    HPD_SOURCEID3,
    HPD_SOURCEID4,
    HPD_SOURCEID5,
    HPD_SOURCEID6,

    HPD_SOURCEID_COUNT,
    HPD_SOURCEID_UNKNOWN
}

// DDC unit id - HW direct translation
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum channel_id {
    CHANNEL_ID_UNKNOWN = 0,
    CHANNEL_ID_DDC1,
    CHANNEL_ID_DDC2,
    CHANNEL_ID_DDC3,
    CHANNEL_ID_DDC4,
    CHANNEL_ID_DDC5,
    CHANNEL_ID_DDC6,
    CHANNEL_ID_DDC_VGA,
    CHANNEL_ID_I2C_PAD,
    CHANNEL_ID_COUNT
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum transmitter {
    TRANSMITTER_UNKNOWN = (-1L),
    TRANSMITTER_UNIPHY_A,
    TRANSMITTER_UNIPHY_B,
    TRANSMITTER_UNIPHY_C,
    TRANSMITTER_UNIPHY_D,
    TRANSMITTER_UNIPHY_E,
    TRANSMITTER_UNIPHY_F,
    TRANSMITTER_NUTMEG_CRT,
    TRANSMITTER_TRAVIS_CRT,
    TRANSMITTER_TRAVIS_LCD,
    TRANSMITTER_UNIPHY_G,
    TRANSMITTER_COUNT
}

// Generic source of the synchronisation input/output signal
// Can be used for flow control, stereo sync, timing sync, frame sync, etc
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sync_source {
    SYNC_SOURCE_NONE = 0,

// Source based on controllers
    SYNC_SOURCE_CONTROLLER0,
    SYNC_SOURCE_CONTROLLER1,
    SYNC_SOURCE_CONTROLLER2,
    SYNC_SOURCE_CONTROLLER3,
    SYNC_SOURCE_CONTROLLER4,
    SYNC_SOURCE_CONTROLLER5,

// Source based on GSL group
    SYNC_SOURCE_GSL_GROUP0,
    SYNC_SOURCE_GSL_GROUP1,
    SYNC_SOURCE_GSL_GROUP2,

// Source based on GSL IOs
// These IOs normally used as GSL input/output
    SYNC_SOURCE_GSL_IO_FIRST,
    SYNC_SOURCE_GSL_IO_GENLOCK_CLOCK = SYNC_SOURCE_GSL_IO_FIRST,
    SYNC_SOURCE_GSL_IO_GENLOCK_VSYNC,
    SYNC_SOURCE_GSL_IO_SWAPLOCK_A,
    SYNC_SOURCE_GSL_IO_SWAPLOCK_B,
    SYNC_SOURCE_GSL_IO_LAST = SYNC_SOURCE_GSL_IO_SWAPLOCK_B,

// Source based on regular IOs
    SYNC_SOURCE_IO_FIRST,
    SYNC_SOURCE_IO_GENERIC_A = SYNC_SOURCE_IO_FIRST,
    SYNC_SOURCE_IO_GENERIC_B,
    SYNC_SOURCE_IO_GENERIC_C,
    SYNC_SOURCE_IO_GENERIC_D,
    SYNC_SOURCE_IO_GENERIC_E,
    SYNC_SOURCE_IO_GENERIC_F,
    SYNC_SOURCE_IO_HPD1,
    SYNC_SOURCE_IO_HPD2,
    SYNC_SOURCE_IO_HSYNC_A,
    SYNC_SOURCE_IO_VSYNC_A,
    SYNC_SOURCE_IO_HSYNC_B,
    SYNC_SOURCE_IO_VSYNC_B,
    SYNC_SOURCE_IO_LAST = SYNC_SOURCE_IO_VSYNC_B,

// Misc. flow control sources
    SYNC_SOURCE_DUAL_GPU_PIN
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tx_ffe_id {
    TX_FFE0 = 0,
    TX_FFE1,
    TX_FFE2,
    TX_FFE3,
    TX_FFE_DeEmphasis_Only,
    TX_FFE_PreShoot_Only,
    TX_FFE_No_FFE,
}

// connector sizes in millimeters - from BiosParserTypes.hpp
pub const CONNECTOR_SIZE_DVI: c_int = 40;
pub const CONNECTOR_SIZE_VGA: c_int = 32;
pub const CONNECTOR_SIZE_HDMI: c_int = 16;
pub const CONNECTOR_SIZE_DP: c_int = 16;
pub const CONNECTOR_SIZE_MINI_DP: c_int = 9;
pub const CONNECTOR_SIZE_UNKNOWN: c_int = 30;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum connector_layout_type {
    CONNECTOR_LAYOUT_TYPE_UNKNOWN,
    CONNECTOR_LAYOUT_TYPE_DVI_D,
    CONNECTOR_LAYOUT_TYPE_DVI_I,
    CONNECTOR_LAYOUT_TYPE_VGA,
    CONNECTOR_LAYOUT_TYPE_HDMI,
    CONNECTOR_LAYOUT_TYPE_DP,
    CONNECTOR_LAYOUT_TYPE_MINI_DP,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct connector_layout_info {
    pub connector_id: graphics_object_id,
    pub connector_type: connector_layout_type,
    pub length: c_uint,
    pub /: *mut *mut unsigned int position; / offset in mm from right side of the board,
}

// length and width in mm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct slot_layout_info {
    pub length: c_uint,
    pub width: c_uint,
    pub num_of_connectors: c_uint,
    pub connectors: [connector_layout_info; MAX_CONNECTOR_NUMBER_PER_SLOT],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct board_layout_info {
    pub num_of_slots: c_uint,
// indicates valid information in bracket layout structure.
    pub 1: unsigned int is_number_of_slots_valid :,
    pub 1: unsigned int is_slots_size_valid :,
    pub 1: unsigned int is_connector_offsets_valid :,
    pub 1: unsigned int is_connector_lengths_valid :,
    pub slots: [slot_layout_info; MAX_BOARD_SLOTS],
}
