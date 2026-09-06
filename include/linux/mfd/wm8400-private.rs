//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/wm8400-private.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// wm8400 private definitions.
//
// Copyright 2008 Wolfson Microelectronics plc
//

pub const WM8400_REGISTER_COUNT: c_uint = 0x55;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm8400 {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub regulators: [platform_device; 6],
}

//
// Register values.
//
pub const WM8400_RESET_ID: c_uint = 0x00;
pub const WM8400_ID: c_uint = 0x01;
pub const WM8400_POWER_MANAGEMENT_1: c_uint = 0x02;
pub const WM8400_POWER_MANAGEMENT_2: c_uint = 0x03;
pub const WM8400_POWER_MANAGEMENT_3: c_uint = 0x04;
pub const WM8400_AUDIO_INTERFACE_1: c_uint = 0x05;
pub const WM8400_AUDIO_INTERFACE_2: c_uint = 0x06;
pub const WM8400_CLOCKING_1: c_uint = 0x07;
pub const WM8400_CLOCKING_2: c_uint = 0x08;
pub const WM8400_AUDIO_INTERFACE_3: c_uint = 0x09;
pub const WM8400_AUDIO_INTERFACE_4: c_uint = 0x0A;
pub const WM8400_DAC_CTRL: c_uint = 0x0B;
pub const WM8400_LEFT_DAC_DIGITAL_VOLUME: c_uint = 0x0C;
pub const WM8400_RIGHT_DAC_DIGITAL_VOLUME: c_uint = 0x0D;
pub const WM8400_DIGITAL_SIDE_TONE: c_uint = 0x0E;
pub const WM8400_ADC_CTRL: c_uint = 0x0F;
pub const WM8400_LEFT_ADC_DIGITAL_VOLUME: c_uint = 0x10;
pub const WM8400_RIGHT_ADC_DIGITAL_VOLUME: c_uint = 0x11;
pub const WM8400_GPIO_CTRL_1: c_uint = 0x12;
pub const WM8400_GPIO1_GPIO2: c_uint = 0x13;
pub const WM8400_GPIO3_GPIO4: c_uint = 0x14;
pub const WM8400_GPIO5_GPIO6: c_uint = 0x15;
pub const WM8400_GPIOCTRL_2: c_uint = 0x16;
pub const WM8400_GPIO_POL: c_uint = 0x17;
pub const WM8400_LEFT_LINE_INPUT_1_2_VOLUME: c_uint = 0x18;
pub const WM8400_LEFT_LINE_INPUT_3_4_VOLUME: c_uint = 0x19;
pub const WM8400_RIGHT_LINE_INPUT_1_2_VOLUME: c_uint = 0x1A;
pub const WM8400_RIGHT_LINE_INPUT_3_4_VOLUME: c_uint = 0x1B;
pub const WM8400_LEFT_OUTPUT_VOLUME: c_uint = 0x1C;
pub const WM8400_RIGHT_OUTPUT_VOLUME: c_uint = 0x1D;
pub const WM8400_LINE_OUTPUTS_VOLUME: c_uint = 0x1E;
pub const WM8400_OUT3_4_VOLUME: c_uint = 0x1F;
pub const WM8400_LEFT_OPGA_VOLUME: c_uint = 0x20;
pub const WM8400_RIGHT_OPGA_VOLUME: c_uint = 0x21;
pub const WM8400_SPEAKER_VOLUME: c_uint = 0x22;
pub const WM8400_CLASSD1: c_uint = 0x23;
pub const WM8400_CLASSD3: c_uint = 0x25;
pub const WM8400_INPUT_MIXER1: c_uint = 0x27;
pub const WM8400_INPUT_MIXER2: c_uint = 0x28;
pub const WM8400_INPUT_MIXER3: c_uint = 0x29;
pub const WM8400_INPUT_MIXER4: c_uint = 0x2A;
pub const WM8400_INPUT_MIXER5: c_uint = 0x2B;
pub const WM8400_INPUT_MIXER6: c_uint = 0x2C;
pub const WM8400_OUTPUT_MIXER1: c_uint = 0x2D;
pub const WM8400_OUTPUT_MIXER2: c_uint = 0x2E;
pub const WM8400_OUTPUT_MIXER3: c_uint = 0x2F;
pub const WM8400_OUTPUT_MIXER4: c_uint = 0x30;
pub const WM8400_OUTPUT_MIXER5: c_uint = 0x31;
pub const WM8400_OUTPUT_MIXER6: c_uint = 0x32;
pub const WM8400_OUT3_4_MIXER: c_uint = 0x33;
pub const WM8400_LINE_MIXER1: c_uint = 0x34;
pub const WM8400_LINE_MIXER2: c_uint = 0x35;
pub const WM8400_SPEAKER_MIXER: c_uint = 0x36;
pub const WM8400_ADDITIONAL_CONTROL: c_uint = 0x37;
pub const WM8400_ANTIPOP1: c_uint = 0x38;
pub const WM8400_ANTIPOP2: c_uint = 0x39;
pub const WM8400_MICBIAS: c_uint = 0x3A;
pub const WM8400_FLL_CONTROL_1: c_uint = 0x3C;
pub const WM8400_FLL_CONTROL_2: c_uint = 0x3D;
pub const WM8400_FLL_CONTROL_3: c_uint = 0x3E;
pub const WM8400_FLL_CONTROL_4: c_uint = 0x3F;
pub const WM8400_LDO1_CONTROL: c_uint = 0x41;
pub const WM8400_LDO2_CONTROL: c_uint = 0x42;
pub const WM8400_LDO3_CONTROL: c_uint = 0x43;
pub const WM8400_LDO4_CONTROL: c_uint = 0x44;
pub const WM8400_DCDC1_CONTROL_1: c_uint = 0x46;
pub const WM8400_DCDC1_CONTROL_2: c_uint = 0x47;
pub const WM8400_DCDC2_CONTROL_1: c_uint = 0x48;
pub const WM8400_DCDC2_CONTROL_2: c_uint = 0x49;
pub const WM8400_INTERFACE: c_uint = 0x4B;
pub const WM8400_PM_GENERAL: c_uint = 0x4C;
pub const WM8400_PM_SHUTDOWN_CONTROL: c_uint = 0x4E;
pub const WM8400_INTERRUPT_STATUS_1: c_uint = 0x4F;
pub const WM8400_INTERRUPT_STATUS_1_MASK: c_uint = 0x50;
pub const WM8400_INTERRUPT_LEVELS: c_uint = 0x51;
pub const WM8400_SHUTDOWN_REASON: c_uint = 0x52;
pub const WM8400_LINE_CIRCUITS: c_uint = 0x54;
//
// Field Definitions.
//
// R0 (0x00) - Reset/ID
//
pub const WM8400_SW_RESET_CHIP_ID_MASK: c_uint = 0xFFFF  /* SW_RESET/CHIP_ID - [15:0] */;

