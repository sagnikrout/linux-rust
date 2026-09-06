//! Automatically rewritten from C to Rust
//! Source: drivers/media/rc/redrat3.c
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
// USB RedRat3 IR Transceiver rc-core driver
//
// Copyright (c) 2011 by Jarod Wilson <jarod@redhat.com>
// based heavily on the work of Stephen Cox, with additional
// help from RedRat Ltd.
//
// This driver began life based on an old version of the first-generation
// lirc_mceusb driver from the lirc 0.7.2 distribution. It was then
// significantly rewritten by Stephen Cox with the aid of RedRat Ltd's
// Chris Dodge.
//
// The driver was then ported to rc-core and significantly rewritten again,
// by Jarod, using the in-kernel mceusb driver as a guide, after an initial
// port effort was started by Stephen.
//
// TODO LIST:
// - fix lirc not showing repeats properly
// --
//
// The RedRat3 is a USB transceiver with both send & receive,
// with 2 separate sensors available for receive to enable
// both good long range reception for general use, and good
// short range reception when required for learning a signal.
//
// http://www.redrat.co.uk
//
// It uses its own little protocol to communicate, the required
// parts of which are embedded within this driver.
// --
//

// Driver Information

// bulk data transfer types
pub const RR3_ERROR: c_uint = 0x01;
pub const RR3_MOD_SIGNAL_IN: c_uint = 0x20;
pub const RR3_MOD_SIGNAL_OUT: c_uint = 0x21;
// Get the RR firmware version
pub const RR3_FW_VERSION: c_uint = 0xb1;
pub const RR3_FW_VERSION_LEN: c_int = 64;
// Send encoded signal bulk-sent earlier
pub const RR3_TX_SEND_SIGNAL: c_uint = 0xb3;
pub const RR3_SET_IR_PARAM: c_uint = 0xb7;
pub const RR3_GET_IR_PARAM: c_uint = 0xb8;
// Blink the red LED on the device
pub const RR3_BLINK_LED: c_uint = 0xb9;
// Read serial number of device
pub const RR3_READ_SER_NO: c_uint = 0xba;
pub const RR3_SER_NO_LEN: c_int = 4;
// Start capture with the RC receiver
pub const RR3_RC_DET_ENABLE: c_uint = 0xbb;
// Stop capture with the RC receiver
pub const RR3_RC_DET_DISABLE: c_uint = 0xbc;
// Start capture with the wideband receiver
pub const RR3_MODSIG_CAPTURE: c_uint = 0xb2;
// Return the status of RC detector capture
pub const RR3_RC_DET_STATUS: c_uint = 0xbd;
// Reset redrat
pub const RR3_RESET: c_uint = 0xa0;
// Max number of lengths in the signal.
pub const RR3_IR_IO_MAX_LENGTHS: c_uint = 0x01;
// Periods to measure mod. freq.
pub const RR3_IR_IO_PERIODS_MF: c_uint = 0x02;
// Size of memory for main signal data
pub const RR3_IR_IO_SIG_MEM_SIZE: c_uint = 0x03;
// Delta value when measuring lengths
pub const RR3_IR_IO_LENGTH_FUZZ: c_uint = 0x04;
// Timeout for end of signal detection
pub const RR3_IR_IO_SIG_TIMEOUT: c_uint = 0x05;
// Minimum value for pause recognition.
pub const RR3_IR_IO_MIN_PAUSE: c_uint = 0x06;
// Clock freq. of EZ-USB chip
pub const RR3_CLK: c_int = 24000000;
// Clock periods per timer count
pub const RR3_CLK_PER_COUNT: c_int = 12;
// (RR3_CLK / RR3_CLK_PER_COUNT)
pub const RR3_CLK_CONV_FACTOR: c_int = 2000000;
// USB bulk-in wideband IR data endpoint address
pub const RR3_WIDE_IN_EP_ADDR: c_uint = 0x81;
// USB bulk-in narrowband IR data endpoint address
pub const RR3_NARROW_IN_EP_ADDR: c_uint = 0x82;
// Size of the fixed-length portion of the signal
pub const RR3_DRIVER_MAXLENS: c_int = 255;
pub const RR3_MAX_SIG_SIZE: c_int = 512;
pub const RR3_TIME_UNIT: c_int = 50;
pub const RR3_END_OF_SIGNAL: c_uint = 0x7f;
pub const RR3_TX_TRAILER_LEN: c_int = 2;
pub const RR3_RX_MIN_TIMEOUT: c_int = 5;
pub const RR3_RX_MAX_TIMEOUT: c_int = 2000;
// The 8051's CPUCS Register address
pub const RR3_CPUCS_REG_ADDR: c_uint = 0x7f92;
pub const USB_RR3USB_VENDOR_ID: c_uint = 0x112a;
pub const USB_RR3USB_PRODUCT_ID: c_uint = 0x0001;
pub const USB_RR3IIUSB_PRODUCT_ID: c_uint = 0x0005;
//
// The redrat3 encodes an IR signal as set of different lengths and a set
// of indices into those lengths. This sets how much two lengths must
// differ before they are considered distinct, the value is specified
// in microseconds.
// Default 5, value 0 to 127.
//
    let mut length_fuzz: static int = 5;
    module_param(length_fuzz, uint, 0644);
    MODULE_PARM_DESC(length_fuzz, "Length Fuzz (0-127)");
//
// When receiving a continuous ir stream (for example when a user is
// holding a button down on a remote), this specifies the minimum size
// of a space when the redrat3 sends a irdata packet to the host. Specified
// in milliseconds. Default value 18ms.
// The value can be between 2 and 30 inclusive.
//
    let mut minimum_pause: static int = 18;
    module_param(minimum_pause, uint, 0644);
    MODULE_PARM_DESC(minimum_pause, "Minimum Pause in ms (2-30)");
