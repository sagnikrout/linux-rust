//! Automatically rewritten from C to Rust
//! Source: drivers/net/can/usb/ems_usb.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// CAN driver for EMS Dr. Thomas Wuensche CPC-USB/ARM7
//
// Copyright (C) 2004-2009 EMS Dr. Thomas Wuensche
//

    MODULE_AUTHOR("Sebastian Haas <haas@ems-wuensche.com>");
    MODULE_DESCRIPTION("CAN driver for EMS Dr. Thomas Wuensche CAN/USB interfaces");
    MODULE_LICENSE("GPL v2");
// Control-Values for CPC_Control() Command Subject Selection
pub const CONTR_CAN_MESSAGE: c_uint = 0x04;
pub const CONTR_CAN_STATE: c_uint = 0x0C;
pub const CONTR_BUS_ERROR: c_uint = 0x1C;
// Control Command Actions
pub const CONTR_CONT_OFF: c_int = 0;
pub const CONTR_CONT_ON: c_int = 1;
pub const CONTR_ONCE: c_int = 2;
// Messages from CPC to PC

// Messages from the PC to the CPC interface

pub const CPC_CAN_ECODE_ERRFRAME: c_uint = 0x01 /* Ecode type */;
// Overrun types
pub const CPC_OVR_EVENT_CAN: c_uint = 0x01;
pub const CPC_OVR_EVENT_CANSTATE: c_uint = 0x02;
pub const CPC_OVR_EVENT_BUSERROR: c_uint = 0x04;
//
// If the CAN controller lost a message we indicate it with the highest bit
// set in the count field.
//
pub const CPC_OVR_HW: c_uint = 0x80;
// Size of the "struct ems_cpc_msg" without the union
pub const CPC_MSG_HEADER_LEN: c_int = 11;
pub const CPC_CAN_MSG_MIN_SIZE: c_int = 5;
// Define these values to match your devices
pub const USB_CPCUSB_VENDOR_ID: c_uint = 0x12D6;
pub const USB_CPCUSB_ARM7_PRODUCT_ID: c_uint = 0x0444;
// Mode register NXP LPC2119/SJA1000 CAN Controller
pub const SJA1000_MOD_NORMAL: c_uint = 0x00;
pub const SJA1000_MOD_RM: c_uint = 0x01;
// ECC register NXP LPC2119/SJA1000 CAN Controller
pub const SJA1000_ECC_SEG: c_uint = 0x1F;
pub const SJA1000_ECC_DIR: c_uint = 0x20;
pub const SJA1000_ECC_ERR: c_uint = 0x06;
pub const SJA1000_ECC_BIT: c_uint = 0x00;
pub const SJA1000_ECC_FORM: c_uint = 0x40;
pub const SJA1000_ECC_STUFF: c_uint = 0x80;
pub const SJA1000_ECC_MASK: c_uint = 0xc0;
// Status register content
pub const SJA1000_SR_BS: c_uint = 0x80;
pub const SJA1000_SR_ES: c_uint = 0x40;
pub const SJA1000_DEFAULT_OUTPUT_CONTROL: c_uint = 0xDA;
//
// The device actually uses a 16MHz clock to generate the CAN clock
// but it expects SJA1000 bit settings based on 8MHz (is internally
// converted).
//
pub const EMS_USB_ARM7_CLOCK: c_int = 8000000;
pub const CPC_TX_QUEUE_TRIGGER_LOW: c_int = 25;
pub const CPC_TX_QUEUE_TRIGGER_HIGH: c_int = 35;
//
// CAN-Message representation in a CPC_MSG. Message object type is
// CPC_MSG_TYPE_CAN_FRAME or CPC_MSG_TYPE_RTR_FRAME or
// CPC_MSG_TYPE_EXT_CAN_FRAME or CPC_MSG_TYPE_EXT_RTR_FRAME.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpc_can_msg {
    pub id: __le32,
    pub length: u8,
    pub msg: [u8; 8],
}

// Representation of the CAN parameters for the SJA1000 controller
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpc_sja1000_params {
    pub mode: u8,
    pub acc_code0: u8,
    pub acc_code1: u8,
    pub acc_code2: u8,
    pub acc_code3: u8,
    pub acc_mask0: u8,
    pub acc_mask1: u8,
    pub acc_mask2: u8,
    pub acc_mask3: u8,
    pub btr0: u8,
    pub btr1: u8,
    pub outp_contr: u8,
}

// CAN params message representation
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpc_can_params {
    pub cc_type: u8,
// Will support M16C CAN controller in the future
    union {
    pub sja1000: cpc_sja1000_params,
    pub cc_params: },
}

// Structure for confirmed message handling
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpc_confirm {
    pub /: *mut *mut u8 error; / error code,
}

// Structure for overrun conditions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpc_overrun {
    pub event: u8,
    pub count: u8,
}

