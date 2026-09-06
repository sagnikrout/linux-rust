//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/radeon/pptable.h
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
// Copyright 2013 Advanced Micro Devices, Inc.
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

pub const ATOM_PP_FANPARAMETERS_TACHOMETER_PULSES_PER_REVOLUTION_MASK: c_uint = 0x0f;
pub const ATOM_PP_FANPARAMETERS_NOFAN: c_uint = 0x80    // No fan is connected to this controller.;
pub const ATOM_PP_THERMALCONTROLLER_NONE: c_int = 0;

pub const ATOM_PP_THERMALCONTROLLER_LM64: c_int = 5;

pub const ATOM_PP_THERMALCONTROLLER_RV6xx: c_int = 7;
pub const ATOM_PP_THERMALCONTROLLER_RV770: c_int = 8;
pub const ATOM_PP_THERMALCONTROLLER_ADT7473: c_int = 9;
pub const ATOM_PP_THERMALCONTROLLER_KONG: c_int = 10;
pub const ATOM_PP_THERMALCONTROLLER_EXTERNAL_GPIO: c_int = 11;
pub const ATOM_PP_THERMALCONTROLLER_EVERGREEN: c_int = 12;

pub const ATOM_PP_THERMALCONTROLLER_NISLANDS: c_int = 15;
pub const ATOM_PP_THERMALCONTROLLER_SISLANDS: c_int = 16;
pub const ATOM_PP_THERMALCONTROLLER_LM96163: c_int = 17;
pub const ATOM_PP_THERMALCONTROLLER_CISLANDS: c_int = 18;
pub const ATOM_PP_THERMALCONTROLLER_KAVERI: c_int = 19;
// Thermal controller 'combo type' to use an external controller for Fan control and an internal controller for thermal.
// We probably should reserve the bit 0x80 for this use.
// To keep the number of these types low we should also use the same code for all ASICs (i.e. do not distinguish RV6xx and RV7xx Internal here).
// The driver can pick the correct internal controller based on the ASIC.
pub const ATOM_PP_THERMALCONTROLLER_ADT7473_WITH_INTERNAL: c_uint = 0x89    // ADT7473 Fan Control + Internal Thermal Controller;
pub const ATOM_PP_THERMALCONTROLLER_EMC2103_WITH_INTERNAL: c_uint = 0x8D    // EMC2103 Fan Control + Internal Thermal Controller;
// Add extra system parameters here, always adjust size to include all fields.
// ATOM_PPLIB_POWERPLAYTABLE::ulPlatformCaps
pub const ATOM_PP_PLATFORM_CAP_BACKBIAS: c_int = 1;
pub const ATOM_PP_PLATFORM_CAP_POWERPLAY: c_int = 2;
pub const ATOM_PP_PLATFORM_CAP_SBIOSPOWERSOURCE: c_int = 4;
pub const ATOM_PP_PLATFORM_CAP_ASPM_L0s: c_int = 8;
pub const ATOM_PP_PLATFORM_CAP_ASPM_L1: c_int = 16;
pub const ATOM_PP_PLATFORM_CAP_HARDWAREDC: c_int = 32;
pub const ATOM_PP_PLATFORM_CAP_GEMINIPRIMARY: c_int = 64;
pub const ATOM_PP_PLATFORM_CAP_STEPVDDC: c_int = 128;
pub const ATOM_PP_PLATFORM_CAP_VOLTAGECONTROL: c_int = 256;
pub const ATOM_PP_PLATFORM_CAP_SIDEPORTCONTROL: c_int = 512;
pub const ATOM_PP_PLATFORM_CAP_TURNOFFPLL_ASPML1: c_int = 1024;
pub const ATOM_PP_PLATFORM_CAP_HTLINKCONTROL: c_int = 2048;
pub const ATOM_PP_PLATFORM_CAP_MVDDCONTROL: c_int = 4096;
pub const ATOM_PP_PLATFORM_CAP_GOTO_BOOT_ON_ALERT: c_uint = 0x2000              // Go to boot state on alerts, e.g. on an AC->DC transition.;
pub const ATOM_PP_PLATFORM_CAP_DONT_WAIT_FOR_VBLANK_ON_ALERT: c_uint = 0x4000   // Do NOT wait for VBLANK during an alert (e.g. AC->DC transition).;
pub const ATOM_PP_PLATFORM_CAP_VDDCI_CONTROL: c_uint = 0x8000                   // Does the driver control VDDCI independently from VDDC.;
pub const ATOM_PP_PLATFORM_CAP_REGULATOR_HOT: c_uint = 0x00010000               // Enable the 'regulator hot' feature.;
pub const ATOM_PP_PLATFORM_CAP_BACO: c_uint = 0x00020000               // Does the driver supports BACO state.;
pub const ATOM_PP_PLATFORM_CAP_NEW_CAC_VOLTAGE: c_uint = 0x00040000           // Does the driver supports new CAC voltage table.;
pub const ATOM_PP_PLATFORM_CAP_REVERT_GPIO5_POLARITY: c_uint = 0x00080000     // Does the driver supports revert GPIO5 polarity.;
pub const ATOM_PP_PLATFORM_CAP_OUTPUT_THERMAL2GPIO17: c_uint = 0x00100000     // Does the driver supports thermal2GPIO17.;
pub const ATOM_PP_PLATFORM_CAP_VRHOT_GPIO_CONFIGURABLE: c_uint = 0x00200000   // Does the driver supports VR HOT GPIO Configurable.;
pub const ATOM_PP_PLATFORM_CAP_TEMP_INVERSION: c_uint = 0x00400000            // Does the driver supports Temp Inversion feature.;
pub const ATOM_PP_PLATFORM_CAP_EVV: c_uint = 0x00800000;
// offset from start of this table to array of ucNumStates ATOM_PPLIB_STATE structures
// offset from start of this table to array of ASIC-specific structures,
// currently ATOM_PPLIB_CLOCK_INFO.
// offset from start of this table to array of ATOM_PPLIB_NONCLOCK_INFO
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
pub const ATOM_PPLIB_CLASSIFICATION_3DPERFORMANCE: c_uint = 0x0100;
pub const ATOM_PPLIB_CLASSIFICATION_OVERDRIVETEMPLATE: c_uint = 0x0200;
pub const ATOM_PPLIB_CLASSIFICATION_UVDSTATE: c_uint = 0x0400;
pub const ATOM_PPLIB_CLASSIFICATION_3DLOW: c_uint = 0x0800;
pub const ATOM_PPLIB_CLASSIFICATION_ACPI: c_uint = 0x1000;
pub const ATOM_PPLIB_CLASSIFICATION_HD2STATE: c_uint = 0x2000;
pub const ATOM_PPLIB_CLASSIFICATION_HDSTATE: c_uint = 0x4000;
pub const ATOM_PPLIB_CLASSIFICATION_SDSTATE: c_uint = 0x8000;
// ATOM_PPLIB_NONCLOCK_INFO::usClassification2
pub const ATOM_PPLIB_CLASSIFICATION2_LIMITEDPOWERSOURCE_2: c_uint = 0x0001;
pub const ATOM_PPLIB_CLASSIFICATION2_ULV: c_uint = 0x0002;
pub const ATOM_PPLIB_CLASSIFICATION2_MVC: c_uint = 0x0004   //Multi-View Codec (BD-3D);
// ATOM_PPLIB_NONCLOCK_INFO::ulCapsAndSettings
pub const ATOM_PPLIB_SINGLE_DISPLAY_ONLY: c_uint = 0x00000001;
pub const ATOM_PPLIB_SUPPORTS_VIDEO_PLAYBACK: c_uint = 0x00000002;
// 0 is 2.5Gb/s, 1 is 5Gb/s
pub const ATOM_PPLIB_PCIE_LINK_SPEED_MASK: c_uint = 0x00000004;
pub const ATOM_PPLIB_PCIE_LINK_SPEED_SHIFT: c_int = 2;
// lanes - 1: 1, 2, 4, 8, 12, 16 permitted by PCIE spec
pub const ATOM_PPLIB_PCIE_LINK_WIDTH_MASK: c_uint = 0x000000F8;
pub const ATOM_PPLIB_PCIE_LINK_WIDTH_SHIFT: c_int = 3;
// lookup into reduced refresh-rate table
pub const ATOM_PPLIB_LIMITED_REFRESHRATE_VALUE_MASK: c_uint = 0x00000F00;
pub const ATOM_PPLIB_LIMITED_REFRESHRATE_VALUE_SHIFT: c_int = 8;
pub const ATOM_PPLIB_LIMITED_REFRESHRATE_UNLIMITED: c_int = 0;
pub const ATOM_PPLIB_LIMITED_REFRESHRATE_50HZ: c_int = 1;
// 2-15 TBD as needed.
pub const ATOM_PPLIB_SOFTWARE_DISABLE_LOADBALANCING: c_uint = 0x00001000;
pub const ATOM_PPLIB_SOFTWARE_ENABLE_SLEEP_FOR_TIMESTAMPS: c_uint = 0x00002000;
pub const ATOM_PPLIB_DISALLOW_ON_DC: c_uint = 0x00004000;
pub const ATOM_PPLIB_ENABLE_VARIBRIGHT: c_uint = 0x00008000;
// memory related flags
pub const ATOM_PPLIB_SWSTATE_MEMORY_DLL_OFF: c_uint = 0x000010000;
// M3 Arb    //2bits, current 3 sets of parameters in total
pub const ATOM_PPLIB_M3ARB_MASK: c_uint = 0x00060000;
pub const ATOM_PPLIB_M3ARB_SHIFT: c_int = 17;
pub const ATOM_PPLIB_ENABLE_DRR: c_uint = 0x00080000;
// remaining 16 bits are reserved
// Contained in an array starting at the offset
// in ATOM_PPLIB_POWERPLAYTABLE::usNonClockInfoArrayOffset.
// referenced from ATOM_PPLIB_STATE_INFO::ucNonClockStateIndex
pub const ATOM_PPLIB_NONCLOCKINFO_VER1: c_int = 12;
pub const ATOM_PPLIB_NONCLOCKINFO_VER2: c_int = 24;
// Contained in an array starting at the offset
// in ATOM_PPLIB_POWERPLAYTABLE::usClockInfoArrayOffset.
// referenced from ATOM_PPLIB_STATE::ucClockStateIndices
// ulFlags in ATOM_PPLIB_R600_CLOCK_INFO
pub const ATOM_PPLIB_R600_FLAGS_PCIEGEN2: c_int = 1;
pub const ATOM_PPLIB_R600_FLAGS_UVDSAFE: c_int = 2;
pub const ATOM_PPLIB_R600_FLAGS_BACKBIASENABLE: c_int = 4;
pub const ATOM_PPLIB_R600_FLAGS_MEMORY_ODT_OFF: c_int = 8;
pub const ATOM_PPLIB_R600_FLAGS_MEMORY_DLL_OFF: c_int = 16;

