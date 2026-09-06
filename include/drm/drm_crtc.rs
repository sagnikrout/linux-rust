//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/drm_crtc.h
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
// Copyright © 2006 Keith Packard
// Copyright © 2007-2008 Dave Airlie
// Copyright © 2007-2008 Intel Corporation
// Jesse Barnes <jesse.barnes@intel.com>
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

//
// struct drm_crtc_state - mutable CRTC state
//
// Note that the distinction between @enable and @active is rather subtle:
// Flipping @active while @enable is set without changing anything else may
// never return in a failure from the &drm_mode_config_funcs.atomic_check
// callback. Userspace assumes that a DPMS On will always succeed. In other
// words: @enable controls resource assignment, @active controls the actual
// hardware state.
//
// The three booleans active_changed, connectors_changed and mode_changed are
// intended to indicate whether a full modeset is needed, rather than strictly
// describing what has changed in a commit. See also:
// drm_atomic_crtc_needs_modeset()
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_crtc_state {
// @crtc: backpointer to the CRTC
    pub crtc: *mut drm_crtc,
//
// @enable: Whether the CRTC should be enabled, gates all other state.
// This controls reservations of shared resources. Actual hardware state
// is controlled by @active.
//
    pub enable: bool,
//
// @active: Whether the CRTC is actively displaying (used for DPMS).
// Implies that @enable is set. The driver must not release any shared
// resources if @active is set to false but @enable still true, because
// userspace expects that a DPMS ON always succeeds.
//
// Hence drivers must not consult @active in their various
// &drm_mode_config_funcs.atomic_check callback to reject an atomic
// commit. They can consult it to aid in the computation of derived
// hardware state, since even in the DPMS OFF state the display hardware
// should be as much powered down as when the CRTC is completely
// disabled through setting @enable to false.
//
    pub active: bool,
//
// @planes_changed: Planes on this crtc are updated. Used by the atomic
// helpers and drivers to steer the atomic commit control flow.
//
    pub 1: bool planes_changed :,
//
// @mode_changed: @mode or @enable has been changed. Used by the atomic
// helpers and drivers to steer the atomic commit control flow. See also
// drm_atomic_crtc_needs_modeset().
//
// Drivers are supposed to set this for any CRTC state changes that
// require a full modeset. They can also reset it to false if e.g. a
// @mode change can be done without a full modeset by only changing
// scaler settings.
//
    pub 1: bool mode_changed :,
//
// @active_changed: @active has been toggled. Used by the atomic
// helpers and drivers to steer the atomic commit control flow. See also
// drm_atomic_crtc_needs_modeset().
//
    pub 1: bool active_changed :,
//
// @connectors_changed: Connectors to this crtc have been updated,
// either in their state or routing. Used by the atomic
// helpers and drivers to steer the atomic commit control flow. See also
// drm_atomic_crtc_needs_modeset().
//
// Drivers are supposed to set this as-needed from their own atomic
// check code, e.g. from &drm_encoder_helper_funcs.atomic_check
//
    pub 1: bool connectors_changed :,
//
// @zpos_changed: zpos values of planes on this crtc have been updated.
// Used by the atomic helpers and drivers to steer the atomic commit
// control flow.
//
    pub 1: bool zpos_changed :,
//
// @color_mgmt_changed: Color management properties have changed
// (@gamma_lut, @degamma_lut or @ctm). Used by the atomic helpers and
// drivers to steer the atomic commit control flow.
//
    pub 1: bool color_mgmt_changed :,
//
// @no_vblank:
//
// Reflects the ability of a CRTC to send VBLANK events. This state
// usually depends on the pipeline configuration. If set to true, DRM
// atomic helpers will send out a fake VBLANK event during display
// updates after all hardware changes have been committed. This is
// implemented in drm_atomic_helper_fake_vblank().
//
// One usage is for drivers and/or hardware without support for VBLANK
// interrupts. Such drivers typically do not initialize vblanking
// (i.e., call drm_vblank_init() with the number of CRTCs). For CRTCs
// without initialized vblanking, this field is set to true in
// drm_atomic_helper_check_modeset(), and a fake VBLANK event will be
// send out on each update of the display pipeline by
// drm_atomic_helper_fake_vblank().
//
// Another usage is CRTCs feeding a writeback connector operating in
// oneshot mode. In this case the fake VBLANK event is only generated
// when a job is queued to the writeback connector, and we want the
// core to fake VBLANK events when this part of the pipeline hasn't
// changed but others had or when the CRTC and connectors are being
// disabled.
//
// __drm_atomic_helper_crtc_duplicate_state() will not reset the value
// from the current state, the CRTC driver is then responsible for
// updating this field when needed.
//
// Note that the combination of &drm_crtc_state.event == NULL and
// &drm_crtc_state.no_blank == true is valid and usually used when the
// writeback connector attached to the CRTC has a new job queued. In
// this case the driver will send the VBLANK event on its own when the
// writeback job is complete.
//
    pub no_vblank: bool,
//
// @plane_mask: Bitmask of drm_plane_mask(plane) of planes attached to
// this CRTC.
//
    pub plane_mask: u32,
//
// @connector_mask: Bitmask of drm_connector_mask(connector) of
// connectors attached to this CRTC.
//
    pub connector_mask: u32,
//
// @encoder_mask: Bitmask of drm_encoder_mask(encoder) of encoders
// attached to this CRTC.
//
    pub encoder_mask: u32,
//
// @adjusted_mode:
//
// Internal display timings which can be used by the driver to handle
// differences between the mode requested by userspace in @mode and what
// is actually programmed into the hardware.
//
// For drivers using &drm_bridge, this stores hardware display timings
// used between the CRTC and the first bridge. For other drivers, the
// meaning of the adjusted_mode field is purely driver implementation
// defined information, and will usually be used to store the hardware
// display timings used between the CRTC and encoder blocks.
//
    pub adjusted_mode: drm_display_mode,
//
// @mode:
//
// Display timings requested by userspace. The driver should try to
// match the refresh rate as close as possible (but note that it's
// undefined what exactly is close enough, e.g. some of the HDMI modes
// only differ in less than 1% of the refresh rate). The active width
// and height as observed by userspace for positioning planes must match
// exactly.
//
// For external connectors where the sink isn't fixed (like with a
// built-in panel), this mode here should match the physical mode on the
// wire to the last details (i.e. including sync polarities and
// everything).
//
    pub mode: drm_display_mode,
//
// @mode_blob: &drm_property_blob for @mode, for exposing the mode to
// atomic userspace.
//
    pub mode_blob: *mut drm_property_blob,
//
// @degamma_lut:
//
// Lookup table for converting framebuffer pixel data before apply the
// color conversion matrix @ctm. See drm_crtc_enable_color_mgmt(). The
// blob (if not NULL) is an array of &struct drm_color_lut.
//
    pub degamma_lut: *mut drm_property_blob,
//
// @ctm:
//
// Color transformation matrix. See drm_crtc_enable_color_mgmt(). The
// blob (if not NULL) is a &struct drm_color_ctm.
//
    pub ctm: *mut drm_property_blob,
//
// @gamma_lut:
//
// Lookup table for converting pixel data after the color conversion
// matrix @ctm.  See drm_crtc_enable_color_mgmt(). The blob (if not
// NULL) is an array of &struct drm_color_lut.
//
// Note that for mostly historical reasons stemming from Xorg heritage,
// this is also used to store the color map (also sometimes color lut,
// CLUT or color palette) for indexed formats like DRM_FORMAT_C8.
//
    pub gamma_lut: *mut drm_property_blob,
//
// @background_color:
//
// RGB value representing the CRTC's background color.  The background
// color (aka "canvas color") of a CRTC is the color that will be used
// for pixels not covered by a plane, or covered by transparent pixels
// of a plane.  The value here should be built using DRM_ARGB64_PREP*()
// helpers, while the individual color components can be extracted with
// desired precision via the DRM_ARGB64_GET*() macros.
//
    pub background_color: u64,
//
// @target_vblank:
//
// Target vertical blank period when a page flip
// should take effect.
//
    pub target_vblank: u32,
//
// @async_flip:
//
// This is set when DRM_MODE_PAGE_FLIP_ASYNC is set in the legacy
// PAGE_FLIP IOCTL. It's not wired up for the atomic IOCTL itself yet.
//
    pub async_flip: bool,
//
// @vrr_enabled:
//
// Indicates if variable refresh rate should be enabled for the CRTC.
// Support for the requested vrr state will depend on driver and
// hardware capabiltiy - lacking support is not treated as failure.
//
    pub vrr_enabled: bool,
//
// @self_refresh_active:
//
// Used by the self refresh helpers to denote when a self refresh
// transition is occurring. This will be set on enable/disable callbacks
// when self refresh is being enabled or disabled. In some cases, it may
// not be desirable to fully shut off the crtc during self refresh.
// CRTC's can inspect this flag and determine the best course of action.
//
    pub self_refresh_active: bool,
//
// @scaling_filter:
//
// Scaling filter to be applied
//
    pub scaling_filter: drm_scaling_filter,
//
// @sharpness_strength:
//
// Used by the user to set the sharpness intensity.
// The value ranges from 0-255.
// Default value is 0 which disable the sharpness feature.
// Any value greater than 0 enables sharpening with the
// specified strength.
//
    pub sharpness_strength: u8,
//
// @event:
//
// Optional pointer to a DRM event to signal upon completion of the
// state update. The driver must send out the event when the atomic
// commit operation completes. There are two cases:
//
// - The event is for a CRTC which is being disabled through this
// atomic commit. In that case the event can be send out any time
// after the hardware has stopped scanning out the current
// framebuffers. It should contain the timestamp and counter for the
// last vblank before the display pipeline was shut off. The simplest
// way to achieve that is calling drm_crtc_send_vblank_event()
// somewhen after drm_crtc_vblank_off() has been called.
//
// - For a CRTC which is enabled at the end of the commit (even when it
// undergoes an full modeset) the vblank timestamp and counter must
// be for the vblank right before the first frame that scans out the
// new set of buffers. Again the event can only be sent out after the
// hardware has stopped scanning out the old buffers.
//
// - Events for disabled CRTCs are not allowed, and drivers can ignore
// that case.
//
// For very simple hardware without VBLANK interrupt, enabling
// &struct drm_crtc_state.no_vblank makes DRM's atomic commit helpers
// send a fake VBLANK event at the end of the display update after all
// hardware changes have been applied. See
// drm_atomic_helper_fake_vblank().
//
// For more complex hardware this
// can be handled by the drm_crtc_send_vblank_event() function,
// which the driver should call on the provided event upon completion of
// the atomic commit. Note that if the driver supports vblank signalling
// and timestamping the vblank counters and timestamps must agree with
// the ones returned from page flip events. With the current vblank
// helper infrastructure this can be achieved by holding a vblank
// reference while the page flip is pending, acquired through
// drm_crtc_vblank_get() and released with drm_crtc_vblank_put().
// Drivers are free to implement their own vblank counter and timestamp
// tracking though, e.g. if they have accurate timestamp registers in
// hardware.
//
// For hardware which supports some means to synchronize vblank
// interrupt delivery with committing display state there's also
// drm_crtc_arm_vblank_event(). See the documentation of that function
// for a detailed discussion of the constraints it needs to be used
// safely.
//
// If the device can't notify of flip completion in a race-free way
// at all, then the event should be armed just after the page flip is
// committed. In the worst case the driver will send the event to
// userspace one frame too late. This doesn't allow for a real atomic
// update, but it should avoid tearing.
//
    pub event: *mut drm_pending_vblank_event,
//
// @commit:
//
// This tracks how the commit for this update proceeds through the
// various phases. This is never cleared, except when we destroy the
// state, so that subsequent commits can synchronize with previous ones.
//
    pub commit: *mut drm_crtc_commit,
// @state: backpointer to global drm_atomic_commit
    pub state: *mut drm_atomic_commit,
}

