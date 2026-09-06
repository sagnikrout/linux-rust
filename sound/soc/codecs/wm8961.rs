//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/wm8961.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// wm8961.h  --  WM8961 Soc Audio driver
//

pub const WM8961_BCLK: c_int = 1;
pub const WM8961_LRCLK: c_int = 2;
pub const WM8961_BCLK_DIV_1: c_int = 0;
pub const WM8961_BCLK_DIV_1_5: c_int = 1;
pub const WM8961_BCLK_DIV_2: c_int = 2;
pub const WM8961_BCLK_DIV_3: c_int = 3;
pub const WM8961_BCLK_DIV_4: c_int = 4;
pub const WM8961_BCLK_DIV_5_5: c_int = 5;
pub const WM8961_BCLK_DIV_6: c_int = 6;
pub const WM8961_BCLK_DIV_8: c_int = 7;
pub const WM8961_BCLK_DIV_11: c_int = 8;
pub const WM8961_BCLK_DIV_12: c_int = 9;
pub const WM8961_BCLK_DIV_16: c_int = 10;
pub const WM8961_BCLK_DIV_24: c_int = 11;
pub const WM8961_BCLK_DIV_32: c_int = 13;
//
// Register values.
//
pub const WM8961_LEFT_INPUT_VOLUME: c_uint = 0x00;
pub const WM8961_RIGHT_INPUT_VOLUME: c_uint = 0x01;
pub const WM8961_LOUT1_VOLUME: c_uint = 0x02;
pub const WM8961_ROUT1_VOLUME: c_uint = 0x03;
pub const WM8961_CLOCKING1: c_uint = 0x04;
pub const WM8961_ADC_DAC_CONTROL_1: c_uint = 0x05;
pub const WM8961_ADC_DAC_CONTROL_2: c_uint = 0x06;
pub const WM8961_AUDIO_INTERFACE_0: c_uint = 0x07;
pub const WM8961_CLOCKING2: c_uint = 0x08;
pub const WM8961_AUDIO_INTERFACE_1: c_uint = 0x09;
pub const WM8961_LEFT_DAC_VOLUME: c_uint = 0x0A;
pub const WM8961_RIGHT_DAC_VOLUME: c_uint = 0x0B;
pub const WM8961_AUDIO_INTERFACE_2: c_uint = 0x0E;
pub const WM8961_SOFTWARE_RESET: c_uint = 0x0F;
pub const WM8961_ALC1: c_uint = 0x11;
pub const WM8961_ALC2: c_uint = 0x12;
pub const WM8961_ALC3: c_uint = 0x13;
pub const WM8961_NOISE_GATE: c_uint = 0x14;
pub const WM8961_LEFT_ADC_VOLUME: c_uint = 0x15;
pub const WM8961_RIGHT_ADC_VOLUME: c_uint = 0x16;
pub const WM8961_ADDITIONAL_CONTROL_1: c_uint = 0x17;
pub const WM8961_ADDITIONAL_CONTROL_2: c_uint = 0x18;
pub const WM8961_PWR_MGMT_1: c_uint = 0x19;
pub const WM8961_PWR_MGMT_2: c_uint = 0x1A;
pub const WM8961_ADDITIONAL_CONTROL_3: c_uint = 0x1B;
pub const WM8961_ANTI_POP: c_uint = 0x1C;
pub const WM8961_CLOCKING_3: c_uint = 0x1E;
pub const WM8961_ADCL_SIGNAL_PATH: c_uint = 0x20;
pub const WM8961_ADCR_SIGNAL_PATH: c_uint = 0x21;
pub const WM8961_LOUT2_VOLUME: c_uint = 0x28;
pub const WM8961_ROUT2_VOLUME: c_uint = 0x29;
pub const WM8961_PWR_MGMT_3: c_uint = 0x2F;
pub const WM8961_ADDITIONAL_CONTROL_4: c_uint = 0x30;
pub const WM8961_CLASS_D_CONTROL_1: c_uint = 0x31;
pub const WM8961_CLASS_D_CONTROL_2: c_uint = 0x33;
pub const WM8961_CLOCKING_4: c_uint = 0x38;
pub const WM8961_DSP_SIDETONE_0: c_uint = 0x39;
pub const WM8961_DSP_SIDETONE_1: c_uint = 0x3A;
pub const WM8961_DC_SERVO_0: c_uint = 0x3C;
pub const WM8961_DC_SERVO_1: c_uint = 0x3D;
pub const WM8961_DC_SERVO_3: c_uint = 0x3F;
pub const WM8961_DC_SERVO_5: c_uint = 0x41;
pub const WM8961_ANALOGUE_PGA_BIAS: c_uint = 0x44;
pub const WM8961_ANALOGUE_HP_0: c_uint = 0x45;
pub const WM8961_ANALOGUE_HP_2: c_uint = 0x47;
pub const WM8961_CHARGE_PUMP_1: c_uint = 0x48;
pub const WM8961_CHARGE_PUMP_B: c_uint = 0x52;
pub const WM8961_WRITE_SEQUENCER_1: c_uint = 0x57;
pub const WM8961_WRITE_SEQUENCER_2: c_uint = 0x58;
pub const WM8961_WRITE_SEQUENCER_3: c_uint = 0x59;
pub const WM8961_WRITE_SEQUENCER_4: c_uint = 0x5A;
pub const WM8961_WRITE_SEQUENCER_5: c_uint = 0x5B;
pub const WM8961_WRITE_SEQUENCER_6: c_uint = 0x5C;
pub const WM8961_WRITE_SEQUENCER_7: c_uint = 0x5D;
pub const WM8961_GENERAL_TEST_1: c_uint = 0xFC;
//
// Field Definitions.
//
// R0 (0x00) - Left Input volume
//
pub const WM8961_IPVU: c_uint = 0x0100  /* IPVU */;
pub const WM8961_IPVU_MASK: c_uint = 0x0100  /* IPVU */;

