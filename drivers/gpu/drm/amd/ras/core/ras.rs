//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/ras/core/ras.h
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


// SPDX-License-Identifier: MIT
//
// Copyright 2025 Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//

pub const RAS_GPU_PAGE_SHIFT: c_int = 12;

pub const RAS_CORE_RESET_GPU: c_uint = 0x10000;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ras_gpu_health_status {
    RAS_GPU_HEALTH_NONE = 0,
    RAS_GPU_HEALTH_USABLE = 1,
    RAS_GPU_RETIRED__ECC_REACH_THRESHOLD = 2,
    RAS_GPU_IN_BAD_STATUS = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ras_core_fw_feature_flags {
    RAS_CORE_FW_FEATURE_BIT__RAS_EEPROM = BIT_ULL(0),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ras_block_id {
    RAS_BLOCK_ID__UMC = 0,
    RAS_BLOCK_ID__SDMA,
    RAS_BLOCK_ID__GFX,
    RAS_BLOCK_ID__MMHUB,
    RAS_BLOCK_ID__ATHUB,
    RAS_BLOCK_ID__PCIE_BIF,
    RAS_BLOCK_ID__HDP,
    RAS_BLOCK_ID__XGMI_WAFL,
    RAS_BLOCK_ID__DF,
    RAS_BLOCK_ID__SMN,
    RAS_BLOCK_ID__SEM,
    RAS_BLOCK_ID__MP0,
    RAS_BLOCK_ID__MP1,
    RAS_BLOCK_ID__FUSE,
    RAS_BLOCK_ID__MCA,
    RAS_BLOCK_ID__VCN,
    RAS_BLOCK_ID__JPEG,
    RAS_BLOCK_ID__IH,
    RAS_BLOCK_ID__MPIO,

    RAS_BLOCK_ID__LAST
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ras_ecc_err_type {
    RAS_ECC_ERR__NONE                = 0,
    RAS_ECC_ERR__PARITY              = 1,
    RAS_ECC_ERR__SINGLE_CORRECTABLE  = 2,
    RAS_ECC_ERR__MULTI_UNCORRECTABLE = 4,
    RAS_ECC_ERR__POISON              = 8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ras_err_type {
    RAS_ERR_TYPE__UE = 0,
    RAS_ERR_TYPE__CE,
    RAS_ERR_TYPE__DE,
    RAS_ERR_TYPE__LAST
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ras_seqno_type {
    RAS_SEQNO_TYPE_INVALID = 0,
    RAS_SEQNO_TYPE_UE,
    RAS_SEQNO_TYPE_CE,
    RAS_SEQNO_TYPE_DE,
    RAS_SEQNO_TYPE_POISON_CONSUMPTION,
    RAS_SEQNO_TYPE_COUNT_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ras_seqno_fifo {
    SEQNO_FIFO_INVALID = 0,
    SEQNO_FIFO_POISON_CREATION,
    SEQNO_FIFO_POISON_CONSUMPTION,
    SEQNO_FIFO_COUNT_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ras_notify_event {
    RAS_EVENT_ID__NONE,
    RAS_EVENT_ID__BAD_PAGE_DETECTED,
    RAS_EVENT_ID__POISON_CONSUMPTION,
    RAS_EVENT_ID__RESERVE_BAD_PAGE,
    RAS_EVENT_ID__DEVICE_RMA,
    RAS_EVENT_ID__UPDATE_BAD_PAGE_NUM,
    RAS_EVENT_ID__UPDATE_BAD_CHANNEL_BITMAP,
    RAS_EVENT_ID__FATAL_ERROR_DETECTED,
    RAS_EVENT_ID__RESET_GPU,
    RAS_EVENT_ID__RESET_VF,
    RAS_EVENT_ID__RAS_EVENT_PROC_BEGIN,
    RAS_EVENT_ID__RAS_EVENT_PROC_END,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ras_gpu_status {
    RAS_GPU_STATUS__NOT_READY = 0,
    RAS_GPU_STATUS__READY = 0x1,
    RAS_GPU_STATUS__IN_RESET = 0x2,
    RAS_GPU_STATUS__IS_RMA = 0x4,
    RAS_GPU_STATUS__IS_VF = 0x8,
    RAS_GPU_STATUS__DEVICE_LOST = 0x10,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ras_fw_eeprom_cmd {
    RAS_SMU_GetRASTableVersion = 0,
    RAS_SMU_GetBadPageCount,
    RAS_SMU_SetTimestamp,
    RAS_SMU_GetTimestamp,
    RAS_SMU_GetBadPageIpid,
    RAS_SMU_EraseRasTable,
    RAS_SMU_GetBadPageMcaAddr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_mp1_sys_func {
    pub count): *mut u32 msg, u32,
    pub val): *mut u32 msg, u32 idx, u32 reg_idx, u64,
    pub read_arg): *mut ras_fw_eeprom_cmd index, uint32_t param, uint32_t,
    pub enabled_mask): *mut u64,
    pub enable): *mut *mut *mut int (mp1_set_debug_mode)(struct ras_core_context ras_core, bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_eeprom_sys_func {
    pub read): *mut *mut u32 eeprom_addr, u8 eeprom_buf, u32 buf_size, bool,
    pub ras_core): *mut *mut int (update_eeprom_i2c_config)(struct ras_core_context,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_nbio_sys_func {
    pub state): bool,
    pub state): bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_time {
    pub tm_sec: c_int,
    pub tm_min: c_int,
    pub tm_hour: c_int,
    pub tm_mday: c_int,
    pub tm_mon: c_int,
    pub tm_year: c_long,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct device_system_info {
    pub device_id: u32,
    pub vendor_id: u32,
    pub socket_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gpu_mem_type {
    GPU_MEM_TYPE_DEFAULT,
    GPU_MEM_TYPE_RAS_PSP_RING,
    GPU_MEM_TYPE_RAS_PSP_CMD,
    GPU_MEM_TYPE_RAS_PSP_FENCE,
    GPU_MEM_TYPE_RAS_TA_FW,
    GPU_MEM_TYPE_RAS_TA_CMD,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_psp_sys_func {
    pub status): *mut ras_psp_sys_status,
    pub ras_ta_param): *mut ras_ta_init_param,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_sys_func {
    pub try): bool down, bool,
    pub status): *mut u32,
    pub seqno): *mut ras_seqno_type seqno_type, uint64_t,
    pub data): *mut *mut *mut int (async_handle_ras_event)(struct ras_core_context ras_core, void,
    pub data): *mut ras_notify_event event_id, void,
    pub ras_core): *mut *mut u64 (get_utc_second_timestamp)(struct ras_core_context,
    pub dev_info): *mut device_system_info,
    pub ras_core): *mut *mut bool (detect_ras_interrupt)(struct ras_core_context,
    pub gpu_mem): *mut gpu_mem_type mem_type, struct gpu_mem_block,
    pub gpu_mem): *mut gpu_mem_type mem_type, struct gpu_mem_block,
    pub addr): *mut *mut *mut int (check_address_sanity)(struct ras_core_context ras_core, uint64_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_ecc_count {
    pub new_ce_count: u64,
    pub total_ce_count: u64,
    pub new_ue_count: u64,
    pub total_ue_count: u64,
    pub new_de_count: u64,
    pub total_de_count: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_bank_ecc {
    pub nps: u32,
    pub seq_no: u64,
    pub status: u64,
    pub ipid: u64,
    pub addr: u64,
    pub ts: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_bank_ecc_node {
    pub node: list_head,
    pub ecc: ras_bank_ecc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_aca_config {
    pub socket_num_per_hive: u32,
    pub aid_num_per_socket: u32,
    pub xcd_num_per_aid: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_mp1_config {
    pub mp1_sys_fn: *const ras_mp1_sys_func,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_nbio_config {
    pub nbio_sys_fn: *const ras_nbio_sys_func,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_psp_config {
    pub psp_sys_fn: *const ras_psp_sys_func,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_umc_config {
    pub umc_vram_type: u32,
    pub num_umc: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_eeprom_config {
    pub eeprom_sys_fn: *const ras_eeprom_sys_func,
    pub eeprom_record_threshold_config: c_int,
    pub eeprom_record_threshold_count: u32,
    pub eeprom_i2c_adapter: *mut c_void,
    pub eeprom_i2c_addr: u32,
    pub eeprom_i2c_port: u32,
    pub max_i2c_read_len: u16,
    pub max_i2c_write_len: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_core_config {
    pub aca_ip_version: u32,
    pub umc_ip_version: u32,
    pub mp1_ip_version: u32,
    pub gfx_ip_version: u32,
    pub nbio_ip_version: u32,
    pub psp_ip_version: u32,
    pub poison_supported: bool,
    pub ras_eeprom_supported: bool,
    pub ras_debug_mask: c_uint,
    pub sys_fn: *const ras_sys_func,
    pub aca_cfg: ras_aca_config,
    pub mp1_cfg: ras_mp1_config,
    pub nbio_cfg: ras_nbio_config,
    pub psp_cfg: ras_psp_config,
    pub eeprom_cfg: ras_eeprom_config,
    pub umc_cfg: ras_umc_config,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_core_context {
    pub dev: *mut c_void,
    pub config: *mut ras_core_config,
    pub socket_num_per_hive: u32,
    pub aid_num_per_socket: u32,
    pub xcd_num_per_aid: u32,
    pub max_ue_banks_per_query: c_int,
    pub max_ce_banks_per_query: c_int,
    pub ras_aca: ras_aca,
    pub ras_eeprom_supported: bool,
    pub ras_eeprom: ras_eeprom_control,
    pub ras_fw_eeprom: ras_fw_eeprom_control,
    pub ras_psp: ras_psp,
    pub ras_umc: ras_umc,
    pub ras_nbio: ras_nbio,
    pub ras_gfx: ras_gfx,
    pub ras_mp1: ras_mp1,
    pub ras_proc: ras_process,
    pub ras_log_ring: ras_log_ring,
    pub sys_fn: *const ras_sys_func,
// is poison mode supported
    pub poison_supported: bool,
    pub is_rma: bool,
    pub is_initialized: bool,
    pub de_seqno_fifo: kfifo,
    pub consumption_seqno_fifo: kfifo,
    pub seqno_lock: spinlock_t,
    pub ras_core_enabled: bool,
    pub ras_fw_features: u64,
}

extern "C" {
    pub fn ras_core_destroy(ras_core: *mut ras_core_context);
}
extern "C" {
    pub fn ras_core_sw_init(ras_core: *mut ras_core_context) -> c_int;
}
extern "C" {
    pub fn ras_core_sw_fini(ras_core: *mut ras_core_context) -> c_int;
}
extern "C" {
    pub fn ras_core_hw_init(ras_core: *mut ras_core_context) -> c_int;
}
extern "C" {
    pub fn ras_core_hw_fini(ras_core: *mut ras_core_context) -> c_int;
}
extern "C" {
    pub fn ras_core_is_ready(ras_core: *mut ras_core_context) -> bool;
}
extern "C" {
    pub fn ras_core_update_ecc_info(ras_core: *mut ras_core_context) -> c_int;
}
extern "C" {
    pub fn ras_core_gpu_in_reset(ras_core: *mut ras_core_context) -> bool;
}
extern "C" {
    pub fn ras_core_gpu_is_rma(ras_core: *mut ras_core_context) -> bool;
}
extern "C" {
    pub fn ras_core_gpu_is_vf(ras_core: *mut ras_core_context) -> bool;
}
extern "C" {
    pub fn ras_core_gpu_device_lost(ras_core: *mut ras_core_context) -> bool;
}
extern "C" {
    pub fn ras_core_handle_nbio_irq(ras_core: *mut ras_core_context, data: *mut c_void) -> bool;
}
extern "C" {
    pub fn ras_core_handle_fatal_error(ras_core: *mut ras_core_context) -> c_int;
}
extern "C" {
    pub fn ras_core_get_curr_nps_mode(ras_core: *mut ras_core_context) -> u32;
}
extern "C" {
    pub fn ras_core_set_status(ras_core: *mut ras_core_context, enable: bool) -> c_int;
}
extern "C" {
    pub fn ras_core_is_enabled(ras_core: *mut ras_core_context) -> bool;
}
extern "C" {
    pub fn ras_core_get_utc_second_timestamp(ras_core: *mut ras_core_context) -> u64;
}
extern "C" {
    pub fn ras_core_ras_interrupt_detected(ras_core: *mut ras_core_context) -> bool;
}
extern "C" {
    pub fn ras_core_check_safety_watermark(ras_core: *mut ras_core_context) -> bool;
}
extern "C" {
    pub fn ras_core_down_trylock_gpu_reset_lock(ras_core: *mut ras_core_context) -> c_int;
}
extern "C" {
    pub fn ras_core_down_gpu_reset_lock(ras_core: *mut ras_core_context);
}
extern "C" {
    pub fn ras_core_up_gpu_reset_lock(ras_core: *mut ras_core_context);
}
extern "C" {
    pub fn ras_core_check_address_sanity(ras_core: *mut ras_core_context, addr: u64) -> c_int;
}
extern "C" {
    pub fn ras_core_set_debug_mode(ras_core: *mut ras_core_context, enable: bool) -> c_int;
}
extern "C" {
    pub fn ras_core_is_ce_log_disabled(ras_core: *mut ras_core_context) -> bool;
}
