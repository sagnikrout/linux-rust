//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/drm_atomic.h
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
// Copyright (C) 2014 Red Hat
// Copyright (C) 2014 Intel Corp.
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
// Authors:
// Rob Clark <robdclark@gmail.com>
// Daniel Vetter <daniel.vetter@ffwll.ch>
//

//
// struct drm_crtc_commit - track modeset commits on a CRTC
//
// This structure is used to track pending modeset changes and atomic commit on
// a per-CRTC basis. Since updating the list should never block, this structure
// is reference counted to allow waiters to safely wait on an event to complete,
// without holding any locks.
//
// It has 3 different events in total to allow a fine-grained synchronization
// between outstanding updates::
//
// atomic commit thread			hardware
//
// write new state into hardware	---->	...
// signal hw_done
// switch to new state on next
// ...					v/hblank
//
// wait for buffers to show up		...
//
// ...					send completion irq
// irq handler signals flip_done
// cleanup old buffers
//
// signal cleanup_done
//
// wait for flip_done		<----
// clean up atomic state
//
// The important bit to know is that &cleanup_done is the terminal event, but the
// ordering between &flip_done and &hw_done is entirely up to the specific driver
// and modeset state change.
//
// For an implementation of how to use this look at
// drm_atomic_helper_setup_commit() from the atomic helper library.
//
// See also drm_crtc_commit_wait().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_crtc_commit {
//
// @crtc:
//
// DRM CRTC for this commit.
//
    pub crtc: *mut drm_crtc,
//
// @ref:
//
// Reference count for this structure. Needed to allow blocking on
// completions without the risk of the completion disappearing
// meanwhile.
//
    pub ref: kref,
//
// @flip_done:
//
// Will be signaled when the hardware has flipped to the new set of
// buffers. Signals at the same time as when the drm event for this
// commit is sent to userspace, or when an out-fence is singalled. Note
// that for most hardware, in most cases this happens after @hw_done is
// signalled.
//
// Completion of this stage is signalled implicitly by calling
// drm_crtc_send_vblank_event() on &drm_crtc_state.event.
//
    pub flip_done: completion,
//
// @hw_done:
//
// Will be signalled when all hw register changes for this commit have
// been written out. Especially when disabling a pipe this can be much
// later than @flip_done, since that can signal already when the
// screen goes black, whereas to fully shut down a pipe more register
// I/O is required.
//
// Note that this does not need to include separately reference-counted
// resources like backing storage buffer pinning, or runtime pm
// management.
//
// Drivers should call drm_atomic_helper_commit_hw_done() to signal
// completion of this stage.
//
    pub hw_done: completion,
//
// @cleanup_done:
//
// Will be signalled after old buffers have been cleaned up by calling
// drm_atomic_helper_cleanup_planes(). Since this can only happen after
// a vblank wait completed it might be a bit later. This completion is
// useful to throttle updates and avoid hardware updates getting ahead
// of the buffer cleanup too much.
//
// Drivers should call drm_atomic_helper_commit_cleanup_done() to signal
// completion of this stage.
//
    pub cleanup_done: completion,
//
// @commit_entry:
//
// Entry on the per-CRTC &drm_crtc.commit_list. Protected by
// $drm_crtc.commit_lock.
//
    pub commit_entry: list_head,
//
// @event:
//
// &drm_pending_vblank_event pointer to clean up private events.
//
    pub event: *mut drm_pending_vblank_event,
