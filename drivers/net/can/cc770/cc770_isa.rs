//! Automatically rewritten from C to Rust
//! Source: drivers/net/can/cc770/cc770_isa.c
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
// Driver for CC770 and AN82527 CAN controllers on the legacy ISA bus
//
// Copyright (C) 2009, 2011 Wolfgang Grandegger <wg@grandegger.com>
//
// Bosch CC770 and Intel AN82527 CAN controllers on the ISA or PC-104 bus.
// The I/O port or memory address and the IRQ number must be specified via
// module parameters:
//
// insmod cc770_isa.ko port=0x310,0x380 irq=7,11
//
// for ISA devices using I/O ports or:
//
// insmod cc770_isa.ko mem=0xd1000,0xd1000 irq=7,11
//
// for memory mapped ISA devices.
//
// Indirect access via address and data port is supported as well:
//
// insmod cc770_isa.ko port=0x310,0x380 indirect=1 irq=7,11
//
// Furthermore, the following mode parameter can be defined:
//
// clk: External oscillator clock frequency (default=16000000 [16 MHz])
// cir: CPU interface register (default=0x40 [DSC])
// bcr: Bus configuration register (default=0x40 [CBY])
// cor: Clockout register (default=0x00)
//
// Note: for clk, cir, bcr and cor, the first argument re-defines the
// default for all other devices, e.g.:
//
// insmod cc770_isa.ko mem=0xd1000,0xd1000 irq=7,11 clk=24000000
//
// is equivalent to
//
// insmod cc770_isa.ko mem=0xd1000,0xd1000 irq=7,11 clk=24000000,24000000
//

pub const MAXDEV: c_int = 8;
    MODULE_AUTHOR("Wolfgang Grandegger <wg@grandegger.com>");
    MODULE_DESCRIPTION("Socket-CAN driver for CC770 on the ISA bus");
    MODULE_LICENSE("GPL v2");

