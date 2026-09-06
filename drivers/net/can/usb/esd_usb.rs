//! Automatically rewritten from C to Rust
//! Source: drivers/net/can/usb/esd_usb.c
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
// CAN driver for esd electronics gmbh CAN-USB/2, CAN-USB/3 and CAN-USB/Micro
//
// Copyright (C) 2010-2012 esd electronic system design gmbh, Matthias Fuchs <socketcan@esd.eu>
// Copyright (C) 2022-2024 esd electronics gmbh, Frank Jungclaus <frank.jungclaus@esd.eu>
//

    MODULE_AUTHOR("Matthias Fuchs <socketcan@esd.eu>");
    MODULE_AUTHOR("Frank Jungclaus <frank.jungclaus@esd.eu>");
    MODULE_DESCRIPTION("CAN driver for esd electronics gmbh CAN-USB/2, CAN-USB/3 and CAN-USB/Micro interfaces");
    MODULE_LICENSE("GPL v2");
// USB vendor and product ID
pub const ESD_USB_ESDGMBH_VENDOR_ID: c_uint = 0x0ab4;
pub const ESD_USB_CANUSB2_PRODUCT_ID: c_uint = 0x0010;
pub const ESD_USB_CANUSBM_PRODUCT_ID: c_uint = 0x0011;
pub const ESD_USB_CANUSB3_PRODUCT_ID: c_uint = 0x0014;
// CAN controller clock frequencies

// Maximum number of CAN nets
pub const ESD_USB_MAX_NETS: c_int = 2;
// USB commands

// esd CAN message flags - dlc field

// esd CAN message flags - id field

// esd CAN event ids

// baudrate message flags

// bit timing esd CAN-USB
pub const ESD_USB_2_TSEG1_SHIFT: c_int = 16;
pub const ESD_USB_2_TSEG2_SHIFT: c_int = 20;
pub const ESD_USB_2_SJW_SHIFT: c_int = 14;
pub const ESD_USB_M_SJW_SHIFT: c_int = 24;

// Transmitter Delay Compensation
pub const ESD_USB_3_TDC_MODE_AUTO: c_int = 0;
// esd IDADD message

pub const ESD_USB_MAX_ID_SEGMENT: c_int = 64;
// SJA1000 ECC register (emulated by usb firmware)

pub const ESD_USB_SJA1000_ECC_BIT: c_uint = 0x00;

// esd bus state event codes

pub const ESD_USB_RX_BUFFER_SIZE: c_int = 1024;
pub const ESD_USB_MAX_RX_URBS: c_int = 4;

// Modes for CAN-USB/3, to be used for esd_usb_3_set_baudrate_msg_x.mode

// Flags for CAN-USB/3, to be used for esd_usb_3_set_baudrate_msg_x.flags

