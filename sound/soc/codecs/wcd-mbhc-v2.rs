//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/wcd-mbhc-v2.h
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


// SPDX-License-Identifier: GPL-2.0

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wcd_mbhc_field_function {
    WCD_MBHC_L_DET_EN,
    WCD_MBHC_GND_DET_EN,
    WCD_MBHC_MECH_DETECTION_TYPE,
    WCD_MBHC_MIC_CLAMP_CTL,
    WCD_MBHC_ELECT_DETECTION_TYPE,
    WCD_MBHC_HS_L_DET_PULL_UP_CTRL,
    WCD_MBHC_HS_L_DET_PULL_UP_COMP_CTRL,
    WCD_MBHC_HPHL_PLUG_TYPE,
    WCD_MBHC_GND_PLUG_TYPE,
    WCD_MBHC_SW_HPH_LP_100K_TO_GND,
    WCD_MBHC_ELECT_SCHMT_ISRC,
    WCD_MBHC_FSM_EN,
    WCD_MBHC_INSREM_DBNC,
    WCD_MBHC_BTN_DBNC,
    WCD_MBHC_HS_VREF,
    WCD_MBHC_HS_COMP_RESULT,
    WCD_MBHC_IN2P_CLAMP_STATE,
    WCD_MBHC_MIC_SCHMT_RESULT,
    WCD_MBHC_HPHL_SCHMT_RESULT,
    WCD_MBHC_HPHR_SCHMT_RESULT,
    WCD_MBHC_OCP_FSM_EN,
    WCD_MBHC_BTN_RESULT,
    WCD_MBHC_BTN_ISRC_CTL,
    WCD_MBHC_ELECT_RESULT,
    WCD_MBHC_MICB_CTRL,    /* Pull-up and micb control */
    WCD_MBHC_HPH_CNP_WG_TIME,
    WCD_MBHC_HPHR_PA_EN,
    WCD_MBHC_HPHL_PA_EN,
    WCD_MBHC_HPH_PA_EN,
    WCD_MBHC_SWCH_LEVEL_REMOVE,
    WCD_MBHC_PULLDOWN_CTRL,
    WCD_MBHC_ANC_DET_EN,
    WCD_MBHC_FSM_STATUS,
    WCD_MBHC_MUX_CTL,
    WCD_MBHC_MOISTURE_STATUS,
    WCD_MBHC_HPHR_GND,
    WCD_MBHC_HPHL_GND,
    WCD_MBHC_HPHL_OCP_DET_EN,
    WCD_MBHC_HPHR_OCP_DET_EN,
    WCD_MBHC_HPHL_OCP_STATUS,
    WCD_MBHC_HPHR_OCP_STATUS,
    WCD_MBHC_ADC_EN,
    WCD_MBHC_ADC_COMPLETE,
    WCD_MBHC_ADC_TIMEOUT,
    WCD_MBHC_ADC_RESULT,
    WCD_MBHC_MICB2_VOUT,
    WCD_MBHC_ADC_MODE,
    WCD_MBHC_DETECTION_DONE,
    WCD_MBHC_ELECT_ISRC_EN,
    WCD_MBHC_REG_FUNC_MAX,
}