pub const WM8961_LINMUTE: c_uint = 0x0080  /* LINMUTE */;
pub const WM8961_LINMUTE_MASK: c_uint = 0x0080  /* LINMUTE */;

pub const WM8961_LIZC: c_uint = 0x0040  /* LIZC */;
pub const WM8961_LIZC_MASK: c_uint = 0x0040  /* LIZC */;

pub const WM8961_LINVOL_MASK: c_uint = 0x003F  /* LINVOL - [5:0] */;

//
// R1 (0x01) - Right Input volume
//
pub const WM8961_DEVICE_ID_MASK: c_uint = 0xF000  /* DEVICE_ID - [15:12] */;

pub const WM8961_CHIP_REV_MASK: c_uint = 0x0E00  /* CHIP_REV - [11:9] */;

pub const WM8961_IPVU: c_uint = 0x0100  /* IPVU */;
pub const WM8961_IPVU_MASK: c_uint = 0x0100  /* IPVU */;

pub const WM8961_RINMUTE: c_uint = 0x0080  /* RINMUTE */;
pub const WM8961_RINMUTE_MASK: c_uint = 0x0080  /* RINMUTE */;

pub const WM8961_RIZC: c_uint = 0x0040  /* RIZC */;
pub const WM8961_RIZC_MASK: c_uint = 0x0040  /* RIZC */;

pub const WM8961_RINVOL_MASK: c_uint = 0x003F  /* RINVOL - [5:0] */;

//
// R2 (0x02) - LOUT1 volume
//
pub const WM8961_OUT1VU: c_uint = 0x0100  /* OUT1VU */;
pub const WM8961_OUT1VU_MASK: c_uint = 0x0100  /* OUT1VU */;

pub const WM8961_LO1ZC: c_uint = 0x0080  /* LO1ZC */;
pub const WM8961_LO1ZC_MASK: c_uint = 0x0080  /* LO1ZC */;

pub const WM8961_LOUT1VOL_MASK: c_uint = 0x007F  /* LOUT1VOL - [6:0] */;

//
// R3 (0x03) - ROUT1 volume
//
pub const WM8961_OUT1VU: c_uint = 0x0100  /* OUT1VU */;
pub const WM8961_OUT1VU_MASK: c_uint = 0x0100  /* OUT1VU */;

pub const WM8961_RO1ZC: c_uint = 0x0080  /* RO1ZC */;
pub const WM8961_RO1ZC_MASK: c_uint = 0x0080  /* RO1ZC */;

pub const WM8961_ROUT1VOL_MASK: c_uint = 0x007F  /* ROUT1VOL - [6:0] */;

//
// R4 (0x04) - Clocking1
//
pub const WM8961_ADCDIV_MASK: c_uint = 0x01C0  /* ADCDIV - [8:6] */;

pub const WM8961_DACDIV_MASK: c_uint = 0x0038  /* DACDIV - [5:3] */;

pub const WM8961_MCLKDIV: c_uint = 0x0004  /* MCLKDIV */;
pub const WM8961_MCLKDIV_MASK: c_uint = 0x0004  /* MCLKDIV */;

