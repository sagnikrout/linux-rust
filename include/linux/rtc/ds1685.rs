//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/rtc/ds1685.h
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
// Definitions for the registers, addresses, and platform data of the
// DS1685/DS1687-series RTC chips.
//
// This Driver also works for the DS17X85/DS17X87 RTC chips.  Functionally
// similar to the DS1685/DS1687, they support a few extra features which
// include larger, battery-backed NV-SRAM, burst-mode access, and an RTC
// write counter.
//
// Copyright (C) 2011-2014 Joshua Kinard <linux@kumba.dev>.
// Copyright (C) 2009 Matthias Fuchs <matthias.fuchs@esd-electronics.com>.
//
// References:
// DS1685/DS1687 3V/5V Real-Time Clocks, 19-5215, Rev 4/10.
// DS17x85/DS17x87 3V/5V Real-Time Clocks, 19-5222, Rev 4/10.
// DS1689/DS1693 3V/5V Serialized Real-Time Clocks, Rev 112105.
// Application Note 90, Using the Multiplex Bus RTC Extended Features.
//

//
// struct ds1685_priv - DS1685 private data structure.
// @dev: pointer to the rtc_device structure.
// @regs: iomapped base address pointer of the RTC registers.
// @regstep: padding/step size between registers (optional).
// @baseaddr: base address of the RTC device.
// @size: resource size.
// @lock: private lock variable for spin locking/unlocking.
// @work: private workqueue.
// @irq: IRQ number assigned to the RTC device.
// @prepare_poweroff: pointer to platform pre-poweroff function.
// @wake_alarm: pointer to platform wake alarm function.
// @post_ram_clear: pointer to platform post ram-clear function.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ds1685_priv {
    pub dev: *mut rtc_device,
    pub regs: *mut void __iomem,
    pub data: *mut void __iomem,
    pub regstep: u32,
    pub irq_num: c_int,
    pub bcd_mode: bool,
    pub int): *mut *mut *mut u8 (read)(struct ds1685_priv ,,
    pub u8): *mut *mut *mut void (write)(struct ds1685_priv , int,,
    pub (*prepare_poweroff)(void): *mut c_void,
    pub (*wake_alarm)(void): *mut c_void,
    pub (*post_ram_clear)(void): *mut c_void,
}

//
// struct ds1685_rtc_platform_data - platform data structure.
// @plat_prepare_poweroff: platform-specific pre-poweroff function.
// @plat_wake_alarm: platform-specific wake alarm function.
// @plat_post_ram_clear: platform-specific post ram-clear function.
//
// If your platform needs to use a custom padding/step size between
// registers, or uses one or more of the extended interrupts and needs special
// handling, then include this header file in your platform definition and
// set regstep and the plat_* pointers as appropriate.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ds1685_rtc_platform_data {
    pub regstep: u32,
    pub bcd_mode: bool,
    pub no_irq: bool,
    pub uie_unsupported: bool,
    pub (*plat_prepare_poweroff)(void): *mut c_void,
    pub (*plat_wake_alarm)(void): *mut c_void,
    pub (*plat_post_ram_clear)(void): *mut c_void,
    pub access_type: },
}

