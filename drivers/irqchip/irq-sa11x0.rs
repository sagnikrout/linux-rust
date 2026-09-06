//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-sa11x0.c
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
// Copyright (C) 2015 Dmitry Eremin-Solenikov
// Copyright (C) 1999-2001 Nicolas Pitre
//
// Generic IRQ handling for the SA11x0.
//

pub const ICIP: c_uint = 0x00  /* IC IRQ Pending reg. */;
pub const ICMR: c_uint = 0x04  /* IC Mask Reg.        */;
pub const ICLR: c_uint = 0x08  /* IC Level Reg.       */;
pub const ICCR: c_uint = 0x0C  /* IC Control Reg.     */;
pub const ICFP: c_uint = 0x10  /* IC FIQ Pending reg. */;
pub const ICPR: c_uint = 0x20  /* IC Pending Reg.     */;
    static void __iomem *iobase;
//
// We don't need to ACK IRQs on the SA1100 unless they're GPIOs
// this is for internal IRQs i.e. from IRQ LCD to RTCAlrm.
//
#[no_mangle]
unsafe extern "C" fn sa1100_mask_irq(d: *mut irq_data) {
    static void sa1100_mask_irq(struct irq_data *d)
    {
    u32 reg;
    reg = readl_relaxed(iobase + ICMR);
    reg &= ~BIT(d.hwirq);
    writel_relaxed(reg, iobase + ICMR);
    }
#[no_mangle]
unsafe extern "C" fn sa1100_unmask_irq(d: *mut irq_data) {
    static void sa1100_unmask_irq(struct irq_data *d)
    {
    u32 reg;
    reg = readl_relaxed(iobase + ICMR);
    reg |= BIT(d.hwirq);
    writel_relaxed(reg, iobase + ICMR);
    }
#[no_mangle]
unsafe extern "C" fn sa1100_set_wake(d: *mut irq_data, on: c_uint) -> c_int {
    static int sa1100_set_wake(struct irq_data *d, unsigned int on)
    {
    return sa11x0_sc_set_wake(d.hwirq, on);
    }
    static struct irq_chip sa1100_normal_chip = {
    .name		= "SC",
    .irq_ack	= sa1100_mask_irq,
    .irq_mask	= sa1100_mask_irq,
    .irq_unmask	= sa1100_unmask_irq,
    .irq_set_wake	= sa1100_set_wake,
    };
    static int sa1100_normal_irqdomain_map(struct irq_domain *d,
    unsigned int irq, irq_hw_number_t hwirq)
    {
    irq_set_chip_and_handler(irq, &sa1100_normal_chip,
    handle_level_irq);
    return 0;
    }
    static const struct irq_domain_ops sa1100_normal_irqdomain_ops = {
    .map = sa1100_normal_irqdomain_map,
    .xlate = irq_domain_xlate_onetwocell,
    };
    static struct irq_domain *sa1100_normal_irqdomain;
    static struct sa1100irq_state {
    unsigned int	saved;
    unsigned int	icmr;
    unsigned int	iclr;
    unsigned int	iccr;
    } sa1100irq_state;
#[no_mangle]
unsafe extern "C" fn sa1100irq_suspend(data: *mut c_void) -> c_int {
    static int sa1100irq_suspend(void *data)
    {
    struct sa1100irq_state *st = &sa1100irq_state;
    st.saved = 1;
    st.icmr = readl_relaxed(iobase + ICMR);
    st.iclr = readl_relaxed(iobase + ICLR);
    st.iccr = readl_relaxed(iobase + ICCR);
//
// Disable all GPIO-based interrupts.
//
    writel_relaxed(st.icmr & 0xfffff000, iobase + ICMR);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sa1100irq_resume(data: *mut c_void) {
    static void sa1100irq_resume(void *data)
    {
    struct sa1100irq_state *st = &sa1100irq_state;
    if (st.saved) {
    writel_relaxed(st.iccr, iobase + ICCR);
    writel_relaxed(st.iclr, iobase + ICLR);
    writel_relaxed(st.icmr, iobase + ICMR);
    }
    }
    static const struct syscore_ops sa1100irq_syscore_ops = {
    .suspend	= sa1100irq_suspend,
    .resume		= sa1100irq_resume,
    };
    static struct syscore sa1100irq_syscore = {
    .ops = &sa1100irq_syscore_ops,
    };
#[no_mangle]
unsafe extern "C" fn sa1100irq_init_devicefs() -> int __init {
    static int __init sa1100irq_init_devicefs(void)
    {
    register_syscore(&sa1100irq_syscore);
    return 0;
    }
    device_initcall(sa1100irq_init_devicefs);
#[no_mangle]
unsafe extern "C" fn sa1100_handle_irq(regs: *mut pt_regs) -> void __exception_irq_entry {
    static void __exception_irq_entry sa1100_handle_irq(struct pt_regs *regs)
    {
    uint32_t icip, icmr, mask;
    do {
    icip = readl_relaxed(iobase + ICIP);
    icmr = readl_relaxed(iobase + ICMR);
    mask = icip & icmr;
    if (mask == 0)
    break;
    generic_handle_domain_irq(sa1100_normal_irqdomain,
    ffs(mask) - 1);
    } while (1);
    }
#[no_mangle]
pub unsafe extern "C" fn sa11x0_init_irq_nodt(irq_start: c_int, io_start: resource_size_t) -> void __init {
    void __init sa11x0_init_irq_nodt(int irq_start, resource_size_t io_start)
    {
    iobase = ioremap(io_start, SZ_64K);
    if (WARN_ON(!iobase))
    return;
// disable all IRQs
    writel_relaxed(0, iobase + ICMR);
// all IRQs are IRQ, not FIQ
    writel_relaxed(0, iobase + ICLR);
//
// Whatever the doc says, this has to be set for the wait-on-irq
// instruction to work... on a SA1100 rev 9 at least.
//
    writel_relaxed(1, iobase + ICCR);
    sa1100_normal_irqdomain = irq_domain_create_simple(core::ptr::null_mut(),
    32, irq_start,
    &sa1100_normal_irqdomain_ops, core::ptr::null_mut());
    set_handle_irq(sa1100_handle_irq);
    }
