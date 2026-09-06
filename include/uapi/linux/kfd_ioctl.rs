//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/kfd_ioctl.h
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
// Copyright 2014 Advanced Micro Devices, Inc.
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

// Macro flag: #define KFD_IOCTL_H_INCLUDED

//
// - 1.1 - initial version
// - 1.3 - Add SMI events support
// - 1.4 - Indicate new SRAM EDC bit in device properties
// - 1.5 - Add SVM API
// - 1.6 - Query clear flags in SVM get_attr API
// - 1.7 - Checkpoint Restore (CRIU) API
// - 1.8 - CRIU - Support for SDMA transfers with GTT BOs
// - 1.9 - Add available memory ioctl
// - 1.10 - Add SMI profiler event log
// - 1.11 - Add unified memory for ctx save/restore area
// - 1.12 - Add DMA buf export ioctl
// - 1.13 - Add debugger API
// - 1.14 - Update kfd_event_data
// - 1.15 - Enable managing mappings in compute VMs with GEM_VA ioctl
// - 1.16 - Add contiguous VRAM allocation flag
// - 1.17 - Add SDMA queue creation with target SDMA engine ID
// - 1.18 - Rename pad in set_memory_policy_args to misc_process_flag
// - 1.19 - Add a new ioctl to craete secondary kfd processes
// - 1.20 - Trap handler support for expert scheduling mode available
// - 1.21 - Debugger support to subscribe to LDS out-of-address exceptions
// - 1.22 - Add queue creation with metadata ring base address
// - 1.23 - Add profiler control ioctl to enable/disable profiler on a process
//
pub const KFD_IOCTL_MAJOR_VERSION: c_int = 1;
pub const KFD_IOCTL_MINOR_VERSION: c_int = 23;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_ioctl_get_version_args {
    pub /: *mut *mut __u32 major_version; / from KFD,
    pub /: *mut *mut __u32 minor_version; / from KFD,
}

