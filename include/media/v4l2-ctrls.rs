//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/v4l2-ctrls.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// V4L2 controls support header.
//
// Copyright (C) 2010  Hans Verkuil <hverkuil@kernel.org>
//

// forward references
//
// union v4l2_ctrl_ptr - A pointer to a control value.
// @p_s32:			Pointer to a 32-bit signed value.
// @p_s64:			Pointer to a 64-bit signed value.
// @p_u8:			Pointer to a 8-bit unsigned value.
// @p_u16:			Pointer to a 16-bit unsigned value.
// @p_u32:			Pointer to a 32-bit unsigned value.
// @p_char:			Pointer to a string.
// @p_mpeg2_sequence:		Pointer to a MPEG2 sequence structure.
// @p_mpeg2_picture:		Pointer to a MPEG2 picture structure.
// @p_mpeg2_quantisation:	Pointer to a MPEG2 quantisation data structure.
// @p_fwht_params:		Pointer to a FWHT stateless parameters structure.
// @p_h264_sps:			Pointer to a struct v4l2_ctrl_h264_sps.
// @p_h264_pps:			Pointer to a struct v4l2_ctrl_h264_pps.
// @p_h264_scaling_matrix:	Pointer to a struct v4l2_ctrl_h264_scaling_matrix.
// @p_h264_slice_params:	Pointer to a struct v4l2_ctrl_h264_slice_params.
// @p_h264_decode_params:	Pointer to a struct v4l2_ctrl_h264_decode_params.
// @p_h264_pred_weights:	Pointer to a struct v4l2_ctrl_h264_pred_weights.
// @p_vp8_frame:		Pointer to a VP8 frame params structure.
// @p_vp9_compressed_hdr_probs:	Pointer to a VP9 frame compressed header probs structure.
// @p_vp9_frame:		Pointer to a VP9 frame params structure.
// @p_hevc_sps:			Pointer to an HEVC sequence parameter set structure.
// @p_hevc_pps:			Pointer to an HEVC picture parameter set structure.
// @p_hevc_slice_params:	Pointer to an HEVC slice parameters structure.
// @p_hdr10_cll:		Pointer to an HDR10 Content Light Level structure.
// @p_hdr10_mastering:		Pointer to an HDR10 Mastering Display structure.
// @p_area:			Pointer to an area.
// @p_av1_sequence:		Pointer to an AV1 sequence structure.
// @p_av1_tile_group_entry:	Pointer to an AV1 tile group entry structure.
// @p_av1_frame:		Pointer to an AV1 frame structure.
// @p_av1_film_grain:		Pointer to an AV1 film grain structure.
// @p_rect:			Pointer to a rectangle.
// @p:				Pointer to a compound value.
// @p_const:			Pointer to a constant compound value.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union v4l2_ctrl_ptr {
    pub p_s32: *mut i32,
    pub p_s64: *mut i64,
    pub p_u8: *mut u8,
    pub p_u16: *mut u16,
    pub p_u32: *mut u32,
    pub p_char: *mut c_char,
    pub p_mpeg2_sequence: *mut v4l2_ctrl_mpeg2_sequence,
    pub p_mpeg2_picture: *mut v4l2_ctrl_mpeg2_picture,
    pub p_mpeg2_quantisation: *mut v4l2_ctrl_mpeg2_quantisation,
    pub p_fwht_params: *mut v4l2_ctrl_fwht_params,
    pub p_h264_sps: *mut v4l2_ctrl_h264_sps,
    pub p_h264_pps: *mut v4l2_ctrl_h264_pps,
    pub p_h264_scaling_matrix: *mut v4l2_ctrl_h264_scaling_matrix,
    pub p_h264_slice_params: *mut v4l2_ctrl_h264_slice_params,
    pub p_h264_decode_params: *mut v4l2_ctrl_h264_decode_params,
    pub p_h264_pred_weights: *mut v4l2_ctrl_h264_pred_weights,
    pub p_vp8_frame: *mut v4l2_ctrl_vp8_frame,
    pub p_hevc_sps: *mut v4l2_ctrl_hevc_sps,
    pub p_hevc_pps: *mut v4l2_ctrl_hevc_pps,
    pub p_hevc_slice_params: *mut v4l2_ctrl_hevc_slice_params,
    pub p_vp9_compressed_hdr_probs: *mut v4l2_ctrl_vp9_compressed_hdr,
    pub p_vp9_frame: *mut v4l2_ctrl_vp9_frame,
    pub p_hdr10_cll: *mut v4l2_ctrl_hdr10_cll_info,
    pub p_hdr10_mastering: *mut v4l2_ctrl_hdr10_mastering_display,
    pub p_area: *mut v4l2_area,
    pub p_av1_sequence: *mut v4l2_ctrl_av1_sequence,
    pub p_av1_tile_group_entry: *mut v4l2_ctrl_av1_tile_group_entry,
    pub p_av1_frame: *mut v4l2_ctrl_av1_frame,
    pub p_av1_film_grain: *mut v4l2_ctrl_av1_film_grain,
    pub p_rect: *mut v4l2_rect,
    pub p: *mut c_void,
    pub p_const: *const c_void,
}

//
// v4l2_ctrl_ptr_create() - Helper function to return a v4l2_ctrl_ptr from a
// void pointer
// @ptr:	The void pointer
//
// struct v4l2_ctrl_ops - The control operations that the driver has to provide.
//
// @g_volatile_ctrl: Get a new value for this control. Generally only relevant
// for volatile (and usually read-only) controls such as a control
// that returns the current signal strength which changes
// continuously.
// If not set, then the currently cached value will be returned.
// @try_ctrl:	Test whether the control's value is valid. Only relevant when
// the usual min/max/step checks are not sufficient.
// @s_ctrl:	Actually set the new control value. s_ctrl is compulsory. The
// ctrl->handler->lock is held when these ops are called, so no
// one else can access controls owned by that handler.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_ctrl_ops {
    pub ctrl): *mut *mut int (g_volatile_ctrl)(struct v4l2_ctrl,
    pub ctrl): *mut *mut int (try_ctrl)(struct v4l2_ctrl,
    pub ctrl): *mut *mut int (s_ctrl)(struct v4l2_ctrl,
}

