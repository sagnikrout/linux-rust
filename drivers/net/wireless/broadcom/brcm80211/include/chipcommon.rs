//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/brcm80211/include/chipcommon.h
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


// SPDX-License-Identifier: ISC
//
// Copyright (c) 2010 Broadcom Corporation
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct chipcregs {
    pub /: *mut *mut u32 chipid; / 0x0,
    pub capabilities: u32,
    pub /: *mut *mut u32 corecontrol; / corerev >= 1,
    pub bist: u32,
// OTP
    pub /: *mut *mut u32 otpstatus; / 0x10, corerev >= 10,
    pub otpcontrol: u32,
    pub otpprog: u32,
    pub /: *mut *mut u32 otplayout; / corerev >= 23,
// Interrupt control
    pub /: *mut *mut u32 intstatus; / 0x20,
    pub intmask: u32,
// Chip specific regs
    pub /: *mut *mut u32 chipcontrol; / 0x28, rev >= 11,
    pub /: *mut *mut u32 chipstatus; / 0x2c, rev >= 11,
// Jtag Master
    pub /: *mut *mut u32 jtagcmd; / 0x30, rev >= 10,
    pub jtagir: u32,
    pub jtagdr: u32,
    pub jtagctrl: u32,
// serial flash interface registers
    pub /: *mut *mut u32 flashcontrol; / 0x40,
    pub flashaddress: u32,
    pub flashdata: u32,
    pub PAD: [u32; 1],
// Silicon backplane configuration broadcast control
    pub /: *mut *mut u32 broadcastaddress; / 0x50,
    pub broadcastdata: u32,
// gpio - cleared only by power-on-reset
    pub /: *mut *mut u32 gpiopullup; / 0x58, corerev >= 20,
    pub /: *mut *mut u32 gpiopulldown; / 0x5c, corerev >= 20,
    pub /: *mut *mut u32 gpioin; / 0x60,
    pub /: *mut *mut u32 gpioout; / 0x64,
    pub /: *mut *mut u32 gpioouten; / 0x68,
    pub /: *mut *mut u32 gpiocontrol; / 0x6C,
    pub /: *mut *mut u32 gpiointpolarity; / 0x70,
    pub /: *mut *mut u32 gpiointmask; / 0x74,
// GPIO events corerev >= 11
    pub gpioevent: u32,
    pub gpioeventintmask: u32,
// Watchdog timer
    pub /: *mut *mut u32 watchdog; / 0x80,
// GPIO events corerev >= 11
    pub gpioeventintpolarity: u32,
// GPIO based LED powersave registers corerev >= 16
    pub /: *mut *mut u32 gpiotimerval; / 0x88,
    pub gpiotimeroutmask: u32,
// clock control
    pub /: *mut *mut u32 clockcontrol_n; / 0x90,
    pub /: *mut *mut u32 clockcontrol_sb; / aka m0,
    pub /: *mut *mut u32 clockcontrol_pci; / aka m1,
    pub /: *mut *mut u32 clockcontrol_m2; / mii/uart/mipsref,
    pub /: *mut *mut u32 clockcontrol_m3; / cpu,
    pub /: *mut *mut u32 clkdiv; / corerev >= 3,
    pub /: *mut *mut u32 gpiodebugsel; / corerev >= 28,
    pub /: *mut *mut u32 capabilities_ext; / 0xac,
// pll delay registers (corerev >= 4)
    pub /: *mut *mut u32 pll_on_delay; / 0xb0,
    pub fref_sel_delay: u32,
    pub /: *mut *mut u32 slow_clk_ctl; / 5 < corerev < 10,
    pub PAD: u32,
// Instaclock registers (corerev >= 10)
    pub /: *mut *mut u32 system_clk_ctl; / 0xc0,
    pub clkstatestretch: u32,
    pub PAD: [u32; 2],
// Indirect backplane access (corerev >= 22)
    pub /: *mut *mut u32 bp_addrlow; / 0xd0,
    pub bp_addrhigh: u32,
    pub bp_data: u32,
    pub PAD: u32,
    pub bp_indaccess: u32,
    pub PAD: [u32; 3],
// More clock dividers (corerev >= 32)
    pub clkdiv2: u32,
    pub PAD: [u32; 2],
// In AI chips, pointer to erom
    pub /: *mut *mut u32 eromptr; / 0xfc,
// ExtBus control registers (corerev >= 3)
    pub /: *mut *mut u32 pcmcia_config; / 0x100,
    pub pcmcia_memwait: u32,
    pub pcmcia_attrwait: u32,
    pub pcmcia_iowait: u32,
    pub ide_config: u32,
    pub ide_memwait: u32,
    pub ide_attrwait: u32,
    pub ide_iowait: u32,
    pub prog_config: u32,
    pub prog_waitcount: u32,
    pub flash_config: u32,
    pub flash_waitcount: u32,
    pub /: *mut *mut u32 SECI_config; / 0x130 SECI configuration,
    pub PAD: [u32; 3],
// Enhanced Coexistence Interface (ECI) registers (corerev >= 21)
    pub /: *mut *mut u32 eci_output; / 0x140,
    pub eci_control: u32,
    pub eci_inputlo: u32,
    pub eci_inputmi: u32,
    pub eci_inputhi: u32,
    pub eci_inputintpolaritylo: u32,
    pub eci_inputintpolaritymi: u32,
    pub eci_inputintpolarityhi: u32,
    pub eci_intmasklo: u32,
    pub eci_intmaskmi: u32,
    pub eci_intmaskhi: u32,
    pub eci_eventlo: u32,
    pub eci_eventmi: u32,
    pub eci_eventhi: u32,
    pub eci_eventmasklo: u32,
    pub eci_eventmaskmi: u32,
    pub eci_eventmaskhi: u32,
    pub PAD: [u32; 3],
// SROM interface (corerev >= 32)
    pub /: *mut *mut u32 sromcontrol; / 0x190,
    pub sromaddress: u32,
    pub sromdata: u32,
    pub PAD: [u32; 17],
// Clock control and hardware workarounds (corerev >= 20)
    pub /: *mut *mut u32 clk_ctl_st; / 0x1e0,
    pub hw_war: u32,
    pub PAD: [u32; 70],
// UARTs
    pub /: *mut *mut u8 uart0data; / 0x300,
    pub uart0imr: u8,
    pub uart0fcr: u8,
    pub uart0lcr: u8,
    pub uart0mcr: u8,
    pub uart0lsr: u8,
    pub uart0msr: u8,
    pub uart0scratch: u8,
    pub /: *mut *mut u8 PAD[248]; / corerev >= 1,
    pub /: *mut *mut u8 uart1data; / 0x400,
    pub uart1imr: u8,
    pub uart1fcr: u8,
    pub uart1lcr: u8,
    pub uart1mcr: u8,
    pub uart1lsr: u8,
    pub uart1msr: u8,
    pub uart1scratch: u8,
    pub PAD: [u32; 62],
// save/restore, corerev >= 48
    pub /: *mut *mut u32 sr_capability; / 0x500,
    pub /: *mut *mut u32 sr_control0; / 0x504,
    pub /: *mut *mut u32 sr_control1; / 0x508,
    pub /: *mut *mut u32 gpio_control; / 0x50C,
    pub PAD: [u32; 60],
// PMU registers (corerev >= 20)
    pub /: *mut *mut u32 pmucontrol; / 0x600,
    pub pmucapabilities: u32,
    pub pmustatus: u32,
    pub res_state: u32,
    pub res_pending: u32,
    pub pmutimer: u32,
    pub min_res_mask: u32,
    pub max_res_mask: u32,
    pub res_table_sel: u32,
    pub res_dep_mask: u32,
    pub res_updn_timer: u32,
    pub res_timer: u32,
    pub clkstretch: u32,
    pub pmuwatchdog: u32,
    pub /: *mut *mut u32 gpiosel; / 0x638, rev >= 1,
    pub /: *mut *mut u32 gpioenable; / 0x63c, rev >= 1,
    pub res_req_timer_sel: u32,
    pub res_req_timer: u32,
    pub res_req_mask: u32,
    pub /: *mut *mut u32 pmucapabilities_ext; / 0x64c, pmurev >=15,
    pub /: *mut *mut u32 chipcontrol_addr; / 0x650,
    pub /: *mut *mut u32 chipcontrol_data; / 0x654,
    pub regcontrol_addr: u32,
    pub regcontrol_data: u32,
    pub pllcontrol_addr: u32,
    pub pllcontrol_data: u32,
    pub /: *mut *mut u32 pmustrapopt; / 0x668, corerev >= 28,
    pub /: *mut *mut u32 pmu_xtalfreq; / 0x66C, pmurev >= 10,
    pub /: *mut *mut u32 retention_ctl; / 0x670, pmurev >= 15,
    pub PAD: [u32; 3],
    pub /: *mut *mut u32 retention_grpidx; / 0x680,
    pub /: *mut *mut u32 retention_grpctl; / 0x684,
    pub PAD: [u32; 94],
    pub sromotp: [u16; 768],
}

