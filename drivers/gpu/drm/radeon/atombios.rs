//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/radeon/atombios.h
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
// Copyright 2006-2007 Advanced Micro Devices, Inc.
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
// Portion I: Definitions  shared between VBIOS and Driver
//
pub const ATOM_VERSION_MAJOR: c_uint = 0x00020000;
pub const ATOM_VERSION_MINOR: c_uint = 0x00000002;

// Endianness should be specified before inclusion,
// default to little endian
//

pub type ULONG = c_ulong;

pub type UCHAR = c_uchar;

pub type USHORT = c_ushort;

pub const ATOM_DAC_A: c_int = 0;
pub const ATOM_DAC_B: c_int = 1;
pub const ATOM_EXT_DAC: c_int = 2;
pub const ATOM_CRTC1: c_int = 0;
pub const ATOM_CRTC2: c_int = 1;
pub const ATOM_CRTC3: c_int = 2;
pub const ATOM_CRTC4: c_int = 3;
pub const ATOM_CRTC5: c_int = 4;
pub const ATOM_CRTC6: c_int = 5;
pub const ATOM_CRTC_INVALID: c_uint = 0xFF;
pub const ATOM_DIGA: c_int = 0;
pub const ATOM_DIGB: c_int = 1;
pub const ATOM_PPLL1: c_int = 0;
pub const ATOM_PPLL2: c_int = 1;
pub const ATOM_DCPLL: c_int = 2;
pub const ATOM_PPLL0: c_int = 2;
pub const ATOM_PPLL3: c_int = 3;
pub const ATOM_EXT_PLL1: c_int = 8;
pub const ATOM_EXT_PLL2: c_int = 9;
pub const ATOM_EXT_CLOCK: c_int = 10;
pub const ATOM_PPLL_INVALID: c_uint = 0xFF;
pub const ENCODER_REFCLK_SRC_P1PLL: c_int = 0;
pub const ENCODER_REFCLK_SRC_P2PLL: c_int = 1;
pub const ENCODER_REFCLK_SRC_DCPLL: c_int = 2;
pub const ENCODER_REFCLK_SRC_EXTCLK: c_int = 3;
pub const ENCODER_REFCLK_SRC_INVALID: c_uint = 0xFF;
pub const ATOM_SCALER1: c_int = 0;
pub const ATOM_SCALER2: c_int = 1;
pub const ATOM_SCALER_DISABLE: c_int = 0;
pub const ATOM_SCALER_CENTER: c_int = 1;
pub const ATOM_SCALER_EXPANSION: c_int = 2;
pub const ATOM_SCALER_MULTI_EX: c_int = 3;
pub const ATOM_DISABLE: c_int = 0;
pub const ATOM_ENABLE: c_int = 1;

pub const ATOM_BLANKING: c_int = 1;
pub const ATOM_BLANKING_OFF: c_int = 0;
pub const ATOM_CURSOR1: c_int = 0;
pub const ATOM_CURSOR2: c_int = 1;
pub const ATOM_ICON1: c_int = 0;
pub const ATOM_ICON2: c_int = 1;
pub const ATOM_CRT1: c_int = 0;
pub const ATOM_CRT2: c_int = 1;
pub const ATOM_TV_NTSC: c_int = 1;
pub const ATOM_TV_NTSCJ: c_int = 2;
pub const ATOM_TV_PAL: c_int = 3;
pub const ATOM_TV_PALM: c_int = 4;
pub const ATOM_TV_PALCN: c_int = 5;
pub const ATOM_TV_PALN: c_int = 6;
pub const ATOM_TV_PAL60: c_int = 7;
pub const ATOM_TV_SECAM: c_int = 8;
pub const ATOM_TV_CV: c_int = 16;
pub const ATOM_DAC1_PS2: c_int = 1;
pub const ATOM_DAC1_CV: c_int = 2;
pub const ATOM_DAC1_NTSC: c_int = 3;
pub const ATOM_DAC1_PAL: c_int = 4;

pub const ATOM_PM_ON: c_int = 0;
pub const ATOM_PM_STANDBY: c_int = 1;
pub const ATOM_PM_SUSPEND: c_int = 2;
pub const ATOM_PM_OFF: c_int = 3;
// Bit0:{=0:single, =1:dual},
pub const ATOM_PANEL_MISC_DUAL: c_uint = 0x00000001;
pub const ATOM_PANEL_MISC_888RGB: c_uint = 0x00000002;
pub const ATOM_PANEL_MISC_GREY_LEVEL: c_uint = 0x0000000C;
pub const ATOM_PANEL_MISC_FPDI: c_uint = 0x00000010;
pub const ATOM_PANEL_MISC_GREY_LEVEL_SHIFT: c_int = 2;
pub const ATOM_PANEL_MISC_SPATIAL: c_uint = 0x00000020;
pub const ATOM_PANEL_MISC_TEMPORAL: c_uint = 0x00000040;
pub const ATOM_PANEL_MISC_API_ENABLED: c_uint = 0x00000080;

// Maximum size of that FireGL flag string

pub const HW_ASSISTED_I2C_STATUS_FAILURE: c_int = 2;
pub const HW_ASSISTED_I2C_STATUS_SUCCESS: c_int = 1;

// Define offset to location of ROM header.
pub const OFFSET_TO_POINTER_TO_ATOM_ROM_HEADER: c_uint = 0x00000048L;
pub const OFFSET_TO_ATOM_ROM_IMAGE_SIZE: c_uint = 0x00000002L;
pub const OFFSET_TO_ATOMBIOS_ASIC_BUS_MEM_TYPE: c_uint = 0x94;

pub const OFFSET_TO_GET_ATOMBIOS_STRINGS_NUMBER: c_uint = 0x002f;
pub const OFFSET_TO_GET_ATOMBIOS_STRINGS_START: c_uint = 0x006e;
// Common header for all ROM Data tables.
// Image can't be updated, while Driver needs to carry the new table!
//
// Structure stores the ROM header.
//
// ==============================Command Table Portion====================================

//
// Structures used in Command.mtb
//
// For backward compatible

//
// Structures used in every command table
//

//
// Common header for all command tables.
// Every table pointed by _ATOM_MASTER_COMMAND_TABLE has this common header.
// And the pointer actually points to this header.
//
// Structures used by ComputeMemoryEnginePLLTable
//
pub const COMPUTE_MEMORY_PLL_PARAM: c_int = 1;
pub const COMPUTE_ENGINE_PLL_PARAM: c_int = 2;
pub const ADJUST_MC_SETTING_PARAM: c_int = 3;
//
// Structures used by AdjustMemoryControllerTable
//

pub const POINTER_RETURN_FLAG: c_uint = 0x80;

pub const SET_CLOCK_FREQ_MASK: c_uint = 0x00FFFFFF  //Clock change tables only take bit [23:0] as the requested clock value;
pub const USE_NON_BUS_CLOCK_MASK: c_uint = 0x01000000  //Applicable to both memory and engine clock change, when set, it uses another clock as the temporary clock (engine uses memory and vice versa);
pub const USE_MEMORY_SELF_REFRESH_MASK: c_uint = 0x02000000	//Only applicable to memory clock change, when set, using memory self refresh during clock transition;
pub const SKIP_INTERNAL_MEMORY_PARAMETER_CHANGE: c_uint = 0x04000000  //Only applicable to memory clock change, when set, the table will skip predefined internal memory parameter change;
pub const FIRST_TIME_CHANGE_CLOCK: c_uint = 0x08000000	//Applicable to both memory and engine clock change,when set, it means this is 1st time to change clock after ASIC bootup;
pub const SKIP_SW_PROGRAM_PLL: c_uint = 0x10000000	//Applicable to both memory and engine clock change, when set, it means the table will not program SPLL/MPLL;

pub const b3USE_NON_BUS_CLOCK_MASK: c_uint = 0x01       //Applicable to both memory and engine clock change, when set, it uses another clock as the temporary clock (engine uses memory and vice versa);
pub const b3USE_MEMORY_SELF_REFRESH: c_uint = 0x02	     //Only applicable to memory clock change, when set, using memory self refresh during clock transition;
pub const b3SKIP_INTERNAL_MEMORY_PARAMETER_CHANGE: c_uint = 0x04       //Only applicable to memory clock change, when set, the table will skip predefined internal memory parameter change;
pub const b3FIRST_TIME_CHANGE_CLOCK: c_uint = 0x08       //Applicable to both memory and engine clock change,when set, it means this is 1st time to change clock after ASIC bootup;
pub const b3SKIP_SW_PROGRAM_PLL: c_uint = 0x10			 //Applicable to both memory and engine clock change, when set, it means the table will not program SPLL/MPLL;

// ucCntlFlag
pub const ATOM_PLL_CNTL_FLAG_PLL_POST_DIV_EN: c_int = 1;
pub const ATOM_PLL_CNTL_FLAG_MPLL_VCO_MODE: c_int = 2;
pub const ATOM_PLL_CNTL_FLAG_FRACTION_DISABLE: c_int = 4;
pub const ATOM_PLL_CNTL_FLAG_SPLL_ISPARE_9: c_int = 8;
// V4 are only used for APU which PLL outside GPU

// ATOM_COMPUTE_CLOCK_FREQ.ulComputeClockFlag
pub const COMPUTE_GPUCLK_INPUT_FLAG_CLK_TYPE_MASK: c_uint = 0x0f;
pub const COMPUTE_GPUCLK_INPUT_FLAG_DEFAULT_GPUCLK: c_uint = 0x00;
pub const COMPUTE_GPUCLK_INPUT_FLAG_SCLK: c_uint = 0x01;
// ucPllCntlFlag
pub const SPLL_CNTL_FLAG_VCO_MODE_MASK: c_uint = 0x03;
// ucInputFlag

// use for ComputeMemoryClockParamTable
// definition of ucInputFlag
pub const MPLL_INPUT_FLAG_STROBE_MODE_EN: c_uint = 0x01;
// definition of ucPllCntlFlag
pub const MPLL_CNTL_FLAG_VCO_MODE_MASK: c_uint = 0x03;
pub const MPLL_CNTL_FLAG_BYPASS_DQ_PLL: c_uint = 0x04;
pub const MPLL_CNTL_FLAG_QDR_ENABLE: c_uint = 0x08;
pub const MPLL_CNTL_FLAG_AD_HALF_RATE: c_uint = 0x10;
// MPLL_CNTL_FLAG_BYPASS_AD_PLL has a wrong name, should be BYPASS_DQ_PLL
pub const MPLL_CNTL_FLAG_BYPASS_AD_PLL: c_uint = 0x04;
//
// Structures used by SetEngineClockTable
//
// Structures used by SetMemoryClockTable
//
// Structures used by ASIC_Init.ctb
//
// Structure used by DynamicClockGatingTable.ctb
//

//
// Structure used by EnableDispPowerGatingTable.ctb
//
// Structure used by EnableASIC_StaticPwrMgtTable.ctb
//

//
// Structures used by DAC_LoadDetectionTable.ctb
//
// DAC_LOAD_DETECTION_PARAMETERS.ucMisc
pub const DAC_LOAD_MISC_YPrPb: c_uint = 0x01;
//
// Structures used by DAC1EncoderControlTable.ctb and DAC2EncoderControlTable.ctb
//
// 1: setup and turn on encoder
// 7: ATOM_ENCODER_INIT Initialize DAC

//
// Structures used by DIG1EncoderControlTable
// DIG2EncoderControlTable
// ExternalEncoderControlTable
//
// [2] Link Select:
// =0: PHY linkA if bfLane<3
// =1: PHY linkB if bfLanes<3
// =0: PHY linkA+B if bfLanes=3
// [3] Transmitter Sel
// =0: UNIPHY or PCIEPHY
// =1: LVTMA
// =1: turn on encoder
// =0: DP   encoder
// =1: LVDS encoder
// =2: DVI  encoder
// =3: HDMI encoder
// =4: SDVO encoder

// ucConfig
pub const ATOM_ENCODER_CONFIG_DPLINKRATE_MASK: c_uint = 0x01;
pub const ATOM_ENCODER_CONFIG_DPLINKRATE_1_62GHZ: c_uint = 0x00;
pub const ATOM_ENCODER_CONFIG_DPLINKRATE_2_70GHZ: c_uint = 0x01;
pub const ATOM_ENCODER_CONFIG_DPLINKRATE_5_40GHZ: c_uint = 0x02;
pub const ATOM_ENCODER_CONFIG_LINK_SEL_MASK: c_uint = 0x04;
pub const ATOM_ENCODER_CONFIG_LINKA: c_uint = 0x00;
pub const ATOM_ENCODER_CONFIG_LINKB: c_uint = 0x04;

pub const ATOM_ENCODER_CONFIG_TRANSMITTER_SEL_MASK: c_uint = 0x08;
pub const ATOM_ENCODER_CONFIG_UNIPHY: c_uint = 0x00;
pub const ATOM_ENCODER_CONFIG_LVTMA: c_uint = 0x08;
pub const ATOM_ENCODER_CONFIG_TRANSMITTER1: c_uint = 0x00;
pub const ATOM_ENCODER_CONFIG_TRANSMITTER2: c_uint = 0x08;
pub const ATOM_ENCODER_CONFIG_DIGB: c_uint = 0x80			// VBIOS Internal use, outside SW should set this bit=0;
// ucAction
// ATOM_ENABLE:  Enable Encoder
// ATOM_DISABLE: Disable Encoder
// ucEncoderMode
pub const ATOM_ENCODER_MODE_DP: c_int = 0;
pub const ATOM_ENCODER_MODE_LVDS: c_int = 1;
pub const ATOM_ENCODER_MODE_DVI: c_int = 2;
pub const ATOM_ENCODER_MODE_HDMI: c_int = 3;
pub const ATOM_ENCODER_MODE_SDVO: c_int = 4;
pub const ATOM_ENCODER_MODE_DP_AUDIO: c_int = 5;
pub const ATOM_ENCODER_MODE_TV: c_int = 13;
pub const ATOM_ENCODER_MODE_CV: c_int = 14;
pub const ATOM_ENCODER_MODE_CRT: c_int = 15;
pub const ATOM_ENCODER_MODE_DVO: c_int = 16;

// =0: DP   encoder
// =1: LVDS encoder
// =2: DVI  encoder
// =3: HDMI encoder
// =4: SDVO encoder
// ucConfig
pub const ATOM_ENCODER_CONFIG_V2_DPLINKRATE_MASK: c_uint = 0x01;
pub const ATOM_ENCODER_CONFIG_V2_DPLINKRATE_1_62GHZ: c_uint = 0x00;
pub const ATOM_ENCODER_CONFIG_V2_DPLINKRATE_2_70GHZ: c_uint = 0x01;
pub const ATOM_ENCODER_CONFIG_V2_LINK_SEL_MASK: c_uint = 0x04;
pub const ATOM_ENCODER_CONFIG_V2_LINKA: c_uint = 0x00;
pub const ATOM_ENCODER_CONFIG_V2_LINKB: c_uint = 0x04;
pub const ATOM_ENCODER_CONFIG_V2_TRANSMITTER_SEL_MASK: c_uint = 0x18;
pub const ATOM_ENCODER_CONFIG_V2_TRANSMITTER1: c_uint = 0x00;
pub const ATOM_ENCODER_CONFIG_V2_TRANSMITTER2: c_uint = 0x08;
pub const ATOM_ENCODER_CONFIG_V2_TRANSMITTER3: c_uint = 0x10;
// ucAction:
// ATOM_DISABLE
// ATOM_ENABLE
pub const ATOM_ENCODER_CMD_DP_LINK_TRAINING_START: c_uint = 0x08;
pub const ATOM_ENCODER_CMD_DP_LINK_TRAINING_PATTERN1: c_uint = 0x09;
pub const ATOM_ENCODER_CMD_DP_LINK_TRAINING_PATTERN2: c_uint = 0x0a;
pub const ATOM_ENCODER_CMD_DP_LINK_TRAINING_PATTERN3: c_uint = 0x13;
pub const ATOM_ENCODER_CMD_DP_LINK_TRAINING_COMPLETE: c_uint = 0x0b;
pub const ATOM_ENCODER_CMD_DP_VIDEO_OFF: c_uint = 0x0c;
pub const ATOM_ENCODER_CMD_DP_VIDEO_ON: c_uint = 0x0d;
pub const ATOM_ENCODER_CMD_QUERY_DP_LINK_TRAINING_STATUS: c_uint = 0x0e;
pub const ATOM_ENCODER_CMD_SETUP: c_uint = 0x0f;
pub const ATOM_ENCODER_CMD_SETUP_PANEL_MODE: c_uint = 0x10;
// ucStatus
pub const ATOM_ENCODER_STATUS_LINK_TRAINING_COMPLETE: c_uint = 0x10;
pub const ATOM_ENCODER_STATUS_LINK_TRAINING_INCOMPLETE: c_uint = 0x00;
// ucTableFormatRevision=1
// ucTableContentRevision=3
// Following function ENABLE sub-function will be used by driver when TMDS/HDMI/LVDS is used, disable function will be used by driver

pub const ATOM_ENCODER_CONFIG_V3_DPLINKRATE_MASK: c_uint = 0x03;
pub const ATOM_ENCODER_CONFIG_V3_DPLINKRATE_1_62GHZ: c_uint = 0x00;
pub const ATOM_ENCODER_CONFIG_V3_DPLINKRATE_2_70GHZ: c_uint = 0x01;
pub const ATOM_ENCODER_CONFIG_V3_ENCODER_SEL: c_uint = 0x70;
pub const ATOM_ENCODER_CONFIG_V3_DIG0_ENCODER: c_uint = 0x00;
pub const ATOM_ENCODER_CONFIG_V3_DIG1_ENCODER: c_uint = 0x10;
pub const ATOM_ENCODER_CONFIG_V3_DIG2_ENCODER: c_uint = 0x20;
pub const ATOM_ENCODER_CONFIG_V3_DIG3_ENCODER: c_uint = 0x30;
pub const ATOM_ENCODER_CONFIG_V3_DIG4_ENCODER: c_uint = 0x40;
pub const ATOM_ENCODER_CONFIG_V3_DIG5_ENCODER: c_uint = 0x50;
// =0: DP   encoder
// =1: LVDS encoder
// =2: DVI  encoder
// =3: HDMI encoder
// =4: SDVO encoder
// =5: DP audio
// =0:     external DP
// =1:     internal DP2
// =0x11:  internal DP1 for NutMeg/Travis DP translator
// ucTableFormatRevision=1
// ucTableContentRevision=4
// start from NI
// Following function ENABLE sub-function will be used by driver when TMDS/HDMI/LVDS is used, disable function will be used by driver

pub const ATOM_ENCODER_CONFIG_V4_DPLINKRATE_MASK: c_uint = 0x03;
pub const ATOM_ENCODER_CONFIG_V4_DPLINKRATE_1_62GHZ: c_uint = 0x00;
pub const ATOM_ENCODER_CONFIG_V4_DPLINKRATE_2_70GHZ: c_uint = 0x01;
pub const ATOM_ENCODER_CONFIG_V4_DPLINKRATE_5_40GHZ: c_uint = 0x02;
pub const ATOM_ENCODER_CONFIG_V4_DPLINKRATE_3_24GHZ: c_uint = 0x03;
pub const ATOM_ENCODER_CONFIG_V4_ENCODER_SEL: c_uint = 0x70;
pub const ATOM_ENCODER_CONFIG_V4_DIG0_ENCODER: c_uint = 0x00;
pub const ATOM_ENCODER_CONFIG_V4_DIG1_ENCODER: c_uint = 0x10;
pub const ATOM_ENCODER_CONFIG_V4_DIG2_ENCODER: c_uint = 0x20;
pub const ATOM_ENCODER_CONFIG_V4_DIG3_ENCODER: c_uint = 0x30;
pub const ATOM_ENCODER_CONFIG_V4_DIG4_ENCODER: c_uint = 0x40;
pub const ATOM_ENCODER_CONFIG_V4_DIG5_ENCODER: c_uint = 0x50;
pub const ATOM_ENCODER_CONFIG_V4_DIG6_ENCODER: c_uint = 0x60;
// =0: DP   encoder
// =1: LVDS encoder
// =2: DVI  encoder
// =3: HDMI encoder
// =4: SDVO encoder
// =5: DP audio
// =0:     external DP
// =1:     internal DP2
// =0x11:  internal DP1 for NutMeg/Travis DP translator
// define ucBitPerColor:
pub const PANEL_BPC_UNDEFINE: c_uint = 0x00;
pub const PANEL_6BIT_PER_COLOR: c_uint = 0x01;
pub const PANEL_8BIT_PER_COLOR: c_uint = 0x02;
pub const PANEL_10BIT_PER_COLOR: c_uint = 0x03;
pub const PANEL_12BIT_PER_COLOR: c_uint = 0x04;
pub const PANEL_16BIT_PER_COLOR: c_uint = 0x05;
// define ucPanelMode
pub const DP_PANEL_MODE_EXTERNAL_DP_MODE: c_uint = 0x00;
pub const DP_PANEL_MODE_INTERNAL_DP2_MODE: c_uint = 0x01;
pub const DP_PANEL_MODE_INTERNAL_DP1_MODE: c_uint = 0x11;
//
// Structures used by UNIPHYTransmitterControlTable
// LVTMATransmitterControlTable
// DVOOutputControlTable
//
// [0]=0: 4 lane Link,
// =1: 8 lane Link ( Dual Links TMDS )
// [1]=0: InCoherent mode
// =1: Coherent Mode
// [2] Link Select:
// =0: PHY linkA   if bfLane<3
// =1: PHY linkB   if bfLanes<3
// =0: PHY linkA+B if bfLanes=3
// [5:4]PCIE lane Sel
// =0: lane 0~3 or 0~7
// =1: lane 4~7
// =2: lane 8~11 or 8~15
// =3: lane 12~15
// =1: turn on encoder