//
// Time Registers.
//
pub const RTC_SECS: c_uint = 0x00	/* Seconds 00-59 */;
pub const RTC_SECS_ALARM: c_uint = 0x01	/* Alarm Seconds 00-59 */;
pub const RTC_MINS: c_uint = 0x02	/* Minutes 00-59 */;
pub const RTC_MINS_ALARM: c_uint = 0x03	/* Alarm Minutes 00-59 */;
pub const RTC_HRS: c_uint = 0x04	/* Hours 01-12 AM/PM || 00-23 */;
pub const RTC_HRS_ALARM: c_uint = 0x05	/* Alarm Hours 01-12 AM/PM || 00-23 */;
pub const RTC_WDAY: c_uint = 0x06	/* Day of Week 01-07 */;
pub const RTC_MDAY: c_uint = 0x07	/* Day of Month 01-31 */;
pub const RTC_MONTH: c_uint = 0x08	/* Month 01-12 */;
pub const RTC_YEAR: c_uint = 0x09	/* Year 00-99 */;
pub const RTC_CENTURY: c_uint = 0x48	/* Century 00-99 */;
pub const RTC_MDAY_ALARM: c_uint = 0x49	/* Alarm Day of Month 01-31 */;
//
// Bit masks for the Time registers in BCD Mode (DM = 0).
//
pub const RTC_SECS_BCD_MASK: c_uint = 0x7f	/* - x x x x x x x */;
pub const RTC_MINS_BCD_MASK: c_uint = 0x7f	/* - x x x x x x x */;
pub const RTC_HRS_12_BCD_MASK: c_uint = 0x1f	/* - - - x x x x x */;
pub const RTC_HRS_24_BCD_MASK: c_uint = 0x3f	/* - - x x x x x x */;
pub const RTC_MDAY_BCD_MASK: c_uint = 0x3f	/* - - x x x x x x */;
pub const RTC_MONTH_BCD_MASK: c_uint = 0x1f	/* - - - x x x x x */;
pub const RTC_YEAR_BCD_MASK: c_uint = 0xff	/* x x x x x x x x */;
//
// Bit masks for the Time registers in BIN Mode (DM = 1).
//
pub const RTC_SECS_BIN_MASK: c_uint = 0x3f	/* - - x x x x x x */;
pub const RTC_MINS_BIN_MASK: c_uint = 0x3f	/* - - x x x x x x */;
pub const RTC_HRS_12_BIN_MASK: c_uint = 0x0f	/* - - - - x x x x */;
pub const RTC_HRS_24_BIN_MASK: c_uint = 0x1f	/* - - - x x x x x */;
pub const RTC_MDAY_BIN_MASK: c_uint = 0x1f	/* - - - x x x x x */;
pub const RTC_MONTH_BIN_MASK: c_uint = 0x0f	/* - - - - x x x x */;
pub const RTC_YEAR_BIN_MASK: c_uint = 0x7f	/* - x x x x x x x */;
//
// Bit masks common for the Time registers in BCD or BIN Mode.
//
pub const RTC_WDAY_MASK: c_uint = 0x07	/* - - - - - x x x */;
pub const RTC_CENTURY_MASK: c_uint = 0xff	/* x x x x x x x x */;
pub const RTC_MDAY_ALARM_MASK: c_uint = 0xff	/* x x x x x x x x */;

//
// Control Registers.
//
pub const RTC_CTRL_A: c_uint = 0x0a	/* Control Register A */;
pub const RTC_CTRL_B: c_uint = 0x0b	/* Control Register B */;
pub const RTC_CTRL_C: c_uint = 0x0c	/* Control Register C */;
pub const RTC_CTRL_D: c_uint = 0x0d	/* Control Register D */;
pub const RTC_EXT_CTRL_4A: c_uint = 0x4a	/* Extended Control Register 4A */;
pub const RTC_EXT_CTRL_4B: c_uint = 0x4b	/* Extended Control Register 4B */;
//
// Bit names in Control Register A.
//

pub const RTC_CTRL_A_RS_MASK: c_uint = 0x0f	/* RS3 + RS2 + RS1 + RS0 */;
//
// Bit names in Control Register B.
//

pub const RTC_CTRL_B_PAU_MASK: c_uint = 0x70	/* PIE + AIE + UIE */;
//
// Bit names in Control Register C.
//
// BIT(0), BIT(1), BIT(2), & BIT(3) are unused, always return 0, and cannot
// be written to.
//

pub const RTC_CTRL_C_PAU_MASK: c_uint = 0x70	/* PF + AF + UF */;
//
// Bit names in Control Register D.
//
// BIT(0) through BIT(6) are unused, always return 0, and cannot
// be written to.
//

//
// Bit names in Extended Control Register 4A.
//
// On the DS1685/DS1687/DS1689/DS1693, BIT(4) and BIT(5) are reserved for
// future use.  They can be read from and written to, but have no effect
// on the RTC's operation.
//
// On the DS17x85/DS17x87, BIT(5) is Burst-Mode Enable (BME), and allows
// access to the extended NV-SRAM by automatically incrementing the address
// register when they are read from or written to.
//

