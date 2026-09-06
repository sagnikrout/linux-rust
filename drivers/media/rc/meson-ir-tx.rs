//! Automatically rewritten from C to Rust
//! Source: drivers/media/rc/meson-ir-tx.c
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
// meson-ir-tx.c - Amlogic Meson IR TX driver
//
// Copyright (c) 2021, SberDevices. All Rights Reserved.
//
// Author: Viktor Prutyanov <viktor.prutyanov@phystech.edu>
//

pub const MIRTX_DEFAULT_CARRIER: c_int = 38000;
pub const MIRTX_DEFAULT_DUTY_CYCLE: c_int = 50;
pub const MIRTX_FIFO_THD: c_int = 32;
pub const IRB_MOD_1US_CLK_RATE: c_int = 1000000;
pub const IRB_FIFO_LEN: c_int = 128;
pub const IRB_ADDR0: c_uint = 0x0;
pub const IRB_ADDR1: c_uint = 0x4;
pub const IRB_ADDR2: c_uint = 0x8;
pub const IRB_ADDR3: c_uint = 0xc;

// IRCTRL_IR_BLASTER_ADDR0

pub const IRB_MOD_SYS_CLK: c_int = 0;
pub const IRB_MOD_XTAL3_CLK: c_int = 1;
pub const IRB_MOD_1US_CLK: c_int = 2;
pub const IRB_MOD_10US_CLK: c_int = 3;

// IRCTRL_IR_BLASTER_ADDR2

// IRCTRL_IR_BLASTER_ADDR2

// IRCTRL_IR_BLASTER_ADDR3

#[repr(C)]
#[derive(Copy, Clone)]
pub struct meson_irtx {
    pub dev: *mut device,
    pub reg_base: *mut void __iomem,
    pub buf: *mut u32,
    pub buf_len: c_uint,
    pub buf_head: c_uint,
    pub carrier: c_uint,
    pub duty_cycle: c_uint,
// Locks buf
    pub lock: spinlock_t,
    pub completion: completion,
    pub clk_rate: c_ulong,
}

