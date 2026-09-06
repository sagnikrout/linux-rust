//! Automatically rewritten from C to Rust
//! Source: drivers/usb/host/fhci-tds.c
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
// Freescale QUICC Engine USB Host Controller Driver
//
// Copyright (c) Freescale Semicondutor, Inc. 2006.
// Shlomi Gridish <gridish@freescale.com>
// Jerry Huang <Chang-Ming.Huang@freescale.com>
// Copyright (c) Logic Product Development, Inc. 2007
// Peter Barada <peterb@logicpd.com>
// Copyright (c) MontaVista Software, Inc. 2008.
// Anton Vorontsov <avorontsov@ru.mvista.com>
//

pub const DUMMY_BD_BUFFER: c_uint = 0xdeadbeef;
pub const DUMMY2_BD_BUFFER: c_uint = 0xbaadf00d;
// Transaction Descriptors bits
pub const TD_R: c_uint = 0x8000 /* ready bit */;
pub const TD_W: c_uint = 0x2000 /* wrap bit */;
pub const TD_I: c_uint = 0x1000 /* interrupt on completion */;
pub const TD_L: c_uint = 0x0800 /* last */;
pub const TD_TC: c_uint = 0x0400 /* transmit CRC */;
pub const TD_CNF: c_uint = 0x0200 /* CNF - Must be always 1 */;
pub const TD_LSP: c_uint = 0x0100 /* Low-speed transaction */;
pub const TD_PID: c_uint = 0x00c0 /* packet id */;
pub const TD_RXER: c_uint = 0x0020 /* Rx error or not */;
pub const TD_NAK: c_uint = 0x0010 /* No ack. */;
pub const TD_STAL: c_uint = 0x0008 /* Stall received */;
pub const TD_TO: c_uint = 0x0004 /* time out */;
pub const TD_UN: c_uint = 0x0002 /* underrun */;
pub const TD_NO: c_uint = 0x0010 /* Rx Non Octet Aligned Packet */;
pub const TD_AB: c_uint = 0x0008 /* Frame Aborted */;
pub const TD_CR: c_uint = 0x0004 /* CRC Error */;
pub const TD_OV: c_uint = 0x0002 /* Overrun */;
pub const TD_BOV: c_uint = 0x0001 /* Buffer Overrun */;

    TD_NO | TD_AB | TD_CR | TD_OV | TD_BOV)
pub const TD_PID_DATA0: c_uint = 0x0080 /* Data 0 toggle */;
pub const TD_PID_DATA1: c_uint = 0x00c0 /* Data 1 toggle */;
pub const TD_PID_TOGGLE: c_uint = 0x00c0 /* Data 0/1 toggle mask */;
pub const TD_TOK_SETUP: c_uint = 0x0000;
pub const TD_TOK_OUT: c_uint = 0x4000;
pub const TD_TOK_IN: c_uint = 0x8000;
pub const TD_ISO: c_uint = 0x1000;
pub const TD_ENDP: c_uint = 0x0780;
pub const TD_ADDR: c_uint = 0x007f;
pub const TD_ENDP_SHIFT: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_td {
    pub status: __be16,
    pub length: __be16,
    pub buf_ptr: __be32,
    pub extra: __be16,
    pub reserved: __be16,
}

    static struct usb_td __iomem *next_bd(struct usb_td __iomem *base,
    struct usb_td __iomem *td,
    u16 status)
    {
    if (status & TD_W)
    return base;
    else
    return ++td;
    }