// ucInitInfo
pub const ATOM_TRAMITTER_INITINFO_CONNECTOR_MASK: c_uint = 0x00ff;
// ucConfig
pub const ATOM_TRANSMITTER_CONFIG_8LANE_LINK: c_uint = 0x01;
pub const ATOM_TRANSMITTER_CONFIG_COHERENT: c_uint = 0x02;
pub const ATOM_TRANSMITTER_CONFIG_LINK_SEL_MASK: c_uint = 0x04;
pub const ATOM_TRANSMITTER_CONFIG_LINKA: c_uint = 0x00;
pub const ATOM_TRANSMITTER_CONFIG_LINKB: c_uint = 0x04;
pub const ATOM_TRANSMITTER_CONFIG_LINKA_B: c_uint = 0x00;
pub const ATOM_TRANSMITTER_CONFIG_LINKB_A: c_uint = 0x04;
pub const ATOM_TRANSMITTER_CONFIG_ENCODER_SEL_MASK: c_uint = 0x08			// only used when ATOM_TRANSMITTER_ACTION_ENABLE;
pub const ATOM_TRANSMITTER_CONFIG_DIG1_ENCODER: c_uint = 0x00				// only used when ATOM_TRANSMITTER_ACTION_ENABLE;
pub const ATOM_TRANSMITTER_CONFIG_DIG2_ENCODER: c_uint = 0x08				// only used when ATOM_TRANSMITTER_ACTION_ENABLE;
pub const ATOM_TRANSMITTER_CONFIG_CLKSRC_MASK: c_uint = 0x30;
pub const ATOM_TRANSMITTER_CONFIG_CLKSRC_PPLL: c_uint = 0x00;
pub const ATOM_TRANSMITTER_CONFIG_CLKSRC_PCIE: c_uint = 0x20;
pub const ATOM_TRANSMITTER_CONFIG_CLKSRC_XTALIN: c_uint = 0x30;
pub const ATOM_TRANSMITTER_CONFIG_LANE_SEL_MASK: c_uint = 0xc0;
pub const ATOM_TRANSMITTER_CONFIG_LANE_0_3: c_uint = 0x00;
pub const ATOM_TRANSMITTER_CONFIG_LANE_0_7: c_uint = 0x00;
pub const ATOM_TRANSMITTER_CONFIG_LANE_4_7: c_uint = 0x40;
pub const ATOM_TRANSMITTER_CONFIG_LANE_8_11: c_uint = 0x80;
pub const ATOM_TRANSMITTER_CONFIG_LANE_8_15: c_uint = 0x80;
pub const ATOM_TRANSMITTER_CONFIG_LANE_12_15: c_uint = 0xc0;
// ucAction
pub const ATOM_TRANSMITTER_ACTION_DISABLE: c_int = 0;
pub const ATOM_TRANSMITTER_ACTION_ENABLE: c_int = 1;
pub const ATOM_TRANSMITTER_ACTION_LCD_BLOFF: c_int = 2;
pub const ATOM_TRANSMITTER_ACTION_LCD_BLON: c_int = 3;
pub const ATOM_TRANSMITTER_ACTION_BL_BRIGHTNESS_CONTROL: c_int = 4;
pub const ATOM_TRANSMITTER_ACTION_LCD_SELFTEST_START: c_int = 5;
pub const ATOM_TRANSMITTER_ACTION_LCD_SELFTEST_STOP: c_int = 6;
pub const ATOM_TRANSMITTER_ACTION_INIT: c_int = 7;
pub const ATOM_TRANSMITTER_ACTION_DISABLE_OUTPUT: c_int = 8;
pub const ATOM_TRANSMITTER_ACTION_ENABLE_OUTPUT: c_int = 9;
pub const ATOM_TRANSMITTER_ACTION_SETUP: c_int = 10;
pub const ATOM_TRANSMITTER_ACTION_SETUP_VSEMPH: c_int = 11;
pub const ATOM_TRANSMITTER_ACTION_POWER_ON: c_int = 12;
pub const ATOM_TRANSMITTER_ACTION_POWER_OFF: c_int = 13;
// Following are used for DigTransmitterControlTable ver1.2

// =1 Dig Transmitter 2 ( Uniphy CD )
// =2 Dig Transmitter 3 ( Uniphy EF )
// =1: Uniphy LINKB or D or F when fDualLinkConnector=0. when fDualLinkConnector=1, it means master link of dual link is B or D or F

// =1: Uniphy LINKB or D or F when fDualLinkConnector=0. when fDualLinkConnector=1, it means master link of dual link is B or D or F
// =1 Dig Transmitter 2 ( Uniphy CD )
// =2 Dig Transmitter 3 ( Uniphy EF )

// ucConfig
// Bit0
pub const ATOM_TRANSMITTER_CONFIG_V2_DUAL_LINK_CONNECTOR: c_uint = 0x01;
// Bit1
pub const ATOM_TRANSMITTER_CONFIG_V2_COHERENT: c_uint = 0x02;
// Bit2
pub const ATOM_TRANSMITTER_CONFIG_V2_LINK_SEL_MASK: c_uint = 0x04;
pub const ATOM_TRANSMITTER_CONFIG_V2_LINKA: c_uint = 0x00;
pub const ATOM_TRANSMITTER_CONFIG_V2_LINKB: c_uint = 0x04;
// Bit3
pub const ATOM_TRANSMITTER_CONFIG_V2_ENCODER_SEL_MASK: c_uint = 0x08;
pub const ATOM_TRANSMITTER_CONFIG_V2_DIG1_ENCODER: c_uint = 0x00				// only used when ucAction == ATOM_TRANSMITTER_ACTION_ENABLE or ATOM_TRANSMITTER_ACTION_SETUP;
pub const ATOM_TRANSMITTER_CONFIG_V2_DIG2_ENCODER: c_uint = 0x08				// only used when ucAction == ATOM_TRANSMITTER_ACTION_ENABLE or ATOM_TRANSMITTER_ACTION_SETUP;
// Bit4
pub const ATOM_TRASMITTER_CONFIG_V2_DP_CONNECTOR: c_uint = 0x10;
// Bit7:6
pub const ATOM_TRANSMITTER_CONFIG_V2_TRANSMITTER_SEL_MASK: c_uint = 0xC0;
pub const ATOM_TRANSMITTER_CONFIG_V2_TRANSMITTER1: c_uint = 0x00	//AB;
pub const ATOM_TRANSMITTER_CONFIG_V2_TRANSMITTER2: c_uint = 0x40	//CD;
pub const ATOM_TRANSMITTER_CONFIG_V2_TRANSMITTER3: c_uint = 0x80	//EF;

// =1 Dig Transmitter 2 ( Uniphy CD )
// =2 Dig Transmitter 3 ( Uniphy EF )
// =1: Uniphy LINKB or D or F when fDualLinkConnector=0. when fDualLinkConnector=1, it means master link of dual link is B or D or F

// =1: Uniphy LINKB or D or F when fDualLinkConnector=0. when fDualLinkConnector=1, it means master link of dual link is B or D or F
// =1 Dig Transmitter 2 ( Uniphy CD )
// =2 Dig Transmitter 3 ( Uniphy EF )

// ucConfig
// Bit0
pub const ATOM_TRANSMITTER_CONFIG_V3_DUAL_LINK_CONNECTOR: c_uint = 0x01;
// Bit1
pub const ATOM_TRANSMITTER_CONFIG_V3_COHERENT: c_uint = 0x02;
// Bit2
pub const ATOM_TRANSMITTER_CONFIG_V3_LINK_SEL_MASK: c_uint = 0x04;
pub const ATOM_TRANSMITTER_CONFIG_V3_LINKA: c_uint = 0x00;
pub const ATOM_TRANSMITTER_CONFIG_V3_LINKB: c_uint = 0x04;
// Bit3
pub const ATOM_TRANSMITTER_CONFIG_V3_ENCODER_SEL_MASK: c_uint = 0x08;
pub const ATOM_TRANSMITTER_CONFIG_V3_DIG1_ENCODER: c_uint = 0x00;
pub const ATOM_TRANSMITTER_CONFIG_V3_DIG2_ENCODER: c_uint = 0x08;
// Bit5:4
pub const ATOM_TRASMITTER_CONFIG_V3_REFCLK_SEL_MASK: c_uint = 0x30;
pub const ATOM_TRASMITTER_CONFIG_V3_P1PLL: c_uint = 0x00;
pub const ATOM_TRASMITTER_CONFIG_V3_P2PLL: c_uint = 0x10;
pub const ATOM_TRASMITTER_CONFIG_V3_REFCLK_SRC_EXT: c_uint = 0x20;
// Bit7:6
pub const ATOM_TRANSMITTER_CONFIG_V3_TRANSMITTER_SEL_MASK: c_uint = 0xC0;
pub const ATOM_TRANSMITTER_CONFIG_V3_TRANSMITTER1: c_uint = 0x00	//AB;
pub const ATOM_TRANSMITTER_CONFIG_V3_TRANSMITTER2: c_uint = 0x40	//CD;
pub const ATOM_TRANSMITTER_CONFIG_V3_TRANSMITTER3: c_uint = 0x80	//EF;
//
// Structures used by UNIPHYTransmitterControlTable V1.4
// ASIC Families: NI
// ucTableFormatRevision=1
// ucTableContentRevision=4
//

// =1 Dig Transmitter 2 ( Uniphy CD )
// =2 Dig Transmitter 3 ( Uniphy EF )
// =1: Uniphy LINKB or D or F when fDualLinkConnector=0. when fDualLinkConnector=1, it means master link of dual link is B or D or F

// =1: Uniphy LINKB or D or F when fDualLinkConnector=0. when fDualLinkConnector=1, it means master link of dual link is B or D or F
// =1 Dig Transmitter 2 ( Uniphy CD )
// =2 Dig Transmitter 3 ( Uniphy EF )

// ucConfig
// Bit0
pub const ATOM_TRANSMITTER_CONFIG_V4_DUAL_LINK_CONNECTOR: c_uint = 0x01;
// Bit1
pub const ATOM_TRANSMITTER_CONFIG_V4_COHERENT: c_uint = 0x02;
// Bit2
pub const ATOM_TRANSMITTER_CONFIG_V4_LINK_SEL_MASK: c_uint = 0x04;
pub const ATOM_TRANSMITTER_CONFIG_V4_LINKA: c_uint = 0x00;
pub const ATOM_TRANSMITTER_CONFIG_V4_LINKB: c_uint = 0x04;
// Bit3
pub const ATOM_TRANSMITTER_CONFIG_V4_ENCODER_SEL_MASK: c_uint = 0x08;
pub const ATOM_TRANSMITTER_CONFIG_V4_DIG1_ENCODER: c_uint = 0x00;
pub const ATOM_TRANSMITTER_CONFIG_V4_DIG2_ENCODER: c_uint = 0x08;
// Bit5:4
pub const ATOM_TRANSMITTER_CONFIG_V4_REFCLK_SEL_MASK: c_uint = 0x30;
pub const ATOM_TRANSMITTER_CONFIG_V4_P1PLL: c_uint = 0x00;
pub const ATOM_TRANSMITTER_CONFIG_V4_P2PLL: c_uint = 0x10;
pub const ATOM_TRANSMITTER_CONFIG_V4_DCPLL: c_uint = 0x20   // New in _V4;
pub const ATOM_TRANSMITTER_CONFIG_V4_REFCLK_SRC_EXT: c_uint = 0x30   // Changed comparing to V3;
// Bit7:6
pub const ATOM_TRANSMITTER_CONFIG_V4_TRANSMITTER_SEL_MASK: c_uint = 0xC0;
pub const ATOM_TRANSMITTER_CONFIG_V4_TRANSMITTER1: c_uint = 0x00	//AB;
pub const ATOM_TRANSMITTER_CONFIG_V4_TRANSMITTER2: c_uint = 0x40	//CD;
pub const ATOM_TRANSMITTER_CONFIG_V4_TRANSMITTER3: c_uint = 0x80	//EF;

// ucPhyId
pub const ATOM_PHY_ID_UNIPHYA: c_int = 0;
pub const ATOM_PHY_ID_UNIPHYB: c_int = 1;
pub const ATOM_PHY_ID_UNIPHYC: c_int = 2;
pub const ATOM_PHY_ID_UNIPHYD: c_int = 3;
pub const ATOM_PHY_ID_UNIPHYE: c_int = 4;
pub const ATOM_PHY_ID_UNIPHYF: c_int = 5;
pub const ATOM_PHY_ID_UNIPHYG: c_int = 6;
// ucDigEncoderSel
pub const ATOM_TRANMSITTER_V5__DIGA_SEL: c_uint = 0x01;
pub const ATOM_TRANMSITTER_V5__DIGB_SEL: c_uint = 0x02;
pub const ATOM_TRANMSITTER_V5__DIGC_SEL: c_uint = 0x04;
pub const ATOM_TRANMSITTER_V5__DIGD_SEL: c_uint = 0x08;
pub const ATOM_TRANMSITTER_V5__DIGE_SEL: c_uint = 0x10;
pub const ATOM_TRANMSITTER_V5__DIGF_SEL: c_uint = 0x20;
pub const ATOM_TRANMSITTER_V5__DIGG_SEL: c_uint = 0x40;
// ucDigMode
pub const ATOM_TRANSMITTER_DIGMODE_V5_DP: c_int = 0;
pub const ATOM_TRANSMITTER_DIGMODE_V5_LVDS: c_int = 1;
pub const ATOM_TRANSMITTER_DIGMODE_V5_DVI: c_int = 2;
pub const ATOM_TRANSMITTER_DIGMODE_V5_HDMI: c_int = 3;
pub const ATOM_TRANSMITTER_DIGMODE_V5_SDVO: c_int = 4;
pub const ATOM_TRANSMITTER_DIGMODE_V5_DP_MST: c_int = 5;
// ucDPLaneSet
pub const DP_LANE_SET__0DB_0_4V: c_uint = 0x00;
pub const DP_LANE_SET__0DB_0_6V: c_uint = 0x01;
pub const DP_LANE_SET__0DB_0_8V: c_uint = 0x02;
pub const DP_LANE_SET__0DB_1_2V: c_uint = 0x03;
pub const DP_LANE_SET__3_5DB_0_4V: c_uint = 0x08;
pub const DP_LANE_SET__3_5DB_0_6V: c_uint = 0x09;
pub const DP_LANE_SET__3_5DB_0_8V: c_uint = 0x0a;
pub const DP_LANE_SET__6DB_0_4V: c_uint = 0x10;
pub const DP_LANE_SET__6DB_0_6V: c_uint = 0x11;
pub const DP_LANE_SET__9_5DB_0_4V: c_uint = 0x18;
// ATOM_DIG_TRANSMITTER_CONFIG_V5 asConfig;
// Bit1
pub const ATOM_TRANSMITTER_CONFIG_V5_COHERENT: c_uint = 0x02;
// Bit3:2
pub const ATOM_TRANSMITTER_CONFIG_V5_REFCLK_SEL_MASK: c_uint = 0x0c;
pub const ATOM_TRANSMITTER_CONFIG_V5_REFCLK_SEL_SHIFT: c_uint = 0x02;
pub const ATOM_TRANSMITTER_CONFIG_V5_P1PLL: c_uint = 0x00;
pub const ATOM_TRANSMITTER_CONFIG_V5_P2PLL: c_uint = 0x04;
pub const ATOM_TRANSMITTER_CONFIG_V5_P0PLL: c_uint = 0x08;
pub const ATOM_TRANSMITTER_CONFIG_V5_REFCLK_SRC_EXT: c_uint = 0x0c;
// Bit6:4
pub const ATOM_TRANSMITTER_CONFIG_V5_HPD_SEL_MASK: c_uint = 0x70;
pub const ATOM_TRANSMITTER_CONFIG_V5_HPD_SEL_SHIFT: c_uint = 0x04;
pub const ATOM_TRANSMITTER_CONFIG_V5_NO_HPD_SEL: c_uint = 0x00;
pub const ATOM_TRANSMITTER_CONFIG_V5_HPD1_SEL: c_uint = 0x10;
pub const ATOM_TRANSMITTER_CONFIG_V5_HPD2_SEL: c_uint = 0x20;
pub const ATOM_TRANSMITTER_CONFIG_V5_HPD3_SEL: c_uint = 0x30;
pub const ATOM_TRANSMITTER_CONFIG_V5_HPD4_SEL: c_uint = 0x40;
pub const ATOM_TRANSMITTER_CONFIG_V5_HPD5_SEL: c_uint = 0x50;
pub const ATOM_TRANSMITTER_CONFIG_V5_HPD6_SEL: c_uint = 0x60;

//
// Structures used by ExternalEncoderControlTable V1.3
// ASIC Families: Evergreen, Llano, NI
// ucTableFormatRevision=1
// ucTableContentRevision=3
//
// ucAction
pub const EXTERNAL_ENCODER_ACTION_V3_DISABLE_OUTPUT: c_uint = 0x00;
pub const EXTERNAL_ENCODER_ACTION_V3_ENABLE_OUTPUT: c_uint = 0x01;
pub const EXTERNAL_ENCODER_ACTION_V3_ENCODER_INIT: c_uint = 0x07;
pub const EXTERNAL_ENCODER_ACTION_V3_ENCODER_SETUP: c_uint = 0x0f;
pub const EXTERNAL_ENCODER_ACTION_V3_ENCODER_BLANKING_OFF: c_uint = 0x10;
pub const EXTERNAL_ENCODER_ACTION_V3_ENCODER_BLANKING: c_uint = 0x11;
pub const EXTERNAL_ENCODER_ACTION_V3_DACLOAD_DETECTION: c_uint = 0x12;
pub const EXTERNAL_ENCODER_ACTION_V3_DDC_SETUP: c_uint = 0x14;
// ucConfig
pub const EXTERNAL_ENCODER_CONFIG_V3_DPLINKRATE_MASK: c_uint = 0x03;
pub const EXTERNAL_ENCODER_CONFIG_V3_DPLINKRATE_1_62GHZ: c_uint = 0x00;
pub const EXTERNAL_ENCODER_CONFIG_V3_DPLINKRATE_2_70GHZ: c_uint = 0x01;
pub const EXTERNAL_ENCODER_CONFIG_V3_DPLINKRATE_5_40GHZ: c_uint = 0x02;
pub const EXTERNAL_ENCODER_CONFIG_V3_ENCODER_SEL_MASK: c_uint = 0x70;
pub const EXTERNAL_ENCODER_CONFIG_V3_ENCODER1: c_uint = 0x00;
pub const EXTERNAL_ENCODER_CONFIG_V3_ENCODER2: c_uint = 0x10;
pub const EXTERNAL_ENCODER_CONFIG_V3_ENCODER3: c_uint = 0x20;
//
// Structures used by DAC1OuputControlTable
// DAC2OuputControlTable
// LVTMAOutputControlTable  (Before DEC30)
// TMDSAOutputControlTable  (Before DEC30)
//
// When the display is LCD, in addition to above:
// ATOM_LCD_BLOFF|| ATOM_LCD_BLON ||ATOM_LCD_BL_BRIGHTNESS_CONTROL||ATOM_LCD_SELFTEST_START||
// ATOM_LCD_SELFTEST_STOP

//
// Structures used by BlankCRTCTable
//

//
// Structures used by EnableCRTCTable
// EnableCRTCMemReqTable
// UpdateCRTC_DoubleBufferRegistersTable
//

//
// Structures used by SetCRTC_OverScanTable
//

//
// Structures used by SetCRTC_ReplicationTable
//

//
// Structures used by SelectCRTC_SourceTable
//

// ucEncoderID
// #define ASIC_INT_DAC1_ENCODER_ID    						0x00
// #define ASIC_INT_TV_ENCODER_ID									0x02
// #define ASIC_INT_DIG1_ENCODER_ID								0x03
// #define ASIC_INT_DAC2_ENCODER_ID								0x04
// #define ASIC_EXT_TV_ENCODER_ID									0x06
// #define ASIC_INT_DVO_ENCODER_ID									0x07
// #define ASIC_INT_DIG2_ENCODER_ID								0x09
// #define ASIC_EXT_DIG_ENCODER_ID									0x05
// ucEncodeMode
// #define ATOM_ENCODER_MODE_DP										0
// #define ATOM_ENCODER_MODE_LVDS									1
// #define ATOM_ENCODER_MODE_DVI										2
// #define ATOM_ENCODER_MODE_HDMI									3
// #define ATOM_ENCODER_MODE_SDVO									4
// #define ATOM_ENCODER_MODE_TV										13
// #define ATOM_ENCODER_MODE_CV										14
// #define ATOM_ENCODER_MODE_CRT										15
//
// Structures used by SetPixelClockTable
// GetPixelClockTable
//
// Major revision=1., Minor revision=1
// 0 means disable PPLL
// Major revision=1., Minor revision=2, add ucMiscIfno
// ucMiscInfo:
pub const MISC_FORCE_REPROG_PIXEL_CLOCK: c_uint = 0x1;
pub const MISC_DEVICE_INDEX_MASK: c_uint = 0xF0;
pub const MISC_DEVICE_INDEX_SHIFT: c_int = 4;
// 0 means disable PPLL
// Major revision=1., Minor revision=3, structure/definition change
// ucEncoderMode:
// ATOM_ENCODER_MODE_DP
// ATOM_ENOCDER_MODE_LVDS
// ATOM_ENOCDER_MODE_DVI
// ATOM_ENOCDER_MODE_HDMI
// ATOM_ENOCDER_MODE_SDVO
// ATOM_ENCODER_MODE_TV										13
// ATOM_ENCODER_MODE_CV										14
// ATOM_ENCODER_MODE_CRT										15
// ucDVOConfig
// #define DVO_ENCODER_CONFIG_RATE_SEL							0x01
// #define DVO_ENCODER_CONFIG_DDR_SPEED						0x00
// #define DVO_ENCODER_CONFIG_SDR_SPEED						0x01
// #define DVO_ENCODER_CONFIG_OUTPUT_SEL						0x0c
// #define DVO_ENCODER_CONFIG_LOW12BIT							0x00
// #define DVO_ENCODER_CONFIG_UPPER12BIT						0x04
// #define DVO_ENCODER_CONFIG_24BIT								0x08
// ucMiscInfo: also changed, see below
pub const PIXEL_CLOCK_MISC_FORCE_PROG_PPLL: c_uint = 0x01;
pub const PIXEL_CLOCK_MISC_VGA_MODE: c_uint = 0x02;
pub const PIXEL_CLOCK_MISC_CRTC_SEL_MASK: c_uint = 0x04;
pub const PIXEL_CLOCK_MISC_CRTC_SEL_CRTC1: c_uint = 0x00;
pub const PIXEL_CLOCK_MISC_CRTC_SEL_CRTC2: c_uint = 0x04;
pub const PIXEL_CLOCK_MISC_USE_ENGINE_FOR_DISPCLK: c_uint = 0x08;
pub const PIXEL_CLOCK_MISC_REF_DIV_SRC: c_uint = 0x10;
// V1.4 for RoadRunner
pub const PIXEL_CLOCK_V4_MISC_SS_ENABLE: c_uint = 0x10;
pub const PIXEL_CLOCK_V4_MISC_COHERENT_MODE: c_uint = 0x20;
// 0 means disable PPLL. For VGA PPLL,make sure this value is not 0.
// bit[3]=0:use PPLL for dispclk source, =1: use engine clock for dispclock source
// bit[4]=0:use XTALIN as the source of reference divider,=1 use the pre-defined clock as the source of reference divider

