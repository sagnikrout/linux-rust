//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/drxk_hard.h
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

pub const DRXK_VERSION_MAJOR: c_int = 0;
pub const DRXK_VERSION_MINOR: c_int = 9;
pub const DRXK_VERSION_PATCH: c_int = 4300;
pub const HI_I2C_DELAY: c_int = 42;
pub const HI_I2C_BRIDGE_DELAY: c_int = 350;
pub const DRXK_MAX_RETRIES: c_int = 100;
pub const DRIVER_4400: c_int = 1;
pub const DRXX_JTAGID: c_uint = 0x039210D9;
pub const DRXX_J_JTAGID: c_uint = 0x239310D9;
pub const DRXX_K_JTAGID: c_uint = 0x039210D9;
pub const DRX_UNKNOWN: c_int = 254;
pub const DRX_AUTO: c_int = 255;
pub const DRX_SCU_READY: c_int = 0;

pub const SCU_RESULT_OK: c_int = 0;

pub const IQM_CF_OUT_ENA_OFDM__M: c_uint = 0x4;
pub const IQM_FS_ADJ_SEL_B_QAM: c_uint = 0x1;
pub const IQM_FS_ADJ_SEL_B_OFF: c_uint = 0x0;
pub const IQM_FS_ADJ_SEL_B_VSB: c_uint = 0x2;
pub const IQM_RC_ADJ_SEL_B_OFF: c_uint = 0x0;
pub const IQM_RC_ADJ_SEL_B_QAM: c_uint = 0x1;
pub const IQM_RC_ADJ_SEL_B_VSB: c_uint = 0x2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum operation_mode {
    OM_NONE,
    OM_QAM_ITU_A,
    OM_QAM_ITU_B,
    OM_QAM_ITU_C,
    OM_DVBT
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drx_power_mode {
    DRX_POWER_UP = 0,
    DRX_POWER_MODE_1,
    DRX_POWER_MODE_2,
    DRX_POWER_MODE_3,
    DRX_POWER_MODE_4,
    DRX_POWER_MODE_5,
    DRX_POWER_MODE_6,
    DRX_POWER_MODE_7,
    DRX_POWER_MODE_8,

    DRX_POWER_MODE_9,
    DRX_POWER_MODE_10,
    DRX_POWER_MODE_11,
    DRX_POWER_MODE_12,
    DRX_POWER_MODE_13,
    DRX_POWER_MODE_14,
    DRX_POWER_MODE_15,
    DRX_POWER_MODE_16,
    DRX_POWER_DOWN = 255
}

// Intermediate power mode for DRXK, power down OFDM clock domain

// Intermediate power mode for DRXK, power down core (sysclk)

// Intermediate power mode for DRXK, power down pll (only osc runs)

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum agc_ctrl_mode {
    DRXK_AGC_CTRL_AUTO = 0,
    DRXK_AGC_CTRL_USER,
    DRXK_AGC_CTRL_OFF
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum e_drxk_state {
    DRXK_UNINITIALIZED = 0,
    DRXK_STOPPED,
    DRXK_DTV_STARTED,
    DRXK_ATV_STARTED,
    DRXK_POWERED_DOWN,
    DRXK_NO_DEV			/* If drxk init failed */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum e_drxk_coef_array_index {
    DRXK_COEF_IDX_MN = 0,
    DRXK_COEF_IDX_FM    ,
    DRXK_COEF_IDX_L     ,
    DRXK_COEF_IDX_LP    ,
    DRXK_COEF_IDX_BG    ,
    DRXK_COEF_IDX_DK    ,
    DRXK_COEF_IDX_I     ,
    DRXK_COEF_IDX_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum e_drxk_sif_attenuation {
    DRXK_SIF_ATTENUATION_0DB,
    DRXK_SIF_ATTENUATION_3DB,
    DRXK_SIF_ATTENUATION_6DB,
    DRXK_SIF_ATTENUATION_9DB
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum e_drxk_constellation {
    DRX_CONSTELLATION_BPSK = 0,
    DRX_CONSTELLATION_QPSK,
    DRX_CONSTELLATION_PSK8,
    DRX_CONSTELLATION_QAM16,
    DRX_CONSTELLATION_QAM32,
    DRX_CONSTELLATION_QAM64,
    DRX_CONSTELLATION_QAM128,
    DRX_CONSTELLATION_QAM256,
    DRX_CONSTELLATION_QAM512,
    DRX_CONSTELLATION_QAM1024,
    DRX_CONSTELLATION_UNKNOWN = DRX_UNKNOWN,
    DRX_CONSTELLATION_AUTO    = DRX_AUTO
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum e_drxk_interleave_mode {
    DRXK_QAM_I12_J17    = 16,
    DRXK_QAM_I_UNKNOWN  = DRX_UNKNOWN
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drxk_cfg_dvbt_sqi_speed {
    DRXK_DVBT_SQI_SPEED_FAST = 0,
    DRXK_DVBT_SQI_SPEED_MEDIUM,
    DRXK_DVBT_SQI_SPEED_SLOW,
    DRXK_DVBT_SQI_SPEED_UNKNOWN = DRX_UNKNOWN
    } ;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drx_fftmode_t {
    DRX_FFTMODE_2K = 0,
    DRX_FFTMODE_4K,
    DRX_FFTMODE_8K,
    DRX_FFTMODE_UNKNOWN = DRX_UNKNOWN,
    DRX_FFTMODE_AUTO    = DRX_AUTO
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drxmpeg_str_width_t {
    DRX_MPEG_STR_WIDTH_1,
    DRX_MPEG_STR_WIDTH_8
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drx_qam_lock_range_t {
    DRX_QAM_LOCKRANGE_NORMAL,
    DRX_QAM_LOCKRANGE_EXTENDED
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drxk_cfg_dvbt_echo_thres_t {
    pub threshold: u16,
    pub fft_mode: drx_fftmode_t,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct s_cfg_agc {
    pub /: *mut *mut agc_ctrl_mode ctrl_mode; / off, user, auto,
    pub /: *mut *mut u16 output_level; / range dependent on AGC,
    pub /: *mut *mut u16 min_output_level; / range dependent on AGC,
    pub /: *mut *mut u16 max_output_level; / range dependent on AGC,
    pub /: *mut *mut u16 speed; / range dependent on AGC,
    pub /: *mut *mut u16 top; / rf-agc take over point,
    pub current: *mut *mut u16 cut_off_current; / rf-agc is accelerated if output,
    pub ingain_tgt_max: u16,
    pub fast_clip_ctrl_delay: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct s_cfg_pre_saw {
    pub /: *mut *mut u16 reference; / pre SAW reference value, range 0 .. 31,
    pub /: *mut *mut bool use_pre_saw; / TRUE algorithms must use pre SAW sense,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drxk_ofdm_sc_cmd_t {
    pub /: *mut *mut u16 cmd; / Command number,
    pub parameter*/: *mut *mut u16 subcmd; / Sub-command,
    pub /: *mut *mut u16 param0; / General purpous param,
    pub /: *mut *mut u16 param1; / General purpous param,
    pub /: *mut *mut u16 param2; / General purpous param,
    pub /: *mut *mut u16 param3; / General purpous param,
    pub /: *mut *mut u16 param4; / General purpous param,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drxk_state {
    pub frontend: dvb_frontend,
    pub props: dtv_frontend_properties,
    pub dev: *mut device,
    pub i2c: *mut i2c_adapter,
    pub demod_address: u8,
    pub priv: *mut c_void,
    pub mutex: mutex,
    pub /: *mut *mut u32 m_instance; / Channel 1,2,3 or 4,
    pub m_chunk_size: c_int,
    pub chunk: [u8; 256],
    pub m_has_lna: bool,
    pub m_has_dvbt: bool,
    pub m_has_dvbc: bool,
    pub m_has_audio: bool,
    pub m_has_atv: bool,
    pub m_has_oob: bool,
    pub /: *mut *mut bool m_has_sawsw; / TRUE if mat_tx is available,
    pub /: *mut *mut bool m_has_gpio1; / TRUE if mat_rx is available,
    pub /: *mut *mut bool m_has_gpio2; / TRUE if GPIO is available,
    pub /: *mut *mut bool m_has_irqn; / TRUE if IRQN is available,
    pub m_osc_clock_freq: u16,
    pub m_hi_cfg_timing_div: u16,
    pub m_hi_cfg_bridge_delay: u16,
    pub m_hi_cfg_wake_up_key: u16,
    pub m_hi_cfg_timeout: u16,
    pub m_hi_cfg_ctrl: u16,
    pub /: *mut *mut s32 m_sys_clock_freq; / system clock frequency in kHz,
    pub /: *mut *mut e_drxk_state m_drxk_state; / State of Drxk (init,stopped,started),
    pub /: *mut *mut operation_mode m_operation_mode; / digital standards,
    pub /: *mut *mut s_cfg_agc m_vsb_rf_agc_cfg; / settings for VSB RF-AGC,
    pub /: *mut *mut s_cfg_agc m_vsb_if_agc_cfg; / settings for VSB IF-AGC,
    pub /: *mut *mut u16 m_vsb_pga_cfg; / settings for VSB PGA,
    pub /: *mut *mut s_cfg_pre_saw m_vsb_pre_saw_cfg; / settings for pre SAW sense,
    pub /: *mut *mut *mut s32 m_Quality83percent; / MER level (0.1 dB) for 83% quality indication,
    pub /: *mut *mut *mut s32 m_Quality93percent; / MER level (0.1 dB) for 93% quality indication,
    pub m_smart_ant_inverted: bool,
    pub m_b_debug_enable_bridge: bool,
    pub /: *mut *mut bool m_b_p_down_open_bridge; / only open DRXK bridge before power-down once it has been accessed,
    pub /: *mut *mut bool m_b_power_down; / Power down when not used,
    pub /: *mut *mut u32 m_iqm_fs_rate_ofs; / frequency shift as written to DRXK register (28bit fixpoint),
    pub /: *mut *mut bool m_enable_mpeg_output; / If TRUE, enable MPEG output,
    pub /: *mut *mut bool m_insert_rs_byte; / If TRUE, insert RS byte,
    pub /: *mut *mut bool m_enable_parallel; / If TRUE, parallel out otherwise serial,
    pub /: *mut *mut bool m_invert_data; / If TRUE, invert DATA signals,
    pub /: *mut *mut bool m_invert_err; / If TRUE, invert ERR signal,
    pub /: *mut *mut bool m_invert_str; / If TRUE, invert STR signals,
    pub /: *mut *mut bool m_invert_val; / If TRUE, invert VAL signals,
    pub /: *mut *mut bool m_invert_clk; / If TRUE, invert CLK signals,
    pub m_dvbc_static_clk: bool,
    pub will: *mut *mut bool m_dvbt_static_clk; / If TRUE, static MPEG clockrate,
    pub m_dvbt_bitrate: u32,
    pub m_dvbc_bitrate: u32,
    pub m_ts_data_strength: u8,
    pub m_ts_clockk_strength: u8,
    pub /: *mut *mut bool m_itut_annex_c; / If true, uses ITU-T DVB-C Annex C, instead of Annex A,
    pub /: *mut *mut drxmpeg_str_width_t m_width_str; / MPEG start width,
    pub case: *mut *mut u32 m_mpeg_ts_static_bitrate; / Maximum bitrate in b/s in,
// LARGE_INTEGER   m_startTime; */     /* Contains the time of the last demod start
    pub /: *mut *mut s32 m_mpeg_lock_time_out; / WaitForLockStatus Timeout (counts from start time),
    pub /: *mut *mut s32 m_demod_lock_time_out; / WaitForLockStatus Timeout (counts from start time),
    pub m_disable_te_ihandling: bool,
    pub m_rf_agc_pol: bool,
    pub m_if_agc_pol: bool,
    pub /: *mut *mut s_cfg_agc m_atv_rf_agc_cfg; / settings for ATV RF-AGC,
    pub /: *mut *mut s_cfg_agc m_atv_if_agc_cfg; / settings for ATV IF-AGC,
    pub /: *mut *mut s_cfg_pre_saw m_atv_pre_saw_cfg; / settings for ATV pre SAW sense,
    pub m_phase_correction_bypass: bool,
    pub m_atv_top_vid_peak: i16,
    pub m_atv_top_noise_th: u16,
    pub m_sif_attenuation: e_drxk_sif_attenuation,
    pub m_enable_cvbs_output: bool,
    pub m_enable_sif_output: bool,
    pub m_b_mirror_freq_spect: bool,
    pub /: *mut *mut e_drxk_constellation m_constellation; / constellation type of the channel,
    pub /: *mut *mut u32 m_curr_symbol_rate; / Current QAM symbol rate,
    pub /: *mut *mut s_cfg_agc m_qam_rf_agc_cfg; / settings for QAM RF-AGC,
    pub /: *mut *mut s_cfg_agc m_qam_if_agc_cfg; / settings for QAM IF-AGC,
    pub /: *mut *mut u16 m_qam_pga_cfg; / settings for QAM PGA,
    pub /: *mut *mut s_cfg_pre_saw m_qam_pre_saw_cfg; / settings for QAM pre SAW sense,
    pub /: *mut *mut e_drxk_interleave_mode m_qam_interleave_mode; / QAM Interleave mode,
    pub m_fec_rs_plen: u16,
    pub m_fec_rs_prescale: u16,
    pub m_sqi_speed: drxk_cfg_dvbt_sqi_speed,
    pub m_gpio: u16,
    pub m_gpio_cfg: u16,
    pub /: *mut *mut s_cfg_agc m_dvbt_rf_agc_cfg; / settings for QAM RF-AGC,
    pub /: *mut *mut s_cfg_agc m_dvbt_if_agc_cfg; / settings for QAM IF-AGC,
    pub /: *mut *mut s_cfg_pre_saw m_dvbt_pre_saw_cfg; / settings for QAM pre SAW sense,
    pub m_agcfast_clip_ctrl_delay: u16,
    pub m_adc_comp_passed: bool,
    pub m_adcCompCoef: [u16; 64],
    pub m_adc_state: u16,
    pub m_microcode: *mut u8,
    pub m_microcode_length: c_int,
    pub m_drxk_a3_rom_code: bool,
    pub m_drxk_a3_patch_code: bool,
    pub m_rfmirror: bool,
    pub m_device_spin: u8,
    pub m_iqm_rc_rate: u32,
    pub m_current_power_mode: drx_power_mode,
// when true, avoids other devices to use the I2C bus
    pub drxk_i2c_exclusive_lock: bool,
//
// Configurable parameters at the driver. They stores the values found
// at struct drxk_config.
//
    pub /: *mut *mut u16 uio_mask; / Bits used by UIO,
    pub enable_merr_cfg: bool,
    pub single_master: bool,
    pub no_i2c_bridge: bool,
    pub antenna_dvbt: bool,
    pub antenna_gpio: u16,
    pub fe_status: fe_status,
// Firmware
    pub microcode_name: *const c_char,
    pub fw_wait_load: completion,
    pub fw: *const firmware,
    pub qam_demod_parameter_count: c_int,
}

pub const NEVER_LOCK: c_int = 0;
pub const NOT_LOCKED: c_int = 1;
pub const DEMOD_LOCK: c_int = 2;
pub const FEC_LOCK: c_int = 3;
pub const MPEG_LOCK: c_int = 4;
