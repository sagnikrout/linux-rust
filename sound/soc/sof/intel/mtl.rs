//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/sof/intel/mtl.h
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
// Copyright(c) 2020-2022 Intel Corporation
//
// DSP Registers
pub const MTL_HFDSSCS: c_uint = 0x1000;

pub const MTL_HFSNDWIE: c_uint = 0x114C;
pub const MTL_HFPWRCTL: c_uint = 0x1D18;
pub const PTL_HFPWRCTL2: c_uint = 0x1D20;

pub const MTL_HFPWRSTS: c_uint = 0x1D1C;
pub const PTL_HFPWRSTS2: c_uint = 0x1D24;

pub const MTL_HFINTIPPTR: c_uint = 0x1108;

pub const MTL_HDA_VS_D0I3C: c_uint = 0x1D4A;
pub const MTL_DSP2CXCAP_PRIMARY_CORE: c_uint = 0x178D00;
pub const MTL_DSP2CXCTL_PRIMARY_CORE: c_uint = 0x178D04;

pub const MTL_DSP2CXCTL_PRIMARY_CORE_OSEL_SHIFT: c_int = 24;
// IPC Registers
pub const MTL_DSP_REG_HFIPCXTDR: c_uint = 0x73200;

pub const MTL_DSP_REG_HFIPCXTDA: c_uint = 0x73204;

pub const MTL_DSP_REG_HFIPCXIDR: c_uint = 0x73210;

pub const MTL_DSP_REG_HFIPCXIDA: c_uint = 0x73214;

pub const MTL_DSP_REG_HFIPCXCTL: c_uint = 0x73228;

pub const MTL_DSP_REG_HFIPCXTDDY: c_uint = 0x73300;
pub const MTL_DSP_REG_HFIPCXIDDY: c_uint = 0x73380;
pub const MTL_DSP_REG_HfHIPCIE: c_uint = 0x1140;

pub const MTL_DSP_REG_HfSNDWIE: c_uint = 0x114C;

pub const MTL_DSP_IRQSTS: c_uint = 0x20;

// Memory windows

pub const MTL_DSP_MBOX_UPLINK_SIZE: c_uint = 0x1000;

pub const MTL_DSP_MBOX_DOWNLINK_SIZE: c_uint = 0x1000;
// FW registers

