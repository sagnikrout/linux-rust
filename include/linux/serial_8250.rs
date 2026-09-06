//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/serial_8250.h
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
//
// linux/include/linux/serial_8250.h
//
// Copyright (C) 2004 Russell King
//

//
// This is the platform device platform_data structure
//
// @mapsize:	Port size for ioremap()
// @bugs:	Port bugs
//
// @dl_read: ``u32 ()(struct uart_8250_port *up)``
//
// UART divisor latch read.
//
// @dl_write: ``void ()(struct uart_8250_port *up, u32 value)``
//
// Write @value into UART divisor latch.
//
// Locking: Caller holds port's lock.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct plat_serial8250_port {
    pub /: *mut *mut unsigned long iobase; / io base address,
    pub /: *mut *mut *mut void __iomem membase; / ioremap cookie or NULL,
    pub /: *mut *mut resource_size_t mapbase; / resource base,
    pub mapsize: resource_size_t,
    pub /: *mut *mut unsigned int uartclk; / UART clock rate,
    pub /: *mut *mut unsigned int irq; / interrupt number,
    pub /: *mut *mut unsigned long irqflags; / request_irq flags,
    pub private_data: *mut c_void,
    pub /: *mut *mut unsigned char regshift; / register shift,
    pub /: *mut *mut *mut unsigned char iotype; / UPIO_,
    pub hub6: c_uchar,
    pub /: *mut *mut unsigned char has_sysrq; / supports magic SysRq,
    pub /: *mut *mut unsigned int type; / If UPF_FIXED_TYPE,
    pub /: *mut *mut *mut upf_t flags; / UPF_ flags,
    pub /: *mut *mut u16 bugs; / port bugs,
    pub offset): *mut *mut *mut u32 (serial_in)(struct uart_port , unsigned int,
    pub val): *mut *mut *mut void (serial_out)(struct uart_port , unsigned int offset, u32,
    pub up): *mut *mut u32 (dl_read)(struct uart_8250_port,
    pub value): *mut *mut *mut void (dl_write)(struct uart_8250_port up, u32,
    pub old): *const ktermios,
    pub ): *mut ktermios,
    pub ): *mut *mut unsigned int (get_mctrl)(struct uart_port,
    pub ): *mut *mut int (handle_irq)(struct uart_port,
    pub old): unsigned,
    pub ): *mut *mut void (handle_break)(struct uart_port,
}

//
// Allocate 8250 platform device IDs.  Nothing is implied by
// the numbering here, except for the legacy entry being -1.
//
// 8250 core driver operations
//
// @setup_irq()		Setup irq handling. The universal 8250 driver links this
// port to the irq chain. Other drivers may @request_irq().
// @release_irq()	Undo irq handling. The universal 8250 driver unlinks
// the port from the irq chain.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uart_8250_ops {
    pub ): *mut *mut int (setup_irq)(struct uart_8250_port,
    pub ): *mut *mut void (release_irq)(struct uart_8250_port,
    pub ): *mut *mut void (setup_timer)(struct uart_8250_port,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uart_8250_em485 {
    pub /: *mut *mut hrtimer start_tx_timer; / "rs485 start tx" timer,
    pub /: *mut *mut hrtimer stop_tx_timer; / "rs485 stop tx" timer,
    pub /: *mut *mut *mut hrtimer active_timer; / pointer to active timer,
    pub /: *mut *mut *mut uart_8250_port port; / for hrtimer callbacks,
    pub /: *mut *mut unsigned int tx_stopped:1; / tx is currently stopped,
}

//
// This should be used by drivers which want to register
// their own 8250 ports without registering their own
// platform device.  Using these will make your driver
// dependent on the 8250 driver.
//
// @dl_read: ``u32 ()(struct uart_8250_port *port)``
//
// UART divisor latch read.
//
// @dl_write: ``void ()(struct uart_8250_port *port, u32 value)``
//
// Write @value into UART divisor latch.
//
// Locking: Caller holds port's lock.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uart_8250_port {
    pub port: uart_port,
    pub /: *mut *mut timer_list timer; / "no irq" timer,
    pub /: *mut *mut list_head list; / ports on this IRQ,
    pub /: *mut *mut u32 capabilities; / port capabilities,
    pub /: *mut *mut u16 bugs; / port bugs,
    pub /: *mut *mut unsigned int tx_loadsz; / transmit fifo load size,
    pub acr: c_uchar,
    pub fcr: c_uchar,
    pub ier: c_uchar,
    pub lcr: c_uchar,
    pub mcr: c_uchar,
    pub /: *mut *mut unsigned char cur_iotype; / Running I/O type,
    pub rpm_tx_active: c_uint,
    pub sleep: *mut *mut unsigned char canary; / non-zero during system,
// if no_console_suspend
//
    pub probe: c_uchar,
    pub gpios: *mut mctrl_gpios,

//
// Some bits in registers are cleared on a read, so they must
// be saved whenever the register is read but the bits will not
// be immediately processed.
//

    pub lsr_saved_flags: u16,
    pub lsr_save_mask: u16,
//
// Track when a console line has been fully written to the
// hardware, i.e. true when the most recent byte written to
// UART_TX by the console was '\n'.
//
    pub console_line_ended: bool,
// Allow queuing irq_work for MSR handling
    pub console_msr_work_allow: bool,

    pub msr_saved_flags: c_uchar,
    pub console_msr_work: irq_work,
    pub dma: *mut uart_8250_dma,
    pub ops: *const uart_8250_ops,
// 8250 specific callbacks
    pub up): *mut *mut u32 (dl_read)(struct uart_8250_port,
    pub value): *mut *mut *mut void (dl_write)(struct uart_8250_port up, u32,
    pub em485: *mut uart_8250_em485,
    pub toggle_ier): *mut *mut *mut void (rs485_start_tx)(struct uart_8250_port up, bool,
    pub toggle_ier): *mut *mut *mut void (rs485_stop_tx)(struct uart_8250_port up, bool,
// Serial port overrun backoff
    pub overrun_backoff: delayed_work,
    pub overrun_backoff_time_ms: u32,
}

