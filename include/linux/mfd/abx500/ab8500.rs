//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/abx500/ab8500.h
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
// Copyright (C) ST-Ericsson SA 2010
//
// Author: Srinidhi Kasagar <srinidhi.kasagar@stericsson.com>
//

//
// AB IC versions
//
// AB8500_VERSION_AB8500 should be 0xFF but will never be read as need a
// non-supported multi-byte I2C access via PRCMU. Set to 0x00 to ease the
// print of version string.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ab8500_version {
    AB8500_VERSION_AB8500 = 0x0,
    AB8500_VERSION_AB8505 = 0x1,
    AB8500_VERSION_AB9540 = 0x2,
    AB8500_VERSION_AB8540 = 0x4,
    AB8500_VERSION_UNDEFINED,
}

// AB8500 CIDs
pub const AB8500_CUTEARLY: c_uint = 0x00;
pub const AB8500_CUT1P0: c_uint = 0x10;
pub const AB8500_CUT1P1: c_uint = 0x11;
pub const AB8500_CUT1P2: c_uint = 0x12 /* Only valid for AB8540 */;
pub const AB8500_CUT2P0: c_uint = 0x20;
pub const AB8500_CUT3P0: c_uint = 0x30;
pub const AB8500_CUT3P3: c_uint = 0x33;
//
// AB8500 bank addresses
//
pub const AB8500_M_FSM_RANK: c_uint = 0x0;
pub const AB8500_SYS_CTRL1_BLOCK: c_uint = 0x1;
pub const AB8500_SYS_CTRL2_BLOCK: c_uint = 0x2;
pub const AB8500_REGU_CTRL1: c_uint = 0x3;
pub const AB8500_REGU_CTRL2: c_uint = 0x4;
pub const AB8500_USB: c_uint = 0x5;
pub const AB8500_TVOUT: c_uint = 0x6;
pub const AB8500_DBI: c_uint = 0x7;
pub const AB8500_ECI_AV_ACC: c_uint = 0x8;
pub const AB8500_RESERVED: c_uint = 0x9;
pub const AB8500_GPADC: c_uint = 0xA;
pub const AB8500_CHARGER: c_uint = 0xB;
pub const AB8500_GAS_GAUGE: c_uint = 0xC;
pub const AB8500_AUDIO: c_uint = 0xD;
pub const AB8500_INTERRUPT: c_uint = 0xE;
pub const AB8500_RTC: c_uint = 0xF;
pub const AB8500_MISC: c_uint = 0x10;
pub const AB8500_DEVELOPMENT: c_uint = 0x11;
pub const AB8500_DEBUG: c_uint = 0x12;
pub const AB8500_PROD_TEST: c_uint = 0x13;
pub const AB8500_STE_TEST: c_uint = 0x14;
pub const AB8500_OTP_EMUL: c_uint = 0x15;
pub const AB8500_DEBUG_FIELD_LAST: c_uint = 0x16;
//
// Interrupts
// Values used to index into array ab8500_irq_regoffset[] defined in
// drivers/mdf/ab8500-core.c
//
// Definitions for AB8500, AB9540 and AB8540
// ab8500_irq_regoffset[0] -> IT[Source|Latch|Mask]1

pub const AB8500_INT_TEMP_WARM: c_int = 3;
pub const AB8500_INT_PON_KEY2DB_F: c_int = 4;
pub const AB8500_INT_PON_KEY2DB_R: c_int = 5;
pub const AB8500_INT_PON_KEY1DB_F: c_int = 6;
pub const AB8500_INT_PON_KEY1DB_R: c_int = 7;
// ab8500_irq_regoffset[1] -> IT[Source|Latch|Mask]2
pub const AB8500_INT_BATT_OVV: c_int = 8;

