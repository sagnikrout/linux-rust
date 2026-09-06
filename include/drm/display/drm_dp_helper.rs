//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/display/drm_dp_helper.h
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
// Copyright © 2008 Keith Packard
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

extern "C" {
    pub fn drm_dp_post_lt_adj_req_in_progress(link_status[DP_LINK_STATUS_SIZE]: u8) -> bool;
}
extern "C" {
    pub fn drm_dp_lttpr_link_train_clock_recovery_delay();
}
extern "C" {
    pub fn drm_dp_128b132b_read_aux_rd_interval(aux: *mut drm_dp_aux) -> c_int;
}
extern "C" {
    pub fn drm_dp_128b132b_eq_interlane_align_done(link_status[DP_LINK_STATUS_SIZE]: u8) -> bool;
}
extern "C" {
    pub fn drm_dp_128b132b_cds_interlane_align_done(link_status[DP_LINK_STATUS_SIZE]: u8) -> bool;
}
extern "C" {
    pub fn drm_dp_128b132b_link_training_failed(link_status[DP_LINK_STATUS_SIZE]: u8) -> bool;
}
extern "C" {
    pub fn drm_dp_link_rate_to_bw_code(link_rate: c_int) -> u8;
}
extern "C" {
    pub fn drm_dp_bw_code_to_link_rate(link_bw: u8) -> c_int;
}
//
// struct drm_dp_vsc_sdp - drm DP VSC SDP
//
// This structure represents a DP VSC SDP of drm
// It is based on DP 1.4 spec [Table 2-116: VSC SDP Header Bytes] and
// [Table 2-117: VSC SDP Payload for DB16 through DB18]
//
// @sdp_type: secondary-data packet type
// @revision: revision number
// @length: number of valid data bytes
// @pixelformat: pixel encoding format
// @colorimetry: colorimetry format
// @bpc: bit per color
// @dynamic_range: dynamic range information
// @content_type: CTA-861-G defines content types and expected processing by a sink device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_dp_vsc_sdp {
    pub sdp_type: c_uchar,
    pub revision: c_uchar,
    pub length: c_uchar,
    pub pixelformat: dp_pixelformat,
    pub colorimetry: dp_colorimetry,
    pub bpc: c_int,
    pub dynamic_range: dp_dynamic_range,
    pub content_type: dp_content_type,
}

//
// struct drm_dp_as_sdp - drm DP Adaptive Sync SDP
//
// This structure represents a DP AS SDP of drm
// It is based on DP 2.1 spec [Table 2-126:  Adaptive-Sync SDP Header Bytes] and
// [Table 2-127: Adaptive-Sync SDP Payload for DB0 through DB8]
//
// @sdp_type: Secondary-data packet type
// @revision: Revision Number
// @length: Number of valid data bytes
// @vtotal: Minimum Vertical Vtotal
// @target_rr: Target Refresh
// @duration_incr_ms: Successive frame duration increase
// @duration_decr_ms: Successive frame duration decrease
// @target_rr_divider: Target refresh rate divider
// @mode: Adaptive Sync Operation Mode
// @coasting_vtotal: Coasting vtotal
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_dp_as_sdp {
    pub sdp_type: c_uchar,
    pub revision: c_uchar,
    pub length: c_uchar,
    pub vtotal: c_int,
    pub target_rr: c_int,
    pub duration_incr_ms: c_int,
    pub duration_decr_ms: c_int,
    pub target_rr_divider: bool,
    pub mode: operation_mode,
    pub coasting_vtotal: c_int,
}