//
// struct v4l2_ctrl_type_ops - The control type operations that the driver
// has to provide.
//
// @equal: return true if all ctrl->elems array elements are equal.
// @init: initialize the value for array elements from from_idx to ctrl->elems.
// @minimum: set the value to the minimum value of the control.
// @maximum: set the value to the maximum value of the control.
// @log: log the value.
// @validate: validate the value for ctrl->new_elems array elements.
// Return 0 on success and a negative value otherwise.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_ctrl_type_ops {
    pub ptr2): v4l2_ctrl_ptr ptr1, v4l2_ctrl_ptr,
    pub ptr): v4l2_ctrl_ptr,
    pub ptr): v4l2_ctrl_ptr,
    pub ptr): v4l2_ctrl_ptr,
    pub ctrl): *const *const void (log)(struct v4l2_ctrl,
    pub ptr): *const *const *const int (validate)(struct v4l2_ctrl ctrl, union v4l2_ctrl_ptr,
}

//
// typedef v4l2_ctrl_notify_fnc - typedef for a notify argument with a function
// that should be called when a control value has changed.
//
// @ctrl: pointer to struct &v4l2_ctrl
// @priv: control private data
//
// This typedef definition is used as an argument to v4l2_ctrl_notify()
// and as an argument at struct &v4l2_ctrl_handler.
//
extern "C" {
    pub fn void(ctrl: *mut *mut v4l2_ctrl_notify_fnc)(struct v4l2_ctrl, priv: *mut c_void) -> typedef;
}
//
// struct v4l2_ctrl - The control structure.
//
// @node:	The list node.
// @ev_subs:	The list of control event subscriptions.
// @handler:	The handler that owns the control.
// @cluster:	Point to start of cluster array.
// @ncontrols:	Number of controls in cluster array.
// @done:	Internal flag: set for each processed control.
// @is_new:	Set when the user specified a new value for this control. It
// is also set when called from v4l2_ctrl_handler_setup(). Drivers
// should never set this flag.
// @has_changed: Set when the current value differs from the new value. Drivers
// should never use this flag.
// @is_private: If set, then this control is private to its handler and it
// will not be added to any other handlers. Drivers can set
// this flag.
// @is_auto:   If set, then this control selects whether the other cluster
// members are in 'automatic' mode or 'manual' mode. This is
// used for autogain/gain type clusters. Drivers should never
// set this flag directly.
// @is_int:    If set, then this control has a simple integer value (i.e. it
// uses ctrl->val).
// @is_string: If set, then this control has type %V4L2_CTRL_TYPE_STRING.
// @is_ptr:	If set, then this control is an array and/or has type >=
// %V4L2_CTRL_COMPOUND_TYPES
// and/or has type %V4L2_CTRL_TYPE_STRING. In other words, &struct
// v4l2_ext_control uses field p to point to the data.
// @is_array: If set, then this control contains an N-dimensional array.
// @is_dyn_array: If set, then this control contains a dynamically sized 1-dimensional array.
// If this is set, then @is_array is also set.
// @has_volatiles: If set, then one or more members of the cluster are volatile.
// Drivers should never touch this flag.
// @call_notify: If set, then call the handler's notify function whenever the
// control's value changes.
// @manual_mode_value: If the is_auto flag is set, then this is the value
// of the auto control that determines if that control is in
// manual mode. So if the value of the auto control equals this
// value, then the whole cluster is in manual mode. Drivers should
// never set this flag directly.
// @ops:	The control ops.
// @type_ops:	The control type ops.
// @id:	The control ID.
// @name:	The control name.
// @type:	The control type.
// @minimum:	The control's minimum value.
// @maximum:	The control's maximum value.
// @default_value: The control's default value.
// @step:	The control's step value for non-menu controls.
// @elems:	The number of elements in the N-dimensional array.
// @elem_size:	The size in bytes of the control.
// @new_elems:	The number of elements in p_new. This is the same as @elems,
// except for dynamic arrays. In that case it is in the range of
// 1 to @p_array_alloc_elems.
// @dims:	The size of each dimension.
// @nr_of_dims:The number of dimensions in @dims.
// @menu_skip_mask: The control's skip mask for menu controls. This makes it
// easy to skip menu items that are not valid. If bit X is set,
// then menu item X is skipped. Of course, this only works for
// menus with <= 32 menu items. There are no menus that come
// close to that number, so this is OK. Should we ever need more,
// then this will have to be extended to a u64 or a bit array.
// @qmenu:	A const char * array for all menu items. Array entries that are
// empty strings ("") correspond to non-existing menu items (this
// is in addition to the menu_skip_mask above). The last entry
// must be NULL.
// Used only if the @type is %V4L2_CTRL_TYPE_MENU.
// @qmenu_int:	A 64-bit integer array for with integer menu items.
// The size of array must be equal to the menu size, e. g.:
// :math:`ceil(\frac{maximum - minimum}{step}) + 1`.
// Used only if the @type is %V4L2_CTRL_TYPE_INTEGER_MENU.
// @flags:	The control's flags.
// @priv:	The control's private pointer. For use by the driver. It is
// untouched by the control framework. Note that this pointer is
// not freed when the control is deleted. Should this be needed
// then a new internal bitfield can be added to tell the framework
// to free this pointer.
// @p_array:	Pointer to the allocated array. Only valid if @is_array is true.
// @p_array_alloc_elems: The number of elements in the allocated
// array for both the cur and new values. So @p_array is actually
// sized for 2 * @p_array_alloc_elems * @elem_size. Only valid if
// @is_array is true.
// @cur:	Structure to store the current value.
// @cur.val:	The control's current value, if the @type is represented via
// a u32 integer (see &enum v4l2_ctrl_type).
// @val:	The control's new s32 value.
// @p_def:	The control's default value represented via a union which
// provides a standard way of accessing control types
// through a pointer (for compound controls only).
// @p_min:	The control's minimum value represented via a union which
// provides a standard way of accessing control types
// through a pointer (for compound controls only).
// @p_max:	The control's maximum value represented via a union which
// provides a standard way of accessing control types
// through a pointer (for compound controls only).
// @p_cur:	The control's current value represented via a union which
// provides a standard way of accessing control types
// through a pointer.
// @p_new:	The control's new value represented via a union which provides
// a standard way of accessing control types
// through a pointer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_ctrl {
// Administrative fields
    pub node: list_head,
    pub ev_subs: list_head,
    pub handler: *mut v4l2_ctrl_handler,
    pub cluster: *mut v4l2_ctrl,
    pub ncontrols: c_uint,
    pub done:1: c_uint,
    pub is_new:1: c_uint,
    pub has_changed:1: c_uint,
    pub is_private:1: c_uint,
    pub is_auto:1: c_uint,
    pub is_int:1: c_uint,
    pub is_string:1: c_uint,
    pub is_ptr:1: c_uint,
    pub is_array:1: c_uint,
    pub is_dyn_array:1: c_uint,
    pub has_volatiles:1: c_uint,
    pub call_notify:1: c_uint,
    pub manual_mode_value:8: c_uint,
    pub ops: *const v4l2_ctrl_ops,
    pub type_ops: *const v4l2_ctrl_type_ops,
    pub id: u32,
    pub name: *const c_char,
    pub type: v4l2_ctrl_type,
    pub default_value: s64 minimum, maximum,,
    pub elems: u32,
    pub elem_size: u32,
    pub new_elems: u32,
    pub dims: [u32; V4L2_CTRL_MAX_DIMS],
    pub nr_of_dims: u32,
    pub step: u64,
    pub menu_skip_mask: u64,
}