//
// @abort_completion:
//
// A flag that's set after drm_atomic_helper_setup_commit() takes a
// second reference for the completion of $drm_crtc_state.event. It's
// used by the free code to remove the second reference if commit fails.
//
    pub abort_completion: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __drm_colorops_state {
    pub ptr: *mut drm_colorop,
    pub new_state: *mut *mut *mut drm_colorop_state state, old_state,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __drm_planes_state {
    pub ptr: *mut drm_plane,
//
// @state_to_destroy:
//
// Used to track the @drm_plane_state we will need to free when
// tearing down the associated &drm_atomic_commit in
// $drm_mode_config_funcs.atomic_state_clear or
// drm_atomic_commit_default_clear().
//
// Before a commit, and the call to
// drm_atomic_helper_swap_state() in particular, it points to
// the same state than @new_state. After a commit, it points to
// the same state than @old_state.
//
    pub state_to_destroy: *mut drm_plane_state,
    pub new_state: *mut *mut drm_plane_state old_state,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __drm_crtcs_state {
    pub ptr: *mut drm_crtc,
//
// @state_to_destroy:
//
// Used to track the @drm_crtc_state we will need to free when
// tearing down the associated &drm_atomic_commit in
// $drm_mode_config_funcs.atomic_state_clear or
// drm_atomic_commit_default_clear().
//
// Before a commit, and the call to
// drm_atomic_helper_swap_state() in particular, it points to
// the same state than @new_state. After a commit, it points to
// the same state than @old_state.
//
    pub state_to_destroy: *mut drm_crtc_state,
    pub new_state: *mut *mut drm_crtc_state old_state,,
//
// @commit:
//
// A reference to the CRTC commit object that is kept for use by
// drm_atomic_helper_wait_for_flip_done() after
// drm_atomic_helper_commit_hw_done() is called. This ensures that a
// concurrent commit won't free a commit object that is still in use.
//
    pub commit: *mut drm_crtc_commit,
    pub out_fence_ptr: *mut s32 __user,
    pub last_vblank_count: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __drm_connnectors_state {
    pub ptr: *mut drm_connector,
//
// @state_to_destroy:
//
// Used to track the @drm_connector_state we will need to free
// when tearing down the associated &drm_atomic_commit in
// $drm_mode_config_funcs.atomic_state_clear or
// drm_atomic_commit_default_clear().
//
// Before a commit, and the call to
// drm_atomic_helper_swap_state() in particular, it points to
// the same state than @new_state. After a commit, it points to
// the same state than @old_state.
//
    pub state_to_destroy: *mut drm_connector_state,
    pub new_state: *mut *mut drm_connector_state old_state,,
//
// @out_fence_ptr:
//
// User-provided pointer which the kernel uses to return a sync_file
// file descriptor. Used by writeback connectors to signal completion of
// the writeback.
//
    pub out_fence_ptr: *mut s32 __user,
}

//
// struct drm_private_state_funcs - atomic state functions for private objects
//
// These hooks are used by atomic helpers to create, swap and destroy states of
// private objects. The structure itself is used as a vtable to identify the
// associated private object type. Each private object type that needs to be
// added to the atomic states is expected to have an implementation of these
// hooks and pass a pointer to its drm_private_state_funcs struct to
// drm_atomic_get_private_obj_state().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_private_state_funcs {
//
// @atomic_create_state:
//
// Allocates a pristine, initialized, state for the private
// object and returns it. This callback must have no side
// effects: in particular, the returned state must not be
// assigned to the object's state pointer and it must not affect
// the hardware state.
//
// RETURNS:
//
// A new, pristine, private state instance or an error pointer
// on failure.
//
    pub obj): *mut *mut *mut drm_private_state (atomic_create_state)(drm_private_obj,
//
// @atomic_duplicate_state:
//
// Duplicate the current state of the private object and return it. It
// is an error to call this before obj->state has been initialized.
//
// RETURNS:
//
// Duplicated atomic state or NULL when obj->state is not
// initialized or allocation failed.
//
    pub obj): *mut *mut *mut drm_private_state (atomic_duplicate_state)(drm_private_obj,
//
// @atomic_destroy_state:
//
// Frees the private object state created with @atomic_duplicate_state.
//
    pub state): *mut drm_private_state,
//
// @atomic_print_state:
//
// If driver subclasses &struct drm_private_state, it should implement
// this optional hook for printing additional driver specific state.
//
// Do not call this directly, use drm_atomic_private_obj_print_state()
// instead.
//
    pub state): *const drm_private_state,
}

