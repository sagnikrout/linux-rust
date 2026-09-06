//! Automatically rewritten from C to Rust
//! Source: drivers/i2c/busses/i2c-bcm2835.c
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
// BCM2835 I2C controller driver
//

pub const BCM2835_I2C_C: c_uint = 0x0;
pub const BCM2835_I2C_S: c_uint = 0x4;
pub const BCM2835_I2C_DLEN: c_uint = 0x8;
pub const BCM2835_I2C_A: c_uint = 0xc;
pub const BCM2835_I2C_FIFO: c_uint = 0x10;
pub const BCM2835_I2C_DIV: c_uint = 0x14;
pub const BCM2835_I2C_DEL: c_uint = 0x18;
//
// 16-bit field for the number of SCL cycles to wait after rising SCL
// before deciding the target is not responding. 0 disables the
// timeout detection.
//
pub const BCM2835_I2C_CLKT: c_uint = 0x1c;

pub const BCM2835_I2C_FEDL_SHIFT: c_int = 16;
pub const BCM2835_I2C_REDL_SHIFT: c_int = 0;
pub const BCM2835_I2C_CDIV_MIN: c_uint = 0x0002;
pub const BCM2835_I2C_CDIV_MAX: c_uint = 0xFFFE;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm2835_i2c_dev {
    pub dev: *mut device,
    pub regs: *mut void __iomem,
    pub irq: c_int,
    pub adapter: i2c_adapter,
    pub completion: completion,
    pub curr_msg: *mut i2c_msg,
    pub bus_clk: *mut clk,
    pub num_msgs: c_int,
    pub msg_err: u32,
    pub msg_buf: *mut u8,
    pub msg_buf_remaining: usize,
}

    static inline void bcm2835_i2c_writel(struct bcm2835_i2c_dev *i2c_dev,
    u32 reg, u32 val)
    {
    writel(val, i2c_dev.regs + reg);
    }
#[no_mangle]
pub unsafe extern "C" fn bcm2835_i2c_readl(i2c_dev: *mut bcm2835_i2c_dev, reg: u32) -> u32 {
    static inline u32 bcm2835_i2c_readl(struct bcm2835_i2c_dev *i2c_dev, u32 reg)
    {
    return readl(i2c_dev.regs + reg);
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_bcm2835_i2c {
    pub hw: clk_hw,
    pub i2c_dev: *mut bcm2835_i2c_dev,
}

    static int clk_bcm2835_i2c_calc_divider(unsigned long rate,
    unsigned long parent_rate)
    {
    let mut divider: u32 = DIV_ROUND_UP(parent_rate, rate);
//
// Per the datasheet, the register is always interpreted as an even
// number, by rounding down. In other words, the LSB is ignored. So,
// if the LSB is set, increment the divider to avoid any issue.
//
    if (divider & 1)
    divider++;
    if ((divider < BCM2835_I2C_CDIV_MIN) ||
    (divider > BCM2835_I2C_CDIV_MAX))
    return -EINVAL;
    return divider;
    }
    static int clk_bcm2835_i2c_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct clk_bcm2835_i2c *div = to_clk_bcm2835_i2c(hw);
    u32 redl, fedl;
    let mut divider: u32 = clk_bcm2835_i2c_calc_divider(rate, parent_rate);
    if (divider == -EINVAL)
    return -EINVAL;
    bcm2835_i2c_writel(div.i2c_dev, BCM2835_I2C_DIV, divider);
//
// Number of core clocks to wait after falling edge before
// outputting the next data bit.  Note that both FEDL and REDL
// can't be greater than CDIV/2.
//
    fedl = max(divider / 16, 1u);
//
// Number of core clocks to wait after rising edge before
// sampling the next incoming data bit.
//
    redl = max(divider / 4, 1u);
    bcm2835_i2c_writel(div.i2c_dev, BCM2835_I2C_DEL,
    (fedl << BCM2835_I2C_FEDL_SHIFT) |
    (redl << BCM2835_I2C_REDL_SHIFT));
    return 0;
    }
    static int clk_bcm2835_i2c_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    let mut divider: u32 = clk_bcm2835_i2c_calc_divider(req.rate, req.best_parent_rate);
    req.rate = DIV_ROUND_UP(req.best_parent_rate, divider);
    return 0;
    }
    static unsigned long clk_bcm2835_i2c_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct clk_bcm2835_i2c *div = to_clk_bcm2835_i2c(hw);
    let mut divider: u32 = bcm2835_i2c_readl(div.i2c_dev, BCM2835_I2C_DIV);
    return DIV_ROUND_UP(parent_rate, divider);
    }
    static const struct clk_ops clk_bcm2835_i2c_ops = {
    .set_rate = clk_bcm2835_i2c_set_rate,
    .determine_rate = clk_bcm2835_i2c_determine_rate,
    .recalc_rate = clk_bcm2835_i2c_recalc_rate,
    };
    static struct clk *bcm2835_i2c_register_div(struct device *dev,
    struct clk *mclk,
    struct bcm2835_i2c_dev *i2c_dev)
    {
    struct clk_init_data init;
    struct clk_bcm2835_i2c *priv;
    char name[32];
    const char *mclk_name;
    snprintf(name, sizeof(name), "%s_div", dev_name(dev));
    mclk_name = __clk_get_name(mclk);
    init.ops = &clk_bcm2835_i2c_ops;
    init.name = name;
    init.parent_names = (const char* []) { mclk_name };
    init.num_parents = 1;
    init.flags = 0;
    priv = devm_kzalloc(dev, sizeof(struct clk_bcm2835_i2c), GFP_KERNEL);
    if (priv == core::ptr::null_mut())
    return ERR_PTR(-ENOMEM);
    priv.hw.init = &init;
    priv.i2c_dev = i2c_dev;
    clk_hw_register_clkdev(&priv.hw, "div", dev_name(dev));
    return devm_clk_register(dev, &priv.hw);
    }
