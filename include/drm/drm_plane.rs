//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/drm_plane.h
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
// Copyright (c) 2016 Intel Corporation
//
// Permission to use, copy, modify, distribute, and sell this software and its
// documentation for any purpose is hereby granted without fee, provided that
// the above copyright notice appear in all copies and that both that copyright
// notice and this permission notice appear in supporting documentation, and
// that the name of the copyright holders not be used in advertising or
// publicity pertaining to distribution of the software without specific,
// written prior permission.  The copyright holders make no representations
// about the suitability of this software for any purpose.  It is provided "as
// is" without express or implied warranty.
//
// THE COPYRIGHT HOLDERS DISCLAIM ALL WARRANTIES WITH REGARD TO THIS SOFTWARE,
// INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO
// EVENT SHALL THE COPYRIGHT HOLDERS BE LIABLE FOR ANY SPECIAL, INDIRECT OR
// CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE,
// DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER
// TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE
// OF THIS SOFTWARE.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_scaling_filter {
    DRM_SCALING_FILTER_DEFAULT,
    DRM_SCALING_FILTER_NEAREST_NEIGHBOR,
}

//
// struct drm_plane_state - mutable plane state
//
// Please note that the destination coordinates @crtc_x, @crtc_y, @crtc_h and
// @crtc_w and the source coordinates @src_x, @src_y, @src_h and @src_w are the
// raw coordinates provided by userspace. Drivers should use
// drm_atomic_helper_check_plane_state() and only use the derived rectangles in
// @src and @dst to program the hardware.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_plane_state {
// @plane: backpointer to the plane
    pub plane: *mut drm_plane,
//
// @crtc:
//
// Currently bound CRTC, NULL if disabled. Do not write this directly,
// use drm_atomic_set_crtc_for_plane()
//
    pub crtc: *mut drm_crtc,
//
// @fb:
//
// Currently bound framebuffer. Do not write this directly, use
// drm_atomic_set_fb_for_plane()
//
    pub fb: *mut drm_framebuffer,
//
// @fence:
//
// Optional fence to wait for before scanning out @fb. The core atomic
// code will set this when userspace is using explicit fencing. Do not
// write this field directly for a driver's implicit fence.
//
// Drivers should store any implicit fence in this from their
// &drm_plane_helper_funcs.prepare_fb callback. See
// drm_gem_plane_helper_prepare_fb() for a suitable helper.
//
    pub fence: *mut dma_fence,
//
// @crtc_x:
//
// Left position of visible portion of plane on crtc, signed dest
// location allows it to be partially off screen.
//
    pub crtc_x: i32,
//
// @crtc_y:
//
// Upper position of visible portion of plane on crtc, signed dest
// location allows it to be partially off screen.
//
    pub crtc_y: i32,
// @crtc_w: width of visible portion of plane on crtc
// @crtc_h: height of visible portion of plane on crtc
    pub crtc_h: uint32_t crtc_w,,
//
// @src_x: left position of visible portion of plane within plane (in
// 16.16 fixed point).
//
    pub src_x: u32,
//
// @src_y: upper position of visible portion of plane within plane (in
// 16.16 fixed point).
//
    pub src_y: u32,
// @src_w: width of visible portion of plane (in 16.16)
// @src_h: height of visible portion of plane (in 16.16)
    pub src_w: uint32_t src_h,,
// @hotspot_x: x offset to mouse cursor hotspot
// @hotspot_y: y offset to mouse cursor hotspot
    pub hotspot_y: int32_t hotspot_x,,
//
// @alpha:
// Opacity of the plane with 0 as completely transparent and 0xffff as
// completely opaque. See drm_plane_create_alpha_property() for more
// details.
//
    pub alpha: u16,
//
// @pixel_blend_mode:
// The alpha blending equation selection, describing how the pixels from
// the current plane are composited with the background. Value can be
// one of DRM_MODE_BLEND_
//
    pub pixel_blend_mode: u16,
//
// @rotation:
// Rotation of the plane. See drm_plane_create_rotation_property() for
// more details.
//
    pub rotation: c_uint,
//
// @zpos:
// Priority of the given plane on crtc (optional).
//
// User-space may set mutable zpos properties so that multiple active
// planes on the same CRTC have identical zpos values. This is a
// user-space bug, but drivers can solve the conflict by comparing the
// plane object IDs; the plane with a higher ID is stacked on top of a
// plane with a lower ID.
//
// See drm_plane_create_zpos_property() and
// drm_plane_create_zpos_immutable_property() for more details.
//
    pub zpos: c_uint,
