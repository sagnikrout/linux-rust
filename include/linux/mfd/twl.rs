//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/twl.h
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
// twl4030.h - header for TWL4030 PM and audio CODEC device
//
// Copyright (C) 2005-2006 Texas Instruments, Inc.
//
// Based on tlv320aic23.c:
// Copyright (c) by Kai Svahn <kai.svahn@nokia.com>
//

//
// Using the twl4030 core we address registers using a pair
// { module id, relative register offset }
// which that core then maps to the relevant
// { i2c slave, absolute register address }
//
// The module IDs are meaningful only to the twl4030 core code,
// which uses them as array indices to look up the first register
// address each module uses within a given i2c slave.
//
// Module IDs for similar functionalities found in twl4030/twl6030
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum twl_module_ids {
    TWL_MODULE_USB,
    TWL_MODULE_PIH,
    TWL_MODULE_MAIN_CHARGE,
    TWL_MODULE_PM_MASTER,
    TWL_MODULE_PM_RECEIVER,

    TWL_MODULE_RTC,
    TWL_MODULE_PWM,
    TWL_MODULE_LED,
    TWL_MODULE_SECURED_REG,

    TWL_MODULE_LAST,
}

// Modules only available in twl4030 series
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum twl4030_module_ids {
    TWL4030_MODULE_AUDIO_VOICE = TWL_MODULE_LAST,
    TWL4030_MODULE_GPIO,
    TWL4030_MODULE_INTBR,
    TWL4030_MODULE_TEST,
    TWL4030_MODULE_KEYPAD,

    TWL4030_MODULE_MADC,
    TWL4030_MODULE_INTERRUPTS,
    TWL4030_MODULE_PRECHARGE,
    TWL4030_MODULE_BACKUP,
    TWL4030_MODULE_INT,

    TWL5031_MODULE_ACCESSORY,
    TWL5031_MODULE_INTERRUPTS,

    TWL4030_MODULE_LAST,
}

// Modules only available in twl6030 series
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum twl6030_module_ids {
    TWL6030_MODULE_ID0 = TWL_MODULE_LAST,
    TWL6030_MODULE_ID1,
    TWL6030_MODULE_ID2,
    TWL6030_MODULE_GPADC,
    TWL6030_MODULE_GASGAUGE,

// A few extra registers before the registers shared with the 6030
    TWL6032_MODULE_CHARGE,
    TWL6030_MODULE_LAST,
}

// Until the clients has been converted to use TWL_MODULE_LED

pub const GPIO_INTR_OFFSET: c_int = 0;
pub const KEYPAD_INTR_OFFSET: c_int = 1;
pub const BCI_INTR_OFFSET: c_int = 2;
pub const MADC_INTR_OFFSET: c_int = 3;
pub const USB_INTR_OFFSET: c_int = 4;
pub const CHARGERFAULT_INTR_OFFSET: c_int = 5;
pub const BCI_PRES_INTR_OFFSET: c_int = 9;
pub const USB_PRES_INTR_OFFSET: c_int = 10;
pub const RTC_INTR_OFFSET: c_int = 11;
//
// Offset from TWL6030_IRQ_BASE / pdata->irq_base
//
pub const PWR_INTR_OFFSET: c_int = 0;
pub const HOTDIE_INTR_OFFSET: c_int = 12;
pub const SMPSLDO_INTR_OFFSET: c_int = 13;
pub const BATDETECT_INTR_OFFSET: c_int = 14;
pub const SIMDETECT_INTR_OFFSET: c_int = 15;
pub const MMCDETECT_INTR_OFFSET: c_int = 16;
pub const GASGAUGE_INTR_OFFSET: c_int = 17;
pub const USBOTG_INTR_OFFSET: c_int = 4;
pub const CHARGER_INTR_OFFSET: c_int = 2;
pub const RSV_INTR_OFFSET: c_int = 0;
// INT register offsets
pub const REG_INT_STS_A: c_uint = 0x00;
pub const REG_INT_STS_B: c_uint = 0x01;
pub const REG_INT_STS_C: c_uint = 0x02;
pub const REG_INT_MSK_LINE_A: c_uint = 0x03;
pub const REG_INT_MSK_LINE_B: c_uint = 0x04;
pub const REG_INT_MSK_LINE_C: c_uint = 0x05;
pub const REG_INT_MSK_STS_A: c_uint = 0x06;
pub const REG_INT_MSK_STS_B: c_uint = 0x07;
pub const REG_INT_MSK_STS_C: c_uint = 0x08;
// MASK INT REG GROUP A
pub const TWL6030_PWR_INT_MASK: c_uint = 0x07;
pub const TWL6030_RTC_INT_MASK: c_uint = 0x18;
pub const TWL6030_HOTDIE_INT_MASK: c_uint = 0x20;
pub const TWL6030_SMPSLDOA_INT_MASK: c_uint = 0xC0;
// MASK INT REG GROUP B
pub const TWL6030_SMPSLDOB_INT_MASK: c_uint = 0x01;
pub const TWL6030_BATDETECT_INT_MASK: c_uint = 0x02;
pub const TWL6030_SIMDETECT_INT_MASK: c_uint = 0x04;
pub const TWL6030_MMCDETECT_INT_MASK: c_uint = 0x08;
pub const TWL6030_GPADC_INT_MASK: c_uint = 0x60;
pub const TWL6030_GASGAUGE_INT_MASK: c_uint = 0x80;
// MASK INT REG GROUP C
pub const TWL6030_USBOTG_INT_MASK: c_uint = 0x0F;
pub const TWL6030_CHARGER_CTRL_INT_MASK: c_uint = 0x10;
pub const TWL6030_CHARGER_FAULT_INT_MASK: c_uint = 0x60;
pub const TWL6030_MMCCTRL: c_uint = 0xEE;