//
// struct drm_private_obj - base struct for driver private atomic object
//
// A driver private object is initialized by calling
// drm_atomic_private_obj_init() and cleaned up by calling
// drm_atomic_private_obj_fini().
//
// Currently only tracks the state update functions and the opaque driver
// private state itself, but in the future might also track which
// &drm_modeset_lock is required to duplicate and update this object's state.
//
// All private objects must be initialized before the DRM device they are
// attached to is registered to the DRM subsystem (call to drm_dev_register())
// and should stay around until this DRM device is unregistered (call to
// drm_dev_unregister()). In other words, private objects lifetime is tied
// to the DRM device lifetime. This implies that:
//
// 1/ all calls to drm_atomic_private_obj_init() must be done before calling
// drm_dev_register()
// 2/ all calls to drm_atomic_private_obj_fini() must be done after calling
// drm_dev_unregister()
//
// If that private object is used to store a state shared by multiple
// CRTCs, proper care must be taken to ensure that non-blocking commits are
// properly ordered to avoid a use-after-free issue.
//
// Indeed, assuming a sequence of two non-blocking &drm_atomic_commit on two
// different &drm_crtc using different &drm_plane and &drm_connector, so with no
// resources shared, there's no guarantee on which commit is going to happen
// first. However, the second &drm_atomic_commit will consider the first
// &drm_private_obj its old state, and will be in charge of freeing it whenever
// the second &drm_atomic_commit is done.
//
// If the first &drm_atomic_commit happens after it, it will consider its
// &drm_private_obj the new state and will be likely to access it, resulting in
// an access to a freed memory region. Drivers should store (and get a reference
// to) the &drm_crtc_commit structure in our private state in
// &drm_mode_config_helper_funcs.atomic_commit_setup, and then wait for that
// commit to complete as the first step of
// &drm_mode_config_helper_funcs.atomic_commit_tail, similar to
// drm_atomic_helper_wait_for_dependencies().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_private_obj {
//
// @dev: parent DRM device
//
    pub dev: *mut drm_device,
//
// @head: List entry used to attach a private object to a &drm_device
// (queued to &drm_mode_config.privobj_list).
//
    pub head: list_head,
//
// @lock: Modeset lock to protect the state object.
//
    pub lock: drm_modeset_lock,
//
// @state: Current atomic state for this driver private object.
//
    pub state: *mut drm_private_state,
//
// @funcs:
//
// Functions to manipulate the state of this driver private object, see
// &drm_private_state_funcs.
//
    pub funcs: *const drm_private_state_funcs,
}

//
// drm_for_each_privobj() - private object iterator
//
// @privobj: pointer to the current private object. Updated after each
// iteration
// @dev: the DRM device we want get private objects from
//
// Allows one to iterate over all private objects attached to @dev
//

//
// struct drm_private_state - base struct for driver private object state
//
// Currently only contains a backpointer to the overall atomic update,
// and the relevant private object but in the future also might hold
// synchronization information similar to e.g. &drm_crtc.commit.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_private_state {
//
// @state: backpointer to global drm_atomic_commit
//
    pub state: *mut drm_atomic_commit,
//
// @obj: backpointer to the private object
//
    pub obj: *mut drm_private_obj,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __drm_private_objs_state {
    pub ptr: *mut drm_private_obj,
//
// @state_to_destroy:
//
// Used to track the @drm_private_state we will need to free
// when tearing down the associated &drm_atomic_commit in
// $drm_mode_config_funcs.atomic_state_clear or
// drm_atomic_commit_default_clear().
//
// Before a commit, and the call to
// drm_atomic_helper_swap_state() in particular, it points to
// the same state than @new_state. After a commit, it points to
// the same state than @old_state.
//
    pub state_to_destroy: *mut drm_private_state,
    pub new_state: *mut *mut drm_private_state old_state,,
}

