//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-i8259.c
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
// Code to handle x86 style IRQs plus some generic interrupt stuff.
//
// Copyright (C) 1992 Linus Torvalds
// Copyright (C) 1994 - 2000 Ralf Baechle
//

//
// This is the 'legacy' 8259A Programmable Interrupt Controller,
// present in the majority of PC/AT boxes.
// plus some generic x86 specific things if generic specifics makes
// any sense at all.
// this file should become arch/i386/kernel/irq.c when the old irq.c
// moves to arch independent land
//
    let mut i8259A_auto_eoi: static int = -1;
    DEFINE_RAW_SPINLOCK(i8259A_lock);
    static void disable_8259A_irq(struct irq_data *d);
    static void enable_8259A_irq(struct irq_data *d);
    static void mask_and_ack_8259A(struct irq_data *d);
    static void init_8259A(int auto_eoi);
    static int (*i8259_poll)(void) = i8259_irq;
    static struct irq_chip i8259A_chip = {
    .name			= "XT-PIC",
    .irq_mask		= disable_8259A_irq,
    .irq_disable		= disable_8259A_irq,
    .irq_unmask		= enable_8259A_irq,
    .irq_mask_ack		= mask_and_ack_8259A,
    };
//
// 8259A PIC functions to handle ISA devices:
//
#[no_mangle]
pub unsafe extern "C" fn i8259_set_poll((*poll)(void): *mut c_int) {
    void i8259_set_poll(int (*poll)(void))
    {
    i8259_poll = poll;
    }
//
// This contains the irq mask for both 8259A irq controllers,
//
    let mut cached_irq_mask: static unsigned int = 0xffff;

#[no_mangle]
unsafe extern "C" fn disable_8259A_irq(d: *mut irq_data) {
    static void disable_8259A_irq(struct irq_data *d)
    {
    unsigned int mask, irq = d.irq - I8259A_IRQ_BASE;
    unsigned long flags;
    mask = 1 << irq;
    raw_spin_lock_irqsave(&i8259A_lock, flags);
    cached_irq_mask |= mask;
    if (irq & 8)
    outb(cached_slave_mask, PIC_SLAVE_IMR);
    else
    outb(cached_master_mask, PIC_MASTER_IMR);
    raw_spin_unlock_irqrestore(&i8259A_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn enable_8259A_irq(d: *mut irq_data) {
    static void enable_8259A_irq(struct irq_data *d)
    {
    unsigned int mask, irq = d.irq - I8259A_IRQ_BASE;
    unsigned long flags;
    mask = ~(1 << irq);
    raw_spin_lock_irqsave(&i8259A_lock, flags);
    cached_irq_mask &= mask;
    if (irq & 8)
    outb(cached_slave_mask, PIC_SLAVE_IMR);
    else
    outb(cached_master_mask, PIC_MASTER_IMR);
    raw_spin_unlock_irqrestore(&i8259A_lock, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn make_8259A_irq(irq: c_uint) {
    void make_8259A_irq(unsigned int irq)
    {
    disable_irq_nosync(irq);
    irq_set_chip_and_handler(irq, &i8259A_chip, handle_level_irq);
    enable_irq(irq);
    }
//
// This function assumes to be called rarely. Switching between
// 8259A registers is slow.
// This has to be protected by the irq controller spinlock
// before being called.
//
#[no_mangle]
pub unsafe extern "C" fn i8259A_irq_real(irq: c_uint) -> c_int {
    static inline int i8259A_irq_real(unsigned int irq)
    {
    int value;
    let mut irqmask: c_int = 1 << irq;
    if (irq < 8) {
    outb(0x0B, PIC_MASTER_CMD);	/* ISR register */
    value = inb(PIC_MASTER_CMD) & irqmask;
    outb(0x0A, PIC_MASTER_CMD);	/* back to the IRR register */
    return value;
    }
    outb(0x0B, PIC_SLAVE_CMD);	/* ISR register */
    value = inb(PIC_SLAVE_CMD) & (irqmask >> 8);
    outb(0x0A, PIC_SLAVE_CMD);	/* back to the IRR register */
    return value;
    }
//
// Careful! The 8259A is a fragile beast, it pretty
// much _has_ to be done exactly like this (mask it
// first, _then_ send the EOI, and the order of EOI
// to the two 8259s is important!
//
#[no_mangle]
unsafe extern "C" fn mask_and_ack_8259A(d: *mut irq_data) {
    static void mask_and_ack_8259A(struct irq_data *d)
    {
    unsigned int irqmask, irq = d.irq - I8259A_IRQ_BASE;
    unsigned long flags;
    irqmask = 1 << irq;
    raw_spin_lock_irqsave(&i8259A_lock, flags);
//
// Lightweight spurious IRQ detection. We do not want
// to overdo spurious IRQ handling - it's usually a sign
// of hardware problems, so we only do the checks we can
// do without slowing down good hardware unnecessarily.
//
// Note that IRQ7 and IRQ15 (the two spurious IRQs
// usually resulting from the 8259A-1|2 PICs) occur
// even if the IRQ is masked in the 8259A. Thus we
// can check spurious 8259A IRQs without doing the
// quite slow i8259A_irq_real() call for every IRQ.
// This does not cover 100% of spurious interrupts,
// but should be enough to warn the user that there
// is something bad going on ...
//
    if (cached_irq_mask & irqmask)
    goto spurious_8259A_irq;
    cached_irq_mask |= irqmask;
    handle_real_irq:
    if (irq & 8) {
    inb(PIC_SLAVE_IMR);	/* DUMMY - (do we need this?) */
    outb(cached_slave_mask, PIC_SLAVE_IMR);
    outb(0x60+(irq&7), PIC_SLAVE_CMD);/* 'Specific EOI' to slave */
    outb(0x60+PIC_CASCADE_IR, PIC_MASTER_CMD); /* 'Specific EOI' to master-IRQ2 */
    } else {
    inb(PIC_MASTER_IMR);	/* DUMMY - (do we need this?) */
    outb(cached_master_mask, PIC_MASTER_IMR);
    outb(0x60+irq, PIC_MASTER_CMD); /* 'Specific EOI to master */
    }
    raw_spin_unlock_irqrestore(&i8259A_lock, flags);
    return;
    spurious_8259A_irq:
//
// this is the slow path - should happen rarely.
//
    if (i8259A_irq_real(irq))
//
// oops, the IRQ _is_ in service according to the
// 8259A - not spurious, go handle it.
//
    goto handle_real_irq;
    {
    static int spurious_irq_mask;
//
// At this point we can be sure the IRQ is spurious,
// lets ACK and report it. [once per IRQ]
//
    if (!(spurious_irq_mask & irqmask)) {
    printk(KERN_DEBUG "spurious 8259A interrupt: IRQ%d.\n", irq);
    spurious_irq_mask |= irqmask;
    }
    atomic_inc(&irq_err_count);
//
// Theoretically we do not have to handle this IRQ,
// but in Linux this does not cause problems and is
// simpler for us.
//
    goto handle_real_irq;
    }
    }
#[no_mangle]
unsafe extern "C" fn i8259A_resume(data: *mut c_void) {
    static void i8259A_resume(void *data)
    {
    if (i8259A_auto_eoi >= 0)
    init_8259A(i8259A_auto_eoi);
    }
#[no_mangle]
unsafe extern "C" fn i8259A_shutdown(data: *mut c_void) {
    static void i8259A_shutdown(void *data)
    {
// Put the i8259A into a quiescent state that
// the kernel initialization code can get it
// out of.
//
    if (i8259A_auto_eoi >= 0) {
    outb(0xff, PIC_MASTER_IMR);	/* mask all of 8259A-1 */
    outb(0xff, PIC_SLAVE_IMR);	/* mask all of 8259A-2 */
    }
    }
    static const struct syscore_ops i8259_syscore_ops = {
    .resume = i8259A_resume,
    .shutdown = i8259A_shutdown,
    };
    static struct syscore i8259_syscore = {
    .ops = &i8259_syscore_ops,
    };
#[no_mangle]
unsafe extern "C" fn init_8259A(auto_eoi: c_int) {
    static void init_8259A(int auto_eoi)
    {
    unsigned long flags;
    i8259A_auto_eoi = auto_eoi;
    raw_spin_lock_irqsave(&i8259A_lock, flags);
    outb(0xff, PIC_MASTER_IMR);	/* mask all of 8259A-1 */
    outb(0xff, PIC_SLAVE_IMR);	/* mask all of 8259A-2 */
//
// outb_p - this has to work on a wide range of PC hardware.
//
    outb_p(0x11, PIC_MASTER_CMD);	/* ICW1: select 8259A-1 init */
    outb_p(I8259A_IRQ_BASE + 0, PIC_MASTER_IMR);	/* ICW2: 8259A-1 IR0 mapped to I8259A_IRQ_BASE + 0x00 */
    outb_p(1U << PIC_CASCADE_IR, PIC_MASTER_IMR);	/* 8259A-1 (the master) has a slave on IR2 */
    if (auto_eoi)	/* master does Auto EOI */
    outb_p(MASTER_ICW4_DEFAULT | PIC_ICW4_AEOI, PIC_MASTER_IMR);
    else		/* master expects normal EOI */
    outb_p(MASTER_ICW4_DEFAULT, PIC_MASTER_IMR);
    outb_p(0x11, PIC_SLAVE_CMD);	/* ICW1: select 8259A-2 init */
    outb_p(I8259A_IRQ_BASE + 8, PIC_SLAVE_IMR);	/* ICW2: 8259A-2 IR0 mapped to I8259A_IRQ_BASE + 0x08 */
    outb_p(PIC_CASCADE_IR, PIC_SLAVE_IMR);	/* 8259A-2 is a slave on master's IR2 */
    outb_p(SLAVE_ICW4_DEFAULT, PIC_SLAVE_IMR); /* (slave's support for AEOI in flat mode is to be investigated) */
    if (auto_eoi)
//
// In AEOI mode we just have to mask the interrupt
// when acking.
//
    i8259A_chip.irq_mask_ack = disable_8259A_irq;
    else
    i8259A_chip.irq_mask_ack = mask_and_ack_8259A;
    udelay(100);		/* wait for 8259A to initialize */
    outb(cached_master_mask, PIC_MASTER_IMR); /* restore master IRQ mask */
    outb(cached_slave_mask, PIC_SLAVE_IMR);	  /* restore slave IRQ mask */
    raw_spin_unlock_irqrestore(&i8259A_lock, flags);
    }
    static struct resource pic1_io_resource = {
    .name = "pic1",
    .start = PIC_MASTER_CMD,
    .end = PIC_MASTER_IMR,
    .flags = IORESOURCE_IO | IORESOURCE_BUSY
    };
    static struct resource pic2_io_resource = {
    .name = "pic2",
    .start = PIC_SLAVE_CMD,
    .end = PIC_SLAVE_IMR,
    .flags = IORESOURCE_IO | IORESOURCE_BUSY
    };
    static int i8259A_irq_domain_map(struct irq_domain *d, unsigned int virq,
    irq_hw_number_t hw)
    {
    irq_set_chip_and_handler(virq, &i8259A_chip, handle_level_irq);
    irq_set_probe(virq);
    return 0;
    }
    static const struct irq_domain_ops i8259A_ops = {
    .map = i8259A_irq_domain_map,
    .xlate = irq_domain_xlate_onecell,
    };
//
// On systems with i8259-style interrupt controllers we assume for
// driver compatibility reasons interrupts 0 - 15 to be the i8259
// interrupts even if the hardware uses a different interrupt numbering.
//
#[no_mangle]
pub unsafe extern "C" fn __init_i8259_irqs(node: *mut device_node) -> *mut irq_domain  __init {
    struct irq_domain * __init __init_i8259_irqs(struct device_node *node)
    {
//
// PIC_CASCADE_IR is cascade interrupt to second interrupt controller
//
    let mut irq: c_int = I8259A_IRQ_BASE + PIC_CASCADE_IR;
    struct irq_domain *domain;
    insert_resource(&ioport_resource, &pic1_io_resource);
    insert_resource(&ioport_resource, &pic2_io_resource);
    init_8259A(0);
    domain = irq_domain_create_legacy(of_fwnode_handle(node), 16, I8259A_IRQ_BASE, 0,
    &i8259A_ops, core::ptr::null_mut());
    if (!domain)
    panic("Failed to add i8259 IRQ domain");
    if (request_irq(irq, no_action, IRQF_NO_THREAD, "cascade", core::ptr::null_mut()))
    pr_err("Failed to register cascade interrupt\n");
    register_syscore(&i8259_syscore);
    return domain;
    }
#[no_mangle]
pub unsafe extern "C" fn init_i8259_irqs() -> void __init {
    void __init init_i8259_irqs(void)
    {
    __init_i8259_irqs(core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn i8259_irq_dispatch(desc: *mut irq_desc) {
    static void i8259_irq_dispatch(struct irq_desc *desc)
    {
    struct irq_domain *domain = irq_desc_get_handler_data(desc);
    let mut hwirq: c_int = i8259_poll();
    if (hwirq < 0)
    return;
    generic_handle_domain_irq(domain, hwirq);
    }
#[no_mangle]
unsafe extern "C" fn i8259_of_init(node: *mut device_node, parent: *mut device_node) -> int __init {
    static int __init i8259_of_init(struct device_node *node, struct device_node *parent)
    {
    struct irq_domain *domain;
    unsigned int parent_irq;
    domain = __init_i8259_irqs(node);
    parent_irq = irq_of_parse_and_map(node, 0);
    if (!parent_irq) {
    pr_err("Failed to map i8259 parent IRQ\n");
    irq_domain_remove(domain);
    return -ENODEV;
    }
    irq_set_chained_handler_and_data(parent_irq, i8259_irq_dispatch,
    domain);
    return 0;
    }
    IRQCHIP_DECLARE(i8259, "intel,i8259", i8259_of_init);