// chipid
pub const CID_ID_MASK: c_uint = 0x0000ffff	/* Chip Id mask */;
pub const CID_REV_MASK: c_uint = 0x000f0000	/* Chip Revision mask */;

pub const CID_PKG_MASK: c_uint = 0x00f00000	/* Package Option mask */;

pub const CID_CC_MASK: c_uint = 0x0f000000	/* CoreCount (corerev >= 4) */;
pub const CID_CC_SHIFT: c_int = 24;
pub const CID_TYPE_MASK: c_uint = 0xf0000000	/* Chip Type */;
pub const CID_TYPE_SHIFT: c_int = 28;
// capabilities
pub const CC_CAP_UARTS_MASK: c_uint = 0x00000003	/* Number of UARTs */;
pub const CC_CAP_MIPSEB: c_uint = 0x00000004	/* MIPS is in big-endian mode */;
pub const CC_CAP_UCLKSEL: c_uint = 0x00000018	/* UARTs clock select */;
// UARTs are driven by internal divided clock
pub const CC_CAP_UINTCLK: c_uint = 0x00000008;
pub const CC_CAP_UARTGPIO: c_uint = 0x00000020	/* UARTs own GPIOs 15:12 */;
pub const CC_CAP_EXTBUS_MASK: c_uint = 0x000000c0	/* External bus mask */;
pub const CC_CAP_EXTBUS_NONE: c_uint = 0x00000000	/* No ExtBus present */;
pub const CC_CAP_EXTBUS_FULL: c_uint = 0x00000040	/* ExtBus: PCMCIA, IDE & Prog */;
pub const CC_CAP_EXTBUS_PROG: c_uint = 0x00000080	/* ExtBus: ProgIf only */;
pub const CC_CAP_FLASH_MASK: c_uint = 0x00000700	/* Type of flash */;
pub const CC_CAP_PLL_MASK: c_uint = 0x00038000	/* Type of PLL */;
pub const CC_CAP_PWR_CTL: c_uint = 0x00040000	/* Power control */;
pub const CC_CAP_OTPSIZE: c_uint = 0x00380000	/* OTP Size (0 = none) */;