pub const ATOM_PPLIB_RS780_VOLTAGE_NONE: c_int = 0;
pub const ATOM_PPLIB_RS780_VOLTAGE_LOW: c_int = 1;
pub const ATOM_PPLIB_RS780_VOLTAGE_HIGH: c_int = 2;
pub const ATOM_PPLIB_RS780_VOLTAGE_VARIABLE: c_int = 3;

pub const ATOM_PPLIB_RS780_SPMCLK_LOW: c_int = 1;
pub const ATOM_PPLIB_RS780_SPMCLK_HIGH: c_int = 2;
pub const ATOM_PPLIB_RS780_HTLINKFREQ_NONE: c_int = 0;
pub const ATOM_PPLIB_RS780_HTLINKFREQ_LOW: c_int = 1;
pub const ATOM_PPLIB_RS780_HTLINKFREQ_HIGH: c_int = 2;
// please initalize to 0
// please initialize to 0s
// number of valid dpm levels in this state; Driver uses it to calculate the whole
// size of the state: struct_size(ATOM_PPLIB_STATE_V2, clockInfoIndex, ucNumDPMLevels)
// a index to the array of nonClockInfos
//
// Driver will read the first ucNumDPMLevels in this array
//
// how many states we have
// how many clock levels we have
// sizeof(ATOM_PPLIB_CLOCK_INFO)
// how many non-clock levels we have. normally should be same as number of states
// sizeof(ATOM_PPLIB_NONCLOCK_INFO)
pub type ATOM_PPLIB_CAC_Leakage_Record = _ATOM_PPLIB_CAC_Leakage_Record;
// VCEClockInfoArray array;
// ATOM_PPLIB_VCE_Clock_Voltage_Limit_Table limits;
// ATOM_PPLIB_VCE_State_Table states;
// UVDClockInfoArray array;
// ATOM_PPLIB_UVD_Clock_Voltage_Limit_Table limits;
pub const ATOM_PPM_A_A: c_int = 1;
pub const ATOM_PPM_A_I: c_int = 2;

