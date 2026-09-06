//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/drm_bridge.h
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

//
// enum drm_bridge_attach_flags - Flags for &drm_bridge_funcs.attach
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_bridge_attach_flags {
//
// @DRM_BRIDGE_ATTACH_NO_CONNECTOR: When this flag is set the bridge
// shall not create a drm_connector.
//
    DRM_BRIDGE_ATTACH_NO_CONNECTOR = BIT(0),
}

//
// struct drm_bridge_funcs - drm_bridge control functions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_bridge_funcs {
//
// @attach:
//
// This callback is invoked whenever our bridge is being attached to a
// &drm_encoder. The flags argument tunes the behaviour of the attach
// operation (see DRM_BRIDGE_ATTACH_*).
//
// The @attach callback is optional.
//
// RETURNS:
//
// Zero on success, error code on failure.
//
    pub flags): drm_bridge_attach_flags,
//
// @destroy:
//
// This callback is invoked when the bridge is about to be
// deallocated.
//
// The @destroy callback is optional.
//
    pub bridge): *mut *mut void (destroy)(struct drm_bridge,
//
// @detach:
//
// This callback is invoked whenever our bridge is being detached from a
// &drm_encoder.
//
// The @detach callback is optional.
//
    pub bridge): *mut *mut void (detach)(struct drm_bridge,
//
// @mode_valid:
//
// This callback is used to check if a specific mode is valid in this
// bridge. This should be implemented if the bridge has some sort of
// restriction in the modes it can display. For example, a given bridge
// may be responsible to set a clock value. If the clock can not
// produce all the values for the available modes then this callback
// can be used to restrict the number of modes to only the ones that
// can be displayed.
//
// This hook is used by the probe helpers to filter the mode list in
// drm_helper_probe_single_connector_modes(), and it is used by the
// atomic helpers to validate modes supplied by userspace in
// drm_atomic_helper_check_modeset().
//
// The @mode_valid callback is optional.
//
// NOTE:
//
// Since this function is both called from the check phase of an atomic
// commit, and the mode validation in the probe paths it is not allowed
// to look at anything else but the passed-in mode, and validate it
// against configuration-invariant hardware constraints. Any further
// limits which depend upon the configuration can only be checked in
// @mode_fixup.
//
// RETURNS:
//
// drm_mode_status Enum
//
    pub mode): *const drm_display_mode,
//
// @mode_fixup:
//
// This callback is used to validate and adjust a mode. The parameter
// mode is the display mode that should be fed to the next element in
// the display chain, either the final &drm_connector or the next
// &drm_bridge. The parameter adjusted_mode is the input mode the bridge
// requires. It can be modified by this callback and does not need to
// match mode. See also &drm_crtc_state.adjusted_mode for more details.
//
// This is the only hook that allows a bridge to reject a modeset. If
// this function passes all other callbacks must succeed for this
// configuration.
//
// The mode_fixup callback is optional. &drm_bridge_funcs.mode_fixup()
// is not called when &drm_bridge_funcs.atomic_check() is implemented,
// so only one of them should be provided.
//
// NOTE:
//
// This function is called in the check phase of atomic modesets, which
// can be aborted for any reason (including on userspace's request to
// just check whether a configuration would be possible). Drivers MUST
// NOT touch any persistent state (hardware or software) or data
// structures except the passed in @state parameter.
//
// Also beware that userspace can request its own custom modes, neither
// core nor helpers filter modes to the list of probe modes reported by
// the GETCONNECTOR IOCTL and stored in &drm_connector.modes. To ensure
// that modes are filtered consistently put any bridge constraints and
// limits checks into @mode_valid.
//
// RETURNS:
//
// True if an acceptable configuration is possible, false if the modeset
// operation should be rejected.
//
    pub adjusted_mode): *mut drm_display_mode,
//
// @mode_set:
//
// This callback should set the given mode on the bridge. It is called
// after the @mode_set callback for the preceding element in the display
// pipeline has been called already. If the bridge is the first element
// then this would be &drm_encoder_helper_funcs.mode_set. The display
// pipe (i.e.  clocks and timing signals) is off when this function is
// called.
//
// The adjusted_mode parameter is the mode output by the CRTC for the
// first bridge in the chain. It can be different from the mode
// parameter that contains the desired mode for the connector at the end
// of the bridges chain, for instance when the first bridge in the chain
// performs scaling. The adjusted mode is mostly useful for the first
// bridge in the chain and is likely irrelevant for the other bridges.
//
// For atomic drivers the adjusted_mode is the mode stored in
// &drm_crtc_state.adjusted_mode.
//
// NOTE:
//
// This is deprecated, do not use!
// New drivers shall set their mode in the
// &drm_bridge_funcs.atomic_enable operation.
//
    pub adjusted_mode): *const drm_display_mode,
//
// @atomic_pre_enable:
//
// This callback should enable the bridge. It is called right before
// the preceding element in the display pipe is enabled. If the
// preceding element is a bridge this means it's called before that
// bridge's @atomic_pre_enable or @pre_enable function. If the preceding
// element is a &drm_encoder it's called right before the encoder's
// &drm_encoder_helper_funcs.atomic_enable hook.
//
// The display pipe (i.e. clocks and timing signals) feeding this bridge
// will not yet be running when this callback is called. The bridge must
// not enable the display link feeding the next bridge in the chain (if
// there is one) when this callback is called.
//
// The @atomic_pre_enable callback is optional.
//
    pub state): *mut drm_atomic_commit,