extern "C" {
    pub fn drm_dp_vsc_sdp_log(p: *mut drm_printer, vsc: *const drm_dp_vsc_sdp);
}
extern "C" {
    pub fn drm_dp_vsc_sdp_supported(aux: *mut drm_dp_aux, dpcd[DP_RECEIVER_CAP_SIZE]: u8) -> bool;
}
extern "C" {
    pub fn drm_dp_as_sdp_supported(aux: *mut drm_dp_aux, dpcd[DP_RECEIVER_CAP_SIZE]: u8) -> bool;
}
extern "C" {
    pub fn drm_dp_psr_setup_time(psr_cap[EDP_PSR_RECEIVER_CAP_SIZE]: u8) -> c_int;
}
extern "C" {
    pub fn drm_dp_bw_code_to_link_rate(_arg: dpcd[DP_MAX_LINK_RATE]) -> return;
}
// DP/eDP DSC support
extern "C" {
    pub fn drm_dp_dsc_sink_bpp_incr(dsc_dpcd[DP_DSC_RECEIVER_CAP_SIZE]: u8) -> u8;
}
extern "C" {
    pub fn drm_dp_dsc_slice_count_to_mask(slice_count: c_int) -> u32;
}
extern "C" {
    pub fn drm_dp_dsc_sink_line_buf_depth(dsc_dpcd[DP_DSC_RECEIVER_CAP_SIZE]: u8) -> u8;
}
extern "C" {
    pub fn drm_dp_dsc_branch_max_line_width(dsc_branch_dpcd[DP_DSC_BRANCH_CAP_SIZE]: u8) -> c_int;
}
// Max Slicewidth = Number of Pixels * 320
//
// drm_dp_dsc_sink_supports_format() - check if sink supports DSC with given output format
// @dsc_dpcd : DSC-capability DPCDs of the sink
// @output_format: output_format which is to be checked
//
// Returns true if the sink supports DSC with the given output_format, false otherwise.
//
// Forward Error Correction Support on DP 1.4
// Ignore MSA timing for Adaptive Sync support on DP 1.4
//
// drm_edp_backlight_supported() - Check an eDP DPCD for VESA backlight support
// @edp_dpcd: The DPCD to check
//
// Note that currently this function will return %false for panels which support various DPCD
// backlight features but which require the brightness be set through PWM, and don't support setting
// the brightness level via the DPCD.
//
// Returns: %True if @edp_dpcd indicates that VESA backlight controls are supported, %false
// otherwise
//
// drm_dp_is_uhbr_rate - Determine if a link rate is UHBR
// @link_rate: link rate in 10kbits/s units
//
// Determine if the provided link rate is an UHBR rate.
//
// Returns: %True if @link_rate is an UHBR rate.
//
// DisplayPort AUX channel
//
// struct drm_dp_aux_msg - DisplayPort AUX channel transaction
// @address: address of the (first) register to access
// @request: contains the type of transaction (see DP_AUX_* macros)
// @reply: upon completion, contains the reply type of the transaction
// @buffer: pointer to a transmission or reception buffer
// @size: size of @buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_dp_aux_msg {
    pub address: c_uint,
    pub request: u8,
    pub reply: u8,
    pub buffer: *mut c_void,
    pub size: usize,
}

//
// struct drm_dp_aux_cec - DisplayPort CEC-Tunneling-over-AUX
// @lock: mutex protecting this struct
// @adap: the CEC adapter for CEC-Tunneling-over-AUX support.
// @connector: the connector this CEC adapter is associated with
// @unregister_work: unregister the CEC adapter
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_dp_aux_cec {
    pub lock: mutex,
    pub adap: *mut cec_adapter,
    pub connector: *mut drm_connector,
    pub unregister_work: delayed_work,
}

//
// struct drm_dp_aux - DisplayPort AUX channel
//
// An AUX channel can also be used to transport I2C messages to a sink. A
// typical application of that is to access an EDID that's present in the sink
// device. The @transfer() function can also be used to execute such
// transactions. The drm_dp_aux_register() function registers an I2C adapter
// that can be passed to drm_probe_ddc(). Upon removal, drivers should call
// drm_dp_aux_unregister() to remove the I2C adapter. The I2C adapter uses long
// transfers by default; if a partial response is received, the adapter will
// drop down to the size given by the partial response for this transaction
// only.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_dp_aux {
//
// @name: user-visible name of this AUX channel and the
// I2C-over-AUX adapter.
//
// It's also used to specify the name of the I2C adapter. If set
// to %NULL, dev_name() of @dev will be used.
//
    pub name: *const c_char,
//
// @ddc: I2C adapter that can be used for I2C-over-AUX
// communication
//
    pub ddc: i2c_adapter,
//
// @dev: pointer to struct device that is the parent for this
// AUX channel.
//
    pub dev: *mut device,
