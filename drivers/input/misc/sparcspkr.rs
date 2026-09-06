//! Automatically rewritten from C to Rust
//! Source: drivers/input/misc/sparcspkr.c
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
// Driver for PC-speaker like devices found on various Sparc systems.
//
// Copyright (c) 2002 Vojtech Pavlik
// Copyright (c) 2002, 2006, 2008 David S. Miller (davem@davemloft.net)
//

    MODULE_AUTHOR("David S. Miller <davem@davemloft.net>");
    MODULE_DESCRIPTION("Sparc Speaker beeper driver");
    MODULE_LICENSE("GPL");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct grover_beep_info {
    pub freq_regs: *mut void __iomem,
    pub enable_reg: *mut void __iomem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bbc_beep_info {
    pub clock_freq: u32,
    pub regs: *mut void __iomem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sparcspkr_state {
    pub name: *const c_char,
    pub value): *mut *mut *mut int (event)(struct input_dev dev, unsigned int type, unsigned int code, int,
    pub lock: spinlock_t,
    pub input_dev: *mut input_dev,
    union {
    pub grover: grover_beep_info,
    pub bbc: bbc_beep_info,
    pub u: },
}

#[no_mangle]
unsafe extern "C" fn bbc_count_to_reg(info: *mut bbc_beep_info, count: c_uint) -> u32 {
    static u32 bbc_count_to_reg(struct bbc_beep_info *info, unsigned int count)
    {
    u32 val, clock_freq = info.clock_freq;
    int i;
    if (!count)
    return 0;
    if (count <= clock_freq >> 20)
    return 1 << 18;
    if (count >= clock_freq >> 12)
    return 1 << 10;
    val = 1 << 18;
    for (i = 19; i >= 11; i--) {
    val >>= 1;
    if (count <= clock_freq >> i)
    break;
    }
    return val;
    }
#[no_mangle]
unsafe extern "C" fn bbc_spkr_event(dev: *mut input_dev, type: c_uint, code: c_uint, value: c_int) -> c_int {
    static int bbc_spkr_event(struct input_dev *dev, unsigned int type, unsigned int code, int value)
    {
    struct sparcspkr_state *state = dev_get_drvdata(dev.dev.parent);
    struct bbc_beep_info *info = &state.u.bbc;
    let mut count: c_uint = 0;
    if (type != EV_SND)
    return -1;
    switch (code) {
    case SND_BELL:
    if (value)
    value = 1000;
    break;
    case SND_TONE:
    break;
    default:
    return -1;
    }
    if (value > 20 && value < 32767)
    count = 1193182 / value;
    count = bbc_count_to_reg(info, count);
    guard(spinlock_irqsave)(&state.lock);
    if (count) {
    sbus_writeb(0x01,                 info.regs + 0);
    sbus_writeb(0x00,                 info.regs + 2);
    sbus_writeb((count >> 16) & 0xff, info.regs + 3);
    sbus_writeb((count >>  8) & 0xff, info.regs + 4);
    sbus_writeb(0x00,                 info.regs + 5);
    } else {
    sbus_writeb(0x00,                 info.regs + 0);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn grover_spkr_event(dev: *mut input_dev, type: c_uint, code: c_uint, value: c_int) -> c_int {
    static int grover_spkr_event(struct input_dev *dev, unsigned int type, unsigned int code, int value)
    {
    struct sparcspkr_state *state = dev_get_drvdata(dev.dev.parent);
    struct grover_beep_info *info = &state.u.grover;
    let mut count: c_uint = 0;
    if (type != EV_SND)
    return -1;
    switch (code) {
    case SND_BELL:
    if (value)
    value = 1000;
    break;
    case SND_TONE:
    break;
    default:
    return -1;
    }
    if (value > 20 && value < 32767)
    count = 1193182 / value;
    guard(spinlock_irqsave)(&state.lock);
    if (count) {
// enable counter 2
    sbus_writeb(sbus_readb(info.enable_reg) | 3, info.enable_reg);
// set command for counter 2, 2 byte write
    sbus_writeb(0xB6, info.freq_regs + 1);
// select desired HZ
    sbus_writeb(count & 0xff, info.freq_regs + 0);
    sbus_writeb((count >> 8) & 0xff, info.freq_regs + 0);
    } else {
// disable counter 2
    sbus_writeb(sbus_readb(info.enable_reg) & 0xFC, info.enable_reg);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sparcspkr_probe(dev: *mut device) -> c_int {
    static int sparcspkr_probe(struct device *dev)
    {
    struct sparcspkr_state *state = dev_get_drvdata(dev);
    struct input_dev *input_dev;
    int error;
    input_dev = input_allocate_device();
    if (!input_dev)
    return -ENOMEM;
    input_dev.name = state.name;
    input_dev.phys = "sparc/input0";
    input_dev.id.bustype = BUS_ISA;
    input_dev.id.vendor = 0x001f;
    input_dev.id.product = 0x0001;
    input_dev.id.version = 0x0100;
    input_dev.dev.parent = dev;
    input_dev.evbit[0] = BIT_MASK(EV_SND);
    input_dev.sndbit[0] = BIT_MASK(SND_BELL) | BIT_MASK(SND_TONE);
    input_dev.event = state.event;
    error = input_register_device(input_dev);
    if (error) {
    input_free_device(input_dev);
    return error;
    }
    state.input_dev = input_dev;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sparcspkr_shutdown(dev: *mut platform_device) {
    static void sparcspkr_shutdown(struct platform_device *dev)
    {
    struct sparcspkr_state *state = platform_get_drvdata(dev);
    struct input_dev *input_dev = state.input_dev;
// turn off the speaker
    state.event(input_dev, EV_SND, SND_BELL, 0);
    }
#[no_mangle]
unsafe extern "C" fn bbc_beep_probe(op: *mut platform_device) -> c_int {
    static int bbc_beep_probe(struct platform_device *op)
    {
    struct sparcspkr_state *state;
    struct bbc_beep_info *info;
    int err;
    state = devm_kzalloc(&op.dev, sizeof(*state), GFP_KERNEL);
    if (!state)
    return -ENOMEM;
    state.name = "Sparc BBC Speaker";
    state.event = bbc_spkr_event;
    spin_lock_init(&state.lock);
    struct device_node *dp __free(device_node) = of_find_node_by_path("/");
    if (!dp)
    return -ENODEV;
    info = &state.u.bbc;
    info.clock_freq = of_getintprop_default(dp, "clock-frequency", 0);
    if (!info.clock_freq)
    return -ENODEV;
    info.regs = of_ioremap(&op.resource[0], 0, 6, "bbc beep");
    if (!info.regs)
    return -ENODEV;
    platform_set_drvdata(op, state);
    err = sparcspkr_probe(&op.dev);
    if (err) {
    of_iounmap(&op.resource[0], info.regs, 6);
    return err;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bbc_remove(op: *mut platform_device) {
    static void bbc_remove(struct platform_device *op)
    {
    struct sparcspkr_state *state = platform_get_drvdata(op);
    struct input_dev *input_dev = state.input_dev;
    struct bbc_beep_info *info = &state.u.bbc;
// turn off the speaker
    state.event(input_dev, EV_SND, SND_BELL, 0);
    input_unregister_device(input_dev);
    of_iounmap(&op.resource[0], info.regs, 6);
    }
    static const struct of_device_id bbc_beep_match[] = {
    {
    .name = "beep",
    .compatible = "SUNW,bbc-beep",
    },
    {},
    };
    MODULE_DEVICE_TABLE(of, bbc_beep_match);
    static struct platform_driver bbc_beep_driver = {
    .driver = {
    .name = "bbcbeep",
    .of_match_table = bbc_beep_match,
    },
    .probe		= bbc_beep_probe,
    .remove		= bbc_remove,
    .shutdown	= sparcspkr_shutdown,
    };
#[no_mangle]
unsafe extern "C" fn grover_beep_probe(op: *mut platform_device) -> c_int {
    static int grover_beep_probe(struct platform_device *op)
    {
    struct sparcspkr_state *state;
    struct grover_beep_info *info;
    let mut err: c_int = -ENOMEM;
    state = devm_kzalloc(&op.dev, sizeof(*state), GFP_KERNEL);
    if (!state)
    return err;
    state.name = "Sparc Grover Speaker";
    state.event = grover_spkr_event;
    spin_lock_init(&state.lock);
    info = &state.u.grover;
    info.freq_regs = of_ioremap(&op.resource[2], 0, 2, "grover beep freq");
    if (!info.freq_regs)
    return err;
    info.enable_reg = of_ioremap(&op.resource[3], 0, 1, "grover beep enable");
    if (!info.enable_reg)
    goto out_unmap_freq_regs;
    platform_set_drvdata(op, state);
    err = sparcspkr_probe(&op.dev);
    if (err)
    goto out_clear_drvdata;
    return 0;
    out_clear_drvdata:
    of_iounmap(&op.resource[3], info.enable_reg, 1);
    out_unmap_freq_regs:
    of_iounmap(&op.resource[2], info.freq_regs, 2);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn grover_remove(op: *mut platform_device) {
    static void grover_remove(struct platform_device *op)
    {
    struct sparcspkr_state *state = platform_get_drvdata(op);
    struct grover_beep_info *info = &state.u.grover;
    struct input_dev *input_dev = state.input_dev;
// turn off the speaker
    state.event(input_dev, EV_SND, SND_BELL, 0);
    input_unregister_device(input_dev);
    of_iounmap(&op.resource[3], info.enable_reg, 1);
    of_iounmap(&op.resource[2], info.freq_regs, 2);
    }
    static const struct of_device_id grover_beep_match[] = {
    {
    .name = "beep",
    .compatible = "SUNW,smbus-beep",
    },
    {},
    };
    MODULE_DEVICE_TABLE(of, grover_beep_match);
    static struct platform_driver grover_beep_driver = {
    .driver = {
    .name = "groverbeep",
    .of_match_table = grover_beep_match,
    },
    .probe		= grover_beep_probe,
    .remove		= grover_remove,
    .shutdown	= sparcspkr_shutdown,
    };
    static struct platform_driver * const drivers[] = {
    &bbc_beep_driver,
    &grover_beep_driver,
    };
#[no_mangle]
unsafe extern "C" fn sparcspkr_init() -> int __init {
    static int __init sparcspkr_init(void)
    {
    return platform_register_drivers(drivers, ARRAY_SIZE(drivers));
    }
#[no_mangle]
unsafe extern "C" fn sparcspkr_exit() -> void __exit {
    static void __exit sparcspkr_exit(void)
    {
    platform_unregister_drivers(drivers, ARRAY_SIZE(drivers));
    }
    module_init(sparcspkr_init);
    module_exit(sparcspkr_exit);