//
// struct drm_crtc_funcs - control CRTCs for a given device
//
// The drm_crtc_funcs structure is the central CRTC management structure
// in the DRM.  Each CRTC controls one or more connectors (note that the name
// CRTC is simply historical, a CRTC may control LVDS, VGA, DVI, TV out, etc.
// connectors, not just CRTs).
//
// Each driver is responsible for filling out this structure at startup time,
// in addition to providing other modesetting features, like i2c and DDC
// bus accessors.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_crtc_funcs {
//
// @reset:
//
// Reset CRTC hardware and software state to off. This function isn't
// called by the core directly, only through drm_mode_config_reset().
// It's not a helper hook only for historical reasons.
//
// Atomic drivers can use drm_atomic_helper_crtc_reset() to reset
// atomic state using this hook.
//
    pub crtc): *mut *mut void (reset)(struct drm_crtc,
//
// @cursor_set:
//
// Update the cursor image. The cursor position is relative to the CRTC
// and can be partially or fully outside of the visible area.
//
// Note that contrary to all other KMS functions the legacy cursor entry
// points don't take a framebuffer object, but instead take directly a
// raw buffer object id from the driver's buffer manager (which is
// either GEM or TTM for current drivers).
//
// This entry point is deprecated, drivers should instead implement
// universal plane support and register a proper cursor plane using
// drm_crtc_init_with_planes().
//
// This callback is optional
//
// RETURNS:
//
// 0 on success or a negative error code on failure.
//
    pub height): uint32_t handle, uint32_t width, uint32_t,