//
// @drm_dev: pointer to the &drm_device that owns this AUX channel.
// Beware, this may be %NULL before drm_dp_aux_register() has been
// called.
//
// It should be set to the &drm_device that will be using this AUX
// channel as early as possible. For many graphics drivers this should
// happen before drm_dp_aux_init(), however it's perfectly fine to set
// this field later so long as it's assigned before calling
// drm_dp_aux_register().
//
    pub drm_dev: *mut drm_device,
//
// @crtc: backpointer to the crtc that is currently using this
// AUX channel
//
    pub crtc: *mut drm_crtc,
//
// @hw_mutex: internal mutex used for locking transfers.
//
// Note that if the underlying hardware is shared among multiple
// channels, the driver needs to do additional locking to
// prevent concurrent access.
//
    pub hw_mutex: mutex,
//
// @crc_work: worker that captures CRCs for each frame
//
    pub crc_work: work_struct,
//
// @crc_count: counter of captured frame CRCs
//
    pub crc_count: u8,
//
// @transfer: transfers a message representing a single AUX
// transaction.
//
// This is a hardware-specific implementation of how
// transactions are executed that the drivers must provide.
//
// A pointer to a &drm_dp_aux_msg structure describing the
// transaction is passed into this function. Upon success, the
// implementation should return the number of payload bytes that
// were transferred, or a negative error-code on failure.
//
// Helpers will propagate these errors, with the exception of
// the %-EBUSY error, which causes a transaction to be retried.
// On a short, helpers will return %-EPROTO to make it simpler
// to check for failure.
//
// The @transfer() function must only modify the reply field of
// the &drm_dp_aux_msg structure. The retry logic and i2c
// helpers assume this is the case.
//
// Also note that this callback can be called no matter the
// state @dev is in and also no matter what state the panel is
// in. It's expected:
//
// - If the @dev providing the AUX bus is currently unpowered then
// it will power itself up for the transfer.
//
// - If we're on eDP (using a drm_panel) and the panel is not in a
// state where it can respond (it's not powered or it's in a
// low power state) then this function may return an error, but
// not crash. It's up to the caller of this code to make sure that
// the panel is powered on if getting an error back is not OK. If a
// drm_panel driver is initiating a DP AUX transfer it may power
// itself up however it wants. All other code should ensure that
// the pre_enable() bridge chain (which eventually calls the
// drm_panel prepare function) has powered the panel.
//
    pub msg): *mut drm_dp_aux_msg,
//
// @wait_hpd_asserted: wait for HPD to be asserted
//
// This is mainly useful for eDP panels drivers to wait for an eDP
// panel to finish powering on. It is optional for DP AUX controllers
// to implement this function. It is required for DP AUX endpoints
// (panel drivers) to call this function after powering up but before
// doing AUX transfers unless the DP AUX endpoint driver knows that
// we're not using the AUX controller's HPD. One example of the panel
// driver not needing to call this is if HPD is hooked up to a GPIO
// that the panel driver can read directly.
//
// If a DP AUX controller does not implement this function then it
// may still support eDP panels that use the AUX controller's built-in
// HPD signal by implementing a long wait for HPD in the transfer()
// callback, though this is deprecated.
//
// This function will efficiently wait for the HPD signal to be
// asserted. The `wait_us` parameter that is passed in says that we
// know that the HPD signal is expected to be asserted within `wait_us`
// microseconds. This function could wait for longer than `wait_us` if
// the logic in the DP controller has a long debouncing time. The
// important thing is that if this function returns success that the
// DP controller is ready to send AUX transactions.
//
// This function returns 0 if HPD was asserted or -ETIMEDOUT if time
// expired and HPD wasn't asserted. This function should not print
// timeout errors to the log.
//
// The semantics of this function are designed to match the
// readx_poll_timeout() function. That means a `wait_us` of 0 means
// to wait forever. Like readx_poll_timeout(), this function may sleep.
//
// NOTE: this function specifically reports the state of the HPD pin
// that's associated with the DP AUX channel. This is different from
// the HPD concept in much of the rest of DRM which is more about
// physical presence of a display. For eDP, for instance, a display is
// assumed always present even if the HPD pin is deasserted.
//
    pub wait_us): *mut *mut *mut int (wait_hpd_asserted)(struct drm_dp_aux aux, unsigned long,
//
// @i2c_nack_count: Counts I2C NACKs, used for DP validation.
//
    pub i2c_nack_count: unsigned,
//
// @i2c_defer_count: Counts I2C DEFERs, used for DP validation.
//
    pub i2c_defer_count: unsigned,
//
// @cec: struct containing fields used for CEC-Tunneling-over-AUX.
//
    pub cec: drm_dp_aux_cec,
//
// @is_remote: Is this AUX CH actually using sideband messaging.
//
    pub is_remote: bool,
//
// @powered_down: If true then the remote endpoint is powered down.
//
    pub powered_down: bool,
//
// @no_zero_sized: If the hw can't use zero sized transfers (NVIDIA)
//
    pub no_zero_sized: bool,
//
// @dpcd_probe_disabled: If probing before a DPCD access is disabled.
//
    pub dpcd_probe_disabled: bool,
}