//
// R1 (0x01) - ID
//
pub const WM8400_CHIP_REV_MASK: c_uint = 0x7000  /* CHIP_REV - [14:12] */;

//
// R18 (0x12) - GPIO CTRL 1
//
pub const WM8400_IRQ: c_uint = 0x1000  /* IRQ */;
pub const WM8400_IRQ_MASK: c_uint = 0x1000  /* IRQ */;

pub const WM8400_TEMPOK: c_uint = 0x0800  /* TEMPOK */;
pub const WM8400_TEMPOK_MASK: c_uint = 0x0800  /* TEMPOK */;

pub const WM8400_MIC1SHRT: c_uint = 0x0400  /* MIC1SHRT */;
pub const WM8400_MIC1SHRT_MASK: c_uint = 0x0400  /* MIC1SHRT */;

pub const WM8400_MIC1DET: c_uint = 0x0200  /* MIC1DET */;
pub const WM8400_MIC1DET_MASK: c_uint = 0x0200  /* MIC1DET */;

pub const WM8400_FLL_LCK: c_uint = 0x0100  /* FLL_LCK */;
pub const WM8400_FLL_LCK_MASK: c_uint = 0x0100  /* FLL_LCK */;

pub const WM8400_GPIO_STATUS_MASK: c_uint = 0x00FF  /* GPIO_STATUS - [7:0] */;

//
// R19 (0x13) - GPIO1 & GPIO2
//
pub const WM8400_GPIO2_DEB_ENA: c_uint = 0x8000  /* GPIO2_DEB_ENA */;
pub const WM8400_GPIO2_DEB_ENA_MASK: c_uint = 0x8000  /* GPIO2_DEB_ENA */;

pub const WM8400_GPIO2_IRQ_ENA: c_uint = 0x4000  /* GPIO2_IRQ_ENA */;
pub const WM8400_GPIO2_IRQ_ENA_MASK: c_uint = 0x4000  /* GPIO2_IRQ_ENA */;

pub const WM8400_GPIO2_PU: c_uint = 0x2000  /* GPIO2_PU */;
pub const WM8400_GPIO2_PU_MASK: c_uint = 0x2000  /* GPIO2_PU */;

pub const WM8400_GPIO2_PD: c_uint = 0x1000  /* GPIO2_PD */;
pub const WM8400_GPIO2_PD_MASK: c_uint = 0x1000  /* GPIO2_PD */;

pub const WM8400_GPIO2_SEL_MASK: c_uint = 0x0F00  /* GPIO2_SEL - [11:8] */;

pub const WM8400_GPIO1_DEB_ENA: c_uint = 0x0080  /* GPIO1_DEB_ENA */;
pub const WM8400_GPIO1_DEB_ENA_MASK: c_uint = 0x0080  /* GPIO1_DEB_ENA */;

pub const WM8400_GPIO1_IRQ_ENA: c_uint = 0x0040  /* GPIO1_IRQ_ENA */;
pub const WM8400_GPIO1_IRQ_ENA_MASK: c_uint = 0x0040  /* GPIO1_IRQ_ENA */;

pub const WM8400_GPIO1_PU: c_uint = 0x0020  /* GPIO1_PU */;
pub const WM8400_GPIO1_PU_MASK: c_uint = 0x0020  /* GPIO1_PU */;

pub const WM8400_GPIO1_PD: c_uint = 0x0010  /* GPIO1_PD */;
pub const WM8400_GPIO1_PD_MASK: c_uint = 0x0010  /* GPIO1_PD */;

pub const WM8400_GPIO1_SEL_MASK: c_uint = 0x000F  /* GPIO1_SEL - [3:0] */;

//
// R20 (0x14) - GPIO3 & GPIO4
//
pub const WM8400_GPIO4_DEB_ENA: c_uint = 0x8000  /* GPIO4_DEB_ENA */;
pub const WM8400_GPIO4_DEB_ENA_MASK: c_uint = 0x8000  /* GPIO4_DEB_ENA */;

pub const WM8400_GPIO4_IRQ_ENA: c_uint = 0x4000  /* GPIO4_IRQ_ENA */;
pub const WM8400_GPIO4_IRQ_ENA_MASK: c_uint = 0x4000  /* GPIO4_IRQ_ENA */;

pub const WM8400_GPIO4_PU: c_uint = 0x2000  /* GPIO4_PU */;
pub const WM8400_GPIO4_PU_MASK: c_uint = 0x2000  /* GPIO4_PU */;

pub const WM8400_GPIO4_PD: c_uint = 0x1000  /* GPIO4_PD */;
pub const WM8400_GPIO4_PD_MASK: c_uint = 0x1000  /* GPIO4_PD */;

pub const WM8400_GPIO4_SEL_MASK: c_uint = 0x0F00  /* GPIO4_SEL - [11:8] */;

pub const WM8400_GPIO3_DEB_ENA: c_uint = 0x0080  /* GPIO3_DEB_ENA */;
pub const WM8400_GPIO3_DEB_ENA_MASK: c_uint = 0x0080  /* GPIO3_DEB_ENA */;

pub const WM8400_GPIO3_IRQ_ENA: c_uint = 0x0040  /* GPIO3_IRQ_ENA */;
pub const WM8400_GPIO3_IRQ_ENA_MASK: c_uint = 0x0040  /* GPIO3_IRQ_ENA */;

