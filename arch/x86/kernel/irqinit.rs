//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/irqinit.c
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
// ISA PIC or low IO-APIC triggered (INTA-cycle or APIC) interrupts:
// (these are usually mapped to vectors 0x30-0x3f)
//
// The IO-APIC gives us many more interrupt sources. Most of these
// are unused but an SMP system is supposed to have enough memory ...
// sometimes (mostly wrt. hw bugs) we get corrupted vectors all
// across the spectrum, so we really want to be prepared to get all
// of these. Plus, more powerful systems might have more than 64
// IO-APIC registers.
//
// (these are usually mapped into the 0x30-0xff vector range)
//
    DEFINE_PER_CPU(vector_irq_t, vector_irq) = {
    [0 ... NR_VECTORS - 1] = VECTOR_UNUSED,
    };
#[no_mangle]
pub unsafe extern "C" fn init_ISA_irqs() -> void __init {
    void __init init_ISA_irqs(void)
    {
    struct irq_chip *chip = legacy_pic.chip;
    int i;
//
// Try to set up the through-local-APIC virtual wire mode earlier.
//
// On some 32-bit UP machines, whose APIC has been disabled by BIOS
// and then got re-enabled by "lapic", it hangs at boot time without this.
//
    init_bsp_APIC();
    legacy_pic.init(0);
    for (i = 0; i < nr_legacy_irqs(); i++) {
    irq_set_chip_and_handler(i, chip, handle_level_irq);
    irq_set_status_flags(i, IRQ_LEVEL);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn init_IRQ() -> void __init {
    void __init init_IRQ(void)
    {
    int i;
//
// On cpu 0, Assign ISA_IRQ_VECTOR(irq) to IRQ 0..15.
// If these IRQ's are handled by legacy interrupt-controllers like PIC,
// then this configuration will likely be static after the boot. If
// these IRQs are handled by more modern controllers like IO-APIC,
// then this vector space can be freed and re-used dynamically as the
// irq's migrate etc.
//
    for (i = 0; i < nr_legacy_irqs(); i++)
    per_cpu(vector_irq, 0)[ISA_IRQ_VECTOR(i)] = irq_to_desc(i);
    BUG_ON(irq_init_percpu_irqstack(smp_processor_id()));
    x86_init.irqs.intr_init();
    }
#[no_mangle]
pub unsafe extern "C" fn native_init_IRQ() -> void __init {
    void __init native_init_IRQ(void)
    {
// Execute any quirks before the call gates are initialised:
    x86_init.irqs.pre_vector_init();
// FRED's IRQ path may be used even if FRED isn't fully enabled.
    if (IS_ENABLED(CONFIG_X86_FRED))
    fred_complete_exception_setup();
    if (!cpu_feature_enabled(X86_FEATURE_FRED))
    idt_setup_apic_and_irq_gates();
    lapic_assign_system_vectors();
    if (!acpi_ioapic && !of_ioapic && nr_legacy_irqs()) {
// IRQ2 is cascade interrupt to second interrupt controller
    if (request_irq(2, no_action, IRQF_NO_THREAD, "cascade", core::ptr::null_mut()))
    pr_err("%s: request_irq() failed\n", "cascade");
    }
    }