pub const MTL_DSP_REG_HFFLGPXQWY: c_uint = 0x163200 /* DSP core0 status */;
pub const MTL_DSP_REG_HFFLGPXQWY_ERROR: c_uint = 0x163204 /* DSP core0 error */;
// FSR status codes
pub const FSR_STATE_ROM_RESET_VECTOR_DONE: c_uint = 0x8;
pub const FSR_STATE_ROM_PURGE_BOOT: c_uint = 0x9;
pub const FSR_STATE_ROM_RESTORE_BOOT: c_uint = 0xA;
pub const FSR_STATE_ROM_FW_ENTRY_POINT: c_uint = 0xB;
pub const FSR_STATE_ROM_VALIDATE_PUB_KEY: c_uint = 0xC;
pub const FSR_STATE_ROM_POWER_DOWN_HPSRAM: c_uint = 0xD;
pub const FSR_STATE_ROM_POWER_DOWN_ULPSRAM: c_uint = 0xE;
pub const FSR_STATE_ROM_POWER_UP_ULPSRAM_STACK: c_uint = 0xF;
pub const FSR_STATE_ROM_POWER_UP_HPSRAM_DMA: c_uint = 0x10;
pub const FSR_STATE_ROM_BEFORE_EP_POINTER_READ: c_uint = 0x11;
pub const FSR_STATE_ROM_VALIDATE_MANIFEST: c_uint = 0x12;
pub const FSR_STATE_ROM_VALIDATE_FW_MODULE: c_uint = 0x13;
pub const FSR_STATE_ROM_PROTECT_IMR_REGION: c_uint = 0x14;
pub const FSR_STATE_ROM_PUSH_MODEL_ROUTINE: c_uint = 0x15;
pub const FSR_STATE_ROM_PULL_MODEL_ROUTINE: c_uint = 0x16;
pub const FSR_STATE_ROM_VALIDATE_PKG_DIR: c_uint = 0x17;
pub const FSR_STATE_ROM_VALIDATE_CPD: c_uint = 0x18;
pub const FSR_STATE_ROM_VALIDATE_CSS_MAN_HEADER: c_uint = 0x19;
pub const FSR_STATE_ROM_VALIDATE_BLOB_SVN: c_uint = 0x1A;
pub const FSR_STATE_ROM_VERIFY_IFWI_PARTITION: c_uint = 0x1B;
pub const FSR_STATE_ROM_REMOVE_ACCESS_CONTROL: c_uint = 0x1C;
pub const FSR_STATE_ROM_AUTH_BYPASS: c_uint = 0x1D;
pub const FSR_STATE_ROM_AUTH_ENABLED: c_uint = 0x1E;
pub const FSR_STATE_ROM_INIT_DMA: c_uint = 0x1F;
pub const FSR_STATE_ROM_PURGE_FW_ENTRY: c_uint = 0x20;
pub const FSR_STATE_ROM_PURGE_FW_END: c_uint = 0x21;
pub const FSR_STATE_ROM_CLEAN_UP_BSS_DONE: c_uint = 0x22;
pub const FSR_STATE_ROM_IMR_RESTORE_ENTRY: c_uint = 0x23;
pub const FSR_STATE_ROM_IMR_RESTORE_END: c_uint = 0x24;
pub const FSR_STATE_ROM_FW_MANIFEST_IN_DMA_BUFF: c_uint = 0x25;
pub const FSR_STATE_ROM_LOAD_CSE_MAN_TO_IMR: c_uint = 0x26;
pub const FSR_STATE_ROM_LOAD_FW_MAN_TO_IMR: c_uint = 0x27;
pub const FSR_STATE_ROM_LOAD_FW_CODE_TO_IMR: c_uint = 0x28;
pub const FSR_STATE_ROM_FW_LOADING_DONE: c_uint = 0x29;
pub const FSR_STATE_ROM_FW_CODE_LOADED: c_uint = 0x2A;
pub const FSR_STATE_ROM_VERIFY_IMAGE_TYPE: c_uint = 0x2B;
pub const FSR_STATE_ROM_AUTH_API_INIT: c_uint = 0x2C;
pub const FSR_STATE_ROM_AUTH_API_PROC: c_uint = 0x2D;
pub const FSR_STATE_ROM_AUTH_API_FIRST_BUSY: c_uint = 0x2E;
pub const FSR_STATE_ROM_AUTH_API_FIRST_RESULT: c_uint = 0x2F;
pub const FSR_STATE_ROM_AUTH_API_CLEANUP: c_uint = 0x30;
pub const MTL_DSP_REG_HfIMRIS1: c_uint = 0x162088;

extern "C" {
    pub fn mtl_dsp_check_ipc_irq(sdev: *mut snd_sof_dev) -> bool;
}
extern "C" {
    pub fn mtl_enable_ipc_interrupts(sdev: *mut snd_sof_dev);
}
extern "C" {
    pub fn mtl_disable_ipc_interrupts(sdev: *mut snd_sof_dev);
}
extern "C" {
    pub fn mtl_enable_interrupts(sdev: *mut snd_sof_dev, enable: bool) -> c_int;
}
extern "C" {
    pub fn mtl_power_down_dsp(sdev: *mut snd_sof_dev) -> c_int;
}
extern "C" {
    pub fn mtl_dsp_cl_init(sdev: *mut snd_sof_dev, stream_tag: c_int, imr_boot: bool) -> c_int;
}
extern "C" {
    pub fn sof_mtl_set_ops(sdev: *mut snd_sof_dev, dsp_ops: *mut snd_sof_dsp_ops) -> c_int;
}
