//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/serial_core.h
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
// linux/drivers/char/serial_core.h
//
// Copyright (C) 2000 Deep Blue Solutions Ltd.
//

//
// struct uart_ops -- interface between serial_core and the driver
//
// This structure describes all the operations that can be done on the
// physical hardware.
//
// @tx_empty: ``unsigned int ()(struct uart_port *port)``
//
// This function tests whether the transmitter fifo and shifter for the
// @port is empty. If it is empty, this function should return
// %TIOCSER_TEMT, otherwise return 0. If the port does not support this
// operation, then it should return %TIOCSER_TEMT.
//
// Locking: none.
// Interrupts: caller dependent.
// This call must not sleep
//
// @set_mctrl: ``void ()(struct uart_port *port, unsigned int mctrl)``
//
// This function sets the modem control lines for @port to the state
// described by @mctrl. The relevant bits of @mctrl are:
//
// - %TIOCM_RTS	RTS signal.
// - %TIOCM_DTR	DTR signal.
// - %TIOCM_OUT1	OUT1 signal.
// - %TIOCM_OUT2	OUT2 signal.
// - %TIOCM_LOOP	Set the port into loopback mode.
//
// If the appropriate bit is set, the signal should be driven
// active.  If the bit is clear, the signal should be driven
// inactive.
//
// Locking: @port->lock taken.
// Interrupts: locally disabled.
// This call must not sleep
//
// @get_mctrl: ``unsigned int ()(struct uart_port *port)``
//
// Returns the current state of modem control inputs of @port. The state
// of the outputs should not be returned, since the core keeps track of
// their state. The state information should include:
//
// - %TIOCM_CAR	state of DCD signal
// - %TIOCM_CTS	state of CTS signal
// - %TIOCM_DSR	state of DSR signal
// - %TIOCM_RI	state of RI signal
//
// The bit is set if the signal is currently driven active.  If
// the port does not support CTS, DCD or DSR, the driver should
// indicate that the signal is permanently active. If RI is
// not available, the signal should not be indicated as active.
//
// Locking: @port->lock taken.
// Interrupts: locally disabled.
// This call must not sleep
//
// @stop_tx: ``void ()(struct uart_port *port)``
//
// Stop transmitting characters. This might be due to the CTS line
// becoming inactive or the tty layer indicating we want to stop
// transmission due to an %XOFF character.
//
// The driver should stop transmitting characters as soon as possible.
//
// Locking: @port->lock taken.
// Interrupts: locally disabled.
// This call must not sleep
//
// @start_tx: ``void ()(struct uart_port *port)``
//
// Start transmitting characters.
//
// Locking: @port->lock taken.
// Interrupts: locally disabled.
// This call must not sleep
//
// @throttle: ``void ()(struct uart_port *port)``
//
// Notify the serial driver that input buffers for the line discipline are
// close to full, and it should somehow signal that no more characters
// should be sent to the serial port.
// This will be called only if hardware assisted flow control is enabled.
//
// Locking: serialized with @unthrottle() and termios modification by the
// tty layer.
//
// @unthrottle: ``void ()(struct uart_port *port)``
//
// Notify the serial driver that characters can now be sent to the serial
// port without fear of overrunning the input buffers of the line
// disciplines.
//
// This will be called only if hardware assisted flow control is enabled.
//
// Locking: serialized with @throttle() and termios modification by the
// tty layer.
//
// @send_xchar: ``void ()(struct uart_port *port, char ch)``
//
// Transmit a high priority character, even if the port is stopped. This
// is used to implement XON/XOFF flow control and tcflow(). If the serial
// driver does not implement this function, the tty core will append the
// character to the circular buffer and then call start_tx() / stop_tx()
// to flush the data out.
//
// Do not transmit if @ch == '\0' (%__DISABLED_CHAR).
//
// Locking: none.
// Interrupts: caller dependent.
//
// @start_rx: ``void ()(struct uart_port *port)``
//
// Start receiving characters.
//
// Locking: @port->lock taken.
// Interrupts: locally disabled.
// This call must not sleep
//
// @stop_rx: ``void ()(struct uart_port *port)``
//
// Stop receiving characters; the @port is in the process of being closed.
//
// Locking: @port->lock taken.
// Interrupts: locally disabled.
// This call must not sleep
//
// @enable_ms: ``void ()(struct uart_port *port)``
//
// Enable the modem status interrupts.
//
// This method may be called multiple times. Modem status interrupts
// should be disabled when the @shutdown() method is called.
//
// Locking: @port->lock taken.
// Interrupts: locally disabled.
// This call must not sleep
//
// @break_ctl: ``void ()(struct uart_port *port, int ctl)``
//
// Control the transmission of a break signal. If @ctl is nonzero, the
// break signal should be transmitted. The signal should be terminated
// when another call is made with a zero @ctl.
//
// Locking: caller holds tty_port->mutex
//
// @startup: ``int ()(struct uart_port *port)``
//
// Grab any interrupt resources and initialise any low level driver state.
// Enable the port for reception. It should not activate RTS nor DTR;
// this will be done via a separate call to @set_mctrl().
//
// This method will only be called when the port is initially opened.
//
// Locking: port_sem taken.
// Interrupts: globally disabled.
//
// @shutdown: ``void ()(struct uart_port *port)``
//
// Disable the @port, disable any break condition that may be in effect,
// and free any interrupt resources. It should not disable RTS nor DTR;
// this will have already been done via a separate call to @set_mctrl().
//
// Drivers must not access @port->state once this call has completed.
//
// This method will only be called when there are no more users of this
// @port.
//
// Locking: port_sem taken.
// Interrupts: caller dependent.
//
// @flush_buffer: ``void ()(struct uart_port *port)``
//
// Flush any write buffers, reset any DMA state and stop any ongoing DMA
// transfers.
//
// This will be called whenever the @port->state->xmit circular buffer is
// cleared.
//
// Locking: @port->lock taken.
// Interrupts: locally disabled.
// This call must not sleep
//
// @set_termios: ``void ()(struct uart_port *port, struct ktermios *new,
// struct ktermios *old)``
//
// Change the @port parameters, including word length, parity, stop bits.
// Update @port->read_status_mask and @port->ignore_status_mask to
// indicate the types of events we are interested in receiving. Relevant
// ktermios::c_cflag bits are:
//
// - %CSIZE - word size
// - %CSTOPB - 2 stop bits
// - %PARENB - parity enable
// - %PARODD - odd parity (when %PARENB is in force)
// - %ADDRB - address bit (changed through uart_port::rs485_config()).
// - %CREAD - enable reception of characters (if not set, still receive
// characters from the port, but throw them away).
// - %CRTSCTS - if set, enable CTS status change reporting.
// - %CLOCAL - if not set, enable modem status change reporting.
//
// Relevant ktermios::c_iflag bits are:
//
// - %INPCK - enable frame and parity error events to be passed to the TTY
// layer.
// - %BRKINT / %PARMRK - both of these enable break events to be passed to
// the TTY layer.
// - %IGNPAR - ignore parity and framing errors.
// - %IGNBRK - ignore break errors. If %IGNPAR is also set, ignore overrun
// errors as well.
//
// The interaction of the ktermios::c_iflag bits is as follows (parity
// error given as an example):
//
// ============ ======= ======= =========================================
// Parity error INPCK   IGNPAR
// ============ ======= ======= =========================================
// n/a	     0	     n/a     character received, marked as %TTY_NORMAL
// None	     1	     n/a     character received, marked as %TTY_NORMAL
// Yes	     1	     0	     character received, marked as %TTY_PARITY
// Yes	     1	     1	     character discarded
// ============ ======= ======= =========================================
//
// Other flags may be used (eg, xon/xoff characters) if your hardware
// supports hardware "soft" flow control.
//
// Locking: caller holds tty_port->mutex
// Interrupts: caller dependent.
// This call must not sleep
//
// @set_ldisc: ``void ()(struct uart_port *port, struct ktermios *termios)``
//
// Notifier for discipline change. See
// Documentation/driver-api/tty/tty_ldisc.rst.
//
// Locking: caller holds tty_port->mutex
//
// @pm: ``void ()(struct uart_port *port, unsigned int state,
// unsigned int oldstate)``
//
// Perform any power management related activities on the specified @port.
// @state indicates the new state (defined by enum uart_pm_state),
// @oldstate indicates the previous state.
//
// This function should not be used to grab any resources.
//
// This will be called when the @port is initially opened and finally
// closed, except when the @port is also the system console. This will
// occur even if %CONFIG_PM is not set.
//
// Locking: none.
// Interrupts: caller dependent.
//
// @type: ``const char *()(struct uart_port *port)``
//
// Return a pointer to a string constant describing the specified @port,
// or return %NULL, in which case the string 'unknown' is substituted.
//
// Locking: none.
// Interrupts: caller dependent.
//
// @release_port: ``void ()(struct uart_port *port)``
//
// Release any memory and IO region resources currently in use by the
// @port.
//
// Locking: none.
// Interrupts: caller dependent.
//
// @request_port: ``int ()(struct uart_port *port)``
//
// Request any memory and IO region resources required by the port. If any
// fail, no resources should be registered when this function returns, and
// it should return -%EBUSY on failure.
//
// Locking: none.
// Interrupts: caller dependent.
//
// @config_port: ``void ()(struct uart_port *port, int type)``
//
// Perform any autoconfiguration steps required for the @port. @type
// contains a bit mask of the required configuration. %UART_CONFIG_TYPE
// indicates that the port requires detection and identification.
// @port->type should be set to the type found, or %PORT_UNKNOWN if no
// port was detected.
//
// %UART_CONFIG_IRQ indicates autoconfiguration of the interrupt signal,
// which should be probed using standard kernel autoprobing techniques.
// This is not necessary on platforms where ports have interrupts
// internally hard wired (eg, system on a chip implementations).
//
// Locking: none.
// Interrupts: caller dependent.
//
// @verify_port: ``int ()(struct uart_port *port,
// struct serial_struct *serinfo)``
//
// Verify the new serial port information contained within @serinfo is
// suitable for this port type.
//
// Locking: none.
// Interrupts: caller dependent.
//
// @ioctl: ``int ()(struct uart_port *port, unsigned int cmd,
// unsigned long arg)``
//
// Perform any port specific IOCTLs. IOCTL commands must be defined using
// the standard numbering system found in <asm/ioctl.h>.
//
// Locking: none.
// Interrupts: caller dependent.
//
// @poll_init: ``int ()(struct uart_port *port)``
//
// Called by kgdb to perform the minimal hardware initialization needed to
// support @poll_put_char() and @poll_get_char(). Unlike @startup(), this
// should not request interrupts.
//
// Locking: %tty_mutex and tty_port->mutex taken.
// Interrupts: n/a.
//
// @poll_put_char: ``void ()(struct uart_port *port, unsigned char ch)``
//
// Called by kgdb to write a single character @ch directly to the serial
// @port. It can and should block until there is space in the TX FIFO.
//
// Locking: none.
// Interrupts: caller dependent.
// This call must not sleep
//
// @poll_get_char: ``int ()(struct uart_port *port)``
//
// Called by kgdb to read a single character directly from the serial
// port. If data is available, it should be returned; otherwise the
// function should return %NO_POLL_CHAR immediately.
//
// Locking: none.
// Interrupts: caller dependent.
// This call must not sleep
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uart_ops {
    pub ): *mut *mut unsigned int (tx_empty)(struct uart_port,
    pub mctrl): *mut *mut *mut void (set_mctrl)(struct uart_port , unsigned int,
    pub ): *mut *mut unsigned int (get_mctrl)(struct uart_port,
    pub ): *mut *mut void (stop_tx)(struct uart_port,
    pub ): *mut *mut void (start_tx)(struct uart_port,
    pub ): *mut *mut void (throttle)(struct uart_port,
    pub ): *mut *mut void (unthrottle)(struct uart_port,
    pub ch): *mut *mut *mut void (send_xchar)(struct uart_port , char,
    pub ): *mut *mut void (stop_rx)(struct uart_port,
    pub ): *mut *mut void (start_rx)(struct uart_port,
    pub ): *mut *mut void (enable_ms)(struct uart_port,
    pub ctl): *mut *mut *mut void (break_ctl)(struct uart_port , int,
    pub ): *mut *mut int (startup)(struct uart_port,
    pub ): *mut *mut void (shutdown)(struct uart_port,
    pub ): *mut *mut void (flush_buffer)(struct uart_port,
    pub old): *const ktermios,
    pub ): *mut *mut *mut void (set_ldisc)(struct uart_port , struct ktermios,
    pub oldstate): c_uint,
    pub ): *const *const *const char (type)(struct uart_port,
    pub ): *mut *mut void (release_port)(struct uart_port,
    pub ): *mut *mut int (request_port)(struct uart_port,
    pub int): *mut *mut *mut void (config_port)(struct uart_port ,,
    pub ): *mut *mut *mut int (verify_port)(struct uart_port , struct serial_struct,
    pub long): *mut *mut *mut int (ioctl)(struct uart_port , unsigned int, unsigned,

    pub ): *mut *mut int (poll_init)(struct uart_port,
    pub char): *mut *mut *mut void (poll_put_char)(struct uart_port , unsigned,
    pub ): *mut *mut int (poll_get_char)(struct uart_port,

}