//
// struct v4l2_ctrl_ref - The control reference.
//
// @node:	List node for the sorted list.
// @next:	Single-link list node for the hash.
// @ctrl:	The actual control information.
// @helper:	Pointer to helper struct. Used internally in
// ``prepare_ext_ctrls`` function at ``v4l2-ctrl.c``.
// @from_other_dev: If true, then @ctrl was defined in another
// device than the &struct v4l2_ctrl_handler.
// @req_done:	Internal flag: if the control handler containing this control
// reference is bound to a media request, then this is set when
// the control has been applied. This prevents applying controls
// from a cluster with multiple controls twice (when the first
// control of a cluster is applied, they all are).
// @p_req_valid: If set, then p_req contains the control value for the request.
// @p_req_array_enomem: If set, then p_req is invalid since allocating space for
// an array failed. Attempting to read this value shall
// result in ENOMEM. Only valid if ctrl->is_array is true.
// @p_req_array_alloc_elems: The number of elements allocated for the
// array. Only valid if @p_req_valid and ctrl->is_array are
// true.
// @p_req_elems: The number of elements in @p_req. This is the same as
// ctrl->elems, except for dynamic arrays. In that case it is in
// the range of 1 to @p_req_array_alloc_elems. Only valid if
// @p_req_valid is true.
// @p_req:	If the control handler containing this control reference
// is bound to a media request, then this points to the
// value of the control that must be applied when the request
// is executed, or to the value of the control at the time
// that the request was completed. If @p_req_valid is false,
// then this control was never set for this request and the
// control will not be updated when this request is applied.
//
// Each control handler has a list of these refs. The list_head is used to
// keep a sorted-by-control-ID list of all controls, while the next pointer
// is used to link the control in the hash's bucket.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_ctrl_ref {
    pub node: list_head,
    pub next: *mut v4l2_ctrl_ref,
    pub ctrl: *mut v4l2_ctrl,
    pub helper: *mut v4l2_ctrl_helper,
    pub from_other_dev: bool,
    pub req_done: bool,
    pub p_req_valid: bool,
    pub p_req_array_enomem: bool,
    pub p_req_array_alloc_elems: u32,
    pub p_req_elems: u32,
    pub p_req: v4l2_ctrl_ptr,
}

//
// struct v4l2_ctrl_handler - The control handler keeps track of all the
// controls: both the controls owned by the handler and those inherited
// from other handlers.
//
// @_lock:	Default for "lock".
// @lock:	Lock to control access to this handler and its controls.
// May be replaced by the user right after init.
// @ctrls:	The list of controls owned by this handler.
// @ctrl_refs:	The list of control references.
// @cached:	The last found control reference. It is common that the same
// control is needed multiple times, so this is a simple
// optimization.
// @buckets:	Buckets for the hashing. Allows for quick control lookup.
// @notify:	A notify callback that is called whenever the control changes
// value.
// Note that the handler's lock is held when the notify function
// is called!
// @notify_priv: Passed as argument to the v4l2_ctrl notify callback.
// @nr_of_buckets: Total number of buckets in the array.
// @error:	The error code of the first failed control addition.
// @request_is_queued: True if the request was queued.
// @requests:	List to keep track of open control handler request objects.
// For the parent control handler (@req_obj.ops == NULL) this
// is the list header. When the parent control handler is
// removed, it has to unbind and put all these requests since
// they refer to the parent.
// @requests_queued: List of the queued requests. This determines the order
// in which these controls are applied. Once the request is
// completed it is removed from this list.
// @req_obj:	The &struct media_request_object, used to link into a
// &struct media_request. This request object has a refcount.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_ctrl_handler {
    pub _lock: mutex,
    pub lock: *mut mutex,
    pub ctrls: list_head,
    pub ctrl_refs: list_head,
    pub cached: *mut v4l2_ctrl_ref,
    pub buckets: *mut v4l2_ctrl_ref,
    pub notify: v4l2_ctrl_notify_fnc,
    pub notify_priv: *mut c_void,
    pub nr_of_buckets: u16,
    pub error: c_int,
    pub request_is_queued: bool,
    pub requests: list_head,
    pub requests_queued: list_head,
    pub req_obj: media_request_object,
}

//
// struct v4l2_ctrl_config - Control configuration structure.
//
// @ops:	The control ops.
// @type_ops:	The control type ops. Only needed for compound controls.
// @id:	The control ID.
// @name:	The control name.
// @type:	The control type.
// @min:	The control's minimum value.
// @max:	The control's maximum value.
// @step:	The control's step value for non-menu controls.
// @def:	The control's default value.
// @p_def:	The control's default value for compound controls.
// @p_min:	The control's minimum value for compound controls.
// @p_max:	The control's maximum value for compound controls.
// @dims:	The size of each dimension.
// @elem_size:	The size in bytes of the control.
// @flags:	The control's flags.
// @menu_skip_mask: The control's skip mask for menu controls. This makes it
// easy to skip menu items that are not valid. If bit X is set,
// then menu item X is skipped. Of course, this only works for
// menus with <= 64 menu items. There are no menus that come
// close to that number, so this is OK. Should we ever need more,
// then this will have to be extended to a bit array.
// @qmenu:	A const char * array for all menu items. Array entries that are
// empty strings ("") correspond to non-existing menu items (this
// is in addition to the menu_skip_mask above). The last entry
// must be NULL.
// @qmenu_int:	A const s64 integer array for all menu items of the type
// V4L2_CTRL_TYPE_INTEGER_MENU.
// @is_private: If set, then this control is private to its handler and it
// will not be added to any other handlers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_ctrl_config {
    pub ops: *const v4l2_ctrl_ops,
    pub type_ops: *const v4l2_ctrl_type_ops,
    pub id: u32,
    pub name: *const c_char,
    pub type: v4l2_ctrl_type,
    pub min: i64,
    pub max: i64,
    pub step: u64,
    pub def: i64,
    pub p_def: v4l2_ctrl_ptr,
    pub p_min: v4l2_ctrl_ptr,
    pub p_max: v4l2_ctrl_ptr,
    pub dims: [u32; V4L2_CTRL_MAX_DIMS],
    pub elem_size: u32,
    pub flags: u32,
    pub menu_skip_mask: u64,
    pub qmenu: *const *const c_char,
    pub qmenu_int: *const i64,
    pub is_private:1: c_uint,
}