//
// @cursor_set2:
//
// Update the cursor image, including hotspot information. The hotspot
// must not affect the cursor position in CRTC coordinates, but is only
// meant as a hint for virtualized display hardware to coordinate the
// guests and hosts cursor position. The cursor hotspot is relative to
// the cursor image. Otherwise this works exactly like @cursor_set.
//
// This entry point is deprecated, drivers should instead implement
// universal plane support and register a proper cursor plane using
// drm_crtc_init_with_planes().
//
// This callback is optional.
//
// RETURNS:
//
// 0 on success or a negative error code on failure.
//
    pub hot_y): int32_t hot_x, int32_t,
//
// @cursor_move:
//
// Update the cursor position. The cursor does not need to be visible
// when this hook is called.
//
// This entry point is deprecated, drivers should instead implement
// universal plane support and register a proper cursor plane using
// drm_crtc_init_with_planes().
//
// This callback is optional.
//
// RETURNS:
//
// 0 on success or a negative error code on failure.
//
    pub y): *mut *mut *mut int (cursor_move)(struct drm_crtc crtc, int x, int,
//
// @gamma_set:
//
// Set gamma on the CRTC.
//
// This callback is optional.
//
// Atomic drivers who want to support gamma tables should implement the
// atomic color management support, enabled by calling
// drm_crtc_enable_color_mgmt(), which then supports the legacy gamma
// interface through the drm_atomic_helper_legacy_gamma_set()
// compatibility implementation.
//
    pub ctx): *mut drm_modeset_acquire_ctx,
