//! Automatically rewritten from C to Rust
//! Source: drivers/i2c/busses/i2c-microchip-corei2c.c
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
// Microchip CoreI2C I2C controller driver
//
// Copyright (c) 2018-2022 Microchip Corporation. All rights reserved.
//
// Author: Daire McNamara <daire.mcnamara@microchip.com>
// Author: Conor Dooley <conor.dooley@microchip.com>
//

//
// struct mchp_corei2c_dev - Microchip CoreI2C device private data
//
// @base:		pointer to register struct
// @dev:		device reference
// @i2c_clk:		clock reference for i2c input clock
// @msg_queue:		pointer to the messages requiring sending
// @buf:		pointer to msg buffer for easier use
// @msg_complete:	xfer completion object
// @adapter:		core i2c abstraction
// @msg_err:		error code for completed message
// @bus_clk_rate:	current i2c bus clock rate
// @isr_status:		cached copy of local ISR status
// @total_num:		total number of messages to be sent/received
// @current_num:	index of the current message being sent/received
// @msg_len:		number of bytes transferred in msg
// @addr:		address of the current slave
// @restart_needed:	whether or not a repeated start is required after current message
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mchp_corei2c_dev {
    pub base: *mut void __iomem,
    pub dev: *mut device,
    pub i2c_clk: *mut clk,
    pub msg_queue: *mut i2c_msg,
    pub buf: *mut u8,
    pub msg_complete: completion,
    pub adapter: i2c_adapter,
    pub msg_err: c_int,
    pub total_num: c_int,
    pub current_num: c_int,
    pub bus_clk_rate: u32,
    pub isr_status: u32,
    pub msg_len: u16,
    pub addr: u8,
    pub restart_needed: bool,
}

