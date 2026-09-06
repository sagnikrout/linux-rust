//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/msm/msm_gem.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2013 Red Hat
// Author: Rob Clark <robdclark@gmail.com>
//

// Make all GEM related WARN_ON()s ratelimited.. when things go wrong they
// tend to go wrong 1000s of times in a short timespan.
//

// Additional internal-use only BO flags:
pub const MSM_BO_STOLEN: c_uint = 0x10000000    /* try to use stolen/splash memory */;
pub const MSM_BO_MAP_PRIV: c_uint = 0x20000000    /* use IOMMU_PRIV when mapping */;
//
// struct msm_gem_vm_log_entry - An entry in the VM log
//
// For userspace managed VMs, a log of recent VM updates is tracked and
// captured in GPU devcore dumps, to aid debugging issues caused by (for
// example) incorrectly synchronized VM updates
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_gem_vm_log_entry {
    pub op: *const c_char,
    pub iova: u64,
    pub range: u64,
    pub queue_id: c_int,
}

//
// struct msm_gem_vm - VM object
//
// A VM object representing a GPU (or display or GMU or ...) virtual address
// space.
//
// In the case of GPU, if per-process address spaces are supported, the address
// space is split into two VMs, which map to TTBR0 and TTBR1 in the SMMU.  TTBR0
// is used for userspace objects, and is unique per msm_context/drm_file, while
// TTBR1 is the same for all processes.  (The kernel controlled ringbuffer and
// a few other kernel controlled buffers live in TTBR1.)
//
// The GPU TTBR0 vm can be managed by userspace or by the kernel, depending on
// whether userspace supports VM_BIND.  All other vm's are managed by the kernel.
// (Managed by kernel means the kernel is responsible for VA allocation.)
//
// Note that because VM_BIND allows a given BO to be mapped multiple times in
// a VM, and therefore have multiple VMA's in a VM, there is an extra object
// provided by drm_gpuvm infrastructure.. the drm_gpuvm_bo, which is not
// embedded in any larger driver structure.  The GEM object holds a list of
// drm_gpuvm_bo, which in turn holds a list of msm_gem_vma.  A linked vma
// holds a reference to the vm_bo, and drops it when the vma is unlinked.
// So we just need to call drm_gpuvm_bo_obtain_locked() to return a ref to an
// existing vm_bo, or create a new one.  Once the vma is linked, the ref
// to the vm_bo can be dropped (since the vma is holding one).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_gem_vm {
// @base: Inherit from drm_gpuvm.
    pub base: drm_gpuvm,
//
// @sched: Scheduler used for asynchronous VM_BIND request.
//
// Unused for kernel managed VMs (where all operations are synchronous).
//
    pub sched: drm_gpu_scheduler,
//
// @prealloc_throttle: Used to throttle VM_BIND ops if too much pre-
// allocated memory is in flight.
//
// Because we have to pre-allocate pgtable pages for the worst case
// (ie. new mappings do not share any PTEs with existing mappings)
// we could end up consuming a lot of resources transiently.  The
// prealloc_throttle puts an upper bound on that.
//
// @wait: Notified when preallocated resources are released
    pub wait: wait_queue_head_t,
//
// @in_flight: The # of preallocated pgtable pages in-flight
// for queued VM_BIND jobs.
//
    pub in_flight: core::sync::atomic::AtomicI32,
    pub prealloc_throttle: },
//
// @mm: Memory management for kernel managed VA allocations
//
// Only used for kernel managed VMs, unused for user managed VMs.
//
// Protected by vm lock.  See msm_gem_lock_vm_and_obj(), for ex.
//
    pub mm: drm_mm,
// @mmu: The mmu object which manages the pgtables
    pub mmu: *mut msm_mmu,
// @mmu_lock: Protects access to the mmu
    pub mmu_lock: mutex,
//
// @pid: For address spaces associated with a specific process, this
// will be non-NULL:
//
    pub pid: *mut pid,
// @last_fence: Fence for last pending work scheduled on the VM
    pub last_fence: *mut dma_fence,
// @log: A log of recent VM updates
    pub log: *mut msm_gem_vm_log_entry,
// @log_shift: length of @log is (1 << @log_shift)
    pub log_shift: u32,
// @log_idx: index of next @log entry to write
    pub log_idx: u32,
// @faults: the number of GPU hangs associated with this address space
    pub faults: c_int,
// @managed: is this a kernel managed VM?
    pub managed: bool,
