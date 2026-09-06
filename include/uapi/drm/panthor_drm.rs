//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/drm/panthor_drm.h
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
// Copyright (C) 2023 Collabora ltd.

//
// DOC: Introduction
//
// This documentation describes the Panthor IOCTLs.
//
// Just a few generic rules about the data passed to the Panthor IOCTLs:
//
// - Structures must be aligned on 64-bit/8-byte. If the object is not
// naturally aligned, a padding field must be added.
// - Fields must be explicitly aligned to their natural type alignment with
// pad[0..N] fields.
// - All padding fields will be checked by the driver to make sure they are
// zeroed.
// - Flags can be added, but not removed/replaced.
// - New fields can be added to the main structures (the structures
// directly passed to the ioctl). Those fields can be added at the end of
// the structure, or replace existing padding fields. Any new field being
// added must preserve the behavior that existed before those fields were
// added when a value of zero is passed.
// - New fields can be added to indirect objects (objects pointed by the
// main structure), iff those objects are passed a size to reflect the
// size known by the userspace driver (see drm_panthor_obj_array::stride
// or drm_panthor_dev_query::size).
// - If the kernel driver is too old to know some fields, those will be
// ignored if zero, and otherwise rejected (and so will be zero on output).
// - If userspace is too old to know some fields, those will be zeroed
// (input) before the structure is parsed by the kernel driver.
// - Each new flag/field addition must come with a driver version update so
// the userspace driver doesn't have to trial and error to know which
// flags are supported.
// - Structures should not contain unions, as this would defeat the
// extensibility of such structures.
// - IOCTLs can't be removed or replaced. New IOCTL IDs should be placed
// at the end of the drm_panthor_ioctl_id enum.
//
// DOC: MMIO regions exposed to userspace.
//
// .. c:macro:: DRM_PANTHOR_USER_MMIO_OFFSET
//
// File offset for all MMIO regions being exposed to userspace. Don't use
// this value directly, use DRM_PANTHOR_USER_<name>_OFFSET values instead.
// pgoffset passed to mmap2() is an unsigned long, which forces us to use a
// different offset on 32-bit and 64-bit systems.
//
// .. c:macro:: DRM_PANTHOR_USER_FLUSH_ID_MMIO_OFFSET
//
// File offset for the LATEST_FLUSH_ID register. The Userspace driver controls
// GPU cache flushing through CS instructions, but the flush reduction
// mechanism requires a flush_id. This flush_id could be queried with an
// ioctl, but Arm provides a well-isolated register page containing only this
// read-only register, so let's expose this page through a static mmap offset
// and allow direct mapping of this MMIO region so we can avoid the
// user <-> kernel round-trip.
//

//
// DOC: IOCTL IDs
//
// enum drm_panthor_ioctl_id - IOCTL IDs
//
// Place new ioctls at the end, don't re-order, don't replace or remove entries.
//
// These IDs are not meant to be used directly. Use the DRM_IOCTL_PANTHOR_xxx
// definitions instead.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_panthor_ioctl_id {
// @DRM_PANTHOR_DEV_QUERY: Query device information.
    DRM_PANTHOR_DEV_QUERY = 0,

// @DRM_PANTHOR_VM_CREATE: Create a VM.
    DRM_PANTHOR_VM_CREATE,

// @DRM_PANTHOR_VM_DESTROY: Destroy a VM.
    DRM_PANTHOR_VM_DESTROY,

// @DRM_PANTHOR_VM_BIND: Bind/unbind memory to a VM.
    DRM_PANTHOR_VM_BIND,

// @DRM_PANTHOR_VM_GET_STATE: Get VM state.
    DRM_PANTHOR_VM_GET_STATE,

// @DRM_PANTHOR_BO_CREATE: Create a buffer object.
    DRM_PANTHOR_BO_CREATE,

//
// @DRM_PANTHOR_BO_MMAP_OFFSET: Get the file offset to pass to
// mmap to map a GEM object.
//
    DRM_PANTHOR_BO_MMAP_OFFSET,

// @DRM_PANTHOR_GROUP_CREATE: Create a scheduling group.
    DRM_PANTHOR_GROUP_CREATE,

// @DRM_PANTHOR_GROUP_DESTROY: Destroy a scheduling group.
    DRM_PANTHOR_GROUP_DESTROY,

//
// @DRM_PANTHOR_GROUP_SUBMIT: Submit jobs to queues belonging
// to a specific scheduling group.
//
    DRM_PANTHOR_GROUP_SUBMIT,

// @DRM_PANTHOR_GROUP_GET_STATE: Get the state of a scheduling group.
    DRM_PANTHOR_GROUP_GET_STATE,

// @DRM_PANTHOR_TILER_HEAP_CREATE: Create a tiler heap.
    DRM_PANTHOR_TILER_HEAP_CREATE,

// @DRM_PANTHOR_TILER_HEAP_DESTROY: Destroy a tiler heap.
    DRM_PANTHOR_TILER_HEAP_DESTROY,

// @DRM_PANTHOR_BO_SET_LABEL: Label a BO.
    DRM_PANTHOR_BO_SET_LABEL,