pub const CC_CAP_JTAGP: c_uint = 0x00400000	/* JTAG Master Present */;
pub const CC_CAP_ROM: c_uint = 0x00800000	/* Internal boot rom active */;
pub const CC_CAP_BKPLN64: c_uint = 0x08000000	/* 64-bit backplane */;
pub const CC_CAP_PMU: c_uint = 0x10000000	/* PMU Present, rev >= 20 */;
pub const CC_CAP_SROM: c_uint = 0x40000000	/* Srom Present, rev >= 32 */;
// Nand flash present, rev >= 35
pub const CC_CAP_NFLASH: c_uint = 0x80000000;
pub const CC_CAP2_SECI: c_uint = 0x00000001	/* SECI Present, rev >= 36 */;
// GSIO (spi/i2c) present, rev >= 37
pub const CC_CAP2_GSIO: c_uint = 0x00000002;
// sr_control0, rev >= 48

pub const CC_SR_CTL0_ENABLE_SHIFT: c_int = 0;

// sr_engine
//

// in sr_engine
//
pub const CC_SR_CTL0_EN_SBC_STBY_SHIFT: c_int = 16;
pub const CC_SR_CTL0_EN_SR_ALP_CLK_MASK_SHIFT: c_int = 18;
pub const CC_SR_CTL0_EN_SR_HT_CLK_SHIFT: c_int = 19;

// domains
//
pub const CC_SR_CTL0_MAX_SR_LQ_CLK_CNT_SHIFT: c_int = 25;
pub const CC_SR_CTL0_EN_MEM_DISABLE_FOR_SLEEP: c_int = 30;
// pmucapabilities
pub const PCAP_REV_MASK: c_uint = 0x000000ff;
pub const PCAP_RC_MASK: c_uint = 0x00001f00;
pub const PCAP_RC_SHIFT: c_int = 8;
pub const PCAP_TC_MASK: c_uint = 0x0001e000;
pub const PCAP_TC_SHIFT: c_int = 13;
pub const PCAP_PC_MASK: c_uint = 0x001e0000;
pub const PCAP_PC_SHIFT: c_int = 17;
pub const PCAP_VC_MASK: c_uint = 0x01e00000;
pub const PCAP_VC_SHIFT: c_int = 21;
pub const PCAP_CC_MASK: c_uint = 0x1e000000;
pub const PCAP_CC_SHIFT: c_int = 25;
pub const PCAP5_PC_MASK: c_uint = 0x003e0000	/* PMU corerev >= 5 */;
pub const PCAP5_PC_SHIFT: c_int = 17;
pub const PCAP5_VC_MASK: c_uint = 0x07c00000;
pub const PCAP5_VC_SHIFT: c_int = 22;
pub const PCAP5_CC_MASK: c_uint = 0xf8000000;
pub const PCAP5_CC_SHIFT: c_int = 27;
// pmucapabilites_ext PMU rev >= 15

// retention_ctl PMU rev >= 15

//
// Maximum delay for the PMU state transition in us.
// This is an upper bound intended for spinwaits etc.
//
pub const PMU_MAX_TRANSITION_DLY: c_int = 15000;
