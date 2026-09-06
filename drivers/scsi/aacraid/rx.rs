//! Automatically rewritten from C to Rust
//! Source: drivers/scsi/aacraid/rx.c
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
// Adaptec AAC series RAID controller driver
// (c) Copyright 2001 Red Hat Inc.
//
// based on the old aacraid driver that is..
// Adaptec aacraid device driver for Linux.
//
// Copyright (c) 2000-2010 Adaptec, Inc.
// 2010-2015 PMC-Sierra, Inc. (aacraid@pmc-sierra.com)
// 2016-2017 Microsemi Corp. (aacraid@microsemi.com)
//
// Module Name:
// rx.c
//
// Abstract: Hardware miniport for Drawbridge specific hardware functions.
//

#[no_mangle]
unsafe extern "C" fn aac_rx_intr_producer(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t aac_rx_intr_producer(int irq, void *dev_id)
    {
    struct aac_dev *dev = dev_id;
    unsigned long bellbits;
    let mut intstat: u8 = rx_readb(dev, MUnit.OISR);
//
// Read mask and invert because drawbridge is reversed.
// This allows us to only service interrupts that have
// been enabled.
// Check to see if this is our interrupt.  If it isn't just return
//
    if (likely(intstat & ~(dev.OIMR))) {
    bellbits = rx_readl(dev, OutboundDoorbellReg);
    if (unlikely(bellbits & DoorBellPrintfReady)) {
    aac_printf(dev, readl (&dev.IndexRegs.Mailbox[5]));
    rx_writel(dev, MUnit.ODR,DoorBellPrintfReady);
    rx_writel(dev, InboundDoorbellReg,DoorBellPrintfDone);
    }
#[no_mangle]
pub unsafe extern "C" fn if(DoorBellAdapterNormCmdReady): unlikely(bellbits &) -> else {
    rx_writel(dev, MUnit.ODR, DoorBellAdapterNormCmdReady);
    aac_command_normal(&dev.queues.queue[HostNormCmdQueue]);
    }
#[no_mangle]
pub unsafe extern "C" fn if(DoorBellAdapterNormRespReady): likely(bellbits &) -> else {
    rx_writel(dev, MUnit.ODR,DoorBellAdapterNormRespReady);
    aac_response_normal(&dev.queues.queue[HostNormRespQueue]);
    }
#[no_mangle]
pub unsafe extern "C" fn if(DoorBellAdapterNormCmdNotFull): unlikely(bellbits &) -> else {
    rx_writel(dev, MUnit.ODR, DoorBellAdapterNormCmdNotFull);
    }
#[no_mangle]
pub unsafe extern "C" fn if(DoorBellAdapterNormRespNotFull): unlikely(bellbits &) -> else {
    rx_writel(dev, MUnit.ODR, DoorBellAdapterNormCmdNotFull);
    rx_writel(dev, MUnit.ODR, DoorBellAdapterNormRespNotFull);
    }
    return IRQ_HANDLED;
    }
    return IRQ_NONE;
    }
#[no_mangle]
unsafe extern "C" fn aac_rx_intr_message(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t aac_rx_intr_message(int irq, void *dev_id)
    {
    int isAif, isFastResponse, isSpecial;
    struct aac_dev *dev = dev_id;
    let mut Index: u32 = rx_readl(dev, MUnit.OutboundQueue);
    if (unlikely(Index == 0xFFFFFFFFL))
    Index = rx_readl(dev, MUnit.OutboundQueue);
    if (likely(Index != 0xFFFFFFFFL)) {
    do {
    isAif = isFastResponse = isSpecial = 0;
    if (Index & 0x00000002L) {
    isAif = 1;
    if (Index == 0xFFFFFFFEL)
    isSpecial = 1;
    Index &= ~0x00000002L;
    } else {
    if (Index & 0x00000001L)
    isFastResponse = 1;
    Index >>= 2;
    }
    if (!isSpecial) {
    if (unlikely(aac_intr_normal(dev,
    Index, isAif,
    isFastResponse, core::ptr::null_mut()))) {
    rx_writel(dev,
    MUnit.OutboundQueue,
    Index);
    rx_writel(dev,
    MUnit.ODR,
    DoorBellAdapterNormRespReady);
    }
    }
    Index = rx_readl(dev, MUnit.OutboundQueue);
    } while (Index != 0xFFFFFFFFL);
    return IRQ_HANDLED;
    }
    return IRQ_NONE;
    }
//
// aac_rx_disable_interrupt	-	Disable interrupts
// @dev: Adapter
//
#[no_mangle]
unsafe extern "C" fn aac_rx_disable_interrupt(dev: *mut aac_dev) {
    static void aac_rx_disable_interrupt(struct aac_dev *dev)
    {
    rx_writeb(dev, MUnit.OIMR, dev.OIMR = 0xff);
    }
//
// aac_rx_enable_interrupt_producer	-	Enable interrupts
// @dev: Adapter
//
#[no_mangle]
unsafe extern "C" fn aac_rx_enable_interrupt_producer(dev: *mut aac_dev) {
    static void aac_rx_enable_interrupt_producer(struct aac_dev *dev)
    {
    rx_writeb(dev, MUnit.OIMR, dev.OIMR = 0xfb);
    }
//
// aac_rx_enable_interrupt_message	-	Enable interrupts
// @dev: Adapter
//
#[no_mangle]
unsafe extern "C" fn aac_rx_enable_interrupt_message(dev: *mut aac_dev) {
    static void aac_rx_enable_interrupt_message(struct aac_dev *dev)
    {
    rx_writeb(dev, MUnit.OIMR, dev.OIMR = 0xf7);
    }
//
// rx_sync_cmd	-	send a command and wait
// @dev: Adapter
// @command: Command to execute
// @p1: first parameter
// @p2: second parameter
// @p3: third parameter
// @p4: forth parameter
// @p5: fifth parameter
// @p6: sixth parameter
// @status: adapter status
// @r1: first return value
// @r2: second return value
// @r3: third return value
// @r4: forth return value
//
// This routine will send a synchronous command to the adapter and wait
// for its	completion.
//
    static int rx_sync_cmd(struct aac_dev *dev, u32 command,
    u32 p1, u32 p2, u32 p3, u32 p4, u32 p5, u32 p6,
    u32 *status, u32 * r1, u32 * r2, u32 * r3, u32 * r4)
    {
    unsigned long start;
    int ok;
//
// Write the command into Mailbox 0
//
    writel(command, &dev.IndexRegs.Mailbox[0]);
//
// Write the parameters into Mailboxes 1 - 6
//
    writel(p1, &dev.IndexRegs.Mailbox[1]);
    writel(p2, &dev.IndexRegs.Mailbox[2]);
    writel(p3, &dev.IndexRegs.Mailbox[3]);
    writel(p4, &dev.IndexRegs.Mailbox[4]);
//
// Clear the synch command doorbell to start on a clean slate.
//
    rx_writel(dev, OutboundDoorbellReg, OUTBOUNDDOORBELL_0);
//
// Disable doorbell interrupts
//
    rx_writeb(dev, MUnit.OIMR, dev.OIMR = 0xff);
//
// Force the completion of the mask register write before issuing
// the interrupt.
//
    rx_readb (dev, MUnit.OIMR);
//
// Signal that there is a new synch command
//
    rx_writel(dev, InboundDoorbellReg, INBOUNDDOORBELL_0);
    ok = 0;
    start = jiffies;
//
// Wait up to 30 seconds
//
    while (time_before(jiffies, start+30*HZ))
    {
    udelay(5);	/* Delay 5 microseconds to let Mon960 get info. */
//
// Mon960 will set doorbell0 bit when it has completed the command.
//
    if (rx_readl(dev, OutboundDoorbellReg) & OUTBOUNDDOORBELL_0) {
//
// Clear the doorbell.
//
    rx_writel(dev, OutboundDoorbellReg, OUTBOUNDDOORBELL_0);
    ok = 1;
    break;
    }
//
// Yield the processor in case we are slow
//
    msleep(1);
    }
    if (unlikely(ok != 1)) {
//
// Restore interrupt mask even though we timed out
//
    aac_adapter_enable_int(dev);
    return -ETIMEDOUT;
    }
//
// Pull the synch status from Mailbox 0.
//
    if (status)
// status = readl(&dev->IndexRegs->Mailbox[0]);
    if (r1)
// r1 = readl(&dev->IndexRegs->Mailbox[1]);
    if (r2)
// r2 = readl(&dev->IndexRegs->Mailbox[2]);
    if (r3)
// r3 = readl(&dev->IndexRegs->Mailbox[3]);
    if (r4)
// r4 = readl(&dev->IndexRegs->Mailbox[4]);
//
// Clear the synch command doorbell.
//
    rx_writel(dev, OutboundDoorbellReg, OUTBOUNDDOORBELL_0);
//
// Restore interrupt mask
//
    aac_adapter_enable_int(dev);
    return 0;
    }
//
// aac_rx_interrupt_adapter	-	interrupt adapter
// @dev: Adapter
//
// Send an interrupt to the i960 and breakpoint it.
//
#[no_mangle]
unsafe extern "C" fn aac_rx_interrupt_adapter(dev: *mut aac_dev) {
    static void aac_rx_interrupt_adapter(struct aac_dev *dev)
    {
    rx_sync_cmd(dev, BREAKPOINT_REQUEST, 0, 0, 0, 0, 0, 0, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
    }
//
// aac_rx_notify_adapter		-	send an event to the adapter
// @dev: Adapter
// @event: Event to send
//
// Notify the i960 that something it probably cares about has
// happened.
//
#[no_mangle]
unsafe extern "C" fn aac_rx_notify_adapter(dev: *mut aac_dev, event: u32) {
    static void aac_rx_notify_adapter(struct aac_dev *dev, u32 event)
    {
    switch (event) {
    case AdapNormCmdQue:
    rx_writel(dev, MUnit.IDR,INBOUNDDOORBELL_1);
    break;
    case HostNormRespNotFull:
    rx_writel(dev, MUnit.IDR,INBOUNDDOORBELL_4);
    break;
    case AdapNormRespQue:
    rx_writel(dev, MUnit.IDR,INBOUNDDOORBELL_2);
    break;
    case HostNormCmdNotFull:
    rx_writel(dev, MUnit.IDR,INBOUNDDOORBELL_3);
    break;
    case HostShutdown:
    break;
    case FastIo:
    rx_writel(dev, MUnit.IDR,INBOUNDDOORBELL_6);
    break;
    case AdapPrintfDone:
    rx_writel(dev, MUnit.IDR,INBOUNDDOORBELL_5);
    break;
    default:
    BUG();
    break;
    }
    }
//
// aac_rx_start_adapter		-	activate adapter
// @dev:	Adapter
//
// Start up processing on an i960 based AAC adapter
//
#[no_mangle]
unsafe extern "C" fn aac_rx_start_adapter(dev: *mut aac_dev) {
    static void aac_rx_start_adapter(struct aac_dev *dev)
    {
    union aac_init *init;
    init = dev.init;
    init.r7.host_elapsed_seconds = cpu_to_le32(ktime_get_real_seconds());
// We can only use a 32 bit address here
    rx_sync_cmd(dev, INIT_STRUCT_BASE_ADDRESS, (u32)(ulong)dev.init_pa,
    0, 0, 0, 0, 0, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
    }
//
// aac_rx_check_health
// @dev: device to check if healthy
//
// Will attempt to determine if the specified adapter is alive and
// capable of handling requests, returning 0 if alive.
//
#[no_mangle]
unsafe extern "C" fn aac_rx_check_health(dev: *mut aac_dev) -> c_int {
    static int aac_rx_check_health(struct aac_dev *dev)
    {
    let mut status: u32 = rx_readl(dev, MUnit.OMRx[0]);
//
// Check to see if the board failed any self tests.
//
    if (unlikely(status & SELF_TEST_FAILED))
    return -1;
//
// Check to see if the board panic'd.
//
    if (unlikely(status & KERNEL_PANIC)) {
    char * buffer;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct POSTSTATUS {
    pub Post_Command: __le32,
    pub Post_Address: __le32,
    pub post: *mut *mut },
    pub baddr: dma_addr_t paddr,,
    pub ret: c_int,
    if (likely((status & 0xFF000000L) == 0xBC000000L))
    pub 0xFF: return (status >> 16) &,
    buffer = dma_alloc_coherent(&dev.pdev.dev, 512, &baddr,
    pub -2: ret =,
    if (unlikely(buffer == core::ptr::null_mut()))
    pub ret: return,
    post = dma_alloc_coherent(&dev.pdev.dev,
    sizeof(struct POSTSTATUS), &paddr,
    if (unlikely(post == core::ptr::null_mut())) {
    pub baddr): dma_free_coherent(&dev->pdev->dev, 512, buffer,,
    pub ret: return,
    }
    pub 512): memset(buffer, 0,,
    pub cpu_to_le32(COMMAND_POST_RESULTS): post->Post_Command =,
    pub cpu_to_le32(baddr): post->Post_Address =,
    pub paddr): rx_writel(dev, MUnit.IMRx[0],,
    rx_sync_cmd(dev, COMMAND_POST_RESULTS, baddr, 0, 0, 0, 0, 0,
    pub NULL): NULL, NULL, NULL, NULL,,
    dma_free_coherent(&dev.pdev.dev, sizeof(struct POSTSTATUS),
    pub paddr): post,,
    if (likely((buffer[0] == '0') && ((buffer[1] == 'x') || (buffer[1] == 'X')))) {
    ret = (hex_to_bin(buffer[2]) << 4) +
    }
    pub baddr): dma_free_coherent(&dev->pdev->dev, 512, buffer,,
    pub ret: return,
    }
//
// Wait for the adapter to be up and running.
//
    if (unlikely(!(status & KERNEL_UP_AND_RUNNING)))
    pub -3: return,
//
// Everything is OK
//
    pub 0: return,
    }
//
// aac_rx_deliver_producer
// @fib: fib to issue
//
// Will send a fib, returning 0 if successful.
//
#[no_mangle]
pub unsafe extern "C" fn aac_rx_deliver_producer(fib: *mut *mut fib) -> c_int {
    int aac_rx_deliver_producer(struct fib * fib)
    {
    pub fib->dev: *mut *mut aac_dev dev =,
    pub &dev->queues->queue[AdapNormCmdQueue]: *mut *mut aac_queue q =,
    pub Index: u32,
    pub 0: unsigned long nointr =,
    pub &nointr): aac_queue_get( dev, &Index, AdapNormCmdQueue, fib->hw_fib_va, 1, fib,,
// (q->headers.producer) = cpu_to_le32(Index + 1);
    if (!(nointr & aac_config.irq_mod))
    pub AdapNormCmdQueue): aac_adapter_notify(dev,,
    pub 0: return,
    }
//
// aac_rx_deliver_message
// @fib: fib to issue
//
// Will send a fib, returning 0 if successful.
//
#[no_mangle]
unsafe extern "C" fn aac_rx_deliver_message(fib: *mut *mut fib) -> c_int {
    static int aac_rx_deliver_message(struct fib * fib)
    {
    pub fib->dev: *mut *mut aac_dev dev =,
    pub &dev->queues->queue[AdapNormCmdQueue]: *mut *mut aac_queue q =,
    pub Index: u32,
    pub addr: u64,
    pub device: *mut volatile void __iomem,
    pub /: *mut *mut unsigned long count = 10000000L; / 50 seconds,
    pub {: for(;;),
    pub MUnit.InboundQueue): Index = rx_readl(dev,,
    if (unlikely(Index == 0xFFFFFFFFL))
    pub MUnit.InboundQueue): Index = rx_readl(dev,,
    if (likely(Index != 0xFFFFFFFFL))
    if (--count == 0) {
    pub -ETIMEDOUT: return,
    }
    }
    pub Index: device = dev->base +,
    pub fib->hw_fib_pa: addr =,
    pub device): writel((u32)(addr & 0xffffffff),,
    pub sizeof(u32): device +=,
    pub device): writel((u32)(addr >> 32),,
    pub sizeof(u32): device +=,
    pub device): writel(le16_to_cpu(fib->hw_fib_va->header.Size),,
    pub Index): rx_writel(dev, MUnit.InboundQueue,,
    pub 0: return,
    }
//
// aac_rx_ioremap
// @dev: adapter
// @size: mapping resize request
//
#[no_mangle]
unsafe extern "C" fn aac_rx_ioremap(dev: *mut *mut aac_dev, size: u32) -> c_int {
    static int aac_rx_ioremap(struct aac_dev * dev, u32 size)
    {
    if (!size) {
    pub 0: return,
    }
    pub size): dev->base = dev->regs.rx = ioremap(dev->base_start,,
    if (dev.base == core::ptr::null_mut())
    pub -1: return,
    pub &dev->regs.rx->IndexRegs: dev->IndexRegs =,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn aac_rx_restart_adapter(dev: *mut aac_dev, bled: c_int, reset_type: u8) -> c_int {
    static int aac_rx_restart_adapter(struct aac_dev *dev, int bled, u8 reset_type)
    {
    pub 0: u32 var =,
    if (!(dev.supplement_adapter_info.supported_options2 &
    AAC_OPTION_MU_RESET) || (bled >= 0) || (bled == -2)) {
    if (bled)
    printk(KERN_ERR "%s%d: adapter kernel panic'd %x.\n",
    pub bled): dev->name, dev->id,,
    else {
    bled = aac_adapter_sync_cmd(dev, IOP_RESET_ALWAYS,
    pub NULL): 0, 0, 0, 0, 0, 0, &var, NULL, NULL, NULL,,
    if (!bled && (var != 0x00000001) && (var != 0x3803000F))
    pub -EINVAL: bled =,
    }
    if (bled && (bled != -ETIMEDOUT))
    bled = aac_adapter_sync_cmd(dev, IOP_RESET,
    pub NULL): 0, 0, 0, 0, 0, 0, &var, NULL, NULL, NULL,,
    if (bled && (bled != -ETIMEDOUT))
    pub -EINVAL: return,
    }
    if (bled && (var == 0x3803000F)) { /* USE_OTHER_METHOD */
    pub 3): rx_writel(dev, MUnit.reserved2,,
    pub /: *mut *mut msleep(5000); / Delay 5 seconds,
    pub 0x00000001: var =,
    }
    if (bled && (var != 0x00000001))
    pub -EINVAL: return,
    if (rx_readl(dev, MUnit.OMRx[0]) & KERNEL_PANIC)
    pub -ENODEV: return,
    if (startup_timeout < 300)
    pub 300: startup_timeout =,
    pub 0: return,
    }
//
// aac_rx_select_comm	-	Select communications method
// @dev: Adapter
// @comm: communications method
//
#[no_mangle]
pub unsafe extern "C" fn aac_rx_select_comm(dev: *mut aac_dev, comm: c_int) -> c_int {
    int aac_rx_select_comm(struct aac_dev *dev, int comm)
    {
    switch (comm) {
    case AAC_COMM_PRODUCER:
    pub aac_rx_enable_interrupt_producer: dev->a_ops.adapter_enable_int =,
    pub aac_rx_intr_producer: dev->a_ops.adapter_intr =,
    pub aac_rx_deliver_producer: dev->a_ops.adapter_deliver =,
    case AAC_COMM_MESSAGE:
    pub aac_rx_enable_interrupt_message: dev->a_ops.adapter_enable_int =,
    pub aac_rx_intr_message: dev->a_ops.adapter_intr =,
    pub aac_rx_deliver_message: dev->a_ops.adapter_deliver =,
    default:
    pub 1: return,
    }
    pub 0: return,
    }
//
// _aac_rx_init	-	initialize an i960 based AAC card
// @dev: device to configure
//
// Allocate and set up resources for the i960 based AAC variants. The
// device_interface in the commregion will be allocated and linked
// to the comm region.
//
#[no_mangle]
pub unsafe extern "C" fn _aac_rx_init(dev: *mut aac_dev) -> c_int {
    int _aac_rx_init(struct aac_dev *dev)
    {
    pub start: c_ulong,
    pub status: c_ulong,
    pub 0: int restart =,
    pub dev->id: int instance =,
    pub dev->name: *const *const char  name =,
    if (aac_adapter_ioremap(dev, dev.base_size)) {
    pub name): printk(KERN_WARNING "%s: unable to map adapter.\n",,
    pub error_iounmap: goto,
    }
// Failure to reset here is an option ...
    pub rx_sync_cmd: dev->a_ops.adapter_sync_cmd =,
    pub aac_rx_disable_interrupt: dev->a_ops.adapter_enable_int =,
    pub MUnit.OIMR): dev->OIMR = status = rx_readb (dev,,
    if (((status & 0x0c) != 0x0c) || dev.init_reset) {
    pub false: dev->init_reset =,
    if (!aac_rx_restart_adapter(dev, 0, IOP_HWSOFT_RESET)) {
// Make sure the Hardware FIFO is empty
    while ((++restart < 512) &&
    pub 0xFFFFFFFFL)): (rx_readl(dev, MUnit.OutboundQueue) !=,
    }
    }
//
// Check to see if the board panic'd while booting.
//
    pub MUnit.OMRx[0]): status = rx_readl(dev,,
    if (status & KERNEL_PANIC) {
    if (aac_rx_restart_adapter(dev,
    aac_rx_check_health(dev), IOP_HWSOFT_RESET))
    pub error_iounmap: goto,
    }
//
// Check to see if the board failed any self tests.
//
    pub MUnit.OMRx[0]): status = rx_readl(dev,,
    if (status & SELF_TEST_FAILED) {
    pub instance): printk(KERN_ERR "%s%d: adapter self-test failed.\n", dev->name,,
    pub error_iounmap: goto,
    }
//
// Check to see if the monitor panic'd while booting.
//
    if (status & MONITOR_PANIC) {
    pub instance): printk(KERN_ERR "%s%d: adapter monitor panic.\n", dev->name,,
    pub error_iounmap: goto,
    }
    pub jiffies: start =,
//
// Wait for the adapter to be up and running. Wait up to 3 minutes
//
    while (!((status = rx_readl(dev, MUnit.OMRx[0])) & KERNEL_UP_AND_RUNNING))
    {
    if ((restart &&
    (status & (KERNEL_PANIC|SELF_TEST_FAILED|MONITOR_PANIC))) ||
    time_after(jiffies, start+HZ*startup_timeout)) {
    printk(KERN_ERR "%s%d: adapter kernel failed to start, init status = %lx.\n",
    pub status): dev->name, instance,,
    pub error_iounmap: goto,
    }
    if (!restart &&
    ((status & (KERNEL_PANIC|SELF_TEST_FAILED|MONITOR_PANIC)) ||
    time_after(jiffies, start + HZ *
    ((startup_timeout > 60)
    ? (startup_timeout - 60)
    : (startup_timeout / 2))))) {
    if (likely(!aac_rx_restart_adapter(dev,
    aac_rx_check_health(dev), IOP_HWSOFT_RESET)))
    pub jiffies: start =,
    }
    }
    if (restart && aac_commit)
    pub 1: aac_commit =,
//
// Fill in the common function dispatch table.
//
    pub aac_rx_interrupt_adapter: dev->a_ops.adapter_interrupt =,
    pub aac_rx_disable_interrupt: dev->a_ops.adapter_disable_int =,
    pub aac_rx_notify_adapter: dev->a_ops.adapter_notify =,
    pub rx_sync_cmd: dev->a_ops.adapter_sync_cmd =,
    pub aac_rx_check_health: dev->a_ops.adapter_check_health =,
    pub aac_rx_restart_adapter: dev->a_ops.adapter_restart =,
    pub aac_rx_start_adapter: dev->a_ops.adapter_start =,
//
// First clear out all interrupts.  Then enable the one's that we
// can handle.
//
    pub AAC_COMM_PRODUCER): aac_adapter_comm(dev,,
    pub 0xffffffff): rx_writel(dev, MUnit.ODR,,
    if (aac_init_adapter(dev) == core::ptr::null_mut())
    pub error_iounmap: goto,
    pub dev->comm_interface): aac_adapter_comm(dev,,
    pub /: *mut *mut dev->sync_mode = 0; / sync. mode not supported,
    pub !pci_enable_msi(dev->pdev): dev->msi = aac_msi &&,
    if (request_irq(dev.pdev.irq, dev.a_ops.adapter_intr,
    IRQF_SHARED, "aacraid", dev) < 0) {
    if (dev.msi)
    printk(KERN_ERR "%s%d: Interrupt unavailable.\n",
    pub instance): name,,
    pub error_iounmap: goto,
    }
    pub dev->base_start: dev->dbg_base =,
    pub dev->base: dev->dbg_base_mapped =,
    pub dev->base_size: dev->dbg_size =,
//
// Tell the adapter that all is configured, and it can
// start accepting requests
//
    pub 0: return,
    error_iounmap:
    pub -1: return,
    }
#[no_mangle]
pub unsafe extern "C" fn aac_rx_init(dev: *mut aac_dev) -> c_int {
    int aac_rx_init(struct aac_dev *dev)
    {
//
// Fill in the function dispatch table.
//
    pub aac_rx_ioremap: dev->a_ops.adapter_ioremap =,
    pub aac_rx_select_comm: dev->a_ops.adapter_comm =,
    pub _aac_rx_init(dev): return,
    }
