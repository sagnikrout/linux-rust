//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/qcom-pdc.c
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
// Copyright (c) 2017-2019, The Linux Foundation. All rights reserved.
//

pub const PDC_MAX_IRQS: c_int = 256;

// Secure DRV register to configure the PDC mode via qcom_scm_io_writel()
pub const PDC_GPIO_INT_CTL_ENABLE: c_uint = 0xb2045e8;
pub const PDC_PASS_THROUGH_MODE: c_uint = 0x0;
pub const PDC_SECONDARY_MODE: c_uint = 0x1;
pub const PDC_DRV_SIZE: c_uint = 0x10000;
pub const PDC_VERSION_REG: c_uint = 0x1000;

    FIELD_PREP(PDC_VERSION_MINOR, (min)) | \
    FIELD_PREP(PDC_VERSION_STEP,  (step)))
// Notable PDC versions

//
// PDC Hardware registers layout per version:
//
// IRQ_ENABLE_BANK[b], b = 0....BITS_TO_BYTES(PDC_MAX_IRQS)
// IRQ_CFG[n], n = 0....PDC_MAX_IRQS
//
// +---------------------------------------------------------------+
// |       v2.7        |       v3.0        |       v3.2            |
// |---------------------------------------------------------------|
// |       BASE        |       BASE        |       BASE            |
// |---------------------------------------------------------------|
// |                                                               |
// |   IRQ_ENABLE_BANK | IRQ_ENABLE_BANK   |       NA              |
// |---------------------------------------------------------------|
// |   IRQ_CFG         | IRQ_CFG           | IRQ_CFG               |
// |                   |                   |                       |
// |                   |                   | [31:6] Unused         |
// |                   | [31:5] Unused     |    [5] GPIO_STATUS    |
// |                   |    [4] GPIO_STATUS|    [4] GPIO_MASK      |
// |   [31:3] Unused   |    [3] GPIO_MASK  |    [3] IRQ_ENABLE     |
// |    [0:2] Type     |  [0:2] Type       |  [0:2] Type           |
// |---------------------------------------------------------------|
// |   IRQ_PARAM       | IRQ_PARAM         | IRQ_PARAM             |
// |                   |                                           |
// |   [15:8] NUM_GPIO | [15:8] NUM_GPIO   | [15:8] NUM_GPIO       |
// |    [7:0] NUM_SPI  |  [7:0] NUM_SPI    |  [7:0] NUM_SPI        |
// +---------------------------------------------------------------+
//
// struct pdc_regs: PDC registers location
//
// @irq_en_reg:     IRQ_ENABLE_BANK register location
// @irq_cfg_reg:    IRQ_CFG register location
// @irq_param_reg:  IRQ_PARAM register location
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pdc_regs {
    pub irq_en_reg: u32,
    pub irq_cfg_reg: u32,
    pub irq_param_reg: u32,
}

//
// struct pdc_irq_cfg: bit fields for PDC IRQ_CFG register
//
// @gpio_irq_sts:   bit number for GPIO_STATUS field
// @gpio_irq_mask:  bit number for GPIO_MASK field
// @irq_enable:     bit number for IRQ_ENABLE field
// @irq_type:       GENMASK for IRQ_TYPE field
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pdc_irq_cfg {
    pub gpio_irq_sts: u32,
    pub gpio_irq_mask: u32,
    pub irq_enable: u32,
    pub irq_type: u32,
}

