//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/inc/link_service.h
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
// Copyright 2022 Advanced Micro Devices, Inc.
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
// Authors: AMD
//
// FILE POLICY AND INTENDED USAGE:
//
// This header defines link component function interfaces aka link_service.
// link_service provides the only entry point to link functions with function
// pointer style. This header is strictly private in dc and should never be
// included by DM because it exposes too much dc detail including all dc
// private types defined in core_types.h. Otherwise it will break DM - DC
// encapsulation and turn DM into a maintenance nightmare.
//
// The following shows a link component relation map.
//
// DM to DC:
// DM includes dc.h
// dc_link_exports.c or other dc files implement dc.h
//
// DC to Link:
// dc_link_exports.c or other dc files include link_service.h
// link_factory.c implements link_service.h
//
// Link sub-component to Link sub-component:
// link_factory.c includes --> link_xxx.h
// link_xxx.c implements link_xxx.h
// As you can see if you ever need to add a new dc link function and call it on
// DM/dc side, it is very difficult because you will need layers of translation.
// The most appropriate approach to implement new requirements on DM/dc side is
// to extend or generalize the functionality of existing link function
// interfaces so minimal modification is needed outside link component to
// achieve your new requirements. This approach reduces or even eliminates the
// effort needed outside link component to support a new link feature. This also
// reduces code discrepancy among DMs to support the same link feature. If we
// test full code path on one version of DM, and there is no feature specific
// modification required on other DMs, then we can have higher confidence that
// the feature will run on other DMs and produce the same result. The following
// are some good examples to start with:
//
// - detect_link --> to add new link detection or capability retrieval routines
//
// - validate_mode_timing --> to add new timing validation conditions
//
// - set_dpms_on/set_dpms_off --> to include new link enablement sequences
//
// If you must add new link functions, you will need to:
// 1. declare the function pointer here under the suitable commented category.
// 2. Implement your function in the suitable link_xxx.c file.
// 3. Assign the function to link_service in link_factory.c
// 4. NEVER include link_xxx.h headers outside link component.
// 5. NEVER include link_service.h on DM side.
//

extern "C" {
    pub fn link_destroy_link_service(link_srv: *mut link_service);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct link_init_data {
    pub dc: *const dc,
    pub /: *mut *mut *mut dc_context ctx; / TODO: remove 'dal' when DC is complete.,
    pub /: *mut *mut uint32_t connector_index; / this will be mapped to the HPD pins,
    pub display_index: *mut *mut uint32_t link_index; / this is mapped to DAL,
    pub is_dpia_link: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ddc_service_init_data {
    pub id: graphics_object_id,
    pub ctx: *mut dc_context,
    pub link: *mut dc_link,
    pub is_dpia_link: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct link_service {
// Factory
    pub init_params): *const link_init_data,
    pub link): *mut *mut void (destroy_link)(struct dc_link,
// Detection
    pub reason): *mut *mut *mut bool (detect_link)(struct dc_link link, enum dc_detect_reason,
    pub type): *mut dc_connection_type,
    pub init_data): *mut dc_sink_init_data,
    pub sink): *mut *mut *mut void (remove_remote_sink)(struct dc_link link, struct dc_sink,
    pub link): *mut *mut bool (get_hpd_state)(struct dc_link,
    pub link): *const *const void (enable_hpd)(struct dc_link,
    pub link): *const *const void (disable_hpd)(struct dc_link,
    pub enable): *mut *mut *mut void (enable_hpd_filter)(struct dc_link link, bool,
    pub link): *mut *mut bool (reset_cur_dp_mst_topology)(struct dc_link,
    pub link): *const *const *const dc_link_status (get_status)(dc_link,
    pub signal): signal_type,
    pub signal): signal_type,
    pub link): *mut *mut void (clear_dprx_states)(struct dc_link,
// Resource
    pub map): *const *const *const void (get_cur_res_map)(struct dc dc, uint32_t,
    pub map): *const *const *const void (restore_res_map)(struct dc dc, uint32_t,
    pub link_res): *mut link_resource,
