//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/hisi,hi6220-resets.h
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
//
// This header provides index for the reset controller
// based on hi6220 SoC.
//
pub const PERIPH_RSTDIS0_MMC0: c_uint = 0x000;
pub const PERIPH_RSTDIS0_MMC1: c_uint = 0x001;
pub const PERIPH_RSTDIS0_MMC2: c_uint = 0x002;
pub const PERIPH_RSTDIS0_NANDC: c_uint = 0x003;
pub const PERIPH_RSTDIS0_USBOTG_BUS: c_uint = 0x004;
pub const PERIPH_RSTDIS0_POR_PICOPHY: c_uint = 0x005;
pub const PERIPH_RSTDIS0_USBOTG: c_uint = 0x006;
pub const PERIPH_RSTDIS0_USBOTG_32K: c_uint = 0x007;
pub const PERIPH_RSTDIS1_HIFI: c_uint = 0x100;
pub const PERIPH_RSTDIS1_DIGACODEC: c_uint = 0x105;
pub const PERIPH_RSTEN2_IPF: c_uint = 0x200;
pub const PERIPH_RSTEN2_SOCP: c_uint = 0x201;
pub const PERIPH_RSTEN2_DMAC: c_uint = 0x202;
pub const PERIPH_RSTEN2_SECENG: c_uint = 0x203;
pub const PERIPH_RSTEN2_ABB: c_uint = 0x204;
pub const PERIPH_RSTEN2_HPM0: c_uint = 0x205;
pub const PERIPH_RSTEN2_HPM1: c_uint = 0x206;
pub const PERIPH_RSTEN2_HPM2: c_uint = 0x207;
pub const PERIPH_RSTEN2_HPM3: c_uint = 0x208;
pub const PERIPH_RSTEN3_CSSYS: c_uint = 0x300;
pub const PERIPH_RSTEN3_I2C0: c_uint = 0x301;
pub const PERIPH_RSTEN3_I2C1: c_uint = 0x302;
pub const PERIPH_RSTEN3_I2C2: c_uint = 0x303;
pub const PERIPH_RSTEN3_I2C3: c_uint = 0x304;
pub const PERIPH_RSTEN3_UART1: c_uint = 0x305;
pub const PERIPH_RSTEN3_UART2: c_uint = 0x306;
pub const PERIPH_RSTEN3_UART3: c_uint = 0x307;
pub const PERIPH_RSTEN3_UART4: c_uint = 0x308;
pub const PERIPH_RSTEN3_SSP: c_uint = 0x309;
pub const PERIPH_RSTEN3_PWM: c_uint = 0x30a;
pub const PERIPH_RSTEN3_BLPWM: c_uint = 0x30b;
pub const PERIPH_RSTEN3_TSENSOR: c_uint = 0x30c;
pub const PERIPH_RSTEN3_DAPB: c_uint = 0x312;
pub const PERIPH_RSTEN3_HKADC: c_uint = 0x313;
pub const PERIPH_RSTEN3_CODEC_SSI: c_uint = 0x314;
pub const PERIPH_RSTEN3_PMUSSI1: c_uint = 0x316;
pub const PERIPH_RSTEN8_RS0: c_uint = 0x400;
pub const PERIPH_RSTEN8_RS2: c_uint = 0x401;
pub const PERIPH_RSTEN8_RS3: c_uint = 0x402;
pub const PERIPH_RSTEN8_MS0: c_uint = 0x403;
pub const PERIPH_RSTEN8_MS2: c_uint = 0x405;
pub const PERIPH_RSTEN8_XG2RAM0: c_uint = 0x406;
pub const PERIPH_RSTEN8_X2SRAM_TZMA: c_uint = 0x407;
pub const PERIPH_RSTEN8_SRAM: c_uint = 0x408;
pub const PERIPH_RSTEN8_HARQ: c_uint = 0x40a;
pub const PERIPH_RSTEN8_DDRC: c_uint = 0x40c;
pub const PERIPH_RSTEN8_DDRC_APB: c_uint = 0x40d;
pub const PERIPH_RSTEN8_DDRPACK_APB: c_uint = 0x40e;
pub const PERIPH_RSTEN8_DDRT: c_uint = 0x411;
pub const PERIPH_RSDIST9_CARM_DAP: c_uint = 0x500;
pub const PERIPH_RSDIST9_CARM_ATB: c_uint = 0x501;
pub const PERIPH_RSDIST9_CARM_LBUS: c_uint = 0x502;
pub const PERIPH_RSDIST9_CARM_POR: c_uint = 0x503;
pub const PERIPH_RSDIST9_CARM_CORE: c_uint = 0x504;
pub const PERIPH_RSDIST9_CARM_DBG: c_uint = 0x505;
pub const PERIPH_RSDIST9_CARM_L2: c_uint = 0x506;
pub const PERIPH_RSDIST9_CARM_SOCDBG: c_uint = 0x507;
pub const PERIPH_RSDIST9_CARM_ETM: c_uint = 0x508;
pub const MEDIA_G3D: c_int = 0;
pub const MEDIA_CODEC_VPU: c_int = 2;
pub const MEDIA_CODEC_JPEG: c_int = 3;
pub const MEDIA_ISP: c_int = 4;
pub const MEDIA_ADE: c_int = 5;
pub const MEDIA_MMU: c_int = 6;
pub const MEDIA_XG2RAM1: c_int = 7;
pub const AO_G3D: c_int = 1;
pub const AO_CODECISP: c_int = 2;
pub const AO_MCPU: c_int = 4;
pub const AO_BBPHARQMEM: c_int = 5;
pub const AO_HIFI: c_int = 8;
pub const AO_ACPUSCUL2C: c_int = 12;