//
// R5 (0x05) - ADC & DAC Control 1
//
pub const WM8961_ADCPOL_MASK: c_uint = 0x0060  /* ADCPOL - [6:5] */;

pub const WM8961_DACMU: c_uint = 0x0008  /* DACMU */;
pub const WM8961_DACMU_MASK: c_uint = 0x0008  /* DACMU */;

pub const WM8961_DEEMPH_MASK: c_uint = 0x0006  /* DEEMPH - [2:1] */;

pub const WM8961_ADCHPD: c_uint = 0x0001  /* ADCHPD */;
pub const WM8961_ADCHPD_MASK: c_uint = 0x0001  /* ADCHPD */;

//
// R6 (0x06) - ADC & DAC Control 2
//
pub const WM8961_ADC_HPF_CUT_MASK: c_uint = 0x0180  /* ADC_HPF_CUT - [8:7] */;

pub const WM8961_DACPOL_MASK: c_uint = 0x0060  /* DACPOL - [6:5] */;

pub const WM8961_DACSMM: c_uint = 0x0008  /* DACSMM */;
pub const WM8961_DACSMM_MASK: c_uint = 0x0008  /* DACSMM */;

pub const WM8961_DACMR: c_uint = 0x0004  /* DACMR */;
pub const WM8961_DACMR_MASK: c_uint = 0x0004  /* DACMR */;

pub const WM8961_DACSLOPE: c_uint = 0x0002  /* DACSLOPE */;
pub const WM8961_DACSLOPE_MASK: c_uint = 0x0002  /* DACSLOPE */;

pub const WM8961_DAC_OSR128: c_uint = 0x0001  /* DAC_OSR128 */;
pub const WM8961_DAC_OSR128_MASK: c_uint = 0x0001  /* DAC_OSR128 */;

//
// R7 (0x07) - Audio Interface 0
//
pub const WM8961_ALRSWAP: c_uint = 0x0100  /* ALRSWAP */;
pub const WM8961_ALRSWAP_MASK: c_uint = 0x0100  /* ALRSWAP */;

pub const WM8961_BCLKINV: c_uint = 0x0080  /* BCLKINV */;
pub const WM8961_BCLKINV_MASK: c_uint = 0x0080  /* BCLKINV */;

pub const WM8961_MS: c_uint = 0x0040  /* MS */;
pub const WM8961_MS_MASK: c_uint = 0x0040  /* MS */;

pub const WM8961_DLRSWAP: c_uint = 0x0020  /* DLRSWAP */;
pub const WM8961_DLRSWAP_MASK: c_uint = 0x0020  /* DLRSWAP */;

pub const WM8961_LRP: c_uint = 0x0010  /* LRP */;
pub const WM8961_LRP_MASK: c_uint = 0x0010  /* LRP */;

pub const WM8961_WL_MASK: c_uint = 0x000C  /* WL - [3:2] */;

pub const WM8961_FORMAT_MASK: c_uint = 0x0003  /* FORMAT - [1:0] */;

//
// R8 (0x08) - Clocking2
//
pub const WM8961_DCLKDIV_MASK: c_uint = 0x01C0  /* DCLKDIV - [8:6] */;

pub const WM8961_CLK_SYS_ENA: c_uint = 0x0020  /* CLK_SYS_ENA */;
pub const WM8961_CLK_SYS_ENA_MASK: c_uint = 0x0020  /* CLK_SYS_ENA */;

pub const WM8961_CLK_DSP_ENA: c_uint = 0x0010  /* CLK_DSP_ENA */;
pub const WM8961_CLK_DSP_ENA_MASK: c_uint = 0x0010  /* CLK_DSP_ENA */;

pub const WM8961_BCLKDIV_MASK: c_uint = 0x000F  /* BCLKDIV - [3:0] */;

//
// R9 (0x09) - Audio Interface 1
//
pub const WM8961_DACCOMP_MASK: c_uint = 0x0018  /* DACCOMP - [4:3] */;

pub const WM8961_ADCCOMP_MASK: c_uint = 0x0006  /* ADCCOMP - [2:1] */;

pub const WM8961_LOOPBACK: c_uint = 0x0001  /* LOOPBACK */;
pub const WM8961_LOOPBACK_MASK: c_uint = 0x0001  /* LOOPBACK */;

//
// R10 (0x0A) - Left DAC volume
//
pub const WM8961_DACVU: c_uint = 0x0100  /* DACVU */;
pub const WM8961_DACVU_MASK: c_uint = 0x0100  /* DACVU */;