//
// @DRM_PANTHOR_SET_USER_MMIO_OFFSET: Set the offset to use as the user MMIO offset.
//
// The default behavior is to pick the MMIO offset based on the size of the pgoff_t
// type seen by the process that manipulates the FD, such that a 32-bit process can
// always map the user MMIO ranges. But this approach doesn't work well for emulators
// like FEX, where the emulator is an 64-bit binary which might be executing 32-bit
// code. In that case, the kernel thinks it's the 64-bit process and assumes
// DRM_PANTHOR_USER_MMIO_OFFSET_64BIT is in use, but the UMD library expects
// DRM_PANTHOR_USER_MMIO_OFFSET_32BIT, because it can't mmap() anything above the
// pgoff_t size.
//
    DRM_PANTHOR_SET_USER_MMIO_OFFSET,

// @DRM_PANTHOR_BO_SYNC: Sync BO data to/from the device
    DRM_PANTHOR_BO_SYNC,

//
// @DRM_PANTHOR_BO_QUERY_INFO: Query information about a BO.
//
// This is useful for imported BOs.
//
    DRM_PANTHOR_BO_QUERY_INFO,
}

//
// DOC: IOCTL arguments
//
// struct drm_panthor_obj_array - Object array.
//
// This object is used to pass an array of objects whose size is subject to changes in
// future versions of the driver. In order to support this mutability, we pass a stride
// describing the size of the object as known by userspace.
//
// You shouldn't fill drm_panthor_obj_array fields directly. You should instead use
// the DRM_PANTHOR_OBJ_ARRAY() macro that takes care of initializing the stride to
// the object size.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panthor_obj_array {
// @stride: Stride of object struct. Used for versioning.
    pub stride: __u32,
// @count: Number of objects in the array.
    pub count: __u32,
// @array: User pointer to an array of objects.
    pub array: __u64,
}

//
// DRM_PANTHOR_OBJ_ARRAY() - Initialize a drm_panthor_obj_array field.
// @cnt: Number of elements in the array.
// @ptr: Pointer to the array to pass to the kernel.
//
// Macro initializing a drm_panthor_obj_array based on the object size as known
// by userspace.
//

//
// enum drm_panthor_sync_op_flags - Synchronization operation flags.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_panthor_sync_op_flags {
// @DRM_PANTHOR_SYNC_OP_HANDLE_TYPE_MASK: Synchronization handle type mask.
    DRM_PANTHOR_SYNC_OP_HANDLE_TYPE_MASK = 0xff,

// @DRM_PANTHOR_SYNC_OP_HANDLE_TYPE_SYNCOBJ: Synchronization object type.
    DRM_PANTHOR_SYNC_OP_HANDLE_TYPE_SYNCOBJ = 0,

//
// @DRM_PANTHOR_SYNC_OP_HANDLE_TYPE_TIMELINE_SYNCOBJ: Timeline synchronization
// object type.
//
    DRM_PANTHOR_SYNC_OP_HANDLE_TYPE_TIMELINE_SYNCOBJ = 1,

// @DRM_PANTHOR_SYNC_OP_WAIT: Wait operation.
    DRM_PANTHOR_SYNC_OP_WAIT = 0 << 31,

// @DRM_PANTHOR_SYNC_OP_SIGNAL: Signal operation.
    DRM_PANTHOR_SYNC_OP_SIGNAL = (int)(1u << 31),
}

//
// struct drm_panthor_sync_op - Synchronization operation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panthor_sync_op {
// @flags: Synchronization operation flags. Combination of DRM_PANTHOR_SYNC_OP values.
    pub flags: __u32,
// @handle: Sync handle.
    pub handle: __u32,
//
// @timeline_value: MBZ if
// (flags & DRM_PANTHOR_SYNC_OP_HANDLE_TYPE_MASK) !=
// DRM_PANTHOR_SYNC_OP_HANDLE_TYPE_TIMELINE_SYNCOBJ.
//
    pub timeline_value: __u64,
}

//
// enum drm_panthor_dev_query_type - Query type
//
// Place new types at the end, don't re-order, don't remove or replace.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_panthor_dev_query_type {
// @DRM_PANTHOR_DEV_QUERY_GPU_INFO: Query GPU information.
    DRM_PANTHOR_DEV_QUERY_GPU_INFO = 0,

// @DRM_PANTHOR_DEV_QUERY_CSIF_INFO: Query command-stream interface information.
    DRM_PANTHOR_DEV_QUERY_CSIF_INFO,

// @DRM_PANTHOR_DEV_QUERY_TIMESTAMP_INFO: Query timestamp information.
    DRM_PANTHOR_DEV_QUERY_TIMESTAMP_INFO,

//
// @DRM_PANTHOR_DEV_QUERY_GROUP_PRIORITIES_INFO: Query allowed group priorities information.
//
    DRM_PANTHOR_DEV_QUERY_GROUP_PRIORITIES_INFO,

// @DRM_PANTHOR_DEV_QUERY_MMU_INFO: Query MMU information.
    DRM_PANTHOR_DEV_QUERY_MMU_INFO,
}

//
// enum drm_panthor_gpu_coherency: Type of GPU coherency
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_panthor_gpu_coherency {
//
// @DRM_PANTHOR_GPU_COHERENCY_ACE_LITE: ACE Lite coherency.
//
    DRM_PANTHOR_GPU_COHERENCY_ACE_LITE = 0,

//
// @DRM_PANTHOR_GPU_COHERENCY_ACE: ACE coherency.
//
    DRM_PANTHOR_GPU_COHERENCY_ACE = 1,

//
// @DRM_PANTHOR_GPU_COHERENCY_NONE: No coherency.
//
    DRM_PANTHOR_GPU_COHERENCY_NONE = 31,
}

