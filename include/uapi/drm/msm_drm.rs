//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/drm/msm_drm.h
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
// Copyright (C) 2013 Red Hat
// Author: Rob Clark <robdclark@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice (including the next
// paragraph) shall be included in all copies or substantial portions of the
// Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

// Please note that modifications to all structs defined here are
// subject to backwards-compatibility constraints:
// 1) Do not use pointers, use __u64 instead for 32 bit / 64 bit
// user/kernel compatibility
// 2) Keep fields aligned to their size
// 3) Because of how drm_ioctl() works, we can add new fields at
// the end of an ioctl if some care is taken: drm_ioctl() will
// zero out the new fields at the tail of the ioctl, so a zero
// value should have a backwards compatible meaning.  And for
// output params, userspace won't see the newly added output
// fields.. so that has to be somehow ok.
//
pub const MSM_PIPE_NONE: c_uint = 0x00;
pub const MSM_PIPE_2D0: c_uint = 0x01;
pub const MSM_PIPE_2D1: c_uint = 0x02;
pub const MSM_PIPE_3D0: c_uint = 0x10;
// The pipe-id just uses the lower bits, so can be OR'd with flags in
// the upper 16 bits (which could be extended further, if needed, maybe
// we extend/overload the pipe-id some day to deal with multiple rings,
// but even then I don't think we need the full lower 16 bits).
//
pub const MSM_PIPE_ID_MASK: c_uint = 0xffff;

// timeouts are specified in clock-monotonic absolute times (to simplify
// restarting interrupted ioctls).  The following struct is logically the
// same as 'struct timespec' but 32/64b ABI safe.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_msm_timespec {
    pub /: *mut *mut __s64 tv_sec; / seconds,
    pub /: *mut *mut __s64 tv_nsec; / nanoseconds,
}

// Below "RO" indicates a read-only param, "WO" indicates write-only, and
// "RW" indicates a param that can be both read (GET_PARAM) and written
// (SET_PARAM)
//
pub const MSM_PARAM_GPU_ID: c_uint = 0x01  /* RO */;
pub const MSM_PARAM_GMEM_SIZE: c_uint = 0x02  /* RO */;
pub const MSM_PARAM_CHIP_ID: c_uint = 0x03  /* RO */;
pub const MSM_PARAM_MAX_FREQ: c_uint = 0x04  /* RO */;
pub const MSM_PARAM_TIMESTAMP: c_uint = 0x05  /* RO */;
pub const MSM_PARAM_GMEM_BASE: c_uint = 0x06  /* RO */;
pub const MSM_PARAM_PRIORITIES: c_uint = 0x07  /* RO: The # of priority levels */;
pub const MSM_PARAM_PP_PGTABLE: c_uint = 0x08  /* RO: Deprecated, always returns zero */;
pub const MSM_PARAM_FAULTS: c_uint = 0x09  /* RO */;
pub const MSM_PARAM_SUSPENDS: c_uint = 0x0a  /* RO */;
pub const MSM_PARAM_SYSPROF: c_uint = 0x0b  /* WO: 1 preserves perfcntrs, 2 also disables suspend */;
pub const MSM_PARAM_COMM: c_uint = 0x0c  /* WO: override for task->comm */;
pub const MSM_PARAM_CMDLINE: c_uint = 0x0d  /* WO: override for task cmdline */;
pub const MSM_PARAM_VA_START: c_uint = 0x0e  /* RO: start of valid GPU iova range */;
pub const MSM_PARAM_VA_SIZE: c_uint = 0x0f  /* RO: size of valid GPU iova range (bytes) */;
pub const MSM_PARAM_HIGHEST_BANK_BIT: c_uint = 0x10 /* RO */;
pub const MSM_PARAM_RAYTRACING: c_uint = 0x11 /* RO */;
pub const MSM_PARAM_UBWC_SWIZZLE: c_uint = 0x12 /* RO */;
pub const MSM_PARAM_MACROTILE_MODE: c_uint = 0x13 /* RO */;
pub const MSM_PARAM_UCHE_TRAP_BASE: c_uint = 0x14 /* RO */;
// PRR (Partially Resident Region) is required for sparse residency:
pub const MSM_PARAM_HAS_PRR: c_uint = 0x15  /* RO */;
// MSM_PARAM_EN_VM_BIND is set to 1 to enable VM_BIND ops.
//
// With VM_BIND enabled, userspace is required to allocate iova and use the
// VM_BIND ops for map/unmap ioctls.  MSM_INFO_SET_IOVA and MSM_INFO_GET_IOVA
// will be rejected.  (The latter does not have a sensible meaning when a BO
// can have multiple and/or partial mappings.)
//
// With VM_BIND enabled, userspace does not include a submit_bo table in the
// SUBMIT ioctl (this will be rejected), the resident set is determined by
// the the VM_BIND ops.
//
// Enabling VM_BIND will fail on devices which do not have per-process pgtables.
// And it is not allowed to disable VM_BIND once it has been enabled.
//
// Enabling VM_BIND should be done (attempted) prior to allocating any BOs or
// submitqueues of type MSM_SUBMITQUEUE_VM_BIND.
//
// Relatedly, when VM_BIND mode is enabled, the kernel will not try to recover
// from GPU faults or failed async VM_BIND ops, in particular because it is
// difficult to communicate to userspace which op failed so that userspace
// could rewind and try again.  When the VM is marked unusable, the SUBMIT
// ioctl will throw -EPIPE.
//
pub const MSM_PARAM_EN_VM_BIND: c_uint = 0x16  /* WO, once */;
pub const MSM_PARAM_AQE: c_uint = 0x17  /* RO */;
// For backwards compat.  The original support for preemption was based on
// a single ring per priority level so # of priority levels equals the #
// of rings.  With drm/scheduler providing additional levels of priority,
// the number of priorities is greater than the # of rings.  The param is
// renamed to better reflect this.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_msm_param {
    pub /: *mut *mut __u32 pipe; / in, MSM_PIPE_x,
    pub /: *mut *mut __u32 param; / in, MSM_PARAM_x,
    pub /: *mut *mut __u64 value; / out (get_param) or in (set_param),
    pub /: *mut *mut __u32 len; / zero for non-pointer params,
    pub /: *mut *mut __u32 pad; / must be zero,
}

