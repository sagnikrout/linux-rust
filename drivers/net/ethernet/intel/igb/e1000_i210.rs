//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/igb/e1000_i210.h
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
// Copyright(c) 2007 - 2018 Intel Corporation.
extern "C" {
    pub fn igb_acquire_swfw_sync_i210(hw: *mut e1000_hw, mask: u16) -> i32;
}
extern "C" {
    pub fn igb_release_swfw_sync_i210(hw: *mut e1000_hw, mask: u16);
}
extern "C" {
    pub fn igb_valid_led_default_i210(hw: *mut e1000_hw, data: *mut u16) -> i32;
}
extern "C" {
    pub fn igb_read_xmdio_reg(hw: *mut e1000_hw, addr: u16, dev_addr: u8, data: *mut u16) -> i32;
}
extern "C" {
    pub fn igb_write_xmdio_reg(hw: *mut e1000_hw, addr: u16, dev_addr: u8, data: u16) -> i32;
}
extern "C" {
    pub fn igb_init_nvm_params_i210(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn igb_get_flash_presence_i210(hw: *mut e1000_hw) -> bool;
}
extern "C" {
    pub fn igb_pll_workaround_i210(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn igb_get_cfg_done_i210(hw: *mut e1000_hw) -> i32;
}
pub const E1000_STM_OPCODE: c_uint = 0xDB00;
pub const E1000_EEPROM_FLASH_SIZE_WORD: c_uint = 0x11;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum E1000_INVM_STRUCTURE_TYPE {
    E1000_INVM_UNINITIALIZED_STRUCTURE		= 0x00,
    E1000_INVM_WORD_AUTOLOAD_STRUCTURE		= 0x01,
    E1000_INVM_CSR_AUTOLOAD_STRUCTURE		= 0x02,
    E1000_INVM_PHY_REGISTER_AUTOLOAD_STRUCTURE	= 0x03,
    E1000_INVM_RSA_KEY_SHA256_STRUCTURE		= 0x04,
    E1000_INVM_INVALIDATED_STRUCTURE		= 0x0F,
}

pub const E1000_INVM_RSA_KEY_SHA256_DATA_SIZE_IN_DWORDS: c_int = 8;
pub const E1000_INVM_CSR_AUTOLOAD_DATA_SIZE_IN_DWORDS: c_int = 1;
pub const E1000_INVM_ULT_BYTES_SIZE: c_int = 8;
pub const E1000_INVM_RECORD_SIZE_IN_BYTES: c_int = 4;
pub const E1000_INVM_VER_FIELD_ONE: c_uint = 0x1FF8;
pub const E1000_INVM_VER_FIELD_TWO: c_uint = 0x7FE000;
pub const E1000_INVM_IMGTYPE_FIELD: c_uint = 0x1F800000;
pub const E1000_INVM_MAJOR_MASK: c_uint = 0x3F0;
pub const E1000_INVM_MINOR_MASK: c_uint = 0xF;
pub const E1000_INVM_MAJOR_SHIFT: c_int = 4;

// NVM offset defaults for i211 device

pub const NVM_INIT_CTRL_4_DEFAULT_I211: c_uint = 0x00C1;
pub const NVM_LED_1_CFG_DEFAULT_I211: c_uint = 0x0184;
pub const NVM_LED_0_2_CFG_DEFAULT_I211: c_uint = 0x200C;
// PLL Defines
pub const E1000_PCI_PMCSR: c_uint = 0x44;
pub const E1000_PCI_PMCSR_D3: c_uint = 0x03;
pub const E1000_MAX_PLL_TRIES: c_int = 5;
pub const E1000_PHY_PLL_UNCONF: c_uint = 0xFF;
pub const E1000_PHY_PLL_FREQ_PAGE: c_uint = 0xFC;
pub const E1000_PHY_PLL_FREQ_REG: c_uint = 0x000E;
pub const E1000_INVM_DEFAULT_AL: c_uint = 0x202F;
pub const E1000_INVM_AUTOLOAD: c_uint = 0x0A;
pub const E1000_INVM_PLL_WO_VAL: c_uint = 0x0010;
