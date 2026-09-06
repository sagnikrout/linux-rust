//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/qcom-pm8xxx.c
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
// Copyright (c) 2011, Code Aurora Forum. All rights reserved.
//

pub const SSBI_REG_ADDR_IRQ_BASE: c_uint = 0x1BB;

pub const PM8821_SSBI_REG_ADDR_IRQ_BASE: c_uint = 0x100;

    ((m == 0) ? \
    (PM8821_SSBI_REG_ADDR_IRQ_MASTER0 + b + offset) : \
    (PM8821_SSBI_REG_ADDR_IRQ_MASTER1 + b + offset))

pub const PM8821_BLOCKS_PER_MASTER: c_int = 7;
pub const PM_IRQF_LVL_SEL: c_uint = 0x01	/* level select */;
pub const PM_IRQF_MASK_FE: c_uint = 0x02	/* mask falling edge */;
pub const PM_IRQF_MASK_RE: c_uint = 0x04	/* mask rising edge */;
pub const PM_IRQF_CLR: c_uint = 0x08	/* clear interrupt */;
pub const PM_IRQF_BITS_MASK: c_uint = 0x70;
pub const PM_IRQF_BITS_SHIFT: c_int = 4;
pub const PM_IRQF_WRITE: c_uint = 0x80;

    PM_IRQF_MASK_RE)
