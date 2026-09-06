//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/apic.h
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

pub const ARCH_APICTIMER_STOPS_ON_C3: c_int = 1;
// Macros for apic_extnmi which controls external NMI masking

pub const APIC_EXTNMI_ALL: c_int = 1;
pub const APIC_EXTNMI_NONE: c_int = 2;
//
// Debugging macros
//
pub const APIC_QUIET: c_int = 0;
pub const APIC_VERBOSE: c_int = 1;
pub const APIC_DEBUG: c_int = 2;
//
// Define the default level of output to be very little This can be turned
// up by using apic=verbose for more information and apic=debug for _lots_
// of information.  apic_verbosity is defined in apic.c
//

// Unconditional debug prints for code which is guarded by apic_verbosity already

extern "C" {
    pub fn x86_32_probe_apic();
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum apic_intr_mode_id {
    APIC_PIC,
    APIC_VIRTUAL_WIRE,
    APIC_VIRTUAL_WIRE_NO_CONFIG,
    APIC_SYMMETRIC_IO,
    APIC_SYMMETRIC_IO_NO_ROUTING
}

//
// With 82489DX we can't rely on apic feature bit
// retrieved via cpuid but still have to deal with
// such an apic chip so we assume that SMP configuration
// is found from MP table (64bit case uses ACPI mostly
// which set smp presence flag as well so we are safe
// to use this helper too).
//
// Basic functions accessing APICs.
//
extern "C" {
    pub fn readl(reg): *mut *mut (void __iomem )(APIC_BASE +) -> return;
}
extern "C" {
    pub fn native_apic_icr_write(low: u32, id: u32);
}
extern "C" {
    pub fn native_apic_icr_read() -> u64;
}
extern "C" {
    pub fn enable_IR_x2apic();
}
extern "C" {
    pub fn lapic_get_maxlvt() -> c_int;
}
extern "C" {
    pub fn clear_local_APIC();
}
extern "C" {
    pub fn disconnect_bsp_APIC(virt_wire_setup: c_int);
}
extern "C" {
    pub fn disable_local_APIC();
}
extern "C" {
    pub fn apic_soft_disable();
}
extern "C" {
    pub fn lapic_shutdown();
}
extern "C" {
    pub fn sync_Arb_IDs();
}
extern "C" {
    pub fn init_bsp_APIC();
}
extern "C" {
    pub fn apic_intr_mode_select();
}
extern "C" {
    pub fn apic_intr_mode_init();
}
extern "C" {
    pub fn init_apic_mappings();
}
extern "C" {
    pub fn register_lapic_address(address: c_ulong);
}
extern "C" {
    pub fn setup_boot_APIC_clock();
}
extern "C" {
    pub fn setup_secondary_APIC_clock();
}
extern "C" {
    pub fn lapic_update_tsc_freq();
}

extern "C" {
    pub fn apic_force_enable(addr: c_ulong) -> bool;
}

extern "C" {
    pub fn apic_ap_setup();
}
//
// On 32bit this is mach-xxx local
//

extern "C" {
    pub fn apic_is_clustered_box() -> c_int;
}

extern "C" {
    pub fn setup_APIC_eilvt(lvt_off: u8, vector: u8, msg_type: u8, mask: u8) -> c_int;
}
extern "C" {
    pub fn lapic_assign_system_vectors();
}
extern "C" {
    pub fn lapic_assign_legacy_vector(isairq: c_uint, replace: bool);
}
extern "C" {
    pub fn lapic_update_legacy_vectors();
}
extern "C" {
    pub fn lapic_online();
}
extern "C" {
    pub fn lapic_offline();
}
extern "C" {
    pub fn apic_needs_pit() -> bool;
}
extern "C" {
    pub fn apic_send_IPI_allbutself(vector: c_uint);
}
extern "C" {
    pub fn topology_register_apic(apic_id: u32, acpi_id: u32, present: bool);
}
extern "C" {
    pub fn topology_register_boot_apic(apic_id: u32);
}
extern "C" {
    pub fn topology_hotplug_apic(apic_id: u32, acpi_id: u32) -> c_int;
}
extern "C" {
    pub fn topology_hotunplug_apic(cpu: c_uint);
}
extern "C" {
    pub fn topology_apply_cmdline_limits_early();
}
extern "C" {
    pub fn topology_init_possible_cpus();
}
extern "C" {
    pub fn topology_reset_possible_cpus_up();
}

pub const local_apic_timer_c2_ok: c_int = 1;

extern "C" {
    pub fn x2apic_set_max_apicid(apicid: u32) -> void __init;
}
extern "C" {
    pub fn x2apic_setup();
}
extern "C" {
    pub fn boot_cpu_has(apic_is_x2apic_enabled(: X86_FEATURE_X2APIC) &&) -> return;
}

extern "C" {
    pub fn check_x2apic() -> void __init;
}
//
// Copyright 2004 James Cleverdon, IBM.
//
// Generic APIC sub-arch data struct.
//
// Hacked for x86-64 by James Cleverdon from i386 architecture code by
// Martin Bligh, Andi Kleen, James Bottomley, John Stultz, and
// James Cleverdon.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apic {
// Hotpath functions first
    pub (*eoi)(void): *mut c_void,
    pub (*native_eoi)(void): *mut c_void,
    pub v): *mut *mut void (write)(u32 reg, u32,
    pub reg): *mut *mut u32 (read)(u32,
// IPI related functions
    pub (*wait_icr_idle)(void): *mut c_void,
    pub (*safe_wait_icr_idle)(void): *mut u32,
    pub vector): *mut *mut void (send_IPI)(int cpu, int,
    pub vector): *const *const *const void (send_IPI_mask)(struct cpumask mask, int,
    pub vec): *const *const *const void (send_IPI_mask_allbutself)(struct cpumask msk, int,
    pub vector): *mut *mut void (send_IPI_allbutself)(int,
    pub vector): *mut *mut void (send_IPI_all)(int,
    pub vector): *mut *mut void (send_IPI_self)(int,
    pub 1: nmi_to_offline_cpu :,
    pub cpu): *mut *mut u32 (calc_dest_apicid)(unsigned int,
// ICR related functions
    pub (*icr_read)(void): *mut u64,
    pub high): *mut *mut void (icr_write)(u32 low, u32,
// The limit of the APIC ID space.
    pub max_apic_id: u32,
// Probe, setup and smpboot functions
    pub (*probe)(void): *mut c_int,
    pub (*setup)(void): *mut c_void,
    pub (*teardown)(void): *mut c_void,
    pub oem_table_id): *mut *mut *mut int (acpi_madt_oem_check)(char oem_id, char,
    pub (*init_apic_ldr)(void): *mut c_void,
    pub mps_cpu): *mut *mut u32 (cpu_present_to_apicid)(int,
    pub id): *mut *mut u32 (get_apic_id)(u32,
// wakeup_secondary_cpu
    pub cpu): *mut *mut int (wakeup_secondary_cpu)(u32 apicid, unsigned long start_eip, unsigned int,
// wakeup secondary CPU using 64-bit wakeup point
    pub cpu): *mut *mut int (wakeup_secondary_cpu_64)(u32 apicid, unsigned long start_eip, unsigned int,
    pub set): *mut *mut void (update_vector)(unsigned int cpu, unsigned int vector, bool,
    pub name: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct apic_override {
    pub (*eoi)(void): *mut c_void,
    pub (*native_eoi)(void): *mut c_void,
    pub v): *mut *mut void (write)(u32 reg, u32,
    pub reg): *mut *mut u32 (read)(u32,
    pub vector): *mut *mut void (send_IPI)(int cpu, int,
    pub vector): *const *const *const void (send_IPI_mask)(struct cpumask mask, int,
    pub vec): *const *const *const void (send_IPI_mask_allbutself)(struct cpumask msk, int,
    pub vector): *mut *mut void (send_IPI_allbutself)(int,
    pub vector): *mut *mut void (send_IPI_all)(int,
    pub vector): *mut *mut void (send_IPI_self)(int,
    pub (*icr_read)(void): *mut u64,
    pub high): *mut *mut void (icr_write)(u32 low, u32,
    pub cpu): *mut *mut int (wakeup_secondary_cpu)(u32 apicid, unsigned long start_eip, unsigned int,
    pub cpu): *mut *mut int (wakeup_secondary_cpu_64)(u32 apicid, unsigned long start_eip, unsigned int,
}

//
// Pointer to the local APIC driver in use on this system (there's
// always just one such driver in use - the kernel decides via an
// early probing process which one it picks - and then sticks to it):
//
// APIC drivers are probed based on how they are listed in the .apicdrivers
// section. So the order is important and enforced by the ordering
// of different apic driver files in the Makefile.
//

//
// APIC functionality to boot other CPUs - only used on SMP:
//

extern "C" {
    pub fn lapic_can_unplug_cpu() -> c_int;
}

extern "C" {
    pub fn apic_setup_apic_calls() -> void __init;
}
extern "C" {
    pub fn apic_install_driver(driver: *mut apic) -> void __init;
}

extern "C" {
    pub fn static_call(_arg: apic_call_read)(reg) -> return;
}
extern "C" {
    pub fn static_call(_arg: apic_call_icr_read)() -> return;
}

extern "C" {
    pub fn apic_ack_irq(data: *mut irq_data);
}

extern "C" {
    pub fn lapic_vector_set_in_irr(pi_pending_this_cpu(vector: vector) ||) -> return;
}
pub const MAX_APIC_VECTOR: c_int = 256;
pub const APIC_VECTORS_PER_REG: c_int = 32;
//
// Vector states are maintained by APIC in 32-bit registers that are
// 16 bytes aligned. The status of each vector is kept in a single
// bit.
//
// ((u32 *) (regs + reg)) = val;
// ((u64 *) (regs + reg)) = val;
extern "C" {
    pub fn test_bit(_arg: APIC_VECTOR_TO_BIT_NUMBER(vec), APIC_VECTOR_TO_REG_OFFSET(vec): bitmap +) -> return;
}
//
// Warm reset vector position:
//
pub const TRAMPOLINE_PHYS_LOW: c_uint = 0x467;
pub const TRAMPOLINE_PHYS_HIGH: c_uint = 0x469;

extern "C" {
    pub fn int(apicid: *mut *mut wakeup_cpu_handler)(int, start_eip: c_ulong) -> typedef;
}
extern "C" {
    pub fn default_acpi_madt_oem_check(: *mut c_char, : *mut c_char) -> c_int;
}
extern "C" {
    pub fn x86_64_probe_apic();
}

extern "C" {
    pub fn apic_default_calc_apicid(cpu: c_uint) -> u32;
}
extern "C" {
    pub fn apic_flat_calc_apicid(cpu: c_uint) -> u32;
}
extern "C" {
    pub fn default_cpu_present_to_apicid(mps_cpu: c_int) -> u32;
}
extern "C" {
    pub fn apic_send_nmi_to_offline_cpu(cpu: c_uint);
}

extern "C" {
    pub fn apic_smt_update();
}

extern "C" {
    pub fn ioapic_zap_locks();
}