//
// struct drm_atomic_commit - Atomic commit structure
//
// This structure is the kernel counterpart of @drm_mode_atomic and represents
// an atomic commit that transitions from an old to a new display state. It
// contains all the objects affected by the atomic commit and both the new
// state structures and pointers to the old state structures for
// these.
//
// States are added to an atomic update by calling drm_atomic_get_crtc_state(),
// drm_atomic_get_plane_state(), drm_atomic_get_connector_state(), or for
// private state structures, drm_atomic_get_private_obj_state().
//
// NOTE: struct drm_atomic_commit first started as a single collection of
// entities state pointers (drm_plane_state, drm_crtc_state, etc.).
//
// At atomic_check time, you could get the state about to be committed
// from drm_atomic_commit, and the one currently running from the
// entities state pointer (drm_crtc.state, for example). After the call
// to drm_atomic_helper_swap_state(), the entities state pointer would
// contain the state previously checked, and the drm_atomic_commit
// structure the old state.
//
// Over time, and in order to avoid confusion, drm_atomic_commit has
// grown to have both the old state (ie, the state we replace) and the
// new state (ie, the state we want to apply). Those names are stable
// during the commit process, which makes it easier to reason about.
//
// You can still find some traces of that evolution through some hooks
// or callbacks taking a drm_atomic_commit parameter called names like
// "old_state". This doesn't necessarily mean that the previous
// drm_atomic_commit is passed, but rather that this used to be the state
// collection we were replacing after drm_atomic_helper_swap_state(),
// but the variable name was never updated.
//
// Some atomic operations implementations followed a similar process. We
// first started to pass the entity state only. However, it was pretty
// cumbersome for drivers, and especially CRTCs, to retrieve the states
// of other components. Thus, we switched to passing the whole
// drm_atomic_commit as a parameter to those operations. Similarly, the
// transition isn't complete yet, and one might still find atomic
// operations taking a drm_atomic_commit pointer, or a component state
// pointer. The former is the preferred form.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_atomic_commit {
//
// @ref:
//
// Count of all references to this update (will not be freed until zero).
//
    pub ref: kref,
//
// @dev: Parent DRM Device.
//
    pub dev: *mut drm_device,
//
// @allow_modeset:
//
// Allow full modeset. This is used by the ATOMIC IOCTL handler to
// implement the DRM_MODE_ATOMIC_ALLOW_MODESET flag. Drivers should
// generally not consult this flag, but instead look at the output of
// drm_atomic_crtc_needs_modeset(). The detailed rules are:
//
// - Drivers must not consult @allow_modeset in the atomic commit path.
// Use drm_atomic_crtc_needs_modeset() instead.
//
// - Drivers must consult @allow_modeset before adding unrelated struct
// drm_crtc_state to this commit by calling
// drm_atomic_get_crtc_state(). See also the warning in the
// documentation for that function.
//
// - Drivers must never change this flag, it is under the exclusive
// control of userspace.
//
// - Drivers may consult @allow_modeset in the atomic check path, if
// they have the choice between an optimal hardware configuration
// which requires a modeset, and a less optimal configuration which
// can be committed without a modeset. An example would be suboptimal
// scanout FIFO allocation resulting in increased idle power
// consumption. This allows userspace to avoid flickering and delays
// for the normal composition loop at reasonable cost.
//
    pub 1: bool allow_modeset :,
//
// @legacy_cursor_update:
//
// Hint to enforce legacy cursor IOCTL semantics.
//
// WARNING: This is thoroughly broken and pretty much impossible to
// implement correctly. Drivers must ignore this and should instead
// implement &drm_plane_helper_funcs.atomic_async_check and
// &drm_plane_helper_funcs.atomic_async_commit hooks. New users of this
// flag are not allowed.
//
    pub 1: bool legacy_cursor_update :,
//
// @async_update: hint for asynchronous plane update
//
    pub 1: bool async_update :,
//
// @duplicated:
//
// Indicates whether or not this atomic state was duplicated using
// drm_atomic_helper_duplicate_state(). Drivers and atomic helpers
// should use this to fixup normal  inconsistencies in duplicated
// states.
//
    pub 1: bool duplicated :,
//
// @checked:
//
// Indicates the state has been checked and thus must no longer
// be mutated. For internal use only, do not consult from drivers.
//
    pub 1: bool checked :,
//
// @plane_color_pipeline:
//
// Indicates whether this atomic state originated with a client that
// set the DRM_CLIENT_CAP_PLANE_COLOR_PIPELINE.
//
// Drivers and helper functions should use this to ignore legacy
// properties that are incompatible with the drm_plane COLOR_PIPELINE
// behavior, such as:
//
// - COLOR_RANGE
// - COLOR_ENCODING
//
// or any other driver-specific properties that might affect pixel
// values.
//
    pub 1: bool plane_color_pipeline :,
//
// @colorops:
//
// Pointer to array of @drm_colorop and @drm_colorop_state part of this
// update.
//
    pub colorops: *mut __drm_colorops_state,
