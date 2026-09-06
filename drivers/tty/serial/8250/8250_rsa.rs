//! Automatically rewritten from C to Rust
//! Source: drivers/tty/serial/8250/8250_rsa.c
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

pub const PORT_RSA_MAX: c_int = 4;
    static unsigned long probe_rsa[PORT_RSA_MAX];
    static unsigned int probe_rsa_count;
    static const struct uart_ops *core_port_base_ops;
#[no_mangle]
unsafe extern "C" fn rsa8250_request_resource(up: *mut uart_8250_port) -> c_int {
    static int rsa8250_request_resource(struct uart_8250_port *up)
    {
    struct uart_port *port = &up.port;
    unsigned long start;
    unsigned int size;
    if (!uart_iotype_io(port.iotype))
    return -EINVAL;
    start = UART_RSA_BASE << port.regshift;
    start += port.iobase;
    size = 8 << port.regshift;
    if (!request_region(start, size, "serial-rsa"))
    return -EBUSY;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rsa8250_release_resource(up: *mut uart_8250_port) {
    static void rsa8250_release_resource(struct uart_8250_port *up)
    {
    struct uart_port *port = &up.port;
    unsigned long offset;
    unsigned int size;
    if (!uart_iotype_io(port.iotype))
    return;
    offset = UART_RSA_BASE << port.regshift;
    size = 8 << port.regshift;
    release_region(port.iobase + offset, size);
    }
#[no_mangle]
unsafe extern "C" fn univ8250_config_port(port: *mut uart_port, flags: c_int) {
    static void univ8250_config_port(struct uart_port *port, int flags)
    {
    struct uart_8250_port *up = up_to_u8250p(port);
    unsigned int i;
    up.probe &= ~UART_PROBE_RSA;
    if (port.type == PORT_RSA) {
    if (rsa8250_request_resource(up) == 0)
    up.probe |= UART_PROBE_RSA;
    } else if (flags & UART_CONFIG_TYPE) {
    for (i = 0; i < probe_rsa_count; i++) {
    if (probe_rsa[i] == up.port.iobase) {
    if (rsa8250_request_resource(up) == 0)
    up.probe |= UART_PROBE_RSA;
    break;
    }
    }
    }
    core_port_base_ops.config_port(port, flags);
    if (port.type != PORT_RSA && up.probe & UART_PROBE_RSA)
    rsa8250_release_resource(up);
    }
#[no_mangle]
unsafe extern "C" fn univ8250_request_port(port: *mut uart_port) -> c_int {
    static int univ8250_request_port(struct uart_port *port)
    {
    struct uart_8250_port *up = up_to_u8250p(port);
    int ret;
    ret = core_port_base_ops.request_port(port);
    if (ret == 0 && port.type == PORT_RSA) {
    ret = rsa8250_request_resource(up);
    if (ret < 0)
    core_port_base_ops.release_port(port);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn univ8250_release_port(port: *mut uart_port) {
    static void univ8250_release_port(struct uart_port *port)
    {
    struct uart_8250_port *up = up_to_u8250p(port);
    if (port.type == PORT_RSA)
    rsa8250_release_resource(up);
    core_port_base_ops.release_port(port);
    }
//
// It is not allowed to directly reference any symbols from 8250.ko here as
// that would result in a dependency loop between the 8250.ko and
// 8250_base.ko modules. This function is called from 8250.ko and is used to
// break the symbolic dependency cycle. Anything that is needed from 8250.ko
// has to be passed as pointers to this function which then can adjust those
// variables on 8250.ko side or store them locally as needed.
//
#[no_mangle]
pub unsafe extern "C" fn univ8250_rsa_support(ops: *mut uart_ops, core_ops: *const uart_ops) {
    void univ8250_rsa_support(struct uart_ops *ops, const struct uart_ops *core_ops)
    {
    core_port_base_ops = core_ops;
    ops.config_port  = univ8250_config_port;
    ops.request_port = univ8250_request_port;
    ops.release_port = univ8250_release_port;
    }
    EXPORT_SYMBOL_FOR_MODULES(univ8250_rsa_support, "8250");
    module_param_hw_array(probe_rsa, ulong, ioport, &probe_rsa_count, 0444);
    MODULE_PARM_DESC(probe_rsa, "Probe I/O ports for RSA");
//
// Attempts to turn on the RSA FIFO.  Returns zero on failure.
// We set the port uart clock rate if we succeed.
//
#[no_mangle]
unsafe extern "C" fn __rsa_enable(up: *mut uart_8250_port) -> c_int {
    static int __rsa_enable(struct uart_8250_port *up)
    {
    unsigned char mode;
    int result;
    mode = serial_in(up, UART_RSA_MSR);
    result = mode & UART_RSA_MSR_FIFO;
    if (!result) {
    serial_out(up, UART_RSA_MSR, mode | UART_RSA_MSR_FIFO);
    mode = serial_in(up, UART_RSA_MSR);
    result = mode & UART_RSA_MSR_FIFO;
    }
    if (result)
    up.port.uartclk = SERIAL_RSA_BAUD_BASE * 16;
    return result;
    }
//
// If this is an RSA port, see if we can kick it up to the higher speed clock.
//
#[no_mangle]
pub unsafe extern "C" fn rsa_enable(up: *mut uart_8250_port) {
    void rsa_enable(struct uart_8250_port *up)
    {
    if (up.port.type != PORT_RSA)
    return;
    if (up.port.uartclk != SERIAL_RSA_BAUD_BASE * 16) {
    guard(uart_port_lock_irq)(&up.port);
    __rsa_enable(up);
    }
    if (up.port.uartclk == SERIAL_RSA_BAUD_BASE * 16)
    serial_out(up, UART_RSA_FRR, 0);
    }
//
// Attempts to turn off the RSA FIFO and resets the RSA board back to 115kbps compat mode. It is
// unknown why interrupts were disabled in here. However, the caller is expected to preserve this
// behaviour by grabbing the spinlock before calling this function.
//
#[no_mangle]
pub unsafe extern "C" fn rsa_disable(up: *mut uart_8250_port) {
    void rsa_disable(struct uart_8250_port *up)
    {
    unsigned char mode;
    int result;
    if (up.port.type != PORT_RSA)
    return;
    if (up.port.uartclk != SERIAL_RSA_BAUD_BASE * 16)
    return;
    guard(uart_port_lock_irq)(&up.port);
    mode = serial_in(up, UART_RSA_MSR);
    result = !(mode & UART_RSA_MSR_FIFO);
    if (!result) {
    serial_out(up, UART_RSA_MSR, mode & ~UART_RSA_MSR_FIFO);
    mode = serial_in(up, UART_RSA_MSR);
    result = !(mode & UART_RSA_MSR_FIFO);
    }
    if (result)
    up.port.uartclk = SERIAL_RSA_BAUD_BASE_LO * 16;
    }
#[no_mangle]
pub unsafe extern "C" fn rsa_autoconfig(up: *mut uart_8250_port) {
    void rsa_autoconfig(struct uart_8250_port *up)
    {
// Only probe for RSA ports if we got the region.
    if (up.port.type != PORT_16550A)
    return;
    if (!(up.probe & UART_PROBE_RSA))
    return;
    if (__rsa_enable(up))
    up.port.type = PORT_RSA;
    }
#[no_mangle]
pub unsafe extern "C" fn rsa_reset(up: *mut uart_8250_port) {
    void rsa_reset(struct uart_8250_port *up)
    {
    if (up.port.type != PORT_RSA)
    return;
    serial_out(up, UART_RSA_FRR, 0);
    }
