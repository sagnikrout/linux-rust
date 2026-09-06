//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/firmware/qcom/qcom_scm.h
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
// Copyright (c) 2010-2015, 2018-2019 The Linux Foundation. All rights reserved.
// Copyright (C) 2015 Linaro Ltd.
//

pub const QCOM_SCM_CPU_PWR_DOWN_L2_ON: c_uint = 0x0;
pub const QCOM_SCM_CPU_PWR_DOWN_L2_OFF: c_uint = 0x1;
pub const QCOM_SCM_HDCP_MAX_REQ_CNT: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_scm_hdcp_req {
    pub addr: u32,
    pub val: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_scm_vmperm {
    pub vmid: c_int,
    pub perm: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qcom_scm_ocmem_client {
    QCOM_SCM_OCMEM_UNUSED_ID = 0x0,
    QCOM_SCM_OCMEM_GRAPHICS_ID,
    QCOM_SCM_OCMEM_VIDEO_ID,
    QCOM_SCM_OCMEM_LP_AUDIO_ID,
    QCOM_SCM_OCMEM_SENSORS_ID,
    QCOM_SCM_OCMEM_OTHER_OS_ID,
    QCOM_SCM_OCMEM_DEBUG_ID,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qcom_scm_sec_dev_id {
    QCOM_SCM_MDSS_DEV_ID    = 1,
    QCOM_SCM_OCMEM_DEV_ID   = 5,
    QCOM_SCM_PCIE0_DEV_ID   = 11,
    QCOM_SCM_PCIE1_DEV_ID   = 12,
    QCOM_SCM_GFX_DEV_ID     = 18,
    QCOM_SCM_UFS_DEV_ID     = 19,
    QCOM_SCM_ICE_DEV_ID     = 20,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qcom_scm_ice_cipher {
    QCOM_SCM_ICE_CIPHER_AES_128_XTS = 0,
    QCOM_SCM_ICE_CIPHER_AES_128_CBC = 1,
    QCOM_SCM_ICE_CIPHER_AES_256_XTS = 3,
    QCOM_SCM_ICE_CIPHER_AES_256_CBC = 4,
}

pub const QCOM_SCM_PERM_READ: c_uint = 0x4;
pub const QCOM_SCM_PERM_WRITE: c_uint = 0x2;
pub const QCOM_SCM_PERM_EXEC: c_uint = 0x1;

extern "C" {
    pub fn qcom_scm_is_available() -> bool;
}
extern "C" {
    pub fn qcom_scm_set_cold_boot_addr(entry: *mut c_void) -> c_int;
}
extern "C" {
    pub fn qcom_scm_set_warm_boot_addr(entry: *mut c_void) -> c_int;
}
extern "C" {
    pub fn qcom_scm_cpu_power_down(flags: u32);
}
extern "C" {
    pub fn qcom_scm_set_remote_state(state: u32, id: u32) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_scm_pas_context {
    pub dev: *mut device,
    pub pas_id: u32,
    pub mem_phys: phys_addr_t,
    pub mem_size: usize,
    pub ptr: *mut c_void,
    pub phys: dma_addr_t,
    pub size: isize,
    pub use_tzmem: bool,
}

extern "C" {
    pub fn qcom_scm_pas_metadata_release(ctx: *mut qcom_scm_pas_context);
}
extern "C" {
    pub fn qcom_scm_pas_mem_setup(pas_id: u32, addr: phys_addr_t, size: phys_addr_t) -> c_int;
}
extern "C" {
    pub fn qcom_scm_pas_auth_and_reset(pas_id: u32) -> c_int;
}
extern "C" {
    pub fn qcom_scm_pas_shutdown(pas_id: u32) -> c_int;
}
extern "C" {
    pub fn qcom_scm_pas_supported(pas_id: u32) -> bool;
}
extern "C" {
    pub fn qcom_scm_pas_prepare_and_auth_reset(ctx: *mut qcom_scm_pas_context) -> c_int;
}
extern "C" {
    pub fn qcom_scm_io_readl(addr: phys_addr_t, val: *mut c_uint) -> c_int;
}
extern "C" {
    pub fn qcom_scm_io_writel(addr: phys_addr_t, val: c_uint) -> c_int;
}
extern "C" {
    pub fn qcom_scm_restore_sec_cfg_available() -> bool;
}
extern "C" {
    pub fn qcom_scm_restore_sec_cfg(device_id: u32, spare: u32) -> c_int;
}
extern "C" {
    pub fn qcom_scm_set_gpu_smmu_aperture(context_bank: c_uint) -> c_int;
}
extern "C" {
    pub fn qcom_scm_set_gpu_smmu_aperture_is_available() -> bool;
}
extern "C" {
    pub fn qcom_scm_iommu_secure_ptbl_size(spare: u32, size: *mut usize) -> c_int;
}
extern "C" {
    pub fn qcom_scm_iommu_secure_ptbl_init(addr: u64, size: u32, spare: u32) -> c_int;
}
extern "C" {
    pub fn qcom_scm_iommu_set_cp_pool_size(spare: u32, size: u32) -> c_int;
}
extern "C" {
    pub fn qcom_scm_ocmem_lock_available() -> bool;
}
extern "C" {
    pub fn qcom_scm_ocmem_unlock(id: qcom_scm_ocmem_client, offset: u32, size: u32) -> c_int;
}
extern "C" {
    pub fn qcom_scm_ice_available() -> bool;
}
extern "C" {
    pub fn qcom_scm_ice_invalidate_key(index: u32) -> c_int;
}
extern "C" {
    pub fn qcom_scm_has_wrapped_key_support() -> bool;
}
extern "C" {
    pub fn qcom_scm_generate_ice_key(lt_key: *mut u8, lt_key_size: usize) -> c_int;
}
extern "C" {
    pub fn qcom_scm_hdcp_available() -> bool;
}
extern "C" {
    pub fn qcom_scm_hdcp_req(req: *mut qcom_scm_hdcp_req, req_cnt: u32, resp: *mut u32) -> c_int;
}
extern "C" {
    pub fn qcom_scm_iommu_set_pt_format(sec_id: u32, ctx_num: u32, pt_fmt: u32) -> c_int;
}
extern "C" {
    pub fn qcom_scm_qsmmu500_wait_safe_toggle(en: bool) -> c_int;
}
extern "C" {
    pub fn qcom_scm_lmh_profile_change(profile_id: u32) -> c_int;
}
extern "C" {
    pub fn qcom_scm_lmh_dcvsh_available() -> bool;
}
//
// Request TZ to program set of access controlled registers necessary
// irrespective of any features
//

//
// Request TZ to program BCL id to access controlled register when BCL is
// enabled
//

//
// Request TZ to program set of access controlled register for CLX feature
// when enabled
//

//
// Request TZ to program tsense ids to access controlled registers for reading
// gpu temperature sensors
//

extern "C" {
    pub fn qcom_scm_gpu_init_regs(gpu_req: u32) -> c_int;
}
extern "C" {
    pub fn qcom_scm_shm_bridge_delete(handle: u64) -> c_int;
}

extern "C" {
    pub fn qcom_scm_qseecom_app_get_id(app_name: *const c_char, app_id: *mut u32) -> c_int;
}

