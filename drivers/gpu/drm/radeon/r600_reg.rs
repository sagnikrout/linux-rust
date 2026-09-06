//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/radeon/r600_reg.h
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
// Copyright 2008 Advanced Micro Devices, Inc.
// Copyright 2008 Red Hat Inc.
// Copyright 2009 Jerome Glisse.
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
// Authors: Dave Airlie
// Alex Deucher
// Jerome Glisse
//
pub const R600_PCIE_PORT_INDEX: c_uint = 0x0038;
pub const R600_PCIE_PORT_DATA: c_uint = 0x003c;
pub const R600_RCU_INDEX: c_uint = 0x0100;
pub const R600_RCU_DATA: c_uint = 0x0104;
pub const R600_UVD_CTX_INDEX: c_uint = 0xf4a0;
pub const R600_UVD_CTX_DATA: c_uint = 0xf4a4;
pub const R600_MC_VM_FB_LOCATION: c_uint = 0x2180;
pub const R600_MC_FB_BASE_MASK: c_uint = 0x0000FFFF;
pub const R600_MC_FB_BASE_SHIFT: c_int = 0;
pub const R600_MC_FB_TOP_MASK: c_uint = 0xFFFF0000;
pub const R600_MC_FB_TOP_SHIFT: c_int = 16;
pub const R600_MC_VM_AGP_TOP: c_uint = 0x2184;
pub const R600_MC_AGP_TOP_MASK: c_uint = 0x0003FFFF;
pub const R600_MC_AGP_TOP_SHIFT: c_int = 0;
pub const R600_MC_VM_AGP_BOT: c_uint = 0x2188;
pub const R600_MC_AGP_BOT_MASK: c_uint = 0x0003FFFF;
pub const R600_MC_AGP_BOT_SHIFT: c_int = 0;
pub const R600_MC_VM_AGP_BASE: c_uint = 0x218c;
pub const R600_MC_VM_SYSTEM_APERTURE_LOW_ADDR: c_uint = 0x2190;
pub const R600_LOGICAL_PAGE_NUMBER_MASK: c_uint = 0x000FFFFF;
pub const R600_LOGICAL_PAGE_NUMBER_SHIFT: c_int = 0;
pub const R600_MC_VM_SYSTEM_APERTURE_HIGH_ADDR: c_uint = 0x2194;
pub const R600_MC_VM_SYSTEM_APERTURE_DEFAULT_ADDR: c_uint = 0x2198;
pub const R700_MC_VM_FB_LOCATION: c_uint = 0x2024;
pub const R700_MC_FB_BASE_MASK: c_uint = 0x0000FFFF;
pub const R700_MC_FB_BASE_SHIFT: c_int = 0;
pub const R700_MC_FB_TOP_MASK: c_uint = 0xFFFF0000;
pub const R700_MC_FB_TOP_SHIFT: c_int = 16;
pub const R700_MC_VM_AGP_TOP: c_uint = 0x2028;
pub const R700_MC_AGP_TOP_MASK: c_uint = 0x0003FFFF;
pub const R700_MC_AGP_TOP_SHIFT: c_int = 0;
pub const R700_MC_VM_AGP_BOT: c_uint = 0x202c;
pub const R700_MC_AGP_BOT_MASK: c_uint = 0x0003FFFF;
pub const R700_MC_AGP_BOT_SHIFT: c_int = 0;
pub const R700_MC_VM_AGP_BASE: c_uint = 0x2030;
pub const R700_MC_VM_SYSTEM_APERTURE_LOW_ADDR: c_uint = 0x2034;
pub const R700_LOGICAL_PAGE_NUMBER_MASK: c_uint = 0x000FFFFF;
pub const R700_LOGICAL_PAGE_NUMBER_SHIFT: c_int = 0;
pub const R700_MC_VM_SYSTEM_APERTURE_HIGH_ADDR: c_uint = 0x2038;
pub const R700_MC_VM_SYSTEM_APERTURE_DEFAULT_ADDR: c_uint = 0x203c;
pub const R600_RAMCFG: c_uint = 0x2408;

pub const R600_GENERAL_PWRMGT: c_uint = 0x618;

pub const R600_LOWER_GPIO_ENABLE: c_uint = 0x710;
pub const R600_CTXSW_VID_LOWER_GPIO_CNTL: c_uint = 0x718;
pub const R600_HIGH_VID_LOWER_GPIO_CNTL: c_uint = 0x71c;
pub const R600_MEDIUM_VID_LOWER_GPIO_CNTL: c_uint = 0x720;
pub const R600_LOW_VID_LOWER_GPIO_CNTL: c_uint = 0x724;
pub const R600_D1GRPH_SWAP_CONTROL: c_uint = 0x610C;

