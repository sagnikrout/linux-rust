//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/memory/tegra114-mc.h
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


// SPDX-License-Identifier: GPL-2.0
pub const TEGRA_SWGROUP_PTC: c_int = 0;
pub const TEGRA_SWGROUP_DC: c_int = 1;
pub const TEGRA_SWGROUP_DCB: c_int = 2;
pub const TEGRA_SWGROUP_EPP: c_int = 3;
pub const TEGRA_SWGROUP_G2: c_int = 4;
pub const TEGRA_SWGROUP_AVPC: c_int = 5;
pub const TEGRA_SWGROUP_NV: c_int = 6;
pub const TEGRA_SWGROUP_HDA: c_int = 7;
pub const TEGRA_SWGROUP_HC: c_int = 8;
pub const TEGRA_SWGROUP_MSENC: c_int = 9;
pub const TEGRA_SWGROUP_PPCS: c_int = 10;
pub const TEGRA_SWGROUP_VDE: c_int = 11;
pub const TEGRA_SWGROUP_MPCORELP: c_int = 12;
pub const TEGRA_SWGROUP_MPCORE: c_int = 13;
pub const TEGRA_SWGROUP_VI: c_int = 14;
pub const TEGRA_SWGROUP_ISP: c_int = 15;
pub const TEGRA_SWGROUP_XUSB_HOST: c_int = 16;
pub const TEGRA_SWGROUP_XUSB_DEV: c_int = 17;
pub const TEGRA_SWGROUP_EMUCIF: c_int = 18;
pub const TEGRA_SWGROUP_TSEC: c_int = 19;
pub const TEGRA114_MC_RESET_AVPC: c_int = 0;
pub const TEGRA114_MC_RESET_DC: c_int = 1;
pub const TEGRA114_MC_RESET_DCB: c_int = 2;
pub const TEGRA114_MC_RESET_EPP: c_int = 3;
pub const TEGRA114_MC_RESET_2D: c_int = 4;
pub const TEGRA114_MC_RESET_HC: c_int = 5;
pub const TEGRA114_MC_RESET_HDA: c_int = 6;
pub const TEGRA114_MC_RESET_ISP: c_int = 7;
pub const TEGRA114_MC_RESET_MPCORE: c_int = 8;
pub const TEGRA114_MC_RESET_MPCORELP: c_int = 9;
pub const TEGRA114_MC_RESET_MPE: c_int = 10;
pub const TEGRA114_MC_RESET_3D: c_int = 11;
pub const TEGRA114_MC_RESET_3D2: c_int = 12;
pub const TEGRA114_MC_RESET_PPCS: c_int = 13;
pub const TEGRA114_MC_RESET_VDE: c_int = 14;
pub const TEGRA114_MC_RESET_VI: c_int = 15;
pub const TEGRA114_MC_PTCR: c_int = 0;
pub const TEGRA114_MC_DISPLAY0A: c_int = 1;
pub const TEGRA114_MC_DISPLAY0AB: c_int = 2;
pub const TEGRA114_MC_DISPLAY0B: c_int = 3;
pub const TEGRA114_MC_DISPLAY0BB: c_int = 4;
pub const TEGRA114_MC_DISPLAY0C: c_int = 5;
pub const TEGRA114_MC_DISPLAY0CB: c_int = 6;
pub const TEGRA114_MC_DISPLAY1B: c_int = 7;
pub const TEGRA114_MC_DISPLAY1BB: c_int = 8;
pub const TEGRA114_MC_EPPUP: c_int = 9;
pub const TEGRA114_MC_G2PR: c_int = 10;
pub const TEGRA114_MC_G2SR: c_int = 11;
pub const TEGRA114_MC_MPEUNIFBR: c_int = 12;
pub const TEGRA114_MC_VIRUV: c_int = 13;
pub const TEGRA114_MC_AFIR: c_int = 14;
pub const TEGRA114_MC_AVPCARM7R: c_int = 15;
pub const TEGRA114_MC_DISPLAYHC: c_int = 16;
pub const TEGRA114_MC_DISPLAYHCB: c_int = 17;
pub const TEGRA114_MC_FDCDRD: c_int = 18;
pub const TEGRA114_MC_FDCDRD2: c_int = 19;
pub const TEGRA114_MC_G2DR: c_int = 20;
pub const TEGRA114_MC_HDAR: c_int = 21;
pub const TEGRA114_MC_HOST1XDMAR: c_int = 22;
pub const TEGRA114_MC_HOST1XR: c_int = 23;
pub const TEGRA114_MC_IDXSRD: c_int = 24;
pub const TEGRA114_MC_IDXSRD2: c_int = 25;
pub const TEGRA114_MC_MPE_IPRED: c_int = 26;
pub const TEGRA114_MC_MPEAMEMRD: c_int = 27;
pub const TEGRA114_MC_MPECSRD: c_int = 28;
pub const TEGRA114_MC_PPCSAHBDMAR: c_int = 29;
pub const TEGRA114_MC_PPCSAHBSLVR: c_int = 30;
pub const TEGRA114_MC_SATAR: c_int = 31;
pub const TEGRA114_MC_TEXSRD: c_int = 32;
pub const TEGRA114_MC_TEXSRD2: c_int = 33;
pub const TEGRA114_MC_VDEBSEVR: c_int = 34;
pub const TEGRA114_MC_VDEMBER: c_int = 35;
pub const TEGRA114_MC_VDEMCER: c_int = 36;
pub const TEGRA114_MC_VDETPER: c_int = 37;
pub const TEGRA114_MC_MPCORELPR: c_int = 38;
pub const TEGRA114_MC_MPCORER: c_int = 39;
pub const TEGRA114_MC_EPPU: c_int = 40;
pub const TEGRA114_MC_EPPV: c_int = 41;
pub const TEGRA114_MC_EPPY: c_int = 42;
pub const TEGRA114_MC_MPEUNIFBW: c_int = 43;
pub const TEGRA114_MC_VIWSB: c_int = 44;
pub const TEGRA114_MC_VIWU: c_int = 45;
pub const TEGRA114_MC_VIWV: c_int = 46;
pub const TEGRA114_MC_VIWY: c_int = 47;
pub const TEGRA114_MC_G2DW: c_int = 48;
pub const TEGRA114_MC_AFIW: c_int = 49;
pub const TEGRA114_MC_AVPCARM7W: c_int = 50;
pub const TEGRA114_MC_FDCDWR: c_int = 51;
pub const TEGRA114_MC_FDCDWR2: c_int = 52;
pub const TEGRA114_MC_HDAW: c_int = 53;
pub const TEGRA114_MC_HOST1XW: c_int = 54;
pub const TEGRA114_MC_ISPW: c_int = 55;
pub const TEGRA114_MC_MPCORELPW: c_int = 56;
pub const TEGRA114_MC_MPCOREW: c_int = 57;
pub const TEGRA114_MC_MPECSWR: c_int = 58;
pub const TEGRA114_MC_PPCSAHBDMAW: c_int = 59;
pub const TEGRA114_MC_PPCSAHBSLVW: c_int = 60;
pub const TEGRA114_MC_SATAW: c_int = 61;
pub const TEGRA114_MC_VDEBSEVW: c_int = 62;
pub const TEGRA114_MC_VDEDBGW: c_int = 63;
pub const TEGRA114_MC_VDEMBEW: c_int = 64;
pub const TEGRA114_MC_VDETPMW: c_int = 65;
