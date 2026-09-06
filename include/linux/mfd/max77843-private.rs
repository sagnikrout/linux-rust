//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/max77843-private.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Common variables for the Maxim MAX77843 driver
//
// Copyright (C) 2015 Samsung Electronics
// Author: Jaewon Kim <jaewon02.kim@samsung.com>
// Author: Beomho Seo <beomho.seo@samsung.com>
//

// Topsys, Haptic and LED registers
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum max77843_sys_reg {
    MAX77843_SYS_REG_PMICID		= 0x00,
    MAX77843_SYS_REG_PMICREV	= 0x01,
    MAX77843_SYS_REG_MAINCTRL1	= 0x02,
    MAX77843_SYS_REG_INTSRC		= 0x22,
    MAX77843_SYS_REG_INTSRCMASK	= 0x23,
    MAX77843_SYS_REG_SYSINTSRC	= 0x24,
    MAX77843_SYS_REG_SYSINTMASK	= 0x26,
    MAX77843_SYS_REG_TOPSYS_STAT	= 0x28,
    MAX77843_SYS_REG_SAFEOUTCTRL	= 0xC6,

    MAX77843_SYS_REG_END,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum max77843_haptic_reg {
    MAX77843_HAP_REG_MCONFIG	= 0x10,

    MAX77843_HAP_REG_END,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum max77843_led_reg {
    MAX77843_LED_REG_LEDEN		= 0x30,
    MAX77843_LED_REG_LED0BRT	= 0x31,
    MAX77843_LED_REG_LED1BRT	= 0x32,
    MAX77843_LED_REG_LED2BRT	= 0x33,
    MAX77843_LED_REG_LED3BRT	= 0x34,
    MAX77843_LED_REG_LEDBLNK	= 0x38,
    MAX77843_LED_REG_LEDRAMP	= 0x36,

    MAX77843_LED_REG_END,
}

// Charger registers
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum max77843_charger_reg {
    MAX77843_CHG_REG_CHG_INT	= 0xB0,
    MAX77843_CHG_REG_CHG_INT_MASK	= 0xB1,
    MAX77843_CHG_REG_CHG_INT_OK	= 0xB2,
    MAX77843_CHG_REG_CHG_DTLS_00	= 0xB3,
    MAX77843_CHG_REG_CHG_DTLS_01	= 0xB4,
    MAX77843_CHG_REG_CHG_DTLS_02	= 0xB5,
    MAX77843_CHG_REG_CHG_CNFG_00	= 0xB7,
    MAX77843_CHG_REG_CHG_CNFG_01	= 0xB8,
    MAX77843_CHG_REG_CHG_CNFG_02	= 0xB9,
    MAX77843_CHG_REG_CHG_CNFG_03	= 0xBA,
    MAX77843_CHG_REG_CHG_CNFG_04	= 0xBB,
    MAX77843_CHG_REG_CHG_CNFG_06	= 0xBD,
    MAX77843_CHG_REG_CHG_CNFG_07	= 0xBE,
    MAX77843_CHG_REG_CHG_CNFG_09	= 0xC0,
    MAX77843_CHG_REG_CHG_CNFG_10	= 0xC1,
    MAX77843_CHG_REG_CHG_CNFG_11	= 0xC2,
    MAX77843_CHG_REG_CHG_CNFG_12	= 0xC3,

    MAX77843_CHG_REG_END,
}

// Fuel gauge registers
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum max77843_fuelgauge {
    MAX77843_FG_REG_STATUS		= 0x00,
    MAX77843_FG_REG_VALRT_TH	= 0x01,
    MAX77843_FG_REG_TALRT_TH	= 0x02,
    MAX77843_FG_REG_SALRT_TH	= 0x03,
    MAX77843_FG_RATE_AT_RATE	= 0x04,
    MAX77843_FG_REG_REMCAP_REP	= 0x05,
    MAX77843_FG_REG_SOCREP		= 0x06,
    MAX77843_FG_REG_AGE		= 0x07,
    MAX77843_FG_REG_TEMP		= 0x08,
    MAX77843_FG_REG_VCELL		= 0x09,
    MAX77843_FG_REG_CURRENT		= 0x0A,
    MAX77843_FG_REG_AVG_CURRENT	= 0x0B,
    MAX77843_FG_REG_SOCMIX		= 0x0D,
    MAX77843_FG_REG_SOCAV		= 0x0E,
    MAX77843_FG_REG_REMCAP_MIX	= 0x0F,
    MAX77843_FG_REG_FULLCAP		= 0x10,
    MAX77843_FG_REG_AVG_TEMP	= 0x16,
    MAX77843_FG_REG_CYCLES		= 0x17,
    MAX77843_FG_REG_AVG_VCELL	= 0x19,
    MAX77843_FG_REG_CONFIG		= 0x1D,
    MAX77843_FG_REG_REMCAP_AV	= 0x1F,
    MAX77843_FG_REG_FULLCAP_NOM	= 0x23,
    MAX77843_FG_REG_MISCCFG		= 0x2B,
    MAX77843_FG_REG_RCOMP		= 0x38,
    MAX77843_FG_REG_FSTAT		= 0x3D,
    MAX77843_FG_REG_DQACC		= 0x45,
    MAX77843_FG_REG_DPACC		= 0x46,
    MAX77843_FG_REG_OCV		= 0xEE,
    MAX77843_FG_REG_VFOCV		= 0xFB,
    MAX77843_FG_SOCVF		= 0xFF,

    MAX77843_FG_END,
}

// MUIC registers
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum max77843_muic_reg {
    MAX77843_MUIC_REG_ID		= 0x00,
    MAX77843_MUIC_REG_INT1		= 0x01,
    MAX77843_MUIC_REG_INT2		= 0x02,
    MAX77843_MUIC_REG_INT3		= 0x03,
    MAX77843_MUIC_REG_STATUS1	= 0x04,
    MAX77843_MUIC_REG_STATUS2	= 0x05,
    MAX77843_MUIC_REG_STATUS3	= 0x06,
    MAX77843_MUIC_REG_INTMASK1	= 0x07,
    MAX77843_MUIC_REG_INTMASK2	= 0x08,
    MAX77843_MUIC_REG_INTMASK3	= 0x09,
    MAX77843_MUIC_REG_CDETCTRL1	= 0x0A,
    MAX77843_MUIC_REG_CDETCTRL2	= 0x0B,
    MAX77843_MUIC_REG_CONTROL1	= 0x0C,
    MAX77843_MUIC_REG_CONTROL2	= 0x0D,
    MAX77843_MUIC_REG_CONTROL3	= 0x0E,
    MAX77843_MUIC_REG_CONTROL4	= 0x16,
    MAX77843_MUIC_REG_HVCONTROL1	= 0x17,
    MAX77843_MUIC_REG_HVCONTROL2	= 0x18,

    MAX77843_MUIC_REG_END,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum max77843_irq {
// Topsys: SYSTEM
    MAX77843_SYS_IRQ_SYSINTSRC_SYSUVLO_INT,
    MAX77843_SYS_IRQ_SYSINTSRC_SYSOVLO_INT,
    MAX77843_SYS_IRQ_SYSINTSRC_TSHDN_INT,
    MAX77843_SYS_IRQ_SYSINTSRC_TM_INT,

// Charger: CHG_INT
    MAX77843_CHG_IRQ_CHG_INT_BYP_I,
    MAX77843_CHG_IRQ_CHG_INT_BATP_I,
    MAX77843_CHG_IRQ_CHG_INT_BAT_I,
    MAX77843_CHG_IRQ_CHG_INT_CHG_I,
    MAX77843_CHG_IRQ_CHG_INT_WCIN_I,
    MAX77843_CHG_IRQ_CHG_INT_CHGIN_I,
    MAX77843_CHG_IRQ_CHG_INT_AICL_I,

    MAX77843_IRQ_NUM,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum max77843_irq_muic {
// MUIC: INT1
    MAX77843_MUIC_IRQ_INT1_ADC,
    MAX77843_MUIC_IRQ_INT1_ADCERROR,
    MAX77843_MUIC_IRQ_INT1_ADC1K,

// MUIC: INT2
    MAX77843_MUIC_IRQ_INT2_CHGTYP,
    MAX77843_MUIC_IRQ_INT2_CHGDETRUN,
    MAX77843_MUIC_IRQ_INT2_DCDTMR,
    MAX77843_MUIC_IRQ_INT2_DXOVP,
    MAX77843_MUIC_IRQ_INT2_VBVOLT,

// MUIC: INT3
    MAX77843_MUIC_IRQ_INT3_VBADC,
    MAX77843_MUIC_IRQ_INT3_VDNMON,
    MAX77843_MUIC_IRQ_INT3_DNRES,
    MAX77843_MUIC_IRQ_INT3_MPNACK,
    MAX77843_MUIC_IRQ_INT3_MRXBUFOW,
    MAX77843_MUIC_IRQ_INT3_MRXTRF,
    MAX77843_MUIC_IRQ_INT3_MRXPERR,
    MAX77843_MUIC_IRQ_INT3_MRXRDY,

    MAX77843_MUIC_IRQ_NUM,
}

// MAX77843 interrupts

// MAX77843 MAINCTRL1 register
pub const MAINCTRL1_BIASEN_SHIFT: c_int = 7;

// MAX77843 MCONFIG register
pub const MCONFIG_MODE_SHIFT: c_int = 7;
pub const MCONFIG_MEN_SHIFT: c_int = 6;
pub const MCONFIG_PDIV_SHIFT: c_int = 0;

// Max77843 charger interrupts

// MAX77843 CHG_INT_OK register

// MAX77843 CHG_DETAILS_00 register

// MAX77843 CHG_DETAILS_01 register
pub const MAX77843_CHG_DTLS_MASK: c_uint = 0x0f;
pub const MAX77843_CHG_PQ_MODE: c_uint = 0x00;
pub const MAX77843_CHG_CC_MODE: c_uint = 0x01;
pub const MAX77843_CHG_CV_MODE: c_uint = 0x02;
pub const MAX77843_CHG_TO_MODE: c_uint = 0x03;
pub const MAX77843_CHG_DO_MODE: c_uint = 0x04;
pub const MAX77843_CHG_HT_MODE: c_uint = 0x05;
pub const MAX77843_CHG_TF_MODE: c_uint = 0x06;
pub const MAX77843_CHG_TS_MODE: c_uint = 0x07;
pub const MAX77843_CHG_OFF_MODE: c_uint = 0x08;
pub const MAX77843_CHG_BAT_DTLS_MASK: c_uint = 0xf0;

// MAX77843 CHG_CNFG_00 register
pub const MAX77843_CHG_MODE_MASK: c_uint = 0x0f;
pub const MAX77843_CHG_DISABLE: c_uint = 0x00;
pub const MAX77843_CHG_ENABLE: c_uint = 0x05;
pub const MAX77843_CHG_MASK: c_uint = 0x01;
pub const MAX77843_CHG_OTG_MASK: c_uint = 0x02;
pub const MAX77843_CHG_BUCK_MASK: c_uint = 0x04;
pub const MAX77843_CHG_BOOST_MASK: c_uint = 0x08;
// MAX77843 CHG_CNFG_01 register
pub const MAX77843_CHG_RESTART_THRESHOLD_100: c_uint = 0x00;
pub const MAX77843_CHG_RESTART_THRESHOLD_150: c_uint = 0x10;
pub const MAX77843_CHG_RESTART_THRESHOLD_200: c_uint = 0x20;
pub const MAX77843_CHG_RESTART_THRESHOLD_DISABLE: c_uint = 0x30;
// MAX77843 CHG_CNFG_02 register
pub const MAX77843_CHG_FAST_CHG_CURRENT_MIN: c_int = 100000;
pub const MAX77843_CHG_FAST_CHG_CURRENT_MAX: c_int = 3150000;
pub const MAX77843_CHG_FAST_CHG_CURRENT_STEP: c_int = 50000;
pub const MAX77843_CHG_FAST_CHG_CURRENT_MASK: c_uint = 0x3f;

pub const MAX77843_CHG_OTG_ILIMIT_MASK: c_uint = 0xc0;
// MAX77843 CHG_CNFG_03 register
pub const MAX77843_CHG_TOP_OFF_CURRENT_MIN: c_int = 125000;
pub const MAX77843_CHG_TOP_OFF_CURRENT_MAX: c_int = 650000;
pub const MAX77843_CHG_TOP_OFF_CURRENT_STEP: c_int = 75000;
pub const MAX77843_CHG_TOP_OFF_CURRENT_MASK: c_uint = 0x07;
// MAX77843 CHG_CNFG_06 register
pub const MAX77843_CHG_WRITE_CAP_BLOCK: c_uint = 0x10;
pub const MAX77843_CHG_WRITE_CAP_UNBLOCK: c_uint = 0x0C;
// MAX77843_CHG_CNFG_09_register
pub const MAX77843_CHG_INPUT_CURRENT_LIMIT_MIN: c_int = 100000;
pub const MAX77843_CHG_INPUT_CURRENT_LIMIT_MAX: c_int = 4000000;
pub const MAX77843_CHG_INPUT_CURRENT_LIMIT_REF: c_int = 3367000;
pub const MAX77843_CHG_INPUT_CURRENT_LIMIT_STEP: c_int = 33000;

// MAX77843 INTSRCMASK register
pub const MAX77843_INTSRCMASK_CHGR: c_int = 0;
pub const MAX77843_INTSRCMASK_SYS: c_int = 1;
pub const MAX77843_INTSRCMASK_FG: c_int = 2;
pub const MAX77843_INTSRCMASK_MUIC: c_int = 3;

// MAX77843 STATUS register
pub const MAX77843_MUIC_STATUS1_ADC_SHIFT: c_int = 0;
pub const MAX77843_MUIC_STATUS1_ADCERROR_SHIFT: c_int = 6;
pub const MAX77843_MUIC_STATUS1_ADC1K_SHIFT: c_int = 7;
pub const MAX77843_MUIC_STATUS2_CHGTYP_SHIFT: c_int = 0;
pub const MAX77843_MUIC_STATUS2_CHGDETRUN_SHIFT: c_int = 3;
pub const MAX77843_MUIC_STATUS2_DCDTMR_SHIFT: c_int = 4;
pub const MAX77843_MUIC_STATUS2_DXOVP_SHIFT: c_int = 5;
pub const MAX77843_MUIC_STATUS2_VBVOLT_SHIFT: c_int = 6;
pub const MAX77843_MUIC_STATUS3_VBADC_SHIFT: c_int = 0;
pub const MAX77843_MUIC_STATUS3_VDNMON_SHIFT: c_int = 4;
pub const MAX77843_MUIC_STATUS3_DNRES_SHIFT: c_int = 5;
pub const MAX77843_MUIC_STATUS3_MPNACK_SHIFT: c_int = 6;

// MAX77843 CONTROL register
pub const MAX77843_MUIC_CONTROL1_COMP1SW_SHIFT: c_int = 0;
pub const MAX77843_MUIC_CONTROL1_COMP2SW_SHIFT: c_int = 3;
pub const MAX77843_MUIC_CONTROL1_NOBCCOMP_SHIFT: c_int = 6;
pub const MAX77843_MUIC_CONTROL1_IDBEN_SHIFT: c_int = 7;
pub const MAX77843_MUIC_CONTROL2_LOWPWR_SHIFT: c_int = 0;
pub const MAX77843_MUIC_CONTROL2_ADCEN_SHIFT: c_int = 1;
pub const MAX77843_MUIC_CONTROL2_CPEN_SHIFT: c_int = 2;
pub const MAX77843_MUIC_CONTROL2_ACC_DET_SHIFT: c_int = 5;
pub const MAX77843_MUIC_CONTROL2_USBCPINT_SHIFT: c_int = 6;
pub const MAX77843_MUIC_CONTROL2_RCPS_SHIFT: c_int = 7;
pub const MAX77843_MUIC_CONTROL3_JIGSET_SHIFT: c_int = 0;
pub const MAX77843_MUIC_CONTROL4_ADCDBSET_SHIFT: c_int = 0;
pub const MAX77843_MUIC_CONTROL4_USBAUTO_SHIFT: c_int = 4;
pub const MAX77843_MUIC_CONTROL4_FCTAUTO_SHIFT: c_int = 5;
pub const MAX77843_MUIC_CONTROL4_ADCMODE_SHIFT: c_int = 6;

// MAX77843 switch port
pub const COM_OPEN: c_int = 0;
pub const COM_USB: c_int = 1;
pub const COM_AUDIO: c_int = 2;
pub const COM_UART: c_int = 3;
pub const COM_AUX_USB: c_int = 4;
pub const COM_AUX_UART: c_int = 5;

pub const MAX77843_DISABLE: c_int = 0;
pub const MAX77843_ENABLE: c_int = 1;

// MAX77843 SAFEOUT LDO Control register
pub const SAFEOUTCTRL_SAFEOUT1_SHIFT: c_int = 0;
pub const SAFEOUTCTRL_SAFEOUT2_SHIFT: c_int = 2;
pub const SAFEOUTCTRL_ENSAFEOUT1_SHIFT: c_int = 6;
pub const SAFEOUTCTRL_ENSAFEOUT2_SHIFT: c_int = 7;