pub const WM8961_LDACVOL_MASK: c_uint = 0x00FF  /* LDACVOL - [7:0] */;

//
// R11 (0x0B) - Right DAC volume
//
pub const WM8961_DACVU: c_uint = 0x0100  /* DACVU */;
pub const WM8961_DACVU_MASK: c_uint = 0x0100  /* DACVU */;

pub const WM8961_RDACVOL_MASK: c_uint = 0x00FF  /* RDACVOL - [7:0] */;

//
// R14 (0x0E) - Audio Interface 2
//
pub const WM8961_LRCLK_RATE_MASK: c_uint = 0x01FF  /* LRCLK_RATE - [8:0] */;

//
// R15 (0x0F) - Software Reset
//
pub const WM8961_SW_RST_DEV_ID1_MASK: c_uint = 0xFFFF  /* SW_RST_DEV_ID1 - [15:0] */;

//
// R17 (0x11) - ALC1
//
pub const WM8961_ALCSEL_MASK: c_uint = 0x0180  /* ALCSEL - [8:7] */;

pub const WM8961_MAXGAIN_MASK: c_uint = 0x0070  /* MAXGAIN - [6:4] */;

pub const WM8961_ALCL_MASK: c_uint = 0x000F  /* ALCL - [3:0] */;

//
// R18 (0x12) - ALC2
//
pub const WM8961_ALCZC: c_uint = 0x0080  /* ALCZC */;
pub const WM8961_ALCZC_MASK: c_uint = 0x0080  /* ALCZC */;

pub const WM8961_MINGAIN_MASK: c_uint = 0x0070  /* MINGAIN - [6:4] */;

pub const WM8961_HLD_MASK: c_uint = 0x000F  /* HLD - [3:0] */;

//
// R19 (0x13) - ALC3
//
pub const WM8961_ALCMODE: c_uint = 0x0100  /* ALCMODE */;
pub const WM8961_ALCMODE_MASK: c_uint = 0x0100  /* ALCMODE */;

pub const WM8961_DCY_MASK: c_uint = 0x00F0  /* DCY - [7:4] */;

pub const WM8961_ATK_MASK: c_uint = 0x000F  /* ATK - [3:0] */;

//
// R20 (0x14) - Noise Gate
//
pub const WM8961_NGTH_MASK: c_uint = 0x00F8  /* NGTH - [7:3] */;

pub const WM8961_NGG: c_uint = 0x0002  /* NGG */;
pub const WM8961_NGG_MASK: c_uint = 0x0002  /* NGG */;

pub const WM8961_NGAT: c_uint = 0x0001  /* NGAT */;
pub const WM8961_NGAT_MASK: c_uint = 0x0001  /* NGAT */;

//
// R21 (0x15) - Left ADC volume
//
pub const WM8961_ADCVU: c_uint = 0x0100  /* ADCVU */;
pub const WM8961_ADCVU_MASK: c_uint = 0x0100  /* ADCVU */;

pub const WM8961_LADCVOL_MASK: c_uint = 0x00FF  /* LADCVOL - [7:0] */;

//
// R22 (0x16) - Right ADC volume
//
pub const WM8961_ADCVU: c_uint = 0x0100  /* ADCVU */;
pub const WM8961_ADCVU_MASK: c_uint = 0x0100  /* ADCVU */;

pub const WM8961_RADCVOL_MASK: c_uint = 0x00FF  /* RADCVOL - [7:0] */;

//
// R23 (0x17) - Additional control(1)
//
pub const WM8961_TSDEN: c_uint = 0x0100  /* TSDEN */;
pub const WM8961_TSDEN_MASK: c_uint = 0x0100  /* TSDEN */;

pub const WM8961_DMONOMIX: c_uint = 0x0010  /* DMONOMIX */;
pub const WM8961_DMONOMIX_MASK: c_uint = 0x0010  /* DMONOMIX */;

pub const WM8961_TOEN: c_uint = 0x0001  /* TOEN */;
pub const WM8961_TOEN_MASK: c_uint = 0x0001  /* TOEN */;

//
// R24 (0x18) - Additional control(2)
//
pub const WM8961_TRIS: c_uint = 0x0008  /* TRIS */;
pub const WM8961_TRIS_MASK: c_uint = 0x0008  /* TRIS */;

//
// R25 (0x19) - Pwr Mgmt (1)
//
pub const WM8961_VMIDSEL_MASK: c_uint = 0x0180  /* VMIDSEL - [8:7] */;

pub const WM8961_VREF: c_uint = 0x0040  /* VREF */;
pub const WM8961_VREF_MASK: c_uint = 0x0040  /* VREF */;