//
// @atomic_enable:
//
// This callback should enable the bridge. It is called right after
// the preceding element in the display pipe is enabled. If the
// preceding element is a bridge this means it's called after that
// bridge's @atomic_enable or @enable function. If the preceding element
// is a &drm_encoder it's called right after the encoder's
// &drm_encoder_helper_funcs.atomic_enable hook.
//
// The bridge can assume that the display pipe (i.e. clocks and timing
// signals) feeding it is running when this callback is called. This
// callback must enable the display link feeding the next bridge in the
// chain if there is one.
//
// The @atomic_enable callback is optional.
//
    pub state): *mut drm_atomic_commit,
//
// @atomic_disable:
//
// This callback should disable the bridge. It is called right before
// the preceding element in the display pipe is disabled. If the
// preceding element is a bridge this means it's called before that
// bridge's @atomic_disable or @disable vfunc. If the preceding element
// is a &drm_encoder it's called right before the
// &drm_encoder_helper_funcs.atomic_disable hook.
//
// The bridge can assume that the display pipe (i.e. clocks and timing
// signals) feeding it is still running when this callback is called.
//
// The @atomic_disable callback is optional.
//
    pub state): *mut drm_atomic_commit,
//
// @atomic_post_disable:
//
// This callback should disable the bridge. It is called right after the
// preceding element in the display pipe is disabled. If the preceding
// element is a bridge this means it's called after that bridge's
// @atomic_post_disable or @post_disable function. If the preceding
// element is a &drm_encoder it's called right after the encoder's
// &drm_encoder_helper_funcs.atomic_disable hook.
//
// The bridge must assume that the display pipe (i.e. clocks and timing
// signals) feeding it is no longer running when this callback is
// called.
//
// The @atomic_post_disable callback is optional.
//
    pub state): *mut drm_atomic_commit,
//
// @atomic_duplicate_state:
//
// Duplicate the current bridge state object (which is guaranteed to be
// non-NULL).
//
// The atomic_duplicate_state hook is mandatory if the bridge
// implements any of the atomic hooks, and should be left unassigned
// otherwise. For bridges that don't subclass &drm_bridge_state, the
// drm_atomic_helper_bridge_duplicate_state() helper function shall be
// used to implement this hook.
//
// RETURNS:
// A valid drm_bridge_state object or NULL if the allocation fails.
//
    pub bridge): *mut *mut *mut drm_bridge_state (atomic_duplicate_state)(drm_bridge,
//
// @atomic_destroy_state:
//
// Destroy a bridge state object previously allocated by
// &drm_bridge_funcs.atomic_duplicate_state().
//
// The atomic_destroy_state hook is mandatory if the bridge implements
// any of the atomic hooks, and should be left unassigned otherwise.
// For bridges that don't subclass &drm_bridge_state, the
// drm_atomic_helper_bridge_destroy_state() helper function shall be
// used to implement this hook.
//
    pub state): *mut drm_bridge_state,
//
// @atomic_get_output_bus_fmts:
//
// Return the supported bus formats on the output end of a bridge.
// The returned array must be allocated with kmalloc() and will be
// freed by the caller. If the allocation fails, NULL should be
// returned. num_output_fmts must be set to the returned array size.
// Formats listed in the returned array should be listed in decreasing
// preference order (the core will try all formats until it finds one
// that works).
//
// This method is only called on the last element of the bridge chain
// as part of the bus format negotiation process that happens in
// &drm_atomic_bridge_chain_select_bus_fmts().
// This method is optional. When not implemented, the core will
// fall back to &drm_connector.display_info.bus_formats[0] if
// &drm_connector.display_info.num_bus_formats > 0,
// or to MEDIA_BUS_FMT_FIXED otherwise.
//
    pub num_output_fmts): *mut c_uint,
//
// @atomic_get_input_bus_fmts:
//
// Return the supported bus formats on the input end of a bridge for
// a specific output bus format.
//
// The returned array must be allocated with kmalloc() and will be
// freed by the caller. If the allocation fails, NULL should be
// returned. num_input_fmts must be set to the returned array size.
// Formats listed in the returned array should be listed in decreasing
// preference order (the core will try all formats until it finds one
// that works). When the format is not supported NULL should be
// returned and num_input_fmts should be set to 0.
//
// This method is called on all elements of the bridge chain as part of
// the bus format negotiation process that happens in
// drm_atomic_bridge_chain_select_bus_fmts().
// This method is optional. When not implemented, the core will bypass
// bus format negotiation on this element of the bridge without
// failing, and the previous element in the chain will be passed
// MEDIA_BUS_FMT_FIXED as its output bus format.
//
// Bridge drivers that need to support being linked to bridges that are
// not supporting bus format negotiation should handle the
// output_fmt == MEDIA_BUS_FMT_FIXED case appropriately, by selecting a
// sensible default value or extracting this information from somewhere
// else (FW property, &drm_display_mode, &drm_display_info, ...)
//
// Note: Even if input format selection on the first bridge has no
// impact on the negotiation process (bus format negotiation stops once
// we reach the first element of the chain), drivers are expected to
// return accurate input formats as the input format may be used to
// configure the CRTC output appropriately.
//
    pub num_input_fmts): *mut c_uint,