pub const RTC_CTRL_4A_RWK_MASK: c_uint = 0x07	/* RF + WF + KF */;
//
// Bit names in Extended Control Register 4B.
//

pub const RTC_CTRL_4B_RWK_MASK: c_uint = 0x07	/* RIE + WIE + KSE */;
//
// Misc register names in Bank 1.
//
// The DV0 bit in Control Register A must be set to 1 for these registers
// to become available, including Extended Control Registers 4A & 4B.
//
pub const RTC_BANK1_SSN_MODEL: c_uint = 0x40	/* Model Number */;
pub const RTC_BANK1_SSN_BYTE_1: c_uint = 0x41	/* 1st Byte of Serial Number */;
pub const RTC_BANK1_SSN_BYTE_2: c_uint = 0x42	/* 2nd Byte of Serial Number */;
pub const RTC_BANK1_SSN_BYTE_3: c_uint = 0x43	/* 3rd Byte of Serial Number */;
pub const RTC_BANK1_SSN_BYTE_4: c_uint = 0x44	/* 4th Byte of Serial Number */;
pub const RTC_BANK1_SSN_BYTE_5: c_uint = 0x45	/* 5th Byte of Serial Number */;
pub const RTC_BANK1_SSN_BYTE_6: c_uint = 0x46	/* 6th Byte of Serial Number */;
pub const RTC_BANK1_SSN_CRC: c_uint = 0x47	/* Serial CRC Byte */;
pub const RTC_BANK1_RAM_DATA_PORT: c_uint = 0x53	/* Extended RAM Data Port */;
//
// Model-specific registers in Bank 1.
//
// The addresses below differ depending on the model of the RTC chip
// selected in the kernel configuration.  Not all of these features are
// supported in the main driver at present.
//
// DS1685/DS1687   - Extended NV-SRAM address (LSB only).
// DS1689/DS1693   - Vcc, Vbat, Pwr Cycle Counters & Customer-specific S/N.
// DS17x85/DS17x87 - Extended NV-SRAM addresses (MSB & LSB) & Write counter.
//

pub const RTC_BANK1_RAM_ADDR: c_uint = 0x50	/* NV-SRAM Addr */;

pub const RTC_BANK1_VCC_CTR_LSB: c_uint = 0x54	/* Vcc Counter Addr (LSB) */;
pub const RTC_BANK1_VCC_CTR_MSB: c_uint = 0x57	/* Vcc Counter Addr (MSB) */;
pub const RTC_BANK1_VBAT_CTR_LSB: c_uint = 0x58	/* Vbat Counter Addr (LSB) */;
pub const RTC_BANK1_VBAT_CTR_MSB: c_uint = 0x5b	/* Vbat Counter Addr (MSB) */;
pub const RTC_BANK1_PWR_CTR_LSB: c_uint = 0x5c	/* Pwr Cycle Counter Addr (LSB) */;
pub const RTC_BANK1_PWR_CTR_MSB: c_uint = 0x5d	/* Pwr Cycle Counter Addr (MSB) */;
pub const RTC_BANK1_UNIQ_SN: c_uint = 0x60	/* Customer-specific S/N */;

pub const RTC_BANK1_RAM_ADDR_LSB: c_uint = 0x50	/* NV-SRAM Addr (LSB) */;
pub const RTC_BANK1_RAM_ADDR_MSB: c_uint = 0x51	/* NV-SRAM Addr (MSB) */;
pub const RTC_BANK1_WRITE_CTR: c_uint = 0x5e	/* RTC Write Counter */;

