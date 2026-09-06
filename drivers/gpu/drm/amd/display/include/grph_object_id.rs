//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/include/grph_object_id.h
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
// Types of graphics objects
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum object_type {
    OBJECT_TYPE_UNKNOWN  = 0,

// Direct ATOM BIOS translation
    OBJECT_TYPE_GPU,
    OBJECT_TYPE_ENCODER,
    OBJECT_TYPE_CONNECTOR,
    OBJECT_TYPE_ROUTER,
    OBJECT_TYPE_GENERIC,

// Driver specific
    OBJECT_TYPE_AUDIO,
    OBJECT_TYPE_CONTROLLER,
    OBJECT_TYPE_CLOCK_SOURCE,
    OBJECT_TYPE_ENGINE,

    OBJECT_TYPE_COUNT
}

// Enumeration inside one type of graphics objects
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum object_enum_id {
    ENUM_ID_UNKNOWN = 0,
    ENUM_ID_1,
    ENUM_ID_2,
    ENUM_ID_3,
    ENUM_ID_4,
    ENUM_ID_5,
    ENUM_ID_6,
    ENUM_ID_7,

    ENUM_ID_COUNT
}

// Generic object ids
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum generic_id {
    GENERIC_ID_UNKNOWN = 0,
    GENERIC_ID_MXM_OPM,
    GENERIC_ID_GLSYNC,
    GENERIC_ID_STEREO,

    GENERIC_ID_COUNT
}

// Controller object ids
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum controller_id {
    CONTROLLER_ID_UNDEFINED = 0,
    CONTROLLER_ID_D0,
    CONTROLLER_ID_D1,
    CONTROLLER_ID_D2,
    CONTROLLER_ID_D3,
    CONTROLLER_ID_D4,
    CONTROLLER_ID_D5,
    CONTROLLER_ID_UNDERLAY0,
    CONTROLLER_ID_MAX = CONTROLLER_ID_UNDERLAY0
}

//
// ClockSource object ids.
// We maintain the order matching (more or less) ATOM BIOS
// to improve optimized acquire
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum clock_source_id {
    CLOCK_SOURCE_ID_UNDEFINED = 0,
    CLOCK_SOURCE_ID_PLL0,
    CLOCK_SOURCE_ID_PLL1,
    CLOCK_SOURCE_ID_PLL2,
    CLOCK_SOURCE_ID_EXTERNAL, /* ID (Phy) ref. clk. for DP */
    CLOCK_SOURCE_ID_DCPLL,
    CLOCK_SOURCE_ID_DFS,	/* DENTIST */
    CLOCK_SOURCE_ID_VCE,	/* VCE does not need a real PLL */
// Used to distinguish between programming pixel clock and ID (Phy) clock
    CLOCK_SOURCE_ID_DP_DTO,

    CLOCK_SOURCE_COMBO_PHY_PLL0, /*combo PHY PLL defines (DC 11.2 and up)*/
    CLOCK_SOURCE_COMBO_PHY_PLL1,
    CLOCK_SOURCE_COMBO_PHY_PLL2,
    CLOCK_SOURCE_COMBO_PHY_PLL3,
    CLOCK_SOURCE_COMBO_PHY_PLL4,
    CLOCK_SOURCE_COMBO_PHY_PLL5,
    CLOCK_SOURCE_COMBO_DISPLAY_PLL0
}

