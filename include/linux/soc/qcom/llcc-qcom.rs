//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/soc/qcom/llcc-qcom.h
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
//
// Copyright (c) 2017-2018, The Linux Foundation. All rights reserved.
//

pub const LLCC_CPUSS: c_int = 1;
pub const LLCC_VIDSC0: c_int = 2;
pub const LLCC_VIDSC1: c_int = 3;
pub const LLCC_ROTATOR: c_int = 4;
pub const LLCC_VOICE: c_int = 5;
pub const LLCC_AUDIO: c_int = 6;
pub const LLCC_MDMHPGRW: c_int = 7;
pub const LLCC_MDM: c_int = 8;
pub const LLCC_MODHW: c_int = 9;
pub const LLCC_CMPT: c_int = 10;
pub const LLCC_GPUHTW: c_int = 11;
pub const LLCC_GPU: c_int = 12;
pub const LLCC_MMUHWT: c_int = 13;
pub const LLCC_CMPTDMA: c_int = 15;
pub const LLCC_DISP: c_int = 16;
pub const LLCC_VIDFW: c_int = 17;
pub const LLCC_CAMFW: c_int = 18;
pub const LLCC_MDMHPFX: c_int = 20;
pub const LLCC_MDMPNG: c_int = 21;
pub const LLCC_AUDHW: c_int = 22;
pub const LLCC_NPU: c_int = 23;
pub const LLCC_WLHW: c_int = 24;
pub const LLCC_PIMEM: c_int = 25;
pub const LLCC_ECC: c_int = 26;
pub const LLCC_CVP: c_int = 28;
pub const LLCC_MODPE: c_int = 29;
pub const LLCC_APTCM: c_int = 30;
pub const LLCC_WRCACHE: c_int = 31;
pub const LLCC_CVPFW: c_int = 32;
pub const LLCC_CPUSS1: c_int = 33;
pub const LLCC_CAMEXP0: c_int = 34;
pub const LLCC_CPUMTE: c_int = 35;
pub const LLCC_CPUHWT: c_int = 36;
pub const LLCC_MDMCLAD2: c_int = 37;
pub const LLCC_CAMEXP1: c_int = 38;
pub const LLCC_CMPTHCP: c_int = 39;
pub const LLCC_LCPDARE: c_int = 40;
pub const LLCC_AENPU: c_int = 45;
pub const LLCC_ISLAND1: c_int = 46;
pub const LLCC_ISLAND2: c_int = 47;
pub const LLCC_ISLAND3: c_int = 48;
pub const LLCC_ISLAND4: c_int = 49;
pub const LLCC_CAMEXP2: c_int = 50;
pub const LLCC_CAMEXP3: c_int = 51;
pub const LLCC_CAMEXP4: c_int = 52;
pub const LLCC_DISP_WB: c_int = 53;
pub const LLCC_DISP_1: c_int = 54;
pub const LLCC_VIEYE: c_int = 57;
pub const LLCC_VIDPTH: c_int = 58;
pub const LLCC_GPUMV: c_int = 59;
pub const LLCC_EVA_LEFT: c_int = 60;
pub const LLCC_EVA_RIGHT: c_int = 61;
pub const LLCC_EVAGAIN: c_int = 62;
pub const LLCC_VIPTH: c_int = 63;
pub const LLCC_VIDVSP: c_int = 64;
pub const LLCC_DISP_LEFT: c_int = 65;
pub const LLCC_DISP_RIGHT: c_int = 66;
pub const LLCC_EVCS_LEFT: c_int = 67;
pub const LLCC_EVCS_RIGHT: c_int = 68;
pub const LLCC_SPAD: c_int = 69;
pub const LLCC_VIDDEC: c_int = 70;
pub const LLCC_CAMOFE: c_int = 71;
pub const LLCC_CAMRTIP: c_int = 72;
pub const LLCC_CAMSRTIP: c_int = 73;
pub const LLCC_CAMRTRF: c_int = 74;
pub const LLCC_CAMSRTRF: c_int = 75;
pub const LLCC_OOBM_NS: c_int = 81;
pub const LLCC_OOBM_S: c_int = 82;
pub const LLCC_VIDEO_APV: c_int = 83;
pub const LLCC_COMPUTE1: c_int = 87;
pub const LLCC_CPUSS_OPP: c_int = 88;
pub const LLCC_CPUSSMPAM: c_int = 89;
pub const LLCC_VIDSC_VSP1: c_int = 91;
pub const LLCC_CAM_IPE_STROV: c_int = 92;
pub const LLCC_CAM_OFE_STROV: c_int = 93;
pub const LLCC_CPUSS_HEU: c_int = 94;
pub const LLCC_PCIE_TCU: c_int = 97;
pub const LLCC_MDM_PNG_FIXED: c_int = 100;
//
// struct llcc_slice_desc - Cache slice descriptor
// @slice_id: LLCC slice id
// @uid: Unique ID associated with the LLCC device
// @slice_size: Size allocated for the LLCC slice
// @refcount: Atomic counter to track activate/deactivate calls
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct llcc_slice_desc {
    pub slice_id: u32,
    pub uid: u32,
    pub slice_size: usize,
    pub refcount: refcount_t,
}

