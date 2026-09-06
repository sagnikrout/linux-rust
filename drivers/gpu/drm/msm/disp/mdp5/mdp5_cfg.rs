//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/msm/disp/mdp5/mdp5_cfg.h
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
// Copyright (c) 2014 The Linux Foundation. All rights reserved.
//

//
// mdp5_cfg
//
// This module configures the dynamic offsets used by mdp5.xml.h
// (initialized in mdp5_cfg.c)
//
pub const MAX_CTL: c_int = 8;
pub const MAX_BASES: c_int = 8;
pub const MAX_SMP_BLOCKS: c_int = 44;
pub const MAX_CLIENTS: c_int = 32;
extern "C" {
    pub fn DECLARE_BITMAP(_arg: mdp5_smp_state_t, _arg: MAX_SMP_BLOCKS) -> typedef;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp5_sub_block {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp5_lm_instance {
    pub id: c_int,
    pub pp: c_int,
    pub dspp: c_int,
    pub caps: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp5_lm_block {
    pub instances: [mdp5_lm_instance; MAX_BASES],
    pub /: *mut *mut uint32_t nb_stages; / number of stages per blender,
    pub /: *mut *mut uint32_t max_width; / Maximum output resolution,
    pub max_height: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp5_pipe_block {
    pub /: *mut *mut uint32_t caps; / pipe capabilities,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp5_ctl_block {
    pub /: *mut *mut uint32_t flush_hw_mask; / FLUSH register's hardware mask,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp5_smp_block {
    pub /: *mut *mut int mmb_count; / number of SMP MMBs,
    pub /: *mut *mut int mmb_size; / MMB: size in bytes,
    pub /: *mut *mut uint32_t clients[MAX_CLIENTS]; / SMP port allocation /pipe,
    pub /: *mut *mut mdp5_smp_state_t reserved_state;/ SMP MMBs statically allocated,
    pub /: *mut *mut uint8_t reserved[MAX_CLIENTS]; / # of MMBs allocated per client,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp5_mdp_block {
    pub /: *mut *mut uint32_t caps; / MDP capabilities: MDP_CAP_xxx bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp5_wb_instance {
    pub id: c_int,
    pub lm: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp5_wb_block {
    pub instances: [mdp5_wb_instance; MAX_BASES],
}

pub const MDP5_INTF_NUM_MAX: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp5_intf_block {
    pub base: [u32; MAX_BASES],
    pub /: *mut *mut u32 connect[MDP5_INTF_NUM_MAX]; / array of enum mdp5_intf_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp5_perf_block {
    pub ab_inefficiency: u32,
    pub ib_inefficiency: u32,
    pub clk_inefficiency: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp5_cfg_hw {
    pub name: *mut c_char,
    pub mdp: mdp5_mdp_block,
    pub smp: mdp5_smp_block,
    pub ctl: mdp5_ctl_block,
    pub pipe_vig: mdp5_pipe_block,
    pub pipe_rgb: mdp5_pipe_block,
    pub pipe_dma: mdp5_pipe_block,
    pub pipe_cursor: mdp5_pipe_block,
    pub lm: mdp5_lm_block,
    pub dspp: mdp5_sub_block,
    pub ad: mdp5_sub_block,
    pub pp: mdp5_sub_block,
    pub dsc: mdp5_sub_block,
    pub cdm: mdp5_sub_block,
    pub wb: mdp5_wb_block,
    pub intf: mdp5_intf_block,
    pub perf: mdp5_perf_block,
    pub max_clk: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp5_cfg {
    pub hw: *const mdp5_cfg_hw,
}

extern "C" {
    pub fn mdp5_cfg_get_hw_rev(cfg_hnd: *mut mdp5_cfg_handler) -> c_int;
}

