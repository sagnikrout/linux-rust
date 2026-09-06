//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/wm8993.h
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
pub const WM8993_SYSCLK_MCLK: c_int = 1;
pub const WM8993_SYSCLK_FLL: c_int = 2;
pub const WM8993_FLL_MCLK: c_int = 1;
pub const WM8993_FLL_BCLK: c_int = 2;
pub const WM8993_FLL_LRCLK: c_int = 3;
//
// Register values.
//
pub const WM8993_SOFTWARE_RESET: c_uint = 0x00;
pub const WM8993_POWER_MANAGEMENT_1: c_uint = 0x01;
pub const WM8993_POWER_MANAGEMENT_2: c_uint = 0x02;
pub const WM8993_POWER_MANAGEMENT_3: c_uint = 0x03;
pub const WM8993_AUDIO_INTERFACE_1: c_uint = 0x04;
pub const WM8993_AUDIO_INTERFACE_2: c_uint = 0x05;
pub const WM8993_CLOCKING_1: c_uint = 0x06;
pub const WM8993_CLOCKING_2: c_uint = 0x07;
pub const WM8993_AUDIO_INTERFACE_3: c_uint = 0x08;
pub const WM8993_AUDIO_INTERFACE_4: c_uint = 0x09;
pub const WM8993_DAC_CTRL: c_uint = 0x0A;
pub const WM8993_LEFT_DAC_DIGITAL_VOLUME: c_uint = 0x0B;
pub const WM8993_RIGHT_DAC_DIGITAL_VOLUME: c_uint = 0x0C;
pub const WM8993_DIGITAL_SIDE_TONE: c_uint = 0x0D;
pub const WM8993_ADC_CTRL: c_uint = 0x0E;
pub const WM8993_LEFT_ADC_DIGITAL_VOLUME: c_uint = 0x0F;
pub const WM8993_RIGHT_ADC_DIGITAL_VOLUME: c_uint = 0x10;
pub const WM8993_GPIO_CTRL_1: c_uint = 0x12;
pub const WM8993_GPIO1: c_uint = 0x13;
pub const WM8993_IRQ_DEBOUNCE: c_uint = 0x14;
pub const WM8993_INPUTS_CLAMP_REG: c_uint = 0x15;
pub const WM8993_GPIOCTRL_2: c_uint = 0x16;
pub const WM8993_GPIO_POL: c_uint = 0x17;
pub const WM8993_LEFT_LINE_INPUT_1_2_VOLUME: c_uint = 0x18;
pub const WM8993_LEFT_LINE_INPUT_3_4_VOLUME: c_uint = 0x19;
pub const WM8993_RIGHT_LINE_INPUT_1_2_VOLUME: c_uint = 0x1A;
pub const WM8993_RIGHT_LINE_INPUT_3_4_VOLUME: c_uint = 0x1B;
pub const WM8993_LEFT_OUTPUT_VOLUME: c_uint = 0x1C;
pub const WM8993_RIGHT_OUTPUT_VOLUME: c_uint = 0x1D;
pub const WM8993_LINE_OUTPUTS_VOLUME: c_uint = 0x1E;
pub const WM8993_HPOUT2_VOLUME: c_uint = 0x1F;
pub const WM8993_LEFT_OPGA_VOLUME: c_uint = 0x20;
pub const WM8993_RIGHT_OPGA_VOLUME: c_uint = 0x21;
pub const WM8993_SPKMIXL_ATTENUATION: c_uint = 0x22;
pub const WM8993_SPKMIXR_ATTENUATION: c_uint = 0x23;
pub const WM8993_SPKOUT_MIXERS: c_uint = 0x24;
pub const WM8993_SPKOUT_BOOST: c_uint = 0x25;
pub const WM8993_SPEAKER_VOLUME_LEFT: c_uint = 0x26;
pub const WM8993_SPEAKER_VOLUME_RIGHT: c_uint = 0x27;
pub const WM8993_INPUT_MIXER2: c_uint = 0x28;
pub const WM8993_INPUT_MIXER3: c_uint = 0x29;
pub const WM8993_INPUT_MIXER4: c_uint = 0x2A;
pub const WM8993_INPUT_MIXER5: c_uint = 0x2B;
pub const WM8993_INPUT_MIXER6: c_uint = 0x2C;
pub const WM8993_OUTPUT_MIXER1: c_uint = 0x2D;
pub const WM8993_OUTPUT_MIXER2: c_uint = 0x2E;
pub const WM8993_OUTPUT_MIXER3: c_uint = 0x2F;
pub const WM8993_OUTPUT_MIXER4: c_uint = 0x30;
pub const WM8993_OUTPUT_MIXER5: c_uint = 0x31;
pub const WM8993_OUTPUT_MIXER6: c_uint = 0x32;
pub const WM8993_HPOUT2_MIXER: c_uint = 0x33;
pub const WM8993_LINE_MIXER1: c_uint = 0x34;
pub const WM8993_LINE_MIXER2: c_uint = 0x35;
pub const WM8993_SPEAKER_MIXER: c_uint = 0x36;
pub const WM8993_ADDITIONAL_CONTROL: c_uint = 0x37;
pub const WM8993_ANTIPOP1: c_uint = 0x38;
pub const WM8993_ANTIPOP2: c_uint = 0x39;
pub const WM8993_MICBIAS: c_uint = 0x3A;
pub const WM8993_FLL_CONTROL_1: c_uint = 0x3C;
pub const WM8993_FLL_CONTROL_2: c_uint = 0x3D;
pub const WM8993_FLL_CONTROL_3: c_uint = 0x3E;
pub const WM8993_FLL_CONTROL_4: c_uint = 0x3F;
pub const WM8993_FLL_CONTROL_5: c_uint = 0x40;
pub const WM8993_CLOCKING_3: c_uint = 0x41;
pub const WM8993_CLOCKING_4: c_uint = 0x42;
pub const WM8993_MW_SLAVE_CONTROL: c_uint = 0x43;
pub const WM8993_BUS_CONTROL_1: c_uint = 0x45;
pub const WM8993_WRITE_SEQUENCER_0: c_uint = 0x46;
pub const WM8993_WRITE_SEQUENCER_1: c_uint = 0x47;
pub const WM8993_WRITE_SEQUENCER_2: c_uint = 0x48;
pub const WM8993_WRITE_SEQUENCER_3: c_uint = 0x49;
pub const WM8993_WRITE_SEQUENCER_4: c_uint = 0x4A;
pub const WM8993_WRITE_SEQUENCER_5: c_uint = 0x4B;
pub const WM8993_CHARGE_PUMP_1: c_uint = 0x4C;
pub const WM8993_CLASS_W_0: c_uint = 0x51;
pub const WM8993_DC_SERVO_0: c_uint = 0x54;
pub const WM8993_DC_SERVO_1: c_uint = 0x55;
pub const WM8993_DC_SERVO_3: c_uint = 0x57;
pub const WM8993_DC_SERVO_READBACK_0: c_uint = 0x58;
pub const WM8993_DC_SERVO_READBACK_1: c_uint = 0x59;
pub const WM8993_DC_SERVO_READBACK_2: c_uint = 0x5A;
pub const WM8993_ANALOGUE_HP_0: c_uint = 0x60;
pub const WM8993_EQ1: c_uint = 0x62;
pub const WM8993_EQ2: c_uint = 0x63;
pub const WM8993_EQ3: c_uint = 0x64;
pub const WM8993_EQ4: c_uint = 0x65;
pub const WM8993_EQ5: c_uint = 0x66;
pub const WM8993_EQ6: c_uint = 0x67;
pub const WM8993_EQ7: c_uint = 0x68;
pub const WM8993_EQ8: c_uint = 0x69;
pub const WM8993_EQ9: c_uint = 0x6A;
pub const WM8993_EQ10: c_uint = 0x6B;
pub const WM8993_EQ11: c_uint = 0x6C;
pub const WM8993_EQ12: c_uint = 0x6D;
pub const WM8993_EQ13: c_uint = 0x6E;
pub const WM8993_EQ14: c_uint = 0x6F;
pub const WM8993_EQ15: c_uint = 0x70;
pub const WM8993_EQ16: c_uint = 0x71;
pub const WM8993_EQ17: c_uint = 0x72;
pub const WM8993_EQ18: c_uint = 0x73;
pub const WM8993_EQ19: c_uint = 0x74;
pub const WM8993_EQ20: c_uint = 0x75;
pub const WM8993_EQ21: c_uint = 0x76;
pub const WM8993_EQ22: c_uint = 0x77;
pub const WM8993_EQ23: c_uint = 0x78;
pub const WM8993_EQ24: c_uint = 0x79;
pub const WM8993_DIGITAL_PULLS: c_uint = 0x7A;
pub const WM8993_DRC_CONTROL_1: c_uint = 0x7B;
pub const WM8993_DRC_CONTROL_2: c_uint = 0x7C;
pub const WM8993_DRC_CONTROL_3: c_uint = 0x7D;
pub const WM8993_DRC_CONTROL_4: c_uint = 0x7E;
pub const WM8993_REGISTER_COUNT: c_uint = 0x7F;
pub const WM8993_MAX_REGISTER: c_uint = 0x7E;
//
// Field Definitions.
//
// R0 (0x00) - Software Reset
//
pub const WM8993_SW_RESET_MASK: c_uint = 0xFFFF  /* SW_RESET - [15:0] */;

//
// R1 (0x01) - Power Management (1)
//
pub const WM8993_SPKOUTR_ENA: c_uint = 0x2000  /* SPKOUTR_ENA */;
pub const WM8993_SPKOUTR_ENA_MASK: c_uint = 0x2000  /* SPKOUTR_ENA */;

pub const WM8993_SPKOUTL_ENA: c_uint = 0x1000  /* SPKOUTL_ENA */;
pub const WM8993_SPKOUTL_ENA_MASK: c_uint = 0x1000  /* SPKOUTL_ENA */;

pub const WM8993_HPOUT2_ENA: c_uint = 0x0800  /* HPOUT2_ENA */;
pub const WM8993_HPOUT2_ENA_MASK: c_uint = 0x0800  /* HPOUT2_ENA */;

pub const WM8993_HPOUT1L_ENA: c_uint = 0x0200  /* HPOUT1L_ENA */;
pub const WM8993_HPOUT1L_ENA_MASK: c_uint = 0x0200  /* HPOUT1L_ENA */;

pub const WM8993_HPOUT1R_ENA: c_uint = 0x0100  /* HPOUT1R_ENA */;
pub const WM8993_HPOUT1R_ENA_MASK: c_uint = 0x0100  /* HPOUT1R_ENA */;

pub const WM8993_MICB2_ENA: c_uint = 0x0020  /* MICB2_ENA */;
pub const WM8993_MICB2_ENA_MASK: c_uint = 0x0020  /* MICB2_ENA */;

pub const WM8993_MICB1_ENA: c_uint = 0x0010  /* MICB1_ENA */;
pub const WM8993_MICB1_ENA_MASK: c_uint = 0x0010  /* MICB1_ENA */;

pub const WM8993_VMID_SEL_MASK: c_uint = 0x0006  /* VMID_SEL - [2:1] */;

pub const WM8993_BIAS_ENA: c_uint = 0x0001  /* BIAS_ENA */;
pub const WM8993_BIAS_ENA_MASK: c_uint = 0x0001  /* BIAS_ENA */;

//
// R2 (0x02) - Power Management (2)
//
pub const WM8993_TSHUT_ENA: c_uint = 0x4000  /* TSHUT_ENA */;
pub const WM8993_TSHUT_ENA_MASK: c_uint = 0x4000  /* TSHUT_ENA */;

pub const WM8993_TSHUT_OPDIS: c_uint = 0x2000  /* TSHUT_OPDIS */;
pub const WM8993_TSHUT_OPDIS_MASK: c_uint = 0x2000  /* TSHUT_OPDIS */;

pub const WM8993_OPCLK_ENA: c_uint = 0x0800  /* OPCLK_ENA */;
pub const WM8993_OPCLK_ENA_MASK: c_uint = 0x0800  /* OPCLK_ENA */;

pub const WM8993_MIXINL_ENA: c_uint = 0x0200  /* MIXINL_ENA */;
pub const WM8993_MIXINL_ENA_MASK: c_uint = 0x0200  /* MIXINL_ENA */;

pub const WM8993_MIXINR_ENA: c_uint = 0x0100  /* MIXINR_ENA */;
pub const WM8993_MIXINR_ENA_MASK: c_uint = 0x0100  /* MIXINR_ENA */;

pub const WM8993_IN2L_ENA: c_uint = 0x0080  /* IN2L_ENA */;
pub const WM8993_IN2L_ENA_MASK: c_uint = 0x0080  /* IN2L_ENA */;

pub const WM8993_IN1L_ENA: c_uint = 0x0040  /* IN1L_ENA */;
pub const WM8993_IN1L_ENA_MASK: c_uint = 0x0040  /* IN1L_ENA */;

pub const WM8993_IN2R_ENA: c_uint = 0x0020  /* IN2R_ENA */;
pub const WM8993_IN2R_ENA_MASK: c_uint = 0x0020  /* IN2R_ENA */;

