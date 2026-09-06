//! Automatically rewritten from C to Rust
//! Source: drivers/usb/gadget/udc/max3420_udc.c
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
// MAX3420 Device Controller driver for USB.
//
// Author: Jaswinder Singh Brar <jaswinder.singh@linaro.org>
// (C) Copyright 2019-2020 Linaro Ltd
//
// Based on:
// o MAX3420E datasheet
// https://datasheets.maximintegrated.com/en/ds/MAX3420E.pdf
// o MAX342{0,1}E Programming Guides
// https://pdfserv.maximintegrated.com/en/an/AN3598.pdf
// https://pdfserv.maximintegrated.com/en/an/AN3785.pdf
//

pub const MAX3420_MAX_EPS: c_int = 4;

// SPI commands:
pub const MAX3420_SPI_DIR_SHIFT: c_int = 1;
pub const MAX3420_SPI_REG_SHIFT: c_int = 3;
pub const MAX3420_REG_EP0FIFO: c_int = 0;
pub const MAX3420_REG_EP1FIFO: c_int = 1;
pub const MAX3420_REG_EP2FIFO: c_int = 2;
pub const MAX3420_REG_EP3FIFO: c_int = 3;
pub const MAX3420_REG_SUDFIFO: c_int = 4;
pub const MAX3420_REG_EP0BC: c_int = 5;
pub const MAX3420_REG_EP1BC: c_int = 6;
pub const MAX3420_REG_EP2BC: c_int = 7;
pub const MAX3420_REG_EP3BC: c_int = 8;
pub const MAX3420_REG_EPSTALLS: c_int = 9;

pub const MAX3420_REG_CLRTOGS: c_int = 10;

pub const MAX3420_REG_EPIRQ: c_int = 11;
pub const MAX3420_REG_EPIEN: c_int = 12;

pub const MAX3420_REG_USBIRQ: c_int = 13;
pub const MAX3420_REG_USBIEN: c_int = 14;

pub const MAX3420_REG_USBCTL: c_int = 15;

pub const MAX3420_REG_CPUCTL: c_int = 16;

pub const MAX3420_REG_PINCTL: c_int = 17;

