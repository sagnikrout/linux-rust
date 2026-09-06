//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_vm_types.h
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
// Copyright © 2022 Intel Corporation
//

// Macro flag: #define TEST_VM_OPS_ERROR

pub const FORCE_OP_ERROR_LOCK: c_int = 0;
pub const FORCE_OP_ERROR_PREPARE: c_int = 1;
pub const FORCE_OP_ERROR_RUN: c_int = 2;
pub const FORCE_OP_ERROR_COUNT: c_int = 3;

//
// struct xe_vma_mem_attr - memory attributes associated with vma
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_vma_mem_attr {
// @preferred_loc: preferred memory_location
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_vma_preferred_loc {
// @preferred_loc.migration_policy: Pages migration policy
    pub migration_policy: u32,
//
// @preferred_loc.devmem_fd: used for determining pagemap_fd
// requested by user DRM_XE_PREFERRED_LOC_DEFAULT_SYSTEM and
// DRM_XE_PREFERRED_LOC_DEFAULT_DEVICE mean system memory or
// closest device memory respectively.
//
    pub devmem_fd: u32,
//
// @preferred_loc.dpagemap: Reference-counted pointer to the drm_pagemap preferred
// for migration on a SVM page-fault. The pointer is protected by the
// vm lock, and is %NULL if @devmem_fd should be consulted for special
// values.
//
    pub dpagemap: *mut drm_pagemap,
    pub preferred_loc: },
//
// @atomic_access: The atomic access type for the vma
// See %DRM_XE_VMA_ATOMIC_UNDEFINED, %DRM_XE_VMA_ATOMIC_DEVICE,
// %DRM_XE_VMA_ATOMIC_GLOBAL, and %DRM_XE_VMA_ATOMIC_CPU for possible
// values. These are defined in uapi/drm/xe_drm.h.
//
    pub atomic_access: u32,
//
// @default_pat_index: The pat index for VMA set during first bind by user.
//
    pub default_pat_index: u16,
//
// @pat_index: The pat index to use when encoding the PTEs for this vma.
// same as default_pat_index unless overwritten by madvise.
//
    pub pat_index: u16,
//
// @purgeable_state: Purgeable hint for this VMA mapping
//
// Per-VMA purgeable state from madvise. Valid states are WILLNEED (0)
// or DONTNEED (1). Shared BOs require all VMAs to be DONTNEED before
// the BO can be purged. PURGED state exists only at BO level.
//
// Protected by BO dma-resv lock. Set via DRM_IOCTL_XE_MADVISE.
//
    pub purgeable_state: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_vma {
// @gpuva: Base GPUVA object
    pub gpuva: drm_gpuva,
//
// @combined_links: links into lists which are mutually exclusive.
// Locking: vm lock in write mode OR vm lock in read mode and the vm's
// resv.
//
// @rebind: link into VM if this VMA needs rebinding.
    pub rebind: list_head,
// @destroy: link to contested list when VM is being closed.
    pub destroy: list_head,
    pub combined_links: },
// @destroy_cb: callback to destroy VMA when unbind job is done
    pub destroy_cb: dma_fence_cb,
// @destroy_work: worker to destroy this BO
    pub destroy_work: work_struct,
}

//
// @tile_invalidated: Tile mask of binding are invalidated for this VMA.
// protected by BO's resv and for userptrs, vm->svm.gpusvm.notifier_lock in
// write mode for writing or vm->svm.gpusvm.notifier_lock in read mode and
// the vm->resv. For stable reading, BO's resv or userptr
// vm->svm.gpusvm.notifier_lock in read mode is required. Can be
// opportunistically read with READ_ONCE outside of locks.
//
// @tile_mask: Tile mask of where to create binding for this VMA
//
// @tile_present: Tile mask of binding are present for this VMA.
// protected by vm->lock, vm->resv and for userptrs,
// vm->svm.gpusvm.notifier_lock for writing. Needs either for reading,
// but if reading is done under the vm->lock only, it needs to be held
// in write mode.
//
// @tile_staged: bind is staged for this VMA
//
// @skip_invalidation: Used in madvise to avoid invalidation
// if mem attributes doesn't change
//
// @ufence: The user fence that was provided with MAP.
// Needs to be signalled before UNMAP can be processed.
//
// @attr: The attributes of vma which determines the migration policy
// and encoding of the PTEs for this vma.
//
// struct xe_userptr_vma - A userptr vma subclass
// @vma: The vma.
// @userptr: Additional userptr information.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_userptr_vma {
    pub vma: xe_vma,
    pub userptr: xe_userptr,
}

