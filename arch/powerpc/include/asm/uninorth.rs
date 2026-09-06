//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/uninorth.h
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


// SPDX-License-Identifier: GPL-2.0
//
// uninorth.h: definitions for using the "UniNorth" host bridge chip
// from Apple. This chip is used on "Core99" machines
// This also includes U2 used on more recent MacRISC2/3
// machines and U3 (G5)
//

//
// Uni-N and U3 config space reg. definitions
//
// (Little endian)
//
// Address ranges selection. This one should work with Bandit too
// Not U3
pub const UNI_N_ADDR_SELECT: c_uint = 0x48;
pub const UNI_N_ADDR_COARSE_MASK: c_uint = 0xffff0000	/* 256Mb regions at *0000000 */;
pub const UNI_N_ADDR_FINE_MASK: c_uint = 0x0000ffff	/*  16Mb regions at f*000000 */;
// AGP registers
// Not U3
pub const UNI_N_CFG_GART_BASE: c_uint = 0x8c;
pub const UNI_N_CFG_AGP_BASE: c_uint = 0x90;
pub const UNI_N_CFG_GART_CTRL: c_uint = 0x94;
pub const UNI_N_CFG_INTERNAL_STATUS: c_uint = 0x98;
pub const UNI_N_CFG_GART_DUMMY_PAGE: c_uint = 0xa4;
// UNI_N_CFG_GART_CTRL bits definitions
pub const UNI_N_CFG_GART_INVAL: c_uint = 0x00000001;
pub const UNI_N_CFG_GART_ENABLE: c_uint = 0x00000100;
pub const UNI_N_CFG_GART_2xRESET: c_uint = 0x00010000;
pub const UNI_N_CFG_GART_DISSBADET: c_uint = 0x00020000;
// The following seems to only be used only on U3 <j.glisse@gmail.com>
pub const U3_N_CFG_GART_SYNCMODE: c_uint = 0x00040000;
pub const U3_N_CFG_GART_PERFRD: c_uint = 0x00080000;
pub const U3_N_CFG_GART_B2BGNT: c_uint = 0x00200000;
pub const U3_N_CFG_GART_FASTDDR: c_uint = 0x00400000;
// My understanding of UniNorth AGP as of UniNorth rev 1.0x,
// revision 1.5 (x4 AGP) may need further changes.
//
// AGP_BASE register contains the base address of the AGP aperture on
// the AGP bus. It doesn't seem to be visible to the CPU as of UniNorth 1.x,
// even if decoding of this address range is enabled in the address select
// register. Apparently, the only supported bases are 256Mb multiples
// (high 4 bits of that register).
//
// GART_BASE register appear to contain the physical address of the GART
// in system memory in the high address bits (page aligned), and the
// GART size in the low order bits (number of GART pages)
//
// The GART format itself is one 32bits word per physical memory page.
// This word contains, in little-endian format (!!!), the physical address
// of the page in the high bits, and what appears to be an "enable" bit
// in the LSB bit (0) that must be set to 1 when the entry is valid.
//
// Obviously, the GART is not cache coherent and so any change to it
// must be flushed to memory (or maybe just make the GART space non
// cachable). AGP memory itself doesn't seem to be cache coherent neither.
//
// In order to invalidate the GART (which is probably necessary to inval
// the bridge internal TLBs), the following sequence has to be written,
// in order, to the GART_CTRL register:
//
// UNI_N_CFG_GART_ENABLE | UNI_N_CFG_GART_INVAL
// UNI_N_CFG_GART_ENABLE
// UNI_N_CFG_GART_ENABLE | UNI_N_CFG_GART_2xRESET
// UNI_N_CFG_GART_ENABLE
//
// As far as AGP "features" are concerned, it looks like fast write may
// not be supported but this has to be confirmed.
//
// Turning on AGP seem to require a double invalidate operation, one before
// setting the AGP command register, on after.
//
// Turning off AGP seems to require the following sequence: first wait
// for the AGP to be idle by reading the internal status register, then
// write in that order to the GART_CTRL register:
//
// UNI_N_CFG_GART_ENABLE | UNI_N_CFG_GART_INVAL
// 0
// UNI_N_CFG_GART_2xRESET
// 0
//
// Uni-N memory mapped reg. definitions
//
// Those registers are Big-Endian !!
//
// Their meaning come from either Darwin and/or from experiments I made with
// the bootrom, I'm not sure about their exact meaning yet
//
// Version of the UniNorth chip
pub const UNI_N_VERSION: c_uint = 0x0000		/* Known versions: 3,7 and 8 */;
pub const UNI_N_VERSION_107: c_uint = 0x0003		/* 1.0.7 */;
pub const UNI_N_VERSION_10A: c_uint = 0x0007		/* 1.0.10 */;
pub const UNI_N_VERSION_150: c_uint = 0x0011		/* 1.5 */;
pub const UNI_N_VERSION_200: c_uint = 0x0024		/* 2.0 */;
pub const UNI_N_VERSION_PANGEA: c_uint = 0x00C0		/* Integrated U1 + K */;
pub const UNI_N_VERSION_INTREPID: c_uint = 0x00D2		/* Integrated U2 + K */;
pub const UNI_N_VERSION_300: c_uint = 0x0030		/* 3.0 (U3 on G5) */;
// This register is used to enable/disable various clocks
pub const UNI_N_CLOCK_CNTL: c_uint = 0x0020;
pub const UNI_N_CLOCK_CNTL_PCI: c_uint = 0x00000001	/* PCI2 clock control */;
pub const UNI_N_CLOCK_CNTL_GMAC: c_uint = 0x00000002	/* GMAC clock control */;
pub const UNI_N_CLOCK_CNTL_FW: c_uint = 0x00000004	/* FireWire clock control */;
pub const UNI_N_CLOCK_CNTL_ATA100: c_uint = 0x00000010	/* ATA-100 clock control (U2) */;
// Power Management control
pub const UNI_N_POWER_MGT: c_uint = 0x0030;
pub const UNI_N_POWER_MGT_NORMAL: c_uint = 0x00;
pub const UNI_N_POWER_MGT_IDLE2: c_uint = 0x01;
pub const UNI_N_POWER_MGT_SLEEP: c_uint = 0x02;
// This register is configured by Darwin depending on the UniN
// revision
//
pub const UNI_N_ARB_CTRL: c_uint = 0x0040;
pub const UNI_N_ARB_CTRL_QACK_DELAY_SHIFT: c_int = 15;
pub const UNI_N_ARB_CTRL_QACK_DELAY_MASK: c_uint = 0x0e1f8000;
pub const UNI_N_ARB_CTRL_QACK_DELAY: c_uint = 0x30;
pub const UNI_N_ARB_CTRL_QACK_DELAY105: c_uint = 0x00;
// This one _might_ return the CPU number of the CPU reading it;
// the bootROM decides whether to boot or to sleep/spinloop depending
// on this register being 0 or not
//
pub const UNI_N_CPU_NUMBER: c_uint = 0x0050;
// This register appear to be read by the bootROM to decide what
// to do on a non-recoverable reset (powerup or wakeup)
//
pub const UNI_N_HWINIT_STATE: c_uint = 0x0070;
pub const UNI_N_HWINIT_STATE_SLEEPING: c_uint = 0x01;
pub const UNI_N_HWINIT_STATE_RUNNING: c_uint = 0x02;
// This last bit appear to be used by the bootROM to know the second
// CPU has started and will enter its sleep loop with IP=0
//
pub const UNI_N_HWINIT_STATE_CPU1_FLAG: c_uint = 0x10000000;
// This register controls AACK delay, which is set when 2004 iBook/PowerBook
// is in low speed mode.
//
pub const UNI_N_AACK_DELAY: c_uint = 0x0100;
pub const UNI_N_AACK_DELAY_ENABLE: c_uint = 0x00000001;
// Clock status for Intrepid
pub const UNI_N_CLOCK_STOP_STATUS0: c_uint = 0x0150;
pub const UNI_N_CLOCK_STOPPED_EXTAGP: c_uint = 0x00200000;
pub const UNI_N_CLOCK_STOPPED_AGPDEL: c_uint = 0x00100000;
pub const UNI_N_CLOCK_STOPPED_I2S0_45_49: c_uint = 0x00080000;
pub const UNI_N_CLOCK_STOPPED_I2S0_18: c_uint = 0x00040000;
pub const UNI_N_CLOCK_STOPPED_I2S1_45_49: c_uint = 0x00020000;
pub const UNI_N_CLOCK_STOPPED_I2S1_18: c_uint = 0x00010000;
pub const UNI_N_CLOCK_STOPPED_TIMER: c_uint = 0x00008000;
pub const UNI_N_CLOCK_STOPPED_SCC_RTCLK18: c_uint = 0x00004000;
pub const UNI_N_CLOCK_STOPPED_SCC_RTCLK32: c_uint = 0x00002000;
pub const UNI_N_CLOCK_STOPPED_SCC_VIA32: c_uint = 0x00001000;
pub const UNI_N_CLOCK_STOPPED_SCC_SLOT0: c_uint = 0x00000800;
pub const UNI_N_CLOCK_STOPPED_SCC_SLOT1: c_uint = 0x00000400;
pub const UNI_N_CLOCK_STOPPED_SCC_SLOT2: c_uint = 0x00000200;
pub const UNI_N_CLOCK_STOPPED_PCI_FBCLKO: c_uint = 0x00000100;
pub const UNI_N_CLOCK_STOPPED_VEO0: c_uint = 0x00000080;
pub const UNI_N_CLOCK_STOPPED_VEO1: c_uint = 0x00000040;
pub const UNI_N_CLOCK_STOPPED_USB0: c_uint = 0x00000020;
pub const UNI_N_CLOCK_STOPPED_USB1: c_uint = 0x00000010;
pub const UNI_N_CLOCK_STOPPED_USB2: c_uint = 0x00000008;
pub const UNI_N_CLOCK_STOPPED_32: c_uint = 0x00000004;
pub const UNI_N_CLOCK_STOPPED_45: c_uint = 0x00000002;
pub const UNI_N_CLOCK_STOPPED_49: c_uint = 0x00000001;
pub const UNI_N_CLOCK_STOP_STATUS1: c_uint = 0x0160;
pub const UNI_N_CLOCK_STOPPED_PLL4REF: c_uint = 0x00080000;
pub const UNI_N_CLOCK_STOPPED_CPUDEL: c_uint = 0x00040000;
pub const UNI_N_CLOCK_STOPPED_CPU: c_uint = 0x00020000;
pub const UNI_N_CLOCK_STOPPED_BUF_REFCKO: c_uint = 0x00010000;
pub const UNI_N_CLOCK_STOPPED_PCI2: c_uint = 0x00008000;
pub const UNI_N_CLOCK_STOPPED_FW: c_uint = 0x00004000;
pub const UNI_N_CLOCK_STOPPED_GB: c_uint = 0x00002000;
pub const UNI_N_CLOCK_STOPPED_ATA66: c_uint = 0x00001000;
pub const UNI_N_CLOCK_STOPPED_ATA100: c_uint = 0x00000800;
pub const UNI_N_CLOCK_STOPPED_MAX: c_uint = 0x00000400;
pub const UNI_N_CLOCK_STOPPED_PCI1: c_uint = 0x00000200;
pub const UNI_N_CLOCK_STOPPED_KLPCI: c_uint = 0x00000100;
pub const UNI_N_CLOCK_STOPPED_USB0PCI: c_uint = 0x00000080;
pub const UNI_N_CLOCK_STOPPED_USB1PCI: c_uint = 0x00000040;
pub const UNI_N_CLOCK_STOPPED_USB2PCI: c_uint = 0x00000020;
pub const UNI_N_CLOCK_STOPPED_7PCI1: c_uint = 0x00000008;
pub const UNI_N_CLOCK_STOPPED_AGP: c_uint = 0x00000004;
pub const UNI_N_CLOCK_STOPPED_PCI0: c_uint = 0x00000002;
pub const UNI_N_CLOCK_STOPPED_18: c_uint = 0x00000001;
// Intrepid registe to OF do-platform-clockspreading
pub const UNI_N_CLOCK_SPREADING: c_uint = 0x190;
// Uninorth 1.5 rev. has additional perf. monitor registers at 0xf00-0xf50
//
// U3 specific registers
//
// U3 Toggle
pub const U3_TOGGLE_REG: c_uint = 0x00e0;
pub const U3_PMC_START_STOP: c_uint = 0x0001;
pub const U3_MPIC_RESET: c_uint = 0x0002;
pub const U3_MPIC_OUTPUT_ENABLE: c_uint = 0x0004;
// U3 API PHY Config 1
pub const U3_API_PHY_CONFIG_1: c_uint = 0x23030;
// U3 HyperTransport registers
pub const U3_HT_CONFIG_BASE: c_uint = 0x70000;
pub const U3_HT_LINK_COMMAND: c_uint = 0x100;
pub const U3_HT_LINK_CONFIG: c_uint = 0x110;
pub const U3_HT_LINK_FREQ: c_uint = 0x120;

