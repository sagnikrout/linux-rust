//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/cx2072x.h
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
//
// ALSA SoC CX20721/CX20723 codec driver
//
// Copyright:	(C) 2017 Conexant Systems, Inc.
// Author:	Simon Ho, <Simon.ho@conexant.com>
//
pub const CX2072X_MCLK_PLL: c_int = 1;
pub const CX2072X_MCLK_EXTERNAL_PLL: c_int = 1;
pub const CX2072X_MCLK_INTERNAL_OSC: c_int = 2;
// #define CX2072X_RATES		SNDRV_PCM_RATE_8000_192000

pub const CX2072X_REG_MAX: c_uint = 0x8a3c;
pub const CX2072X_VENDOR_ID: c_uint = 0x0200;
pub const CX2072X_REVISION_ID: c_uint = 0x0208;
pub const CX2072X_CURRENT_BCLK_FREQUENCY: c_uint = 0x00dc;
pub const CX2072X_AFG_POWER_STATE: c_uint = 0x0414;
pub const CX2072X_UM_RESPONSE: c_uint = 0x0420;
pub const CX2072X_GPIO_DATA: c_uint = 0x0454;
pub const CX2072X_GPIO_ENABLE: c_uint = 0x0458;
pub const CX2072X_GPIO_DIRECTION: c_uint = 0x045c;
pub const CX2072X_GPIO_WAKE: c_uint = 0x0460;
pub const CX2072X_GPIO_UM_ENABLE: c_uint = 0x0464;
pub const CX2072X_GPIO_STICKY_MASK: c_uint = 0x0468;
pub const CX2072X_AFG_FUNCTION_RESET: c_uint = 0x07fc;
pub const CX2072X_DAC1_CONVERTER_FORMAT: c_uint = 0x43c8;
pub const CX2072X_DAC1_AMP_GAIN_RIGHT: c_uint = 0x41c0;
pub const CX2072X_DAC1_AMP_GAIN_LEFT: c_uint = 0x41e0;
pub const CX2072X_DAC1_POWER_STATE: c_uint = 0x4014;
pub const CX2072X_DAC1_CONVERTER_STREAM_CHANNEL: c_uint = 0x4018;
pub const CX2072X_DAC1_EAPD_ENABLE: c_uint = 0x4030;
pub const CX2072X_DAC2_CONVERTER_FORMAT: c_uint = 0x47c8;
pub const CX2072X_DAC2_AMP_GAIN_RIGHT: c_uint = 0x45c0;
pub const CX2072X_DAC2_AMP_GAIN_LEFT: c_uint = 0x45e0;
pub const CX2072X_DAC2_POWER_STATE: c_uint = 0x4414;
pub const CX2072X_DAC2_CONVERTER_STREAM_CHANNEL: c_uint = 0x4418;
pub const CX2072X_ADC1_CONVERTER_FORMAT: c_uint = 0x4fc8;
pub const CX2072X_ADC1_AMP_GAIN_RIGHT_0: c_uint = 0x4d80;
pub const CX2072X_ADC1_AMP_GAIN_LEFT_0: c_uint = 0x4da0;
pub const CX2072X_ADC1_AMP_GAIN_RIGHT_1: c_uint = 0x4d84;
pub const CX2072X_ADC1_AMP_GAIN_LEFT_1: c_uint = 0x4da4;
pub const CX2072X_ADC1_AMP_GAIN_RIGHT_2: c_uint = 0x4d88;
pub const CX2072X_ADC1_AMP_GAIN_LEFT_2: c_uint = 0x4da8;
pub const CX2072X_ADC1_AMP_GAIN_RIGHT_3: c_uint = 0x4d8c;
pub const CX2072X_ADC1_AMP_GAIN_LEFT_3: c_uint = 0x4dac;
pub const CX2072X_ADC1_AMP_GAIN_RIGHT_4: c_uint = 0x4d90;
pub const CX2072X_ADC1_AMP_GAIN_LEFT_4: c_uint = 0x4db0;
pub const CX2072X_ADC1_AMP_GAIN_RIGHT_5: c_uint = 0x4d94;
pub const CX2072X_ADC1_AMP_GAIN_LEFT_5: c_uint = 0x4db4;
pub const CX2072X_ADC1_AMP_GAIN_RIGHT_6: c_uint = 0x4d98;
pub const CX2072X_ADC1_AMP_GAIN_LEFT_6: c_uint = 0x4db8;
pub const CX2072X_ADC1_CONNECTION_SELECT_CONTROL: c_uint = 0x4c04;
pub const CX2072X_ADC1_POWER_STATE: c_uint = 0x4c14;
pub const CX2072X_ADC1_CONVERTER_STREAM_CHANNEL: c_uint = 0x4c18;
pub const CX2072X_ADC2_CONVERTER_FORMAT: c_uint = 0x53c8;
pub const CX2072X_ADC2_AMP_GAIN_RIGHT_0: c_uint = 0x5180;
pub const CX2072X_ADC2_AMP_GAIN_LEFT_0: c_uint = 0x51a0;
pub const CX2072X_ADC2_AMP_GAIN_RIGHT_1: c_uint = 0x5184;
pub const CX2072X_ADC2_AMP_GAIN_LEFT_1: c_uint = 0x51a4;
pub const CX2072X_ADC2_AMP_GAIN_RIGHT_2: c_uint = 0x5188;
pub const CX2072X_ADC2_AMP_GAIN_LEFT_2: c_uint = 0x51a8;
pub const CX2072X_ADC2_CONNECTION_SELECT_CONTROL: c_uint = 0x5004;
pub const CX2072X_ADC2_POWER_STATE: c_uint = 0x5014;
pub const CX2072X_ADC2_CONVERTER_STREAM_CHANNEL: c_uint = 0x5018;
pub const CX2072X_PORTA_CONNECTION_SELECT_CTRL: c_uint = 0x5804;
pub const CX2072X_PORTA_POWER_STATE: c_uint = 0x5814;
pub const CX2072X_PORTA_PIN_CTRL: c_uint = 0x581c;
pub const CX2072X_PORTA_UNSOLICITED_RESPONSE: c_uint = 0x5820;
pub const CX2072X_PORTA_PIN_SENSE: c_uint = 0x5824;
pub const CX2072X_PORTA_EAPD_BTL: c_uint = 0x5830;
pub const CX2072X_PORTB_POWER_STATE: c_uint = 0x6014;
pub const CX2072X_PORTB_PIN_CTRL: c_uint = 0x601c;
pub const CX2072X_PORTB_UNSOLICITED_RESPONSE: c_uint = 0x6020;
pub const CX2072X_PORTB_PIN_SENSE: c_uint = 0x6024;
pub const CX2072X_PORTB_EAPD_BTL: c_uint = 0x6030;
pub const CX2072X_PORTB_GAIN_RIGHT: c_uint = 0x6180;
pub const CX2072X_PORTB_GAIN_LEFT: c_uint = 0x61a0;
pub const CX2072X_PORTC_POWER_STATE: c_uint = 0x6814;
pub const CX2072X_PORTC_PIN_CTRL: c_uint = 0x681c;
pub const CX2072X_PORTC_GAIN_RIGHT: c_uint = 0x6980;
pub const CX2072X_PORTC_GAIN_LEFT: c_uint = 0x69a0;
pub const CX2072X_PORTD_POWER_STATE: c_uint = 0x6414;
pub const CX2072X_PORTD_PIN_CTRL: c_uint = 0x641c;
pub const CX2072X_PORTD_UNSOLICITED_RESPONSE: c_uint = 0x6420;
pub const CX2072X_PORTD_PIN_SENSE: c_uint = 0x6424;
pub const CX2072X_PORTD_GAIN_RIGHT: c_uint = 0x6580;
pub const CX2072X_PORTD_GAIN_LEFT: c_uint = 0x65a0;
pub const CX2072X_PORTE_CONNECTION_SELECT_CTRL: c_uint = 0x7404;
pub const CX2072X_PORTE_POWER_STATE: c_uint = 0x7414;
pub const CX2072X_PORTE_PIN_CTRL: c_uint = 0x741c;
pub const CX2072X_PORTE_UNSOLICITED_RESPONSE: c_uint = 0x7420;
pub const CX2072X_PORTE_PIN_SENSE: c_uint = 0x7424;
pub const CX2072X_PORTE_EAPD_BTL: c_uint = 0x7430;
pub const CX2072X_PORTE_GAIN_RIGHT: c_uint = 0x7580;
pub const CX2072X_PORTE_GAIN_LEFT: c_uint = 0x75a0;
pub const CX2072X_PORTF_POWER_STATE: c_uint = 0x7814;
pub const CX2072X_PORTF_PIN_CTRL: c_uint = 0x781c;
pub const CX2072X_PORTF_UNSOLICITED_RESPONSE: c_uint = 0x7820;
pub const CX2072X_PORTF_PIN_SENSE: c_uint = 0x7824;
pub const CX2072X_PORTF_GAIN_RIGHT: c_uint = 0x7980;
pub const CX2072X_PORTF_GAIN_LEFT: c_uint = 0x79a0;
pub const CX2072X_PORTG_POWER_STATE: c_uint = 0x5c14;
pub const CX2072X_PORTG_PIN_CTRL: c_uint = 0x5c1c;
pub const CX2072X_PORTG_CONNECTION_SELECT_CTRL: c_uint = 0x5c04;
pub const CX2072X_PORTG_EAPD_BTL: c_uint = 0x5c30;
pub const CX2072X_PORTM_POWER_STATE: c_uint = 0x8814;
pub const CX2072X_PORTM_PIN_CTRL: c_uint = 0x881c;
pub const CX2072X_PORTM_CONNECTION_SELECT_CTRL: c_uint = 0x8804;
pub const CX2072X_PORTM_EAPD_BTL: c_uint = 0x8830;
pub const CX2072X_MIXER_POWER_STATE: c_uint = 0x5414;
pub const CX2072X_MIXER_GAIN_RIGHT_0: c_uint = 0x5580;
pub const CX2072X_MIXER_GAIN_LEFT_0: c_uint = 0x55a0;
pub const CX2072X_MIXER_GAIN_RIGHT_1: c_uint = 0x5584;
pub const CX2072X_MIXER_GAIN_LEFT_1: c_uint = 0x55a4;
pub const CX2072X_EQ_ENABLE_BYPASS: c_uint = 0x6d00;
pub const CX2072X_EQ_B0_COEFF: c_uint = 0x6d02;
pub const CX2072X_EQ_B1_COEFF: c_uint = 0x6d04;
pub const CX2072X_EQ_B2_COEFF: c_uint = 0x6d06;
pub const CX2072X_EQ_A1_COEFF: c_uint = 0x6d08;
pub const CX2072X_EQ_A2_COEFF: c_uint = 0x6d0a;
pub const CX2072X_EQ_G_COEFF: c_uint = 0x6d0c;
pub const CX2072X_EQ_BAND: c_uint = 0x6d0d;
pub const CX2072X_SPKR_DRC_ENABLE_STEP: c_uint = 0x6d10;
pub const CX2072X_SPKR_DRC_CONTROL: c_uint = 0x6d14;
pub const CX2072X_SPKR_DRC_TEST: c_uint = 0x6d18;
pub const CX2072X_DIGITAL_BIOS_TEST0: c_uint = 0x6d80;
pub const CX2072X_DIGITAL_BIOS_TEST2: c_uint = 0x6d84;
pub const CX2072X_I2SPCM_CONTROL1: c_uint = 0x6e00;
pub const CX2072X_I2SPCM_CONTROL2: c_uint = 0x6e04;
pub const CX2072X_I2SPCM_CONTROL3: c_uint = 0x6e08;
pub const CX2072X_I2SPCM_CONTROL4: c_uint = 0x6e0c;
pub const CX2072X_I2SPCM_CONTROL5: c_uint = 0x6e10;
pub const CX2072X_I2SPCM_CONTROL6: c_uint = 0x6e18;
pub const CX2072X_UM_INTERRUPT_CRTL_E: c_uint = 0x6e14;
pub const CX2072X_CODEC_TEST2: c_uint = 0x7108;
pub const CX2072X_CODEC_TEST9: c_uint = 0x7124;
pub const CX2072X_CODEC_TESTXX: c_uint = 0x7290;
pub const CX2072X_CODEC_TEST20: c_uint = 0x7310;
pub const CX2072X_CODEC_TEST24: c_uint = 0x731c;
pub const CX2072X_CODEC_TEST26: c_uint = 0x7328;
pub const CX2072X_ANALOG_TEST3: c_uint = 0x718c;
pub const CX2072X_ANALOG_TEST4: c_uint = 0x7190;
pub const CX2072X_ANALOG_TEST5: c_uint = 0x7194;
pub const CX2072X_ANALOG_TEST6: c_uint = 0x7198;
pub const CX2072X_ANALOG_TEST7: c_uint = 0x719c;
pub const CX2072X_ANALOG_TEST8: c_uint = 0x71a0;
pub const CX2072X_ANALOG_TEST9: c_uint = 0x71a4;
pub const CX2072X_ANALOG_TEST10: c_uint = 0x71a8;
pub const CX2072X_ANALOG_TEST11: c_uint = 0x71ac;
pub const CX2072X_ANALOG_TEST12: c_uint = 0x71b0;
pub const CX2072X_ANALOG_TEST13: c_uint = 0x71b4;
pub const CX2072X_DIGITAL_TEST0: c_uint = 0x7200;
pub const CX2072X_DIGITAL_TEST1: c_uint = 0x7204;
pub const CX2072X_DIGITAL_TEST11: c_uint = 0x722c;
pub const CX2072X_DIGITAL_TEST12: c_uint = 0x7230;
pub const CX2072X_DIGITAL_TEST15: c_uint = 0x723c;
pub const CX2072X_DIGITAL_TEST16: c_uint = 0x7080;
pub const CX2072X_DIGITAL_TEST17: c_uint = 0x7084;
pub const CX2072X_DIGITAL_TEST18: c_uint = 0x7088;
pub const CX2072X_DIGITAL_TEST19: c_uint = 0x708c;
pub const CX2072X_DIGITAL_TEST20: c_uint = 0x7090;
// not used in the current code, for future extensions (if any)
pub const CX2072X_MAX_EQ_BAND: c_int = 7;
pub const CX2072X_MAX_EQ_COEFF: c_int = 11;
pub const CX2072X_MAX_DRC_REGS: c_int = 9;
pub const CX2072X_MIC_EQ_COEFF: c_int = 10;
pub const CX2072X_PLBK_EQ_BAND_NUM: c_int = 7;
pub const CX2072X_PLBK_EQ_COEF_LEN: c_int = 11;
pub const CX2072X_PLBK_DRC_PARM_LEN: c_int = 9;
pub const CX2072X_CLASSD_AMP_LEN: c_int = 6;
// DAI interface type
pub const CX2072X_DAI_HIFI: c_int = 1;
pub const CX2072X_DAI_DSP: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cx2072x_reg_sample_size {
    CX2072X_SAMPLE_SIZE_8_BITS = 0,
    CX2072X_SAMPLE_SIZE_16_BITS = 1,
    CX2072X_SAMPLE_SIZE_24_BITS = 2,
    CX2072X_SAMPLE_SIZE_RESERVED = 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cx2072x_reg_i2spcm_ctrl_reg1 {
    pub rx_data_one_line:1: u32,
    pub rx_ws_pol:1: u32,
    pub rx_ws_wid:7: u32,
    pub rx_frm_len:5: u32,
    pub rx_sa_size:2: u32,
    pub tx_data_one_line:1: u32,
    pub tx_ws_pol:1: u32,
    pub tx_ws_wid:7: u32,
    pub tx_frm_len:5: u32,
    pub tx_sa_size:2: u32,
    pub r: },
    pub ulval: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cx2072x_reg_i2spcm_ctrl_reg2 {
    pub tx_en_ch1:1: u32,
    pub tx_en_ch2:1: u32,
    pub tx_en_ch3:1: u32,
    pub tx_en_ch4:1: u32,
    pub tx_en_ch5:1: u32,
    pub tx_en_ch6:1: u32,
    pub tx_slot_1:5: u32,
    pub tx_slot_2:5: u32,
    pub tx_slot_3:5: u32,
    pub tx_slot_4:5: u32,
    pub res:1: u32,
    pub tx_data_neg_bclk:1: u32,
    pub tx_master:1: u32,
    pub tx_tri_n:1: u32,
    pub tx_endian_sel:1: u32,
    pub tx_dstart_dly:1: u32,
    pub r: },
    pub ulval: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cx2072x_reg_i2spcm_ctrl_reg3 {
    pub rx_en_ch1:1: u32,
    pub rx_en_ch2:1: u32,
    pub rx_en_ch3:1: u32,
    pub rx_en_ch4:1: u32,
    pub rx_en_ch5:1: u32,
    pub rx_en_ch6:1: u32,
    pub rx_slot_1:5: u32,
    pub rx_slot_2:5: u32,
    pub rx_slot_3:5: u32,
    pub rx_slot_4:5: u32,
    pub res:1: u32,
    pub rx_data_neg_bclk:1: u32,
    pub rx_master:1: u32,
    pub rx_tri_n:1: u32,
    pub rx_endian_sel:1: u32,
    pub rx_dstart_dly:1: u32,
    pub r: },
    pub ulval: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cx2072x_reg_i2spcm_ctrl_reg4 {
    pub rx_mute:1: u32,
    pub tx_mute:1: u32,
    pub reserved:1: u32,
    pub dac_34_independent:1: u32,
    pub dac_bclk_lrck_share:1: u32,
    pub bclk_lrck_share_en:1: u32,
    pub reserved2:2: u32,
    pub rx_last_dac_ch_en:1: u32,
    pub rx_last_dac_ch:3: u32,
    pub tx_last_adc_ch_en:1: u32,
    pub tx_last_adc_ch:3: u32,
    pub rx_slot_5:5: u32,
    pub rx_slot_6:5: u32,
    pub reserved3:6: u32,
    pub r: },
    pub ulval: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cx2072x_reg_i2spcm_ctrl_reg5 {
    pub tx_slot_5:5: u32,
    pub reserved:3: u32,
    pub tx_slot_6:5: u32,
    pub reserved2:3: u32,
    pub reserved3:8: u32,
    pub i2s_pcm_clk_div:7: u32,
    pub i2s_pcm_clk_div_chan_en:1: u32,
    pub r: },
    pub ulval: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cx2072x_reg_i2spcm_ctrl_reg6 {
    pub reserved:5: u32,
    pub rx_pause_cycles:3: u32,
    pub rx_pause_start_pos:8: u32,
    pub reserved2:5: u32,
    pub tx_pause_cycles:3: u32,
    pub tx_pause_start_pos:8: u32,
    pub r: },
    pub ulval: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cx2072x_reg_digital_bios_test2 {
    pub pull_down_eapd:2: u32,
    pub input_en_eapd_pad:1: u32,
    pub push_pull_mode:1: u32,
    pub eapd_pad_output_driver:2: u32,
    pub pll_source:1: u32,
    pub i2s_bclk_en:1: u32,
    pub i2s_bclk_invert:1: u32,
    pub pll_ref_clock:1: u32,
    pub class_d_shield_clk:1: u32,
    pub audio_pll_bypass_mode:1: u32,
    pub reserved:4: u32,
    pub r: },
    pub ulval: u32,
}
