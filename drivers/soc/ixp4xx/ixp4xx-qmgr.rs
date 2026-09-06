//! Automatically rewritten from C to Rust
//! Source: drivers/soc/ixp4xx/ixp4xx-qmgr.c
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
// Intel IXP4xx Queue Manager driver for Linux
//
// Copyright (C) 2007 Krzysztof Halasa <khc@pm.waw.pl>
//

    static struct qmgr_regs __iomem *qmgr_regs;
    static int qmgr_irq_1;
    static int qmgr_irq_2;
    static spinlock_t qmgr_lock;
    static u32 used_sram_bitmap[4]; /* 128 16-dword pages */
    static void (*irq_handlers[QUEUES])(void *pdev);
    static void *irq_pdevs[QUEUES];

    char qmgr_queue_descs[QUEUES][32];

#[no_mangle]
pub unsafe extern "C" fn qmgr_put_entry(queue: c_uint, val: u32) {
    void qmgr_put_entry(unsigned int queue, u32 val)
    {

    BUG_ON(!qmgr_queue_descs[queue]); /* not yet requested */
    printk(KERN_DEBUG "Queue %s(%i) put %X\n",
    qmgr_queue_descs[queue], queue, val);

    __raw_writel(val, &qmgr_regs.acc[queue][0]);
    }
#[no_mangle]
pub unsafe extern "C" fn qmgr_get_entry(queue: c_uint) -> u32 {
    u32 qmgr_get_entry(unsigned int queue)
    {
    u32 val;
    val = __raw_readl(&qmgr_regs.acc[queue][0]);

    BUG_ON(!qmgr_queue_descs[queue]); /* not yet requested */
    printk(KERN_DEBUG "Queue %s(%i) get %X\n",
    qmgr_queue_descs[queue], queue, val);

    return val;
    }
#[no_mangle]
unsafe extern "C" fn __qmgr_get_stat1(queue: c_uint) -> c_int {
    static int __qmgr_get_stat1(unsigned int queue)
    {
    return (__raw_readl(&qmgr_regs.stat1[queue >> 3])
    >> ((queue & 7) << 2)) & 0xF;
    }
#[no_mangle]
unsafe extern "C" fn __qmgr_get_stat2(queue: c_uint) -> c_int {
    static int __qmgr_get_stat2(unsigned int queue)
    {
    BUG_ON(queue >= HALF_QUEUES);
    return (__raw_readl(&qmgr_regs.stat2[queue >> 4])
    >> ((queue & 0xF) << 1)) & 0x3;
    }
//
// qmgr_stat_empty() - checks if a hardware queue is empty
// @queue:	queue number
//
// Returns non-zero value if the queue is empty.
//
#[no_mangle]
pub unsafe extern "C" fn qmgr_stat_empty(queue: c_uint) -> c_int {
    int qmgr_stat_empty(unsigned int queue)
    {
    BUG_ON(queue >= HALF_QUEUES);
    return __qmgr_get_stat1(queue) & QUEUE_STAT1_EMPTY;
    }
//
// qmgr_stat_below_low_watermark() - checks if a queue is below low watermark
// @queue:	queue number
//
// Returns non-zero value if the queue is below low watermark.
//
#[no_mangle]
pub unsafe extern "C" fn qmgr_stat_below_low_watermark(queue: c_uint) -> c_int {
    int qmgr_stat_below_low_watermark(unsigned int queue)
    {
    if (queue >= HALF_QUEUES)
    return (__raw_readl(&qmgr_regs.statne_h) >>
    (queue - HALF_QUEUES)) & 0x01;
    return __qmgr_get_stat1(queue) & QUEUE_STAT1_NEARLY_EMPTY;
    }
//
// qmgr_stat_full() - checks if a hardware queue is full
// @queue:	queue number
//
// Returns non-zero value if the queue is full.
//
#[no_mangle]
pub unsafe extern "C" fn qmgr_stat_full(queue: c_uint) -> c_int {
    int qmgr_stat_full(unsigned int queue)
    {
    if (queue >= HALF_QUEUES)
    return (__raw_readl(&qmgr_regs.statf_h) >>
    (queue - HALF_QUEUES)) & 0x01;
    return __qmgr_get_stat1(queue) & QUEUE_STAT1_FULL;
    }
//
// qmgr_stat_overflow() - checks if a hardware queue experienced overflow
// @queue:	queue number
//
// Returns non-zero value if the queue experienced overflow.
//
#[no_mangle]
pub unsafe extern "C" fn qmgr_stat_overflow(queue: c_uint) -> c_int {
    int qmgr_stat_overflow(unsigned int queue)
    {
    return __qmgr_get_stat2(queue) & QUEUE_STAT2_OVERFLOW;
    }
    void qmgr_set_irq(unsigned int queue, int src,
    void (*handler)(void *pdev), void *pdev)
    {
    unsigned long flags;
    spin_lock_irqsave(&qmgr_lock, flags);
    if (queue < HALF_QUEUES) {
    u32 __iomem *reg;
    int bit;
    BUG_ON(src > QUEUE_IRQ_SRC_NOT_FULL);
    reg = &qmgr_regs.irqsrc[queue >> 3]; /* 8 queues per u32 */
    bit = (queue % 8) * 4; /* 3 bits + 1 reserved bit per queue */
    __raw_writel((__raw_readl(reg) & ~(7 << bit)) | (src << bit),
    reg);
    } else
// IRQ source for queues 32-63 is fixed
    BUG_ON(src != QUEUE_IRQ_SRC_NOT_NEARLY_EMPTY);
    irq_handlers[queue] = handler;
    irq_pdevs[queue] = pdev;
    spin_unlock_irqrestore(&qmgr_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn qmgr_irq1_a0(irq: c_int, pdev: *mut c_void) -> irqreturn_t {
    static irqreturn_t qmgr_irq1_a0(int irq, void *pdev)
    {
    int i, ret = 0;
    u32 en_bitmap, src, stat;
// ACK - it may clear any bits so don't rely on it
    __raw_writel(0xFFFFFFFF, &qmgr_regs.irqstat[0]);
    en_bitmap = __raw_readl(&qmgr_regs.irqen[0]);
    while (en_bitmap) {
    i = __fls(en_bitmap); /* number of the last "low" queue */
    en_bitmap &= ~BIT(i);
    src = __raw_readl(&qmgr_regs.irqsrc[i >> 3]);
    stat = __raw_readl(&qmgr_regs.stat1[i >> 3]);
    if (src & 4) /* the IRQ condition is inverted */
    stat = ~stat;
    if (stat & BIT(src & 3)) {
    irq_handlers[i](irq_pdevs[i]);
    ret = IRQ_HANDLED;
    }
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn qmgr_irq2_a0(irq: c_int, pdev: *mut c_void) -> irqreturn_t {
    static irqreturn_t qmgr_irq2_a0(int irq, void *pdev)
    {
    int i, ret = 0;
    u32 req_bitmap;
// ACK - it may clear any bits so don't rely on it
    __raw_writel(0xFFFFFFFF, &qmgr_regs.irqstat[1]);
    req_bitmap = __raw_readl(&qmgr_regs.irqen[1]) &
    __raw_readl(&qmgr_regs.statne_h);
    while (req_bitmap) {
    i = __fls(req_bitmap); /* number of the last "high" queue */
    req_bitmap &= ~BIT(i);
    irq_handlers[HALF_QUEUES + i](irq_pdevs[HALF_QUEUES + i]);
    ret = IRQ_HANDLED;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn qmgr_irq(irq: c_int, pdev: *mut c_void) -> irqreturn_t {
    static irqreturn_t qmgr_irq(int irq, void *pdev)
    {
    int i, half = (irq == qmgr_irq_1 ? 0 : 1);
    let mut req_bitmap: u32 = __raw_readl(&qmgr_regs.irqstat[half]);
    if (!req_bitmap)
    return 0;
    __raw_writel(req_bitmap, &qmgr_regs.irqstat[half]); /* ACK */
    while (req_bitmap) {
    i = __fls(req_bitmap); /* number of the last queue */
    req_bitmap &= ~BIT(i);
    i += half * HALF_QUEUES;
    irq_handlers[i](irq_pdevs[i]);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
pub unsafe extern "C" fn qmgr_enable_irq(queue: c_uint) {
    void qmgr_enable_irq(unsigned int queue)
    {
    unsigned long flags;
    let mut half: c_int = queue / 32;
    let mut mask: u32 = 1 << (queue & (HALF_QUEUES - 1));
    spin_lock_irqsave(&qmgr_lock, flags);
    __raw_writel(__raw_readl(&qmgr_regs.irqen[half]) | mask,
    &qmgr_regs.irqen[half]);
    spin_unlock_irqrestore(&qmgr_lock, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn qmgr_disable_irq(queue: c_uint) {
    void qmgr_disable_irq(unsigned int queue)
    {
    unsigned long flags;
    let mut half: c_int = queue / 32;
    let mut mask: u32 = 1 << (queue & (HALF_QUEUES - 1));
    spin_lock_irqsave(&qmgr_lock, flags);
    __raw_writel(__raw_readl(&qmgr_regs.irqen[half]) & ~mask,
    &qmgr_regs.irqen[half]);
    __raw_writel(mask, &qmgr_regs.irqstat[half]); /* clear */
    spin_unlock_irqrestore(&qmgr_lock, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn shift_mask(mask: *mut u32) {
    static inline void shift_mask(u32 *mask)
    {
    mask[3] = mask[3] << 1 | mask[2] >> 31;
    mask[2] = mask[2] << 1 | mask[1] >> 31;
    mask[1] = mask[1] << 1 | mask[0] >> 31;
    mask[0] <<= 1;
    }

    int qmgr_request_queue(unsigned int queue, unsigned int len /* dwords */,
    unsigned int nearly_empty_watermark,
    unsigned int nearly_full_watermark,
    const char *desc_format, const char* name)

    int __qmgr_request_queue(unsigned int queue, unsigned int len /* dwords */,
    unsigned int nearly_empty_watermark,
    unsigned int nearly_full_watermark)

    {
    u32 cfg, addr = 0, mask[4]; /* in 16-dwords */
    int err;
    BUG_ON(queue >= QUEUES);
    if ((nearly_empty_watermark | nearly_full_watermark) & ~7)
    return -EINVAL;
    switch (len) {
    case  16:
    cfg = 0 << 24;
    mask[0] = 0x1;
    break;
    case  32:
    cfg = 1 << 24;
    mask[0] = 0x3;
    break;
    case  64:
    cfg = 2 << 24;
    mask[0] = 0xF;
    break;
    case 128:
    cfg = 3 << 24;
    mask[0] = 0xFF;
    break;
    default:
    return -EINVAL;
    }
    cfg |= nearly_empty_watermark << 26;
    cfg |= nearly_full_watermark << 29;
    len /= 16;		/* in 16-dwords: 1, 2, 4 or 8 */
    mask[1] = mask[2] = mask[3] = 0;
    if (!try_module_get(THIS_MODULE))
    return -ENODEV;
    spin_lock_irq(&qmgr_lock);
    if (__raw_readl(&qmgr_regs.sram[queue])) {
    err = -EBUSY;
    goto err;
    }
    while (1) {
    if (!(used_sram_bitmap[0] & mask[0]) &&
    !(used_sram_bitmap[1] & mask[1]) &&
    !(used_sram_bitmap[2] & mask[2]) &&
    !(used_sram_bitmap[3] & mask[3]))
    break; /* found free space */
    addr++;
    shift_mask(mask);
    if (addr + len > ARRAY_SIZE(qmgr_regs.sram)) {
    printk(KERN_ERR "qmgr: no free SRAM space for"
    " queue %i\n", queue);
    err = -ENOMEM;
    goto err;
    }
    }
    used_sram_bitmap[0] |= mask[0];
    used_sram_bitmap[1] |= mask[1];
    used_sram_bitmap[2] |= mask[2];
    used_sram_bitmap[3] |= mask[3];
    __raw_writel(cfg | (addr << 14), &qmgr_regs.sram[queue]);

    snprintf(qmgr_queue_descs[queue], sizeof(qmgr_queue_descs[0]),
    desc_format, name);
    printk(KERN_DEBUG "qmgr: requested queue %s(%i) addr = 0x%02X\n",
    qmgr_queue_descs[queue], queue, addr);

    spin_unlock_irq(&qmgr_lock);
    return 0;
    err:
    spin_unlock_irq(&qmgr_lock);
    module_put(THIS_MODULE);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn qmgr_release_queue(queue: c_uint) {
    void qmgr_release_queue(unsigned int queue)
    {
    u32 cfg, addr, mask[4];
    BUG_ON(queue >= QUEUES); /* not in valid range */
    spin_lock_irq(&qmgr_lock);
    cfg = __raw_readl(&qmgr_regs.sram[queue]);
    addr = (cfg >> 14) & 0xFF;
    BUG_ON(!addr);		/* not requested */
    switch ((cfg >> 24) & 3) {
    case 0: mask[0] = 0x1; break;
    case 1: mask[0] = 0x3; break;
    case 2: mask[0] = 0xF; break;
    case 3: mask[0] = 0xFF; break;
    }
    mask[1] = mask[2] = mask[3] = 0;
    while (addr--)
    shift_mask(mask);

    printk(KERN_DEBUG "qmgr: releasing queue %s(%i)\n",
    qmgr_queue_descs[queue], queue);
    qmgr_queue_descs[queue][0] = '\x0';

    while ((addr = qmgr_get_entry(queue)))
    printk(KERN_ERR "qmgr: released queue %i not empty: 0x%08X\n",
    queue, addr);
    __raw_writel(0, &qmgr_regs.sram[queue]);
    used_sram_bitmap[0] &= ~mask[0];
    used_sram_bitmap[1] &= ~mask[1];
    used_sram_bitmap[2] &= ~mask[2];
    used_sram_bitmap[3] &= ~mask[3];
    irq_handlers[queue] = core::ptr::null_mut(); /* catch IRQ bugs */
    spin_unlock_irq(&qmgr_lock);
    module_put(THIS_MODULE);
    }
#[no_mangle]
unsafe extern "C" fn ixp4xx_qmgr_probe(pdev: *mut platform_device) -> c_int {
    static int ixp4xx_qmgr_probe(struct platform_device *pdev)
    {
    int i, err;
    irq_handler_t handler1, handler2;
    struct device *dev = &pdev.dev;
    struct resource *res;
    int irq1, irq2;
    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!res)
    return -ENODEV;
    qmgr_regs = devm_ioremap_resource(dev, res);
    if (IS_ERR(qmgr_regs))
    return PTR_ERR(qmgr_regs);
    irq1 = platform_get_irq(pdev, 0);
    if (irq1 <= 0)
    return irq1 ? irq1 : -EINVAL;
    qmgr_irq_1 = irq1;
    irq2 = platform_get_irq(pdev, 1);
    if (irq2 <= 0)
    return irq2 ? irq2 : -EINVAL;
    qmgr_irq_2 = irq2;
// reset qmgr registers
    for (i = 0; i < 4; i++) {
    __raw_writel(0x33333333, &qmgr_regs.stat1[i]);
    __raw_writel(0, &qmgr_regs.irqsrc[i]);
    }
    for (i = 0; i < 2; i++) {
    __raw_writel(0, &qmgr_regs.stat2[i]);
    __raw_writel(0xFFFFFFFF, &qmgr_regs.irqstat[i]); /* clear */
    __raw_writel(0, &qmgr_regs.irqen[i]);
    }
    __raw_writel(0xFFFFFFFF, &qmgr_regs.statne_h);
    __raw_writel(0, &qmgr_regs.statf_h);
    for (i = 0; i < QUEUES; i++)
    __raw_writel(0, &qmgr_regs.sram[i]);
    if (cpu_is_ixp42x_rev_a0()) {
    handler1 = qmgr_irq1_a0;
    handler2 = qmgr_irq2_a0;
    } else
    handler1 = handler2 = qmgr_irq;
    err = devm_request_irq(dev, irq1, handler1, 0, "IXP4xx Queue Manager",
    core::ptr::null_mut());
    if (err)
    return err;
    err = devm_request_irq(dev, irq2, handler2, 0, "IXP4xx Queue Manager",
    core::ptr::null_mut());
    if (err)
    return err;
    used_sram_bitmap[0] = 0xF; /* 4 first pages reserved for config */
    spin_lock_init(&qmgr_lock);
    dev_info(dev, "IXP4xx Queue Manager initialized.\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ixp4xx_qmgr_remove(pdev: *mut platform_device) {
    static void ixp4xx_qmgr_remove(struct platform_device *pdev)
    {
    synchronize_irq(qmgr_irq_1);
    synchronize_irq(qmgr_irq_2);
    }
    static const struct of_device_id ixp4xx_qmgr_of_match[] = {
    {
    .compatible = "intel,ixp4xx-ahb-queue-manager",
    },
    {},
    };
    MODULE_DEVICE_TABLE(of, ixp4xx_qmgr_of_match);
    static struct platform_driver ixp4xx_qmgr_driver = {
    .driver = {
    .name           = "ixp4xx-qmgr",
    .of_match_table = ixp4xx_qmgr_of_match,
    },
    .probe = ixp4xx_qmgr_probe,
    .remove = ixp4xx_qmgr_remove,
    };
    module_platform_driver(ixp4xx_qmgr_driver);
    MODULE_DESCRIPTION("Intel IXP4xx Queue Manager driver");
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Krzysztof Halasa");
    EXPORT_SYMBOL(qmgr_put_entry);
    EXPORT_SYMBOL(qmgr_get_entry);
    EXPORT_SYMBOL(qmgr_stat_empty);
    EXPORT_SYMBOL(qmgr_stat_below_low_watermark);
    EXPORT_SYMBOL(qmgr_stat_full);
    EXPORT_SYMBOL(qmgr_stat_overflow);
    EXPORT_SYMBOL(qmgr_set_irq);
    EXPORT_SYMBOL(qmgr_enable_irq);
    EXPORT_SYMBOL(qmgr_disable_irq);

    EXPORT_SYMBOL(qmgr_queue_descs);
    EXPORT_SYMBOL(qmgr_request_queue);

    EXPORT_SYMBOL(__qmgr_request_queue);

    EXPORT_SYMBOL(qmgr_release_queue);