pub const WM8993_IN1R_ENA: c_uint = 0x0010  /* IN1R_ENA */;
pub const WM8993_IN1R_ENA_MASK: c_uint = 0x0010  /* IN1R_ENA */;

pub const WM8993_ADCL_ENA: c_uint = 0x0002  /* ADCL_ENA */;
pub const WM8993_ADCL_ENA_MASK: c_uint = 0x0002  /* ADCL_ENA */;

pub const WM8993_ADCR_ENA: c_uint = 0x0001  /* ADCR_ENA */;
pub const WM8993_ADCR_ENA_MASK: c_uint = 0x0001  /* ADCR_ENA */;

//
// R3 (0x03) - Power Management (3)
//
pub const WM8993_LINEOUT1N_ENA: c_uint = 0x2000  /* LINEOUT1N_ENA */;
pub const WM8993_LINEOUT1N_ENA_MASK: c_uint = 0x2000  /* LINEOUT1N_ENA */;

pub const WM8993_LINEOUT1P_ENA: c_uint = 0x1000  /* LINEOUT1P_ENA */;
pub const WM8993_LINEOUT1P_ENA_MASK: c_uint = 0x1000  /* LINEOUT1P_ENA */;

pub const WM8993_LINEOUT2N_ENA: c_uint = 0x0800  /* LINEOUT2N_ENA */;
pub const WM8993_LINEOUT2N_ENA_MASK: c_uint = 0x0800  /* LINEOUT2N_ENA */;

pub const WM8993_LINEOUT2P_ENA: c_uint = 0x0400  /* LINEOUT2P_ENA */;
pub const WM8993_LINEOUT2P_ENA_MASK: c_uint = 0x0400  /* LINEOUT2P_ENA */;

pub const WM8993_SPKRVOL_ENA: c_uint = 0x0200  /* SPKRVOL_ENA */;
pub const WM8993_SPKRVOL_ENA_MASK: c_uint = 0x0200  /* SPKRVOL_ENA */;

pub const WM8993_SPKLVOL_ENA: c_uint = 0x0100  /* SPKLVOL_ENA */;
pub const WM8993_SPKLVOL_ENA_MASK: c_uint = 0x0100  /* SPKLVOL_ENA */;

pub const WM8993_MIXOUTLVOL_ENA: c_uint = 0x0080  /* MIXOUTLVOL_ENA */;
pub const WM8993_MIXOUTLVOL_ENA_MASK: c_uint = 0x0080  /* MIXOUTLVOL_ENA */;

pub const WM8993_MIXOUTRVOL_ENA: c_uint = 0x0040  /* MIXOUTRVOL_ENA */;
pub const WM8993_MIXOUTRVOL_ENA_MASK: c_uint = 0x0040  /* MIXOUTRVOL_ENA */;

pub const WM8993_MIXOUTL_ENA: c_uint = 0x0020  /* MIXOUTL_ENA */;
pub const WM8993_MIXOUTL_ENA_MASK: c_uint = 0x0020  /* MIXOUTL_ENA */;

pub const WM8993_MIXOUTR_ENA: c_uint = 0x0010  /* MIXOUTR_ENA */;
pub const WM8993_MIXOUTR_ENA_MASK: c_uint = 0x0010  /* MIXOUTR_ENA */;

pub const WM8993_DACL_ENA: c_uint = 0x0002  /* DACL_ENA */;
pub const WM8993_DACL_ENA_MASK: c_uint = 0x0002  /* DACL_ENA */;

pub const WM8993_DACR_ENA: c_uint = 0x0001  /* DACR_ENA */;
pub const WM8993_DACR_ENA_MASK: c_uint = 0x0001  /* DACR_ENA */;

//
// R4 (0x04) - Audio Interface (1)
//
pub const WM8993_AIFADCL_SRC: c_uint = 0x8000  /* AIFADCL_SRC */;
pub const WM8993_AIFADCL_SRC_MASK: c_uint = 0x8000  /* AIFADCL_SRC */;

pub const WM8993_AIFADCR_SRC: c_uint = 0x4000  /* AIFADCR_SRC */;
pub const WM8993_AIFADCR_SRC_MASK: c_uint = 0x4000  /* AIFADCR_SRC */;

pub const WM8993_AIFADC_TDM: c_uint = 0x2000  /* AIFADC_TDM */;
pub const WM8993_AIFADC_TDM_MASK: c_uint = 0x2000  /* AIFADC_TDM */;

pub const WM8993_AIFADC_TDM_CHAN: c_uint = 0x1000  /* AIFADC_TDM_CHAN */;
pub const WM8993_AIFADC_TDM_CHAN_MASK: c_uint = 0x1000  /* AIFADC_TDM_CHAN */;

pub const WM8993_BCLK_DIR: c_uint = 0x0200  /* BCLK_DIR */;
pub const WM8993_BCLK_DIR_MASK: c_uint = 0x0200  /* BCLK_DIR */;

pub const WM8993_AIF_BCLK_INV: c_uint = 0x0100  /* AIF_BCLK_INV */;
pub const WM8993_AIF_BCLK_INV_MASK: c_uint = 0x0100  /* AIF_BCLK_INV */;

pub const WM8993_AIF_LRCLK_INV: c_uint = 0x0080  /* AIF_LRCLK_INV */;
pub const WM8993_AIF_LRCLK_INV_MASK: c_uint = 0x0080  /* AIF_LRCLK_INV */;

pub const WM8993_AIF_WL_MASK: c_uint = 0x0060  /* AIF_WL - [6:5] */;

pub const WM8993_AIF_FMT_MASK: c_uint = 0x0018  /* AIF_FMT - [4:3] */;

//
// R5 (0x05) - Audio Interface (2)
//
pub const WM8993_AIFDACL_SRC: c_uint = 0x8000  /* AIFDACL_SRC */;
pub const WM8993_AIFDACL_SRC_MASK: c_uint = 0x8000  /* AIFDACL_SRC */;

pub const WM8993_AIFDACR_SRC: c_uint = 0x4000  /* AIFDACR_SRC */;
pub const WM8993_AIFDACR_SRC_MASK: c_uint = 0x4000  /* AIFDACR_SRC */;

pub const WM8993_AIFDAC_TDM: c_uint = 0x2000  /* AIFDAC_TDM */;
pub const WM8993_AIFDAC_TDM_MASK: c_uint = 0x2000  /* AIFDAC_TDM */;

pub const WM8993_AIFDAC_TDM_CHAN: c_uint = 0x1000  /* AIFDAC_TDM_CHAN */;
pub const WM8993_AIFDAC_TDM_CHAN_MASK: c_uint = 0x1000  /* AIFDAC_TDM_CHAN */;

pub const WM8993_DAC_BOOST_MASK: c_uint = 0x0C00  /* DAC_BOOST - [11:10] */;

pub const WM8993_DAC_COMP: c_uint = 0x0010  /* DAC_COMP */;
pub const WM8993_DAC_COMP_MASK: c_uint = 0x0010  /* DAC_COMP */;

pub const WM8993_DAC_COMPMODE: c_uint = 0x0008  /* DAC_COMPMODE */;
pub const WM8993_DAC_COMPMODE_MASK: c_uint = 0x0008  /* DAC_COMPMODE */;

pub const WM8993_ADC_COMP: c_uint = 0x0004  /* ADC_COMP */;
pub const WM8993_ADC_COMP_MASK: c_uint = 0x0004  /* ADC_COMP */;

pub const WM8993_ADC_COMPMODE: c_uint = 0x0002  /* ADC_COMPMODE */;
pub const WM8993_ADC_COMPMODE_MASK: c_uint = 0x0002  /* ADC_COMPMODE */;

pub const WM8993_LOOPBACK: c_uint = 0x0001  /* LOOPBACK */;
pub const WM8993_LOOPBACK_MASK: c_uint = 0x0001  /* LOOPBACK */;

//
// R6 (0x06) - Clocking 1
//
pub const WM8993_TOCLK_RATE: c_uint = 0x8000  /* TOCLK_RATE */;
pub const WM8993_TOCLK_RATE_MASK: c_uint = 0x8000  /* TOCLK_RATE */;

pub const WM8993_TOCLK_ENA: c_uint = 0x4000  /* TOCLK_ENA */;
pub const WM8993_TOCLK_ENA_MASK: c_uint = 0x4000  /* TOCLK_ENA */;

pub const WM8993_OPCLK_DIV_MASK: c_uint = 0x1E00  /* OPCLK_DIV - [12:9] */;

pub const WM8993_DCLK_DIV_MASK: c_uint = 0x01C0  /* DCLK_DIV - [8:6] */;

pub const WM8993_BCLK_DIV_MASK: c_uint = 0x001E  /* BCLK_DIV - [4:1] */;

//
// R7 (0x07) - Clocking 2
//
pub const WM8993_MCLK_SRC: c_uint = 0x8000  /* MCLK_SRC */;
pub const WM8993_MCLK_SRC_MASK: c_uint = 0x8000  /* MCLK_SRC */;

pub const WM8993_SYSCLK_SRC: c_uint = 0x4000  /* SYSCLK_SRC */;
pub const WM8993_SYSCLK_SRC_MASK: c_uint = 0x4000  /* SYSCLK_SRC */;

pub const WM8993_MCLK_DIV: c_uint = 0x1000  /* MCLK_DIV */;
pub const WM8993_MCLK_DIV_MASK: c_uint = 0x1000  /* MCLK_DIV */;

pub const WM8993_MCLK_INV: c_uint = 0x0400  /* MCLK_INV */;
pub const WM8993_MCLK_INV_MASK: c_uint = 0x0400  /* MCLK_INV */;

pub const WM8993_ADC_DIV_MASK: c_uint = 0x00E0  /* ADC_DIV - [7:5] */;

pub const WM8993_DAC_DIV_MASK: c_uint = 0x001C  /* DAC_DIV - [4:2] */;

//
// R8 (0x08) - Audio Interface (3)
//
pub const WM8993_AIF_MSTR1: c_uint = 0x8000  /* AIF_MSTR1 */;
pub const WM8993_AIF_MSTR1_MASK: c_uint = 0x8000  /* AIF_MSTR1 */;

//
// R9 (0x09) - Audio Interface (4)
//
pub const WM8993_AIF_TRIS: c_uint = 0x2000  /* AIF_TRIS */;
pub const WM8993_AIF_TRIS_MASK: c_uint = 0x2000  /* AIF_TRIS */;

pub const WM8993_LRCLK_DIR: c_uint = 0x0800  /* LRCLK_DIR */;
pub const WM8993_LRCLK_DIR_MASK: c_uint = 0x0800  /* LRCLK_DIR */;

pub const WM8993_LRCLK_RATE_MASK: c_uint = 0x07FF  /* LRCLK_RATE - [10:0] */;

//
// R10 (0x0A) - DAC CTRL
//
pub const WM8993_DAC_OSR128: c_uint = 0x2000  /* DAC_OSR128 */;
pub const WM8993_DAC_OSR128_MASK: c_uint = 0x2000  /* DAC_OSR128 */;

pub const WM8993_DAC_MONO: c_uint = 0x0200  /* DAC_MONO */;
pub const WM8993_DAC_MONO_MASK: c_uint = 0x0200  /* DAC_MONO */;

pub const WM8993_DAC_SB_FILT: c_uint = 0x0100  /* DAC_SB_FILT */;
pub const WM8993_DAC_SB_FILT_MASK: c_uint = 0x0100  /* DAC_SB_FILT */;

pub const WM8993_DAC_MUTERATE: c_uint = 0x0080  /* DAC_MUTERATE */;
pub const WM8993_DAC_MUTERATE_MASK: c_uint = 0x0080  /* DAC_MUTERATE */;

pub const WM8993_DAC_UNMUTE_RAMP: c_uint = 0x0040  /* DAC_UNMUTE_RAMP */;
pub const WM8993_DAC_UNMUTE_RAMP_MASK: c_uint = 0x0040  /* DAC_UNMUTE_RAMP */;

pub const WM8993_DEEMPH_MASK: c_uint = 0x0030  /* DEEMPH - [5:4] */;

pub const WM8993_DAC_MUTE: c_uint = 0x0004  /* DAC_MUTE */;
pub const WM8993_DAC_MUTE_MASK: c_uint = 0x0004  /* DAC_MUTE */;

pub const WM8993_DACL_DATINV: c_uint = 0x0002  /* DACL_DATINV */;
pub const WM8993_DACL_DATINV_MASK: c_uint = 0x0002  /* DACL_DATINV */;

pub const WM8993_DACR_DATINV: c_uint = 0x0001  /* DACR_DATINV */;
pub const WM8993_DACR_DATINV_MASK: c_uint = 0x0001  /* DACR_DATINV */;

//
// R11 (0x0B) - Left DAC Digital Volume
//
pub const WM8993_DAC_VU: c_uint = 0x0100  /* DAC_VU */;
pub const WM8993_DAC_VU_MASK: c_uint = 0x0100  /* DAC_VU */;

pub const WM8993_DACL_VOL_MASK: c_uint = 0x00FF  /* DACL_VOL - [7:0] */;

//
// R12 (0x0C) - Right DAC Digital Volume
//
pub const WM8993_DAC_VU: c_uint = 0x0100  /* DAC_VU */;
pub const WM8993_DAC_VU_MASK: c_uint = 0x0100  /* DAC_VU */;