// drive the pixel clock. not used for DCPLL case.
// 0 means disable PPLL/DCPLL.
// indicate which graphic encoder will be used.
// bit[1]= when VGA timing is used.
// bit[3:2]= HDMI panel bit depth: =0: 24bpp =1:30bpp, =2:32bpp
// bit[4]= RefClock source for PPLL.
// =0: XTLAIN( default mode )
// =1: other external clock source, which is pre-defined
// by VBIOS depend on the feature required.
// bit[7:5]: reserved.
pub const PIXEL_CLOCK_V5_MISC_FORCE_PROG_PPLL: c_uint = 0x01;
pub const PIXEL_CLOCK_V5_MISC_VGA_MODE: c_uint = 0x02;
pub const PIXEL_CLOCK_V5_MISC_HDMI_BPP_MASK: c_uint = 0x0c;
pub const PIXEL_CLOCK_V5_MISC_HDMI_24BPP: c_uint = 0x00;
pub const PIXEL_CLOCK_V5_MISC_HDMI_30BPP: c_uint = 0x04;
pub const PIXEL_CLOCK_V5_MISC_HDMI_32BPP: c_uint = 0x08;
pub const PIXEL_CLOCK_V5_MISC_REF_DIV_SRC: c_uint = 0x10;

// drive the pixel clock. not used for DCPLL case.
// 0 means disable PPLL/DCPLL. Expanded to 24 bits comparing to previous version.

// 0 means disable PPLL/DCPLL. Expanded to 24 bits comparing to previous version.
// drive the pixel clock. not used for DCPLL case.

// indicate which graphic encoder will be used.
// bit[1]= when VGA timing is used.
// bit[3:2]= HDMI panel bit depth: =0: 24bpp =1:30bpp, =2:32bpp
// bit[4]= RefClock source for PPLL.
// =0: XTLAIN( default mode )
// =1: other external clock source, which is pre-defined
// by VBIOS depend on the feature required.
// bit[7:5]: reserved.
pub const PIXEL_CLOCK_V6_MISC_FORCE_PROG_PPLL: c_uint = 0x01;
pub const PIXEL_CLOCK_V6_MISC_VGA_MODE: c_uint = 0x02;
pub const PIXEL_CLOCK_V6_MISC_HDMI_BPP_MASK: c_uint = 0x0c;
pub const PIXEL_CLOCK_V6_MISC_HDMI_24BPP: c_uint = 0x00;
pub const PIXEL_CLOCK_V6_MISC_HDMI_36BPP: c_uint = 0x04;
pub const PIXEL_CLOCK_V6_MISC_HDMI_36BPP_V6: c_uint = 0x08    //for V6, the correct defintion for 36bpp should be 2 for 36bpp(2:1);
pub const PIXEL_CLOCK_V6_MISC_HDMI_30BPP: c_uint = 0x08;
pub const PIXEL_CLOCK_V6_MISC_HDMI_30BPP_V6: c_uint = 0x04    //for V6, the correct defintion for 30bpp should be 1 for 36bpp(5:4);
pub const PIXEL_CLOCK_V6_MISC_HDMI_48BPP: c_uint = 0x0c;
pub const PIXEL_CLOCK_V6_MISC_REF_DIV_SRC: c_uint = 0x10;
pub const PIXEL_CLOCK_V6_MISC_GEN_DPREFCLK: c_uint = 0x40;
//
// Structures used by AdjustDisplayPllTable
//
pub const ADJUST_DISPLAY_CONFIG_SS_ENABLE: c_uint = 0x10;

// usDispPllConfig v1.2 for RoadRunner
pub const DISPPLL_CONFIG_DVO_RATE_SEL: c_uint = 0x0001     // need only when ucTransmitterID = DVO;
pub const DISPPLL_CONFIG_DVO_DDR_SPEED: c_uint = 0x0000     // need only when ucTransmitterID = DVO;
pub const DISPPLL_CONFIG_DVO_SDR_SPEED: c_uint = 0x0001     // need only when ucTransmitterID = DVO;
pub const DISPPLL_CONFIG_DVO_OUTPUT_SEL: c_uint = 0x000c     // need only when ucTransmitterID = DVO;
pub const DISPPLL_CONFIG_DVO_LOW12BIT: c_uint = 0x0000     // need only when ucTransmitterID = DVO;
pub const DISPPLL_CONFIG_DVO_UPPER12BIT: c_uint = 0x0004     // need only when ucTransmitterID = DVO;
pub const DISPPLL_CONFIG_DVO_24BIT: c_uint = 0x0008     // need only when ucTransmitterID = DVO;
pub const DISPPLL_CONFIG_SS_ENABLE: c_uint = 0x0010     // Only used when ucEncoderMode = DP or LVDS;
pub const DISPPLL_CONFIG_COHERENT_MODE: c_uint = 0x0020     // Only used when ucEncoderMode = TMDS or HDMI;
pub const DISPPLL_CONFIG_DUAL_LINK: c_uint = 0x0040     // Only used when ucEncoderMode = TMDS or LVDS;
//
// Structures used by EnableYUVTable
//

//
// Structures used by GetMemoryClockTable
//

//
// Structures used by GetEngineClockTable
//

//
// Following Structures and constant may be obsolete
//
// Maxium 8 bytes,the data read in will be placed in the parameter space.
// Read operaion successeful when the paramter space is non-zero, otherwise read operation failed
// WHen use input:  lower byte as 'byte to read':currently limited to 128byte or 1byte

pub const ATOM_WRITE_I2C_FORMAT_PSOFFSET_PSDATABYTE: c_int = 0;
pub const ATOM_WRITE_I2C_FORMAT_PSOFFSET_PSTWODATABYTES: c_int = 1;
pub const ATOM_WRITE_I2C_FORMAT_PSCOUNTER_PSOFFSET_IDDATABLOCK: c_int = 2;
pub const ATOM_WRITE_I2C_FORMAT_PSCOUNTER_IDOFFSET_PLUS_IDDATABLOCK: c_int = 3;
pub const ATOM_WRITE_I2C_FORMAT_IDCOUNTER_IDOFFSET_IDDATABLOCK: c_int = 4;
// Upper portion of usByteOffset is Format of data
// 1bytePS+offsetPS
// 2bytesPS+offsetPS
// blockID+offsetPS
// blockID+offsetID
// blockID+counterID+offsetID

//

//
// Structures used by PowerConnectorDetectionTable
//
// LVDS SS Command Table Definitions
//
// Structures used by EnableSpreadSpectrumOnPPLLTable
//
// ucTableFormatRevision=1,ucTableContentRevision=2
// This new structure is based on ENABLE_LVDS_SS_PARAMETERS but expands to SS on PPLL, so other devices can use SS.
// Bit[1]: 1-Ext. 0-Int.
// Bit[3:2]: =0 P1PLL =1 P2PLL =2 DCPLL
// Bits[7:4] reserved
pub const ATOM_PPLL_SS_TYPE_V2_DOWN_SPREAD: c_uint = 0x00;
pub const ATOM_PPLL_SS_TYPE_V2_CENTRE_SPREAD: c_uint = 0x01;
pub const ATOM_PPLL_SS_TYPE_V2_EXT_SPREAD: c_uint = 0x02;
pub const ATOM_PPLL_SS_TYPE_V2_PPLL_SEL_MASK: c_uint = 0x0c;
pub const ATOM_PPLL_SS_TYPE_V2_P1PLL: c_uint = 0x00;
pub const ATOM_PPLL_SS_TYPE_V2_P2PLL: c_uint = 0x04;
pub const ATOM_PPLL_SS_TYPE_V2_DCPLL: c_uint = 0x08;
pub const ATOM_PPLL_SS_AMOUNT_V2_FBDIV_MASK: c_uint = 0x00FF;
pub const ATOM_PPLL_SS_AMOUNT_V2_FBDIV_SHIFT: c_int = 0;
pub const ATOM_PPLL_SS_AMOUNT_V2_NFRAC_MASK: c_uint = 0x0F00;
pub const ATOM_PPLL_SS_AMOUNT_V2_NFRAC_SHIFT: c_int = 8;
// Used by DCE5.0
// Bit[1]: 1-Ext. 0-Int.
// Bit[3:2]: =0 P1PLL =1 P2PLL =2 DCPLL
// Bits[7:4] reserved
pub const ATOM_PPLL_SS_TYPE_V3_DOWN_SPREAD: c_uint = 0x00;
pub const ATOM_PPLL_SS_TYPE_V3_CENTRE_SPREAD: c_uint = 0x01;
pub const ATOM_PPLL_SS_TYPE_V3_EXT_SPREAD: c_uint = 0x02;
pub const ATOM_PPLL_SS_TYPE_V3_PPLL_SEL_MASK: c_uint = 0x0c;
pub const ATOM_PPLL_SS_TYPE_V3_P1PLL: c_uint = 0x00;
pub const ATOM_PPLL_SS_TYPE_V3_P2PLL: c_uint = 0x04;
pub const ATOM_PPLL_SS_TYPE_V3_DCPLL: c_uint = 0x08;

pub const ATOM_PPLL_SS_AMOUNT_V3_FBDIV_MASK: c_uint = 0x00FF;
pub const ATOM_PPLL_SS_AMOUNT_V3_FBDIV_SHIFT: c_int = 0;
pub const ATOM_PPLL_SS_AMOUNT_V3_NFRAC_MASK: c_uint = 0x0F00;
pub const ATOM_PPLL_SS_AMOUNT_V3_NFRAC_SHIFT: c_int = 8;

//

//
// Structures used by ###
//

// LVDS and other encoder command table definitions
//
// Structures used by LVDSEncoderControlTable   (Before DCE30)
// LVTMAEncoderControlTable  (Before DCE30)
// TMDSAEncoderControlTable  (Before DCE30)
//
// =1: Enable dual link
// Bit1=0: 666RGB
// =1: 888RGB
// 1: setup and turn on encoder

// ucTableFormatRevision=1,ucTableContentRevision=2
// 1: setup and turn on encoder
// =1: Enable truncate
// bit4=0: 666RGB
// =1: 888RGB
// =1: Enable spatial dithering
// bit4=0: 666RGB
// =1: 888RGB
// =1: Enable temporal dithering
// bit4=0: 666RGB
// =1: 888RGB
// bit5=0: Gray level 2
// =1: Gray level 4
// =1: 25FRC_SEL pattern F
// bit6:5=0: 50FRC_SEL pattern A
// =1: 50FRC_SEL pattern B
// =2: 50FRC_SEL pattern C
// =3: 50FRC_SEL pattern D
// bit7=0: 75FRC_SEL pattern E
// =1: 75FRC_SEL pattern F

//
// Structures used by ###
//

//
// Structures used by DVOEncoderControlTable
//
// ucTableFormatRevision=1,ucTableContentRevision=3
// ucDVOConfig:
pub const DVO_ENCODER_CONFIG_RATE_SEL: c_uint = 0x01;
pub const DVO_ENCODER_CONFIG_DDR_SPEED: c_uint = 0x00;
pub const DVO_ENCODER_CONFIG_SDR_SPEED: c_uint = 0x01;
pub const DVO_ENCODER_CONFIG_OUTPUT_SEL: c_uint = 0x0c;
pub const DVO_ENCODER_CONFIG_LOW12BIT: c_uint = 0x00;
pub const DVO_ENCODER_CONFIG_UPPER12BIT: c_uint = 0x04;
pub const DVO_ENCODER_CONFIG_24BIT: c_uint = 0x08;

// ucTableFormatRevision=1
// ucTableContentRevision=3 structure is not changed but usMisc add bit 1 as another input for
// bit1=0: non-coherent mode
// =1: coherent mode
// ==========================================================================================
// Only change is here next time when changing encoder parameter definitions again!

// ==========================================================================================
pub const PANEL_ENCODER_MISC_DUAL: c_uint = 0x01;
pub const PANEL_ENCODER_MISC_COHERENT: c_uint = 0x02;
pub const PANEL_ENCODER_MISC_TMDS_LINKB: c_uint = 0x04;
pub const PANEL_ENCODER_MISC_HDMI_TYPE: c_uint = 0x08;

pub const PANEL_ENCODER_TRUNCATE_EN: c_uint = 0x01;
pub const PANEL_ENCODER_TRUNCATE_DEPTH: c_uint = 0x10;
pub const PANEL_ENCODER_SPATIAL_DITHER_EN: c_uint = 0x01;
pub const PANEL_ENCODER_SPATIAL_DITHER_DEPTH: c_uint = 0x10;
pub const PANEL_ENCODER_TEMPORAL_DITHER_EN: c_uint = 0x01;
pub const PANEL_ENCODER_TEMPORAL_DITHER_DEPTH: c_uint = 0x10;
pub const PANEL_ENCODER_TEMPORAL_LEVEL_4: c_uint = 0x20;
pub const PANEL_ENCODER_25FRC_MASK: c_uint = 0x10;
pub const PANEL_ENCODER_25FRC_E: c_uint = 0x00;
pub const PANEL_ENCODER_25FRC_F: c_uint = 0x10;
pub const PANEL_ENCODER_50FRC_MASK: c_uint = 0x60;
pub const PANEL_ENCODER_50FRC_A: c_uint = 0x00;
pub const PANEL_ENCODER_50FRC_B: c_uint = 0x20;
pub const PANEL_ENCODER_50FRC_C: c_uint = 0x40;
pub const PANEL_ENCODER_50FRC_D: c_uint = 0x60;
pub const PANEL_ENCODER_75FRC_MASK: c_uint = 0x80;
pub const PANEL_ENCODER_75FRC_E: c_uint = 0x00;
pub const PANEL_ENCODER_75FRC_F: c_uint = 0x80;
//
// Structures used by SetVoltageTable
//
pub const SET_VOLTAGE_TYPE_ASIC_VDDC: c_int = 1;
pub const SET_VOLTAGE_TYPE_ASIC_MVDDC: c_int = 2;
pub const SET_VOLTAGE_TYPE_ASIC_MVDDQ: c_int = 3;
pub const SET_VOLTAGE_TYPE_ASIC_VDDCI: c_int = 4;
pub const SET_VOLTAGE_INIT_MODE: c_int = 5;

pub const SET_ASIC_VOLTAGE_MODE_ALL_SOURCE: c_uint = 0x1;
pub const SET_ASIC_VOLTAGE_MODE_SOURCE_A: c_uint = 0x2;
pub const SET_ASIC_VOLTAGE_MODE_SOURCE_B: c_uint = 0x4;
pub const SET_ASIC_VOLTAGE_MODE_SET_VOLTAGE: c_uint = 0x0;
pub const SET_ASIC_VOLTAGE_MODE_GET_GPIOVAL: c_uint = 0x1;
pub const SET_ASIC_VOLTAGE_MODE_GET_GPIOMASK: c_uint = 0x2;
// used by both SetVoltageTable v1.3 and v1.4
// ucVoltageType
pub const VOLTAGE_TYPE_VDDC: c_int = 1;
pub const VOLTAGE_TYPE_MVDDC: c_int = 2;
pub const VOLTAGE_TYPE_MVDDQ: c_int = 3;
pub const VOLTAGE_TYPE_VDDCI: c_int = 4;
// SET_VOLTAGE_PARAMETERS_V3.ucVoltageMode

// define vitual voltage id in usVoltageLevel
pub const ATOM_VIRTUAL_VOLTAGE_ID0: c_uint = 0xff01;
pub const ATOM_VIRTUAL_VOLTAGE_ID1: c_uint = 0xff02;
pub const ATOM_VIRTUAL_VOLTAGE_ID2: c_uint = 0xff03;
pub const ATOM_VIRTUAL_VOLTAGE_ID3: c_uint = 0xff04;
pub const ATOM_VIRTUAL_VOLTAGE_ID4: c_uint = 0xff05;
pub const ATOM_VIRTUAL_VOLTAGE_ID5: c_uint = 0xff06;
pub const ATOM_VIRTUAL_VOLTAGE_ID6: c_uint = 0xff07;
pub const ATOM_VIRTUAL_VOLTAGE_ID7: c_uint = 0xff08;
// New Added from SI for GetVoltageInfoTable, input parameter structure
// New Added from SI for GetVoltageInfoTable, output parameter structure when ucVotlageMode == ATOM_GET_VOLTAGE_VID
// New Added from SI for GetVoltageInfoTable, output parameter structure when ucVotlageMode == ATOM_GET_VOLTAGE_STATEx_LEAKAGE_VID
// GetVoltageInfo v1.1 ucVoltageMode
pub const ATOM_GET_VOLTAGE_VID: c_uint = 0x00;
pub const ATOM_GET_VOTLAGE_INIT_SEQ: c_uint = 0x03;
pub const ATOM_GET_VOLTTAGE_PHASE_PHASE_VID: c_uint = 0x04;
pub const ATOM_GET_VOLTAGE_SVID2: c_uint = 0x07        //Get SVI2 Regulator Info;
// for SI, this state map to 0xff02 voltage state in Power Play table, which is power boost state
pub const ATOM_GET_VOLTAGE_STATE0_LEAKAGE_VID: c_uint = 0x10;
// for SI, this state map to 0xff01 voltage state in Power Play table, which is performance state
pub const ATOM_GET_VOLTAGE_STATE1_LEAKAGE_VID: c_uint = 0x11;
pub const ATOM_GET_VOLTAGE_STATE2_LEAKAGE_VID: c_uint = 0x12;
pub const ATOM_GET_VOLTAGE_STATE3_LEAKAGE_VID: c_uint = 0x13;
// New Added from CI Hawaii for GetVoltageInfoTable, input parameter structure
// New in GetVoltageInfo v1.2 ucVoltageMode
pub const ATOM_GET_VOLTAGE_EVV_VOLTAGE: c_uint = 0x09;
// New Added from CI Hawaii for EVV feature
//
// Structures used by TVEncoderControlTable
//
// 1: setup and turn on encoder
// ==============================Data Table Portion====================================
//
// Structure used in Data.mtb
//
// For backward compatible

//
// Structure used in MultimediaCapabilityInfoTable
//
// Structure used in MultimediaConfigInfoTable
//
// Structures used in FirmwareInfoTable
//
// usBIOSCapability Definition:
// Bit 0 = 0: Bios image is not Posted, =1:Bios image is Posted;
// Bit 1 = 0: Dual CRTC is not supported, =1: Dual CRTC is supported;
// Bit 2 = 0: Extended Desktop is not supported, =1: Extended Desktop is supported;
// Others: Reserved
pub const ATOM_BIOS_INFO_ATOM_FIRMWARE_POSTED: c_uint = 0x0001;
pub const ATOM_BIOS_INFO_DUAL_CRTC_SUPPORT: c_uint = 0x0002;
pub const ATOM_BIOS_INFO_EXTENDED_DESKTOP_SUPPORT: c_uint = 0x0004;
pub const ATOM_BIOS_INFO_MEMORY_CLOCK_SS_SUPPORT: c_uint = 0x0008		// (valid from v1.1 ~v1.4):=1: memclk SS enable, =0 memclk SS disable.;
pub const ATOM_BIOS_INFO_ENGINE_CLOCK_SS_SUPPORT: c_uint = 0x0010		// (valid from v1.1 ~v1.4):=1: engclk SS enable, =0 engclk SS disable.;
pub const ATOM_BIOS_INFO_BL_CONTROLLED_BY_GPU: c_uint = 0x0020;
pub const ATOM_BIOS_INFO_WMI_SUPPORT: c_uint = 0x0040;
pub const ATOM_BIOS_INFO_PPMODE_ASSIGNGED_BY_SYSTEM: c_uint = 0x0080;
pub const ATOM_BIOS_INFO_HYPERMEMORY_SUPPORT: c_uint = 0x0100;
pub const ATOM_BIOS_INFO_HYPERMEMORY_SIZE_MASK: c_uint = 0x1E00;
pub const ATOM_BIOS_INFO_VPOST_WITHOUT_FIRST_MODE_SET: c_uint = 0x2000;
pub const ATOM_BIOS_INFO_BIOS_SCRATCH6_SCL2_REDEFINE: c_uint = 0x4000;
pub const ATOM_BIOS_INFO_MEMORY_CLOCK_EXT_SS_SUPPORT: c_uint = 0x0008		// (valid from v2.1 ): =1: memclk ss enable with external ss chip;
pub const ATOM_BIOS_INFO_ENGINE_CLOCK_EXT_SS_SUPPORT: c_uint = 0x0010		// (valid from v2.1 ): =1: engclk ss enable with external ss chip;
// Please don't add or expand this bitfield structure below, this one will retire soon.!

// the structure below to be used from Cypress
// the structure below to be used from NI
// ucTableFormatRevision=2
// ucTableContentRevision=2

// definition of ucRemoteDisplayConfig
pub const REMOTE_DISPLAY_DISABLE: c_uint = 0x00;
pub const REMOTE_DISPLAY_ENABLE: c_uint = 0x01;
//
// Structures used in IntegratedSystemInfoTable
//
pub const IGP_CAP_FLAG_DYNAMIC_CLOCK_EN: c_uint = 0x2;
pub const IGP_CAP_FLAG_AC_CARD: c_uint = 0x4;
pub const IGP_CAP_FLAG_SDVO_CARD: c_uint = 0x8;
pub const IGP_CAP_FLAG_POSTDIV_BY_2_MODE: c_uint = 0x10;
// Bit[3:2]== 0:No PCIE card, 1:AC card, 2:SDVO card
// Bit[4]==1: P/2 mode, ==0: P/1 mode
// Explanation on entries in ATOM_INTEGRATED_SYSTEM_INFO
//
// ATOM_INTEGRATED_SYSTEM_INFO::ulCPUCapInfo  - CPU type definition
pub const INTEGRATED_SYSTEM_INFO__UNKNOWN_CPU: c_int = 0;
pub const INTEGRATED_SYSTEM_INFO__AMD_CPU__GRIFFIN: c_int = 1;
pub const INTEGRATED_SYSTEM_INFO__AMD_CPU__GREYHOUND: c_int = 2;
pub const INTEGRATED_SYSTEM_INFO__AMD_CPU__K8: c_int = 3;
pub const INTEGRATED_SYSTEM_INFO__AMD_CPU__PHARAOH: c_int = 4;
pub const INTEGRATED_SYSTEM_INFO__AMD_CPU__OROCHI: c_int = 5;

