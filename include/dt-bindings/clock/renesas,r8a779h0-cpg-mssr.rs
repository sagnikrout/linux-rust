//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/renesas,r8a779h0-cpg-mssr.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
//
// Copyright (C) 2023 Renesas Electronics Corp.
//

// r8a779h0 CPG Core Clocks
pub const R8A779H0_CLK_ZX: c_int = 0;
pub const R8A779H0_CLK_ZD: c_int = 1;
pub const R8A779H0_CLK_ZS: c_int = 2;
pub const R8A779H0_CLK_ZT: c_int = 3;
pub const R8A779H0_CLK_ZTR: c_int = 4;
pub const R8A779H0_CLK_S0D2: c_int = 5;
pub const R8A779H0_CLK_S0D3: c_int = 6;
pub const R8A779H0_CLK_S0D4: c_int = 7;
pub const R8A779H0_CLK_S0D1_VIO: c_int = 8;
pub const R8A779H0_CLK_S0D2_VIO: c_int = 9;
pub const R8A779H0_CLK_S0D4_VIO: c_int = 10;
pub const R8A779H0_CLK_S0D8_VIO: c_int = 11;
pub const R8A779H0_CLK_VIOBUSD1: c_int = 12;
pub const R8A779H0_CLK_VIOBUSD2: c_int = 13;
pub const R8A779H0_CLK_S0D1_VC: c_int = 14;
pub const R8A779H0_CLK_S0D2_VC: c_int = 15;
pub const R8A779H0_CLK_S0D4_VC: c_int = 16;
pub const R8A779H0_CLK_VCBUSD1: c_int = 17;
pub const R8A779H0_CLK_VCBUSD2: c_int = 18;
pub const R8A779H0_CLK_S0D2_MM: c_int = 19;
pub const R8A779H0_CLK_S0D4_MM: c_int = 20;
pub const R8A779H0_CLK_S0D2_U3DG: c_int = 21;
pub const R8A779H0_CLK_S0D4_U3DG: c_int = 22;
pub const R8A779H0_CLK_S0D2_RT: c_int = 23;
pub const R8A779H0_CLK_S0D3_RT: c_int = 24;
pub const R8A779H0_CLK_S0D4_RT: c_int = 25;
pub const R8A779H0_CLK_S0D6_RT: c_int = 26;
pub const R8A779H0_CLK_S0D2_PER: c_int = 27;
pub const R8A779H0_CLK_S0D3_PER: c_int = 28;
pub const R8A779H0_CLK_S0D4_PER: c_int = 29;
pub const R8A779H0_CLK_S0D6_PER: c_int = 30;
pub const R8A779H0_CLK_S0D12_PER: c_int = 31;
pub const R8A779H0_CLK_S0D24_PER: c_int = 32;
pub const R8A779H0_CLK_S0D1_HSC: c_int = 33;
pub const R8A779H0_CLK_S0D2_HSC: c_int = 34;
pub const R8A779H0_CLK_S0D4_HSC: c_int = 35;
pub const R8A779H0_CLK_S0D8_HSC: c_int = 36;
pub const R8A779H0_CLK_SVD1_IR: c_int = 37;
pub const R8A779H0_CLK_SVD2_IR: c_int = 38;
pub const R8A779H0_CLK_IMPAD1: c_int = 39;
pub const R8A779H0_CLK_IMPAD4: c_int = 40;
pub const R8A779H0_CLK_IMPB: c_int = 41;
pub const R8A779H0_CLK_SVD1_VIP: c_int = 42;
pub const R8A779H0_CLK_SVD2_VIP: c_int = 43;
pub const R8A779H0_CLK_CL: c_int = 44;
pub const R8A779H0_CLK_CL16M: c_int = 45;
pub const R8A779H0_CLK_CL16M_MM: c_int = 46;
pub const R8A779H0_CLK_CL16M_RT: c_int = 47;
pub const R8A779H0_CLK_CL16M_PER: c_int = 48;
pub const R8A779H0_CLK_CL16M_HSC: c_int = 49;
pub const R8A779H0_CLK_ZC0: c_int = 50;
pub const R8A779H0_CLK_ZC1: c_int = 51;
pub const R8A779H0_CLK_ZC2: c_int = 52;
pub const R8A779H0_CLK_ZC3: c_int = 53;
pub const R8A779H0_CLK_ZB3: c_int = 54;
pub const R8A779H0_CLK_ZB3D2: c_int = 55;
pub const R8A779H0_CLK_ZB3D4: c_int = 56;
pub const R8A779H0_CLK_ZG: c_int = 57;
pub const R8A779H0_CLK_SD0H: c_int = 58;
pub const R8A779H0_CLK_SD0: c_int = 59;
pub const R8A779H0_CLK_RPC: c_int = 60;
pub const R8A779H0_CLK_RPCD2: c_int = 61;
pub const R8A779H0_CLK_MSO: c_int = 62;
pub const R8A779H0_CLK_CANFD: c_int = 63;
pub const R8A779H0_CLK_CSI: c_int = 64;
pub const R8A779H0_CLK_FRAY: c_int = 65;
pub const R8A779H0_CLK_IPC: c_int = 66;
pub const R8A779H0_CLK_SASYNCRT: c_int = 67;
pub const R8A779H0_CLK_SASYNCPERD1: c_int = 68;
pub const R8A779H0_CLK_SASYNCPERD2: c_int = 69;
pub const R8A779H0_CLK_SASYNCPERD4: c_int = 70;
pub const R8A779H0_CLK_DSIEXT: c_int = 71;
pub const R8A779H0_CLK_DSIREF: c_int = 72;
pub const R8A779H0_CLK_ADGH: c_int = 73;
pub const R8A779H0_CLK_OSC: c_int = 74;
pub const R8A779H0_CLK_ZR0: c_int = 75;
pub const R8A779H0_CLK_ZR1: c_int = 76;
pub const R8A779H0_CLK_ZR2: c_int = 77;
pub const R8A779H0_CLK_RGMII: c_int = 78;
pub const R8A779H0_CLK_CPEX: c_int = 79;
pub const R8A779H0_CLK_CP: c_int = 80;
pub const R8A779H0_CLK_CBFUSA: c_int = 81;
pub const R8A779H0_CLK_R: c_int = 82;
