//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/powerplay/hwmgr/pptable_v1_0.h
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
// Copyright 2015 Advanced Micro Devices, Inc.
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
// \file
// This is a PowerPlay table header file
//

pub const ATOM_TONGA_PP_FANPARAMETERS_TACHOMETER_PULSES_PER_REVOLUTION_MASK: c_uint = 0x0f;
pub const ATOM_TONGA_PP_FANPARAMETERS_NOFAN: c_uint = 0x80    /* No fan is connected to this controller. */;
pub const ATOM_TONGA_PP_THERMALCONTROLLER_NONE: c_int = 0;
pub const ATOM_TONGA_PP_THERMALCONTROLLER_LM96163: c_int = 17;
pub const ATOM_TONGA_PP_THERMALCONTROLLER_TONGA: c_int = 21;
pub const ATOM_TONGA_PP_THERMALCONTROLLER_FIJI: c_int = 22;
//
// Thermal controller 'combo type' to use an external controller for Fan control and an internal controller for thermal.
// We probably should reserve the bit 0x80 for this use.
// To keep the number of these types low we should also use the same code for all ASICs (i.e. do not distinguish RV6xx and RV7xx Internal here).
// The driver can pick the correct internal controller based on the ASIC.
//
pub const ATOM_TONGA_PP_THERMALCONTROLLER_ADT7473_WITH_INTERNAL: c_uint = 0x89    /* ADT7473 Fan Control + Internal Thermal Controller */;
pub const ATOM_TONGA_PP_THERMALCONTROLLER_EMC2103_WITH_INTERNAL: c_uint = 0x8D    /* EMC2103 Fan Control + Internal Thermal Controller */;
// ATOM_TONGA_POWERPLAYTABLE::ulPlatformCaps
pub const ATOM_TONGA_PP_PLATFORM_CAP_VDDGFX_CONTROL: c_uint = 0x1            /* This cap indicates whether vddgfx will be a separated power rail. */;
pub const ATOM_TONGA_PP_PLATFORM_CAP_POWERPLAY: c_uint = 0x2            /* This cap indicates whether this is a mobile part and CCC need to show Powerplay page. */;
pub const ATOM_TONGA_PP_PLATFORM_CAP_SBIOSPOWERSOURCE: c_uint = 0x4            /* This cap indicates whether power source notificaiton is done by SBIOS directly. */;
pub const ATOM_TONGA_PP_PLATFORM_CAP_DISABLE_VOLTAGE_ISLAND: c_uint = 0x8            /* Enable the option to overwrite voltage island feature to be disabled, regardless of VddGfx power rail support. */;
pub const ____RETIRE16____: c_uint = 0x10;
pub const ATOM_TONGA_PP_PLATFORM_CAP_HARDWAREDC: c_uint = 0x20            /* This cap indicates whether power source notificaiton is done by GPIO directly. */;
pub const ____RETIRE64____: c_uint = 0x40;
pub const ____RETIRE128____: c_uint = 0x80;
pub const ____RETIRE256____: c_uint = 0x100;
pub const ____RETIRE512____: c_uint = 0x200;
pub const ____RETIRE1024____: c_uint = 0x400;
pub const ____RETIRE2048____: c_uint = 0x800;
pub const ATOM_TONGA_PP_PLATFORM_CAP_MVDD_CONTROL: c_uint = 0x1000            /* This cap indicates dynamic MVDD is required. Uncheck to disable it. */;
pub const ____RETIRE2000____: c_uint = 0x2000;
pub const ____RETIRE4000____: c_uint = 0x4000;
pub const ATOM_TONGA_PP_PLATFORM_CAP_VDDCI_CONTROL: c_uint = 0x8000            /* This cap indicates dynamic VDDCI is required. Uncheck to disable it. */;
pub const ____RETIRE10000____: c_uint = 0x10000;
pub const ATOM_TONGA_PP_PLATFORM_CAP_BACO: c_uint = 0x20000            /* Enable to indicate the driver supports BACO state. */;
pub const ATOM_TONGA_PP_PLATFORM_CAP_OUTPUT_THERMAL2GPIO17: c_uint = 0x100000     /* Enable to indicate the driver supports thermal2GPIO17. */;
pub const ATOM_TONGA_PP_PLATFORM_COMBINE_PCC_WITH_THERMAL_SIGNAL: c_uint = 0x1000000     /* Enable to indicate if thermal and PCC are sharing the same GPIO */;
pub const ATOM_TONGA_PLATFORM_LOAD_POST_PRODUCTION_FIRMWARE: c_uint = 0x2000000;
// ATOM_PPLIB_NONCLOCK_INFO::usClassification
pub const ATOM_PPLIB_CLASSIFICATION_UI_MASK: c_uint = 0x0007;
pub const ATOM_PPLIB_CLASSIFICATION_UI_SHIFT: c_int = 0;
pub const ATOM_PPLIB_CLASSIFICATION_UI_NONE: c_int = 0;
pub const ATOM_PPLIB_CLASSIFICATION_UI_BATTERY: c_int = 1;
pub const ATOM_PPLIB_CLASSIFICATION_UI_BALANCED: c_int = 3;
pub const ATOM_PPLIB_CLASSIFICATION_UI_PERFORMANCE: c_int = 5;
// 2, 4, 6, 7 are reserved
pub const ATOM_PPLIB_CLASSIFICATION_BOOT: c_uint = 0x0008;
pub const ATOM_PPLIB_CLASSIFICATION_THERMAL: c_uint = 0x0010;
pub const ATOM_PPLIB_CLASSIFICATION_LIMITEDPOWERSOURCE: c_uint = 0x0020;
pub const ATOM_PPLIB_CLASSIFICATION_REST: c_uint = 0x0040;
pub const ATOM_PPLIB_CLASSIFICATION_FORCED: c_uint = 0x0080;
pub const ATOM_PPLIB_CLASSIFICATION_ACPI: c_uint = 0x1000;
// ATOM_PPLIB_NONCLOCK_INFO::usClassification2
pub const ATOM_PPLIB_CLASSIFICATION2_LIMITEDPOWERSOURCE_2: c_uint = 0x0001;
pub const ATOM_Tonga_DISALLOW_ON_DC: c_uint = 0x00004000;
pub const ATOM_Tonga_ENABLE_VARIBRIGHT: c_uint = 0x00008000;
pub const ATOM_Tonga_TABLE_REVISION_TONGA: c_int = 7;
pub const ATOM_PPM_A_A: c_int = 1;
pub const ATOM_PPM_A_I: c_int = 2;

