//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/drx39xyj/drxj.h
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
// Redistributions of source code must retain the above copyright notice,
// Redistributions in binary form must reproduce the above copyright notice,
// Neither the name of Trident Microsystems nor Hauppauge Computer Works
//
// -------------------------------------------------------------------------

// Check DRX-J specific dap condition
// Multi master mode and short addr format only will not work.

// ;			/* Generate a fatal compiler error to make sure it stops here,

// -------------------------------------------------------------------------
// ============================================================================
// == code support ============================================================
// ============================================================================
// == SCU cmd if  =============================================================
// ============================================================================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drxjscu_cmd {
    pub command: u16,
// < Command number
    pub parameter_len: u16,
// < Data length in byte
    pub result_len: u16,
// < result length in byte
    pub parameter: *mut u16,
// < General purpose param
    pub result: *mut u16,
// < General purpose param */};
// ============================================================================
// == CTRL CFG related data structures ========================================
// ============================================================================
// extra intermediate lock state for VSB,QAM,NTSC

// OOB lock states

// Intermediate powermodes for DRXJ

// supstition for GPIO FNC mux

// #define DRX_CTRL_BASE         (0x0000)

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drxj_cfg_type {
    DRXJ_CFG_AGC_RF = DRXJ_CTRL_CFG_BASE,
    DRXJ_CFG_AGC_IF,
    DRXJ_CFG_AGC_INTERNAL,
    DRXJ_CFG_PRE_SAW,
    DRXJ_CFG_AFE_GAIN,
    DRXJ_CFG_SYMBOL_CLK_OFFSET,
    DRXJ_CFG_ACCUM_CR_RS_CW_ERR,
    DRXJ_CFG_FEC_MERS_SEQ_COUNT,
    DRXJ_CFG_OOB_MISC,
    DRXJ_CFG_SMART_ANT,
    DRXJ_CFG_OOB_PRE_SAW,
    DRXJ_CFG_VSB_MISC,
    DRXJ_CFG_RESET_PACKET_ERR,

// ATV (FM)
    DRXJ_CFG_ATV_OUTPUT,	/* also for FM (SIF control) but not likely */
    DRXJ_CFG_ATV_MISC,
    DRXJ_CFG_ATV_EQU_COEF,
    DRXJ_CFG_ATV_AGC_STATUS,	/* also for FM ( IF,RF, audioAGC ) */

    DRXJ_CFG_MPEG_OUTPUT_MISC,
    DRXJ_CFG_HW_CFG,
    DRXJ_CFG_OOB_LO_POW,

    DRXJ_CFG_MAX	/* dummy, never to be used */};

//
// /enum drxj_cfg_smart_ant_io * smart antenna i/o.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drxj_cfg_smart_ant_io {
    DRXJ_SMT_ANT_OUTPUT = 0,
    DRXJ_SMT_ANT_INPUT
}

//
// /struct drxj_cfg_smart_ant * Set smart antenna.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drxj_cfg_smart_ant {
    pub io: drxj_cfg_smart_ant_io,
    pub ctrl_data: u16,
}

//
// /struct DRXJAGCSTATUS_t
// AGC status information from the DRXJ-IQM-AF.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drxj_agc_status {
    pub IFAGC: u16,
    pub RFAGC: u16,
    pub digital_agc: u16,
}

// DRXJ_CFG_AGC_RF, DRXJ_CFG_AGC_IF
//
// /enum drxj_agc_ctrl_mode * Available AGCs modes in the DRXJ.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drxj_agc_ctrl_mode {
    DRX_AGC_CTRL_AUTO = 0,
    DRX_AGC_CTRL_USER,
    DRX_AGC_CTRL_OFF};

//
// /struct drxj_cfg_agc * Generic interface for all AGCs present on the DRXJ.
//
    struct drxj_cfg_agc {
    enum drx_standard standard;	/* standard for which these settings apply */
    enum drxj_agc_ctrl_mode ctrl_mode;	/* off, user, auto          */
    u16 output_level;	/* range dependent on AGC   */
    u16 min_output_level;	/* range dependent on AGC   */
    u16 max_output_level;	/* range dependent on AGC   */
    u16 speed;	/* range dependent on AGC   */
    u16 top;	/* rf-agc take over point   */
    u16 cut_off_current;	/* rf-agc is accelerated if output current
    is below cut-off current                */};

