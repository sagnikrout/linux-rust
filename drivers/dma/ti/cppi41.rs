//! Automatically rewritten from C to Rust
//! Source: drivers/dma/ti/cppi41.c
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

pub const DESC_TYPE: c_int = 27;
pub const DESC_TYPE_HOST: c_uint = 0x10;
pub const DESC_TYPE_TEARD: c_uint = 0x13;

pub const TD_DESC_DMA_NUM: c_int = 10;
pub const DESC_LENGTH_BITS_NUM: c_int = 21;

// DMA engine
pub const DMA_TDFDQ: c_int = 4;

pub const RXHPCRA0: c_int = 4;

// DMA scheduler
pub const DMA_SCHED_CTRL: c_int = 0;

// Queue manager
// 4 KiB of memory for descriptors, 2 for each endpoint
pub const ALLOC_DECS_NUM: c_int = 128;
pub const DESCS_AREAS: c_int = 1;

pub const QMGR_LRAM0_BASE: c_uint = 0x80;
pub const QMGR_LRAM_SIZE: c_uint = 0x84;
pub const QMGR_LRAM1_BASE: c_uint = 0x88;

pub const QMGR_MEMCTRL_IDX_SH: c_int = 16;
pub const QMGR_MEMCTRL_DESC_SH: c_int = 8;

