//! Automatically rewritten from C to Rust
//! Source: drivers/most/most_usb.c
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
// usb.c - Hardware dependent module for USB
//
// Copyright (C) 2013-2015 Microchip Technology Germany II GmbH & Co. KG
//

pub const USB_MTU: c_int = 512;
pub const NO_ISOCHRONOUS_URB: c_int = 0;
pub const AV_PACKETS_PER_XACT: c_int = 2;
pub const BUF_CHAIN_SIZE: c_uint = 0xFFFF;
pub const MAX_NUM_ENDPOINTS: c_int = 30;
pub const MAX_SUFFIX_LEN: c_int = 10;
pub const MAX_STRING_LEN: c_int = 80;
pub const MAX_BUF_SIZE: c_uint = 0xFFFF;
pub const USB_VENDOR_ID_SMSC: c_uint = 0x0424  /* VID: SMSC */;
pub const USB_DEV_ID_BRDG: c_uint = 0xC001  /* PID: USB Bridge */;
pub const USB_DEV_ID_OS81118: c_uint = 0xCF18  /* PID: USB OS81118 */;
pub const USB_DEV_ID_OS81119: c_uint = 0xCF19  /* PID: USB OS81119 */;
pub const USB_DEV_ID_OS81210: c_uint = 0xCF30  /* PID: USB OS81210 */;
// DRCI Addresses
pub const DRCI_REG_NI_STATE: c_uint = 0x0100;
pub const DRCI_REG_PACKET_BW: c_uint = 0x0101;
pub const DRCI_REG_NODE_ADDR: c_uint = 0x0102;
pub const DRCI_REG_NODE_POS: c_uint = 0x0103;
pub const DRCI_REG_MEP_FILTER: c_uint = 0x0140;
pub const DRCI_REG_HASH_TBL0: c_uint = 0x0141;
pub const DRCI_REG_HASH_TBL1: c_uint = 0x0142;
pub const DRCI_REG_HASH_TBL2: c_uint = 0x0143;
pub const DRCI_REG_HASH_TBL3: c_uint = 0x0144;
pub const DRCI_REG_HW_ADDR_HI: c_uint = 0x0145;
pub const DRCI_REG_HW_ADDR_MI: c_uint = 0x0146;
pub const DRCI_REG_HW_ADDR_LO: c_uint = 0x0147;
pub const DRCI_REG_BASE: c_uint = 0x1100;
pub const DRCI_COMMAND: c_uint = 0x02;
pub const DRCI_READ_REQ: c_uint = 0xA0;
pub const DRCI_WRITE_REQ: c_uint = 0xA1;
//
// struct most_dci_obj - Direct Communication Interface
// @kobj:position in sysfs
// @usb_device: pointer to the usb device
// @reg_addr: register address for arbitrary DCI access
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct most_dci_obj {
    pub dev: device,
    pub usb_device: *mut usb_device,
    pub reg_addr: u16,
}

    struct most_dev;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clear_hold_work {
    pub ws: work_struct,
    pub mdev: *mut most_dev,
    pub channel: c_uint,
    pub pipe: c_int,
}

//
// struct most_dev - holds all usb interface specific stuff
// @usb_device: pointer to usb device
// @iface: hardware interface
// @cap: channel capabilities
// @conf: channel configuration
// @dci: direct communication interface of hardware
// @ep_address: endpoint address table
// @description: device description
// @suffix: suffix for channel name
// @channel_lock: synchronize channel access
// @padding_active: indicates channel uses padding
// @is_channel_healthy: health status table of each channel
// @busy_urbs: list of anchored items
// @io_mutex: synchronize I/O with disconnect
// @link_stat_timer: timer for link status reports
// @poll_work_obj: work for polling link status
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct most_dev {
    pub dev: device,
    pub usb_device: *mut usb_device,
    pub iface: most_interface,
    pub cap: *mut most_channel_capability,
    pub conf: *mut most_channel_config,
    pub dci: *mut most_dci_obj,
    pub ep_address: *mut u8,
    pub description: [c_char; MAX_STRING_LEN],
    pub suffix: [c_char; MAX_NUM_ENDPOINTS][MAX_SUFFIX_LEN],
    pub /: *mut *mut spinlock_t channel_lock[MAX_NUM_ENDPOINTS]; / sync channel access,
    pub padding_active: [bool; MAX_NUM_ENDPOINTS],
    pub is_channel_healthy: [bool; MAX_NUM_ENDPOINTS],
    pub clear_work: [clear_hold_work; MAX_NUM_ENDPOINTS],
    pub busy_urbs: *mut usb_anchor,
    pub io_mutex: mutex,
    pub link_stat_timer: timer_list,
    pub poll_work_obj: work_struct,
    void (*on_netinfo)(struct most_interface *most_iface,
    pub addrs): *mut unsigned char link_state, unsigned char,
}

    static void wq_clear_halt(struct work_struct *wq_obj);
    static void wq_netinfo(struct work_struct *wq_obj);
