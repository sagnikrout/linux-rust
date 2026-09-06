//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/bcm-ns2.h
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
// BSD LICENSE
//
// Copyright(c) 2015 Broadcom Corporation.  All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
//
// * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
// * Redistributions in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in
// the documentation and/or other materials provided with the
// distribution.
// * Neither the name of Broadcom Corporation nor the names of its
// contributors may be used to endorse or promote products derived
// from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//
// GENPLL SCR clock channel ID
pub const BCM_NS2_GENPLL_SCR: c_int = 0;
pub const BCM_NS2_GENPLL_SCR_SCR_CLK: c_int = 1;
pub const BCM_NS2_GENPLL_SCR_FS_CLK: c_int = 2;
pub const BCM_NS2_GENPLL_SCR_AUDIO_CLK: c_int = 3;
pub const BCM_NS2_GENPLL_SCR_CH3_UNUSED: c_int = 4;
pub const BCM_NS2_GENPLL_SCR_CH4_UNUSED: c_int = 5;
pub const BCM_NS2_GENPLL_SCR_CH5_UNUSED: c_int = 6;
// GENPLL SW clock channel ID
pub const BCM_NS2_GENPLL_SW: c_int = 0;
pub const BCM_NS2_GENPLL_SW_RPE_CLK: c_int = 1;
pub const BCM_NS2_GENPLL_SW_250_CLK: c_int = 2;
pub const BCM_NS2_GENPLL_SW_NIC_CLK: c_int = 3;
pub const BCM_NS2_GENPLL_SW_CHIMP_CLK: c_int = 4;
pub const BCM_NS2_GENPLL_SW_PORT_CLK: c_int = 5;
pub const BCM_NS2_GENPLL_SW_SDIO_CLK: c_int = 6;
// LCPLL DDR clock channel ID
pub const BCM_NS2_LCPLL_DDR: c_int = 0;
pub const BCM_NS2_LCPLL_DDR_PCIE_SATA_USB_CLK: c_int = 1;
pub const BCM_NS2_LCPLL_DDR_DDR_CLK: c_int = 2;
pub const BCM_NS2_LCPLL_DDR_CH2_UNUSED: c_int = 3;
pub const BCM_NS2_LCPLL_DDR_CH3_UNUSED: c_int = 4;
pub const BCM_NS2_LCPLL_DDR_CH4_UNUSED: c_int = 5;
pub const BCM_NS2_LCPLL_DDR_CH5_UNUSED: c_int = 6;
// LCPLL PORTS clock channel ID
pub const BCM_NS2_LCPLL_PORTS: c_int = 0;
pub const BCM_NS2_LCPLL_PORTS_WAN_CLK: c_int = 1;
pub const BCM_NS2_LCPLL_PORTS_RGMII_CLK: c_int = 2;
pub const BCM_NS2_LCPLL_PORTS_CH2_UNUSED: c_int = 3;
pub const BCM_NS2_LCPLL_PORTS_CH3_UNUSED: c_int = 4;
pub const BCM_NS2_LCPLL_PORTS_CH4_UNUSED: c_int = 5;
pub const BCM_NS2_LCPLL_PORTS_CH5_UNUSED: c_int = 6;
