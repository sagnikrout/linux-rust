//! Automatically rewritten from C to Rust
//! Source: drivers/usb/serial/mxuport.c
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
// mxuport.c - MOXA UPort series driver
//
// Copyright (c) 2006 Moxa Technologies Co., Ltd.
// Copyright (c) 2013 Andrew Lunn <andrew@lunn.ch>
//
// Supports the following Moxa USB to serial converters:
// 2 ports : UPort 1250, UPort 1250I
// 4 ports : UPort 1410, UPort 1450, UPort 1450I
// 8 ports : UPort 1610-8, UPort 1650-8
// 16 ports : UPort 1610-16, UPort 1650-16
//

// Definitions for the vendor ID and device ID
pub const MX_USBSERIAL_VID: c_uint = 0x110A;
pub const MX_UPORT1250_PID: c_uint = 0x1250;
pub const MX_UPORT1251_PID: c_uint = 0x1251;
pub const MX_UPORT1410_PID: c_uint = 0x1410;
pub const MX_UPORT1450_PID: c_uint = 0x1450;
pub const MX_UPORT1451_PID: c_uint = 0x1451;
pub const MX_UPORT1618_PID: c_uint = 0x1618;
pub const MX_UPORT1658_PID: c_uint = 0x1658;
pub const MX_UPORT1613_PID: c_uint = 0x1613;
pub const MX_UPORT1653_PID: c_uint = 0x1653;
// Definitions for USB info
pub const HEADER_SIZE: c_int = 4;
pub const EVENT_LENGTH: c_int = 8;
pub const DOWN_BLOCK_SIZE: c_int = 64;
// Definitions for firmware info
pub const VER_ADDR_1: c_uint = 0x20;
pub const VER_ADDR_2: c_uint = 0x24;
pub const VER_ADDR_3: c_uint = 0x28;
// Definitions for USB vendor request
pub const RQ_VENDOR_NONE: c_uint = 0x00;
pub const RQ_VENDOR_SET_BAUD: c_uint = 0x01 /* Set baud rate */;
pub const RQ_VENDOR_SET_LINE: c_uint = 0x02 /* Set line status */;
pub const RQ_VENDOR_SET_CHARS: c_uint = 0x03 /* Set Xon/Xoff chars */;
pub const RQ_VENDOR_SET_RTS: c_uint = 0x04 /* Set RTS */;
pub const RQ_VENDOR_SET_DTR: c_uint = 0x05 /* Set DTR */;
pub const RQ_VENDOR_SET_XONXOFF: c_uint = 0x06 /* Set auto Xon/Xoff */;
pub const RQ_VENDOR_SET_RX_HOST_EN: c_uint = 0x07 /* Set RX host enable */;
pub const RQ_VENDOR_SET_OPEN: c_uint = 0x08 /* Set open/close port */;
pub const RQ_VENDOR_PURGE: c_uint = 0x09 /* Purge Rx/Tx buffer */;
pub const RQ_VENDOR_SET_MCR: c_uint = 0x0A /* Set MCR register */;
pub const RQ_VENDOR_SET_BREAK: c_uint = 0x0B /* Set Break signal */;
pub const RQ_VENDOR_START_FW_DOWN: c_uint = 0x0C /* Start firmware download */;
pub const RQ_VENDOR_STOP_FW_DOWN: c_uint = 0x0D /* Stop firmware download */;
pub const RQ_VENDOR_QUERY_FW_READY: c_uint = 0x0E /* Query if new firmware ready */;
pub const RQ_VENDOR_SET_FIFO_DISABLE: c_uint = 0x0F /* Set fifo disable */;
pub const RQ_VENDOR_SET_INTERFACE: c_uint = 0x10 /* Set interface */;
pub const RQ_VENDOR_SET_HIGH_PERFOR: c_uint = 0x11 /* Set hi-performance */;
pub const RQ_VENDOR_ERASE_BLOCK: c_uint = 0x12 /* Erase flash block */;
pub const RQ_VENDOR_WRITE_PAGE: c_uint = 0x13 /* Write flash page */;
pub const RQ_VENDOR_PREPARE_WRITE: c_uint = 0x14 /* Prepare write flash */;
pub const RQ_VENDOR_CONFIRM_WRITE: c_uint = 0x15 /* Confirm write flash */;
pub const RQ_VENDOR_LOCATE: c_uint = 0x16 /* Locate the device */;
pub const RQ_VENDOR_START_ROM_DOWN: c_uint = 0x17 /* Start firmware download */;
pub const RQ_VENDOR_ROM_DATA: c_uint = 0x18 /* Rom file data */;
pub const RQ_VENDOR_STOP_ROM_DOWN: c_uint = 0x19 /* Stop firmware download */;
pub const RQ_VENDOR_FW_DATA: c_uint = 0x20 /* Firmware data */;
pub const RQ_VENDOR_RESET_DEVICE: c_uint = 0x23 /* Try to reset the device */;
pub const RQ_VENDOR_QUERY_FW_CONFIG: c_uint = 0x24;
pub const RQ_VENDOR_GET_VERSION: c_uint = 0x81 /* Get firmware version */;
pub const RQ_VENDOR_GET_PAGE: c_uint = 0x82 /* Read flash page */;
pub const RQ_VENDOR_GET_ROM_PROC: c_uint = 0x83 /* Get ROM process state */;
pub const RQ_VENDOR_GET_INQUEUE: c_uint = 0x84 /* Data in input buffer */;
pub const RQ_VENDOR_GET_OUTQUEUE: c_uint = 0x85 /* Data in output buffer */;
pub const RQ_VENDOR_GET_MSR: c_uint = 0x86 /* Get modem status register */;
// Definitions for UPort event type

