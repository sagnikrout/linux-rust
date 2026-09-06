//! Automatically rewritten from C to Rust
//! Source: drivers/comedi/drivers/vmk80xx.c
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
// vmk80xx.c
// Velleman USB Board Low-Level Driver
//
// Copyright (C) 2009 Manuel Gebele <forensixs@gmx.de>, Germany
//
// COMEDI - Linux Control and Measurement Device Interface
// Copyright (C) 2000 David A. Schleef <ds@schleef.org>
//
// Driver: vmk80xx
// Description: Velleman USB Board Low-Level Driver
// Devices: [Velleman] K8055 (K8055/VM110), K8061 (K8061/VM140),
// VM110 (K8055/VM110), VM140 (K8061/VM140)
// Author: Manuel Gebele <forensixs@gmx.de>
// Updated: Sun, 10 May 2009 11:14:59 +0200
// Status: works
//
// Supports:
// - analog input
// - analog output
// - digital input
// - digital output
// - counter
// - pwm
//

    enum {
    DEVICE_VMK8055,
    DEVICE_VMK8061
    };
pub const VMK8055_DI_REG: c_uint = 0x00;
pub const VMK8055_DO_REG: c_uint = 0x01;
pub const VMK8055_AO1_REG: c_uint = 0x02;
pub const VMK8055_AO2_REG: c_uint = 0x03;
pub const VMK8055_AI1_REG: c_uint = 0x02;
pub const VMK8055_AI2_REG: c_uint = 0x03;
pub const VMK8055_CNT1_REG: c_uint = 0x04;
pub const VMK8055_CNT2_REG: c_uint = 0x06;
pub const VMK8061_CH_REG: c_uint = 0x01;
pub const VMK8061_DI_REG: c_uint = 0x01;
pub const VMK8061_DO_REG: c_uint = 0x01;
pub const VMK8061_PWM_REG1: c_uint = 0x01;
pub const VMK8061_PWM_REG2: c_uint = 0x02;
pub const VMK8061_CNT_REG: c_uint = 0x02;
pub const VMK8061_AO_REG: c_uint = 0x02;
pub const VMK8061_AI_REG1: c_uint = 0x02;
pub const VMK8061_AI_REG2: c_uint = 0x03;
pub const VMK8055_CMD_RST: c_uint = 0x00;
pub const VMK8055_CMD_DEB1_TIME: c_uint = 0x01;
pub const VMK8055_CMD_DEB2_TIME: c_uint = 0x02;
pub const VMK8055_CMD_RST_CNT1: c_uint = 0x03;
pub const VMK8055_CMD_RST_CNT2: c_uint = 0x04;
pub const VMK8055_CMD_WRT_AD: c_uint = 0x05;
pub const VMK8061_CMD_RD_AI: c_uint = 0x00;
pub const VMK8061_CMR_RD_ALL_AI: c_uint = 0x01	/* !non-active! */;
pub const VMK8061_CMD_SET_AO: c_uint = 0x02;
pub const VMK8061_CMD_SET_ALL_AO: c_uint = 0x03	/* !non-active! */;
pub const VMK8061_CMD_OUT_PWM: c_uint = 0x04;
pub const VMK8061_CMD_RD_DI: c_uint = 0x05;
pub const VMK8061_CMD_DO: c_uint = 0x06	/* !non-active! */;
pub const VMK8061_CMD_CLR_DO: c_uint = 0x07;
pub const VMK8061_CMD_SET_DO: c_uint = 0x08;
pub const VMK8061_CMD_RD_CNT: c_uint = 0x09	/* TODO: completely pointless? */;
pub const VMK8061_CMD_RST_CNT: c_uint = 0x0a	/* TODO: completely pointless? */;
pub const VMK8061_CMD_RD_VERSION: c_uint = 0x0b	/* internal usage */;
pub const VMK8061_CMD_RD_JMP_STAT: c_uint = 0x0c	/* TODO: not implemented yet */;
pub const VMK8061_CMD_RD_PWR_STAT: c_uint = 0x0d	/* internal usage */;
pub const VMK8061_CMD_RD_DO: c_uint = 0x0e;
pub const VMK8061_CMD_RD_AO: c_uint = 0x0f;
pub const VMK8061_CMD_RD_PWM: c_uint = 0x10;