// Encoder object ids
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum encoder_id {
    ENCODER_ID_UNKNOWN = 0,

// Radeon Class Display Hardware
    ENCODER_ID_INTERNAL_LVDS,
    ENCODER_ID_INTERNAL_TMDS1,
    ENCODER_ID_INTERNAL_TMDS2,
    ENCODER_ID_INTERNAL_DAC1,
    ENCODER_ID_INTERNAL_DAC2,	/* TV/CV DAC */

// External Third Party Encoders
    ENCODER_ID_INTERNAL_LVTM1,	/* not used for Radeon */
    ENCODER_ID_INTERNAL_HDMI,

// Kaledisope (KLDSCP) Class Display Hardware
    ENCODER_ID_INTERNAL_KLDSCP_TMDS1,
    ENCODER_ID_INTERNAL_KLDSCP_DAC1,
    ENCODER_ID_INTERNAL_KLDSCP_DAC2,	/* Shared with CV/TV and CRT */
// External TMDS (dual link)
    ENCODER_ID_EXTERNAL_MVPU_FPGA,	/* MVPU FPGA chip */
    ENCODER_ID_INTERNAL_DDI,
    ENCODER_ID_INTERNAL_UNIPHY,
    ENCODER_ID_INTERNAL_KLDSCP_LVTMA,
    ENCODER_ID_INTERNAL_UNIPHY1,
    ENCODER_ID_INTERNAL_UNIPHY2,
    ENCODER_ID_EXTERNAL_NUTMEG,
    ENCODER_ID_EXTERNAL_TRAVIS,

    ENCODER_ID_INTERNAL_WIRELESS,	/* Internal wireless display encoder */
    ENCODER_ID_INTERNAL_UNIPHY3,
    ENCODER_ID_INTERNAL_VIRTUAL,
}

// Connector object ids
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum connector_id {
    CONNECTOR_ID_UNKNOWN = 0,
    CONNECTOR_ID_SINGLE_LINK_DVII = 1,
    CONNECTOR_ID_DUAL_LINK_DVII = 2,
    CONNECTOR_ID_SINGLE_LINK_DVID = 3,
    CONNECTOR_ID_DUAL_LINK_DVID = 4,
    CONNECTOR_ID_VGA = 5,
    CONNECTOR_ID_HDMI_TYPE_A = 12,
    CONNECTOR_ID_LVDS = 14,
    CONNECTOR_ID_PCIE = 16,
    CONNECTOR_ID_HARDCODE_DVI = 18,
    CONNECTOR_ID_DISPLAY_PORT = 19,
    CONNECTOR_ID_EDP = 20,
    CONNECTOR_ID_MXM = 21,
    CONNECTOR_ID_WIRELESS = 22,
    CONNECTOR_ID_MIRACAST = 23,
    CONNECTOR_ID_USBC = 24,

    CONNECTOR_ID_VIRTUAL = 100
}

// Audio object ids
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum audio_id {
    AUDIO_ID_UNKNOWN = 0,
    AUDIO_ID_INTERNAL_AZALIA
}

// Engine object ids
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum engine_id {
    ENGINE_ID_DIGA,
    ENGINE_ID_DIGB,
    ENGINE_ID_DIGC,
    ENGINE_ID_DIGD,
    ENGINE_ID_DIGE,
    ENGINE_ID_DIGF,
    ENGINE_ID_DIGG,
    ENGINE_ID_DACA,
    ENGINE_ID_DACB,
    ENGINE_ID_VCE,	/* wireless display pseudo-encoder */
    ENGINE_ID_HPO_0,
    ENGINE_ID_HPO_1,
    ENGINE_ID_HPO_DP_0,
    ENGINE_ID_HPO_DP_1,
    ENGINE_ID_HPO_DP_2,
    ENGINE_ID_HPO_DP_3,
    ENGINE_ID_VIRTUAL,

    ENGINE_ID_COUNT,
    ENGINE_ID_UNKNOWN = (-1L)
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum transmitter_color_depth {
    TRANSMITTER_COLOR_DEPTH_24 = 0,  /* 8  bits */
    TRANSMITTER_COLOR_DEPTH_30,      /* 10 bits */
    TRANSMITTER_COLOR_DEPTH_36,      /* 12 bits */
    TRANSMITTER_COLOR_DEPTH_48       /* 16 bits */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dp_alt_mode {
    DP_Alt_mode__Unknown = 0,
    DP_Alt_mode__Connect,
    DP_Alt_mode__NoConnect,
}

//
// graphics_object_id struct
//
// graphics_object_id is a very simple struct wrapping 32bit Graphics
// Object identication
//
// This struct should stay very simple
// No dependencies at all (no includes)
// No debug messages or asserts
// No #ifndef and preprocessor directives
// No grow in space (no more data member)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct graphics_object_id {
    pub id:8: u32,
    pub :4: object_enum_id enum_id,
    pub :4: object_type type,
    pub /: *mut *mut uint32_t reserved:16; / for padding. total size should be u32,
}

// some simple functions for convenient graphics_object_id handle
// Based on internal data members memory layout