//
// struct drm_panthor_gpu_info - GPU information
//
// Structure grouping all queryable information relating to the GPU.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panthor_gpu_info {
// @gpu_id : GPU ID.
    pub gpu_id: __u32,

// @gpu_rev: GPU revision.
    pub gpu_rev: __u32,
// @csf_id: Command stream frontend ID.
    pub csf_id: __u32,

// @l2_features: L2-cache features.
    pub l2_features: __u32,
// @tiler_features: Tiler features.
    pub tiler_features: __u32,
// @mem_features: Memory features.
    pub mem_features: __u32,
// @mmu_features: MMU features.
    pub mmu_features: __u32,

// @thread_features: Thread features.
    pub thread_features: __u32,
// @max_threads: Maximum number of threads.
    pub max_threads: __u32,
// @thread_max_workgroup_size: Maximum workgroup size.
    pub thread_max_workgroup_size: __u32,
//
// @thread_max_barrier_size: Maximum number of threads that can wait
// simultaneously on a barrier.
//
    pub thread_max_barrier_size: __u32,
//
// @coherency_features: Coherency features.
//
// Combination of drm_panthor_gpu_coherency flags.
//
// Note that this is just what the coherency protocols supported by the
// GPU, but the actual coherency in place depends on the SoC
// integration and is reflected by
// drm_panthor_gpu_info::selected_coherency.
//
    pub coherency_features: __u32,
// @texture_features: Texture features.
    pub texture_features: [__u32; 4],
// @as_present: Bitmask encoding the number of address-space exposed by the MMU.
    pub as_present: __u32,
//
// @selected_coherency: Coherency selected for this device.
//
// One of drm_panthor_gpu_coherency.
//
    pub selected_coherency: __u32,
// @shader_present: Bitmask encoding the shader cores exposed by the GPU.
    pub shader_present: __u64,
// @l2_present: Bitmask encoding the L2 caches exposed by the GPU.
    pub l2_present: __u64,
// @tiler_present: Bitmask encoding the tiler units exposed by the GPU.
    pub tiler_present: __u64,
// @core_features: Used to discriminate core variants when they exist.
    pub core_features: __u32,
// @pad: MBZ.
    pub pad: __u32,
// @gpu_features: Bitmask describing supported GPU-wide features
    pub gpu_features: __u64,
}

//
// struct drm_panthor_csif_info - Command stream interface information
//
// Structure grouping all queryable information relating to the command stream interface.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panthor_csif_info {
// @csg_slot_count: Number of command stream group slots exposed by the firmware.
    pub csg_slot_count: __u32,
// @cs_slot_count: Number of command stream slots per group.
    pub cs_slot_count: __u32,
// @cs_reg_count: Number of command stream registers.
    pub cs_reg_count: __u32,
// @scoreboard_slot_count: Number of scoreboard slots.
    pub scoreboard_slot_count: __u32,
//
// @unpreserved_cs_reg_count: Number of command stream registers reserved by
// the kernel driver to call a userspace command stream.
//
// All registers can be used by a userspace command stream, but the
// [cs_slot_count - unpreserved_cs_reg_count .. cs_slot_count] registers are
// used by the kernel when DRM_PANTHOR_IOCTL_GROUP_SUBMIT is called.
//
    pub unpreserved_cs_reg_count: __u32,
//
// @pad: Padding field, set to zero.
//
    pub pad: __u32,
}

//
// enum drm_panthor_timestamp_info_flags - drm_panthor_timestamp_info.flags
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_panthor_timestamp_info_flags {
// @DRM_PANTHOR_TIMESTAMP_GPU: Query GPU time.
    DRM_PANTHOR_TIMESTAMP_GPU = 1 << 0,

// @DRM_PANTHOR_TIMESTAMP_CPU_NONE: Don't query CPU time.
    DRM_PANTHOR_TIMESTAMP_CPU_NONE = 0 << 1,

// @DRM_PANTHOR_TIMESTAMP_CPU_MONOTONIC: Query CPU time using CLOCK_MONOTONIC.
    DRM_PANTHOR_TIMESTAMP_CPU_MONOTONIC = 1 << 1,

// @DRM_PANTHOR_TIMESTAMP_CPU_MONOTONIC_RAW: Query CPU time using CLOCK_MONOTONIC_RAW.
    DRM_PANTHOR_TIMESTAMP_CPU_MONOTONIC_RAW = 2 << 1,

// @DRM_PANTHOR_TIMESTAMP_CPU_TYPE_MASK: Space reserved for CPU clock type.
    DRM_PANTHOR_TIMESTAMP_CPU_TYPE_MASK = 7 << 1,

// @DRM_PANTHOR_TIMESTAMP_GPU_OFFSET: Query GPU offset.
    DRM_PANTHOR_TIMESTAMP_GPU_OFFSET = 1 << 4,

// @DRM_PANTHOR_TIMESTAMP_GPU_CYCLE_COUNT: Query GPU cycle count.
    DRM_PANTHOR_TIMESTAMP_GPU_CYCLE_COUNT = 1 << 5,

// @DRM_PANTHOR_TIMESTAMP_FREQ: Query timestamp frequency.
    DRM_PANTHOR_TIMESTAMP_FREQ = 1 << 6,

// @DRM_PANTHOR_TIMESTAMP_DURATION: Return duration of time query.
    DRM_PANTHOR_TIMESTAMP_DURATION = 1 << 7,
}

