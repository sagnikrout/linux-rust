//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/motorola-cpcap.c
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
// Motorola CPCAP PMIC core driver
//
// Copyright (C) 2016 Tony Lindgren <tony@atomide.com>
//

pub const CPCAP_NR_IRQ_REG_BANKS: c_int = 6;
pub const CPCAP_NR_IRQ_CHIPS: c_int = 3;
pub const CPCAP_REGISTER_SIZE: c_int = 4;
pub const CPCAP_REGISTER_BITS: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpcap_ddata {
    pub spi: *mut spi_device,
    pub irqs: *mut regmap_irq,
    pub irqdata: [*mut regmap_irq_chip_data; CPCAP_NR_IRQ_CHIPS],
    pub regmap_conf: *const regmap_config,
    pub regmap: *mut regmap,
}

#[no_mangle]
unsafe extern "C" fn cpcap_sense_irq(regmap: *mut regmap, irq: c_int) -> c_int {
    static int cpcap_sense_irq(struct regmap *regmap, int irq)
    {
    let mut regnum: c_int = irq / CPCAP_REGISTER_BITS;
    let mut mask: c_int = BIT(irq % CPCAP_REGISTER_BITS);
    let mut reg: c_int = CPCAP_REG_INTS1 + (regnum * CPCAP_REGISTER_SIZE);
    int err, val;
    if (reg < CPCAP_REG_INTS1 || reg > CPCAP_REG_INTS4)
    return -EINVAL;
    err = regmap_read(regmap, reg, &val);
    if (err)
    return err;
    return !!(val & mask);
    }
#[no_mangle]
pub unsafe extern "C" fn cpcap_sense_virq(regmap: *mut regmap, virq: c_int) -> c_int {
    int cpcap_sense_virq(struct regmap *regmap, int virq)
    {
    struct regmap_irq_chip_data *d = irq_get_chip_data(virq);
    let mut irq_base: c_int = regmap_irq_chip_get_base(d);
    return cpcap_sense_irq(regmap, virq - irq_base);
    }
    EXPORT_SYMBOL_GPL(cpcap_sense_virq);
#[no_mangle]
unsafe extern "C" fn cpcap_check_revision(cpcap: *mut cpcap_ddata) -> c_int {
    static int cpcap_check_revision(struct cpcap_ddata *cpcap)
    {
    u16 vendor, rev;
    int ret;
    ret = cpcap_get_vendor(&cpcap.spi.dev, cpcap.regmap, &vendor);
    if (ret)
    return ret;
    ret = cpcap_get_revision(&cpcap.spi.dev, cpcap.regmap, &rev);
    if (ret)
    return ret;
    dev_info(&cpcap.spi.dev, "CPCAP vendor: %s rev: %i.%i (%x)\n",
    vendor == CPCAP_VENDOR_ST ? "ST" : "TI",
    CPCAP_REVISION_MAJOR(rev), CPCAP_REVISION_MINOR(rev),
    rev);
    if (rev < CPCAP_REVISION_2_1) {
    dev_info(&cpcap.spi.dev,
    "Please add old CPCAP revision support as needed\n");
    return -ENODEV;
    }
    return 0;
    }
//
// First two irq chips are the two private macro interrupt chips, the third
// irq chip is for register banks 1 - 4 and is available for drivers to use.
//
    static struct regmap_irq_chip cpcap_irq_chip[CPCAP_NR_IRQ_CHIPS] = {
    {
    .name = "cpcap-m2",
    .num_regs = 1,
    .status_base = CPCAP_REG_MI1,
    .ack_base = CPCAP_REG_MI1,
    .mask_base = CPCAP_REG_MIM1,
    .use_ack = true,
    .clear_ack = true,
    },
    {
    .name = "cpcap-m2",
    .num_regs = 1,
    .status_base = CPCAP_REG_MI2,
    .ack_base = CPCAP_REG_MI2,
    .mask_base = CPCAP_REG_MIM2,
    .use_ack = true,
    .clear_ack = true,
    },
    {
    .name = "cpcap1-4",
    .num_regs = 4,
    .status_base = CPCAP_REG_INT1,
    .ack_base = CPCAP_REG_INT1,
    .mask_base = CPCAP_REG_INTM1,
    .use_ack = true,
    .clear_ack = true,
    },
    };
    static void cpcap_init_one_regmap_irq(struct cpcap_ddata *cpcap,
    struct regmap_irq *rirq,
    int irq_base, int irq)
    {
    unsigned int reg_offset;
    unsigned int bit, mask;
    reg_offset = irq - irq_base;
    reg_offset /= cpcap.regmap_conf.val_bits;
    reg_offset *= cpcap.regmap_conf.reg_stride;
    bit = irq % cpcap.regmap_conf.val_bits;
    mask = (1 << bit);
    rirq.reg_offset = reg_offset;
    rirq.mask = mask;
    }
    static int cpcap_init_irq_chip(struct cpcap_ddata *cpcap, int irq_chip,
    int irq_start, int nr_irqs)
    {
    struct regmap_irq_chip *chip = &cpcap_irq_chip[irq_chip];
    int i, ret;
    for (i = irq_start; i < irq_start + nr_irqs; i++) {
    struct regmap_irq *rirq = &cpcap.irqs[i];
    cpcap_init_one_regmap_irq(cpcap, rirq, irq_start, i);
    }
    chip.irqs = &cpcap.irqs[irq_start];
    chip.num_irqs = nr_irqs;
    chip.irq_drv_data = cpcap;
    ret = devm_regmap_add_irq_chip(&cpcap.spi.dev, cpcap.regmap,
    cpcap.spi.irq,
    irq_get_trigger_type(cpcap.spi.irq) |
    IRQF_SHARED, -1,
    chip, &cpcap.irqdata[irq_chip]);
    if (ret) {
    dev_err(&cpcap.spi.dev, "could not add irq chip %i: %i\n",
    irq_chip, ret);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cpcap_init_irq(cpcap: *mut cpcap_ddata) -> c_int {
    static int cpcap_init_irq(struct cpcap_ddata *cpcap)
    {
    int ret;
    cpcap.irqs = devm_kzalloc(&cpcap.spi.dev,
    array3_size(sizeof(*cpcap.irqs),
    CPCAP_NR_IRQ_REG_BANKS,
    cpcap.regmap_conf.val_bits),
    GFP_KERNEL);
    if (!cpcap.irqs)
    return -ENOMEM;
    ret = cpcap_init_irq_chip(cpcap, 0, 0, 16);
    if (ret)
    return ret;
    ret = cpcap_init_irq_chip(cpcap, 1, 16, 16);
    if (ret)
    return ret;
    ret = cpcap_init_irq_chip(cpcap, 2, 32, 64);
    if (ret)
    return ret;
    enable_irq_wake(cpcap.spi.irq);
    return 0;
    }
    static const struct of_device_id cpcap_of_match[] = {
    { .compatible = "motorola,cpcap", },
    { .compatible = "st,6556002", },
    {},
    };
    MODULE_DEVICE_TABLE(of, cpcap_of_match);
    static const struct spi_device_id cpcap_spi_ids[] = {
    { .name = "cpcap" },
    { .name = "6556002" },
    { }
    };
    MODULE_DEVICE_TABLE(spi, cpcap_spi_ids);
    static const struct regmap_config cpcap_regmap_config = {
    .reg_bits = 16,
    .reg_stride = 4,
    .pad_bits = 0,
    .val_bits = 16,
    .write_flag_mask = 0x8000,
    .max_register = CPCAP_REG_ST_TEST2,
    .cache_type = REGCACHE_NONE,
    .reg_format_endian = REGMAP_ENDIAN_LITTLE,
    .val_format_endian = REGMAP_ENDIAN_LITTLE,
    };
#[no_mangle]
unsafe extern "C" fn cpcap_suspend(dev: *mut device) -> c_int {
    static int cpcap_suspend(struct device *dev)
    {
    struct spi_device *spi = to_spi_device(dev);
    disable_irq(spi.irq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cpcap_resume(dev: *mut device) -> c_int {
    static int cpcap_resume(struct device *dev)
    {
    struct spi_device *spi = to_spi_device(dev);
    enable_irq(spi.irq);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(cpcap_pm, cpcap_suspend, cpcap_resume);
    static const struct mfd_cell cpcap_mfd_devices[] = {
    {
    .name          = "cpcap_adc",
    .of_compatible = "motorola,mapphone-cpcap-adc",
    }, {
    .name          = "cpcap_battery",
    .of_compatible = "motorola,cpcap-battery",
    }, {
    .name          = "cpcap-charger",
    .of_compatible = "motorola,mapphone-cpcap-charger",
    }, {
    .name          = "cpcap-regulator",
    .of_compatible = "motorola,mapphone-cpcap-regulator",
    }, {
    .name          = "cpcap-rtc",
    .of_compatible = "motorola,cpcap-rtc",
    }, {
    .name          = "cpcap-pwrbutton",
    .of_compatible = "motorola,cpcap-pwrbutton",
    }, {
    .name          = "cpcap-usb-phy",
    .of_compatible = "motorola,mapphone-cpcap-usb-phy",
    }, {
    .name          = "cpcap-led",
    .id            = 0,
    .of_compatible = "motorola,cpcap-led-red",
    }, {
    .name          = "cpcap-led",
    .id            = 1,
    .of_compatible = "motorola,cpcap-led-green",
    }, {
    .name          = "cpcap-led",
    .id            = 2,
    .of_compatible = "motorola,cpcap-led-blue",
    }, {
    .name          = "cpcap-led",
    .id            = 3,
    .of_compatible = "motorola,cpcap-led-adl",
    }, {
    .name          = "cpcap-led",
    .id            = 4,
    .of_compatible = "motorola,cpcap-led-cp",
    }, {
    .name          = "cpcap-codec",
    }
    };
#[no_mangle]
unsafe extern "C" fn cpcap_probe(spi: *mut spi_device) -> c_int {
    static int cpcap_probe(struct spi_device *spi)
    {
    struct cpcap_ddata *cpcap;
    int ret;
    cpcap = devm_kzalloc(&spi.dev, sizeof(*cpcap), GFP_KERNEL);
    if (!cpcap)
    return -ENOMEM;
    cpcap.spi = spi;
    spi_set_drvdata(spi, cpcap);
    spi.bits_per_word = 16;
    spi.mode = SPI_MODE_0 | SPI_CS_HIGH;
    ret = spi_setup(spi);
    if (ret)
    return ret;
    cpcap.regmap_conf = &cpcap_regmap_config;
    cpcap.regmap = devm_regmap_init_spi(spi, &cpcap_regmap_config);
    if (IS_ERR(cpcap.regmap)) {
    ret = PTR_ERR(cpcap.regmap);
    dev_err(&cpcap.spi.dev, "Failed to initialize regmap: %d\n",
    ret);
    return ret;
    }
    ret = cpcap_check_revision(cpcap);
    if (ret) {
    dev_err(&cpcap.spi.dev, "Failed to detect CPCAP: %i\n", ret);
    return ret;
    }
    ret = cpcap_init_irq(cpcap);
    if (ret)
    return ret;
// Parent SPI controller uses DMA, CPCAP and child devices do not
    spi.dev.coherent_dma_mask = 0;
    spi.dev.dma_mask = &spi.dev.coherent_dma_mask;
    return devm_mfd_add_devices(&spi.dev, 0, cpcap_mfd_devices,
    ARRAY_SIZE(cpcap_mfd_devices), core::ptr::null_mut(), 0, core::ptr::null_mut());
    }
    static struct spi_driver cpcap_driver = {
    .driver = {
    .name = "cpcap-core",
    .of_match_table = cpcap_of_match,
    .pm = pm_sleep_ptr(&cpcap_pm),
    },
    .probe = cpcap_probe,
    .id_table = cpcap_spi_ids,
    };
    module_spi_driver(cpcap_driver);
    MODULE_ALIAS("platform:cpcap");
    MODULE_DESCRIPTION("CPCAP driver");
    MODULE_AUTHOR("Tony Lindgren <tony@atomide.com>");
    MODULE_LICENSE("GPL v2");