extern "C" {
    pub fn drm_dp_dpcd_probe(aux: *mut drm_dp_aux, offset: c_uint) -> c_int;
}
extern "C" {
    pub fn drm_dp_dpcd_set_powered(aux: *mut drm_dp_aux, powered: bool);
}
extern "C" {
    pub fn drm_dp_dpcd_set_probe(aux: *mut drm_dp_aux, enable: bool);
}
//
// drm_dp_dpcd_readb() - read a single byte from the DPCD
// @aux: DisplayPort AUX channel
// @offset: address of the register to read
// @valuep: location where the value of the register will be stored
//
// Returns the number of bytes transferred (1) on success, or a negative
// error code on failure. In most of the cases you should be using
// drm_dp_dpcd_read_byte() instead.
//
extern "C" {
    pub fn drm_dp_dpcd_read(_arg: aux, _arg: offset, _arg: valuep, _arg: 1) -> return;
}
//
// drm_dp_dpcd_read_data() - read a series of bytes from the DPCD
// @aux: DisplayPort AUX channel (SST or MST)
// @offset: address of the (first) register to read
// @buffer: buffer to store the register values
// @size: number of bytes in @buffer
//
// Returns zero (0) on success, or a negative error
// code on failure. -EIO is returned if the request was NAKed by the sink or
// if the retry count was exceeded. If not all bytes were transferred, this
// function returns -EPROTO. Errors from the underlying AUX channel transfer
// function, with the exception of -EBUSY (which causes the transaction to
// be retried), are propagated to the caller.
//
// Workaround for USB-C hubs/adapters with buggy firmware that fail
// multi-byte AUX reads but work with single-byte reads.
// Known affected devices:
// - Lenovo USB-C to VGA adapter (VIA VL817, idVendor=17ef, idProduct=7217)
// - Dell DA310 USB-C hub (idVendor=413c, idProduct=c010)
// Attempt byte-by-byte reading as a fallback.
//
// drm_dp_dpcd_write_data() - write a series of bytes to the DPCD
// @aux: DisplayPort AUX channel (SST or MST)
// @offset: address of the (first) register to write
// @buffer: buffer containing the values to write
// @size: number of bytes in @buffer
//
// Returns zero (0) on success, or a negative error
// code on failure. -EIO is returned if the request was NAKed by the sink or
// if the retry count was exceeded. If not all bytes were transferred, this
// function returns -EPROTO. Errors from the underlying AUX channel transfer
// function, with the exception of -EBUSY (which causes the transaction to
// be retried), are propagated to the caller.
//
// drm_dp_dpcd_writeb() - write a single byte to the DPCD
// @aux: DisplayPort AUX channel
// @offset: address of the register to write
// @value: value to write to the register
//
// Returns the number of bytes transferred (1) on success, or a negative
// error code on failure. In most of the cases you should be using
// drm_dp_dpcd_write_byte() instead.
//
extern "C" {
    pub fn drm_dp_dpcd_write(_arg: aux, _arg: offset, _arg: &value, _arg: 1) -> return;
}
//
// drm_dp_dpcd_read_byte() - read a single byte from the DPCD
// @aux: DisplayPort AUX channel
// @offset: address of the register to read
// @valuep: location where the value of the register will be stored
//
// Returns zero (0) on success, or a negative error code on failure.
//
extern "C" {
    pub fn drm_dp_dpcd_read_data(_arg: aux, _arg: offset, _arg: valuep, _arg: 1) -> return;
}
//
// drm_dp_dpcd_write_byte() - write a single byte to the DPCD
// @aux: DisplayPort AUX channel
// @offset: address of the register to write
// @value: value to write to the register
//
// Returns zero (0) on success, or a negative error code on failure.
//
extern "C" {
    pub fn drm_dp_dpcd_write_data(_arg: aux, _arg: offset, _arg: &value, _arg: 1) -> return;
}
extern "C" {
    pub fn drm_dp_link_power_up(aux: *mut drm_dp_aux, revision: c_uchar) -> c_int;
}
extern "C" {
    pub fn drm_dp_link_power_down(aux: *mut drm_dp_aux, revision: c_uchar) -> c_int;
}
extern "C" {
    pub fn drm_dp_dpcd_clear_payload(aux: *mut drm_dp_aux) -> c_int;
}
extern "C" {
    pub fn drm_dp_dpcd_poll_act_handled(aux: *mut drm_dp_aux, timeout_ms: c_int) -> c_int;
}
extern "C" {
    pub fn drm_dp_downstream_id(aux: *mut drm_dp_aux, id[6]: c_char) -> c_int;
}
extern "C" {
    pub fn drm_dp_read_sink_count(aux: *mut drm_dp_aux) -> c_int;
}
extern "C" {
    pub fn drm_dp_lttpr_count(cap[DP_LTTPR_COMMON_CAP_SIZE]: u8) -> c_int;
}
extern "C" {
    pub fn drm_dp_lttpr_max_link_rate(caps[DP_LTTPR_COMMON_CAP_SIZE]: u8) -> c_int;
}
extern "C" {
    pub fn drm_dp_lttpr_set_transparent_mode(aux: *mut drm_dp_aux, enable: bool) -> c_int;
}
extern "C" {
    pub fn drm_dp_lttpr_init(aux: *mut drm_dp_aux, lttpr_count: c_int) -> c_int;
}
extern "C" {
    pub fn drm_dp_lttpr_max_lane_count(caps[DP_LTTPR_COMMON_CAP_SIZE]: u8) -> c_int;
}
extern "C" {
    pub fn drm_dp_lttpr_voltage_swing_level_3_supported(caps[DP_LTTPR_PHY_CAP_SIZE]: u8) -> bool;
}
extern "C" {
    pub fn drm_dp_lttpr_pre_emphasis_level_3_supported(caps[DP_LTTPR_PHY_CAP_SIZE]: u8) -> bool;
}
extern "C" {
    pub fn drm_dp_lttpr_wake_timeout_setup(aux: *mut drm_dp_aux, transparent_mode: bool);
}
extern "C" {
    pub fn drm_dp_remote_aux_init(aux: *mut drm_dp_aux);
}
extern "C" {
    pub fn drm_dp_aux_init(aux: *mut drm_dp_aux);
}
extern "C" {
    pub fn drm_dp_aux_register(aux: *mut drm_dp_aux) -> c_int;
}
extern "C" {
    pub fn drm_dp_aux_unregister(aux: *mut drm_dp_aux);
}
extern "C" {
    pub fn drm_dp_start_crc(aux: *mut drm_dp_aux, crtc: *mut drm_crtc) -> c_int;
}
extern "C" {
    pub fn drm_dp_stop_crc(aux: *mut drm_dp_aux) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_dp_dpcd_ident {
    pub oui: [u8; 3],
    pub device_id: [u8; 6],
    pub hw_rev: u8,
    pub sw_major_rev: u8,
    pub sw_minor_rev: u8,
    pub __packed: },
//
// struct drm_dp_desc - DP branch/sink device descriptor
// @ident: DP device identification from DPCD 0x400 (sink) or 0x500 (branch).
// @quirks: Quirks; use drm_dp_has_quirk() to query for the quirks.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_dp_desc {
    pub ident: drm_dp_dpcd_ident,
    pub quirks: u32,
}