pub const NO_POLL_CHAR: c_uint = 0x00ff0000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uart_icount {
    pub cts: __u32,
    pub dsr: __u32,
    pub rng: __u32,
    pub dcd: __u32,
    pub rx: __u32,
    pub tx: __u32,
    pub frame: __u32,
    pub overrun: __u32,
    pub parity: __u32,
    pub brk: __u32,
    pub buf_overrun: __u32,
}

pub type upf_t = u64 ;
pub type upstat_t = u32;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum uart_iotype {
    UPIO_UNKNOWN	= -1,
    UPIO_PORT	= SERIAL_IO_PORT,	/* 8b I/O port access */
    UPIO_HUB6	= SERIAL_IO_HUB6,	/* Hub6 ISA card */
    UPIO_MEM	= SERIAL_IO_MEM,	/* driver-specific */
    UPIO_MEM32	= SERIAL_IO_MEM32,	/* 32b little endian */
    UPIO_AU		= SERIAL_IO_AU,		/* Au1x00 and RT288x type IO */
    UPIO_TSI	= SERIAL_IO_TSI,	/* Tsi108/109 type IO */
    UPIO_MEM32BE	= SERIAL_IO_MEM32BE,	/* 32b big endian */
    UPIO_MEM16	= SERIAL_IO_MEM16,	/* 16b little endian */
    UPIO_BUS	= SERIAL_IO_BUS,	/* Serial bus I/O access (ex: SPI, I2C) */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uart_port {
    pub /: *mut *mut spinlock_t lock; / port lock,
    pub /: *mut *mut unsigned long iobase; / in/out[bwl],
    pub /: *mut *mut *mut unsigned char __iomem membase; / read/write[bwl],
    pub offset): *mut *mut *mut u32 (serial_in)(struct uart_port , unsigned int,
    pub val): *mut *mut *mut void (serial_out)(struct uart_port , unsigned int offset, u32,
    pub old): *const ktermios,
    pub ): *mut ktermios,
    pub ): *mut *mut unsigned int (get_mctrl)(struct uart_port,
    pub int): *mut *mut *mut void (set_mctrl)(struct uart_port , unsigned,
    pub frac): *mut c_uint,
    pub quot_frac): c_uint,
    pub port): *mut *mut int (get_rxtrig)(struct uart_port,
    pub bytes): *mut *mut *mut int (set_rxtrig)(struct uart_port port, unsigned char,
    pub port): *mut *mut int (startup)(struct uart_port,
    pub port): *mut *mut void (shutdown)(struct uart_port,
    pub port): *mut *mut void (throttle)(struct uart_port,
    pub port): *mut *mut void (unthrottle)(struct uart_port,
    pub break_state): *mut *mut *mut void (break_ctl)(struct uart_port port, int,
    pub ): *mut *mut int (handle_irq)(struct uart_port,
    pub old): c_uint,
    pub ): *mut *mut void (handle_break)(struct uart_port,
    pub rs485): *mut serial_rs485,
    pub iso7816): *mut serial_iso7816,
    pub /: *mut *mut unsigned int ctrl_id; / optional serial core controller id,
    pub /: *mut *mut unsigned int port_id; / optional serial core port id,
    pub /: *mut *mut unsigned int irq; / irq number,
    pub /: *mut *mut unsigned long irqflags; / irq flags,
    pub /: *mut *mut unsigned int uartclk; / base uart clock,
    pub /: *mut *mut unsigned int fifosize; / tx fifo size,
    pub /: *mut *mut unsigned char x_char; / xon/xoff char,
    pub /: *mut *mut unsigned char regshift; / reg offset shift,
    pub /: *mut *mut unsigned char quirks; / internal quirks,