//
// struct llcc_edac_reg_data - LLCC EDAC registers data for each error type
// @name: Name of the error
// @reg_cnt: Number of registers
// @count_mask: Mask value to get the error count
// @ways_mask: Mask value to get the error ways
// @count_shift: Shift value to get the error count
// @ways_shift: Shift value to get the error ways
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct llcc_edac_reg_data {
    pub name: *mut c_char,
    pub reg_cnt: u32,
    pub count_mask: u32,
    pub ways_mask: u32,
    pub count_shift: u8,
    pub ways_shift: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct llcc_edac_reg_offset {
// LLCC TRP registers
    pub trp_ecc_error_status0: u32,
    pub trp_ecc_error_status1: u32,
    pub trp_ecc_sb_err_syn0: u32,
    pub trp_ecc_db_err_syn0: u32,
    pub trp_ecc_error_cntr_clear: u32,
    pub trp_interrupt_0_status: u32,
    pub trp_interrupt_0_clear: u32,
    pub trp_interrupt_0_enable: u32,
// LLCC Common registers
    pub cmn_status0: u32,
    pub cmn_interrupt_0_enable: u32,
    pub cmn_interrupt_2_enable: u32,
// LLCC DRP registers
    pub drp_ecc_error_cfg: u32,
    pub drp_ecc_error_cntr_clear: u32,
    pub drp_interrupt_status: u32,
    pub drp_interrupt_clear: u32,
    pub drp_interrupt_enable: u32,
    pub drp_ecc_error_status0: u32,
    pub drp_ecc_error_status1: u32,
    pub drp_ecc_sb_err_syn0: u32,
    pub drp_ecc_db_err_syn0: u32,
}

//
// struct llcc_drv_data - Data associated with the LLCC driver
// @dev: device back-pointer for this LLCC instance
// @regmaps: regmaps associated with the LLCC device
// @bcast_regmap: regmap associated with LLCC broadcast OR offset
// @bcast_and_regmap: regmap associated with LLCC broadcast AND offset
// @cfg: pointer to the data structure for slice configuration
// @edac_reg_offset: Offset of the LLCC EDAC registers
// @lock: mutex associated with each slice
// @cfg_size: size of the config data table
// @num_banks: Number of LLCC banks
// @ecc_irq: interrupt for LLCC cache error detection and reporting
// @ecc_irq_configured: 'True' if firmware has already configured the irq propagation
// @version: Indicates the LLCC version
// @desc: Array pointer of pre-allocated LLCC slice descriptors
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct llcc_drv_data {
    pub dev: *mut device,
    pub regmaps: *mut regmap,
    pub bcast_regmap: *mut regmap,
    pub bcast_and_regmap: *mut regmap,
    pub cfg: *const llcc_slice_config,
    pub edac_reg_offset: *const llcc_edac_reg_offset,
    pub lock: mutex,
    pub cfg_size: u32,
    pub num_banks: u32,
    pub ecc_irq: c_int,
    pub ecc_irq_configured: bool,
    pub version: u32,
    pub desc: *mut llcc_slice_desc,
}

//
// llcc_slice_getd - get LLCC slice descriptor
// @uid: usecase_id of the client
//
// llcc_slice_putd - LLCC slice descriptor
// @desc: Pointer to LLCC slice descriptor
//
extern "C" {
    pub fn llcc_slice_putd(desc: *mut llcc_slice_desc);
}
//
// llcc_get_slice_id - get slice id
// @desc: Pointer to LLCC slice descriptor
//
extern "C" {
    pub fn llcc_get_slice_id(desc: *mut llcc_slice_desc) -> c_int;
}
//
// llcc_get_slice_size - LLCC slice size
// @desc: Pointer to LLCC slice descriptor
//
extern "C" {
    pub fn llcc_get_slice_size(desc: *mut llcc_slice_desc) -> usize;
}
//
// llcc_slice_activate - Activate the LLCC slice
// @desc: Pointer to LLCC slice descriptor
//
extern "C" {
    pub fn llcc_slice_activate(desc: *mut llcc_slice_desc) -> c_int;
}
//
// llcc_slice_deactivate - Deactivate the LLCC slice
// @desc: Pointer to LLCC slice descriptor
//
extern "C" {
    pub fn llcc_slice_deactivate(desc: *mut llcc_slice_desc) -> c_int;
}

