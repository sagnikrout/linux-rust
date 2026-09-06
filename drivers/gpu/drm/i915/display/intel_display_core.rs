//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_display_core.h
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
// Copyright © 2022 Intel Corporation
//

// Amount of SAGV/QGV points, BSpec precisely defines this
pub const I915_NUM_QGV_POINTS: c_int = 8;
// Amount of PSF GV points, BSpec precisely defines this
pub const I915_NUM_PSF_GV_POINTS: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_modeset_funcs {
//
// Returns the active state of the crtc, and if the crtc is active,
// fills out the pipe-config with the hw state.
//
    pub ): *mut intel_crtc_state,
    pub ): *mut intel_initial_plane_config,
    pub plane_config): *const intel_initial_plane_config,
    pub crtc): *mut intel_crtc,
    pub crtc): *mut intel_crtc,
    pub state): *mut *mut void (commit_modeset_enables)(struct intel_atomic_state,
}

// functions used for watermark calcs for display.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_wm_funcs {
// update_wm is for legacy wm management
    pub display): *mut *mut void (update_wm)(struct intel_display,
    pub crtc): *mut intel_crtc,
    pub crtc): *mut intel_crtc,
    pub crtc): *mut intel_crtc,
    pub crtc): *mut intel_crtc,
    pub state): *mut *mut int (compute_global_watermarks)(struct intel_atomic_state,
    pub display): *mut *mut void (get_hw_state)(struct intel_display,
    pub display): *mut *mut void (sanitize)(struct intel_display,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_audio_state {
    pub encoder: *mut intel_encoder,
    pub eld: [u8; MAX_ELD_BYTES],
// MST, or SST on UHBR link
    pub needs_cpu_transcoder_id: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_audio {
// internal display audio functions
    pub funcs: *const intel_audio_funcs,
// hda/i915 audio component
    pub component: *mut i915_audio_component,
    pub component_registered: bool,
// mutex for audio/video sync
    pub mutex: mutex,
    pub power_refcount: c_int,
    pub freq_cntrl: u32,
// current audio state for the audio component hooks
    pub state: [intel_audio_state; I915_MAX_TRANSCODERS],
// necessary resource sharing with HDMI LPE audio driver.
    pub platdev: *mut platform_device,
    pub irq: c_int,
    pub lpe: },
}

//
// dpll and cdclk state is protected by connection_mutex dpll.lock serializes
// intel_{prepare,enable,disable}_shared_dpll.  Must be global rather than per
// dpll, because on some platforms plls share registers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_dpll_global {
// internal dpll functions
    pub funcs: *const intel_dpll_global_funcs,
    pub lock: mutex,
    pub num_dpll: c_int,
    pub dplls: [intel_dpll; I915_NUM_PLLS],
    pub mgr: *const intel_dpll_mgr,
    pub nssc: c_int,
    pub ssc: c_int,
    pub ref_clks: },
//
// Bitmask of PLLs using the PCH SSC, indexed using enum intel_dpll_id.
//
    pub pch_ssc_use: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_frontbuffer_tracking {
// protects busy_bits
    pub lock: spinlock_t,
//
// Tracking bits for delayed frontbuffer flushing due to gpu activity.
//
    pub busy_bits: unsigned,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_hotplug {
// internal hotplug irq functions
    pub funcs: *const intel_hotplug_irq_funcs,
    pub hotplug_work: delayed_work,
    pub pch_hpd: *const *const u32 hpd,,
    pub last_jiffies: c_ulong,
    pub count: c_int,
    pub blocked_count: c_int,
    pub state: },
    pub stats: [}; HPD_NUM_PINS],
    pub event_bits: u32,
    pub retry_bits: u32,
    pub reenable_work: delayed_work,
    pub long_hpd_pin_mask: u32,
    pub short_hpd_pin_mask: u32,
    pub dig_port_work: work_struct,
    pub poll_init_work: work_struct,
    pub poll_enabled: bool,