// For kfd_ioctl_create_queue_args.queue_type.
pub const KFD_IOC_QUEUE_TYPE_COMPUTE: c_uint = 0x0;
pub const KFD_IOC_QUEUE_TYPE_SDMA: c_uint = 0x1;
pub const KFD_IOC_QUEUE_TYPE_COMPUTE_AQL: c_uint = 0x2;
pub const KFD_IOC_QUEUE_TYPE_SDMA_XGMI: c_uint = 0x3;
pub const KFD_IOC_QUEUE_TYPE_SDMA_BY_ENG_ID: c_uint = 0x4;
pub const KFD_MAX_QUEUE_PERCENTAGE: c_int = 100;
pub const KFD_MAX_QUEUE_PRIORITY: c_int = 15;
pub const KFD_MIN_QUEUE_RING_SIZE: c_int = 1024;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_ioctl_create_queue_args {
    pub /: *mut *mut __u64 ring_base_address; / to KFD,
    pub /: *mut *mut __u64 write_pointer_address; / to KFD,
    pub /: *mut *mut __u64 read_pointer_address; / to KFD,
    pub /: *mut *mut __u64 doorbell_offset; / from KFD,
    pub /: *mut *mut __u32 ring_size; / to KFD,
    pub /: *mut *mut __u32 gpu_id; / to KFD,
    pub /: *mut *mut __u32 queue_type; / to KFD,
    pub /: *mut *mut __u32 queue_percentage; / to KFD,
    pub /: *mut *mut __u32 queue_priority; / to KFD,
    pub /: *mut *mut __u32 queue_id; / from KFD,
    pub /: *mut *mut __u64 eop_buffer_address; / to KFD,
    pub /: *mut *mut __u64 eop_buffer_size; / to KFD,
    pub /: *mut *mut __u64 ctx_save_restore_address; / to KFD,
    pub /: *mut *mut __u32 ctx_save_restore_size; / to KFD,
    pub /: *mut *mut __u32 ctl_stack_size; / to KFD,
    pub /: *mut *mut __u32 sdma_engine_id; / to KFD,
    pub /: *mut *mut __u32 metadata_ring_size; / to KFD,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_ioctl_destroy_queue_args {
    pub /: *mut *mut __u32 queue_id; / to KFD,
    pub pad: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_ioctl_update_queue_args {
    pub /: *mut *mut __u64 ring_base_address; / to KFD,
    pub /: *mut *mut __u32 queue_id; / to KFD,
    pub /: *mut *mut __u32 ring_size; / to KFD,
    pub /: *mut *mut __u32 queue_percentage; / to KFD,
    pub /: *mut *mut __u32 queue_priority; / to KFD,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_ioctl_set_cu_mask_args {
    pub /: *mut *mut __u32 queue_id; / to KFD,
    pub /: *mut *mut __u32 num_cu_mask; / to KFD,
    pub /: *mut *mut __u64 cu_mask_ptr; / to KFD,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_ioctl_get_queue_wave_state_args {
    pub /: *mut *mut __u64 ctl_stack_address; / to KFD,
    pub /: *mut *mut __u32 ctl_stack_used_size; / from KFD,
    pub /: *mut *mut __u32 save_area_used_size; / from KFD,
    pub /: *mut *mut __u32 queue_id; / to KFD,
    pub pad: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_ioctl_get_available_memory_args {
    pub /: *mut *mut __u64 available; / from KFD,
    pub /: *mut *mut __u32 gpu_id; / to KFD,
    pub pad: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_dbg_device_info_entry {
    pub exception_status: __u64,
    pub lds_base: __u64,
    pub lds_limit: __u64,
    pub scratch_base: __u64,
    pub scratch_limit: __u64,
    pub gpuvm_base: __u64,
    pub gpuvm_limit: __u64,
    pub gpu_id: __u32,
    pub location_id: __u32,
    pub vendor_id: __u32,
    pub device_id: __u32,
    pub revision_id: __u32,
    pub subsystem_vendor_id: __u32,
    pub subsystem_device_id: __u32,
    pub fw_version: __u32,
    pub gfx_target_version: __u32,
    pub simd_count: __u32,
    pub max_waves_per_simd: __u32,
    pub array_count: __u32,
    pub simd_arrays_per_engine: __u32,
    pub num_xcc: __u32,
    pub capability: __u32,
    pub debug_prop: __u32,
    pub capability2: __u32,
    pub pad: __u32,
}

// For kfd_ioctl_set_memory_policy_args.default_policy and alternate_policy
pub const KFD_IOC_CACHE_POLICY_COHERENT: c_int = 0;
pub const KFD_IOC_CACHE_POLICY_NONCOHERENT: c_int = 1;
// Misc. per process flags

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_ioctl_set_memory_policy_args {
    pub /: *mut *mut __u64 alternate_aperture_base; / to KFD,
    pub /: *mut *mut __u64 alternate_aperture_size; / to KFD,
    pub /: *mut *mut __u32 gpu_id; / to KFD,
    pub /: *mut *mut __u32 default_policy; / to KFD,
    pub /: *mut *mut __u32 alternate_policy; / to KFD,
    pub /: *mut *mut __u32 misc_process_flag; / to KFD,
}

//
// All counters are monotonic. They are used for profiling of compute jobs.
// The profiling is done by userspace.
//
// In case of GPU reset, the counter should not be affected.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_ioctl_get_clock_counters_args {
    pub /: *mut *mut __u64 gpu_clock_counter; / from KFD,
    pub /: *mut *mut __u64 cpu_clock_counter; / from KFD,
    pub /: *mut *mut __u64 system_clock_counter; / from KFD,
    pub /: *mut *mut __u64 system_clock_freq; / from KFD,
    pub /: *mut *mut __u32 gpu_id; / to KFD,
    pub pad: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_process_device_apertures {
    pub /: *mut *mut __u64 lds_base; / from KFD,
    pub /: *mut *mut __u64 lds_limit; / from KFD,
    pub /: *mut *mut __u64 scratch_base; / from KFD,
    pub /: *mut *mut __u64 scratch_limit; / from KFD,
    pub /: *mut *mut __u64 gpuvm_base; / from KFD,
    pub /: *mut *mut __u64 gpuvm_limit; / from KFD,
    pub /: *mut *mut __u32 gpu_id; / from KFD,
    pub pad: __u32,
}

//
// AMDKFD_IOC_GET_PROCESS_APERTURES is deprecated. Use
// AMDKFD_IOC_GET_PROCESS_APERTURES_NEW instead, which supports an
// unlimited number of GPUs.
//
pub const NUM_OF_SUPPORTED_GPUS: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_ioctl_get_process_apertures_args {
    pub /: *mut *mut process_apertures[NUM_OF_SUPPORTED_GPUS];/ from KFD,
// from KFD, should be in the range [1 - NUM_OF_SUPPORTED_GPUS]
    pub num_of_nodes: __u32,
    pub pad: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_ioctl_get_process_apertures_new_args {
// User allocated. Pointer to struct kfd_process_device_apertures
// filled in by Kernel
//
    pub kfd_process_device_apertures_ptr: __u64,
// to KFD - indicates amount of memory present in
// kfd_process_device_apertures_ptr
// from KFD - Number of entries filled by KFD.
//
    pub num_of_nodes: __u32,
    pub pad: __u32,
}

pub const MAX_ALLOWED_NUM_POINTS: c_int = 100;
pub const MAX_ALLOWED_AW_BUFF_SIZE: c_int = 4096;
pub const MAX_ALLOWED_WAC_BUFF_SIZE: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_ioctl_dbg_register_args {
    pub /: *mut *mut __u32 gpu_id; / to KFD,
    pub pad: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_ioctl_dbg_unregister_args {
    pub /: *mut *mut __u32 gpu_id; / to KFD,
    pub pad: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_ioctl_dbg_address_watch_args {
    pub /: *mut *mut __u64 content_ptr; / a pointer to the actual content,
    pub /: *mut *mut __u32 gpu_id; / to KFD,
    pub /: *mut *mut __u32 buf_size_in_bytes; /including gpu_id and buf_size,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_ioctl_dbg_wave_control_args {
    pub /: *mut *mut __u64 content_ptr; / a pointer to the actual content,
    pub /: *mut *mut __u32 gpu_id; / to KFD,
    pub /: *mut *mut __u32 buf_size_in_bytes; /including gpu_id and buf_size,
}

pub const KFD_INVALID_FD: c_uint = 0xffffffff;
// Matching HSA_EVENTTYPE
pub const KFD_IOC_EVENT_SIGNAL: c_int = 0;
pub const KFD_IOC_EVENT_NODECHANGE: c_int = 1;
pub const KFD_IOC_EVENT_DEVICESTATECHANGE: c_int = 2;
pub const KFD_IOC_EVENT_HW_EXCEPTION: c_int = 3;
pub const KFD_IOC_EVENT_SYSTEM_EVENT: c_int = 4;
pub const KFD_IOC_EVENT_DEBUG_EVENT: c_int = 5;
pub const KFD_IOC_EVENT_PROFILE_EVENT: c_int = 6;
pub const KFD_IOC_EVENT_QUEUE_EVENT: c_int = 7;
pub const KFD_IOC_EVENT_MEMORY: c_int = 8;
pub const KFD_IOC_WAIT_RESULT_COMPLETE: c_int = 0;
pub const KFD_IOC_WAIT_RESULT_TIMEOUT: c_int = 1;
pub const KFD_IOC_WAIT_RESULT_FAIL: c_int = 2;
pub const KFD_SIGNAL_EVENT_LIMIT: c_int = 4096;
// For kfd_event_data.hw_exception_data.reset_type.
pub const KFD_HW_EXCEPTION_WHOLE_GPU_RESET: c_int = 0;
pub const KFD_HW_EXCEPTION_PER_ENGINE_RESET: c_int = 1;
// For kfd_event_data.hw_exception_data.reset_cause.
pub const KFD_HW_EXCEPTION_GPU_HANG: c_int = 0;
pub const KFD_HW_EXCEPTION_ECC: c_int = 1;
// For kfd_hsa_memory_exception_data.ErrorType
pub const KFD_MEM_ERR_NO_RAS: c_int = 0;
pub const KFD_MEM_ERR_SRAM_ECC: c_int = 1;
pub const KFD_MEM_ERR_POISON_CONSUMED: c_int = 2;
pub const KFD_MEM_ERR_GPU_HANG: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_ioctl_create_event_args {
    pub /: *mut *mut __u64 event_page_offset; / from KFD,
    pub /: *mut *mut __u32 event_trigger_data; / from KFD - signal events only,
    pub /: *mut *mut __u32 event_type; / to KFD,
    pub /: *mut *mut __u32 auto_reset; / to KFD,
    pub certain: *mut *mut __u32 node_id; / to KFD - only valid for,
    pub /: *mut *mut __u32 event_id; / from KFD,
    pub /: *mut *mut __u32 event_slot_index; / from KFD,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_ioctl_destroy_event_args {
    pub /: *mut *mut __u32 event_id; / to KFD,
    pub pad: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_ioctl_set_event_args {
    pub /: *mut *mut __u32 event_id; / to KFD,
    pub pad: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_ioctl_reset_event_args {
    pub /: *mut *mut __u32 event_id; / to KFD,
    pub pad: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_memory_exception_failure {
    pub /: *mut *mut __u32 NotPresent; / Page not present or supervisor privilege,
    pub /: *mut *mut __u32 ReadOnly; / Write access to a read-only page,
    pub /: *mut *mut __u32 NoExecute; / Execute access to a page marked NX,
    pub /: *mut *mut __u32 imprecise; / Can't determine the exact fault address,
}

// memory exception data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_hsa_memory_exception_data {
    pub failure: kfd_memory_exception_failure,
    pub va: __u64,
    pub gpu_id: __u32,
    pub error,: *mut *mut __u32 ErrorType; / 0 = no RAS,
// 1 = ECC_SRAM,
// 2 = Link_SYNFLOOD (poison),
// 3 = GPU hang (not attributable to a specific cause),
// other values reserved
//
}

// hw exception data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_hsa_hw_exception_data {
    pub reset_type: __u32,
    pub reset_cause: __u32,
    pub memory_lost: __u32,
    pub gpu_id: __u32,
}

// hsa signal event data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_hsa_signal_event_data {
    pub /: *mut *mut __u64 last_event_age; / to and from KFD,
}

// Event data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_event_data {
// From KFD
    pub memory_exception_data: kfd_hsa_memory_exception_data,
    pub hw_exception_data: kfd_hsa_hw_exception_data,
// To and From KFD
    pub signal_event_data: kfd_hsa_signal_event_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_ioctl_wait_events_args {
    pub struct: *mut *mut __u64 events_ptr; / pointed to,
    pub /: *mut *mut __u32 num_events; / to KFD,
    pub /: *mut *mut __u32 wait_for_all; / to KFD,
    pub /: *mut *mut __u32 timeout; / to KFD,
    pub /: *mut *mut __u32 wait_result; / from KFD,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_ioctl_set_scratch_backing_va_args {
    pub /: *mut *mut __u64 va_addr; / to KFD,
    pub /: *mut *mut __u32 gpu_id; / to KFD,
    pub pad: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_ioctl_get_tile_config_args {
// to KFD: pointer to tile array
    pub tile_config_ptr: __u64,
// to KFD: pointer to macro tile array
    pub macro_tile_config_ptr: __u64,
// to KFD: array size allocated by user mode
// from KFD: array size filled by kernel
//
    pub num_tile_configs: __u32,
// to KFD: array size allocated by user mode
// from KFD: array size filled by kernel
//
    pub num_macro_tile_configs: __u32,
    pub /: *mut *mut __u32 gpu_id; / to KFD,
    pub /: *mut *mut __u32 gb_addr_config; / from KFD,
    pub /: *mut *mut __u32 num_banks; / from KFD,
    pub /: *mut *mut __u32 num_ranks; / from KFD,
// struct size can be extended later if needed
// without breaking ABI compatibility
//
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_ioctl_set_trap_handler_args {
    pub /: *mut *mut __u64 tba_addr; / to KFD,
    pub /: *mut *mut __u64 tma_addr; / to KFD,
    pub /: *mut *mut __u32 gpu_id; / to KFD,
    pub pad: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_ioctl_acquire_vm_args {
    pub /: *mut *mut __u32 drm_fd; / to KFD,
    pub /: *mut *mut __u32 gpu_id; / to KFD,
}

// Allocation flags: memory types

// Allocation flags: attributes/access options

// Allocate memory for later SVM (shared virtual memory) mapping.
//
// @va_addr:     virtual address of the memory to be allocated
// all later mappings on all GPUs will use this address
// @size:        size in bytes
// @handle:      buffer handle returned to user mode, used to refer to
// this allocation for mapping, unmapping and freeing
// @mmap_offset: for CPU-mapping the allocation by mmapping a render node
// for userptrs this is overloaded to specify the CPU address
// @gpu_id:      device identifier
// @flags:       memory type and attributes. See KFD_IOC_ALLOC_MEM_FLAGS above
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_ioctl_alloc_memory_of_gpu_args {
    pub /: *mut *mut __u64 va_addr; / to KFD,
    pub /: *mut *mut __u64 size; / to KFD,
    pub /: *mut *mut __u64 handle; / from KFD,
    pub /: *mut *mut __u64 mmap_offset; / to KFD (userptr), from KFD (mmap offset),
    pub /: *mut *mut __u32 gpu_id; / to KFD,
    pub flags: __u32,
}

// Free memory allocated with kfd_ioctl_alloc_memory_of_gpu
//
// @handle: memory handle returned by alloc
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_ioctl_free_memory_of_gpu_args {
    pub /: *mut *mut __u64 handle; / to KFD,
}

// Map memory to one or more GPUs
//
// @handle:                memory handle returned by alloc
// @device_ids_array_ptr:  array of gpu_ids (__u32 per device)
// @n_devices:             number of devices in the array
// @n_success:             number of devices mapped successfully
//
// @n_success returns information to the caller how many devices from
// the start of the array have mapped the buffer successfully. It can
// be passed into a subsequent retry call to skip those devices. For
// the first call the caller should initialize it to 0.
//
// If the ioctl completes with return code 0 (success), n_success ==
// n_devices.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_ioctl_map_memory_to_gpu_args {
    pub /: *mut *mut __u64 handle; / to KFD,
    pub /: *mut *mut __u64 device_ids_array_ptr; / to KFD,
    pub /: *mut *mut __u32 n_devices; / to KFD,
    pub /: *mut *mut __u32 n_success; / to/from KFD,
}

// Unmap memory from one or more GPUs
//
// same arguments as for mapping
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_ioctl_unmap_memory_from_gpu_args {
    pub /: *mut *mut __u64 handle; / to KFD,
    pub /: *mut *mut __u64 device_ids_array_ptr; / to KFD,
    pub /: *mut *mut __u32 n_devices; / to KFD,
    pub /: *mut *mut __u32 n_success; / to/from KFD,
}

// Allocate GWS for specific queue
//
// @queue_id:    queue's id that GWS is allocated for
// @num_gws:     how many GWS to allocate
// @first_gws:   index of the first GWS allocated.
// only support contiguous GWS allocation
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_ioctl_alloc_queue_gws_args {
    pub /: *mut *mut __u32 queue_id; / to KFD,
    pub /: *mut *mut __u32 num_gws; / to KFD,
    pub /: *mut *mut __u32 first_gws; / from KFD,
    pub pad: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_ioctl_get_dmabuf_info_args {
    pub /: *mut *mut __u64 size; / from KFD,
    pub /: *mut *mut __u64 metadata_ptr; / to KFD,
    pub user): *mut *mut __u32 metadata_size; / to KFD (space allocated by,
// from KFD (actual metadata size)
//
    pub /: *mut *mut __u32 gpu_id; / from KFD,
    pub /: *mut *mut __u32 flags; / from KFD (KFD_IOC_ALLOC_MEM_FLAGS),
    pub /: *mut *mut __u32 dmabuf_fd; / to KFD,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_ioctl_import_dmabuf_args {
    pub /: *mut *mut __u64 va_addr; / to KFD,
    pub /: *mut *mut __u64 handle; / from KFD,
    pub /: *mut *mut __u32 gpu_id; / to KFD,
    pub /: *mut *mut __u32 dmabuf_fd; / to KFD,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_ioctl_export_dmabuf_args {
    pub /: *mut *mut __u64 handle; / to KFD,
    pub /: *mut *mut __u32 flags; / to KFD,
    pub /: *mut *mut __u32 dmabuf_fd; / from KFD,
}

//
// KFD SMI(System Management Interface) events
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kfd_smi_event {
    KFD_SMI_EVENT_NONE = 0, /* not used */
    KFD_SMI_EVENT_VMFAULT = 1, /* event start counting at 1 */
    KFD_SMI_EVENT_THERMAL_THROTTLE = 2,
    KFD_SMI_EVENT_GPU_PRE_RESET = 3,
    KFD_SMI_EVENT_GPU_POST_RESET = 4,
    KFD_SMI_EVENT_MIGRATE_START = 5,
    KFD_SMI_EVENT_MIGRATE_END = 6,
    KFD_SMI_EVENT_PAGE_FAULT_START = 7,
    KFD_SMI_EVENT_PAGE_FAULT_END = 8,
    KFD_SMI_EVENT_QUEUE_EVICTION = 9,
    KFD_SMI_EVENT_QUEUE_RESTORE = 10,
    KFD_SMI_EVENT_UNMAP_FROM_GPU = 11,
    KFD_SMI_EVENT_PROCESS_START = 12,
    KFD_SMI_EVENT_PROCESS_END = 13,

//
// max event number, as a flag bit to get events from all processes,
// this requires super user permission, otherwise will not be able to
// receive event from any process. Without this flag to receive events
// from same process.
//
    KFD_SMI_EVENT_ALL_PROCESS = 64
}

// The reason of the page migration event
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum KFD_MIGRATE_TRIGGERS {
    KFD_MIGRATE_TRIGGER_PREFETCH,		/* Prefetch to GPU VRAM or system memory */
    KFD_MIGRATE_TRIGGER_PAGEFAULT_GPU,	/* GPU page fault recover */
    KFD_MIGRATE_TRIGGER_PAGEFAULT_CPU,	/* CPU page fault recover */
    KFD_MIGRATE_TRIGGER_TTM_EVICTION	/* TTM eviction */
}

// The reason of user queue evition event
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum KFD_QUEUE_EVICTION_TRIGGERS {
    KFD_QUEUE_EVICTION_TRIGGER_SVM,		/* SVM buffer migration */
    KFD_QUEUE_EVICTION_TRIGGER_USERPTR,	/* userptr movement */
    KFD_QUEUE_EVICTION_TRIGGER_TTM,		/* TTM move buffer */
    KFD_QUEUE_EVICTION_TRIGGER_SUSPEND,	/* GPU suspend */
    KFD_QUEUE_EVICTION_CRIU_CHECKPOINT,	/* CRIU checkpoint */
    KFD_QUEUE_EVICTION_CRIU_RESTORE		/* CRIU restore */
}

// The reason of unmap buffer from GPU event
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum KFD_SVM_UNMAP_TRIGGERS {
    KFD_SVM_UNMAP_TRIGGER_MMU_NOTIFY,	/* MMU notifier CPU buffer movement */
    KFD_SVM_UNMAP_TRIGGER_MMU_NOTIFY_MIGRATE,/* MMU notifier page migration */
    KFD_SVM_UNMAP_TRIGGER_UNMAP_FROM_CPU	/* Unmap to free the buffer */
}

pub const KFD_SMI_EVENT_MSG_SIZE: c_int = 96;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_ioctl_smi_events_args {
    pub /: *mut *mut __u32 gpuid; / to KFD,
    pub /: *mut *mut __u32 anon_fd; / from KFD,
}

//
// SVM event tracing via SMI system management interface
//
// Open event file descriptor
// use ioctl AMDKFD_IOC_SMI_EVENTS, pass in gpuid and return a anonymous file
// descriptor to receive SMI events.
// If calling with sudo permission, then file descriptor can be used to receive
// SVM events from all processes, otherwise, to only receive SVM events of same
// process.
//
// To enable the SVM event
// Write event file descriptor with KFD_SMI_EVENT_MASK_FROM_INDEX(event) bitmap
// mask to start record the event to the kfifo, use bitmap mask combination
// for multiple events. New event mask will overwrite the previous event mask.
// KFD_SMI_EVENT_MASK_FROM_INDEX(KFD_SMI_EVENT_ALL_PROCESS) bit requires sudo
// permisson to receive SVM events from all process.
//
// To receive the event
// Application can poll file descriptor to wait for the events, then read event
// from the file into a buffer. Each event is one line string message, starting
// with the event id, then the event specific information.
//
// To decode event information
// The following event format string macro can be used with sscanf to decode
// the specific event information.
// event triggers: the reason to generate the event, defined as enum for unmap,
// eviction and migrate events.
// node, from, to, prefetch_loc, preferred_loc: GPU ID, or 0 for system memory.
// addr: user mode address, in pages
// size: in pages
// pid: the process ID to generate the event
// ns: timestamp in nanosecond-resolution, starts at system boot time but
// stops during suspend
// migrate_update: GPU page fault is recovered by 'M' for migrate, 'U' for update
// rw: 'W' for write page fault, 'R' for read page fault
// rescheduled: 'R' if the queue restore failed and rescheduled to try again
// error_code: migrate failure error code, 0 if no error
//

//
// CRIU IOCTLs (Checkpoint Restore In Userspace)
//
// When checkpointing a process, the userspace application will perform:
// 1. PROCESS_INFO op to determine current process information. This pauses execution and evicts
// all the queues.
// 2. CHECKPOINT op to checkpoint process contents (BOs, queues, events, svm-ranges)
// 3. UNPAUSE op to un-evict all the queues
//
// When restoring a process, the CRIU userspace application will perform:
//
// 1. RESTORE op to restore process contents
// 2. RESUME op to start the process
//
// Note: Queues are forced into an evicted state after a successful PROCESS_INFO. User
// application needs to perform an UNPAUSE operation after calling PROCESS_INFO.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kfd_criu_op {
    KFD_CRIU_OP_PROCESS_INFO,
    KFD_CRIU_OP_CHECKPOINT,
    KFD_CRIU_OP_UNPAUSE,
    KFD_CRIU_OP_RESTORE,
    KFD_CRIU_OP_RESUME,
}

//
// struct kfd_ioctl_criu_args - Arguments perform CRIU operation
// @devices:		[in/out] User pointer to memory location for devices information.
// This is an array of type kfd_criu_device_bucket.
// @bos:		[in/out] User pointer to memory location for BOs information
// This is an array of type kfd_criu_bo_bucket.
// @priv_data:		[in/out] User pointer to memory location for private data
// @priv_data_size:	[in/out] Size of priv_data in bytes
// @num_devices:	[in/out] Number of GPUs used by process. Size of @devices array.
// @num_bos:		[in/out] Number of BOs used by process. Size of @bos array.
// @num_objects:	[in/out] Number of objects used by process. Objects are opaque to
// user application.
// @pid:		[in/out] PID of the process being checkpointed
// @op:			[in] Type of operation (kfd_criu_op)
//
// Return: 0 on success, -errno on failure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_ioctl_criu_args {
    pub /: *mut *mut __u64 devices; / Used during ops: CHECKPOINT, RESTORE,
    pub /: *mut *mut __u64 bos; / Used during ops: CHECKPOINT, RESTORE,
    pub /: *mut *mut __u64 priv_data; / Used during ops: CHECKPOINT, RESTORE,
    pub /: *mut *mut __u64 priv_data_size; / Used during ops: PROCESS_INFO, RESTORE,
    pub /: *mut *mut __u32 num_devices; / Used during ops: PROCESS_INFO, RESTORE,
    pub /: *mut *mut __u32 num_bos; / Used during ops: PROCESS_INFO, RESTORE,
    pub /: *mut *mut __u32 num_objects; / Used during ops: PROCESS_INFO, RESTORE,
    pub /: *mut *mut __u32 pid; / Used during ops: PROCESS_INFO, RESUME,
    pub op: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_criu_device_bucket {
    pub user_gpu_id: __u32,
    pub actual_gpu_id: __u32,
    pub drm_fd: __u32,
    pub pad: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_criu_bo_bucket {
    pub addr: __u64,
    pub size: __u64,
    pub offset: __u64,
    pub /: *mut *mut __u64 restored_offset; / During restore, updated offset for BO,
    pub /: *mut *mut __u32 gpu_id; / This is the user_gpu_id,
    pub alloc_flags: __u32,
    pub dmabuf_fd: __u32,
    pub pad: __u32,
}

// CRIU IOCTLs - END
//
// Register offset inside the remapped mmio page
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kfd_mmio_remap {
    KFD_MMIO_REMAP_HDP_MEM_FLUSH_CNTL = 0,
    KFD_MMIO_REMAP_HDP_REG_FLUSH_CNTL = 4,
}

// Guarantee host access to memory
pub const KFD_IOCTL_SVM_FLAG_HOST_ACCESS: c_uint = 0x00000001;
// Fine grained coherency between all devices with access
pub const KFD_IOCTL_SVM_FLAG_COHERENT: c_uint = 0x00000002;
// Use any GPU in same hive as preferred device
pub const KFD_IOCTL_SVM_FLAG_HIVE_LOCAL: c_uint = 0x00000004;
// GPUs only read, allows replication
pub const KFD_IOCTL_SVM_FLAG_GPU_RO: c_uint = 0x00000008;
// Allow execution on GPU
pub const KFD_IOCTL_SVM_FLAG_GPU_EXEC: c_uint = 0x00000010;
// GPUs mostly read, may allow similar optimizations as RO, but writes fault
pub const KFD_IOCTL_SVM_FLAG_GPU_READ_MOSTLY: c_uint = 0x00000020;
// Keep GPU memory mapping always valid as if XNACK is disable
pub const KFD_IOCTL_SVM_FLAG_GPU_ALWAYS_MAPPED: c_uint = 0x00000040;
// Fine grained coherency between all devices using device-scope atomics
pub const KFD_IOCTL_SVM_FLAG_EXT_COHERENT: c_uint = 0x00000080;
//
// enum kfd_ioctl_svm_op - SVM ioctl operations
//
// @KFD_IOCTL_SVM_OP_SET_ATTR: Modify one or more attributes
// @KFD_IOCTL_SVM_OP_GET_ATTR: Query one or more attributes
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kfd_ioctl_svm_op {
    KFD_IOCTL_SVM_OP_SET_ATTR,
    KFD_IOCTL_SVM_OP_GET_ATTR
}

// kfd_ioctl_svm_location - Enum for preferred and prefetch locations
//
// GPU IDs are used to specify GPUs as preferred and prefetch locations.
// Below definitions are used for system memory or for leaving the preferred
// location unspecified.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kfd_ioctl_svm_location {
    KFD_IOCTL_SVM_LOCATION_SYSMEM = 0,
    KFD_IOCTL_SVM_LOCATION_UNDEFINED = 0xffffffff
}

//
// enum kfd_ioctl_svm_attr_type - SVM attribute types
//
// @KFD_IOCTL_SVM_ATTR_PREFERRED_LOC: gpuid of the preferred location, 0 for
// system memory
// @KFD_IOCTL_SVM_ATTR_PREFETCH_LOC: gpuid of the prefetch location, 0 for
// system memory. Setting this triggers an
// immediate prefetch (migration).
// @KFD_IOCTL_SVM_ATTR_ACCESS:
// @KFD_IOCTL_SVM_ATTR_ACCESS_IN_PLACE:
// @KFD_IOCTL_SVM_ATTR_NO_ACCESS: specify memory access for the gpuid given
// by the attribute value
// @KFD_IOCTL_SVM_ATTR_SET_FLAGS: bitmask of flags to set (see
// KFD_IOCTL_SVM_FLAG_...)
// @KFD_IOCTL_SVM_ATTR_CLR_FLAGS: bitmask of flags to clear
// @KFD_IOCTL_SVM_ATTR_GRANULARITY: migration granularity
// (log2 num pages)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kfd_ioctl_svm_attr_type {
    KFD_IOCTL_SVM_ATTR_PREFERRED_LOC,
    KFD_IOCTL_SVM_ATTR_PREFETCH_LOC,
    KFD_IOCTL_SVM_ATTR_ACCESS,
    KFD_IOCTL_SVM_ATTR_ACCESS_IN_PLACE,
    KFD_IOCTL_SVM_ATTR_NO_ACCESS,
    KFD_IOCTL_SVM_ATTR_SET_FLAGS,
    KFD_IOCTL_SVM_ATTR_CLR_FLAGS,
    KFD_IOCTL_SVM_ATTR_GRANULARITY
}

//
// struct kfd_ioctl_svm_attribute - Attributes as pairs of type and value
//
// The meaning of the @value depends on the attribute type.
//
// @type: attribute type (see enum @kfd_ioctl_svm_attr_type)
// @value: attribute value
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_ioctl_svm_attribute {
    pub type: __u32,
    pub value: __u32,
}

//
// struct kfd_ioctl_svm_args - Arguments for SVM ioctl
//
// @op specifies the operation to perform (see enum
// @kfd_ioctl_svm_op).  @start_addr and @size are common for all
// operations.
//
// A variable number of attributes can be given in @attrs.
// @nattr specifies the number of attributes. New attributes can be
// added in the future without breaking the ABI. If unknown attributes
// are given, the function returns -EINVAL.
//
// @KFD_IOCTL_SVM_OP_SET_ATTR sets attributes for a virtual address
// range. It may overlap existing virtual address ranges. If it does,
// the existing ranges will be split such that the attribute changes
// only apply to the specified address range.
//
// @KFD_IOCTL_SVM_OP_GET_ATTR returns the intersection of attributes
// over all memory in the given range and returns the result as the
// attribute value. If different pages have different preferred or
// prefetch locations, 0xffffffff will be returned for
// @KFD_IOCTL_SVM_ATTR_PREFERRED_LOC or
// @KFD_IOCTL_SVM_ATTR_PREFETCH_LOC resepctively. For
// @KFD_IOCTL_SVM_ATTR_SET_FLAGS, flags of all pages will be
// aggregated by bitwise AND. That means, a flag will be set in the
// output, if that flag is set for all pages in the range. For
// @KFD_IOCTL_SVM_ATTR_CLR_FLAGS, flags of all pages will be
// aggregated by bitwise NOR. That means, a flag will be set in the
// output, if that flag is clear for all pages in the range.
// The minimum migration granularity throughout the range will be
// returned for @KFD_IOCTL_SVM_ATTR_GRANULARITY.
//
// Querying of accessibility attributes works by initializing the
// attribute type to @KFD_IOCTL_SVM_ATTR_ACCESS and the value to the
// GPUID being queried. Multiple attributes can be given to allow
// querying multiple GPUIDs. The ioctl function overwrites the
// attribute type to indicate the access for the specified GPU.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_ioctl_svm_args {
    pub start_addr: __u64,
    pub size: __u64,
    pub op: __u32,
    pub nattr: __u32,
// Variable length array of attributes
    pub attrs: [kfd_ioctl_svm_attribute; ],
}

//
// struct kfd_ioctl_set_xnack_mode_args - Arguments for set_xnack_mode
//
// @xnack_enabled:       [in/out] Whether to enable XNACK mode for this process
//
// @xnack_enabled indicates whether recoverable page faults should be
// enabled for the current process. 0 means disabled, positive means
// enabled, negative means leave unchanged. If enabled, virtual address
// translations on GFXv9 and later AMD GPUs can return XNACK and retry
// the access until a valid PTE is available. This is used to implement
// device page faults.
//
// On output, @xnack_enabled returns the (new) current mode (0 or
// positive). Therefore, a negative input value can be used to query
// the current mode without changing it.
//
// The XNACK mode fundamentally changes the way SVM managed memory works
// in the driver, with subtle effects on application performance and
// functionality.
//
// Enabling XNACK mode requires shader programs to be compiled
// differently. Furthermore, not all GPUs support changing the mode
// per-process. Therefore changing the mode is only allowed while no
// user mode queues exist in the process. This ensure that no shader
// code is running that may be compiled for the wrong mode. And GPUs
// that cannot change to the requested mode will prevent the XNACK
// mode from occurring. All GPUs used by the process must be in the
// same XNACK mode.
//
// GFXv8 or older GPUs do not support 48 bit virtual addresses or SVM.
// Therefore those GPUs are not considered for the XNACK mode switch.
//
// Return: 0 on success, -errno on failure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_ioctl_set_xnack_mode_args {
    pub xnack_enabled: __s32,
}

// Wave launch override modes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kfd_dbg_trap_override_mode {
    KFD_DBG_TRAP_OVERRIDE_OR = 0,
    KFD_DBG_TRAP_OVERRIDE_REPLACE = 1
}

// Wave launch overrides
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kfd_dbg_trap_mask {
    KFD_DBG_TRAP_MASK_FP_INVALID = 1,
    KFD_DBG_TRAP_MASK_FP_INPUT_DENORMAL = 2,
    KFD_DBG_TRAP_MASK_FP_DIVIDE_BY_ZERO = 4,
    KFD_DBG_TRAP_MASK_FP_OVERFLOW = 8,
    KFD_DBG_TRAP_MASK_FP_UNDERFLOW = 16,
    KFD_DBG_TRAP_MASK_FP_INEXACT = 32,
    KFD_DBG_TRAP_MASK_INT_DIVIDE_BY_ZERO = 64,
    KFD_DBG_TRAP_MASK_DBG_ADDRESS_WATCH = 128,
    KFD_DBG_TRAP_MASK_DBG_MEMORY_VIOLATION = 256,
    KFD_DBG_TRAP_MASK_TRAP_ON_WAVE_START = (1 << 30),
    KFD_DBG_TRAP_MASK_TRAP_ON_WAVE_END = (1 << 31)
}

// Wave launch modes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kfd_dbg_trap_wave_launch_mode {
    KFD_DBG_TRAP_WAVE_LAUNCH_MODE_NORMAL = 0,
    KFD_DBG_TRAP_WAVE_LAUNCH_MODE_HALT = 1,
    KFD_DBG_TRAP_WAVE_LAUNCH_MODE_DEBUG = 3
}

// Address watch modes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kfd_dbg_trap_address_watch_mode {
    KFD_DBG_TRAP_ADDRESS_WATCH_MODE_READ = 0,
    KFD_DBG_TRAP_ADDRESS_WATCH_MODE_NONREAD = 1,
    KFD_DBG_TRAP_ADDRESS_WATCH_MODE_ATOMIC = 2,
    KFD_DBG_TRAP_ADDRESS_WATCH_MODE_ALL = 3
}

// Additional wave settings
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kfd_dbg_trap_flags {
    KFD_DBG_TRAP_FLAG_SINGLE_MEM_OP = 1,
    KFD_DBG_TRAP_FLAG_SINGLE_ALU_OP = 2,
    KFD_DBG_TRAP_FLAG_LDS_OUT_OF_ADDR_RANGE = 4
}

// Trap exceptions
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kfd_dbg_trap_exception_code {
    EC_NONE = 0,
// per queue
    EC_QUEUE_WAVE_ABORT = 1,
    EC_QUEUE_WAVE_TRAP = 2,
    EC_QUEUE_WAVE_MATH_ERROR = 3,
    EC_QUEUE_WAVE_ILLEGAL_INSTRUCTION = 4,
    EC_QUEUE_WAVE_MEMORY_VIOLATION = 5,
    EC_QUEUE_WAVE_APERTURE_VIOLATION = 6,
    EC_QUEUE_PACKET_DISPATCH_DIM_INVALID = 16,
    EC_QUEUE_PACKET_DISPATCH_GROUP_SEGMENT_SIZE_INVALID = 17,
    EC_QUEUE_PACKET_DISPATCH_CODE_INVALID = 18,
    EC_QUEUE_PACKET_RESERVED = 19,
    EC_QUEUE_PACKET_UNSUPPORTED = 20,
    EC_QUEUE_PACKET_DISPATCH_WORK_GROUP_SIZE_INVALID = 21,
    EC_QUEUE_PACKET_DISPATCH_REGISTER_INVALID = 22,
    EC_QUEUE_PACKET_VENDOR_UNSUPPORTED = 23,
    EC_QUEUE_PREEMPTION_ERROR = 30,
    EC_QUEUE_NEW = 31,
// per device
    EC_DEVICE_QUEUE_DELETE = 32,
    EC_DEVICE_MEMORY_VIOLATION = 33,
    EC_DEVICE_RAS_ERROR = 34,
    EC_DEVICE_FATAL_HALT = 35,
    EC_DEVICE_NEW = 36,
// per process
    EC_PROCESS_RUNTIME = 48,
    EC_PROCESS_DEVICE_REMOVE = 49,
    EC_MAX
}

// Mask generated by ecode in kfd_dbg_trap_exception_code

// Masks for exception code type checks below

// Checks for exception code types for KFD search

// Runtime enable states
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kfd_dbg_runtime_state {
    DEBUG_RUNTIME_STATE_DISABLED = 0,
    DEBUG_RUNTIME_STATE_ENABLED = 1,
    DEBUG_RUNTIME_STATE_ENABLED_BUSY = 2,
    DEBUG_RUNTIME_STATE_ENABLED_ERROR = 3
}

// Runtime enable status
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_runtime_info {
    pub r_debug: __u64,
    pub runtime_state: __u32,
    pub ttmp_setup: __u32,
}

// Enable modes for runtime enable
pub const KFD_RUNTIME_ENABLE_MODE_ENABLE_MASK: c_int = 1;
pub const KFD_RUNTIME_ENABLE_MODE_TTMP_SAVE_MASK: c_int = 2;
//
// struct kfd_ioctl_runtime_enable_args - Arguments for runtime enable
//
// Coordinates debug exception signalling and debug device enablement with runtime.
//
// @r_debug: pointer to user struct for sharing information between ROCr and the debuggger
// @mode_mask: mask to set mode
// KFD_RUNTIME_ENABLE_MODE_ENABLE_MASK - enable runtime for debugging, otherwise disable
// KFD_RUNTIME_ENABLE_MODE_TTMP_SAVE_MASK - enable trap temporary setup (ignore on disable)
// @capabilities_mask: mask to notify runtime on what KFD supports
//
// Return - 0 on SUCCESS.
// - EBUSY if runtime enable call already pending.
// - EEXIST if user queues already active prior to call.
// If process is debug enabled, runtime enable will enable debug devices and
// wait for debugger process to send runtime exception EC_PROCESS_RUNTIME
// to unblock - see kfd_ioctl_dbg_trap_args.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_ioctl_runtime_enable_args {
    pub r_debug: __u64,
    pub mode_mask: __u32,
    pub capabilities_mask: __u32,
}

// Queue information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_queue_snapshot_entry {
    pub exception_status: __u64,
    pub ring_base_address: __u64,
    pub write_pointer_address: __u64,
    pub read_pointer_address: __u64,
    pub ctx_save_restore_address: __u64,
    pub queue_id: __u32,
    pub gpu_id: __u32,
    pub ring_size: __u32,
    pub queue_type: __u32,
    pub ctx_save_restore_area_size: __u32,
    pub reserved: __u32,
}

// Queue status return for suspend/resume
pub const KFD_DBG_QUEUE_ERROR_BIT: c_int = 30;
pub const KFD_DBG_QUEUE_INVALID_BIT: c_int = 31;

// Context save area header information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_context_save_area_header {
    pub control_stack_offset: __u32,
    pub control_stack_size: __u32,
    pub wave_state_offset: __u32,
    pub wave_state_size: __u32,
    pub wave_state: },
    pub debug_offset: __u32,
    pub debug_size: __u32,
    pub err_payload_addr: __u64,
    pub err_event_id: __u32,
    pub reserved1: __u32,
}

//
// Debug operations
//
// For specifics on usage and return values, see documentation per operation
// below.  Otherwise, generic error returns apply:
// - ESRCH if the process to debug does not exist.
//
// - EINVAL (with KFD_IOC_DBG_TRAP_ENABLE exempt) if operation
// KFD_IOC_DBG_TRAP_ENABLE has not succeeded prior.
// Also returns this error if GPU hardware scheduling is not supported.
//
// - EPERM (with KFD_IOC_DBG_TRAP_DISABLE exempt) if target process is not
// PTRACE_ATTACHED.  KFD_IOC_DBG_TRAP_DISABLE is exempt to allow
// clean up of debug mode as long as process is debug enabled.
//
// - EACCES if any DBG_HW_OP (debug hardware operation) is requested when
// AMDKFD_IOC_RUNTIME_ENABLE has not succeeded prior.
//
// - ENODEV if any GPU does not support debugging on a DBG_HW_OP call.
//
// - Other errors may be returned when a DBG_HW_OP occurs while the GPU
// is in a fatal state.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kfd_dbg_trap_operations {
    KFD_IOC_DBG_TRAP_ENABLE = 0,
    KFD_IOC_DBG_TRAP_DISABLE = 1,
    KFD_IOC_DBG_TRAP_SEND_RUNTIME_EVENT = 2,
    KFD_IOC_DBG_TRAP_SET_EXCEPTIONS_ENABLED = 3,
    KFD_IOC_DBG_TRAP_SET_WAVE_LAUNCH_OVERRIDE = 4,  /* DBG_HW_OP */
    KFD_IOC_DBG_TRAP_SET_WAVE_LAUNCH_MODE = 5,      /* DBG_HW_OP */
    KFD_IOC_DBG_TRAP_SUSPEND_QUEUES = 6,		/* DBG_HW_OP */
    KFD_IOC_DBG_TRAP_RESUME_QUEUES = 7,		/* DBG_HW_OP */
    KFD_IOC_DBG_TRAP_SET_NODE_ADDRESS_WATCH = 8,	/* DBG_HW_OP */
    KFD_IOC_DBG_TRAP_CLEAR_NODE_ADDRESS_WATCH = 9,	/* DBG_HW_OP */
    KFD_IOC_DBG_TRAP_SET_FLAGS = 10,
    KFD_IOC_DBG_TRAP_QUERY_DEBUG_EVENT = 11,
    KFD_IOC_DBG_TRAP_QUERY_EXCEPTION_INFO = 12,
    KFD_IOC_DBG_TRAP_GET_QUEUE_SNAPSHOT = 13,
    KFD_IOC_DBG_TRAP_GET_DEVICE_SNAPSHOT = 14
}

//
// struct kfd_ioctl_dbg_trap_enable_args - Arguments for KFD_IOC_DBG_TRAP_ENABLE.
//
// Enables debug session for target process. Call @op KFD_IOC_DBG_TRAP_DISABLE in
// kfd_ioctl_dbg_trap_args to disable debug session.
//
// @exception_mask: (IN) - exceptions to raise to the debugger
// @rinfo_ptr:      (IN) - pointer to runtime info buffer (see kfd_runtime_info)
// @rinfo_size:     (IN/OUT) - size of runtime info buffer in bytes
// @dbg_fd:	        (IN) - fd the KFD will nofify the debugger with of raised
// exceptions set in exception_mask.
//
// Generic errors apply (see kfd_dbg_trap_operations).
// Return - 0 on SUCCESS.
// Copies KFD saved kfd_runtime_info to @rinfo_ptr on enable.
// Size of kfd_runtime saved by the KFD returned to @rinfo_size.
// - EBADF if KFD cannot get a reference to dbg_fd.
// - EFAULT if KFD cannot copy runtime info to rinfo_ptr.
// - EINVAL if target process is already debug enabled.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_ioctl_dbg_trap_enable_args {
    pub exception_mask: __u64,
    pub rinfo_ptr: __u64,
    pub rinfo_size: __u32,
    pub dbg_fd: __u32,
}

//
// struct kfd_ioctl_dbg_trap_send_runtime_event_args - Arguments for
// KFD_IOC_DBG_TRAP_SEND_RUNTIME_EVENT.
//
// Raises exceptions to runtime.
//
// @exception_mask: (IN) - exceptions to raise to runtime
// @gpu_id:	        (IN) - target device id
// @queue_id:       (IN) - target queue id
//
// Generic errors apply (see kfd_dbg_trap_operations).
// Return - 0 on SUCCESS.
// - ENODEV if gpu_id not found.
// If exception_mask contains EC_PROCESS_RUNTIME, unblocks pending
// AMDKFD_IOC_RUNTIME_ENABLE call - see kfd_ioctl_runtime_enable_args.
// All other exceptions are raised to runtime through err_payload_addr.
// See kfd_context_save_area_header.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_ioctl_dbg_trap_send_runtime_event_args {
    pub exception_mask: __u64,
    pub gpu_id: __u32,
    pub queue_id: __u32,
}

//
// struct kfd_ioctl_dbg_trap_set_exceptions_enabled_args - Arguments for
// KFD_IOC_SET_EXCEPTIONS_ENABLED
//
// Set new exceptions to be raised to the debugger.
//
// @exception_mask: (IN) - new exceptions to raise the debugger
//
// Generic errors apply (see kfd_dbg_trap_operations).
// Return - 0 on SUCCESS.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_ioctl_dbg_trap_set_exceptions_enabled_args {
    pub exception_mask: __u64,
}

//
// struct kfd_ioctl_dbg_trap_set_wave_launch_override_args - Arguments for
// KFD_IOC_DBG_TRAP_SET_WAVE_LAUNCH_OVERRIDE
//
// Enable HW exceptions to raise trap.
//
// @override_mode:	     (IN)     - see kfd_dbg_trap_override_mode
// @enable_mask:	     (IN/OUT) - reference kfd_dbg_trap_mask.
// IN is the override modes requested to be enabled.
// OUT is referenced in Return below.
// @support_request_mask: (IN/OUT) - reference kfd_dbg_trap_mask.
// IN is the override modes requested for support check.
// OUT is referenced in Return below.
//
// Generic errors apply (see kfd_dbg_trap_operations).
// Return - 0 on SUCCESS.
// Previous enablement is returned in @enable_mask.
// Actual override support is returned in @support_request_mask.
// - EINVAL if override mode is not supported.
// - EACCES if trap support requested is not actually supported.
// i.e. enable_mask (IN) is not a subset of support_request_mask (OUT).
// Otherwise it is considered a generic error (see kfd_dbg_trap_operations).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_ioctl_dbg_trap_set_wave_launch_override_args {
    pub override_mode: __u32,
    pub enable_mask: __u32,
    pub support_request_mask: __u32,
// private:
    pub pad: __u32,
}

//
// struct kfd_ioctl_dbg_trap_set_wave_launch_mode_args - Arguments for
// KFD_IOC_DBG_TRAP_SET_WAVE_LAUNCH_MODE
//
// Set wave launch mode.
//
// @launch_mode: (IN) - see kfd_dbg_trap_wave_launch_mode
//
// Generic errors apply (see kfd_dbg_trap_operations).
// Return - 0 on SUCCESS.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_ioctl_dbg_trap_set_wave_launch_mode_args {
    pub launch_mode: __u32,
// private:
    pub pad: __u32,
}