//
// struct xe_vm_fault_entry - Elements of vm->faults.list
// @list: link into @xe_vm.faults.list
// @address: address of the fault
// @address_precision: precision of faulted address
// @access_type: type of address access that resulted in fault
// @fault_type: type of fault reported
// @fault_level: fault level of the fault
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_vm_fault_entry {
    pub list: list_head,
    pub address: u64,
    pub address_precision: u32,
    pub access_type: u8,
    pub fault_type: u8,
    pub fault_level: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_vm {
// @gpuvm: base GPUVM used to track VMAs
    pub gpuvm: drm_gpuvm,
// @svm: Shared virtual memory state
// @svm.gpusvm: base GPUSVM used to track fault allocations
    pub gpusvm: drm_gpusvm,
//
// @svm.garbage_collector: Garbage collector which is used unmap
// SVM range's GPU bindings and destroy the ranges.
//
// @svm.garbage_collector.lock: Protect's range list
    pub lock: spinlock_t,
//
// @svm.garbage_collector.range_list: List of SVM ranges
// in the garbage collector.
//
    pub range_list: list_head,
//
// @svm.garbage_collector.work: Worker which the
// garbage collector runs on.
//
    pub work: work_struct,
    pub garbage_collector: },
    pub pagemaps: [*mut xe_pagemap; XE_MAX_TILES_PER_DEVICE],
// @svm.peer: Used for pagemap connectivity computations.
    pub peer: drm_pagemap_peer,
    pub svm: },
    pub xe: *mut xe_device,
// exec queue used for (un)binding vma's
    pub q: [*mut xe_exec_queue; XE_MAX_TILES_PER_DEVICE],
// @lru_bulk_move: Bulk LRU move list for this VM's BOs
    pub lru_bulk_move: ttm_lru_bulk_move,
    pub size: u64,
    pub pt_root: [*mut xe_pt; XE_MAX_TILES_PER_DEVICE],
    pub scratch_pt: [*mut xe_pt; XE_MAX_TILES_PER_DEVICE][XE_VM_MAX_LEVEL],
//
// @flags: flags for this VM, statically setup a creation time aside
// from XE_VM_FLAG_BANNED which requires vm->lock to set / read safely
//

    pub flags: c_ulong,
//
// @lock: outer most lock, protects objects of anything attached to this
// VM
//
    pub lock: rw_semaphore,
//
// @snap_mutex: Mutex used to guard insertions and removals from gpuva,
// so we can take a snapshot safely from devcoredump.
//
    pub snap_mutex: mutex,
//
// @rebind_list: list of VMAs that need rebinding. Protected by the
// vm->lock in write mode, OR (the vm->lock in read mode and the
// vm resv).
//
    pub rebind_list: list_head,
//
// @destroy_work: worker to destroy VM, needed as a dma_fence signaling
// from an irq context can be last put and the destroy needs to be able
// to sleep.
//
    pub destroy_work: work_struct,
//
// @rftree: range fence tree to track updates to page table structure.
// Used to implement conflict tracking between independent bind engines.
//
    pub rftree: [xe_range_fence_tree; XE_MAX_TILES_PER_DEVICE],
    pub pt_ops: *const xe_pt_ops,
// @userptr: user pointer state
    pub userptr: xe_userptr_vm,
// @preempt: preempt state
//
// @min_run_period_ms: The minimum run period before preempting
// an engine again
//
    pub min_run_period_ms: c_uint,
