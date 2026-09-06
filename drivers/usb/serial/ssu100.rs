//! Automatically rewritten from C to Rust
//! Source: drivers/usb/serial/ssu100.c
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
// usb-serial driver for Quatech SSU-100
//
// based on ftdi_sio.c and the original serqt_usb.c from Quatech
//

pub const QT_OPEN_CLOSE_CHANNEL: c_uint = 0xca;
pub const QT_SET_GET_DEVICE: c_uint = 0xc2;
pub const QT_SET_GET_REGISTER: c_uint = 0xc0;
pub const QT_GET_SET_PREBUF_TRIG_LVL: c_uint = 0xcc;
pub const QT_SET_ATF: c_uint = 0xcd;
pub const QT_GET_SET_UART: c_uint = 0xc1;
pub const QT_TRANSFER_IN: c_uint = 0xc0;
pub const QT_HW_FLOW_CONTROL_MASK: c_uint = 0xc5;
pub const QT_SW_FLOW_CONTROL_MASK: c_uint = 0xc6;
pub const SERIAL_MSR_MASK: c_uint = 0xf0;

pub const MAX_BAUD_RATE: c_int = 460800;
pub const ATC_DISABLED: c_uint = 0x00;
pub const DUPMODE_BITS: c_uint = 0xc0;
pub const RR_BITS: c_uint = 0x03;
pub const LOOPMODE_BITS: c_uint = 0x41;
pub const RS232_MODE: c_uint = 0x00;
pub const RTSCTS_TO_CONNECTOR: c_uint = 0x40;
pub const CLKS_X4: c_uint = 0x02;
pub const FULLPWRBIT: c_uint = 0x00000080;
pub const NEXT_BOARD_POWER_BIT: c_uint = 0x00000004;

pub const USB_VENDOR_ID_QUATECH: c_uint = 0x061d	/* Quatech VID */;
pub const QUATECH_SSU100: c_uint = 0xC020	/* SSU100 */;
    static const struct usb_device_id id_table[] = {
    {USB_DEVICE(USB_VENDOR_ID_QUATECH, QUATECH_SSU100)},
    {}			/* Terminating entry */
    };
    MODULE_DEVICE_TABLE(usb, id_table);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssu100_port_private {
    pub status_lock: spinlock_t,
    pub shadowLSR: u8,
    pub shadowMSR: u8,
}

    static inline int ssu100_control_msg(struct usb_device *dev,
    u8 request, u16 data, u16 index)
    {
    return usb_control_msg(dev, usb_sndctrlpipe(dev, 0),
    request, 0x40, data, index,
    core::ptr::null_mut(), 0, 300);
    }