pub const WM8993_DACR_VOL_MASK: c_uint = 0x00FF  /* DACR_VOL - [7:0] */;

//
// R13 (0x0D) - Digital Side Tone
//
pub const WM8993_ADCL_DAC_SVOL_MASK: c_uint = 0x1E00  /* ADCL_DAC_SVOL - [12:9] */;

pub const WM8993_ADCR_DAC_SVOL_MASK: c_uint = 0x01E0  /* ADCR_DAC_SVOL - [8:5] */;

pub const WM8993_ADC_TO_DACL_MASK: c_uint = 0x000C  /* ADC_TO_DACL - [3:2] */;

pub const WM8993_ADC_TO_DACR_MASK: c_uint = 0x0003  /* ADC_TO_DACR - [1:0] */;

//
// R14 (0x0E) - ADC CTRL
//
pub const WM8993_ADC_OSR128: c_uint = 0x0200  /* ADC_OSR128 */;
pub const WM8993_ADC_OSR128_MASK: c_uint = 0x0200  /* ADC_OSR128 */;

pub const WM8993_ADC_HPF: c_uint = 0x0100  /* ADC_HPF */;
pub const WM8993_ADC_HPF_MASK: c_uint = 0x0100  /* ADC_HPF */;

pub const WM8993_ADC_HPF_CUT_MASK: c_uint = 0x0060  /* ADC_HPF_CUT - [6:5] */;

pub const WM8993_ADCL_DATINV: c_uint = 0x0002  /* ADCL_DATINV */;
pub const WM8993_ADCL_DATINV_MASK: c_uint = 0x0002  /* ADCL_DATINV */;

pub const WM8993_ADCR_DATINV: c_uint = 0x0001  /* ADCR_DATINV */;
pub const WM8993_ADCR_DATINV_MASK: c_uint = 0x0001  /* ADCR_DATINV */;

//
// R15 (0x0F) - Left ADC Digital Volume
//
pub const WM8993_ADC_VU: c_uint = 0x0100  /* ADC_VU */;
pub const WM8993_ADC_VU_MASK: c_uint = 0x0100  /* ADC_VU */;

pub const WM8993_ADCL_VOL_MASK: c_uint = 0x00FF  /* ADCL_VOL - [7:0] */;

//
// R16 (0x10) - Right ADC Digital Volume
//
pub const WM8993_ADC_VU: c_uint = 0x0100  /* ADC_VU */;
pub const WM8993_ADC_VU_MASK: c_uint = 0x0100  /* ADC_VU */;

pub const WM8993_ADCR_VOL_MASK: c_uint = 0x00FF  /* ADCR_VOL - [7:0] */;

//
// R18 (0x12) - GPIO CTRL 1
//
pub const WM8993_JD2_SC_EINT: c_uint = 0x8000  /* JD2_SC_EINT */;
pub const WM8993_JD2_SC_EINT_MASK: c_uint = 0x8000  /* JD2_SC_EINT */;

pub const WM8993_JD2_EINT: c_uint = 0x4000  /* JD2_EINT */;
pub const WM8993_JD2_EINT_MASK: c_uint = 0x4000  /* JD2_EINT */;

pub const WM8993_WSEQ_EINT: c_uint = 0x2000  /* WSEQ_EINT */;
pub const WM8993_WSEQ_EINT_MASK: c_uint = 0x2000  /* WSEQ_EINT */;

pub const WM8993_IRQ: c_uint = 0x1000  /* IRQ */;
pub const WM8993_IRQ_MASK: c_uint = 0x1000  /* IRQ */;

pub const WM8993_TEMPOK_EINT: c_uint = 0x0800  /* TEMPOK_EINT */;
pub const WM8993_TEMPOK_EINT_MASK: c_uint = 0x0800  /* TEMPOK_EINT */;

pub const WM8993_JD1_SC_EINT: c_uint = 0x0400  /* JD1_SC_EINT */;
pub const WM8993_JD1_SC_EINT_MASK: c_uint = 0x0400  /* JD1_SC_EINT */;

pub const WM8993_JD1_EINT: c_uint = 0x0200  /* JD1_EINT */;
pub const WM8993_JD1_EINT_MASK: c_uint = 0x0200  /* JD1_EINT */;

pub const WM8993_FLL_LOCK_EINT: c_uint = 0x0100  /* FLL_LOCK_EINT */;
pub const WM8993_FLL_LOCK_EINT_MASK: c_uint = 0x0100  /* FLL_LOCK_EINT */;

pub const WM8993_GPI8_EINT: c_uint = 0x0080  /* GPI8_EINT */;
pub const WM8993_GPI8_EINT_MASK: c_uint = 0x0080  /* GPI8_EINT */;

pub const WM8993_GPI7_EINT: c_uint = 0x0040  /* GPI7_EINT */;
pub const WM8993_GPI7_EINT_MASK: c_uint = 0x0040  /* GPI7_EINT */;

pub const WM8993_GPIO1_EINT: c_uint = 0x0001  /* GPIO1_EINT */;
pub const WM8993_GPIO1_EINT_MASK: c_uint = 0x0001  /* GPIO1_EINT */;

//
// R19 (0x13) - GPIO1
//
pub const WM8993_GPIO1_PU: c_uint = 0x0020  /* GPIO1_PU */;
pub const WM8993_GPIO1_PU_MASK: c_uint = 0x0020  /* GPIO1_PU */;

pub const WM8993_GPIO1_PD: c_uint = 0x0010  /* GPIO1_PD */;
pub const WM8993_GPIO1_PD_MASK: c_uint = 0x0010  /* GPIO1_PD */;

pub const WM8993_GPIO1_SEL_MASK: c_uint = 0x000F  /* GPIO1_SEL - [3:0] */;

//
// R20 (0x14) - IRQ_DEBOUNCE
//
pub const WM8993_JD2_SC_DB: c_uint = 0x8000  /* JD2_SC_DB */;
pub const WM8993_JD2_SC_DB_MASK: c_uint = 0x8000  /* JD2_SC_DB */;

pub const WM8993_JD2_DB: c_uint = 0x4000  /* JD2_DB */;
pub const WM8993_JD2_DB_MASK: c_uint = 0x4000  /* JD2_DB */;

pub const WM8993_WSEQ_DB: c_uint = 0x2000  /* WSEQ_DB */;
pub const WM8993_WSEQ_DB_MASK: c_uint = 0x2000  /* WSEQ_DB */;

pub const WM8993_TEMPOK_DB: c_uint = 0x0800  /* TEMPOK_DB */;
pub const WM8993_TEMPOK_DB_MASK: c_uint = 0x0800  /* TEMPOK_DB */;

pub const WM8993_JD1_SC_DB: c_uint = 0x0400  /* JD1_SC_DB */;
pub const WM8993_JD1_SC_DB_MASK: c_uint = 0x0400  /* JD1_SC_DB */;

pub const WM8993_JD1_DB: c_uint = 0x0200  /* JD1_DB */;
pub const WM8993_JD1_DB_MASK: c_uint = 0x0200  /* JD1_DB */;

pub const WM8993_FLL_LOCK_DB: c_uint = 0x0100  /* FLL_LOCK_DB */;
pub const WM8993_FLL_LOCK_DB_MASK: c_uint = 0x0100  /* FLL_LOCK_DB */;

pub const WM8993_GPI8_DB: c_uint = 0x0080  /* GPI8_DB */;
pub const WM8993_GPI8_DB_MASK: c_uint = 0x0080  /* GPI8_DB */;

pub const WM8993_GPI7_DB: c_uint = 0x0008  /* GPI7_DB */;
pub const WM8993_GPI7_DB_MASK: c_uint = 0x0008  /* GPI7_DB */;

pub const WM8993_GPIO1_DB: c_uint = 0x0001  /* GPIO1_DB */;
pub const WM8993_GPIO1_DB_MASK: c_uint = 0x0001  /* GPIO1_DB */;

//
// R21 (0x15) - Inputs Clamp
//
pub const WM8993_INPUTS_CLAMP: c_uint = 0x0040  /* INPUTS_CLAMP */;
pub const WM8993_INPUTS_CLAMP_MASK: c_uint = 0x0040  /* INPUTS_CLAMP */;

//
// R22 (0x16) - GPIOCTRL 2
//
pub const WM8993_IM_JD2_EINT: c_uint = 0x2000  /* IM_JD2_EINT */;
pub const WM8993_IM_JD2_EINT_MASK: c_uint = 0x2000  /* IM_JD2_EINT */;

pub const WM8993_IM_JD2_SC_EINT: c_uint = 0x1000  /* IM_JD2_SC_EINT */;
pub const WM8993_IM_JD2_SC_EINT_MASK: c_uint = 0x1000  /* IM_JD2_SC_EINT */;

pub const WM8993_IM_TEMPOK_EINT: c_uint = 0x0800  /* IM_TEMPOK_EINT */;
pub const WM8993_IM_TEMPOK_EINT_MASK: c_uint = 0x0800  /* IM_TEMPOK_EINT */;

pub const WM8993_IM_JD1_SC_EINT: c_uint = 0x0400  /* IM_JD1_SC_EINT */;
pub const WM8993_IM_JD1_SC_EINT_MASK: c_uint = 0x0400  /* IM_JD1_SC_EINT */;

pub const WM8993_IM_JD1_EINT: c_uint = 0x0200  /* IM_JD1_EINT */;
pub const WM8993_IM_JD1_EINT_MASK: c_uint = 0x0200  /* IM_JD1_EINT */;

pub const WM8993_IM_FLL_LOCK_EINT: c_uint = 0x0100  /* IM_FLL_LOCK_EINT */;
pub const WM8993_IM_FLL_LOCK_EINT_MASK: c_uint = 0x0100  /* IM_FLL_LOCK_EINT */;

pub const WM8993_IM_GPI8_EINT: c_uint = 0x0040  /* IM_GPI8_EINT */;
pub const WM8993_IM_GPI8_EINT_MASK: c_uint = 0x0040  /* IM_GPI8_EINT */;

pub const WM8993_IM_GPIO1_EINT: c_uint = 0x0020  /* IM_GPIO1_EINT */;
pub const WM8993_IM_GPIO1_EINT_MASK: c_uint = 0x0020  /* IM_GPIO1_EINT */;

pub const WM8993_GPI8_ENA: c_uint = 0x0010  /* GPI8_ENA */;
pub const WM8993_GPI8_ENA_MASK: c_uint = 0x0010  /* GPI8_ENA */;

pub const WM8993_IM_GPI7_EINT: c_uint = 0x0004  /* IM_GPI7_EINT */;
pub const WM8993_IM_GPI7_EINT_MASK: c_uint = 0x0004  /* IM_GPI7_EINT */;

pub const WM8993_IM_WSEQ_EINT: c_uint = 0x0002  /* IM_WSEQ_EINT */;
pub const WM8993_IM_WSEQ_EINT_MASK: c_uint = 0x0002  /* IM_WSEQ_EINT */;

pub const WM8993_GPI7_ENA: c_uint = 0x0001  /* GPI7_ENA */;
pub const WM8993_GPI7_ENA_MASK: c_uint = 0x0001  /* GPI7_ENA */;

//
// R23 (0x17) - GPIO_POL
//
pub const WM8993_JD2_SC_POL: c_uint = 0x8000  /* JD2_SC_POL */;
pub const WM8993_JD2_SC_POL_MASK: c_uint = 0x8000  /* JD2_SC_POL */;

pub const WM8993_JD2_POL: c_uint = 0x4000  /* JD2_POL */;
pub const WM8993_JD2_POL_MASK: c_uint = 0x4000  /* JD2_POL */;

pub const WM8993_WSEQ_POL: c_uint = 0x2000  /* WSEQ_POL */;
pub const WM8993_WSEQ_POL_MASK: c_uint = 0x2000  /* WSEQ_POL */;

pub const WM8993_IRQ_POL: c_uint = 0x1000  /* IRQ_POL */;
pub const WM8993_IRQ_POL_MASK: c_uint = 0x1000  /* IRQ_POL */;

pub const WM8993_TEMPOK_POL: c_uint = 0x0800  /* TEMPOK_POL */;
pub const WM8993_TEMPOK_POL_MASK: c_uint = 0x0800  /* TEMPOK_POL */;

pub const WM8993_JD1_SC_POL: c_uint = 0x0400  /* JD1_SC_POL */;
pub const WM8993_JD1_SC_POL_MASK: c_uint = 0x0400  /* JD1_SC_POL */;

pub const WM8993_JD1_POL: c_uint = 0x0200  /* JD1_POL */;
pub const WM8993_JD1_POL_MASK: c_uint = 0x0200  /* JD1_POL */;

pub const WM8993_FLL_LOCK_POL: c_uint = 0x0100  /* FLL_LOCK_POL */;
pub const WM8993_FLL_LOCK_POL_MASK: c_uint = 0x0100  /* FLL_LOCK_POL */;

pub const WM8993_GPI8_POL: c_uint = 0x0080  /* GPI8_POL */;
pub const WM8993_GPI8_POL_MASK: c_uint = 0x0080  /* GPI8_POL */;

