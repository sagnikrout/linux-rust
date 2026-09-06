//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/amdgpu_vcn.h
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


//
// Copyright 2016-2024 Advanced Micro Devices, Inc. All rights reserved.
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

pub const AMDGPU_VCN_FIRMWARE_OFFSET: c_int = 256;
pub const AMDGPU_VCN_MAX_ENC_RINGS: c_int = 3;
pub const AMDGPU_MAX_VCN_INSTANCES: c_int = 4;

pub const VCN_DEC_KMD_CMD: c_uint = 0x80000000;
pub const VCN_DEC_CMD_FENCE: c_uint = 0x00000000;
pub const VCN_DEC_CMD_TRAP: c_uint = 0x00000001;
pub const VCN_DEC_CMD_WRITE_REG: c_uint = 0x00000004;
pub const VCN_DEC_CMD_REG_READ_COND_WAIT: c_uint = 0x00000006;
pub const VCN_DEC_CMD_PACKET_START: c_uint = 0x0000000a;
pub const VCN_DEC_CMD_PACKET_END: c_uint = 0x0000000b;
pub const VCN_DEC_SW_CMD_NO_OP: c_uint = 0x00000000;
pub const VCN_DEC_SW_CMD_END: c_uint = 0x00000001;
pub const VCN_DEC_SW_CMD_IB: c_uint = 0x00000002;
pub const VCN_DEC_SW_CMD_FENCE: c_uint = 0x00000003;
pub const VCN_DEC_SW_CMD_TRAP: c_uint = 0x00000004;
pub const VCN_DEC_SW_CMD_IB_AUTO: c_uint = 0x00000005;
pub const VCN_DEC_SW_CMD_SEMAPHORE: c_uint = 0x00000006;
pub const VCN_DEC_SW_CMD_PREEMPT_FENCE: c_uint = 0x00000009;
pub const VCN_DEC_SW_CMD_REG_WRITE: c_uint = 0x0000000b;
pub const VCN_DEC_SW_CMD_REG_WAIT: c_uint = 0x0000000c;
pub const VCN_ENC_CMD_NO_OP: c_uint = 0x00000000;
pub const VCN_ENC_CMD_END: c_uint = 0x00000001;
pub const VCN_ENC_CMD_IB: c_uint = 0x00000002;
pub const VCN_ENC_CMD_FENCE: c_uint = 0x00000003;
pub const VCN_ENC_CMD_TRAP: c_uint = 0x00000004;
pub const VCN_ENC_CMD_REG_WRITE: c_uint = 0x0000000b;
pub const VCN_ENC_CMD_REG_WAIT: c_uint = 0x0000000c;
pub const VCN_AON_SOC_ADDRESS_2_0: c_uint = 0x1f800;
pub const VCN_VID_IP_ADDRESS_2_0: c_uint = 0x0;
pub const VCN_AON_IP_ADDRESS_2_0: c_uint = 0x30000;
pub const mmUVD_RBC_XX_IB_REG_CHECK: c_uint = 0x026b;
pub const mmUVD_RBC_XX_IB_REG_CHECK_BASE_IDX: c_int = 1;
pub const mmUVD_REG_XX_MASK: c_uint = 0x026c;
pub const mmUVD_REG_XX_MASK_BASE_IDX: c_int = 1;
// 1 second timeout

// To avoid a -Wunused-but-set-variable warning. */				\

// adev->vcn.inst[inst_idx].dpg_sram_curr_addr++ =              \

// To avoid a -Wunused-but-set-variable warning. */				\

// adev->vcn.inst[inst_idx].dpg_sram_curr_addr++ =		\

pub const MAX_NUM_VCN_RB_SETUP: c_int = 4;
pub const AMDGPU_VCN_IB_FLAG_DECODE_BUFFER: c_uint = 0x00000001;
pub const AMDGPU_VCN_CMD_FLAG_MSG_BUFFER: c_uint = 0x00000001;

