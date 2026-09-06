//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/boot/dts/ti/k3-serdes.h
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


// SPDX-License-Identifier: GPL-2.0-only OR MIT
//
// This header provides constants for SERDES MUX for TI SoCs
//
// Copyright (C) 2023-2024 Texas Instruments Incorporated - https://www.ti.com
//
// J721E
pub const J721E_SERDES0_LANE0_QSGMII_LANE1: c_uint = 0x0;
pub const J721E_SERDES0_LANE0_PCIE0_LANE0: c_uint = 0x1;
pub const J721E_SERDES0_LANE0_USB3_0_SWAP: c_uint = 0x2;
pub const J721E_SERDES0_LANE0_IP4_UNUSED: c_uint = 0x3;
pub const J721E_SERDES0_LANE1_QSGMII_LANE2: c_uint = 0x0;
pub const J721E_SERDES0_LANE1_PCIE0_LANE1: c_uint = 0x1;
pub const J721E_SERDES0_LANE1_USB3_0: c_uint = 0x2;
pub const J721E_SERDES0_LANE1_IP4_UNUSED: c_uint = 0x3;
pub const J721E_SERDES1_LANE0_QSGMII_LANE3: c_uint = 0x0;
pub const J721E_SERDES1_LANE0_PCIE1_LANE0: c_uint = 0x1;
pub const J721E_SERDES1_LANE0_USB3_1_SWAP: c_uint = 0x2;
pub const J721E_SERDES1_LANE0_SGMII_LANE0: c_uint = 0x3;
pub const J721E_SERDES1_LANE1_QSGMII_LANE4: c_uint = 0x0;
pub const J721E_SERDES1_LANE1_PCIE1_LANE1: c_uint = 0x1;
pub const J721E_SERDES1_LANE1_USB3_1: c_uint = 0x2;
pub const J721E_SERDES1_LANE1_SGMII_LANE1: c_uint = 0x3;
pub const J721E_SERDES2_LANE0_IP1_UNUSED: c_uint = 0x0;
pub const J721E_SERDES2_LANE0_PCIE2_LANE0: c_uint = 0x1;
pub const J721E_SERDES2_LANE0_USB3_1_SWAP: c_uint = 0x2;
pub const J721E_SERDES2_LANE0_SGMII_LANE0: c_uint = 0x3;
pub const J721E_SERDES2_LANE1_IP1_UNUSED: c_uint = 0x0;
pub const J721E_SERDES2_LANE1_PCIE2_LANE1: c_uint = 0x1;
pub const J721E_SERDES2_LANE1_USB3_1: c_uint = 0x2;
pub const J721E_SERDES2_LANE1_SGMII_LANE1: c_uint = 0x3;
pub const J721E_SERDES3_LANE0_IP1_UNUSED: c_uint = 0x0;
pub const J721E_SERDES3_LANE0_PCIE3_LANE0: c_uint = 0x1;
pub const J721E_SERDES3_LANE0_USB3_0_SWAP: c_uint = 0x2;
pub const J721E_SERDES3_LANE0_IP4_UNUSED: c_uint = 0x3;
pub const J721E_SERDES3_LANE1_IP1_UNUSED: c_uint = 0x0;
pub const J721E_SERDES3_LANE1_PCIE3_LANE1: c_uint = 0x1;
pub const J721E_SERDES3_LANE1_USB3_0: c_uint = 0x2;
pub const J721E_SERDES3_LANE1_IP4_UNUSED: c_uint = 0x3;
pub const J721E_SERDES4_LANE0_EDP_LANE0: c_uint = 0x0;
pub const J721E_SERDES4_LANE0_IP2_UNUSED: c_uint = 0x1;
pub const J721E_SERDES4_LANE0_QSGMII_LANE5: c_uint = 0x2;
pub const J721E_SERDES4_LANE0_IP4_UNUSED: c_uint = 0x3;
pub const J721E_SERDES4_LANE1_EDP_LANE1: c_uint = 0x0;
pub const J721E_SERDES4_LANE1_IP2_UNUSED: c_uint = 0x1;
pub const J721E_SERDES4_LANE1_QSGMII_LANE6: c_uint = 0x2;
pub const J721E_SERDES4_LANE1_IP4_UNUSED: c_uint = 0x3;
pub const J721E_SERDES4_LANE2_EDP_LANE2: c_uint = 0x0;
pub const J721E_SERDES4_LANE2_IP2_UNUSED: c_uint = 0x1;
pub const J721E_SERDES4_LANE2_QSGMII_LANE7: c_uint = 0x2;
pub const J721E_SERDES4_LANE2_IP4_UNUSED: c_uint = 0x3;
pub const J721E_SERDES4_LANE3_EDP_LANE3: c_uint = 0x0;
pub const J721E_SERDES4_LANE3_IP2_UNUSED: c_uint = 0x1;
pub const J721E_SERDES4_LANE3_QSGMII_LANE8: c_uint = 0x2;
pub const J721E_SERDES4_LANE3_IP4_UNUSED: c_uint = 0x3;
// J7200
pub const J7200_SERDES0_LANE0_QSGMII_LANE3: c_uint = 0x0;
pub const J7200_SERDES0_LANE0_PCIE1_LANE0: c_uint = 0x1;
pub const J7200_SERDES0_LANE0_IP3_UNUSED: c_uint = 0x2;
pub const J7200_SERDES0_LANE0_IP4_UNUSED: c_uint = 0x3;
pub const J7200_SERDES0_LANE1_QSGMII_LANE4: c_uint = 0x0;
pub const J7200_SERDES0_LANE1_PCIE1_LANE1: c_uint = 0x1;
pub const J7200_SERDES0_LANE1_IP3_UNUSED: c_uint = 0x2;
pub const J7200_SERDES0_LANE1_IP4_UNUSED: c_uint = 0x3;
pub const J7200_SERDES0_LANE2_QSGMII_LANE1: c_uint = 0x0;
pub const J7200_SERDES0_LANE2_PCIE1_LANE2: c_uint = 0x1;
pub const J7200_SERDES0_LANE2_IP3_UNUSED: c_uint = 0x2;
pub const J7200_SERDES0_LANE2_IP4_UNUSED: c_uint = 0x3;
pub const J7200_SERDES0_LANE3_QSGMII_LANE2: c_uint = 0x0;
pub const J7200_SERDES0_LANE3_PCIE1_LANE3: c_uint = 0x1;
pub const J7200_SERDES0_LANE3_USB: c_uint = 0x2;
pub const J7200_SERDES0_LANE3_IP4_UNUSED: c_uint = 0x3;
// AM64
pub const AM64_SERDES0_LANE0_PCIE0: c_uint = 0x0;
pub const AM64_SERDES0_LANE0_USB: c_uint = 0x1;
// J721S2
pub const J721S2_SERDES0_LANE0_EDP_LANE0: c_uint = 0x0;
pub const J721S2_SERDES0_LANE0_PCIE1_LANE0: c_uint = 0x1;
pub const J721S2_SERDES0_LANE0_IP3_UNUSED: c_uint = 0x2;
pub const J721S2_SERDES0_LANE0_IP4_UNUSED: c_uint = 0x3;
pub const J721S2_SERDES0_LANE1_EDP_LANE1: c_uint = 0x0;
pub const J721S2_SERDES0_LANE1_PCIE1_LANE1: c_uint = 0x1;
pub const J721S2_SERDES0_LANE1_USB: c_uint = 0x2;
pub const J721S2_SERDES0_LANE1_IP4_UNUSED: c_uint = 0x3;
pub const J721S2_SERDES0_LANE2_EDP_LANE2: c_uint = 0x0;
pub const J721S2_SERDES0_LANE2_PCIE1_LANE2: c_uint = 0x1;
pub const J721S2_SERDES0_LANE2_USB_SWAP: c_uint = 0x2;
pub const J721S2_SERDES0_LANE2_IP4_UNUSED: c_uint = 0x3;
pub const J721S2_SERDES0_LANE3_EDP_LANE3: c_uint = 0x0;
pub const J721S2_SERDES0_LANE3_PCIE1_LANE3: c_uint = 0x1;
pub const J721S2_SERDES0_LANE3_USB: c_uint = 0x2;
pub const J721S2_SERDES0_LANE3_IP4_UNUSED: c_uint = 0x3;
// J784S4
pub const J784S4_SERDES0_LANE0_IP1_UNUSED: c_uint = 0x0;
pub const J784S4_SERDES0_LANE0_PCIE1_LANE0: c_uint = 0x1;
pub const J784S4_SERDES0_LANE0_IP3_UNUSED: c_uint = 0x2;
pub const J784S4_SERDES0_LANE0_IP4_UNUSED: c_uint = 0x3;
pub const J784S4_SERDES0_LANE1_IP1_UNUSED: c_uint = 0x0;
pub const J784S4_SERDES0_LANE1_PCIE1_LANE1: c_uint = 0x1;
pub const J784S4_SERDES0_LANE1_IP3_UNUSED: c_uint = 0x2;
pub const J784S4_SERDES0_LANE1_IP4_UNUSED: c_uint = 0x3;
pub const J784S4_SERDES0_LANE2_PCIE3_LANE0: c_uint = 0x0;
pub const J784S4_SERDES0_LANE2_PCIE1_LANE2: c_uint = 0x1;
pub const J784S4_SERDES0_LANE2_IP3_UNUSED: c_uint = 0x2;
pub const J784S4_SERDES0_LANE2_IP4_UNUSED: c_uint = 0x3;
pub const J784S4_SERDES0_LANE3_PCIE3_LANE1: c_uint = 0x0;
pub const J784S4_SERDES0_LANE3_PCIE1_LANE3: c_uint = 0x1;
pub const J784S4_SERDES0_LANE3_USB: c_uint = 0x2;
pub const J784S4_SERDES0_LANE3_IP4_UNUSED: c_uint = 0x3;
pub const J784S4_SERDES1_LANE0_QSGMII_LANE3: c_uint = 0x0;
pub const J784S4_SERDES1_LANE0_PCIE0_LANE0: c_uint = 0x1;
pub const J784S4_SERDES1_LANE0_IP3_UNUSED: c_uint = 0x2;
pub const J784S4_SERDES1_LANE0_IP4_UNUSED: c_uint = 0x3;
pub const J784S4_SERDES1_LANE1_QSGMII_LANE4: c_uint = 0x0;
pub const J784S4_SERDES1_LANE1_PCIE0_LANE1: c_uint = 0x1;
pub const J784S4_SERDES1_LANE1_IP3_UNUSED: c_uint = 0x2;
pub const J784S4_SERDES1_LANE1_IP4_UNUSED: c_uint = 0x3;
pub const J784S4_SERDES1_LANE2_QSGMII_LANE1: c_uint = 0x0;
pub const J784S4_SERDES1_LANE2_PCIE0_LANE2: c_uint = 0x1;
pub const J784S4_SERDES1_LANE2_PCIE2_LANE0: c_uint = 0x2;
pub const J784S4_SERDES1_LANE2_IP4_UNUSED: c_uint = 0x3;
pub const J784S4_SERDES1_LANE3_QSGMII_LANE2: c_uint = 0x0;
pub const J784S4_SERDES1_LANE3_PCIE0_LANE3: c_uint = 0x1;
pub const J784S4_SERDES1_LANE3_PCIE2_LANE1: c_uint = 0x2;
pub const J784S4_SERDES1_LANE3_IP4_UNUSED: c_uint = 0x3;
pub const J784S4_SERDES2_LANE0_QSGMII_LANE5: c_uint = 0x0;
pub const J784S4_SERDES2_LANE0_IP2_UNUSED: c_uint = 0x1;
pub const J784S4_SERDES2_LANE0_IP3_UNUSED: c_uint = 0x2;
pub const J784S4_SERDES2_LANE0_IP4_UNUSED: c_uint = 0x3;
pub const J784S4_SERDES2_LANE1_QSGMII_LANE6: c_uint = 0x0;
pub const J784S4_SERDES2_LANE1_IP2_UNUSED: c_uint = 0x1;
pub const J784S4_SERDES2_LANE1_IP3_UNUSED: c_uint = 0x2;
pub const J784S4_SERDES2_LANE1_IP4_UNUSED: c_uint = 0x3;
pub const J784S4_SERDES2_LANE2_QSGMII_LANE7: c_uint = 0x0;
pub const J784S4_SERDES2_LANE2_QSGMII_LANE1: c_uint = 0x1;
pub const J784S4_SERDES2_LANE2_IP3_UNUSED: c_uint = 0x2;
pub const J784S4_SERDES2_LANE2_IP4_UNUSED: c_uint = 0x3;
pub const J784S4_SERDES2_LANE3_QSGMII_LANE8: c_uint = 0x0;
pub const J784S4_SERDES2_LANE3_QSGMII_LANE2: c_uint = 0x1;
pub const J784S4_SERDES2_LANE3_IP3_UNUSED: c_uint = 0x2;
pub const J784S4_SERDES2_LANE3_IP4_UNUSED: c_uint = 0x3;
pub const J784S4_SERDES4_LANE0_EDP_LANE0: c_uint = 0x0;
pub const J784S4_SERDES4_LANE0_QSGMII_LANE5: c_uint = 0x1;
pub const J784S4_SERDES4_LANE0_IP3_UNUSED: c_uint = 0x2;
pub const J784S4_SERDES4_LANE0_IP4_UNUSED: c_uint = 0x3;
pub const J784S4_SERDES4_LANE1_EDP_LANE1: c_uint = 0x0;
pub const J784S4_SERDES4_LANE1_QSGMII_LANE6: c_uint = 0x1;
pub const J784S4_SERDES4_LANE1_IP3_UNUSED: c_uint = 0x2;
pub const J784S4_SERDES4_LANE1_IP4_UNUSED: c_uint = 0x3;
pub const J784S4_SERDES4_LANE2_EDP_LANE2: c_uint = 0x0;
pub const J784S4_SERDES4_LANE2_QSGMII_LANE7: c_uint = 0x1;
pub const J784S4_SERDES4_LANE2_IP3_UNUSED: c_uint = 0x2;
pub const J784S4_SERDES4_LANE2_IP4_UNUSED: c_uint = 0x3;
pub const J784S4_SERDES4_LANE3_EDP_LANE3: c_uint = 0x0;
pub const J784S4_SERDES4_LANE3_QSGMII_LANE8: c_uint = 0x1;
pub const J784S4_SERDES4_LANE3_USB: c_uint = 0x2;
pub const J784S4_SERDES4_LANE3_IP4_UNUSED: c_uint = 0x3;
// J722S
pub const J722S_SERDES0_LANE0_USB: c_uint = 0x0;
pub const J722S_SERDES0_LANE0_QSGMII_LANE2: c_uint = 0x1;
pub const J722S_SERDES1_LANE0_PCIE0_LANE0: c_uint = 0x0;
pub const J722S_SERDES1_LANE0_QSGMII_LANE1: c_uint = 0x1;