pub const SYSTEM_CONFIG_POWEREXPRESS_ENABLE: c_uint = 0x00000001;
pub const SYSTEM_CONFIG_RUN_AT_OVERDRIVE_ENGINE: c_uint = 0x00000002;
pub const SYSTEM_CONFIG_USE_PWM_ON_VOLTAGE: c_uint = 0x00000004;
pub const SYSTEM_CONFIG_PERFORMANCE_POWERSTATE_ONLY: c_uint = 0x00000008;
pub const SYSTEM_CONFIG_CLMC_ENABLED: c_uint = 0x00000010;
pub const SYSTEM_CONFIG_CDLW_ENABLED: c_uint = 0x00000020;
pub const SYSTEM_CONFIG_HIGH_VOLTAGE_REQUESTED: c_uint = 0x00000040;
pub const SYSTEM_CONFIG_CLMC_HYBRID_MODE_ENABLED: c_uint = 0x00000080;
pub const SYSTEM_CONFIG_CDLF_ENABLED: c_uint = 0x00000100;
pub const SYSTEM_CONFIG_DLL_SHUTDOWN_ENABLED: c_uint = 0x00000200;
pub const IGP_DDI_SLOT_LANE_CONFIG_MASK: c_uint = 0x000000FF;
pub const b0IGP_DDI_SLOT_LANE_MAP_MASK: c_uint = 0x0F;
pub const b0IGP_DDI_SLOT_DOCKING_LANE_MAP_MASK: c_uint = 0xF0;
pub const b0IGP_DDI_SLOT_CONFIG_LANE_0_3: c_uint = 0x01;
pub const b0IGP_DDI_SLOT_CONFIG_LANE_4_7: c_uint = 0x02;
pub const b0IGP_DDI_SLOT_CONFIG_LANE_8_11: c_uint = 0x04;
pub const b0IGP_DDI_SLOT_CONFIG_LANE_12_15: c_uint = 0x08;
pub const IGP_DDI_SLOT_ATTRIBUTE_MASK: c_uint = 0x0000FF00;
pub const IGP_DDI_SLOT_CONFIG_REVERSED: c_uint = 0x00000100;
pub const b1IGP_DDI_SLOT_CONFIG_REVERSED: c_uint = 0x01;
pub const IGP_DDI_SLOT_CONNECTOR_TYPE_MASK: c_uint = 0x00FF0000;
// IntegratedSystemInfoTable new Rev is V5 after V2, because of the real rev of V2 is v1.4. This rev is used for RR
pub const ATOM_CRT_INT_ENCODER1_INDEX: c_uint = 0x00000000;
pub const ATOM_LCD_INT_ENCODER1_INDEX: c_uint = 0x00000001;
pub const ATOM_TV_INT_ENCODER1_INDEX: c_uint = 0x00000002;
pub const ATOM_DFP_INT_ENCODER1_INDEX: c_uint = 0x00000003;
pub const ATOM_CRT_INT_ENCODER2_INDEX: c_uint = 0x00000004;
pub const ATOM_LCD_EXT_ENCODER1_INDEX: c_uint = 0x00000005;
pub const ATOM_TV_EXT_ENCODER1_INDEX: c_uint = 0x00000006;
pub const ATOM_DFP_EXT_ENCODER1_INDEX: c_uint = 0x00000007;
pub const ATOM_CV_INT_ENCODER1_INDEX: c_uint = 0x00000008;
pub const ATOM_DFP_INT_ENCODER2_INDEX: c_uint = 0x00000009;
pub const ATOM_CRT_EXT_ENCODER1_INDEX: c_uint = 0x0000000A;
pub const ATOM_CV_EXT_ENCODER1_INDEX: c_uint = 0x0000000B;
pub const ATOM_DFP_INT_ENCODER3_INDEX: c_uint = 0x0000000C;
pub const ATOM_DFP_INT_ENCODER4_INDEX: c_uint = 0x0000000D;
// define ASIC internal encoder id ( bit vector ), used for CRTC_SourceSelTable
pub const ASIC_INT_DAC1_ENCODER_ID: c_uint = 0x00;
pub const ASIC_INT_TV_ENCODER_ID: c_uint = 0x02;
pub const ASIC_INT_DIG1_ENCODER_ID: c_uint = 0x03;
pub const ASIC_INT_DAC2_ENCODER_ID: c_uint = 0x04;
pub const ASIC_EXT_TV_ENCODER_ID: c_uint = 0x06;
pub const ASIC_INT_DVO_ENCODER_ID: c_uint = 0x07;
pub const ASIC_INT_DIG2_ENCODER_ID: c_uint = 0x09;
pub const ASIC_EXT_DIG_ENCODER_ID: c_uint = 0x05;
pub const ASIC_EXT_DIG2_ENCODER_ID: c_uint = 0x08;
pub const ASIC_INT_DIG3_ENCODER_ID: c_uint = 0x0a;
pub const ASIC_INT_DIG4_ENCODER_ID: c_uint = 0x0b;
pub const ASIC_INT_DIG5_ENCODER_ID: c_uint = 0x0c;
pub const ASIC_INT_DIG6_ENCODER_ID: c_uint = 0x0d;
pub const ASIC_INT_DIG7_ENCODER_ID: c_uint = 0x0e;
// define Encoder attribute
pub const ATOM_ANALOG_ENCODER: c_int = 0;
pub const ATOM_DIGITAL_ENCODER: c_int = 1;
pub const ATOM_DP_ENCODER: c_int = 2;
pub const ATOM_ENCODER_ENUM_MASK: c_uint = 0x70;
pub const ATOM_ENCODER_ENUM_ID1: c_uint = 0x00;
pub const ATOM_ENCODER_ENUM_ID2: c_uint = 0x10;
pub const ATOM_ENCODER_ENUM_ID3: c_uint = 0x20;
pub const ATOM_ENCODER_ENUM_ID4: c_uint = 0x30;
pub const ATOM_ENCODER_ENUM_ID5: c_uint = 0x40;
pub const ATOM_ENCODER_ENUM_ID6: c_uint = 0x50;
pub const ATOM_DEVICE_CRT1_INDEX: c_uint = 0x00000000;
pub const ATOM_DEVICE_LCD1_INDEX: c_uint = 0x00000001;
pub const ATOM_DEVICE_TV1_INDEX: c_uint = 0x00000002;
pub const ATOM_DEVICE_DFP1_INDEX: c_uint = 0x00000003;
pub const ATOM_DEVICE_CRT2_INDEX: c_uint = 0x00000004;
pub const ATOM_DEVICE_LCD2_INDEX: c_uint = 0x00000005;
pub const ATOM_DEVICE_DFP6_INDEX: c_uint = 0x00000006;
pub const ATOM_DEVICE_DFP2_INDEX: c_uint = 0x00000007;
pub const ATOM_DEVICE_CV_INDEX: c_uint = 0x00000008;
pub const ATOM_DEVICE_DFP3_INDEX: c_uint = 0x00000009;
pub const ATOM_DEVICE_DFP4_INDEX: c_uint = 0x0000000A;
pub const ATOM_DEVICE_DFP5_INDEX: c_uint = 0x0000000B;
pub const ATOM_DEVICE_RESERVEDC_INDEX: c_uint = 0x0000000C;
pub const ATOM_DEVICE_RESERVEDD_INDEX: c_uint = 0x0000000D;
pub const ATOM_DEVICE_RESERVEDE_INDEX: c_uint = 0x0000000E;
pub const ATOM_DEVICE_RESERVEDF_INDEX: c_uint = 0x0000000F;

pub const ATOM_DEVICE_CONNECTOR_TYPE_MASK: c_uint = 0x000000F0;
pub const ATOM_DEVICE_CONNECTOR_TYPE_SHIFT: c_uint = 0x00000004;
pub const ATOM_DEVICE_CONNECTOR_VGA: c_uint = 0x00000001;
pub const ATOM_DEVICE_CONNECTOR_DVI_I: c_uint = 0x00000002;
pub const ATOM_DEVICE_CONNECTOR_DVI_D: c_uint = 0x00000003;
pub const ATOM_DEVICE_CONNECTOR_DVI_A: c_uint = 0x00000004;
pub const ATOM_DEVICE_CONNECTOR_SVIDEO: c_uint = 0x00000005;
pub const ATOM_DEVICE_CONNECTOR_COMPOSITE: c_uint = 0x00000006;
pub const ATOM_DEVICE_CONNECTOR_LVDS: c_uint = 0x00000007;
pub const ATOM_DEVICE_CONNECTOR_DIGI_LINK: c_uint = 0x00000008;
pub const ATOM_DEVICE_CONNECTOR_SCART: c_uint = 0x00000009;
pub const ATOM_DEVICE_CONNECTOR_HDMI_TYPE_A: c_uint = 0x0000000A;
pub const ATOM_DEVICE_CONNECTOR_HDMI_TYPE_B: c_uint = 0x0000000B;
pub const ATOM_DEVICE_CONNECTOR_CASE_1: c_uint = 0x0000000E;
pub const ATOM_DEVICE_CONNECTOR_DISPLAYPORT: c_uint = 0x0000000F;
pub const ATOM_DEVICE_DAC_INFO_MASK: c_uint = 0x0000000F;
pub const ATOM_DEVICE_DAC_INFO_SHIFT: c_uint = 0x00000000;
pub const ATOM_DEVICE_DAC_INFO_NODAC: c_uint = 0x00000000;
pub const ATOM_DEVICE_DAC_INFO_DACA: c_uint = 0x00000001;
pub const ATOM_DEVICE_DAC_INFO_DACB: c_uint = 0x00000002;
pub const ATOM_DEVICE_DAC_INFO_EXDAC: c_uint = 0x00000003;
pub const ATOM_DEVICE_I2C_ID_NOI2C: c_uint = 0x00000000;
pub const ATOM_DEVICE_I2C_LINEMUX_MASK: c_uint = 0x0000000F;
pub const ATOM_DEVICE_I2C_LINEMUX_SHIFT: c_uint = 0x00000000;
pub const ATOM_DEVICE_I2C_ID_MASK: c_uint = 0x00000070;
pub const ATOM_DEVICE_I2C_ID_SHIFT: c_uint = 0x00000004;
pub const ATOM_DEVICE_I2C_ID_IS_FOR_NON_MM_USE: c_uint = 0x00000001;
pub const ATOM_DEVICE_I2C_ID_IS_FOR_MM_USE: c_uint = 0x00000002;
pub const ATOM_DEVICE_I2C_ID_IS_FOR_SDVO_USE: c_uint = 0x00000003    //For IGP RS600;
pub const ATOM_DEVICE_I2C_ID_IS_FOR_DAC_SCL: c_uint = 0x00000004    //For IGP RS690;
pub const ATOM_DEVICE_I2C_HARDWARE_CAP_MASK: c_uint = 0x00000080;
pub const ATOM_DEVICE_I2C_HARDWARE_CAP_SHIFT: c_uint = 0x00000007;
pub const ATOM_DEVICE_USES_SOFTWARE_ASSISTED_I2C: c_uint = 0x00000000;
pub const ATOM_DEVICE_USES_HARDWARE_ASSISTED_I2C: c_uint = 0x00000001;
// usDeviceSupport:
// Bits0	= 0 - no CRT1 support= 1- CRT1 is supported
// Bit 1	= 0 - no LCD1 support= 1- LCD1 is supported
// Bit 2	= 0 - no TV1  support= 1- TV1  is supported
// Bit 3	= 0 - no DFP1 support= 1- DFP1 is supported
// Bit 4	= 0 - no CRT2 support= 1- CRT2 is supported
// Bit 5	= 0 - no LCD2 support= 1- LCD2 is supported
// Bit 6	= 0 - no DFP6 support= 1- DFP6 is supported
// Bit 7	= 0 - no DFP2 support= 1- DFP2 is supported
// Bit 8	= 0 - no CV   support= 1- CV   is supported
// Bit 9	= 0 - no DFP3 support= 1- DFP3 is supported
// Bit 10      = 0 - no DFP4 support= 1- DFP4 is supported
// Bit 11      = 0 - no DFP5 support= 1- DFP5 is supported
//
// Structure used in MclkSS_InfoTable
//
// ucI2C_ConfigID
// [7:0] - I2C LINE Associate ID
// = 0   - no I2C
// [7]		-	HW_Cap        =	1,  [6:0]=HW assisted I2C ID(HW line selection)
// =	0,  [6:0]=SW assisted I2C ID
// [6-4]	- HW_ENGINE_ID  =	1,  HW engine for NON multimedia use
// =	2,	HW engine for Multimedia use
// =	3-7	Reserved for future I2C engines
// [3-0] - I2C_LINE_MUX  = A Mux number when it's HW assisted I2C or GPIO ID when it's SW I2C

//
// Structure used in GPIO_I2C_InfoTable
//
// Common Structure used in other structures
//
// Please don't add or expand this bitfield structure below, this one will retire soon.!

// usModeMiscInfo-
pub const ATOM_H_CUTOFF: c_uint = 0x01;
pub const ATOM_HSYNC_POLARITY: c_uint = 0x02             //0=Active High, 1=Active Low;
pub const ATOM_VSYNC_POLARITY: c_uint = 0x04             //0=Active High, 1=Active Low;
pub const ATOM_V_CUTOFF: c_uint = 0x08;
pub const ATOM_H_REPLICATIONBY2: c_uint = 0x10;
pub const ATOM_V_REPLICATIONBY2: c_uint = 0x20;
pub const ATOM_COMPOSITESYNC: c_uint = 0x40;
pub const ATOM_INTERLACE: c_uint = 0x80;
pub const ATOM_DOUBLE_CLOCK_MODE: c_uint = 0x100;
pub const ATOM_RGB888_MODE: c_uint = 0x200;
// usRefreshRate-
pub const ATOM_REFRESH_43: c_int = 43;
pub const ATOM_REFRESH_47: c_int = 47;
pub const ATOM_REFRESH_56: c_int = 56;
pub const ATOM_REFRESH_60: c_int = 60;
pub const ATOM_REFRESH_65: c_int = 65;
pub const ATOM_REFRESH_70: c_int = 70;
pub const ATOM_REFRESH_72: c_int = 72;
pub const ATOM_REFRESH_75: c_int = 75;
pub const ATOM_REFRESH_85: c_int = 85;
// ATOM_MODE_TIMING data are exactly the same as VESA timing data.
// Translation from EDID to ATOM_MODE_TIMING, use the following formula.
//
// VESA_HTOTAL			=	VESA_ACTIVE + 2* VESA_BORDER + VESA_BLANK
// =	EDID_HA + EDID_HBL
// VESA_HDISP			=	VESA_ACTIVE	=	EDID_HA
// VESA_HSYNC_START	=	VESA_ACTIVE + VESA_BORDER + VESA_FRONT_PORCH
// =	EDID_HA + EDID_HSO
// VESA_HSYNC_WIDTH	=	VESA_HSYNC_TIME	=	EDID_HSPW
// VESA_BORDER			=	EDID_BORDER
//
// Structure used in SetCRTC_UsingDTDTimingTable
//
// Structure used in SetCRTC_TimingTable
//

//
// Structure used in StandardVESA_TimingTable
// AnalogTV_InfoTable
// ComponentVideoInfoTable
//
// Structure used in LVDS_InfoTable
// * Need a document to describe this table
//
pub const SUPPORTED_LCD_REFRESHRATE_30Hz: c_uint = 0x0004;
pub const SUPPORTED_LCD_REFRESHRATE_40Hz: c_uint = 0x0008;
pub const SUPPORTED_LCD_REFRESHRATE_50Hz: c_uint = 0x0010;
pub const SUPPORTED_LCD_REFRESHRATE_60Hz: c_uint = 0x0020;
// ucTableFormatRevision=1
// ucTableContentRevision=1
// Bit4:{=0:LDI format for RGB888, =1 FPDI format for RGB888}
// Bit5:{=0:Spatial Dithering disabled;1 Spatial Dithering enabled}
// Bit6:{=0:Temporal Dithering disabled;1 Temporal Dithering enabled}
// ucTableFormatRevision=1
// ucTableContentRevision=2
// Bit4:{=0:LDI format for RGB888, =1 FPDI format for RGB888}
// Bit5:{=0:Spatial Dithering disabled;1 Spatial Dithering enabled}
// Bit6:{=0:Temporal Dithering disabled;1 Temporal Dithering enabled}
// Definitions for ucLCDPanel_SpecialHandlingCap:
// Once DAL sees this CAP is set, it will read EDID from LCD on its own instead of using sLCDTiming in ATOM_LVDS_INFO_V12.
// Other entries in ATOM_LVDS_INFO_V12 are still valid/useful to DAL
pub const LCDPANEL_CAP_READ_EDID: c_uint = 0x1;
// If a design supports DRR (dynamic refresh rate) on internal panels (LVDS or EDP), this cap is set in ucLCDPanel_SpecialHandlingCap together
// with multiple supported refresh rates@usSupportedRefreshRate. This cap should not be set when only slow refresh rate is supported (static
// refresh rate switch by SW. This is only valid from ATOM_LVDS_INFO_V12
pub const LCDPANEL_CAP_DRR_SUPPORTED: c_uint = 0x2;
// Use this cap bit for a quick reference whether an embadded panel (LCD1 ) is LVDS or eDP.
pub const LCDPANEL_CAP_eDP: c_uint = 0x4;
// Color Bit Depth definition in EDID V1.4 @BYTE 14h
// Bit 6  5  4
// 0  0  0  -  Color bit depth is undefined
// 0  0  1  -  6 Bits per Primary Color
// 0  1  0  -  8 Bits per Primary Color
// 0  1  1  - 10 Bits per Primary Color
// 1  0  0  - 12 Bits per Primary Color
// 1  0  1  - 14 Bits per Primary Color
// 1  1  0  - 16 Bits per Primary Color
// 1  1  1  - Reserved
pub const PANEL_COLOR_BIT_DEPTH_MASK: c_uint = 0x70;
// Bit7:{=0:Random Dithering disabled;1 Random Dithering enabled}
pub const PANEL_RANDOM_DITHER: c_uint = 0x80;
pub const PANEL_RANDOM_DITHER_MASK: c_uint = 0x80;

//
// Structures used by LCD_InfoTable V1.3    Note: previous version was called ATOM_LVDS_INFO_V12
// ASIC Families:  NI
// ucTableFormatRevision=1
// ucTableContentRevision=3
//
// Bit0: {=0:single, =1:dual},
// Bit1: {=0:LDI format for RGB888, =1 FPDI format for RGB888}  // was {=0:666RGB, =1:888RGB},
// Bit3:2: {Grey level}
// Bit6:4 Color Bit Depth definition (see below definition in EDID V1.4 @BYTE 14h)
// Bit7   Reserved.  was for ATOM_PANEL_MISC_API_ENABLED, still need it?
// Bit0: Once DAL sees this CAP is set, it will read EDID from LCD on its own
// Bit1: See LCDPANEL_CAP_DRR_SUPPORTED
// Bit2: a quick reference whether an embadded panel (LCD1 ) is LVDS (0) or eDP (1)
// Bit7-3: Reserved