pub const AB8500_INT_VBUS_DET_F: c_int = 14;
pub const AB8500_INT_VBUS_DET_R: c_int = 15;
// ab8500_irq_regoffset[2] -> IT[Source|Latch|Mask]3
pub const AB8500_INT_VBUS_CH_DROP_END: c_int = 16;
pub const AB8500_INT_RTC_60S: c_int = 17;
pub const AB8500_INT_RTC_ALARM: c_int = 18;
pub const AB8540_INT_BIF_INT: c_int = 19;
pub const AB8500_INT_BAT_CTRL_INDB: c_int = 20;
pub const AB8500_INT_CH_WD_EXP: c_int = 21;
pub const AB8500_INT_VBUS_OVV: c_int = 22;

// ab8500_irq_regoffset[3] -> IT[Source|Latch|Mask]4
pub const AB8500_INT_CCN_CONV_ACC: c_int = 24;
pub const AB8500_INT_INT_AUD: c_int = 25;
pub const AB8500_INT_CCEOC: c_int = 26;
pub const AB8500_INT_CC_INT_CALIB: c_int = 27;
pub const AB8500_INT_LOW_BAT_F: c_int = 28;
pub const AB8500_INT_LOW_BAT_R: c_int = 29;
pub const AB8500_INT_BUP_CHG_NOT_OK: c_int = 30;
pub const AB8500_INT_BUP_CHG_OK: c_int = 31;
// ab8500_irq_regoffset[4] -> IT[Source|Latch|Mask]5

pub const AB8500_INT_ACC_DETECT_1DB_F: c_int = 33;
pub const AB8500_INT_ACC_DETECT_1DB_R: c_int = 34;
pub const AB8500_INT_ACC_DETECT_22DB_F: c_int = 35;
pub const AB8500_INT_ACC_DETECT_22DB_R: c_int = 36;
pub const AB8500_INT_ACC_DETECT_21DB_F: c_int = 37;
pub const AB8500_INT_ACC_DETECT_21DB_R: c_int = 38;
pub const AB8500_INT_GP_SW_ADC_CONV_END: c_int = 39;
// ab8500_irq_regoffset[5] -> IT[Source|Latch|Mask]7

// ab8500_irq_regoffset[6] -> IT[Source|Latch|Mask]8

// ab8500_irq_regoffset[7] -> IT[Source|Latch|Mask]9

pub const AB8500_INT_GPIO10F: c_int = 60;
pub const AB8500_INT_GPIO11F: c_int = 61;

pub const AB8500_INT_GPIO13F: c_int = 63;
// ab8500_irq_regoffset[8] -> IT[Source|Latch|Mask]10

// ab8500_irq_regoffset[9] -> IT[Source|Latch|Mask]12
pub const AB8500_INT_ADP_SOURCE_ERROR: c_int = 72;
pub const AB8500_INT_ADP_SINK_ERROR: c_int = 73;
pub const AB8500_INT_ADP_PROBE_PLUG: c_int = 74;
pub const AB8500_INT_ADP_PROBE_UNPLUG: c_int = 75;
pub const AB8500_INT_ADP_SENSE_OFF: c_int = 76;
pub const AB8500_INT_USB_PHY_POWER_ERR: c_int = 78;
pub const AB8500_INT_USB_LINK_STATUS: c_int = 79;
// ab8500_irq_regoffset[10] -> IT[Source|Latch|Mask]19
pub const AB8500_INT_BTEMP_LOW: c_int = 80;
pub const AB8500_INT_BTEMP_LOW_MEDIUM: c_int = 81;
pub const AB8500_INT_BTEMP_MEDIUM_HIGH: c_int = 82;
pub const AB8500_INT_BTEMP_HIGH: c_int = 83;
// ab8500_irq_regoffset[11] -> IT[Source|Latch|Mask]20
pub const AB8500_INT_SRP_DETECT: c_int = 88;
pub const AB8500_INT_USB_CHARGER_NOT_OKR: c_int = 89;
pub const AB8500_INT_ID_WAKEUP_R: c_int = 90;

pub const AB8500_INT_ID_DET_R1R: c_int = 92;
pub const AB8500_INT_ID_DET_R2R: c_int = 93;
pub const AB8500_INT_ID_DET_R3R: c_int = 94;
pub const AB8500_INT_ID_DET_R4R: c_int = 95;
// ab8500_irq_regoffset[12] -> IT[Source|Latch|Mask]21

