//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/firmware/cirrus/cs_dsp_test_utils.h
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
// Support utilities for cs_dsp testing.
//
// Copyright (C) 2024 Cirrus Logic, Inc. and
// Cirrus Logic International Semiconductor Ltd.
//

//
// struct cs_dsp_test - base class for test utilities
//
// @test:	Pointer to struct kunit instance.
// @dsp:	Pointer to struct cs_dsp instance.
// @local:	Private data for each test suite.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs_dsp_test {
    pub test: *mut kunit,
    pub dsp: *mut cs_dsp,
    pub local: *mut cs_dsp_test_local,
// private: Following members are private
    pub saw_bus_write: bool,
}

//
// struct cs_dsp_mock_alg_def - Info for creating a mock algorithm entry.
//
// @id:		   Algorithm ID.
// @ver:	   Algorithm version.
// @xm_base_words: XM base address in DSP words.
// @xm_size_words: XM size in DSP words.
// @ym_base_words: YM base address in DSP words.
// @ym_size_words: YM size in DSP words.
// @zm_base_words: ZM base address in DSP words.
// @zm_size_words: ZM size in DSP words.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs_dsp_mock_alg_def {
    pub id: c_uint,
    pub ver: c_uint,
    pub xm_base_words: c_uint,
    pub xm_size_words: c_uint,
    pub ym_base_words: c_uint,
    pub ym_size_words: c_uint,
    pub zm_base_words: c_uint,
    pub zm_size_words: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs_dsp_mock_coeff_def {
    pub shortname: *const c_char,
    pub fullname: *const c_char,
    pub description: *const c_char,
    pub type: u16,
    pub flags: u16,
    pub mem_type: u16,
    pub offset_dsp_words: c_uint,
    pub length_bytes: c_uint,
}

//
// struct cs_dsp_mock_xm_header - XM header builder
//
// @test_priv:	     Pointer to the struct cs_dsp_test.
// @blob_data:	     Pointer to the created blob data.
// @blob_size_bytes: Size of the data at blob_data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs_dsp_mock_xm_header {
    pub test_priv: *mut cs_dsp_test,
    pub blob_data: *mut c_void,
    pub blob_size_bytes: usize,
}

extern "C" {
    pub fn cs_dsp_mock_count_regions(region_sizes: *const c_uint) -> c_int;
}
extern "C" {
    pub fn cs_dsp_mock_size_of_region(dsp: *const cs_dsp, mem_type: c_int) -> c_uint;
}
extern "C" {
    pub fn cs_dsp_mock_base_addr_for_mem(priv: *mut cs_dsp_test, mem_type: c_int) -> c_uint;
}
extern "C" {
    pub fn cs_dsp_mock_reg_addr_inc_per_unpacked_word(priv: *mut cs_dsp_test) -> c_uint;
}
extern "C" {
    pub fn cs_dsp_mock_reg_block_length_bytes(priv: *mut cs_dsp_test, mem_type: c_int) -> c_uint;
}
extern "C" {
    pub fn cs_dsp_mock_reg_block_length_registers(priv: *mut cs_dsp_test, mem_type: c_int) -> c_uint;
}
extern "C" {
    pub fn cs_dsp_mock_reg_block_length_dsp_words(priv: *mut cs_dsp_test, mem_type: c_int) -> c_uint;
}
extern "C" {
    pub fn cs_dsp_mock_has_zm(priv: *mut cs_dsp_test) -> bool;
}
extern "C" {
    pub fn cs_dsp_mock_packed_to_unpacked_mem_type(packed_mem_type: c_int) -> c_int;
}
extern "C" {
    pub fn cs_dsp_mock_num_dsp_words_to_num_packed_regs(num_dsp_words: c_uint) -> c_uint;
}
extern "C" {
    pub fn cs_dsp_mock_xm_header_get_fw_version(header: *mut cs_dsp_mock_xm_header) -> c_uint;
}
extern "C" {
    pub fn cs_dsp_mock_xm_header_drop_from_regmap_cache(priv: *mut cs_dsp_test);
}
extern "C" {
    pub fn cs_dsp_mock_xm_header_write_to_regmap(header: *mut cs_dsp_mock_xm_header) -> c_int;
}
extern "C" {
    pub fn cs_dsp_mock_regmap_init(priv: *mut cs_dsp_test) -> c_int;
}
extern "C" {
    pub fn cs_dsp_mock_regmap_drop_system_regs(priv: *mut cs_dsp_test);
}
extern "C" {
    pub fn cs_dsp_mock_regmap_is_dirty(priv: *mut cs_dsp_test, drop_system_regs: bool) -> bool;
}
extern "C" {
    pub fn cs_dsp_mock_wmfw_end_alg_info_block(builder: *mut cs_dsp_mock_wmfw_builder);
}
extern "C" {
    pub fn cs_dsp_mock_wmfw_format_version(builder: *mut cs_dsp_mock_wmfw_builder) -> c_int;
}