//
// v4l2_ctrl_fill - Fill in the control fields based on the control ID.
//
// @id: ID of the control
// @name: pointer to be filled with a string with the name of the control
// @type: pointer for storing the type of the control
// @min: pointer for storing the minimum value for the control
// @max: pointer for storing the maximum value for the control
// @step: pointer for storing the control step
// @def: pointer for storing the default value for the control
// @flags: pointer for storing the flags to be used on the control
//
// This works for all standard V4L2 controls.
// For non-standard controls it will only fill in the given arguments
// and @name content will be set to %NULL.
//
// This function will overwrite the contents of @name, @type and @flags.
// The contents of @min, @max, @step and @def may be modified depending on
// the type.
//
// .. note::
//
// Do not use in drivers! It is used internally for backwards compatibility
// control handling only. Once all drivers are converted to use the new
// control framework this function will no longer be exported.
//
// v4l2_ctrl_handler_init_class() - Initialize the control handler.
// @hdl:	The control handler.
// @nr_of_controls_hint: A hint of how many controls this handler is
// expected to refer to. This is the total number, so including
// any inherited controls. It doesn't have to be precise, but if
// it is way off, then you either waste memory (too many buckets
// are allocated) or the control lookup becomes slower (not enough
// buckets are allocated, so there are more slow list lookups).
// It will always work, though.
// @key:	Used by the lock validator if CONFIG_LOCKDEP is set.
// @name:	Used by the lock validator if CONFIG_LOCKDEP is set.
//
// .. attention::
//
// Never use this call directly, always use the v4l2_ctrl_handler_init()
// macro that hides the @key and @name arguments.
//
// Return: returns an error if the buckets could not be allocated. This
// error will also be stored in @hdl->error.
//

//
// v4l2_ctrl_handler_init - helper function to create a static struct
// &lock_class_key and calls v4l2_ctrl_handler_init_class()
//
// @hdl:	The control handler.
// @nr_of_controls_hint: A hint of how many controls this handler is
// expected to refer to. This is the total number, so including
// any inherited controls. It doesn't have to be precise, but if
// it is way off, then you either waste memory (too many buckets
// are allocated) or the control lookup becomes slower (not enough
// buckets are allocated, so there are more slow list lookups).
// It will always work, though.
//
// This helper function creates a static struct &lock_class_key and
// calls v4l2_ctrl_handler_init_class(), providing a proper name for the lock
// validador.
//
// Use this helper function to initialize a control handler.
//