pub const AB8500_INT_CHSTOPBYSEC: c_int = 103;
// ab8500_irq_regoffset[13] -> IT[Source|Latch|Mask]22
pub const AB8500_INT_USB_CH_TH_PROT_F: c_int = 104;
pub const AB8500_INT_USB_CH_TH_PROT_R: c_int = 105;

pub const AB8500_INT_CHCURLIMNOHSCHIRP: c_int = 109;
pub const AB8500_INT_CHCURLIMHSCHIRP: c_int = 110;
pub const AB8500_INT_XTAL32K_KO: c_int = 111;
// Definitions for AB9540 / AB8505
// ab8500_irq_regoffset[14] -> IT[Source|Latch|Mask]13

pub const AB9540_INT_IEXT_CH_RF_BFN_R: c_int = 118;
// ab8500_irq_regoffset[15] -> IT[Source|Latch|Mask]14

pub const AB9540_INT_IEXT_CH_RF_BFN_F: c_int = 126;
// ab8500_irq_regoffset[16] -> IT[Source|Latch|Mask]25
pub const AB8505_INT_KEYSTUCK: c_int = 128;
pub const AB8505_INT_IKR: c_int = 129;
pub const AB8505_INT_IKP: c_int = 130;
pub const AB8505_INT_KP: c_int = 131;
pub const AB8505_INT_KEYDEGLITCH: c_int = 132;
pub const AB8505_INT_MODPWRSTATUSF: c_int = 134;
pub const AB8505_INT_MODPWRSTATUSR: c_int = 135;
// ab8500_irq_regoffset[17] -> IT[Source|Latch|Mask]6
pub const AB8500_INT_HOOK_DET_NEG_F: c_int = 138;
pub const AB8500_INT_HOOK_DET_NEG_R: c_int = 139;
pub const AB8500_INT_HOOK_DET_POS_F: c_int = 140;
pub const AB8500_INT_HOOK_DET_POS_R: c_int = 141;
pub const AB8500_INT_PLUG_DET_COMP_F: c_int = 142;
pub const AB8500_INT_PLUG_DET_COMP_R: c_int = 143;
// ab8500_irq_regoffset[18] -> IT[Source|Latch|Mask]23
pub const AB8505_INT_COLL: c_int = 144;
pub const AB8505_INT_RESERR: c_int = 145;
pub const AB8505_INT_FRAERR: c_int = 146;
pub const AB8505_INT_COMERR: c_int = 147;
pub const AB8505_INT_SPDSET: c_int = 148;
pub const AB8505_INT_DSENT: c_int = 149;
pub const AB8505_INT_DREC: c_int = 150;
pub const AB8505_INT_ACC_INT: c_int = 151;
// ab8500_irq_regoffset[19] -> IT[Source|Latch|Mask]24
pub const AB8505_INT_NOPINT: c_int = 152;
// ab8540_irq_regoffset[20] -> IT[Source|Latch|Mask]26
pub const AB8540_INT_IDPLUGDETCOMPF: c_int = 160;
pub const AB8540_INT_IDPLUGDETCOMPR: c_int = 161;
pub const AB8540_INT_FMDETCOMPLOF: c_int = 162;
pub const AB8540_INT_FMDETCOMPLOR: c_int = 163;
pub const AB8540_INT_FMDETCOMPHIF: c_int = 164;
pub const AB8540_INT_FMDETCOMPHIR: c_int = 165;
pub const AB8540_INT_ID5VDETCOMPF: c_int = 166;
pub const AB8540_INT_ID5VDETCOMPR: c_int = 167;
// ab8540_irq_regoffset[21] -> IT[Source|Latch|Mask]27
pub const AB8540_INT_GPIO43F: c_int = 168;
pub const AB8540_INT_GPIO43R: c_int = 169;
pub const AB8540_INT_GPIO44F: c_int = 170;
pub const AB8540_INT_GPIO44R: c_int = 171;
pub const AB8540_INT_KEYPOSDETCOMPF: c_int = 172;
pub const AB8540_INT_KEYPOSDETCOMPR: c_int = 173;
pub const AB8540_INT_KEYNEGDETCOMPF: c_int = 174;
pub const AB8540_INT_KEYNEGDETCOMPR: c_int = 175;
// ab8540_irq_regoffset[22] -> IT[Source|Latch|Mask]28
pub const AB8540_INT_GPIO1VBATF: c_int = 176;
pub const AB8540_INT_GPIO1VBATR: c_int = 177;
pub const AB8540_INT_GPIO2VBATF: c_int = 178;
pub const AB8540_INT_GPIO2VBATR: c_int = 179;
pub const AB8540_INT_GPIO3VBATF: c_int = 180;
pub const AB8540_INT_GPIO3VBATR: c_int = 181;
pub const AB8540_INT_GPIO4VBATF: c_int = 182;
pub const AB8540_INT_GPIO4VBATR: c_int = 183;
// ab8540_irq_regoffset[23] -> IT[Source|Latch|Mask]29
pub const AB8540_INT_SYSCLKREQ2F: c_int = 184;
pub const AB8540_INT_SYSCLKREQ2R: c_int = 185;
pub const AB8540_INT_SYSCLKREQ3F: c_int = 186;
pub const AB8540_INT_SYSCLKREQ3R: c_int = 187;
pub const AB8540_INT_SYSCLKREQ4F: c_int = 188;
pub const AB8540_INT_SYSCLKREQ4R: c_int = 189;
pub const AB8540_INT_SYSCLKREQ5F: c_int = 190;
pub const AB8540_INT_SYSCLKREQ5R: c_int = 191;
// ab8540_irq_regoffset[24] -> IT[Source|Latch|Mask]30
pub const AB8540_INT_PWMOUT1F: c_int = 192;
pub const AB8540_INT_PWMOUT1R: c_int = 193;
pub const AB8540_INT_PWMCTRL0F: c_int = 194;
pub const AB8540_INT_PWMCTRL0R: c_int = 195;
pub const AB8540_INT_PWMCTRL1F: c_int = 196;
pub const AB8540_INT_PWMCTRL1R: c_int = 197;
pub const AB8540_INT_SYSCLKREQ6F: c_int = 198;
pub const AB8540_INT_SYSCLKREQ6R: c_int = 199;
// ab8540_irq_regoffset[25] -> IT[Source|Latch|Mask]31
pub const AB8540_INT_PWMEXTVIBRA1F: c_int = 200;
pub const AB8540_INT_PWMEXTVIBRA1R: c_int = 201;
pub const AB8540_INT_PWMEXTVIBRA2F: c_int = 202;
pub const AB8540_INT_PWMEXTVIBRA2R: c_int = 203;
pub const AB8540_INT_PWMOUT2F: c_int = 204;
pub const AB8540_INT_PWMOUT2R: c_int = 205;
pub const AB8540_INT_PWMOUT3F: c_int = 206;
pub const AB8540_INT_PWMOUT3R: c_int = 207;
// ab8540_irq_regoffset[26] -> IT[Source|Latch|Mask]32
pub const AB8540_INT_ADDATA2F: c_int = 208;
pub const AB8540_INT_ADDATA2R: c_int = 209;
pub const AB8540_INT_DADATA2F: c_int = 210;
pub const AB8540_INT_DADATA2R: c_int = 211;
pub const AB8540_INT_FSYNC2F: c_int = 212;
pub const AB8540_INT_FSYNC2R: c_int = 213;
pub const AB8540_INT_BITCLK2F: c_int = 214;
pub const AB8540_INT_BITCLK2R: c_int = 215;
// ab8540_irq_regoffset[27] -> IT[Source|Latch|Mask]33
pub const AB8540_INT_RTC_1S: c_int = 216;
//
// AB8500_AB9540_NR_IRQS is used when configuring the IRQ numbers for the
// entire platform. This is a "compile time" constant so this must be set to
// the largest possible value that may be encountered with different AB SOCs.
// Of the currently supported AB devices, AB8500 and AB9540, it is the AB9540
// which is larger.
//
pub const AB8500_NR_IRQS: c_int = 112;
pub const AB8505_NR_IRQS: c_int = 153;
pub const AB9540_NR_IRQS: c_int = 153;
pub const AB8540_NR_IRQS: c_int = 216;
// This is set to the roof of any AB8500 chip variant IRQ counts