//
// GEM buffers:
//
pub const MSM_BO_SCANOUT: c_uint = 0x00000001     /* scanout capable */;
pub const MSM_BO_GPU_READONLY: c_uint = 0x00000002;
// Private buffers do not need to be explicitly listed in the SUBMIT
// ioctl, unless referenced by a drm_msm_gem_submit_cmd.  Private
// buffers may NOT be imported/exported or used for scanout (or any
// other situation where buffers can be indefinitely pinned, but
// cases other than scanout are all kernel owned BOs which are not
// visible to userspace).
//
// In exchange for those constraints, all private BOs associated with
// a single context (drm_file) share a single dma_resv, and if there
// has been no eviction since the last submit, there are no per-BO
// bookeeping to do, significantly cutting the SUBMIT overhead.
//
pub const MSM_BO_NO_SHARE: c_uint = 0x00000004;
pub const MSM_BO_CACHE_MASK: c_uint = 0x000f0000;
// cache modes
pub const MSM_BO_CACHED: c_uint = 0x00010000;
pub const MSM_BO_WC: c_uint = 0x00020000;
pub const MSM_BO_UNCACHED: c_uint = 0x00040000 /* deprecated, use MSM_BO_WC */;
pub const MSM_BO_CACHED_COHERENT: c_uint = 0x080000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_msm_gem_new {
    pub /: *mut *mut __u64 size; / in,
    pub /: *mut *mut __u32 flags; / in, mask of MSM_BO_x,
    pub /: *mut *mut __u32 handle; / out,
}

