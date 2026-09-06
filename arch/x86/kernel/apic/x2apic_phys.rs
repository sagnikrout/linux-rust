//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/apic/x2apic_phys.c
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

    int x2apic_phys;
    static struct apic apic_x2apic_phys;
    let mut __ro_after_init: u32 x2apic_max_apicid = UINT_MAX;
#[no_mangle]
pub unsafe extern "C" fn x2apic_set_max_apicid(apicid: u32) -> void __init {
    void __init x2apic_set_max_apicid(u32 apicid)
    {
    x2apic_max_apicid = apicid;
    if (apic.x2apic_set_max_apicid)
    apic.max_apic_id = apicid;
    }
#[no_mangle]
unsafe extern "C" fn set_x2apic_phys_mode(arg: *mut c_char) -> int __init {
    static int __init set_x2apic_phys_mode(char *arg)
    {
    x2apic_phys = 1;
    return 0;
    }
    early_param("x2apic_phys", set_x2apic_phys_mode);
#[no_mangle]
unsafe extern "C" fn x2apic_fadt_phys() -> bool {
    static bool x2apic_fadt_phys(void)
    {

    if ((acpi_gbl_FADT.header.revision >= FADT2_REVISION_ID) &&
    (acpi_gbl_FADT.flags & ACPI_FADT_APIC_PHYSICAL)) {
    printk(KERN_DEBUG "System requires x2apic physical mode\n");
    return true;
    }

    return false;
    }
#[no_mangle]
unsafe extern "C" fn x2apic_acpi_madt_oem_check(oem_id: *mut c_char, oem_table_id: *mut c_char) -> c_int {
    static int x2apic_acpi_madt_oem_check(char *oem_id, char *oem_table_id)
    {
    return x2apic_enabled() && (x2apic_phys || x2apic_fadt_phys());
    }
#[no_mangle]
unsafe extern "C" fn x2apic_send_IPI(cpu: c_int, vector: c_int) {
    static void x2apic_send_IPI(int cpu, int vector)
    {
    let mut dest: u32 = per_cpu(x86_cpu_to_apicid, cpu);
// x2apic MSRs are special and need a special fence:
    weak_wrmsr_fence();
    __x2apic_send_IPI_dest(dest, vector, APIC_DEST_PHYSICAL);
    }
    static void
    __x2apic_send_IPI_mask(const struct cpumask *mask, int vector, int apic_dest)
    {
    unsigned long query_cpu;
    unsigned long this_cpu;
    unsigned long flags;
// x2apic MSRs are special and need a special fence:
    weak_wrmsr_fence();
    local_irq_save(flags);
    this_cpu = smp_processor_id();
    for_each_cpu(query_cpu, mask) {
    if (apic_dest == APIC_DEST_ALLBUT && this_cpu == query_cpu)
    continue;
    __x2apic_send_IPI_dest(per_cpu(x86_cpu_to_apicid, query_cpu),
    vector, APIC_DEST_PHYSICAL);
    }
    local_irq_restore(flags);
    }
#[no_mangle]
unsafe extern "C" fn x2apic_send_IPI_mask(mask: *const cpumask, vector: c_int) {
    static void x2apic_send_IPI_mask(const struct cpumask *mask, int vector)
    {
    __x2apic_send_IPI_mask(mask, vector, APIC_DEST_ALLINC);
    }
    static void
    x2apic_send_IPI_mask_allbutself(const struct cpumask *mask, int vector)
    {
    __x2apic_send_IPI_mask(mask, vector, APIC_DEST_ALLBUT);
    }
#[no_mangle]
unsafe extern "C" fn __x2apic_send_IPI_shorthand(vector: c_int, which: u32) {
    static void __x2apic_send_IPI_shorthand(int vector, u32 which)
    {
// x2apic MSRs are special and need a special fence:
    weak_wrmsr_fence();
    native_x2apic_icr_write(__prepare_ICR(which, vector, 0), 0);
    }
#[no_mangle]
pub unsafe extern "C" fn x2apic_send_IPI_allbutself(vector: c_int) {
    void x2apic_send_IPI_allbutself(int vector)
    {
    __x2apic_send_IPI_shorthand(vector, APIC_DEST_ALLBUT);
    }
#[no_mangle]
pub unsafe extern "C" fn x2apic_send_IPI_all(vector: c_int) {
    void x2apic_send_IPI_all(int vector)
    {
    __x2apic_send_IPI_shorthand(vector, APIC_DEST_ALLINC);
    }
#[no_mangle]
pub unsafe extern "C" fn x2apic_send_IPI_self(vector: c_int) {
    void x2apic_send_IPI_self(int vector)
    {
    apic_write(APIC_SELF_IPI, vector);
    }
#[no_mangle]
unsafe extern "C" fn x2apic_phys_probe() -> c_int {
    static int x2apic_phys_probe(void)
    {
    if (!x2apic_mode)
    return 0;
    if (x2apic_phys || x2apic_fadt_phys())
    return 1;
    let mut apic: return = = &apic_x2apic_phys;
    }
#[no_mangle]
pub unsafe extern "C" fn x2apic_get_apic_id(id: u32) -> u32 {
    u32 x2apic_get_apic_id(u32 id)
    {
    return id;
    }
    static struct apic apic_x2apic_phys __ro_after_init = {
    .name				= "physical x2apic",
    .probe				= x2apic_phys_probe,
    .acpi_madt_oem_check		= x2apic_acpi_madt_oem_check,
    .dest_mode_logical		= false,
    .disable_esr			= 0,
    .cpu_present_to_apicid		= default_cpu_present_to_apicid,
    .max_apic_id			= UINT_MAX,
    .x2apic_set_max_apicid		= true,
    .get_apic_id			= x2apic_get_apic_id,
    .calc_dest_apicid		= apic_default_calc_apicid,
    .send_IPI			= x2apic_send_IPI,
    .send_IPI_mask			= x2apic_send_IPI_mask,
    .send_IPI_mask_allbutself	= x2apic_send_IPI_mask_allbutself,
    .send_IPI_allbutself		= x2apic_send_IPI_allbutself,
    .send_IPI_all			= x2apic_send_IPI_all,
    .send_IPI_self			= x2apic_send_IPI_self,
    .nmi_to_offline_cpu		= true,
    .read				= native_apic_msr_read,
    .write				= native_apic_msr_write,
    .eoi				= native_apic_msr_eoi,
    .icr_read			= native_x2apic_icr_read,
    .icr_write			= native_x2apic_icr_write,
    };
    apic_driver(apic_x2apic_phys);
