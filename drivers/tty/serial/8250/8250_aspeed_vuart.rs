//! Automatically rewritten from C to Rust
//! Source: drivers/tty/serial/8250/8250_aspeed_vuart.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Serial Port driver for Aspeed VUART device
//
// Copyright (C) 2016 Jeremy Kerr <jk@ozlabs.org>, IBM Corp.
// Copyright (C) 2006 Arnd Bergmann <arnd@arndb.de>, IBM Corp.
//

pub const ASPEED_VUART_GCRA: c_uint = 0x20;

pub const ASPEED_VUART_GCRB: c_uint = 0x24;

pub const ASPEED_VUART_GCRB_HOST_SIRQ_SHIFT: c_int = 4;
pub const ASPEED_VUART_ADDRL: c_uint = 0x28;
pub const ASPEED_VUART_ADDRH: c_uint = 0x2c;
pub const ASPEED_VUART_DEFAULT_LPC_ADDR: c_uint = 0x3f8;
pub const ASPEED_VUART_DEFAULT_SIRQ: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aspeed_vuart {
    pub dev: *mut device,
    pub line: c_int,
    pub unthrottle_timer: timer_list,
    pub port: *mut uart_8250_port,
}

//
// If we fill the tty flip buffers, we throttle the data ready interrupt
// to prevent dropped characters. This timeout defines how long we wait
// to (conditionally, depending on buffer state) unthrottle.
//
    let mut unthrottle_timeout: static int = HZ/10;
//
// The VUART is basically two UART 'front ends' connected by their FIFO
// (no actual serial line in between). One is on the BMC side (management
// controller) and one is on the host CPU side.
//
// It allows the BMC to provide to the host a "UART" that pipes into
// the BMC itself and can then be turned by the BMC into a network console
// of some sort for example.
//
// This driver is for the BMC side. The sysfs files allow the BMC
// userspace which owns the system configuration policy, to specify
// at what IO port and interrupt number the host side will appear
// to the host on the Host <-> BMC LPC bus. It could be different on a
// different system (though most of them use 3f8/4).
//
#[no_mangle]
pub unsafe extern "C" fn aspeed_vuart_readb(vuart: *mut aspeed_vuart, reg: u8) -> u8 {
    static inline u8 aspeed_vuart_readb(struct aspeed_vuart *vuart, u8 reg)
    {
    return readb(vuart.port.port.membase + reg);
    }
#[no_mangle]
pub unsafe extern "C" fn aspeed_vuart_writeb(vuart: *mut aspeed_vuart, val: u8, reg: u8) {
    static inline void aspeed_vuart_writeb(struct aspeed_vuart *vuart, u8 val, u8 reg)
    {
    writeb(val, vuart.port.port.membase + reg);
    }
    static ssize_t lpc_address_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct aspeed_vuart *vuart = dev_get_drvdata(dev);
    u16 addr;
    addr = (aspeed_vuart_readb(vuart, ASPEED_VUART_ADDRH) << 8) |
    (aspeed_vuart_readb(vuart, ASPEED_VUART_ADDRL));
    return sysfs_emit(buf, "0x%x\n", addr);
    }
#[no_mangle]
unsafe extern "C" fn aspeed_vuart_set_lpc_address(vuart: *mut aspeed_vuart, addr: u32) -> c_int {
    static int aspeed_vuart_set_lpc_address(struct aspeed_vuart *vuart, u32 addr)
    {
    if (addr > U16_MAX)
    return -EINVAL;
    aspeed_vuart_writeb(vuart, addr >> 8, ASPEED_VUART_ADDRH);
    aspeed_vuart_writeb(vuart, addr >> 0, ASPEED_VUART_ADDRL);
    return 0;
    }
    static ssize_t lpc_address_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct aspeed_vuart *vuart = dev_get_drvdata(dev);
    u32 val;
    int err;
    err = kstrtou32(buf, 0, &val);
    if (err)
    return err;
    err = aspeed_vuart_set_lpc_address(vuart, val);
    return err ? : count;
    }
    static DEVICE_ATTR_RW(lpc_address);
    static ssize_t sirq_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct aspeed_vuart *vuart = dev_get_drvdata(dev);
    u8 reg;
    reg = aspeed_vuart_readb(vuart, ASPEED_VUART_GCRB);
    reg &= ASPEED_VUART_GCRB_HOST_SIRQ_MASK;
    reg >>= ASPEED_VUART_GCRB_HOST_SIRQ_SHIFT;
    return sysfs_emit(buf, "%u\n", reg);
    }
