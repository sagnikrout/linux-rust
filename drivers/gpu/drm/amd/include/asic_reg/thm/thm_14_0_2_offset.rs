//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/include/asic_reg/thm/thm_14_0_2_offset.h
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
// Copyright 2024 Advanced Micro Devices, Inc.
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

// Macro flag: #define _thm_14_0_2_OFFSET_HEADER
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
pub const regTHM_SW_TEMP: c_uint = 0x000d;
pub const regTHM_SW_TEMP_BASE_IDX: c_int = 0;
pub const regCG_MULT_THERMAL_CTRL: c_uint = 0x000e;
pub const regCG_MULT_THERMAL_CTRL_BASE_IDX: c_int = 0;
pub const regCG_MULT_THERMAL_STATUS: c_uint = 0x000f;
pub const regCG_MULT_THERMAL_STATUS_BASE_IDX: c_int = 0;
pub const regCG_THERMAL_RANGE: c_uint = 0x0010;
pub const regCG_THERMAL_RANGE_BASE_IDX: c_int = 0;
pub const regCG_FDO_CTRL0: c_uint = 0x0011;
pub const regCG_FDO_CTRL0_BASE_IDX: c_int = 0;
pub const regCG_FDO_CTRL1: c_uint = 0x0012;
pub const regCG_FDO_CTRL1_BASE_IDX: c_int = 0;
pub const regCG_FDO_CTRL2: c_uint = 0x0013;
pub const regCG_FDO_CTRL2_BASE_IDX: c_int = 0;
pub const regCG_TACH_CTRL: c_uint = 0x0014;
pub const regCG_TACH_CTRL_BASE_IDX: c_int = 0;
pub const regCG_TACH_STATUS: c_uint = 0x0015;
pub const regCG_TACH_STATUS_BASE_IDX: c_int = 0;
pub const regCG_THERMAL_STATUS: c_uint = 0x0016;
pub const regCG_THERMAL_STATUS_BASE_IDX: c_int = 0;
pub const regCG_PUMP_CTRL0: c_uint = 0x0017;
pub const regCG_PUMP_CTRL0_BASE_IDX: c_int = 0;
pub const regCG_PUMP_CTRL1: c_uint = 0x0018;
pub const regCG_PUMP_CTRL1_BASE_IDX: c_int = 0;
pub const regCG_PUMP_CTRL2: c_uint = 0x0019;
pub const regCG_PUMP_CTRL2_BASE_IDX: c_int = 0;
pub const regCG_PUMP_TACH_CTRL: c_uint = 0x001a;
pub const regCG_PUMP_TACH_CTRL_BASE_IDX: c_int = 0;
pub const regCG_PUMP_TACH_STATUS: c_uint = 0x001b;
pub const regCG_PUMP_TACH_STATUS_BASE_IDX: c_int = 0;
pub const regCG_PUMP_STATUS: c_uint = 0x001c;
pub const regCG_PUMP_STATUS_BASE_IDX: c_int = 0;
pub const regTHM_TCON_LOCAL2: c_uint = 0x001d;
pub const regTHM_TCON_LOCAL2_BASE_IDX: c_int = 0;
pub const regTHM_TCON_LOCAL3: c_uint = 0x001e;
pub const regTHM_TCON_LOCAL3_BASE_IDX: c_int = 0;
pub const regTHM_TCON_LOCAL4: c_uint = 0x001f;
pub const regTHM_TCON_LOCAL4_BASE_IDX: c_int = 0;
pub const regTHM_TCON_LOCAL5: c_uint = 0x0020;
pub const regTHM_TCON_LOCAL5_BASE_IDX: c_int = 0;
pub const regTHM_TCON_LOCAL6: c_uint = 0x0021;
pub const regTHM_TCON_LOCAL6_BASE_IDX: c_int = 0;
pub const regTHM_TCON_LOCAL7: c_uint = 0x0022;
pub const regTHM_TCON_LOCAL7_BASE_IDX: c_int = 0;
pub const regTHM_TCON_LOCAL8: c_uint = 0x0023;
pub const regTHM_TCON_LOCAL8_BASE_IDX: c_int = 0;
pub const regTHM_TCON_LOCAL9: c_uint = 0x0024;
pub const regTHM_TCON_LOCAL9_BASE_IDX: c_int = 0;
pub const regTHM_TCON_LOCAL10: c_uint = 0x0025;
pub const regTHM_TCON_LOCAL10_BASE_IDX: c_int = 0;
pub const regTHM_TCON_LOCAL11: c_uint = 0x0026;
pub const regTHM_TCON_LOCAL11_BASE_IDX: c_int = 0;
pub const regTHM_TCON_LOCAL12: c_uint = 0x0027;
pub const regTHM_TCON_LOCAL12_BASE_IDX: c_int = 0;
pub const regTHM_TCON_LOCAL13: c_uint = 0x0028;
pub const regTHM_TCON_LOCAL13_BASE_IDX: c_int = 0;
pub const regTHM_TCON_LOCAL14: c_uint = 0x0029;
pub const regTHM_TCON_LOCAL14_BASE_IDX: c_int = 0;
pub const regTHM_TCON_LOCAL15: c_uint = 0x002a;
pub const regTHM_TCON_LOCAL15_BASE_IDX: c_int = 0;
pub const regTHM_BACO_CNTL: c_uint = 0x002d;
pub const regTHM_BACO_CNTL_BASE_IDX: c_int = 0;
pub const regTHM_BACO_TIMING0: c_uint = 0x002e;
pub const regTHM_BACO_TIMING0_BASE_IDX: c_int = 0;
pub const regTHM_BACO_TIMING1: c_uint = 0x002f;
pub const regTHM_BACO_TIMING1_BASE_IDX: c_int = 0;
pub const regTHM_BACO_TIMING2: c_uint = 0x0030;
pub const regTHM_BACO_TIMING2_BASE_IDX: c_int = 0;
pub const regTHM_BACO_TIMING: c_uint = 0x0031;
pub const regTHM_BACO_TIMING_BASE_IDX: c_int = 0;
pub const regXTAL_CNTL: c_uint = 0x0032;
pub const regXTAL_CNTL_BASE_IDX: c_int = 0;
pub const regTHM_PWRMGT: c_uint = 0x0033;
pub const regTHM_PWRMGT_BASE_IDX: c_int = 0;
pub const regSMUSBI_SBIREGADDR: c_uint = 0x0158;
pub const regSMUSBI_SBIREGADDR_BASE_IDX: c_int = 0;
pub const regSMUSBI_SBIREGDATA: c_uint = 0x0159;
pub const regSMUSBI_SBIREGDATA_BASE_IDX: c_int = 0;
pub const regSMUSBI_ERRATA_STAT_REG: c_uint = 0x015d;
pub const regSMUSBI_ERRATA_STAT_REG_BASE_IDX: c_int = 0;
pub const regSMUSBI_SBICTRL: c_uint = 0x015e;
pub const regSMUSBI_SBICTRL_BASE_IDX: c_int = 0;
pub const regSMUSBI_CKNBIRESET: c_uint = 0x015f;
pub const regSMUSBI_CKNBIRESET_BASE_IDX: c_int = 0;
pub const regSMUSBI_TIMING: c_uint = 0x0160;
pub const regSMUSBI_TIMING_BASE_IDX: c_int = 0;
pub const regSMUSBI_HS_TIMING: c_uint = 0x0161;
pub const regSMUSBI_HS_TIMING_BASE_IDX: c_int = 0;
pub const regSBTSI_REMOTE_TEMP: c_uint = 0x0162;
pub const regSBTSI_REMOTE_TEMP_BASE_IDX: c_int = 0;
pub const regSBRMI_CONTROL: c_uint = 0x0163;
pub const regSBRMI_CONTROL_BASE_IDX: c_int = 0;
pub const regSBRMI_COMMAND: c_uint = 0x0164;
pub const regSBRMI_COMMAND_BASE_IDX: c_int = 0;
pub const regSBRMI_WRITE_DATA0: c_uint = 0x0166;
pub const regSBRMI_WRITE_DATA0_BASE_IDX: c_int = 0;
pub const regSBRMI_WRITE_DATA1: c_uint = 0x0167;
pub const regSBRMI_WRITE_DATA1_BASE_IDX: c_int = 0;
pub const regSBRMI_WRITE_DATA2: c_uint = 0x0168;
pub const regSBRMI_WRITE_DATA2_BASE_IDX: c_int = 0;
pub const regSBRMI_READ_DATA0: c_uint = 0x016a;
pub const regSBRMI_READ_DATA0_BASE_IDX: c_int = 0;
pub const regSBRMI_READ_DATA1: c_uint = 0x016b;
pub const regSBRMI_READ_DATA1_BASE_IDX: c_int = 0;
pub const regSBRMI_CORE_EN_NUMBER: c_uint = 0x016c;
pub const regSBRMI_CORE_EN_NUMBER_BASE_IDX: c_int = 0;
pub const regSBRMI_CORE_EN_STATUS0: c_uint = 0x016d;
pub const regSBRMI_CORE_EN_STATUS0_BASE_IDX: c_int = 0;
pub const regSBRMI_CORE_EN_STATUS1: c_uint = 0x016e;
pub const regSBRMI_CORE_EN_STATUS1_BASE_IDX: c_int = 0;
pub const regSBRMI_APIC_STATUS0: c_uint = 0x016f;
pub const regSBRMI_APIC_STATUS0_BASE_IDX: c_int = 0;
pub const regSBRMI_APIC_STATUS1: c_uint = 0x0170;
pub const regSBRMI_APIC_STATUS1_BASE_IDX: c_int = 0;
pub const regSBRMI_MCE_STATUS0: c_uint = 0x0171;
pub const regSBRMI_MCE_STATUS0_BASE_IDX: c_int = 0;
pub const regSBRMI_MCE_STATUS1: c_uint = 0x0172;
pub const regSBRMI_MCE_STATUS1_BASE_IDX: c_int = 0;
pub const regSMBUS_CNTL0: c_uint = 0x0173;
pub const regSMBUS_CNTL0_BASE_IDX: c_int = 0;
pub const regSMBUS_CNTL1: c_uint = 0x0174;
pub const regSMBUS_CNTL1_BASE_IDX: c_int = 0;
pub const regSMBUS_BLKWR_CMD_CTRL0: c_uint = 0x0175;
pub const regSMBUS_BLKWR_CMD_CTRL0_BASE_IDX: c_int = 0;
pub const regSMBUS_BLKWR_CMD_CTRL1: c_uint = 0x0176;
pub const regSMBUS_BLKWR_CMD_CTRL1_BASE_IDX: c_int = 0;
pub const regSMBUS_BLKRD_CMD_CTRL0: c_uint = 0x0177;
pub const regSMBUS_BLKRD_CMD_CTRL0_BASE_IDX: c_int = 0;
pub const regSMBUS_BLKRD_CMD_CTRL1: c_uint = 0x0178;
pub const regSMBUS_BLKRD_CMD_CTRL1_BASE_IDX: c_int = 0;
pub const regSMBUS_TIMING_CNTL0: c_uint = 0x0179;
pub const regSMBUS_TIMING_CNTL0_BASE_IDX: c_int = 0;
pub const regSMBUS_TIMING_CNTL1: c_uint = 0x017a;
pub const regSMBUS_TIMING_CNTL1_BASE_IDX: c_int = 0;
pub const regSMBUS_TIMING_CNTL2: c_uint = 0x017b;
pub const regSMBUS_TIMING_CNTL2_BASE_IDX: c_int = 0;
pub const regSMBUS_TRIGGER_CNTL: c_uint = 0x017c;
pub const regSMBUS_TRIGGER_CNTL_BASE_IDX: c_int = 0;
pub const regSMBUS_UDID_CNTL0: c_uint = 0x017d;
pub const regSMBUS_UDID_CNTL0_BASE_IDX: c_int = 0;
pub const regSMBUS_UDID_CNTL1: c_uint = 0x017e;
pub const regSMBUS_UDID_CNTL1_BASE_IDX: c_int = 0;
pub const regSMBUS_UDID_CNTL2: c_uint = 0x017f;
pub const regSMBUS_UDID_CNTL2_BASE_IDX: c_int = 0;
pub const regSMUSBI_SMBUS: c_uint = 0x0180;
pub const regSMUSBI_SMBUS_BASE_IDX: c_int = 0;
pub const regSMUSBI_ALERT: c_uint = 0x0181;
pub const regSMUSBI_ALERT_BASE_IDX: c_int = 0;
pub const regSMBUS_BACO_DUMMY: c_uint = 0x0182;
pub const regSMBUS_BACO_DUMMY_BASE_IDX: c_int = 0;
pub const regSMBUS_BACO_ADDR_RANGE0_LOW: c_uint = 0x0183;
pub const regSMBUS_BACO_ADDR_RANGE0_LOW_BASE_IDX: c_int = 0;
pub const regSMBUS_BACO_ADDR_RANGE0_HIGH: c_uint = 0x0184;
pub const regSMBUS_BACO_ADDR_RANGE0_HIGH_BASE_IDX: c_int = 0;
pub const regSMBUS_BACO_ADDR_RANGE1_LOW: c_uint = 0x0185;
pub const regSMBUS_BACO_ADDR_RANGE1_LOW_BASE_IDX: c_int = 0;
pub const regSMBUS_BACO_ADDR_RANGE1_HIGH: c_uint = 0x0186;
pub const regSMBUS_BACO_ADDR_RANGE1_HIGH_BASE_IDX: c_int = 0;
pub const regSMBUS_BACO_ADDR_RANGE2_LOW: c_uint = 0x0187;
pub const regSMBUS_BACO_ADDR_RANGE2_LOW_BASE_IDX: c_int = 0;
pub const regSMBUS_BACO_ADDR_RANGE2_HIGH: c_uint = 0x0188;
pub const regSMBUS_BACO_ADDR_RANGE2_HIGH_BASE_IDX: c_int = 0;
pub const regSMBUS_BACO_ADDR_RANGE3_LOW: c_uint = 0x0189;
pub const regSMBUS_BACO_ADDR_RANGE3_LOW_BASE_IDX: c_int = 0;
pub const regSMBUS_BACO_ADDR_RANGE3_HIGH: c_uint = 0x018a;
pub const regSMBUS_BACO_ADDR_RANGE3_HIGH_BASE_IDX: c_int = 0;
pub const regSMBUS_BACO_ADDR_RANGE4_LOW: c_uint = 0x018b;
pub const regSMBUS_BACO_ADDR_RANGE4_LOW_BASE_IDX: c_int = 0;
pub const regSMBUS_BACO_ADDR_RANGE4_HIGH: c_uint = 0x018c;
pub const regSMBUS_BACO_ADDR_RANGE4_HIGH_BASE_IDX: c_int = 0;
