//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-mlxbf2.c
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
// Copyright (C) 2020-2021 NVIDIA CORPORATION & AFFILIATES
//

//
// There are 3 YU GPIO blocks:
// gpio[0]: HOST_GPIO0->HOST_GPIO31
// gpio[1]: HOST_GPIO32->HOST_GPIO63
// gpio[2]: HOST_GPIO64->HOST_GPIO69
//
pub const MLXBF2_GPIO_MAX_PINS_PER_BLOCK: c_int = 32;
//
// arm_gpio_lock register:
// bit[31]	lock status: active if set
// bit[15:0]	set lock
// The lock is enabled only if 0xd42f is written to this field
//
pub const YU_ARM_GPIO_LOCK_ADDR: c_uint = 0x2801088;
pub const YU_ARM_GPIO_LOCK_SIZE: c_uint = 0x8;

pub const YU_ARM_GPIO_LOCK_ACQUIRE: c_uint = 0xd42f;
pub const YU_ARM_GPIO_LOCK_RELEASE: c_uint = 0x0;
//
// gpio[x] block registers and their offset
//
pub const YU_GPIO_DATAIN: c_uint = 0x04;
pub const YU_GPIO_MODE1: c_uint = 0x08;
pub const YU_GPIO_MODE0: c_uint = 0x0c;
pub const YU_GPIO_DATASET: c_uint = 0x14;
pub const YU_GPIO_DATACLEAR: c_uint = 0x18;
pub const YU_GPIO_CAUSE_RISE_EN: c_uint = 0x44;
pub const YU_GPIO_CAUSE_FALL_EN: c_uint = 0x48;
pub const YU_GPIO_MODE1_CLEAR: c_uint = 0x50;
pub const YU_GPIO_MODE0_SET: c_uint = 0x54;
pub const YU_GPIO_MODE0_CLEAR: c_uint = 0x58;
pub const YU_GPIO_CAUSE_OR_CAUSE_EVTEN0: c_uint = 0x80;
pub const YU_GPIO_CAUSE_OR_EVTEN0: c_uint = 0x94;
pub const YU_GPIO_CAUSE_OR_CLRCAUSE: c_uint = 0x98;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxbf2_gpio_context_save_regs {
    pub gpio_mode0: u32,
    pub gpio_mode1: u32,
}

// BlueField-2 gpio block context structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxbf2_gpio_context {
    pub chip: gpio_generic_chip,
// YU GPIO blocks address
    pub gpio_io: *mut void __iomem,
    pub dev: *mut device,
    pub csave_regs: *mut mlxbf2_gpio_context_save_regs,
}

// BlueField-2 gpio shared structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxbf2_gpio_param {
    pub io: *mut void __iomem,
    pub res: *mut resource,
    pub lock: *mut mutex,
}

    static struct resource yu_arm_gpio_lock_res =
    DEFINE_RES_MEM_NAMED(YU_ARM_GPIO_LOCK_ADDR, YU_ARM_GPIO_LOCK_SIZE, "YU_ARM_GPIO_LOCK");
    static DEFINE_MUTEX(yu_arm_gpio_lock_mutex);
    static struct mlxbf2_gpio_param yu_arm_gpio_lock_param = {
    .res = &yu_arm_gpio_lock_res,
    .lock = &yu_arm_gpio_lock_mutex,
    };