extern "C" {
    pub fn drm_dp_dump_lttpr_desc(aux: *mut drm_dp_aux, dp_phy: drm_dp_phy) -> c_int;
}
//
// enum drm_dp_quirk - Display Port sink/branch device specific quirks
//
// Display Port sink and branch devices in the wild have a variety of bugs, try
// to collect them here. The quirks are shared, but it's up to the drivers to
// implement workarounds for them.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_dp_quirk {
//
// @DP_DPCD_QUIRK_CONSTANT_N:
//
// The device requires main link attributes Mvid and Nvid to be limited
// to 16 bits. So will give a constant value (0x8000) for compatability.
//
    DP_DPCD_QUIRK_CONSTANT_N,
//
// @DP_DPCD_QUIRK_NO_PSR:
//
// The device does not support PSR even if reports that it supports or
// driver still need to implement proper handling for such device.
//
    DP_DPCD_QUIRK_NO_PSR,
//
// @DP_DPCD_QUIRK_NO_SINK_COUNT:
//
// The device does not set SINK_COUNT to a non-zero value.
// The driver should ignore SINK_COUNT during detection. Note that
// drm_dp_read_sink_count_cap() automatically checks for this quirk.
//
    DP_DPCD_QUIRK_NO_SINK_COUNT,
//
// @DP_DPCD_QUIRK_DSC_WITHOUT_VIRTUAL_DPCD:
//
// The device supports MST DSC despite not supporting Virtual DPCD.
// The DSC caps can be read from the physical aux instead.
//
    DP_DPCD_QUIRK_DSC_WITHOUT_VIRTUAL_DPCD,
