//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvkm/subdev/bios/dcb.h
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
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dcb_output_type {
    DCB_OUTPUT_ANALOG	= 0x0,
    DCB_OUTPUT_TV		= 0x1,
    DCB_OUTPUT_TMDS		= 0x2,
    DCB_OUTPUT_LVDS		= 0x3,
    DCB_OUTPUT_DP		= 0x6,
    DCB_OUTPUT_WFD		= 0x8,
    DCB_OUTPUT_EOL		= 0xe,
    DCB_OUTPUT_UNUSED	= 0xf,
    DCB_OUTPUT_ANY = -1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcb_output {
    pub /: *mut *mut int index; / may not be raw dcb index if merging has happened,
    pub hasht: u16,
    pub hashm: u16,
    pub type: dcb_output_type,
    pub i2c_index: u8,
    pub heads: u8,
    pub connector: u8,
    pub bus: u8,
    pub location: u8,
    pub or: u8,
    pub link: u8,
    pub duallink_possible: bool,
    pub extdev: u8,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sor_conf {
    pub link: c_int,
    pub sorconf: },
    pub maxfreq: c_int,
    pub crtconf: },
    pub sor: sor_conf,
    pub use_straps_for_mode: bool,
    pub use_acpi_for_edid: bool,
    pub use_power_scripts: bool,
    pub lvdsconf: },
    pub has_component_output: bool,
    pub tvconf: },
    pub sor: sor_conf,
    pub link_nr: c_int,
    pub link_bw: c_int,
    pub dpconf: },
    pub sor: sor_conf,
    pub slave_addr: c_int,
    pub tmdsconf: },
}

extern "C" {
    pub fn dcb_table(: *mut nvkm_bios, ver: *mut u8, hdr: *mut u8, ent: *mut u8, len: *mut u8) -> u16;
}
extern "C" {
    pub fn dcb_outp(: *mut nvkm_bios, idx: u8, ver: *mut u8, len: *mut u8) -> u16;
}