pub const AB8500_NUM_IRQ_REGS: c_int = 14;
pub const AB9540_NUM_IRQ_REGS: c_int = 20;
pub const AB8540_NUM_IRQ_REGS: c_int = 27;
// Turn On Status Event
pub const AB8500_POR_ON_VBAT: c_uint = 0x01;
pub const AB8500_POW_KEY_1_ON: c_uint = 0x02;
pub const AB8500_POW_KEY_2_ON: c_uint = 0x04;
pub const AB8500_RTC_ALARM: c_uint = 0x08;
pub const AB8500_MAIN_CH_DET: c_uint = 0x10;
pub const AB8500_VBUS_DET: c_uint = 0x20;
pub const AB8500_USB_ID_DET: c_uint = 0x40;
//
// struct ab8500 - ab8500 internal structure
// @dev: parent device
// @lock: read/write operations lock
// @irq_lock: genirq bus lock
// @transfer_ongoing: 0 if no transfer ongoing
// @irq: irq line
// @irq_domain: irq domain
// @version: chip version id (e.g. ab8500 or ab9540)
// @chip_id: chip revision id
// @write: register write
// @write_masked: masked register write
// @read: register read
// @rx_buf: rx buf for SPI
// @tx_buf: tx buf for SPI
// @mask: cache of IRQ regs for bus lock
// @oldmask: cache of previous IRQ regs for bus lock
// @mask_size: Actual number of valid entries in mask[], oldmask[] and
// irq_reg_offset
// @irq_reg_offset: Array of offsets into IRQ registers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ab8500 {
    pub dev: *mut device,
    pub lock: mutex,
    pub irq_lock: mutex,
    pub transfer_ongoing: core::sync::atomic::AtomicI32,
    pub irq: c_int,
    pub domain: *mut irq_domain,
    pub version: ab8500_version,
    pub chip_id: u8,
    pub data): *mut *mut *mut int (write)(struct ab8500 ab8500, u16 addr, u8,
    pub data): *mut *mut *mut int (write_masked)(struct ab8500 ab8500, u16 addr, u8 mask, u8,
    pub addr): *mut *mut *mut int (read)(struct ab8500 ab8500, u16,
    pub tx_buf: [c_ulong; 4],
    pub rx_buf: [c_ulong; 4],
    pub mask: *mut u8,
    pub oldmask: *mut u8,
    pub mask_size: c_int,
    pub irq_reg_offset: *const c_int,
    pub it_latchhier_num: c_int,
}

