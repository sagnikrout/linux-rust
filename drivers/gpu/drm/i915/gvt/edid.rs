//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gvt/edid.h
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
// Copyright(c) 2011-2016 Intel Corporation. All rights reserved.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice (including the next
// paragraph) shall be included in all copies or substantial portions of the
// Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//
// Authors:
// Ke Yu
// Zhiyuan Lv <zhiyuan.lv@intel.com>
//
// Contributors:
// Terrence Xu <terrence.xu@intel.com>
// Changbin Du <changbin.du@intel.com>
// Bing Niu <bing.niu@intel.com>
// Zhi Wang <zhi.a.wang@intel.com>
//

pub const EDID_SIZE: c_int = 128;
pub const EDID_ADDR: c_uint = 0x50 /* Linux hvm EDID addr */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_vgpu_edid_data {
    pub data_valid: bool,
    pub edid_block: [c_uchar; EDID_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gmbus_cycle_type {
    GMBUS_NOCYCLE	= 0x0,
    NIDX_NS_W	= 0x1,
    IDX_NS_W	= 0x3,
    GMBUS_STOP	= 0x4,
    NIDX_STOP	= 0x5,
    IDX_STOP	= 0x7
}

//
// States of GMBUS
//
// GMBUS0-3 could be related to the EDID virtualization. Another two GMBUS
// registers, GMBUS4 (interrupt mask) and GMBUS5 (2 byte indes register), are
// not considered here. Below describes the usage of GMBUS registers that are
// cared by the EDID virtualization
//
// GMBUS0:
// R/W
// port selection. value of bit0 - bit2 corresponds to the GPIO registers.
//
// GMBUS1:
// R/W Protect
// Command and Status.
// bit0 is the direction bit: 1 is read; 0 is write.
// bit1 - bit7 is target 7-bit address.
// bit16 - bit24 total byte count (ignore?)
//
// GMBUS2:
// Most of bits are read only except bit 15 (IN_USE)
// Status register
// bit0 - bit8 current byte count
// bit 11: hardware ready;
//
// GMBUS3:
// Read/Write
// Data for transfer
//
// From hw specs, Other phases like START, ADDRESS, INDEX
// are invisible to GMBUS MMIO interface. So no definitions
// in below enum types
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gvt_gmbus_phase {
    GMBUS_IDLE_PHASE = 0,
    GMBUS_DATA_PHASE,
    GMBUS_WAIT_PHASE,
// GMBUS_STOP_PHASE,
    GMBUS_MAX_PHASE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_vgpu_i2c_gmbus {
    pub /: *mut *mut unsigned int total_byte_count; / from GMBUS1,
    pub cycle_type: gmbus_cycle_type,
    pub phase: gvt_gmbus_phase,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_vgpu_i2c_aux_ch {
    pub i2c_over_aux_ch: bool,
    pub aux_ch_mot: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i2c_state {
    I2C_NOT_SPECIFIED = 0,
    I2C_GMBUS = 1,
    I2C_AUX_CH = 2
}

// I2C sequences cannot interleave.
// GMBUS and AUX_CH sequences cannot interleave.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_vgpu_i2c_edid {
    pub state: i2c_state,
    pub port: c_uint,
    pub target_selected: bool,
    pub edid_available: bool,
    pub current_edid_read: c_uint,
    pub gmbus: intel_vgpu_i2c_gmbus,
    pub aux_ch: intel_vgpu_i2c_aux_ch,
}

extern "C" {
    pub fn intel_vgpu_init_i2c_edid(vgpu: *mut intel_vgpu);
}
