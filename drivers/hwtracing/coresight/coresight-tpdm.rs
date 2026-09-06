//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hwtracing/coresight/coresight-tpdm.h
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
// Copyright (c) 2023-2025 Qualcomm Innovation Center, Inc. All rights reserved.
//
// The max number of the datasets that TPDM supports
pub const TPDM_DATASETS: c_int = 7;
// CMB/MCMB Subunit Registers

// CMB subunit timestamp insertion enable register

// CMB subunit timestamp pattern registers

// CMB subunit timestamp pattern mask registers

// CMB subunit trigger pattern registers

// CMB subunit trigger pattern mask registers

// CMB MSR register

// Enable bit for CMB subunit

// Trace collection mode for CMB subunit

// MCMB trigger lane select

// MCMB lane enablement

// Timestamp control for pattern match

// CMB CTI timestamp request

// For timestamp fo all trace

// Patten register number
pub const TPDM_CMB_MAX_PATT: c_int = 2;
// MAX number of DSB MSR
pub const TPDM_CMB_MAX_MSR: c_int = 32;
// MAX lanes in the output pattern for MCMB configurations
pub const TPDM_MCMB_MAX_LANES: c_int = 8;
// Filter bit 0~7 from the value for CR_E_LN

// DSB Subunit Registers

// Enable bit for DSB subunit

// Enable bit for DSB subunit perfmance mode

// Enable bit for DSB subunit trigger type

// Data bits for DSB high performace mode

// Data bits for DSB test mode

// Enable bit for DSB subunit pattern timestamp

// Enable bit for DSB subunit trigger timestamp

// Bit for DSB subunit pattern type

// DSB programming modes
// DSB mode bits mask

// Test mode control bit

// Performance mode

// High performance mode

pub const EDCRS_PER_WORD: c_int = 16;

pub const EDCMRS_PER_WORD: c_int = 32;

// TPDM integration test registers

// Register value for integration test
pub const ATBCNTRL_VAL_32: c_uint = 0xC00F1409;
pub const ATBCNTRL_VAL_64: c_uint = 0xC01F1409;
//
// Number of cycles to write value when
// integration test.
//
pub const INTEGRATION_TEST_CYCLE: c_int = 10;
//
// The bits of PERIPHIDR0 register.
// The fields [6:0] of PERIPHIDR0 are used to determine what
// interfaces and subunits are present on a given TPDM.
//
// PERIPHIDR0[0] : Fix to 1 if ImplDef subunit present, else 0
// PERIPHIDR0[1] : Fix to 1 if DSB subunit present, else 0
// PERIPHIDR0[2] : Fix to 1 if CMB subunit present, else 0
// PERIPHIDR0[6] : Fix to 1 if MCMB subunit present, else 0
//

pub const TPDM_DSB_MAX_LINES: c_int = 256;
// MAX number of EDCR registers
pub const TPDM_DSB_MAX_EDCR: c_int = 16;
// MAX number of EDCMR registers
pub const TPDM_DSB_MAX_EDCMR: c_int = 8;
// MAX number of DSB pattern
pub const TPDM_DSB_MAX_PATT: c_int = 8;
// MAX number of DSB MSR
pub const TPDM_DSB_MAX_MSR: c_int = 32;

