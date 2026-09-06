//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-bcm2835.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright 2010 Broadcom
// Copyright 2012 Simon Arlott, Chris Boot, Stephen Warren
//
// Quirk 1: Shortcut interrupts don't set the bank 1/2 register pending bits
//
// If an interrupt fires on bank 1 that isn't in the shortcuts list, bit 8
// on bank 0 is set to signify that an interrupt in bank 1 has fired, and
// to look in the bank 1 status register for more information.
//
// If an interrupt fires on bank 1 that _is_ in the shortcuts list, its
// shortcut bit in bank 0 is set as well as its interrupt bit in the bank 1
// status register, but bank 0 bit 8 is _not_ set.
//
// Quirk 2: You can't mask the register 1/2 pending interrupts
//
// In a proper cascaded interrupt controller, the interrupt lines with
// cascaded interrupt controllers on them are just normal interrupt lines.
// You can mask the interrupts and get on with things. With this controller
// you can't do that.
//
// Quirk 3: The shortcut interrupts can't be (un)masked in bank 0
//
// Those interrupts that have shortcuts can only be masked/unmasked in
// their respective banks' enable/disable registers. Doing so in the bank 0
// enable/disable registers has no effect.
//
// The FIQ control register:
// Bits 0-6: IRQ (index in order of interrupts from banks 1, 2, then 0)
// Bit    7: Enable FIQ generation
// Bits  8+: Unused
//
// An interrupt must be disabled before configuring it for FIQ generation
// otherwise both handlers will fire at the same time!
//

// Put the bank and irq (32 bits) into the hwirq

pub const NR_IRQS_BANK0: c_int = 8;
pub const BANK0_HWIRQ_MASK: c_uint = 0xff;
// Shortcuts can't be disabled so any unknown new ones need to be masked
pub const SHORTCUT1_MASK: c_uint = 0x00007c00;
pub const SHORTCUT2_MASK: c_uint = 0x001f8000;
pub const SHORTCUT_SHIFT: c_int = 10;

    | SHORTCUT1_MASK | SHORTCUT2_MASK)
pub const REG_FIQ_CONTROL: c_uint = 0x0c;