//
// @planes:
//
// Pointer to array of @drm_plane and @drm_plane_state part of this
// update.
//
    pub planes: *mut __drm_planes_state,
//
// @crtcs:
//
// Pointer to array of @drm_crtc and @drm_crtc_state part of this
// update.
//
    pub crtcs: *mut __drm_crtcs_state,
//
// @num_connector: size of the @connectors array
//
    pub num_connector: c_int,
//
// @connectors:
//
// Pointer to array of @drm_connector and @drm_connector_state part of
// this update.
//
    pub connectors: *mut __drm_connnectors_state,
//
// @num_private_objs: size of the @private_objs array
//
    pub num_private_objs: c_int,
//
// @private_objs:
//
// Pointer to array of @drm_private_obj and @drm_private_obj_state part
// of this update.
//
    pub private_objs: *mut __drm_private_objs_state,
//
// @acquire_ctx: acquire context for this atomic modeset state update
//
    pub acquire_ctx: *mut drm_modeset_acquire_ctx,
//
// @fake_commit:
//
// Used for signaling unbound planes/connectors.
// When a connector or plane is not bound to any CRTC, it's still important
// to preserve linearity to prevent the atomic states from being freed too early.
//
// This commit (if set) is not bound to any CRTC, but will be completed when
// drm_atomic_helper_commit_hw_done() is called.
//
    pub fake_commit: *mut drm_crtc_commit,
//
// @commit_work:
//
// Work item which can be used by the driver or helpers to execute the
// commit without blocking.
//
    pub commit_work: work_struct,
}

extern "C" {
    pub fn __drm_crtc_commit_free(kref: *mut kref);
}
//
// drm_crtc_commit_get - acquire a reference to the CRTC commit
// @commit: CRTC commit
//
// Increases the reference of @commit.
//
// Returns:
// The pointer to @commit, with reference increased.
//
// drm_crtc_commit_put - release a reference to the CRTC commmit
// @commit: CRTC commit
//
// This releases a reference to @commit which is freed after removing the
// final reference. No locking required and callable from any context.
//
extern "C" {
    pub fn drm_crtc_commit_wait(commit: *mut drm_crtc_commit) -> c_int;
}
extern "C" {
    pub fn drm_atomic_commit_clear(state: *mut drm_atomic_commit);
}
//
// drm_atomic_commit_get - acquire a reference to the atomic state
// @state: The atomic state
//
// Returns a new reference to the @state
//
extern "C" {
    pub fn __drm_atomic_commit_free(ref: *mut kref);
}
//
// drm_atomic_commit_put - release a reference to the atomic state
// @state: The atomic state
//
// This releases a reference to @state which is freed after removing the
// final reference. No locking required and callable from any context.
//
extern "C" {
    pub fn drm_atomic_commit_default_clear(state: *mut drm_atomic_commit);
}
extern "C" {
    pub fn drm_atomic_commit_default_release(state: *mut drm_atomic_commit);
}
extern "C" {
    pub fn drm_atomic_private_obj_fini(obj: *mut drm_private_obj);
}
//
// drm_atomic_get_old_crtc_state - get old CRTC state, if it exists
// @state: global atomic state object
// @crtc: CRTC to grab
//
// This function returns the old CRTC state for the given CRTC, or
// NULL if the CRTC is not part of the global atomic state.
//
// drm_atomic_get_new_crtc_state - get new CRTC state, if it exists
// @state: global atomic state object
// @crtc: CRTC to grab
//
// This function returns the new CRTC state for the given CRTC, or
// NULL if the CRTC is not part of the global atomic state.
//
// drm_atomic_get_old_plane_state - get plane state, if it exists
// @state: global atomic state object
// @plane: plane to grab
//
// This function returns the old plane state for the given plane, or
// NULL if the plane is not part of the global atomic state.
//
// drm_atomic_get_new_plane_state - get plane state, if it exists
// @state: global atomic state object
// @plane: plane to grab
//
// This function returns the new plane state for the given plane, or
// NULL if the plane is not part of the global atomic state.
//
// drm_atomic_get_old_connector_state - get connector state, if it exists
// @state: global atomic state object
// @connector: connector to grab
//
// This function returns the old connector state for the given connector,
// or NULL if the connector is not part of the global atomic state.
//
// drm_atomic_get_new_connector_state - get connector state, if it exists
// @state: global atomic state object
// @connector: connector to grab
//
// This function returns the new connector state for the given connector,
// or NULL if the connector is not part of the global atomic state.
//
// __drm_atomic_get_current_plane_state - get current plane state
// @state: global atomic state object
// @plane: plane to grab
//
// This function returns the plane state for the given plane, either the
// new plane state from @state, or if the plane isn't part of the atomic
// state update, from @plane. This is useful in atomic check callbacks,
// when drivers need to peek at, but not change, state of other planes,
// since it avoids threading an error code back up the call chain.
//
// WARNING:
//
// Note that this function is in general unsafe since it doesn't check for the
// required locking for access state structures. Drivers must ensure that it is
// safe to access the returned state structure through other means. One common
// example is when planes are fixed to a single CRTC, and the driver knows that
// the CRTC lock is held already. In that case holding the CRTC lock gives a
// read-lock on all planes connected to that CRTC. But if planes can be
// reassigned things get more tricky. In that case it's better to use
// drm_atomic_get_plane_state and wire up full error handling.
//
// Returns:
//
// Read-only pointer to the current plane state.
//
// If the plane isn't part of the state, fallback to the currently active one.
//
extern "C" {
    pub fn drm_atomic_check_only(state: *mut drm_atomic_commit) -> int __must_check;
}
extern "C" {
    pub fn drm_atomic_commit(state: *mut drm_atomic_commit) -> int __must_check;
}
extern "C" {
    pub fn drm_atomic_nonblocking_commit(state: *mut drm_atomic_commit) -> int __must_check;
}
extern "C" {
    pub fn drm_state_dump(dev: *mut drm_device, p: *mut drm_printer);
}
//
// for_each_oldnew_connector_in_state - iterate over all connectors in an atomic update
// @__state: &struct drm_atomic_commit pointer
// @connector: &struct drm_connector iteration cursor
// @old_connector_state: &struct drm_connector_state iteration cursor for the
// old state
// @new_connector_state: &struct drm_connector_state iteration cursor for the
// new state
// @__i: int iteration cursor, for macro-internal use
//
// This iterates over all connectors in an atomic update, tracking both old and
// new state. This is useful in places where the state delta needs to be
// considered, for example in atomic check functions.
//