// Get or set GEM buffer info.  The requested value can be passed
// directly in 'value', or for data larger than 64b 'value' is a
// pointer to userspace buffer, with 'len' specifying the number of
// bytes copied into that buffer.  For info returned by pointer,
// calling the GEM_INFO ioctl with null 'value' will return the
// required buffer size in 'len'
//
pub const MSM_INFO_GET_OFFSET: c_uint = 0x00   /* get mmap() offset, returned by value */;
pub const MSM_INFO_GET_IOVA: c_uint = 0x01   /* get iova, returned by value */;
pub const MSM_INFO_SET_NAME: c_uint = 0x02   /* set the debug name (by pointer) */;
pub const MSM_INFO_GET_NAME: c_uint = 0x03   /* get debug name, returned by pointer */;
pub const MSM_INFO_SET_IOVA: c_uint = 0x04   /* set the iova, passed by value */;
pub const MSM_INFO_GET_FLAGS: c_uint = 0x05   /* get the MSM_BO_x flags */;
pub const MSM_INFO_SET_METADATA: c_uint = 0x06   /* set userspace metadata */;
pub const MSM_INFO_GET_METADATA: c_uint = 0x07   /* get userspace metadata */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_msm_gem_info {
    pub /: *mut *mut __u32 handle; / in,
    pub /: *mut *mut *mut __u32 info; / in - one of MSM_INFO_,
    pub /: *mut *mut __u64 value; / in or out,
    pub /: *mut *mut __u32 len; / in or out,
    pub pad: __u32,
}

pub const MSM_PREP_READ: c_uint = 0x01;
pub const MSM_PREP_WRITE: c_uint = 0x02;
pub const MSM_PREP_NOSYNC: c_uint = 0x04;
pub const MSM_PREP_BOOST: c_uint = 0x08;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_msm_gem_cpu_prep {
    pub /: *mut *mut __u32 handle; / in,
    pub /: *mut *mut __u32 op; / in, mask of MSM_PREP_x,
    pub /: *mut *mut drm_msm_timespec timeout; / in,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_msm_gem_cpu_fini {
    pub /: *mut *mut __u32 handle; / in,
}

//
// Cmdstream Submission:
//
pub const MSM_SYNCOBJ_RESET: c_uint = 0x00000001 /* Reset syncobj after wait. */;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_msm_syncobj {
    pub /: *mut *mut __u32 handle; / in, syncobj handle.,
    pub /: *mut *mut __u32 flags; / in, from MSM_SYNCOBJ_FLAGS,
    pub /: *mut *mut __u64 point; / in, timepoint for timeline syncobjs.,
}

// The value written into the cmdstream is logically:
//
// ((relocbuf->gpuaddr + reloc_offset) << shift) | or
//
// When we have GPU's w/ >32bit ptrs, it should be possible to deal
// with this by emit'ing two reloc entries with appropriate shift
// values.  Or a new MSM_SUBMIT_CMD_x type would also be an option.
//
// NOTE that reloc's must be sorted by order of increasing submit_offset,
// otherwise EINVAL.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_msm_gem_submit_reloc {
    pub /: *mut *mut __u32 submit_offset; / in, offset from submit_bo,

    pub /: *mut *mut __u32 _or; / in, value OR'd with result,

    pub /: *mut *mut __u32 or; / in, value OR'd with result,

    pub /: *mut *mut __s32 shift; / in, amount of left shift (can be negative),
    pub /: *mut *mut __u32 reloc_idx; / in, index of reloc_bo buffer,
    pub /: *mut *mut __u64 reloc_offset; / in, offset from start of reloc_bo,
}

// submit-types:
// BUF - this cmd buffer is executed normally.
// IB_TARGET_BUF - this cmd buffer is an IB target.  Reloc's are
// processed normally, but the kernel does not setup an IB to
// this buffer in the first-level ringbuffer
// CTX_RESTORE_BUF - only executed if there has been a GPU context
// switch since the last SUBMIT ioctl
//
pub const MSM_SUBMIT_CMD_BUF: c_uint = 0x0001;
pub const MSM_SUBMIT_CMD_IB_TARGET_BUF: c_uint = 0x0002;
pub const MSM_SUBMIT_CMD_CTX_RESTORE_BUF: c_uint = 0x0003;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_msm_gem_submit_cmd {
    pub /: *mut *mut __u32 type; / in, one of MSM_SUBMIT_CMD_x,
    pub /: *mut *mut __u32 submit_idx; / in, index of submit_bo cmdstream buffer,
    pub /: *mut *mut __u32 submit_offset; / in, offset into submit_bo,
    pub /: *mut *mut __u32 size; / in, cmdstream size,
    pub pad: __u32,
    pub /: *mut *mut __u32 nr_relocs; / in, number of submit_reloc's,
    pub /: *mut *mut __u64 relocs; / in, ptr to array of submit_reloc's,
    pub /: *mut *mut __u64 iova; / cmdstream address (for VM_BIND contexts),
}