pub const WM8400_GPIO3_PU: c_uint = 0x0020  /* GPIO3_PU */;
pub const WM8400_GPIO3_PU_MASK: c_uint = 0x0020  /* GPIO3_PU */;

pub const WM8400_GPIO3_PD: c_uint = 0x0010  /* GPIO3_PD */;
pub const WM8400_GPIO3_PD_MASK: c_uint = 0x0010  /* GPIO3_PD */;

pub const WM8400_GPIO3_SEL_MASK: c_uint = 0x000F  /* GPIO3_SEL - [3:0] */;

//
// R21 (0x15) - GPIO5 & GPIO6
//
pub const WM8400_GPIO6_DEB_ENA: c_uint = 0x8000  /* GPIO6_DEB_ENA */;
pub const WM8400_GPIO6_DEB_ENA_MASK: c_uint = 0x8000  /* GPIO6_DEB_ENA */;

pub const WM8400_GPIO6_IRQ_ENA: c_uint = 0x4000  /* GPIO6_IRQ_ENA */;
pub const WM8400_GPIO6_IRQ_ENA_MASK: c_uint = 0x4000  /* GPIO6_IRQ_ENA */;

pub const WM8400_GPIO6_PU: c_uint = 0x2000  /* GPIO6_PU */;
pub const WM8400_GPIO6_PU_MASK: c_uint = 0x2000  /* GPIO6_PU */;

pub const WM8400_GPIO6_PD: c_uint = 0x1000  /* GPIO6_PD */;
pub const WM8400_GPIO6_PD_MASK: c_uint = 0x1000  /* GPIO6_PD */;

pub const WM8400_GPIO6_SEL_MASK: c_uint = 0x0F00  /* GPIO6_SEL - [11:8] */;

pub const WM8400_GPIO5_DEB_ENA: c_uint = 0x0080  /* GPIO5_DEB_ENA */;
pub const WM8400_GPIO5_DEB_ENA_MASK: c_uint = 0x0080  /* GPIO5_DEB_ENA */;

pub const WM8400_GPIO5_IRQ_ENA: c_uint = 0x0040  /* GPIO5_IRQ_ENA */;
pub const WM8400_GPIO5_IRQ_ENA_MASK: c_uint = 0x0040  /* GPIO5_IRQ_ENA */;

pub const WM8400_GPIO5_PU: c_uint = 0x0020  /* GPIO5_PU */;
pub const WM8400_GPIO5_PU_MASK: c_uint = 0x0020  /* GPIO5_PU */;

pub const WM8400_GPIO5_PD: c_uint = 0x0010  /* GPIO5_PD */;
pub const WM8400_GPIO5_PD_MASK: c_uint = 0x0010  /* GPIO5_PD */;

pub const WM8400_GPIO5_SEL_MASK: c_uint = 0x000F  /* GPIO5_SEL - [3:0] */;

//
// R22 (0x16) - GPIOCTRL 2
//
pub const WM8400_TEMPOK_IRQ_ENA: c_uint = 0x0800  /* TEMPOK_IRQ_ENA */;
pub const WM8400_TEMPOK_IRQ_ENA_MASK: c_uint = 0x0800  /* TEMPOK_IRQ_ENA */;

pub const WM8400_MIC1SHRT_IRQ_ENA: c_uint = 0x0400  /* MIC1SHRT_IRQ_ENA */;
pub const WM8400_MIC1SHRT_IRQ_ENA_MASK: c_uint = 0x0400  /* MIC1SHRT_IRQ_ENA */;

pub const WM8400_MIC1DET_IRQ_ENA: c_uint = 0x0200  /* MIC1DET_IRQ_ENA */;
pub const WM8400_MIC1DET_IRQ_ENA_MASK: c_uint = 0x0200  /* MIC1DET_IRQ_ENA */;

pub const WM8400_FLL_LCK_IRQ_ENA: c_uint = 0x0100  /* FLL_LCK_IRQ_ENA */;
pub const WM8400_FLL_LCK_IRQ_ENA_MASK: c_uint = 0x0100  /* FLL_LCK_IRQ_ENA */;

pub const WM8400_GPI8_DEB_ENA: c_uint = 0x0080  /* GPI8_DEB_ENA */;
pub const WM8400_GPI8_DEB_ENA_MASK: c_uint = 0x0080  /* GPI8_DEB_ENA */;

pub const WM8400_GPI8_IRQ_ENA: c_uint = 0x0040  /* GPI8_IRQ_ENA */;
pub const WM8400_GPI8_IRQ_ENA_MASK: c_uint = 0x0040  /* GPI8_IRQ_ENA */;

pub const WM8400_GPI8_ENA: c_uint = 0x0010  /* GPI8_ENA */;
pub const WM8400_GPI8_ENA_MASK: c_uint = 0x0010  /* GPI8_ENA */;

pub const WM8400_GPI7_DEB_ENA: c_uint = 0x0008  /* GPI7_DEB_ENA */;
pub const WM8400_GPI7_DEB_ENA_MASK: c_uint = 0x0008  /* GPI7_DEB_ENA */;

pub const WM8400_GPI7_IRQ_ENA: c_uint = 0x0004  /* GPI7_IRQ_ENA */;
pub const WM8400_GPI7_IRQ_ENA_MASK: c_uint = 0x0004  /* GPI7_IRQ_ENA */;

pub const WM8400_GPI7_ENA: c_uint = 0x0001  /* GPI7_ENA */;
pub const WM8400_GPI7_ENA_MASK: c_uint = 0x0001  /* GPI7_ENA */;

//
// R23 (0x17) - GPIO_POL
//
pub const WM8400_IRQ_INV: c_uint = 0x1000  /* IRQ_INV */;
pub const WM8400_IRQ_INV_MASK: c_uint = 0x1000  /* IRQ_INV */;

pub const WM8400_TEMPOK_POL: c_uint = 0x0800  /* TEMPOK_POL */;
pub const WM8400_TEMPOK_POL_MASK: c_uint = 0x0800  /* TEMPOK_POL */;

pub const WM8400_MIC1SHRT_POL: c_uint = 0x0400  /* MIC1SHRT_POL */;
pub const WM8400_MIC1SHRT_POL_MASK: c_uint = 0x0400  /* MIC1SHRT_POL */;