pub const STS_MMC: c_uint = 0x1;
pub const TWL6030_CFG_INPUT_PUPD3: c_uint = 0xF2;

pub const TWL_SIL_5030: c_uint = 0x09002F;
pub const TWL5030_REV_1_0: c_uint = 0x00;
pub const TWL5030_REV_1_1: c_uint = 0x10;
pub const TWL5030_REV_1_2: c_uint = 0x30;
pub const TWL4030_CLASS_ID: c_uint = 0x4030;
pub const TWL6030_CLASS_ID: c_uint = 0x6030;
extern "C" {
    pub fn twl_rev() -> c_uint;
}

// Set the regcache bypass for the regmap associated with the nodule
extern "C" {
    pub fn twl_set_regcache_bypass(mod_no: u8, enable: bool) -> c_int;
}
//
// Read and write several 8-bit registers at once.
//
extern "C" {
    pub fn twl_i2c_write(mod_no: u8, value: *mut u8, reg: u8, num_bytes: unsigned) -> c_int;
}
extern "C" {
    pub fn twl_i2c_read(mod_no: u8, value: *mut u8, reg: u8, num_bytes: unsigned) -> c_int;
}
//
// Read and write single 8-bit registers
//
extern "C" {
    pub fn twl_i2c_write(_arg: mod_no, _arg: &val, _arg: reg, _arg: 1) -> return;
}
extern "C" {
    pub fn twl_i2c_read(_arg: mod_no, _arg: val, _arg: reg, _arg: 1) -> return;
}
extern "C" {
    pub fn twl_i2c_write(_arg: mod_no, &value: *mut *mut (u8 ), _arg: reg, _arg: 2) -> return;
}
// val = le16_to_cpu(value);
extern "C" {
    pub fn twl_get_type() -> c_int;
}
extern "C" {
    pub fn twl_get_version() -> c_int;
}
extern "C" {
    pub fn twl_get_hfclk_rate() -> c_int;
}
extern "C" {
    pub fn twl6030_interrupt_unmask(bit_mask: u8, offset: u8) -> c_int;
}
extern "C" {
    pub fn twl6030_interrupt_mask(bit_mask: u8, offset: u8) -> c_int;
}
// ----------------------------------------------------------------------
//
// NOTE:  at up to 1024 registers, this is a big chip.
//
// Avoid putting register declarations in this file, instead of into
// a driver-private file, unless some of the registers in a block
// need to be shared with other drivers.  One example is blocks that
// have Secondary IRQ Handler (SIH) registers.
//

