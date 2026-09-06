//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-renesas-rzv2h.c
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
// Renesas RZ/V2H(P) ICU Driver
//
// Based on irq-renesas-rzg2l.c
//
// Copyright (C) 2024 Renesas Electronics Corporation.
//
// Author: Fabrizio Castro <fabrizio.castro.jz@renesas.com>
//

// DT "interrupts" indexes
pub const ICU_IRQ_START: c_int = 1;
pub const ICU_IRQ_COUNT: c_int = 16;

pub const ICU_TINT_COUNT: c_int = 32;

pub const ICU_CA55_INT_COUNT: c_int = 4;

pub const ICU_ERR_INT_COUNT: c_int = 1;

// Registers
pub const ICU_NSCNT: c_uint = 0x00;
pub const ICU_NSCLR: c_uint = 0x04;
pub const ICU_NITSR: c_uint = 0x08;
pub const ICU_ISCTR: c_uint = 0x10;
pub const ICU_ISCLR: c_uint = 0x14;
pub const ICU_IITSR: c_uint = 0x18;
pub const ICU_TSCTR: c_uint = 0x20;
pub const ICU_TSCLR: c_uint = 0x24;

pub const ICU_SWINT: c_uint = 0x130;

pub const ICU_SWPE: c_uint = 0x370;

// NMI
pub const ICU_NMI_EDGE_FALLING: c_int = 0;
pub const ICU_NMI_EDGE_RISING: c_int = 1;

// IRQ
pub const ICU_IRQ_LEVEL_LOW: c_int = 0;
pub const ICU_IRQ_EDGE_FALLING: c_int = 1;
pub const ICU_IRQ_EDGE_RISING: c_int = 2;
pub const ICU_IRQ_EDGE_BOTH: c_int = 3;

// TINT
pub const ICU_TINT_EDGE_RISING: c_int = 0;
pub const ICU_TINT_EDGE_FALLING: c_int = 1;
pub const ICU_TINT_LEVEL_HIGH: c_int = 2;
pub const ICU_TINT_LEVEL_LOW: c_int = 3;

    ({\
    typeof(field_width) (_field_width) = (field_width); \
    ICU_TSSR_TSSEL_PREP((GENMASK(((_field_width) - 2), 0)), (n), _field_width); \
    })

    ({\
    typeof(field_width) (_field_width) = (field_width); \
    BIT((_field_width) - 1) << ((n) * (_field_width)); \
    })

pub const ICU_RZG3E_TINT_OFFSET: c_uint = 0x800;
pub const ICU_RZG3E_TSSEL_MAX_VAL: c_uint = 0x8c;
pub const ICU_RZV2H_TSSEL_MAX_VAL: c_uint = 0x55;
pub const ICU_SWPE_NUM: c_int = 16;
pub const ICU_NUM_BE: c_int = 4;
pub const ICU_NUM_A55ERR: c_int = 4;
//
// struct rzv2h_irqc_reg_cache - registers cache (necessary for suspend/resume)
// @nitsr: ICU_NITSR register
// @iitsr: ICU_IITSR register
// @titsr: ICU_TITSR registers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rzv2h_irqc_reg_cache {
    pub nitsr: u32,
    pub iitsr: u32,
    pub titsr: [u32; 2],
}

//
// struct rzv2h_hw_info - Interrupt Control Unit controller hardware info structure.
// @tssel_lut:		TINT lookup table
// @t_offs:		TINT offset
// @max_tssel:		TSSEL max value
// @field_width:	TSSR field width
// @ecc_start:		Start index of ECC RAM interrupts
// @ecc_end:		End index of ECC RAM interrupts
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rzv2h_hw_info {
    pub tssel_lut: *const u8,
    pub t_offs: u16,
    pub max_tssel: u8,
    pub field_width: u8,
    pub ecc_start: u8,
    pub ecc_end: u8,
}

// DMAC

    << ICU_DMAC_DMAREQ_SHIFT(up))

    << ICU_DMAC_DMAREQ_SHIFT(up))
// DMAC ACK routing - 4 x 7-bit fields per 32-bit register, 8-bit spacing

//
// struct rzv2h_icu_priv - Interrupt Control Unit controller private data structure.
// @base:	Controller's base address
// @fwspec:	IRQ firmware specific data
// @lock:	Lock to serialize access to hardware registers
// @info:	Pointer to struct rzv2h_hw_info
// @cache:	Registers cache for suspend/resume
//
    static struct rzv2h_icu_priv {
    void __iomem			*base;
    struct irq_fwspec		fwspec[ICU_NUM_IRQ];
    raw_spinlock_t			lock;
    const struct rzv2h_hw_info	*info;
    struct rzv2h_irqc_reg_cache	cache;
    } *rzv2h_icu_data;
    void rzv2h_icu_register_dma_req(struct platform_device *icu_dev, u8 dmac_index, u8 dmac_channel,
    u16 req_no)
    {
    struct rzv2h_icu_priv *priv = platform_get_drvdata(icu_dev);
    u32 icu_dmksely, dmareq, dmareq_mask;
    u8 y, upper;
    y = dmac_channel / 2;
    upper = dmac_channel % 2;
    dmareq = ICU_DMAC_PREP_DMAREQ(req_no, upper);
    dmareq_mask = ICU_DMAC_DMAREQ_MASK(upper);
    guard(raw_spinlock_irqsave)(&priv.lock);
    icu_dmksely = readl(priv.base + ICU_DMkSELy(dmac_index, y));
    icu_dmksely = (icu_dmksely & ~dmareq_mask) | dmareq;
    writel(icu_dmksely, priv.base + ICU_DMkSELy(dmac_index, y));
    }
    EXPORT_SYMBOL_GPL(rzv2h_icu_register_dma_req);
