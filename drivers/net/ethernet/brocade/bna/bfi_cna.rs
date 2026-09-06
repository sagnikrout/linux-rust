//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/brocade/bna/bfi_cna.h
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
// Linux network driver for QLogic BR-series Converged Network Adapter.
//
// Copyright (c) 2005-2014 Brocade Communications Systems, Inc.
// Copyright (c) 2014-2015 QLogic Corporation
// All rights reserved
// www.qlogic.com
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_port_h2i {
    BFI_PORT_H2I_ENABLE_REQ		= (1),
    BFI_PORT_H2I_DISABLE_REQ	= (2),
    BFI_PORT_H2I_GET_STATS_REQ	= (3),
    BFI_PORT_H2I_CLEAR_STATS_REQ	= (4),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_port_i2h {
    BFI_PORT_I2H_ENABLE_RSP		= BFA_I2HM(1),
    BFI_PORT_I2H_DISABLE_RSP	= BFA_I2HM(2),
    BFI_PORT_I2H_GET_STATS_RSP	= BFA_I2HM(3),
    BFI_PORT_I2H_CLEAR_STATS_RSP	= BFA_I2HM(4),
}

// Generic REQ type
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_port_generic_req {
    pub /: *mut *mut bfi_mhdr mh; /!< msg header,
    pub /: *mut *mut u32 msgtag; /!< msgtag for reply,
    pub rsvd: u32,
    pub __packed: },
// Generic RSP type
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_port_generic_rsp {
    pub /: *mut *mut bfi_mhdr mh; /!< common msg header,
    pub /: *mut *mut u8 status; /!< port enable status,
    pub rsvd: [u8; 3],
    pub /: *mut *mut u32 msgtag; /!< msgtag for reply,
    pub __packed: },
// BFI_PORT_H2I_GET_STATS_REQ
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_port_get_stats_req {
    pub /: *mut *mut bfi_mhdr mh; /!< common msg header,
    pub dma_addr: bfi_addr_u,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union bfi_port_h2i_msg_u {
    pub mh: bfi_mhdr,
    pub enable_req: bfi_port_generic_req,
    pub disable_req: bfi_port_generic_req,
    pub getstats_req: bfi_port_get_stats_req,
    pub clearstats_req: bfi_port_generic_req,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union bfi_port_i2h_msg_u {
    pub mh: bfi_mhdr,
    pub enable_rsp: bfi_port_generic_rsp,
    pub disable_rsp: bfi_port_generic_rsp,
    pub getstats_rsp: bfi_port_generic_rsp,
    pub clearstats_rsp: bfi_port_generic_rsp,
    pub __packed: },
// @brief Mailbox commands from host to (DCBX/LLDP) firmware
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_cee_h2i_msgs {
    BFI_CEE_H2I_GET_CFG_REQ = 1,
    BFI_CEE_H2I_RESET_STATS = 2,
    BFI_CEE_H2I_GET_STATS_REQ = 3,
}

// @brief Mailbox reply and AEN messages from DCBX/LLDP firmware to host
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_cee_i2h_msgs {
    BFI_CEE_I2H_GET_CFG_RSP = BFA_I2HM(1),
    BFI_CEE_I2H_RESET_STATS_RSP = BFA_I2HM(2),
    BFI_CEE_I2H_GET_STATS_RSP = BFA_I2HM(3),
}

// Data structures
//
// @brief H2I command structure for resetting the stats.
// BFI_CEE_H2I_RESET_STATS
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_lldp_reset_stats {
    pub mh: bfi_mhdr,
    pub __packed: },
//
// @brief H2I command structure for resetting the stats.
// BFI_CEE_H2I_RESET_STATS
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_cee_reset_stats {
    pub mh: bfi_mhdr,
    pub __packed: },
//
// @brief  get configuration  command from host
// BFI_CEE_H2I_GET_CFG_REQ
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_cee_get_req {
    pub mh: bfi_mhdr,
    pub dma_addr: bfi_addr_u,
    pub __packed: },
//
// @brief reply message from firmware
// BFI_CEE_I2H_GET_CFG_RSP
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_cee_get_rsp {
    pub mh: bfi_mhdr,
    pub cmd_status: u8,
    pub rsvd: [u8; 3],
    pub __packed: },
//
// @brief  get configuration  command from host
// BFI_CEE_H2I_GET_STATS_REQ
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_cee_stats_req {
    pub mh: bfi_mhdr,
    pub dma_addr: bfi_addr_u,
    pub __packed: },
//
// @brief reply message from firmware
// BFI_CEE_I2H_GET_STATS_RSP
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_cee_stats_rsp {
    pub mh: bfi_mhdr,
    pub cmd_status: u8,
    pub rsvd: [u8; 3],
    pub __packed: },
// @brief mailbox command structures from host to firmware
#[repr(C)]
#[derive(Copy, Clone)]
pub union bfi_cee_h2i_msg_u {
    pub mh: bfi_mhdr,
    pub get_req: bfi_cee_get_req,
    pub stats_req: bfi_cee_stats_req,
    pub __packed: },
// @brief mailbox message structures from firmware to host
#[repr(C)]
#[derive(Copy, Clone)]
pub union bfi_cee_i2h_msg_u {
    pub mh: bfi_mhdr,
    pub get_rsp: bfi_cee_get_rsp,
    pub stats_rsp: bfi_cee_stats_rsp,
    pub __packed: },