// Each buffer referenced elsewhere in the cmdstream submit (ie. the
// cmdstream buffer(s) themselves or reloc entries) has one (and only
// one) entry in the submit->bos[] table.
//
// As a optimization, the current buffer (gpu virtual address) can be
// passed back through the 'presumed' field.  If on a subsequent reloc,
// userspace passes back a 'presumed' address that is still valid,
// then patching the cmdstream for this entry is skipped.  This can
// avoid kernel needing to map/access the cmdstream bo in the common
// case.
//
pub const MSM_SUBMIT_BO_READ: c_uint = 0x0001;
pub const MSM_SUBMIT_BO_WRITE: c_uint = 0x0002;
pub const MSM_SUBMIT_BO_DUMP: c_uint = 0x0004;
pub const MSM_SUBMIT_BO_NO_IMPLICIT: c_uint = 0x0008;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_msm_gem_submit_bo {
    pub /: *mut *mut __u32 flags; / in, mask of MSM_SUBMIT_BO_x,
    pub /: *mut *mut __u32 handle; / in, GEM handle,
    pub /: *mut *mut __u64 presumed; / in/out, presumed buffer address,
}

// Valid submit ioctl flags:
pub const MSM_SUBMIT_NO_IMPLICIT: c_uint = 0x80000000 /* disable implicit sync */;
pub const MSM_SUBMIT_FENCE_FD_IN: c_uint = 0x40000000 /* enable input fence_fd */;
pub const MSM_SUBMIT_FENCE_FD_OUT: c_uint = 0x20000000 /* enable output fence_fd */;
pub const MSM_SUBMIT_SUDO: c_uint = 0x10000000 /* run submitted cmds from RB */;
pub const MSM_SUBMIT_SYNCOBJ_IN: c_uint = 0x08000000 /* enable input syncobj */;
pub const MSM_SUBMIT_SYNCOBJ_OUT: c_uint = 0x04000000 /* enable output syncobj */;
pub const MSM_SUBMIT_FENCE_SN_IN: c_uint = 0x02000000 /* userspace passes in seqno fence */;

// Each cmdstream submit consists of a table of buffers involved, and
// one or more cmdstream buffers.  This allows for conditional execution
// (context-restore), and IB buffers needed for per tile/bin draw cmds.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_msm_gem_submit {
    pub /: *mut *mut __u32 flags; / MSM_PIPE_x | MSM_SUBMIT_x,
    pub /: *mut *mut __u32 fence; / out (or in with MSM_SUBMIT_FENCE_SN_IN flag),
    pub /: *mut *mut __u32 nr_bos; / in, number of submit_bo's,
    pub /: *mut *mut __u32 nr_cmds; / in, number of submit_cmd's,
    pub /: *mut *mut __u64 bos; / in, ptr to array of submit_bo's,
    pub /: *mut *mut __u64 cmds; / in, ptr to array of submit_cmd's,
    pub /: *mut *mut __s32 fence_fd; / in/out fence fd (see MSM_SUBMIT_FENCE_FD_IN/OUT),
    pub /: *mut *mut __u32 queueid; / in, submitqueue id,
    pub /: *mut *mut __u64 in_syncobjs; / in, ptr to array of drm_msm_syncobj,
    pub /: *mut *mut __u64 out_syncobjs; / in, ptr to array of drm_msm_syncobj,
    pub /: *mut *mut __u32 nr_in_syncobjs; / in, number of entries in in_syncobj,
    pub /: *mut *mut __u32 nr_out_syncobjs; / in, number of entries in out_syncobj.,
    pub /: *mut *mut __u32 syncobj_stride; / in, stride of syncobj arrays.,
    pub /: *mut *mut __u32 pad; /in, reserved for future use, always 0.,
}

pub const MSM_VM_BIND_OP_UNMAP: c_int = 0;
pub const MSM_VM_BIND_OP_MAP: c_int = 1;
pub const MSM_VM_BIND_OP_MAP_NULL: c_int = 2;
pub const MSM_VM_BIND_OP_DUMP: c_int = 1;

