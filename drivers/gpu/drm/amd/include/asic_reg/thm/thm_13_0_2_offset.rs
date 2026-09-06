//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/include/asic_reg/thm/thm_13_0_2_offset.h
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
// Copyright 2020 Advanced Micro Devices, Inc.
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

// Macro flag: #define _thm_13_0_2_OFFSET_HEADER
// addressBlock: thm_thm_SmuThmDec
// base address: 0x59800
pub const regTHM_TCON_CUR_TMP: c_uint = 0x0000;
pub const regTHM_TCON_CUR_TMP_BASE_IDX: c_int = 0;
pub const regTHM_TCON_HTC: c_uint = 0x0001;
pub const regTHM_TCON_HTC_BASE_IDX: c_int = 0;
pub const regTHM_TCON_THERM_TRIP: c_uint = 0x0002;
pub const regTHM_TCON_THERM_TRIP_BASE_IDX: c_int = 0;
pub const regTHM_CTF_DELAY: c_uint = 0x0003;
pub const regTHM_CTF_DELAY_BASE_IDX: c_int = 0;
pub const regTHM_GPIO_PROCHOT_CTRL: c_uint = 0x0004;
pub const regTHM_GPIO_PROCHOT_CTRL_BASE_IDX: c_int = 0;
pub const regTHM_GPIO_THERMTRIP_CTRL: c_uint = 0x0005;
pub const regTHM_GPIO_THERMTRIP_CTRL_BASE_IDX: c_int = 0;
pub const regTHM_GPIO_PWM_CTRL: c_uint = 0x0006;
pub const regTHM_GPIO_PWM_CTRL_BASE_IDX: c_int = 0;
pub const regTHM_GPIO_TACHIN_CTRL: c_uint = 0x0007;
pub const regTHM_GPIO_TACHIN_CTRL_BASE_IDX: c_int = 0;
pub const regTHM_GPIO_PUMPOUT_CTRL: c_uint = 0x0008;
pub const regTHM_GPIO_PUMPOUT_CTRL_BASE_IDX: c_int = 0;
pub const regTHM_GPIO_PUMPIN_CTRL: c_uint = 0x0009;
pub const regTHM_GPIO_PUMPIN_CTRL_BASE_IDX: c_int = 0;
pub const regTHM_THERMAL_INT_ENA: c_uint = 0x000a;
pub const regTHM_THERMAL_INT_ENA_BASE_IDX: c_int = 0;
pub const regTHM_THERMAL_INT_CTRL: c_uint = 0x000b;
pub const regTHM_THERMAL_INT_CTRL_BASE_IDX: c_int = 0;
pub const regTHM_THERMAL_INT_STATUS: c_uint = 0x000c;
pub const regTHM_THERMAL_INT_STATUS_BASE_IDX: c_int = 0;
pub const regTHM_TMON0_RDIL0_DATA: c_uint = 0x000d;
pub const regTHM_TMON0_RDIL0_DATA_BASE_IDX: c_int = 0;
pub const regTHM_TMON0_RDIL1_DATA: c_uint = 0x000e;
pub const regTHM_TMON0_RDIL1_DATA_BASE_IDX: c_int = 0;
pub const regTHM_TMON0_RDIL2_DATA: c_uint = 0x000f;
pub const regTHM_TMON0_RDIL2_DATA_BASE_IDX: c_int = 0;
pub const regTHM_TMON0_RDIL3_DATA: c_uint = 0x0010;
pub const regTHM_TMON0_RDIL3_DATA_BASE_IDX: c_int = 0;
pub const regTHM_TMON0_RDIL4_DATA: c_uint = 0x0011;
pub const regTHM_TMON0_RDIL4_DATA_BASE_IDX: c_int = 0;
pub const regTHM_TMON0_RDIL5_DATA: c_uint = 0x0012;
pub const regTHM_TMON0_RDIL5_DATA_BASE_IDX: c_int = 0;
pub const regTHM_TMON0_RDIL6_DATA: c_uint = 0x0013;
pub const regTHM_TMON0_RDIL6_DATA_BASE_IDX: c_int = 0;
pub const regTHM_TMON0_RDIL7_DATA: c_uint = 0x0014;
pub const regTHM_TMON0_RDIL7_DATA_BASE_IDX: c_int = 0;
pub const regTHM_TMON0_RDIL8_DATA: c_uint = 0x0015;
pub const regTHM_TMON0_RDIL8_DATA_BASE_IDX: c_int = 0;
pub const regTHM_TMON0_RDIL9_DATA: c_uint = 0x0016;
pub const regTHM_TMON0_RDIL9_DATA_BASE_IDX: c_int = 0;
pub const regTHM_TMON0_RDIL10_DATA: c_uint = 0x0017;
pub const regTHM_TMON0_RDIL10_DATA_BASE_IDX: c_int = 0;
pub const regTHM_TMON0_RDIL11_DATA: c_uint = 0x0018;
pub const regTHM_TMON0_RDIL11_DATA_BASE_IDX: c_int = 0;
pub const regTHM_TMON0_RDIL12_DATA: c_uint = 0x0019;
pub const regTHM_TMON0_RDIL12_DATA_BASE_IDX: c_int = 0;
pub const regTHM_TMON0_RDIL13_DATA: c_uint = 0x001a;
pub const regTHM_TMON0_RDIL13_DATA_BASE_IDX: c_int = 0;
pub const regTHM_TMON0_RDIL14_DATA: c_uint = 0x001b;
pub const regTHM_TMON0_RDIL14_DATA_BASE_IDX: c_int = 0;
pub const regTHM_TMON0_RDIL15_DATA: c_uint = 0x001c;
pub const regTHM_TMON0_RDIL15_DATA_BASE_IDX: c_int = 0;
pub const regTHM_TMON0_RDIR0_DATA: c_uint = 0x001d;
pub const regTHM_TMON0_RDIR0_DATA_BASE_IDX: c_int = 0;
pub const regTHM_TMON0_RDIR1_DATA: c_uint = 0x001e;
pub const regTHM_TMON0_RDIR1_DATA_BASE_IDX: c_int = 0;
pub const regTHM_TMON0_RDIR2_DATA: c_uint = 0x001f;
pub const regTHM_TMON0_RDIR2_DATA_BASE_IDX: c_int = 0;
pub const regTHM_TMON0_RDIR3_DATA: c_uint = 0x0020;
pub const regTHM_TMON0_RDIR3_DATA_BASE_IDX: c_int = 0;
pub const regTHM_TMON0_RDIR4_DATA: c_uint = 0x0021;
pub const regTHM_TMON0_RDIR4_DATA_BASE_IDX: c_int = 0;
pub const regTHM_TMON0_RDIR5_DATA: c_uint = 0x0022;
pub const regTHM_TMON0_RDIR5_DATA_BASE_IDX: c_int = 0;
pub const regTHM_TMON0_RDIR6_DATA: c_uint = 0x0023;
pub const regTHM_TMON0_RDIR6_DATA_BASE_IDX: c_int = 0;
pub const regTHM_TMON0_RDIR7_DATA: c_uint = 0x0024;
pub const regTHM_TMON0_RDIR7_DATA_BASE_IDX: c_int = 0;
pub const regTHM_TMON0_RDIR8_DATA: c_uint = 0x0025;
pub const regTHM_TMON0_RDIR8_DATA_BASE_IDX: c_int = 0;
pub const regTHM_TMON0_RDIR9_DATA: c_uint = 0x0026;
pub const regTHM_TMON0_RDIR9_DATA_BASE_IDX: c_int = 0;
pub const regTHM_TMON0_RDIR10_DATA: c_uint = 0x0027;
pub const regTHM_TMON0_RDIR10_DATA_BASE_IDX: c_int = 0;
pub const regTHM_TMON0_RDIR11_DATA: c_uint = 0x0028;
pub const regTHM_TMON0_RDIR11_DATA_BASE_IDX: c_int = 0;
pub const regTHM_TMON0_RDIR12_DATA: c_uint = 0x0029;
pub const regTHM_TMON0_RDIR12_DATA_BASE_IDX: c_int = 0;
pub const regTHM_TMON0_RDIR13_DATA: c_uint = 0x002a;
pub const regTHM_TMON0_RDIR13_DATA_BASE_IDX: c_int = 0;
pub const regTHM_TMON0_RDIR14_DATA: c_uint = 0x002b;
pub const regTHM_TMON0_RDIR14_DATA_BASE_IDX: c_int = 0;
pub const regTHM_TMON0_RDIR15_DATA: c_uint = 0x002c;
pub const regTHM_TMON0_RDIR15_DATA_BASE_IDX: c_int = 0;
pub const regTHM_TMON0_INT_DATA: c_uint = 0x002d;
pub const regTHM_TMON0_INT_DATA_BASE_IDX: c_int = 0;
pub const regTHM_TMON0_CTRL: c_uint = 0x002e;
pub const regTHM_TMON0_CTRL_BASE_IDX: c_int = 0;
pub const regTHM_TMON0_CTRL2: c_uint = 0x002f;
pub const regTHM_TMON0_CTRL2_BASE_IDX: c_int = 0;
pub const regTHM_TMON1_RDIL0_DATA: c_uint = 0x0031;
pub const regTHM_TMON1_RDIL0_DATA_BASE_IDX: c_int = 0;
pub const regTHM_TMON1_RDIL1_DATA: c_uint = 0x0032;
pub const regTHM_TMON1_RDIL1_DATA_BASE_IDX: c_int = 0;
pub const regTHM_TMON1_RDIL2_DATA: c_uint = 0x0033;
pub const regTHM_TMON1_RDIL2_DATA_BASE_IDX: c_int = 0;
pub const regTHM_TMON1_RDIL3_DATA: c_uint = 0x0034;
pub const regTHM_TMON1_RDIL3_DATA_BASE_IDX: c_int = 0;
pub const regTHM_TMON1_RDIL4_DATA: c_uint = 0x0035;
pub const regTHM_TMON1_RDIL4_DATA_BASE_IDX: c_int = 0;
pub const regTHM_TMON1_RDIL5_DATA: c_uint = 0x0036;
pub const regTHM_TMON1_RDIL5_DATA_BASE_IDX: c_int = 0;
pub const regTHM_TMON1_RDIL6_DATA: c_uint = 0x0037;
pub const regTHM_TMON1_RDIL6_DATA_BASE_IDX: c_int = 0;
pub const regTHM_TMON1_RDIL7_DATA: c_uint = 0x0038;
pub const regTHM_TMON1_RDIL7_DATA_BASE_IDX: c_int = 0;
pub const regTHM_TMON1_RDIL8_DATA: c_uint = 0x0039;
pub const regTHM_TMON1_RDIL8_DATA_BASE_IDX: c_int = 0;
pub const regTHM_TMON1_RDIL9_DATA: c_uint = 0x003a;
pub const regTHM_TMON1_RDIL9_DATA_BASE_IDX: c_int = 0;
pub const regTHM_TMON1_RDIL10_DATA: c_uint = 0x003b;
pub const regTHM_TMON1_RDIL10_DATA_BASE_IDX: c_int = 0;
pub const regTHM_TMON1_RDIL11_DATA: c_uint = 0x003c;
pub const regTHM_TMON1_RDIL11_DATA_BASE_IDX: c_int = 0;
pub const regTHM_TMON1_RDIL12_DATA: c_uint = 0x003d;
pub const regTHM_TMON1_RDIL12_DATA_BASE_IDX: c_int = 0;
pub const regTHM_TMON1_RDIL13_DATA: c_uint = 0x003e;
pub const regTHM_TMON1_RDIL13_DATA_BASE_IDX: c_int = 0;
pub const regTHM_TMON1_RDIL14_DATA: c_uint = 0x003f;
pub const regTHM_TMON1_RDIL14_DATA_BASE_IDX: c_int = 0;
pub const regTHM_TMON1_RDIL15_DATA: c_uint = 0x0040;
pub const regTHM_TMON1_RDIL15_DATA_BASE_IDX: c_int = 0;
pub const regTHM_TMON1_RDIR0_DATA: c_uint = 0x0041;
pub const regTHM_TMON1_RDIR0_DATA_BASE_IDX: c_int = 0;
pub const regTHM_TMON1_RDIR1_DATA: c_uint = 0x0042;
pub const regTHM_TMON1_RDIR1_DATA_BASE_IDX: c_int = 0;
pub const regTHM_TMON1_RDIR2_DATA: c_uint = 0x0043;
pub const regTHM_TMON1_RDIR2_DATA_BASE_IDX: c_int = 0;
pub const regTHM_TMON1_RDIR3_DATA: c_uint = 0x0044;
pub const regTHM_TMON1_RDIR3_DATA_BASE_IDX: c_int = 0;
pub const regTHM_TMON1_RDIR4_DATA: c_uint = 0x0045;
pub const regTHM_TMON1_RDIR4_DATA_BASE_IDX: c_int = 0;
pub const regTHM_TMON1_RDIR5_DATA: c_uint = 0x0046;
pub const regTHM_TMON1_RDIR5_DATA_BASE_IDX: c_int = 0;
pub const regTHM_TMON1_RDIR6_DATA: c_uint = 0x0047;
pub const regTHM_TMON1_RDIR6_DATA_BASE_IDX: c_int = 0;
pub const regTHM_TMON1_RDIR7_DATA: c_uint = 0x0048;
pub const regTHM_TMON1_RDIR7_DATA_BASE_IDX: c_int = 0;
pub const regTHM_TMON1_RDIR8_DATA: c_uint = 0x0049;
pub const regTHM_TMON1_RDIR8_DATA_BASE_IDX: c_int = 0;
pub const regTHM_TMON1_RDIR9_DATA: c_uint = 0x004a;
pub const regTHM_TMON1_RDIR9_DATA_BASE_IDX: c_int = 0;
pub const regTHM_TMON1_RDIR10_DATA: c_uint = 0x004b;
pub const regTHM_TMON1_RDIR10_DATA_BASE_IDX: c_int = 0;
pub const regTHM_TMON1_RDIR11_DATA: c_uint = 0x004c;
pub const regTHM_TMON1_RDIR11_DATA_BASE_IDX: c_int = 0;
pub const regTHM_TMON1_RDIR12_DATA: c_uint = 0x004d;
pub const regTHM_TMON1_RDIR12_DATA_BASE_IDX: c_int = 0;
pub const regTHM_TMON1_RDIR13_DATA: c_uint = 0x004e;
pub const regTHM_TMON1_RDIR13_DATA_BASE_IDX: c_int = 0;
pub const regTHM_TMON1_RDIR14_DATA: c_uint = 0x004f;
pub const regTHM_TMON1_RDIR14_DATA_BASE_IDX: c_int = 0;
pub const regTHM_TMON1_RDIR15_DATA: c_uint = 0x0050;
pub const regTHM_TMON1_RDIR15_DATA_BASE_IDX: c_int = 0;
pub const regTHM_TMON1_INT_DATA: c_uint = 0x0051;
pub const regTHM_TMON1_INT_DATA_BASE_IDX: c_int = 0;
pub const regTHM_DIE1_TEMP: c_uint = 0x0079;
pub const regTHM_DIE1_TEMP_BASE_IDX: c_int = 0;
pub const regTHM_DIE2_TEMP: c_uint = 0x007a;
pub const regTHM_DIE2_TEMP_BASE_IDX: c_int = 0;
pub const regTHM_DIE3_TEMP: c_uint = 0x007b;
pub const regTHM_DIE3_TEMP_BASE_IDX: c_int = 0;
pub const regTHM_SW_TEMP: c_uint = 0x0081;
pub const regTHM_SW_TEMP_BASE_IDX: c_int = 0;
pub const regCG_MULT_THERMAL_CTRL: c_uint = 0x0082;
pub const regCG_MULT_THERMAL_CTRL_BASE_IDX: c_int = 0;
pub const regCG_MULT_THERMAL_STATUS: c_uint = 0x0083;
pub const regCG_MULT_THERMAL_STATUS_BASE_IDX: c_int = 0;
pub const regCG_THERMAL_RANGE: c_uint = 0x0084;
pub const regCG_THERMAL_RANGE_BASE_IDX: c_int = 0;
pub const regTHM_TMON_CONFIG: c_uint = 0x0085;
pub const regTHM_TMON_CONFIG_BASE_IDX: c_int = 0;
pub const regTHM_TMON_CONFIG2: c_uint = 0x0086;
pub const regTHM_TMON_CONFIG2_BASE_IDX: c_int = 0;
pub const regTHM_TMON0_COEFF: c_uint = 0x0087;
pub const regTHM_TMON0_COEFF_BASE_IDX: c_int = 0;
pub const regTHM_TMON1_COEFF: c_uint = 0x0088;
pub const regTHM_TMON1_COEFF_BASE_IDX: c_int = 0;
pub const regCG_FDO_CTRL0: c_uint = 0x008b;
pub const regCG_FDO_CTRL0_BASE_IDX: c_int = 0;
pub const regCG_FDO_CTRL1: c_uint = 0x008c;
pub const regCG_FDO_CTRL1_BASE_IDX: c_int = 0;
pub const regCG_FDO_CTRL2: c_uint = 0x008d;
pub const regCG_FDO_CTRL2_BASE_IDX: c_int = 0;
pub const regCG_TACH_CTRL: c_uint = 0x008e;
pub const regCG_TACH_CTRL_BASE_IDX: c_int = 0;
pub const regCG_TACH_STATUS: c_uint = 0x008f;
pub const regCG_TACH_STATUS_BASE_IDX: c_int = 0;
pub const regCG_THERMAL_STATUS: c_uint = 0x0090;
pub const regCG_THERMAL_STATUS_BASE_IDX: c_int = 0;
pub const regCG_PUMP_CTRL0: c_uint = 0x0091;
pub const regCG_PUMP_CTRL0_BASE_IDX: c_int = 0;
pub const regCG_PUMP_CTRL1: c_uint = 0x0092;
pub const regCG_PUMP_CTRL1_BASE_IDX: c_int = 0;
pub const regCG_PUMP_CTRL2: c_uint = 0x0093;
pub const regCG_PUMP_CTRL2_BASE_IDX: c_int = 0;
pub const regCG_PUMP_TACH_CTRL: c_uint = 0x0094;
pub const regCG_PUMP_TACH_CTRL_BASE_IDX: c_int = 0;
pub const regCG_PUMP_TACH_STATUS: c_uint = 0x0095;
pub const regCG_PUMP_TACH_STATUS_BASE_IDX: c_int = 0;
pub const regCG_PUMP_STATUS: c_uint = 0x0096;
pub const regCG_PUMP_STATUS_BASE_IDX: c_int = 0;
pub const regTHM_TCON_LOCAL0: c_uint = 0x0097;
pub const regTHM_TCON_LOCAL0_BASE_IDX: c_int = 0;
pub const regTHM_TCON_LOCAL1: c_uint = 0x0098;
pub const regTHM_TCON_LOCAL1_BASE_IDX: c_int = 0;
pub const regTHM_TCON_LOCAL2: c_uint = 0x0099;
pub const regTHM_TCON_LOCAL2_BASE_IDX: c_int = 0;
pub const regTHM_TCON_LOCAL3: c_uint = 0x009a;
pub const regTHM_TCON_LOCAL3_BASE_IDX: c_int = 0;
pub const regTHM_TCON_LOCAL4: c_uint = 0x009b;
pub const regTHM_TCON_LOCAL4_BASE_IDX: c_int = 0;
pub const regTHM_TCON_LOCAL5: c_uint = 0x009c;
pub const regTHM_TCON_LOCAL5_BASE_IDX: c_int = 0;
pub const regTHM_TCON_LOCAL6: c_uint = 0x009d;
pub const regTHM_TCON_LOCAL6_BASE_IDX: c_int = 0;
pub const regTHM_TCON_LOCAL7: c_uint = 0x009e;
pub const regTHM_TCON_LOCAL7_BASE_IDX: c_int = 0;
pub const regTHM_TCON_LOCAL8: c_uint = 0x009f;
pub const regTHM_TCON_LOCAL8_BASE_IDX: c_int = 0;
pub const regTHM_TCON_LOCAL9: c_uint = 0x00a0;
pub const regTHM_TCON_LOCAL9_BASE_IDX: c_int = 0;
pub const regTHM_TCON_LOCAL10: c_uint = 0x00a1;
pub const regTHM_TCON_LOCAL10_BASE_IDX: c_int = 0;
pub const regTHM_TCON_LOCAL11: c_uint = 0x00a2;
pub const regTHM_TCON_LOCAL11_BASE_IDX: c_int = 0;
pub const regTHM_TCON_LOCAL12: c_uint = 0x00a3;
pub const regTHM_TCON_LOCAL12_BASE_IDX: c_int = 0;
pub const regTHM_TCON_LOCAL14: c_uint = 0x00a4;
pub const regTHM_TCON_LOCAL14_BASE_IDX: c_int = 0;
pub const regTHM_TCON_LOCAL15: c_uint = 0x00a5;
pub const regTHM_TCON_LOCAL15_BASE_IDX: c_int = 0;
pub const regTHM_TCON_LOCAL13: c_uint = 0x00a6;
pub const regTHM_TCON_LOCAL13_BASE_IDX: c_int = 0;
pub const regXTAL_CNTL: c_uint = 0x00ac;
pub const regXTAL_CNTL_BASE_IDX: c_int = 0;
pub const regTHM_PWRMGT: c_uint = 0x00ad;
pub const regTHM_PWRMGT_BASE_IDX: c_int = 0;
pub const regTHM_GPIO_MACO_EN_CTRL: c_uint = 0x00ae;
pub const regTHM_GPIO_MACO_EN_CTRL_BASE_IDX: c_int = 0;
pub const regSBTSI_REMOTE_TEMP: c_uint = 0x00ca;
pub const regSBTSI_REMOTE_TEMP_BASE_IDX: c_int = 0;
pub const regSBRMI_CONTROL: c_uint = 0x00cb;
pub const regSBRMI_CONTROL_BASE_IDX: c_int = 0;
pub const regSBRMI_COMMAND: c_uint = 0x00cc;
pub const regSBRMI_COMMAND_BASE_IDX: c_int = 0;
pub const regSBRMI_WRITE_DATA0: c_uint = 0x00cd;
pub const regSBRMI_WRITE_DATA0_BASE_IDX: c_int = 0;
pub const regSBRMI_WRITE_DATA1: c_uint = 0x00ce;
pub const regSBRMI_WRITE_DATA1_BASE_IDX: c_int = 0;
pub const regSBRMI_WRITE_DATA2: c_uint = 0x00cf;
pub const regSBRMI_WRITE_DATA2_BASE_IDX: c_int = 0;
pub const regSBRMI_READ_DATA0: c_uint = 0x00d0;
pub const regSBRMI_READ_DATA0_BASE_IDX: c_int = 0;
pub const regSBRMI_READ_DATA1: c_uint = 0x00d1;
pub const regSBRMI_READ_DATA1_BASE_IDX: c_int = 0;
pub const regSBRMI_CORE_EN_NUMBER: c_uint = 0x00d2;
pub const regSBRMI_CORE_EN_NUMBER_BASE_IDX: c_int = 0;
pub const regSBRMI_CORE_EN_STATUS0: c_uint = 0x00d3;
pub const regSBRMI_CORE_EN_STATUS0_BASE_IDX: c_int = 0;
pub const regSBRMI_CORE_EN_STATUS1: c_uint = 0x00d4;
pub const regSBRMI_CORE_EN_STATUS1_BASE_IDX: c_int = 0;
pub const regSBRMI_APIC_STATUS0: c_uint = 0x00d5;
pub const regSBRMI_APIC_STATUS0_BASE_IDX: c_int = 0;
pub const regSBRMI_APIC_STATUS1: c_uint = 0x00d6;
pub const regSBRMI_APIC_STATUS1_BASE_IDX: c_int = 0;
pub const regSBRMI_MCE_STATUS0: c_uint = 0x00db;
pub const regSBRMI_MCE_STATUS0_BASE_IDX: c_int = 0;
pub const regSBRMI_MCE_STATUS1: c_uint = 0x00dc;
pub const regSBRMI_MCE_STATUS1_BASE_IDX: c_int = 0;
pub const regSMBUS_CNTL0: c_uint = 0x00df;
pub const regSMBUS_CNTL0_BASE_IDX: c_int = 0;
pub const regSMBUS_CNTL1: c_uint = 0x00e0;
pub const regSMBUS_CNTL1_BASE_IDX: c_int = 0;
pub const regSMBUS_BLKWR_CMD_CTRL0: c_uint = 0x00e1;
pub const regSMBUS_BLKWR_CMD_CTRL0_BASE_IDX: c_int = 0;
pub const regSMBUS_BLKWR_CMD_CTRL1: c_uint = 0x00e2;
pub const regSMBUS_BLKWR_CMD_CTRL1_BASE_IDX: c_int = 0;
pub const regSMBUS_BLKRD_CMD_CTRL0: c_uint = 0x00e3;
pub const regSMBUS_BLKRD_CMD_CTRL0_BASE_IDX: c_int = 0;
pub const regSMBUS_BLKRD_CMD_CTRL1: c_uint = 0x00e4;
pub const regSMBUS_BLKRD_CMD_CTRL1_BASE_IDX: c_int = 0;
pub const regSMBUS_TIMING_CNTL0: c_uint = 0x00e5;
pub const regSMBUS_TIMING_CNTL0_BASE_IDX: c_int = 0;
pub const regSMBUS_TIMING_CNTL1: c_uint = 0x00e6;
pub const regSMBUS_TIMING_CNTL1_BASE_IDX: c_int = 0;
pub const regSMBUS_TIMING_CNTL2: c_uint = 0x00e7;
pub const regSMBUS_TIMING_CNTL2_BASE_IDX: c_int = 0;
pub const regSMBUS_TRIGGER_CNTL: c_uint = 0x00e8;
pub const regSMBUS_TRIGGER_CNTL_BASE_IDX: c_int = 0;
pub const regSMBUS_UDID_CNTL0: c_uint = 0x00e9;
pub const regSMBUS_UDID_CNTL0_BASE_IDX: c_int = 0;
pub const regSMBUS_UDID_CNTL1: c_uint = 0x00ea;
pub const regSMBUS_UDID_CNTL1_BASE_IDX: c_int = 0;
pub const regSMBUS_UDID_CNTL2: c_uint = 0x00eb;
pub const regSMBUS_UDID_CNTL2_BASE_IDX: c_int = 0;
pub const regTHM_TMON0_REMOTE_START: c_uint = 0x0100;
pub const regTHM_TMON0_REMOTE_START_BASE_IDX: c_int = 0;
pub const regTHM_TMON0_REMOTE_END: c_uint = 0x013f;
pub const regTHM_TMON0_REMOTE_END_BASE_IDX: c_int = 0;
pub const regTHM_TMON1_REMOTE_START: c_uint = 0x0140;
pub const regTHM_TMON1_REMOTE_START_BASE_IDX: c_int = 0;
pub const regTHM_TMON1_REMOTE_END: c_uint = 0x017f;
pub const regTHM_TMON1_REMOTE_END_BASE_IDX: c_int = 0;
pub const regTHM_TMON2_REMOTE_START: c_uint = 0x0180;
pub const regTHM_TMON2_REMOTE_START_BASE_IDX: c_int = 0;
pub const regTHM_TMON2_REMOTE_END: c_uint = 0x01bf;
pub const regTHM_TMON2_REMOTE_END_BASE_IDX: c_int = 0;
