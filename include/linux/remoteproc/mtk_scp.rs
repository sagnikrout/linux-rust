//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/remoteproc/mtk_scp.h
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
// Copyright (c) 2019 MediaTek Inc.
//

//
// enum ipi_id - the id of inter-processor interrupt
//
// @SCP_IPI_INIT:	 The interrupt from scp is to notfiy kernel
// SCP initialization completed.
// IPI_SCP_INIT is sent from SCP when firmware is
// loaded. AP doesn't need to send IPI_SCP_INIT
// command to SCP.
// For other IPI below, AP should send the request
// to SCP to trigger the interrupt.
// @SCP_IPI_MAX:	 The maximum IPI number
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scp_ipi_id {
    SCP_IPI_INIT = 0,
    SCP_IPI_VDEC_H264,
    SCP_IPI_VDEC_VP8,
    SCP_IPI_VDEC_VP9,
    SCP_IPI_VENC_H264,
    SCP_IPI_VENC_VP8,
    SCP_IPI_MDP_INIT,
    SCP_IPI_MDP_DEINIT,
    SCP_IPI_MDP_FRAME,
    SCP_IPI_DIP,
    SCP_IPI_ISP_CMD,
    SCP_IPI_ISP_FRAME,
    SCP_IPI_FD_CMD,
    SCP_IPI_CROS_HOST_CMD,
    SCP_IPI_VDEC_LAT,
    SCP_IPI_VDEC_CORE,
    SCP_IPI_IMGSYS_CMD,
    SCP_IPI_NS_SERVICE = 0xFF,
    SCP_IPI_MAX = 0x100,
}

extern "C" {
    pub fn scp_put(scp: *mut mtk_scp);
}
extern "C" {
    pub fn scp_ipi_unregister(scp: *mut mtk_scp, id: u32);
}
extern "C" {
    pub fn scp_get_vdec_hw_capa(scp: *mut mtk_scp) -> c_uint;
}
extern "C" {
    pub fn scp_get_venc_hw_capa(scp: *mut mtk_scp) -> c_uint;
}
