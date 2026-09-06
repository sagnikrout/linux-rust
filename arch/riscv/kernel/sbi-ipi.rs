//! Automatically rewritten from C to Rust
//! Source: arch/riscv/kernel/sbi-ipi.c
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
// Multiplex several IPIs over a single HW IPI.
//
// Copyright (c) 2022 Ventana Micro Systems Inc.
//

    DEFINE_STATIC_KEY_FALSE(riscv_sbi_for_rfence);
    EXPORT_SYMBOL_GPL(riscv_sbi_for_rfence);
    static int sbi_ipi_virq;
#[no_mangle]
unsafe extern "C" fn sbi_ipi_handle(desc: *mut irq_desc) {
    static void sbi_ipi_handle(struct irq_desc *desc)
    {
    struct irq_chip *chip = irq_desc_get_chip(desc);
    chained_irq_enter(chip, desc);
    csr_clear(CSR_IP, IE_SIE);
    ipi_mux_process();
    chained_irq_exit(chip, desc);
    }
#[no_mangle]
unsafe extern "C" fn sbi_ipi_starting_cpu(cpu: c_uint) -> c_int {
    static int sbi_ipi_starting_cpu(unsigned int cpu)
    {
    enable_percpu_irq(sbi_ipi_virq, irq_get_trigger_type(sbi_ipi_virq));
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn sbi_ipi_init() -> void __init {
    void __init sbi_ipi_init(void)
    {
    int virq;
    struct irq_domain *domain;
    if (riscv_ipi_have_virq_range())
    return;
    domain = irq_find_matching_fwnode(riscv_get_intc_hwnode(),
    DOMAIN_BUS_ANY);
    if (!domain) {
    pr_err("unable to find INTC IRQ domain\n");
    return;
    }
    sbi_ipi_virq = irq_create_mapping(domain, RV_IRQ_SOFT);
    if (!sbi_ipi_virq) {
    pr_err("unable to create INTC IRQ mapping\n");
    return;
    }
    virq = ipi_mux_create(BITS_PER_BYTE, sbi_send_ipi);
    if (virq <= 0) {
    pr_err("unable to create muxed IPIs\n");
    irq_dispose_mapping(sbi_ipi_virq);
    return;
    }
    irq_set_chained_handler(sbi_ipi_virq, sbi_ipi_handle);
//
// Don't disable IPI when CPU goes offline because
// the masking/unmasking of virtual IPIs is done
// via generic IPI-Mux
//
    cpuhp_setup_state(CPUHP_AP_IRQ_RISCV_SBI_IPI_STARTING,
    "irqchip/sbi-ipi:starting",
    sbi_ipi_starting_cpu, core::ptr::null_mut());
    riscv_ipi_set_virq_range(virq, BITS_PER_BYTE);
    pr_info("providing IPIs using SBI IPI extension\n");
//
// Use the SBI remote fence extension to avoid
// the extra context switch needed to handle IPIs.
//
    static_branch_enable(&riscv_sbi_for_rfence);
    }
