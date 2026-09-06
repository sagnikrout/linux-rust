//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/mmp/clk.h
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

// Clock type "factor"
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmp_clk_factor_masks {
    pub factor: c_uint,
    pub num_mask: c_uint,
    pub den_mask: c_uint,
    pub num_shift: c_uint,
    pub den_shift: c_uint,
    pub enable_mask: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmp_clk_factor {
    pub hw: clk_hw,
    pub base: *mut void __iomem,
    pub masks: *mut mmp_clk_factor_masks,
    pub ftbl: *mut u32_fract,
    pub ftbl_cnt: c_uint,
    pub lock: *mut spinlock_t,
}

// Clock type "mix"

// The register layout
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmp_clk_mix_reg_info {
    pub reg_clk_ctrl: *mut void __iomem,
    pub reg_clk_sel: *mut void __iomem,
    pub width_div: u8,
    pub shift_div: u8,
    pub width_mux: u8,
    pub shift_mux: u8,
    pub bit_fc: u8,
}

// The suggested clock table from user.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmp_clk_mix_clk_table {
    pub rate: c_ulong,
    pub parent_index: u8,
    pub divisor: c_uint,
    pub valid: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmp_clk_mix_config {
    pub reg_info: mmp_clk_mix_reg_info,
    pub table: *mut mmp_clk_mix_clk_table,
    pub table_size: c_uint,
    pub mux_table: *mut u32,
    pub div_table: *mut clk_div_table,
    pub div_flags: u8,
    pub mux_flags: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmp_clk_mix {
    pub hw: clk_hw,
    pub reg_info: mmp_clk_mix_reg_info,
    pub table: *mut mmp_clk_mix_clk_table,
    pub mux_table: *mut u32,
    pub div_table: *mut clk_div_table,
    pub table_size: c_uint,
    pub div_flags: u8,
    pub mux_flags: u8,
    pub type: c_uint,
    pub lock: *mut spinlock_t,
}

// Clock type "gate". MMP private gate

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmp_clk_gate {
    pub hw: clk_hw,
    pub reg: *mut void __iomem,
    pub mask: u32,
    pub val_enable: u32,
    pub val_disable: u32,
    pub flags: c_uint,
    pub lock: *mut spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmp_clk_unit {
    pub nr_clks: c_uint,
    pub clk_table: *mut clk,
    pub clk_data: clk_onecell_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmp_param_fixed_rate_clk {
    pub id: c_uint,
    pub name: *mut c_char,
    pub parent_name: *const c_char,
    pub flags: c_ulong,
    pub fixed_rate: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmp_param_fixed_factor_clk {
    pub id: c_uint,
    pub name: *mut c_char,
    pub parent_name: *const c_char,
    pub mult: c_ulong,
    pub div: c_ulong,
    pub flags: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmp_param_general_gate_clk {
    pub id: c_uint,
    pub name: *const c_char,
    pub parent_name: *const c_char,
    pub flags: c_ulong,
    pub offset: c_ulong,
    pub bit_idx: u8,
    pub gate_flags: u8,
    pub lock: *mut spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmp_param_gate_clk {
    pub id: c_uint,
    pub name: *mut c_char,
    pub parent_name: *const c_char,
    pub flags: c_ulong,
    pub offset: c_ulong,
    pub mask: u32,
    pub val_enable: u32,
    pub val_disable: u32,
    pub gate_flags: c_uint,
    pub lock: *mut spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmp_param_mux_clk {
    pub id: c_uint,
    pub name: *mut c_char,
    pub parent_name: *const *const c_char,
    pub num_parents: u8,
    pub flags: c_ulong,
    pub offset: c_ulong,
    pub shift: u8,
    pub width: u8,
    pub mux_flags: u8,
    pub lock: *mut spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmp_param_div_clk {
    pub id: c_uint,
    pub name: *mut c_char,
    pub parent_name: *const c_char,
    pub flags: c_ulong,
    pub offset: c_ulong,
    pub shift: u8,
    pub width: u8,
    pub div_flags: u8,
    pub lock: *mut spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmp_param_pll_clk {
    pub id: c_uint,
    pub name: *mut c_char,
    pub default_rate: c_ulong,
    pub enable_offset: c_ulong,
    pub enable: u32,
    pub offset: c_ulong,
    pub shift: u8,
// MMP3 specific:
    pub input_rate: c_ulong,
    pub postdiv_offset: c_ulong,
    pub postdiv_shift: c_ulong,
}

// Power islands

