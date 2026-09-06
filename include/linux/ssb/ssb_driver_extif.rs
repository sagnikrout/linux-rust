//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ssb/ssb_driver_extif.h
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
// Hardware-specific External Interface I/O core definitions
// for the BCM47xx family of SiliconBackplane-based chips.
//
// The External Interface core supports a total of three external chip selects
// supporting external interfaces. One of the external chip selects is
// used for Flash, one is used for PCMCIA, and the other may be
// programmed to support either a synchronous interface or an
// asynchronous interface. The asynchronous interface can be used to
// support external devices such as UARTs and the BCM2019 Bluetooth
// baseband processor.
// The external interface core also contains 2 on-chip 16550 UARTs, clock
// frequency control, a watchdog interrupt timer, and a GPIO interface.
//
// Copyright 2005, Broadcom Corporation
// Copyright 2006, Michael Buesch
//
// external interface address space

pub const SSB_EXTIF_NR_GPIOOUT: c_int = 5;
// GPIO NOTE:
// The multiple instances of output and output enable registers
// are present to allow driver software for multiple cores to control
// gpio outputs without needing to share a single register pair.
// Use the following helper macro to get a register offset value.
//

// EXTIF core registers
pub const SSB_EXTIF_CTL: c_uint = 0x0000;

pub const SSB_EXTIF_EXTSTAT: c_uint = 0x0004;

pub const SSB_EXTIF_PCMCIA_CFG: c_uint = 0x0010;
pub const SSB_EXTIF_PCMCIA_MEMWAIT: c_uint = 0x0014;
pub const SSB_EXTIF_PCMCIA_ATTRWAIT: c_uint = 0x0018;
pub const SSB_EXTIF_PCMCIA_IOWAIT: c_uint = 0x001C;
pub const SSB_EXTIF_PROG_CFG: c_uint = 0x0020;
pub const SSB_EXTIF_PROG_WAITCNT: c_uint = 0x0024;
pub const SSB_EXTIF_FLASH_CFG: c_uint = 0x0028;
pub const SSB_EXTIF_FLASH_WAITCNT: c_uint = 0x002C;
pub const SSB_EXTIF_WATCHDOG: c_uint = 0x0040;
pub const SSB_EXTIF_CLOCK_N: c_uint = 0x0044;
pub const SSB_EXTIF_CLOCK_SB: c_uint = 0x0048;
pub const SSB_EXTIF_CLOCK_PCI: c_uint = 0x004C;
pub const SSB_EXTIF_CLOCK_MII: c_uint = 0x0050;
pub const SSB_EXTIF_GPIO_IN: c_uint = 0x0060;
pub const SSB_EXTIF_GPIO_OUT_BASE: c_uint = 0x0064;
pub const SSB_EXTIF_GPIO_OUTEN_BASE: c_uint = 0x0068;
pub const SSB_EXTIF_EJTAG_OUTEN: c_uint = 0x0090;
pub const SSB_EXTIF_GPIO_INTPOL: c_uint = 0x0094;
pub const SSB_EXTIF_GPIO_INTMASK: c_uint = 0x0098;
pub const SSB_EXTIF_UART_DATA: c_uint = 0x0300;
pub const SSB_EXTIF_UART_TIMER: c_uint = 0x0310;
pub const SSB_EXTIF_UART_FCR: c_uint = 0x0320;
pub const SSB_EXTIF_UART_LCR: c_uint = 0x0330;
pub const SSB_EXTIF_UART_MCR: c_uint = 0x0340;
pub const SSB_EXTIF_UART_LSR: c_uint = 0x0350;
pub const SSB_EXTIF_UART_MSR: c_uint = 0x0360;
pub const SSB_EXTIF_UART_SCRATCH: c_uint = 0x0370;
// pcmcia/prog/flash_config

pub const SSB_EXTCFG_MODE: c_uint = 0xE		/* mode */;
pub const SSB_EXTCFG_MODE_SHIFT: c_int = 1;
pub const SSB_EXTCFG_MODE_FLASH: c_uint = 0x0		/* flash/asynchronous mode */;
pub const SSB_EXTCFG_MODE_SYNC: c_uint = 0x2		/* synchronous mode */;
pub const SSB_EXTCFG_MODE_PCMCIA: c_uint = 0x4		/* pcmcia mode */;

pub const SSB_EXTCFG_CLKDIV: c_uint = 0xC0		/* clock divider */;
pub const SSB_EXTCFG_CLKDIV_SHIFT: c_int = 6;
pub const SSB_EXTCFG_CLKDIV_2: c_uint = 0x0		/* backplane/2 */;
pub const SSB_EXTCFG_CLKDIV_3: c_uint = 0x40		/* backplane/3 */;
pub const SSB_EXTCFG_CLKDIV_4: c_uint = 0x80		/* backplane/4 */;

