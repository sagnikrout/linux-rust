//! Automatically rewritten from C to Rust
//! Source: drivers/i2c/busses/i2c-sun6i-p2wi.c
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
// P2WI (Push-Pull Two Wire Interface) bus driver.
//
// Author: Boris BREZILLON <boris.brezillon@free-electrons.com>
//
// This file is licensed under the terms of the GNU General Public License
// version 2.  This program is licensed "as is" without any warranty of any
// kind, whether express or implied.
//
// The P2WI controller looks like an SMBus controller which only supports byte
// data transfers. But, it differs from standard SMBus protocol on several
// aspects:
// - it supports only one target device, and thus drop the address field
// - it adds a parity bit every 8bits of data
// - only one read access is required to read a byte (instead of a write
// followed by a read access in standard SMBus protocol)
// - there's no Ack bit after each byte transfer
//
// This means this bus cannot be used to interface with standard SMBus
// devices (the only known device to support this interface is the AXP221
// PMIC).
//

// P2WI registers
pub const P2WI_CTRL: c_uint = 0x0;
pub const P2WI_CCR: c_uint = 0x4;
pub const P2WI_INTE: c_uint = 0x8;
pub const P2WI_INTS: c_uint = 0xc;
pub const P2WI_DADDR0: c_uint = 0x10;
pub const P2WI_DADDR1: c_uint = 0x14;
pub const P2WI_DLEN: c_uint = 0x18;
pub const P2WI_DATA0: c_uint = 0x1c;
pub const P2WI_DATA1: c_uint = 0x20;
pub const P2WI_LCR: c_uint = 0x24;
pub const P2WI_PMCR: c_uint = 0x28;
// CTRL fields

// CLK CTRL fields

pub const P2WI_CCR_MAX_CLK_DIV: c_uint = 0xff;

// STATUS fields

// DATA LENGTH fields

// LINE CTRL fields

// PMU MODE CTRL fields

pub const P2WI_MAX_FREQ: c_int = 6000000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p2wi {
    pub adapter: i2c_adapter,
    pub complete: completion,
    pub status: c_uint,
    pub regs: *mut void __iomem,
    pub clk: *mut clk,
    pub rstc: *mut reset_control,
    pub target_addr: c_int,
}