//
// @destroy:
//
// Clean up CRTC resources. This is only called at driver unload time
// through drm_mode_config_cleanup() since a CRTC cannot be hotplugged
// in DRM.
//
    pub crtc): *mut *mut void (destroy)(struct drm_crtc,
//
// @set_config:
//
// This is the main legacy entry point to change the modeset state on a
// CRTC. All the details of the desired configuration are passed in a
// &struct drm_mode_set - see there for details.
//
// Drivers implementing atomic modeset should use
// drm_atomic_helper_set_config() to implement this hook.
//
// RETURNS:
//
// 0 on success or a negative error code on failure.
//
    pub ctx): *mut drm_modeset_acquire_ctx,
//
// @page_flip:
//
// Legacy entry point to schedule a flip to the given framebuffer.
//
// Page flipping is a synchronization mechanism that replaces the frame
// buffer being scanned out by the CRTC with a new frame buffer during
// vertical blanking, avoiding tearing (except when requested otherwise
// through the DRM_MODE_PAGE_FLIP_ASYNC flag). When an application
// requests a page flip the DRM core verifies that the new frame buffer
// is large enough to be scanned out by the CRTC in the currently
// configured mode and then calls this hook with a pointer to the new
// frame buffer.
//
// The driver must wait for any pending rendering to the new framebuffer
// to complete before executing the flip. It should also wait for any
// pending rendering from other drivers if the underlying buffer is a
// shared dma-buf.
//
// An application can request to be notified when the page flip has
// completed. The drm core will supply a &struct drm_event in the event
// parameter in this case. This can be handled by the
// drm_crtc_send_vblank_event() function, which the driver should call on
// the provided event upon completion of the flip. Note that if
// the driver supports vblank signalling and timestamping the vblank
// counters and timestamps must agree with the ones returned from page
// flip events. With the current vblank helper infrastructure this can
// be achieved by holding a vblank reference while the page flip is
// pending, acquired through drm_crtc_vblank_get() and released with
// drm_crtc_vblank_put(). Drivers are free to implement their own vblank
// counter and timestamp tracking though, e.g. if they have accurate
// timestamp registers in hardware.
//
// This callback is optional.
//
// NOTE:
//
// Very early versions of the KMS ABI mandated that the driver must
// block (but not reject) any rendering to the old framebuffer until the
// flip operation has completed and the old framebuffer is no longer
// visible. This requirement has been lifted, and userspace is instead
// expected to request delivery of an event and wait with recycling old
// buffers until such has been received.
//
// RETURNS:
//
// 0 on success or a negative error code on failure. Note that if a
// page flip operation is already pending the callback should return
// -EBUSY. Pageflips on a disabled CRTC (either by setting a NULL mode
// or just runtime disabled through DPMS respectively the new atomic
// "ACTIVE" state) should result in an -EINVAL error code. Note that
// drm_atomic_helper_page_flip() checks this already for atomic drivers.
//
    pub ctx): *mut drm_modeset_acquire_ctx,
//
// @page_flip_target:
//
// Same as @page_flip but with an additional parameter specifying the
// absolute target vertical blank period (as reported by
// drm_crtc_vblank_count()) when the flip should take effect.
//
// Note that the core code calls drm_crtc_vblank_get before this entry
// point, and will call drm_crtc_vblank_put if this entry point returns
// any non-0 error code. It's the driver's responsibility to call
// drm_crtc_vblank_put after this entry point returns 0, typically when
// the flip completes.
//
    pub ctx): *mut drm_modeset_acquire_ctx,
