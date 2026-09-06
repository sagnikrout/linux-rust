//! Automatically rewritten from C to Rust
//! Source: drivers/w1/masters/ds2490.c
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
// ds2490.c  USB to one wire bridge
//
// Copyright (c) 2004 Evgeniy Polyakov <zbr@ioremap.net>
//

// USB Standard
// USB Control request vendor type
pub const VENDOR: c_uint = 0x40;
// COMMAND TYPE CODES
pub const CONTROL_CMD: c_uint = 0x00;
pub const COMM_CMD: c_uint = 0x01;
pub const MODE_CMD: c_uint = 0x02;
// CONTROL COMMAND CODES
pub const CTL_RESET_DEVICE: c_uint = 0x0000;
pub const CTL_START_EXE: c_uint = 0x0001;
pub const CTL_RESUME_EXE: c_uint = 0x0002;
pub const CTL_HALT_EXE_IDLE: c_uint = 0x0003;
pub const CTL_HALT_EXE_DONE: c_uint = 0x0004;
pub const CTL_FLUSH_COMM_CMDS: c_uint = 0x0007;
pub const CTL_FLUSH_RCV_BUFFER: c_uint = 0x0008;
pub const CTL_FLUSH_XMT_BUFFER: c_uint = 0x0009;
pub const CTL_GET_COMM_CMDS: c_uint = 0x000A;
// MODE COMMAND CODES
pub const MOD_PULSE_EN: c_uint = 0x0000;
pub const MOD_SPEED_CHANGE_EN: c_uint = 0x0001;
pub const MOD_1WIRE_SPEED: c_uint = 0x0002;
pub const MOD_STRONG_PU_DURATION: c_uint = 0x0003;
pub const MOD_PULLDOWN_SLEWRATE: c_uint = 0x0004;
pub const MOD_PROG_PULSE_DURATION: c_uint = 0x0005;
pub const MOD_WRITE1_LOWTIME: c_uint = 0x0006;
pub const MOD_DSOW0_TREC: c_uint = 0x0007;
// COMMUNICATION COMMAND CODES
pub const COMM_ERROR_ESCAPE: c_uint = 0x0601;
pub const COMM_SET_DURATION: c_uint = 0x0012;
pub const COMM_BIT_IO: c_uint = 0x0020;
pub const COMM_PULSE: c_uint = 0x0030;
pub const COMM_1_WIRE_RESET: c_uint = 0x0042;
pub const COMM_BYTE_IO: c_uint = 0x0052;
pub const COMM_MATCH_ACCESS: c_uint = 0x0064;
pub const COMM_BLOCK_IO: c_uint = 0x0074;
pub const COMM_READ_STRAIGHT: c_uint = 0x0080;
pub const COMM_DO_RELEASE: c_uint = 0x6092;
pub const COMM_SET_PATH: c_uint = 0x00A2;
pub const COMM_WRITE_SRAM_PAGE: c_uint = 0x00B2;
pub const COMM_WRITE_EPROM: c_uint = 0x00C4;
pub const COMM_READ_CRC_PROT_PAGE: c_uint = 0x00D4;
pub const COMM_READ_REDIRECT_PAGE_CRC: c_uint = 0x21E4;
pub const COMM_SEARCH_ACCESS: c_uint = 0x00F4;
// Communication command bits
pub const COMM_TYPE: c_uint = 0x0008;
pub const COMM_SE: c_uint = 0x0008;
pub const COMM_D: c_uint = 0x0008;
pub const COMM_Z: c_uint = 0x0008;
pub const COMM_CH: c_uint = 0x0008;
pub const COMM_SM: c_uint = 0x0008;
pub const COMM_R: c_uint = 0x0008;
pub const COMM_IM: c_uint = 0x0001;
pub const COMM_PS: c_uint = 0x4000;
pub const COMM_PST: c_uint = 0x4000;
pub const COMM_CIB: c_uint = 0x4000;
pub const COMM_RTS: c_uint = 0x4000;
pub const COMM_DT: c_uint = 0x2000;
pub const COMM_SPU: c_uint = 0x1000;
pub const COMM_F: c_uint = 0x0800;
pub const COMM_NTF: c_uint = 0x0400;
pub const COMM_ICP: c_uint = 0x0200;
pub const COMM_RST: c_uint = 0x0100;
pub const PULSE_PROG: c_uint = 0x01;
pub const PULSE_SPUE: c_uint = 0x02;
pub const BRANCH_MAIN: c_uint = 0xCC;
pub const BRANCH_AUX: c_uint = 0x33;
// Status flags
pub const ST_SPUA: c_uint = 0x01  /* Strong Pull-up is active */;
pub const ST_PRGA: c_uint = 0x02  /* 12V programming pulse is being generated */;
pub const ST_12VP: c_uint = 0x04  /* external 12V programming voltage is present */;
pub const ST_PMOD: c_uint = 0x08  /* DS2490 powered from USB and external sources */;
pub const ST_HALT: c_uint = 0x10  /* DS2490 is currently halted */;
pub const ST_IDLE: c_uint = 0x20  /* DS2490 is currently idle */;
pub const ST_EPOF: c_uint = 0x80;
// Status transfer size, 16 bytes status, 16 byte result flags
pub const ST_SIZE: c_uint = 0x20;
// 1-wire data i/o fifo size, 128 bytes
pub const FIFO_SIZE: c_uint = 0x80;
// Result Register flags
pub const RR_DETECT: c_uint = 0xA5 /* New device detected */;
pub const RR_NRS: c_uint = 0x01 /* Reset no presence or ... */;
pub const RR_SH: c_uint = 0x02 /* short on reset or set path */;
pub const RR_APP: c_uint = 0x04 /* alarming presence on reset */;
pub const RR_VPP: c_uint = 0x08 /* 12V expected not seen */;
pub const RR_CMP: c_uint = 0x10 /* compare error */;
pub const RR_CRC: c_uint = 0x20 /* CRC error detected */;
pub const RR_RDP: c_uint = 0x40 /* redirected page */;
pub const RR_EOS: c_uint = 0x80 /* end of search error */;
pub const SPEED_NORMAL: c_uint = 0x00;
pub const SPEED_FLEXIBLE: c_uint = 0x01;
pub const SPEED_OVERDRIVE: c_uint = 0x02;
pub const NUM_EP: c_int = 4;
pub const EP_CONTROL: c_int = 0;
pub const EP_STATUS: c_int = 1;
pub const EP_DATA_OUT: c_int = 2;
pub const EP_DATA_IN: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ds_device {
    pub ds_entry: list_head,
    pub udev: *mut usb_device,
    pub intf: *mut usb_interface,
    pub ep: [c_int; NUM_EP],
// Strong PullUp
// 0: pullup not active, else duration in milliseconds
//
    pub spu_sleep: c_int,
// spu_bit contains COMM_SPU or 0 depending on if the strong pullup
// should be active or not for writes.
//
    pub spu_bit: u16,
    pub st_buf: [u8; ST_SIZE],
    pub byte_buf: u8,
    pub master: w1_bus_master,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ds_status {
    pub enable: u8,
    pub speed: u8,
    pub pullup_dur: u8,
    pub ppuls_dur: u8,
    pub pulldown_slew: u8,
    pub write1_time: u8,
    pub write0_time: u8,
    pub reserved0: u8,
    pub status: u8,
    pub command0: u8,
    pub command1: u8,
    pub command_buffer_status: u8,
    pub data_out_buffer_status: u8,
    pub data_in_buffer_status: u8,
    pub reserved1: u8,
    pub reserved2: u8,
}

    static LIST_HEAD(ds_devices);
    static DEFINE_MUTEX(ds_mutex);
#[no_mangle]
unsafe extern "C" fn ds_send_control_cmd(dev: *mut ds_device, value: u16, index: u16) -> c_int {
    static int ds_send_control_cmd(struct ds_device *dev, u16 value, u16 index)
    {
    int err;
    err = usb_control_msg(dev.udev, usb_sndctrlpipe(dev.udev, dev.ep[EP_CONTROL]),
    CONTROL_CMD, VENDOR, value, index, core::ptr::null_mut(), 0, 1000);
    if (err < 0) {
    dev_err(&dev.udev.dev,
    "Failed to send command control message %x.%x: err=%d.\n",
    value, index, err);
    return err;
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ds_send_control_mode(dev: *mut ds_device, value: u16, index: u16) -> c_int {
    static int ds_send_control_mode(struct ds_device *dev, u16 value, u16 index)
    {
    int err;
    err = usb_control_msg(dev.udev, usb_sndctrlpipe(dev.udev, dev.ep[EP_CONTROL]),
    MODE_CMD, VENDOR, value, index, core::ptr::null_mut(), 0, 1000);
    if (err < 0) {
    dev_err(&dev.udev.dev,
    "Failed to send mode control message %x.%x: err=%d.\n",
    value, index, err);
    return err;
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ds_send_control(dev: *mut ds_device, value: u16, index: u16) -> c_int {
    static int ds_send_control(struct ds_device *dev, u16 value, u16 index)
    {
    int err;
    err = usb_control_msg(dev.udev, usb_sndctrlpipe(dev.udev, dev.ep[EP_CONTROL]),
    COMM_CMD, VENDOR, value, index, core::ptr::null_mut(), 0, 1000);
    if (err < 0) {
    dev_err(&dev.udev.dev,
    "Failed to send control message %x.%x: err=%d.\n",
    value, index, err);
    return err;
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ds_dump_status(ds_dev: *mut ds_device, buf: *mut c_uchar, count: c_int) {
    static void ds_dump_status(struct ds_device *ds_dev, unsigned char *buf, int count)
    {
    struct device *dev = &ds_dev.udev.dev;
    int i;
    dev_info(dev, "ep_status=0x%x, count=%d, status=%*phC",
    ds_dev.ep[EP_STATUS], count, count, buf);
    if (count >= 16) {
    dev_dbg(dev, "enable flag: 0x%02x", buf[0]);
    dev_dbg(dev, "1-wire speed: 0x%02x", buf[1]);
    dev_dbg(dev, "strong pullup duration: 0x%02x", buf[2]);
    dev_dbg(dev, "programming pulse duration: 0x%02x", buf[3]);
    dev_dbg(dev, "pulldown slew rate control: 0x%02x", buf[4]);
    dev_dbg(dev, "write-1 low time: 0x%02x", buf[5]);
    dev_dbg(dev, "data sample offset/write-0 recovery time: 0x%02x", buf[6]);
    dev_dbg(dev, "reserved (test register): 0x%02x", buf[7]);
    dev_dbg(dev, "device status flags: 0x%02x", buf[8]);
    dev_dbg(dev, "communication command byte 1: 0x%02x", buf[9]);
    dev_dbg(dev, "communication command byte 2: 0x%02x", buf[10]);
    dev_dbg(dev, "communication command buffer status: 0x%02x", buf[11]);
    dev_dbg(dev, "1-wire data output buffer status: 0x%02x", buf[12]);
    dev_dbg(dev, "1-wire data input buffer status: 0x%02x", buf[13]);
    dev_dbg(dev, "reserved: 0x%02x", buf[14]);
    dev_dbg(dev, "reserved: 0x%02x", buf[15]);
    }
    for (i = 16; i < count; ++i) {
    if (buf[i] == RR_DETECT) {
    dev_dbg(dev, "New device detect.\n");
    continue;
    }
    dev_dbg(dev, "Result Register Value: 0x%02x", buf[i]);
    if (buf[i] & RR_NRS)
    dev_dbg(dev, "NRS: Reset no presence or ...\n");
    if (buf[i] & RR_SH)
    dev_dbg(dev, "SH: short on reset or set path\n");
    if (buf[i] & RR_APP)
    dev_dbg(dev, "APP: alarming presence on reset\n");
    if (buf[i] & RR_VPP)
    dev_dbg(dev, "VPP: 12V expected not seen\n");
    if (buf[i] & RR_CMP)
    dev_dbg(dev, "CMP: compare error\n");
    if (buf[i] & RR_CRC)
    dev_dbg(dev, "CRC: CRC error detected\n");
    if (buf[i] & RR_RDP)
    dev_dbg(dev, "RDP: redirected page\n");
    if (buf[i] & RR_EOS)
    dev_dbg(dev, "EOS: end of search error\n");
    }
    }
#[no_mangle]
unsafe extern "C" fn ds_recv_status(dev: *mut ds_device, st: *mut ds_status) -> c_int {
    static int ds_recv_status(struct ds_device *dev, struct ds_status *st)
    {
    int count, err;
    if (st)
    memset(st, 0, sizeof(*st));
    count = 0;
    err = usb_interrupt_msg(dev.udev,
    usb_rcvintpipe(dev.udev,
    dev.ep[EP_STATUS]),
    dev.st_buf, sizeof(dev.st_buf),
    &count, 1000);
    if (err < 0) {
    dev_err(&dev.udev.dev,
    "Failed to read 1-wire data from 0x%x: err=%d.\n",
    dev.ep[EP_STATUS], err);
    return err;
    }
    if (st && count >= sizeof(*st))
    memcpy(st, dev.st_buf, sizeof(*st));
    return count;
    }
#[no_mangle]
unsafe extern "C" fn ds_reset_device(dev: *mut ds_device) {
    static void ds_reset_device(struct ds_device *dev)
    {
    ds_send_control_cmd(dev, CTL_RESET_DEVICE, 0);
// Always allow strong pullup which allow individual writes to use
// the strong pullup.
//
    if (ds_send_control_mode(dev, MOD_PULSE_EN, PULSE_SPUE))
    dev_err(&dev.udev.dev,
    "%s: Error allowing strong pullup\n", __func__);
// Chip strong pullup time was cleared.
    if (dev.spu_sleep) {
// lower 4 bits are 0, see ds_set_pullup
    let mut del: u8 = dev.spu_sleep>>4;
    if (ds_send_control(dev, COMM_SET_DURATION | COMM_IM, del))
    dev_err(&dev.udev.dev,
    "%s: Error setting duration\n", __func__);
    }
    }
#[no_mangle]
unsafe extern "C" fn ds_recv_data(dev: *mut ds_device, buf: *mut c_uchar, size: c_int) -> c_int {
    static int ds_recv_data(struct ds_device *dev, unsigned char *buf, int size)
    {
    int count, err;
// Careful on size.  If size is less than what is available in
// the input buffer, the device fails the bulk transfer and
// clears the input buffer.  It could read the maximum size of
// the data buffer, but then do you return the first, last, or
// some set of the middle size bytes?  As long as the rest of
// the code is correct there will be size bytes waiting.  A
// call to ds_wait_status will wait until the device is idle
// and any data to be received would have been available.
//
    count = 0;
    err = usb_bulk_msg(dev.udev, usb_rcvbulkpipe(dev.udev, dev.ep[EP_DATA_IN]),
    buf, size, &count, 1000);
    if (err < 0) {
    int recv_len;
    dev_info(&dev.udev.dev, "Clearing ep0x%x.\n", dev.ep[EP_DATA_IN]);
    usb_clear_halt(dev.udev, usb_rcvbulkpipe(dev.udev, dev.ep[EP_DATA_IN]));
// status might tell us why endpoint is stuck?
    recv_len = ds_recv_status(dev, core::ptr::null_mut());
    if (recv_len >= 0)
    ds_dump_status(dev, dev.st_buf, recv_len);
    return err;
    }

    {
    int i;
    printk("%s: count=%d: ", __func__, count);
    for (i = 0; i < count; ++i)
    printk("%02x ", buf[i]);
    printk("\n");
    }

    return count;
    }
#[no_mangle]
unsafe extern "C" fn ds_send_data(dev: *mut ds_device, buf: *mut c_uchar, len: c_int) -> c_int {
    static int ds_send_data(struct ds_device *dev, unsigned char *buf, int len)
    {
    int count, err;
    count = 0;
    err = usb_bulk_msg(dev.udev, usb_sndbulkpipe(dev.udev, dev.ep[EP_DATA_OUT]), buf, len, &count, 1000);
    if (err < 0) {
    dev_err(&dev.udev.dev, "Failed to write 1-wire data to ep0x%x: "
    "err=%d.\n", dev.ep[EP_DATA_OUT], err);
    return err;
    }
    return err;
    }

#[no_mangle]
pub unsafe extern "C" fn ds_stop_pulse(dev: *mut ds_device, limit: c_int) -> c_int {
    int ds_stop_pulse(struct ds_device *dev, int limit)
    {
    struct ds_status st;
    let mut count: c_int = 0, err = 0;
    do {
    err = ds_send_control(dev, CTL_HALT_EXE_IDLE, 0);
    if (err)
    break;
    err = ds_send_control(dev, CTL_RESUME_EXE, 0);
    if (err)
    break;
    err = ds_recv_status(dev, &st);
    if (err)
    break;
    if ((st.status & ST_SPUA) == 0) {
    err = ds_send_control_mode(dev, MOD_PULSE_EN, 0);
    if (err)
    break;
    }
    } while (++count < limit);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn ds_detect(dev: *mut ds_device, st: *mut ds_status) -> c_int {
    int ds_detect(struct ds_device *dev, struct ds_status *st)
    {
    int err;
    err = ds_send_control_cmd(dev, CTL_RESET_DEVICE, 0);
    if (err)
    return err;
    err = ds_send_control(dev, COMM_SET_DURATION | COMM_IM, 0);
    if (err)
    return err;
    err = ds_send_control(dev, COMM_SET_DURATION | COMM_IM | COMM_TYPE, 0x40);
    if (err)
    return err;
    err = ds_send_control_mode(dev, MOD_PULSE_EN, PULSE_PROG);
    if (err)
    return err;
    err = ds_dump_status(dev, st);
    return err;
    }

#[no_mangle]
unsafe extern "C" fn ds_wait_status(dev: *mut ds_device, st: *mut ds_status) -> c_int {
    static int ds_wait_status(struct ds_device *dev, struct ds_status *st)
    {
    int err, count = 0;
    do {
    st.status = 0;
    err = ds_recv_status(dev, st);

    if (err >= 0) {
    int i;
    printk("0x%x: count=%d, status: ", dev.ep[EP_STATUS], err);
    for (i = 0; i < err; ++i)
    printk("%02x ", dev.st_buf[i]);
    printk("\n");
    }

    } while (!(st.status & ST_IDLE) && !(err < 0) && ++count < 100);
    if (err >= 16 && st.status & ST_EPOF) {
    dev_info(&dev.udev.dev, "Resetting device after ST_EPOF.\n");
    ds_reset_device(dev);
// Always dump the device status.
    count = 101;
    }
// Dump the status for errors or if there is extended return data.
// The extended status includes new device detection (maybe someone
// can do something with it).
//
    if (err > 16 || count >= 100 || err < 0)
    ds_dump_status(dev, dev.st_buf, err);
// Extended data isn't an error.  Well, a short is, but the dump
// would have already told the user that and we can't do anything
// about it in software anyway.
//
    if (count >= 100 || err < 0)
    return -1;
    else
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ds_reset(dev: *mut ds_device) -> c_int {
    static int ds_reset(struct ds_device *dev)
    {
    int err;
// Other potentionally interesting flags for reset.
//
// COMM_NTF: Return result register feedback.  This could be used to
// detect some conditions such as short, alarming presence, or
// detect if a new device was detected.
//
// COMM_SE which allows SPEED_NORMAL, SPEED_FLEXIBLE, SPEED_OVERDRIVE:
// Select the data transfer rate.
//
    err = ds_send_control(dev, COMM_1_WIRE_RESET | COMM_IM, SPEED_NORMAL);
    if (err)
    return err;
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn ds_set_speed(dev: *mut ds_device, speed: c_int) -> c_int {
    static int ds_set_speed(struct ds_device *dev, int speed)
    {
    int err;
    if (speed != SPEED_NORMAL && speed != SPEED_FLEXIBLE && speed != SPEED_OVERDRIVE)
    return -EINVAL;
    if (speed != SPEED_OVERDRIVE)
    speed = SPEED_FLEXIBLE;
    speed &= 0xff;
    err = ds_send_control_mode(dev, MOD_1WIRE_SPEED, speed);
    if (err)
    return err;
    return err;
    }

#[no_mangle]
unsafe extern "C" fn ds_set_pullup(dev: *mut ds_device, delay: c_int) -> c_int {
    static int ds_set_pullup(struct ds_device *dev, int delay)
    {
    let mut err: c_int = 0;
    let mut del: u8 = 1 + (u8)(delay >> 4);
// Just storing delay would not get the trunication and roundup.
    let mut ms: c_int = del<<4;
// Enable spu_bit if a delay is set.
    dev.spu_bit = delay ? COMM_SPU : 0;
// If delay is zero, it has already been disabled, if the time is
// the same as the hardware was last programmed to, there is also
// nothing more to do.  Compare with the recalculated value ms
// rather than del or delay which can have a different value.
//
    if (delay == 0 || ms == dev.spu_sleep)
    return err;
    err = ds_send_control(dev, COMM_SET_DURATION | COMM_IM, del);
    if (err)
    return err;
    dev.spu_sleep = ms;
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ds_touch_bit(dev: *mut ds_device, bit: u8, tbit: *mut u8) -> c_int {
    static int ds_touch_bit(struct ds_device *dev, u8 bit, u8 *tbit)
    {
    int err;
    struct ds_status st;
    err = ds_send_control(dev, COMM_BIT_IO | COMM_IM | (bit ? COMM_D : 0),
    0);
    if (err)
    return err;
    ds_wait_status(dev, &st);
    err = ds_recv_data(dev, tbit, sizeof(*tbit));
    if (err < 0)
    return err;
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn ds_write_bit(dev: *mut ds_device, bit: u8) -> c_int {
    static int ds_write_bit(struct ds_device *dev, u8 bit)
    {
    int err;
    struct ds_status st;
// Set COMM_ICP to write without a readback.  Note, this will
// produce one time slot, a down followed by an up with COMM_D
// only determing the timing.
//
    err = ds_send_control(dev, COMM_BIT_IO | COMM_IM | COMM_ICP |
    (bit ? COMM_D : 0), 0);
    if (err)
    return err;
    ds_wait_status(dev, &st);
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn ds_write_byte(dev: *mut ds_device, byte: u8) -> c_int {
    static int ds_write_byte(struct ds_device *dev, u8 byte)
    {
    int err;
    struct ds_status st;
    err = ds_send_control(dev, COMM_BYTE_IO | COMM_IM | dev.spu_bit, byte);
    if (err)
    return err;
    if (dev.spu_bit)
    msleep(dev.spu_sleep);
    err = ds_wait_status(dev, &st);
    if (err)
    return err;
    err = ds_recv_data(dev, &dev.byte_buf, 1);
    if (err < 0)
    return err;
    return !(byte == dev.byte_buf);
    }
#[no_mangle]
unsafe extern "C" fn ds_read_byte(dev: *mut ds_device, byte: *mut u8) -> c_int {
    static int ds_read_byte(struct ds_device *dev, u8 *byte)
    {
    int err;
    struct ds_status st;
    err = ds_send_control(dev, COMM_BYTE_IO | COMM_IM, 0xff);
    if (err)
    return err;
    ds_wait_status(dev, &st);
    err = ds_recv_data(dev, byte, sizeof(*byte));
    if (err < 0)
    return err;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn read_block_chunk(dev: *mut ds_device, buf: *mut u8, len: c_int) -> c_int {
    static int read_block_chunk(struct ds_device *dev, u8 *buf, int len)
    {
    struct ds_status st;
    int err;
    memset(buf, 0xFF, len);
    err = ds_send_data(dev, buf, len);
    if (err < 0)
    return err;
    err = ds_send_control(dev, COMM_BLOCK_IO | COMM_IM, len);
    if (err)
    return err;
    ds_wait_status(dev, &st);
    memset(buf, 0x00, len);
    err = ds_recv_data(dev, buf, len);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ds_read_block(dev: *mut ds_device, buf: *mut u8, len: c_int) -> c_int {
    static int ds_read_block(struct ds_device *dev, u8 *buf, int len)
    {
    int err, to_read, rem = len;
    if (len > 64 * 1024)
    return -E2BIG;
    do {
    to_read = rem <= FIFO_SIZE ? rem : FIFO_SIZE;
    err = read_block_chunk(dev, &buf[len - rem], to_read);
    if (err < 0)
    return err;
    rem -= to_read;
    } while (rem);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ds_write_block(dev: *mut ds_device, buf: *mut u8, len: c_int) -> c_int {
    static int ds_write_block(struct ds_device *dev, u8 *buf, int len)
    {
    int err;
    struct ds_status st;
    err = ds_send_data(dev, buf, len);
    if (err < 0)
    return err;
    err = ds_send_control(dev, COMM_BLOCK_IO | COMM_IM | dev.spu_bit, len);
    if (err)
    return err;
    if (dev.spu_bit)
    msleep(dev.spu_sleep);
    ds_wait_status(dev, &st);
    err = ds_recv_data(dev, buf, len);
    if (err < 0)
    return err;
    return !(err == len);
    }
    static void ds9490r_search(void *data, struct w1_master *master,
    u8 search_type, w1_slave_found_callback callback)
    {
// When starting with an existing id, the first id returned will
// be that device (if it is still on the bus most likely).
//
// If the number of devices found is less than or equal to the
// search_limit, that number of IDs will be returned.  If there are
// more, search_limit IDs will be returned followed by a non-zero
// discrepency value.
//
    struct ds_device *dev = data;
    int err;
    u16 value, index;
    struct ds_status st;
    int search_limit;
    let mut found: c_int = 0;
    int i;
// DS18b20 spec, 13.16 ms per device, 75 per second, sleep for
// discovering 8 devices (1 bulk transfer and 1/2 FIFO size) at a time.
//
    let mut jtime: c_ulong = msecs_to_jiffies(1000*8/75);
// FIFO 128 bytes, bulk packet size 64, read a multiple of the
// packet size.
//
    let mut bufsize: usize = 2 * 64;
    u64 *buf, *found_ids;
    buf = kmalloc(bufsize, GFP_KERNEL);
    if (!buf)
    return;
//
// We are holding the bus mutex during the scan, but adding devices via the
// callback needs the bus to be unlocked. So we queue up found ids here.
//
    found_ids = kmalloc_array(master.max_slave_count, sizeof(u64), GFP_KERNEL);
    if (!found_ids) {
    kfree(buf);
    return;
    }
    mutex_lock(&master.bus_mutex);
// address to start searching at
    if (ds_send_data(dev, (u8 *)&master.search_id, 8) < 0)
    goto search_out;
    master.search_id = 0;
    value = COMM_SEARCH_ACCESS | COMM_IM | COMM_RST | COMM_SM | COMM_F |
    COMM_RTS;
    search_limit = master.max_slave_count;
    if (search_limit > 255)
    search_limit = 0;
    index = search_type | (search_limit << 8);
    if (ds_send_control(dev, value, index) < 0)
    goto search_out;
    do {
    schedule_timeout(jtime);
    err = ds_recv_status(dev, &st);
    if (err < 0 || err < sizeof(st))
    break;
    if (st.data_in_buffer_status) {
//
// Bulk in can receive partial ids, but when it does
// they fail crc and will be discarded anyway.
// That has only been seen when status in buffer
// is 0 and bulk is read anyway, so don't read
// bulk without first checking if status says there
// is data to read.
//
    err = ds_recv_data(dev, (u8 *)buf, bufsize);
    if (err < 0)
    break;
    for (i = 0; i < err/8; ++i) {
    found_ids[found++] = buf[i];
//
// can't know if there will be a discrepancy
// value after until the next id
//
    if (found == search_limit) {
    master.search_id = buf[i];
    break;
    }
    }
    }
    if (test_bit(W1_ABORT_SEARCH, &master.flags))
    break;
    } while (!(st.status & (ST_IDLE | ST_HALT)));
// only continue the search if some weren't found
    if (found <= search_limit) {
    master.search_id = 0;
    } else if (!test_bit(W1_WARN_MAX_COUNT, &master.flags)) {
//
// Only max_slave_count will be scanned in a search,
// but it will start where it left off next search
// until all ids are identified and then it will start
// over.  A continued search will report the previous
// last id as the first id (provided it is still on the
// bus).
//
    dev_info(&dev.udev.dev, "%s: max_slave_count %d reached, "
    "will continue next search.\n", __func__,
    master.max_slave_count);
    set_bit(W1_WARN_MAX_COUNT, &master.flags);
    }
    search_out:
    mutex_unlock(&master.bus_mutex);
    kfree(buf);
    for (i = 0; i < found; i++) /* run callback for all queued up IDs */
    callback(master, found_ids[i]);
    kfree(found_ids);
    }

//
// FIXME: if this disabled code is ever used in the future all ds_send_data()
// calls must be changed to use a DMAable buffer.
//
#[no_mangle]
unsafe extern "C" fn ds_match_access(dev: *mut ds_device, init: u64) -> c_int {
    static int ds_match_access(struct ds_device *dev, u64 init)
    {
    int err;
    struct ds_status st;
    err = ds_send_data(dev, (unsigned char *)&init, sizeof(init));
    if (err)
    return err;
    ds_wait_status(dev, &st);
    err = ds_send_control(dev, COMM_MATCH_ACCESS | COMM_IM | COMM_RST, 0x0055);
    if (err)
    return err;
    ds_wait_status(dev, &st);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ds_set_path(dev: *mut ds_device, init: u64) -> c_int {
    static int ds_set_path(struct ds_device *dev, u64 init)
    {
    int err;
    struct ds_status st;
    u8 buf[9];
    memcpy(buf, &init, 8);
    buf[8] = BRANCH_MAIN;
    err = ds_send_data(dev, buf, sizeof(buf));
    if (err)
    return err;
    ds_wait_status(dev, &st);
    err = ds_send_control(dev, COMM_SET_PATH | COMM_IM | COMM_RST, 0);
    if (err)
    return err;
    ds_wait_status(dev, &st);
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn ds9490r_touch_bit(data: *mut c_void, bit: u8) -> u8 {
    static u8 ds9490r_touch_bit(void *data, u8 bit)
    {
    struct ds_device *dev = data;
    if (ds_touch_bit(dev, bit, &dev.byte_buf))
    return 0;
    return dev.byte_buf;
    }

#[no_mangle]
unsafe extern "C" fn ds9490r_write_bit(data: *mut c_void, bit: u8) {
    static void ds9490r_write_bit(void *data, u8 bit)
    {
    struct ds_device *dev = data;
    ds_write_bit(dev, bit);
    }
#[no_mangle]
unsafe extern "C" fn ds9490r_read_bit(data: *mut c_void) -> u8 {
    static u8 ds9490r_read_bit(void *data)
    {
    struct ds_device *dev = data;
    int err;
    err = ds_touch_bit(dev, 1, &dev.byte_buf);
    if (err)
    return 0;
    return dev.byte_buf & 1;
    }

#[no_mangle]
unsafe extern "C" fn ds9490r_write_byte(data: *mut c_void, byte: u8) {
    static void ds9490r_write_byte(void *data, u8 byte)
    {
    struct ds_device *dev = data;
    ds_write_byte(dev, byte);
    }
#[no_mangle]
unsafe extern "C" fn ds9490r_read_byte(data: *mut c_void) -> u8 {
    static u8 ds9490r_read_byte(void *data)
    {
    struct ds_device *dev = data;
    int err;
    err = ds_read_byte(dev, &dev.byte_buf);
    if (err)
    return 0;
    return dev.byte_buf;
    }
#[no_mangle]
unsafe extern "C" fn ds9490r_write_block(data: *mut c_void, buf: *const u8, len: c_int) {
    static void ds9490r_write_block(void *data, const u8 *buf, int len)
    {
    struct ds_device *dev = data;
    u8 *tbuf;
    if (len <= 0)
    return;
    tbuf = kmemdup(buf, len, GFP_KERNEL);
    if (!tbuf)
    return;
    ds_write_block(dev, tbuf, len);
    kfree(tbuf);
    }
#[no_mangle]
unsafe extern "C" fn ds9490r_read_block(data: *mut c_void, buf: *mut u8, len: c_int) -> u8 {
    static u8 ds9490r_read_block(void *data, u8 *buf, int len)
    {
    struct ds_device *dev = data;
    int err;
    u8 *tbuf;
    if (len <= 0)
    return 0;
    tbuf = kmalloc(len, GFP_KERNEL);
    if (!tbuf)
    return 0;
    err = ds_read_block(dev, tbuf, len);
    if (err >= 0)
    memcpy(buf, tbuf, len);
    kfree(tbuf);
    return err >= 0 ? len : 0;
    }
#[no_mangle]
unsafe extern "C" fn ds9490r_reset(data: *mut c_void) -> u8 {
    static u8 ds9490r_reset(void *data)
    {
    struct ds_device *dev = data;
    int err;
    err = ds_reset(dev);
    if (err)
    return 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ds9490r_set_pullup(data: *mut c_void, delay: c_int) -> u8 {
    static u8 ds9490r_set_pullup(void *data, int delay)
    {
    struct ds_device *dev = data;
    if (ds_set_pullup(dev, delay))
    return 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ds_w1_init(dev: *mut ds_device) -> c_int {
    static int ds_w1_init(struct ds_device *dev)
    {
    memset(&dev.master, 0, sizeof(struct w1_bus_master));
// Reset the device as it can be in a bad state.
// This is necessary because a block write will wait for data
// to be placed in the output buffer and block any later
// commands which will keep accumulating and the device will
// not be idle.  Another case is removing the ds2490 module
// while a bus search is in progress, somehow a few commands
// get through, but the input transfers fail leaving data in
// the input buffer.  This will cause the next read to fail
// see the note in ds_recv_data.
//
    ds_reset_device(dev);
    dev.master.data	= dev;
    dev.master.touch_bit	= &ds9490r_touch_bit;
// read_bit and write_bit in w1_bus_master are expected to set and
// sample the line level.  For write_bit that means it is expected to
// set it to that value and leave it there.  ds2490 only supports an
// individual time slot at the lowest level.  The requirement from
// pulling the bus state down to reading the state is 15us, something
// that isn't realistic on the USB bus anyway.
    dev.master.read_bit	= &ds9490r_read_bit;
    dev.master.write_bit	= &ds9490r_write_bit;
//
    dev.master.read_byte	= &ds9490r_read_byte;
    dev.master.write_byte	= &ds9490r_write_byte;
    dev.master.read_block	= &ds9490r_read_block;
    dev.master.write_block	= &ds9490r_write_block;
    dev.master.reset_bus	= &ds9490r_reset;
    dev.master.set_pullup	= &ds9490r_set_pullup;
    dev.master.search	= &ds9490r_search;
    return w1_add_master_device(&dev.master);
    }
#[no_mangle]
unsafe extern "C" fn ds_w1_fini(dev: *mut ds_device) {
    static void ds_w1_fini(struct ds_device *dev)
    {
    w1_remove_master_device(&dev.master);
    }
    static int ds_probe(struct usb_interface *intf,
    const struct usb_device_id *udev_id)
    {
    struct usb_device *udev = interface_to_usbdev(intf);
    struct usb_endpoint_descriptor *endpoint;
    struct usb_host_interface *iface_desc;
    struct ds_device *dev;
    int i, err, alt;
    dev = kzalloc_obj(struct ds_device);
    if (!dev)
    return -ENOMEM;
    dev.udev = udev;
    memset(dev.ep, 0, sizeof(dev.ep));
    usb_set_intfdata(intf, dev);
    err = usb_reset_configuration(dev.udev);
    if (err) {
    dev_err(&dev.udev.dev,
    "Failed to reset configuration: err=%d.\n", err);
    goto err_out_clear;
    }
// alternative 3, 1ms interrupt (greatly speeds search), 64 byte bulk
    alt = 3;
    err = usb_set_interface(dev.udev,
    intf.cur_altsetting.desc.bInterfaceNumber, alt);
    if (err) {
    dev_err(&dev.udev.dev, "Failed to set alternative setting %d "
    "for %d interface: err=%d.\n", alt,
    intf.cur_altsetting.desc.bInterfaceNumber, err);
    goto err_out_clear;
    }
    iface_desc = intf.cur_altsetting;
    if (iface_desc.desc.bNumEndpoints != NUM_EP-1) {
    dev_err(&dev.udev.dev, "Num endpoints=%d. It is not DS9490R.\n",
    iface_desc.desc.bNumEndpoints);
    err = -EINVAL;
    goto err_out_clear;
    }
//
// This loop doesn'd show control 0 endpoint,
// so we will fill only 1-3 endpoints entry.
//
    for (i = 0; i < iface_desc.desc.bNumEndpoints; ++i) {
    endpoint = &iface_desc.endpoint[i].desc;
    dev.ep[i+1] = endpoint.bEndpointAddress;

    printk("%d: addr=%x, size=%d, dir=%s, type=%x\n",
    i, endpoint.bEndpointAddress, le16_to_cpu(endpoint.wMaxPacketSize),
    (endpoint.bEndpointAddress & USB_DIR_IN)?"IN":"OUT",
    endpoint.bmAttributes & USB_ENDPOINT_XFERTYPE_MASK);

    }
    err = ds_w1_init(dev);
    if (err)
    goto err_out_clear;
    mutex_lock(&ds_mutex);
    list_add_tail(&dev.ds_entry, &ds_devices);
    mutex_unlock(&ds_mutex);
    return 0;
    err_out_clear:
    usb_set_intfdata(intf, core::ptr::null_mut());
    kfree(dev);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ds_disconnect(intf: *mut usb_interface) {
    static void ds_disconnect(struct usb_interface *intf)
    {
    struct ds_device *dev;
    dev = usb_get_intfdata(intf);
    if (!dev)
    return;
    mutex_lock(&ds_mutex);
    list_del(&dev.ds_entry);
    mutex_unlock(&ds_mutex);
    ds_w1_fini(dev);
    usb_set_intfdata(intf, core::ptr::null_mut());
    kfree(dev);
    }
    static const struct usb_device_id ds_id_table[] = {
    { USB_DEVICE(0x04fa, 0x2490) },
    { },
    };
    MODULE_DEVICE_TABLE(usb, ds_id_table);
    static struct usb_driver ds_driver = {
    .name =		"DS9490R",
    .probe =	ds_probe,
    .disconnect =	ds_disconnect,
    .id_table =	ds_id_table,
    };
    module_usb_driver(ds_driver);
    MODULE_AUTHOR("Evgeniy Polyakov <zbr@ioremap.net>");
    MODULE_DESCRIPTION("DS2490 USB <. W1 bus master driver (DS9490*)");
    MODULE_LICENSE("GPL");
