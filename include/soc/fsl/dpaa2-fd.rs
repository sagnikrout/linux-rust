//! Automatically rewritten from C Header to Rust Module
//! Source: include/soc/fsl/dpaa2-fd.h
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


// SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause)
//
// Copyright 2014-2016 Freescale Semiconductor Inc.
// Copyright 2016 NXP
//

//
// DOC: DPAA2 FD - Frame Descriptor APIs for DPAA2
//
// Frame Descriptors (FDs) are used to describe frame data in the DPAA2.
// Frames can be enqueued and dequeued to Frame Queues (FQs) which are consumed
// by the various DPAA accelerators (WRIOP, SEC, PME, DCE)
//
// There are three types of frames: single, scatter gather, and frame lists.
//
// The set of APIs in this file must be used to create, manipulate and
// query Frame Descriptors.
//
// struct dpaa2_fd - Struct describing FDs
// @words:                for easier/faster copying the whole FD structure
// @simple:               struct for the FD fields
// @simple.addr:          address in the FD
// @simple.len:           length in the FD
// @simple.bpid:          buffer pool ID
// @simple.format_offset: format, offset, and short-length fields
// @simple.frc:           frame context
// @simple.ctrl:          control bits...including dd, sc, va, err, etc
// @simple.flc:           flow context address
//
// This structure represents the basic Frame Descriptor used in the system.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpaa2_fd {
    pub words: [u32; 8],
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpaa2_fd_simple {
    pub addr: __le64,
    pub len: __le32,
    pub bpid: __le16,
    pub format_offset: __le16,
    pub frc: __le32,
    pub ctrl: __le32,
    pub flc: __le64,
    pub simple: },
}

pub const FD_SHORT_LEN_FLAG_MASK: c_uint = 0x1;
pub const FD_SHORT_LEN_FLAG_SHIFT: c_int = 14;
pub const FD_SHORT_LEN_MASK: c_uint = 0x3FFFF;
pub const FD_OFFSET_MASK: c_uint = 0x0FFF;
pub const FD_FORMAT_MASK: c_uint = 0x3;
pub const FD_FORMAT_SHIFT: c_int = 12;
pub const FD_BPID_MASK: c_uint = 0x3FFF;
pub const SG_SHORT_LEN_FLAG_MASK: c_uint = 0x1;
pub const SG_SHORT_LEN_FLAG_SHIFT: c_int = 14;
pub const SG_SHORT_LEN_MASK: c_uint = 0x1FFFF;
pub const SG_OFFSET_MASK: c_uint = 0x0FFF;
pub const SG_FORMAT_MASK: c_uint = 0x3;
pub const SG_FORMAT_SHIFT: c_int = 12;
pub const SG_BPID_MASK: c_uint = 0x3FFF;
pub const SG_FINAL_FLAG_MASK: c_uint = 0x1;
pub const SG_FINAL_FLAG_SHIFT: c_int = 15;
pub const FL_SHORT_LEN_FLAG_MASK: c_uint = 0x1;
pub const FL_SHORT_LEN_FLAG_SHIFT: c_int = 14;
pub const FL_SHORT_LEN_MASK: c_uint = 0x3FFFF;
pub const FL_OFFSET_MASK: c_uint = 0x0FFF;
pub const FL_FORMAT_MASK: c_uint = 0x3;
pub const FL_FORMAT_SHIFT: c_int = 12;
pub const FL_BPID_MASK: c_uint = 0x3FFF;
pub const FL_FINAL_FLAG_MASK: c_uint = 0x1;
pub const FL_FINAL_FLAG_SHIFT: c_int = 15;
// Error bits in FD CTRL
pub const FD_CTRL_ERR_MASK: c_uint = 0x000000FF;
pub const FD_CTRL_UFD: c_uint = 0x00000004;
pub const FD_CTRL_SBE: c_uint = 0x00000008;
pub const FD_CTRL_FLC: c_uint = 0x00000010;
pub const FD_CTRL_FSE: c_uint = 0x00000020;
pub const FD_CTRL_FAERR: c_uint = 0x00000040;
// Annotation bits in FD CTRL
pub const FD_CTRL_PTA: c_uint = 0x00800000;
pub const FD_CTRL_PTV1: c_uint = 0x00400000;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpaa2_fd_format {
    dpaa2_fd_single = 0,
    dpaa2_fd_list,
    dpaa2_fd_sg
}

