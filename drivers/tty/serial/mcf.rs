//! Automatically rewritten from C to Rust
//! Source: drivers/tty/serial/mcf.c
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
//
// mcf.c -- Freescale ColdFire UART driver
//
// (C) Copyright 2003-2007, Greg Ungerer <gerg@uclinux.org>
//

//
// Some boards implement the DTR/DCD lines using GPIO lines, most
// don't. Dummy out the access macros for those that don't. Those
// that do should define these macros somewhere in there board
// specific inlude files.
//

//
// Local per-uart structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcf_uart {
    pub port: uart_port,
    pub /: *mut *mut unsigned int sigs; / Local copy of line sigs,
    pub /: *mut *mut unsigned char imr; / Local IMR mirror,
}

//
#[no_mangle]
unsafe extern "C" fn mcf_tx_empty(port: *mut uart_port) -> c_uint {
    static unsigned int mcf_tx_empty(struct uart_port *port)
    {
    return (readb(port.membase + MCFUART_USR) & MCFUART_USR_TXEMPTY) ?
    TIOCSER_TEMT : 0;
    }
//
#[no_mangle]
unsafe extern "C" fn mcf_get_mctrl(port: *mut uart_port) -> c_uint {
    static unsigned int mcf_get_mctrl(struct uart_port *port)
    {
    struct mcf_uart *pp = container_of(port, struct mcf_uart, port);
    unsigned int sigs;
    sigs = (readb(port.membase + MCFUART_UIPR) & MCFUART_UIPR_CTS) ?
    0 : TIOCM_CTS;
    sigs |= (pp.sigs & TIOCM_RTS);
    sigs |= (mcf_getppdcd(port.line) ? TIOCM_CD : 0);
    sigs |= (mcf_getppdtr(port.line) ? TIOCM_DTR : 0);
    return sigs;
    }
//
#[no_mangle]
unsafe extern "C" fn mcf_set_mctrl(port: *mut uart_port, sigs: c_uint) {
    static void mcf_set_mctrl(struct uart_port *port, unsigned int sigs)
    {
    struct mcf_uart *pp = container_of(port, struct mcf_uart, port);
    pp.sigs = sigs;
    mcf_setppdtr(port.line, (sigs & TIOCM_DTR));
    if (sigs & TIOCM_RTS)
    writeb(MCFUART_UOP_RTS, port.membase + MCFUART_UOP1);
    else
    writeb(MCFUART_UOP_RTS, port.membase + MCFUART_UOP0);
    }
//
#[no_mangle]
unsafe extern "C" fn mcf_start_tx(port: *mut uart_port) {
    static void mcf_start_tx(struct uart_port *port)
    {
    struct mcf_uart *pp = container_of(port, struct mcf_uart, port);
    if (port.rs485.flags & SER_RS485_ENABLED) {
// Enable Transmitter
    writeb(MCFUART_UCR_TXENABLE, port.membase + MCFUART_UCR);
// Manually assert RTS
    writeb(MCFUART_UOP_RTS, port.membase + MCFUART_UOP1);
    }
    pp.imr |= MCFUART_UIR_TXREADY;
    writeb(pp.imr, port.membase + MCFUART_UIMR);
    }
//
#[no_mangle]
unsafe extern "C" fn mcf_stop_tx(port: *mut uart_port) {
    static void mcf_stop_tx(struct uart_port *port)
    {
    struct mcf_uart *pp = container_of(port, struct mcf_uart, port);
    pp.imr &= ~MCFUART_UIR_TXREADY;
    writeb(pp.imr, port.membase + MCFUART_UIMR);
    }
//
#[no_mangle]
unsafe extern "C" fn mcf_stop_rx(port: *mut uart_port) {
    static void mcf_stop_rx(struct uart_port *port)
    {
    struct mcf_uart *pp = container_of(port, struct mcf_uart, port);
    pp.imr &= ~MCFUART_UIR_RXREADY;
    writeb(pp.imr, port.membase + MCFUART_UIMR);
    }
//
#[no_mangle]
unsafe extern "C" fn mcf_break_ctl(port: *mut uart_port, break_state: c_int) {
    static void mcf_break_ctl(struct uart_port *port, int break_state)
    {
    unsigned long flags;
    uart_port_lock_irqsave(port, &flags);
    if (break_state == -1)
    writeb(MCFUART_UCR_CMDBREAKSTART, port.membase + MCFUART_UCR);
    else
    writeb(MCFUART_UCR_CMDBREAKSTOP, port.membase + MCFUART_UCR);
    uart_port_unlock_irqrestore(port, flags);
    }
//
#[no_mangle]
unsafe extern "C" fn mcf_startup(port: *mut uart_port) -> c_int {
    static int mcf_startup(struct uart_port *port)
    {
    struct mcf_uart *pp = container_of(port, struct mcf_uart, port);
    unsigned long flags;
    uart_port_lock_irqsave(port, &flags);
// Reset UART, get it into known state...
    writeb(MCFUART_UCR_CMDRESETRX, port.membase + MCFUART_UCR);
    writeb(MCFUART_UCR_CMDRESETTX, port.membase + MCFUART_UCR);
// Enable the UART transmitter and receiver
    writeb(MCFUART_UCR_RXENABLE | MCFUART_UCR_TXENABLE,
    port.membase + MCFUART_UCR);
// Enable RX interrupts now
    pp.imr = MCFUART_UIR_RXREADY;
    writeb(pp.imr, port.membase + MCFUART_UIMR);
    uart_port_unlock_irqrestore(port, flags);
    return 0;
    }
//
#[no_mangle]
unsafe extern "C" fn mcf_shutdown(port: *mut uart_port) {
    static void mcf_shutdown(struct uart_port *port)
    {
    struct mcf_uart *pp = container_of(port, struct mcf_uart, port);
    unsigned long flags;
    uart_port_lock_irqsave(port, &flags);
// Disable all interrupts now
    pp.imr = 0;
    writeb(pp.imr, port.membase + MCFUART_UIMR);
// Disable UART transmitter and receiver
    writeb(MCFUART_UCR_CMDRESETRX, port.membase + MCFUART_UCR);
    writeb(MCFUART_UCR_CMDRESETTX, port.membase + MCFUART_UCR);
    uart_port_unlock_irqrestore(port, flags);
    }
//
    static void mcf_set_termios(struct uart_port *port, struct ktermios *termios,
    const struct ktermios *old)
    {
    unsigned long flags;
    unsigned int baud, baudclk;

    unsigned int baudfr;

    unsigned char mr1, mr2;
    baud = uart_get_baud_rate(port, termios, old, 0, 230400);

    baudclk = (MCF_BUSCLK / baud) / 32;
    baudfr = (((MCF_BUSCLK / baud) + 1) / 2) % 16;

    baudclk = ((MCF_BUSCLK / baud) + 16) / 32;

    mr1 = MCFUART_MR1_RXIRQRDY | MCFUART_MR1_RXERRCHAR;
    mr2 = 0;
    switch (termios.c_cflag & CSIZE) {
    case CS5: mr1 |= MCFUART_MR1_CS5; break;
    case CS6: mr1 |= MCFUART_MR1_CS6; break;
    case CS7: mr1 |= MCFUART_MR1_CS7; break;
    case CS8:
    default:  mr1 |= MCFUART_MR1_CS8; break;
    }
    if (termios.c_cflag & PARENB) {
    if (termios.c_cflag & CMSPAR) {
    if (termios.c_cflag & PARODD)
    mr1 |= MCFUART_MR1_PARITYMARK;
    else
    mr1 |= MCFUART_MR1_PARITYSPACE;
    } else {
    if (termios.c_cflag & PARODD)
    mr1 |= MCFUART_MR1_PARITYODD;
    else
    mr1 |= MCFUART_MR1_PARITYEVEN;
    }
    } else {
    mr1 |= MCFUART_MR1_PARITYNONE;
    }
//
// FIXME: port->read_status_mask and port->ignore_status_mask
// need to be initialized based on termios settings for
// INPCK, IGNBRK, IGNPAR, PARMRK, BRKINT
//
    if (termios.c_cflag & CSTOPB)
    mr2 |= MCFUART_MR2_STOP2;
    else
    mr2 |= MCFUART_MR2_STOP1;
    if (termios.c_cflag & CRTSCTS) {
    mr1 |= MCFUART_MR1_RXRTS;
    mr2 |= MCFUART_MR2_TXCTS;
    }
    uart_port_lock_irqsave(port, &flags);
    if (port.rs485.flags & SER_RS485_ENABLED) {
    dev_dbg(port.dev, "Setting UART to RS485\n");
    mr2 |= MCFUART_MR2_TXRTS;
    }
    uart_update_timeout(port, termios.c_cflag, baud);
    writeb(MCFUART_UCR_CMDRESETRX, port.membase + MCFUART_UCR);
    writeb(MCFUART_UCR_CMDRESETTX, port.membase + MCFUART_UCR);
    writeb(MCFUART_UCR_CMDRESETMRPTR, port.membase + MCFUART_UCR);
    writeb(mr1, port.membase + MCFUART_UMR);
    writeb(mr2, port.membase + MCFUART_UMR);
    writeb((baudclk & 0xff00) >> 8, port.membase + MCFUART_UBG1);
    writeb((baudclk & 0xff), port.membase + MCFUART_UBG2);

    writeb((baudfr & 0x0f), port.membase + MCFUART_UFPD);

    writeb(MCFUART_UCSR_RXCLKTIMER | MCFUART_UCSR_TXCLKTIMER,
    port.membase + MCFUART_UCSR);
    writeb(MCFUART_UCR_RXENABLE | MCFUART_UCR_TXENABLE,
    port.membase + MCFUART_UCR);
    uart_port_unlock_irqrestore(port, flags);
    }
//
#[no_mangle]
unsafe extern "C" fn mcf_rx_chars(pp: *mut mcf_uart) {
    static void mcf_rx_chars(struct mcf_uart *pp)
    {
    struct uart_port *port = &pp.port;
    u8 status, ch, flag;
    while ((status = readb(port.membase + MCFUART_USR)) & MCFUART_USR_RXREADY) {
    ch = readb(port.membase + MCFUART_URB);
    flag = TTY_NORMAL;
    port.icount.rx++;
    if (status & MCFUART_USR_RXERR) {
    writeb(MCFUART_UCR_CMDRESETERR,
    port.membase + MCFUART_UCR);
    if (status & MCFUART_USR_RXBREAK) {
    port.icount.brk++;
    if (uart_handle_break(port))
    continue;
    } else if (status & MCFUART_USR_RXPARITY) {
    port.icount.parity++;
    } else if (status & MCFUART_USR_RXOVERRUN) {
    port.icount.overrun++;
    } else if (status & MCFUART_USR_RXFRAMING) {
    port.icount.frame++;
    }
    status &= port.read_status_mask;
    if (status & MCFUART_USR_RXBREAK)
    flag = TTY_BREAK;
#[no_mangle]
pub unsafe extern "C" fn if(MCFUART_USR_RXPARITY: status &) -> else {
    else if (status & MCFUART_USR_RXPARITY)
    flag = TTY_PARITY;
#[no_mangle]
pub unsafe extern "C" fn if(MCFUART_USR_RXFRAMING: status &) -> else {
    else if (status & MCFUART_USR_RXFRAMING)
    flag = TTY_FRAME;
    }
    if (uart_handle_sysrq_char(port, ch))
    continue;
    uart_insert_char(port, status, MCFUART_USR_RXOVERRUN, ch, flag);
    }
    tty_flip_buffer_push(&port.state.port);
    }
//
#[no_mangle]
unsafe extern "C" fn mcf_tx_chars(pp: *mut mcf_uart) {
    static void mcf_tx_chars(struct mcf_uart *pp)
    {
    struct uart_port *port = &pp.port;
    bool pending;
    u8 ch;
    pending = uart_port_tx(port, ch,
    readb(port.membase + MCFUART_USR) & MCFUART_USR_TXREADY,
    writeb(ch, port.membase + MCFUART_UTB));
// Disable TX to negate RTS automatically
    if (!pending && (port.rs485.flags & SER_RS485_ENABLED))
    writeb(MCFUART_UCR_TXDISABLE, port.membase + MCFUART_UCR);
    }
//
#[no_mangle]
unsafe extern "C" fn mcf_interrupt(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t mcf_interrupt(int irq, void *data)
    {
    struct uart_port *port = data;
    struct mcf_uart *pp = container_of(port, struct mcf_uart, port);
    unsigned int isr;
    let mut ret: irqreturn_t = IRQ_NONE;
    isr = readb(port.membase + MCFUART_UISR) & pp.imr;
    uart_port_lock(port);
    if (isr & MCFUART_UIR_RXREADY) {
    mcf_rx_chars(pp);
    ret = IRQ_HANDLED;
    }
    if (isr & MCFUART_UIR_TXREADY) {
    mcf_tx_chars(pp);
    ret = IRQ_HANDLED;
    }
    uart_port_unlock(port);
    return ret;
    }
//
#[no_mangle]
unsafe extern "C" fn mcf_config_port(port: *mut uart_port, flags: c_int) {
    static void mcf_config_port(struct uart_port *port, int flags)
    {
    port.type = PORT_MCF;
    port.fifosize = MCFUART_TXFIFOSIZE;
// Clear mask, so no surprise interrupts.
    writeb(0, port.membase + MCFUART_UIMR);
    if (request_irq(port.irq, mcf_interrupt, 0, "UART", port))
    printk(KERN_ERR "MCF: unable to attach ColdFire UART %d "
    "interrupt vector=%d\n", port.line, port.irq);
    }
//
    static const char *mcf_type(struct uart_port *port)
    {
    return (port.type == PORT_MCF) ? "ColdFire UART" : core::ptr::null_mut();
    }
//
#[no_mangle]
unsafe extern "C" fn mcf_request_port(port: *mut uart_port) -> c_int {
    static int mcf_request_port(struct uart_port *port)
    {
// UARTs always present
    return 0;
    }
//
#[no_mangle]
unsafe extern "C" fn mcf_release_port(port: *mut uart_port) {
    static void mcf_release_port(struct uart_port *port)
    {
// Nothing to release...
    }
//
#[no_mangle]
unsafe extern "C" fn mcf_verify_port(port: *mut uart_port, ser: *mut serial_struct) -> c_int {
    static int mcf_verify_port(struct uart_port *port, struct serial_struct *ser)
    {
    if ((ser.type != PORT_UNKNOWN) && (ser.type != PORT_MCF))
    return -EINVAL;
    return 0;
    }
//
// Enable or disable the RS485 support
    static int mcf_config_rs485(struct uart_port *port, struct ktermios *termios,
    struct serial_rs485 *rs485)
    {
    unsigned char mr1, mr2;
// Get mode registers
    mr1 = readb(port.membase + MCFUART_UMR);
    mr2 = readb(port.membase + MCFUART_UMR);
    if (rs485.flags & SER_RS485_ENABLED) {
    dev_dbg(port.dev, "Setting UART to RS485\n");
// Automatically negate RTS after TX completes
    mr2 |= MCFUART_MR2_TXRTS;
    } else {
    dev_dbg(port.dev, "Setting UART to RS232\n");
    mr2 &= ~MCFUART_MR2_TXRTS;
    }
    writeb(mr1, port.membase + MCFUART_UMR);
    writeb(mr2, port.membase + MCFUART_UMR);
    return 0;
    }
    static const struct serial_rs485 mcf_rs485_supported = {
    .flags = SER_RS485_ENABLED | SER_RS485_RTS_AFTER_SEND,
    };
//
// Define the basic serial functions we support.
//
    static const struct uart_ops mcf_uart_ops = {
    .tx_empty	= mcf_tx_empty,
    .get_mctrl	= mcf_get_mctrl,
    .set_mctrl	= mcf_set_mctrl,
    .start_tx	= mcf_start_tx,
    .stop_tx	= mcf_stop_tx,
    .stop_rx	= mcf_stop_rx,
    .break_ctl	= mcf_break_ctl,
    .startup	= mcf_startup,
    .shutdown	= mcf_shutdown,
    .set_termios	= mcf_set_termios,
    .type		= mcf_type,
    .request_port	= mcf_request_port,
    .release_port	= mcf_release_port,
    .config_port	= mcf_config_port,
    .verify_port	= mcf_verify_port,
    };
    static struct mcf_uart mcf_ports[10];

//

//
#[no_mangle]
unsafe extern "C" fn mcf_console_putc(co: *mut console, c: c_char) {
    static void mcf_console_putc(struct console *co, const char c)
    {
    struct uart_port *port = &(mcf_ports + co.index).port;
    int i;
    for (i = 0; (i < 0x10000); i++) {
    if (readb(port.membase + MCFUART_USR) & MCFUART_USR_TXREADY)
    break;
    }
    writeb(c, port.membase + MCFUART_UTB);
    for (i = 0; (i < 0x10000); i++) {
    if (readb(port.membase + MCFUART_USR) & MCFUART_USR_TXREADY)
    break;
    }
    }
//
#[no_mangle]
unsafe extern "C" fn mcf_console_write(co: *mut console, s: *const c_char, count: c_uint) {
    static void mcf_console_write(struct console *co, const char *s, unsigned int count)
    {
    for (; (count); count--, s++) {
    mcf_console_putc(co, *s);
    if (*s == '\n')
    mcf_console_putc(co, '\r');
    }
    }
//
#[no_mangle]
unsafe extern "C" fn mcf_console_setup(co: *mut console, options: *mut c_char) -> int __init {
    static int __init mcf_console_setup(struct console *co, char *options)
    {
    struct uart_port *port;
    let mut baud: c_int = CONFIG_SERIAL_MCF_BAUDRATE;
    let mut bits: c_int = 8;
    let mut parity: c_int = 'n';
    let mut flow: c_int = 'n';
    if ((co.index < 0) || (co.index >= MCF_MAXPORTS))
    co.index = 0;
    port = &mcf_ports[co.index].port;
    if (port.membase == 0)
    return -ENODEV;
    if (options)
    uart_parse_options(options, &baud, &parity, &bits, &flow);
    return uart_set_options(port, co, baud, parity, bits, flow);
    }
//
    static struct uart_driver mcf_driver;
    static struct console mcf_console = {
    .name		= "ttyS",
    .write		= mcf_console_write,
    .device		= uart_console_device,
    .setup		= mcf_console_setup,
    .flags		= CON_PRINTBUFFER,
    .index		= -1,
    .data		= &mcf_driver,
    };
#[no_mangle]
unsafe extern "C" fn mcf_console_init() -> int __init {
    static int __init mcf_console_init(void)
    {
    register_console(&mcf_console);
    return 0;
    }
    console_initcall(mcf_console_init);

//

//

//

//
// Define the mcf UART driver structure.
//
    static struct uart_driver mcf_driver = {
    .owner		= THIS_MODULE,
    .driver_name	= "mcf",
    .dev_name	= "ttyS",
    .major		= TTY_MAJOR,
    .minor		= 64,
    .nr		= MCF_MAXPORTS,
    .cons		= MCF_CONSOLE,
    };
//
#[no_mangle]
unsafe extern "C" fn mcf_probe(pdev: *mut platform_device) -> c_int {
    static int mcf_probe(struct platform_device *pdev)
    {
    struct mcf_platform_uart *platp = dev_get_platdata(&pdev.dev);
    struct uart_port *port;
    int i;
    for (i = 0; ((i < MCF_MAXPORTS) && (platp[i].mapbase)); i++) {
    port = &mcf_ports[i].port;
    port.line = i;
    port.type = PORT_MCF;
    port.mapbase = platp[i].mapbase;
    port.membase = (platp[i].membase) ? platp[i].membase :
    (unsigned char __iomem *) platp[i].mapbase;
    port.dev = &pdev.dev;
    port.iotype = SERIAL_IO_MEM;
    port.irq = platp[i].irq;
    port.uartclk = MCF_BUSCLK;
    port.ops = &mcf_uart_ops;
    port.flags = UPF_BOOT_AUTOCONF;
    port.rs485_config = mcf_config_rs485;
    port.rs485_supported = mcf_rs485_supported;
    port.has_sysrq = IS_ENABLED(CONFIG_SERIAL_MCF_CONSOLE);
    uart_add_one_port(&mcf_driver, port);
    }
    return 0;
    }
//
#[no_mangle]
unsafe extern "C" fn mcf_remove(pdev: *mut platform_device) {
    static void mcf_remove(struct platform_device *pdev)
    {
    struct uart_port *port;
    int i;
    for (i = 0; (i < MCF_MAXPORTS); i++) {
    port = &mcf_ports[i].port;
    if (port)
    uart_remove_one_port(&mcf_driver, port);
    }
    }
//
    static struct platform_driver mcf_platform_driver = {
    .probe		= mcf_probe,
    .remove		= mcf_remove,
    .driver		= {
    .name	= "mcfuart",
    },
    };
//
#[no_mangle]
unsafe extern "C" fn mcf_init() -> int __init {
    static int __init mcf_init(void)
    {
    int rc;
    printk("ColdFire internal UART serial driver\n");
    rc = uart_register_driver(&mcf_driver);
    if (rc)
    return rc;
    rc = platform_driver_register(&mcf_platform_driver);
    if (rc) {
    uart_unregister_driver(&mcf_driver);
    return rc;
    }
    return 0;
    }
//
#[no_mangle]
unsafe extern "C" fn mcf_exit() -> void __exit {
    static void __exit mcf_exit(void)
    {
    platform_driver_unregister(&mcf_platform_driver);
    uart_unregister_driver(&mcf_driver);
    }
//
    module_init(mcf_init);
    module_exit(mcf_exit);
    MODULE_AUTHOR("Greg Ungerer <gerg@uclinux.org>");
    MODULE_DESCRIPTION("Freescale ColdFire UART driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:mcfuart");
//