pub const WM8993_GPI7_POL: c_uint = 0x0040  /* GPI7_POL */;
pub const WM8993_GPI7_POL_MASK: c_uint = 0x0040  /* GPI7_POL */;

pub const WM8993_GPIO1_POL: c_uint = 0x0001  /* GPIO1_POL */;
pub const WM8993_GPIO1_POL_MASK: c_uint = 0x0001  /* GPIO1_POL */;

//
// R24 (0x18) - Left Line Input 1&2 Volume
//
pub const WM8993_IN1_VU: c_uint = 0x0100  /* IN1_VU */;
pub const WM8993_IN1_VU_MASK: c_uint = 0x0100  /* IN1_VU */;

pub const WM8993_IN1L_MUTE: c_uint = 0x0080  /* IN1L_MUTE */;
pub const WM8993_IN1L_MUTE_MASK: c_uint = 0x0080  /* IN1L_MUTE */;

pub const WM8993_IN1L_ZC: c_uint = 0x0040  /* IN1L_ZC */;
pub const WM8993_IN1L_ZC_MASK: c_uint = 0x0040  /* IN1L_ZC */;

pub const WM8993_IN1L_VOL_MASK: c_uint = 0x001F  /* IN1L_VOL - [4:0] */;

//
// R25 (0x19) - Left Line Input 3&4 Volume
//
pub const WM8993_IN2_VU: c_uint = 0x0100  /* IN2_VU */;
pub const WM8993_IN2_VU_MASK: c_uint = 0x0100  /* IN2_VU */;

pub const WM8993_IN2L_MUTE: c_uint = 0x0080  /* IN2L_MUTE */;
pub const WM8993_IN2L_MUTE_MASK: c_uint = 0x0080  /* IN2L_MUTE */;

pub const WM8993_IN2L_ZC: c_uint = 0x0040  /* IN2L_ZC */;
pub const WM8993_IN2L_ZC_MASK: c_uint = 0x0040  /* IN2L_ZC */;

pub const WM8993_IN2L_VOL_MASK: c_uint = 0x001F  /* IN2L_VOL - [4:0] */;

//
// R26 (0x1A) - Right Line Input 1&2 Volume
//
pub const WM8993_IN1_VU: c_uint = 0x0100  /* IN1_VU */;
pub const WM8993_IN1_VU_MASK: c_uint = 0x0100  /* IN1_VU */;

pub const WM8993_IN1R_MUTE: c_uint = 0x0080  /* IN1R_MUTE */;
pub const WM8993_IN1R_MUTE_MASK: c_uint = 0x0080  /* IN1R_MUTE */;

pub const WM8993_IN1R_ZC: c_uint = 0x0040  /* IN1R_ZC */;
pub const WM8993_IN1R_ZC_MASK: c_uint = 0x0040  /* IN1R_ZC */;

pub const WM8993_IN1R_VOL_MASK: c_uint = 0x001F  /* IN1R_VOL - [4:0] */;

//
// R27 (0x1B) - Right Line Input 3&4 Volume
//
pub const WM8993_IN2_VU: c_uint = 0x0100  /* IN2_VU */;
pub const WM8993_IN2_VU_MASK: c_uint = 0x0100  /* IN2_VU */;

pub const WM8993_IN2R_MUTE: c_uint = 0x0080  /* IN2R_MUTE */;
pub const WM8993_IN2R_MUTE_MASK: c_uint = 0x0080  /* IN2R_MUTE */;

pub const WM8993_IN2R_ZC: c_uint = 0x0040  /* IN2R_ZC */;
pub const WM8993_IN2R_ZC_MASK: c_uint = 0x0040  /* IN2R_ZC */;

pub const WM8993_IN2R_VOL_MASK: c_uint = 0x001F  /* IN2R_VOL - [4:0] */;

//
// R28 (0x1C) - Left Output Volume
//
pub const WM8993_HPOUT1_VU: c_uint = 0x0100  /* HPOUT1_VU */;
pub const WM8993_HPOUT1_VU_MASK: c_uint = 0x0100  /* HPOUT1_VU */;

pub const WM8993_HPOUT1L_ZC: c_uint = 0x0080  /* HPOUT1L_ZC */;
pub const WM8993_HPOUT1L_ZC_MASK: c_uint = 0x0080  /* HPOUT1L_ZC */;

pub const WM8993_HPOUT1L_MUTE_N: c_uint = 0x0040  /* HPOUT1L_MUTE_N */;
pub const WM8993_HPOUT1L_MUTE_N_MASK: c_uint = 0x0040  /* HPOUT1L_MUTE_N */;

pub const WM8993_HPOUT1L_VOL_MASK: c_uint = 0x003F  /* HPOUT1L_VOL - [5:0] */;

//
// R29 (0x1D) - Right Output Volume
//
pub const WM8993_HPOUT1_VU: c_uint = 0x0100  /* HPOUT1_VU */;
pub const WM8993_HPOUT1_VU_MASK: c_uint = 0x0100  /* HPOUT1_VU */;

pub const WM8993_HPOUT1R_ZC: c_uint = 0x0080  /* HPOUT1R_ZC */;
pub const WM8993_HPOUT1R_ZC_MASK: c_uint = 0x0080  /* HPOUT1R_ZC */;

pub const WM8993_HPOUT1R_MUTE_N: c_uint = 0x0040  /* HPOUT1R_MUTE_N */;
pub const WM8993_HPOUT1R_MUTE_N_MASK: c_uint = 0x0040  /* HPOUT1R_MUTE_N */;

pub const WM8993_HPOUT1R_VOL_MASK: c_uint = 0x003F  /* HPOUT1R_VOL - [5:0] */;

//
// R30 (0x1E) - Line Outputs Volume
//
pub const WM8993_LINEOUT1N_MUTE: c_uint = 0x0040  /* LINEOUT1N_MUTE */;
pub const WM8993_LINEOUT1N_MUTE_MASK: c_uint = 0x0040  /* LINEOUT1N_MUTE */;

pub const WM8993_LINEOUT1P_MUTE: c_uint = 0x0020  /* LINEOUT1P_MUTE */;
pub const WM8993_LINEOUT1P_MUTE_MASK: c_uint = 0x0020  /* LINEOUT1P_MUTE */;

pub const WM8993_LINEOUT1_VOL: c_uint = 0x0010  /* LINEOUT1_VOL */;
pub const WM8993_LINEOUT1_VOL_MASK: c_uint = 0x0010  /* LINEOUT1_VOL */;

pub const WM8993_LINEOUT2N_MUTE: c_uint = 0x0004  /* LINEOUT2N_MUTE */;
pub const WM8993_LINEOUT2N_MUTE_MASK: c_uint = 0x0004  /* LINEOUT2N_MUTE */;

pub const WM8993_LINEOUT2P_MUTE: c_uint = 0x0002  /* LINEOUT2P_MUTE */;
pub const WM8993_LINEOUT2P_MUTE_MASK: c_uint = 0x0002  /* LINEOUT2P_MUTE */;

pub const WM8993_LINEOUT2_VOL: c_uint = 0x0001  /* LINEOUT2_VOL */;
pub const WM8993_LINEOUT2_VOL_MASK: c_uint = 0x0001  /* LINEOUT2_VOL */;

//
// R31 (0x1F) - HPOUT2 Volume
//
pub const WM8993_HPOUT2_MUTE: c_uint = 0x0020  /* HPOUT2_MUTE */;
pub const WM8993_HPOUT2_MUTE_MASK: c_uint = 0x0020  /* HPOUT2_MUTE */;

pub const WM8993_HPOUT2_VOL: c_uint = 0x0010  /* HPOUT2_VOL */;
pub const WM8993_HPOUT2_VOL_MASK: c_uint = 0x0010  /* HPOUT2_VOL */;

//
// R32 (0x20) - Left OPGA Volume
//
pub const WM8993_MIXOUT_VU: c_uint = 0x0100  /* MIXOUT_VU */;
pub const WM8993_MIXOUT_VU_MASK: c_uint = 0x0100  /* MIXOUT_VU */;

pub const WM8993_MIXOUTL_ZC: c_uint = 0x0080  /* MIXOUTL_ZC */;
pub const WM8993_MIXOUTL_ZC_MASK: c_uint = 0x0080  /* MIXOUTL_ZC */;

pub const WM8993_MIXOUTL_MUTE_N: c_uint = 0x0040  /* MIXOUTL_MUTE_N */;
pub const WM8993_MIXOUTL_MUTE_N_MASK: c_uint = 0x0040  /* MIXOUTL_MUTE_N */;

pub const WM8993_MIXOUTL_VOL_MASK: c_uint = 0x003F  /* MIXOUTL_VOL - [5:0] */;

//
// R33 (0x21) - Right OPGA Volume
//
pub const WM8993_MIXOUT_VU: c_uint = 0x0100  /* MIXOUT_VU */;
pub const WM8993_MIXOUT_VU_MASK: c_uint = 0x0100  /* MIXOUT_VU */;

pub const WM8993_MIXOUTR_ZC: c_uint = 0x0080  /* MIXOUTR_ZC */;
pub const WM8993_MIXOUTR_ZC_MASK: c_uint = 0x0080  /* MIXOUTR_ZC */;

pub const WM8993_MIXOUTR_MUTE_N: c_uint = 0x0040  /* MIXOUTR_MUTE_N */;
pub const WM8993_MIXOUTR_MUTE_N_MASK: c_uint = 0x0040  /* MIXOUTR_MUTE_N */;

pub const WM8993_MIXOUTR_VOL_MASK: c_uint = 0x003F  /* MIXOUTR_VOL - [5:0] */;

//
// R34 (0x22) - SPKMIXL Attenuation
//
pub const WM8993_MIXINL_SPKMIXL_VOL: c_uint = 0x0020  /* MIXINL_SPKMIXL_VOL */;
pub const WM8993_MIXINL_SPKMIXL_VOL_MASK: c_uint = 0x0020  /* MIXINL_SPKMIXL_VOL */;

pub const WM8993_IN1LP_SPKMIXL_VOL: c_uint = 0x0010  /* IN1LP_SPKMIXL_VOL */;
pub const WM8993_IN1LP_SPKMIXL_VOL_MASK: c_uint = 0x0010  /* IN1LP_SPKMIXL_VOL */;

pub const WM8993_MIXOUTL_SPKMIXL_VOL: c_uint = 0x0008  /* MIXOUTL_SPKMIXL_VOL */;
pub const WM8993_MIXOUTL_SPKMIXL_VOL_MASK: c_uint = 0x0008  /* MIXOUTL_SPKMIXL_VOL */;

pub const WM8993_DACL_SPKMIXL_VOL: c_uint = 0x0004  /* DACL_SPKMIXL_VOL */;
pub const WM8993_DACL_SPKMIXL_VOL_MASK: c_uint = 0x0004  /* DACL_SPKMIXL_VOL */;

pub const WM8993_SPKMIXL_VOL_MASK: c_uint = 0x0003  /* SPKMIXL_VOL - [1:0] */;

//
// R35 (0x23) - SPKMIXR Attenuation
//
pub const WM8993_SPKOUT_CLASSAB_MODE: c_uint = 0x0100  /* SPKOUT_CLASSAB_MODE */;
pub const WM8993_SPKOUT_CLASSAB_MODE_MASK: c_uint = 0x0100  /* SPKOUT_CLASSAB_MODE */;

pub const WM8993_MIXINR_SPKMIXR_VOL: c_uint = 0x0020  /* MIXINR_SPKMIXR_VOL */;
pub const WM8993_MIXINR_SPKMIXR_VOL_MASK: c_uint = 0x0020  /* MIXINR_SPKMIXR_VOL */;

pub const WM8993_IN1RP_SPKMIXR_VOL: c_uint = 0x0010  /* IN1RP_SPKMIXR_VOL */;
pub const WM8993_IN1RP_SPKMIXR_VOL_MASK: c_uint = 0x0010  /* IN1RP_SPKMIXR_VOL */;

pub const WM8993_MIXOUTR_SPKMIXR_VOL: c_uint = 0x0008  /* MIXOUTR_SPKMIXR_VOL */;
pub const WM8993_MIXOUTR_SPKMIXR_VOL_MASK: c_uint = 0x0008  /* MIXOUTR_SPKMIXR_VOL */;

pub const WM8993_DACR_SPKMIXR_VOL: c_uint = 0x0004  /* DACR_SPKMIXR_VOL */;
pub const WM8993_DACR_SPKMIXR_VOL_MASK: c_uint = 0x0004  /* DACR_SPKMIXR_VOL */;

pub const WM8993_SPKMIXR_VOL_MASK: c_uint = 0x0003  /* SPKMIXR_VOL - [1:0] */;

//
// R36 (0x24) - SPKOUT Mixers
//
pub const WM8993_VRX_TO_SPKOUTL: c_uint = 0x0020  /* VRX_TO_SPKOUTL */;
pub const WM8993_VRX_TO_SPKOUTL_MASK: c_uint = 0x0020  /* VRX_TO_SPKOUTL */;

pub const WM8993_SPKMIXL_TO_SPKOUTL: c_uint = 0x0010  /* SPKMIXL_TO_SPKOUTL */;
pub const WM8993_SPKMIXL_TO_SPKOUTL_MASK: c_uint = 0x0010  /* SPKMIXL_TO_SPKOUTL */;