//
// dpaa2_fd_get_addr() - get the addr field of frame descriptor
// @fd: the given frame descriptor
//
// Return the address in the frame descriptor.
//
// dpaa2_fd_set_addr() - Set the addr field of frame descriptor
// @fd: the given frame descriptor
// @addr: the address needs to be set in frame descriptor
//
// dpaa2_fd_get_frc() - Get the frame context in the frame descriptor
// @fd: the given frame descriptor
//
// Return the frame context field in the frame descriptor.
//
extern "C" {
    pub fn le32_to_cpu(_arg: fd->simple.frc) -> return;
}
//
// dpaa2_fd_set_frc() - Set the frame context in the frame descriptor
// @fd: the given frame descriptor
// @frc: the frame context needs to be set in frame descriptor
//
// dpaa2_fd_get_ctrl() - Get the control bits in the frame descriptor
// @fd: the given frame descriptor
//
// Return the control bits field in the frame descriptor.
//
extern "C" {
    pub fn le32_to_cpu(_arg: fd->simple.ctrl) -> return;
}
//
// dpaa2_fd_set_ctrl() - Set the control bits in the frame descriptor
// @fd: the given frame descriptor
// @ctrl: the control bits to be set in the frame descriptor
//
// dpaa2_fd_get_flc() - Get the flow context in the frame descriptor
// @fd: the given frame descriptor
//
// Return the flow context in the frame descriptor.
//
// dpaa2_fd_set_flc() - Set the flow context field of frame descriptor
// @fd: the given frame descriptor
// @flc_addr: the flow context needs to be set in frame descriptor
//
// dpaa2_fd_get_len() - Get the length in the frame descriptor
// @fd: the given frame descriptor
//
// Return the length field in the frame descriptor.
//
extern "C" {
    pub fn le32_to_cpu(_arg: fd->simple.len) -> return;
}
//
// dpaa2_fd_set_len() - Set the length field of frame descriptor
// @fd: the given frame descriptor
// @len: the length needs to be set in frame descriptor
//
// dpaa2_fd_get_offset() - Get the offset field in the frame descriptor
// @fd: the given frame descriptor
//
// Return the offset.
//
// dpaa2_fd_set_offset() - Set the offset field of frame descriptor
// @fd: the given frame descriptor
// @offset: the offset needs to be set in frame descriptor
//
// dpaa2_fd_get_format() - Get the format field in the frame descriptor
// @fd: the given frame descriptor
//
// Return the format.
//
// dpaa2_fd_set_format() - Set the format field of frame descriptor
// @fd: the given frame descriptor
// @format: the format needs to be set in frame descriptor
//
// dpaa2_fd_get_bpid() - Get the bpid field in the frame descriptor
// @fd: the given frame descriptor
//
// Return the buffer pool id.
//
// dpaa2_fd_set_bpid() - Set the bpid field of frame descriptor
// @fd: the given frame descriptor
// @bpid: buffer pool id to be set
//
// struct dpaa2_sg_entry - the scatter-gathering structure
// @addr: address of the sg entry
// @len: length in this sg entry
// @bpid: buffer pool id
// @format_offset: format and offset fields
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpaa2_sg_entry {
    pub addr: __le64,
    pub len: __le32,
    pub bpid: __le16,
    pub format_offset: __le16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpaa2_sg_format {
    dpaa2_sg_single = 0,
    dpaa2_sg_frame_data,
    dpaa2_sg_sgt_ext
}

// Accessors for SG entry fields
//
// dpaa2_sg_get_addr() - Get the address from SG entry
// @sg: the given scatter-gathering object
//
// Return the address.
//
// dpaa2_sg_set_addr() - Set the address in SG entry
// @sg: the given scatter-gathering object
// @addr: the address to be set
//
// dpaa2_sg_get_len() - Get the length in SG entry
// @sg: the given scatter-gathering object
//
// Return the length.
//
extern "C" {
    pub fn le32_to_cpu(_arg: sg->len) -> return;
}
//
// dpaa2_sg_set_len() - Set the length in SG entry
// @sg: the given scatter-gathering object
// @len: the length to be set
//
// dpaa2_sg_get_offset() - Get the offset in SG entry
// @sg: the given scatter-gathering object
//
// Return the offset.
//
// dpaa2_sg_set_offset() - Set the offset in SG entry
// @sg: the given scatter-gathering object
// @offset: the offset to be set
//
// dpaa2_sg_get_format() - Get the SG format in SG entry
// @sg: the given scatter-gathering object
//
// Return the format.
//
// dpaa2_sg_set_format() - Set the SG format in SG entry
// @sg: the given scatter-gathering object
// @format: the format to be set
//
// dpaa2_sg_get_bpid() - Get the buffer pool id in SG entry
// @sg: the given scatter-gathering object
//
// Return the bpid.
//
// dpaa2_sg_set_bpid() - Set the buffer pool id in SG entry
// @sg: the given scatter-gathering object
// @bpid: the bpid to be set
//
// dpaa2_sg_is_final() - Check final bit in SG entry
// @sg: the given scatter-gathering object
//
// Return bool.
//
// dpaa2_sg_set_final() - Set the final bit in SG entry
// @sg: the given scatter-gathering object
// @final: the final boolean to be set
//
// struct dpaa2_fl_entry - structure for frame list entry.
// @addr:          address in the FLE
// @len:           length in the FLE
// @bpid:          buffer pool ID
// @format_offset: format, offset, and short-length fields
// @frc:           frame context
// @ctrl:          control bits...including pta, pvt1, pvt2, err, etc
// @flc:           flow context address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpaa2_fl_entry {
    pub addr: __le64,
    pub len: __le32,
    pub bpid: __le16,
    pub format_offset: __le16,
    pub frc: __le32,
    pub ctrl: __le32,
    pub flc: __le64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpaa2_fl_format {
    dpaa2_fl_single = 0,
    dpaa2_fl_res,
    dpaa2_fl_sg
}

//
// dpaa2_fl_get_addr() - get the addr field of FLE
// @fle: the given frame list entry
//
// Return the address in the frame list entry.
//
// dpaa2_fl_set_addr() - Set the addr field of FLE
// @fle: the given frame list entry
// @addr: the address needs to be set in frame list entry
//
// dpaa2_fl_get_frc() - Get the frame context in the FLE
// @fle: the given frame list entry
//
// Return the frame context field in the frame list entry.
//
extern "C" {
    pub fn le32_to_cpu(_arg: fle->frc) -> return;
}
//
// dpaa2_fl_set_frc() - Set the frame context in the FLE
// @fle: the given frame list entry
// @frc: the frame context needs to be set in frame list entry
//
// dpaa2_fl_get_ctrl() - Get the control bits in the FLE
// @fle: the given frame list entry
//
// Return the control bits field in the frame list entry.
//
extern "C" {
    pub fn le32_to_cpu(_arg: fle->ctrl) -> return;
}
//
// dpaa2_fl_set_ctrl() - Set the control bits in the FLE
// @fle: the given frame list entry
// @ctrl: the control bits to be set in the frame list entry
//
// dpaa2_fl_get_flc() - Get the flow context in the FLE
// @fle: the given frame list entry
//
// Return the flow context in the frame list entry.
//
// dpaa2_fl_set_flc() - Set the flow context field of FLE
// @fle: the given frame list entry
// @flc_addr: the flow context needs to be set in frame list entry
//
// dpaa2_fl_get_len() - Get the length in the FLE
// @fle: the given frame list entry
//
// Return the length field in the frame list entry.
//
extern "C" {
    pub fn le32_to_cpu(_arg: fle->len) -> return;
}
//
// dpaa2_fl_set_len() - Set the length field of FLE
// @fle: the given frame list entry
// @len: the length needs to be set in frame list entry
//
// dpaa2_fl_get_offset() - Get the offset field in the frame list entry
// @fle: the given frame list entry
//
// Return the offset.
//
// dpaa2_fl_set_offset() - Set the offset field of FLE
// @fle: the given frame list entry
// @offset: the offset needs to be set in frame list entry
//
// dpaa2_fl_get_format() - Get the format field in the FLE
// @fle: the given frame list entry
//
// Return the format.
//
// dpaa2_fl_set_format() - Set the format field of FLE
// @fle: the given frame list entry
// @format: the format needs to be set in frame list entry
//
// dpaa2_fl_get_bpid() - Get the bpid field in the FLE
// @fle: the given frame list entry
//
// Return the buffer pool id.
//
// dpaa2_fl_set_bpid() - Set the bpid field of FLE
// @fle: the given frame list entry
// @bpid: buffer pool id to be set
//
// dpaa2_fl_is_final() - Check final bit in FLE
// @fle: the given frame list entry
//
// Return bool.
//
// dpaa2_fl_set_final() - Set the final bit in FLE
// @fle: the given frame list entry
// @final: the final boolean to be set
//
