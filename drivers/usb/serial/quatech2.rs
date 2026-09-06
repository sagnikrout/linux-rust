//! Automatically rewritten from C to Rust
//! Source: drivers/usb/serial/quatech2.c
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
// usb-serial driver for Quatech USB 2 devices
//
// Copyright (C) 2012 Bill Pemberton (wfp5p@virginia.edu)
//
// These devices all have only 1 bulk in and 1 bulk out that is shared
// for all serial ports.
//

// default urb timeout for usb operations

pub const QT_OPEN_CLOSE_CHANNEL: c_uint = 0xca;
pub const QT_SET_GET_DEVICE: c_uint = 0xc2;
pub const QT_SET_GET_REGISTER: c_uint = 0xc0;
pub const QT_GET_SET_PREBUF_TRIG_LVL: c_uint = 0xcc;
pub const QT_SET_ATF: c_uint = 0xcd;
pub const QT_TRANSFER_IN: c_uint = 0xc0;
pub const QT_HW_FLOW_CONTROL_MASK: c_uint = 0xc5;
pub const QT_SW_FLOW_CONTROL_MASK: c_uint = 0xc6;
pub const QT2_BREAK_CONTROL: c_uint = 0xc8;
pub const QT2_GET_SET_UART: c_uint = 0xc1;
pub const QT2_FLUSH_DEVICE: c_uint = 0xc4;
pub const QT2_GET_SET_QMCR: c_uint = 0xe1;
pub const QT2_QMCR_RS232: c_uint = 0x40;
pub const QT2_QMCR_RS422: c_uint = 0x10;

// status bytes for the device
pub const QT2_CONTROL_BYTE: c_uint = 0x1b;
pub const QT2_LINE_STATUS: c_uint = 0x00  /* following 1 byte is line status */;
pub const QT2_MODEM_STATUS: c_uint = 0x01  /* following 1 byte is modem status */;
pub const QT2_XMIT_HOLD: c_uint = 0x02  /* following 2 bytes are ?? */;
pub const QT2_CHANGE_PORT: c_uint = 0x03  /* following 1 byte is port to change to */;
pub const QT2_REC_FLUSH: c_uint = 0x04  /* no following info */;
pub const QT2_XMIT_FLUSH: c_uint = 0x05  /* no following info */;
pub const QT2_CONTROL_ESCAPE: c_uint = 0xff  /* pass through previous 2 control bytes */;
pub const MAX_BAUD_RATE: c_int = 921600;
pub const DEFAULT_BAUD_RATE: c_int = 9600;