//
// @normalized_zpos:
// Normalized value of zpos: unique, range from 0 to N-1 where N is the
// number of active planes for given crtc. Note that the driver must set
// &drm_mode_config.normalize_zpos or call drm_atomic_normalize_zpos() to
// update this before it can be trusted.
//
    pub normalized_zpos: c_uint,
//
// @color_encoding:
//
// Color encoding for non RGB formats
//
    pub color_encoding: drm_color_encoding,
//
// @color_range:
//
// Color range for non RGB formats
//
    pub color_range: drm_color_range,
//
// @fb_damage_clips:
//
// Blob representing damage (area in plane framebuffer that changed
// since last plane update) as an array of &drm_mode_rect in framebuffer
// coodinates of the attached framebuffer. Note that unlike plane src,
// damage clips are not in 16.16 fixed point.
//
// See drm_plane_get_damage_clips() and
// drm_plane_get_damage_clips_count() for accessing these.
//
    pub fb_damage_clips: *mut drm_property_blob,
//
// @ignore_damage_clips:
//
// Set by drivers to indicate the drm_atomic_helper_damage_iter_init()
// helper that the @fb_damage_clips blob property should be ignored.
//
// See :ref:`damage_tracking_properties` for more information.
//
    pub ignore_damage_clips: bool,
//
// @src:
//
// source coordinates of the plane (in 16.16).
//
// When using drm_atomic_helper_check_plane_state(),
// the coordinates are clipped, but the driver may choose
// to use unclipped coordinates instead when the hardware
// performs the clipping automatically.
//
// @dst:
//
// clipped destination coordinates of the plane.
//
// When using drm_atomic_helper_check_plane_state(),
// the coordinates are clipped, but the driver may choose
// to use unclipped coordinates instead when the hardware
// performs the clipping automatically.
//
    pub dst: drm_rect src,,
//
// @visible:
//
// Visibility of the plane. This can be false even if fb!=NULL and
// crtc!=NULL, due to clipping.
//
    pub visible: bool,
//
// @scaling_filter:
//
// Scaling filter to be applied
//
    pub scaling_filter: drm_scaling_filter,
//
// @color_pipeline:
//
// The first colorop of the active color pipeline, or NULL, if no
// color pipeline is active.
//
    pub color_pipeline: *mut drm_colorop,
//
// @commit: Tracks the pending commit to prevent use-after-free conditions,
// and for async plane updates.
//
// May be NULL.
//
    pub commit: *mut drm_crtc_commit,
// @state: backpointer to global drm_atomic_commit
    pub state: *mut drm_atomic_commit,
//
// @color_mgmt_changed: Color management properties have changed. Used
// by the atomic helpers and drivers to steer the atomic commit control
// flow.
//
    pub 1: bool color_mgmt_changed :,
}

//
// struct drm_plane_funcs - driver plane control functions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_plane_funcs {
//
// @update_plane:
//
// This is the legacy entry point to enable and configure the plane for
// the given CRTC and framebuffer. It is never called to disable the
// plane, i.e. the passed-in crtc and fb paramters are never NULL.
//
// The source rectangle in frame buffer memory coordinates is given by
// the src_x, src_y, src_w and src_h parameters (as 16.16 fixed point
// values). Devices that don't support subpixel plane coordinates can
// ignore the fractional part.
//
// The destination rectangle in CRTC coordinates is given by the
// crtc_x, crtc_y, crtc_w and crtc_h parameters (as integer values).
// Devices scale the source rectangle to the destination rectangle. If
// scaling is not supported, and the source rectangle size doesn't match
// the destination rectangle size, the driver must return a
// -<errorname>EINVAL</errorname> error.
//
// Drivers implementing atomic modeset should use
// drm_atomic_helper_update_plane() to implement this hook.
//
// RETURNS:
//
// 0 on success or a negative error code on failure.
//
    pub ctx): *mut drm_modeset_acquire_ctx,
//
// @disable_plane:
//
// This is the legacy entry point to disable the plane. The DRM core
// calls this method in response to a DRM_IOCTL_MODE_SETPLANE IOCTL call
// with the frame buffer ID set to 0.  Disabled planes must not be
// processed by the CRTC.
//
// Drivers implementing atomic modeset should use
// drm_atomic_helper_disable_plane() to implement this hook.
//
// RETURNS:
//
// 0 on success or a negative error code on failure.
//
    pub ctx): *mut drm_modeset_acquire_ctx,