#[no_mangle]
unsafe extern "C" fn bcm2835_fill_txfifo(i2c_dev: *mut bcm2835_i2c_dev) {
    static void bcm2835_fill_txfifo(struct bcm2835_i2c_dev *i2c_dev)
    {
    u32 val;
    while (i2c_dev.msg_buf_remaining) {
    val = bcm2835_i2c_readl(i2c_dev, BCM2835_I2C_S);
    if (!(val & BCM2835_I2C_S_TXD))
    break;
    bcm2835_i2c_writel(i2c_dev, BCM2835_I2C_FIFO,
// i2c_dev->msg_buf);
    i2c_dev.msg_buf++;
    i2c_dev.msg_buf_remaining--;
    }
    }
#[no_mangle]
unsafe extern "C" fn bcm2835_drain_rxfifo(i2c_dev: *mut bcm2835_i2c_dev) {
    static void bcm2835_drain_rxfifo(struct bcm2835_i2c_dev *i2c_dev)
    {
    u32 val;
    while (i2c_dev.msg_buf_remaining) {
    val = bcm2835_i2c_readl(i2c_dev, BCM2835_I2C_S);
    if (!(val & BCM2835_I2C_S_RXD))
    break;
// i2c_dev->msg_buf = bcm2835_i2c_readl(i2c_dev,
    BCM2835_I2C_FIFO);
    i2c_dev.msg_buf++;
    i2c_dev.msg_buf_remaining--;
    }
    }
//
// Repeated Start Condition (Sr)
// The BCM2835 ARM Peripherals datasheet mentions a way to trigger a Sr when it
// talks about reading from a target with 10 bit address. This is achieved by
// issuing a write, poll the I2CS.TA flag and wait for it to be set, and then
// issue a read.
// A comment in https://github.com/raspberrypi/linux/issues/254 shows how the
// firmware actually does it using polling and says that it's a workaround for
// a problem in the state machine.
// It turns out that it is possible to use the TXW interrupt to know when the
// transfer is active, provided the FIFO has not been prefilled.
//
#[no_mangle]
unsafe extern "C" fn bcm2835_i2c_start_transfer(i2c_dev: *mut bcm2835_i2c_dev) {
    static void bcm2835_i2c_start_transfer(struct bcm2835_i2c_dev *i2c_dev)
    {
    let mut c: u32 = BCM2835_I2C_C_ST | BCM2835_I2C_C_I2CEN;
    struct i2c_msg *msg = i2c_dev.curr_msg;
    let mut last_msg: bool = (i2c_dev.num_msgs == 1);
    if (!i2c_dev.num_msgs)
    return;
    i2c_dev.num_msgs--;
    i2c_dev.msg_buf = msg.buf;
    i2c_dev.msg_buf_remaining = msg.len;
    if (msg.flags & I2C_M_RD)
    c |= BCM2835_I2C_C_READ | BCM2835_I2C_C_INTR;
    else
    c |= BCM2835_I2C_C_INTT;
    if (last_msg)
    c |= BCM2835_I2C_C_INTD;
    bcm2835_i2c_writel(i2c_dev, BCM2835_I2C_A, msg.addr);
    bcm2835_i2c_writel(i2c_dev, BCM2835_I2C_DLEN, msg.len);
    bcm2835_i2c_writel(i2c_dev, BCM2835_I2C_C, c);
    }
#[no_mangle]
unsafe extern "C" fn bcm2835_i2c_finish_transfer(i2c_dev: *mut bcm2835_i2c_dev) {
    static void bcm2835_i2c_finish_transfer(struct bcm2835_i2c_dev *i2c_dev)
    {
    i2c_dev.curr_msg = core::ptr::null_mut();
    i2c_dev.num_msgs = 0;
    i2c_dev.msg_buf = core::ptr::null_mut();
    i2c_dev.msg_buf_remaining = 0;
    }
//
// Note about I2C_C_CLEAR on error:
// The I2C_C_CLEAR on errors will take some time to resolve -- if you were in
// non-idle state and I2C_C_READ, it sets an abort_rx flag and runs through
// the state machine to send a NACK and a STOP. Since we're setting CLEAR
// without I2CEN, that NACK will be hanging around queued up for next time
// we start the engine.
//
#[no_mangle]
unsafe extern "C" fn bcm2835_i2c_isr(this_irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t bcm2835_i2c_isr(int this_irq, void *data)
    {
    struct bcm2835_i2c_dev *i2c_dev = data;
    u32 val, err;
    val = bcm2835_i2c_readl(i2c_dev, BCM2835_I2C_S);
    err = val & (BCM2835_I2C_S_CLKT | BCM2835_I2C_S_ERR);
    if (err) {
    i2c_dev.msg_err = err;
    goto complete;
    }
    if (val & BCM2835_I2C_S_DONE) {
    if (!i2c_dev.curr_msg) {
    dev_err(i2c_dev.dev, "Got unexpected interrupt (from firmware?)\n");
    } else if (i2c_dev.curr_msg.flags & I2C_M_RD) {
    bcm2835_drain_rxfifo(i2c_dev);
    val = bcm2835_i2c_readl(i2c_dev, BCM2835_I2C_S);
    }
    if ((val & BCM2835_I2C_S_RXD) || i2c_dev.msg_buf_remaining)
    i2c_dev.msg_err = BCM2835_I2C_S_LEN;
    else
    i2c_dev.msg_err = 0;
    goto complete;
    }
    if (val & BCM2835_I2C_S_TXW) {
    if (!i2c_dev.msg_buf_remaining) {
    i2c_dev.msg_err = val | BCM2835_I2C_S_LEN;
    goto complete;
    }
    bcm2835_fill_txfifo(i2c_dev);
    if (i2c_dev.num_msgs && !i2c_dev.msg_buf_remaining) {
    i2c_dev.curr_msg++;
    bcm2835_i2c_start_transfer(i2c_dev);
    }
    return IRQ_HANDLED;
    }
    if (val & BCM2835_I2C_S_RXR) {
    if (!i2c_dev.msg_buf_remaining) {
    i2c_dev.msg_err = val | BCM2835_I2C_S_LEN;
    goto complete;
    }
    bcm2835_drain_rxfifo(i2c_dev);
    return IRQ_HANDLED;
    }
    return IRQ_NONE;
    complete:
    bcm2835_i2c_writel(i2c_dev, BCM2835_I2C_C, BCM2835_I2C_C_CLEAR);
    bcm2835_i2c_writel(i2c_dev, BCM2835_I2C_S, BCM2835_I2C_S_CLKT |
    BCM2835_I2C_S_ERR | BCM2835_I2C_S_DONE);
    complete(&i2c_dev.completion);
    return IRQ_HANDLED;
    }
    static int bcm2835_i2c_xfer(struct i2c_adapter *adap, struct i2c_msg msgs[],
    int num)
    {
    struct bcm2835_i2c_dev *i2c_dev = i2c_get_adapdata(adap);
    unsigned long time_left;
    int i;
    for (i = 0; i < (num - 1); i++)
    if (msgs[i].flags & I2C_M_RD) {
    dev_warn_once(i2c_dev.dev,
    "only one read message supported, has to be last\n");
    return -EOPNOTSUPP;
    }
    i2c_dev.curr_msg = msgs;
    i2c_dev.num_msgs = num;
    reinit_completion(&i2c_dev.completion);
    bcm2835_i2c_start_transfer(i2c_dev);
    time_left = wait_for_completion_timeout(&i2c_dev.completion,
    adap.timeout);
    bcm2835_i2c_finish_transfer(i2c_dev);
    if (!time_left) {
    bcm2835_i2c_writel(i2c_dev, BCM2835_I2C_C,
    BCM2835_I2C_C_CLEAR);
    return -ETIMEDOUT;
    }
    if (!i2c_dev.msg_err)
    return num;
    dev_dbg(i2c_dev.dev, "i2c transfer failed: %x\n", i2c_dev.msg_err);
    if (i2c_dev.msg_err & BCM2835_I2C_S_ERR)
    return -EREMOTEIO;
    return -EIO;
    }
#[no_mangle]
unsafe extern "C" fn bcm2835_i2c_func(adap: *mut i2c_adapter) -> u32 {
    static u32 bcm2835_i2c_func(struct i2c_adapter *adap)
    {
    return I2C_FUNC_I2C | I2C_FUNC_SMBUS_EMUL;
    }
    static const struct i2c_algorithm bcm2835_i2c_algo = {
    .xfer = bcm2835_i2c_xfer,
    .functionality = bcm2835_i2c_func,
    };
//
// The BCM2835 was reported to have problems with clock stretching:
// https://www.advamation.com/knowhow/raspberrypi/rpi-i2c-bug.html
// https://www.raspberrypi.org/forums/viewtopic.php?p=146272
//
    static const struct i2c_adapter_quirks bcm2835_i2c_quirks = {
    .flags = I2C_AQ_NO_CLK_STRETCH,
    };
#[no_mangle]
unsafe extern "C" fn bcm2835_i2c_probe(pdev: *mut platform_device) -> c_int {
    static int bcm2835_i2c_probe(struct platform_device *pdev)
    {
    struct bcm2835_i2c_dev *i2c_dev;
    int ret;
    struct i2c_adapter *adap;
    struct clk *mclk;
    u32 bus_clk_rate;
    i2c_dev = devm_kzalloc(&pdev.dev, sizeof(*i2c_dev), GFP_KERNEL);
    if (!i2c_dev)
    return -ENOMEM;
    platform_set_drvdata(pdev, i2c_dev);
    i2c_dev.dev = &pdev.dev;
    init_completion(&i2c_dev.completion);
    i2c_dev.regs = devm_platform_get_and_ioremap_resource(pdev, 0, core::ptr::null_mut());
    if (IS_ERR(i2c_dev.regs))
    return PTR_ERR(i2c_dev.regs);
    mclk = devm_clk_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(mclk))
    return dev_err_probe(&pdev.dev, PTR_ERR(mclk),
    "Could not get clock\n");
    i2c_dev.bus_clk = bcm2835_i2c_register_div(&pdev.dev, mclk, i2c_dev);
    if (IS_ERR(i2c_dev.bus_clk))
    return dev_err_probe(&pdev.dev, PTR_ERR(i2c_dev.bus_clk),
    "Could not register clock\n");
    ret = of_property_read_u32(pdev.dev.of_node, "clock-frequency",
    &bus_clk_rate);
    if (ret < 0) {
    dev_warn(&pdev.dev,
    "Could not read clock-frequency property\n");
    bus_clk_rate = I2C_MAX_STANDARD_MODE_FREQ;
    }
    ret = clk_set_rate_exclusive(i2c_dev.bus_clk, bus_clk_rate);
    if (ret < 0)
    return dev_err_probe(&pdev.dev, ret,
    "Could not set clock frequency\n");
    ret = clk_prepare_enable(i2c_dev.bus_clk);
    if (ret) {
    dev_err(&pdev.dev, "Couldn't prepare clock");
    goto err_put_exclusive_rate;
    }
    i2c_dev.irq = platform_get_irq(pdev, 0);
    if (i2c_dev.irq < 0) {
    ret = i2c_dev.irq;
    goto err_disable_unprepare_clk;
    }
    ret = request_irq(i2c_dev.irq, bcm2835_i2c_isr, IRQF_SHARED,
    dev_name(&pdev.dev), i2c_dev);
    if (ret) {
    dev_err(&pdev.dev, "Could not request IRQ\n");
    goto err_disable_unprepare_clk;
    }
    adap = &i2c_dev.adapter;
    i2c_set_adapdata(adap, i2c_dev);
    adap.owner = THIS_MODULE;
    adap.class = I2C_CLASS_DEPRECATED;
    snprintf(adap.name, sizeof(adap.name), "bcm2835 (%s)",
    of_node_full_name(pdev.dev.of_node));
    adap.algo = &bcm2835_i2c_algo;
    adap.dev.parent = &pdev.dev;
    adap.dev.of_node = pdev.dev.of_node;
    adap.quirks = of_device_get_match_data(&pdev.dev);
//
// Disable the hardware clock stretching timeout. SMBUS
// specifies a limit for how long the device can stretch the
// clock, but core I2C doesn't.
//
    bcm2835_i2c_writel(i2c_dev, BCM2835_I2C_CLKT, 0);
    bcm2835_i2c_writel(i2c_dev, BCM2835_I2C_C, 0);
    ret = i2c_add_adapter(adap);
    if (ret)
    goto err_free_irq;
    return 0;
    err_free_irq:
    free_irq(i2c_dev.irq, i2c_dev);
    err_disable_unprepare_clk:
    clk_disable_unprepare(i2c_dev.bus_clk);
    err_put_exclusive_rate:
    clk_rate_exclusive_put(i2c_dev.bus_clk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn bcm2835_i2c_remove(pdev: *mut platform_device) {
    static void bcm2835_i2c_remove(struct platform_device *pdev)
    {
    struct bcm2835_i2c_dev *i2c_dev = platform_get_drvdata(pdev);
    clk_rate_exclusive_put(i2c_dev.bus_clk);
    clk_disable_unprepare(i2c_dev.bus_clk);
    free_irq(i2c_dev.irq, i2c_dev);
    i2c_del_adapter(&i2c_dev.adapter);
    }
    static const struct of_device_id bcm2835_i2c_of_match[] = {
    { .compatible = "brcm,bcm2711-i2c" },
    { .compatible = "brcm,bcm2835-i2c", .data = &bcm2835_i2c_quirks },
    {},
    };
    MODULE_DEVICE_TABLE(of, bcm2835_i2c_of_match);
    static struct platform_driver bcm2835_i2c_driver = {
    .probe		= bcm2835_i2c_probe,
    .remove		= bcm2835_i2c_remove,
    .driver		= {
    .name	= "i2c-bcm2835",
    .of_match_table = bcm2835_i2c_of_match,
    },
    };
    module_platform_driver(bcm2835_i2c_driver);
    MODULE_AUTHOR("Stephen Warren <swarren@wwwdotorg.org>");
    MODULE_DESCRIPTION("BCM2835 I2C bus adapter");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:i2c-bcm2835");
