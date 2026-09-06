//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/vc4/vc4_hdmi.h
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


#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vc4_hdmi_phy_channel {
    PHY_LANE_0 = 0,
    PHY_LANE_1,
    PHY_LANE_2,
    PHY_LANE_CK,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vc4_hdmi_variant {
// Encoder Type for that controller
    pub encoder_type: vc4_encoder_type,
// ALSA card name
    pub card_name: *const c_char,
// Filename to expose the registers in debugfs
    pub debugfs_name: *const c_char,
// Maximum pixel clock supported by the controller (in Hz)
    pub max_pixel_clock: c_ulonglong,
// List of the registers available on that variant
    pub registers: *const vc4_hdmi_register,
// Number of registers on that variant
    pub num_registers: c_uint,
// BCM2711 Only.
// The variants don't map the lane in the same order in the
// PHY, so this is an array mapping the HDMI channel (index)
// to the PHY lane (value).
//
    pub phy_lane_mapping: [vc4_hdmi_phy_channel; 4],
// The BCM2711 cannot deal with odd horizontal pixel timings
    pub unsupported_odd_h_timings: bool,
//
// The BCM2711 CEC/hotplug IRQ controller is shared between the
// two HDMI controllers, and we have a proper irqchip driver for
// it.
//
    pub external_irq_controller: bool,
// Callback to get the resources (memory region, interrupts,
// clocks, etc) for that variant.
//
    pub vc4_hdmi): *mut vc4_hdmi,
// Callback to reset the HDMI block
    pub vc4_hdmi): *mut *mut void (reset)(struct vc4_hdmi,
// Callback to enable / disable the CSC
    pub mode): *const drm_display_mode,
// Callback to configure the video timings in the HDMI block
    pub mode): *const drm_display_mode,
// Callback to initialize the PHY according to the connector state
    pub conn_state): *mut drm_connector_state,
// Callback to disable the PHY
    pub vc4_hdmi): *mut *mut void (phy_disable)(struct vc4_hdmi,
// Callback to enable the RNG in the PHY
    pub vc4_hdmi): *mut *mut void (phy_rng_enable)(struct vc4_hdmi,
// Callback to disable the RNG in the PHY
    pub vc4_hdmi): *mut *mut void (phy_rng_disable)(struct vc4_hdmi,
// Callback to get channel map
    pub channel_mask): *mut *mut *mut u32 (channel_map)(struct vc4_hdmi vc4_hdmi, u32,
// Enables HDR metadata
    pub supports_hdr: bool,
// Callback for hardware specific hotplug detect
    pub vc4_hdmi): *mut *mut bool (hp_detect)(struct vc4_hdmi,
}

// HDMI audio information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vc4_hdmi_audio {
    pub card: snd_soc_card,
    pub link: snd_soc_dai_link,
    pub cpu: snd_soc_dai_link_component,
    pub codec: snd_soc_dai_link_component,
    pub platform: snd_soc_dai_link_component,
    pub dma_data: snd_dmaengine_dai_dma_data,
    pub streaming: bool,
}

// General HDMI hardware state.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vc4_hdmi {
    pub audio: vc4_hdmi_audio,
    pub pdev: *mut platform_device,
    pub variant: *const vc4_hdmi_variant,
    pub encoder: vc4_encoder,
    pub connector: drm_connector,
    pub scrambling_work: delayed_work,
    pub ddc: *mut i2c_adapter,
    pub hdmicore_regs: *mut void __iomem,
    pub hd_regs: *mut void __iomem,
// VC5 Only
    pub cec_regs: *mut void __iomem,
// VC5 Only
    pub csc_regs: *mut void __iomem,
// VC5 Only
    pub dvp_regs: *mut void __iomem,
// VC5 Only
    pub phy_regs: *mut void __iomem,
// VC5 Only
    pub ram_regs: *mut void __iomem,
// VC5 Only
    pub rm_regs: *mut void __iomem,
    pub hpd_gpio: *mut gpio_desc,
//
// On some systems (like the RPi4), some modes are in the same
// frequency range than the WiFi channels (1440p@60Hz for
// example). Should we take evasive actions because that system
// has a wifi adapter?
//
    pub disable_wifi_frequencies: bool,
    pub cec_rx_msg: cec_msg,
    pub cec_tx_ok: bool,
    pub cec_irq_was_rx: bool,
    pub cec_clock: *mut clk,
    pub pixel_clock: *mut clk,
    pub hsm_clock: *mut clk,
    pub audio_clock: *mut clk,
    pub pixel_bvb_clock: *mut clk,
    pub reset: *mut reset_control,
    pub hdmi_regset: debugfs_regset32,
    pub hd_regset: debugfs_regset32,
// VC5 only
    pub cec_regset: debugfs_regset32,
    pub csc_regset: debugfs_regset32,
    pub dvp_regset: debugfs_regset32,
    pub phy_regset: debugfs_regset32,
    pub ram_regset: debugfs_regset32,
    pub rm_regset: debugfs_regset32,
//
// @hw_lock: Spinlock protecting device register access.
//
    pub hw_lock: spinlock_t,
//
// @mutex: Mutex protecting the driver access across multiple
// frameworks (KMS, ALSA, CEC).
//
    pub mutex: mutex,
//
// @saved_adjusted_mode: Copy of @drm_crtc_state.adjusted_mode
// for use by ALSA hooks and interrupt handlers. Protected by @mutex.
//
    pub saved_adjusted_mode: drm_display_mode,
//
// @packet_ram_enabled: Is the HDMI controller packet RAM currently
// on? Protected by @mutex.
//
    pub packet_ram_enabled: bool,
//
// @scdc_enabled: Is the HDMI controller currently running with
// the scrambler on? Protected by @mutex.
//
    pub scdc_enabled: bool,
//
// @output_bpc: Copy of @drm_connector_state.hdmi.output_bpc for
// use outside of KMS hooks. Protected by @mutex.
//
    pub output_bpc: c_uint,
//
// @output_format: Copy of
// @drm_connector_state.hdmi.output_format for use outside of
// KMS hooks. Protected by @mutex.
//
    pub output_format: drm_output_color_format,
//
// @hdmi_jack: Represents the connection state of the HDMI plug, for
// ALSA jack detection.
//
    pub hdmi_jack: snd_soc_jack,
}

extern "C" {
    pub fn container_of_const(_arg: _encoder, vc4_hdmi: struct, _arg: encoder) -> return;
}
extern "C" {
    pub fn vc4_hdmi_phy_disable(vc4_hdmi: *mut vc4_hdmi);
}
extern "C" {
    pub fn vc4_hdmi_phy_rng_enable(vc4_hdmi: *mut vc4_hdmi);
}
extern "C" {
    pub fn vc4_hdmi_phy_rng_disable(vc4_hdmi: *mut vc4_hdmi);
}
extern "C" {
    pub fn vc5_hdmi_phy_disable(vc4_hdmi: *mut vc4_hdmi);
}
extern "C" {
    pub fn vc5_hdmi_phy_rng_enable(vc4_hdmi: *mut vc4_hdmi);
}
extern "C" {
    pub fn vc5_hdmi_phy_rng_disable(vc4_hdmi: *mut vc4_hdmi);
}
extern "C" {
    pub fn vc6_hdmi_phy_disable(vc4_hdmi: *mut vc4_hdmi);
}