// ----------------------------------------------------------------------
//
// GPIO Block Register offsets (use TWL4030_MODULE_GPIO)
//
pub const REG_GPIODATAIN1: c_uint = 0x0;
pub const REG_GPIODATAIN2: c_uint = 0x1;
pub const REG_GPIODATAIN3: c_uint = 0x2;
pub const REG_GPIODATADIR1: c_uint = 0x3;
pub const REG_GPIODATADIR2: c_uint = 0x4;
pub const REG_GPIODATADIR3: c_uint = 0x5;
pub const REG_GPIODATAOUT1: c_uint = 0x6;
pub const REG_GPIODATAOUT2: c_uint = 0x7;
pub const REG_GPIODATAOUT3: c_uint = 0x8;
pub const REG_CLEARGPIODATAOUT1: c_uint = 0x9;
pub const REG_CLEARGPIODATAOUT2: c_uint = 0xA;
pub const REG_CLEARGPIODATAOUT3: c_uint = 0xB;
pub const REG_SETGPIODATAOUT1: c_uint = 0xC;
pub const REG_SETGPIODATAOUT2: c_uint = 0xD;
pub const REG_SETGPIODATAOUT3: c_uint = 0xE;
pub const REG_GPIO_DEBEN1: c_uint = 0xF;
pub const REG_GPIO_DEBEN2: c_uint = 0x10;
pub const REG_GPIO_DEBEN3: c_uint = 0x11;
pub const REG_GPIO_CTRL: c_uint = 0x12;
pub const REG_GPIOPUPDCTR1: c_uint = 0x13;
pub const REG_GPIOPUPDCTR2: c_uint = 0x14;
pub const REG_GPIOPUPDCTR3: c_uint = 0x15;
pub const REG_GPIOPUPDCTR4: c_uint = 0x16;
pub const REG_GPIOPUPDCTR5: c_uint = 0x17;
pub const REG_GPIO_ISR1A: c_uint = 0x19;
pub const REG_GPIO_ISR2A: c_uint = 0x1A;
pub const REG_GPIO_ISR3A: c_uint = 0x1B;
pub const REG_GPIO_IMR1A: c_uint = 0x1C;
pub const REG_GPIO_IMR2A: c_uint = 0x1D;
pub const REG_GPIO_IMR3A: c_uint = 0x1E;
pub const REG_GPIO_ISR1B: c_uint = 0x1F;
pub const REG_GPIO_ISR2B: c_uint = 0x20;
pub const REG_GPIO_ISR3B: c_uint = 0x21;
pub const REG_GPIO_IMR1B: c_uint = 0x22;
pub const REG_GPIO_IMR2B: c_uint = 0x23;
pub const REG_GPIO_IMR3B: c_uint = 0x24;
pub const REG_GPIO_EDR1: c_uint = 0x28;
pub const REG_GPIO_EDR2: c_uint = 0x29;
pub const REG_GPIO_EDR3: c_uint = 0x2A;
pub const REG_GPIO_EDR4: c_uint = 0x2B;
pub const REG_GPIO_EDR5: c_uint = 0x2C;
pub const REG_GPIO_SIH_CTRL: c_uint = 0x2D;
// Up to 18 signals are available as GPIOs, when their
// pins are not assigned to another use (such as ULPI/USB).
//
pub const TWL4030_GPIO_MAX: c_int = 18;
// ----------------------------------------------------------------------
// Interface Bit Register (INTBR) offsets
// (Use TWL_4030_MODULE_INTBR)
//
pub const REG_IDCODE_7_0: c_uint = 0x00;
pub const REG_IDCODE_15_8: c_uint = 0x01;
pub const REG_IDCODE_16_23: c_uint = 0x02;
pub const REG_IDCODE_31_24: c_uint = 0x03;
pub const REG_GPPUPDCTR1: c_uint = 0x0F;
pub const REG_UNLOCK_TEST_REG: c_uint = 0x12;
// I2C1 and I2C4(SR) SDA/SCL pull-up control bits