//
// struct kfd_ioctl_dbg_trap_suspend_queues_args - Arguments for
// KFD_IOC_DBG_TRAP_SUSPEND_QUEUES
//
// Suspend queues.
//
// @exception_mask:	(IN) - raised exceptions to clear
// @queue_array_ptr: (IN) - pointer to array of queue ids (u32 per queue id)
// to suspend
// @num_queues:	(IN) - number of queues to suspend in @queue_array_ptr
// @grace_period:	(IN) - wave time allowance before preemption
// per 1K GPU clock cycle unit
//
// Generic errors apply (see kfd_dbg_trap_operations).
// Destruction of a suspended queue is blocked until the queue is
// resumed.  This allows the debugger to access queue information and
// the its context save area without running into a race condition on
// queue destruction.
// Automatically copies per queue context save area header information
// into the save area base
// (see kfd_queue_snapshot_entry and kfd_context_save_area_header).
//
// Return - Number of queues suspended on SUCCESS.
// .	KFD_DBG_QUEUE_ERROR_MASK and KFD_DBG_QUEUE_INVALID_MASK masked
// for each queue id in @queue_array_ptr array reports unsuccessful
// suspend reason.
// KFD_DBG_QUEUE_ERROR_MASK = HW failure.
// KFD_DBG_QUEUE_INVALID_MASK = queue does not exist, is new or
// is being destroyed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_ioctl_dbg_trap_suspend_queues_args {
    pub exception_mask: __u64,
    pub queue_array_ptr: __u64,
    pub num_queues: __u32,
    pub grace_period: __u32,
}