//
// drci_rd_reg - read a DCI register
// @dev: usb device
// @reg: register address
// @buf: buffer to store data
//
// This is reads data from INIC's direct register communication interface
//
#[no_mangle]
pub unsafe extern "C" fn drci_rd_reg(dev: *mut usb_device, reg: u16, buf: *mut u16) -> c_int {
    static inline int drci_rd_reg(struct usb_device *dev, u16 reg, u16 *buf)
    {
    int retval;
    __le16 *dma_buf;
    let mut req_type: u8 = USB_DIR_IN | USB_TYPE_VENDOR | USB_RECIP_DEVICE;
    dma_buf = kzalloc_obj(*dma_buf);
    if (!dma_buf)
    return -ENOMEM;
    retval = usb_control_msg(dev, usb_rcvctrlpipe(dev, 0),
    DRCI_READ_REQ, req_type,
    0x0000,
    reg, dma_buf, sizeof(*dma_buf),
    USB_CTRL_GET_TIMEOUT);
// buf = le16_to_cpu(*dma_buf);
    kfree(dma_buf);
    if (retval < 0)
    return retval;
    return 0;
    }
//
// drci_wr_reg - write a DCI register
// @dev: usb device
// @reg: register address
// @data: data to write
//
// This is writes data to INIC's direct register communication interface
//
#[no_mangle]
pub unsafe extern "C" fn drci_wr_reg(dev: *mut usb_device, reg: u16, data: u16) -> c_int {
    static inline int drci_wr_reg(struct usb_device *dev, u16 reg, u16 data)
    {
    return usb_control_msg(dev,
    usb_sndctrlpipe(dev, 0),
    DRCI_WRITE_REQ,
    USB_DIR_OUT | USB_TYPE_VENDOR | USB_RECIP_DEVICE,
    data,
    reg,
    core::ptr::null_mut(),
    0,
    USB_CTRL_SET_TIMEOUT);
    }
#[no_mangle]
pub unsafe extern "C" fn start_sync_ep(usb_dev: *mut usb_device, ep: u16) -> c_int {
    static inline int start_sync_ep(struct usb_device *usb_dev, u16 ep)
    {
    return drci_wr_reg(usb_dev, DRCI_REG_BASE + DRCI_COMMAND + ep * 16, 1);
    }
//
// get_stream_frame_size - calculate frame size of current configuration
// @dev: device structure
// @cfg: channel configuration
//
    static unsigned int get_stream_frame_size(struct device *dev,
    struct most_channel_config *cfg)
    {
    unsigned int frame_size;
    let mut sub_size: c_uint = cfg.subbuffer_size;
    if (!sub_size) {
    dev_warn(dev, "Misconfig: Subbuffer size zero.\n");
    return 0;
    }
    switch (cfg.data_type) {
    case MOST_CH_ISOC:
    frame_size = AV_PACKETS_PER_XACT * sub_size;
    break;
    case MOST_CH_SYNC:
    if (cfg.packets_per_xact == 0) {
    dev_warn(dev, "Misconfig: Packets per XACT zero\n");
    frame_size = 0;
    } else if (cfg.packets_per_xact == 0xFF) {
    frame_size = (USB_MTU / sub_size) * sub_size;
    } else {
    frame_size = cfg.packets_per_xact * sub_size;
    }
    break;
    default:
    dev_warn(dev, "Query frame size of non-streaming channel\n");
    frame_size = 0;
    break;
    }
    return frame_size;
    }
//
// hdm_poison_channel - mark buffers of this channel as invalid
// @iface: pointer to the interface
// @channel: channel ID
//
// This unlinks all URBs submitted to the HCD,
// calls the associated completion function of the core and removes
// them from the list.
//
// Returns 0 on success or error code otherwise.
//
#[no_mangle]
unsafe extern "C" fn hdm_poison_channel(iface: *mut most_interface, channel: c_int) -> c_int {
    static int hdm_poison_channel(struct most_interface *iface, int channel)
    {
    struct most_dev *mdev = to_mdev(iface);
    unsigned long flags;
    spinlock_t *lock; /* temp. lock */
    if (channel < 0 || channel >= iface.num_channels) {
    dev_warn(&mdev.usb_device.dev, "Channel ID out of range.\n");
    return -ECHRNG;
    }
    lock = mdev.channel_lock + channel;
    spin_lock_irqsave(lock, flags);
    mdev.is_channel_healthy[channel] = false;
    spin_unlock_irqrestore(lock, flags);
    cancel_work_sync(&mdev.clear_work[channel].ws);
    mutex_lock(&mdev.io_mutex);
    usb_kill_anchored_urbs(&mdev.busy_urbs[channel]);
    if (mdev.padding_active[channel])
    mdev.padding_active[channel] = false;
    if (mdev.conf[channel].data_type == MOST_CH_ASYNC) {
    timer_delete_sync(&mdev.link_stat_timer);
    cancel_work_sync(&mdev.poll_work_obj);
    }
    mutex_unlock(&mdev.io_mutex);
    return 0;
    }
//
// hdm_add_padding - add padding bytes
// @mdev: most device
// @channel: channel ID
// @mbo: buffer object
//
// This inserts the INIC hardware specific padding bytes into a streaming
// channel's buffer
//
#[no_mangle]
unsafe extern "C" fn hdm_add_padding(mdev: *mut most_dev, channel: c_int, mbo: *mut mbo) -> c_int {
    static int hdm_add_padding(struct most_dev *mdev, int channel, struct mbo *mbo)
    {
    struct most_channel_config *conf = &mdev.conf[channel];
    let mut frame_size: c_uint = get_stream_frame_size(&mdev.dev, conf);
    unsigned int j, num_frames;
    if (!frame_size)
    return -EINVAL;
    num_frames = mbo.buffer_length / frame_size;
    if (num_frames < 1) {
    dev_err(&mdev.usb_device.dev,
    "Missed minimal transfer unit.\n");
    return -EINVAL;
    }
    for (j = num_frames - 1; j > 0; j--)
    memmove(mbo.virt_address + j * USB_MTU,
    mbo.virt_address + j * frame_size,
    frame_size);
    mbo.buffer_length = num_frames * USB_MTU;
    return 0;
    }
//
// hdm_remove_padding - remove padding bytes
// @mdev: most device
// @channel: channel ID
// @mbo: buffer object
//
// This takes the INIC hardware specific padding bytes off a streaming
// channel's buffer.
//
    static int hdm_remove_padding(struct most_dev *mdev, int channel,
    struct mbo *mbo)
    {
    let mut conf: *mut most_channel_config const = &mdev.conf[channel];
    let mut frame_size: c_uint = get_stream_frame_size(&mdev.dev, conf);
    unsigned int j, num_frames;
    if (!frame_size)
    return -EINVAL;
    num_frames = mbo.processed_length / USB_MTU;
    for (j = 1; j < num_frames; j++)
    memmove(mbo.virt_address + frame_size * j,
    mbo.virt_address + USB_MTU * j,
    frame_size);
    mbo.processed_length = frame_size * num_frames;
    return 0;
    }
//
// hdm_write_completion - completion function for submitted Tx URBs
// @urb: the URB that has been completed
//
// This checks the status of the completed URB. In case the URB has been
// unlinked before, it is immediately freed. On any other error the MBO
// transfer flag is set. On success it frees allocated resources and calls
// the completion function.
//
// Context: interrupt!
//
#[no_mangle]
unsafe extern "C" fn hdm_write_completion(urb: *mut urb) {
    static void hdm_write_completion(struct urb *urb)
    {
    struct mbo *mbo = urb.context;
    struct most_dev *mdev = to_mdev(mbo.ifp);
    let mut channel: c_uint = mbo.hdm_channel_id;
    spinlock_t *lock = mdev.channel_lock + channel;
    unsigned long flags;
    spin_lock_irqsave(lock, flags);
    mbo.processed_length = 0;
    mbo.status = MBO_E_INVAL;
    if (likely(mdev.is_channel_healthy[channel])) {
    switch (urb.status) {
    case 0:
    case -ESHUTDOWN:
    mbo.processed_length = urb.actual_length;
    mbo.status = MBO_SUCCESS;
    break;
    case -EPIPE:
    dev_warn(&mdev.usb_device.dev,
    "Broken pipe on ep%02x\n",
    mdev.ep_address[channel]);
    mdev.is_channel_healthy[channel] = false;
    mdev.clear_work[channel].pipe = urb.pipe;
    schedule_work(&mdev.clear_work[channel].ws);
    break;
    case -ENODEV:
    case -EPROTO:
    mbo.status = MBO_E_CLOSE;
    break;
    }
    }
    spin_unlock_irqrestore(lock, flags);
    if (likely(mbo.complete))
    mbo.complete(mbo);
    usb_free_urb(urb);
    }
//
// hdm_read_completion - completion function for submitted Rx URBs
// @urb: the URB that has been completed
//
// This checks the status of the completed URB. In case the URB has been
// unlinked before it is immediately freed. On any other error the MBO transfer
// flag is set. On success it frees allocated resources, removes
// padding bytes -if necessary- and calls the completion function.
//
// Context: interrupt!
//
#[no_mangle]
unsafe extern "C" fn hdm_read_completion(urb: *mut urb) {
    static void hdm_read_completion(struct urb *urb)
    {
    struct mbo *mbo = urb.context;
    struct most_dev *mdev = to_mdev(mbo.ifp);
    let mut channel: c_uint = mbo.hdm_channel_id;
    struct device *dev = &mdev.usb_device.dev;
    spinlock_t *lock = mdev.channel_lock + channel;
    unsigned long flags;
    spin_lock_irqsave(lock, flags);
    mbo.processed_length = 0;
    mbo.status = MBO_E_INVAL;
    if (likely(mdev.is_channel_healthy[channel])) {
    switch (urb.status) {
    case 0:
    case -ESHUTDOWN:
    mbo.processed_length = urb.actual_length;
    mbo.status = MBO_SUCCESS;
    if (mdev.padding_active[channel] &&
    hdm_remove_padding(mdev, channel, mbo)) {
    mbo.processed_length = 0;
    mbo.status = MBO_E_INVAL;
    }
    break;
    case -EPIPE:
    dev_warn(dev, "Broken pipe on ep%02x\n",
    mdev.ep_address[channel]);
    mdev.is_channel_healthy[channel] = false;
    mdev.clear_work[channel].pipe = urb.pipe;
    schedule_work(&mdev.clear_work[channel].ws);
    break;
    case -ENODEV:
    case -EPROTO:
    mbo.status = MBO_E_CLOSE;
    break;
    case -EOVERFLOW:
    dev_warn(dev, "Babble on ep%02x\n",
    mdev.ep_address[channel]);
    break;
    }
    }
    spin_unlock_irqrestore(lock, flags);
    if (likely(mbo.complete))
    mbo.complete(mbo);
    usb_free_urb(urb);
    }
//
// hdm_enqueue - receive a buffer to be used for data transfer
// @iface: interface to enqueue to
// @channel: ID of the channel
// @mbo: pointer to the buffer object
//
// This allocates a new URB and fills it according to the channel
// that is being used for transmission of data. Before the URB is
// submitted it is stored in the private anchor list.
//
// Returns 0 on success. On any error the URB is freed and a error code
// is returned.
//
// Context: Could in _some_ cases be interrupt!
//
    static int hdm_enqueue(struct most_interface *iface, int channel,
    struct mbo *mbo)
    {
    struct most_dev *mdev = to_mdev(iface);
    struct most_channel_config *conf;
    let mut retval: c_int = 0;
    struct urb *urb;
    unsigned long length;
    void *virt_address;
    if (!mbo)
    return -EINVAL;
    if (iface.num_channels <= channel || channel < 0)
    return -ECHRNG;
    urb = usb_alloc_urb(NO_ISOCHRONOUS_URB, GFP_KERNEL);
    if (!urb)
    return -ENOMEM;
    conf = &mdev.conf[channel];
    mutex_lock(&mdev.io_mutex);
    if (!mdev.usb_device) {
    retval = -ENODEV;
    goto err_free_urb;
    }
    if ((conf.direction & MOST_CH_TX) && mdev.padding_active[channel] &&
    hdm_add_padding(mdev, channel, mbo)) {
    retval = -EINVAL;
    goto err_free_urb;
    }
    urb.transfer_dma = mbo.bus_address;
    virt_address = mbo.virt_address;
    length = mbo.buffer_length;
    if (conf.direction & MOST_CH_TX) {
    usb_fill_bulk_urb(urb, mdev.usb_device,
    usb_sndbulkpipe(mdev.usb_device,
    mdev.ep_address[channel]),
    virt_address,
    length,
    hdm_write_completion,
    mbo);
    if (conf.data_type != MOST_CH_ISOC &&
    conf.data_type != MOST_CH_SYNC)
    urb.transfer_flags |= URB_ZERO_PACKET;
    } else {
    usb_fill_bulk_urb(urb, mdev.usb_device,
    usb_rcvbulkpipe(mdev.usb_device,
    mdev.ep_address[channel]),
    virt_address,
    length + conf.extra_len,
    hdm_read_completion,
    mbo);
    }
    urb.transfer_flags |= URB_NO_TRANSFER_DMA_MAP;
    usb_anchor_urb(urb, &mdev.busy_urbs[channel]);
    retval = usb_submit_urb(urb, GFP_KERNEL);
    if (retval) {
    dev_err(&mdev.usb_device.dev,
    "URB submit failed with error %d.\n", retval);
    goto err_unanchor_urb;
    }
    mutex_unlock(&mdev.io_mutex);
    return 0;
    err_unanchor_urb:
    usb_unanchor_urb(urb);
    err_free_urb:
    usb_free_urb(urb);
    mutex_unlock(&mdev.io_mutex);
    return retval;
    }
    static void *hdm_dma_alloc(struct mbo *mbo, u32 size)
    {
    struct most_dev *mdev = to_mdev(mbo.ifp);
    return usb_alloc_coherent(mdev.usb_device, size, GFP_KERNEL,
    &mbo.bus_address);
    }
#[no_mangle]
unsafe extern "C" fn hdm_dma_free(mbo: *mut mbo, size: u32) {
    static void hdm_dma_free(struct mbo *mbo, u32 size)
    {
    struct most_dev *mdev = to_mdev(mbo.ifp);
    usb_free_coherent(mdev.usb_device, size, mbo.virt_address,
    mbo.bus_address);
    }
//
// hdm_configure_channel - receive channel configuration from core
// @iface: interface
// @channel: channel ID
// @conf: structure that holds the configuration information
//
// The attached network interface controller (NIC) supports a padding mode
// to avoid short packets on USB, hence increasing the performance due to a
// lower interrupt load. This mode is default for synchronous data and can
// be switched on for isochronous data. In case padding is active the
// driver needs to know the frame size of the payload in order to calculate
// the number of bytes it needs to pad when transmitting or to cut off when
// receiving data.
//
    static int hdm_configure_channel(struct most_interface *iface, int channel,
    struct most_channel_config *conf)
    {
    unsigned int num_frames;
    unsigned int frame_size;
    struct most_dev *mdev = to_mdev(iface);
    struct device *dev = &mdev.usb_device.dev;
    if (!conf) {
    dev_err(dev, "Bad config pointer.\n");
    return -EINVAL;
    }
    if (channel < 0 || channel >= iface.num_channels) {
    dev_err(dev, "Channel ID out of range.\n");
    return -EINVAL;
    }
    mdev.is_channel_healthy[channel] = true;
    mdev.clear_work[channel].channel = channel;
    mdev.clear_work[channel].mdev = mdev;
    INIT_WORK(&mdev.clear_work[channel].ws, wq_clear_halt);
    if (!conf.num_buffers || !conf.buffer_size) {
    dev_err(dev, "Misconfig: buffer size or #buffers zero.\n");
    return -EINVAL;
    }
    if (conf.data_type != MOST_CH_SYNC &&
    !(conf.data_type == MOST_CH_ISOC &&
    conf.packets_per_xact != 0xFF)) {
    mdev.padding_active[channel] = false;
//
// Since the NIC's padding mode is not going to be
// used, we can skip the frame size calculations and
// move directly on to exit.
//
    goto exit;
    }
    mdev.padding_active[channel] = true;
    frame_size = get_stream_frame_size(&mdev.dev, conf);
    if (frame_size == 0 || frame_size > USB_MTU) {
    dev_warn(dev, "Misconfig: frame size wrong\n");
    return -EINVAL;
    }
    num_frames = conf.buffer_size / frame_size;
    if (conf.buffer_size % frame_size) {
    let mut old_size: u16 = conf.buffer_size;
    conf.buffer_size = num_frames * frame_size;
    dev_warn(dev, "%s: fixed buffer size (%d . %d)\n",
    mdev.suffix[channel], old_size, conf.buffer_size);
    }
// calculate extra length to comply w/ HW padding
    conf.extra_len = num_frames * (USB_MTU - frame_size);
    exit:
    mdev.conf[channel] = *conf;
    if (conf.data_type == MOST_CH_ASYNC) {
    let mut ep: u16 = mdev.ep_address[channel];
    if (start_sync_ep(mdev.usb_device, ep) < 0)
    dev_warn(dev, "sync for ep%02x failed", ep);
    }
    return 0;
    }
//
// hdm_request_netinfo - request network information
// @iface: pointer to interface
// @channel: channel ID
//
// This is used as trigger to set up the link status timer that
// polls for the NI state of the INIC every 2 seconds.
//
    static void hdm_request_netinfo(struct most_interface *iface, int channel,
    void (*on_netinfo)(struct most_interface *,
    unsigned char,
    unsigned char *))
    {
    struct most_dev *mdev = to_mdev(iface);
    mdev.on_netinfo = on_netinfo;
    if (!on_netinfo)
    return;
    mdev.link_stat_timer.expires = jiffies + HZ;
    mod_timer(&mdev.link_stat_timer, mdev.link_stat_timer.expires);
    }
//
// link_stat_timer_handler - schedule work obtaining mac address and link status
// @t: pointer to timer_list which holds a pointer to the USB device instance
//
// The handler runs in interrupt context. That's why we need to defer the
// tasks to a work queue.
//
#[no_mangle]
unsafe extern "C" fn link_stat_timer_handler(t: *mut timer_list) {
    static void link_stat_timer_handler(struct timer_list *t)
    {
    struct most_dev *mdev = timer_container_of(mdev, t, link_stat_timer);
    schedule_work(&mdev.poll_work_obj);
    mdev.link_stat_timer.expires = jiffies + (2 * HZ);
    add_timer(&mdev.link_stat_timer);
    }
//
// wq_netinfo - work queue function to deliver latest networking information
// @wq_obj: object that holds data for our deferred work to do
//
// This retrieves the network interface status of the USB INIC
//
#[no_mangle]
unsafe extern "C" fn wq_netinfo(wq_obj: *mut work_struct) {
    static void wq_netinfo(struct work_struct *wq_obj)
    {
    struct most_dev *mdev = to_mdev_from_work(wq_obj);
    struct usb_device *usb_device = mdev.usb_device;
    struct device *dev = &usb_device.dev;
    u16 hi, mi, lo, link;
    u8 hw_addr[6];
    if (drci_rd_reg(usb_device, DRCI_REG_HW_ADDR_HI, &hi)) {
    dev_err(dev, "Vendor request 'hw_addr_hi' failed\n");
    return;
    }
    if (drci_rd_reg(usb_device, DRCI_REG_HW_ADDR_MI, &mi)) {
    dev_err(dev, "Vendor request 'hw_addr_mid' failed\n");
    return;
    }
    if (drci_rd_reg(usb_device, DRCI_REG_HW_ADDR_LO, &lo)) {
    dev_err(dev, "Vendor request 'hw_addr_low' failed\n");
    return;
    }
    if (drci_rd_reg(usb_device, DRCI_REG_NI_STATE, &link)) {
    dev_err(dev, "Vendor request 'link status' failed\n");
    return;
    }
    hw_addr[0] = hi >> 8;
    hw_addr[1] = hi;
    hw_addr[2] = mi >> 8;
    hw_addr[3] = mi;
    hw_addr[4] = lo >> 8;
    hw_addr[5] = lo;
    if (mdev.on_netinfo)
    mdev.on_netinfo(&mdev.iface, link, hw_addr);
    }
//
// wq_clear_halt - work queue function
// @wq_obj: work_struct object to execute
//
// This sends a clear_halt to the given USB pipe.
//
#[no_mangle]
unsafe extern "C" fn wq_clear_halt(wq_obj: *mut work_struct) {
    static void wq_clear_halt(struct work_struct *wq_obj)
    {
    struct clear_hold_work *clear_work = to_clear_hold_work(wq_obj);
    struct most_dev *mdev = clear_work.mdev;
    let mut channel: c_uint = clear_work.channel;
    let mut pipe: c_int = clear_work.pipe;
    int snd_pipe;
    int peer;
    mutex_lock(&mdev.io_mutex);
    most_stop_enqueue(&mdev.iface, channel);
    usb_kill_anchored_urbs(&mdev.busy_urbs[channel]);
    if (usb_clear_halt(mdev.usb_device, pipe))
    dev_warn(&mdev.usb_device.dev, "Failed to reset endpoint.\n");
// If the functional Stall condition has been set on an
// asynchronous rx channel, we need to clear the tx channel
// too, since the hardware runs its clean-up sequence on both
// channels, as they are physically one on the network.
//
// The USB interface that exposes the asynchronous channels
// contains always two endpoints, and two only.
//
    if (mdev.conf[channel].data_type == MOST_CH_ASYNC &&
    mdev.conf[channel].direction == MOST_CH_RX) {
    if (channel == 0)
    peer = 1;
    else
    peer = 0;
    snd_pipe = usb_sndbulkpipe(mdev.usb_device,
    mdev.ep_address[peer]);
    usb_clear_halt(mdev.usb_device, snd_pipe);
    }
    mdev.is_channel_healthy[channel] = true;
    most_resume_enqueue(&mdev.iface, channel);
    mutex_unlock(&mdev.io_mutex);
    }
//
// hdm_usb_fops - file operation table for USB driver
//
    static const struct file_operations hdm_usb_fops = {
    .owner = THIS_MODULE,
    };
//
// usb_device_id - ID table for HCD device probing
//
    static const struct usb_device_id usbid[] = {
    { USB_DEVICE(USB_VENDOR_ID_SMSC, USB_DEV_ID_BRDG), },
    { USB_DEVICE(USB_VENDOR_ID_SMSC, USB_DEV_ID_OS81118), },
    { USB_DEVICE(USB_VENDOR_ID_SMSC, USB_DEV_ID_OS81119), },
    { USB_DEVICE(USB_VENDOR_ID_SMSC, USB_DEV_ID_OS81210), },
    { } /* Terminating entry */
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct regs {
    pub name: *const c_char,
    pub reg: u16,
}

    static const struct regs ro_regs[] = {
    { "ni_state", DRCI_REG_NI_STATE },
    { "packet_bandwidth", DRCI_REG_PACKET_BW },
    { "node_address", DRCI_REG_NODE_ADDR },
    { "node_position", DRCI_REG_NODE_POS },
    };
    static const struct regs rw_regs[] = {
    { "mep_filter", DRCI_REG_MEP_FILTER },
    { "mep_hash0", DRCI_REG_HASH_TBL0 },
    { "mep_hash1", DRCI_REG_HASH_TBL1 },
    { "mep_hash2", DRCI_REG_HASH_TBL2 },
    { "mep_hash3", DRCI_REG_HASH_TBL3 },
    { "mep_eui48_hi", DRCI_REG_HW_ADDR_HI },
    { "mep_eui48_mi", DRCI_REG_HW_ADDR_MI },
    { "mep_eui48_lo", DRCI_REG_HW_ADDR_LO },
    };
    static int get_stat_reg_addr(const struct regs *regs, int size,
    const char *name, u16 *reg_addr)
    {
    int i;
    for (i = 0; i < size; i++) {
    if (sysfs_streq(name, regs[i].name)) {
// reg_addr = regs[i].reg;
    return 0;
    }
    }
    return -EINVAL;
    }

    get_stat_reg_addr(regs, ARRAY_SIZE(regs), name, reg_addr)
    static ssize_t value_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    const char *name = attr.attr.name;
    struct most_dci_obj *dci_obj = to_dci_obj(dev);
    u16 val;
    u16 reg_addr;
    int err;
    if (sysfs_streq(name, "arb_address"))
    return sysfs_emit(buf, "%04x\n", dci_obj.reg_addr);
    if (sysfs_streq(name, "arb_value"))
    reg_addr = dci_obj.reg_addr;
    else if (get_static_reg_addr(ro_regs, name, &reg_addr) &&
    get_static_reg_addr(rw_regs, name, &reg_addr))
    return -EINVAL;
    err = drci_rd_reg(dci_obj.usb_device, reg_addr, &val);
    if (err < 0)
    return err;
    return sysfs_emit(buf, "%04x\n", val);
    }
    static ssize_t value_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    u16 val;
    u16 reg_addr;
    const char *name = attr.attr.name;
    struct most_dci_obj *dci_obj = to_dci_obj(dev);
    struct usb_device *usb_dev = dci_obj.usb_device;
    int err;
    err = kstrtou16(buf, 16, &val);
    if (err)
    return err;
    if (sysfs_streq(name, "arb_address")) {
    dci_obj.reg_addr = val;
    return count;
    }
    if (sysfs_streq(name, "arb_value"))
    err = drci_wr_reg(usb_dev, dci_obj.reg_addr, val);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: sysfs_streq(name, _arg: "sync_ep")) -> else {
    else if (sysfs_streq(name, "sync_ep"))
    err = start_sync_ep(usb_dev, val);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !get_static_reg_addr(rw_regs, _arg: name, _arg: &reg_addr)) -> else {
    else if (!get_static_reg_addr(rw_regs, name, &reg_addr))
    err = drci_wr_reg(usb_dev, reg_addr, val);
    else
    return -EINVAL;
    if (err < 0)
    return err;
    return count;
    }
    static DEVICE_ATTR(ni_state, 0444, value_show, core::ptr::null_mut());
    static DEVICE_ATTR(packet_bandwidth, 0444, value_show, core::ptr::null_mut());
    static DEVICE_ATTR(node_address, 0444, value_show, core::ptr::null_mut());
    static DEVICE_ATTR(node_position, 0444, value_show, core::ptr::null_mut());
    static DEVICE_ATTR(sync_ep, 0200, core::ptr::null_mut(), value_store);
    static DEVICE_ATTR(mep_filter, 0644, value_show, value_store);
    static DEVICE_ATTR(mep_hash0, 0644, value_show, value_store);
    static DEVICE_ATTR(mep_hash1, 0644, value_show, value_store);
    static DEVICE_ATTR(mep_hash2, 0644, value_show, value_store);
    static DEVICE_ATTR(mep_hash3, 0644, value_show, value_store);
    static DEVICE_ATTR(mep_eui48_hi, 0644, value_show, value_store);
    static DEVICE_ATTR(mep_eui48_mi, 0644, value_show, value_store);
    static DEVICE_ATTR(mep_eui48_lo, 0644, value_show, value_store);
    static DEVICE_ATTR(arb_address, 0644, value_show, value_store);
    static DEVICE_ATTR(arb_value, 0644, value_show, value_store);
    static struct attribute *dci_attrs[] = {
    &dev_attr_ni_state.attr,
    &dev_attr_packet_bandwidth.attr,
    &dev_attr_node_address.attr,
    &dev_attr_node_position.attr,
    &dev_attr_sync_ep.attr,
    &dev_attr_mep_filter.attr,
    &dev_attr_mep_hash0.attr,
    &dev_attr_mep_hash1.attr,
    &dev_attr_mep_hash2.attr,
    &dev_attr_mep_hash3.attr,
    &dev_attr_mep_eui48_hi.attr,
    &dev_attr_mep_eui48_mi.attr,
    &dev_attr_mep_eui48_lo.attr,
    &dev_attr_arb_address.attr,
    &dev_attr_arb_value.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(dci);
#[no_mangle]
unsafe extern "C" fn release_dci(dev: *mut device) {
    static void release_dci(struct device *dev)
    {
    struct most_dci_obj *dci = to_dci_obj(dev);
    put_device(dev.parent);
    kfree(dci);
    }
#[no_mangle]
unsafe extern "C" fn release_mdev(dev: *mut device) {
    static void release_mdev(struct device *dev)
    {
    struct most_dev *mdev = to_mdev_from_dev(dev);
    kfree(mdev.busy_urbs);
    kfree(mdev.cap);
    kfree(mdev.conf);
    kfree(mdev.ep_address);
    kfree(mdev);
    }
//
// hdm_probe - probe function of USB device driver
// @interface: Interface of the attached USB device
// @id: Pointer to the USB ID table.
//
// This allocates and initializes the device instance, adds the new
// entry to the internal list, scans the USB descriptors and registers
// the interface with the core.
// Additionally, the DCI objects are created and the hardware is sync'd.
//
// Return 0 on success. In case of an error a negative number is returned.
//
    static int
    hdm_probe(struct usb_interface *interface, const struct usb_device_id *id)
    {
    struct usb_host_interface *usb_iface_desc = interface.cur_altsetting;
    struct usb_device *usb_dev = interface_to_usbdev(interface);
    struct device *dev = &usb_dev.dev;
    struct most_dev *mdev;
    unsigned int i;
    unsigned int num_endpoints;
    struct most_channel_capability *tmp_cap;
    struct usb_endpoint_descriptor *ep_desc;
    let mut ret: c_int = -ENOMEM;
    mdev = kzalloc_obj(*mdev);
    if (!mdev)
    return -ENOMEM;
    usb_set_intfdata(interface, mdev);
    num_endpoints = usb_iface_desc.desc.bNumEndpoints;
    if (num_endpoints > MAX_NUM_ENDPOINTS) {
    kfree(mdev);
    return -EINVAL;
    }
    mutex_init(&mdev.io_mutex);
    INIT_WORK(&mdev.poll_work_obj, wq_netinfo);
    timer_setup(&mdev.link_stat_timer, link_stat_timer_handler, 0);
    mdev.usb_device = usb_dev;
    mdev.link_stat_timer.expires = jiffies + (2 * HZ);
    mdev.iface.mod = hdm_usb_fops.owner;
    mdev.iface.dev = &mdev.dev;
    mdev.iface.driver_dev = &interface.dev;
    mdev.iface.interface = ITYPE_USB;
    mdev.iface.configure = hdm_configure_channel;
    mdev.iface.request_netinfo = hdm_request_netinfo;
    mdev.iface.enqueue = hdm_enqueue;
    mdev.iface.poison_channel = hdm_poison_channel;
    mdev.iface.dma_alloc = hdm_dma_alloc;
    mdev.iface.dma_free = hdm_dma_free;
    mdev.iface.description = mdev.description;
    mdev.iface.num_channels = num_endpoints;
    snprintf(mdev.description, sizeof(mdev.description),
    "%d-%s:%d.%d",
    usb_dev.bus.busnum,
    usb_dev.devpath,
    usb_dev.config.desc.bConfigurationValue,
    usb_iface_desc.desc.bInterfaceNumber);
    mdev.dev.init_name = mdev.description;
    mdev.dev.parent = &interface.dev;
    mdev.dev.release = release_mdev;
    mdev.conf = kzalloc_objs(*mdev.conf, num_endpoints);
    if (!mdev.conf)
    goto err_free_mdev;
    mdev.cap = kzalloc_objs(*mdev.cap, num_endpoints);
    if (!mdev.cap)
    goto err_free_conf;
    mdev.iface.channel_vector = mdev.cap;
    mdev.ep_address = kzalloc_objs(*mdev.ep_address, num_endpoints);
    if (!mdev.ep_address)
    goto err_free_cap;
    mdev.busy_urbs = kzalloc_objs(*mdev.busy_urbs, num_endpoints);
    if (!mdev.busy_urbs)
    goto err_free_ep_address;
    tmp_cap = mdev.cap;
    for (i = 0; i < num_endpoints; i++) {
    ep_desc = &usb_iface_desc.endpoint[i].desc;
    mdev.ep_address[i] = ep_desc.bEndpointAddress;
    mdev.padding_active[i] = false;
    mdev.is_channel_healthy[i] = true;
    snprintf(&mdev.suffix[i][0], MAX_SUFFIX_LEN, "ep%02x",
    mdev.ep_address[i]);
    tmp_cap.name_suffix = &mdev.suffix[i][0];
    tmp_cap.buffer_size_packet = MAX_BUF_SIZE;
    tmp_cap.buffer_size_streaming = MAX_BUF_SIZE;
    tmp_cap.num_buffers_packet = BUF_CHAIN_SIZE;
    tmp_cap.num_buffers_streaming = BUF_CHAIN_SIZE;
    tmp_cap.data_type = MOST_CH_CONTROL | MOST_CH_ASYNC |
    MOST_CH_ISOC | MOST_CH_SYNC;
    if (usb_endpoint_dir_in(ep_desc))
    tmp_cap.direction = MOST_CH_RX;
    else
    tmp_cap.direction = MOST_CH_TX;
    tmp_cap++;
    init_usb_anchor(&mdev.busy_urbs[i]);
    spin_lock_init(&mdev.channel_lock[i]);
    }
    dev_dbg(dev, "claimed gadget: Vendor=%4.4x ProdID=%4.4x Bus=%02x Device=%02x\n",
    le16_to_cpu(usb_dev.descriptor.idVendor),
    le16_to_cpu(usb_dev.descriptor.idProduct),
    usb_dev.bus.busnum,
    usb_dev.devnum);
    dev_dbg(dev, "device path: /sys/bus/usb/devices/%d-%s:%d.%d\n",
    usb_dev.bus.busnum,
    usb_dev.devpath,
    usb_dev.config.desc.bConfigurationValue,
    usb_iface_desc.desc.bInterfaceNumber);
    ret = most_register_interface(&mdev.iface);
    if (ret)
    return ret;
    mutex_lock(&mdev.io_mutex);
    if (le16_to_cpu(usb_dev.descriptor.idProduct) == USB_DEV_ID_OS81118 ||
    le16_to_cpu(usb_dev.descriptor.idProduct) == USB_DEV_ID_OS81119 ||
    le16_to_cpu(usb_dev.descriptor.idProduct) == USB_DEV_ID_OS81210) {
    mdev.dci = kzalloc_obj(*mdev.dci);
    if (!mdev.dci) {
    mutex_unlock(&mdev.io_mutex);
    most_deregister_interface(&mdev.iface);
    return -ENOMEM;
    }
    mdev.dci.dev.init_name = "dci";
    mdev.dci.dev.parent = get_device(mdev.iface.dev);
    mdev.dci.dev.groups = dci_groups;
    mdev.dci.dev.release = release_dci;
    if (device_register(&mdev.dci.dev)) {
    mutex_unlock(&mdev.io_mutex);
    put_device(&mdev.dci.dev);
    most_deregister_interface(&mdev.iface);
    return -ENOMEM;
    }
    mdev.dci.usb_device = mdev.usb_device;
    }
    mutex_unlock(&mdev.io_mutex);
    return 0;
    err_free_ep_address:
    kfree(mdev.ep_address);
    err_free_cap:
    kfree(mdev.cap);
    err_free_conf:
    kfree(mdev.conf);
    err_free_mdev:
    kfree(mdev);
    return ret;
    }
//
// hdm_disconnect - disconnect function of USB device driver
// @interface: Interface of the attached USB device
//
// This deregisters the interface with the core, removes the kernel timer
// and frees resources.
//
// Context: hub kernel thread
//
#[no_mangle]
unsafe extern "C" fn hdm_disconnect(interface: *mut usb_interface) {
    static void hdm_disconnect(struct usb_interface *interface)
    {
    struct most_dev *mdev = usb_get_intfdata(interface);
    mutex_lock(&mdev.io_mutex);
    usb_set_intfdata(interface, core::ptr::null_mut());
    mdev.usb_device = core::ptr::null_mut();
    mutex_unlock(&mdev.io_mutex);
    timer_delete_sync(&mdev.link_stat_timer);
    cancel_work_sync(&mdev.poll_work_obj);
    if (mdev.dci)
    device_unregister(&mdev.dci.dev);
    most_deregister_interface(&mdev.iface);
    }
#[no_mangle]
unsafe extern "C" fn hdm_suspend(interface: *mut usb_interface, message: pm_message_t) -> c_int {
    static int hdm_suspend(struct usb_interface *interface, pm_message_t message)
    {
    struct most_dev *mdev = usb_get_intfdata(interface);
    int i;
    mutex_lock(&mdev.io_mutex);
    for (i = 0; i < mdev.iface.num_channels; i++) {
    most_stop_enqueue(&mdev.iface, i);
    usb_kill_anchored_urbs(&mdev.busy_urbs[i]);
    }
    mutex_unlock(&mdev.io_mutex);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hdm_resume(interface: *mut usb_interface) -> c_int {
    static int hdm_resume(struct usb_interface *interface)
    {
    struct most_dev *mdev = usb_get_intfdata(interface);
    int i;
    mutex_lock(&mdev.io_mutex);
    for (i = 0; i < mdev.iface.num_channels; i++)
    most_resume_enqueue(&mdev.iface, i);
    mutex_unlock(&mdev.io_mutex);
    return 0;
    }
    static struct usb_driver hdm_usb = {
    .name = "hdm_usb",
    .id_table = usbid,
    .probe = hdm_probe,
    .disconnect = hdm_disconnect,
    .resume = hdm_resume,
    .suspend = hdm_suspend,
    };
    module_usb_driver(hdm_usb);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Christian Gromm <christian.gromm@microchip.com>");
    MODULE_DESCRIPTION("HDM_4_USB");
