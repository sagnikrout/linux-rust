//! Automatically rewritten from C to Rust
//! Source: drivers/usb/serial/metro-usb.c
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
    Some of this code is credited to Linux USB open source files that are
    distributed with Linux.
    Copyright:	2007 Metrologic Instruments. All rights reserved.
    Copyright:	2011 Azimut Ltd. <http://azimutrzn.ru/>
//

// Product information.
pub const FOCUS_VENDOR_ID: c_uint = 0x0C2E;
pub const FOCUS_PRODUCT_ID_BI: c_uint = 0x0720;
pub const FOCUS_PRODUCT_ID_UNI: c_uint = 0x0700;
pub const METROUSB_SET_REQUEST_TYPE: c_uint = 0x40;
pub const METROUSB_SET_MODEM_CTRL_REQUEST: c_int = 10;
pub const METROUSB_SET_BREAK_REQUEST: c_uint = 0x40;
pub const METROUSB_MCR_NONE: c_uint = 0x08	/* Deactivate DTR and RTS. */;
pub const METROUSB_MCR_RTS: c_uint = 0x0a	/* Activate RTS. */;
pub const METROUSB_MCR_DTR: c_uint = 0x09	/* Activate DTR. */;

// Private data structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct metrousb_private {
    pub lock: spinlock_t,
    pub throttled: c_int,
    pub throttle_req: c_int,
    pub control_state: c_ulong,
}

// Device table list.
    static const struct usb_device_id id_table[] = {
    { USB_DEVICE(FOCUS_VENDOR_ID, FOCUS_PRODUCT_ID_BI) },
    { USB_DEVICE(FOCUS_VENDOR_ID, FOCUS_PRODUCT_ID_UNI) },
    { USB_DEVICE_INTERFACE_CLASS(0x0c2e, 0x0730, 0xff) },	/* MS7820 */
    { }, /* Terminating entry. */
    };
    MODULE_DEVICE_TABLE(usb, id_table);