pub const TWL_EEPROM_R_UNLOCK: c_uint = 0x49;
// ----------------------------------------------------------------------
//
// Keypad register offsets (use TWL4030_MODULE_KEYPAD)
// ... SIH/interrupt only
//
pub const TWL4030_KEYPAD_KEYP_ISR1: c_uint = 0x11;
pub const TWL4030_KEYPAD_KEYP_IMR1: c_uint = 0x12;
pub const TWL4030_KEYPAD_KEYP_ISR2: c_uint = 0x13;
pub const TWL4030_KEYPAD_KEYP_IMR2: c_uint = 0x14;
pub const TWL4030_KEYPAD_KEYP_SIR: c_uint = 0x15	/* test register */;
pub const TWL4030_KEYPAD_KEYP_EDR: c_uint = 0x16;
pub const TWL4030_KEYPAD_KEYP_SIH_CTRL: c_uint = 0x17;
// ----------------------------------------------------------------------
//
// Multichannel ADC register offsets (use TWL4030_MODULE_MADC)
// ... SIH/interrupt only
//
pub const TWL4030_MADC_ISR1: c_uint = 0x61;
pub const TWL4030_MADC_IMR1: c_uint = 0x62;
pub const TWL4030_MADC_ISR2: c_uint = 0x63;
pub const TWL4030_MADC_IMR2: c_uint = 0x64;
pub const TWL4030_MADC_SIR: c_uint = 0x65	/* test register */;
pub const TWL4030_MADC_EDR: c_uint = 0x66;
pub const TWL4030_MADC_SIH_CTRL: c_uint = 0x67;
// ----------------------------------------------------------------------
//
// Battery charger register offsets (use TWL4030_MODULE_INTERRUPTS)
//
pub const TWL4030_INTERRUPTS_BCIISR1A: c_uint = 0x0;
pub const TWL4030_INTERRUPTS_BCIISR2A: c_uint = 0x1;
pub const TWL4030_INTERRUPTS_BCIIMR1A: c_uint = 0x2;
pub const TWL4030_INTERRUPTS_BCIIMR2A: c_uint = 0x3;
pub const TWL4030_INTERRUPTS_BCIISR1B: c_uint = 0x4;
pub const TWL4030_INTERRUPTS_BCIISR2B: c_uint = 0x5;
pub const TWL4030_INTERRUPTS_BCIIMR1B: c_uint = 0x6;
pub const TWL4030_INTERRUPTS_BCIIMR2B: c_uint = 0x7;
pub const TWL4030_INTERRUPTS_BCISIR1: c_uint = 0x8	/* test register */;
pub const TWL4030_INTERRUPTS_BCISIR2: c_uint = 0x9	/* test register */;
pub const TWL4030_INTERRUPTS_BCIEDR1: c_uint = 0xa;
pub const TWL4030_INTERRUPTS_BCIEDR2: c_uint = 0xb;
pub const TWL4030_INTERRUPTS_BCIEDR3: c_uint = 0xc;
pub const TWL4030_INTERRUPTS_BCISIHCTRL: c_uint = 0xd;
// ----------------------------------------------------------------------
//
// Power Interrupt block register offsets (use TWL4030_MODULE_INT)
//
pub const TWL4030_INT_PWR_ISR1: c_uint = 0x0;
pub const TWL4030_INT_PWR_IMR1: c_uint = 0x1;
pub const TWL4030_INT_PWR_ISR2: c_uint = 0x2;
pub const TWL4030_INT_PWR_IMR2: c_uint = 0x3;
pub const TWL4030_INT_PWR_SIR: c_uint = 0x4	/* test register */;
pub const TWL4030_INT_PWR_EDR1: c_uint = 0x5;
pub const TWL4030_INT_PWR_EDR2: c_uint = 0x6;
pub const TWL4030_INT_PWR_SIH_CTRL: c_uint = 0x7;
// ----------------------------------------------------------------------
//
// Accessory Interrupts
//
pub const TWL5031_ACIIMR_LSB: c_uint = 0x05;
pub const TWL5031_ACIIMR_MSB: c_uint = 0x06;
pub const TWL5031_ACIIDR_LSB: c_uint = 0x07;
pub const TWL5031_ACIIDR_MSB: c_uint = 0x08;
pub const TWL5031_ACCISR1: c_uint = 0x0F;
pub const TWL5031_ACCIMR1: c_uint = 0x10;
pub const TWL5031_ACCISR2: c_uint = 0x11;
pub const TWL5031_ACCIMR2: c_uint = 0x12;
pub const TWL5031_ACCSIR: c_uint = 0x13;
pub const TWL5031_ACCEDR1: c_uint = 0x14;
pub const TWL5031_ACCSIHCTRL: c_uint = 0x15;
// ----------------------------------------------------------------------
//
// Battery Charger Controller
//
pub const TWL5031_INTERRUPTS_BCIISR1: c_uint = 0x0;
pub const TWL5031_INTERRUPTS_BCIIMR1: c_uint = 0x1;
pub const TWL5031_INTERRUPTS_BCIISR2: c_uint = 0x2;
pub const TWL5031_INTERRUPTS_BCIIMR2: c_uint = 0x3;
pub const TWL5031_INTERRUPTS_BCISIR: c_uint = 0x4;
pub const TWL5031_INTERRUPTS_BCIEDR1: c_uint = 0x5;
pub const TWL5031_INTERRUPTS_BCIEDR2: c_uint = 0x6;
pub const TWL5031_INTERRUPTS_BCISIHCTRL: c_uint = 0x7;
// ----------------------------------------------------------------------
//
// PM Master module register offsets (use TWL4030_MODULE_PM_MASTER)
//
pub const TWL4030_PM_MASTER_CFG_P1_TRANSITION: c_uint = 0x00;
pub const TWL4030_PM_MASTER_CFG_P2_TRANSITION: c_uint = 0x01;
pub const TWL4030_PM_MASTER_CFG_P3_TRANSITION: c_uint = 0x02;
pub const TWL4030_PM_MASTER_CFG_P123_TRANSITION: c_uint = 0x03;
pub const TWL4030_PM_MASTER_STS_BOOT: c_uint = 0x04;
pub const TWL4030_PM_MASTER_CFG_BOOT: c_uint = 0x05;
pub const TWL4030_PM_MASTER_SHUNDAN: c_uint = 0x06;
pub const TWL4030_PM_MASTER_BOOT_BCI: c_uint = 0x07;
pub const TWL4030_PM_MASTER_CFG_PWRANA1: c_uint = 0x08;
pub const TWL4030_PM_MASTER_CFG_PWRANA2: c_uint = 0x09;
pub const TWL4030_PM_MASTER_BACKUP_MISC_STS: c_uint = 0x0b;
pub const TWL4030_PM_MASTER_BACKUP_MISC_CFG: c_uint = 0x0c;
pub const TWL4030_PM_MASTER_BACKUP_MISC_TST: c_uint = 0x0d;
pub const TWL4030_PM_MASTER_PROTECT_KEY: c_uint = 0x0e;
pub const TWL4030_PM_MASTER_STS_HW_CONDITIONS: c_uint = 0x0f;
pub const TWL4030_PM_MASTER_P1_SW_EVENTS: c_uint = 0x10;
pub const TWL4030_PM_MASTER_P2_SW_EVENTS: c_uint = 0x11;
pub const TWL4030_PM_MASTER_P3_SW_EVENTS: c_uint = 0x12;
pub const TWL4030_PM_MASTER_STS_P123_STATE: c_uint = 0x13;
pub const TWL4030_PM_MASTER_PB_CFG: c_uint = 0x14;
pub const TWL4030_PM_MASTER_PB_WORD_MSB: c_uint = 0x15;
pub const TWL4030_PM_MASTER_PB_WORD_LSB: c_uint = 0x16;
pub const TWL4030_PM_MASTER_SEQ_ADD_W2P: c_uint = 0x1c;
pub const TWL4030_PM_MASTER_SEQ_ADD_P2A: c_uint = 0x1d;
pub const TWL4030_PM_MASTER_SEQ_ADD_A2W: c_uint = 0x1e;
pub const TWL4030_PM_MASTER_SEQ_ADD_A2S: c_uint = 0x1f;
pub const TWL4030_PM_MASTER_SEQ_ADD_S2A12: c_uint = 0x20;
pub const TWL4030_PM_MASTER_SEQ_ADD_S2A3: c_uint = 0x21;
pub const TWL4030_PM_MASTER_SEQ_ADD_WARM: c_uint = 0x22;
pub const TWL4030_PM_MASTER_MEMORY_ADDRESS: c_uint = 0x23;
pub const TWL4030_PM_MASTER_MEMORY_DATA: c_uint = 0x24;
pub const TWL4030_PM_MASTER_KEY_CFG1: c_uint = 0xc0;
pub const TWL4030_PM_MASTER_KEY_CFG2: c_uint = 0x0c;
pub const TWL4030_PM_MASTER_KEY_TST1: c_uint = 0xe0;
pub const TWL4030_PM_MASTER_KEY_TST2: c_uint = 0x0e;
pub const TWL4030_PM_MASTER_GLOBAL_TST: c_uint = 0xb6;
pub const TWL6030_PHOENIX_DEV_ON: c_uint = 0x06;
// ----------------------------------------------------------------------
// Power bus message definitions
// The TWL4030/5030 splits its power-management resources (the various
// regulators, clock and reset lines) into 3 processor groups - P1, P2 and
// P3. These groups can then be configured to transition between sleep, wait-on
// and active states by sending messages to the power bus.  See Section 5.4.2
// Power Resources of TWL4030 TRM
//
// Processor groups
pub const DEV_GRP_NULL: c_uint = 0x0;
pub const DEV_GRP_P1: c_uint = 0x1	/* P1: all OMAP devices */;
pub const DEV_GRP_P2: c_uint = 0x2	/* P2: all Modem devices */;
pub const DEV_GRP_P3: c_uint = 0x4	/* P3: all peripheral devices */;
// Resource groups
pub const RES_GRP_RES: c_uint = 0x0	/* Reserved */;
pub const RES_GRP_PP: c_uint = 0x1	/* Power providers */;
pub const RES_GRP_RC: c_uint = 0x2	/* Reset and control */;
pub const RES_GRP_PP_RC: c_uint = 0x3;
pub const RES_GRP_PR: c_uint = 0x4	/* Power references */;
pub const RES_GRP_PP_PR: c_uint = 0x5;
pub const RES_GRP_RC_PR: c_uint = 0x6;
pub const RES_GRP_ALL: c_uint = 0x7	/* All resource groups */;
pub const RES_TYPE2_R0: c_uint = 0x0;
pub const RES_TYPE2_R1: c_uint = 0x1;
pub const RES_TYPE2_R2: c_uint = 0x2;
pub const RES_TYPE_R0: c_uint = 0x0;
pub const RES_TYPE_ALL: c_uint = 0x7;
// Resource states
pub const RES_STATE_WRST: c_uint = 0xF;
pub const RES_STATE_ACTIVE: c_uint = 0xE;
pub const RES_STATE_SLEEP: c_uint = 0x8;
pub const RES_STATE_OFF: c_uint = 0x0;
// Power resources
// Power providers
pub const RES_VAUX1: c_int = 1;
pub const RES_VAUX2: c_int = 2;
pub const RES_VAUX3: c_int = 3;
pub const RES_VAUX4: c_int = 4;
pub const RES_VMMC1: c_int = 5;
pub const RES_VMMC2: c_int = 6;
pub const RES_VPLL1: c_int = 7;
pub const RES_VPLL2: c_int = 8;
pub const RES_VSIM: c_int = 9;
pub const RES_VDAC: c_int = 10;
pub const RES_VINTANA1: c_int = 11;
pub const RES_VINTANA2: c_int = 12;
pub const RES_VINTDIG: c_int = 13;
pub const RES_VIO: c_int = 14;
pub const RES_VDD1: c_int = 15;
pub const RES_VDD2: c_int = 16;
pub const RES_VUSB_1V5: c_int = 17;
pub const RES_VUSB_1V8: c_int = 18;
pub const RES_VUSB_3V1: c_int = 19;
pub const RES_VUSBCP: c_int = 20;
pub const RES_REGEN: c_int = 21;
// Reset and control
pub const RES_NRES_PWRON: c_int = 22;
pub const RES_CLKEN: c_int = 23;
pub const RES_SYSEN: c_int = 24;
pub const RES_HFCLKOUT: c_int = 25;
pub const RES_32KCLKOUT: c_int = 26;
pub const RES_RESET: c_int = 27;
// Power Reference
pub const RES_MAIN_REF: c_int = 28;
pub const TOTAL_RESOURCES: c_int = 28;
//
// Power Bus Message Format ... these can be sent individually by Linux,
// but are usually part of downloaded scripts that are run when various
// power events are triggered.
//
// Broadcast Message (16 Bits):
// DEV_GRP[15:13] MT[12]  RES_GRP[11:9]  RES_TYPE2[8:7] RES_TYPE[6:4]
// RES_STATE[3:0]
//
// Singular Message (16 Bits):
// DEV_GRP[15:13] MT[12]  RES_ID[11:4]  RES_STATE[3:0]
//

