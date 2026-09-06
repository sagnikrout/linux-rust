//! Automatically rewritten from C to Rust
//! Source: drivers/tty/serdev/serdev-ttyport.c
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
// Copyright (C) 2016-2017 Linaro Ltd., Rob Herring <robh@kernel.org>
//

pub const SERPORT_ACTIVE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct serport {
    pub port: *mut tty_port,
    pub tty: *mut tty_struct,
    pub tty_drv: *mut tty_driver,
    pub tty_idx: c_int,
    pub flags: c_ulong,
}

//
// Callback functions from the tty port.
//
    static size_t ttyport_receive_buf(struct tty_port *port, const u8 *cp,
    const u8 *fp, size_t count)
    {
    struct serdev_controller *ctrl = port.client_data;
    struct serport *serport = serdev_controller_get_drvdata(ctrl);
    size_t ret;
    if (!test_bit(SERPORT_ACTIVE, &serport.flags))
    return 0;
    ret = serdev_controller_receive_buf(ctrl, cp, count);
    dev_WARN_ONCE(&ctrl.dev, ret > count,
    "receive_buf returns %zu (count = %zu)\n",
    ret, count);
    if (ret > count)
    return count;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ttyport_write_wakeup(port: *mut tty_port) {
    static void ttyport_write_wakeup(struct tty_port *port)
    {
    struct serdev_controller *ctrl = port.client_data;
    struct serport *serport = serdev_controller_get_drvdata(ctrl);
    struct tty_struct *tty;
    tty = tty_port_tty_get(port);
    if (!tty)
    return;
    if (test_and_clear_bit(TTY_DO_WRITE_WAKEUP, &tty.flags) &&
    test_bit(SERPORT_ACTIVE, &serport.flags))
    serdev_controller_write_wakeup(ctrl);
// Wake up any tty_wait_until_sent()
    wake_up_interruptible(&tty.write_wait);
    tty_kref_put(tty);
    }
    static const struct tty_port_client_operations client_ops = {
    .receive_buf = ttyport_receive_buf,
    .write_wakeup = ttyport_write_wakeup,
    };
//
// Callback functions from the serdev core.
//
#[no_mangle]
unsafe extern "C" fn ttyport_write_buf(ctrl: *mut serdev_controller, data: *const u8, len: usize) -> isize {
    static ssize_t ttyport_write_buf(struct serdev_controller *ctrl, const u8 *data, size_t len)
    {
    struct serport *serport = serdev_controller_get_drvdata(ctrl);
    struct tty_struct *tty = serport.tty;
    if (!test_bit(SERPORT_ACTIVE, &serport.flags))
    return 0;
    set_bit(TTY_DO_WRITE_WAKEUP, &tty.flags);
    return tty.ops.write(serport.tty, data, len);
    }
#[no_mangle]
unsafe extern "C" fn ttyport_write_flush(ctrl: *mut serdev_controller) {
    static void ttyport_write_flush(struct serdev_controller *ctrl)
    {
    struct serport *serport = serdev_controller_get_drvdata(ctrl);
    struct tty_struct *tty = serport.tty;
    tty_driver_flush_buffer(tty);
    }
#[no_mangle]
unsafe extern "C" fn ttyport_open(ctrl: *mut serdev_controller) -> c_int {
    static int ttyport_open(struct serdev_controller *ctrl)
    {
    struct serport *serport = serdev_controller_get_drvdata(ctrl);
    struct tty_struct *tty;
    struct ktermios ktermios;
    int ret;
    tty = tty_init_dev(serport.tty_drv, serport.tty_idx);
    if (IS_ERR(tty))
    return PTR_ERR(tty);
    serport.tty = tty;
    if (!tty.ops.open || !tty.ops.close) {
    ret = -ENODEV;
    goto err_unlock;
    }
    ret = tty.ops.open(serport.tty, core::ptr::null_mut());
    if (ret)
    goto err_close;
    tty_unlock(serport.tty);
// Bring the UART into a known 8 bits no parity hw fc state
    ktermios = tty.termios;
    ktermios.c_iflag &= ~(IGNBRK | BRKINT | PARMRK | ISTRIP |
    INLCR | IGNCR | ICRNL | IXON);
    ktermios.c_oflag &= ~OPOST;
    ktermios.c_lflag &= ~(ECHO | ECHONL | ICANON | ISIG | IEXTEN);
    ktermios.c_cflag &= ~(CSIZE | PARENB);
    ktermios.c_cflag |= CS8;
    ktermios.c_cflag |= CRTSCTS;
// Hangups are not supported so make sure to ignore carrier detect.
    ktermios.c_cflag |= CLOCAL;
    tty_set_termios(tty, &ktermios);
    set_bit(SERPORT_ACTIVE, &serport.flags);
    return 0;
    err_close:
    tty.ops.close(tty, core::ptr::null_mut());
    err_unlock:
    tty_unlock(tty);
    tty_release_struct(tty, serport.tty_idx);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ttyport_close(ctrl: *mut serdev_controller) {
    static void ttyport_close(struct serdev_controller *ctrl)
    {
    struct serport *serport = serdev_controller_get_drvdata(ctrl);
    struct tty_struct *tty = serport.tty;
    clear_bit(SERPORT_ACTIVE, &serport.flags);
    tty_lock(tty);
    if (tty.ops.close)
    tty.ops.close(tty, core::ptr::null_mut());
    tty_unlock(tty);
    tty_release_struct(tty, serport.tty_idx);
    }
#[no_mangle]
unsafe extern "C" fn ttyport_set_baudrate(ctrl: *mut serdev_controller, speed: c_uint) -> c_uint {
    static unsigned int ttyport_set_baudrate(struct serdev_controller *ctrl, unsigned int speed)
    {
    struct serport *serport = serdev_controller_get_drvdata(ctrl);
    struct tty_struct *tty = serport.tty;
    let mut ktermios: ktermios = tty.termios;
    ktermios.c_cflag &= ~CBAUD;
    tty_termios_encode_baud_rate(&ktermios, speed, speed);
// tty_set_termios() return not checked as it is always 0
    tty_set_termios(tty, &ktermios);
    return ktermios.c_ospeed;
    }
#[no_mangle]
unsafe extern "C" fn ttyport_set_flow_control(ctrl: *mut serdev_controller, enable: bool) {
    static void ttyport_set_flow_control(struct serdev_controller *ctrl, bool enable)
    {
    struct serport *serport = serdev_controller_get_drvdata(ctrl);
    struct tty_struct *tty = serport.tty;
    let mut ktermios: ktermios = tty.termios;
    if (enable)
    ktermios.c_cflag |= CRTSCTS;
    else
    ktermios.c_cflag &= ~CRTSCTS;
    tty_set_termios(tty, &ktermios);
    }
    static int ttyport_set_parity(struct serdev_controller *ctrl,
    enum serdev_parity parity)
    {
    struct serport *serport = serdev_controller_get_drvdata(ctrl);
    struct tty_struct *tty = serport.tty;
    let mut ktermios: ktermios = tty.termios;
    ktermios.c_cflag &= ~(PARENB | PARODD | CMSPAR);
    if (parity != SERDEV_PARITY_NONE) {
    ktermios.c_cflag |= PARENB;
    if (parity == SERDEV_PARITY_ODD)
    ktermios.c_cflag |= PARODD;
    }
    tty_set_termios(tty, &ktermios);
    if ((tty.termios.c_cflag & (PARENB | PARODD | CMSPAR)) !=
    (ktermios.c_cflag & (PARENB | PARODD | CMSPAR)))
    return -EINVAL;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ttyport_wait_until_sent(ctrl: *mut serdev_controller, timeout: c_long) {
    static void ttyport_wait_until_sent(struct serdev_controller *ctrl, long timeout)
    {
    struct serport *serport = serdev_controller_get_drvdata(ctrl);
    struct tty_struct *tty = serport.tty;
    tty_wait_until_sent(tty, timeout);
    }
#[no_mangle]
unsafe extern "C" fn ttyport_get_tiocm(ctrl: *mut serdev_controller) -> c_int {
    static int ttyport_get_tiocm(struct serdev_controller *ctrl)
    {
    struct serport *serport = serdev_controller_get_drvdata(ctrl);
    struct tty_struct *tty = serport.tty;
    if (!tty.ops.tiocmget)
    return -EOPNOTSUPP;
    return tty.ops.tiocmget(tty);
    }
#[no_mangle]
unsafe extern "C" fn ttyport_set_tiocm(ctrl: *mut serdev_controller, set: c_uint, clear: c_uint) -> c_int {
    static int ttyport_set_tiocm(struct serdev_controller *ctrl, unsigned int set, unsigned int clear)
    {
    struct serport *serport = serdev_controller_get_drvdata(ctrl);
    struct tty_struct *tty = serport.tty;
    if (!tty.ops.tiocmset)
    return -EOPNOTSUPP;
    return tty.ops.tiocmset(tty, set, clear);
    }
#[no_mangle]
unsafe extern "C" fn ttyport_break_ctl(ctrl: *mut serdev_controller, break_state: c_uint) -> c_int {
    static int ttyport_break_ctl(struct serdev_controller *ctrl, unsigned int break_state)
    {
    struct serport *serport = serdev_controller_get_drvdata(ctrl);
    struct tty_struct *tty = serport.tty;
    if (!tty.ops.break_ctl)
    return -EOPNOTSUPP;
    return tty.ops.break_ctl(tty, break_state);
    }
    static const struct serdev_controller_ops ctrl_ops = {
    .write_buf = ttyport_write_buf,
    .write_flush = ttyport_write_flush,
    .open = ttyport_open,
    .close = ttyport_close,
    .set_flow_control = ttyport_set_flow_control,
    .set_parity = ttyport_set_parity,
    .set_baudrate = ttyport_set_baudrate,
    .wait_until_sent = ttyport_wait_until_sent,
    .get_tiocm = ttyport_get_tiocm,
    .set_tiocm = ttyport_set_tiocm,
    .break_ctl = ttyport_break_ctl,
    };
    struct device *serdev_tty_port_register(struct tty_port *port,
    struct device *host,
    struct device *parent,
    struct tty_driver *drv, int idx)
    {
    struct serdev_controller *ctrl;
    struct serport *serport;
    int ret;
    if (!port || !drv || !parent)
    return ERR_PTR(-ENODEV);
    ctrl = serdev_controller_alloc(host, parent, sizeof(struct serport));
    if (!ctrl)
    return ERR_PTR(-ENOMEM);
    serport = serdev_controller_get_drvdata(ctrl);
    serport.port = port;
    serport.tty_idx = idx;
    serport.tty_drv = drv;
    ctrl.ops = &ctrl_ops;
    port.client_ops = &client_ops;
    port.client_data = ctrl;
    ret = serdev_controller_add(ctrl);
    if (ret)
    goto err_reset_data;
    dev_info(&ctrl.dev, "tty port %s%d registered\n", drv.name, idx);
    return &ctrl.dev;
    err_reset_data:
    port.client_data = core::ptr::null_mut();
    port.client_ops = &tty_port_default_client_ops;
    serdev_controller_put(ctrl);
    return ERR_PTR(ret);
    }
#[no_mangle]
pub unsafe extern "C" fn serdev_tty_port_unregister(port: *mut tty_port) -> c_int {
    int serdev_tty_port_unregister(struct tty_port *port)
    {
    struct serdev_controller *ctrl = port.client_data;
    struct serport *serport = serdev_controller_get_drvdata(ctrl);
    if (!serport)
    return -ENODEV;
    serdev_controller_remove(ctrl);
    port.client_data = core::ptr::null_mut();
    port.client_ops = &tty_port_default_client_ops;
    serdev_controller_put(ctrl);
    return 0;
    }
