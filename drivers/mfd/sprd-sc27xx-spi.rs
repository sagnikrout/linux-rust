//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/sprd-sc27xx-spi.c
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
// Copyright (C) 2017 Spreadtrum Communications Inc.
//

    enum sprd_pmic_type {
    PMIC_TYPE_SC2730 = 1,
    PMIC_TYPE_SC2731,
    };
pub const SPRD_PMIC_INT_MASK_STATUS: c_uint = 0x0;
pub const SPRD_PMIC_INT_RAW_STATUS: c_uint = 0x4;
pub const SPRD_PMIC_INT_EN: c_uint = 0x8;
pub const SPRD_SC2730_IRQ_BASE: c_uint = 0x80;
pub const SPRD_SC2730_IRQ_NUMS: c_int = 10;
pub const SPRD_SC2730_CHG_DET: c_uint = 0x1b9c;
pub const SPRD_SC2731_IRQ_BASE: c_uint = 0x140;
pub const SPRD_SC2731_IRQ_NUMS: c_int = 16;
pub const SPRD_SC2731_CHG_DET: c_uint = 0xedc;
// PMIC charger detection definition
pub const SPRD_PMIC_CHG_DET_DELAY_US: c_int = 200000;
pub const SPRD_PMIC_CHG_DET_TIMEOUT: c_int = 2000000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sprd_pmic {
    pub regmap: *mut regmap,
    pub dev: *mut device,
    pub irqs: *mut regmap_irq,
    pub irq_chip: regmap_irq_chip,
    pub irq_data: *mut regmap_irq_chip_data,
    pub pdata: *const sprd_pmic_data,
    pub irq: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sprd_pmic_data {
    pub irq_base: u32,
    pub num_irqs: u32,
    pub charger_det: u32,
}

    static const struct mfd_cell sc2730_devices[] = {
    MFD_CELL_OF("sc2730-adc", core::ptr::null_mut(), core::ptr::null_mut(), 0, 0, "sprd,sc2730-adc"),
    MFD_CELL_OF("sc2730-bltc", core::ptr::null_mut(), core::ptr::null_mut(), 0, 0, "sprd,sc2730-bltc"),
    MFD_CELL_OF("sc2730-efuse", core::ptr::null_mut(), core::ptr::null_mut(), 0, 0, "sprd,sc2730-efuse"),
    MFD_CELL_OF("sc2730-eic", core::ptr::null_mut(), core::ptr::null_mut(), 0, 0, "sprd,sc2730-eic"),
    MFD_CELL_OF("sc2730-fgu", core::ptr::null_mut(), core::ptr::null_mut(), 0, 0, "sprd,sc2730-fgu"),
    MFD_CELL_NAME("sc2730-regulator"),
    MFD_CELL_OF("sc2730-rtc", core::ptr::null_mut(), core::ptr::null_mut(), 0, 0, "sprd,sc2730-rtc"),
    MFD_CELL_OF("sc2730-vibrator", core::ptr::null_mut(), core::ptr::null_mut(), 0, 0, "sprd,sc2730-vibrator"),
    };
    static const struct mfd_cell sc2731_devices[] = {
    MFD_CELL_OF("sc2731-adc", core::ptr::null_mut(), core::ptr::null_mut(), 0, 0, "sprd,sc2731-adc"),
    MFD_CELL_OF("sc2731-bltc", core::ptr::null_mut(), core::ptr::null_mut(), 0, 0, "sprd,sc2731-bltc"),
    MFD_CELL_OF("sc2731-charger", core::ptr::null_mut(), core::ptr::null_mut(), 0, 0, "sprd,sc2731-charger"),
    MFD_CELL_OF("sc2731-efuse", core::ptr::null_mut(), core::ptr::null_mut(), 0, 0, "sprd,sc2731-efuse"),
    MFD_CELL_OF("sc2731-eic", core::ptr::null_mut(), core::ptr::null_mut(), 0, 0, "sprd,sc2731-eic"),
    MFD_CELL_OF("sc2731-fgu", core::ptr::null_mut(), core::ptr::null_mut(), 0, 0, "sprd,sc2731-fgu"),
    MFD_CELL_NAME("sc2731-poweroff"),
    MFD_CELL_NAME("sc2731-regulator"),
    MFD_CELL_OF("sc2731-rtc", core::ptr::null_mut(), core::ptr::null_mut(), 0, 0, "sprd,sc2731-rtc"),
    MFD_CELL_OF("sc2731-vibrator", core::ptr::null_mut(), core::ptr::null_mut(), 0, 0, "sprd,sc2731-vibrator"),
    };
//
// Since different PMICs of SC27xx series can have different interrupt
// base address and irq number, we should save irq number and irq base
// in the device data structure.
//
    static const struct sprd_pmic_data sc2730_data = {
    .irq_base = SPRD_SC2730_IRQ_BASE,
    .num_irqs = SPRD_SC2730_IRQ_NUMS,
    .charger_det = SPRD_SC2730_CHG_DET,
    };
    static const struct sprd_pmic_data sc2731_data = {
    .irq_base = SPRD_SC2731_IRQ_BASE,
    .num_irqs = SPRD_SC2731_IRQ_NUMS,
    .charger_det = SPRD_SC2731_CHG_DET,
    };
#[no_mangle]
pub unsafe extern "C" fn sprd_pmic_detect_charger_type(dev: *mut device) -> enum usb_charger_type {
    enum usb_charger_type sprd_pmic_detect_charger_type(struct device *dev)
    {
    struct spi_device *spi = to_spi_device(dev);
    struct sprd_pmic *ddata = spi_get_drvdata(spi);
    const struct sprd_pmic_data *pdata = ddata.pdata;
    enum usb_charger_type type;
    u32 val;
    int ret;
    ret = regmap_read_poll_timeout(ddata.regmap, pdata.charger_det, val,
    (val & SPRD_PMIC_CHG_DET_DONE),
    SPRD_PMIC_CHG_DET_DELAY_US,
    SPRD_PMIC_CHG_DET_TIMEOUT);
    if (ret) {
    dev_err(&spi.dev, "failed to detect charger type\n");
    return UNKNOWN_TYPE;
    }
    switch (val & SPRD_PMIC_CHG_TYPE_MASK) {
    case SPRD_PMIC_CDP_TYPE:
    type = CDP_TYPE;
    break;
    case SPRD_PMIC_DCP_TYPE:
    type = DCP_TYPE;
    break;
    case SPRD_PMIC_SDP_TYPE:
    type = SDP_TYPE;
    break;
    default:
    type = UNKNOWN_TYPE;
    break;
    }
    return type;
    }
    EXPORT_SYMBOL_GPL(sprd_pmic_detect_charger_type);
#[no_mangle]
unsafe extern "C" fn sprd_pmic_spi_write(context: *mut c_void, data: *const c_void, count: usize) -> c_int {
    static int sprd_pmic_spi_write(void *context, const void *data, size_t count)
    {
    struct device *dev = context;
    struct spi_device *spi = to_spi_device(dev);
    return spi_write(spi, data, count);
    }
    static int sprd_pmic_spi_read(void *context,
    const void *reg, size_t reg_size,
    void *val, size_t val_size)
    {
    struct device *dev = context;
    struct spi_device *spi = to_spi_device(dev);
    u32 rx_buf[2] = { 0 };
    int ret;
// Now we only support one PMIC register to read every time.
    if (reg_size != sizeof(u32) || val_size != sizeof(u32))
    return -EINVAL;
// Copy address to read from into first element of SPI buffer.
    memcpy(rx_buf, reg, sizeof(u32));
    ret = spi_read(spi, rx_buf, 1);
    if (ret < 0)
    return ret;
    memcpy(val, rx_buf, val_size);
    return 0;
    }
    static const struct regmap_bus sprd_pmic_regmap = {
    .write = sprd_pmic_spi_write,
    .read = sprd_pmic_spi_read,
    .reg_format_endian_default = REGMAP_ENDIAN_NATIVE,
    .val_format_endian_default = REGMAP_ENDIAN_NATIVE,
    };
    static const struct regmap_config sprd_pmic_config = {
    .reg_bits = 32,
    .val_bits = 32,
    .reg_stride = 4,
    .max_register = 0xffff,
    };
#[no_mangle]
unsafe extern "C" fn sprd_pmic_probe(spi: *mut spi_device) -> c_int {
    static int sprd_pmic_probe(struct spi_device *spi)
    {
    struct sprd_pmic *ddata;
    enum sprd_pmic_type pmic_type;
    const struct sprd_pmic_data *pdata;
    const struct mfd_cell *cells;
    int ret, i, num_cells;
    pmic_type = (uintptr_t)of_device_get_match_data(&spi.dev);
    switch (pmic_type) {
    case PMIC_TYPE_SC2730:
    pdata = &sc2730_data;
    cells = sc2730_devices;
    num_cells = ARRAY_SIZE(sc2730_devices);
    break;
    case PMIC_TYPE_SC2731:
    pdata = &sc2731_data;
    cells = sc2731_devices;
    num_cells = ARRAY_SIZE(sc2731_devices);
    break;
    default:
    dev_err(&spi.dev, "Invalid device ID\n");
    return -EINVAL;
    }
    ddata = devm_kzalloc(&spi.dev, sizeof(*ddata), GFP_KERNEL);
    if (!ddata)
    return -ENOMEM;
    ddata.regmap = devm_regmap_init(&spi.dev, &sprd_pmic_regmap,
    &spi.dev, &sprd_pmic_config);
    if (IS_ERR(ddata.regmap)) {
    ret = PTR_ERR(ddata.regmap);
    dev_err(&spi.dev, "Failed to allocate register map %d\n", ret);
    return ret;
    }
    spi_set_drvdata(spi, ddata);
    ddata.dev = &spi.dev;
    ddata.irq = spi.irq;
    ddata.pdata = pdata;
    ddata.irq_chip.name = dev_name(&spi.dev);
    ddata.irq_chip.status_base =
    pdata.irq_base + SPRD_PMIC_INT_MASK_STATUS;
    ddata.irq_chip.unmask_base = pdata.irq_base + SPRD_PMIC_INT_EN;
    ddata.irq_chip.ack_base = 0;
    ddata.irq_chip.num_regs = 1;
    ddata.irq_chip.num_irqs = pdata.num_irqs;
    ddata.irqs = devm_kcalloc(&spi.dev,
    pdata.num_irqs, sizeof(struct regmap_irq),
    GFP_KERNEL);
    if (!ddata.irqs)
    return -ENOMEM;
    ddata.irq_chip.irqs = ddata.irqs;
    for (i = 0; i < pdata.num_irqs; i++)
    ddata.irqs[i].mask = BIT(i);
    ret = devm_regmap_add_irq_chip(&spi.dev, ddata.regmap, ddata.irq,
    IRQF_ONESHOT, 0,
    &ddata.irq_chip, &ddata.irq_data);
    if (ret) {
    dev_err(&spi.dev, "Failed to add PMIC irq chip %d\n", ret);
    return ret;
    }
    ret = devm_mfd_add_devices(&spi.dev, PLATFORM_DEVID_AUTO,
    cells, num_cells, core::ptr::null_mut(), 0,
    regmap_irq_get_domain(ddata.irq_data));
    if (ret) {
    dev_err(&spi.dev, "Failed to populate sub-devices %d\n", ret);
    return ret;
    }
    ret = devm_device_init_wakeup(&spi.dev);
    if (ret)
    return dev_err_probe(&spi.dev, ret, "Failed to init wakeup\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sprd_pmic_suspend(dev: *mut device) -> c_int {
    static int sprd_pmic_suspend(struct device *dev)
    {
    struct sprd_pmic *ddata = dev_get_drvdata(dev);
    if (device_may_wakeup(dev))
    enable_irq_wake(ddata.irq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sprd_pmic_resume(dev: *mut device) -> c_int {
    static int sprd_pmic_resume(struct device *dev)
    {
    struct sprd_pmic *ddata = dev_get_drvdata(dev);
    if (device_may_wakeup(dev))
    disable_irq_wake(ddata.irq);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(sprd_pmic_pm_ops,
    sprd_pmic_suspend, sprd_pmic_resume);
    static const struct of_device_id sprd_pmic_match[] = {
    { .compatible = "sprd,sc2730", .data = (void *)PMIC_TYPE_SC2730 },
    { .compatible = "sprd,sc2731", .data = (void *)PMIC_TYPE_SC2731 },
    {},
    };
    MODULE_DEVICE_TABLE(of, sprd_pmic_match);
    static const struct spi_device_id sprd_pmic_spi_ids[] = {
    { .name = "sc2730", .driver_data = PMIC_TYPE_SC2730 },
    { .name = "sc2731", .driver_data = PMIC_TYPE_SC2731 },
    { }
    };
    MODULE_DEVICE_TABLE(spi, sprd_pmic_spi_ids);
    static struct spi_driver sprd_pmic_driver = {
    .driver = {
    .name = "sc27xx-pmic",
    .of_match_table = sprd_pmic_match,
    .pm = pm_sleep_ptr(&sprd_pmic_pm_ops),
    },
    .probe = sprd_pmic_probe,
    .id_table = sprd_pmic_spi_ids,
    };
#[no_mangle]
unsafe extern "C" fn sprd_pmic_init() -> int __init {
    static int __init sprd_pmic_init(void)
    {
    return spi_register_driver(&sprd_pmic_driver);
    }
    subsys_initcall(sprd_pmic_init);
#[no_mangle]
unsafe extern "C" fn sprd_pmic_exit() -> void __exit {
    static void __exit sprd_pmic_exit(void)
    {
    spi_unregister_driver(&sprd_pmic_driver);
    }
    module_exit(sprd_pmic_exit);
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("Spreadtrum SC27xx PMICs driver");
    MODULE_AUTHOR("Baolin Wang <baolin.wang@spreadtrum.com>");