//
// @destroy:
//
// Clean up plane resources. This is only called at driver unload time
// through drm_mode_config_cleanup() since a plane cannot be hotplugged
// in DRM.
//
    pub plane): *mut *mut void (destroy)(struct drm_plane,
//
// @reset:
//
// Reset plane hardware and software state to off. This function isn't
// called by the core directly, only through drm_mode_config_reset().
// It's not a helper hook only for historical reasons.
//
// Atomic drivers can use drm_atomic_helper_plane_reset() to reset
// atomic state using this hook.
//
    pub plane): *mut *mut void (reset)(struct drm_plane,
//
// @set_property:
//
// This is the legacy entry point to update a property attached to the
// plane.
//
// This callback is optional if the driver does not support any legacy
// driver-private properties. For atomic drivers it is not used because
// property handling is done entirely in the DRM core.
//
// RETURNS:
//
// 0 on success or a negative error code on failure.
//
    pub val): *mut *mut drm_property property, uint64_t,
//
// @atomic_create_state:
//
// Allocate a pristine, initialized, state for the plane object
// and return it. This callback must have no side effects: in
// particular, the returned state must not be assigned to the
// object's state pointer and it must not affect the hardware
// state.
//
// RETURNS:
//
// A new, pristine, plane state instance or an error pointer
// on failure.
//
    pub plane): *mut *mut *mut drm_plane_state (atomic_create_state)(drm_plane,
//
// @atomic_duplicate_state:
//
// Duplicate the current atomic state for this plane and return it.
// The core and helpers guarantee that any atomic state duplicated with
// this hook and still owned by the caller (i.e. not transferred to the
// driver by calling &drm_mode_config_funcs.atomic_commit) will be
// cleaned up by calling the @atomic_destroy_state hook in this
// structure.
//
// This callback is mandatory for atomic drivers.
//
// Atomic drivers which don't subclass &struct drm_plane_state should use
// drm_atomic_helper_plane_duplicate_state(). Drivers that subclass the
// state structure to extend it with driver-private state should use
// __drm_atomic_helper_plane_duplicate_state() to make sure shared state is
// duplicated in a consistent fashion across drivers.
//
// It is an error to call this hook before &drm_plane.state has been
// initialized correctly.
//
// NOTE:
//
// If the duplicate state references refcounted resources this hook must
// acquire a reference for each of them. The driver must release these
// references again in @atomic_destroy_state.
//
// RETURNS:
//
// Duplicated atomic state or NULL when the allocation failed.
//
    pub plane): *mut *mut *mut drm_plane_state (atomic_duplicate_state)(drm_plane,
//
// @atomic_destroy_state:
//
// Destroy a state duplicated with @atomic_duplicate_state and release
// or unreference all resources it references
//
// This callback is mandatory for atomic drivers.
//
    pub state): *mut drm_plane_state,
//
// @atomic_set_property:
//
// Decode a driver-private property value and store the decoded value
// into the passed-in state structure. Since the atomic core decodes all
// standardized properties (even for extensions beyond the core set of
// properties which might not be implemented by all drivers) this
// requires drivers to subclass the state structure.
//
// Such driver-private properties should really only be implemented for
// truly hardware/vendor specific state. Instead it is preferred to
// standardize atomic extension and decode the properties used to expose
// such an extension in the core.
//
// Do not call this function directly, use
// drm_atomic_plane_set_property() instead.
//
// This callback is optional if the driver does not support any
// driver-private atomic properties.
//
// NOTE:
//
// This function is called in the state assembly phase of atomic
// modesets, which can be aborted for any reason (including on
// userspace's request to just check whether a configuration would be
// possible). Drivers MUST NOT touch any persistent state (hardware or
// software) or data structures except the passed in @state parameter.
//
// Also since userspace controls in which order properties are set this
// function must not do any input validation (since the state update is
// incomplete and hence likely inconsistent). Instead any such input
// validation must be done in the various atomic_check callbacks.
//
// RETURNS:
//
// 0 if the property has been found, -EINVAL if the property isn't
// implemented by the driver (which shouldn't ever happen, the core only
// asks for properties attached to this plane). No other validation is
// allowed by the driver. The core already checks that the property
// value is within the range (integer, valid enum value, ...) the driver
// set when registering the property.
//
    pub val): u64,