//
// The carrier frequency is measured during the first pulse of the IR
// signal. The larger the number of periods used To measure, the more
// accurate the result is likely to be, however some signals have short
// initial pulses, so in some case it may be necessary to reduce this value.
// Default 8, value 1 to 255.
//
    let mut periods_measure_carrier: static int = 8;
    module_param(periods_measure_carrier, uint, 0644);
    MODULE_PARM_DESC(periods_measure_carrier, "Number of Periods to Measure Carrier (1-255)");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct redrat3_header {
    pub length: __be16,
    pub transfer_type: __be16,
    pub __packed: },
// sending and receiving irdata
#[repr(C)]
#[derive(Copy, Clone)]
pub struct redrat3_irdata {
    pub header: redrat3_header,
    pub pause: __be32,
    pub mod_freq_count: __be16,
    pub num_periods: __be16,
    pub max_lengths: __u8,
    pub no_lengths: __u8,
    pub max_sig_size: __be16,
    pub sig_size: __be16,
    pub no_repeats: __u8,
    pub /: *mut *mut __be16 lens[RR3_DRIVER_MAXLENS]; / not aligned,
    pub sigdata: [__u8; RR3_MAX_SIG_SIZE],
    pub __packed: },
// firmware errors
#[repr(C)]
#[derive(Copy, Clone)]
pub struct redrat3_error {
    pub header: redrat3_header,
    pub fw_error: __be16,
    pub __packed: },
// table of devices that work with this driver
    static const struct usb_device_id redrat3_dev_table[] = {
// Original version of the RedRat3
    {USB_DEVICE(USB_RR3USB_VENDOR_ID, USB_RR3USB_PRODUCT_ID)},
// Second Version/release of the RedRat3 - RetRat3-II
    {USB_DEVICE(USB_RR3USB_VENDOR_ID, USB_RR3IIUSB_PRODUCT_ID)},
    {}			/* Terminating entry */
}

// Structure to hold all of our device specific stuff
#[repr(C)]
#[derive(Copy, Clone)]
pub struct redrat3_dev {
// core device bits
    pub rc: *mut rc_dev,
    pub dev: *mut device,
// led control
    pub led: led_classdev,
    pub flash: core::sync::atomic::AtomicI32,
    pub flash_control: usb_ctrlrequest,
    pub flash_urb: *mut urb,
    pub flash_in_buf: u8,
// learning
    pub wideband: bool,
    pub learn_control: usb_ctrlrequest,
    pub learn_urb: *mut urb,
    pub learn_buf: u8,
// save off the usb device pointer
    pub udev: *mut usb_device,
// the receive endpoint
    pub ep_narrow: *mut usb_endpoint_descriptor,
// the buffer to receive data
    pub bulk_in_buf: *mut c_void,
// urb used to read ir data
    pub narrow_urb: *mut urb,
    pub wide_urb: *mut urb,
// the send endpoint
    pub ep_out: *mut usb_endpoint_descriptor,
// usb dma
    pub dma_in: dma_addr_t,
// Is the device currently transmitting?
    pub transmitting: bool,
// store for current packet
    pub irdata: redrat3_irdata,
    pub bytes_read: u16,
    pub carrier: u32,
    pub name: [c_char; 64],
    pub phys: [c_char; 64],
}