//
// struct drm_msm_vm_bind_op - bind/unbind op to run
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_msm_vm_bind_op {
// @op: one of MSM_VM_BIND_OP_x
    pub op: __u32,
// @handle: GEM object handle, MBZ for UNMAP or MAP_NULL
    pub handle: __u32,
// @obj_offset: Offset into GEM object, MBZ for UNMAP or MAP_NULL
    pub obj_offset: __u64,
// @iova: Address to operate on
    pub iova: __u64,
// @range: Number of bites to to map/unmap
    pub range: __u64,
// @flags: Bitmask of MSM_VM_BIND_OP_FLAG_x
    pub flags: __u32,
// @pad: MBZ
    pub pad: __u32,
}

pub const MSM_VM_BIND_FENCE_FD_IN: c_uint = 0x00000001;
pub const MSM_VM_BIND_FENCE_FD_OUT: c_uint = 0x00000002;

//
// struct drm_msm_vm_bind - Input of &DRM_IOCTL_MSM_VM_BIND
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_msm_vm_bind {
// @flags: in, bitmask of MSM_VM_BIND_x
    pub flags: __u32,
// @nr_ops: the number of bind ops in this ioctl
    pub nr_ops: __u32,
// @fence_fd: in/out fence fd (see MSM_VM_BIND_FENCE_FD_IN/OUT)
    pub fence_fd: __s32,
// @queue_id: in, submitqueue id
    pub queue_id: __u32,
// @in_syncobjs: in, ptr to array of drm_msm_syncobj
    pub in_syncobjs: __u64,
// @out_syncobjs: in, ptr to array of drm_msm_syncobj
    pub out_syncobjs: __u64,
// @nr_in_syncobjs: in, number of entries in in_syncobj
    pub nr_in_syncobjs: __u32,
// @nr_out_syncobjs: in, number of entries in out_syncobj
    pub nr_out_syncobjs: __u32,
// @syncobj_stride: in, stride of syncobj arrays
    pub syncobj_stride: __u32,
// @op_stride: sizeof each struct drm_msm_vm_bind_op in @ops
    pub op_stride: __u32,
// @op: used if num_ops == 1
    pub op: drm_msm_vm_bind_op,
// @ops: userptr to array of drm_msm_vm_bind_op if num_ops > 1
    pub ops: __u64,
}

pub const MSM_WAIT_FENCE_BOOST: c_uint = 0x00000001;

// The normal way to synchronize with the GPU is just to CPU_PREP on
// a buffer if you need to access it from the CPU (other cmdstream
// submission from same or other contexts, PAGE_FLIP ioctl, etc, all
// handle the required synchronization under the hood).  This ioctl
// mainly just exists as a way to implement the gallium pipe_fence
// APIs without requiring a dummy bo to synchronize on.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_msm_wait_fence {
    pub /: *mut *mut __u32 fence; / in,
    pub /: *mut *mut __u32 flags; / in, bitmask of MSM_WAIT_FENCE_x,
    pub /: *mut *mut drm_msm_timespec timeout; / in,
    pub /: *mut *mut __u32 queueid; / in, submitqueue id,
}

// madvise provides a way to tell the kernel in case a buffers contents
// can be discarded under memory pressure, which is useful for userspace
// bo cache where we want to optimistically hold on to buffer allocate
// and potential mmap, but allow the pages to be discarded under memory
// pressure.
//
// Typical usage would involve madvise(DONTNEED) when buffer enters BO
// cache, and madvise(WILLNEED) if trying to recycle buffer from BO cache.
// In the WILLNEED case, 'retained' indicates to userspace whether the
// backing pages still exist.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_msm_gem_madvise {
    pub /: *mut *mut __u32 handle; / in, GEM handle,
    pub /: *mut *mut __u32 madv; / in, MSM_MADV_x,
    pub /: *mut *mut __u32 retained; / out, whether backing store still exists,
}

//
// Draw queues allow the user to set specific submission parameter. Command
// submissions specify a specific submitqueue to use.  ID 0 is reserved for
// backwards compatibility as a "default" submitqueue.
//
// Because VM_BIND async updates happen on the CPU, they must run on a
// virtual queue created with the flag MSM_SUBMITQUEUE_VM_BIND.  If we had
// a way to do pgtable updates on the GPU, we could drop this restriction.
//
pub const MSM_SUBMITQUEUE_ALLOW_PREEMPT: c_uint = 0x00000001;
pub const MSM_SUBMITQUEUE_VM_BIND: c_uint = 0x00000002  /* virtual queue for VM_BIND ops */;