pub const WM8993_SPKMIXR_TO_SPKOUTL: c_uint = 0x0008  /* SPKMIXR_TO_SPKOUTL */;
pub const WM8993_SPKMIXR_TO_SPKOUTL_MASK: c_uint = 0x0008  /* SPKMIXR_TO_SPKOUTL */;

pub const WM8993_VRX_TO_SPKOUTR: c_uint = 0x0004  /* VRX_TO_SPKOUTR */;
pub const WM8993_VRX_TO_SPKOUTR_MASK: c_uint = 0x0004  /* VRX_TO_SPKOUTR */;

pub const WM8993_SPKMIXL_TO_SPKOUTR: c_uint = 0x0002  /* SPKMIXL_TO_SPKOUTR */;
pub const WM8993_SPKMIXL_TO_SPKOUTR_MASK: c_uint = 0x0002  /* SPKMIXL_TO_SPKOUTR */;

pub const WM8993_SPKMIXR_TO_SPKOUTR: c_uint = 0x0001  /* SPKMIXR_TO_SPKOUTR */;
pub const WM8993_SPKMIXR_TO_SPKOUTR_MASK: c_uint = 0x0001  /* SPKMIXR_TO_SPKOUTR */;

//
// R37 (0x25) - SPKOUT Boost
//
pub const WM8993_SPKOUTL_BOOST_MASK: c_uint = 0x0038  /* SPKOUTL_BOOST - [5:3] */;

pub const WM8993_SPKOUTR_BOOST_MASK: c_uint = 0x0007  /* SPKOUTR_BOOST - [2:0] */;

//
// R38 (0x26) - Speaker Volume Left
//
pub const WM8993_SPKOUT_VU: c_uint = 0x0100  /* SPKOUT_VU */;
pub const WM8993_SPKOUT_VU_MASK: c_uint = 0x0100  /* SPKOUT_VU */;

pub const WM8993_SPKOUTL_ZC: c_uint = 0x0080  /* SPKOUTL_ZC */;
pub const WM8993_SPKOUTL_ZC_MASK: c_uint = 0x0080  /* SPKOUTL_ZC */;

pub const WM8993_SPKOUTL_MUTE_N: c_uint = 0x0040  /* SPKOUTL_MUTE_N */;
pub const WM8993_SPKOUTL_MUTE_N_MASK: c_uint = 0x0040  /* SPKOUTL_MUTE_N */;

pub const WM8993_SPKOUTL_VOL_MASK: c_uint = 0x003F  /* SPKOUTL_VOL - [5:0] */;

//
// R39 (0x27) - Speaker Volume Right
//
pub const WM8993_SPKOUT_VU: c_uint = 0x0100  /* SPKOUT_VU */;
pub const WM8993_SPKOUT_VU_MASK: c_uint = 0x0100  /* SPKOUT_VU */;

pub const WM8993_SPKOUTR_ZC: c_uint = 0x0080  /* SPKOUTR_ZC */;
pub const WM8993_SPKOUTR_ZC_MASK: c_uint = 0x0080  /* SPKOUTR_ZC */;

pub const WM8993_SPKOUTR_MUTE_N: c_uint = 0x0040  /* SPKOUTR_MUTE_N */;
pub const WM8993_SPKOUTR_MUTE_N_MASK: c_uint = 0x0040  /* SPKOUTR_MUTE_N */;

pub const WM8993_SPKOUTR_VOL_MASK: c_uint = 0x003F  /* SPKOUTR_VOL - [5:0] */;

//
// R40 (0x28) - Input Mixer2
//
pub const WM8993_IN2LP_TO_IN2L: c_uint = 0x0080  /* IN2LP_TO_IN2L */;
pub const WM8993_IN2LP_TO_IN2L_MASK: c_uint = 0x0080  /* IN2LP_TO_IN2L */;

pub const WM8993_IN2LN_TO_IN2L: c_uint = 0x0040  /* IN2LN_TO_IN2L */;
pub const WM8993_IN2LN_TO_IN2L_MASK: c_uint = 0x0040  /* IN2LN_TO_IN2L */;

pub const WM8993_IN1LP_TO_IN1L: c_uint = 0x0020  /* IN1LP_TO_IN1L */;
pub const WM8993_IN1LP_TO_IN1L_MASK: c_uint = 0x0020  /* IN1LP_TO_IN1L */;

pub const WM8993_IN1LN_TO_IN1L: c_uint = 0x0010  /* IN1LN_TO_IN1L */;
pub const WM8993_IN1LN_TO_IN1L_MASK: c_uint = 0x0010  /* IN1LN_TO_IN1L */;

pub const WM8993_IN2RP_TO_IN2R: c_uint = 0x0008  /* IN2RP_TO_IN2R */;
pub const WM8993_IN2RP_TO_IN2R_MASK: c_uint = 0x0008  /* IN2RP_TO_IN2R */;

pub const WM8993_IN2RN_TO_IN2R: c_uint = 0x0004  /* IN2RN_TO_IN2R */;
pub const WM8993_IN2RN_TO_IN2R_MASK: c_uint = 0x0004  /* IN2RN_TO_IN2R */;

pub const WM8993_IN1RP_TO_IN1R: c_uint = 0x0002  /* IN1RP_TO_IN1R */;
pub const WM8993_IN1RP_TO_IN1R_MASK: c_uint = 0x0002  /* IN1RP_TO_IN1R */;

pub const WM8993_IN1RN_TO_IN1R: c_uint = 0x0001  /* IN1RN_TO_IN1R */;
pub const WM8993_IN1RN_TO_IN1R_MASK: c_uint = 0x0001  /* IN1RN_TO_IN1R */;

//
// R41 (0x29) - Input Mixer3
//
pub const WM8993_IN2L_TO_MIXINL: c_uint = 0x0100  /* IN2L_TO_MIXINL */;
pub const WM8993_IN2L_TO_MIXINL_MASK: c_uint = 0x0100  /* IN2L_TO_MIXINL */;

pub const WM8993_IN2L_MIXINL_VOL: c_uint = 0x0080  /* IN2L_MIXINL_VOL */;
pub const WM8993_IN2L_MIXINL_VOL_MASK: c_uint = 0x0080  /* IN2L_MIXINL_VOL */;

pub const WM8993_IN1L_TO_MIXINL: c_uint = 0x0020  /* IN1L_TO_MIXINL */;
pub const WM8993_IN1L_TO_MIXINL_MASK: c_uint = 0x0020  /* IN1L_TO_MIXINL */;

pub const WM8993_IN1L_MIXINL_VOL: c_uint = 0x0010  /* IN1L_MIXINL_VOL */;
pub const WM8993_IN1L_MIXINL_VOL_MASK: c_uint = 0x0010  /* IN1L_MIXINL_VOL */;

pub const WM8993_MIXOUTL_MIXINL_VOL_MASK: c_uint = 0x0007  /* MIXOUTL_MIXINL_VOL - [2:0] */;

//
// R42 (0x2A) - Input Mixer4
//
pub const WM8993_IN2R_TO_MIXINR: c_uint = 0x0100  /* IN2R_TO_MIXINR */;
pub const WM8993_IN2R_TO_MIXINR_MASK: c_uint = 0x0100  /* IN2R_TO_MIXINR */;

pub const WM8993_IN2R_MIXINR_VOL: c_uint = 0x0080  /* IN2R_MIXINR_VOL */;
pub const WM8993_IN2R_MIXINR_VOL_MASK: c_uint = 0x0080  /* IN2R_MIXINR_VOL */;

pub const WM8993_IN1R_TO_MIXINR: c_uint = 0x0020  /* IN1R_TO_MIXINR */;
pub const WM8993_IN1R_TO_MIXINR_MASK: c_uint = 0x0020  /* IN1R_TO_MIXINR */;

pub const WM8993_IN1R_MIXINR_VOL: c_uint = 0x0010  /* IN1R_MIXINR_VOL */;
pub const WM8993_IN1R_MIXINR_VOL_MASK: c_uint = 0x0010  /* IN1R_MIXINR_VOL */;

pub const WM8993_MIXOUTR_MIXINR_VOL_MASK: c_uint = 0x0007  /* MIXOUTR_MIXINR_VOL - [2:0] */;

//
// R43 (0x2B) - Input Mixer5
//
pub const WM8993_IN1LP_MIXINL_VOL_MASK: c_uint = 0x01C0  /* IN1LP_MIXINL_VOL - [8:6] */;

pub const WM8993_VRX_MIXINL_VOL_MASK: c_uint = 0x0007  /* VRX_MIXINL_VOL - [2:0] */;

//
// R44 (0x2C) - Input Mixer6
//
pub const WM8993_IN1RP_MIXINR_VOL_MASK: c_uint = 0x01C0  /* IN1RP_MIXINR_VOL - [8:6] */;

pub const WM8993_VRX_MIXINR_VOL_MASK: c_uint = 0x0007  /* VRX_MIXINR_VOL - [2:0] */;

//
// R45 (0x2D) - Output Mixer1
//
pub const WM8993_DACL_TO_HPOUT1L: c_uint = 0x0100  /* DACL_TO_HPOUT1L */;
pub const WM8993_DACL_TO_HPOUT1L_MASK: c_uint = 0x0100  /* DACL_TO_HPOUT1L */;

pub const WM8993_MIXINR_TO_MIXOUTL: c_uint = 0x0080  /* MIXINR_TO_MIXOUTL */;
pub const WM8993_MIXINR_TO_MIXOUTL_MASK: c_uint = 0x0080  /* MIXINR_TO_MIXOUTL */;

pub const WM8993_MIXINL_TO_MIXOUTL: c_uint = 0x0040  /* MIXINL_TO_MIXOUTL */;
pub const WM8993_MIXINL_TO_MIXOUTL_MASK: c_uint = 0x0040  /* MIXINL_TO_MIXOUTL */;

pub const WM8993_IN2RN_TO_MIXOUTL: c_uint = 0x0020  /* IN2RN_TO_MIXOUTL */;
pub const WM8993_IN2RN_TO_MIXOUTL_MASK: c_uint = 0x0020  /* IN2RN_TO_MIXOUTL */;

pub const WM8993_IN2LN_TO_MIXOUTL: c_uint = 0x0010  /* IN2LN_TO_MIXOUTL */;
pub const WM8993_IN2LN_TO_MIXOUTL_MASK: c_uint = 0x0010  /* IN2LN_TO_MIXOUTL */;

pub const WM8993_IN1R_TO_MIXOUTL: c_uint = 0x0008  /* IN1R_TO_MIXOUTL */;
pub const WM8993_IN1R_TO_MIXOUTL_MASK: c_uint = 0x0008  /* IN1R_TO_MIXOUTL */;

pub const WM8993_IN1L_TO_MIXOUTL: c_uint = 0x0004  /* IN1L_TO_MIXOUTL */;
pub const WM8993_IN1L_TO_MIXOUTL_MASK: c_uint = 0x0004  /* IN1L_TO_MIXOUTL */;

pub const WM8993_IN2LP_TO_MIXOUTL: c_uint = 0x0002  /* IN2LP_TO_MIXOUTL */;
pub const WM8993_IN2LP_TO_MIXOUTL_MASK: c_uint = 0x0002  /* IN2LP_TO_MIXOUTL */;

pub const WM8993_DACL_TO_MIXOUTL: c_uint = 0x0001  /* DACL_TO_MIXOUTL */;
pub const WM8993_DACL_TO_MIXOUTL_MASK: c_uint = 0x0001  /* DACL_TO_MIXOUTL */;

//
// R46 (0x2E) - Output Mixer2
//
pub const WM8993_DACR_TO_HPOUT1R: c_uint = 0x0100  /* DACR_TO_HPOUT1R */;
pub const WM8993_DACR_TO_HPOUT1R_MASK: c_uint = 0x0100  /* DACR_TO_HPOUT1R */;

pub const WM8993_MIXINL_TO_MIXOUTR: c_uint = 0x0080  /* MIXINL_TO_MIXOUTR */;
pub const WM8993_MIXINL_TO_MIXOUTR_MASK: c_uint = 0x0080  /* MIXINL_TO_MIXOUTR */;

pub const WM8993_MIXINR_TO_MIXOUTR: c_uint = 0x0040  /* MIXINR_TO_MIXOUTR */;
pub const WM8993_MIXINR_TO_MIXOUTR_MASK: c_uint = 0x0040  /* MIXINR_TO_MIXOUTR */;

pub const WM8993_IN2LN_TO_MIXOUTR: c_uint = 0x0020  /* IN2LN_TO_MIXOUTR */;
pub const WM8993_IN2LN_TO_MIXOUTR_MASK: c_uint = 0x0020  /* IN2LN_TO_MIXOUTR */;

pub const WM8993_IN2RN_TO_MIXOUTR: c_uint = 0x0010  /* IN2RN_TO_MIXOUTR */;
pub const WM8993_IN2RN_TO_MIXOUTR_MASK: c_uint = 0x0010  /* IN2RN_TO_MIXOUTR */;

pub const WM8993_IN1L_TO_MIXOUTR: c_uint = 0x0008  /* IN1L_TO_MIXOUTR */;
pub const WM8993_IN1L_TO_MIXOUTR_MASK: c_uint = 0x0008  /* IN1L_TO_MIXOUTR */;

