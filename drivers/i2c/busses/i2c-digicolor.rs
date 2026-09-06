//! Automatically rewritten from C to Rust
//! Source: drivers/i2c/busses/i2c-digicolor.c
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
// I2C bus driver for Conexant Digicolor SoCs
//
// Author: Baruch Siach <baruch@tkos.co.il>
//
// Copyright (C) 2015 Paradox Innovation Ltd.
//

pub const TIMEOUT_MS: c_int = 100;
pub const II_CONTROL: c_uint = 0x0;

pub const II_CLOCKTIME: c_uint = 0x1;
pub const II_COMMAND: c_uint = 0x2;
pub const II_CMD_START: c_int = 1;
pub const II_CMD_RESTART: c_int = 2;
pub const II_CMD_SEND_ACK: c_int = 3;
pub const II_CMD_GET_ACK: c_int = 6;
pub const II_CMD_GET_NOACK: c_int = 7;
pub const II_CMD_STOP: c_int = 10;

pub const II_CMD_STATUS_NORMAL: c_int = 0;
pub const II_CMD_STATUS_ACK_GOOD: c_int = 1;
pub const II_CMD_STATUS_ACK_BAD: c_int = 2;
pub const II_CMD_STATUS_ABORT: c_int = 3;
pub const II_DATA: c_uint = 0x3;
pub const II_INTFLAG_CLEAR: c_uint = 0x8;
pub const II_INTENABLE: c_uint = 0xa;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_i2c {
    pub adap: i2c_adapter,
    pub dev: *mut device,
    pub regs: *mut void __iomem,
    pub clk: *mut clk,
    pub frequency: c_uint,
    pub msg: *mut i2c_msg,
    pub msgbuf_ptr: c_uint,
    pub last: c_int,
    pub lock: spinlock_t,
    pub done: completion,
    pub state: c_int,
    pub error: c_int,
}

    enum {
    STATE_IDLE,
    STATE_START,
    STATE_ADDR,
    STATE_WRITE,
    STATE_READ,
    STATE_STOP,
    };
