//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdkfd/kfd_priv.h
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

// Macro flag: #define KFD_PRIV_H_INCLUDED

pub const KFD_MAX_RING_ENTRY_SIZE: c_int = 8;
pub const KFD_SYSFS_FILE_MODE: c_int = 0444;
// GPU ID hash width in bits
pub const KFD_GPU_ID_HASH_WIDTH: c_int = 16;
// Use upper bits of mmap offset to store KFD driver specific information.
// BITS[63:62] - Encode MMAP type
// BITS[61:46] - Encode gpu_id. To identify to which GPU the offset belongs to
// BITS[45:0]  - MMAP offset value
//
// NOTE: struct vm_area_struct.vm_pgoff uses offset in pages. Hence, these
// defines are w.r.t to PAGE_SIZE
//
pub const KFD_MMAP_TYPE_SHIFT: c_int = 62;

pub const KFD_MMAP_GPU_ID_SHIFT: c_int = 46;

//
// When working with cp scheduler we should assign the HIQ manually or via
// the amdgpu driver to a fixed hqd slot, here are the fixed HIQ hqd slot
// definitions for Kaveri. In Kaveri only the first ME queues participates
// in the cp scheduling taking that in mind we set the HIQ slot in the
// second ME.
//
pub const KFD_CIK_HIQ_PIPE: c_int = 4;
pub const KFD_CIK_HIQ_QUEUE: c_int = 0;
// Macro for allocating structures

pub const KFD_MAX_NUM_OF_PROCESSES: c_int = 512;
pub const KFD_MAX_NUM_OF_QUEUES_PER_PROCESS: c_int = 1024;
//
// Size of the per-process TBA+TMA buffer: 2 pages
//
// The first chunk is the TBA used for the CWSR ISA code. The second
// chunk is used as TMA for user-mode trap handler setup in daisy-chain mode.
//

pub const KFD_KERNEL_QUEUE_SIZE: c_int = 2048;
// KFD_UNMAP_LATENCY_MS is the timeout CP waiting for SDMA preemption. One XCC
// can be associated to 2 SDMA engines. queue_preemption_timeout_ms is the time
// driver waiting for CP returning the UNMAP_QUEUE fence. Thus the math is
// queue_preemption_timeout_ms = sdma_preemption_time * 2 + cp workload
// The format here makes CP workload 10% of total timeout
//

pub const KFD_MAX_SDMA_QUEUES: c_int = 128;
//
// 512 = 0x200
// The doorbell index distance between SDMA RLC (2*i) and (2*i+1) in the
// same SDMA engine on SOC15, which has 8-byte doorbells for SDMA.
// 512 8-byte doorbell distance (i.e. one page away) ensures that SDMA RLC
// (2*i+1) doorbells (in terms of the lower 12 bit address) lie exactly in
// the OFFSET and SIZE set in registers like BIF_SDMA0_DOORBELL_RANGE.
//
pub const KFD_QUEUE_DOORBELL_MIRROR_OFFSET: c_int = 512;
//
// enum kfd_ioctl_flags - KFD ioctl flags
// Various flags that can be set in &amdkfd_ioctl_desc.flags to control how
// userspace can use a given ioctl.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kfd_ioctl_flags {
//
// @KFD_IOC_FLAG_CHECKPOINT_RESTORE:
// Certain KFD ioctls such as AMDKFD_IOC_CRIU_OP can potentially
// perform privileged operations and load arbitrary data into MQDs and
// eventually HQD registers when the queue is mapped by HWS. In order to
// prevent this we should perform additional security checks.
//
// This is equivalent to callers with the CHECKPOINT_RESTORE capability.
//
// Note: Since earlier versions of docker do not support CHECKPOINT_RESTORE,
// we also allow ioctls with SYS_ADMIN capability.
//
    KFD_IOC_FLAG_CHECKPOINT_RESTORE = BIT(0),
}

//
// Kernel module parameter to specify maximum number of supported queues per
// device
//
// Kernel module parameter to specify the scheduling policy
//
// Kernel module parameter to specify the maximum process
// number per HW scheduler
//
// Kernel module parameter to specify whether to send sigterm to HSA process on
// unhandled exception
//
// This kernel module is used to simulate large bar machine on non-large bar
// enabled machines.
//
// Set sh_mem_config.retry_disable on GFX v9
// Halt if HWS hang is detected
// Whether MEC FW support GWS barriers
// Queue preemption timeout in ms
//
// Don't evict process queues on vm fault
//
// Enable eviction debug messages
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cache_policy {
    cache_policy_coherent,
    cache_policy_noncoherent
}

// Macro flag: #define KFD_SUPPORT_XNACK_PER_PROCESS(dev)\
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_event_interrupt_class {
    pub patched_flag): *mut bool,
    pub ih_ring_entry): *const u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_device_info {
    pub gfx_target_version: u32,
    pub event_interrupt_class: *const kfd_event_interrupt_class,
    pub max_pasid_bits: c_uint,
    pub max_no_of_hqd: c_uint,
    pub doorbell_size: c_uint,
    pub ih_ring_entry_size: usize,
    pub num_of_watch_points: u8,
    pub mqd_size_aligned: u16,
    pub supports_cwsr: bool,
    pub needs_pci_atomics: bool,
    pub no_atomic_fw_version: u32,
    pub num_sdma_queues_per_engine: c_uint,
    pub num_reserved_sdma_queues_per_engine: c_uint,
}

