//! Automatically rewritten from C to Rust
//! Source: drivers/comedi/drivers/usbduxsigma.c
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
// usbduxsigma.c
// Copyright (C) 2011-2015 Bernd Porr, mail@berndporr.me.uk
//
// Driver: usbduxsigma
// Description: University of Stirling USB DAQ & INCITE Technology Limited
// Devices: [ITL] USB-DUX-SIGMA (usbduxsigma)
// Author: Bernd Porr <mail@berndporr.me.uk>
// Updated: 20 July 2015
// Status: stable
//
// I must give credit here to Chris Baugher who
// wrote the driver for AT-MIO-16d. I used some parts of this
// driver. I also must give credits to David Brownell
// who supported me with the USB development.
//
// Note: the raw data from the A/D converter is 24 bit big endian
// anything else is little endian to/from the dux board
//
// Revision history:
// 0.1: initial version
// 0.2: all basic functions implemented, digital I/O only for one port
// 0.3: proper vendor ID and driver name
// 0.4: fixed D/A voltage range
// 0.5: various bug fixes, health check at startup
// 0.6: corrected wrong input range
// 0.7: rewrite code that urb->interval is always 1
//

// timeout for the USB-transfer in ms
pub const BULK_TIMEOUT: c_int = 1000;
// constants for "firmware" upload and download

pub const FIRMWARE_MAX_LEN: c_uint = 0x4000;
pub const USBDUXSUB_FIRMWARE: c_uint = 0xa0;
pub const VENDOR_DIR_IN: c_uint = 0xc0;
pub const VENDOR_DIR_OUT: c_uint = 0x40;
// internal addresses of the 8051 processor
pub const USBDUXSUB_CPUCS: c_uint = 0xE600;
// 300Hz max frequ under PWM

// Default PWM frequency

// Number of channels (16 AD and offset)
pub const NUMCHANNELS: c_int = 16;
// Size of one A/D value

//
// Size of the async input-buffer IN BYTES, the DIO state is transmitted
// as the first byte.
//

// 16 bytes.
pub const SIZEINSNBUF: c_int = 16;
// Number of DA channels
pub const NUMOUTCHANNELS: c_int = 8;
// size of one value for the D/A converter: channel and value

//
// Size of the output-buffer in bytes
// Actually only the first 4 triplets are used but for the
// high speed mode we need to pad it to 8 (microframes).
//

//
// Size of the buffer for the dux commands: just now max size is determined
// by the analogue out + command byte + panic bytes...
//

