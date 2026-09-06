//! Automatically rewritten from C to Rust
//! Source: drivers/usb/misc/uss720.c
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
// uss720.c  --  USS720 USB Parport Cable.
//
// Copyright (C) 1999, 2005, 2010
// Thomas Sailer (t.sailer@alumni.ethz.ch)
//
// Based on parport_pc.c
//
// History:
// 0.1  04.08.1999  Created
// 0.2  07.08.1999  Some fixes mainly suggested by Tim Waugh
// Interrupt handling currently disabled because
// usb_request_irq crashes somewhere within ohci.c
// for no apparent reason (that is for me, anyway)
// ECP currently untested
// 0.3  10.08.1999  fixing merge errors
// 0.4  13.08.1999  Added Vendor/Product ID of Brad Hard's cable
// 0.5  20.09.1999  usb_control_msg wrapper used
// Nov01.2000  usb_device_table support by Adam J. Richter
// 08.04.2001  Identify version on module load.  gb
// 0.6  02.09.2005  Fix "scheduling in interrupt" problem by making save/restore
// context asynchronous
//

// ---------------------------------------------------------------------
#[repr(C)]
#[derive(Copy, Clone)]
pub struct parport_uss720_private {
    pub usbdev: *mut usb_device,
    pub pp: *mut parport,
    pub ref_count: kref,
    pub /: *mut *mut __u8 reg[7]; / USB registers,
    pub asynclist: list_head,
    pub asynclock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uss720_async_request {
    pub priv: *mut parport_uss720_private,
    pub ref_count: kref,
    pub asynclist: list_head,
    pub compl: completion,
    pub urb: *mut urb,
    pub dr: *mut usb_ctrlrequest,
    pub reg: [__u8; 7],
}

// ---------------------------------------------------------------------
#[no_mangle]
unsafe extern "C" fn destroy_priv(kref: *mut kref) {
    static void destroy_priv(struct kref *kref)
    {
    struct parport_uss720_private *priv = container_of(kref, struct parport_uss720_private, ref_count);
    dev_dbg(&priv.usbdev.dev, "destroying priv datastructure\n");
    usb_put_dev(priv.usbdev);
    priv.usbdev = core::ptr::null_mut();
    kfree(priv);
    }
#[no_mangle]
unsafe extern "C" fn destroy_async(kref: *mut kref) {
    static void destroy_async(struct kref *kref)
    {
    struct uss720_async_request *rq = container_of(kref, struct uss720_async_request, ref_count);
    struct parport_uss720_private *priv = rq.priv;
    unsigned long flags;
    if (likely(rq.urb))
    usb_free_urb(rq.urb);
    kfree(rq.dr);
    spin_lock_irqsave(&priv.asynclock, flags);
    list_del_init(&rq.asynclist);
    spin_unlock_irqrestore(&priv.asynclock, flags);
    kfree(rq);
    kref_put(&priv.ref_count, destroy_priv);
    }
// ---------------------------------------------------------------------
#[no_mangle]
unsafe extern "C" fn async_complete(urb: *mut urb) {
    static void async_complete(struct urb *urb)
    {
    struct uss720_async_request *rq;
    struct parport *pp;
    struct parport_uss720_private *priv;
    let mut status: c_int = urb.status;
    rq = urb.context;
    priv = rq.priv;
    pp = priv.pp;
    if (status) {
    dev_err(&urb.dev.dev, "async_complete: urb error %d\n",
    status);
    } else if (rq.dr.bRequest == 3) {
    memcpy(priv.reg, rq.reg, sizeof(priv.reg));

    dev_dbg(&priv.usbdev.dev, "async_complete regs %7ph\n",
    priv.reg);

// if nAck interrupts are enabled and we have an interrupt, call the interrupt procedure
    if (rq.reg[2] & rq.reg[1] & 0x10 && pp)
    parport_generic_irq(pp);
    }
    complete(&rq.compl);
    kref_put(&rq.ref_count, destroy_async);
    }
    static struct uss720_async_request *submit_async_request(struct parport_uss720_private *priv,
    __u8 request, __u8 requesttype, __u16 value, __u16 index,
    gfp_t mem_flags)
    {
    struct usb_device *usbdev;
    struct uss720_async_request *rq;
    unsigned long flags;
    int ret;
    if (!priv)
    return core::ptr::null_mut();
    usbdev = priv.usbdev;
    if (!usbdev)
    return core::ptr::null_mut();
    rq = kzalloc_obj(struct uss720_async_request, mem_flags);
    if (!rq)
    return core::ptr::null_mut();
    kref_init(&rq.ref_count);
    INIT_LIST_HEAD(&rq.asynclist);
    init_completion(&rq.compl);
    kref_get(&priv.ref_count);
    rq.priv = priv;
    rq.urb = usb_alloc_urb(0, mem_flags);
    if (!rq.urb) {
    kref_put(&rq.ref_count, destroy_async);
    return core::ptr::null_mut();
    }
    rq.dr = kmalloc_obj(*rq.dr, mem_flags);
    if (!rq.dr) {
    kref_put(&rq.ref_count, destroy_async);
    return core::ptr::null_mut();
    }
    rq.dr.bRequestType = requesttype;
    rq.dr.bRequest = request;
    rq.dr.wValue = cpu_to_le16(value);
    rq.dr.wIndex = cpu_to_le16(index);
    rq.dr.wLength = cpu_to_le16((request == 3) ? sizeof(rq.reg) : 0);
    usb_fill_control_urb(rq.urb, usbdev, (requesttype & 0x80) ? usb_rcvctrlpipe(usbdev, 0) : usb_sndctrlpipe(usbdev, 0),
    (unsigned char *)rq.dr,
    (request == 3) ? rq.reg : core::ptr::null_mut(), (request == 3) ? sizeof(rq.reg) : 0, async_complete, rq);
// rq->urb->transfer_flags |= URB_ASYNC_UNLINK;
    spin_lock_irqsave(&priv.asynclock, flags);
    list_add_tail(&rq.asynclist, &priv.asynclist);
    spin_unlock_irqrestore(&priv.asynclock, flags);
    kref_get(&rq.ref_count);
    ret = usb_submit_urb(rq.urb, mem_flags);
    if (!ret)
    return rq;
    destroy_async(&rq.ref_count);
    dev_err(&usbdev.dev, "submit_async_request submit_urb failed with %d\n", ret);
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn kill_all_async_requests_priv(priv: *mut parport_uss720_private) -> c_uint {
    static unsigned int kill_all_async_requests_priv(struct parport_uss720_private *priv)
    {
    struct uss720_async_request *rq;
    unsigned long flags;
    let mut ret: c_uint = 0;
    spin_lock_irqsave(&priv.asynclock, flags);
    list_for_each_entry(rq, &priv.asynclist, asynclist) {
    usb_unlink_urb(rq.urb);
    ret++;
    }
    spin_unlock_irqrestore(&priv.asynclock, flags);
    return ret;
    }
// ---------------------------------------------------------------------
#[no_mangle]
unsafe extern "C" fn get_1284_register(pp: *mut parport, reg: c_uchar, val: *mut c_uchar, mem_flags: gfp_t) -> c_int {
    static int get_1284_register(struct parport *pp, unsigned char reg, unsigned char *val, gfp_t mem_flags)
    {
    struct parport_uss720_private *priv;
    struct uss720_async_request *rq;
    static const unsigned char regindex[9] = {
    4, 0, 1, 5, 5, 0, 2, 3, 6
    };
    int ret;
    if (!pp)
    return -EIO;
    priv = pp.private_data;
    rq = submit_async_request(priv, 3, 0xc0, ((unsigned int)reg) << 8, 0, mem_flags);
    if (!rq) {
    dev_err(&priv.usbdev.dev, "get_1284_register(%u) failed",
    (unsigned int)reg);
    return -EIO;
    }
    if (!val) {
    kref_put(&rq.ref_count, destroy_async);
    return 0;
    }
    if (wait_for_completion_timeout(&rq.compl, HZ)) {
    ret = rq.urb.status;
// val = priv->reg[(reg >= 9) ? 0 : regindex[reg]];
    if (ret)
    printk(KERN_WARNING "get_1284_register: "
    "usb error %d\n", ret);
    kref_put(&rq.ref_count, destroy_async);
    return ret;
    }
    printk(KERN_WARNING "get_1284_register timeout\n");
    kill_all_async_requests_priv(priv);
    kref_put(&rq.ref_count, destroy_async);
    return -EIO;
    }
#[no_mangle]
unsafe extern "C" fn set_1284_register(pp: *mut parport, reg: c_uchar, val: c_uchar, mem_flags: gfp_t) -> c_int {
    static int set_1284_register(struct parport *pp, unsigned char reg, unsigned char val, gfp_t mem_flags)
    {
    struct parport_uss720_private *priv;
    struct uss720_async_request *rq;
    if (!pp)
    return -EIO;
    priv = pp.private_data;
    rq = submit_async_request(priv, 4, 0x40, (((unsigned int)reg) << 8) | val, 0, mem_flags);
    if (!rq) {
    dev_err(&priv.usbdev.dev, "set_1284_register(%u,%u) failed",
    (unsigned int)reg, (unsigned int)val);
    return -EIO;
    }
    kref_put(&rq.ref_count, destroy_async);
    return 0;
    }
// ---------------------------------------------------------------------
// ECR modes
pub const ECR_SPP: c_int = 00;
pub const ECR_PS2: c_int = 01;
pub const ECR_PPF: c_int = 02;
pub const ECR_ECP: c_int = 03;
pub const ECR_EPP: c_int = 04;
// Safely change the mode bits in the ECR
#[no_mangle]
unsafe extern "C" fn change_mode(pp: *mut parport, m: c_int) -> c_int {
    static int change_mode(struct parport *pp, int m)
    {
    struct parport_uss720_private *priv = pp.private_data;
    int mode;
    __u8 reg;
    if (get_1284_register(pp, 6, &reg, GFP_KERNEL))
    return -EIO;
// Bits <7:5> contain the mode.
    mode = (priv.reg[2] >> 5) & 0x7;
    if (mode == m)
    return 0;
// We have to go through mode 000 or 001
    if (mode > ECR_PS2 && m > ECR_PS2)
    if (change_mode(pp, ECR_PS2))
    return -EIO;
    if (m <= ECR_PS2 && !(priv.reg[1] & 0x20)) {
// This mode resets the FIFO, so we may
// have to wait for it to drain first.
    let mut expire: c_ulong = jiffies + pp.physport.cad.timeout;
    switch (mode) {
    case ECR_PPF: /* Parallel Port FIFO mode */
    case ECR_ECP: /* ECP Parallel Port mode */
// Poll slowly.
    for (;;) {
    if (get_1284_register(pp, 6, &reg, GFP_KERNEL))
    return -EIO;
    if (priv.reg[2] & 0x01)
    break;
    if (time_after_eq (jiffies, expire))
// The FIFO is stuck.
    return -EBUSY;
    msleep_interruptible(10);
    if (signal_pending (current))
    break;
    }
    }
    }
// Set the mode.
    if (set_1284_register(pp, 6, m << 5, GFP_KERNEL))
    return -EIO;
    if (get_1284_register(pp, 6, &reg, GFP_KERNEL))
    return -EIO;
    return 0;
    }
//
// Clear TIMEOUT BIT in EPP MODE
//
#[no_mangle]
unsafe extern "C" fn clear_epp_timeout(pp: *mut parport) -> c_int {
    static int clear_epp_timeout(struct parport *pp)
    {
    unsigned char stat;
    if (get_1284_register(pp, 1, &stat, GFP_KERNEL))
    return 1;
    return stat & 1;
    }
//
// Access functions.
//

#[no_mangle]
unsafe extern "C" fn uss720_irq(usbstatus: c_int, buffer: *mut c_void, len: c_int, dev_id: *mut c_void) -> c_int {
    static int uss720_irq(int usbstatus, void *buffer, int len, void *dev_id)
    {
    struct parport *pp = (struct parport *)dev_id;
    struct parport_uss720_private *priv = pp.private_data;
    if (usbstatus != 0 || len < 4 || !buffer)
    return 1;
    memcpy(priv.reg, buffer, 4);
// if nAck interrupts are enabled and we have an interrupt, call the interrupt procedure
    if (priv.reg[2] & priv.reg[1] & 0x10)
    parport_generic_irq(pp);
    return 1;
    }

#[no_mangle]
unsafe extern "C" fn parport_uss720_write_data(pp: *mut parport, d: c_uchar) {
    static void parport_uss720_write_data(struct parport *pp, unsigned char d)
    {
    set_1284_register(pp, 0, d, GFP_KERNEL);
    }
#[no_mangle]
unsafe extern "C" fn parport_uss720_read_data(pp: *mut parport) -> c_uchar {
    static unsigned char parport_uss720_read_data(struct parport *pp)
    {
    unsigned char ret;
    if (get_1284_register(pp, 0, &ret, GFP_KERNEL))
    return 0;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn parport_uss720_write_control(pp: *mut parport, d: c_uchar) {
    static void parport_uss720_write_control(struct parport *pp, unsigned char d)
    {
    struct parport_uss720_private *priv = pp.private_data;
    d = (d & 0xf) | (priv.reg[1] & 0xf0);
    if (set_1284_register(pp, 2, d, GFP_KERNEL))
    return;
    priv.reg[1] = d;
    }
#[no_mangle]
unsafe extern "C" fn parport_uss720_read_control(pp: *mut parport) -> c_uchar {
    static unsigned char parport_uss720_read_control(struct parport *pp)
    {
    struct parport_uss720_private *priv = pp.private_data;
    return priv.reg[1] & 0xf; /* Use soft copy */
    }
#[no_mangle]
unsafe extern "C" fn parport_uss720_frob_control(pp: *mut parport, mask: c_uchar, val: c_uchar) -> c_uchar {
    static unsigned char parport_uss720_frob_control(struct parport *pp, unsigned char mask, unsigned char val)
    {
    struct parport_uss720_private *priv = pp.private_data;
    unsigned char d;
    mask &= 0x0f;
    val &= 0x0f;
    d = (priv.reg[1] & (~mask)) ^ val;
    if (set_1284_register(pp, 2, d, GFP_ATOMIC))
    return 0;
    priv.reg[1] = d;
    return d & 0xf;
    }
#[no_mangle]
unsafe extern "C" fn parport_uss720_read_status(pp: *mut parport) -> c_uchar {
    static unsigned char parport_uss720_read_status(struct parport *pp)
    {
    unsigned char ret;
    if (get_1284_register(pp, 1, &ret, GFP_ATOMIC))
    return 0;
    return ret & 0xf8;
    }
#[no_mangle]
unsafe extern "C" fn parport_uss720_disable_irq(pp: *mut parport) {
    static void parport_uss720_disable_irq(struct parport *pp)
    {
    struct parport_uss720_private *priv = pp.private_data;
    unsigned char d;
    d = priv.reg[1] & ~0x10;
    if (set_1284_register(pp, 2, d, GFP_KERNEL))
    return;
    priv.reg[1] = d;
    }
#[no_mangle]
unsafe extern "C" fn parport_uss720_enable_irq(pp: *mut parport) {
    static void parport_uss720_enable_irq(struct parport *pp)
    {
    struct parport_uss720_private *priv = pp.private_data;
    unsigned char d;
    d = priv.reg[1] | 0x10;
    if (set_1284_register(pp, 2, d, GFP_KERNEL))
    return;
    priv.reg[1] = d;
    }
#[no_mangle]
unsafe extern "C" fn parport_uss720_data_forward(pp: *mut parport) {
    static void parport_uss720_data_forward (struct parport *pp)
    {
    struct parport_uss720_private *priv = pp.private_data;
    unsigned char d;
    d = priv.reg[1] & ~0x20;
    if (set_1284_register(pp, 2, d, GFP_KERNEL))
    return;
    priv.reg[1] = d;
    }
#[no_mangle]
unsafe extern "C" fn parport_uss720_data_reverse(pp: *mut parport) {
    static void parport_uss720_data_reverse (struct parport *pp)
    {
    struct parport_uss720_private *priv = pp.private_data;
    unsigned char d;
    d = priv.reg[1] | 0x20;
    if (set_1284_register(pp, 2, d, GFP_KERNEL))
    return;
    priv.reg[1] = d;
    }
#[no_mangle]
unsafe extern "C" fn parport_uss720_init_state(dev: *mut pardevice, s: *mut parport_state) {
    static void parport_uss720_init_state(struct pardevice *dev, struct parport_state *s)
    {
    s.u.pc.ctr = 0xc | (dev.irq_func ? 0x10 : 0x0);
    s.u.pc.ecr = 0x24;
    }
#[no_mangle]
unsafe extern "C" fn parport_uss720_save_state(pp: *mut parport, s: *mut parport_state) {
    static void parport_uss720_save_state(struct parport *pp, struct parport_state *s)
    {
    struct parport_uss720_private *priv = pp.private_data;

    if (get_1284_register(pp, 2, core::ptr::null_mut(), GFP_ATOMIC))
    return;

    s.u.pc.ctr = priv.reg[1];
    s.u.pc.ecr = priv.reg[2];
    }
#[no_mangle]
unsafe extern "C" fn parport_uss720_restore_state(pp: *mut parport, s: *mut parport_state) {
    static void parport_uss720_restore_state(struct parport *pp, struct parport_state *s)
    {
    struct parport_uss720_private *priv = pp.private_data;
    set_1284_register(pp, 2, s.u.pc.ctr, GFP_ATOMIC);
    set_1284_register(pp, 6, s.u.pc.ecr, GFP_ATOMIC);
    get_1284_register(pp, 2, core::ptr::null_mut(), GFP_ATOMIC);
    priv.reg[1] = s.u.pc.ctr;
    priv.reg[2] = s.u.pc.ecr;
    }
#[no_mangle]
unsafe extern "C" fn parport_uss720_epp_read_data(pp: *mut parport, buf: *mut c_void, length: usize, flags: c_int) -> usize {
    static size_t parport_uss720_epp_read_data(struct parport *pp, void *buf, size_t length, int flags)
    {
    struct parport_uss720_private *priv = pp.private_data;
    let mut got: usize = 0;
    if (change_mode(pp, ECR_EPP))
    return 0;
    for (; got < length; got++) {
    if (get_1284_register(pp, 4, (char *)buf, GFP_KERNEL))
    break;
    buf++;
    if (priv.reg[0] & 0x01) {
    clear_epp_timeout(pp);
    break;
    }
    }
    change_mode(pp, ECR_PS2);
    return got;
    }
#[no_mangle]
unsafe extern "C" fn parport_uss720_epp_write_data(pp: *mut parport, buf: *const c_void, length: usize, flags: c_int) -> usize {
    static size_t parport_uss720_epp_write_data(struct parport *pp, const void *buf, size_t length, int flags)
    {

    struct parport_uss720_private *priv = pp.private_data;
    let mut written: usize = 0;
    if (change_mode(pp, ECR_EPP))
    return 0;
    for (; written < length; written++) {
    if (set_1284_register(pp, 4, (char *)buf, GFP_KERNEL))
    break;
    ((char*)buf)++;
    if (get_1284_register(pp, 1, core::ptr::null_mut(), GFP_KERNEL))
    break;
    if (priv.reg[0] & 0x01) {
    clear_epp_timeout(pp);
    break;
    }
    }
    change_mode(pp, ECR_PS2);
    return written;

    struct parport_uss720_private *priv = pp.private_data;
    struct usb_device *usbdev = priv.usbdev;
    let mut rlen: c_int = 0;
    int i;
    if (!usbdev)
    return 0;
    if (change_mode(pp, ECR_EPP))
    return 0;
    i = usb_bulk_msg(usbdev, usb_sndbulkpipe(usbdev, 1), (void *)buf, length, &rlen, 20000);
    if (i)
    printk(KERN_ERR "uss720: sendbulk ep 1 buf %p len %zu rlen %u\n", buf, length, rlen);
    change_mode(pp, ECR_PS2);
    return rlen;

    }
#[no_mangle]
unsafe extern "C" fn parport_uss720_epp_read_addr(pp: *mut parport, buf: *mut c_void, length: usize, flags: c_int) -> usize {
    static size_t parport_uss720_epp_read_addr(struct parport *pp, void *buf, size_t length, int flags)
    {
    struct parport_uss720_private *priv = pp.private_data;
    let mut got: usize = 0;
    if (change_mode(pp, ECR_EPP))
    return 0;
    for (; got < length; got++) {
    if (get_1284_register(pp, 3, (char *)buf, GFP_KERNEL))
    break;
    buf++;
    if (priv.reg[0] & 0x01) {
    clear_epp_timeout(pp);
    break;
    }
    }
    change_mode(pp, ECR_PS2);
    return got;
    }
#[no_mangle]
unsafe extern "C" fn parport_uss720_epp_write_addr(pp: *mut parport, buf: *const c_void, length: usize, flags: c_int) -> usize {
    static size_t parport_uss720_epp_write_addr(struct parport *pp, const void *buf, size_t length, int flags)
    {
    struct parport_uss720_private *priv = pp.private_data;
    let mut written: usize = 0;
    if (change_mode(pp, ECR_EPP))
    return 0;
    for (; written < length; written++) {
    if (set_1284_register(pp, 3, *(char *)buf, GFP_KERNEL))
    break;
    buf++;
    if (get_1284_register(pp, 1, core::ptr::null_mut(), GFP_KERNEL))
    break;
    if (priv.reg[0] & 0x01) {
    clear_epp_timeout(pp);
    break;
    }
    }
    change_mode(pp, ECR_PS2);
    return written;
    }
#[no_mangle]
unsafe extern "C" fn parport_uss720_ecp_write_data(pp: *mut parport, buffer: *const c_void, len: usize, flags: c_int) -> usize {
    static size_t parport_uss720_ecp_write_data(struct parport *pp, const void *buffer, size_t len, int flags)
    {
    struct parport_uss720_private *priv = pp.private_data;
    struct usb_device *usbdev = priv.usbdev;
    let mut rlen: c_int = 0;
    int i;
    if (!usbdev)
    return 0;
    if (change_mode(pp, ECR_ECP))
    return 0;
    i = usb_bulk_msg(usbdev, usb_sndbulkpipe(usbdev, 1), (void *)buffer, len, &rlen, 20000);
    if (i)
    printk(KERN_ERR "uss720: sendbulk ep 1 buf %p len %zu rlen %u\n", buffer, len, rlen);
    change_mode(pp, ECR_PS2);
    return rlen;
    }
#[no_mangle]
unsafe extern "C" fn parport_uss720_ecp_read_data(pp: *mut parport, buffer: *mut c_void, len: usize, flags: c_int) -> usize {
    static size_t parport_uss720_ecp_read_data(struct parport *pp, void *buffer, size_t len, int flags)
    {
    struct parport_uss720_private *priv = pp.private_data;
    struct usb_device *usbdev = priv.usbdev;
    let mut rlen: c_int = 0;
    int i;
    if (!usbdev)
    return 0;
    if (change_mode(pp, ECR_ECP))
    return 0;
    i = usb_bulk_msg(usbdev, usb_rcvbulkpipe(usbdev, 2), buffer, len, &rlen, 20000);
    if (i)
    printk(KERN_ERR "uss720: recvbulk ep 2 buf %p len %zu rlen %u\n", buffer, len, rlen);
    change_mode(pp, ECR_PS2);
    return rlen;
    }
#[no_mangle]
unsafe extern "C" fn parport_uss720_ecp_write_addr(pp: *mut parport, buffer: *const c_void, len: usize, flags: c_int) -> usize {
    static size_t parport_uss720_ecp_write_addr(struct parport *pp, const void *buffer, size_t len, int flags)
    {
    let mut written: usize = 0;
    if (change_mode(pp, ECR_ECP))
    return 0;
    for (; written < len; written++) {
    if (set_1284_register(pp, 5, *(char *)buffer, GFP_KERNEL))
    break;
    buffer++;
    }
    change_mode(pp, ECR_PS2);
    return written;
    }
#[no_mangle]
unsafe extern "C" fn parport_uss720_write_compat(pp: *mut parport, buffer: *const c_void, len: usize, flags: c_int) -> usize {
    static size_t parport_uss720_write_compat(struct parport *pp, const void *buffer, size_t len, int flags)
    {
    struct parport_uss720_private *priv = pp.private_data;
    struct usb_device *usbdev = priv.usbdev;
    let mut rlen: c_int = 0;
    int i;
    if (!usbdev)
    return 0;
    if (change_mode(pp, ECR_PPF))
    return 0;
    i = usb_bulk_msg(usbdev, usb_sndbulkpipe(usbdev, 1), (void *)buffer, len, &rlen, 20000);
    if (i)
    printk(KERN_ERR "uss720: sendbulk ep 1 buf %p len %zu rlen %u\n", buffer, len, rlen);
    change_mode(pp, ECR_PS2);
    return rlen;
    }
// ---------------------------------------------------------------------
    static struct parport_operations parport_uss720_ops =
    {
    .owner =		THIS_MODULE,
    .write_data =		parport_uss720_write_data,
    .read_data =		parport_uss720_read_data,
    .write_control =	parport_uss720_write_control,
    .read_control =		parport_uss720_read_control,
    .frob_control =		parport_uss720_frob_control,
    .read_status =		parport_uss720_read_status,
    .enable_irq =		parport_uss720_enable_irq,
    .disable_irq =		parport_uss720_disable_irq,
    .data_forward =		parport_uss720_data_forward,
    .data_reverse =		parport_uss720_data_reverse,
    .init_state =		parport_uss720_init_state,
    .save_state =		parport_uss720_save_state,
    .restore_state =	parport_uss720_restore_state,
    .epp_write_data =	parport_uss720_epp_write_data,
    .epp_read_data =	parport_uss720_epp_read_data,
    .epp_write_addr =	parport_uss720_epp_write_addr,
    .epp_read_addr =	parport_uss720_epp_read_addr,
    .ecp_write_data =	parport_uss720_ecp_write_data,
    .ecp_read_data =	parport_uss720_ecp_read_data,
    .ecp_write_addr =	parport_uss720_ecp_write_addr,
    .compat_write_data =	parport_uss720_write_compat,
    .nibble_read_data =	parport_ieee1284_read_nibble,
    .byte_read_data =	parport_ieee1284_read_byte,
    };
// ---------------------------------------------------------------------
    static int uss720_probe(struct usb_interface *intf,
    const struct usb_device_id *id)
    {
    struct usb_device *usbdev = usb_get_dev(interface_to_usbdev(intf));
    struct usb_host_interface *interface;
    struct usb_endpoint_descriptor *epd;
    struct parport_uss720_private *priv;
    struct parport *pp;
    unsigned char reg;
    let mut ret: c_int = -ENODEV;
    dev_dbg(&intf.dev, "probe: vendor id 0x%x, device id 0x%x\n",
    le16_to_cpu(usbdev.descriptor.idVendor),
    le16_to_cpu(usbdev.descriptor.idProduct));
// our known interfaces have 3 alternate settings
    if (intf.num_altsetting != 3)
    goto bail_out_early;
    ret = usb_set_interface(usbdev, intf.altsetting.desc.bInterfaceNumber, 2);
    dev_dbg(&intf.dev, "set interface result %d\n", ret);
    interface = intf.cur_altsetting;
    if (interface.desc.bNumEndpoints < 2)
    goto bail_out_early;
//
// Allocate parport interface
//
    ret = -ENOMEM;
    priv = kzalloc_obj(struct parport_uss720_private);
    if (!priv)
    goto bail_out_early;
    priv.pp = core::ptr::null_mut();
    priv.usbdev = usbdev;
    kref_init(&priv.ref_count);
    spin_lock_init(&priv.asynclock);
    INIT_LIST_HEAD(&priv.asynclist);
    pp = parport_register_port(0, PARPORT_IRQ_NONE, PARPORT_DMA_NONE, &parport_uss720_ops);
    if (!pp) {
    printk(KERN_WARNING "uss720: could not register parport\n");
    goto probe_abort;
    }
    priv.pp = pp;
    pp.private_data = priv;
    pp.modes = PARPORT_MODE_PCSPP | PARPORT_MODE_TRISTATE | PARPORT_MODE_EPP | PARPORT_MODE_COMPAT;
    if (interface.desc.bNumEndpoints >= 3)
    pp.modes |= PARPORT_MODE_ECP;
    pp.dev = &usbdev.dev;
// set the USS720 control register to manual mode, no ECP compression, enable all ints
    set_1284_register(pp, 7, 0x00, GFP_KERNEL);
    set_1284_register(pp, 6, 0x30, GFP_KERNEL);  /* PS/2 mode */
    set_1284_register(pp, 2, 0x0c, GFP_KERNEL);
// The Belkin F5U002 Rev 2 P80453-B USB parallel port adapter shares the
// device ID 050d:0002 with some other device that works with this
// driver, but it itself does not. Detect and handle the bad cable
// here.
    ret = get_1284_register(pp, 0, &reg, GFP_KERNEL);
    dev_dbg(&intf.dev, "reg: %7ph\n", priv.reg);
    if (ret < 0) {
    priv.pp = core::ptr::null_mut();
    parport_del_port(pp);
    goto probe_abort;
    }
    ret = usb_find_last_int_in_endpoint(interface, &epd);
    if (!ret) {
    dev_dbg(&intf.dev, "epaddr %d interval %d\n",
    epd.bEndpointAddress, epd.bInterval);
    }
    parport_announce_port(pp);
    usb_set_intfdata(intf, pp);
    return 0;
    probe_abort:
    kill_all_async_requests_priv(priv);
    kref_put(&priv.ref_count, destroy_priv);
    return -ENODEV;
    bail_out_early:
    usb_put_dev(usbdev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn uss720_disconnect(intf: *mut usb_interface) {
    static void uss720_disconnect(struct usb_interface *intf)
    {
    struct parport *pp = usb_get_intfdata(intf);
    struct parport_uss720_private *priv;
    dev_dbg(&intf.dev, "disconnect\n");
    usb_set_intfdata(intf, core::ptr::null_mut());
    if (pp) {
    priv = pp.private_data;
    priv.pp = core::ptr::null_mut();
    dev_dbg(&intf.dev, "parport_remove_port\n");
    parport_remove_port(pp);
    parport_put_port(pp);
    kill_all_async_requests_priv(priv);
    kref_put(&priv.ref_count, destroy_priv);
    }
    dev_dbg(&intf.dev, "disconnect done\n");
    }
// table of cables that work through this driver
    static const struct usb_device_id uss720_table[] = {
    { USB_DEVICE(0x047e, 0x1001) }, /* Infowave 901-0030 */
    { USB_DEVICE(0x04b8, 0x0002) }, /* Epson CAEUL0002 ISD-103 */
    { USB_DEVICE(0x04b8, 0x0003) }, /* Epson ISD-101 */
    { USB_DEVICE(0x050d, 0x0002) },
    { USB_DEVICE(0x050d, 0x1202) }, /* Belkin F5U120-PC */
    { USB_DEVICE(0x0557, 0x2001) },
    { USB_DEVICE(0x05ab, 0x0002) }, /* Belkin F5U002 ISD-101 */
    { USB_DEVICE(0x05ab, 0x1001) }, /* Belkin F5U002 P80453-A */
    { USB_DEVICE(0x06c6, 0x0100) }, /* Infowave ISD-103 */
    { USB_DEVICE(0x0729, 0x1284) },
    { USB_DEVICE(0x1293, 0x0002) },
    { }						/* Terminating entry */
    };
    MODULE_DEVICE_TABLE (usb, uss720_table);
    static struct usb_driver uss720_driver = {
    .name =		"uss720",
    .probe =	uss720_probe,
    .disconnect =	uss720_disconnect,
    .id_table =	uss720_table,
    };
// ---------------------------------------------------------------------
    MODULE_AUTHOR(DRIVER_AUTHOR);
    MODULE_DESCRIPTION(DRIVER_DESC);
    MODULE_LICENSE("GPL");
#[no_mangle]
unsafe extern "C" fn uss720_init() -> int __init {
    static int __init uss720_init(void)
    {
    int retval;
    retval = usb_register(&uss720_driver);
    if (retval)
    goto out;
    printk(KERN_INFO KBUILD_MODNAME ": " DRIVER_DESC "\n");
    printk(KERN_INFO KBUILD_MODNAME ": NOTE: this is a special purpose "
    "driver to allow nonstandard\n");
    printk(KERN_INFO KBUILD_MODNAME ": protocols (eg. bitbang) over "
    "USS720 usb to parallel cables\n");
    printk(KERN_INFO KBUILD_MODNAME ": If you just want to connect to a "
    "printer, use usblp instead\n");
    out:
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn uss720_cleanup() -> void __exit {
    static void __exit uss720_cleanup(void)
    {
    usb_deregister(&uss720_driver);
    }
    module_init(uss720_init);
    module_exit(uss720_cleanup);
// ---------------------------------------------------------------------
