//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/fsl/guts.h
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
// Freecale 85xx and 86xx Global Utilties register set
//
// Authors: Jeff Brown
// Timur Tabi <timur@freescale.com>
//
// Copyright 2004,2007,2012 Freescale Semiconductor, Inc
//

//
// Global Utility Registers.
//
// Not all registers defined in this structure are available on all chips, so
// you are expected to know whether a given register actually exists on your
// chip before you access it.
//
// Also, some registers are similar on different chips but have slightly
// different names.  In these cases, one name is chosen to avoid extraneous
// #ifdefs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccsr_guts {
    pub /: *mut *mut u32 porpllsr; / 0x.0000 - POR PLL Ratio Status Register,
    pub /: *mut *mut u32 porbmsr; / 0x.0004 - POR Boot Mode Status Register,
    pub and: *mut *mut u32 porimpscr; / 0x.0008 - POR I/O Impedance Status,
// Control Register
//
    pub /: *mut *mut u32 pordevsr; / 0x.000c - POR I/O Device Status Register,
    pub /: *mut *mut u32 pordbgmsr; / 0x.0010 - POR Debug Mode Status Register,
    pub /: *mut *mut u32 pordevsr2; / 0x.0014 - POR device status register 2,
    pub 0x18]: u8 res018[0x20 -,
    pub Information: *mut *mut u32 porcir; / 0x.0020 - POR Configuration,
// Register
//
    pub 0x24]: u8 res024[0x30 -,
    pub /: *mut *mut u32 gpiocr; / 0x.0030 - GPIO Control Register,
    pub 0x34]: u8 res034[0x40 -,
    pub Data: *mut *mut u32 gpoutdr; / 0x.0040 - General-Purpose Output,
// Register
//
    pub 0x44]: u8 res044[0x50 -,
    pub Data: *mut *mut u32 gpindr; / 0x.0050 - General-Purpose Input,
// Register
//
    pub 0x54]: u8 res054[0x60 -,
    pub Signal: *mut *mut u32 pmuxcr; / 0x.0060 - Alternate Function,
// Multiplex Control
//
    pub signal: *mut *mut u32 pmuxcr2; / 0x.0064 - Alternate function,
// multiplex control 2
//
    pub /: *mut *mut u32 dmuxcr; / 0x.0068 - DMA Mux Control Register,
    pub 0x6c]: u8 res06c[0x70 -,
    pub /: *mut *mut u32 devdisr; / 0x.0070 - Device Disable Control,
pub const CCSR_GUTS_DEVDISR_TB1: c_uint = 0x00001000;
pub const CCSR_GUTS_DEVDISR_TB0: c_uint = 0x00004000;
    pub /: *mut *mut u32 devdisr2; / 0x.0074 - Device Disable Control 2,
    pub 0x78]: u8 res078[0x7c -,
    pub Control: *mut *mut u32 pmjcr; / 0x.007c - 4 Power Management Jog,
// Register
//
    pub and: *mut *mut u32 powmgtcsr; / 0x.0080 - Power Management Status,
// Control Register
//
    pub Counter: *mut *mut u32 pmrccr; / 0x.0084 - Power Management Reset,
// Configuration Register
//
    pub Counter: *mut *mut u32 pmpdccr; / 0x.0088 - Power Management Power Down,
// Configuration Register
//
    pub disable: *mut *mut u32 pmcdr; / 0x.008c - 4Power management clock,
// register
//
    pub /: *mut *mut u32 mcpsumr; / 0x.0090 - Machine Check Summary Register,
    pub and: *mut *mut u32 rstrscr; / 0x.0094 - Reset Request Status,
// Control Register
//
    pub /: *mut *mut u32 ectrstcr; / 0x.0098 - Exception reset control register,
    pub /: *mut *mut u32 autorstsr; / 0x.009c - Automatic reset status register,
    pub /: *mut *mut u32 pvr; / 0x.00a0 - Processor Version Register,
    pub /: *mut *mut u32 svr; / 0x.00a4 - System Version Register,
    pub 0xa8]: u8 res0a8[0xb0 -,
    pub /: *mut *mut u32 rstcr; / 0x.00b0 - Reset Control Register,
    pub 0xb4]: u8 res0b4[0xc0 -,
    pub register: *mut *mut u32 iovselsr; / 0x.00c0 - I/O voltage select status,
    pub 0xc4]: u8 res0c4[0x100 -,
// 0x.0100 - read-only Reset Configuration Word Status registers in
// CCSR, or write-only Reset Configuration Word Control registers in
// DCSR. In both cases there are 32 registers.
//
    pub rcwsr: [u32; 32],
    pub rcwcr: [u32; 32],
}

