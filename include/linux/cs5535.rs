//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/cs5535.h
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
// AMD CS5535/CS5536 definitions
// Copyright (C) 2006  Advanced Micro Devices, Inc.
// Copyright (C) 2009  Andres Salomon <dilinger@collabora.co.uk>
//

// MSRs
pub const MSR_GLIU_P2D_RO0: c_uint = 0x10000029;
pub const MSR_LX_GLD_MSR_CONFIG: c_uint = 0x48002001;
pub const MSR_LX_MSR_PADSEL: c_uint = 0x48002011	/* NOT 0x48000011; the data;
// sheet has the wrong value
pub const MSR_GLCP_SYS_RSTPLL: c_uint = 0x4C000014;
pub const MSR_GLCP_DOTPLL: c_uint = 0x4C000015;
pub const MSR_LBAR_SMB: c_uint = 0x5140000B;
pub const MSR_LBAR_GPIO: c_uint = 0x5140000C;
pub const MSR_LBAR_MFGPT: c_uint = 0x5140000D;
pub const MSR_LBAR_ACPI: c_uint = 0x5140000E;
pub const MSR_LBAR_PMS: c_uint = 0x5140000F;
pub const MSR_DIVIL_SOFT_RESET: c_uint = 0x51400017;
pub const MSR_PIC_YSEL_LOW: c_uint = 0x51400020;
pub const MSR_PIC_YSEL_HIGH: c_uint = 0x51400021;
pub const MSR_PIC_ZSEL_LOW: c_uint = 0x51400022;
pub const MSR_PIC_ZSEL_HIGH: c_uint = 0x51400023;
pub const MSR_PIC_IRQM_LPC: c_uint = 0x51400025;
pub const MSR_MFGPT_IRQ: c_uint = 0x51400028;
pub const MSR_MFGPT_NR: c_uint = 0x51400029;
pub const MSR_MFGPT_SETUP: c_uint = 0x5140002B;
pub const MSR_RTC_DOMA_OFFSET: c_uint = 0x51400055;
pub const MSR_RTC_MONA_OFFSET: c_uint = 0x51400056;
pub const MSR_RTC_CEN_OFFSET: c_uint = 0x51400057;
pub const MSR_LX_SPARE_MSR: c_uint = 0x80000011	/* DC-specific */;
pub const MSR_GX_GLD_MSR_CONFIG: c_uint = 0xC0002001;
pub const MSR_GX_MSR_PADSEL: c_uint = 0xC0002011;
// PIC registers
pub const CS5536_PIC_INT_SEL1: c_uint = 0x4d0;
pub const CS5536_PIC_INT_SEL2: c_uint = 0x4d1;
// resource sizes
pub const LBAR_GPIO_SIZE: c_uint = 0xFF;
pub const LBAR_MFGPT_SIZE: c_uint = 0x40;
pub const LBAR_ACPI_SIZE: c_uint = 0x40;
pub const LBAR_PMS_SIZE: c_uint = 0x80;
//
// PMC registers (PMS block)
// It is only safe to access these registers as dword accesses.
// See CS5536 Specification Update erratas 17 & 18
//
pub const CS5536_PM_SCLK: c_uint = 0x10;
pub const CS5536_PM_IN_SLPCTL: c_uint = 0x20;
pub const CS5536_PM_WKXD: c_uint = 0x34;
pub const CS5536_PM_WKD: c_uint = 0x30;
pub const CS5536_PM_SSC: c_uint = 0x54;
//
// PM registers (ACPI block)
// It is only safe to access these registers as dword accesses.
// See CS5536 Specification Update erratas 17 & 18
//
pub const CS5536_PM1_STS: c_uint = 0x00;
pub const CS5536_PM1_EN: c_uint = 0x02;
pub const CS5536_PM1_CNT: c_uint = 0x08;
pub const CS5536_PM_GPE0_STS: c_uint = 0x18;
pub const CS5536_PM_GPE0_EN: c_uint = 0x1c;
// CS5536_PM1_STS bits

// CS5536_PM1_EN bits

// CS5536_PM_GPE0_STS bits

// CS5536_PM_GPE0_EN bits