//
// Queuing of hotplug_work, reenable_work and poll_init_work is
// enabled. Protected by intel_display::irq::lock.
//
    pub detection_work_enabled: bool,
    pub hpd_storm_threshold: c_uint,
// Whether or not to count short HPD IRQs in HPD storms
    pub hpd_short_storm_enabled: u8,
// Last state reported by oob_hotplug_event for each encoder
    pub oob_hotplug_last_state: c_ulong,
//
// if we get a HPD irq from DP and a HPD irq from non-DP
// the non-DP HPD could block the workqueue on a mode config
// mutex getting, that userspace may have taken. However
// userspace is waiting on the DP workqueue to run which is
// blocked behind the non-DP one.
//
    pub dp_wq: *mut workqueue_struct,
//
// Flag to track if long HPDs need not to be processed
//
// Some panels generate long HPDs while keep connected to the port.
// This can cause issues with CI tests results. In CI systems we
// don't expect to disconnect the panels and could ignore the long
// HPDs generated from the faulty panels. This flag can be used as
// cue to ignore the long HPDs and can be set / unset using debugfs.
//
    pub ignore_long_hpd: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_vbt_data {
// bdb version
    pub version: u16,
// Feature bits
    pub int_tv_support:1: c_uint,
    pub int_crt_support:1: c_uint,
    pub lvds_use_ssc:1: c_uint,
    pub int_lvds_support:1: c_uint,
    pub display_clock_mode:1: c_uint,
    pub fdi_rx_polarity_inverted:1: c_uint,
    pub lvds_ssc_freq: c_int,
    pub orientation: drm_panel_orientation,
    pub override_afc_startup: bool,
    pub override_afc_startup_val: u8,
    pub crt_ddc_pin: c_int,
    pub display_devices: list_head,
    pub bdb_blocks: list_head,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdvo_device_mapping {
    pub initialized: u8,
    pub dvo_port: u8,
    pub target_addr: u8,
    pub dvo_wiring: u8,
    pub i2c_pin: u8,
    pub ddc_pin: u8,
    pub sdvo_mappings: [}; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_wm {
// internal watermark functions
    pub funcs: *const intel_wm_funcs,
//
// Raw watermark latency values:
// in 0.1us units for WM0,
// in 0.5us units for WM1+.
//
// primary
    pub pri_latency: [u16; 5],
// sprite
    pub spr_latency: [u16; 5],
// cursor
    pub cur_latency: [u16; 5],
//
// Raw watermark memory latency values
// for SKL for all 8 levels
// in 1us units.
//
    pub skl_latency: [u16; 8],
// current hardware state
    pub hw: ilk_wm_values,
    pub vlv: vlv_wm_values,
    pub g4x: g4x_wm_values,
}

//
// Should be held around atomic WM register writing; also
// protects * intel_crtc->wm.active and
// crtc_state->wm.need_postvbl_update.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_display {
// drm device backpointer
    pub drm: *mut drm_device,
// Platform (and subplatform, if any) identification
    pub platform: intel_display_platforms,
// Intel PCH: where the south display engine lives
    pub pch_type: intel_pch,
// Parent, or core, driver functions exposed to display
    pub parent: *const intel_display_parent_interface,
// list of all intel_crtcs sorted by pipe
    pub pipe_list: list_head,
    pub any_task_allowed: bool,
    pub allowed_task: *mut task_struct,
    pub access: },
// backlight registers and fields in struct intel_panel
    pub lock: mutex,
    pub backlight: },
    pub obj: intel_global_obj,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_bw_info {
// for each QGV point
    pub deratedbw: [c_uint; I915_NUM_QGV_POINTS],
    pub num_planes: u8,
    pub max: [}; 6],
// for each PSF GV point
    pub psf_bw: [c_uint; I915_NUM_PSF_GV_POINTS],
// Peak BW for each QGV point
    pub peakbw: [c_uint; I915_NUM_QGV_POINTS],
    pub num_qgv_points: u8,
    pub num_psf_gv_points: u8,
    pub bw: },