//
// struct kfd_ioctl_dbg_trap_resume_queues_args - Arguments for
// KFD_IOC_DBG_TRAP_RESUME_QUEUES
//
// Resume queues.
//
// @queue_array_ptr: (IN) - pointer to array of queue ids (u32 per queue id)
// to resume
// @num_queues:	(IN) - number of queues to resume in @queue_array_ptr
//
// Generic errors apply (see kfd_dbg_trap_operations).
// Return - Number of queues resumed on SUCCESS.
// KFD_DBG_QUEUE_ERROR_MASK and KFD_DBG_QUEUE_INVALID_MASK mask
// for each queue id in @queue_array_ptr array reports unsuccessful
// resume reason.
// KFD_DBG_QUEUE_ERROR_MASK = HW failure.
// KFD_DBG_QUEUE_INVALID_MASK = queue does not exist.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_ioctl_dbg_trap_resume_queues_args {
    pub queue_array_ptr: __u64,
    pub num_queues: __u32,
// private:
    pub pad: __u32,
}

//
// struct kfd_ioctl_dbg_trap_set_node_address_watch_args - Arguments for
// KFD_IOC_DBG_TRAP_SET_NODE_ADDRESS_WATCH
//
// Sets address watch for device.
//
// @address: (IN)  - watch address to set
// @mode:    (IN)  - see kfd_dbg_trap_address_watch_mode
// @mask:    (IN)  - watch address mask
// @gpu_id:  (IN)  - target gpu to set watch point
// @id:      (OUT) - watch id allocated
//
// Generic errors apply (see kfd_dbg_trap_operations).
// Return - 0 on SUCCESS.
// Allocated watch ID returned to @id.
// - ENODEV if gpu_id not found.
// - ENOMEM if watch IDs can be allocated
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_ioctl_dbg_trap_set_node_address_watch_args {
    pub address: __u64,
    pub mode: __u32,
    pub mask: __u32,
    pub gpu_id: __u32,
    pub id: __u32,
}