//
// v4l2_ctrl_handler_free() - Free all controls owned by the handler and free
// the control list.
// @hdl:	The control handler.
//
// Does nothing if @hdl == NULL.
//
// Return: @hdl's error field or 0 if @hdl is NULL.
//
extern "C" {
    pub fn v4l2_ctrl_handler_free(hdl: *mut v4l2_ctrl_handler) -> c_int;
}
//
// v4l2_ctrl_lock() - Helper function to lock the handler
// associated with the control.
// @ctrl:	The control to lock.
//
// v4l2_ctrl_unlock() - Helper function to unlock the handler
// associated with the control.
// @ctrl:	The control to unlock.
//
// __v4l2_ctrl_handler_setup() - Call the s_ctrl op for all controls belonging
// to the handler to initialize the hardware to the current control values. The
// caller is responsible for acquiring the control handler mutex on behalf of
// __v4l2_ctrl_handler_setup().
// @hdl:	The control handler.
//
// Button controls will be skipped, as are read-only controls.
//
// If @hdl == NULL, then this just returns 0.
//
extern "C" {
    pub fn __v4l2_ctrl_handler_setup(hdl: *mut v4l2_ctrl_handler) -> c_int;
}
//
// v4l2_ctrl_handler_setup() - Call the s_ctrl op for all controls belonging
// to the handler to initialize the hardware to the current control values.
// @hdl:	The control handler.
//
// Button controls will be skipped, as are read-only controls.
//
// If @hdl == NULL, then this just returns 0.
//
extern "C" {
    pub fn v4l2_ctrl_handler_setup(hdl: *mut v4l2_ctrl_handler) -> c_int;
}
//
// v4l2_ctrl_handler_log_status() - Log all controls owned by the handler.
// @hdl:	The control handler.
// @prefix:	The prefix to use when logging the control values. If the
// prefix does not end with a space, then ": " will be added
// after the prefix. If @prefix == NULL, then no prefix will be
// used.
//
// For use with VIDIOC_LOG_STATUS.
//
// Does nothing if @hdl == NULL.
//
// v4l2_ctrl_new_custom() - Allocate and initialize a new custom V4L2
// control.
//
// @hdl:	The control handler.
// @cfg:	The control's configuration data.
// @priv:	The control's driver-specific private data.
//
// If the &v4l2_ctrl struct could not be allocated then NULL is returned
// and @hdl->error is set to the error code (if it wasn't set already).
//
// v4l2_ctrl_new_std() - Allocate and initialize a new standard V4L2 non-menu
// control.
//
// @hdl:	The control handler.
// @ops:	The control ops.
// @id:		The control ID.
// @min:	The control's minimum value.
// @max:	The control's maximum value.
// @step:	The control's step value
// @def:	The control's default value.
//
// If the &v4l2_ctrl struct could not be allocated, or the control
// ID is not known, then NULL is returned and @hdl->error is set to the
// appropriate error code (if it wasn't set already).
//
// If @id refers to a menu control, then this function will return NULL.
//
// Use v4l2_ctrl_new_std_menu() when adding menu controls.
//
// v4l2_ctrl_new_std_menu() - Allocate and initialize a new standard V4L2
// menu control.
//
// @hdl:	The control handler.
// @ops:	The control ops.
// @id:		The control ID.
// @max:	The control's maximum value.
// @mask:	The control's skip mask for menu controls. This makes it
// easy to skip menu items that are not valid. If bit X is set,
// then menu item X is skipped. Of course, this only works for
// menus with <= 64 menu items. There are no menus that come
// close to that number, so this is OK. Should we ever need more,
// then this will have to be extended to a bit array.
// @def:	The control's default value.
//
// Same as v4l2_ctrl_new_std(), but @min is set to 0 and the @mask value
// determines which menu items are to be skipped.
//
// If @id refers to a non-menu control, then this function will return NULL.
//
// v4l2_ctrl_new_std_menu_items() - Create a new standard V4L2 menu control
// with driver specific menu.
//
// @hdl:	The control handler.
// @ops:	The control ops.
// @id:	The control ID.
// @max:	The control's maximum value.
// @mask:	The control's skip mask for menu controls. This makes it
// easy to skip menu items that are not valid. If bit X is set,
// then menu item X is skipped. Of course, this only works for
// menus with <= 64 menu items. There are no menus that come
// close to that number, so this is OK. Should we ever need more,
// then this will have to be extended to a bit array.
// @def:	The control's default value.
// @qmenu:	The new menu.
//
// Same as v4l2_ctrl_new_std_menu(), but @qmenu will be the driver specific
// menu of this control.
//
// v4l2_ctrl_new_std_compound() - Allocate and initialize a new standard V4L2
// compound control.
//
// @hdl:       The control handler.
// @ops:       The control ops.
// @id:        The control ID.
// @p_def:     The control's default value.
// @p_min:     The control's minimum value.
// @p_max:     The control's maximum value.
//
// Same as v4l2_ctrl_new_std(), but with support for compound controls.
// To fill in the @p_def, @p_min and @p_max fields, use v4l2_ctrl_ptr_create()
// to convert a pointer to a const union v4l2_ctrl_ptr.
// Use v4l2_ctrl_ptr_create(NULL) if you want the default, minimum or maximum
// value of the compound control to be all zeroes.
// If the compound control does not set the ``V4L2_CTRL_FLAG_HAS_WHICH_MIN_MAX``
// flag, then it does not has minimum and maximum values. In that case just use
// v4l2_ctrl_ptr_create(NULL) for the @p_min and @p_max arguments.
//
// v4l2_ctrl_new_int_menu() - Create a new standard V4L2 integer menu control.
//
// @hdl:	The control handler.
// @ops:	The control ops.
// @id:	The control ID.
// @max:	The control's maximum value.
// @def:	The control's default value.
// @qmenu_int:	The control's menu entries.
//
// Same as v4l2_ctrl_new_std_menu(), but @mask is set to 0 and it additionally
// takes as an argument an array of integers determining the menu items.
//
// If @id refers to a non-integer-menu control, then this function will
// return %NULL.
//
// typedef v4l2_ctrl_filter - Typedef to define the filter function to be
// used when adding a control handler.
//
// @ctrl: pointer to struct &v4l2_ctrl.
//
extern "C" {
    pub fn bool(ctrl: *const *const v4l2_ctrl_filter)(struct v4l2_ctrl) -> typedef;
}
//
// v4l2_ctrl_add_handler() - Add all controls from handler @add to
// handler @hdl.
//
// @hdl:	The control handler.
// @add:	The control handler whose controls you want to add to
// the @hdl control handler.
// @filter:	This function will filter which controls should be added.
// @from_other_dev: If true, then the controls in @add were defined in another
// device than @hdl.
//
// Does nothing if either of the two handlers is a NULL pointer.
// If @filter is NULL, then all controls are added. Otherwise only those
// controls for which @filter returns true will be added.
// In case of an error @hdl->error will be set to the error code (if it
// wasn't set already).
//
// v4l2_ctrl_radio_filter() - Standard filter for radio controls.
//
// @ctrl:	The control that is filtered.
//
// This will return true for any controls that are valid for radio device
// nodes. Those are all of the V4L2_CID_AUDIO_* user controls and all FM
// transmitter class controls.
//
// This function is to be used with v4l2_ctrl_add_handler().
//
extern "C" {
    pub fn v4l2_ctrl_radio_filter(ctrl: *const v4l2_ctrl) -> bool;
}
//
// v4l2_ctrl_cluster() - Mark all controls in the cluster as belonging
// to that cluster.
//
// @ncontrols:	The number of controls in this cluster.
// @controls:	The cluster control array of size @ncontrols.
//
extern "C" {
    pub fn v4l2_ctrl_cluster(ncontrols: c_uint, controls: *mut v4l2_ctrl);
}
//
// v4l2_ctrl_auto_cluster() - Mark all controls in the cluster as belonging
// to that cluster and set it up for autofoo/foo-type handling.
//
// @ncontrols:	The number of controls in this cluster.
// @controls:	The cluster control array of size @ncontrols. The first control
// must be the 'auto' control (e.g. autogain, autoexposure, etc.)
// @manual_val: The value for the first control in the cluster that equals the
// manual setting.
// @set_volatile: If true, then all controls except the first auto control will
// be volatile.
//
// Use for control groups where one control selects some automatic feature and
// the other controls are only active whenever the automatic feature is turned
// off (manual mode). Typical examples: autogain vs gain, auto-whitebalance vs
// red and blue balance, etc.
//
// The behavior of such controls is as follows:
//
// When the autofoo control is set to automatic, then any manual controls
// are set to inactive and any reads will call g_volatile_ctrl (if the control
// was marked volatile).
//
// When the autofoo control is set to manual, then any manual controls will
// be marked active, and any reads will just return the current value without
// going through g_volatile_ctrl.
//
// In addition, this function will set the %V4L2_CTRL_FLAG_UPDATE flag
// on the autofoo control and %V4L2_CTRL_FLAG_INACTIVE on the foo control(s)
// if autofoo is in auto mode.
//
// v4l2_ctrl_find() - Find a control with the given ID.
//
// @hdl:	The control handler.
// @id:	The control ID to find.
//
// If @hdl == NULL this will return NULL as well. Will lock the handler so
// do not use from inside &v4l2_ctrl_ops.
//
// v4l2_ctrl_activate() - Make the control active or inactive.
// @ctrl:	The control to (de)activate.
// @active:	True if the control should become active.
//
// This sets or clears the V4L2_CTRL_FLAG_INACTIVE flag atomically.
// Does nothing if @ctrl == NULL.
// This will usually be called from within the s_ctrl op.
// The V4L2_EVENT_CTRL event will be generated afterwards.
//
// This function assumes that the control handler is locked.
//
extern "C" {
    pub fn v4l2_ctrl_activate(ctrl: *mut v4l2_ctrl, active: bool);
}
//
// __v4l2_ctrl_grab() - Unlocked variant of v4l2_ctrl_grab.
//
// @ctrl:	The control to (de)activate.
// @grabbed:	True if the control should become grabbed.
//
// This sets or clears the V4L2_CTRL_FLAG_GRABBED flag atomically.
// Does nothing if @ctrl == NULL.
// The V4L2_EVENT_CTRL event will be generated afterwards.
// This will usually be called when starting or stopping streaming in the
// driver.
//
// This function assumes that the control handler is locked by the caller.
//
extern "C" {
    pub fn __v4l2_ctrl_grab(ctrl: *mut v4l2_ctrl, grabbed: bool);
}
//
// v4l2_ctrl_grab() - Mark the control as grabbed or not grabbed.
//
// @ctrl:	The control to (de)activate.
// @grabbed:	True if the control should become grabbed.
//
// This sets or clears the V4L2_CTRL_FLAG_GRABBED flag atomically.
// Does nothing if @ctrl == NULL.
// The V4L2_EVENT_CTRL event will be generated afterwards.
// This will usually be called when starting or stopping streaming in the
// driver.
//
// This function assumes that the control handler is not locked and will
// take the lock itself.
//
// __v4l2_ctrl_modify_range() - Unlocked variant of v4l2_ctrl_modify_range()
//
// @ctrl:	The control to update.
// @min:	The control's minimum value.
// @max:	The control's maximum value.
// @step:	The control's step value
// @def:	The control's default value.
//
// Update the range of a control on the fly. This works for control types
// INTEGER, BOOLEAN, MENU, INTEGER MENU and BITMASK. For menu controls the
// @step value is interpreted as a menu_skip_mask.
//
// An error is returned if one of the range arguments is invalid for this
// control type.
//
// The caller is responsible for acquiring the control handler mutex on behalf
// of __v4l2_ctrl_modify_range().
//
// v4l2_ctrl_modify_range() - Update the range of a control.
//
// @ctrl:	The control to update.
// @min:	The control's minimum value.
// @max:	The control's maximum value.
// @step:	The control's step value
// @def:	The control's default value.
//
// Update the range of a control on the fly. This works for control types
// INTEGER, BOOLEAN, MENU, INTEGER MENU and BITMASK. For menu controls the
// @step value is interpreted as a menu_skip_mask.
//
// An error is returned if one of the range arguments is invalid for this
// control type.
//
// This function assumes that the control handler is not locked and will
// take the lock itself.
//
// __v4l2_ctrl_modify_dimensions() - Unlocked variant of v4l2_ctrl_modify_dimensions()
//
// @ctrl:	The control to update.
// @dims:	The control's new dimensions.
//
// Update the dimensions of an array control on the fly. The elements of the
// array are reset to their default value, even if the dimensions are
// unchanged.
//
// An error is returned if @dims is invalid for this control.
//
// The caller is responsible for acquiring the control handler mutex on behalf
// of __v4l2_ctrl_modify_dimensions().
//
// Note: calling this function when the same control is used in pending requests
// is untested. It should work (a request with the wrong size of the control
// will drop that control silently), but it will be very confusing.
//
// v4l2_ctrl_modify_dimensions() - Update the dimensions of an array control.
//
// @ctrl:	The control to update.
// @dims:	The control's new dimensions.
//
// Update the dimensions of an array control on the fly. The elements of the
// array are reset to their default value, even if the dimensions are
// unchanged.
//
// An error is returned if @dims is invalid for this control type.
//
// This function assumes that the control handler is not locked and will
// take the lock itself.
//
// Note: calling this function when the same control is used in pending requests
// is untested. It should work (a request with the wrong size of the control
// will drop that control silently), but it will be very confusing.
//
// v4l2_ctrl_notify() - Function to set a notify callback for a control.
//
// @ctrl:	The control.
// @notify:	The callback function.
// @priv:	The callback private handle, passed as argument to the callback.
//
// This function sets a callback function for the control. If @ctrl is NULL,
// then it will do nothing. If @notify is NULL, then the notify callback will
// be removed.
//
// There can be only one notify. If another already exists, then a WARN_ON
// will be issued and the function will do nothing.
//
// v4l2_ctrl_get_name() - Get the name of the control
//
// @id:		The control ID.
//
// This function returns the name of the given control ID or NULL if it isn't
// a known control.
//
// v4l2_ctrl_get_menu() - Get the menu string array of the control
//
// @id:		The control ID.
//
// This function returns the NULL-terminated menu string array name of the
// given control ID or NULL if it isn't a known menu control.
//
// v4l2_ctrl_get_int_menu() - Get the integer menu array of the control
//
// @id:		The control ID.
// @len:	The size of the integer array.
//
// This function returns the integer array of the given control ID or NULL if it
// if it isn't a known integer menu control.
//
// v4l2_ctrl_g_ctrl() - Helper function to get the control's value from
// within a driver.
//
// @ctrl:	The control.
//
// This returns the control's value safely by going through the control
// framework. This function will lock the control's handler, so it cannot be
// used from within the &v4l2_ctrl_ops functions.
//
// This function is for integer type controls only.
//
extern "C" {
    pub fn v4l2_ctrl_g_ctrl(ctrl: *mut v4l2_ctrl) -> i32;
}
//
// __v4l2_ctrl_s_ctrl() - Unlocked variant of v4l2_ctrl_s_ctrl().
//
// @ctrl:	The control.
// @val:	The new value.
//
// This sets the control's new value safely by going through the control
// framework. This function assumes the control's handler is already locked,
// allowing it to be used from within the &v4l2_ctrl_ops functions.
//
// This function is for integer type controls only.
//
extern "C" {
    pub fn __v4l2_ctrl_s_ctrl(ctrl: *mut v4l2_ctrl, val: i32) -> c_int;
}
//
// v4l2_ctrl_s_ctrl() - Helper function to set the control's value from
// within a driver.
// @ctrl:	The control.
// @val:	The new value.
//
// This sets the control's new value safely by going through the control
// framework. This function will lock the control's handler, so it cannot be
// used from within the &v4l2_ctrl_ops functions.
//
// This function is for integer type controls only.
//
// v4l2_ctrl_g_ctrl_int64() - Helper function to get a 64-bit control's value
// from within a driver.
//
// @ctrl:	The control.
//
// This returns the control's value safely by going through the control
// framework. This function will lock the control's handler, so it cannot be
// used from within the &v4l2_ctrl_ops functions.
//
// This function is for 64-bit integer type controls only.
//
extern "C" {
    pub fn v4l2_ctrl_g_ctrl_int64(ctrl: *mut v4l2_ctrl) -> i64;
}
//
// __v4l2_ctrl_s_ctrl_int64() - Unlocked variant of v4l2_ctrl_s_ctrl_int64().
//
// @ctrl:	The control.
// @val:	The new value.
//
// This sets the control's new value safely by going through the control
// framework. This function assumes the control's handler is already locked,
// allowing it to be used from within the &v4l2_ctrl_ops functions.
//
// This function is for 64-bit integer type controls only.
//
extern "C" {
    pub fn __v4l2_ctrl_s_ctrl_int64(ctrl: *mut v4l2_ctrl, val: i64) -> c_int;
}
//
// v4l2_ctrl_s_ctrl_int64() - Helper function to set a 64-bit control's value
// from within a driver.
//
// @ctrl:	The control.
// @val:	The new value.
//
// This sets the control's new value safely by going through the control
// framework. This function will lock the control's handler, so it cannot be
// used from within the &v4l2_ctrl_ops functions.
//
// This function is for 64-bit integer type controls only.
//
// __v4l2_ctrl_s_ctrl_string() - Unlocked variant of v4l2_ctrl_s_ctrl_string().
//
// @ctrl:	The control.
// @s:		The new string.
//
// This sets the control's new string safely by going through the control
// framework. This function assumes the control's handler is already locked,
// allowing it to be used from within the &v4l2_ctrl_ops functions.
//
// This function is for string type controls only.
//
extern "C" {
    pub fn __v4l2_ctrl_s_ctrl_string(ctrl: *mut v4l2_ctrl, s: *const c_char) -> c_int;
}
//
// v4l2_ctrl_s_ctrl_string() - Helper function to set a control's string value
// from within a driver.
//
// @ctrl:	The control.
// @s:		The new string.
//
// This sets the control's new string safely by going through the control
// framework. This function will lock the control's handler, so it cannot be
// used from within the &v4l2_ctrl_ops functions.
//
// This function is for string type controls only.
//
// __v4l2_ctrl_s_ctrl_compound() - Unlocked variant to set a compound control
//
// @ctrl: The control.
// @type: The type of the data.
// @p:    The new compound payload.
//
// This sets the control's new compound payload safely by going through the
// control framework. This function assumes the control's handler is already
// locked, allowing it to be used from within the &v4l2_ctrl_ops functions.
//
// This function is for compound type controls only.
//
// v4l2_ctrl_s_ctrl_compound() - Helper function to set a compound control
// from within a driver.
//
// @ctrl: The control.
// @type: The type of the data.
// @p:    The new compound payload.
//
// This sets the control's new compound payload safely by going through the
// control framework. This function will lock the control's handler, so it
// cannot be used from within the &v4l2_ctrl_ops functions.
//
// This function is for compound type controls only.
//
// Helper defines for area type controls