// Internal CDCLK functions
    pub funcs: *const intel_cdclk_funcs,
// The current hardware cdclk configuration
    pub hw: intel_cdclk_config,
// cdclk, divider, and ratio table from bspec
    pub table: *const intel_cdclk_vals,
    pub obj: intel_global_obj,
    pub max_cdclk_freq: c_uint,
    pub max_dotclk_freq: c_uint,
    pub skl_preferred_vco_freq: c_uint,
    pub cdclk: },
// internal color functions
    pub funcs: *const intel_color_funcs,
    pub glk_linear_degamma_lut: *mut drm_property_blob,
    pub color: },
// The current hardware dbuf configuration
    pub enabled_slices: u8,
    pub obj: intel_global_obj,
    pub dbuf: },
    pub obj: intel_global_obj,
    pub dbuf_bw: },
//
// dkl.phy_lock protects against concurrent access of the
// Dekel TypeC PHYs.
//
    pub phy_lock: spinlock_t,
    pub dkl: },
    pub dmc: *mut intel_dmc,
    pub wakeref: *mut ref_tracker,
    pub dmc: },
// VLV/CHV/BXT/GLK DSI MMIO register base address
    pub mmio_base: u32,
    pub dsi: },
    pub info: *const dram_info,
    pub dram: },
    pub instances: [*mut intel_fbc; I915_MAX_FBCS],
// xe3p_lpd+: FBC instance utilizing the system cache
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sys_cache_cfg {
// Protect concurrecnt access to system cache configuration
    pub lock: mutex,
    pub id: intel_fbc_id,
    pub sys_cache: },
    pub fbc: },
// list of fbdev register on this device
    pub fbdev: *mut intel_fbdev,
    pub fbdev: },
// internal fdi functions
    pub funcs: *const intel_fdi_funcs,
    pub pll_freq: c_uint,
    pub rx_config: u32,
    pub fdi: },
    pub obj_list: list_head,
    pub global: },
//
// Base address of where the gmbus and gpio blocks are located
// (either on PCH or on SoC for platforms without PCH).
//
    pub mmio_base: u32,
//
// gmbus.mutex protects against concurrent usage of the single
// hw gmbus controller on different i2c buses.
//
    pub mutex: mutex,
    pub bus: [*mut intel_gmbus; GMBUS_NUM_PINS],
    pub wait_queue: wait_queue_head_t,
    pub gmbus: },
    pub arbiter: *mut i915_hdcp_arbiter,
    pub comp_added: bool,
//
// HDCP message struct for allocation of memory which can be
// reused when sending message to gsc cs.
// this is only populated post Meteorlake
//
    pub gsc_context: *mut intel_hdcp_gsc_context,
// Mutex to protect the above hdcp related values.
    pub hdcp_mutex: mutex,
    pub hdcp: },
//
// HTI (aka HDPORT) state read during initial hw readout. Most
// platforms don't have HTI, so this will just stay 0. Those
// that do will use this later to figure out which PLLs and PHYs
// are unavailable for driver usage.
//
    pub state: u32,
    pub hti: },
// Access with DISPLAY_INFO()
    pub __device_info: *const intel_display_device_info,
// Access with DISPLAY_RUNTIME_INFO()
    pub __runtime_info: intel_display_runtime_info,
    pub info: },
    pub false_color: bool,
    pub ips: },
// internal display irq functions
    pub funcs: *const intel_display_irq_funcs,
// protects the irq masks
    pub lock: spinlock_t,
//
// Most platforms treat the display irq block as an always-on
// power domain. vlv/chv can disable it at runtime and need
// special care to avoid writing any of the display block
// registers outside of the power domain. We defer setting up
// the display irqs in this case to the runtime pm.
//
    pub vlv_display_irqs_enabled: bool,
// For i915gm/i945gm vblank irq workaround
    pub vblank_enabled: u8,
    pub vblank_enable_count: c_int,
    pub vblank_status_last_notified: bool,
    pub vblank_notify_work: work_struct,