//
// struct kfd_ioctl_dbg_trap_clear_node_address_watch_args - Arguments for
// KFD_IOC_DBG_TRAP_CLEAR_NODE_ADDRESS_WATCH
//
// Clear address watch for device.
//
// @gpu_id:  (IN)  - target device to clear watch point
// @id:      (IN) - allocated watch id to clear
//
// Generic errors apply (see kfd_dbg_trap_operations).
// Return - 0 on SUCCESS.
// - ENODEV if gpu_id not found.
// - EINVAL if watch ID has not been allocated.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_ioctl_dbg_trap_clear_node_address_watch_args {
    pub gpu_id: __u32,
    pub id: __u32,
}

//
// struct kfd_ioctl_dbg_trap_set_flags_args - Arguments for
// KFD_IOC_DBG_TRAP_SET_FLAGS
//
// Sets flags for wave behaviour.
//
// @flags: (IN/OUT) - IN = flags to enable, OUT = flags previously enabled
//
// Generic errors apply (see kfd_dbg_trap_operations).
// Return - 0 on SUCCESS.
// - EACCESS if any debug device does not allow flag options.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_ioctl_dbg_trap_set_flags_args {
    pub flags: __u32,
// private:
    pub pad: __u32,
}

//
// struct kfd_ioctl_dbg_trap_query_debug_event_args - Arguments for
// KFD_IOC_DBG_TRAP_QUERY_DEBUG_EVENT
//
// Find one or more raised exceptions. This function can return multiple
// exceptions from a single queue or a single device with one call. To find
// all raised exceptions, this function must be called repeatedly until it
// returns -EAGAIN. Returned exceptions can optionally be cleared by
// setting the corresponding bit in the @exception_mask input parameter.
// However, clearing an exception prevents retrieving further information
// about it with KFD_IOC_DBG_TRAP_QUERY_EXCEPTION_INFO.
//
// @exception_mask: (IN/OUT) - exception to clear (IN) and raised (OUT)
// @gpu_id:	        (OUT)    - gpu id of exceptions raised
// @queue_id:       (OUT)    - queue id of exceptions raised
//
// Generic errors apply (see kfd_dbg_trap_operations).
// Return - 0 on raised exception found
// Raised exceptions found are returned in @exception mask
// with reported source id returned in @gpu_id or @queue_id.
// - EAGAIN if no raised exception has been found
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_ioctl_dbg_trap_query_debug_event_args {
    pub exception_mask: __u64,
    pub gpu_id: __u32,
    pub queue_id: __u32,
}