//
// @set_property:
//
// This is the legacy entry point to update a property attached to the
// CRTC.
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
// Allocate a pristine, initialized, state for the CRTC object
// and return it. This callback must have no side effects: in
// particular, the returned state must not be assigned to the
// object's state pointer and it must not affect the hardware
// state.
//
// RETURNS:
//
// A new, pristine, CRTC state instance or an error pointer
// on failure.
//
    pub crtc): *mut *mut *mut drm_crtc_state (atomic_create_state)(drm_crtc,
//
// @atomic_duplicate_state:
//
// Duplicate the current atomic state for this CRTC and return it.
// The core and helpers guarantee that any atomic state duplicated with
// this hook and still owned by the caller (i.e. not transferred to the
// driver by calling &drm_mode_config_funcs.atomic_commit) will be
// cleaned up by calling the @atomic_destroy_state hook in this
// structure.
//
// This callback is mandatory for atomic drivers.
//
// Atomic drivers which don't subclass &struct drm_crtc_state should use
// drm_atomic_helper_crtc_duplicate_state(). Drivers that subclass the
// state structure to extend it with driver-private state should use
// __drm_atomic_helper_crtc_duplicate_state() to make sure shared state is
// duplicated in a consistent fashion across drivers.
//
// It is an error to call this hook before &drm_crtc.state has been
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
    pub crtc): *mut *mut *mut drm_crtc_state (atomic_duplicate_state)(drm_crtc,
//
// @atomic_destroy_state:
//
// Destroy a state duplicated with @atomic_duplicate_state and release
// or unreference all resources it references
//
// This callback is mandatory for atomic drivers.
//
    pub state): *mut drm_crtc_state,
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
// drm_atomic_crtc_set_property() instead.
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
// implemented by the driver (which should never happen, the core only
// asks for properties attached to this CRTC). No other validation is
// allowed by the driver. The core already checks that the property
// value is within the range (integer, valid enum value, ...) the driver
// set when registering the property.
//
    pub val): u64,
//
// @atomic_get_property:
//
// Reads out the decoded driver-private property. This is used to
// implement the GETCRTC IOCTL.
//
// Do not call this function directly, use
// drm_atomic_crtc_get_property() instead.
//
// This callback is optional if the driver does not support any
// driver-private atomic properties.
//
// RETURNS:
//
// 0 on success, -EINVAL if the property isn't implemented by the
// driver (which should never happen, the core only asks for
// properties attached to this CRTC).
//
    pub val): *mut u64,
//
// @late_register:
//
// This optional hook can be used to register additional userspace
// interfaces attached to the crtc like debugfs interfaces.
// It is called late in the driver load sequence from drm_dev_register().
// Everything added from this callback should be unregistered in
// the early_unregister callback.
//
// Returns:
//
// 0 on success, or a negative error code on failure.
//
    pub crtc): *mut *mut int (late_register)(struct drm_crtc,
//
// @early_unregister:
//
// This optional hook should be used to unregister the additional
// userspace interfaces attached to the crtc from
// @late_register. It is called from drm_dev_unregister(),
// early in the driver unload sequence to disable userspace access
// before data structures are torndown.
//
    pub crtc): *mut *mut void (early_unregister)(struct drm_crtc,
//
// @set_crc_source:
//
// Changes the source of CRC checksums of frames at the request of
// userspace, typically for testing purposes. The sources available are
// specific of each driver and a %NULL value indicates that CRC
// generation is to be switched off.
//
// When CRC generation is enabled, the driver should call
// drm_crtc_add_crc_entry() at each frame, providing any information
// that characterizes the frame contents in the crcN arguments, as
// provided from the configured source. Drivers must accept an "auto"
// source name that will select a default source for this CRTC.
//
// This may trigger an atomic modeset commit if necessary, to enable CRC
// generation.
//
// Note that "auto" can depend upon the current modeset configuration,
// e.g. it could pick an encoder or output specific CRC sampling point.
//
// This callback is optional if the driver does not support any CRC
// generation functionality.
//
// RETURNS:
//
// 0 on success or a negative error code on failure.
//
    pub source): *const *const *const int (set_crc_source)(struct drm_crtc crtc, char,
//
// @verify_crc_source:
//
// verifies the source of CRC checksums of frames before setting the
// source for CRC and during crc open. Source parameter can be NULL
// while disabling crc source.
//
// This callback is optional if the driver does not support any CRC
// generation functionality.
//
// RETURNS:
//
// 0 on success or a negative error code on failure.
//
    pub values_cnt): *mut usize,
//
// @get_crc_sources:
//
// Driver callback for getting a list of all the available sources for
// CRC generation. This callback depends upon verify_crc_source, So
// verify_crc_source callback should be implemented before implementing
// this. Driver can pass full list of available crc sources, this
// callback does the verification on each crc-source before passing it
// to userspace.
//
// This callback is optional if the driver does not support exporting of
// possible CRC sources list.
//
// RETURNS:
//
// a constant character pointer to the list of all the available CRC
// sources. On failure driver should return NULL. count should be
// updated with number of sources in list. if zero we don't process any
// source from the list.
//
    pub count): *mut usize,
