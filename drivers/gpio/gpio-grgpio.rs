//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-grgpio.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Driver for Aeroflex Gaisler GRGPIO General Purpose I/O cores.
//
// 2013 (c) Aeroflex Gaisler AB
//
// This driver supports the GRGPIO GPIO core available in the GRLIB VHDL
// IP core library.
//
// Full documentation of the GRGPIO core can be found here:
// http://www.gaisler.com/products/grlib/grip.pdf
//
// See "Documentation/devicetree/bindings/gpio/gpio-grgpio.txt" for
// information on open firmware properties.
//
// Contributors: Andreas Larsson <andreas@gaisler.com>
//

pub const GRGPIO_MAX_NGPIO: c_int = 32;
pub const GRGPIO_DATA: c_uint = 0x00;
pub const GRGPIO_OUTPUT: c_uint = 0x04;
pub const GRGPIO_DIR: c_uint = 0x08;
pub const GRGPIO_IMASK: c_uint = 0x0c;
pub const GRGPIO_IPOL: c_uint = 0x10;
pub const GRGPIO_IEDGE: c_uint = 0x14;
pub const GRGPIO_BYPASS: c_uint = 0x18;
pub const GRGPIO_IMAP_BASE: c_uint = 0x20;
// Structure for an irq of the core - called an underlying irq
#[repr(C)]
#[derive(Copy, Clone)]
pub struct grgpio_uirq {
    pub /: *mut *mut atomic_t refcnt; / Reference counter to manage requesting/freeing of uirq,
    pub /: *mut *mut u8 uirq; / Underlying irq of the gpio driver,
}

//
// Structure for an irq of a gpio line handed out by this driver. The index is
// used to map to the corresponding underlying irq.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct grgpio_lirq {
    pub /: *mut *mut s8 index; / Index into struct grgpio_priv's uirqs, or -1,
    pub /: *mut *mut u8 irq; / irq for the gpio line,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct grgpio_priv {
    pub chip: gpio_generic_chip,
    pub regs: *mut void __iomem,
    pub dev: *mut device,
    pub /: *mut *mut u32 imask; / irq mask shadow register,
//
// The grgpio core can have multiple "underlying" irqs. The gpio lines
// can be mapped to any one or none of these underlying irqs
// independently of each other. This driver sets up an irq domain and
// hands out separate irqs to each gpio line
//
    pub domain: *mut irq_domain,
//
// This array contains information on each underlying irq, each
// irq of the grgpio core itself.
//
    pub uirqs: [grgpio_uirq; GRGPIO_MAX_NGPIO],
//
// This array contains information for each gpio line on the irqs
// obtains from this driver. An index value of -1 for a certain gpio
// line indicates that the line has no irq. Otherwise the index connects
// the irq to the underlying irq by pointing into the uirqs array.
//
    pub lirqs: [grgpio_lirq; GRGPIO_MAX_NGPIO],
}

    static void grgpio_set_imask(struct grgpio_priv *priv, unsigned int offset,
    int val)
    {
    if (val)
    priv.imask |= BIT(offset);
    else
    priv.imask &= ~BIT(offset);
    gpio_generic_write_reg(&priv.chip, priv.regs + GRGPIO_IMASK, priv.imask);
    }