//
// struct kfd_ioctl_dbg_trap_query_exception_info_args - Arguments for
// KFD_IOC_DBG_TRAP_QUERY_EXCEPTION_INFO
//
// Get additional info on raised exception.
//
// @info_ptr:	(IN)	 - pointer to exception info buffer to copy to
// @info_size:	(IN/OUT) - exception info buffer size (bytes)
// @source_id:	(IN)     - target gpu or queue id
// @exception_code:	(IN)     - target exception
// @clear_exception: (IN)     - clear raised @exception_code exception
// (0 = false, 1 = true)
//
// Generic errors apply (see kfd_dbg_trap_operations).
// Return - 0 on SUCCESS.
// If @exception_code is EC_DEVICE_MEMORY_VIOLATION, copy @info_size(OUT)
// bytes of memory exception data to @info_ptr.
// If @exception_code is EC_PROCESS_RUNTIME, copy saved
// kfd_runtime_info to @info_ptr.
// Actual required @info_ptr size (bytes) is returned in @info_size.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_ioctl_dbg_trap_query_exception_info_args {
    pub info_ptr: __u64,
    pub info_size: __u32,
    pub source_id: __u32,
    pub exception_code: __u32,
    pub clear_exception: __u32,
}

//
// struct kfd_ioctl_dbg_trap_queue_snapshot_args - Arguments for
// KFD_IOC_DBG_TRAP_GET_QUEUE_SNAPSHOT
//
// Get queue information.
//
// @exception_mask:	 (IN)	  - exceptions raised to clear
// @snapshot_buf_ptr: (IN)	  - queue snapshot entry buffer (see kfd_queue_snapshot_entry)
// @num_queues:	 (IN/OUT) - number of queue snapshot entries
// The debugger specifies the size of the array allocated in @num_queues.
// KFD returns the number of queues that actually existed. If this is
// larger than the size specified by the debugger, KFD will not overflow
// the array allocated by the debugger.
//
// @entry_size:	 (IN/OUT) - size per entry in bytes
// The debugger specifies sizeof(struct kfd_queue_snapshot_entry) in
// @entry_size. KFD returns the number of bytes actually populated per
// entry. The debugger should use the KFD_IOCTL_MINOR_VERSION to determine,
// which fields in struct kfd_queue_snapshot_entry are valid. This allows
// growing the ABI in a backwards compatible manner.
// Note that entry_size(IN) should still be used to stride the snapshot buffer in the
// event that it's larger than actual kfd_queue_snapshot_entry.
//
// Generic errors apply (see kfd_dbg_trap_operations).
// Return - 0 on SUCCESS.
// Copies @num_queues(IN) queue snapshot entries of size @entry_size(IN)
// into @snapshot_buf_ptr if @num_queues(IN) > 0.
// Otherwise return @num_queues(OUT) queue snapshot entries that exist.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_ioctl_dbg_trap_queue_snapshot_args {
    pub exception_mask: __u64,
    pub snapshot_buf_ptr: __u64,
    pub num_queues: __u32,
    pub entry_size: __u32,
}

