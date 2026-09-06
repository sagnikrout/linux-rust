//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/ObjectID.h
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
// Copyright 2006-2007 Advanced Micro Devices, Inc.
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
// based on stg/asic_reg/drivers/inc/asic_reg/ObjectID.h ver 23

//
// Graphics Object Type Definition
//
pub const GRAPH_OBJECT_TYPE_NONE: c_uint = 0x0;
pub const GRAPH_OBJECT_TYPE_GPU: c_uint = 0x1;
pub const GRAPH_OBJECT_TYPE_ENCODER: c_uint = 0x2;
pub const GRAPH_OBJECT_TYPE_CONNECTOR: c_uint = 0x3;
pub const GRAPH_OBJECT_TYPE_ROUTER: c_uint = 0x4;
// deleted
pub const GRAPH_OBJECT_TYPE_DISPLAY_PATH: c_uint = 0x6;
pub const GRAPH_OBJECT_TYPE_GENERIC: c_uint = 0x7;
//
// Encoder Object ID Definition
//
pub const ENCODER_OBJECT_ID_NONE: c_uint = 0x00;
// Radeon Class Display Hardware
pub const ENCODER_OBJECT_ID_INTERNAL_LVDS: c_uint = 0x01;
pub const ENCODER_OBJECT_ID_INTERNAL_TMDS1: c_uint = 0x02;
pub const ENCODER_OBJECT_ID_INTERNAL_TMDS2: c_uint = 0x03;
pub const ENCODER_OBJECT_ID_INTERNAL_DAC1: c_uint = 0x04;
pub const ENCODER_OBJECT_ID_INTERNAL_DAC2: c_uint = 0x05     /* TV/CV DAC */;
pub const ENCODER_OBJECT_ID_INTERNAL_SDVOA: c_uint = 0x06;
pub const ENCODER_OBJECT_ID_INTERNAL_SDVOB: c_uint = 0x07;
// External Third Party Encoders
pub const ENCODER_OBJECT_ID_SI170B: c_uint = 0x08;
pub const ENCODER_OBJECT_ID_CH7303: c_uint = 0x09;
pub const ENCODER_OBJECT_ID_CH7301: c_uint = 0x0A;
pub const ENCODER_OBJECT_ID_INTERNAL_DVO1: c_uint = 0x0B    /* This belongs to Radeon Class Display Hardware */;
pub const ENCODER_OBJECT_ID_EXTERNAL_SDVOA: c_uint = 0x0C;
pub const ENCODER_OBJECT_ID_EXTERNAL_SDVOB: c_uint = 0x0D;
pub const ENCODER_OBJECT_ID_TITFP513: c_uint = 0x0E;
pub const ENCODER_OBJECT_ID_INTERNAL_LVTM1: c_uint = 0x0F    /* not used for Radeon */;
pub const ENCODER_OBJECT_ID_VT1623: c_uint = 0x10;
pub const ENCODER_OBJECT_ID_HDMI_SI1930: c_uint = 0x11;
pub const ENCODER_OBJECT_ID_HDMI_INTERNAL: c_uint = 0x12;
pub const ENCODER_OBJECT_ID_ALMOND: c_uint = 0x22;
pub const ENCODER_OBJECT_ID_TRAVIS: c_uint = 0x23;
pub const ENCODER_OBJECT_ID_NUTMEG: c_uint = 0x22;
pub const ENCODER_OBJECT_ID_HDMI_ANX9805: c_uint = 0x26;
// Kaleidoscope (KLDSCP) Class Display Hardware (internal)
pub const ENCODER_OBJECT_ID_INTERNAL_KLDSCP_TMDS1: c_uint = 0x13;
pub const ENCODER_OBJECT_ID_INTERNAL_KLDSCP_DVO1: c_uint = 0x14;
pub const ENCODER_OBJECT_ID_INTERNAL_KLDSCP_DAC1: c_uint = 0x15;
pub const ENCODER_OBJECT_ID_INTERNAL_KLDSCP_DAC2: c_uint = 0x16  /* Shared with CV/TV and CRT */;