pub const WM8400_MIC1DET_POL: c_uint = 0x0200  /* MIC1DET_POL */;
pub const WM8400_MIC1DET_POL_MASK: c_uint = 0x0200  /* MIC1DET_POL */;

pub const WM8400_FLL_LCK_POL: c_uint = 0x0100  /* FLL_LCK_POL */;
pub const WM8400_FLL_LCK_POL_MASK: c_uint = 0x0100  /* FLL_LCK_POL */;

pub const WM8400_GPIO_POL_MASK: c_uint = 0x00FF  /* GPIO_POL - [7:0] */;

//
// R65 (0x41) - LDO 1 Control
//
pub const WM8400_LDO1_ENA: c_uint = 0x8000  /* LDO1_ENA */;
pub const WM8400_LDO1_ENA_MASK: c_uint = 0x8000  /* LDO1_ENA */;

pub const WM8400_LDO1_SWI: c_uint = 0x4000  /* LDO1_SWI */;
pub const WM8400_LDO1_SWI_MASK: c_uint = 0x4000  /* LDO1_SWI */;

pub const WM8400_LDO1_OPFLT: c_uint = 0x1000  /* LDO1_OPFLT */;
pub const WM8400_LDO1_OPFLT_MASK: c_uint = 0x1000  /* LDO1_OPFLT */;

pub const WM8400_LDO1_ERRACT: c_uint = 0x0800  /* LDO1_ERRACT */;
pub const WM8400_LDO1_ERRACT_MASK: c_uint = 0x0800  /* LDO1_ERRACT */;

pub const WM8400_LDO1_HIB_MODE: c_uint = 0x0400  /* LDO1_HIB_MODE */;
pub const WM8400_LDO1_HIB_MODE_MASK: c_uint = 0x0400  /* LDO1_HIB_MODE */;

pub const WM8400_LDO1_VIMG_MASK: c_uint = 0x03E0  /* LDO1_VIMG - [9:5] */;

pub const WM8400_LDO1_VSEL_MASK: c_uint = 0x001F  /* LDO1_VSEL - [4:0] */;

//
// R66 (0x42) - LDO 2 Control
//
pub const WM8400_LDO2_ENA: c_uint = 0x8000  /* LDO2_ENA */;
pub const WM8400_LDO2_ENA_MASK: c_uint = 0x8000  /* LDO2_ENA */;

pub const WM8400_LDO2_SWI: c_uint = 0x4000  /* LDO2_SWI */;
pub const WM8400_LDO2_SWI_MASK: c_uint = 0x4000  /* LDO2_SWI */;

pub const WM8400_LDO2_OPFLT: c_uint = 0x1000  /* LDO2_OPFLT */;
pub const WM8400_LDO2_OPFLT_MASK: c_uint = 0x1000  /* LDO2_OPFLT */;

pub const WM8400_LDO2_ERRACT: c_uint = 0x0800  /* LDO2_ERRACT */;
pub const WM8400_LDO2_ERRACT_MASK: c_uint = 0x0800  /* LDO2_ERRACT */;

pub const WM8400_LDO2_HIB_MODE: c_uint = 0x0400  /* LDO2_HIB_MODE */;
pub const WM8400_LDO2_HIB_MODE_MASK: c_uint = 0x0400  /* LDO2_HIB_MODE */;

pub const WM8400_LDO2_VIMG_MASK: c_uint = 0x03E0  /* LDO2_VIMG - [9:5] */;

pub const WM8400_LDO2_VSEL_MASK: c_uint = 0x001F  /* LDO2_VSEL - [4:0] */;

//
// R67 (0x43) - LDO 3 Control
//
pub const WM8400_LDO3_ENA: c_uint = 0x8000  /* LDO3_ENA */;
pub const WM8400_LDO3_ENA_MASK: c_uint = 0x8000  /* LDO3_ENA */;

pub const WM8400_LDO3_SWI: c_uint = 0x4000  /* LDO3_SWI */;
pub const WM8400_LDO3_SWI_MASK: c_uint = 0x4000  /* LDO3_SWI */;

pub const WM8400_LDO3_OPFLT: c_uint = 0x1000  /* LDO3_OPFLT */;
pub const WM8400_LDO3_OPFLT_MASK: c_uint = 0x1000  /* LDO3_OPFLT */;

pub const WM8400_LDO3_ERRACT: c_uint = 0x0800  /* LDO3_ERRACT */;
pub const WM8400_LDO3_ERRACT_MASK: c_uint = 0x0800  /* LDO3_ERRACT */;

pub const WM8400_LDO3_HIB_MODE: c_uint = 0x0400  /* LDO3_HIB_MODE */;
pub const WM8400_LDO3_HIB_MODE_MASK: c_uint = 0x0400  /* LDO3_HIB_MODE */;

pub const WM8400_LDO3_VIMG_MASK: c_uint = 0x03E0  /* LDO3_VIMG - [9:5] */;

pub const WM8400_LDO3_VSEL_MASK: c_uint = 0x001F  /* LDO3_VSEL - [4:0] */;

//
// R68 (0x44) - LDO 4 Control
//
pub const WM8400_LDO4_ENA: c_uint = 0x8000  /* LDO4_ENA */;
pub const WM8400_LDO4_ENA_MASK: c_uint = 0x8000  /* LDO4_ENA */;

pub const WM8400_LDO4_SWI: c_uint = 0x4000  /* LDO4_SWI */;
pub const WM8400_LDO4_SWI_MASK: c_uint = 0x4000  /* LDO4_SWI */;

pub const WM8400_LDO4_OPFLT: c_uint = 0x1000  /* LDO4_OPFLT */;
pub const WM8400_LDO4_OPFLT_MASK: c_uint = 0x1000  /* LDO4_OPFLT */;

pub const WM8400_LDO4_ERRACT: c_uint = 0x0800  /* LDO4_ERRACT */;
pub const WM8400_LDO4_ERRACT_MASK: c_uint = 0x0800  /* LDO4_ERRACT */;

