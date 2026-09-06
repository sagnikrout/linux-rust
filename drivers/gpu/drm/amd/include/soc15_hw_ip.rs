//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/include/soc15_hw_ip.h
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
// Copyright (C) 2018  Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included
// in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
// OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN
// AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//

// Macro flag: #define _soc15_hw_ip_HEADER
// HW ID
pub const MP1_HWID: c_int = 1;
pub const MP2_HWID: c_int = 2;
pub const THM_HWID: c_int = 3;
pub const SMUIO_HWID: c_int = 4;
pub const FUSE_HWID: c_int = 5;
pub const CLKA_HWID: c_int = 6;
pub const PWR_HWID: c_int = 10;
pub const GC_HWID: c_int = 11;
pub const UVD_HWID: c_int = 12;

pub const AUDIO_AZ_HWID: c_int = 13;
pub const ACP_HWID: c_int = 14;
pub const DCI_HWID: c_int = 15;
pub const DMU_HWID: c_int = 271;
pub const DCO_HWID: c_int = 16;
pub const DIO_HWID: c_int = 272;
pub const XDMA_HWID: c_int = 17;
pub const DCEAZ_HWID: c_int = 18;
pub const DAZ_HWID: c_int = 274;
pub const SDPMUX_HWID: c_int = 19;
pub const NTB_HWID: c_int = 20;
pub const VPE_HWID: c_int = 21;
pub const UMSCH_HWID: c_int = 22;
pub const IOHC_HWID: c_int = 24;
pub const L2IMU_HWID: c_int = 28;
pub const VCE_HWID: c_int = 32;
pub const MMHUB_HWID: c_int = 34;
pub const ATHUB_HWID: c_int = 35;
pub const DBGU_NBIO_HWID: c_int = 36;
pub const DFX_HWID: c_int = 37;
pub const DBGU0_HWID: c_int = 38;
pub const DBGU1_HWID: c_int = 39;
pub const OSSSYS_HWID: c_int = 40;
pub const HDP_HWID: c_int = 41;
pub const SDMA0_HWID: c_int = 42;
pub const SDMA1_HWID: c_int = 43;
pub const ISP_HWID: c_int = 44;
pub const DBGU_IO_HWID: c_int = 45;
pub const DF_HWID: c_int = 46;
pub const CLKB_HWID: c_int = 47;
pub const FCH_HWID: c_int = 48;
pub const DFX_DAP_HWID: c_int = 49;
pub const L1IMU_PCIE_HWID: c_int = 50;
pub const L1IMU_NBIF_HWID: c_int = 51;
pub const L1IMU_IOAGR_HWID: c_int = 52;
pub const L1IMU3_HWID: c_int = 53;
pub const L1IMU4_HWID: c_int = 54;
pub const L1IMU5_HWID: c_int = 55;
pub const L1IMU6_HWID: c_int = 56;
pub const L1IMU7_HWID: c_int = 57;
pub const L1IMU8_HWID: c_int = 58;
pub const L1IMU9_HWID: c_int = 59;
pub const L1IMU10_HWID: c_int = 60;
pub const L1IMU11_HWID: c_int = 61;
pub const L1IMU12_HWID: c_int = 62;
pub const L1IMU13_HWID: c_int = 63;
pub const L1IMU14_HWID: c_int = 64;
pub const L1IMU15_HWID: c_int = 65;
pub const WAFLC_HWID: c_int = 66;
pub const FCH_USB_PD_HWID: c_int = 67;
pub const SDMA2_HWID: c_int = 68;
pub const SDMA3_HWID: c_int = 69;
pub const PCIE_HWID: c_int = 70;
pub const PCS_HWID: c_int = 80;
pub const DDCL_HWID: c_int = 89;
pub const SST_HWID: c_int = 90;
pub const LSDMA_HWID: c_int = 91;
pub const IOAGR_HWID: c_int = 100;
pub const NBIF_HWID: c_int = 108;
pub const IOAPIC_HWID: c_int = 124;
pub const SYSTEMHUB_HWID: c_int = 128;
pub const NTBCCP_HWID: c_int = 144;
pub const UMC_HWID: c_int = 150;
pub const SATA_HWID: c_int = 168;
pub const USB_HWID: c_int = 170;
pub const CCXSEC_HWID: c_int = 176;
pub const XGMI_HWID: c_int = 200;
pub const XGBE_HWID: c_int = 216;
pub const MP0_HWID: c_int = 255;
pub const ATU_HWID: c_int = 294;
pub const AIGC_HWID: c_int = 295;