//
// @atomic_check:
//
// This method is responsible for checking bridge state correctness.
// It can also check the state of the surrounding components in chain
// to make sure the whole pipeline can work properly.
//
// &drm_bridge_funcs.atomic_check() hooks are called in reverse
// order (from the last to the first bridge).
//
// This method is optional. &drm_bridge_funcs.mode_fixup() is not
// called when &drm_bridge_funcs.atomic_check() is implemented, so only
// one of them should be provided.
//
// If drivers need to tweak &drm_bridge_state.input_bus_cfg.flags or
// &drm_bridge_state.output_bus_cfg.flags it should happen in
// this function. By default the &drm_bridge_state.output_bus_cfg.flags
// field is set to the next bridge
// &drm_bridge_state.input_bus_cfg.flags value or
// &drm_connector.display_info.bus_flags if the bridge is the last
// element in the chain.
//
// RETURNS:
// zero if the check passed, a negative error code otherwise.
//
    pub conn_state): *mut drm_connector_state,
//
// @atomic_create_state:
//
// Allocate a pristine, initialized, state for the bridge
// object and return it. This callback must have no side
// effects: in particular, the returned state must not be
// assigned to the object's state pointer and it must not affect
// the hardware state.
//
// RETURNS:
//
// A new, pristine, bridge state instance or an error pointer
// on failure.
//
    pub bridge): *mut *mut *mut drm_bridge_state (atomic_create_state)(drm_bridge,
//
// @detect:
//
// Check if anything is attached to the bridge output.
//
// This callback is optional, if not implemented the bridge will be
// considered as always having a component attached to its output.
// Bridges that implement this callback shall set the
// DRM_BRIDGE_OP_DETECT flag in their &drm_bridge->ops.
//
// RETURNS:
//
// drm_connector_status indicating the bridge output status.
//
    pub connector): *mut drm_connector,
//
// @get_modes:
//
// Fill all modes currently valid for the sink into the &drm_connector
// with drm_mode_probed_add().
//
// The @get_modes callback is mostly intended to support non-probeable
// displays such as many fixed panels. Bridges that support reading
// EDID shall leave @get_modes unimplemented and implement the
// &drm_bridge_funcs->edid_read callback instead.
//
// This callback is optional. Bridges that implement it shall set the
// DRM_BRIDGE_OP_MODES flag in their &drm_bridge->ops.
//
// The connector parameter shall be used for the sole purpose of
// filling modes, and shall not be stored internally by bridge drivers
// for future usage.
//
// RETURNS:
//
// The number of modes added by calling drm_mode_probed_add().
//
    pub connector): *mut drm_connector,
//
// @edid_read:
//
// Read the EDID data of the connected display.
//
// The @edid_read callback is the preferred way of reporting mode
// information for a display connected to the bridge output. Bridges
// that support reading EDID shall implement this callback and leave
// the @get_modes callback unimplemented.
//
// The caller of this operation shall first verify the output
// connection status and refrain from reading EDID from a disconnected
// output.
//
// This callback is optional. Bridges that implement it shall set the
// DRM_BRIDGE_OP_EDID flag in their &drm_bridge->ops.
//
// The connector parameter shall be used for the sole purpose of EDID
// retrieval, and shall not be stored internally by bridge drivers for
// future usage.
//
// RETURNS:
//
// An edid structure newly allocated with drm_edid_alloc() or returned
// from drm_edid_read() family of functions on success, or NULL
// otherwise. The caller is responsible for freeing the returned edid
// structure with drm_edid_free().
//
    pub connector): *mut drm_connector,
//
// @hpd_notify:
//
// Notify the bridge of hot plug detection.
//
// This callback is optional, it may be implemented by bridges that
// need to be notified of display connection or disconnection for
// internal reasons. One use case is to reset the internal state of CEC
// controllers for HDMI bridges.
//
    pub status): drm_connector_status,
//
// @hpd_enable:
//
// Enable hot plug detection. From now on the bridge shall call
// drm_bridge_hpd_notify() each time a change is detected in the output
// connection status, until hot plug detection gets disabled with
// @hpd_disable.
//
// This callback is optional and shall only be implemented by bridges
// that support hot-plug notification without polling. Bridges that
// implement it shall also implement the @hpd_disable callback and set
// the DRM_BRIDGE_OP_HPD flag in their &drm_bridge->ops.
//
    pub bridge): *mut *mut void (hpd_enable)(struct drm_bridge,
//
// @hpd_disable:
//
// Disable hot plug detection. Once this function returns the bridge
// shall not call drm_bridge_hpd_notify() when a change in the output
// connection status occurs.
//
// This callback is optional and shall only be implemented by bridges
// that support hot-plug notification without polling. Bridges that
// implement it shall also implement the @hpd_enable callback and set
// the DRM_BRIDGE_OP_HPD flag in their &drm_bridge->ops.
//
    pub bridge): *mut *mut void (hpd_disable)(struct drm_bridge,
//
// @hdmi_tmds_char_rate_valid:
//
// Check whether a particular TMDS character rate is supported by the
// driver.
//
// This callback is optional and should only be implemented by the
// bridges that take part in the HDMI connector implementation. Bridges
// that implement it shall set the DRM_BRIDGE_OP_HDMI flag in their
// &drm_bridge->ops.
//
// Returns:
//
// Either &drm_mode_status.MODE_OK or one of the failure reasons
// in &enum drm_mode_status.
//
    pub tmds_rate): c_ulonglong,
