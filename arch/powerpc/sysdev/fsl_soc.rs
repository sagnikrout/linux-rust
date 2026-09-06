//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/sysdev/fsl_soc.h
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

extern "C" {
    pub fn get_immrbase() -> phys_addr_t;
}

extern "C" {
    pub fn get_brgfreq() -> u32;
}
extern "C" {
    pub fn get_baudrate() -> u32;
}

extern "C" {
    pub fn fsl_get_sys_freq() -> u32;
}
// The different ports that the DIU can be connected to
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fsl_diu_monitor_port {
    FSL_DIU_PORT_DVI,	/* DVI */
    FSL_DIU_PORT_LVDS,	/* Single-link LVDS */
    FSL_DIU_PORT_DLVDS	/* Dual-link LVDS */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct platform_diu_data_ops {
    pub bpp): c_uint,
    pub gamma_table_base): *mut c_char,
    pub port): *mut *mut void (set_monitor_port)(enum fsl_diu_monitor_port,
    pub pixclock): *mut *mut void (set_pixel_clock)(unsigned int,
    pub port): (enum fsl_diu_monitor_port,
    pub (*release_bootmem)(void): *mut c_void,
}

extern "C" {
    pub fn fsl_hv_restart(cmd: *mut c_char) -> void __noreturn;
}
extern "C" {
    pub fn fsl_hv_halt() -> void __noreturn;
}