//
// struct drm_panthor_timestamp_info - Timestamp information
//
// Structure grouping all queryable information relating to the GPU timestamp.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panthor_timestamp_info {
//
// @timestamp_frequency: The frequency of the timestamp timer or 0 if
// unknown.
//
    pub timestamp_frequency: __u64,
// @current_timestamp: The current GPU timestamp.
    pub current_timestamp: __u64,
// @timestamp_offset: The offset of the GPU timestamp timer.
    pub timestamp_offset: __u64,
//
// @flags: Bitmask of drm_panthor_timestamp_info_flags.
//
// If set to 0, then it is interpreted as:
// DRM_PANTHOR_TIMESTAMP_GPU |
// DRM_PANTHOR_TIMESTAMP_GPU_OFFSET |
// DRM_PANTHOR_TIMESTAMP_FREQ
//
// Note: these flags are exclusive to each other (only one can be used):
// - DRM_PANTHOR_TIMESTAMP_CPU_NONE
// - DRM_PANTHOR_TIMESTAMP_CPU_MONOTONIC
// - DRM_PANTHOR_TIMESTAMP_CPU_MONOTONIC_RAW
//
    pub flags: __u32,
// @duration_nsec: Duration of time query.
    pub duration_nsec: __u32,
// @cycle_count: Value of GPU_CYCLE_COUNT.
    pub cycle_count: __u64,
// @cpu_timestamp_sec: Seconds part of CPU timestamp.
    pub cpu_timestamp_sec: __u64,
// @cpu_timestamp_nsec: Nanseconds part of CPU timestamp.
    pub cpu_timestamp_nsec: __u64,
}

//
// struct drm_panthor_mmu_info - MMU information
//
// Structure grouping all queryable information relating to the MMU.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panthor_mmu_info {
// @page_size_bitmap: Allowed page sizes
    pub page_size_bitmap: __u64,
}

//
// struct drm_panthor_group_priorities_info - Group priorities information
//
// Structure grouping all queryable information relating to the allowed group priorities.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panthor_group_priorities_info {
//
// @allowed_mask: Bitmask of the allowed group priorities.
//
// Each bit represents a variant of the enum drm_panthor_group_priority.
//
    pub allowed_mask: __u8,
// @pad: Padding fields, MBZ.
    pub pad: [__u8; 3],
}

//
// struct drm_panthor_dev_query - Arguments passed to DRM_PANTHOR_IOCTL_DEV_QUERY
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panthor_dev_query {
// @type: the query type (see drm_panthor_dev_query_type).
    pub type: __u32,
//
// @size: size of the type being queried.
//
// If pointer is NULL, size is updated by the driver to provide the
// output structure size. If pointer is not NULL, the driver will
// only copy min(size, actual_structure_size) bytes to the pointer,
// and update the size accordingly. This allows us to extend query
// types without breaking userspace.
//
    pub size: __u32,
//
// @pointer: user pointer to a query type struct.
//
// Pointer can be NULL, in which case, nothing is copied, but the
// actual structure size is returned. If not NULL, it must point to
// a location that's large enough to hold size bytes.
//
    pub pointer: __u64,
}

//
// struct drm_panthor_vm_create - Arguments passed to DRM_PANTHOR_IOCTL_VM_CREATE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panthor_vm_create {
// @flags: VM flags, MBZ.
    pub flags: __u32,
// @id: Returned VM ID.
    pub id: __u32,
//
// @user_va_range: Size of the VA space reserved for user objects.
//
// The kernel will pick the remaining space to map kernel-only objects to the
// VM (heap chunks, heap context, ring buffers, kernel synchronization objects,
// ...). If the space left for kernel objects is too small, kernel object
// allocation will fail further down the road. One can use
// drm_panthor_gpu_info::mmu_features to extract the total virtual address
// range, and chose a user_va_range that leaves some space to the kernel.
//
// If user_va_range is zero, the kernel will pick a sensible value based on
// TASK_SIZE and the virtual range supported by the GPU MMU (the kernel/user
// split should leave enough VA space for userspace processes to support SVM,
// while still allowing the kernel to map some amount of kernel objects in
// the kernel VA range). The value chosen by the driver will be returned in
// @user_va_range.
//
// User VA space always starts at 0x0, kernel VA space is always placed after
// the user VA range.
//
    pub user_va_range: __u64,
}

//
// struct drm_panthor_vm_destroy - Arguments passed to DRM_PANTHOR_IOCTL_VM_DESTROY
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panthor_vm_destroy {
// @id: ID of the VM to destroy.
    pub id: __u32,
// @pad: MBZ.
    pub pad: __u32,
}