pub const MAX3420_REG_REVISION: c_int = 18;
pub const MAX3420_REG_FNADDR: c_int = 19;
pub const FNADDR_MASK: c_uint = 0x7f;
pub const MAX3420_REG_IOPINS: c_int = 20;
pub const MAX3420_REG_IOPINS2: c_int = 21;
pub const MAX3420_REG_GPINIRQ: c_int = 22;
pub const MAX3420_REG_GPINIEN: c_int = 23;
pub const MAX3420_REG_GPINPOL: c_int = 24;
pub const MAX3420_REG_HIRQ: c_int = 25;
pub const MAX3420_REG_HIEN: c_int = 26;
pub const MAX3420_REG_MODE: c_int = 27;
pub const MAX3420_REG_PERADDR: c_int = 28;
pub const MAX3420_REG_HCTL: c_int = 29;
pub const MAX3420_REG_HXFR: c_int = 30;
pub const MAX3420_REG_HRSL: c_int = 31;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct max3420_req {
    pub usb_req: usb_request,
    pub queue: list_head,
    pub ep: *mut max3420_ep,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct max3420_ep {
    pub ep_usb: usb_ep,
    pub udc: *mut max3420_udc,
    pub queue: list_head,
    pub name: [c_char; MAX3420_EPNAME_SIZE],
    pub maxpacket: c_uint,
    pub lock: spinlock_t,
    pub halted: c_int,
    pub todo: u32,
    pub id: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct max3420_udc {
    pub gadget: usb_gadget,
    pub ep: [max3420_ep; MAX3420_MAX_EPS],
    pub driver: *mut usb_gadget_driver,
    pub thread_task: *mut task_struct,
    pub is_selfpowered: int remote_wkp,,
    pub softconnect: bool vbus_active,,
    pub setup: usb_ctrlrequest,
    pub spi_bus_mutex: mutex,
    pub ep0req: max3420_req,
    pub spi: *mut spi_device,
    pub dev: *mut device,
    pub lock: spinlock_t,
    pub suspended: bool,
    pub ep0buf: [u8; 64],
    pub todo: u32,
}

    static const char driver_name[] = "max3420-udc";
// Control endpoint configuration.
    static const struct usb_endpoint_descriptor ep0_desc = {
    .bEndpointAddress	= USB_DIR_OUT,
    .bmAttributes		= USB_ENDPOINT_XFER_CONTROL,
    .wMaxPacketSize		= cpu_to_le16(MAX3420_EP_MAX_PACKET),
    };
#[no_mangle]
unsafe extern "C" fn spi_ack_ctrl(udc: *mut max3420_udc) {
    static void spi_ack_ctrl(struct max3420_udc *udc)
    {
    struct spi_device *spi = udc.spi;
    struct spi_transfer transfer;
    struct spi_message msg;
    u8 txdata[1];
    memset(&transfer, 0, sizeof(transfer));
    spi_message_init(&msg);
    txdata[0] = MAX3420_ACKSTAT;
    transfer.tx_buf = txdata;
    transfer.len = 1;
    spi_message_add_tail(&transfer, &msg);
    spi_sync(spi, &msg);
    }
#[no_mangle]
unsafe extern "C" fn spi_rd8_ack(udc: *mut max3420_udc, reg: u8, actstat: c_int) -> u8 {
    static u8 spi_rd8_ack(struct max3420_udc *udc, u8 reg, int actstat)
    {
    struct spi_device *spi = udc.spi;
    struct spi_transfer transfer;
    struct spi_message msg;
    u8 txdata[2], rxdata[2];
    memset(&transfer, 0, sizeof(transfer));
    spi_message_init(&msg);
    txdata[0] = MAX3420_SPI_CMD_RD(reg) | (actstat ? MAX3420_ACKSTAT : 0);
    transfer.tx_buf = txdata;
    transfer.rx_buf = rxdata;
    transfer.len = 2;
    spi_message_add_tail(&transfer, &msg);
    spi_sync(spi, &msg);
    return rxdata[1];
    }
#[no_mangle]
unsafe extern "C" fn spi_rd8(udc: *mut max3420_udc, reg: u8) -> u8 {
    static u8 spi_rd8(struct max3420_udc *udc, u8 reg)
    {
    return spi_rd8_ack(udc, reg, 0);
    }
#[no_mangle]
unsafe extern "C" fn spi_wr8_ack(udc: *mut max3420_udc, reg: u8, val: u8, actstat: c_int) {
    static void spi_wr8_ack(struct max3420_udc *udc, u8 reg, u8 val, int actstat)
    {
    struct spi_device *spi = udc.spi;
    struct spi_transfer transfer;
    struct spi_message msg;
    u8 txdata[2];
    memset(&transfer, 0, sizeof(transfer));
    spi_message_init(&msg);
    txdata[0] = MAX3420_SPI_CMD_WR(reg) | (actstat ? MAX3420_ACKSTAT : 0);
    txdata[1] = val;
    transfer.tx_buf = txdata;
    transfer.len = 2;
    spi_message_add_tail(&transfer, &msg);
    spi_sync(spi, &msg);
    }
#[no_mangle]
unsafe extern "C" fn spi_wr8(udc: *mut max3420_udc, reg: u8, val: u8) {
    static void spi_wr8(struct max3420_udc *udc, u8 reg, u8 val)
    {
    spi_wr8_ack(udc, reg, val, 0);
    }
#[no_mangle]
unsafe extern "C" fn spi_rd_buf(udc: *mut max3420_udc, reg: u8, buf: *mut c_void, len: u8) {
    static void spi_rd_buf(struct max3420_udc *udc, u8 reg, void *buf, u8 len)
    {
    struct spi_device *spi = udc.spi;
    struct spi_transfer transfer;
    struct spi_message msg;
    u8 local_buf[MAX3420_EP_MAX_PACKET + 1] = {};
    memset(&transfer, 0, sizeof(transfer));
    spi_message_init(&msg);
    local_buf[0] = MAX3420_SPI_CMD_RD(reg);
    transfer.tx_buf = &local_buf[0];
    transfer.rx_buf = &local_buf[0];
    transfer.len = len + 1;
    spi_message_add_tail(&transfer, &msg);
    spi_sync(spi, &msg);
    memcpy(buf, &local_buf[1], len);
    }
#[no_mangle]
unsafe extern "C" fn spi_wr_buf(udc: *mut max3420_udc, reg: u8, buf: *mut c_void, len: u8) {
    static void spi_wr_buf(struct max3420_udc *udc, u8 reg, void *buf, u8 len)
    {
    struct spi_device *spi = udc.spi;
    struct spi_transfer transfer;
    struct spi_message msg;
    u8 local_buf[MAX3420_EP_MAX_PACKET + 1] = {};
    memset(&transfer, 0, sizeof(transfer));
    spi_message_init(&msg);
    local_buf[0] = MAX3420_SPI_CMD_WR(reg);
    memcpy(&local_buf[1], buf, len);
    transfer.tx_buf = local_buf;
    transfer.len = len + 1;
    spi_message_add_tail(&transfer, &msg);
    spi_sync(spi, &msg);
    }
#[no_mangle]
unsafe extern "C" fn spi_max3420_enable(ep: *mut max3420_ep) -> c_int {
    static int spi_max3420_enable(struct max3420_ep *ep)
    {
    struct max3420_udc *udc = ep.udc;
    unsigned long flags;
    u8 epdis, epien;
    int todo;
    spin_lock_irqsave(&ep.lock, flags);
    todo = ep.todo & ENABLE_EP;
    ep.todo &= ~ENABLE_EP;
    spin_unlock_irqrestore(&ep.lock, flags);
    if (!todo || ep.id == 0)
    return false;
    epien = spi_rd8(udc, MAX3420_REG_EPIEN);
    epdis = spi_rd8(udc, MAX3420_REG_CLRTOGS);
    if (todo == ENABLE) {
    epdis &= ~BIT(ep.id + 4);
    epien |= BIT(ep.id + 1);
    } else {
    epdis |= BIT(ep.id + 4);
    epien &= ~BIT(ep.id + 1);
    }
    spi_wr8(udc, MAX3420_REG_CLRTOGS, epdis);
    spi_wr8(udc, MAX3420_REG_EPIEN, epien);
    return true;
    }
#[no_mangle]
unsafe extern "C" fn spi_max3420_stall(ep: *mut max3420_ep) -> c_int {
    static int spi_max3420_stall(struct max3420_ep *ep)
    {
    struct max3420_udc *udc = ep.udc;
    unsigned long flags;
    u8 epstalls;
    int todo;
    spin_lock_irqsave(&ep.lock, flags);
    todo = ep.todo & STALL_EP;
    ep.todo &= ~STALL_EP;
    spin_unlock_irqrestore(&ep.lock, flags);
    if (!todo || ep.id == 0)
    return false;
    epstalls = spi_rd8(udc, MAX3420_REG_EPSTALLS);
    if (todo == STALL) {
    ep.halted = 1;
    epstalls |= BIT(ep.id + 1);
    } else {
    u8 clrtogs;
    ep.halted = 0;
    epstalls &= ~BIT(ep.id + 1);
    clrtogs = spi_rd8(udc, MAX3420_REG_CLRTOGS);
    clrtogs |= BIT(ep.id + 1);
    spi_wr8(udc, MAX3420_REG_CLRTOGS, clrtogs);
    }
    spi_wr8(udc, MAX3420_REG_EPSTALLS, epstalls | ACKSTAT);
    return true;
    }
#[no_mangle]
unsafe extern "C" fn spi_max3420_rwkup(udc: *mut max3420_udc) -> c_int {
    static int spi_max3420_rwkup(struct max3420_udc *udc)
    {
    unsigned long flags;
    int wake_remote;
    u8 usbctl;
    spin_lock_irqsave(&udc.lock, flags);
    wake_remote = udc.todo & REMOTE_WAKEUP;
    udc.todo &= ~REMOTE_WAKEUP;
    spin_unlock_irqrestore(&udc.lock, flags);
    if (!wake_remote || !udc.suspended)
    return false;
// Set Remote-WkUp Signal
    usbctl = spi_rd8(udc, MAX3420_REG_USBCTL);
    usbctl |= SIGRWU;
    spi_wr8(udc, MAX3420_REG_USBCTL, usbctl);
    msleep_interruptible(5);
// Clear Remote-WkUp Signal
    usbctl = spi_rd8(udc, MAX3420_REG_USBCTL);
    usbctl &= ~SIGRWU;
    spi_wr8(udc, MAX3420_REG_USBCTL, usbctl);
    udc.suspended = false;
    return true;
    }
    static void max3420_nuke(struct max3420_ep *ep, int status);
#[no_mangle]
unsafe extern "C" fn __max3420_stop(udc: *mut max3420_udc) {
    static void __max3420_stop(struct max3420_udc *udc)
    {
    u8 val;
    int i;
// clear all pending requests
    for (i = 1; i < MAX3420_MAX_EPS; i++)
    max3420_nuke(&udc.ep[i], -ECONNRESET);
// Disable IRQ to CPU
    spi_wr8(udc, MAX3420_REG_CPUCTL, 0);
    val = spi_rd8(udc, MAX3420_REG_USBCTL);
    val |= PWRDOWN;
    if (udc.is_selfpowered)
    val &= ~HOSCSTEN;
    else
    val |= HOSCSTEN;
    spi_wr8(udc, MAX3420_REG_USBCTL, val);
    }
#[no_mangle]
unsafe extern "C" fn __max3420_start(udc: *mut max3420_udc) {
    static void __max3420_start(struct max3420_udc *udc)
    {
    u8 val;
// Need this delay if bus-powered,
// but even for self-powered it helps stability
//
    msleep_interruptible(250);
// configure SPI
    spi_wr8(udc, MAX3420_REG_PINCTL, FDUPSPI);
// Chip Reset
    spi_wr8(udc, MAX3420_REG_USBCTL, CHIPRES);
    msleep_interruptible(5);
    spi_wr8(udc, MAX3420_REG_USBCTL, 0);
// Poll for OSC to stabilize
    while (1) {
    val = spi_rd8(udc, MAX3420_REG_USBIRQ);
    if (val & OSCOKIRQ)
    break;
    cond_resched();
    }
// Enable PULL-UP only when Vbus detected
    val = spi_rd8(udc, MAX3420_REG_USBCTL);
    val |= VBGATE | CONNECT;
    spi_wr8(udc, MAX3420_REG_USBCTL, val);
    val = URESDNIRQ | URESIRQ;
    if (udc.is_selfpowered)
    val |= NOVBUSIRQ;
    spi_wr8(udc, MAX3420_REG_USBIEN, val);
// Enable only EP0 interrupts
    val = IN0BAVIRQ | OUT0DAVIRQ | SUDAVIRQ;
    spi_wr8(udc, MAX3420_REG_EPIEN, val);
// Enable IRQ to CPU
    spi_wr8(udc, MAX3420_REG_CPUCTL, IE);
    }
#[no_mangle]
unsafe extern "C" fn max3420_start(udc: *mut max3420_udc) -> c_int {
    static int max3420_start(struct max3420_udc *udc)
    {
    unsigned long flags;
    int todo;
    spin_lock_irqsave(&udc.lock, flags);
    todo = udc.todo & UDC_START;
    udc.todo &= ~UDC_START;
    spin_unlock_irqrestore(&udc.lock, flags);
    if (!todo)
    return false;
    if (udc.vbus_active && udc.softconnect)
    __max3420_start(udc);
    else
    __max3420_stop(udc);
    return true;
    }
#[no_mangle]
unsafe extern "C" fn max3420_vbus_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t max3420_vbus_handler(int irq, void *dev_id)
    {
    struct max3420_udc *udc = dev_id;
    unsigned long flags;
    spin_lock_irqsave(&udc.lock, flags);
// its a vbus change interrupt
    udc.vbus_active = !udc.vbus_active;
    udc.todo |= UDC_START;
    usb_udc_vbus_handler(&udc.gadget, udc.vbus_active);
    usb_gadget_set_state(&udc.gadget, udc.vbus_active
    ? USB_STATE_POWERED : USB_STATE_NOTATTACHED);
    spin_unlock_irqrestore(&udc.lock, flags);
    if (udc.thread_task)
    wake_up_process(udc.thread_task);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn max3420_irq_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t max3420_irq_handler(int irq, void *dev_id)
    {
    struct max3420_udc *udc = dev_id;
    struct spi_device *spi = udc.spi;
    unsigned long flags;
    spin_lock_irqsave(&udc.lock, flags);
    if ((udc.todo & ENABLE_IRQ) == 0) {
    disable_irq_nosync(spi.irq);
    udc.todo |= ENABLE_IRQ;
    }
    spin_unlock_irqrestore(&udc.lock, flags);
    if (udc.thread_task)
    wake_up_process(udc.thread_task);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn max3420_getstatus(udc: *mut max3420_udc) {
    static void max3420_getstatus(struct max3420_udc *udc)
    {
    struct max3420_ep *ep;
    let mut status: u16 = 0;
    switch (udc.setup.bRequestType & USB_RECIP_MASK) {
    case USB_RECIP_DEVICE:
// Get device status
    status = udc.gadget.is_selfpowered << USB_DEVICE_SELF_POWERED;
    status |= (udc.remote_wkp << USB_DEVICE_REMOTE_WAKEUP);
    break;
    case USB_RECIP_INTERFACE:
    if (udc.driver.setup(&udc.gadget, &udc.setup) < 0)
    goto stall;
    break;
    case USB_RECIP_ENDPOINT:
    ep = &udc.ep[udc.setup.wIndex & USB_ENDPOINT_NUMBER_MASK];
    if (udc.setup.wIndex & USB_DIR_IN) {
    if (!ep.ep_usb.caps.dir_in)
    goto stall;
    } else {
    if (!ep.ep_usb.caps.dir_out)
    goto stall;
    }
    if (ep.halted)
    status = 1 << USB_ENDPOINT_HALT;
    break;
    default:
    goto stall;
    }
    status = cpu_to_le16(status);
    spi_wr_buf(udc, MAX3420_REG_EP0FIFO, &status, 2);
    spi_wr8_ack(udc, MAX3420_REG_EP0BC, 2, 1);
    return;
    stall:
    dev_err(udc.dev, "Can't respond to getstatus request\n");
    spi_wr8(udc, MAX3420_REG_EPSTALLS, STLEP0IN | STLEP0OUT | STLSTAT);
    }
#[no_mangle]
unsafe extern "C" fn max3420_set_clear_feature(udc: *mut max3420_udc) {
    static void max3420_set_clear_feature(struct max3420_udc *udc)
    {
    struct max3420_ep *ep;
    let mut set: c_int = udc.setup.bRequest == USB_REQ_SET_FEATURE;
    unsigned long flags;
    int id;
    switch (udc.setup.bRequestType) {
    case USB_RECIP_DEVICE:
    if (udc.setup.wValue != USB_DEVICE_REMOTE_WAKEUP)
    break;
    if (udc.setup.bRequest == USB_REQ_SET_FEATURE)
    udc.remote_wkp = 1;
    else
    udc.remote_wkp = 0;
    return spi_ack_ctrl(udc);
    case USB_RECIP_ENDPOINT:
    if (udc.setup.wValue != USB_ENDPOINT_HALT)
    break;
    id = udc.setup.wIndex & USB_ENDPOINT_NUMBER_MASK;
    ep = &udc.ep[id];
    spin_lock_irqsave(&ep.lock, flags);
    ep.todo &= ~STALL_EP;
    if (set)
    ep.todo |= STALL;
    else
    ep.todo |= UNSTALL;
    spin_unlock_irqrestore(&ep.lock, flags);
    spi_max3420_stall(ep);
    return;
    default:
    break;
    }
    dev_err(udc.dev, "Can't respond to SET/CLEAR FEATURE\n");
    spi_wr8(udc, MAX3420_REG_EPSTALLS, STLEP0IN | STLEP0OUT | STLSTAT);
    }
#[no_mangle]
unsafe extern "C" fn max3420_handle_setup(udc: *mut max3420_udc) {
    static void max3420_handle_setup(struct max3420_udc *udc)
    {
    struct usb_ctrlrequest setup;
    spi_rd_buf(udc, MAX3420_REG_SUDFIFO, (void *)&setup, 8);
    udc.setup = setup;
    udc.setup.wValue = cpu_to_le16(setup.wValue);
    udc.setup.wIndex = cpu_to_le16(setup.wIndex);
    udc.setup.wLength = cpu_to_le16(setup.wLength);
    switch (udc.setup.bRequest) {
    case USB_REQ_GET_STATUS:
// Data+Status phase form udc
    if ((udc.setup.bRequestType &
    (USB_DIR_IN | USB_TYPE_MASK)) !=
    (USB_DIR_IN | USB_TYPE_STANDARD)) {
    break;
    }
    return max3420_getstatus(udc);
    case USB_REQ_SET_ADDRESS:
// Status phase from udc
    if (udc.setup.bRequestType != (USB_DIR_OUT |
    USB_TYPE_STANDARD | USB_RECIP_DEVICE)) {
    break;
    }
    spi_rd8_ack(udc, MAX3420_REG_FNADDR, 1);
    dev_dbg(udc.dev, "Assigned Address=%d\n", udc.setup.wValue);
    return;
    case USB_REQ_CLEAR_FEATURE:
    case USB_REQ_SET_FEATURE:
// Requests with no data phase, status phase from udc
    if ((udc.setup.bRequestType & USB_TYPE_MASK)
    != USB_TYPE_STANDARD)
    break;
    return max3420_set_clear_feature(udc);
    default:
    break;
    }
    if (udc.driver.setup(&udc.gadget, &setup) < 0) {
// Stall EP0
    spi_wr8(udc, MAX3420_REG_EPSTALLS,
    STLEP0IN | STLEP0OUT | STLSTAT);
    }
    }
#[no_mangle]
unsafe extern "C" fn max3420_req_done(req: *mut max3420_req, status: c_int) {
    static void max3420_req_done(struct max3420_req *req, int status)
    {
    struct max3420_ep *ep = req.ep;
    struct max3420_udc *udc = ep.udc;
    if (req.usb_req.status == -EINPROGRESS)
    req.usb_req.status = status;
    else
    status = req.usb_req.status;
    if (status && status != -ESHUTDOWN)
    dev_err(udc.dev, "%s done %p, status %d\n",
    ep.ep_usb.name, req, status);
    if (req.usb_req.complete)
    req.usb_req.complete(&ep.ep_usb, &req.usb_req);
    }
#[no_mangle]
unsafe extern "C" fn max3420_do_data(udc: *mut max3420_udc, ep_id: c_int, in: c_int) -> c_int {
    static int max3420_do_data(struct max3420_udc *udc, int ep_id, int in)
    {
    struct max3420_ep *ep = &udc.ep[ep_id];
    struct max3420_req *req;
    int done, length, psz;
    void *buf;
    if (list_empty(&ep.queue))
    return false;
    req = list_first_entry(&ep.queue, struct max3420_req, queue);
    buf = req.usb_req.buf + req.usb_req.actual;
    psz = ep.ep_usb.maxpacket;
    length = req.usb_req.length - req.usb_req.actual;
    length = min(length, psz);
    if (length == 0) {
    done = 1;
    goto xfer_done;
    }
    done = 0;
    if (in) {
    prefetch(buf);
    spi_wr_buf(udc, MAX3420_REG_EP0FIFO + ep_id, buf, length);
    spi_wr8(udc, MAX3420_REG_EP0BC + ep_id, length);
    if (length < psz)
    done = 1;
    } else {
    psz = spi_rd8(udc, MAX3420_REG_EP0BC + ep_id);
    length = min(length, psz);
    prefetchw(buf);
    spi_rd_buf(udc, MAX3420_REG_EP0FIFO + ep_id, buf, length);
    if (length < ep.ep_usb.maxpacket)
    done = 1;
    }
    req.usb_req.actual += length;
    if (req.usb_req.actual == req.usb_req.length)
    done = 1;
    xfer_done:
    if (done) {
    unsigned long flags;
    spin_lock_irqsave(&ep.lock, flags);
    list_del_init(&req.queue);
    spin_unlock_irqrestore(&ep.lock, flags);
    if (ep_id == 0)
    spi_ack_ctrl(udc);
    max3420_req_done(req, 0);
    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn max3420_handle_irqs(udc: *mut max3420_udc) -> c_int {
    static int max3420_handle_irqs(struct max3420_udc *udc)
    {
    u8 epien, epirq, usbirq, usbien, reg[4];
    let mut ret: bool = false;
    spi_rd_buf(udc, MAX3420_REG_EPIRQ, reg, 4);
    epirq = reg[0];
    epien = reg[1];
    usbirq = reg[2];
    usbien = reg[3];
    usbirq &= usbien;
    epirq &= epien;
    if (epirq & SUDAVIRQ) {
    spi_wr8(udc, MAX3420_REG_EPIRQ, SUDAVIRQ);
    max3420_handle_setup(udc);
    return true;
    }
    if (usbirq & VBUSIRQ) {
    spi_wr8(udc, MAX3420_REG_USBIRQ, VBUSIRQ);
    dev_dbg(udc.dev, "Cable plugged in\n");
    return true;
    }
    if (usbirq & NOVBUSIRQ) {
    spi_wr8(udc, MAX3420_REG_USBIRQ, NOVBUSIRQ);
    dev_dbg(udc.dev, "Cable pulled out\n");
    return true;
    }
    if (usbirq & URESIRQ) {
    spi_wr8(udc, MAX3420_REG_USBIRQ, URESIRQ);
    dev_dbg(udc.dev, "USB Reset - Start\n");
    return true;
    }
    if (usbirq & URESDNIRQ) {
    spi_wr8(udc, MAX3420_REG_USBIRQ, URESDNIRQ);
    dev_dbg(udc.dev, "USB Reset - END\n");
    spi_wr8(udc, MAX3420_REG_USBIEN, URESDNIRQ | URESIRQ);
    spi_wr8(udc, MAX3420_REG_EPIEN, SUDAVIRQ | IN0BAVIRQ
    | OUT0DAVIRQ);
    return true;
    }
    if (usbirq & SUSPIRQ) {
    spi_wr8(udc, MAX3420_REG_USBIRQ, SUSPIRQ);
    dev_dbg(udc.dev, "USB Suspend - Enter\n");
    udc.suspended = true;
    return true;
    }
    if (usbirq & BUSACTIRQ) {
    spi_wr8(udc, MAX3420_REG_USBIRQ, BUSACTIRQ);
    dev_dbg(udc.dev, "USB Suspend - Exit\n");
    udc.suspended = false;
    return true;
    }
    if (usbirq & RWUDNIRQ) {
    spi_wr8(udc, MAX3420_REG_USBIRQ, RWUDNIRQ);
    dev_dbg(udc.dev, "Asked Host to wakeup\n");
    return true;
    }
    if (usbirq & OSCOKIRQ) {
    spi_wr8(udc, MAX3420_REG_USBIRQ, OSCOKIRQ);
    dev_dbg(udc.dev, "Osc stabilized, start work\n");
    return true;
    }
    if (epirq & OUT0DAVIRQ && max3420_do_data(udc, 0, 0)) {
    spi_wr8_ack(udc, MAX3420_REG_EPIRQ, OUT0DAVIRQ, 1);
    ret = true;
    }
    if (epirq & IN0BAVIRQ && max3420_do_data(udc, 0, 1))
    ret = true;
    if (epirq & OUT1DAVIRQ && max3420_do_data(udc, 1, 0)) {
    spi_wr8_ack(udc, MAX3420_REG_EPIRQ, OUT1DAVIRQ, 1);
    ret = true;
    }
    if (epirq & IN2BAVIRQ && max3420_do_data(udc, 2, 1))
    ret = true;
    if (epirq & IN3BAVIRQ && max3420_do_data(udc, 3, 1))
    ret = true;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn max3420_thread(dev_id: *mut c_void) -> c_int {
    static int max3420_thread(void *dev_id)
    {
    struct max3420_udc *udc = dev_id;
    struct spi_device *spi = udc.spi;
    int i, loop_again = 1;
    unsigned long flags;
    while (!kthread_should_stop()) {
    if (!loop_again) {
    ktime_t kt = ns_to_ktime(1000 * 1000 * 250); /* 250ms */
    set_current_state(TASK_INTERRUPTIBLE);
    spin_lock_irqsave(&udc.lock, flags);
    if (udc.todo & ENABLE_IRQ) {
    enable_irq(spi.irq);
    udc.todo &= ~ENABLE_IRQ;
    }
    spin_unlock_irqrestore(&udc.lock, flags);
    schedule_hrtimeout(&kt, HRTIMER_MODE_REL);
    }
    loop_again = 0;
    mutex_lock(&udc.spi_bus_mutex);
// If bus-vbus_active and disconnected
    if (!udc.vbus_active || !udc.softconnect)
    goto loop;
    if (max3420_start(udc)) {
    loop_again = 1;
    goto loop;
    }
    if (max3420_handle_irqs(udc)) {
    loop_again = 1;
    goto loop;
    }
    if (spi_max3420_rwkup(udc)) {
    loop_again = 1;
    goto loop;
    }
    max3420_do_data(udc, 0, 1); /* get done with the EP0 ZLP */
    for (i = 1; i < MAX3420_MAX_EPS; i++) {
    struct max3420_ep *ep = &udc.ep[i];
    if (spi_max3420_enable(ep))
    loop_again = 1;
    if (spi_max3420_stall(ep))
    loop_again = 1;
    }
    loop:
    mutex_unlock(&udc.spi_bus_mutex);
    }
    set_current_state(TASK_RUNNING);
    dev_info(udc.dev, "SPI thread exiting\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn max3420_ep_set_halt(_ep: *mut usb_ep, stall: c_int) -> c_int {
    static int max3420_ep_set_halt(struct usb_ep *_ep, int stall)
    {
    struct max3420_ep *ep = to_max3420_ep(_ep);
    struct max3420_udc *udc = ep.udc;
    unsigned long flags;
    spin_lock_irqsave(&ep.lock, flags);
    ep.todo &= ~STALL_EP;
    if (stall)
    ep.todo |= STALL;
    else
    ep.todo |= UNSTALL;
    spin_unlock_irqrestore(&ep.lock, flags);
    wake_up_process(udc.thread_task);
    dev_dbg(udc.dev, "%sStall %s\n", stall ? "" : "Un", ep.name);
    return 0;
    }
    static int __max3420_ep_enable(struct max3420_ep *ep,
    const struct usb_endpoint_descriptor *desc)
    {
    let mut maxp: c_uint = usb_endpoint_maxp(desc);
    unsigned long flags;
    spin_lock_irqsave(&ep.lock, flags);
    ep.ep_usb.desc = desc;
    ep.ep_usb.maxpacket = maxp;
    ep.todo &= ~ENABLE_EP;
    ep.todo |= ENABLE;
    spin_unlock_irqrestore(&ep.lock, flags);
    return 0;
    }
    static int max3420_ep_enable(struct usb_ep *_ep,
    const struct usb_endpoint_descriptor *desc)
    {
    struct max3420_ep *ep = to_max3420_ep(_ep);
    struct max3420_udc *udc = ep.udc;
    __max3420_ep_enable(ep, desc);
    wake_up_process(udc.thread_task);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn max3420_nuke(ep: *mut max3420_ep, status: c_int) {
    static void max3420_nuke(struct max3420_ep *ep, int status)
    {
    struct max3420_req *req, *r;
    unsigned long flags;
    spin_lock_irqsave(&ep.lock, flags);
    list_for_each_entry_safe(req, r, &ep.queue, queue) {
    list_del_init(&req.queue);
    spin_unlock_irqrestore(&ep.lock, flags);
    max3420_req_done(req, status);
    spin_lock_irqsave(&ep.lock, flags);
    }
    spin_unlock_irqrestore(&ep.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn __max3420_ep_disable(ep: *mut max3420_ep) {
    static void __max3420_ep_disable(struct max3420_ep *ep)
    {
    struct max3420_udc *udc = ep.udc;
    unsigned long flags;
    spin_lock_irqsave(&ep.lock, flags);
    ep.ep_usb.desc = core::ptr::null_mut();
    ep.todo &= ~ENABLE_EP;
    ep.todo |= DISABLE;
    spin_unlock_irqrestore(&ep.lock, flags);
    dev_dbg(udc.dev, "Disabled %s\n", ep.name);
    }
#[no_mangle]
unsafe extern "C" fn max3420_ep_disable(_ep: *mut usb_ep) -> c_int {
    static int max3420_ep_disable(struct usb_ep *_ep)
    {
    struct max3420_ep *ep = to_max3420_ep(_ep);
    struct max3420_udc *udc = ep.udc;
    max3420_nuke(ep, -ESHUTDOWN);
    __max3420_ep_disable(ep);
    wake_up_process(udc.thread_task);
    return 0;
    }
    static struct usb_request *max3420_alloc_request(struct usb_ep *_ep,
    gfp_t gfp_flags)
    {
    struct max3420_ep *ep = to_max3420_ep(_ep);
    struct max3420_req *req;
    req = kzalloc_obj(*req, gfp_flags);
    if (!req)
    return core::ptr::null_mut();
    req.ep = ep;
    return &req.usb_req;
    }
#[no_mangle]
unsafe extern "C" fn max3420_free_request(_ep: *mut usb_ep, _req: *mut usb_request) {
    static void max3420_free_request(struct usb_ep *_ep, struct usb_request *_req)
    {
    kfree(to_max3420_req(_req));
    }
    static int max3420_ep_queue(struct usb_ep *_ep, struct usb_request *_req,
    gfp_t ignored)
    {
    struct max3420_req *req = to_max3420_req(_req);
    struct max3420_ep *ep  = to_max3420_ep(_ep);
    struct max3420_udc *udc = ep.udc;
    unsigned long flags;
    _req.status = -EINPROGRESS;
    _req.actual = 0;
    spin_lock_irqsave(&ep.lock, flags);
    list_add_tail(&req.queue, &ep.queue);
    spin_unlock_irqrestore(&ep.lock, flags);
    wake_up_process(udc.thread_task);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn max3420_ep_dequeue(_ep: *mut usb_ep, _req: *mut usb_request) -> c_int {
    static int max3420_ep_dequeue(struct usb_ep *_ep, struct usb_request *_req)
    {
    struct max3420_req *t = core::ptr::null_mut();
    struct max3420_req *req = to_max3420_req(_req);
    struct max3420_req *iter;
    struct max3420_ep *ep = to_max3420_ep(_ep);
    unsigned long flags;
    spin_lock_irqsave(&ep.lock, flags);
// Pluck the descriptor from queue
    list_for_each_entry(iter, &ep.queue, queue) {
    if (iter != req)
    continue;
    list_del_init(&req.queue);
    t = iter;
    break;
    }
    spin_unlock_irqrestore(&ep.lock, flags);
    if (t)
    max3420_req_done(req, -ECONNRESET);
    return 0;
    }
    static const struct usb_ep_ops max3420_ep_ops = {
    .enable		= max3420_ep_enable,
    .disable	= max3420_ep_disable,
    .alloc_request	= max3420_alloc_request,
    .free_request	= max3420_free_request,
    .queue		= max3420_ep_queue,
    .dequeue	= max3420_ep_dequeue,
    .set_halt	= max3420_ep_set_halt,
    };
#[no_mangle]
unsafe extern "C" fn max3420_wakeup(gadget: *mut usb_gadget) -> c_int {
    static int max3420_wakeup(struct usb_gadget *gadget)
    {
    struct max3420_udc *udc = to_udc(gadget);
    unsigned long flags;
    let mut ret: c_int = -EINVAL;
    spin_lock_irqsave(&udc.lock, flags);
// Only if wakeup allowed by host
    if (udc.remote_wkp) {
    udc.todo |= REMOTE_WAKEUP;
    ret = 0;
    }
    spin_unlock_irqrestore(&udc.lock, flags);
    if (udc.thread_task)
    wake_up_process(udc.thread_task);
    return ret;
    }
    static int max3420_udc_start(struct usb_gadget *gadget,
    struct usb_gadget_driver *driver)
    {
    struct max3420_udc *udc = to_udc(gadget);
    unsigned long flags;
    spin_lock_irqsave(&udc.lock, flags);
// hook up the driver
    udc.driver = driver;
    udc.gadget.speed = USB_SPEED_FULL;
    udc.gadget.is_selfpowered = udc.is_selfpowered;
    udc.remote_wkp = 0;
    udc.softconnect = true;
    udc.todo |= UDC_START;
    spin_unlock_irqrestore(&udc.lock, flags);
    if (udc.thread_task)
    wake_up_process(udc.thread_task);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn max3420_udc_stop(gadget: *mut usb_gadget) -> c_int {
    static int max3420_udc_stop(struct usb_gadget *gadget)
    {
    struct max3420_udc *udc = to_udc(gadget);
    unsigned long flags;
    spin_lock_irqsave(&udc.lock, flags);
    udc.is_selfpowered = udc.gadget.is_selfpowered;
    udc.gadget.speed = USB_SPEED_UNKNOWN;
    udc.driver = core::ptr::null_mut();
    udc.softconnect = false;
    udc.todo |= UDC_START;
    spin_unlock_irqrestore(&udc.lock, flags);
    if (udc.thread_task)
    wake_up_process(udc.thread_task);
    return 0;
    }
    static const struct usb_gadget_ops max3420_udc_ops = {
    .udc_start	= max3420_udc_start,
    .udc_stop	= max3420_udc_stop,
    .wakeup		= max3420_wakeup,
    };
#[no_mangle]
unsafe extern "C" fn max3420_eps_init(udc: *mut max3420_udc) {
    static void max3420_eps_init(struct max3420_udc *udc)
    {
    int idx;
    INIT_LIST_HEAD(&udc.gadget.ep_list);
    for (idx = 0; idx < MAX3420_MAX_EPS; idx++) {
    struct max3420_ep *ep = &udc.ep[idx];
    spin_lock_init(&ep.lock);
    INIT_LIST_HEAD(&ep.queue);
    ep.udc = udc;
    ep.id = idx;
    ep.halted = 0;
    ep.maxpacket = 0;
    ep.ep_usb.name = ep.name;
    ep.ep_usb.ops = &max3420_ep_ops;
    usb_ep_set_maxpacket_limit(&ep.ep_usb, MAX3420_EP_MAX_PACKET);
    if (idx == 0) { /* For EP0 */
    ep.ep_usb.desc = &ep0_desc;
    ep.ep_usb.maxpacket = usb_endpoint_maxp(&ep0_desc);
    ep.ep_usb.caps.type_control = true;
    ep.ep_usb.caps.dir_in = true;
    ep.ep_usb.caps.dir_out = true;
    snprintf(ep.name, MAX3420_EPNAME_SIZE, "ep0");
    continue;
    }
    if (idx == 1) { /* EP1 is OUT */
    ep.ep_usb.caps.dir_in = false;
    ep.ep_usb.caps.dir_out = true;
    snprintf(ep.name, MAX3420_EPNAME_SIZE, "ep1-bulk-out");
    } else { /* EP2 & EP3 are IN */
    ep.ep_usb.caps.dir_in = true;
    ep.ep_usb.caps.dir_out = false;
    snprintf(ep.name, MAX3420_EPNAME_SIZE,
    "ep%d-bulk-in", idx);
    }
    ep.ep_usb.caps.type_iso = false;
    ep.ep_usb.caps.type_int = false;
    ep.ep_usb.caps.type_bulk = true;
    list_add_tail(&ep.ep_usb.ep_list,
    &udc.gadget.ep_list);
    }
    }
#[no_mangle]
unsafe extern "C" fn max3420_probe(spi: *mut spi_device) -> c_int {
    static int max3420_probe(struct spi_device *spi)
    {
    struct max3420_udc *udc;
    int err, irq;
    u8 reg[8];
    if (spi.controller.flags & SPI_CONTROLLER_HALF_DUPLEX) {
    dev_err(&spi.dev, "UDC needs full duplex to work\n");
    return -EINVAL;
    }
    spi.mode = SPI_MODE_3;
    spi.bits_per_word = 8;
    err = spi_setup(spi);
    if (err) {
    dev_err(&spi.dev, "Unable to setup SPI bus\n");
    return -EFAULT;
    }
    udc = devm_kzalloc(&spi.dev, sizeof(*udc), GFP_KERNEL);
    if (!udc)
    return -ENOMEM;
    udc.spi = spi;
    udc.remote_wkp = 0;
// Setup gadget structure
    udc.gadget.ops = &max3420_udc_ops;
    udc.gadget.max_speed = USB_SPEED_FULL;
    udc.gadget.speed = USB_SPEED_UNKNOWN;
    udc.gadget.ep0 = &udc.ep[0].ep_usb;
    udc.gadget.name = driver_name;
    spin_lock_init(&udc.lock);
    mutex_init(&udc.spi_bus_mutex);
    udc.ep0req.ep = &udc.ep[0];
    udc.ep0req.usb_req.buf = udc.ep0buf;
    INIT_LIST_HEAD(&udc.ep0req.queue);
// setup Endpoints
    max3420_eps_init(udc);
// configure SPI
    spi_rd_buf(udc, MAX3420_REG_EPIRQ, reg, 8);
    spi_wr8(udc, MAX3420_REG_PINCTL, FDUPSPI);
    err = usb_add_gadget_udc(&spi.dev, &udc.gadget);
    if (err)
    return err;
    udc.dev = &udc.gadget.dev;
    spi_set_drvdata(spi, udc);
    irq = of_irq_get_byname(spi.dev.of_node, "udc");
    err = devm_request_irq(&spi.dev, irq, max3420_irq_handler, 0,
    "max3420", udc);
    if (err < 0)
    goto del_gadget;
    udc.thread_task = kthread_create(max3420_thread, udc,
    "max3420-thread");
    if (IS_ERR(udc.thread_task)) {
    err = PTR_ERR(udc.thread_task);
    goto del_gadget;
    }
    irq = of_irq_get_byname(spi.dev.of_node, "vbus");
    if (irq <= 0) { /* no vbus irq implies self-powered design */
    udc.is_selfpowered = 1;
    udc.vbus_active = true;
    udc.todo |= UDC_START;
    usb_udc_vbus_handler(&udc.gadget, udc.vbus_active);
    usb_gadget_set_state(&udc.gadget, USB_STATE_POWERED);
    max3420_start(udc);
    } else {
    udc.is_selfpowered = 0;
// Detect current vbus status
    spi_rd_buf(udc, MAX3420_REG_EPIRQ, reg, 8);
    if (reg[7] != 0xff)
    udc.vbus_active = true;
    err = devm_request_irq(&spi.dev, irq,
    max3420_vbus_handler, 0, "vbus", udc);
    if (err < 0)
    goto del_gadget;
    }
    return 0;
    del_gadget:
    usb_del_gadget_udc(&udc.gadget);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn max3420_remove(spi: *mut spi_device) {
    static void max3420_remove(struct spi_device *spi)
    {
    struct max3420_udc *udc = spi_get_drvdata(spi);
    unsigned long flags;
    usb_del_gadget_udc(&udc.gadget);
    spin_lock_irqsave(&udc.lock, flags);
    kthread_stop(udc.thread_task);
    spin_unlock_irqrestore(&udc.lock, flags);
    }
    static const struct of_device_id max3420_udc_of_match[] = {
    { .compatible = "maxim,max3420-udc"},
    { .compatible = "maxim,max3421-udc"},
    {},
    };
    MODULE_DEVICE_TABLE(of, max3420_udc_of_match);
    static struct spi_driver max3420_driver = {
    .driver = {
    .name = "max3420-udc",
    .of_match_table = max3420_udc_of_match,
    },
    .probe = max3420_probe,
    .remove = max3420_remove,
    };
    module_spi_driver(max3420_driver);
    MODULE_DESCRIPTION(DRIVER_DESC);
    MODULE_AUTHOR("Jassi Brar <jaswinder.singh@linaro.org>");
    MODULE_LICENSE("GPL");
