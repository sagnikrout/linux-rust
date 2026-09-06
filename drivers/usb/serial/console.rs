//! Automatically rewritten from C to Rust
//! Source: drivers/usb/serial/console.c
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
// USB Serial Console driver
//
// Copyright (C) 2001 - 2002 Greg Kroah-Hartman (greg@kroah.com)
//
// Thanks to Randy Dunlap for the original version of this code.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbcons_info {
    pub magic: c_int,
    pub break_flag: c_int,
    pub port: *mut usb_serial_port,
}

    static struct usbcons_info usbcons_info;
    static struct console usbcons;
//
// ------------------------------------------------------------
// USB Serial console driver
//
// Much of the code here is copied from drivers/char/serial.c
// and implements a phony serial console in the same way that
// serial.c does so that in case some software queries it,
// it will get the same results.
//
// Things that are different from the way the serial port code
// does things, is that we call the lower level usb-serial
// driver code to initialize the device, and we set the initial
// console speeds based on the command line arguments.
// ------------------------------------------------------------
//
    static const struct tty_operations usb_console_fake_tty_ops = {
    };
//
// The parsing of the command line works exactly like the
// serial.c code, except that the specifier is "ttyUSB" instead
// of "ttyS".
//
#[no_mangle]
unsafe extern "C" fn usb_console_setup(co: *mut console, options: *mut c_char) -> c_int {
    static int usb_console_setup(struct console *co, char *options)
    {
    struct usbcons_info *info = &usbcons_info;
    let mut baud: c_int = 9600;
    let mut bits: c_int = 8;
    let mut parity: c_int = 'n';
    let mut doflow: c_int = 0;
    let mut cflag: c_int = CREAD | HUPCL | CLOCAL;
    char *s;
    struct usb_serial *serial;
    struct usb_serial_port *port;
    int retval;
    struct tty_struct *tty = core::ptr::null_mut();
    struct ktermios dummy;
    if (options) {
    baud = simple_strtoul(options, core::ptr::null_mut(), 10);
    s = options;
    while (*s >= '0' && *s <= '9')
    s++;
    if (*s)
    parity = *s++;
    if (*s)
    bits   = *s++ - '0';
    if (*s)
    doflow = (*s++ == 'r');
    }
// Sane default
    if (baud == 0)
    baud = 9600;
    switch (bits) {
    case 7:
    cflag |= CS7;
    break;
    default:
    case 8:
    cflag |= CS8;
    break;
    }
    switch (parity) {
    case 'o': case 'O':
    cflag |= PARODD;
    break;
    case 'e': case 'E':
    cflag |= PARENB;
    break;
    }
    if (doflow)
    cflag |= CRTSCTS;
//
// no need to check the index here: if the index is wrong, console
// code won't call us
//
    port = usb_serial_port_get_by_minor(co.index);
    if (port == core::ptr::null_mut()) {
// no device is connected yet, sorry :(
    pr_err("No USB device connected to ttyUSB%i\n", co.index);
    return -ENODEV;
    }
    serial = port.serial;
    retval = usb_autopm_get_interface(serial.interface);
    if (retval)
    goto error_get_interface;
    tty_port_tty_set(&port.port, core::ptr::null_mut());
    info.port = port;
    ++port.port.count;
    if (!tty_port_initialized(&port.port)) {
    if (serial.type.set_termios) {
//
// allocate a fake tty so the driver can initialize
// the termios structure, then later call set_termios to
// configure according to command line arguments
//
    tty = kzalloc_obj(*tty);
    if (!tty) {
    retval = -ENOMEM;
    goto reset_open_count;
    }
    kref_init(&tty.kref);
    tty.driver = usb_serial_tty_driver;
    tty.index = co.index;
    init_ldsem(&tty.ldisc_sem);
    spin_lock_init(&tty.files_lock);
    INIT_LIST_HEAD(&tty.tty_files);
    kref_get(&tty.driver.kref);
    __module_get(tty.driver.owner);
    tty.ops = &usb_console_fake_tty_ops;
    tty_init_termios(tty);
    tty_port_tty_set(&port.port, tty);
    }
// only call the device specific open if this
// is the first time the port is opened
    retval = serial.type.open(core::ptr::null_mut(), port);
    if (retval) {
    dev_err(&port.dev, "could not open USB console port\n");
    goto fail;
    }
    if (serial.type.set_termios) {
    tty.termios.c_cflag = cflag;
    tty_termios_encode_baud_rate(&tty.termios, baud, baud);
    memset(&dummy, 0, sizeof(struct ktermios));
    serial.type.set_termios(tty, port, &dummy);
    tty_port_tty_set(&port.port, core::ptr::null_mut());
    tty_save_termios(tty);
    tty_kref_put(tty);
    }
    tty_port_set_initialized(&port.port, true);
    }
// Now that any required fake tty operations are completed restore
// the tty port count
    --port.port.count;
// The console is special in terms of closing the device so
// indicate this port is now acting as a system console.
    port.port.console = 1;
    mutex_unlock(&serial.disc_mutex);
    return retval;
    fail:
    tty_port_tty_set(&port.port, core::ptr::null_mut());
    tty_kref_put(tty);
    reset_open_count:
    port.port.count = 0;
    info.port = core::ptr::null_mut();
    usb_autopm_put_interface(serial.interface);
    error_get_interface:
    mutex_unlock(&serial.disc_mutex);
    usb_serial_put(serial);
    return retval;
    }
    static void usb_console_write(struct console *co,
    const char *buf, unsigned count)
    {
    static struct usbcons_info *info = &usbcons_info;
    struct usb_serial_port *port = info.port;
    struct usb_serial *serial;
    let mut retval: c_int = -ENODEV;
    if (!port || port.serial.dev.state == USB_STATE_NOTATTACHED)
    return;
    serial = port.serial;
    if (count == 0)
    return;
    dev_dbg(&port.dev, "%s - %d byte(s)\n", __func__, count);
    if (!port.port.console) {
    dev_dbg(&port.dev, "%s - port not opened\n", __func__);
    return;
    }
    while (count) {
    unsigned int i;
    unsigned int lf;
// search for LF so we can insert CR if necessary
    for (i = 0, lf = 0 ; i < count ; i++) {
    if (*(buf + i) == 10) {
    lf = 1;
    i++;
    break;
    }
    }
// pass on to the driver specific version of this function if
    it is available */
    retval = serial.type.write(core::ptr::null_mut(), port, buf, i);
    dev_dbg(&port.dev, "%s - write: %d\n", __func__, retval);
    if (lf) {
// append CR after LF
    let mut cr: c_uchar = 13;
    retval = serial.type.write(core::ptr::null_mut(), port, &cr, 1);
    dev_dbg(&port.dev, "%s - write cr: %d\n",
    __func__, retval);
    }
    buf += i;
    count -= i;
    }
    }
    static struct tty_driver *usb_console_device(struct console *co, int *index)
    {
    struct tty_driver **p = (struct tty_driver **)co.data;
    if (!*p)
    return core::ptr::null_mut();
// index = co->index;
    return *p;
    }
    static struct console usbcons = {
    .name =		"ttyUSB",
    .write =	usb_console_write,
    .device =	usb_console_device,
    .setup =	usb_console_setup,
    .flags =	CON_PRINTBUFFER,
    .index =	-1,
    .data = 	&usb_serial_tty_driver,
    };
#[no_mangle]
pub unsafe extern "C" fn usb_serial_console_disconnect(serial: *mut usb_serial) {
    void usb_serial_console_disconnect(struct usb_serial *serial)
    {
    if (serial.port[0] && serial.port[0] == usbcons_info.port) {
    usb_serial_console_exit();
    usb_serial_put(serial);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn usb_serial_console_init(minor: c_int) {
    void usb_serial_console_init(int minor)
    {
    if (minor == 0) {
//
// Call register_console() if this is the first device plugged
// in.  If we call it earlier, then the callback to
// console_setup() will fail, as there is not a device seen by
// the USB subsystem yet.
//
// Register console.
// NOTES:
// console_setup() is called (back) immediately (from
// register_console). console_write() is called immediately
// from register_console iff CON_PRINTBUFFER is set in flags.
//
    pr_debug("registering the USB serial console.\n");
    register_console(&usbcons);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn usb_serial_console_exit() {
    void usb_serial_console_exit(void)
    {
    if (usbcons_info.port) {
    unregister_console(&usbcons);
    usbcons_info.port.port.console = 0;
    usbcons_info.port = core::ptr::null_mut();
    }
    }