//
// struct ab8500_platform_data - AB8500 platform data
// @irq_base: start of AB8500 IRQs, AB8500_NR_IRQS will be used
// @init: board-specific initialization after detection of ab8500
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ab8500_platform_data {
    pub ): *mut *mut void (init) (struct ab8500,
    pub codec: *mut ab8500_codec_platform_data,
    pub sysctrl: *mut ab8500_sysctrl_platform_data,
}

extern "C" {
    pub fn ab8500_suspend(ab8500: *mut ab8500) -> c_int;
}
// exclude also ab8505, ab9540...
//
// Be careful, the marketing name for this chip is 2.1
// but the value read from the chip is 3.0 (0x30)
//
extern "C" {
    pub fn is_ab8540(AB8500_CUT1P0: ab) && (ab->chip_id <=) -> return;
}
extern "C" {
    pub fn is_ab8540(AB8500_CUT1P1: ab) && (ab->chip_id <=) -> return;
}
extern "C" {
    pub fn is_ab8540(AB8500_CUT1P2: ab) && (ab->chip_id <=) -> return;
}
extern "C" {
    pub fn is_ab8540(AB8500_CUT2P0: ab) && (ab->chip_id <=) -> return;
}
extern "C" {
    pub fn is_ab8540(AB8500_CUT2P0: ab) && (ab->chip_id ==) -> return;
}
extern "C" {
    pub fn ab8500_override_turn_on_stat(mask: u8, set: u8);
}