//
// @hdmi_clear_avi_infoframe:
//
// This callback clears the infoframes in the hardware during commit.
//
// This callback is optional but it must be implemented by bridges that
// set the DRM_BRIDGE_OP_HDMI flag in their &drm_bridge->ops.
//
    pub bridge): *mut *mut int (hdmi_clear_avi_infoframe)(struct drm_bridge,
//
// @hdmi_write_avi_infoframe:
//
// Program the infoframe into the hardware.
//
// This callback is optional but it must be implemented by bridges that
// set the DRM_BRIDGE_OP_HDMI flag in their &drm_bridge->ops.
//
    pub len): *const *const u8 buffer, size_t,
//
// @hdmi_clear_hdmi_infoframe:
//
// This callback clears the infoframes in the hardware during commit.
//
// This callback is optional but it must be implemented by bridges that
// set the DRM_BRIDGE_OP_HDMI flag in their &drm_bridge->ops.
//
    pub bridge): *mut *mut int (hdmi_clear_hdmi_infoframe)(struct drm_bridge,
//
// @hdmi_write_hdmi_infoframe:
//
// Program the infoframe into the hardware.
//
// This callback is optional but it must be implemented by bridges that
// set the DRM_BRIDGE_OP_HDMI flag in their &drm_bridge->ops.
//
    pub len): *const *const u8 buffer, size_t,
//
// @hdmi_clear_hdr_drm_infoframe:
//
// This callback clears the infoframes in the hardware during commit.
//
// This callback is optional but it must be implemented by bridges that
// set the DRM_BRIDGE_OP_HDMI_HDR_DRM_INFOFRAME flag in their
// &drm_bridge->ops.
//
    pub bridge): *mut *mut int (hdmi_clear_hdr_drm_infoframe)(struct drm_bridge,
//
// @hdmi_write_hdr_drm_infoframe:
//
// Program the infoframe into the hardware.
//
// This callback is optional but it must be implemented by bridges that
// set the DRM_BRIDGE_OP_HDMI_HDR_DRM_INFOFRAME flag in their
// &drm_bridge->ops.
//
    pub len): *const *const u8 buffer, size_t,
//
// @hdmi_clear_spd_infoframe:
//
// This callback clears the infoframes in the hardware during commit.
//
// This callback is optional but it must be implemented by bridges that
// set the DRM_BRIDGE_OP_HDMI_SPD_INFOFRAME flag in their
// &drm_bridge->ops.
//
    pub bridge): *mut *mut int (hdmi_clear_spd_infoframe)(struct drm_bridge,
//
// @hdmi_write_spd_infoframe:
//
// Program the infoframe into the hardware.
//
// This callback is optional but it must be implemented by bridges that
// set the DRM_BRIDGE_OP_HDMI_SPD_INFOFRAME flag in their
// &drm_bridge->ops.
//
    pub len): *const *const u8 buffer, size_t,
//
// @hdmi_clear_audio_infoframe:
//
// This callback clears the infoframes in the hardware during commit.
//
// This callback is optional but it must be implemented by bridges that
// set the DRM_BRIDGE_OP_HDMI_AUDIO flag in their &drm_bridge->ops.
//
    pub bridge): *mut *mut int (hdmi_clear_audio_infoframe)(struct drm_bridge,
//
// @hdmi_write_audio_infoframe:
//
// Program the infoframe into the hardware.
//
// This callback is optional but it must be implemented by bridges that
// set the DRM_BRIDGE_OP_HDMI_AUDIO flag in their &drm_bridge->ops.
//
    pub len): *const *const u8 buffer, size_t,
//
// @hdmi_audio_startup:
//
// Called when ASoC starts an audio stream setup.
//
// This callback is optional, it can be implemented by bridges that
// set the @DRM_BRIDGE_OP_HDMI_AUDIO flag in their &drm_bridge->ops.
//
// Returns:
// 0 on success, a negative error code otherwise
//
    pub connector): *mut drm_connector,
//
// @hdmi_audio_prepare:
// Configures HDMI-encoder for audio stream. Can be called multiple
// times for each setup.
//
// This callback is optional but it must be implemented by bridges that
// set the @DRM_BRIDGE_OP_HDMI_AUDIO flag in their &drm_bridge->ops.
//
// Returns:
// 0 on success, a negative error code otherwise
//
    pub hparms): *mut hdmi_codec_params,
//
// @hdmi_audio_shutdown:
//
// Shut down the audio stream.
//
// This callback is optional but it must be implemented by bridges that
// set the @DRM_BRIDGE_OP_HDMI_AUDIO flag in their &drm_bridge->ops.
//
// Returns:
// 0 on success, a negative error code otherwise
//
    pub connector): *mut drm_connector,
//
// @hdmi_audio_mute_stream:
//
// Mute/unmute HDMI audio stream.
//
// This callback is optional, it can be implemented by bridges that
// set the @DRM_BRIDGE_OP_HDMI_AUDIO flag in their &drm_bridge->ops.
//
// Returns:
// 0 on success, a negative error code otherwise
//
    pub direction): bool enable, int,