// internal quirks must be updated while holding port mutex

    pub /: *mut *mut uart_iotype iotype; / io access style,
    pub /: *mut *mut unsigned int read_status_mask; / driver specific,
    pub /: *mut *mut unsigned int ignore_status_mask; / driver specific,
    pub /: *mut *mut *mut uart_state state; / pointer to parent state,
    pub /: *mut *mut uart_icount icount; / statistics,
    pub /: *mut *mut *mut console cons; / console, if any,
// flags must be updated while holding port mutex
    pub flags: upf_t,
//
// These flags must be equivalent to the flags defined in
// include/uapi/linux/tty_flags.h which are the userspace definitions
// assigned from the serial_struct flags in uart_set_info()
// [for bit definitions in the UPF_CHANGE_MASK]
//
// Bits [0..ASYNCB_LAST_USER] are userspace defined/visible/changeable
// The remaining bits are serial-core specific and not modifiable by
// userspace.
//

pub const UPF_FOURPORT: c_int = 0;

// Port has hardware-assisted h/w flow control

// Port has hardware-assisted s/w flow control

// Deprecated: use uart_set_cons_flow_enabled()/uart_cons_flow_enabled() instead.

// The exact UART type is known and should not be probed.

pub const __UPF_CHANGE_MASK: c_uint = 0x17fff;

