//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/sof/intel/shim.h
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
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2017 Intel Corporation
//
// Author: Liam Girdwood <liam.r.girdwood@linux.intel.com>
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sof_intel_hw_ip_version {
    SOF_INTEL_TANGIER,
    SOF_INTEL_BAYTRAIL,
    SOF_INTEL_BROADWELL,
    SOF_INTEL_CAVS_1_5,	/* SkyLake, KabyLake, AmberLake */
    SOF_INTEL_CAVS_1_5_PLUS,/* ApolloLake, GeminiLake */
    SOF_INTEL_CAVS_1_8,	/* CannonLake, CometLake, CoffeeLake */
    SOF_INTEL_CAVS_2_0,	/* IceLake, JasperLake */
    SOF_INTEL_CAVS_2_5,	/* TigerLake, AlderLake */
    SOF_INTEL_ACE_1_0,	/* MeteorLake */
    SOF_INTEL_ACE_2_0,	/* LunarLake */
    SOF_INTEL_ACE_3_0,	/* PantherLake */
    SOF_INTEL_ACE_4_0,	/* NovaLake */
}

//
// SHIM registers for BYT, BSW, CHT, BDW
//

pub const SHIM_PWMCTRL: c_uint = 0x1000;
//
// SST SHIM register bits for BYT, BSW, CHT, BDW
// Register bit naming and functionaility can differ between devices.
//
// CSR / CS

// ISRX / ISC

// ISRD / ISD

// IMRX / IMC

// IMRD / IMD

// IPCX / IPCC

// IPCD

// CLKCTL

// CSR2 / CS2

// LTRC

// HMDC

pub const SHIM_HMDC_HDDA_E0_CH0: c_int = 1;
pub const SHIM_HMDC_HDDA_E0_CH1: c_int = 2;
pub const SHIM_HMDC_HDDA_E0_CH2: c_int = 4;
pub const SHIM_HMDC_HDDA_E0_CH3: c_int = 8;

// Audio DSP PCI registers
pub const PCI_VDRTCTL0: c_uint = 0xa0;
pub const PCI_VDRTCTL1: c_uint = 0xa4;
pub const PCI_VDRTCTL2: c_uint = 0xa8;
pub const PCI_VDRTCTL3: c_uint = 0xaC;
// VDRTCTL0

pub const PCI_VDRTCL0_DSRAMPGE_SHIFT: c_int = 12;

pub const PCI_VDRTCL0_ISRAMPGE_SHIFT: c_int = 2;

// VDRTCTL2

// PMCS
pub const PCI_PMCS: c_uint = 0x84;
pub const PCI_PMCS_PS_MASK: c_uint = 0x3;
// Intel quirks

// DSP hardware descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_intel_dsp_desc {
    pub cores_num: c_int,
    pub host_managed_cores_mask: c_int,
    pub /: *mut *mut int init_core_mask; / cores available after fw boot,
    pub ipc_req: c_int,
    pub ipc_req_mask: c_int,
    pub ipc_ack: c_int,
    pub ipc_ack_mask: c_int,
    pub ipc_ctl: c_int,
    pub rom_status_reg: c_int,
    pub rom_init_timeout: c_int,
    pub /: *mut *mut int ssp_count; / ssp count of the platform,
    pub /: *mut *mut int ssp_base_offset; / base address of the SSPs,
    pub sdw_shim_base: u32,
    pub sdw_alh_base: u32,
    pub d0i3_offset: u32,
    pub quirks: u32,
    pub platform: *const c_char,
    pub hw_ip_version: sof_intel_hw_ip_version,
    pub sdev): *mut *mut int (read_sdw_lcount)(struct snd_sof_dev,
    pub enable): *mut *mut *mut void (enable_sdw_irq)(struct snd_sof_dev sdev, bool,
    pub sdev): *mut *mut bool (check_sdw_irq)(struct snd_sof_dev,
    pub sdev): *mut *mut bool (check_sdw_wakeen_irq)(struct snd_sof_dev,
    pub sdev): *mut *mut void (sdw_process_wakeen)(struct snd_sof_dev,
    pub sdev): *mut *mut bool (check_ipc_irq)(struct snd_sof_dev,
    pub elid): *mut *mut *mut bool (check_mic_privacy_irq)(struct snd_sof_dev sdev, bool alt, int,
    pub elid): *mut *mut *mut void (process_mic_privacy)(struct snd_sof_dev sdev, bool alt, int,
    pub sdev): *mut *mut int (power_down_dsp)(struct snd_sof_dev,
    pub sdev): *mut *mut int (disable_interrupts)(struct snd_sof_dev,
    pub imr_boot): *mut *mut *mut int (cl_init)(struct snd_sof_dev sdev, int stream_tag, bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_intel_stream {
    pub posn_offset: usize,
}
