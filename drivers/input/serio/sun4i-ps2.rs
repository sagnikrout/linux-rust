//! Automatically rewritten from C to Rust
//! Source: drivers/input/serio/sun4i-ps2.c
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
// Driver for Allwinner A10 PS2 host controller
//
// Author: Vishnu Patekar <vishnupatekar0510@gmail.com>
// Aaron.maoye <leafy.myeh@newbietech.com>
//

// register offset definitions
pub const PS2_REG_GCTL: c_uint = 0x00	/* PS2 Module Global Control Reg */;
pub const PS2_REG_DATA: c_uint = 0x04	/* PS2 Module Data Reg		*/;
pub const PS2_REG_LCTL: c_uint = 0x08	/* PS2 Module Line Control Reg */;
pub const PS2_REG_LSTS: c_uint = 0x0C	/* PS2 Module Line Status Reg	*/;
pub const PS2_REG_FCTL: c_uint = 0x10	/* PS2 Module FIFO Control Reg */;
pub const PS2_REG_FSTS: c_uint = 0x14	/* PS2 Module FIFO Status Reg	*/;
pub const PS2_REG_CLKDR: c_uint = 0x18	/* PS2 Module Clock Divider Reg*/;
// PS2 GLOBAL CONTROL REGISTER PS2_GCTL

// PS2 LINE CONTROL REGISTER

// PS2 LINE STATUS REGISTER

    (PS2_LSTS_TXTDO | PS2_LSTS_STOPERR | PS2_LSTS_ACKERR | \
    PS2_LSTS_PARERR | PS2_LSTS_RXTDO)
// PS2 FIFO CONTROL REGISTER

// PS2 FIFO STATUS REGISTER

    (PS2_FSTS_TXUF | PS2_FSTS_TXOF | PS2_FSTS_RXUF | PS2_FSTS_RXOF)
pub const PS2_SAMPLE_CLK: c_int = 1000000;
pub const PS2_SCLK: c_int = 125000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun4i_ps2data {
    pub serio: *mut serio,
    pub dev: *mut device,
// IO mapping base
    pub reg_base: *mut void __iomem,
// clock management
    pub clk: *mut clk,
// irq
    pub lock: spinlock_t,
    pub irq: c_int,
}