pub const WM8400_LDO4_HIB_MODE: c_uint = 0x0400  /* LDO4_HIB_MODE */;
pub const WM8400_LDO4_HIB_MODE_MASK: c_uint = 0x0400  /* LDO4_HIB_MODE */;

pub const WM8400_LDO4_VIMG_MASK: c_uint = 0x03E0  /* LDO4_VIMG - [9:5] */;

pub const WM8400_LDO4_VSEL_MASK: c_uint = 0x001F  /* LDO4_VSEL - [4:0] */;

//
// R70 (0x46) - DCDC1 Control 1
//
pub const WM8400_DC1_ENA: c_uint = 0x8000  /* DC1_ENA */;
pub const WM8400_DC1_ENA_MASK: c_uint = 0x8000  /* DC1_ENA */;

pub const WM8400_DC1_ACTIVE: c_uint = 0x4000  /* DC1_ACTIVE */;
pub const WM8400_DC1_ACTIVE_MASK: c_uint = 0x4000  /* DC1_ACTIVE */;

pub const WM8400_DC1_SLEEP: c_uint = 0x2000  /* DC1_SLEEP */;
pub const WM8400_DC1_SLEEP_MASK: c_uint = 0x2000  /* DC1_SLEEP */;

pub const WM8400_DC1_OPFLT: c_uint = 0x1000  /* DC1_OPFLT */;
pub const WM8400_DC1_OPFLT_MASK: c_uint = 0x1000  /* DC1_OPFLT */;

pub const WM8400_DC1_ERRACT: c_uint = 0x0800  /* DC1_ERRACT */;
pub const WM8400_DC1_ERRACT_MASK: c_uint = 0x0800  /* DC1_ERRACT */;

pub const WM8400_DC1_HIB_MODE: c_uint = 0x0400  /* DC1_HIB_MODE */;
pub const WM8400_DC1_HIB_MODE_MASK: c_uint = 0x0400  /* DC1_HIB_MODE */;

pub const WM8400_DC1_SOFTST_MASK: c_uint = 0x0300  /* DC1_SOFTST - [9:8] */;

pub const WM8400_DC1_OV_PROT: c_uint = 0x0080  /* DC1_OV_PROT */;
pub const WM8400_DC1_OV_PROT_MASK: c_uint = 0x0080  /* DC1_OV_PROT */;

pub const WM8400_DC1_VSEL_MASK: c_uint = 0x007F  /* DC1_VSEL - [6:0] */;

//
// R71 (0x47) - DCDC1 Control 2
//
pub const WM8400_DC1_FRC_PWM: c_uint = 0x2000  /* DC1_FRC_PWM */;
pub const WM8400_DC1_FRC_PWM_MASK: c_uint = 0x2000  /* DC1_FRC_PWM */;

pub const WM8400_DC1_STBY_LIM_MASK: c_uint = 0x0300  /* DC1_STBY_LIM - [9:8] */;

pub const WM8400_DC1_ACT_LIM: c_uint = 0x0080  /* DC1_ACT_LIM */;
pub const WM8400_DC1_ACT_LIM_MASK: c_uint = 0x0080  /* DC1_ACT_LIM */;

pub const WM8400_DC1_VIMG_MASK: c_uint = 0x007F  /* DC1_VIMG - [6:0] */;

//
// R72 (0x48) - DCDC2 Control 1
//
pub const WM8400_DC2_ENA: c_uint = 0x8000  /* DC2_ENA */;
pub const WM8400_DC2_ENA_MASK: c_uint = 0x8000  /* DC2_ENA */;

pub const WM8400_DC2_ACTIVE: c_uint = 0x4000  /* DC2_ACTIVE */;
pub const WM8400_DC2_ACTIVE_MASK: c_uint = 0x4000  /* DC2_ACTIVE */;

pub const WM8400_DC2_SLEEP: c_uint = 0x2000  /* DC2_SLEEP */;
pub const WM8400_DC2_SLEEP_MASK: c_uint = 0x2000  /* DC2_SLEEP */;

pub const WM8400_DC2_OPFLT: c_uint = 0x1000  /* DC2_OPFLT */;
pub const WM8400_DC2_OPFLT_MASK: c_uint = 0x1000  /* DC2_OPFLT */;

pub const WM8400_DC2_ERRACT: c_uint = 0x0800  /* DC2_ERRACT */;
pub const WM8400_DC2_ERRACT_MASK: c_uint = 0x0800  /* DC2_ERRACT */;

pub const WM8400_DC2_HIB_MODE: c_uint = 0x0400  /* DC2_HIB_MODE */;
pub const WM8400_DC2_HIB_MODE_MASK: c_uint = 0x0400  /* DC2_HIB_MODE */;

pub const WM8400_DC2_SOFTST_MASK: c_uint = 0x0300  /* DC2_SOFTST - [9:8] */;

pub const WM8400_DC2_OV_PROT: c_uint = 0x0080  /* DC2_OV_PROT */;
pub const WM8400_DC2_OV_PROT_MASK: c_uint = 0x0080  /* DC2_OV_PROT */;

pub const WM8400_DC2_VSEL_MASK: c_uint = 0x007F  /* DC2_VSEL - [6:0] */;

//
// R73 (0x49) - DCDC2 Control 2
//
pub const WM8400_DC2_FRC_PWM: c_uint = 0x2000  /* DC2_FRC_PWM */;
pub const WM8400_DC2_FRC_PWM_MASK: c_uint = 0x2000  /* DC2_FRC_PWM */;

pub const WM8400_DC2_STBY_LIM_MASK: c_uint = 0x0300  /* DC2_STBY_LIM - [9:8] */;

pub const WM8400_DC2_ACT_LIM: c_uint = 0x0080  /* DC2_ACT_LIM */;
pub const WM8400_DC2_ACT_LIM_MASK: c_uint = 0x0080  /* DC2_ACT_LIM */;

pub const WM8400_DC2_VIMG_MASK: c_uint = 0x007F  /* DC2_VIMG - [6:0] */;

//
// R75 (0x4B) - Interface
//
pub const WM8400_AUTOINC: c_uint = 0x0008  /* AUTOINC */;
pub const WM8400_AUTOINC_MASK: c_uint = 0x0008  /* AUTOINC */;