pub const MIN_BUF_SIZE: c_int = 64;

    enum vmk80xx_model {
    VMK8055_MODEL,
    VMK8061_MODEL
    };
    static const struct comedi_lrange vmk8061_range = {
    2, {
    UNI_RANGE(5),
    UNI_RANGE(10)
    }
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmk80xx_board {
    pub name: *const c_char,
    pub model: enum vmk80xx_model,
    pub range: *const comedi_lrange,
    pub ai_nchans: c_int,
    pub ai_maxdata: c_uint,
    pub ao_nchans: c_int,
    pub di_nchans: c_int,
    pub cnt_maxdata: c_uint,
    pub pwm_nchans: c_int,
    pub pwm_maxdata: c_uint,
}

    static const struct vmk80xx_board vmk80xx_boardinfo[] = {
    [DEVICE_VMK8055] = {
    .name		= "K8055 (VM110)",
    .model		= VMK8055_MODEL,
    .range		= &range_unipolar5,
    .ai_nchans	= 2,
    .ai_maxdata	= 0x00ff,
    .ao_nchans	= 2,
    .di_nchans	= 6,
    .cnt_maxdata	= 0xffff,
    },
    [DEVICE_VMK8061] = {
    .name		= "K8061 (VM140)",
    .model		= VMK8061_MODEL,
    .range		= &vmk8061_range,
    .ai_nchans	= 8,
    .ai_maxdata	= 0x03ff,
    .ao_nchans	= 8,
    .di_nchans	= 8,
    .cnt_maxdata	= 0,	/* unknown, device is not writeable */
    .pwm_nchans	= 1,
    .pwm_maxdata	= 0x03ff,
    },
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmk80xx_private {
    pub ep_rx: *mut usb_endpoint_descriptor,
    pub ep_tx: *mut usb_endpoint_descriptor,
    pub limit_sem: semaphore,
    pub usb_rx_buf: *mut c_uchar,
    pub usb_tx_buf: *mut c_uchar,
    pub model: enum vmk80xx_model,
}

#[no_mangle]
unsafe extern "C" fn vmk80xx_do_bulk_msg(dev: *mut comedi_device) {
    static void vmk80xx_do_bulk_msg(struct comedi_device *dev)
    {
    struct vmk80xx_private *devpriv = dev.private;
    struct usb_device *usb = comedi_to_usb_dev(dev);
    __u8 tx_addr;
    __u8 rx_addr;
    unsigned int tx_pipe;
    unsigned int rx_pipe;
    size_t tx_size;
    size_t rx_size;
    tx_addr = devpriv.ep_tx.bEndpointAddress;
    rx_addr = devpriv.ep_rx.bEndpointAddress;
    tx_pipe = usb_sndbulkpipe(usb, tx_addr);
    rx_pipe = usb_rcvbulkpipe(usb, rx_addr);
    tx_size = usb_endpoint_maxp(devpriv.ep_tx);
    rx_size = usb_endpoint_maxp(devpriv.ep_rx);
    usb_bulk_msg(usb, tx_pipe, devpriv.usb_tx_buf, tx_size, core::ptr::null_mut(),
    PACKET_TIMEOUT);
    usb_bulk_msg(usb, rx_pipe, devpriv.usb_rx_buf, rx_size, core::ptr::null_mut(),
    PACKET_TIMEOUT);
    }
#[no_mangle]
unsafe extern "C" fn vmk80xx_read_packet(dev: *mut comedi_device) -> c_int {
    static int vmk80xx_read_packet(struct comedi_device *dev)
    {
    struct vmk80xx_private *devpriv = dev.private;
    struct usb_device *usb = comedi_to_usb_dev(dev);
    struct usb_endpoint_descriptor *ep;
    unsigned int pipe;
    if (devpriv.model == VMK8061_MODEL) {
    vmk80xx_do_bulk_msg(dev);
    return 0;
    }
    ep = devpriv.ep_rx;
    pipe = usb_rcvintpipe(usb, ep.bEndpointAddress);
    return usb_interrupt_msg(usb, pipe, devpriv.usb_rx_buf,
    usb_endpoint_maxp(ep), core::ptr::null_mut(),
    PACKET_TIMEOUT);
    }
#[no_mangle]
unsafe extern "C" fn vmk80xx_write_packet(dev: *mut comedi_device, cmd: c_int) -> c_int {
    static int vmk80xx_write_packet(struct comedi_device *dev, int cmd)
    {
    struct vmk80xx_private *devpriv = dev.private;
    struct usb_device *usb = comedi_to_usb_dev(dev);
    struct usb_endpoint_descriptor *ep;
    unsigned int pipe;
    devpriv.usb_tx_buf[0] = cmd;
    if (devpriv.model == VMK8061_MODEL) {
    vmk80xx_do_bulk_msg(dev);
    return 0;
    }
    ep = devpriv.ep_tx;
    pipe = usb_sndintpipe(usb, ep.bEndpointAddress);
    return usb_interrupt_msg(usb, pipe, devpriv.usb_tx_buf,
    usb_endpoint_maxp(ep), core::ptr::null_mut(),
    PACKET_TIMEOUT);
    }
#[no_mangle]
unsafe extern "C" fn vmk80xx_reset_device(dev: *mut comedi_device) -> c_int {
    static int vmk80xx_reset_device(struct comedi_device *dev)
    {
    struct vmk80xx_private *devpriv = dev.private;
    size_t size;
    int retval;
    size = usb_endpoint_maxp(devpriv.ep_tx);
    memset(devpriv.usb_tx_buf, 0, size);
    retval = vmk80xx_write_packet(dev, VMK8055_CMD_RST);
    if (retval)
    return retval;
// set outputs to known state as we cannot read them
    return vmk80xx_write_packet(dev, VMK8055_CMD_WRT_AD);
    }
    static int vmk80xx_ai_insn_read(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    struct vmk80xx_private *devpriv = dev.private;
    int chan;
    int reg[2];
    int n;
    down(&devpriv.limit_sem);
    chan = CR_CHAN(insn.chanspec);
    switch (devpriv.model) {
    case VMK8055_MODEL:
    if (!chan)
    reg[0] = VMK8055_AI1_REG;
    else
    reg[0] = VMK8055_AI2_REG;
    break;
    case VMK8061_MODEL:
    default:
    reg[0] = VMK8061_AI_REG1;
    reg[1] = VMK8061_AI_REG2;
    devpriv.usb_tx_buf[0] = VMK8061_CMD_RD_AI;
    devpriv.usb_tx_buf[VMK8061_CH_REG] = chan;
    break;
    }
    for (n = 0; n < insn.n; n++) {
    if (vmk80xx_read_packet(dev))
    break;
    if (devpriv.model == VMK8055_MODEL) {
    data[n] = devpriv.usb_rx_buf[reg[0]];
    continue;
    }
// VMK8061_MODEL
    data[n] = devpriv.usb_rx_buf[reg[0]] + 256 *
    devpriv.usb_rx_buf[reg[1]];
    }
    up(&devpriv.limit_sem);
    return n;
    }
    static int vmk80xx_ao_insn_write(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    struct vmk80xx_private *devpriv = dev.private;
    int chan;
    int cmd;
    int reg;
    int n;
    down(&devpriv.limit_sem);
    chan = CR_CHAN(insn.chanspec);
    switch (devpriv.model) {
    case VMK8055_MODEL:
    cmd = VMK8055_CMD_WRT_AD;
    if (!chan)
    reg = VMK8055_AO1_REG;
    else
    reg = VMK8055_AO2_REG;
    break;
    default:		/* NOTE: avoid compiler warnings */
    cmd = VMK8061_CMD_SET_AO;
    reg = VMK8061_AO_REG;
    devpriv.usb_tx_buf[VMK8061_CH_REG] = chan;
    break;
    }
    for (n = 0; n < insn.n; n++) {
    devpriv.usb_tx_buf[reg] = data[n];
    if (vmk80xx_write_packet(dev, cmd))
    break;
    }
    up(&devpriv.limit_sem);
    return n;
    }
    static int vmk80xx_ao_insn_read(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    struct vmk80xx_private *devpriv = dev.private;
    int chan;
    int reg;
    int n;
    down(&devpriv.limit_sem);
    chan = CR_CHAN(insn.chanspec);
    reg = VMK8061_AO_REG - 1;
    devpriv.usb_tx_buf[0] = VMK8061_CMD_RD_AO;
    for (n = 0; n < insn.n; n++) {
    if (vmk80xx_read_packet(dev))
    break;
    data[n] = devpriv.usb_rx_buf[reg + chan];
    }
    up(&devpriv.limit_sem);
    return n;
    }
    static int vmk80xx_di_insn_bits(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    struct vmk80xx_private *devpriv = dev.private;
    unsigned char *rx_buf;
    int reg;
    int retval;
    down(&devpriv.limit_sem);
    rx_buf = devpriv.usb_rx_buf;
    if (devpriv.model == VMK8061_MODEL) {
    reg = VMK8061_DI_REG;
    devpriv.usb_tx_buf[0] = VMK8061_CMD_RD_DI;
    } else {
    reg = VMK8055_DI_REG;
    }
    retval = vmk80xx_read_packet(dev);
    if (!retval) {
    if (devpriv.model == VMK8055_MODEL)
    data[1] = (((rx_buf[reg] >> 4) & 0x03) |
    ((rx_buf[reg] << 2) & 0x04) |
    ((rx_buf[reg] >> 3) & 0x18));
    else
    data[1] = rx_buf[reg];
    retval = 2;
    }
    up(&devpriv.limit_sem);
    return retval;
    }
    static int vmk80xx_do_insn_bits(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    struct vmk80xx_private *devpriv = dev.private;
    unsigned char *rx_buf = devpriv.usb_rx_buf;
    unsigned char *tx_buf = devpriv.usb_tx_buf;
    int reg, cmd;
    let mut ret: c_int = 0;
    if (devpriv.model == VMK8061_MODEL) {
    reg = VMK8061_DO_REG;
    cmd = VMK8061_CMD_DO;
    } else { /* VMK8055_MODEL */
    reg = VMK8055_DO_REG;
    cmd = VMK8055_CMD_WRT_AD;
    }
    down(&devpriv.limit_sem);
    if (comedi_dio_update_state(s, data)) {
    tx_buf[reg] = s.state;
    ret = vmk80xx_write_packet(dev, cmd);
    if (ret)
    goto out;
    }
    if (devpriv.model == VMK8061_MODEL) {
    tx_buf[0] = VMK8061_CMD_RD_DO;
    ret = vmk80xx_read_packet(dev);
    if (ret)
    goto out;
    data[1] = rx_buf[reg];
    } else {
    data[1] = s.state;
    }
    out:
    up(&devpriv.limit_sem);
    return ret ? ret : insn.n;
    }
    static int vmk80xx_cnt_insn_read(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    struct vmk80xx_private *devpriv = dev.private;
    int chan;
    int reg[2];
    int n;
    down(&devpriv.limit_sem);
    chan = CR_CHAN(insn.chanspec);
    switch (devpriv.model) {
    case VMK8055_MODEL:
    if (!chan)
    reg[0] = VMK8055_CNT1_REG;
    else
    reg[0] = VMK8055_CNT2_REG;
    break;
    case VMK8061_MODEL:
    default:
    reg[0] = VMK8061_CNT_REG;
    reg[1] = VMK8061_CNT_REG;
    devpriv.usb_tx_buf[0] = VMK8061_CMD_RD_CNT;
    break;
    }
    for (n = 0; n < insn.n; n++) {
    if (vmk80xx_read_packet(dev))
    break;
    if (devpriv.model == VMK8055_MODEL)
    data[n] = devpriv.usb_rx_buf[reg[0]];
    else /* VMK8061_MODEL */
    data[n] = devpriv.usb_rx_buf[reg[0] * (chan + 1) + 1]
    + 256 * devpriv.usb_rx_buf[reg[1] * 2 + 2];
    }
    up(&devpriv.limit_sem);
    return n;
    }
    static int vmk80xx_cnt_insn_config(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    struct vmk80xx_private *devpriv = dev.private;
    let mut chan: c_uint = CR_CHAN(insn.chanspec);
    int cmd;
    int reg;
    int ret;
    down(&devpriv.limit_sem);
    switch (data[0]) {
    case INSN_CONFIG_RESET:
    if (devpriv.model == VMK8055_MODEL) {
    if (!chan) {
    cmd = VMK8055_CMD_RST_CNT1;
    reg = VMK8055_CNT1_REG;
    } else {
    cmd = VMK8055_CMD_RST_CNT2;
    reg = VMK8055_CNT2_REG;
    }
    devpriv.usb_tx_buf[reg] = 0x00;
    } else {
    cmd = VMK8061_CMD_RST_CNT;
    }
    ret = vmk80xx_write_packet(dev, cmd);
    break;
    default:
    ret = -EINVAL;
    break;
    }
    up(&devpriv.limit_sem);
    return ret ? ret : insn.n;
    }
    static int vmk80xx_cnt_insn_write(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    struct vmk80xx_private *devpriv = dev.private;
    unsigned long debtime;
    unsigned long val;
    int chan;
    int cmd;
    int n;
    down(&devpriv.limit_sem);
    chan = CR_CHAN(insn.chanspec);
    if (!chan)
    cmd = VMK8055_CMD_DEB1_TIME;
    else
    cmd = VMK8055_CMD_DEB2_TIME;
    for (n = 0; n < insn.n; n++) {
    debtime = data[n];
    if (debtime == 0)
    debtime = 1;
// TODO: Prevent overflows
    if (debtime > 7450)
    debtime = 7450;
    val = int_sqrt(debtime * 1000 / 115);
    if (((val + 1) * val) < debtime * 1000 / 115)
    val += 1;
    devpriv.usb_tx_buf[6 + chan] = val;
    if (vmk80xx_write_packet(dev, cmd))
    break;
    }
    up(&devpriv.limit_sem);
    return n;
    }
    static int vmk80xx_pwm_insn_read(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    struct vmk80xx_private *devpriv = dev.private;
    unsigned char *tx_buf;
    unsigned char *rx_buf;
    int reg[2];
    int n;
    down(&devpriv.limit_sem);
    tx_buf = devpriv.usb_tx_buf;
    rx_buf = devpriv.usb_rx_buf;
    reg[0] = VMK8061_PWM_REG1;
    reg[1] = VMK8061_PWM_REG2;
    tx_buf[0] = VMK8061_CMD_RD_PWM;
    for (n = 0; n < insn.n; n++) {
    if (vmk80xx_read_packet(dev))
    break;
    data[n] = rx_buf[reg[0]] + 4 * rx_buf[reg[1]];
    }
    up(&devpriv.limit_sem);
    return n;
    }
    static int vmk80xx_pwm_insn_write(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    struct vmk80xx_private *devpriv = dev.private;
    unsigned char *tx_buf;
    int reg[2];
    int cmd;
    int n;
    down(&devpriv.limit_sem);
    tx_buf = devpriv.usb_tx_buf;
    reg[0] = VMK8061_PWM_REG1;
    reg[1] = VMK8061_PWM_REG2;
    cmd = VMK8061_CMD_OUT_PWM;
//
// The followin piece of code was translated from the inline
// assembler code in the DLL source code.
//
// asm
// mov eax, k  ; k is the value (data[n])
// and al, 03h ; al are the lower 8 bits of eax
// mov lo, al  ; lo is the low part (tx_buf[reg[0]])
// mov eax, k
// shr eax, 2  ; right shift eax register by 2
// mov hi, al  ; hi is the high part (tx_buf[reg[1]])
// end;
//
    for (n = 0; n < insn.n; n++) {
    tx_buf[reg[0]] = (unsigned char)(data[n] & 0x03);
    tx_buf[reg[1]] = (unsigned char)(data[n] >> 2) & 0xff;
    if (vmk80xx_write_packet(dev, cmd))
    break;
    }
    up(&devpriv.limit_sem);
    return n;
    }
#[no_mangle]
unsafe extern "C" fn vmk80xx_find_usb_endpoints(dev: *mut comedi_device) -> c_int {
    static int vmk80xx_find_usb_endpoints(struct comedi_device *dev)
    {
    struct vmk80xx_private *devpriv = dev.private;
    struct usb_interface *intf = comedi_to_usb_interface(dev);
    struct usb_host_interface *iface_desc = intf.cur_altsetting;
    struct usb_endpoint_descriptor *ep_rx_desc, *ep_tx_desc;
    int ret;
    if (devpriv.model == VMK8061_MODEL)
    ret = usb_find_common_endpoints(iface_desc, &ep_rx_desc,
    &ep_tx_desc, core::ptr::null_mut(), core::ptr::null_mut());
    else
    ret = usb_find_common_endpoints(iface_desc, core::ptr::null_mut(), core::ptr::null_mut(),
    &ep_rx_desc, &ep_tx_desc);
    if (ret)
    return -ENODEV;
    devpriv.ep_rx = ep_rx_desc;
    devpriv.ep_tx = ep_tx_desc;
    if (!usb_endpoint_maxp(devpriv.ep_rx) || !usb_endpoint_maxp(devpriv.ep_tx))
    return -EINVAL;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vmk80xx_alloc_usb_buffers(dev: *mut comedi_device) -> c_int {
    static int vmk80xx_alloc_usb_buffers(struct comedi_device *dev)
    {
    struct vmk80xx_private *devpriv = dev.private;
    size_t size;
    size = max(usb_endpoint_maxp(devpriv.ep_rx), MIN_BUF_SIZE);
    devpriv.usb_rx_buf = kzalloc(size, GFP_KERNEL);
    if (!devpriv.usb_rx_buf)
    return -ENOMEM;
    size = max(usb_endpoint_maxp(devpriv.ep_tx), MIN_BUF_SIZE);
    devpriv.usb_tx_buf = kzalloc(size, GFP_KERNEL);
    if (!devpriv.usb_tx_buf)
    return -ENOMEM;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vmk80xx_init_subdevices(dev: *mut comedi_device) -> c_int {
    static int vmk80xx_init_subdevices(struct comedi_device *dev)
    {
    const struct vmk80xx_board *board = dev.board_ptr;
    struct vmk80xx_private *devpriv = dev.private;
    struct comedi_subdevice *s;
    int n_subd;
    int ret;
    down(&devpriv.limit_sem);
    if (devpriv.model == VMK8055_MODEL)
    n_subd = 5;
    else
    n_subd = 6;
    ret = comedi_alloc_subdevices(dev, n_subd);
    if (ret) {
    up(&devpriv.limit_sem);
    return ret;
    }
// Analog input subdevice
    s = &dev.subdevices[0];
    s.type		= COMEDI_SUBD_AI;
    s.subdev_flags	= SDF_READABLE | SDF_GROUND;
    s.n_chan	= board.ai_nchans;
    s.maxdata	= board.ai_maxdata;
    s.range_table	= board.range;
    s.insn_read	= vmk80xx_ai_insn_read;
// Analog output subdevice
    s = &dev.subdevices[1];
    s.type		= COMEDI_SUBD_AO;
    s.subdev_flags	= SDF_WRITABLE | SDF_GROUND;
    s.n_chan	= board.ao_nchans;
    s.maxdata	= 0x00ff;
    s.range_table	= board.range;
    s.insn_write	= vmk80xx_ao_insn_write;
    if (devpriv.model == VMK8061_MODEL) {
    s.subdev_flags	|= SDF_READABLE;
    s.insn_read	= vmk80xx_ao_insn_read;
    }
// Digital input subdevice
    s = &dev.subdevices[2];
    s.type		= COMEDI_SUBD_DI;
    s.subdev_flags	= SDF_READABLE;
    s.n_chan	= board.di_nchans;
    s.maxdata	= 1;
    s.range_table	= &range_digital;
    s.insn_bits	= vmk80xx_di_insn_bits;
// Digital output subdevice
    s = &dev.subdevices[3];
    s.type		= COMEDI_SUBD_DO;
    s.subdev_flags	= SDF_WRITABLE;
    s.n_chan	= 8;
    s.maxdata	= 1;
    s.range_table	= &range_digital;
    s.insn_bits	= vmk80xx_do_insn_bits;
// Counter subdevice
    s = &dev.subdevices[4];
    s.type		= COMEDI_SUBD_COUNTER;
    s.subdev_flags	= SDF_READABLE;
    s.n_chan	= 2;
    s.maxdata	= board.cnt_maxdata;
    s.insn_read	= vmk80xx_cnt_insn_read;
    s.insn_config	= vmk80xx_cnt_insn_config;
    if (devpriv.model == VMK8055_MODEL) {
    s.subdev_flags	|= SDF_WRITABLE;
    s.insn_write	= vmk80xx_cnt_insn_write;
    }
// PWM subdevice
    if (devpriv.model == VMK8061_MODEL) {
    s = &dev.subdevices[5];
    s.type		= COMEDI_SUBD_PWM;
    s.subdev_flags	= SDF_READABLE | SDF_WRITABLE;
    s.n_chan	= board.pwm_nchans;
    s.maxdata	= board.pwm_maxdata;
    s.insn_read	= vmk80xx_pwm_insn_read;
    s.insn_write	= vmk80xx_pwm_insn_write;
    }
    up(&devpriv.limit_sem);
    return 0;
    }
    static int vmk80xx_auto_attach(struct comedi_device *dev,
    unsigned long context)
    {
    struct usb_interface *intf = comedi_to_usb_interface(dev);
    const struct vmk80xx_board *board = core::ptr::null_mut();
    struct vmk80xx_private *devpriv;
    int ret;
    if (context < ARRAY_SIZE(vmk80xx_boardinfo))
    board = &vmk80xx_boardinfo[context];
    if (!board)
    return -ENODEV;
    dev.board_ptr = board;
    dev.board_name = board.name;
    devpriv = comedi_alloc_devpriv(dev, sizeof(*devpriv));
    if (!devpriv)
    return -ENOMEM;
    devpriv.model = board.model;
    sema_init(&devpriv.limit_sem, 8);
    ret = vmk80xx_find_usb_endpoints(dev);
    if (ret)
    return ret;
    ret = vmk80xx_alloc_usb_buffers(dev);
    if (ret)
    return ret;
    usb_set_intfdata(intf, devpriv);
    if (devpriv.model == VMK8055_MODEL)
    vmk80xx_reset_device(dev);
    return vmk80xx_init_subdevices(dev);
    }
#[no_mangle]
unsafe extern "C" fn vmk80xx_detach(dev: *mut comedi_device) {
    static void vmk80xx_detach(struct comedi_device *dev)
    {
    struct usb_interface *intf = comedi_to_usb_interface(dev);
    struct vmk80xx_private *devpriv = dev.private;
    if (!devpriv)
    return;
    down(&devpriv.limit_sem);
    usb_set_intfdata(intf, core::ptr::null_mut());
    kfree(devpriv.usb_rx_buf);
    kfree(devpriv.usb_tx_buf);
    up(&devpriv.limit_sem);
    }
    static struct comedi_driver vmk80xx_driver = {
    .module		= THIS_MODULE,
    .driver_name	= "vmk80xx",
    .auto_attach	= vmk80xx_auto_attach,
    .detach		= vmk80xx_detach,
    };
    static int vmk80xx_usb_probe(struct usb_interface *intf,
    const struct usb_device_id *id)
    {
    return comedi_usb_auto_config(intf, &vmk80xx_driver, id.driver_info);
    }
    static const struct usb_device_id vmk80xx_usb_id_table[] = {
    { USB_DEVICE(0x10cf, 0x5500), .driver_info = DEVICE_VMK8055 },
    { USB_DEVICE(0x10cf, 0x5501), .driver_info = DEVICE_VMK8055 },
    { USB_DEVICE(0x10cf, 0x5502), .driver_info = DEVICE_VMK8055 },
    { USB_DEVICE(0x10cf, 0x5503), .driver_info = DEVICE_VMK8055 },
    { USB_DEVICE(0x10cf, 0x8061), .driver_info = DEVICE_VMK8061 },
    { USB_DEVICE(0x10cf, 0x8062), .driver_info = DEVICE_VMK8061 },
    { USB_DEVICE(0x10cf, 0x8063), .driver_info = DEVICE_VMK8061 },
    { USB_DEVICE(0x10cf, 0x8064), .driver_info = DEVICE_VMK8061 },
    { USB_DEVICE(0x10cf, 0x8065), .driver_info = DEVICE_VMK8061 },
    { USB_DEVICE(0x10cf, 0x8066), .driver_info = DEVICE_VMK8061 },
    { USB_DEVICE(0x10cf, 0x8067), .driver_info = DEVICE_VMK8061 },
    { USB_DEVICE(0x10cf, 0x8068), .driver_info = DEVICE_VMK8061 },
    { }
    };
    MODULE_DEVICE_TABLE(usb, vmk80xx_usb_id_table);
    static struct usb_driver vmk80xx_usb_driver = {
    .name		= "vmk80xx",
    .id_table	= vmk80xx_usb_id_table,
    .probe		= vmk80xx_usb_probe,
    .disconnect	= comedi_usb_auto_unconfig,
    };
    module_comedi_usb_driver(vmk80xx_driver, vmk80xx_usb_driver);
    MODULE_AUTHOR("Manuel Gebele <forensixs@gmx.de>");
    MODULE_DESCRIPTION("Velleman USB Board Low-Level Driver");
    MODULE_LICENSE("GPL");