//
// @atomic_print_state:
//
// If driver subclasses &struct drm_crtc_state, it should implement
// this optional hook for printing additional driver specific state.
//
// Do not call this directly, use drm_atomic_crtc_print_state()
// instead.
//
    pub state): *const drm_crtc_state,
//
// @get_vblank_counter:
//
// Driver callback for fetching a raw hardware vblank counter for the
// CRTC. It's meant to be used by new drivers as the replacement of
// &drm_driver.get_vblank_counter hook.
//
// This callback is optional. If a device doesn't have a hardware
// counter, the driver can simply leave the hook as NULL. The DRM core
// will account for missed vblank events while interrupts where disabled
// based on system timestamps.
//
// Wraparound handling and loss of events due to modesetting is dealt
// with in the DRM core code, as long as drivers call
// drm_crtc_vblank_off() and drm_crtc_vblank_on() when disabling or
// enabling a CRTC.
//
// See also &drm_device.vblank_disable_immediate and
// &drm_device.max_vblank_count.
//
// Returns:
//
// Raw vblank counter value.
//
    pub crtc): *mut *mut u32 (get_vblank_counter)(struct drm_crtc,
//
// @enable_vblank:
//
// Enable vblank interrupts for the CRTC. It's meant to be used by
// new drivers as the replacement of &drm_driver.enable_vblank hook.
//
// Returns:
//
// Zero on success, appropriate errno if the vblank interrupt cannot
// be enabled.
//
    pub crtc): *mut *mut int (enable_vblank)(struct drm_crtc,
//
// @disable_vblank:
//
// Disable vblank interrupts for the CRTC. It's meant to be used by
// new drivers as the replacement of &drm_driver.disable_vblank hook.
//
    pub crtc): *mut *mut void (disable_vblank)(struct drm_crtc,
//
// @get_vblank_timestamp:
//
// Called by drm_get_last_vbltimestamp(). Should return a precise
// timestamp when the most recent vblank interval ended or will end.
//
// Specifically, the timestamp in @vblank_time should correspond as
// closely as possible to the time when the first video scanline of
// the video frame after the end of vblank will start scanning out,
// the time immediately after end of the vblank interval. If the
// @crtc is currently inside vblank, this will be a time in the future.
// If the @crtc is currently scanning out a frame, this will be the
// past start time of the current scanout. This is meant to adhere
// to the OpenML OML_sync_control extension specification.
//
// Parameters:
//
// crtc:
// CRTC for which timestamp should be returned.
// max_error:
// Maximum allowable timestamp error in nanoseconds.
// Implementation should strive to provide timestamp
// with an error of at most max_error nanoseconds.
// Returns true upper bound on error for timestamp.
// vblank_time:
// Target location for returned vblank timestamp.
// in_vblank_irq:
// True when called from drm_crtc_handle_vblank().  Some drivers
// need to apply some workarounds for gpu-specific vblank irq quirks
// if flag is set.
//
// Returns:
//
// True on success, false on failure, which means the core should
// fallback to a simple timestamp taken in drm_crtc_handle_vblank().
//
    pub in_vblank_irq): bool,
}

//
// struct drm_crtc - central CRTC control structure
//
// Each CRTC may have one or more connectors associated with it.  This structure
// allows the CRTC to be controlled.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_crtc {
// @dev: parent DRM device
    pub dev: *mut drm_device,
// @port: OF node used by drm_of_find_possible_crtcs().
    pub port: *mut device_node,
//
// @head:
//
// List of all CRTCs on @dev, linked from &drm_mode_config.crtc_list.
// Invariant over the lifetime of @dev and therefore does not need
// locking.
//
    pub head: list_head,
// @name: human readable name, can be overwritten by the driver
    pub name: *mut c_char,
//
// @mutex:
//
// This provides a read lock for the overall CRTC state (mode, dpms
// state, ...) and a write lock for everything which can be update
// without a full modeset (fb, cursor data, CRTC properties ...). A full
// modeset also need to grab &drm_mode_config.connection_mutex.
//
// For atomic drivers specifically this protects @state.
//
    pub mutex: drm_modeset_lock,
// @base: base KMS object for ID tracking etc.
    pub base: drm_mode_object,
//
// @primary:
// Primary plane for this CRTC. Note that this is only
// relevant for legacy IOCTL, it specifies the plane implicitly used by
// the SETCRTC and PAGE_FLIP IOCTLs. It does not have any significance
// beyond that.
//
    pub primary: *mut drm_plane,