pub const WM8961_AINL: c_uint = 0x0020  /* AINL */;
pub const WM8961_AINL_MASK: c_uint = 0x0020  /* AINL */;

pub const WM8961_AINR: c_uint = 0x0010  /* AINR */;
pub const WM8961_AINR_MASK: c_uint = 0x0010  /* AINR */;

pub const WM8961_ADCL: c_uint = 0x0008  /* ADCL */;
pub const WM8961_ADCL_MASK: c_uint = 0x0008  /* ADCL */;

pub const WM8961_ADCR: c_uint = 0x0004  /* ADCR */;
pub const WM8961_ADCR_MASK: c_uint = 0x0004  /* ADCR */;

pub const WM8961_MICB: c_uint = 0x0002  /* MICB */;
pub const WM8961_MICB_MASK: c_uint = 0x0002  /* MICB */;

//
// R26 (0x1A) - Pwr Mgmt (2)
//
pub const WM8961_DACL: c_uint = 0x0100  /* DACL */;
pub const WM8961_DACL_MASK: c_uint = 0x0100  /* DACL */;

pub const WM8961_DACR: c_uint = 0x0080  /* DACR */;
pub const WM8961_DACR_MASK: c_uint = 0x0080  /* DACR */;

pub const WM8961_LOUT1_PGA: c_uint = 0x0040  /* LOUT1_PGA */;
pub const WM8961_LOUT1_PGA_MASK: c_uint = 0x0040  /* LOUT1_PGA */;

pub const WM8961_ROUT1_PGA: c_uint = 0x0020  /* ROUT1_PGA */;
pub const WM8961_ROUT1_PGA_MASK: c_uint = 0x0020  /* ROUT1_PGA */;

pub const WM8961_SPKL_PGA: c_uint = 0x0010  /* SPKL_PGA */;
pub const WM8961_SPKL_PGA_MASK: c_uint = 0x0010  /* SPKL_PGA */;

pub const WM8961_SPKR_PGA: c_uint = 0x0008  /* SPKR_PGA */;
pub const WM8961_SPKR_PGA_MASK: c_uint = 0x0008  /* SPKR_PGA */;

//
// R27 (0x1B) - Additional Control (3)
//
pub const WM8961_SAMPLE_RATE_MASK: c_uint = 0x0007  /* SAMPLE_RATE - [2:0] */;

//
// R28 (0x1C) - Anti-pop
//
pub const WM8961_BUFDCOPEN: c_uint = 0x0010  /* BUFDCOPEN */;
pub const WM8961_BUFDCOPEN_MASK: c_uint = 0x0010  /* BUFDCOPEN */;

pub const WM8961_BUFIOEN: c_uint = 0x0008  /* BUFIOEN */;
pub const WM8961_BUFIOEN_MASK: c_uint = 0x0008  /* BUFIOEN */;

pub const WM8961_SOFT_ST: c_uint = 0x0004  /* SOFT_ST */;
pub const WM8961_SOFT_ST_MASK: c_uint = 0x0004  /* SOFT_ST */;

//
// R30 (0x1E) - Clocking 3
//
pub const WM8961_CLK_TO_DIV_MASK: c_uint = 0x0180  /* CLK_TO_DIV - [8:7] */;

pub const WM8961_CLK_256K_DIV_MASK: c_uint = 0x007E  /* CLK_256K_DIV - [6:1] */;

pub const WM8961_MANUAL_MODE: c_uint = 0x0001  /* MANUAL_MODE */;
pub const WM8961_MANUAL_MODE_MASK: c_uint = 0x0001  /* MANUAL_MODE */;

//
// R32 (0x20) - ADCL signal path
//
pub const WM8961_LMICBOOST_MASK: c_uint = 0x0030  /* LMICBOOST - [5:4] */;

//
// R33 (0x21) - ADCR signal path
//
pub const WM8961_RMICBOOST_MASK: c_uint = 0x0030  /* RMICBOOST - [5:4] */;

//
// R40 (0x28) - LOUT2 volume
//
pub const WM8961_SPKVU: c_uint = 0x0100  /* SPKVU */;
pub const WM8961_SPKVU_MASK: c_uint = 0x0100  /* SPKVU */;

pub const WM8961_SPKLZC: c_uint = 0x0080  /* SPKLZC */;
pub const WM8961_SPKLZC_MASK: c_uint = 0x0080  /* SPKLZC */;

pub const WM8961_SPKLVOL_MASK: c_uint = 0x007F  /* SPKLVOL - [6:0] */;

