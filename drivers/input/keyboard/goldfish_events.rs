//! Automatically rewritten from C to Rust
//! Source: drivers/input/keyboard/goldfish_events.c
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
// Copyright (C) 2007 Google, Inc.
// Copyright (C) 2012 Intel, Inc.
//

    enum {
    REG_READ        = 0x00,
    REG_SET_PAGE    = 0x00,
    REG_LEN         = 0x04,
    REG_DATA        = 0x08,
    PAGE_NAME       = 0x00000,
    PAGE_EVBITS     = 0x10000,
    PAGE_ABSDATA    = 0x20000 | EV_ABS,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct event_dev {
    pub input: *mut input_dev,
    pub irq: c_int,
    pub addr: *mut void __iomem,
    pub name: [c_char; ],
}

#[no_mangle]
unsafe extern "C" fn events_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t events_interrupt(int irq, void *dev_id)
    {
    struct event_dev *edev = dev_id;
    unsigned int type, code, value;
    type = __raw_readl(edev.addr + REG_READ);
    code = __raw_readl(edev.addr + REG_READ);
    value = __raw_readl(edev.addr + REG_READ);
    input_event(edev.input, type, code, value);
    input_sync(edev.input);
    return IRQ_HANDLED;
    }
    static void events_import_bits(struct event_dev *edev,
    unsigned long bits[], unsigned int type, size_t count)
    {
    void __iomem *addr = edev.addr;
    int i, j;
    size_t size;
    uint8_t val;
    __raw_writel(PAGE_EVBITS | type, addr + REG_SET_PAGE);
    size = __raw_readl(addr + REG_LEN) * 8;
    if (size < count)
    count = size;
    addr += REG_DATA;
    for (i = 0; i < count; i += 8) {
    val = __raw_readb(addr++);
    for (j = 0; j < 8; j++)
    if (val & 1 << j)
    set_bit(i + j, bits);
    }
    }
#[no_mangle]
unsafe extern "C" fn events_import_abs_params(edev: *mut event_dev) {
    static void events_import_abs_params(struct event_dev *edev)
    {
    struct input_dev *input_dev = edev.input;
    void __iomem *addr = edev.addr;
    u32 val[4];
    int count;
    int i, j;
    __raw_writel(PAGE_ABSDATA, addr + REG_SET_PAGE);
    count = __raw_readl(addr + REG_LEN) / sizeof(val);
    if (count > ABS_MAX)
    count = ABS_MAX;
    for (i = 0; i < count; i++) {
    if (!test_bit(i, input_dev.absbit))
    continue;
    for (j = 0; j < ARRAY_SIZE(val); j++) {
    let mut offset: c_int = (i * ARRAY_SIZE(val) + j) * sizeof(u32);
    val[j] = __raw_readl(edev.addr + REG_DATA + offset);
    }
    input_set_abs_params(input_dev, i,
    val[0], val[1], val[2], val[3]);
    }
    }
#[no_mangle]
unsafe extern "C" fn events_probe(pdev: *mut platform_device) -> c_int {
    static int events_probe(struct platform_device *pdev)
    {
    struct input_dev *input_dev;
    struct event_dev *edev;
    struct resource *res;
    unsigned int keymapnamelen;
    void __iomem *addr;
    int irq;
    int i;
    int error;
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return -EINVAL;
    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!res)
    return -EINVAL;
    addr = devm_ioremap(&pdev.dev, res.start, 4096);
    if (!addr)
    return -ENOMEM;
    __raw_writel(PAGE_NAME, addr + REG_SET_PAGE);
    keymapnamelen = __raw_readl(addr + REG_LEN);
    edev = devm_kzalloc(&pdev.dev,
    sizeof(struct event_dev) + keymapnamelen + 1,
    GFP_KERNEL);
    if (!edev)
    return -ENOMEM;
    input_dev = devm_input_allocate_device(&pdev.dev);
    if (!input_dev)
    return -ENOMEM;
    edev.input = input_dev;
    edev.addr = addr;
    edev.irq = irq;
    for (i = 0; i < keymapnamelen; i++)
    edev.name[i] = __raw_readb(edev.addr + REG_DATA + i);
    pr_debug("%s: keymap=%s\n", __func__, edev.name);
    input_dev.name = edev.name;
    input_dev.id.bustype = BUS_HOST;
    events_import_bits(edev, input_dev.evbit, EV_SYN, EV_MAX);
    events_import_bits(edev, input_dev.keybit, EV_KEY, KEY_MAX);
    events_import_bits(edev, input_dev.relbit, EV_REL, REL_MAX);
    events_import_bits(edev, input_dev.absbit, EV_ABS, ABS_MAX);
    events_import_bits(edev, input_dev.mscbit, EV_MSC, MSC_MAX);
    events_import_bits(edev, input_dev.ledbit, EV_LED, LED_MAX);
    events_import_bits(edev, input_dev.sndbit, EV_SND, SND_MAX);
    events_import_bits(edev, input_dev.ffbit, EV_FF, FF_MAX);
    events_import_bits(edev, input_dev.swbit, EV_SW, SW_MAX);
    events_import_abs_params(edev);
    error = devm_request_irq(&pdev.dev, edev.irq, events_interrupt, 0,
    "goldfish-events-keypad", edev);
    if (error)
    return error;
    error = input_register_device(input_dev);
    if (error)
    return error;
    return 0;
    }
    static const struct of_device_id goldfish_events_of_match[] = {
    { .compatible = "google,goldfish-events-keypad", },
    {},
    };
    MODULE_DEVICE_TABLE(of, goldfish_events_of_match);

    static const struct acpi_device_id goldfish_events_acpi_match[] = {
    { "GFSH0002", 0 },
    { },
    };
    MODULE_DEVICE_TABLE(acpi, goldfish_events_acpi_match);

    static struct platform_driver events_driver = {
    .probe	= events_probe,
    .driver	= {
    .name	= "goldfish_events",
    .of_match_table = goldfish_events_of_match,
    .acpi_match_table = ACPI_PTR(goldfish_events_acpi_match),
    },
    };
    module_platform_driver(events_driver);
    MODULE_AUTHOR("Brian Swetland");
    MODULE_DESCRIPTION("Goldfish Event Device");
    MODULE_LICENSE("GPL");
