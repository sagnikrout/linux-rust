//! Automatically rewritten from C to Rust
//! Source: drivers/gpib/lpvo_usb_gpib/lpvo_usb_gpib.c
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
// This code has been developed at the Department of Physics (University
// of Florence, Italy) to support in linux-gpib the open usb-gpib adapter
// implemented at the University of Ljubljana (lpvo.fe.uni-lj.si/gpib)
//
// copyright		 : (C) 2011 Marcello Carla'
//

// base module includes

    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("GPIB driver for LPVO usb devices");
//
// Table of devices that work with this driver.
//
// Currently, only one device is known to be used in the lpvo_usb_gpib
// adapter (FTDI 0403:6001) but as this device id is already handled by the
// ftdi_sio USB serial driver the LPVO driver must not bind to it by default.
//
// If your adapter uses a different chip, insert a line
// in the following table with proper <Vendor-id>, <Product-id>.
//
// To have your chip automatically handled by the driver,
// update files "/usr/local/etc/modprobe.d/lpvo_usb_gpib.conf"
// and /usr/local/etc/udev/rules.d/99-lpvo_usb_gpib.rules.
//
    static const struct usb_device_id lpvo_table[] = {
    { }					   /* Terminating entry */
    };
    MODULE_DEVICE_TABLE(usb, lpvo_table);