#[no_mangle]
unsafe extern "C" fn meson_irtx_set_mod(ir: *mut meson_irtx) {
    static void meson_irtx_set_mod(struct meson_irtx *ir)
    {
    let mut cnt: c_uint = DIV_ROUND_CLOSEST(ir.clk_rate, ir.carrier);
    let mut pulse_cnt: c_uint = DIV_ROUND_CLOSEST(cnt * ir.duty_cycle, 100);
    let mut space_cnt: c_uint = cnt - pulse_cnt;
    dev_dbg(ir.dev, "F_mod = %uHz, T_mod = %luns, duty_cycle = %u%%\n",
    ir.carrier, NSEC_PER_SEC / ir.clk_rate * cnt,
    100 * pulse_cnt / cnt);
    writel(IRB_MOD_COUNT(pulse_cnt, space_cnt),
    ir.reg_base + IRB_ADDR1);
    }
#[no_mangle]
unsafe extern "C" fn meson_irtx_setup(ir: *mut meson_irtx, clk_nr: c_uint) {
    static void meson_irtx_setup(struct meson_irtx *ir, unsigned int clk_nr)
    {
//
// Disable the TX, set modulator clock tick and set initialize
// output to be high. Set up carrier frequency and duty cycle. Then
// unset initialize output. Enable FIFO interrupt, set FIFO interrupt
// threshold. Finally, enable the transmitter back.
//
    writel(~IRB_ENABLE & (IRB_MOD_CLK(clk_nr) | IRB_INIT_HIGH),
    ir.reg_base + IRB_ADDR0);
    meson_irtx_set_mod(ir);
    writel(readl(ir.reg_base + IRB_ADDR0) & ~IRB_INIT_HIGH,
    ir.reg_base + IRB_ADDR0);
    writel(IRB_FIFO_IRQ_ENABLE | MIRTX_FIFO_THD,
    ir.reg_base + IRB_ADDR3);
    writel(readl(ir.reg_base + IRB_ADDR0) | IRB_ENABLE,
    ir.reg_base + IRB_ADDR0);
    }
#[no_mangle]
unsafe extern "C" fn meson_irtx_prepare_pulse(ir: *mut meson_irtx, time: c_uint) -> u32 {
    static u32 meson_irtx_prepare_pulse(struct meson_irtx *ir, unsigned int time)
    {
    unsigned int delay;
    let mut tb: c_uint = IRB_TB_MOD_CLK;
    let mut tb_us: c_uint = DIV_ROUND_CLOSEST(USEC_PER_SEC, ir.carrier);
    delay = (DIV_ROUND_CLOSEST(time, tb_us) - 1) & IRB_DELAY_MASK;
    return ((IRB_WRITE_FIFO | IRB_MOD_ENABLE) | tb | delay);
    }
#[no_mangle]
unsafe extern "C" fn meson_irtx_prepare_space(ir: *mut meson_irtx, time: c_uint) -> u32 {
    static u32 meson_irtx_prepare_space(struct meson_irtx *ir, unsigned int time)
    {
    unsigned int delay;
    let mut tb: c_uint = IRB_TB_100US;
    let mut tb_us: c_uint = 100;
    if (time <= IRB_MAX_DELAY) {
    tb = IRB_TB_1US;
    tb_us = 1;
    } else if (time <= 10 * IRB_MAX_DELAY) {
    tb = IRB_TB_10US;
    tb_us = 10;
    } else if (time <= 100 * IRB_MAX_DELAY) {
    tb = IRB_TB_100US;
    tb_us = 100;
    }
    delay = (DIV_ROUND_CLOSEST(time, tb_us) - 1) & IRB_DELAY_MASK;
    return ((IRB_WRITE_FIFO & ~IRB_MOD_ENABLE) | tb | delay);
    }
#[no_mangle]
unsafe extern "C" fn meson_irtx_send_buffer(ir: *mut meson_irtx) {
    static void meson_irtx_send_buffer(struct meson_irtx *ir)
    {
    let mut nr: c_uint = 0;
    let mut max_fifo_level: c_uint = IRB_FIFO_LEN - MIRTX_FIFO_THD;
    while (ir.buf_head < ir.buf_len && nr < max_fifo_level) {
    writel(ir.buf[ir.buf_head], ir.reg_base + IRB_ADDR2);
    ir.buf_head++;
    nr++;
    }
    }
    static bool meson_irtx_check_buf(struct meson_irtx *ir,
    unsigned int *buf, unsigned int len)
    {
    unsigned int i;
    for (i = 0; i < len; i++) {
    unsigned int max_tb_us;
//
// Max space timebase is 100 us.
// Pulse timebase equals to carrier period.
//
    if (i % 2 == 0)
    max_tb_us = USEC_PER_SEC / ir.carrier;
    else
    max_tb_us = 100;
    if (buf[i] >= max_tb_us * IRB_MAX_DELAY)
    return false;
    }
    return true;
    }
    static void meson_irtx_fill_buf(struct meson_irtx *ir, u32 *dst_buf,
    unsigned int *src_buf, unsigned int len)
    {
    unsigned int i;
    for (i = 0; i < len; i++) {
    if (i % 2 == 0)
    dst_buf[i] = meson_irtx_prepare_pulse(ir, src_buf[i]);
    else
    dst_buf[i] = meson_irtx_prepare_space(ir, src_buf[i]);
    }
    }
#[no_mangle]
unsafe extern "C" fn meson_irtx_irqhandler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t meson_irtx_irqhandler(int irq, void *data)
    {
    unsigned long flags;
    struct meson_irtx *ir = data;
    writel(readl(ir.reg_base + IRB_ADDR3) & ~IRB_FIFO_THD_PENDING,
    ir.reg_base + IRB_ADDR3);
    if (completion_done(&ir.completion))
    return IRQ_HANDLED;
    spin_lock_irqsave(&ir.lock, flags);
    if (ir.buf_head < ir.buf_len)
    meson_irtx_send_buffer(ir);
    else
    complete(&ir.completion);
    spin_unlock_irqrestore(&ir.lock, flags);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn meson_irtx_set_carrier(rc: *mut rc_dev, carrier: u32) -> c_int {
    static int meson_irtx_set_carrier(struct rc_dev *rc, u32 carrier)
    {
    struct meson_irtx *ir = rc.priv;
    if (carrier == 0)
    return -EINVAL;
    ir.carrier = carrier;
    meson_irtx_set_mod(ir);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn meson_irtx_set_duty_cycle(rc: *mut rc_dev, duty_cycle: u32) -> c_int {
    static int meson_irtx_set_duty_cycle(struct rc_dev *rc, u32 duty_cycle)
    {
    struct meson_irtx *ir = rc.priv;
    ir.duty_cycle = duty_cycle;
    meson_irtx_set_mod(ir);
    return 0;
    }
    static void meson_irtx_update_buf(struct meson_irtx *ir, u32 *buf,
    unsigned int len, unsigned int head)
    {
    ir.buf = buf;
    ir.buf_len = len;
    ir.buf_head = head;
    }
    static int meson_irtx_transmit(struct rc_dev *rc, unsigned int *buf,
    unsigned int len)
    {
    unsigned long flags;
    struct meson_irtx *ir = rc.priv;
    u32 *tx_buf;
    let mut ret: c_int = len;
    if (!meson_irtx_check_buf(ir, buf, len))
    return -EINVAL;
    tx_buf = kmalloc_array(len, sizeof(u32), GFP_KERNEL);
    if (!tx_buf)
    return -ENOMEM;
    meson_irtx_fill_buf(ir, tx_buf, buf, len);
    dev_dbg(ir.dev, "TX buffer filled, length = %u\n", len);
    spin_lock_irqsave(&ir.lock, flags);
    meson_irtx_update_buf(ir, tx_buf, len, 0);
    reinit_completion(&ir.completion);
    meson_irtx_send_buffer(ir);
    spin_unlock_irqrestore(&ir.lock, flags);
    if (!wait_for_completion_timeout(&ir.completion,
    usecs_to_jiffies(IR_MAX_DURATION)))
    ret = -ETIMEDOUT;
    spin_lock_irqsave(&ir.lock, flags);
    kfree(ir.buf);
    meson_irtx_update_buf(ir, core::ptr::null_mut(), 0, 0);
    spin_unlock_irqrestore(&ir.lock, flags);
    return ret;
    }
    static int meson_irtx_mod_clock_probe(struct meson_irtx *ir,
    unsigned int *clk_nr)
    {
    struct device_node *np = ir.dev.of_node;
    struct clk *clock;
    if (!np)
    return -ENODEV;
    clock = devm_clk_get(ir.dev, "xtal");
    if (IS_ERR(clock) || clk_prepare_enable(clock))
    return -ENODEV;
// clk_nr = IRB_MOD_XTAL3_CLK;
    ir.clk_rate = clk_get_rate(clock) / 3;
    if (ir.clk_rate < IRB_MOD_1US_CLK_RATE) {
// clk_nr = IRB_MOD_1US_CLK;
    ir.clk_rate = IRB_MOD_1US_CLK_RATE;
    }
    dev_info(ir.dev, "F_clk = %luHz\n", ir.clk_rate);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn meson_irtx_probe(pdev: *mut platform_device) -> c_int {
    static int meson_irtx_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct meson_irtx *ir;
    struct rc_dev *rc;
    int irq;
    unsigned int clk_nr;
    int ret;
    ir = devm_kzalloc(dev, sizeof(*ir), GFP_KERNEL);
    if (!ir)
    return -ENOMEM;
    ir.reg_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(ir.reg_base))
    return PTR_ERR(ir.reg_base);
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return -ENODEV;
    ir.dev = dev;
    ir.carrier = MIRTX_DEFAULT_CARRIER;
    ir.duty_cycle = MIRTX_DEFAULT_DUTY_CYCLE;
    init_completion(&ir.completion);
    spin_lock_init(&ir.lock);
    ret = meson_irtx_mod_clock_probe(ir, &clk_nr);
    if (ret)
    return dev_err_probe(dev, ret, "modulator clock setup failed\n");
    meson_irtx_setup(ir, clk_nr);
    ret = devm_request_irq(dev, irq,
    meson_irtx_irqhandler,
    IRQF_TRIGGER_RISING,
    DRIVER_NAME, ir);
    if (ret)
    return dev_err_probe(dev, ret, "irq request failed\n");
    rc = rc_allocate_device(RC_DRIVER_IR_RAW_TX);
    if (!rc)
    return -ENOMEM;
    rc.driver_name = DRIVER_NAME;
    rc.device_name = DEVICE_NAME;
    rc.priv = ir;
    rc.tx_ir = meson_irtx_transmit;
    rc.s_tx_carrier = meson_irtx_set_carrier;
    rc.s_tx_duty_cycle = meson_irtx_set_duty_cycle;
    ret = devm_rc_register_device(dev, rc);
    if (ret < 0) {
    rc_free_device(rc);
    return dev_err_probe(dev, ret, "rc_dev registration failed\n");
    }
    return 0;
    }
    static const struct of_device_id meson_irtx_dt_match[] = {
    {
    .compatible = "amlogic,meson-g12a-ir-tx",
    },
    {},
    };
    MODULE_DEVICE_TABLE(of, meson_irtx_dt_match);
    static struct platform_driver meson_irtx_pd = {
    .probe = meson_irtx_probe,
    .driver = {
    .name = DRIVER_NAME,
    .of_match_table = meson_irtx_dt_match,
    },
    };
    module_platform_driver(meson_irtx_pd);
    MODULE_DESCRIPTION("Meson IR TX driver");
    MODULE_AUTHOR("Viktor Prutyanov <viktor.prutyanov@phystech.edu>");
    MODULE_LICENSE("GPL");
