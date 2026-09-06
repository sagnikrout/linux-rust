//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/da903x.c
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
// Base driver for Dialog Semiconductor DA9030/DA9034
//
// Copyright (C) 2008 Compulab, Ltd.
// Mike Rapoport <mike@compulab.co.il>
//
// Copyright (C) 2006-2008 Marvell International Ltd.
// Eric Miao <eric.miao@marvell.com>
//

pub const DA9030_CHIP_ID: c_uint = 0x00;
pub const DA9030_EVENT_A: c_uint = 0x01;
pub const DA9030_EVENT_B: c_uint = 0x02;
pub const DA9030_EVENT_C: c_uint = 0x03;
pub const DA9030_STATUS: c_uint = 0x04;
pub const DA9030_IRQ_MASK_A: c_uint = 0x05;
pub const DA9030_IRQ_MASK_B: c_uint = 0x06;
pub const DA9030_IRQ_MASK_C: c_uint = 0x07;
pub const DA9030_SYS_CTRL_A: c_uint = 0x08;
pub const DA9030_SYS_CTRL_B: c_uint = 0x09;
pub const DA9030_FAULT_LOG: c_uint = 0x0a;
pub const DA9034_CHIP_ID: c_uint = 0x00;
pub const DA9034_EVENT_A: c_uint = 0x01;
pub const DA9034_EVENT_B: c_uint = 0x02;
pub const DA9034_EVENT_C: c_uint = 0x03;
pub const DA9034_EVENT_D: c_uint = 0x04;
pub const DA9034_STATUS_A: c_uint = 0x05;
pub const DA9034_STATUS_B: c_uint = 0x06;
pub const DA9034_IRQ_MASK_A: c_uint = 0x07;
pub const DA9034_IRQ_MASK_B: c_uint = 0x08;
pub const DA9034_IRQ_MASK_C: c_uint = 0x09;
pub const DA9034_IRQ_MASK_D: c_uint = 0x0a;
pub const DA9034_SYS_CTRL_A: c_uint = 0x0b;
pub const DA9034_SYS_CTRL_B: c_uint = 0x0c;
pub const DA9034_FAULT_LOG: c_uint = 0x0d;
    struct da903x_chip;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct da903x_chip_ops {
    pub ): *mut *mut int (init_chip)(struct da903x_chip,
    pub events): *mut *mut *mut int (unmask_events)(struct da903x_chip , unsigned int,
    pub events): *mut *mut *mut int (mask_events)(struct da903x_chip , unsigned int,
    pub events): *mut *mut *mut int (read_events)(struct da903x_chip , unsigned int,
    pub status): *mut *mut *mut int (read_status)(struct da903x_chip , unsigned int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct da903x_chip {
    pub client: *mut i2c_client,
    pub dev: *mut device,
    pub ops: *const da903x_chip_ops,
    pub type: c_int,
    pub events_mask: u32,
    pub lock: mutex,
    pub irq_work: work_struct,
    pub notifier_list: blocking_notifier_head,
}

    static inline int __da903x_read(struct i2c_client *client,
    int reg, uint8_t *val)
    {
    int ret;
    ret = i2c_smbus_read_byte_data(client, reg);
    if (ret < 0) {
    dev_err(&client.dev, "failed reading at 0x%02x\n", reg);
    return ret;
    }
// val = (uint8_t)ret;
    return 0;
    }
    static inline int __da903x_reads(struct i2c_client *client, int reg,
    int len, uint8_t *val)
    {
    int ret;
    ret = i2c_smbus_read_i2c_block_data(client, reg, len, val);
    if (ret < 0) {
    dev_err(&client.dev, "failed reading from 0x%02x\n", reg);
    return ret;
    }
    return 0;
    }
    static inline int __da903x_write(struct i2c_client *client,
    int reg, uint8_t val)
    {
    int ret;
    ret = i2c_smbus_write_byte_data(client, reg, val);
    if (ret < 0) {
    dev_err(&client.dev, "failed writing 0x%02x to 0x%02x\n",
    val, reg);
    return ret;
    }
    return 0;
    }
    static inline int __da903x_writes(struct i2c_client *client, int reg,
    int len, uint8_t *val)
    {
    int ret;
    ret = i2c_smbus_write_i2c_block_data(client, reg, len, val);
    if (ret < 0) {
    dev_err(&client.dev, "failed writings to 0x%02x\n", reg);
    return ret;
    }
    return 0;
    }
    int da903x_register_notifier(struct device *dev, struct notifier_block *nb,
    unsigned int events)
    {
    struct da903x_chip *chip = dev_get_drvdata(dev);
    chip.ops.unmask_events(chip, events);
    return blocking_notifier_chain_register(&chip.notifier_list, nb);
    }
    EXPORT_SYMBOL_GPL(da903x_register_notifier);
    int da903x_unregister_notifier(struct device *dev, struct notifier_block *nb,
    unsigned int events)
    {
    struct da903x_chip *chip = dev_get_drvdata(dev);
    chip.ops.mask_events(chip, events);
    return blocking_notifier_chain_unregister(&chip.notifier_list, nb);
    }
    EXPORT_SYMBOL_GPL(da903x_unregister_notifier);
#[no_mangle]
pub unsafe extern "C" fn da903x_write(dev: *mut device, reg: c_int, val: u8) -> c_int {
    int da903x_write(struct device *dev, int reg, uint8_t val)
    {
    return __da903x_write(to_i2c_client(dev), reg, val);
    }
    EXPORT_SYMBOL_GPL(da903x_write);
#[no_mangle]
pub unsafe extern "C" fn da903x_writes(dev: *mut device, reg: c_int, len: c_int, val: *mut u8) -> c_int {
    int da903x_writes(struct device *dev, int reg, int len, uint8_t *val)
    {
    return __da903x_writes(to_i2c_client(dev), reg, len, val);
    }
    EXPORT_SYMBOL_GPL(da903x_writes);
#[no_mangle]
pub unsafe extern "C" fn da903x_read(dev: *mut device, reg: c_int, val: *mut u8) -> c_int {
    int da903x_read(struct device *dev, int reg, uint8_t *val)
    {
    return __da903x_read(to_i2c_client(dev), reg, val);
    }
    EXPORT_SYMBOL_GPL(da903x_read);
#[no_mangle]
pub unsafe extern "C" fn da903x_reads(dev: *mut device, reg: c_int, len: c_int, val: *mut u8) -> c_int {
    int da903x_reads(struct device *dev, int reg, int len, uint8_t *val)
    {
    return __da903x_reads(to_i2c_client(dev), reg, len, val);
    }
    EXPORT_SYMBOL_GPL(da903x_reads);
#[no_mangle]
pub unsafe extern "C" fn da903x_set_bits(dev: *mut device, reg: c_int, bit_mask: u8) -> c_int {
    int da903x_set_bits(struct device *dev, int reg, uint8_t bit_mask)
    {
    struct da903x_chip *chip = dev_get_drvdata(dev);
    uint8_t reg_val;
    let mut ret: c_int = 0;
    mutex_lock(&chip.lock);
    ret = __da903x_read(chip.client, reg, &reg_val);
    if (ret)
    goto out;
    if ((reg_val & bit_mask) != bit_mask) {
    reg_val |= bit_mask;
    ret = __da903x_write(chip.client, reg, reg_val);
    }
    out:
    mutex_unlock(&chip.lock);
    return ret;
    }
    EXPORT_SYMBOL_GPL(da903x_set_bits);
#[no_mangle]
pub unsafe extern "C" fn da903x_clr_bits(dev: *mut device, reg: c_int, bit_mask: u8) -> c_int {
    int da903x_clr_bits(struct device *dev, int reg, uint8_t bit_mask)
    {
    struct da903x_chip *chip = dev_get_drvdata(dev);
    uint8_t reg_val;
    let mut ret: c_int = 0;
    mutex_lock(&chip.lock);
    ret = __da903x_read(chip.client, reg, &reg_val);
    if (ret)
    goto out;
    if (reg_val & bit_mask) {
    reg_val &= ~bit_mask;
    ret = __da903x_write(chip.client, reg, reg_val);
    }
    out:
    mutex_unlock(&chip.lock);
    return ret;
    }
    EXPORT_SYMBOL_GPL(da903x_clr_bits);
#[no_mangle]
pub unsafe extern "C" fn da903x_update(dev: *mut device, reg: c_int, val: u8, mask: u8) -> c_int {
    int da903x_update(struct device *dev, int reg, uint8_t val, uint8_t mask)
    {
    struct da903x_chip *chip = dev_get_drvdata(dev);
    uint8_t reg_val;
    let mut ret: c_int = 0;
    mutex_lock(&chip.lock);
    ret = __da903x_read(chip.client, reg, &reg_val);
    if (ret)
    goto out;
    if ((reg_val & mask) != val) {
    reg_val = (reg_val & ~mask) | val;
    ret = __da903x_write(chip.client, reg, reg_val);
    }
    out:
    mutex_unlock(&chip.lock);
    return ret;
    }
    EXPORT_SYMBOL_GPL(da903x_update);
#[no_mangle]
pub unsafe extern "C" fn da903x_query_status(dev: *mut device, sbits: c_uint) -> c_int {
    int da903x_query_status(struct device *dev, unsigned int sbits)
    {
    struct da903x_chip *chip = dev_get_drvdata(dev);
    let mut status: c_uint = 0;
    chip.ops.read_status(chip, &status);
    return ((status & sbits) == sbits);
    }
    EXPORT_SYMBOL(da903x_query_status);
#[no_mangle]
unsafe extern "C" fn da9030_init_chip(chip: *mut da903x_chip) -> c_int {
    static int da9030_init_chip(struct da903x_chip *chip)
    {
    uint8_t chip_id;
    int err;
    err = __da903x_read(chip.client, DA9030_CHIP_ID, &chip_id);
    if (err)
    return err;
    err = __da903x_write(chip.client, DA9030_SYS_CTRL_A, 0xE8);
    if (err)
    return err;
    dev_info(chip.dev, "DA9030 (CHIP ID: 0x%02x) detected\n", chip_id);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn da9030_unmask_events(chip: *mut da903x_chip, events: c_uint) -> c_int {
    static int da9030_unmask_events(struct da903x_chip *chip, unsigned int events)
    {
    uint8_t v[3];
    chip.events_mask &= ~events;
    v[0] = (chip.events_mask & 0xff);
    v[1] = (chip.events_mask >> 8) & 0xff;
    v[2] = (chip.events_mask >> 16) & 0xff;
    return __da903x_writes(chip.client, DA9030_IRQ_MASK_A, 3, v);
    }
#[no_mangle]
unsafe extern "C" fn da9030_mask_events(chip: *mut da903x_chip, events: c_uint) -> c_int {
    static int da9030_mask_events(struct da903x_chip *chip, unsigned int events)
    {
    uint8_t v[3];
    chip.events_mask |= events;
    v[0] = (chip.events_mask & 0xff);
    v[1] = (chip.events_mask >> 8) & 0xff;
    v[2] = (chip.events_mask >> 16) & 0xff;
    return __da903x_writes(chip.client, DA9030_IRQ_MASK_A, 3, v);
    }
#[no_mangle]
unsafe extern "C" fn da9030_read_events(chip: *mut da903x_chip, events: *mut c_uint) -> c_int {
    static int da9030_read_events(struct da903x_chip *chip, unsigned int *events)
    {
    uint8_t v[3] = {0, 0, 0};
    int ret;
    ret = __da903x_reads(chip.client, DA9030_EVENT_A, 3, v);
    if (ret < 0)
    return ret;
// events = (v[2] << 16) | (v[1] << 8) | v[0];
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn da9030_read_status(chip: *mut da903x_chip, status: *mut c_uint) -> c_int {
    static int da9030_read_status(struct da903x_chip *chip, unsigned int *status)
    {
    return __da903x_read(chip.client, DA9030_STATUS, (uint8_t *)status);
    }
#[no_mangle]
unsafe extern "C" fn da9034_init_chip(chip: *mut da903x_chip) -> c_int {
    static int da9034_init_chip(struct da903x_chip *chip)
    {
    uint8_t chip_id;
    int err;
    err = __da903x_read(chip.client, DA9034_CHIP_ID, &chip_id);
    if (err)
    return err;
    err = __da903x_write(chip.client, DA9034_SYS_CTRL_A, 0xE8);
    if (err)
    return err;
// avoid SRAM power off during sleep
    __da903x_write(chip.client, 0x10, 0x07);
    __da903x_write(chip.client, 0x11, 0xff);
    __da903x_write(chip.client, 0x12, 0xff);
// Enable the ONKEY power down functionality
    __da903x_write(chip.client, DA9034_SYS_CTRL_B, 0x20);
    __da903x_write(chip.client, DA9034_SYS_CTRL_A, 0x60);
// workaround to make LEDs work
    __da903x_write(chip.client, 0x90, 0x01);
    __da903x_write(chip.client, 0xB0, 0x08);
// make ADTV1 and SDTV1 effective
    __da903x_write(chip.client, 0x20, 0x00);
    dev_info(chip.dev, "DA9034 (CHIP ID: 0x%02x) detected\n", chip_id);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn da9034_unmask_events(chip: *mut da903x_chip, events: c_uint) -> c_int {
    static int da9034_unmask_events(struct da903x_chip *chip, unsigned int events)
    {
    uint8_t v[4];
    chip.events_mask &= ~events;
    v[0] = (chip.events_mask & 0xff);
    v[1] = (chip.events_mask >> 8) & 0xff;
    v[2] = (chip.events_mask >> 16) & 0xff;
    v[3] = (chip.events_mask >> 24) & 0xff;
    return __da903x_writes(chip.client, DA9034_IRQ_MASK_A, 4, v);
    }
#[no_mangle]
unsafe extern "C" fn da9034_mask_events(chip: *mut da903x_chip, events: c_uint) -> c_int {
    static int da9034_mask_events(struct da903x_chip *chip, unsigned int events)
    {
    uint8_t v[4];
    chip.events_mask |= events;
    v[0] = (chip.events_mask & 0xff);
    v[1] = (chip.events_mask >> 8) & 0xff;
    v[2] = (chip.events_mask >> 16) & 0xff;
    v[3] = (chip.events_mask >> 24) & 0xff;
    return __da903x_writes(chip.client, DA9034_IRQ_MASK_A, 4, v);
    }
#[no_mangle]
unsafe extern "C" fn da9034_read_events(chip: *mut da903x_chip, events: *mut c_uint) -> c_int {
    static int da9034_read_events(struct da903x_chip *chip, unsigned int *events)
    {
    uint8_t v[4] = {0, 0, 0, 0};
    int ret;
    ret = __da903x_reads(chip.client, DA9034_EVENT_A, 4, v);
    if (ret < 0)
    return ret;
// events = (v[3] << 24) | (v[2] << 16) | (v[1] << 8) | v[0];
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn da9034_read_status(chip: *mut da903x_chip, status: *mut c_uint) -> c_int {
    static int da9034_read_status(struct da903x_chip *chip, unsigned int *status)
    {
    uint8_t v[2] = {0, 0};
    let mut ret: c_int = 0;
    ret = __da903x_reads(chip.client, DA9034_STATUS_A, 2, v);
    if (ret)
    return ret;
// status = (v[1] << 8) | v[0];
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn da903x_irq_work(work: *mut work_struct) {
    static void da903x_irq_work(struct work_struct *work)
    {
    struct da903x_chip *chip =
    container_of(work, struct da903x_chip, irq_work);
    let mut events: c_uint = 0;
    while (1) {
    if (chip.ops.read_events(chip, &events))
    break;
    events &= ~chip.events_mask;
    if (events == 0)
    break;
    blocking_notifier_call_chain(
    &chip.notifier_list, events, core::ptr::null_mut());
    }
    enable_irq(chip.client.irq);
    }
#[no_mangle]
unsafe extern "C" fn da903x_irq_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t da903x_irq_handler(int irq, void *data)
    {
    struct da903x_chip *chip = data;
    disable_irq_nosync(irq);
    (void)schedule_work(&chip.irq_work);
    return IRQ_HANDLED;
    }
    static const struct da903x_chip_ops da903x_ops[] = {
    [0] = {
    .init_chip	= da9030_init_chip,
    .unmask_events	= da9030_unmask_events,
    .mask_events	= da9030_mask_events,
    .read_events	= da9030_read_events,
    .read_status	= da9030_read_status,
    },
    [1] = {
    .init_chip	= da9034_init_chip,
    .unmask_events	= da9034_unmask_events,
    .mask_events	= da9034_mask_events,
    .read_events	= da9034_read_events,
    .read_status	= da9034_read_status,
    }
    };
    static const struct i2c_device_id da903x_id_table[] = {
    { "da9030", 0 },
    { "da9034", 1 },
    { },
    };
    MODULE_DEVICE_TABLE(i2c, da903x_id_table);
#[no_mangle]
unsafe extern "C" fn __remove_subdev(dev: *mut device, unused: *mut c_void) -> c_int {
    static int __remove_subdev(struct device *dev, void *unused)
    {
    platform_device_unregister(to_platform_device(dev));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn da903x_remove_subdevs(chip: *mut da903x_chip) -> c_int {
    static int da903x_remove_subdevs(struct da903x_chip *chip)
    {
    return device_for_each_child(chip.dev, core::ptr::null_mut(), __remove_subdev);
    }
    static int da903x_add_subdevs(struct da903x_chip *chip,
    struct da903x_platform_data *pdata)
    {
    struct da903x_subdev_info *subdev;
    struct platform_device *pdev;
    int i, ret = 0;
    for (i = 0; i < pdata.num_subdevs; i++) {
    subdev = &pdata.subdevs[i];
    pdev = platform_device_alloc(subdev.name, subdev.id);
    if (!pdev) {
    ret = -ENOMEM;
    goto failed;
    }
    pdev.dev.parent = chip.dev;
    pdev.dev.platform_data = subdev.platform_data;
    ret = platform_device_add(pdev);
    if (ret) {
    platform_device_put(pdev);
    goto failed;
    }
    }
    return 0;
    failed:
    da903x_remove_subdevs(chip);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn da903x_probe(client: *mut i2c_client) -> c_int {
    static int da903x_probe(struct i2c_client *client)
    {
    const struct i2c_device_id *id = i2c_client_get_device_id(client);
    struct da903x_platform_data *pdata = dev_get_platdata(&client.dev);
    struct da903x_chip *chip;
    unsigned int tmp;
    int ret;
    chip = devm_kzalloc(&client.dev, sizeof(struct da903x_chip),
    GFP_KERNEL);
    if (chip == core::ptr::null_mut())
    return -ENOMEM;
    chip.client = client;
    chip.dev = &client.dev;
    chip.ops = &da903x_ops[id.driver_data];
    mutex_init(&chip.lock);
    INIT_WORK(&chip.irq_work, da903x_irq_work);
    BLOCKING_INIT_NOTIFIER_HEAD(&chip.notifier_list);
    i2c_set_clientdata(client, chip);
    ret = chip.ops.init_chip(chip);
    if (ret)
    return ret;
// mask and clear all IRQs
    chip.events_mask = 0xffffffff;
    chip.ops.mask_events(chip, chip.events_mask);
    chip.ops.read_events(chip, &tmp);
    ret = devm_request_irq(&client.dev, client.irq, da903x_irq_handler,
    IRQF_TRIGGER_FALLING,
    "da903x", chip);
    if (ret) {
    dev_err(&client.dev, "failed to request irq %d\n",
    client.irq);
    return ret;
    }
    return da903x_add_subdevs(chip, pdata);
    }
#[no_mangle]
unsafe extern "C" fn da903x_remove(client: *mut i2c_client) {
    static void da903x_remove(struct i2c_client *client)
    {
    struct da903x_chip *chip = i2c_get_clientdata(client);
    da903x_remove_subdevs(chip);
    }
    static struct i2c_driver da903x_driver = {
    .driver	= {
    .name	= "da903x",
    },
    .probe		= da903x_probe,
    .remove		= da903x_remove,
    .id_table	= da903x_id_table,
    };
#[no_mangle]
unsafe extern "C" fn da903x_init() -> int __init {
    static int __init da903x_init(void)
    {
    return i2c_add_driver(&da903x_driver);
    }
    subsys_initcall(da903x_init);
#[no_mangle]
unsafe extern "C" fn da903x_exit() -> void __exit {
    static void __exit da903x_exit(void)
    {
    i2c_del_driver(&da903x_driver);
    }
    module_exit(da903x_exit);
    MODULE_DESCRIPTION("PMIC Driver for Dialog Semiconductor DA9034");
    MODULE_AUTHOR("Eric Miao <eric.miao@marvell.com>");
    MODULE_AUTHOR("Mike Rapoport <mike@compulab.co.il>");