// @exec_queues: list of exec queues attached to this VM
    pub exec_queues: list_head,
// @num_exec_queues: number exec queues attached to this VM
    pub num_exec_queues: c_int,
//
// @rebind_deactivated: Whether rebind has been temporarily deactivated
// due to no work available. Protected by the vm resv.
//
    pub rebind_deactivated: bool,
//
// @rebind_work: worker to rebind invalidated userptrs / evicted
// BOs
//
    pub rebind_work: work_struct,
//
// @preempt.pm_activate_link: Link to list of rebind workers to be
// kicked on resume.
//
    pub pm_activate_link: list_head,
    pub preempt: },
// @exec_queues: Manages list of exec queues attached to this VM, protected by lock.
//
// @exec_queues.list: list of exec queues attached to this VM,
// per GT
//
    pub XE_MAX_GT_PER_TILE]: *mut *mut list_head list[XE_MAX_TILES_PER_DEVICE,
//
// @exec_queues.count: count of exec queues attached to this VM,
// per GT
//
    pub XE_MAX_GT_PER_TILE]: *mut *mut int count[XE_MAX_TILES_PER_DEVICE,
// @exec_queues.lock: lock to protect exec_queues list
    pub lock: rw_semaphore,
    pub exec_queues: },
// @um: unified memory state
// @asid: address space ID, unique to each VM
    pub asid: u32,
//
// @last_fault_vma: Last fault VMA, used for fast lookup when we
// get a flood of faults to the same VMA
//
    pub last_fault_vma: *mut xe_vma,
    pub usm: },
// @error_capture: allow to track errors
// @capture_once: capture only one error per VM
    pub capture_once: bool,
    pub error_capture: },
// @faults: List of all faults associated with this VM
// @faults.lock: lock protecting @faults.list
    pub lock: spinlock_t,
// @faults.list: list of xe_vm_fault_entry entries
    pub list: list_head,
// @faults.len: length of @faults.list
    pub len: c_uint,
    pub faults: },
//
// @validation: Validation data only valid with the vm resv held.
// Note: This is really task state of the task holding the vm resv,
// and moving forward we should
// come up with a better way of passing this down the call-
// chain.
//
// @validation.validating: The task that is currently making bos resident.
// for this vm.
// Protected by the VM's resv for writing. Opportunistic reading can be done
// using READ_ONCE. Note: This is a workaround for the
// TTM eviction_valuable() callback not being passed a struct
// ttm_operation_context(). Future work might want to address this.
//
    pub validating: *mut task_struct,
//
// @validation.exec The drm_exec context used when locking the vm resv.
// Protected by the vm's resv.
//
    pub _exec: *mut drm_exec,
    pub validation: },
//
// @tlb_flush_seqno: Required TLB flush seqno for the next exec.
// protected by the vm resv.
//
    pub tlb_flush_seqno: u64,
// @batch_invalidate_tlb: Always invalidate TLB before batch start
    pub batch_invalidate_tlb: bool,
// @xef: Xe file handle for tracking this VM's drm client
    pub xef: *mut xe_file,
}

// struct xe_vma_op_map - VMA map operation
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_vma_op_map {
// @vma: VMA to map
    pub vma: *mut xe_vma,
// @vma_flags: VMA flags for this operation
    pub vma_flags: c_uint,
// @immediate: Immediate bind
    pub immediate: bool,
// @invalidate_on_bind: Invalidate on bind
    pub invalidate_on_bind: bool,
// @request_decompress: schedule decompression for GPU map
    pub request_decompress: bool,
// @pat_index: The pat index to use for this operation.
    pub pat_index: u16,
}

// struct xe_vma_op_remap - VMA remap operation
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_vma_op_remap {
// @prev: VMA preceding part of a split mapping
    pub prev: *mut xe_vma,
// @next: VMA subsequent part of a split mapping
    pub next: *mut xe_vma,
// @start: start of the VMA unmap
    pub start: u64,
// @range: range of the VMA unmap
    pub range: u64,
// @old_start: Original start of the VMA we unmap
    pub old_start: u64,
// @old_range: Original range of the VMA we unmap
    pub old_range: u64,
// @skip_prev: skip prev rebind
    pub skip_prev: bool,
// @skip_next: skip next rebind
    pub skip_next: bool,
// @unmap_done: unmap operation in done
    pub unmap_done: bool,
}