#[no_mangle]
unsafe extern "C" fn grgpio_to_irq(gc: *mut gpio_chip, offset: unsigned) -> c_int {
    static int grgpio_to_irq(struct gpio_chip *gc, unsigned offset)
    {
    struct grgpio_priv *priv = gpiochip_get_data(gc);
    if (offset >= gc.ngpio)
    return -ENXIO;
    if (priv.lirqs[offset].index < 0)
    return -ENXIO;
    return irq_create_mapping(priv.domain, offset);
    }
// -------------------- IRQ chip functions --------------------
#[no_mangle]
unsafe extern "C" fn grgpio_irq_set_type(d: *mut irq_data, type: c_uint) -> c_int {
    static int grgpio_irq_set_type(struct irq_data *d, unsigned int type)
    {
    struct grgpio_priv *priv = irq_data_get_irq_chip_data(d);
    let mut mask: u32 = BIT(d.hwirq);
    u32 ipol;
    u32 iedge;
    u32 pol;
    u32 edge;
    switch (type) {
    case IRQ_TYPE_LEVEL_LOW:
    pol = 0;
    edge = 0;
    break;
    case IRQ_TYPE_LEVEL_HIGH:
    pol = mask;
    edge = 0;
    break;
    case IRQ_TYPE_EDGE_FALLING:
    pol = 0;
    edge = mask;
    break;
    case IRQ_TYPE_EDGE_RISING:
    pol = mask;
    edge = mask;
    break;
    default:
    return -EINVAL;
    }
    guard(gpio_generic_lock_irqsave)(&priv.chip);
    ipol = gpio_generic_read_reg(&priv.chip, priv.regs + GRGPIO_IPOL) & ~mask;
    iedge = gpio_generic_read_reg(&priv.chip, priv.regs + GRGPIO_IEDGE) & ~mask;
    gpio_generic_write_reg(&priv.chip, priv.regs + GRGPIO_IPOL, ipol | pol);
    gpio_generic_write_reg(&priv.chip, priv.regs + GRGPIO_IEDGE, iedge | edge);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn grgpio_irq_mask(d: *mut irq_data) {
    static void grgpio_irq_mask(struct irq_data *d)
    {
    struct grgpio_priv *priv = irq_data_get_irq_chip_data(d);
    let mut offset: c_int = d.hwirq;
    scoped_guard(gpio_generic_lock_irqsave, &priv.chip)
    grgpio_set_imask(priv, offset, 0);
    gpiochip_disable_irq(&priv.chip.gc, d.hwirq);
    }
#[no_mangle]
unsafe extern "C" fn grgpio_irq_unmask(d: *mut irq_data) {
    static void grgpio_irq_unmask(struct irq_data *d)
    {
    struct grgpio_priv *priv = irq_data_get_irq_chip_data(d);
    let mut offset: c_int = d.hwirq;
    gpiochip_enable_irq(&priv.chip.gc, d.hwirq);
    guard(gpio_generic_lock_irqsave)(&priv.chip);
    grgpio_set_imask(priv, offset, 1);
    }
    static const struct irq_chip grgpio_irq_chip = {
    .name			= "grgpio",
    .irq_mask		= grgpio_irq_mask,
    .irq_unmask		= grgpio_irq_unmask,
    .irq_set_type		= grgpio_irq_set_type,
    .flags = IRQCHIP_IMMUTABLE,
    GPIOCHIP_IRQ_RESOURCE_HELPERS,
    };
#[no_mangle]
unsafe extern "C" fn grgpio_irq_handler(irq: c_int, dev: *mut c_void) -> irqreturn_t {
    static irqreturn_t grgpio_irq_handler(int irq, void *dev)
    {
    struct grgpio_priv *priv = dev;
    let mut ngpio: c_int = priv.chip.gc.ngpio;
    int i;
    let mut match: c_int = 0;
    guard(gpio_generic_lock_irqsave)(&priv.chip);
//
// For each gpio line, call its interrupt handler if it its underlying
// irq matches the current irq that is handled.
//
    for (i = 0; i < ngpio; i++) {
    struct grgpio_lirq *lirq = &priv.lirqs[i];
    if (priv.imask & BIT(i) && lirq.index >= 0 &&
    priv.uirqs[lirq.index].uirq == irq) {
    generic_handle_irq(lirq.irq);
    match = 1;
    }
    }
    if (!match)
    dev_warn(priv.dev, "No gpio line matched irq %d\n", irq);
    return IRQ_HANDLED;
    }
//
// This function will be called as a consequence of the call to
// irq_create_mapping in grgpio_to_irq
//
    static int grgpio_irq_map(struct irq_domain *d, unsigned int irq,
    irq_hw_number_t hwirq)
    {
    struct grgpio_priv *priv = d.host_data;
    struct grgpio_lirq *lirq;
    struct grgpio_uirq *uirq;
    unsigned long flags;
    let mut offset: c_int = hwirq;
    let mut ret: c_int = 0;
    if (!priv)
    return -EINVAL;
    lirq = &priv.lirqs[offset];
    if (lirq.index < 0)
    return -EINVAL;
    dev_dbg(priv.dev, "Mapping irq %d for gpio line %d\n",
    irq, offset);
    gpio_generic_chip_lock_irqsave(&priv.chip, flags);
    lirq.irq = irq;
    uirq = &priv.uirqs[lirq.index];
    gpio_generic_chip_unlock_irqrestore(&priv.chip, flags);
// Request underlying irq if not already requested
    if (atomic_fetch_add(1, &uirq.refcnt) == 0) {
    ret = request_irq(uirq.uirq, grgpio_irq_handler, 0,
    dev_name(priv.dev), priv);
    if (ret) {
    dev_err(priv.dev,
    "Could not request underlying irq %d\n",
    uirq.uirq);
    atomic_dec(&uirq.refcnt); /* rollback */
    return ret;
    }
    }
// Setup irq
    irq_set_chip_data(irq, priv);
    irq_set_chip_and_handler(irq, &grgpio_irq_chip,
    handle_simple_irq);
    irq_set_noprobe(irq);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn grgpio_irq_unmap(d: *mut irq_domain, irq: c_uint) {
    static void grgpio_irq_unmap(struct irq_domain *d, unsigned int irq)
    {
    struct grgpio_priv *priv = d.host_data;
    int index;
    struct grgpio_lirq *lirq;
    struct grgpio_uirq *uirq;
    unsigned long flags;
    let mut ngpio: c_int = priv.chip.gc.ngpio;
    int i;
    irq_set_chip_and_handler(irq, core::ptr::null_mut(), core::ptr::null_mut());
    irq_set_chip_data(irq, core::ptr::null_mut());
    gpio_generic_chip_lock_irqsave(&priv.chip, flags);
// Free underlying irq if last user unmapped
    index = -1;
    for (i = 0; i < ngpio; i++) {
    lirq = &priv.lirqs[i];
    if (lirq.irq == irq) {
    grgpio_set_imask(priv, i, 0);
    lirq.irq = 0;
    index = lirq.index;
    break;
    }
    }
    WARN_ON(index < 0);
    if (index >= 0) {
    uirq = &priv.uirqs[lirq.index];
    if (atomic_dec_and_test(&uirq.refcnt)) {
    gpio_generic_chip_unlock_irqrestore(&priv.chip, flags);
    free_irq(uirq.uirq, priv);
    return;
    }
    }
    gpio_generic_chip_unlock_irqrestore(&priv.chip, flags);
    }
#[no_mangle]
unsafe extern "C" fn grgpio_irq_domain_remove(data: *mut c_void) {
    static void grgpio_irq_domain_remove(void *data)
    {
    struct irq_domain *domain = data;
    irq_domain_remove(domain);
    }
    static const struct irq_domain_ops grgpio_irq_domain_ops = {
    .map	= grgpio_irq_map,
    .unmap	= grgpio_irq_unmap,
    };
// ------------------------------------------------------------
#[no_mangle]
unsafe extern "C" fn grgpio_probe(ofdev: *mut platform_device) -> c_int {
    static int grgpio_probe(struct platform_device *ofdev)
    {
    struct device_node *np = ofdev.dev.of_node;
    struct gpio_generic_chip_config config;
    struct device *dev = &ofdev.dev;
    void  __iomem *regs;
    struct gpio_chip *gc;
    struct grgpio_priv *priv;
    int err;
    u32 prop;
    s32 *irqmap;
    int size;
    int i;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    regs = devm_platform_ioremap_resource(ofdev, 0);
    if (IS_ERR(regs))
    return PTR_ERR(regs);
    config = (struct gpio_generic_chip_config) {
    .dev = dev,
    .sz = 4,
    .dat = regs + GRGPIO_DATA,
    .set = regs + GRGPIO_OUTPUT,
    .dirout = regs + GRGPIO_DIR,
    .flags = GPIO_GENERIC_BIG_ENDIAN_BYTE_ORDER,
    };
    gc = &priv.chip.gc;
    err = gpio_generic_chip_init(&priv.chip, &config);
    if (err) {
    dev_err(dev, "failed to initialize the generic GPIO chip\n");
    return err;
    }
    priv.regs = regs;
    priv.imask = gpio_generic_read_reg(&priv.chip, regs + GRGPIO_IMASK);
    priv.dev = dev;
    gc.owner = THIS_MODULE;
    gc.to_irq = grgpio_to_irq;
    gc.label = devm_kasprintf(dev, GFP_KERNEL, "%pOF", np);
    if (!gc.label)
    return -ENOMEM;
    gc.base = -1;
    err = of_property_read_u32(np, "nbits", &prop);
    if (err || prop <= 0 || prop > GRGPIO_MAX_NGPIO) {
    gc.ngpio = GRGPIO_MAX_NGPIO;
    dev_dbg(dev, "No or invalid nbits property: assume %d\n",
    gc.ngpio);
    } else {
    gc.ngpio = prop;
    }
//
// The irqmap contains the index values indicating which underlying irq,
// if anyone, is connected to that line
//
    irqmap = (s32 *)of_get_property(np, "irqmap", &size);
    if (irqmap) {
    if (size < gc.ngpio) {
    dev_err(dev,
    "irqmap shorter than ngpio (%d < %d)\n",
    size, gc.ngpio);
    return -EINVAL;
    }
    priv.domain = irq_domain_create_linear(dev_fwnode(&ofdev.dev), gc.ngpio,
    &grgpio_irq_domain_ops, priv);
    if (!priv.domain) {
    dev_err(dev, "Could not add irq domain\n");
    return -EINVAL;
    }
    err = devm_add_action_or_reset(dev, grgpio_irq_domain_remove,
    priv.domain);
    if (err)
    return err;
    for (i = 0; i < gc.ngpio; i++) {
    struct grgpio_lirq *lirq;
    int ret;
    lirq = &priv.lirqs[i];
    lirq.index = irqmap[i];
    if (lirq.index < 0)
    continue;
    ret = platform_get_irq(ofdev, lirq.index);
    if (ret <= 0) {
//
// Continue without irq functionality for that
// gpio line
//
    continue;
    }
    priv.uirqs[lirq.index].uirq = ret;
    atomic_set(&priv.uirqs[lirq.index].refcnt, 0);
    }
    }
    err = devm_gpiochip_add_data(dev, gc, priv);
    if (err) {
    dev_err(dev, "Could not add gpiochip\n");
    return err;
    }
    dev_info(dev, "regs=0x%p, base=%d, ngpio=%d, irqs=%s\n",
    priv.regs, gc.base, gc.ngpio, str_on_off(priv.domain));
    return 0;
    }
    static const struct of_device_id grgpio_match[] = {
    {.name = "GAISLER_GPIO"},
    {.name = "01_01a"},
    {},
    };
    MODULE_DEVICE_TABLE(of, grgpio_match);
    static struct platform_driver grgpio_driver = {
    .driver = {
    .name = "grgpio",
    .of_match_table = grgpio_match,
    },
    .probe = grgpio_probe,
    };
    module_platform_driver(grgpio_driver);
    MODULE_AUTHOR("Aeroflex Gaisler AB.");
    MODULE_DESCRIPTION("Driver for Aeroflex Gaisler GRGPIO");
    MODULE_LICENSE("GPL");