pub const WM8993_IN1R_TO_MIXOUTR: c_uint = 0x0004  /* IN1R_TO_MIXOUTR */;
pub const WM8993_IN1R_TO_MIXOUTR_MASK: c_uint = 0x0004  /* IN1R_TO_MIXOUTR */;

pub const WM8993_IN2RP_TO_MIXOUTR: c_uint = 0x0002  /* IN2RP_TO_MIXOUTR */;
pub const WM8993_IN2RP_TO_MIXOUTR_MASK: c_uint = 0x0002  /* IN2RP_TO_MIXOUTR */;

pub const WM8993_DACR_TO_MIXOUTR: c_uint = 0x0001  /* DACR_TO_MIXOUTR */;
pub const WM8993_DACR_TO_MIXOUTR_MASK: c_uint = 0x0001  /* DACR_TO_MIXOUTR */;

//
// R47 (0x2F) - Output Mixer3
//
pub const WM8993_IN2LP_MIXOUTL_VOL_MASK: c_uint = 0x0E00  /* IN2LP_MIXOUTL_VOL - [11:9] */;

pub const WM8993_IN2LN_MIXOUTL_VOL_MASK: c_uint = 0x01C0  /* IN2LN_MIXOUTL_VOL - [8:6] */;

pub const WM8993_IN1R_MIXOUTL_VOL_MASK: c_uint = 0x0038  /* IN1R_MIXOUTL_VOL - [5:3] */;

pub const WM8993_IN1L_MIXOUTL_VOL_MASK: c_uint = 0x0007  /* IN1L_MIXOUTL_VOL - [2:0] */;

//
// R48 (0x30) - Output Mixer4
//
pub const WM8993_IN2RP_MIXOUTR_VOL_MASK: c_uint = 0x0E00  /* IN2RP_MIXOUTR_VOL - [11:9] */;

pub const WM8993_IN2RN_MIXOUTR_VOL_MASK: c_uint = 0x01C0  /* IN2RN_MIXOUTR_VOL - [8:6] */;

pub const WM8993_IN1L_MIXOUTR_VOL_MASK: c_uint = 0x0038  /* IN1L_MIXOUTR_VOL - [5:3] */;

pub const WM8993_IN1R_MIXOUTR_VOL_MASK: c_uint = 0x0007  /* IN1R_MIXOUTR_VOL - [2:0] */;

//
// R49 (0x31) - Output Mixer5
//
pub const WM8993_DACL_MIXOUTL_VOL_MASK: c_uint = 0x0E00  /* DACL_MIXOUTL_VOL - [11:9] */;

pub const WM8993_IN2RN_MIXOUTL_VOL_MASK: c_uint = 0x01C0  /* IN2RN_MIXOUTL_VOL - [8:6] */;

pub const WM8993_MIXINR_MIXOUTL_VOL_MASK: c_uint = 0x0038  /* MIXINR_MIXOUTL_VOL - [5:3] */;

pub const WM8993_MIXINL_MIXOUTL_VOL_MASK: c_uint = 0x0007  /* MIXINL_MIXOUTL_VOL - [2:0] */;

//
// R50 (0x32) - Output Mixer6
//
pub const WM8993_DACR_MIXOUTR_VOL_MASK: c_uint = 0x0E00  /* DACR_MIXOUTR_VOL - [11:9] */;

pub const WM8993_IN2LN_MIXOUTR_VOL_MASK: c_uint = 0x01C0  /* IN2LN_MIXOUTR_VOL - [8:6] */;

pub const WM8993_MIXINL_MIXOUTR_VOL_MASK: c_uint = 0x0038  /* MIXINL_MIXOUTR_VOL - [5:3] */;

pub const WM8993_MIXINR_MIXOUTR_VOL_MASK: c_uint = 0x0007  /* MIXINR_MIXOUTR_VOL - [2:0] */;

//
// R51 (0x33) - HPOUT2 Mixer
//
pub const WM8993_VRX_TO_HPOUT2: c_uint = 0x0020  /* VRX_TO_HPOUT2 */;
pub const WM8993_VRX_TO_HPOUT2_MASK: c_uint = 0x0020  /* VRX_TO_HPOUT2 */;

pub const WM8993_MIXOUTLVOL_TO_HPOUT2: c_uint = 0x0010  /* MIXOUTLVOL_TO_HPOUT2 */;
pub const WM8993_MIXOUTLVOL_TO_HPOUT2_MASK: c_uint = 0x0010  /* MIXOUTLVOL_TO_HPOUT2 */;

pub const WM8993_MIXOUTRVOL_TO_HPOUT2: c_uint = 0x0008  /* MIXOUTRVOL_TO_HPOUT2 */;
pub const WM8993_MIXOUTRVOL_TO_HPOUT2_MASK: c_uint = 0x0008  /* MIXOUTRVOL_TO_HPOUT2 */;

//
// R52 (0x34) - Line Mixer1
//
pub const WM8993_MIXOUTL_TO_LINEOUT1N: c_uint = 0x0040  /* MIXOUTL_TO_LINEOUT1N */;
pub const WM8993_MIXOUTL_TO_LINEOUT1N_MASK: c_uint = 0x0040  /* MIXOUTL_TO_LINEOUT1N */;

pub const WM8993_MIXOUTR_TO_LINEOUT1N: c_uint = 0x0020  /* MIXOUTR_TO_LINEOUT1N */;
pub const WM8993_MIXOUTR_TO_LINEOUT1N_MASK: c_uint = 0x0020  /* MIXOUTR_TO_LINEOUT1N */;

pub const WM8993_LINEOUT1_MODE: c_uint = 0x0010  /* LINEOUT1_MODE */;
pub const WM8993_LINEOUT1_MODE_MASK: c_uint = 0x0010  /* LINEOUT1_MODE */;

pub const WM8993_IN1R_TO_LINEOUT1P: c_uint = 0x0004  /* IN1R_TO_LINEOUT1P */;
pub const WM8993_IN1R_TO_LINEOUT1P_MASK: c_uint = 0x0004  /* IN1R_TO_LINEOUT1P */;

pub const WM8993_IN1L_TO_LINEOUT1P: c_uint = 0x0002  /* IN1L_TO_LINEOUT1P */;
pub const WM8993_IN1L_TO_LINEOUT1P_MASK: c_uint = 0x0002  /* IN1L_TO_LINEOUT1P */;

pub const WM8993_MIXOUTL_TO_LINEOUT1P: c_uint = 0x0001  /* MIXOUTL_TO_LINEOUT1P */;
pub const WM8993_MIXOUTL_TO_LINEOUT1P_MASK: c_uint = 0x0001  /* MIXOUTL_TO_LINEOUT1P */;

//
// R53 (0x35) - Line Mixer2
//
pub const WM8993_MIXOUTR_TO_LINEOUT2N: c_uint = 0x0040  /* MIXOUTR_TO_LINEOUT2N */;
pub const WM8993_MIXOUTR_TO_LINEOUT2N_MASK: c_uint = 0x0040  /* MIXOUTR_TO_LINEOUT2N */;

pub const WM8993_MIXOUTL_TO_LINEOUT2N: c_uint = 0x0020  /* MIXOUTL_TO_LINEOUT2N */;
pub const WM8993_MIXOUTL_TO_LINEOUT2N_MASK: c_uint = 0x0020  /* MIXOUTL_TO_LINEOUT2N */;

pub const WM8993_LINEOUT2_MODE: c_uint = 0x0010  /* LINEOUT2_MODE */;
pub const WM8993_LINEOUT2_MODE_MASK: c_uint = 0x0010  /* LINEOUT2_MODE */;

pub const WM8993_IN1L_TO_LINEOUT2P: c_uint = 0x0004  /* IN1L_TO_LINEOUT2P */;
pub const WM8993_IN1L_TO_LINEOUT2P_MASK: c_uint = 0x0004  /* IN1L_TO_LINEOUT2P */;

pub const WM8993_IN1R_TO_LINEOUT2P: c_uint = 0x0002  /* IN1R_TO_LINEOUT2P */;
pub const WM8993_IN1R_TO_LINEOUT2P_MASK: c_uint = 0x0002  /* IN1R_TO_LINEOUT2P */;

pub const WM8993_MIXOUTR_TO_LINEOUT2P: c_uint = 0x0001  /* MIXOUTR_TO_LINEOUT2P */;
pub const WM8993_MIXOUTR_TO_LINEOUT2P_MASK: c_uint = 0x0001  /* MIXOUTR_TO_LINEOUT2P */;

//
// R54 (0x36) - Speaker Mixer
//
pub const WM8993_SPKAB_REF_SEL: c_uint = 0x0100  /* SPKAB_REF_SEL */;
pub const WM8993_SPKAB_REF_SEL_MASK: c_uint = 0x0100  /* SPKAB_REF_SEL */;

pub const WM8993_MIXINL_TO_SPKMIXL: c_uint = 0x0080  /* MIXINL_TO_SPKMIXL */;
pub const WM8993_MIXINL_TO_SPKMIXL_MASK: c_uint = 0x0080  /* MIXINL_TO_SPKMIXL */;

pub const WM8993_MIXINR_TO_SPKMIXR: c_uint = 0x0040  /* MIXINR_TO_SPKMIXR */;
pub const WM8993_MIXINR_TO_SPKMIXR_MASK: c_uint = 0x0040  /* MIXINR_TO_SPKMIXR */;

pub const WM8993_IN1LP_TO_SPKMIXL: c_uint = 0x0020  /* IN1LP_TO_SPKMIXL */;
pub const WM8993_IN1LP_TO_SPKMIXL_MASK: c_uint = 0x0020  /* IN1LP_TO_SPKMIXL */;

pub const WM8993_IN1RP_TO_SPKMIXR: c_uint = 0x0010  /* IN1RP_TO_SPKMIXR */;
pub const WM8993_IN1RP_TO_SPKMIXR_MASK: c_uint = 0x0010  /* IN1RP_TO_SPKMIXR */;

pub const WM8993_MIXOUTL_TO_SPKMIXL: c_uint = 0x0008  /* MIXOUTL_TO_SPKMIXL */;
pub const WM8993_MIXOUTL_TO_SPKMIXL_MASK: c_uint = 0x0008  /* MIXOUTL_TO_SPKMIXL */;

pub const WM8993_MIXOUTR_TO_SPKMIXR: c_uint = 0x0004  /* MIXOUTR_TO_SPKMIXR */;
pub const WM8993_MIXOUTR_TO_SPKMIXR_MASK: c_uint = 0x0004  /* MIXOUTR_TO_SPKMIXR */;

pub const WM8993_DACL_TO_SPKMIXL: c_uint = 0x0002  /* DACL_TO_SPKMIXL */;
pub const WM8993_DACL_TO_SPKMIXL_MASK: c_uint = 0x0002  /* DACL_TO_SPKMIXL */;

pub const WM8993_DACR_TO_SPKMIXR: c_uint = 0x0001  /* DACR_TO_SPKMIXR */;
pub const WM8993_DACR_TO_SPKMIXR_MASK: c_uint = 0x0001  /* DACR_TO_SPKMIXR */;

//
// R55 (0x37) - Additional Control
//
pub const WM8993_LINEOUT1_FB: c_uint = 0x0080  /* LINEOUT1_FB */;
pub const WM8993_LINEOUT1_FB_MASK: c_uint = 0x0080  /* LINEOUT1_FB */;

pub const WM8993_LINEOUT2_FB: c_uint = 0x0040  /* LINEOUT2_FB */;
pub const WM8993_LINEOUT2_FB_MASK: c_uint = 0x0040  /* LINEOUT2_FB */;

pub const WM8993_VROI: c_uint = 0x0001  /* VROI */;
pub const WM8993_VROI_MASK: c_uint = 0x0001  /* VROI */;

//
// R56 (0x38) - AntiPOP1
//
pub const WM8993_LINEOUT_VMID_BUF_ENA: c_uint = 0x0080  /* LINEOUT_VMID_BUF_ENA */;
pub const WM8993_LINEOUT_VMID_BUF_ENA_MASK: c_uint = 0x0080  /* LINEOUT_VMID_BUF_ENA */;

pub const WM8993_HPOUT2_IN_ENA: c_uint = 0x0040  /* HPOUT2_IN_ENA */;
pub const WM8993_HPOUT2_IN_ENA_MASK: c_uint = 0x0040  /* HPOUT2_IN_ENA */;

pub const WM8993_LINEOUT1_DISCH: c_uint = 0x0020  /* LINEOUT1_DISCH */;
pub const WM8993_LINEOUT1_DISCH_MASK: c_uint = 0x0020  /* LINEOUT1_DISCH */;

pub const WM8993_LINEOUT2_DISCH: c_uint = 0x0010  /* LINEOUT2_DISCH */;
pub const WM8993_LINEOUT2_DISCH_MASK: c_uint = 0x0010  /* LINEOUT2_DISCH */;

//
// R57 (0x39) - AntiPOP2
//
pub const WM8993_VMID_RAMP_MASK: c_uint = 0x0060  /* VMID_RAMP - [6:5] */;