pub const ENCODER_OBJECT_ID_MVPU_FPGA: c_uint = 0x18  /* MVPU FPGA chip */;
pub const ENCODER_OBJECT_ID_INTERNAL_DDI: c_uint = 0x19;
pub const ENCODER_OBJECT_ID_VT1625: c_uint = 0x1A;
pub const ENCODER_OBJECT_ID_HDMI_SI1932: c_uint = 0x1B;
pub const ENCODER_OBJECT_ID_DP_AN9801: c_uint = 0x1C;
pub const ENCODER_OBJECT_ID_DP_DP501: c_uint = 0x1D;
pub const ENCODER_OBJECT_ID_INTERNAL_UNIPHY: c_uint = 0x1E;
pub const ENCODER_OBJECT_ID_INTERNAL_KLDSCP_LVTMA: c_uint = 0x1F;
pub const ENCODER_OBJECT_ID_INTERNAL_UNIPHY1: c_uint = 0x20;
pub const ENCODER_OBJECT_ID_INTERNAL_UNIPHY2: c_uint = 0x21;
pub const ENCODER_OBJECT_ID_INTERNAL_VCE: c_uint = 0x24;
pub const ENCODER_OBJECT_ID_INTERNAL_UNIPHY3: c_uint = 0x25;
pub const ENCODER_OBJECT_ID_INTERNAL_AMCLK: c_uint = 0x27;
pub const ENCODER_OBJECT_ID_GENERAL_EXTERNAL_DVO: c_uint = 0xFF;
//
// Connector Object ID Definition
//
pub const CONNECTOR_OBJECT_ID_NONE: c_uint = 0x00;
pub const CONNECTOR_OBJECT_ID_SINGLE_LINK_DVI_I: c_uint = 0x01;
pub const CONNECTOR_OBJECT_ID_DUAL_LINK_DVI_I: c_uint = 0x02;
pub const CONNECTOR_OBJECT_ID_SINGLE_LINK_DVI_D: c_uint = 0x03;
pub const CONNECTOR_OBJECT_ID_DUAL_LINK_DVI_D: c_uint = 0x04;
pub const CONNECTOR_OBJECT_ID_VGA: c_uint = 0x05;
pub const CONNECTOR_OBJECT_ID_COMPOSITE: c_uint = 0x06;
pub const CONNECTOR_OBJECT_ID_SVIDEO: c_uint = 0x07;
pub const CONNECTOR_OBJECT_ID_YPbPr: c_uint = 0x08;
pub const CONNECTOR_OBJECT_ID_D_CONNECTOR: c_uint = 0x09;
pub const CONNECTOR_OBJECT_ID_9PIN_DIN: c_uint = 0x0A  /* Supports both CV & TV */;
pub const CONNECTOR_OBJECT_ID_SCART: c_uint = 0x0B;
pub const CONNECTOR_OBJECT_ID_HDMI_TYPE_A: c_uint = 0x0C;
pub const CONNECTOR_OBJECT_ID_HDMI_TYPE_B: c_uint = 0x0D;
pub const CONNECTOR_OBJECT_ID_LVDS: c_uint = 0x0E;
pub const CONNECTOR_OBJECT_ID_7PIN_DIN: c_uint = 0x0F;
pub const CONNECTOR_OBJECT_ID_PCIE_CONNECTOR: c_uint = 0x10;
pub const CONNECTOR_OBJECT_ID_CROSSFIRE: c_uint = 0x11;
pub const CONNECTOR_OBJECT_ID_HARDCODE_DVI: c_uint = 0x12;
pub const CONNECTOR_OBJECT_ID_DISPLAYPORT: c_uint = 0x13;
pub const CONNECTOR_OBJECT_ID_eDP: c_uint = 0x14;
pub const CONNECTOR_OBJECT_ID_MXM: c_uint = 0x15;
pub const CONNECTOR_OBJECT_ID_LVDS_eDP: c_uint = 0x16;
pub const CONNECTOR_OBJECT_ID_USBC: c_uint = 0x17;
// deleted
//
// Router Object ID Definition
//
pub const ROUTER_OBJECT_ID_NONE: c_uint = 0x00;
pub const ROUTER_OBJECT_ID_I2C_EXTENDER_CNTL: c_uint = 0x01;
//
// Generic Object ID Definition
//
pub const GENERIC_OBJECT_ID_NONE: c_uint = 0x00;
pub const GENERIC_OBJECT_ID_GLSYNC: c_uint = 0x01;
pub const GENERIC_OBJECT_ID_PX2_NON_DRIVABLE: c_uint = 0x02;
pub const GENERIC_OBJECT_ID_MXM_OPM: c_uint = 0x03;
pub const GENERIC_OBJECT_ID_STEREO_PIN: c_uint = 0x04        //This object could show up from Misc Object table, it follows ATOM_OBJECT format, and contains one ATOM_OBJECT_GPIO_CNTL_RECORD for the stereo pin;
pub const GENERIC_OBJECT_ID_BRACKET_LAYOUT: c_uint = 0x05;
//
// Graphics Object ENUM ID Definition
//
pub const GRAPH_OBJECT_ENUM_ID1: c_uint = 0x01;
pub const GRAPH_OBJECT_ENUM_ID2: c_uint = 0x02;
pub const GRAPH_OBJECT_ENUM_ID3: c_uint = 0x03;
pub const GRAPH_OBJECT_ENUM_ID4: c_uint = 0x04;
pub const GRAPH_OBJECT_ENUM_ID5: c_uint = 0x05;
pub const GRAPH_OBJECT_ENUM_ID6: c_uint = 0x06;
pub const GRAPH_OBJECT_ENUM_ID7: c_uint = 0x07;
//
// Graphics Object ID Bit definition
//
pub const OBJECT_ID_MASK: c_uint = 0x00FF;
pub const ENUM_ID_MASK: c_uint = 0x0700;
pub const RESERVED1_ID_MASK: c_uint = 0x0800;
pub const OBJECT_TYPE_MASK: c_uint = 0x7000;
pub const RESERVED2_ID_MASK: c_uint = 0x8000;
pub const OBJECT_ID_SHIFT: c_uint = 0x00;
pub const ENUM_ID_SHIFT: c_uint = 0x08;
pub const OBJECT_TYPE_SHIFT: c_uint = 0x0C;
//
// Graphics Object family definition
//