//
// @atomic_get_property:
//
// Reads out the decoded driver-private property. This is used to
// implement the GETPLANE IOCTL.
//
// Do not call this function directly, use
// drm_atomic_plane_get_property() instead.
//
// This callback is optional if the driver does not support any
// driver-private atomic properties.
//
// RETURNS:
//
// 0 on success, -EINVAL if the property isn't implemented by the
// driver (which should never happen, the core only asks for
// properties attached to this plane).
//
    pub val): *mut u64,
//
// @late_register:
//
// This optional hook can be used to register additional userspace
// interfaces attached to the plane like debugfs interfaces.
// It is called late in the driver load sequence from drm_dev_register().
// Everything added from this callback should be unregistered in
// the early_unregister callback.
//
// Returns:
//
// 0 on success, or a negative error code on failure.
//
    pub plane): *mut *mut int (late_register)(struct drm_plane,
//
// @early_unregister:
//
// This optional hook should be used to unregister the additional
// userspace interfaces attached to the plane from
// @late_register. It is called from drm_dev_unregister(),
// early in the driver unload sequence to disable userspace access
// before data structures are torndown.
//
    pub plane): *mut *mut void (early_unregister)(struct drm_plane,
//
// @atomic_print_state:
//
// If driver subclasses &struct drm_plane_state, it should implement
// this optional hook for printing additional driver specific state.
//
// Do not call this directly, use drm_atomic_plane_print_state()
// instead.
//
    pub state): *const drm_plane_state,
//
// @format_mod_supported:
//
// This optional hook is used for the DRM to determine if the given
// format/modifier combination is valid for the plane. This allows the
// DRM to generate the correct format bitmask (which formats apply to
// which modifier), and to validate modifiers at atomic_check time.
//
// If not present, then any modifier in the plane's modifier
// list is allowed with any of the plane's formats.
//
// Returns:
//
// True if the given modifier is valid for that format on the plane.
// False otherwise.
//
    pub modifier): u64,
//
// @format_mod_supported_async:
//
// This optional hook is used for the DRM to determine if for
// asynchronous flip the given format/modifier combination is valid for
// the plane. This allows the DRM to generate the correct format
// bitmask (which formats apply to which modifier), and to validate
// modifiers at atomic_check time.
//
// Returns:
//
// True if the given modifier is valid for that format on the plane.
// False otherwise.
//
    pub modifier): u32 format, u64,
}

//
// enum drm_plane_type - uapi plane type enumeration
//
// For historical reasons not all planes are made the same. This enumeration is
// used to tell the different types of planes apart to implement the different
// uapi semantics for them. For userspace which is universal plane aware and
// which is using that atomic IOCTL there's no difference between these planes
// (beyong what the driver and hardware can support of course).
//
// For compatibility with legacy userspace, only overlay planes are made
// available to userspace by default. Userspace clients may set the
// &DRM_CLIENT_CAP_UNIVERSAL_PLANES client capability bit to indicate that they
// wish to receive a universal plane list containing all plane types. See also
// drm_for_each_legacy_plane().
//
// In addition to setting each plane's type, drivers need to setup the
// &drm_crtc.primary and optionally &drm_crtc.cursor pointers for legacy
// IOCTLs. See drm_crtc_init_with_planes().
//
// WARNING: The values of this enum is UABI since they're exposed in the "type"
// property.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_plane_type {
//
// @DRM_PLANE_TYPE_OVERLAY:
//
// Overlay planes represent all non-primary, non-cursor planes. Some
// drivers refer to these types of planes as "sprites" internally.
//
    DRM_PLANE_TYPE_OVERLAY,

//
// @DRM_PLANE_TYPE_PRIMARY:
//
// A primary plane attached to a CRTC is the most likely to be able to
// light up the CRTC when no scaling/cropping is used and the plane
// covers the whole CRTC.
//
    DRM_PLANE_TYPE_PRIMARY,

//
// @DRM_PLANE_TYPE_CURSOR:
//
// A cursor plane attached to a CRTC is more likely to be able to be
// enabled when no scaling/cropping is used and the framebuffer has the
// size indicated by &drm_mode_config.cursor_width and
// &drm_mode_config.cursor_height. Additionally, if the driver doesn't
// support modifiers, the framebuffer should have a linear layout.
//
    DRM_PLANE_TYPE_CURSOR,
}

//
// struct drm_plane - central DRM plane control structure
//
// Planes represent the scanout hardware of a display block. They receive their
// input data from a &drm_framebuffer and feed it to a &drm_crtc. Planes control
// the color conversion, see `Plane Composition Properties`_ for more details,
// and are also involved in the color conversion of input pixels, see `Color
// Management Properties`_ for details on that.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_plane {
// @dev: DRM device this plane belongs to
    pub dev: *mut drm_device,