// Definitions for ucLCD_Misc
pub const ATOM_PANEL_MISC_V13_DUAL: c_uint = 0x00000001;
pub const ATOM_PANEL_MISC_V13_FPDI: c_uint = 0x00000002;
pub const ATOM_PANEL_MISC_V13_GREY_LEVEL: c_uint = 0x0000000C;
pub const ATOM_PANEL_MISC_V13_GREY_LEVEL_SHIFT: c_int = 2;
pub const ATOM_PANEL_MISC_V13_COLOR_BIT_DEPTH_MASK: c_uint = 0x70;
pub const ATOM_PANEL_MISC_V13_6BIT_PER_COLOR: c_uint = 0x10;
pub const ATOM_PANEL_MISC_V13_8BIT_PER_COLOR: c_uint = 0x20;
// Color Bit Depth definition in EDID V1.4 @BYTE 14h
// Bit 6  5  4
// 0  0  0  -  Color bit depth is undefined
// 0  0  1  -  6 Bits per Primary Color
// 0  1  0  -  8 Bits per Primary Color
// 0  1  1  - 10 Bits per Primary Color
// 1  0  0  - 12 Bits per Primary Color
// 1  0  1  - 14 Bits per Primary Color
// 1  1  0  - 16 Bits per Primary Color
// 1  1  1  - Reserved
// Definitions for ucLCDPanel_SpecialHandlingCap:
// Once DAL sees this CAP is set, it will read EDID from LCD on its own instead of using sLCDTiming in ATOM_LVDS_INFO_V12.
// Other entries in ATOM_LVDS_INFO_V12 are still valid/useful to DAL
pub const LCDPANEL_CAP_V13_READ_EDID: c_uint = 0x1        // = LCDPANEL_CAP_READ_EDID no change comparing to previous version;
// If a design supports DRR (dynamic refresh rate) on internal panels (LVDS or EDP), this cap is set in ucLCDPanel_SpecialHandlingCap together
// with multiple supported refresh rates@usSupportedRefreshRate. This cap should not be set when only slow refresh rate is supported (static
// refresh rate switch by SW. This is only valid from ATOM_LVDS_INFO_V12
pub const LCDPANEL_CAP_V13_DRR_SUPPORTED: c_uint = 0x2        // = LCDPANEL_CAP_DRR_SUPPORTED no change comparing to previous version;
// Use this cap bit for a quick reference whether an embadded panel (LCD1 ) is LVDS or eDP.
pub const LCDPANEL_CAP_V13_eDP: c_uint = 0x4        // = LCDPANEL_CAP_eDP no change comparing to previous version;
// uceDPToLVDSRxId
pub const eDP_TO_LVDS_RX_DISABLE: c_uint = 0x00       // no eDP->LVDS translator chip;
pub const eDP_TO_LVDS_COMMON_ID: c_uint = 0x01       // common eDP->LVDS translator chip without AMD SW init;
pub const eDP_TO_LVDS_RT_ID: c_uint = 0x02       // RT tanslator which require AMD SW init;
// !! If the record below exists, it should always be the first record for easy use in command table!!!
// The record below is only used when LVDS_Info is present. From ATOM_LVDS_INFO_V12, use ucLCDPanel_SpecialHandlingCap instead.
pub const LCD_MODE_CAP_BL_OFF: c_int = 1;
pub const LCD_MODE_CAP_CRTC_OFF: c_int = 2;
pub const LCD_MODE_CAP_PANEL_OFF: c_int = 4;
pub const LCD_MODE_PATCH_RECORD_MODE_TYPE: c_int = 1;
pub const LCD_RTS_RECORD_TYPE: c_int = 2;
pub const LCD_CAP_RECORD_TYPE: c_int = 3;
pub const LCD_FAKE_EDID_PATCH_RECORD_TYPE: c_int = 4;
pub const LCD_PANEL_RESOLUTION_RECORD_TYPE: c_int = 5;
pub const LCD_EDID_OFFSET_PATCH_RECORD_TYPE: c_int = 6;
pub const ATOM_RECORD_END_TYPE: c_uint = 0xFF;
// Spread Spectrum Info Table Definitions
// ucTableFormatRevision=1
// ucTableContentRevision=2
pub const ATOM_MAX_SS_ENTRY: c_int = 16;
pub const ATOM_DP_SS_ID1: c_uint = 0x0f1			// SS ID for internal DP stream at 2.7Ghz. if ATOM_DP_SS_ID2 does not exist in SS_InfoTable, it is used for internal DP stream at 1.62Ghz as well.;
pub const ATOM_DP_SS_ID2: c_uint = 0x0f2			// SS ID for internal DP stream at 1.62Ghz, if it exists in SS_InfoTable.;
pub const ATOM_LVLINK_2700MHz_SS_ID: c_uint = 0x0f3      // SS ID for LV link translator chip at 2.7Ghz;
pub const ATOM_LVLINK_1620MHz_SS_ID: c_uint = 0x0f4      // SS ID for LV link translator chip at 1.62Ghz;
pub const ATOM_SS_DOWN_SPREAD_MODE_MASK: c_uint = 0x00000000;
pub const ATOM_SS_DOWN_SPREAD_MODE: c_uint = 0x00000000;
pub const ATOM_SS_CENTRE_SPREAD_MODE_MASK: c_uint = 0x00000001;
pub const ATOM_SS_CENTRE_SPREAD_MODE: c_uint = 0x00000001;
pub const ATOM_INTERNAL_SS_MASK: c_uint = 0x00000000;
pub const ATOM_EXTERNAL_SS_MASK: c_uint = 0x00000002;
pub const EXEC_SS_STEP_SIZE_SHIFT: c_int = 2;
pub const EXEC_SS_DELAY_SHIFT: c_int = 4;
pub const ACTIVEDATA_TO_BLON_DELAY_SHIFT: c_int = 4;
//
// Structure used in AnalogTV_InfoTable (Top level)
//
// ucTVBootUpDefaultStd definition:
// ATOM_TV_NTSC                1
// ATOM_TV_NTSCJ               2
// ATOM_TV_PAL                 3
// ATOM_TV_PALM                4
// ATOM_TV_PALCN               5
// ATOM_TV_PALN                6
// ATOM_TV_PAL60               7
// ATOM_TV_SECAM               8
// ucTVSupportedStd definition:
pub const NTSC_SUPPORT: c_uint = 0x1;
pub const NTSCJ_SUPPORT: c_uint = 0x2;
pub const PAL_SUPPORT: c_uint = 0x4;
pub const PALM_SUPPORT: c_uint = 0x8;
pub const PALCN_SUPPORT: c_uint = 0x10;
pub const PALN_SUPPORT: c_uint = 0x20;
pub const PAL60_SUPPORT: c_uint = 0x40;
pub const SECAM_SUPPORT: c_uint = 0x80;
pub const MAX_SUPPORTED_TV_TIMING: c_int = 2;
// ATOM_DTD_FORMAT          aModeTimings[MAX_SUPPORTED_TV_TIMING];
pub const MAX_SUPPORTED_TV_TIMING_V1_2: c_int = 3;
pub const ATOM_DPCD_MAX_LANE_MASK: c_uint = 0x1F;
//
// VRAM usage and their defintions
// One chunk of VRAM used by Bios are for HWICON surfaces,EDID data.
// Current Mode timing and Dail Timing and/or STD timing data EACH device. They can be broken down as below.
// All the addresses below are the offsets from the frame buffer start.They all MUST be Dword aligned!
// To driver: The physical address of this memory portion=mmFB_START(4K aligned)+ATOMBIOS_VRAM_USAGE_START_ADDR+ATOM_x_ADDR
// To Bios:  ATOMBIOS_VRAM_USAGE_START_ADDR+ATOM_x_ADDR->MM_INDEX

pub const VESA_MEMORY_IN_64K_BLOCK: c_uint = 0x100       //256*64K=16Mb (Max. VESA memory is 16Mb!);

pub const ATOM_HWICON_INFOTABLE_SIZE: c_int = 32;
pub const MAX_DTD_MODE_IN_VRAM: c_int = 6;

// 20 bytes for Encoder Type and DPCD in STD EDID area

pub const ATOM_HWICON1_SURFACE_ADDR: c_int = 0;

// The size below is in Kb!

pub const ATOM_VRAM_RESERVE_V2_SIZE: c_int = 32;
pub const ATOM_VRAM_OPERATION_FLAGS_MASK: c_uint = 0xC0000000L;
pub const ATOM_VRAM_OPERATION_FLAGS_SHIFT: c_int = 30;
pub const ATOM_VRAM_BLOCK_NEEDS_NO_RESERVATION: c_uint = 0x1;
pub const ATOM_VRAM_BLOCK_NEEDS_RESERVATION: c_uint = 0x0;
//
// Structure used in VRAM_UsageByFirmwareTable
// Note1: This table is filled by SetBiosReservationStartInFB in CoreCommSubs.asm
// at running time.
// note2: From RV770, the memory is more than 32bit addressable, so we will change
// ucTableFormatRevision=1,ucTableContentRevision=4, the structure remains
// exactly same as 1.1 and 1.2 (1.3 is never in use), but ulStartAddrUsedByFirmware
// (in offset to start of memory address) is KB aligned instead of byte aligned.
// Note3:
// If we change usReserved to "usFBUsedbyDrvInKB", then to VBIOS this usFBUsedbyDrvInKB is a predefined, unchanged constant across VGA or non VGA adapter,
//
pub const ATOM_MAX_FIRMWARE_VRAM_USAGE_INFO: c_int = 1;
// change version to 1.5, when allow driver to allocate the vram area for command table access.
//
// Structure used in GPIO_Pin_LUTTable
//
// ucGPIO_ID pre-define id for multiple usage
// from SMU7.x, if ucGPIO_ID=PP_AC_DC_SWITCH_GPIO_PINID in GPIO_LUTTable, AC/DC switching feature is enable
pub const PP_AC_DC_SWITCH_GPIO_PINID: c_int = 60;
// from SMU7.x, if ucGPIO_ID=VDDC_REGULATOR_VRHOT_GPIO_PINID in GPIO_LUTable, VRHot feature is enable
pub const VDDC_VRHOT_GPIO_PINID: c_int = 61;
// if ucGPIO_ID=VDDC_PCC_GPIO_PINID in GPIO_LUTable, Peak Current Control feature is enabled
pub const VDDC_PCC_GPIO_PINID: c_int = 62;
//
// Structure used in ComponentVideoInfoTable
//
pub const GPIO_PIN_ACTIVE_HIGH: c_uint = 0x1;
pub const MAX_SUPPORTED_CV_STANDARDS: c_int = 5;
// definitions for ATOM_D_INFO.ucSettings
pub const ATOM_GPIO_SETTINGS_BITSHIFT_MASK: c_uint = 0x1F    // [4:0];
pub const ATOM_GPIO_SETTINGS_RESERVED_MASK: c_uint = 0x60    // [6:5] = must be zeroed out;
pub const ATOM_GPIO_SETTINGS_ACTIVE_MASK: c_uint = 0x80    // [7];
// definitions for ATOM_COMPONENT_VIDEO_INFO.ucMiscInfo (bit vector)
pub const ATOM_CV_RESTRICT_FORMAT_SELECTION: c_uint = 0x2;
// definitions for ATOM_COMPONENT_VIDEO_INFO.uc480i/uc480p/uc720p/uc1080i
pub const ATOM_GPIO_DEFAULT_MODE_EN: c_uint = 0x80 //[7];;
pub const ATOM_GPIO_SETTING_PERMODE_MASK: c_uint = 0x7F //[6:0];
// definitions for ATOM_COMPONENT_VIDEO_INFO.ucLetterBoxMode
// Line 3 out put 5V.
pub const ATOM_CV_LINE3_ASPECTRATIO_16_9_GPIO_A: c_uint = 0x01     //represent gpio 3 state for 16:9;
pub const ATOM_CV_LINE3_ASPECTRATIO_16_9_GPIO_B: c_uint = 0x02     //represent gpio 4 state for 16:9;
pub const ATOM_CV_LINE3_ASPECTRATIO_16_9_GPIO_SHIFT: c_uint = 0x0;
// Line 3 out put 2.2V
pub const ATOM_CV_LINE3_ASPECTRATIO_4_3_LETBOX_GPIO_A: c_uint = 0x04     //represent gpio 3 state for 4:3 Letter box;
pub const ATOM_CV_LINE3_ASPECTRATIO_4_3_LETBOX_GPIO_B: c_uint = 0x08     //represent gpio 4 state for 4:3 Letter box;
pub const ATOM_CV_LINE3_ASPECTRATIO_4_3_LETBOX_GPIO_SHIFT: c_uint = 0x2;
// Line 3 out put 0V
pub const ATOM_CV_LINE3_ASPECTRATIO_4_3_GPIO_A: c_uint = 0x10     //represent gpio 3 state for 4:3;
pub const ATOM_CV_LINE3_ASPECTRATIO_4_3_GPIO_B: c_uint = 0x20     //represent gpio 4 state for 4:3;
pub const ATOM_CV_LINE3_ASPECTRATIO_4_3_GPIO_SHIFT: c_uint = 0x4;
pub const ATOM_CV_LINE3_ASPECTRATIO_MASK: c_uint = 0x3F     // bit [5:0];
pub const ATOM_CV_LINE3_ASPECTRATIO_EXIST: c_uint = 0x80     //bit 7;
// GPIO bit index in gpio setting per mode value, also represend the block no. in gpio blocks.

// ucTableFormatRevision=2
// ucTableContentRevision=1

//
// Structure used in object_InfoTable
//
// Two definitions below are for OPM on MXM module designs
pub const EXT_HPDPIN_LUTINDEX_0: c_int = 0;
pub const EXT_HPDPIN_LUTINDEX_1: c_int = 1;
pub const EXT_HPDPIN_LUTINDEX_2: c_int = 2;
pub const EXT_HPDPIN_LUTINDEX_3: c_int = 3;
pub const EXT_HPDPIN_LUTINDEX_4: c_int = 4;
pub const EXT_HPDPIN_LUTINDEX_5: c_int = 5;
pub const EXT_HPDPIN_LUTINDEX_6: c_int = 6;
pub const EXT_HPDPIN_LUTINDEX_7: c_int = 7;

pub const EXT_AUXDDC_LUTINDEX_0: c_int = 0;
pub const EXT_AUXDDC_LUTINDEX_1: c_int = 1;
pub const EXT_AUXDDC_LUTINDEX_2: c_int = 2;
pub const EXT_AUXDDC_LUTINDEX_3: c_int = 3;
pub const EXT_AUXDDC_LUTINDEX_4: c_int = 4;
pub const EXT_AUXDDC_LUTINDEX_5: c_int = 5;
pub const EXT_AUXDDC_LUTINDEX_6: c_int = 6;
pub const EXT_AUXDDC_LUTINDEX_7: c_int = 7;

// ucChannelMapping are defined as following
// for DP connector, eDP, DP to VGA/LVDS
// Bit[1:0]: Define which pin connect to DP connector DP_Lane0, =0: source from GPU pin TX0, =1: from GPU pin TX1, =2: from GPU pin TX2, =3 from GPU pin TX3
// Bit[3:2]: Define which pin connect to DP connector DP_Lane1, =0: source from GPU pin TX0, =1: from GPU pin TX1, =2: from GPU pin TX2, =3 from GPU pin TX3
// Bit[5:4]: Define which pin connect to DP connector DP_Lane2, =0: source from GPU pin TX0, =1: from GPU pin TX1, =2: from GPU pin TX2, =3 from GPU pin TX3
// Bit[7:6]: Define which pin connect to DP connector DP_Lane3, =0: source from GPU pin TX0, =1: from GPU pin TX1, =2: from GPU pin TX2, =3 from GPU pin TX3

// for DVI/HDMI, in dual link case, both links have to have same mapping.
// Bit[1:0]: Define which pin connect to DVI connector data Lane2, =0: source from GPU pin TX0, =1: from GPU pin TX1, =2: from GPU pin TX2, =3 from GPU pin TX3
// Bit[3:2]: Define which pin connect to DVI connector data Lane1, =0: source from GPU pin TX0, =1: from GPU pin TX1, =2: from GPU pin TX2, =3 from GPU pin TX3
// Bit[5:4]: Define which pin connect to DVI connector data Lane0, =0: source from GPU pin TX0, =1: from GPU pin TX1, =2: from GPU pin TX2, =3 from GPU pin TX3
// Bit[7:6]: Define which pin connect to DVI connector clock lane, =0: source from GPU pin TX0, =1: from GPU pin TX1, =2: from GPU pin TX2, =3 from GPU pin TX3

pub const NUMBER_OF_UCHAR_FOR_GUID: c_int = 16;
pub const MAX_NUMBER_OF_EXT_DISPLAY_PATH: c_int = 7;
// usCaps
pub const EXT_DISPLAY_PATH_CAPS__HBR2_DISABLE: c_uint = 0x01;
pub const EXT_DISPLAY_PATH_CAPS__DP_FIXED_VS_EN: c_uint = 0x02;
// Related definitions, all records are different but they have a commond header
pub const ATOM_I2C_RECORD_TYPE: c_int = 1;
pub const ATOM_HPD_INT_RECORD_TYPE: c_int = 2;
pub const ATOM_OUTPUT_PROTECTION_RECORD_TYPE: c_int = 3;
pub const ATOM_CONNECTOR_DEVICE_TAG_RECORD_TYPE: c_int = 4;

pub const ATOM_CONNECTOR_CVTV_SHARE_DIN_RECORD_TYPE: c_int = 7;

pub const ATOM_OBJECT_GPIO_CNTL_RECORD_TYPE: c_int = 9;
pub const ATOM_ENCODER_DVO_CF_RECORD_TYPE: c_int = 10;
pub const ATOM_CONNECTOR_CF_RECORD_TYPE: c_int = 11;
pub const ATOM_CONNECTOR_HARDCODE_DTD_RECORD_TYPE: c_int = 12;
pub const ATOM_CONNECTOR_PCIE_SUBCONNECTOR_RECORD_TYPE: c_int = 13;
pub const ATOM_ROUTER_DDC_PATH_SELECT_RECORD_TYPE: c_int = 14;
pub const ATOM_ROUTER_DATA_CLOCK_PATH_SELECT_RECORD_TYPE: c_int = 15;

pub const ATOM_CONNECTOR_REMOTE_CAP_RECORD_TYPE: c_int = 19;
pub const ATOM_ENCODER_CAP_RECORD_TYPE: c_int = 20;
pub const ATOM_BRACKET_LAYOUT_RECORD_TYPE: c_int = 21;
// Must be updated when new record type is added,equal to that record definition!

// The following generic object gpio pin control record type will replace JTAG_RECORD/FPGA_CONTROL_RECORD/DVI_EXT_INPUT_RECORD above gradually
// Definitions for GPIO pin state
pub const GPIO_PIN_TYPE_INPUT: c_uint = 0x00;
pub const GPIO_PIN_TYPE_OUTPUT: c_uint = 0x10;
pub const GPIO_PIN_TYPE_HW_CONTROL: c_uint = 0x20;
// For GPIO_PIN_TYPE_OUTPUT the following is defined
pub const GPIO_PIN_OUTPUT_STATE_MASK: c_uint = 0x01;
pub const GPIO_PIN_OUTPUT_STATE_SHIFT: c_int = 0;
pub const GPIO_PIN_STATE_ACTIVE_LOW: c_uint = 0x0;
pub const GPIO_PIN_STATE_ACTIVE_HIGH: c_uint = 0x1;
// Indexes to GPIO array in GLSync record
// GLSync record is for Frame Lock/Gen Lock feature.
pub const ATOM_GPIO_INDEX_GLSYNC_REFCLK: c_int = 0;
pub const ATOM_GPIO_INDEX_GLSYNC_HSYNC: c_int = 1;
pub const ATOM_GPIO_INDEX_GLSYNC_VSYNC: c_int = 2;
pub const ATOM_GPIO_INDEX_GLSYNC_SWAP_REQ: c_int = 3;
pub const ATOM_GPIO_INDEX_GLSYNC_SWAP_GNT: c_int = 4;
pub const ATOM_GPIO_INDEX_GLSYNC_INTERRUPT: c_int = 5;
pub const ATOM_GPIO_INDEX_GLSYNC_V_RESET: c_int = 6;
pub const ATOM_GPIO_INDEX_GLSYNC_SWAP_CNTL: c_int = 7;
pub const ATOM_GPIO_INDEX_GLSYNC_SWAP_SEL: c_int = 8;
pub const ATOM_GPIO_INDEX_GLSYNC_MAX: c_int = 9;
// Bit maps for ATOM_ENCODER_CAP_RECORD.ucEncoderCap
pub const ATOM_ENCODER_CAP_RECORD_HBR2: c_uint = 0x01         // DP1.2 HBR2 is supported by HW encoder;
pub const ATOM_ENCODER_CAP_RECORD_HBR2_EN: c_uint = 0x02         // DP1.2 HBR2 setting is qualified and HBR2 can be enabled;

// value for ATOM_CONNECTOR_CF_RECORD.ucConnectedDvoBundle
pub const ATOM_CONNECTOR_CF_RECORD_CONNECTED_UPPER12BITBUNDLEA: c_int = 1;
pub const ATOM_CONNECTOR_CF_RECORD_CONNECTED_LOWER12BITBUNDLEB: c_int = 2;
// define ucMuxType
pub const ATOM_ROUTER_MUX_PIN_STATE_MASK: c_uint = 0x0f;
pub const ATOM_ROUTER_MUX_PIN_SINGLE_STATE_COMPLEMENT: c_uint = 0x01;
// define ATOM_CONNECTOR_LAYOUT_INFO.ucConnectorType to describe the display connector size
pub const CONNECTOR_TYPE_DVI_D: c_int = 1;
pub const CONNECTOR_TYPE_DVI_I: c_int = 2;
pub const CONNECTOR_TYPE_VGA: c_int = 3;
pub const CONNECTOR_TYPE_HDMI: c_int = 4;
pub const CONNECTOR_TYPE_DISPLAY_PORT: c_int = 5;
pub const CONNECTOR_TYPE_MINI_DISPLAY_PORT: c_int = 6;
//
// ASIC voltage data table
//
// Define ucVoltageControlId
pub const VOLTAGE_CONTROLLED_BY_HW: c_uint = 0x00;
pub const VOLTAGE_CONTROLLED_BY_I2C_MASK: c_uint = 0x7F;
pub const VOLTAGE_CONTROLLED_BY_GPIO: c_uint = 0x80;
pub const VOLTAGE_CONTROL_ID_LM64: c_uint = 0x01									//I2C control, used for R5xx Core Voltage;
pub const VOLTAGE_CONTROL_ID_DAC: c_uint = 0x02									//I2C control, used for R5xx/R6xx MVDDC,MVDDQ or VDDCI;
pub const VOLTAGE_CONTROL_ID_VT116xM: c_uint = 0x03									//I2C control, used for R6xx Core Voltage;
pub const VOLTAGE_CONTROL_ID_DS4402: c_uint = 0x04;
pub const VOLTAGE_CONTROL_ID_UP6266: c_uint = 0x05;
pub const VOLTAGE_CONTROL_ID_SCORPIO: c_uint = 0x06;
pub const VOLTAGE_CONTROL_ID_VT1556M: c_uint = 0x07;
pub const VOLTAGE_CONTROL_ID_CHL822x: c_uint = 0x08;
pub const VOLTAGE_CONTROL_ID_VT1586M: c_uint = 0x09;
pub const VOLTAGE_CONTROL_ID_UP1637: c_uint = 0x0A;
pub const VOLTAGE_CONTROL_ID_CHL8214: c_uint = 0x0B;
pub const VOLTAGE_CONTROL_ID_UP1801: c_uint = 0x0C;
pub const VOLTAGE_CONTROL_ID_ST6788A: c_uint = 0x0D;
pub const VOLTAGE_CONTROL_ID_CHLIR3564SVI2: c_uint = 0x0E;
pub const VOLTAGE_CONTROL_ID_AD527x: c_uint = 0x0F;
pub const VOLTAGE_CONTROL_ID_NCP81022: c_uint = 0x10;
pub const VOLTAGE_CONTROL_ID_LTC2635: c_uint = 0x11;
// ATOM_VOLTAGE_OBJECT_HEADER_V3.ucVoltageMode

pub const VOLTAGE_OBJ_EVV: c_int = 8;
pub const VOLTAGE_OBJ_PWRBOOST_LEAKAGE_LUT: c_uint = 0x10     //Powerboost Voltage and LeakageId lookup table->ATOM_LEAKAGE_VOLTAGE_OBJECT_V3;
pub const VOLTAGE_OBJ_HIGH_STATE_LEAKAGE_LUT: c_uint = 0x11     //High voltage state Voltage and LeakageId lookup table->ATOM_LEAKAGE_VOLTAGE_OBJECT_V3;
pub const VOLTAGE_OBJ_HIGH1_STATE_LEAKAGE_LUT: c_uint = 0x12     //High1 voltage state Voltage and LeakageId lookup table->ATOM_LEAKAGE_VOLTAGE_OBJECT_V3;
// ATOM_I2C_VOLTAGE_OBJECT_V3.ucVoltageControlFlag
pub const VOLTAGE_DATA_ONE_BYTE: c_int = 0;
pub const VOLTAGE_DATA_TWO_BYTE: c_int = 1;
// 14:7  PSI0_VID
// 6  PSI0_EN
// 5  PSI1
// 4:2  load line slope trim.
// 1:0  offset trim,
// GPU GPIO pin Id to SVID2 regulator VRHot pin. possible value 0~31. 0 means GPIO0, 31 means GPIO31
// ucProfileId
pub const ATOM_ASIC_PROFILE_ID_EFUSE_VOLTAGE: c_int = 1;
pub const ATOM_ASIC_PROFILE_ID_EFUSE_PERFORMANCE_VOLTAGE: c_int = 1;
pub const ATOM_ASIC_PROFILE_ID_EFUSE_THERMAL_VOLTAGE: c_int = 2;
// Define ucPwrSrcId
pub const POWERSOURCE_PCIE_ID1: c_uint = 0x00;
pub const POWERSOURCE_6PIN_CONNECTOR_ID1: c_uint = 0x01;
pub const POWERSOURCE_8PIN_CONNECTOR_ID1: c_uint = 0x02;
pub const POWERSOURCE_6PIN_CONNECTOR_ID2: c_uint = 0x04;
pub const POWERSOURCE_8PIN_CONNECTOR_ID2: c_uint = 0x08;
// define ucPwrSensorId
pub const POWER_SENSOR_ALWAYS: c_uint = 0x00;
pub const POWER_SENSOR_GPIO: c_uint = 0x01;
pub const POWER_SENSOR_I2C: c_uint = 0x02;
// ATOM_INTEGRATED_SYSTEM_INFO_V6 ulSystemConfig cap definition

