//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/renesas/vsp1/vsp1_dl.h
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
// vsp1_dl.h  --  R-Car VSP1 Display List
//
// Copyright (C) 2015 Renesas Corporation
//
// Contact: Laurent Pinchart (laurent.pinchart@ideasonboard.com)
//

// Keep these flags in sync with VSP1_DU_STATUS_* in include/media/vsp1.h.

//
// struct vsp1_dl_ext_cmd - Extended Display command
// @pool: pool to which this command belongs
// @free: entry in the pool of free commands list
// @opcode: command type opcode
// @flags: flags used by the command
// @cmds: array of command bodies for this extended cmd
// @num_cmds: quantity of commands in @cmds array
// @cmd_dma: DMA address of the command body
// @data: memory allocation for command-specific data
// @data_dma: DMA address for command-specific data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vsp1_dl_ext_cmd {
    pub pool: *mut vsp1_dl_cmd_pool,
    pub free: list_head,
    pub opcode: u8,
    pub flags: u32,
    pub cmds: *mut vsp1_pre_ext_dl_body,
    pub num_cmds: c_uint,
    pub cmd_dma: dma_addr_t,
    pub data: *mut c_void,
    pub data_dma: dma_addr_t,
}

extern "C" {
    pub fn vsp1_dlm_setup(vsp1: *mut vsp1_device);
}
extern "C" {
    pub fn vsp1_dlm_destroy(dlm: *mut vsp1_dl_manager);
}
extern "C" {
    pub fn vsp1_dlm_reset(dlm: *mut vsp1_dl_manager);
}
extern "C" {
    pub fn vsp1_dlm_irq_frame_end(dlm: *mut vsp1_dl_manager) -> c_uint;
}
extern "C" {
    pub fn vsp1_dl_list_put(dl: *mut vsp1_dl_list);
}
extern "C" {
    pub fn vsp1_dl_list_commit(dl: *mut vsp1_dl_list, dl_flags: c_uint);
}
extern "C" {
    pub fn vsp1_dl_body_pool_destroy(pool: *mut vsp1_dl_body_pool);
}
extern "C" {
    pub fn vsp1_dl_body_put(dlb: *mut vsp1_dl_body);
}
extern "C" {
    pub fn vsp1_dl_body_write(dlb: *mut vsp1_dl_body, reg: u32, data: u32);
}
extern "C" {
    pub fn vsp1_dl_list_add_body(dl: *mut vsp1_dl_list, dlb: *mut vsp1_dl_body) -> c_int;
}
extern "C" {
    pub fn vsp1_dl_list_add_chain(head: *mut vsp1_dl_list, dl: *mut vsp1_dl_list) -> c_int;
}
