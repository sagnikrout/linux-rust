//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/qlogic/qed/qed_hw.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
// QLogic qed NIC Driver
// Copyright (c) 2015-2017  QLogic Corporation
// Copyright (c) 2019-2020 Marvell International Ltd.
//

// Forward decleration
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum reserved_ptts {
    RESERVED_PTT_EDIAG,
    RESERVED_PTT_USER_SPACE,
    RESERVED_PTT_MAIN,
    RESERVED_PTT_DPC,
    RESERVED_PTT_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum _dmae_cmd_dst_mask {
    DMAE_CMD_DST_MASK_NONE	= 0,
    DMAE_CMD_DST_MASK_PCIE	= 1,
    DMAE_CMD_DST_MASK_GRC	= 2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum _dmae_cmd_src_mask {
    DMAE_CMD_SRC_MASK_PCIE	= 0,
    DMAE_CMD_SRC_MASK_GRC	= 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum _dmae_cmd_crc_mask {
    DMAE_CMD_COMP_CRC_EN_MASK_NONE	= 0,
    DMAE_CMD_COMP_CRC_EN_MASK_SET	= 1
}

// definitions for DMA constants
pub const DMAE_GO_VALUE: c_uint = 0x1;
pub const DMAE_COMPLETION_VAL: c_uint = 0xD1AE;
pub const DMAE_CMD_ENDIANITY: c_uint = 0x2;
pub const DMAE_CMD_SIZE: c_int = 14;

pub const DMAE_MIN_WAIT_TIME: c_uint = 0x2;
pub const DMAE_MAX_CLIENTS: c_int = 32;
//
// qed_gtt_init(): Initialize GTT windows.
//
// @p_hwfn: HW device data.
//
// Return: Void.
//
extern "C" {
    pub fn qed_gtt_init(p_hwfn: *mut qed_hwfn);
}
//
// qed_ptt_pool_alloc(): Allocate and initialize PTT pool.
//
// @p_hwfn: HW device data.
//
// Return: struct _qed_status - success (0), negative - error.
//
extern "C" {
    pub fn qed_ptt_pool_alloc(p_hwfn: *mut qed_hwfn) -> c_int;
}
//
// qed_ptt_pool_free(): Free PTT pool.
//
// @p_hwfn: HW device data.
//
// Return: Void.
//
extern "C" {
    pub fn qed_ptt_pool_free(p_hwfn: *mut qed_hwfn);
}
//
// qed_ptt_get_hw_addr(): Get PTT's GRC/HW address.
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt
//
// Return: u32.
//
// qed_ptt_get_bar_addr(): Get PPT's external BAR address.
//
// @p_ptt: P_ptt
//
// Return: u32.
//
extern "C" {
    pub fn qed_ptt_get_bar_addr(p_ptt: *mut qed_ptt) -> u32;
}
//
// qed_ptt_set_win(): Set PTT Window's GRC BAR address
//
// @p_hwfn: HW device data.
// @new_hw_addr: New HW address.
// @p_ptt: P_Ptt
//
// Return: Void.
//
// qed_get_reserved_ptt(): Get a specific reserved PTT.
//
// @p_hwfn: HW device data.
// @ptt_idx: Ptt Index.
//
// Return: struct qed_ptt *.
//
// qed_wr(): Write value to BAR using the given ptt.
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
// @val: Val.
// @hw_addr: HW address
//
// Return: Void.
//
// qed_rd(): Read value from BAR using the given ptt.
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
// @hw_addr: HW address
//
// Return: Void.
//
// qed_memcpy_from(): Copy n bytes from BAR using the given ptt.
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
// @dest: Destination.
// @hw_addr: HW address.
// @n: N
//
// Return: Void.
//
// qed_memcpy_to(): Copy n bytes to BAR using the given  ptt
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
// @hw_addr: HW address.
// @src: Source.
// @n: N
//
// Return: Void.
//
// qed_fid_pretend(): pretend to another function when
// accessing the ptt window. There is no way to unpretend
// a function. The only way to cancel a pretend is to
// pretend back to the original function.
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
// @fid: fid field of pxp_pretend structure. Can contain
// either pf / vf, port/path fields are don't care.
//
// Return: Void.
//
// qed_port_pretend(): Pretend to another port when accessing the ptt window
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
// @port_id: The port to pretend to
//
// Return: Void.
//
// qed_port_unpretend(): Cancel any previously set port pretend
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
//
// Return: Void.
//
// qed_port_fid_pretend(): Pretend to another port and another function
// when accessing the ptt window
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
// @port_id: The port to pretend to
// @fid: fid field of pxp_pretend structure. Can contain either pf / vf.
//
// Return: Void.
//
// qed_vfid_to_concrete(): Build a concrete FID for a given VF ID
//
// @p_hwfn: HW device data.
// @vfid: VFID.
//
// Return: Void.
//
extern "C" {
    pub fn qed_vfid_to_concrete(p_hwfn: *mut qed_hwfn, vfid: u8) -> u32;
}
//
// qed_dmae_idx_to_go_cmd(): Map the idx to dmae cmd
// this is declared here since other files will require it.
//
// @idx: Index
//
// Return: Void.
//
extern "C" {
    pub fn qed_dmae_idx_to_go_cmd(idx: u8) -> u32;
}
//
// qed_dmae_info_alloc(): Init the dmae_info structure
// which is part of p_hwfn.
//
// @p_hwfn: HW device data.
//
// Return: Int.
//
extern "C" {
    pub fn qed_dmae_info_alloc(p_hwfn: *mut qed_hwfn) -> c_int;
}
//
// qed_dmae_info_free(): Free the dmae_info structure
// which is part of p_hwfn.
//
// @p_hwfn: HW device data.
//
// Return: Void.
//
extern "C" {
    pub fn qed_dmae_info_free(p_hwfn: *mut qed_hwfn);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union qed_qm_pq_params {
    pub q_idx: u8,
    pub iscsi: },
    pub tc: u8,
    pub core: },
    pub is_vf: u8,
    pub vf_id: u8,
    pub tc: u8,
    pub eth: },
    pub dcqcn: u8,
    pub /: *mut *mut u8 qpid; / roce relative,
    pub roce: },
}

pub const QED_HW_ERR_MAX_STR_SIZE: c_int = 256;
//
// qed_hw_err_notify(): Notify upper layer driver and management FW
// about a HW error.
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
// @err_type: Err Type.
// @fmt: Debug data buffer to send to the MFW
// @...: buffer format args
//
// Return void.
//
