//! Automatically rewritten from C to Rust
//! Source: drivers/usb/serial/upd78f0730.c
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
// Renesas Electronics uPD78F0730 USB to serial converter driver
//
// Copyright (C) 2014,2016 Maksim Salau <maksim.salau@gmail.com>
//
// Protocol of the adaptor is described in the application note U19660EJ1V0AN00
// μPD78F0730 8-bit Single-Chip Microcontroller
// USB-to-Serial Conversion Software
// <https://www.renesas.com/en-eu/doc/DocumentServer/026/U19660EJ1V0AN00.pdf>
//
// The adaptor functionality is limited to the following:
// - data bits: 7 or 8
// - stop bits: 1 or 2
// - parity: even, odd or none
// - flow control: none
// - baud rates: 0, 2400, 4800, 9600, 19200, 38400, 57600, 115200, 153600
// - signals: DTR, RTS and BREAK
//

    static const struct usb_device_id id_table[] = {
    { USB_DEVICE(0x0409, 0x0063) }, /* V850ESJX3-STICK */
    { USB_DEVICE(0x045B, 0x0212) }, /* YRPBRL78G13, YRPBRL78G14 */
    { USB_DEVICE(0x064B, 0x7825) }, /* Analog Devices EVAL-ADXL362Z-DB */
    {}
    };
    MODULE_DEVICE_TABLE(usb, id_table);
//
// Each adaptor is associated with a private structure, that holds the current
// state of control signals (DTR, RTS and BREAK).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct upd78f0730_port_private {
    pub /: *mut *mut mutex lock; / mutex to protect line_signals,
    pub line_signals: u8,
}

// Op-codes of control commands
pub const UPD78F0730_CMD_LINE_CONTROL: c_uint = 0x00;
pub const UPD78F0730_CMD_SET_DTR_RTS: c_uint = 0x01;
pub const UPD78F0730_CMD_SET_XON_XOFF_CHR: c_uint = 0x02;
pub const UPD78F0730_CMD_OPEN_CLOSE: c_uint = 0x03;
pub const UPD78F0730_CMD_SET_ERR_CHR: c_uint = 0x04;
// Data sizes in UPD78F0730_CMD_LINE_CONTROL command
pub const UPD78F0730_DATA_SIZE_7_BITS: c_uint = 0x00;
pub const UPD78F0730_DATA_SIZE_8_BITS: c_uint = 0x01;
pub const UPD78F0730_DATA_SIZE_MASK: c_uint = 0x01;
// Stop-bit modes in UPD78F0730_CMD_LINE_CONTROL command
pub const UPD78F0730_STOP_BIT_1_BIT: c_uint = 0x00;
pub const UPD78F0730_STOP_BIT_2_BIT: c_uint = 0x02;
pub const UPD78F0730_STOP_BIT_MASK: c_uint = 0x02;
// Parity modes in UPD78F0730_CMD_LINE_CONTROL command
pub const UPD78F0730_PARITY_NONE: c_uint = 0x00;
pub const UPD78F0730_PARITY_EVEN: c_uint = 0x04;
pub const UPD78F0730_PARITY_ODD: c_uint = 0x08;
pub const UPD78F0730_PARITY_MASK: c_uint = 0x0C;
// Flow control modes in UPD78F0730_CMD_LINE_CONTROL command
pub const UPD78F0730_FLOW_CONTROL_NONE: c_uint = 0x00;
pub const UPD78F0730_FLOW_CONTROL_HW: c_uint = 0x10;
pub const UPD78F0730_FLOW_CONTROL_SW: c_uint = 0x20;
pub const UPD78F0730_FLOW_CONTROL_MASK: c_uint = 0x30;
// Control signal bits in UPD78F0730_CMD_SET_DTR_RTS command
pub const UPD78F0730_RTS: c_uint = 0x01;
pub const UPD78F0730_DTR: c_uint = 0x02;
pub const UPD78F0730_BREAK: c_uint = 0x04;
// Port modes in UPD78F0730_CMD_OPEN_CLOSE command
pub const UPD78F0730_PORT_CLOSE: c_uint = 0x00;
pub const UPD78F0730_PORT_OPEN: c_uint = 0x01;
// Error character substitution modes in UPD78F0730_CMD_SET_ERR_CHR command
pub const UPD78F0730_ERR_CHR_DISABLED: c_uint = 0x00;
pub const UPD78F0730_ERR_CHR_ENABLED: c_uint = 0x01;
//
// Declaration of command structures
//
// UPD78F0730_CMD_LINE_CONTROL command
#[repr(C)]
#[derive(Copy, Clone)]
pub struct upd78f0730_line_control {
    pub opcode: u8,
    pub baud_rate: __le32,
    pub params: u8,
    pub __packed: },