//
// @DP_DPCD_QUIRK_CAN_DO_MAX_LINK_RATE_3_24_GBPS:
//
// The device supports a link rate of 3.24 Gbps (multiplier 0xc) despite
// the DP_MAX_LINK_RATE register reporting a lower max multiplier.
//
    DP_DPCD_QUIRK_CAN_DO_MAX_LINK_RATE_3_24_GBPS,
//
// @DP_DPCD_QUIRK_HBLANK_EXPANSION_REQUIRES_DSC:
//
// The device applies HBLANK expansion for some modes, but this
// requires enabling DSC.
//
    DP_DPCD_QUIRK_HBLANK_EXPANSION_REQUIRES_DSC,
//
// @DP_DPCD_QUIRK_DSC_THROUGHPUT_BPP_LIMIT:
//
// The device doesn't support DSC decompression at the maximum DSC
// pixel throughput and compressed bpp it indicates via its DPCD DSC
// capabilities. The compressed bpp must be limited above a device
// specific DSC pixel throughput.
//
    DP_DPCD_QUIRK_DSC_THROUGHPUT_BPP_LIMIT,
}

//
// drm_dp_has_quirk() - does the DP device have a specific quirk
// @desc: Device descriptor filled by drm_dp_read_desc()
// @quirk: Quirk to query for
//
// Return true if DP device identified by @desc has @quirk.
//
// struct drm_edp_backlight_info - Probed eDP backlight info struct
// @pwmgen_bit_count: The pwmgen bit count
// @pwm_freq_pre_divider: The PWM frequency pre-divider value being used for this backlight, if any
// @max: The maximum backlight level that may be set
// @lsb_reg_used: Do we also write values to the DP_EDP_BACKLIGHT_BRIGHTNESS_LSB register?
// @aux_enable: Does the panel support the AUX enable cap?
// @aux_set: Does the panel support setting the brightness through AUX?
// @luminance_set: Does the panel support setting the brightness through AUX using luminance values?
//
// This structure contains various data about an eDP backlight, which can be populated by using
// drm_edp_backlight_init().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_edp_backlight_info {
    pub pwmgen_bit_count: u8,
    pub pwm_freq_pre_divider: u8,
    pub max: u32,
    pub 1: bool lsb_reg_used :,
    pub 1: bool aux_enable :,
    pub 1: bool aux_set :,
    pub 1: bool luminance_set :,
}

extern "C" {
    pub fn drm_edp_backlight_disable(aux: *mut drm_dp_aux, bl: *const drm_edp_backlight_info) -> c_int;
}

extern "C" {
    pub fn drm_panel_dp_aux_backlight(panel: *mut drm_panel, aux: *mut drm_dp_aux) -> c_int;
}