#[no_mangle]
pub unsafe extern "C" fn ssu100_setdevice(dev: *mut usb_device, data: *mut u8) -> c_int {
    static inline int ssu100_setdevice(struct usb_device *dev, u8 *data)
    {
    let mut x: u16 = ((u16)(data[1] << 8) | (u16)(data[0]));
    return ssu100_control_msg(dev, QT_SET_GET_DEVICE, x, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn ssu100_getdevice(dev: *mut usb_device, data: *mut u8) -> c_int {
    static inline int ssu100_getdevice(struct usb_device *dev, u8 *data)
    {
    int ret;
    ret = usb_control_msg(dev, usb_rcvctrlpipe(dev, 0),
    QT_SET_GET_DEVICE, 0xc0, 0, 0,
    data, 3, 300);
    if (ret < 3) {
    if (ret >= 0)
    ret = -EIO;
    }
    return ret;
    }
    static inline int ssu100_getregister(struct usb_device *dev,
    unsigned short uart,
    unsigned short reg,
    u8 *data)
    {
    int ret;
    ret = usb_control_msg(dev, usb_rcvctrlpipe(dev, 0),
    QT_SET_GET_REGISTER, 0xc0, reg,
    uart, data, sizeof(*data), 300);
    if (ret < (int)sizeof(*data)) {
    if (ret >= 0)
    ret = -EIO;
    }
    return ret;
    }
    static inline int ssu100_setregister(struct usb_device *dev,
    unsigned short uart,
    unsigned short reg,
    u16 data)
    {
    let mut value: u16 = (data << 8) | reg;
    return usb_control_msg(dev, usb_sndctrlpipe(dev, 0),
    QT_SET_GET_REGISTER, 0x40, value, uart,
    core::ptr::null_mut(), 0, 300);
    }

// these do not deal with device that have more than 1 port
    static inline int update_mctrl(struct usb_device *dev, unsigned int set,
    unsigned int clear)
    {
    unsigned urb_value;
    int result;
    if (((set | clear) & (TIOCM_DTR | TIOCM_RTS)) == 0) {
    dev_dbg(&dev.dev, "%s - DTR|RTS not being set|cleared\n", __func__);
    return 0;	/* no change */
    }
    clear &= ~set;	/* 'set' takes precedence over 'clear' */
    urb_value = 0;
    if (set & TIOCM_DTR)
    urb_value |= UART_MCR_DTR;
    if (set & TIOCM_RTS)
    urb_value |= UART_MCR_RTS;
    result = ssu100_setregister(dev, 0, UART_MCR, urb_value);
    if (result < 0)
    dev_dbg(&dev.dev, "%s Error from MODEM_CTRL urb\n", __func__);
    return result;
    }
#[no_mangle]
unsafe extern "C" fn ssu100_initdevice(dev: *mut usb_device) -> c_int {
    static int ssu100_initdevice(struct usb_device *dev)
    {
    u8 *data;
    let mut result: c_int = 0;
    data = kzalloc(3, GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    result = ssu100_getdevice(dev, data);
    if (result < 0) {
    dev_dbg(&dev.dev, "%s - get_device failed %i\n", __func__, result);
    goto out;
    }
    data[1] &= ~FULLPWRBIT;
    result = ssu100_setdevice(dev, data);
    if (result < 0) {
    dev_dbg(&dev.dev, "%s - setdevice failed %i\n", __func__, result);
    goto out;
    }
    result = ssu100_control_msg(dev, QT_GET_SET_PREBUF_TRIG_LVL, 128, 0);
    if (result < 0) {
    dev_dbg(&dev.dev, "%s - set prebuffer level failed %i\n", __func__, result);
    goto out;
    }
    result = ssu100_control_msg(dev, QT_SET_ATF, ATC_DISABLED, 0);
    if (result < 0) {
    dev_dbg(&dev.dev, "%s - set ATFprebuffer level failed %i\n", __func__, result);
    goto out;
    }
    result = ssu100_getdevice(dev, data);
    if (result < 0) {
    dev_dbg(&dev.dev, "%s - get_device failed %i\n", __func__, result);
    goto out;
    }
    data[0] &= ~(RR_BITS | DUPMODE_BITS);
    data[0] |= CLKS_X4;
    data[1] &= ~(LOOPMODE_BITS);
    data[1] |= RS232_MODE;
    result = ssu100_setdevice(dev, data);
    if (result < 0) {
    dev_dbg(&dev.dev, "%s - setdevice failed %i\n", __func__, result);
    goto out;
    }
    out:	kfree(data);
    return result;
    }
    static void ssu100_set_termios(struct tty_struct *tty,
    struct usb_serial_port *port,
    const struct ktermios *old_termios)
    {
    struct usb_device *dev = port.serial.dev;
    struct ktermios *termios = &tty.termios;
    u16 baud, divisor, remainder;
    let mut cflag: c_uint = termios.c_cflag;
    u16 urb_value = 0; /* will hold the new flags */
    int result;
    if (cflag & PARENB) {
    if (cflag & PARODD)
    urb_value |= UART_LCR_PARITY;
    else
    urb_value |= SERIAL_EVEN_PARITY;
    }
    urb_value |= UART_LCR_WLEN(tty_get_char_size(cflag));
    baud = tty_get_baud_rate(tty);
    if (!baud)
    baud = 9600;
    dev_dbg(&port.dev, "%s - got baud = %d\n", __func__, baud);
    divisor = MAX_BAUD_RATE / baud;
    remainder = MAX_BAUD_RATE % baud;
    if (((remainder * 2) >= baud) && (baud != 110))
    divisor++;
    urb_value = urb_value << 8;
    result = ssu100_control_msg(dev, QT_GET_SET_UART, divisor, urb_value);
    if (result < 0)
    dev_dbg(&port.dev, "%s - set uart failed\n", __func__);
    if (cflag & CRTSCTS)
    result = ssu100_control_msg(dev, QT_HW_FLOW_CONTROL_MASK,
    SERIAL_CRTSCTS, 0);
    else
    result = ssu100_control_msg(dev, QT_HW_FLOW_CONTROL_MASK,
    0, 0);
    if (result < 0)
    dev_dbg(&port.dev, "%s - set HW flow control failed\n", __func__);
    if (I_IXOFF(tty) || I_IXON(tty)) {
    let mut x: u16 = ((u16)(START_CHAR(tty) << 8) | (u16)(STOP_CHAR(tty)));
    result = ssu100_control_msg(dev, QT_SW_FLOW_CONTROL_MASK,
    x, 0);
    } else
    result = ssu100_control_msg(dev, QT_SW_FLOW_CONTROL_MASK,
    0, 0);
    if (result < 0)
    dev_dbg(&port.dev, "%s - set SW flow control failed\n", __func__);
    }
#[no_mangle]
unsafe extern "C" fn ssu100_open(tty: *mut tty_struct, port: *mut usb_serial_port) -> c_int {
    static int ssu100_open(struct tty_struct *tty, struct usb_serial_port *port)
    {
    struct usb_device *dev = port.serial.dev;
    struct ssu100_port_private *priv = usb_get_serial_port_data(port);
    u8 *data;
    int result;
    unsigned long flags;
    data = kzalloc(2, GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    result = usb_control_msg(dev, usb_rcvctrlpipe(dev, 0),
    QT_OPEN_CLOSE_CHANNEL,
    QT_TRANSFER_IN, 0x01,
    0, data, 2, 300);
    if (result < 2) {
    dev_dbg(&port.dev, "%s - open failed %i\n", __func__, result);
    if (result >= 0)
    result = -EIO;
    kfree(data);
    return result;
    }
    spin_lock_irqsave(&priv.status_lock, flags);
    priv.shadowLSR = data[0];
    priv.shadowMSR = data[1];
    spin_unlock_irqrestore(&priv.status_lock, flags);
    kfree(data);
// set to 9600
    result = ssu100_control_msg(dev, QT_GET_SET_UART, 0x30, 0x0300);
    if (result < 0)
    dev_dbg(&port.dev, "%s - set uart failed\n", __func__);
    if (tty)
    ssu100_set_termios(tty, port, &tty.termios);
    return usb_serial_generic_open(tty, port);
    }
#[no_mangle]
unsafe extern "C" fn ssu100_attach(serial: *mut usb_serial) -> c_int {
    static int ssu100_attach(struct usb_serial *serial)
    {
    return ssu100_initdevice(serial.dev);
    }
#[no_mangle]
unsafe extern "C" fn ssu100_port_probe(port: *mut usb_serial_port) -> c_int {
    static int ssu100_port_probe(struct usb_serial_port *port)
    {
    struct ssu100_port_private *priv;
    priv = kzalloc_obj(*priv);
    if (!priv)
    return -ENOMEM;
    spin_lock_init(&priv.status_lock);
    usb_set_serial_port_data(port, priv);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ssu100_port_remove(port: *mut usb_serial_port) {
    static void ssu100_port_remove(struct usb_serial_port *port)
    {
    struct ssu100_port_private *priv;
    priv = usb_get_serial_port_data(port);
    kfree(priv);
    }
#[no_mangle]
unsafe extern "C" fn ssu100_tiocmget(tty: *mut tty_struct) -> c_int {
    static int ssu100_tiocmget(struct tty_struct *tty)
    {
    struct usb_serial_port *port = tty.driver_data;
    struct usb_device *dev = port.serial.dev;
    u8 *d;
    int r;
    d = kzalloc(2, GFP_KERNEL);
    if (!d)
    return -ENOMEM;
    r = ssu100_getregister(dev, 0, UART_MCR, d);
    if (r < 0)
    goto mget_out;
    r = ssu100_getregister(dev, 0, UART_MSR, d+1);
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
    static int ssu100_tiocmset(struct tty_struct *tty,
    unsigned int set, unsigned int clear)
    {
    struct usb_serial_port *port = tty.driver_data;
    struct usb_device *dev = port.serial.dev;
    return update_mctrl(dev, set, clear);
    }
#[no_mangle]
unsafe extern "C" fn ssu100_dtr_rts(port: *mut usb_serial_port, on: c_int) {
    static void ssu100_dtr_rts(struct usb_serial_port *port, int on)
    {
    struct usb_device *dev = port.serial.dev;
// Disable flow control
    if (!on) {
    if (ssu100_setregister(dev, 0, UART_MCR, 0) < 0)
    dev_err(&port.dev, "error from flowcontrol urb\n");
    }
// drop RTS and DTR
    if (on)
    set_mctrl(dev, TIOCM_DTR | TIOCM_RTS);
    else
    clear_mctrl(dev, TIOCM_DTR | TIOCM_RTS);
    }
#[no_mangle]
unsafe extern "C" fn ssu100_update_msr(port: *mut usb_serial_port, msr: u8) {
    static void ssu100_update_msr(struct usb_serial_port *port, u8 msr)
    {
    struct ssu100_port_private *priv = usb_get_serial_port_data(port);
    unsigned long flags;
    spin_lock_irqsave(&priv.status_lock, flags);
    priv.shadowMSR = msr;
    spin_unlock_irqrestore(&priv.status_lock, flags);
    if (msr & UART_MSR_ANY_DELTA) {
// update input line counters
    if (msr & UART_MSR_DCTS)
    port.icount.cts++;
    if (msr & UART_MSR_DDSR)
    port.icount.dsr++;
    if (msr & UART_MSR_DDCD)
    port.icount.dcd++;
    if (msr & UART_MSR_TERI)
    port.icount.rng++;
    wake_up_interruptible(&port.port.delta_msr_wait);
    }
    }
    static void ssu100_update_lsr(struct usb_serial_port *port, u8 lsr,
    char *tty_flag)
    {
    struct ssu100_port_private *priv = usb_get_serial_port_data(port);
    unsigned long flags;
    spin_lock_irqsave(&priv.status_lock, flags);
    priv.shadowLSR = lsr;
    spin_unlock_irqrestore(&priv.status_lock, flags);
// tty_flag = TTY_NORMAL;
    if (lsr & UART_LSR_BRK_ERROR_BITS) {
// we always want to update icount, but we only want to
// update tty_flag for one case
    if (lsr & UART_LSR_BI) {
    port.icount.brk++;
// tty_flag = TTY_BREAK;
    usb_serial_handle_break(port);
    }
    if (lsr & UART_LSR_PE) {
    port.icount.parity++;
    if (*tty_flag == TTY_NORMAL)
// tty_flag = TTY_PARITY;
    }
    if (lsr & UART_LSR_FE) {
    port.icount.frame++;
    if (*tty_flag == TTY_NORMAL)
// tty_flag = TTY_FRAME;
    }
    if (lsr & UART_LSR_OE) {
    port.icount.overrun++;
    tty_insert_flip_char(&port.port, 0, TTY_OVERRUN);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn ssu100_process_read_urb(urb: *mut urb) {
    static void ssu100_process_read_urb(struct urb *urb)
    {
    struct usb_serial_port *port = urb.context;
    char *packet = urb.transfer_buffer;
    let mut flag: c_char = TTY_NORMAL;
    let mut len: u32 = urb.actual_length;
    int i;
    char *ch;
    if ((len >= 4) &&
    (packet[0] == 0x1b) && (packet[1] == 0x1b) &&
    ((packet[2] == 0x00) || (packet[2] == 0x01))) {
    if (packet[2] == 0x00)
    ssu100_update_lsr(port, packet[3], &flag);
    if (packet[2] == 0x01)
    ssu100_update_msr(port, packet[3]);
    len -= 4;
    ch = packet + 4;
    } else
    ch = packet;
    if (!len)
    return;	/* status only */
    if (port.sysrq) {
    for (i = 0; i < len; i++, ch++) {
    if (!usb_serial_handle_sysrq_char(port, *ch))
    tty_insert_flip_char(&port.port, *ch, flag);
    }
    } else {
    tty_insert_flip_string_fixed_flag(&port.port, ch, flag, len);
    }
    tty_flip_buffer_push(&port.port);
    }
    static struct usb_serial_driver ssu100_device = {
    .driver = {
    .name = "ssu100",
    },
    .description	     = DRIVER_DESC,
    .id_table	     = id_table,
    .num_ports	     = 1,
    .open		     = ssu100_open,
    .attach              = ssu100_attach,
    .port_probe          = ssu100_port_probe,
    .port_remove         = ssu100_port_remove,
    .dtr_rts             = ssu100_dtr_rts,
    .process_read_urb    = ssu100_process_read_urb,
    .tiocmget            = ssu100_tiocmget,
    .tiocmset            = ssu100_tiocmset,
    .tiocmiwait          = usb_serial_generic_tiocmiwait,
    .get_icount	     = usb_serial_generic_get_icount,
    .set_termios         = ssu100_set_termios,
    };
    static struct usb_serial_driver * const serial_drivers[] = {
    &ssu100_device, core::ptr::null_mut()
    };
    module_usb_serial_driver(serial_drivers, id_table);
    MODULE_DESCRIPTION(DRIVER_DESC);
    MODULE_LICENSE("GPL v2");