// SJA1000 CAN errors (compatible to NXP LPC2119)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpc_sja1000_can_error {
    pub ecc: u8,
    pub rxerr: u8,
    pub txerr: u8,
}

// structure for CAN error conditions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpc_can_error {
    pub ecode: u8,
    struct {
    pub cc_type: u8,
// Other controllers may also provide error code capture regs
    union {
    pub sja1000: cpc_sja1000_can_error,
    pub regs: },
    pub cc: },
}

//
// Structure containing RX/TX error counter. This structure is used to request
// the values of the CAN controllers TX and RX error counter.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpc_can_err_counter {
    pub rx: u8,
    pub tx: u8,
}

// Main message type used between library and application
    struct __packed ems_cpc_msg {
    u8 type;	/* type of message */
    u8 length;	/* length of data within union 'msg' */
    u8 msgid;	/* confirmation handle */
    __le32 ts_sec;	/* timestamp in seconds */
    __le32 ts_nsec;	/* timestamp in nano seconds */
    union __packed {
    u8 generic[64];
    struct cpc_can_msg can_msg;
    struct cpc_can_params can_params;
    struct cpc_confirm confirmation;
    struct cpc_overrun overrun;
    struct cpc_can_error error;
    struct cpc_can_err_counter err_counter;
    u8 can_state;
    } msg;
    };
//
// Table of devices that work with this driver
// NOTE: This driver supports only CPC-USB/ARM7 (LPC2119) yet.
//
    static struct usb_device_id ems_usb_table[] = {
    {USB_DEVICE(USB_CPCUSB_VENDOR_ID, USB_CPCUSB_ARM7_PRODUCT_ID)},
    {} /* Terminating entry */
    };
    MODULE_DEVICE_TABLE(usb, ems_usb_table);