pub const AMDGPU_DRM_KEY_INJECT_WORKAROUND_VCNFW_ASD_HANDSHAKING: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_vcn_caps {
    AMDGPU_VCN_RRMT_ENABLED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_queue_mode {
    FW_QUEUE_RING_RESET = 1,
    FW_QUEUE_DPG_HOLD_OFF = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum engine_status_constants {
    UVD_PGFSM_STATUS__UVDM_UVDU_PWR_ON = 0x2AAAA0,
    UVD_PGFSM_STATUS__UVDM_UVDU_PWR_ON_2_0 = 0xAAAA0,
    UVD_PGFSM_STATUS__UVDM_UVDU_UVDLM_PWR_ON_3_0 = 0x2A2A8AA0,
    UVD_PGFSM_CONFIG__UVDM_UVDU_PWR_ON = 0x00000002,
    UVD_STATUS__UVD_BUSY = 0x00000004,
    GB_ADDR_CONFIG_DEFAULT = 0x26010011,
    UVD_STATUS__IDLE = 0x2,
    UVD_STATUS__BUSY = 0x5,
    UVD_POWER_STATUS__UVD_POWER_STATUS_TILES_OFF = 0x1,
    UVD_STATUS__RBC_BUSY = 0x1,
    UVD_PGFSM_STATUS_UVDJ_PWR_ON = 0,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum internal_dpg_state {
    VCN_DPG_STATE__UNPAUSE = 0,
    VCN_DPG_STATE__PAUSE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpg_pause_state {
    pub fw_based: internal_dpg_state,
    pub jpeg: internal_dpg_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_vcn_reg {
    pub data0: unsigned,
    pub data1: unsigned,
    pub cmd: unsigned,
    pub nop: unsigned,
    pub context_id: unsigned,
    pub ib_vmid: unsigned,
    pub ib_bar_low: unsigned,
    pub ib_bar_high: unsigned,
    pub ib_size: unsigned,
    pub gp_scratch8: unsigned,
    pub scratch9: unsigned,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_vcn_fw_shared {
    pub cpu_addr: *mut c_void,
    pub gpu_addr: u64,
    pub mem_size: u32,
    pub log_offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_vcn_inst {
    pub adev: *mut amdgpu_device,
    pub inst: c_int,
    pub vcpu_bo: *mut amdgpu_bo,
    pub cpu_addr: *mut c_void,
    pub gpu_addr: u64,
    pub saved_bo: *mut c_void,
    pub ring_dec: amdgpu_ring,
    pub ring_enc: [amdgpu_ring; AMDGPU_VCN_MAX_ENC_RINGS],
    pub sched_score: core::sync::atomic::AtomicI32,
    pub irq: amdgpu_irq_src,
    pub ras_poison_irq: amdgpu_irq_src,
    pub external: amdgpu_vcn_reg,
    pub dpg_sram_bo: *mut amdgpu_bo,
    pub pause_state: dpg_pause_state,
    pub dpg_sram_cpu_addr: *mut c_void,
    pub dpg_sram_gpu_addr: u64,
    pub dpg_sram_curr_addr: *mut u32,
    pub dpg_enc_submission_cnt: core::sync::atomic::AtomicI32,
    pub fw_shared: amdgpu_vcn_fw_shared,
    pub aid_id: u8,
    pub /: *const *const *const firmware fw; / VCN firmware,
    pub vcn_config: u8,
    pub vcn_codec_disable_mask: u32,
    pub total_submission_cnt: core::sync::atomic::AtomicI32,
    pub vcn_pg_lock: mutex,
    pub cur_state: amd_powergating_state,
    pub idle_work: delayed_work,
    pub fw_version: unsigned,
    pub num_enc_rings: unsigned,
    pub indirect_sram: bool,
    pub internal: amdgpu_vcn_reg,
    pub vcn1_jpeg1_workaround: mutex,
    pub new_state): *mut dpg_pause_state,
    pub state): amd_powergating_state,
    pub vinst): *mut *mut int (reset)(struct amdgpu_vcn_inst,
    pub using_unified_queue: bool,
    pub engine_reset_mutex: mutex,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_vcn_ras {
    pub ras_block: amdgpu_ras_block_object,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_vcn {
    pub num_vcn_inst: u8,
    pub inst: [amdgpu_vcn_inst; AMDGPU_MAX_VCN_INSTANCES],
    pub harvest_config: unsigned,
    pub ras_if: *mut ras_common_if,
    pub ras: *mut amdgpu_vcn_ras,
    pub inst_mask: u16,
    pub num_inst_per_aid: u8,
// IP reg dump
    pub ip_dump: *mut u32,
    pub supported_reset: u32,
    pub caps: u32,
    pub per_inst_fw: bool,
    pub fw_version: unsigned,
    pub workload_profile_active: bool,
    pub workload_profile_mutex: mutex,
    pub reg_count: u32,
    pub reg_list: *const amdgpu_hwip_reg_entry,
    pub disable_uq: bool,
    pub disable_kq: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_fw_shared_rb_ptrs_struct {
// to WA DPG R/W ptr issues.
    pub rptr: u32,
    pub wptr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_fw_shared_multi_queue {
    pub decode_queue_mode: u8,
    pub encode_generalpurpose_queue_mode: u8,
    pub encode_lowlatency_queue_mode: u8,
    pub encode_realtime_queue_mode: u8,
    pub padding: [u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_fw_shared_sw_ring {
    pub is_enabled: u8,
    pub padding: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_fw_shared_unified_queue_struct {
    pub is_enabled: u8,
    pub queue_mode: u8,
    pub queue_status: u8,
    pub padding: [u8; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_fw_shared_fw_logging {
    pub is_enabled: u8,
    pub addr_lo: u32,
    pub addr_hi: u32,
    pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_fw_shared_smu_interface_info {
    pub smu_interface_type: u8,
    pub padding: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_fw_shared {
    pub present_flag_0: u32,
    pub pad: [u8; 44],
    pub rb: amdgpu_fw_shared_rb_ptrs_struct,
    pub pad1: [u8; 1],
    pub multi_queue: amdgpu_fw_shared_multi_queue,
    pub sw_ring: amdgpu_fw_shared_sw_ring,
    pub fw_log: amdgpu_fw_shared_fw_logging,
    pub smu_interface_info: amdgpu_fw_shared_smu_interface_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_vcn_rb_setup_info {
    pub rb_addr_lo: u32,
    pub rb_addr_hi: u32,
    pub rb_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_fw_shared_rb_setup {
    pub is_rb_enabled_flags: u32,
    pub rb_addr_lo: u32,
    pub rb_addr_hi: u32,
    pub rb_size: u32,
    pub rb4_addr_lo: u32,
    pub rb4_addr_hi: u32,
    pub rb4_size: u32,
    pub reserved: [u32; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_fw_shared_drm_key_wa {
    pub method: u8,
    pub reserved: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_fw_shared_queue_decouple {
    pub is_enabled: u8,
    pub reserved: [u8; 7],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_vcn4_fw_shared {
    pub present_flag_0: u32,
    pub pad: [u8; 12],
    pub sq: amdgpu_fw_shared_unified_queue_struct,
    pub pad1: [u8; 8],
    pub fw_log: amdgpu_fw_shared_fw_logging,
    pub pad2: [u8; 20],
    pub rb_setup: amdgpu_fw_shared_rb_setup,
    pub smu_dpm_interface: amdgpu_fw_shared_smu_interface_info,
    pub drm_key_wa: amdgpu_fw_shared_drm_key_wa,
    pub pad3: [u8; 9],
    pub decouple: amdgpu_fw_shared_queue_decouple,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_vcn_fwlog {
    pub rptr: u32,
    pub wptr: u32,
    pub buffer_size: u32,
    pub header_size: u32,
    pub wrapped: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_vcn_decode_buffer {
    pub valid_buf_flag: u32,
    pub msg_buffer_address_hi: u32,
    pub msg_buffer_address_lo: u32,
    pub pad: [u32; 30],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_vcn_rb_metadata {
    pub size: u32,
    pub present_flag_0: u32,
    pub version: u8,
    pub ring_id: u8,
    pub pad: [u8; 26],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_vcn5_fw_shared {
    pub present_flag_0: u32,
    pub pad: [u8; 12],
    pub sq: amdgpu_fw_shared_unified_queue_struct,
    pub pad1: [u8; 8],
    pub fw_log: amdgpu_fw_shared_fw_logging,
    pub pad2: [u8; 20],
    pub rb_setup: amdgpu_fw_shared_rb_setup,
    pub smu_dpm_interface: amdgpu_fw_shared_smu_interface_info,
    pub drm_key_wa: amdgpu_fw_shared_drm_key_wa,
    pub pad3: [u8; 404],
}

pub const VCN_BLOCK_ENCODE_DISABLE_MASK: c_uint = 0x80;
pub const VCN_BLOCK_DECODE_DISABLE_MASK: c_uint = 0x40;
pub const VCN_BLOCK_QUEUE_DISABLE_MASK: c_uint = 0xC0;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vcn_ring_type {
    VCN_ENCODE_RING,
    VCN_DECODE_RING,
    VCN_UNIFIED_RING,
}

extern "C" {
    pub fn amdgpu_vcn_early_init(adev: *mut amdgpu_device, i: c_int) -> c_int;
}
extern "C" {
    pub fn amdgpu_vcn_sw_init(adev: *mut amdgpu_device, i: c_int) -> c_int;
}
extern "C" {
    pub fn amdgpu_vcn_sw_fini(adev: *mut amdgpu_device, i: c_int);
}
extern "C" {
    pub fn amdgpu_vcn_suspend(adev: *mut amdgpu_device, i: c_int) -> c_int;
}
extern "C" {
    pub fn amdgpu_vcn_resume(adev: *mut amdgpu_device, i: c_int) -> c_int;
}
extern "C" {
    pub fn amdgpu_vcn_ring_begin_use(ring: *mut amdgpu_ring);
}
extern "C" {
    pub fn amdgpu_vcn_ring_end_use(ring: *mut amdgpu_ring);
}
extern "C" {
    pub fn amdgpu_vcn_dec_ring_test_ring(ring: *mut amdgpu_ring) -> c_int;
}
extern "C" {
    pub fn amdgpu_vcn_dec_ring_test_ib(ring: *mut amdgpu_ring, timeout: c_long) -> c_int;
}
extern "C" {
    pub fn amdgpu_vcn_dec_sw_ring_test_ring(ring: *mut amdgpu_ring) -> c_int;
}
extern "C" {
    pub fn amdgpu_vcn_dec_sw_ring_test_ib(ring: *mut amdgpu_ring, timeout: c_long) -> c_int;
}
extern "C" {
    pub fn amdgpu_vcn_unified_ring_test_ib(ring: *mut amdgpu_ring, timeout: c_long) -> c_int;
}
extern "C" {
    pub fn amdgpu_vcn_enc_ring_test_ring(ring: *mut amdgpu_ring) -> c_int;
}
extern "C" {
    pub fn amdgpu_vcn_enc_ring_test_ib(ring: *mut amdgpu_ring, timeout: c_long) -> c_int;
}
extern "C" {
    pub fn amdgpu_vcn_get_enc_ring_prio(ring: c_int) -> amdgpu_ring_priority_level;
}
extern "C" {
    pub fn amdgpu_vcn_setup_ucode(adev: *mut amdgpu_device, i: c_int);
}
extern "C" {
    pub fn amdgpu_vcn_fwlog_init(vcn: *mut amdgpu_vcn_inst);
}
extern "C" {
    pub fn amdgpu_vcn_ras_sw_init(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_vcn_save_vcpu_bo(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_vcn_sysfs_reset_mask_init(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_vcn_sysfs_reset_mask_fini(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_debugfs_vcn_sched_mask_init(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_vcn_dump_ip_state(ip_block: *mut amdgpu_ip_block);
}
extern "C" {
    pub fn amdgpu_vcn_print_ip_state(ip_block: *mut amdgpu_ip_block, p: *mut drm_printer);
}
extern "C" {
    pub fn amdgpu_vcn_get_profile(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_vcn_put_profile(adev: *mut amdgpu_device);
}
