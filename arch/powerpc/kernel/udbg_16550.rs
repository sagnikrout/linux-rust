//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kernel/udbg_16550.c
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
// udbg for NS16550 compatible serial ports
//
// Copyright (C) 2001-2005 PPC 64 Team, IBM Corp
//

    extern u8 real_readb(volatile u8 __iomem  *addr);
    extern void real_writeb(u8 data, volatile u8 __iomem *addr);
    extern u8 real_205_readb(volatile u8 __iomem  *addr);
    extern void real_205_writeb(u8 data, volatile u8 __iomem *addr);
pub const UART_RBR: c_int = 0;
pub const UART_IER: c_int = 1;
pub const UART_FCR: c_int = 2;
pub const UART_LCR: c_int = 3;
pub const UART_MCR: c_int = 4;
pub const UART_LSR: c_int = 5;
pub const UART_MSR: c_int = 6;
pub const UART_SCR: c_int = 7;

pub const LSR_DR: c_uint = 0x01  /* Data ready */;
pub const LSR_OE: c_uint = 0x02  /* Overrun */;
pub const LSR_PE: c_uint = 0x04  /* Parity error */;
pub const LSR_FE: c_uint = 0x08  /* Framing error */;
pub const LSR_BI: c_uint = 0x10  /* Break */;
pub const LSR_THRE: c_uint = 0x20  /* Xmit holding register empty */;
pub const LSR_TEMT: c_uint = 0x40  /* Xmitter empty */;
pub const LSR_ERR: c_uint = 0x80  /* Error */;
pub const LCR_DLAB: c_uint = 0x80;
    static u8 (*udbg_uart_in)(unsigned int reg);
    static void (*udbg_uart_out)(unsigned int reg, u8 data);