// DRXJ_CFG_PRE_SAW

//
// /struct drxj_cfg_pre_saw * Interface to configure pre SAW sense.
//
    struct drxj_cfg_pre_saw {
    enum drx_standard standard;	/* standard to which these settings apply */
    u16 reference;	/* pre SAW reference value, range 0 .. 31 */
    bool use_pre_saw;	/* true algorithms must use pre SAW sense */};

// DRXJ_CFG_AFE_GAIN

//
// /struct drxj_cfg_afe_gain * Interface to configure gain of AFE (LNA + PGA).
//
    struct drxj_cfg_afe_gain {
    enum drx_standard standard;	/* standard to which these settings apply */
    u16 gain;	/* gain in 0.1 dB steps, DRXJ range 140 .. 335 */};

//
// /struct drxjrs_errors
// Available failure information in DRXJ_FEC_RS.
//
// Container for errors that are received in the most recently finished measurement period
//
    struct drxjrs_errors {
    u16 nr_bit_errors;
// < no of pre RS bit errors
    u16 nr_symbol_errors;
// < no of pre RS symbol errors
    u16 nr_packet_errors;
// < no of pre RS packet errors
    u16 nr_failures;
// < no of post RS failures to decode
    u16 nr_snc_par_fail_count;
// < no of post RS bit erros
}

//
// /struct drxj_cfg_vsb_misc * symbol error rate
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drxj_cfg_vsb_misc {
    pub symb_error: u32,
// < symbol error rate sps */};
//
// /enum drxj_mpeg_output_clock_rate * Mpeg output clock rate.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drxj_mpeg_start_width {
    DRXJ_MPEG_START_WIDTH_1CLKCYC,
    DRXJ_MPEG_START_WIDTH_8CLKCYC};

//
// /enum drxj_mpeg_output_clock_rate * Mpeg output clock rate.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drxj_mpeg_output_clock_rate {
    DRXJ_MPEGOUTPUT_CLOCK_RATE_AUTO,
    DRXJ_MPEGOUTPUT_CLOCK_RATE_75973K,
    DRXJ_MPEGOUTPUT_CLOCK_RATE_50625K,
    DRXJ_MPEGOUTPUT_CLOCK_RATE_37968K,
    DRXJ_MPEGOUTPUT_CLOCK_RATE_30375K,
    DRXJ_MPEGOUTPUT_CLOCK_RATE_25313K,
    DRXJ_MPEGOUTPUT_CLOCK_RATE_21696K};

//
// /struct DRXJCfgMisc_t
// Change TEI bit of MPEG output
// reverse MPEG output bit order
// set MPEG output clock rate
//
    struct drxj_cfg_mpeg_output_misc {
    bool disable_tei_handling;	      /*< if true pass (not change) TEI bit */
    bool bit_reverse_mpeg_outout;	      /*< if true, parallel: msb on MD0; serial: lsb out first */
    enum drxj_mpeg_output_clock_rate mpeg_output_clock_rate;
// < set MPEG output clock rate that overwirtes the derived one from symbol rate
    enum drxj_mpeg_start_width mpeg_start_width;  /*< set MPEG output start width */};

//
// /enum drxj_xtal_freq * Supported external crystal reference frequency.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drxj_xtal_freq {
    DRXJ_XTAL_FREQ_RSVD,
    DRXJ_XTAL_FREQ_27MHZ,
    DRXJ_XTAL_FREQ_20P25MHZ,
    DRXJ_XTAL_FREQ_4MHZ};

//
// /enum drxj_xtal_freq * Supported external crystal reference frequency.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drxji2c_speed {
    DRXJ_I2C_SPEED_400KBPS,
    DRXJ_I2C_SPEED_100KBPS};