// Packet Descriptor

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cppi41_channel {
    pub chan: dma_chan,
    pub txd: dma_async_tx_descriptor,
    pub cdd: *mut cppi41_dd,
    pub desc: *mut cppi41_desc,
    pub desc_phys: dma_addr_t,
    pub gcr_reg: *mut void __iomem,
    pub is_tx: c_int,
    pub residue: u32,
    pub q_num: c_uint,
    pub q_comp_num: c_uint,
    pub port_num: c_uint,
    pub td_retry: unsigned,
    pub td_queued:1: unsigned,
    pub td_seen:1: unsigned,
    pub td_desc_seen:1: unsigned,
    pub /: *mut *mut list_head node; / Node for pending list,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cppi41_desc {
    pub pd0: u32,
    pub pd1: u32,
    pub pd2: u32,
    pub pd3: u32,
    pub pd4: u32,
    pub pd5: u32,
    pub pd6: u32,
    pub pd7: u32,
    pub __aligned(32): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct chan_queues {
    pub submit: u16,
    pub complete: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cppi41_dd {
    pub ddev: dma_device,
    pub qmgr_scratch: *mut c_void,
    pub scratch_phys: dma_addr_t,
    pub cd: *mut cppi41_desc,
    pub descs_phys: dma_addr_t,
    pub first_td_desc: u32,
    pub chan_busy: [*mut cppi41_channel; ALLOC_DECS_NUM],
    pub ctrl_mem: *mut void __iomem,
    pub sched_mem: *mut void __iomem,
    pub qmgr_mem: *mut void __iomem,
    pub irq: c_uint,
    pub queues_rx: *const chan_queues,
    pub queues_tx: *const chan_queues,
    pub td_queue: chan_queues,
    pub first_completion_queue: u16,
    pub qmgr_num_pend: u16,
    pub n_chans: u32,
    pub platform: u8,
    pub /: *mut *mut list_head pending; / Pending queued transfers,
    pub /: *mut *mut spinlock_t lock; / Lock for pending list,
// context for suspend/resume
    pub dma_tdfdq: c_uint,
    pub is_suspended: bool,
}

    static struct chan_queues am335x_usb_queues_tx[] = {
// USB0 ENDP 1
    [ 0] = { .submit = 32, .complete =  93},
    [ 1] = { .submit = 34, .complete =  94},
    [ 2] = { .submit = 36, .complete =  95},
    [ 3] = { .submit = 38, .complete =  96},
    [ 4] = { .submit = 40, .complete =  97},
    [ 5] = { .submit = 42, .complete =  98},
    [ 6] = { .submit = 44, .complete =  99},
    [ 7] = { .submit = 46, .complete = 100},
    [ 8] = { .submit = 48, .complete = 101},
    [ 9] = { .submit = 50, .complete = 102},
    [10] = { .submit = 52, .complete = 103},
    [11] = { .submit = 54, .complete = 104},
    [12] = { .submit = 56, .complete = 105},
    [13] = { .submit = 58, .complete = 106},
    [14] = { .submit = 60, .complete = 107},
// USB1 ENDP1
    [15] = { .submit = 62, .complete = 125},
    [16] = { .submit = 64, .complete = 126},
    [17] = { .submit = 66, .complete = 127},
    [18] = { .submit = 68, .complete = 128},
    [19] = { .submit = 70, .complete = 129},
    [20] = { .submit = 72, .complete = 130},
    [21] = { .submit = 74, .complete = 131},
    [22] = { .submit = 76, .complete = 132},
    [23] = { .submit = 78, .complete = 133},
    [24] = { .submit = 80, .complete = 134},
    [25] = { .submit = 82, .complete = 135},
    [26] = { .submit = 84, .complete = 136},
    [27] = { .submit = 86, .complete = 137},
    [28] = { .submit = 88, .complete = 138},
    [29] = { .submit = 90, .complete = 139},
    };
    static const struct chan_queues am335x_usb_queues_rx[] = {
// USB0 ENDP 1
    [ 0] = { .submit =  1, .complete = 109},
    [ 1] = { .submit =  2, .complete = 110},
    [ 2] = { .submit =  3, .complete = 111},
    [ 3] = { .submit =  4, .complete = 112},
    [ 4] = { .submit =  5, .complete = 113},
    [ 5] = { .submit =  6, .complete = 114},
    [ 6] = { .submit =  7, .complete = 115},
    [ 7] = { .submit =  8, .complete = 116},
    [ 8] = { .submit =  9, .complete = 117},
    [ 9] = { .submit = 10, .complete = 118},
    [10] = { .submit = 11, .complete = 119},
    [11] = { .submit = 12, .complete = 120},
    [12] = { .submit = 13, .complete = 121},
    [13] = { .submit = 14, .complete = 122},
    [14] = { .submit = 15, .complete = 123},
// USB1 ENDP 1
    [15] = { .submit = 16, .complete = 141},
    [16] = { .submit = 17, .complete = 142},
    [17] = { .submit = 18, .complete = 143},
    [18] = { .submit = 19, .complete = 144},
    [19] = { .submit = 20, .complete = 145},
    [20] = { .submit = 21, .complete = 146},
    [21] = { .submit = 22, .complete = 147},
    [22] = { .submit = 23, .complete = 148},
    [23] = { .submit = 24, .complete = 149},
    [24] = { .submit = 25, .complete = 150},
    [25] = { .submit = 26, .complete = 151},
    [26] = { .submit = 27, .complete = 152},
    [27] = { .submit = 28, .complete = 153},
    [28] = { .submit = 29, .complete = 154},
    [29] = { .submit = 30, .complete = 155},
    };
    static const struct chan_queues da8xx_usb_queues_tx[] = {
    [0] = { .submit =  16, .complete = 24},
    [1] = { .submit =  18, .complete = 24},
    [2] = { .submit =  20, .complete = 24},
    [3] = { .submit =  22, .complete = 24},
    };
    static const struct chan_queues da8xx_usb_queues_rx[] = {
    [0] = { .submit =  1, .complete = 26},
    [1] = { .submit =  3, .complete = 26},
    [2] = { .submit =  5, .complete = 26},
    [3] = { .submit =  7, .complete = 26},
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cppi_glue_infos {
    pub queues_rx: *const chan_queues,
    pub queues_tx: *const chan_queues,
    pub td_queue: chan_queues,
    pub first_completion_queue: u16,
    pub qmgr_num_pend: u16,
}

    static struct cppi41_channel *to_cpp41_chan(struct dma_chan *c)
    {
    return container_of(c, struct cppi41_channel, chan);
    }
    static struct cppi41_channel *desc_to_chan(struct cppi41_dd *cdd, u32 desc)
    {
    struct cppi41_channel *c;
    u32 descs_size;
    u32 desc_num;
    descs_size = sizeof(struct cppi41_desc) * ALLOC_DECS_NUM;
    if (!((desc >= cdd.descs_phys) &&
    (desc < (cdd.descs_phys + descs_size)))) {
    return core::ptr::null_mut();
    }
    desc_num = (desc - cdd.descs_phys) / sizeof(struct cppi41_desc);
    BUG_ON(desc_num >= ALLOC_DECS_NUM);
    c = cdd.chan_busy[desc_num];
    cdd.chan_busy[desc_num] = core::ptr::null_mut();
// Usecount for chan_busy[], paired with push_desc_queue()
    pm_runtime_put(cdd.ddev.dev);
    return c;
    }
#[no_mangle]
unsafe extern "C" fn cppi_writel(val: u32, mem: *mut *mut void __iomem) {
    static void cppi_writel(u32 val, void *__iomem *mem)
    {
    __raw_writel(val, mem);
    }
#[no_mangle]
unsafe extern "C" fn cppi_readl(mem: *mut *mut void __iomem) -> u32 {
    static u32 cppi_readl(void *__iomem *mem)
    {
    return __raw_readl(mem);
    }
#[no_mangle]
unsafe extern "C" fn pd_trans_len(val: u32) -> u32 {
    static u32 pd_trans_len(u32 val)
    {
    return val & ((1 << (DESC_LENGTH_BITS_NUM + 1)) - 1);
    }
#[no_mangle]
unsafe extern "C" fn cppi41_pop_desc(cdd: *mut cppi41_dd, queue_num: unsigned) -> u32 {
    static u32 cppi41_pop_desc(struct cppi41_dd *cdd, unsigned queue_num)
    {
    u32 desc;
    desc = cppi_readl(cdd.qmgr_mem + QMGR_QUEUE_D(queue_num));
    desc &= ~0x1f;
    return desc;
    }
#[no_mangle]
unsafe extern "C" fn cppi41_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t cppi41_irq(int irq, void *data)
    {
    struct cppi41_dd *cdd = data;
    let mut first_completion_queue: u16 = cdd.first_completion_queue;
    let mut qmgr_num_pend: u16 = cdd.qmgr_num_pend;
    struct cppi41_channel *c;
    int i;
    for (i = QMGR_PENDING_SLOT_Q(first_completion_queue); i < qmgr_num_pend;
    i++) {
    u32 val;
    u32 q_num;
    val = cppi_readl(cdd.qmgr_mem + QMGR_PEND(i));
    if (i == QMGR_PENDING_SLOT_Q(first_completion_queue) && val) {
    u32 mask;
// set corresponding bit for completion Q 93
    mask = 1 << QMGR_PENDING_BIT_Q(first_completion_queue);
// not set all bits for queues less than Q 93
    mask--;
// now invert and keep only Q 93+ set
    val &= ~mask;
    }
    if (val)
    __iormb();
    while (val) {
    u32 desc, len;
//
// This should never trigger, see the comments in
// push_desc_queue()
//
    WARN_ON(cdd.is_suspended);
    q_num = __fls(val);
    val &= ~(1 << q_num);
    q_num += 32 * i;
    desc = cppi41_pop_desc(cdd, q_num);
    c = desc_to_chan(cdd, desc);
    if (WARN_ON(!c)) {
    pr_err("%s() q %d desc %08x\n", __func__,
    q_num, desc);
    continue;
    }
    if (c.desc.pd2 & PD2_ZERO_LENGTH)
    len = 0;
    else
    len = pd_trans_len(c.desc.pd0);
    c.residue = pd_trans_len(c.desc.pd6) - len;
    dma_cookie_complete(&c.txd);
    dmaengine_desc_get_callback_invoke(&c.txd, core::ptr::null_mut());
    }
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn cppi41_tx_submit(tx: *mut dma_async_tx_descriptor) -> dma_cookie_t {
    static dma_cookie_t cppi41_tx_submit(struct dma_async_tx_descriptor *tx)
    {
    dma_cookie_t cookie;
    cookie = dma_cookie_assign(tx);
    return cookie;
    }
#[no_mangle]
unsafe extern "C" fn cppi41_dma_alloc_chan_resources(chan: *mut dma_chan) -> c_int {
    static int cppi41_dma_alloc_chan_resources(struct dma_chan *chan)
    {
    struct cppi41_channel *c = to_cpp41_chan(chan);
    struct cppi41_dd *cdd = c.cdd;
    int error;
    error = pm_runtime_get_sync(cdd.ddev.dev);
    if (error < 0) {
    dev_err(cdd.ddev.dev, "%s pm runtime get: %i\n",
    __func__, error);
    pm_runtime_put_noidle(cdd.ddev.dev);
    return error;
    }
    dma_cookie_init(chan);
    dma_async_tx_descriptor_init(&c.txd, chan);
    c.txd.tx_submit = cppi41_tx_submit;
    if (!c.is_tx)
    cppi_writel(c.q_num, c.gcr_reg + RXHPCRA0);
    pm_runtime_put_autosuspend(cdd.ddev.dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cppi41_dma_free_chan_resources(chan: *mut dma_chan) {
    static void cppi41_dma_free_chan_resources(struct dma_chan *chan)
    {
    struct cppi41_channel *c = to_cpp41_chan(chan);
    struct cppi41_dd *cdd = c.cdd;
    int error;
    error = pm_runtime_get_sync(cdd.ddev.dev);
    if (error < 0) {
    pm_runtime_put_noidle(cdd.ddev.dev);
    return;
    }
    WARN_ON(!list_empty(&cdd.pending));
    pm_runtime_put_autosuspend(cdd.ddev.dev);
    }
    static enum dma_status cppi41_dma_tx_status(struct dma_chan *chan,
    dma_cookie_t cookie, struct dma_tx_state *txstate)
    {
    struct cppi41_channel *c = to_cpp41_chan(chan);
    enum dma_status ret;
    ret = dma_cookie_status(chan, cookie, txstate);
    dma_set_residue(txstate, c.residue);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn push_desc_queue(c: *mut cppi41_channel) {
    static void push_desc_queue(struct cppi41_channel *c)
    {
    struct cppi41_dd *cdd = c.cdd;
    u32 desc_num;
    u32 desc_phys;
    u32 reg;
    c.residue = 0;
    reg = GCR_CHAN_ENABLE;
    if (!c.is_tx) {
    reg |= GCR_STARV_RETRY;
    reg |= GCR_DESC_TYPE_HOST;
    reg |= c.q_comp_num;
    }
    cppi_writel(reg, c.gcr_reg);
//
// We don't use writel() but __raw_writel() so we have to make sure
// that the DMA descriptor in coherent memory made to the main memory
// before starting the dma engine.
//
    __iowmb();
//
// DMA transfers can take at least 200ms to complete with USB mass
// storage connected. To prevent autosuspend timeouts, we must use
// pm_runtime_get/put() when chan_busy[] is modified. This will get
// cleared in desc_to_chan() or cppi41_stop_chan() depending on the
// outcome of the transfer.
//
    pm_runtime_get(cdd.ddev.dev);
    desc_phys = lower_32_bits(c.desc_phys);
    desc_num = (desc_phys - cdd.descs_phys) / sizeof(struct cppi41_desc);
    WARN_ON(cdd.chan_busy[desc_num]);
    cdd.chan_busy[desc_num] = c;
    reg = (sizeof(struct cppi41_desc) - 24) / 4;
    reg |= desc_phys;
    cppi_writel(reg, cdd.qmgr_mem + QMGR_QUEUE_D(c.q_num));
    }
//
// Caller must hold cdd->lock to prevent push_desc_queue()
// getting called out of order. We have both cppi41_dma_issue_pending()
// and cppi41_runtime_resume() call this function.
//
#[no_mangle]
unsafe extern "C" fn cppi41_run_queue(cdd: *mut cppi41_dd) {
    static void cppi41_run_queue(struct cppi41_dd *cdd)
    {
    struct cppi41_channel *c, *_c;
    list_for_each_entry_safe(c, _c, &cdd.pending, node) {
    push_desc_queue(c);
    list_del(&c.node);
    }
    }
#[no_mangle]
unsafe extern "C" fn cppi41_dma_issue_pending(chan: *mut dma_chan) {
    static void cppi41_dma_issue_pending(struct dma_chan *chan)
    {
    struct cppi41_channel *c = to_cpp41_chan(chan);
    struct cppi41_dd *cdd = c.cdd;
    unsigned long flags;
    int error;
    error = pm_runtime_get(cdd.ddev.dev);
    if ((error != -EINPROGRESS) && error < 0) {
    pm_runtime_put_noidle(cdd.ddev.dev);
    dev_err(cdd.ddev.dev, "Failed to pm_runtime_get: %i\n",
    error);
    return;
    }
    spin_lock_irqsave(&cdd.lock, flags);
    list_add_tail(&c.node, &cdd.pending);
    if (!cdd.is_suspended)
    cppi41_run_queue(cdd);
    spin_unlock_irqrestore(&cdd.lock, flags);
    pm_runtime_put_autosuspend(cdd.ddev.dev);
    }
#[no_mangle]
unsafe extern "C" fn get_host_pd0(length: u32) -> u32 {
    static u32 get_host_pd0(u32 length)
    {
    u32 reg;
    reg = DESC_TYPE_HOST << DESC_TYPE;
    reg |= length;
    return reg;
    }
#[no_mangle]
unsafe extern "C" fn get_host_pd1(c: *mut cppi41_channel) -> u32 {
    static u32 get_host_pd1(struct cppi41_channel *c)
    {
    u32 reg;
    reg = 0;
    return reg;
    }
#[no_mangle]
unsafe extern "C" fn get_host_pd2(c: *mut cppi41_channel) -> u32 {
    static u32 get_host_pd2(struct cppi41_channel *c)
    {
    u32 reg;
    reg = DESC_TYPE_USB;
    reg |= c.q_comp_num;
    return reg;
    }
#[no_mangle]
unsafe extern "C" fn get_host_pd3(length: u32) -> u32 {
    static u32 get_host_pd3(u32 length)
    {
    u32 reg;
// PD3 = packet size
    reg = length;
    return reg;
    }
#[no_mangle]
unsafe extern "C" fn get_host_pd6(length: u32) -> u32 {
    static u32 get_host_pd6(u32 length)
    {
    u32 reg;
// PD6 buffer size
    reg = DESC_PD_COMPLETE;
    reg |= length;
    return reg;
    }
#[no_mangle]
unsafe extern "C" fn get_host_pd4_or_7(addr: u32) -> u32 {
    static u32 get_host_pd4_or_7(u32 addr)
    {
    u32 reg;
    reg = addr;
    return reg;
    }
#[no_mangle]
unsafe extern "C" fn get_host_pd5() -> u32 {
    static u32 get_host_pd5(void)
    {
    u32 reg;
    reg = 0;
    return reg;
    }
    static struct dma_async_tx_descriptor *cppi41_dma_prep_slave_sg(
    struct dma_chan *chan, struct scatterlist *sgl, unsigned sg_len,
    enum dma_transfer_direction dir, unsigned long tx_flags, void *context)
    {
    struct cppi41_channel *c = to_cpp41_chan(chan);
    struct dma_async_tx_descriptor *txd = core::ptr::null_mut();
    struct cppi41_dd *cdd = c.cdd;
    struct cppi41_desc *d;
    struct scatterlist *sg;
    unsigned int i;
    int error;
    error = pm_runtime_get(cdd.ddev.dev);
    if (error < 0) {
    pm_runtime_put_noidle(cdd.ddev.dev);
    return core::ptr::null_mut();
    }
    if (cdd.is_suspended)
    goto err_out_not_ready;
    d = c.desc;
    for_each_sg(sgl, sg, sg_len, i) {
    u32 addr;
    u32 len;
// We need to use more than one desc once musb supports sg
    addr = lower_32_bits(sg_dma_address(sg));
    len = sg_dma_len(sg);
    d.pd0 = get_host_pd0(len);
    d.pd1 = get_host_pd1(c);
    d.pd2 = get_host_pd2(c);
    d.pd3 = get_host_pd3(len);
    d.pd4 = get_host_pd4_or_7(addr);
    d.pd5 = get_host_pd5();
    d.pd6 = get_host_pd6(len);
    d.pd7 = get_host_pd4_or_7(addr);
    d++;
    }
    txd = &c.txd;
    err_out_not_ready:
    pm_runtime_put_autosuspend(cdd.ddev.dev);
    return txd;
    }
#[no_mangle]
unsafe extern "C" fn cppi41_compute_td_desc(d: *mut cppi41_desc) {
    static void cppi41_compute_td_desc(struct cppi41_desc *d)
    {
    d.pd0 = DESC_TYPE_TEARD << DESC_TYPE;
    }
#[no_mangle]
unsafe extern "C" fn cppi41_tear_down_chan(c: *mut cppi41_channel) -> c_int {
    static int cppi41_tear_down_chan(struct cppi41_channel *c)
    {
    struct dmaengine_result abort_result;
    struct cppi41_dd *cdd = c.cdd;
    struct cppi41_desc *td;
    u32 reg;
    u32 desc_phys;
    u32 td_desc_phys;
    td = cdd.cd;
    td += cdd.first_td_desc;
    td_desc_phys = cdd.descs_phys;
    td_desc_phys += cdd.first_td_desc * sizeof(struct cppi41_desc);
    if (!c.td_queued) {
    cppi41_compute_td_desc(td);
    __iowmb();
    reg = (sizeof(struct cppi41_desc) - 24) / 4;
    reg |= td_desc_phys;
    cppi_writel(reg, cdd.qmgr_mem +
    QMGR_QUEUE_D(cdd.td_queue.submit));
    reg = GCR_CHAN_ENABLE;
    if (!c.is_tx) {
    reg |= GCR_STARV_RETRY;
    reg |= GCR_DESC_TYPE_HOST;
    reg |= cdd.td_queue.complete;
    }
    reg |= GCR_TEARDOWN;
    cppi_writel(reg, c.gcr_reg);
    c.td_queued = 1;
    c.td_retry = 500;
    }
    if (!c.td_seen || !c.td_desc_seen) {
    desc_phys = cppi41_pop_desc(cdd, cdd.td_queue.complete);
    if (!desc_phys && c.is_tx)
    desc_phys = cppi41_pop_desc(cdd, c.q_comp_num);
    if (desc_phys == c.desc_phys) {
    c.td_desc_seen = 1;
    } else if (desc_phys == td_desc_phys) {
    u32 pd0;
    __iormb();
    pd0 = td.pd0;
    WARN_ON((pd0 >> DESC_TYPE) != DESC_TYPE_TEARD);
    WARN_ON(!c.is_tx && !(pd0 & TD_DESC_IS_RX));
    WARN_ON((pd0 & 0x1f) != c.port_num);
    c.td_seen = 1;
    } else if (desc_phys) {
    WARN_ON_ONCE(1);
    }
    }
    c.td_retry--;
//
// If the TX descriptor / channel is in use, the caller needs to poke
// his TD bit multiple times. After that he hardware releases the
// transfer descriptor followed by TD descriptor. Waiting seems not to
// cause any difference.
// RX seems to be thrown out right away. However once the TearDown
// descriptor gets through we are done. If we have seen the transfer
// descriptor before the TD we fetch it from enqueue, it has to be
// there waiting for us.
//
    if (!c.td_seen && c.td_retry) {
    udelay(1);
    return -EAGAIN;
    }
    WARN_ON(!c.td_retry);
    if (!c.td_desc_seen) {
    desc_phys = cppi41_pop_desc(cdd, c.q_num);
    if (!desc_phys)
    desc_phys = cppi41_pop_desc(cdd, c.q_comp_num);
    WARN_ON(!desc_phys);
    }
    c.td_queued = 0;
    c.td_seen = 0;
    c.td_desc_seen = 0;
    cppi_writel(0, c.gcr_reg);
// Invoke the callback to do the necessary clean-up
    abort_result.result = DMA_TRANS_ABORTED;
    dma_cookie_complete(&c.txd);
    dmaengine_desc_get_callback_invoke(&c.txd, &abort_result);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cppi41_stop_chan(chan: *mut dma_chan) -> c_int {
    static int cppi41_stop_chan(struct dma_chan *chan)
    {
    struct cppi41_channel *c = to_cpp41_chan(chan);
    struct cppi41_dd *cdd = c.cdd;
    u32 desc_num;
    u32 desc_phys;
    int ret;
    desc_phys = lower_32_bits(c.desc_phys);
    desc_num = (desc_phys - cdd.descs_phys) / sizeof(struct cppi41_desc);
    if (!cdd.chan_busy[desc_num]) {
    struct cppi41_channel *cc, *_ct;
//
// channels might still be in the pending list if
// cppi41_dma_issue_pending() is called after
// cppi41_runtime_suspend() is called
//
    list_for_each_entry_safe(cc, _ct, &cdd.pending, node) {
    if (cc != c)
    continue;
    list_del(&cc.node);
    break;
    }
    return 0;
    }
    ret = cppi41_tear_down_chan(c);
    if (ret)
    return ret;
    WARN_ON(!cdd.chan_busy[desc_num]);
    cdd.chan_busy[desc_num] = core::ptr::null_mut();
// Usecount for chan_busy[], paired with push_desc_queue()
    pm_runtime_put(cdd.ddev.dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cppi41_add_chans(dev: *mut device, cdd: *mut cppi41_dd) -> c_int {
    static int cppi41_add_chans(struct device *dev, struct cppi41_dd *cdd)
    {
    struct cppi41_channel *cchan, *chans;
    int i;
    let mut n_chans: u32 = cdd.n_chans;
//
// The channels can only be used as TX or as RX. So we add twice
// that much dma channels because USB can only do RX or TX.
//
    n_chans *= 2;
    chans = devm_kcalloc(dev, n_chans, sizeof(*chans), GFP_KERNEL);
    if (!chans)
    return -ENOMEM;
    for (i = 0; i < n_chans; i++) {
    cchan = &chans[i];
    cchan.cdd = cdd;
    if (i & 1) {
    cchan.gcr_reg = cdd.ctrl_mem + DMA_TXGCR(i >> 1);
    cchan.is_tx = 1;
    } else {
    cchan.gcr_reg = cdd.ctrl_mem + DMA_RXGCR(i >> 1);
    cchan.is_tx = 0;
    }
    cchan.port_num = i >> 1;
    cchan.desc = &cdd.cd[i];
    cchan.desc_phys = cdd.descs_phys;
    cchan.desc_phys += i * sizeof(struct cppi41_desc);
    cchan.chan.device = &cdd.ddev;
    list_add_tail(&cchan.chan.device_node, &cdd.ddev.channels);
    }
    cdd.first_td_desc = n_chans;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn purge_descs(dev: *mut device, cdd: *mut cppi41_dd) {
    static void purge_descs(struct device *dev, struct cppi41_dd *cdd)
    {
    unsigned int mem_decs;
    int i;
    mem_decs = ALLOC_DECS_NUM * sizeof(struct cppi41_desc);
    for (i = 0; i < DESCS_AREAS; i++) {
    cppi_writel(0, cdd.qmgr_mem + QMGR_MEMBASE(i));
    cppi_writel(0, cdd.qmgr_mem + QMGR_MEMCTRL(i));
    dma_free_coherent(dev, mem_decs, cdd.cd,
    cdd.descs_phys);
    }
    }
#[no_mangle]
unsafe extern "C" fn disable_sched(cdd: *mut cppi41_dd) {
    static void disable_sched(struct cppi41_dd *cdd)
    {
    cppi_writel(0, cdd.sched_mem + DMA_SCHED_CTRL);
    }
#[no_mangle]
unsafe extern "C" fn deinit_cppi41(dev: *mut device, cdd: *mut cppi41_dd) {
    static void deinit_cppi41(struct device *dev, struct cppi41_dd *cdd)
    {
    disable_sched(cdd);
    purge_descs(dev, cdd);
    cppi_writel(0, cdd.qmgr_mem + QMGR_LRAM0_BASE);
    cppi_writel(0, cdd.qmgr_mem + QMGR_LRAM0_BASE);
    dma_free_coherent(dev, QMGR_SCRATCH_SIZE, cdd.qmgr_scratch,
    cdd.scratch_phys);
    }
#[no_mangle]
unsafe extern "C" fn init_descs(dev: *mut device, cdd: *mut cppi41_dd) -> c_int {
    static int init_descs(struct device *dev, struct cppi41_dd *cdd)
    {
    unsigned int desc_size;
    unsigned int mem_decs;
    int i;
    u32 reg;
    u32 idx;
    BUILD_BUG_ON(sizeof(struct cppi41_desc) &
    (sizeof(struct cppi41_desc) - 1));
    BUILD_BUG_ON(sizeof(struct cppi41_desc) < 32);
    BUILD_BUG_ON(ALLOC_DECS_NUM < 32);
    desc_size = sizeof(struct cppi41_desc);
    mem_decs = ALLOC_DECS_NUM * desc_size;
    idx = 0;
    for (i = 0; i < DESCS_AREAS; i++) {
    reg = idx << QMGR_MEMCTRL_IDX_SH;
    reg |= (ilog2(desc_size) - 5) << QMGR_MEMCTRL_DESC_SH;
    reg |= ilog2(ALLOC_DECS_NUM) - 5;
    BUILD_BUG_ON(DESCS_AREAS != 1);
    cdd.cd = dma_alloc_coherent(dev, mem_decs,
    &cdd.descs_phys, GFP_KERNEL);
    if (!cdd.cd)
    return -ENOMEM;
    cppi_writel(cdd.descs_phys, cdd.qmgr_mem + QMGR_MEMBASE(i));
    cppi_writel(reg, cdd.qmgr_mem + QMGR_MEMCTRL(i));
    idx += ALLOC_DECS_NUM;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn init_sched(cdd: *mut cppi41_dd) {
    static void init_sched(struct cppi41_dd *cdd)
    {
    unsigned ch;
    unsigned word;
    u32 reg;
    word = 0;
    cppi_writel(0, cdd.sched_mem + DMA_SCHED_CTRL);
    for (ch = 0; ch < cdd.n_chans; ch += 2) {
    reg = SCHED_ENTRY0_CHAN(ch);
    reg |= SCHED_ENTRY1_CHAN(ch) | SCHED_ENTRY1_IS_RX;
    reg |= SCHED_ENTRY2_CHAN(ch + 1);
    reg |= SCHED_ENTRY3_CHAN(ch + 1) | SCHED_ENTRY3_IS_RX;
    cppi_writel(reg, cdd.sched_mem + DMA_SCHED_WORD(word));
    word++;
    }
    reg = cdd.n_chans * 2 - 1;
    reg |= DMA_SCHED_CTRL_EN;
    cppi_writel(reg, cdd.sched_mem + DMA_SCHED_CTRL);
    }
#[no_mangle]
unsafe extern "C" fn init_cppi41(dev: *mut device, cdd: *mut cppi41_dd) -> c_int {
    static int init_cppi41(struct device *dev, struct cppi41_dd *cdd)
    {
    int ret;
    BUILD_BUG_ON(QMGR_SCRATCH_SIZE > ((1 << 14) - 1));
    cdd.qmgr_scratch = dma_alloc_coherent(dev, QMGR_SCRATCH_SIZE,
    &cdd.scratch_phys, GFP_KERNEL);
    if (!cdd.qmgr_scratch)
    return -ENOMEM;
    cppi_writel(cdd.scratch_phys, cdd.qmgr_mem + QMGR_LRAM0_BASE);
    cppi_writel(TOTAL_DESCS_NUM, cdd.qmgr_mem + QMGR_LRAM_SIZE);
    cppi_writel(0, cdd.qmgr_mem + QMGR_LRAM1_BASE);
    ret = init_descs(dev, cdd);
    if (ret)
    goto err_td;
    cppi_writel(cdd.td_queue.submit, cdd.ctrl_mem + DMA_TDFDQ);
    init_sched(cdd);
    return 0;
    err_td:
    deinit_cppi41(dev, cdd);
    return ret;
    }
    static struct platform_driver cpp41_dma_driver;
//
// The param format is:
// X Y
// X: Port
// Y: 0 = RX else TX
//
pub const INFO_PORT: c_int = 0;
pub const INFO_IS_TX: c_int = 1;
#[no_mangle]
unsafe extern "C" fn cpp41_dma_filter_fn(chan: *mut dma_chan, param: *mut c_void) -> bool {
    static bool cpp41_dma_filter_fn(struct dma_chan *chan, void *param)
    {
    struct cppi41_channel *cchan;
    struct cppi41_dd *cdd;
    const struct chan_queues *queues;
    u32 *num = param;
    if (chan.device.dev.driver != &cpp41_dma_driver.driver)
    return false;
    cchan = to_cpp41_chan(chan);
    if (cchan.port_num != num[INFO_PORT])
    return false;
    if (cchan.is_tx && !num[INFO_IS_TX])
    return false;
    cdd = cchan.cdd;
    if (cchan.is_tx)
    queues = cdd.queues_tx;
    else
    queues = cdd.queues_rx;
    BUILD_BUG_ON(ARRAY_SIZE(am335x_usb_queues_rx) !=
    ARRAY_SIZE(am335x_usb_queues_tx));
    if (WARN_ON(cchan.port_num >= ARRAY_SIZE(am335x_usb_queues_rx)))
    return false;
    cchan.q_num = queues[cchan.port_num].submit;
    cchan.q_comp_num = queues[cchan.port_num].complete;
    return true;
    }
    static struct of_dma_filter_info cpp41_dma_info = {
    .filter_fn = cpp41_dma_filter_fn,
    };
    static struct dma_chan *cppi41_dma_xlate(struct of_phandle_args *dma_spec,
    struct of_dma *ofdma)
    {
    let mut count: c_int = dma_spec.args_count;
    struct of_dma_filter_info *info = ofdma.of_dma_data;
    if (!info || !info.filter_fn)
    return core::ptr::null_mut();
    if (count != 2)
    return core::ptr::null_mut();
    return dma_request_channel(info.dma_cap, info.filter_fn,
    &dma_spec.args[0]);
    }
    static const struct cppi_glue_infos am335x_usb_infos = {
    .queues_rx = am335x_usb_queues_rx,
    .queues_tx = am335x_usb_queues_tx,
    .td_queue = { .submit = 31, .complete = 0 },
    .first_completion_queue = 93,
    .qmgr_num_pend = 5,
    };
    static const struct cppi_glue_infos da8xx_usb_infos = {
    .queues_rx = da8xx_usb_queues_rx,
    .queues_tx = da8xx_usb_queues_tx,
    .td_queue = { .submit = 31, .complete = 0 },
    .first_completion_queue = 24,
    .qmgr_num_pend = 2,
    };
    static const struct of_device_id cppi41_dma_ids[] = {
    { .compatible = "ti,am3359-cppi41", .data = &am335x_usb_infos},
    { .compatible = "ti,da830-cppi41", .data = &da8xx_usb_infos},
    {},
    };
    MODULE_DEVICE_TABLE(of, cppi41_dma_ids);
    static const struct cppi_glue_infos *get_glue_info(struct device *dev)
    {
    const struct of_device_id *of_id;
    of_id = of_match_node(cppi41_dma_ids, dev.of_node);
    if (!of_id)
    return core::ptr::null_mut();
    return of_id.data;
    }

    BIT(DMA_SLAVE_BUSWIDTH_2_BYTES) | \
    BIT(DMA_SLAVE_BUSWIDTH_3_BYTES) | \
    BIT(DMA_SLAVE_BUSWIDTH_4_BYTES))
#[no_mangle]
unsafe extern "C" fn cppi41_dma_probe(pdev: *mut platform_device) -> c_int {
    static int cppi41_dma_probe(struct platform_device *pdev)
    {
    struct cppi41_dd *cdd;
    struct device *dev = &pdev.dev;
    const struct cppi_glue_infos *glue_info;
    int index;
    int irq;
    int ret;
    glue_info = get_glue_info(dev);
    if (!glue_info)
    return -EINVAL;
    cdd = devm_kzalloc(&pdev.dev, sizeof(*cdd), GFP_KERNEL);
    if (!cdd)
    return -ENOMEM;
    dma_cap_set(DMA_SLAVE, cdd.ddev.cap_mask);
    cdd.ddev.device_alloc_chan_resources = cppi41_dma_alloc_chan_resources;
    cdd.ddev.device_free_chan_resources = cppi41_dma_free_chan_resources;
    cdd.ddev.device_tx_status = cppi41_dma_tx_status;
    cdd.ddev.device_issue_pending = cppi41_dma_issue_pending;
    cdd.ddev.device_prep_slave_sg = cppi41_dma_prep_slave_sg;
    cdd.ddev.device_terminate_all = cppi41_stop_chan;
    cdd.ddev.directions = BIT(DMA_DEV_TO_MEM) | BIT(DMA_MEM_TO_DEV);
    cdd.ddev.src_addr_widths = CPPI41_DMA_BUSWIDTHS;
    cdd.ddev.dst_addr_widths = CPPI41_DMA_BUSWIDTHS;
    cdd.ddev.residue_granularity = DMA_RESIDUE_GRANULARITY_BURST;
    cdd.ddev.dev = dev;
    INIT_LIST_HEAD(&cdd.ddev.channels);
    cpp41_dma_info.dma_cap = cdd.ddev.cap_mask;
    index = of_property_match_string(dev.of_node,
    "reg-names", "controller");
    if (index < 0)
    return index;
    cdd.ctrl_mem = devm_platform_ioremap_resource(pdev, index);
    if (IS_ERR(cdd.ctrl_mem))
    return PTR_ERR(cdd.ctrl_mem);
    cdd.sched_mem = devm_platform_ioremap_resource(pdev, index + 1);
    if (IS_ERR(cdd.sched_mem))
    return PTR_ERR(cdd.sched_mem);
    cdd.qmgr_mem = devm_platform_ioremap_resource(pdev, index + 2);
    if (IS_ERR(cdd.qmgr_mem))
    return PTR_ERR(cdd.qmgr_mem);
    spin_lock_init(&cdd.lock);
    INIT_LIST_HEAD(&cdd.pending);
    platform_set_drvdata(pdev, cdd);
    pm_runtime_enable(dev);
    pm_runtime_set_autosuspend_delay(dev, 100);
    pm_runtime_use_autosuspend(dev);
    ret = pm_runtime_get_sync(dev);
    if (ret < 0)
    goto err_get_sync;
    cdd.queues_rx = glue_info.queues_rx;
    cdd.queues_tx = glue_info.queues_tx;
    cdd.td_queue = glue_info.td_queue;
    cdd.qmgr_num_pend = glue_info.qmgr_num_pend;
    cdd.first_completion_queue = glue_info.first_completion_queue;
// Parse new and deprecated dma-channels properties
    ret = of_property_read_u32(dev.of_node,
    "dma-channels", &cdd.n_chans);
    if (ret)
    ret = of_property_read_u32(dev.of_node,
    "#dma-channels", &cdd.n_chans);
    if (ret)
    goto err_get_n_chans;
    ret = init_cppi41(dev, cdd);
    if (ret)
    goto err_init_cppi;
    ret = cppi41_add_chans(dev, cdd);
    if (ret)
    goto err_chans;
    irq = irq_of_parse_and_map(dev.of_node, 0);
    if (!irq) {
    ret = -EINVAL;
    goto err_chans;
    }
    ret = devm_request_irq(&pdev.dev, irq, cppi41_irq, IRQF_SHARED,
    dev_name(dev), cdd);
    if (ret)
    goto err_chans;
    cdd.irq = irq;
    ret = dma_async_device_register(&cdd.ddev);
    if (ret)
    goto err_chans;
    ret = of_dma_controller_register(dev.of_node,
    cppi41_dma_xlate, &cpp41_dma_info);
    if (ret)
    goto err_of;
    pm_runtime_put_autosuspend(dev);
    return 0;
    err_of:
    dma_async_device_unregister(&cdd.ddev);
    err_chans:
    deinit_cppi41(dev, cdd);
    err_init_cppi:
    pm_runtime_dont_use_autosuspend(dev);
    err_get_n_chans:
    err_get_sync:
    pm_runtime_put_sync(dev);
    pm_runtime_disable(dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cppi41_dma_remove(pdev: *mut platform_device) {
    static void cppi41_dma_remove(struct platform_device *pdev)
    {
    struct cppi41_dd *cdd = platform_get_drvdata(pdev);
    int error;
    error = pm_runtime_get_sync(&pdev.dev);
    if (error < 0)
    dev_err(&pdev.dev, "%s could not pm_runtime_get: %i\n",
    __func__, error);
    of_dma_controller_free(pdev.dev.of_node);
    dma_async_device_unregister(&cdd.ddev);
    devm_free_irq(&pdev.dev, cdd.irq, cdd);
    deinit_cppi41(&pdev.dev, cdd);
    pm_runtime_dont_use_autosuspend(&pdev.dev);
    pm_runtime_put_sync(&pdev.dev);
    pm_runtime_disable(&pdev.dev);
    }
#[no_mangle]
unsafe extern "C" fn cppi41_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused cppi41_suspend(struct device *dev)
    {
    struct cppi41_dd *cdd = dev_get_drvdata(dev);
    cdd.dma_tdfdq = cppi_readl(cdd.ctrl_mem + DMA_TDFDQ);
    disable_sched(cdd);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cppi41_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused cppi41_resume(struct device *dev)
    {
    struct cppi41_dd *cdd = dev_get_drvdata(dev);
    struct cppi41_channel *c;
    int i;
    for (i = 0; i < DESCS_AREAS; i++)
    cppi_writel(cdd.descs_phys, cdd.qmgr_mem + QMGR_MEMBASE(i));
    list_for_each_entry(c, &cdd.ddev.channels, chan.device_node)
    if (!c.is_tx)
    cppi_writel(c.q_num, c.gcr_reg + RXHPCRA0);
    init_sched(cdd);
    cppi_writel(cdd.dma_tdfdq, cdd.ctrl_mem + DMA_TDFDQ);
    cppi_writel(cdd.scratch_phys, cdd.qmgr_mem + QMGR_LRAM0_BASE);
    cppi_writel(QMGR_SCRATCH_SIZE, cdd.qmgr_mem + QMGR_LRAM_SIZE);
    cppi_writel(0, cdd.qmgr_mem + QMGR_LRAM1_BASE);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cppi41_runtime_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused cppi41_runtime_suspend(struct device *dev)
    {
    struct cppi41_dd *cdd = dev_get_drvdata(dev);
    unsigned long flags;
    spin_lock_irqsave(&cdd.lock, flags);
    cdd.is_suspended = true;
    WARN_ON(!list_empty(&cdd.pending));
    spin_unlock_irqrestore(&cdd.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cppi41_runtime_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused cppi41_runtime_resume(struct device *dev)
    {
    struct cppi41_dd *cdd = dev_get_drvdata(dev);
    unsigned long flags;
    spin_lock_irqsave(&cdd.lock, flags);
    cdd.is_suspended = false;
    cppi41_run_queue(cdd);
    spin_unlock_irqrestore(&cdd.lock, flags);
    return 0;
    }
    static const struct dev_pm_ops cppi41_pm_ops = {
    SET_LATE_SYSTEM_SLEEP_PM_OPS(cppi41_suspend, cppi41_resume)
    SET_RUNTIME_PM_OPS(cppi41_runtime_suspend,
    cppi41_runtime_resume,
    core::ptr::null_mut())
    };
    static struct platform_driver cpp41_dma_driver = {
    .probe  = cppi41_dma_probe,
    .remove = cppi41_dma_remove,
    .driver = {
    .name = "cppi41-dma-engine",
    .pm = &cppi41_pm_ops,
    .of_match_table = of_match_ptr(cppi41_dma_ids),
    },
    };
    module_platform_driver(cpp41_dma_driver);
    MODULE_DESCRIPTION("Texas Instruments CPPI 4.1 DMA support");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Sebastian Andrzej Siewior <bigeasy@linutronix.de>");