#[no_mangle]
unsafe extern "C" fn sun4i_ps2_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t sun4i_ps2_interrupt(int irq, void *dev_id)
    {
    struct sun4i_ps2data *drvdata = dev_id;
    u32 intr_status;
    u32 fifo_status;
    unsigned char byte;
    let mut rxflags: c_uint = 0;
    u32 rval;
    guard(spinlock)(&drvdata.lock);
// Get the PS/2 interrupts and clear them
    intr_status  = readl(drvdata.reg_base + PS2_REG_LSTS);
    fifo_status  = readl(drvdata.reg_base + PS2_REG_FSTS);
// Check line status register
    if (intr_status & PS2_LINE_ERROR_BIT) {
    rxflags = (intr_status & PS2_LINE_ERROR_BIT) ? SERIO_FRAME : 0;
    rxflags |= (intr_status & PS2_LSTS_PARERR) ? SERIO_PARITY : 0;
    rxflags |= (intr_status & PS2_LSTS_PARERR) ? SERIO_TIMEOUT : 0;
    rval = PS2_LSTS_TXTDO | PS2_LSTS_STOPERR | PS2_LSTS_ACKERR |
    PS2_LSTS_PARERR | PS2_LSTS_RXTDO;
    writel(rval, drvdata.reg_base + PS2_REG_LSTS);
    }
// Check FIFO status register
    if (fifo_status & PS2_FIFO_ERROR_BIT) {
    rval = PS2_FSTS_TXUF | PS2_FSTS_TXOF | PS2_FSTS_TXRDY |
    PS2_FSTS_RXUF | PS2_FSTS_RXOF | PS2_FSTS_RXRDY;
    writel(rval, drvdata.reg_base + PS2_REG_FSTS);
    }
    rval = (fifo_status >> 16) & 0x3;
    while (rval--) {
    byte = readl(drvdata.reg_base + PS2_REG_DATA) & 0xff;
    serio_interrupt(drvdata.serio, byte, rxflags);
    }
    writel(intr_status, drvdata.reg_base + PS2_REG_LSTS);
    writel(fifo_status, drvdata.reg_base + PS2_REG_FSTS);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn sun4i_ps2_open(serio: *mut serio) -> c_int {
    static int sun4i_ps2_open(struct serio *serio)
    {
    struct sun4i_ps2data *drvdata = serio.port_data;
    let mut src_clk: u32 = 0;
    u32 clk_scdf;
    u32 clk_pcdf;
    u32 rval;
// Set line control and enable interrupt
    rval = PS2_LCTL_STOPERREN | PS2_LCTL_ACKERREN
    | PS2_LCTL_PARERREN | PS2_LCTL_RXDTOEN;
    writel(rval, drvdata.reg_base + PS2_REG_LCTL);
// Reset FIFO
    rval = PS2_FCTL_TXRST | PS2_FCTL_RXRST | PS2_FCTL_TXUFIEN
    | PS2_FCTL_TXOFIEN | PS2_FCTL_RXUFIEN
    | PS2_FCTL_RXOFIEN | PS2_FCTL_RXRDYIEN;
    writel(rval, drvdata.reg_base + PS2_REG_FCTL);
    src_clk = clk_get_rate(drvdata.clk);
// Set clock divider register
    clk_scdf = src_clk / PS2_SAMPLE_CLK - 1;
    clk_pcdf = PS2_SAMPLE_CLK / PS2_SCLK - 1;
    rval = (clk_scdf << 8) | clk_pcdf;
    writel(rval, drvdata.reg_base + PS2_REG_CLKDR);
// Set global control register
    rval = PS2_GCTL_RESET | PS2_GCTL_INTEN | PS2_GCTL_MASTER
    | PS2_GCTL_BUSEN;
    guard(spinlock_irqsave)(&drvdata.lock);
    writel(rval, drvdata.reg_base + PS2_REG_GCTL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sun4i_ps2_close(serio: *mut serio) {
    static void sun4i_ps2_close(struct serio *serio)
    {
    struct sun4i_ps2data *drvdata = serio.port_data;
    u32 rval;
// Shut off the interrupt
    rval = readl(drvdata.reg_base + PS2_REG_GCTL);
    writel(rval & ~(PS2_GCTL_INTEN), drvdata.reg_base + PS2_REG_GCTL);
    synchronize_irq(drvdata.irq);
    }
#[no_mangle]
unsafe extern "C" fn sun4i_ps2_write(serio: *mut serio, val: c_uchar) -> c_int {
    static int sun4i_ps2_write(struct serio *serio, unsigned char val)
    {
    let mut expire: c_ulong = jiffies + msecs_to_jiffies(10000);
    struct sun4i_ps2data *drvdata = serio.port_data;
    do {
    if (readl(drvdata.reg_base + PS2_REG_FSTS) & PS2_FSTS_TXRDY) {
    writel(val, drvdata.reg_base + PS2_REG_DATA);
    return 0;
    }
    } while (time_before(jiffies, expire));
    return SERIO_TIMEOUT;
    }
#[no_mangle]
unsafe extern "C" fn sun4i_ps2_probe(pdev: *mut platform_device) -> c_int {
    static int sun4i_ps2_probe(struct platform_device *pdev)
    {
    struct resource *res; /* IO mem resources */
    struct sun4i_ps2data *drvdata;
    struct serio *serio;
    struct device *dev = &pdev.dev;
    int error;
    drvdata = kzalloc_obj(*drvdata);
    serio = kzalloc_obj(*serio);
    if (!drvdata || !serio) {
    error = -ENOMEM;
    goto err_free_mem;
    }
    spin_lock_init(&drvdata.lock);
// IO
    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!res) {
    dev_err(dev, "failed to locate registers\n");
    error = -ENXIO;
    goto err_free_mem;
    }
    drvdata.reg_base = ioremap(res.start, resource_size(res));
    if (!drvdata.reg_base) {
    dev_err(dev, "failed to map registers\n");
    error = -ENOMEM;
    goto err_free_mem;
    }
    drvdata.clk = clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(drvdata.clk)) {
    error = PTR_ERR(drvdata.clk);
    dev_err(dev, "couldn't get clock %d\n", error);
    goto err_ioremap;
    }
    error = clk_prepare_enable(drvdata.clk);
    if (error) {
    dev_err(dev, "failed to enable clock %d\n", error);
    goto err_clk;
    }
    serio.id.type = SERIO_8042;
    serio.write = sun4i_ps2_write;
    serio.open = sun4i_ps2_open;
    serio.close = sun4i_ps2_close;
    serio.port_data = drvdata;
    serio.dev.parent = dev;
    strscpy(serio.name, dev_name(dev), sizeof(serio.name));
    strscpy(serio.phys, dev_name(dev), sizeof(serio.phys));
// shutoff interrupt
    writel(0, drvdata.reg_base + PS2_REG_GCTL);
// Get IRQ for the device
    drvdata.irq = platform_get_irq(pdev, 0);
    if (drvdata.irq < 0) {
    error = drvdata.irq;
    goto err_disable_clk;
    }
    drvdata.serio = serio;
    drvdata.dev = dev;
    error = request_irq(drvdata.irq, sun4i_ps2_interrupt, 0,
    DRIVER_NAME, drvdata);
    if (error) {
    dev_err(drvdata.dev, "failed to allocate interrupt %d: %d\n",
    drvdata.irq, error);
    goto err_disable_clk;
    }
    serio_register_port(serio);
    platform_set_drvdata(pdev, drvdata);
    return 0;	/* success */
    err_disable_clk:
    clk_disable_unprepare(drvdata.clk);
    err_clk:
    clk_put(drvdata.clk);
    err_ioremap:
    iounmap(drvdata.reg_base);
    err_free_mem:
    kfree(serio);
    kfree(drvdata);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn sun4i_ps2_remove(pdev: *mut platform_device) {
    static void sun4i_ps2_remove(struct platform_device *pdev)
    {
    struct sun4i_ps2data *drvdata = platform_get_drvdata(pdev);
    serio_unregister_port(drvdata.serio);
    free_irq(drvdata.irq, drvdata);
    clk_disable_unprepare(drvdata.clk);
    clk_put(drvdata.clk);
    iounmap(drvdata.reg_base);
    kfree(drvdata);
    }
    static const struct of_device_id sun4i_ps2_match[] = {
    { .compatible = "allwinner,sun4i-a10-ps2", },
    { },
    };
    MODULE_DEVICE_TABLE(of, sun4i_ps2_match);
    static struct platform_driver sun4i_ps2_driver = {
    .probe		= sun4i_ps2_probe,
    .remove		= sun4i_ps2_remove,
    .driver = {
    .name = DRIVER_NAME,
    .of_match_table = sun4i_ps2_match,
    },
    };
    module_platform_driver(sun4i_ps2_driver);
    MODULE_AUTHOR("Vishnu Patekar <vishnupatekar0510@gmail.com>");
    MODULE_AUTHOR("Aaron.maoye <leafy.myeh@newbietech.com>");
    MODULE_DESCRIPTION("Allwinner A10/Sun4i PS/2 driver");
    MODULE_LICENSE("GPL v2");
