//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/apic/apic_numachip.c
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
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file "COPYING" in the main directory of this archive
// for more details.
//
// Numascale NumaConnect-Specific APIC Code
//
// Copyright (C) 2011 Numascale AS. All rights reserved.
//
// Send feedback to <support@numascale.com>
//

    u8 numachip_system __read_mostly;
    static const struct apic apic_numachip1;
    static const struct apic apic_numachip2;
    static void (*numachip_apic_icr_write)(int apicid, unsigned int val) __read_mostly;
#[no_mangle]
unsafe extern "C" fn numachip1_get_apic_id(x: u32) -> u32 {
    static u32 numachip1_get_apic_id(u32 x)
    {
    unsigned long value;
    let mut id: c_uint = (x >> 24) & 0xff;
    if (cpu_feature_enabled(X86_FEATURE_NODEID_MSR)) {
    rdmsrq(MSR_FAM10H_NODE_ID, value);
    id |= (value << 2) & 0xff00;
    }
    return id;
    }
#[no_mangle]
unsafe extern "C" fn numachip2_get_apic_id(x: u32) -> u32 {
    static u32 numachip2_get_apic_id(u32 x)
    {
    u64 mcfg;
    rdmsrq(MSR_FAM10H_MMIO_CONF_BASE, mcfg);
    return ((mcfg >> (28 - 8)) & 0xfff00) | (x >> 24);
    }
#[no_mangle]
unsafe extern "C" fn numachip1_apic_icr_write(apicid: c_int, val: c_uint) {
    static void numachip1_apic_icr_write(int apicid, unsigned int val)
    {
    write_lcsr(CSR_G3_EXT_IRQ_GEN, (apicid << 16) | val);
    }
#[no_mangle]
unsafe extern "C" fn numachip2_apic_icr_write(apicid: c_int, val: c_uint) {
    static void numachip2_apic_icr_write(int apicid, unsigned int val)
    {
    numachip2_write32_lcsr(NUMACHIP2_APIC_ICR, (apicid << 12) | val);
    }
#[no_mangle]
unsafe extern "C" fn numachip_wakeup_secondary(phys_apicid: u32, start_rip: c_ulong, cpu: c_uint) -> c_int {
    static int numachip_wakeup_secondary(u32 phys_apicid, unsigned long start_rip, unsigned int cpu)
    {
    numachip_apic_icr_write(phys_apicid, APIC_DM_INIT);
    numachip_apic_icr_write(phys_apicid, APIC_DM_STARTUP |
    (start_rip >> 12));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn numachip_send_IPI_one(cpu: c_int, vector: c_int) {
    static void numachip_send_IPI_one(int cpu, int vector)
    {
    int local_apicid, apicid = per_cpu(x86_cpu_to_apicid, cpu);
    unsigned int dmode;
    preempt_disable();
    local_apicid = __this_cpu_read(x86_cpu_to_apicid);
// Send via local APIC where non-local part matches
    if (!((apicid ^ local_apicid) >> NUMACHIP_LAPIC_BITS)) {
    unsigned long flags;
    local_irq_save(flags);
    __default_send_IPI_dest_field(apicid, vector,
    APIC_DEST_PHYSICAL);
    local_irq_restore(flags);
    preempt_enable();
    return;
    }
    preempt_enable();
    dmode = (vector == NMI_VECTOR) ? APIC_DM_NMI : APIC_DM_FIXED;
    numachip_apic_icr_write(apicid, dmode | vector);
    }
#[no_mangle]
unsafe extern "C" fn numachip_send_IPI_mask(mask: *const cpumask, vector: c_int) {
    static void numachip_send_IPI_mask(const struct cpumask *mask, int vector)
    {
    unsigned int cpu;
    for_each_cpu(cpu, mask)
    numachip_send_IPI_one(cpu, vector);
    }
    static void numachip_send_IPI_mask_allbutself(const struct cpumask *mask,
    int vector)
    {
    let mut this_cpu: c_uint = smp_processor_id();
    unsigned int cpu;
    for_each_cpu(cpu, mask) {
    if (cpu != this_cpu)
    numachip_send_IPI_one(cpu, vector);
    }
    }
#[no_mangle]
unsafe extern "C" fn numachip_send_IPI_allbutself(vector: c_int) {
    static void numachip_send_IPI_allbutself(int vector)
    {
    let mut this_cpu: c_uint = smp_processor_id();
    unsigned int cpu;
    for_each_online_cpu(cpu) {
    if (cpu != this_cpu)
    numachip_send_IPI_one(cpu, vector);
    }
    }
#[no_mangle]
unsafe extern "C" fn numachip_send_IPI_all(vector: c_int) {
    static void numachip_send_IPI_all(int vector)
    {
    numachip_send_IPI_mask(cpu_online_mask, vector);
    }
#[no_mangle]
unsafe extern "C" fn numachip_send_IPI_self(vector: c_int) {
    static void numachip_send_IPI_self(int vector)
    {
    apic_write(APIC_SELF_IPI, vector);
    }
#[no_mangle]
unsafe extern "C" fn numachip1_probe() -> int __init {
    static int __init numachip1_probe(void)
    {
    let mut apic: return = = &apic_numachip1;
    }
#[no_mangle]
unsafe extern "C" fn numachip2_probe() -> int __init {
    static int __init numachip2_probe(void)
    {
    let mut apic: return = = &apic_numachip2;
    }
#[no_mangle]
unsafe extern "C" fn fixup_cpu_id(c: *mut cpuinfo_x86, node: c_int) {
    static void fixup_cpu_id(struct cpuinfo_x86 *c, int node)
    {
    u64 val;
    let mut nodes: u32 = 1;
    c.topo.llc_id = node;
// Account for nodes per socket in multi-core-module processors
    if (boot_cpu_has(X86_FEATURE_NODEID_MSR)) {
    rdmsrq(MSR_FAM10H_NODE_ID, val);
    nodes = ((val >> 3) & 7) + 1;
    }
    c.topo.pkg_id = node / nodes;
    }
#[no_mangle]
unsafe extern "C" fn numachip_system_init() -> int __init {
    static int __init numachip_system_init(void)
    {
// Map the LCSR area and set up the apic_icr_write function
    switch (numachip_system) {
    case 1:
    init_extra_mapping_uc(NUMACHIP_LCSR_BASE, NUMACHIP_LCSR_SIZE);
    numachip_apic_icr_write = numachip1_apic_icr_write;
    break;
    case 2:
    init_extra_mapping_uc(NUMACHIP2_LCSR_BASE, NUMACHIP2_LCSR_SIZE);
    numachip_apic_icr_write = numachip2_apic_icr_write;
    break;
    default:
    return 0;
    }
    x86_cpuinit.fixup_cpu_id = fixup_cpu_id;
    x86_init.pci.arch_init = pci_numachip_init;
    return 0;
    }
    early_initcall(numachip_system_init);
#[no_mangle]
unsafe extern "C" fn numachip1_acpi_madt_oem_check(oem_id: *mut c_char, oem_table_id: *mut c_char) -> c_int {
    static int numachip1_acpi_madt_oem_check(char *oem_id, char *oem_table_id)
    {
    if ((strncmp(oem_id, "NUMASC", 6) != 0) ||
    (strncmp(oem_table_id, "NCONNECT", 8) != 0))
    return 0;
    numachip_system = 1;
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn numachip2_acpi_madt_oem_check(oem_id: *mut c_char, oem_table_id: *mut c_char) -> c_int {
    static int numachip2_acpi_madt_oem_check(char *oem_id, char *oem_table_id)
    {
    if ((strncmp(oem_id, "NUMASC", 6) != 0) ||
    (strncmp(oem_table_id, "NCONECT2", 8) != 0))
    return 0;
    numachip_system = 2;
    return 1;
    }
    static const struct apic apic_numachip1 __refconst = {
    .name				= "NumaConnect system",
    .probe				= numachip1_probe,
    .acpi_madt_oem_check		= numachip1_acpi_madt_oem_check,
    .dest_mode_logical		= false,
    .disable_esr			= 0,
    .cpu_present_to_apicid		= default_cpu_present_to_apicid,
    .max_apic_id			= UINT_MAX,
    .get_apic_id			= numachip1_get_apic_id,
    .calc_dest_apicid		= apic_default_calc_apicid,
    .send_IPI			= numachip_send_IPI_one,
    .send_IPI_mask			= numachip_send_IPI_mask,
    .send_IPI_mask_allbutself	= numachip_send_IPI_mask_allbutself,
    .send_IPI_allbutself		= numachip_send_IPI_allbutself,
    .send_IPI_all			= numachip_send_IPI_all,
    .send_IPI_self			= numachip_send_IPI_self,
    .wakeup_secondary_cpu		= numachip_wakeup_secondary,
    .read				= native_apic_mem_read,
    .write				= native_apic_mem_write,
    .eoi				= native_apic_mem_eoi,
    .icr_read			= native_apic_icr_read,
    .icr_write			= native_apic_icr_write,
    };
    apic_driver(apic_numachip1);
    static const struct apic apic_numachip2 __refconst = {
    .name				= "NumaConnect2 system",
    .probe				= numachip2_probe,
    .acpi_madt_oem_check		= numachip2_acpi_madt_oem_check,
    .dest_mode_logical		= false,
    .disable_esr			= 0,
    .cpu_present_to_apicid		= default_cpu_present_to_apicid,
    .max_apic_id			= UINT_MAX,
    .get_apic_id			= numachip2_get_apic_id,
    .calc_dest_apicid		= apic_default_calc_apicid,
    .send_IPI			= numachip_send_IPI_one,
    .send_IPI_mask			= numachip_send_IPI_mask,
    .send_IPI_mask_allbutself	= numachip_send_IPI_mask_allbutself,
    .send_IPI_allbutself		= numachip_send_IPI_allbutself,
    .send_IPI_all			= numachip_send_IPI_all,
    .send_IPI_self			= numachip_send_IPI_self,
    .wakeup_secondary_cpu		= numachip_wakeup_secondary,
    .read				= native_apic_mem_read,
    .write				= native_apic_mem_write,
    .eoi				= native_apic_mem_eoi,
    .icr_read			= native_apic_icr_read,
    .icr_write			= native_apic_icr_write,
    };
    apic_driver(apic_numachip2);