//
// @unusable: True if the VM has turned unusable because something
// bad happened during an asynchronous request.
//
// We don't try to recover from such failures, because this implies
// informing userspace about the specific operation that failed, and
// hoping the userspace driver can replay things from there. This all
// sounds very complicated for little gain.
//
// Instead, we should just flag the VM as unusable, and fail any
// further request targeting this VM.
//
// As an analogy, this would be mapped to a VK_ERROR_DEVICE_LOST
// situation, where the logical device needs to be re-created.
//
    pub unusable: bool,
}

extern "C" {
    pub fn msm_gem_vm_close(gpuvm: *mut drm_gpuvm);
}
extern "C" {
    pub fn msm_gem_vm_unusable(gpuvm: *mut drm_gpuvm);
}

//
// struct msm_gem_vma - a VMA mapping
//
// Represents a combination of a GEM object plus a VM.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_gem_vma {
// @base: inherit from drm_gpuva
    pub base: drm_gpuva,
//
// @node: mm node for VA allocation
//
// Only used by kernel managed VMs
//
    pub node: drm_mm_node,
// @mapped: Is this VMA mapped?
    pub mapped: bool,
}

extern "C" {
    pub fn msm_gem_vma_unmap(vma: *mut drm_gpuva, reason: *const c_char);
}
extern "C" {
    pub fn msm_gem_vma_map(vma: *mut drm_gpuva, prot: c_int, sgt: *mut sg_table) -> c_int;
}
extern "C" {
    pub fn msm_gem_vma_close(vma: *mut drm_gpuva);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_gem_object {
    pub base: drm_gem_object,
    pub flags: u32,
//
// madv: are the backing pages purgeable?
//
// Protected by obj lock and LRU lock
//
    pub madv: u8,
//
// count of active vmap'ing
//
    pub vmap_count: u8,
//
// Node in list of all objects (mainly for debugfs, protected by
// priv->obj_lock
//
    pub node: list_head,
    pub pages: *mut page,
    pub sgt: *mut sg_table,
    pub vaddr: *mut c_void,
    pub /: *mut *mut char name[32]; / Identifier to print for the debugfs files,
// userspace metadata backchannel
    pub metadata: *mut c_void,
    pub metadata_size: u32,
//
// pin_count: Number of times the pages are pinned
//
// Protected by LRU lock.
//
    pub pin_count: c_int,
//
// @vma_ref: Reference count of VMA users.
//
// With the vm_bo/vma holding a reference to the GEM object, we'd
// otherwise have to actively tear down a VMA when, for example,
// a buffer is unpinned for scanout, vs. the pre-drm_gpuvm approach
// where a VMA did not hold a reference to the BO, but instead was
// implicitly torn down when the BO was freed.
//
// To regain the lazy VMA teardown, we use the @vma_ref.  It is
// incremented for any of the following:
//
// 1) the BO is exported as a dma_buf
// 2) the BO has open userspace handle
//
// All of those conditions will hold an reference to the BO,
// preventing it from being freed.  So lazily keeping around the
// VMA will not prevent the BO from being freed.  (Or rather, the
// reference loop is harmless in this case.)
//
// When the @vma_ref drops to zero, then kms->vm VMA will be
// torn down.
//
    pub vma_ref: core::sync::atomic::AtomicI32,
}

extern "C" {
    pub fn msm_gem_vma_get(obj: *mut drm_gem_object);
}
extern "C" {
    pub fn msm_gem_vma_put(obj: *mut drm_gem_object);
}
extern "C" {
    pub fn msm_gem_prot(obj: *mut drm_gem_object) -> c_int;
}
extern "C" {
    pub fn msm_gem_pin_vma_locked(obj: *mut drm_gem_object, vma: *mut drm_gpuva) -> c_int;
}
extern "C" {
    pub fn msm_gem_unpin_locked(obj: *mut drm_gem_object);
}
extern "C" {
    pub fn msm_gem_unpin_active(obj: *mut drm_gem_object);
}
extern "C" {
    pub fn msm_gem_unpin_iova(obj: *mut drm_gem_object, vm: *mut drm_gpuvm);
}
extern "C" {
    pub fn msm_gem_pin_obj_locked(obj: *mut drm_gem_object);
}
extern "C" {
    pub fn msm_gem_unpin_pages_locked(obj: *mut drm_gem_object);
}
extern "C" {
    pub fn msm_gem_put_vaddr_locked(obj: *mut drm_gem_object);
}
extern "C" {
    pub fn msm_gem_put_vaddr(obj: *mut drm_gem_object);
}
extern "C" {
    pub fn msm_gem_madvise(obj: *mut drm_gem_object, madv: unsigned) -> c_int;
}
extern "C" {
    pub fn msm_gem_active(obj: *mut drm_gem_object) -> bool;
}
extern "C" {
    pub fn msm_gem_cpu_prep(obj: *mut drm_gem_object, op: u32, timeout: *mut ktime_t) -> c_int;
}
extern "C" {
    pub fn msm_gem_cpu_fini(obj: *mut drm_gem_object) -> c_int;
}
extern "C" {
    pub fn msm_gem_kernel_put(bo: *mut drm_gem_object, vm: *mut drm_gpuvm);
}
extern "C" {
    pub fn msm_gem_object_set_name(bo: *mut drm_gem_object, fmt: *const c_char, ...);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_gem_stats {
    pub count: unsigned,
    pub size: usize,
    pub purged: } all, active, resident, purgeable,,
}

extern "C" {
    pub fn msm_gem_describe_objects(list: *mut list_head, m: *mut seq_file);
}

extern "C" {
    pub fn dma_resv_trylock(_arg: obj->resv) -> return;
}
extern "C" {
    pub fn dma_resv_lock_interruptible(_arg: obj->resv, _arg: NULL) -> return;
}
//
// msm_gem_lock_vm_and_obj() - Helper to lock an obj + VM
// @exec: the exec context helper which will be initalized
// @obj: the GEM object to lock
// @vm: the VM to lock
//
// Operations which modify a VM frequently need to lock both the VM and
// the object being mapped/unmapped/etc.  This helper uses drm_exec to
// acquire both locks, dealing with potential deadlock/backoff scenarios
// which arise when multiple locks are involved.
//
// Destroying the object is a special case.. msm_gem_free_object()
// calls many things that WARN_ON if the obj lock is not held.  But
// acquiring the obj lock in msm_gem_free_object() can cause a
// locking order inversion between reservation_ww_class_mutex and
// fs_reclaim.
//
// This deadlock is not actually possible, because no one should
// be already holding the lock when msm_gem_free_object() is called.
// Unfortunately lockdep is not aware of this detail.  So when the
// refcount drops to zero, we pretend it is already locked.
//
// imported/exported objects are not purgeable:
extern "C" {
    pub fn msm_gem_purge(obj: *mut drm_gem_object);
}
extern "C" {
    pub fn msm_gem_evict(obj: *mut drm_gem_object);
}
extern "C" {
    pub fn msm_gem_vunmap(obj: *mut drm_gem_object);
}
// Created per submit-ioctl, to track bo's and cmdstream bufs, etc,
// associated with the cmdstream submission for synchronization (and
// make it easier to unwind when things go wrong, etc).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_gem_submit {
    pub base: drm_sched_job,
    pub ref: kref,
    pub dev: *mut drm_device,
    pub gpu: *mut msm_gpu,
    pub vm: *mut drm_gpuvm,
    pub /: *mut *mut list_head node; / node in ring submit list,
    pub exec: drm_exec,
    pub /: *mut *mut uint32_t seqno; / Sequence number of the submit on the ring,
// Hw fence, which is created when the scheduler executes the job, and
// is signaled when the hw finishes (via seqno write from cmdstream)
//
    pub hw_fence: *mut dma_fence,
// Userspace visible fence, which is signaled by the scheduler after
// the hw_fence is signaled.
//
    pub user_fence: *mut dma_fence,
    pub /: *mut *mut int fence_id; / key into queue->fence_idr,
    pub queue: *mut msm_gpu_submitqueue,
    pub /: *mut *mut *mut pid pid; / submitting process,
    pub 1: bool bos_pinned :,
    pub /: *mut *mut bool fault_dumped:1;/ Limit devcoredump dumping to one per submit,
    pub /: *mut *mut bool in_rb : 1; / "sudo" mode, copy cmds into RB,
    pub /: *mut *mut bool has_exec : 1; / @exec is initialized.,
    pub ring: *mut msm_ringbuffer,
    pub nr_cmds: c_uint,
    pub nr_bos: c_uint,
    pub /: *mut *mut u32 ident; / A "identifier" for the submit for logging,
    pub type: u32,
    pub /: *mut *mut uint32_t size; / in dwords,
    pub iova: u64,
    pub /: *mut *mut uint32_t offset;/ in dwords,
    pub /: *mut *mut uint32_t idx; / cmdstream buffer idx in bos[],
    pub nr_relocs: u32,
    pub relocs: *mut drm_msm_gem_submit_reloc,
    pub /: *mut *mut *mut } cmd; / array of size nr_cmds,
    pub flags: u32,
    pub obj: *mut drm_gem_object,
    pub handle: u32,
}

extern "C" {
    pub fn container_of(_arg: job, msm_gem_submit: struct, _arg: base) -> return;
}
extern "C" {
    pub fn __msm_gem_submit_destroy(kref: *mut kref);
}
extern "C" {
    pub fn msm_submit_retire(submit: *mut msm_gem_submit);
}