//
// for_each_old_connector_in_state - iterate over all connectors in an atomic update
// @__state: &struct drm_atomic_commit pointer
// @connector: &struct drm_connector iteration cursor
// @old_connector_state: &struct drm_connector_state iteration cursor for the
// old state
// @__i: int iteration cursor, for macro-internal use
//
// This iterates over all connectors in an atomic update, tracking only the old
// state. This is useful in disable functions, where we need the old state the
// hardware is still in.
//

//
// for_each_new_connector_in_state - iterate over all connectors in an atomic update
// @__state: &struct drm_atomic_commit pointer
// @connector: &struct drm_connector iteration cursor
// @new_connector_state: &struct drm_connector_state iteration cursor for the
// new state
// @__i: int iteration cursor, for macro-internal use
//
// This iterates over all connectors in an atomic update, tracking only the new
// state. This is useful in enable functions, where we need the new state the
// hardware should be in when the atomic commit operation has completed.
//

//
// for_each_oldnew_crtc_in_state - iterate over all CRTCs in an atomic update
// @__state: &struct drm_atomic_commit pointer
// @crtc: &struct drm_crtc iteration cursor
// @old_crtc_state: &struct drm_crtc_state iteration cursor for the old state
// @new_crtc_state: &struct drm_crtc_state iteration cursor for the new state
// @__i: int iteration cursor, for macro-internal use
//
// This iterates over all CRTCs in an atomic update, tracking both old and
// new state. This is useful in places where the state delta needs to be
// considered, for example in atomic check functions.
//

//
// for_each_old_crtc_in_state - iterate over all CRTCs in an atomic update
// @__state: &struct drm_atomic_commit pointer
// @crtc: &struct drm_crtc iteration cursor
// @old_crtc_state: &struct drm_crtc_state iteration cursor for the old state
// @__i: int iteration cursor, for macro-internal use
//
// This iterates over all CRTCs in an atomic update, tracking only the old
// state. This is useful in disable functions, where we need the old state the
// hardware is still in.
//

