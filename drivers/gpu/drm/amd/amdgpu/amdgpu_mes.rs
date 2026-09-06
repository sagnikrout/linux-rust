//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/amdgpu_mes.h
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
// Copyright 2019 Advanced Micro Devices, Inc.
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

pub const AMDGPU_MES_MAX_COMPUTE_PIPES: c_int = 8;
pub const AMDGPU_MES_MAX_GFX_PIPES: c_int = 2;
pub const AMDGPU_MES_MAX_SDMA_PIPES: c_int = 2;
pub const AMDGPU_MES_API_VERSION_SHIFT: c_int = 12;
pub const AMDGPU_MES_FEAT_VERSION_SHIFT: c_int = 24;
pub const AMDGPU_MES_VERSION_MASK: c_uint = 0x00000fff;
pub const AMDGPU_MES_API_VERSION_MASK: c_uint = 0x00fff000;
pub const AMDGPU_MES_FEAT_VERSION_MASK: c_uint = 0xff000000;
pub const AMDGPU_MES_MSCRATCH_SIZE: c_uint = 0x40000;
pub const AMDGPU_MES_INVALID_DB_OFFSET: c_uint = 0xffffffff;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_mes_priority_level {
    AMDGPU_MES_PRIORITY_LEVEL_LOW       = 0,
    AMDGPU_MES_PRIORITY_LEVEL_NORMAL    = 1,
    AMDGPU_MES_PRIORITY_LEVEL_MEDIUM    = 2,
    AMDGPU_MES_PRIORITY_LEVEL_HIGH      = 3,
    AMDGPU_MES_PRIORITY_LEVEL_REALTIME  = 4,
    AMDGPU_MES_PRIORITY_NUM_LEVELS
}

pub const AMDGPU_MES_PROC_CTX_SIZE: c_uint = 0x1000 /* one page area */;
pub const AMDGPU_MES_GANG_CTX_SIZE: c_uint = 0x1000 /* one page area */;
pub const AMDGPU_MES_PROC_CTX_ARRAY_MAX: c_int = 128;
pub const AMDGPU_MES_GANG_CTX_ARRAY_MAX: c_int = 512;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_mes_pipe {
    AMDGPU_MES_PIPE_0 = 0,
    AMDGPU_MES_PIPE_1,
    AMDGPU_MAX_MES_PIPES = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_mes {
    pub adev: *mut amdgpu_device,
    pub mutex_hidden: mutex,
    pub doorbell_ida: ida,
    pub queue_id_lock: spinlock_t,
    pub sched_version: u32,
    pub kiq_version: u32,
    pub fw_version: [u32; AMDGPU_MAX_MES_PIPES],
    pub enable_legacy_queue_map: bool,
    pub total_max_queue: u32,
    pub max_doorbell_slices: u32,
    pub default_process_quantum: u64,
    pub default_gang_quantum: u64,
    pub ring: [amdgpu_ring; AMDGPU_MAX_MES_INST_PIPES],
    pub ring_lock: [spinlock_t; AMDGPU_MAX_MES_INST_PIPES],
    pub fw: [*const firmware; AMDGPU_MAX_MES_PIPES],
// mes ucode
    pub ucode_fw_obj: [*mut amdgpu_bo; AMDGPU_MAX_MES_INST_PIPES],
    pub ucode_fw_gpu_addr: [u64; AMDGPU_MAX_MES_INST_PIPES],
    pub ucode_fw_ptr: [*mut u32; AMDGPU_MAX_MES_INST_PIPES],
    pub uc_start_addr: [u64; AMDGPU_MAX_MES_PIPES],
// mes ucode data
    pub data_fw_obj: [*mut amdgpu_bo; AMDGPU_MAX_MES_INST_PIPES],
    pub data_fw_gpu_addr: [u64; AMDGPU_MAX_MES_INST_PIPES],
    pub data_fw_ptr: [*mut u32; AMDGPU_MAX_MES_INST_PIPES],
    pub data_start_addr: [u64; AMDGPU_MAX_MES_PIPES],
// eop gpu obj
    pub eop_gpu_obj: [*mut amdgpu_bo; AMDGPU_MAX_MES_INST_PIPES],
    pub eop_gpu_addr: [u64; AMDGPU_MAX_MES_INST_PIPES],
    pub mqd_backup: [*mut c_void; AMDGPU_MAX_MES_INST_PIPES],
    pub irq: [amdgpu_irq_src; AMDGPU_MAX_MES_INST_PIPES],
    pub vmid_mask_gfxhub: u32,
    pub vmid_mask_mmhub: u32,
    pub gfx_hqd_mask: [u32; AMDGPU_MES_MAX_GFX_PIPES],
    pub compute_hqd_mask: [u32; AMDGPU_MES_MAX_COMPUTE_PIPES],
    pub sdma_hqd_mask: [u32; AMDGPU_MES_MAX_SDMA_PIPES],
    pub aggregated_doorbells: [u32; AMDGPU_MES_PRIORITY_NUM_LEVELS],
    pub sch_ctx_offs: [u32; AMDGPU_MAX_MES_INST_PIPES],
    pub sch_ctx_gpu_addr: [u64; AMDGPU_MAX_MES_INST_PIPES],
    pub sch_ctx_ptr: [*mut u64; AMDGPU_MAX_MES_INST_PIPES],
    pub query_status_fence_offs: [u32; AMDGPU_MAX_MES_INST_PIPES],
    pub query_status_fence_gpu_addr: [u64; AMDGPU_MAX_MES_INST_PIPES],
    pub query_status_fence_ptr: [*mut u64; AMDGPU_MAX_MES_INST_PIPES],
    pub saved_flags: u32,
// initialize kiq pipe
    pub xcc_id): u32,
    pub xcc_id): u32,
