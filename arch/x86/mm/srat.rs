//! Automatically rewritten from C to Rust
//! Source: arch/x86/mm/srat.c
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
// ACPI 3.0 based NUMA setup
// Copyright 2004 Andi Kleen, SuSE Labs.
//
// Reads the ACPI SRAT table to figure out what memory belongs to which CPUs.
//
// Called from acpi_numa_init while reading the SRAT and SLIT tables.
// Assumes all memory regions belonging to a single proximity domain
// are in one chunk. Holes between them will be included in the node.
//

// Callback for Proximity Domain -> x2APIC mapping
    void __init
    acpi_numa_x2apic_affinity_init(struct acpi_srat_x2apic_cpu_affinity *pa)
    {
    int pxm, node;
    int apic_id;
    if (srat_disabled())
    return;
    if (pa.header.length < sizeof(struct acpi_srat_x2apic_cpu_affinity)) {
    bad_srat();
    return;
    }
    if ((pa.flags & ACPI_SRAT_CPU_ENABLED) == 0)
    return;
    pxm = pa.proximity_domain;
    apic_id = pa.apic_id;
    if (!apic_id_valid(apic_id)) {
    pr_info("SRAT: PXM %u . X2APIC 0x%04x ignored\n", pxm, apic_id);
    return;
    }
    node = acpi_map_pxm_to_node(pxm);
    if (node < 0) {
    printk(KERN_ERR "SRAT: Too many proximity domains %x\n", pxm);
    bad_srat();
    return;
    }
    if (apic_id >= MAX_LOCAL_APIC) {
    printk(KERN_INFO "SRAT: PXM %u . APIC 0x%04x . Node %u skipped apicid that is too big\n", pxm, apic_id, node);
    return;
    }
    set_apicid_to_node(apic_id, node);
    node_set(node, numa_nodes_parsed);
    node_set(node, numa_phys_nodes_parsed);
    pr_debug("SRAT: PXM %u . APIC 0x%04x . Node %u\n", pxm, apic_id, node);
    }
// Callback for Proximity Domain -> LAPIC mapping
    void __init
    acpi_numa_processor_affinity_init(struct acpi_srat_cpu_affinity *pa)
    {
    int pxm, node;
    int apic_id;
    if (srat_disabled())
    return;
    if (pa.header.length != sizeof(struct acpi_srat_cpu_affinity)) {
    bad_srat();
    return;
    }
    if ((pa.flags & ACPI_SRAT_CPU_ENABLED) == 0)
    return;
    pxm = pa.proximity_domain_lo;
    if (acpi_srat_revision >= 2)
    pxm |= *((unsigned int*)pa.proximity_domain_hi) << 8;
    node = acpi_map_pxm_to_node(pxm);
    if (node < 0) {
    printk(KERN_ERR "SRAT: Too many proximity domains %x\n", pxm);
    bad_srat();
    return;
    }
    if (get_uv_system_type() >= UV_X2APIC)
    apic_id = (pa.apic_id << 8) | pa.local_sapic_eid;
    else
    apic_id = pa.apic_id;
    if (apic_id >= MAX_LOCAL_APIC) {
    printk(KERN_INFO "SRAT: PXM %u . APIC 0x%02x . Node %u skipped apicid that is too big\n", pxm, apic_id, node);
    return;
    }
    set_apicid_to_node(apic_id, node);
    node_set(node, numa_nodes_parsed);
    node_set(node, numa_phys_nodes_parsed);
    pr_debug("SRAT: PXM %u . APIC 0x%02x . Node %u\n", pxm, apic_id, node);
    }
#[no_mangle]
pub unsafe extern "C" fn x86_acpi_numa_init() -> int __init {
    int __init x86_acpi_numa_init(void)
    {
    int ret;
    ret = acpi_numa_init();
    if (ret < 0)
    return ret;
    return srat_disabled() ? -EINVAL : 0;
    }