//
// struct dsb_dataset - specifics associated to dsb dataset
// @mode:             DSB programming mode
// @edge_ctrl_idx     Index number of the edge control
// @edge_ctrl:        Save value for edge control
// @edge_ctrl_mask:   Save value for edge control mask
// @patt_val:         Save value for pattern
// @patt_mask:        Save value for pattern mask
// @trig_patt:        Save value for trigger pattern
// @trig_patt_mask:   Save value for trigger pattern mask
// @msr               Save value for MSR
// @patt_ts:          Enable/Disable pattern timestamp
// @patt_type:        Set pattern type
// @trig_ts:          Enable/Disable trigger timestamp.
// @trig_type:        Enable/Disable trigger type.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsb_dataset {
    pub mode: u32,
    pub edge_ctrl_idx: u32,
    pub edge_ctrl: [u32; TPDM_DSB_MAX_EDCR],
    pub edge_ctrl_mask: [u32; TPDM_DSB_MAX_EDCMR],
    pub patt_val: [u32; TPDM_DSB_MAX_PATT],
    pub patt_mask: [u32; TPDM_DSB_MAX_PATT],
    pub trig_patt: [u32; TPDM_DSB_MAX_PATT],
    pub trig_patt_mask: [u32; TPDM_DSB_MAX_PATT],
    pub msr: [u32; TPDM_DSB_MAX_MSR],
    pub patt_ts: bool,
    pub patt_type: bool,
    pub trig_ts: bool,
    pub trig_type: bool,
}

//
// struct cmb_dataset
// @trace_mode:       Dataset collection mode
// @patt_val:         Save value for pattern
// @patt_mask:        Save value for pattern mask
// @trig_patt:        Save value for trigger pattern
// @trig_patt_mask:   Save value for trigger pattern mask
// @msr               Save value for MSR
// @patt_ts:          Indicates if pattern match for timestamp is enabled.
// @trig_ts:          Indicates if CTI trigger for timestamp is enabled.
// @ts_all:           Indicates if timestamp is enabled for all packets.
// struct mcmb_dataset
// @mcmb_trig_lane:       Save data for trigger lane
// @mcmb_lane_select:     Save data for lane enablement
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmb_dataset {
    pub trace_mode: u32,
    pub patt_val: [u32; TPDM_CMB_MAX_PATT],
    pub patt_mask: [u32; TPDM_CMB_MAX_PATT],
    pub trig_patt: [u32; TPDM_CMB_MAX_PATT],
    pub trig_patt_mask: [u32; TPDM_CMB_MAX_PATT],
    pub msr: [u32; TPDM_CMB_MAX_MSR],
    pub patt_ts: bool,
    pub trig_ts: bool,
    pub ts_all: bool,
    pub trig_lane: u8,
    pub lane_select: u8,
    pub mcmb: },
}

//
// struct tpdm_drvdata - specifics associated to an TPDM component
// @base:       memory mapped base address for this component.
// @dev:        The device entity associated to this component.
// @csdev:      component vitals needed by the framework.
// @spinlock:   lock for the drvdata value.
// @enable:     enable status of the component.
// @datasets:   The datasets types present of the TPDM.
// @dsb         Specifics associated to TPDM DSB.
// @cmb         Specifics associated to TPDM CMB.
// @dsb_msr_num Number of MSR supported by DSB TPDM
// @cmb_msr_num Number of MSR supported by CMB TPDM
// @traceid	Trace ID of the path.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpdm_drvdata {
    pub base: *mut void __iomem,
    pub dev: *mut device,
    pub csdev: *mut coresight_device,
    pub spinlock: spinlock_t,
    pub enable: bool,
    pub datasets: c_ulong,
    pub dsb: *mut dsb_dataset,
    pub cmb: *mut cmb_dataset,
    pub dsb_msr_num: u32,
    pub cmb_msr_num: u32,
    pub traceid: u8,
}

// Enumerate members of various datasets
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dataset_mem {
    DSB_EDGE_CTRL,
    DSB_EDGE_CTRL_MASK,
    DSB_TRIG_PATT,
    DSB_TRIG_PATT_MASK,
    DSB_PATT,
    DSB_PATT_MASK,
    DSB_MSR,
    CMB_TRIG_PATT,
    CMB_TRIG_PATT_MASK,
    CMB_PATT,
    CMB_PATT_MASK,
    CMB_MSR
}

//
// struct tpdm_dataset_attribute - Record the member variables and
// index number of datasets that need to be operated by sysfs file
// @attr:       The device attribute
// @mem:        The member in the dataset data structure
// @idx:        The index number of the array data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpdm_dataset_attribute {
    pub attr: device_attribute,
    pub mem: dataset_mem,
    pub idx: u32,
}