//
// @hdmi_cec_init:
//
// Initialize CEC part of the bridge.
//
// This callback is optional, it can be implemented by bridges that
// set the @DRM_BRIDGE_OP_HDMI_CEC_ADAPTER flag in their
// &drm_bridge->ops.
//
// Returns:
// 0 on success, a negative error code otherwise
//
    pub connector): *mut drm_connector,
//
// @hdmi_cec_enable:
//
// Enable or disable the CEC adapter inside the bridge.
//
// This callback is optional, it can be implemented by bridges that
// set the @DRM_BRIDGE_OP_HDMI_CEC_ADAPTER flag in their
// &drm_bridge->ops.
//
// Returns:
// 0 on success, a negative error code otherwise
//
    pub enable): *mut *mut *mut int (hdmi_cec_enable)(struct drm_bridge bridge, bool,
//
// @hdmi_cec_log_addr:
//
// Set the logical address of the CEC adapter inside the bridge.
//
// This callback is optional, it can be implemented by bridges that
// set the @DRM_BRIDGE_OP_HDMI_CEC_ADAPTER flag in their
// &drm_bridge->ops.
//
// Returns:
// 0 on success, a negative error code otherwise
//
    pub logical_addr): *mut *mut *mut int (hdmi_cec_log_addr)(struct drm_bridge bridge, u8,
//
// @hdmi_cec_transmit:
//
// Transmit the message using the CEC adapter inside the bridge.
//
// This callback is optional, it can be implemented by bridges that
// set the @DRM_BRIDGE_OP_HDMI_CEC_ADAPTER flag in their
// &drm_bridge->ops.
//
// Returns:
// 0 on success, a negative error code otherwise
//
    pub msg): *mut u32 signal_free_time, struct cec_msg,
//
// @dp_audio_startup:
//
// Called when ASoC starts a DisplayPort audio stream setup.
//
// This callback is optional, it can be implemented by bridges that
// set the @DRM_BRIDGE_OP_DP_AUDIO flag in their &drm_bridge->ops.
//
// Returns:
// 0 on success, a negative error code otherwise
//
    pub connector): *mut drm_connector,
//
// @dp_audio_prepare:
// Configures DisplayPort audio stream. Can be called multiple
// times for each setup.
//
// This callback is optional but it must be implemented by bridges that
// set the @DRM_BRIDGE_OP_DP_AUDIO flag in their &drm_bridge->ops.
//
// Returns:
// 0 on success, a negative error code otherwise
//
    pub hparms): *mut hdmi_codec_params,
//
// @dp_audio_shutdown:
//
// Shut down the DisplayPort audio stream.
//
// This callback is optional but it must be implemented by bridges that
// set the @DRM_BRIDGE_OP_DP_AUDIO flag in their &drm_bridge->ops.
//
// Returns:
// 0 on success, a negative error code otherwise
//
    pub connector): *mut drm_connector,
//
// @dp_audio_mute_stream:
//
// Mute/unmute DisplayPort audio stream.
//
// This callback is optional, it can be implemented by bridges that
// set the @DRM_BRIDGE_OP_DP_AUDIO flag in their &drm_bridge->ops.
//
// Returns:
// 0 on success, a negative error code otherwise
//
    pub direction): bool enable, int,
//
// @debugfs_init:
//
// Allows bridges to create bridge-specific debugfs files.
//
    pub root): *mut *mut *mut void (debugfs_init)(struct drm_bridge bridge, struct dentry,
}

//
// struct drm_bridge_timings - timing information for the bridge
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_bridge_timings {
//
// @input_bus_flags:
//
// Tells what additional settings for the pixel data on the bus
// this bridge requires (like pixel signal polarity). See also
// &drm_display_info->bus_flags.
//
    pub input_bus_flags: u32,
//
// @setup_time_ps:
//
// Defines the time in picoseconds the input data lines must be
// stable before the clock edge.
//
    pub setup_time_ps: u32,
//
// @hold_time_ps:
//
// Defines the time in picoseconds taken for the bridge to sample the
// input signal after the clock edge.
//
    pub hold_time_ps: u32,
//
// @dual_link:
//
// True if the bus operates in dual-link mode. The exact meaning is
// dependent on the bus type. For LVDS buses, this indicates that even-
// and odd-numbered pixels are received on separate links.
//
    pub dual_link: bool,
}

