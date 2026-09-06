//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvfw/pmu.h
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


#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv_pmu_args {
    pub reserved: u32,
    pub freq_hz: u32,
    pub trace_size: u32,
    pub trace_dma_base: u32,
    pub trace_dma_base1: u16,
    pub trace_dma_offset: u8,
    pub trace_dma_idx: u32,
    pub secure_mode: bool,
    pub raise_priv_sec: bool,
    pub dma_base: u32,
    pub dma_base1: u16,
    pub dma_offset: u8,
    pub fb_size: u16,
    pub dma_idx: u8,
    pub gc6_ctx: },
    pub pad: u8,
}

pub const NV_PMU_UNIT_INIT: c_uint = 0x07;
pub const NV_PMU_UNIT_ACR: c_uint = 0x0a;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv_pmu_init_msg {
    pub hdr: nvfw_falcon_msg,
pub const NV_PMU_INIT_MSG_INIT: c_uint = 0x00;
    pub msg_type: u8,
    pub pad: u8,
    pub os_debug_entry_point: u16,
    pub size: u16,
    pub offset: u16,
    pub index: u8,
    pub pad: u8,
    pub queue_info: [}; 5],
    pub sw_managed_area_offset: u16,
    pub sw_managed_area_size: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv_pmu_acr_cmd {
    pub hdr: nvfw_falcon_cmd,
pub const NV_PMU_ACR_CMD_INIT_WPR_REGION: c_uint = 0x00;
pub const NV_PMU_ACR_CMD_BOOTSTRAP_FALCON: c_uint = 0x01;
pub const NV_PMU_ACR_CMD_BOOTSTRAP_MULTIPLE_FALCONS: c_uint = 0x03;
    pub cmd_type: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv_pmu_acr_msg {
    pub hdr: nvfw_falcon_cmd,
    pub msg_type: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv_pmu_acr_init_wpr_region_cmd {
    pub cmd: nv_pmu_acr_cmd,
    pub region_id: u32,
    pub wpr_offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv_pmu_acr_init_wpr_region_msg {
    pub msg: nv_pmu_acr_msg,
    pub error_code: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv_pmu_acr_bootstrap_falcon_cmd {
    pub cmd: nv_pmu_acr_cmd,
pub const NV_PMU_ACR_BOOTSTRAP_FALCON_FLAGS_RESET_YES: c_uint = 0x00000000;
pub const NV_PMU_ACR_BOOTSTRAP_FALCON_FLAGS_RESET_NO: c_uint = 0x00000001;
    pub flags: u32,
    pub falcon_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv_pmu_acr_bootstrap_falcon_msg {
    pub msg: nv_pmu_acr_msg,
    pub falcon_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv_pmu_acr_bootstrap_multiple_falcons_cmd {
    pub cmd: nv_pmu_acr_cmd,
pub const NV_PMU_ACR_BOOTSTRAP_MULTIPLE_FALCONS_FLAGS_RESET_YES: c_uint = 0x00000000;
pub const NV_PMU_ACR_BOOTSTRAP_MULTIPLE_FALCONS_FLAGS_RESET_NO: c_uint = 0x00000001;
    pub flags: u32,
    pub falcon_mask: u32,
    pub use_va_mask: u32,
    pub wpr_lo: u32,
    pub wpr_hi: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv_pmu_acr_bootstrap_multiple_falcons_msg {
    pub msg: nv_pmu_acr_msg,
    pub falcon_mask: u32,
}