pub const WCD_MBHC_DEF_BUTTONS: c_int = 8;
pub const WCD_MBHC_KEYCODE_NUM: c_int = 8;
pub const WCD_MBHC_USLEEP_RANGE_MARGIN_US: c_int = 100;
pub const WCD_MBHC_THR_HS_MICB_MV: c_int = 2700;
pub const WCD_MONO_HS_MIN_THR: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wcd_mbhc_detect_logic {
    WCD_DETECTION_LEGACY,
    WCD_DETECTION_ADC,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wcd_mbhc_cs_mb_en_flag {
    WCD_MBHC_EN_CS = 0,
    WCD_MBHC_EN_MB,
    WCD_MBHC_EN_PULLUP,
    WCD_MBHC_EN_NONE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wcd_mbhc_plug_type {
    MBHC_PLUG_TYPE_INVALID = -1,
    MBHC_PLUG_TYPE_NONE,
    MBHC_PLUG_TYPE_HEADSET,
    MBHC_PLUG_TYPE_HEADPHONE,
    MBHC_PLUG_TYPE_HIGH_HPH,
    MBHC_PLUG_TYPE_GND_MIC_SWAP,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pa_dac_ack_flags {
    WCD_MBHC_HPHL_PA_OFF_ACK = 0,
    WCD_MBHC_HPHR_PA_OFF_ACK,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wcd_mbhc_btn_det_mem {
    WCD_MBHC_BTN_DET_V_BTN_LOW,
    WCD_MBHC_BTN_DET_V_BTN_HIGH
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wcd_notify_event {
    WCD_EVENT_INVALID,
// events for micbias ON and OFF
    WCD_EVENT_PRE_MICBIAS_2_OFF,
    WCD_EVENT_POST_MICBIAS_2_OFF,
    WCD_EVENT_PRE_MICBIAS_2_ON,
    WCD_EVENT_POST_MICBIAS_2_ON,
    WCD_EVENT_PRE_DAPM_MICBIAS_2_OFF,
    WCD_EVENT_POST_DAPM_MICBIAS_2_OFF,
    WCD_EVENT_PRE_DAPM_MICBIAS_2_ON,
    WCD_EVENT_POST_DAPM_MICBIAS_2_ON,
// events for PA ON and OFF
    WCD_EVENT_PRE_HPHL_PA_ON,
    WCD_EVENT_POST_HPHL_PA_OFF,
    WCD_EVENT_PRE_HPHR_PA_ON,
    WCD_EVENT_POST_HPHR_PA_OFF,
    WCD_EVENT_PRE_HPHL_PA_OFF,
    WCD_EVENT_PRE_HPHR_PA_OFF,
    WCD_EVENT_OCP_OFF,
    WCD_EVENT_OCP_ON,
    WCD_EVENT_LAST,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wcd_mbhc_event_state {
    WCD_MBHC_EVENT_PA_HPHL,
    WCD_MBHC_EVENT_PA_HPHR,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wcd_mbhc_hph_type {
    WCD_MBHC_HPH_NONE = 0,
    WCD_MBHC_HPH_MONO,
    WCD_MBHC_HPH_STEREO,
}

//
// These enum definitions are directly mapped to the register
// definitions
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mbhc_hs_pullup_iref {
    I_DEFAULT = -1,
    I_OFF = 0,
    I_1P0_UA,
    I_2P0_UA,
    I_3P0_UA,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mbhc_hs_pullup_iref_v2 {
    HS_PULLUP_I_DEFAULT = -1,
    HS_PULLUP_I_3P0_UA = 0,
    HS_PULLUP_I_2P25_UA,
    HS_PULLUP_I_1P5_UA,
    HS_PULLUP_I_0P75_UA,
    HS_PULLUP_I_1P125_UA = 0x05,
    HS_PULLUP_I_0P375_UA = 0x07,
    HS_PULLUP_I_2P0_UA,
    HS_PULLUP_I_1P0_UA = 0x0A,
    HS_PULLUP_I_0P5_UA,
    HS_PULLUP_I_0P25_UA = 0x0F,
    HS_PULLUP_I_0P125_UA = 0x17,
    HS_PULLUP_I_OFF,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mbhc_moisture_rref {
    R_OFF,
    R_24_KOHM,
    R_84_KOHM,
    R_184_KOHM,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcd_mbhc_config {
    pub btn_high: [c_int; WCD_MBHC_DEF_BUTTONS],
    pub btn_low: [c_int; WCD_MBHC_DEF_BUTTONS],
    pub v_hs_max: c_int,
    pub num_btn: c_int,
    pub mono_stero_detection: bool,
    pub typec_analog_mux: bool,
    pub component): *mut *mut bool (swap_gnd_mic)(struct snd_soc_component,
    pub hs_ext_micbias: bool,
    pub gnd_det_en: bool,
    pub linein_th: u32,
    pub moisture_en: bool,
    pub mbhc_micbias: c_int,
    pub anc_micbias: c_int,
    pub moisture_duty_cycle_en: bool,
    pub /: *mut *mut bool hphl_swh; /track HPHL switch NC / NO,
    pub /: *mut *mut bool gnd_swh; /track GND switch NC / NO,
    pub hs_thr: u32,
    pub hph_thr: u32,
    pub micb_mv: u32,
    pub moist_vref: u32,
    pub moist_iref: u32,
    pub moist_rref: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcd_mbhc_intr {
    pub mbhc_sw_intr: c_int,
    pub mbhc_btn_press_intr: c_int,
    pub mbhc_btn_release_intr: c_int,
    pub mbhc_hs_ins_intr: c_int,
    pub mbhc_hs_rem_intr: c_int,
    pub hph_left_ocp: c_int,
    pub hph_right_ocp: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcd_mbhc_field {
    pub reg: u16,
    pub mask: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcd_mbhc_cb {
    pub component): *mut *mut void (update_cross_conn_thr)(struct snd_soc_component,
    pub mb): *mut *mut *mut void (get_micbias_val)(struct snd_soc_component component, int,
    pub bcs_enable): *mut *mut *mut void (bcs_enable)(struct snd_soc_component component, bool,
    pub zr): *mut *mut uint32_t zl, uint32_t,
    pub component): *mut *mut void (set_micbias_value)(struct snd_soc_component,
    pub enable): bool,
    pub enable): *mut *mut *mut void (clk_setup)(struct snd_soc_component component, bool,
    pub micb_num): *mut *mut *mut bool (micbias_enable_status)(struct snd_soc_component component, int,
    pub enable): *mut *mut *mut void (mbhc_bias)(struct snd_soc_component component, bool,
    pub is_micbias): int num_btn, bool,
    pub mbhc_hs_pullup_iref): enum,
    pub req): int micb_num, int,
    pub enable): bool,
    pub component): *mut *mut bool (extn_use_mb)(struct snd_soc_component,
    pub req_en): int micb_num, bool,
    pub enable): bool,
    pub enable): bool,
    pub component): *mut *mut void (mbhc_moisture_config)(struct snd_soc_component,
    pub anc_num): bool enable, int,
    pub pull_up_cur): c_int,
    pub component): *mut *mut bool (mbhc_get_moisture_status)(struct snd_soc_component,
    pub enable): *mut *mut *mut void (mbhc_moisture_polling_ctrl)(struct snd_soc_component component, bool,
    pub enable): *mut *mut *mut void (mbhc_moisture_detect_en)(struct snd_soc_component component, bool,
}

extern "C" {
    pub fn wcd_dt_parse_mbhc_data(dev: *mut device, cfg: *mut wcd_mbhc_config) -> c_int;
}
extern "C" {
    pub fn wcd_mbhc_stop(mbhc: *mut wcd_mbhc);
}
extern "C" {
    pub fn wcd_mbhc_set_hph_type(mbhc: *mut wcd_mbhc, hph_type: c_int);
}
extern "C" {
    pub fn wcd_mbhc_get_hph_type(mbhc: *mut wcd_mbhc) -> c_int;
}
extern "C" {
    pub fn wcd_mbhc_typec_report_plug(mbhc: *mut wcd_mbhc) -> c_int;
}
extern "C" {
    pub fn wcd_mbhc_typec_report_unplug(mbhc: *mut wcd_mbhc) -> c_int;
}
extern "C" {
    pub fn wcd_mbhc_deinit(mbhc: *mut wcd_mbhc);
}
extern "C" {
    pub fn wcd_mbhc_event_notify(mbhc: *mut wcd_mbhc, event: c_ulong) -> c_int;
}

extern "C" {
    pub fn ERR_PTR(_arg: -ENOTSUPP) -> return;
}
// zl = 0;
// zr = 0;