pub const R600_HDP_NONSURFACE_BASE: c_uint = 0x2c04;
pub const R600_BUS_CNTL: c_uint = 0x5420;

pub const R600_CONFIG_CNTL: c_uint = 0x5424;
pub const R600_CONFIG_MEMSIZE: c_uint = 0x5428;
pub const R600_CONFIG_F0_BASE: c_uint = 0x542C;
pub const R600_CONFIG_APER_SIZE: c_uint = 0x5430;
pub const R600_BIF_FB_EN: c_uint = 0x5490;

pub const R600_CITF_CNTL: c_uint = 0x200c;
pub const R600_BLACKOUT_MASK: c_uint = 0x00000003;
pub const R700_MC_CITF_CNTL: c_uint = 0x25c0;
pub const R600_ROM_CNTL: c_uint = 0x1600;

pub const R600_CG_SPLL_FUNC_CNTL: c_uint = 0x600;

pub const R600_CG_SPLL_STATUS: c_uint = 0x60c;

pub const R600_BIOS_0_SCRATCH: c_uint = 0x1724;
pub const R600_BIOS_1_SCRATCH: c_uint = 0x1728;
pub const R600_BIOS_2_SCRATCH: c_uint = 0x172c;
pub const R600_BIOS_3_SCRATCH: c_uint = 0x1730;
pub const R600_BIOS_4_SCRATCH: c_uint = 0x1734;
pub const R600_BIOS_5_SCRATCH: c_uint = 0x1738;
pub const R600_BIOS_6_SCRATCH: c_uint = 0x173c;
pub const R600_BIOS_7_SCRATCH: c_uint = 0x1740;
// Audio, these regs were reverse enginered,
// so the chance is high that the naming is wrong
// R6xx+ ???
// Audio clocks
pub const R600_AUDIO_PLL1_MUL: c_uint = 0x0514;
pub const R600_AUDIO_PLL1_DIV: c_uint = 0x0518;
pub const R600_AUDIO_PLL2_MUL: c_uint = 0x0524;
pub const R600_AUDIO_PLL2_DIV: c_uint = 0x0528;
pub const R600_AUDIO_CLK_SRCSEL: c_uint = 0x0534;
// Audio general
pub const R600_AUDIO_ENABLE: c_uint = 0x7300;
pub const R600_AUDIO_TIMING: c_uint = 0x7344;
// Audio params
pub const R600_AUDIO_VENDOR_ID: c_uint = 0x7380;
pub const R600_AUDIO_REVISION_ID: c_uint = 0x7384;
pub const R600_AUDIO_ROOT_NODE_COUNT: c_uint = 0x7388;
pub const R600_AUDIO_NID1_NODE_COUNT: c_uint = 0x738c;
pub const R600_AUDIO_NID1_TYPE: c_uint = 0x7390;
pub const R600_AUDIO_SUPPORTED_SIZE_RATE: c_uint = 0x7394;
pub const R600_AUDIO_SUPPORTED_CODEC: c_uint = 0x7398;
pub const R600_AUDIO_SUPPORTED_POWER_STATES: c_uint = 0x739c;
pub const R600_AUDIO_NID2_CAPS: c_uint = 0x73a0;
pub const R600_AUDIO_NID3_CAPS: c_uint = 0x73a4;
pub const R600_AUDIO_NID3_PIN_CAPS: c_uint = 0x73a8;
// Audio conn list
pub const R600_AUDIO_CONN_LIST_LEN: c_uint = 0x73ac;
pub const R600_AUDIO_CONN_LIST: c_uint = 0x73b0;
// Audio verbs
pub const R600_AUDIO_RATE_BPS_CHANNEL: c_uint = 0x73c0;
pub const R600_AUDIO_PLAYING: c_uint = 0x73c4;
pub const R600_AUDIO_IMPLEMENTATION_ID: c_uint = 0x73c8;
pub const R600_AUDIO_CONFIG_DEFAULT: c_uint = 0x73cc;
pub const R600_AUDIO_PIN_SENSE: c_uint = 0x73d0;
pub const R600_AUDIO_PIN_WIDGET_CNTL: c_uint = 0x73d4;
pub const R600_AUDIO_STATUS_BITS: c_uint = 0x73d8;

// DCE3.2 second instance starts at 0x7800

