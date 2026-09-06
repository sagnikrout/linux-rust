//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/bcma/bcma_private.h
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

// main.c
extern "C" {
    pub fn bcma_prepare_core(bus: *mut bcma_bus, core: *mut bcma_device);
}
extern "C" {
    pub fn bcma_init_bus(bus: *mut bcma_bus);
}
extern "C" {
    pub fn bcma_unregister_cores(bus: *mut bcma_bus);
}
extern "C" {
    pub fn bcma_bus_register(bus: *mut bcma_bus) -> c_int;
}
extern "C" {
    pub fn bcma_bus_unregister(bus: *mut bcma_bus);
}
extern "C" {
    pub fn bcma_bus_early_register(bus: *mut bcma_bus) -> int __init;
}

extern "C" {
    pub fn bcma_bus_suspend(bus: *mut bcma_bus) -> c_int;
}
extern "C" {
    pub fn bcma_bus_resume(bus: *mut bcma_bus) -> c_int;
}

// scan.c
extern "C" {
    pub fn bcma_detect_chip(bus: *mut bcma_bus);
}
extern "C" {
    pub fn bcma_bus_scan(bus: *mut bcma_bus) -> c_int;
}
// sprom.c
extern "C" {
    pub fn bcma_sprom_get(bus: *mut bcma_bus) -> c_int;
}
// driver_chipcommon.c
extern "C" {
    pub fn bcma_core_chipcommon_early_init(cc: *mut bcma_drv_cc);
}
extern "C" {
    pub fn bcma_core_chipcommon_init(cc: *mut bcma_drv_cc);
}
extern "C" {
    pub fn bcma_chipco_bcm4331_ext_pa_lines_ctl(cc: *mut bcma_drv_cc, enable: bool);
}

extern "C" {
    pub fn bcma_chipco_serial_init(cc: *mut bcma_drv_cc);
}

// driver_chipcommon_b.c
extern "C" {
    pub fn bcma_core_chipcommon_b_init(ccb: *mut bcma_drv_cc_b) -> c_int;
}
extern "C" {
    pub fn bcma_core_chipcommon_b_free(ccb: *mut bcma_drv_cc_b);
}
// driver_chipcommon_pmu.c
extern "C" {
    pub fn bcma_pmu_early_init(cc: *mut bcma_drv_cc);
}
extern "C" {
    pub fn bcma_pmu_init(cc: *mut bcma_drv_cc);
}
extern "C" {
    pub fn bcma_pmu_get_alp_clock(cc: *mut bcma_drv_cc) -> u32;
}
extern "C" {
    pub fn bcma_pmu_get_cpu_clock(cc: *mut bcma_drv_cc) -> u32;
}
//
// driver_chipcommon_sflash.c
//

extern "C" {
    pub fn bcma_pflash_init(cc: *mut bcma_drv_cc) -> c_int;
}

// driver_chipcommon_sflash.c
extern "C" {
    pub fn bcma_sflash_init(cc: *mut bcma_drv_cc) -> c_int;
}

// driver_chipcommon_nflash.c
extern "C" {
    pub fn bcma_nflash_init(cc: *mut bcma_drv_cc) -> c_int;
}

// host_pci.c
extern "C" {
    pub fn bcma_host_pci_init() -> int __init;
}
extern "C" {
    pub fn bcma_host_pci_exit() -> void __exit;
}

// host_soc.c

extern "C" {
    pub fn bcma_host_soc_register_driver() -> int __init;
}
extern "C" {
    pub fn bcma_host_soc_unregister_driver() -> void __exit;
}

// driver_pci.c

extern "C" {
    pub fn bcma_pcie_read(pc: *mut bcma_drv_pci, address: u32) -> u32;
}
extern "C" {
    pub fn bcma_core_pci_early_init(pc: *mut bcma_drv_pci);
}
extern "C" {
    pub fn bcma_core_pci_init(pc: *mut bcma_drv_pci);
}
extern "C" {
    pub fn bcma_core_pci_up(pc: *mut bcma_drv_pci);
}
extern "C" {
    pub fn bcma_core_pci_down(pc: *mut bcma_drv_pci);
}

// Initialization is required for PCI hosted bus

// driver_pcie2.c

extern "C" {
    pub fn bcma_core_pcie2_init(pcie2: *mut bcma_drv_pcie2);
}
extern "C" {
    pub fn bcma_core_pcie2_up(pcie2: *mut bcma_drv_pcie2);
}

// Initialization is required for PCI hosted bus

extern "C" {
    pub fn bcma_chipco_watchdog_register(cc: *mut bcma_drv_cc) -> c_int;
}

extern "C" {
    pub fn bcma_core_pci_is_in_hostmode(pc: *mut bcma_drv_pci) -> bool;
}
extern "C" {
    pub fn bcma_core_pci_hostmode_init(pc: *mut bcma_drv_pci);
}

//
// driver_mips.c
//

extern "C" {
    pub fn bcma_core_mips_irq(dev: *mut bcma_device) -> c_uint;
}
extern "C" {
    pub fn bcma_core_mips_early_init(mcore: *mut bcma_drv_mips);
}
extern "C" {
    pub fn bcma_core_mips_init(mcore: *mut bcma_drv_mips);
}

//
// driver_gmac_cmn.c
//

extern "C" {
    pub fn bcma_core_gmac_cmn_init(gc: *mut bcma_drv_gmac_cmn);
}

// driver_gpio.c
extern "C" {
    pub fn bcma_gpio_init(cc: *mut bcma_drv_cc) -> c_int;
}
extern "C" {
    pub fn bcma_gpio_unregister(cc: *mut bcma_drv_cc) -> c_int;
}

