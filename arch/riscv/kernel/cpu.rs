//! Automatically rewritten from C to Rust
//! Source: arch/riscv/kernel/cpu.c
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
// Copyright (C) 2012 Regents of the University of California
//

#[no_mangle]
pub unsafe extern "C" fn arch_match_cpu_phys_id(cpu: c_int, phys_id: u64) -> bool {
    bool arch_match_cpu_phys_id(int cpu, u64 phys_id)
    {
    let mut phys_id: return = = cpuid_to_hartid_map(cpu);
    }
//
// Returns the hart ID of the given device tree node, or -ENODEV if the node
// isn't an enabled and valid RISC-V hart node.
//
#[no_mangle]
pub unsafe extern "C" fn riscv_of_processor_hartid(node: *mut device_node, hart: *mut c_ulong) -> c_int {
    int riscv_of_processor_hartid(struct device_node *node, unsigned long *hart)
    {
    int cpu;
// hart = (unsigned long)of_get_cpu_hwid(node, 0);
    if (*hart == ~0UL) {
    pr_warn("Found CPU without hart ID\n");
    return -ENODEV;
    }
    cpu = riscv_hartid_to_cpuid(*hart);
    if (cpu < 0)
    return cpu;
    if (!cpu_possible(cpu))
    return -ENODEV;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn riscv_early_of_processor_hartid(node: *mut device_node, hart: *mut c_ulong) -> int __init {
    int __init riscv_early_of_processor_hartid(struct device_node *node, unsigned long *hart)
    {
    const char *isa;
    if (!of_device_is_compatible(node, "riscv")) {
    pr_warn("Found incompatible CPU\n");
    return -ENODEV;
    }
// hart = (unsigned long)of_get_cpu_hwid(node, 0);
    if (*hart == ~0UL) {
    pr_warn("Found CPU without hart ID\n");
    return -ENODEV;
    }
    if (!of_device_is_available(node))
    return -ENODEV;
    if (of_property_read_string(node, "riscv,isa-base", &isa))
    goto old_interface;
    if (IS_ENABLED(CONFIG_32BIT) && strncasecmp(isa, "rv32i", 5)) {
    pr_warn("CPU with hartid=%lu does not support rv32i", *hart);
    return -ENODEV;
    }
    if (IS_ENABLED(CONFIG_64BIT) && strncasecmp(isa, "rv64i", 5)) {
    pr_warn("CPU with hartid=%lu does not support rv64i", *hart);
    return -ENODEV;
    }
    if (!of_property_present(node, "riscv,isa-extensions"))
    return -ENODEV;
    if (of_property_match_string(node, "riscv,isa-extensions", "i") < 0 ||
    of_property_match_string(node, "riscv,isa-extensions", "m") < 0 ||
    of_property_match_string(node, "riscv,isa-extensions", "a") < 0) {
    pr_warn("CPU with hartid=%lu does not support ima", *hart);
    return -ENODEV;
    }
    return 0;
    old_interface:
    if (!riscv_isa_fallback) {
    pr_warn("CPU with hartid=%lu is invalid: this kernel does not parse \"riscv,isa\"",
// hart);
    return -ENODEV;
    }
    if (of_property_read_string(node, "riscv,isa", &isa)) {
    pr_warn("CPU with hartid=%lu has no \"riscv,isa-base\" or \"riscv,isa\" property\n",
// hart);
    return -ENODEV;
    }
    if (IS_ENABLED(CONFIG_32BIT) && strncasecmp(isa, "rv32ima", 7)) {
    pr_warn("CPU with hartid=%lu does not support rv32ima", *hart);
    return -ENODEV;
    }
    if (IS_ENABLED(CONFIG_64BIT) && strncasecmp(isa, "rv64ima", 7)) {
    pr_warn("CPU with hartid=%lu does not support rv64ima", *hart);
    return -ENODEV;
    }
    return 0;
    }
//
// Find hart ID of the CPU DT node under which given DT node falls.
//
// To achieve this, we walk up the DT tree until we find an active
// RISC-V core (HART) node and extract the cpuid from it.
//
#[no_mangle]
pub unsafe extern "C" fn riscv_of_parent_hartid(node: *mut device_node, hartid: *mut c_ulong) -> c_int {
    int riscv_of_parent_hartid(struct device_node *node, unsigned long *hartid)
    {
    for (; node; node = node.parent) {
    if (of_device_is_compatible(node, "riscv")) {
// hartid = (unsigned long)of_get_cpu_hwid(node, 0);
    if (*hartid == ~0UL) {
    pr_warn("Found CPU without hart ID\n");
    return -ENODEV;
    }
    return 0;
    }
    }
    return -1;
    }
#[no_mangle]
pub unsafe extern "C" fn riscv_get_marchid() -> unsigned long __init {
    unsigned long __init riscv_get_marchid(void)
    {
    struct riscv_cpuinfo *ci = this_cpu_ptr(&riscv_cpuinfo);

    ci.marchid = sbi_spec_is_0_1() ? 0 : sbi_get_marchid();

    ci.marchid = csr_read(CSR_MARCHID);

    ci.marchid = 0;

    return ci.marchid;
    }
#[no_mangle]
pub unsafe extern "C" fn riscv_get_mvendorid() -> unsigned long __init {
    unsigned long __init riscv_get_mvendorid(void)
    {
    struct riscv_cpuinfo *ci = this_cpu_ptr(&riscv_cpuinfo);

    ci.mvendorid = sbi_spec_is_0_1() ? 0 : sbi_get_mvendorid();

    ci.mvendorid = csr_read(CSR_MVENDORID);

    ci.mvendorid = 0;

    return ci.mvendorid;
    }
    DEFINE_PER_CPU(struct riscv_cpuinfo, riscv_cpuinfo);
#[no_mangle]
pub unsafe extern "C" fn riscv_cached_mvendorid(cpu_id: c_uint) -> c_ulong {
    unsigned long riscv_cached_mvendorid(unsigned int cpu_id)
    {
    struct riscv_cpuinfo *ci = per_cpu_ptr(&riscv_cpuinfo, cpu_id);
    return ci.mvendorid;
    }
    EXPORT_SYMBOL(riscv_cached_mvendorid);
#[no_mangle]
pub unsafe extern "C" fn riscv_cached_marchid(cpu_id: c_uint) -> c_ulong {
    unsigned long riscv_cached_marchid(unsigned int cpu_id)
    {
    struct riscv_cpuinfo *ci = per_cpu_ptr(&riscv_cpuinfo, cpu_id);
    return ci.marchid;
    }
    EXPORT_SYMBOL(riscv_cached_marchid);
#[no_mangle]
pub unsafe extern "C" fn riscv_cached_mimpid(cpu_id: c_uint) -> c_ulong {
    unsigned long riscv_cached_mimpid(unsigned int cpu_id)
    {
    struct riscv_cpuinfo *ci = per_cpu_ptr(&riscv_cpuinfo, cpu_id);
    return ci.mimpid;
    }
    EXPORT_SYMBOL(riscv_cached_mimpid);
#[no_mangle]
unsafe extern "C" fn riscv_cpuinfo_starting(cpu: c_uint) -> c_int {
    static int riscv_cpuinfo_starting(unsigned int cpu)
    {
    struct riscv_cpuinfo *ci = this_cpu_ptr(&riscv_cpuinfo);

    if (!ci.mvendorid)
    ci.mvendorid = sbi_spec_is_0_1() ? 0 : sbi_get_mvendorid();
    if (!ci.marchid)
    ci.marchid = sbi_spec_is_0_1() ? 0 : sbi_get_marchid();
    ci.mimpid = sbi_spec_is_0_1() ? 0 : sbi_get_mimpid();

    if (!ci.mvendorid)
    ci.mvendorid = csr_read(CSR_MVENDORID);
    if (!ci.marchid)
    ci.marchid = csr_read(CSR_MARCHID);
    ci.mimpid = csr_read(CSR_MIMPID);

    ci.mvendorid = 0;
    ci.marchid = 0;
    ci.mimpid = 0;

    return 0;
    }
#[no_mangle]
unsafe extern "C" fn riscv_cpuinfo_init() -> int __init {
    static int __init riscv_cpuinfo_init(void)
    {
    int ret;
    ret = cpuhp_setup_state(CPUHP_AP_ONLINE_DYN, "riscv/cpuinfo:starting",
    riscv_cpuinfo_starting, core::ptr::null_mut());
    if (ret < 0) {
    pr_err("cpuinfo: failed to register hotplug callbacks.\n");
    return ret;
    }
    return 0;
    }
    arch_initcall(riscv_cpuinfo_init);

#[no_mangle]
unsafe extern "C" fn print_vendor_isa(f: *mut seq_file, cpu: c_int) {
    static void print_vendor_isa(struct seq_file *f, int cpu)
    {
    struct riscv_isavendorinfo *vendor_bitmap;
    struct riscv_isa_vendor_ext_data_list *ext_list;
    const struct riscv_isa_ext_data *ext_data;
    for (int i = 0; i < riscv_isa_vendor_ext_list_size; i++) {
    ext_list = riscv_isa_vendor_ext_list[i];
    ext_data = riscv_isa_vendor_ext_list[i].ext_data;
    if (cpu == ALL_CPUS)
    vendor_bitmap = &ext_list.all_harts_isa_bitmap;
    else
    vendor_bitmap = &ext_list.per_hart_isa_bitmap[cpu];
    for (int j = 0; j < ext_list.ext_data_count; j++) {
    if (!__riscv_isa_extension_available(vendor_bitmap.isa, ext_data[j].id))
    continue;
    seq_printf(f, "_%s", ext_data[j].name);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn print_isa(f: *mut seq_file, isa_bitmap: *const c_ulong, cpu: c_int) {
    static void print_isa(struct seq_file *f, const unsigned long *isa_bitmap, int cpu)
    {
    if (IS_ENABLED(CONFIG_32BIT))
    seq_write(f, "rv32", 4);
    else
    seq_write(f, "rv64", 4);
    for (int i = 0; i < riscv_isa_ext_count; i++) {
    if (!__riscv_isa_extension_available(isa_bitmap, riscv_isa_ext[i].id))
    continue;
// Only multi-letter extensions are split by underscores
    if (strnlen(riscv_isa_ext[i].name, 2) != 1)
    seq_puts(f, "_");
    seq_printf(f, "%s", riscv_isa_ext[i].name);
    }
    print_vendor_isa(f, cpu);
    seq_puts(f, "\n");
    }
#[no_mangle]
unsafe extern "C" fn print_mmu(f: *mut seq_file) {
    static void print_mmu(struct seq_file *f)
    {
    const char *sv_type;

    sv_type = "sv32";

    if (pgtable_l5_enabled)
    sv_type = "sv57";
#[no_mangle]
pub unsafe extern "C" fn if(_arg: pgtable_l4_enabled) -> else {
    else if (pgtable_l4_enabled)
    sv_type = "sv48";
    else
    sv_type = "sv39";

    sv_type = "none";

    seq_printf(f, "mmu\t\t: %s\n", sv_type);
    }
    static void *c_start(struct seq_file *m, loff_t *pos)
    {
    if (*pos == nr_cpu_ids)
    return core::ptr::null_mut();
// pos = cpumask_next(*pos - 1, cpu_online_mask);
    if ((*pos) < nr_cpu_ids)
    return (void *)(uintptr_t)(1 + *pos);
    return core::ptr::null_mut();
    }
    static void *c_next(struct seq_file *m, void *v, loff_t *pos)
    {
    (*pos)++;
    return c_start(m, pos);
    }
#[no_mangle]
unsafe extern "C" fn c_stop(m: *mut seq_file, v: *mut c_void) {
    static void c_stop(struct seq_file *m, void *v)
    {
    }
#[no_mangle]
unsafe extern "C" fn c_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    static int c_show(struct seq_file *m, void *v)
    {
    let mut cpu_id: c_ulong = (unsigned long)v - 1;
    struct riscv_cpuinfo *ci = per_cpu_ptr(&riscv_cpuinfo, cpu_id);
    struct device_node *node;
    const char *compat;
    seq_printf(m, "processor\t: %lu\n", cpu_id);
    seq_printf(m, "hart\t\t: %lu\n", cpuid_to_hartid_map(cpu_id));
//
// For historical raisins, the isa: line is limited to the lowest common
// denominator of extensions supported across all harts. A true list of
// extensions supported on this hart is printed later in the hart isa:
// line.
//
    seq_puts(m, "isa\t\t: ");
    print_isa(m, core::ptr::null_mut(), ALL_CPUS);
    print_mmu(m);
    if (acpi_disabled) {
    node = of_get_cpu_node(cpu_id, core::ptr::null_mut());
    if (!of_property_read_string(node, "compatible", &compat) &&
    strcmp(compat, "riscv"))
    seq_printf(m, "uarch\t\t: %s\n", compat);
    of_node_put(node);
    }
    seq_printf(m, "mvendorid\t: 0x%lx\n", ci.mvendorid);
    seq_printf(m, "marchid\t\t: 0x%lx\n", ci.marchid);
    seq_printf(m, "mimpid\t\t: 0x%lx\n", ci.mimpid);
//
// Print the ISA extensions specific to this hart, which may show
// additional extensions not present across all harts.
//
    seq_puts(m, "hart isa\t: ");
    print_isa(m, hart_isa[cpu_id].isa, cpu_id);
    seq_puts(m, "\n");
    return 0;
    }
    const struct seq_operations cpuinfo_op = {
    .start	= c_start,
    .next	= c_next,
    .stop	= c_stop,
    .show	= c_show
    };
