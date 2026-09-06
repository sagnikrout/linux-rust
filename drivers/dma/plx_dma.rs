//! Automatically rewritten from C to Rust
//! Source: drivers/dma/plx_dma.c
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
// Microsemi Switchtec(tm) PCIe Management Driver
// Copyright (c) 2019, Logan Gunthorpe <logang@deltatee.com>
// Copyright (c) 2019, GigaIO Networks, Inc
//

    MODULE_DESCRIPTION("PLX ExpressLane PEX PCI Switch DMA Engine");
    MODULE_VERSION("0.1");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Logan Gunthorpe");
pub const PLX_REG_DESC_RING_ADDR: c_uint = 0x214;
pub const PLX_REG_DESC_RING_ADDR_HI: c_uint = 0x218;
pub const PLX_REG_DESC_RING_NEXT_ADDR: c_uint = 0x21C;
pub const PLX_REG_DESC_RING_COUNT: c_uint = 0x220;
pub const PLX_REG_DESC_RING_LAST_ADDR: c_uint = 0x224;
pub const PLX_REG_DESC_RING_LAST_SIZE: c_uint = 0x228;
pub const PLX_REG_PREF_LIMIT: c_uint = 0x234;
pub const PLX_REG_CTRL: c_uint = 0x238;
pub const PLX_REG_CTRL2: c_uint = 0x23A;
pub const PLX_REG_INTR_CTRL: c_uint = 0x23C;
pub const PLX_REG_INTR_STATUS: c_uint = 0x23E;
pub const PLX_REG_PREF_LIMIT_PREF_FOUR: c_int = 8;

    PLX_REG_CTRL_GRACEFUL_PAUSE_DONE | \
    PLX_REG_CTRL_ABORT_DONE | \
    PLX_REG_CTRL_IMM_PAUSE_DONE)

    PLX_REG_CTRL_DESC_MODE_OFF_CHIP | \
    PLX_REG_CTRL_START | \
    PLX_REG_CTRL_RESET_VAL)
pub const PLX_REG_CTRL2_MAX_TXFR_SIZE_64B: c_int = 0;
pub const PLX_REG_CTRL2_MAX_TXFR_SIZE_128B: c_int = 1;
pub const PLX_REG_CTRL2_MAX_TXFR_SIZE_256B: c_int = 2;
pub const PLX_REG_CTRL2_MAX_TXFR_SIZE_512B: c_int = 3;
pub const PLX_REG_CTRL2_MAX_TXFR_SIZE_1KB: c_int = 4;
pub const PLX_REG_CTRL2_MAX_TXFR_SIZE_2KB: c_int = 5;
pub const PLX_REG_CTRL2_MAX_TXFR_SIZE_4B: c_int = 7;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct plx_dma_hw_std_desc {
    pub flags_and_size: __le32,
    pub dst_addr_hi: __le16,
    pub src_addr_hi: __le16,
    pub dst_addr_lo: __le32,
    pub src_addr_lo: __le32,
}

pub const PLX_DESC_SIZE_MASK: c_uint = 0x7ffffff;