//
// /struct drxj_cfg_hw_cfg * Get hw configuration, such as crystal
// reference frequency, I2C speed, etc...
//
    struct drxj_cfg_hw_cfg {
    enum drxj_xtal_freq xtal_freq;
// < crystal reference frequency
    enum drxji2c_speed i2c_speed;
// < 100 or 400 kbps */};

//
// DRXJ_CFG_ATV_MISC
//
    struct drxj_cfg_atv_misc {
    s16 peak_filter;	/* -8 .. 15 */
    u16 noise_filter;	/* 0 .. 15 */};

//
// struct drxj_cfg_oob_misc
pub const DRXJ_OOB_STATE_RESET: c_uint = 0x0;
pub const DRXJ_OOB_STATE_AGN_HUNT: c_uint = 0x1;
pub const DRXJ_OOB_STATE_DGN_HUNT: c_uint = 0x2;
pub const DRXJ_OOB_STATE_AGC_HUNT: c_uint = 0x3;
pub const DRXJ_OOB_STATE_FRQ_HUNT: c_uint = 0x4;
pub const DRXJ_OOB_STATE_PHA_HUNT: c_uint = 0x8;
pub const DRXJ_OOB_STATE_TIM_HUNT: c_uint = 0x10;
pub const DRXJ_OOB_STATE_EQU_HUNT: c_uint = 0x20;
pub const DRXJ_OOB_STATE_EQT_HUNT: c_uint = 0x30;
pub const DRXJ_OOB_STATE_SYNC: c_uint = 0x40;

    struct drxj_cfg_oob_misc {
    struct drxj_agc_status agc;
    bool eq_lock;
    bool sym_timing_lock;
    bool phase_lock;
    bool freq_lock;
    bool dig_gain_lock;
    bool ana_gain_lock;
    u8 state;
}

//
// Index of in array of coef
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drxj_cfg_oob_lo_power {
    DRXJ_OOB_LO_POW_MINUS0DB = 0,
    DRXJ_OOB_LO_POW_MINUS5DB,
    DRXJ_OOB_LO_POW_MINUS10DB,
    DRXJ_OOB_LO_POW_MINUS15DB,
    DRXJ_OOB_LO_POW_MAX};

//
// DRXJ_CFG_ATV_EQU_COEF
//
    struct drxj_cfg_atv_equ_coef {
    s16 coef0;	/* -256 .. 255 */
    s16 coef1;	/* -256 .. 255 */
    s16 coef2;	/* -256 .. 255 */
    s16 coef3;	/* -256 .. 255 */};

//
// Index of in array of coef
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drxj_coef_array_index {
    DRXJ_COEF_IDX_MN = 0,
    DRXJ_COEF_IDX_FM,
    DRXJ_COEF_IDX_L,
    DRXJ_COEF_IDX_LP,
    DRXJ_COEF_IDX_BG,
    DRXJ_COEF_IDX_DK,
    DRXJ_COEF_IDX_I,
    DRXJ_COEF_IDX_MAX};

//
// DRXJ_CFG_ATV_OUTPUT
//

//
// /enum DRXJAttenuation_t
// Attenuation setting for SIF AGC.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drxjsif_attenuation {
    DRXJ_SIF_ATTENUATION_0DB,
    DRXJ_SIF_ATTENUATION_3DB,
    DRXJ_SIF_ATTENUATION_6DB,
    DRXJ_SIF_ATTENUATION_9DB};

//
// /struct drxj_cfg_atv_output * SIF attenuation setting.
//
    struct drxj_cfg_atv_output {
    bool enable_cvbs_output;	/* true= enabled */
    bool enable_sif_output;	/* true= enabled */
    enum drxjsif_attenuation sif_attenuation;
}

//
// TODO : AFE interface not yet finished, subject to change
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drxj_cfg_atv_agc_status {
    pub /: *mut *mut u16 rf_agc_gain; / 0 .. 877 uA,
    pub /: *mut *mut u16 if_agc_gain; / 0 .. 877 uA,
    pub /: *mut *mut s16 video_agc_gain; / -75 .. 1972 in 0.1 dB steps,
    pub /: *mut *mut s16 audio_agc_gain; / -4 .. 1020 in 0.1 dB steps,
    pub /: *mut *mut u16 rf_agc_loop_gain; / 0 .. 7,
    pub /: *mut *mut u16 if_agc_loop_gain; / 0 .. 7,
    pub /}: *mut *mut u16 video_agc_loop_gain; / 0 .. 7,