//
// The submitqueue priority should be between 0 and MSM_PARAM_PRIORITIES-1,
// a lower numeric value is higher priority.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_msm_submitqueue {
    pub /: *mut *mut __u32 flags; / in, MSM_SUBMITQUEUE_x,
    pub /: *mut *mut __u32 prio; / in, Priority level,
    pub /: *mut *mut __u32 id; / out, identifier,
}

pub const MSM_SUBMITQUEUE_PARAM_FAULTS: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_msm_submitqueue_query {
    pub data: __u64,
    pub id: __u32,
    pub param: __u32,
    pub len: __u32,
    pub pad: __u32,
}

pub const MSM_PERFCNTR_STREAM: c_uint = 0x00000001;
pub const MSM_PERFCNTR_UPDATE: c_uint = 0x00000002;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_msm_perfcntr_group {
    pub group_name: [c_char; 16],
    pub nr_countables: __u32,
    pub /: *mut *mut __u32 pad; / mbz,
    pub /: *mut *mut __u64 countables; / pointer to an array of nr_countables u32,
}

//
// Note, for MSM_PERFCNTR_STREAM, the ioctl returns an fd to read recorded
// counters.  This only works because the ioctl is DRM_IOW(), if we returned
// a out param in the ioctl struct the copy_to_user() (in drm_ioctl())
// could fault, causing us to leak the fd.
//
// If the ioctl returns with error E2BIG, that means more counters/countables
// are requested than are currently available.  If MSM_PERFCNTR_UPDATE flag
// is set, drm_msm_perfcntr_group::nr_countables will be updated to return
// the actual # of counters available.
//
// The data read from the has the following format for each sampling period:
//
// uint64_t timestamp;  // CP_ALWAYS_ON_COUNTER captured at sample time
// uint32_t seqno;      // increments by 1 each period, reset to 0 on discontinuity
// uint32_t mbz;        // pad out counters to 64b
// struct {
// uint64_t counter[nr_countables];
// } groups[nr_groups];
//
// The ordering of groups and counters matches the order in PERFCNTR_CONFIG
// ioctl.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_msm_perfcntr_config {
    pub /: *mut *mut __u32 flags; / bitmask of MSM_PERFCNTR_x,
    pub /: *mut *mut __u32 nr_groups; / # of entries in groups array,
    pub /: *mut *mut __u64 groups; / pointer to array of drm_msm_perfcntr_group,
    pub /: *mut *mut __u64 period; / sampling period in ns,
    pub /: *mut *mut __u32 bufsz_shift; / sample buffer size in bytes is 1<<bufsz_shift,
    pub /: *mut *mut __u32 group_stride; / sizeof(struct drm_msm_perfcntr_group),
}

pub const DRM_MSM_GET_PARAM: c_uint = 0x00;
pub const DRM_MSM_SET_PARAM: c_uint = 0x01;
pub const DRM_MSM_GEM_NEW: c_uint = 0x02;
pub const DRM_MSM_GEM_INFO: c_uint = 0x03;
pub const DRM_MSM_GEM_CPU_PREP: c_uint = 0x04;
pub const DRM_MSM_GEM_CPU_FINI: c_uint = 0x05;
pub const DRM_MSM_GEM_SUBMIT: c_uint = 0x06;
pub const DRM_MSM_WAIT_FENCE: c_uint = 0x07;
pub const DRM_MSM_GEM_MADVISE: c_uint = 0x08;
// placeholder:
pub const DRM_MSM_GEM_SVM_NEW: c_uint = 0x09;
//
pub const DRM_MSM_SUBMITQUEUE_NEW: c_uint = 0x0A;
pub const DRM_MSM_SUBMITQUEUE_CLOSE: c_uint = 0x0B;
pub const DRM_MSM_SUBMITQUEUE_QUERY: c_uint = 0x0C;
pub const DRM_MSM_VM_BIND: c_uint = 0x0D;
pub const DRM_MSM_PERFCNTR_CONFIG: c_uint = 0x0E;

