//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gem/i915_gem_context_types.h
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

//
// struct i915_gem_engines - A set of engines
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_gem_engines {
// @link: Link in i915_gem_context::stale::engines
    pub link: list_head,
// @rcu: RCU to use when freeing
    pub rcu: rcu_head,
}

// @fence: Fence used for delayed destruction of engines
// @ctx: i915_gem_context backpointer
// @num_engines: Number of engines in this set
// @engines: Array of engines
//
// struct i915_gem_engines_iter - Iterator for an i915_gem_engines set
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_gem_engines_iter {
// @idx: Index into i915_gem_engines::engines
    pub idx: c_uint,
// @engines: Engine set being iterated
    pub engines: *const i915_gem_engines,
}

//
// enum i915_gem_engine_type - Describes the type of an i915_gem_proto_engine
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i915_gem_engine_type {
// @I915_GEM_ENGINE_TYPE_INVALID: An invalid engine
    I915_GEM_ENGINE_TYPE_INVALID = 0,

// @I915_GEM_ENGINE_TYPE_PHYSICAL: A single physical engine
    I915_GEM_ENGINE_TYPE_PHYSICAL,

// @I915_GEM_ENGINE_TYPE_BALANCED: A load-balanced engine set
    I915_GEM_ENGINE_TYPE_BALANCED,

// @I915_GEM_ENGINE_TYPE_PARALLEL: A parallel engine set
    I915_GEM_ENGINE_TYPE_PARALLEL,
}

//
// struct i915_gem_proto_engine - prototype engine
//
// This struct describes an engine that a context may contain.  Engines
// have four types:
//
// - I915_GEM_ENGINE_TYPE_INVALID: Invalid engines can be created but they
// show up as a NULL in i915_gem_engines::engines[i] and any attempt to
// use them by the user results in -EINVAL.  They are also useful during
// proto-context construction because the client may create invalid
// engines and then set them up later as virtual engines.
//
// - I915_GEM_ENGINE_TYPE_PHYSICAL: A single physical engine, described by
// i915_gem_proto_engine::engine.
//
// - I915_GEM_ENGINE_TYPE_BALANCED: A load-balanced engine set, described
// i915_gem_proto_engine::num_siblings and i915_gem_proto_engine::siblings.
//
// - I915_GEM_ENGINE_TYPE_PARALLEL: A parallel submission engine set, described
// i915_gem_proto_engine::width, i915_gem_proto_engine::num_siblings, and
// i915_gem_proto_engine::siblings.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_gem_proto_engine {
// @type: Type of this engine
    pub type: i915_gem_engine_type,
// @engine: Engine, for physical
    pub engine: *mut intel_engine_cs,
// @num_siblings: Number of balanced or parallel siblings
    pub num_siblings: c_uint,
// @width: Width of each sibling
    pub width: c_uint,
// @siblings: Balanced siblings or num_siblings * width for parallel
    pub siblings: *mut intel_engine_cs,
// @sseu: Client-set SSEU parameters
    pub sseu: intel_sseu,
}