// ============================================================================
// == CTRL related data structures ============================================
// ============================================================================
// NONE
// ============================================================================
// ========================================
//
// /struct struct drxj_data * DRXJ specific attributes.
//
// Global data container for DRXJ specific data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drxj_data {
// device capabilities (determined during drx_open())
    pub /: *mut *mut bool has_lna; /< true if LNA (aka PGA) present,
    pub /: *mut *mut bool has_oob; /< true if OOB supported,
    pub /: *mut *mut bool has_ntsc; /< true if NTSC supported,
    pub /: *mut *mut bool has_btsc; /< true if BTSC supported,
    pub /: *mut *mut bool has_smatx; /< true if mat_tx is available,
    pub /: *mut *mut bool has_smarx; /< true if mat_rx is available,
    pub /: *mut *mut bool has_gpio; /< true if GPIO is available,
    pub /: *mut *mut bool has_irqn; /< true if IRQN is available,
// A1/A2/A...
    pub /: *mut *mut u8 mfx; /< metal fix,
// tuner settings
    pub /: *mut *mut bool mirror_freq_spect_oob;/< tuner inversion (true = tuner mirrors the signal,
// standard/channel settings
    pub /: *mut *mut drx_standard standard; /< current standard information,
    pub constellation: drx_modulation,
// < current constellation
    pub /: *mut *mut s32 frequency; /< center signal frequency in KHz,
    pub curr_bandwidth: drx_bandwidth,
// < current channel bandwidth
    pub /: *mut *mut drx_mirror mirror; /< current channel mirror,
// signal quality information
    pub /: *mut *mut u32 fec_bits_desired; /< BER accounting period,
    pub /: *mut *mut u16 fec_vd_plen; /< no of trellis symbols: VD SER measurement period,
    pub /: *mut *mut u16 qam_vd_prescale; /< Viterbi Measurement Prescale,
    pub /: *mut *mut u16 qam_vd_period; /< Viterbi Measurement period,
    pub /: *mut *mut u16 fec_rs_plen; /< defines RS BER measurement period,
    pub /: *mut *mut u16 fec_rs_prescale; /< ReedSolomon Measurement Prescale,
    pub /: *mut *mut u16 fec_rs_period; /< ReedSolomon Measurement period,
    pub /: *mut *mut bool reset_pkt_err_acc; /< Set a flag to reset accumulated packet error,
    pub /: *mut *mut u16 pkt_err_acc_start; /< Set a flag to reset accumulated packet error,
// HI configuration
    pub /: *mut *mut u16 hi_cfg_timing_div; /< HI Configure() parameter 2,
    pub /: *mut *mut u16 hi_cfg_bridge_delay; /< HI Configure() parameter 3,
    pub /: *mut *mut u16 hi_cfg_wake_up_key; /< HI Configure() parameter 4,
    pub /: *mut *mut u16 hi_cfg_ctrl; /< HI Configure() parameter 5,
    pub /: *mut *mut u16 hi_cfg_transmit; /< HI Configure() parameter 6,
// UIO configuration
    pub /: *mut *mut drxuio_mode uio_sma_rx_mode;/< current mode of SmaRx pin,
    pub /: *mut *mut drxuio_mode uio_sma_tx_mode;/< current mode of SmaTx pin,
    pub /: *mut *mut drxuio_mode uio_gpio_mode; /< current mode of ASEL pin,
    pub /: *mut *mut drxuio_mode uio_irqn_mode; /< current mode of IRQN pin,
// IQM fs frequency shift and inversion
    pub /: *mut *mut u32 iqm_fs_rate_ofs; /< frequency shifter setting after setchannel,
    pub /: *mut *mut bool pos_image; /< True: positive image,
// IQM RC frequency shift
    pub /: *mut *mut u32 iqm_rc_rate_ofs; /< frequency shifter setting after setchannel,
// ATV configuration
    pub /: *mut *mut u32 atv_cfg_changed_flags; /< flag: flags cfg changes,
    pub /: *mut *mut s16 atv_top_equ0[DRXJ_COEF_IDX_MAX]; /< shadow of ATV_TOP_EQU0__A,
    pub /: *mut *mut s16 atv_top_equ1[DRXJ_COEF_IDX_MAX]; /< shadow of ATV_TOP_EQU1__A,
    pub /: *mut *mut s16 atv_top_equ2[DRXJ_COEF_IDX_MAX]; /< shadow of ATV_TOP_EQU2__A,
    pub /: *mut *mut s16 atv_top_equ3[DRXJ_COEF_IDX_MAX]; /< shadow of ATV_TOP_EQU3__A,
    pub /: *mut *mut bool phase_correction_bypass;/< flag: true=bypass,
    pub /: *mut *mut s16 atv_top_vid_peak; /< shadow of ATV_TOP_VID_PEAK__A,
    pub /: *mut *mut u16 atv_top_noise_th; /< shadow of ATV_TOP_NOISE_TH__A,
    pub /: *mut *mut bool enable_cvbs_output; /< flag CVBS output enable,
    pub /: *mut *mut bool enable_sif_output; /< flag SIF output enable,
    pub sif_attenuation: drxjsif_attenuation,
// < current SIF att setting
// Agc configuration for QAM and VSB
    pub /: *mut *mut drxj_cfg_agc qam_rf_agc_cfg; /< qam RF AGC config,
    pub /: *mut *mut drxj_cfg_agc qam_if_agc_cfg; /< qam IF AGC config,
    pub /: *mut *mut drxj_cfg_agc vsb_rf_agc_cfg; /< vsb RF AGC config,
    pub /: *mut *mut drxj_cfg_agc vsb_if_agc_cfg; /< vsb IF AGC config,
// PGA gain configuration for QAM and VSB
    pub /: *mut *mut u16 qam_pga_cfg; /< qam PGA config,
    pub /: *mut *mut u16 vsb_pga_cfg; /< vsb PGA config,
// Pre SAW configuration for QAM and VSB
    pub qam_pre_saw_cfg: drxj_cfg_pre_saw,
// < qam pre SAW config
    pub vsb_pre_saw_cfg: drxj_cfg_pre_saw,
// < qam pre SAW config
// Version information
    pub /: *mut *mut char v_text[2][12]; /< allocated text versions,
    pub /: *mut *mut drx_version v_version[2]; /< allocated versions structs,
    pub v_list_elements: [drx_version_list; 2],
// < allocated version list
// smart antenna configuration
    pub smart_ant_inverted: bool,
// Tracking filter setting for OOB
    pub oob_trk_filter_cfg: [u16; 8],
    pub oob_power_on: bool,
// MPEG static bitrate setting
    pub /: *mut *mut u32 mpeg_ts_static_bitrate; /< bitrate static MPEG output,
    pub /: *mut *mut bool disable_te_ihandling; /< MPEG TS TEI handling,
    pub /: *mut *mut bool bit_reverse_mpeg_outout;/< MPEG output bit order,
    pub mpeg_output_clock_rate: drxj_mpeg_output_clock_rate,
// < MPEG output clock rate
    pub mpeg_start_width: drxj_mpeg_start_width,
// < MPEG Start width
// Pre SAW & Agc configuration for ATV
    pub atv_pre_saw_cfg: drxj_cfg_pre_saw,
// < atv pre SAW config
    pub /: *mut *mut drxj_cfg_agc atv_rf_agc_cfg; /< atv RF AGC config,
    pub /: *mut *mut drxj_cfg_agc atv_if_agc_cfg; /< atv IF AGC config,
    pub /: *mut *mut u16 atv_pga_cfg; /< atv pga config,
    pub curr_symbol_rate: u32,
// pin-safe mode
    pub /: *mut *mut bool pdr_safe_mode; /< PDR safe mode activated,
    pub pdr_safe_restore_val_gpio: u16,
    pub pdr_safe_restore_val_v_sync: u16,
    pub pdr_safe_restore_val_sma_rx: u16,
    pub pdr_safe_restore_val_sma_tx: u16,
// OOB pre-saw value
    pub oob_pre_saw: u16,
    pub oob_lo_pow: drxj_cfg_oob_lo_power,
    pub aud_data: drx_aud_data,
// < audio storage                  */};
// -------------------------------------------------------------------------
//
// \brief Compilable references to attributes
// \param d pointer to demod instance
//
// Used as main reference to an attribute field.
// Can be used by both macro implementation and function implementation.
// These macros are defined to avoid duplication of code in macro and function
// definitions that handle access of demod common or extended attributes.
//

// -------------------------------------------------------------------------
//
// \def DRXJ_NTSC_CARRIER_FREQ_OFFSET
// \brief Offset from picture carrier to centre frequency in kHz, in RF domain
//
// For NTSC standard.
// NTSC channels are listed by their picture carrier frequency (Fpc).
// The function DRX_CTRL_SET_CHANNEL requires the centre frequency as input.
// In case the tuner module is not used the DRX-J requires that the tuner is
// tuned to the centre frequency of the channel:
//
// Fcentre = Fpc + DRXJ_NTSC_CARRIER_FREQ_OFFSET
//

//
// \def DRXJ_PAL_SECAM_BG_CARRIER_FREQ_OFFSET
// \brief Offset from picture carrier to centre frequency in kHz, in RF domain
//
// For PAL/SECAM - BG standard. This define is needed in case the tuner module
// is NOT used. PAL/SECAM channels are listed by their picture carrier frequency (Fpc).
// The DRX-J requires that the tuner is tuned to:
// Fpc + DRXJ_PAL_SECAM_BG_CARRIER_FREQ_OFFSET
//
// In case the tuner module is used the drxdriver takes care of this.
// In case the tuner module is NOT used the application programmer must take
// care of this.
//

//
// \def DRXJ_PAL_SECAM_DKIL_CARRIER_FREQ_OFFSET
// \brief Offset from picture carrier to centre frequency in kHz, in RF domain
//
// For PAL/SECAM - DK, I, L standards. This define is needed in case the tuner module
// is NOT used. PAL/SECAM channels are listed by their picture carrier frequency (Fpc).
// The DRX-J requires that the tuner is tuned to:
// Fpc + DRXJ_PAL_SECAM_DKIL_CARRIER_FREQ_OFFSET
//
// In case the tuner module is used the drxdriver takes care of this.
// In case the tuner module is NOT used the application programmer must take
// care of this.
//

//
// \def DRXJ_PAL_SECAM_LP_CARRIER_FREQ_OFFSET
// \brief Offset from picture carrier to centre frequency in kHz, in RF domain
//
// For PAL/SECAM - LP standard. This define is needed in case the tuner module
// is NOT used. PAL/SECAM channels are listed by their picture carrier frequency (Fpc).
// The DRX-J requires that the tuner is tuned to:
// Fpc + DRXJ_PAL_SECAM_LP_CARRIER_FREQ_OFFSET
//
// In case the tuner module is used the drxdriver takes care of this.
// In case the tuner module is NOT used the application programmer must take
// care of this.
//

//
// \def DRXJ_FM_CARRIER_FREQ_OFFSET
// \brief Offset from sound carrier to centre frequency in kHz, in RF domain
//
// For FM standard.
// FM channels are listed by their sound carrier frequency (Fsc).
// The function DRX_CTRL_SET_CHANNEL requires the Ffm frequency (see below) as
// input.
// In case the tuner module is not used the DRX-J requires that the tuner is
// tuned to the Ffm frequency of the channel.
//
// Ffm = Fsc + DRXJ_FM_CARRIER_FREQ_OFFSET
//

// Revision types -------------------------------------------------------

// Macros ---------------------------------------------------------------
// Convert OOB lock status to string