pub const RX_BUFFER_SIZE: c_int = 64;
pub const CPC_HEADER_SIZE: c_int = 4;
pub const INTR_IN_BUFFER_SIZE: c_int = 4;
pub const MAX_RX_URBS: c_int = 10;
pub const MAX_TX_URBS: c_int = 10;
    struct ems_usb;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ems_tx_urb_context {
    pub dev: *mut ems_usb,
    pub echo_index: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ems_usb {
    pub /: *mut *mut can_priv can; / must be the first member,
    pub echo_skb: [*mut sk_buff; MAX_TX_URBS],
    pub udev: *mut usb_device,
    pub netdev: *mut net_device,
    pub active_tx_urbs: core::sync::atomic::AtomicI32,
    pub tx_submitted: usb_anchor,
    pub tx_contexts: [ems_tx_urb_context; MAX_TX_URBS],
    pub rx_submitted: usb_anchor,
    pub intr_urb: *mut urb,
    pub tx_msg_buffer: *mut u8,
    pub intr_in_buffer: *mut u8,
    pub /: *mut *mut unsigned int free_slots; / remember number of available slots,
    pub /: *mut *mut ems_cpc_msg active_params; / active controller parameters,
    pub rxbuf: [*mut c_void; MAX_RX_URBS],
    pub rxbuf_dma: [dma_addr_t; MAX_RX_URBS],
}

#[no_mangle]
unsafe extern "C" fn ems_usb_read_interrupt_callback(urb: *mut urb) {
    static void ems_usb_read_interrupt_callback(struct urb *urb)
    {
    struct ems_usb *dev = urb.context;
    struct net_device *netdev = dev.netdev;
    int err;
    if (!netif_device_present(netdev))
    return;
    switch (urb.status) {
    case 0:
    dev.free_slots = dev.intr_in_buffer[1];
    if (dev.free_slots > CPC_TX_QUEUE_TRIGGER_HIGH &&
    netif_queue_stopped(netdev))
    netif_wake_queue(netdev);
    break;
    case -ECONNRESET: /* unlink */
    case -ENOENT:
    case -EPIPE:
    case -EPROTO:
    case -ESHUTDOWN:
    return;
    default:
    netdev_info(netdev, "Rx interrupt aborted %d\n", urb.status);
    break;
    }
    err = usb_submit_urb(urb, GFP_ATOMIC);
    if (err == -ENODEV)
    netif_device_detach(netdev);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: err) -> else {
    else if (err)
    netdev_err(netdev, "failed resubmitting intr urb: %d\n", err);
    }
#[no_mangle]
unsafe extern "C" fn ems_usb_rx_can_msg(dev: *mut ems_usb, msg: *mut ems_cpc_msg) {
    static void ems_usb_rx_can_msg(struct ems_usb *dev, struct ems_cpc_msg *msg)
    {
    struct can_frame *cf;
    struct sk_buff *skb;
    int i;
    struct net_device_stats *stats = &dev.netdev.stats;
    skb = alloc_can_skb(dev.netdev, &cf);
    if (skb == core::ptr::null_mut())
    return;
    cf.can_id = le32_to_cpu(msg.msg.can_msg.id);
    cf.len = can_cc_dlc2len(msg.msg.can_msg.length & 0xF);
    if (msg.type == CPC_MSG_TYPE_EXT_CAN_FRAME ||
    msg.type == CPC_MSG_TYPE_EXT_RTR_FRAME)
    cf.can_id |= CAN_EFF_FLAG;
    if (msg.type == CPC_MSG_TYPE_RTR_FRAME ||
    msg.type == CPC_MSG_TYPE_EXT_RTR_FRAME) {
    cf.can_id |= CAN_RTR_FLAG;
    } else {
    for (i = 0; i < cf.len; i++)
    cf.data[i] = msg.msg.can_msg.msg[i];
    stats.rx_bytes += cf.len;
    }
    stats.rx_packets++;
    netif_rx(skb);
    }
#[no_mangle]
unsafe extern "C" fn ems_usb_rx_err(dev: *mut ems_usb, msg: *mut ems_cpc_msg) {
    static void ems_usb_rx_err(struct ems_usb *dev, struct ems_cpc_msg *msg)
    {
    struct can_frame *cf;
    struct sk_buff *skb;
    struct net_device_stats *stats = &dev.netdev.stats;
    skb = alloc_can_err_skb(dev.netdev, &cf);
    if (msg.type == CPC_MSG_TYPE_CAN_STATE) {
    let mut state: u8 = msg.msg.can_state;
    if (state & SJA1000_SR_BS) {
    dev.can.state = CAN_STATE_BUS_OFF;
    if (skb)
    cf.can_id |= CAN_ERR_BUSOFF;
    dev.can.can_stats.bus_off++;
    can_bus_off(dev.netdev);
    } else if (state & SJA1000_SR_ES) {
    dev.can.state = CAN_STATE_ERROR_WARNING;
    dev.can.can_stats.error_warning++;
    } else {
    dev.can.state = CAN_STATE_ERROR_ACTIVE;
    dev.can.can_stats.error_passive++;
    }
    } else if (msg.type == CPC_MSG_TYPE_CAN_FRAME_ERROR) {
    let mut ecc: u8 = msg.msg.error.cc.regs.sja1000.ecc;
    let mut txerr: u8 = msg.msg.error.cc.regs.sja1000.txerr;
    let mut rxerr: u8 = msg.msg.error.cc.regs.sja1000.rxerr;
// bus error interrupt
    dev.can.can_stats.bus_error++;
    if (skb) {
    cf.can_id |= CAN_ERR_PROT | CAN_ERR_BUSERROR;
    switch (ecc & SJA1000_ECC_MASK) {
    case SJA1000_ECC_BIT:
    cf.data[2] |= CAN_ERR_PROT_BIT;
    break;
    case SJA1000_ECC_FORM:
    cf.data[2] |= CAN_ERR_PROT_FORM;
    break;
    case SJA1000_ECC_STUFF:
    cf.data[2] |= CAN_ERR_PROT_STUFF;
    break;
    default:
    cf.data[3] = ecc & SJA1000_ECC_SEG;
    break;
    }
    }
// Error occurred during transmission?
    if ((ecc & SJA1000_ECC_DIR) == 0) {
    stats.tx_errors++;
    if (skb)
    cf.data[2] |= CAN_ERR_PROT_TX;
    } else {
    stats.rx_errors++;
    }
    if (skb && (dev.can.state == CAN_STATE_ERROR_WARNING ||
    dev.can.state == CAN_STATE_ERROR_PASSIVE)) {
    cf.can_id |= CAN_ERR_CRTL;
    cf.data[1] = (txerr > rxerr) ?
    CAN_ERR_CRTL_TX_PASSIVE : CAN_ERR_CRTL_RX_PASSIVE;
    }
    } else if (msg.type == CPC_MSG_TYPE_OVERRUN) {
    if (skb) {
    cf.can_id |= CAN_ERR_CRTL;
    cf.data[1] = CAN_ERR_CRTL_RX_OVERFLOW;
    }
    stats.rx_over_errors++;
    stats.rx_errors++;
    }
    if (skb)
    netif_rx(skb);
    }
#[no_mangle]
unsafe extern "C" fn ems_usb_rx_msg_len_valid(msg: *mut ems_cpc_msg) -> bool {
    static bool ems_usb_rx_msg_len_valid(struct ems_cpc_msg *msg)
    {
    let mut len: usize = msg.length;
    size_t can_len;
    switch (msg.type) {
    case CPC_MSG_TYPE_CAN_STATE:
    return len >= sizeof(msg.msg.can_state);
    case CPC_MSG_TYPE_CAN_FRAME:
    case CPC_MSG_TYPE_EXT_CAN_FRAME:
    case CPC_MSG_TYPE_RTR_FRAME:
    case CPC_MSG_TYPE_EXT_RTR_FRAME:
    if (len < CPC_CAN_MSG_MIN_SIZE)
    return false;
    if (msg.type == CPC_MSG_TYPE_RTR_FRAME ||
    msg.type == CPC_MSG_TYPE_EXT_RTR_FRAME)
    return true;
    can_len = can_cc_dlc2len(msg.msg.can_msg.length & 0xf);
    return len >= CPC_CAN_MSG_MIN_SIZE + can_len;
    case CPC_MSG_TYPE_CAN_FRAME_ERROR:
    return len >= sizeof(msg.msg.error);
    case CPC_MSG_TYPE_OVERRUN:
    return len >= sizeof(msg.msg.overrun);
    default:
    return true;
    }
    }
//
// callback for bulk IN urb
//
#[no_mangle]
unsafe extern "C" fn ems_usb_read_bulk_callback(urb: *mut urb) {
    static void ems_usb_read_bulk_callback(struct urb *urb)
    {
    struct ems_usb *dev = urb.context;
    struct net_device *netdev;
    int retval;
    netdev = dev.netdev;
    if (!netif_device_present(netdev))
    return;
    switch (urb.status) {
    case 0: /* success */
    break;
    case -ENOENT:
    return;
    default:
    netdev_info(netdev, "Rx URB aborted (%d)\n", urb.status);
    goto resubmit_urb;
    }
    if (urb.actual_length > CPC_HEADER_SIZE) {
    struct ems_cpc_msg *msg;
    u8 *ibuf = urb.transfer_buffer;
    u8 msg_count, start;
    msg_count = ibuf[0] & ~0x80;
    start = CPC_HEADER_SIZE;
    while (msg_count) {
    if (start + CPC_MSG_HEADER_LEN > urb.actual_length) {
    netdev_err(netdev, "format error\n");
    break;
    }
    msg = (struct ems_cpc_msg *)&ibuf[start];
    if (msg.length >
    urb.actual_length - start - CPC_MSG_HEADER_LEN) {
    netdev_err(netdev, "format error\n");
    break;
    }
    if (!ems_usb_rx_msg_len_valid(msg)) {
    netdev_err(netdev, "format error\n");
    break;
    }
    switch (msg.type) {
    case CPC_MSG_TYPE_CAN_STATE:
// Process CAN state changes
    ems_usb_rx_err(dev, msg);
    break;
    case CPC_MSG_TYPE_CAN_FRAME:
    case CPC_MSG_TYPE_EXT_CAN_FRAME:
    case CPC_MSG_TYPE_RTR_FRAME:
    case CPC_MSG_TYPE_EXT_RTR_FRAME:
    ems_usb_rx_can_msg(dev, msg);
    break;
    case CPC_MSG_TYPE_CAN_FRAME_ERROR:
// Process errorframe
    ems_usb_rx_err(dev, msg);
    break;
    case CPC_MSG_TYPE_OVERRUN:
// Message lost while receiving
    ems_usb_rx_err(dev, msg);
    break;
    }
    start += CPC_MSG_HEADER_LEN + msg.length;
    msg_count--;
    if (start > urb.actual_length) {
    netdev_err(netdev, "format error\n");
    break;
    }
    }
    }
    resubmit_urb:
    usb_fill_bulk_urb(urb, dev.udev, usb_rcvbulkpipe(dev.udev, 2),
    urb.transfer_buffer, RX_BUFFER_SIZE,
    ems_usb_read_bulk_callback, dev);
    usb_anchor_urb(urb, &dev.rx_submitted);
    retval = usb_submit_urb(urb, GFP_ATOMIC);
    if (!retval)
    return;
    usb_unanchor_urb(urb);
    if (retval == -ENODEV)
    netif_device_detach(netdev);
    else
    netdev_err(netdev,
    "failed resubmitting read bulk urb: %d\n", retval);
    }
//
// callback for bulk IN urb
//
#[no_mangle]
unsafe extern "C" fn ems_usb_write_bulk_callback(urb: *mut urb) {
    static void ems_usb_write_bulk_callback(struct urb *urb)
    {
    struct ems_tx_urb_context *context = urb.context;
    struct ems_usb *dev;
    struct net_device *netdev;
    BUG_ON(!context);
    dev = context.dev;
    netdev = dev.netdev;
// free up our allocated buffer
    usb_free_coherent(urb.dev, urb.transfer_buffer_length,
    urb.transfer_buffer, urb.transfer_dma);
    atomic_dec(&dev.active_tx_urbs);
    if (!netif_device_present(netdev))
    return;
    if (urb.status)
    netdev_info(netdev, "Tx URB aborted (%d)\n", urb.status);
    netif_trans_update(netdev);
// transmission complete interrupt
    netdev.stats.tx_packets++;
    netdev.stats.tx_bytes += can_get_echo_skb(netdev, context.echo_index,
    core::ptr::null_mut());
// Release context
    context.echo_index = MAX_TX_URBS;
    }
//
// Send the given CPC command synchronously
//
#[no_mangle]
unsafe extern "C" fn ems_usb_command_msg(dev: *mut ems_usb, msg: *mut ems_cpc_msg) -> c_int {
    static int ems_usb_command_msg(struct ems_usb *dev, struct ems_cpc_msg *msg)
    {
    int actual_length;
// Copy payload
    memcpy(&dev.tx_msg_buffer[CPC_HEADER_SIZE], msg,
    msg.length + CPC_MSG_HEADER_LEN);
// Clear header
    memset(&dev.tx_msg_buffer[0], 0, CPC_HEADER_SIZE);
    return usb_bulk_msg(dev.udev, usb_sndbulkpipe(dev.udev, 2),
    &dev.tx_msg_buffer[0],
    msg.length + CPC_MSG_HEADER_LEN + CPC_HEADER_SIZE,
    &actual_length, 1000);
    }
//
// Change CAN controllers' mode register
//
#[no_mangle]
unsafe extern "C" fn ems_usb_write_mode(dev: *mut ems_usb, mode: u8) -> c_int {
    static int ems_usb_write_mode(struct ems_usb *dev, u8 mode)
    {
    dev.active_params.msg.can_params.cc_params.sja1000.mode = mode;
    return ems_usb_command_msg(dev, &dev.active_params);
    }
//
// Send a CPC_Control command to change behaviour when interface receives a CAN
// message, bus error or CAN state changed notifications.
//
#[no_mangle]
unsafe extern "C" fn ems_usb_control_cmd(dev: *mut ems_usb, val: u8) -> c_int {
    static int ems_usb_control_cmd(struct ems_usb *dev, u8 val)
    {
    struct ems_cpc_msg cmd;
    cmd.type = CPC_CMD_TYPE_CONTROL;
    cmd.length = CPC_MSG_HEADER_LEN + 1;
    cmd.msgid = 0;
    cmd.msg.generic[0] = val;
    return ems_usb_command_msg(dev, &cmd);
    }
//
// Start interface
//
#[no_mangle]
unsafe extern "C" fn ems_usb_start(dev: *mut ems_usb) -> c_int {
    static int ems_usb_start(struct ems_usb *dev)
    {
    struct net_device *netdev = dev.netdev;
    int err, i;
    dev.intr_in_buffer[0] = 0;
    dev.free_slots = 50; /* initial size */
    for (i = 0; i < MAX_RX_URBS; i++) {
    struct urb *urb = core::ptr::null_mut();
    u8 *buf = core::ptr::null_mut();
    dma_addr_t buf_dma;
// create a URB, and a buffer for it
    urb = usb_alloc_urb(0, GFP_KERNEL);
    if (!urb) {
    err = -ENOMEM;
    break;
    }
    buf = usb_alloc_coherent(dev.udev, RX_BUFFER_SIZE, GFP_KERNEL,
    &buf_dma);
    if (!buf) {
    netdev_err(netdev, "No memory left for USB buffer\n");
    usb_free_urb(urb);
    err = -ENOMEM;
    break;
    }
    urb.transfer_dma = buf_dma;
    usb_fill_bulk_urb(urb, dev.udev, usb_rcvbulkpipe(dev.udev, 2),
    buf, RX_BUFFER_SIZE,
    ems_usb_read_bulk_callback, dev);
    urb.transfer_flags |= URB_NO_TRANSFER_DMA_MAP;
    usb_anchor_urb(urb, &dev.rx_submitted);
    err = usb_submit_urb(urb, GFP_KERNEL);
    if (err) {
    usb_unanchor_urb(urb);
    usb_free_coherent(dev.udev, RX_BUFFER_SIZE, buf,
    urb.transfer_dma);
    usb_free_urb(urb);
    break;
    }
    dev.rxbuf[i] = buf;
    dev.rxbuf_dma[i] = buf_dma;
// Drop reference, USB core will take care of freeing it
    usb_free_urb(urb);
    }
// Did we submit any URBs
    if (i == 0) {
    netdev_warn(netdev, "couldn't setup read URBs\n");
    return err;
    }
// Warn if we've couldn't transmit all the URBs
    if (i < MAX_RX_URBS)
    netdev_warn(netdev, "rx performance may be slow\n");
// Setup and start interrupt URB
    usb_fill_int_urb(dev.intr_urb, dev.udev,
    usb_rcvintpipe(dev.udev, 1),
    dev.intr_in_buffer,
    INTR_IN_BUFFER_SIZE,
    ems_usb_read_interrupt_callback, dev, 1);
    err = usb_submit_urb(dev.intr_urb, GFP_KERNEL);
    if (err) {
    netdev_warn(netdev, "intr URB submit failed: %d\n", err);
    return err;
    }
// CPC-USB will transfer received message to host
    err = ems_usb_control_cmd(dev, CONTR_CAN_MESSAGE | CONTR_CONT_ON);
    if (err)
    goto failed;
// CPC-USB will transfer CAN state changes to host
    err = ems_usb_control_cmd(dev, CONTR_CAN_STATE | CONTR_CONT_ON);
    if (err)
    goto failed;
// CPC-USB will transfer bus errors to host
    err = ems_usb_control_cmd(dev, CONTR_BUS_ERROR | CONTR_CONT_ON);
    if (err)
    goto failed;
    err = ems_usb_write_mode(dev, SJA1000_MOD_NORMAL);
    if (err)
    goto failed;
    dev.can.state = CAN_STATE_ERROR_ACTIVE;
    return 0;
    failed:
    netdev_warn(netdev, "couldn't submit control: %d\n", err);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn unlink_all_urbs(dev: *mut ems_usb) {
    static void unlink_all_urbs(struct ems_usb *dev)
    {
    int i;
    usb_unlink_urb(dev.intr_urb);
    usb_kill_anchored_urbs(&dev.rx_submitted);
    for (i = 0; i < MAX_RX_URBS; ++i)
    usb_free_coherent(dev.udev, RX_BUFFER_SIZE,
    dev.rxbuf[i], dev.rxbuf_dma[i]);
    usb_kill_anchored_urbs(&dev.tx_submitted);
    atomic_set(&dev.active_tx_urbs, 0);
    for (i = 0; i < MAX_TX_URBS; i++)
    dev.tx_contexts[i].echo_index = MAX_TX_URBS;
    }
#[no_mangle]
unsafe extern "C" fn ems_usb_open(netdev: *mut net_device) -> c_int {
    static int ems_usb_open(struct net_device *netdev)
    {
    struct ems_usb *dev = netdev_priv(netdev);
    int err;
    err = ems_usb_write_mode(dev, SJA1000_MOD_RM);
    if (err)
    return err;
// common open
    err = open_candev(netdev);
    if (err)
    return err;
// finally start device
    err = ems_usb_start(dev);
    if (err) {
    if (err == -ENODEV)
    netif_device_detach(dev.netdev);
    netdev_warn(netdev, "couldn't start device: %d\n", err);
    close_candev(netdev);
    return err;
    }
    netif_start_queue(netdev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ems_usb_start_xmit(skb: *mut sk_buff, netdev: *mut net_device) -> netdev_tx_t {
    static netdev_tx_t ems_usb_start_xmit(struct sk_buff *skb, struct net_device *netdev)
    {
    struct ems_usb *dev = netdev_priv(netdev);
    struct ems_tx_urb_context *context = core::ptr::null_mut();
    struct net_device_stats *stats = &netdev.stats;
    struct can_frame *cf = (struct can_frame *)skb.data;
    struct ems_cpc_msg *msg;
    struct urb *urb;
    u8 *buf;
    int i, err;
    size_t size = CPC_HEADER_SIZE + CPC_MSG_HEADER_LEN
    + sizeof(struct cpc_can_msg);
    if (can_dev_dropped_skb(netdev, skb))
    return NETDEV_TX_OK;
// create a URB, and a buffer for it, and copy the data to the URB
    urb = usb_alloc_urb(0, GFP_ATOMIC);
    if (!urb)
    goto nomem;
    buf = usb_alloc_coherent(dev.udev, size, GFP_ATOMIC, &urb.transfer_dma);
    if (!buf) {
    netdev_err(netdev, "No memory left for USB buffer\n");
    usb_free_urb(urb);
    goto nomem;
    }
    msg = (struct ems_cpc_msg *)&buf[CPC_HEADER_SIZE];
    msg.msg.can_msg.id = cpu_to_le32(cf.can_id & CAN_ERR_MASK);
    msg.msg.can_msg.length = cf.len;
    if (cf.can_id & CAN_RTR_FLAG) {
    msg.type = cf.can_id & CAN_EFF_FLAG ?
    CPC_CMD_TYPE_EXT_RTR_FRAME : CPC_CMD_TYPE_RTR_FRAME;
    msg.length = CPC_CAN_MSG_MIN_SIZE;
    } else {
    msg.type = cf.can_id & CAN_EFF_FLAG ?
    CPC_CMD_TYPE_EXT_CAN_FRAME : CPC_CMD_TYPE_CAN_FRAME;
    for (i = 0; i < cf.len; i++)
    msg.msg.can_msg.msg[i] = cf.data[i];
    msg.length = CPC_CAN_MSG_MIN_SIZE + cf.len;
    }
    for (i = 0; i < MAX_TX_URBS; i++) {
    if (dev.tx_contexts[i].echo_index == MAX_TX_URBS) {
    context = &dev.tx_contexts[i];
    break;
    }
    }
//
// May never happen! When this happens we'd more URBs in flight as
// allowed (MAX_TX_URBS).
//
    if (!context) {
    usb_free_coherent(dev.udev, size, buf, urb.transfer_dma);
    usb_free_urb(urb);
    netdev_warn(netdev, "couldn't find free context\n");
    return NETDEV_TX_BUSY;
    }
    context.dev = dev;
    context.echo_index = i;
    usb_fill_bulk_urb(urb, dev.udev, usb_sndbulkpipe(dev.udev, 2), buf,
    size, ems_usb_write_bulk_callback, context);
    urb.transfer_flags |= URB_NO_TRANSFER_DMA_MAP;
    usb_anchor_urb(urb, &dev.tx_submitted);
    can_put_echo_skb(skb, netdev, context.echo_index, 0);
    atomic_inc(&dev.active_tx_urbs);
    err = usb_submit_urb(urb, GFP_ATOMIC);
    if (unlikely(err)) {
    can_free_echo_skb(netdev, context.echo_index, core::ptr::null_mut());
    usb_unanchor_urb(urb);
    usb_free_coherent(dev.udev, size, buf, urb.transfer_dma);
    atomic_dec(&dev.active_tx_urbs);
    if (err == -ENODEV) {
    netif_device_detach(netdev);
    } else {
    netdev_warn(netdev, "failed tx_urb %d\n", err);
    stats.tx_dropped++;
    }
    } else {
    netif_trans_update(netdev);
// Slow down tx path
    if (atomic_read(&dev.active_tx_urbs) >= MAX_TX_URBS ||
    dev.free_slots < CPC_TX_QUEUE_TRIGGER_LOW) {
    netif_stop_queue(netdev);
    }
    }
//
// Release our reference to this URB, the USB core will eventually free
// it entirely.
//
    usb_free_urb(urb);
    return NETDEV_TX_OK;
    nomem:
    dev_kfree_skb(skb);
    stats.tx_dropped++;
    return NETDEV_TX_OK;
    }
#[no_mangle]
unsafe extern "C" fn ems_usb_close(netdev: *mut net_device) -> c_int {
    static int ems_usb_close(struct net_device *netdev)
    {
    struct ems_usb *dev = netdev_priv(netdev);
// Stop polling
    unlink_all_urbs(dev);
    netif_stop_queue(netdev);
// Set CAN controller to reset mode
    if (ems_usb_write_mode(dev, SJA1000_MOD_RM))
    netdev_warn(netdev, "couldn't stop device");
    close_candev(netdev);
    return 0;
    }
    static const struct net_device_ops ems_usb_netdev_ops = {
    .ndo_open = ems_usb_open,
    .ndo_stop = ems_usb_close,
    .ndo_start_xmit = ems_usb_start_xmit,
    };
    static const struct ethtool_ops ems_usb_ethtool_ops = {
    .get_ts_info = ethtool_op_get_ts_info,
    };
    static const struct can_bittiming_const ems_usb_bittiming_const = {
    .name = KBUILD_MODNAME,
    .tseg1_min = 1,
    .tseg1_max = 16,
    .tseg2_min = 1,
    .tseg2_max = 8,
    .sjw_max = 4,
    .brp_min = 1,
    .brp_max = 64,
    .brp_inc = 1,
    };
#[no_mangle]
unsafe extern "C" fn ems_usb_set_mode(netdev: *mut net_device, mode: enum can_mode) -> c_int {
    static int ems_usb_set_mode(struct net_device *netdev, enum can_mode mode)
    {
    struct ems_usb *dev = netdev_priv(netdev);
    switch (mode) {
    case CAN_MODE_START:
    if (ems_usb_write_mode(dev, SJA1000_MOD_NORMAL))
    netdev_warn(netdev, "couldn't start device");
    if (netif_queue_stopped(netdev))
    netif_wake_queue(netdev);
    break;
    default:
    return -EOPNOTSUPP;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ems_usb_set_bittiming(netdev: *mut net_device) -> c_int {
    static int ems_usb_set_bittiming(struct net_device *netdev)
    {
    struct ems_usb *dev = netdev_priv(netdev);
    struct can_bittiming *bt = &dev.can.bittiming;
    u8 btr0, btr1;
    btr0 = ((bt.brp - 1) & 0x3f) | (((bt.sjw - 1) & 0x3) << 6);
    btr1 = ((bt.prop_seg + bt.phase_seg1 - 1) & 0xf) |
    (((bt.phase_seg2 - 1) & 0x7) << 4);
    if (dev.can.ctrlmode & CAN_CTRLMODE_3_SAMPLES)
    btr1 |= 0x80;
    netdev_info(netdev, "setting BTR0=0x%02x BTR1=0x%02x\n", btr0, btr1);
    dev.active_params.msg.can_params.cc_params.sja1000.btr0 = btr0;
    dev.active_params.msg.can_params.cc_params.sja1000.btr1 = btr1;
    return ems_usb_command_msg(dev, &dev.active_params);
    }
#[no_mangle]
unsafe extern "C" fn init_params_sja1000(msg: *mut ems_cpc_msg) {
    static void init_params_sja1000(struct ems_cpc_msg *msg)
    {
    struct cpc_sja1000_params *sja1000 =
    &msg.msg.can_params.cc_params.sja1000;
    msg.type = CPC_CMD_TYPE_CAN_PARAMS;
    msg.length = sizeof(struct cpc_can_params);
    msg.msgid = 0;
    msg.msg.can_params.cc_type = CPC_CC_TYPE_SJA1000;
// Acceptance filter open
    sja1000.acc_code0 = 0x00;
    sja1000.acc_code1 = 0x00;
    sja1000.acc_code2 = 0x00;
    sja1000.acc_code3 = 0x00;
// Acceptance filter open
    sja1000.acc_mask0 = 0xFF;
    sja1000.acc_mask1 = 0xFF;
    sja1000.acc_mask2 = 0xFF;
    sja1000.acc_mask3 = 0xFF;
    sja1000.btr0 = 0;
    sja1000.btr1 = 0;
    sja1000.outp_contr = SJA1000_DEFAULT_OUTPUT_CONTROL;
    sja1000.mode = SJA1000_MOD_RM;
    }
//
// probe function for new CPC-USB devices
//
    static int ems_usb_probe(struct usb_interface *intf,
    const struct usb_device_id *id)
    {
    struct net_device *netdev;
    struct ems_usb *dev;
    int i, err = -ENOMEM;
    netdev = alloc_candev(sizeof(struct ems_usb), MAX_TX_URBS);
    if (!netdev) {
    dev_err(&intf.dev, "ems_usb: Couldn't alloc candev\n");
    return -ENOMEM;
    }
    dev = netdev_priv(netdev);
    dev.udev = interface_to_usbdev(intf);
    dev.netdev = netdev;
    dev.can.state = CAN_STATE_STOPPED;
    dev.can.clock.freq = EMS_USB_ARM7_CLOCK;
    dev.can.bittiming_const = &ems_usb_bittiming_const;
    dev.can.do_set_bittiming = ems_usb_set_bittiming;
    dev.can.do_set_mode = ems_usb_set_mode;
    dev.can.ctrlmode_supported = CAN_CTRLMODE_3_SAMPLES;
    netdev.netdev_ops = &ems_usb_netdev_ops;
    netdev.ethtool_ops = &ems_usb_ethtool_ops;
    netdev.flags |= IFF_ECHO; /* we support local echo */
    init_usb_anchor(&dev.rx_submitted);
    init_usb_anchor(&dev.tx_submitted);
    atomic_set(&dev.active_tx_urbs, 0);
    for (i = 0; i < MAX_TX_URBS; i++)
    dev.tx_contexts[i].echo_index = MAX_TX_URBS;
    dev.intr_urb = usb_alloc_urb(0, GFP_KERNEL);
    if (!dev.intr_urb)
    goto cleanup_candev;
    dev.intr_in_buffer = kzalloc(INTR_IN_BUFFER_SIZE, GFP_KERNEL);
    if (!dev.intr_in_buffer)
    goto cleanup_intr_urb;
    dev.tx_msg_buffer = kzalloc(CPC_HEADER_SIZE +
    sizeof(struct ems_cpc_msg), GFP_KERNEL);
    if (!dev.tx_msg_buffer)
    goto cleanup_intr_in_buffer;
    usb_set_intfdata(intf, dev);
    SET_NETDEV_DEV(netdev, &intf.dev);
    init_params_sja1000(&dev.active_params);
    err = ems_usb_command_msg(dev, &dev.active_params);
    if (err) {
    netdev_err(netdev, "couldn't initialize controller: %d\n", err);
    goto cleanup_tx_msg_buffer;
    }
    err = register_candev(netdev);
    if (err) {
    netdev_err(netdev, "couldn't register CAN device: %d\n", err);
    goto cleanup_tx_msg_buffer;
    }
    return 0;
    cleanup_tx_msg_buffer:
    kfree(dev.tx_msg_buffer);
    cleanup_intr_in_buffer:
    kfree(dev.intr_in_buffer);
    cleanup_intr_urb:
    usb_free_urb(dev.intr_urb);
    cleanup_candev:
    free_candev(netdev);
    return err;
    }
//
// called by the usb core when the device is removed from the system
//
#[no_mangle]
unsafe extern "C" fn ems_usb_disconnect(intf: *mut usb_interface) {
    static void ems_usb_disconnect(struct usb_interface *intf)
    {
    struct ems_usb *dev = usb_get_intfdata(intf);
    usb_set_intfdata(intf, core::ptr::null_mut());
    if (dev) {
    unregister_netdev(dev.netdev);
    unlink_all_urbs(dev);
    usb_free_urb(dev.intr_urb);
    kfree(dev.intr_in_buffer);
    kfree(dev.tx_msg_buffer);
    free_candev(dev.netdev);
    }
    }
// usb specific object needed to register this driver with the usb subsystem
    static struct usb_driver ems_usb_driver = {
    .name = KBUILD_MODNAME,
    .probe = ems_usb_probe,
    .disconnect = ems_usb_disconnect,
    .id_table = ems_usb_table,
    };
    module_usb_driver(ems_usb_driver);