// struct xe_vma_op_prefetch - VMA prefetch operation
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_vma_op_prefetch {
// @region: memory region to prefetch to
    pub region: u32,
}

// struct xe_vma_op_map_range - VMA map range operation
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_vma_op_map_range {
// @vma: VMA to map (system allocator VMA)
    pub vma: *mut xe_vma,
// @range: SVM range to map
    pub range: *mut xe_svm_range,
}

// struct xe_vma_op_unmap_range - VMA unmap range operation
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_vma_op_unmap_range {
// @range: SVM range to unmap
    pub range: *mut xe_svm_range,
}

// struct xe_vma_op_prefetch_range - VMA prefetch range operation
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_vma_op_prefetch_range {
// @range: xarray for SVM ranges data
    pub range: xarray,
// @ranges_count: number of svm ranges to map
    pub ranges_count: u32,
//
// @dpagemap: Pointer to the dpagemap structure containing memory to prefetch.
// NULL if prefetch requested region is smem
//
    pub dpagemap: *mut drm_pagemap,
}

// enum xe_vma_op_flags - flags for VMA operation
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xe_vma_op_flags {
// @XE_VMA_OP_COMMITTED: VMA operation committed
    XE_VMA_OP_COMMITTED		= BIT(0),
// @XE_VMA_OP_PREV_COMMITTED: Previous VMA operation committed
    XE_VMA_OP_PREV_COMMITTED	= BIT(1),
// @XE_VMA_OP_NEXT_COMMITTED: Next VMA operation committed
    XE_VMA_OP_NEXT_COMMITTED	= BIT(2),
}

// enum xe_vma_subop - VMA sub-operation
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xe_vma_subop {
// @XE_VMA_SUBOP_MAP_RANGE: Map range
    XE_VMA_SUBOP_MAP_RANGE,
// @XE_VMA_SUBOP_UNMAP_RANGE: Unmap range
    XE_VMA_SUBOP_UNMAP_RANGE,
}

// struct xe_vma_op - VMA operation
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_vma_op {
// @base: GPUVA base operation
    pub base: drm_gpuva_op,
// @link: async operation link
    pub link: list_head,
// @flags: operation flags
    pub flags: xe_vma_op_flags,
// @subop: user defined sub-operation
    pub subop: xe_vma_subop,
// @tile_mask: Tile mask for operation
    pub tile_mask: u8,
// @map: VMA map operation specific data
    pub map: xe_vma_op_map,
// @remap: VMA remap operation specific data
    pub remap: xe_vma_op_remap,
// @prefetch: VMA prefetch operation specific data
    pub prefetch: xe_vma_op_prefetch,
// @map_range: VMA map range operation specific data
    pub map_range: xe_vma_op_map_range,
// @unmap_range: VMA unmap range operation specific data
    pub unmap_range: xe_vma_op_unmap_range,
// @prefetch_range: VMA prefetch range operation specific data
    pub prefetch_range: xe_vma_op_prefetch_range,
}

// struct xe_vma_ops - VMA operations
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_vma_ops {
// @list: list of VMA operations
    pub list: list_head,
// @vm: VM
    pub vm: *mut xe_vm,
// @q: exec queue for VMA operations
    pub q: *mut xe_exec_queue,
// @syncs: syncs these operation
    pub syncs: *mut xe_sync_entry,
// @num_syncs: number of syncs
    pub num_syncs: u32,
// @pt_update_ops: page table update operations
    pub pt_update_ops: [xe_vm_pgtable_update_ops; XE_MAX_TILES_PER_DEVICE],
// @flag: signify the properties within xe_vma_ops

    pub flags: u32,

// @inject_error: inject error to test error handling
    pub inject_error: bool,

}