#[no_mangle]
unsafe extern "C" fn mchp_corei2c_core_disable(idev: *mut mchp_corei2c_dev) {
    static void mchp_corei2c_core_disable(struct mchp_corei2c_dev *idev)
    {
    let mut ctrl: u8 = readb(idev.base + CORE_I2C_CTRL);
    ctrl &= ~CTRL_ENS1;
    writeb(ctrl, idev.base + CORE_I2C_CTRL);
    }
#[no_mangle]
unsafe extern "C" fn mchp_corei2c_core_enable(idev: *mut mchp_corei2c_dev) {
    static void mchp_corei2c_core_enable(struct mchp_corei2c_dev *idev)
    {
    let mut ctrl: u8 = readb(idev.base + CORE_I2C_CTRL);
    ctrl |= CTRL_ENS1;
    writeb(ctrl, idev.base + CORE_I2C_CTRL);
    }
#[no_mangle]
unsafe extern "C" fn mchp_corei2c_reset(idev: *mut mchp_corei2c_dev) {
    static void mchp_corei2c_reset(struct mchp_corei2c_dev *idev)
    {
    mchp_corei2c_core_disable(idev);
    mchp_corei2c_core_enable(idev);
    }
#[no_mangle]
pub unsafe extern "C" fn mchp_corei2c_stop(idev: *mut mchp_corei2c_dev) {
    static inline void mchp_corei2c_stop(struct mchp_corei2c_dev *idev)
    {
    let mut ctrl: u8 = readb(idev.base + CORE_I2C_CTRL);
    ctrl |= CTRL_STO;
    writeb(ctrl, idev.base + CORE_I2C_CTRL);
    }
    static inline int mchp_corei2c_set_divisor(u32 rate,
    struct mchp_corei2c_dev *idev)
    {
    u8 clkval, ctrl;
    if (rate >= 960)
    clkval = PCLK_DIV_960;
#[no_mangle]
pub unsafe extern "C" fn if(256: rate >=) -> else {
    else if (rate >= 256)
    clkval = PCLK_DIV_256;
#[no_mangle]
pub unsafe extern "C" fn if(224: rate >=) -> else {
    else if (rate >= 224)
    clkval = PCLK_DIV_224;
#[no_mangle]
pub unsafe extern "C" fn if(192: rate >=) -> else {
    else if (rate >= 192)
    clkval = PCLK_DIV_192;
#[no_mangle]
pub unsafe extern "C" fn if(160: rate >=) -> else {
    else if (rate >= 160)
    clkval = PCLK_DIV_160;
#[no_mangle]
pub unsafe extern "C" fn if(120: rate >=) -> else {
    else if (rate >= 120)
    clkval = PCLK_DIV_120;
#[no_mangle]
pub unsafe extern "C" fn if(60: rate >=) -> else {
    else if (rate >= 60)
    clkval = PCLK_DIV_60;
#[no_mangle]
pub unsafe extern "C" fn if(8: rate >=) -> else {
    else if (rate >= 8)
    clkval = BCLK_DIV_8;
    else
    return -EINVAL;
    ctrl = readb(idev.base + CORE_I2C_CTRL);
    ctrl &= ~CLK_MASK;
    ctrl |= clkval;
    writeb(ctrl, idev.base + CORE_I2C_CTRL);
    ctrl = readb(idev.base + CORE_I2C_CTRL);
    if ((ctrl & CLK_MASK) != clkval)
    return -EIO;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mchp_corei2c_init(idev: *mut mchp_corei2c_dev) -> c_int {
    static int mchp_corei2c_init(struct mchp_corei2c_dev *idev)
    {
    let mut clk_rate: u32 = clk_get_rate(idev.i2c_clk);
    let mut divisor: u32 = clk_rate / idev.bus_clk_rate;
    int ret;
    ret = mchp_corei2c_set_divisor(divisor, idev);
    if (ret)
    return ret;
    mchp_corei2c_reset(idev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mchp_corei2c_empty_rx(idev: *mut mchp_corei2c_dev) {
    static void mchp_corei2c_empty_rx(struct mchp_corei2c_dev *idev)
    {
    u8 ctrl;
    if (idev.msg_len > 0) {
// idev->buf++ = readb(idev->base + CORE_I2C_DATA);
    idev.msg_len--;
    }
    if (idev.msg_len <= 1) {
    ctrl = readb(idev.base + CORE_I2C_CTRL);
    ctrl &= ~CTRL_AA;
    writeb(ctrl, idev.base + CORE_I2C_CTRL);
    }
    }
#[no_mangle]
unsafe extern "C" fn mchp_corei2c_fill_tx(idev: *mut mchp_corei2c_dev) -> c_int {
    static int mchp_corei2c_fill_tx(struct mchp_corei2c_dev *idev)
    {
    if (idev.msg_len > 0)
    writeb(*idev.buf++, idev.base + CORE_I2C_DATA);
    idev.msg_len--;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mchp_corei2c_next_msg(idev: *mut mchp_corei2c_dev) {
    static void mchp_corei2c_next_msg(struct mchp_corei2c_dev *idev)
    {
    struct i2c_msg *this_msg;
    u8 ctrl;
    if (idev.current_num >= idev.total_num) {
    complete(&idev.msg_complete);
    return;
    }
//
// If there's been an error, the isr needs to return control
// to the "main" part of the driver, so as not to keep sending
// messages once it completes and clears the SI bit.
//
    if (idev.msg_err) {
    complete(&idev.msg_complete);
    return;
    }
    this_msg = idev.msg_queue++;
    if (idev.current_num < (idev.total_num - 1)) {
    struct i2c_msg *next_msg = idev.msg_queue;
    idev.restart_needed = next_msg.flags & I2C_M_RD;
    } else {
    idev.restart_needed = false;
    }
    idev.addr = i2c_8bit_addr_from_msg(this_msg);
    idev.msg_len = this_msg.len;
    idev.buf = this_msg.buf;
    ctrl = readb(idev.base + CORE_I2C_CTRL);
    ctrl |= CTRL_STA;
    writeb(ctrl, idev.base + CORE_I2C_CTRL);
    idev.current_num++;
    }
#[no_mangle]
unsafe extern "C" fn mchp_corei2c_handle_isr(idev: *mut mchp_corei2c_dev) -> irqreturn_t {
    static irqreturn_t mchp_corei2c_handle_isr(struct mchp_corei2c_dev *idev)
    {
    let mut status: u32 = idev.isr_status;
    u8 ctrl;
    let mut last_byte: bool = false, finished = false;
    if (!idev.buf)
    return IRQ_NONE;
    switch (status) {
    case STATUS_M_START_SENT:
    case STATUS_M_REPEATED_START_SENT:
    ctrl = readb(idev.base + CORE_I2C_CTRL);
    ctrl &= ~CTRL_STA;
    writeb(idev.addr, idev.base + CORE_I2C_DATA);
    writeb(ctrl, idev.base + CORE_I2C_CTRL);
    break;
    case STATUS_M_ARB_LOST:
    idev.msg_err = -EAGAIN;
    finished = true;
    break;
    case STATUS_M_SLAW_ACK:
    case STATUS_M_TX_DATA_ACK:
    if (idev.msg_len > 0) {
    mchp_corei2c_fill_tx(idev);
    } else {
    if (idev.restart_needed)
    finished = true;
    else
    last_byte = true;
    }
    break;
    case STATUS_M_TX_DATA_NACK:
    case STATUS_M_SLAR_NACK:
    case STATUS_M_SLAW_NACK:
    idev.msg_err = -ENXIO;
    last_byte = true;
    break;
    case STATUS_M_SLAR_ACK:
    ctrl = readb(idev.base + CORE_I2C_CTRL);
    if (idev.msg_len == 1u) {
    ctrl &= ~CTRL_AA;
    writeb(ctrl, idev.base + CORE_I2C_CTRL);
    } else {
    ctrl |= CTRL_AA;
    writeb(ctrl, idev.base + CORE_I2C_CTRL);
    }
    if (idev.msg_len < 1u)
    last_byte = true;
    break;
    case STATUS_M_RX_DATA_ACKED:
    mchp_corei2c_empty_rx(idev);
    break;
    case STATUS_M_RX_DATA_NACKED:
    mchp_corei2c_empty_rx(idev);
    if (idev.msg_len == 0)
    last_byte = true;
    break;
    default:
    break;
    }
// On the last byte to be transmitted, send STOP
    if (last_byte)
    mchp_corei2c_stop(idev);
    if (last_byte || finished)
    mchp_corei2c_next_msg(idev);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn mchp_corei2c_isr(irq: c_int, _dev: *mut c_void) -> irqreturn_t {
    static irqreturn_t mchp_corei2c_isr(int irq, void *_dev)
    {
    struct mchp_corei2c_dev *idev = _dev;
    let mut ret: irqreturn_t = IRQ_NONE;
    u8 ctrl;
    ctrl = readb(idev.base + CORE_I2C_CTRL);
    if (ctrl & CTRL_SI) {
    idev.isr_status = readb(idev.base + CORE_I2C_STATUS);
    ret = mchp_corei2c_handle_isr(idev);
    }
    ctrl = readb(idev.base + CORE_I2C_CTRL);
    ctrl &= ~CTRL_SI;
    writeb(ctrl, idev.base + CORE_I2C_CTRL);
    return ret;
    }
    static int mchp_corei2c_xfer(struct i2c_adapter *adap, struct i2c_msg *msgs,
    int num)
    {
    struct mchp_corei2c_dev *idev = i2c_get_adapdata(adap);
    struct i2c_msg *this_msg = msgs;
    unsigned long time_left;
    u8 ctrl;
    mchp_corei2c_core_enable(idev);
//
// The isr controls the flow of a transfer, this info needs to be saved
// to a location that it can access the queue information from.
//
    idev.restart_needed = false;
    idev.msg_queue = msgs;
    idev.total_num = num;
    idev.current_num = 0;
//
// But the first entry to the isr is triggered by the start in this
// function, so the first message needs to be "dequeued".
//
    idev.addr = i2c_8bit_addr_from_msg(this_msg);
    idev.msg_len = this_msg.len;
    idev.buf = this_msg.buf;
    idev.msg_err = 0;
    if (idev.total_num > 1) {
    struct i2c_msg *next_msg = msgs + 1;
    idev.restart_needed = next_msg.flags & I2C_M_RD;
    }
    idev.current_num++;
    idev.msg_queue++;
    reinit_completion(&idev.msg_complete);
//
// Send the first start to pass control to the isr
//
    ctrl = readb(idev.base + CORE_I2C_CTRL);
    ctrl |= CTRL_STA;
    writeb(ctrl, idev.base + CORE_I2C_CTRL);
    time_left = wait_for_completion_timeout(&idev.msg_complete,
    idev.adapter.timeout);
    if (!time_left)
    return -ETIMEDOUT;
    if (idev.msg_err)
    return idev.msg_err;
    return num;
    }
#[no_mangle]
unsafe extern "C" fn mchp_corei2c_func(adap: *mut i2c_adapter) -> u32 {
    static u32 mchp_corei2c_func(struct i2c_adapter *adap)
    {
    return I2C_FUNC_I2C | I2C_FUNC_SMBUS_EMUL;
    }
    static int mchp_corei2c_smbus_xfer(struct i2c_adapter *adap, u16 addr, unsigned short flags,
    char read_write, u8 command,
    int size, union i2c_smbus_data *data)
    {
    struct i2c_msg msgs[2];
    struct mchp_corei2c_dev *idev = i2c_get_adapdata(adap);
    u8 tx_buf[I2C_SMBUS_BLOCK_MAX + 2];
    u8 rx_buf[I2C_SMBUS_BLOCK_MAX + 1];
    let mut num_msgs: c_int = 1;
    int ret;
    msgs[CORE_I2C_SMBUS_MSG_WR].addr = addr;
    msgs[CORE_I2C_SMBUS_MSG_WR].flags = 0;
    if (read_write == I2C_SMBUS_READ && size <= I2C_SMBUS_BYTE)
    msgs[CORE_I2C_SMBUS_MSG_WR].flags = I2C_M_RD;
    if (read_write == I2C_SMBUS_WRITE && size <= I2C_SMBUS_WORD_DATA)
    msgs[CORE_I2C_SMBUS_MSG_WR].len = size;
    if (read_write == I2C_SMBUS_WRITE && size > I2C_SMBUS_BYTE) {
    msgs[CORE_I2C_SMBUS_MSG_WR].buf = tx_buf;
    msgs[CORE_I2C_SMBUS_MSG_WR].buf[0] = command;
    }
    if (read_write == I2C_SMBUS_READ && size >= I2C_SMBUS_BYTE_DATA) {
    msgs[CORE_I2C_SMBUS_MSG_WR].buf = tx_buf;
    msgs[CORE_I2C_SMBUS_MSG_WR].buf[0] = command;
    msgs[CORE_I2C_SMBUS_MSG_RD].addr = addr;
    msgs[CORE_I2C_SMBUS_MSG_RD].flags = I2C_M_RD;
    num_msgs = 2;
    }
    if (read_write == I2C_SMBUS_READ && size > I2C_SMBUS_QUICK)
    msgs[CORE_I2C_SMBUS_MSG_WR].len = 1;
    switch (size) {
    case I2C_SMBUS_QUICK:
    msgs[CORE_I2C_SMBUS_MSG_WR].buf = core::ptr::null_mut();
    return 0;
    case I2C_SMBUS_BYTE:
    if (read_write == I2C_SMBUS_WRITE)
    msgs[CORE_I2C_SMBUS_MSG_WR].buf = &command;
    else
    msgs[CORE_I2C_SMBUS_MSG_WR].buf = &data.byte;
    break;
    case I2C_SMBUS_BYTE_DATA:
    if (read_write == I2C_SMBUS_WRITE) {
    msgs[CORE_I2C_SMBUS_MSG_WR].buf[1] = data.byte;
    } else {
    msgs[CORE_I2C_SMBUS_MSG_RD].len = size - 1;
    msgs[CORE_I2C_SMBUS_MSG_RD].buf = &data.byte;
    }
    break;
    case I2C_SMBUS_WORD_DATA:
    if (read_write == I2C_SMBUS_WRITE) {
    msgs[CORE_I2C_SMBUS_MSG_WR].buf[1] = data.word & 0xFF;
    msgs[CORE_I2C_SMBUS_MSG_WR].buf[2] = (data.word >> 8) & 0xFF;
    } else {
    msgs[CORE_I2C_SMBUS_MSG_RD].len = size - 1;
    msgs[CORE_I2C_SMBUS_MSG_RD].buf = rx_buf;
    }
    break;
    case I2C_SMBUS_BLOCK_DATA:
    if (read_write == I2C_SMBUS_WRITE) {
    int data_len;
    data_len = data.block[0];
    msgs[CORE_I2C_SMBUS_MSG_WR].len = data_len + 2;
    for (int i = 0; i <= data_len; i++)
    msgs[CORE_I2C_SMBUS_MSG_WR].buf[i + 1] = data.block[i];
    } else {
    msgs[CORE_I2C_SMBUS_MSG_RD].len = I2C_SMBUS_BLOCK_MAX + 1;
    msgs[CORE_I2C_SMBUS_MSG_RD].buf = rx_buf;
    }
    break;
    default:
    return -EOPNOTSUPP;
    }
    ret = mchp_corei2c_xfer(&idev.adapter, msgs, num_msgs);
    if (ret < 0)
    return ret;
    if (read_write == I2C_SMBUS_WRITE || size <= I2C_SMBUS_BYTE_DATA)
    return 0;
    switch (size) {
    case I2C_SMBUS_WORD_DATA:
    data.word = (rx_buf[0] | (rx_buf[1] << 8));
    break;
    case I2C_SMBUS_BLOCK_DATA:
    if (rx_buf[0] > I2C_SMBUS_BLOCK_MAX)
    rx_buf[0] = I2C_SMBUS_BLOCK_MAX;
// As per protocol first member of block is size of the block.
    for (int i = 0; i <= rx_buf[0]; i++)
    data.block[i] = rx_buf[i];
    break;
    }
    return 0;
    }
    static const struct i2c_algorithm mchp_corei2c_algo = {
    .xfer = mchp_corei2c_xfer,
    .functionality = mchp_corei2c_func,
    .smbus_xfer = mchp_corei2c_smbus_xfer,
    };
#[no_mangle]
unsafe extern "C" fn mchp_corei2c_probe(pdev: *mut platform_device) -> c_int {
    static int mchp_corei2c_probe(struct platform_device *pdev)
    {
    struct mchp_corei2c_dev *idev;
    struct resource *res;
    int irq, ret;
    idev = devm_kzalloc(&pdev.dev, sizeof(*idev), GFP_KERNEL);
    if (!idev)
    return -ENOMEM;
    idev.base = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(idev.base))
    return PTR_ERR(idev.base);
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    idev.i2c_clk = devm_clk_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(idev.i2c_clk))
    return dev_err_probe(&pdev.dev, PTR_ERR(idev.i2c_clk),
    "missing clock\n");
    idev.dev = &pdev.dev;
    init_completion(&idev.msg_complete);
    ret = device_property_read_u32(idev.dev, "clock-frequency",
    &idev.bus_clk_rate);
    if (ret || !idev.bus_clk_rate) {
    dev_info(&pdev.dev, "default to 100kHz\n");
    idev.bus_clk_rate = I2C_MAX_STANDARD_MODE_FREQ;
    }
    if (idev.bus_clk_rate > I2C_MAX_FAST_MODE_FREQ)
    return dev_err_probe(&pdev.dev, -EINVAL,
    "clock-frequency too high: %d\n",
    idev.bus_clk_rate);
//
// This driver supports both the hard peripherals & soft FPGA cores.
// The hard peripherals do not have shared IRQs, but we don't have
// control over what way the interrupts are wired for the soft cores.
//
    ret = devm_request_irq(&pdev.dev, irq, mchp_corei2c_isr, IRQF_SHARED,
    pdev.name, idev);
    if (ret)
    return ret;
    ret = clk_prepare_enable(idev.i2c_clk);
    if (ret)
    return dev_err_probe(&pdev.dev, ret,
    "failed to enable clock\n");
    ret = mchp_corei2c_init(idev);
    if (ret) {
    clk_disable_unprepare(idev.i2c_clk);
    return dev_err_probe(&pdev.dev, ret, "failed to program clock divider\n");
    }
    i2c_set_adapdata(&idev.adapter, idev);
    snprintf(idev.adapter.name, sizeof(idev.adapter.name),
    "Microchip I2C hw bus at %08lx", (unsigned long)res.start);
    idev.adapter.owner = THIS_MODULE;
    idev.adapter.algo = &mchp_corei2c_algo;
    idev.adapter.dev.parent = &pdev.dev;
    idev.adapter.dev.of_node = pdev.dev.of_node;
    idev.adapter.timeout = HZ;
    platform_set_drvdata(pdev, idev);
    ret = i2c_add_adapter(&idev.adapter);
    if (ret) {
    clk_disable_unprepare(idev.i2c_clk);
    return ret;
    }
    dev_info(&pdev.dev, "registered CoreI2C bus driver\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mchp_corei2c_remove(pdev: *mut platform_device) {
    static void mchp_corei2c_remove(struct platform_device *pdev)
    {
    struct mchp_corei2c_dev *idev = platform_get_drvdata(pdev);
    clk_disable_unprepare(idev.i2c_clk);
    i2c_del_adapter(&idev.adapter);
    }
    static const struct of_device_id mchp_corei2c_of_match[] = {
    { .compatible = "microchip,mpfs-i2c" },
    { .compatible = "microchip,corei2c-rtl-v7" },
    {},
    };
    MODULE_DEVICE_TABLE(of, mchp_corei2c_of_match);
    static struct platform_driver mchp_corei2c_driver = {
    .probe = mchp_corei2c_probe,
    .remove = mchp_corei2c_remove,
    .driver = {
    .name = "microchip-corei2c",
    .of_match_table = mchp_corei2c_of_match,
    },
    };
    module_platform_driver(mchp_corei2c_driver);
    MODULE_DESCRIPTION("Microchip CoreI2C bus driver");
    MODULE_AUTHOR("Daire McNamara <daire.mcnamara@microchip.com>");
    MODULE_AUTHOR("Conor Dooley <conor.dooley@microchip.com>");
    MODULE_LICENSE("GPL");