// Request memory region and map yu_arm_gpio_lock resource
#[no_mangle]
unsafe extern "C" fn mlxbf2_gpio_get_lock_res(pdev: *mut platform_device) -> c_int {
    static int mlxbf2_gpio_get_lock_res(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct resource *res;
    resource_size_t size;
    let mut ret: c_int = 0;
    mutex_lock(yu_arm_gpio_lock_param.lock);
// Check if the memory map already exists
    if (yu_arm_gpio_lock_param.io)
    goto exit;
    res = yu_arm_gpio_lock_param.res;
    size = resource_size(res);
    if (!devm_request_mem_region(dev, res.start, size, res.name)) {
    ret = -EFAULT;
    goto exit;
    }
    yu_arm_gpio_lock_param.io = devm_ioremap(dev, res.start, size);
    if (!yu_arm_gpio_lock_param.io)
    ret = -ENOMEM;
    exit:
    mutex_unlock(yu_arm_gpio_lock_param.lock);
    return ret;
    }
//
// Acquire the YU arm_gpio_lock to be able to change the direction
// mode. If the lock_active bit is already set, return an error.
//
#[no_mangle]
unsafe extern "C" fn mlxbf2_gpio_lock_acquire(gs: *mut mlxbf2_gpio_context) -> c_int {
    static int mlxbf2_gpio_lock_acquire(struct mlxbf2_gpio_context *gs)
    {
    u32 arm_gpio_lock_val;
    mutex_lock(yu_arm_gpio_lock_param.lock);
    gpio_generic_chip_lock(&gs.chip);
    arm_gpio_lock_val = readl(yu_arm_gpio_lock_param.io);
//
// When lock active bit[31] is set, ModeX is write enabled
//
    if (YU_LOCK_ACTIVE_BIT(arm_gpio_lock_val)) {
    gpio_generic_chip_unlock(&gs.chip);
    mutex_unlock(yu_arm_gpio_lock_param.lock);
    return -EINVAL;
    }
    writel(YU_ARM_GPIO_LOCK_ACQUIRE, yu_arm_gpio_lock_param.io);
    return 0;
    }
//
// Release the YU arm_gpio_lock after changing the direction mode.
//
#[no_mangle]
unsafe extern "C" fn mlxbf2_gpio_lock_release(gs: *mut mlxbf2_gpio_context) {
    static void mlxbf2_gpio_lock_release(struct mlxbf2_gpio_context *gs)
    __releases(&gs.chip.lock)
    __releases(yu_arm_gpio_lock_param.lock)
    {
    writel(YU_ARM_GPIO_LOCK_RELEASE, yu_arm_gpio_lock_param.io);
    gpio_generic_chip_unlock(&gs.chip);
    mutex_unlock(yu_arm_gpio_lock_param.lock);
    }
//
// mode0 and mode1 are both locked by the gpio_lock field.
//
// Together, mode0 and mode1 define the gpio Mode dependeing also
// on Reg_DataOut.
//
// {mode1,mode0}:{Reg_DataOut=0,Reg_DataOut=1}->{DataOut=0,DataOut=1}
//
// {0,0}:Reg_DataOut{0,1}->{Z,Z} Input PAD
// {0,1}:Reg_DataOut{0,1}->{0,1} Full drive Output PAD
// {1,0}:Reg_DataOut{0,1}->{0,Z} 0-set PAD to low, 1-float
// {1,1}:Reg_DataOut{0,1}->{Z,1} 0-float, 1-set PAD to high
//
// Set input direction:
// {mode1,mode0} = {0,0}
//
    static int mlxbf2_gpio_direction_input(struct gpio_chip *chip,
    unsigned int offset)
    {
    struct mlxbf2_gpio_context *gs = gpiochip_get_data(chip);
    int ret;
//
// Although the arm_gpio_lock was set in the probe function, check again
// if it is still enabled to be able to write to the ModeX registers.
//
    ret = mlxbf2_gpio_lock_acquire(gs);
    if (ret < 0)
    return ret;
    writel(BIT(offset), gs.gpio_io + YU_GPIO_MODE0_CLEAR);
    writel(BIT(offset), gs.gpio_io + YU_GPIO_MODE1_CLEAR);
    mlxbf2_gpio_lock_release(gs);
    return ret;
    }
//
// Set output direction:
// {mode1,mode0} = {0,1}
//
    static int mlxbf2_gpio_direction_output(struct gpio_chip *chip,
    unsigned int offset,
    int value)
    {
    struct mlxbf2_gpio_context *gs = gpiochip_get_data(chip);
    let mut ret: c_int = 0;
//
// Although the arm_gpio_lock was set in the probe function,
// check again it is still enabled to be able to write to the
// ModeX registers.
//
    ret = mlxbf2_gpio_lock_acquire(gs);
    if (ret < 0)
    return ret;
    writel(BIT(offset), gs.gpio_io + YU_GPIO_MODE1_CLEAR);
    writel(BIT(offset), gs.gpio_io + YU_GPIO_MODE0_SET);
    mlxbf2_gpio_lock_release(gs);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mlxbf2_gpio_irq_enable(irqd: *mut irq_data) {
    static void mlxbf2_gpio_irq_enable(struct irq_data *irqd)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(irqd);
    struct mlxbf2_gpio_context *gs = gpiochip_get_data(gc);
    let mut offset: c_int = irqd_to_hwirq(irqd);
    u32 val;
    gpiochip_enable_irq(gc, irqd_to_hwirq(irqd));
    guard(gpio_generic_lock_irqsave)(&gs.chip);
    val = readl(gs.gpio_io + YU_GPIO_CAUSE_OR_CLRCAUSE);
    val |= BIT(offset);
    writel(val, gs.gpio_io + YU_GPIO_CAUSE_OR_CLRCAUSE);
    val = readl(gs.gpio_io + YU_GPIO_CAUSE_OR_EVTEN0);
    val |= BIT(offset);
    writel(val, gs.gpio_io + YU_GPIO_CAUSE_OR_EVTEN0);
    }
#[no_mangle]
unsafe extern "C" fn mlxbf2_gpio_irq_disable(irqd: *mut irq_data) {
    static void mlxbf2_gpio_irq_disable(struct irq_data *irqd)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(irqd);
    struct mlxbf2_gpio_context *gs = gpiochip_get_data(gc);
    let mut offset: c_int = irqd_to_hwirq(irqd);
    u32 val;
    scoped_guard(gpio_generic_lock_irqsave, &gs.chip) {
    val = readl(gs.gpio_io + YU_GPIO_CAUSE_OR_EVTEN0);
    val &= ~BIT(offset);
    writel(val, gs.gpio_io + YU_GPIO_CAUSE_OR_EVTEN0);
    }
    gpiochip_disable_irq(gc, irqd_to_hwirq(irqd));
    }
#[no_mangle]
unsafe extern "C" fn mlxbf2_gpio_irq_handler(irq: c_int, ptr: *mut c_void) -> irqreturn_t {
    static irqreturn_t mlxbf2_gpio_irq_handler(int irq, void *ptr)
    {
    struct mlxbf2_gpio_context *gs = ptr;
    struct gpio_chip *gc = &gs.chip.gc;
    unsigned long pending;
    u32 level;
    pending = readl(gs.gpio_io + YU_GPIO_CAUSE_OR_CAUSE_EVTEN0);
    writel(pending, gs.gpio_io + YU_GPIO_CAUSE_OR_CLRCAUSE);
    for_each_set_bit(level, &pending, gc.ngpio)
    generic_handle_domain_irq_safe(gc.irq.domain, level);
    return IRQ_RETVAL(pending);
    }
    static int
    mlxbf2_gpio_irq_set_type(struct irq_data *irqd, unsigned int type)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(irqd);
    struct mlxbf2_gpio_context *gs = gpiochip_get_data(gc);
    let mut offset: c_int = irqd_to_hwirq(irqd);
    let mut fall: bool = false;
    let mut rise: bool = false;
    u32 val;
    switch (type & IRQ_TYPE_SENSE_MASK) {
    case IRQ_TYPE_EDGE_BOTH:
    fall = true;
    rise = true;
    break;
    case IRQ_TYPE_EDGE_RISING:
    rise = true;
    break;
    case IRQ_TYPE_EDGE_FALLING:
    fall = true;
    break;
    default:
    return -EINVAL;
    }
    guard(gpio_generic_lock_irqsave)(&gs.chip);
    if (fall) {
    val = readl(gs.gpio_io + YU_GPIO_CAUSE_FALL_EN);
    val |= BIT(offset);
    writel(val, gs.gpio_io + YU_GPIO_CAUSE_FALL_EN);
    }
    if (rise) {
    val = readl(gs.gpio_io + YU_GPIO_CAUSE_RISE_EN);
    val |= BIT(offset);
    writel(val, gs.gpio_io + YU_GPIO_CAUSE_RISE_EN);
    }
    return 0;
    }
    static void mlxbf2_gpio_irq_print_chip(struct irq_data *irqd,
    struct seq_file *p)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(irqd);
    struct mlxbf2_gpio_context *gs = gpiochip_get_data(gc);
    seq_puts(p, dev_name(gs.dev));
    }
    static const struct irq_chip mlxbf2_gpio_irq_chip = {
    .irq_set_type = mlxbf2_gpio_irq_set_type,
    .irq_enable = mlxbf2_gpio_irq_enable,
    .irq_disable = mlxbf2_gpio_irq_disable,
    .irq_print_chip = mlxbf2_gpio_irq_print_chip,
    .flags = IRQCHIP_IMMUTABLE,
    GPIOCHIP_IRQ_RESOURCE_HELPERS,
    };
// BlueField-2 GPIO driver initialization routine.
    static int
    mlxbf2_gpio_probe(struct platform_device *pdev)
    {
    struct gpio_generic_chip_config config;
    struct mlxbf2_gpio_context *gs;
    struct device *dev = &pdev.dev;
    struct gpio_irq_chip *girq;
    struct gpio_chip *gc;
    unsigned int npins;
    const char *name;
    int ret, irq;
    name = dev_name(dev);
    gs = devm_kzalloc(dev, sizeof(*gs), GFP_KERNEL);
    if (!gs)
    return -ENOMEM;
    gs.dev = dev;
// YU GPIO block address
    gs.gpio_io = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(gs.gpio_io))
    return PTR_ERR(gs.gpio_io);
    ret = mlxbf2_gpio_get_lock_res(pdev);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to get yu_arm_gpio_lock resource\n");
    if (device_property_read_u32(dev, "npins", &npins))
    npins = MLXBF2_GPIO_MAX_PINS_PER_BLOCK;
    gc = &gs.chip.gc;
    config = (struct gpio_generic_chip_config) {
    .dev = dev,
    .sz = 4,
    .dat = gs.gpio_io + YU_GPIO_DATAIN,
    .set = gs.gpio_io + YU_GPIO_DATASET,
    .clr = gs.gpio_io + YU_GPIO_DATACLEAR,
    };
    ret = gpio_generic_chip_init(&gs.chip, &config);
    if (ret)
    return dev_err_probe(dev, ret, "failed to initialize the generic GPIO chip\n");
    gc.direction_input = mlxbf2_gpio_direction_input;
    gc.direction_output = mlxbf2_gpio_direction_output;
    gc.ngpio = npins;
    gc.owner = THIS_MODULE;
    irq = platform_get_irq_optional(pdev, 0);
    if (irq >= 0) {
    girq = &gs.chip.gc.irq;
    gpio_irq_chip_set_chip(girq, &mlxbf2_gpio_irq_chip);
    girq.handler = handle_simple_irq;
    girq.default_type = IRQ_TYPE_NONE;
// This will let us handle the parent IRQ in the driver
    girq.num_parents = 0;
    girq.parents = core::ptr::null_mut();
    girq.parent_handler = core::ptr::null_mut();
//
// Directly request the irq here instead of passing
// a flow-handler because the irq is shared.
//
    ret = devm_request_irq(dev, irq, mlxbf2_gpio_irq_handler,
    IRQF_SHARED, name, gs);
    if (ret)
    return dev_err_probe(dev, ret, "failed to request IRQ");
    }
    platform_set_drvdata(pdev, gs);
    ret = devm_gpiochip_add_data(dev, &gs.chip.gc, gs);
    if (ret)
    return dev_err_probe(dev, ret, "Failed adding memory mapped gpiochip\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mlxbf2_gpio_suspend(dev: *mut device) -> c_int {
    static int mlxbf2_gpio_suspend(struct device *dev)
    {
    struct mlxbf2_gpio_context *gs = dev_get_drvdata(dev);
    gs.csave_regs.gpio_mode0 = readl(gs.gpio_io +
    YU_GPIO_MODE0);
    gs.csave_regs.gpio_mode1 = readl(gs.gpio_io +
    YU_GPIO_MODE1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mlxbf2_gpio_resume(dev: *mut device) -> c_int {
    static int mlxbf2_gpio_resume(struct device *dev)
    {
    struct mlxbf2_gpio_context *gs = dev_get_drvdata(dev);
    writel(gs.csave_regs.gpio_mode0, gs.gpio_io +
    YU_GPIO_MODE0);
    writel(gs.csave_regs.gpio_mode1, gs.gpio_io +
    YU_GPIO_MODE1);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(mlxbf2_pm_ops, mlxbf2_gpio_suspend, mlxbf2_gpio_resume);
    static const struct acpi_device_id __maybe_unused mlxbf2_gpio_acpi_match[] = {
    { "MLNXBF22", 0 },
    {},
    };
    MODULE_DEVICE_TABLE(acpi, mlxbf2_gpio_acpi_match);
    static struct platform_driver mlxbf2_gpio_driver = {
    .driver = {
    .name = "mlxbf2_gpio",
    .acpi_match_table = mlxbf2_gpio_acpi_match,
    .pm = pm_sleep_ptr(&mlxbf2_pm_ops),
    },
    .probe    = mlxbf2_gpio_probe,
    };
    module_platform_driver(mlxbf2_gpio_driver);
    MODULE_DESCRIPTION("Mellanox BlueField-2 GPIO Driver");
    MODULE_AUTHOR("Asmaa Mnebhi <asmaa@nvidia.com>");
    MODULE_LICENSE("GPL v2");