extern "C" {
    pub fn drm_dp_cec_irq(aux: *mut drm_dp_aux);
}
extern "C" {
    pub fn drm_dp_cec_unregister_connector(aux: *mut drm_dp_aux);
}
extern "C" {
    pub fn drm_dp_cec_attach(aux: *mut drm_dp_aux, source_physical_address: u16);
}
extern "C" {
    pub fn drm_dp_cec_set_edid(aux: *mut drm_dp_aux, edid: *const edid);
}
extern "C" {
    pub fn drm_dp_cec_unset_edid(aux: *mut drm_dp_aux);
}

//
// struct drm_dp_phy_test_params - DP Phy Compliance parameters
// @link_rate: Requested Link rate from DPCD 0x219
// @num_lanes: Number of lanes requested by sing through DPCD 0x220
// @phy_pattern: DP Phy test pattern from DPCD 0x248
// @hbr2_reset: DP HBR2_COMPLIANCE_SCRAMBLER_RESET from DCPD 0x24A and 0x24B
// @custom80: DP Test_80BIT_CUSTOM_PATTERN from DPCDs 0x250 through 0x259
// @enhanced_frame_cap: flag for enhanced frame capability.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_dp_phy_test_params {
    pub link_rate: c_int,
    pub num_lanes: u8,
    pub phy_pattern: u8,
    pub hbr2_reset: [u8; 2],
    pub custom80: [u8; 10],
    pub enhanced_frame_cap: bool,
}

extern "C" {
    pub fn drm_dp_pcon_frl_prepare(aux: *mut drm_dp_aux, enable_frl_ready_hpd: bool) -> c_int;
}
extern "C" {
    pub fn drm_dp_pcon_is_frl_ready(aux: *mut drm_dp_aux) -> bool;
}
extern "C" {
    pub fn drm_dp_pcon_reset_frl_config(aux: *mut drm_dp_aux) -> c_int;
}
extern "C" {
    pub fn drm_dp_pcon_frl_enable(aux: *mut drm_dp_aux) -> c_int;
}
extern "C" {
    pub fn drm_dp_pcon_hdmi_link_active(aux: *mut drm_dp_aux) -> bool;
}
extern "C" {
    pub fn drm_dp_pcon_hdmi_link_mode(aux: *mut drm_dp_aux, frl_trained_mask: *mut u8) -> c_int;
}
extern "C" {
    pub fn drm_dp_pcon_enc_is_dsc_1_2(pcon_dsc_dpcd[DP_PCON_DSC_ENCODER_CAP_SIZE]: u8) -> bool;
}
extern "C" {
    pub fn drm_dp_pcon_dsc_max_slices(pcon_dsc_dpcd[DP_PCON_DSC_ENCODER_CAP_SIZE]: u8) -> c_int;
}
extern "C" {
    pub fn drm_dp_pcon_dsc_max_slice_width(pcon_dsc_dpcd[DP_PCON_DSC_ENCODER_CAP_SIZE]: u8) -> c_int;
}
extern "C" {
    pub fn drm_dp_pcon_dsc_bpp_incr(pcon_dsc_dpcd[DP_PCON_DSC_ENCODER_CAP_SIZE]: u8) -> c_int;
}
extern "C" {
    pub fn drm_dp_pcon_pps_default(aux: *mut drm_dp_aux) -> c_int;
}
extern "C" {
    pub fn drm_dp_pcon_pps_override_buf(aux: *mut drm_dp_aux, pps_buf[128]: u8) -> c_int;
}
extern "C" {
    pub fn drm_dp_pcon_pps_override_param(aux: *mut drm_dp_aux, pps_param[6]: u8) -> c_int;
}
extern "C" {
    pub fn drm_dp_pcon_convert_rgb_to_ycbcr(aux: *mut drm_dp_aux, color_spc: u8) -> c_int;
}

extern "C" {
    pub fn drm_dp_bw_channel_coding_efficiency(is_uhbr: bool) -> c_int;
}
extern "C" {
    pub fn drm_dp_max_dprx_data_rate(max_link_rate: c_int, max_lanes: c_int) -> c_int;
}
extern "C" {
    pub fn drm_dp_vsc_sdp_pack(vsc: *const drm_dp_vsc_sdp, sdp: *mut dp_sdp) -> isize;
}
