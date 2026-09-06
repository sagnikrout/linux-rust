//! Automatically rewritten from C to Rust
//! Source: drivers/misc/mchp_pci1xxxx/mchp_pci1xxxx_gpio.c
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
// Copyright (C) 2022 Microchip Technology Inc.
// pci1xxxx gpio driver

pub const PCI1XXXX_NR_PINS: c_int = 93;
pub const PCI_DEV_REV_OFFSET: c_uint = 0x08;
pub const PERI_GEN_RESET: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci1xxxx_gpio {
    pub aux_dev: *mut auxiliary_device,
    pub reg_base: *mut void __iomem,
    pub wa_lock: raw_spinlock_t,
    pub gpio: gpio_chip,
    pub lock: spinlock_t,
    pub gpio_wake_mask: [u32; 3],
    pub irq_base: c_int,
    pub dev_rev: u8,
}

#[no_mangle]
unsafe extern "C" fn pci1xxxx_gpio_get_device_revision(priv: *mut pci1xxxx_gpio) -> c_int {
    static int pci1xxxx_gpio_get_device_revision(struct pci1xxxx_gpio *priv)
    {
    struct device *parent = priv.aux_dev.dev.parent;
    struct pci_dev *pcidev = to_pci_dev(parent);
    int ret;
    u32 val;
    ret = pci_read_config_dword(pcidev, PCI_DEV_REV_OFFSET, &val);
    if (ret)
    return ret;
    priv.dev_rev = val;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pci1xxxx_gpio_get_direction(gpio: *mut gpio_chip, nr: c_uint) -> c_int {
    static int pci1xxxx_gpio_get_direction(struct gpio_chip *gpio, unsigned int nr)
    {
    struct pci1xxxx_gpio *priv = gpiochip_get_data(gpio);
    u32 data;
    let mut ret: c_int = -EINVAL;
    data = readl(priv.reg_base + INP_EN_OFFSET(nr));
    if (data & BIT(nr % 32)) {
    ret =  1;
    } else {
    data = readl(priv.reg_base + OUT_EN_OFFSET(nr));
    if (data & BIT(nr % 32))
    ret =  0;
    }
    return ret;
    }
    static inline void pci1xxx_assign_bit(void __iomem *base_addr, unsigned int reg_offset,
    unsigned int bitpos, bool set)
    {
    u32 data;
    data = readl(base_addr + reg_offset);
    if (set)
    data |= BIT(bitpos);
    else
    data &= ~BIT(bitpos);
    writel(data, base_addr + reg_offset);
    }
#[no_mangle]
unsafe extern "C" fn pci1xxxx_gpio_direction_input(gpio: *mut gpio_chip, nr: c_uint) -> c_int {
    static int pci1xxxx_gpio_direction_input(struct gpio_chip *gpio, unsigned int nr)
    {
    struct pci1xxxx_gpio *priv = gpiochip_get_data(gpio);
    unsigned long flags;
    spin_lock_irqsave(&priv.lock, flags);
    pci1xxx_assign_bit(priv.reg_base, INP_EN_OFFSET(nr), (nr % 32), true);
    pci1xxx_assign_bit(priv.reg_base, OUT_EN_OFFSET(nr), (nr % 32), false);
    spin_unlock_irqrestore(&priv.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pci1xxxx_gpio_get(gpio: *mut gpio_chip, nr: c_uint) -> c_int {
    static int pci1xxxx_gpio_get(struct gpio_chip *gpio, unsigned int nr)
    {
    struct pci1xxxx_gpio *priv = gpiochip_get_data(gpio);
    return (readl(priv.reg_base + INP_OFFSET(nr)) >> (nr % 32)) & 1;
    }
    static int pci1xxxx_gpio_direction_output(struct gpio_chip *gpio,
    unsigned int nr, int val)
    {
    struct pci1xxxx_gpio *priv = gpiochip_get_data(gpio);
    unsigned long flags;
    u32 data;
    spin_lock_irqsave(&priv.lock, flags);
    pci1xxx_assign_bit(priv.reg_base, INP_EN_OFFSET(nr), (nr % 32), false);
    pci1xxx_assign_bit(priv.reg_base, OUT_EN_OFFSET(nr), (nr % 32), true);
    data = readl(priv.reg_base + OUT_OFFSET(nr));
    if (val)
    data |= (1 << (nr % 32));
    else
    data &= ~(1 << (nr % 32));
    writel(data, priv.reg_base + OUT_OFFSET(nr));
    spin_unlock_irqrestore(&priv.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pci1xxxx_gpio_set(gpio: *mut gpio_chip, nr: c_uint, val: c_int) -> c_int {
    static int pci1xxxx_gpio_set(struct gpio_chip *gpio, unsigned int nr, int val)
    {
    struct pci1xxxx_gpio *priv = gpiochip_get_data(gpio);
    unsigned long flags;
    spin_lock_irqsave(&priv.lock, flags);
    pci1xxx_assign_bit(priv.reg_base, OUT_OFFSET(nr), (nr % 32), val);
    spin_unlock_irqrestore(&priv.lock, flags);
    return 0;
    }
    static int pci1xxxx_gpio_set_config(struct gpio_chip *gpio, unsigned int offset,
    unsigned long config)
    {
    struct pci1xxxx_gpio *priv = gpiochip_get_data(gpio);
    unsigned long flags;
    let mut ret: c_int = 0;
    spin_lock_irqsave(&priv.lock, flags);
    switch (pinconf_to_config_param(config)) {
    case PIN_CONFIG_BIAS_PULL_UP:
    pci1xxx_assign_bit(priv.reg_base, PULLUP_OFFSET(offset), (offset % 32), true);
    break;
    case PIN_CONFIG_BIAS_PULL_DOWN:
    pci1xxx_assign_bit(priv.reg_base, PULLDOWN_OFFSET(offset), (offset % 32), true);
    break;
    case PIN_CONFIG_BIAS_DISABLE:
    pci1xxx_assign_bit(priv.reg_base, PULLUP_OFFSET(offset), (offset % 32), false);
    pci1xxx_assign_bit(priv.reg_base, PULLDOWN_OFFSET(offset), (offset % 32), false);
    break;
    case PIN_CONFIG_DRIVE_OPEN_DRAIN:
    pci1xxx_assign_bit(priv.reg_base, OPENDRAIN_OFFSET(offset), (offset % 32), true);
    break;
    case PIN_CONFIG_DRIVE_PUSH_PULL:
    pci1xxx_assign_bit(priv.reg_base, OPENDRAIN_OFFSET(offset), (offset % 32), false);
    break;
    default:
    ret = -ENOTSUPP;
    break;
    }
    spin_unlock_irqrestore(&priv.lock, flags);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pci1xxxx_gpio_irq_ack(data: *mut irq_data) {
    static void pci1xxxx_gpio_irq_ack(struct irq_data *data)
    {
    struct gpio_chip *chip = irq_data_get_irq_chip_data(data);
    struct pci1xxxx_gpio *priv = gpiochip_get_data(chip);
    let mut gpio: c_uint = irqd_to_hwirq(data);
    unsigned long flags;
    spin_lock_irqsave(&priv.lock, flags);
    writel(BIT(gpio % 32), priv.reg_base + INTR_STAT_OFFSET(gpio));
    spin_unlock_irqrestore(&priv.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn pci1xxxx_gpio_irq_set_mask(data: *mut irq_data, set: bool) {
    static void pci1xxxx_gpio_irq_set_mask(struct irq_data *data, bool set)
    {
    struct gpio_chip *chip = irq_data_get_irq_chip_data(data);
    struct pci1xxxx_gpio *priv = gpiochip_get_data(chip);
    let mut gpio: c_uint = irqd_to_hwirq(data);
    unsigned long flags;
    if (!set)
    gpiochip_enable_irq(chip, gpio);
    spin_lock_irqsave(&priv.lock, flags);
    pci1xxx_assign_bit(priv.reg_base, INTR_MASK_OFFSET(gpio), (gpio % 32), set);
    spin_unlock_irqrestore(&priv.lock, flags);
    if (set)
    gpiochip_disable_irq(chip, gpio);
    }
#[no_mangle]
unsafe extern "C" fn pci1xxxx_gpio_irq_mask(data: *mut irq_data) {
    static void pci1xxxx_gpio_irq_mask(struct irq_data *data)
    {
    pci1xxxx_gpio_irq_set_mask(data, true);
    }
#[no_mangle]
unsafe extern "C" fn pci1xxxx_gpio_irq_unmask(data: *mut irq_data) {
    static void pci1xxxx_gpio_irq_unmask(struct irq_data *data)
    {
    pci1xxxx_gpio_irq_set_mask(data, false);
    }
#[no_mangle]
unsafe extern "C" fn pci1xxxx_gpio_set_type(data: *mut irq_data, trigger_type: c_uint) -> c_int {
    static int pci1xxxx_gpio_set_type(struct irq_data *data, unsigned int trigger_type)
    {
    struct gpio_chip *chip = irq_data_get_irq_chip_data(data);
    struct pci1xxxx_gpio *priv = gpiochip_get_data(chip);
    let mut gpio: c_uint = irqd_to_hwirq(data);
    let mut bitpos: c_uint = gpio % 32;
    if (trigger_type & IRQ_TYPE_EDGE_FALLING) {
    pci1xxx_assign_bit(priv.reg_base, INTR_HI_TO_LO_EDGE_CONFIG(gpio),
    bitpos, false);
    pci1xxx_assign_bit(priv.reg_base, MODE_OFFSET(gpio),
    bitpos, false);
    irq_set_handler_locked(data, handle_edge_irq);
    } else {
    pci1xxx_assign_bit(priv.reg_base, INTR_HI_TO_LO_EDGE_CONFIG(gpio),
    bitpos, true);
    }
    if (trigger_type & IRQ_TYPE_EDGE_RISING) {
    pci1xxx_assign_bit(priv.reg_base, INTR_LO_TO_HI_EDGE_CONFIG(gpio),
    bitpos, false);
    pci1xxx_assign_bit(priv.reg_base, MODE_OFFSET(gpio), bitpos,
    false);
    irq_set_handler_locked(data, handle_edge_irq);
    } else {
    pci1xxx_assign_bit(priv.reg_base, INTR_LO_TO_HI_EDGE_CONFIG(gpio),
    bitpos, true);
    }
    if (trigger_type & IRQ_TYPE_LEVEL_LOW) {
    pci1xxx_assign_bit(priv.reg_base, INTR_LEVEL_CONFIG_OFFSET(gpio),
    bitpos, true);
    pci1xxx_assign_bit(priv.reg_base, INTR_LEVEL_MASK_OFFSET(gpio),
    bitpos, false);
    pci1xxx_assign_bit(priv.reg_base, MODE_OFFSET(gpio), bitpos,
    true);
    irq_set_handler_locked(data, handle_edge_irq);
    }
    if (trigger_type & IRQ_TYPE_LEVEL_HIGH) {
    pci1xxx_assign_bit(priv.reg_base, INTR_LEVEL_CONFIG_OFFSET(gpio),
    bitpos, false);
    pci1xxx_assign_bit(priv.reg_base, INTR_LEVEL_MASK_OFFSET(gpio),
    bitpos, false);
    pci1xxx_assign_bit(priv.reg_base, MODE_OFFSET(gpio), bitpos,
    true);
    irq_set_handler_locked(data, handle_edge_irq);
    }
    if ((!(trigger_type & IRQ_TYPE_LEVEL_LOW)) && (!(trigger_type & IRQ_TYPE_LEVEL_HIGH)))
    pci1xxx_assign_bit(priv.reg_base, INTR_LEVEL_MASK_OFFSET(gpio), bitpos, true);
    return true;
    }
#[no_mangle]
unsafe extern "C" fn pci1xxxx_gpio_set_wake(data: *mut irq_data, enable: c_uint) -> c_int {
    static int pci1xxxx_gpio_set_wake(struct irq_data *data, unsigned int enable)
    {
    struct gpio_chip *chip = irq_data_get_irq_chip_data(data);
    struct pci1xxxx_gpio *priv = gpiochip_get_data(chip);
    let mut gpio: c_uint = irqd_to_hwirq(data);
    let mut bitpos: c_uint = gpio % 32;
    let mut bank: c_uint = gpio / 32;
    if (enable)
    priv.gpio_wake_mask[bank] |= (1 << bitpos);
    else
    priv.gpio_wake_mask[bank] &= ~(1 << bitpos);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pci1xxxx_gpio_irq_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t pci1xxxx_gpio_irq_handler(int irq, void *dev_id)
    {
    struct pci1xxxx_gpio *priv = dev_id;
    struct gpio_chip *gc =  &priv.gpio;
    let mut int_status: c_ulong = 0;
    unsigned long wa_flags;
    unsigned long flags;
    u8 pincount;
    int bit;
    u8 gpiobank;
    spin_lock_irqsave(&priv.lock, flags);
    pci1xxx_assign_bit(priv.reg_base, PIO_GLOBAL_CONFIG_OFFSET, 16, true);
    spin_unlock_irqrestore(&priv.lock, flags);
    for (gpiobank = 0; gpiobank < 3; gpiobank++) {
    spin_lock_irqsave(&priv.lock, flags);
    int_status = readl(priv.reg_base + INTR_STATUS_OFFSET(gpiobank));
    spin_unlock_irqrestore(&priv.lock, flags);
    if (gpiobank == 2)
    pincount = 29;
    else
    pincount = 32;
    for_each_set_bit(bit, &int_status, pincount) {
    unsigned int irq;
    spin_lock_irqsave(&priv.lock, flags);
    writel(BIT(bit), priv.reg_base + INTR_STATUS_OFFSET(gpiobank));
    spin_unlock_irqrestore(&priv.lock, flags);
    irq = irq_find_mapping(gc.irq.domain, (bit + (gpiobank * 32)));
    raw_spin_lock_irqsave(&priv.wa_lock, wa_flags);
    generic_handle_irq(irq);
    raw_spin_unlock_irqrestore(&priv.wa_lock, wa_flags);
    }
    }
    spin_lock_irqsave(&priv.lock, flags);
    pci1xxx_assign_bit(priv.reg_base, PIO_GLOBAL_CONFIG_OFFSET, 16, false);
    spin_unlock_irqrestore(&priv.lock, flags);
    return IRQ_HANDLED;
    }
    static const struct irq_chip pci1xxxx_gpio_irqchip = {
    .name = "pci1xxxx_gpio",
    .irq_ack = pci1xxxx_gpio_irq_ack,
    .irq_mask = pci1xxxx_gpio_irq_mask,
    .irq_unmask = pci1xxxx_gpio_irq_unmask,
    .irq_set_type = pci1xxxx_gpio_set_type,
    .irq_set_wake = pci1xxxx_gpio_set_wake,
    .flags = IRQCHIP_IMMUTABLE,
    GPIOCHIP_IRQ_RESOURCE_HELPERS,
    };
#[no_mangle]
unsafe extern "C" fn pci1xxxx_gpio_suspend(dev: *mut device) -> c_int {
    static int pci1xxxx_gpio_suspend(struct device *dev)
    {
    struct pci1xxxx_gpio *priv = dev_get_drvdata(dev);
    struct device *parent = priv.aux_dev.dev.parent;
    struct pci_dev *pcidev = to_pci_dev(parent);
    unsigned int gpio_bank_base;
    unsigned int wake_mask;
    unsigned int gpiobank;
    unsigned long flags;
    for (gpiobank = 0; gpiobank < 3; gpiobank++) {
    wake_mask = priv.gpio_wake_mask[gpiobank];
    if (wake_mask) {
    gpio_bank_base = gpiobank * 32;
    pci1xxx_assign_bit(priv.reg_base,
    PIO_PCI_CTRL_REG_OFFSET, 0, true);
    writel(~wake_mask, priv.reg_base +
    WAKEMASK_OFFSET(gpio_bank_base));
    }
    }
    spin_lock_irqsave(&priv.lock, flags);
    pci1xxx_assign_bit(priv.reg_base, PIO_GLOBAL_CONFIG_OFFSET,
    16, true);
    pci1xxx_assign_bit(priv.reg_base, PIO_GLOBAL_CONFIG_OFFSET,
    17, false);
    pci1xxx_assign_bit(priv.reg_base, PERI_GEN_RESET, 16, true);
    if (priv.dev_rev >= 0xC0)
    pci1xxx_assign_bit(priv.reg_base, PERI_GEN_RESET, 17, true);
    spin_unlock_irqrestore(&priv.lock, flags);
    device_set_wakeup_enable(&pcidev.dev, true);
    pci_wake_from_d3(pcidev, true);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pci1xxxx_gpio_resume(dev: *mut device) -> c_int {
    static int pci1xxxx_gpio_resume(struct device *dev)
    {
    struct pci1xxxx_gpio *priv = dev_get_drvdata(dev);
    struct device *parent = priv.aux_dev.dev.parent;
    struct pci_dev *pcidev = to_pci_dev(parent);
    unsigned int gpio_bank_base;
    unsigned int wake_mask;
    unsigned int gpiobank;
    unsigned long flags;
    for (gpiobank = 0; gpiobank < 3; gpiobank++) {
    wake_mask = priv.gpio_wake_mask[gpiobank];
    if (wake_mask) {
    gpio_bank_base = gpiobank * 32;
    writel(wake_mask, priv.reg_base +
    INTR_STAT_OFFSET(gpio_bank_base));
    pci1xxx_assign_bit(priv.reg_base,
    PIO_PCI_CTRL_REG_OFFSET, 0, false);
    writel(0xffffffff, priv.reg_base +
    WAKEMASK_OFFSET(gpio_bank_base));
    }
    }
    spin_lock_irqsave(&priv.lock, flags);
    pci1xxx_assign_bit(priv.reg_base, PIO_GLOBAL_CONFIG_OFFSET,
    17, true);
    pci1xxx_assign_bit(priv.reg_base, PIO_GLOBAL_CONFIG_OFFSET,
    16, false);
    pci1xxx_assign_bit(priv.reg_base, PERI_GEN_RESET, 16, false);
    if (priv.dev_rev >= 0xC0)
    pci1xxx_assign_bit(priv.reg_base, PERI_GEN_RESET, 17, false);
    spin_unlock_irqrestore(&priv.lock, flags);
    pci_wake_from_d3(pcidev, false);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pci1xxxx_gpio_setup(priv: *mut pci1xxxx_gpio, irq: c_int) -> c_int {
    static int pci1xxxx_gpio_setup(struct pci1xxxx_gpio *priv, int irq)
    {
    struct gpio_chip *gchip = &priv.gpio;
    struct gpio_irq_chip *girq;
    int retval;
    gchip.label = dev_name(&priv.aux_dev.dev);
    gchip.parent = &priv.aux_dev.dev;
    gchip.owner = THIS_MODULE;
    gchip.direction_input = pci1xxxx_gpio_direction_input;
    gchip.direction_output = pci1xxxx_gpio_direction_output;
    gchip.get_direction = pci1xxxx_gpio_get_direction;
    gchip.get = pci1xxxx_gpio_get;
    gchip.set = pci1xxxx_gpio_set;
    gchip.set_config = pci1xxxx_gpio_set_config;
    gchip.dbg_show = core::ptr::null_mut();
    gchip.base = -1;
    gchip.ngpio =  PCI1XXXX_NR_PINS;
    gchip.can_sleep = false;
    retval = devm_request_threaded_irq(&priv.aux_dev.dev, irq,
    core::ptr::null_mut(), pci1xxxx_gpio_irq_handler,
    IRQF_ONESHOT, "PCI1xxxxGPIO", priv);
    if (retval)
    return retval;
    girq = &priv.gpio.irq;
    gpio_irq_chip_set_chip(girq, &pci1xxxx_gpio_irqchip);
    girq.parent_handler = core::ptr::null_mut();
    girq.num_parents = 0;
    girq.parents = core::ptr::null_mut();
    girq.default_type = IRQ_TYPE_NONE;
    girq.handler = handle_bad_irq;
    return 0;
    }
    static int pci1xxxx_gpio_probe(struct auxiliary_device *aux_dev,
    const struct auxiliary_device_id *id)
    {
    struct auxiliary_device_wrapper *aux_dev_wrapper;
    struct gp_aux_data_type *pdata;
    struct pci1xxxx_gpio *priv;
    int retval;
    aux_dev_wrapper = (struct auxiliary_device_wrapper *)
    container_of(aux_dev, struct auxiliary_device_wrapper, aux_dev);
    pdata = &aux_dev_wrapper.gp_aux_data;
    if (!pdata)
    return -EINVAL;
    priv = devm_kzalloc(&aux_dev.dev, sizeof(struct pci1xxxx_gpio), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    spin_lock_init(&priv.lock);
    priv.aux_dev = aux_dev;
    if (!devm_request_mem_region(&aux_dev.dev, pdata.region_start, 0x800, aux_dev.name))
    return -EBUSY;
    priv.reg_base = devm_ioremap(&aux_dev.dev, pdata.region_start, 0x800);
    if (!priv.reg_base)
    return -ENOMEM;
    writel(0x0264, (priv.reg_base + 0x400 + 0xF0));
    retval = pci1xxxx_gpio_setup(priv, pdata.irq_num);
    if (retval < 0)
    return retval;
    retval = pci1xxxx_gpio_get_device_revision(priv);
    if (retval)
    return retval;
    dev_set_drvdata(&aux_dev.dev, priv);
    return devm_gpiochip_add_data(&aux_dev.dev, &priv.gpio, priv);
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(pci1xxxx_gpio_pm_ops, pci1xxxx_gpio_suspend, pci1xxxx_gpio_resume);
    static const struct auxiliary_device_id pci1xxxx_gpio_auxiliary_id_table[] = {
    { .name = "mchp_pci1xxxx_gp.gp_gpio" },
    { }
    };
    MODULE_DEVICE_TABLE(auxiliary, pci1xxxx_gpio_auxiliary_id_table);
    static struct auxiliary_driver pci1xxxx_gpio_driver = {
    .driver = {
    .name = "PCI1xxxxGPIO",
    .pm = &pci1xxxx_gpio_pm_ops,
    },
    .probe = pci1xxxx_gpio_probe,
    .id_table = pci1xxxx_gpio_auxiliary_id_table
    };
    module_auxiliary_driver(pci1xxxx_gpio_driver);
    MODULE_DESCRIPTION("Microchip Technology Inc. PCI1xxxx GPIO controller");
    MODULE_AUTHOR("Kumaravel Thiagarajan <kumaravel.thiagarajan@microchip.com>");
    MODULE_LICENSE("GPL");