//
// enum drm_bridge_ops - Bitmask of operations supported by the bridge
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_bridge_ops {
//
// @DRM_BRIDGE_OP_DETECT: The bridge can detect displays connected to
// its output. Bridges that set this flag shall implement the
// &drm_bridge_funcs->detect callback.
//
    DRM_BRIDGE_OP_DETECT = BIT(0),
//
// @DRM_BRIDGE_OP_EDID: The bridge can retrieve the EDID of the display
// connected to its output. Bridges that set this flag shall implement
// the &drm_bridge_funcs->edid_read callback.
//
    DRM_BRIDGE_OP_EDID = BIT(1),
//
// @DRM_BRIDGE_OP_HPD: The bridge can detect hot-plug and hot-unplug
// without requiring polling. Bridges that set this flag shall
// implement the &drm_bridge_funcs->hpd_enable and
// &drm_bridge_funcs->hpd_disable callbacks if they support enabling
// and disabling hot-plug detection dynamically.
//
    DRM_BRIDGE_OP_HPD = BIT(2),
//
// @DRM_BRIDGE_OP_MODES: The bridge can retrieve the modes supported
// by the display at its output. This does not include reading EDID
// which is separately covered by @DRM_BRIDGE_OP_EDID. Bridges that set
// this flag shall implement the &drm_bridge_funcs->get_modes callback.
//
    DRM_BRIDGE_OP_MODES = BIT(3),
//
// @DRM_BRIDGE_OP_HDMI: The bridge provides HDMI connector operations,
// including infoframes support. Bridges that set this flag must
// provide HDMI-related information and implement the
// &drm_bridge_funcs->clear_avi_infoframe,
// &drm_bridge_funcs->write_avi_infoframe,
// &drm_bridge_funcs->clear_hdmi_infoframe and
// &drm_bridge_funcs->write_hdmi_infoframe callbacks.
//
// Note: currently there can be at most one bridge in a chain that sets
// this bit. This is to simplify corresponding glue code in connector
// drivers.
//
    DRM_BRIDGE_OP_HDMI = BIT(4),
//
// @DRM_BRIDGE_OP_HDMI_AUDIO: The bridge provides HDMI audio operations.
// Bridges that set this flag must implement the
// &drm_bridge_funcs->hdmi_audio_prepare and
// &drm_bridge_funcs->hdmi_audio_shutdown callbacks.
// If the bridge implements @DRM_BRIDGE_OP_HDMI, it also must implement
// &drm_bridge_funcs->hdmi_write_audio_infoframe and
// &drm_bridge_funcs->hdmi_cleaer_audio_infoframe callbacks.
//
// Note: currently there can be at most one bridge in a chain that sets
// this bit. This is to simplify corresponding glue code in connector
// drivers. Also it is not possible to have a bridge in the chain that
// sets @DRM_BRIDGE_OP_DP_AUDIO if there is a bridge that sets this
// flag.
//
    DRM_BRIDGE_OP_HDMI_AUDIO = BIT(5),
//
// @DRM_BRIDGE_OP_DP_AUDIO: The bridge provides DisplayPort audio operations.
// Bridges that set this flag must implement the
// &drm_bridge_funcs->dp_audio_prepare and
// &drm_bridge_funcs->dp_audio_shutdown callbacks.
//
// Note: currently there can be at most one bridge in a chain that sets
// this bit. This is to simplify corresponding glue code in connector
// drivers. Also it is not possible to have a bridge in the chain that
// sets @DRM_BRIDGE_OP_HDMI_AUDIO if there is a bridge that sets this
// flag.
//
    DRM_BRIDGE_OP_DP_AUDIO = BIT(6),
//
// @DRM_BRIDGE_OP_HDMI_CEC_NOTIFIER: The bridge requires CEC notifier
// to be present.
//
    DRM_BRIDGE_OP_HDMI_CEC_NOTIFIER = BIT(7),
//
// @DRM_BRIDGE_OP_HDMI_CEC_ADAPTER: The bridge requires CEC adapter
// to be present.
//
    DRM_BRIDGE_OP_HDMI_CEC_ADAPTER = BIT(8),
//
// @DRM_BRIDGE_OP_HDMI_HDR_DRM_INFOFRAME: The bridge supports
// &drm_bridge_funcs->hdmi_write_hdr_drm_infoframe and
// &drm_bridge_funcs->hdmi_clear_hdr_drm_infoframe callbacks.
//
    DRM_BRIDGE_OP_HDMI_HDR_DRM_INFOFRAME = BIT(9),
//
// @DRM_BRIDGE_OP_HDMI_SPD_INFOFRAME: The bridge supports
// &drm_bridge_funcs->hdmi_write_spd_infoframe and
// &drm_bridge_funcs->hdmi_clear_spd_infoframe callbacks.
//
    DRM_BRIDGE_OP_HDMI_SPD_INFOFRAME = BIT(10),
}

//
// struct drm_bridge - central DRM bridge control structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_bridge {
// @base: inherit from &drm_private_object
    pub base: drm_private_obj,
// @dev: DRM device this bridge belongs to
    pub dev: *mut drm_device,
// @encoder: encoder to which this bridge is connected
    pub encoder: *mut drm_encoder,
// @chain_node: used to form a bridge chain
    pub chain_node: list_head,
// @of_node: device node pointer to the bridge
    pub of_node: *mut device_node,
// @list: to keep track of all added bridges
    pub list: list_head,
//
// @timings:
//
// the timing specification for the bridge, if any (may be NULL)
//
    pub timings: *const drm_bridge_timings,
// @funcs: control functions
    pub funcs: *const drm_bridge_funcs,
//
// @container: Pointer to the private driver struct embedding this
// @struct drm_bridge.
//
    pub container: *mut c_void,
//
// @refcount: reference count of users referencing this bridge.
//
    pub refcount: kref,
//
// @unplugged:
//
// Flag to tell if the bridge has been unplugged.
// See drm_bridge_enter() and drm_bridge_unplug().
//
    pub unplugged: bool,