#[no_mangle]
unsafe extern "C" fn redrat3_dump_fw_error(rr3: *mut redrat3_dev, code: c_int) {
    static void redrat3_dump_fw_error(struct redrat3_dev *rr3, int code)
    {
    if (!rr3.transmitting && (code != 0x40))
    dev_info(rr3.dev, "fw error code 0x%02x: ", code);
    switch (code) {
    case 0x00:
    pr_cont("No Error\n");
    break;
// Codes 0x20 through 0x2f are IR Firmware Errors
    case 0x20:
    pr_cont("Initial signal pulse not long enough to measure carrier frequency\n");
    break;
    case 0x21:
    pr_cont("Not enough length values allocated for signal\n");
    break;
    case 0x22:
    pr_cont("Not enough memory allocated for signal data\n");
    break;
    case 0x23:
    pr_cont("Too many signal repeats\n");
    break;
    case 0x28:
    pr_cont("Insufficient memory available for IR signal data memory allocation\n");
    break;
    case 0x29:
    pr_cont("Insufficient memory available for IrDa signal data memory allocation\n");
    break;
// Codes 0x30 through 0x3f are USB Firmware Errors
    case 0x30:
    pr_cont("Insufficient memory available for bulk transfer structure\n");
    break;
//
// Other error codes... These are primarily errors that can occur in
// the control messages sent to the redrat
//
    case 0x40:
    if (!rr3.transmitting)
    pr_cont("Signal capture has been terminated\n");
    break;
    case 0x41:
    pr_cont("Attempt to set/get and unknown signal I/O algorithm parameter\n");
    break;
    case 0x42:
    pr_cont("Signal capture already started\n");
    break;
    default:
    pr_cont("Unknown Error\n");
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn redrat3_val_to_mod_freq(irdata: *mut redrat3_irdata) -> u32 {
    static u32 redrat3_val_to_mod_freq(struct redrat3_irdata *irdata)
    {
    let mut mod_freq: u32 = 0;
    let mut mod_freq_count: u16 = be16_to_cpu(irdata.mod_freq_count);
    if (mod_freq_count != 0)
    mod_freq = (RR3_CLK * be16_to_cpu(irdata.num_periods)) /
    (mod_freq_count * RR3_CLK_PER_COUNT);
    return mod_freq;
    }
// this function scales down the figures for the same result...
#[no_mangle]
unsafe extern "C" fn redrat3_len_to_us(length: u32) -> u32 {
    static u32 redrat3_len_to_us(u32 length)
    {
    let mut biglen: u32 = length * 1000;
    let mut divisor: u32 = (RR3_CLK_CONV_FACTOR) / 1000;
    let mut result: u32 = (u32) (biglen / divisor);
// don't allow zero lengths to go back, breaks lirc
    return result ? result : 1;
    }
//
// convert us back into redrat3 lengths
//
// length * 1000   length * 1000000
// ------------- = ---------------- = micro
// rr3clk / 1000       rr3clk
// 6 * 2       4 * 3        micro * rr3clk          micro * rr3clk / 1000
// ----- = 4   ----- = 6    -------------- = len    ---------------------
// 3           2             1000000                    1000
//
#[no_mangle]
unsafe extern "C" fn redrat3_us_to_len(microsec: u32) -> u32 {
    static u32 redrat3_us_to_len(u32 microsec)
    {
    u32 result;
    u32 divisor;
    microsec = (microsec > IR_MAX_DURATION) ? IR_MAX_DURATION : microsec;
    divisor = (RR3_CLK_CONV_FACTOR / 1000);
    result = (u32)(microsec * divisor) / 1000;
// don't allow zero lengths to go back, breaks lirc
    return result ? result : 1;
    }
#[no_mangle]
unsafe extern "C" fn redrat3_process_ir_data(rr3: *mut redrat3_dev) {
    static void redrat3_process_ir_data(struct redrat3_dev *rr3)
    {
    let mut rawir: ir_raw_event = {};
    struct device *dev;
    unsigned int i, sig_size, offset, val;
    u32 mod_freq;
    dev = rr3.dev;
    mod_freq = redrat3_val_to_mod_freq(&rr3.irdata);
    dev_dbg(dev, "Got mod_freq of %u\n", mod_freq);
    if (mod_freq && rr3.wideband) {
    struct ir_raw_event ev = {
    .carrier_report = 1,
    .carrier = mod_freq
    };
    ir_raw_event_store(rr3.rc, &ev);
    }
// process each rr3 encoded byte into an int
    sig_size = be16_to_cpu(rr3.irdata.sig_size);
    for (i = 0; i < sig_size; i++) {
    offset = rr3.irdata.sigdata[i];
    val = get_unaligned_be16(&rr3.irdata.lens[offset]);
// we should always get pulse/space/pulse/space samples
    if (i % 2)
    rawir.pulse = false;
    else
    rawir.pulse = true;
    rawir.duration = redrat3_len_to_us(val);
// cap the value to IR_MAX_DURATION
    rawir.duration = (rawir.duration > IR_MAX_DURATION) ?
    IR_MAX_DURATION : rawir.duration;
    dev_dbg(dev, "storing %s with duration %d (i: %d)\n",
    rawir.pulse ? "pulse" : "space", rawir.duration, i);
    ir_raw_event_store_with_filter(rr3.rc, &rawir);
    }
// add a trailing space
    rawir.pulse = false;
    rawir.timeout = true;
    rawir.duration = rr3.rc.timeout;
    dev_dbg(dev, "storing trailing timeout with duration %d\n",
    rawir.duration);
    ir_raw_event_store_with_filter(rr3.rc, &rawir);
    dev_dbg(dev, "calling ir_raw_event_handle\n");
    ir_raw_event_handle(rr3.rc);
    }
// Util fn to send rr3 cmds
#[no_mangle]
unsafe extern "C" fn redrat3_send_cmd(cmd: c_int, rr3: *mut redrat3_dev) -> c_int {
    static int redrat3_send_cmd(int cmd, struct redrat3_dev *rr3)
    {
    struct usb_device *udev;
    u8 *data;
    int res;
    data = kzalloc(sizeof(u8), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    udev = rr3.udev;
    res = usb_control_msg(udev, usb_rcvctrlpipe(udev, 0), cmd,
    USB_TYPE_VENDOR | USB_RECIP_DEVICE | USB_DIR_IN,
    0x0000, 0x0000, data, sizeof(u8), 10000);
    if (res < 0) {
    dev_err(rr3.dev, "%s: Error sending rr3 cmd res %d, data %d",
    __func__, res, *data);
    res = -EIO;
    } else
    res = data[0];
    kfree(data);
    return res;
    }
// Enables the long range detector and starts async receive
#[no_mangle]
unsafe extern "C" fn redrat3_enable_detector(rr3: *mut redrat3_dev) -> c_int {
    static int redrat3_enable_detector(struct redrat3_dev *rr3)
    {
    struct device *dev = rr3.dev;
    int ret;
    ret = redrat3_send_cmd(RR3_RC_DET_ENABLE, rr3);
    if (ret != 0)
    dev_dbg(dev, "%s: unexpected ret of %d\n",
    __func__, ret);
    ret = redrat3_send_cmd(RR3_RC_DET_STATUS, rr3);
    if (ret != 1) {
    dev_err(dev, "%s: detector status: %d, should be 1\n",
    __func__, ret);
    return -EIO;
    }
    ret = usb_submit_urb(rr3.narrow_urb, GFP_KERNEL);
    if (ret) {
    dev_err(rr3.dev, "narrow band urb failed: %d", ret);
    return ret;
    }
    ret = usb_submit_urb(rr3.wide_urb, GFP_KERNEL);
    if (ret)
    dev_err(rr3.dev, "wide band urb failed: %d", ret);
    return ret;
    }
    static inline void redrat3_delete(struct redrat3_dev *rr3,
    struct usb_device *udev)
    {
    usb_kill_urb(rr3.narrow_urb);
    usb_kill_urb(rr3.wide_urb);
    usb_kill_urb(rr3.flash_urb);
    usb_kill_urb(rr3.learn_urb);
    usb_free_urb(rr3.narrow_urb);
    usb_free_urb(rr3.wide_urb);
    usb_free_urb(rr3.flash_urb);
    usb_free_urb(rr3.learn_urb);
    usb_free_coherent(udev, le16_to_cpu(rr3.ep_narrow.wMaxPacketSize),
    rr3.bulk_in_buf, rr3.dma_in);
    kfree(rr3);
    }
#[no_mangle]
unsafe extern "C" fn redrat3_get_timeout(rr3: *mut redrat3_dev) -> u32 {
    static u32 redrat3_get_timeout(struct redrat3_dev *rr3)
    {
    __be32 *tmp;
    u32 timeout = MS_TO_US(150); /* a sane default, if things go haywire */
    int len, ret, pipe;
    len = sizeof(*tmp);
    tmp = kzalloc(len, GFP_KERNEL);
    if (!tmp)
    return timeout;
    pipe = usb_rcvctrlpipe(rr3.udev, 0);
    ret = usb_control_msg(rr3.udev, pipe, RR3_GET_IR_PARAM,
    USB_TYPE_VENDOR | USB_RECIP_DEVICE | USB_DIR_IN,
    RR3_IR_IO_SIG_TIMEOUT, 0, tmp, len, 5000);
    if (ret != len)
    dev_warn(rr3.dev, "Failed to read timeout from hardware\n");
    else {
    timeout = redrat3_len_to_us(be32_to_cpup(tmp));
    dev_dbg(rr3.dev, "Got timeout of %d ms\n", timeout / 1000);
    }
    kfree(tmp);
    return timeout;
    }
#[no_mangle]
unsafe extern "C" fn redrat3_set_timeout(rc_dev: *mut rc_dev, timeoutus: c_uint) -> c_int {
    static int redrat3_set_timeout(struct rc_dev *rc_dev, unsigned int timeoutus)
    {
    struct redrat3_dev *rr3 = rc_dev.priv;
    struct usb_device *udev = rr3.udev;
    struct device *dev = rr3.dev;
    __be32 *timeout;
    int ret;
    timeout = kmalloc_obj(*timeout);
    if (!timeout)
    return -ENOMEM;
// timeout = cpu_to_be32(redrat3_us_to_len(timeoutus));
    ret = usb_control_msg(udev, usb_sndctrlpipe(udev, 0), RR3_SET_IR_PARAM,
    USB_TYPE_VENDOR | USB_RECIP_DEVICE | USB_DIR_OUT,
    RR3_IR_IO_SIG_TIMEOUT, 0, timeout, sizeof(*timeout),
    25000);
    dev_dbg(dev, "set ir parm timeout %d ret 0x%02x\n",
    be32_to_cpu(*timeout), ret);
    if (ret == sizeof(*timeout))
    ret = 0;
#[no_mangle]
pub unsafe extern "C" fn if(0: ret >=) -> else {
    else if (ret >= 0)
    ret = -EIO;
    kfree(timeout);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn redrat3_reset(rr3: *mut redrat3_dev) {
    static void redrat3_reset(struct redrat3_dev *rr3)
    {
    struct usb_device *udev = rr3.udev;
    struct device *dev = rr3.dev;
    int rc, rxpipe, txpipe;
    u8 *val;
    let mut len: size_t const = sizeof(*val);
    rxpipe = usb_rcvctrlpipe(udev, 0);
    txpipe = usb_sndctrlpipe(udev, 0);
    val = kmalloc(len, GFP_KERNEL);
    if (!val)
    return;
// val = 0x01;
    rc = usb_control_msg(udev, rxpipe, RR3_RESET,
    USB_TYPE_VENDOR | USB_RECIP_DEVICE | USB_DIR_IN,
    RR3_CPUCS_REG_ADDR, 0, val, len, 25000);
    dev_dbg(dev, "reset returned 0x%02x\n", rc);
// val = length_fuzz;
    rc = usb_control_msg(udev, txpipe, RR3_SET_IR_PARAM,
    USB_TYPE_VENDOR | USB_RECIP_DEVICE | USB_DIR_OUT,
    RR3_IR_IO_LENGTH_FUZZ, 0, val, len, 25000);
    dev_dbg(dev, "set ir parm len fuzz %d rc 0x%02x\n", *val, rc);
// val = (65536 - (minimum_pause * 2000)) / 256;
    rc = usb_control_msg(udev, txpipe, RR3_SET_IR_PARAM,
    USB_TYPE_VENDOR | USB_RECIP_DEVICE | USB_DIR_OUT,
    RR3_IR_IO_MIN_PAUSE, 0, val, len, 25000);
    dev_dbg(dev, "set ir parm min pause %d rc 0x%02x\n", *val, rc);
// val = periods_measure_carrier;
    rc = usb_control_msg(udev, txpipe, RR3_SET_IR_PARAM,
    USB_TYPE_VENDOR | USB_RECIP_DEVICE | USB_DIR_OUT,
    RR3_IR_IO_PERIODS_MF, 0, val, len, 25000);
    dev_dbg(dev, "set ir parm periods measure carrier %d rc 0x%02x", *val,
    rc);
// val = RR3_DRIVER_MAXLENS;
    rc = usb_control_msg(udev, txpipe, RR3_SET_IR_PARAM,
    USB_TYPE_VENDOR | USB_RECIP_DEVICE | USB_DIR_OUT,
    RR3_IR_IO_MAX_LENGTHS, 0, val, len, 25000);
    dev_dbg(dev, "set ir parm max lens %d rc 0x%02x\n", *val, rc);
    kfree(val);
    }
#[no_mangle]
unsafe extern "C" fn redrat3_get_firmware_rev(rr3: *mut redrat3_dev) {
    static void redrat3_get_firmware_rev(struct redrat3_dev *rr3)
    {
    int rc;
    char *buffer;
    buffer = kcalloc(RR3_FW_VERSION_LEN + 1, sizeof(*buffer), GFP_KERNEL);
    if (!buffer)
    return;
    rc = usb_control_msg(rr3.udev, usb_rcvctrlpipe(rr3.udev, 0),
    RR3_FW_VERSION,
    USB_TYPE_VENDOR | USB_RECIP_DEVICE | USB_DIR_IN,
    0, 0, buffer, RR3_FW_VERSION_LEN, 5000);
    if (rc >= 0)
    dev_info(rr3.dev, "Firmware rev: %s", buffer);
    else
    dev_err(rr3.dev, "Problem fetching firmware ID\n");
    kfree(buffer);
    }
#[no_mangle]
unsafe extern "C" fn redrat3_read_packet_start(rr3: *mut redrat3_dev, len: unsigned) {
    static void redrat3_read_packet_start(struct redrat3_dev *rr3, unsigned len)
    {
    struct redrat3_header *header = rr3.bulk_in_buf;
    unsigned pktlen, pkttype;
// grab the Length and type of transfer
    pktlen = be16_to_cpu(header.length);
    pkttype = be16_to_cpu(header.transfer_type);
    if (pktlen > sizeof(rr3.irdata)) {
    dev_warn(rr3.dev, "packet length %u too large\n", pktlen);
    return;
    }
    switch (pkttype) {
    case RR3_ERROR:
    if (len >= sizeof(struct redrat3_error)) {
    struct redrat3_error *error = rr3.bulk_in_buf;
    let mut fw_error: unsigned = be16_to_cpu(error.fw_error);
    redrat3_dump_fw_error(rr3, fw_error);
    }
    break;
    case RR3_MOD_SIGNAL_IN:
    memcpy(&rr3.irdata, rr3.bulk_in_buf, len);
    rr3.bytes_read = len;
    dev_dbg(rr3.dev, "bytes_read %d, pktlen %d\n",
    rr3.bytes_read, pktlen);
    break;
    default:
    dev_dbg(rr3.dev, "ignoring packet with type 0x%02x, len of %d, 0x%02x\n",
    pkttype, len, pktlen);
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn redrat3_read_packet_continue(rr3: *mut redrat3_dev, len: unsigned) {
    static void redrat3_read_packet_continue(struct redrat3_dev *rr3, unsigned len)
    {
    void *irdata = &rr3.irdata;
    if (len + rr3.bytes_read > sizeof(rr3.irdata)) {
    dev_warn(rr3.dev, "too much data for packet\n");
    rr3.bytes_read = 0;
    return;
    }
    memcpy(irdata + rr3.bytes_read, rr3.bulk_in_buf, len);
    rr3.bytes_read += len;
    dev_dbg(rr3.dev, "bytes_read %d, pktlen %d\n", rr3.bytes_read,
    be16_to_cpu(rr3.irdata.header.length));
    }
// gather IR data from incoming urb, process it when we have enough
#[no_mangle]
unsafe extern "C" fn redrat3_get_ir_data(rr3: *mut redrat3_dev, len: unsigned) -> c_int {
    static int redrat3_get_ir_data(struct redrat3_dev *rr3, unsigned len)
    {
    struct device *dev = rr3.dev;
    unsigned pkttype;
    let mut ret: c_int = 0;
    if (rr3.bytes_read == 0 && len >= sizeof(struct redrat3_header)) {
    redrat3_read_packet_start(rr3, len);
    } else if (rr3.bytes_read != 0) {
    redrat3_read_packet_continue(rr3, len);
    } else if (rr3.bytes_read == 0) {
    dev_err(dev, "error: no packet data read\n");
    ret = -ENODATA;
    goto out;
    }
    if (rr3.bytes_read < be16_to_cpu(rr3.irdata.header.length) +
    sizeof(struct redrat3_header))
// we're still accumulating data
    return 0;
// if we get here, we've got IR data to decode
    pkttype = be16_to_cpu(rr3.irdata.header.transfer_type);
    if (pkttype == RR3_MOD_SIGNAL_IN)
    redrat3_process_ir_data(rr3);
    else
    dev_dbg(dev, "discarding non-signal data packet (type 0x%02x)\n",
    pkttype);
    out:
    rr3.bytes_read = 0;
    return ret;
    }
// callback function from USB when async USB request has completed
#[no_mangle]
unsafe extern "C" fn redrat3_handle_async(urb: *mut urb) {
    static void redrat3_handle_async(struct urb *urb)
    {
    struct redrat3_dev *rr3 = urb.context;
    int ret;
    switch (urb.status) {
    case 0:
    ret = redrat3_get_ir_data(rr3, urb.actual_length);
    if (!ret && rr3.wideband && !rr3.learn_urb.hcpriv) {
    ret = usb_submit_urb(rr3.learn_urb, GFP_ATOMIC);
    if (ret)
    dev_err(rr3.dev, "Failed to submit learning urb: %d",
    ret);
    }
    if (!ret) {
// no error, prepare to read more
    ret = usb_submit_urb(urb, GFP_ATOMIC);
    if (ret)
    dev_err(rr3.dev, "Failed to resubmit urb: %d",
    ret);
    }
    break;
    case -ECONNRESET:
    case -ENOENT:
    case -ESHUTDOWN:
    usb_unlink_urb(urb);
    return;
    case -EPIPE:
    default:
    dev_warn(rr3.dev, "Error: urb status = %d\n", urb.status);
    rr3.bytes_read = 0;
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn mod_freq_to_val(mod_freq: c_uint) -> u16 {
    static u16 mod_freq_to_val(unsigned int mod_freq)
    {
    let mut mult: c_int = 6000000;
// Clk used in mod. freq. generation is CLK24/4.
    return 65536 - (mult / mod_freq);
    }
#[no_mangle]
unsafe extern "C" fn redrat3_set_tx_carrier(rcdev: *mut rc_dev, carrier: u32) -> c_int {
    static int redrat3_set_tx_carrier(struct rc_dev *rcdev, u32 carrier)
    {
    struct redrat3_dev *rr3 = rcdev.priv;
    struct device *dev = rr3.dev;
    dev_dbg(dev, "Setting modulation frequency to %u", carrier);
    if (carrier == 0)
    return -EINVAL;
    rr3.carrier = carrier;
    return 0;
    }
    static int redrat3_transmit_ir(struct rc_dev *rcdev, unsigned *txbuf,
    unsigned count)
    {
    struct redrat3_dev *rr3 = rcdev.priv;
    struct device *dev = rr3.dev;
    struct redrat3_irdata *irdata = core::ptr::null_mut();
    int ret, ret_len;
    int lencheck, cur_sample_len, pipe;
    int *sample_lens = core::ptr::null_mut();
    let mut curlencheck: u8 = 0;
    unsigned i, sendbuf_len;
    if (rr3.transmitting) {
    dev_warn(dev, "%s: transmitter already in use\n", __func__);
    return -EAGAIN;
    }
    if (count > RR3_MAX_SIG_SIZE - RR3_TX_TRAILER_LEN)
    return -EINVAL;
// rr3 will disable rc detector on transmit
    rr3.transmitting = true;
    sample_lens = kzalloc_objs(*sample_lens, RR3_DRIVER_MAXLENS);
    if (!sample_lens)
    return -ENOMEM;
    irdata = kzalloc_obj(*irdata);
    if (!irdata) {
    ret = -ENOMEM;
    goto out;
    }
    for (i = 0; i < count; i++) {
    cur_sample_len = redrat3_us_to_len(txbuf[i]);
    if (cur_sample_len > 0xffff) {
    dev_warn(dev, "transmit period of %uus truncated to %uus\n",
    txbuf[i], redrat3_len_to_us(0xffff));
    cur_sample_len = 0xffff;
    }
    for (lencheck = 0; lencheck < curlencheck; lencheck++) {
    if (sample_lens[lencheck] == cur_sample_len)
    break;
    }
    if (lencheck == curlencheck) {
    dev_dbg(dev, "txbuf[%d]=%u, pos %d, enc %u\n",
    i, txbuf[i], curlencheck, cur_sample_len);
    if (curlencheck < RR3_DRIVER_MAXLENS) {
// now convert the value to a proper
// rr3 value..
    sample_lens[curlencheck] = cur_sample_len;
    put_unaligned_be16(cur_sample_len,
    &irdata.lens[curlencheck]);
    curlencheck++;
    } else {
    ret = -EINVAL;
    goto out;
    }
    }
    irdata.sigdata[i] = lencheck;
    }
    irdata.sigdata[count] = RR3_END_OF_SIGNAL;
    irdata.sigdata[count + 1] = RR3_END_OF_SIGNAL;
    sendbuf_len = offsetof(struct redrat3_irdata,
    sigdata[count + RR3_TX_TRAILER_LEN]);
// fill in our packet header
    irdata.header.length = cpu_to_be16(sendbuf_len -
    sizeof(struct redrat3_header));
    irdata.header.transfer_type = cpu_to_be16(RR3_MOD_SIGNAL_OUT);
    irdata.pause = cpu_to_be32(redrat3_len_to_us(100));
    irdata.mod_freq_count = cpu_to_be16(mod_freq_to_val(rr3.carrier));
    irdata.no_lengths = curlencheck;
    irdata.sig_size = cpu_to_be16(count + RR3_TX_TRAILER_LEN);
    pipe = usb_sndbulkpipe(rr3.udev, rr3.ep_out.bEndpointAddress);
    ret = usb_bulk_msg(rr3.udev, pipe, irdata,
    sendbuf_len, &ret_len, 10000);
    dev_dbg(dev, "sent %d bytes, (ret %d)\n", ret_len, ret);
// now tell the hardware to transmit what we sent it
    pipe = usb_rcvctrlpipe(rr3.udev, 0);
    ret = usb_control_msg(rr3.udev, pipe, RR3_TX_SEND_SIGNAL,
    USB_TYPE_VENDOR | USB_RECIP_DEVICE | USB_DIR_IN,
    0, 0, irdata, 2, 10000);
    if (ret < 0)
    dev_err(dev, "Error: control msg send failed, rc %d\n", ret);
    else
    ret = count;
    out:
    kfree(irdata);
    kfree(sample_lens);
    rr3.transmitting = false;
// rr3 re-enables rc detector because it was enabled before
    return ret;
    }
    static void redrat3_brightness_set(struct led_classdev *led_dev, enum
    led_brightness brightness)
    {
    struct redrat3_dev *rr3 = container_of(led_dev, struct redrat3_dev,
    led);
    if (brightness != LED_OFF && atomic_cmpxchg(&rr3.flash, 0, 1) == 0) {
    let mut ret: c_int = usb_submit_urb(rr3.flash_urb, GFP_ATOMIC);
    if (ret != 0) {
    dev_dbg(rr3.dev, "%s: unexpected ret of %d\n",
    __func__, ret);
    atomic_set(&rr3.flash, 0);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn redrat3_wideband_receiver(rcdev: *mut rc_dev, enable: c_int) -> c_int {
    static int redrat3_wideband_receiver(struct rc_dev *rcdev, int enable)
    {
    struct redrat3_dev *rr3 = rcdev.priv;
    let mut ret: c_int = 0;
    rr3.wideband = enable != 0;
    if (enable) {
    ret = usb_submit_urb(rr3.learn_urb, GFP_KERNEL);
    if (ret)
    dev_err(rr3.dev, "Failed to submit learning urb: %d",
    ret);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn redrat3_learn_complete(urb: *mut urb) {
    static void redrat3_learn_complete(struct urb *urb)
    {
    struct redrat3_dev *rr3 = urb.context;
    switch (urb.status) {
    case 0:
    break;
    case -ECONNRESET:
    case -ENOENT:
    case -ESHUTDOWN:
    usb_unlink_urb(urb);
    return;
    case -EPIPE:
    default:
    dev_err(rr3.dev, "Error: learn urb status = %d", urb.status);
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn redrat3_led_complete(urb: *mut urb) {
    static void redrat3_led_complete(struct urb *urb)
    {
    struct redrat3_dev *rr3 = urb.context;
    switch (urb.status) {
    case 0:
    break;
    case -ECONNRESET:
    case -ENOENT:
    case -ESHUTDOWN:
    usb_unlink_urb(urb);
    return;
    case -EPIPE:
    default:
    dev_dbg(rr3.dev, "Error: urb status = %d\n", urb.status);
    break;
    }
    rr3.led.brightness = LED_OFF;
    atomic_dec(&rr3.flash);
    }
    static struct rc_dev *redrat3_init_rc_dev(struct redrat3_dev *rr3)
    {
    struct device *dev = rr3.dev;
    struct rc_dev *rc;
    int ret;
    let mut prod: u16 = le16_to_cpu(rr3.udev.descriptor.idProduct);
    rc = rc_allocate_device(RC_DRIVER_IR_RAW);
    if (!rc)
    return core::ptr::null_mut();
    snprintf(rr3.name, sizeof(rr3.name),
    "RedRat3%s Infrared Remote Transceiver",
    prod == USB_RR3IIUSB_PRODUCT_ID ? "-II" : "");
    usb_make_path(rr3.udev, rr3.phys, sizeof(rr3.phys));
    rc.device_name = rr3.name;
    rc.input_phys = rr3.phys;
    usb_to_input_id(rr3.udev, &rc.input_id);
    rc.dev.parent = dev;
    rc.priv = rr3;
    rc.allowed_protocols = RC_PROTO_BIT_ALL_IR_DECODER;
    rc.min_timeout = MS_TO_US(RR3_RX_MIN_TIMEOUT);
    rc.max_timeout = MS_TO_US(RR3_RX_MAX_TIMEOUT);
    rc.timeout = redrat3_get_timeout(rr3);
    rc.s_timeout = redrat3_set_timeout;
    rc.tx_ir = redrat3_transmit_ir;
    rc.s_tx_carrier = redrat3_set_tx_carrier;
    rc.s_carrier_report = redrat3_wideband_receiver;
    rc.driver_name = DRIVER_NAME;
    rc.rx_resolution = 2;
    rc.map_name = RC_MAP_HAUPPAUGE;
    ret = rc_register_device(rc);
    if (ret < 0) {
    dev_err(dev, "remote dev registration failed\n");
    goto out;
    }
    return rc;
    out:
    rc_free_device(rc);
    return core::ptr::null_mut();
    }
    static int redrat3_dev_probe(struct usb_interface *intf,
    const struct usb_device_id *id)
    {
    struct usb_device *udev = interface_to_usbdev(intf);
    struct device *dev = &intf.dev;
    struct usb_host_interface *uhi;
    struct redrat3_dev *rr3;
    struct usb_endpoint_descriptor *ep;
    struct usb_endpoint_descriptor *ep_narrow = core::ptr::null_mut();
    struct usb_endpoint_descriptor *ep_wide = core::ptr::null_mut();
    struct usb_endpoint_descriptor *ep_out = core::ptr::null_mut();
    u8 addr, attrs;
    int pipe, i;
    let mut retval: c_int = -ENOMEM;
    uhi = intf.cur_altsetting;
// find our bulk-in and bulk-out endpoints
    for (i = 0; i < uhi.desc.bNumEndpoints; ++i) {
    ep = &uhi.endpoint[i].desc;
    addr = ep.bEndpointAddress;
    attrs = ep.bmAttributes;
    if (((addr & USB_ENDPOINT_DIR_MASK) == USB_DIR_IN) &&
    ((attrs & USB_ENDPOINT_XFERTYPE_MASK) ==
    USB_ENDPOINT_XFER_BULK)) {
    dev_dbg(dev, "found bulk-in endpoint at 0x%02x\n",
    ep.bEndpointAddress);
// data comes in on 0x82, 0x81 is for learning
    if (ep.bEndpointAddress == RR3_NARROW_IN_EP_ADDR)
    ep_narrow = ep;
    if (ep.bEndpointAddress == RR3_WIDE_IN_EP_ADDR)
    ep_wide = ep;
    }
    if ((ep_out == core::ptr::null_mut()) &&
    ((addr & USB_ENDPOINT_DIR_MASK) == USB_DIR_OUT) &&
    ((attrs & USB_ENDPOINT_XFERTYPE_MASK) ==
    USB_ENDPOINT_XFER_BULK)) {
    dev_dbg(dev, "found bulk-out endpoint at 0x%02x\n",
    ep.bEndpointAddress);
    ep_out = ep;
    }
    }
    if (!ep_narrow || !ep_out || !ep_wide) {
    dev_err(dev, "Couldn't find all endpoints\n");
    retval = -ENODEV;
    goto no_endpoints;
    }
// allocate memory for our device state and initialize it
    rr3 = kzalloc_obj(*rr3);
    if (!rr3)
    goto no_endpoints;
    rr3.dev = &intf.dev;
    rr3.ep_narrow = ep_narrow;
    rr3.ep_out = ep_out;
    rr3.udev = udev;
// set up bulk-in endpoint
    rr3.narrow_urb = usb_alloc_urb(0, GFP_KERNEL);
    if (!rr3.narrow_urb)
    goto redrat_free;
    rr3.wide_urb = usb_alloc_urb(0, GFP_KERNEL);
    if (!rr3.wide_urb)
    goto redrat_free;
    rr3.bulk_in_buf = usb_alloc_coherent(udev,
    le16_to_cpu(ep_narrow.wMaxPacketSize),
    GFP_KERNEL, &rr3.dma_in);
    if (!rr3.bulk_in_buf)
    goto redrat_free;
    pipe = usb_rcvbulkpipe(udev, ep_narrow.bEndpointAddress);
    usb_fill_bulk_urb(rr3.narrow_urb, udev, pipe, rr3.bulk_in_buf,
    le16_to_cpu(ep_narrow.wMaxPacketSize),
    redrat3_handle_async, rr3);
    rr3.narrow_urb.transfer_dma = rr3.dma_in;
    rr3.narrow_urb.transfer_flags |= URB_NO_TRANSFER_DMA_MAP;
    pipe = usb_rcvbulkpipe(udev, ep_wide.bEndpointAddress);
    usb_fill_bulk_urb(rr3.wide_urb, udev, pipe, rr3.bulk_in_buf,
    le16_to_cpu(ep_narrow.wMaxPacketSize),
    redrat3_handle_async, rr3);
    rr3.wide_urb.transfer_dma = rr3.dma_in;
    rr3.wide_urb.transfer_flags |= URB_NO_TRANSFER_DMA_MAP;
    redrat3_reset(rr3);
    redrat3_get_firmware_rev(rr3);
// default.. will get overridden by any sends with a freq defined
    rr3.carrier = 38000;
    atomic_set(&rr3.flash, 0);
    rr3.flash_urb = usb_alloc_urb(0, GFP_KERNEL);
    if (!rr3.flash_urb)
    goto redrat_free;
// learn urb
    rr3.learn_urb = usb_alloc_urb(0, GFP_KERNEL);
    if (!rr3.learn_urb)
    goto redrat_free;
// setup packet is 'c0 b2 0000 0000 0001'
    rr3.learn_control.bRequestType = 0xc0;
    rr3.learn_control.bRequest = RR3_MODSIG_CAPTURE;
    rr3.learn_control.wLength = cpu_to_le16(1);
    usb_fill_control_urb(rr3.learn_urb, udev, usb_rcvctrlpipe(udev, 0),
    (unsigned char *)&rr3.learn_control,
    &rr3.learn_buf, sizeof(rr3.learn_buf),
    redrat3_learn_complete, rr3);
// setup packet is 'c0 b9 0000 0000 0001'
    rr3.flash_control.bRequestType = 0xc0;
    rr3.flash_control.bRequest = RR3_BLINK_LED;
    rr3.flash_control.wLength = cpu_to_le16(1);
    usb_fill_control_urb(rr3.flash_urb, udev, usb_rcvctrlpipe(udev, 0),
    (unsigned char *)&rr3.flash_control,
    &rr3.flash_in_buf, sizeof(rr3.flash_in_buf),
    redrat3_led_complete, rr3);
// led control
    rr3.led.name = "redrat3:red:feedback";
    rr3.led.default_trigger = "rc-feedback";
    rr3.led.brightness_set = redrat3_brightness_set;
    retval = led_classdev_register(&intf.dev, &rr3.led);
    if (retval)
    goto redrat_free;
    rr3.rc = redrat3_init_rc_dev(rr3);
    if (!rr3.rc) {
    retval = -ENOMEM;
    goto led_free;
    }
// might be all we need to do?
    retval = redrat3_enable_detector(rr3);
    if (retval < 0)
    goto led_free;
// we can register the device now, as it is ready
    usb_set_intfdata(intf, rr3);
    return 0;
    led_free:
    led_classdev_unregister(&rr3.led);
    redrat_free:
    redrat3_delete(rr3, rr3.udev);
    no_endpoints:
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn redrat3_dev_disconnect(intf: *mut usb_interface) {
    static void redrat3_dev_disconnect(struct usb_interface *intf)
    {
    struct usb_device *udev = interface_to_usbdev(intf);
    struct redrat3_dev *rr3 = usb_get_intfdata(intf);
    struct rc_dev *rc = rr3.rc;
    usb_set_intfdata(intf, core::ptr::null_mut());
    rc_unregister_device(rc);
    led_classdev_unregister(&rr3.led);
    redrat3_delete(rr3, udev);
    rc_free_device(rc);
    }
#[no_mangle]
unsafe extern "C" fn redrat3_dev_suspend(intf: *mut usb_interface, message: pm_message_t) -> c_int {
    static int redrat3_dev_suspend(struct usb_interface *intf, pm_message_t message)
    {
    struct redrat3_dev *rr3 = usb_get_intfdata(intf);
    led_classdev_suspend(&rr3.led);
    usb_kill_urb(rr3.narrow_urb);
    usb_kill_urb(rr3.wide_urb);
    usb_kill_urb(rr3.flash_urb);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn redrat3_dev_resume(intf: *mut usb_interface) -> c_int {
    static int redrat3_dev_resume(struct usb_interface *intf)
    {
    struct redrat3_dev *rr3 = usb_get_intfdata(intf);
    if (usb_submit_urb(rr3.narrow_urb, GFP_NOIO))
    return -EIO;
    if (usb_submit_urb(rr3.wide_urb, GFP_NOIO))
    return -EIO;
    led_classdev_resume(&rr3.led);
    return 0;
    }
    static struct usb_driver redrat3_dev_driver = {
    .name		= DRIVER_NAME,
    .probe		= redrat3_dev_probe,
    .disconnect	= redrat3_dev_disconnect,
    .suspend	= redrat3_dev_suspend,
    .resume		= redrat3_dev_resume,
    .reset_resume	= redrat3_dev_resume,
    .id_table	= redrat3_dev_table
    };
    module_usb_driver(redrat3_dev_driver);
    MODULE_DESCRIPTION(DRIVER_DESC);
    MODULE_AUTHOR(DRIVER_AUTHOR);
    MODULE_AUTHOR(DRIVER_AUTHOR2);
    MODULE_LICENSE("GPL");
    MODULE_DEVICE_TABLE(usb, redrat3_dev_table);