#[no_mangle]
pub unsafe extern "C" fn fhci_push_dummy_bd(ep: *mut endpoint) {
    void fhci_push_dummy_bd(struct endpoint *ep)
    {
    if (!ep.already_pushed_dummy_bd) {
    let mut td_status: u16 = in_be16(&ep.empty_td.status);
    out_be32(&ep.empty_td.buf_ptr, DUMMY_BD_BUFFER);
// get the next TD in the ring
    ep.empty_td = next_bd(ep.td_base, ep.empty_td, td_status);
    ep.already_pushed_dummy_bd = true;
    }
    }
// destroy an USB endpoint
#[no_mangle]
pub unsafe extern "C" fn fhci_ep0_free(usb: *mut fhci_usb) {
    void fhci_ep0_free(struct fhci_usb *usb)
    {
    struct endpoint *ep;
    int size;
    ep = usb.ep0;
    if (ep) {
    if (ep.td_base)
    cpm_muram_free(cpm_muram_offset(ep.td_base));
    if (kfifo_initialized(&ep.conf_frame_Q)) {
    size = cq_howmany(&ep.conf_frame_Q);
    for (; size; size--) {
    struct packet *pkt = cq_get(&ep.conf_frame_Q);
    kfree(pkt);
    }
    cq_delete(&ep.conf_frame_Q);
    }
    if (kfifo_initialized(&ep.empty_frame_Q)) {
    size = cq_howmany(&ep.empty_frame_Q);
    for (; size; size--) {
    struct packet *pkt = cq_get(&ep.empty_frame_Q);
    kfree(pkt);
    }
    cq_delete(&ep.empty_frame_Q);
    }
    if (kfifo_initialized(&ep.dummy_packets_Q)) {
    size = cq_howmany(&ep.dummy_packets_Q);
    for (; size; size--) {
    u8 *buff = cq_get(&ep.dummy_packets_Q);
    kfree(buff);
    }
    cq_delete(&ep.dummy_packets_Q);
    }
    kfree(ep);
    usb.ep0 = core::ptr::null_mut();
    }
    }
//
// create the endpoint structure
//
// arguments:
// usb		A pointer to the data structure of the USB
// data_mem	The data memory partition(BUS)
// ring_len	TD ring length
//
    u32 fhci_create_ep(struct fhci_usb *usb, enum fhci_mem_alloc data_mem,
    u32 ring_len)
    {
    struct endpoint *ep;
    struct usb_td __iomem *td;
    unsigned long ep_offset;
    char *err_for = "endpoint PRAM";
    int ep_mem_size;
    u32 i;
// we need at least 3 TDs in the ring
    if (!(ring_len > 2)) {
    fhci_err(usb.fhci, "illegal TD ring length parameters\n");
    return -EINVAL;
    }
    ep = kzalloc_obj(*ep);
    if (!ep)
    return -ENOMEM;
    ep_mem_size = ring_len * sizeof(*td) + sizeof(struct fhci_ep_pram);
    ep_offset = cpm_muram_alloc(ep_mem_size, 32);
    if (IS_ERR_VALUE(ep_offset))
    goto err;
    ep.td_base = cpm_muram_addr(ep_offset);
// zero all queue pointers
    if (cq_new(&ep.conf_frame_Q, ring_len + 2) ||
    cq_new(&ep.empty_frame_Q, ring_len + 2) ||
    cq_new(&ep.dummy_packets_Q, ring_len + 2)) {
    err_for = "frame_queues";
    goto err;
    }
    for (i = 0; i < (ring_len + 1); i++) {
    struct packet *pkt;
    u8 *buff;
    pkt = kmalloc_obj(*pkt);
    if (!pkt) {
    err_for = "frame";
    goto err;
    }
    buff = kmalloc_array(1028, sizeof(*buff), GFP_KERNEL);
    if (!buff) {
    kfree(pkt);
    err_for = "buffer";
    goto err;
    }
    cq_put(&ep.empty_frame_Q, pkt);
    cq_put(&ep.dummy_packets_Q, buff);
    }
// we put the endpoint parameter RAM right behind the TD ring
    ep.ep_pram_ptr = (void __iomem *)ep.td_base + sizeof(*td) * ring_len;
    ep.conf_td = ep.td_base;
    ep.empty_td = ep.td_base;
    ep.already_pushed_dummy_bd = false;
// initialize tds
    td = ep.td_base;
    for (i = 0; i < ring_len; i++) {
    out_be32(&td.buf_ptr, 0);
    out_be16(&td.status, 0);
    out_be16(&td.length, 0);
    out_be16(&td.extra, 0);
    td++;
    }
    td--;
    out_be16(&td.status, TD_W); /* for last TD set Wrap bit */
    out_be16(&td.length, 0);
// endpoint structure has been created
    usb.ep0 = ep;
    return 0;
    err:
    fhci_ep0_free(usb);
    kfree(ep);
    fhci_err(usb.fhci, "no memory for the %s\n", err_for);
    return -ENOMEM;
    }
//
// initialize the endpoint register according to the given parameters
//
// artuments:
// usb		A pointer to the data strucutre of the USB
// ep		A pointer to the endpoint structre
// data_mem	The data memory partition(BUS)
//
    void fhci_init_ep_registers(struct fhci_usb *usb, struct endpoint *ep,
    enum fhci_mem_alloc data_mem)
    {
    u8 rt;
// set the endpoint registers according to the endpoint
    out_be16(&usb.fhci.regs.usb_usep[0],
    USB_TRANS_CTR | USB_EP_MF | USB_EP_RTE);
    out_be16(&usb.fhci.pram.ep_ptr[0],
    cpm_muram_offset(ep.ep_pram_ptr));
    rt = (BUS_MODE_BO_BE | BUS_MODE_GBL);

    if (data_mem == MEM_SECONDARY)
    rt |= BUS_MODE_DTB;

    out_8(&ep.ep_pram_ptr.rx_func_code, rt);
    out_8(&ep.ep_pram_ptr.tx_func_code, rt);
    out_be16(&ep.ep_pram_ptr.rx_buff_len, 1028);
    out_be16(&ep.ep_pram_ptr.rx_base, 0);
    out_be16(&ep.ep_pram_ptr.tx_base, cpm_muram_offset(ep.td_base));
    out_be16(&ep.ep_pram_ptr.rx_bd_ptr, 0);
    out_be16(&ep.ep_pram_ptr.tx_bd_ptr, cpm_muram_offset(ep.td_base));
    out_be32(&ep.ep_pram_ptr.tx_state, 0);
    }
//
// Collect the submitted frames and inform the application about them
// It is also preparing the TDs for new frames. If the Tx interrupts
// are disabled, the application should call that routine to get
// confirmation about the submitted frames. Otherwise, the routine is
// called from the interrupt service routine during the Tx interrupt.
// In that case the application is informed by calling the application
// specific 'fhci_transaction_confirm' routine
//
#[no_mangle]
unsafe extern "C" fn fhci_td_transaction_confirm(usb: *mut fhci_usb) {
    static void fhci_td_transaction_confirm(struct fhci_usb *usb)
    {
    struct endpoint *ep = usb.ep0;
    struct packet *pkt;
    struct usb_td __iomem *td;
    u16 extra_data;
    u16 td_status;
    u16 td_length;
    u32 buf;
//
// collect transmitted BDs from the chip. The routine clears all BDs
// with R bit = 0 and the pointer to data buffer is not NULL, that is
// BDs which point to the transmitted data buffer
//
    while (1) {
    td = ep.conf_td;
    td_status = in_be16(&td.status);
    td_length = in_be16(&td.length);
    buf = in_be32(&td.buf_ptr);
    extra_data = in_be16(&td.extra);
// check if the TD is empty
    if (!(!(td_status & TD_R) && ((td_status & ~TD_W) || buf)))
    break;
// check if it is a dummy buffer
#[no_mangle]
pub unsafe extern "C" fn if(~TD_W): (buf == DUMMY_BD_BUFFER) && !(td_status &) -> else {
    else if ((buf == DUMMY_BD_BUFFER) && !(td_status & ~TD_W))
    break;
// mark TD as empty
    clrbits16(&td.status, ~TD_W);
    out_be16(&td.length, 0);
    out_be32(&td.buf_ptr, 0);
    out_be16(&td.extra, 0);
// advance the TD pointer
    ep.conf_td = next_bd(ep.td_base, ep.conf_td, td_status);
// check if it is a dummy buffer(type2)
    if ((buf == DUMMY2_BD_BUFFER) && !(td_status & ~TD_W))
    continue;
    pkt = cq_get(&ep.conf_frame_Q);
    if (!pkt)
    fhci_err(usb.fhci, "no frame to confirm\n");
    if (td_status & TD_ERRORS) {
    if (td_status & TD_RXER) {
    if (td_status & TD_CR)
    pkt.status = USB_TD_RX_ER_CRC;
#[no_mangle]
pub unsafe extern "C" fn if(TD_AB: td_status &) -> else {
    else if (td_status & TD_AB)
    pkt.status = USB_TD_RX_ER_BITSTUFF;
#[no_mangle]
pub unsafe extern "C" fn if(TD_OV: td_status &) -> else {
    else if (td_status & TD_OV)
    pkt.status = USB_TD_RX_ER_OVERUN;
#[no_mangle]
pub unsafe extern "C" fn if(TD_BOV: td_status &) -> else {
    else if (td_status & TD_BOV)
    pkt.status = USB_TD_RX_DATA_OVERUN;
#[no_mangle]
pub unsafe extern "C" fn if(TD_NO: td_status &) -> else {
    else if (td_status & TD_NO)
    pkt.status = USB_TD_RX_ER_NONOCT;
    else
    fhci_err(usb.fhci, "illegal error "
    "occurred\n");
    } else if (td_status & TD_NAK)
    pkt.status = USB_TD_TX_ER_NAK;
#[no_mangle]
pub unsafe extern "C" fn if(TD_TO: td_status &) -> else {
    else if (td_status & TD_TO)
    pkt.status = USB_TD_TX_ER_TIMEOUT;
#[no_mangle]
pub unsafe extern "C" fn if(TD_UN: td_status &) -> else {
    else if (td_status & TD_UN)
    pkt.status = USB_TD_TX_ER_UNDERUN;
#[no_mangle]
pub unsafe extern "C" fn if(TD_STAL: td_status &) -> else {
    else if (td_status & TD_STAL)
    pkt.status = USB_TD_TX_ER_STALL;
    else
    fhci_err(usb.fhci, "illegal error occurred\n");
    } else if ((extra_data & TD_TOK_IN) &&
    pkt.len > td_length - CRC_SIZE) {
    pkt.status = USB_TD_RX_DATA_UNDERUN;
    }
    if (extra_data & TD_TOK_IN)
    pkt.len = td_length - CRC_SIZE;
#[no_mangle]
pub unsafe extern "C" fn if(PKT_ZLP: pkt->info &) -> else {
    else if (pkt.info & PKT_ZLP)
    pkt.len = 0;
    else
    pkt.len = td_length;
    fhci_transaction_confirm(usb, pkt);
    }
    }
//
// Submitting a data frame to a specified endpoint of a USB device
// The frame is put in the driver's transmit queue for this endpoint
//
// Arguments:
// usb          A pointer to the USB structure
// pkt          A pointer to the user frame structure
// trans_type   Transaction tyep - IN,OUT or SETUP
// dest_addr    Device address - 0~127
// dest_ep      Endpoint number of the device - 0~16
// trans_mode   Pipe type - ISO,Interrupt,bulk or control
// dest_speed   USB speed - Low speed or FULL speed
// data_toggle  Data sequence toggle - 0 or 1
//
    u32 fhci_host_transaction(struct fhci_usb *usb,
    struct packet *pkt,
    enum fhci_ta_type trans_type,
    u8 dest_addr,
    u8 dest_ep,
    enum fhci_tf_mode trans_mode,
    enum fhci_speed dest_speed, u8 data_toggle)
    {
    struct endpoint *ep = usb.ep0;
    struct usb_td __iomem *td;
    u16 extra_data;
    u16 td_status;
    fhci_usb_disable_interrupt(usb);
// start from the next BD that should be filled
    td = ep.empty_td;
    td_status = in_be16(&td.status);
    if (td_status & TD_R && in_be16(&td.length)) {
// if the TD is not free
    fhci_usb_enable_interrupt(usb);
    return -1;
    }
// get the next TD in the ring
    ep.empty_td = next_bd(ep.td_base, ep.empty_td, td_status);
    fhci_usb_enable_interrupt(usb);
    pkt.priv_data = td;
    out_be32(&td.buf_ptr, virt_to_phys(pkt.data));
// sets up transaction parameters - addr,endp,dir,and type
    extra_data = (dest_ep << TD_ENDP_SHIFT) | dest_addr;
    switch (trans_type) {
    case FHCI_TA_IN:
    extra_data |= TD_TOK_IN;
    break;
    case FHCI_TA_OUT:
    extra_data |= TD_TOK_OUT;
    break;
    case FHCI_TA_SETUP:
    extra_data |= TD_TOK_SETUP;
    break;
    }
    if (trans_mode == FHCI_TF_ISO)
    extra_data |= TD_ISO;
    out_be16(&td.extra, extra_data);
// sets up the buffer descriptor
    td_status = ((td_status & TD_W) | TD_R | TD_L | TD_I | TD_CNF);
    if (!(pkt.info & PKT_NO_CRC))
    td_status |= TD_TC;
    switch (trans_type) {
    case FHCI_TA_IN:
    if (data_toggle)
    pkt.info |= PKT_PID_DATA1;
    else
    pkt.info |= PKT_PID_DATA0;
    break;
    default:
    if (data_toggle) {
    td_status |= TD_PID_DATA1;
    pkt.info |= PKT_PID_DATA1;
    } else {
    td_status |= TD_PID_DATA0;
    pkt.info |= PKT_PID_DATA0;
    }
    break;
    }
    if ((dest_speed == FHCI_LOW_SPEED) &&
    (usb.port_status == FHCI_PORT_FULL))
    td_status |= TD_LSP;
    out_be16(&td.status, td_status);
// set up buffer length
    if (trans_type == FHCI_TA_IN)
    out_be16(&td.length, pkt.len + CRC_SIZE);
    else
    out_be16(&td.length, pkt.len);
// put the frame to the confirmation queue
    cq_put(&ep.conf_frame_Q, pkt);
    if (cq_howmany(&ep.conf_frame_Q) == 1)
    out_8(&usb.fhci.regs.usb_uscom, USB_CMD_STR_FIFO);
    return 0;
    }
// Reset the Tx BD ring
#[no_mangle]
pub unsafe extern "C" fn fhci_flush_bds(usb: *mut fhci_usb) {
    void fhci_flush_bds(struct fhci_usb *usb)
    {
    u16 td_status;
    struct usb_td __iomem *td;
    struct endpoint *ep = usb.ep0;
    td = ep.td_base;
    while (1) {
    td_status = in_be16(&td.status);
    in_be32(&td.buf_ptr);
    in_be16(&td.extra);
// if the TD is not empty - we'll confirm it as Timeout
    if (td_status & TD_R)
    out_be16(&td.status, (td_status & ~TD_R) | TD_TO);
// if this TD is dummy - let's skip this TD
#[no_mangle]
pub unsafe extern "C" fn if(DUMMY_BD_BUFFER: in_be32(&td->buf_ptr) ==) -> else {
    else if (in_be32(&td.buf_ptr) == DUMMY_BD_BUFFER)
    out_be32(&td.buf_ptr, DUMMY2_BD_BUFFER);
// if this is the last TD - break
    if (td_status & TD_W)
    break;
    td++;
    }
    fhci_td_transaction_confirm(usb);
    td = ep.td_base;
    do {
    out_be16(&td.status, 0);
    out_be16(&td.length, 0);
    out_be32(&td.buf_ptr, 0);
    out_be16(&td.extra, 0);
    td++;
    } while (!(in_be16(&td.status) & TD_W));
    out_be16(&td.status, TD_W); /* for last TD set Wrap bit */
    out_be16(&td.length, 0);
    out_be32(&td.buf_ptr, 0);
    out_be16(&td.extra, 0);
    out_be16(&ep.ep_pram_ptr.tx_bd_ptr,
    in_be16(&ep.ep_pram_ptr.tx_base));
    out_be32(&ep.ep_pram_ptr.tx_state, 0);
    out_be16(&ep.ep_pram_ptr.tx_cnt, 0);
    ep.empty_td = ep.td_base;
    ep.conf_td = ep.td_base;
    }
//
// Flush all transmitted packets from TDs in the actual frame.
// This routine is called when something wrong with the controller and
// we want to get rid of the actual frame and start again next frame
//
#[no_mangle]
pub unsafe extern "C" fn fhci_flush_actual_frame(usb: *mut fhci_usb) {
    void fhci_flush_actual_frame(struct fhci_usb *usb)
    {
    u8 mode;
    u16 tb_ptr;
    u16 td_status;
    u32 buf_ptr;
    struct usb_td __iomem *td;
    struct endpoint *ep = usb.ep0;
// disable the USB controller
    mode = in_8(&usb.fhci.regs.usb_usmod);
    out_8(&usb.fhci.regs.usb_usmod, mode & ~USB_MODE_EN);
    tb_ptr = in_be16(&ep.ep_pram_ptr.tx_bd_ptr);
    td = cpm_muram_addr(tb_ptr);
    td_status = in_be16(&td.status);
    buf_ptr = in_be32(&td.buf_ptr);
    in_be16(&td.extra);
    do {
    if (td_status & TD_R) {
    out_be16(&td.status, (td_status & ~TD_R) | TD_TO);
    } else {
    out_be32(&td.buf_ptr, 0);
    ep.already_pushed_dummy_bd = false;
    break;
    }
// advance the TD pointer
    td = next_bd(ep.td_base, td, td_status);
    td_status = in_be16(&td.status);
    buf_ptr = in_be32(&td.buf_ptr);
    in_be16(&td.extra);
    } while ((td_status & TD_R) || buf_ptr);
    fhci_td_transaction_confirm(usb);
    out_be16(&ep.ep_pram_ptr.tx_bd_ptr,
    in_be16(&ep.ep_pram_ptr.tx_base));
    out_be32(&ep.ep_pram_ptr.tx_state, 0);
    out_be16(&ep.ep_pram_ptr.tx_cnt, 0);
    ep.empty_td = ep.td_base;
    ep.conf_td = ep.td_base;
    usb.actual_frame.frame_status = FRAME_TIMER_END_TRANSMISSION;
// reset the event register
    out_be16(&usb.fhci.regs.usb_usber, 0xffff);
// enable the USB controller
    out_8(&usb.fhci.regs.usb_usmod, mode | USB_MODE_EN);
    }
// handles Tx confirm and Tx error interrupt
#[no_mangle]
pub unsafe extern "C" fn fhci_tx_conf_interrupt(usb: *mut fhci_usb) {
    void fhci_tx_conf_interrupt(struct fhci_usb *usb)
    {
    fhci_td_transaction_confirm(usb);
//
// Schedule another transaction to this frame only if we have
// already confirmed all transaction in the frame.
//
    if (((fhci_get_sof_timer_count(usb) < usb.max_frame_usage) ||
    (usb.actual_frame.frame_status & FRAME_END_TRANSMISSION)) &&
    (list_empty(&usb.actual_frame.tds_list)))
    fhci_schedule_transactions(usb);
    }
#[no_mangle]
pub unsafe extern "C" fn fhci_host_transmit_actual_frame(usb: *mut fhci_usb) {
    void fhci_host_transmit_actual_frame(struct fhci_usb *usb)
    {
    u16 tb_ptr;
    u16 td_status;
    struct usb_td __iomem *td;
    struct endpoint *ep = usb.ep0;
    tb_ptr = in_be16(&ep.ep_pram_ptr.tx_bd_ptr);
    td = cpm_muram_addr(tb_ptr);
    if (in_be32(&td.buf_ptr) == DUMMY_BD_BUFFER) {
    struct usb_td __iomem *old_td = td;
    ep.already_pushed_dummy_bd = false;
    td_status = in_be16(&td.status);
// gets the next TD in the ring
    td = next_bd(ep.td_base, td, td_status);
    tb_ptr = cpm_muram_offset(td);
    out_be16(&ep.ep_pram_ptr.tx_bd_ptr, tb_ptr);
// start transmit only if we have something in the TDs
    if (in_be16(&td.status) & TD_R)
    out_8(&usb.fhci.regs.usb_uscom, USB_CMD_STR_FIFO);
    if (in_be32(&ep.conf_td.buf_ptr) == DUMMY_BD_BUFFER) {
    out_be32(&old_td.buf_ptr, 0);
    ep.conf_td = next_bd(ep.td_base, ep.conf_td,
    td_status);
    } else {
    out_be32(&old_td.buf_ptr, DUMMY2_BD_BUFFER);
    }
    }
    }