// ----------------------------------------------------------------------
#[repr(C)]
#[derive(Copy, Clone)]
pub struct twl4030_clock_init_data {
    pub ck32k_lowpwr_enable: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct twl4030_bci_platform_data {
    pub battery_tmp_tbl: *mut c_int,
    pub tblsize: c_uint,
    pub /: *mut *mut int bb_uvolt; / voltage to charge backup battery,
    pub /: *mut *mut int bb_uamp; / current for backup battery charging,
}

// TWL4030_GPIO_MAX (18) GPIOs, with interrupts
#[repr(C)]
#[derive(Copy, Clone)]
pub struct twl4030_gpio_platform_data {
// package the two LED signals as output-only GPIOs?
    pub use_leds: bool,
// gpio-n should control VMMC(n+1) if BIT(n) in mmc_cd is set
    pub mmc_cd: u8,
// if BIT(N) is set, or VMMC(n+1) is linked, debounce GPIO-N
    pub debounce: u32,
// For gpio-N, bit (1 << N) in "pullups" is set if that pullup
// should be enabled.  Else, if that bit is set in "pulldowns",
// that pulldown is enabled.  Don't waste power by letting any
// digital inputs float...
//
    pub pullups: u32,
    pub pulldowns: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct twl4030_madc_platform_data {
    pub irq_line: c_int,
}

// Boards have unique mappings of {row, col} --> keycode.
// Column and row are 8 bits each, but range only from 0..7.
// a PERSISTENT_KEY is "always on" and never reported.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct twl4030_keypad_data {
    pub keymap_data: *const matrix_keymap_data,
    pub rows: unsigned,
    pub cols: unsigned,
    pub rep: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum twl4030_usb_mode {
    T2_USB_MODE_ULPI = 1,
    T2_USB_MODE_CEA2011_3PIN = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct twl4030_usb_data {
    pub usb_mode: twl4030_usb_mode,
    pub features: c_ulong,
    pub dev): *mut *mut int (phy_init)(struct device,
    pub dev): *mut *mut int (phy_exit)(struct device,
// Power on/off the PHY
    pub on): *mut *mut *mut int (phy_power)(struct device dev, int iD, int,
// enable/disable  phy clocks
    pub on): *mut *mut *mut int (phy_set_clock)(struct device dev, int,
// suspend/resume of phy
    pub suspend): *mut *mut *mut int (phy_suspend)(struct device dev, int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct twl4030_ins {
    pub pmb_message: u16,
    pub delay: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct twl4030_script {
    pub script: *mut twl4030_ins,
    pub size: unsigned,
    pub flags: u8,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct twl4030_resconfig {
    pub resource: u8,
    pub /: *mut *mut u8 devgroup; / Processor group that Power resource belongs to,
    pub /: *mut *mut u8 type; / Power resource addressed, 6 / broadcast message,
    pub /: *mut *mut u8 type2; / Power resource addressed, 3 / broadcast message,
    pub /: *mut *mut u8 remap_off; / off state remapping,
    pub /: *mut *mut u8 remap_sleep; / sleep state remapping,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct twl4030_power_data {
    pub scripts: *mut twl4030_script,
    pub num: unsigned,
    pub resource_config: *mut twl4030_resconfig,
    pub board_config: *mut twl4030_resconfig,

    pub /: *mut *mut bool use_poweroff; / Board is wired for TWL poweroff,
    pub /: *mut *mut bool ac_charger_quirk; / Disable AC charger on board,
}

extern "C" {
    pub fn twl4030_remove_script(flags: u8) -> c_int;
}
extern "C" {
    pub fn twl4030_power_off();
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct twl4030_codec_data {
    pub /: *mut *mut unsigned int digimic_delay; / in ms,
    pub ramp_delay_value: c_uint,
    pub offset_cncl_path: c_uint,
    pub hs_extmute:1: c_uint,
    pub hs_extmute_gpio: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct twl4030_vibra_data {
    pub coexist: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct twl4030_audio_data {
    pub audio_mclk: c_uint,
    pub codec: *mut twl4030_codec_data,
    pub vibra: *mut twl4030_vibra_data,
// twl6040
    pub /: *mut *mut int audpwron_gpio; / audio power-on gpio,
    pub /: *mut *mut int naudint_irq; / audio interrupt,
    pub irq_base: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct twl_regulator_driver_data {
    pub target_uV): *mut *mut *mut int (set_voltage)(void data, int,
    pub data): *mut *mut int (get_voltage)(void,
    pub data: *mut c_void,
    pub features: c_ulong,
}

// chip-specific feature flags, for twl_regulator_driver_data.features

// but not officially supported.
// This flag is necessary to
// enable them.
//
// ----------------------------------------------------------------------
extern "C" {
    pub fn twl4030_sih_setup(dev: *mut device, module: c_int, irq_base: c_int) -> c_int;
}
// Offsets to Power Registers
pub const TWL4030_VDAC_DEV_GRP: c_uint = 0x3B;
pub const TWL4030_VDAC_DEDICATED: c_uint = 0x3E;
pub const TWL4030_VAUX1_DEV_GRP: c_uint = 0x17;
pub const TWL4030_VAUX1_DEDICATED: c_uint = 0x1A;
pub const TWL4030_VAUX2_DEV_GRP: c_uint = 0x1B;
pub const TWL4030_VAUX2_DEDICATED: c_uint = 0x1E;
pub const TWL4030_VAUX3_DEV_GRP: c_uint = 0x1F;
pub const TWL4030_VAUX3_DEDICATED: c_uint = 0x22;
// ----------------------------------------------------------------------
// Linux-specific regulator identifiers ... for now, we only support
// the LDOs, and leave the three buck converters alone.  VDD1 and VDD2
// need to tie into hardware based voltage scaling (cpufreq etc), while
// VIO is generally fixed.
//
// TWL4030 SMPS/LDO's
// EXTERNAL dc-to-dc buck converters
pub const TWL4030_REG_VDD1: c_int = 0;
pub const TWL4030_REG_VDD2: c_int = 1;
pub const TWL4030_REG_VIO: c_int = 2;
// EXTERNAL LDOs
pub const TWL4030_REG_VDAC: c_int = 3;
pub const TWL4030_REG_VPLL1: c_int = 4;

pub const TWL4030_REG_VMMC1: c_int = 6;

// INTERNAL LDOs
pub const TWL4030_REG_VINTANA1: c_int = 14;
pub const TWL4030_REG_VINTANA2: c_int = 15;
pub const TWL4030_REG_VINTDIG: c_int = 16;
pub const TWL4030_REG_VUSB1V5: c_int = 17;
pub const TWL4030_REG_VUSB1V8: c_int = 18;
pub const TWL4030_REG_VUSB3V1: c_int = 19;
// TWL6030 SMPS/LDO's
// EXTERNAL dc-to-dc buck convertor controllable via SR
pub const TWL6030_REG_VDD1: c_int = 30;
pub const TWL6030_REG_VDD2: c_int = 31;
pub const TWL6030_REG_VDD3: c_int = 32;
// Non SR compliant dc-to-dc buck convertors
pub const TWL6030_REG_VMEM: c_int = 33;
pub const TWL6030_REG_V2V1: c_int = 34;
pub const TWL6030_REG_V1V29: c_int = 35;
pub const TWL6030_REG_V1V8: c_int = 36;
// EXTERNAL LDOs
pub const TWL6030_REG_VAUX1_6030: c_int = 37;
pub const TWL6030_REG_VAUX2_6030: c_int = 38;
pub const TWL6030_REG_VAUX3_6030: c_int = 39;
pub const TWL6030_REG_VMMC: c_int = 40;
pub const TWL6030_REG_VPP: c_int = 41;
pub const TWL6030_REG_VUSIM: c_int = 42;
pub const TWL6030_REG_VANA: c_int = 43;
pub const TWL6030_REG_VCXIO: c_int = 44;
pub const TWL6030_REG_VDAC: c_int = 45;
pub const TWL6030_REG_VUSB: c_int = 46;
// INTERNAL LDOs
pub const TWL6030_REG_VRTC: c_int = 47;
pub const TWL6030_REG_CLK32KG: c_int = 48;
// LDOs on 6025 have different names
pub const TWL6032_REG_LDO2: c_int = 49;
pub const TWL6032_REG_LDO4: c_int = 50;
pub const TWL6032_REG_LDO3: c_int = 51;
pub const TWL6032_REG_LDO5: c_int = 52;
pub const TWL6032_REG_LDO1: c_int = 53;
pub const TWL6032_REG_LDO7: c_int = 54;
pub const TWL6032_REG_LDO6: c_int = 55;
pub const TWL6032_REG_LDOLN: c_int = 56;
pub const TWL6032_REG_LDOUSB: c_int = 57;
// 6025 DCDC supplies
pub const TWL6032_REG_SMPS3: c_int = 58;
pub const TWL6032_REG_SMPS4: c_int = 59;
pub const TWL6032_REG_VIO: c_int = 60;