// @driver_private: pointer to the bridge driver's internal context
    pub driver_private: *mut c_void,
// @ops: bitmask of operations supported by the bridge
    pub ops: drm_bridge_ops,
//
// @type: Type of the connection at the bridge output
// (DRM_MODE_CONNECTOR_*). For bridges at the end of this chain this
// identifies the type of connected display.
//
    pub type: c_int,
//
// @interlace_allowed: Indicate that the bridge can handle interlaced
// modes.
//
    pub interlace_allowed: bool,
//
// @ycbcr_420_allowed: Indicate that the bridge can handle YCbCr 420
// output.
//
    pub ycbcr_420_allowed: bool,
//
// @pre_enable_prev_first: The bridge requires that the prev
// bridge @pre_enable function is called before its @pre_enable,
// and conversely for post_disable. This is most frequently a
// requirement for DSI devices which need the host to be initialised
// before the peripheral.
//
    pub pre_enable_prev_first: bool,
//
// @support_hdcp: Indicate that the bridge supports HDCP.
//
    pub support_hdcp: bool,
//
// @ddc: Associated I2C adapter for DDC access, if any.
//
    pub ddc: *mut i2c_adapter,
//
// @vendor: Vendor of the product to be used for the SPD InfoFrame
// generation. This is required if @DRM_BRIDGE_OP_HDMI is set.
//
    pub vendor: *const c_char,
//
// @product: Name of the product to be used for the SPD InfoFrame
// generation. This is required if @DRM_BRIDGE_OP_HDMI is set.
//
    pub product: *const c_char,
//
// @supported_formats: Bitmask of @drm_output_color_format listing
// supported output formats. This is only relevant if
// @DRM_BRIDGE_OP_HDMI is set.
//
    pub supported_formats: c_uint,
//
// @max_bpc: Maximum bits per char the HDMI bridge supports. Allowed
// values are 8, 10 and 12. This is only relevant if
// @DRM_BRIDGE_OP_HDMI is set.
//
    pub max_bpc: c_uint,
//
// @hdmi_cec_dev: device to be used as a containing device for CEC
// functions.
//
    pub hdmi_cec_dev: *mut device,
//
// @hdmi_audio_dev: device to be used as a parent for the HDMI Codec if
// either of @DRM_BRIDGE_OP_HDMI_AUDIO or @DRM_BRIDGE_OP_DP_AUDIO is set.
//
    pub hdmi_audio_dev: *mut device,
//
// @hdmi_audio_max_i2s_playback_channels: maximum number of playback
// I2S channels for the @DRM_BRIDGE_OP_HDMI_AUDIO or
// @DRM_BRIDGE_OP_DP_AUDIO.
//
    pub hdmi_audio_max_i2s_playback_channels: c_int,
//
// @hdmi_audio_i2s_formats: supported I2S formats, optional. The
// default is to allow all formats supported by the corresponding I2S
// bus driver. This is only used for bridges setting
// @DRM_BRIDGE_OP_HDMI_AUDIO or @DRM_BRIDGE_OP_DP_AUDIO.
//
    pub hdmi_audio_i2s_formats: u64,
//
// @hdmi_audio_spdif_playback: set if this bridge has S/PDIF playback
// port for @DRM_BRIDGE_OP_HDMI_AUDIO or @DRM_BRIDGE_OP_DP_AUDIO.
//
    pub 1: unsigned int hdmi_audio_spdif_playback :,
//
// @hdmi_audio_dai_port: sound DAI port for either of
// @DRM_BRIDGE_OP_HDMI_AUDIO and @DRM_BRIDGE_OP_DP_AUDIO, -1 if it is
// not used.
//
    pub hdmi_audio_dai_port: c_int,
//
// @hdmi_cec_adapter_name: the name of the adapter to register
//
    pub hdmi_cec_adapter_name: *const c_char,
//
// @hdmi_cec_available_las: number of logical addresses, CEC_MAX_LOG_ADDRS if unset
//
    pub hdmi_cec_available_las: u8,
// private:
//
// @hpd_mutex: Protects the @hpd_cb and @hpd_data fields.
//
    pub hpd_mutex: mutex,
//
// @hpd_state_mutex: Protects the HPD en/disablement state for the bridge.
//
    pub hpd_state_mutex: mutex,
//
// @hpd_cb: Hot plug detection callback, registered with
// drm_bridge_hpd_enable().
//
    pub status): *mut *mut *mut void (hpd_cb)(void data, enum drm_connector_status,
//
// @hpd_data: Private data passed to the Hot plug detection callback
// @hpd_cb.
//
    pub hpd_data: *mut c_void,
//
// @next_bridge: Pointer to the following bridge, automatically put
// when this bridge is freed (i.e. at destroy time). This is for
// drivers needing to store a pointer to the next bridge in the
// chain, and ensures any code still holding a reference to this
// bridge after its removal cannot use-after-free the next
// bridge. Any other bridge pointers stored by the driver must be
// put in the .destroy callback by driver code.
//
    pub next_bridge: *mut drm_bridge,
}