pub const WM8993_VMID_BUF_ENA: c_uint = 0x0008  /* VMID_BUF_ENA */;
pub const WM8993_VMID_BUF_ENA_MASK: c_uint = 0x0008  /* VMID_BUF_ENA */;

pub const WM8993_STARTUP_BIAS_ENA: c_uint = 0x0004  /* STARTUP_BIAS_ENA */;
pub const WM8993_STARTUP_BIAS_ENA_MASK: c_uint = 0x0004  /* STARTUP_BIAS_ENA */;

pub const WM8993_BIAS_SRC: c_uint = 0x0002  /* BIAS_SRC */;
pub const WM8993_BIAS_SRC_MASK: c_uint = 0x0002  /* BIAS_SRC */;

pub const WM8993_VMID_DISCH: c_uint = 0x0001  /* VMID_DISCH */;
pub const WM8993_VMID_DISCH_MASK: c_uint = 0x0001  /* VMID_DISCH */;

//
// R58 (0x3A) - MICBIAS
//
pub const WM8993_JD_SCTHR_MASK: c_uint = 0x00C0  /* JD_SCTHR - [7:6] */;

pub const WM8993_JD_THR_MASK: c_uint = 0x0030  /* JD_THR - [5:4] */;

pub const WM8993_JD_ENA: c_uint = 0x0004  /* JD_ENA */;
pub const WM8993_JD_ENA_MASK: c_uint = 0x0004  /* JD_ENA */;

pub const WM8993_MICB2_LVL: c_uint = 0x0002  /* MICB2_LVL */;
pub const WM8993_MICB2_LVL_MASK: c_uint = 0x0002  /* MICB2_LVL */;

pub const WM8993_MICB1_LVL: c_uint = 0x0001  /* MICB1_LVL */;
pub const WM8993_MICB1_LVL_MASK: c_uint = 0x0001  /* MICB1_LVL */;

//
// R60 (0x3C) - FLL Control 1
//
pub const WM8993_FLL_FRAC: c_uint = 0x0004  /* FLL_FRAC */;
pub const WM8993_FLL_FRAC_MASK: c_uint = 0x0004  /* FLL_FRAC */;

pub const WM8993_FLL_OSC_ENA: c_uint = 0x0002  /* FLL_OSC_ENA */;
pub const WM8993_FLL_OSC_ENA_MASK: c_uint = 0x0002  /* FLL_OSC_ENA */;

pub const WM8993_FLL_ENA: c_uint = 0x0001  /* FLL_ENA */;
pub const WM8993_FLL_ENA_MASK: c_uint = 0x0001  /* FLL_ENA */;

//
// R61 (0x3D) - FLL Control 2
//
pub const WM8993_FLL_OUTDIV_MASK: c_uint = 0x0700  /* FLL_OUTDIV - [10:8] */;

pub const WM8993_FLL_CTRL_RATE_MASK: c_uint = 0x0070  /* FLL_CTRL_RATE - [6:4] */;

pub const WM8993_FLL_FRATIO_MASK: c_uint = 0x0007  /* FLL_FRATIO - [2:0] */;

//
// R62 (0x3E) - FLL Control 3
//
pub const WM8993_FLL_K_MASK: c_uint = 0xFFFF  /* FLL_K - [15:0] */;

//
// R63 (0x3F) - FLL Control 4
//
pub const WM8993_FLL_N_MASK: c_uint = 0x7FE0  /* FLL_N - [14:5] */;

pub const WM8993_FLL_GAIN_MASK: c_uint = 0x000F  /* FLL_GAIN - [3:0] */;

//
// R64 (0x40) - FLL Control 5
//
pub const WM8993_FLL_FRC_NCO_VAL_MASK: c_uint = 0x1F80  /* FLL_FRC_NCO_VAL - [12:7] */;

pub const WM8993_FLL_FRC_NCO: c_uint = 0x0040  /* FLL_FRC_NCO */;
pub const WM8993_FLL_FRC_NCO_MASK: c_uint = 0x0040  /* FLL_FRC_NCO */;

pub const WM8993_FLL_CLK_REF_DIV_MASK: c_uint = 0x0018  /* FLL_CLK_REF_DIV - [4:3] */;

pub const WM8993_FLL_CLK_SRC_MASK: c_uint = 0x0003  /* FLL_CLK_SRC - [1:0] */;

//
// R65 (0x41) - Clocking 3
//
pub const WM8993_CLK_DCS_DIV_MASK: c_uint = 0x3C00  /* CLK_DCS_DIV - [13:10] */;

pub const WM8993_SAMPLE_RATE_MASK: c_uint = 0x0380  /* SAMPLE_RATE - [9:7] */;

pub const WM8993_CLK_SYS_RATE_MASK: c_uint = 0x001E  /* CLK_SYS_RATE - [4:1] */;

pub const WM8993_CLK_DSP_ENA: c_uint = 0x0001  /* CLK_DSP_ENA */;
pub const WM8993_CLK_DSP_ENA_MASK: c_uint = 0x0001  /* CLK_DSP_ENA */;

//
// R66 (0x42) - Clocking 4
//
pub const WM8993_DAC_DIV4: c_uint = 0x0200  /* DAC_DIV4 */;
pub const WM8993_DAC_DIV4_MASK: c_uint = 0x0200  /* DAC_DIV4 */;

pub const WM8993_CLK_256K_DIV_MASK: c_uint = 0x007E  /* CLK_256K_DIV - [6:1] */;

pub const WM8993_SR_MODE: c_uint = 0x0001  /* SR_MODE */;
pub const WM8993_SR_MODE_MASK: c_uint = 0x0001  /* SR_MODE */;

//
// R67 (0x43) - MW Slave Control
//
pub const WM8993_MASK_WRITE_ENA: c_uint = 0x0001  /* MASK_WRITE_ENA */;
pub const WM8993_MASK_WRITE_ENA_MASK: c_uint = 0x0001  /* MASK_WRITE_ENA */;

//
// R69 (0x45) - Bus Control 1
//
pub const WM8993_CLK_SYS_ENA: c_uint = 0x0002  /* CLK_SYS_ENA */;
pub const WM8993_CLK_SYS_ENA_MASK: c_uint = 0x0002  /* CLK_SYS_ENA */;

//
// R70 (0x46) - Write Sequencer 0
//
pub const WM8993_WSEQ_ENA: c_uint = 0x0100  /* WSEQ_ENA */;
pub const WM8993_WSEQ_ENA_MASK: c_uint = 0x0100  /* WSEQ_ENA */;

pub const WM8993_WSEQ_WRITE_INDEX_MASK: c_uint = 0x001F  /* WSEQ_WRITE_INDEX - [4:0] */;

//
// R71 (0x47) - Write Sequencer 1
//
pub const WM8993_WSEQ_DATA_WIDTH_MASK: c_uint = 0x7000  /* WSEQ_DATA_WIDTH - [14:12] */;

pub const WM8993_WSEQ_DATA_START_MASK: c_uint = 0x0F00  /* WSEQ_DATA_START - [11:8] */;

pub const WM8993_WSEQ_ADDR_MASK: c_uint = 0x00FF  /* WSEQ_ADDR - [7:0] */;

//
// R72 (0x48) - Write Sequencer 2
//
pub const WM8993_WSEQ_EOS: c_uint = 0x4000  /* WSEQ_EOS */;
pub const WM8993_WSEQ_EOS_MASK: c_uint = 0x4000  /* WSEQ_EOS */;

pub const WM8993_WSEQ_DELAY_MASK: c_uint = 0x0F00  /* WSEQ_DELAY - [11:8] */;

pub const WM8993_WSEQ_DATA_MASK: c_uint = 0x00FF  /* WSEQ_DATA - [7:0] */;

//
// R73 (0x49) - Write Sequencer 3
//
pub const WM8993_WSEQ_ABORT: c_uint = 0x0200  /* WSEQ_ABORT */;
pub const WM8993_WSEQ_ABORT_MASK: c_uint = 0x0200  /* WSEQ_ABORT */;

pub const WM8993_WSEQ_START: c_uint = 0x0100  /* WSEQ_START */;
pub const WM8993_WSEQ_START_MASK: c_uint = 0x0100  /* WSEQ_START */;

pub const WM8993_WSEQ_START_INDEX_MASK: c_uint = 0x003F  /* WSEQ_START_INDEX - [5:0] */;

//
// R74 (0x4A) - Write Sequencer 4
//
pub const WM8993_WSEQ_BUSY: c_uint = 0x0001  /* WSEQ_BUSY */;
pub const WM8993_WSEQ_BUSY_MASK: c_uint = 0x0001  /* WSEQ_BUSY */;

//
// R75 (0x4B) - Write Sequencer 5
//
pub const WM8993_WSEQ_CURRENT_INDEX_MASK: c_uint = 0x003F  /* WSEQ_CURRENT_INDEX - [5:0] */;

//
// R76 (0x4C) - Charge Pump 1
//
pub const WM8993_CP_ENA: c_uint = 0x8000  /* CP_ENA */;
pub const WM8993_CP_ENA_MASK: c_uint = 0x8000  /* CP_ENA */;

//
// R81 (0x51) - Class W 0
//
pub const WM8993_CP_DYN_FREQ: c_uint = 0x0002  /* CP_DYN_FREQ */;
pub const WM8993_CP_DYN_FREQ_MASK: c_uint = 0x0002  /* CP_DYN_FREQ */;

pub const WM8993_CP_DYN_V: c_uint = 0x0001  /* CP_DYN_V */;
pub const WM8993_CP_DYN_V_MASK: c_uint = 0x0001  /* CP_DYN_V */;

//
// R84 (0x54) - DC Servo 0
//
pub const WM8993_DCS_TRIG_SINGLE_1: c_uint = 0x2000  /* DCS_TRIG_SINGLE_1 */;
pub const WM8993_DCS_TRIG_SINGLE_1_MASK: c_uint = 0x2000  /* DCS_TRIG_SINGLE_1 */;

pub const WM8993_DCS_TRIG_SINGLE_0: c_uint = 0x1000  /* DCS_TRIG_SINGLE_0 */;
pub const WM8993_DCS_TRIG_SINGLE_0_MASK: c_uint = 0x1000  /* DCS_TRIG_SINGLE_0 */;

pub const WM8993_DCS_TRIG_SERIES_1: c_uint = 0x0200  /* DCS_TRIG_SERIES_1 */;
pub const WM8993_DCS_TRIG_SERIES_1_MASK: c_uint = 0x0200  /* DCS_TRIG_SERIES_1 */;

pub const WM8993_DCS_TRIG_SERIES_0: c_uint = 0x0100  /* DCS_TRIG_SERIES_0 */;
pub const WM8993_DCS_TRIG_SERIES_0_MASK: c_uint = 0x0100  /* DCS_TRIG_SERIES_0 */;

pub const WM8993_DCS_TRIG_STARTUP_1: c_uint = 0x0020  /* DCS_TRIG_STARTUP_1 */;
pub const WM8993_DCS_TRIG_STARTUP_1_MASK: c_uint = 0x0020  /* DCS_TRIG_STARTUP_1 */;

pub const WM8993_DCS_TRIG_STARTUP_0: c_uint = 0x0010  /* DCS_TRIG_STARTUP_0 */;
pub const WM8993_DCS_TRIG_STARTUP_0_MASK: c_uint = 0x0010  /* DCS_TRIG_STARTUP_0 */;

pub const WM8993_DCS_TRIG_DAC_WR_1: c_uint = 0x0008  /* DCS_TRIG_DAC_WR_1 */;
pub const WM8993_DCS_TRIG_DAC_WR_1_MASK: c_uint = 0x0008  /* DCS_TRIG_DAC_WR_1 */;

pub const WM8993_DCS_TRIG_DAC_WR_0: c_uint = 0x0004  /* DCS_TRIG_DAC_WR_0 */;
pub const WM8993_DCS_TRIG_DAC_WR_0_MASK: c_uint = 0x0004  /* DCS_TRIG_DAC_WR_0 */;

pub const WM8993_DCS_ENA_CHAN_1: c_uint = 0x0002  /* DCS_ENA_CHAN_1 */;
pub const WM8993_DCS_ENA_CHAN_1_MASK: c_uint = 0x0002  /* DCS_ENA_CHAN_1 */;

pub const WM8993_DCS_ENA_CHAN_0: c_uint = 0x0001  /* DCS_ENA_CHAN_0 */;
pub const WM8993_DCS_ENA_CHAN_0_MASK: c_uint = 0x0001  /* DCS_ENA_CHAN_0 */;

//
// R85 (0x55) - DC Servo 1
//
pub const WM8993_DCS_SERIES_NO_01_MASK: c_uint = 0x0FE0  /* DCS_SERIES_NO_01 - [11:5] */;

pub const WM8993_DCS_TIMER_PERIOD_01_MASK: c_uint = 0x000F  /* DCS_TIMER_PERIOD_01 - [3:0] */;

//
// R87 (0x57) - DC Servo 3
//
pub const WM8993_DCS_DAC_WR_VAL_1_MASK: c_uint = 0xFF00  /* DCS_DAC_WR_VAL_1 - [15:8] */;

pub const WM8993_DCS_DAC_WR_VAL_0_MASK: c_uint = 0x00FF  /* DCS_DAC_WR_VAL_0 - [7:0] */;

