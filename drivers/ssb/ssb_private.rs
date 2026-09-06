//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/ssb/ssb_private.h
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

// pci.c

extern "C" {
    pub fn ssb_pci_exit(bus: *mut ssb_bus);
}
extern "C" {
    pub fn ssb_pci_init(bus: *mut ssb_bus) -> c_int;
}

// pcmcia.c

extern "C" {
    pub fn ssb_pcmcia_hardware_setup(bus: *mut ssb_bus) -> c_int;
}
extern "C" {
    pub fn ssb_pcmcia_exit(bus: *mut ssb_bus);
}
extern "C" {
    pub fn ssb_pcmcia_init(bus: *mut ssb_bus) -> c_int;
}
extern "C" {
    pub fn ssb_host_pcmcia_init() -> c_int;
}
extern "C" {
    pub fn ssb_host_pcmcia_exit();
}

// sdio.c

extern "C" {
    pub fn ssb_sdio_scan_read32(bus: *mut ssb_bus, offset: u16) -> u32;
}
extern "C" {
    pub fn ssb_sdio_scan_switch_coreidx(bus: *mut ssb_bus, coreidx: u8) -> c_int;
}
extern "C" {
    pub fn ssb_sdio_exit(bus: *mut ssb_bus);
}
extern "C" {
    pub fn ssb_sdio_init(bus: *mut ssb_bus) -> c_int;
}

//
// host_soc.c
//

// scan.c
extern "C" {
    pub fn ssb_iounmap(ssb: *mut ssb_bus);
}
// sprom.c
// core.c
extern "C" {
    pub fn ssb_calc_clock_rate(plltype: u32, n: u32, m: u32) -> u32;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssb_freeze_context {
// Pointer to the bus
    pub bus: *mut ssb_bus,
// Boolean list to indicate whether a device is frozen on this bus.
    pub device_frozen: [bool; SSB_MAX_NR_CORES],
}

extern "C" {
    pub fn ssb_devices_freeze(bus: *mut ssb_bus, ctx: *mut ssb_freeze_context) -> c_int;
}
extern "C" {
    pub fn ssb_devices_thaw(ctx: *mut ssb_freeze_context) -> c_int;
}
// b43_pci_bridge.c

extern "C" {
    pub fn b43_pci_ssb_bridge_init() -> int __init;
}
extern "C" {
    pub fn b43_pci_ssb_bridge_exit() -> void __exit;
}

// driver_chipcommon_pmu.c
extern "C" {
    pub fn ssb_pmu_get_cpu_clock(cc: *mut ssb_chipcommon) -> u32;
}
extern "C" {
    pub fn ssb_pmu_get_controlclock(cc: *mut ssb_chipcommon) -> u32;
}
extern "C" {
    pub fn ssb_pmu_get_alp_clock(cc: *mut ssb_chipcommon) -> u32;
}
extern "C" {
    pub fn ssb_chipco_watchdog_timer_set_ms(wdt: *mut bcm47xx_wdt, ms: u32) -> u32;
}
// driver_chipcommon_sflash.c

extern "C" {
    pub fn ssb_sflash_init(cc: *mut ssb_chipcommon) -> c_int;
}

extern "C" {
    pub fn ssb_extif_watchdog_timer_set_wdt(wdt: *mut bcm47xx_wdt, ticks: u32) -> u32;
}
extern "C" {
    pub fn ssb_extif_watchdog_timer_set_ms(wdt: *mut bcm47xx_wdt, ms: u32) -> u32;
}

extern "C" {
    pub fn ssb_watchdog_register(bus: *mut ssb_bus) -> c_int;
}

extern "C" {
    pub fn ssb_extif_init(extif: *mut ssb_extif);
}

extern "C" {
    pub fn ssb_gpio_init(bus: *mut ssb_bus) -> c_int;
}
extern "C" {
    pub fn ssb_gpio_unregister(bus: *mut ssb_bus) -> c_int;
}

