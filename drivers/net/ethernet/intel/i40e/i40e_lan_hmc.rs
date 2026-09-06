//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/i40e/i40e_lan_hmc.h
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
// Copyright(c) 2013 - 2018 Intel Corporation.

// forward-declare the HW struct for the compiler
// HMC element context information
// Rx queue context data
//
// The sizes of the variables may be larger than needed due to crossing byte
// boundaries. If we do not have the width of the variable set to the correct
// size then we could end up shifting bits off the top of the variable when the
// variable is at the top of a byte and crosses over into the next byte.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_hmc_obj_rxq {
    pub head: u16,
    pub /: *mut *mut u16 cpuid; / bigger than needed, see above for reason,
    pub base: u64,
    pub qlen: u16,
pub const I40E_RXQ_CTX_DBUFF_SHIFT: c_int = 7;
    pub /: *mut *mut u16 dbuff; / bigger than needed, see above for reason,
pub const I40E_RXQ_CTX_HBUFF_SHIFT: c_int = 6;
    pub /: *mut *mut u16 hbuff; / bigger than needed, see above for reason,
    pub dtype: u8,
    pub dsize: u8,
    pub crcstrip: u8,
    pub fc_ena: u8,
    pub l2tsel: u8,
    pub hsplit_0: u8,
    pub hsplit_1: u8,
    pub showiv: u8,
    pub /: *mut *mut u32 rxmax; / bigger than needed, see above for reason,
    pub tphrdesc_ena: u8,
    pub tphwdesc_ena: u8,
    pub tphdata_ena: u8,
    pub tphhead_ena: u8,
    pub /: *mut *mut u16 lrxqthresh; / bigger than needed, see above for reason,
    pub /: *mut *mut u8 prefena; / NOTE: normally must be set to 1 at init,
}

// Tx queue context data
//
// The sizes of the variables may be larger than needed due to crossing byte
// boundaries. If we do not have the width of the variable set to the correct
// size then we could end up shifting bits off the top of the variable when the
// variable is at the top of a byte and crosses over into the next byte.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_hmc_obj_txq {
    pub head: u16,
    pub new_context: u8,
    pub base: u64,
    pub fc_ena: u8,
    pub timesync_ena: u8,
    pub fd_ena: u8,
    pub alt_vlan_ena: u8,
    pub thead_wb: u16,
    pub cpuid: u8,
    pub head_wb_ena: u8,
    pub qlen: u16,
    pub tphrdesc_ena: u8,
    pub tphrpacket_ena: u8,
    pub tphwdesc_ena: u8,
    pub head_wb_addr: u64,
    pub crc: u32,
    pub rdylist: u16,
    pub rdylist_act: u8,
}

// for hsplit_0 field of Rx HMC context
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i40e_hmc_obj_rx_hsplit_0 {
    I40E_HMC_OBJ_RX_HSPLIT_0_NO_SPLIT      = 0,
    I40E_HMC_OBJ_RX_HSPLIT_0_SPLIT_L2      = 1,
    I40E_HMC_OBJ_RX_HSPLIT_0_SPLIT_IP      = 2,
    I40E_HMC_OBJ_RX_HSPLIT_0_SPLIT_TCP_UDP = 4,
    I40E_HMC_OBJ_RX_HSPLIT_0_SPLIT_SCTP    = 8,
}

// fcoe_cntx and fcoe_filt are for debugging purpose only
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_hmc_obj_fcoe_cntx {
    pub rsv: [u32; 32],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_hmc_obj_fcoe_filt {
    pub rsv: [u32; 8],
}

// Context sizes for LAN objects
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i40e_hmc_lan_object_size {
    I40E_HMC_LAN_OBJ_SZ_8   = 0x3,
    I40E_HMC_LAN_OBJ_SZ_16  = 0x4,
    I40E_HMC_LAN_OBJ_SZ_32  = 0x5,
    I40E_HMC_LAN_OBJ_SZ_64  = 0x6,
    I40E_HMC_LAN_OBJ_SZ_128 = 0x7,
    I40E_HMC_LAN_OBJ_SZ_256 = 0x8,
    I40E_HMC_LAN_OBJ_SZ_512 = 0x9,
}

pub const I40E_HMC_L2OBJ_BASE_ALIGNMENT: c_int = 512;
pub const I40E_HMC_OBJ_SIZE_TXQ: c_int = 128;
pub const I40E_HMC_OBJ_SIZE_RXQ: c_int = 32;
pub const I40E_HMC_OBJ_SIZE_FCOE_CNTX: c_int = 64;
pub const I40E_HMC_OBJ_SIZE_FCOE_FILT: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i40e_hmc_lan_rsrc_type {
    I40E_HMC_LAN_FULL  = 0,
    I40E_HMC_LAN_TX    = 1,
    I40E_HMC_LAN_RX    = 2,
    I40E_HMC_FCOE_CTX  = 3,
    I40E_HMC_FCOE_FILT = 4,
    I40E_HMC_LAN_MAX   = 5
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i40e_hmc_model {
    I40E_HMC_MODEL_DIRECT_PREFERRED = 0,
    I40E_HMC_MODEL_DIRECT_ONLY      = 1,
    I40E_HMC_MODEL_PAGED_ONLY       = 2,
    I40E_HMC_MODEL_UNKNOWN,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_hmc_lan_create_obj_info {
    pub hmc_info: *mut i40e_hmc_info,
    pub rsrc_type: u32,
    pub start_idx: u32,
    pub count: u32,
    pub entry_type: i40e_sd_entry_type,
    pub direct_mode_sz: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_hmc_lan_delete_obj_info {
    pub hmc_info: *mut i40e_hmc_info,
    pub rsrc_type: u32,
    pub start_idx: u32,
    pub count: u32,
}

extern "C" {
    pub fn i40e_shutdown_lan_hmc(hw: *mut i40e_hw) -> c_int;
}