// UPD78F0730_CMD_SET_DTR_RTS command
#[repr(C)]
#[derive(Copy, Clone)]
pub struct upd78f0730_set_dtr_rts {
    pub opcode: u8,
    pub params: u8,
}

// UPD78F0730_CMD_SET_XON_OFF_CHR command
#[repr(C)]
#[derive(Copy, Clone)]
pub struct upd78f0730_set_xon_xoff_chr {
    pub opcode: u8,
    pub xon: u8,
    pub xoff: u8,
}

// UPD78F0730_CMD_OPEN_CLOSE command
#[repr(C)]
#[derive(Copy, Clone)]
pub struct upd78f0730_open_close {
    pub opcode: u8,
    pub state: u8,
}

// UPD78F0730_CMD_SET_ERR_CHR command
#[repr(C)]
#[derive(Copy, Clone)]
pub struct upd78f0730_set_err_chr {
    pub opcode: u8,
    pub state: u8,
    pub err_char: u8,
}

    static int upd78f0730_send_ctl(struct usb_serial_port *port,
    const void *data, int size)
    {
    struct usb_device *usbdev = port.serial.dev;
    void *buf;
    int res;
    if (size <= 0 || !data)
    return -EINVAL;
    buf = kmemdup(data, size, GFP_KERNEL);
    if (!buf)
    return -ENOMEM;
    res = usb_control_msg(usbdev, usb_sndctrlpipe(usbdev, 0), 0x00,
    USB_TYPE_VENDOR | USB_RECIP_DEVICE | USB_DIR_OUT,
    0x0000, 0x0000, buf, size, USB_CTRL_SET_TIMEOUT);
    kfree(buf);
    if (res < 0) {
    struct device *dev = &port.dev;
    dev_err(dev, "failed to send control request %02x: %d\n",
// (u8 *)data, res);
    return res;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn upd78f0730_port_probe(port: *mut usb_serial_port) -> c_int {
    static int upd78f0730_port_probe(struct usb_serial_port *port)
    {
    struct upd78f0730_port_private *private;
    private = kzalloc_obj(*private);
    if (!private)
    return -ENOMEM;
    mutex_init(&private.lock);
    usb_set_serial_port_data(port, private);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn upd78f0730_port_remove(port: *mut usb_serial_port) {
    static void upd78f0730_port_remove(struct usb_serial_port *port)
    {
    struct upd78f0730_port_private *private;
    private = usb_get_serial_port_data(port);
    mutex_destroy(&private.lock);
    kfree(private);
    }
#[no_mangle]
unsafe extern "C" fn upd78f0730_tiocmget(tty: *mut tty_struct) -> c_int {
    static int upd78f0730_tiocmget(struct tty_struct *tty)
    {
    struct upd78f0730_port_private *private;
    struct usb_serial_port *port = tty.driver_data;
    int signals;
    int res;
    private = usb_get_serial_port_data(port);
    mutex_lock(&private.lock);
    signals = private.line_signals;
    mutex_unlock(&private.lock);
    res = ((signals & UPD78F0730_DTR) ? TIOCM_DTR : 0) |
    ((signals & UPD78F0730_RTS) ? TIOCM_RTS : 0);
    dev_dbg(&port.dev, "%s - res = %x\n", __func__, res);
    return res;
    }
    static int upd78f0730_tiocmset(struct tty_struct *tty,
    unsigned int set, unsigned int clear)
    {
    struct usb_serial_port *port = tty.driver_data;
    struct upd78f0730_port_private *private;
    struct upd78f0730_set_dtr_rts request;
    struct device *dev = &port.dev;
    int res;
    private = usb_get_serial_port_data(port);
    mutex_lock(&private.lock);
    if (set & TIOCM_DTR) {
    private.line_signals |= UPD78F0730_DTR;
    dev_dbg(dev, "%s - set DTR\n", __func__);
    }
    if (set & TIOCM_RTS) {
    private.line_signals |= UPD78F0730_RTS;
    dev_dbg(dev, "%s - set RTS\n", __func__);
    }
    if (clear & TIOCM_DTR) {
    private.line_signals &= ~UPD78F0730_DTR;
    dev_dbg(dev, "%s - clear DTR\n", __func__);
    }
    if (clear & TIOCM_RTS) {
    private.line_signals &= ~UPD78F0730_RTS;
    dev_dbg(dev, "%s - clear RTS\n", __func__);
    }
    request.opcode = UPD78F0730_CMD_SET_DTR_RTS;
    request.params = private.line_signals;
    res = upd78f0730_send_ctl(port, &request, sizeof(request));
    mutex_unlock(&private.lock);
    return res;
    }
#[no_mangle]
unsafe extern "C" fn upd78f0730_break_ctl(tty: *mut tty_struct, break_state: c_int) -> c_int {
    static int upd78f0730_break_ctl(struct tty_struct *tty, int break_state)
    {
    struct upd78f0730_port_private *private;
    struct usb_serial_port *port = tty.driver_data;
    struct upd78f0730_set_dtr_rts request;
    struct device *dev = &port.dev;
    int res;
    private = usb_get_serial_port_data(port);
    mutex_lock(&private.lock);
    if (break_state) {
    private.line_signals |= UPD78F0730_BREAK;
    dev_dbg(dev, "%s - set BREAK\n", __func__);
    } else {
    private.line_signals &= ~UPD78F0730_BREAK;
    dev_dbg(dev, "%s - clear BREAK\n", __func__);
    }
    request.opcode = UPD78F0730_CMD_SET_DTR_RTS;
    request.params = private.line_signals;
    res = upd78f0730_send_ctl(port, &request, sizeof(request));
    mutex_unlock(&private.lock);
    return res;
    }
#[no_mangle]
unsafe extern "C" fn upd78f0730_dtr_rts(port: *mut usb_serial_port, on: c_int) {
    static void upd78f0730_dtr_rts(struct usb_serial_port *port, int on)
    {
    struct tty_struct *tty = port.port.tty;
    let mut set: c_uint = 0;
    let mut clear: c_uint = 0;
    if (on)
    set = TIOCM_DTR | TIOCM_RTS;
    else
    clear = TIOCM_DTR | TIOCM_RTS;
    upd78f0730_tiocmset(tty, set, clear);
    }
#[no_mangle]
unsafe extern "C" fn upd78f0730_get_baud_rate(tty: *mut tty_struct) -> speed_t {
    static speed_t upd78f0730_get_baud_rate(struct tty_struct *tty)
    {
    let mut baud_rate: speed_t = tty_get_baud_rate(tty);
    static const speed_t supported[] = {
    0, 2400, 4800, 9600, 19200, 38400, 57600, 115200, 153600
    };
    int i;
    for (i = ARRAY_SIZE(supported) - 1; i >= 0; i--) {
    if (baud_rate == supported[i])
    return baud_rate;
    }
// If the baud rate is not supported, switch to the default one
    tty_encode_baud_rate(tty, 9600, 9600);
    return tty_get_baud_rate(tty);
    }
    static void upd78f0730_set_termios(struct tty_struct *tty,
    struct usb_serial_port *port,
    const struct ktermios *old_termios)
    {
    struct device *dev = &port.dev;
    struct upd78f0730_line_control request;
    speed_t baud_rate;
    if (old_termios && !tty_termios_hw_change(&tty.termios, old_termios))
    return;
    if (C_BAUD(tty) == B0)
    upd78f0730_dtr_rts(port, 0);
#[no_mangle]
pub unsafe extern "C" fn if(B0: old_termios && (old_termios->c_cflag & CBAUD) ==) -> else {
    else if (old_termios && (old_termios.c_cflag & CBAUD) == B0)
    upd78f0730_dtr_rts(port, 1);
    baud_rate = upd78f0730_get_baud_rate(tty);
    request.opcode = UPD78F0730_CMD_LINE_CONTROL;
    request.baud_rate = cpu_to_le32(baud_rate);
    request.params = 0;
    dev_dbg(dev, "%s - baud rate = %d\n", __func__, baud_rate);
    switch (C_CSIZE(tty)) {
    case CS7:
    request.params |= UPD78F0730_DATA_SIZE_7_BITS;
    dev_dbg(dev, "%s - 7 data bits\n", __func__);
    break;
    default:
    tty.termios.c_cflag &= ~CSIZE;
    tty.termios.c_cflag |= CS8;
    dev_warn(dev, "data size is not supported, using 8 bits\n");
    fallthrough;
    case CS8:
    request.params |= UPD78F0730_DATA_SIZE_8_BITS;
    dev_dbg(dev, "%s - 8 data bits\n", __func__);
    break;
    }
    if (C_PARENB(tty)) {
    if (C_PARODD(tty)) {
    request.params |= UPD78F0730_PARITY_ODD;
    dev_dbg(dev, "%s - odd parity\n", __func__);
    } else {
    request.params |= UPD78F0730_PARITY_EVEN;
    dev_dbg(dev, "%s - even parity\n", __func__);
    }
    if (C_CMSPAR(tty)) {
    tty.termios.c_cflag &= ~CMSPAR;
    dev_warn(dev, "MARK/SPACE parity is not supported\n");
    }
    } else {
    request.params |= UPD78F0730_PARITY_NONE;
    dev_dbg(dev, "%s - no parity\n", __func__);
    }
    if (C_CSTOPB(tty)) {
    request.params |= UPD78F0730_STOP_BIT_2_BIT;
    dev_dbg(dev, "%s - 2 stop bits\n", __func__);
    } else {
    request.params |= UPD78F0730_STOP_BIT_1_BIT;
    dev_dbg(dev, "%s - 1 stop bit\n", __func__);
    }
    if (C_CRTSCTS(tty)) {
    tty.termios.c_cflag &= ~CRTSCTS;
    dev_warn(dev, "RTSCTS flow control is not supported\n");
    }
    if (I_IXOFF(tty) || I_IXON(tty)) {
    tty.termios.c_iflag &= ~(IXOFF | IXON);
    dev_warn(dev, "XON/XOFF flow control is not supported\n");
    }
    request.params |= UPD78F0730_FLOW_CONTROL_NONE;
    dev_dbg(dev, "%s - no flow control\n", __func__);
    upd78f0730_send_ctl(port, &request, sizeof(request));
    }
#[no_mangle]
unsafe extern "C" fn upd78f0730_open(tty: *mut tty_struct, port: *mut usb_serial_port) -> c_int {
    static int upd78f0730_open(struct tty_struct *tty, struct usb_serial_port *port)
    {
    static const struct upd78f0730_open_close request = {
    .opcode = UPD78F0730_CMD_OPEN_CLOSE,
    .state = UPD78F0730_PORT_OPEN
    };
    int res;
    res = upd78f0730_send_ctl(port, &request, sizeof(request));
    if (res)
    return res;
    if (tty)
    upd78f0730_set_termios(tty, port, core::ptr::null_mut());
    return usb_serial_generic_open(tty, port);
    }
#[no_mangle]
unsafe extern "C" fn upd78f0730_close(port: *mut usb_serial_port) {
    static void upd78f0730_close(struct usb_serial_port *port)
    {
    static const struct upd78f0730_open_close request = {
    .opcode = UPD78F0730_CMD_OPEN_CLOSE,
    .state = UPD78F0730_PORT_CLOSE
    };
    usb_serial_generic_close(port);
    upd78f0730_send_ctl(port, &request, sizeof(request));
    }
    static struct usb_serial_driver upd78f0730_device = {
    .driver	 = {
    .name	= "upd78f0730",
    },
    .id_table	= id_table,
    .num_ports	= 1,
    .port_probe	= upd78f0730_port_probe,
    .port_remove	= upd78f0730_port_remove,
    .open		= upd78f0730_open,
    .close		= upd78f0730_close,
    .set_termios	= upd78f0730_set_termios,
    .tiocmget	= upd78f0730_tiocmget,
    .tiocmset	= upd78f0730_tiocmset,
    .dtr_rts	= upd78f0730_dtr_rts,
    .break_ctl	= upd78f0730_break_ctl,
    };
    static struct usb_serial_driver * const serial_drivers[] = {
    &upd78f0730_device,
    core::ptr::null_mut()
    };
    module_usb_serial_driver(serial_drivers, id_table);
    MODULE_DESCRIPTION(DRIVER_DESC);
    MODULE_AUTHOR(DRIVER_AUTHOR);
    MODULE_LICENSE("GPL v2");
