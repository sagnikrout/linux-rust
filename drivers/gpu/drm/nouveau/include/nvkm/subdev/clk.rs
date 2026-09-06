//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvkm/subdev/clk.h
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
pub enum nv_clk_src {
    nv_clk_src_crystal,
    nv_clk_src_href,

    nv_clk_src_hclk,
    nv_clk_src_hclkm3,
    nv_clk_src_hclkm3d2,
    nv_clk_src_hclkm2d3, /* NVAA */
    nv_clk_src_hclkm4, /* NVAA */
    nv_clk_src_cclk, /* NVAA */

    nv_clk_src_host,

    nv_clk_src_sppll0,
    nv_clk_src_sppll1,

    nv_clk_src_mpllsrcref,
    nv_clk_src_mpllsrc,
    nv_clk_src_mpll,
    nv_clk_src_mdiv,

    nv_clk_src_core,
    nv_clk_src_core_intm,
    nv_clk_src_shader,

    nv_clk_src_mem,

    nv_clk_src_gpc,
    nv_clk_src_rop,
    nv_clk_src_hubk01,
    nv_clk_src_hubk06,
    nv_clk_src_hubk07,
    nv_clk_src_copy,
    nv_clk_src_pmu,
    nv_clk_src_disp,
    nv_clk_src_vdec,

    nv_clk_src_dom6,

    nv_clk_src_max,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_cstate {
    pub head: list_head,
    pub voltage: u8,
    pub domain: [u32; nv_clk_src_max],
    pub id: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_pstate {
    pub head: list_head,
    pub /: *mut *mut list_head list; / c-states,
    pub base: nvkm_cstate,
    pub pstate: u8,
    pub fanspeed: u8,
    pub pcie_speed: nvkm_pcie_speed,
    pub pcie_width: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_domain {
    pub name: nv_clk_src,
    pub /: *mut *mut u8 bios; / 0xff for none,
pub const NVKM_CLK_DOM_FLAG_CORE: c_uint = 0x01;
pub const NVKM_CLK_DOM_FLAG_VPSTATE: c_uint = 0x02;
    pub flags: u8,
    pub mname: *const c_char,
    pub mdiv: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_clk {
    pub func: *const nvkm_clk_func,
    pub subdev: nvkm_subdev,
    pub domains: *const nvkm_domain,
    pub bstate: nvkm_pstate,
    pub states: list_head,
    pub state_nr: c_int,
    pub work: work_struct,
    pub wait: wait_queue_head_t,
    pub waiting: core::sync::atomic::AtomicI32,
    pub pwrsrc: c_int,
    pub /: *mut *mut int pstate; / current,
    pub /: *mut *mut int ustate_ac; / user-requested (-1 disabled, -2 perfmon),
    pub /: *mut *mut int ustate_dc; / user-requested (-1 disabled, -2 perfmon),
    pub /: *mut *mut int astate; / perfmon adjustment (base),
    pub /: *mut *mut int dstate; / display adjustment (min+),
    pub temp: u8,
    pub allow_reclock: bool,
pub const NVKM_CLK_BOOST_NONE: c_uint = 0x0;
pub const NVKM_CLK_BOOST_BIOS: c_uint = 0x1;
pub const NVKM_CLK_BOOST_FULL: c_uint = 0x2;
    pub boost_mode: u8,
    pub base_khz: u32,
    pub boost_khz: u32,
// XXX: die, these are here *only* to support the completely
// bat-shit insane what-was-nouveau_hw.c code
//
    pub pv): *mut nvkm_pll_vals,
    pub pv): *mut *mut *mut int (pll_prog)(struct nvkm_clk , u32 reg1, struct nvkm_pll_vals,
}

extern "C" {
    pub fn nvkm_clk_read(: *mut nvkm_clk, nv_clk_src: enum) -> c_int;
}
extern "C" {
    pub fn nvkm_clk_ustate(: *mut nvkm_clk, req: c_int, pwr: c_int) -> c_int;
}
extern "C" {
    pub fn nvkm_clk_astate(: *mut nvkm_clk, req: c_int, rel: c_int, wait: bool) -> c_int;
}
extern "C" {
    pub fn nvkm_clk_dstate(: *mut nvkm_clk, req: c_int, rel: c_int) -> c_int;
}
extern "C" {
    pub fn nvkm_clk_tstate(: *mut nvkm_clk, temperature: u8) -> c_int;
}
extern "C" {
    pub fn nvkm_clk_pwrsrc(: *mut nvkm_device) -> c_int;
}
extern "C" {
    pub fn nv04_clk_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_clk) -> c_int;
}
extern "C" {
    pub fn nv40_clk_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_clk) -> c_int;
}
extern "C" {
    pub fn nv50_clk_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_clk) -> c_int;
}
extern "C" {
    pub fn g84_clk_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_clk) -> c_int;
}
extern "C" {
    pub fn mcp77_clk_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_clk) -> c_int;
}
extern "C" {
    pub fn gt215_clk_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_clk) -> c_int;
}
extern "C" {
    pub fn gf100_clk_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_clk) -> c_int;
}
extern "C" {
    pub fn gk104_clk_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_clk) -> c_int;
}
extern "C" {
    pub fn gk20a_clk_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_clk) -> c_int;
}
extern "C" {
    pub fn gm20b_clk_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_clk) -> c_int;
}
extern "C" {
    pub fn gp10b_clk_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_clk) -> c_int;
}