// pcmcia_memwait
pub const SSB_PCMCIA_MEMW_0: c_uint = 0x0000003F	/* waitcount0 */;
pub const SSB_PCMCIA_MEMW_1: c_uint = 0x00001F00	/* waitcount1 */;
pub const SSB_PCMCIA_MEMW_1_SHIFT: c_int = 8;
pub const SSB_PCMCIA_MEMW_2: c_uint = 0x001F0000	/* waitcount2 */;
pub const SSB_PCMCIA_MEMW_2_SHIFT: c_int = 16;
pub const SSB_PCMCIA_MEMW_3: c_uint = 0x1F000000	/* waitcount3 */;
pub const SSB_PCMCIA_MEMW_3_SHIFT: c_int = 24;
// pcmcia_attrwait
pub const SSB_PCMCIA_ATTW_0: c_uint = 0x0000003F	/* waitcount0 */;
pub const SSB_PCMCIA_ATTW_1: c_uint = 0x00001F00	/* waitcount1 */;
pub const SSB_PCMCIA_ATTW_1_SHIFT: c_int = 8;
pub const SSB_PCMCIA_ATTW_2: c_uint = 0x001F0000	/* waitcount2 */;
pub const SSB_PCMCIA_ATTW_2_SHIFT: c_int = 16;
pub const SSB_PCMCIA_ATTW_3: c_uint = 0x1F000000	/* waitcount3 */;
pub const SSB_PCMCIA_ATTW_3_SHIFT: c_int = 24;
// pcmcia_iowait
pub const SSB_PCMCIA_IOW_0: c_uint = 0x0000003F	/* waitcount0 */;
pub const SSB_PCMCIA_IOW_1: c_uint = 0x00001F00	/* waitcount1 */;
pub const SSB_PCMCIA_IOW_1_SHIFT: c_int = 8;
pub const SSB_PCMCIA_IOW_2: c_uint = 0x001F0000	/* waitcount2 */;
pub const SSB_PCMCIA_IOW_2_SHIFT: c_int = 16;
pub const SSB_PCMCIA_IOW_3: c_uint = 0x1F000000	/* waitcount3 */;
pub const SSB_PCMCIA_IOW_3_SHIFT: c_int = 24;
// prog_waitcount
pub const SSB_PROG_WCNT_0: c_uint = 0x0000001F	/* waitcount0 */;
pub const SSB_PROG_WCNT_1: c_uint = 0x00001F00	/* waitcount1 */;
pub const SSB_PROG_WCNT_1_SHIFT: c_int = 8;
pub const SSB_PROG_WCNT_2: c_uint = 0x001F0000	/* waitcount2 */;
pub const SSB_PROG_WCNT_2_SHIFT: c_int = 16;
pub const SSB_PROG_WCNT_3: c_uint = 0x1F000000	/* waitcount3 */;
pub const SSB_PROG_WCNT_3_SHIFT: c_int = 24;
pub const SSB_PROG_W0: c_uint = 0x0000000C;
pub const SSB_PROG_W1: c_uint = 0x00000A00;
pub const SSB_PROG_W2: c_uint = 0x00020000;
pub const SSB_PROG_W3: c_uint = 0x01000000;
// flash_waitcount
pub const SSB_FLASH_WCNT_0: c_uint = 0x0000001F	/* waitcount0 */;
pub const SSB_FLASH_WCNT_1: c_uint = 0x00001F00	/* waitcount1 */;
pub const SSB_FLASH_WCNT_1_SHIFT: c_int = 8;
pub const SSB_FLASH_WCNT_2: c_uint = 0x001F0000	/* waitcount2 */;
pub const SSB_FLASH_WCNT_2_SHIFT: c_int = 16;
pub const SSB_FLASH_WCNT_3: c_uint = 0x1F000000	/* waitcount3 */;
pub const SSB_FLASH_WCNT_3_SHIFT: c_int = 24;
// watchdog

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssb_extif {
    pub dev: *mut ssb_device,
    pub gpio_lock: spinlock_t,
}

extern "C" {
    pub fn ssb_extif_watchdog_timer_set(extif: *mut ssb_extif, ticks: u32) -> u32;
}
// Extif GPIO pin access
extern "C" {
    pub fn ssb_extif_gpio_in(extif: *mut ssb_extif, mask: u32) -> u32;
}
extern "C" {
    pub fn ssb_extif_gpio_out(extif: *mut ssb_extif, mask: u32, value: u32) -> u32;
}
extern "C" {
    pub fn ssb_extif_gpio_outen(extif: *mut ssb_extif, mask: u32, value: u32) -> u32;
}
extern "C" {
    pub fn ssb_extif_gpio_polarity(extif: *mut ssb_extif, mask: u32, value: u32) -> u32;
}
extern "C" {
    pub fn ssb_extif_gpio_intmask(extif: *mut ssb_extif, mask: u32, value: u32) -> u32;
}

// extif disabled
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssb_extif {
}