extern "C" {
    pub fn kfd_get_num_sdma_engines(kdev: *mut kfd_node) -> c_uint;
}
extern "C" {
    pub fn kfd_get_num_xgmi_sdma_engines(kdev: *mut kfd_node) -> c_uint;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_mem_obj {
    pub range_start: u32,
    pub range_end: u32,
    pub gpu_addr: u64,
    pub cpu_ptr: *mut u32,
    pub mem: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_vmid_info {
    pub first_vmid_kfd: u32,
    pub last_vmid_kfd: u32,
    pub vmid_num_kfd: u32,
}

pub const MAX_KFD_NODES: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_node {
    pub node_id: c_uint,
    pub keeping: *mut *mut *mut amdgpu_device adev; / Duplicated here along with,
// a copy in kfd_dev to save a hop
//
    pub with: *const *const *const kfd2kgd_calls kfd2kgd; / Duplicated here along,
// keeping a copy in kfd_dev to
// save a hop
//
    pub vm_info: kfd_vmid_info,
    pub /: *mut *mut unsigned int id; / topology stub index,
    pub /: *mut *mut uint32_t xcc_mask; / Instance mask of XCCs present,
    pub xcp: *mut amdgpu_xcp,
// Interrupts
    pub ih_fifo: kfifo,
    pub interrupt_work: work_struct,
    pub interrupt_lock: spinlock_t,
//
// Interrupts of interest to KFD are copied
// from the HW ring into a SW ring.
//
    pub interrupts_active: bool,
    pub /: *mut *mut uint32_t interrupt_bitmap; / Only used for GFX 9.4.3,
// QCM Device instance
    pub dqm: *mut device_queue_manager,
// Global GWS resource shared between processes
    pub gws: *mut c_void,
// Clients watching SMI events
    pub smi_clients: list_head,
    pub smi_lock: spinlock_t,
    pub reset_seq_num: u32,
// SRAM ECC flag
    pub sram_ecc_flag: core::sync::atomic::AtomicI32,
// spm process id
    pub spm_pasid: c_uint,
// Maximum process number mapped to HW scheduler
    pub max_proc_per_quantum: c_uint,
    pub compute_vmid_bitmap: c_uint,
    pub local_mem_info: kfd_local_mem_info,
    pub kfd: *mut kfd_dev,
// Track per device allocated watch points
    pub alloc_watch_ids: u32,
    pub watch_points_lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_dev {
    pub adev: *mut amdgpu_device,
    pub device_info: kfd_device_info,
    pub doorbells: *mut *mut *mut u32 __iomem doorbell_kernel_ptr; / This is a pointer for a,
// page used by kernel queue
//
    pub shared_resources: kgd2kfd_shared_resources,
    pub kfd2kgd: *const kfd2kgd_calls,
    pub doorbell_mutex: mutex,
    pub gtt_mem: *mut c_void,
    pub gtt_start_gpu_addr: u64,
    pub gtt_start_cpu_ptr: *mut c_void,
    pub gtt_sa_bitmap: *mut c_void,
    pub gtt_sa_lock: mutex,
    pub gtt_sa_chunk_size: c_uint,
    pub gtt_sa_num_of_chunks: c_uint,
    pub init_complete: bool,
// Firmware versions
    pub mec_fw_version: u16,
    pub mec2_fw_version: u16,
    pub sdma_fw_version: u16,
// CWSR
    pub cwsr_enabled: bool,
    pub cwsr_isa: *const c_void,
    pub cwsr_isa_size: c_uint,
// xGMI
    pub hive_id: u64,
    pub pci_atomic_requested: bool,
// Compute Profile ref. count
    pub compute_profile: core::sync::atomic::AtomicI32,
    pub doorbell_ida: ida,
    pub max_doorbell_slices: c_uint,
    pub noretry: c_int,
    pub nodes: [*mut kfd_node; MAX_KFD_NODES],
    pub num_nodes: c_uint,
    pub ih_wq: *mut workqueue_struct,
// Kernel doorbells for KFD device
    pub doorbells: *mut amdgpu_bo,
// bitmap for dynamic doorbell allocation from doorbell object
    pub doorbell_bitmap: *mut c_ulong,
// for dynamic partitioning
    pub kfd_dev_lock: c_int,
    pub kfd_processes_count: core::sync::atomic::AtomicI32,
// Lock for profiler process
    pub profiler_lock: mutex,
// Process currently holding the lock
    pub profiler_process: *mut kfd_process,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kfd_mempool {
    KFD_MEMPOOL_SYSTEM_CACHEABLE = 1,
    KFD_MEMPOOL_SYSTEM_WRITECOMBINE = 2,
    KFD_MEMPOOL_FRAMEBUFFER = 3,
}

// Character device interface
extern "C" {
    pub fn kfd_chardev_init() -> c_int;
}
extern "C" {
    pub fn kfd_chardev_exit();
}
extern "C" {
    pub fn kfd_dev_unmap_mapping_range(holebegin: loff_t const, holelen: loff_t const);
}
//
// enum kfd_unmap_queues_filter - Enum for queue filters.
//
// @KFD_UNMAP_QUEUES_FILTER_ALL_QUEUES: Preempts all queues in the
// running queues list.
//
// @KFD_UNMAP_QUEUES_FILTER_DYNAMIC_QUEUES: Preempts all non-static queues
// in the run list.
//
// @KFD_UNMAP_QUEUES_FILTER_BY_PASID: Preempts queues that belongs to
// specific process.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kfd_unmap_queues_filter {
    KFD_UNMAP_QUEUES_FILTER_ALL_QUEUES = 1,
    KFD_UNMAP_QUEUES_FILTER_DYNAMIC_QUEUES = 2,
    KFD_UNMAP_QUEUES_FILTER_BY_PASID = 3
}

//
// enum kfd_queue_type - Enum for various queue types.
//
// @KFD_QUEUE_TYPE_COMPUTE: Regular user mode queue type.
//
// @KFD_QUEUE_TYPE_SDMA: SDMA user mode queue type.
//
// @KFD_QUEUE_TYPE_HIQ: HIQ queue type.
//
// @KFD_QUEUE_TYPE_DIQ: DIQ queue type.
//
// @KFD_QUEUE_TYPE_SDMA_XGMI: Special SDMA queue for XGMI interface.
//
// @KFD_QUEUE_TYPE_SDMA_BY_ENG_ID:  SDMA user mode queue with target SDMA engine ID.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kfd_queue_type {
    KFD_QUEUE_TYPE_COMPUTE,
    KFD_QUEUE_TYPE_SDMA,
    KFD_QUEUE_TYPE_HIQ,
    KFD_QUEUE_TYPE_SDMA_XGMI,
    KFD_QUEUE_TYPE_SDMA_BY_ENG_ID,
    KFD_QUEUE_TYPE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kfd_queue_format {
    KFD_QUEUE_FORMAT_PM4,
    KFD_QUEUE_FORMAT_AQL
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum KFD_QUEUE_PRIORITY {
    KFD_QUEUE_PRIORITY_MINIMUM = 0,
    KFD_QUEUE_PRIORITY_MAXIMUM = 15
}

//
// struct queue_properties
//
// @type: The queue type.
//
// @queue_id: Queue identifier.
//
// @queue_address: Queue ring buffer address.
//
// @queue_size: Queue ring buffer size.
//
// @priority: Defines the queue priority relative to other queues in the
// process.
// This is just an indication and HW scheduling may override the priority as
// necessary while keeping the relative prioritization.
// the priority granularity is from 0 to f which f is the highest priority.
// currently all queues are initialized with the highest priority.
//
// @queue_percent: This field is partially implemented and currently a zero in
// this field defines that the queue is non active.
//
// @read_ptr: User space address which points to the number of dwords the
// cp read from the ring buffer. This field updates automatically by the H/W.
//
// @write_ptr: Defines the number of dwords written to the ring buffer.
//
// @doorbell_ptr: Notifies the H/W of new packet written to the queue ring
// buffer. This field should be similar to write_ptr and the user should
// update this field after updating the write_ptr.
//
// @doorbell_off: The doorbell offset in the doorbell pci-bar.
//
// @is_interop: Defines if this is a interop queue. Interop queue means that
// the queue can access both graphics and compute resources.
//
// @is_evicted: Defines if the queue is evicted. Only active queues
// are evicted, rendering them inactive.
//
// @is_active: Defines if the queue is active or not. @is_active and
// @is_evicted are protected by the DQM lock.
//
// @is_gws: Defines if the queue has been updated to be GWS-capable or not.
// @is_gws should be protected by the DQM lock, since changing it can yield the
// possibility of updating DQM state on number of GWS queues.
//
// @vmid: If the scheduling mode is no cp scheduling the field defines the vmid
// of the queue.
//
// This structure represents the queue properties for each queue no matter if
// it's user mode or kernel mode queue.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct queue_properties {
    pub type: kfd_queue_type,
    pub format: kfd_queue_format,
    pub queue_id: c_uint,
    pub queue_address: u64,
    pub queue_size: u64,
    pub metadata_queue_size: u64,
    pub priority: u32,
    pub queue_percent: u32,
    pub read_ptr: *mut void __user,
    pub write_ptr: *mut void __user,
    pub doorbell_ptr: *mut void __iomem,
    pub doorbell_off: u32,
    pub is_interop: bool,
    pub is_evicted: bool,
    pub is_suspended: bool,
    pub is_being_destroyed: bool,
    pub is_active: bool,
    pub is_gws: bool,
    pub pm4_target_xcc: u32,
    pub is_dbg_wa: bool,
    pub is_user_cu_masked: bool,
    pub is_reset: bool,
// Not relevant for user mode queues in cp scheduling
    pub vmid: c_uint,
// Relevant only for sdma queues
    pub sdma_engine_id: u32,
    pub sdma_queue_id: u32,
    pub sdma_vm_addr: u32,
// Relevant only for VI
    pub eop_ring_buffer_address: u64,
    pub eop_ring_buffer_size: u32,
    pub ctx_save_restore_area_address: u64,
    pub ctx_save_restore_area_size: u32,
    pub ctl_stack_size: u32,
    pub tba_addr: u64,
    pub tma_addr: u64,
    pub exception_status: u64,
    pub wptr_bo: *mut amdgpu_bo,
    pub rptr_bo: *mut amdgpu_bo,
    pub ring_bo: *mut amdgpu_bo,
    pub eop_buf_bo: *mut amdgpu_bo,
    pub cwsr_bo: *mut amdgpu_bo,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mqd_update_flag {
    UPDATE_FLAG_DBG_WA_ENABLE = 1,
    UPDATE_FLAG_DBG_WA_DISABLE = 2,
    UPDATE_FLAG_IS_GWS = 4, /* quirk for gfx9 IP */
    UPDATE_FLAG_PERFCOUNT_ENABLE = 5,
    UPDATE_FLAG_PERFCOUNT_DISABLE = 6,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mqd_update_info {
    pub /: *mut *mut uint32_t count; / Must be a multiple of 32,
    pub ptr: *mut u32,
    pub cu_mask: },
}

//
// struct queue
//
// @list: Queue linked list.
//
// @mqd: The queue MQD (memory queue descriptor).
//
// @mqd_mem_obj: The MQD local gpu memory object.
//
// @gart_mqd_addr: The MQD gart mc address.
//
// @properties: The queue properties.
//
// @mec: Used only in no cp scheduling mode and identifies to micro engine id
// that the queue should be executed on.
//
// @pipe: Used only in no cp scheduling mode and identifies the queue's pipe
// id.
//
// @queue: Used only in no cp scheduliong mode and identifies the queue's slot.
//
// @process: The kfd process that created this queue.
//
// @device: The kfd device that created this queue.
//
// @gws: Pointing to gws kgd_mem if this is a gws control queue; NULL
// otherwise.
//
// This structure represents user mode compute queues.
// It contains all the necessary data to handle such queues.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct queue {
    pub list: list_head,
    pub mqd: *mut c_void,
    pub mqd_mem_obj: *mut kfd_mem_obj,
    pub gart_mqd_addr: u64,
    pub properties: queue_properties,
    pub mec: u32,
    pub pipe: u32,
    pub queue: u32,
    pub sdma_id: c_uint,
    pub doorbell_id: c_uint,
    pub process: *mut kfd_process,
    pub device: *mut kfd_node,
    pub gws: *mut c_void,
// procfs
    pub kobj: kobject,
    pub gang_ctx_bo: *mut c_void,
    pub gang_ctx_gpu_addr: u64,
    pub gang_ctx_cpu_ptr: *mut c_void,
    pub gang_ctx_array_index: u32,
    pub wptr_bo_gart: *mut amdgpu_bo,
// The VRAM-resident MQD BO (mqd_on_vram()) is unpinned at S4 suspend so
// TTM evicts it into the hibernation image, and repinned on resume. Set
// while the BO is unpinned so the resume path knows to repin it.
//
    pub needs_mqd_repin: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum KFD_MQD_TYPE {
    KFD_MQD_TYPE_HIQ = 0,		/* for hiq */
    KFD_MQD_TYPE_CP,		/* for cp queues and diq */
    KFD_MQD_TYPE_SDMA,		/* for sdma queues */
    KFD_MQD_TYPE_DIQ,		/* for diq */
    KFD_MQD_TYPE_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum KFD_PIPE_PRIORITY {
    KFD_PIPE_PRIORITY_CS_LOW = 0,
    KFD_PIPE_PRIORITY_CS_MEDIUM,
    KFD_PIPE_PRIORITY_CS_HIGH
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scheduling_resources {
    pub vmid_mask: c_uint,
    pub type: kfd_queue_type,
    pub queue_mask: u64,
    pub gws_mask: u64,
    pub oac_mask: u32,
    pub gds_heap_base: u32,
    pub gds_heap_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct process_queue_manager {
// data
    pub process: *mut kfd_process,
    pub queues: list_head,
    pub queue_slot_bitmap: *mut c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcm_process_device {
// The Device Queue Manager that owns this data
    pub dqm: *mut device_queue_manager,
    pub pqm: *mut process_queue_manager,
// Queues list
    pub queues_list: list_head,
    pub priv_queue_list: list_head,
    pub queue_count: c_uint,
    pub vmid: c_uint,
    pub is_debug: bool,
    pub /: *mut *mut unsigned int evicted; / eviction counter, 0=active,
// This flag tells if we should reset all wavefronts on
// process termination
//
    pub reset_wavefronts: bool,
// This flag tells us if this process has a GWS-capable
// queue that will be mapped into the runlist. It's
// possible to request a GWS BO, but not have the queue
// currently mapped, and this changes how the MAP_PROCESS
// PM4 packet is configured.
//
    pub mapped_gws_queue: bool,
// All the memory management data should be here too
    pub gds_context_area: u64,
// Contains page table flags such as AMDGPU_PTE_VALID since gfx9
    pub page_table_base: u64,
    pub sh_mem_config: u32,
    pub sh_mem_bases: u32,
    pub sh_mem_ape1_base: u32,
    pub sh_mem_ape1_limit: u32,
    pub gds_size: u32,
    pub num_gws: u32,
    pub num_oac: u32,
    pub sh_hidden_private_base: u32,
    pub vm_cntx_cntl: u32,
// CWSR memory
    pub cwsr_mem: *mut kgd_mem,
    pub cwsr_map: iosys_map,
    pub cwsr_base: u64,
    pub tba_addr: u64,
    pub tma_addr: u64,
// IB memory
    pub ib_mem: *mut kgd_mem,
    pub ib_base: u64,
    pub ib_kaddr: *mut c_void,
// doorbells for kfd process
    pub proc_doorbells: *mut amdgpu_bo,
// bitmap for dynamic doorbell allocation from the bo
    pub doorbell_bitmap: *mut c_ulong,
}

// KFD Memory Eviction
// Approx. wait time before attempting to restore evicted BOs
pub const PROCESS_RESTORE_TIME_MS: c_int = 100;
// Approx. back off time if restore fails due to lack of memory
pub const PROCESS_BACK_OFF_TIME_MS: c_int = 100;
// Approx. time before evicting the process again
pub const PROCESS_ACTIVE_TIME_MS: c_int = 10;
// 8 byte handle containing GPU ID in the most significant 4 bytes and
// idr_handle in the least significant 4 bytes
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kfd_pdd_bound {
    PDD_UNBOUND = 0,
    PDD_BOUND,
    PDD_BOUND_SUSPENDED,
}

pub const MAX_SYSFS_FILENAME_LEN: c_int = 15;
//
// SDMA counter runs at 100MHz frequency.
// We display SDMA activity in microsecond granularity in sysfs.
// As a result, the divisor is 100.
//
pub const SDMA_ACTIVITY_DIVISOR: c_int = 100;
// Data that is per-process-per device.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_process_device {
// The device that owns this data.
    pub dev: *mut kfd_node,
// The process that owns this kfd_process_device.
    pub process: *mut kfd_process,
// per-process-per device QCM data structure
    pub qpd: qcm_process_device,
// Apertures
    pub lds_base: u64,
    pub lds_limit: u64,
    pub gpuvm_base: u64,
    pub gpuvm_limit: u64,
    pub scratch_base: u64,
    pub scratch_limit: u64,
// VM context for GPUVM allocations
    pub drm_file: *mut file,
    pub drm_priv: *mut c_void,
// GPUVM allocations storage
    pub alloc_idr: idr,
// Flag used to tell the pdd has dequeued from the dqm.
// This is used to prevent dev->dqm->ops.process_termination() from
// being called twice when it is already called in IOMMU callback
// function.
//
    pub already_dequeued: bool,
    pub runtime_inuse: bool,
// Is this process/pasid bound to this device? (amd_iommu_bind_pasid)
    pub bound: kfd_pdd_bound,
// VRAM usage
    pub vram_usage: core::sync::atomic::AtomicI64,
    pub attr_vram: attribute,
    pub vram_filename: [c_char; MAX_SYSFS_FILENAME_LEN],
// SDMA activity tracking
    pub sdma_past_activity_counter: u64,
    pub attr_sdma: attribute,
    pub sdma_filename: [c_char; MAX_SYSFS_FILENAME_LEN],
// Eviction activity tracking
    pub last_evict_timestamp: u64,
    pub evict_duration_counter: core::sync::atomic::AtomicI64,
    pub attr_evict: attribute,
    pub kobj_stats: *mut kobject,
//
// @cu_occupancy: Reports occupancy of Compute Units (CU) of a process
// that is associated with device encoded by "this" struct instance. The
// value reflects CU usage by all of the waves launched by this process
// on this device. A very important property of occupancy parameter is
// that its value is a snapshot of current use.
//
// Following is to be noted regarding how this parameter is reported:
//
// The number of waves that a CU can launch is limited by couple of
// parameters. These are encoded by struct amdgpu_cu_info instance
// that is part of every device definition. For GFX9 devices this
// translates to 40 waves (simd_per_cu * max_waves_per_simd) when waves
// do not use scratch memory and 32 waves (max_scratch_slots_per_cu)
// when they do use scratch memory. This could change for future
// devices and therefore this example should be considered as a guide.
//
// All CU's of a device are available for the process. This may not be true
// under certain conditions - e.g. CU masking.
//
// Finally number of CU's that are occupied by a process is affected by both
// number of CU's a device has along with number of other competing processes
//
    pub attr_cu_occupancy: attribute,
// sysfs counters for GPU retry fault and page migration tracking
    pub kobj_counters: *mut kobject,
    pub attr_faults: attribute,
    pub attr_page_in: attribute,
    pub attr_page_out: attribute,
    pub faults: u64,
    pub page_in: u64,
    pub page_out: u64,
// Exception code status
    pub exception_status: u64,
    pub vm_fault_exc_data: *mut c_void,
    pub vm_fault_exc_data_size: usize,
// Tracks debug per-vmid request settings
    pub spi_dbg_override: u32,
    pub spi_dbg_launch_mode: u32,
    pub watch_points: [u32; 4],
    pub alloc_watch_ids: u32,
//
// If this process has been checkpointed before, then the user
// application will use the original gpu_id on the
// checkpointed node to refer to this device.
//
    pub user_gpu_id: u32,
    pub proc_ctx_bo: *mut c_void,
    pub proc_ctx_gpu_addr: u64,
    pub proc_ctx_cpu_ptr: *mut c_void,
    pub proc_ctx_array_index: u32,
// Tracks queue reset status
    pub has_reset_queue: bool,
    pub pasid: u32,
// Indicates this process has requested PTL stay disabled
    pub ptl_disable_req: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct svm_range_list {
    pub lock: mutex,
    pub objects: rb_root_cached,
    pub list: list_head,
    pub deferred_list_work: work_struct,
    pub deferred_range_list: list_head,
    pub criu_svm_metadata_list: list_head,
    pub deferred_list_lock: spinlock_t,
    pub evicted_ranges: core::sync::atomic::AtomicI32,
    pub drain_pagefaults: core::sync::atomic::AtomicI32,
    pub restore_work: delayed_work,
    pub MAX_GPU_INSTANCE): DECLARE_BITMAP(bitmap_supported,,
    pub faulting_task: *mut task_struct,
// check point ts decides if page fault recovery need be dropped
    pub checkpoint_ts: [core::sync::atomic::AtomicI64; MAX_GPU_INSTANCE],
// Default granularity to use in buffer migration
// and restoration of backing memory while handling
// recoverable page faults
//
    pub default_granularity: u8,
}

// Process data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_process {
//
// kfd_process are stored in an mm_struct*->kfd_process
// hash table (kfd_processes in kfd_process.c)
//
    pub kfd_processes: hlist_node,
//
// Opaque pointer to mm_struct. We don't hold a reference to
// it so it should never be dereferenced from here. This is
// only used for looking up processes by their mm.
//
    pub mm: *mut c_void,
    pub ref: kref,
    pub release_work: work_struct,
    pub mutex: mutex,
//
// In any process, the thread that started main() is the lead
// thread and outlives the rest.
// It is here because amd_iommu_bind_pasid wants a task_struct.
// It can also be used for safely getting a reference to the
// mm_struct of the process.
//
    pub lead_thread: *mut task_struct,
// We want to receive a notification when the mm_struct is destroyed
    pub mmu_notifier: mmu_notifier,
//
// Array of kfd_process_device pointers,
// one for each device the process is using.
//
    pub pdds: [*mut kfd_process_device; MAX_GPU_INSTANCE],
    pub n_pdds: u32,
    pub pqm: process_queue_manager,
// Is the user space process 32 bit?
    pub is_32bit_user_mode: bool,
// Event-related data
    pub event_mutex: mutex,
// Event ID allocator and lookup
    pub event_idr: idr,
// Event page
    pub signal_handle: u64,
//
// Each signal event needs a 64-bit signal slot where the signaler will
// write a 1 before sending an interrupt. (This is needed because some
// interrupts do not contain enough spare data bits to identify an
// event.) The signal page is allocated in user mode and mapped to the
// kernel; individual signal events use their event_id as slot index.
//
    pub signal_page: *mut u64,
    pub signal_mapped_size: usize,
    pub signal_event_count: usize,
    pub signal_event_limit_reached: bool,
//
// @kfd_sigbus_delay_ms: Per-process KFD SIGBUS delivery option for
// poison/RAS events (set via DRM_IOCTL_AMDGPU_PROC_OPTIONS
// AMDGPU_PROC_OPTIONS_OP_KFD_SIGBUS_DELAY).
//
// 0          - send SIGBUS immediately (default)
// 0xFFFFFFFF - suppress SIGBUS delivery
// other      - delay SIGBUS delivery by this many milliseconds
//
    pub kfd_sigbus_delay_ms: core::sync::atomic::AtomicI32,
// Delayed signal delivery to user
    pub signal_work: delayed_work,
// Information used for memory eviction
    pub kgd_process_info: *mut c_void,
// Eviction fence that is attached to all the BOs of this process. The
// fence will be triggered during eviction and new one will be created
// during restore
//
    pub ef: *mut dma_fence __rcu,
// Work items for evicting and restoring BOs
    pub eviction_work: delayed_work,
    pub restore_work: delayed_work,
// seqno of the last scheduled eviction
    pub last_eviction_seqno: c_uint,
// Approx. the last timestamp (in jiffies) when the process was
// restored after an eviction
//
    pub last_restore_timestamp: c_ulong,
// Indicates device process is debug attached with reserved vmid.
    pub debug_trap_enabled: bool,
// per-process-per device debug event fd file
    pub dbg_ev_file: *mut file,
// If the process is a kfd debugger, we need to know so we can clean
// up at exit time.  If a process enables debugging on itself, it does
// its own clean-up, so we don't set the flag here.  We track this by
// counting the number of processes this process is debugging.
//
    pub debugged_process_count: core::sync::atomic::AtomicI32,
// If the process is a debugged, this is the debugger process
    pub debugger_process: *mut kfd_process,
// Kobj for our procfs
    pub kobj: *mut kobject,
    pub kobj_queues: *mut kobject,
    pub attr_pasid: attribute,
// Exception code enable mask and status
    pub exception_enable_mask: u64,
    pub exception_status: u64,
// Used to drain stale interrupts
    pub wait_irq_drain: wait_queue_head_t,
    pub irq_drain_is_open: bool,
// shared virtual memory registered by this process
    pub svms: svm_range_list,
    pub xnack_enabled: bool,
// Work area for debugger event writer worker.
    pub debug_event_workarea: work_struct,
// Tracks debug per-vmid request for debug flags
    pub dbg_flags: u32,
    pub poison: core::sync::atomic::AtomicI32,
// Queues are in paused stated because we are in the process of doing a CRIU checkpoint
    pub queues_paused: bool,
// Tracks runtime enable status
    pub runtime_enable_sema: semaphore,
    pub is_runtime_retry: bool,
    pub runtime_info: kfd_runtime_info,
// if gpu page fault sent to KFD
    pub gpu_page_fault: bool,
// kfd context id
    pub context_id: u16,
// The primary kfd_process allocating IDs for its secondary kfd_process, 0 for primary kfd_process
    pub id_table: ida,
}

pub const KFD_CONTEXT_ID_PRIMARY: c_uint = 0xFFFF;
pub const KFD_CONTEXT_ID_MIN: c_int = 0;
extern "C" {
    pub fn DECLARE_HASHTABLE(_arg: kfd_processes_table, _arg: KFD_PROCESS_TABLE_SIZE) -> extern;
}
//
// typedef amdkfd_ioctl_t - typedef for ioctl function pointer.
//
// @filep: pointer to file structure.
// @p: amdkfd process pointer.
// @data: pointer to arg that was copied from user.
//
// Return: returns ioctl completion code.
//
extern "C" {
    pub fn amdkfd_ioctl_validate_t(kdata: *mut c_void, usize: c_uint) -> typedef int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdkfd_ioctl_desc {
    pub cmd: c_uint,
    pub flags: c_int,
    pub func: *mut amdkfd_ioctl_t,
    pub validate: *mut amdkfd_ioctl_validate_t,
    pub cmd_drv: c_uint,
    pub name: *const c_char,
}

extern "C" {
    pub fn kfd_dev_is_large_bar(dev: *mut kfd_node) -> bool;
}
extern "C" {
    pub fn kfd_process_create_wq() -> c_int;
}
extern "C" {
    pub fn kfd_process_destroy_wq();
}
extern "C" {
    pub fn kfd_cleanup_processes();
}
extern "C" {
    pub fn kfd_create_process_sysfs(process: *mut kfd_process) -> c_int;
}
extern "C" {
    pub fn kfd_process_gpuidx_from_gpuid(p: *mut kfd_process, gpu_id: u32) -> c_int;
}
extern "C" {
    pub fn kfd_unref_process(p: *mut kfd_process);
}
extern "C" {
    pub fn kfd_process_evict_queues(p: *mut kfd_process, trigger: u32) -> c_int;
}
extern "C" {
    pub fn kfd_process_restore_queues(p: *mut kfd_process) -> c_int;
}
extern "C" {
    pub fn kfd_suspend_all_processes();
}
extern "C" {
    pub fn kfd_resume_all_processes() -> c_int;
}
extern "C" {
    pub fn kfd_process_get_user_gpu_id(p: *mut kfd_process, actual_gpu_id: u32) -> c_int;
}
extern "C" {
    pub fn kfd_process_xnack_mode(p: *mut kfd_process, supported: bool) -> bool;
}
extern "C" {
    pub fn kfd_process_notifier_release_internal(p: *mut kfd_process);
}
// KFD process API for creating and translating handles
// PASIDs
extern "C" {
    pub fn kfd_pasid_init() -> c_int;
}
extern "C" {
    pub fn kfd_pasid_exit();
}
extern "C" {
    pub fn kfd_pasid_alloc() -> u32;
}
extern "C" {
    pub fn kfd_pasid_free(pasid: u32);
}
// Doorbells
extern "C" {
    pub fn kfd_doorbell_process_slice(kfd: *mut kfd_dev) -> usize;
}
extern "C" {
    pub fn kfd_doorbell_init(kfd: *mut kfd_dev) -> c_int;
}
extern "C" {
    pub fn kfd_doorbell_fini(kfd: *mut kfd_dev);
}
extern "C" {
    pub fn kfd_release_kernel_doorbell(kfd: *mut kfd_dev, db_addr: *mut u32 __iomem);
}
extern "C" {
    pub fn read_kernel_doorbell(db: *mut u32 __iomem) -> u32;
}
extern "C" {
    pub fn write_kernel_doorbell(db: *mut void __iomem, value: u32);
}
extern "C" {
    pub fn write_kernel_doorbell64(db: *mut void __iomem, value: u64);
}
extern "C" {
    pub fn kfd_get_process_doorbells(pdd: *mut kfd_process_device) -> phys_addr_t;
}
// GTT Sub-Allocator
extern "C" {
    pub fn kfd_gtt_sa_free(node: *mut kfd_node, mem_obj: *mut kfd_mem_obj) -> c_int;
}
// KFD's procfs
extern "C" {
    pub fn kfd_procfs_init();
}
extern "C" {
    pub fn kfd_procfs_shutdown();
}
extern "C" {
    pub fn kfd_procfs_add_queue(q: *mut queue) -> c_int;
}
extern "C" {
    pub fn kfd_procfs_del_queue(q: *mut queue);
}
// Topology
extern "C" {
    pub fn kfd_topology_init() -> c_int;
}
extern "C" {
    pub fn kfd_topology_shutdown();
}
extern "C" {
    pub fn kfd_topology_add_device(gpu: *mut kfd_node) -> c_int;
}
extern "C" {
    pub fn kfd_topology_remove_device(gpu: *mut kfd_node) -> c_int;
}
//
// On multi-aid system, attempt per-node matching. Otherwise,
// fall back to the first node.
//
extern "C" {
    pub fn kfd_topology_enum_kfd_devices(idx: u8, kdev: *mut kfd_node) -> c_int;
}
extern "C" {
    pub fn kfd_topology_get_num_devices() -> u32;
}
extern "C" {
    pub fn kfd_numa_node_to_apic_id(numa_node_id: c_int) -> c_int;
}
extern "C" {
    pub fn kfd_gpu_node_num() -> u32;
}
// Interrupts
pub const KFD_IRQ_FENCE_CLIENTID: c_uint = 0xff;
pub const KFD_IRQ_FENCE_SOURCEID: c_uint = 0xff;

extern "C" {
    pub fn kfd_interrupt_init(dev: *mut kfd_node) -> c_int;
}
extern "C" {
    pub fn kfd_interrupt_exit(dev: *mut kfd_node);
}
extern "C" {
    pub fn enqueue_ih_ring_entry(kfd: *mut kfd_node, ih_ring_entry: *const c_void) -> bool;
}
extern "C" {
    pub fn kfd_process_drain_interrupts(pdd: *mut kfd_process_device) -> c_int;
}
extern "C" {
    pub fn kfd_process_close_interrupt_drain(pasid: c_uint);
}
// amdkfd Apertures
extern "C" {
    pub fn kfd_init_apertures(process: *mut kfd_process) -> c_int;
}
// CRIU
//
// Need to increment KFD_CRIU_PRIV_VERSION each time a change is made to any of the CRIU private
// structures:
// kfd_criu_process_priv_data
// kfd_criu_device_priv_data
// kfd_criu_bo_priv_data
// kfd_criu_queue_priv_data
// kfd_criu_event_priv_data
// kfd_criu_svm_range_priv_data
//
pub const KFD_CRIU_PRIV_VERSION: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_criu_process_priv_data {
    pub version: u32,
    pub xnack_mode: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_criu_device_priv_data {
// For future use
    pub reserved: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_criu_bo_priv_data {
    pub user_addr: u64,
    pub idr_handle: u32,
    pub mapped_gpuids: [u32; MAX_GPU_INSTANCE],
}

//
// The first 4 bytes of kfd_criu_queue_priv_data, kfd_criu_event_priv_data,
// kfd_criu_svm_range_priv_data is the object type
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kfd_criu_object_type {
    KFD_CRIU_OBJECT_TYPE_QUEUE,
    KFD_CRIU_OBJECT_TYPE_EVENT,
    KFD_CRIU_OBJECT_TYPE_SVM_RANGE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_criu_svm_range_priv_data {
    pub object_type: u32,
    pub start_addr: u64,
    pub size: u64,
// Variable length array of attributes
    pub attrs: [kfd_ioctl_svm_attribute; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_criu_queue_priv_data {
    pub object_type: u32,
    pub q_address: u64,
    pub q_size: u64,
    pub read_ptr_addr: u64,
    pub write_ptr_addr: u64,
    pub doorbell_off: u64,
    pub eop_ring_buffer_address: u64,
    pub ctx_save_restore_area_address: u64,
    pub gpu_id: u32,
    pub type: u32,
    pub format: u32,
    pub q_id: u32,
    pub priority: u32,
    pub q_percent: u32,
    pub doorbell_id: u32,
    pub gws: u32,
    pub sdma_id: u32,
    pub eop_ring_buffer_size: u32,
    pub ctx_save_restore_area_size: u32,
    pub ctl_stack_size: u32,
    pub mqd_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_criu_event_priv_data {
    pub object_type: u32,
    pub user_handle: u64,
    pub event_id: u32,
    pub auto_reset: u32,
    pub type: u32,
    pub signaled: u32,
    pub memory_exception_data: kfd_hsa_memory_exception_data,
    pub hw_exception_data: kfd_hsa_hw_exception_data,
}

// CRIU - End
// Queue Context Management
extern "C" {
    pub fn init_queue(q: *mut queue, properties: *const queue_properties) -> c_int;
}
extern "C" {
    pub fn uninit_queue(q: *mut queue);
}
extern "C" {
    pub fn print_queue_properties(q: *mut queue_properties);
}
extern "C" {
    pub fn print_queue(q: *mut queue);
}
extern "C" {
    pub fn kfd_queue_buffer_put(bo: *mut amdgpu_bo);
}
extern "C" {
    pub fn kfd_queue_acquire_buffers(pdd: *mut kfd_process_device, properties: *mut queue_properties) -> c_int;
}
extern "C" {
    pub fn kfd_queue_release_buffers(pdd: *mut kfd_process_device, properties: *mut queue_properties) -> c_int;
}
extern "C" {
    pub fn kfd_queue_unref_bo_va(vm: *mut amdgpu_vm, bo: *mut amdgpu_bo);
}
extern "C" {
    pub fn kfd_queue_ctx_save_restore_size(dev: *mut kfd_topology_device);
}
extern "C" {
    pub fn device_queue_manager_uninit(dqm: *mut device_queue_manager);
}
extern "C" {
    pub fn kernel_queue_uninit(kq: *mut kernel_queue);
}
extern "C" {
    pub fn kfd_evict_process_device(pdd: *mut kfd_process_device) -> c_int;
}
extern "C" {
    pub fn kfd_dqm_suspend_bad_queue_mes(knode: *mut kfd_node, pasid: u32, doorbell_id: u32) -> c_int;
}
// Process Queue Manager
#[repr(C)]
#[derive(Copy, Clone)]
pub struct process_queue_node {
    pub q: *mut queue,
    pub kq: *mut kernel_queue,
    pub process_queue_list: list_head,
}

extern "C" {
    pub fn kfd_process_dequeue_from_device(pdd: *mut kfd_process_device);
}
extern "C" {
    pub fn kfd_process_dequeue_from_all_devices(p: *mut kfd_process);
}
extern "C" {
    pub fn pqm_init(pqm: *mut process_queue_manager, p: *mut kfd_process) -> c_int;
}
extern "C" {
    pub fn pqm_uninit(pqm: *mut process_queue_manager);
}
extern "C" {
    pub fn pqm_destroy_queue(pqm: *mut process_queue_manager, qid: c_uint) -> c_int;
}
// Packet Manager

//
// enum kfd_config_dequeue_wait_counts_cmd - Command for configuring
// dequeue wait counts.
//
// @KFD_DEQUEUE_WAIT_INIT: Set optimized dequeue wait counts for a
// certain ASICs. For these ASICs, this is default value used by RESET
// @KFD_DEQUEUE_WAIT_RESET: Reset dequeue wait counts to the optimized value
// for certain ASICs. For others set it to default hardware reset value
// @KFD_DEQUEUE_WAIT_SET_SCH_WAVE: Set context switch latency wait
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kfd_config_dequeue_wait_counts_cmd {
    KFD_DEQUEUE_WAIT_INIT = 1,
    KFD_DEQUEUE_WAIT_RESET = 2,
    KFD_DEQUEUE_WAIT_SET_SCH_WAVE = 3
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct packet_manager {
    pub dqm: *mut device_queue_manager,
    pub priv_queue: *mut kernel_queue,
    pub lock: mutex,
    pub allocated: bool,
    pub ib_buffer_obj: *mut kfd_mem_obj,
    pub ib_size_bytes: c_uint,
    pub is_over_subscription: bool,
    pub pmf: *const packet_manager_funcs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct packet_manager_funcs {
// Support ASIC-specific packet formats for PM4 packets
    pub qpd): *mut qcm_process_device,
    pub chain): uint64_t ib, size_t ib_size_in_dwords, bool,
    pub res): *mut scheduling_resources,
    pub is_static): *mut *mut queue q, bool,
    pub reset): uint32_t filter_param, bool,
    pub value): kfd_config_dequeue_wait_counts_cmd cmd, uint32_t,
    pub fence_value): uint64_t fence_address, uint64_t,
    pub buffer): *mut *mut int (release_mem)(uint64_t gpu_addr, uint32_t,
// Packet sizes
    pub map_process_size: c_int,
    pub runlist_size: c_int,
    pub set_resources_size: c_int,
    pub map_queues_size: c_int,
    pub unmap_queues_size: c_int,
    pub config_dequeue_wait_counts_size: c_int,
    pub query_status_size: c_int,
    pub release_mem_size: c_int,
}

extern "C" {
    pub fn pm_init(pm: *mut packet_manager, dqm: *mut device_queue_manager) -> c_int;
}
extern "C" {
    pub fn pm_uninit(pm: *mut packet_manager);
}
extern "C" {
    pub fn pm_send_runlist(pm: *mut packet_manager, dqm_queues: *mut list_head) -> c_int;
}
extern "C" {
    pub fn pm_release_ib(pm: *mut packet_manager);
}
// Following PM funcs can be shared among VI and AI
extern "C" {
    pub fn pm_build_pm4_header(opcode: c_uint, packet_size: usize) -> c_uint;
}
extern "C" {
    pub fn kfd_get_number_elems(kfd: *mut kfd_dev) -> u64;
}
// Events
extern "C" {
    pub fn kfd_event_init_process(p: *mut kfd_process) -> c_int;
}
extern "C" {
    pub fn kfd_event_free_process(p: *mut kfd_process);
}
extern "C" {
    pub fn kfd_signal_hw_exception_event(pasid: u32);
}
extern "C" {
    pub fn kfd_set_event(p: *mut kfd_process, event_id: u32) -> c_int;
}
extern "C" {
    pub fn kfd_reset_event(p: *mut kfd_process, event_id: u32) -> c_int;
}
extern "C" {
    pub fn kfd_kmap_event_page(p: *mut kfd_process, event_page_offset: u64) -> c_int;
}
extern "C" {
    pub fn kfd_get_num_events(p: *mut kfd_process) -> c_int;
}
extern "C" {
    pub fn kfd_event_destroy(p: *mut kfd_process, event_id: u32) -> c_int;
}
extern "C" {
    pub fn kfd_signal_vm_fault_event_with_userptr(p: *mut kfd_process, gpu_va: u64);
}
extern "C" {
    pub fn kfd_signal_reset_event(dev: *mut kfd_node);
}
extern "C" {
    pub fn kfd_signal_poison_consumed_event(dev: *mut kfd_node, pasid: u32);
}
extern "C" {
    pub fn kfd_signal_sigbus_delayed_fn(work: *mut work_struct);
}
extern "C" {
    pub fn kfd_signal_process_terminate_event(p: *mut kfd_process);
}
extern "C" {
    pub fn kfd_is_locked(kfd: *mut kfd_dev) -> bool;
}
// Compute profile
extern "C" {
    pub fn kfd_inc_compute_active(dev: *mut kfd_node);
}
extern "C" {
    pub fn kfd_dec_compute_active(dev: *mut kfd_node);
}
// Cgroup Support
// Check with device cgroup if @kfd device is accessible

// PTL support
// Debugfs

extern "C" {
    pub fn kfd_debugfs_init();
}
extern "C" {
    pub fn kfd_debugfs_fini();
}
extern "C" {
    pub fn kfd_debugfs_mqds_by_process(m: *mut seq_file, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn pqm_debugfs_mqds(m: *mut seq_file, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn kfd_debugfs_hqds_by_device(m: *mut seq_file, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn dqm_debugfs_hqds(m: *mut seq_file, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn kfd_debugfs_rls_by_device(m: *mut seq_file, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn pm_debugfs_runlist(m: *mut seq_file, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn kfd_debugfs_hang_hws(dev: *mut kfd_node) -> c_int;
}
extern "C" {
    pub fn pm_debugfs_hang_hws(pm: *mut packet_manager) -> c_int;
}
extern "C" {
    pub fn dqm_debugfs_hang_hws(dqm: *mut device_queue_manager) -> c_int;
}
extern "C" {
    pub fn kfd_debugfs_add_process(p: *mut kfd_process) -> c_int;
}
extern "C" {
    pub fn kfd_debugfs_remove_process(p: *mut kfd_process);
}