//
// R41 (0x29) - ROUT2 volume
//
pub const WM8961_SPKVU: c_uint = 0x0100  /* SPKVU */;
pub const WM8961_SPKVU_MASK: c_uint = 0x0100  /* SPKVU */;

pub const WM8961_SPKRZC: c_uint = 0x0080  /* SPKRZC */;
pub const WM8961_SPKRZC_MASK: c_uint = 0x0080  /* SPKRZC */;

pub const WM8961_SPKRVOL_MASK: c_uint = 0x007F  /* SPKRVOL - [6:0] */;

//
// R47 (0x2F) - Pwr Mgmt (3)
//
pub const WM8961_TEMP_SHUT: c_uint = 0x0002  /* TEMP_SHUT */;
pub const WM8961_TEMP_SHUT_MASK: c_uint = 0x0002  /* TEMP_SHUT */;

pub const WM8961_TEMP_WARN: c_uint = 0x0001  /* TEMP_WARN */;
pub const WM8961_TEMP_WARN_MASK: c_uint = 0x0001  /* TEMP_WARN */;

//
// R48 (0x30) - Additional Control (4)
//
pub const WM8961_TSENSEN: c_uint = 0x0002  /* TSENSEN */;
pub const WM8961_TSENSEN_MASK: c_uint = 0x0002  /* TSENSEN */;

pub const WM8961_MBSEL: c_uint = 0x0001  /* MBSEL */;
pub const WM8961_MBSEL_MASK: c_uint = 0x0001  /* MBSEL */;

//
// R49 (0x31) - Class D Control 1
//
pub const WM8961_SPKR_ENA: c_uint = 0x0080  /* SPKR_ENA */;
pub const WM8961_SPKR_ENA_MASK: c_uint = 0x0080  /* SPKR_ENA */;

pub const WM8961_SPKL_ENA: c_uint = 0x0040  /* SPKL_ENA */;
pub const WM8961_SPKL_ENA_MASK: c_uint = 0x0040  /* SPKL_ENA */;

//
// R51 (0x33) - Class D Control 2
//
pub const WM8961_CLASSD_ACGAIN_MASK: c_uint = 0x0007  /* CLASSD_ACGAIN - [2:0] */;

//
// R56 (0x38) - Clocking 4
//
pub const WM8961_CLK_DCS_DIV_MASK: c_uint = 0x01E0  /* CLK_DCS_DIV - [8:5] */;

pub const WM8961_CLK_SYS_RATE_MASK: c_uint = 0x001E  /* CLK_SYS_RATE - [4:1] */;

//
// R57 (0x39) - DSP Sidetone 0
//
pub const WM8961_ADCR_DAC_SVOL_MASK: c_uint = 0x00F0  /* ADCR_DAC_SVOL - [7:4] */;

pub const WM8961_ADC_TO_DACR_MASK: c_uint = 0x000C  /* ADC_TO_DACR - [3:2] */;

//
// R58 (0x3A) - DSP Sidetone 1
//
pub const WM8961_ADCL_DAC_SVOL_MASK: c_uint = 0x00F0  /* ADCL_DAC_SVOL - [7:4] */;

pub const WM8961_ADC_TO_DACL_MASK: c_uint = 0x000C  /* ADC_TO_DACL - [3:2] */;

//
// R60 (0x3C) - DC Servo 0
//
pub const WM8961_DCS_ENA_CHAN_INL: c_uint = 0x0080  /* DCS_ENA_CHAN_INL */;
pub const WM8961_DCS_ENA_CHAN_INL_MASK: c_uint = 0x0080  /* DCS_ENA_CHAN_INL */;

pub const WM8961_DCS_TRIG_STARTUP_INL: c_uint = 0x0040  /* DCS_TRIG_STARTUP_INL */;
pub const WM8961_DCS_TRIG_STARTUP_INL_MASK: c_uint = 0x0040  /* DCS_TRIG_STARTUP_INL */;

pub const WM8961_DCS_TRIG_SERIES_INL: c_uint = 0x0010  /* DCS_TRIG_SERIES_INL */;
pub const WM8961_DCS_TRIG_SERIES_INL_MASK: c_uint = 0x0010  /* DCS_TRIG_SERIES_INL */;

pub const WM8961_DCS_ENA_CHAN_INR: c_uint = 0x0008  /* DCS_ENA_CHAN_INR */;
pub const WM8961_DCS_ENA_CHAN_INR_MASK: c_uint = 0x0008  /* DCS_ENA_CHAN_INR */;