// register
//
// register
//
// Alternate function signal multiplex control

//
// Set the DMACR register in the GUTS
//
// The DMACR register determines the source of initiated transfers for each
// channel on each DMA controller.  Rather than have a bunch of repetitive
// macros for the bit patterns, we just have a function that calculates
// them.
//
// guts: Pointer to GUTS structure
// co: The DMA controller (0 or 1)
// ch: The channel on the DMA controller (0, 1, 2, or 3)
// device: The device to set as the source (CCSR_GUTS_DMACR_DEV_xx)
//
pub const CCSR_GUTS_PMUXCR_LDPSEL: c_uint = 0x00010000;
pub const CCSR_GUTS_PMUXCR_SSI1_MASK: c_uint = 0x0000C000	/* Bitmask for SSI1 */;
pub const CCSR_GUTS_PMUXCR_SSI1_LA: c_uint = 0x00000000	/* Latched address */;
pub const CCSR_GUTS_PMUXCR_SSI1_HI: c_uint = 0x00004000	/* High impedance */;
pub const CCSR_GUTS_PMUXCR_SSI1_SSI: c_uint = 0x00008000	/* Used for SSI1 */;
pub const CCSR_GUTS_PMUXCR_SSI2_MASK: c_uint = 0x00003000	/* Bitmask for SSI2 */;
pub const CCSR_GUTS_PMUXCR_SSI2_LA: c_uint = 0x00000000	/* Latched address */;
pub const CCSR_GUTS_PMUXCR_SSI2_HI: c_uint = 0x00001000	/* High impedance */;
pub const CCSR_GUTS_PMUXCR_SSI2_SSI: c_uint = 0x00002000	/* Used for SSI2 */;
pub const CCSR_GUTS_PMUXCR_LA_22_25_LA: c_uint = 0x00000000	/* Latched Address */;
pub const CCSR_GUTS_PMUXCR_LA_22_25_HI: c_uint = 0x00000400	/* High impedance */;
pub const CCSR_GUTS_PMUXCR_DBGDRV: c_uint = 0x00000200	/* Signals not driven */;
pub const CCSR_GUTS_PMUXCR_DMA2_0: c_uint = 0x00000008;
pub const CCSR_GUTS_PMUXCR_DMA2_3: c_uint = 0x00000004;
pub const CCSR_GUTS_PMUXCR_DMA1_0: c_uint = 0x00000002;
pub const CCSR_GUTS_PMUXCR_DMA1_3: c_uint = 0x00000001;
//
// Set the DMA external control bits in the GUTS
//
// The DMA external control bits in the PMUXCR are only meaningful for
// channels 0 and 3.  Any other channels are ignored.
//
// guts: Pointer to GUTS structure
// co: The DMA controller (0 or 1)
// ch: The channel on the DMA controller (0, 1, 2, or 3)
// value: the new value for the bit (0 or 1)
//
pub const CCSR_GUTS_CLKDVDR_PXCKEN: c_uint = 0x80000000;
pub const CCSR_GUTS_CLKDVDR_SSICKEN: c_uint = 0x20000000;
pub const CCSR_GUTS_CLKDVDR_PXCKINV: c_uint = 0x10000000;
pub const CCSR_GUTS_CLKDVDR_PXCKDLY_SHIFT: c_int = 25;
pub const CCSR_GUTS_CLKDVDR_PXCKDLY_MASK: c_uint = 0x06000000;

