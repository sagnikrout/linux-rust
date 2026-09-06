//! Automatically rewritten from C to Rust
//! Source: arch/x86/platform/intel-mid/intel-mid.c
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
// Intel MID platform setup code
//
// (C) Copyright 2008, 2012, 2021 Intel Corporation
// Author: Jacob Pan (jacob.jun.pan@intel.com)
// Author: Sathyanarayanan Kuppuswamy <sathyanarayanan.kuppuswamy@intel.com>
//

pub const IPCMSG_COLD_OFF: c_uint = 0x80	/* Only for Tangier */;
pub const IPCMSG_COLD_RESET: c_uint = 0xF1;
#[no_mangle]
unsafe extern "C" fn intel_mid_power_off() {
    static void intel_mid_power_off(void)
    {
// Shut down South Complex via PWRMU
    intel_mid_pwr_power_off();
// Only for Tangier, the rest will ignore this command
    intel_scu_ipc_dev_simple_command(core::ptr::null_mut(), IPCMSG_COLD_OFF, 1);
    };
#[no_mangle]
unsafe extern "C" fn intel_mid_reboot() {
    static void intel_mid_reboot(void)
    {
    intel_scu_ipc_dev_simple_command(core::ptr::null_mut(), IPCMSG_COLD_RESET, 0);
    }
#[no_mangle]
unsafe extern "C" fn intel_mid_time_init() -> void __init {
    static void __init intel_mid_time_init(void)
    {
// Lapic only, no apbt
    x86_init.timers.setup_percpu_clockev = setup_boot_APIC_clock;
    x86_cpuinit.setup_percpu_clockev = setup_secondary_APIC_clock;
    }
#[no_mangle]
unsafe extern "C" fn intel_mid_arch_setup() {
    static void intel_mid_arch_setup(void)
    {
    switch (boot_cpu_data.x86_vfm) {
    case INTEL_ATOM_SILVERMONT_MID:
    x86_platform.legacy.rtc = 1;
    break;
    default:
    break;
    }
//
// Intel MID platforms are using explicitly defined regulators.
//
// Let the regulator core know that we do not have any additional
// regulators left. This lets it substitute unprovided regulators with
// dummy ones:
//
    regulator_has_full_constraints();
    }
//
// Moorestown does not have external NMI source nor port 0x61 to report
// NMI status. The possible NMI sources are from pmu as a result of NMI
// watchdog or lock debug. Reading io port 0x61 results in 0xff which
// misled NMI handler.
//
#[no_mangle]
unsafe extern "C" fn intel_mid_get_nmi_reason() -> c_uchar {
    static unsigned char intel_mid_get_nmi_reason(void)
    {
    return 0;
    }
//
// Moorestown specific x86_init function overrides and early setup
// calls.
//
#[no_mangle]
pub unsafe extern "C" fn x86_intel_mid_early_setup() -> void __init {
    void __init x86_intel_mid_early_setup(void)
    {
    x86_init.resources.probe_roms = x86_init_noop;
    x86_init.resources.reserve_resources = x86_init_noop;
    x86_init.timers.timer_init = intel_mid_time_init;
    x86_init.timers.setup_percpu_clockev = x86_init_noop;
    x86_init.irqs.pre_vector_init = x86_init_noop;
    x86_init.oem.arch_setup = intel_mid_arch_setup;
    x86_platform.get_nmi_reason = intel_mid_get_nmi_reason;
    x86_init.pci.arch_init = intel_mid_pci_init;
    x86_init.pci.fixup_irqs = x86_init_noop;
    legacy_pic = &null_legacy_pic;
//
// Do nothing for now as everything needed done in
// x86_intel_mid_early_setup() below.
//
    x86_init.acpi.reduced_hw_early_init = x86_init_noop;
    pm_power_off = intel_mid_power_off;
    machine_ops.emergency_restart  = intel_mid_reboot;
// Avoid searching for BIOS MP tables
    x86_init.mpparse.find_mptable		= x86_init_noop;
    x86_init.mpparse.early_parse_smp_cfg	= x86_init_noop;
    x86_init.mpparse.parse_smp_cfg		= x86_init_noop;
    set_bit(MP_BUS_ISA, mp_bus_not_pci);
    }