//
// @head:
//
// List of all planes on @dev, linked from &drm_mode_config.plane_list.
// Invariant over the lifetime of @dev and therefore does not need
// locking.
//
    pub head: list_head,
// @name: human readable name, can be overwritten by the driver
    pub name: *mut c_char,
//
// @mutex:
//
// Protects modeset plane state, together with the &drm_crtc.mutex of
// CRTC this plane is linked to (when active, getting activated or
// getting disabled).
//
// For atomic drivers specifically this protects @state.
//
    pub mutex: drm_modeset_lock,
// @base: base mode object
    pub base: drm_mode_object,
//
// @possible_crtcs: pipes this plane can be bound to constructed from
// drm_crtc_mask()
//
    pub possible_crtcs: u32,
// @format_types: array of formats supported by this plane
    pub format_types: *mut u32,
// @format_count: Size of the array pointed at by @format_types.
    pub format_count: c_uint,
//
// @format_default: driver hasn't supplied supported formats for the
// plane. Used by the non-atomic driver compatibility wrapper only.
//
    pub format_default: bool,
// @modifiers: array of modifiers supported by this plane
    pub modifiers: *mut u64,
// @modifier_count: Size of the array pointed at by @modifier_count.
    pub modifier_count: c_uint,
//
// @crtc:
//
// Currently bound CRTC, only meaningful for non-atomic drivers. For
// atomic drivers this is forced to be NULL, atomic drivers should
// instead check &drm_plane_state.crtc.
//
    pub crtc: *mut drm_crtc,
//
// @fb:
//
// Currently bound framebuffer, only meaningful for non-atomic drivers.
// For atomic drivers this is forced to be NULL, atomic drivers should
// instead check &drm_plane_state.fb.
//
    pub fb: *mut drm_framebuffer,
//
// @old_fb:
//
// Temporary tracking of the old fb while a modeset is ongoing. Only
// used by non-atomic drivers, forced to be NULL for atomic drivers.
//
    pub old_fb: *mut drm_framebuffer,
// @funcs: plane control functions
    pub funcs: *const drm_plane_funcs,
// @properties: property tracking for this plane
    pub properties: drm_object_properties,
// @type: Type of plane, see &enum drm_plane_type for details.
    pub type: drm_plane_type,
//
// @index: Position inside the mode_config.list, can be used as an array
// index. It is invariant over the lifetime of the plane.
//
    pub index: unsigned,
// @helper_private: mid-layer private data
    pub helper_private: *const drm_plane_helper_funcs,
//
// @state:
//
// Current atomic state for this plane.
//
// This is protected by @mutex. Note that nonblocking atomic commits
// access the current plane state without taking locks. Either by going
// through the &struct drm_atomic_commit pointers, see
// for_each_oldnew_plane_in_state(), for_each_old_plane_in_state() and
// for_each_new_plane_in_state(). Or through careful ordering of atomic
// commit operations as implemented in the atomic helpers, see
// &struct drm_crtc_commit.
//
    pub state: *mut drm_plane_state,
//
// @alpha_property:
// Optional alpha property for this plane. See
// drm_plane_create_alpha_property().
//
    pub alpha_property: *mut drm_property,
//
// @zpos_property:
// Optional zpos property for this plane. See
// drm_plane_create_zpos_property().
//
    pub zpos_property: *mut drm_property,
//
// @rotation_property:
// Optional rotation property for this plane. See
// drm_plane_create_rotation_property().
//
    pub rotation_property: *mut drm_property,
//
// @blend_mode_property:
// Optional "pixel blend mode" enum property for this plane.
// Blend mode property represents the alpha blending equation selection,
// describing how the pixels from the current plane are composited with
// the background.
//
    pub blend_mode_property: *mut drm_property,
//
// @color_encoding_property:
//
// Optional "COLOR_ENCODING" enum property for specifying
// color encoding for non RGB formats.
// See drm_plane_create_color_properties().
//
    pub color_encoding_property: *mut drm_property,
//
// @color_range_property:
//
// Optional "COLOR_RANGE" enum property for specifying
// color range for non RGB formats.
// See drm_plane_create_color_properties().
//
    pub color_range_property: *mut drm_property,
//
// @color_pipeline_property:
//
// Optional "COLOR_PIPELINE" enum property for specifying
// a color pipeline to use on the plane.
//
    pub color_pipeline_property: *mut drm_property,