// this IntegrateSystemInfoTable is used for Liano/Ontario APU
// ulGPUCapInfo
pub const INTEGRATED_SYSTEM_INFO_V6_GPUCAPINFO__TMDSHDMI_COHERENT_SINGLEPLL_MODE: c_uint = 0x01;
pub const INTEGRATED_SYSTEM_INFO_V6_GPUCAPINFO__DISABLE_AUX_HW_MODE_DETECTION: c_uint = 0x08;
// ucLVDSMisc:
pub const SYS_INFO_LVDSMISC__888_FPDI_MODE: c_uint = 0x01;
pub const SYS_INFO_LVDSMISC__DL_CH_SWAP: c_uint = 0x02;
pub const SYS_INFO_LVDSMISC__888_BPC: c_uint = 0x04;
pub const SYS_INFO_LVDSMISC__OVERRIDE_EN: c_uint = 0x08;
pub const SYS_INFO_LVDSMISC__BLON_ACTIVE_LOW: c_uint = 0x10;
// new since Trinity
pub const SYS_INFO_LVDSMISC__TRAVIS_LVDS_VOL_OVERRIDE_EN: c_uint = 0x20;
// not used any more
pub const SYS_INFO_LVDSMISC__VSYNC_ACTIVE_LOW: c_uint = 0x04;
pub const SYS_INFO_LVDSMISC__HSYNC_ACTIVE_LOW: c_uint = 0x08;
//
// this Table is used for Liano/Ontario APU

//
// this IntegrateSystemInfoTable is used for Trinity APU
// ulOtherDisplayMisc
pub const INTEGRATED_SYSTEM_INFO__GET_EDID_CALLBACK_FUNC_SUPPORT: c_uint = 0x01;
pub const INTEGRATED_SYSTEM_INFO__GET_BOOTUP_DISPLAY_CALLBACK_FUNC_SUPPORT: c_uint = 0x02;
pub const INTEGRATED_SYSTEM_INFO__GET_EXPANSION_CALLBACK_FUNC_SUPPORT: c_uint = 0x04;
pub const INTEGRATED_SYSTEM_INFO__FAST_BOOT_SUPPORT: c_uint = 0x08;
// ulGPUCapInfo
pub const SYS_INFO_GPUCAPS__TMDSHDMI_COHERENT_SINGLEPLL_MODE: c_uint = 0x01;
pub const SYS_INFO_GPUCAPS__DP_SINGLEPLL_MODE: c_uint = 0x02;
pub const SYS_INFO_GPUCAPS__DISABLE_AUX_MODE_DETECT: c_uint = 0x08;
pub const SYS_INFO_GPUCAPS__ENABLE_DFS_BYPASS: c_uint = 0x10;
//
// this IntegrateSystemInfoTable is used for Kaveri & Kabini APU
//
// this Table is used for Kaveri/Kabini APU
//
// This portion is only used when ext thermal chip or engine/memory clock SS chip is populated on a design
// Memory SS Info Table
// Define Memory Clock SS chip ID
pub const ICS91719: c_int = 1;
pub const ICS91720: c_int = 2;
// Define one structure to inform SW a "block of data" writing to external SS chip via I2C protocol
// Define one structure to inform SW how many blocks of data writing to external SS chip via I2C protocol, in addition to other information
// ==========================================================================================

// ==========================================================================================
//
// Define ucClockIndication, SW uses the IDs below to search if the SS is required/enabled on a clock branch/signal type.
// SS is not required or enabled if a match is not found.
pub const ASIC_INTERNAL_MEMORY_SS: c_int = 1;
pub const ASIC_INTERNAL_ENGINE_SS: c_int = 2;
pub const ASIC_INTERNAL_UVD_SS: c_int = 3;
pub const ASIC_INTERNAL_SS_ON_TMDS: c_int = 4;
pub const ASIC_INTERNAL_SS_ON_HDMI: c_int = 5;
pub const ASIC_INTERNAL_SS_ON_LVDS: c_int = 6;
pub const ASIC_INTERNAL_SS_ON_DP: c_int = 7;
pub const ASIC_INTERNAL_SS_ON_DCPLL: c_int = 8;
pub const ASIC_EXTERNAL_SS_ON_DP_CLOCK: c_int = 9;
pub const ASIC_INTERNAL_VCE_SS: c_int = 10;
pub const ASIC_INTERNAL_GPUPLL_SS: c_int = 11;
// For TMDS/HDMI/LVDS, it is pixel clock , for DP, it is link clock ( 27000 or 16200 )
// ucSpreadSpectrumMode
// #define ATOM_SS_DOWN_SPREAD_MODE_MASK          0x00000000
// #define ATOM_SS_DOWN_SPREAD_MODE               0x00000000
// #define ATOM_SS_CENTRE_SPREAD_MODE_MASK        0x00000001
// #define ATOM_SS_CENTRE_SPREAD_MODE             0x00000001
// #define ATOM_INTERNAL_SS_MASK                  0x00000000
// #define ATOM_EXTERNAL_SS_MASK                  0x00000002
// For TMDS/HDMI/LVDS, it is pixel clock , for DP, it is link clock ( 27000 or 16200 )
// ATOM_ASIC_SS_ASSIGNMENT_V3.ucSpreadSpectrumMode
pub const SS_MODE_V3_CENTRE_SPREAD_MASK: c_uint = 0x01;
pub const SS_MODE_V3_EXTERNAL_SS_MASK: c_uint = 0x02;
pub const SS_MODE_V3_PERCENTAGE_DIV_BY_1000_MASK: c_uint = 0x10;
// ==============================Scratch Pad Definition Portion===============================
pub const ATOM_DEVICE_CONNECT_INFO_DEF: c_int = 0;
pub const ATOM_ROM_LOCATION_DEF: c_int = 1;
pub const ATOM_TV_STANDARD_DEF: c_int = 2;
pub const ATOM_ACTIVE_INFO_DEF: c_int = 3;
pub const ATOM_LCD_INFO_DEF: c_int = 4;
pub const ATOM_DOS_REQ_INFO_DEF: c_int = 5;
pub const ATOM_ACC_CHANGE_INFO_DEF: c_int = 6;
pub const ATOM_DOS_MODE_INFO_DEF: c_int = 7;
pub const ATOM_I2C_CHANNEL_STATUS_DEF: c_int = 8;
pub const ATOM_I2C_CHANNEL_STATUS1_DEF: c_int = 9;
pub const ATOM_INTERNAL_TIMER_DEF: c_int = 10;
// BIOS_0_SCRATCH Definition
pub const ATOM_S0_CRT1_MONO: c_uint = 0x00000001L;
pub const ATOM_S0_CRT1_COLOR: c_uint = 0x00000002L;

pub const ATOM_S0_TV1_COMPOSITE_A: c_uint = 0x00000004L;
pub const ATOM_S0_TV1_SVIDEO_A: c_uint = 0x00000008L;

pub const ATOM_S0_CV_A: c_uint = 0x00000010L;
pub const ATOM_S0_CV_DIN_A: c_uint = 0x00000020L;

pub const ATOM_S0_CRT2_MONO: c_uint = 0x00000100L;
pub const ATOM_S0_CRT2_COLOR: c_uint = 0x00000200L;

pub const ATOM_S0_TV1_COMPOSITE: c_uint = 0x00000400L;
pub const ATOM_S0_TV1_SVIDEO: c_uint = 0x00000800L;
pub const ATOM_S0_TV1_SCART: c_uint = 0x00004000L;

pub const ATOM_S0_CV: c_uint = 0x00001000L;
pub const ATOM_S0_CV_DIN: c_uint = 0x00002000L;

pub const ATOM_S0_DFP1: c_uint = 0x00010000L;
pub const ATOM_S0_DFP2: c_uint = 0x00020000L;
pub const ATOM_S0_LCD1: c_uint = 0x00040000L;
pub const ATOM_S0_LCD2: c_uint = 0x00080000L;
pub const ATOM_S0_DFP6: c_uint = 0x00100000L;
pub const ATOM_S0_DFP3: c_uint = 0x00200000L;
pub const ATOM_S0_DFP4: c_uint = 0x00400000L;
pub const ATOM_S0_DFP5: c_uint = 0x00800000L;

pub const ATOM_S0_FAD_REGISTER_BUG: c_uint = 0x02000000L // If set, indicates we are running a PCIE asic with;
// the FAD/HDP reg access bug.  Bit is read by DAL, this is obsolete from RV5xx
pub const ATOM_S0_THERMAL_STATE_MASK: c_uint = 0x1C000000L;
pub const ATOM_S0_THERMAL_STATE_SHIFT: c_int = 26;
pub const ATOM_S0_SYSTEM_POWER_STATE_MASK: c_uint = 0xE0000000L;
pub const ATOM_S0_SYSTEM_POWER_STATE_SHIFT: c_int = 29;
pub const ATOM_S0_SYSTEM_POWER_STATE_VALUE_AC: c_int = 1;
pub const ATOM_S0_SYSTEM_POWER_STATE_VALUE_DC: c_int = 2;
pub const ATOM_S0_SYSTEM_POWER_STATE_VALUE_LITEAC: c_int = 3;
pub const ATOM_S0_SYSTEM_POWER_STATE_VALUE_LIT2AC: c_int = 4;
// Byte aligned definition for BIOS usage
pub const ATOM_S0_CRT1_MONOb0: c_uint = 0x01;
pub const ATOM_S0_CRT1_COLORb0: c_uint = 0x02;

pub const ATOM_S0_TV1_COMPOSITEb0: c_uint = 0x04;
pub const ATOM_S0_TV1_SVIDEOb0: c_uint = 0x08;

pub const ATOM_S0_CVb0: c_uint = 0x10;
pub const ATOM_S0_CV_DINb0: c_uint = 0x20;

pub const ATOM_S0_CRT2_MONOb1: c_uint = 0x01;
pub const ATOM_S0_CRT2_COLORb1: c_uint = 0x02;

pub const ATOM_S0_TV1_COMPOSITEb1: c_uint = 0x04;
pub const ATOM_S0_TV1_SVIDEOb1: c_uint = 0x08;
pub const ATOM_S0_TV1_SCARTb1: c_uint = 0x40;

pub const ATOM_S0_CVb1: c_uint = 0x10;
pub const ATOM_S0_CV_DINb1: c_uint = 0x20;

pub const ATOM_S0_DFP1b2: c_uint = 0x01;
pub const ATOM_S0_DFP2b2: c_uint = 0x02;
pub const ATOM_S0_LCD1b2: c_uint = 0x04;
pub const ATOM_S0_LCD2b2: c_uint = 0x08;
pub const ATOM_S0_DFP6b2: c_uint = 0x10;
pub const ATOM_S0_DFP3b2: c_uint = 0x20;
pub const ATOM_S0_DFP4b2: c_uint = 0x40;
pub const ATOM_S0_DFP5b2: c_uint = 0x80;
pub const ATOM_S0_THERMAL_STATE_MASKb3: c_uint = 0x1C;
pub const ATOM_S0_THERMAL_STATE_SHIFTb3: c_int = 2;
pub const ATOM_S0_SYSTEM_POWER_STATE_MASKb3: c_uint = 0xE0;
pub const ATOM_S0_LCD1_SHIFT: c_int = 18;
// BIOS_1_SCRATCH Definition
pub const ATOM_S1_ROM_LOCATION_MASK: c_uint = 0x0000FFFFL;
pub const ATOM_S1_PCI_BUS_DEV_MASK: c_uint = 0xFFFF0000L;
// BIOS_2_SCRATCH Definition
pub const ATOM_S2_TV1_STANDARD_MASK: c_uint = 0x0000000FL;
pub const ATOM_S2_CURRENT_BL_LEVEL_MASK: c_uint = 0x0000FF00L;
pub const ATOM_S2_CURRENT_BL_LEVEL_SHIFT: c_int = 8;
pub const ATOM_S2_FORCEDLOWPWRMODE_STATE_MASK: c_uint = 0x0C000000L;
pub const ATOM_S2_FORCEDLOWPWRMODE_STATE_MASK_SHIFT: c_int = 26;
pub const ATOM_S2_FORCEDLOWPWRMODE_STATE_CHANGE: c_uint = 0x10000000L;
pub const ATOM_S2_DEVICE_DPMS_STATE: c_uint = 0x00010000L;
pub const ATOM_S2_VRI_BRIGHT_ENABLE: c_uint = 0x20000000L;
pub const ATOM_S2_DISPLAY_ROTATION_0_DEGREE: c_uint = 0x0;
pub const ATOM_S2_DISPLAY_ROTATION_90_DEGREE: c_uint = 0x1;
pub const ATOM_S2_DISPLAY_ROTATION_180_DEGREE: c_uint = 0x2;
pub const ATOM_S2_DISPLAY_ROTATION_270_DEGREE: c_uint = 0x3;
pub const ATOM_S2_DISPLAY_ROTATION_DEGREE_SHIFT: c_int = 30;
pub const ATOM_S2_DISPLAY_ROTATION_ANGLE_MASK: c_uint = 0xC0000000L;
// Byte aligned definition for BIOS usage
pub const ATOM_S2_TV1_STANDARD_MASKb0: c_uint = 0x0F;
pub const ATOM_S2_CURRENT_BL_LEVEL_MASKb1: c_uint = 0xFF;
pub const ATOM_S2_DEVICE_DPMS_STATEb2: c_uint = 0x01;
pub const ATOM_S2_DEVICE_DPMS_MASKw1: c_uint = 0x3FF;
pub const ATOM_S2_FORCEDLOWPWRMODE_STATE_MASKb3: c_uint = 0x0C;
pub const ATOM_S2_FORCEDLOWPWRMODE_STATE_CHANGEb3: c_uint = 0x10;
pub const ATOM_S2_TMDS_COHERENT_MODEb3: c_uint = 0x10          // used by VBIOS code only, use coherent mode for TMDS/HDMI mode;
pub const ATOM_S2_VRI_BRIGHT_ENABLEb3: c_uint = 0x20;
pub const ATOM_S2_ROTATION_STATE_MASKb3: c_uint = 0xC0;
// BIOS_3_SCRATCH Definition
pub const ATOM_S3_CRT1_ACTIVE: c_uint = 0x00000001L;
pub const ATOM_S3_LCD1_ACTIVE: c_uint = 0x00000002L;
pub const ATOM_S3_TV1_ACTIVE: c_uint = 0x00000004L;
pub const ATOM_S3_DFP1_ACTIVE: c_uint = 0x00000008L;
pub const ATOM_S3_CRT2_ACTIVE: c_uint = 0x00000010L;
pub const ATOM_S3_LCD2_ACTIVE: c_uint = 0x00000020L;
pub const ATOM_S3_DFP6_ACTIVE: c_uint = 0x00000040L;
pub const ATOM_S3_DFP2_ACTIVE: c_uint = 0x00000080L;
pub const ATOM_S3_CV_ACTIVE: c_uint = 0x00000100L;
pub const ATOM_S3_DFP3_ACTIVE: c_uint = 0x00000200L;
pub const ATOM_S3_DFP4_ACTIVE: c_uint = 0x00000400L;
pub const ATOM_S3_DFP5_ACTIVE: c_uint = 0x00000800L;
pub const ATOM_S3_DEVICE_ACTIVE_MASK: c_uint = 0x00000FFFL;
pub const ATOM_S3_LCD_FULLEXPANSION_ACTIVE: c_uint = 0x00001000L;
pub const ATOM_S3_LCD_EXPANSION_ASPEC_RATIO_ACTIVE: c_uint = 0x00002000L;
pub const ATOM_S3_CRT1_CRTC_ACTIVE: c_uint = 0x00010000L;
pub const ATOM_S3_LCD1_CRTC_ACTIVE: c_uint = 0x00020000L;
pub const ATOM_S3_TV1_CRTC_ACTIVE: c_uint = 0x00040000L;
pub const ATOM_S3_DFP1_CRTC_ACTIVE: c_uint = 0x00080000L;
pub const ATOM_S3_CRT2_CRTC_ACTIVE: c_uint = 0x00100000L;
pub const ATOM_S3_LCD2_CRTC_ACTIVE: c_uint = 0x00200000L;
pub const ATOM_S3_DFP6_CRTC_ACTIVE: c_uint = 0x00400000L;
pub const ATOM_S3_DFP2_CRTC_ACTIVE: c_uint = 0x00800000L;
pub const ATOM_S3_CV_CRTC_ACTIVE: c_uint = 0x01000000L;
pub const ATOM_S3_DFP3_CRTC_ACTIVE: c_uint = 0x02000000L;
pub const ATOM_S3_DFP4_CRTC_ACTIVE: c_uint = 0x04000000L;
pub const ATOM_S3_DFP5_CRTC_ACTIVE: c_uint = 0x08000000L;
pub const ATOM_S3_DEVICE_CRTC_ACTIVE_MASK: c_uint = 0x0FFF0000L;
pub const ATOM_S3_ASIC_GUI_ENGINE_HUNG: c_uint = 0x20000000L;
// Below two definitions are not supported in pplib, but in the old powerplay in DAL
pub const ATOM_S3_ALLOW_FAST_PWR_SWITCH: c_uint = 0x40000000L;
pub const ATOM_S3_RQST_GPU_USE_MIN_PWR: c_uint = 0x80000000L;
// Byte aligned definition for BIOS usage
pub const ATOM_S3_CRT1_ACTIVEb0: c_uint = 0x01;
pub const ATOM_S3_LCD1_ACTIVEb0: c_uint = 0x02;
pub const ATOM_S3_TV1_ACTIVEb0: c_uint = 0x04;
pub const ATOM_S3_DFP1_ACTIVEb0: c_uint = 0x08;
pub const ATOM_S3_CRT2_ACTIVEb0: c_uint = 0x10;
pub const ATOM_S3_LCD2_ACTIVEb0: c_uint = 0x20;
pub const ATOM_S3_DFP6_ACTIVEb0: c_uint = 0x40;
pub const ATOM_S3_DFP2_ACTIVEb0: c_uint = 0x80;
pub const ATOM_S3_CV_ACTIVEb1: c_uint = 0x01;
pub const ATOM_S3_DFP3_ACTIVEb1: c_uint = 0x02;
pub const ATOM_S3_DFP4_ACTIVEb1: c_uint = 0x04;
pub const ATOM_S3_DFP5_ACTIVEb1: c_uint = 0x08;
pub const ATOM_S3_ACTIVE_CRTC1w0: c_uint = 0xFFF;
pub const ATOM_S3_CRT1_CRTC_ACTIVEb2: c_uint = 0x01;
pub const ATOM_S3_LCD1_CRTC_ACTIVEb2: c_uint = 0x02;
pub const ATOM_S3_TV1_CRTC_ACTIVEb2: c_uint = 0x04;
pub const ATOM_S3_DFP1_CRTC_ACTIVEb2: c_uint = 0x08;
pub const ATOM_S3_CRT2_CRTC_ACTIVEb2: c_uint = 0x10;
pub const ATOM_S3_LCD2_CRTC_ACTIVEb2: c_uint = 0x20;
pub const ATOM_S3_DFP6_CRTC_ACTIVEb2: c_uint = 0x40;
pub const ATOM_S3_DFP2_CRTC_ACTIVEb2: c_uint = 0x80;
pub const ATOM_S3_CV_CRTC_ACTIVEb3: c_uint = 0x01;
pub const ATOM_S3_DFP3_CRTC_ACTIVEb3: c_uint = 0x02;
pub const ATOM_S3_DFP4_CRTC_ACTIVEb3: c_uint = 0x04;
pub const ATOM_S3_DFP5_CRTC_ACTIVEb3: c_uint = 0x08;
pub const ATOM_S3_ACTIVE_CRTC2w1: c_uint = 0xFFF;
// BIOS_4_SCRATCH Definition
pub const ATOM_S4_LCD1_PANEL_ID_MASK: c_uint = 0x000000FFL;
pub const ATOM_S4_LCD1_REFRESH_MASK: c_uint = 0x0000FF00L;
pub const ATOM_S4_LCD1_REFRESH_SHIFT: c_int = 8;
// Byte aligned definition for BIOS usage
pub const ATOM_S4_LCD1_PANEL_ID_MASKb0: c_uint = 0x0FF;

// BIOS_5_SCRATCH Definition, BIOS_5_SCRATCH is used by Firmware only !!!!
pub const ATOM_S5_DOS_REQ_CRT1b0: c_uint = 0x01;
pub const ATOM_S5_DOS_REQ_LCD1b0: c_uint = 0x02;
pub const ATOM_S5_DOS_REQ_TV1b0: c_uint = 0x04;
pub const ATOM_S5_DOS_REQ_DFP1b0: c_uint = 0x08;
pub const ATOM_S5_DOS_REQ_CRT2b0: c_uint = 0x10;
pub const ATOM_S5_DOS_REQ_LCD2b0: c_uint = 0x20;
pub const ATOM_S5_DOS_REQ_DFP6b0: c_uint = 0x40;
pub const ATOM_S5_DOS_REQ_DFP2b0: c_uint = 0x80;
pub const ATOM_S5_DOS_REQ_CVb1: c_uint = 0x01;
pub const ATOM_S5_DOS_REQ_DFP3b1: c_uint = 0x02;
pub const ATOM_S5_DOS_REQ_DFP4b1: c_uint = 0x04;
pub const ATOM_S5_DOS_REQ_DFP5b1: c_uint = 0x08;
pub const ATOM_S5_DOS_REQ_DEVICEw0: c_uint = 0x0FFF;
pub const ATOM_S5_DOS_REQ_CRT1: c_uint = 0x0001;
pub const ATOM_S5_DOS_REQ_LCD1: c_uint = 0x0002;
pub const ATOM_S5_DOS_REQ_TV1: c_uint = 0x0004;
pub const ATOM_S5_DOS_REQ_DFP1: c_uint = 0x0008;
pub const ATOM_S5_DOS_REQ_CRT2: c_uint = 0x0010;
pub const ATOM_S5_DOS_REQ_LCD2: c_uint = 0x0020;
pub const ATOM_S5_DOS_REQ_DFP6: c_uint = 0x0040;
pub const ATOM_S5_DOS_REQ_DFP2: c_uint = 0x0080;
pub const ATOM_S5_DOS_REQ_CV: c_uint = 0x0100;
pub const ATOM_S5_DOS_REQ_DFP3: c_uint = 0x0200;
pub const ATOM_S5_DOS_REQ_DFP4: c_uint = 0x0400;
pub const ATOM_S5_DOS_REQ_DFP5: c_uint = 0x0800;