// UNI-Directional mode commands for device configure
pub const UNI_CMD_OPEN: c_uint = 0x80;
pub const UNI_CMD_CLOSE: c_uint = 0xFF;
#[no_mangle]
unsafe extern "C" fn metrousb_is_unidirectional_mode(serial: *mut usb_serial) -> c_int {
    static int metrousb_is_unidirectional_mode(struct usb_serial *serial)
    {
    let mut product_id: u16 = le16_to_cpu(serial.dev.descriptor.idProduct);
    let mut product_id: return = = FOCUS_PRODUCT_ID_UNI;
    }
    static int metrousb_calc_num_ports(struct usb_serial *serial,
    struct usb_serial_endpoints *epds)
    {
    if (metrousb_is_unidirectional_mode(serial)) {
    if (epds.num_interrupt_out == 0) {
    dev_err(&serial.interface.dev, "interrupt-out endpoint missing\n");
    return -ENODEV;
    }
    }
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn metrousb_send_unidirectional_cmd(cmd: u8, port: *mut usb_serial_port) -> c_int {
    static int metrousb_send_unidirectional_cmd(u8 cmd, struct usb_serial_port *port)
    {
    int ret;
    int actual_len;
    u8 *buffer_cmd = core::ptr::null_mut();
    if (!metrousb_is_unidirectional_mode(port.serial))
    return 0;
    buffer_cmd = kzalloc(sizeof(cmd), GFP_KERNEL);
    if (!buffer_cmd)
    return -ENOMEM;
// buffer_cmd = cmd;
    ret = usb_interrupt_msg(port.serial.dev,
    usb_sndintpipe(port.serial.dev, port.interrupt_out_endpointAddress),
    buffer_cmd, sizeof(cmd),
    &actual_len, USB_CTRL_SET_TIMEOUT);
    kfree(buffer_cmd);
    if (ret < 0)
    return ret;
#[no_mangle]
pub unsafe extern "C" fn if(sizeof(cmd): actual_len !=) -> else {
    else if (actual_len != sizeof(cmd))
    return -EIO;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn metrousb_read_int_callback(urb: *mut urb) {
    static void metrousb_read_int_callback(struct urb *urb)
    {
    struct usb_serial_port *port = urb.context;
    struct metrousb_private *metro_priv = usb_get_serial_port_data(port);
    unsigned char *data = urb.transfer_buffer;
    unsigned long flags;
    let mut throttled: c_int = 0;
    int result;
    dev_dbg(&port.dev, "%s\n", __func__);
    switch (urb.status) {
    case 0:
// Success status, read from the port.
    break;
    case -ECONNRESET:
    case -ENOENT:
    case -ESHUTDOWN:
// urb has been terminated.
    dev_dbg(&port.dev,
    "%s - urb shutting down, error code=%d\n",
    __func__, urb.status);
    return;
    default:
    dev_dbg(&port.dev,
    "%s - non-zero urb received, error code=%d\n",
    __func__, urb.status);
    goto exit;
    }
// Set the data read from the usb port into the serial port buffer.
    if (urb.actual_length) {
// Loop through the data copying each byte to the tty layer.
    tty_insert_flip_string(&port.port, data, urb.actual_length);
// Force the data to the tty layer.
    tty_flip_buffer_push(&port.port);
    }
// Set any port variables.
    spin_lock_irqsave(&metro_priv.lock, flags);
    if (metro_priv.throttle_req) {
    metro_priv.throttled = 1;
    throttled = 1;
    }
    spin_unlock_irqrestore(&metro_priv.lock, flags);
    if (throttled)
    return;
    exit:
// Try to resubmit the urb.
    result = usb_submit_urb(urb, GFP_ATOMIC);
    if (result)
    dev_err(&port.dev,
    "%s - failed submitting interrupt in urb, error code=%d\n",
    __func__, result);
    }
#[no_mangle]
unsafe extern "C" fn metrousb_cleanup(port: *mut usb_serial_port) {
    static void metrousb_cleanup(struct usb_serial_port *port)
    {
    usb_kill_urb(port.interrupt_in_urb);
    metrousb_send_unidirectional_cmd(UNI_CMD_CLOSE, port);
    }
#[no_mangle]
unsafe extern "C" fn metrousb_open(tty: *mut tty_struct, port: *mut usb_serial_port) -> c_int {
    static int metrousb_open(struct tty_struct *tty, struct usb_serial_port *port)
    {
    struct usb_serial *serial = port.serial;
    struct metrousb_private *metro_priv = usb_get_serial_port_data(port);
    unsigned long flags;
    int result;
// Set the private data information for the port.
    spin_lock_irqsave(&metro_priv.lock, flags);
    metro_priv.control_state = 0;
    metro_priv.throttled = 0;
    metro_priv.throttle_req = 0;
    spin_unlock_irqrestore(&metro_priv.lock, flags);
// Clear the urb pipe.
    usb_clear_halt(serial.dev, port.interrupt_in_urb.pipe);
// Start reading from the device
    usb_fill_int_urb(port.interrupt_in_urb, serial.dev,
    usb_rcvintpipe(serial.dev, port.interrupt_in_endpointAddress),
    port.interrupt_in_urb.transfer_buffer,
    port.interrupt_in_urb.transfer_buffer_length,
    metrousb_read_int_callback, port, 1);
    result = usb_submit_urb(port.interrupt_in_urb, GFP_KERNEL);
    if (result) {
    dev_err(&port.dev,
    "%s - failed submitting interrupt in urb, error code=%d\n",
    __func__, result);
    return result;
    }
// Send activate cmd to device
    result = metrousb_send_unidirectional_cmd(UNI_CMD_OPEN, port);
    if (result) {
    dev_err(&port.dev,
    "%s - failed to configure device, error code=%d\n",
    __func__, result);
    goto err_kill_urb;
    }
    return 0;
    err_kill_urb:
    usb_kill_urb(port.interrupt_in_urb);
    return result;
    }
#[no_mangle]
unsafe extern "C" fn metrousb_set_modem_ctrl(serial: *mut usb_serial, control_state: c_uint) -> c_int {
    static int metrousb_set_modem_ctrl(struct usb_serial *serial, unsigned int control_state)
    {
    let mut retval: c_int = 0;
    let mut mcr: c_uchar = METROUSB_MCR_NONE;
    dev_dbg(&serial.dev.dev, "%s - control state = %d\n",
    __func__, control_state);
// Set the modem control value.
    if (control_state & TIOCM_DTR)
    mcr |= METROUSB_MCR_DTR;
    if (control_state & TIOCM_RTS)
    mcr |= METROUSB_MCR_RTS;
// Send the command to the usb port.
    retval = usb_control_msg(serial.dev, usb_sndctrlpipe(serial.dev, 0),
    METROUSB_SET_REQUEST_TYPE, METROUSB_SET_MODEM_CTRL_REQUEST,
    control_state, 0, core::ptr::null_mut(), 0, WDR_TIMEOUT);
    if (retval < 0)
    dev_err(&serial.dev.dev,
    "%s - set modem ctrl=0x%x failed, error code=%d\n",
    __func__, mcr, retval);
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn metrousb_port_probe(port: *mut usb_serial_port) -> c_int {
    static int metrousb_port_probe(struct usb_serial_port *port)
    {
    struct metrousb_private *metro_priv;
    metro_priv = kzalloc_obj(*metro_priv);
    if (!metro_priv)
    return -ENOMEM;
    spin_lock_init(&metro_priv.lock);
    usb_set_serial_port_data(port, metro_priv);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn metrousb_port_remove(port: *mut usb_serial_port) {
    static void metrousb_port_remove(struct usb_serial_port *port)
    {
    struct metrousb_private *metro_priv;
    metro_priv = usb_get_serial_port_data(port);
    kfree(metro_priv);
    }
#[no_mangle]
unsafe extern "C" fn metrousb_throttle(tty: *mut tty_struct) {
    static void metrousb_throttle(struct tty_struct *tty)
    {
    struct usb_serial_port *port = tty.driver_data;
    struct metrousb_private *metro_priv = usb_get_serial_port_data(port);
    unsigned long flags;
// Set the private information for the port to stop reading data.
    spin_lock_irqsave(&metro_priv.lock, flags);
    metro_priv.throttle_req = 1;
    spin_unlock_irqrestore(&metro_priv.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn metrousb_tiocmget(tty: *mut tty_struct) -> c_int {
    static int metrousb_tiocmget(struct tty_struct *tty)
    {
    let mut control_state: c_ulong = 0;
    struct usb_serial_port *port = tty.driver_data;
    struct metrousb_private *metro_priv = usb_get_serial_port_data(port);
    unsigned long flags;
    spin_lock_irqsave(&metro_priv.lock, flags);
    control_state = metro_priv.control_state;
    spin_unlock_irqrestore(&metro_priv.lock, flags);
    return control_state;
    }
    static int metrousb_tiocmset(struct tty_struct *tty,
    unsigned int set, unsigned int clear)
    {
    struct usb_serial_port *port = tty.driver_data;
    struct usb_serial *serial = port.serial;
    struct metrousb_private *metro_priv = usb_get_serial_port_data(port);
    unsigned long flags;
    let mut control_state: c_ulong = 0;
    dev_dbg(&port.dev, "%s - set=%d, clear=%d\n", __func__, set, clear);
    spin_lock_irqsave(&metro_priv.lock, flags);
    control_state = metro_priv.control_state;
// Set the RTS and DTR values.
    if (set & TIOCM_RTS)
    control_state |= TIOCM_RTS;
    if (set & TIOCM_DTR)
    control_state |= TIOCM_DTR;
    if (clear & TIOCM_RTS)
    control_state &= ~TIOCM_RTS;
    if (clear & TIOCM_DTR)
    control_state &= ~TIOCM_DTR;
    metro_priv.control_state = control_state;
    spin_unlock_irqrestore(&metro_priv.lock, flags);
    return metrousb_set_modem_ctrl(serial, control_state);
    }
#[no_mangle]
unsafe extern "C" fn metrousb_unthrottle(tty: *mut tty_struct) {
    static void metrousb_unthrottle(struct tty_struct *tty)
    {
    struct usb_serial_port *port = tty.driver_data;
    struct metrousb_private *metro_priv = usb_get_serial_port_data(port);
    unsigned long flags;
    int throttled;
    int result;
// Set the private information for the port to resume reading data.
    spin_lock_irqsave(&metro_priv.lock, flags);
    throttled = metro_priv.throttled;
    metro_priv.throttled = 0;
    metro_priv.throttle_req = 0;
    spin_unlock_irqrestore(&metro_priv.lock, flags);
    if (throttled) {
    result = usb_submit_urb(port.interrupt_in_urb, GFP_KERNEL);
    if (result) {
    dev_err(&port.dev, "failed to submit interrupt in urb: %d\n",
    result);
    }
    }
    }
    static struct usb_serial_driver metrousb_device = {
    .driver = {
    .name =		"metro-usb",
    },
    .description		= "Metrologic USB to Serial",
    .id_table		= id_table,
    .num_interrupt_in	= 1,
    .calc_num_ports		= metrousb_calc_num_ports,
    .open			= metrousb_open,
    .close			= metrousb_cleanup,
    .read_int_callback	= metrousb_read_int_callback,
    .port_probe		= metrousb_port_probe,
    .port_remove		= metrousb_port_remove,
    .throttle		= metrousb_throttle,
    .unthrottle		= metrousb_unthrottle,
    .tiocmget		= metrousb_tiocmget,
    .tiocmset		= metrousb_tiocmset,
    };
    static struct usb_serial_driver * const serial_drivers[] = {
    &metrousb_device,
    core::ptr::null_mut(),
    };
    module_usb_serial_driver(serial_drivers, id_table);
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Philip Nicastro");
    MODULE_AUTHOR("Aleksey Babahin <tamerlan311@gmail.com>");
    MODULE_DESCRIPTION(DRIVER_DESC);