//
// struct i915_gem_proto_context - prototype context
//
// The struct i915_gem_proto_context represents the creation parameters for
// a struct i915_gem_context.  This is used to gather parameters provided
// either through creation flags or via SET_CONTEXT_PARAM so that, when we
// create the final i915_gem_context, those parameters can be immutable.
//
// The context uAPI allows for two methods of setting context parameters:
// SET_CONTEXT_PARAM and CONTEXT_CREATE_EXT_SETPARAM.  The former is
// allowed to be called at any time while the later happens as part of
// GEM_CONTEXT_CREATE.  When these were initially added, Currently,
// everything settable via one is settable via the other.  While some
// params are fairly simple and setting them on a live context is harmless
// such the context priority, others are far trickier such as the VM or the
// set of engines.  To avoid some truly nasty race conditions, we don't
// allow setting the VM or the set of engines on live contexts.
//
// The way we dealt with this without breaking older userspace that sets
// the VM or engine set via SET_CONTEXT_PARAM is to delay the creation of
// the actual context until after the client is done configuring it with
// SET_CONTEXT_PARAM.  From the perspective of the client, it has the same
// u32 context ID the whole time.  From the perspective of i915, however,
// it's an i915_gem_proto_context right up until the point where we attempt
// to do something which the proto-context can't handle at which point the
// real context gets created.
//
// This is accomplished via a little xarray dance.  When GEM_CONTEXT_CREATE
// is called, we create a proto-context, reserve a slot in context_xa but
// leave it NULL, the proto-context in the corresponding slot in
// proto_context_xa.  Then, whenever we go to look up a context, we first
// check context_xa.  If it's there, we return the i915_gem_context and
// we're done.  If it's not, we look in proto_context_xa and, if we find it
// there, we create the actual context and kill the proto-context.
//
// At the time we made this change (April, 2021), we did a fairly complete
// audit of existing userspace to ensure this wouldn't break anything:
//
// - Mesa/i965 didn't use the engines or VM APIs at all
//
// - Mesa/ANV used the engines API but via CONTEXT_CREATE_EXT_SETPARAM and
// didn't use the VM API.
//
// - Mesa/iris didn't use the engines or VM APIs at all
//
// - The open-source compute-runtime didn't yet use the engines API but
// did use the VM API via SET_CONTEXT_PARAM.  However, CONTEXT_SETPARAM
// was always the second ioctl on that context, immediately following
// GEM_CONTEXT_CREATE.
//
// - The media driver sets engines and bonding/balancing via
// SET_CONTEXT_PARAM.  However, CONTEXT_SETPARAM to set the VM was
// always the second ioctl on that context, immediately following
// GEM_CONTEXT_CREATE and setting engines immediately followed that.
//
// In order for this dance to work properly, any modification to an
// i915_gem_proto_context that is exposed to the client via
// drm_i915_file_private::proto_context_xa must be guarded by
// drm_i915_file_private::proto_context_lock.  The exception is when a
// proto-context has not yet been exposed such as when handling
// CONTEXT_CREATE_SET_PARAM during GEM_CONTEXT_CREATE.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_gem_proto_context {
// @fpriv: Client which creates the context
    pub fpriv: *mut drm_i915_file_private,
// @vm: See &i915_gem_context.vm
    pub vm: *mut i915_address_space,
// @user_flags: See &i915_gem_context.user_flags
    pub user_flags: c_ulong,
// @sched: See &i915_gem_context.sched
    pub sched: i915_sched_attr,
// @num_user_engines: Number of user-specified engines or -1
    pub num_user_engines: c_int,
// @user_engines: User-specified engines
    pub user_engines: *mut i915_gem_proto_engine,
// @legacy_rcs_sseu: Client-set SSEU parameters for the legacy RCS
    pub legacy_rcs_sseu: intel_sseu,
// @single_timeline: See See &i915_gem_context.syncobj
    pub single_timeline: bool,
// @uses_protected_content: See &i915_gem_context.uses_protected_content
    pub uses_protected_content: bool,
// @pxp_wakeref: See &i915_gem_context.pxp_wakeref
    pub pxp_wakeref: intel_wakeref_t,
}

//
// struct i915_gem_context - client state
//
// The struct i915_gem_context represents the combined view of the driver and
// logical hardware state for a particular client.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_gem_context {
// @i915: i915 device backpointer
    pub i915: *mut drm_i915_private,
// @file_priv: owning file descriptor
    pub file_priv: *mut drm_i915_file_private,
//
// @engines: User defined engines for this context
//
// Various uAPI offer the ability to lookup up an
// index from this array to select an engine operate on.
//
// Multiple logically distinct instances of the same engine
// may be defined in the array, as well as composite virtual
// engines.
//
// Execbuf uses the I915_EXEC_RING_MASK as an index into this
// array to select which HW context + engine to execute on. For
// the default array, the user_ring_map[] is used to translate
// the legacy uABI onto the appropriate index (e.g. both
// I915_EXEC_DEFAULT and I915_EXEC_RENDER select the same
// context, and I915_EXEC_BSD is weird). For a user defined
// array, execbuf uses I915_EXEC_RING_MASK as a plain index.
//
// User defined by I915_CONTEXT_PARAM_ENGINE (when the
// CONTEXT_USER_ENGINES flag is set).
//
    pub engines: *mut i915_gem_engines __rcu,
// @engines_mutex: guards writes to engines
    pub engines_mutex: mutex,
//
// @syncobj: Shared timeline syncobj
//
// When the SHARED_TIMELINE flag is set on context creation, we
// emulate a single timeline across all engines using this syncobj.
// For every execbuffer2 call, this syncobj is used as both an in-
// and out-fence.  Unlike the real intel_timeline, this doesn't
// provide perfect atomic in-order guarantees if the client races
// with itself by calling execbuffer2 twice concurrently.  However,
// if userspace races with itself, that's not likely to yield well-
// defined results anyway so we choose to not care.
//
    pub syncobj: *mut drm_syncobj,