pub const NR_BANKS: c_int = 3;
pub const IRQS_PER_BANK: c_int = 32;
    static const int reg_pending[] __initconst = { 0x00, 0x04, 0x08 };
    static const int reg_enable[] __initconst = { 0x18, 0x10, 0x14 };
    static const int reg_disable[] __initconst = { 0x24, 0x1c, 0x20 };
    static const int bank_irqs[] __initconst = { 8, 32, 32 };
    static const int shortcuts[] = {
    7, 9, 10, 18, 19,		/* Bank 1 */
    21, 22, 23, 24, 25, 30		/* Bank 2 */
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct armctrl_ic {
    pub base: *mut void __iomem,
    pub pending: [*mut void __iomem; NR_BANKS],
    pub enable: [*mut void __iomem; NR_BANKS],
    pub disable: [*mut void __iomem; NR_BANKS],
    pub domain: *mut irq_domain,
}

    static struct armctrl_ic intc __read_mostly;
    static void __exception_irq_entry bcm2835_handle_irq(
    struct pt_regs *regs);
    static void bcm2836_chained_handle_irq(struct irq_desc *desc);
#[no_mangle]
unsafe extern "C" fn armctrl_mask_irq(d: *mut irq_data) {
    static void armctrl_mask_irq(struct irq_data *d)
    {
    writel_relaxed(HWIRQ_BIT(d.hwirq), intc.disable[HWIRQ_BANK(d.hwirq)]);
    }
#[no_mangle]
unsafe extern "C" fn armctrl_unmask_irq(d: *mut irq_data) {
    static void armctrl_unmask_irq(struct irq_data *d)
    {
    writel_relaxed(HWIRQ_BIT(d.hwirq), intc.enable[HWIRQ_BANK(d.hwirq)]);
    }
    static struct irq_chip armctrl_chip = {
    .name = "ARMCTRL-level",
    .irq_mask = armctrl_mask_irq,
    .irq_unmask = armctrl_unmask_irq,
    .flags = IRQCHIP_MASK_ON_SUSPEND |
    IRQCHIP_SKIP_SET_WAKE,
    };
    static int armctrl_xlate(struct irq_domain *d, struct device_node *ctrlr,
    const u32 *intspec, unsigned int intsize,
    unsigned long *out_hwirq, unsigned int *out_type)
    {
    if (WARN_ON(intsize != 2))
    return -EINVAL;
    if (WARN_ON(intspec[0] >= NR_BANKS))
    return -EINVAL;
    if (WARN_ON(intspec[1] >= IRQS_PER_BANK))
    return -EINVAL;
    if (WARN_ON(intspec[0] == 0 && intspec[1] >= NR_IRQS_BANK0))
    return -EINVAL;
// out_hwirq = MAKE_HWIRQ(intspec[0], intspec[1]);
// out_type = IRQ_TYPE_NONE;
    return 0;
    }
    static const struct irq_domain_ops armctrl_ops = {
    .xlate = armctrl_xlate
    };
    static int __init armctrl_of_init(struct device_node *node,
    struct device_node *parent,
    bool is_2836)
    {
    void __iomem *base;
    int irq, b, i;
    u32 reg;
    base = of_iomap(node, 0);
    if (!base)
    panic("%pOF: unable to map IC registers\n", node);
    intc.domain = irq_domain_create_linear(of_fwnode_handle(node), MAKE_HWIRQ(NR_BANKS, 0),
    &armctrl_ops, core::ptr::null_mut());
    if (!intc.domain)
    panic("%pOF: unable to create IRQ domain\n", node);
    for (b = 0; b < NR_BANKS; b++) {
    intc.pending[b] = base + reg_pending[b];
    intc.enable[b] = base + reg_enable[b];
    intc.disable[b] = base + reg_disable[b];
    for (i = 0; i < bank_irqs[b]; i++) {
    irq = irq_create_mapping(intc.domain, MAKE_HWIRQ(b, i));
    BUG_ON(irq <= 0);
    irq_set_chip_and_handler(irq, &armctrl_chip,
    handle_level_irq);
    irq_set_probe(irq);
    }
    reg = readl_relaxed(intc.enable[b]);
    if (reg) {
    writel_relaxed(reg, intc.disable[b]);
    pr_err(FW_BUG "Bootloader left irq enabled: "
    "bank %d irq %*pbl\n", b, IRQS_PER_BANK, &reg);
    }
    }
    reg = readl_relaxed(base + REG_FIQ_CONTROL);
    if (reg & FIQ_CONTROL_ENABLE) {
    writel_relaxed(0, base + REG_FIQ_CONTROL);
    pr_err(FW_BUG "Bootloader left fiq enabled\n");
    }
    if (is_2836) {
    let mut parent_irq: c_int = irq_of_parse_and_map(node, 0);
    if (!parent_irq) {
    panic("%pOF: unable to get parent interrupt.\n",
    node);
    }
    irq_set_chained_handler(parent_irq, bcm2836_chained_handle_irq);
    } else {
    set_handle_irq(bcm2835_handle_irq);
    }
    return 0;
    }
    static int __init bcm2835_armctrl_of_init(struct device_node *node,
    struct device_node *parent)
    {
    return armctrl_of_init(node, parent, false);
    }
    static int __init bcm2836_armctrl_of_init(struct device_node *node,
    struct device_node *parent)
    {
    return armctrl_of_init(node, parent, true);
    }
//
// Handle each interrupt across the entire interrupt controller.  This reads the
// status register before handling each interrupt, which is necessary given that
// handle_IRQ may briefly re-enable interrupts for soft IRQ handling.
//
#[no_mangle]
unsafe extern "C" fn armctrl_translate_bank(bank: c_int) -> u32 {
    static u32 armctrl_translate_bank(int bank)
    {
    let mut stat: u32 = readl_relaxed(intc.pending[bank]);
    return MAKE_HWIRQ(bank, ffs(stat) - 1);
    }
#[no_mangle]
unsafe extern "C" fn armctrl_translate_shortcut(bank: c_int, stat: u32) -> u32 {
    static u32 armctrl_translate_shortcut(int bank, u32 stat)
    {
    return MAKE_HWIRQ(bank, shortcuts[ffs(stat >> SHORTCUT_SHIFT) - 1]);
    }
#[no_mangle]
unsafe extern "C" fn get_next_armctrl_hwirq() -> u32 {
    static u32 get_next_armctrl_hwirq(void)
    {
    let mut stat: u32 = readl_relaxed(intc.pending[0]) & BANK0_VALID_MASK;
    if (stat == 0)
    return ~0;
#[no_mangle]
pub unsafe extern "C" fn if(BANK0_HWIRQ_MASK: stat &) -> else {
    else if (stat & BANK0_HWIRQ_MASK)
    return MAKE_HWIRQ(0, ffs(stat & BANK0_HWIRQ_MASK) - 1);
#[no_mangle]
pub unsafe extern "C" fn if(SHORTCUT1_MASK: stat &) -> else {
    else if (stat & SHORTCUT1_MASK)
    return armctrl_translate_shortcut(1, stat & SHORTCUT1_MASK);
#[no_mangle]
pub unsafe extern "C" fn if(SHORTCUT2_MASK: stat &) -> else {
    else if (stat & SHORTCUT2_MASK)
    return armctrl_translate_shortcut(2, stat & SHORTCUT2_MASK);
#[no_mangle]
pub unsafe extern "C" fn if(BANK1_HWIRQ: stat &) -> else {
    else if (stat & BANK1_HWIRQ)
    return armctrl_translate_bank(1);
#[no_mangle]
pub unsafe extern "C" fn if(BANK2_HWIRQ: stat &) -> else {
    else if (stat & BANK2_HWIRQ)
    return armctrl_translate_bank(2);
    else
    BUG();
    }
    static void __exception_irq_entry bcm2835_handle_irq(
    struct pt_regs *regs)
    {
    u32 hwirq;
    while ((hwirq = get_next_armctrl_hwirq()) != ~0)
    generic_handle_domain_irq(intc.domain, hwirq);
    }
#[no_mangle]
unsafe extern "C" fn bcm2836_chained_handle_irq(desc: *mut irq_desc) {
    static void bcm2836_chained_handle_irq(struct irq_desc *desc)
    {
    u32 hwirq;
    while ((hwirq = get_next_armctrl_hwirq()) != ~0)
    generic_handle_domain_irq(intc.domain, hwirq);
    }
    IRQCHIP_DECLARE(bcm2835_armctrl_ic, "brcm,bcm2835-armctrl-ic",
    bcm2835_armctrl_of_init);
    IRQCHIP_DECLARE(bcm2836_armctrl_ic, "brcm,bcm2836-armctrl-ic",
    bcm2836_armctrl_of_init);