pub const WM8400_ARA_ENA: c_uint = 0x0004  /* ARA_ENA */;
pub const WM8400_ARA_ENA_MASK: c_uint = 0x0004  /* ARA_ENA */;

pub const WM8400_SPI_CFG: c_uint = 0x0002  /* SPI_CFG */;
pub const WM8400_SPI_CFG_MASK: c_uint = 0x0002  /* SPI_CFG */;

//
// R76 (0x4C) - PM GENERAL
//
pub const WM8400_CODEC_SOFTST: c_uint = 0x8000  /* CODEC_SOFTST */;
pub const WM8400_CODEC_SOFTST_MASK: c_uint = 0x8000  /* CODEC_SOFTST */;

pub const WM8400_CODEC_SOFTSD: c_uint = 0x4000  /* CODEC_SOFTSD */;
pub const WM8400_CODEC_SOFTSD_MASK: c_uint = 0x4000  /* CODEC_SOFTSD */;

pub const WM8400_CHIP_SOFTSD: c_uint = 0x2000  /* CHIP_SOFTSD */;
pub const WM8400_CHIP_SOFTSD_MASK: c_uint = 0x2000  /* CHIP_SOFTSD */;

pub const WM8400_DSLEEP1_POL: c_uint = 0x0008  /* DSLEEP1_POL */;
pub const WM8400_DSLEEP1_POL_MASK: c_uint = 0x0008  /* DSLEEP1_POL */;

pub const WM8400_DSLEEP2_POL: c_uint = 0x0004  /* DSLEEP2_POL */;
pub const WM8400_DSLEEP2_POL_MASK: c_uint = 0x0004  /* DSLEEP2_POL */;

pub const WM8400_PWR_STATE_MASK: c_uint = 0x0003  /* PWR_STATE - [1:0] */;

//
// R78 (0x4E) - PM Shutdown Control
//
pub const WM8400_CHIP_GT150_ERRACT: c_uint = 0x0200  /* CHIP_GT150_ERRACT */;
pub const WM8400_CHIP_GT150_ERRACT_MASK: c_uint = 0x0200  /* CHIP_GT150_ERRACT */;

pub const WM8400_CHIP_GT115_ERRACT: c_uint = 0x0100  /* CHIP_GT115_ERRACT */;
pub const WM8400_CHIP_GT115_ERRACT_MASK: c_uint = 0x0100  /* CHIP_GT115_ERRACT */;

pub const WM8400_LINE_CMP_ERRACT: c_uint = 0x0080  /* LINE_CMP_ERRACT */;
pub const WM8400_LINE_CMP_ERRACT_MASK: c_uint = 0x0080  /* LINE_CMP_ERRACT */;

pub const WM8400_UVLO_ERRACT: c_uint = 0x0040  /* UVLO_ERRACT */;
pub const WM8400_UVLO_ERRACT_MASK: c_uint = 0x0040  /* UVLO_ERRACT */;

//
// R79 (0x4F) - Interrupt Status 1
//
pub const WM8400_MICD_CINT: c_uint = 0x8000  /* MICD_CINT */;
pub const WM8400_MICD_CINT_MASK: c_uint = 0x8000  /* MICD_CINT */;

pub const WM8400_MICSCD_CINT: c_uint = 0x4000  /* MICSCD_CINT */;
pub const WM8400_MICSCD_CINT_MASK: c_uint = 0x4000  /* MICSCD_CINT */;

pub const WM8400_JDL_CINT: c_uint = 0x2000  /* JDL_CINT */;
pub const WM8400_JDL_CINT_MASK: c_uint = 0x2000  /* JDL_CINT */;

pub const WM8400_JDR_CINT: c_uint = 0x1000  /* JDR_CINT */;
pub const WM8400_JDR_CINT_MASK: c_uint = 0x1000  /* JDR_CINT */;

pub const WM8400_CODEC_SEQ_END_EINT: c_uint = 0x0800  /* CODEC_SEQ_END_EINT */;
pub const WM8400_CODEC_SEQ_END_EINT_MASK: c_uint = 0x0800  /* CODEC_SEQ_END_EINT */;

pub const WM8400_CDEL_TO_EINT: c_uint = 0x0400  /* CDEL_TO_EINT */;
pub const WM8400_CDEL_TO_EINT_MASK: c_uint = 0x0400  /* CDEL_TO_EINT */;

pub const WM8400_CHIP_GT150_EINT: c_uint = 0x0200  /* CHIP_GT150_EINT */;
pub const WM8400_CHIP_GT150_EINT_MASK: c_uint = 0x0200  /* CHIP_GT150_EINT */;

pub const WM8400_CHIP_GT115_EINT: c_uint = 0x0100  /* CHIP_GT115_EINT */;
pub const WM8400_CHIP_GT115_EINT_MASK: c_uint = 0x0100  /* CHIP_GT115_EINT */;

pub const WM8400_LINE_CMP_EINT: c_uint = 0x0080  /* LINE_CMP_EINT */;
pub const WM8400_LINE_CMP_EINT_MASK: c_uint = 0x0080  /* LINE_CMP_EINT */;

pub const WM8400_UVLO_EINT: c_uint = 0x0040  /* UVLO_EINT */;
pub const WM8400_UVLO_EINT_MASK: c_uint = 0x0040  /* UVLO_EINT */;

pub const WM8400_DC2_UV_EINT: c_uint = 0x0020  /* DC2_UV_EINT */;
pub const WM8400_DC2_UV_EINT_MASK: c_uint = 0x0020  /* DC2_UV_EINT */;

pub const WM8400_DC1_UV_EINT: c_uint = 0x0010  /* DC1_UV_EINT */;
pub const WM8400_DC1_UV_EINT_MASK: c_uint = 0x0010  /* DC1_UV_EINT */;

pub const WM8400_LDO4_UV_EINT: c_uint = 0x0008  /* LDO4_UV_EINT */;
pub const WM8400_LDO4_UV_EINT_MASK: c_uint = 0x0008  /* LDO4_UV_EINT */;

pub const WM8400_LDO3_UV_EINT: c_uint = 0x0004  /* LDO3_UV_EINT */;
pub const WM8400_LDO3_UV_EINT_MASK: c_uint = 0x0004  /* LDO3_UV_EINT */;

