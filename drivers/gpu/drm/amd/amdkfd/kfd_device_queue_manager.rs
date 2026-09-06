//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdkfd/kfd_device_queue_manager.h
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


// SPDX-License-Identifier: GPL-2.0 OR MIT
//
// Copyright 2014-2022 Advanced Micro Devices, Inc.
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

pub const VMID_NUM: c_int = 16;
pub const KFD_MES_PROCESS_QUANTUM: c_int = 100000;
pub const KFD_MES_GANG_QUANTUM: c_int = 10000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct device_process_node {
    pub qpd: *mut qcm_process_device,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union SQ_CMD_BITS {
    pub cmd:3: u32,
    pub mode:3: u32,
    pub check_vmid:1: u32,
    pub trap_id:3: u32,
    pub wave_id:4: u32,
    pub simd_id:2: u32,
    pub queue_id:3: u32,
    pub vm_id:4: u32,
    pub bits: } bitfields,,
    pub u32All: u32,
    pub i32All: signed int,
    pub f32All: float,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union GRBM_GFX_INDEX_BITS {
    pub instance_index:8: u32,
    pub sh_index:8: u32,
    pub se_index:8: u32,
    pub sh_broadcast_writes:1: u32,
    pub instance_broadcast_writes:1: u32,
    pub se_broadcast_writes:1: u32,
    pub bits: } bitfields,,
    pub u32All: u32,
    pub i32All: signed int,
    pub f32All: float,
}

//
// struct device_queue_manager_ops
//
// @create_queue: Queue creation routine.
//
// @destroy_queue: Queue destruction routine.
//
// @update_queue: Queue update routine.
//
// @exeute_queues: Dispatches the queues list to the H/W.
//
// @register_process: This routine associates a specific process with device.
//
// @unregister_process: destroys the associations between process to device.
//
// @initialize: Initializes the pipelines and memory module for that device.
//
// @start: Initializes the resources/modules the device needs for queues
// execution. This function is called on device initialization and after the
// system woke up after suspension.
//
// @stop: This routine stops execution of all the active queue running on the
// H/W and basically this function called on system suspend.
//
// @uninitialize: Destroys all the device queue manager resources allocated in
// initialize routine.
//
// @halt: This routine unmaps queues from runlist and set halt status to true
// so no more queues will be mapped to runlist until unhalt.
//
// @unhalt: This routine unset halt status to flase and maps queues back to
// runlist.
//
// @create_kernel_queue: Creates kernel queue. Used for debug queue.
//
// @destroy_kernel_queue: Destroys kernel queue. Used for debug queue.
//
// @set_cache_memory_policy: Sets memory policy (cached/ non cached) for the
// memory apertures.
//
// @process_termination: Clears all process queues belongs to that device.
//
// @evict_process_queues: Evict all active queues of a process
//
// @restore_process_queues: Restore all evicted queues of a process
//
// @get_wave_state: Retrieves context save state and optionally copies the
// control stack, if kept in the MQD, to the given userspace address.
//
// @reset_queues: reset queues which consume RAS poison
// @get_queue_checkpoint_info: Retrieves queue size information for CRIU checkpoint.
//
// @checkpoint_mqd: checkpoint queue MQD contents for CRIU.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct device_queue_manager_ops {
    pub restore_ctl_stack): *const c_void,
    pub q): *mut queue,
    pub minfo): *mut *mut queue q, mqd_update_info,
    pub qpd): *mut qcm_process_device,
    pub qpd): *mut qcm_process_device,
    pub dqm): *mut *mut int (initialize)(struct device_queue_manager,
    pub dqm): *mut *mut int (start)(struct device_queue_manager,
    pub dqm): *mut *mut int (stop)(struct device_queue_manager,
    pub dqm): *mut *mut void (uninitialize)(struct device_queue_manager,
    pub dqm): *mut *mut int (halt)(struct device_queue_manager,
    pub dqm): *mut *mut int (unhalt)(struct device_queue_manager,
    pub qpd): *mut qcm_process_device,
    pub qpd): *mut qcm_process_device,
    pub misc_process_properties): u32,
    pub qpd): *mut qcm_process_device,
    pub qpd): *mut qcm_process_device,
    pub qpd): *mut qcm_process_device,
    pub save_area_used_size): *mut u32,
    pub pasid): u16,
    pub ctl_stack_size): *mut u32,
    pub ctl_stack): *mut c_void,
    pub enable): c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct device_queue_manager_asic_ops {
    pub qpd): *mut qcm_process_device,
    pub misc_process_properties): u32,
    pub qpd): *mut qcm_process_device,
    pub dev): *mut kfd_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dqm_detect_hang_info {
    pub pipe_id: c_int,
    pub queue_id: c_int,
    pub xcc_id: c_int,
    pub queue_address: u64,
}