pub const COR_DEFAULT: c_uint = 0x00;

    static unsigned long port[MAXDEV];
    static unsigned long mem[MAXDEV];
    static int irq[MAXDEV];
    static int clk[MAXDEV];
    static u8 cir[MAXDEV] = {[0 ... (MAXDEV - 1)] = 0xff};
    static u8 cor[MAXDEV] = {[0 ... (MAXDEV - 1)] = 0xff};
    static u8 bcr[MAXDEV] = {[0 ... (MAXDEV - 1)] = 0xff};
    static int indirect[MAXDEV] = {[0 ... (MAXDEV - 1)] = -1};
    module_param_hw_array(port, ulong, ioport, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(port, "I/O port number");
    module_param_hw_array(mem, ulong, iomem, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(mem, "I/O memory address");
    module_param_hw_array(indirect, int, ioport, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(indirect, "Indirect access via address and data port");
    module_param_hw_array(irq, int, irq, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(irq, "IRQ number");
    module_param_array(clk, int, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(clk, "External oscillator clock frequency "
    "(default=16000000 [16 MHz])");
    module_param_array(cir, byte, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(cir, "CPU interface register (default=0x40 [DSC])");
    module_param_array(cor, byte, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(cor, "Clockout register (default=0x00)");
    module_param_array(bcr, byte, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(bcr, "Bus configuration register (default=0x40 [CBY])");
pub const CC770_IOSIZE: c_uint = 0x20;
pub const CC770_IOSIZE_INDIRECT: c_uint = 0x02;
// Spinlock for cc770_isa_port_write_reg_indirect
// and cc770_isa_port_read_reg_indirect
//
    static DEFINE_SPINLOCK(cc770_isa_port_lock);
    static struct platform_device *cc770_isa_devs[MAXDEV];
#[no_mangle]
unsafe extern "C" fn cc770_isa_mem_read_reg(priv: *const cc770_priv, reg: c_int) -> u8 {
    static u8 cc770_isa_mem_read_reg(const struct cc770_priv *priv, int reg)
    {
    return readb(priv.reg_base + reg);
    }
    static void cc770_isa_mem_write_reg(const struct cc770_priv *priv,
    int reg, u8 val)
    {
    writeb(val, priv.reg_base + reg);
    }
#[no_mangle]
unsafe extern "C" fn cc770_isa_port_read_reg(priv: *const cc770_priv, reg: c_int) -> u8 {
    static u8 cc770_isa_port_read_reg(const struct cc770_priv *priv, int reg)
    {
    return inb((unsigned long)priv.reg_base + reg);
    }
    static void cc770_isa_port_write_reg(const struct cc770_priv *priv,
    int reg, u8 val)
    {
    outb(val, (unsigned long)priv.reg_base + reg);
    }
    static u8 cc770_isa_port_read_reg_indirect(const struct cc770_priv *priv,
    int reg)
    {
    let mut base: c_ulong = (unsigned long)priv.reg_base;
    unsigned long flags;
    u8 val;
    spin_lock_irqsave(&cc770_isa_port_lock, flags);
    outb(reg, base);
    val = inb(base + 1);
    spin_unlock_irqrestore(&cc770_isa_port_lock, flags);
    return val;
    }
    static void cc770_isa_port_write_reg_indirect(const struct cc770_priv *priv,
    int reg, u8 val)
    {
    let mut base: c_ulong = (unsigned long)priv.reg_base;
    unsigned long flags;
    spin_lock_irqsave(&cc770_isa_port_lock, flags);
    outb(reg, base);
    outb(val, base + 1);
    spin_unlock_irqrestore(&cc770_isa_port_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn cc770_isa_probe(pdev: *mut platform_device) -> c_int {
    static int cc770_isa_probe(struct platform_device *pdev)
    {
    struct net_device *dev;
    struct cc770_priv *priv;
    void __iomem *base = core::ptr::null_mut();
    let mut iosize: c_int = CC770_IOSIZE;
    let mut idx: c_int = pdev.id;
    int err;
    u32 clktmp;
    dev_dbg(&pdev.dev, "probing idx=%d: port=%#lx, mem=%#lx, irq=%d\n",
    idx, port[idx], mem[idx], irq[idx]);
    if (mem[idx]) {
    if (!request_mem_region(mem[idx], iosize, KBUILD_MODNAME)) {
    err = -EBUSY;
    goto exit;
    }
    base = ioremap(mem[idx], iosize);
    if (!base) {
    err = -ENOMEM;
    goto exit_release;
    }
    } else {
    if (indirect[idx] > 0 ||
    (indirect[idx] == -1 && indirect[0] > 0))
    iosize = CC770_IOSIZE_INDIRECT;
    if (!request_region(port[idx], iosize, KBUILD_MODNAME)) {
    err = -EBUSY;
    goto exit;
    }
    }
    dev = alloc_cc770dev(0);
    if (!dev) {
    err = -ENOMEM;
    goto exit_unmap;
    }
    priv = netdev_priv(dev);
    dev.irq = irq[idx];
    priv.irq_flags = IRQF_SHARED;
    if (mem[idx]) {
    priv.reg_base = base;
    dev.base_addr = mem[idx];
    priv.read_reg = cc770_isa_mem_read_reg;
    priv.write_reg = cc770_isa_mem_write_reg;
    } else {
    priv.reg_base = (void __iomem *)port[idx];
    dev.base_addr = port[idx];
    if (iosize == CC770_IOSIZE_INDIRECT) {
    priv.read_reg = cc770_isa_port_read_reg_indirect;
    priv.write_reg = cc770_isa_port_write_reg_indirect;
    } else {
    priv.read_reg = cc770_isa_port_read_reg;
    priv.write_reg = cc770_isa_port_write_reg;
    }
    }
    if (clk[idx])
    clktmp = clk[idx];
#[no_mangle]
pub unsafe extern "C" fn if(_arg: clk[0]) -> else {
    else if (clk[0])
    clktmp = clk[0];
    else
    clktmp = CLK_DEFAULT;
    priv.can.clock.freq = clktmp;
    if (cir[idx] != 0xff) {
    priv.cpu_interface = cir[idx];
    } else if (cir[0] != 0xff) {
    priv.cpu_interface = cir[0];
    } else {
// The system clock may not exceed 10 MHz
    if (clktmp > 10000000) {
    priv.cpu_interface |= CPUIF_DSC;
    clktmp /= 2;
    }
// The memory clock may not exceed 8 MHz
    if (clktmp > 8000000)
    priv.cpu_interface |= CPUIF_DMC;
    }
    if (priv.cpu_interface & CPUIF_DSC)
    priv.can.clock.freq /= 2;
    if (bcr[idx] != 0xff)
    priv.bus_config = bcr[idx];
#[no_mangle]
pub unsafe extern "C" fn if(0xff: bcr[0] !=) -> else {
    else if (bcr[0] != 0xff)
    priv.bus_config = bcr[0];
    else
    priv.bus_config = BCR_DEFAULT;
    if (cor[idx] != 0xff)
    priv.clkout = cor[idx];
#[no_mangle]
pub unsafe extern "C" fn if(0xff: cor[0] !=) -> else {
    else if (cor[0] != 0xff)
    priv.clkout = cor[0];
    else
    priv.clkout = COR_DEFAULT;
    platform_set_drvdata(pdev, dev);
    SET_NETDEV_DEV(dev, &pdev.dev);
    err = register_cc770dev(dev);
    if (err) {
    dev_err(&pdev.dev,
    "couldn't register device (err=%d)\n", err);
    goto exit_free;
    }
    dev_info(&pdev.dev, "device registered (reg_base=0x%p, irq=%d)\n",
    priv.reg_base, dev.irq);
    return 0;
    exit_free:
    free_cc770dev(dev);
    exit_unmap:
    if (mem[idx])
    iounmap(base);
    exit_release:
    if (mem[idx])
    release_mem_region(mem[idx], iosize);
    else
    release_region(port[idx], iosize);
    exit:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn cc770_isa_remove(pdev: *mut platform_device) {
    static void cc770_isa_remove(struct platform_device *pdev)
    {
    struct net_device *dev = platform_get_drvdata(pdev);
    struct cc770_priv *priv = netdev_priv(dev);
    let mut idx: c_int = pdev.id;
    unregister_cc770dev(dev);
    if (mem[idx]) {
    iounmap(priv.reg_base);
    release_mem_region(mem[idx], CC770_IOSIZE);
    } else {
    if (priv.read_reg == cc770_isa_port_read_reg_indirect)
    release_region(port[idx], CC770_IOSIZE_INDIRECT);
    else
    release_region(port[idx], CC770_IOSIZE);
    }
    free_cc770dev(dev);
    }
    static struct platform_driver cc770_isa_driver = {
    .probe = cc770_isa_probe,
    .remove = cc770_isa_remove,
    .driver = {
    .name = KBUILD_MODNAME,
    },
    };
#[no_mangle]
unsafe extern "C" fn cc770_isa_init() -> int __init {
    static int __init cc770_isa_init(void)
    {
    int idx, err;
    for (idx = 0; idx < ARRAY_SIZE(cc770_isa_devs); idx++) {
    if ((port[idx] || mem[idx]) && irq[idx]) {
    cc770_isa_devs[idx] =
    platform_device_alloc(KBUILD_MODNAME, idx);
    if (!cc770_isa_devs[idx]) {
    err = -ENOMEM;
    goto exit_free_devices;
    }
    err = platform_device_add(cc770_isa_devs[idx]);
    if (err) {
    platform_device_put(cc770_isa_devs[idx]);
    goto exit_free_devices;
    }
    pr_debug("platform device %d: port=%#lx, mem=%#lx, "
    "irq=%d\n",
    idx, port[idx], mem[idx], irq[idx]);
    } else if (idx == 0 || port[idx] || mem[idx]) {
    pr_err("insufficient parameters supplied\n");
    err = -EINVAL;
    goto exit_free_devices;
    }
    }
    err = platform_driver_register(&cc770_isa_driver);
    if (err)
    goto exit_free_devices;
    pr_info("driver for max. %d devices registered\n", MAXDEV);
    return 0;
    exit_free_devices:
    while (--idx >= 0) {
    if (cc770_isa_devs[idx])
    platform_device_unregister(cc770_isa_devs[idx]);
    }
    return err;
    }
    module_init(cc770_isa_init);
#[no_mangle]
unsafe extern "C" fn cc770_isa_exit() -> void __exit {
    static void __exit cc770_isa_exit(void)
    {
    int idx;
    platform_driver_unregister(&cc770_isa_driver);
    for (idx = 0; idx < ARRAY_SIZE(cc770_isa_devs); idx++) {
    if (cc770_isa_devs[idx])
    platform_device_unregister(cc770_isa_devs[idx]);
    }
    }
    module_exit(cc770_isa_exit);
