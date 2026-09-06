//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/sysdev/fsl_rmu.c
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
// Freescale MPC85xx/MPC86xx RapidIO RMU support
//
// Copyright 2009 Sysgo AG
// Thomas Moll <thomas.moll@sysgo.com>
// - fixed maintenance access routines, check for aligned access
//
// Copyright 2009 Integrated Device Technology, Inc.
// Alex Bounine <alexandre.bounine@idt.com>
// - Added Port-Write message handling
// - Added Machine Check exception handling
//
// Copyright (C) 2007, 2008, 2010, 2011 Freescale Semiconductor, Inc.
// Zhang Wei <wei.zhang@freescale.com>
// Lian Minghuan-B31939 <Minghuan.Lian@freescale.com>
// Liu Gang <Gang.Liu@freescale.com>
//
// Copyright 2005 MontaVista Software, Inc.
// Matt Porter <mporter@kernel.crashing.org>
//

    (((struct rio_priv *)(mport.priv)).rmm_handle)
// RapidIO definition irq, which read from OF-tree

pub const RIO_MIN_TX_RING_SIZE: c_int = 2;
pub const RIO_MAX_TX_RING_SIZE: c_int = 2048;
pub const RIO_MIN_RX_RING_SIZE: c_int = 2;
pub const RIO_MAX_RX_RING_SIZE: c_int = 2048;
pub const RIO_IPWMR_SEN: c_uint = 0x00100000;
pub const RIO_IPWMR_QFIE: c_uint = 0x00000100;
pub const RIO_IPWMR_EIE: c_uint = 0x00000020;
pub const RIO_IPWMR_CQ: c_uint = 0x00000002;
pub const RIO_IPWMR_PWE: c_uint = 0x00000001;
pub const RIO_IPWSR_QF: c_uint = 0x00100000;
pub const RIO_IPWSR_TE: c_uint = 0x00000080;
pub const RIO_IPWSR_QFI: c_uint = 0x00000010;
pub const RIO_IPWSR_PWD: c_uint = 0x00000008;
pub const RIO_IPWSR_PWB: c_uint = 0x00000004;
pub const RIO_EPWISR: c_uint = 0x10010;
// EPWISR Error match value
pub const RIO_EPWISR_PINT1: c_uint = 0x80000000;
pub const RIO_EPWISR_PINT2: c_uint = 0x40000000;
pub const RIO_EPWISR_MU: c_uint = 0x00000002;
pub const RIO_EPWISR_PW: c_uint = 0x00000001;
pub const IPWSR_CLEAR: c_uint = 0x98;
pub const OMSR_CLEAR: c_uint = 0x1cb3;
pub const IMSR_CLEAR: c_uint = 0x491;
pub const IDSR_CLEAR: c_uint = 0x91;
pub const ODSR_CLEAR: c_uint = 0x1c00;
pub const LTLEECSR_ENABLE_ALL: c_uint = 0xFFC000FC;
pub const RIO_LTLEECSR: c_uint = 0x060c;
pub const RIO_IM0SR: c_uint = 0x64;
pub const RIO_IM1SR: c_uint = 0x164;
pub const RIO_OM0SR: c_uint = 0x4;
pub const RIO_OM1SR: c_uint = 0x104;
pub const RIO_DBELL_WIN_SIZE: c_uint = 0x1000;
pub const RIO_MSG_OMR_MUI: c_uint = 0x00000002;
pub const RIO_MSG_OSR_TE: c_uint = 0x00000080;
pub const RIO_MSG_OSR_QOI: c_uint = 0x00000020;
pub const RIO_MSG_OSR_QFI: c_uint = 0x00000010;
pub const RIO_MSG_OSR_MUB: c_uint = 0x00000004;
pub const RIO_MSG_OSR_EOMI: c_uint = 0x00000002;
pub const RIO_MSG_OSR_QEI: c_uint = 0x00000001;
pub const RIO_MSG_IMR_MI: c_uint = 0x00000002;
pub const RIO_MSG_ISR_TE: c_uint = 0x00000080;
pub const RIO_MSG_ISR_QFI: c_uint = 0x00000010;
pub const RIO_MSG_ISR_DIQI: c_uint = 0x00000001;
pub const RIO_MSG_DESC_SIZE: c_int = 32;
pub const RIO_MSG_BUFFER_SIZE: c_int = 4096;
pub const DOORBELL_DMR_DI: c_uint = 0x00000002;
pub const DOORBELL_DSR_TE: c_uint = 0x00000080;
pub const DOORBELL_DSR_QFI: c_uint = 0x00000010;
pub const DOORBELL_DSR_DIQI: c_uint = 0x00000001;
pub const DOORBELL_MESSAGE_SIZE: c_uint = 0x08;
    static DEFINE_SPINLOCK(fsl_rio_doorbell_lock);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rio_msg_regs {
    pub omr: u32,
    pub osr: u32,
    pub pad1: u32,
    pub odqdpar: u32,
    pub pad2: u32,
    pub osar: u32,
    pub odpr: u32,
    pub odatr: u32,
    pub odcr: u32,
    pub pad3: u32,
    pub odqepar: u32,
    pub pad4: [u32; 13],
    pub imr: u32,
    pub isr: u32,
    pub pad5: u32,
    pub ifqdpar: u32,
    pub pad6: u32,
    pub ifqepar: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rio_dbell_regs {
    pub odmr: u32,
    pub odsr: u32,
    pub pad1: [u32; 4],
    pub oddpr: u32,
    pub oddatr: u32,
    pub pad2: [u32; 3],
    pub odretcr: u32,
    pub pad3: [u32; 12],
    pub dmr: u32,
    pub dsr: u32,
    pub pad4: u32,
    pub dqdpar: u32,
    pub pad5: u32,
    pub dqepar: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rio_pw_regs {
    pub pwmr: u32,
    pub pwsr: u32,
    pub epwqbar: u32,
    pub pwqbar: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rio_tx_desc {
    pub pad1: u32,
    pub saddr: u32,
    pub dport: u32,
    pub dattr: u32,
    pub pad2: u32,
    pub pad3: u32,
    pub dwcnt: u32,
    pub pad4: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rio_msg_tx_ring {
    pub virt: *mut c_void,
    pub phys: dma_addr_t,
    pub virt_buffer: [*mut c_void; RIO_MAX_TX_RING_SIZE],
    pub phys_buffer: [dma_addr_t; RIO_MAX_TX_RING_SIZE],
    pub tx_slot: c_int,
    pub size: c_int,
    pub dev_id: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rio_msg_rx_ring {
    pub virt: *mut c_void,
    pub phys: dma_addr_t,
    pub virt_buffer: [*mut c_void; RIO_MAX_RX_RING_SIZE],
    pub rx_slot: c_int,
    pub size: c_int,
    pub dev_id: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_rmu {
    pub msg_regs: *mut rio_msg_regs __iomem,
    pub msg_tx_ring: rio_msg_tx_ring,
    pub msg_rx_ring: rio_msg_rx_ring,
    pub txirq: c_int,
    pub rxirq: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rio_dbell_msg {
    pub pad1: u16,
    pub tid: u16,
    pub sid: u16,
    pub info: u16,
}

//
// fsl_rio_tx_handler - MPC85xx outbound message interrupt handler
// @irq: Linux interrupt number
// @dev_instance: Pointer to interrupt-specific data
//
// Handles outbound message interrupts. Executes a register outbound
// mailbox event handler and acks the interrupt occurrence.
//
    static irqreturn_t
    fsl_rio_tx_handler(int irq, void *dev_instance)
    {
    int osr;
    struct rio_mport *port = (struct rio_mport *)dev_instance;
    struct fsl_rmu *rmu = GET_RMM_HANDLE(port);
    osr = in_be32(&rmu.msg_regs.osr);
    if (osr & RIO_MSG_OSR_TE) {
    pr_info("RIO: outbound message transmission error\n");
    out_be32(&rmu.msg_regs.osr, RIO_MSG_OSR_TE);
    goto out;
    }
    if (osr & RIO_MSG_OSR_QOI) {
    pr_info("RIO: outbound message queue overflow\n");
    out_be32(&rmu.msg_regs.osr, RIO_MSG_OSR_QOI);
    goto out;
    }
    if (osr & RIO_MSG_OSR_EOMI) {
    let mut dqp: u32 = in_be32(&rmu.msg_regs.odqdpar);
    let mut slot: c_int = (dqp - rmu.msg_tx_ring.phys) >> 5;
    if (port.outb_msg[0].mcback != core::ptr::null_mut()) {
    port.outb_msg[0].mcback(port, rmu.msg_tx_ring.dev_id,
    -1,
    slot);
    }
// Ack the end-of-message interrupt
    out_be32(&rmu.msg_regs.osr, RIO_MSG_OSR_EOMI);
    }
    out:
    return IRQ_HANDLED;
    }
//
// fsl_rio_rx_handler - MPC85xx inbound message interrupt handler
// @irq: Linux interrupt number
// @dev_instance: Pointer to interrupt-specific data
//
// Handles inbound message interrupts. Executes a registered inbound
// mailbox event handler and acks the interrupt occurrence.
//
    static irqreturn_t
    fsl_rio_rx_handler(int irq, void *dev_instance)
    {
    int isr;
    struct rio_mport *port = (struct rio_mport *)dev_instance;
    struct fsl_rmu *rmu = GET_RMM_HANDLE(port);
    isr = in_be32(&rmu.msg_regs.isr);
    if (isr & RIO_MSG_ISR_TE) {
    pr_info("RIO: inbound message reception error\n");
    out_be32((void *)&rmu.msg_regs.isr, RIO_MSG_ISR_TE);
    goto out;
    }
// XXX Need to check/dispatch until queue empty
    if (isr & RIO_MSG_ISR_DIQI) {
//
// Can receive messages for any mailbox/letter to that
// mailbox destination. So, make the callback with an
// unknown/invalid mailbox number argument.
//
    if (port.inb_msg[0].mcback != core::ptr::null_mut())
    port.inb_msg[0].mcback(port, rmu.msg_rx_ring.dev_id,
    -1,
    -1);
// Ack the queueing interrupt
    out_be32(&rmu.msg_regs.isr, RIO_MSG_ISR_DIQI);
    }
    out:
    return IRQ_HANDLED;
    }
//
// fsl_rio_dbell_handler - MPC85xx doorbell interrupt handler
// @irq: Linux interrupt number
// @dev_instance: Pointer to interrupt-specific data
//
// Handles doorbell interrupts. Parses a list of registered
// doorbell event handlers and executes a matching event handler.
//
    static irqreturn_t
    fsl_rio_dbell_handler(int irq, void *dev_instance)
    {
    int dsr;
    struct fsl_rio_dbell *fsl_dbell = (struct fsl_rio_dbell *)dev_instance;
    int i;
    dsr = in_be32(&fsl_dbell.dbell_regs.dsr);
    if (dsr & DOORBELL_DSR_TE) {
    pr_info("RIO: doorbell reception error\n");
    out_be32(&fsl_dbell.dbell_regs.dsr, DOORBELL_DSR_TE);
    goto out;
    }
    if (dsr & DOORBELL_DSR_QFI) {
    pr_info("RIO: doorbell queue full\n");
    out_be32(&fsl_dbell.dbell_regs.dsr, DOORBELL_DSR_QFI);
    }
// XXX Need to check/dispatch until queue empty
    if (dsr & DOORBELL_DSR_DIQI) {
    struct rio_dbell_msg *dmsg =
    fsl_dbell.dbell_ring.virt +
    (in_be32(&fsl_dbell.dbell_regs.dqdpar) & 0xfff);
    struct rio_dbell *dbell;
    let mut found: c_int = 0;
    pr_debug
    ("RIO: processing doorbell,"
    " sid %2.2x tid %2.2x info %4.4x\n",
    dmsg.sid, dmsg.tid, dmsg.info);
    for (i = 0; i < MAX_PORT_NUM; i++) {
    if (fsl_dbell.mport[i]) {
    list_for_each_entry(dbell,
    &fsl_dbell.mport[i].dbells, node) {
    if ((dbell.res.start
    <= dmsg.info)
    && (dbell.res.end
    >= dmsg.info)) {
    found = 1;
    break;
    }
    }
    if (found && dbell.dinb) {
    dbell.dinb(fsl_dbell.mport[i],
    dbell.dev_id, dmsg.sid,
    dmsg.tid,
    dmsg.info);
    break;
    }
    }
    }
    if (!found) {
    pr_debug
    ("RIO: spurious doorbell,"
    " sid %2.2x tid %2.2x info %4.4x\n",
    dmsg.sid, dmsg.tid,
    dmsg.info);
    }
    setbits32(&fsl_dbell.dbell_regs.dmr, DOORBELL_DMR_DI);
    out_be32(&fsl_dbell.dbell_regs.dsr, DOORBELL_DSR_DIQI);
    }
    out:
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn msg_unit_error_handler() {
    static void msg_unit_error_handler(void)
    {
// XXX: Error recovery is not implemented, we just clear errors
    out_be32((u32 *)(rio_regs_win + RIO_LTLEDCSR), 0);
    out_be32((u32 *)(rmu_regs_win + RIO_IM0SR), IMSR_CLEAR);
    out_be32((u32 *)(rmu_regs_win + RIO_IM1SR), IMSR_CLEAR);
    out_be32((u32 *)(rmu_regs_win + RIO_OM0SR), OMSR_CLEAR);
    out_be32((u32 *)(rmu_regs_win + RIO_OM1SR), OMSR_CLEAR);
    out_be32(&dbell.dbell_regs.odsr, ODSR_CLEAR);
    out_be32(&dbell.dbell_regs.dsr, IDSR_CLEAR);
    out_be32(&pw.pw_regs.pwsr, IPWSR_CLEAR);
    }
//
// fsl_rio_port_write_handler - MPC85xx port write interrupt handler
// @irq: Linux interrupt number
// @dev_instance: Pointer to interrupt-specific data
//
// Handles port write interrupts. Parses a list of registered
// port write event handlers and executes a matching event handler.
//
    static irqreturn_t
    fsl_rio_port_write_handler(int irq, void *dev_instance)
    {
    u32 ipwmr, ipwsr;
    struct fsl_rio_pw *pw = (struct fsl_rio_pw *)dev_instance;
    u32 epwisr, tmp;
    epwisr = in_be32(rio_regs_win + RIO_EPWISR);
    if (!(epwisr & RIO_EPWISR_PW))
    goto pw_done;
    ipwmr = in_be32(&pw.pw_regs.pwmr);
    ipwsr = in_be32(&pw.pw_regs.pwsr);

    pr_debug("PW Int.IPWMR: 0x%08x IPWSR: 0x%08x (", ipwmr, ipwsr);
    if (ipwsr & RIO_IPWSR_QF)
    pr_debug(" QF");
    if (ipwsr & RIO_IPWSR_TE)
    pr_debug(" TE");
    if (ipwsr & RIO_IPWSR_QFI)
    pr_debug(" QFI");
    if (ipwsr & RIO_IPWSR_PWD)
    pr_debug(" PWD");
    if (ipwsr & RIO_IPWSR_PWB)
    pr_debug(" PWB");
    pr_debug(" )\n");

// Schedule deferred processing if PW was received
    if (ipwsr & RIO_IPWSR_QFI) {
// Save PW message (if there is room in FIFO),
// otherwise discard it.
//
    if (kfifo_avail(&pw.pw_fifo) >= RIO_PW_MSG_SIZE) {
    pw.port_write_msg.msg_count++;
    kfifo_in(&pw.pw_fifo, pw.port_write_msg.virt,
    RIO_PW_MSG_SIZE);
    } else {
    pw.port_write_msg.discard_count++;
    pr_debug("RIO: ISR Discarded Port-Write Msg(s) (%d)\n",
    pw.port_write_msg.discard_count);
    }
// Clear interrupt and issue Clear Queue command. This allows
// another port-write to be received.
//
    out_be32(&pw.pw_regs.pwsr,	RIO_IPWSR_QFI);
    out_be32(&pw.pw_regs.pwmr, ipwmr | RIO_IPWMR_CQ);
    schedule_work(&pw.pw_work);
    }
    if ((ipwmr & RIO_IPWMR_EIE) && (ipwsr & RIO_IPWSR_TE)) {
    pw.port_write_msg.err_count++;
    pr_debug("RIO: Port-Write Transaction Err (%d)\n",
    pw.port_write_msg.err_count);
// Clear Transaction Error: port-write controller should be
// disabled when clearing this error
//
    out_be32(&pw.pw_regs.pwmr, ipwmr & ~RIO_IPWMR_PWE);
    out_be32(&pw.pw_regs.pwsr,	RIO_IPWSR_TE);
    out_be32(&pw.pw_regs.pwmr, ipwmr);
    }
    if (ipwsr & RIO_IPWSR_PWD) {
    pw.port_write_msg.discard_count++;
    pr_debug("RIO: Port Discarded Port-Write Msg(s) (%d)\n",
    pw.port_write_msg.discard_count);
    out_be32(&pw.pw_regs.pwsr, RIO_IPWSR_PWD);
    }
    pw_done:
    if (epwisr & RIO_EPWISR_PINT1) {
    tmp = in_be32(rio_regs_win + RIO_LTLEDCSR);
    pr_debug("RIO_LTLEDCSR = 0x%x\n", tmp);
    fsl_rio_port_error_handler(0);
    }
    if (epwisr & RIO_EPWISR_PINT2) {
    tmp = in_be32(rio_regs_win + RIO_LTLEDCSR);
    pr_debug("RIO_LTLEDCSR = 0x%x\n", tmp);
    fsl_rio_port_error_handler(1);
    }
    if (epwisr & RIO_EPWISR_MU) {
    tmp = in_be32(rio_regs_win + RIO_LTLEDCSR);
    pr_debug("RIO_LTLEDCSR = 0x%x\n", tmp);
    msg_unit_error_handler();
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn fsl_pw_dpc(work: *mut work_struct) {
    static void fsl_pw_dpc(struct work_struct *work)
    {
    struct fsl_rio_pw *pw = container_of(work, struct fsl_rio_pw, pw_work);
    union rio_pw_msg msg_buffer;
    int i;
//
// Process port-write messages
//
    while (kfifo_out_spinlocked(&pw.pw_fifo, (unsigned char *)&msg_buffer,
    RIO_PW_MSG_SIZE, &pw.pw_fifo_lock)) {

    {
    u32 i;
    pr_debug("%s : Port-Write Message:", __func__);
    for (i = 0; i < RIO_PW_MSG_SIZE/sizeof(u32); i++) {
    if ((i%4) == 0)
    pr_debug("\n0x%02x: 0x%08x", i*4,
    msg_buffer.raw[i]);
    else
    pr_debug(" 0x%08x", msg_buffer.raw[i]);
    }
    pr_debug("\n");
    }

// Pass the port-write message to RIO core for processing
    for (i = 0; i < MAX_PORT_NUM; i++) {
    if (pw.mport[i])
    rio_inb_pwrite_handler(pw.mport[i],
    &msg_buffer);
    }
    }
    }
//
// fsl_rio_pw_enable - enable/disable port-write interface init
// @mport: Master port implementing the port write unit
// @enable:    1=enable; 0=disable port-write message handling
//
#[no_mangle]
pub unsafe extern "C" fn fsl_rio_pw_enable(mport: *mut rio_mport, enable: c_int) -> c_int {
    int fsl_rio_pw_enable(struct rio_mport *mport, int enable)
    {
    u32 rval;
    rval = in_be32(&pw.pw_regs.pwmr);
    if (enable)
    rval |= RIO_IPWMR_PWE;
    else
    rval &= ~RIO_IPWMR_PWE;
    out_be32(&pw.pw_regs.pwmr, rval);
    return 0;
    }
//
// fsl_rio_port_write_init - MPC85xx port write interface init
// @mport: Master port implementing the port write unit
//
// Initializes port write unit hardware and DMA buffer
// ring. Called from fsl_rio_setup(). Returns %0 on success
// or %-ENOMEM on failure.
//
#[no_mangle]
pub unsafe extern "C" fn fsl_rio_port_write_init(pw: *mut fsl_rio_pw) -> c_int {
    int fsl_rio_port_write_init(struct fsl_rio_pw *pw)
    {
    let mut rc: c_int = 0;
// Following configurations require a disabled port write controller
    out_be32(&pw.pw_regs.pwmr,
    in_be32(&pw.pw_regs.pwmr) & ~RIO_IPWMR_PWE);
// Initialize port write
    pw.port_write_msg.virt = dma_alloc_coherent(pw.dev,
    RIO_PW_MSG_SIZE,
    &pw.port_write_msg.phys, GFP_KERNEL);
    if (!pw.port_write_msg.virt) {
    pr_err("RIO: unable allocate port write queue\n");
    return -ENOMEM;
    }
    pw.port_write_msg.err_count = 0;
    pw.port_write_msg.discard_count = 0;
// Point dequeue/enqueue pointers at first entry
    out_be32(&pw.pw_regs.epwqbar, 0);
    out_be32(&pw.pw_regs.pwqbar, (u32) pw.port_write_msg.phys);
    pr_debug("EIPWQBAR: 0x%08x IPWQBAR: 0x%08x\n",
    in_be32(&pw.pw_regs.epwqbar),
    in_be32(&pw.pw_regs.pwqbar));
// Clear interrupt status IPWSR
    out_be32(&pw.pw_regs.pwsr,
    (RIO_IPWSR_TE | RIO_IPWSR_QFI | RIO_IPWSR_PWD));
// Configure port write controller for snooping enable all reporting,
    clear queue full */
    out_be32(&pw.pw_regs.pwmr,
    RIO_IPWMR_SEN | RIO_IPWMR_QFIE | RIO_IPWMR_EIE | RIO_IPWMR_CQ);
// Hook up port-write handler
    rc = request_irq(IRQ_RIO_PW(pw), fsl_rio_port_write_handler,
    IRQF_SHARED, "port-write", (void *)pw);
    if (rc < 0) {
    pr_err("MPC85xx RIO: unable to request inbound doorbell irq");
    goto err_out;
    }
// Enable Error Interrupt
    out_be32((u32 *)(rio_regs_win + RIO_LTLEECSR), LTLEECSR_ENABLE_ALL);
    INIT_WORK(&pw.pw_work, fsl_pw_dpc);
    spin_lock_init(&pw.pw_fifo_lock);
    if (kfifo_alloc(&pw.pw_fifo, RIO_PW_MSG_SIZE * 32, GFP_KERNEL)) {
    pr_err("FIFO allocation failed\n");
    rc = -ENOMEM;
    goto err_out_irq;
    }
    pr_debug("IPWMR: 0x%08x IPWSR: 0x%08x\n",
    in_be32(&pw.pw_regs.pwmr),
    in_be32(&pw.pw_regs.pwsr));
    return rc;
    err_out_irq:
    free_irq(IRQ_RIO_PW(pw), (void *)pw);
    err_out:
    dma_free_coherent(pw.dev, RIO_PW_MSG_SIZE,
    pw.port_write_msg.virt,
    pw.port_write_msg.phys);
    return rc;
    }
//
// fsl_rio_doorbell_send - Send a MPC85xx doorbell message
// @mport: RapidIO master port info
// @index: ID of RapidIO interface
// @destid: Destination ID of target device
// @data: 16-bit info field of RapidIO doorbell message
//
// Sends a MPC85xx doorbell message. Returns %0 on success or
// %-EINVAL on failure.
//
    int fsl_rio_doorbell_send(struct rio_mport *mport,
    int index, u16 destid, u16 data)
    {
    unsigned long flags;
    pr_debug("fsl_doorbell_send: index %d destid %4.4x data %4.4x\n",
    index, destid, data);
    spin_lock_irqsave(&fsl_rio_doorbell_lock, flags);
// In the serial version silicons, such as MPC8548, MPC8641,
// below operations is must be.
//
    out_be32(&dbell.dbell_regs.odmr, 0x00000000);
    out_be32(&dbell.dbell_regs.odretcr, 0x00000004);
    out_be32(&dbell.dbell_regs.oddpr, destid << 16);
    out_be32(&dbell.dbell_regs.oddatr, (index << 20) | data);
    out_be32(&dbell.dbell_regs.odmr, 0x00000001);
    spin_unlock_irqrestore(&fsl_rio_doorbell_lock, flags);
    return 0;
    }
//
// fsl_add_outb_message - Add message to the MPC85xx outbound message queue
// @mport: Master port with outbound message queue
// @rdev: Target of outbound message
// @mbox: Outbound mailbox
// @buffer: Message to add to outbound queue
// @len: Length of message
//
// Adds the @buffer message to the MPC85xx outbound message queue. Returns
// %0 on success or %-EINVAL on failure.
//
    int
    fsl_add_outb_message(struct rio_mport *mport, struct rio_dev *rdev, int mbox,
    void *buffer, size_t len)
    {
    struct fsl_rmu *rmu = GET_RMM_HANDLE(mport);
    u32 omr;
    struct rio_tx_desc *desc = (struct rio_tx_desc *)rmu.msg_tx_ring.virt
    + rmu.msg_tx_ring.tx_slot;
    let mut ret: c_int = 0;
    pr_debug("RIO: fsl_add_outb_message(): destid %4.4x mbox %d buffer " \
    "%p len %8.8zx\n", rdev.destid, mbox, buffer, len);
    if ((len < 8) || (len > RIO_MAX_MSG_SIZE)) {
    ret = -EINVAL;
    goto out;
    }
// Copy and clear rest of buffer
    memcpy(rmu.msg_tx_ring.virt_buffer[rmu.msg_tx_ring.tx_slot], buffer,
    len);
    if (len < (RIO_MAX_MSG_SIZE - 4))
    memset(rmu.msg_tx_ring.virt_buffer[rmu.msg_tx_ring.tx_slot]
    + len, 0, RIO_MAX_MSG_SIZE - len);
// Set mbox field for message, and set destid
    desc.dport = (rdev.destid << 16) | (mbox & 0x3);
// Enable EOMI interrupt and priority
    desc.dattr = 0x28000000 | ((mport.index) << 20);
// Set transfer size aligned to next power of 2 (in double words)
    desc.dwcnt = is_power_of_2(len) ? len : 1 << get_bitmask_order(len);
// Set snooping and source buffer address
    desc.saddr = 0x00000004
    | rmu.msg_tx_ring.phys_buffer[rmu.msg_tx_ring.tx_slot];
// Increment enqueue pointer
    omr = in_be32(&rmu.msg_regs.omr);
    out_be32(&rmu.msg_regs.omr, omr | RIO_MSG_OMR_MUI);
// Go to next descriptor
    if (++rmu.msg_tx_ring.tx_slot == rmu.msg_tx_ring.size)
    rmu.msg_tx_ring.tx_slot = 0;
    out:
    return ret;
    }
//
// fsl_open_outb_mbox - Initialize MPC85xx outbound mailbox
// @mport: Master port implementing the outbound message unit
// @dev_id: Device specific pointer to pass on event
// @mbox: Mailbox to open
// @entries: Number of entries in the outbound mailbox ring
//
// Initializes buffer ring, request the outbound message interrupt,
// and enables the outbound message unit. Returns %0 on success and
// %-EINVAL or %-ENOMEM on failure.
//
    int
    fsl_open_outb_mbox(struct rio_mport *mport, void *dev_id, int mbox, int entries)
    {
    int i, j, rc = 0;
    struct rio_priv *priv = mport.priv;
    struct fsl_rmu *rmu = GET_RMM_HANDLE(mport);
    if ((entries < RIO_MIN_TX_RING_SIZE) ||
    (entries > RIO_MAX_TX_RING_SIZE) || (!is_power_of_2(entries))) {
    rc = -EINVAL;
    goto out;
    }
// Initialize shadow copy ring
    rmu.msg_tx_ring.dev_id = dev_id;
    rmu.msg_tx_ring.size = entries;
    for (i = 0; i < rmu.msg_tx_ring.size; i++) {
    rmu.msg_tx_ring.virt_buffer[i] =
    dma_alloc_coherent(priv.dev, RIO_MSG_BUFFER_SIZE,
    &rmu.msg_tx_ring.phys_buffer[i], GFP_KERNEL);
    if (!rmu.msg_tx_ring.virt_buffer[i]) {
    rc = -ENOMEM;
    for (j = 0; j < rmu.msg_tx_ring.size; j++)
    if (rmu.msg_tx_ring.virt_buffer[j])
    dma_free_coherent(priv.dev,
    RIO_MSG_BUFFER_SIZE,
    rmu.msg_tx_ring.
    virt_buffer[j],
    rmu.msg_tx_ring.
    phys_buffer[j]);
    goto out;
    }
    }
// Initialize outbound message descriptor ring
    rmu.msg_tx_ring.virt = dma_alloc_coherent(priv.dev,
    rmu.msg_tx_ring.size * RIO_MSG_DESC_SIZE,
    &rmu.msg_tx_ring.phys,
    GFP_KERNEL);
    if (!rmu.msg_tx_ring.virt) {
    rc = -ENOMEM;
    goto out_dma;
    }
    rmu.msg_tx_ring.tx_slot = 0;
// Point dequeue/enqueue pointers at first entry in ring
    out_be32(&rmu.msg_regs.odqdpar, rmu.msg_tx_ring.phys);
    out_be32(&rmu.msg_regs.odqepar, rmu.msg_tx_ring.phys);
// Configure for snooping
    out_be32(&rmu.msg_regs.osar, 0x00000004);
// Clear interrupt status
    out_be32(&rmu.msg_regs.osr, 0x000000b3);
// Hook up outbound message handler
    rc = request_irq(IRQ_RIO_TX(mport), fsl_rio_tx_handler, 0,
    "msg_tx", (void *)mport);
    if (rc < 0)
    goto out_irq;
//
// Configure outbound message unit
// Snooping
// Interrupts (all enabled, except QEIE)
// Chaining mode
// Disable
//
    out_be32(&rmu.msg_regs.omr, 0x00100220);
// Set number of entries
    out_be32(&rmu.msg_regs.omr,
    in_be32(&rmu.msg_regs.omr) |
    ((get_bitmask_order(entries) - 2) << 12));
// Now enable the unit
    out_be32(&rmu.msg_regs.omr, in_be32(&rmu.msg_regs.omr) | 0x1);
    out:
    return rc;
    out_irq:
    dma_free_coherent(priv.dev,
    rmu.msg_tx_ring.size * RIO_MSG_DESC_SIZE,
    rmu.msg_tx_ring.virt, rmu.msg_tx_ring.phys);
    out_dma:
    for (i = 0; i < rmu.msg_tx_ring.size; i++)
    dma_free_coherent(priv.dev, RIO_MSG_BUFFER_SIZE,
    rmu.msg_tx_ring.virt_buffer[i],
    rmu.msg_tx_ring.phys_buffer[i]);
    return rc;
    }
//
// fsl_close_outb_mbox - Shut down MPC85xx outbound mailbox
// @mport: Master port implementing the outbound message unit
// @mbox: Mailbox to close
//
// Disables the outbound message unit, free all buffers, and
// frees the outbound message interrupt.
//
#[no_mangle]
pub unsafe extern "C" fn fsl_close_outb_mbox(mport: *mut rio_mport, mbox: c_int) {
    void fsl_close_outb_mbox(struct rio_mport *mport, int mbox)
    {
    struct rio_priv *priv = mport.priv;
    struct fsl_rmu *rmu = GET_RMM_HANDLE(mport);
// Disable inbound message unit
    out_be32(&rmu.msg_regs.omr, 0);
// Free ring
    dma_free_coherent(priv.dev,
    rmu.msg_tx_ring.size * RIO_MSG_DESC_SIZE,
    rmu.msg_tx_ring.virt, rmu.msg_tx_ring.phys);
// Free interrupt
    free_irq(IRQ_RIO_TX(mport), (void *)mport);
    }
//
// fsl_open_inb_mbox - Initialize MPC85xx inbound mailbox
// @mport: Master port implementing the inbound message unit
// @dev_id: Device specific pointer to pass on event
// @mbox: Mailbox to open
// @entries: Number of entries in the inbound mailbox ring
//
// Initializes buffer ring, request the inbound message interrupt,
// and enables the inbound message unit. Returns %0 on success
// and %-EINVAL or %-ENOMEM on failure.
//
    int
    fsl_open_inb_mbox(struct rio_mport *mport, void *dev_id, int mbox, int entries)
    {
    int i, rc = 0;
    struct rio_priv *priv = mport.priv;
    struct fsl_rmu *rmu = GET_RMM_HANDLE(mport);
    if ((entries < RIO_MIN_RX_RING_SIZE) ||
    (entries > RIO_MAX_RX_RING_SIZE) || (!is_power_of_2(entries))) {
    rc = -EINVAL;
    goto out;
    }
// Initialize client buffer ring
    rmu.msg_rx_ring.dev_id = dev_id;
    rmu.msg_rx_ring.size = entries;
    rmu.msg_rx_ring.rx_slot = 0;
    for (i = 0; i < rmu.msg_rx_ring.size; i++)
    rmu.msg_rx_ring.virt_buffer[i] = core::ptr::null_mut();
// Initialize inbound message ring
    rmu.msg_rx_ring.virt = dma_alloc_coherent(priv.dev,
    rmu.msg_rx_ring.size * RIO_MAX_MSG_SIZE,
    &rmu.msg_rx_ring.phys, GFP_KERNEL);
    if (!rmu.msg_rx_ring.virt) {
    rc = -ENOMEM;
    goto out;
    }
// Point dequeue/enqueue pointers at first entry in ring
    out_be32(&rmu.msg_regs.ifqdpar, (u32) rmu.msg_rx_ring.phys);
    out_be32(&rmu.msg_regs.ifqepar, (u32) rmu.msg_rx_ring.phys);
// Clear interrupt status
    out_be32(&rmu.msg_regs.isr, 0x00000091);
// Hook up inbound message handler
    rc = request_irq(IRQ_RIO_RX(mport), fsl_rio_rx_handler, 0,
    "msg_rx", (void *)mport);
    if (rc < 0) {
    dma_free_coherent(priv.dev,
    rmu.msg_rx_ring.size * RIO_MAX_MSG_SIZE,
    rmu.msg_rx_ring.virt, rmu.msg_rx_ring.phys);
    goto out;
    }
//
// Configure inbound message unit:
// Snooping
// 4KB max message size
// Unmask all interrupt sources
// Disable
//
    out_be32(&rmu.msg_regs.imr, 0x001b0060);
// Set number of queue entries
    setbits32(&rmu.msg_regs.imr, (get_bitmask_order(entries) - 2) << 12);
// Now enable the unit
    setbits32(&rmu.msg_regs.imr, 0x1);
    out:
    return rc;
    }
//
// fsl_close_inb_mbox - Shut down MPC85xx inbound mailbox
// @mport: Master port implementing the inbound message unit
// @mbox: Mailbox to close
//
// Disables the inbound message unit, free all buffers, and
// frees the inbound message interrupt.
//
#[no_mangle]
pub unsafe extern "C" fn fsl_close_inb_mbox(mport: *mut rio_mport, mbox: c_int) {
    void fsl_close_inb_mbox(struct rio_mport *mport, int mbox)
    {
    struct rio_priv *priv = mport.priv;
    struct fsl_rmu *rmu = GET_RMM_HANDLE(mport);
// Disable inbound message unit
    out_be32(&rmu.msg_regs.imr, 0);
// Free ring
    dma_free_coherent(priv.dev, rmu.msg_rx_ring.size * RIO_MAX_MSG_SIZE,
    rmu.msg_rx_ring.virt, rmu.msg_rx_ring.phys);
// Free interrupt
    free_irq(IRQ_RIO_RX(mport), (void *)mport);
    }
//
// fsl_add_inb_buffer - Add buffer to the MPC85xx inbound message queue
// @mport: Master port implementing the inbound message unit
// @mbox: Inbound mailbox number
// @buf: Buffer to add to inbound queue
//
// Adds the @buf buffer to the MPC85xx inbound message queue. Returns
// %0 on success or %-EINVAL on failure.
//
#[no_mangle]
pub unsafe extern "C" fn fsl_add_inb_buffer(mport: *mut rio_mport, mbox: c_int, buf: *mut c_void) -> c_int {
    int fsl_add_inb_buffer(struct rio_mport *mport, int mbox, void *buf)
    {
    let mut rc: c_int = 0;
    struct fsl_rmu *rmu = GET_RMM_HANDLE(mport);
    pr_debug("RIO: fsl_add_inb_buffer(), msg_rx_ring.rx_slot %d\n",
    rmu.msg_rx_ring.rx_slot);
    if (rmu.msg_rx_ring.virt_buffer[rmu.msg_rx_ring.rx_slot]) {
    printk(KERN_ERR
    "RIO: error adding inbound buffer %d, buffer exists\n",
    rmu.msg_rx_ring.rx_slot);
    rc = -EINVAL;
    goto out;
    }
    rmu.msg_rx_ring.virt_buffer[rmu.msg_rx_ring.rx_slot] = buf;
    if (++rmu.msg_rx_ring.rx_slot == rmu.msg_rx_ring.size)
    rmu.msg_rx_ring.rx_slot = 0;
    out:
    return rc;
    }
//
// fsl_get_inb_message - Fetch inbound message from the MPC85xx message unit
// @mport: Master port implementing the inbound message unit
// @mbox: Inbound mailbox number
//
// Gets the next available inbound message from the inbound message queue.
// A pointer to the message is returned on success or NULL on failure.
//
    void *fsl_get_inb_message(struct rio_mport *mport, int mbox)
    {
    struct fsl_rmu *rmu = GET_RMM_HANDLE(mport);
    u32 phys_buf;
    void *virt_buf;
    void *buf = core::ptr::null_mut();
    int buf_idx;
    phys_buf = in_be32(&rmu.msg_regs.ifqdpar);
// If no more messages, then bail out
    if (phys_buf == in_be32(&rmu.msg_regs.ifqepar))
    goto out2;
    virt_buf = rmu.msg_rx_ring.virt + (phys_buf
    - rmu.msg_rx_ring.phys);
    buf_idx = (phys_buf - rmu.msg_rx_ring.phys) / RIO_MAX_MSG_SIZE;
    buf = rmu.msg_rx_ring.virt_buffer[buf_idx];
    if (!buf) {
    printk(KERN_ERR
    "RIO: inbound message copy failed, no buffers\n");
    goto out1;
    }
// Copy max message size, caller is expected to allocate that big
    memcpy(buf, virt_buf, RIO_MAX_MSG_SIZE);
// Clear the available buffer
    rmu.msg_rx_ring.virt_buffer[buf_idx] = core::ptr::null_mut();
    out1:
    setbits32(&rmu.msg_regs.imr, RIO_MSG_IMR_MI);
    out2:
    return buf;
    }
//
// fsl_rio_doorbell_init - MPC85xx doorbell interface init
// @mport: Master port implementing the inbound doorbell unit
//
// Initializes doorbell unit hardware and inbound DMA buffer
// ring. Called from fsl_rio_setup(). Returns %0 on success
// or %-ENOMEM on failure.
//
#[no_mangle]
pub unsafe extern "C" fn fsl_rio_doorbell_init(dbell: *mut fsl_rio_dbell) -> c_int {
    int fsl_rio_doorbell_init(struct fsl_rio_dbell *dbell)
    {
    let mut rc: c_int = 0;
// Initialize inbound doorbells
    dbell.dbell_ring.virt = dma_alloc_coherent(dbell.dev, 512 *
    DOORBELL_MESSAGE_SIZE, &dbell.dbell_ring.phys, GFP_KERNEL);
    if (!dbell.dbell_ring.virt) {
    printk(KERN_ERR "RIO: unable allocate inbound doorbell ring\n");
    rc = -ENOMEM;
    goto out;
    }
// Point dequeue/enqueue pointers at first entry in ring
    out_be32(&dbell.dbell_regs.dqdpar, (u32) dbell.dbell_ring.phys);
    out_be32(&dbell.dbell_regs.dqepar, (u32) dbell.dbell_ring.phys);
// Clear interrupt status
    out_be32(&dbell.dbell_regs.dsr, 0x00000091);
// Hook up doorbell handler
    rc = request_irq(IRQ_RIO_BELL(dbell), fsl_rio_dbell_handler, 0,
    "dbell_rx", (void *)dbell);
    if (rc < 0) {
    dma_free_coherent(dbell.dev, 512 * DOORBELL_MESSAGE_SIZE,
    dbell.dbell_ring.virt, dbell.dbell_ring.phys);
    printk(KERN_ERR
    "MPC85xx RIO: unable to request inbound doorbell irq");
    goto out;
    }
// Configure doorbells for snooping, 512 entries, and enable
    out_be32(&dbell.dbell_regs.dmr, 0x00108161);
    out:
    return rc;
    }
#[no_mangle]
pub unsafe extern "C" fn fsl_rio_setup_rmu(mport: *mut rio_mport, node: *mut device_node) -> c_int {
    int fsl_rio_setup_rmu(struct rio_mport *mport, struct device_node *node)
    {
    struct rio_priv *priv;
    struct fsl_rmu *rmu;
    u64 msg_start;
    if (!mport || !mport.priv)
    return -EINVAL;
    priv = mport.priv;
    if (!node) {
    dev_warn(priv.dev, "Can't get %pOF property 'fsl,rmu'\n",
    priv.dev.of_node);
    return -EINVAL;
    }
    rmu = kzalloc_obj(struct fsl_rmu);
    if (!rmu)
    return -ENOMEM;
    if (of_property_read_reg(node, 0, &msg_start, core::ptr::null_mut())) {
    pr_err("%pOF: unable to find 'reg' property of message-unit\n",
    node);
    kfree(rmu);
    return -ENOMEM;
    }
    rmu.msg_regs = (struct rio_msg_regs *)
    (rmu_regs_win + (u32)msg_start);
    rmu.txirq = irq_of_parse_and_map(node, 0);
    rmu.rxirq = irq_of_parse_and_map(node, 1);
    printk(KERN_INFO "%pOF: txirq: %d, rxirq %d\n",
    node, rmu.txirq, rmu.rxirq);
    priv.rmm_handle = rmu;
    rio_init_dbell_res(&mport.riores[RIO_DOORBELL_RESOURCE], 0, 0xffff);
    rio_init_mbox_res(&mport.riores[RIO_INB_MBOX_RESOURCE], 0, 0);
    rio_init_mbox_res(&mport.riores[RIO_OUTB_MBOX_RESOURCE], 0, 0);
    return 0;
    }