// Number of in-URBs which receive the data: min=2
pub const NUMOFINBUFFERSFULL: c_int = 5;
// Number of out-URBs which send the data: min=2
pub const NUMOFOUTBUFFERSFULL: c_int = 5;
// Number of in-URBs which receive the data: min=5
// must have more buffers due to buggy USB ctr
pub const NUMOFINBUFFERSHIGH: c_int = 10;
// Number of out-URBs which send the data: min=5
// must have more buffers due to buggy USB ctr
pub const NUMOFOUTBUFFERSHIGH: c_int = 10;
// number of retries to get the right dux command
pub const RETRIES: c_int = 10;
// bulk transfer commands to usbduxsigma
pub const USBBUXSIGMA_AD_CMD: c_int = 9;
pub const USBDUXSIGMA_DA_CMD: c_int = 1;
pub const USBDUXSIGMA_DIO_CFG_CMD: c_int = 2;
pub const USBDUXSIGMA_DIO_BITS_CMD: c_int = 3;
pub const USBDUXSIGMA_SINGLE_AD_CMD: c_int = 4;
pub const USBDUXSIGMA_PWM_ON_CMD: c_int = 7;
pub const USBDUXSIGMA_PWM_OFF_CMD: c_int = 8;
    static const struct comedi_lrange usbduxsigma_ai_range = {
    1, {
    BIP_RANGE(2.5 * 0x800000 / 0x780000 / 2.0)
    }
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbduxsigma_private {
// actual number of in-buffers
    pub n_ai_urbs: c_int,
// actual number of out-buffers
    pub n_ao_urbs: c_int,
// ISO-transfer handling: buffers
    pub ai_urbs: *mut urb,
    pub ao_urbs: *mut urb,
// pwm-transfer handling
    pub pwm_urb: *mut urb,
// PWM period
    pub pwm_period: c_uint,
// PWM internal delay for the GPIF in the FX2
    pub pwm_delay: u8,
// size of the PWM buffer which holds the bit pattern
    pub pwm_buf_sz: c_int,
// input buffer for the ISO-transfer
    pub in_buf: *mut __be32,
// input buffer for single insn
    pub insn_buf: *mut u8,
    pub high_speed:1: unsigned,
    pub ai_cmd_running:1: unsigned,
    pub ao_cmd_running:1: unsigned,
    pub pwm_cmd_running:1: unsigned,
// time between samples in units of the timer
    pub ai_timer: c_uint,
    pub ao_timer: c_uint,
// counter between acquisitions
    pub ai_counter: c_uint,
    pub ao_counter: c_uint,
// interval in frames/uframes
    pub ai_interval: c_uint,
// commands
    pub dux_commands: *mut u8,
    pub mut: mutex,
}

#[no_mangle]
unsafe extern "C" fn usbduxsigma_unlink_urbs(urbs: *mut urb, num_urbs: c_int) {
    static void usbduxsigma_unlink_urbs(struct urb **urbs, int num_urbs)
    {
    int i;
    for (i = 0; i < num_urbs; i++)
    usb_kill_urb(urbs[i]);
    }
#[no_mangle]
unsafe extern "C" fn usbduxsigma_ai_stop(dev: *mut comedi_device, do_unlink: c_int) {
    static void usbduxsigma_ai_stop(struct comedi_device *dev, int do_unlink)
    {
    struct usbduxsigma_private *devpriv = dev.private;
    if (do_unlink && devpriv.ai_urbs)
    usbduxsigma_unlink_urbs(devpriv.ai_urbs, devpriv.n_ai_urbs);
    devpriv.ai_cmd_running = 0;
    }
    static int usbduxsigma_ai_cancel(struct comedi_device *dev,
    struct comedi_subdevice *s)
    {
    struct usbduxsigma_private *devpriv = dev.private;
    mutex_lock(&devpriv.mut);
// unlink only if it is really running
    usbduxsigma_ai_stop(dev, devpriv.ai_cmd_running);
    mutex_unlock(&devpriv.mut);
    return 0;
    }
    static void usbduxsigma_ai_handle_urb(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct urb *urb)
    {
    struct usbduxsigma_private *devpriv = dev.private;
    struct comedi_async *async = s.async;
    struct comedi_cmd *cmd = &async.cmd;
    u32 val;
    int ret;
    int i;
    if ((urb.actual_length > 0) && (urb.status != -EXDEV)) {
    devpriv.ai_counter--;
    if (devpriv.ai_counter == 0) {
    devpriv.ai_counter = devpriv.ai_timer;
//
// Get the data from the USB bus and hand it over
// to comedi. Note, first byte is the DIO state.
//
    for (i = 0; i < cmd.chanlist_len; i++) {
    val = be32_to_cpu(devpriv.in_buf[i + 1]);
    val &= 0x00ffffff; /* strip status byte */
    val = comedi_offset_munge(s, val);
    if (!comedi_buf_write_samples(s, &val, 1))
    return;
    }
    if (cmd.stop_src == TRIG_COUNT &&
    async.scans_done >= cmd.stop_arg)
    async.events |= COMEDI_CB_EOA;
    }
    }
// if command is still running, resubmit urb
    if (!(async.events & COMEDI_CB_CANCEL_MASK)) {
    urb.dev = comedi_to_usb_dev(dev);
    ret = usb_submit_urb(urb, GFP_ATOMIC);
    if (ret < 0) {
    dev_err(dev.class_dev, "urb resubmit failed (%d)\n",
    ret);
    if (ret == -EL2NSYNC)
    dev_err(dev.class_dev,
    "buggy USB host controller or bug in IRQ handler\n");
    async.events |= COMEDI_CB_ERROR;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn usbduxsigma_ai_urb_complete(urb: *mut urb) {
    static void usbduxsigma_ai_urb_complete(struct urb *urb)
    {
    struct comedi_device *dev = urb.context;
    struct usbduxsigma_private *devpriv = dev.private;
    struct comedi_subdevice *s = dev.read_subdev;
    struct comedi_async *async = s.async;
// exit if not running a command, do not resubmit urb
    if (!devpriv.ai_cmd_running)
    return;
    switch (urb.status) {
    case 0:
// copy the result in the transfer buffer
    memcpy(devpriv.in_buf, urb.transfer_buffer, SIZEINBUF);
    usbduxsigma_ai_handle_urb(dev, s, urb);
    break;
    case -EILSEQ:
//
// error in the ISOchronous data
// we don't copy the data into the transfer buffer
// and recycle the last data byte
//
    dev_dbg(dev.class_dev, "CRC error in ISO IN stream\n");
    usbduxsigma_ai_handle_urb(dev, s, urb);
    break;
    case -ECONNRESET:
    case -ENOENT:
    case -ESHUTDOWN:
    case -ECONNABORTED:
// happens after an unlink command
    async.events |= COMEDI_CB_ERROR;
    break;
    default:
// a real error
    dev_err(dev.class_dev, "non-zero urb status (%d)\n",
    urb.status);
    async.events |= COMEDI_CB_ERROR;
    break;
    }
//
// comedi_handle_events() cannot be used in this driver. The (*cancel)
// operation would unlink the urb.
//
    if (async.events & COMEDI_CB_CANCEL_MASK)
    usbduxsigma_ai_stop(dev, 0);
    comedi_event(dev, s);
    }
#[no_mangle]
unsafe extern "C" fn usbduxsigma_ao_stop(dev: *mut comedi_device, do_unlink: c_int) {
    static void usbduxsigma_ao_stop(struct comedi_device *dev, int do_unlink)
    {
    struct usbduxsigma_private *devpriv = dev.private;
    if (do_unlink && devpriv.ao_urbs)
    usbduxsigma_unlink_urbs(devpriv.ao_urbs, devpriv.n_ao_urbs);
    devpriv.ao_cmd_running = 0;
    }
    static int usbduxsigma_ao_cancel(struct comedi_device *dev,
    struct comedi_subdevice *s)
    {
    struct usbduxsigma_private *devpriv = dev.private;
    mutex_lock(&devpriv.mut);
// unlink only if it is really running
    usbduxsigma_ao_stop(dev, devpriv.ao_cmd_running);
    mutex_unlock(&devpriv.mut);
    return 0;
    }
    static void usbduxsigma_ao_handle_urb(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct urb *urb)
    {
    struct usbduxsigma_private *devpriv = dev.private;
    struct comedi_async *async = s.async;
    struct comedi_cmd *cmd = &async.cmd;
    u8 *datap;
    int ret;
    int i;
    devpriv.ao_counter--;
    if (devpriv.ao_counter == 0) {
    devpriv.ao_counter = devpriv.ao_timer;
    if (cmd.stop_src == TRIG_COUNT &&
    async.scans_done >= cmd.stop_arg) {
    async.events |= COMEDI_CB_EOA;
    return;
    }
// transmit data to the USB bus
    datap = urb.transfer_buffer;
// datap++ = cmd->chanlist_len;
    for (i = 0; i < cmd.chanlist_len; i++) {
    let mut chan: c_uint = CR_CHAN(cmd.chanlist[i]);
    unsigned short val;
    if (!comedi_buf_read_samples(s, &val, 1)) {
    dev_err(dev.class_dev, "buffer underflow\n");
    async.events |= COMEDI_CB_OVERFLOW;
    return;
    }
// datap++ = val;
// datap++ = chan;
    s.readback[chan] = val;
    }
    }
// if command is still running, resubmit urb
    if (!(async.events & COMEDI_CB_CANCEL_MASK)) {
    urb.transfer_buffer_length = SIZEOUTBUF;
    urb.dev = comedi_to_usb_dev(dev);
    urb.status = 0;
    urb.interval = 1;	/* (u)frames */
    urb.number_of_packets = 1;
    urb.iso_frame_desc[0].offset = 0;
    urb.iso_frame_desc[0].length = SIZEOUTBUF;
    urb.iso_frame_desc[0].status = 0;
    ret = usb_submit_urb(urb, GFP_ATOMIC);
    if (ret < 0) {
    dev_err(dev.class_dev, "urb resubmit failed (%d)\n",
    ret);
    if (ret == -EL2NSYNC)
    dev_err(dev.class_dev,
    "buggy USB host controller or bug in IRQ handler\n");
    async.events |= COMEDI_CB_ERROR;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn usbduxsigma_ao_urb_complete(urb: *mut urb) {
    static void usbduxsigma_ao_urb_complete(struct urb *urb)
    {
    struct comedi_device *dev = urb.context;
    struct usbduxsigma_private *devpriv = dev.private;
    struct comedi_subdevice *s = dev.write_subdev;
    struct comedi_async *async = s.async;
// exit if not running a command, do not resubmit urb
    if (!devpriv.ao_cmd_running)
    return;
    switch (urb.status) {
    case 0:
    usbduxsigma_ao_handle_urb(dev, s, urb);
    break;
    case -ECONNRESET:
    case -ENOENT:
    case -ESHUTDOWN:
    case -ECONNABORTED:
// happens after an unlink command
    async.events |= COMEDI_CB_ERROR;
    break;
    default:
// a real error
    dev_err(dev.class_dev, "non-zero urb status (%d)\n",
    urb.status);
    async.events |= COMEDI_CB_ERROR;
    break;
    }
//
// comedi_handle_events() cannot be used in this driver. The (*cancel)
// operation would unlink the urb.
//
    if (async.events & COMEDI_CB_CANCEL_MASK)
    usbduxsigma_ao_stop(dev, 0);
    comedi_event(dev, s);
    }
    static int usbduxsigma_submit_urbs(struct comedi_device *dev,
    struct urb **urbs, int num_urbs,
    int input_urb)
    {
    struct usb_device *usb = comedi_to_usb_dev(dev);
    struct urb *urb;
    int ret;
    int i;
// Submit all URBs and start the transfer on the bus
    for (i = 0; i < num_urbs; i++) {
    urb = urbs[i];
// in case of a resubmission after an unlink...
    if (input_urb)
    urb.interval = 1;
    urb.context = dev;
    urb.dev = usb;
    urb.status = 0;
    urb.transfer_flags = URB_ISO_ASAP;
    ret = usb_submit_urb(urb, GFP_ATOMIC);
    if (ret)
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn usbduxsigma_chans_to_interval(num_chan: c_int) -> c_int {
    static int usbduxsigma_chans_to_interval(int num_chan)
    {
    if (num_chan <= 2)
    return 2;	/* 4kHz */
    if (num_chan <= 8)
    return 4;	/* 2kHz */
    return 8;		/* 1kHz */
    }
    static int usbduxsigma_ai_cmdtest(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_cmd *cmd)
    {
    struct usbduxsigma_private *devpriv = dev.private;
    let mut high_speed: c_int = devpriv.high_speed;
    let mut interval: c_int = usbduxsigma_chans_to_interval(cmd.chanlist_len);
    unsigned int tmp;
    let mut err: c_int = 0;
// Step 1 : check if triggers are trivially valid
    err |= comedi_check_trigger_src(&cmd.start_src, TRIG_NOW | TRIG_INT);
    err |= comedi_check_trigger_src(&cmd.scan_begin_src, TRIG_TIMER);
    err |= comedi_check_trigger_src(&cmd.convert_src, TRIG_NOW);
    err |= comedi_check_trigger_src(&cmd.scan_end_src, TRIG_COUNT);
    err |= comedi_check_trigger_src(&cmd.stop_src, TRIG_COUNT | TRIG_NONE);
    if (err)
    return 1;
// Step 2a : make sure trigger sources are unique
    err |= comedi_check_trigger_is_unique(cmd.start_src);
    err |= comedi_check_trigger_is_unique(cmd.stop_src);
// Step 2b : and mutually compatible
    if (err)
    return 2;
// Step 3: check if arguments are trivially valid
    err |= comedi_check_trigger_arg_is(&cmd.start_arg, 0);
    if (high_speed) {
//
// In high speed mode microframes are possible.
// However, during one microframe we can roughly
// sample two channels. Thus, the more channels
// are in the channel list the more time we need.
//
    err |= comedi_check_trigger_arg_min(&cmd.scan_begin_arg,
    (125000 * interval));
    } else {
// full speed
// 1kHz scans every USB frame
    err |= comedi_check_trigger_arg_min(&cmd.scan_begin_arg,
    1000000);
    }
    err |= comedi_check_trigger_arg_is(&cmd.scan_end_arg,
    cmd.chanlist_len);
    if (cmd.stop_src == TRIG_COUNT)
    err |= comedi_check_trigger_arg_min(&cmd.stop_arg, 1);
    else	/* TRIG_NONE */
    err |= comedi_check_trigger_arg_is(&cmd.stop_arg, 0);
    if (err)
    return 3;
// Step 4: fix up any arguments
    tmp = rounddown(cmd.scan_begin_arg, high_speed ? 125000 : 1000000);
    err |= comedi_check_trigger_arg_is(&cmd.scan_begin_arg, tmp);
    if (err)
    return 4;
    return 0;
    }
//
// creates the ADC command for the MAX1271
// range is the range value from comedi
//
    static void create_adc_command(unsigned int chan,
    u8 *muxsg0, u8 *muxsg1)
    {
    if (chan < 8)
    (*muxsg0) = (*muxsg0) | (1 << chan);
#[no_mangle]
pub unsafe extern "C" fn if(16: chan <) -> else {
    else if (chan < 16)
    (*muxsg1) = (*muxsg1) | (1 << (chan - 8));
    }
#[no_mangle]
unsafe extern "C" fn usbbuxsigma_send_cmd(dev: *mut comedi_device, cmd_type: c_int) -> c_int {
    static int usbbuxsigma_send_cmd(struct comedi_device *dev, int cmd_type)
    {
    struct usb_device *usb = comedi_to_usb_dev(dev);
    struct usbduxsigma_private *devpriv = dev.private;
    int nsent;
    devpriv.dux_commands[0] = cmd_type;
    return usb_bulk_msg(usb, usb_sndbulkpipe(usb, 1),
    devpriv.dux_commands, SIZEOFDUXBUFFER,
    &nsent, BULK_TIMEOUT);
    }
#[no_mangle]
unsafe extern "C" fn usbduxsigma_receive_cmd(dev: *mut comedi_device, command: c_int) -> c_int {
    static int usbduxsigma_receive_cmd(struct comedi_device *dev, int command)
    {
    struct usb_device *usb = comedi_to_usb_dev(dev);
    struct usbduxsigma_private *devpriv = dev.private;
    int nrec;
    int ret;
    int i;
    for (i = 0; i < RETRIES; i++) {
    ret = usb_bulk_msg(usb, usb_rcvbulkpipe(usb, 8),
    devpriv.insn_buf, SIZEINSNBUF,
    &nrec, BULK_TIMEOUT);
    if (ret < 0)
    return ret;
    if (devpriv.insn_buf[0] == command)
    return 0;
    }
//
// This is only reached if the data has been requested a
// couple of times and the command was not received.
//
    return -EFAULT;
    }
    static int usbduxsigma_ai_inttrig(struct comedi_device *dev,
    struct comedi_subdevice *s,
    unsigned int trig_num)
    {
    struct usbduxsigma_private *devpriv = dev.private;
    struct comedi_cmd *cmd = &s.async.cmd;
    int ret;
    if (trig_num != cmd.start_arg)
    return -EINVAL;
    mutex_lock(&devpriv.mut);
    if (!devpriv.ai_cmd_running) {
    devpriv.ai_cmd_running = 1;
    ret = usbduxsigma_submit_urbs(dev, devpriv.ai_urbs,
    devpriv.n_ai_urbs, 1);
    if (ret < 0) {
    devpriv.ai_cmd_running = 0;
    mutex_unlock(&devpriv.mut);
    return ret;
    }
    s.async.inttrig = core::ptr::null_mut();
    }
    mutex_unlock(&devpriv.mut);
    return 1;
    }
    static int usbduxsigma_ai_cmd(struct comedi_device *dev,
    struct comedi_subdevice *s)
    {
    struct usbduxsigma_private *devpriv = dev.private;
    struct comedi_cmd *cmd = &s.async.cmd;
    let mut len: c_uint = cmd.chanlist_len;
    let mut muxsg0: u8 = 0;
    let mut muxsg1: u8 = 0;
    let mut sysred: u8 = 0;
    int ret;
    int i;
    mutex_lock(&devpriv.mut);
    if (devpriv.high_speed) {
//
// every 2 channels get a time window of 125us. Thus, if we
// sample all 16 channels we need 1ms. If we sample only one
// channel we need only 125us
//
    let mut interval: c_uint = usbduxsigma_chans_to_interval(len);
    devpriv.ai_interval = interval;
    devpriv.ai_timer = cmd.scan_begin_arg / (125000 * interval);
    } else {
// interval always 1ms
    devpriv.ai_interval = 1;
    devpriv.ai_timer = cmd.scan_begin_arg / 1000000;
    }
    for (i = 0; i < len; i++) {
    let mut chan: c_uint = CR_CHAN(cmd.chanlist[i]);
    create_adc_command(chan, &muxsg0, &muxsg1);
    }
    devpriv.dux_commands[1] = devpriv.ai_interval;
    devpriv.dux_commands[2] = len;  /* num channels per time step */
    devpriv.dux_commands[3] = 0x12; /* CONFIG0 */
    devpriv.dux_commands[4] = 0x03; /* CONFIG1: 23kHz sample, delay 0us */
    devpriv.dux_commands[5] = 0x00; /* CONFIG3: diff. channels off */
    devpriv.dux_commands[6] = muxsg0;
    devpriv.dux_commands[7] = muxsg1;
    devpriv.dux_commands[8] = sysred;
    ret = usbbuxsigma_send_cmd(dev, USBBUXSIGMA_AD_CMD);
    if (ret < 0) {
    mutex_unlock(&devpriv.mut);
    return ret;
    }
    devpriv.ai_counter = devpriv.ai_timer;
    if (cmd.start_src == TRIG_NOW) {
// enable this acquisition operation
    devpriv.ai_cmd_running = 1;
    ret = usbduxsigma_submit_urbs(dev, devpriv.ai_urbs,
    devpriv.n_ai_urbs, 1);
    if (ret < 0) {
    devpriv.ai_cmd_running = 0;
    mutex_unlock(&devpriv.mut);
    return ret;
    }
    s.async.inttrig = core::ptr::null_mut();
    } else {	/* TRIG_INT */
    s.async.inttrig = usbduxsigma_ai_inttrig;
    }
    mutex_unlock(&devpriv.mut);
    return 0;
    }
    static int usbduxsigma_ai_insn_read(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    struct usbduxsigma_private *devpriv = dev.private;
    let mut chan: c_uint = CR_CHAN(insn.chanspec);
    let mut muxsg0: u8 = 0;
    let mut muxsg1: u8 = 0;
    let mut sysred: u8 = 0;
    int ret;
    int i;
    mutex_lock(&devpriv.mut);
    if (devpriv.ai_cmd_running) {
    mutex_unlock(&devpriv.mut);
    return -EBUSY;
    }
    create_adc_command(chan, &muxsg0, &muxsg1);
// Mode 0 is used to get a single conversion on demand
    devpriv.dux_commands[1] = 0x16; /* CONFIG0: chopper on */
    devpriv.dux_commands[2] = 0x80; /* CONFIG1: 2kHz sampling rate */
    devpriv.dux_commands[3] = 0x00; /* CONFIG3: diff. channels off */
    devpriv.dux_commands[4] = muxsg0;
    devpriv.dux_commands[5] = muxsg1;
    devpriv.dux_commands[6] = sysred;
// adc commands
    ret = usbbuxsigma_send_cmd(dev, USBDUXSIGMA_SINGLE_AD_CMD);
    if (ret < 0) {
    mutex_unlock(&devpriv.mut);
    return ret;
    }
    for (i = 0; i < insn.n; i++) {
    u32 val;
    ret = usbduxsigma_receive_cmd(dev, USBDUXSIGMA_SINGLE_AD_CMD);
    if (ret < 0) {
    mutex_unlock(&devpriv.mut);
    return ret;
    }
// 32 bits big endian from the A/D converter
    val = be32_to_cpu(get_unaligned((__be32
// )(devpriv->insn_buf + 1)));
    val &= 0x00ffffff;	/* strip status byte */
    data[i] = comedi_offset_munge(s, val);
    }
    mutex_unlock(&devpriv.mut);
    return insn.n;
    }
    static int usbduxsigma_ao_insn_read(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    struct usbduxsigma_private *devpriv = dev.private;
    int ret;
    mutex_lock(&devpriv.mut);
    ret = comedi_readback_insn_read(dev, s, insn, data);
    mutex_unlock(&devpriv.mut);
    return ret;
    }
    static int usbduxsigma_ao_insn_write(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    struct usbduxsigma_private *devpriv = dev.private;
    let mut chan: c_uint = CR_CHAN(insn.chanspec);
    int ret;
    int i;
    mutex_lock(&devpriv.mut);
    if (devpriv.ao_cmd_running) {
    mutex_unlock(&devpriv.mut);
    return -EBUSY;
    }
    for (i = 0; i < insn.n; i++) {
    devpriv.dux_commands[1] = 1;		/* num channels */
    devpriv.dux_commands[2] = data[i];	/* value */
    devpriv.dux_commands[3] = chan;	/* channel number */
    ret = usbbuxsigma_send_cmd(dev, USBDUXSIGMA_DA_CMD);
    if (ret < 0) {
    mutex_unlock(&devpriv.mut);
    return ret;
    }
    s.readback[chan] = data[i];
    }
    mutex_unlock(&devpriv.mut);
    return insn.n;
    }
    static int usbduxsigma_ao_inttrig(struct comedi_device *dev,
    struct comedi_subdevice *s,
    unsigned int trig_num)
    {
    struct usbduxsigma_private *devpriv = dev.private;
    struct comedi_cmd *cmd = &s.async.cmd;
    int ret;
    if (trig_num != cmd.start_arg)
    return -EINVAL;
    mutex_lock(&devpriv.mut);
    if (!devpriv.ao_cmd_running) {
    devpriv.ao_cmd_running = 1;
    ret = usbduxsigma_submit_urbs(dev, devpriv.ao_urbs,
    devpriv.n_ao_urbs, 0);
    if (ret < 0) {
    devpriv.ao_cmd_running = 0;
    mutex_unlock(&devpriv.mut);
    return ret;
    }
    s.async.inttrig = core::ptr::null_mut();
    }
    mutex_unlock(&devpriv.mut);
    return 1;
    }
    static int usbduxsigma_ao_cmdtest(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_cmd *cmd)
    {
    struct usbduxsigma_private *devpriv = dev.private;
    unsigned int tmp;
    let mut err: c_int = 0;
// Step 1 : check if triggers are trivially valid
    err |= comedi_check_trigger_src(&cmd.start_src, TRIG_NOW | TRIG_INT);
//
// For now, always use "scan" timing with all channels updated at once
// (cmd->scan_begin_src == TRIG_TIMER, cmd->convert_src == TRIG_NOW).
//
// In a future version, "convert" timing with channels updated
// indivually may be supported in high speed mode
// (cmd->scan_begin_src == TRIG_FOLLOW, cmd->convert_src == TRIG_TIMER).
//
    err |= comedi_check_trigger_src(&cmd.scan_begin_src, TRIG_TIMER);
    err |= comedi_check_trigger_src(&cmd.convert_src, TRIG_NOW);
    err |= comedi_check_trigger_src(&cmd.scan_end_src, TRIG_COUNT);
    err |= comedi_check_trigger_src(&cmd.stop_src, TRIG_COUNT | TRIG_NONE);
    if (err) {
    mutex_unlock(&devpriv.mut);
    return 1;
    }
// Step 2a : make sure trigger sources are unique
    err |= comedi_check_trigger_is_unique(cmd.start_src);
    err |= comedi_check_trigger_is_unique(cmd.stop_src);
// Step 2b : and mutually compatible
    if (err)
    return 2;
// Step 3: check if arguments are trivially valid
    err |= comedi_check_trigger_arg_is(&cmd.start_arg, 0);
    err |= comedi_check_trigger_arg_min(&cmd.scan_begin_arg, 1000000);
    err |= comedi_check_trigger_arg_is(&cmd.scan_end_arg,
    cmd.chanlist_len);
    if (cmd.stop_src == TRIG_COUNT)
    err |= comedi_check_trigger_arg_min(&cmd.stop_arg, 1);
    else	/* TRIG_NONE */
    err |= comedi_check_trigger_arg_is(&cmd.stop_arg, 0);
    if (err)
    return 3;
// Step 4: fix up any arguments
    tmp = rounddown(cmd.scan_begin_arg, 1000000);
    err |= comedi_check_trigger_arg_is(&cmd.scan_begin_arg, tmp);
    if (err)
    return 4;
    return 0;
    }
    static int usbduxsigma_ao_cmd(struct comedi_device *dev,
    struct comedi_subdevice *s)
    {
    struct usbduxsigma_private *devpriv = dev.private;
    struct comedi_cmd *cmd = &s.async.cmd;
    int ret;
    mutex_lock(&devpriv.mut);
//
// For now, only "scan" timing is supported.  A future version may
// support "convert" timing in high speed mode.
//
// Timing of the scan: every 1ms all channels updated at once.
//
    devpriv.ao_timer = cmd.scan_begin_arg / 1000000;
    devpriv.ao_counter = devpriv.ao_timer;
    if (cmd.start_src == TRIG_NOW) {
// enable this acquisition operation
    devpriv.ao_cmd_running = 1;
    ret = usbduxsigma_submit_urbs(dev, devpriv.ao_urbs,
    devpriv.n_ao_urbs, 0);
    if (ret < 0) {
    devpriv.ao_cmd_running = 0;
    mutex_unlock(&devpriv.mut);
    return ret;
    }
    s.async.inttrig = core::ptr::null_mut();
    } else {	/* TRIG_INT */
    s.async.inttrig = usbduxsigma_ao_inttrig;
    }
    mutex_unlock(&devpriv.mut);
    return 0;
    }
    static int usbduxsigma_dio_insn_config(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    int ret;
    ret = comedi_dio_insn_config(dev, s, insn, data, 0);
    if (ret)
    return ret;
//
// We don't tell the firmware here as it would take 8 frames
// to submit the information. We do it in the (*insn_bits).
//
    return insn.n;
    }
    static int usbduxsigma_dio_insn_bits(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    struct usbduxsigma_private *devpriv = dev.private;
    int ret;
    mutex_lock(&devpriv.mut);
    comedi_dio_update_state(s, data);
// Always update the hardware. See the (*insn_config).
    devpriv.dux_commands[1] = s.io_bits & 0xff;
    devpriv.dux_commands[4] = s.state & 0xff;
    devpriv.dux_commands[2] = (s.io_bits >> 8) & 0xff;
    devpriv.dux_commands[5] = (s.state >> 8) & 0xff;
    devpriv.dux_commands[3] = (s.io_bits >> 16) & 0xff;
    devpriv.dux_commands[6] = (s.state >> 16) & 0xff;
    ret = usbbuxsigma_send_cmd(dev, USBDUXSIGMA_DIO_BITS_CMD);
    if (ret < 0)
    goto done;
    ret = usbduxsigma_receive_cmd(dev, USBDUXSIGMA_DIO_BITS_CMD);
    if (ret < 0)
    goto done;
    s.state = devpriv.insn_buf[1] |
    (devpriv.insn_buf[2] << 8) |
    (devpriv.insn_buf[3] << 16);
    data[1] = s.state;
    ret = insn.n;
    done:
    mutex_unlock(&devpriv.mut);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn usbduxsigma_pwm_stop(dev: *mut comedi_device, do_unlink: c_int) {
    static void usbduxsigma_pwm_stop(struct comedi_device *dev, int do_unlink)
    {
    struct usbduxsigma_private *devpriv = dev.private;
    if (do_unlink) {
    if (devpriv.pwm_urb)
    usb_kill_urb(devpriv.pwm_urb);
    }
    devpriv.pwm_cmd_running = 0;
    }
    static int usbduxsigma_pwm_cancel(struct comedi_device *dev,
    struct comedi_subdevice *s)
    {
    struct usbduxsigma_private *devpriv = dev.private;
// unlink only if it is really running
    usbduxsigma_pwm_stop(dev, devpriv.pwm_cmd_running);
    return usbbuxsigma_send_cmd(dev, USBDUXSIGMA_PWM_OFF_CMD);
    }
#[no_mangle]
unsafe extern "C" fn usbduxsigma_pwm_urb_complete(urb: *mut urb) {
    static void usbduxsigma_pwm_urb_complete(struct urb *urb)
    {
    struct comedi_device *dev = urb.context;
    struct usbduxsigma_private *devpriv = dev.private;
    int ret;
    switch (urb.status) {
    case 0:
// success
    break;
    case -ECONNRESET:
    case -ENOENT:
    case -ESHUTDOWN:
    case -ECONNABORTED:
// happens after an unlink command
    if (devpriv.pwm_cmd_running)
    usbduxsigma_pwm_stop(dev, 0);	/* w/o unlink */
    return;
    default:
// a real error
    if (devpriv.pwm_cmd_running) {
    dev_err(dev.class_dev, "non-zero urb status (%d)\n",
    urb.status);
    usbduxsigma_pwm_stop(dev, 0);	/* w/o unlink */
    }
    return;
    }
    if (!devpriv.pwm_cmd_running)
    return;
    urb.transfer_buffer_length = devpriv.pwm_buf_sz;
    urb.dev = comedi_to_usb_dev(dev);
    urb.status = 0;
    ret = usb_submit_urb(urb, GFP_ATOMIC);
    if (ret < 0) {
    dev_err(dev.class_dev, "urb resubmit failed (%d)\n", ret);
    if (ret == -EL2NSYNC)
    dev_err(dev.class_dev,
    "buggy USB host controller or bug in IRQ handler\n");
    usbduxsigma_pwm_stop(dev, 0);	/* w/o unlink */
    }
    }
#[no_mangle]
unsafe extern "C" fn usbduxsigma_submit_pwm_urb(dev: *mut comedi_device) -> c_int {
    static int usbduxsigma_submit_pwm_urb(struct comedi_device *dev)
    {
    struct usb_device *usb = comedi_to_usb_dev(dev);
    struct usbduxsigma_private *devpriv = dev.private;
    struct urb *urb = devpriv.pwm_urb;
// in case of a resubmission after an unlink...
    usb_fill_bulk_urb(urb, usb, usb_sndbulkpipe(usb, 4),
    urb.transfer_buffer, devpriv.pwm_buf_sz,
    usbduxsigma_pwm_urb_complete, dev);
    return usb_submit_urb(urb, GFP_ATOMIC);
    }
    static int usbduxsigma_pwm_period(struct comedi_device *dev,
    struct comedi_subdevice *s,
    unsigned int period)
    {
    struct usbduxsigma_private *devpriv = dev.private;
    int fx2delay;
    if (period < MIN_PWM_PERIOD)
    return -EAGAIN;
    fx2delay = (period / (6 * 512 * 1000 / 33)) - 6;
    if (fx2delay > 255)
    return -EAGAIN;
    devpriv.pwm_delay = fx2delay;
    devpriv.pwm_period = period;
    return 0;
    }
    static int usbduxsigma_pwm_start(struct comedi_device *dev,
    struct comedi_subdevice *s)
    {
    struct usbduxsigma_private *devpriv = dev.private;
    int ret;
    if (devpriv.pwm_cmd_running)
    return 0;
    devpriv.dux_commands[1] = devpriv.pwm_delay;
    ret = usbbuxsigma_send_cmd(dev, USBDUXSIGMA_PWM_ON_CMD);
    if (ret < 0)
    return ret;
    memset(devpriv.pwm_urb.transfer_buffer, 0, devpriv.pwm_buf_sz);
    devpriv.pwm_cmd_running = 1;
    ret = usbduxsigma_submit_pwm_urb(dev);
    if (ret < 0) {
    devpriv.pwm_cmd_running = 0;
    return ret;
    }
    return 0;
    }
    static void usbduxsigma_pwm_pattern(struct comedi_device *dev,
    struct comedi_subdevice *s,
    unsigned int chan,
    unsigned int value,
    unsigned int sign)
    {
    struct usbduxsigma_private *devpriv = dev.private;
    char pwm_mask = (1 << chan);	/* DIO bit for the PWM data */
    char sgn_mask = (16 << chan);	/* DIO bit for the sign */
    char *buf = (char *)(devpriv.pwm_urb.transfer_buffer);
    let mut szbuf: c_int = devpriv.pwm_buf_sz;
    int i;
    for (i = 0; i < szbuf; i++) {
    let mut c: c_char = *buf;
    c &= ~pwm_mask;
    if (i < value)
    c |= pwm_mask;
    if (!sign)
    c &= ~sgn_mask;
    else
    c |= sgn_mask;
// buf++ = c;
    }
    }
    static int usbduxsigma_pwm_write(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    let mut chan: c_uint = CR_CHAN(insn.chanspec);
//
// It doesn't make sense to support more than one value here
// because it would just overwrite the PWM buffer.
//
    if (insn.n != 1)
    return -EINVAL;
//
// The sign is set via a special INSN only, this gives us 8 bits
// for normal operation, sign is 0 by default.
//
    usbduxsigma_pwm_pattern(dev, s, chan, data[0], 0);
    return insn.n;
    }
    static int usbduxsigma_pwm_config(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    struct usbduxsigma_private *devpriv = dev.private;
    let mut chan: c_uint = CR_CHAN(insn.chanspec);
    switch (data[0]) {
    case INSN_CONFIG_ARM:
//
// if not zero the PWM is limited to a certain time which is
// not supported here
//
    if (data[1] != 0)
    return -EINVAL;
    return usbduxsigma_pwm_start(dev, s);
    case INSN_CONFIG_DISARM:
    return usbduxsigma_pwm_cancel(dev, s);
    case INSN_CONFIG_GET_PWM_STATUS:
    data[1] = devpriv.pwm_cmd_running;
    return 0;
    case INSN_CONFIG_PWM_SET_PERIOD:
    return usbduxsigma_pwm_period(dev, s, data[1]);
    case INSN_CONFIG_PWM_GET_PERIOD:
    data[1] = devpriv.pwm_period;
    return 0;
    case INSN_CONFIG_PWM_SET_H_BRIDGE:
//
// data[1] = value
// data[2] = sign (for a relay)
//
    usbduxsigma_pwm_pattern(dev, s, chan, data[1], (data[2] != 0));
    return 0;
    case INSN_CONFIG_PWM_GET_H_BRIDGE:
// values are not kept in this driver, nothing to return
    return -EINVAL;
    }
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn usbduxsigma_getstatusinfo(dev: *mut comedi_device, chan: c_int) -> c_int {
    static int usbduxsigma_getstatusinfo(struct comedi_device *dev, int chan)
    {
    struct comedi_subdevice *s = dev.read_subdev;
    struct usbduxsigma_private *devpriv = dev.private;
    u8 sysred;
    u32 val;
    int ret;
    switch (chan) {
    default:
    case 0:
    sysred = 0;		/* ADC zero */
    break;
    case 1:
    sysred = 1;		/* ADC offset */
    break;
    case 2:
    sysred = 4;		/* VCC */
    break;
    case 3:
    sysred = 8;		/* temperature */
    break;
    case 4:
    sysred = 16;		/* gain */
    break;
    case 5:
    sysred =  32;		/* ref */
    break;
    }
    devpriv.dux_commands[1] = 0x12; /* CONFIG0 */
    devpriv.dux_commands[2] = 0x80; /* CONFIG1: 2kHz sampling rate */
    devpriv.dux_commands[3] = 0x00; /* CONFIG3: diff. channels off */
    devpriv.dux_commands[4] = 0;
    devpriv.dux_commands[5] = 0;
    devpriv.dux_commands[6] = sysred;
    ret = usbbuxsigma_send_cmd(dev, USBDUXSIGMA_SINGLE_AD_CMD);
    if (ret < 0)
    return ret;
    ret = usbduxsigma_receive_cmd(dev, USBDUXSIGMA_SINGLE_AD_CMD);
    if (ret < 0)
    return ret;
// 32 bits big endian from the A/D converter
    val = be32_to_cpu(get_unaligned((__be32 *)(devpriv.insn_buf + 1)));
    val &= 0x00ffffff;	/* strip status byte */
    return (int)comedi_offset_munge(s, val);
    }
    static int usbduxsigma_firmware_upload(struct comedi_device *dev,
    const u8 *data, size_t size,
    unsigned long context)
    {
    struct usb_device *usb = comedi_to_usb_dev(dev);
    u8 *buf;
    u8 *tmp;
    int ret;
    if (!data)
    return 0;
    if (size > FIRMWARE_MAX_LEN) {
    dev_err(dev.class_dev, "firmware binary too large for FX2\n");
    return -ENOMEM;
    }
// we generate a local buffer for the firmware
    buf = kmemdup(data, size, GFP_KERNEL);
    if (!buf)
    return -ENOMEM;
// we need a malloc'ed buffer for usb_control_msg()
    tmp = kmalloc(1, GFP_KERNEL);
    if (!tmp) {
    kfree(buf);
    return -ENOMEM;
    }
// stop the current firmware on the device
// tmp = 1;	/* 7f92 to one
    ret = usb_control_msg(usb, usb_sndctrlpipe(usb, 0),
    USBDUXSUB_FIRMWARE,
    VENDOR_DIR_OUT,
    USBDUXSUB_CPUCS, 0x0000,
    tmp, 1,
    BULK_TIMEOUT);
    if (ret < 0) {
    dev_err(dev.class_dev, "can not stop firmware\n");
    goto done;
    }
// upload the new firmware to the device
    ret = usb_control_msg(usb, usb_sndctrlpipe(usb, 0),
    USBDUXSUB_FIRMWARE,
    VENDOR_DIR_OUT,
    0, 0x0000,
    buf, size,
    BULK_TIMEOUT);
    if (ret < 0) {
    dev_err(dev.class_dev, "firmware upload failed\n");
    goto done;
    }
// start the new firmware on the device
// tmp = 0;	/* 7f92 to zero
    ret = usb_control_msg(usb, usb_sndctrlpipe(usb, 0),
    USBDUXSUB_FIRMWARE,
    VENDOR_DIR_OUT,
    USBDUXSUB_CPUCS, 0x0000,
    tmp, 1,
    BULK_TIMEOUT);
    if (ret < 0)
    dev_err(dev.class_dev, "can not start firmware\n");
    done:
    kfree(tmp);
    kfree(buf);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn usbduxsigma_alloc_usb_buffers(dev: *mut comedi_device) -> c_int {
    static int usbduxsigma_alloc_usb_buffers(struct comedi_device *dev)
    {
    struct usb_device *usb = comedi_to_usb_dev(dev);
    struct usbduxsigma_private *devpriv = dev.private;
    struct urb *urb;
    int i;
    devpriv.dux_commands = kzalloc(SIZEOFDUXBUFFER, GFP_KERNEL);
    devpriv.in_buf = kzalloc(SIZEINBUF, GFP_KERNEL);
    devpriv.insn_buf = kzalloc(SIZEINSNBUF, GFP_KERNEL);
    devpriv.ai_urbs = kzalloc_objs(urb, devpriv.n_ai_urbs);
    devpriv.ao_urbs = kzalloc_objs(urb, devpriv.n_ao_urbs);
    if (!devpriv.dux_commands || !devpriv.in_buf || !devpriv.insn_buf ||
    !devpriv.ai_urbs || !devpriv.ao_urbs)
    return -ENOMEM;
    for (i = 0; i < devpriv.n_ai_urbs; i++) {
// one frame: 1ms
    urb = usb_alloc_urb(1, GFP_KERNEL);
    if (!urb)
    return -ENOMEM;
    devpriv.ai_urbs[i] = urb;
    urb.dev = usb;
// will be filled later with a pointer to the comedi-device
// and ONLY then the urb should be submitted
    urb.context = core::ptr::null_mut();
    urb.pipe = usb_rcvisocpipe(usb, 6);
    urb.transfer_flags = URB_ISO_ASAP;
    urb.transfer_buffer = kzalloc(SIZEINBUF, GFP_KERNEL);
    if (!urb.transfer_buffer)
    return -ENOMEM;
    urb.complete = usbduxsigma_ai_urb_complete;
    urb.number_of_packets = 1;
    urb.transfer_buffer_length = SIZEINBUF;
    urb.iso_frame_desc[0].offset = 0;
    urb.iso_frame_desc[0].length = SIZEINBUF;
    }
    for (i = 0; i < devpriv.n_ao_urbs; i++) {
// one frame: 1ms
    urb = usb_alloc_urb(1, GFP_KERNEL);
    if (!urb)
    return -ENOMEM;
    devpriv.ao_urbs[i] = urb;
    urb.dev = usb;
// will be filled later with a pointer to the comedi-device
// and ONLY then the urb should be submitted
    urb.context = core::ptr::null_mut();
    urb.pipe = usb_sndisocpipe(usb, 2);
    urb.transfer_flags = URB_ISO_ASAP;
    urb.transfer_buffer = kzalloc(SIZEOUTBUF, GFP_KERNEL);
    if (!urb.transfer_buffer)
    return -ENOMEM;
    urb.complete = usbduxsigma_ao_urb_complete;
    urb.number_of_packets = 1;
    urb.transfer_buffer_length = SIZEOUTBUF;
    urb.iso_frame_desc[0].offset = 0;
    urb.iso_frame_desc[0].length = SIZEOUTBUF;
    urb.interval = 1;	/* (u)frames */
    }
    if (devpriv.pwm_buf_sz) {
    urb = usb_alloc_urb(0, GFP_KERNEL);
    if (!urb)
    return -ENOMEM;
    devpriv.pwm_urb = urb;
    urb.transfer_buffer = kzalloc(devpriv.pwm_buf_sz,
    GFP_KERNEL);
    if (!urb.transfer_buffer)
    return -ENOMEM;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn usbduxsigma_free_usb_buffers(dev: *mut comedi_device) {
    static void usbduxsigma_free_usb_buffers(struct comedi_device *dev)
    {
    struct usbduxsigma_private *devpriv = dev.private;
    struct urb *urb;
    int i;
    urb = devpriv.pwm_urb;
    if (urb) {
    kfree(urb.transfer_buffer);
    usb_free_urb(urb);
    }
    if (devpriv.ao_urbs) {
    for (i = 0; i < devpriv.n_ao_urbs; i++) {
    urb = devpriv.ao_urbs[i];
    if (urb) {
    kfree(urb.transfer_buffer);
    usb_free_urb(urb);
    }
    }
    kfree(devpriv.ao_urbs);
    }
    if (devpriv.ai_urbs) {
    for (i = 0; i < devpriv.n_ai_urbs; i++) {
    urb = devpriv.ai_urbs[i];
    if (urb) {
    kfree(urb.transfer_buffer);
    usb_free_urb(urb);
    }
    }
    kfree(devpriv.ai_urbs);
    }
    kfree(devpriv.insn_buf);
    kfree(devpriv.in_buf);
    kfree(devpriv.dux_commands);
    }
    static int usbduxsigma_auto_attach(struct comedi_device *dev,
    unsigned long context_unused)
    {
    struct usb_interface *intf = comedi_to_usb_interface(dev);
    struct usb_device *usb = comedi_to_usb_dev(dev);
    struct usbduxsigma_private *devpriv;
    struct comedi_subdevice *s;
    int offset;
    int ret;
    devpriv = comedi_alloc_devpriv(dev, sizeof(*devpriv));
    if (!devpriv)
    return -ENOMEM;
    mutex_init(&devpriv.mut);
    usb_set_intfdata(intf, devpriv);
    devpriv.high_speed = (usb.speed == USB_SPEED_HIGH);
    if (devpriv.high_speed) {
    devpriv.n_ai_urbs = NUMOFINBUFFERSHIGH;
    devpriv.n_ao_urbs = NUMOFOUTBUFFERSHIGH;
    devpriv.pwm_buf_sz = 512;
    } else {
    devpriv.n_ai_urbs = NUMOFINBUFFERSFULL;
    devpriv.n_ao_urbs = NUMOFOUTBUFFERSFULL;
    }
    ret = usbduxsigma_alloc_usb_buffers(dev);
    if (ret)
    return ret;
// setting to alternate setting 3: enabling iso ep and bulk ep.
    ret = usb_set_interface(usb, intf.altsetting.desc.bInterfaceNumber,
    3);
    if (ret < 0) {
    dev_err(dev.class_dev,
    "could not set alternate setting 3 in high speed\n");
    return ret;
    }
    ret = comedi_load_firmware(dev, &usb.dev, FIRMWARE,
    usbduxsigma_firmware_upload, 0);
    if (ret)
    return ret;
    ret = comedi_alloc_subdevices(dev, (devpriv.high_speed) ? 4 : 3);
    if (ret)
    return ret;
// Analog Input subdevice
    s = &dev.subdevices[0];
    dev.read_subdev = s;
    s.type		= COMEDI_SUBD_AI;
    s.subdev_flags	= SDF_READABLE | SDF_GROUND | SDF_CMD_READ | SDF_LSAMPL;
    s.n_chan	= NUMCHANNELS;
    s.len_chanlist	= NUMCHANNELS;
    s.maxdata	= 0x00ffffff;
    s.range_table	= &usbduxsigma_ai_range;
    s.insn_read	= usbduxsigma_ai_insn_read;
    s.do_cmdtest	= usbduxsigma_ai_cmdtest;
    s.do_cmd	= usbduxsigma_ai_cmd;
    s.cancel	= usbduxsigma_ai_cancel;
// Analog Output subdevice
    s = &dev.subdevices[1];
    dev.write_subdev = s;
    s.type		= COMEDI_SUBD_AO;
    s.subdev_flags	= SDF_WRITABLE | SDF_GROUND | SDF_CMD_WRITE;
    s.n_chan	= 4;
    s.len_chanlist	= s.n_chan;
    s.maxdata	= 0x00ff;
    s.range_table	= &range_unipolar2_5;
    s.insn_write	= usbduxsigma_ao_insn_write;
    s.insn_read	= usbduxsigma_ao_insn_read;
    s.do_cmdtest	= usbduxsigma_ao_cmdtest;
    s.do_cmd	= usbduxsigma_ao_cmd;
    s.cancel	= usbduxsigma_ao_cancel;
    ret = comedi_alloc_subdev_readback(s);
    if (ret)
    return ret;
// Digital I/O subdevice
    s = &dev.subdevices[2];
    s.type		= COMEDI_SUBD_DIO;
    s.subdev_flags	= SDF_READABLE | SDF_WRITABLE;
    s.n_chan	= 24;
    s.maxdata	= 1;
    s.range_table	= &range_digital;
    s.insn_bits	= usbduxsigma_dio_insn_bits;
    s.insn_config	= usbduxsigma_dio_insn_config;
    if (devpriv.high_speed) {
// Timer / pwm subdevice
    s = &dev.subdevices[3];
    s.type		= COMEDI_SUBD_PWM;
    s.subdev_flags	= SDF_WRITABLE | SDF_PWM_HBRIDGE;
    s.n_chan	= 8;
    s.maxdata	= devpriv.pwm_buf_sz;
    s.insn_write	= usbduxsigma_pwm_write;
    s.insn_config	= usbduxsigma_pwm_config;
    usbduxsigma_pwm_period(dev, s, PWM_DEFAULT_PERIOD);
    }
    offset = usbduxsigma_getstatusinfo(dev, 0);
    if (offset < 0) {
    dev_err(dev.class_dev,
    "Communication to USBDUXSIGMA failed! Check firmware and cabling.\n");
    return offset;
    }
    dev_info(dev.class_dev, "ADC_zero = %x\n", offset);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn usbduxsigma_detach(dev: *mut comedi_device) {
    static void usbduxsigma_detach(struct comedi_device *dev)
    {
    struct usb_interface *intf = comedi_to_usb_interface(dev);
    struct usbduxsigma_private *devpriv = dev.private;
    usb_set_intfdata(intf, core::ptr::null_mut());
    if (!devpriv)
    return;
    mutex_lock(&devpriv.mut);
// force unlink all urbs
    usbduxsigma_ai_stop(dev, 1);
    usbduxsigma_ao_stop(dev, 1);
    usbduxsigma_pwm_stop(dev, 1);
    usbduxsigma_free_usb_buffers(dev);
    mutex_unlock(&devpriv.mut);
    mutex_destroy(&devpriv.mut);
    }
    static struct comedi_driver usbduxsigma_driver = {
    .driver_name	= "usbduxsigma",
    .module		= THIS_MODULE,
    .auto_attach	= usbduxsigma_auto_attach,
    .detach		= usbduxsigma_detach,
    };
    static int usbduxsigma_usb_probe(struct usb_interface *intf,
    const struct usb_device_id *id)
    {
    return comedi_usb_auto_config(intf, &usbduxsigma_driver, 0);
    }
    static const struct usb_device_id usbduxsigma_usb_table[] = {
    { USB_DEVICE(0x13d8, 0x0020) },
    { USB_DEVICE(0x13d8, 0x0021) },
    { USB_DEVICE(0x13d8, 0x0022) },
    { }
    };
    MODULE_DEVICE_TABLE(usb, usbduxsigma_usb_table);
    static struct usb_driver usbduxsigma_usb_driver = {
    .name		= "usbduxsigma",
    .probe		= usbduxsigma_usb_probe,
    .disconnect	= comedi_usb_auto_unconfig,
    .id_table	= usbduxsigma_usb_table,
    };
    module_comedi_usb_driver(usbduxsigma_driver, usbduxsigma_usb_driver);
    MODULE_AUTHOR("Bernd Porr, mail@berndporr.me.uk");
    MODULE_DESCRIPTION("Stirling/ITL USB-DUX SIGMA -- mail@berndporr.me.uk");
    MODULE_LICENSE("GPL");
    MODULE_FIRMWARE(FIRMWARE);