//
// GPU Object ID definition - Shared with BIOS
//

//
// Encoder Object ID definition - Shared with BIOS
//
pub const ENCODER_INTERNAL_LVDS_ENUM_ID1: c_uint = 0x2101;
pub const ENCODER_INTERNAL_TMDS1_ENUM_ID1: c_uint = 0x2102;
pub const ENCODER_INTERNAL_TMDS2_ENUM_ID1: c_uint = 0x2103;
pub const ENCODER_INTERNAL_DAC1_ENUM_ID1: c_uint = 0x2104;
pub const ENCODER_INTERNAL_DAC2_ENUM_ID1: c_uint = 0x2105;
pub const ENCODER_INTERNAL_SDVOA_ENUM_ID1: c_uint = 0x2106;
pub const ENCODER_INTERNAL_SDVOB_ENUM_ID1: c_uint = 0x2107;
pub const ENCODER_SIL170B_ENUM_ID1: c_uint = 0x2108;
pub const ENCODER_CH7303_ENUM_ID1: c_uint = 0x2109;
pub const ENCODER_CH7301_ENUM_ID1: c_uint = 0x210A;
pub const ENCODER_INTERNAL_DVO1_ENUM_ID1: c_uint = 0x210B;
pub const ENCODER_EXTERNAL_SDVOA_ENUM_ID1: c_uint = 0x210C;
pub const ENCODER_EXTERNAL_SDVOB_ENUM_ID1: c_uint = 0x210D;
pub const ENCODER_TITFP513_ENUM_ID1: c_uint = 0x210E;
pub const ENCODER_INTERNAL_LVTM1_ENUM_ID1: c_uint = 0x210F;
pub const ENCODER_VT1623_ENUM_ID1: c_uint = 0x2110;
pub const ENCODER_HDMI_SI1930_ENUM_ID1: c_uint = 0x2111;
pub const ENCODER_HDMI_INTERNAL_ENUM_ID1: c_uint = 0x2112;
pub const ENCODER_INTERNAL_KLDSCP_TMDS1_ENUM_ID1: c_uint = 0x2113;
pub const ENCODER_INTERNAL_KLDSCP_DVO1_ENUM_ID1: c_uint = 0x2114;
pub const ENCODER_INTERNAL_KLDSCP_DAC1_ENUM_ID1: c_uint = 0x2115;
pub const ENCODER_INTERNAL_KLDSCP_DAC2_ENUM_ID1: c_uint = 0x2116;
pub const ENCODER_SI178_ENUM_ID1: c_uint = 0x2117;
pub const ENCODER_MVPU_FPGA_ENUM_ID1: c_uint = 0x2118;
pub const ENCODER_INTERNAL_DDI_ENUM_ID1: c_uint = 0x2119;
pub const ENCODER_VT1625_ENUM_ID1: c_uint = 0x211A;
pub const ENCODER_HDMI_SI1932_ENUM_ID1: c_uint = 0x211B;
pub const ENCODER_ENCODER_DP_AN9801_ENUM_ID1: c_uint = 0x211C;
pub const ENCODER_DP_DP501_ENUM_ID1: c_uint = 0x211D;
pub const ENCODER_INTERNAL_UNIPHY_ENUM_ID1: c_uint = 0x211E;
//