pub const CCSR_GUTS_CLKDVDR_PXCLK_SHIFT: c_int = 16;
pub const CCSR_GUTS_CLKDVDR_PXCLK_MASK: c_uint = 0x001F0000;

pub const CCSR_GUTS_CLKDVDR_SSICLK_MASK: c_uint = 0x000000FF;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccsr_rcpm_v1 {
    pub res0000: [u8; 4],
    pub /: *mut *mut __be32 cdozsr; / 0x0004 Core Doze Status Register,
    pub res0008: [u8; 4],
    pub /: *mut *mut __be32 cdozcr; / 0x000c Core Doze Control Register,
    pub res0010: [u8; 4],
    pub /: *mut *mut __be32 cnapsr; / 0x0014 Core Nap Status Register,
    pub res0018: [u8; 4],
    pub /: *mut *mut __be32 cnapcr; / 0x001c Core Nap Control Register,
    pub res0020: [u8; 4],
    pub /: *mut *mut __be32 cdozpsr; / 0x0024 Core Doze Previous Status Register,
    pub res0028: [u8; 4],
    pub /: *mut *mut __be32 cnappsr; / 0x002c Core Nap Previous Status Register,
    pub res0030: [u8; 4],
    pub /: *mut *mut __be32 cwaitsr; / 0x0034 Core Wait Status Register,
    pub res0038: [u8; 4],
    pub /: *mut *mut __be32 cwdtdsr; / 0x003c Core Watchdog Detect Status Register,
    pub /: *mut *mut __be32 powmgtcsr; / 0x0040 PM Control&Status Register,
pub const RCPM_POWMGTCSR_SLP: c_uint = 0x00020000;
    pub res0044: [u8; 12],
    pub /: *mut *mut __be32 ippdexpcr; / 0x0050 IP Powerdown Exception Control Register,
    pub res0054: [u8; 16],
    pub /: *mut *mut __be32 cpmimr; / 0x0064 Core PM IRQ Mask Register,
    pub res0068: [u8; 4],
    pub /: *mut *mut __be32 cpmcimr; / 0x006c Core PM Critical IRQ Mask Register,
    pub res0070: [u8; 4],
    pub /: *mut *mut __be32 cpmmcmr; / 0x0074 Core PM Machine Check Mask Register,
    pub res0078: [u8; 4],
    pub /: *mut *mut __be32 cpmnmimr; / 0x007c Core PM NMI Mask Register,
    pub res0080: [u8; 4],
    pub /: *mut *mut __be32 ctbenr; / 0x0084 Core Time Base Enable Register,
    pub res0088: [u8; 4],
    pub /: *mut *mut __be32 ctbckselr; / 0x008c Core Time Base Clock Select Register,
    pub res0090: [u8; 4],
    pub /: *mut *mut __be32 ctbhltcr; / 0x0094 Core Time Base Halt Control Register,
    pub res0098: [u8; 4],
    pub /: *mut *mut __be32 cmcpmaskcr; / 0x00a4 Core Machine Check Mask Register,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccsr_rcpm_v2 {
    pub res_00: [u8; 12],
    pub /: *mut *mut __be32 tph10sr0; / Thread PH10 Status Register,
    pub res_10: [u8; 12],
    pub /: *mut *mut __be32 tph10setr0; / Thread PH10 Set Control Register,
    pub res_20: [u8; 12],
    pub /: *mut *mut __be32 tph10clrr0; / Thread PH10 Clear Control Register,
    pub res_30: [u8; 12],
    pub /: *mut *mut __be32 tph10psr0; / Thread PH10 Previous Status Register,
    pub res_40: [u8; 12],
    pub /: *mut *mut __be32 twaitsr0; / Thread Wait Status Register,
    pub res_50: [u8; 96],
    pub /: *mut *mut __be32 pcph15sr; / Physical Core PH15 Status Register,
    pub /: *mut *mut __be32 pcph15setr; / Physical Core PH15 Set Control Register,
    pub /: *mut *mut __be32 pcph15clrr; / Physical Core PH15 Clear Control Register,
    pub /: *mut *mut __be32 pcph15psr; / Physical Core PH15 Prev Status Register,
    pub res_c0: [u8; 16],
    pub /: *mut *mut __be32 pcph20sr; / Physical Core PH20 Status Register,
    pub /: *mut *mut __be32 pcph20setr; / Physical Core PH20 Set Control Register,
    pub /: *mut *mut __be32 pcph20clrr; / Physical Core PH20 Clear Control Register,
    pub /: *mut *mut __be32 pcph20psr; / Physical Core PH20 Prev Status Register,
    pub /: *mut *mut __be32 pcpw20sr; / Physical Core PW20 Status Register,
    pub res_e0: [u8; 12],
    pub /: *mut *mut __be32 pcph30sr; / Physical Core PH30 Status Register,
    pub /: *mut *mut __be32 pcph30setr; / Physical Core PH30 Set Control Register,
    pub /: *mut *mut __be32 pcph30clrr; / Physical Core PH30 Clear Control Register,
    pub /: *mut *mut __be32 pcph30psr; / Physical Core PH30 Prev Status Register,
    pub res_100: [u8; 32],
    pub /: *mut *mut __be32 ippwrgatecr; / IP Power Gating Control Register,
    pub res_124: [u8; 12],
    pub /: *mut *mut __be32 powmgtcsr; / Power Management Control & Status Reg,
pub const RCPM_POWMGTCSR_LPM20_RQ: c_uint = 0x00100000;
pub const RCPM_POWMGTCSR_LPM20_ST: c_uint = 0x00000200;
pub const RCPM_POWMGTCSR_P_LPM20_ST: c_uint = 0x00000100;
    pub res_134: [u8; 12],
    pub /: *mut *mut __be32 ippdexpcr[4]; / IP Powerdown Exception Control Reg,
    pub res_150: [u8; 12],
    pub /: *mut *mut __be32 tpmimr0; / Thread PM Interrupt Mask Reg,
    pub res_160: [u8; 12],
    pub /: *mut *mut __be32 tpmcimr0; / Thread PM Crit Interrupt Mask Reg,
    pub res_170: [u8; 12],
    pub /: *mut *mut __be32 tpmmcmr0; / Thread PM Machine Check Interrupt Mask Reg,
    pub res_180: [u8; 12],
    pub /: *mut *mut __be32 tpmnmimr0; / Thread PM NMI Mask Reg,
    pub res_190: [u8; 12],
    pub /: *mut *mut __be32 tmcpmaskcr0; / Thread Machine Check Mask Control Reg,
    pub /: *mut *mut __be32 pctbenr; / Physical Core Time Base Enable Reg,
    pub /: *mut *mut __be32 pctbclkselr; / Physical Core Time Base Clock Select,
    pub /: *mut *mut __be32 tbclkdivr; / Time Base Clock Divider Register,
    pub res_1ac: [u8; 4],
    pub /: *mut *mut __be32 ttbhltcr[4]; / Thread Time Base Halt Control Register,
    pub /: *mut *mut __be32 clpcl10sr; / Cluster PCL10 Status Register,
    pub /: *mut *mut __be32 clpcl10setr; / Cluster PCL30 Set Control Register,
    pub /: *mut *mut __be32 clpcl10clrr; / Cluster PCL30 Clear Control Register,
    pub /: *mut *mut __be32 clpcl10psr; / Cluster PCL30 Prev Status Register,
    pub /: *mut *mut __be32 cddslpsetr; / Core Domain Deep Sleep Set Register,
    pub /: *mut *mut __be32 cddslpclrr; / Core Domain Deep Sleep Clear Register,
    pub /: *mut *mut __be32 cdpwroksetr; / Core Domain Power OK Set Register,
    pub /: *mut *mut __be32 cdpwrokclrr; / Core Domain Power OK Clear Register,
    pub /: *mut *mut __be32 cdpwrensr; / Core Domain Power Enable Status Register,
    pub /: *mut *mut __be32 cddslsr; / Core Domain Deep Sleep Status Register,
    pub res_1e8: [u8; 8],
    pub /: *mut *mut __be32 dslpcntcr[8]; / Deep Sleep Counter Cfg Register,
    pub res_300: [u8; 3568],
}
