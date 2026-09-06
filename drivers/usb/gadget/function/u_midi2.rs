//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/gadget/function/u_midi2.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Utility definitions for MIDI 2.0 function
//

// UMP Function Block info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct f_midi2_block_info {
    pub /: *mut *mut unsigned int direction; / FB direction: 1-3,
    pub /: *mut *mut unsigned int first_group; / first UMP group: 0-15,
    pub /: *mut *mut unsigned int num_groups; / number of UMP groups: 1-16,
    pub /: *mut *mut unsigned int midi1_first_group; / first UMP group for MIDI 1.0,
    pub /: *mut *mut unsigned int midi1_num_groups; / number of UMP groups for MIDI 1.0,
    pub /: *mut *mut unsigned int ui_hint; / UI-hint: 0-3,
    pub /: *mut *mut unsigned int midi_ci_version; / MIDI-CI version: 0-255,
    pub /: *mut *mut unsigned int sysex8_streams; / number of sysex8 streams: 0-255,
    pub /: *mut *mut unsigned int is_midi1; / MIDI 1.0 port: 0-2,
    pub /: *mut *mut bool active; / FB active flag: bool,
    pub /: *const *const *const char name; / FB name,
}

// UMP Endpoint info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct f_midi2_ep_info {
    pub /: *mut *mut unsigned int protocol_caps; / protocol capabilities: 1-3,
    pub /: *mut *mut unsigned int protocol; / default protocol: 1-2,
    pub /: *mut *mut unsigned int manufacturer; / manufacturer id: 0-0xffffff,
    pub /: *mut *mut unsigned int family; / device family id: 0-0xffff,
    pub /: *mut *mut unsigned int model; / device model id: 0x-0xffff,
    pub /: *mut *mut unsigned int sw_revision; / software revision: 32bit,
    pub /: *const *const *const char ep_name; / Endpoint name,
    pub /: *const *const *const char product_id; / Product ID,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct f_midi2_card_info {
    pub /: *mut *mut bool process_ump; / process UMP stream: bool,
    pub /: *mut *mut bool static_block; / static FBs: bool,
    pub /: *mut *mut unsigned int req_buf_size; / request buffer size,
    pub /: *mut *mut unsigned int num_reqs; / number of requests,
    pub /: *const *const *const char iface_name; / interface name,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct f_midi2_block_opts {
    pub group: config_group,
    pub id: c_uint,
    pub info: f_midi2_block_info,
    pub ep: *mut f_midi2_ep_opts,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct f_midi2_ep_opts {
    pub group: config_group,
    pub index: c_uint,
    pub info: f_midi2_ep_info,
    pub blks: [*mut f_midi2_block_opts; SNDRV_UMP_MAX_BLOCKS],
    pub opts: *mut f_midi2_opts,
}

pub const MAX_UMP_EPS: c_int = 4;
pub const MAX_CABLES: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct f_midi2_opts {
    pub func_inst: usb_function_instance,
    pub lock: mutex,
    pub refcnt: c_int,
    pub info: f_midi2_card_info,
    pub num_eps: c_uint,
    pub eps: [*mut f_midi2_ep_opts; MAX_UMP_EPS],
}