pub const PLX_DMA_RING_COUNT: c_int = 2048;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct plx_dma_desc {
    pub txd: dma_async_tx_descriptor,
    pub hw: *mut plx_dma_hw_std_desc,
    pub orig_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct plx_dma_dev {
    pub dma_dev: dma_device,
    pub dma_chan: dma_chan,
    pub pdev: *mut pci_dev __rcu,
    pub bar: *mut void __iomem,
    pub desc_task: tasklet_struct,
    pub ring_lock: spinlock_t,
    pub ring_active: bool,
    pub head: c_int,
    pub tail: c_int,
    pub hw_ring: *mut plx_dma_hw_std_desc,
    pub hw_ring_dma: dma_addr_t,
    pub desc_ring: *mut plx_dma_desc,
}

    static struct plx_dma_dev *chan_to_plx_dma_dev(struct dma_chan *c)
    {
    return container_of(c, struct plx_dma_dev, dma_chan);
    }
    static struct plx_dma_desc *to_plx_desc(struct dma_async_tx_descriptor *txd)
    {
    return container_of(txd, struct plx_dma_desc, txd);
    }
    static struct plx_dma_desc *plx_dma_get_desc(struct plx_dma_dev *plxdev, int i)
    {
    return plxdev.desc_ring[i & (PLX_DMA_RING_COUNT - 1)];
    }
#[no_mangle]
unsafe extern "C" fn plx_dma_process_desc(plxdev: *mut plx_dma_dev) {
    static void plx_dma_process_desc(struct plx_dma_dev *plxdev)
    {
    struct dmaengine_result res;
    struct plx_dma_desc *desc;
    u32 flags;
    spin_lock(&plxdev.ring_lock);
    while (plxdev.tail != plxdev.head) {
    desc = plx_dma_get_desc(plxdev, plxdev.tail);
    flags = le32_to_cpu(READ_ONCE(desc.hw.flags_and_size));
    if (flags & PLX_DESC_FLAG_VALID)
    break;
    res.residue = desc.orig_size - (flags & PLX_DESC_SIZE_MASK);
    if (flags & PLX_DESC_WB_SUCCESS)
    res.result = DMA_TRANS_NOERROR;
#[no_mangle]
pub unsafe extern "C" fn if(PLX_DESC_WB_WR_FAIL: flags &) -> else {
    else if (flags & PLX_DESC_WB_WR_FAIL)
    res.result = DMA_TRANS_WRITE_FAILED;
    else
    res.result = DMA_TRANS_READ_FAILED;
    dma_cookie_complete(&desc.txd);
    dma_descriptor_unmap(&desc.txd);
    dmaengine_desc_get_callback_invoke(&desc.txd, &res);
    desc.txd.callback = core::ptr::null_mut();
    desc.txd.callback_result = core::ptr::null_mut();
    plxdev.tail++;
    }
    spin_unlock(&plxdev.ring_lock);
    }
#[no_mangle]
unsafe extern "C" fn plx_dma_abort_desc(plxdev: *mut plx_dma_dev) {
    static void plx_dma_abort_desc(struct plx_dma_dev *plxdev)
    {
    struct dmaengine_result res;
    struct plx_dma_desc *desc;
    plx_dma_process_desc(plxdev);
    spin_lock_bh(&plxdev.ring_lock);
    while (plxdev.tail != plxdev.head) {
    desc = plx_dma_get_desc(plxdev, plxdev.tail);
    res.residue = desc.orig_size;
    res.result = DMA_TRANS_ABORTED;
    dma_cookie_complete(&desc.txd);
    dma_descriptor_unmap(&desc.txd);
    dmaengine_desc_get_callback_invoke(&desc.txd, &res);
    desc.txd.callback = core::ptr::null_mut();
    desc.txd.callback_result = core::ptr::null_mut();
    plxdev.tail++;
    }
    spin_unlock_bh(&plxdev.ring_lock);
    }
#[no_mangle]
unsafe extern "C" fn __plx_dma_stop(plxdev: *mut plx_dma_dev) {
    static void __plx_dma_stop(struct plx_dma_dev *plxdev)
    {
    let mut timeout: c_ulong = jiffies + msecs_to_jiffies(1000);
    u32 val;
    val = readl(plxdev.bar + PLX_REG_CTRL);
    if (!(val & ~PLX_REG_CTRL_GRACEFUL_PAUSE))
    return;
    writel(PLX_REG_CTRL_RESET_VAL | PLX_REG_CTRL_GRACEFUL_PAUSE,
    plxdev.bar + PLX_REG_CTRL);
    while (!time_after(jiffies, timeout)) {
    val = readl(plxdev.bar + PLX_REG_CTRL);
    if (val & PLX_REG_CTRL_GRACEFUL_PAUSE_DONE)
    break;
    cpu_relax();
    }
    if (!(val & PLX_REG_CTRL_GRACEFUL_PAUSE_DONE))
    dev_err(plxdev.dma_dev.dev,
    "Timeout waiting for graceful pause!\n");
    writel(PLX_REG_CTRL_RESET_VAL | PLX_REG_CTRL_GRACEFUL_PAUSE,
    plxdev.bar + PLX_REG_CTRL);
    writel(0, plxdev.bar + PLX_REG_DESC_RING_COUNT);
    writel(0, plxdev.bar + PLX_REG_DESC_RING_ADDR);
    writel(0, plxdev.bar + PLX_REG_DESC_RING_ADDR_HI);
    writel(0, plxdev.bar + PLX_REG_DESC_RING_NEXT_ADDR);
    }
#[no_mangle]
unsafe extern "C" fn plx_dma_stop(plxdev: *mut plx_dma_dev) {
    static void plx_dma_stop(struct plx_dma_dev *plxdev)
    {
    rcu_read_lock();
    if (!rcu_dereference(plxdev.pdev)) {
    rcu_read_unlock();
    return;
    }
    __plx_dma_stop(plxdev);
    rcu_read_unlock();
    }
#[no_mangle]
unsafe extern "C" fn plx_dma_desc_task(t: *mut tasklet_struct) {
    static void plx_dma_desc_task(struct tasklet_struct *t)
    {
    struct plx_dma_dev *plxdev = from_tasklet(plxdev, t, desc_task);
    plx_dma_process_desc(plxdev);
    }
    static struct dma_async_tx_descriptor *plx_dma_prep_memcpy(struct dma_chan *c,
    dma_addr_t dma_dst, dma_addr_t dma_src, size_t len,
    unsigned long flags)
    __acquires(plxdev.ring_lock)
    {
    struct plx_dma_dev *plxdev = chan_to_plx_dma_dev(c);
    struct plx_dma_desc *plxdesc;
    spin_lock_bh(&plxdev.ring_lock);
    if (!plxdev.ring_active)
    goto err_unlock;
    if (!CIRC_SPACE(plxdev.head, plxdev.tail, PLX_DMA_RING_COUNT))
    goto err_unlock;
    if (len > PLX_DESC_SIZE_MASK)
    goto err_unlock;
    plxdesc = plx_dma_get_desc(plxdev, plxdev.head);
    plxdev.head++;
    plxdesc.hw.dst_addr_lo = cpu_to_le32(lower_32_bits(dma_dst));
    plxdesc.hw.dst_addr_hi = cpu_to_le16(upper_32_bits(dma_dst));
    plxdesc.hw.src_addr_lo = cpu_to_le32(lower_32_bits(dma_src));
    plxdesc.hw.src_addr_hi = cpu_to_le16(upper_32_bits(dma_src));
    plxdesc.orig_size = len;
    if (flags & DMA_PREP_INTERRUPT)
    len |= PLX_DESC_FLAG_INT_WHEN_DONE;
    plxdesc.hw.flags_and_size = cpu_to_le32(len);
    plxdesc.txd.flags = flags;
// return with the lock held, it will be released in tx_submit
    return &plxdesc.txd;
    err_unlock:
//
// Keep sparse happy by restoring an even lock count on
// this lock.
//
    __acquire(plxdev.ring_lock);
    spin_unlock_bh(&plxdev.ring_lock);
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn plx_dma_tx_submit(desc: *mut dma_async_tx_descriptor) -> dma_cookie_t {
    static dma_cookie_t plx_dma_tx_submit(struct dma_async_tx_descriptor *desc)
    __releases(plxdev.ring_lock)
    {
    struct plx_dma_dev *plxdev = chan_to_plx_dma_dev(desc.chan);
    struct plx_dma_desc *plxdesc = to_plx_desc(desc);
    dma_cookie_t cookie;
    cookie = dma_cookie_assign(desc);
//
// Ensure the descriptor updates are visible to the dma device
// before setting the valid bit.
//
    wmb();
    plxdesc.hw.flags_and_size |= cpu_to_le32(PLX_DESC_FLAG_VALID);
    spin_unlock_bh(&plxdev.ring_lock);
    return cookie;
    }
    static enum dma_status plx_dma_tx_status(struct dma_chan *chan,
    dma_cookie_t cookie, struct dma_tx_state *txstate)
    {
    struct plx_dma_dev *plxdev = chan_to_plx_dma_dev(chan);
    enum dma_status ret;
    ret = dma_cookie_status(chan, cookie, txstate);
    if (ret == DMA_COMPLETE)
    return ret;
    plx_dma_process_desc(plxdev);
    return dma_cookie_status(chan, cookie, txstate);
    }
#[no_mangle]
unsafe extern "C" fn plx_dma_issue_pending(chan: *mut dma_chan) {
    static void plx_dma_issue_pending(struct dma_chan *chan)
    {
    struct plx_dma_dev *plxdev = chan_to_plx_dma_dev(chan);
    rcu_read_lock();
    if (!rcu_dereference(plxdev.pdev)) {
    rcu_read_unlock();
    return;
    }
//
// Ensure the valid bits are visible before starting the
// DMA engine.
//
    wmb();
    writew(PLX_REG_CTRL_START_VAL, plxdev.bar + PLX_REG_CTRL);
    rcu_read_unlock();
    }
#[no_mangle]
unsafe extern "C" fn plx_dma_isr(irq: c_int, devid: *mut c_void) -> irqreturn_t {
    static irqreturn_t plx_dma_isr(int irq, void *devid)
    {
    struct plx_dma_dev *plxdev = devid;
    u32 status;
    status = readw(plxdev.bar + PLX_REG_INTR_STATUS);
    if (!status)
    return IRQ_NONE;
    if (status & PLX_REG_INTR_STATUS_DESC_DONE && plxdev.ring_active)
    tasklet_schedule(&plxdev.desc_task);
    writew(status, plxdev.bar + PLX_REG_INTR_STATUS);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn plx_dma_alloc_desc(plxdev: *mut plx_dma_dev) -> c_int {
    static int plx_dma_alloc_desc(struct plx_dma_dev *plxdev)
    {
    struct plx_dma_desc *desc;
    int i;
    plxdev.desc_ring = kzalloc_objs(*plxdev.desc_ring, PLX_DMA_RING_COUNT);
    if (!plxdev.desc_ring)
    return -ENOMEM;
    for (i = 0; i < PLX_DMA_RING_COUNT; i++) {
    desc = kzalloc_obj(*desc);
    if (!desc)
    goto free_and_exit;
    dma_async_tx_descriptor_init(&desc.txd, &plxdev.dma_chan);
    desc.txd.tx_submit = plx_dma_tx_submit;
    desc.hw = &plxdev.hw_ring[i];
    plxdev.desc_ring[i] = desc;
    }
    return 0;
    free_and_exit:
    for (i = 0; i < PLX_DMA_RING_COUNT; i++)
    kfree(plxdev.desc_ring[i]);
    kfree(plxdev.desc_ring);
    return -ENOMEM;
    }
#[no_mangle]
unsafe extern "C" fn plx_dma_alloc_chan_resources(chan: *mut dma_chan) -> c_int {
    static int plx_dma_alloc_chan_resources(struct dma_chan *chan)
    {
    struct plx_dma_dev *plxdev = chan_to_plx_dma_dev(chan);
    let mut ring_sz: usize = PLX_DMA_RING_COUNT * sizeof(*plxdev.hw_ring);
    int rc;
    plxdev.head = plxdev.tail = 0;
    plxdev.hw_ring = dma_alloc_coherent(plxdev.dma_dev.dev, ring_sz,
    &plxdev.hw_ring_dma, GFP_KERNEL);
    if (!plxdev.hw_ring)
    return -ENOMEM;
    rc = plx_dma_alloc_desc(plxdev);
    if (rc)
    goto out_free_hw_ring;
    rcu_read_lock();
    if (!rcu_dereference(plxdev.pdev)) {
    rcu_read_unlock();
    rc = -ENODEV;
    goto out_free_hw_ring;
    }
    writel(PLX_REG_CTRL_RESET_VAL, plxdev.bar + PLX_REG_CTRL);
    writel(lower_32_bits(plxdev.hw_ring_dma),
    plxdev.bar + PLX_REG_DESC_RING_ADDR);
    writel(upper_32_bits(plxdev.hw_ring_dma),
    plxdev.bar + PLX_REG_DESC_RING_ADDR_HI);
    writel(lower_32_bits(plxdev.hw_ring_dma),
    plxdev.bar + PLX_REG_DESC_RING_NEXT_ADDR);
    writel(PLX_DMA_RING_COUNT, plxdev.bar + PLX_REG_DESC_RING_COUNT);
    writel(PLX_REG_PREF_LIMIT_PREF_FOUR, plxdev.bar + PLX_REG_PREF_LIMIT);
    plxdev.ring_active = true;
    rcu_read_unlock();
    return PLX_DMA_RING_COUNT;
    out_free_hw_ring:
    dma_free_coherent(plxdev.dma_dev.dev, ring_sz, plxdev.hw_ring,
    plxdev.hw_ring_dma);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn plx_dma_free_chan_resources(chan: *mut dma_chan) {
    static void plx_dma_free_chan_resources(struct dma_chan *chan)
    {
    struct plx_dma_dev *plxdev = chan_to_plx_dma_dev(chan);
    let mut ring_sz: usize = PLX_DMA_RING_COUNT * sizeof(*plxdev.hw_ring);
    struct pci_dev *pdev;
    let mut irq: c_int = -1;
    int i;
    spin_lock_bh(&plxdev.ring_lock);
    plxdev.ring_active = false;
    spin_unlock_bh(&plxdev.ring_lock);
    plx_dma_stop(plxdev);
    rcu_read_lock();
    pdev = rcu_dereference(plxdev.pdev);
    if (pdev)
    irq = pci_irq_vector(pdev, 0);
    rcu_read_unlock();
    if (irq > 0)
    synchronize_irq(irq);
    tasklet_kill(&plxdev.desc_task);
    plx_dma_abort_desc(plxdev);
    for (i = 0; i < PLX_DMA_RING_COUNT; i++)
    kfree(plxdev.desc_ring[i]);
    kfree(plxdev.desc_ring);
    dma_free_coherent(plxdev.dma_dev.dev, ring_sz, plxdev.hw_ring,
    plxdev.hw_ring_dma);
    }
#[no_mangle]
unsafe extern "C" fn plx_dma_release(dma_dev: *mut dma_device) {
    static void plx_dma_release(struct dma_device *dma_dev)
    {
    struct plx_dma_dev *plxdev =
    container_of(dma_dev, struct plx_dma_dev, dma_dev);
    put_device(dma_dev.dev);
    kfree(plxdev);
    }
#[no_mangle]
unsafe extern "C" fn plx_dma_create(pdev: *mut pci_dev) -> c_int {
    static int plx_dma_create(struct pci_dev *pdev)
    {
    struct plx_dma_dev *plxdev;
    struct dma_device *dma;
    struct dma_chan *chan;
    int rc;
    plxdev = kzalloc_obj(*plxdev);
    if (!plxdev)
    return -ENOMEM;
    rc = request_irq(pci_irq_vector(pdev, 0), plx_dma_isr, 0,
    KBUILD_MODNAME, plxdev);
    if (rc)
    goto free_plx;
    spin_lock_init(&plxdev.ring_lock);
    tasklet_setup(&plxdev.desc_task, plx_dma_desc_task);
    RCU_INIT_POINTER(plxdev.pdev, pdev);
    plxdev.bar = pcim_iomap_table(pdev)[0];
    dma = &plxdev.dma_dev;
    INIT_LIST_HEAD(&dma.channels);
    dma_cap_set(DMA_MEMCPY, dma.cap_mask);
    dma.copy_align = DMAENGINE_ALIGN_1_BYTE;
    dma.dev = get_device(&pdev.dev);
    dma.device_alloc_chan_resources = plx_dma_alloc_chan_resources;
    dma.device_free_chan_resources = plx_dma_free_chan_resources;
    dma.device_prep_dma_memcpy = plx_dma_prep_memcpy;
    dma.device_issue_pending = plx_dma_issue_pending;
    dma.device_tx_status = plx_dma_tx_status;
    dma.device_release = plx_dma_release;
    chan = &plxdev.dma_chan;
    chan.device = dma;
    dma_cookie_init(chan);
    list_add_tail(&chan.device_node, &dma.channels);
    rc = dma_async_device_register(dma);
    if (rc) {
    pci_err(pdev, "Failed to register dma device: %d\n", rc);
    goto put_device;
    }
    pci_set_drvdata(pdev, plxdev);
    return 0;
    put_device:
    put_device(&pdev.dev);
    free_irq(pci_irq_vector(pdev, 0),  plxdev);
    free_plx:
    kfree(plxdev);
    return rc;
    }
    static int plx_dma_probe(struct pci_dev *pdev,
    const struct pci_device_id *id)
    {
    int rc;
    rc = pcim_enable_device(pdev);
    if (rc)
    return rc;
    rc = dma_set_mask_and_coherent(&pdev.dev, DMA_BIT_MASK(48));
    if (rc)
    rc = dma_set_mask_and_coherent(&pdev.dev, DMA_BIT_MASK(32));
    if (rc)
    return rc;
    rc = pcim_iomap_regions(pdev, 1, KBUILD_MODNAME);
    if (rc)
    return rc;
    rc = pci_alloc_irq_vectors(pdev, 1, 1, PCI_IRQ_ALL_TYPES);
    if (rc <= 0)
    return rc;
    pci_set_master(pdev);
    rc = plx_dma_create(pdev);
    if (rc)
    goto err_free_irq_vectors;
    pci_info(pdev, "PLX DMA Channel Registered\n");
    return 0;
    err_free_irq_vectors:
    pci_free_irq_vectors(pdev);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn plx_dma_remove(pdev: *mut pci_dev) {
    static void plx_dma_remove(struct pci_dev *pdev)
    {
    struct plx_dma_dev *plxdev = pci_get_drvdata(pdev);
    free_irq(pci_irq_vector(pdev, 0),  plxdev);
    rcu_assign_pointer(plxdev.pdev, core::ptr::null_mut());
    synchronize_rcu();
    spin_lock_bh(&plxdev.ring_lock);
    plxdev.ring_active = false;
    spin_unlock_bh(&plxdev.ring_lock);
    __plx_dma_stop(plxdev);
    plx_dma_abort_desc(plxdev);
    plxdev.bar = core::ptr::null_mut();
    dma_async_device_unregister(&plxdev.dma_dev);
    pci_free_irq_vectors(pdev);
    }
    static const struct pci_device_id plx_dma_pci_tbl[] = {
    {
    .vendor		= PCI_VENDOR_ID_PLX,
    .device		= 0x87D0,
    .subvendor	= PCI_ANY_ID,
    .subdevice	= PCI_ANY_ID,
    .class		= PCI_CLASS_SYSTEM_OTHER << 8,
    .class_mask	= 0xFFFFFFFF,
    },
    {0}
    };
    MODULE_DEVICE_TABLE(pci, plx_dma_pci_tbl);
    static struct pci_driver plx_dma_pci_driver = {
    .name           = KBUILD_MODNAME,
    .id_table       = plx_dma_pci_tbl,
    .probe          = plx_dma_probe,
    .remove		= plx_dma_remove,
    };
    module_pci_driver(plx_dma_pci_driver);
