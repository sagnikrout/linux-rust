//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/am437x-vpfe.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// Copyright (C) 2013 - 2014 Texas Instruments, Inc.
//
// Benoit Parrot <bparrot@ti.com>
// Lad, Prabhakar <prabhakar.csengg@gmail.com>
//
// This program is free software; you may redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; version 2 of the License.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vpfe_ccdc_data_size {
    VPFE_CCDC_DATA_16BITS = 0,
    VPFE_CCDC_DATA_15BITS,
    VPFE_CCDC_DATA_14BITS,
    VPFE_CCDC_DATA_13BITS,
    VPFE_CCDC_DATA_12BITS,
    VPFE_CCDC_DATA_11BITS,
    VPFE_CCDC_DATA_10BITS,
    VPFE_CCDC_DATA_8BITS,
}

// enum for No of pixel per line to be avg. in Black Clamping
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vpfe_ccdc_sample_length {
    VPFE_CCDC_SAMPLE_1PIXELS = 0,
    VPFE_CCDC_SAMPLE_2PIXELS,
    VPFE_CCDC_SAMPLE_4PIXELS,
    VPFE_CCDC_SAMPLE_8PIXELS,
    VPFE_CCDC_SAMPLE_16PIXELS,
}

// enum for No of lines in Black Clamping
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vpfe_ccdc_sample_line {
    VPFE_CCDC_SAMPLE_1LINES = 0,
    VPFE_CCDC_SAMPLE_2LINES,
    VPFE_CCDC_SAMPLE_4LINES,
    VPFE_CCDC_SAMPLE_8LINES,
    VPFE_CCDC_SAMPLE_16LINES,
}

// enum for Alaw gamma width
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vpfe_ccdc_gamma_width {
    VPFE_CCDC_GAMMA_BITS_15_6 = 0,	/* use bits 15-6 for gamma */
    VPFE_CCDC_GAMMA_BITS_14_5,
    VPFE_CCDC_GAMMA_BITS_13_4,
    VPFE_CCDC_GAMMA_BITS_12_3,
    VPFE_CCDC_GAMMA_BITS_11_2,
    VPFE_CCDC_GAMMA_BITS_10_1,
    VPFE_CCDC_GAMMA_BITS_09_0,	/* use bits 9-0 for gamma */
}

// structure for ALaw
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpfe_ccdc_a_law {
// Enable/disable A-Law
    pub enable: c_uchar,
// Gamma Width Input
    pub gamma_wd: vpfe_ccdc_gamma_width,
}

// structure for Black Clamping
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpfe_ccdc_black_clamp {
    pub enable: c_uchar,
// only if bClampEnable is TRUE
    pub sample_pixel: vpfe_ccdc_sample_length,
// only if bClampEnable is TRUE
    pub sample_ln: vpfe_ccdc_sample_line,
// only if bClampEnable is TRUE
    pub start_pixel: c_ushort,
// only if bClampEnable is TRUE
    pub sgain: c_ushort,
// only if bClampEnable is FALSE
    pub dc_sub: c_ushort,
}

// structure for Black Level Compensation
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpfe_ccdc_black_compensation {
// Constant value to subtract from Red component
    pub r: c_char,
// Constant value to subtract from Gr component
    pub gr: c_char,
// Constant value to subtract from Blue component
    pub b: c_char,
// Constant value to subtract from Gb component
    pub gb: c_char,
}

// Structure for CCDC configuration parameters for raw capture mode passed
// by application
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpfe_ccdc_config_params_raw {
// data size value from 8 to 16 bits
    pub data_sz: vpfe_ccdc_data_size,
// Structure for Optional A-Law
    pub alaw: vpfe_ccdc_a_law,
// Structure for Optical Black Clamp
    pub blk_clamp: vpfe_ccdc_black_clamp,
// Structure for Black Compensation
    pub blk_comp: vpfe_ccdc_black_compensation,
}

//
// Private IOCTL
// VIDIOC_AM437X_CCDC_CFG - Set CCDC configuration for raw capture
// This is an experimental ioctl that will change in future kernels. So use
// this ioctl with care !
//