// BIOS_6_SCRATCH Definition
pub const ATOM_S6_DEVICE_CHANGE: c_uint = 0x00000001L;
pub const ATOM_S6_SCALER_CHANGE: c_uint = 0x00000002L;
pub const ATOM_S6_LID_CHANGE: c_uint = 0x00000004L;
pub const ATOM_S6_DOCKING_CHANGE: c_uint = 0x00000008L;
pub const ATOM_S6_ACC_MODE: c_uint = 0x00000010L;
pub const ATOM_S6_EXT_DESKTOP_MODE: c_uint = 0x00000020L;
pub const ATOM_S6_LID_STATE: c_uint = 0x00000040L;
pub const ATOM_S6_DOCK_STATE: c_uint = 0x00000080L;
pub const ATOM_S6_CRITICAL_STATE: c_uint = 0x00000100L;
pub const ATOM_S6_HW_I2C_BUSY_STATE: c_uint = 0x00000200L;
pub const ATOM_S6_THERMAL_STATE_CHANGE: c_uint = 0x00000400L;
pub const ATOM_S6_INTERRUPT_SET_BY_BIOS: c_uint = 0x00000800L;
pub const ATOM_S6_REQ_LCD_EXPANSION_FULL: c_uint = 0x00001000L //Normal expansion Request bit for LCD;
pub const ATOM_S6_REQ_LCD_EXPANSION_ASPEC_RATIO: c_uint = 0x00002000L //Aspect ratio expansion Request bit for LCD;
pub const ATOM_S6_DISPLAY_STATE_CHANGE: c_uint = 0x00004000L        //This bit is recycled when ATOM_BIOS_INFO_BIOS_SCRATCH6_SCL2_REDEFINE is set,previously it's SCL2_H_expansion;
pub const ATOM_S6_I2C_STATE_CHANGE: c_uint = 0x00008000L        //This bit is recycled,when ATOM_BIOS_INFO_BIOS_SCRATCH6_SCL2_REDEFINE is set,previously it's SCL2_V_expansion;
pub const ATOM_S6_ACC_REQ_CRT1: c_uint = 0x00010000L;
pub const ATOM_S6_ACC_REQ_LCD1: c_uint = 0x00020000L;
pub const ATOM_S6_ACC_REQ_TV1: c_uint = 0x00040000L;
pub const ATOM_S6_ACC_REQ_DFP1: c_uint = 0x00080000L;
pub const ATOM_S6_ACC_REQ_CRT2: c_uint = 0x00100000L;
pub const ATOM_S6_ACC_REQ_LCD2: c_uint = 0x00200000L;
pub const ATOM_S6_ACC_REQ_DFP6: c_uint = 0x00400000L;
pub const ATOM_S6_ACC_REQ_DFP2: c_uint = 0x00800000L;
pub const ATOM_S6_ACC_REQ_CV: c_uint = 0x01000000L;
pub const ATOM_S6_ACC_REQ_DFP3: c_uint = 0x02000000L;
pub const ATOM_S6_ACC_REQ_DFP4: c_uint = 0x04000000L;
pub const ATOM_S6_ACC_REQ_DFP5: c_uint = 0x08000000L;
pub const ATOM_S6_ACC_REQ_MASK: c_uint = 0x0FFF0000L;
pub const ATOM_S6_SYSTEM_POWER_MODE_CHANGE: c_uint = 0x10000000L;
pub const ATOM_S6_ACC_BLOCK_DISPLAY_SWITCH: c_uint = 0x20000000L;
pub const ATOM_S6_VRI_BRIGHTNESS_CHANGE: c_uint = 0x40000000L;
pub const ATOM_S6_CONFIG_DISPLAY_CHANGE_MASK: c_uint = 0x80000000L;
// Byte aligned definition for BIOS usage
pub const ATOM_S6_DEVICE_CHANGEb0: c_uint = 0x01;
pub const ATOM_S6_SCALER_CHANGEb0: c_uint = 0x02;
pub const ATOM_S6_LID_CHANGEb0: c_uint = 0x04;
pub const ATOM_S6_DOCKING_CHANGEb0: c_uint = 0x08;
pub const ATOM_S6_ACC_MODEb0: c_uint = 0x10;
pub const ATOM_S6_EXT_DESKTOP_MODEb0: c_uint = 0x20;
pub const ATOM_S6_LID_STATEb0: c_uint = 0x40;
pub const ATOM_S6_DOCK_STATEb0: c_uint = 0x80;
pub const ATOM_S6_CRITICAL_STATEb1: c_uint = 0x01;
pub const ATOM_S6_HW_I2C_BUSY_STATEb1: c_uint = 0x02;
pub const ATOM_S6_THERMAL_STATE_CHANGEb1: c_uint = 0x04;
pub const ATOM_S6_INTERRUPT_SET_BY_BIOSb1: c_uint = 0x08;
pub const ATOM_S6_REQ_LCD_EXPANSION_FULLb1: c_uint = 0x10;
pub const ATOM_S6_REQ_LCD_EXPANSION_ASPEC_RATIOb1: c_uint = 0x20;
pub const ATOM_S6_ACC_REQ_CRT1b2: c_uint = 0x01;
pub const ATOM_S6_ACC_REQ_LCD1b2: c_uint = 0x02;
pub const ATOM_S6_ACC_REQ_TV1b2: c_uint = 0x04;
pub const ATOM_S6_ACC_REQ_DFP1b2: c_uint = 0x08;
pub const ATOM_S6_ACC_REQ_CRT2b2: c_uint = 0x10;
pub const ATOM_S6_ACC_REQ_LCD2b2: c_uint = 0x20;
pub const ATOM_S6_ACC_REQ_DFP6b2: c_uint = 0x40;
pub const ATOM_S6_ACC_REQ_DFP2b2: c_uint = 0x80;
pub const ATOM_S6_ACC_REQ_CVb3: c_uint = 0x01;
pub const ATOM_S6_ACC_REQ_DFP3b3: c_uint = 0x02;
pub const ATOM_S6_ACC_REQ_DFP4b3: c_uint = 0x04;
pub const ATOM_S6_ACC_REQ_DFP5b3: c_uint = 0x08;

pub const ATOM_S6_SYSTEM_POWER_MODE_CHANGEb3: c_uint = 0x10;
pub const ATOM_S6_ACC_BLOCK_DISPLAY_SWITCHb3: c_uint = 0x20;
pub const ATOM_S6_VRI_BRIGHTNESS_CHANGEb3: c_uint = 0x40;
pub const ATOM_S6_CONFIG_DISPLAY_CHANGEb3: c_uint = 0x80;
pub const ATOM_S6_DEVICE_CHANGE_SHIFT: c_int = 0;
pub const ATOM_S6_SCALER_CHANGE_SHIFT: c_int = 1;
pub const ATOM_S6_LID_CHANGE_SHIFT: c_int = 2;
pub const ATOM_S6_DOCKING_CHANGE_SHIFT: c_int = 3;
pub const ATOM_S6_ACC_MODE_SHIFT: c_int = 4;
pub const ATOM_S6_EXT_DESKTOP_MODE_SHIFT: c_int = 5;
pub const ATOM_S6_LID_STATE_SHIFT: c_int = 6;
pub const ATOM_S6_DOCK_STATE_SHIFT: c_int = 7;
pub const ATOM_S6_CRITICAL_STATE_SHIFT: c_int = 8;
pub const ATOM_S6_HW_I2C_BUSY_STATE_SHIFT: c_int = 9;
pub const ATOM_S6_THERMAL_STATE_CHANGE_SHIFT: c_int = 10;
pub const ATOM_S6_INTERRUPT_SET_BY_BIOS_SHIFT: c_int = 11;
pub const ATOM_S6_REQ_SCALER_SHIFT: c_int = 12;
pub const ATOM_S6_REQ_SCALER_ARATIO_SHIFT: c_int = 13;
pub const ATOM_S6_DISPLAY_STATE_CHANGE_SHIFT: c_int = 14;
pub const ATOM_S6_I2C_STATE_CHANGE_SHIFT: c_int = 15;
pub const ATOM_S6_SYSTEM_POWER_MODE_CHANGE_SHIFT: c_int = 28;
pub const ATOM_S6_ACC_BLOCK_DISPLAY_SWITCH_SHIFT: c_int = 29;
pub const ATOM_S6_VRI_BRIGHTNESS_CHANGE_SHIFT: c_int = 30;
pub const ATOM_S6_CONFIG_DISPLAY_CHANGE_SHIFT: c_int = 31;
// BIOS_7_SCRATCH Definition, BIOS_7_SCRATCH is used by Firmware only !!!!
pub const ATOM_S7_DOS_MODE_TYPEb0: c_uint = 0x03;
pub const ATOM_S7_DOS_MODE_VGAb0: c_uint = 0x00;
pub const ATOM_S7_DOS_MODE_VESAb0: c_uint = 0x01;
pub const ATOM_S7_DOS_MODE_EXTb0: c_uint = 0x02;
pub const ATOM_S7_DOS_MODE_PIXEL_DEPTHb0: c_uint = 0x0C;
pub const ATOM_S7_DOS_MODE_PIXEL_FORMATb0: c_uint = 0xF0;
pub const ATOM_S7_DOS_8BIT_DAC_ENb1: c_uint = 0x01;
pub const ATOM_S7_ASIC_INIT_COMPLETEb1: c_uint = 0x02;
pub const ATOM_S7_ASIC_INIT_COMPLETE_MASK: c_uint = 0x00000200;
pub const ATOM_S7_DOS_MODE_NUMBERw1: c_uint = 0x0FFFF;
pub const ATOM_S7_DOS_8BIT_DAC_EN_SHIFT: c_int = 8;
// BIOS_8_SCRATCH Definition
pub const ATOM_S8_I2C_CHANNEL_BUSY_MASK: c_uint = 0x00000FFFF;
pub const ATOM_S8_I2C_HW_ENGINE_BUSY_MASK: c_uint = 0x0FFFF0000;
pub const ATOM_S8_I2C_CHANNEL_BUSY_SHIFT: c_int = 0;
pub const ATOM_S8_I2C_ENGINE_BUSY_SHIFT: c_int = 16;
// BIOS_9_SCRATCH Definition

pub const ATOM_S9_I2C_CHANNEL_COMPLETED_MASK: c_uint = 0x0000FFFF;

pub const ATOM_S9_I2C_CHANNEL_ABORTED_MASK: c_uint = 0xFFFF0000;

pub const ATOM_S9_I2C_CHANNEL_COMPLETED_SHIFT: c_int = 0;

pub const ATOM_S9_I2C_CHANNEL_ABORTED_SHIFT: c_int = 16;

pub const ATOM_FLAG_SET: c_uint = 0x20;
pub const ATOM_FLAG_CLEAR: c_int = 0;

//
// Portion II: Definitions only used in Driver
//
// Macros used by driver

//
// Portion III: Definitinos only used in VBIOS
//
pub const ATOM_DAC_SRC: c_uint = 0x80;
pub const ATOM_SRC_DAC1: c_int = 0;
pub const ATOM_SRC_DAC2: c_uint = 0x80;

pub const GPIO_PIN_WRITE: c_uint = 0x01;
pub const GPIO_PIN_READ: c_uint = 0x00;

// ucEnable:
pub const SCALER_BYPASS_AUTO_CENTER_NO_REPLICATION: c_int = 0;
pub const SCALER_BYPASS_AUTO_CENTER_AUTO_REPLICATION: c_int = 1;
pub const SCALER_ENABLE_2TAP_ALPHA_MODE: c_int = 2;
pub const SCALER_ENABLE_MULTITAP_MODE: c_int = 3;
// ucEnable
pub const ATOM_GRAPH_CONTROL_SET_PITCH: c_uint = 0x0f;
pub const ATOM_GRAPH_CONTROL_SET_DISP_START: c_uint = 0x10;

// ucAction:
pub const PALETTE_DATA_AUTO_FILL: c_int = 1;
pub const PALETTE_DATA_READ: c_int = 2;
pub const PALETTE_DATA_WRITE: c_int = 3;
// ucInterruptId
pub const HDP1_INTERRUPT_ID: c_int = 1;
pub const HDP2_INTERRUPT_ID: c_int = 2;
pub const HDP3_INTERRUPT_ID: c_int = 3;
pub const HDP4_INTERRUPT_ID: c_int = 4;
pub const HDP5_INTERRUPT_ID: c_int = 5;
pub const HDP6_INTERRUPT_ID: c_int = 6;
pub const SW_INTERRUPT_ID: c_int = 11;
// ucAction
pub const INTERRUPT_SERVICE_GEN_SW_INT: c_int = 1;
pub const INTERRUPT_SERVICE_GET_STATUS: c_int = 2;
// ucStatus
pub const INTERRUPT_STATUS__INT_TRIGGER: c_int = 1;
pub const INTERRUPT_STATUS__HPD_HIGH: c_int = 2;
pub const INDIRECT_READ: c_uint = 0x00;
pub const INDIRECT_WRITE: c_uint = 0x80;
pub const INDIRECT_IO_MM: c_int = 0;
pub const INDIRECT_IO_PLL: c_int = 1;
pub const INDIRECT_IO_MC: c_int = 2;
pub const INDIRECT_IO_PCIE: c_int = 3;
pub const INDIRECT_IO_PCIEP: c_int = 4;
pub const INDIRECT_IO_NBMISC: c_int = 5;
pub const INDIRECT_IO_SMU: c_int = 5;

// ATOM Memory Related Data Structure

pub const END_OF_REG_INDEX_BLOCK: c_uint = 0x0ffff;
pub const END_OF_REG_DATA_BLOCK: c_uint = 0x00000000;
pub const ATOM_INIT_REG_MASK_FLAG: c_uint = 0x80               //Not used in BIOS;
pub const CLOCK_RANGE_HIGHEST: c_uint = 0x00ffffff;

pub const VALUE_SAME_AS_ABOVE: c_int = 0;
pub const VALUE_MASK_DWORD: c_uint = 0x84;

// #define ACCESS_MCIODEBUGIND            0x40       //defined in BIOS code
pub const ACCESS_PLACEHOLDER: c_uint = 0x80;
pub const _4Mx16: c_uint = 0x2;
pub const _4Mx32: c_uint = 0x3;
pub const _8Mx16: c_uint = 0x12;
pub const _8Mx32: c_uint = 0x13;
pub const _16Mx16: c_uint = 0x22;
pub const _16Mx32: c_uint = 0x23;
pub const _32Mx16: c_uint = 0x32;
pub const _32Mx32: c_uint = 0x33;
pub const _64Mx8: c_uint = 0x41;
pub const _64Mx16: c_uint = 0x42;
pub const _64Mx32: c_uint = 0x43;
pub const _128Mx8: c_uint = 0x51;
pub const _128Mx16: c_uint = 0x52;
pub const _128Mx32: c_uint = 0x53;
pub const _256Mx8: c_uint = 0x61;
pub const _256Mx16: c_uint = 0x62;
pub const _512Mx8: c_uint = 0x71;
pub const SAMSUNG: c_uint = 0x1;
pub const INFINEON: c_uint = 0x2;
pub const ELPIDA: c_uint = 0x3;
pub const ETRON: c_uint = 0x4;
pub const NANYA: c_uint = 0x5;
pub const HYNIX: c_uint = 0x6;
pub const MOSEL: c_uint = 0x7;
pub const WINBOND: c_uint = 0x8;
pub const ESMT: c_uint = 0x9;
pub const MICRON: c_uint = 0xF;

// Support for GDDR5 MC uCode to reside in upper 64K of ROM
pub const UCODE_ROM_START_ADDRESS: c_uint = 0x1b800;
pub const UCODE_SIGNATURE: c_uint = 0x4375434d // 'MCuC' - MC uCode;
// uCode block header for reference
//
pub const ATOM_MAX_NUMBER_OF_VRAM_MODULE: c_int = 16;
pub const ATOM_VRAM_MODULE_MEMORY_VENDOR_ID_MASK: c_uint = 0xF;
// GDDR parameters
// ATOM_VRAM_MODULE_V3.ucNPL_RT
pub const NPL_RT_MASK: c_uint = 0x0f;
pub const BATTERY_ODT_MASK: c_uint = 0xc0;

// MC_ARB_RAMCFG (includes NOOFBANK,NOOFRANKS,NOOFROWS,NOOFCOLS)
// Total memory size in unit of 16MB for CONFIG_MEMSIZE - bit[23:0] zeros
// compare with V3, we flat the struct by merging ATOM_MEMORY_FORMAT (as is) into V4 as the same level
pub const VRAM_MODULE_V4_MISC_RANK_MASK: c_uint = 0x3;
pub const VRAM_MODULE_V4_MISC_DUAL_RANK: c_uint = 0x1;
pub const VRAM_MODULE_V4_MISC_BL_MASK: c_uint = 0x4;
pub const VRAM_MODULE_V4_MISC_BL8: c_uint = 0x4;
pub const VRAM_MODULE_V4_MISC_DUAL_CS: c_uint = 0x10;
// MC_ARB_RAMCFG (includes NOOFBANK,NOOFRANKS,NOOFROWS,NOOFCOLS)
// Total memory size in unit of 16MB for CONFIG_MEMSIZE - bit[23:0] zeros
// compare with V3, we flat the struct by merging ATOM_MEMORY_FORMAT (as is) into V4 as the same level
// MC_ARB_RAMCFG (includes NOOFBANK,NOOFRANKS,NOOFROWS,NOOFCOLS)
// Total memory size in unit of 16MB for CONFIG_MEMSIZE - bit[23:0] zeros
// compare with V3, we flat the struct by merging ATOM_MEMORY_FORMAT (as is) into V4 as the same level
// Design Specific Values
// Memory Module specific values
// ATOM_INIT_REG_BLOCK				 aMemAdjust;

// ATOM_INIT_REG_BLOCK				 aMemAdjust;

// SW I2C CNTL DEFINITIONS
pub const SW_I2C_IO_RESET: c_int = 0;
pub const SW_I2C_IO_GET: c_int = 1;
pub const SW_I2C_IO_DRIVE: c_int = 2;
pub const SW_I2C_IO_SET: c_int = 3;
pub const SW_I2C_IO_START: c_int = 4;
pub const SW_I2C_IO_CLOCK: c_int = 0;
pub const SW_I2C_IO_DATA: c_uint = 0x80;
pub const SW_I2C_IO_ZERO: c_int = 0;
pub const SW_I2C_IO_ONE: c_uint = 0x100;
pub const SW_I2C_CNTL_READ: c_int = 0;
pub const SW_I2C_CNTL_WRITE: c_int = 1;
pub const SW_I2C_CNTL_START: c_int = 2;
pub const SW_I2C_CNTL_STOP: c_int = 3;
pub const SW_I2C_CNTL_OPEN: c_int = 4;
pub const SW_I2C_CNTL_CLOSE: c_int = 5;
pub const SW_I2C_CNTL_WRITE1BIT: c_int = 6;
// ==============================VESA definition Portion===============================