pub const WM8961_DCS_TRIG_STARTUP_INR: c_uint = 0x0004  /* DCS_TRIG_STARTUP_INR */;
pub const WM8961_DCS_TRIG_STARTUP_INR_MASK: c_uint = 0x0004  /* DCS_TRIG_STARTUP_INR */;

pub const WM8961_DCS_TRIG_SERIES_INR: c_uint = 0x0001  /* DCS_TRIG_SERIES_INR */;
pub const WM8961_DCS_TRIG_SERIES_INR_MASK: c_uint = 0x0001  /* DCS_TRIG_SERIES_INR */;

//
// R61 (0x3D) - DC Servo 1
//
pub const WM8961_DCS_ENA_CHAN_HPL: c_uint = 0x0080  /* DCS_ENA_CHAN_HPL */;
pub const WM8961_DCS_ENA_CHAN_HPL_MASK: c_uint = 0x0080  /* DCS_ENA_CHAN_HPL */;

pub const WM8961_DCS_TRIG_STARTUP_HPL: c_uint = 0x0040  /* DCS_TRIG_STARTUP_HPL */;
pub const WM8961_DCS_TRIG_STARTUP_HPL_MASK: c_uint = 0x0040  /* DCS_TRIG_STARTUP_HPL */;

pub const WM8961_DCS_TRIG_SERIES_HPL: c_uint = 0x0010  /* DCS_TRIG_SERIES_HPL */;
pub const WM8961_DCS_TRIG_SERIES_HPL_MASK: c_uint = 0x0010  /* DCS_TRIG_SERIES_HPL */;

pub const WM8961_DCS_ENA_CHAN_HPR: c_uint = 0x0008  /* DCS_ENA_CHAN_HPR */;
pub const WM8961_DCS_ENA_CHAN_HPR_MASK: c_uint = 0x0008  /* DCS_ENA_CHAN_HPR */;

pub const WM8961_DCS_TRIG_STARTUP_HPR: c_uint = 0x0004  /* DCS_TRIG_STARTUP_HPR */;
pub const WM8961_DCS_TRIG_STARTUP_HPR_MASK: c_uint = 0x0004  /* DCS_TRIG_STARTUP_HPR */;

pub const WM8961_DCS_TRIG_SERIES_HPR: c_uint = 0x0001  /* DCS_TRIG_SERIES_HPR */;
pub const WM8961_DCS_TRIG_SERIES_HPR_MASK: c_uint = 0x0001  /* DCS_TRIG_SERIES_HPR */;

//
// R63 (0x3F) - DC Servo 3
//
pub const WM8961_DCS_FILT_BW_SERIES_MASK: c_uint = 0x0030  /* DCS_FILT_BW_SERIES - [5:4] */;

//
// R65 (0x41) - DC Servo 5
//
pub const WM8961_DCS_SERIES_NO_HP_MASK: c_uint = 0x007F  /* DCS_SERIES_NO_HP - [6:0] */;

//
// R68 (0x44) - Analogue PGA Bias
//
pub const WM8961_HP_PGAS_BIAS_MASK: c_uint = 0x0007  /* HP_PGAS_BIAS - [2:0] */;

//
// R69 (0x45) - Analogue HP 0
//
pub const WM8961_HPL_RMV_SHORT: c_uint = 0x0080  /* HPL_RMV_SHORT */;
pub const WM8961_HPL_RMV_SHORT_MASK: c_uint = 0x0080  /* HPL_RMV_SHORT */;

pub const WM8961_HPL_ENA_OUTP: c_uint = 0x0040  /* HPL_ENA_OUTP */;
pub const WM8961_HPL_ENA_OUTP_MASK: c_uint = 0x0040  /* HPL_ENA_OUTP */;

pub const WM8961_HPL_ENA_DLY: c_uint = 0x0020  /* HPL_ENA_DLY */;
pub const WM8961_HPL_ENA_DLY_MASK: c_uint = 0x0020  /* HPL_ENA_DLY */;

pub const WM8961_HPL_ENA: c_uint = 0x0010  /* HPL_ENA */;
pub const WM8961_HPL_ENA_MASK: c_uint = 0x0010  /* HPL_ENA */;

pub const WM8961_HPR_RMV_SHORT: c_uint = 0x0008  /* HPR_RMV_SHORT */;
pub const WM8961_HPR_RMV_SHORT_MASK: c_uint = 0x0008  /* HPR_RMV_SHORT */;

pub const WM8961_HPR_ENA_OUTP: c_uint = 0x0004  /* HPR_ENA_OUTP */;
pub const WM8961_HPR_ENA_OUTP_MASK: c_uint = 0x0004  /* HPR_ENA_OUTP */;