//
// struct pdc_desc: PDC driver state
//
// @base:           PDC base register for DRV2 / HLOS
// @prev_base:      PDC DRV1 base, applicable only for x1e RTL bug.
// @version:        PDC version
// @num_spis:       Total number of direct SPI interrupts
// @region:         PDC interrupt continuous range
// @region_cnt:     Total PDC ranges
// @mode:           PDC_PASS_THROUGH_MODE or PDC_SECONDARY_MODE
// @x1e_quirk:      x1e H/W Bug handling
// @lock:           lock for IRQ_ENABLE_BANK protection
// @regs:           PDC regs (IRQ_ENABLE_BANK and IRQ_CFG)
// @cfg_fields:     Fields of IRQ_CFG reg
// @enable_intr:    pointer to enable function based on PDC version
// @unmask_gpio:    pointer to GPIO irq unmask function
// @clear_gpio:     pointer to GPIO irq clear function
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pdc_desc {
    pub base: *mut void __iomem,
    pub prev_base: *mut void __iomem,
    pub version: u32,
    pub num_spis: u32,
    pub region: *mut pdc_pin_region,
    pub region_cnt: c_int,
    pub x1e_quirk: bool,
    pub mode: u8,
    pub lock: raw_spinlock_t,
    pub regs: *const pdc_regs,
    pub cfg_fields: *const pdc_irq_cfg,
    pub on): *mut *mut void (enable_intr)(int pin_out, bool,
    pub on): *mut *mut void (unmask_gpio)(int pin_out, bool,
    pub pin_out): *mut *mut void (clear_gpio)(int,
}

    static const struct pdc_regs pdc_v3_2 = {
    .irq_cfg_reg	= 0x110,
    .irq_param_reg	= 0x100c,
    };
    static const struct pdc_irq_cfg pdc_cfg_v3_2 = {
    .gpio_irq_sts	= 5,
    .gpio_irq_mask	= 4,
    .irq_enable	= 3,
    .irq_type	= GENMASK(2, 0),
    };
    static const struct pdc_regs pdc_v3_0 = {
    .irq_en_reg	= 0x10,
    .irq_cfg_reg	= 0x110,
    .irq_param_reg	= 0x100c,
    };
    static const struct pdc_irq_cfg pdc_cfg_v3_0 = {
    .gpio_irq_sts	= 4,
    .gpio_irq_mask	= 3,
    .irq_type	= GENMASK(2, 0),
    };
    static const struct pdc_regs pdc_v2_7 = {
    .irq_en_reg	= 0x10,
    .irq_cfg_reg	= 0x110,
    .irq_param_reg	= 0x100c,
    };
    static const struct pdc_irq_cfg pdc_cfg_v2_7 = {
    .irq_type	= GENMASK(2, 0),
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pdc_pin_region {
    pub pin_base: u32,
    pub parent_base: u32,
    pub cnt: u32,
}

    static struct pdc_desc *pdc;
#[no_mangle]
unsafe extern "C" fn pdc_base_reg_write(base: *mut void __iomem, reg: c_int, i: u32, val: u32) {
    static void pdc_base_reg_write(void __iomem *base, int reg, u32 i, u32 val)
    {
    writel_relaxed(val, base + reg + i * sizeof(u32));
    }
#[no_mangle]
unsafe extern "C" fn pdc_reg_write(reg: c_int, i: u32, val: u32) {
    static void pdc_reg_write(int reg, u32 i, u32 val)
    {
    pdc_base_reg_write(pdc.base, reg, i, val);
    }
#[no_mangle]
unsafe extern "C" fn pdc_reg_read(reg: c_int, i: u32) -> u32 {
    static u32 pdc_reg_read(int reg, u32 i)
    {
    return readl_relaxed(pdc.base + reg + i * sizeof(u32));
    }
#[no_mangle]
pub unsafe extern "C" fn pdc_pin_is_gpio(pin_out: c_int) -> bool {
    static inline bool pdc_pin_is_gpio(int pin_out)
    {
//
// PDC allocates direct SPIs at the beginning and
// all GPIOs as SPIs are allocated after direct SPIs.
//
    return pin_out >= pdc.num_spis;
    }
#[no_mangle]
unsafe extern "C" fn pdc_x1e_irq_enable_write(bank: u32, enable: u32) {
    static void pdc_x1e_irq_enable_write(u32 bank, u32 enable)
    {
    void __iomem *base;
// Remap the write access to work around a hardware bug on X1E
    switch (bank) {
    case 0 ... 1:
// Use previous DRV (client) region and shift to bank 3-4
    base = pdc.prev_base;
    bank += 3;
    break;
    case 2 ... 4:
// Use our own region and shift to bank 0-2
    base = pdc.base;
    bank -= 2;
    break;
    case 5:
// No fixup required for bank 5
    base = pdc.base;
    break;
    default:
    WARN_ON(1);
    return;
    }
    pdc_base_reg_write(base, pdc.regs.irq_en_reg, bank, enable);
    }
#[no_mangle]
unsafe extern "C" fn pdc_enable_intr_bank(pin_out: c_int, on: bool) {
    static void pdc_enable_intr_bank(int pin_out, bool on)
    {
    unsigned long enable;
    u32 index, mask;
    index = FIELD_GET(IRQ_ENABLE_BANK_INDEX_MASK, pin_out);
    mask = FIELD_GET(IRQ_ENABLE_BANK_BIT_MASK, pin_out);
    guard(raw_spinlock_irqsave)(&pdc.lock);
    enable = pdc_reg_read(pdc.regs.irq_en_reg, index);
    __assign_bit(mask, &enable, on);
    if (pdc.x1e_quirk)
    pdc_x1e_irq_enable_write(index, enable);
    else
    pdc_reg_write(pdc.regs.irq_en_reg, index, enable);
    }
#[no_mangle]
unsafe extern "C" fn pdc_clear_gpio_cfg(pin_out: c_int) {
    static void pdc_clear_gpio_cfg(int pin_out)
    {
    unsigned long gpio_sts;
    gpio_sts = pdc_reg_read(pdc.regs.irq_cfg_reg, pin_out);
    __clear_bit(pdc.cfg_fields.gpio_irq_sts, &gpio_sts);
    pdc_reg_write(pdc.regs.irq_cfg_reg, pin_out, gpio_sts);
    }
#[no_mangle]
unsafe extern "C" fn pdc_unmask_gpio_cfg(pin_out: c_int, unmask: bool) {
    static void pdc_unmask_gpio_cfg(int pin_out, bool unmask)
    {
    unsigned long gpio_mask;
    gpio_mask = pdc_reg_read(pdc.regs.irq_cfg_reg, pin_out);
    __assign_bit(pdc.cfg_fields.gpio_irq_mask, &gpio_mask, !unmask);
    pdc_reg_write(pdc.regs.irq_cfg_reg, pin_out, gpio_mask);
    }
#[no_mangle]
unsafe extern "C" fn pdc_enable_intr_cfg(pin_out: c_int, on: bool) {
    static void pdc_enable_intr_cfg(int pin_out, bool on)
    {
    let mut enable: c_ulong = pdc_reg_read(pdc.regs.irq_cfg_reg, pin_out);
    __assign_bit(pdc.cfg_fields.irq_enable, &enable, on);
    pdc_reg_write(pdc.regs.irq_cfg_reg, pin_out, enable);
    }
#[no_mangle]
unsafe extern "C" fn qcom_pdc_gic_secondary_disable(d: *mut irq_data) {
    static void qcom_pdc_gic_secondary_disable(struct irq_data *d)
    {
    pdc.enable_intr(d.hwirq, false);
    pdc.unmask_gpio(d.hwirq, false);
    irq_chip_disable_parent(d);
    }
#[no_mangle]
unsafe extern "C" fn qcom_pdc_gic_disable(d: *mut irq_data) {
    static void qcom_pdc_gic_disable(struct irq_data *d)
    {
    pdc.enable_intr(d.hwirq, false);
    irq_chip_disable_parent(d);
    }
#[no_mangle]
unsafe extern "C" fn qcom_pdc_gic_enable(d: *mut irq_data) {
    static void qcom_pdc_gic_enable(struct irq_data *d)
    {
    pdc.enable_intr(d.hwirq, true);
    irq_chip_enable_parent(d);
    }
#[no_mangle]
unsafe extern "C" fn qcom_pdc_gic_secondary_enable(d: *mut irq_data) {
    static void qcom_pdc_gic_secondary_enable(struct irq_data *d)
    {
    pdc.enable_intr(d.hwirq, true);
    pdc.unmask_gpio(d.hwirq, true);
    irq_chip_enable_parent(d);
    }
#[no_mangle]
unsafe extern "C" fn qcom_pdc_secondary_ack(d: *mut irq_data) {
    static void qcom_pdc_secondary_ack(struct irq_data *d)
    {
    if (!irqd_is_level_type(d))
    pdc.clear_gpio(d.hwirq);
    }
#[no_mangle]
unsafe extern "C" fn qcom_pdc_gic_secondary_eoi(d: *mut irq_data) {
    static void qcom_pdc_gic_secondary_eoi(struct irq_data *d)
    {
    if (irqd_is_level_type(d))
    pdc.clear_gpio(d.hwirq);
    irq_chip_eoi_parent(d);
    }
#[no_mangle]
unsafe extern "C" fn qcom_pdc_secondary_mask(d: *mut irq_data) {
    static void qcom_pdc_secondary_mask(struct irq_data *d)
    {
    pdc.enable_intr(d.hwirq, false);
    pdc.unmask_gpio(d.hwirq, false);
    irq_chip_mask_parent(d);
    }
#[no_mangle]
unsafe extern "C" fn qcom_pdc_secondary_unmask(d: *mut irq_data) {
    static void qcom_pdc_secondary_unmask(struct irq_data *d)
    {
    pdc.enable_intr(d.hwirq, true);
    pdc.unmask_gpio(d.hwirq, true);
    irq_chip_unmask_parent(d);
    }
//
// GIC does not handle falling edge or active low. To allow falling edge and
// active low interrupts to be handled at GIC, PDC has an inverter that inverts
// falling edge into a rising edge and active low into an active high.
// For the inverter to work, the polarity bit in the IRQ_CONFIG register has to
// set as per the table below.
// Level sensitive active low    LOW
// Rising edge sensitive         NOT USED
// Falling edge sensitive        LOW
// Dual Edge sensitive           NOT USED
// Level sensitive active High   HIGH
// Falling Edge sensitive        NOT USED
// Rising edge sensitive         HIGH
// Dual Edge sensitive           HIGH
//
    enum pdc_irq_config_bits {
    PDC_LEVEL_LOW		= 0b000,
    PDC_EDGE_FALLING	= 0b010,
    PDC_LEVEL_HIGH		= 0b100,
    PDC_EDGE_RISING		= 0b110,
    PDC_EDGE_DUAL		= 0b111,
    };
//
// qcom_pdc_gic_set_type: Configure PDC for the interrupt
//
// @d:    the interrupt data
// @type: the interrupt type
//
// If @type is edge triggered, forward that as rising edge as PDC
// takes care of converting all edge types to rising edge signal
// If @type is level, then forward that as level high as PDC
// takes care of converting all level types to level high signal
//
#[no_mangle]
unsafe extern "C" fn qcom_pdc_gic_set_type(d: *mut irq_data, type: c_uint) -> c_int {
    static int qcom_pdc_gic_set_type(struct irq_data *d, unsigned int type)
    {
    enum pdc_irq_config_bits old_pdc_type;
    enum pdc_irq_config_bits pdc_type;
    int ret;
    switch (type) {
    case IRQ_TYPE_EDGE_RISING:
    pdc_type = PDC_EDGE_RISING;
    break;
    case IRQ_TYPE_EDGE_FALLING:
    pdc_type = PDC_EDGE_FALLING;
    type = IRQ_TYPE_EDGE_RISING;
    break;
    case IRQ_TYPE_EDGE_BOTH:
    pdc_type = PDC_EDGE_DUAL;
    type = IRQ_TYPE_EDGE_RISING;
    break;
    case IRQ_TYPE_LEVEL_HIGH:
    pdc_type = PDC_LEVEL_HIGH;
    break;
    case IRQ_TYPE_LEVEL_LOW:
    pdc_type = PDC_LEVEL_LOW;
    type = IRQ_TYPE_LEVEL_HIGH;
    break;
    default:
    WARN_ON(1);
    return -EINVAL;
    }
    old_pdc_type = pdc_reg_read(pdc.regs.irq_cfg_reg, d.hwirq);
    pdc_type |= (old_pdc_type & ~pdc.cfg_fields.irq_type);
    pdc_reg_write(pdc.regs.irq_cfg_reg, d.hwirq, pdc_type);
    ret = irq_chip_set_type_parent(d, type);
    if (ret)
    return ret;
//
// When we change types the PDC can give a phantom interrupt.
// Clear it.  Specifically the phantom shows up when reconfiguring
// polarity of interrupt without changing the state of the signal
// but let's be consistent and clear it always.
//
// Doing this works because we have IRQCHIP_SET_TYPE_MASKED so the
// interrupt will be cleared before the rest of the system sees it.
//
    if (old_pdc_type != pdc_type)
    irq_chip_set_parent_state(d, IRQCHIP_STATE_PENDING, false);
    return 0;
    }
//
// qcom_pdc_gic_secondary_set_type: Configure PDC for the interrupt in secondary mode
//
// @d:    the interrupt data
// @type: the interrupt type
//
// All @type are forwarded as level high type to parent GIC
//
#[no_mangle]
unsafe extern "C" fn qcom_pdc_gic_secondary_set_type(d: *mut irq_data, type: c_uint) -> c_int {
    static int qcom_pdc_gic_secondary_set_type(struct irq_data *d, unsigned int type)
    {
    enum pdc_irq_config_bits old_pdc_type;
    enum pdc_irq_config_bits pdc_type;
    int ret;
    switch (type) {
    case IRQ_TYPE_EDGE_RISING:
    pdc_type = PDC_EDGE_RISING;
    break;
    case IRQ_TYPE_EDGE_FALLING:
    pdc_type = PDC_EDGE_FALLING;
    break;
    case IRQ_TYPE_EDGE_BOTH:
    pdc_type = PDC_EDGE_DUAL;
    break;
    case IRQ_TYPE_LEVEL_HIGH:
    pdc_type = PDC_LEVEL_HIGH;
    break;
    case IRQ_TYPE_LEVEL_LOW:
    pdc_type = PDC_LEVEL_LOW;
    break;
    default:
    WARN_ON(1);
    return -EINVAL;
    }
    old_pdc_type = pdc_reg_read(pdc.regs.irq_cfg_reg, d.hwirq);
    pdc_type |= (old_pdc_type & ~pdc.cfg_fields.irq_type);
    pdc_reg_write(pdc.regs.irq_cfg_reg, d.hwirq, pdc_type);
//
// PDC forwards GPIOs as level high to GIC in secondary
// mode. Update the type and clear any previously latched
// phantom interrupt at PDC.
//
    type = IRQ_TYPE_LEVEL_HIGH;
    pdc.clear_gpio(d.hwirq);
    ret = irq_chip_set_type_parent(d, type);
    if (ret)
    return ret;
//
// When we change types the PDC can give a phantom interrupt.
// Clear it.  Specifically the phantom shows up when reconfiguring
// polarity of interrupt without changing the state of the signal
// but let's be consistent and clear it always.
//
// Doing this works because we have IRQCHIP_SET_TYPE_MASKED so the
// interrupt will be cleared before the rest of the system sees it.
//
    if (old_pdc_type != pdc_type)
    irq_chip_set_parent_state(d, IRQCHIP_STATE_PENDING, false);
    return 0;
    }
    static struct irq_chip qcom_pdc_gic_chip = {
    .name			= "PDC",
    .irq_eoi		= irq_chip_eoi_parent,
    .irq_mask		= irq_chip_mask_parent,
    .irq_unmask		= irq_chip_unmask_parent,
    .irq_disable		= qcom_pdc_gic_disable,
    .irq_enable		= qcom_pdc_gic_enable,
    .irq_get_irqchip_state	= irq_chip_get_parent_state,
    .irq_set_irqchip_state	= irq_chip_set_parent_state,
    .irq_retrigger		= irq_chip_retrigger_hierarchy,
    .irq_set_type		= qcom_pdc_gic_set_type,
    .flags			= IRQCHIP_MASK_ON_SUSPEND |
    IRQCHIP_SET_TYPE_MASKED |
    IRQCHIP_SKIP_SET_WAKE |
    IRQCHIP_ENABLE_WAKEUP_ON_SUSPEND,
    .irq_set_vcpu_affinity	= irq_chip_set_vcpu_affinity_parent,
    .irq_set_affinity	= irq_chip_set_affinity_parent,
    };
    static struct irq_chip qcom_pdc_gic_secondary_chip = {
    .name			= "PDC",
    .irq_ack		= qcom_pdc_secondary_ack,
    .irq_eoi		= qcom_pdc_gic_secondary_eoi,
    .irq_mask		= qcom_pdc_secondary_mask,
    .irq_unmask		= qcom_pdc_secondary_unmask,
    .irq_disable		= qcom_pdc_gic_secondary_disable,
    .irq_enable		= qcom_pdc_gic_secondary_enable,
    .irq_get_irqchip_state	= irq_chip_get_parent_state,
    .irq_set_irqchip_state	= irq_chip_set_parent_state,
    .irq_retrigger		= irq_chip_retrigger_hierarchy,
    .irq_set_type		= qcom_pdc_gic_secondary_set_type,
    .flags			= IRQCHIP_MASK_ON_SUSPEND |
    IRQCHIP_SET_TYPE_MASKED |
    IRQCHIP_SKIP_SET_WAKE |
    IRQCHIP_ENABLE_WAKEUP_ON_SUSPEND,
    .irq_set_vcpu_affinity	= irq_chip_set_vcpu_affinity_parent,
    .irq_set_affinity	= irq_chip_set_affinity_parent,
    };
    static struct pdc_pin_region *get_pin_region(int pin)
    {
    for (int i = 0; i < pdc.region_cnt; i++) {
    if (pin >= pdc.region[i].pin_base &&
    pin < pdc.region[i].pin_base + pdc.region[i].cnt)
    return &pdc.region[i];
    }
    return core::ptr::null_mut();
    }
    static int qcom_pdc_alloc(struct irq_domain *domain, unsigned int virq,
    unsigned int nr_irqs, void *data)
    {
    struct irq_fwspec *fwspec = data;
    struct irq_fwspec parent_fwspec;
    struct pdc_pin_region *region;
    irq_hw_number_t hwirq;
    unsigned int type;
    int ret;
    ret = irq_domain_translate_twocell(domain, fwspec, &hwirq, &type);
    if (ret)
    return ret;
    if (hwirq == GPIO_NO_WAKE_IRQ)
    return irq_domain_disconnect_hierarchy(domain, virq);
    ret = irq_domain_set_hwirq_and_chip(domain, virq, hwirq,
    &qcom_pdc_gic_chip, core::ptr::null_mut());
    if (ret)
    return ret;
//
// PDC secondary chip is only set for the GPIO interrupts as SPIs.
// Direct SPI interrupts are still in pass through mode (no latching
// at PDC).
//
    if (pdc.mode == PDC_SECONDARY_MODE && pdc_pin_is_gpio(hwirq)) {
    ret = irq_domain_set_hwirq_and_chip(domain, virq, hwirq,
    &qcom_pdc_gic_secondary_chip,
    core::ptr::null_mut());
    if (ret)
    return ret;
// Secondary mode converts all interrupts to LEVEL HIGH type
    type = IRQ_TYPE_LEVEL_HIGH;
    } else {
    ret = irq_domain_set_hwirq_and_chip(domain, virq, hwirq,
    &qcom_pdc_gic_chip,
    core::ptr::null_mut());
    if (ret)
    return ret;
    if (type & IRQ_TYPE_EDGE_BOTH)
    type = IRQ_TYPE_EDGE_RISING;
    if (type & IRQ_TYPE_LEVEL_MASK)
    type = IRQ_TYPE_LEVEL_HIGH;
    }
    region = get_pin_region(hwirq);
    if (!region)
    return irq_domain_disconnect_hierarchy(domain.parent, virq);
    parent_fwspec.fwnode      = domain.parent.fwnode;
    parent_fwspec.param_count = 3;
    parent_fwspec.param[0]    = 0;
    parent_fwspec.param[1]    = pin_to_hwirq(region, hwirq);
    parent_fwspec.param[2]    = type;
    return irq_domain_alloc_irqs_parent(domain, virq, nr_irqs, &parent_fwspec);
    }
    static const struct irq_domain_ops qcom_pdc_ops = {
    .translate	= irq_domain_translate_twocell,
    .alloc		= qcom_pdc_alloc,
    .free		= irq_domain_free_irqs_common,
    };
#[no_mangle]
unsafe extern "C" fn pdc_setup_pin_mapping(dev: *mut device) -> c_int {
    static int pdc_setup_pin_mapping(struct device *dev)
    {
    struct device_node *np = dev.of_node;
    int ret, n;
    n = of_property_count_elems_of_size(np, "qcom,pdc-ranges", sizeof(u32));
    if (n <= 0 || n % 3)
    return -EINVAL;
    pdc.region_cnt = n / 3;
    pdc.region = devm_kcalloc(dev, pdc.region_cnt, sizeof(*pdc.region), GFP_KERNEL);
    if (!pdc.region) {
    pdc.region_cnt = 0;
    return -ENOMEM;
    }
    for (n = 0; n < pdc.region_cnt; n++) {
    ret = of_property_read_u32_index(np, "qcom,pdc-ranges", n * 3 + 0,
    &pdc.region[n].pin_base);
    if (ret)
    return ret;
    ret = of_property_read_u32_index(np, "qcom,pdc-ranges", n * 3 + 1,
    &pdc.region[n].parent_base);
    if (ret)
    return ret;
    ret = of_property_read_u32_index(np, "qcom,pdc-ranges", n * 3 + 2,
    &pdc.region[n].cnt);
    if (ret)
    return ret;
    for (int i = 0; i < pdc.region[n].cnt; i++) {
    if (pdc_pin_is_gpio(i + pdc.region[n].pin_base) &&
    pdc.mode == PDC_SECONDARY_MODE)
    pdc.clear_gpio(i + pdc.region[n].pin_base);
    pdc.enable_intr(i + pdc.region[n].pin_base, false);
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qcom_pdc_probe(pdev: *mut platform_device, parent: *mut device_node) -> c_int {
    static int qcom_pdc_probe(struct platform_device *pdev, struct device_node *parent)
    {
    struct irq_domain *parent_domain, *pdc_domain;
    struct device_node *node = pdev.dev.of_node;
    struct device *dev = &pdev.dev;
    resource_size_t res_size;
    struct resource res;
    u32 irq_param;
    int ret;
// compat with old sm8150 DT which had very small region for PDC
    if (of_address_to_resource(node, 0, &res))
    return -EINVAL;
    res_size = max_t(resource_size_t, resource_size(&res), PDC_DRV_SIZE);
    if (res_size > resource_size(&res))
    pr_warn("%pOF: invalid reg size, please fix DT\n", node);
    pdc = devm_kzalloc(dev, sizeof(*pdc), GFP_KERNEL);
    if (!pdc)
    return -ENOMEM;
    pdc.base = devm_ioremap(dev, res.start, res_size);
    if (!pdc.base) {
    pr_err("%pOF: unable to map PDC registers\n", node);
    return -ENXIO;
    }
    pdc.version = pdc_reg_read(PDC_VERSION_REG, 0);
    if (pdc.version >= PDC_VERSION_3_2) {
    pdc.cfg_fields = &pdc_cfg_v3_2;
    pdc.regs = &pdc_v3_2;
    pdc.enable_intr = pdc_enable_intr_cfg;
    } else if (pdc.version >= PDC_VERSION_3_0) {
    pdc.cfg_fields = &pdc_cfg_v3_0;
    pdc.regs = &pdc_v3_0;
    pdc.enable_intr = pdc_enable_intr_bank;
    } else {
    pdc.cfg_fields = &pdc_cfg_v2_7;
    pdc.regs = &pdc_v2_7;
    pdc.enable_intr = pdc_enable_intr_bank;
    }
    pdc.mode = PDC_PASS_THROUGH_MODE;
//
// PDC has multiple DRV regions, each one provides the same set of
// registers for a particular client in the system. Due to a hardware
// bug on X1E, some writes to the IRQ_ENABLE_BANK register must be
// issued inside the previous region. This region belongs to
// a different client and is not described in the device tree. Map the
// region with the expected offset to preserve support for old DTs.
//
    if (of_device_is_compatible(node, "qcom,x1e80100-pdc")) {
    pdc.prev_base = devm_ioremap(dev, res.start - PDC_DRV_SIZE,
    pdc.regs.irq_en_reg + IRQ_ENABLE_BANK_MAX);
    if (!pdc.prev_base) {
    pr_err("%pOF: unable to map previous PDC DRV region\n", node);
    return -ENXIO;
    }
    pdc.x1e_quirk = true;
    if (!qcom_scm_is_available())
    return -EPROBE_DEFER;
    ret = qcom_scm_io_writel(PDC_GPIO_INT_CTL_ENABLE, PDC_PASS_THROUGH_MODE);
    if (ret) {
    pdc.mode = PDC_SECONDARY_MODE;
    pdc.unmask_gpio = pdc_unmask_gpio_cfg;
    pdc.clear_gpio = pdc_clear_gpio_cfg;
    }
    }
    irq_param = pdc_reg_read(pdc.regs.irq_param_reg, 0);
    pdc.num_spis = FIELD_GET(GENMASK(7, 0), irq_param);
    parent_domain = irq_find_host(parent);
    if (!parent_domain) {
    pr_err("%pOF: unable to find PDC's parent domain\n", node);
    return -ENXIO;
    }
    raw_spin_lock_init(&pdc.lock);
    ret = pdc_setup_pin_mapping(dev);
    if (ret) {
    pr_err("%pOF: failed to init PDC pin-hwirq mapping\n", node);
    return ret;
    }
    pdc_domain = irq_domain_create_hierarchy(parent_domain, IRQ_DOMAIN_FLAG_QCOM_PDC_WAKEUP,
    PDC_MAX_IRQS, of_fwnode_handle(node),
    &qcom_pdc_ops, core::ptr::null_mut());
    if (!pdc_domain) {
    pr_err("%pOF: PDC domain add failed\n", node);
    return -ENOMEM;
    }
    irq_domain_update_bus_token(pdc_domain, DOMAIN_BUS_WAKEUP);
    return 0;
    }
    IRQCHIP_PLATFORM_DRIVER_BEGIN(qcom_pdc)
    IRQCHIP_MATCH("qcom,pdc", qcom_pdc_probe)
    IRQCHIP_PLATFORM_DRIVER_END(qcom_pdc)
    MODULE_DESCRIPTION("Qualcomm Technologies, Inc. Power Domain Controller");
    MODULE_LICENSE("GPL v2");