//
// @cursor:
// Cursor plane for this CRTC. Note that this is only relevant for
// legacy IOCTL, it specifies the plane implicitly used by the SETCURSOR
// and SETCURSOR2 IOCTLs. It does not have any significance
// beyond that.
//
    pub cursor: *mut drm_plane,
//
// @index: Position inside the mode_config.list, can be used as an array
// index. It is invariant over the lifetime of the CRTC.
//
    pub index: unsigned,
//
// @cursor_x: Current x position of the cursor, used for universal
// cursor planes because the SETCURSOR IOCTL only can update the
// framebuffer without supplying the coordinates. Drivers should not use
// this directly, atomic drivers should look at &drm_plane_state.crtc_x
// of the cursor plane instead.
//
    pub cursor_x: c_int,
//
// @cursor_y: Current y position of the cursor, used for universal
// cursor planes because the SETCURSOR IOCTL only can update the
// framebuffer without supplying the coordinates. Drivers should not use
// this directly, atomic drivers should look at &drm_plane_state.crtc_y
// of the cursor plane instead.
//
    pub cursor_y: c_int,
//
// @enabled:
//
// Is this CRTC enabled? Should only be used by legacy drivers, atomic
// drivers should instead consult &drm_crtc_state.enable and
// &drm_crtc_state.active. Atomic drivers can update this by calling
// drm_atomic_helper_update_legacy_modeset_state().
//
    pub enabled: bool,
//
// @mode:
//
// Current mode timings. Should only be used by legacy drivers, atomic
// drivers should instead consult &drm_crtc_state.mode. Atomic drivers
// can update this by calling
// drm_atomic_helper_update_legacy_modeset_state().
//
    pub mode: drm_display_mode,
//
// @hwmode:
//
// Programmed mode in hw, after adjustments for encoders, crtc, panel
// scaling etc. Should only be used by legacy drivers, for high
// precision vblank timestamps in
// drm_crtc_vblank_helper_get_vblank_timestamp().
//
// Note that atomic drivers should not use this, but instead use
// &drm_crtc_state.adjusted_mode. And for high-precision timestamps
// drm_crtc_vblank_helper_get_vblank_timestamp() used
// &drm_vblank_crtc.hwmode,
// which is filled out by calling drm_calc_timestamping_constants().
//
    pub hwmode: drm_display_mode,
//
// @x:
// x position on screen. Should only be used by legacy drivers, atomic
// drivers should look at &drm_plane_state.crtc_x of the primary plane
// instead. Updated by calling
// drm_atomic_helper_update_legacy_modeset_state().
//
    pub x: c_int,
//
// @y:
// y position on screen. Should only be used by legacy drivers, atomic
// drivers should look at &drm_plane_state.crtc_y of the primary plane
// instead. Updated by calling
// drm_atomic_helper_update_legacy_modeset_state().
//
    pub y: c_int,
// @funcs: CRTC control functions
    pub funcs: *const drm_crtc_funcs,
//
// @gamma_size: Size of legacy gamma ramp reported to userspace. Set up
// by calling drm_mode_crtc_set_gamma_size().
//
// Note that atomic drivers need to instead use
// &drm_crtc_state.gamma_lut. See drm_crtc_enable_color_mgmt().
//
    pub gamma_size: u32,
//
// @gamma_store: Gamma ramp values used by the legacy SETGAMMA and
// GETGAMMA IOCTls. Set up by calling drm_mode_crtc_set_gamma_size().
//
// Note that atomic drivers need to instead use
// &drm_crtc_state.gamma_lut. See drm_crtc_enable_color_mgmt().
//
    pub gamma_store: *mut u16,
// @helper_private: mid-layer private data
    pub helper_private: *const drm_crtc_helper_funcs,
// @properties: property tracking for this CRTC
    pub properties: drm_object_properties,
//
// @scaling_filter_property: property to apply a particular filter while
// scaling.
//
    pub scaling_filter_property: *mut drm_property,
//
// @sharpness_strength_property: property to apply
// the intensity of the sharpness requested.
//
    pub sharpness_strength_property: *mut drm_property,
//
// @state:
//
// Current atomic state for this CRTC.
//
// This is protected by @mutex. Note that nonblocking atomic commits
// access the current CRTC state without taking locks. Either by going
// through the &struct drm_atomic_commit pointers, see
// for_each_oldnew_crtc_in_state(), for_each_old_crtc_in_state() and
// for_each_new_crtc_in_state(). Or through careful ordering of atomic
// commit operations as implemented in the atomic helpers, see
// &struct drm_crtc_commit.
//
    pub state: *mut drm_crtc_state,
