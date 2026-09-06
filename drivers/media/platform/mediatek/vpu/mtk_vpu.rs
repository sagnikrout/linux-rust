//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/mediatek/vpu/mtk_vpu.h
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
// Copyright (c) 2016 MediaTek Inc.
// Author: Andrew-CT Chen <andrew-ct.chen@mediatek.com>
//

//
// DOC: VPU
//
// VPU (video processor unit) is a tiny processor controlling video hardware
// related to video codec, scaling and color format converting.
// VPU interfaces with other blocks by share memory and interrupt.
//
// enum ipi_id - the id of inter-processor interrupt
//
// @IPI_VPU_INIT:	 The interrupt from vpu is to notfiy kernel
// VPU initialization completed.
// IPI_VPU_INIT is sent from VPU when firmware is
// loaded. AP doesn't need to send IPI_VPU_INIT
// command to VPU.
// For other IPI below, AP should send the request
// to VPU to trigger the interrupt.
// @IPI_VDEC_H264:	 The interrupt from vpu is to notify kernel to
// handle H264 vidoe decoder job, and vice versa.
// Decode output format is always MT21 no matter what
// the input format is.
// @IPI_VDEC_VP8:	 The interrupt from is to notify kernel to
// handle VP8 video decoder job, and vice versa.
// Decode output format is always MT21 no matter what
// the input format is.
// @IPI_VDEC_VP9:	 The interrupt from vpu is to notify kernel to
// handle VP9 video decoder job, and vice versa.
// Decode output format is always MT21 no matter what
// the input format is.
// @IPI_VENC_H264:	 The interrupt from vpu is to notify kernel to
// handle H264 video encoder job, and vice versa.
// @IPI_VENC_VP8:	 The interrupt fro vpu is to notify kernel to
// handle VP8 video encoder job,, and vice versa.
// @IPI_MDP:		 The interrupt from vpu is to notify kernel to
// handle MDP (Media Data Path) job, and vice versa.
// @IPI_MAX:		 The maximum IPI number
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipi_id {
    IPI_VPU_INIT = 0,
    IPI_VDEC_H264,
    IPI_VDEC_VP8,
    IPI_VDEC_VP9,
    IPI_VENC_H264,
    IPI_VENC_VP8,
    IPI_MDP,
    IPI_MAX,
}

//
// enum rst_id - reset id to register reset function for VPU watchdog timeout
//
// @VPU_RST_ENC: encoder reset id
// @VPU_RST_DEC: decoder reset id
// @VPU_RST_MDP: MDP (Media Data Path) reset id
// @VPU_RST_MAX: maximum reset id
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rst_id {
    VPU_RST_ENC,
    VPU_RST_DEC,
    VPU_RST_MDP,
    VPU_RST_MAX,
}

//
// vpu_ipi_register - register an ipi function
//
// @pdev:	VPU platform device
// @id:		IPI ID
// @handler:	IPI handler
// @name:	IPI name
// @priv:	private data for IPI handler
//
// Register an ipi function to receive ipi interrupt from VPU.
//
// Return: Return 0 if ipi registers successfully, otherwise it is failed.
//
// vpu_ipi_send - send data from AP to vpu.
//
// @pdev:	VPU platform device
// @id:		IPI ID
// @buf:	the data buffer
// @len:	the data buffer length
//
// This function is thread-safe. When this function returns,
// VPU has received the data and starts the processing.
// When the processing completes, IPI handler registered
// by vpu_ipi_register will be called in interrupt context.
//
// Return: Return 0 if sending data successfully, otherwise it is failed.
//
// vpu_get_plat_device - get VPU's platform device
//
// @pdev:	the platform device of the module requesting VPU platform
// device for using VPU API.
//
// Return: a reference to the VPU's platform device, or NULL on failure.
//
// vpu_wdt_reg_handler - register a VPU watchdog handler
//
// @pdev:               VPU platform device
// @vpu_wdt_reset_func():	the callback reset function
// @priv: the private data for reset function
// @priv:		the private data for reset function
// @id:			reset id
//
// Register a handler performing own tasks when vpu reset by watchdog
//
// Return: Return 0 if the handler is added successfully,
// otherwise it is failed.
//
// vpu_get_vdec_hw_capa - get video decoder hardware capability
//
// @pdev:	VPU platform device
//
// Return: video decoder hardware capability
//
extern "C" {
    pub fn vpu_get_vdec_hw_capa(pdev: *mut platform_device) -> c_uint;
}
//
// vpu_get_venc_hw_capa - get video encoder hardware capability
//
// @pdev:	VPU platform device
//
// Return: video encoder hardware capability
//
extern "C" {
    pub fn vpu_get_venc_hw_capa(pdev: *mut platform_device) -> c_uint;
}
//
// vpu_load_firmware - download VPU firmware and boot it
//
// @pdev:	VPU platform device
//
// Return: Return 0 if downloading firmware successfully,
// otherwise it is failed
//
extern "C" {
    pub fn vpu_load_firmware(pdev: *mut platform_device) -> c_int;
}
//
// vpu_mapping_dm_addr - Mapping DTCM/DMEM to kernel virtual address
//
// @pdev:		VPU platform device
// @dtcm_dmem_addr:	VPU's data memory address
//
// Mapping the VPU's DTCM (Data Tightly-Coupled Memory)
// DMEM (Data Extended Memory) memory address to
// kernel virtual address.
//
// Return: Return ERR_PTR(-EINVAL) if mapping failed,
// otherwise the mapped kernel virtual address
//
