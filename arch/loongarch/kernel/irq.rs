//! Automatically rewritten from C to Rust
//! Source: arch/loongarch/kernel/irq.c
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
// Copyright (C) 2020-2022 Loongson Technology Corporation Limited
//

    DEFINE_PER_CPU(unsigned long, irq_stack);
    DEFINE_PER_CPU_SHARED_ALIGNED(irq_cpustat_t, irq_stat);
    EXPORT_PER_CPU_SYMBOL(irq_stat);
    struct acpi_vector_group pch_group[MAX_IO_PICS];
    struct acpi_vector_group msi_group[MAX_IO_PICS];
//
// 'what should we do if we get a hw irq event on an illegal vector'.
// each architecture has to answer this themselves.
//
#[no_mangle]
pub unsafe extern "C" fn ack_bad_irq(irq: c_uint) {
    void ack_bad_irq(unsigned int irq)
    {
    pr_warn("Unexpected IRQ # %d\n", irq);
    }
    atomic_t irq_err_count;
#[no_mangle]
pub unsafe extern "C" fn spurious_interrupt() -> asmlinkage void {
    asmlinkage void spurious_interrupt(void)
    {
    atomic_inc(&irq_err_count);
    }
#[no_mangle]
pub unsafe extern "C" fn arch_show_interrupts(p: *mut seq_file, prec: c_int) -> c_int {
    int arch_show_interrupts(struct seq_file *p, int prec)
    {

    show_ipi_list(p, prec);

    seq_printf(p, "%*s: %10u\n", prec, "ERR", atomic_read(&irq_err_count));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn early_pci_mcfg_parse(header: *mut acpi_table_header) -> int __init {
    static int __init early_pci_mcfg_parse(struct acpi_table_header *header)
    {
    struct acpi_table_mcfg *mcfg;
    struct acpi_mcfg_allocation *mptr;
    int i, n;
    if (header.length < sizeof(struct acpi_table_mcfg))
    return -EINVAL;
    n = (header.length - sizeof(struct acpi_table_mcfg)) /
    sizeof(struct acpi_mcfg_allocation);
    mcfg = (struct acpi_table_mcfg *)header;
    mptr = (struct acpi_mcfg_allocation *) &mcfg[1];
    for (i = 0; i < n; i++, mptr++) {
    msi_group[i].pci_segment = mptr.pci_segment;
    pch_group[i].node = msi_group[i].node = (mptr.address >> 44) & 0xf;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn init_vec_parent_group() -> void __init {
    static void __init init_vec_parent_group(void)
    {
    int i;
    for (i = 0; i < MAX_IO_PICS; i++) {
    msi_group[i].pci_segment = -1;
    msi_group[i].node = -1;
    pch_group[i].node = -1;
    }
    acpi_table_parse(ACPI_SIG_MCFG, early_pci_mcfg_parse);
    }
#[no_mangle]
pub unsafe extern "C" fn arch_probe_nr_irqs() -> int __init {
    int __init arch_probe_nr_irqs(void)
    {
    let mut nr_io_pics: c_int = bitmap_weight(loongson_sysconf.cores_io_master, NR_CPUS);
    if (!cpu_has_avecint)
    irq_set_nr_irqs(64 + NR_VECTORS * nr_io_pics);
    else
    irq_set_nr_irqs(64 + NR_VECTORS * (nr_cpu_ids + nr_io_pics));
    return NR_IRQS_LEGACY;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_dynirq_lower_bound(from: c_uint) -> c_uint {
    unsigned int arch_dynirq_lower_bound(unsigned int from)
    {
    return MAX(from, NR_IRQS_LEGACY);
    }
#[no_mangle]
pub unsafe extern "C" fn init_IRQ() -> void __init {
    void __init init_IRQ(void)
    {
    int i;
    let mut order: c_uint = get_order(IRQ_STACK_SIZE);
    struct page *page;
    clear_csr_ecfg(ECFG0_IM);
    clear_csr_estat(ESTATF_IP);
    init_vec_parent_group();
    irqchip_init();

    mp_ops.init_ipi();

    for_each_possible_cpu(i) {
    page = alloc_pages_node(cpu_to_node(i), GFP_KERNEL, order);
    per_cpu(irq_stack, i) = (unsigned long)page_address(page);
    pr_debug("CPU%d IRQ stack at 0x%lx - 0x%lx\n", i,
    per_cpu(irq_stack, i), per_cpu(irq_stack, i) + IRQ_STACK_SIZE);
    }
    set_csr_ecfg(ECFGF_SIP0 | ECFGF_IP0 | ECFGF_IP1 | ECFGF_IP2 | ECFGF_IPI | ECFGF_PMC);
    }