#[no_mangle]
unsafe extern "C" fn dc_i2c_cmd(i2c: *mut dc_i2c, cmd: u8) {
    static void dc_i2c_cmd(struct dc_i2c *i2c, u8 cmd)
    {
    writeb_relaxed(cmd | II_COMMAND_GO, i2c.regs + II_COMMAND);
    }
#[no_mangle]
unsafe extern "C" fn dc_i2c_addr_cmd(msg: *mut i2c_msg) -> u8 {
    static u8 dc_i2c_addr_cmd(struct i2c_msg *msg)
    {
    let mut addr: u8 = (msg.addr & 0x7f) << 1;
    if (msg.flags & I2C_M_RD)
    addr |= 1;
    return addr;
    }
#[no_mangle]
unsafe extern "C" fn dc_i2c_data(i2c: *mut dc_i2c, data: u8) {
    static void dc_i2c_data(struct dc_i2c *i2c, u8 data)
    {
    writeb_relaxed(data, i2c.regs + II_DATA);
    }
#[no_mangle]
unsafe extern "C" fn dc_i2c_write_byte(i2c: *mut dc_i2c, byte: u8) {
    static void dc_i2c_write_byte(struct dc_i2c *i2c, u8 byte)
    {
    dc_i2c_data(i2c, byte);
    dc_i2c_cmd(i2c, II_CMD_SEND_ACK);
    }
#[no_mangle]
unsafe extern "C" fn dc_i2c_write_buf(i2c: *mut dc_i2c) {
    static void dc_i2c_write_buf(struct dc_i2c *i2c)
    {
    dc_i2c_write_byte(i2c, i2c.msg.buf[i2c.msgbuf_ptr++]);
    }
#[no_mangle]
unsafe extern "C" fn dc_i2c_next_read(i2c: *mut dc_i2c) {
    static void dc_i2c_next_read(struct dc_i2c *i2c)
    {
    let mut last: bool = (i2c.msgbuf_ptr + 1 == i2c.msg.len);
    dc_i2c_cmd(i2c, last ? II_CMD_GET_NOACK : II_CMD_GET_ACK);
    }
#[no_mangle]
unsafe extern "C" fn dc_i2c_stop(i2c: *mut dc_i2c) {
    static void dc_i2c_stop(struct dc_i2c *i2c)
    {
    i2c.state = STATE_STOP;
    if (i2c.last)
    dc_i2c_cmd(i2c, II_CMD_STOP);
    else
    complete(&i2c.done);
    }
#[no_mangle]
unsafe extern "C" fn dc_i2c_read_byte(i2c: *mut dc_i2c) -> u8 {
    static u8 dc_i2c_read_byte(struct dc_i2c *i2c)
    {
    return readb_relaxed(i2c.regs + II_DATA);
    }
#[no_mangle]
unsafe extern "C" fn dc_i2c_read_buf(i2c: *mut dc_i2c) {
    static void dc_i2c_read_buf(struct dc_i2c *i2c)
    {
    i2c.msg.buf[i2c.msgbuf_ptr++] = dc_i2c_read_byte(i2c);
    dc_i2c_next_read(i2c);
    }
#[no_mangle]
unsafe extern "C" fn dc_i2c_set_irq(i2c: *mut dc_i2c, enable: c_int) {
    static void dc_i2c_set_irq(struct dc_i2c *i2c, int enable)
    {
    if (enable)
    writeb_relaxed(1, i2c.regs + II_INTFLAG_CLEAR);
    writeb_relaxed(!!enable, i2c.regs + II_INTENABLE);
    }
#[no_mangle]
unsafe extern "C" fn dc_i2c_cmd_status(i2c: *mut dc_i2c) -> c_int {
    static int dc_i2c_cmd_status(struct dc_i2c *i2c)
    {
    let mut cmd: u8 = readb_relaxed(i2c.regs + II_COMMAND);
    return II_COMMAND_COMPLETION_STATUS(cmd);
    }
#[no_mangle]
unsafe extern "C" fn dc_i2c_start_msg(i2c: *mut dc_i2c, first: c_int) {
    static void dc_i2c_start_msg(struct dc_i2c *i2c, int first)
    {
    struct i2c_msg *msg = i2c.msg;
    if (!(msg.flags & I2C_M_NOSTART)) {
    i2c.state = STATE_START;
    dc_i2c_cmd(i2c, first ? II_CMD_START : II_CMD_RESTART);
    } else if (msg.flags & I2C_M_RD) {
    i2c.state = STATE_READ;
    dc_i2c_next_read(i2c);
    } else {
    i2c.state = STATE_WRITE;
    dc_i2c_write_buf(i2c);
    }
    }
#[no_mangle]
unsafe extern "C" fn dc_i2c_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t dc_i2c_irq(int irq, void *dev_id)
    {
    struct dc_i2c *i2c = dev_id;
    let mut cmd_status: c_int = dc_i2c_cmd_status(i2c);
    u8 addr_cmd;
    writeb_relaxed(1, i2c.regs + II_INTFLAG_CLEAR);
    spin_lock(&i2c.lock);
    if (cmd_status == II_CMD_STATUS_ACK_BAD
    || cmd_status == II_CMD_STATUS_ABORT) {
    i2c.error = -EIO;
    complete(&i2c.done);
    goto out;
    }
    switch (i2c.state) {
    case STATE_START:
    addr_cmd = dc_i2c_addr_cmd(i2c.msg);
    dc_i2c_write_byte(i2c, addr_cmd);
    i2c.state = STATE_ADDR;
    break;
    case STATE_ADDR:
    if (i2c.msg.flags & I2C_M_RD) {
    dc_i2c_next_read(i2c);
    i2c.state = STATE_READ;
    break;
    }
    i2c.state = STATE_WRITE;
    fallthrough;
    case STATE_WRITE:
    if (i2c.msgbuf_ptr < i2c.msg.len)
    dc_i2c_write_buf(i2c);
    else
    dc_i2c_stop(i2c);
    break;
    case STATE_READ:
    if (i2c.msgbuf_ptr < i2c.msg.len)
    dc_i2c_read_buf(i2c);
    else
    dc_i2c_stop(i2c);
    break;
    case STATE_STOP:
    i2c.state = STATE_IDLE;
    complete(&i2c.done);
    break;
    }
    out:
    spin_unlock(&i2c.lock);
    return IRQ_HANDLED;
    }
    static int dc_i2c_xfer_msg(struct dc_i2c *i2c, struct i2c_msg *msg, int first,
    int last)
    {
    let mut time_left: c_ulong = msecs_to_jiffies(TIMEOUT_MS);
    unsigned long flags;
    spin_lock_irqsave(&i2c.lock, flags);
    i2c.msg = msg;
    i2c.msgbuf_ptr = 0;
    i2c.last = last;
    i2c.error = 0;
    reinit_completion(&i2c.done);
    dc_i2c_set_irq(i2c, 1);
    dc_i2c_start_msg(i2c, first);
    spin_unlock_irqrestore(&i2c.lock, flags);
    time_left = wait_for_completion_timeout(&i2c.done, time_left);
    dc_i2c_set_irq(i2c, 0);
    if (time_left == 0) {
    i2c.state = STATE_IDLE;
    return -ETIMEDOUT;
    }
    if (i2c.error)
    return i2c.error;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dc_i2c_xfer(adap: *mut i2c_adapter, msgs: *mut i2c_msg, num: c_int) -> c_int {
    static int dc_i2c_xfer(struct i2c_adapter *adap, struct i2c_msg *msgs, int num)
    {
    struct dc_i2c *i2c = adap.algo_data;
    int i, ret;
    for (i = 0; i < num; i++) {
    ret = dc_i2c_xfer_msg(i2c, &msgs[i], i == 0, i == num - 1);
    if (ret)
    return ret;
    }
    return num;
    }
#[no_mangle]
unsafe extern "C" fn dc_i2c_init_hw(i2c: *mut dc_i2c) -> c_int {
    static int dc_i2c_init_hw(struct dc_i2c *i2c)
    {
    let mut clk_rate: c_ulong = clk_get_rate(i2c.clk);
    unsigned int clocktime;
    writeb_relaxed(II_CONTROL_LOCAL_RESET, i2c.regs + II_CONTROL);
    udelay(100);
    writeb_relaxed(0, i2c.regs + II_CONTROL);
    udelay(100);
    clocktime = DIV_ROUND_UP(clk_rate, 64 * i2c.frequency);
    if (clocktime < 1 || clocktime > 0xff) {
    dev_err(i2c.dev, "can't set bus speed of %u Hz\n",
    i2c.frequency);
    return -EINVAL;
    }
    writeb_relaxed(clocktime - 1, i2c.regs + II_CLOCKTIME);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dc_i2c_func(adap: *mut i2c_adapter) -> u32 {
    static u32 dc_i2c_func(struct i2c_adapter *adap)
    {
    return I2C_FUNC_I2C | I2C_FUNC_SMBUS_EMUL | I2C_FUNC_NOSTART;
    }
    static const struct i2c_algorithm dc_i2c_algorithm = {
    .xfer = dc_i2c_xfer,
    .functionality = dc_i2c_func,
    };
#[no_mangle]
unsafe extern "C" fn dc_i2c_probe(pdev: *mut platform_device) -> c_int {
    static int dc_i2c_probe(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    struct dc_i2c *i2c;
    let mut ret: c_int = 0, irq;
    i2c = devm_kzalloc(&pdev.dev, sizeof(struct dc_i2c), GFP_KERNEL);
    if (!i2c)
    return -ENOMEM;
    if (of_property_read_u32(pdev.dev.of_node, "clock-frequency",
    &i2c.frequency))
    i2c.frequency = I2C_MAX_STANDARD_MODE_FREQ;
    i2c.dev = &pdev.dev;
    platform_set_drvdata(pdev, i2c);
    spin_lock_init(&i2c.lock);
    init_completion(&i2c.done);
    i2c.clk = devm_clk_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(i2c.clk))
    return PTR_ERR(i2c.clk);
    i2c.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(i2c.regs))
    return PTR_ERR(i2c.regs);
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    ret = devm_request_irq(&pdev.dev, irq, dc_i2c_irq, 0,
    dev_name(&pdev.dev), i2c);
    if (ret < 0)
    return ret;
    strscpy(i2c.adap.name, "Conexant Digicolor I2C adapter",
    sizeof(i2c.adap.name));
    i2c.adap.owner = THIS_MODULE;
    i2c.adap.algo = &dc_i2c_algorithm;
    i2c.adap.dev.parent = &pdev.dev;
    i2c.adap.dev.of_node = np;
    i2c.adap.algo_data = i2c;
    ret = dc_i2c_init_hw(i2c);
    if (ret)
    return ret;
    ret = clk_prepare_enable(i2c.clk);
    if (ret < 0)
    return ret;
    ret = i2c_add_adapter(&i2c.adap);
    if (ret < 0) {
    clk_disable_unprepare(i2c.clk);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dc_i2c_remove(pdev: *mut platform_device) {
    static void dc_i2c_remove(struct platform_device *pdev)
    {
    struct dc_i2c *i2c = platform_get_drvdata(pdev);
    i2c_del_adapter(&i2c.adap);
    clk_disable_unprepare(i2c.clk);
    }
    static const struct of_device_id dc_i2c_match[] = {
    { .compatible = "cnxt,cx92755-i2c" },
    { }
    };
    MODULE_DEVICE_TABLE(of, dc_i2c_match);
    static struct platform_driver dc_i2c_driver = {
    .probe   = dc_i2c_probe,
    .remove = dc_i2c_remove,
    .driver  = {
    .name  = "digicolor-i2c",
    .of_match_table = dc_i2c_match,
    },
    };
    module_platform_driver(dc_i2c_driver);
    MODULE_AUTHOR("Baruch Siach <baruch@tkos.co.il>");
    MODULE_DESCRIPTION("Conexant Digicolor I2C controller driver");
    MODULE_LICENSE("GPL v2");