//
// Model numbers.
//
// The DS1688/DS1691 and DS1689/DS1693 chips share the same model number
// and the manual doesn't indicate any major differences.  As such, they
// are regarded as the same chip in this driver.
//
pub const RTC_MODEL_DS1685: c_uint = 0x71	/* DS1685/DS1687 */;
pub const RTC_MODEL_DS17285: c_uint = 0x72	/* DS17285/DS17287 */;
pub const RTC_MODEL_DS1689: c_uint = 0x73	/* DS1688/DS1691/DS1689/DS1693 */;
pub const RTC_MODEL_DS17485: c_uint = 0x74	/* DS17485/DS17487 */;
pub const RTC_MODEL_DS17885: c_uint = 0x78	/* DS17885/DS17887 */;
//
// Periodic Interrupt Rates / Square-Wave Output Frequency
//
// Periodic rates are selected by setting the RS3-RS0 bits in Control
// Register A and enabled via either the E32K bit in Extended Control
// Register 4B or the SQWE bit in Control Register B.
//
// E32K overrides the settings of RS3-RS0 and outputs a frequency of 32768Hz
// on the SQW pin of the RTC chip.  While there are 16 possible selections,
// the 1-of-16 decoder is only able to divide the base 32768Hz signal into 13
// smaller frequencies.  The values 0x01 and 0x02 are not used and are
// synonymous with 0x08 and 0x09, respectively.
//
// When E32K is set to a logic 1, periodic interrupts are disabled and reading
// /dev/rtc will return -EINVAL.  This also applies if the periodic interrupt
// frequency is set to 0Hz.
//
// Not currently used by the rtc-ds1685 driver because the RTC core removed
// support for hardware-generated periodic-interrupts in favour of
// hrtimer-generated interrupts.  But these defines are kept around for use
// in userland, as documentation to the hardware, and possible future use if
// hardware-generated periodic interrupts are ever added back.
//
// E32K RS3 RS2 RS1 RS0
pub const RTC_SQW_8192HZ: c_uint = 0x03	/*  0    0   0   1   1  */;
pub const RTC_SQW_4096HZ: c_uint = 0x04	/*  0    0   1   0   0  */;
pub const RTC_SQW_2048HZ: c_uint = 0x05	/*  0    0   1   0   1  */;
pub const RTC_SQW_1024HZ: c_uint = 0x06	/*  0    0   1   1   0  */;
pub const RTC_SQW_512HZ: c_uint = 0x07	/*  0    0   1   1   1  */;
pub const RTC_SQW_256HZ: c_uint = 0x08	/*  0    1   0   0   0  */;
pub const RTC_SQW_128HZ: c_uint = 0x09	/*  0    1   0   0   1  */;
pub const RTC_SQW_64HZ: c_uint = 0x0a	/*  0    1   0   1   0  */;
pub const RTC_SQW_32HZ: c_uint = 0x0b	/*  0    1   0   1   1  */;
pub const RTC_SQW_16HZ: c_uint = 0x0c	/*  0    1   1   0   0  */;
pub const RTC_SQW_8HZ: c_uint = 0x0d	/*  0    1   1   0   1  */;
pub const RTC_SQW_4HZ: c_uint = 0x0e	/*  0    1   1   1   0  */;
pub const RTC_SQW_2HZ: c_uint = 0x0f	/*  0    1   1   1   1  */;
pub const RTC_SQW_0HZ: c_uint = 0x00	/*  0    0   0   0   0  */;

//
// NVRAM data & addresses:
// - 50 bytes of NVRAM are available just past the clock registers.
// - 64 additional bytes are available in Bank0.
//
// Extended, battery-backed NV-SRAM:
// - DS1685/DS1687    - 128 bytes.
// - DS1689/DS1693    - 0 bytes.
// - DS17285/DS17287  - 2048 bytes.
// - DS17485/DS17487  - 4096 bytes.
// - DS17885/DS17887  - 8192 bytes.
//
pub const NVRAM_TIME_BASE: c_uint = 0x0e	/* NVRAM Addr in Time regs */;
pub const NVRAM_BANK0_BASE: c_uint = 0x40	/* NVRAM Addr in Bank0 regs */;
pub const NVRAM_SZ_TIME: c_int = 50;
pub const NVRAM_SZ_BANK0: c_int = 64;

//
// Function Prototypes.
//
