//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/dpll/zl3073x/fw.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// enum zl3073x_fw_component_id - Identifiers for possible flash components
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum zl3073x_fw_component_id {
    ZL_FW_COMPONENT_INVALID = -1,
    ZL_FW_COMPONENT_UTIL = 0,
    ZL_FW_COMPONENT_FW1,
    ZL_FW_COMPONENT_FW2,
    ZL_FW_COMPONENT_FW3,
    ZL_FW_COMPONENT_CFG0,
    ZL_FW_COMPONENT_CFG1,
    ZL_FW_COMPONENT_CFG2,
    ZL_FW_COMPONENT_CFG3,
    ZL_FW_COMPONENT_CFG4,
    ZL_FW_COMPONENT_CFG5,
    ZL_FW_COMPONENT_CFG6,
    ZL_FW_NUM_COMPONENTS
}

//
// struct zl3073x_fw_component - Firmware component
// @id: Flash component ID
// @size: Size of the buffer
// @data: Pointer to buffer with component data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zl3073x_fw_component {
    pub id: zl3073x_fw_component_id,
    pub size: usize,
    pub data: *mut c_void,
}

//
// struct zl3073x_fw - Firmware bundle
// @component: firmware components array
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zl3073x_fw {
    pub component: [*mut zl3073x_fw_component; ZL_FW_NUM_COMPONENTS],
}

extern "C" {
    pub fn zl3073x_fw_free(fw: *mut zl3073x_fw);
}