//
// enum drm_panthor_vm_bind_op_flags - VM bind operation flags
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_panthor_vm_bind_op_flags {
//
// @DRM_PANTHOR_VM_BIND_OP_MAP_READONLY: Map the memory read-only.
//
// Only valid with DRM_PANTHOR_VM_BIND_OP_TYPE_MAP.
//
    DRM_PANTHOR_VM_BIND_OP_MAP_READONLY = 1 << 0,

//
// @DRM_PANTHOR_VM_BIND_OP_MAP_NOEXEC: Map the memory not-executable.
//
// Only valid with DRM_PANTHOR_VM_BIND_OP_TYPE_MAP.
//
    DRM_PANTHOR_VM_BIND_OP_MAP_NOEXEC = 1 << 1,

//
// @DRM_PANTHOR_VM_BIND_OP_MAP_UNCACHED: Map the memory uncached.
//
// Only valid with DRM_PANTHOR_VM_BIND_OP_TYPE_MAP.
//
    DRM_PANTHOR_VM_BIND_OP_MAP_UNCACHED = 1 << 2,

//
// @DRM_PANTHOR_VM_BIND_OP_MAP_SPARSE: Sparsely map a virtual memory range
//
// Only valid with DRM_PANTHOR_VM_BIND_OP_TYPE_MAP.
//
// When this flag is set, the whole vm_bind range is mapped over a dummy object in a cyclic
// fashion, and all GPU reads from addresses in the range return undefined values. This flag
// being set means drm_panthor_vm_bind_op::bo_offset and drm_panthor_vm_bind_op::bo_handle
// must both be set to 0. DRM_PANTHOR_VM_BIND_OP_MAP_NOEXEC must also be set.
//
    DRM_PANTHOR_VM_BIND_OP_MAP_SPARSE = 1 << 3,

//
// @DRM_PANTHOR_VM_BIND_OP_TYPE_MASK: Mask used to determine the type of operation.
//
    DRM_PANTHOR_VM_BIND_OP_TYPE_MASK = (int)(0xfu << 28),

// @DRM_PANTHOR_VM_BIND_OP_TYPE_MAP: Map operation.
    DRM_PANTHOR_VM_BIND_OP_TYPE_MAP = 0 << 28,

// @DRM_PANTHOR_VM_BIND_OP_TYPE_UNMAP: Unmap operation.
    DRM_PANTHOR_VM_BIND_OP_TYPE_UNMAP = 1 << 28,

//
// @DRM_PANTHOR_VM_BIND_OP_TYPE_SYNC_ONLY: No VM operation.
//
// Just serves as a synchronization point on a VM queue.
//
// Only valid if %DRM_PANTHOR_VM_BIND_ASYNC is set in drm_panthor_vm_bind::flags,
// and drm_panthor_vm_bind_op::syncs contains at least one element.
//
    DRM_PANTHOR_VM_BIND_OP_TYPE_SYNC_ONLY = 2 << 28,
}

//
// struct drm_panthor_vm_bind_op - VM bind operation
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panthor_vm_bind_op {
// @flags: Combination of drm_panthor_vm_bind_op_flags flags.
    pub flags: __u32,
//
// @bo_handle: Handle of the buffer object to map.
// MBZ for unmap or sync-only operations.
//
    pub bo_handle: __u32,
//
// @bo_offset: Buffer object offset.
// MBZ for unmap or sync-only operations.
//
    pub bo_offset: __u64,
//
// @va: Virtual address to map/unmap.
// MBZ for sync-only operations.
//
    pub va: __u64,
//
// @size: Size to map/unmap.
// MBZ for sync-only operations.
//
    pub size: __u64,
//
// @syncs: Array of struct drm_panthor_sync_op synchronization
// operations.
//
// This array must be empty if %DRM_PANTHOR_VM_BIND_ASYNC is not set on
// the drm_panthor_vm_bind object containing this VM bind operation.
//
// This array shall not be empty for sync-only operations.
//
    pub syncs: drm_panthor_obj_array,
}

//
// enum drm_panthor_vm_bind_flags - VM bind flags
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_panthor_vm_bind_flags {
//
// @DRM_PANTHOR_VM_BIND_ASYNC: VM bind operations are queued to the VM
// queue instead of being executed synchronously.
//
    DRM_PANTHOR_VM_BIND_ASYNC = 1 << 0,
}

//
// struct drm_panthor_vm_bind - Arguments passed to DRM_IOCTL_PANTHOR_VM_BIND
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panthor_vm_bind {
// @vm_id: VM targeted by the bind request.
    pub vm_id: __u32,
// @flags: Combination of drm_panthor_vm_bind_flags flags.
    pub flags: __u32,
// @ops: Array of struct drm_panthor_vm_bind_op bind operations.
    pub ops: drm_panthor_obj_array,
}

//
// enum drm_panthor_vm_state - VM states.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_panthor_vm_state {
//
// @DRM_PANTHOR_VM_STATE_USABLE: VM is usable.
//
// New VM operations will be accepted on this VM.
//
    DRM_PANTHOR_VM_STATE_USABLE,

//
// @DRM_PANTHOR_VM_STATE_UNUSABLE: VM is unusable.
//
// Something put the VM in an unusable state (like an asynchronous
// VM_BIND request failing for any reason).
//
// Once the VM is in this state, all new MAP operations will be
// rejected, and any GPU job targeting this VM will fail.
// UNMAP operations are still accepted.
//
// The only way to recover from an unusable VM is to create a new
// VM, and destroy the old one.
//
    DRM_PANTHOR_VM_STATE_UNUSABLE,
}

//
// struct drm_panthor_vm_get_state - Get VM state.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panthor_vm_get_state {
// @vm_id: VM targeted by the get_state request.
    pub vm_id: __u32,
//
// @state: state returned by the driver.
//
// Must be one of the enum drm_panthor_vm_state values.
//
    pub state: __u32,
}