//
// Must hold termios_rwsem, port mutex and port lock to change;
// can hold any one lock to read.
//
    pub status: upstat_t,

    pub /: *mut *mut bool hw_stopped; / sw-assisted CTS flow state,
    pub /: *mut *mut bool cons_flow; / user specified console flow control,
    pub /: *mut *mut unsigned int mctrl; / current modem ctrl settings,
    pub /: *mut *mut unsigned int frame_time; / frame timing in ns,
    pub /: *mut *mut unsigned int type; / port type,
    pub ops: *const uart_ops,
    pub custom_divisor: c_uint,
    pub /: *mut *mut unsigned int line; / port index,
    pub minor: c_uint,
    pub /: *mut *mut resource_size_t mapbase; / for ioremap,
    pub mapsize: resource_size_t,
    pub /: *mut *mut *mut device dev; / serial port physical parent device,
    pub /: *mut *mut *mut serial_port_device port_dev; / serial core port device,
    pub /: *mut *mut unsigned long sysrq; / sysrq timeout,
    pub /: *mut *mut u8 sysrq_ch; / char for sysrq,
    pub has_sysrq: c_uchar,
    pub /: *mut *mut unsigned char sysrq_seq; / index in sysrq_toggle_seq,
    pub /: *mut *mut unsigned char hub6; / this should be in the 8250 driver,
    pub suspended: c_uchar,
    pub console_reinit: c_uchar,
    pub /: *const *const *const char name; / port name,
    pub /: *mut *mut *mut attribute_group attr_group; / port specific attributes,
    pub /: *const *const *const *const attribute_group tty_groups; / all attributes (serial core use only),
    pub rs485: serial_rs485,
    pub /: *mut *mut serial_rs485 rs485_supported; / Supported mask for serial_rs485,
    pub /: *mut *mut *mut gpio_desc rs485_term_gpio; / enable RS485 bus termination,
    pub /: *mut *mut *mut gpio_desc rs485_rx_during_tx_gpio; / Output GPIO that sets the state of RS485 RX during TX,
    pub iso7816: serial_iso7816,
    pub /: *mut *mut *mut void private_data; / generic platform data pointer,
}