pub const USB_VENDOR_ID_QUATECH: c_uint = 0x061d;
pub const QUATECH_SSU2_100: c_uint = 0xC120	/* RS232 single port */;
pub const QUATECH_DSU2_100: c_uint = 0xC140	/* RS232 dual port */;
pub const QUATECH_DSU2_400: c_uint = 0xC150	/* RS232/422/485 dual port */;
pub const QUATECH_QSU2_100: c_uint = 0xC160	/* RS232 four port */;
pub const QUATECH_QSU2_400: c_uint = 0xC170	/* RS232/422/485 four port */;
pub const QUATECH_ESU2_100: c_uint = 0xC1A0	/* RS232 eight port */;
pub const QUATECH_ESU2_400: c_uint = 0xC180	/* RS232/422/485 eight port */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qt2_device_detail {
    pub product_id: c_int,
    pub num_ports: c_int,
}

    .product_id = (prod),   \
    .num_ports = (ports)
    static const struct qt2_device_detail qt2_device_details[] = {
    {QT_DETAILS(QUATECH_SSU2_100, 1)},
    {QT_DETAILS(QUATECH_DSU2_400, 2)},
    {QT_DETAILS(QUATECH_DSU2_100, 2)},
    {QT_DETAILS(QUATECH_QSU2_400, 4)},
    {QT_DETAILS(QUATECH_QSU2_100, 4)},
    {QT_DETAILS(QUATECH_ESU2_400, 8)},
    {QT_DETAILS(QUATECH_ESU2_100, 8)},
    {QT_DETAILS(0, 0)}	/* Terminating entry */
    };
    static const struct usb_device_id id_table[] = {
    {USB_DEVICE(USB_VENDOR_ID_QUATECH, QUATECH_SSU2_100)},
    {USB_DEVICE(USB_VENDOR_ID_QUATECH, QUATECH_DSU2_100)},
    {USB_DEVICE(USB_VENDOR_ID_QUATECH, QUATECH_DSU2_400)},
    {USB_DEVICE(USB_VENDOR_ID_QUATECH, QUATECH_QSU2_100)},
    {USB_DEVICE(USB_VENDOR_ID_QUATECH, QUATECH_QSU2_400)},
    {USB_DEVICE(USB_VENDOR_ID_QUATECH, QUATECH_ESU2_100)},
    {USB_DEVICE(USB_VENDOR_ID_QUATECH, QUATECH_ESU2_400)},
    {}			/* Terminating entry */
    };
    MODULE_DEVICE_TABLE(usb, id_table);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qt2_serial_private {
    pub /: *mut *mut unsigned char current_port; / current port for incoming data,
    pub /: *mut *mut *mut urb read_urb; / shared among all ports,
    pub read_buffer: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qt2_port_private {
    pub device_port: u8,
    pub urb_lock: spinlock_t,
    pub urb_in_use: bool,
    pub write_urb: *mut urb,
    pub write_buffer: *mut c_char,
    pub lock: spinlock_t,
    pub shadowLSR: u8,
    pub shadowMSR: u8,
    pub port: *mut usb_serial_port,
}

    static void qt2_update_lsr(struct usb_serial_port *port, unsigned char *ch);
    static void qt2_update_msr(struct usb_serial_port *port, unsigned char *ch);
    static void qt2_write_bulk_callback(struct urb *urb);
    static void qt2_read_bulk_callback(struct urb *urb);
#[no_mangle]
unsafe extern "C" fn qt2_release(serial: *mut usb_serial) {
    static void qt2_release(struct usb_serial *serial)
    {
    struct qt2_serial_private *serial_priv;
    serial_priv = usb_get_serial_data(serial);
    usb_kill_urb(serial_priv.read_urb);
    usb_free_urb(serial_priv.read_urb);
    kfree(serial_priv.read_buffer);
    kfree(serial_priv);
    }
#[no_mangle]
pub unsafe extern "C" fn calc_baud_divisor(baudrate: c_int) -> c_int {
    static inline int calc_baud_divisor(int baudrate)
    {
    int divisor, rem;
    divisor = MAX_BAUD_RATE / baudrate;
    rem = MAX_BAUD_RATE % baudrate;
// Round to nearest divisor
    if (((rem * 2) >= baudrate) && (baudrate != 110))
    divisor++;
    return divisor;
    }
    static inline int qt2_set_port_config(struct usb_device *dev,
    unsigned char port_number,
    u16 baudrate, u16 lcr)
    {
    let mut divisor: c_int = calc_baud_divisor(baudrate);
    let mut index: u16 = ((u16) (lcr << 8) | (u16) (port_number));
    return usb_control_msg(dev, usb_sndctrlpipe(dev, 0),
    QT2_GET_SET_UART, 0x40,
    divisor, index, core::ptr::null_mut(), 0, QT2_USB_TIMEOUT);
    }
    static inline int qt2_control_msg(struct usb_device *dev,
    u8 request, u16 data, u16 index)
    {
    return usb_control_msg(dev, usb_sndctrlpipe(dev, 0),
    request, 0x40, data, index,
    core::ptr::null_mut(), 0, QT2_USB_TIMEOUT);
    }
    static inline int qt2_getregister(struct usb_device *dev,
    u8 uart,
    u8 reg,
    u8 *data)
    {
    int ret;
    ret = usb_control_msg(dev, usb_rcvctrlpipe(dev, 0),
    QT_SET_GET_REGISTER, 0xc0, reg,
    uart, data, sizeof(*data), QT2_USB_TIMEOUT);
    if (ret < (int)sizeof(*data)) {
    if (ret >= 0)
    ret = -EIO;
    }
    return ret;
    }
    static inline int qt2_setregister(struct usb_device *dev,
    u8 uart, u8 reg, u16 data)
    {
    let mut value: u16 = (data << 8) | reg;
    return usb_control_msg(dev, usb_sndctrlpipe(dev, 0),
    QT_SET_GET_REGISTER, 0x40, value, uart,
    core::ptr::null_mut(), 0, QT2_USB_TIMEOUT);
    }
    static inline int update_mctrl(struct qt2_port_private *port_priv,
    unsigned int set, unsigned int clear)
    {
    struct usb_serial_port *port = port_priv.port;
    struct usb_device *dev = port.serial.dev;
    unsigned urb_value;
    int status;
    if (((set | clear) & (TIOCM_DTR | TIOCM_RTS)) == 0) {
    dev_dbg(&port.dev,
    "update_mctrl - DTR|RTS not being set|cleared\n");
    return 0;	/* no change */
    }
    clear &= ~set;	/* 'set' takes precedence over 'clear' */
    urb_value = 0;
    if (set & TIOCM_DTR)
    urb_value |= UART_MCR_DTR;
    if (set & TIOCM_RTS)
    urb_value |= UART_MCR_RTS;
    status = qt2_setregister(dev, port_priv.device_port, UART_MCR,
    urb_value);
    if (status < 0)
    dev_err(&port.dev,
    "update_mctrl - Error from MODEM_CTRL urb: %i\n",
    status);
    return status;
    }
    static int qt2_calc_num_ports(struct usb_serial *serial,
    struct usb_serial_endpoints *epds)
    {
    struct qt2_device_detail d;
    int i;
    for (i = 0; d = qt2_device_details[i], d.product_id != 0; i++) {
    if (d.product_id == le16_to_cpu(serial.dev.descriptor.idProduct))
    return d.num_ports;
    }
// we didn't recognize the device
    dev_err(&serial.dev.dev,
    "don't know the number of ports, assuming 1\n");
    return 1;
    }
    static void qt2_set_termios(struct tty_struct *tty,
    struct usb_serial_port *port,
    const struct ktermios *old_termios)
    {
    struct usb_device *dev = port.serial.dev;
    struct qt2_port_private *port_priv;
    struct ktermios *termios = &tty.termios;
    u16 baud;
    let mut cflag: c_uint = termios.c_cflag;
    let mut new_lcr: u16 = 0;
    int status;
    port_priv = usb_get_serial_port_data(port);
    if (cflag & PARENB) {
    if (cflag & PARODD)
    new_lcr |= UART_LCR_PARITY;
    else
    new_lcr |= SERIAL_EVEN_PARITY;
    }
    new_lcr |= UART_LCR_WLEN(tty_get_char_size(cflag));
    baud = tty_get_baud_rate(tty);
    if (!baud)
    baud = 9600;
    status = qt2_set_port_config(dev, port_priv.device_port, baud,
    new_lcr);
    if (status < 0)
    dev_err(&port.dev, "%s - qt2_set_port_config failed: %i\n",
    __func__, status);
    if (cflag & CRTSCTS)
    status = qt2_control_msg(dev, QT_HW_FLOW_CONTROL_MASK,
    SERIAL_CRTSCTS,
    port_priv.device_port);
    else
    status = qt2_control_msg(dev, QT_HW_FLOW_CONTROL_MASK,
    0, port_priv.device_port);
    if (status < 0)
    dev_err(&port.dev, "%s - set HW flow control failed: %i\n",
    __func__, status);
    if (I_IXOFF(tty) || I_IXON(tty)) {
    let mut x: u16 = ((u16) (START_CHAR(tty) << 8) | (u16) (STOP_CHAR(tty)));
    status = qt2_control_msg(dev, QT_SW_FLOW_CONTROL_MASK,
    x, port_priv.device_port);
    } else
    status = qt2_control_msg(dev, QT_SW_FLOW_CONTROL_MASK,
    0, port_priv.device_port);
    if (status < 0)
    dev_err(&port.dev, "%s - set SW flow control failed: %i\n",
    __func__, status);
    }
#[no_mangle]
unsafe extern "C" fn qt2_open(tty: *mut tty_struct, port: *mut usb_serial_port) -> c_int {
    static int qt2_open(struct tty_struct *tty, struct usb_serial_port *port)
    {
    struct usb_serial *serial;
    struct qt2_port_private *port_priv;
    u8 *data;
    u16 device_port;
    int status;
    unsigned long flags;
    device_port = port.port_number;
    serial = port.serial;
    port_priv = usb_get_serial_port_data(port);
// set the port to RS232 mode
    status = qt2_control_msg(serial.dev, QT2_GET_SET_QMCR,
    QT2_QMCR_RS232, device_port);
    if (status < 0) {
    dev_err(&port.dev,
    "%s failed to set RS232 mode for port %i error %i\n",
    __func__, device_port, status);
    return status;
    }
    data = kzalloc(2, GFP_KERNEL);
    if (!data)
    return -ENOMEM;
// open the port
    status = usb_control_msg(serial.dev,
    usb_rcvctrlpipe(serial.dev, 0),
    QT_OPEN_CLOSE_CHANNEL,
    0xc0, 0,
    device_port, data, 2, QT2_USB_TIMEOUT);
    if (status < 2) {
    dev_err(&port.dev, "%s - open port failed %i\n", __func__,
    status);
    if (status >= 0)
    status = -EIO;
    kfree(data);
    return status;
    }
    spin_lock_irqsave(&port_priv.lock, flags);
    port_priv.shadowLSR = data[0];
    port_priv.shadowMSR = data[1];
    spin_unlock_irqrestore(&port_priv.lock, flags);
    kfree(data);
// set to default speed and 8bit word size
    status = qt2_set_port_config(serial.dev, device_port,
    DEFAULT_BAUD_RATE, UART_LCR_WLEN8);
    if (status < 0) {
    dev_err(&port.dev, "%s - initial setup failed (%i)\n",
    __func__, device_port);
    return status;
    }
    port_priv.device_port = (u8) device_port;
    if (tty)
    qt2_set_termios(tty, port, &tty.termios);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qt2_close(port: *mut usb_serial_port) {
    static void qt2_close(struct usb_serial_port *port)
    {
    struct usb_serial *serial;
    struct qt2_port_private *port_priv;
    int i;
    serial = port.serial;
    port_priv = usb_get_serial_port_data(port);
    usb_kill_urb(port_priv.write_urb);
// flush the port transmit buffer
    i = usb_control_msg(serial.dev,
    usb_sndctrlpipe(serial.dev, 0),
    QT2_FLUSH_DEVICE, 0x40, 1,
    port_priv.device_port, core::ptr::null_mut(), 0, QT2_USB_TIMEOUT);
    if (i < 0)
    dev_err(&port.dev, "%s - transmit buffer flush failed: %i\n",
    __func__, i);
// flush the port receive buffer
    i = usb_control_msg(serial.dev,
    usb_sndctrlpipe(serial.dev, 0),
    QT2_FLUSH_DEVICE, 0x40, 0,
    port_priv.device_port, core::ptr::null_mut(), 0, QT2_USB_TIMEOUT);
    if (i < 0)
    dev_err(&port.dev, "%s - receive buffer flush failed: %i\n",
    __func__, i);
// close the port
    i = usb_control_msg(serial.dev,
    usb_sndctrlpipe(serial.dev, 0),
    QT_OPEN_CLOSE_CHANNEL,
    0x40, 0,
    port_priv.device_port, core::ptr::null_mut(), 0, QT2_USB_TIMEOUT);
    if (i < 0)
    dev_err(&port.dev, "%s - close port failed %i\n",
    __func__, i);
    }
#[no_mangle]
unsafe extern "C" fn qt2_disconnect(serial: *mut usb_serial) {
    static void qt2_disconnect(struct usb_serial *serial)
    {
    struct qt2_serial_private *serial_priv = usb_get_serial_data(serial);
    usb_kill_urb(serial_priv.read_urb);
    }
#[no_mangle]
unsafe extern "C" fn qt2_process_status(port: *mut usb_serial_port, ch: *mut c_uchar) {
    static void qt2_process_status(struct usb_serial_port *port, unsigned char *ch)
    {
    switch (*ch) {
    case QT2_LINE_STATUS:
    qt2_update_lsr(port, ch + 1);
    break;
    case QT2_MODEM_STATUS:
    qt2_update_msr(port, ch + 1);
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn qt2_process_read_urb(urb: *mut urb) {
    static void qt2_process_read_urb(struct urb *urb)
    {
    struct usb_serial *serial;
    struct qt2_serial_private *serial_priv;
    struct usb_serial_port *port;
    bool escapeflag;
    unsigned char *ch;
    int i;
    unsigned char newport;
    let mut len: c_int = urb.actual_length;
    if (!len)
    return;
    ch = urb.transfer_buffer;
    serial = urb.context;
    serial_priv = usb_get_serial_data(serial);
    port = serial.port[serial_priv.current_port];
    for (i = 0; i < urb.actual_length; i++) {
    ch = (unsigned char *)urb.transfer_buffer + i;
    if ((i <= (len - 3)) &&
    (*ch == QT2_CONTROL_BYTE) &&
    (*(ch + 1) == QT2_CONTROL_BYTE)) {
    escapeflag = false;
    switch (*(ch + 2)) {
    case QT2_LINE_STATUS:
    case QT2_MODEM_STATUS:
    if (i > (len - 4)) {
    dev_warn(&port.dev,
    "%s - status message too short\n",
    __func__);
    break;
    }
    qt2_process_status(port, ch + 2);
    i += 3;
    escapeflag = true;
    break;
    case QT2_XMIT_HOLD:
    if (i > (len - 5)) {
    dev_warn(&port.dev,
    "%s - xmit_empty message too short\n",
    __func__);
    break;
    }
// bytes_written = (ch[1] << 4) + ch[0];
    i += 4;
    escapeflag = true;
    break;
    case QT2_CHANGE_PORT:
    if (i > (len - 4)) {
    dev_warn(&port.dev,
    "%s - change_port message too short\n",
    __func__);
    break;
    }
    tty_flip_buffer_push(&port.port);
    newport = *(ch + 3);
    if (newport >= serial.num_ports) {
    dev_err(&port.dev,
    "%s - port change to invalid port: %i\n",
    __func__, newport);
    break;
    }
    serial_priv.current_port = newport;
    port = serial.port[serial_priv.current_port];
    i += 3;
    escapeflag = true;
    break;
    case QT2_REC_FLUSH:
    case QT2_XMIT_FLUSH:
    i += 2;
    escapeflag = true;
    break;
    case QT2_CONTROL_ESCAPE:
    tty_insert_flip_string(&port.port, ch, 2);
    i += 2;
    escapeflag = true;
    break;
    default:
    dev_warn(&port.dev,
    "%s - unsupported command %i\n",
    __func__, *(ch + 2));
    break;
    }
    if (escapeflag)
    continue;
    }
    tty_insert_flip_char(&port.port, *ch, TTY_NORMAL);
    }
    tty_flip_buffer_push(&port.port);
    }
#[no_mangle]
unsafe extern "C" fn qt2_write_bulk_callback(urb: *mut urb) {
    static void qt2_write_bulk_callback(struct urb *urb)
    {
    struct usb_serial_port *port;
    struct qt2_port_private *port_priv;
    unsigned long flags;
    port = urb.context;
    port_priv = usb_get_serial_port_data(port);
    spin_lock_irqsave(&port_priv.urb_lock, flags);
    port_priv.urb_in_use = false;
    usb_serial_port_softint(port);
    spin_unlock_irqrestore(&port_priv.urb_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn qt2_read_bulk_callback(urb: *mut urb) {
    static void qt2_read_bulk_callback(struct urb *urb)
    {
    struct usb_serial *serial = urb.context;
    int status;
    if (urb.status) {
    dev_warn(&serial.dev.dev,
    "%s - non-zero urb status: %i\n", __func__,
    urb.status);
    return;
    }
    qt2_process_read_urb(urb);
    status = usb_submit_urb(urb, GFP_ATOMIC);
    if (status != 0)
    dev_err(&serial.dev.dev,
    "%s - resubmit read urb failed: %i\n",
    __func__, status);
    }
#[no_mangle]
unsafe extern "C" fn qt2_setup_urbs(serial: *mut usb_serial) -> c_int {
    static int qt2_setup_urbs(struct usb_serial *serial)
    {
    struct usb_serial_port *port0;
    struct qt2_serial_private *serial_priv;
    int status;
    port0 = serial.port[0];
    serial_priv = usb_get_serial_data(serial);
    serial_priv.read_urb = usb_alloc_urb(0, GFP_KERNEL);
    if (!serial_priv.read_urb)
    return -ENOMEM;
    usb_fill_bulk_urb(serial_priv.read_urb, serial.dev,
    usb_rcvbulkpipe(serial.dev,
    port0.bulk_in_endpointAddress),
    serial_priv.read_buffer,
    QT2_READ_BUFFER_SIZE,
    qt2_read_bulk_callback, serial);
    status = usb_submit_urb(serial_priv.read_urb, GFP_KERNEL);
    if (status != 0) {
    dev_err(&serial.dev.dev,
    "%s - submit read urb failed %i\n", __func__, status);
    usb_free_urb(serial_priv.read_urb);
    return status;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qt2_attach(serial: *mut usb_serial) -> c_int {
    static int qt2_attach(struct usb_serial *serial)
    {
    struct qt2_serial_private *serial_priv;
    int status;
// power on unit
    status = usb_control_msg(serial.dev, usb_sndctrlpipe(serial.dev, 0),
    0xc2, 0x40, 0x8000, 0, core::ptr::null_mut(), 0,
    QT2_USB_TIMEOUT);
    if (status < 0) {
    dev_err(&serial.dev.dev,
    "%s - failed to power on unit: %i\n", __func__, status);
    return status;
    }
    serial_priv = kzalloc_obj(*serial_priv);
    if (!serial_priv)
    return -ENOMEM;
    serial_priv.read_buffer = kmalloc(QT2_READ_BUFFER_SIZE, GFP_KERNEL);
    if (!serial_priv.read_buffer) {
    status = -ENOMEM;
    goto err_buf;
    }
    usb_set_serial_data(serial, serial_priv);
    status = qt2_setup_urbs(serial);
    if (status != 0)
    goto attach_failed;
    return 0;
    attach_failed:
    kfree(serial_priv.read_buffer);
    err_buf:
    kfree(serial_priv);
    return status;
    }
#[no_mangle]
unsafe extern "C" fn qt2_port_probe(port: *mut usb_serial_port) -> c_int {
    static int qt2_port_probe(struct usb_serial_port *port)
    {
    struct usb_serial *serial = port.serial;
    struct qt2_port_private *port_priv;
    u8 bEndpointAddress;
    port_priv = kzalloc_obj(*port_priv);
    if (!port_priv)
    return -ENOMEM;
    spin_lock_init(&port_priv.lock);
    spin_lock_init(&port_priv.urb_lock);
    port_priv.port = port;
    port_priv.write_buffer = kmalloc(QT2_WRITE_BUFFER_SIZE, GFP_KERNEL);
    if (!port_priv.write_buffer)
    goto err_buf;
    port_priv.write_urb = usb_alloc_urb(0, GFP_KERNEL);
    if (!port_priv.write_urb)
    goto err_urb;
    bEndpointAddress = serial.port[0].bulk_out_endpointAddress;
    usb_fill_bulk_urb(port_priv.write_urb, serial.dev,
    usb_sndbulkpipe(serial.dev, bEndpointAddress),
    port_priv.write_buffer,
    QT2_WRITE_BUFFER_SIZE,
    qt2_write_bulk_callback, port);
    usb_set_serial_port_data(port, port_priv);
    return 0;
    err_urb:
    kfree(port_priv.write_buffer);
    err_buf:
    kfree(port_priv);
    return -ENOMEM;
    }
#[no_mangle]
unsafe extern "C" fn qt2_port_remove(port: *mut usb_serial_port) {
    static void qt2_port_remove(struct usb_serial_port *port)
    {
    struct qt2_port_private *port_priv;
    port_priv = usb_get_serial_port_data(port);
    usb_free_urb(port_priv.write_urb);
    kfree(port_priv.write_buffer);
    kfree(port_priv);
    }
#[no_mangle]
unsafe extern "C" fn qt2_tiocmget(tty: *mut tty_struct) -> c_int {
    static int qt2_tiocmget(struct tty_struct *tty)
    {
    struct usb_serial_port *port = tty.driver_data;
    struct usb_device *dev = port.serial.dev;
    struct qt2_port_private *port_priv = usb_get_serial_port_data(port);
    u8 *d;
    int r;
    d = kzalloc(2, GFP_KERNEL);
    if (!d)
    return -ENOMEM;
    r = qt2_getregister(dev, port_priv.device_port, UART_MCR, d);
    if (r < 0)
    goto mget_out;
    r = qt2_getregister(dev, port_priv.device_port, UART_MSR, d + 1);
    if (r < 0)
    goto mget_out;
    r = (d[0] & UART_MCR_DTR ? TIOCM_DTR : 0) |
    (d[0] & UART_MCR_RTS ? TIOCM_RTS : 0) |
    (d[1] & UART_MSR_CTS ? TIOCM_CTS : 0) |
    (d[1] & UART_MSR_DCD ? TIOCM_CAR : 0) |
    (d[1] & UART_MSR_RI ? TIOCM_RI : 0) |
    (d[1] & UART_MSR_DSR ? TIOCM_DSR : 0);
    mget_out:
    kfree(d);
    return r;
    }
    static int qt2_tiocmset(struct tty_struct *tty,
    unsigned int set, unsigned int clear)
    {
    struct qt2_port_private *port_priv;
    port_priv = usb_get_serial_port_data(tty.driver_data);
    return update_mctrl(port_priv, set, clear);
    }
#[no_mangle]
unsafe extern "C" fn qt2_break_ctl(tty: *mut tty_struct, break_state: c_int) -> c_int {
    static int qt2_break_ctl(struct tty_struct *tty, int break_state)
    {
    struct usb_serial_port *port = tty.driver_data;
    struct qt2_port_private *port_priv;
    int status;
    u16 val;
    port_priv = usb_get_serial_port_data(port);
    val = (break_state == -1) ? 1 : 0;
    status = qt2_control_msg(port.serial.dev, QT2_BREAK_CONTROL,
    val, port_priv.device_port);
    if (status < 0) {
    dev_warn(&port.dev,
    "%s - failed to send control message: %i\n", __func__,
    status);
    return status;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qt2_dtr_rts(port: *mut usb_serial_port, on: c_int) {
    static void qt2_dtr_rts(struct usb_serial_port *port, int on)
    {
    struct usb_device *dev = port.serial.dev;
    struct qt2_port_private *port_priv = usb_get_serial_port_data(port);
// Disable flow control
    if (!on) {
    if (qt2_setregister(dev, port_priv.device_port,
    UART_MCR, 0) < 0)
    dev_warn(&port.dev, "error from flowcontrol urb\n");
    }
// drop RTS and DTR
    if (on)
    update_mctrl(port_priv, TIOCM_DTR | TIOCM_RTS, 0);
    else
    update_mctrl(port_priv, 0, TIOCM_DTR | TIOCM_RTS);
    }
#[no_mangle]
unsafe extern "C" fn qt2_update_msr(port: *mut usb_serial_port, ch: *mut c_uchar) {
    static void qt2_update_msr(struct usb_serial_port *port, unsigned char *ch)
    {
    struct qt2_port_private *port_priv;
    let mut newMSR: u8 = (u8) *ch;
    unsigned long flags;
// May be called from qt2_process_read_urb() for an unbound port.
    port_priv = usb_get_serial_port_data(port);
    if (!port_priv)
    return;
    spin_lock_irqsave(&port_priv.lock, flags);
    port_priv.shadowMSR = newMSR;
    spin_unlock_irqrestore(&port_priv.lock, flags);
    if (newMSR & UART_MSR_ANY_DELTA) {
// update input line counters
    if (newMSR & UART_MSR_DCTS)
    port.icount.cts++;
    if (newMSR & UART_MSR_DDSR)
    port.icount.dsr++;
    if (newMSR & UART_MSR_DDCD)
    port.icount.dcd++;
    if (newMSR & UART_MSR_TERI)
    port.icount.rng++;
    wake_up_interruptible(&port.port.delta_msr_wait);
    }
    }
#[no_mangle]
unsafe extern "C" fn qt2_update_lsr(port: *mut usb_serial_port, ch: *mut c_uchar) {
    static void qt2_update_lsr(struct usb_serial_port *port, unsigned char *ch)
    {
    struct qt2_port_private *port_priv;
    struct async_icount *icount;
    unsigned long flags;
    let mut newLSR: u8 = (u8) *ch;
// May be called from qt2_process_read_urb() for an unbound port.
    port_priv = usb_get_serial_port_data(port);
    if (!port_priv)
    return;
    if (newLSR & UART_LSR_BI)
    newLSR &= (u8) (UART_LSR_OE | UART_LSR_BI);
    spin_lock_irqsave(&port_priv.lock, flags);
    port_priv.shadowLSR = newLSR;
    spin_unlock_irqrestore(&port_priv.lock, flags);
    icount = &port.icount;
    if (newLSR & UART_LSR_BRK_ERROR_BITS) {
    if (newLSR & UART_LSR_BI)
    icount.brk++;
    if (newLSR & UART_LSR_OE)
    icount.overrun++;
    if (newLSR & UART_LSR_PE)
    icount.parity++;
    if (newLSR & UART_LSR_FE)
    icount.frame++;
    }
    }
#[no_mangle]
unsafe extern "C" fn qt2_write_room(tty: *mut tty_struct) -> c_uint {
    static unsigned int qt2_write_room(struct tty_struct *tty)
    {
    struct usb_serial_port *port = tty.driver_data;
    struct qt2_port_private *port_priv;
    unsigned long flags;
    unsigned int r;
    port_priv = usb_get_serial_port_data(port);
    spin_lock_irqsave(&port_priv.urb_lock, flags);
    if (port_priv.urb_in_use)
    r = 0;
    else
    r = QT2_WRITE_BUFFER_SIZE - QT2_WRITE_CONTROL_SIZE;
    spin_unlock_irqrestore(&port_priv.urb_lock, flags);
    return r;
    }
    static int qt2_write(struct tty_struct *tty,
    struct usb_serial_port *port,
    const unsigned char *buf, int count)
    {
    struct qt2_port_private *port_priv;
    struct urb *write_urb;
    unsigned char *data;
    unsigned long flags;
    int status;
    let mut bytes_out: c_int = 0;
    port_priv = usb_get_serial_port_data(port);
    if (port_priv.write_urb == core::ptr::null_mut()) {
    dev_err(&port.dev, "%s - no output urb\n", __func__);
    return 0;
    }
    write_urb = port_priv.write_urb;
    count = min(count, QT2_WRITE_BUFFER_SIZE - QT2_WRITE_CONTROL_SIZE);
    data = write_urb.transfer_buffer;
    spin_lock_irqsave(&port_priv.urb_lock, flags);
    if (port_priv.urb_in_use) {
    dev_err(&port.dev, "qt2_write - urb is in use\n");
    goto write_out;
    }
// data++ = QT2_CONTROL_BYTE;
// data++ = port_priv->device_port;
    put_unaligned_le16(count, data);
    data += 2;
    memcpy(data, buf, count);
    write_urb.transfer_buffer_length = count + QT2_WRITE_CONTROL_SIZE;
    status = usb_submit_urb(write_urb, GFP_ATOMIC);
    if (status == 0) {
    port_priv.urb_in_use = true;
    bytes_out += count;
    }
    write_out:
    spin_unlock_irqrestore(&port_priv.urb_lock, flags);
    return bytes_out;
    }
    static struct usb_serial_driver qt2_device = {
    .driver = {
    .name = "quatech-serial",
    },
    .description	     = DRIVER_DESC,
    .id_table	     = id_table,
    .open		     = qt2_open,
    .close		     = qt2_close,
    .write               = qt2_write,
    .write_room          = qt2_write_room,
    .calc_num_ports      = qt2_calc_num_ports,
    .attach              = qt2_attach,
    .release             = qt2_release,
    .disconnect          = qt2_disconnect,
    .port_probe          = qt2_port_probe,
    .port_remove         = qt2_port_remove,
    .dtr_rts             = qt2_dtr_rts,
    .break_ctl           = qt2_break_ctl,
    .tiocmget            = qt2_tiocmget,
    .tiocmset            = qt2_tiocmset,
    .tiocmiwait          = usb_serial_generic_tiocmiwait,
    .get_icount	     = usb_serial_generic_get_icount,
    .set_termios         = qt2_set_termios,
    };
    static struct usb_serial_driver *const serial_drivers[] = {
    &qt2_device, core::ptr::null_mut()
    };
    module_usb_serial_driver(serial_drivers, id_table);
    MODULE_DESCRIPTION(DRIVER_DESC);
    MODULE_LICENSE("GPL v2");