//
// @scaling_filter_property: property to apply a particular filter while
// scaling.
//
    pub scaling_filter_property: *mut drm_property,
//
// @hotspot_x_property: property to set mouse hotspot x offset.
//
    pub hotspot_x_property: *mut drm_property,
//
// @hotspot_y_property: property to set mouse hotspot y offset.
//
    pub hotspot_y_property: *mut drm_property,
//
// @kmsg_panic: Used to register a panic notifier for this plane
//
    pub kmsg_panic: kmsg_dumper,
}

extern "C" {
    pub fn drm_plane_cleanup(plane: *mut drm_plane);
}
//
// drmm_universal_plane_alloc - Allocate and initialize an universal plane object
// @dev: DRM device
// @type: the type of the struct which contains struct &drm_plane
// @member: the name of the &drm_plane within @type
// @possible_crtcs: bitmask of possible CRTCs
// @funcs: callbacks for the new plane
// @formats: array of supported formats (DRM_FORMAT\_\*)
// @format_count: number of elements in @formats
// @format_modifiers: array of struct drm_format modifiers terminated by
// DRM_FORMAT_MOD_INVALID
// @plane_type: type of plane (overlay, primary, cursor)
// @name: printf style format string for the plane name, or NULL for default name
//
// Allocates and initializes a plane object of type @type. Cleanup is
// automatically handled through registering drm_plane_cleanup() with
// drmm_add_action().
//
// The @drm_plane_funcs.destroy hook must be NULL.
//
// Drivers that only support the DRM_FORMAT_MOD_LINEAR modifier support may set
// @format_modifiers to NULL. The plane will advertise the linear modifier.
//
// Returns:
// Pointer to new plane, or ERR_PTR on failure.
//

//
// drm_universal_plane_alloc() - Allocate and initialize an universal plane object
// @dev: DRM device
// @type: the type of the struct which contains struct &drm_plane
// @member: the name of the &drm_plane within @type
// @possible_crtcs: bitmask of possible CRTCs
// @funcs: callbacks for the new plane
// @formats: array of supported formats (DRM_FORMAT\_\*)
// @format_count: number of elements in @formats
// @format_modifiers: array of struct drm_format modifiers terminated by
// DRM_FORMAT_MOD_INVALID
// @plane_type: type of plane (overlay, primary, cursor)
// @name: printf style format string for the plane name, or NULL for default name
//
// Allocates and initializes a plane object of type @type. The caller
// is responsible for releasing the allocated memory with kfree().
//
// Drivers are encouraged to use drmm_universal_plane_alloc() instead.
//
// Drivers that only support the DRM_FORMAT_MOD_LINEAR modifier support may set
// @format_modifiers to NULL. The plane will advertise the linear modifier.
//
// Returns:
// Pointer to new plane, or ERR_PTR on failure.
//

//
// drm_plane_index - find the index of a registered plane
// @plane: plane to find index for
//
// Given a registered plane, return the index of that plane within a DRM
// device's list of planes.
//
// drm_plane_mask - find the mask of a registered plane
// @plane: plane to find mask for
//
extern "C" {
    pub fn drm_plane_from_index(dev: *mut drm_device, idx: c_int) -> *mut drm_plane;
}
extern "C" {
    pub fn drm_plane_force_disable(plane: *mut drm_plane);
}
//
// drm_plane_find - find a &drm_plane
// @dev: DRM device
// @file_priv: drm file to check for lease against.
// @id: plane id
//
// Returns the plane with @id, NULL if it doesn't exist. Simple wrapper around
// drm_mode_object_find().
//
// drm_for_each_plane_mask - iterate over planes specified by bitmask
// @plane: the loop cursor
// @dev: the DRM device
// @plane_mask: bitmask of plane indices
//
// Iterate over all planes specified by bitmask.
//

//
// drm_for_each_legacy_plane - iterate over all planes for legacy userspace
// @plane: the loop cursor
// @dev: the DRM device
//
// Iterate over all legacy planes of @dev, excluding primary and cursor planes.
// This is useful for implementing userspace apis when userspace is not
// universal plane aware. See also &enum drm_plane_type.
//

//
// drm_for_each_plane - iterate over all planes
// @plane: the loop cursor
// @dev: the DRM device
//
// Iterate over all planes of @dev, include primary and cursor planes.
//

extern "C" {
    pub fn drm_plane_enable_fb_damage_clips(plane: *mut drm_plane);
}
