//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/brcm80211/brcmsmac/aiutils.h
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
// Copyright (c) 2011 Broadcom Corporation
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY
// SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION
// OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
// CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//

//
// SOC Interconnect Address Map.
// All regions may not exist on all chips.
//
// each core gets 4Kbytes for registers
pub const SI_CORE_SIZE: c_uint = 0x1000;
//
// Max cores (this is arbitrary, for software
// convenience and could be changed if we
// make any larger chips
//
pub const SI_MAXCORES: c_int = 16;
// Client Mode sb2pcitranslation2 size in bytes
pub const SI_PCI_DMA_SZ: c_uint = 0x40000000;
// PCIE Client Mode sb2pcitranslation2 (2 ZettaBytes), high 32 bits
pub const SI_PCIE_DMA_H32: c_uint = 0x80000000;
// chipcommon being the first core:
pub const SI_CC_IDX: c_int = 0;
// SOC Interconnect types (aka chip types)
pub const SOCI_AI: c_int = 1;
// A register that is common to all cores to
// communicate w/PMU regarding clock control.
//
pub const SI_CLK_CTL_ST: c_uint = 0x1e0	/* clock control and status */;
// clk_ctl_st register
pub const CCS_FORCEALP: c_uint = 0x00000001	/* force ALP request */;
pub const CCS_FORCEHT: c_uint = 0x00000002	/* force HT request */;
pub const CCS_FORCEILP: c_uint = 0x00000004	/* force ILP request */;
pub const CCS_ALPAREQ: c_uint = 0x00000008	/* ALP Avail Request */;
pub const CCS_HTAREQ: c_uint = 0x00000010	/* HT Avail Request */;
pub const CCS_FORCEHWREQOFF: c_uint = 0x00000020	/* Force HW Clock Request Off */;
pub const CCS_ERSRC_REQ_MASK: c_uint = 0x00000700	/* external resource requests */;
pub const CCS_ERSRC_REQ_SHIFT: c_int = 8;
pub const CCS_ALPAVAIL: c_uint = 0x00010000	/* ALP is available */;
pub const CCS_HTAVAIL: c_uint = 0x00020000	/* HT is available */;
pub const CCS_BP_ON_APL: c_uint = 0x00040000	/* RO: running on ALP clock */;
pub const CCS_BP_ON_HT: c_uint = 0x00080000	/* RO: running on HT clock */;
pub const CCS_ERSRC_STS_MASK: c_uint = 0x07000000	/* external resource status */;
pub const CCS_ERSRC_STS_SHIFT: c_int = 24;
// HT avail in chipc and pcmcia on 4328a0
pub const CCS0_HTAVAIL: c_uint = 0x00010000;
// ALP avail in chipc and pcmcia on 4328a0
pub const CCS0_ALPAVAIL: c_uint = 0x00020000;
// Not really related to SOC Interconnect, but a couple of software
// conventions for the use the flash space:
//
// Minimum amount of flash we support
pub const FLASH_MIN: c_uint = 0x00020000	/* Minimum flash size */;
pub const CC_SROM_OTP: c_uint = 0x800	/* SROM/OTP address space */;
// gpiotimerval
pub const GPIO_ONTIME_SHIFT: c_int = 16;
// Fields in clkdiv
pub const CLKD_OTP: c_uint = 0x000f0000;
pub const CLKD_OTP_SHIFT: c_int = 16;
// dynamic clock control defines

// clkctl xtal what flags
pub const XTAL: c_uint = 0x1	/* primary crystal oscillator (2050) */;
pub const PLL: c_uint = 0x2	/* main chip pll */;
// GPIO usage priorities

// reservation
//
// GPIO pull up/down
pub const GPIO_PULLUP: c_int = 0;
pub const GPIO_PULLDN: c_int = 1;
// GPIO event regtype

// device path

// SI routine enumeration: to be used by update function with multiple hooks
pub const SI_DOATTACH: c_int = 1;
pub const SI_PCIDOWN: c_int = 2;
pub const SI_PCIUP: c_int = 3;
//
// Data structure to export all chip specific common variables
// public (read-only) portion of aiutils handle returned by si_attach()
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct si_pub {
    pub /: *mut *mut int ccrev; / chip common core rev,
    pub /: *mut *mut u32 cccaps; / chip common capabilities,
    pub /: *mut *mut int pmurev; / pmu core rev,
    pub /: *mut *mut u32 pmucaps; / pmu capabilities,
    pub /: *mut *mut uint boardtype; / board type,
    pub /: *mut *mut uint boardvendor; / board vendor,
    pub /: *mut *mut uint chip; / chip number,
    pub /: *mut *mut uint chiprev; / chip revision,
    pub /: *mut *mut uint chippkg; / chip package option,
}

// misc si info needed by some of the routines
#[repr(C)]
#[derive(Copy, Clone)]
pub struct si_info {
    pub /: *mut *mut si_pub pub; / back plane public state (must be first),
    pub /: *mut *mut *mut bcma_bus icbus; / handle to soc interconnect bus,
    pub /: *mut *mut *mut pci_dev pcibus; / handle to pci bus,
    pub /: *mut *mut u32 chipst; / chip status,
}

//
// Many of the routines below take an 'sih' handle as their first arg.
// Allocate this by calling si_attach().  Free it by calling si_detach().
// At any one time, the sih is logically focused on one particular si core
// (the "current core").
// Use si_setcore() or si_setcoreidx() to change the association to another core
//
// AMBA Interconnect exported externs
extern "C" {
    pub fn ai_core_cflags(core: *mut bcma_device, mask: u32, val: u32) -> u32;
}
// === exported functions ===
extern "C" {
    pub fn ai_detach(sih: *mut si_pub);
}
extern "C" {
    pub fn ai_cc_reg(sih: *mut si_pub, regoff: c_uint, mask: u32, val: u32) -> c_uint;
}
extern "C" {
    pub fn ai_clkctl_init(sih: *mut si_pub);
}
extern "C" {
    pub fn ai_clkctl_fast_pwrup_delay(sih: *mut si_pub) -> u16;
}
extern "C" {
    pub fn ai_clkctl_cc(sih: *mut si_pub, mode: bcma_clkmode) -> bool;
}
extern "C" {
    pub fn ai_deviceremoved(sih: *mut si_pub) -> bool;
}
// Enable Ex-PA for 4313
extern "C" {
    pub fn ai_epa_4313war(sih: *mut si_pub);
}