//
// Only for console->device_lock()/_unlock() callbacks and internal
// port lock wrapper synchronization.
//
// Only for console->device_lock()/_unlock() callbacks and internal
// port lock wrapper synchronization.
//
// uart_port_set_cons - Safely set the @cons field for a uart
// @up:		The uart port to set
// @con:	The new console to set to
//
// This function must be used to set @up->cons. It uses the port lock to
// synchronize with the port lock wrappers in order to ensure that the console
// cannot change or disappear while another context is holding the port lock.
//
// Only for internal port lock wrapper usage.
//
// @up->cons is only modified under the port lock. Therefore it is
// certain that it cannot disappear here.
//
// @up->cons->node is added/removed from the console list under the
// port lock. Therefore it is certain that the registration status
// cannot change here, thus @up->cons->flags can be read directly.
//
// Only for internal port lock wrapper usage.
extern "C" {
    pub fn nbcon_device_try_acquire(_arg: up->cons) -> return;
}
// Only for internal port lock wrapper usage.
//
// uart_port_lock - Lock the UART port
// @up:		Pointer to UART port structure
//
// uart_port_lock_irq - Lock the UART port and disable interrupts
// @up:		Pointer to UART port structure
//
// uart_port_lock_irqsave - Lock the UART port, save and disable interrupts
// @up:		Pointer to UART port structure
// @flags:	Pointer to interrupt flags storage
//
// uart_port_trylock - Try to lock the UART port
// @up:		Pointer to UART port structure
//
// Returns: True if lock was acquired, false otherwise
//
// uart_port_trylock_irqsave - Try to lock the UART port, save and disable interrupts
// @up:		Pointer to UART port structure
// @flags:	Pointer to interrupt flags storage
//
// Returns: True if lock was acquired, false otherwise
//
// uart_port_unlock - Unlock the UART port
// @up:		Pointer to UART port structure
//
// uart_port_unlock_irq - Unlock the UART port and re-enable interrupts
// @up:		Pointer to UART port structure
//
// uart_port_unlock_irqrestore - Unlock the UART port, restore interrupts
// @up:		Pointer to UART port structure
// @flags:	The saved interrupt flags for restore
//
// enum uart_pm_state - power states for UARTs
// @UART_PM_STATE_ON: UART is powered, up and operational
// @UART_PM_STATE_OFF: UART is powered off
// @UART_PM_STATE_UNDEFINED: sentinel
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum uart_pm_state {
    UART_PM_STATE_ON = 0,
    UART_PM_STATE_OFF = 3, /* number taken from ACPI */
    UART_PM_STATE_UNDEFINED,
}

