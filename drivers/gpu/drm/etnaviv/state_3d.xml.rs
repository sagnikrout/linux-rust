//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/etnaviv/state_3d.xml.h
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

// Macro flag: #define STATE_3D_XML
// This is a cut-down version of the state_3d.xml.h file
pub const VIVS_CL_CONFIG: c_uint = 0x00000900;
pub const VIVS_CL_CONFIG_DIMENSIONS__MASK: c_uint = 0x00000003;
pub const VIVS_CL_CONFIG_DIMENSIONS__SHIFT: c_int = 0;

pub const VIVS_CL_CONFIG_TRAVERSE_ORDER__MASK: c_uint = 0x00000070;
pub const VIVS_CL_CONFIG_TRAVERSE_ORDER__SHIFT: c_int = 4;

pub const VIVS_CL_CONFIG_ENABLE_SWATH_X: c_uint = 0x00000100;
pub const VIVS_CL_CONFIG_ENABLE_SWATH_Y: c_uint = 0x00000200;
pub const VIVS_CL_CONFIG_ENABLE_SWATH_Z: c_uint = 0x00000400;
pub const VIVS_CL_CONFIG_SWATH_SIZE_X__MASK: c_uint = 0x0000f000;
pub const VIVS_CL_CONFIG_SWATH_SIZE_X__SHIFT: c_int = 12;

pub const VIVS_CL_CONFIG_SWATH_SIZE_Y__MASK: c_uint = 0x000f0000;
pub const VIVS_CL_CONFIG_SWATH_SIZE_Y__SHIFT: c_int = 16;

pub const VIVS_CL_CONFIG_SWATH_SIZE_Z__MASK: c_uint = 0x00f00000;
pub const VIVS_CL_CONFIG_SWATH_SIZE_Z__SHIFT: c_int = 20;

pub const VIVS_CL_CONFIG_DIMENSIONS__MASK: c_uint = 0x00000003;
pub const VIVS_CL_CONFIG_DIMENSIONS__SHIFT: c_int = 0;

pub const VIVS_CL_CONFIG_VALUE_ORDER__MASK: c_uint = 0x07000000;
pub const VIVS_CL_CONFIG_VALUE_ORDER__SHIFT: c_int = 24;

pub const VIVS_CL_GLOBAL_WORK_OFFSET_X: c_uint = 0x0000092c;
pub const VIVS_CL_GLOBAL_WORK_OFFSET_Y: c_uint = 0x00000934;
pub const VIVS_CL_GLOBAL_WORK_OFFSET_Z: c_uint = 0x0000093c;
pub const VIVS_CL_KICKER: c_uint = 0x00000920;
pub const VIVS_CL_THREAD_ALLOCATION: c_uint = 0x0000091c;
pub const VIVS_CL_UNK00924: c_uint = 0x00000924;
pub const VIVS_CL_WORKGROUP_COUNT_X: c_uint = 0x00000940;
pub const VIVS_CL_WORKGROUP_COUNT_Y: c_uint = 0x00000944;
pub const VIVS_CL_WORKGROUP_COUNT_Z: c_uint = 0x00000948;
pub const VIVS_CL_WORKGROUP_SIZE_X: c_uint = 0x0000094c;
pub const VIVS_CL_WORKGROUP_SIZE_Y: c_uint = 0x00000950;
pub const VIVS_CL_WORKGROUP_SIZE_Z: c_uint = 0x00000954;
pub const VIVS_CL_GLOBAL_SCALE_X: c_uint = 0x00000958;
pub const VIVS_CL_GLOBAL_SCALE_Y: c_uint = 0x0000095c;
pub const VIVS_CL_GLOBAL_SCALE_Z: c_uint = 0x00000960;
pub const VIVS_PA_VS_OUTPUT_COUNT: c_uint = 0x00000aa8;
pub const VIVS_PS_CONTROL_EXT: c_uint = 0x00001030;
pub const VIVS_PS_ICACHE_COUNT: c_uint = 0x00001094;
pub const VIVS_PS_ICACHE_PREFETCH: c_uint = 0x00001048;
pub const VIVS_PS_INPUT_COUNT: c_uint = 0x00001008;
pub const VIVS_PS_INPUT_COUNT_COUNT__MASK: c_uint = 0x0000001f;
pub const VIVS_PS_INPUT_COUNT_COUNT__SHIFT: c_int = 0;

pub const VIVS_PS_NEWRANGE_LOW: c_uint = 0x0000087c;
pub const VIVS_PS_NEWRANGE_HIGH: c_uint = 0x00001090;
pub const VIVS_PS_SAMPLER_BASE: c_uint = 0x00001058;
pub const VIVS_PS_UNIFORM_BASE: c_uint = 0x00001024;
pub const VIVS_PS_INST_ADDR: c_uint = 0x00001028;
pub const VIVS_PS_TEMP_REGISTER_CONTROL: c_uint = 0x0000100c;
pub const VIVS_PS_TEMP_REGISTER_CONTROL_NUM_TEMPS__MASK: c_uint = 0x0000003f;
pub const VIVS_PS_TEMP_REGISTER_CONTROL_NUM_TEMPS__SHIFT: c_int = 0;

pub const VIVS_PS_VARYING_NUM_COMPONENTS__ESIZE: c_uint = 0x00000004;
pub const VIVS_PS_VARYING_NUM_COMPONENTS__LEN: c_uint = 0x00000004;
pub const VIVS_SH_CONFIG: c_uint = 0x00015600;
pub const VIVS_SH_CONFIG_RTNE_ROUNDING: c_uint = 0x00000002;

pub const VIVS_SH_HALTI5_UNIFORMS__ESIZE: c_uint = 0x00000004;
pub const VIVS_SH_HALTI5_UNIFORMS__LEN: c_uint = 0x00000800;
pub const VIVS_VS_HALTI5_UNK008A0: c_uint = 0x000008a0;
pub const VIVS_VS_HALTI5_UNK008A0_A__MASK: c_uint = 0x0000003f;
pub const VIVS_VS_HALTI5_UNK008A0_A__SHIFT: c_int = 0;

pub const VIVS_VS_ICACHE_CONTROL: c_uint = 0x00000868;
pub const VIVS_VS_ICACHE_CONTROL_ENABLE: c_uint = 0x00000001;
pub const VIVS_VS_ICACHE_INVALIDATE: c_uint = 0x000008b0;
pub const VIVS_VS_OUTPUT_COUNT: c_uint = 0x00000804;
pub const VIVS_VS_OUTPUT_COUNT_COUNT__MASK: c_uint = 0x000000ff;
pub const VIVS_VS_OUTPUT_COUNT_COUNT__SHIFT: c_int = 0;

pub const VIVS_TS_FLUSH_CACHE: c_uint = 0x00001650;
pub const VIVS_TS_FLUSH_CACHE_FLUSH: c_uint = 0x00000001;
pub const VIVS_NTE_DESCRIPTOR_FLUSH: c_uint = 0x00014c44;
pub const VIVS_NTE_DESCRIPTOR_FLUSH_UNK28__MASK: c_uint = 0xf0000000;
pub const VIVS_NTE_DESCRIPTOR_FLUSH_UNK28__SHIFT: c_int = 28;