// Validation
    pub timing): *const dc_crtc_timing,
    pub link_settings): *const dc_link_settings,
    pub new_ctx): *const dc_state,
    pub link_rate): *mut *mut uint32_t (frl_link_bandwidth_kbps)(enum hdmi_frl_link_rate,
    pub inter): *mut frl_cap_chk_intermediates_fixed31_32,
    pub audio_params): *mut dp_audio_bandwidth_params,
// DPMS
    pub pipe_ctx): *mut *mut *mut dc_status (set_dpms_on)(struct dc_state state, struct pipe_ctx,
    pub pipe_ctx): *mut *mut dc_status (set_dpms_off)(struct pipe_ctx,
    pub link): *mut *mut void (resume)(struct dc_link,
    pub dc): *mut *mut void (blank_all_dp_displays)(struct dc,
    pub dc): *mut *mut void (blank_all_edp_displays)(struct dc,
    pub hw_init): *mut *mut *mut void (blank_dp_stream)(struct dc_link link, bool,
    pub req_pbn): *mut *mut pipe_ctx pipe_ctx, uint32_t,
    pub req_pbn): *mut *mut pipe_ctx pipe_ctx, uint32_t,
    pub enable): *mut *mut *mut void (set_dsc_on_stream)(struct pipe_ctx pipe_ctx, bool,
    pub enable): *mut *mut *mut bool (set_dsc_enable)(struct pipe_ctx pipe_ctx, bool,
    pub pipe_ctx): *mut *mut bool (update_dsc_config)(struct pipe_ctx,
    pub link): *mut *mut void (wait_for_unlocked)(struct dc_link,
// DDC
    pub ddc_init_data): *mut ddc_service_init_data,
    pub ddc): *mut *mut void (destroy_ddc_service)(struct ddc_service,
    pub read_size): u32,
    pub operation_result): *mut aux_return_code_type,
    pub len): u32,
    pub payload): *mut aux_payload,
    pub ddc): *mut *mut bool (is_in_aux_transaction_mode)(struct ddc_service,
    pub ddc): *mut *mut uint32_t (get_aux_defer_delay)(struct ddc_service,
    pub link): *const *const uint8_t (get_ddc_aux_inst)(struct dc_link,
// DP Capability
    pub link): *mut *mut bool (dp_is_sink_present)(struct dc_link,
    pub link): *const *const bool (dp_is_fec_supported)(struct dc_link,
    pub pipe_ctx): *mut *mut bool (dp_is_128b_132b_signal)(struct pipe_ctx,
    pub max_link_enc_cap): *mut dc_link_settings,
    pub link): *const dc_link,
    pub link_settings): *const dc_link_settings,
    pub link): *const *const bool (dp_should_enable_fec)(struct dc_link,
    pub link_setting): *mut dc_link_settings,
    pub dp_tunnel_setting): *mut dc_tunnel_settings,
    pub link): *const dc_link,
    pub req_bw): *mut *mut dc_link_settings link_setting, uint32_t,
    pub bw): *mut *mut uint32_t (bw_kbps_from_raw_frl_link_rate_data)(uint8_t,
    pub link): *mut *mut bool (dp_overwrite_extended_receiver_cap)(struct dc_link,
    pub link_setting): *mut dc_link_settings,
    pub link): *mut *mut uint8_t (dp_get_lttpr_count)(struct dc_link,
    pub auxwake_support): *mut bool,
// DP DPIA/PHY
    pub peak_bw): *mut *mut dc_link link, int,
    pub lt_settings): *mut link_training_settings,
    pub on): *mut *mut *mut void (dpcd_write_rx_power_ctrl)(struct dc_link link, bool,
// DP IRQ Handler
    pub hpd_irq_dpcd_data): *mut hpd_irq_data,
    pub link): *const *const bool (dp_should_allow_hpd_rx_irq)(struct dc_link,
    pub link): *mut *mut void (dp_handle_link_loss)(struct dc_link,
    pub irq_data): *mut hpd_irq_data,
    pub has_left_work): *mut bool defer_handling, bool,