pub const VESA_MODE_ATTRIBUTE_MODE_SUPPORT: c_uint = 0xBB	//refer to VBE spec p.32, no TTY support;
pub const VESA_MODE_WIN_ATTRIBUTE: c_int = 7;
pub const VESA_WIN_SIZE: c_int = 64;
// Mandatory information for all VBE revisions
// ; Mandatory information for VBE 1.2 and above
// ; Direct Color fields(required for direct/6 and YUV/7 memory models)
// ; Mandatory information for VBE 2.0 and above
// ; Mandatory information for VBE 3.0 and above
// BIOS function CALLS
pub const ATOM_BIOS_EXTENDED_FUNCTION_CODE: c_uint = 0xA0	        // ATI Extended Function code;
pub const ATOM_BIOS_FUNCTION_COP_MODE: c_uint = 0x00;
pub const ATOM_BIOS_FUNCTION_SHORT_QUERY1: c_uint = 0x04;
pub const ATOM_BIOS_FUNCTION_SHORT_QUERY2: c_uint = 0x05;
pub const ATOM_BIOS_FUNCTION_SHORT_QUERY3: c_uint = 0x06;
pub const ATOM_BIOS_FUNCTION_GET_DDC: c_uint = 0x0B;
pub const ATOM_BIOS_FUNCTION_ASIC_DSTATE: c_uint = 0x0E;
pub const ATOM_BIOS_FUNCTION_DEBUG_PLAY: c_uint = 0x0F;
pub const ATOM_BIOS_FUNCTION_STV_STD: c_uint = 0x16;
pub const ATOM_BIOS_FUNCTION_DEVICE_DET: c_uint = 0x17;
pub const ATOM_BIOS_FUNCTION_DEVICE_SWITCH: c_uint = 0x18;
pub const ATOM_BIOS_FUNCTION_PANEL_CONTROL: c_uint = 0x82;
pub const ATOM_BIOS_FUNCTION_OLD_DEVICE_DET: c_uint = 0x83;
pub const ATOM_BIOS_FUNCTION_OLD_DEVICE_SWITCH: c_uint = 0x84;
pub const ATOM_BIOS_FUNCTION_HW_ICON: c_uint = 0x8A;
pub const ATOM_BIOS_FUNCTION_SET_CMOS: c_uint = 0x8B;
pub const SUB_FUNCTION_UPDATE_DISPLAY_INFO: c_uint = 0x8000          // Sub function 80;
pub const SUB_FUNCTION_UPDATE_EXPANSION_INFO: c_uint = 0x8100          // Sub function 80;
pub const ATOM_BIOS_FUNCTION_DISPLAY_INFO: c_uint = 0x8D;
pub const ATOM_BIOS_FUNCTION_DEVICE_ON_OFF: c_uint = 0x8E;
pub const ATOM_BIOS_FUNCTION_VIDEO_STATE: c_uint = 0x8F;
pub const ATOM_SUB_FUNCTION_GET_CRITICAL_STATE: c_uint = 0x0300          // Sub function 03;
pub const ATOM_SUB_FUNCTION_GET_LIDSTATE: c_uint = 0x0700          // Sub function 7;
pub const ATOM_SUB_FUNCTION_THERMAL_STATE_NOTICE: c_uint = 0x1400          // Notify caller the current thermal state;
pub const ATOM_SUB_FUNCTION_CRITICAL_STATE_NOTICE: c_uint = 0x8300          // Notify caller the current critical state;
pub const ATOM_SUB_FUNCTION_SET_LIDSTATE: c_uint = 0x8500          // Sub function 85;
pub const ATOM_SUB_FUNCTION_GET_REQ_DISPLAY_FROM_SBIOS_MODE: c_uint = 0x8900// Sub function 89;
pub const ATOM_SUB_FUNCTION_INFORM_ADC_SUPPORT: c_uint = 0x9400          // Notify caller that ADC is supported;
pub const ATOM_BIOS_FUNCTION_VESA_DPMS: c_uint = 0x4F10          // Set DPMS;
pub const ATOM_SUB_FUNCTION_SET_DPMS: c_uint = 0x0001          // BL: Sub function 01;
pub const ATOM_SUB_FUNCTION_GET_DPMS: c_uint = 0x0002          // BL: Sub function 02;
pub const ATOM_PARAMETER_VESA_DPMS_ON: c_uint = 0x0000          // BH Parameter for DPMS ON.;
pub const ATOM_PARAMETER_VESA_DPMS_STANDBY: c_uint = 0x0100          // BH Parameter for DPMS STANDBY;
pub const ATOM_PARAMETER_VESA_DPMS_SUSPEND: c_uint = 0x0200          // BH Parameter for DPMS SUSPEND;
pub const ATOM_PARAMETER_VESA_DPMS_OFF: c_uint = 0x0400          // BH Parameter for DPMS OFF;
pub const ATOM_PARAMETER_VESA_DPMS_REDUCE_ON: c_uint = 0x0800          // BH Parameter for DPMS REDUCE ON (NOT SUPPORTED);
pub const ATOM_BIOS_RETURN_CODE_MASK: c_uint = 0x0000FF00L;
pub const ATOM_BIOS_REG_HIGH_MASK: c_uint = 0x0000FF00L;
pub const ATOM_BIOS_REG_LOW_MASK: c_uint = 0x000000FFL;
// structure used for VBIOS only
// DispOutInfoTable
pub const ASIC_TRANSMITTER_INFO_CONFIG__DVO_SDR_MODE: c_uint = 0x01;
pub const ASIC_TRANSMITTER_INFO_CONFIG__COHERENT_MODE: c_uint = 0x02;
pub const ASIC_TRANSMITTER_INFO_CONFIG__ENCODEROBJ_ID_MASK: c_uint = 0xc4;
pub const ASIC_TRANSMITTER_INFO_CONFIG__ENCODER_A: c_uint = 0x00;
pub const ASIC_TRANSMITTER_INFO_CONFIG__ENCODER_B: c_uint = 0x04;
pub const ASIC_TRANSMITTER_INFO_CONFIG__ENCODER_C: c_uint = 0x40;
pub const ASIC_TRANSMITTER_INFO_CONFIG__ENCODER_D: c_uint = 0x44;
pub const ASIC_TRANSMITTER_INFO_CONFIG__ENCODER_E: c_uint = 0x80;
pub const ASIC_TRANSMITTER_INFO_CONFIG__ENCODER_F: c_uint = 0x84;
// ucPpllAttribute
pub const CLOCK_SOURCE_SHAREABLE: c_uint = 0x01;
pub const CLOCK_SOURCE_DP_MODE: c_uint = 0x02;
pub const CLOCK_SOURCE_NONE_DP_MODE: c_uint = 0x04;
// DispOutInfoTable
// ucDispCaps
pub const DISPLAY_CAPS__DP_PCLK_FROM_PPLL: c_uint = 0x01;
pub const DISPLAY_CAPS__FORCE_DISPDEV_CONNECTED: c_uint = 0x02;
// DispDevicePriorityInfo
// ProcessAuxChannelTransactionTable

// GetSinkType
// ucAction
pub const ATOM_DP_ACTION_GET_SINK_TYPE: c_uint = 0x01;
// obsolete
pub const ATOM_DP_ACTION_TRAINING_START: c_uint = 0x02;
pub const ATOM_DP_ACTION_TRAINING_COMPLETE: c_uint = 0x03;
pub const ATOM_DP_ACTION_TRAINING_PATTERN_SEL: c_uint = 0x04;
pub const ATOM_DP_ACTION_SET_VSWING_PREEMP: c_uint = 0x05;
pub const ATOM_DP_ACTION_GET_VSWING_PREEMP: c_uint = 0x06;
pub const ATOM_DP_ACTION_BLANKING: c_uint = 0x07;
// ucConfig
pub const ATOM_DP_CONFIG_ENCODER_SEL_MASK: c_uint = 0x03;
pub const ATOM_DP_CONFIG_DIG1_ENCODER: c_uint = 0x00;
pub const ATOM_DP_CONFIG_DIG2_ENCODER: c_uint = 0x01;
pub const ATOM_DP_CONFIG_EXTERNAL_ENCODER: c_uint = 0x02;
pub const ATOM_DP_CONFIG_LINK_SEL_MASK: c_uint = 0x04;
pub const ATOM_DP_CONFIG_LINK_A: c_uint = 0x00;
pub const ATOM_DP_CONFIG_LINK_B: c_uint = 0x04;
// /obselete

// ucAction
pub const DP_SERVICE_V2_ACTION_GET_SINK_TYPE: c_uint = 0x01;
pub const DP_SERVICE_V2_ACTION_DET_LCD_CONNECTION: c_uint = 0x02;
// DP_TRAINING_TABLE

// ucFlag
pub const HW_I2C_WRITE: c_int = 1;
pub const HW_I2C_READ: c_int = 0;
pub const I2C_2BYTE_ADDR: c_uint = 0x02;
//
// Structures used by HW_Misc_OperationTable
//
// Actions code
pub const ATOM_GET_SDI_SUPPORT: c_uint = 0xF0;
// Return code
pub const ATOM_UNKNOWN_CMD: c_int = 0;
pub const ATOM_FEATURE_NOT_SUPPORTED: c_int = 1;
pub const ATOM_FEATURE_SUPPORTED: c_int = 2;
//
pub const HWBLKINST_INSTANCE_MASK: c_uint = 0x07;
pub const HWBLKINST_HWBLK_MASK: c_uint = 0xF0;
pub const HWBLKINST_HWBLK_SHIFT: c_uint = 0x04;
// ucHWBlock
pub const SELECT_DISP_ENGINE: c_int = 0;
pub const SELECT_DISP_PLL: c_int = 1;
pub const SELECT_DCIO_UNIPHY_LINK0: c_int = 2;
pub const SELECT_DCIO_UNIPHY_LINK1: c_int = 3;
pub const SELECT_DCIO_IMPCAL: c_int = 4;
pub const SELECT_DCIO_DIG: c_int = 6;
pub const SELECT_CRTC_PIXEL_RATE: c_int = 7;
pub const SELECT_VGA_BLK: c_int = 8;
// DIGTransmitterInfoTable structure used to program UNIPHY settings
// ucGfxBlkId
pub const GFX_HARVESTING_CU_ID: c_int = 0;
pub const GFX_HARVESTING_RB_ID: c_int = 1;
pub const GFX_HARVESTING_PRIM_ID: c_int = 2;
//
// Portion VI: Definitinos for vbios MC scratch registers that driver used
//
pub const MC_MISC0__MEMORY_TYPE_MASK: c_uint = 0xF0000000;
pub const MC_MISC0__MEMORY_TYPE__GDDR1: c_uint = 0x10000000;
pub const MC_MISC0__MEMORY_TYPE__DDR2: c_uint = 0x20000000;
pub const MC_MISC0__MEMORY_TYPE__GDDR3: c_uint = 0x30000000;
pub const MC_MISC0__MEMORY_TYPE__GDDR4: c_uint = 0x40000000;
pub const MC_MISC0__MEMORY_TYPE__GDDR5: c_uint = 0x50000000;
pub const MC_MISC0__MEMORY_TYPE__HBM: c_uint = 0x60000000;
pub const MC_MISC0__MEMORY_TYPE__DDR3: c_uint = 0xB0000000;

//
// Portion VI: Definitinos being oboselete
//
// ==========================================================================================
// Remove the definitions below when driver is ready!
// ==============================  DAC1 portion
// ==============================  DAC2 portion
// Supported Device Info Table Definitions
// ucConnectInfo:
// [7:4] - connector type
// = 1   - VGA connector
// = 2   - DVI-I
// = 3   - DVI-D
// = 4   - DVI-A
// = 5   - SVIDEO
// = 6   - COMPOSITE
// = 7   - LVDS
// = 8   - DIGITAL LINK
// = 9   - SCART
// = 0xA - HDMI_type A
// = 0xB - HDMI_type B
// = 0xE - Special case1 (DVI+DIN)
// Others=TBD
// [3:0] - DAC Associated
// = 0   - no DAC
// = 1   - DACA
// = 2   - DACB
// = 3   - External DAC
// Others=TBD
//

pub const NO_INT_SRC_MAPPED: c_uint = 0xFF;

pub const ATOM_MAX_MISC_INFO: c_int = 4;
pub const ATOM_XTMDS_ASIC_SI164_ID: c_int = 1;
pub const ATOM_XTMDS_ASIC_SI178_ID: c_int = 2;
pub const ATOM_XTMDS_ASIC_TFP513_ID: c_int = 3;
pub const ATOM_XTMDS_SUPPORTED_SINGLELINK: c_uint = 0x00000001;
pub const ATOM_XTMDS_SUPPORTED_DUALLINK: c_uint = 0x00000002;
pub const ATOM_XTMDS_MVPU_FPGA: c_uint = 0x00000004;
// due to design. This ID is used to alert driver that the sequence is not "standard"!
// Legacy Power Play Table Definitions
// Definitions for ulPowerPlayMiscInfo
pub const ATOM_PM_MISCINFO_SPLIT_CLOCK: c_uint = 0x00000000L;
pub const ATOM_PM_MISCINFO_USING_MCLK_SRC: c_uint = 0x00000001L;
pub const ATOM_PM_MISCINFO_USING_SCLK_SRC: c_uint = 0x00000002L;
pub const ATOM_PM_MISCINFO_VOLTAGE_DROP_SUPPORT: c_uint = 0x00000004L;
pub const ATOM_PM_MISCINFO_VOLTAGE_DROP_ACTIVE_HIGH: c_uint = 0x00000008L;
pub const ATOM_PM_MISCINFO_LOAD_PERFORMANCE_EN: c_uint = 0x00000010L;
pub const ATOM_PM_MISCINFO_ENGINE_CLOCK_CONTRL_EN: c_uint = 0x00000020L;
pub const ATOM_PM_MISCINFO_MEMORY_CLOCK_CONTRL_EN: c_uint = 0x00000040L;
pub const ATOM_PM_MISCINFO_PROGRAM_VOLTAGE: c_uint = 0x00000080L  //When this bit set, ucVoltageDropIndex is not an index for GPIO pin, but a voltage ID that SW needs program;
pub const ATOM_PM_MISCINFO_ASIC_REDUCED_SPEED_SCLK_EN: c_uint = 0x00000100L;
pub const ATOM_PM_MISCINFO_ASIC_DYNAMIC_VOLTAGE_EN: c_uint = 0x00000200L;
pub const ATOM_PM_MISCINFO_ASIC_SLEEP_MODE_EN: c_uint = 0x00000400L;
pub const ATOM_PM_MISCINFO_LOAD_BALANCE_EN: c_uint = 0x00000800L;
pub const ATOM_PM_MISCINFO_DEFAULT_DC_STATE_ENTRY_TRUE: c_uint = 0x00001000L;
pub const ATOM_PM_MISCINFO_DEFAULT_LOW_DC_STATE_ENTRY_TRUE: c_uint = 0x00002000L;
pub const ATOM_PM_MISCINFO_LOW_LCD_REFRESH_RATE: c_uint = 0x00004000L;
pub const ATOM_PM_MISCINFO_DRIVER_DEFAULT_MODE: c_uint = 0x00008000L;
pub const ATOM_PM_MISCINFO_OVER_CLOCK_MODE: c_uint = 0x00010000L;
pub const ATOM_PM_MISCINFO_OVER_DRIVE_MODE: c_uint = 0x00020000L;
pub const ATOM_PM_MISCINFO_POWER_SAVING_MODE: c_uint = 0x00040000L;
pub const ATOM_PM_MISCINFO_THERMAL_DIODE_MODE: c_uint = 0x00080000L;
pub const ATOM_PM_MISCINFO_FRAME_MODULATION_MASK: c_uint = 0x00300000L  //0-FM Disable, 1-2 level FM, 2-4 level FM, 3-Reserved;
pub const ATOM_PM_MISCINFO_FRAME_MODULATION_SHIFT: c_int = 20;
pub const ATOM_PM_MISCINFO_DYN_CLK_3D_IDLE: c_uint = 0x00400000L;
pub const ATOM_PM_MISCINFO_DYNAMIC_CLOCK_DIVIDER_BY_2: c_uint = 0x00800000L;
pub const ATOM_PM_MISCINFO_DYNAMIC_CLOCK_DIVIDER_BY_4: c_uint = 0x01000000L;
pub const ATOM_PM_MISCINFO_DYNAMIC_HDP_BLOCK_EN: c_uint = 0x02000000L  //When set, Dynamic;
pub const ATOM_PM_MISCINFO_DYNAMIC_MC_HOST_BLOCK_EN: c_uint = 0x04000000L  //When set, Dynamic;
pub const ATOM_PM_MISCINFO_3D_ACCELERATION_EN: c_uint = 0x08000000L  //When set, This mode is for acceleated 3D mode;
pub const ATOM_PM_MISCINFO_POWERPLAY_SETTINGS_GROUP_MASK: c_uint = 0x70000000L  //1-Optimal Battery Life Group, 2-High Battery, 3-Balanced, 4-High Performance, 5- Optimal Performance (Default state with Default clocks);
pub const ATOM_PM_MISCINFO_POWERPLAY_SETTINGS_GROUP_SHIFT: c_int = 28;
pub const ATOM_PM_MISCINFO_ENABLE_BACK_BIAS: c_uint = 0x80000000L;
pub const ATOM_PM_MISCINFO2_SYSTEM_AC_LITE_MODE: c_uint = 0x00000001L;
pub const ATOM_PM_MISCINFO2_MULTI_DISPLAY_SUPPORT: c_uint = 0x00000002L;
pub const ATOM_PM_MISCINFO2_DYNAMIC_BACK_BIAS_EN: c_uint = 0x00000004L;
pub const ATOM_PM_MISCINFO2_FS3D_OVERDRIVE_INFO: c_uint = 0x00000008L;
pub const ATOM_PM_MISCINFO2_FORCEDLOWPWR_MODE: c_uint = 0x00000010L;
pub const ATOM_PM_MISCINFO2_VDDCI_DYNAMIC_VOLTAGE_EN: c_uint = 0x00000020L;
pub const ATOM_PM_MISCINFO2_VIDEO_PLAYBACK_CAPABLE: c_uint = 0x00000040L  //If this bit is set in multi-pp mode, then driver will pack up one with the minior power consumption.;
// If it's not set in any pp mode, driver will use its default logic to pick a pp mode in video playback
pub const ATOM_PM_MISCINFO2_NOT_VALID_ON_DC: c_uint = 0x00000080L;
pub const ATOM_PM_MISCINFO2_STUTTER_MODE_EN: c_uint = 0x00000100L;
pub const ATOM_PM_MISCINFO2_UVD_SUPPORT_MODE: c_uint = 0x00000200L;
// ucTableFormatRevision=1
// ucTableContentRevision=1
// ucTableFormatRevision=2
// ucTableContentRevision=1
// ucTableFormatRevision=2
// ucTableContentRevision=2
pub const ATOM_MAX_NUMBEROF_POWER_BLOCK: c_int = 8;
pub const ATOM_PP_OVERDRIVE_INTBITMAP_AUXWIN: c_uint = 0x01;
pub const ATOM_PP_OVERDRIVE_INTBITMAP_OVERDRIVE: c_uint = 0x02;
pub const ATOM_PP_OVERDRIVE_THERMALCONTROLLER_LM63: c_uint = 0x01;
pub const ATOM_PP_OVERDRIVE_THERMALCONTROLLER_ADM1032: c_uint = 0x02;
pub const ATOM_PP_OVERDRIVE_THERMALCONTROLLER_ADM1030: c_uint = 0x03;
pub const ATOM_PP_OVERDRIVE_THERMALCONTROLLER_MUA6649: c_uint = 0x04;
pub const ATOM_PP_OVERDRIVE_THERMALCONTROLLER_LM64: c_uint = 0x05;
pub const ATOM_PP_OVERDRIVE_THERMALCONTROLLER_F75375: c_uint = 0x06;
pub const ATOM_PP_OVERDRIVE_THERMALCONTROLLER_ASC7512: c_uint = 0x07	// Andigilog;
// Following definitions are for compatibility issue in different SW components.
pub const ATOM_MASTER_DATA_TABLE_REVISION: c_uint = 0x01;

// New device naming, remove them when both DAL/VBIOS is ready

pub const ATOM_DEVICE_DFP2I_INDEX: c_uint = 0x00000009;

pub const ATOM_S0_DFP2I: c_uint = 0x00200000L;
pub const ATOM_S0_DFP2Ib2: c_uint = 0x20;

pub const ATOM_S2_DFP2I_DPMS_STATE: c_uint = 0x02000000L;
pub const ATOM_S2_DFP2I_DPMS_STATEb3: c_uint = 0x02;
pub const ATOM_S3_DFP2I_ACTIVEb1: c_uint = 0x02;

pub const ATOM_S3_DFP2I_ACTIVE: c_uint = 0x00000200L;

pub const ATOM_S3_DFP2I_CRTC_ACTIVE: c_uint = 0x02000000L;
pub const ATOM_S3_DFP2I_CRTC_ACTIVEb3: c_uint = 0x02;
pub const ATOM_S5_DOS_REQ_DFP2Ib1: c_uint = 0x02;
pub const ATOM_S5_DOS_REQ_DFP2I: c_uint = 0x0200;

pub const ATOM_S6_ACC_REQ_DFP2Ib3: c_uint = 0x02;
pub const ATOM_S6_ACC_REQ_DFP2I: c_uint = 0x02000000L;

// These two lines will be removed for sure in a few days, will follow up with Michael V.

// #define ATOM_S2_CRT1_DPMS_STATE         0x00010000L
// #define ATOM_S2_LCD1_DPMS_STATE	        ATOM_S2_CRT1_DPMS_STATE
// #define ATOM_S2_TV1_DPMS_STATE          ATOM_S2_CRT1_DPMS_STATE
// #define ATOM_S2_DFP1_DPMS_STATE         ATOM_S2_CRT1_DPMS_STATE
// #define ATOM_S2_CRT2_DPMS_STATE         ATOM_S2_CRT1_DPMS_STATE
pub const ATOM_S6_ACC_REQ_TV2: c_uint = 0x00400000L;
pub const ATOM_DEVICE_TV2_INDEX: c_uint = 0x00000006;

pub const ATOM_S0_TV2: c_uint = 0x00100000L;

//
pub const ATOM_S2_CRT1_DPMS_STATE: c_uint = 0x00010000L;
pub const ATOM_S2_LCD1_DPMS_STATE: c_uint = 0x00020000L;
pub const ATOM_S2_TV1_DPMS_STATE: c_uint = 0x00040000L;
pub const ATOM_S2_DFP1_DPMS_STATE: c_uint = 0x00080000L;
pub const ATOM_S2_CRT2_DPMS_STATE: c_uint = 0x00100000L;
pub const ATOM_S2_LCD2_DPMS_STATE: c_uint = 0x00200000L;
pub const ATOM_S2_TV2_DPMS_STATE: c_uint = 0x00400000L;
pub const ATOM_S2_DFP2_DPMS_STATE: c_uint = 0x00800000L;
pub const ATOM_S2_CV_DPMS_STATE: c_uint = 0x01000000L;
pub const ATOM_S2_DFP3_DPMS_STATE: c_uint = 0x02000000L;
pub const ATOM_S2_DFP4_DPMS_STATE: c_uint = 0x04000000L;
pub const ATOM_S2_DFP5_DPMS_STATE: c_uint = 0x08000000L;
pub const ATOM_S2_CRT1_DPMS_STATEb2: c_uint = 0x01;
pub const ATOM_S2_LCD1_DPMS_STATEb2: c_uint = 0x02;
pub const ATOM_S2_TV1_DPMS_STATEb2: c_uint = 0x04;
pub const ATOM_S2_DFP1_DPMS_STATEb2: c_uint = 0x08;
pub const ATOM_S2_CRT2_DPMS_STATEb2: c_uint = 0x10;
pub const ATOM_S2_LCD2_DPMS_STATEb2: c_uint = 0x20;
pub const ATOM_S2_TV2_DPMS_STATEb2: c_uint = 0x40;
pub const ATOM_S2_DFP2_DPMS_STATEb2: c_uint = 0x80;
pub const ATOM_S2_CV_DPMS_STATEb3: c_uint = 0x01;
pub const ATOM_S2_DFP3_DPMS_STATEb3: c_uint = 0x02;
pub const ATOM_S2_DFP4_DPMS_STATEb3: c_uint = 0x04;
pub const ATOM_S2_DFP5_DPMS_STATEb3: c_uint = 0x08;
pub const ATOM_S3_ASIC_GUI_ENGINE_HUNGb3: c_uint = 0x20;
pub const ATOM_S3_ALLOW_FAST_PWR_SWITCHb3: c_uint = 0x40;
pub const ATOM_S3_RQST_GPU_USE_MIN_PWRb3: c_uint = 0x80;
//

//
// AMD ACPI Table
//

//
// EFI_ACPI_DESCRIPTION_HEADER from AcpiCommon.h
//