pub const WM8400_LDO2_UV_EINT: c_uint = 0x0002  /* LDO2_UV_EINT */;
pub const WM8400_LDO2_UV_EINT_MASK: c_uint = 0x0002  /* LDO2_UV_EINT */;

pub const WM8400_LDO1_UV_EINT: c_uint = 0x0001  /* LDO1_UV_EINT */;
pub const WM8400_LDO1_UV_EINT_MASK: c_uint = 0x0001  /* LDO1_UV_EINT */;

//
// R80 (0x50) - Interrupt Status 1 Mask
//
pub const WM8400_IM_MICD_CINT: c_uint = 0x8000  /* IM_MICD_CINT */;
pub const WM8400_IM_MICD_CINT_MASK: c_uint = 0x8000  /* IM_MICD_CINT */;

pub const WM8400_IM_MICSCD_CINT: c_uint = 0x4000  /* IM_MICSCD_CINT */;
pub const WM8400_IM_MICSCD_CINT_MASK: c_uint = 0x4000  /* IM_MICSCD_CINT */;

pub const WM8400_IM_JDL_CINT: c_uint = 0x2000  /* IM_JDL_CINT */;
pub const WM8400_IM_JDL_CINT_MASK: c_uint = 0x2000  /* IM_JDL_CINT */;

pub const WM8400_IM_JDR_CINT: c_uint = 0x1000  /* IM_JDR_CINT */;
pub const WM8400_IM_JDR_CINT_MASK: c_uint = 0x1000  /* IM_JDR_CINT */;

pub const WM8400_IM_CODEC_SEQ_END_EINT: c_uint = 0x0800  /* IM_CODEC_SEQ_END_EINT */;
pub const WM8400_IM_CODEC_SEQ_END_EINT_MASK: c_uint = 0x0800  /* IM_CODEC_SEQ_END_EINT */;

pub const WM8400_IM_CDEL_TO_EINT: c_uint = 0x0400  /* IM_CDEL_TO_EINT */;
pub const WM8400_IM_CDEL_TO_EINT_MASK: c_uint = 0x0400  /* IM_CDEL_TO_EINT */;

pub const WM8400_IM_CHIP_GT150_EINT: c_uint = 0x0200  /* IM_CHIP_GT150_EINT */;
pub const WM8400_IM_CHIP_GT150_EINT_MASK: c_uint = 0x0200  /* IM_CHIP_GT150_EINT */;

pub const WM8400_IM_CHIP_GT115_EINT: c_uint = 0x0100  /* IM_CHIP_GT115_EINT */;
pub const WM8400_IM_CHIP_GT115_EINT_MASK: c_uint = 0x0100  /* IM_CHIP_GT115_EINT */;

pub const WM8400_IM_LINE_CMP_EINT: c_uint = 0x0080  /* IM_LINE_CMP_EINT */;
pub const WM8400_IM_LINE_CMP_EINT_MASK: c_uint = 0x0080  /* IM_LINE_CMP_EINT */;

pub const WM8400_IM_UVLO_EINT: c_uint = 0x0040  /* IM_UVLO_EINT */;
pub const WM8400_IM_UVLO_EINT_MASK: c_uint = 0x0040  /* IM_UVLO_EINT */;

pub const WM8400_IM_DC2_UV_EINT: c_uint = 0x0020  /* IM_DC2_UV_EINT */;
pub const WM8400_IM_DC2_UV_EINT_MASK: c_uint = 0x0020  /* IM_DC2_UV_EINT */;

pub const WM8400_IM_DC1_UV_EINT: c_uint = 0x0010  /* IM_DC1_UV_EINT */;
pub const WM8400_IM_DC1_UV_EINT_MASK: c_uint = 0x0010  /* IM_DC1_UV_EINT */;

pub const WM8400_IM_LDO4_UV_EINT: c_uint = 0x0008  /* IM_LDO4_UV_EINT */;
pub const WM8400_IM_LDO4_UV_EINT_MASK: c_uint = 0x0008  /* IM_LDO4_UV_EINT */;

pub const WM8400_IM_LDO3_UV_EINT: c_uint = 0x0004  /* IM_LDO3_UV_EINT */;
pub const WM8400_IM_LDO3_UV_EINT_MASK: c_uint = 0x0004  /* IM_LDO3_UV_EINT */;

pub const WM8400_IM_LDO2_UV_EINT: c_uint = 0x0002  /* IM_LDO2_UV_EINT */;
pub const WM8400_IM_LDO2_UV_EINT_MASK: c_uint = 0x0002  /* IM_LDO2_UV_EINT */;

pub const WM8400_IM_LDO1_UV_EINT: c_uint = 0x0001  /* IM_LDO1_UV_EINT */;
pub const WM8400_IM_LDO1_UV_EINT_MASK: c_uint = 0x0001  /* IM_LDO1_UV_EINT */;

//
// R81 (0x51) - Interrupt Levels
//
pub const WM8400_MICD_LVL: c_uint = 0x8000  /* MICD_LVL */;
pub const WM8400_MICD_LVL_MASK: c_uint = 0x8000  /* MICD_LVL */;

pub const WM8400_MICSCD_LVL: c_uint = 0x4000  /* MICSCD_LVL */;
pub const WM8400_MICSCD_LVL_MASK: c_uint = 0x4000  /* MICSCD_LVL */;

pub const WM8400_JDL_LVL: c_uint = 0x2000  /* JDL_LVL */;
pub const WM8400_JDL_LVL_MASK: c_uint = 0x2000  /* JDL_LVL */;

pub const WM8400_JDR_LVL: c_uint = 0x1000  /* JDR_LVL */;
pub const WM8400_JDR_LVL_MASK: c_uint = 0x1000  /* JDR_LVL */;

pub const WM8400_CODEC_SEQ_END_LVL: c_uint = 0x0800  /* CODEC_SEQ_END_LVL */;
pub const WM8400_CODEC_SEQ_END_LVL_MASK: c_uint = 0x0800  /* CODEC_SEQ_END_LVL */;

