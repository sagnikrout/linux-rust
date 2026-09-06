//! Automatically rewritten from C to Rust
//! Source: drivers/usb/serial/spcp8x5.c
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
// spcp8x5 USB to serial adaptor driver
//
// Copyright (C) 2010-2013 Johan Hovold (jhovold@gmail.com)
// Copyright (C) 2006 Linxb (xubin.lin@worldplus.com.cn)
// Copyright (C) 2006 S1 Corp.
//
// Original driver for 2.6.10 pl2303 driver by
// Greg Kroah-Hartman (greg@kroah.com)
// Changes for 2.6.20 by Harald Klein <hari@vt100.at>
//

pub const SPCP825_QUIRK_NO_UART_STATUS: c_uint = 0x01;
pub const SPCP825_QUIRK_NO_WORK_MODE: c_uint = 0x02;
pub const SPCP8x5_007_VID: c_uint = 0x04FC;
pub const SPCP8x5_007_PID: c_uint = 0x0201;
pub const SPCP8x5_008_VID: c_uint = 0x04fc;
pub const SPCP8x5_008_PID: c_uint = 0x0235;
pub const SPCP8x5_PHILIPS_VID: c_uint = 0x0471;
pub const SPCP8x5_PHILIPS_PID: c_uint = 0x081e;
pub const SPCP8x5_INTERMATIC_VID: c_uint = 0x04FC;
pub const SPCP8x5_INTERMATIC_PID: c_uint = 0x0204;
pub const SPCP8x5_835_VID: c_uint = 0x04fc;
pub const SPCP8x5_835_PID: c_uint = 0x0231;
    static const struct usb_device_id id_table[] = {
    { USB_DEVICE(SPCP8x5_PHILIPS_VID , SPCP8x5_PHILIPS_PID)},
    { USB_DEVICE(SPCP8x5_INTERMATIC_VID, SPCP8x5_INTERMATIC_PID)},
    { USB_DEVICE(SPCP8x5_835_VID, SPCP8x5_835_PID)},
    { USB_DEVICE(SPCP8x5_008_VID, SPCP8x5_008_PID)},
    { USB_DEVICE(SPCP8x5_007_VID, SPCP8x5_007_PID),
    .driver_info = SPCP825_QUIRK_NO_UART_STATUS |
    SPCP825_QUIRK_NO_WORK_MODE },
    { }					/* Terminating entry */
    };
    MODULE_DEVICE_TABLE(usb, id_table);
