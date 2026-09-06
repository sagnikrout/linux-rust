//! Automatically rewritten from C to Rust
//! Source: drivers/usb/dwc3/ep0.c
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
// ep0.c - DesignWare USB3 DRD Controller Endpoint 0 Handling
//
// Copyright (C) 2010-2011 Texas Instruments Incorporated - https://www.ti.com
//
// Authors: Felipe Balbi <balbi@ti.com>,
// Sebastian Andrzej Siewior <bigeasy@linutronix.de>
//

    static void __dwc3_ep0_do_control_status(struct dwc3 *dwc, struct dwc3_ep *dep);
    static void __dwc3_ep0_do_control_data(struct dwc3 *dwc,
    struct dwc3_ep *dep, struct dwc3_request *req);
    static int dwc3_ep0_delegate_req(struct dwc3 *dwc,
    struct usb_ctrlrequest *ctrl);
    static void dwc3_ep0_prepare_one_trb(struct dwc3_ep *dep,
    dma_addr_t buf_dma, u32 len, u32 type, bool chain)
    {
    struct dwc3_trb			*trb;
    struct dwc3			*dwc;
    dwc = dep.dwc;
    trb = &dwc.ep0_trb[dep.trb_enqueue];
    if (chain)
    dep.trb_enqueue++;
    trb.bpl = lower_32_bits(buf_dma);
    trb.bph = upper_32_bits(buf_dma);
    trb.size = len;
    trb.ctrl = type;
    trb.ctrl |= (DWC3_TRB_CTRL_HWO
    | DWC3_TRB_CTRL_ISP_IMI);
    if (chain)
    trb.ctrl |= DWC3_TRB_CTRL_CHN;
    else
    trb.ctrl |= (DWC3_TRB_CTRL_IOC
    | DWC3_TRB_CTRL_LST);
    trace_dwc3_prepare_trb(dep, trb);
    }