//
// rzv2h_icu_register_dma_ack - Configure DMA ACK signal routing
// @icu_dev:      ICU platform device
// @dmac_index:   DMAC instance index (0-4)
// @dmac_channel: DMAC channel number (0-15), or RZV2H_ICU_DMAC_ACK_NO_DEFAULT
// to disconnect routing for a given ack_no
// @ack_no:       Peripheral ACK number (0-88) per RZ/G3E manual Table 4.6-28,
// used as index into ICU_DMACKSELk
//
// Routes the ACK signal of the peripheral identified by @ack_no to DMAC
// channel @dmac_channel of instance @dmac_index. When @dmac_channel is
// RZV2H_ICU_DMAC_ACK_NO_DEFAULT the field is reset, disconnecting any
// previously configured routing for that peripheral.
//
    void rzv2h_icu_register_dma_ack(struct platform_device *icu_dev, u8 dmac_index,
    u8 dmac_channel, u16 ack_no)
    {
    struct rzv2h_icu_priv *priv = platform_get_drvdata(icu_dev);
    let mut reg_idx: u8 = ack_no / 4;
    let mut field_idx: u8 = ack_no & 0x3;
    u8 dmac_ack_src = (dmac_channel == RZV2H_ICU_DMAC_ACK_NO_DEFAULT) ?
    RZV2H_ICU_DMAC_ACK_NO_DEFAULT :
    (dmac_index * 16 + dmac_channel);
    u32 val;
    guard(raw_spinlock_irqsave)(&priv.lock);
    val = readl(priv.base + ICU_DMACKSELk(reg_idx));
    val &= ~ICU_DMAC_DACK_FIELD_MASK(field_idx);
    val |= ICU_DMAC_PREP_DACK(dmac_ack_src, field_idx);
    writel(val, priv.base + ICU_DMACKSELk(reg_idx));
    }
    EXPORT_SYMBOL_GPL(rzv2h_icu_register_dma_ack);
    static inline struct rzv2h_icu_priv *irq_data_to_priv(struct irq_data *data)
    {
    return data.domain.host_data;
    }
