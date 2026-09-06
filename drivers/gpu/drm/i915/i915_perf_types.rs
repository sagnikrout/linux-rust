//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/i915_perf_types.h
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
// Copyright © 2019 Intel Corporation
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum report_header {
    HDR_32_BIT = 0,
    HDR_64_BIT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_perf_regs {
    pub base: u32,
    pub oa_head_ptr: i915_reg_t,
    pub oa_tail_ptr: i915_reg_t,
    pub oa_buffer: i915_reg_t,
    pub oa_ctx_ctrl: i915_reg_t,
    pub oa_ctrl: i915_reg_t,
    pub oa_debug: i915_reg_t,
    pub oa_status: i915_reg_t,
    pub oa_ctrl_counter_format_shift: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum oa_type {
    TYPE_OAG,
    TYPE_OAM,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_oa_format {
    pub format: u32,
    pub size: c_int,
    pub type: c_int,
    pub header: report_header,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_oa_reg {
    pub addr: i915_reg_t,
    pub value: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_oa_config {
    pub perf: *mut i915_perf,
    pub 1]: char uuid[UUID_STRING_LEN +,
    pub id: c_int,
    pub mux_regs: *const i915_oa_reg,
    pub mux_regs_len: u32,
    pub b_counter_regs: *const i915_oa_reg,
    pub b_counter_regs_len: u32,
    pub flex_regs: *const i915_oa_reg,
    pub flex_regs_len: u32,
    pub sysfs_metric: attribute_group,
    pub attrs: [*mut attribute; 2],
    pub sysfs_metric_id: kobj_attribute,
    pub ref: kref,
    pub rcu: rcu_head,
}

//
// struct i915_perf_stream_ops - the OPs to support a specific stream type
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_perf_stream_ops {
//
// @enable: Enables the collection of HW samples, either in response to
// `I915_PERF_IOCTL_ENABLE` or implicitly called when stream is opened
// without `I915_PERF_FLAG_DISABLED`.
//
    pub stream): *mut *mut void (enable)(struct i915_perf_stream,
//
// @disable: Disables the collection of HW samples, either in response
// to `I915_PERF_IOCTL_DISABLE` or implicitly called before destroying
// the stream.
//
    pub stream): *mut *mut void (disable)(struct i915_perf_stream,
//
// @poll_wait: Call poll_wait, passing a wait queue that will be woken
// once there is something ready to read() for the stream
//
    pub wait): *mut poll_table,
//
// @wait_unlocked: For handling a blocking read, wait until there is
// something to ready to read() for the stream. E.g. wait on the same
// wait queue that would be passed to poll_wait().
//
    pub stream): *mut *mut int (wait_unlocked)(struct i915_perf_stream,
//
// @read: Copy buffered metrics as records to userspace
// **buf**: the userspace, destination buffer
// **count**: the number of bytes to copy, requested by userspace
// **offset**: zero at the start of the read, updated as the read
// proceeds, it represents how many bytes have been copied so far and
// the buffer offset for copying the next record.
//
// Copy as many buffered i915 perf samples and records for this stream
// to userspace as will fit in the given buffer.
//
// Only write complete records; returning -%ENOSPC if there isn't room
// for a complete record.
//
// Return any error condition that results in a short read such as
// -%ENOSPC or -%EFAULT, even though these may be squashed before
// returning to userspace.
//
    pub offset): *mut usize,
//
// @destroy: Cleanup any stream specific resources.
//
// The stream will always be disabled before this is called.
//
    pub stream): *mut *mut void (destroy)(struct i915_perf_stream,
}

//
// struct i915_perf_stream - state for a single open stream FD
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_perf_stream {
//
// @perf: i915_perf backpointer
//
    pub perf: *mut i915_perf,
//
// @uncore: mmio access path
//
    pub uncore: *mut intel_uncore,
//
// @engine: Engine associated with this performance stream.
//
    pub engine: *mut intel_engine_cs,
//
// @lock: Lock associated with operations on stream
//
    pub lock: mutex,
//
// @sample_flags: Flags representing the `DRM_I915_PERF_PROP_SAMPLE_*`
// properties given when opening a stream, representing the contents
// of a single sample as read() by userspace.
//
    pub sample_flags: u32,
//
// @sample_size: Considering the configured contents of a sample
// combined with the required header size, this is the total size
// of a single sample record.
//
    pub sample_size: c_int,
//
// @ctx: %NULL if measuring system-wide across all contexts or a
// specific context that is being monitored.
//
    pub ctx: *mut i915_gem_context,
//
// @enabled: Whether the stream is currently enabled, considering
// whether the stream was opened in a disabled state and based
// on `I915_PERF_IOCTL_ENABLE` and `I915_PERF_IOCTL_DISABLE` calls.
//
    pub enabled: bool,
//
// @hold_preemption: Whether preemption is put on hold for command
// submissions done on the @ctx. This is useful for some drivers that
// cannot easily post process the OA buffer context to subtract delta
// of performance counters not associated with @ctx.
//
    pub hold_preemption: bool,
//
// @ops: The callbacks providing the implementation of this specific
// type of configured stream.
//
    pub ops: *const i915_perf_stream_ops,
//
// @oa_config: The OA configuration used by the stream.
//
    pub oa_config: *mut i915_oa_config,
//
// @oa_config_bos: A list of struct i915_oa_config_bo allocated lazily
// each time @oa_config changes.
//
    pub oa_config_bos: llist_head,
//
// @pinned_ctx: The OA context specific information.
//
    pub pinned_ctx: *mut intel_context,
//
// @specific_ctx_id: The id of the specific context.
//
    pub specific_ctx_id: u32,
//
// @specific_ctx_id_mask: The mask used to masking specific_ctx_id bits.
//
    pub specific_ctx_id_mask: u32,
//
// @poll_check_timer: High resolution timer that will periodically
// check for data in the circular OA buffer for notifying userspace
// (e.g. during a read() or poll()).
//
    pub poll_check_timer: hrtimer,
//
// @poll_wq: The wait queue that hrtimer callback wakes when it
// sees data ready to read in the circular OA buffer.
//
    pub poll_wq: wait_queue_head_t,
//
// @pollin: Whether there is data available to read.
//
    pub pollin: bool,
//
// @periodic: Whether periodic sampling is currently enabled.
//
    pub periodic: bool,
//
// @period_exponent: The OA unit sampling frequency is derived from this.
//
    pub period_exponent: c_int,
//
// @oa_buffer: State of the OA buffer.
//
    pub format: *const i915_oa_format,
    pub vma: *mut i915_vma,
    pub vaddr: *mut u8,
    pub last_ctx_id: u32,
//
// @oa_buffer.ptr_lock: Locks reads and writes to all
// head/tail state
//
// Consider: the head and tail pointer state needs to be read
// consistently from a hrtimer callback (atomic context) and
// read() fop (user context) with tail pointer updates happening
// in atomic context and head updates in user context and the
// (unlikely) possibility of read() errors needing to reset all
// head/tail state.
//
// Note: Contention/performance aren't currently a significant
// concern here considering the relatively low frequency of
// hrtimer callbacks (5ms period) and that reads typically only
// happen in response to a hrtimer event and likely complete
// before the next callback.
//
// Note: This lock is not held *while* reading and copying data
// to userspace so the value of head observed in htrimer
// callbacks won't represent any partial consumption of data.
//
    pub ptr_lock: spinlock_t,
//
// @oa_buffer.head: Although we can always read back
// the head pointer register,
// we prefer to avoid trusting the HW state, just to avoid any
// risk that some hardware condition could * somehow bump the
// head pointer unpredictably and cause us to forward the wrong
// OA buffer data to userspace.
//
    pub head: u32,
//
// @oa_buffer.tail: The last verified tail that can be
// read by userspace.
//
    pub tail: u32,
    pub oa_buffer: },
//
// @noa_wait: A batch buffer doing a wait on the GPU for the NOA logic to be
// reprogrammed.
//
    pub noa_wait: *mut i915_vma,
//
// @poll_oa_period: The period in nanoseconds at which the OA
// buffer should be checked for available data.
//
    pub poll_oa_period: u64,
}

//
// struct i915_oa_ops - Gen specific implementation of an OA unit stream
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_oa_ops {
//
// @is_valid_b_counter_reg: Validates register's address for
// programming boolean counters for a particular platform.
//
    pub addr): *mut *mut *mut bool (is_valid_b_counter_reg)(struct i915_perf perf, u32,
//
// @is_valid_mux_reg: Validates register's address for programming mux
// for a particular platform.
//
    pub addr): *mut *mut *mut bool (is_valid_mux_reg)(struct i915_perf perf, u32,
//
// @is_valid_flex_reg: Validates register's address for programming
// flex EU filtering for a particular platform.
//
    pub addr): *mut *mut *mut bool (is_valid_flex_reg)(struct i915_perf perf, u32,
//
// @enable_metric_set: Selects and applies any MUX configuration to set
// up the Boolean and Custom (B/C) counters that are part of the
// counter reports being sampled. May apply system constraints such as
// disabling EU clock gating as required.
//
    pub active): *mut i915_active,
//
// @disable_metric_set: Remove system constraints associated with using
// the OA unit.
//
    pub stream): *mut *mut void (disable_metric_set)(struct i915_perf_stream,
//
// @oa_enable: Enable periodic sampling
//
    pub stream): *mut *mut void (oa_enable)(struct i915_perf_stream,
//
// @oa_disable: Disable periodic sampling
//
    pub stream): *mut *mut void (oa_disable)(struct i915_perf_stream,
//
// @read: Copy data from the circular OA buffer into a given userspace
// buffer.
//
    pub offset): *mut usize,
//
// @oa_hw_tail_read: read the OA tail pointer register
//
// In particular this enables us to share all the fiddly code for
// handling the OA unit tail pointer race that affects multiple
// generations.
//
    pub stream): *mut *mut u32 (oa_hw_tail_read)(struct i915_perf_stream,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_perf_group {
//
// @exclusive_stream: The stream currently using the OA unit. This is
// sometimes accessed outside a syscall associated to its file
// descriptor.
//
    pub exclusive_stream: *mut i915_perf_stream,
//
// @num_engines: The number of engines using this OA unit.
//
    pub num_engines: u32,
//
// @regs: OA buffer register group for programming the OA unit.
//
    pub regs: i915_perf_regs,
//
// @type: Type of OA unit - OAM, OAG etc.
//
    pub type: oa_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_perf_gt {
//
// Lock associated with anything below within this structure.
//
    pub lock: mutex,
//
// @sseu: sseu configuration selected to run while perf is active,
// applies to all contexts.
//
    pub sseu: intel_sseu,
//
// @num_perf_groups: number of perf groups per gt.
//
    pub num_perf_groups: u32,
//
// @group: list of OA groups - one for each OA buffer.
//
    pub group: *mut i915_perf_group,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_perf {
    pub i915: *mut drm_i915_private,
    pub metrics_kobj: *mut kobject,
//
// Lock associated with adding/modifying/removing OA configs
// in perf->metrics_idr.
//
    pub metrics_lock: mutex,
//
// List of dynamic configurations (struct i915_oa_config), you
// need to hold perf->metrics_lock to access it.
//
    pub metrics_idr: idr,
//
// For rate limiting any notifications of spurious
// invalid OA reports
//
    pub spurious_report_rs: ratelimit_state,
//
// For rate limiting any notifications of tail pointer
// race.
//
    pub tail_pointer_race: ratelimit_state,
    pub gen7_latched_oastatus1: u32,
    pub ctx_oactxctrl_offset: u32,
    pub ctx_flexeu0_offset: u32,
//
// The RPT_ID/reason field for Gen8+ includes a bit
// to determine if the CTX ID in the report is valid
// but the specific bit differs between Gen 8 and 9
//
    pub gen8_valid_ctx_bit: u32,
    pub ops: i915_oa_ops,
    pub oa_formats: *const i915_oa_format,
//
// Use a format mask to store the supported formats
// for a platform.
//
    pub format_mask: [c_ulong; FORMAT_MASK_SIZE],
    pub noa_programming_delay: core::sync::atomic::AtomicI64,
}
