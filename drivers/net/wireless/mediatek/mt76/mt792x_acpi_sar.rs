//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/mediatek/mt76/mt792x_acpi_sar.h
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


// SPDX-License-Identifier: BSD-3-Clause-Clear
// Copyright (C) 2023 MediaTek Inc.
pub const MT792x_ASAR_MIN_DYN: c_int = 1;
pub const MT792x_ASAR_MAX_DYN: c_int = 8;
pub const MT792x_ASAR_MIN_GEO: c_int = 3;
pub const MT792x_ASAR_MAX_GEO: c_int = 8;
pub const MT792x_ASAR_MIN_FG: c_int = 8;

pub const MT792X_ACPI_MTCL_INVALID: c_uint = 0xffffffff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt792x_asar_dyn_limit {
    pub idx: u8,
    pub frp: [u8; 5],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt792x_asar_dyn {
    pub names: [u8; 4],
    pub enable: u8,
    pub nr_tbl: u8,
    pub tbl): DECLARE_FLEX_ARRAY(struct mt792x_asar_dyn_limit,,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt792x_asar_dyn_limit_v2 {
    pub idx: u8,
    pub frp: [u8; 11],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt792x_asar_dyn_v2 {
    pub names: [u8; 4],
    pub enable: u8,
    pub rsvd: u8,
    pub nr_tbl: u8,
    pub tbl): DECLARE_FLEX_ARRAY(struct mt792x_asar_dyn_limit_v2,,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt792x_asar_geo_band {
    pub pwr: u8,
    pub offset: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt792x_asar_geo_limit {
    pub idx: u8,
// 0:2G, 1:5G
    pub band: [mt792x_asar_geo_band; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt792x_asar_geo {
    pub names: [u8; 4],
    pub version: u8,
    pub nr_tbl: u8,
    pub tbl): DECLARE_FLEX_ARRAY(struct mt792x_asar_geo_limit,,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt792x_asar_geo_limit_v2 {
    pub idx: u8,
// 0:2G, 1:5G, 2:6G
    pub band: [mt792x_asar_geo_band; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt792x_asar_geo_v2 {
    pub names: [u8; 4],
    pub version: u8,
    pub rsvd: u8,
    pub nr_tbl: u8,
    pub tbl): DECLARE_FLEX_ARRAY(struct mt792x_asar_geo_limit_v2,,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt792x_asar_cl_v3 {
    pub names: [u8; 4],
    pub version: u8,
    pub mode_6g: u8,
    pub cl6g: [u8; 6],
    pub mode_5g9: u8,
    pub cl5g9: [u8; 6],
    pub mode_be: u8,
    pub clbe: [u8; 6],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt792x_asar_cl {
    pub names: [u8; 4],
    pub version: u8,
    pub mode_6g: u8,
    pub cl6g: [u8; 6],
    pub mode_5g9: u8,
    pub cl5g9: [u8; 6],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt792x_asar_fg {
    pub names: [u8; 4],
    pub version: u8,
    pub rsvd: u8,
    pub nr_flag: u8,
    pub rsvd1: u8,
    pub flag: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt792x_acpi_sar {
    pub ver: u8,
    pub dyn: *mut mt792x_asar_dyn,
    pub dyn_v2: *mut mt792x_asar_dyn_v2,
}