#[no_mangle]
unsafe extern "C" fn rzv2h_icu_tint_eoi(d: *mut irq_data) {
    static void rzv2h_icu_tint_eoi(struct irq_data *d)
    {
    struct rzv2h_icu_priv *priv = irq_data_to_priv(d);
    let mut hw_irq: c_uint = irqd_to_hwirq(d);
    unsigned int tintirq_nr;
    u32 bit;
    if (!irqd_is_level_type(d)) {
    tintirq_nr = hw_irq - ICU_TINT_START;
    bit = BIT(tintirq_nr);
    writel_relaxed(bit, priv.base + priv.info.t_offs + ICU_TSCLR);
    }
    irq_chip_eoi_parent(d);
    }
#[no_mangle]
unsafe extern "C" fn rzv2h_icu_irq_eoi(d: *mut irq_data) {
    static void rzv2h_icu_irq_eoi(struct irq_data *d)
    {
    struct rzv2h_icu_priv *priv = irq_data_to_priv(d);
    let mut hw_irq: c_uint = irqd_to_hwirq(d);
    unsigned int tintirq_nr;
    u32 bit;
    if (!irqd_is_level_type(d)) {
    tintirq_nr = hw_irq - ICU_IRQ_START;
    bit = BIT(tintirq_nr);
    writel_relaxed(bit, priv.base + ICU_ISCLR);
    }
    irq_chip_eoi_parent(d);
    }
#[no_mangle]
unsafe extern "C" fn rzv2h_icu_nmi_eoi(d: *mut irq_data) {
    static void rzv2h_icu_nmi_eoi(struct irq_data *d)
    {
    struct rzv2h_icu_priv *priv = irq_data_to_priv(d);
    writel_relaxed(ICU_NSCLR_NCLR, priv.base + ICU_NSCLR);
    irq_chip_eoi_parent(d);
    }
#[no_mangle]
unsafe extern "C" fn rzv2h_tint_irq_endisable(d: *mut irq_data, enable: bool) {
    static void rzv2h_tint_irq_endisable(struct irq_data *d, bool enable)
    {
    struct rzv2h_icu_priv *priv = irq_data_to_priv(d);
    let mut hw_irq: c_uint = irqd_to_hwirq(d);
    u32 tint_nr, tssel_n, k, tssr;
    u8 nr_tint;
    tint_nr = hw_irq - ICU_TINT_START;
    nr_tint = 32 / priv.info.field_width;
    k = tint_nr / nr_tint;
    tssel_n = tint_nr % nr_tint;
    guard(raw_spinlock)(&priv.lock);
    tssr = readl_relaxed(priv.base + priv.info.t_offs + ICU_TSSR(k));
    if (enable)
    tssr |= ICU_TSSR_TIEN(tssel_n, priv.info.field_width);
    else
    tssr &= ~ICU_TSSR_TIEN(tssel_n, priv.info.field_width);
    writel_relaxed(tssr, priv.base + priv.info.t_offs + ICU_TSSR(k));
//
// A glitch in the edge detection circuit can cause a spurious
// interrupt. Clear the status flag after setting the ICU_TSSRk
// registers, which is recommended by the hardware manual as a
// countermeasure.
//
    writel_relaxed(BIT(tint_nr), priv.base + priv.info.t_offs + ICU_TSCLR);
    }
#[no_mangle]
unsafe extern "C" fn rzv2h_icu_tint_disable(d: *mut irq_data) {
    static void rzv2h_icu_tint_disable(struct irq_data *d)
    {
    irq_chip_disable_parent(d);
    rzv2h_tint_irq_endisable(d, false);
    }
#[no_mangle]
unsafe extern "C" fn rzv2h_icu_tint_enable(d: *mut irq_data) {
    static void rzv2h_icu_tint_enable(struct irq_data *d)
    {
    rzv2h_tint_irq_endisable(d, true);
    irq_chip_enable_parent(d);
    }
#[no_mangle]
unsafe extern "C" fn rzv2h_nmi_set_type(d: *mut irq_data, type: c_uint) -> c_int {
    static int rzv2h_nmi_set_type(struct irq_data *d, unsigned int type)
    {
    struct rzv2h_icu_priv *priv = irq_data_to_priv(d);
    u32 sense;
    switch (type & IRQ_TYPE_SENSE_MASK) {
    case IRQ_TYPE_EDGE_FALLING:
    sense = ICU_NMI_EDGE_FALLING;
    break;
    case IRQ_TYPE_EDGE_RISING:
    sense = ICU_NMI_EDGE_RISING;
    break;
    default:
    return -EINVAL;
    }
    writel_relaxed(sense, priv.base + ICU_NITSR);
    return irq_chip_set_type_parent(d, IRQ_TYPE_LEVEL_HIGH);
    }
#[no_mangle]
unsafe extern "C" fn rzv2h_clear_irq_int(priv: *mut rzv2h_icu_priv, hwirq: c_uint) {
    static void rzv2h_clear_irq_int(struct rzv2h_icu_priv *priv, unsigned int hwirq)
    {
    let mut irq_nr: c_uint = hwirq - ICU_IRQ_START;
    u32 isctr, iitsr, iitsel;
    let mut bit: u32 = BIT(irq_nr);
    isctr = readl_relaxed(priv.base + ICU_ISCTR);
    iitsr = readl_relaxed(priv.base + ICU_IITSR);
    iitsel = ICU_IITSR_IITSEL_GET(iitsr, irq_nr);
//
// When level sensing is used, the interrupt flag gets automatically cleared when the
// interrupt signal is de-asserted by the source of the interrupt request, therefore clear
// the interrupt only for edge triggered interrupts.
//
    if ((isctr & bit) && (iitsel != ICU_IRQ_LEVEL_LOW))
    writel_relaxed(bit, priv.base + ICU_ISCLR);
    }
#[no_mangle]
unsafe extern "C" fn rzv2h_irq_set_type(d: *mut irq_data, type: c_uint) -> c_int {
    static int rzv2h_irq_set_type(struct irq_data *d, unsigned int type)
    {
    struct rzv2h_icu_priv *priv = irq_data_to_priv(d);
    let mut hwirq: c_uint = irqd_to_hwirq(d);
    let mut irq_nr: u32 = hwirq - ICU_IRQ_START;
    u32 iitsr, sense;
    switch (type & IRQ_TYPE_SENSE_MASK) {
    case IRQ_TYPE_LEVEL_LOW:
    sense = ICU_IRQ_LEVEL_LOW;
    break;
    case IRQ_TYPE_EDGE_FALLING:
    sense = ICU_IRQ_EDGE_FALLING;
    break;
    case IRQ_TYPE_EDGE_RISING:
    sense = ICU_IRQ_EDGE_RISING;
    break;
    case IRQ_TYPE_EDGE_BOTH:
    sense = ICU_IRQ_EDGE_BOTH;
    break;
    default:
    return -EINVAL;
    }
    scoped_guard(raw_spinlock, &priv.lock) {
    iitsr = readl_relaxed(priv.base + ICU_IITSR);
    iitsr &= ~ICU_IITSR_IITSEL_MASK(irq_nr);
    iitsr |= ICU_IITSR_IITSEL_PREP(sense, irq_nr);
    rzv2h_clear_irq_int(priv, hwirq);
    writel_relaxed(iitsr, priv.base + ICU_IITSR);
    }
    return irq_chip_set_type_parent(d, IRQ_TYPE_LEVEL_HIGH);
    }
#[no_mangle]
unsafe extern "C" fn rzv2h_clear_tint_int(priv: *mut rzv2h_icu_priv, hwirq: c_uint) {
    static void rzv2h_clear_tint_int(struct rzv2h_icu_priv *priv, unsigned int hwirq)
    {
    let mut tint_nr: c_uint = hwirq - ICU_TINT_START;
    let mut titsel_n: c_int = ICU_TITSR_TITSEL_N(tint_nr);
    u32 tsctr, titsr, titsel;
    let mut bit: u32 = BIT(tint_nr);
    let mut k: c_int = tint_nr / 16;
    tsctr = readl_relaxed(priv.base + priv.info.t_offs + ICU_TSCTR);
    titsr = readl_relaxed(priv.base + priv.info.t_offs + ICU_TITSR(k));
    titsel = ICU_TITSR_TITSEL_GET(titsr, titsel_n);
//
// Writing 1 to the corresponding flag from register ICU_TSCTR only has effect if
// TSTATn = 1b and if it's a rising edge or a falling edge interrupt.
//
    if ((tsctr & bit) && ((titsel == ICU_TINT_EDGE_RISING) ||
    (titsel == ICU_TINT_EDGE_FALLING)))
    writel_relaxed(bit, priv.base + priv.info.t_offs + ICU_TSCLR);
    }
#[no_mangle]
unsafe extern "C" fn rzv2h_tint_set_type(d: *mut irq_data, type: c_uint) -> c_int {
    static int rzv2h_tint_set_type(struct irq_data *d, unsigned int type)
    {
    u32 titsr, titsr_k, titsel_n, tien;
    struct rzv2h_icu_priv *priv;
    u32 tssr, tssr_k, tssel_n;
    u32 titsr_cur, tssr_cur;
    unsigned int hwirq;
    u32 tint, sense;
    int tint_nr;
    u8 nr_tint;
    switch (type & IRQ_TYPE_SENSE_MASK) {
    case IRQ_TYPE_LEVEL_LOW:
    sense = ICU_TINT_LEVEL_LOW;
    break;
    case IRQ_TYPE_LEVEL_HIGH:
    sense = ICU_TINT_LEVEL_HIGH;
    break;
    case IRQ_TYPE_EDGE_RISING:
    sense = ICU_TINT_EDGE_RISING;
    break;
    case IRQ_TYPE_EDGE_FALLING:
    sense = ICU_TINT_EDGE_FALLING;
    break;
    default:
    return -EINVAL;
    }
    priv = irq_data_to_priv(d);
    tint = (u32)(uintptr_t)irq_data_get_irq_chip_data(d);
    if (tint > priv.info.max_tssel)
    return -EINVAL;
    if (priv.info.tssel_lut)
    tint = priv.info.tssel_lut[tint];
    hwirq = irqd_to_hwirq(d);
    tint_nr = hwirq - ICU_TINT_START;
    nr_tint = 32 / priv.info.field_width;
    tssr_k = tint_nr / nr_tint;
    tssel_n = tint_nr % nr_tint;
    tien = ICU_TSSR_TIEN(tssel_n, priv.info.field_width);
    titsr_k = ICU_TITSR_K(tint_nr);
    titsel_n = ICU_TITSR_TITSEL_N(tint_nr);
    scoped_guard(raw_spinlock, &priv.lock) {
    tssr = readl_relaxed(priv.base + priv.info.t_offs + ICU_TSSR(tssr_k));
    titsr = readl_relaxed(priv.base + priv.info.t_offs + ICU_TITSR(titsr_k));
    tssr_cur = field_get(ICU_TSSR_TSSEL_MASK(tssel_n, priv.info.field_width), tssr);
    titsr_cur = field_get(ICU_TITSR_TITSEL_MASK(titsel_n), titsr);
    if (tssr_cur == tint && titsr_cur == sense)
    goto set_parent_type;
    tssr &= ~(ICU_TSSR_TSSEL_MASK(tssel_n, priv.info.field_width) | tien);
    tssr |= ICU_TSSR_TSSEL_PREP(tint, tssel_n, priv.info.field_width);
    writel_relaxed(tssr, priv.base + priv.info.t_offs + ICU_TSSR(tssr_k));
    titsr &= ~ICU_TITSR_TITSEL_MASK(titsel_n);
    titsr |= ICU_TITSR_TITSEL_PREP(sense, titsel_n);
    writel_relaxed(titsr, priv.base + priv.info.t_offs + ICU_TITSR(titsr_k));
    rzv2h_clear_tint_int(priv, hwirq);
    writel_relaxed(tssr | tien, priv.base + priv.info.t_offs + ICU_TSSR(tssr_k));
    }
    set_parent_type:
    return irq_chip_set_type_parent(d, IRQ_TYPE_LEVEL_HIGH);
    }
    static int rzv2h_icu_swint_set_irqchip_state(struct irq_data *d, enum irqchip_irq_state which,
    bool state)
    {
    let mut hwirq: c_uint = irqd_to_hwirq(d);
    struct rzv2h_icu_priv *priv;
    unsigned int bit;
    if (which != IRQCHIP_STATE_PENDING)
    return irq_chip_set_parent_state(d, which, state);
    if (!state)
    return 0;
    priv = irq_data_to_priv(d);
    bit = BIT(hwirq - ICU_CA55_INT_START);
// Trigger the software interrupt
    writel_relaxed(bit, priv.base + ICU_SWINT);
    return 0;
    }
    static int rzv2h_icu_swpe_set_irqchip_state(struct irq_data *d, enum irqchip_irq_state which,
    bool state)
    {
    struct rzv2h_icu_priv *priv;
    unsigned int bit;
    static u8 swpe;
    if (which != IRQCHIP_STATE_PENDING)
    return irq_chip_set_parent_state(d, which, state);
    if (!state)
    return 0;
    priv = irq_data_to_priv(d);
    bit = BIT(swpe);
//
// SWPE has 16 bits; the bit position is rotated on each trigger
// and wraps around once all bits have been used.
//
    if (++swpe >= ICU_SWPE_NUM)
    swpe = 0;
// Trigger the pseudo error interrupt
    writel_relaxed(bit, priv.base + ICU_SWPE);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rzv2h_irqc_irq_suspend(data: *mut c_void) -> c_int {
    static int rzv2h_irqc_irq_suspend(void *data)
    {
    struct rzv2h_irqc_reg_cache *cache = &rzv2h_icu_data.cache;
    void __iomem *base = rzv2h_icu_data.base;
    cache.nitsr = readl_relaxed(base + ICU_NITSR);
    cache.iitsr = readl_relaxed(base + ICU_IITSR);
    for (unsigned int i = 0; i < 2; i++)
    cache.titsr[i] = readl_relaxed(base + rzv2h_icu_data.info.t_offs + ICU_TITSR(i));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rzv2h_irqc_irq_resume(data: *mut c_void) {
    static void rzv2h_irqc_irq_resume(void *data)
    {
    struct rzv2h_irqc_reg_cache *cache = &rzv2h_icu_data.cache;
    void __iomem *base = rzv2h_icu_data.base;
//
// Restore only interrupt type. TSSRx will be restored at the
// request of pin controller to avoid spurious interrupts due
// to invalid PIN states.
//
    for (unsigned int i = 0; i < 2; i++)
    writel_relaxed(cache.titsr[i], base + rzv2h_icu_data.info.t_offs + ICU_TITSR(i));
    writel_relaxed(cache.iitsr, base + ICU_IITSR);
    writel_relaxed(cache.nitsr, base + ICU_NITSR);
    }
    static const struct syscore_ops rzv2h_irqc_syscore_ops = {
    .suspend	= rzv2h_irqc_irq_suspend,
    .resume		= rzv2h_irqc_irq_resume,
    };
    static struct syscore rzv2h_irqc_syscore = {
    .ops = &rzv2h_irqc_syscore_ops,
    };
    static const struct irq_chip rzv2h_icu_tint_chip = {
    .name			= "rzv2h-icu",
    .irq_eoi		= rzv2h_icu_tint_eoi,
    .irq_mask		= irq_chip_mask_parent,
    .irq_unmask		= irq_chip_unmask_parent,
    .irq_disable		= rzv2h_icu_tint_disable,
    .irq_enable		= rzv2h_icu_tint_enable,
    .irq_get_irqchip_state	= irq_chip_get_parent_state,
    .irq_set_irqchip_state	= irq_chip_set_parent_state,
    .irq_retrigger		= irq_chip_retrigger_hierarchy,
    .irq_set_type		= rzv2h_tint_set_type,
    .irq_set_affinity	= irq_chip_set_affinity_parent,
    .flags			= IRQCHIP_MASK_ON_SUSPEND |
    IRQCHIP_SET_TYPE_MASKED |
    IRQCHIP_SKIP_SET_WAKE,
    };
    static const struct irq_chip rzv2h_icu_irq_chip = {
    .name			= "rzv2h-icu",
    .irq_eoi		= rzv2h_icu_irq_eoi,
    .irq_mask		= irq_chip_mask_parent,
    .irq_unmask		= irq_chip_unmask_parent,
    .irq_disable		= irq_chip_disable_parent,
    .irq_enable		= irq_chip_enable_parent,
    .irq_get_irqchip_state	= irq_chip_get_parent_state,
    .irq_set_irqchip_state	= irq_chip_set_parent_state,
    .irq_retrigger		= irq_chip_retrigger_hierarchy,
    .irq_set_type		= rzv2h_irq_set_type,
    .irq_set_affinity	= irq_chip_set_affinity_parent,
    .flags			= IRQCHIP_MASK_ON_SUSPEND |
    IRQCHIP_SET_TYPE_MASKED |
    IRQCHIP_SKIP_SET_WAKE,
    };
    static const struct irq_chip rzv2h_icu_nmi_chip = {
    .name			= "rzv2h-icu",
    .irq_eoi		= rzv2h_icu_nmi_eoi,
    .irq_mask		= irq_chip_mask_parent,
    .irq_unmask		= irq_chip_unmask_parent,
    .irq_disable		= irq_chip_disable_parent,
    .irq_enable		= irq_chip_enable_parent,
    .irq_get_irqchip_state	= irq_chip_get_parent_state,
    .irq_set_irqchip_state	= irq_chip_set_parent_state,
    .irq_retrigger		= irq_chip_retrigger_hierarchy,
    .irq_set_type		= rzv2h_nmi_set_type,
    .irq_set_affinity	= irq_chip_set_affinity_parent,
    .flags			= IRQCHIP_MASK_ON_SUSPEND |
    IRQCHIP_SET_TYPE_MASKED |
    IRQCHIP_SKIP_SET_WAKE,
    };
    static const struct irq_chip rzv2h_icu_swint_chip = {
    .name			= "rzv2h-icu",
    .irq_eoi		= irq_chip_eoi_parent,
    .irq_mask		= irq_chip_mask_parent,
    .irq_unmask		= irq_chip_unmask_parent,
    .irq_disable		= irq_chip_disable_parent,
    .irq_enable		= irq_chip_enable_parent,
    .irq_get_irqchip_state	= irq_chip_get_parent_state,
    .irq_set_irqchip_state	= rzv2h_icu_swint_set_irqchip_state,
    .irq_retrigger		= irq_chip_retrigger_hierarchy,
    .irq_set_type		= irq_chip_set_type_parent,
    .irq_set_affinity	= irq_chip_set_affinity_parent,
    .flags			= IRQCHIP_MASK_ON_SUSPEND |
    IRQCHIP_SET_TYPE_MASKED |
    IRQCHIP_SKIP_SET_WAKE,
    };
    static const struct irq_chip rzv2h_icu_swpe_err_chip = {
    .name			= "rzv2h-icu",
    .irq_eoi		= irq_chip_eoi_parent,
    .irq_mask		= irq_chip_mask_parent,
    .irq_unmask		= irq_chip_unmask_parent,
    .irq_disable		= irq_chip_disable_parent,
    .irq_enable		= irq_chip_enable_parent,
    .irq_get_irqchip_state	= irq_chip_get_parent_state,
    .irq_set_irqchip_state	= rzv2h_icu_swpe_set_irqchip_state,
    .irq_retrigger		= irq_chip_retrigger_hierarchy,
    .irq_set_type		= irq_chip_set_type_parent,
    .irq_set_affinity	= irq_chip_set_affinity_parent,
    .flags			= IRQCHIP_MASK_ON_SUSPEND |
    IRQCHIP_SET_TYPE_MASKED |
    IRQCHIP_SKIP_SET_WAKE,
    };

    static int rzv2h_icu_alloc(struct irq_domain *domain, unsigned int virq, unsigned int nr_irqs,
    void *arg)
    {
    struct rzv2h_icu_priv *priv = domain.host_data;
    const struct irq_chip *chip;
    let mut tint: c_ulong = 0;
    irq_hw_number_t hwirq;
    unsigned int type;
    int ret;
    ret = irq_domain_translate_twocell(domain, arg, &hwirq, &type);
    if (ret)
    return ret;
//
// For TINT interrupts the hwirq and TINT are encoded in
// fwspec->param[0].
// hwirq is embedded in bits 0-15.
// TINT is embedded in bits 16-31.
//
    tint = ICU_TINT_EXTRACT_GPIOINT(hwirq);
    if (tint || hwirq_within(hwirq, ICU_TINT)) {
    hwirq = ICU_TINT_EXTRACT_HWIRQ(hwirq);
    if (!hwirq_within(hwirq, ICU_TINT))
    return -EINVAL;
    chip = &rzv2h_icu_tint_chip;
    } else if (hwirq_within(hwirq, ICU_IRQ)) {
    chip = &rzv2h_icu_irq_chip;
    } else if (hwirq_within(hwirq, ICU_CA55_INT)) {
    chip = &rzv2h_icu_swint_chip;
    } else if (hwirq_within(hwirq, ICU_ERR_INT)) {
    chip = &rzv2h_icu_swpe_err_chip;
    } else {
    chip = &rzv2h_icu_nmi_chip;
    }
    if (hwirq > (ICU_NUM_IRQ - 1))
    return -EINVAL;
    ret = irq_domain_set_hwirq_and_chip(domain, virq, hwirq, chip, (void *)(uintptr_t)tint);
    if (ret)
    return ret;
    return irq_domain_alloc_irqs_parent(domain, virq, nr_irqs, &priv.fwspec[hwirq]);
    }
    static const struct irq_domain_ops rzv2h_icu_domain_ops = {
    .alloc		= rzv2h_icu_alloc,
    .free		= irq_domain_free_irqs_common,
    .translate	= irq_domain_translate_twocell,
    };
#[no_mangle]
unsafe extern "C" fn rzv2h_icu_parse_interrupts(priv: *mut rzv2h_icu_priv, np: *mut device_node) -> c_int {
    static int rzv2h_icu_parse_interrupts(struct rzv2h_icu_priv *priv, struct device_node *np)
    {
    struct of_phandle_args map;
    unsigned int i;
    int ret;
    for (i = 0; i < ICU_NUM_IRQ; i++) {
    ret = of_irq_parse_one(np, i, &map);
    if (ret)
    return ret;
    of_phandle_args_to_fwspec(np, map.args, map.args_count, &priv.fwspec[i]);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rzv2h_icu_error_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t rzv2h_icu_error_irq(int irq, void *data)
    {
    struct rzv2h_icu_priv *priv = data;
    const struct rzv2h_hw_info *hw_info = priv.info;
    void __iomem *base = priv.base;
    unsigned int k;
    u32 st;
// 1) Bus errors (BEISR0..3)
    for (k = 0; k < ICU_NUM_BE; k++) {
    st = readl(base + ICU_BEISR(k));
    if (!st)
    continue;
    writel_relaxed(st, base + ICU_BECLR(k));
    pr_warn("rzv2h-icu: BUS error k=%u status=0x%08x\n", k, st);
    }
// 2) ECC RAM errors (EREISR0..X)
    for (k = hw_info.ecc_start; k <= hw_info.ecc_end; k++) {
    st = readl(base + ICU_EREISR(k));
    if (!st)
    continue;
    writel_relaxed(st, base + ICU_ERCLR(k));
    pr_warn("rzv2h-icu: ECC error k=%u status=0x%08x\n", k, st);
    }
// 3) IP/CA55 error interrupt status (ERINTA55CTL0..3)
    for (k = 0; k < ICU_NUM_A55ERR; k++) {
    st = readl(base + ICU_ERINTA55CTL(k));
    if (!st)
    continue;
// there is no relation with status bits so clear all the interrupts
    writel_relaxed(0xffffffff, base + ICU_ERINTA55CRL(k));
    pr_warn("rzv2h-icu: IP/CA55 error k=%u status=0x%08x\n", k, st);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn rzv2h_icu_swint_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t rzv2h_icu_swint_irq(int irq, void *data)
    {
    let mut cpu: c_uint = (uintptr_t)data;
    pr_info("SWINT interrupt for CA55 core %u\n", cpu);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn rzv2h_icu_setup_irqs(pdev: *mut platform_device, irq_domain: *mut irq_domain) -> c_int {
    static int rzv2h_icu_setup_irqs(struct platform_device *pdev, struct irq_domain *irq_domain)
    {
    const struct rzv2h_hw_info *hw_info = rzv2h_icu_data.info;
    let mut irq_inject: bool = IS_ENABLED(CONFIG_GENERIC_IRQ_INJECTION);
    void __iomem *base = rzv2h_icu_data.base;
    struct device *dev = &pdev.dev;
    struct irq_fwspec fwspec;
    unsigned int i, virq;
    int ret;
    for (i = 0; i < ICU_CA55_INT_COUNT && irq_inject; i++) {
    fwspec.fwnode = irq_domain.fwnode;
    fwspec.param_count = 2;
    fwspec.param[0] = ICU_CA55_INT_START + i;
    fwspec.param[1] = IRQ_TYPE_EDGE_RISING;
    virq = irq_create_fwspec_mapping(&fwspec);
    if (!virq) {
    return dev_err_probe(dev, -EINVAL,
    "failed to create int-ca55-%u IRQ mapping\n", i);
    }
    ret = devm_request_irq(dev, virq, rzv2h_icu_swint_irq, 0, dev_name(dev),
    (void *)(uintptr_t)i);
    if (ret)
    return ret;
    }
// Unmask and clear all IP/CA55 error interrupts
    for (i = 0; i < ICU_NUM_A55ERR; i++) {
    writel_relaxed(0xffffff, base + ICU_ERINTA55CRL(i));
    writel_relaxed(0x0, base + ICU_ERINTA55MSK(i));
    }
// Clear all Bus errors
    for (i = 0; i < ICU_NUM_BE; i++)
    writel_relaxed(0xffffffff, base + ICU_BECLR(i));
// Clear all ECCRAM errors
    for (i = hw_info.ecc_start; i <= hw_info.ecc_end; i++)
    writel_relaxed(0xffffffff, base + ICU_ERCLR(i));
    fwspec.fwnode = irq_domain.fwnode;
    fwspec.param_count = 2;
    fwspec.param[0] = ICU_ERR_INT_START;
    fwspec.param[1] = IRQ_TYPE_LEVEL_HIGH;
    virq = irq_create_fwspec_mapping(&fwspec);
    if (!virq)
    return dev_err_probe(dev, -EINVAL, "failed to create icu-error-ca55 IRQ mapping\n");
    ret = devm_request_irq(dev, virq, rzv2h_icu_error_irq, 0, dev_name(dev), rzv2h_icu_data);
    if (ret)
    return ret;
    return 0;
    }
    static int rzv2h_icu_probe_common(struct platform_device *pdev, struct device_node *parent,
    const struct rzv2h_hw_info *hw_info)
    {
    struct irq_domain *irq_domain, *parent_domain;
    struct device_node *node = pdev.dev.of_node;
    struct device *dev = &pdev.dev;
    struct reset_control *resetn;
    int ret;
    parent_domain = irq_find_host(parent);
    if (!parent_domain)
    return dev_err_probe(dev, -ENODEV, "cannot find parent domain\n");
    rzv2h_icu_data = devm_kzalloc(dev, sizeof(*rzv2h_icu_data), GFP_KERNEL);
    if (!rzv2h_icu_data)
    return -ENOMEM;
    platform_set_drvdata(pdev, rzv2h_icu_data);
    rzv2h_icu_data.base = devm_of_iomap(dev, node, 0, core::ptr::null_mut());
    if (IS_ERR(rzv2h_icu_data.base))
    return PTR_ERR(rzv2h_icu_data.base);
    ret = rzv2h_icu_parse_interrupts(rzv2h_icu_data, node);
    if (ret)
    return dev_err_probe(dev, ret, "cannot parse interrupts\n");
    resetn = devm_reset_control_get_exclusive_deasserted(dev, core::ptr::null_mut());
    if (IS_ERR(resetn))
    return dev_err_probe(dev, PTR_ERR(resetn), "failed to acquire deasserted reset\n");
    ret = devm_pm_runtime_enable(dev);
    if (ret < 0)
    return dev_err_probe(dev, ret, "devm_pm_runtime_enable failed\n");
    ret = pm_runtime_resume_and_get(dev);
    if (ret < 0)
    return dev_err_probe(dev, ret, "pm_runtime_resume_and_get failed\n");
    raw_spin_lock_init(&rzv2h_icu_data.lock);
    irq_domain = irq_domain_create_hierarchy(parent_domain, 0, ICU_NUM_IRQ,
    dev_fwnode(dev), &rzv2h_icu_domain_ops,
    rzv2h_icu_data);
    if (!irq_domain) {
    dev_err(dev, "failed to add irq domain\n");
    ret = -ENOMEM;
    goto pm_put;
    }
    rzv2h_icu_data.info = hw_info;
    register_syscore(&rzv2h_irqc_syscore);
    ret = rzv2h_icu_setup_irqs(pdev, irq_domain);
    if (ret)
    goto pm_put;
//
// coccicheck complains about a missing put_device call before returning, but it's a false
// positive. We still need dev after successfully returning from this function.
//
    return 0;
    pm_put:
    pm_runtime_put_sync(dev);
    return ret;
    }
// Mapping based on port index on Table 4.2-6 and TSSEL bits on Table 4.6-4
    static const u8 rzg3e_tssel_lut[] = {
    81, 82, 83, 84, 85, 86, 87, 88,		/* P00-P07 */
    89, 90, 91, 92, 93, 94, 95, 96,		/* P10-P17 */
    111, 112,				/* P20-P21 */
    97, 98, 99, 100, 101, 102, 103, 104,	/* P30-P37 */
    105, 106, 107, 108, 109, 110,		/* P40-P45 */
    113, 114, 115, 116, 117, 118, 119,	/* P50-P56 */
    120, 121, 122, 123, 124, 125, 126,	/* P60-P66 */
    127, 128, 129, 130, 131, 132, 133, 134,	/* P70-P77 */
    135, 136, 137, 138, 139, 140,		/* P80-P85 */
    43, 44, 45, 46, 47, 48, 49, 50,		/* PA0-PA7 */
    51, 52, 53, 54, 55, 56, 57, 58,		/* PB0-PB7 */
    59, 60,	61,				/* PC0-PC2 */
    62, 63, 64, 65, 66, 67, 68, 69,		/* PD0-PD7 */
    70, 71, 72, 73, 74, 75, 76, 77,		/* PE0-PE7 */
    78, 79, 80,				/* PF0-PF2 */
    25, 26, 27, 28, 29, 30, 31, 32,		/* PG0-PG7 */
    33, 34, 35, 36, 37, 38,			/* PH0-PH5 */
    4, 5, 6, 7, 8,				/* PJ0-PJ4 */
    39, 40, 41, 42,				/* PK0-PK3 */
    9, 10, 11, 12, 21, 22, 23, 24,		/* PL0-PL7 */
    13, 14, 15, 16, 17, 18, 19, 20,		/* PM0-PM7 */
    0, 1, 2, 3				/* PS0-PS3 */
    };
    static const struct rzv2h_hw_info rzg3e_hw_params = {
    .tssel_lut	= rzg3e_tssel_lut,
    .t_offs		= ICU_RZG3E_TINT_OFFSET,
    .max_tssel	= ICU_RZG3E_TSSEL_MAX_VAL,
    .field_width	= 16,
    .ecc_start	= 1,
    .ecc_end	= 4,
    };
    static const struct rzv2h_hw_info rzv2n_hw_params = {
    .t_offs		= 0,
    .max_tssel	= ICU_RZV2H_TSSEL_MAX_VAL,
    .field_width	= 8,
    .ecc_start	= 0,
    .ecc_end	= 2,
    };
    static const struct rzv2h_hw_info rzv2h_hw_params = {
    .t_offs		= 0,
    .max_tssel	= ICU_RZV2H_TSSEL_MAX_VAL,
    .field_width	= 8,
    .ecc_start	= 0,
    .ecc_end	= 11,
    };
#[no_mangle]
unsafe extern "C" fn rzg3e_icu_probe(pdev: *mut platform_device, parent: *mut device_node) -> c_int {
    static int rzg3e_icu_probe(struct platform_device *pdev, struct device_node *parent)
    {
    return rzv2h_icu_probe_common(pdev, parent, &rzg3e_hw_params);
    }
#[no_mangle]
unsafe extern "C" fn rzv2n_icu_probe(pdev: *mut platform_device, parent: *mut device_node) -> c_int {
    static int rzv2n_icu_probe(struct platform_device *pdev, struct device_node *parent)
    {
    return rzv2h_icu_probe_common(pdev, parent, &rzv2n_hw_params);
    }
#[no_mangle]
unsafe extern "C" fn rzv2h_icu_probe(pdev: *mut platform_device, parent: *mut device_node) -> c_int {
    static int rzv2h_icu_probe(struct platform_device *pdev, struct device_node *parent)
    {
    return rzv2h_icu_probe_common(pdev, parent, &rzv2h_hw_params);
    }
    IRQCHIP_PLATFORM_DRIVER_BEGIN(rzv2h_icu)
    IRQCHIP_MATCH("renesas,r9a09g047-icu", rzg3e_icu_probe)
    IRQCHIP_MATCH("renesas,r9a09g056-icu", rzv2n_icu_probe)
    IRQCHIP_MATCH("renesas,r9a09g057-icu", rzv2h_icu_probe)
    IRQCHIP_PLATFORM_DRIVER_END(rzv2h_icu)
    MODULE_AUTHOR("Fabrizio Castro <fabrizio.castro.jz@renesas.com>");
    MODULE_DESCRIPTION("Renesas RZ/V2H(P) ICU Driver");