// VSA2 magic values
pub const VSA_VRC_INDEX: c_uint = 0xAC1C;
pub const VSA_VRC_DATA: c_uint = 0xAC1E;
pub const VSA_VR_UNLOCK: c_uint = 0xFC53  /* unlock virtual register */;
pub const VSA_VR_SIGNATURE: c_uint = 0x0003;
pub const VSA_VR_MEM_SIZE: c_uint = 0x0200;
pub const AMD_VSA_SIG: c_uint = 0x4132  /* signature is ascii 'VSA2' */;
pub const GSW_VSA_SIG: c_uint = 0x534d  /* General Software signature */;

//
// The VSA has virtual registers that we can query for a
// signature.
//
// GPIOs
pub const GPIO_OUTPUT_VAL: c_uint = 0x00;
pub const GPIO_OUTPUT_ENABLE: c_uint = 0x04;
pub const GPIO_OUTPUT_OPEN_DRAIN: c_uint = 0x08;
pub const GPIO_OUTPUT_INVERT: c_uint = 0x0C;
pub const GPIO_OUTPUT_AUX1: c_uint = 0x10;
pub const GPIO_OUTPUT_AUX2: c_uint = 0x14;
pub const GPIO_PULL_UP: c_uint = 0x18;
pub const GPIO_PULL_DOWN: c_uint = 0x1C;
pub const GPIO_INPUT_ENABLE: c_uint = 0x20;
pub const GPIO_INPUT_INVERT: c_uint = 0x24;
pub const GPIO_INPUT_FILTER: c_uint = 0x28;
pub const GPIO_INPUT_EVENT_COUNT: c_uint = 0x2C;
pub const GPIO_READ_BACK: c_uint = 0x30;
pub const GPIO_INPUT_AUX1: c_uint = 0x34;
pub const GPIO_EVENTS_ENABLE: c_uint = 0x38;
pub const GPIO_LOCK_ENABLE: c_uint = 0x3C;
pub const GPIO_POSITIVE_EDGE_EN: c_uint = 0x40;
pub const GPIO_NEGATIVE_EDGE_EN: c_uint = 0x44;
pub const GPIO_POSITIVE_EDGE_STS: c_uint = 0x48;
pub const GPIO_NEGATIVE_EDGE_STS: c_uint = 0x4C;
pub const GPIO_FLTR7_AMOUNT: c_uint = 0xD8;
pub const GPIO_MAP_X: c_uint = 0xE0;
pub const GPIO_MAP_Y: c_uint = 0xE4;
pub const GPIO_MAP_Z: c_uint = 0xE8;
pub const GPIO_MAP_W: c_uint = 0xEC;
pub const GPIO_FE7_SEL: c_uint = 0xF7;
extern "C" {
    pub fn cs5535_gpio_set(offset: unsigned, reg: c_uint);
}
extern "C" {
    pub fn cs5535_gpio_clear(offset: unsigned, reg: c_uint);
}
extern "C" {
    pub fn cs5535_gpio_isset(offset: unsigned, reg: c_uint) -> c_int;
}
extern "C" {
    pub fn cs5535_gpio_set_irq(group: unsigned, irq: unsigned) -> c_int;
}
extern "C" {
    pub fn cs5535_gpio_setup_event(offset: unsigned, pair: c_int, pme: c_int);
}
// MFGPTs
pub const MFGPT_MAX_TIMERS: c_int = 8;

pub const MFGPT_DOMAIN_WORKING: c_int = 1;
pub const MFGPT_DOMAIN_STANDBY: c_int = 2;

pub const MFGPT_CMP1: c_int = 0;
pub const MFGPT_CMP2: c_int = 1;
pub const MFGPT_EVENT_IRQ: c_int = 0;
pub const MFGPT_EVENT_NMI: c_int = 1;
pub const MFGPT_EVENT_RESET: c_int = 3;
pub const MFGPT_REG_CMP1: c_int = 0;
pub const MFGPT_REG_CMP2: c_int = 2;
pub const MFGPT_REG_COUNTER: c_int = 4;
pub const MFGPT_REG_SETUP: c_int = 6;

extern "C" {
    pub fn cs5535_mfgpt_free_timer(timer: *mut cs5535_mfgpt_timer);
}
extern "C" {
    pub fn cs5535_mfgpt_set_irq(_arg: timer, _arg: cmp, _arg: irq, _arg: 1) -> return;
}
extern "C" {
    pub fn cs5535_mfgpt_set_irq(_arg: timer, _arg: cmp, _arg: irq, _arg: 0) -> return;
}
