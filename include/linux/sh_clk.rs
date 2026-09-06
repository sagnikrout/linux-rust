//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sh_clk.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_mapping {
    pub phys: phys_addr_t,
    pub base: *mut void __iomem,
    pub len: c_ulong,
    pub ref: kref,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sh_clk_ops {

    pub clk): *mut *mut void (init)(struct clk,

    pub clk): *mut *mut int (enable)(struct clk,
    pub clk): *mut *mut void (disable)(struct clk,
    pub clk): *mut *mut unsigned long (recalc)(struct clk,
    pub rate): *mut *mut *mut int (set_rate)(struct clk clk, unsigned long,
    pub parent): *mut *mut *mut int (set_parent)(struct clk clk, struct clk,
    pub rate): *mut *mut *mut long (round_rate)(struct clk clk, unsigned long,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk {
    pub node: list_head,
    pub parent: *mut clk,
    pub /: *mut *mut *mut *mut clk parent_table; / list of parents to,
    pub /: *mut *mut unsigned short parent_num; / choose between,
    pub /: *mut *mut unsigned char src_shift; / source clock field in the,
    pub /: *mut *mut unsigned char src_width; / configuration register,
    pub ops: *mut sh_clk_ops,
    pub children: list_head,
    pub /: *mut *mut list_head sibling; / node for children,
    pub usecount: c_int,
    pub rate: c_ulong,
    pub flags: c_ulong,
    pub enable_reg: *mut void __iomem,
    pub status_reg: *mut void __iomem,
    pub enable_bit: c_uint,
    pub mapped_reg: *mut void __iomem,
    pub div_mask: c_uint,
    pub arch_flags: c_ulong,
    pub priv: *mut c_void,
    pub mapping: *mut clk_mapping,
    pub freq_table: *mut cpufreq_frequency_table,
    pub nr_freqs: c_uint,
}

// drivers/sh/clk.c
extern "C" {
    pub fn followparent_recalc(: *mut clk) -> c_ulong;
}
extern "C" {
    pub fn recalculate_root_clocks();
}
extern "C" {
    pub fn propagate_rate(: *mut clk);
}
extern "C" {
    pub fn clk_reparent(child: *mut clk, parent: *mut clk) -> c_int;
}
extern "C" {
    pub fn clk_register(: *mut clk) -> c_int;
}
extern "C" {
    pub fn clk_unregister(: *mut clk);
}
extern "C" {
    pub fn clk_enable_init_clocks();
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_div_mult_table {
    pub divisors: *mut c_uint,
    pub nr_divisors: c_uint,
    pub multipliers: *mut c_uint,
    pub nr_multipliers: c_uint,
}

extern "C" {
    pub fn sh_clk_mstp_register(clks: *mut clk, nr: c_int) -> c_int;
}
//
// MSTP registration never really cared about access size, despite the
// original enable/disable pairs assuming a 32-bit access. Clocks are
// responsible for defining their access sizes either directly or via the
// clock definition wrappers.
//
extern "C" {
    pub fn sh_clk_mstp_register(_arg: clks, _arg: nr) -> return;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_div_table {
    pub div_mult_table: *mut clk_div_mult_table,
    pub clk): *mut *mut void (kick)(struct clk,
}

extern "C" {
    pub fn sh_clk_div6_register(clks: *mut clk, nr: c_int) -> c_int;
}
extern "C" {
    pub fn sh_clk_div6_reparent_register(clks: *mut clk, nr: c_int) -> c_int;
}

// .enable_reg will be updated to .mapping on sh_clk_fsidiv_register()

extern "C" {
    pub fn sh_clk_fsidiv_register(clks: *mut clk, nr: c_int) -> c_int;
}
