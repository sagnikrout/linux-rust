//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/tty/serial/sh-sci-common.h
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

// Private port IDs
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SCI_PORT_TYPE {
    RSCI_PORT_SCIF16 = BIT(7) | 0,
    RSCI_PORT_SCIF32 = BIT(7) | 1,
    RSCI_PORT_SCIF32_SINGLE_TCLK = BIT(7) | 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SCI_CLKS {
    SCI_FCK,		/* Functional Clock */
    SCI_SCK,		/* Optional External Clock */
    SCI_BRG_INT,		/* Optional BRG Internal Clock Source */
    SCI_SCIF_CLK,		/* Optional BRG External Clock Source */
    SCI_FCK_DIV4,		/* Optional Functional Clock frequency-divided by 4 */
    SCI_FCK_DIV16,		/* Optional Functional Clock frequency-divided by 16 */
    SCI_FCK_DIV64,		/* Optional Functional Clock frequency-divided by 64 */
    SCI_NUM_CLKS
}

// Offsets into the sci_port->irqs array
// Bit x set means sampling rate x + 1 is supported

extern "C" {
    pub fn sci_release_port(port: *mut uart_port);
}
extern "C" {
    pub fn sci_request_port(port: *mut uart_port) -> c_int;
}
extern "C" {
    pub fn sci_config_port(port: *mut uart_port, flags: c_int);
}
extern "C" {
    pub fn sci_verify_port(port: *mut uart_port, ser: *mut serial_struct) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct plat_sci_reg {
    pub offset: u8,
    pub size: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sci_port_params_bits {
    pub rxtx_enable: c_uint,
    pub te_clear: c_uint,
    pub poll_sent_bits: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sci_common_regs {
    pub status: c_uint,
    pub control: c_uint,
}

// The actual number of needed registers. This is used by sci only
pub const SCI_NR_REGS: c_int = 20;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sci_port_params {
    pub regs: [plat_sci_reg; SCI_NR_REGS],
    pub common_regs: *const sci_common_regs,
    pub param_bits: *const sci_port_params_bits,
    pub fifosize: c_uint,
    pub overrun_reg: c_uint,
    pub overrun_mask: c_uint,
    pub sampling_rate_mask: c_uint,
    pub error_mask: c_uint,
    pub error_clear: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sci_port_ops {
    pub reg): *mut *mut *mut u32 (read_reg)(struct uart_port port, int,
    pub value): *mut *mut *mut void (write_reg)(struct uart_port port, int reg, int,
    pub mask): *mut *mut *mut void (clear_SCxSR)(struct uart_port port, unsigned int,
    pub port): *mut *mut void (transmit_chars)(struct uart_port,
    pub port): *mut *mut void (receive_chars)(struct uart_port,
    pub c): *mut *mut *mut void (poll_put_char)(struct uart_port port, unsigned char,
    pub rx_trig): *mut *mut *mut int (set_rtrg)(struct uart_port port, int,
    pub port): *mut *mut int (rtrg_enabled)(struct uart_port,
    pub port): *mut *mut void (shutdown_complete)(struct uart_port,
    pub ctrl): *mut *mut *mut void (prepare_console_write)(struct uart_port port, u32,
    pub ctrl): *mut *mut *mut void (finish_console_write)(struct uart_port port, u32,
    pub port): *mut *mut void (console_save)(struct uart_port,
    pub port): *mut *mut void (console_restore)(struct uart_port,
    pub (*suspend_regs_size)(void): *mut usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sci_of_data {
    pub params: *const sci_port_params,
    pub uart_ops: *const uart_ops,
    pub ops: *const sci_port_ops,
    pub regtype: c_ushort,
    pub type: c_ushort,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sci_port {
    pub port: uart_port,
// Platform configuration
    pub params: *const sci_port_params,
    pub cfg: *const plat_sci_port,
    pub sampling_rate_mask: c_uint,
    pub reg_size: resource_size_t,
    pub gpios: *mut mctrl_gpios,
// Clocks
    pub clks: [*mut clk; SCI_NUM_CLKS],
    pub clk_rates: [c_ulong; SCI_NUM_CLKS],
    pub irqs: [c_int; SCIx_NR_IRQS],
    pub irqstr: [*mut c_char; SCIx_NR_IRQS],
    pub chan_tx: *mut dma_chan,
    pub chan_rx: *mut dma_chan,
    pub rstc: *mut reset_control,
    pub suspend_regs: *mut sci_suspend_regs,

    pub chan_tx_saved: *mut dma_chan,
    pub chan_rx_saved: *mut dma_chan,
    pub cookie_tx: dma_cookie_t,
    pub cookie_rx: [dma_cookie_t; 2],
    pub active_rx: dma_cookie_t,
    pub tx_dma_addr: dma_addr_t,
    pub tx_dma_len: c_uint,
    pub sg_rx: [scatterlist; 2],
    pub rx_buf: [*mut c_void; 2],
    pub buf_len_rx: usize,
    pub work_tx: work_struct,
    pub rx_timer: hrtimer,
    pub /: *mut *mut unsigned int rx_timeout; / microseconds,

    pub rx_frame: c_uint,
    pub rx_trigger: c_int,
    pub rx_fifo_timer: timer_list,
    pub rx_fifo_timeout: c_int,
    pub hscif_tot: u16,
    pub type: u8,
    pub regtype: u8,
    pub ops: *const sci_port_ops,
    pub has_rtscts: bool,
    pub autorts: bool,
    pub tx_occurred: bool,
}

extern "C" {
    pub fn sci_port_disable(sci_port: *mut sci_port);
}
extern "C" {
    pub fn sci_port_enable(sci_port: *mut sci_port);
}
extern "C" {
    pub fn sci_startup(port: *mut uart_port) -> c_int;
}
extern "C" {
    pub fn sci_shutdown(port: *mut uart_port);
}

extern "C" {
    pub fn scix_early_console_setup(device: *mut earlycon_device, data: *const sci_of_data) -> int __init;
}

