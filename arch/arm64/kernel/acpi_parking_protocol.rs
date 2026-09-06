//! Automatically rewritten from C to Rust
//! Source: arch/arm64/kernel/acpi_parking_protocol.c
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
// ARM64 ACPI Parking Protocol implementation
//
// Authors: Lorenzo Pieralisi <lorenzo.pieralisi@arm.com>
// Mark Salter <msalter@redhat.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct parking_protocol_mailbox {
    pub cpu_id: __le32,
    pub reserved: __le32,
    pub entry_point: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpu_mailbox_entry {
    pub mailbox: *mut parking_protocol_mailbox __iomem,
    pub mailbox_addr: phys_addr_t,
    pub version: u8,
    pub gic_cpu_id: u8,
}

    static struct cpu_mailbox_entry cpu_mailbox_entries[NR_CPUS];
    void __init acpi_set_mailbox_entry(int cpu,
    struct acpi_madt_generic_interrupt *p)
    {
    struct cpu_mailbox_entry *cpu_entry = &cpu_mailbox_entries[cpu];
    cpu_entry.mailbox_addr = p.parked_address;
    cpu_entry.version = p.parking_version;
    cpu_entry.gic_cpu_id = p.cpu_interface_number;
    }
#[no_mangle]
pub unsafe extern "C" fn acpi_parking_protocol_valid(cpu: c_int) -> bool {
    bool acpi_parking_protocol_valid(int cpu)
    {
    struct cpu_mailbox_entry *cpu_entry = &cpu_mailbox_entries[cpu];
    return cpu_entry.mailbox_addr && cpu_entry.version;
    }
#[no_mangle]
unsafe extern "C" fn acpi_parking_protocol_cpu_init(cpu: c_uint) -> c_int {
    static int acpi_parking_protocol_cpu_init(unsigned int cpu)
    {
    pr_debug("%s: ACPI parked addr=%llx\n", __func__,
    cpu_mailbox_entries[cpu].mailbox_addr);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn acpi_parking_protocol_cpu_prepare(cpu: c_uint) -> c_int {
    static int acpi_parking_protocol_cpu_prepare(unsigned int cpu)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn acpi_parking_protocol_cpu_boot(cpu: c_uint) -> c_int {
    static int acpi_parking_protocol_cpu_boot(unsigned int cpu)
    {
    struct cpu_mailbox_entry *cpu_entry = &cpu_mailbox_entries[cpu];
    struct parking_protocol_mailbox __iomem *mailbox;
    u32 cpu_id;
//
// Map mailbox memory with attribute device nGnRE (ie ioremap -
// this deviates from the parking protocol specifications since
// the mailboxes are required to be mapped nGnRnE; the attribute
// discrepancy is harmless insofar as the protocol specification
// is concerned).
// If the mailbox is mistakenly allocated in the linear mapping
// by FW ioremap will fail since the mapping will be prevented
// by the kernel (it clashes with the linear mapping attributes
// specifications).
//
    mailbox = ioremap(cpu_entry.mailbox_addr, sizeof(*mailbox));
    if (!mailbox)
    return -EIO;
    cpu_id = readl_relaxed(&mailbox.cpu_id);
//
// Check if firmware has set-up the mailbox entry properly
// before kickstarting the respective cpu.
//
    if (cpu_id != ~0U) {
    iounmap(mailbox);
    return -ENXIO;
    }
//
// stash the mailbox address mapping to use it for further FW
// checks in the postboot method
//
    cpu_entry.mailbox = mailbox;
//
// We write the entry point and cpu id as LE regardless of the
// native endianness of the kernel. Therefore, any boot-loaders
// that read this address need to convert this address to the
// Boot-Loader's endianness before jumping.
//
    writeq_relaxed(__pa_symbol(secondary_entry),
    &mailbox.entry_point);
    writel_relaxed(cpu_entry.gic_cpu_id, &mailbox.cpu_id);
    arch_send_wakeup_ipi(cpu);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn acpi_parking_protocol_cpu_postboot() {
    static void acpi_parking_protocol_cpu_postboot(void)
    {
    let mut cpu: c_int = smp_processor_id();
    struct cpu_mailbox_entry *cpu_entry = &cpu_mailbox_entries[cpu];
    struct parking_protocol_mailbox __iomem *mailbox = cpu_entry.mailbox;
    u64 entry_point;
    entry_point = readq_relaxed(&mailbox.entry_point);
//
// Check if firmware has cleared the entry_point as expected
// by the protocol specification.
//
    WARN_ON(entry_point);
    }
    const struct cpu_operations acpi_parking_protocol_ops = {
    .name		= "parking-protocol",
    .cpu_init	= acpi_parking_protocol_cpu_init,
    .cpu_prepare	= acpi_parking_protocol_cpu_prepare,
    .cpu_boot	= acpi_parking_protocol_cpu_boot,
    .cpu_postboot	= acpi_parking_protocol_cpu_postboot
    };
