//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvfw/sec2.h
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
pub struct nv_sec2_args {
    pub freq_hz: u32,
    pub falc_trace_size: u32,
    pub falc_trace_dma_base: u32,
    pub falc_trace_dma_idx: u32,
    pub secure_mode: bool,
}

pub const NV_SEC2_UNIT_INIT: c_uint = 0x01;
pub const NV_SEC2_UNIT_UNLOAD: c_uint = 0x06;
pub const NV_SEC2_UNIT_ACR: c_uint = 0x08;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv_sec2_init_msg {
    pub hdr: nvfw_falcon_msg,
pub const NV_SEC2_INIT_MSG_INIT: c_uint = 0x00;
    pub msg_type: u8,
    pub num_queues: u8,
    pub os_debug_entry_point: u16,
    pub offset: u32,
    pub size: u16,
    pub index: u8,
pub const NV_SEC2_INIT_MSG_QUEUE_ID_CMDQ: c_uint = 0x00;
pub const NV_SEC2_INIT_MSG_QUEUE_ID_MSGQ: c_uint = 0x01;
    pub id: u8,
    pub queue_info: [}; 2],
    pub sw_managed_area_offset: u32,
    pub sw_managed_area_size: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv_sec2_init_msg_v1 {
    pub hdr: nvfw_falcon_msg,
pub const NV_SEC2_INIT_MSG_INIT: c_uint = 0x00;
    pub msg_type: u8,
    pub num_queues: u8,
    pub os_debug_entry_point: u16,
    pub offset: u32,
    pub size: u16,
    pub index: u8,
pub const NV_SEC2_INIT_MSG_QUEUE_ID_CMDQ: c_uint = 0x00;
pub const NV_SEC2_INIT_MSG_QUEUE_ID_MSGQ: c_uint = 0x01;
    pub id: u8,
    pub queue_info: [}; 2],
    pub sw_managed_area_offset: u32,
    pub sw_managed_area_size: u16,
    pub unkn: [u32; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv_sec2_acr_cmd {
    pub hdr: nvfw_falcon_cmd,
pub const NV_SEC2_ACR_CMD_BOOTSTRAP_FALCON: c_uint = 0x00;
    pub cmd_type: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv_sec2_acr_msg {
    pub hdr: nvfw_falcon_cmd,
    pub msg_type: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv_sec2_acr_bootstrap_falcon_cmd {
    pub cmd: nv_sec2_acr_cmd,
pub const NV_SEC2_ACR_BOOTSTRAP_FALCON_FLAGS_RESET_YES: c_uint = 0x00000000;
pub const NV_SEC2_ACR_BOOTSTRAP_FALCON_FLAGS_RESET_NO: c_uint = 0x00000001;
    pub flags: u32,
    pub falcon_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv_sec2_acr_bootstrap_falcon_msg {
    pub msg: nv_sec2_acr_msg,
    pub error_code: u32,
    pub falcon_id: u32,
}

pub const NV_SEC2_UNIT_V2_INIT: c_uint = 0x01;
pub const NV_SEC2_UNIT_V2_UNLOAD: c_uint = 0x05;
pub const NV_SEC2_UNIT_V2_ACR: c_uint = 0x07;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv_sec2_acr_bootstrap_falcon_cmd_v1 {
    pub cmd: nv_sec2_acr_cmd,
pub const NV_SEC2_ACR_BOOTSTRAP_FALCON_FLAGS_RESET_YES: c_uint = 0x00000000;
pub const NV_SEC2_ACR_BOOTSTRAP_FALCON_FLAGS_RESET_NO: c_uint = 0x00000001;
    pub flags: u32,
    pub falcon_id: u32,
    pub unkn08: u32,
    pub unkn0c: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv_sec2_acr_bootstrap_falcon_msg_v1 {
    pub msg: nv_sec2_acr_msg,
    pub error_code: u32,
    pub falcon_id: u32,
    pub unkn08: u32,
}