// Definitions for serial event type
pub const SERIAL_EV_CTS: c_uint = 0x0008	/* CTS changed state */;
pub const SERIAL_EV_DSR: c_uint = 0x0010	/* DSR changed state */;
pub const SERIAL_EV_RLSD: c_uint = 0x0020	/* RLSD changed state */;
// Definitions for modem control event type
pub const SERIAL_EV_XOFF: c_uint = 0x40	/* XOFF received */;
// Definitions for line control of communication
pub const MX_WORDLENGTH_5: c_int = 5;
pub const MX_WORDLENGTH_6: c_int = 6;
pub const MX_WORDLENGTH_7: c_int = 7;
pub const MX_WORDLENGTH_8: c_int = 8;
pub const MX_PARITY_NONE: c_int = 0;
pub const MX_PARITY_ODD: c_int = 1;
pub const MX_PARITY_EVEN: c_int = 2;
pub const MX_PARITY_MARK: c_int = 3;
pub const MX_PARITY_SPACE: c_int = 4;
pub const MX_STOP_BITS_1: c_int = 0;
pub const MX_STOP_BITS_1_5: c_int = 1;
pub const MX_STOP_BITS_2: c_int = 2;
pub const MX_RTS_DISABLE: c_uint = 0x0;
pub const MX_RTS_ENABLE: c_uint = 0x1;
pub const MX_RTS_HW: c_uint = 0x2;
pub const MX_RTS_NO_CHANGE: c_uint = 0x3 /* Flag, not valid register value*/;
pub const MX_INT_RS232: c_int = 0;
pub const MX_INT_2W_RS485: c_int = 1;
pub const MX_INT_RS422: c_int = 2;
pub const MX_INT_4W_RS485: c_int = 3;
// Definitions for holding reason
pub const MX_WAIT_FOR_CTS: c_uint = 0x0001;
pub const MX_WAIT_FOR_DSR: c_uint = 0x0002;
pub const MX_WAIT_FOR_DCD: c_uint = 0x0004;
pub const MX_WAIT_FOR_XON: c_uint = 0x0008;
pub const MX_WAIT_FOR_START_TX: c_uint = 0x0010;
pub const MX_WAIT_FOR_UNTHROTTLE: c_uint = 0x0020;
pub const MX_WAIT_FOR_LOW_WATER: c_uint = 0x0040;
pub const MX_WAIT_FOR_SEND_NEXT: c_uint = 0x0080;
// This structure holds all of the local port information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxuport_port {
    pub /: *mut *mut u8 mcr_state; / Last MCR state,
    pub /: *mut *mut u8 msr_state; / Last MSR state,
    pub /: *mut *mut mutex mutex; / Protects mcr_state,
    pub /: *mut *mut spinlock_t spinlock; / Protects msr_state,
}

// Encode number of ports (2..16 or undefined)

pub const MX_PORTS_OFFSET: c_int = 1;

// Table of devices that work with this driver
    static const struct usb_device_id mxuport_idtable[] = {
    { USB_DEVICE(MX_USBSERIAL_VID, MX_UPORT1250_PID),
    .driver_info = MX_PORTS(2) },
    { USB_DEVICE(MX_USBSERIAL_VID, MX_UPORT1251_PID),
    .driver_info = MX_PORTS(2) },
    { USB_DEVICE(MX_USBSERIAL_VID, MX_UPORT1410_PID),
    .driver_info = MX_PORTS(4) },
    { USB_DEVICE(MX_USBSERIAL_VID, MX_UPORT1450_PID),
    .driver_info = MX_PORTS(4) },
    { USB_DEVICE(MX_USBSERIAL_VID, MX_UPORT1451_PID),
    .driver_info = MX_PORTS(4) },
    { USB_DEVICE(MX_USBSERIAL_VID, MX_UPORT1618_PID),
    .driver_info = MX_PORTS(8) },
    { USB_DEVICE(MX_USBSERIAL_VID, MX_UPORT1658_PID),
    .driver_info = MX_PORTS(8) },
    { USB_DEVICE(MX_USBSERIAL_VID, MX_UPORT1613_PID),
    .driver_info = MX_PORTS(16) },
    { USB_DEVICE(MX_USBSERIAL_VID, MX_UPORT1653_PID),
    .driver_info = MX_PORTS(16) },
    {}			/* Terminating entry */
    };
    MODULE_DEVICE_TABLE(usb, mxuport_idtable);
//
// Add a four byte header containing the port number and the number of
// bytes of data in the message. Return the number of bytes in the
// buffer.
//
    static int mxuport_prepare_write_buffer(struct usb_serial_port *port,
    void *dest, size_t size)
    {
    u8 *buf = dest;
    int count;
    count = kfifo_out_locked(&port.write_fifo, buf + HEADER_SIZE,
    size - HEADER_SIZE,
    &port.lock);
    put_unaligned_be16(port.port_number, buf);
    put_unaligned_be16(count, buf + 2);
    dev_dbg(&port.dev, "%s - size %zd count %d\n", __func__,
    size, count);
    return count + HEADER_SIZE;
    }
// Read the given buffer in from the control pipe.
    static int mxuport_recv_ctrl_urb(struct usb_serial *serial,
    u8 request, u16 value, u16 index,
    u8 *data, size_t size)
    {
    int status;
    status = usb_control_msg(serial.dev,
    usb_rcvctrlpipe(serial.dev, 0),
    request,
    (USB_DIR_IN | USB_TYPE_VENDOR |
    USB_RECIP_DEVICE), value, index,
    data, size,
    USB_CTRL_GET_TIMEOUT);
    if (status < 0) {
    dev_err(&serial.interface.dev,
    "%s - usb_control_msg failed (%d)\n",
    __func__, status);
    return status;
    }
    if (status != size) {
    dev_err(&serial.interface.dev,
    "%s - short read (%d / %zd)\n",
    __func__, status, size);
    return -EIO;
    }
    return status;
    }