//
// struct kfd_ioctl_dbg_trap_device_snapshot_args - Arguments for
// KFD_IOC_DBG_TRAP_GET_DEVICE_SNAPSHOT
//
// Get device information.
//
// @exception_mask:	  (IN)	  - exceptions raised to clear
// @snapshot_buf_ptr: (IN)	  - pointer to snapshot buffer (see kfd_dbg_device_info_entry)
// @num_devices:	  (IN/OUT) - number of debug devices to snapshot
// The debugger specifies the size of the array allocated in @num_devices.
// KFD returns the number of devices that actually existed. If this is
// larger than the size specified by the debugger, KFD will not overflow
// the array allocated by the debugger.
//
// @entry_size:	  (IN/OUT) - size per entry in bytes
// The debugger specifies sizeof(struct kfd_dbg_device_info_entry) in
// @entry_size. KFD returns the number of bytes actually populated. The
// debugger should use KFD_IOCTL_MINOR_VERSION to determine, which fields
// in struct kfd_dbg_device_info_entry are valid. This allows growing the
// ABI in a backwards compatible manner.
// Note that entry_size(IN) should still be used to stride the snapshot buffer in the
// event that it's larger than actual kfd_dbg_device_info_entry.
//
// Generic errors apply (see kfd_dbg_trap_operations).
// Return - 0 on SUCCESS.
// Copies @num_devices(IN) device snapshot entries of size @entry_size(IN)
// into @snapshot_buf_ptr if @num_devices(IN) > 0.
// Otherwise return @num_devices(OUT) queue snapshot entries that exist.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_ioctl_dbg_trap_device_snapshot_args {
    pub exception_mask: __u64,
    pub snapshot_buf_ptr: __u64,
    pub num_devices: __u32,
    pub entry_size: __u32,
}