//
// @commit_list:
//
// List of &drm_crtc_commit structures tracking pending commits.
// Protected by @commit_lock. This list holds its own full reference,
// as does the ongoing commit.
//
// "Note that the commit for a state change is also tracked in
// &drm_crtc_state.commit. For accessing the immediately preceding
// commit in an atomic update it is recommended to just use that
// pointer in the old CRTC state, since accessing that doesn't need
// any locking or list-walking. @commit_list should only be used to
// stall for framebuffer cleanup that's signalled through
// &drm_crtc_commit.cleanup_done."
//
    pub commit_list: list_head,
//
// @commit_lock:
//
// Spinlock to protect @commit_list.
//
    pub commit_lock: spinlock_t,
//
// @debugfs_entry:
//
// Debugfs directory for this CRTC.
//
    pub debugfs_entry: *mut dentry,
//
// @crc:
//
// Configuration settings of CRC capture.
//
    pub crc: drm_crtc_crc,
//
// @fence_context:
//
// timeline context used for fence operations.
//
    pub fence_context: c_uint,
//
// @fence_lock:
//
// spinlock to protect the fences in the fence_context.
//
    pub fence_lock: spinlock_t,
//
// @fence_seqno:
//
// Seqno variable used as monotonic counter for the fences
// created on the CRTC's timeline.
//
    pub fence_seqno: c_ulong,
//
// @timeline_name:
//
// The name of the CRTC's fence timeline.
//
    pub timeline_name: [c_char; 32],
//
// @self_refresh_data: Holds the state for the self refresh helpers
//
// Initialized via drm_self_refresh_helper_init().
//
    pub self_refresh_data: *mut drm_self_refresh_data,
}

//
// struct drm_mode_set - new values for a CRTC config change
// @fb: framebuffer to use for new config
// @crtc: CRTC whose configuration we're about to change
// @mode: mode timings to use
// @x: position of this CRTC relative to @fb
// @y: position of this CRTC relative to @fb
// @connectors: array of connectors to drive with this CRTC if possible
// @num_connectors: size of @connectors array
//
// This represents a modeset configuration for the legacy SETCRTC ioctl and is
// also used internally. Atomic drivers instead use &drm_atomic_commit.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_set {
    pub fb: *mut drm_framebuffer,
    pub crtc: *mut drm_crtc,
    pub mode: *mut drm_display_mode,
    pub x: u32,
    pub y: u32,
    pub connectors: *mut drm_connector,
    pub num_connectors: usize,
}

extern "C" {
    pub fn drm_crtc_cleanup(crtc: *mut drm_crtc);
}
//
// drmm_crtc_alloc_with_planes - Allocate and initialize a new CRTC object with
// specified primary and cursor planes.
// @dev: DRM device
// @type: the type of the struct which contains struct &drm_crtc
// @member: the name of the &drm_crtc within @type.
// @primary: Primary plane for CRTC
// @cursor: Cursor plane for CRTC
// @funcs: callbacks for the new CRTC
// @name: printf style format string for the CRTC name, or NULL for default name
//
// Allocates and initializes a new crtc object. Cleanup is automatically
// handled through registering drmm_crtc_cleanup() with drmm_add_action().
//
// The @drm_crtc_funcs.destroy hook must be NULL.
//
// Returns:
// Pointer to new crtc, or ERR_PTR on failure.
//

//
// drm_crtc_index - find the index of a registered CRTC
// @crtc: CRTC to find index for
//
// Given a registered CRTC, return the index of that CRTC within a DRM
// device's list of CRTCs.
//
// drm_crtc_mask - find the mask of a registered CRTC
// @crtc: CRTC to find mask for
//
// Given a registered CRTC, return the mask bit of that CRTC for the
// &drm_encoder.possible_crtcs and &drm_plane.possible_crtcs fields.
//
extern "C" {
    pub fn drm_mode_set_config_internal(set: *mut drm_mode_set) -> c_int;
}
//
// drm_crtc_find - look up a CRTC object from its ID
// @dev: DRM device
// @file_priv: drm file to check for lease against.
// @id: &drm_mode_object ID
//
// This can be used to look up a CRTC from its userspace ID. Only used by
// drivers for legacy IOCTLs and interface, nowadays extensions to the KMS
// userspace interface should be done using &drm_property.
//
// drm_for_each_crtc - iterate over all CRTCs
// @crtc: a &struct drm_crtc as the loop cursor
// @dev: the &struct drm_device
//
// Iterate over all CRTCs of @dev.
//

//
// drm_for_each_crtc_reverse - iterate over all CRTCs in reverse order
// @crtc: a &struct drm_crtc as the loop cursor
// @dev: the &struct drm_device
//
// Iterate over all CRTCs of @dev.
//

extern "C" {
    pub fn drm_crtc_in_clone_mode(crtc_state: *mut drm_crtc_state) -> bool;
}
extern "C" {
    pub fn drm_crtc_create_sharpness_strength_property(crtc: *mut drm_crtc) -> c_int;
}
