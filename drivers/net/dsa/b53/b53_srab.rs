//! Automatically rewritten from C to Rust
//! Source: drivers/net/dsa/b53/b53_srab.c
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


//
// B53 register access through Switch Register Access Bridge Registers
//
// Copyright (C) 2013 Hauke Mehrtens <hauke@hauke-m.de>
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//

// command and status register of the SRAB
pub const B53_SRAB_CMDSTAT: c_uint = 0x2c;

pub const B53_SRAB_CMDSTAT_PAGE: c_int = 24;
pub const B53_SRAB_CMDSTAT_REG: c_int = 16;
// high order word of write data to switch registe
pub const B53_SRAB_WD_H: c_uint = 0x30;
// low order word of write data to switch registe
pub const B53_SRAB_WD_L: c_uint = 0x34;
// high order word of read data from switch register
pub const B53_SRAB_RD_H: c_uint = 0x38;
// low order word of read data from switch register
pub const B53_SRAB_RD_L: c_uint = 0x3c;
// command and status register of the SRAB
pub const B53_SRAB_CTRLS: c_uint = 0x40;

// the register captures interrupt pulses from the switch
pub const B53_SRAB_INTR: c_uint = 0x44;

// Port mux configuration registers
pub const B53_MUX_CONFIG_P5: c_uint = 0x00;
pub const MUX_CONFIG_SGMII: c_int = 0;
pub const MUX_CONFIG_MII_LITE: c_int = 1;
pub const MUX_CONFIG_RGMII: c_int = 2;
pub const MUX_CONFIG_GMII: c_int = 3;
pub const MUX_CONFIG_GPHY: c_int = 4;
pub const MUX_CONFIG_INTERNAL: c_int = 5;
pub const MUX_CONFIG_MASK: c_uint = 0x7;
pub const B53_MUX_CONFIG_P4: c_uint = 0x04;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b53_srab_port_priv {
    pub irq: c_int,
    pub irq_enabled: bool,
    pub dev: *mut b53_device,
    pub num: c_uint,
    pub mode: phy_interface_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct b53_srab_priv {
    pub regs: *mut void __iomem,
    pub mux_config: *mut void __iomem,
    pub port_intrs: [b53_srab_port_priv; B53_N_PORTS],
}

#[no_mangle]
unsafe extern "C" fn b53_srab_request_grant(dev: *mut b53_device) -> c_int {
    static int b53_srab_request_grant(struct b53_device *dev)
    {
    struct b53_srab_priv *priv = dev.priv;
    u8 __iomem *regs = priv.regs;
    u32 ctrls;
    int i;
    ctrls = readl(regs + B53_SRAB_CTRLS);
    ctrls |= B53_SRAB_CTRLS_RCAREQ;
    writel(ctrls, regs + B53_SRAB_CTRLS);
    for (i = 0; i < 20; i++) {
    ctrls = readl(regs + B53_SRAB_CTRLS);
    if (ctrls & B53_SRAB_CTRLS_RCAGNT)
    break;
    usleep_range(10, 100);
    }
    if (WARN_ON(i == 5))
    return -EIO;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn b53_srab_release_grant(dev: *mut b53_device) {
    static void b53_srab_release_grant(struct b53_device *dev)
    {
    struct b53_srab_priv *priv = dev.priv;
    u8 __iomem *regs = priv.regs;
    u32 ctrls;
    ctrls = readl(regs + B53_SRAB_CTRLS);
    ctrls &= ~B53_SRAB_CTRLS_RCAREQ;
    writel(ctrls, regs + B53_SRAB_CTRLS);
    }
#[no_mangle]
unsafe extern "C" fn b53_srab_op(dev: *mut b53_device, page: u8, reg: u8, op: u32) -> c_int {
    static int b53_srab_op(struct b53_device *dev, u8 page, u8 reg, u32 op)
    {
    struct b53_srab_priv *priv = dev.priv;
    u8 __iomem *regs = priv.regs;
    int i;
    u32 cmdstat;
// set register address
    cmdstat = (page << B53_SRAB_CMDSTAT_PAGE) |
    (reg << B53_SRAB_CMDSTAT_REG) |
    B53_SRAB_CMDSTAT_GORDYN |
    op;
    writel(cmdstat, regs + B53_SRAB_CMDSTAT);
// check if operation completed
    for (i = 0; i < 5; ++i) {
    cmdstat = readl(regs + B53_SRAB_CMDSTAT);
    if (!(cmdstat & B53_SRAB_CMDSTAT_GORDYN))
    break;
    usleep_range(10, 100);
    }
    if (WARN_ON(i == 5))
    return -EIO;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn b53_srab_read8(dev: *mut b53_device, page: u8, reg: u8, val: *mut u8) -> c_int {
    static int b53_srab_read8(struct b53_device *dev, u8 page, u8 reg, u8 *val)
    {
    struct b53_srab_priv *priv = dev.priv;
    u8 __iomem *regs = priv.regs;
    let mut ret: c_int = 0;
    ret = b53_srab_request_grant(dev);
    if (ret)
    goto err;
    ret = b53_srab_op(dev, page, reg, 0);
    if (ret)
    goto err;
// val = readl(regs + B53_SRAB_RD_L) & 0xff;
    err:
    b53_srab_release_grant(dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn b53_srab_read16(dev: *mut b53_device, page: u8, reg: u8, val: *mut u16) -> c_int {
    static int b53_srab_read16(struct b53_device *dev, u8 page, u8 reg, u16 *val)
    {
    struct b53_srab_priv *priv = dev.priv;
    u8 __iomem *regs = priv.regs;
    let mut ret: c_int = 0;
    ret = b53_srab_request_grant(dev);
    if (ret)
    goto err;
    ret = b53_srab_op(dev, page, reg, 0);
    if (ret)
    goto err;
// val = readl(regs + B53_SRAB_RD_L) & 0xffff;
    err:
    b53_srab_release_grant(dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn b53_srab_read32(dev: *mut b53_device, page: u8, reg: u8, val: *mut u32) -> c_int {
    static int b53_srab_read32(struct b53_device *dev, u8 page, u8 reg, u32 *val)
    {
    struct b53_srab_priv *priv = dev.priv;
    u8 __iomem *regs = priv.regs;
    let mut ret: c_int = 0;
    ret = b53_srab_request_grant(dev);
    if (ret)
    goto err;
    ret = b53_srab_op(dev, page, reg, 0);
    if (ret)
    goto err;
// val = readl(regs + B53_SRAB_RD_L);
    err:
    b53_srab_release_grant(dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn b53_srab_read48(dev: *mut b53_device, page: u8, reg: u8, val: *mut u64) -> c_int {
    static int b53_srab_read48(struct b53_device *dev, u8 page, u8 reg, u64 *val)
    {
    struct b53_srab_priv *priv = dev.priv;
    u8 __iomem *regs = priv.regs;
    let mut ret: c_int = 0;
    ret = b53_srab_request_grant(dev);
    if (ret)
    goto err;
    ret = b53_srab_op(dev, page, reg, 0);
    if (ret)
    goto err;
// val = readl(regs + B53_SRAB_RD_L);
// val += ((u64)readl(regs + B53_SRAB_RD_H) & 0xffff) << 32;
    err:
    b53_srab_release_grant(dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn b53_srab_read64(dev: *mut b53_device, page: u8, reg: u8, val: *mut u64) -> c_int {
    static int b53_srab_read64(struct b53_device *dev, u8 page, u8 reg, u64 *val)
    {
    struct b53_srab_priv *priv = dev.priv;
    u8 __iomem *regs = priv.regs;
    let mut ret: c_int = 0;
    ret = b53_srab_request_grant(dev);
    if (ret)
    goto err;
    ret = b53_srab_op(dev, page, reg, 0);
    if (ret)
    goto err;
// val = readl(regs + B53_SRAB_RD_L);
// val += (u64)readl(regs + B53_SRAB_RD_H) << 32;
    err:
    b53_srab_release_grant(dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn b53_srab_write8(dev: *mut b53_device, page: u8, reg: u8, value: u8) -> c_int {
    static int b53_srab_write8(struct b53_device *dev, u8 page, u8 reg, u8 value)
    {
    struct b53_srab_priv *priv = dev.priv;
    u8 __iomem *regs = priv.regs;
    let mut ret: c_int = 0;
    ret = b53_srab_request_grant(dev);
    if (ret)
    goto err;
    writel(value, regs + B53_SRAB_WD_L);
    ret = b53_srab_op(dev, page, reg, B53_SRAB_CMDSTAT_WRITE);
    err:
    b53_srab_release_grant(dev);
    return ret;
    }
    static int b53_srab_write16(struct b53_device *dev, u8 page, u8 reg,
    u16 value)
    {
    struct b53_srab_priv *priv = dev.priv;
    u8 __iomem *regs = priv.regs;
    let mut ret: c_int = 0;
    ret = b53_srab_request_grant(dev);
    if (ret)
    goto err;
    writel(value, regs + B53_SRAB_WD_L);
    ret = b53_srab_op(dev, page, reg, B53_SRAB_CMDSTAT_WRITE);
    err:
    b53_srab_release_grant(dev);
    return ret;
    }
    static int b53_srab_write32(struct b53_device *dev, u8 page, u8 reg,
    u32 value)
    {
    struct b53_srab_priv *priv = dev.priv;
    u8 __iomem *regs = priv.regs;
    let mut ret: c_int = 0;
    ret = b53_srab_request_grant(dev);
    if (ret)
    goto err;
    writel(value, regs + B53_SRAB_WD_L);
    ret = b53_srab_op(dev, page, reg, B53_SRAB_CMDSTAT_WRITE);
    err:
    b53_srab_release_grant(dev);
    return ret;
    }
    static int b53_srab_write48(struct b53_device *dev, u8 page, u8 reg,
    u64 value)
    {
    struct b53_srab_priv *priv = dev.priv;
    u8 __iomem *regs = priv.regs;
    let mut ret: c_int = 0;
    ret = b53_srab_request_grant(dev);
    if (ret)
    goto err;
    writel((u32)value, regs + B53_SRAB_WD_L);
    writel((u16)(value >> 32), regs + B53_SRAB_WD_H);
    ret = b53_srab_op(dev, page, reg, B53_SRAB_CMDSTAT_WRITE);
    err:
    b53_srab_release_grant(dev);
    return ret;
    }
    static int b53_srab_write64(struct b53_device *dev, u8 page, u8 reg,
    u64 value)
    {
    struct b53_srab_priv *priv = dev.priv;
    u8 __iomem *regs = priv.regs;
    let mut ret: c_int = 0;
    ret = b53_srab_request_grant(dev);
    if (ret)
    goto err;
    writel((u32)value, regs + B53_SRAB_WD_L);
    writel((u32)(value >> 32), regs + B53_SRAB_WD_H);
    ret = b53_srab_op(dev, page, reg, B53_SRAB_CMDSTAT_WRITE);
    err:
    b53_srab_release_grant(dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn b53_srab_port_thread(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t b53_srab_port_thread(int irq, void *dev_id)
    {
    struct b53_srab_port_priv *port = dev_id;
    struct b53_device *dev = port.dev;
    if (port.mode == PHY_INTERFACE_MODE_SGMII)
    b53_port_event(dev.ds, port.num);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn b53_srab_port_isr(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t b53_srab_port_isr(int irq, void *dev_id)
    {
    struct b53_srab_port_priv *port = dev_id;
    struct b53_device *dev = port.dev;
    struct b53_srab_priv *priv = dev.priv;
// Acknowledge the interrupt
    writel(BIT(port.num), priv.regs + B53_SRAB_INTR);
    return IRQ_WAKE_THREAD;
    }

#[no_mangle]
unsafe extern "C" fn b53_srab_serdes_map_lane(dev: *mut b53_device, port: c_int) -> u8 {
    static u8 b53_srab_serdes_map_lane(struct b53_device *dev, int port)
    {
    struct b53_srab_priv *priv = dev.priv;
    struct b53_srab_port_priv *p = &priv.port_intrs[port];
    if (p.mode != PHY_INTERFACE_MODE_SGMII)
    return B53_INVALID_LANE;
    switch (port) {
    case 5:
    return 0;
    case 4:
    return 1;
    default:
    return B53_INVALID_LANE;
    }
    }

#[no_mangle]
unsafe extern "C" fn b53_srab_irq_enable(dev: *mut b53_device, port: c_int) -> c_int {
    static int b53_srab_irq_enable(struct b53_device *dev, int port)
    {
    struct b53_srab_priv *priv = dev.priv;
    struct b53_srab_port_priv *p = &priv.port_intrs[port];
    let mut ret: c_int = 0;
// Interrupt is optional and was not specified, do not make
// this fatal
//
    if (p.irq == -ENXIO)
    return ret;
    ret = request_threaded_irq(p.irq, b53_srab_port_isr,
    b53_srab_port_thread, 0,
    dev_name(dev.dev), p);
    if (!ret)
    p.irq_enabled = true;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn b53_srab_irq_disable(dev: *mut b53_device, port: c_int) {
    static void b53_srab_irq_disable(struct b53_device *dev, int port)
    {
    struct b53_srab_priv *priv = dev.priv;
    struct b53_srab_port_priv *p = &priv.port_intrs[port];
    if (p.irq_enabled) {
    free_irq(p.irq, p);
    p.irq_enabled = false;
    }
    }
    static void b53_srab_phylink_get_caps(struct b53_device *dev, int port,
    struct phylink_config *config)
    {
    struct b53_srab_priv *priv = dev.priv;
    struct b53_srab_port_priv *p = &priv.port_intrs[port];
    switch (p.mode) {
    case PHY_INTERFACE_MODE_SGMII:

// If p->mode indicates SGMII mode, that essentially means we
// are using a serdes. As the serdes for the capabilities.
//
    b53_serdes_phylink_get_caps(dev, port, config);

    break;
    case PHY_INTERFACE_MODE_NA:
    break;
    case PHY_INTERFACE_MODE_RGMII:
// If we support RGMII, support all RGMII modes, since
// that dictates the PHY delay settings.
//
    phy_interface_set_rgmii(config.supported_interfaces);
    break;
    default:
// Some other mode (e.g. MII, GMII etc)
    __set_bit(p.mode, config.supported_interfaces);
    break;
    }
    }
    static const struct b53_io_ops b53_srab_ops = {
    .read8 = b53_srab_read8,
    .read16 = b53_srab_read16,
    .read32 = b53_srab_read32,
    .read48 = b53_srab_read48,
    .read64 = b53_srab_read64,
    .write8 = b53_srab_write8,
    .write16 = b53_srab_write16,
    .write32 = b53_srab_write32,
    .write48 = b53_srab_write48,
    .write64 = b53_srab_write64,
    .irq_enable = b53_srab_irq_enable,
    .irq_disable = b53_srab_irq_disable,
    .phylink_get_caps = b53_srab_phylink_get_caps,

    .phylink_mac_select_pcs = b53_serdes_phylink_mac_select_pcs,
    .serdes_map_lane = b53_srab_serdes_map_lane,
    .serdes_link_set = b53_serdes_link_set,

    };
    static const struct of_device_id b53_srab_of_match[] = {
    { .compatible = "brcm,bcm53010-srab" },
    { .compatible = "brcm,bcm53011-srab" },
    { .compatible = "brcm,bcm53012-srab" },
    { .compatible = "brcm,bcm53018-srab" },
    { .compatible = "brcm,bcm53019-srab" },
    { .compatible = "brcm,bcm5301x-srab" },
    { .compatible = "brcm,bcm11360-srab", .data = (void *)BCM583XX_DEVICE_ID },
    { .compatible = "brcm,bcm58522-srab", .data = (void *)BCM58XX_DEVICE_ID },
    { .compatible = "brcm,bcm58525-srab", .data = (void *)BCM58XX_DEVICE_ID },
    { .compatible = "brcm,bcm58535-srab", .data = (void *)BCM58XX_DEVICE_ID },
    { .compatible = "brcm,bcm58622-srab", .data = (void *)BCM58XX_DEVICE_ID },
    { .compatible = "brcm,bcm58623-srab", .data = (void *)BCM58XX_DEVICE_ID },
    { .compatible = "brcm,bcm58625-srab", .data = (void *)BCM58XX_DEVICE_ID },
    { .compatible = "brcm,bcm88312-srab", .data = (void *)BCM58XX_DEVICE_ID },
    { .compatible = "brcm,cygnus-srab", .data = (void *)BCM583XX_DEVICE_ID },
    { .compatible = "brcm,nsp-srab", .data = (void *)BCM58XX_DEVICE_ID },
    { .compatible = "brcm,omega-srab", .data = (void *)BCM583XX_DEVICE_ID },
    { /* sentinel */ },
    };
    MODULE_DEVICE_TABLE(of, b53_srab_of_match);
#[no_mangle]
unsafe extern "C" fn b53_srab_intr_set(priv: *mut b53_srab_priv, set: bool) {
    static void b53_srab_intr_set(struct b53_srab_priv *priv, bool set)
    {
    u32 reg;
    reg = readl(priv.regs + B53_SRAB_CTRLS);
    if (set)
    reg |= B53_SRAB_CTRLS_HOST_INTR;
    else
    reg &= ~B53_SRAB_CTRLS_HOST_INTR;
    writel(reg, priv.regs + B53_SRAB_CTRLS);
    }
#[no_mangle]
unsafe extern "C" fn b53_srab_prepare_irq(pdev: *mut platform_device) {
    static void b53_srab_prepare_irq(struct platform_device *pdev)
    {
    struct b53_device *dev = platform_get_drvdata(pdev);
    struct b53_srab_priv *priv = dev.priv;
    struct b53_srab_port_priv *port;
    unsigned int i;
    char *name;
// Clear all pending interrupts
    writel(0xffffffff, priv.regs + B53_SRAB_INTR);
    for (i = 0; i < B53_N_PORTS; i++) {
    port = &priv.port_intrs[i];
// There is no port 6
    if (i == 6)
    continue;
    name = kasprintf(GFP_KERNEL, "link_state_p%d", i);
    if (!name)
    return;
    port.num = i;
    port.dev = dev;
    port.irq = platform_get_irq_byname_optional(pdev, name);
    kfree(name);
    }
    b53_srab_intr_set(priv, true);
    }
#[no_mangle]
unsafe extern "C" fn b53_srab_mux_init(pdev: *mut platform_device) {
    static void b53_srab_mux_init(struct platform_device *pdev)
    {
    struct b53_device *dev = platform_get_drvdata(pdev);
    struct b53_srab_priv *priv = dev.priv;
    struct b53_srab_port_priv *p;
    unsigned int port;
    u32 reg, off = 0;
    int ret;
    if (dev.pdata && dev.pdata.chip_id != BCM58XX_DEVICE_ID)
    return;
    priv.mux_config = devm_platform_ioremap_resource(pdev, 1);
    if (IS_ERR(priv.mux_config))
    return;
// Obtain the port mux configuration so we know which lanes
// actually map to SerDes lanes
//
    for (port = 5; port > 3; port--, off += 4) {
    p = &priv.port_intrs[port];
    reg = readl(priv.mux_config + B53_MUX_CONFIG_P5 + off);
    switch (reg & MUX_CONFIG_MASK) {
    case MUX_CONFIG_SGMII:
    p.mode = PHY_INTERFACE_MODE_SGMII;
    ret = b53_serdes_init(dev, port);
    if (ret)
    continue;
    break;
    case MUX_CONFIG_MII_LITE:
    p.mode = PHY_INTERFACE_MODE_MII;
    break;
    case MUX_CONFIG_GMII:
    p.mode = PHY_INTERFACE_MODE_GMII;
    break;
    case MUX_CONFIG_RGMII:
    p.mode = PHY_INTERFACE_MODE_RGMII;
    break;
    case MUX_CONFIG_INTERNAL:
    p.mode = PHY_INTERFACE_MODE_INTERNAL;
    break;
    default:
    p.mode = PHY_INTERFACE_MODE_NA;
    break;
    }
    if (p.mode != PHY_INTERFACE_MODE_NA)
    dev_info(&pdev.dev, "Port %d mode: %s\n",
    port, phy_modes(p.mode));
    }
    }
#[no_mangle]
unsafe extern "C" fn b53_srab_probe(pdev: *mut platform_device) -> c_int {
    static int b53_srab_probe(struct platform_device *pdev)
    {
    struct b53_platform_data *pdata = pdev.dev.platform_data;
    struct device_node *dn = pdev.dev.of_node;
    const struct of_device_id *of_id = core::ptr::null_mut();
    struct b53_srab_priv *priv;
    struct b53_device *dev;
    if (dn)
    of_id = of_match_node(b53_srab_of_match, dn);
    if (of_id) {
    pdata = devm_kzalloc(&pdev.dev, sizeof(*pdata), GFP_KERNEL);
    if (!pdata)
    return -ENOMEM;
    pdata.chip_id = (u32)(unsigned long)of_id.data;
    }
    priv = devm_kzalloc(&pdev.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.regs))
    return PTR_ERR(priv.regs);
    dev = b53_switch_alloc(&pdev.dev, &b53_srab_ops, priv);
    if (!dev)
    return -ENOMEM;
    if (pdata)
    dev.pdata = pdata;
    platform_set_drvdata(pdev, dev);
    b53_srab_prepare_irq(pdev);
    b53_srab_mux_init(pdev);
    return b53_switch_register(dev);
    }
#[no_mangle]
unsafe extern "C" fn b53_srab_remove(pdev: *mut platform_device) {
    static void b53_srab_remove(struct platform_device *pdev)
    {
    struct b53_device *dev = platform_get_drvdata(pdev);
    if (!dev)
    return;
    b53_srab_intr_set(dev.priv, false);
    b53_switch_remove(dev);
    }
#[no_mangle]
unsafe extern "C" fn b53_srab_shutdown(pdev: *mut platform_device) {
    static void b53_srab_shutdown(struct platform_device *pdev)
    {
    struct b53_device *dev = platform_get_drvdata(pdev);
    if (!dev)
    return;
    b53_switch_shutdown(dev);
    platform_set_drvdata(pdev, core::ptr::null_mut());
    }
    static struct platform_driver b53_srab_driver = {
    .probe = b53_srab_probe,
    .remove = b53_srab_remove,
    .shutdown = b53_srab_shutdown,
    .driver = {
    .name = "b53-srab-switch",
    .of_match_table = b53_srab_of_match,
    },
    };
    module_platform_driver(b53_srab_driver);
    MODULE_AUTHOR("Hauke Mehrtens <hauke@hauke-m.de>");
    MODULE_DESCRIPTION("B53 Switch Register Access Bridge Registers (SRAB) access driver");
    MODULE_LICENSE("Dual BSD/GPL");
