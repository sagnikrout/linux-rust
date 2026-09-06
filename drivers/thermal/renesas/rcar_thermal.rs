//! Automatically rewritten from C to Rust
//! Source: drivers/thermal/renesas/rcar_thermal.c
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
// R-Car THS/TSC thermal sensor driver
//
// Copyright (C) 2012 Renesas Solutions Corp.
// Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>
//

pub const IDLE_INTERVAL: c_int = 5000;
pub const COMMON_STR: c_uint = 0x00;
pub const COMMON_ENR: c_uint = 0x04;
pub const COMMON_INTMSK: c_uint = 0x0c;
pub const REG_POSNEG: c_uint = 0x20;
pub const REG_FILONOFF: c_uint = 0x28;
pub const REG_THSCR: c_uint = 0x2c;
pub const REG_THSSR: c_uint = 0x30;
pub const REG_INTCTRL: c_uint = 0x34;
// THSCR

// THSSR
pub const CTEMP: c_uint = 0x3f;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcar_thermal_common {
    pub base: *mut void __iomem,
    pub dev: *mut device,
    pub head: list_head,
    pub lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcar_thermal_chip {
    pub 1: unsigned int use_of_thermal :,
    pub 1: unsigned int has_filonoff :,
    pub 1: unsigned int irq_per_ch :,
    pub 1: unsigned int needs_suspend_resume :,
    pub nirqs: c_uint,
    pub ctemp_bands: c_uint,
}

    static const struct rcar_thermal_chip rcar_thermal = {
    .use_of_thermal = 0,
    .has_filonoff = 1,
    .irq_per_ch = 0,
    .needs_suspend_resume = 0,
    .nirqs = 1,
    .ctemp_bands = 1,
    };
    static const struct rcar_thermal_chip rcar_gen2_thermal = {
    .use_of_thermal = 1,
    .has_filonoff = 1,
    .irq_per_ch = 0,
    .needs_suspend_resume = 0,
    .nirqs = 1,
    .ctemp_bands = 1,
    };
    static const struct rcar_thermal_chip rcar_gen3_thermal = {
    .use_of_thermal = 1,
    .has_filonoff = 0,
    .irq_per_ch = 1,
    .needs_suspend_resume = 1,
//
// The Gen3 chip has 3 interrupts, but this driver uses only 2
// interrupts to detect a temperature change, rise or fall.
//
    .nirqs = 2,
    .ctemp_bands = 2,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcar_thermal_priv {
    pub base: *mut void __iomem,
    pub common: *mut rcar_thermal_common,
    pub zone: *mut thermal_zone_device,
    pub chip: *const rcar_thermal_chip,
    pub work: delayed_work,
    pub lock: mutex,
    pub list: list_head,
    pub id: c_int,
}

    list_for_each_entry(pos, &common.head, list)

    static const struct of_device_id rcar_thermal_dt_ids[] = {
    {
    .compatible = "renesas,rcar-thermal",
    .data = &rcar_thermal,
    },
    {
    .compatible = "renesas,rcar-gen2-thermal",
    .data = &rcar_gen2_thermal,
    },
    {
    .compatible = "renesas,thermal-r8a774c0",
    .data = &rcar_gen3_thermal,
    },
    {
    .compatible = "renesas,thermal-r8a77970",
    .data = &rcar_gen3_thermal,
    },
    {
    .compatible = "renesas,thermal-r8a77990",
    .data = &rcar_gen3_thermal,
    },
    {
    .compatible = "renesas,thermal-r8a77995",
    .data = &rcar_gen3_thermal,
    },
    {},
    };
    MODULE_DEVICE_TABLE(of, rcar_thermal_dt_ids);
//
// basic functions
//

    _rcar_thermal_common_read(c, COMMON_ ##r)
    static u32 _rcar_thermal_common_read(struct rcar_thermal_common *common,
    u32 reg)
    {
    return ioread32(common.base + reg);
    }

    _rcar_thermal_common_write(c, COMMON_ ##r, d)
    static void _rcar_thermal_common_write(struct rcar_thermal_common *common,
    u32 reg, u32 data)
    {
    iowrite32(data, common.base + reg);
    }

    _rcar_thermal_common_bset(c, COMMON_ ##r, m, d)
    static void _rcar_thermal_common_bset(struct rcar_thermal_common *common,
    u32 reg, u32 mask, u32 data)
    {
    u32 val;
    val = ioread32(common.base + reg);
    val &= ~mask;
    val |= (data & mask);
    iowrite32(val, common.base + reg);
    }

#[no_mangle]
unsafe extern "C" fn _rcar_thermal_read(priv: *mut rcar_thermal_priv, reg: u32) -> u32 {
    static u32 _rcar_thermal_read(struct rcar_thermal_priv *priv, u32 reg)
    {
    return ioread32(priv.base + reg);
    }

    static void _rcar_thermal_write(struct rcar_thermal_priv *priv,
    u32 reg, u32 data)
    {
    iowrite32(data, priv.base + reg);
    }

    static void _rcar_thermal_bset(struct rcar_thermal_priv *priv, u32 reg,
    u32 mask, u32 data)
    {
    u32 val;
    val = ioread32(priv.base + reg);
    val &= ~mask;
    val |= (data & mask);
    iowrite32(val, priv.base + reg);
    }
//
// zone device functions
//
#[no_mangle]
unsafe extern "C" fn rcar_thermal_update_temp(priv: *mut rcar_thermal_priv) -> c_int {
    static int rcar_thermal_update_temp(struct rcar_thermal_priv *priv)
    {
    struct device *dev = rcar_priv_to_dev(priv);
    int old, new, ctemp = -EINVAL;
    unsigned int i;
    mutex_lock(&priv.lock);
//
// TSC decides a value of CPTAP automatically,
// and this is the conditions which validate interrupt.
//
    rcar_thermal_bset(priv, THSCR, CPCTL, CPCTL);
    old = ~0;
    for (i = 0; i < 128; i++) {
//
// we need to wait 300us after changing comparator offset
// to get stable temperature.
// see "Usage Notes" on datasheet
//
    usleep_range(300, 400);
    new = rcar_thermal_read(priv, THSSR) & CTEMP;
    if (new == old) {
    ctemp = new;
    break;
    }
    old = new;
    }
    if (ctemp < 0) {
    dev_err(dev, "thermal sensor was broken\n");
    goto err_out_unlock;
    }
//
// enable IRQ
//
    if (rcar_has_irq_support(priv)) {
    if (priv.chip.has_filonoff)
    rcar_thermal_write(priv, FILONOFF, 0);
// enable Rising/Falling edge interrupt
    rcar_thermal_write(priv, POSNEG,  0x1);
    rcar_thermal_write(priv, INTCTRL, (((ctemp - 0) << 8) |
    ((ctemp - 1) << 0)));
    }
    err_out_unlock:
    mutex_unlock(&priv.lock);
    return ctemp;
    }
    static int rcar_thermal_get_current_temp(struct rcar_thermal_priv *priv,
    int *temp)
    {
    int ctemp;
    ctemp = rcar_thermal_update_temp(priv);
    if (ctemp < 0)
    return ctemp;
// Guaranteed operating range is -45C to 125C.
    if (priv.chip.ctemp_bands == 1)
// temp = MCELSIUS((ctemp * 5) - 65);
#[no_mangle]
pub unsafe extern "C" fn if(24: ctemp <) -> else {
    else if (ctemp < 24)
// temp = MCELSIUS(((ctemp * 55) - 720) / 10);
    else
// temp = MCELSIUS((ctemp * 5) - 60);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rcar_thermal_get_temp(zone: *mut thermal_zone_device, temp: *mut c_int) -> c_int {
    static int rcar_thermal_get_temp(struct thermal_zone_device *zone, int *temp)
    {
    struct rcar_thermal_priv *priv = thermal_zone_device_priv(zone);
    return rcar_thermal_get_current_temp(priv, temp);
    }
    static const struct thermal_zone_device_ops rcar_thermal_zone_ops = {
    .get_temp	= rcar_thermal_get_temp,
    };
    static struct thermal_trip trips[] = {
    { .type = THERMAL_TRIP_CRITICAL, .temperature = 90000 }
    };
//
// interrupt
//

#[no_mangle]
unsafe extern "C" fn _rcar_thermal_irq_ctrl(priv: *mut rcar_thermal_priv, enable: c_int) {
    static void _rcar_thermal_irq_ctrl(struct rcar_thermal_priv *priv, int enable)
    {
    struct rcar_thermal_common *common = priv.common;
    unsigned long flags;
    u32 mask = 0x3 << rcar_id_to_shift(priv); /* enable Rising/Falling */
    if (!rcar_has_irq_support(priv))
    return;
    spin_lock_irqsave(&common.lock, flags);
    rcar_thermal_common_bset(common, INTMSK, mask, enable ? 0 : mask);
    spin_unlock_irqrestore(&common.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn rcar_thermal_work(work: *mut work_struct) {
    static void rcar_thermal_work(struct work_struct *work)
    {
    struct rcar_thermal_priv *priv;
    int ret;
    priv = container_of(work, struct rcar_thermal_priv, work.work);
    ret = rcar_thermal_update_temp(priv);
    if (ret < 0)
    return;
    rcar_thermal_irq_enable(priv);
    thermal_zone_device_update(priv.zone, THERMAL_EVENT_UNSPECIFIED);
    }
#[no_mangle]
unsafe extern "C" fn rcar_thermal_had_changed(priv: *mut rcar_thermal_priv, status: u32) -> u32 {
    static u32 rcar_thermal_had_changed(struct rcar_thermal_priv *priv, u32 status)
    {
    struct device *dev = rcar_priv_to_dev(priv);
    status = (status >> rcar_id_to_shift(priv)) & 0x3;
    if (status) {
    dev_dbg(dev, "thermal%d %s%s\n",
    priv.id,
    (status & 0x2) ? "Rising " : "",
    (status & 0x1) ? "Falling" : "");
    }
    return status;
    }
#[no_mangle]
unsafe extern "C" fn rcar_thermal_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t rcar_thermal_irq(int irq, void *data)
    {
    struct rcar_thermal_common *common = data;
    struct rcar_thermal_priv *priv;
    u32 status, mask;
    spin_lock(&common.lock);
    mask	= rcar_thermal_common_read(common, INTMSK);
    status	= rcar_thermal_common_read(common, STR);
    rcar_thermal_common_write(common, STR, 0x000F0F0F & mask);
    spin_unlock(&common.lock);
    status = status & ~mask;
//
// check the status
//
    rcar_thermal_for_each_priv(priv, common) {
    if (rcar_thermal_had_changed(priv, status)) {
    rcar_thermal_irq_disable(priv);
    queue_delayed_work(system_freezable_wq, &priv.work,
    msecs_to_jiffies(300));
    }
    }
    return IRQ_HANDLED;
    }
//
// platform functions
//
#[no_mangle]
unsafe extern "C" fn rcar_thermal_remove(pdev: *mut platform_device) {
    static void rcar_thermal_remove(struct platform_device *pdev)
    {
    struct rcar_thermal_common *common = platform_get_drvdata(pdev);
    struct device *dev = &pdev.dev;
    struct rcar_thermal_priv *priv;
    rcar_thermal_for_each_priv(priv, common) {
    rcar_thermal_irq_disable(priv);
    cancel_delayed_work_sync(&priv.work);
    if (priv.chip.use_of_thermal)
    thermal_remove_hwmon_sysfs(priv.zone);
    else
    thermal_zone_device_unregister(priv.zone);
    }
    pm_runtime_put(dev);
    pm_runtime_disable(dev);
    }
#[no_mangle]
unsafe extern "C" fn rcar_thermal_probe(pdev: *mut platform_device) -> c_int {
    static int rcar_thermal_probe(struct platform_device *pdev)
    {
    struct rcar_thermal_common *common;
    struct rcar_thermal_priv *priv;
    struct device *dev = &pdev.dev;
    struct resource *res;
    const struct rcar_thermal_chip *chip = of_device_get_match_data(dev);
    let mut mres: c_int = 0;
    int i;
    let mut ret: c_int = -ENODEV;
    let mut idle: c_int = IDLE_INTERVAL;
    let mut enr_bits: u32 = 0;
    common = devm_kzalloc(dev, sizeof(*common), GFP_KERNEL);
    if (!common)
    return -ENOMEM;
    platform_set_drvdata(pdev, common);
    INIT_LIST_HEAD(&common.head);
    spin_lock_init(&common.lock);
    common.dev = dev;
    pm_runtime_enable(dev);
    pm_runtime_get_sync(dev);
    for (i = 0; i < chip.nirqs; i++) {
    int irq;
    ret = platform_get_irq_optional(pdev, i);
    if (ret < 0 && ret != -ENXIO)
    goto error_unregister;
    if (ret > 0)
    irq = ret;
    else
    break;
    if (!common.base) {
//
// platform has IRQ support.
// Then, driver uses common registers
// rcar_has_irq_support() will be enabled
//
    res = platform_get_resource(pdev, IORESOURCE_MEM,
    mres++);
    common.base = devm_ioremap_resource(dev, res);
    if (IS_ERR(common.base)) {
    ret = PTR_ERR(common.base);
    goto error_unregister;
    }
    idle = 0; /* polling delay is not needed */
    }
    ret = devm_request_irq(dev, irq, rcar_thermal_irq,
    IRQF_SHARED, dev_name(dev), common);
    if (ret)
    goto error_unregister;
// update ENR bits
    if (chip.irq_per_ch)
    enr_bits |= 1 << i;
    }
    for (i = 0;; i++) {
    res = platform_get_resource(pdev, IORESOURCE_MEM, mres++);
    if (!res)
    break;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv) {
    ret = -ENOMEM;
    goto error_unregister;
    }
    priv.base = devm_ioremap_resource(dev, res);
    if (IS_ERR(priv.base)) {
    ret = PTR_ERR(priv.base);
    goto error_unregister;
    }
    priv.common = common;
    priv.id = i;
    priv.chip = chip;
    mutex_init(&priv.lock);
    INIT_LIST_HEAD(&priv.list);
    INIT_DELAYED_WORK(&priv.work, rcar_thermal_work);
    ret = rcar_thermal_update_temp(priv);
    if (ret < 0)
    goto error_unregister;
    if (chip.use_of_thermal) {
    priv.zone = devm_thermal_of_zone_register(
    dev, i, priv,
    &rcar_thermal_zone_ops);
    } else {
    priv.zone = thermal_zone_device_register_with_trips(
    "rcar_thermal", trips, ARRAY_SIZE(trips), priv,
    &rcar_thermal_zone_ops, core::ptr::null_mut(), 0,
    idle);
    }
    if (IS_ERR(priv.zone)) {
    dev_err(dev, "can't register thermal zone\n");
    ret = PTR_ERR(priv.zone);
    priv.zone = core::ptr::null_mut();
    goto error_unregister;
    }
    if (chip.use_of_thermal)
    ret = thermal_add_hwmon_sysfs(priv.zone);
    else
    ret = thermal_zone_device_enable(priv.zone);
    if (ret)
    goto error_unregister;
    rcar_thermal_irq_enable(priv);
    list_move_tail(&priv.list, &common.head);
// update ENR bits
    if (!chip.irq_per_ch)
    enr_bits |= 3 << (i * 8);
    }
    if (common.base && enr_bits)
    rcar_thermal_common_write(common, ENR, enr_bits);
    dev_info(dev, "%d sensor probed\n", i);
    return 0;
    error_unregister:
    rcar_thermal_remove(pdev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rcar_thermal_suspend(dev: *mut device) -> c_int {
    static int rcar_thermal_suspend(struct device *dev)
    {
    struct rcar_thermal_common *common = dev_get_drvdata(dev);
    struct rcar_thermal_priv *priv = list_first_entry(&common.head,
    typeof(*priv), list);
    if (priv.chip.needs_suspend_resume) {
    rcar_thermal_common_write(common, ENR, 0);
    rcar_thermal_irq_disable(priv);
    rcar_thermal_bset(priv, THSCR, CPCTL, 0);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rcar_thermal_resume(dev: *mut device) -> c_int {
    static int rcar_thermal_resume(struct device *dev)
    {
    struct rcar_thermal_common *common = dev_get_drvdata(dev);
    struct rcar_thermal_priv *priv = list_first_entry(&common.head,
    typeof(*priv), list);
    int ret;
    if (priv.chip.needs_suspend_resume) {
    ret = rcar_thermal_update_temp(priv);
    if (ret < 0)
    return ret;
    rcar_thermal_irq_enable(priv);
    rcar_thermal_common_write(common, ENR, 0x03);
    }
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(rcar_thermal_pm_ops, rcar_thermal_suspend,
    rcar_thermal_resume);
    static struct platform_driver rcar_thermal_driver = {
    .driver	= {
    .name	= "rcar_thermal",
    .pm = pm_sleep_ptr(&rcar_thermal_pm_ops),
    .of_match_table = rcar_thermal_dt_ids,
    },
    .probe		= rcar_thermal_probe,
    .remove		= rcar_thermal_remove,
    };
    module_platform_driver(rcar_thermal_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("R-Car THS/TSC thermal sensor driver");
    MODULE_AUTHOR("Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>");