//
// @vm: unique address space (GTT)
//
// In full-ppgtt mode, each context has its own address space ensuring
// complete separation of one client from all others.
//
// In other modes, this is a NULL pointer with the expectation that
// the caller uses the shared global GTT.
//
    pub vm: *mut i915_address_space,
//
// @pid: process id of creator
//
// Note that who created the context may not be the principle user,
// as the context may be shared across a local socket. However,
// that should only affect the default context, all contexts created
// explicitly by the client are expected to be isolated.
//
    pub pid: *mut pid,
// @link: place with &drm_i915_private.context_list
    pub link: list_head,
// @client: struct i915_drm_client
    pub client: *mut i915_drm_client,
// @client_link: for linking onto &i915_drm_client.ctx_list
    pub client_link: list_head,
//
// @ref: reference count
//
// A reference to a context is held by both the client who created it
// and on each request submitted to the hardware using the request
// (to ensure the hardware has access to the state until it has
// finished all pending writes). See i915_gem_context_get() and
// i915_gem_context_put() for access.
//
    pub ref: kref,
//
// @release_work:
//
// Work item for deferred cleanup, since i915_gem_context_put() tends to
// be called from hardirq context.
//
// FIXME: The only real reason for this is &i915_gem_engines.fence, all
// other callers are from process context and need at most some mild
// shuffling to pull the i915_gem_context_put() call out of a spinlock.
//
    pub release_work: work_struct,
//
// @rcu: rcu_head for deferred freeing.
//
    pub rcu: rcu_head,
//
// @user_flags: small set of booleans controlled by the user
//
    pub user_flags: c_ulong,
pub const UCONTEXT_NO_ERROR_CAPTURE: c_int = 1;
pub const UCONTEXT_BANNABLE: c_int = 2;
pub const UCONTEXT_RECOVERABLE: c_int = 3;
pub const UCONTEXT_PERSISTENCE: c_int = 4;
pub const UCONTEXT_LOW_LATENCY: c_int = 5;
//
// @flags: small set of booleans
//
    pub flags: c_ulong,
pub const CONTEXT_CLOSED: c_int = 0;
pub const CONTEXT_USER_ENGINES: c_int = 1;
//
// @uses_protected_content: context uses PXP-encrypted objects.
//
// This flag can only be set at ctx creation time and it's immutable for
// the lifetime of the context. See I915_CONTEXT_PARAM_PROTECTED_CONTENT
// in uapi/drm/i915_drm.h for more info on setting restrictions and
// expected behaviour of marked contexts.
//
    pub uses_protected_content: bool,
//
// @pxp_wakeref: wakeref to keep the device awake when PXP is in use
//
// PXP sessions are invalidated when the device is suspended, which in
// turns invalidates all contexts and objects using it. To keep the
// flow simple, we keep the device awake when contexts using PXP objects
// are in use. It is expected that the userspace application only uses
// PXP when the display is on, so taking a wakeref here shouldn't worsen
// our power metrics.
//
    pub pxp_wakeref: intel_wakeref_t,
// @mutex: guards everything that isn't engines or handles_vma
    pub mutex: mutex,
// @sched: scheduler parameters
    pub sched: i915_sched_attr,
// @guilty_count: How many times this context has caused a GPU hang.
    pub guilty_count: core::sync::atomic::AtomicI32,
//
// @active_count: How many times this context was active during a GPU
// hang, but did not cause it.
//
    pub active_count: core::sync::atomic::AtomicI32,
//
// @hang_timestamp: The last time(s) this context caused a GPU hang
//
    pub hang_timestamp: [c_ulong; 2],
// @remap_slice: Bitmask of cache lines that need remapping
    pub remap_slice: u8,
//
// @handles_vma: rbtree to look up our context specific obj/vma for
// the user handle. (user handles are per fd, but the binding is
// per vm, which may be one per context or shared with the global GTT)
//
    pub handles_vma: radix_tree_root,
// @lut_mutex: Locks handles_vma
    pub lut_mutex: mutex,
//
// @name: arbitrary name, used for user debug
//
// A name is constructed for the context from the creator's process
// name, pid and user handle in order to uniquely identify the
// context in messages.
//
    pub 8]: char name[TASK_COMM_LEN +,
// @stale: tracks stale engines to be destroyed
// @stale.lock: guards engines
    pub lock: spinlock_t,
// @stale.engines: list of stale engines
    pub engines: list_head,
    pub stale: },
}