// MES doorbells
    pub db_start_dw_offset: u32,
    pub num_mes_dbs: u32,
    pub doorbell_bitmap: *mut c_ulong,
// MES event log buffer
    pub event_log_size: u32,
    pub event_log_gpu_obj: *mut amdgpu_bo,
    pub event_log_gpu_addr: u64,
    pub event_log_cpu_addr: *mut c_void,
// ip specific functions
    pub funcs: *const amdgpu_mes_funcs,
// mes resource_1 bo
    pub resource_1: [*mut amdgpu_bo; AMDGPU_MAX_MES_PIPES],
    pub resource_1_gpu_addr: [u64; AMDGPU_MAX_MES_PIPES],
    pub resource_1_addr: [*mut c_void; AMDGPU_MAX_MES_PIPES],
    pub hung_queue_db_array_size: c_int,
    pub hung_queue_hqd_info_offset: c_int,
    pub hung_queue_db_array_gpu_obj: [*mut amdgpu_bo; AMDGPU_MAX_MES_INST_PIPES],
    pub hung_queue_db_array_gpu_addr: [u64; AMDGPU_MAX_MES_INST_PIPES],
    pub hung_queue_db_array_cpu_addr: [*mut c_void; AMDGPU_MAX_MES_INST_PIPES],
// cooperative dispatch
    pub enable_coop_mode: bool,
    pub master_xcc_ids: [c_int; AMDGPU_MAX_MES_INST_PIPES],
    pub shared_cmd_buf_obj: [*mut amdgpu_bo; AMDGPU_MAX_MES_INST_PIPES],
    pub shared_cmd_buf_gpu_addr: [u64; AMDGPU_MAX_MES_INST_PIPES],
    pub compute_pipe_reset_enabled: bool,
    pub gfx_pipe_reset_enabled: bool,
    pub use_rs64mem: bool,
    pub ctx_array_size_bo: *mut amdgpu_bo,
    pub ctx_array_size_gpu_addr: u64,
    pub ctx_array_size_cpu_ptr: *mut u32,
    pub proc_ctx_array_size: u32,
    pub proc_ctx_bitmap: *mut c_ulong,
    pub gang_ctx_array_size: u32,
    pub gang_ctx_array_index: u32,
    pub gang_ctx_bitmap: *mut c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_mes_hung_queue_hqd_info {
    pub type: u32 queue_type: 3; // queue,
    pub index: u32 pipe_index: 4; // pipe,
    pub index: u32 queue_index: 8; // queue,
    pub 17: u32 reserved:,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_mes_gang {
    pub gang_id: c_int,
    pub priority: c_int,
    pub inprocess_gang_priority: c_int,
    pub global_priority_level: c_int,
    pub list: list_head,
    pub process: *mut amdgpu_mes_process,
    pub gang_ctx_bo: *mut amdgpu_bo,
    pub gang_ctx_gpu_addr: u64,
    pub gang_ctx_cpu_ptr: *mut c_void,
    pub gang_quantum: u64,
    pub queue_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_mes_queue {
    pub list: list_head,
    pub gang: *mut amdgpu_mes_gang,
    pub queue_id: c_int,
    pub doorbell_off: u64,
    pub mqd_obj: *mut amdgpu_bo,
    pub mqd_cpu_ptr: *mut c_void,
    pub mqd_gpu_addr: u64,
    pub wptr_gpu_addr: u64,
    pub queue_type: c_int,
    pub paging: c_int,
    pub ring: *mut amdgpu_ring,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_mes_queue_properties {
    pub queue_type: c_int,
    pub hqd_base_gpu_addr: u64,
    pub rptr_gpu_addr: u64,
    pub wptr_gpu_addr: u64,
    pub wptr_mc_addr: u64,
    pub queue_size: u32,
    pub eop_gpu_addr: u64,
    pub hqd_pipe_priority: u32,
    pub hqd_queue_priority: u32,
    pub paging: bool,
    pub ring: *mut amdgpu_ring,
// out
    pub doorbell_off: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_mes_gang_properties {
    pub priority: u32,
    pub gang_quantum: u32,
    pub inprocess_gang_priority: u32,
    pub priority_level: u32,
    pub global_priority_level: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mes_add_queue_input {
    pub xcc_id: u32,
    pub process_id: u32,
    pub page_table_base_addr: u64,
    pub process_va_start: u64,
    pub process_va_end: u64,
    pub process_quantum: u64,
    pub process_context_addr: u64,
    pub gang_quantum: u64,
    pub gang_context_addr: u64,
    pub inprocess_gang_priority: u32,
    pub gang_global_priority_level: u32,
    pub doorbell_offset: u32,
    pub mqd_addr: u64,
    pub wptr_addr: u64,
    pub wptr_mc_addr: u64,
    pub queue_type: u32,
    pub paging: u32,
    pub gws_base: u32,
    pub gws_size: u32,
    pub tba_addr: u64,
    pub tma_addr: u64,
    pub trap_en: u32,
    pub skip_process_ctx_clear: u32,
    pub is_kfd_process: u32,
    pub is_aql_queue: u32,
    pub queue_size: u32,
    pub exclusively_scheduled: u32,
    pub sh_mem_config_data: u32,
    pub vm_cntx_cntl: u32,
    pub process_context_array_index: u32,
    pub gang_context_array_index: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mes_remove_queue_input {
    pub xcc_id: u32,
    pub doorbell_offset: u32,
    pub gang_context_addr: u64,
    pub queue_type: u32,
    pub remove_queue_after_reset: bool,
    pub gang_context_array_index: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mes_map_legacy_queue_input {
    pub xcc_id: u32,
    pub queue_type: u32,
    pub doorbell_offset: u32,
    pub pipe_id: u32,
    pub queue_id: u32,
    pub mqd_addr: u64,
    pub wptr_addr: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mes_unmap_legacy_queue_input {
    pub xcc_id: u32,
    pub action: amdgpu_unmap_queues_action,
    pub queue_type: u32,
    pub doorbell_offset: u32,
    pub pipe_id: u32,
    pub queue_id: u32,
    pub trail_fence_addr: u64,
    pub trail_fence_data: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mes_suspend_gang_input {
    pub xcc_id: u32,
    pub suspend_all_gangs: bool,
    pub suspend_all_sdma_gangs: bool,
    pub gang_context_addr: u64,
    pub suspend_fence_addr: u64,
    pub suspend_fence_value: u32,
    pub doorbell_offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mes_resume_gang_input {
    pub xcc_id: u32,
    pub resume_all_gangs: bool,
    pub gang_context_addr: u64,
    pub doorbell_offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mes_reset_queue_input {
    pub xcc_id: u32,
    pub queue_type: u32,
    pub doorbell_offset: u32,
    pub use_mmio: bool,
    pub me_id: u32,
    pub pipe_id: u32,
    pub queue_id: u32,
    pub mqd_addr: u64,
    pub wptr_addr: u64,
    pub vmid: u32,
    pub legacy_gfx: bool,
    pub is_kq: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mes_detect_and_reset_queue_input {
    pub queue_type: u32,
    pub detect_only: bool,
    pub xcc_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mes_inv_tlbs_pasid_input {
    pub xcc_id: u32,
    pub pasid: u16,
    pub hub_id: u8,
    pub flush_type: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mes_misc_opcode {
    MES_MISC_OP_WRITE_REG,
    MES_MISC_OP_READ_REG,
    MES_MISC_OP_WRM_REG_WAIT,
    MES_MISC_OP_WRM_REG_WR_WAIT,
    MES_MISC_OP_SET_SHADER_DEBUGGER,
    MES_MISC_OP_CHANGE_CONFIG,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mes_misc_op_input {
    pub xcc_id: u32,
    pub op: mes_misc_opcode,
    pub reg_offset: u32,
    pub buffer_addr: u64,
    pub read_reg: },
    pub reg_offset: u32,
    pub reg_value: u32,
    pub write_reg: },
    pub ref: u32,
    pub mask: u32,
    pub reg0: u32,
    pub reg1: u32,
    pub wrm_reg: },
    pub process_context_addr: u64,
    pub 1: uint32_t single_memop :,
    pub 1: uint32_t single_alu_op :,
    pub 29: uint32_t reserved:,
    pub 1: uint32_t process_ctx_flush:,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_mes_funcs {
    pub input): *mut mes_add_queue_input,
    pub input): *mut mes_remove_queue_input,
    pub input): *mut mes_map_legacy_queue_input,
    pub input): *mut mes_unmap_legacy_queue_input,
    pub input): *mut mes_suspend_gang_input,
    pub input): *mut mes_resume_gang_input,
    pub input): *mut mes_misc_op_input,
    pub input): *mut mes_reset_queue_input,
    pub input): *mut mes_detect_and_reset_queue_input,
    pub input): *mut mes_inv_tlbs_pasid_input,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_mqd_update_flag {
    AMDGPU_UPDATE_FLAG_DBG_WA_ENABLE = 1,
    AMDGPU_UPDATE_FLAG_DBG_WA_DISABLE = 2,
    AMDGPU_UPDATE_FLAG_IS_GWS = 4, /* quirk for gfx9 IP */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_mqd_prop {
    pub mqd_gpu_addr: u64,
    pub hqd_base_gpu_addr: u64,
    pub rptr_gpu_addr: u64,
    pub wptr_gpu_addr: u64,
    pub queue_size: u32,
    pub use_doorbell: bool,
    pub doorbell_index: u32,
    pub eop_gpu_addr: u64,
    pub hqd_pipe_priority: u32,
    pub hqd_queue_priority: u32,
    pub mqd_stride_size: u32,
    pub allow_tunneling: bool,
    pub hqd_active: bool,
    pub shadow_addr: u64,
    pub gds_bkup_addr: u64,
    pub csa_addr: u64,
    pub fence_address: u64,
    pub tmz_queue: bool,
    pub kernel_queue: bool,
    pub cu_mask: *mut u32,
    pub cu_mask_count: u32,
    pub cu_flags: u32,
    pub is_user_cu_masked: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_mqd {
    pub mqd_size: unsigned,
    pub p): *mut amdgpu_mqd_prop,
}

//
// MES FW uses address(mqd_addr + sizeof(struct mqd) + 3*sizeof(uint32_t))
// as fence address and writes a 32 bit fence value to this address.
// Driver needs to allocate at least 4 DWs extra memory in addition to
// sizeof(struct mqd). Add 8 DWs and align to AMDGPU_GPU_PAGE_SIZE for safety.
//

extern "C" {
    pub fn amdgpu_mes_init_microcode(adev: *mut amdgpu_device, pipe: c_int) -> c_int;
}
extern "C" {
    pub fn amdgpu_mes_validate_fw_version(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_mes_init(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_mes_fini(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_mes_suspend(adev: *mut amdgpu_device, xcc_id: u32) -> c_int;
}
extern "C" {
    pub fn amdgpu_mes_resume(adev: *mut amdgpu_device, xcc_id: u32) -> c_int;
}
extern "C" {
    pub fn amdgpu_mes_get_hung_queue_db_array_size(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_mes_hdp_flush(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_mes_doorbell_process_slice(adev: *mut amdgpu_device) -> c_int;
}
//
// MES lock can be taken in MMU notifiers.
//
// A bit more detail about why to set no-FS reclaim with MES lock:
//
// The purpose of the MMU notifier is to stop GPU access to memory so
// that the Linux VM subsystem can move pages around safely. This is
// done by preempting user mode queues for the affected process. When
// MES is used, MES lock needs to be taken to preempt the queues.
//
// The MMU notifier callback entry point in the driver is
// amdgpu_mn_invalidate_range_start_hsa. The relevant call chain from
// there is:
// amdgpu_amdkfd_evict_userptr -> kgd2kfd_quiesce_mm ->
// kfd_process_evict_queues -> pdd->dev->dqm->ops.evict_process_queues
//
// The last part of the chain is a function pointer where we take the
// MES lock.
//
// The problem with taking locks in the MMU notifier is, that MMU
// notifiers can be called in reclaim-FS context. That's where the
// kernel frees up pages to make room for new page allocations under
// memory pressure. While we are running in reclaim-FS context, we must
// not trigger another memory reclaim operation because that would
// recursively reenter the reclaim code and cause a deadlock. The
// memalloc_nofs_save/restore calls guarantee that.
//
// In addition we also need to avoid lock dependencies on other locks taken
// under the MES lock, for example reservation locks. Here is a possible
// scenario of a deadlock:
// Thread A: takes and holds reservation lock | triggers reclaim-FS |
// MMU notifier | blocks trying to take MES lock
// Thread B: takes and holds MES lock | blocks trying to take reservation lock
//
// In this scenario Thread B gets involved in a deadlock even without
// triggering a reclaim-FS operation itself.
// To fix this and break the lock dependency chain you'd need to either:
// 1. protect reservation locks with memalloc_nofs_save/restore, or
// 2. avoid taking reservation locks under the MES lock.
//
// Reservation locks are taken all over the kernel in different subsystems, we
// have no control over them and their lock dependencies.So the only workable
// solution is to avoid taking other locks under the MES lock.
// As a result, make sure no reclaim-FS happens while holding this lock anywhere
// to prevent deadlocks when an MMU notifier runs in reclaim-FS context.
//
extern "C" {
    pub fn amdgpu_mes_suspend_resume_all_supported(adev: *mut amdgpu_device) -> bool;
}
extern "C" {
    pub fn amdgpu_mes_queue_reset_by_mes_supported(adev: *mut amdgpu_device) -> bool;
}
extern "C" {
    pub fn amdgpu_mes_update_enforce_isolation(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_mes_rs64mem_init(mes: *mut amdgpu_mes) -> c_int;
}
extern "C" {
    pub fn amdgpu_mes_rs64mem_fini(mes: *mut amdgpu_mes);
}
extern "C" {
    pub fn amdgpu_mes_rs64mem_setup_bitmaps(mes: *mut amdgpu_mes) -> c_int;
}
