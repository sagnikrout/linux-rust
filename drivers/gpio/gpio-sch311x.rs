//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-sch311x.c
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
// GPIO driver for the SMSC SCH311x Super-I/O chips
//
// Copyright (C) 2013 Bruno Randolf <br1@einfach.org>
//
// SuperIO functions and chip detection:
// (c) Copyright 2008 Wim Van Sebroeck <wim@iguana.be>.
//

pub const SIO_CONFIG_KEY_ENTER: c_uint = 0x55;
pub const SIO_CONFIG_KEY_EXIT: c_uint = 0xaa;
pub const GP1: c_uint = 0x4b;
    static int sch311x_ioports[] = { 0x2e, 0x4e, 0x162e, 0x164e };
    static struct platform_device *sch311x_gpio_pdev;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sch311x_pdev_data {
    pub /: *mut *mut unsigned short runtime_reg; / runtime register base address,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sch311x_gpio_block {
    pub chip: gpio_chip,
    pub /: *mut *mut unsigned short data_reg; / from definition below,
    pub /: *mut *mut *mut unsigned short config_regs; / pointer to definition below,
    pub /: *mut *mut unsigned short runtime_reg; / runtime register,
    pub /: *mut *mut spinlock_t lock; / lock for this GPIO block,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sch311x_gpio_priv {
    pub blocks: [sch311x_gpio_block; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sch311x_gpio_block_def {
    pub data_reg: c_ushort,
    pub config_regs: [c_ushort; 8],
    pub base: c_ushort,
}

// Note: some GPIOs are not available, these are marked with 0x00
    static struct sch311x_gpio_block_def sch311x_gpio_blocks[] = {
    {
    .data_reg = 0x4b,	/* GP1 */
    .config_regs = {0x23, 0x24, 0x25, 0x26, 0x27, 0x29, 0x2a, 0x2b},
    .base = 10,
    },
    {
    .data_reg = 0x4c,	/* GP2 */
    .config_regs = {0x00, 0x2c, 0x2d, 0x00, 0x00, 0x00, 0x00, 0x32},
    .base = 20,
    },
    {
    .data_reg = 0x4d,	/* GP3 */
    .config_regs = {0x33, 0x34, 0x35, 0x36, 0x37, 0x00, 0x39, 0x3a},
    .base = 30,
    },
    {
    .data_reg = 0x4e,	/* GP4 */
    .config_regs = {0x3b, 0x00, 0x3d, 0x00, 0x6e, 0x6f, 0x72, 0x73},
    .base = 40,
    },
    {
    .data_reg = 0x4f,	/* GP5 */
    .config_regs = {0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46},
    .base = 50,
    },
    {
    .data_reg = 0x50,	/* GP6 */
    .config_regs = {0x47, 0x48, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59},
    .base = 60,
    },
    };
//
// Super-IO functions
//
#[no_mangle]
pub unsafe extern "C" fn sch311x_sio_enter(sio_config_port: c_int) -> c_int {
    static inline int sch311x_sio_enter(int sio_config_port)
    {
// Don't step on other drivers' I/O space by accident.
    if (!request_muxed_region(sio_config_port, 2, DRV_NAME)) {
    pr_err(DRV_NAME "I/O address 0x%04x already in use\n",
    sio_config_port);
    return -EBUSY;
    }
    outb(SIO_CONFIG_KEY_ENTER, sio_config_port);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn sch311x_sio_exit(sio_config_port: c_int) {
    static inline void sch311x_sio_exit(int sio_config_port)
    {
    outb(SIO_CONFIG_KEY_EXIT, sio_config_port);
    release_region(sio_config_port, 2);
    }
#[no_mangle]
pub unsafe extern "C" fn sch311x_sio_inb(sio_config_port: c_int, reg: c_int) -> c_int {
    static inline int sch311x_sio_inb(int sio_config_port, int reg)
    {
    outb(reg, sio_config_port);
    return inb(sio_config_port + 1);
    }
#[no_mangle]
pub unsafe extern "C" fn sch311x_sio_outb(sio_config_port: c_int, reg: c_int, val: c_int) {
    static inline void sch311x_sio_outb(int sio_config_port, int reg, int val)
    {
    outb(reg, sio_config_port);
    outb(val, sio_config_port + 1);
    }
//
// GPIO functions
//
#[no_mangle]
unsafe extern "C" fn sch311x_gpio_request(chip: *mut gpio_chip, offset: unsigned) -> c_int {
    static int sch311x_gpio_request(struct gpio_chip *chip, unsigned offset)
    {
    struct sch311x_gpio_block *block = gpiochip_get_data(chip);
    if (block.config_regs[offset] == 0) /* GPIO is not available */
    return -ENODEV;
    if (!request_region(block.runtime_reg + block.config_regs[offset],
    1, DRV_NAME)) {
    dev_err(chip.parent, "Failed to request region 0x%04x.\n",
    block.runtime_reg + block.config_regs[offset]);
    return -EBUSY;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sch311x_gpio_free(chip: *mut gpio_chip, offset: unsigned) {
    static void sch311x_gpio_free(struct gpio_chip *chip, unsigned offset)
    {
    struct sch311x_gpio_block *block = gpiochip_get_data(chip);
    if (block.config_regs[offset] == 0) /* GPIO is not available */
    return;
    release_region(block.runtime_reg + block.config_regs[offset], 1);
    }
#[no_mangle]
unsafe extern "C" fn sch311x_gpio_get(chip: *mut gpio_chip, offset: unsigned) -> c_int {
    static int sch311x_gpio_get(struct gpio_chip *chip, unsigned offset)
    {
    struct sch311x_gpio_block *block = gpiochip_get_data(chip);
    u8 data;
    spin_lock(&block.lock);
    data = inb(block.runtime_reg + block.data_reg);
    spin_unlock(&block.lock);
    return !!(data & BIT(offset));
    }
    static void __sch311x_gpio_set(struct sch311x_gpio_block *block,
    unsigned offset, int value)
    {
    let mut data: u8 = inb(block.runtime_reg + block.data_reg);
    if (value)
    data |= BIT(offset);
    else
    data &= ~BIT(offset);
    outb(data, block.runtime_reg + block.data_reg);
    }
    static int sch311x_gpio_set(struct gpio_chip *chip, unsigned int offset,
    int value)
    {
    struct sch311x_gpio_block *block = gpiochip_get_data(chip);
    spin_lock(&block.lock);
    __sch311x_gpio_set(block, offset, value);
    spin_unlock(&block.lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sch311x_gpio_direction_in(chip: *mut gpio_chip, offset: unsigned) -> c_int {
    static int sch311x_gpio_direction_in(struct gpio_chip *chip, unsigned offset)
    {
    struct sch311x_gpio_block *block = gpiochip_get_data(chip);
    u8 data;
    spin_lock(&block.lock);
    data = inb(block.runtime_reg + block.config_regs[offset]);
    data |= SCH311X_GPIO_CONF_DIR;
    outb(data, block.runtime_reg + block.config_regs[offset]);
    spin_unlock(&block.lock);
    return 0;
    }
    static int sch311x_gpio_direction_out(struct gpio_chip *chip, unsigned offset,
    int value)
    {
    struct sch311x_gpio_block *block = gpiochip_get_data(chip);
    u8 data;
    spin_lock(&block.lock);
    data = inb(block.runtime_reg + block.config_regs[offset]);
    data &= ~SCH311X_GPIO_CONF_DIR;
    outb(data, block.runtime_reg + block.config_regs[offset]);
    __sch311x_gpio_set(block, offset, value);
    spin_unlock(&block.lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sch311x_gpio_get_direction(chip: *mut gpio_chip, offset: unsigned) -> c_int {
    static int sch311x_gpio_get_direction(struct gpio_chip *chip, unsigned offset)
    {
    struct sch311x_gpio_block *block = gpiochip_get_data(chip);
    u8 data;
    spin_lock(&block.lock);
    data = inb(block.runtime_reg + block.config_regs[offset]);
    spin_unlock(&block.lock);
    if (data & SCH311X_GPIO_CONF_DIR)
    return GPIO_LINE_DIRECTION_IN;
    return GPIO_LINE_DIRECTION_OUT;
    }
    static int sch311x_gpio_set_config(struct gpio_chip *chip, unsigned offset,
    unsigned long config)
    {
    struct sch311x_gpio_block *block = gpiochip_get_data(chip);
    let mut param: enum pin_config_param = pinconf_to_config_param(config);
    u8 data;
    switch (param) {
    case PIN_CONFIG_DRIVE_OPEN_DRAIN:
    spin_lock(&block.lock);
    data = inb(block.runtime_reg + block.config_regs[offset]);
    data |= SCH311X_GPIO_CONF_OPEN_DRAIN;
    outb(data, block.runtime_reg + block.config_regs[offset]);
    spin_unlock(&block.lock);
    return 0;
    case PIN_CONFIG_DRIVE_PUSH_PULL:
    spin_lock(&block.lock);
    data = inb(block.runtime_reg + block.config_regs[offset]);
    data &= ~SCH311X_GPIO_CONF_OPEN_DRAIN;
    outb(data, block.runtime_reg + block.config_regs[offset]);
    spin_unlock(&block.lock);
    return 0;
    default:
    break;
    }
    return -ENOTSUPP;
    }
#[no_mangle]
unsafe extern "C" fn sch311x_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int sch311x_gpio_probe(struct platform_device *pdev)
    {
    struct sch311x_pdev_data *pdata = dev_get_platdata(&pdev.dev);
    struct sch311x_gpio_priv *priv;
    struct sch311x_gpio_block *block;
    int err, i;
// we can register all GPIO data registers at once
    if (!devm_request_region(&pdev.dev, pdata.runtime_reg + GP1, 6,
    DRV_NAME)) {
    dev_err(&pdev.dev, "Failed to request region 0x%04x-0x%04x.\n",
    pdata.runtime_reg + GP1, pdata.runtime_reg + GP1 + 5);
    return -EBUSY;
    }
    priv = devm_kzalloc(&pdev.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    for (i = 0; i < ARRAY_SIZE(priv.blocks); i++) {
    block = &priv.blocks[i];
    spin_lock_init(&block.lock);
    block.chip.label = DRV_NAME;
    block.chip.owner = THIS_MODULE;
    block.chip.request = sch311x_gpio_request;
    block.chip.free = sch311x_gpio_free;
    block.chip.direction_input = sch311x_gpio_direction_in;
    block.chip.direction_output = sch311x_gpio_direction_out;
    block.chip.get_direction = sch311x_gpio_get_direction;
    block.chip.set_config = sch311x_gpio_set_config;
    block.chip.get = sch311x_gpio_get;
    block.chip.set = sch311x_gpio_set;
    block.chip.ngpio = 8;
    block.chip.parent = &pdev.dev;
    block.chip.base = sch311x_gpio_blocks[i].base;
    block.config_regs = sch311x_gpio_blocks[i].config_regs;
    block.data_reg = sch311x_gpio_blocks[i].data_reg;
    block.runtime_reg = pdata.runtime_reg;
    err = devm_gpiochip_add_data(&pdev.dev, &block.chip, block);
    if (err < 0) {
    dev_err(&pdev.dev,
    "Could not register gpiochip, %d\n", err);
    return err;
    }
    dev_info(&pdev.dev,
    "SMSC SCH311x GPIO block %d registered.\n", i);
    }
    return 0;
    }
    static struct platform_driver sch311x_gpio_driver = {
    .driver.name	= DRV_NAME,
    .probe		= sch311x_gpio_probe,
    };
//
// Init & exit routines
//
#[no_mangle]
unsafe extern "C" fn sch311x_detect(sio_config_port: c_int, addr: *mut c_ushort) -> int __init {
    static int __init sch311x_detect(int sio_config_port, unsigned short *addr)
    {
    let mut err: c_int = 0, reg;
    unsigned short base_addr;
    u8 dev_id;
    err = sch311x_sio_enter(sio_config_port);
    if (err)
    return err;
// Check device ID.
    reg = sch311x_sio_inb(sio_config_port, 0x20);
    switch (reg) {
    case 0x7c: /* SCH3112 */
    dev_id = 2;
    break;
    case 0x7d: /* SCH3114 */
    dev_id = 4;
    break;
    case 0x7f: /* SCH3116 */
    dev_id = 6;
    break;
    default:
    err = -ENODEV;
    goto exit;
    }
// Select logical device A (runtime registers)
    sch311x_sio_outb(sio_config_port, 0x07, 0x0a);
// Check if Logical Device Register is currently active
    if ((sch311x_sio_inb(sio_config_port, 0x30) & 0x01) == 0)
    pr_info("Seems that LDN 0x0a is not active...\n");
// Get the base address of the runtime registers
    base_addr = (sch311x_sio_inb(sio_config_port, 0x60) << 8) |
    sch311x_sio_inb(sio_config_port, 0x61);
    if (!base_addr) {
    pr_err("Base address not set\n");
    err = -ENODEV;
    goto exit;
    }
// addr = base_addr;
    pr_info("Found an SMSC SCH311%d chip at 0x%04x\n", dev_id, base_addr);
    exit:
    sch311x_sio_exit(sio_config_port);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn sch311x_gpio_pdev_add(addr: c_ushort) -> int __init {
    static int __init sch311x_gpio_pdev_add(const unsigned short addr)
    {
    struct sch311x_pdev_data pdata;
    int err;
    pdata.runtime_reg = addr;
    sch311x_gpio_pdev = platform_device_alloc(DRV_NAME, -1);
    if (!sch311x_gpio_pdev)
    return -ENOMEM;
    err = platform_device_add_data(sch311x_gpio_pdev,
    &pdata, sizeof(pdata));
    if (err) {
    pr_err(DRV_NAME "Platform data allocation failed\n");
    goto err;
    }
    err = platform_device_add(sch311x_gpio_pdev);
    if (err) {
    pr_err(DRV_NAME "Device addition failed\n");
    goto err;
    }
    return 0;
    err:
    platform_device_put(sch311x_gpio_pdev);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn sch311x_gpio_init() -> int __init {
    static int __init sch311x_gpio_init(void)
    {
    int err, i;
    let mut addr: c_ushort = 0;
    for (i = 0; i < ARRAY_SIZE(sch311x_ioports); i++)
    if (sch311x_detect(sch311x_ioports[i], &addr) == 0)
    break;
    if (!addr)
    return -ENODEV;
    err = platform_driver_register(&sch311x_gpio_driver);
    if (err)
    return err;
    err = sch311x_gpio_pdev_add(addr);
    if (err)
    goto unreg_platform_driver;
    return 0;
    unreg_platform_driver:
    platform_driver_unregister(&sch311x_gpio_driver);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn sch311x_gpio_exit() -> void __exit {
    static void __exit sch311x_gpio_exit(void)
    {
    platform_device_unregister(sch311x_gpio_pdev);
    platform_driver_unregister(&sch311x_gpio_driver);
    }
    module_init(sch311x_gpio_init);
    module_exit(sch311x_gpio_exit);
    MODULE_AUTHOR("Bruno Randolf <br1@einfach.org>");
    MODULE_DESCRIPTION("SMSC SCH311x GPIO Driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:gpio-sch311x");
