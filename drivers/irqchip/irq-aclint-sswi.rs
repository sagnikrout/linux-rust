//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-aclint-sswi.c
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
// Copyright (C) 2024 Inochi Amaoto <inochiama@gmail.com>
//

    static int sswi_ipi_virq __ro_after_init;
    static DEFINE_PER_CPU(void __iomem *, sswi_cpu_regs);
#[no_mangle]
unsafe extern "C" fn aclint_sswi_ipi_send(cpu: c_uint) {
    static void aclint_sswi_ipi_send(unsigned int cpu)
    {
    writel(0x1, per_cpu(sswi_cpu_regs, cpu));
    }
#[no_mangle]
unsafe extern "C" fn aclint_sswi_ipi_clear() {
    static void aclint_sswi_ipi_clear(void)
    {
    writel_relaxed(0x0, this_cpu_read(sswi_cpu_regs));
    }
#[no_mangle]
unsafe extern "C" fn aclint_sswi_ipi_handle(desc: *mut irq_desc) {
    static void aclint_sswi_ipi_handle(struct irq_desc *desc)
    {
    struct irq_chip *chip = irq_desc_get_chip(desc);
    chained_irq_enter(chip, desc);
    csr_clear(CSR_IP, IE_SIE);
    aclint_sswi_ipi_clear();
    ipi_mux_process();
    chained_irq_exit(chip, desc);
    }
#[no_mangle]
unsafe extern "C" fn aclint_sswi_starting_cpu(cpu: c_uint) -> c_int {
    static int aclint_sswi_starting_cpu(unsigned int cpu)
    {
    enable_percpu_irq(sswi_ipi_virq, irq_get_trigger_type(sswi_ipi_virq));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn aclint_sswi_dying_cpu(cpu: c_uint) -> c_int {
    static int aclint_sswi_dying_cpu(unsigned int cpu)
    {
    aclint_sswi_ipi_clear();
    disable_percpu_irq(sswi_ipi_virq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn aclint_sswi_parse_irq(fwnode: *mut fwnode_handle, reg: *mut void __iomem) -> int __init {
    static int __init aclint_sswi_parse_irq(struct fwnode_handle *fwnode, void __iomem *reg)
    {
    let mut contexts: u32 = of_irq_count(to_of_node(fwnode));
    if (!(contexts)) {
    pr_err("%pfwP: no ACLINT SSWI context available\n", fwnode);
    return -EINVAL;
    }
    for (u32 i = 0; i < contexts; i++) {
    struct of_phandle_args parent;
    unsigned long hartid;
    u32 hart_index;
    int rc, cpu;
    rc = of_irq_parse_one(to_of_node(fwnode), i, &parent);
    if (rc)
    return rc;
    rc = riscv_of_parent_hartid(parent.np, &hartid);
    if (rc)
    return rc;
    if (parent.args[0] != RV_IRQ_SOFT)
    return -ENOTSUPP;
    cpu = riscv_hartid_to_cpuid(hartid);
    rc = riscv_get_hart_index(fwnode, i, &hart_index);
    if (rc) {
    pr_warn("%pfwP: hart index [%d] not found\n", fwnode, i);
    return -EINVAL;
    }
    per_cpu(sswi_cpu_regs, cpu) = reg + hart_index * 4;
    }
    pr_info("%pfwP: register %u CPU%s\n", fwnode, contexts, str_plural(contexts));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn aclint_sswi_probe(fwnode: *mut fwnode_handle) -> int __init {
    static int __init aclint_sswi_probe(struct fwnode_handle *fwnode)
    {
    struct irq_domain *domain;
    void __iomem *reg;
    int virq, rc;
    if (!is_of_node(fwnode))
    return -EINVAL;
    reg = of_io_request_and_map(to_of_node(fwnode), 0, core::ptr::null_mut());
    if (IS_ERR(reg)) {
    pr_err("%pfwP: Failed to map MMIO region\n", fwnode);
    return PTR_ERR(reg);
    }
// Parse SSWI setting
    rc = aclint_sswi_parse_irq(fwnode, reg);
    if (rc < 0)
    return rc;
// If mulitple SSWI devices are present, do not register irq again
    if (sswi_ipi_virq)
    return 0;
// Find riscv intc domain and create IPI irq mapping
    domain = irq_find_matching_fwnode(riscv_get_intc_hwnode(), DOMAIN_BUS_ANY);
    if (!domain) {
    pr_err("%pfwP: Failed to find INTC domain\n", fwnode);
    return -ENOENT;
    }
    sswi_ipi_virq = irq_create_mapping(domain, RV_IRQ_SOFT);
    if (!sswi_ipi_virq) {
    pr_err("unable to create ACLINT SSWI IRQ mapping\n");
    return -ENOMEM;
    }
// Register SSWI irq and handler
    virq = ipi_mux_create(BITS_PER_BYTE, aclint_sswi_ipi_send);
    if (virq <= 0) {
    pr_err("unable to create muxed IPIs\n");
    irq_dispose_mapping(sswi_ipi_virq);
    return virq < 0 ? virq : -ENOMEM;
    }
    irq_set_chained_handler(sswi_ipi_virq, aclint_sswi_ipi_handle);
    cpuhp_setup_state(CPUHP_AP_IRQ_ACLINT_SSWI_STARTING,
    "irqchip/aclint-sswi:starting",
    aclint_sswi_starting_cpu,
    aclint_sswi_dying_cpu);
    riscv_ipi_set_virq_range(virq, BITS_PER_BYTE);
    return 0;
    }
// generic/MIPS variant
#[no_mangle]
unsafe extern "C" fn generic_aclint_sswi_probe(fwnode: *mut fwnode_handle) -> int __init {
    static int __init generic_aclint_sswi_probe(struct fwnode_handle *fwnode)
    {
    int rc;
    rc = aclint_sswi_probe(fwnode);
    if (rc)
    return rc;
// Announce that SSWI is providing IPIs
    pr_info("providing IPIs using ACLINT SSWI\n");
    return 0;
    }
    static int __init generic_aclint_sswi_early_probe(struct device_node *node,
    struct device_node *parent)
    {
    return generic_aclint_sswi_probe(&node.fwnode);
    }
    IRQCHIP_DECLARE(mips_p8700_sswi, "mips,p8700-aclint-sswi", generic_aclint_sswi_early_probe);
    IRQCHIP_DECLARE(nuclei_ux900_sswi, "nuclei,ux900-aclint-sswi", generic_aclint_sswi_early_probe);
// THEAD variant
pub const THEAD_C9XX_CSR_SXSTATUS: c_uint = 0x5c0;

#[no_mangle]
unsafe extern "C" fn thead_aclint_sswi_probe(fwnode: *mut fwnode_handle) -> int __init {
    static int __init thead_aclint_sswi_probe(struct fwnode_handle *fwnode)
    {
    int rc;
// If it is T-HEAD CPU, check whether SSWI is enabled
    if (riscv_cached_mvendorid(0) == THEAD_VENDOR_ID &&
    !(csr_read(THEAD_C9XX_CSR_SXSTATUS) & THEAD_C9XX_SXSTATUS_CLINTEE))
    return -ENOTSUPP;
    rc = aclint_sswi_probe(fwnode);
    if (rc)
    return rc;
// Announce that SSWI is providing IPIs
    pr_info("providing IPIs using THEAD ACLINT SSWI\n");
    return 0;
    }
    static int __init thead_aclint_sswi_early_probe(struct device_node *node,
    struct device_node *parent)
    {
    return thead_aclint_sswi_probe(&node.fwnode);
    }
    IRQCHIP_DECLARE(thead_aclint_sswi, "thead,c900-aclint-sswi", thead_aclint_sswi_early_probe);
