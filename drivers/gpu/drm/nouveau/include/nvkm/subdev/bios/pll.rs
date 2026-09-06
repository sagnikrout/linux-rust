//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvkm/subdev/bios/pll.h
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


// SPDX-License-Identifier: MIT
// XXX: kill me
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_pll_vals {

    pub M2: uint8_t N1, M1, N2,,

    pub N2: uint8_t M1, N1, M2,,

}

// these match types in pll limits table version 0x40,
// nvkm uses them on all chipsets internally where a
// specific pll needs to be referenced, but the exact
// register isn't known.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvbios_pll_type {
    PLL_CORE   = 0x01,
    PLL_SHADER = 0x02,
    PLL_UNK03  = 0x03,
    PLL_MEMORY = 0x04,
    PLL_VDEC   = 0x05,
    PLL_UNK40  = 0x40,
    PLL_UNK41  = 0x41,
    PLL_UNK42  = 0x42,
    PLL_VPLL0  = 0x80,
    PLL_VPLL1  = 0x81,
    PLL_VPLL2  = 0x82,
    PLL_VPLL3  = 0x83,
    PLL_MAX    = 0xff
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvbios_pll {
    pub type: nvbios_pll_type,
    pub reg: u32,
    pub refclk: u32,
    pub min_p: u8,
    pub max_p: u8,
    pub bias_p: u8,
//
// for most pre nv50 cards setting a log2P of 7 (the common max_log2p
// value) is no different to 6 (at least for vplls) so allowing the MNP
// calc to use 7 causes the generated clock to be out by a factor of 2.
// however, max_log2p cannot be fixed-up during parsing as the
// unmodified max_log2p value is still needed for setting mplls, hence
// an additional max_usable_log2p member
//
    pub max_p_usable: u8,
    pub min_freq: u32,
    pub max_freq: u32,
    pub min_inputfreq: u32,
    pub max_inputfreq: u32,
    pub min_m: u8,
    pub max_m: u8,
    pub min_n: u8,
    pub max_n: u8,
    pub vco2: } vco1,,
}

extern "C" {
    pub fn nvbios_pll_parse(: *mut nvkm_bios, type: u32, : *mut nvbios_pll) -> c_int;
}