pub const WM8961_HPR_ENA_DLY: c_uint = 0x0002  /* HPR_ENA_DLY */;
pub const WM8961_HPR_ENA_DLY_MASK: c_uint = 0x0002  /* HPR_ENA_DLY */;

pub const WM8961_HPR_ENA: c_uint = 0x0001  /* HPR_ENA */;
pub const WM8961_HPR_ENA_MASK: c_uint = 0x0001  /* HPR_ENA */;

//
// R71 (0x47) - Analogue HP 2
//
pub const WM8961_HPL_VOL_MASK: c_uint = 0x01C0  /* HPL_VOL - [8:6] */;

pub const WM8961_HPR_VOL_MASK: c_uint = 0x0038  /* HPR_VOL - [5:3] */;

pub const WM8961_HP_BIAS_BOOST_MASK: c_uint = 0x0007  /* HP_BIAS_BOOST - [2:0] */;

//
// R72 (0x48) - Charge Pump 1
//
pub const WM8961_CP_ENA: c_uint = 0x0001  /* CP_ENA */;
pub const WM8961_CP_ENA_MASK: c_uint = 0x0001  /* CP_ENA */;

//
// R82 (0x52) - Charge Pump B
//
pub const WM8961_CP_DYN_PWR_MASK: c_uint = 0x0003  /* CP_DYN_PWR - [1:0] */;

//
// R87 (0x57) - Write Sequencer 1
//
pub const WM8961_WSEQ_ENA: c_uint = 0x0020  /* WSEQ_ENA */;
pub const WM8961_WSEQ_ENA_MASK: c_uint = 0x0020  /* WSEQ_ENA */;

pub const WM8961_WSEQ_WRITE_INDEX_MASK: c_uint = 0x001F  /* WSEQ_WRITE_INDEX - [4:0] */;

//
// R88 (0x58) - Write Sequencer 2
//
pub const WM8961_WSEQ_EOS: c_uint = 0x0100  /* WSEQ_EOS */;
pub const WM8961_WSEQ_EOS_MASK: c_uint = 0x0100  /* WSEQ_EOS */;

pub const WM8961_WSEQ_ADDR_MASK: c_uint = 0x00FF  /* WSEQ_ADDR - [7:0] */;

//
// R89 (0x59) - Write Sequencer 3
//
pub const WM8961_WSEQ_DATA_MASK: c_uint = 0x00FF  /* WSEQ_DATA - [7:0] */;

//
// R90 (0x5A) - Write Sequencer 4
//
pub const WM8961_WSEQ_ABORT: c_uint = 0x0100  /* WSEQ_ABORT */;
pub const WM8961_WSEQ_ABORT_MASK: c_uint = 0x0100  /* WSEQ_ABORT */;

pub const WM8961_WSEQ_START: c_uint = 0x0080  /* WSEQ_START */;
pub const WM8961_WSEQ_START_MASK: c_uint = 0x0080  /* WSEQ_START */;

pub const WM8961_WSEQ_START_INDEX_MASK: c_uint = 0x003F  /* WSEQ_START_INDEX - [5:0] */;

//
// R91 (0x5B) - Write Sequencer 5
//
pub const WM8961_WSEQ_DATA_WIDTH_MASK: c_uint = 0x0070  /* WSEQ_DATA_WIDTH - [6:4] */;

pub const WM8961_WSEQ_DATA_START_MASK: c_uint = 0x000F  /* WSEQ_DATA_START - [3:0] */;

//
// R92 (0x5C) - Write Sequencer 6
//
pub const WM8961_WSEQ_DELAY_MASK: c_uint = 0x000F  /* WSEQ_DELAY - [3:0] */;

//
// R93 (0x5D) - Write Sequencer 7
//
pub const WM8961_WSEQ_BUSY: c_uint = 0x0001  /* WSEQ_BUSY */;
pub const WM8961_WSEQ_BUSY_MASK: c_uint = 0x0001  /* WSEQ_BUSY */;

//
// R252 (0xFC) - General test 1
//
pub const WM8961_ARA_ENA: c_uint = 0x0002  /* ARA_ENA */;
pub const WM8961_ARA_ENA_MASK: c_uint = 0x0002  /* ARA_ENA */;

pub const WM8961_AUTO_INC: c_uint = 0x0001  /* AUTO_INC */;
pub const WM8961_AUTO_INC_MASK: c_uint = 0x0001  /* AUTO_INC */;