// Internal helper functions that deal with control events.
//
// v4l2_ctrl_replace - Function to be used as a callback to
// &struct v4l2_subscribed_event_ops replace\(\)
//
// @old: pointer to struct &v4l2_event with the reported
// event;
// @new: pointer to struct &v4l2_event with the modified
// event;
//
extern "C" {
    pub fn v4l2_ctrl_replace(old: *mut v4l2_event, new: *const v4l2_event);
}
//
// v4l2_ctrl_merge - Function to be used as a callback to
// &struct v4l2_subscribed_event_ops merge(\)
//
// @old: pointer to struct &v4l2_event with the reported
// event;
// @new: pointer to struct &v4l2_event with the merged
// event;
//
extern "C" {
    pub fn v4l2_ctrl_merge(old: *const v4l2_event, new: *mut v4l2_event);
}
//
// v4l2_ctrl_log_status - helper function to implement %VIDIOC_LOG_STATUS ioctl
//
// @file: pointer to struct file
// @priv: unused. Kept just to be compatible to the arguments expected by
// &struct v4l2_ioctl_ops.vidioc_log_status.
//
// Can be used as a vidioc_log_status function that just dumps all controls
// associated with the filehandle.
//
extern "C" {
    pub fn v4l2_ctrl_log_status(file: *mut file, priv: *mut c_void) -> c_int;
}
//
// v4l2_ctrl_subscribe_event - Subscribes to an event
//
// @fh: pointer to struct v4l2_fh
// @sub: pointer to &struct v4l2_event_subscription
//
// Can be used as a vidioc_subscribe_event function that just subscribes
// control events.
//
// v4l2_ctrl_poll - function to be used as a callback to the poll()
// That just polls for control events.
//
// @file: pointer to struct file
// @wait: pointer to struct poll_table_struct
//
extern "C" {
    pub fn v4l2_ctrl_poll(file: *mut file, wait: *mut poll_table_struct) -> __poll_t;
}
//
// v4l2_ctrl_request_setup - helper function to apply control values in a request
//
// @req: The request
// @parent: The parent control handler ('priv' in media_request_object_find())
//
// This is a helper function to call the control handler's s_ctrl callback with
// the control values contained in the request. Do note that this approach of
// applying control values in a request is only applicable to memory-to-memory
// devices.
//
// v4l2_ctrl_request_complete - Complete a control handler request object
//
// @req: The request
// @parent: The parent control handler ('priv' in media_request_object_find())
//
// This function is to be called on each control handler that may have had a
// request object associated with it, i.e. control handlers of a driver that
// supports requests.
//
// The function first obtains the values of any volatile controls in the control
// handler and attach them to the request. Then, the function completes the
// request object.
//
// v4l2_ctrl_request_hdl_find - Find the control handler in the request
//
// @req: The request
// @parent: The parent control handler ('priv' in media_request_object_find())
//
// This function finds the control handler in the request. It may return
// NULL if not found. When done, you must call v4l2_ctrl_request_hdl_put()
// with the returned handler pointer.
//
// If the request is not in state VALIDATING or QUEUED, then this function
// will always return NULL.
//
// Note that in state VALIDATING the req_queue_mutex is held, so
// no objects can be added or deleted from the request.
//
// In state QUEUED it is the driver that will have to ensure this.
//
// v4l2_ctrl_request_hdl_put - Put the control handler
//
// @hdl: Put this control handler
//
// This function released the control handler previously obtained from'
// v4l2_ctrl_request_hdl_find().
//
// v4l2_ctrl_request_hdl_ctrl_find() - Find a control with the given ID.
//
// @hdl: The control handler from the request.
// @id: The ID of the control to find.
//
// This function returns a pointer to the control if this control is
// part of the request or NULL otherwise.
//
// Helpers for ioctl_ops
//
// v4l2_queryctrl - Helper function to implement
// :ref:`VIDIOC_QUERYCTRL <vidioc_queryctrl>` ioctl
//
// @hdl: pointer to &struct v4l2_ctrl_handler
// @qc: pointer to &struct v4l2_queryctrl
//
// If hdl == NULL then they will all return -EINVAL.
//
extern "C" {
    pub fn v4l2_queryctrl(hdl: *mut v4l2_ctrl_handler, qc: *mut v4l2_queryctrl) -> c_int;
}
//
// v4l2_query_ext_ctrl_to_v4l2_queryctrl - Convert a qec to qe.
//
// @to: The v4l2_queryctrl to write to.
// @from: The v4l2_query_ext_ctrl to read from.
//
// This function is a helper to convert a v4l2_query_ext_ctrl into a
// v4l2_queryctrl.
//
// v4l2_query_ext_ctrl - Helper function to implement
// :ref:`VIDIOC_QUERY_EXT_CTRL <vidioc_queryctrl>` ioctl
//
// @hdl: pointer to &struct v4l2_ctrl_handler
// @qc: pointer to &struct v4l2_query_ext_ctrl
//
// If hdl == NULL then they will all return -EINVAL.
//
// v4l2_querymenu - Helper function to implement
// :ref:`VIDIOC_QUERYMENU <vidioc_queryctrl>` ioctl
//
// @hdl: pointer to &struct v4l2_ctrl_handler
// @qm: pointer to &struct v4l2_querymenu
//
// If hdl == NULL then they will all return -EINVAL.
//
extern "C" {
    pub fn v4l2_querymenu(hdl: *mut v4l2_ctrl_handler, qm: *mut v4l2_querymenu) -> c_int;
}
//
// v4l2_g_ctrl - Helper function to implement
// :ref:`VIDIOC_G_CTRL <vidioc_g_ctrl>` ioctl
//
// @hdl: pointer to &struct v4l2_ctrl_handler
// @ctrl: pointer to &struct v4l2_control
//
// If hdl == NULL then they will all return -EINVAL.
//
extern "C" {
    pub fn v4l2_g_ctrl(hdl: *mut v4l2_ctrl_handler, ctrl: *mut v4l2_control) -> c_int;
}
//
// v4l2_s_ctrl - Helper function to implement
// :ref:`VIDIOC_S_CTRL <vidioc_g_ctrl>` ioctl
//
// @fh: pointer to &struct v4l2_fh
// @hdl: pointer to &struct v4l2_ctrl_handler
//
// @ctrl: pointer to &struct v4l2_control
//
// If hdl == NULL then they will all return -EINVAL.
//
// v4l2_g_ext_ctrls - Helper function to implement
// :ref:`VIDIOC_G_EXT_CTRLS <vidioc_g_ext_ctrls>` ioctl
//
// @hdl: pointer to &struct v4l2_ctrl_handler
// @vdev: pointer to &struct video_device
// @mdev: pointer to &struct media_device
// @c: pointer to &struct v4l2_ext_controls
//
// If hdl == NULL then they will all return -EINVAL.
//
// v4l2_try_ext_ctrls - Helper function to implement
// :ref:`VIDIOC_TRY_EXT_CTRLS <vidioc_g_ext_ctrls>` ioctl
//
// @hdl: pointer to &struct v4l2_ctrl_handler
// @vdev: pointer to &struct video_device
// @mdev: pointer to &struct media_device
// @c: pointer to &struct v4l2_ext_controls
//
// If hdl == NULL then they will all return -EINVAL.
//
// v4l2_s_ext_ctrls - Helper function to implement
// :ref:`VIDIOC_S_EXT_CTRLS <vidioc_g_ext_ctrls>` ioctl
//
// @fh: pointer to &struct v4l2_fh
// @hdl: pointer to &struct v4l2_ctrl_handler
// @vdev: pointer to &struct video_device
// @mdev: pointer to &struct media_device
// @c: pointer to &struct v4l2_ext_controls
//
// If hdl == NULL then they will all return -EINVAL.
//
// v4l2_ctrl_subdev_subscribe_event - Helper function to implement
// as a &struct v4l2_subdev_core_ops subscribe_event function
// that just subscribes control events.
//
// @sd: pointer to &struct v4l2_subdev
// @fh: pointer to &struct v4l2_fh
// @sub: pointer to &struct v4l2_event_subscription
//
// v4l2_ctrl_subdev_log_status - Log all controls owned by subdev's control
// handler.
//
// @sd: pointer to &struct v4l2_subdev
//
extern "C" {
    pub fn v4l2_ctrl_subdev_log_status(sd: *mut v4l2_subdev) -> c_int;
}
//
// v4l2_ctrl_new_fwnode_properties() - Register controls for the device
// properties
//
// @hdl: pointer to &struct v4l2_ctrl_handler to register controls on
// @ctrl_ops: pointer to &struct v4l2_ctrl_ops to register controls with
// @p: pointer to &struct v4l2_fwnode_device_properties
//
// This function registers controls associated to device properties, using the
// property values contained in @p parameter, if the property has been set to
// a value.
//
// Currently the following v4l2 controls are parsed and registered:
// - V4L2_CID_CAMERA_ORIENTATION
// - V4L2_CID_CAMERA_SENSOR_ROTATION;
//
// Controls already registered by the caller with the @hdl control handler are
// not overwritten. Callers should register the controls they want to handle
// themselves before calling this function.
//
// This function will set the control handler's error field on failure, just as
// other functions adding controls to the handler.
//
// Return: 0 on success, a negative error code on failure.
//
// v4l2_ctrl_type_op_equal - Default v4l2_ctrl_type_ops equal callback.
//
// @ctrl: The v4l2_ctrl pointer.
// @ptr1: A v4l2 control value.
// @ptr2: A v4l2 control value.
//
// Return: true if values are equal, otherwise false.
//
// v4l2_ctrl_type_op_init - Default v4l2_ctrl_type_ops init callback.
//
// @ctrl: The v4l2_ctrl pointer.
// @from_idx: Starting element index.
// @ptr: The v4l2 control value.
//
// Return: void
//
// v4l2_ctrl_type_op_log - Default v4l2_ctrl_type_ops log callback.
//
// @ctrl: The v4l2_ctrl pointer.
//
// Return: void
//
extern "C" {
    pub fn v4l2_ctrl_type_op_log(ctrl: *const v4l2_ctrl);
}
//
// v4l2_ctrl_type_op_validate - Default v4l2_ctrl_type_ops validate callback.
//
// @ctrl: The v4l2_ctrl pointer.
// @ptr: The v4l2 control value.
//
// Return: 0 on success, a negative error code on failure.
//
extern "C" {
    pub fn v4l2_ctrl_type_op_validate(ctrl: *const v4l2_ctrl, ptr: v4l2_ctrl_ptr) -> c_int;
}