#[repr(C)]
#[derive(Copy, Clone)]
pub struct esd_usb_header_msg {
    pub /: *mut *mut u8 len; / total message length in 32bit words,
    pub cmd: u8,
    pub rsvd: [u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct esd_usb_version_msg {
    pub /: *mut *mut u8 len; / total message length in 32bit words,
    pub cmd: u8,
    pub rsvd: u8,
    pub flags: u8,
    pub drv_version: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct esd_usb_version_reply_msg {
    pub /: *mut *mut u8 len; / total message length in 32bit words,
    pub cmd: u8,
    pub nets: u8,
    pub features: u8,
    pub version: __le32,
    pub name: [u8; 16],
    pub rsvd: __le32,
    pub ts: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct esd_usb_rx_msg {
    pub /: *mut *mut u8 len; / total message length in 32bit words,
    pub cmd: u8,
    pub net: u8,
    pub dlc: u8,
    pub ts: __le32,
    pub /: *mut *mut __le32 id; / upper 3 bits contain flags,
    union {
    pub data: [u8; CAN_MAX_DLEN],
    pub data_fd: [u8; CANFD_MAX_DLEN],
    struct {
    pub /: *mut *mut u8 status; / CAN Controller Status,
    pub /: *mut *mut u8 ecc; / Error Capture Register,
    pub /: *mut *mut u8 rec; / RX Error Counter,
    pub /: *mut *mut u8 tec; / TX Error Counter,
    pub /: *mut *mut } ev_can_err_ext; / For ESD_EV_CAN_ERROR_EXT,
}

    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct esd_usb_tx_msg {
    pub /: *mut *mut u8 len; / total message length in 32bit words,
    pub cmd: u8,
    pub net: u8,
    pub dlc: u8,
    pub /: *mut *mut u32 hnd; / opaque handle, not used by device,
    pub /: *mut *mut __le32 id; / upper 3 bits contain flags,
    union {
    pub data: [u8; CAN_MAX_DLEN],
    pub data_fd: [u8; CANFD_MAX_DLEN],
}

    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct esd_usb_tx_done_msg {
    pub /: *mut *mut u8 len; / total message length in 32bit words,
    pub cmd: u8,
    pub net: u8,
    pub status: u8,
    pub /: *mut *mut u32 hnd; / opaque handle, not used by device,
    pub ts: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct esd_usb_id_filter_msg {
    pub /: *mut *mut u8 len; / total message length in 32bit words,
    pub cmd: u8,
    pub net: u8,
    pub option: u8,
    pub /: *mut *mut __le32 mask[ESD_USB_MAX_ID_SEGMENT + 1]; / +1 for 29bit extended IDs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct esd_usb_set_baudrate_msg {
    pub /: *mut *mut u8 len; / total message length in 32bit words,
    pub cmd: u8,
    pub net: u8,
    pub rsvd: u8,
    pub baud: __le32,
}

// CAN-USB/3 baudrate configuration, used for nominal as well as for data bit rate
#[repr(C)]
#[derive(Copy, Clone)]
pub struct esd_usb_3_baudrate_cfg {
    pub /: *mut *mut __le16 brp; / bit rate pre-scaler,
    pub /: *mut *mut __le16 tseg1; / time segment before sample point,
    pub /: *mut *mut __le16 tseg2; / time segment after sample point,
    pub /: *mut *mut __le16 sjw; / synchronization jump Width,
}

// In principle, the esd CAN-USB/3 supports Transmitter Delay Compensation (TDC),
// but currently only the automatic TDC mode is supported by this driver.
// An implementation for manual TDC configuration will follow.
//
// For information about struct esd_usb_3_tdc_cfg, see
// NTCAN Application Developers Manual, 6.2.25 NTCAN_TDC_CFG + related chapters
// https://esd.eu/fileadmin/esd/docs/manuals/NTCAN_Part1_Function_API_Manual_en_56.pdf
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct esd_usb_3_tdc_cfg {
    pub /: *mut *mut u8 tdc_mode; / transmitter delay compensation mode,
    pub /: *mut *mut u8 ssp_offset; / secondary sample point offset in mtq,
    pub /: *mut *mut s8 ssp_shift; / secondary sample point shift in mtq,
    pub /: *mut *mut u8 tdc_filter; / TDC filter in mtq,
}

// Extended version of the above set_baudrate_msg for a CAN-USB/3
// to define the CAN bit timing configuration of the CAN controller in
// CAN FD mode as well as in Classical CAN mode.
//
// The payload of this command is a NTCAN_BAUDRATE_X structure according to
// esd electronics gmbh, NTCAN Application Developers Manual, 6.2.15 NTCAN_BAUDRATE_X
// https://esd.eu/fileadmin/esd/docs/manuals/NTCAN_Part1_Function_API_Manual_en_56.pdf
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct esd_usb_3_set_baudrate_msg_x {
    pub /: *mut *mut u8 len; / total message length in 32bit words,
    pub cmd: u8,
    pub net: u8,
    pub /: *mut *mut u8 rsvd; /reserved,
// Payload ...
    pub /: *mut *mut __le16 mode; / mode word, see ESD_USB_3_BAUDRATE_MODE_xxx,
    pub /: *mut *mut __le16 flags; / control flags, see ESD_USB_3_BAUDRATE_FLAG_xxx,
    pub /: *mut *mut esd_usb_3_tdc_cfg tdc; / TDC configuration,
    pub /: *mut *mut esd_usb_3_baudrate_cfg nom; / nominal bit rate,
    pub /: *mut *mut esd_usb_3_baudrate_cfg data; / data bit rate,
}

// Main message type used between library and application
    union __packed esd_usb_msg {
    struct esd_usb_header_msg hdr;
    struct esd_usb_version_msg version;
    struct esd_usb_version_reply_msg version_reply;
    struct esd_usb_rx_msg rx;
    struct esd_usb_tx_msg tx;
    struct esd_usb_tx_done_msg txdone;
    struct esd_usb_set_baudrate_msg setbaud;
    struct esd_usb_3_set_baudrate_msg_x setbaud_x;
    struct esd_usb_id_filter_msg filter;
    };
    static struct usb_device_id esd_usb_table[] = {
    {USB_DEVICE(ESD_USB_ESDGMBH_VENDOR_ID, ESD_USB_CANUSB2_PRODUCT_ID)},
    {USB_DEVICE(ESD_USB_ESDGMBH_VENDOR_ID, ESD_USB_CANUSBM_PRODUCT_ID)},
    {USB_DEVICE(ESD_USB_ESDGMBH_VENDOR_ID, ESD_USB_CANUSB3_PRODUCT_ID)},
    {}
    };
    MODULE_DEVICE_TABLE(usb, esd_usb_table);
    struct esd_usb_net_priv;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct esd_tx_urb_context {
    pub priv: *mut esd_usb_net_priv,
    pub echo_index: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct esd_usb {
    pub udev: *mut usb_device,
    pub nets: [*mut esd_usb_net_priv; ESD_USB_MAX_NETS],
    pub rx_submitted: usb_anchor,
    pub rx_pipe: c_uint,
    pub tx_pipe: c_uint,
    pub net_count: c_int,
    pub version: u32,
    pub rxinitdone: c_int,
    pub in_usb_disconnect: c_int,
    pub rxbuf: [*mut c_void; ESD_USB_MAX_RX_URBS],
    pub rxbuf_dma: [dma_addr_t; ESD_USB_MAX_RX_URBS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct esd_usb_net_priv {
    pub /: *mut *mut can_priv can; / must be the first member,
    pub active_tx_jobs: core::sync::atomic::AtomicI32,
    pub tx_submitted: usb_anchor,
    pub tx_contexts: [esd_tx_urb_context; ESD_USB_MAX_TX_URBS],
    pub usb: *mut esd_usb,
    pub netdev: *mut net_device,
    pub index: c_int,
    pub old_state: u8,
    pub bec: can_berr_counter,
}

    static void esd_usb_rx_event(struct esd_usb_net_priv *priv,
    union esd_usb_msg *msg)
    {
    struct net_device_stats *stats = &priv.netdev.stats;
    struct can_frame *cf;
    struct sk_buff *skb;
    let mut id: u32 = le32_to_cpu(msg.rx.id) & ESD_USB_IDMASK;
    if (id == ESD_USB_EV_CAN_ERROR_EXT) {
    let mut state: u8 = msg.rx.ev_can_err_ext.status;
    let mut ecc: u8 = msg.rx.ev_can_err_ext.ecc;
    priv.bec.rxerr = msg.rx.ev_can_err_ext.rec;
    priv.bec.txerr = msg.rx.ev_can_err_ext.tec;
    netdev_dbg(priv.netdev,
    "CAN_ERR_EV_EXT: dlc=%#02x state=%02x ecc=%02x rec=%02x tec=%02x\n",
    msg.rx.dlc, state, ecc,
    priv.bec.rxerr, priv.bec.txerr);
// if berr-reporting is off, only pass through on state change ...
    if (!(priv.can.ctrlmode & CAN_CTRLMODE_BERR_REPORTING) &&
    state == priv.old_state)
    return;
    skb = alloc_can_err_skb(priv.netdev, &cf);
    if (!skb)
    stats.rx_dropped++;
    if (state != priv.old_state) {
    enum can_state tx_state, rx_state;
    let mut new_state: enum can_state = CAN_STATE_ERROR_ACTIVE;
    priv.old_state = state;
    switch (state & ESD_USB_BUSSTATE_MASK) {
    case ESD_USB_BUSSTATE_BUSOFF:
    new_state = CAN_STATE_BUS_OFF;
    can_bus_off(priv.netdev);
    break;
    case ESD_USB_BUSSTATE_WARN:
    new_state = CAN_STATE_ERROR_WARNING;
    break;
    case ESD_USB_BUSSTATE_ERRPASSIVE:
    new_state = CAN_STATE_ERROR_PASSIVE;
    break;
    default:
    new_state = CAN_STATE_ERROR_ACTIVE;
    priv.bec.txerr = 0;
    priv.bec.rxerr = 0;
    break;
    }
    if (new_state != priv.can.state) {
    tx_state = (priv.bec.txerr >= priv.bec.rxerr) ? new_state : 0;
    rx_state = (priv.bec.txerr <= priv.bec.rxerr) ? new_state : 0;
    can_change_state(priv.netdev, cf,
    tx_state, rx_state);
    }
    } else if (skb) {
    priv.can.can_stats.bus_error++;
    stats.rx_errors++;
    cf.can_id |= CAN_ERR_PROT | CAN_ERR_BUSERROR;
    switch (ecc & ESD_USB_SJA1000_ECC_MASK) {
    case ESD_USB_SJA1000_ECC_BIT:
    cf.data[2] |= CAN_ERR_PROT_BIT;
    break;
    case ESD_USB_SJA1000_ECC_FORM:
    cf.data[2] |= CAN_ERR_PROT_FORM;
    break;
    case ESD_USB_SJA1000_ECC_STUFF:
    cf.data[2] |= CAN_ERR_PROT_STUFF;
    break;
    default:
    break;
    }
// Error occurred during transmission?
    if (!(ecc & ESD_USB_SJA1000_ECC_DIR))
    cf.data[2] |= CAN_ERR_PROT_TX;
// Bit stream position in CAN frame as the error was detected
    cf.data[3] = ecc & ESD_USB_SJA1000_ECC_SEG;
    }
    if (skb) {
    cf.can_id |= CAN_ERR_CNT;
    cf.data[6] = priv.bec.txerr;
    cf.data[7] = priv.bec.rxerr;
    netif_rx(skb);
    }
    }
    }
    static void esd_usb_rx_can_msg(struct esd_usb_net_priv *priv,
    union esd_usb_msg *msg)
    {
    struct net_device_stats *stats = &priv.netdev.stats;
    struct can_frame *cf;
    struct canfd_frame *cfd;
    struct sk_buff *skb;
    u32 id;
    u8 len;
    if (!netif_device_present(priv.netdev))
    return;
    id = le32_to_cpu(msg.rx.id);
    if (id & ESD_USB_EVENT) {
    esd_usb_rx_event(priv, msg);
    } else {
    if (msg.rx.dlc & ESD_USB_FD) {
    skb = alloc_canfd_skb(priv.netdev, &cfd);
    } else {
    skb = alloc_can_skb(priv.netdev, &cf);
    cfd = (struct canfd_frame *)cf;
    }
    if (skb == core::ptr::null_mut()) {
    stats.rx_dropped++;
    return;
    }
    cfd.can_id = id & ESD_USB_IDMASK;
    if (msg.rx.dlc & ESD_USB_FD) {
// masking by 0x0F is already done within can_fd_dlc2len()
    cfd.len = can_fd_dlc2len(msg.rx.dlc);
    len = cfd.len;
    if ((msg.rx.dlc & ESD_USB_NO_BRS) == 0)
    cfd.flags |= CANFD_BRS;
    if (msg.rx.dlc & ESD_USB_ESI)
    cfd.flags |= CANFD_ESI;
    } else {
    can_frame_set_cc_len(cf, msg.rx.dlc & ~ESD_USB_RTR, priv.can.ctrlmode);
    len = cf.len;
    if (msg.rx.dlc & ESD_USB_RTR) {
    cf.can_id |= CAN_RTR_FLAG;
    len = 0;
    }
    }
    if (id & ESD_USB_EXTID)
    cfd.can_id |= CAN_EFF_FLAG;
    memcpy(cfd.data, msg.rx.data_fd, len);
    stats.rx_bytes += len;
    stats.rx_packets++;
    netif_rx(skb);
    }
    }
    static void esd_usb_tx_done_msg(struct esd_usb_net_priv *priv,
    union esd_usb_msg *msg)
    {
    struct net_device_stats *stats = &priv.netdev.stats;
    struct net_device *netdev = priv.netdev;
    struct esd_tx_urb_context *context;
    if (!netif_device_present(netdev))
    return;
    context = &priv.tx_contexts[msg.txdone.hnd & (ESD_USB_MAX_TX_URBS - 1)];
    if (!msg.txdone.status) {
    stats.tx_packets++;
    stats.tx_bytes += can_get_echo_skb(netdev, context.echo_index,
    core::ptr::null_mut());
    } else {
    stats.tx_errors++;
    can_free_echo_skb(netdev, context.echo_index, core::ptr::null_mut());
    }
// Release context
    context.echo_index = ESD_USB_MAX_TX_URBS;
    atomic_dec(&priv.active_tx_jobs);
    netif_wake_queue(netdev);
    }
#[no_mangle]
unsafe extern "C" fn esd_usb_read_bulk_callback(urb: *mut urb) {
    static void esd_usb_read_bulk_callback(struct urb *urb)
    {
    struct esd_usb *dev = urb.context;
    int err;
    let mut pos: c_int = 0;
    int i;
    switch (urb.status) {
    case 0: /* success */
    break;
    case -ENOENT:
    case -EPIPE:
    case -EPROTO:
    case -ESHUTDOWN:
    return;
    default:
    dev_info(dev.udev.dev.parent,
    "Rx URB aborted (%pe)\n", ERR_PTR(urb.status));
    goto resubmit_urb;
    }
    while (pos < urb.actual_length) {
    union esd_usb_msg *msg;
    msg = (union esd_usb_msg *)(urb.transfer_buffer + pos);
    switch (msg.hdr.cmd) {
    case ESD_USB_CMD_CAN_RX:
    if (msg.rx.net >= dev.net_count) {
    dev_err(dev.udev.dev.parent, "format error\n");
    break;
    }
    esd_usb_rx_can_msg(dev.nets[msg.rx.net], msg);
    break;
    case ESD_USB_CMD_CAN_TX:
    if (msg.txdone.net >= dev.net_count) {
    dev_err(dev.udev.dev.parent, "format error\n");
    break;
    }
    esd_usb_tx_done_msg(dev.nets[msg.txdone.net],
    msg);
    break;
    }
    pos += msg.hdr.len * sizeof(u32); /* convert to # of bytes */
    if (pos > urb.actual_length) {
    dev_err(dev.udev.dev.parent, "format error\n");
    break;
    }
    }
    resubmit_urb:
    usb_fill_bulk_urb(urb, dev.udev, dev.rx_pipe,
    urb.transfer_buffer, ESD_USB_RX_BUFFER_SIZE,
    esd_usb_read_bulk_callback, dev);
    usb_anchor_urb(urb, &dev.rx_submitted);
    err = usb_submit_urb(urb, GFP_ATOMIC);
    if (!err)
    return;
    usb_unanchor_urb(urb);
    if (err == -ENODEV) {
    for (i = 0; i < dev.net_count; i++) {
    if (dev.nets[i])
    netif_device_detach(dev.nets[i].netdev);
    }
    } else {
    dev_err(dev.udev.dev.parent,
    "failed resubmitting read bulk urb: %pe\n", ERR_PTR(err));
    }
    }
// callback for bulk IN urb
#[no_mangle]
unsafe extern "C" fn esd_usb_write_bulk_callback(urb: *mut urb) {
    static void esd_usb_write_bulk_callback(struct urb *urb)
    {
    struct esd_tx_urb_context *context = urb.context;
    struct esd_usb_net_priv *priv;
    struct net_device *netdev;
    let mut size: usize = sizeof(union esd_usb_msg);
    WARN_ON(!context);
    priv = context.priv;
    netdev = priv.netdev;
// free up our allocated buffer
    usb_free_coherent(urb.dev, size,
    urb.transfer_buffer, urb.transfer_dma);
    if (!netif_device_present(netdev))
    return;
    if (urb.status)
    netdev_info(netdev, "Tx URB aborted (%pe)\n", ERR_PTR(urb.status));
    netif_trans_update(netdev);
    }
    static ssize_t firmware_show(struct device *d,
    struct device_attribute *attr, char *buf)
    {
    struct usb_interface *intf = to_usb_interface(d);
    struct esd_usb *dev = usb_get_intfdata(intf);
    return sprintf(buf, "%d.%d.%d\n",
    (dev.version >> 12) & 0xf,
    (dev.version >> 8) & 0xf,
    dev.version & 0xff);
    }
    static DEVICE_ATTR_RO(firmware);
    static ssize_t hardware_show(struct device *d,
    struct device_attribute *attr, char *buf)
    {
    struct usb_interface *intf = to_usb_interface(d);
    struct esd_usb *dev = usb_get_intfdata(intf);
    return sprintf(buf, "%d.%d.%d\n",
    (dev.version >> 28) & 0xf,
    (dev.version >> 24) & 0xf,
    (dev.version >> 16) & 0xff);
    }
    static DEVICE_ATTR_RO(hardware);
    static ssize_t nets_show(struct device *d,
    struct device_attribute *attr, char *buf)
    {
    struct usb_interface *intf = to_usb_interface(d);
    struct esd_usb *dev = usb_get_intfdata(intf);
    return sprintf(buf, "%d", dev.net_count);
    }
    static DEVICE_ATTR_RO(nets);
#[no_mangle]
unsafe extern "C" fn esd_usb_send_msg(dev: *mut esd_usb, msg: *mut union esd_usb_msg) -> c_int {
    static int esd_usb_send_msg(struct esd_usb *dev, union esd_usb_msg *msg)
    {
    int actual_length;
    return usb_bulk_msg(dev.udev, dev.tx_pipe, msg,
    msg.hdr.len * sizeof(u32), /* convert to # of bytes */
    &actual_length,
    1000);
    }
    static int esd_usb_wait_msg(struct esd_usb *dev,
    union esd_usb_msg *msg)
    {
    int actual_length;
    return usb_bulk_msg(dev.udev, dev.rx_pipe, msg,
    sizeof(*msg), &actual_length, 1000);
    }
#[no_mangle]
unsafe extern "C" fn esd_usb_setup_rx_urbs(dev: *mut esd_usb) -> c_int {
    static int esd_usb_setup_rx_urbs(struct esd_usb *dev)
    {
    int i, err = 0;
    if (dev.rxinitdone)
    return 0;
    for (i = 0; i < ESD_USB_MAX_RX_URBS; i++) {
    struct urb *urb = core::ptr::null_mut();
    u8 *buf = core::ptr::null_mut();
    dma_addr_t buf_dma;
// create a URB, and a buffer for it
    urb = usb_alloc_urb(0, GFP_KERNEL);
    if (!urb) {
    err = -ENOMEM;
    break;
    }
    buf = usb_alloc_coherent(dev.udev, ESD_USB_RX_BUFFER_SIZE, GFP_KERNEL,
    &buf_dma);
    if (!buf) {
    dev_warn(dev.udev.dev.parent,
    "No memory left for USB buffer\n");
    err = -ENOMEM;
    goto freeurb;
    }
    urb.transfer_dma = buf_dma;
    usb_fill_bulk_urb(urb, dev.udev, dev.rx_pipe,
    buf, ESD_USB_RX_BUFFER_SIZE,
    esd_usb_read_bulk_callback, dev);
    urb.transfer_flags |= URB_NO_TRANSFER_DMA_MAP;
    usb_anchor_urb(urb, &dev.rx_submitted);
    err = usb_submit_urb(urb, GFP_KERNEL);
    if (err) {
    usb_unanchor_urb(urb);
    usb_free_coherent(dev.udev, ESD_USB_RX_BUFFER_SIZE, buf,
    urb.transfer_dma);
    goto freeurb;
    }
    dev.rxbuf[i] = buf;
    dev.rxbuf_dma[i] = buf_dma;
    freeurb:
// Drop reference, USB core will take care of freeing it
    usb_free_urb(urb);
    if (err)
    break;
    }
// Did we submit any URBs
    if (i == 0) {
    dev_err(dev.udev.dev.parent, "couldn't setup read URBs\n");
    return err;
    }
// Warn if we've couldn't transmit all the URBs
    if (i < ESD_USB_MAX_RX_URBS) {
    dev_warn(dev.udev.dev.parent,
    "rx performance may be slow\n");
    }
    dev.rxinitdone = 1;
    return 0;
    }
// Start interface
#[no_mangle]
unsafe extern "C" fn esd_usb_start(priv: *mut esd_usb_net_priv) -> c_int {
    static int esd_usb_start(struct esd_usb_net_priv *priv)
    {
    struct esd_usb *dev = priv.usb;
    struct net_device *netdev = priv.netdev;
    union esd_usb_msg *msg;
    int err, i;
    msg = kmalloc_obj(*msg);
    if (!msg) {
    err = -ENOMEM;
    goto out;
    }
// Enable all IDs
// The IDADD message takes up to 64 32 bit bitmasks (2048 bits).
// Each bit represents one 11 bit CAN identifier. A set bit
// enables reception of the corresponding CAN identifier. A cleared
// bit disabled this identifier. An additional bitmask value
// following the CAN 2.0A bits is used to enable reception of
// extended CAN frames. Only the LSB of this final mask is checked
// for the complete 29 bit ID range. The IDADD message also allows
// filter configuration for an ID subset. In this case you can add
// the number of the starting bitmask (0..64) to the filter.option
// field followed by only some bitmasks.
//
    msg.hdr.cmd = ESD_USB_CMD_IDADD;
    msg.hdr.len = sizeof(struct esd_usb_id_filter_msg) / sizeof(u32); /* # of 32bit words */
    msg.filter.net = priv.index;
    msg.filter.option = ESD_USB_ID_ENABLE; /* start with segment 0 */
    for (i = 0; i < ESD_USB_MAX_ID_SEGMENT; i++)
    msg.filter.mask[i] = cpu_to_le32(GENMASK(31, 0));
// enable 29bit extended IDs
    msg.filter.mask[ESD_USB_MAX_ID_SEGMENT] = cpu_to_le32(BIT(0));
    err = esd_usb_send_msg(dev, msg);
    if (err)
    goto out;
    err = esd_usb_setup_rx_urbs(dev);
    if (err)
    goto out;
    priv.can.state = CAN_STATE_ERROR_ACTIVE;
    out:
    if (err == -ENODEV)
    netif_device_detach(netdev);
    if (err)
    netdev_err(netdev, "couldn't start device: %pe\n", ERR_PTR(err));
    kfree(msg);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn unlink_all_urbs(dev: *mut esd_usb) {
    static void unlink_all_urbs(struct esd_usb *dev)
    {
    struct esd_usb_net_priv *priv;
    int i, j;
    usb_kill_anchored_urbs(&dev.rx_submitted);
    for (i = 0; i < ESD_USB_MAX_RX_URBS; ++i)
    usb_free_coherent(dev.udev, ESD_USB_RX_BUFFER_SIZE,
    dev.rxbuf[i], dev.rxbuf_dma[i]);
    for (i = 0; i < dev.net_count; i++) {
    priv = dev.nets[i];
    if (priv) {
    usb_kill_anchored_urbs(&priv.tx_submitted);
    atomic_set(&priv.active_tx_jobs, 0);
    for (j = 0; j < ESD_USB_MAX_TX_URBS; j++)
    priv.tx_contexts[j].echo_index = ESD_USB_MAX_TX_URBS;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn esd_usb_open(netdev: *mut net_device) -> c_int {
    static int esd_usb_open(struct net_device *netdev)
    {
    struct esd_usb_net_priv *priv = netdev_priv(netdev);
    int err;
// common open
    err = open_candev(netdev);
    if (err)
    return err;
// finally start device
    err = esd_usb_start(priv);
    if (err) {
    close_candev(netdev);
    return err;
    }
    netif_start_queue(netdev);
    return 0;
    }
    static netdev_tx_t esd_usb_start_xmit(struct sk_buff *skb,
    struct net_device *netdev)
    {
    struct esd_usb_net_priv *priv = netdev_priv(netdev);
    struct esd_usb *dev = priv.usb;
    struct esd_tx_urb_context *context = core::ptr::null_mut();
    struct net_device_stats *stats = &netdev.stats;
    struct canfd_frame *cfd = (struct canfd_frame *)skb.data;
    union esd_usb_msg *msg;
    struct urb *urb;
    u8 *buf;
    int i, err;
    let mut ret: c_int = NETDEV_TX_OK;
    let mut size: usize = sizeof(union esd_usb_msg);
    if (can_dev_dropped_skb(netdev, skb))
    return NETDEV_TX_OK;
// create a URB, and a buffer for it, and copy the data to the URB
    urb = usb_alloc_urb(0, GFP_ATOMIC);
    if (!urb) {
    stats.tx_dropped++;
    dev_kfree_skb(skb);
    goto nourbmem;
    }
    buf = usb_alloc_coherent(dev.udev, size, GFP_ATOMIC,
    &urb.transfer_dma);
    if (!buf) {
    netdev_err(netdev, "No memory left for USB buffer\n");
    stats.tx_dropped++;
    dev_kfree_skb(skb);
    goto nobufmem;
    }
    msg = (union esd_usb_msg *)buf;
// minimal length as # of 32bit words
    msg.hdr.len = offsetof(struct esd_usb_tx_msg, data) / sizeof(u32);
    msg.hdr.cmd = ESD_USB_CMD_CAN_TX;
    msg.tx.net = priv.index;
    if (can_is_canfd_skb(skb)) {
    msg.tx.dlc = can_fd_len2dlc(cfd.len);
    msg.tx.dlc |= ESD_USB_FD;
    if ((cfd.flags & CANFD_BRS) == 0)
    msg.tx.dlc |= ESD_USB_NO_BRS;
    } else {
    msg.tx.dlc = can_get_cc_dlc((struct can_frame *)cfd, priv.can.ctrlmode);
    if (cfd.can_id & CAN_RTR_FLAG)
    msg.tx.dlc |= ESD_USB_RTR;
    }
    msg.tx.id = cpu_to_le32(cfd.can_id & CAN_ERR_MASK);
    if (cfd.can_id & CAN_EFF_FLAG)
    msg.tx.id |= cpu_to_le32(ESD_USB_EXTID);
    memcpy(msg.tx.data_fd, cfd.data, cfd.len);
// round up, then divide by 4 to add the payload length as # of 32bit words
    msg.hdr.len += DIV_ROUND_UP(cfd.len, sizeof(u32));
    for (i = 0; i < ESD_USB_MAX_TX_URBS; i++) {
    if (priv.tx_contexts[i].echo_index == ESD_USB_MAX_TX_URBS) {
    context = &priv.tx_contexts[i];
    break;
    }
    }
// This may never happen
    if (!context) {
    netdev_warn(netdev, "couldn't find free context\n");
    ret = NETDEV_TX_BUSY;
    goto releasebuf;
    }
    context.priv = priv;
    context.echo_index = i;
// hnd must not be 0 - MSB is stripped in txdone handling
    msg.tx.hnd = BIT(31) | i; /* returned in TX done message */
    usb_fill_bulk_urb(urb, dev.udev, dev.tx_pipe, buf,
    msg.hdr.len * sizeof(u32), /* convert to # of bytes */
    esd_usb_write_bulk_callback, context);
    urb.transfer_flags |= URB_NO_TRANSFER_DMA_MAP;
    usb_anchor_urb(urb, &priv.tx_submitted);
    can_put_echo_skb(skb, netdev, context.echo_index, 0);
    atomic_inc(&priv.active_tx_jobs);
// Slow down tx path
    if (atomic_read(&priv.active_tx_jobs) >= ESD_USB_MAX_TX_URBS)
    netif_stop_queue(netdev);
    err = usb_submit_urb(urb, GFP_ATOMIC);
    if (err) {
    can_free_echo_skb(netdev, context.echo_index, core::ptr::null_mut());
    atomic_dec(&priv.active_tx_jobs);
    usb_unanchor_urb(urb);
    stats.tx_dropped++;
    if (err == -ENODEV)
    netif_device_detach(netdev);
    else
    netdev_warn(netdev, "failed tx_urb %pe\n", ERR_PTR(err));
    goto releasebuf;
    }
    netif_trans_update(netdev);
// Release our reference to this URB, the USB core will eventually free
// it entirely.
//
    usb_free_urb(urb);
    return NETDEV_TX_OK;
    releasebuf:
    usb_free_coherent(dev.udev, size, buf, urb.transfer_dma);
    nobufmem:
    usb_free_urb(urb);
    nourbmem:
    return ret;
    }
// Stop interface
#[no_mangle]
unsafe extern "C" fn esd_usb_stop(priv: *mut esd_usb_net_priv) -> c_int {
    static int esd_usb_stop(struct esd_usb_net_priv *priv)
    {
    union esd_usb_msg *msg;
    int err;
    int i;
    msg = kmalloc_obj(*msg);
    if (!msg)
    return -ENOMEM;
// Disable all IDs (see esd_usb_start())
    msg.hdr.cmd = ESD_USB_CMD_IDADD;
    msg.hdr.len = sizeof(struct esd_usb_id_filter_msg) / sizeof(u32);/* # of 32bit words */
    msg.filter.net = priv.index;
    msg.filter.option = ESD_USB_ID_ENABLE; /* start with segment 0 */
    for (i = 0; i <= ESD_USB_MAX_ID_SEGMENT; i++)
    msg.filter.mask[i] = 0;
    err = esd_usb_send_msg(priv.usb, msg);
    if (err < 0) {
    netdev_err(priv.netdev, "sending idadd message failed: %pe\n", ERR_PTR(err));
    goto bail;
    }
// set CAN controller to reset mode
    msg.hdr.len = sizeof(struct esd_usb_set_baudrate_msg) / sizeof(u32); /* # of 32bit words */
    msg.hdr.cmd = ESD_USB_CMD_SETBAUD;
    msg.setbaud.net = priv.index;
    msg.setbaud.rsvd = 0;
    msg.setbaud.baud = cpu_to_le32(ESD_USB_NO_BAUDRATE);
    err = esd_usb_send_msg(priv.usb, msg);
    if (err < 0)
    netdev_err(priv.netdev, "sending setbaud message failed: %pe\n", ERR_PTR(err));
    bail:
    kfree(msg);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn esd_usb_close(netdev: *mut net_device) -> c_int {
    static int esd_usb_close(struct net_device *netdev)
    {
    struct esd_usb_net_priv *priv = netdev_priv(netdev);
    let mut err: c_int = 0;
    if (!priv.usb.in_usb_disconnect) {
// It's moot to try this in usb_disconnect()!
    err = esd_usb_stop(priv);
    }
    priv.can.state = CAN_STATE_STOPPED;
    netif_stop_queue(netdev);
    close_candev(netdev);
    return err;
    }
    static const struct net_device_ops esd_usb_netdev_ops = {
    .ndo_open = esd_usb_open,
    .ndo_stop = esd_usb_close,
    .ndo_start_xmit = esd_usb_start_xmit,
    };
    static const struct ethtool_ops esd_usb_ethtool_ops = {
    .get_ts_info = ethtool_op_get_ts_info,
    };
    static const struct can_bittiming_const esd_usb_2_bittiming_const = {
    .name = "esd_usb_2",
    .tseg1_min = 1,
    .tseg1_max = 16,
    .tseg2_min = 1,
    .tseg2_max = 8,
    .sjw_max = 4,
    .brp_min = 1,
    .brp_max = 1024,
    .brp_inc = 1,
    };
#[no_mangle]
unsafe extern "C" fn esd_usb_2_set_bittiming(netdev: *mut net_device) -> c_int {
    static int esd_usb_2_set_bittiming(struct net_device *netdev)
    {
    const struct can_bittiming_const *btc = &esd_usb_2_bittiming_const;
    struct esd_usb_net_priv *priv = netdev_priv(netdev);
    struct can_bittiming *bt = &priv.can.bittiming;
    union esd_usb_msg *msg;
    int err;
    u32 canbtr;
    int sjw_shift;
    canbtr = ESD_USB_UBR;
    if (priv.can.ctrlmode & CAN_CTRLMODE_LISTENONLY)
    canbtr |= ESD_USB_LOM;
    canbtr |= (bt.brp - 1) & (btc.brp_max - 1);
    if (le16_to_cpu(priv.usb.udev.descriptor.idProduct) ==
    ESD_USB_CANUSBM_PRODUCT_ID)
    sjw_shift = ESD_USB_M_SJW_SHIFT;
    else
    sjw_shift = ESD_USB_2_SJW_SHIFT;
    canbtr |= ((bt.sjw - 1) & (btc.sjw_max - 1))
    << sjw_shift;
    canbtr |= ((bt.prop_seg + bt.phase_seg1 - 1)
    & (btc.tseg1_max - 1))
    << ESD_USB_2_TSEG1_SHIFT;
    canbtr |= ((bt.phase_seg2 - 1) & (btc.tseg2_max - 1))
    << ESD_USB_2_TSEG2_SHIFT;
    if (priv.can.ctrlmode & CAN_CTRLMODE_3_SAMPLES)
    canbtr |= ESD_USB_TRIPLE_SAMPLES;
    msg = kmalloc_obj(*msg);
    if (!msg)
    return -ENOMEM;
    msg.hdr.len = sizeof(struct esd_usb_set_baudrate_msg) / sizeof(u32); /* # of 32bit words */
    msg.hdr.cmd = ESD_USB_CMD_SETBAUD;
    msg.setbaud.net = priv.index;
    msg.setbaud.rsvd = 0;
    msg.setbaud.baud = cpu_to_le32(canbtr);
    netdev_dbg(netdev, "setting BTR=%#x\n", canbtr);
    err = esd_usb_send_msg(priv.usb, msg);
    kfree(msg);
    return err;
    }
// Nominal bittiming constants, see
// Microchip SAM E70/S70/V70/V71, Data Sheet, Rev. G - 07/2022
// 48.6.8 MCAN Nominal Bit Timing and Prescaler Register
//
    static const struct can_bittiming_const esd_usb_3_nom_bittiming_const = {
    .name = "esd_usb_3",
    .tseg1_min = 2,
    .tseg1_max = 256,
    .tseg2_min = 2,
    .tseg2_max = 128,
    .sjw_max = 128,
    .brp_min = 1,
    .brp_max = 512,
    .brp_inc = 1,
    };
// Data bittiming constants, see
// Microchip SAM E70/S70/V70/V71, Data Sheet, Rev. G - 07/2022
// 48.6.4 MCAN Data Bit Timing and Prescaler Register
//
    static const struct can_bittiming_const esd_usb_3_data_bittiming_const = {
    .name = "esd_usb_3",
    .tseg1_min = 2,
    .tseg1_max = 32,
    .tseg2_min = 1,
    .tseg2_max = 16,
    .sjw_max = 8,
    .brp_min = 1,
    .brp_max = 32,
    .brp_inc = 1,
    };
#[no_mangle]
unsafe extern "C" fn esd_usb_3_set_bittiming(netdev: *mut net_device) -> c_int {
    static int esd_usb_3_set_bittiming(struct net_device *netdev)
    {
    const struct can_bittiming_const *nom_btc = &esd_usb_3_nom_bittiming_const;
    const struct can_bittiming_const *data_btc = &esd_usb_3_data_bittiming_const;
    struct esd_usb_net_priv *priv = netdev_priv(netdev);
    struct can_bittiming *nom_bt = &priv.can.bittiming;
    struct can_bittiming *data_bt = &priv.can.fd.data_bittiming;
    struct esd_usb_3_set_baudrate_msg_x *baud_x;
    union esd_usb_msg *msg;
    let mut flags: u16 = 0;
    int err;
    msg = kmalloc_obj(*msg);
    if (!msg)
    return -ENOMEM;
    baud_x = &msg.setbaud_x;
// Canonical is the most reasonable mode for SocketCAN on CAN-USB/3 ...
    baud_x.mode = cpu_to_le16(ESD_USB_3_BAUDRATE_MODE_BTR_CANONICAL);
    if (priv.can.ctrlmode & CAN_CTRLMODE_LISTENONLY)
    flags |= ESD_USB_3_BAUDRATE_FLAG_LOM;
    baud_x.nom.brp = cpu_to_le16(nom_bt.brp & (nom_btc.brp_max - 1));
    baud_x.nom.sjw = cpu_to_le16(nom_bt.sjw & (nom_btc.sjw_max - 1));
    baud_x.nom.tseg1 = cpu_to_le16((nom_bt.prop_seg + nom_bt.phase_seg1)
    & (nom_btc.tseg1_max - 1));
    baud_x.nom.tseg2 = cpu_to_le16(nom_bt.phase_seg2 & (nom_btc.tseg2_max - 1));
    if (priv.can.ctrlmode & CAN_CTRLMODE_FD) {
    baud_x.data.brp = cpu_to_le16(data_bt.brp & (data_btc.brp_max - 1));
    baud_x.data.sjw = cpu_to_le16(data_bt.sjw & (data_btc.sjw_max - 1));
    baud_x.data.tseg1 = cpu_to_le16((data_bt.prop_seg + data_bt.phase_seg1)
    & (data_btc.tseg1_max - 1));
    baud_x.data.tseg2 = cpu_to_le16(data_bt.phase_seg2 & (data_btc.tseg2_max - 1));
    flags |= ESD_USB_3_BAUDRATE_FLAG_FD;
    }
// Currently this driver only supports the automatic TDC mode
    baud_x.tdc.tdc_mode = ESD_USB_3_TDC_MODE_AUTO;
    baud_x.tdc.ssp_offset = 0;
    baud_x.tdc.ssp_shift = 0;
    baud_x.tdc.tdc_filter = 0;
    baud_x.flags = cpu_to_le16(flags);
    baud_x.net = priv.index;
    baud_x.rsvd = 0;
// set len as # of 32bit words
    msg.hdr.len = sizeof(struct esd_usb_3_set_baudrate_msg_x) / sizeof(u32);
    msg.hdr.cmd = ESD_USB_CMD_SETBAUD;
    netdev_dbg(netdev,
    "ctrlmode=%#x/%#x, esd-net=%u, esd-mode=%#x, esd-flags=%#x\n",
    priv.can.ctrlmode, priv.can.ctrlmode_supported,
    priv.index, le16_to_cpu(baud_x.mode), flags);
    err = esd_usb_send_msg(priv.usb, msg);
    kfree(msg);
    return err;
    }
    static int esd_usb_get_berr_counter(const struct net_device *netdev,
    struct can_berr_counter *bec)
    {
    struct esd_usb_net_priv *priv = netdev_priv(netdev);
    bec.txerr = priv.bec.txerr;
    bec.rxerr = priv.bec.rxerr;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn esd_usb_set_mode(netdev: *mut net_device, mode: enum can_mode) -> c_int {
    static int esd_usb_set_mode(struct net_device *netdev, enum can_mode mode)
    {
    switch (mode) {
    case CAN_MODE_START:
    netif_wake_queue(netdev);
    break;
    default:
    return -EOPNOTSUPP;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn esd_usb_probe_one_net(intf: *mut usb_interface, index: c_int) -> c_int {
    static int esd_usb_probe_one_net(struct usb_interface *intf, int index)
    {
    struct esd_usb *dev = usb_get_intfdata(intf);
    struct net_device *netdev;
    struct esd_usb_net_priv *priv;
    let mut err: c_int = 0;
    int i;
    netdev = alloc_candev(sizeof(*priv), ESD_USB_MAX_TX_URBS);
    if (!netdev) {
    dev_err(&intf.dev, "couldn't alloc candev\n");
    err = -ENOMEM;
    goto done;
    }
    priv = netdev_priv(netdev);
    init_usb_anchor(&priv.tx_submitted);
    atomic_set(&priv.active_tx_jobs, 0);
    for (i = 0; i < ESD_USB_MAX_TX_URBS; i++)
    priv.tx_contexts[i].echo_index = ESD_USB_MAX_TX_URBS;
    priv.usb = dev;
    priv.netdev = netdev;
    priv.index = index;
    priv.can.state = CAN_STATE_STOPPED;
    priv.can.ctrlmode_supported = CAN_CTRLMODE_LISTENONLY |
    CAN_CTRLMODE_CC_LEN8_DLC |
    CAN_CTRLMODE_BERR_REPORTING;
    switch (le16_to_cpu(dev.udev.descriptor.idProduct)) {
    case ESD_USB_CANUSB3_PRODUCT_ID:
    priv.can.clock.freq = ESD_USB_3_CAN_CLOCK;
    priv.can.ctrlmode_supported |= CAN_CTRLMODE_FD;
    priv.can.bittiming_const = &esd_usb_3_nom_bittiming_const;
    priv.can.fd.data_bittiming_const = &esd_usb_3_data_bittiming_const;
    priv.can.do_set_bittiming = esd_usb_3_set_bittiming;
    priv.can.fd.do_set_data_bittiming = esd_usb_3_set_bittiming;
    break;
    case ESD_USB_CANUSBM_PRODUCT_ID:
    priv.can.clock.freq = ESD_USB_M_CAN_CLOCK;
    priv.can.bittiming_const = &esd_usb_2_bittiming_const;
    priv.can.do_set_bittiming = esd_usb_2_set_bittiming;
    break;
    case ESD_USB_CANUSB2_PRODUCT_ID:
    default:
    priv.can.clock.freq = ESD_USB_2_CAN_CLOCK;
    priv.can.ctrlmode_supported |= CAN_CTRLMODE_3_SAMPLES;
    priv.can.bittiming_const = &esd_usb_2_bittiming_const;
    priv.can.do_set_bittiming = esd_usb_2_set_bittiming;
    break;
    }
    priv.can.do_set_mode = esd_usb_set_mode;
    priv.can.do_get_berr_counter = esd_usb_get_berr_counter;
    netdev.flags |= IFF_ECHO; /* we support local echo */
    netdev.netdev_ops = &esd_usb_netdev_ops;
    netdev.ethtool_ops = &esd_usb_ethtool_ops;
    SET_NETDEV_DEV(netdev, &intf.dev);
    netdev.dev_id = index;
    err = register_candev(netdev);
    if (err) {
    dev_err(&intf.dev, "couldn't register CAN device: %pe\n", ERR_PTR(err));
    free_candev(netdev);
    err = -ENOMEM;
    goto done;
    }
    dev.nets[index] = priv;
    netdev_info(netdev, "registered\n");
    done:
    return err;
    }
// probe function for new USB devices
//
// check version information and number of available
// CAN interfaces
//
    static int esd_usb_probe(struct usb_interface *intf,
    const struct usb_device_id *id)
    {
    struct usb_endpoint_descriptor *ep_in, *ep_out;
    struct esd_usb *dev;
    union esd_usb_msg *msg;
    int i, err;
    err = usb_find_common_endpoints(intf.cur_altsetting, &ep_in, &ep_out,
    core::ptr::null_mut(), core::ptr::null_mut());
    if (err)
    return err;
    dev = kzalloc_obj(*dev);
    if (!dev) {
    err = -ENOMEM;
    goto done;
    }
    dev.udev = interface_to_usbdev(intf);
    dev.rx_pipe = usb_rcvbulkpipe(dev.udev, ep_in.bEndpointAddress);
    dev.tx_pipe = usb_sndbulkpipe(dev.udev, ep_out.bEndpointAddress);
    init_usb_anchor(&dev.rx_submitted);
    usb_set_intfdata(intf, dev);
    msg = kmalloc_obj(*msg);
    if (!msg) {
    err = -ENOMEM;
    goto free_msg;
    }
// query number of CAN interfaces (nets)
    msg.hdr.cmd = ESD_USB_CMD_VERSION;
    msg.hdr.len = sizeof(struct esd_usb_version_msg) / sizeof(u32); /* # of 32bit words */
    msg.version.rsvd = 0;
    msg.version.flags = 0;
    msg.version.drv_version = 0;
    err = esd_usb_send_msg(dev, msg);
    if (err < 0) {
    dev_err(&intf.dev, "sending version message failed\n");
    goto free_msg;
    }
    err = esd_usb_wait_msg(dev, msg);
    if (err < 0) {
    dev_err(&intf.dev, "no version message answer\n");
    goto free_msg;
    }
    dev.net_count = (int)msg.version_reply.nets;
    dev.version = le32_to_cpu(msg.version_reply.version);
    if (device_create_file(&intf.dev, &dev_attr_firmware))
    dev_err(&intf.dev,
    "Couldn't create device file for firmware\n");
    if (device_create_file(&intf.dev, &dev_attr_hardware))
    dev_err(&intf.dev,
    "Couldn't create device file for hardware\n");
    if (device_create_file(&intf.dev, &dev_attr_nets))
    dev_err(&intf.dev,
    "Couldn't create device file for nets\n");
// do per device probing
    for (i = 0; i < dev.net_count; i++)
    esd_usb_probe_one_net(intf, i);
    free_msg:
    kfree(msg);
    if (err)
    kfree(dev);
    done:
    return err;
    }
// called by the usb core when the device is removed from the system
#[no_mangle]
unsafe extern "C" fn esd_usb_disconnect(intf: *mut usb_interface) {
    static void esd_usb_disconnect(struct usb_interface *intf)
    {
    struct esd_usb *dev = usb_get_intfdata(intf);
    struct net_device *netdev;
    int i;
    device_remove_file(&intf.dev, &dev_attr_firmware);
    device_remove_file(&intf.dev, &dev_attr_hardware);
    device_remove_file(&intf.dev, &dev_attr_nets);
    usb_set_intfdata(intf, core::ptr::null_mut());
    if (dev) {
    dev.in_usb_disconnect = 1;
    for (i = 0; i < dev.net_count; i++) {
    if (dev.nets[i]) {
    netdev = dev.nets[i].netdev;
    netdev_info(netdev, "unregister\n");
    unregister_netdev(netdev);
    }
    }
    unlink_all_urbs(dev);
    for (i = 0; i < dev.net_count; i++) {
    if (dev.nets[i])
    free_candev(dev.nets[i].netdev);
    }
    kfree(dev);
    }
    }
// usb specific object needed to register this driver with the usb subsystem
    static struct usb_driver esd_usb_driver = {
    .name = KBUILD_MODNAME,
    .probe = esd_usb_probe,
    .disconnect = esd_usb_disconnect,
    .id_table = esd_usb_table,
    };
    module_usb_driver(esd_usb_driver);
