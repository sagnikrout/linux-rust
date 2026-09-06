//! Automatically rewritten from C to Rust
//! Source: drivers/net/pse-pd/realtek-pse-mcu-uart.c
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


// SPDX-License-Identifier: GPL-2.0-or-later

pub const RTPSE_MCU_UART_BAUD_DEFAULT: c_int = 19200;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtpse_mcu_uart {
    pub pse: rtpse_mcu_ctrl,
    pub serdev: *mut serdev_device,
    pub rx_done: completion,
    pub /: *mut *mut spinlock_t rx_lock; / protects rx_buf and rx_len,
    pub rx_len: usize,
    pub rx_buf: [u8; RTPSE_MCU_MSG_SIZE],
}

//
// No framing is done here: a glitched frame costs one transaction, then
// the next _send re-frames from rx_len 0. Resync works by returning count
// (not take), dropping any overflow so serdev keeps no leftover to bleed
// into the next frame.
//
    static size_t rtpse_mcu_uart_receive(struct serdev_device *serdev,
    const u8 *buf, size_t count)
    {
    struct rtpse_mcu_uart *ctx = serdev_device_get_drvdata(serdev);
    size_t take;
    scoped_guard(spinlock_irqsave, &ctx.rx_lock) {
    take = min(count, sizeof(ctx.rx_buf) - ctx.rx_len);
    if (take) {
    memcpy(ctx.rx_buf + ctx.rx_len, buf, take);
    ctx.rx_len += take;
    if (ctx.rx_len == sizeof(ctx.rx_buf))
    complete(&ctx.rx_done);
    }
    }
// consume all to avoid desync/misalignment
    return count;
    }
    static const struct serdev_device_ops rtpse_mcu_uart_serdev_ops = {
    .receive_buf = rtpse_mcu_uart_receive,
    .write_wakeup = serdev_device_write_wakeup,
    };
#[no_mangle]
unsafe extern "C" fn rtpse_mcu_uart_send(pse: *mut rtpse_mcu_ctrl, req: *const rtpse_mcu_msg) -> c_int {
    static int rtpse_mcu_uart_send(struct rtpse_mcu_ctrl *pse, const struct rtpse_mcu_msg *req)
    {
    struct rtpse_mcu_uart *ctx = to_rtpse_mcu_uart(pse);
    int written;
// clear any leftover rx state before transmitting
    scoped_guard(spinlock_irqsave, &ctx.rx_lock) {
    reinit_completion(&ctx.rx_done);
    ctx.rx_len = 0;
    }
    written = serdev_device_write(ctx.serdev, (const u8 *)req, sizeof(*req),
    RTPSE_MCU_UART_TX_TIMEOUT);
    if (written < 0)
    return written;
    if (written != sizeof(*req))
    return -EIO;
    return 0;
    }
    static int rtpse_mcu_uart_recv(struct rtpse_mcu_ctrl *pse,
    const struct rtpse_mcu_msg *req,
    struct rtpse_mcu_msg *resp)
    {
    struct rtpse_mcu_uart *ctx = to_rtpse_mcu_uart(pse);
    if (!wait_for_completion_timeout(&ctx.rx_done, RTPSE_MCU_UART_RX_TIMEOUT))
    return -ETIMEDOUT;
    scoped_guard(spinlock_irqsave, &ctx.rx_lock) {
    if (ctx.rx_len != sizeof(*resp))
    return -EIO;
    memcpy(resp, ctx.rx_buf, sizeof(*resp));
    }
    return 0;
    }
    static const struct rtpse_mcu_transport_ops rtpse_mcu_uart_transport_ops = {
    .send = rtpse_mcu_uart_send,
    .recv = rtpse_mcu_uart_recv,
    };
#[no_mangle]
unsafe extern "C" fn rtpse_mcu_uart_probe(serdev: *mut serdev_device) -> c_int {
    static int rtpse_mcu_uart_probe(struct serdev_device *serdev)
    {
    let mut speed: u32 = RTPSE_MCU_UART_BAUD_DEFAULT;
    struct device *dev = &serdev.dev;
    struct rtpse_mcu_uart *ctx;
    unsigned int baud;
    int ret;
    ctx = devm_kzalloc(dev, sizeof(*ctx), GFP_KERNEL);
    if (!ctx)
    return -ENOMEM;
    ctx.serdev = serdev;
    ctx.pse.dev = dev;
    ctx.pse.pcdev.owner = THIS_MODULE;
    ctx.pse.transport = &rtpse_mcu_uart_transport_ops;
    init_completion(&ctx.rx_done);
    spin_lock_init(&ctx.rx_lock);
    serdev_device_set_drvdata(serdev, ctx);
    serdev_device_set_client_ops(serdev, &rtpse_mcu_uart_serdev_ops);
    ret = devm_serdev_device_open(dev, serdev);
    if (ret)
    return dev_err_probe(dev, ret, "failed to open serdev\n");
    fwnode_property_read_u32(dev_fwnode(dev), "current-speed", &speed);
    baud = serdev_device_set_baudrate(serdev, speed);
    if (baud != speed)
    dev_warn(dev, "could not set baudrate %u, controller uses %u\n",
    speed, baud);
    serdev_device_set_flow_control(serdev, false);
    ret = serdev_device_set_parity(serdev, SERDEV_PARITY_NONE);
    if (ret)
    dev_warn(dev, "could not set parity to none: %d\n", ret);
    return rtpse_mcu_register(&ctx.pse);
    }
    static const struct of_device_id rtpse_mcu_uart_of_match[] = {
    { .compatible = "realtek,pse-mcu-gen1", .data = &rtpse_mcu_gen1_data },
    { .compatible = "realtek,pse-mcu-gen2", .data = &rtpse_mcu_gen2_data },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, rtpse_mcu_uart_of_match);
    static struct serdev_device_driver rtpse_mcu_uart_driver = {
    .driver = {
    .name = "realtek-pse-mcu-uart",
    .of_match_table = rtpse_mcu_uart_of_match,
    },
    .probe  = rtpse_mcu_uart_probe,
    };
    module_serdev_device_driver(rtpse_mcu_uart_driver);
    MODULE_AUTHOR("Jonas Jelonek <jelonek.jonas@gmail.com>");
    MODULE_DESCRIPTION("Realtek PSE MCU driver (UART transport)");
    MODULE_LICENSE("GPL");