//
// for_each_new_crtc_in_state - iterate over all CRTCs in an atomic update
// @__state: &struct drm_atomic_commit pointer
// @crtc: &struct drm_crtc iteration cursor
// @new_crtc_state: &struct drm_crtc_state iteration cursor for the new state
// @__i: int iteration cursor, for macro-internal use
//
// This iterates over all CRTCs in an atomic update, tracking only the new
// state. This is useful in enable functions, where we need the new state the
// hardware should be in when the atomic commit operation has completed.
//

//
// for_each_oldnew_colorop_in_state - iterate over all colorops in an atomic update
// @__state: &struct drm_atomic_commit pointer
// @colorop: &struct drm_colorop iteration cursor
// @old_colorop_state: &struct drm_colorop_state iteration cursor for the old state
// @new_colorop_state: &struct drm_colorop_state iteration cursor for the new state
// @__i: int iteration cursor, for macro-internal use
//
// This iterates over all colorops in an atomic update, tracking both old and
// new state. This is useful in places where the state delta needs to be
// considered, for example in atomic check functions.
//

//
// for_each_new_colorop_in_state - iterate over all colorops in an atomic update
// @__state: &struct drm_atomic_commit pointer
// @colorop: &struct drm_colorop iteration cursor
// @new_colorop_state: &struct drm_colorop_state iteration cursor for the new state
// @__i: int iteration cursor, for macro-internal use
//
// This iterates over all colorops in an atomic update, tracking new state. This is
// useful in places where the state delta needs to be considered, for example in
// atomic check functions.
//

//
// for_each_oldnew_plane_in_state - iterate over all planes in an atomic update
// @__state: &struct drm_atomic_commit pointer
// @plane: &struct drm_plane iteration cursor
// @old_plane_state: &struct drm_plane_state iteration cursor for the old state
// @new_plane_state: &struct drm_plane_state iteration cursor for the new state
// @__i: int iteration cursor, for macro-internal use
//
// This iterates over all planes in an atomic update, tracking both old and
// new state. This is useful in places where the state delta needs to be
// considered, for example in atomic check functions.
//

//
// for_each_oldnew_plane_in_state_reverse - iterate over all planes in an atomic
// update in reverse order
// @__state: &struct drm_atomic_commit pointer
// @plane: &struct drm_plane iteration cursor
// @old_plane_state: &struct drm_plane_state iteration cursor for the old state
// @new_plane_state: &struct drm_plane_state iteration cursor for the new state
// @__i: int iteration cursor, for macro-internal use
//
// This iterates over all planes in an atomic update in reverse order,
// tracking both old and  new state. This is useful in places where the
// state delta needs to be considered, for example in atomic check functions.
//

//
// for_each_new_plane_in_state_reverse - other than only tracking new state,
// it's the same as for_each_oldnew_plane_in_state_reverse
// @__state: &struct drm_atomic_commit pointer
// @plane: &struct drm_plane iteration cursor
// @new_plane_state: &struct drm_plane_state iteration cursor for the new state
// @__i: int iteration cursor, for macro-internal use
//

//
// for_each_old_plane_in_state - iterate over all planes in an atomic update
// @__state: &struct drm_atomic_commit pointer
// @plane: &struct drm_plane iteration cursor
// @old_plane_state: &struct drm_plane_state iteration cursor for the old state
// @__i: int iteration cursor, for macro-internal use
//
// This iterates over all planes in an atomic update, tracking only the old
// state. This is useful in disable functions, where we need the old state the
// hardware is still in.
//

//
// for_each_new_plane_in_state - iterate over all planes in an atomic update
// @__state: &struct drm_atomic_commit pointer
// @plane: &struct drm_plane iteration cursor
// @new_plane_state: &struct drm_plane_state iteration cursor for the new state
// @__i: int iteration cursor, for macro-internal use
//
// This iterates over all planes in an atomic update, tracking only the new
// state. This is useful in enable functions, where we need the new state the
// hardware should be in when the atomic commit operation has completed.
//