//
// struct kfd_ioctl_dbg_trap_args - Arguments to debug target process.
//
// @pid: target process to debug
// @op:  debug operation (see kfd_dbg_trap_operations)
//
// @op determines which union struct args to use.
// Refer to kern docs for each kfd_ioctl_dbg_trap_*_args struct.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_ioctl_dbg_trap_args {
    pub pid: __u32,
    pub op: __u32,
    pub enable: kfd_ioctl_dbg_trap_enable_args,
    pub send_runtime_event: kfd_ioctl_dbg_trap_send_runtime_event_args,
    pub set_exceptions_enabled: kfd_ioctl_dbg_trap_set_exceptions_enabled_args,
    pub launch_override: kfd_ioctl_dbg_trap_set_wave_launch_override_args,
    pub launch_mode: kfd_ioctl_dbg_trap_set_wave_launch_mode_args,
    pub suspend_queues: kfd_ioctl_dbg_trap_suspend_queues_args,
    pub resume_queues: kfd_ioctl_dbg_trap_resume_queues_args,
    pub set_node_address_watch: kfd_ioctl_dbg_trap_set_node_address_watch_args,
    pub clear_node_address_watch: kfd_ioctl_dbg_trap_clear_node_address_watch_args,
    pub set_flags: kfd_ioctl_dbg_trap_set_flags_args,
    pub query_debug_event: kfd_ioctl_dbg_trap_query_debug_event_args,
    pub query_exception_info: kfd_ioctl_dbg_trap_query_exception_info_args,
    pub queue_snapshot: kfd_ioctl_dbg_trap_queue_snapshot_args,
    pub device_snapshot: kfd_ioctl_dbg_trap_device_snapshot_args,
}

pub const KFD_IOC_PROFILER_VERSION_NUM: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kfd_profiler_ops {
    KFD_IOC_PROFILER_PMC = 0,
    KFD_IOC_PROFILER_VERSION = 2,
    KFD_IOC_PROFILER_PTL_CONTROL = 3,
}

//
// struct kfd_ioctl_pmc_settings - Enables/Disables GPU Specific profiler settings
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_ioctl_pmc_settings {
    pub /: *mut *mut __u32 gpu_id; / This is the user_gpu_id,
    pub /: *mut *mut __u32 lock; / Lock GPU for Profiling,
    pub /: *mut *mut __u32 perfcount_enable; / Force Perfcount Enable for queues on GPU,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_ioctl_ptl_control {
    pub /: *mut *mut __u32 gpu_id; / user_gpu_id,
    pub /: *mut *mut __u32 enable; / set 1 to enable PTL, set 0 to disable PTL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_ioctl_profiler_args {
    pub /: *mut *mut __u32 op; / kfd_profiler_op,
    pub pmc: kfd_ioctl_pmc_settings,
    pub ptl: kfd_ioctl_ptl_control,
    pub /: *mut *mut __u32 version; / KFD_IOC_PROFILER_VERSION_NUM,
}

pub const AMDKFD_COMMAND_START: c_uint = 0x01;
pub const AMDKFD_COMMAND_END: c_uint = 0x29;