#[no_mangle]
unsafe extern "C" fn udbg_uart_flush() {
    static void udbg_uart_flush(void)
    {
    if (!udbg_uart_in)
    return;
// wait for idle
    while ((udbg_uart_in(UART_LSR) & LSR_THRE) == 0)
    cpu_relax();
    }
#[no_mangle]
unsafe extern "C" fn udbg_uart_putc(c: c_char) {
    static void udbg_uart_putc(char c)
    {
    if (!udbg_uart_out)
    return;
    if (c == '\n')
    udbg_uart_putc('\r');
    udbg_uart_flush();
    udbg_uart_out(UART_THR, c);
    }
#[no_mangle]
unsafe extern "C" fn udbg_uart_getc_poll() -> c_int {
    static int udbg_uart_getc_poll(void)
    {
    if (!udbg_uart_in)
    return -1;
    if (!(udbg_uart_in(UART_LSR) & LSR_DR))
    return udbg_uart_in(UART_RBR);
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn udbg_uart_getc() -> c_int {
    static int udbg_uart_getc(void)
    {
    if (!udbg_uart_in)
    return -1;
// wait for char
    while (!(udbg_uart_in(UART_LSR) & LSR_DR))
    cpu_relax();
    return udbg_uart_in(UART_RBR);
    }
#[no_mangle]
unsafe extern "C" fn udbg_use_uart() -> void __init {
    static void __init udbg_use_uart(void)
    {
    udbg_putc = udbg_uart_putc;
    udbg_flush = udbg_uart_flush;
    udbg_getc = udbg_uart_getc;
    udbg_getc_poll = udbg_uart_getc_poll;
    }
#[no_mangle]
pub unsafe extern "C" fn udbg_uart_setup(speed: c_uint, clock: c_uint) -> void __init {
    void __init udbg_uart_setup(unsigned int speed, unsigned int clock)
    {
    unsigned int dll, base_bauds;
    if (!udbg_uart_out)
    return;
    if (clock == 0)
    clock = 1843200;
    if (speed == 0)
    speed = 9600;
    base_bauds = clock / 16;
    dll = base_bauds / speed;
    udbg_uart_out(UART_LCR, 0x00);
    udbg_uart_out(UART_IER, 0xff);
    udbg_uart_out(UART_IER, 0x00);
    udbg_uart_out(UART_LCR, LCR_DLAB);
    udbg_uart_out(UART_DLL, dll & 0xff);
    udbg_uart_out(UART_DLM, dll >> 8);
// 8 data, 1 stop, no parity
    udbg_uart_out(UART_LCR, 0x3);
// RTS/DTR
    udbg_uart_out(UART_MCR, 0x3);
// Clear & enable FIFOs
    udbg_uart_out(UART_FCR, 0x7);
    }
#[no_mangle]
pub unsafe extern "C" fn udbg_probe_uart_speed(clock: c_uint) -> unsigned int __init {
    unsigned int __init udbg_probe_uart_speed(unsigned int clock)
    {
    unsigned int dll, dlm, divisor, prescaler, speed;
    u8 old_lcr;
    old_lcr = udbg_uart_in(UART_LCR);
// select divisor latch registers.
    udbg_uart_out(UART_LCR, old_lcr | LCR_DLAB);
// now, read the divisor
    dll = udbg_uart_in(UART_DLL);
    dlm = udbg_uart_in(UART_DLM);
    divisor = dlm << 8 | dll;
// check prescaling
    if (udbg_uart_in(UART_MCR) & 0x80)
    prescaler = 4;
    else
    prescaler = 1;
// restore the LCR
    udbg_uart_out(UART_LCR, old_lcr);
// calculate speed
    speed = (clock / prescaler) / (divisor * 16);
// sanity check
    if (speed > (clock / 16))
    speed = 9600;
    return speed;
    }
    static union {
    unsigned char __iomem *mmio_base;
    unsigned long pio_base;
    } udbg_uart;
    let mut udbg_uart_stride: static unsigned int = 1;
#[no_mangle]
unsafe extern "C" fn udbg_uart_in_pio(reg: c_uint) -> u8 {
    static u8 udbg_uart_in_pio(unsigned int reg)
    {
    return inb(udbg_uart.pio_base + (reg * udbg_uart_stride));
    }
#[no_mangle]
unsafe extern "C" fn udbg_uart_out_pio(reg: c_uint, data: u8) {
    static void udbg_uart_out_pio(unsigned int reg, u8 data)
    {
    outb(data, udbg_uart.pio_base + (reg * udbg_uart_stride));
    }
#[no_mangle]
pub unsafe extern "C" fn udbg_uart_init_pio(port: c_ulong, stride: c_uint) -> void __init {
    void __init udbg_uart_init_pio(unsigned long port, unsigned int stride)
    {
    if (!port)
    return;
    udbg_uart.pio_base = port;
    udbg_uart_stride = stride;
    udbg_uart_in = udbg_uart_in_pio;
    udbg_uart_out = udbg_uart_out_pio;
    udbg_use_uart();
    }
#[no_mangle]
unsafe extern "C" fn udbg_uart_in_mmio(reg: c_uint) -> u8 {
    static u8 udbg_uart_in_mmio(unsigned int reg)
    {
    return in_8(udbg_uart.mmio_base + (reg * udbg_uart_stride));
    }
#[no_mangle]
unsafe extern "C" fn udbg_uart_out_mmio(reg: c_uint, data: u8) {
    static void udbg_uart_out_mmio(unsigned int reg, u8 data)
    {
    out_8(udbg_uart.mmio_base + (reg * udbg_uart_stride), data);
    }
#[no_mangle]
pub unsafe extern "C" fn udbg_uart_init_mmio(addr: *mut void __iomem, stride: c_uint) -> void __init {
    void __init udbg_uart_init_mmio(void __iomem *addr, unsigned int stride)
    {
    if (!addr)
    return;
    udbg_uart.mmio_base = addr;
    udbg_uart_stride = stride;
    udbg_uart_in = udbg_uart_in_mmio;
    udbg_uart_out = udbg_uart_out_mmio;
    udbg_use_uart();
    }

#[no_mangle]
unsafe extern "C" fn udbg_uart_in_pas(reg: c_uint) -> u8 {
    static u8 udbg_uart_in_pas(unsigned int reg)
    {
    return real_205_readb(UDBG_UART_PAS_ADDR + reg);
    }
#[no_mangle]
unsafe extern "C" fn udbg_uart_out_pas(reg: c_uint, val: u8) {
    static void udbg_uart_out_pas(unsigned int reg, u8 val)
    {
    real_205_writeb(val, UDBG_UART_PAS_ADDR + reg);
    }
#[no_mangle]
pub unsafe extern "C" fn udbg_init_pas_realmode() -> void __init {
    void __init udbg_init_pas_realmode(void)
    {
    udbg_uart_in = udbg_uart_in_pas;
    udbg_uart_out = udbg_uart_out_pas;
    udbg_use_uart();
    }

#[no_mangle]
unsafe extern "C" fn udbg_uart_in_44x_as1(reg: c_uint) -> u8 {
    static u8 udbg_uart_in_44x_as1(unsigned int reg)
    {
    return as1_readb((void __iomem *)PPC44x_EARLY_DEBUG_VIRTADDR + reg);
    }
#[no_mangle]
unsafe extern "C" fn udbg_uart_out_44x_as1(reg: c_uint, val: u8) {
    static void udbg_uart_out_44x_as1(unsigned int reg, u8 val)
    {
    as1_writeb(val, (void __iomem *)PPC44x_EARLY_DEBUG_VIRTADDR + reg);
    }
#[no_mangle]
pub unsafe extern "C" fn udbg_init_44x_as1() -> void __init {
    void __init udbg_init_44x_as1(void)
    {
    udbg_uart_in = udbg_uart_in_44x_as1;
    udbg_uart_out = udbg_uart_out_44x_as1;
    udbg_use_uart();
    }

    static void __iomem *udbg_uart_early_addr;
#[no_mangle]
pub unsafe extern "C" fn udbg_init_debug_16550() -> void __init {
    void __init udbg_init_debug_16550(void)
    {
    udbg_uart_early_addr = early_ioremap(CONFIG_PPC_EARLY_DEBUG_16550_PHYSADDR, 0x1000);
    udbg_uart_init_mmio(udbg_uart_early_addr, CONFIG_PPC_EARLY_DEBUG_16550_STRIDE);
    }
#[no_mangle]
unsafe extern "C" fn udbg_init_debug_16550_ioremap() -> int __init {
    static int __init udbg_init_debug_16550_ioremap(void)
    {
    void __iomem *addr;
    if (!udbg_uart_early_addr)
    return 0;
    addr = ioremap(CONFIG_PPC_EARLY_DEBUG_16550_PHYSADDR, 0x1000);
    if (WARN_ON(!addr))
    return -ENOMEM;
    udbg_uart_init_mmio(addr, CONFIG_PPC_EARLY_DEBUG_16550_STRIDE);
    early_iounmap(udbg_uart_early_addr, 0x1000);
    udbg_uart_early_addr = core::ptr::null_mut();
    return 0;
    }
    early_initcall(udbg_init_debug_16550_ioremap);