// Write the given buffer out to the control pipe.
    static int mxuport_send_ctrl_data_urb(struct usb_serial *serial,
    u8 request,
    u16 value, u16 index,
    u8 *data, size_t size)
    {
    int status;
    status = usb_control_msg(serial.dev,
    usb_sndctrlpipe(serial.dev, 0),
    request,
    (USB_DIR_OUT | USB_TYPE_VENDOR |
    USB_RECIP_DEVICE), value, index,
    data, size,
    USB_CTRL_SET_TIMEOUT);
    if (status < 0) {
    dev_err(&serial.interface.dev,
    "%s - usb_control_msg failed (%d)\n",
    __func__, status);
    return status;
    }
    return 0;
    }
// Send a vendor request without any data
    static int mxuport_send_ctrl_urb(struct usb_serial *serial,
    u8 request, u16 value, u16 index)
    {
    return mxuport_send_ctrl_data_urb(serial, request, value, index,
    core::ptr::null_mut(), 0);
    }
//
// mxuport_throttle - throttle function of driver
//
// This function is called by the tty driver when it wants to stop the
// data being read from the port. Since all the data comes over one
// bulk in endpoint, we cannot stop submitting urbs by setting
// port->throttle. Instead tell the device to stop sending us data for
// the port.
//
#[no_mangle]
unsafe extern "C" fn mxuport_throttle(tty: *mut tty_struct) {
    static void mxuport_throttle(struct tty_struct *tty)
    {
    struct usb_serial_port *port = tty.driver_data;
    struct usb_serial *serial = port.serial;
    dev_dbg(&port.dev, "%s\n", __func__);
    mxuport_send_ctrl_urb(serial, RQ_VENDOR_SET_RX_HOST_EN,
    0, port.port_number);
    }
//
// mxuport_unthrottle - unthrottle function of driver
//
// This function is called by the tty driver when it wants to resume
// the data being read from the port. Tell the device it can resume
// sending us received data from the port.
//
#[no_mangle]
unsafe extern "C" fn mxuport_unthrottle(tty: *mut tty_struct) {
    static void mxuport_unthrottle(struct tty_struct *tty)
    {
    struct usb_serial_port *port = tty.driver_data;
    struct usb_serial *serial = port.serial;
    dev_dbg(&port.dev, "%s\n", __func__);
    mxuport_send_ctrl_urb(serial, RQ_VENDOR_SET_RX_HOST_EN,
    1, port.port_number);
    }
//
// Processes one chunk of data received for a port.  Mostly a copy of
// usb_serial_generic_process_read_urb().
//
    static void mxuport_process_read_urb_data(struct usb_serial_port *port,
    char *data, int size)
    {
    int i;
    if (port.sysrq) {
    for (i = 0; i < size; i++, data++) {
    if (!usb_serial_handle_sysrq_char(port, *data))
    tty_insert_flip_char(&port.port, *data,
    TTY_NORMAL);
    }
    } else {
    tty_insert_flip_string(&port.port, data, size);
    }
    tty_flip_buffer_push(&port.port);
    }
#[no_mangle]
unsafe extern "C" fn mxuport_msr_event(port: *mut usb_serial_port, buf[4]: u8) {
    static void mxuport_msr_event(struct usb_serial_port *port, u8 buf[4])
    {
    struct mxuport_port *mxport = usb_get_serial_port_data(port);
    let mut rcv_msr_hold: u8 = buf[2] & 0xF0;
    let mut rcv_msr_event: u16 = get_unaligned_be16(buf);
    unsigned long flags;
    if (rcv_msr_event == 0)
    return;
// Update MSR status
    spin_lock_irqsave(&mxport.spinlock, flags);
    dev_dbg(&port.dev, "%s - current MSR status = 0x%x\n",
    __func__, mxport.msr_state);
    if (rcv_msr_hold & UART_MSR_CTS) {
    mxport.msr_state |= UART_MSR_CTS;
    dev_dbg(&port.dev, "%s - CTS high\n", __func__);
    } else {
    mxport.msr_state &= ~UART_MSR_CTS;
    dev_dbg(&port.dev, "%s - CTS low\n", __func__);
    }
    if (rcv_msr_hold & UART_MSR_DSR) {
    mxport.msr_state |= UART_MSR_DSR;
    dev_dbg(&port.dev, "%s - DSR high\n", __func__);
    } else {
    mxport.msr_state &= ~UART_MSR_DSR;
    dev_dbg(&port.dev, "%s - DSR low\n", __func__);
    }
    if (rcv_msr_hold & UART_MSR_DCD) {
    mxport.msr_state |= UART_MSR_DCD;
    dev_dbg(&port.dev, "%s - DCD high\n", __func__);
    } else {
    mxport.msr_state &= ~UART_MSR_DCD;
    dev_dbg(&port.dev, "%s - DCD low\n", __func__);
    }
    spin_unlock_irqrestore(&mxport.spinlock, flags);
    if (rcv_msr_event &
    (SERIAL_EV_CTS | SERIAL_EV_DSR | SERIAL_EV_RLSD)) {
    if (rcv_msr_event & SERIAL_EV_CTS) {
    port.icount.cts++;
    dev_dbg(&port.dev, "%s - CTS change\n", __func__);
    }
    if (rcv_msr_event & SERIAL_EV_DSR) {
    port.icount.dsr++;
    dev_dbg(&port.dev, "%s - DSR change\n", __func__);
    }
    if (rcv_msr_event & SERIAL_EV_RLSD) {
    port.icount.dcd++;
    dev_dbg(&port.dev, "%s - DCD change\n", __func__);
    }
    wake_up_interruptible(&port.port.delta_msr_wait);
    }
    }
#[no_mangle]
unsafe extern "C" fn mxuport_lsr_event(port: *mut usb_serial_port, buf[4]: u8) {
    static void mxuport_lsr_event(struct usb_serial_port *port, u8 buf[4])
    {
    let mut lsr_event: u8 = buf[2];
    if (lsr_event & UART_LSR_BI) {
    port.icount.brk++;
    dev_dbg(&port.dev, "%s - break error\n", __func__);
    }
    if (lsr_event & UART_LSR_FE) {
    port.icount.frame++;
    dev_dbg(&port.dev, "%s - frame error\n", __func__);
    }
    if (lsr_event & UART_LSR_PE) {
    port.icount.parity++;
    dev_dbg(&port.dev, "%s - parity error\n", __func__);
    }
    if (lsr_event & UART_LSR_OE) {
    port.icount.overrun++;
    dev_dbg(&port.dev, "%s - overrun error\n", __func__);
    }
    }
//
// When something interesting happens, modem control lines XON/XOFF
// etc, the device sends an event. Process these events.
//
    static void mxuport_process_read_urb_event(struct usb_serial_port *port,
    u8 buf[4], u32 event)
    {
    dev_dbg(&port.dev, "%s - receive event : %04x\n", __func__, event);
    switch (event) {
    case UPORT_EVENT_SEND_NEXT:
//
// Sent as part of the flow control on device buffers.
// Not currently used.
//
    break;
    case UPORT_EVENT_MSR:
    mxuport_msr_event(port, buf);
    break;
    case UPORT_EVENT_LSR:
    mxuport_lsr_event(port, buf);
    break;
    case UPORT_EVENT_MCR:
//
// Event to indicate a change in XON/XOFF from the
// peer.  Currently not used. We just continue
// sending the device data and it will buffer it if
// needed. This event could be used for flow control
// between the host and the device.
//
    break;
    default:
    dev_dbg(&port.dev, "Unexpected event\n");
    break;
    }
    }
//
// One URB can contain data for multiple ports. Demultiplex the data,
// checking the port exists, is opened and the message is valid.
//
#[no_mangle]
unsafe extern "C" fn mxuport_process_read_urb_demux_data(urb: *mut urb) {
    static void mxuport_process_read_urb_demux_data(struct urb *urb)
    {
    struct usb_serial_port *port = urb.context;
    struct usb_serial *serial = port.serial;
    u8 *data = urb.transfer_buffer;
    u8 *end = data + urb.actual_length;
    struct usb_serial_port *demux_port;
    u8 *ch;
    u16 rcv_port;
    u16 rcv_len;
    while (data < end) {
    if (data + HEADER_SIZE > end) {
    dev_warn(&port.dev, "%s - message with short header\n",
    __func__);
    return;
    }
    rcv_port = get_unaligned_be16(data);
    if (rcv_port >= serial.num_ports) {
    dev_warn(&port.dev, "%s - message for invalid port\n",
    __func__);
    return;
    }
    demux_port = serial.port[rcv_port];
    rcv_len = get_unaligned_be16(data + 2);
    if (!rcv_len || data + HEADER_SIZE + rcv_len > end) {
    dev_warn(&port.dev, "%s - short data\n", __func__);
    return;
    }
    if (tty_port_initialized(&demux_port.port)) {
    ch = data + HEADER_SIZE;
    mxuport_process_read_urb_data(demux_port, ch, rcv_len);
    } else {
    dev_dbg(&demux_port.dev, "%s - data for closed port\n",
    __func__);
    }
    data += HEADER_SIZE + rcv_len;
    }
    }
//
// One URB can contain events for multiple ports. Demultiplex the event,
// checking the port exists, and is opened.
//
#[no_mangle]
unsafe extern "C" fn mxuport_process_read_urb_demux_event(urb: *mut urb) {
    static void mxuport_process_read_urb_demux_event(struct urb *urb)
    {
    struct usb_serial_port *port = urb.context;
    struct usb_serial *serial = port.serial;
    u8 *data = urb.transfer_buffer;
    u8 *end = data + urb.actual_length;
    struct usb_serial_port *demux_port;
    u8 *ch;
    u16 rcv_port;
    u16 rcv_event;
    while (data < end) {
    if (data + EVENT_LENGTH > end) {
    dev_warn(&port.dev, "%s - message with short event\n",
    __func__);
    return;
    }
    rcv_port = get_unaligned_be16(data);
    if (rcv_port >= serial.num_ports) {
    dev_warn(&port.dev, "%s - message for invalid port\n",
    __func__);
    return;
    }
    demux_port = serial.port[rcv_port];
    if (tty_port_initialized(&demux_port.port)) {
    ch = data + HEADER_SIZE;
    rcv_event = get_unaligned_be16(data + 2);
    mxuport_process_read_urb_event(demux_port, ch,
    rcv_event);
    } else {
    dev_dbg(&demux_port.dev,
    "%s - event for closed port\n", __func__);
    }
    data += EVENT_LENGTH;
    }
    }
//
// This is called when we have received data on the bulk in
// endpoint. Depending on which port it was received on, it can
// contain serial data or events.
//
#[no_mangle]
unsafe extern "C" fn mxuport_process_read_urb(urb: *mut urb) {
    static void mxuport_process_read_urb(struct urb *urb)
    {
    struct usb_serial_port *port = urb.context;
    struct usb_serial *serial = port.serial;
    if (port == serial.port[0])
    mxuport_process_read_urb_demux_data(urb);
    if (port == serial.port[1])
    mxuport_process_read_urb_demux_event(urb);
    }
//
// Ask the device how many bytes it has queued to be sent out. If
// there are none, return true.
//
#[no_mangle]
unsafe extern "C" fn mxuport_tx_empty(port: *mut usb_serial_port) -> bool {
    static bool mxuport_tx_empty(struct usb_serial_port *port)
    {
    struct usb_serial *serial = port.serial;
    let mut is_empty: bool = true;
    u32 txlen;
    u8 *len_buf;
    int err;
    len_buf = kzalloc(4, GFP_KERNEL);
    if (!len_buf)
    goto out;
    err = mxuport_recv_ctrl_urb(serial, RQ_VENDOR_GET_OUTQUEUE, 0,
    port.port_number, len_buf, 4);
    if (err < 0)
    goto out;
    txlen = get_unaligned_be32(len_buf);
    dev_dbg(&port.dev, "%s - tx len = %u\n", __func__, txlen);
    if (txlen != 0)
    is_empty = false;
    out:
    kfree(len_buf);
    return is_empty;
    }
#[no_mangle]
unsafe extern "C" fn mxuport_set_mcr(port: *mut usb_serial_port, mcr_state: u8) -> c_int {
    static int mxuport_set_mcr(struct usb_serial_port *port, u8 mcr_state)
    {
    struct usb_serial *serial = port.serial;
    int err;
    dev_dbg(&port.dev, "%s - %02x\n", __func__, mcr_state);
    err = mxuport_send_ctrl_urb(serial, RQ_VENDOR_SET_MCR,
    mcr_state, port.port_number);
    if (err)
    dev_err(&port.dev, "%s - failed to change MCR\n", __func__);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn mxuport_set_dtr(port: *mut usb_serial_port, on: c_int) -> c_int {
    static int mxuport_set_dtr(struct usb_serial_port *port, int on)
    {
    struct mxuport_port *mxport = usb_get_serial_port_data(port);
    struct usb_serial *serial = port.serial;
    int err;
    mutex_lock(&mxport.mutex);
    err = mxuport_send_ctrl_urb(serial, RQ_VENDOR_SET_DTR,
    !!on, port.port_number);
    if (!err) {
    if (on)
    mxport.mcr_state |= UART_MCR_DTR;
    else
    mxport.mcr_state &= ~UART_MCR_DTR;
    }
    mutex_unlock(&mxport.mutex);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn mxuport_set_rts(port: *mut usb_serial_port, state: u8) -> c_int {
    static int mxuport_set_rts(struct usb_serial_port *port, u8 state)
    {
    struct mxuport_port *mxport = usb_get_serial_port_data(port);
    struct usb_serial *serial = port.serial;
    int err;
    u8 mcr_state;
    mutex_lock(&mxport.mutex);
    mcr_state = mxport.mcr_state;
    switch (state) {
    case MX_RTS_DISABLE:
    mcr_state &= ~UART_MCR_RTS;
    break;
    case MX_RTS_ENABLE:
    mcr_state |= UART_MCR_RTS;
    break;
    case MX_RTS_HW:
//
// Do not update mxport->mcr_state when doing hardware
// flow control.
//
    break;
    default:
//
// Should not happen, but somebody might try passing
// MX_RTS_NO_CHANGE, which is not valid.
//
    err = -EINVAL;
    goto out;
    }
    err = mxuport_send_ctrl_urb(serial, RQ_VENDOR_SET_RTS,
    state, port.port_number);
    if (!err)
    mxport.mcr_state = mcr_state;
    out:
    mutex_unlock(&mxport.mutex);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn mxuport_dtr_rts(port: *mut usb_serial_port, on: c_int) {
    static void mxuport_dtr_rts(struct usb_serial_port *port, int on)
    {
    struct mxuport_port *mxport = usb_get_serial_port_data(port);
    u8 mcr_state;
    int err;
    mutex_lock(&mxport.mutex);
    mcr_state = mxport.mcr_state;
    if (on)
    mcr_state |= (UART_MCR_RTS | UART_MCR_DTR);
    else
    mcr_state &= ~(UART_MCR_RTS | UART_MCR_DTR);
    err = mxuport_set_mcr(port, mcr_state);
    if (!err)
    mxport.mcr_state = mcr_state;
    mutex_unlock(&mxport.mutex);
    }
    static int mxuport_tiocmset(struct tty_struct *tty, unsigned int set,
    unsigned int clear)
    {
    struct usb_serial_port *port = tty.driver_data;
    struct mxuport_port *mxport = usb_get_serial_port_data(port);
    int err;
    u8 mcr_state;
    mutex_lock(&mxport.mutex);
    mcr_state = mxport.mcr_state;
    if (set & TIOCM_RTS)
    mcr_state |= UART_MCR_RTS;
    if (set & TIOCM_DTR)
    mcr_state |= UART_MCR_DTR;
    if (clear & TIOCM_RTS)
    mcr_state &= ~UART_MCR_RTS;
    if (clear & TIOCM_DTR)
    mcr_state &= ~UART_MCR_DTR;
    err = mxuport_set_mcr(port, mcr_state);
    if (!err)
    mxport.mcr_state = mcr_state;
    mutex_unlock(&mxport.mutex);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn mxuport_tiocmget(tty: *mut tty_struct) -> c_int {
    static int mxuport_tiocmget(struct tty_struct *tty)
    {
    struct mxuport_port *mxport;
    struct usb_serial_port *port = tty.driver_data;
    unsigned int result;
    unsigned long flags;
    unsigned int msr;
    unsigned int mcr;
    mxport = usb_get_serial_port_data(port);
    mutex_lock(&mxport.mutex);
    spin_lock_irqsave(&mxport.spinlock, flags);
    msr = mxport.msr_state;
    mcr = mxport.mcr_state;
    spin_unlock_irqrestore(&mxport.spinlock, flags);
    mutex_unlock(&mxport.mutex);
    result = (((mcr & UART_MCR_DTR) ? TIOCM_DTR : 0) |	/* 0x002 */
    ((mcr & UART_MCR_RTS) ? TIOCM_RTS : 0) |	/* 0x004 */
    ((msr & UART_MSR_CTS) ? TIOCM_CTS : 0) |	/* 0x020 */
    ((msr & UART_MSR_DCD) ? TIOCM_CAR : 0) |	/* 0x040 */
    ((msr & UART_MSR_RI) ? TIOCM_RI : 0) |	/* 0x080 */
    ((msr & UART_MSR_DSR) ? TIOCM_DSR : 0));	/* 0x100 */
    dev_dbg(&port.dev, "%s - 0x%04x\n", __func__, result);
    return result;
    }
    static int mxuport_set_termios_flow(struct tty_struct *tty,
    const struct ktermios *old_termios,
    struct usb_serial_port *port,
    struct usb_serial *serial)
    {
    let mut xon: u8 = START_CHAR(tty);
    let mut xoff: u8 = STOP_CHAR(tty);
    int enable;
    int err;
    u8 *buf;
    u8 rts;
    buf = kmalloc(2, GFP_KERNEL);
    if (!buf)
    return -ENOMEM;
// S/W flow control settings
    if (I_IXOFF(tty) || I_IXON(tty)) {
    enable = 1;
    buf[0] = xon;
    buf[1] = xoff;
    err = mxuport_send_ctrl_data_urb(serial, RQ_VENDOR_SET_CHARS,
    0, port.port_number,
    buf, 2);
    if (err)
    goto out;
    dev_dbg(&port.dev, "%s - XON = 0x%02x, XOFF = 0x%02x\n",
    __func__, xon, xoff);
    } else {
    enable = 0;
    }
    err = mxuport_send_ctrl_urb(serial, RQ_VENDOR_SET_XONXOFF,
    enable, port.port_number);
    if (err)
    goto out;
    rts = MX_RTS_NO_CHANGE;
// H/W flow control settings
    if (!old_termios ||
    C_CRTSCTS(tty) != (old_termios.c_cflag & CRTSCTS)) {
    if (C_CRTSCTS(tty))
    rts = MX_RTS_HW;
    else
    rts = MX_RTS_ENABLE;
    }
    if (C_BAUD(tty)) {
    if (old_termios && (old_termios.c_cflag & CBAUD) == B0) {
// Raise DTR and RTS
    if (C_CRTSCTS(tty))
    rts = MX_RTS_HW;
    else
    rts = MX_RTS_ENABLE;
    mxuport_set_dtr(port, 1);
    }
    } else {
// Drop DTR and RTS
    rts = MX_RTS_DISABLE;
    mxuport_set_dtr(port, 0);
    }
    if (rts != MX_RTS_NO_CHANGE)
    err = mxuport_set_rts(port, rts);
    out:
    kfree(buf);
    return err;
    }
    static void mxuport_set_termios(struct tty_struct *tty,
    struct usb_serial_port *port,
    const struct ktermios *old_termios)
    {
    struct usb_serial *serial = port.serial;
    u8 *buf;
    u8 data_bits;
    u8 stop_bits;
    u8 parity;
    int baud;
    int err;
    if (old_termios &&
    !tty_termios_hw_change(&tty.termios, old_termios) &&
    tty.termios.c_iflag == old_termios.c_iflag) {
    dev_dbg(&port.dev, "%s - nothing to change\n", __func__);
    return;
    }
    buf = kmalloc(4, GFP_KERNEL);
    if (!buf)
    return;
// Set data bit of termios
    switch (C_CSIZE(tty)) {
    case CS5:
    data_bits = MX_WORDLENGTH_5;
    break;
    case CS6:
    data_bits = MX_WORDLENGTH_6;
    break;
    case CS7:
    data_bits = MX_WORDLENGTH_7;
    break;
    case CS8:
    default:
    data_bits = MX_WORDLENGTH_8;
    break;
    }
// Set parity of termios
    if (C_PARENB(tty)) {
    if (C_CMSPAR(tty)) {
    if (C_PARODD(tty))
    parity = MX_PARITY_MARK;
    else
    parity = MX_PARITY_SPACE;
    } else {
    if (C_PARODD(tty))
    parity = MX_PARITY_ODD;
    else
    parity = MX_PARITY_EVEN;
    }
    } else {
    parity = MX_PARITY_NONE;
    }
// Set stop bit of termios
    if (C_CSTOPB(tty))
    stop_bits = MX_STOP_BITS_2;
    else
    stop_bits = MX_STOP_BITS_1;
    buf[0] = data_bits;
    buf[1] = parity;
    buf[2] = stop_bits;
    buf[3] = 0;
    err = mxuport_send_ctrl_data_urb(serial, RQ_VENDOR_SET_LINE,
    0, port.port_number, buf, 4);
    if (err)
    goto out;
    err = mxuport_set_termios_flow(tty, old_termios, port, serial);
    if (err)
    goto out;
    baud = tty_get_baud_rate(tty);
    if (!baud)
    baud = 9600;
// Note: Little Endian
    put_unaligned_le32(baud, buf);
    err = mxuport_send_ctrl_data_urb(serial, RQ_VENDOR_SET_BAUD,
    0, port.port_number,
    buf, 4);
    if (err)
    goto out;
    dev_dbg(&port.dev, "baud_rate	: %d\n", baud);
    dev_dbg(&port.dev, "data_bits	: %d\n", data_bits);
    dev_dbg(&port.dev, "parity	: %d\n", parity);
    dev_dbg(&port.dev, "stop_bits	: %d\n", stop_bits);
    out:
    kfree(buf);
    }
//
// Determine how many ports this device has dynamically.  It will be
// called after the probe() callback is called, but before attach().
//
    static int mxuport_calc_num_ports(struct usb_serial *serial,
    struct usb_serial_endpoints *epds)
    {
    let mut features: c_ulong = (unsigned long)usb_get_serial_data(serial);
    int num_ports;
    int i;
    if (features & MX_PORTS_MASK) {
    num_ports = (features & MX_PORTS_MASK) + MX_PORTS_OFFSET;
    } else {
    dev_warn(&serial.interface.dev,
    "unknown device, assuming two ports\n");
    num_ports = 2;
    }
//
// Setup bulk-out endpoint multiplexing. All ports share the same
// bulk-out endpoint.
//
    BUILD_BUG_ON(ARRAY_SIZE(epds.bulk_out) < 16);
//
// The bulk-out buffers must be large enough for the four-byte header
// (and following data), but assume anything smaller than eight bytes
// is broken.
//
    if (usb_endpoint_maxp(epds.bulk_out[0]) < 8)
    return -EINVAL;
    for (i = 1; i < num_ports; ++i)
    epds.bulk_out[i] = epds.bulk_out[0];
    epds.num_bulk_out = num_ports;
    return num_ports;
    }
// Get the version of the firmware currently running.
#[no_mangle]
unsafe extern "C" fn mxuport_get_fw_version(serial: *mut usb_serial, version: *mut u32) -> c_int {
    static int mxuport_get_fw_version(struct usb_serial *serial, u32 *version)
    {
    u8 *ver_buf;
    int err;
    ver_buf = kzalloc(4, GFP_KERNEL);
    if (!ver_buf)
    return -ENOMEM;
// Get firmware version from SDRAM
    err = mxuport_recv_ctrl_urb(serial, RQ_VENDOR_GET_VERSION, 0, 0,
    ver_buf, 4);
    if (err != 4) {
    err = -EIO;
    goto out;
    }
// version = (ver_buf[0] << 16) | (ver_buf[1] << 8) | ver_buf[2];
    err = 0;
    out:
    kfree(ver_buf);
    return err;
    }
// Given a firmware blob, download it to the device.
    static int mxuport_download_fw(struct usb_serial *serial,
    const struct firmware *fw_p)
    {
    u8 *fw_buf;
    size_t txlen;
    size_t fwidx;
    int err;
    fw_buf = kmalloc(DOWN_BLOCK_SIZE, GFP_KERNEL);
    if (!fw_buf)
    return -ENOMEM;
    dev_dbg(&serial.interface.dev, "Starting firmware download...\n");
    err = mxuport_send_ctrl_urb(serial, RQ_VENDOR_START_FW_DOWN, 0, 0);
    if (err)
    goto out;
    fwidx = 0;
    do {
    txlen = min_t(size_t, (fw_p.size - fwidx), DOWN_BLOCK_SIZE);
    memcpy(fw_buf, &fw_p.data[fwidx], txlen);
    err = mxuport_send_ctrl_data_urb(serial, RQ_VENDOR_FW_DATA,
    0, 0, fw_buf, txlen);
    if (err) {
    mxuport_send_ctrl_urb(serial, RQ_VENDOR_STOP_FW_DOWN,
    0, 0);
    goto out;
    }
    fwidx += txlen;
    usleep_range(1000, 2000);
    } while (fwidx < fw_p.size);
    msleep(1000);
    err = mxuport_send_ctrl_urb(serial, RQ_VENDOR_STOP_FW_DOWN, 0, 0);
    if (err)
    goto out;
    msleep(1000);
    err = mxuport_send_ctrl_urb(serial, RQ_VENDOR_QUERY_FW_READY, 0, 0);
    out:
    kfree(fw_buf);
    return err;
    }
    static int mxuport_probe(struct usb_serial *serial,
    const struct usb_device_id *id)
    {
    let mut productid: u16 = le16_to_cpu(serial.dev.descriptor.idProduct);
    const struct firmware *fw_p = core::ptr::null_mut();
    u32 version;
    int local_ver;
    char buf[32];
    int err;
// Load our firmware
    err = mxuport_send_ctrl_urb(serial, RQ_VENDOR_QUERY_FW_CONFIG, 0, 0);
    if (err) {
    mxuport_send_ctrl_urb(serial, RQ_VENDOR_RESET_DEVICE, 0, 0);
    return err;
    }
    err = mxuport_get_fw_version(serial, &version);
    if (err < 0)
    return err;
    dev_dbg(&serial.interface.dev, "Device firmware version v%x.%x.%x\n",
    (version & 0xff0000) >> 16,
    (version & 0xff00) >> 8,
    (version & 0xff));
    snprintf(buf, sizeof(buf) - 1, "moxa/moxa-%04x.fw", productid);
    err = request_firmware(&fw_p, buf, &serial.interface.dev);
    if (err) {
    dev_warn(&serial.interface.dev, "Firmware %s not found\n",
    buf);
// Use the firmware already in the device
    err = 0;
    } else {
    if (fw_p.size <= VER_ADDR_3) {
    dev_err(&serial.interface.dev,
    "Firmware %s is too short\n", buf);
    err = -EINVAL;
    goto out;
    }
    local_ver = ((fw_p.data[VER_ADDR_1] << 16) |
    (fw_p.data[VER_ADDR_2] << 8) |
    fw_p.data[VER_ADDR_3]);
    dev_dbg(&serial.interface.dev,
    "Available firmware version v%x.%x.%x\n",
    fw_p.data[VER_ADDR_1], fw_p.data[VER_ADDR_2],
    fw_p.data[VER_ADDR_3]);
    if (local_ver > version) {
    err = mxuport_download_fw(serial, fw_p);
    if (err)
    goto out;
    err  = mxuport_get_fw_version(serial, &version);
    if (err < 0)
    goto out;
    }
    }
    dev_info(&serial.interface.dev,
    "Using device firmware version v%x.%x.%x\n",
    (version & 0xff0000) >> 16,
    (version & 0xff00) >> 8,
    (version & 0xff));
//
// Contains the features of this hardware. Store away for
// later use, eg, number of ports.
//
    usb_set_serial_data(serial, (void *)id.driver_info);
    out:
    if (fw_p)
    release_firmware(fw_p);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn mxuport_port_probe(port: *mut usb_serial_port) -> c_int {
    static int mxuport_port_probe(struct usb_serial_port *port)
    {
    struct usb_serial *serial = port.serial;
    struct mxuport_port *mxport;
    int err;
    mxport = devm_kzalloc(&port.dev, sizeof(struct mxuport_port),
    GFP_KERNEL);
    if (!mxport)
    return -ENOMEM;
    mutex_init(&mxport.mutex);
    spin_lock_init(&mxport.spinlock);
// Set the port private data
    usb_set_serial_port_data(port, mxport);
// Set FIFO (Enable)
    err = mxuport_send_ctrl_urb(serial, RQ_VENDOR_SET_FIFO_DISABLE,
    0, port.port_number);
    if (err)
    return err;
// Set transmission mode (Hi-Performance)
    err = mxuport_send_ctrl_urb(serial, RQ_VENDOR_SET_HIGH_PERFOR,
    0, port.port_number);
    if (err)
    return err;
// Set interface (RS-232)
    return mxuport_send_ctrl_urb(serial, RQ_VENDOR_SET_INTERFACE,
    MX_INT_RS232,
    port.port_number);
    }
#[no_mangle]
unsafe extern "C" fn mxuport_attach(serial: *mut usb_serial) -> c_int {
    static int mxuport_attach(struct usb_serial *serial)
    {
    struct usb_serial_port *port0 = serial.port[0];
    struct usb_serial_port *port1 = serial.port[1];
    int err;
//
// All data from the ports is received on the first bulk in
// endpoint, with a multiplex header. The second bulk in is
// used for events.
//
// Start to read from the device.
//
    err = usb_serial_generic_submit_read_urbs(port0, GFP_KERNEL);
    if (err)
    return err;
    err = usb_serial_generic_submit_read_urbs(port1, GFP_KERNEL);
    if (err) {
    usb_serial_generic_close(port0);
    return err;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mxuport_release(serial: *mut usb_serial) {
    static void mxuport_release(struct usb_serial *serial)
    {
    struct usb_serial_port *port0 = serial.port[0];
    struct usb_serial_port *port1 = serial.port[1];
    usb_serial_generic_close(port1);
    usb_serial_generic_close(port0);
    }
#[no_mangle]
unsafe extern "C" fn mxuport_open(tty: *mut tty_struct, port: *mut usb_serial_port) -> c_int {
    static int mxuport_open(struct tty_struct *tty, struct usb_serial_port *port)
    {
    struct mxuport_port *mxport = usb_get_serial_port_data(port);
    struct usb_serial *serial = port.serial;
    int err;
// Set receive host (enable)
    err = mxuport_send_ctrl_urb(serial, RQ_VENDOR_SET_RX_HOST_EN,
    1, port.port_number);
    if (err)
    return err;
    err = mxuport_send_ctrl_urb(serial, RQ_VENDOR_SET_OPEN,
    1, port.port_number);
    if (err) {
    mxuport_send_ctrl_urb(serial, RQ_VENDOR_SET_RX_HOST_EN,
    0, port.port_number);
    return err;
    }
// Initial port termios
    if (tty)
    mxuport_set_termios(tty, port, core::ptr::null_mut());
//
// TODO: use RQ_VENDOR_GET_MSR, once we know what it
// returns.
//
    mxport.msr_state = 0;
    return err;
    }
#[no_mangle]
unsafe extern "C" fn mxuport_close(port: *mut usb_serial_port) {
    static void mxuport_close(struct usb_serial_port *port)
    {
    struct usb_serial *serial = port.serial;
    mxuport_send_ctrl_urb(serial, RQ_VENDOR_SET_OPEN, 0,
    port.port_number);
    mxuport_send_ctrl_urb(serial, RQ_VENDOR_SET_RX_HOST_EN, 0,
    port.port_number);
    }
// Send a break to the port.
#[no_mangle]
unsafe extern "C" fn mxuport_break_ctl(tty: *mut tty_struct, break_state: c_int) -> c_int {
    static int mxuport_break_ctl(struct tty_struct *tty, int break_state)
    {
    struct usb_serial_port *port = tty.driver_data;
    struct usb_serial *serial = port.serial;
    int enable;
    if (break_state == -1) {
    enable = 1;
    dev_dbg(&port.dev, "%s - sending break\n", __func__);
    } else {
    enable = 0;
    dev_dbg(&port.dev, "%s - clearing break\n", __func__);
    }
    return mxuport_send_ctrl_urb(serial, RQ_VENDOR_SET_BREAK,
    enable, port.port_number);
    }
#[no_mangle]
unsafe extern "C" fn mxuport_resume(serial: *mut usb_serial) -> c_int {
    static int mxuport_resume(struct usb_serial *serial)
    {
    struct usb_serial_port *port;
    let mut c: c_int = 0;
    int i;
    int r;
    for (i = 0; i < 2; i++) {
    port = serial.port[i];
    r = usb_serial_generic_submit_read_urbs(port, GFP_NOIO);
    if (r < 0)
    c++;
    }
    for (i = 0; i < serial.num_ports; i++) {
    port = serial.port[i];
    if (!tty_port_initialized(&port.port))
    continue;
    r = usb_serial_generic_write_start(port, GFP_NOIO);
    if (r < 0)
    c++;
    }
    return c ? -EIO : 0;
    }
    static struct usb_serial_driver mxuport_device = {
    .driver = {
    .name =		"mxuport",
    },
    .description		= "MOXA UPort",
    .id_table		= mxuport_idtable,
    .num_bulk_in		= 2,
    .num_bulk_out		= 1,
    .probe			= mxuport_probe,
    .port_probe		= mxuport_port_probe,
    .attach			= mxuport_attach,
    .release		= mxuport_release,
    .calc_num_ports		= mxuport_calc_num_ports,
    .open			= mxuport_open,
    .close			= mxuport_close,
    .set_termios		= mxuport_set_termios,
    .break_ctl		= mxuport_break_ctl,
    .tx_empty		= mxuport_tx_empty,
    .tiocmiwait		= usb_serial_generic_tiocmiwait,
    .get_icount		= usb_serial_generic_get_icount,
    .throttle		= mxuport_throttle,
    .unthrottle		= mxuport_unthrottle,
    .tiocmget		= mxuport_tiocmget,
    .tiocmset		= mxuport_tiocmset,
    .dtr_rts		= mxuport_dtr_rts,
    .process_read_urb	= mxuport_process_read_urb,
    .prepare_write_buffer	= mxuport_prepare_write_buffer,
    .resume			= mxuport_resume,
    };
    static struct usb_serial_driver *const serial_drivers[] = {
    &mxuport_device, core::ptr::null_mut()
    };
    module_usb_serial_driver(serial_drivers, mxuport_idtable);
    MODULE_AUTHOR("Andrew Lunn <andrew@lunn.ch>");
    MODULE_AUTHOR("<support@moxa.com>");
    MODULE_DESCRIPTION("Moxa UPORT USB Serial driver");
    MODULE_LICENSE("GPL");