pub const WM8400_CDEL_TO_LVL: c_uint = 0x0400  /* CDEL_TO_LVL */;
pub const WM8400_CDEL_TO_LVL_MASK: c_uint = 0x0400  /* CDEL_TO_LVL */;

pub const WM8400_CHIP_GT150_LVL: c_uint = 0x0200  /* CHIP_GT150_LVL */;
pub const WM8400_CHIP_GT150_LVL_MASK: c_uint = 0x0200  /* CHIP_GT150_LVL */;

pub const WM8400_CHIP_GT115_LVL: c_uint = 0x0100  /* CHIP_GT115_LVL */;
pub const WM8400_CHIP_GT115_LVL_MASK: c_uint = 0x0100  /* CHIP_GT115_LVL */;

pub const WM8400_LINE_CMP_LVL: c_uint = 0x0080  /* LINE_CMP_LVL */;
pub const WM8400_LINE_CMP_LVL_MASK: c_uint = 0x0080  /* LINE_CMP_LVL */;

pub const WM8400_UVLO_LVL: c_uint = 0x0040  /* UVLO_LVL */;
pub const WM8400_UVLO_LVL_MASK: c_uint = 0x0040  /* UVLO_LVL */;

pub const WM8400_DC2_UV_LVL: c_uint = 0x0020  /* DC2_UV_LVL */;
pub const WM8400_DC2_UV_LVL_MASK: c_uint = 0x0020  /* DC2_UV_LVL */;

pub const WM8400_DC1_UV_LVL: c_uint = 0x0010  /* DC1_UV_LVL */;
pub const WM8400_DC1_UV_LVL_MASK: c_uint = 0x0010  /* DC1_UV_LVL */;

pub const WM8400_LDO4_UV_LVL: c_uint = 0x0008  /* LDO4_UV_LVL */;
pub const WM8400_LDO4_UV_LVL_MASK: c_uint = 0x0008  /* LDO4_UV_LVL */;

pub const WM8400_LDO3_UV_LVL: c_uint = 0x0004  /* LDO3_UV_LVL */;
pub const WM8400_LDO3_UV_LVL_MASK: c_uint = 0x0004  /* LDO3_UV_LVL */;

pub const WM8400_LDO2_UV_LVL: c_uint = 0x0002  /* LDO2_UV_LVL */;
pub const WM8400_LDO2_UV_LVL_MASK: c_uint = 0x0002  /* LDO2_UV_LVL */;

pub const WM8400_LDO1_UV_LVL: c_uint = 0x0001  /* LDO1_UV_LVL */;
pub const WM8400_LDO1_UV_LVL_MASK: c_uint = 0x0001  /* LDO1_UV_LVL */;

//
// R82 (0x52) - Shutdown Reason
//
pub const WM8400_SDR_CHIP_SOFTSD: c_uint = 0x2000  /* SDR_CHIP_SOFTSD */;
pub const WM8400_SDR_CHIP_SOFTSD_MASK: c_uint = 0x2000  /* SDR_CHIP_SOFTSD */;

pub const WM8400_SDR_NPDN: c_uint = 0x0800  /* SDR_NPDN */;
pub const WM8400_SDR_NPDN_MASK: c_uint = 0x0800  /* SDR_NPDN */;

pub const WM8400_SDR_CHIP_GT150: c_uint = 0x0200  /* SDR_CHIP_GT150 */;
pub const WM8400_SDR_CHIP_GT150_MASK: c_uint = 0x0200  /* SDR_CHIP_GT150 */;

pub const WM8400_SDR_CHIP_GT115: c_uint = 0x0100  /* SDR_CHIP_GT115 */;
pub const WM8400_SDR_CHIP_GT115_MASK: c_uint = 0x0100  /* SDR_CHIP_GT115 */;

pub const WM8400_SDR_LINE_CMP: c_uint = 0x0080  /* SDR_LINE_CMP */;
pub const WM8400_SDR_LINE_CMP_MASK: c_uint = 0x0080  /* SDR_LINE_CMP */;

pub const WM8400_SDR_UVLO: c_uint = 0x0040  /* SDR_UVLO */;
pub const WM8400_SDR_UVLO_MASK: c_uint = 0x0040  /* SDR_UVLO */;

pub const WM8400_SDR_DC2_UV: c_uint = 0x0020  /* SDR_DC2_UV */;
pub const WM8400_SDR_DC2_UV_MASK: c_uint = 0x0020  /* SDR_DC2_UV */;

pub const WM8400_SDR_DC1_UV: c_uint = 0x0010  /* SDR_DC1_UV */;
pub const WM8400_SDR_DC1_UV_MASK: c_uint = 0x0010  /* SDR_DC1_UV */;

pub const WM8400_SDR_LDO4_UV: c_uint = 0x0008  /* SDR_LDO4_UV */;
pub const WM8400_SDR_LDO4_UV_MASK: c_uint = 0x0008  /* SDR_LDO4_UV */;

pub const WM8400_SDR_LDO3_UV: c_uint = 0x0004  /* SDR_LDO3_UV */;
pub const WM8400_SDR_LDO3_UV_MASK: c_uint = 0x0004  /* SDR_LDO3_UV */;

pub const WM8400_SDR_LDO2_UV: c_uint = 0x0002  /* SDR_LDO2_UV */;
pub const WM8400_SDR_LDO2_UV_MASK: c_uint = 0x0002  /* SDR_LDO2_UV */;

pub const WM8400_SDR_LDO1_UV: c_uint = 0x0001  /* SDR_LDO1_UV */;
pub const WM8400_SDR_LDO1_UV_MASK: c_uint = 0x0001  /* SDR_LDO1_UV */;

//
// R84 (0x54) - Line Circuits
//
pub const WM8400_BG_LINE_COMP: c_uint = 0x8000  /* BG_LINE_COMP */;
pub const WM8400_BG_LINE_COMP_MASK: c_uint = 0x8000  /* BG_LINE_COMP */;

pub const WM8400_LINE_CMP_VTHI_MASK: c_uint = 0x00F0  /* LINE_CMP_VTHI - [7:4] */;

pub const WM8400_LINE_CMP_VTHD_MASK: c_uint = 0x000F  /* LINE_CMP_VTHD - [3:0] */;