//
// struct device_queue_manager
//
// This struct is a base class for the kfd queues scheduler in the
// device level. The device base class should expose the basic operations
// for queue creation and queue destruction. This base class hides the
// scheduling mode of the driver and the specific implementation of the
// concrete device. This class is the only class in the queues scheduler
// that configures the H/W.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct device_queue_manager {
    pub ops: device_queue_manager_ops,
    pub asic_ops: device_queue_manager_asic_ops,
    pub mqd_mgrs: [*mut mqd_manager; KFD_MQD_TYPE_MAX],
    pub packet_mgr: packet_manager,
    pub dev: *mut kfd_node,
    pub /: *mut *mut mutex lock_hidden; / use dqm_lock/unlock(dqm),
    pub queues: list_head,
    pub saved_flags: c_uint,
    pub processes_count: c_uint,
    pub active_queue_count: c_uint,
    pub active_cp_queue_count: c_uint,
    pub gws_queue_count: c_uint,
    pub total_queue_count: c_uint,
    pub next_pipe_to_allocate: c_uint,
    pub allocated_queues: *mut c_uint,
    pub KFD_MAX_SDMA_QUEUES): DECLARE_BITMAP(sdma_bitmap,,
    pub KFD_MAX_SDMA_QUEUES): DECLARE_BITMAP(xgmi_sdma_bitmap,,
// the pasid mapping for each kfd vmid
    pub vmid_pasid: [u16; VMID_NUM],
    pub pipelines_addr: u64,
    pub fence_gpu_addr: u64,
    pub fence_addr: *mut u64,
    pub fence_mem: *mut kfd_mem_obj,
    pub active_runlist: bool,
    pub sched_policy: c_int,
    pub trap_debug_vmid: u32,
// hw exception
    pub is_hws_hang: bool,
    pub is_resetting: bool,
    pub hiq_sdma_mqd: kfd_mem_obj,
    pub sched_running: bool,
    pub sched_halt: bool,
// used for GFX 9.4.3 only
    pub current_logical_xcc_start: u32,
    pub wait_times: u32,
    pub destroy_wait: wait_queue_head_t,
// for per-queue reset support
    pub detect_hang_info: *mut dqm_detect_hang_info,
    pub detect_hang_info_size: usize,
    pub detect_hang_count: c_int,
// for per-queue reset with mes
    pub hung_db_array: *mut u32,
    pub hqd_info: *mut amdgpu_mes_hung_queue_hqd_info,
}

extern "C" {
    pub fn get_cp_queues_num(dqm: *mut device_queue_manager) -> c_uint;
}
extern "C" {
    pub fn get_queues_per_pipe(dqm: *mut device_queue_manager) -> c_uint;
}
extern "C" {
    pub fn get_pipes_per_mec(dqm: *mut device_queue_manager) -> c_uint;
}
extern "C" {
    pub fn get_num_sdma_queues(dqm: *mut device_queue_manager) -> c_uint;
}
extern "C" {
    pub fn get_num_xgmi_sdma_queues(dqm: *mut device_queue_manager) -> c_uint;
}
extern "C" {
    pub fn debug_lock_and_unmap(dqm: *mut device_queue_manager) -> c_int;
}
extern "C" {
    pub fn debug_map_and_unlock(dqm: *mut device_queue_manager) -> c_int;
}
extern "C" {
    pub fn debug_refresh_runlist(dqm: *mut device_queue_manager) -> c_int;
}
// The DQM lock can be taken in MMU notifiers. Make sure no reclaim-FS
// happens while holding this lock anywhere to prevent deadlocks when
// an MMU notifier runs in reclaim-FS context.
//
// SDMA activity counter is stored at queue's RPTR + 0x8 location.
extern "C" {
    pub fn get_user(_arg: *mut val, 1: q_rptr +) -> return;
}