//
// Connector Object ID definition - Shared with BIOS
//
pub const CONNECTOR_SINGLE_LINK_DVI_I_ENUM_ID1: c_uint = 0x3101;
pub const CONNECTOR_DUAL_LINK_DVI_I_ENUM_ID1: c_uint = 0x3102;
pub const CONNECTOR_SINGLE_LINK_DVI_D_ENUM_ID1: c_uint = 0x3103;
pub const CONNECTOR_DUAL_LINK_DVI_D_ENUM_ID1: c_uint = 0x3104;
pub const CONNECTOR_VGA_ENUM_ID1: c_uint = 0x3105;
pub const CONNECTOR_COMPOSITE_ENUM_ID1: c_uint = 0x3106;
pub const CONNECTOR_SVIDEO_ENUM_ID1: c_uint = 0x3107;
pub const CONNECTOR_YPbPr_ENUM_ID1: c_uint = 0x3108;
pub const CONNECTOR_D_CONNECTORE_ENUM_ID1: c_uint = 0x3109;
pub const CONNECTOR_9PIN_DIN_ENUM_ID1: c_uint = 0x310A;
pub const CONNECTOR_SCART_ENUM_ID1: c_uint = 0x310B;
pub const CONNECTOR_HDMI_TYPE_A_ENUM_ID1: c_uint = 0x310C;
pub const CONNECTOR_HDMI_TYPE_B_ENUM_ID1: c_uint = 0x310D;
pub const CONNECTOR_LVDS_ENUM_ID1: c_uint = 0x310E;
pub const CONNECTOR_7PIN_DIN_ENUM_ID1: c_uint = 0x310F;
pub const CONNECTOR_PCIE_CONNECTOR_ENUM_ID1: c_uint = 0x3110;
//

//
// Router Object ID definition - Shared with BIOS
//

// deleted
//
// Generic Object ID definition - Shared with BIOS
//

//
// Object Cap definition - Shared with BIOS
//
pub const GRAPHICS_OBJECT_CAP_I2C: c_uint = 0x00000001L;
pub const GRAPHICS_OBJECT_CAP_TABLE_ID: c_uint = 0x00000002L;
pub const GRAPHICS_OBJECT_I2CCOMMAND_TABLE_ID: c_uint = 0x01;
pub const GRAPHICS_OBJECT_HOTPLUGDETECTIONINTERUPT_TABLE_ID: c_uint = 0x02;
pub const GRAPHICS_OBJECT_ENCODER_OUTPUT_PROTECTION_TABLE_ID: c_uint = 0x03;