#[no_mangle]
unsafe extern "C" fn dwc3_ep0_start_trans(dep: *mut dwc3_ep) -> c_int {
    static int dwc3_ep0_start_trans(struct dwc3_ep *dep)
    {
    struct dwc3_gadget_ep_cmd_params params;
    struct dwc3			*dwc;
    int				ret;
    if (dep.flags & DWC3_EP_TRANSFER_STARTED)
    return 0;
    dwc = dep.dwc;
    memset(&params, 0, sizeof(params));
    params.param0 = upper_32_bits(dwc.ep0_trb_addr);
    params.param1 = lower_32_bits(dwc.ep0_trb_addr);
    ret = dwc3_send_gadget_ep_cmd(dep, DWC3_DEPCMD_STARTTRANSFER, &params);
    if (ret < 0)
    return ret;
    dwc.ep0_next_event = DWC3_EP0_COMPLETE;
    return 0;
    }
    static int __dwc3_gadget_ep0_queue(struct dwc3_ep *dep,
    struct dwc3_request *req)
    {
    struct dwc3		*dwc = dep.dwc;
    req.request.actual	= 0;
    req.request.status	= -EINPROGRESS;
    req.epnum		= dep.number;
    req.status		= DWC3_REQUEST_STATUS_QUEUED;
    list_add_tail(&req.list, &dep.pending_list);
//
// Gadget driver might not be quick enough to queue a request
// before we get a Transfer Not Ready event on this endpoint.
//
// In that case, we will set DWC3_EP_PENDING_REQUEST. When that
// flag is set, it's telling us that as soon as Gadget queues the
// required request, we should kick the transfer here because the
// IRQ we were waiting for is long gone.
//
    if (dep.flags & DWC3_EP_PENDING_REQUEST) {
    unsigned int direction;
    direction = !!(dep.flags & DWC3_EP0_DIR_IN);
    if (dwc.ep0state != EP0_DATA_PHASE) {
    dev_WARN(dwc.dev, "Unexpected pending request\n");
    return 0;
    }
    __dwc3_ep0_do_control_data(dwc, dwc.eps[direction], req);
    dep.flags &= ~(DWC3_EP_PENDING_REQUEST |
    DWC3_EP0_DIR_IN);
    return 0;
    }
//
// In case gadget driver asked us to delay the STATUS phase,
// handle it here.
//
    if (dwc.delayed_status) {
    unsigned int direction;
    direction = !dwc.ep0_expect_in;
    dwc.delayed_status = false;
    usb_gadget_set_state(dwc.gadget, USB_STATE_CONFIGURED);
    if (dwc.ep0state == EP0_STATUS_PHASE)
    __dwc3_ep0_do_control_status(dwc, dwc.eps[direction]);
    return 0;
    }
//
// Unfortunately we have uncovered a limitation wrt the Data Phase.
//
// Section 9.4 says we can wait for the XferNotReady(DATA) event to
// come before issuing Start Transfer command, but if we do, we will
// miss situations where the host starts another SETUP phase instead of
// the DATA phase.  Such cases happen at least on TD.7.6 of the Link
// Layer Compliance Suite.
//
// The problem surfaces due to the fact that in case of back-to-back
// SETUP packets there will be no XferNotReady(DATA) generated and we
// will be stuck waiting for XferNotReady(DATA) forever.
//
// By looking at tables 9-13 and 9-14 of the Databook, we can see that
// it tells us to start Data Phase right away. It also mentions that if
// we receive a SETUP phase instead of the DATA phase, core will issue
// XferComplete for the DATA phase, before actually initiating it in
// the wire, with the TRB's status set to "SETUP_PENDING". Such status
// can only be used to print some debugging logs, as the core expects
// us to go through to the STATUS phase and start a CONTROL_STATUS TRB,
// just so it completes right away, without transferring anything and,
// only then, we can go back to the SETUP phase.
//
// Because of this scenario, SNPS decided to change the programming
// model of control transfers and support on-demand transfers only for
// the STATUS phase. To fix the issue we have now, we will always wait
// for gadget driver to queue the DATA phase's struct usb_request, then
// start it right away.
//
// If we're actually in a 2-stage transfer, we will wait for
// XferNotReady(STATUS).
//
    if (dwc.three_stage_setup) {
    unsigned int direction;
    direction = dwc.ep0_expect_in;
    dwc.ep0state = EP0_DATA_PHASE;
    __dwc3_ep0_do_control_data(dwc, dwc.eps[direction], req);
    dep.flags &= ~DWC3_EP0_DIR_IN;
    }
    return 0;
    }
    int dwc3_gadget_ep0_queue(struct usb_ep *ep, struct usb_request *request,
    gfp_t gfp_flags)
    {
    struct dwc3_request		*req = to_dwc3_request(request);
    struct dwc3_ep			*dep = to_dwc3_ep(ep);
    struct dwc3			*dwc = dep.dwc;
    unsigned long			flags;
    int				ret;
    spin_lock_irqsave(&dwc.lock, flags);
    if (!dep.endpoint.desc || !dwc.pullups_connected || !dwc.connected) {
    dev_err(dwc.dev, "%s: can't queue to disabled endpoint\n",
    dep.name);
    ret = -ESHUTDOWN;
    goto out;
    }
// we share one TRB for ep0/1
    if (!list_empty(&dep.pending_list)) {
    ret = -EBUSY;
    goto out;
    }
    ret = __dwc3_gadget_ep0_queue(dep, req);
    out:
    spin_unlock_irqrestore(&dwc.lock, flags);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn dwc3_ep0_stall_and_restart(dwc: *mut dwc3) {
    void dwc3_ep0_stall_and_restart(struct dwc3 *dwc)
    {
    struct dwc3_ep		*dep;
// reinitialize physical ep1
    dep = dwc.eps[1];
    dep.flags &= DWC3_EP_RESOURCE_ALLOCATED;
    dep.flags |= DWC3_EP_ENABLED;
// stall is always issued on EP0
    dep = dwc.eps[0];
    __dwc3_gadget_ep_set_halt(dep, 1, false);
    dep.flags &= DWC3_EP_RESOURCE_ALLOCATED | DWC3_EP_TRANSFER_STARTED;
    dep.flags |= DWC3_EP_ENABLED;
    dwc.delayed_status = false;
    if (!list_empty(&dep.pending_list)) {
    struct dwc3_request	*req;
    req = next_request(&dep.pending_list);
    if (!dwc.connected)
    dwc3_gadget_giveback(dep, req, -ESHUTDOWN);
    else
    dwc3_gadget_giveback(dep, req, -ECONNRESET);
    }
    dwc.eps[0].trb_enqueue = 0;
    dwc.eps[1].trb_enqueue = 0;
    dwc.ep0state = EP0_SETUP_PHASE;
    dwc3_ep0_out_start(dwc);
    }
#[no_mangle]
pub unsafe extern "C" fn __dwc3_gadget_ep0_set_halt(ep: *mut usb_ep, value: c_int) -> c_int {
    int __dwc3_gadget_ep0_set_halt(struct usb_ep *ep, int value)
    {
    struct dwc3_ep			*dep = to_dwc3_ep(ep);
    struct dwc3			*dwc = dep.dwc;
    dwc3_ep0_stall_and_restart(dwc);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn dwc3_gadget_ep0_set_halt(ep: *mut usb_ep, value: c_int) -> c_int {
    int dwc3_gadget_ep0_set_halt(struct usb_ep *ep, int value)
    {
    struct dwc3_ep			*dep = to_dwc3_ep(ep);
    struct dwc3			*dwc = dep.dwc;
    unsigned long			flags;
    int				ret;
    spin_lock_irqsave(&dwc.lock, flags);
    ret = __dwc3_gadget_ep0_set_halt(ep, value);
    spin_unlock_irqrestore(&dwc.lock, flags);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn dwc3_ep0_out_start(dwc: *mut dwc3) {
    void dwc3_ep0_out_start(struct dwc3 *dwc)
    {
    struct dwc3_ep			*dep;
    int				ret;
    int                             i;
    complete(&dwc.ep0_in_setup);
    dep = dwc.eps[0];
    dwc3_ep0_prepare_one_trb(dep, dwc.ep0_trb_addr, 8,
    DWC3_TRBCTL_CONTROL_SETUP, false);
    ret = dwc3_ep0_start_trans(dep);
    if (ret < 0)
    dev_err(dwc.dev, "ep0 out start transfer failed: %d\n", ret);
    for (i = 2; i < DWC3_ENDPOINTS_NUM; i++) {
    struct dwc3_ep *dwc3_ep;
    dwc3_ep = dwc.eps[i];
    if (!dwc3_ep)
    continue;
    if (!(dwc3_ep.flags & DWC3_EP_DELAY_STOP))
    continue;
    dwc3_ep.flags &= ~DWC3_EP_DELAY_STOP;
    if (dwc.connected)
    dwc3_stop_active_transfer(dwc3_ep, true, true);
    else
    dwc3_remove_requests(dwc, dwc3_ep, -ESHUTDOWN);
    }
    }
    static struct dwc3_ep *dwc3_wIndex_to_dep(struct dwc3 *dwc, __le16 wIndex_le)
    {
    struct dwc3_ep		*dep;
    let mut windex: u32 = le16_to_cpu(wIndex_le);
    u32			epnum;
    epnum = (windex & USB_ENDPOINT_NUMBER_MASK) << 1;
    if ((windex & USB_ENDPOINT_DIR_MASK) == USB_DIR_IN)
    epnum |= 1;
    dep = dwc.eps[epnum];
    if (dep == core::ptr::null_mut())
    return core::ptr::null_mut();
    if (dep.flags & DWC3_EP_ENABLED)
    return dep;
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn dwc3_ep0_status_cmpl(ep: *mut usb_ep, req: *mut usb_request) {
    static void dwc3_ep0_status_cmpl(struct usb_ep *ep, struct usb_request *req)
    {
    }
//
// ch 9.4.5
//
    static int dwc3_ep0_handle_status(struct dwc3 *dwc,
    struct usb_ctrlrequest *ctrl)
    {
    struct dwc3_ep		*dep;
    u32			recip;
    u32			value;
    u32			reg;
    let mut usb_status: u16 = 0;
    __le16			*response_pkt;
// We don't support PTM_STATUS
    value = le16_to_cpu(ctrl.wValue);
    if (value != 0)
    return -EINVAL;
    recip = ctrl.bRequestType & USB_RECIP_MASK;
    switch (recip) {
    case USB_RECIP_DEVICE:
//
// LTM will be set once we know how to set this in HW.
//
    usb_status |= dwc.gadget.is_selfpowered;
    if ((dwc.speed == DWC3_DSTS_SUPERSPEED) ||
    (dwc.speed == DWC3_DSTS_SUPERSPEED_PLUS)) {
    reg = dwc3_readl(dwc, DWC3_DCTL);
    if (reg & DWC3_DCTL_INITU1ENA)
    usb_status |= 1 << USB_DEV_STAT_U1_ENABLED;
    if (reg & DWC3_DCTL_INITU2ENA)
    usb_status |= 1 << USB_DEV_STAT_U2_ENABLED;
    } else {
    usb_status |= dwc.gadget.wakeup_armed <<
    USB_DEVICE_REMOTE_WAKEUP;
    }
    break;
    case USB_RECIP_INTERFACE:
//
// Function Remote Wake Capable	D0
// Function Remote Wakeup	D1
//
    return dwc3_ep0_delegate_req(dwc, ctrl);
    case USB_RECIP_ENDPOINT:
    dep = dwc3_wIndex_to_dep(dwc, ctrl.wIndex);
    if (!dep)
    return -EINVAL;
    if (dep.flags & DWC3_EP_STALL)
    usb_status = 1 << USB_ENDPOINT_HALT;
    break;
    default:
    return -EINVAL;
    }
    response_pkt = (__le16 *) dwc.setup_buf;
// response_pkt = cpu_to_le16(usb_status);
    dep = dwc.eps[0];
    dwc.ep0_usb_req.dep = dep;
    dwc.ep0_usb_req.request.length = sizeof(*response_pkt);
    dwc.ep0_usb_req.request.buf = dwc.setup_buf;
    dwc.ep0_usb_req.request.complete = dwc3_ep0_status_cmpl;
    return __dwc3_gadget_ep0_queue(dep, &dwc.ep0_usb_req);
    }
    static int dwc3_ep0_handle_u1(struct dwc3 *dwc, enum usb_device_state state,
    int set)
    {
    u32 reg;
    if (state != USB_STATE_CONFIGURED)
    return -EINVAL;
    if ((dwc.speed != DWC3_DSTS_SUPERSPEED) &&
    (dwc.speed != DWC3_DSTS_SUPERSPEED_PLUS))
    return -EINVAL;
    if (set && dwc.dis_u1_entry_quirk)
    return -EINVAL;
    reg = dwc3_readl(dwc, DWC3_DCTL);
    if (set)
    reg |= DWC3_DCTL_INITU1ENA;
    else
    reg &= ~DWC3_DCTL_INITU1ENA;
    dwc3_writel(dwc, DWC3_DCTL, reg);
    return 0;
    }
    static int dwc3_ep0_handle_u2(struct dwc3 *dwc, enum usb_device_state state,
    int set)
    {
    u32 reg;
    if (state != USB_STATE_CONFIGURED)
    return -EINVAL;
    if ((dwc.speed != DWC3_DSTS_SUPERSPEED) &&
    (dwc.speed != DWC3_DSTS_SUPERSPEED_PLUS))
    return -EINVAL;
    if (set && dwc.dis_u2_entry_quirk)
    return -EINVAL;
    reg = dwc3_readl(dwc, DWC3_DCTL);
    if (set)
    reg |= DWC3_DCTL_INITU2ENA;
    else
    reg &= ~DWC3_DCTL_INITU2ENA;
    dwc3_writel(dwc, DWC3_DCTL, reg);
    return 0;
    }
    static int dwc3_ep0_handle_test(struct dwc3 *dwc, enum usb_device_state state,
    u32 wIndex, int set)
    {
    if ((wIndex & 0xff) != 0)
    return -EINVAL;
    if (!set)
    return -EINVAL;
    switch (wIndex >> 8) {
    case USB_TEST_J:
    case USB_TEST_K:
    case USB_TEST_SE0_NAK:
    case USB_TEST_PACKET:
    case USB_TEST_FORCE_ENABLE:
    dwc.test_mode_nr = wIndex >> 8;
    dwc.test_mode = true;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static int dwc3_ep0_handle_device(struct dwc3 *dwc,
    struct usb_ctrlrequest *ctrl, int set)
    {
    enum usb_device_state	state;
    u32			wValue;
    u32			wIndex;
    let mut ret: c_int = 0;
    wValue = le16_to_cpu(ctrl.wValue);
    wIndex = le16_to_cpu(ctrl.wIndex);
    state = dwc.gadget.state;
    switch (wValue) {
    case USB_DEVICE_REMOTE_WAKEUP:
    if (dwc.wakeup_configured)
    dwc.gadget.wakeup_armed = set;
    else
    ret = -EINVAL;
    break;
//
// 9.4.1 says only for SS, in AddressState only for
// default control pipe
//
    case USB_DEVICE_U1_ENABLE:
    ret = dwc3_ep0_handle_u1(dwc, state, set);
    break;
    case USB_DEVICE_U2_ENABLE:
    ret = dwc3_ep0_handle_u2(dwc, state, set);
    break;
    case USB_DEVICE_LTM_ENABLE:
    ret = -EINVAL;
    break;
    case USB_DEVICE_TEST_MODE:
    ret = dwc3_ep0_handle_test(dwc, state, wIndex, set);
    break;
    default:
    ret = -EINVAL;
    }
    return ret;
    }
    static int dwc3_ep0_handle_intf(struct dwc3 *dwc,
    struct usb_ctrlrequest *ctrl, int set)
    {
    u32			wValue;
    let mut ret: c_int = 0;
    wValue = le16_to_cpu(ctrl.wValue);
    switch (wValue) {
    case USB_INTRF_FUNC_SUSPEND:
    ret = dwc3_ep0_delegate_req(dwc, ctrl);
    break;
    default:
    ret = -EINVAL;
    }
    return ret;
    }
    static int dwc3_ep0_handle_endpoint(struct dwc3 *dwc,
    struct usb_ctrlrequest *ctrl, int set)
    {
    struct dwc3_ep		*dep;
    u32			wValue;
    int			ret;
    wValue = le16_to_cpu(ctrl.wValue);
    switch (wValue) {
    case USB_ENDPOINT_HALT:
    dep = dwc3_wIndex_to_dep(dwc, ctrl.wIndex);
    if (!dep)
    return -EINVAL;
    if (set == 0 && (dep.flags & DWC3_EP_WEDGE))
    break;
    ret = __dwc3_gadget_ep_set_halt(dep, set, true);
    if (ret)
    return -EINVAL;
// ClearFeature(Halt) may need delayed status
    if (!set && (dep.flags & DWC3_EP_END_TRANSFER_PENDING))
    return USB_GADGET_DELAYED_STATUS;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static int dwc3_ep0_handle_feature(struct dwc3 *dwc,
    struct usb_ctrlrequest *ctrl, int set)
    {
    u32			recip;
    int			ret;
    recip = ctrl.bRequestType & USB_RECIP_MASK;
    switch (recip) {
    case USB_RECIP_DEVICE:
    ret = dwc3_ep0_handle_device(dwc, ctrl, set);
    break;
    case USB_RECIP_INTERFACE:
    ret = dwc3_ep0_handle_intf(dwc, ctrl, set);
    break;
    case USB_RECIP_ENDPOINT:
    ret = dwc3_ep0_handle_endpoint(dwc, ctrl, set);
    break;
    default:
    ret = -EINVAL;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn dwc3_ep0_set_address(dwc: *mut dwc3, ctrl: *mut usb_ctrlrequest) -> c_int {
    static int dwc3_ep0_set_address(struct dwc3 *dwc, struct usb_ctrlrequest *ctrl)
    {
    let mut state: enum usb_device_state = dwc.gadget.state;
    u32 addr;
    u32 reg;
    addr = le16_to_cpu(ctrl.wValue);
    if (addr > 127) {
    dev_err(dwc.dev, "invalid device address %d\n", addr);
    return -EINVAL;
    }
    if (state == USB_STATE_CONFIGURED) {
    dev_err(dwc.dev, "can't SetAddress() from Configured State\n");
    return -EINVAL;
    }
    reg = dwc3_readl(dwc, DWC3_DCFG);
    reg &= ~(DWC3_DCFG_DEVADDR_MASK);
    reg |= DWC3_DCFG_DEVADDR(addr);
    dwc3_writel(dwc, DWC3_DCFG, reg);
    if (addr)
    usb_gadget_set_state(dwc.gadget, USB_STATE_ADDRESS);
    else
    usb_gadget_set_state(dwc.gadget, USB_STATE_DEFAULT);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dwc3_ep0_delegate_req(dwc: *mut dwc3, ctrl: *mut usb_ctrlrequest) -> c_int {
    static int dwc3_ep0_delegate_req(struct dwc3 *dwc, struct usb_ctrlrequest *ctrl)
    {
    let mut ret: c_int = -EINVAL;
    if (dwc.async_callbacks) {
    spin_unlock(&dwc.lock);
    ret = dwc.gadget_driver.setup(dwc.gadget, ctrl);
    spin_lock(&dwc.lock);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn dwc3_ep0_set_config(dwc: *mut dwc3, ctrl: *mut usb_ctrlrequest) -> c_int {
    static int dwc3_ep0_set_config(struct dwc3 *dwc, struct usb_ctrlrequest *ctrl)
    {
    let mut state: enum usb_device_state = dwc.gadget.state;
    u32 cfg;
    int ret;
    u32 reg;
    cfg = le16_to_cpu(ctrl.wValue);
    switch (state) {
    case USB_STATE_DEFAULT:
    return -EINVAL;
    case USB_STATE_ADDRESS:
    dwc3_gadget_start_config(dwc, 2);
    dwc3_gadget_clear_tx_fifos(dwc);
    ret = dwc3_ep0_delegate_req(dwc, ctrl);
// if the cfg matches and the cfg is non zero
    if (cfg && (!ret || (ret == USB_GADGET_DELAYED_STATUS))) {
//
// only change state if set_config has already
// been processed. If gadget driver returns
// USB_GADGET_DELAYED_STATUS, we will wait
// to change the state on the next usb_ep_queue()
//
    if (ret == 0)
    usb_gadget_set_state(dwc.gadget,
    USB_STATE_CONFIGURED);
//
// Enable transition to U1/U2 state when
// nothing is pending from application.
//
    reg = dwc3_readl(dwc, DWC3_DCTL);
    if (!dwc.dis_u1_entry_quirk)
    reg |= DWC3_DCTL_ACCEPTU1ENA;
    if (!dwc.dis_u2_entry_quirk)
    reg |= DWC3_DCTL_ACCEPTU2ENA;
    dwc3_writel(dwc, DWC3_DCTL, reg);
    }
    break;
    case USB_STATE_CONFIGURED:
    ret = dwc3_ep0_delegate_req(dwc, ctrl);
    if (!cfg && !ret)
    usb_gadget_set_state(dwc.gadget,
    USB_STATE_ADDRESS);
    break;
    default:
    ret = -EINVAL;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn dwc3_ep0_set_sel_cmpl(ep: *mut usb_ep, req: *mut usb_request) {
    static void dwc3_ep0_set_sel_cmpl(struct usb_ep *ep, struct usb_request *req)
    {
    struct dwc3_ep	*dep = to_dwc3_ep(ep);
    struct dwc3	*dwc = dep.dwc;
    let mut param: u32 = 0;
    u32		reg;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct timing {
    pub u1sel: u8,
    pub u1pel: u8,
    pub u2sel: __le16,
    pub u2pel: __le16,
    pub timing: } __packed,
    pub ret: c_int,
    pub sizeof(timing)): memcpy(&timing, req->buf,,
    pub timing.u1sel: dwc->u1sel =,
    pub timing.u1pel: dwc->u1pel =,
    pub le16_to_cpu(timing.u2sel): dwc->u2sel =,
    pub le16_to_cpu(timing.u2pel): dwc->u2pel =,
    pub DWC3_DCTL): reg = dwc3_readl(dwc,,
    if (reg & DWC3_DCTL_INITU2ENA)
    pub dwc->u2pel: param =,
    if (reg & DWC3_DCTL_INITU1ENA)
    pub dwc->u1pel: param =,
//
// According to Synopsys Databook, if parameter is
// greater than 125, a value of zero should be
// programmed in the register.
//
    if (param > 125)
    pub 0: param =,
// now that we have the time, issue DGCMD Set Sel
    ret = dwc3_send_gadget_generic_command(dwc,
    pub param): DWC3_DGCMD_SET_PERIODIC_PAR,,
    pub 0): WARN_ON(ret <,
    }
#[no_mangle]
unsafe extern "C" fn dwc3_ep0_set_sel(dwc: *mut dwc3, ctrl: *mut usb_ctrlrequest) -> c_int {
    static int dwc3_ep0_set_sel(struct dwc3 *dwc, struct usb_ctrlrequest *ctrl)
    {
    pub dep: *mut dwc3_ep,
    pub dwc->gadget->state: enum usb_device_state state =,
    pub wLength: u16,
    if (state == USB_STATE_DEFAULT)
    pub -EINVAL: return,
    pub le16_to_cpu(ctrl->wLength): wLength =,
    if (wLength != 6) {
    dev_err(dwc.dev, "Set SEL should be 6 bytes, got %d\n",
    pub -EINVAL: return,
    }
//
// To handle Set SEL we need to receive 6 bytes from Host. So let's
// queue a usb_request for 6 bytes.
//
// Remember, though, this controller can't handle non-wMaxPacketSize
// aligned transfers on the OUT direction, so we queue a request for
// wMaxPacketSize instead.
//
    pub dwc->eps[0]: dep =,
    pub dep: dwc->ep0_usb_req.dep =,
    pub dep->endpoint.maxpacket: dwc->ep0_usb_req.request.length =,
    pub dwc->setup_buf: dwc->ep0_usb_req.request.buf =,
    pub dwc3_ep0_set_sel_cmpl: dwc->ep0_usb_req.request.complete =,
    pub &dwc->ep0_usb_req): return __dwc3_gadget_ep0_queue(dep,,
    }
#[no_mangle]
unsafe extern "C" fn dwc3_ep0_set_isoch_delay(dwc: *mut dwc3, ctrl: *mut usb_ctrlrequest) -> c_int {
    static int dwc3_ep0_set_isoch_delay(struct dwc3 *dwc, struct usb_ctrlrequest *ctrl)
    {
    pub wLength: u16,
    pub wValue: u16,
    pub wIndex: u16,
    pub le16_to_cpu(ctrl->wValue): wValue =,
    pub le16_to_cpu(ctrl->wLength): wLength =,
    pub le16_to_cpu(ctrl->wIndex): wIndex =,
    if (wIndex || wLength)
    pub -EINVAL: return,
    pub wValue: dwc->gadget->isoch_delay =,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn dwc3_ep0_std_request(dwc: *mut dwc3, ctrl: *mut usb_ctrlrequest) -> c_int {
    static int dwc3_ep0_std_request(struct dwc3 *dwc, struct usb_ctrlrequest *ctrl)
    {
    pub ret: c_int,
    switch (ctrl.bRequest) {
    case USB_REQ_GET_STATUS:
    pub ctrl): ret = dwc3_ep0_handle_status(dwc,,
    case USB_REQ_CLEAR_FEATURE:
    pub 0): ret = dwc3_ep0_handle_feature(dwc, ctrl,,
    case USB_REQ_SET_FEATURE:
    pub 1): ret = dwc3_ep0_handle_feature(dwc, ctrl,,
    case USB_REQ_SET_ADDRESS:
    pub ctrl): ret = dwc3_ep0_set_address(dwc,,
    case USB_REQ_SET_CONFIGURATION:
    pub ctrl): ret = dwc3_ep0_set_config(dwc,,
    case USB_REQ_SET_SEL:
    pub ctrl): ret = dwc3_ep0_set_sel(dwc,,
    case USB_REQ_SET_ISOCH_DELAY:
    pub ctrl): ret = dwc3_ep0_set_isoch_delay(dwc,,
    default:
    pub ctrl): ret = dwc3_ep0_delegate_req(dwc,,
    }
    pub ret: return,
    }
    static void dwc3_ep0_inspect_setup(struct dwc3 *dwc,
    const struct dwc3_event_depevt *event)
    {
    pub dwc->ep0_trb: *mut *mut *mut usb_ctrlrequest ctrl = (void ),
    pub -EINVAL: int ret =,
    pub len: u32,
    if (!dwc.gadget_driver || !dwc.softconnect || !dwc.connected)
    pub out: goto,
    pub ctrl): trace_dwc3_ctrl_req(dwc,,
    pub le16_to_cpu(ctrl->wLength): len =,
    if (!len) {
    pub false: dwc->three_stage_setup =,
    pub false: dwc->ep0_expect_in =,
    pub DWC3_EP0_NRDY_STATUS: dwc->ep0_next_event =,
    } else {
    pub true: dwc->three_stage_setup =,
    pub USB_DIR_IN): dwc->ep0_expect_in = !!(ctrl->bRequestType &,
    pub DWC3_EP0_NRDY_DATA: dwc->ep0_next_event =,
    }
    if ((ctrl.bRequestType & USB_TYPE_MASK) == USB_TYPE_STANDARD)
    pub ctrl): ret = dwc3_ep0_std_request(dwc,,
    else
    pub ctrl): ret = dwc3_ep0_delegate_req(dwc,,
    if (ret == USB_GADGET_DELAYED_STATUS)
    pub true: dwc->delayed_status =,
    out:
    if (ret < 0)
    }
    static void dwc3_ep0_complete_data(struct dwc3 *dwc,
    const struct dwc3_event_depevt *event)
    {
    pub r: *mut dwc3_request,
    pub ur: *mut usb_request,
    pub trb: *mut dwc3_trb,
    pub ep0: *mut dwc3_ep,
    pub 0: u32 transferred =,
    pub status: u32,
    pub length: u32,
    pub epnum: u8,
    pub event->endpoint_number: epnum =,
    pub dwc->eps[0]: ep0 =,
    pub DWC3_EP0_NRDY_STATUS: dwc->ep0_next_event =,
    pub dwc->ep0_trb: trb =,
    pub trb): trace_dwc3_complete_trb(ep0,,
    pub next_request(&ep0->pending_list): r =,
    if (!r)
    pub DWC3_TRB_SIZE_TRBSTS(trb->size): status =,
    if (status == DWC3_TRBSTS_SETUP_PENDING) {
    pub true: dwc->setup_packet_pending =,
    if (r)
    pub -ECONNRESET): dwc3_gadget_giveback(ep0, r,,
    }
    pub &r->request: ur =,
    pub DWC3_TRB_SIZE_MASK: length = trb->size &,
    pub length: transferred = ur->length -,
    pub transferred: ur->actual +=,
    if ((IS_ALIGNED(ur.length, ep0.endpoint.maxpacket) &&
    ur.length && ur.zero) || dwc.ep0_bounced) {
    pub ~DWC3_TRB_CTRL_HWO: trb->ctrl &=,
    pub trb): trace_dwc3_complete_trb(ep0,,
    if (r.direction)
    pub 0: dwc->eps[1]->trb_enqueue =,
    else
    pub 0: dwc->eps[0]->trb_enqueue =,
    pub false: dwc->ep0_bounced =,
    }
    if ((epnum & 1) && ur.actual < ur.length)
    else
    pub 0): dwc3_gadget_giveback(ep0, r,,
    }
    static void dwc3_ep0_complete_status(struct dwc3 *dwc,
    const struct dwc3_event_depevt *event)
    {
    pub r: *mut dwc3_request,
    pub dep: *mut dwc3_ep,
    pub trb: *mut dwc3_trb,
    pub status: u32,
    pub dwc->eps[0]: dep =,
    pub dwc->ep0_trb: trb =,
    pub trb): trace_dwc3_complete_trb(dep,,
    if (!list_empty(&dep.pending_list)) {
    pub next_request(&dep->pending_list): r =,
    pub 0): dwc3_gadget_giveback(dep, r,,
    }
    if (dwc.test_mode) {
    pub ret: c_int,
    pub dwc->test_mode_nr): ret = dwc3_gadget_set_test_mode(dwc,,
    if (ret < 0) {
    dev_err(dwc.dev, "invalid test #%d\n",
    }
    }
    pub DWC3_TRB_SIZE_TRBSTS(trb->size): status =,
    if (status == DWC3_TRBSTS_SETUP_PENDING)
    pub true: dwc->setup_packet_pending =,
    pub EP0_SETUP_PHASE: dwc->ep0state =,
    }
    static void dwc3_ep0_xfer_complete(struct dwc3 *dwc,
    const struct dwc3_event_depevt *event)
    {
    pub dwc->eps[event->endpoint_number]: *mut *mut dwc3_ep dep =,
    pub ~DWC3_EP_TRANSFER_STARTED: dep->flags &=,
    pub 0: dep->resource_index =,
    pub false: dwc->setup_packet_pending =,
    switch (dwc.ep0state) {
    case EP0_SETUP_PHASE:
    pub event): dwc3_ep0_inspect_setup(dwc,,
    case EP0_DATA_PHASE:
    pub event): dwc3_ep0_complete_data(dwc,,
    case EP0_STATUS_PHASE:
    pub event): dwc3_ep0_complete_status(dwc,,
    default:
    pub dwc->ep0state): WARN(true, "UNKNOWN ep0state %d\n",,
    }
    }
    static void __dwc3_ep0_do_control_data(struct dwc3 *dwc,
    struct dwc3_ep *dep, struct dwc3_request *req)
    {
    pub 0: unsigned int trb_length =,
    pub ret: c_int,
    pub !!dep->number: req->direction =,
    if (req.request.length == 0) {
    if (!req.direction)
    pub dep->endpoint.maxpacket: trb_length =,
    dwc3_ep0_prepare_one_trb(dep, dwc.bounce_addr, trb_length,
    pub false): DWC3_TRBCTL_CONTROL_DATA,,
    pub dwc3_ep0_start_trans(dep): ret =,
    } else if (!IS_ALIGNED(req.request.length, dep.endpoint.maxpacket)
    && (dep.number == 0)) {
    pub maxpacket: u32,
    pub rem: u32,
    ret = usb_gadget_map_request_by_dev(dwc.sysdev,
    pub dep->number): &req->request,,
    if (ret)
    pub dep->endpoint.maxpacket: maxpacket =,
    pub maxpacket: rem = req->request.length %,
    pub true: dwc->ep0_bounced =,
// prepare normal TRB
    dwc3_ep0_prepare_one_trb(dep, req.request.dma,
    req.request.length,
    DWC3_TRBCTL_CONTROL_DATA,
    pub 1]: req->trb = &dwc->ep0_trb[dep->trb_enqueue -,
// Now prepare one extra TRB to align transfer size
    dwc3_ep0_prepare_one_trb(dep, dwc.bounce_addr,
    maxpacket - rem,
    DWC3_TRBCTL_CONTROL_DATA,
    pub dwc3_ep0_start_trans(dep): ret =,
    } else if (IS_ALIGNED(req.request.length, dep.endpoint.maxpacket) &&
    req.request.length && req.request.zero) {
    ret = usb_gadget_map_request_by_dev(dwc.sysdev,
    pub dep->number): &req->request,,
    if (ret)
// prepare normal TRB
    dwc3_ep0_prepare_one_trb(dep, req.request.dma,
    req.request.length,
    DWC3_TRBCTL_CONTROL_DATA,
    pub 1]: req->trb = &dwc->ep0_trb[dep->trb_enqueue -,
    if (!req.direction)
    pub dep->endpoint.maxpacket: trb_length =,
// Now prepare one extra TRB to align transfer size
    dwc3_ep0_prepare_one_trb(dep, dwc.bounce_addr,
    trb_length, DWC3_TRBCTL_CONTROL_DATA,
    pub dwc3_ep0_start_trans(dep): ret =,
    } else {
    ret = usb_gadget_map_request_by_dev(dwc.sysdev,
    pub dep->number): &req->request,,
    if (ret)
    dwc3_ep0_prepare_one_trb(dep, req.request.dma,
    req.request.length, DWC3_TRBCTL_CONTROL_DATA,
    pub &dwc->ep0_trb[dep->trb_enqueue]: req->trb =,
    pub dwc3_ep0_start_trans(dep): ret =,
    }
    if (ret < 0)
    dev_err(dwc.dev,
    pub ret): "ep0 data phase start transfer failed: %d\n",,
    }
#[no_mangle]
unsafe extern "C" fn dwc3_ep0_start_control_status(dep: *mut dwc3_ep) -> c_int {
    static int dwc3_ep0_start_control_status(struct dwc3_ep *dep)
    {
    pub dep->dwc: *mut *mut dwc3 dwc =,
    pub type: u32,
    type = dwc.three_stage_setup ? DWC3_TRBCTL_CONTROL_STATUS3
    pub DWC3_TRBCTL_CONTROL_STATUS2: :,
    pub false): dwc3_ep0_prepare_one_trb(dep, dwc->ep0_trb_addr, 0, type,,
    pub dwc3_ep0_start_trans(dep): return,
    }
#[no_mangle]
unsafe extern "C" fn __dwc3_ep0_do_control_status(dwc: *mut dwc3, dep: *mut dwc3_ep) {
    static void __dwc3_ep0_do_control_status(struct dwc3 *dwc, struct dwc3_ep *dep)
    {
    pub ret: c_int,
    pub dwc3_ep0_start_control_status(dep): ret =,
    if (ret)
    dev_err(dwc.dev,
    pub ret): "ep0 status phase start transfer failed: %d\n",,
    }
    static void dwc3_ep0_do_control_status(struct dwc3 *dwc,
    const struct dwc3_event_depevt *event)
    {
    pub dwc->eps[event->endpoint_number]: *mut *mut dwc3_ep dep =,
    pub dep): __dwc3_ep0_do_control_status(dwc,,
    }
#[no_mangle]
pub unsafe extern "C" fn dwc3_ep0_send_delayed_status(dwc: *mut dwc3) {
    void dwc3_ep0_send_delayed_status(struct dwc3 *dwc)
    {
    pub !dwc->ep0_expect_in: unsigned int direction =,
    pub false: dwc->delayed_status =,
    pub 0: dwc->clear_stall_protocol =,
    if (dwc.ep0state != EP0_STATUS_PHASE)
    pub dwc->eps[direction]): __dwc3_ep0_do_control_status(dwc,,
    }
#[no_mangle]
pub unsafe extern "C" fn dwc3_ep0_end_control_data(dwc: *mut dwc3, dep: *mut dwc3_ep) {
    void dwc3_ep0_end_control_data(struct dwc3 *dwc, struct dwc3_ep *dep)
    {
    pub params: dwc3_gadget_ep_cmd_params,
    pub cmd: u32,
    pub ret: c_int,
//
// For status/DATA OUT stage, TRB will be queued on ep0 out
// endpoint for which resource index is zero. Hence allow
// queuing ENDXFER command for ep0 out endpoint.
//
    if (!dep.resource_index && dep.number)
    pub DWC3_DEPCMD_ENDTRANSFER: cmd =,
    pub DWC3_DEPCMD_CMDIOC: cmd |=,
    pub DWC3_DEPCMD_PARAM(dep->resource_index): cmd |=,
    pub sizeof(params)): memset(&params, 0,,
    pub &params): ret = dwc3_send_gadget_ep_cmd(dep, cmd,,
    if (ret)
    dev_err_ratelimited(dwc.dev,
    pub ret): "ep0 data phase end transfer failed: %d\n",,
    pub 0: dep->resource_index =,
    }
    static void dwc3_ep0_xfernotready(struct dwc3 *dwc,
    const struct dwc3_event_depevt *event)
    {
    switch (event.status) {
    case DEPEVT_STATUS_CONTROL_DATA:
    if (!dwc.softconnect || !dwc.connected)
//
// We already have a DATA transfer in the controller's cache,
// if we receive a XferNotReady(DATA) we will ignore it, unless
// it's for the wrong direction.
//
// In that case, we must issue END_TRANSFER command to the Data
// Phase we already have started and issue SetStall on the
// control endpoint.
//
    if (dwc.ep0_expect_in != event.endpoint_number) {
    pub dwc->eps[dwc->ep0_expect_in]: *mut *mut dwc3_ep dep =,
    pub Phase\n"): dev_err(dwc->dev, "unexpected direction for Data,
    pub dep): dwc3_ep0_end_control_data(dwc,,
    }
    case DEPEVT_STATUS_CONTROL_STATUS:
    if (dwc.ep0_next_event != DWC3_EP0_NRDY_STATUS)
    if (dwc.setup_packet_pending) {
    }
    pub EP0_STATUS_PHASE: dwc->ep0state =,
    if (dwc.delayed_status) {
    pub dwc->eps[0]: *mut *mut dwc3_ep dep =,
    pub 1): WARN_ON_ONCE(event->endpoint_number !=,
//
// We should handle the delay STATUS phase here if the
// request for handling delay STATUS has been queued
// into the list.
//
    if (!list_empty(&dep.pending_list)) {
    pub false: dwc->delayed_status =,
    usb_gadget_set_state(dwc.gadget,
    pub event): dwc3_ep0_do_control_status(dwc,,
    }
    }
    pub event): dwc3_ep0_do_control_status(dwc,,
    }
    }
    void dwc3_ep0_interrupt(struct dwc3 *dwc,
    const struct dwc3_event_depevt *event)
    {
    pub dwc->eps[event->endpoint_number]: *mut *mut dwc3_ep dep =,
    pub cmd: u8,
    switch (event.endpoint_event) {
    case DWC3_DEPEVT_XFERCOMPLETE:
    pub event): dwc3_ep0_xfer_complete(dwc,,
    case DWC3_DEPEVT_XFERNOTREADY:
    pub event): dwc3_ep0_xfernotready(dwc,,
    case DWC3_DEPEVT_XFERINPROGRESS:
    case DWC3_DEPEVT_RXTXFIFOEVT:
    case DWC3_DEPEVT_STREAMEVT:
    case DWC3_DEPEVT_EPCMDCMPLT:
    pub DEPEVT_PARAMETER_CMD(event->parameters): cmd =,
    if (cmd == DWC3_DEPCMD_ENDTRANSFER) {
    pub ~DWC3_EP_END_TRANSFER_PENDING: dep->flags &=,
    pub ~DWC3_EP_TRANSFER_STARTED: dep->flags &=,
    }
    default:
    pub event->endpoint_event): dev_err(dwc->dev, "unknown endpoint event %d\n",,
    }
    }