//
// This is the state information which is persistent across opens.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uart_state {
    pub port: tty_port,
    pub pm_state: uart_pm_state,
    pub refcount: core::sync::atomic::AtomicI32,
    pub remove_wait: wait_queue_head_t,
    pub uart_port: *mut uart_port,
}

// number of characters left in xmit buffer before we ask for more
pub const WAKEUP_CHARS: c_int = 256;
//
// uart_xmit_advance - Advance xmit buffer and account Tx'ed chars
// @up: uart_port structure describing the port
// @chars: number of characters sent
//
// This function advances the tail of circular xmit buffer by the number of
// @chars transmitted and handles accounting of transmitted bytes (into
// @up's icount.tx).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uart_driver {
    pub owner: *mut module,
    pub driver_name: *const c_char,
    pub dev_name: *const c_char,
    pub major: c_int,
    pub minor: c_int,
    pub nr: c_int,
    pub cons: *mut console,
//
// these are private; the low level driver should not
// touch these; they should be initialised to NULL
//
    pub state: *mut uart_state,
    pub tty_driver: *mut tty_driver,
}

extern "C" {
    pub fn uart_write_wakeup(port: *mut uart_port);
}
//
// enum UART_TX_FLAGS -- flags for uart_port_tx_flags()
//
// @UART_TX_NOSTOP: don't call port->ops->stop_tx() on empty buffer
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum UART_TX_FLAGS {
    UART_TX_NOSTOP = BIT(0),
}

//
// uart_port_tx_limited -- transmit helper for uart_port with count limiting
// @port: uart port
// @ch: variable to store a character to be written to the HW
// @count: a limit of characters to send
// @tx_ready: can HW accept more data function
// @put_char: function to write a character
// @tx_done: function to call after the loop is done
//
// This helper transmits characters from the xmit buffer to the hardware using
// @put_char(). It does so until @count characters are sent and while @tx_ready
// evaluates to true.
//
// Returns: the number of characters in the xmit buffer when done.
//
// The expression in macro parameters shall be designed as follows:
// * **tx_ready:** should evaluate to true if the HW can accept more data to
// be sent. This parameter can be %true, which means the HW is always ready.
// * **put_char:** shall write @ch to the device of @port.
// * **tx_done:** when the write loop is done, this can perform arbitrary
// action before potential invocation of ops->stop_tx() happens. If the
// driver does not need to do anything, use e.g. ({}).
//
// For all of them, @port->lock is held, interrupts are locally disabled and
// the expressions must not sleep.
//

//
// uart_port_tx_limited_flags -- transmit helper for uart_port with count limiting with flags
// @port: uart port
// @ch: variable to store a character to be written to the HW
// @flags: %UART_TX_NOSTOP or similar
// @count: a limit of characters to send
// @tx_ready: can HW accept more data function
// @put_char: function to write a character
// @tx_done: function to call after the loop is done
//
// See uart_port_tx_limited() for more details.
//