//
// enum drm_panthor_bo_flags - Buffer object flags, passed at creation time.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_panthor_bo_flags {
// @DRM_PANTHOR_BO_NO_MMAP: The buffer object will never be CPU-mapped in userspace.
    DRM_PANTHOR_BO_NO_MMAP = (1 << 0),

//
// @DRM_PANTHOR_BO_WB_MMAP: Force "Write-Back Cacheable" CPU mapping.
//
// CPU map the buffer object in userspace by forcing the "Write-Back
// Cacheable" cacheability attribute. The mapping otherwise uses the
// "Non-Cacheable" attribute if the GPU is not IO coherent.
//
    DRM_PANTHOR_BO_WB_MMAP = (1 << 1),
}

//
// struct drm_panthor_bo_create - Arguments passed to DRM_IOCTL_PANTHOR_BO_CREATE.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panthor_bo_create {
//
// @size: Requested size for the object
//
// The (page-aligned) allocated size for the object will be returned.
//
    pub size: __u64,
//
// @flags: Flags. Must be a combination of drm_panthor_bo_flags flags.
//
    pub flags: __u32,
//
// @exclusive_vm_id: Exclusive VM this buffer object will be mapped to.
//
// If not zero, the field must refer to a valid VM ID, and implies that:
// - the buffer object will only ever be bound to that VM
// - cannot be exported as a PRIME fd
//
    pub exclusive_vm_id: __u32,
//
// @handle: Returned handle for the object.
//
// Object handles are nonzero.
//
    pub handle: __u32,
// @pad: MBZ.
    pub pad: __u32,
}

//
// struct drm_panthor_bo_mmap_offset - Arguments passed to DRM_IOCTL_PANTHOR_BO_MMAP_OFFSET.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panthor_bo_mmap_offset {
// @handle: Handle of the object we want an mmap offset for.
    pub handle: __u32,
// @pad: MBZ.
    pub pad: __u32,
// @offset: The fake offset to use for subsequent mmap calls.
    pub offset: __u64,
}

//
// struct drm_panthor_queue_create - Queue creation arguments.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panthor_queue_create {
//
// @priority: Defines the priority of queues inside a group. Goes from 0 to 15,
// 15 being the highest priority.
//
    pub priority: __u8,
// @pad: Padding fields, MBZ.
    pub pad: [__u8; 3],
// @ringbuf_size: Size of the ring buffer to allocate to this queue.
    pub ringbuf_size: __u32,
}

//
// enum drm_panthor_group_priority - Scheduling group priority
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_panthor_group_priority {
// @PANTHOR_GROUP_PRIORITY_LOW: Low priority group.
    PANTHOR_GROUP_PRIORITY_LOW = 0,

// @PANTHOR_GROUP_PRIORITY_MEDIUM: Medium priority group.
    PANTHOR_GROUP_PRIORITY_MEDIUM,

//
// @PANTHOR_GROUP_PRIORITY_HIGH: High priority group.
//
// Requires CAP_SYS_NICE or DRM_MASTER.
//
    PANTHOR_GROUP_PRIORITY_HIGH,

//
// @PANTHOR_GROUP_PRIORITY_REALTIME: Realtime priority group.
//
// Requires CAP_SYS_NICE or DRM_MASTER.
//
    PANTHOR_GROUP_PRIORITY_REALTIME,
}

//
// struct drm_panthor_group_create - Arguments passed to DRM_IOCTL_PANTHOR_GROUP_CREATE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panthor_group_create {
// @queues: Array of drm_panthor_queue_create elements.
    pub queues: drm_panthor_obj_array,
//
// @max_compute_cores: Maximum number of cores that can be used by compute
// jobs across CS queues bound to this group.
//
// Must be less or equal to the number of bits set in @compute_core_mask.
//
    pub max_compute_cores: __u8,
//
// @max_fragment_cores: Maximum number of cores that can be used by fragment
// jobs across CS queues bound to this group.
//
// Must be less or equal to the number of bits set in @fragment_core_mask.
//
    pub max_fragment_cores: __u8,
//
// @max_tiler_cores: Maximum number of tilers that can be used by tiler jobs
// across CS queues bound to this group.
//
// Must be less or equal to the number of bits set in @tiler_core_mask.
//
    pub max_tiler_cores: __u8,
// @priority: Group priority (see enum drm_panthor_group_priority).
    pub priority: __u8,
// @pad: Padding field, MBZ.
    pub pad: __u32,
//
// @compute_core_mask: Mask encoding cores that can be used for compute jobs.
//
// This field must have at least @max_compute_cores bits set.
//
// The bits set here should also be set in drm_panthor_gpu_info::shader_present.
//
    pub compute_core_mask: __u64,
//
// @fragment_core_mask: Mask encoding cores that can be used for fragment jobs.
//
// This field must have at least @max_fragment_cores bits set.
//
// The bits set here should also be set in drm_panthor_gpu_info::shader_present.
//
    pub fragment_core_mask: __u64,
//
// @tiler_core_mask: Mask encoding cores that can be used for tiler jobs.
//
// This field must have at least @max_tiler_cores bits set.
//
// The bits set here should also be set in drm_panthor_gpu_info::tiler_present.
//
    pub tiler_core_mask: __u64,