extern "C" {
    pub fn container_of(_arg: priv, drm_bridge: struct, _arg: base) -> return;
}
extern "C" {
    pub fn drm_bridge_enter(bridge: *mut drm_bridge, idx: *mut c_int) -> bool;
}
extern "C" {
    pub fn drm_bridge_exit(idx: c_int);
}
extern "C" {
    pub fn drm_bridge_unplug(bridge: *mut drm_bridge);
}
extern "C" {
    pub fn drm_bridge_put(bridge: *mut drm_bridge);
}
extern "C" {
    pub fn drm_bridge_clear_and_put(bridge_pp: *mut drm_bridge);
}
// Cleanup action for use with __free()
//
// devm_drm_bridge_alloc - Allocate and initialize a bridge
// @dev: struct device of the bridge device
// @type: the type of the struct which contains struct &drm_bridge
// @member: the name of the &drm_bridge within @type
// @funcs: callbacks for this bridge
//
// The reference count of the returned bridge is initialized to 1. This
// reference will be automatically dropped via devm (by calling
// drm_bridge_put()) when @dev is removed.
//
// Returns:
// Pointer to new bridge, or ERR_PTR on failure.
//

extern "C" {
    pub fn drm_bridge_add(bridge: *mut drm_bridge);
}
extern "C" {
    pub fn devm_drm_bridge_add(dev: *mut device, bridge: *mut drm_bridge) -> c_int;
}
extern "C" {
    pub fn drm_bridge_remove(bridge: *mut drm_bridge);
}

extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}

extern "C" {
    pub fn list_is_last(_arg: &bridge->chain_node, _arg: &bridge->encoder->bridge_chain) -> return;
}
//
// drm_bridge_get_current_state() - Get the current bridge state
// @bridge: bridge object
//
// This function must be called with the modeset lock held.
//
// RETURNS:
//
// The current bridge state, or NULL if there is none.
//
extern "C" {
    pub fn drm_priv_to_bridge_state(_arg: bridge->base.state) -> return;
}
//
// drm_bridge_get_next_bridge() - Get the next bridge in the chain
// @bridge: bridge object
//
// The caller is responsible of having a reference to @bridge via
// drm_bridge_get() or equivalent. This function leaves the refcount of
// @bridge unmodified.
//
// The refcount of the returned bridge is incremented. Use drm_bridge_put()
// when done with it.
//
// RETURNS:
// the next bridge in the chain after @bridge, or NULL if @bridge is the last.
//
extern "C" {
    pub fn drm_bridge_get(_arg: list_next_entry(bridge, _arg: chain_node)) -> return;
}
//
// drm_bridge_get_prev_bridge() - Get the previous bridge in the chain
// @bridge: bridge object
//
// The caller is responsible of having a reference to @bridge via
// drm_bridge_get() or equivalent. This function leaves the refcount of
// @bridge unmodified.
//
// The refcount of the returned bridge is incremented. Use drm_bridge_put()
// when done with it.
//
// RETURNS:
// the previous bridge in the chain, or NULL if @bridge is the first.
//
extern "C" {
    pub fn drm_bridge_get(_arg: list_prev_entry(bridge, _arg: chain_node)) -> return;
}
//
// drm_bridge_chain_get_first_bridge() - Get the first bridge in the chain
// @encoder: encoder object
//
// The refcount of the returned bridge is incremented. Use drm_bridge_put()
// when done with it.
//
// RETURNS:
// the first bridge in the chain, or NULL if @encoder has no bridge attached
// to it.
//
// drm_bridge_chain_get_last_bridge() - Get the last bridge in the chain
// @encoder: encoder object
//
// The refcount of the returned bridge is incremented. Use drm_bridge_put()
// when done with it.
//
// RETURNS:
// the last bridge in the chain, or NULL if @encoder has no bridge attached
// to it.
//
// Internal to drm_for_each_bridge_in_chain*()
// Internal to drm_for_each_bridge_in_chain()
//
// drm_for_each_bridge_in_chain - iterate over all bridges attached to an encoder
// @encoder: the encoder to iterate bridges on
// @bridge: a bridge pointer updated to point to the current bridge at each
// iteration
//
// Iterate over all bridges present in the bridge chain attached to @encoder.
//
// Automatically gets/puts the bridge reference while iterating and locks
// the encoder chain mutex to prevent chain modifications while iterating.
//

// Internal to drm_for_each_bridge_in_chain_from()
//
// drm_for_each_bridge_in_chain_from - iterate over all bridges starting
// from the given bridge
// @first_bridge: the bridge to start from
// @bridge: a bridge pointer updated to point to the current bridge at each
// iteration
//
// Iterate over all bridges in the encoder chain starting from
// @first_bridge, included.
//
// Automatically gets/puts the bridge reference while iterating and locks
// the encoder chain mutex to prevent chain modifications while iterating.
//

extern "C" {
    pub fn drm_bridge_hpd_disable(bridge: *mut drm_bridge);
}

extern "C" {
    pub fn drm_bridge_is_panel(bridge: *const drm_bridge) -> bool;
}
extern "C" {
    pub fn drm_panel_bridge_remove(bridge: *mut drm_bridge);
}

extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}

extern "C" {
    pub fn devm_drm_put_bridge(dev: *mut device, bridge: *mut drm_bridge);
}
extern "C" {
    pub fn drm_bridge_debugfs_params(root: *mut dentry);
}
extern "C" {
    pub fn drm_bridge_debugfs_encoder_params(root: *mut dentry, encoder: *mut drm_encoder);
}