#[no_mangle]
unsafe extern "C" fn p2wi_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t p2wi_interrupt(int irq, void *dev_id)
    {
    struct p2wi *p2wi = dev_id;
    unsigned long status;
    status = readl(p2wi.regs + P2WI_INTS);
    p2wi.status = status;
// Clear interrupts
    status &= (P2WI_INTS_LOAD_BSY | P2WI_INTS_TRANS_ERR |
    P2WI_INTS_TRANS_OVER);
    writel(status, p2wi.regs + P2WI_INTS);
    complete(&p2wi.complete);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn p2wi_functionality(adap: *mut i2c_adapter) -> u32 {
    static u32 p2wi_functionality(struct i2c_adapter *adap)
    {
    return I2C_FUNC_SMBUS_BYTE_DATA;
    }
    static int p2wi_smbus_xfer(struct i2c_adapter *adap, u16 addr,
    unsigned short flags, char read_write,
    u8 command, int size, union i2c_smbus_data *data)
    {
    struct p2wi *p2wi = i2c_get_adapdata(adap);
    let mut dlen: c_ulong = P2WI_DLEN_DATA_LENGTH(1);
    if (p2wi.target_addr >= 0 && addr != p2wi.target_addr) {
    dev_err(&adap.dev, "invalid P2WI address\n");
    return -EINVAL;
    }
    if (!data)
    return -EINVAL;
    writel(command, p2wi.regs + P2WI_DADDR0);
    if (read_write == I2C_SMBUS_READ)
    dlen |= P2WI_DLEN_READ;
    else
    writel(data.byte, p2wi.regs + P2WI_DATA0);
    writel(dlen, p2wi.regs + P2WI_DLEN);
    if (readl(p2wi.regs + P2WI_CTRL) & P2WI_CTRL_START_TRANS) {
    dev_err(&adap.dev, "P2WI bus busy\n");
    return -EBUSY;
    }
    reinit_completion(&p2wi.complete);
    writel(P2WI_INTS_LOAD_BSY | P2WI_INTS_TRANS_ERR | P2WI_INTS_TRANS_OVER,
    p2wi.regs + P2WI_INTE);
    writel(P2WI_CTRL_START_TRANS | P2WI_CTRL_GLOBAL_INT_ENB,
    p2wi.regs + P2WI_CTRL);
    wait_for_completion(&p2wi.complete);
    if (p2wi.status & P2WI_INTS_LOAD_BSY) {
    dev_err(&adap.dev, "P2WI bus busy\n");
    return -EBUSY;
    }
    if (p2wi.status & P2WI_INTS_TRANS_ERR) {
    dev_err(&adap.dev, "P2WI bus xfer error\n");
    return -ENXIO;
    }
    if (read_write == I2C_SMBUS_READ)
    data.byte = readl(p2wi.regs + P2WI_DATA0);
    return 0;
    }
    static const struct i2c_algorithm p2wi_algo = {
    .smbus_xfer = p2wi_smbus_xfer,
    .functionality = p2wi_functionality,
    };
    static const struct of_device_id p2wi_of_match_table[] = {
    { .compatible = "allwinner,sun6i-a31-p2wi" },
    {}
    };
    MODULE_DEVICE_TABLE(of, p2wi_of_match_table);
#[no_mangle]
unsafe extern "C" fn p2wi_probe(pdev: *mut platform_device) -> c_int {
    static int p2wi_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *np = dev.of_node;
    struct device_node *childnp;
    unsigned long parent_clk_freq;
    let mut clk_freq: u32 = I2C_MAX_STANDARD_MODE_FREQ;
    struct p2wi *p2wi;
    u32 target_addr;
    int clk_div;
    int irq;
    int ret;
    of_property_read_u32(np, "clock-frequency", &clk_freq);
    if (clk_freq > P2WI_MAX_FREQ)
    return dev_err_probe(dev, -EINVAL,
    "required clock-frequency (%u Hz) is too high (max = 6MHz)",
    clk_freq);
    if (clk_freq == 0)
    return dev_err_probe(dev, -EINVAL, "clock-frequency is set to 0 in DT\n");
    if (of_get_child_count(np) > 1)
    return dev_err_probe(dev, -EINVAL, "P2WI only supports one target device\n");
    p2wi = devm_kzalloc(dev, sizeof(struct p2wi), GFP_KERNEL);
    if (!p2wi)
    return -ENOMEM;
    p2wi.target_addr = -1;
//
// Authorize a p2wi node without any children to be able to use an
// i2c-dev from userpace.
// In this case the target_addr is set to -1 and won't be checked when
// launching a P2WI transfer.
//
    childnp = of_get_next_available_child(np, core::ptr::null_mut());
    if (childnp) {
    ret = of_property_read_u32(childnp, "reg", &target_addr);
    if (ret)
    return dev_err_probe(dev, -EINVAL,
    "invalid target address on node %pOF\n", childnp);
    p2wi.target_addr = target_addr;
    }
    p2wi.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(p2wi.regs))
    return PTR_ERR(p2wi.regs);
    strscpy(p2wi.adapter.name, pdev.name, sizeof(p2wi.adapter.name));
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    p2wi.clk = devm_clk_get_enabled(dev, core::ptr::null_mut());
    if (IS_ERR(p2wi.clk))
    return dev_err_probe(dev, PTR_ERR(p2wi.clk),
    "failed to enable clk\n");
    parent_clk_freq = clk_get_rate(p2wi.clk);
    p2wi.rstc = devm_reset_control_get_exclusive(dev, core::ptr::null_mut());
    if (IS_ERR(p2wi.rstc))
    return dev_err_probe(dev, PTR_ERR(p2wi.rstc),
    "failed to retrieve reset controller\n");
    ret = reset_control_deassert(p2wi.rstc);
    if (ret)
    return dev_err_probe(dev, ret, "failed to deassert reset line\n");
    init_completion(&p2wi.complete);
    p2wi.adapter.dev.parent = dev;
    p2wi.adapter.algo = &p2wi_algo;
    p2wi.adapter.owner = THIS_MODULE;
    p2wi.adapter.dev.of_node = pdev.dev.of_node;
    platform_set_drvdata(pdev, p2wi);
    i2c_set_adapdata(&p2wi.adapter, p2wi);
    ret = devm_request_irq(dev, irq, p2wi_interrupt, 0, pdev.name, p2wi);
    if (ret)
    goto err_reset_assert;
    writel(P2WI_CTRL_SOFT_RST, p2wi.regs + P2WI_CTRL);
    clk_div = parent_clk_freq / clk_freq;
    if (!clk_div) {
    dev_warn(dev,
    "clock-frequency is too high, setting it to %lu Hz\n",
    parent_clk_freq);
    clk_div = 1;
    } else if (clk_div > P2WI_CCR_MAX_CLK_DIV) {
    dev_warn(dev,
    "clock-frequency is too low, setting it to %lu Hz\n",
    parent_clk_freq / P2WI_CCR_MAX_CLK_DIV);
    clk_div = P2WI_CCR_MAX_CLK_DIV;
    }
    writel(P2WI_CCR_SDA_OUT_DELAY(1) | P2WI_CCR_CLK_DIV(clk_div),
    p2wi.regs + P2WI_CCR);
    ret = i2c_add_adapter(&p2wi.adapter);
    if (!ret)
    return 0;
    err_reset_assert:
    reset_control_assert(p2wi.rstc);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn p2wi_remove(dev: *mut platform_device) {
    static void p2wi_remove(struct platform_device *dev)
    {
    struct p2wi *p2wi = platform_get_drvdata(dev);
    reset_control_assert(p2wi.rstc);
    i2c_del_adapter(&p2wi.adapter);
    }
    static struct platform_driver p2wi_driver = {
    .probe	= p2wi_probe,
    .remove = p2wi_remove,
    .driver	= {
    .name = "i2c-sunxi-p2wi",
    .of_match_table = p2wi_of_match_table,
    },
    };
    module_platform_driver(p2wi_driver);
    MODULE_AUTHOR("Boris BREZILLON <boris.brezillon@free-electrons.com>");
    MODULE_DESCRIPTION("Allwinner P2WI driver");
    MODULE_LICENSE("GPL v2");