// eDP Panel Control
    pub wait_for_hpd): *mut *mut dc_link link, bool,
    pub link): *const *const int (edp_get_backlight_level)(struct dc_link,
    pub backlight_millinits_peak): *mut u32,
    pub backlight_level_params): *mut set_backlight_level_params,
    pub transition_time_in_ms): u32,
    pub link): *const *const int (edp_get_target_backlight_pwm)(struct dc_link,
    pub state): *const *const dc_link link, enum dc_psr_state,
    pub power_opts): *const c_uint,
    pub psr_context): *mut psr_context,
    pub psr_vtotal_su): u16,
    pub mode): *const *const *const dc_link link, uint32_t residency, enum psr_residency_mode,
    pub state): *const *const dc_link link, uint64_t,
    pub power_opts): *const c_uint,
    pub cmd_data): *mut dmub_replay_cmd_set,
    pub frame_skip_number): *mut *mut dc_link link, uint32_t coasting_vtotal, uint16_t,
    pub mode): pr_residency_mode,
    pub frame_skip_number): *const *const unsigned int power_opts, uint32_t coasting_vtotal, uint16_t,
    pub link): *mut *mut bool (edp_wait_for_t12)(struct dc_link,
    pub crtc_timing): *mut dc_crtc_timing,
    pub enable): *mut *mut *mut bool (edp_backlight_enable_aux)(struct dc_link link, bool,
    pub link): *mut *mut void (edp_add_delay_for_T9)(struct dc_link,
    pub link): *mut *mut bool (edp_receiver_ready_T9)(struct dc_link,
    pub link): *mut *mut bool (edp_receiver_ready_T7)(struct dc_link,
    pub enable): *mut *mut *mut bool (edp_power_alpm_dpcd_enable)(struct dc_link link, bool,
    pub stream): *const *const *const bool (dp_setup_replay)(struct dc_link link, struct dc_stream_state,
    pub inst_out): *const *const *const *const bool (dp_pr_get_panel_inst)(struct dc dc, struct dc_link link, unsigned int,
    pub enable): *mut *mut *mut bool (dp_pr_enable)(struct dc_link link, bool,
    pub update_state_data): *mut *mut *mut bool (dp_pr_update_state)(struct dc_link link, struct dmub_cmd_pr_update_state_data,
    pub general_cmd_data): *mut *mut *mut bool (dp_pr_set_general_cmd)(struct dc_link link, struct dmub_cmd_pr_general_cmd_data,
    pub state): *const *const *const bool (dp_pr_get_state)(struct dc_link link, uint64_t,
    pub powerOn): *mut *mut *mut void (edp_set_panel_power)(struct dc_link link, bool,
// HDMI FRL
    pub link): *mut *mut bool (hdmi_frl_poll_status_flag)(struct dc_link,
    pub link): *mut dc_link,
    pub link): *mut dc_link,
// DP CTS
    pub link): *mut *mut void (dp_handle_automated_test)(struct dc_link,
    pub cust_pattern_size): c_uint,
    pub link): *mut dc_link,
    pub skip_immediate_retrain): bool,
// DP Trace
    pub link): *mut *mut bool (dp_trace_is_initialized)(struct dc_link,
    pub is_logged): bool,
    pub in_detection): *mut *mut *mut bool (dp_trace_is_logged)(struct dc_link link, bool,
    pub in_detection): *mut *mut dc_link link, bool,
    pub in_detection): *mut *mut dc_link link, bool,
    pub link): *mut *mut unsigned int (dp_trace_get_link_loss_count)(struct dc_link,
    pub power_up): bool,
    pub link): *mut *mut uint64_t (dp_trace_get_edp_poweron_timestamp)(struct dc_link,
    pub link): *mut *mut uint64_t (dp_trace_get_edp_poweroff_timestamp)(struct dc_link,
    pub dp_test_mode): *mut *mut dc_link link, uint8_t,
}