//
// R88 (0x58) - DC Servo Readback 0
//
pub const WM8993_DCS_DATAPATH_BUSY: c_uint = 0x4000  /* DCS_DATAPATH_BUSY */;
pub const WM8993_DCS_DATAPATH_BUSY_MASK: c_uint = 0x4000  /* DCS_DATAPATH_BUSY */;

pub const WM8993_DCS_CHANNEL_MASK: c_uint = 0x3000  /* DCS_CHANNEL - [13:12] */;

pub const WM8993_DCS_CAL_COMPLETE_MASK: c_uint = 0x0300  /* DCS_CAL_COMPLETE - [9:8] */;

pub const WM8993_DCS_DAC_WR_COMPLETE_MASK: c_uint = 0x0030  /* DCS_DAC_WR_COMPLETE - [5:4] */;

pub const WM8993_DCS_STARTUP_COMPLETE_MASK: c_uint = 0x0003  /* DCS_STARTUP_COMPLETE - [1:0] */;

//
// R89 (0x59) - DC Servo Readback 1
//
pub const WM8993_DCS_INTEG_CHAN_1_MASK: c_uint = 0x00FF  /* DCS_INTEG_CHAN_1 - [7:0] */;

//
// R90 (0x5A) - DC Servo Readback 2
//
pub const WM8993_DCS_INTEG_CHAN_0_MASK: c_uint = 0x00FF  /* DCS_INTEG_CHAN_0 - [7:0] */;

//
// R96 (0x60) - Analogue HP 0
//
pub const WM8993_HPOUT1_AUTO_PU: c_uint = 0x0100  /* HPOUT1_AUTO_PU */;
pub const WM8993_HPOUT1_AUTO_PU_MASK: c_uint = 0x0100  /* HPOUT1_AUTO_PU */;

pub const WM8993_HPOUT1L_RMV_SHORT: c_uint = 0x0080  /* HPOUT1L_RMV_SHORT */;
pub const WM8993_HPOUT1L_RMV_SHORT_MASK: c_uint = 0x0080  /* HPOUT1L_RMV_SHORT */;

pub const WM8993_HPOUT1L_OUTP: c_uint = 0x0040  /* HPOUT1L_OUTP */;
pub const WM8993_HPOUT1L_OUTP_MASK: c_uint = 0x0040  /* HPOUT1L_OUTP */;

pub const WM8993_HPOUT1L_DLY: c_uint = 0x0020  /* HPOUT1L_DLY */;
pub const WM8993_HPOUT1L_DLY_MASK: c_uint = 0x0020  /* HPOUT1L_DLY */;

pub const WM8993_HPOUT1R_RMV_SHORT: c_uint = 0x0008  /* HPOUT1R_RMV_SHORT */;
pub const WM8993_HPOUT1R_RMV_SHORT_MASK: c_uint = 0x0008  /* HPOUT1R_RMV_SHORT */;

pub const WM8993_HPOUT1R_OUTP: c_uint = 0x0004  /* HPOUT1R_OUTP */;
pub const WM8993_HPOUT1R_OUTP_MASK: c_uint = 0x0004  /* HPOUT1R_OUTP */;

pub const WM8993_HPOUT1R_DLY: c_uint = 0x0002  /* HPOUT1R_DLY */;
pub const WM8993_HPOUT1R_DLY_MASK: c_uint = 0x0002  /* HPOUT1R_DLY */;

//
// R98 (0x62) - EQ1
//
pub const WM8993_EQ_ENA: c_uint = 0x0001  /* EQ_ENA */;
pub const WM8993_EQ_ENA_MASK: c_uint = 0x0001  /* EQ_ENA */;

//
// R99 (0x63) - EQ2
//
pub const WM8993_EQ_B1_GAIN_MASK: c_uint = 0x001F  /* EQ_B1_GAIN - [4:0] */;

//
// R100 (0x64) - EQ3
//
pub const WM8993_EQ_B2_GAIN_MASK: c_uint = 0x001F  /* EQ_B2_GAIN - [4:0] */;

//
// R101 (0x65) - EQ4
//
pub const WM8993_EQ_B3_GAIN_MASK: c_uint = 0x001F  /* EQ_B3_GAIN - [4:0] */;

//
// R102 (0x66) - EQ5
//
pub const WM8993_EQ_B4_GAIN_MASK: c_uint = 0x001F  /* EQ_B4_GAIN - [4:0] */;

//
// R103 (0x67) - EQ6
//
pub const WM8993_EQ_B5_GAIN_MASK: c_uint = 0x001F  /* EQ_B5_GAIN - [4:0] */;

//
// R104 (0x68) - EQ7
//
pub const WM8993_EQ_B1_A_MASK: c_uint = 0xFFFF  /* EQ_B1_A - [15:0] */;

//
// R105 (0x69) - EQ8
//
pub const WM8993_EQ_B1_B_MASK: c_uint = 0xFFFF  /* EQ_B1_B - [15:0] */;

//
// R106 (0x6A) - EQ9
//
pub const WM8993_EQ_B1_PG_MASK: c_uint = 0xFFFF  /* EQ_B1_PG - [15:0] */;

//
// R107 (0x6B) - EQ10
//
pub const WM8993_EQ_B2_A_MASK: c_uint = 0xFFFF  /* EQ_B2_A - [15:0] */;

//
// R108 (0x6C) - EQ11
//
pub const WM8993_EQ_B2_B_MASK: c_uint = 0xFFFF  /* EQ_B2_B - [15:0] */;

//
// R109 (0x6D) - EQ12
//
pub const WM8993_EQ_B2_C_MASK: c_uint = 0xFFFF  /* EQ_B2_C - [15:0] */;

//
// R110 (0x6E) - EQ13
//
pub const WM8993_EQ_B2_PG_MASK: c_uint = 0xFFFF  /* EQ_B2_PG - [15:0] */;

//
// R111 (0x6F) - EQ14
//
pub const WM8993_EQ_B3_A_MASK: c_uint = 0xFFFF  /* EQ_B3_A - [15:0] */;

//
// R112 (0x70) - EQ15
//
pub const WM8993_EQ_B3_B_MASK: c_uint = 0xFFFF  /* EQ_B3_B - [15:0] */;

//
// R113 (0x71) - EQ16
//
pub const WM8993_EQ_B3_C_MASK: c_uint = 0xFFFF  /* EQ_B3_C - [15:0] */;

//
// R114 (0x72) - EQ17
//
pub const WM8993_EQ_B3_PG_MASK: c_uint = 0xFFFF  /* EQ_B3_PG - [15:0] */;

//
// R115 (0x73) - EQ18
//
pub const WM8993_EQ_B4_A_MASK: c_uint = 0xFFFF  /* EQ_B4_A - [15:0] */;

//
// R116 (0x74) - EQ19
//
pub const WM8993_EQ_B4_B_MASK: c_uint = 0xFFFF  /* EQ_B4_B - [15:0] */;

//
// R117 (0x75) - EQ20
//
pub const WM8993_EQ_B4_C_MASK: c_uint = 0xFFFF  /* EQ_B4_C - [15:0] */;

//
// R118 (0x76) - EQ21
//
pub const WM8993_EQ_B4_PG_MASK: c_uint = 0xFFFF  /* EQ_B4_PG - [15:0] */;

//
// R119 (0x77) - EQ22
//
pub const WM8993_EQ_B5_A_MASK: c_uint = 0xFFFF  /* EQ_B5_A - [15:0] */;

//
// R120 (0x78) - EQ23
//
pub const WM8993_EQ_B5_B_MASK: c_uint = 0xFFFF  /* EQ_B5_B - [15:0] */;

//
// R121 (0x79) - EQ24
//
pub const WM8993_EQ_B5_PG_MASK: c_uint = 0xFFFF  /* EQ_B5_PG - [15:0] */;

//
// R122 (0x7A) - Digital Pulls
//
pub const WM8993_MCLK_PU: c_uint = 0x0080  /* MCLK_PU */;
pub const WM8993_MCLK_PU_MASK: c_uint = 0x0080  /* MCLK_PU */;

pub const WM8993_MCLK_PD: c_uint = 0x0040  /* MCLK_PD */;
pub const WM8993_MCLK_PD_MASK: c_uint = 0x0040  /* MCLK_PD */;

pub const WM8993_DACDAT_PU: c_uint = 0x0020  /* DACDAT_PU */;
pub const WM8993_DACDAT_PU_MASK: c_uint = 0x0020  /* DACDAT_PU */;

pub const WM8993_DACDAT_PD: c_uint = 0x0010  /* DACDAT_PD */;
pub const WM8993_DACDAT_PD_MASK: c_uint = 0x0010  /* DACDAT_PD */;

pub const WM8993_LRCLK_PU: c_uint = 0x0008  /* LRCLK_PU */;
pub const WM8993_LRCLK_PU_MASK: c_uint = 0x0008  /* LRCLK_PU */;

pub const WM8993_LRCLK_PD: c_uint = 0x0004  /* LRCLK_PD */;
pub const WM8993_LRCLK_PD_MASK: c_uint = 0x0004  /* LRCLK_PD */;

pub const WM8993_BCLK_PU: c_uint = 0x0002  /* BCLK_PU */;
pub const WM8993_BCLK_PU_MASK: c_uint = 0x0002  /* BCLK_PU */;

pub const WM8993_BCLK_PD: c_uint = 0x0001  /* BCLK_PD */;
pub const WM8993_BCLK_PD_MASK: c_uint = 0x0001  /* BCLK_PD */;

//
// R123 (0x7B) - DRC Control 1
//
pub const WM8993_DRC_ENA: c_uint = 0x8000  /* DRC_ENA */;
pub const WM8993_DRC_ENA_MASK: c_uint = 0x8000  /* DRC_ENA */;

pub const WM8993_DRC_DAC_PATH: c_uint = 0x4000  /* DRC_DAC_PATH */;
pub const WM8993_DRC_DAC_PATH_MASK: c_uint = 0x4000  /* DRC_DAC_PATH */;

pub const WM8993_DRC_SMOOTH_ENA: c_uint = 0x0800  /* DRC_SMOOTH_ENA */;
pub const WM8993_DRC_SMOOTH_ENA_MASK: c_uint = 0x0800  /* DRC_SMOOTH_ENA */;

pub const WM8993_DRC_QR_ENA: c_uint = 0x0400  /* DRC_QR_ENA */;
pub const WM8993_DRC_QR_ENA_MASK: c_uint = 0x0400  /* DRC_QR_ENA */;

pub const WM8993_DRC_ANTICLIP_ENA: c_uint = 0x0200  /* DRC_ANTICLIP_ENA */;
pub const WM8993_DRC_ANTICLIP_ENA_MASK: c_uint = 0x0200  /* DRC_ANTICLIP_ENA */;

pub const WM8993_DRC_HYST_ENA: c_uint = 0x0100  /* DRC_HYST_ENA */;
pub const WM8993_DRC_HYST_ENA_MASK: c_uint = 0x0100  /* DRC_HYST_ENA */;

pub const WM8993_DRC_THRESH_HYST_MASK: c_uint = 0x0030  /* DRC_THRESH_HYST - [5:4] */;

pub const WM8993_DRC_MINGAIN_MASK: c_uint = 0x000C  /* DRC_MINGAIN - [3:2] */;

pub const WM8993_DRC_MAXGAIN_MASK: c_uint = 0x0003  /* DRC_MAXGAIN - [1:0] */;

//
// R124 (0x7C) - DRC Control 2
//
pub const WM8993_DRC_ATTACK_RATE_MASK: c_uint = 0xF000  /* DRC_ATTACK_RATE - [15:12] */;

pub const WM8993_DRC_DECAY_RATE_MASK: c_uint = 0x0F00  /* DRC_DECAY_RATE - [11:8] */;

pub const WM8993_DRC_THRESH_COMP_MASK: c_uint = 0x00FC  /* DRC_THRESH_COMP - [7:2] */;

//
// R125 (0x7D) - DRC Control 3
//
pub const WM8993_DRC_AMP_COMP_MASK: c_uint = 0xF800  /* DRC_AMP_COMP - [15:11] */;

pub const WM8993_DRC_R0_SLOPE_COMP_MASK: c_uint = 0x0700  /* DRC_R0_SLOPE_COMP - [10:8] */;

pub const WM8993_DRC_FF_DELAY: c_uint = 0x0080  /* DRC_FF_DELAY */;
pub const WM8993_DRC_FF_DELAY_MASK: c_uint = 0x0080  /* DRC_FF_DELAY */;

pub const WM8993_DRC_THRESH_QR_MASK: c_uint = 0x000C  /* DRC_THRESH_QR - [3:2] */;

pub const WM8993_DRC_RATE_QR_MASK: c_uint = 0x0003  /* DRC_RATE_QR - [1:0] */;

//
// R126 (0x7E) - DRC Control 4
//
pub const WM8993_DRC_R1_SLOPE_COMP_MASK: c_uint = 0xE000  /* DRC_R1_SLOPE_COMP - [15:13] */;

pub const WM8993_DRC_STARTUP_GAIN_MASK: c_uint = 0x1F00  /* DRC_STARTUP_GAIN - [12:8] */;