pub const REG_HWREV: c_uint = 0x002  /* PMIC4 revision */;
pub const REG_HWREV_2: c_uint = 0x0E8  /* PMIC4 revision 2 */;
pub const PM8XXX_NR_IRQS: c_int = 256;
pub const PM8821_NR_IRQS: c_int = 112;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm_irq_data {
    pub num_irqs: c_int,
    pub irq_chip: *mut irq_chip,
    pub irq_handler: irq_handler_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm_irq_chip {
    pub regmap: *mut regmap,
    pub pm_irq_lock: spinlock_t,
    pub irqdomain: *mut irq_domain,
    pub num_blocks: c_uint,
    pub num_masters: c_uint,
    pub pm_irq_data: *const pm_irq_data,
// MUST BE AT THE END OF THIS STRUCT
    pub config: [u8; ],
}

    static int pm8xxx_read_block_irq(struct pm_irq_chip *chip, unsigned int bp,
    unsigned int *ip)
    {
    int	rc;
    spin_lock(&chip.pm_irq_lock);
    rc = regmap_write(chip.regmap, SSBI_REG_ADDR_IRQ_BLK_SEL, bp);
    if (rc) {
    pr_err("Failed Selecting Block %d rc=%d\n", bp, rc);
    goto bail;
    }
    rc = regmap_read(chip.regmap, SSBI_REG_ADDR_IRQ_IT_STATUS, ip);
    if (rc)
    pr_err("Failed Reading Status rc=%d\n", rc);
    bail:
    spin_unlock(&chip.pm_irq_lock);
    return rc;
    }
    static int
    pm8xxx_config_irq(struct pm_irq_chip *chip, unsigned int bp, unsigned int cp)
    {
    int	rc;
    unsigned long flags;
    spin_lock_irqsave(&chip.pm_irq_lock, flags);
    rc = regmap_write(chip.regmap, SSBI_REG_ADDR_IRQ_BLK_SEL, bp);
    if (rc) {
    pr_err("Failed Selecting Block %d rc=%d\n", bp, rc);
    goto bail;
    }
    cp |= PM_IRQF_WRITE;
    rc = regmap_write(chip.regmap, SSBI_REG_ADDR_IRQ_CONFIG, cp);
    if (rc)
    pr_err("Failed Configuring IRQ rc=%d\n", rc);
    bail:
    spin_unlock_irqrestore(&chip.pm_irq_lock, flags);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn pm8xxx_irq_block_handler(chip: *mut pm_irq_chip, block: c_int) -> c_int {
    static int pm8xxx_irq_block_handler(struct pm_irq_chip *chip, int block)
    {
    int pmirq, i, ret = 0;
    unsigned int bits;
    ret = pm8xxx_read_block_irq(chip, block, &bits);
    if (ret) {
    pr_err("Failed reading %d block ret=%d", block, ret);
    return ret;
    }
    if (!bits) {
    pr_err("block bit set in master but no irqs: %d", block);
    return 0;
    }
// Check IRQ bits
    for (i = 0; i < 8; i++) {
    if (bits & (1 << i)) {
    pmirq = block * 8 + i;
    generic_handle_domain_irq(chip.irqdomain, pmirq);
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pm8xxx_irq_master_handler(chip: *mut pm_irq_chip, master: c_int) -> c_int {
    static int pm8xxx_irq_master_handler(struct pm_irq_chip *chip, int master)
    {
    unsigned int blockbits;
    int block_number, i, ret = 0;
    ret = regmap_read(chip.regmap, SSBI_REG_ADDR_IRQ_M_STATUS1 + master,
    &blockbits);
    if (ret) {
    pr_err("Failed to read master %d ret=%d\n", master, ret);
    return ret;
    }
    if (!blockbits) {
    pr_err("master bit set in root but no blocks: %d", master);
    return 0;
    }
    for (i = 0; i < 8; i++)
    if (blockbits & (1 << i)) {
    block_number = master * 8 + i;	/* block # */
    ret |= pm8xxx_irq_block_handler(chip, block_number);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pm8xxx_irq_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t pm8xxx_irq_handler(int irq, void *data)
    {
    struct pm_irq_chip *chip = data;
    unsigned int root;
    int	i, ret, masters = 0;
    ret = regmap_read(chip.regmap, SSBI_REG_ADDR_IRQ_ROOT, &root);
    if (ret) {
    pr_err("Can't read root status ret=%d\n", ret);
    return IRQ_NONE;
    }
// on pm8xxx series masters start from bit 1 of the root
    masters = root >> 1;
// Read allowed masters for blocks.
    for (i = 0; i < chip.num_masters; i++)
    if (masters & (1 << i))
    pm8xxx_irq_master_handler(chip, i);
    return IRQ_HANDLED;
    }
    static void pm8821_irq_block_handler(struct pm_irq_chip *chip,
    int master, int block)
    {
    int pmirq, i, ret;
    unsigned int bits;
    ret = regmap_read(chip.regmap,
    PM8821_SSBI_ADDR_IRQ_ROOT(master, block), &bits);
    if (ret) {
    pr_err("Reading block %d failed ret=%d", block, ret);
    return;
    }
// Convert block offset to global block number
    block += (master * PM8821_BLOCKS_PER_MASTER) - 1;
// Check IRQ bits
    for (i = 0; i < 8; i++) {
    if (bits & BIT(i)) {
    pmirq = block * 8 + i;
    generic_handle_domain_irq(chip.irqdomain, pmirq);
    }
    }
    }
    static inline void pm8821_irq_master_handler(struct pm_irq_chip *chip,
    int master, u8 master_val)
    {
    int block;
    for (block = 1; block < 8; block++)
    if (master_val & BIT(block))
    pm8821_irq_block_handler(chip, master, block);
    }
#[no_mangle]
unsafe extern "C" fn pm8821_irq_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t pm8821_irq_handler(int irq, void *data)
    {
    struct pm_irq_chip *chip = data;
    unsigned int master;
    int ret;
    ret = regmap_read(chip.regmap,
    PM8821_SSBI_REG_ADDR_IRQ_MASTER0, &master);
    if (ret) {
    pr_err("Failed to read master 0 ret=%d\n", ret);
    return IRQ_NONE;
    }
// bits 1 through 7 marks the first 7 blocks in master 0
    if (master & GENMASK(7, 1))
    pm8821_irq_master_handler(chip, 0, master);
// bit 0 marks if master 1 contains any bits
    if (!(master & BIT(0)))
    return IRQ_NONE;
    ret = regmap_read(chip.regmap,
    PM8821_SSBI_REG_ADDR_IRQ_MASTER1, &master);
    if (ret) {
    pr_err("Failed to read master 1 ret=%d\n", ret);
    return IRQ_NONE;
    }
    pm8821_irq_master_handler(chip, 1, master);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn pm8xxx_irq_mask_ack(d: *mut irq_data) {
    static void pm8xxx_irq_mask_ack(struct irq_data *d)
    {
    struct pm_irq_chip *chip = irq_data_get_irq_chip_data(d);
    let mut pmirq: c_uint = irqd_to_hwirq(d);
    u8	block, config;
    block = pmirq / 8;
    config = chip.config[pmirq] | PM_IRQF_MASK_ALL | PM_IRQF_CLR;
    pm8xxx_config_irq(chip, block, config);
    }
#[no_mangle]
unsafe extern "C" fn pm8xxx_irq_unmask(d: *mut irq_data) {
    static void pm8xxx_irq_unmask(struct irq_data *d)
    {
    struct pm_irq_chip *chip = irq_data_get_irq_chip_data(d);
    let mut pmirq: c_uint = irqd_to_hwirq(d);
    u8	block, config;
    block = pmirq / 8;
    config = chip.config[pmirq];
    pm8xxx_config_irq(chip, block, config);
    }
#[no_mangle]
unsafe extern "C" fn pm8xxx_irq_set_type(d: *mut irq_data, flow_type: c_uint) -> c_int {
    static int pm8xxx_irq_set_type(struct irq_data *d, unsigned int flow_type)
    {
    struct pm_irq_chip *chip = irq_data_get_irq_chip_data(d);
    let mut pmirq: c_uint = irqd_to_hwirq(d);
    int irq_bit;
    u8 block, config;
    block = pmirq / 8;
    irq_bit  = pmirq % 8;
    chip.config[pmirq] = (irq_bit << PM_IRQF_BITS_SHIFT)
    | PM_IRQF_MASK_ALL;
    if (flow_type & (IRQF_TRIGGER_RISING | IRQF_TRIGGER_FALLING)) {
    if (flow_type & IRQF_TRIGGER_RISING)
    chip.config[pmirq] &= ~PM_IRQF_MASK_RE;
    if (flow_type & IRQF_TRIGGER_FALLING)
    chip.config[pmirq] &= ~PM_IRQF_MASK_FE;
    } else {
    chip.config[pmirq] |= PM_IRQF_LVL_SEL;
    if (flow_type & IRQF_TRIGGER_HIGH)
    chip.config[pmirq] &= ~PM_IRQF_MASK_RE;
    else
    chip.config[pmirq] &= ~PM_IRQF_MASK_FE;
    }
    config = chip.config[pmirq] | PM_IRQF_CLR;
    return pm8xxx_config_irq(chip, block, config);
    }
    static int pm8xxx_irq_get_irqchip_state(struct irq_data *d,
    enum irqchip_irq_state which,
    bool *state)
    {
    struct pm_irq_chip *chip = irq_data_get_irq_chip_data(d);
    let mut pmirq: c_uint = irqd_to_hwirq(d);
    unsigned int bits;
    unsigned long flags;
    int irq_bit;
    u8 block;
    int rc;
    if (which != IRQCHIP_STATE_LINE_LEVEL)
    return -EINVAL;
    block = pmirq / 8;
    irq_bit = pmirq % 8;
    spin_lock_irqsave(&chip.pm_irq_lock, flags);
    rc = regmap_write(chip.regmap, SSBI_REG_ADDR_IRQ_BLK_SEL, block);
    if (rc) {
    pr_err("Failed Selecting Block %d rc=%d\n", block, rc);
    goto bail;
    }
    rc = regmap_read(chip.regmap, SSBI_REG_ADDR_IRQ_RT_STATUS, &bits);
    if (rc) {
    pr_err("Failed Reading Status rc=%d\n", rc);
    goto bail;
    }
// state = !!(bits & BIT(irq_bit));
    bail:
    spin_unlock_irqrestore(&chip.pm_irq_lock, flags);
    return rc;
    }
    static struct irq_chip pm8xxx_irq_chip = {
    .name		= "pm8xxx",
    .irq_mask_ack	= pm8xxx_irq_mask_ack,
    .irq_unmask	= pm8xxx_irq_unmask,
    .irq_set_type	= pm8xxx_irq_set_type,
    .irq_get_irqchip_state = pm8xxx_irq_get_irqchip_state,
    .flags		= IRQCHIP_MASK_ON_SUSPEND | IRQCHIP_SKIP_SET_WAKE,
    };
    static void pm8xxx_irq_domain_map(struct pm_irq_chip *chip,
    struct irq_domain *domain, unsigned int irq,
    irq_hw_number_t hwirq, unsigned int type)
    {
    irq_domain_set_info(domain, irq, hwirq, chip.pm_irq_data.irq_chip,
    chip, handle_level_irq, core::ptr::null_mut(), core::ptr::null_mut());
    irq_set_noprobe(irq);
    }
    static int pm8xxx_irq_domain_alloc(struct irq_domain *domain, unsigned int virq,
    unsigned int nr_irqs, void *data)
    {
    struct pm_irq_chip *chip = domain.host_data;
    struct irq_fwspec *fwspec = data;
    irq_hw_number_t hwirq;
    unsigned int type;
    int ret, i;
    ret = irq_domain_translate_twocell(domain, fwspec, &hwirq, &type);
    if (ret)
    return ret;
    for (i = 0; i < nr_irqs; i++)
    pm8xxx_irq_domain_map(chip, domain, virq + i, hwirq + i, type);
    return 0;
    }
    static const struct irq_domain_ops pm8xxx_irq_domain_ops = {
    .alloc = pm8xxx_irq_domain_alloc,
    .free = irq_domain_free_irqs_common,
    .translate = irq_domain_translate_twocell,
    };
#[no_mangle]
unsafe extern "C" fn pm8821_irq_mask_ack(d: *mut irq_data) {
    static void pm8821_irq_mask_ack(struct irq_data *d)
    {
    struct pm_irq_chip *chip = irq_data_get_irq_chip_data(d);
    let mut pmirq: c_uint = irqd_to_hwirq(d);
    u8 block, master;
    int irq_bit, rc;
    block = pmirq / 8;
    master = block / PM8821_BLOCKS_PER_MASTER;
    irq_bit = pmirq % 8;
    block %= PM8821_BLOCKS_PER_MASTER;
    rc = regmap_update_bits(chip.regmap,
    PM8821_SSBI_ADDR_IRQ_MASK(master, block),
    BIT(irq_bit), BIT(irq_bit));
    if (rc) {
    pr_err("Failed to mask IRQ:%d rc=%d\n", pmirq, rc);
    return;
    }
    rc = regmap_update_bits(chip.regmap,
    PM8821_SSBI_ADDR_IRQ_CLEAR(master, block),
    BIT(irq_bit), BIT(irq_bit));
    if (rc)
    pr_err("Failed to CLEAR IRQ:%d rc=%d\n", pmirq, rc);
    }
#[no_mangle]
unsafe extern "C" fn pm8821_irq_unmask(d: *mut irq_data) {
    static void pm8821_irq_unmask(struct irq_data *d)
    {
    struct pm_irq_chip *chip = irq_data_get_irq_chip_data(d);
    let mut pmirq: c_uint = irqd_to_hwirq(d);
    int irq_bit, rc;
    u8 block, master;
    block = pmirq / 8;
    master = block / PM8821_BLOCKS_PER_MASTER;
    irq_bit = pmirq % 8;
    block %= PM8821_BLOCKS_PER_MASTER;
    rc = regmap_update_bits(chip.regmap,
    PM8821_SSBI_ADDR_IRQ_MASK(master, block),
    BIT(irq_bit), ~BIT(irq_bit));
    if (rc)
    pr_err("Failed to read/write unmask IRQ:%d rc=%d\n", pmirq, rc);
    }
    static int pm8821_irq_get_irqchip_state(struct irq_data *d,
    enum irqchip_irq_state which,
    bool *state)
    {
    struct pm_irq_chip *chip = irq_data_get_irq_chip_data(d);
    int rc, pmirq = irqd_to_hwirq(d);
    u8 block, irq_bit, master;
    unsigned int bits;
    block = pmirq / 8;
    master = block / PM8821_BLOCKS_PER_MASTER;
    irq_bit = pmirq % 8;
    block %= PM8821_BLOCKS_PER_MASTER;
    rc = regmap_read(chip.regmap,
    PM8821_SSBI_ADDR_IRQ_RT_STATUS(master, block), &bits);
    if (rc) {
    pr_err("Reading Status of IRQ %d failed rc=%d\n", pmirq, rc);
    return rc;
    }
// state = !!(bits & BIT(irq_bit));
    return rc;
    }
    static struct irq_chip pm8821_irq_chip = {
    .name		= "pm8821",
    .irq_mask_ack	= pm8821_irq_mask_ack,
    .irq_unmask	= pm8821_irq_unmask,
    .irq_get_irqchip_state = pm8821_irq_get_irqchip_state,
    .flags		= IRQCHIP_MASK_ON_SUSPEND | IRQCHIP_SKIP_SET_WAKE,
    };
    static const struct regmap_config ssbi_regmap_config = {
    .reg_bits = 16,
    .val_bits = 8,
    .max_register = 0x3ff,
    .fast_io = true,
    .reg_read = ssbi_reg_read,
    .reg_write = ssbi_reg_write
    };
    static const struct pm_irq_data pm8xxx_data = {
    .num_irqs = PM8XXX_NR_IRQS,
    .irq_chip = &pm8xxx_irq_chip,
    .irq_handler = pm8xxx_irq_handler,
    };
    static const struct pm_irq_data pm8821_data = {
    .num_irqs = PM8821_NR_IRQS,
    .irq_chip = &pm8821_irq_chip,
    .irq_handler = pm8821_irq_handler,
    };
    static const struct of_device_id pm8xxx_id_table[] = {
    { .compatible = "qcom,pm8058", .data = &pm8xxx_data},
    { .compatible = "qcom,pm8821", .data = &pm8821_data},
    { .compatible = "qcom,pm8921", .data = &pm8xxx_data},
    { }
    };
    MODULE_DEVICE_TABLE(of, pm8xxx_id_table);
#[no_mangle]
unsafe extern "C" fn pm8xxx_probe(pdev: *mut platform_device) -> c_int {
    static int pm8xxx_probe(struct platform_device *pdev)
    {
    const struct pm_irq_data *data;
    struct regmap *regmap;
    int irq, rc;
    unsigned int val;
    struct pm_irq_chip *chip;
    data = of_device_get_match_data(&pdev.dev);
    if (!data) {
    dev_err(&pdev.dev, "No matching driver data found\n");
    return -EINVAL;
    }
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    regmap = devm_regmap_init(&pdev.dev, core::ptr::null_mut(), pdev.dev.parent,
    &ssbi_regmap_config);
    if (IS_ERR(regmap))
    return PTR_ERR(regmap);
// Read PMIC chip revision
    rc = regmap_read(regmap, REG_HWREV, &val);
    if (rc) {
    pr_err("Failed to read hw rev reg %d:rc=%d\n", REG_HWREV, rc);
    return rc;
    }
    pr_info("PMIC revision 1: %02X\n", val);
// Read PMIC chip revision 2
    rc = regmap_read(regmap, REG_HWREV_2, &val);
    if (rc) {
    pr_err("Failed to read hw rev 2 reg %d:rc=%d\n",
    REG_HWREV_2, rc);
    return rc;
    }
    pr_info("PMIC revision 2: %02X\n", val);
    chip = devm_kzalloc(&pdev.dev,
    struct_size(chip, config, data.num_irqs),
    GFP_KERNEL);
    if (!chip)
    return -ENOMEM;
    platform_set_drvdata(pdev, chip);
    chip.regmap = regmap;
    chip.num_blocks = DIV_ROUND_UP(data.num_irqs, 8);
    chip.num_masters = DIV_ROUND_UP(chip.num_blocks, 8);
    chip.pm_irq_data = data;
    spin_lock_init(&chip.pm_irq_lock);
    chip.irqdomain = irq_domain_create_linear(dev_fwnode(&pdev.dev), data.num_irqs,
    &pm8xxx_irq_domain_ops, chip);
    if (!chip.irqdomain)
    return -ENODEV;
    rc = devm_request_irq(&pdev.dev, irq, data.irq_handler, 0, dev_name(&pdev.dev), chip);
    if (rc)
    return rc;
    irq_set_irq_wake(irq, 1);
    rc = of_platform_populate(pdev.dev.of_node, core::ptr::null_mut(), core::ptr::null_mut(), &pdev.dev);
    if (rc)
    irq_domain_remove(chip.irqdomain);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn pm8xxx_remove(pdev: *mut platform_device) {
    static void pm8xxx_remove(struct platform_device *pdev)
    {
    struct pm_irq_chip *chip = platform_get_drvdata(pdev);
    of_platform_depopulate(&pdev.dev);
    irq_domain_remove(chip.irqdomain);
    }
    static struct platform_driver pm8xxx_driver = {
    .probe		= pm8xxx_probe,
    .remove		= pm8xxx_remove,
    .driver		= {
    .name	= "pm8xxx-core",
    .of_match_table = pm8xxx_id_table,
    },
    };
#[no_mangle]
unsafe extern "C" fn pm8xxx_init() -> int __init {
    static int __init pm8xxx_init(void)
    {
    return platform_driver_register(&pm8xxx_driver);
    }
    subsys_initcall(pm8xxx_init);
#[no_mangle]
unsafe extern "C" fn pm8xxx_exit() -> void __exit {
    static void __exit pm8xxx_exit(void)
    {
    platform_driver_unregister(&pm8xxx_driver);
    }
    module_exit(pm8xxx_exit);
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("PMIC 8xxx core driver");
    MODULE_VERSION("1.0");
    MODULE_ALIAS("platform:pm8xxx-core");