#[no_mangle]
unsafe extern "C" fn aspeed_vuart_set_sirq(vuart: *mut aspeed_vuart, sirq: u32) -> c_int {
    static int aspeed_vuart_set_sirq(struct aspeed_vuart *vuart, u32 sirq)
    {
    u8 reg;
    if (sirq > (ASPEED_VUART_GCRB_HOST_SIRQ_MASK >> ASPEED_VUART_GCRB_HOST_SIRQ_SHIFT))
    return -EINVAL;
    sirq <<= ASPEED_VUART_GCRB_HOST_SIRQ_SHIFT;
    sirq &= ASPEED_VUART_GCRB_HOST_SIRQ_MASK;
    reg = aspeed_vuart_readb(vuart, ASPEED_VUART_GCRB);
    reg &= ~ASPEED_VUART_GCRB_HOST_SIRQ_MASK;
    reg |= sirq;
    aspeed_vuart_writeb(vuart, reg, ASPEED_VUART_GCRB);
    return 0;
    }
    static ssize_t sirq_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct aspeed_vuart *vuart = dev_get_drvdata(dev);
    unsigned long val;
    int err;
    err = kstrtoul(buf, 0, &val);
    if (err)
    return err;
    err = aspeed_vuart_set_sirq(vuart, val);
    return err ? : count;
    }
    static DEVICE_ATTR_RW(sirq);
    static ssize_t sirq_polarity_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct aspeed_vuart *vuart = dev_get_drvdata(dev);
    u8 reg;
    reg = aspeed_vuart_readb(vuart, ASPEED_VUART_GCRA);
    reg &= ASPEED_VUART_GCRA_HOST_SIRQ_POLARITY;
    return sysfs_emit(buf, "%u\n", reg ? 1 : 0);
    }
    static void aspeed_vuart_set_sirq_polarity(struct aspeed_vuart *vuart,
    bool polarity)
    {
    let mut reg: u8 = aspeed_vuart_readb(vuart, ASPEED_VUART_GCRA);
    if (polarity)
    reg |= ASPEED_VUART_GCRA_HOST_SIRQ_POLARITY;
    else
    reg &= ~ASPEED_VUART_GCRA_HOST_SIRQ_POLARITY;
    aspeed_vuart_writeb(vuart, reg, ASPEED_VUART_GCRA);
    }
    static ssize_t sirq_polarity_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct aspeed_vuart *vuart = dev_get_drvdata(dev);
    unsigned long val;
    int err;
    err = kstrtoul(buf, 0, &val);
    if (err)
    return err;
    aspeed_vuart_set_sirq_polarity(vuart, val != 0);
    return count;
    }
    static DEVICE_ATTR_RW(sirq_polarity);
    static struct attribute *aspeed_vuart_attrs[] = {
    &dev_attr_sirq.attr,
    &dev_attr_sirq_polarity.attr,
    &dev_attr_lpc_address.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group aspeed_vuart_attr_group = {
    .attrs = aspeed_vuart_attrs,
    };
#[no_mangle]
unsafe extern "C" fn aspeed_vuart_set_enabled(vuart: *mut aspeed_vuart, enabled: bool) {
    static void aspeed_vuart_set_enabled(struct aspeed_vuart *vuart, bool enabled)
    {
    let mut reg: u8 = aspeed_vuart_readb(vuart, ASPEED_VUART_GCRA);
    if (enabled)
    reg |= ASPEED_VUART_GCRA_VUART_EN;
    else
    reg &= ~ASPEED_VUART_GCRA_VUART_EN;
    aspeed_vuart_writeb(vuart, reg, ASPEED_VUART_GCRA);
    }
    static void aspeed_vuart_set_host_tx_discard(struct aspeed_vuart *vuart,
    bool discard)
    {
    u8 reg;
    reg = aspeed_vuart_readb(vuart, ASPEED_VUART_GCRA);
// If the DISABLE_HOST_TX_DISCARD bit is set, discard is disabled
    if (!discard)
    reg |= ASPEED_VUART_GCRA_DISABLE_HOST_TX_DISCARD;
    else
    reg &= ~ASPEED_VUART_GCRA_DISABLE_HOST_TX_DISCARD;
    aspeed_vuart_writeb(vuart, reg, ASPEED_VUART_GCRA);
    }
#[no_mangle]
unsafe extern "C" fn aspeed_vuart_startup(uart_port: *mut uart_port) -> c_int {
    static int aspeed_vuart_startup(struct uart_port *uart_port)
    {
    struct uart_8250_port *uart_8250_port = up_to_u8250p(uart_port);
    struct aspeed_vuart *vuart = uart_8250_port.port.private_data;
    int rc;
    rc = serial8250_do_startup(uart_port);
    if (rc)
    return rc;
    aspeed_vuart_set_host_tx_discard(vuart, false);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn aspeed_vuart_shutdown(uart_port: *mut uart_port) {
    static void aspeed_vuart_shutdown(struct uart_port *uart_port)
    {
    struct uart_8250_port *uart_8250_port = up_to_u8250p(uart_port);
    struct aspeed_vuart *vuart = uart_8250_port.port.private_data;
    aspeed_vuart_set_host_tx_discard(vuart, true);
    serial8250_do_shutdown(uart_port);
    }
    static void __aspeed_vuart_set_throttle(struct uart_8250_port *up,
    bool throttle)
    {
    let mut irqs: c_uchar = UART_IER_RLSI | UART_IER_RDI;
// Port locked to synchronize UART_IER access against the console.
    lockdep_assert_held_once(&up.port.lock);
    up.ier &= ~irqs;
    if (!throttle)
    up.ier |= irqs;
    serial_out(up, UART_IER, up.ier);
    }
#[no_mangle]
unsafe extern "C" fn aspeed_vuart_set_throttle(port: *mut uart_port, throttle: bool) {
    static void aspeed_vuart_set_throttle(struct uart_port *port, bool throttle)
    {
    struct uart_8250_port *up = up_to_u8250p(port);
    unsigned long flags;
    uart_port_lock_irqsave(port, &flags);
    __aspeed_vuart_set_throttle(up, throttle);
    uart_port_unlock_irqrestore(port, flags);
    }
#[no_mangle]
unsafe extern "C" fn aspeed_vuart_throttle(port: *mut uart_port) {
    static void aspeed_vuart_throttle(struct uart_port *port)
    {
    aspeed_vuart_set_throttle(port, true);
    }
#[no_mangle]
unsafe extern "C" fn aspeed_vuart_unthrottle(port: *mut uart_port) {
    static void aspeed_vuart_unthrottle(struct uart_port *port)
    {
    aspeed_vuart_set_throttle(port, false);
    }
#[no_mangle]
unsafe extern "C" fn aspeed_vuart_unthrottle_exp(timer: *mut timer_list) {
    static void aspeed_vuart_unthrottle_exp(struct timer_list *timer)
    {
    struct aspeed_vuart *vuart = timer_container_of(vuart, timer,
    unthrottle_timer);
    struct uart_8250_port *up = vuart.port;
    if (!tty_buffer_space_avail(&up.port.state.port)) {
    mod_timer(&vuart.unthrottle_timer,
    jiffies + unthrottle_timeout);
    return;
    }
    aspeed_vuart_unthrottle(&up.port);
    }
//
// Custom interrupt handler to manage finer-grained flow control. Although we
// have throttle/unthrottle callbacks, we've seen that the VUART device can
// deliver characters faster than the ldisc has a chance to check buffer space
// against the throttle threshold. This results in dropped characters before
// the throttle.
//
// We do this by checking for flip buffer space before RX. If we have no space,
// throttle now and schedule an unthrottle for later, once the ldisc has had
// a chance to drain the buffers.
//
#[no_mangle]
unsafe extern "C" fn aspeed_vuart_handle_irq(port: *mut uart_port) -> c_int {
    static int aspeed_vuart_handle_irq(struct uart_port *port)
    {
    struct uart_8250_port *up = up_to_u8250p(port);
    unsigned int iir, lsr;
    unsigned long flags;
    unsigned int space, count;
    iir = serial_port_in(port, UART_IIR);
    if (iir & UART_IIR_NO_INT)
    return 0;
    uart_port_lock_irqsave(port, &flags);
    lsr = serial_port_in(port, UART_LSR);
    if (lsr & (UART_LSR_DR | UART_LSR_BI)) {
    space = tty_buffer_space_avail(&port.state.port);
    if (!space) {
// throttle and schedule an unthrottle later
    struct aspeed_vuart *vuart = port.private_data;
    __aspeed_vuart_set_throttle(up, true);
    if (!timer_pending(&vuart.unthrottle_timer))
    mod_timer(&vuart.unthrottle_timer,
    jiffies + unthrottle_timeout);
    } else {
    count = min(space, 256U);
    do {
    serial8250_read_char(up, lsr);
    lsr = serial_in(up, UART_LSR);
    if (--count == 0)
    break;
    } while (lsr & (UART_LSR_DR | UART_LSR_BI));
    tty_flip_buffer_push(&port.state.port);
    }
    }
    serial8250_modem_status(up);
    if (lsr & UART_LSR_THRE)
    serial8250_tx_chars(up);
    uart_unlock_and_check_sysrq_irqrestore(port, flags);
    return 1;
    }
    static void aspeed_vuart_auto_configure_sirq_polarity(
    struct aspeed_vuart *vuart, struct device_node *syscon_np,
    u32 reg_offset, u32 reg_mask)
    {
    struct regmap *regmap;
    u32 value;
    regmap = syscon_node_to_regmap(syscon_np);
    if (IS_ERR(regmap)) {
    dev_warn(vuart.dev,
    "could not get regmap for aspeed,sirq-polarity-sense\n");
    return;
    }
    if (regmap_read(regmap, reg_offset, &value)) {
    dev_warn(vuart.dev, "could not read hw strap table\n");
    return;
    }
    aspeed_vuart_set_sirq_polarity(vuart, (value & reg_mask) == 0);
    }
#[no_mangle]
unsafe extern "C" fn aspeed_vuart_map_irq_polarity(dt: u32) -> c_int {
    static int aspeed_vuart_map_irq_polarity(u32 dt)
    {
    switch (dt) {
    case IRQ_TYPE_LEVEL_LOW:
    return 0;
    case IRQ_TYPE_LEVEL_HIGH:
    return 1;
    default:
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn aspeed_vuart_probe(pdev: *mut platform_device) -> c_int {
    static int aspeed_vuart_probe(struct platform_device *pdev)
    {
    struct of_phandle_args sirq_polarity_sense_args;
    struct device *dev = &pdev.dev;
    struct uart_8250_port port;
    struct aspeed_vuart *vuart;
    struct device_node *np;
    struct resource *res;
    int rc, sirq_polarity;
    u32 prop, sirq[2];
    struct clk *vclk;
    np = pdev.dev.of_node;
    vuart = devm_kzalloc(&pdev.dev, sizeof(*vuart), GFP_KERNEL);
    if (!vuart)
    return -ENOMEM;
    vuart.dev = &pdev.dev;
    timer_setup(&vuart.unthrottle_timer, aspeed_vuart_unthrottle_exp, 0);
    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!res)
    return -EINVAL;
    memset(&port, 0, sizeof(port));
    port.port.private_data = vuart;
    port.port.mapbase = res.start;
    port.port.mapsize = resource_size(res);
    port.port.startup = aspeed_vuart_startup;
    port.port.shutdown = aspeed_vuart_shutdown;
    port.port.throttle = aspeed_vuart_throttle;
    port.port.unthrottle = aspeed_vuart_unthrottle;
    port.port.status = UPSTAT_SYNC_FIFO;
    port.port.dev = &pdev.dev;
    port.port.has_sysrq = IS_ENABLED(CONFIG_SERIAL_8250_CONSOLE);
    port.port.flags = UPF_BOOT_AUTOCONF | UPF_IOREMAP | UPF_FIXED_PORT | UPF_FIXED_TYPE |
    UPF_NO_THRE_TEST;
    port.bugs |= UART_BUG_TXRACE;
    rc = sysfs_create_group(&vuart.dev.kobj, &aspeed_vuart_attr_group);
    if (rc < 0)
    return rc;
    rc = uart_read_port_properties(&port.port);
    if (rc)
    goto err_sysfs_remove;
// Get clk rate through clk driver if present
    if (!port.port.uartclk) {
    vclk = devm_clk_get_enabled(dev, core::ptr::null_mut());
    if (IS_ERR(vclk)) {
    rc = dev_err_probe(dev, PTR_ERR(vclk), "clk or clock-frequency not defined\n");
    goto err_sysfs_remove;
    }
    port.port.uartclk = clk_get_rate(vclk);
    }
// If current-speed was set, then try not to change it.
    if (of_property_read_u32(np, "current-speed", &prop) == 0)
    port.port.custom_divisor = port.port.uartclk / (16 * prop);
    port.port.handle_irq = aspeed_vuart_handle_irq;
    port.port.type = PORT_ASPEED_VUART;
    if (port.port.fifosize)
    port.capabilities = UART_CAP_FIFO;
    if (of_property_read_bool(np, "auto-flow-control"))
    port.capabilities |= UART_CAP_AFE;
    rc = serial8250_register_8250_port(&port);
    if (rc < 0)
    goto err_sysfs_remove;
    vuart.line = rc;
    vuart.port = serial8250_get_port(vuart.line);
    rc = of_parse_phandle_with_fixed_args(
    np, "aspeed,sirq-polarity-sense", 2, 0,
    &sirq_polarity_sense_args);
    if (rc < 0) {
    dev_dbg(&pdev.dev,
    "aspeed,sirq-polarity-sense property not found\n");
    } else {
    aspeed_vuart_auto_configure_sirq_polarity(
    vuart, sirq_polarity_sense_args.np,
    sirq_polarity_sense_args.args[0],
    BIT(sirq_polarity_sense_args.args[1]));
    of_node_put(sirq_polarity_sense_args.np);
    }
    rc = of_property_read_u32(np, "aspeed,lpc-io-reg", &prop);
    if (rc < 0)
    prop = ASPEED_VUART_DEFAULT_LPC_ADDR;
    rc = aspeed_vuart_set_lpc_address(vuart, prop);
    if (rc < 0) {
    dev_err_probe(dev, rc, "invalid value in aspeed,lpc-io-reg property\n");
    goto err_sysfs_remove;
    }
    rc = of_property_read_u32_array(np, "aspeed,lpc-interrupts", sirq, 2);
    if (rc < 0) {
    sirq[0] = ASPEED_VUART_DEFAULT_SIRQ;
    sirq[1] = ASPEED_VUART_DEFAULT_SIRQ_POLARITY;
    }
    rc = aspeed_vuart_set_sirq(vuart, sirq[0]);
    if (rc < 0) {
    dev_err_probe(dev, rc, "invalid sirq number in aspeed,lpc-interrupts property\n");
    goto err_sysfs_remove;
    }
    sirq_polarity = aspeed_vuart_map_irq_polarity(sirq[1]);
    if (sirq_polarity < 0) {
    rc = dev_err_probe(dev, sirq_polarity,
    "invalid sirq polarity in aspeed,lpc-interrupts property\n");
    goto err_sysfs_remove;
    }
    aspeed_vuart_set_sirq_polarity(vuart, sirq_polarity);
    aspeed_vuart_set_enabled(vuart, true);
    aspeed_vuart_set_host_tx_discard(vuart, true);
    platform_set_drvdata(pdev, vuart);
    return 0;
    err_sysfs_remove:
    sysfs_remove_group(&vuart.dev.kobj, &aspeed_vuart_attr_group);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn aspeed_vuart_remove(pdev: *mut platform_device) {
    static void aspeed_vuart_remove(struct platform_device *pdev)
    {
    struct aspeed_vuart *vuart = platform_get_drvdata(pdev);
    timer_delete_sync(&vuart.unthrottle_timer);
    aspeed_vuart_set_enabled(vuart, false);
    serial8250_unregister_port(vuart.line);
    sysfs_remove_group(&vuart.dev.kobj, &aspeed_vuart_attr_group);
    }
    static const struct of_device_id aspeed_vuart_table[] = {
    { .compatible = "aspeed,ast2400-vuart" },
    { .compatible = "aspeed,ast2500-vuart" },
    { },
    };
    MODULE_DEVICE_TABLE(of, aspeed_vuart_table);
    static struct platform_driver aspeed_vuart_driver = {
    .driver = {
    .name = "aspeed-vuart",
    .of_match_table = aspeed_vuart_table,
    },
    .probe = aspeed_vuart_probe,
    .remove = aspeed_vuart_remove,
    };
    module_platform_driver(aspeed_vuart_driver);
    MODULE_AUTHOR("Jeremy Kerr <jk@ozlabs.org>");
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Driver for Aspeed VUART device");