//
// for_each_oldnew_private_obj_in_state - iterate over all private objects in an atomic update
// @__state: &struct drm_atomic_commit pointer
// @obj: &struct drm_private_obj iteration cursor
// @old_obj_state: &struct drm_private_state iteration cursor for the old state
// @new_obj_state: &struct drm_private_state iteration cursor for the new state
// @__i: int iteration cursor, for macro-internal use
//
// This iterates over all private objects in an atomic update, tracking both
// old and new state. This is useful in places where the state delta needs
// to be considered, for example in atomic check functions.
//

//
// for_each_old_private_obj_in_state - iterate over all private objects in an atomic update
// @__state: &struct drm_atomic_commit pointer
// @obj: &struct drm_private_obj iteration cursor
// @old_obj_state: &struct drm_private_state iteration cursor for the old state
// @__i: int iteration cursor, for macro-internal use
//
// This iterates over all private objects in an atomic update, tracking only
// the old state. This is useful in disable functions, where we need the old
// state the hardware is still in.
//

//
// for_each_new_private_obj_in_state - iterate over all private objects in an atomic update
// @__state: &struct drm_atomic_commit pointer
// @obj: &struct drm_private_obj iteration cursor
// @new_obj_state: &struct drm_private_state iteration cursor for the new state
// @__i: int iteration cursor, for macro-internal use
//
// This iterates over all private objects in an atomic update, tracking only
// the new state. This is useful in enable functions, where we need the new state the
// hardware should be in when the atomic commit operation has completed.
//

//
// drm_atomic_crtc_needs_modeset - compute combined modeset need
// @state: &drm_crtc_state for the CRTC
//
// To give drivers flexibility &struct drm_crtc_state has 3 booleans to track
// whether the state CRTC changed enough to need a full modeset cycle:
// mode_changed, active_changed and connectors_changed. This helper simply
// combines these three to compute the overall need for a modeset for @state.
//
// The atomic helper code sets these booleans, but drivers can and should
// change them appropriately to accurately represent whether a modeset is
// really needed. In general, drivers should avoid full modesets whenever
// possible.
//
// For example if the CRTC mode has changed, and the hardware is able to enact
// the requested mode change without going through a full modeset, the driver
// should clear mode_changed in its &drm_mode_config_funcs.atomic_check
// implementation.
//
// drm_atomic_crtc_effectively_active - compute whether CRTC is actually active
// @state: &drm_crtc_state for the CRTC
//
// When in self refresh mode, the crtc_state->active value will be false, since
// the CRTC is off. However in some cases we're interested in whether the CRTC
// is active, or effectively active (ie: it's connected to an active display).
// In these cases, use this function instead of just checking active.
//
// struct drm_bus_cfg - bus configuration
//
// This structure stores the configuration of a physical bus between two
// components in an output pipeline, usually between two bridges, an encoder
// and a bridge, or a bridge and a connector.
//
// The bus configuration is stored in &drm_bridge_state separately for the
// input and output buses, as seen from the point of view of each bridge. The
// bus configuration of a bridge output is usually identical to the
// configuration of the next bridge's input, but may differ if the signals are
// modified between the two bridges, for instance by an inverter on the board.
// The input and output configurations of a bridge may differ if the bridge
// modifies the signals internally, for instance by performing format
// conversion, or modifying signals polarities.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_bus_cfg {
//
// @format: format used on this bus (one of the MEDIA_BUS_FMT_* format)
//
// This field should not be directly modified by drivers
// (drm_atomic_bridge_chain_select_bus_fmts() takes care of the bus
// format negotiation).
//
    pub format: u32,
//
// @flags: DRM_BUS_* flags used on this bus
//
    pub flags: u32,
}

//
// struct drm_bridge_state - Atomic bridge state object
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_bridge_state {
//
// @base: inherit from &drm_private_state
//
    pub base: drm_private_state,
//
// @bridge: the bridge this state refers to
//
    pub bridge: *mut drm_bridge,
//
// @input_bus_cfg: input bus configuration
//
    pub input_bus_cfg: drm_bus_cfg,
//
// @output_bus_cfg: output bus configuration
//
    pub output_bus_cfg: drm_bus_cfg,
}

extern "C" {
    pub fn container_of(_arg: priv, drm_bridge_state: struct, _arg: base) -> return;
}