//
// @vm_id: VM ID to bind this group to.
//
// All submission to queues bound to this group will use this VM.
//
    pub vm_id: __u32,
//
// @group_handle: Returned group handle. Passed back when submitting jobs or
// destroying a group.
//
    pub group_handle: __u32,
}

//
// struct drm_panthor_group_destroy - Arguments passed to DRM_IOCTL_PANTHOR_GROUP_DESTROY
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panthor_group_destroy {
// @group_handle: Group to destroy
    pub group_handle: __u32,
// @pad: Padding field, MBZ.
    pub pad: __u32,
}

//
// struct drm_panthor_queue_submit - Job submission arguments.
//
// This is describing the userspace command stream to call from the kernel
// command stream ring-buffer. Queue submission is always part of a group
// submission, taking one or more jobs to submit to the underlying queues.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panthor_queue_submit {
// @queue_index: Index of the queue inside a group.
    pub queue_index: __u32,
//
// @stream_size: Size of the command stream to execute.
//
// Must be 64-bit/8-byte aligned (the size of a CS instruction)
//
// Can be zero if stream_addr is zero too.
//
// When the stream size is zero, the queue submit serves as a
// synchronization point.
//
    pub stream_size: __u32,
//
// @stream_addr: GPU address of the command stream to execute.
//
// Must be aligned on 64-byte.
//
// Can be zero is stream_size is zero too.
//
    pub stream_addr: __u64,
//
// @latest_flush: FLUSH_ID read at the time the stream was built.
//
// This allows cache flush elimination for the automatic
// flush+invalidate(all) done at submission time, which is needed to
// ensure the GPU doesn't get garbage when reading the indirect command
// stream buffers. If you want the cache flush to happen
// unconditionally, pass a zero here.
//
// Ignored when stream_size is zero.
//
    pub latest_flush: __u32,
// @pad: MBZ.
    pub pad: __u32,
// @syncs: Array of struct drm_panthor_sync_op sync operations.
    pub syncs: drm_panthor_obj_array,
}

//
// struct drm_panthor_group_submit - Arguments passed to DRM_IOCTL_PANTHOR_GROUP_SUBMIT
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panthor_group_submit {
// @group_handle: Handle of the group to queue jobs to.
    pub group_handle: __u32,
// @pad: MBZ.
    pub pad: __u32,
// @queue_submits: Array of drm_panthor_queue_submit objects.
    pub queue_submits: drm_panthor_obj_array,
}

//
// enum drm_panthor_group_state_flags - Group state flags
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_panthor_group_state_flags {
//
// @DRM_PANTHOR_GROUP_STATE_TIMEDOUT: Group had unfinished jobs.
//
// When a group ends up with this flag set, no jobs can be submitted to its queues.
//
    DRM_PANTHOR_GROUP_STATE_TIMEDOUT = 1 << 0,

//
// @DRM_PANTHOR_GROUP_STATE_FATAL_FAULT: Group had fatal faults.
//
// When a group ends up with this flag set, no jobs can be submitted to its queues.
//
    DRM_PANTHOR_GROUP_STATE_FATAL_FAULT = 1 << 1,

//
// @DRM_PANTHOR_GROUP_STATE_INNOCENT: Group was killed during a reset caused by other
// groups.
//
// This flag can only be set if DRM_PANTHOR_GROUP_STATE_TIMEDOUT is set and
// DRM_PANTHOR_GROUP_STATE_FATAL_FAULT is not.
//
    DRM_PANTHOR_GROUP_STATE_INNOCENT = 1 << 2,
}

//
// struct drm_panthor_group_get_state - Arguments passed to DRM_IOCTL_PANTHOR_GROUP_GET_STATE
//
// Used to query the state of a group and decide whether a new group should be created to
// replace it.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panthor_group_get_state {
// @group_handle: Handle of the group to query state on
    pub group_handle: __u32,
//
// @state: Combination of DRM_PANTHOR_GROUP_STATE_* flags encoding the
// group state.
//
    pub state: __u32,
// @fatal_queues: Bitmask of queues that faced fatal faults.
    pub fatal_queues: __u32,
// @pad: MBZ
    pub pad: __u32,
}

//
// struct drm_panthor_tiler_heap_create - Arguments passed to DRM_IOCTL_PANTHOR_TILER_HEAP_CREATE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panthor_tiler_heap_create {
// @vm_id: VM ID the tiler heap should be mapped to
    pub vm_id: __u32,
// @initial_chunk_count: Initial number of chunks to allocate. Must be at least one.
    pub initial_chunk_count: __u32,
//
// @chunk_size: Chunk size.
//
// Must be page-aligned and lie in the [128k:8M] range.
//
    pub chunk_size: __u32,
//
// @max_chunks: Maximum number of chunks that can be allocated.
//
// Must be at least @initial_chunk_count.
//
    pub max_chunks: __u32,
//
// @target_in_flight: Maximum number of in-flight render passes.
//
// If the heap has more than tiler jobs in-flight, the FW will wait for render
// passes to finish before queuing new tiler jobs.
//
    pub target_in_flight: __u32,
// @handle: Returned heap handle. Passed back to DESTROY_TILER_HEAP.
    pub handle: __u32,
// @tiler_heap_ctx_gpu_va: Returned heap GPU virtual address returned
    pub tiler_heap_ctx_gpu_va: __u64,
