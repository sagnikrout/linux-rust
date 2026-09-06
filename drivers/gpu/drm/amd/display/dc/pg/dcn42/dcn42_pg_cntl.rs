//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/pg/dcn42/dcn42_pg_cntl.h
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
// Copyright 2026 Advanced Micro Devices, Inc.

// Macro flag: #define PG_CNTL_REG_LIST_DCN42()\
// Macro flag: #define PG_CNTL_REG_LIST_DCN42B()\
// SR(DOMAIN19_PG_CONFIG), */ \

// Not in DCN42B:
// PG_CNTL_SF(DOMAIN19_PG_CONFIG, DOMAIN_POWER_FORCEON, mask_sh),
// PG_CNTL_SF(DOMAIN19_PG_CONFIG, DOMAIN_POWER_GATE, mask_sh),
// PG_CNTL_SF(DOMAIN19_PG_STATUS, DOMAIN_DESIRED_PWR_STATE, mask_sh),
// PG_CNTL_SF(DOMAIN19_PG_STATUS, DOMAIN_PGFSM_PWR_STATUS, mask_sh),
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pg_cntl_shift {
    pub IP_REQUEST_EN: u8,
    pub DOMAIN_POWER_FORCEON: u8,
    pub DOMAIN_POWER_GATE: u8,
    pub DOMAIN_DESIRED_PWR_STATE: u8,
    pub DOMAIN_PGFSM_PWR_STATUS: u8,
    pub LONO_FGCG_REP_DIS: u8,
    pub AZ_GLOBAL_FGCG_REP_DIS: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pg_cntl_mask {
    pub IP_REQUEST_EN: u32,
    pub DOMAIN_POWER_FORCEON: u32,
    pub DOMAIN_POWER_GATE: u32,
    pub DOMAIN_DESIRED_PWR_STATE: u32,
    pub DOMAIN_PGFSM_PWR_STATUS: u32,
    pub LONO_FGCG_REP_DIS: u32,
    pub AZ_GLOBAL_FGCG_REP_DIS: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pg_cntl_registers {
    pub LONO_STATE: u32,
    pub DC_IP_REQUEST_CNTL: u32,
    pub DOMAIN0_PG_CONFIG: u32,
    pub DOMAIN1_PG_CONFIG: u32,
    pub DOMAIN2_PG_CONFIG: u32,
    pub DOMAIN3_PG_CONFIG: u32,
    pub DOMAIN16_PG_CONFIG: u32,
    pub DOMAIN17_PG_CONFIG: u32,
    pub DOMAIN18_PG_CONFIG: u32,
    pub DOMAIN19_PG_CONFIG: u32,
    pub DOMAIN22_PG_CONFIG: u32,
    pub DOMAIN23_PG_CONFIG: u32,
    pub DOMAIN24_PG_CONFIG: u32,
    pub DOMAIN25_PG_CONFIG: u32,
    pub DOMAIN26_PG_CONFIG: u32,
    pub DOMAIN0_PG_STATUS: u32,
    pub DOMAIN1_PG_STATUS: u32,
    pub DOMAIN2_PG_STATUS: u32,
    pub DOMAIN3_PG_STATUS: u32,
    pub DOMAIN16_PG_STATUS: u32,
    pub DOMAIN17_PG_STATUS: u32,
    pub DOMAIN18_PG_STATUS: u32,
    pub DOMAIN19_PG_STATUS: u32,
    pub DOMAIN22_PG_STATUS: u32,
    pub DOMAIN23_PG_STATUS: u32,
    pub DOMAIN24_PG_STATUS: u32,
    pub DOMAIN25_PG_STATUS: u32,
    pub DOMAIN26_PG_STATUS: u32,
    pub DMU_CLK_CNTL: u32,
    pub AZ_CLOCK_CNTL: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn_pg_cntl {
    pub base: pg_cntl,
    pub regs: *const pg_cntl_registers,
    pub pg_cntl_shift: *const pg_cntl_shift,
    pub pg_cntl_mask: *const pg_cntl_mask,
}

extern "C" {
    pub fn pg_cntl42_dsc_pg_control(pg_cntl: *mut pg_cntl, dsc_inst: c_uint, power_on: bool);
}
extern "C" {
    pub fn pg_cntl42_hpo_pg_control(pg_cntl: *mut pg_cntl, power_on: bool);
}
extern "C" {
    pub fn pg_cntl42_io_clk_pg_control(pg_cntl: *mut pg_cntl, power_on: bool);
}
extern "C" {
    pub fn pg_cntl42_plane_otg_pg_control(pg_cntl: *mut pg_cntl, power_on: bool);
}
extern "C" {
    pub fn pg_cntl42_mem_pg_control(pg_cntl: *mut pg_cntl, power_on: bool);
}
extern "C" {
    pub fn pg_cntl42_dio_pg_control(pg_cntl: *mut pg_cntl, power_on: bool);
}
extern "C" {
    pub fn dcn42_pg_cntl_destroy(pg_cntl: *mut pg_cntl);
}
extern "C" {
    pub fn pg_cntl42_init_pg_status(pg_cntl: *mut pg_cntl);
}
extern "C" {
    pub fn dcn_pg_cntl_destroy(pg_cntl: *mut pg_cntl);
}
