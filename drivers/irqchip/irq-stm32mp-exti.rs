//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-stm32mp-exti.c
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
// Copyright (C) Maxime Coquelin 2015
// Copyright (C) STMicroelectronics 2017-2024
// Author:  Maxime Coquelin <mcoquelin.stm32@gmail.com>
//

pub const IRQS_PER_BANK: c_int = 32;

pub const EXTI_HWCFGR1: c_uint = 0x3f0;
// Register: EXTI_EnCIDCFGR(n)

pub const EXTI_CIDCFGR_CID_SHIFT: c_int = 4;
// Register: EXTI_HWCFGR1

pub const EXTI_CID1: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32mp_exti_bank {
    pub imr_ofst: u32,
    pub rtsr_ofst: u32,
    pub ftsr_ofst: u32,
    pub swier_ofst: u32,
    pub rpr_ofst: u32,
    pub fpr_ofst: u32,
    pub trg_ofst: u32,
    pub seccfgr_ofst: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32mp_exti_drv_data {
    pub exti_banks: *const stm32mp_exti_bank,
    pub desc_irqs: *const u8,
    pub bank_nr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32mp_exti_chip_data {
    pub host_data: *mut stm32mp_exti_host_data,
    pub reg_bank: *const stm32mp_exti_bank,
    pub rlock: raw_spinlock,
    pub wake_active: u32,
    pub mask_cache: u32,
    pub rtsr_cache: u32,
    pub ftsr_cache: u32,
    pub event_reserved: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32mp_exti_host_data {
    pub base: *mut void __iomem,
    pub dev: *mut device,
    pub chips_data: *mut stm32mp_exti_chip_data,
    pub drv_data: *const stm32mp_exti_drv_data,
    pub hwlock: *mut hwspinlock,
// skip internal desc_irqs array and get it from DT
    pub dt_has_irqs_desc: bool,
}

    static const struct stm32mp_exti_bank stm32mp_exti_b1 = {
    .imr_ofst	= 0x80,
    .rtsr_ofst	= 0x00,
    .ftsr_ofst	= 0x04,
    .swier_ofst	= 0x08,
    .rpr_ofst	= 0x0C,
    .fpr_ofst	= 0x10,
    .trg_ofst	= 0x3EC,
    .seccfgr_ofst	= 0x14,
    };
    static const struct stm32mp_exti_bank stm32mp_exti_b2 = {
    .imr_ofst	= 0x90,
    .rtsr_ofst	= 0x20,
    .ftsr_ofst	= 0x24,
    .swier_ofst	= 0x28,
    .rpr_ofst	= 0x2C,
    .fpr_ofst	= 0x30,
    .trg_ofst	= 0x3E8,
    .seccfgr_ofst	= 0x34,
    };
    static const struct stm32mp_exti_bank stm32mp_exti_b3 = {
    .imr_ofst	= 0xA0,
    .rtsr_ofst	= 0x40,
    .ftsr_ofst	= 0x44,
    .swier_ofst	= 0x48,
    .rpr_ofst	= 0x4C,
    .fpr_ofst	= 0x50,
    .trg_ofst	= 0x3E4,
    .seccfgr_ofst	= 0x54,
    };
    static const struct stm32mp_exti_bank *stm32mp_exti_banks[] = {
    &stm32mp_exti_b1,
    &stm32mp_exti_b2,
    &stm32mp_exti_b3,
    };
    static struct irq_chip stm32mp_exti_chip;
    static struct irq_chip stm32mp_exti_chip_direct;

//
// Use some intentionally tricky logic here to initialize the whole array to
// EXTI_INVALID_IRQ, but then override certain fields, requiring us to indicate
// that we "know" that there are overrides in this structure, and we'll need to
// disable that warning from W=1 builds.
//
    __diag_push();
    __diag_ignore_all("-Woverride-init",
    "logic to initialize all and then override some is OK");
    static const u8 stm32mp1_desc_irq[] = {
// default value
    [0 ... (STM32MP_DESC_IRQ_SIZE - 1)] = EXTI_INVALID_IRQ,
    [0] = 6,
    [1] = 7,
    [2] = 8,
    [3] = 9,
    [4] = 10,
    [5] = 23,
    [6] = 64,
    [7] = 65,
    [8] = 66,
    [9] = 67,
    [10] = 40,
    [11] = 42,
    [12] = 76,
    [13] = 77,
    [14] = 121,
    [15] = 127,
    [16] = 1,
    [19] = 3,
    [21] = 31,
    [22] = 33,
    [23] = 72,
    [24] = 95,
    [25] = 107,
    [26] = 37,
    [27] = 38,
    [28] = 39,
    [29] = 71,
    [30] = 52,
    [31] = 53,
    [32] = 82,
    [33] = 83,
    [46] = 151,
    [47] = 93,
    [48] = 138,
    [50] = 139,
    [52] = 140,
    [53] = 141,
    [54] = 135,
    [61] = 100,
    [65] = 144,
    [68] = 143,
    [70] = 62,
    [73] = 129,
    };
    static const u8 stm32mp13_desc_irq[] = {
// default value
    [0 ... (STM32MP_DESC_IRQ_SIZE - 1)] = EXTI_INVALID_IRQ,
    [0] = 6,
    [1] = 7,
    [2] = 8,
    [3] = 9,
    [4] = 10,
    [5] = 24,
    [6] = 65,
    [7] = 66,
    [8] = 67,
    [9] = 68,
    [10] = 41,
    [11] = 43,
    [12] = 77,
    [13] = 78,
    [14] = 106,
    [15] = 109,
    [16] = 1,
    [19] = 3,
    [21] = 32,
    [22] = 34,
    [23] = 73,
    [24] = 93,
    [25] = 114,
    [26] = 38,
    [27] = 39,
    [28] = 40,
    [29] = 72,
    [30] = 53,
    [31] = 54,
    [32] = 83,
    [33] = 84,
    [44] = 96,
    [47] = 92,
    [48] = 116,
    [50] = 117,
    [52] = 118,
    [53] = 119,
    [68] = 63,
    [70] = 98,
    };
    __diag_pop();
    static const struct stm32mp_exti_drv_data stm32mp1_drv_data = {
    .exti_banks = stm32mp_exti_banks,
    .bank_nr = ARRAY_SIZE(stm32mp_exti_banks),
    .desc_irqs = stm32mp1_desc_irq,
    };
    static const struct stm32mp_exti_drv_data stm32mp13_drv_data = {
    .exti_banks = stm32mp_exti_banks,
    .bank_nr = ARRAY_SIZE(stm32mp_exti_banks),
    .desc_irqs = stm32mp13_desc_irq,
    };
#[no_mangle]
unsafe extern "C" fn stm32mp_exti_convert_type(d: *mut irq_data, type: c_uint, rtsr: *mut u32, ftsr: *mut u32) -> c_int {
    static int stm32mp_exti_convert_type(struct irq_data *d, unsigned int type, u32 *rtsr, u32 *ftsr)
    {
    let mut mask: u32 = BIT(d.hwirq % IRQS_PER_BANK);
    switch (type) {
    case IRQ_TYPE_EDGE_RISING:
// rtsr |= mask;
// ftsr &= ~mask;
    break;
    case IRQ_TYPE_EDGE_FALLING:
// rtsr &= ~mask;
// ftsr |= mask;
    break;
    case IRQ_TYPE_EDGE_BOTH:
// rtsr |= mask;
// ftsr |= mask;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stm32mp_chip_suspend(chip_data: *mut stm32mp_exti_chip_data, wake_active: u32) {
    static void stm32mp_chip_suspend(struct stm32mp_exti_chip_data *chip_data, u32 wake_active)
    {
    const struct stm32mp_exti_bank *bank = chip_data.reg_bank;
    void __iomem *base = chip_data.host_data.base;
// save rtsr, ftsr registers
    chip_data.rtsr_cache = readl_relaxed(base + bank.rtsr_ofst);
    chip_data.ftsr_cache = readl_relaxed(base + bank.ftsr_ofst);
    writel_relaxed(wake_active, base + bank.imr_ofst);
    }
#[no_mangle]
unsafe extern "C" fn stm32mp_chip_resume(chip_data: *mut stm32mp_exti_chip_data, mask_cache: u32) {
    static void stm32mp_chip_resume(struct stm32mp_exti_chip_data *chip_data, u32 mask_cache)
    {
    const struct stm32mp_exti_bank *bank = chip_data.reg_bank;
    void __iomem *base = chip_data.host_data.base;
// restore rtsr, ftsr, registers
    writel_relaxed(chip_data.rtsr_cache, base + bank.rtsr_ofst);
    writel_relaxed(chip_data.ftsr_cache, base + bank.ftsr_ofst);
    writel_relaxed(mask_cache, base + bank.imr_ofst);
    }
// directly set the target bit without reading first.
#[no_mangle]
pub unsafe extern "C" fn stm32mp_exti_write_bit(d: *mut irq_data, reg: u32) {
    static inline void stm32mp_exti_write_bit(struct irq_data *d, u32 reg)
    {
    struct stm32mp_exti_chip_data *chip_data = irq_data_get_irq_chip_data(d);
    void __iomem *base = chip_data.host_data.base;
    let mut val: u32 = BIT(d.hwirq % IRQS_PER_BANK);
    writel_relaxed(val, base + reg);
    }
#[no_mangle]
pub unsafe extern "C" fn stm32mp_exti_set_bit(d: *mut irq_data, reg: u32) -> u32 {
    static inline u32 stm32mp_exti_set_bit(struct irq_data *d, u32 reg)
    {
    struct stm32mp_exti_chip_data *chip_data = irq_data_get_irq_chip_data(d);
    void __iomem *base = chip_data.host_data.base;
    u32 val;
    val = readl_relaxed(base + reg);
    val |= BIT(d.hwirq % IRQS_PER_BANK);
    writel_relaxed(val, base + reg);
    return val;
    }
#[no_mangle]
pub unsafe extern "C" fn stm32mp_exti_clr_bit(d: *mut irq_data, reg: u32) -> u32 {
    static inline u32 stm32mp_exti_clr_bit(struct irq_data *d, u32 reg)
    {
    struct stm32mp_exti_chip_data *chip_data = irq_data_get_irq_chip_data(d);
    void __iomem *base = chip_data.host_data.base;
    u32 val;
    val = readl_relaxed(base + reg);
    val &= ~BIT(d.hwirq % IRQS_PER_BANK);
    writel_relaxed(val, base + reg);
    return val;
    }
#[no_mangle]
unsafe extern "C" fn stm32mp_exti_eoi(d: *mut irq_data) {
    static void stm32mp_exti_eoi(struct irq_data *d)
    {
    struct stm32mp_exti_chip_data *chip_data = irq_data_get_irq_chip_data(d);
    const struct stm32mp_exti_bank *bank = chip_data.reg_bank;
    raw_spin_lock(&chip_data.rlock);
    stm32mp_exti_write_bit(d, bank.rpr_ofst);
    stm32mp_exti_write_bit(d, bank.fpr_ofst);
    raw_spin_unlock(&chip_data.rlock);
    if (d.parent_data.chip)
    irq_chip_eoi_parent(d);
    }
#[no_mangle]
unsafe extern "C" fn stm32mp_exti_mask(d: *mut irq_data) {
    static void stm32mp_exti_mask(struct irq_data *d)
    {
    struct stm32mp_exti_chip_data *chip_data = irq_data_get_irq_chip_data(d);
    const struct stm32mp_exti_bank *bank = chip_data.reg_bank;
    raw_spin_lock(&chip_data.rlock);
    chip_data.mask_cache = stm32mp_exti_clr_bit(d, bank.imr_ofst);
    raw_spin_unlock(&chip_data.rlock);
    if (d.parent_data.chip)
    irq_chip_mask_parent(d);
    }
#[no_mangle]
unsafe extern "C" fn stm32mp_exti_unmask(d: *mut irq_data) {
    static void stm32mp_exti_unmask(struct irq_data *d)
    {
    struct stm32mp_exti_chip_data *chip_data = irq_data_get_irq_chip_data(d);
    const struct stm32mp_exti_bank *bank = chip_data.reg_bank;
    raw_spin_lock(&chip_data.rlock);
    chip_data.mask_cache = stm32mp_exti_set_bit(d, bank.imr_ofst);
    raw_spin_unlock(&chip_data.rlock);
    if (d.parent_data.chip)
    irq_chip_unmask_parent(d);
    }
#[no_mangle]
unsafe extern "C" fn stm32mp_exti_set_type(d: *mut irq_data, type: c_uint) -> c_int {
    static int stm32mp_exti_set_type(struct irq_data *d, unsigned int type)
    {
    struct stm32mp_exti_chip_data *chip_data = irq_data_get_irq_chip_data(d);
    const struct stm32mp_exti_bank *bank = chip_data.reg_bank;
    struct hwspinlock *hwlock = chip_data.host_data.hwlock;
    void __iomem *base = chip_data.host_data.base;
    u32 rtsr, ftsr;
    int err;
    raw_spin_lock(&chip_data.rlock);
    if (hwlock) {
    err = hwspin_lock_timeout_in_atomic(hwlock, HWSPNLCK_TIMEOUT);
    if (err) {
    pr_err("%s can't get hwspinlock (%d)\n", __func__, err);
    goto unlock;
    }
    }
    rtsr = readl_relaxed(base + bank.rtsr_ofst);
    ftsr = readl_relaxed(base + bank.ftsr_ofst);
    err = stm32mp_exti_convert_type(d, type, &rtsr, &ftsr);
    if (!err) {
    writel_relaxed(rtsr, base + bank.rtsr_ofst);
    writel_relaxed(ftsr, base + bank.ftsr_ofst);
    }
    if (hwlock)
    hwspin_unlock_in_atomic(hwlock);
    unlock:
    raw_spin_unlock(&chip_data.rlock);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn stm32mp_exti_set_wake(d: *mut irq_data, on: c_uint) -> c_int {
    static int stm32mp_exti_set_wake(struct irq_data *d, unsigned int on)
    {
    struct stm32mp_exti_chip_data *chip_data = irq_data_get_irq_chip_data(d);
    let mut mask: u32 = BIT(d.hwirq % IRQS_PER_BANK);
    raw_spin_lock(&chip_data.rlock);
    if (on)
    chip_data.wake_active |= mask;
    else
    chip_data.wake_active &= ~mask;
    raw_spin_unlock(&chip_data.rlock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stm32mp_exti_set_affinity(d: *mut irq_data, dest: *const cpumask, force: bool) -> c_int {
    static int stm32mp_exti_set_affinity(struct irq_data *d, const struct cpumask *dest, bool force)
    {
    if (d.parent_data.chip)
    return irq_chip_set_affinity_parent(d, dest, force);
    return IRQ_SET_MASK_OK_DONE;
    }
#[no_mangle]
unsafe extern "C" fn stm32mp_exti_suspend(dev: *mut device) -> c_int {
    static int stm32mp_exti_suspend(struct device *dev)
    {
    struct stm32mp_exti_host_data *host_data = dev_get_drvdata(dev);
    struct stm32mp_exti_chip_data *chip_data;
    int i;
    for (i = 0; i < host_data.drv_data.bank_nr; i++) {
    chip_data = &host_data.chips_data[i];
    stm32mp_chip_suspend(chip_data, chip_data.wake_active);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stm32mp_exti_resume(dev: *mut device) -> c_int {
    static int stm32mp_exti_resume(struct device *dev)
    {
    struct stm32mp_exti_host_data *host_data = dev_get_drvdata(dev);
    struct stm32mp_exti_chip_data *chip_data;
    int i;
    for (i = 0; i < host_data.drv_data.bank_nr; i++) {
    chip_data = &host_data.chips_data[i];
    stm32mp_chip_resume(chip_data, chip_data.mask_cache);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stm32mp_exti_retrigger(d: *mut irq_data) -> c_int {
    static int stm32mp_exti_retrigger(struct irq_data *d)
    {
    struct stm32mp_exti_chip_data *chip_data = irq_data_get_irq_chip_data(d);
    const struct stm32mp_exti_bank *bank = chip_data.reg_bank;
    void __iomem *base = chip_data.host_data.base;
    let mut mask: u32 = BIT(d.hwirq % IRQS_PER_BANK);
    writel_relaxed(mask, base + bank.swier_ofst);
    return 0;
    }
    static struct irq_chip stm32mp_exti_chip = {
    .name			= "stm32mp-exti",
    .irq_eoi		= stm32mp_exti_eoi,
    .irq_mask		= stm32mp_exti_mask,
    .irq_unmask		= stm32mp_exti_unmask,
    .irq_retrigger		= stm32mp_exti_retrigger,
    .irq_set_type		= stm32mp_exti_set_type,
    .irq_set_wake		= stm32mp_exti_set_wake,
    .flags			= IRQCHIP_MASK_ON_SUSPEND,
    .irq_set_affinity	= IS_ENABLED(CONFIG_SMP) ? stm32mp_exti_set_affinity : core::ptr::null_mut(),
    };
    static struct irq_chip stm32mp_exti_chip_direct = {
    .name			= "stm32mp-exti-direct",
    .irq_eoi		= irq_chip_eoi_parent,
    .irq_ack		= irq_chip_ack_parent,
    .irq_mask		= stm32mp_exti_mask,
    .irq_unmask		= stm32mp_exti_unmask,
    .irq_retrigger		= irq_chip_retrigger_hierarchy,
    .irq_set_type		= irq_chip_set_type_parent,
    .irq_set_wake		= stm32mp_exti_set_wake,
    .flags			= IRQCHIP_MASK_ON_SUSPEND,
    .irq_set_affinity	= IS_ENABLED(CONFIG_SMP) ? irq_chip_set_affinity_parent : core::ptr::null_mut(),
    };
    static int stm32mp_exti_domain_alloc(struct irq_domain *dm,
    unsigned int virq,
    unsigned int nr_irqs, void *data)
    {
    struct stm32mp_exti_host_data *host_data = dm.host_data;
    struct stm32mp_exti_chip_data *chip_data;
    struct irq_fwspec *fwspec = data;
    struct irq_fwspec p_fwspec;
    irq_hw_number_t hwirq;
    struct irq_chip *chip;
    u32 event_trg;
    u8 desc_irq;
    int bank;
    hwirq = fwspec.param[0];
    if (hwirq >= host_data.drv_data.bank_nr * IRQS_PER_BANK)
    return -EINVAL;
    bank  = hwirq / IRQS_PER_BANK;
    chip_data = &host_data.chips_data[bank];
// Check if event is reserved (Secure)
    if (chip_data.event_reserved & BIT(hwirq % IRQS_PER_BANK)) {
    dev_err(host_data.dev, "event %lu is reserved, secure\n", hwirq);
    return -EPERM;
    }
    event_trg = readl_relaxed(host_data.base + chip_data.reg_bank.trg_ofst);
    chip = (event_trg & BIT(hwirq % IRQS_PER_BANK)) ?
    &stm32mp_exti_chip : &stm32mp_exti_chip_direct;
    irq_domain_set_hwirq_and_chip(dm, virq, hwirq, chip, chip_data);
    if (host_data.dt_has_irqs_desc) {
    struct of_phandle_args out_irq;
    int ret;
    ret = of_irq_parse_one(host_data.dev.of_node, hwirq, &out_irq);
    if (ret)
    return ret;
// we only support one parent, so far
    if (of_fwnode_handle(out_irq.np) != dm.parent.fwnode)
    return -EINVAL;
    of_phandle_args_to_fwspec(out_irq.np, out_irq.args,
    out_irq.args_count, &p_fwspec);
    return irq_domain_alloc_irqs_parent(dm, virq, 1, &p_fwspec);
    }
    if (!host_data.drv_data.desc_irqs)
    return -EINVAL;
    desc_irq = host_data.drv_data.desc_irqs[hwirq];
    if (desc_irq != EXTI_INVALID_IRQ) {
    p_fwspec.fwnode = dm.parent.fwnode;
    p_fwspec.param_count = 3;
    p_fwspec.param[0] = GIC_SPI;
    p_fwspec.param[1] = desc_irq;
    p_fwspec.param[2] = IRQ_TYPE_LEVEL_HIGH;
    return irq_domain_alloc_irqs_parent(dm, virq, 1, &p_fwspec);
    }
    return 0;
    }
    static struct stm32mp_exti_chip_data *stm32mp_exti_chip_init(struct stm32mp_exti_host_data *h_data,
    u32 bank_idx, struct device_node *node)
    {
    struct stm32mp_exti_chip_data *chip_data;
    const struct stm32mp_exti_bank *bank;
    void __iomem *base = h_data.base;
    bank = h_data.drv_data.exti_banks[bank_idx];
    chip_data = &h_data.chips_data[bank_idx];
    chip_data.host_data = h_data;
    chip_data.reg_bank = bank;
    raw_spin_lock_init(&chip_data.rlock);
//
// This IP has no reset, so after hot reboot we should
// clear registers to avoid residue
//
    writel_relaxed(0, base + bank.imr_ofst);
// reserve Secure events
    chip_data.event_reserved = readl_relaxed(base + bank.seccfgr_ofst);
    pr_info("%pOF: bank%d\n", node, bank_idx);
    return chip_data;
    }
    static const struct irq_domain_ops stm32mp_exti_domain_ops = {
    .alloc	= stm32mp_exti_domain_alloc,
    .free	= irq_domain_free_irqs_common,
    .xlate = irq_domain_xlate_twocell,
    };
#[no_mangle]
unsafe extern "C" fn stm32mp_exti_check_rif(host_data: *mut stm32mp_exti_host_data) {
    static void stm32mp_exti_check_rif(struct stm32mp_exti_host_data *host_data)
    {
    unsigned int bank, i, event;
    u32 cid, cidcfgr, hwcfgr1;
// quit on CID not supported
    hwcfgr1 = readl_relaxed(host_data.base + EXTI_HWCFGR1);
    if ((hwcfgr1 & EXTI_HWCFGR1_CIDWIDTH_MASK) == 0)
    return;
    for (bank = 0; bank < host_data.drv_data.bank_nr; bank++) {
    for (i = 0; i < IRQS_PER_BANK; i++) {
    event = bank * IRQS_PER_BANK + i;
    cidcfgr = readl_relaxed(host_data.base + EXTI_EnCIDCFGR(event));
    cid = (cidcfgr & EXTI_CIDCFGR_CID_MASK) >> EXTI_CIDCFGR_CID_SHIFT;
    if ((cidcfgr & EXTI_CIDCFGR_CFEN_MASK) && cid != EXTI_CID1)
    host_data.chips_data[bank].event_reserved |= BIT(i);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn stm32mp_exti_remove_irq(data: *mut c_void) {
    static void stm32mp_exti_remove_irq(void *data)
    {
    struct irq_domain *domain = data;
    irq_domain_remove(domain);
    }
#[no_mangle]
unsafe extern "C" fn stm32mp_exti_probe(pdev: *mut platform_device) -> c_int {
    static int stm32mp_exti_probe(struct platform_device *pdev)
    {
    const struct stm32mp_exti_drv_data *drv_data;
    struct irq_domain *parent_domain, *domain;
    struct stm32mp_exti_host_data *host_data;
    struct device *dev = &pdev.dev;
    struct device_node *np = dev.of_node;
    int ret, i;
    host_data = devm_kzalloc(dev, sizeof(*host_data), GFP_KERNEL);
    if (!host_data)
    return -ENOMEM;
    dev_set_drvdata(dev, host_data);
    host_data.dev = dev;
// check for optional hwspinlock which may be not available yet
    ret = of_hwspin_lock_get_id(np, 0);
    if (ret == -EPROBE_DEFER)
// hwspinlock framework not yet ready
    return ret;
    if (ret >= 0) {
    host_data.hwlock = devm_hwspin_lock_request_specific(dev, ret);
    if (!host_data.hwlock) {
    dev_err(dev, "Failed to request hwspinlock\n");
    return -EINVAL;
    }
    } else if (ret != -ENOENT) {
// note: ENOENT is a valid case (means 'no hwspinlock')
    dev_err(dev, "Failed to get hwspinlock\n");
    return ret;
    }
// initialize host_data
    drv_data = of_device_get_match_data(dev);
    if (!drv_data) {
    dev_err(dev, "no of match data\n");
    return -ENODEV;
    }
    host_data.drv_data = drv_data;
    host_data.chips_data = devm_kcalloc(dev, drv_data.bank_nr,
    sizeof(*host_data.chips_data),
    GFP_KERNEL);
    if (!host_data.chips_data)
    return -ENOMEM;
    host_data.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(host_data.base))
    return PTR_ERR(host_data.base);
    for (i = 0; i < drv_data.bank_nr; i++)
    stm32mp_exti_chip_init(host_data, i, np);
    stm32mp_exti_check_rif(host_data);
    parent_domain = irq_find_host(of_irq_find_parent(np));
    if (!parent_domain) {
    dev_err(dev, "GIC interrupt-parent not found\n");
    return -EINVAL;
    }
    domain = irq_domain_create_hierarchy(parent_domain, 0, drv_data.bank_nr * IRQS_PER_BANK,
    dev_fwnode(dev), &stm32mp_exti_domain_ops, host_data);
    if (!domain) {
    dev_err(dev, "Could not register exti domain\n");
    return -ENOMEM;
    }
    ret = devm_add_action_or_reset(dev, stm32mp_exti_remove_irq, domain);
    if (ret)
    return ret;
    host_data.dt_has_irqs_desc = of_property_present(np, "interrupts-extended");
    return 0;
    }
    static const struct of_device_id stm32mp_exti_ids[] = {
    { .compatible = "st,stm32mp1-exti", .data = &stm32mp1_drv_data},
    { .compatible = "st,stm32mp13-exti", .data = &stm32mp13_drv_data},
    {},
    };
    MODULE_DEVICE_TABLE(of, stm32mp_exti_ids);
    static const struct dev_pm_ops stm32mp_exti_dev_pm_ops = {
    NOIRQ_SYSTEM_SLEEP_PM_OPS(stm32mp_exti_suspend, stm32mp_exti_resume)
    };
    static struct platform_driver stm32mp_exti_driver = {
    .probe		= stm32mp_exti_probe,
    .driver		= {
    .name		= "stm32mp_exti",
    .of_match_table	= stm32mp_exti_ids,
    .pm		= &stm32mp_exti_dev_pm_ops,
    },
    };
    module_platform_driver(stm32mp_exti_driver);
    MODULE_AUTHOR("Maxime Coquelin <mcoquelin.stm32@gmail.com>");
    MODULE_DESCRIPTION("STM32MP EXTI driver");
    MODULE_LICENSE("GPL");