//
// ***  Diagnostics and Debug
// To enable the diagnostic and debug messages either compile with DEBUG set
// or control via the dynamic debug mechanisms.
// The module parameter "debug" controls the sending of debug messages to
// syslog. By default it is set to 0
// debug = 0: only attach/detach messages are sent
// 1: every action is logged
// 2: extended logging; each single exchanged byte is documented
// (about twice the log volume of [1])
// To switch debug level:
// At module loading:  modprobe lpvo_usb_gpib debug={0,1,2}
// On the fly: echo {0,1,2} > /sys/module/lpvo_usb_gpib/parameters/debug
//
    static int debug;
    module_param(debug, int, 0644);

    do { if (debug >= (level))					\
    dev_dbg(board.gpib_dev, format, ## __VA_ARGS__); } \
    while (0)

// standard and extended command sets of the usb-gpib adapter

// incomplete commands

// command sequences

// special characters used by the adapter

pub const IB_BUS_REN: c_uint = 0x01;
pub const IB_BUS_IFC: c_uint = 0x02;
pub const IB_BUS_NDAC: c_uint = 0x04;
pub const IB_BUS_NRFD: c_uint = 0x08;
pub const IB_BUS_DAV: c_uint = 0x10;
pub const IB_BUS_EOI: c_uint = 0x20;
pub const IB_BUS_ATN: c_uint = 0x40;
pub const IB_BUS_SRQ: c_uint = 0x80;
pub const INBUF_SIZE: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct char_buf {
    pub inbuf: *mut c_char,
    pub last: c_int,
    pub nchar: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_gpib_priv {
    pub /: *mut *mut u8 eos; / eos character,
    pub /: *mut *mut short eos_flags; / eos mode,
    pub /: *mut *mut int timeout; / current value for timeout,
    pub /: *mut *mut *mut void dev; / the usb device private data structure,
}

#[no_mangle]
unsafe extern "C" fn show_status(board: *mut gpib_board) {
    static void show_status(struct gpib_board *board)
    {
    DIA_LOG(2, "# - buffer_length %d\n", board.buffer_length);
    DIA_LOG(2, "# - status %lx\n", board.status);
    DIA_LOG(2, "# - use_count %d\n", board.use_count);
    DIA_LOG(2, "# - pad %x\n", board.pad);
    DIA_LOG(2, "# - sad %x\n", board.sad);
    DIA_LOG(2, "# - timeout %d\n", board.usec_timeout);
    DIA_LOG(2, "# - ppc %d\n", board.parallel_poll_configuration);
    DIA_LOG(2, "# - t1delay %d\n", board.t1_nano_sec);
    DIA_LOG(2, "# - online %d\n", board.online);
    DIA_LOG(2, "# - autopoll %d\n", board.autospollers);
    DIA_LOG(2, "# - autopoll task %p\n", board.autospoll_task);
    DIA_LOG(2, "# - minor %d\n", board.minor);
    DIA_LOG(2, "# - master %d\n", board.master);
    DIA_LOG(2, "# - list %d\n", board.ist);
    }
//
// GLOBAL VARIABLES: required for
// pairing among gpib minor and usb minor.
// MAX_DEV is the max number of usb-gpib adapters; free
// to change as you like, but no more than 32
//
pub const MAX_DEV: c_int = 8;
    static struct usb_interface *lpvo_usb_interfaces[MAX_DEV];   /* registered interfaces */
    static int usb_minors[MAX_DEV];			   /* usb minors */
    static int assigned_usb_minors;		   /* mask of filled slots */
    static struct mutex minors_lock;     /* operations on usb_minors are to be protected */
    struct lpvo;
    static ssize_t lpvo_do_write(struct lpvo *, const char *, size_t);
    static ssize_t lpvo_do_read(struct lpvo *, char *, size_t);
    static int lpvo_do_open(struct gpib_board *, int);
    static int lpvo_do_release(struct gpib_board *);
//
// usec_diff : take difference in MICROsec between two 'timespec'
// (unix time in sec and NANOsec)
//
#[no_mangle]
pub unsafe extern "C" fn usec_diff(a: *mut timespec64, b: *mut timespec64) -> c_int {
    static inline int usec_diff(struct timespec64 *a, struct timespec64 *b)
    {
    return ((a.tv_sec - b.tv_sec) * 1000000 +
    (a.tv_nsec - b.tv_nsec) / 1000);
    }
//
// ***  these routines are specific to the usb-gpib adapter
//
// write_loop() - Send a byte sequence to the adapter
//
// @dev:      the private device structure
// @msg:      the byte sequence.
// @leng:     the byte sequence length.
//
#[no_mangle]
unsafe extern "C" fn write_loop(dev: *mut c_void, msg: *mut c_char, leng: c_int) -> c_int {
    static int write_loop(void *dev, char *msg, int leng)
    {
    return lpvo_do_write(dev, msg, leng);
    }
//
// send_command() - Send a byte sequence and return a single byte reply.
//
// @board:    the gpib_board_struct data area for this gpib interface
// @msg:      the byte sequence.
// @leng:     the byte sequence length; can be given as zero and is
// computed automatically, but if 'msg' contains a zero byte,
// it has to be given explicitly.
//
#[no_mangle]
unsafe extern "C" fn send_command(board: *mut gpib_board, msg: *mut c_char, leng: c_int) -> c_int {
    static int send_command(struct gpib_board *board, char *msg, int leng)
    {
    char buffer[64];
    int nchar;
    int retval;
    struct timespec64 before, after;
    ktime_get_real_ts64 (&before);
    if (!leng)
    leng = strlen(msg);
    retval = write_loop(GPIB_DEV, msg, leng);
    if (retval < 0)
    return retval;
    nchar = lpvo_do_read(GPIB_DEV, buffer, 64);
    if (nchar < 0) {
    dev_err(board.gpib_dev, " return from read: %d\n", nchar);
    return nchar;
    } else if (nchar != 1) {
    dev_err(board.gpib_dev, " Irregular reply to command: %s\n", msg);
    return -EIO;
    }
    ktime_get_real_ts64 (&after);
    DIA_LOG(1, "Sent %d - done %d us.\n", leng, usec_diff(&after, &before));
    return buffer[0] & 0xff;
    }
//
// set_control_line() - Set the value of a single gpib control line
//
// @board:    the gpib_board_struct data area for this gpib interface
// @line:     line mask
// @value:    line new value (0/1)
//
#[no_mangle]
unsafe extern "C" fn set_control_line(board: *mut gpib_board, line: c_int, value: c_int) -> c_int {
    static int set_control_line(struct gpib_board *board, int line, int value)
    {
    char msg[] = USB_GPIB_SET_LINES;
    int retval;
    let mut leng: c_int = strlen(msg);
    DIA_LOG(1, "setting line %x to %x\n", line, value);
    retval = send_command(board, USB_GPIB_READ_LINES, 0);
    DIA_LOG(1, "old line values: %x\n", retval);
    if (retval == -EIO)
    return retval;
    msg[leng - 2] = value ? (retval & ~line) : retval | line;
    retval = send_command(board, msg, 0);
    DIA_LOG(1, "operation result: %x\n", retval);
    return retval;
    }
//
// one_char() - read one single byte from input buffer
//
// @board:	the gpib_board_struct data area for this gpib interface
// @char_buf:	the routine private data structure
//
#[no_mangle]
unsafe extern "C" fn one_char(board: *mut gpib_board, b: *mut char_buf) -> c_int {
    static int one_char(struct gpib_board *board, struct char_buf *b)
    {
    struct timespec64 before, after;
    if (b.nchar) {
    DIA_LOG(2, ". %x\n", b.inbuf[b.last - b.nchar]);
    return b.inbuf[b.last - b.nchar--];
    }
    ktime_get_real_ts64 (&before);
    b.nchar = lpvo_do_read(GPIB_DEV, b.inbuf, INBUF_SIZE);
    b.last = b.nchar;
    ktime_get_real_ts64 (&after);
    DIA_LOG(2, "read %d bytes in %d usec\n",
    b.nchar, usec_diff(&after, &before));
    if (b.nchar > 0) {
    DIA_LOG(2, "-. %x\n", b.inbuf[b.last - b.nchar]);
    return b.inbuf[b.last - b.nchar--];
    }
    return -EIO;
    }
//
// set_timeout() - set single byte / total timeouts on the adapter
//
// @board:    the gpib_board_struct data area for this gpib interface
//
// For sake of speed, the operation is performed only if it
// modifies the current (saved) value. Minimum allowed timeout
// is 30 ms (T30ms -> 8); timeout disable (TNONE -> 0) currently
// not supported.
//
#[no_mangle]
unsafe extern "C" fn set_timeout(board: *mut gpib_board) {
    static void set_timeout(struct gpib_board *board)
    {
    int n, val;
    char command[sizeof(USB_GPIB_TTMO) + 6];
    struct usb_gpib_priv *data = board.private_data;
    if (data.timeout == board.usec_timeout)
    return;
    n = (board.usec_timeout + 32767) / 32768;
    if (n < 2)
    n = 2;
    DIA_LOG(1, "Set timeout to %d us . %d\n", board.usec_timeout, n);
    sprintf(command, "%s%d\n", USB_GPIB_BTMO, n > 255 ? 255 : n);
    val = send_command(board, command, 0);
    if (val == ACK) {
    if (n > 65535)
    n = 65535;
    sprintf(command, "%s%d\n", USB_GPIB_TTMO, n);
    val = send_command(board, command, 0);
    }
    if (val != ACK)
    dev_err(board.gpib_dev, "error in timeout set: <%s>\n", command);
    else
    data.timeout = board.usec_timeout;
    }
//
// now the standard interface functions - attach and detach
//
// usb_gpib_attach() - activate the usb-gpib converter board
//
// @board:    the gpib_board_struct data area for this gpib interface
// @config:   firmware data, if any (from gpib_config -I <file>)
//
// The channel name is ttyUSBn, with n=0 by default. Other values for n
// passed with gpib_config -b <n>.
//
// In this routine I trust that when an error code is returned
// detach() will be called. Always.
//
#[no_mangle]
unsafe extern "C" fn usb_gpib_attach(board: *mut gpib_board, config: *const gpib_board_config) -> c_int {
    static int usb_gpib_attach(struct gpib_board *board, const struct gpib_board_config *config)
    {
    int retval, j;
    let mut base: u32 = config.ibbase;
    char *device_path;
    int match;
    struct usb_device *udev;
    DIA_LOG(0, "Board %p -t %s -m %d -a %p -u %d -l %d -b %d\n",
    board, board.interface.name, board.minor, config.device_path,
    config.pci_bus, config.pci_slot, base);
    board.private_data = core::ptr::null_mut();  /* to be sure - we can detach before setting */
// identify device to be attached
    mutex_lock(&minors_lock);
    if (config.device_path) {
// if config->device_path given, try that first
    for (j = 0 ; j < MAX_DEV ; j++) {
    if ((assigned_usb_minors & 1 << j) == 0)
    continue;
    udev = interface_to_usbdev(lpvo_usb_interfaces[j]);
    device_path = kobject_get_path(&udev.dev.kobj, GFP_KERNEL);
    match = gpib_match_device_path(&lpvo_usb_interfaces[j].dev,
    config.device_path);
    DIA_LOG(1, "dev. %d: minor %d  path: %s -. %d\n", j,
    lpvo_usb_interfaces[j].minor, device_path, match);
    kfree(device_path);
    if (match)
    break;
    }
    } else if (config.pci_bus != -1 && config.pci_slot != -1) {
// second: look for bus and slot
    for (j = 0 ; j < MAX_DEV ; j++) {
    if ((assigned_usb_minors & 1 << j) == 0)
    continue;
    udev = interface_to_usbdev(lpvo_usb_interfaces[j]);
    DIA_LOG(1, "dev. %d: bus %d . %d  dev: %d . %d\n", j,
    udev.bus.busnum, config.pci_bus, udev.devnum, config.pci_slot);
    if (config.pci_bus == udev.bus.busnum &&
    config.pci_slot == udev.devnum)
    break;
    }
    } else {		/* last chance: usb_minor, given as ibbase */
    for (j = 0 ; j < MAX_DEV ; j++) {
    if (usb_minors[j] == base && assigned_usb_minors & 1 << j)
    break;
    }
    }
    mutex_unlock(&minors_lock);
    if (j == MAX_DEV) {
    dev_err(board.gpib_dev, "Requested device is not registered.\n");
    return -EIO;
    }
    board.private_data = kzalloc_obj(struct usb_gpib_priv);
    if (!board.private_data)
    return -ENOMEM;
    retval = lpvo_do_open(board, usb_minors[j]);
    DIA_LOG(1, "lpvo open: %d\n", retval);
    if (retval) {
    dev_err(board.gpib_dev, "lpvo open failed.\n");
    kfree(board.private_data);
    board.private_data = core::ptr::null_mut();
    return -ENODEV;
    }
    show_status(board);
    retval = send_command(board, USB_GPIB_ON, 0);
    DIA_LOG(1, "USB_GPIB_ON returns %x\n", retval);
    if (retval != ACK)
    return -EIO;
//
// We must setup debug mode because we need the extended instruction
// set to cope with the Core (gpib_common) point of view
//
    retval = send_command(board, USB_GPIB_DEBUG_ON, 0);
    DIA_LOG(1, "USB_GPIB_DEBUG_ON returns %x\n", retval);
    if (retval != ACK)
    return -EIO;
//
// We must keep REN off after an IFC because so it is
// assumed by the Core
//
    retval = send_command(board, USB_GPIB_IBm0, 0);
    DIA_LOG(1, "USB_GPIB_IBm0 returns %x\n", retval);
    if (retval != ACK)
    return -EIO;
    retval = set_control_line(board, IB_BUS_REN, 0);
    if (retval != ACK)
    return -EIO;
    retval = send_command(board, USB_GPIB_FTMO, 0);
    DIA_LOG(1, "USB_GPIB_FTMO returns %x\n", retval);
    if (retval != ACK)
    return -EIO;
    show_status(board);
    DIA_LOG(0, "attached\n");
    return 0;
    }
//
// usb_gpib_detach() - deactivate the usb-gpib converter board
//
// @board:    the gpib_board data area for this gpib interface
//
#[no_mangle]
unsafe extern "C" fn usb_gpib_detach(board: *mut gpib_board) {
    static void usb_gpib_detach(struct gpib_board *board)
    {
    int retval;
    show_status(board);
    DIA_LOG(0, "detaching\n");
    if (board.private_data) {
    if (GPIB_DEV) {
    write_loop(GPIB_DEV, USB_GPIB_OFF, strlen(USB_GPIB_OFF));
    msleep(100);
    DIA_LOG(1, "%s", "GPIB off\n");
    retval = lpvo_do_release(board);
    DIA_LOG(1, "lpvo release . %d\n", retval);
    }
    kfree(board.private_data);
    board.private_data = core::ptr::null_mut();
    }
    DIA_LOG(0, "detached\n");
    }
//
// Other functions follow in alphabetical order
//
// command
    static int usb_gpib_command(struct gpib_board *board,
    u8 *buffer,
    size_t length,
    size_t *bytes_written)
    {
    int i, retval;
    char command[6] = "IBc.\n";
    DIA_LOG(1, "enter %p\n", board);
    set_timeout(board);
// bytes_written = 0;
    for (i = 0 ; i < length ; i++) {
    command[3] = buffer[i];
    retval = send_command(board, command, 5);
    DIA_LOG(2, "%d ==> %x %x\n", i, buffer[i], retval);
    if (retval != 0x06)
    return retval;
    ++(*bytes_written);
    }
    return 0;
    }
//
// usb_gpib_disable_eos() - Disable END on eos byte (END on EOI only)
//
// @board:    the gpib_board data area for this gpib interface
//
// With the lpvo adapter eos can only be handled via software.
// Cannot do nothing here, but remember for future use.
//
#[no_mangle]
unsafe extern "C" fn usb_gpib_disable_eos(board: *mut gpib_board) {
    static void usb_gpib_disable_eos(struct gpib_board *board)
    {
    ((struct usb_gpib_priv *)board.private_data).eos_flags &= ~REOS;
    DIA_LOG(1, "done: %x\n",
    ((struct usb_gpib_priv *)board.private_data).eos_flags);
    }
//
// usb_gpib_enable_eos() - Enable END for reads when eos byte is received.
//
// @board:    the gpib_board data area for this gpib interface
// @eos_byte: the 'eos' byte
// @compare_8_bits: if zero ignore eigthth bit when comparing
//
    static int usb_gpib_enable_eos(struct gpib_board *board,
    u8 eos_byte,
    int compare_8_bits)
    {
    struct usb_gpib_priv *pd = (struct usb_gpib_priv *)board.private_data;
    DIA_LOG(1, "enter with %x\n", eos_byte);
    pd.eos = eos_byte;
    pd.eos_flags = REOS;
    if (compare_8_bits)
    pd.eos_flags |= BIN;
    return 0;
    }
//
// usb_gpib_go_to_standby() - De-assert ATN
//
// @board:    the gpib_board data area for this gpib interface
//
#[no_mangle]
unsafe extern "C" fn usb_gpib_go_to_standby(board: *mut gpib_board) -> c_int {
    static int usb_gpib_go_to_standby(struct gpib_board *board)
    {
    let mut retval: c_int = set_control_line(board, IB_BUS_ATN, 0);
    DIA_LOG(1, "done with %x\n", retval);
    if (retval == ACK)
    return 0;
    return -EIO;
    }
//
// usb_gpib_interface_clear() - Assert or de-assert IFC
//
// @board:    the gpib_board data area for this gpib interface
// @assert:   1: assert IFC;  0: de-assert IFC
//
// Currently on the assert request we issue the lpvo IBZ
// command that cycles IFC low for 100 usec, then we ignore
// the de-assert request.
//
#[no_mangle]
unsafe extern "C" fn usb_gpib_interface_clear(board: *mut gpib_board, assert: c_int) {
    static void usb_gpib_interface_clear(struct gpib_board *board, int assert)
    {
    let mut retval: c_int = 0;
    DIA_LOG(1, "enter with %d\n", assert);
    if (assert) {
    retval = send_command(board, USB_GPIB_IBCL, 0);
    set_bit(CIC_NUM, &board.status);
    }
    DIA_LOG(1, "done with %d %d\n", assert, retval);
    }
//
// usb_gpib_line_status() - Read the status of the bus lines.
//
// @board:    the gpib_board data area for this gpib interface
//
// We can read all lines.
//
#[no_mangle]
unsafe extern "C" fn usb_gpib_line_status(board: *const gpib_board) -> c_int {
    static int usb_gpib_line_status(const struct gpib_board *board)
    {
    int buffer;
    int line_status = VALID_ALL;   /* all lines will be read */
    struct list_head *p, *q;
    WQT *item;
    unsigned long flags;
    let mut sleep: c_int = 0;
    DIA_LOG(1, "%s\n", "request");
//
// if we are on the wait queue (board->wait), do not hurry
// reading status line; instead, pause a little
//
    spin_lock_irqsave((spinlock_t *)&board.wait.lock, flags);
    q = (struct list_head *)&board.wait.WQH;
    list_for_each(p, q) {
    item = container_of(p, WQT, WQE);
    if (item.private == current) {
    sleep = 20;
    break;
    }
// pid is: ((struct task_struct *) item->private)->pid);
    }
    spin_unlock_irqrestore((spinlock_t *)&board.wait.lock, flags);
    if (sleep) {
    DIA_LOG(1, "we are on the wait queue - sleep %d ms\n", sleep);
    msleep(sleep);
    }
    buffer = send_command((struct gpib_board *)board, USB_GPIB_STATUS, 0);
    if (buffer < 0) {
    dev_err(board.gpib_dev, "line status read failed with %d\n", buffer);
    return -1;
    }
    if ((buffer & 0x01) == 0)
    line_status |= BUS_REN;
    if ((buffer & 0x02) == 0)
    line_status |= BUS_IFC;
    if ((buffer & 0x04) == 0)
    line_status |= BUS_NDAC;
    if ((buffer & 0x08) == 0)
    line_status |= BUS_NRFD;
    if ((buffer & 0x10) == 0)
    line_status |= BUS_DAV;
    if ((buffer & 0x20) == 0)
    line_status |= BUS_EOI;
    if ((buffer & 0x40) == 0)
    line_status |= BUS_ATN;
    if ((buffer & 0x80) == 0)
    line_status |= BUS_SRQ;
    DIA_LOG(1, "done with %x %x\n", buffer, line_status);
    return line_status;
    }
// parallel_poll
#[no_mangle]
unsafe extern "C" fn usb_gpib_parallel_poll(board: *mut gpib_board, result: *mut u8) -> c_int {
    static int usb_gpib_parallel_poll(struct gpib_board *board, u8 *result)
    {
//
// request parallel poll asserting ATN | EOI;
// we suppose ATN already asserted
//
    int retval;
    DIA_LOG(1, "enter %p\n", board);
    retval = set_control_line(board, IB_BUS_EOI, 1);
    if (retval != ACK)
    return -EIO;
// result = send_command(board, USB_GPIB_READ_DATA, 0);
    DIA_LOG(1, "done with %x\n", *result);
    retval = set_control_line(board, IB_BUS_EOI, 0);
    if (retval != 0x06)
    return -EIO;
    return 0;
    }
// read
    static int usb_gpib_read(struct gpib_board *board,
    u8 *buffer,
    size_t length,
    int *end,
    size_t *bytes_read)
    {
pub const MAX_READ_EXCESS: c_int = 16384;
    let mut b: char_buf = {core::ptr::null_mut(), 0};
    int retval;
    char c, nc;
    int ic;
    struct timespec64 before, after;
    let mut read_count: c_int = MAX_READ_EXCESS;
    struct usb_gpib_priv *pd = (struct usb_gpib_priv *)board.private_data;
    DIA_LOG(1, "enter %p . %zu\n", board, length);
// bytes_read = 0;      /* by default, things go wrong
// end = 0;
    set_timeout(board);
// single byte read has a special handling
    if (length == 1) {
    char inbuf[2] = {0, 0};
// read a single character
    ktime_get_real_ts64 (&before);
    retval = write_loop(GPIB_DEV, USB_GPIB_READ_1, strlen(USB_GPIB_READ_1));
    if (retval < 0)
    return retval;
    retval = lpvo_do_read(GPIB_DEV, inbuf, 1);
    retval += lpvo_do_read(GPIB_DEV, inbuf + 1, 1);
    ktime_get_real_ts64 (&after);
    DIA_LOG(1, "single read: %x %x %x in %d\n", retval,
    inbuf[0], inbuf[1],
    usec_diff(&after, &before));
// good char / last char?
    if (retval == 2 && inbuf[1] == ACK) {
    buffer[0] = inbuf[0];
// bytes_read = 1;
    return 0;
    }
    if (retval < 2)
    return -EIO;
    else
    return -ETIME;
    }
// allocate buffer for multibyte read
    b.inbuf = kmalloc(INBUF_SIZE, GFP_KERNEL);
    if (!b.inbuf)
    return -ENOMEM;
// send read command and check <DLE><STX> sequence
    retval = write_loop(GPIB_DEV, USB_GPIB_READ, strlen(USB_GPIB_READ));
    if (retval < 0)
    goto read_return;
    if (one_char(board, &b) != DLE || one_char(board, &b) != STX) {
    dev_err(board.gpib_dev, "wrong <DLE><STX> sequence\n");
    retval = -EIO;
    goto read_return;
    }
// get data flow
    while (1) {
    ic = one_char(board, &b);
    if (ic == -EIO) {
    retval = -EIO;
    goto read_return;
    }
    c = ic;
    if (c == DLE)
    nc = one_char(board, &b);
    if (c != DLE || nc == DLE) {
// data byte - store into buffer
    if (*bytes_read == length)
    break; /* data overflow */
    if (c == DLE)
    c = nc;
    buffer[(*bytes_read)++] = c;
    if (c == pd.eos) {
// end = 1;
    break;
    }
    } else {
// we are in the closing <DLE><ETX> sequence
    c = nc;
    if (c == ETX) {
    c = one_char(board, &b);
    if (c == ACK) {
// end = 1;
    retval = 0;
    goto read_return;
    } else {
    dev_err(board.gpib_dev, "wrong end of message %x", c);
    retval = -ETIME;
    goto read_return;
    }
    } else {
    dev_err(board.gpib_dev, "lone <DLE> in stream");
    retval = -EIO;
    goto read_return;
    }
    }
    }
// we had a data overflow - flush excess data
    while (read_count--) {
    if (one_char(board, &b) != DLE)
    continue;
    c = one_char(board, &b);
    if (c == DLE)
    continue;
    if (c == ETX) {
    c = one_char(board, &b);
    if (c == ACK) {
    if (MAX_READ_EXCESS - read_count > 1)
    dev_dbg(board.gpib_dev, "small buffer - maybe some data lost");
    retval = 0;
    goto read_return;
    }
    break;
    }
    }
    dev_err(board.gpib_dev, "no input end - board in odd state\n");
    retval = -EIO;
    read_return:
    kfree(b.inbuf);
    DIA_LOG(1, "done with byte/status: %d %x %d\n",	(int)*bytes_read, retval, *end);
    if (retval == 0 || retval == -ETIME) {
    if (send_command(board, USB_GPIB_UNTALK, sizeof(USB_GPIB_UNTALK)) == 0x06)
    return retval;
    return	-EIO;
    }
    return retval;
    }
// remote_enable
#[no_mangle]
unsafe extern "C" fn usb_gpib_remote_enable(board: *mut gpib_board, enable: c_int) {
    static void usb_gpib_remote_enable(struct gpib_board *board, int enable)
    {
    int retval;
    retval = set_control_line(board, IB_BUS_REN, enable ? 1 : 0);
    if (retval != ACK)
    dev_err(board.gpib_dev, "could not set REN line: %x\n", retval);
    DIA_LOG(1, "done with %x\n", retval);
    }
// request_system_control
#[no_mangle]
unsafe extern "C" fn usb_gpib_request_system_control(board: *mut gpib_board, request_control: c_int) -> c_int {
    static int usb_gpib_request_system_control(struct gpib_board *board, int request_control)
    {
    if (!request_control)
    return -EINVAL;
    DIA_LOG(1, "done with %d . %lx\n", request_control, board.status);
    return 0;
    }
// take_control
// beware: the sync flag is ignored; what is its real meaning?
#[no_mangle]
unsafe extern "C" fn usb_gpib_take_control(board: *mut gpib_board, sync: c_int) -> c_int {
    static int usb_gpib_take_control(struct gpib_board *board, int sync)
    {
    int retval;
    retval = set_control_line(board, IB_BUS_ATN, 1);
    DIA_LOG(1, "done with %d %x\n", sync, retval);
    if (retval == ACK)
    return 0;
    return -EIO;
    }
// update_status
    static unsigned int usb_gpib_update_status(struct gpib_board *board,
    unsigned int clear_mask)
    {
// There is nothing we can do here, I guess
    board.status &= ~clear_mask;
    DIA_LOG(1, "done with %x %lx\n", clear_mask, board.status);
    return board.status;
    }
// write
// beware: DLE characters are not escaped - can only send ASCII data
    static int usb_gpib_write(struct gpib_board *board,
    u8 *buffer,
    size_t length,
    int send_eoi,
    size_t *bytes_written)
    {
    int retval;
    char *msg;
    DIA_LOG(1, "enter %p . %zu\n", board, length);
    set_timeout(board);
    msg = kmalloc(length + 8, GFP_KERNEL);
    if (!msg)
    return -ENOMEM;
    memcpy(msg, "\nIB\020\002", 5);
    memcpy(msg + 5, buffer, length);
    memcpy(msg + 5 + length, "\020\003\n", 3);
    retval = send_command(board, msg, length + 8);
    kfree(msg);
    DIA_LOG(1, "<%.*s> . %x\n", (int)length, buffer, retval);
    if (retval != ACK)
    return -EPIPE;
// bytes_written = length;
    if (send_command(board, USB_GPIB_UNLISTEN, sizeof(USB_GPIB_UNLISTEN)) != 0x06)
    return -EPIPE;
    return length;
    }
//
// ***	 following functions not implemented yet
//
// parallel_poll configure
    static void usb_gpib_parallel_poll_configure(struct gpib_board *board,
    u8 configuration)
    {
    }
// parallel_poll_response
#[no_mangle]
unsafe extern "C" fn usb_gpib_parallel_poll_response(board: *mut gpib_board, ist: c_int) {
    static void usb_gpib_parallel_poll_response(struct gpib_board *board, int ist)
    {
    }
// primary_address
#[no_mangle]
unsafe extern "C" fn usb_gpib_primary_address(board: *mut gpib_board, address: c_uint) -> c_int {
    static int  usb_gpib_primary_address(struct gpib_board *board, unsigned int address)
    {
    return 0;
    }
// return_to_local
#[no_mangle]
unsafe extern "C" fn usb_gpib_return_to_local(board: *mut gpib_board) {
    static	void usb_gpib_return_to_local(struct gpib_board *board)
    {
    }
// secondary_address
    static int usb_gpib_secondary_address(struct gpib_board *board,
    unsigned int address,
    int enable)
    {
    return 0;
    }
// serial_poll_response
#[no_mangle]
unsafe extern "C" fn usb_gpib_serial_poll_response(board: *mut gpib_board, status: u8) {
    static void usb_gpib_serial_poll_response(struct gpib_board *board, u8 status)
    {
    }
// serial_poll_status
#[no_mangle]
unsafe extern "C" fn usb_gpib_serial_poll_status(board: *mut gpib_board) -> u8 {
    static u8 usb_gpib_serial_poll_status(struct gpib_board *board)
    {
    return 0;
    }
// t1_delay
#[no_mangle]
unsafe extern "C" fn usb_gpib_t1_delay(board: *mut gpib_board, nano_sec: c_uint) -> c_int {
    static int usb_gpib_t1_delay(struct gpib_board *board, unsigned int nano_sec)
    {
    return 0;
    }
//
// ***  module dispatch table and init/exit functions
//
    static struct gpib_interface usb_gpib_interface = {
    .name = NAME,
    .attach = usb_gpib_attach,
    .detach = usb_gpib_detach,
    .read = usb_gpib_read,
    .write = usb_gpib_write,
    .command = usb_gpib_command,
    .take_control = usb_gpib_take_control,
    .go_to_standby = usb_gpib_go_to_standby,
    .request_system_control = usb_gpib_request_system_control,
    .interface_clear = usb_gpib_interface_clear,
    .remote_enable = usb_gpib_remote_enable,
    .enable_eos = usb_gpib_enable_eos,
    .disable_eos = usb_gpib_disable_eos,
    .parallel_poll = usb_gpib_parallel_poll,
    .parallel_poll_configure = usb_gpib_parallel_poll_configure,
    .parallel_poll_response = usb_gpib_parallel_poll_response,
    .local_parallel_poll_mode = core::ptr::null_mut(), // XXX
    .line_status = usb_gpib_line_status,
    .update_status = usb_gpib_update_status,
    .primary_address = usb_gpib_primary_address,
    .secondary_address = usb_gpib_secondary_address,
    .serial_poll_response = usb_gpib_serial_poll_response,
    .serial_poll_status = usb_gpib_serial_poll_status,
    .t1_delay = usb_gpib_t1_delay,
    .return_to_local = usb_gpib_return_to_local,
    .skip_check_for_command_acceptors = 1
    };
//
// usb_gpib_init_module(), usb_gpib_exit_module()
//
// This functions are called every time a new device is detected
// and registered or is removed and unregistered.
// We must take note of created and destroyed usb minors to be used
// when usb_gpib_attach() and usb_gpib_detach() will be called on
// request by gpib_config.
//
#[no_mangle]
unsafe extern "C" fn usb_gpib_init_module(interface: *mut usb_interface) -> c_int {
    static int usb_gpib_init_module(struct usb_interface *interface)
    {
    int j, mask, rv;
    rv = mutex_lock_interruptible(&minors_lock);
    if (rv < 0)
    return rv;
    if (!assigned_usb_minors) {
    rv = gpib_register_driver(&usb_gpib_interface, THIS_MODULE);
    if (rv) {
    pr_err("gpib_register_driver failed: error = %d\n", rv);
    goto exit;
    }
    } else {
//
// check if minor is already registered - maybe useless, but if
// it happens the code is inconsistent somewhere
//
    for (j = 0 ; j < MAX_DEV ; j++) {
    if (usb_minors[j] == interface.minor && assigned_usb_minors & 1 << j) {
    pr_err("CODE BUG: USB minor %d registered at %d.\n",
    interface.minor, j);
    rv = -1;
    goto exit;
    }
    }
    }
// find a free slot
    for (j = 0 ; j < MAX_DEV ; j++) {
    mask = 1 << j;
    if ((assigned_usb_minors & mask) == 0) {
    usb_minors[j] = interface.minor;
    lpvo_usb_interfaces[j] = interface;
    assigned_usb_minors |= mask;
    rv = 0;
    goto exit;
    }
    }
    pr_err("No slot available for interface %p minor %d\n", interface, interface.minor);
    rv = -1;
    exit:
    mutex_unlock(&minors_lock);
    return rv;
    }
#[no_mangle]
unsafe extern "C" fn usb_gpib_exit_module(minor: c_int) {
    static void usb_gpib_exit_module(int minor)
    {
    int j;
    mutex_lock(&minors_lock);
    for (j = 0 ; j < MAX_DEV ; j++) {
    if (usb_minors[j] == minor && assigned_usb_minors & 1 << j) {
    assigned_usb_minors &= ~(1 << j);
    usb_minors[j] = -1;
    if (assigned_usb_minors == 0)
    gpib_unregister_driver(&usb_gpib_interface);
    goto exit;
    }
    }
    pr_err("CODE BUG: USB minor %d not found.\n", minor);
    exit:
    mutex_unlock(&minors_lock);
    }
//
// Default latency time (16 msec) is too long.
// We must use 1 msec (best); anyhow, no more than 5 msec.
//
// Defines and function taken and modified from the kernel tree
// (see ftdi_sio.h and ftdi_sio.c).
//

pub const FTDI_SIO_SET_LATENCY_TIMER_REQUEST_TYPE: c_uint = 0x40;

#[no_mangle]
unsafe extern "C" fn write_latency_timer(udev: *mut usb_device) -> c_int {
    static int write_latency_timer(struct usb_device *udev)
    {
    int rv = usb_control_msg(udev,
    usb_sndctrlpipe(udev, 0),
    FTDI_SIO_SET_LATENCY_TIMER_REQUEST,
    FTDI_SIO_SET_LATENCY_TIMER_REQUEST_TYPE,
    LATENCY_TIMER, LATENCY_CHANNEL,
    core::ptr::null_mut(), 0, WDR_TIMEOUT);
    if (rv < 0)
    dev_err(&udev.dev, "Unable to write latency timer: %i\n", rv);
    return rv;
    }
//
// The following code is a modified version of the USB Skeleton driver
// written by Greg Kroah-Hartman and available in the kernel tree.
//
// Functions skel_open() and skel_release() have been rewritten and named
// lpvo_do_open() and lpvo_do_release() to process the attach and detach
// requests coming from gpib_config.
//
// Functions lpvo_read() and lpvo_write() have been split into a
// lpvo_do_read() and lpvo_do_write(), that cover the kernel stuff of read
// and write operations, and the original lpvo_read() and lpvo_write(),
// that handle communication with user space and call their _do_ companion.
//
// Only the _do_ versions are used by the lpvo_usb_gpib driver; other ones
// can be (optionally) maintained in the compilation to have direct access
// to a gpib controller for debug and diagnostics.
//
// To avoid collisions in names, devices in user space have been renamed
// lpvo_raw1, lpvo_raw2 ....  and the usb driver has been renamed with the
// gpib module name.
//
// USB Skeleton driver - 2.2
//
// Copyright (C) 2001-2004 Greg Kroah-Hartman (greg@kroah.com)
//
// This driver is based on the 2.6.3 version of drivers/usb/usb-skeleton.c
// but has been rewritten to be easier to read and use.
//

// Get a minor range for your devices from the usb maintainer
pub const USB_LPVO_MINOR_BASE: c_int = 192;
// private defines

//
// MAX_TRANSFER is chosen so that the VM is not stressed by
// allocations > PAGE_SIZE and the number of packets in a page
// is an integer 512 is the largest possible packet on EHCI
//

// Structure to hold all of our device specific stuff
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpvo {
    pub /: *mut *mut *mut usb_device udev; / the usb device for this device,
    pub /: *mut *mut *mut usb_interface interface; / the interface for this device,
    pub /: *mut *mut semaphore limit_sem; / limiting the number of writes in progress,
    pub /: *mut *mut usb_anchor submitted; / in case need to retract our submissions,
    pub /: *mut *mut *mut urb bulk_in_urb; / the urb to read data with,
    pub /: *mut *mut *mut unsigned char bulk_in_buffer; / the buffer to receive data,
    pub /: *mut *mut size_t bulk_in_size; / the size of the receive buffer,
    pub /: *mut *mut size_t bulk_in_filled; / number of bytes in the buffer,
    pub /: *mut *mut size_t bulk_in_copied; / already copied to user space,
    pub /: *mut *mut __u8 bulk_in_endpoint_addr; / the address of the bulk in endpoint,
    pub /: *mut *mut __u8 bulk_out_endpoint_addr; / the address of the bulk out endpoint,
    pub /: *mut *mut int errors; / the last request tanked,
    pub /: *mut *mut bool ongoing_read; / a read is going on,
    pub /: *mut *mut spinlock_t err_lock; / lock for errors,
    pub kref: kref,
    pub /: *mut *mut mutex io_mutex; / synchronize I/O with disconnect,
    pub /: *mut *mut wait_queue_head_t bulk_in_wait; / to wait for an ongoing read,
}

    static struct usb_driver lpvo_driver;
    static void lpvo_draw_down(struct lpvo *dev);
#[no_mangle]
unsafe extern "C" fn lpvo_delete(kref: *mut kref) {
    static void lpvo_delete(struct kref *kref)
    {
    struct lpvo *dev = to_lpvo_dev(kref);
    usb_free_urb(dev.bulk_in_urb);
    usb_put_dev(dev.udev);
    kfree(dev.bulk_in_buffer);
    kfree(dev);
    }
//
// lpvo_do_open() - to be called by usb_gpib_attach
//
#[no_mangle]
unsafe extern "C" fn lpvo_do_open(board: *mut gpib_board, subminor: c_int) -> c_int {
    static int lpvo_do_open(struct gpib_board *board, int subminor)
    {
    struct lpvo *dev;
    struct usb_interface *interface;
    let mut retval: c_int = 0;
    interface = usb_find_interface(&lpvo_driver, subminor);
    if (!interface) {
    dev_err(board.gpib_dev, "can't find device for minor %d\n", subminor);
    retval = -ENODEV;
    goto exit;
    }
    dev = usb_get_intfdata(interface);
    if (!dev) {
    retval = -ENODEV;
    goto exit;
    }
    retval = usb_autopm_get_interface(interface);
    if (retval)
    goto exit;
// increment our usage count for the device
    kref_get(&dev.kref);
// save our object in the file's private structure
    GPIB_DEV = dev;
    exit:
    return retval;
    }
//
// lpvo_do_release() - to be called by usb_gpib_detach
//
#[no_mangle]
unsafe extern "C" fn lpvo_do_release(board: *mut gpib_board) -> c_int {
    static int lpvo_do_release(struct gpib_board *board)
    {
    struct lpvo *dev;
    dev = GPIB_DEV;
    if (!dev)
    return -ENODEV;
// allow the device to be autosuspended
    mutex_lock(&dev.io_mutex);
    if (dev.interface)
    usb_autopm_put_interface(dev.interface);
    mutex_unlock(&dev.io_mutex);
// decrement the count on our device
    kref_put(&dev.kref, lpvo_delete);
    return 0;
    }
//
// read functions
//
#[no_mangle]
unsafe extern "C" fn lpvo_read_bulk_callback(urb: *mut urb) {
    static void lpvo_read_bulk_callback(struct urb *urb)
    {
    struct lpvo *dev;
    unsigned long flags;
    dev = urb.context;
    spin_lock_irqsave(&dev.err_lock, flags);
// sync/async unlink faults aren't errors
    if (urb.status) {
    if (!(urb.status == -ENOENT ||
    urb.status == -ECONNRESET ||
    urb.status == -ESHUTDOWN))
    dev_err(&dev.interface.dev, "nonzero read bulk status received: %d\n",
    urb.status);
    dev.errors = urb.status;
    } else {
    dev.bulk_in_filled = urb.actual_length;
    }
    dev.ongoing_read = 0;
    spin_unlock_irqrestore(&dev.err_lock, flags);
    wake_up_interruptible(&dev.bulk_in_wait);
    }
#[no_mangle]
unsafe extern "C" fn lpvo_do_read_io(dev: *mut lpvo, count: usize) -> c_int {
    static int lpvo_do_read_io(struct lpvo *dev, size_t count)
    {
    int rv;
// prepare a read
    usb_fill_bulk_urb(dev.bulk_in_urb,
    dev.udev,
    usb_rcvbulkpipe(dev.udev,
    dev.bulk_in_endpoint_addr),
    dev.bulk_in_buffer,
    min(dev.bulk_in_size, count),
    lpvo_read_bulk_callback,
    dev);
// tell everybody to leave the URB alone
    spin_lock_irq(&dev.err_lock);
    dev.ongoing_read = 1;
    spin_unlock_irq(&dev.err_lock);
// submit bulk in urb, which means no data to deliver
    dev.bulk_in_filled = 0;
    dev.bulk_in_copied = 0;
// do it
    rv = usb_submit_urb(dev.bulk_in_urb, GFP_KERNEL);
    if (rv < 0) {
    dev_err(&dev.interface.dev, "failed submitting read urb, error %d\n", rv);
    rv = (rv == -ENOMEM) ? rv : -EIO;
    spin_lock_irq(&dev.err_lock);
    dev.ongoing_read = 0;
    spin_unlock_irq(&dev.err_lock);
    }
    return rv;
    }
//
// lpvo_do_read() - read operations from lpvo_usb_gpib
//
#[no_mangle]
unsafe extern "C" fn lpvo_do_read(dev: *mut lpvo, buffer: *mut c_char, count: usize) -> isize {
    static ssize_t lpvo_do_read(struct lpvo *dev, char *buffer, size_t count)
    {
    int rv;
    bool ongoing_io;
// if we cannot read at all, return EOF
    if (!dev.bulk_in_urb || !count)
    return 0;
    restart:  /* added to comply with ftdi timeout technique */
// no concurrent readers
    rv = mutex_lock_interruptible(&dev.io_mutex);
    if (rv < 0)
    return rv;
    if (!dev.interface) {		      /* disconnect() was called */
    rv = -ENODEV;
    goto exit;
    }
    retry:
// if IO is under way, we must not touch things
    spin_lock_irq(&dev.err_lock);
    ongoing_io = dev.ongoing_read;
    spin_unlock_irq(&dev.err_lock);
    if (ongoing_io) {
// /* nonblocking IO shall not wait
// /* no file, no O_NONBLOCK; maybe provide when from user space
// if (file->f_flags & O_NONBLOCK) {
// rv = -EAGAIN;
// goto exit;
// }
//
// IO may take forever
// hence wait in an interruptible state
//
    rv = wait_event_interruptible(dev.bulk_in_wait, (!dev.ongoing_read));
    if (rv < 0)
    goto exit;
    }
// errors must be reported
    rv = dev.errors;
    if (rv < 0) {
// any error is reported once
    dev.errors = 0;
// to preserve notifications about reset
    rv = (rv == -EPIPE) ? rv : -EIO;
// report it
    goto exit;
    }
//
// if the buffer is filled we may satisfy the read
// else we need to start IO
//
    if (dev.bulk_in_filled) {
// we had read data
    let mut available: usize = dev.bulk_in_filled - dev.bulk_in_copied;
// size_t chunk = min(available, count);	 /* compute chunk later
    size_t chunk;
    if (!available) {
//
// all data has been used
// actual IO needs to be done
//
// it seems that requests for less than dev->bulk_in_size
// are not accepted
//
    rv = lpvo_do_read_io(dev, dev.bulk_in_size);
    if (rv < 0)
    goto exit;
    else
    goto retry;
    }
//
// data is available - chunk tells us how much shall be copied
//
// Condition dev->bulk_in_copied > 0 maybe will never happen. In case,
// signal the event and copy using the original procedure, i.e., copy
// first two bytes also
//
    if (dev.bulk_in_copied) {
    chunk = min(available, count);
    memcpy(buffer, dev.bulk_in_buffer + dev.bulk_in_copied, chunk);
    rv = chunk;
    dev.bulk_in_copied += chunk;
// copy discarding first two bytes that contain ftdi chip status
    } else {
// account for two bytes to be discarded
    chunk = min(available, count + 2);
    if (chunk < 2) {
    dev_err(&dev.udev.dev, "BAD READ - chunk: %zu\n", chunk);
    rv = -EIO;
    goto exit;
    }
    memcpy(buffer, dev.bulk_in_buffer + 2, chunk - 2);
    rv = chunk;
    dev.bulk_in_copied += chunk;
    }
//
// if we are asked for more than we have,
// we start IO but don't wait
//
// No, no read ahead allowed; if the case, more data will be
// asked for by the lpvo_usb_gpib layer.
//
// if (available < count)
// lpvo_do_read_io(dev, dev->bulk_in_size);
    } else {
// no data in the buffer
    rv = lpvo_do_read_io(dev, dev.bulk_in_size);
    if (rv < 0)
    goto exit;
    else
    goto retry;
    }
    exit:
    mutex_unlock(&dev.io_mutex);
    if (rv == 2)
    goto restart;	/* ftdi chip returns two status bytes after a latency anyhow */
    if (rv > 0)
    return rv - 2;	/* account for 2 discarded bytes in a valid buffer */
    return rv;
    }
//
// write functions
//
#[no_mangle]
unsafe extern "C" fn lpvo_write_bulk_callback(urb: *mut urb) {
    static void lpvo_write_bulk_callback(struct urb *urb)
    {
    struct lpvo *dev;
    unsigned long flags;
    dev = urb.context;
// sync/async unlink faults aren't errors
    if (urb.status) {
    if (!(urb.status == -ENOENT ||
    urb.status == -ECONNRESET ||
    urb.status == -ESHUTDOWN))
    dev_err(&dev.interface.dev,
    "nonzero write bulk status received: %d\n", urb.status);
    spin_lock_irqsave(&dev.err_lock, flags);
    dev.errors = urb.status;
    spin_unlock_irqrestore(&dev.err_lock, flags);
    }
// free up our allocated buffer
    usb_free_coherent(urb.dev, urb.transfer_buffer_length,
    urb.transfer_buffer, urb.transfer_dma);
    up(&dev.limit_sem);
    }
//
// lpvo_do_write() - write operations from lpvo_usb_gpib
//
#[no_mangle]
unsafe extern "C" fn lpvo_do_write(dev: *mut lpvo, buffer: *const c_char, count: usize) -> isize {
    static ssize_t lpvo_do_write(struct lpvo *dev, const char *buffer, size_t count)
    {
    let mut retval: c_int = 0;
    struct urb *urb = core::ptr::null_mut();
    char *buf = core::ptr::null_mut();
    let mut writesize: usize = min_t(size_t, count, (size_t)MAX_TRANSFER);
// verify that we actually have some data to write
    if (count == 0)
    goto exit;
//
// limit the number of URBs in flight to stop a user from using up all
// RAM
//
// Only one URB is used, because we can't have a pending write() and go on
// if (!(file->f_flags & O_NONBLOCK)) {	/* no NONBLOCK provided
    if (down_interruptible(&dev.limit_sem)) {
    retval = -ERESTARTSYS;
    goto exit;
    }
// } else {
// if (down_trylock(&dev->limit_sem)) {
// retval = -EAGAIN;
// goto exit;
// }
    spin_lock_irq(&dev.err_lock);
    retval = dev.errors;
    if (retval < 0) {
// any error is reported once
    dev.errors = 0;
// to preserve notifications about reset
    retval = (retval == -EPIPE) ? retval : -EIO;
    }
    spin_unlock_irq(&dev.err_lock);
    if (retval < 0)
    goto error;
// create a urb, and a buffer for it, and copy the data to the urb
    urb = usb_alloc_urb(0, GFP_KERNEL);
    if (!urb) {
    retval = -ENOMEM;
    goto error;
    }
    buf = usb_alloc_coherent(dev.udev, writesize, GFP_KERNEL,
    &urb.transfer_dma);
    if (!buf) {
    retval = -ENOMEM;
    goto error;
    }
    memcpy(buf, buffer, count);
// this lock makes sure we don't submit URBs to gone devices
    mutex_lock(&dev.io_mutex);
    if (!dev.interface) {		      /* disconnect() was called */
    mutex_unlock(&dev.io_mutex);
    retval = -ENODEV;
    goto error;
    }
// initialize the urb properly
    usb_fill_bulk_urb(urb, dev.udev,
    usb_sndbulkpipe(dev.udev, dev.bulk_out_endpoint_addr),
    buf, writesize, lpvo_write_bulk_callback, dev);
    urb.transfer_flags |= URB_NO_TRANSFER_DMA_MAP;
    usb_anchor_urb(urb, &dev.submitted);
// send the data out the bulk port
    retval = usb_submit_urb(urb, GFP_KERNEL);
    mutex_unlock(&dev.io_mutex);
    if (retval) {
    dev_err(&dev.interface.dev, "failed submitting write urb, error %d\n", retval);
    goto error_unanchor;
    }
//
// release our reference to this urb, the USB core will eventually free
// it entirely
//
    usb_free_urb(urb);
    return writesize;
    error_unanchor:
    usb_unanchor_urb(urb);
    error:
    if (urb) {
    usb_free_coherent(dev.udev, writesize, buf, urb.transfer_dma);
    usb_free_urb(urb);
    }
    up(&dev.limit_sem);
    exit:
    return retval;
    }
//
// services for the user space devices
//

#[no_mangle]
unsafe extern "C" fn lpvo_flush(file: *mut file, id: fl_owner_t) -> c_int {
    static int lpvo_flush(struct file *file, fl_owner_t id)
    {
    struct lpvo *dev;
    int res;
    dev = file.private_data;
    if (!dev)
    return -ENODEV;
// wait for io to stop
    mutex_lock(&dev.io_mutex);
    lpvo_draw_down(dev);
// read out errors, leave subsequent opens a clean slate
    spin_lock_irq(&dev.err_lock);
    res = dev.errors ? (dev.errors == -EPIPE ? -EPIPE : -EIO) : 0;
    dev.errors = 0;
    spin_unlock_irq(&dev.err_lock);
    mutex_unlock(&dev.io_mutex);
    return res;
    }
#[no_mangle]
unsafe extern "C" fn lpvo_open(inode: *mut inode, file: *mut file) -> c_int {
    static int lpvo_open(struct inode *inode, struct file *file)
    {
    struct lpvo *dev;
    struct usb_interface *interface;
    int subminor;
    let mut retval: c_int = 0;
    subminor = iminor(inode);
    interface = usb_find_interface(&lpvo_driver, subminor);
    if (!interface) {
    pr_err("can't find device for minor %d\n", subminor);
    retval = -ENODEV;
    goto exit;
    }
    dev = usb_get_intfdata(interface);
    if (!dev) {
    retval = -ENODEV;
    goto exit;
    }
    retval = usb_autopm_get_interface(interface);
    if (retval)
    goto exit;
// increment our usage count for the device
    kref_get(&dev.kref);
// save our object in the file's private structure
    file.private_data = dev;
    exit:
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn lpvo_release(inode: *mut inode, file: *mut file) -> c_int {
    static int lpvo_release(struct inode *inode, struct file *file)
    {
    struct lpvo *dev;
    dev = file.private_data;
    if (!dev)
    return -ENODEV;
// allow the device to be autosuspended
    mutex_lock(&dev.io_mutex);
    if (dev.interface)
    usb_autopm_put_interface(dev.interface);
    mutex_unlock(&dev.io_mutex);
// decrement the count on our device
    kref_put(&dev.kref, lpvo_delete);
    return 0;
    }
//
// user space access to read function
//
    static ssize_t lpvo_read(struct file *file, char __user *buffer, size_t count,
    loff_t *ppos)
    {
    struct lpvo *dev;
    char *buf;
    ssize_t rv;
    dev = file.private_data;
    buf = kmalloc(count, GFP_KERNEL);
    if (!buf)
    return -ENOMEM;
    rv = lpvo_do_read(dev, buf, count);
    if (rv > 0) {
    if (copy_to_user(buffer, buf, rv)) {
    kfree(buf);
    return -EFAULT;
    }
    }
    kfree(buf);
    return rv;
    }
//
// user space access to write function
//
    static ssize_t lpvo_write(struct file *file, const char __user *user_buffer,
    size_t count, loff_t *ppos)
    {
    struct lpvo *dev;
    char *buf;
    ssize_t rv;
    dev = file.private_data;
    buf = memdup_user(user_buffer, count);
    if (IS_ERR(buf))
    return PTR_ERR(buf);
    rv = lpvo_do_write(dev, buf, count);
    kfree(buf);
    return rv;
    }

    static const struct file_operations lpvo_fops = {
    .owner =	THIS_MODULE,

    .read =	   lpvo_read,
    .write =   lpvo_write,
    .open =	   lpvo_open,
    .release = lpvo_release,
    .flush =   lpvo_flush,
    .llseek =  noop_llseek,

    };
//
// usb class driver info in order to get a minor number from the usb core,
// and to have the device registered with the driver core
//

    static struct usb_class_driver lpvo_class = {
    .name =		       "lpvo_raw%d",
    .fops =		       &lpvo_fops,
    .minor_base =	     USB_LPVO_MINOR_BASE,
    };

    static int lpvo_probe(struct usb_interface *interface,
    const struct usb_device_id *id)
    {
    struct lpvo *dev;
    struct usb_endpoint_descriptor *bulk_in, *bulk_out;
    int retval;
    char *device_path;
    mutex_init(&minors_lock);   /* required for handling minor numbers table */
// allocate memory for our device state and initialize it
    dev = kzalloc_obj(*dev);
    if (!dev)
    return -ENOMEM;
    kref_init(&dev.kref);
    sema_init(&dev.limit_sem, WRITES_IN_FLIGHT);
    mutex_init(&dev.io_mutex);
    spin_lock_init(&dev.err_lock);
    init_usb_anchor(&dev.submitted);
    init_waitqueue_head(&dev.bulk_in_wait);
    dev.udev = usb_get_dev(interface_to_usbdev(interface));
    dev.interface = interface;
// set up the endpoint information
// use only the first bulk-in and bulk-out endpoints
    retval = usb_find_common_endpoints(interface.cur_altsetting,
    &bulk_in, &bulk_out, core::ptr::null_mut(), core::ptr::null_mut());
    if (retval) {
    dev_err(&interface.dev,
    "Could not find both bulk-in and bulk-out endpoints\n");
    goto error;
    }
    dev.bulk_in_size = usb_endpoint_maxp(bulk_in);
    dev.bulk_in_endpoint_addr = bulk_in.bEndpointAddress;
    dev.bulk_in_buffer = kmalloc(dev.bulk_in_size, GFP_KERNEL);
    if (!dev.bulk_in_buffer) {
    retval = -ENOMEM;
    goto error;
    }
    dev.bulk_in_urb = usb_alloc_urb(0, GFP_KERNEL);
    if (!dev.bulk_in_urb) {
    retval = -ENOMEM;
    goto error;
    }
    dev.bulk_out_endpoint_addr = bulk_out.bEndpointAddress;
// save our data pointer in this interface device
    usb_set_intfdata(interface, dev);
// let the world know
    device_path = kobject_get_path(&dev.udev.dev.kobj, GFP_KERNEL);
    dev_dbg(&interface.dev, "New lpvo_usb_device . bus: %d  dev: %d  path: %s\n",
    dev.udev.bus.busnum, dev.udev.devnum, device_path);
    kfree(device_path);

// we can register the device now, as it is ready
    retval = usb_register_dev(interface, &lpvo_class);
    if (retval) {
// something prevented us from registering this driver
    dev_err(&interface.dev,
    "Not able to get a minor for this device.\n");
    usb_set_intfdata(interface, core::ptr::null_mut());
    goto error;
    }

    write_latency_timer(dev.udev);	    /* adjust the latency timer */
    usb_gpib_init_module(interface);    /* last, init the lpvo for this minor */
    return 0;
    error:
// this frees allocated memory
    kref_put(&dev.kref, lpvo_delete);
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn lpvo_disconnect(interface: *mut usb_interface) {
    static void lpvo_disconnect(struct usb_interface *interface)
    {
    struct lpvo *dev;
    let mut minor: c_int = interface.minor;
    usb_gpib_exit_module(minor);	  /* first, disactivate the lpvo */
    dev = usb_get_intfdata(interface);
    usb_set_intfdata(interface, core::ptr::null_mut());

// give back our minor
    usb_deregister_dev(interface, &lpvo_class);

// prevent more I/O from starting
    mutex_lock(&dev.io_mutex);
    dev.interface = core::ptr::null_mut();
    mutex_unlock(&dev.io_mutex);
    usb_kill_anchored_urbs(&dev.submitted);
// decrement our usage count
    kref_put(&dev.kref, lpvo_delete);
    }
#[no_mangle]
unsafe extern "C" fn lpvo_draw_down(dev: *mut lpvo) {
    static void lpvo_draw_down(struct lpvo *dev)
    {
    int time;
    time = usb_wait_anchor_empty_timeout(&dev.submitted, 1000);
    if (!time)
    usb_kill_anchored_urbs(&dev.submitted);
    usb_kill_urb(dev.bulk_in_urb);
    }
#[no_mangle]
unsafe extern "C" fn lpvo_suspend(intf: *mut usb_interface, message: pm_message_t) -> c_int {
    static int lpvo_suspend(struct usb_interface *intf, pm_message_t message)
    {
    struct lpvo *dev = usb_get_intfdata(intf);
    if (!dev)
    return 0;
    lpvo_draw_down(dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lpvo_resume(intf: *mut usb_interface) -> c_int {
    static int lpvo_resume(struct usb_interface *intf)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lpvo_pre_reset(intf: *mut usb_interface) -> c_int {
    static int lpvo_pre_reset(struct usb_interface *intf)
    {
    struct lpvo *dev = usb_get_intfdata(intf);
    mutex_lock(&dev.io_mutex);
    lpvo_draw_down(dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lpvo_post_reset(intf: *mut usb_interface) -> c_int {
    static int lpvo_post_reset(struct usb_interface *intf)
    {
    struct lpvo *dev = usb_get_intfdata(intf);
// we are sure no URBs are active - no locking needed
    dev.errors = -EPIPE;
    mutex_unlock(&dev.io_mutex);
    return 0;
    }
    static struct usb_driver lpvo_driver = {
    .name =			NAME,
    .probe =		lpvo_probe,
    .disconnect =		lpvo_disconnect,
    .suspend =		lpvo_suspend,
    .resume =		lpvo_resume,
    .pre_reset =		lpvo_pre_reset,
    .post_reset =		lpvo_post_reset,
    .id_table =		lpvo_table,
    .supports_autosuspend = 1,
    };
    module_usb_driver(lpvo_driver);