//
// uart_port_tx -- transmit helper for uart_port
// @port: uart port
// @ch: variable to store a character to be written to the HW
// @tx_ready: can HW accept more data function
// @put_char: function to write a character
//
// See uart_port_tx_limited() for more details.
//

//
// uart_port_tx_flags -- transmit helper for uart_port with flags
// @port: uart port
// @ch: variable to store a character to be written to the HW
// @flags: %UART_TX_NOSTOP or similar
// @tx_ready: can HW accept more data function
// @put_char: function to write a character
//
// See uart_port_tx_limited() for more details.
//

//
// Baud rate helpers.
//
extern "C" {
    pub fn uart_get_divisor(port: *mut uart_port, baud: c_uint) -> c_uint;
}
//
// Calculates FIFO drain time.
//
// Add .02 seconds of slop
extern "C" {
    pub fn max(_arg: nsecs_to_jiffies(fifo_timeout), _arg: 1UL) -> return;
}
// Base timer interval for polling
//
// Console helpers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct earlycon_device {
    pub con: *mut console,
    pub port: uart_port,
    pub /: *mut *mut char options[32]; / e.g., 115200n8,
    pub baud: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct earlycon_id {
    pub name: [c_char; 15],
    pub /: *mut *mut char name_term; / In case compiler didn't '\0' term name,
    pub compatible: [c_char; 128],
    pub options): *const *const *const int (setup)(struct earlycon_device , char,
}

extern "C" {
    pub fn setup_earlycon(buf: *mut c_char) -> c_int;
}

// Variant of uart_console_registered() when the console_list_lock is held.
extern "C" {
    pub fn uart_console(console_is_registered_locked(port->cons: port) &&) -> return;
}
extern "C" {
    pub fn uart_console(console_is_registered(port->cons: port) &&) -> return;
}
//
// Port/driver registration/removal
//
extern "C" {
    pub fn uart_register_driver(uart: *mut uart_driver) -> c_int;
}
extern "C" {
    pub fn uart_unregister_driver(uart: *mut uart_driver);
}
extern "C" {
    pub fn uart_add_one_port(reg: *mut uart_driver, port: *mut uart_port) -> c_int;
}
extern "C" {
    pub fn uart_remove_one_port(reg: *mut uart_driver, port: *mut uart_port);
}
extern "C" {
    pub fn uart_read_port_properties(port: *mut uart_port) -> c_int;
}
extern "C" {
    pub fn uart_read_and_validate_port_properties(port: *mut uart_port) -> c_int;
}
//
// Power Management
//
extern "C" {
    pub fn uart_suspend_port(reg: *mut uart_driver, port: *mut uart_port) -> c_int;
}
extern "C" {
    pub fn uart_resume_port(reg: *mut uart_driver, port: *mut uart_port) -> c_int;
}
//
// The following are helper functions for the low level drivers.
//
extern "C" {
    pub fn uart_handle_dcd_change(uport: *mut uart_port, active: bool);
}
extern "C" {
    pub fn uart_handle_cts_change(uport: *mut uart_port, active: bool);
}
extern "C" {
    pub fn uart_xchar_out(uport: *mut uart_port, offset: c_int);
}

extern "C" {
    pub fn uart_try_toggle_sysrq(port: *mut uart_port, ch: u8) -> bool;
}

//
// Variant of guard(uart_port_lock_irqsave) for IRQ handlers that may capture
// a SysRq character via uart_prepare_sysrq_char(). The destructor uses the
// sysrq-aware unlock helper so that a captured port->sysrq_ch is dispatched
// to handle_sysrq() on scope exit. The plain guard variant silently drops
// sysrq_ch and must not be used by callers that process RX.
//
// We do the SysRQ and SAK checking like this...
//

//
// UART_ENABLE_MS - determine if port should enable modem status irqs
//

extern "C" {
    pub fn uart_get_rs485_mode(port: *mut uart_port) -> c_int;
}
extern "C" {
    pub fn uart_get_ioinfos(port: *mut uart_port, buf: *mut c_char, size: usize);
}
extern "C" {
    pub fn uart_iotype_mmio(iotype: uart_iotype) -> bool;
}
extern "C" {
    pub fn uart_iotype_io(iotype: uart_iotype) -> bool;
}