// spcp8x5 spec register define
pub const MCR_CONTROL_LINE_RTS: c_uint = 0x02;
pub const MCR_CONTROL_LINE_DTR: c_uint = 0x01;
pub const MCR_DTR: c_uint = 0x01;
pub const MCR_RTS: c_uint = 0x02;
pub const MSR_STATUS_LINE_DCD: c_uint = 0x80;
pub const MSR_STATUS_LINE_RI: c_uint = 0x40;
pub const MSR_STATUS_LINE_DSR: c_uint = 0x20;
pub const MSR_STATUS_LINE_CTS: c_uint = 0x10;
// verdor command here , we should define myself
pub const SET_DEFAULT: c_uint = 0x40;
pub const SET_DEFAULT_TYPE: c_uint = 0x20;
pub const SET_UART_FORMAT: c_uint = 0x40;
pub const SET_UART_FORMAT_TYPE: c_uint = 0x21;
pub const SET_UART_FORMAT_SIZE_5: c_uint = 0x00;
pub const SET_UART_FORMAT_SIZE_6: c_uint = 0x01;
pub const SET_UART_FORMAT_SIZE_7: c_uint = 0x02;
pub const SET_UART_FORMAT_SIZE_8: c_uint = 0x03;
pub const SET_UART_FORMAT_STOP_1: c_uint = 0x00;
pub const SET_UART_FORMAT_STOP_2: c_uint = 0x04;
pub const SET_UART_FORMAT_PAR_NONE: c_uint = 0x00;
pub const SET_UART_FORMAT_PAR_ODD: c_uint = 0x10;
pub const SET_UART_FORMAT_PAR_EVEN: c_uint = 0x30;
pub const SET_UART_FORMAT_PAR_MASK: c_uint = 0xD0;
pub const SET_UART_FORMAT_PAR_SPACE: c_uint = 0x90;
pub const GET_UART_STATUS_TYPE: c_uint = 0xc0;
pub const GET_UART_STATUS: c_uint = 0x22;
pub const GET_UART_STATUS_MSR: c_uint = 0x06;
pub const SET_UART_STATUS: c_uint = 0x40;
pub const SET_UART_STATUS_TYPE: c_uint = 0x23;
pub const SET_UART_STATUS_MCR: c_uint = 0x0004;
pub const SET_UART_STATUS_MCR_DTR: c_uint = 0x01;
pub const SET_UART_STATUS_MCR_RTS: c_uint = 0x02;
pub const SET_UART_STATUS_MCR_LOOP: c_uint = 0x10;
pub const SET_WORKING_MODE: c_uint = 0x40;
pub const SET_WORKING_MODE_TYPE: c_uint = 0x24;
pub const SET_WORKING_MODE_U2C: c_uint = 0x00;
pub const SET_WORKING_MODE_RS485: c_uint = 0x01;
pub const SET_WORKING_MODE_PDMA: c_uint = 0x02;
pub const SET_WORKING_MODE_SPP: c_uint = 0x03;
pub const SET_FLOWCTL_CHAR: c_uint = 0x40;
pub const SET_FLOWCTL_CHAR_TYPE: c_uint = 0x25;
pub const GET_VERSION: c_uint = 0xc0;
pub const GET_VERSION_TYPE: c_uint = 0x26;
pub const SET_REGISTER: c_uint = 0x40;
pub const SET_REGISTER_TYPE: c_uint = 0x27;
pub const GET_REGISTER: c_uint = 0xc0;
pub const GET_REGISTER_TYPE: c_uint = 0x28;
pub const SET_RAM: c_uint = 0x40;
pub const SET_RAM_TYPE: c_uint = 0x31;
pub const GET_RAM: c_uint = 0xc0;
pub const GET_RAM_TYPE: c_uint = 0x32;
// how come ???
pub const UART_STATE: c_uint = 0x08;
pub const UART_STATE_TRANSIENT_MASK: c_uint = 0x75;
pub const UART_DCD: c_uint = 0x01;
pub const UART_DSR: c_uint = 0x02;
pub const UART_BREAK_ERROR: c_uint = 0x04;
pub const UART_RING: c_uint = 0x08;
pub const UART_FRAME_ERROR: c_uint = 0x10;
pub const UART_PARITY_ERROR: c_uint = 0x20;
pub const UART_OVERRUN_ERROR: c_uint = 0x40;
pub const UART_CTS: c_uint = 0x80;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spcp8x5_private {
    pub quirks: unsigned,
    pub lock: spinlock_t,
    pub line_control: u8,
}

    static int spcp8x5_probe(struct usb_serial *serial,
    const struct usb_device_id *id)
    {
    usb_set_serial_data(serial, (void *)id.driver_info);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn spcp8x5_port_probe(port: *mut usb_serial_port) -> c_int {
    static int spcp8x5_port_probe(struct usb_serial_port *port)
    {
    let mut quirks: c_uint = (unsigned int)(unsigned long)usb_get_serial_data(port.serial);
    struct spcp8x5_private *priv;
    priv = kzalloc_obj(*priv);
    if (!priv)
    return -ENOMEM;
    spin_lock_init(&priv.lock);
    priv.quirks = quirks;
    usb_set_serial_port_data(port, priv);
    port.port.drain_delay = 256;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn spcp8x5_port_remove(port: *mut usb_serial_port) {
    static void spcp8x5_port_remove(struct usb_serial_port *port)
    {
    struct spcp8x5_private *priv;
    priv = usb_get_serial_port_data(port);
    kfree(priv);
    }
#[no_mangle]
unsafe extern "C" fn spcp8x5_set_ctrl_line(port: *mut usb_serial_port, mcr: u8) -> c_int {
    static int spcp8x5_set_ctrl_line(struct usb_serial_port *port, u8 mcr)
    {
    struct spcp8x5_private *priv = usb_get_serial_port_data(port);
    struct usb_device *dev = port.serial.dev;
    int retval;
    if (priv.quirks & SPCP825_QUIRK_NO_UART_STATUS)
    return -EPERM;
    retval = usb_control_msg(dev, usb_sndctrlpipe(dev, 0),
    SET_UART_STATUS_TYPE, SET_UART_STATUS,
    mcr, 0x04, core::ptr::null_mut(), 0, 100);
    if (retval != 0) {
    dev_err(&port.dev, "failed to set control lines: %d\n",
    retval);
    }
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn spcp8x5_get_msr(port: *mut usb_serial_port, status: *mut u8) -> c_int {
    static int spcp8x5_get_msr(struct usb_serial_port *port, u8 *status)
    {
    struct spcp8x5_private *priv = usb_get_serial_port_data(port);
    struct usb_device *dev = port.serial.dev;
    u8 *buf;
    int ret;
    if (priv.quirks & SPCP825_QUIRK_NO_UART_STATUS)
    return -EPERM;
    buf = kzalloc(1, GFP_KERNEL);
    if (!buf)
    return -ENOMEM;
    ret = usb_control_msg(dev, usb_rcvctrlpipe(dev, 0),
    GET_UART_STATUS, GET_UART_STATUS_TYPE,
    0, GET_UART_STATUS_MSR, buf, 1, 100);
    if (ret < 1) {
    dev_err(&port.dev, "failed to get modem status: %d\n", ret);
    if (ret >= 0)
    ret = -EIO;
    goto out;
    }
    dev_dbg(&port.dev, "0xc0:0x22:0:6  %d - 0x02%x\n", ret, *buf);
// status = *buf;
    ret = 0;
    out:
    kfree(buf);
    return ret;
    }
    static void spcp8x5_set_work_mode(struct usb_serial_port *port, u16 value,
    u16 index)
    {
    struct spcp8x5_private *priv = usb_get_serial_port_data(port);
    struct usb_device *dev = port.serial.dev;
    int ret;
    if (priv.quirks & SPCP825_QUIRK_NO_WORK_MODE)
    return;
    ret = usb_control_msg(dev, usb_sndctrlpipe(dev, 0),
    SET_WORKING_MODE_TYPE, SET_WORKING_MODE,
    value, index, core::ptr::null_mut(), 0, 100);
    dev_dbg(&port.dev, "value = %#x , index = %#x\n", value, index);
    if (ret < 0)
    dev_err(&port.dev, "failed to set work mode: %d\n", ret);
    }
#[no_mangle]
unsafe extern "C" fn spcp8x5_dtr_rts(port: *mut usb_serial_port, on: c_int) {
    static void spcp8x5_dtr_rts(struct usb_serial_port *port, int on)
    {
    struct spcp8x5_private *priv = usb_get_serial_port_data(port);
    unsigned long flags;
    u8 control;
    spin_lock_irqsave(&priv.lock, flags);
    if (on)
    priv.line_control = MCR_CONTROL_LINE_DTR
    | MCR_CONTROL_LINE_RTS;
    else
    priv.line_control &= ~ (MCR_CONTROL_LINE_DTR
    | MCR_CONTROL_LINE_RTS);
    control = priv.line_control;
    spin_unlock_irqrestore(&priv.lock, flags);
    spcp8x5_set_ctrl_line(port, control);
    }
#[no_mangle]
unsafe extern "C" fn spcp8x5_init_termios(tty: *mut tty_struct) {
    static void spcp8x5_init_termios(struct tty_struct *tty)
    {
    tty_encode_baud_rate(tty, 115200, 115200);
    }
    static void spcp8x5_set_termios(struct tty_struct *tty,
    struct usb_serial_port *port,
    const struct ktermios *old_termios)
    {
    struct usb_serial *serial = port.serial;
    struct spcp8x5_private *priv = usb_get_serial_port_data(port);
    unsigned long flags;
    let mut cflag: c_uint = tty.termios.c_cflag;
    unsigned short uartdata;
    unsigned char buf[2] = {0, 0};
    int baud;
    int i;
    u8 control;
// check that they really want us to change something
    if (old_termios && !tty_termios_hw_change(&tty.termios, old_termios))
    return;
// set DTR/RTS active
    spin_lock_irqsave(&priv.lock, flags);
    control = priv.line_control;
    if (old_termios && (old_termios.c_cflag & CBAUD) == B0) {
    priv.line_control |= MCR_DTR;
    if (!(old_termios.c_cflag & CRTSCTS))
    priv.line_control |= MCR_RTS;
    }
    if (control != priv.line_control) {
    control = priv.line_control;
    spin_unlock_irqrestore(&priv.lock, flags);
    spcp8x5_set_ctrl_line(port, control);
    } else {
    spin_unlock_irqrestore(&priv.lock, flags);
    }
// Set Baud Rate
    baud = tty_get_baud_rate(tty);
    switch (baud) {
    case 300:	buf[0] = 0x00;	break;
    case 600:	buf[0] = 0x01;	break;
    case 1200:	buf[0] = 0x02;	break;
    case 2400:	buf[0] = 0x03;	break;
    case 4800:	buf[0] = 0x04;	break;
    case 9600:	buf[0] = 0x05;	break;
    case 19200:	buf[0] = 0x07;	break;
    case 38400:	buf[0] = 0x09;	break;
    case 57600:	buf[0] = 0x0a;	break;
    case 115200:	buf[0] = 0x0b;	break;
    case 230400:	buf[0] = 0x0c;	break;
    case 460800:	buf[0] = 0x0d;	break;
    case 921600:	buf[0] = 0x0e;	break;
// case 1200000:	buf[0] = 0x0f;	break;
// case 2400000:	buf[0] = 0x10;	break;
    case 3000000:	buf[0] = 0x11;	break;
// case 6000000:	buf[0] = 0x12;	break;
    case 0:
    case 1000000:
    buf[0] = 0x0b;	break;
    default:
    dev_err(&port.dev, "unsupported baudrate, using 9600\n");
    }
// Set Data Length : 00:5bit, 01:6bit, 10:7bit, 11:8bit
    switch (cflag & CSIZE) {
    case CS5:
    buf[1] |= SET_UART_FORMAT_SIZE_5;
    break;
    case CS6:
    buf[1] |= SET_UART_FORMAT_SIZE_6;
    break;
    case CS7:
    buf[1] |= SET_UART_FORMAT_SIZE_7;
    break;
    default:
    case CS8:
    buf[1] |= SET_UART_FORMAT_SIZE_8;
    break;
    }
// Set Stop bit2 : 0:1bit 1:2bit
    buf[1] |= (cflag & CSTOPB) ? SET_UART_FORMAT_STOP_2 :
    SET_UART_FORMAT_STOP_1;
// Set Parity bit3-4 01:Odd 11:Even
    if (cflag & PARENB) {
    buf[1] |= (cflag & PARODD) ?
    SET_UART_FORMAT_PAR_ODD : SET_UART_FORMAT_PAR_EVEN ;
    } else {
    buf[1] |= SET_UART_FORMAT_PAR_NONE;
    }
    uartdata = buf[0] | buf[1]<<8;
    i = usb_control_msg(serial.dev, usb_sndctrlpipe(serial.dev, 0),
    SET_UART_FORMAT_TYPE, SET_UART_FORMAT,
    uartdata, 0, core::ptr::null_mut(), 0, 100);
    if (i < 0)
    dev_err(&port.dev, "Set UART format %#x failed (error = %d)\n",
    uartdata, i);
    dev_dbg(&port.dev, "0x21:0x40:0:0  %d\n", i);
    if (cflag & CRTSCTS) {
// enable hardware flow control
    spcp8x5_set_work_mode(port, 0x000a, SET_WORKING_MODE_U2C);
    }
    }
#[no_mangle]
unsafe extern "C" fn spcp8x5_open(tty: *mut tty_struct, port: *mut usb_serial_port) -> c_int {
    static int spcp8x5_open(struct tty_struct *tty, struct usb_serial_port *port)
    {
    struct usb_serial *serial = port.serial;
    struct spcp8x5_private *priv = usb_get_serial_port_data(port);
    int ret;
    usb_clear_halt(serial.dev, port.write_urb.pipe);
    usb_clear_halt(serial.dev, port.read_urb.pipe);
    ret = usb_control_msg(serial.dev, usb_sndctrlpipe(serial.dev, 0),
    0x09, 0x00,
    0x01, 0x00, core::ptr::null_mut(), 0x00, 100);
    if (ret)
    return ret;
    spcp8x5_set_ctrl_line(port, priv.line_control);
    if (tty)
    spcp8x5_set_termios(tty, port, core::ptr::null_mut());
    return usb_serial_generic_open(tty, port);
    }
    static int spcp8x5_tiocmset(struct tty_struct *tty,
    unsigned int set, unsigned int clear)
    {
    struct usb_serial_port *port = tty.driver_data;
    struct spcp8x5_private *priv = usb_get_serial_port_data(port);
    unsigned long flags;
    u8 control;
    spin_lock_irqsave(&priv.lock, flags);
    if (set & TIOCM_RTS)
    priv.line_control |= MCR_RTS;
    if (set & TIOCM_DTR)
    priv.line_control |= MCR_DTR;
    if (clear & TIOCM_RTS)
    priv.line_control &= ~MCR_RTS;
    if (clear & TIOCM_DTR)
    priv.line_control &= ~MCR_DTR;
    control = priv.line_control;
    spin_unlock_irqrestore(&priv.lock, flags);
    return spcp8x5_set_ctrl_line(port, control);
    }
#[no_mangle]
unsafe extern "C" fn spcp8x5_tiocmget(tty: *mut tty_struct) -> c_int {
    static int spcp8x5_tiocmget(struct tty_struct *tty)
    {
    struct usb_serial_port *port = tty.driver_data;
    struct spcp8x5_private *priv = usb_get_serial_port_data(port);
    unsigned long flags;
    unsigned int mcr;
    u8 status;
    unsigned int result;
    result = spcp8x5_get_msr(port, &status);
    if (result)
    return result;
    spin_lock_irqsave(&priv.lock, flags);
    mcr = priv.line_control;
    spin_unlock_irqrestore(&priv.lock, flags);
    result = ((mcr & MCR_DTR)			? TIOCM_DTR : 0)
    | ((mcr & MCR_RTS)			? TIOCM_RTS : 0)
    | ((status & MSR_STATUS_LINE_CTS)	? TIOCM_CTS : 0)
    | ((status & MSR_STATUS_LINE_DSR)	? TIOCM_DSR : 0)
    | ((status & MSR_STATUS_LINE_RI)	? TIOCM_RI  : 0)
    | ((status & MSR_STATUS_LINE_DCD)	? TIOCM_CD  : 0);
    return result;
    }
    static struct usb_serial_driver spcp8x5_device = {
    .driver = {
    .name =		"SPCP8x5",
    },
    .id_table		= id_table,
    .num_ports		= 1,
    .num_bulk_in		= 1,
    .num_bulk_out		= 1,
    .open			= spcp8x5_open,
    .dtr_rts		= spcp8x5_dtr_rts,
    .set_termios		= spcp8x5_set_termios,
    .init_termios		= spcp8x5_init_termios,
    .tiocmget		= spcp8x5_tiocmget,
    .tiocmset		= spcp8x5_tiocmset,
    .probe			= spcp8x5_probe,
    .port_probe		= spcp8x5_port_probe,
    .port_remove		= spcp8x5_port_remove,
    };
    static struct usb_serial_driver * const serial_drivers[] = {
    &spcp8x5_device, core::ptr::null_mut()
    };
    module_usb_serial_driver(serial_drivers, id_table);
    MODULE_DESCRIPTION(DRIVER_DESC);
    MODULE_LICENSE("GPL");