//
// Cached value of VLV/CHV IMR to avoid reads in updating the
// bitfield.
//
    pub vlv_imr_mask: u32,
//
// Cached value of gen 5-7 DE IMR to avoid reads in updating the
// bitfield.
//
    pub ilk_de_imr_mask: u32,
//
// Cached value of BDW+ DE pipe IMR to avoid reads in updating
// the bitfield.
//
    pub de_pipe_imr_mask: [u32; I915_MAX_PIPES],
    pub pipestat_irq_mask: [u32; I915_MAX_PIPES],
    pub irq: },
// Top level crtc-ish functions
    pub funcs: *const intel_modeset_funcs,
    pub modeset: },
// protected by wm.wm_mutex
    pub linetime: [u16; I915_MAX_PIPES],
    pub disable: [bool; I915_MAX_PIPES],
    pub pkgc: },
    pub waitqueue: wait_queue_head_t,
// mutex to protect pmdemand programming sequence
    pub lock: mutex,
    pub obj: intel_global_obj,
    pub pmdemand: },
    pub domains: i915_power_domains,
// DC3CO state
    pub dc3co: intel_dc3co_state,
// Shadow for DISPLAY_PHY_CONTROL which can't be safely read
    pub chv_phy_control: u32,
// perform PHY state sanity checks?
    pub chv_phy_assert: [bool; 2],
    pub power: },
    pub mmio_base: u32,
// protects panel power sequencer state
    pub mutex: mutex,
    pub pps: },
    pub broadcast_rgb: *mut drm_property,
    pub force_audio: *mut drm_property,
    pub properties: },
    pub mask: c_ulong,
    pub quirks: },
    pub count: u32,
    pub reset: },
// restore state for suspend/resume and display reset
    pub modeset_state: *mut drm_atomic_commit,
    pub reset_ctx: drm_modeset_acquire_ctx,
// modeset stuck tracking for reset
    pub saveDSPARB: u32,
    pub saveSWF0: [u32; 16],
    pub saveSWF1: [u32; 16],
    pub saveSWF3: [u32; 3],
    pub saveGCDGMBUS: u16,
    pub restore: },
    pub status: },
    pub block_time_us: u32,
    pub sagv: },
// LPT/WPT IOSF sideband protection
    pub lock: mutex,
    pub sbi: },
//
// DG2: Mask of PHYs that were not calibrated by the firmware
// and should not be used.
//
    pub phy_failed_calibration: u8,
    pub snps: },
//
// Shadows for CHV DPLL_MD regs to keep the state
// checker somewhat working in the presence hardware
// crappiness (can't read out DPLL_MD for pipes B & C).
//
    pub chv_dpll_md: [u32; I915_MAX_PIPES],
    pub bxt_phy_grc: u32,
    pub state: },
    pub hpll_freq: c_uint,
    pub czclk_freq: c_uint,
    pub vlv_clock: },
// ordered wq for modesets
    pub modeset: *mut workqueue_struct,
// unbound hipri wq for page flips/plane updates
    pub flip: *mut workqueue_struct,
// hipri wq for commit cleanups
    pub cleanup: *mut workqueue_struct,
// unordered workqueue for all display unordered work
    pub unordered: *mut workqueue_struct,
    pub wq: },
// Grouping using named structs. Keep sorted.
    pub dp_tunnel_mgr: *mut drm_dp_tunnel_mgr,
    pub audio: intel_audio,
    pub dpll: intel_dpll_global,
    pub fb_tracking: intel_frontbuffer_tracking,
    pub hotplug: intel_hotplug,
    pub opregion: *mut intel_opregion,
    pub overlay: *mut intel_overlay,
    pub params: intel_display_params,
    pub vbt: intel_vbt_data,
    pub wl: intel_dmc_wl,
    pub wm: intel_wm,
    pub psr_dc5_dc6_wa_work: work_struct,
}