extern "C" {
    pub fn container_of(_arg: up, uart_8250_port: struct, _arg: port) -> return;
}
extern "C" {
    pub fn serial8250_register_8250_port(: *const uart_8250_port) -> c_int;
}
extern "C" {
    pub fn serial8250_unregister_port(line: c_int);
}
extern "C" {
    pub fn serial8250_suspend_port(line: c_int);
}
extern "C" {
    pub fn serial8250_resume_port(line: c_int);
}
extern "C" {
    pub fn early_serial_setup(port: *mut uart_port) -> c_int;
}
extern "C" {
    pub fn early_serial8250_setup(device: *mut earlycon_device, options: *const c_char) -> c_int;
}
extern "C" {
    pub fn serial8250_update_uartclk(port: *mut uart_port, uartclk: c_uint);
}
extern "C" {
    pub fn serial8250_do_set_ldisc(port: *mut uart_port, termios: *mut ktermios);
}
extern "C" {
    pub fn serial8250_do_get_mctrl(port: *mut uart_port) -> c_uint;
}
extern "C" {
    pub fn serial8250_do_startup(port: *mut uart_port) -> c_int;
}
extern "C" {
    pub fn serial8250_do_shutdown(port: *mut uart_port);
}
extern "C" {
    pub fn serial8250_do_set_mctrl(port: *mut uart_port, mctrl: c_uint);
}
extern "C" {
    pub fn serial8250_do_break_ctl(port: *mut uart_port, break_state: c_int);
}
extern "C" {
    pub fn fsl8250_handle_irq(port: *mut uart_port) -> c_int;
}
extern "C" {
    pub fn serial8250_handle_irq_locked(port: *mut uart_port, iir: c_uint);
}
extern "C" {
    pub fn serial8250_handle_irq(port: *mut uart_port, iir: c_uint) -> c_int;
}
extern "C" {
    pub fn serial8250_rx_chars(up: *mut uart_8250_port, lsr: u16) -> u16;
}
extern "C" {
    pub fn serial8250_read_char(up: *mut uart_8250_port, lsr: u16);
}
extern "C" {
    pub fn serial8250_tx_chars(up: *mut uart_8250_port);
}
extern "C" {
    pub fn serial8250_modem_status(up: *mut uart_8250_port) -> c_uint;
}
extern "C" {
    pub fn serial8250_init_port(up: *mut uart_8250_port);
}
extern "C" {
    pub fn serial8250_set_defaults(up: *mut uart_8250_port);
}
extern "C" {
    pub fn serial8250_console_setup(port: *mut uart_port, options: *mut c_char, probe: bool) -> c_int;
}
extern "C" {
    pub fn serial8250_console_exit(port: *mut uart_port) -> c_int;
}

extern "C" {
    pub fn rt288x_setup(p: *mut uart_port) -> c_int;
}
extern "C" {
    pub fn au_platform_setup(p: *mut plat_serial8250_port) -> c_int;
}