//
// @first_heap_chunk_gpu_va: First heap chunk.
//
// The tiler heap is formed of heap chunks forming a single-link list. This
// is the first element in the list.
//
    pub first_heap_chunk_gpu_va: __u64,
}

//
// struct drm_panthor_tiler_heap_destroy - Arguments passed to DRM_IOCTL_PANTHOR_TILER_HEAP_DESTROY
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panthor_tiler_heap_destroy {
//
// @handle: Handle of the tiler heap to destroy.
//
// Must be a valid heap handle returned by DRM_IOCTL_PANTHOR_TILER_HEAP_CREATE.
//
    pub handle: __u32,
// @pad: Padding field, MBZ.
    pub pad: __u32,
}

//
// struct drm_panthor_bo_set_label - Arguments passed to DRM_IOCTL_PANTHOR_BO_SET_LABEL
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panthor_bo_set_label {
// @handle: Handle of the buffer object to label.
    pub handle: __u32,
// @pad: MBZ.
    pub pad: __u32,
//
// @label: User pointer to a NUL-terminated string
//
// Length cannot be greater than 4096
//
    pub label: __u64,
}

//
// struct drm_panthor_set_user_mmio_offset - Arguments passed to
// DRM_IOCTL_PANTHOR_SET_USER_MMIO_OFFSET
//
// This ioctl is only really useful if you want to support userspace
// CPU emulation environments where the size of an unsigned long differs
// between the host and the guest architectures.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panthor_set_user_mmio_offset {
//
// @offset: User MMIO offset to use.
//
// Must be either DRM_PANTHOR_USER_MMIO_OFFSET_32BIT or
// DRM_PANTHOR_USER_MMIO_OFFSET_64BIT.
//
// Use DRM_PANTHOR_USER_MMIO_OFFSET (which selects OFFSET_32BIT or
// OFFSET_64BIT based on the size of an unsigned long) unless you
// have a very good reason to overrule this decision.
//
    pub offset: __u64,
}

//
// enum drm_panthor_bo_sync_op_type - BO sync type
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_panthor_bo_sync_op_type {
// @DRM_PANTHOR_BO_SYNC_CPU_CACHE_FLUSH: Flush CPU caches.
    DRM_PANTHOR_BO_SYNC_CPU_CACHE_FLUSH = 0,

// @DRM_PANTHOR_BO_SYNC_CPU_CACHE_FLUSH_AND_INVALIDATE: Flush and invalidate CPU caches.
    DRM_PANTHOR_BO_SYNC_CPU_CACHE_FLUSH_AND_INVALIDATE = 1,
}

//
// struct drm_panthor_bo_sync_op - BO map sync op
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panthor_bo_sync_op {
// @handle: Handle of the buffer object to sync.
    pub handle: __u32,
// @type: Type of operation.
    pub type: __u32,
//
// @offset: Offset into the BO at which the sync range starts.
//
// This will be rounded down to the nearest cache line as needed.
//
    pub offset: __u64,
//
// @size: Size of the range to sync
//
// @size + @offset will be rounded up to the nearest cache line as
// needed.
//
    pub size: __u64,
}

//
// struct drm_panthor_bo_sync - BO map sync request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panthor_bo_sync {
//
// @ops: Array of struct drm_panthor_bo_sync_op sync operations.
//
    pub ops: drm_panthor_obj_array,
}

//
// enum drm_panthor_bo_extra_flags - Set of flags returned on a BO_QUERY_INFO request
//
// Those are flags reflecting BO properties that are not directly coming from the flags
// passed are creation time, or information on BOs that were imported from other drivers.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_panthor_bo_extra_flags {
//
// @DRM_PANTHOR_BO_IS_IMPORTED: BO has been imported from an external driver.
//
// Note that imported dma-buf handles are not flagged as imported if they
// where exported by panthor. Only buffers that are coming from other drivers
// (dma heaps, other GPUs, display controllers, V4L, ...).
//
// It's also important to note that all imported BOs are mapped cached and can't
// be considered IO-coherent even if the GPU is. This means they require explicit
// syncs that must go through the DRM_PANTHOR_BO_SYNC ioctl (userland cache
// maintenance is not allowed in that case, because extra operations might be
// needed to make changes visible to the CPU/device, like buffer migration when the
// exporter is a GPU with its own VRAM).
//
    DRM_PANTHOR_BO_IS_IMPORTED = (1 << 0),
}

//
// struct drm_panthor_bo_query_info - Query BO info
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panthor_bo_query_info {
// @handle: Handle of the buffer object to query flags on.
    pub handle: __u32,
//
// @extra_flags: Combination of enum drm_panthor_bo_extra_flags flags.
//
    pub extra_flags: __u32,
//
// @create_flags: Flags passed at creation time.
//
// Combination of enum drm_panthor_bo_flags flags.
// Will be zero if the buffer comes from a different driver.
//
    pub create_flags: __u32,
// @pad: Will be zero on return.
    pub pad: __u32,
}

//
// DRM_IOCTL_PANTHOR() - Build a Panthor IOCTL number
// @__access: Access type. Must be R, W or RW.
// @__id: One of the DRM_PANTHOR_xxx id.
// @__type: Suffix of the type being passed to the IOCTL.
//
// Don't use this macro directly, use the DRM_IOCTL_PANTHOR_xxx
// values instead.
//
// Return: An IOCTL number to be passed to ioctl() from userspace.
//

