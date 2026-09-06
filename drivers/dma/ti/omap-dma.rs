//! Automatically rewritten from C to Rust
//! Source: drivers/dma/ti/omap-dma.c
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
// OMAP DMAengine support
//

pub const OMAP_SDMA_REQUESTS: c_int = 127;
pub const OMAP_SDMA_CHANNELS: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_dma_config {
    pub lch_end: c_int,
    pub rw_priority:1: c_uint,
    pub needs_busy_check:1: c_uint,
    pub may_lose_context:1: c_uint,
    pub needs_lch_clear:1: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_dma_context {
    pub irqenable_l0: u32,
    pub irqenable_l1: u32,
    pub ocp_sysconfig: u32,
    pub gcr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_dmadev {
    pub ddev: dma_device,
    pub lock: spinlock_t,
    pub base: *mut void __iomem,
    pub reg_map: *const omap_dma_reg,
    pub plat: *mut omap_system_dma_plat_info,
    pub cfg: *const omap_dma_config,
    pub nb: notifier_block,
    pub context: omap_dma_context,
    pub lch_count: c_int,
    pub OMAP_SDMA_CHANNELS): DECLARE_BITMAP(lch_bitmap,,
    pub /: *mut *mut mutex lch_lock; / for assigning logical channels,
    pub legacy: bool,
    pub ll123_supported: bool,
    pub desc_pool: *mut dma_pool,
    pub dma_requests: unsigned,
    pub irq_lock: spinlock_t,
    pub irq_enable_mask: u32,
    pub lch_map: *mut omap_chan,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_chan {
    pub vc: virt_dma_chan,
    pub channel_base: *mut void __iomem,
    pub reg_map: *const omap_dma_reg,
    pub ccr: u32,
    pub cfg: dma_slave_config,
    pub dma_sig: unsigned,
    pub cyclic: bool,
    pub paused: bool,
    pub running: bool,
    pub dma_ch: c_int,
    pub desc: *mut omap_desc,
    pub sgidx: unsigned,
}

// Type 2 descriptor with Source or Destination address update
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_type2_desc {
    pub next_desc: u32,
    pub en: u32,
    pub /: *mut *mut uint32_t addr; / src or dst,
    pub fn: u16,
    pub cicr: u16,
    pub cdei: i16,
    pub csei: i16,
    pub cdfi: i32,
    pub csfi: i32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_sg {
    pub addr: dma_addr_t,
    pub /: *mut *mut uint32_t en; / number of elements (24-bit),
    pub /: *mut *mut uint32_t fn; / number of frames (16-bit),
    pub /: *mut *mut int32_t fi; / for double indexing,
    pub /: *mut *mut int16_t ei; / for double indexing,
// Linked list
    pub t2_desc: *mut omap_type2_desc,
    pub t2_desc_paddr: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_desc {
    pub vd: virt_dma_desc,
    pub using_ll: bool,
    pub dir: enum dma_transfer_direction,
    pub dev_addr: dma_addr_t,
    pub polled: bool,
    pub /: *mut *mut int32_t fi; / for OMAP_DMA_SYNC_PACKET / double indexing,
    pub /: *mut *mut int16_t ei; / for double indexing,
    pub /: *mut *mut uint8_t es; / CSDP_DATA_TYPE_xxx,
    pub /: *mut *mut uint32_t ccr; / CCR value,
    pub /: *mut *mut uint16_t clnk_ctrl; / CLNK_CTRL value,
    pub /: *mut *mut uint16_t cicr; / CICR value,
    pub /: *mut *mut uint32_t csdp; / CSDP value,
    pub sglen: unsigned,
    pub __counted_by(sglen): omap_sg sg[],
}

    enum {
    CAPS_0_SUPPORT_LL123	= BIT(20),	/* Linked List type1/2/3 */
    CAPS_0_SUPPORT_LL4	= BIT(21),	/* Linked List type4 */
    CCR_FS			= BIT(5),
    CCR_READ_PRIORITY	= BIT(6),
    CCR_ENABLE		= BIT(7),
    CCR_AUTO_INIT		= BIT(8),	/* OMAP1 only */
    CCR_REPEAT		= BIT(9),	/* OMAP1 only */
    CCR_OMAP31_DISABLE	= BIT(10),	/* OMAP1 only */
    CCR_SUSPEND_SENSITIVE	= BIT(8),	/* OMAP2+ only */
    CCR_RD_ACTIVE		= BIT(9),	/* OMAP2+ only */
    CCR_WR_ACTIVE		= BIT(10),	/* OMAP2+ only */
    CCR_SRC_AMODE_CONSTANT	= 0 << 12,
    CCR_SRC_AMODE_POSTINC	= 1 << 12,
    CCR_SRC_AMODE_SGLIDX	= 2 << 12,
    CCR_SRC_AMODE_DBLIDX	= 3 << 12,
    CCR_DST_AMODE_CONSTANT	= 0 << 14,
    CCR_DST_AMODE_POSTINC	= 1 << 14,
    CCR_DST_AMODE_SGLIDX	= 2 << 14,
    CCR_DST_AMODE_DBLIDX	= 3 << 14,
    CCR_CONSTANT_FILL	= BIT(16),
    CCR_TRANSPARENT_COPY	= BIT(17),
    CCR_BS			= BIT(18),
    CCR_SUPERVISOR		= BIT(22),
    CCR_PREFETCH		= BIT(23),
    CCR_TRIGGER_SRC		= BIT(24),
    CCR_BUFFERING_DISABLE	= BIT(25),
    CCR_WRITE_PRIORITY	= BIT(26),
    CCR_SYNC_ELEMENT	= 0,
    CCR_SYNC_FRAME		= CCR_FS,
    CCR_SYNC_BLOCK		= CCR_BS,
    CCR_SYNC_PACKET		= CCR_BS | CCR_FS,
    CSDP_DATA_TYPE_8	= 0,
    CSDP_DATA_TYPE_16	= 1,
    CSDP_DATA_TYPE_32	= 2,
    CSDP_SRC_PORT_EMIFF	= 0 << 2, /* OMAP1 only */
    CSDP_SRC_PORT_EMIFS	= 1 << 2, /* OMAP1 only */
    CSDP_SRC_PORT_OCP_T1	= 2 << 2, /* OMAP1 only */
    CSDP_SRC_PORT_TIPB	= 3 << 2, /* OMAP1 only */
    CSDP_SRC_PORT_OCP_T2	= 4 << 2, /* OMAP1 only */
    CSDP_SRC_PORT_MPUI	= 5 << 2, /* OMAP1 only */
    CSDP_SRC_PACKED		= BIT(6),
    CSDP_SRC_BURST_1	= 0 << 7,
    CSDP_SRC_BURST_16	= 1 << 7,
    CSDP_SRC_BURST_32	= 2 << 7,
    CSDP_SRC_BURST_64	= 3 << 7,
    CSDP_DST_PORT_EMIFF	= 0 << 9, /* OMAP1 only */
    CSDP_DST_PORT_EMIFS	= 1 << 9, /* OMAP1 only */
    CSDP_DST_PORT_OCP_T1	= 2 << 9, /* OMAP1 only */
    CSDP_DST_PORT_TIPB	= 3 << 9, /* OMAP1 only */
    CSDP_DST_PORT_OCP_T2	= 4 << 9, /* OMAP1 only */
    CSDP_DST_PORT_MPUI	= 5 << 9, /* OMAP1 only */
    CSDP_DST_PACKED		= BIT(13),
    CSDP_DST_BURST_1	= 0 << 14,
    CSDP_DST_BURST_16	= 1 << 14,
    CSDP_DST_BURST_32	= 2 << 14,
    CSDP_DST_BURST_64	= 3 << 14,
    CSDP_WRITE_NON_POSTED	= 0 << 16,
    CSDP_WRITE_POSTED	= 1 << 16,
    CSDP_WRITE_LAST_NON_POSTED = 2 << 16,
    CICR_TOUT_IE		= BIT(0),	/* OMAP1 only */
    CICR_DROP_IE		= BIT(1),
    CICR_HALF_IE		= BIT(2),
    CICR_FRAME_IE		= BIT(3),
    CICR_LAST_IE		= BIT(4),
    CICR_BLOCK_IE		= BIT(5),
    CICR_PKT_IE		= BIT(7),	/* OMAP2+ only */
    CICR_TRANS_ERR_IE	= BIT(8),	/* OMAP2+ only */
    CICR_SUPERVISOR_ERR_IE	= BIT(10),	/* OMAP2+ only */
    CICR_MISALIGNED_ERR_IE	= BIT(11),	/* OMAP2+ only */
    CICR_DRAIN_IE		= BIT(12),	/* OMAP2+ only */
    CICR_SUPER_BLOCK_IE	= BIT(14),	/* OMAP2+ only */
    CLNK_CTRL_ENABLE_LNK	= BIT(15),
    CDP_DST_VALID_INC	= 0 << 0,
    CDP_DST_VALID_RELOAD	= 1 << 0,
    CDP_DST_VALID_REUSE	= 2 << 0,
    CDP_SRC_VALID_INC	= 0 << 2,
    CDP_SRC_VALID_RELOAD	= 1 << 2,
    CDP_SRC_VALID_REUSE	= 2 << 2,
    CDP_NTYPE_TYPE1		= 1 << 4,
    CDP_NTYPE_TYPE2		= 2 << 4,
    CDP_NTYPE_TYPE3		= 3 << 4,
    CDP_TMODE_NORMAL	= 0 << 8,
    CDP_TMODE_LLIST		= 1 << 8,
    CDP_FAST		= BIT(10),
    };
    static const unsigned es_bytes[] = {
    [CSDP_DATA_TYPE_8] = 1,
    [CSDP_DATA_TYPE_16] = 2,
    [CSDP_DATA_TYPE_32] = 4,
    };
    static bool omap_dma_filter_fn(struct dma_chan *chan, void *param);
    static struct of_dma_filter_info omap_dma_info = {
    .filter_fn = omap_dma_filter_fn,
    };
    static inline struct omap_dmadev *to_omap_dma_dev(struct dma_device *d)
    {
    return container_of(d, struct omap_dmadev, ddev);
    }
    static inline struct omap_chan *to_omap_dma_chan(struct dma_chan *c)
    {
    return container_of(c, struct omap_chan, vc.chan);
    }
    static inline struct omap_desc *to_omap_dma_desc(struct dma_async_tx_descriptor *t)
    {
    return container_of(t, struct omap_desc, vd.tx);
    }
#[no_mangle]
unsafe extern "C" fn omap_dma_desc_free(vd: *mut virt_dma_desc) {
    static void omap_dma_desc_free(struct virt_dma_desc *vd)
    {
    struct omap_desc *d = to_omap_dma_desc(&vd.tx);
    if (d.using_ll) {
    struct omap_dmadev *od = to_omap_dma_dev(vd.tx.chan.device);
    int i;
    for (i = 0; i < d.sglen; i++) {
    if (d.sg[i].t2_desc)
    dma_pool_free(od.desc_pool, d.sg[i].t2_desc,
    d.sg[i].t2_desc_paddr);
    }
    }
    kfree(d);
    }
    static void omap_dma_fill_type2_desc(struct omap_desc *d, int idx,
    enum dma_transfer_direction dir, bool last)
    {
    struct omap_sg *sg = &d.sg[idx];
    struct omap_type2_desc *t2_desc = sg.t2_desc;
    if (idx)
    d.sg[idx - 1].t2_desc.next_desc = sg.t2_desc_paddr;
    if (last)
    t2_desc.next_desc = 0xfffffffc;
    t2_desc.en = sg.en;
    t2_desc.addr = sg.addr;
    t2_desc.fn = sg.fn & 0xffff;
    t2_desc.cicr = d.cicr;
    if (!last)
    t2_desc.cicr &= ~CICR_BLOCK_IE;
    switch (dir) {
    case DMA_DEV_TO_MEM:
    t2_desc.cdei = sg.ei;
    t2_desc.csei = d.ei;
    t2_desc.cdfi = sg.fi;
    t2_desc.csfi = d.fi;
    t2_desc.en |= DESC_NXT_DV_REFRESH;
    t2_desc.en |= DESC_NXT_SV_REUSE;
    break;
    case DMA_MEM_TO_DEV:
    t2_desc.cdei = d.ei;
    t2_desc.csei = sg.ei;
    t2_desc.cdfi = d.fi;
    t2_desc.csfi = sg.fi;
    t2_desc.en |= DESC_NXT_SV_REFRESH;
    t2_desc.en |= DESC_NXT_DV_REUSE;
    break;
    default:
    return;
    }
    t2_desc.en |= DESC_NTYPE_TYPE2;
    }
#[no_mangle]
unsafe extern "C" fn omap_dma_write(val: u32, type: unsigned, addr: *mut void __iomem) {
    static void omap_dma_write(uint32_t val, unsigned type, void __iomem *addr)
    {
    switch (type) {
    case OMAP_DMA_REG_16BIT:
    writew_relaxed(val, addr);
    break;
    case OMAP_DMA_REG_2X16BIT:
    writew_relaxed(val, addr);
    writew_relaxed(val >> 16, addr + 2);
    break;
    case OMAP_DMA_REG_32BIT:
    writel_relaxed(val, addr);
    break;
    default:
    WARN_ON(1);
    }
    }
#[no_mangle]
unsafe extern "C" fn omap_dma_read(type: unsigned, addr: *mut void __iomem) -> unsigned {
    static unsigned omap_dma_read(unsigned type, void __iomem *addr)
    {
    unsigned val;
    switch (type) {
    case OMAP_DMA_REG_16BIT:
    val = readw_relaxed(addr);
    break;
    case OMAP_DMA_REG_2X16BIT:
    val = readw_relaxed(addr);
    val |= readw_relaxed(addr + 2) << 16;
    break;
    case OMAP_DMA_REG_32BIT:
    val = readl_relaxed(addr);
    break;
    default:
    WARN_ON(1);
    val = 0;
    }
    return val;
    }
#[no_mangle]
unsafe extern "C" fn omap_dma_glbl_write(od: *mut omap_dmadev, reg: unsigned, val: unsigned) {
    static void omap_dma_glbl_write(struct omap_dmadev *od, unsigned reg, unsigned val)
    {
    const struct omap_dma_reg *r = od.reg_map + reg;
    WARN_ON(r.stride);
    omap_dma_write(val, r.type, od.base + r.offset);
    }
#[no_mangle]
unsafe extern "C" fn omap_dma_glbl_read(od: *mut omap_dmadev, reg: unsigned) -> unsigned {
    static unsigned omap_dma_glbl_read(struct omap_dmadev *od, unsigned reg)
    {
    const struct omap_dma_reg *r = od.reg_map + reg;
    WARN_ON(r.stride);
    return omap_dma_read(r.type, od.base + r.offset);
    }
#[no_mangle]
unsafe extern "C" fn omap_dma_chan_write(c: *mut omap_chan, reg: unsigned, val: unsigned) {
    static void omap_dma_chan_write(struct omap_chan *c, unsigned reg, unsigned val)
    {
    const struct omap_dma_reg *r = c.reg_map + reg;
    omap_dma_write(val, r.type, c.channel_base + r.offset);
    }
#[no_mangle]
unsafe extern "C" fn omap_dma_chan_read(c: *mut omap_chan, reg: unsigned) -> unsigned {
    static unsigned omap_dma_chan_read(struct omap_chan *c, unsigned reg)
    {
    const struct omap_dma_reg *r = c.reg_map + reg;
    return omap_dma_read(r.type, c.channel_base + r.offset);
    }
#[no_mangle]
unsafe extern "C" fn omap_dma_clear_csr(c: *mut omap_chan) {
    static void omap_dma_clear_csr(struct omap_chan *c)
    {
    if (dma_omap1())
    omap_dma_chan_read(c, CSR);
    else
    omap_dma_chan_write(c, CSR, ~0);
    }
#[no_mangle]
unsafe extern "C" fn omap_dma_get_csr(c: *mut omap_chan) -> unsigned {
    static unsigned omap_dma_get_csr(struct omap_chan *c)
    {
    let mut val: unsigned = omap_dma_chan_read(c, CSR);
    if (!dma_omap1())
    omap_dma_chan_write(c, CSR, val);
    return val;
    }
#[no_mangle]
unsafe extern "C" fn omap_dma_clear_lch(od: *mut omap_dmadev, lch: c_int) {
    static void omap_dma_clear_lch(struct omap_dmadev *od, int lch)
    {
    struct omap_chan *c;
    int i;
    c = od.lch_map[lch];
    if (!c)
    return;
    for (i = CSDP; i <= od.cfg.lch_end; i++)
    omap_dma_chan_write(c, i, 0);
    }
    static void omap_dma_assign(struct omap_dmadev *od, struct omap_chan *c,
    unsigned lch)
    {
    c.channel_base = od.base + od.plat.channel_stride * lch;
    od.lch_map[lch] = c;
    }
#[no_mangle]
unsafe extern "C" fn omap_dma_start(c: *mut omap_chan, d: *mut omap_desc) {
    static void omap_dma_start(struct omap_chan *c, struct omap_desc *d)
    {
    struct omap_dmadev *od = to_omap_dma_dev(c.vc.chan.device);
    let mut cicr: u16 = d.cicr;
    if (__dma_omap15xx(od.plat.dma_attr))
    omap_dma_chan_write(c, CPC, 0);
    else
    omap_dma_chan_write(c, CDAC, 0);
    omap_dma_clear_csr(c);
    if (d.using_ll) {
    let mut cdp: u32 = CDP_TMODE_LLIST | CDP_NTYPE_TYPE2 | CDP_FAST;
    if (d.dir == DMA_DEV_TO_MEM)
    cdp |= (CDP_DST_VALID_RELOAD | CDP_SRC_VALID_REUSE);
    else
    cdp |= (CDP_DST_VALID_REUSE | CDP_SRC_VALID_RELOAD);
    omap_dma_chan_write(c, CDP, cdp);
    omap_dma_chan_write(c, CNDP, d.sg[0].t2_desc_paddr);
    omap_dma_chan_write(c, CCDN, 0);
    omap_dma_chan_write(c, CCFN, 0xffff);
    omap_dma_chan_write(c, CCEN, 0xffffff);
    cicr &= ~CICR_BLOCK_IE;
    } else if (od.ll123_supported) {
    omap_dma_chan_write(c, CDP, 0);
    }
// Enable interrupts
    omap_dma_chan_write(c, CICR, cicr);
// Enable channel
    omap_dma_chan_write(c, CCR, d.ccr | CCR_ENABLE);
    c.running = true;
    }
#[no_mangle]
unsafe extern "C" fn omap_dma_drain_chan(c: *mut omap_chan) {
    static void omap_dma_drain_chan(struct omap_chan *c)
    {
    int i;
    u32 val;
// Wait for sDMA FIFO to drain
    for (i = 0; ; i++) {
    val = omap_dma_chan_read(c, CCR);
    if (!(val & (CCR_RD_ACTIVE | CCR_WR_ACTIVE)))
    break;
    if (i > 100)
    break;
    udelay(5);
    }
    if (val & (CCR_RD_ACTIVE | CCR_WR_ACTIVE))
    dev_err(c.vc.chan.device.dev,
    "DMA drain did not complete on lch %d\n",
    c.dma_ch);
    }
#[no_mangle]
unsafe extern "C" fn omap_dma_stop(c: *mut omap_chan) -> c_int {
    static int omap_dma_stop(struct omap_chan *c)
    {
    struct omap_dmadev *od = to_omap_dma_dev(c.vc.chan.device);
    uint32_t val;
// disable irq
    omap_dma_chan_write(c, CICR, 0);
    omap_dma_clear_csr(c);
    val = omap_dma_chan_read(c, CCR);
    if (od.plat.errata & DMA_ERRATA_i541 && val & CCR_TRIGGER_SRC) {
    uint32_t sysconfig;
    sysconfig = omap_dma_glbl_read(od, OCP_SYSCONFIG);
    val = sysconfig & ~DMA_SYSCONFIG_MIDLEMODE_MASK;
    val |= DMA_SYSCONFIG_MIDLEMODE(DMA_IDLEMODE_NO_IDLE);
    omap_dma_glbl_write(od, OCP_SYSCONFIG, val);
    val = omap_dma_chan_read(c, CCR);
    val &= ~CCR_ENABLE;
    omap_dma_chan_write(c, CCR, val);
    if (!(c.ccr & CCR_BUFFERING_DISABLE))
    omap_dma_drain_chan(c);
    omap_dma_glbl_write(od, OCP_SYSCONFIG, sysconfig);
    } else {
    if (!(val & CCR_ENABLE))
    return -EINVAL;
    val &= ~CCR_ENABLE;
    omap_dma_chan_write(c, CCR, val);
    if (!(c.ccr & CCR_BUFFERING_DISABLE))
    omap_dma_drain_chan(c);
    }
    mb();
    if (!__dma_omap15xx(od.plat.dma_attr) && c.cyclic) {
    val = omap_dma_chan_read(c, CLNK_CTRL);
    if (dma_omap1())
    val |= 1 << 14; /* set the STOP_LNK bit */
    else
    val &= ~CLNK_CTRL_ENABLE_LNK;
    omap_dma_chan_write(c, CLNK_CTRL, val);
    }
    c.running = false;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn omap_dma_start_sg(c: *mut omap_chan, d: *mut omap_desc) {
    static void omap_dma_start_sg(struct omap_chan *c, struct omap_desc *d)
    {
    struct omap_sg *sg = d.sg + c.sgidx;
    unsigned cxsa, cxei, cxfi;
    if (d.dir == DMA_DEV_TO_MEM || d.dir == DMA_MEM_TO_MEM) {
    cxsa = CDSA;
    cxei = CDEI;
    cxfi = CDFI;
    } else {
    cxsa = CSSA;
    cxei = CSEI;
    cxfi = CSFI;
    }
    omap_dma_chan_write(c, cxsa, sg.addr);
    omap_dma_chan_write(c, cxei, sg.ei);
    omap_dma_chan_write(c, cxfi, sg.fi);
    omap_dma_chan_write(c, CEN, sg.en);
    omap_dma_chan_write(c, CFN, sg.fn);
    omap_dma_start(c, d);
    c.sgidx++;
    }
#[no_mangle]
unsafe extern "C" fn omap_dma_start_desc(c: *mut omap_chan) {
    static void omap_dma_start_desc(struct omap_chan *c)
    {
    struct virt_dma_desc *vd = vchan_next_desc(&c.vc);
    struct omap_desc *d;
    unsigned cxsa, cxei, cxfi;
    if (!vd) {
    c.desc = core::ptr::null_mut();
    return;
    }
    list_del(&vd.node);
    c.desc = d = to_omap_dma_desc(&vd.tx);
    c.sgidx = 0;
//
// This provides the necessary barrier to ensure data held in
// DMA coherent memory is visible to the DMA engine prior to
// the transfer starting.
//
    mb();
    omap_dma_chan_write(c, CCR, d.ccr);
    if (dma_omap1())
    omap_dma_chan_write(c, CCR2, d.ccr >> 16);
    if (d.dir == DMA_DEV_TO_MEM || d.dir == DMA_MEM_TO_MEM) {
    cxsa = CSSA;
    cxei = CSEI;
    cxfi = CSFI;
    } else {
    cxsa = CDSA;
    cxei = CDEI;
    cxfi = CDFI;
    }
    omap_dma_chan_write(c, cxsa, d.dev_addr);
    omap_dma_chan_write(c, cxei, d.ei);
    omap_dma_chan_write(c, cxfi, d.fi);
    omap_dma_chan_write(c, CSDP, d.csdp);
    omap_dma_chan_write(c, CLNK_CTRL, d.clnk_ctrl);
    omap_dma_start_sg(c, d);
    }
#[no_mangle]
unsafe extern "C" fn omap_dma_callback(ch: c_int, status: u16, data: *mut c_void) {
    static void omap_dma_callback(int ch, u16 status, void *data)
    {
    struct omap_chan *c = data;
    struct omap_desc *d;
    unsigned long flags;
    spin_lock_irqsave(&c.vc.lock, flags);
    d = c.desc;
    if (d) {
    if (c.cyclic) {
    vchan_cyclic_callback(&d.vd);
    } else if (d.using_ll || c.sgidx == d.sglen) {
    omap_dma_start_desc(c);
    vchan_cookie_complete(&d.vd);
    } else {
    omap_dma_start_sg(c, d);
    }
    }
    spin_unlock_irqrestore(&c.vc.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn omap_dma_irq(irq: c_int, devid: *mut c_void) -> irqreturn_t {
    static irqreturn_t omap_dma_irq(int irq, void *devid)
    {
    struct omap_dmadev *od = devid;
    unsigned status, channel;
    spin_lock(&od.irq_lock);
    status = omap_dma_glbl_read(od, IRQSTATUS_L1);
    status &= od.irq_enable_mask;
    if (status == 0) {
    spin_unlock(&od.irq_lock);
    return IRQ_NONE;
    }
    while ((channel = ffs(status)) != 0) {
    unsigned mask, csr;
    struct omap_chan *c;
    channel -= 1;
    mask = BIT(channel);
    status &= ~mask;
    c = od.lch_map[channel];
    if (c == core::ptr::null_mut()) {
// This should never happen
    dev_err(od.ddev.dev, "invalid channel %u\n", channel);
    continue;
    }
    csr = omap_dma_get_csr(c);
    omap_dma_glbl_write(od, IRQSTATUS_L1, mask);
    omap_dma_callback(channel, csr, c);
    }
    spin_unlock(&od.irq_lock);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn omap_dma_get_lch(od: *mut omap_dmadev, lch: *mut c_int) -> c_int {
    static int omap_dma_get_lch(struct omap_dmadev *od, int *lch)
    {
    int channel;
    mutex_lock(&od.lch_lock);
    channel = find_first_zero_bit(od.lch_bitmap, od.lch_count);
    if (channel >= od.lch_count)
    goto out_busy;
    set_bit(channel, od.lch_bitmap);
    mutex_unlock(&od.lch_lock);
    omap_dma_clear_lch(od, channel);
// lch = channel;
    return 0;
    out_busy:
    mutex_unlock(&od.lch_lock);
// lch = -EINVAL;
    return -EBUSY;
    }
#[no_mangle]
unsafe extern "C" fn omap_dma_put_lch(od: *mut omap_dmadev, lch: c_int) {
    static void omap_dma_put_lch(struct omap_dmadev *od, int lch)
    {
    omap_dma_clear_lch(od, lch);
    mutex_lock(&od.lch_lock);
    clear_bit(lch, od.lch_bitmap);
    mutex_unlock(&od.lch_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn omap_dma_legacy(od: *mut omap_dmadev) -> bool {
    static inline bool omap_dma_legacy(struct omap_dmadev *od)
    {
    return IS_ENABLED(CONFIG_ARCH_OMAP1) && od.legacy;
    }
#[no_mangle]
unsafe extern "C" fn omap_dma_alloc_chan_resources(chan: *mut dma_chan) -> c_int {
    static int omap_dma_alloc_chan_resources(struct dma_chan *chan)
    {
    struct omap_dmadev *od = to_omap_dma_dev(chan.device);
    struct omap_chan *c = to_omap_dma_chan(chan);
    struct device *dev = od.ddev.dev;
    int ret;
    if (omap_dma_legacy(od)) {
    ret = omap_request_dma(c.dma_sig, "DMA engine",
    omap_dma_callback, c, &c.dma_ch);
    } else {
    ret = omap_dma_get_lch(od, &c.dma_ch);
    }
    dev_dbg(dev, "allocating channel %u for %u\n", c.dma_ch, c.dma_sig);
    if (ret >= 0) {
    omap_dma_assign(od, c, c.dma_ch);
    if (!omap_dma_legacy(od)) {
    unsigned val;
    spin_lock_irq(&od.irq_lock);
    val = BIT(c.dma_ch);
    omap_dma_glbl_write(od, IRQSTATUS_L1, val);
    od.irq_enable_mask |= val;
    omap_dma_glbl_write(od, IRQENABLE_L1, od.irq_enable_mask);
    val = omap_dma_glbl_read(od, IRQENABLE_L0);
    val &= ~BIT(c.dma_ch);
    omap_dma_glbl_write(od, IRQENABLE_L0, val);
    spin_unlock_irq(&od.irq_lock);
    }
    }
    if (dma_omap1()) {
    if (__dma_omap16xx(od.plat.dma_attr)) {
    c.ccr = CCR_OMAP31_DISABLE;
// Duplicate what plat-omap/dma.c does
    c.ccr |= c.dma_ch + 1;
    } else {
    c.ccr = c.dma_sig & 0x1f;
    }
    } else {
    c.ccr = c.dma_sig & 0x1f;
    c.ccr |= (c.dma_sig & ~0x1f) << 14;
    }
    if (od.plat.errata & DMA_ERRATA_IFRAME_BUFFERING)
    c.ccr |= CCR_BUFFERING_DISABLE;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn omap_dma_free_chan_resources(chan: *mut dma_chan) {
    static void omap_dma_free_chan_resources(struct dma_chan *chan)
    {
    struct omap_dmadev *od = to_omap_dma_dev(chan.device);
    struct omap_chan *c = to_omap_dma_chan(chan);
    if (!omap_dma_legacy(od)) {
    spin_lock_irq(&od.irq_lock);
    od.irq_enable_mask &= ~BIT(c.dma_ch);
    omap_dma_glbl_write(od, IRQENABLE_L1, od.irq_enable_mask);
    spin_unlock_irq(&od.irq_lock);
    }
    c.channel_base = core::ptr::null_mut();
    od.lch_map[c.dma_ch] = core::ptr::null_mut();
    vchan_free_chan_resources(&c.vc);
    if (omap_dma_legacy(od))
    omap_free_dma(c.dma_ch);
    else
    omap_dma_put_lch(od, c.dma_ch);
    dev_dbg(od.ddev.dev, "freeing channel %u used for %u\n", c.dma_ch,
    c.dma_sig);
    c.dma_sig = 0;
    }
#[no_mangle]
unsafe extern "C" fn omap_dma_sg_size(sg: *mut omap_sg) -> usize {
    static size_t omap_dma_sg_size(struct omap_sg *sg)
    {
    return sg.en * sg.fn;
    }
#[no_mangle]
unsafe extern "C" fn omap_dma_desc_size(d: *mut omap_desc) -> usize {
    static size_t omap_dma_desc_size(struct omap_desc *d)
    {
    unsigned i;
    size_t size;
    for (size = i = 0; i < d.sglen; i++)
    size += omap_dma_sg_size(&d.sg[i]);
    return size * es_bytes[d.es];
    }
#[no_mangle]
unsafe extern "C" fn omap_dma_desc_size_pos(d: *mut omap_desc, addr: dma_addr_t) -> usize {
    static size_t omap_dma_desc_size_pos(struct omap_desc *d, dma_addr_t addr)
    {
    unsigned i;
    size_t size, es_size = es_bytes[d.es];
    for (size = i = 0; i < d.sglen; i++) {
    let mut this_size: usize = omap_dma_sg_size(&d.sg[i]) * es_size;
    if (size)
    size += this_size;
    else if (addr >= d.sg[i].addr &&
    addr < d.sg[i].addr + this_size)
    size += d.sg[i].addr + this_size - addr;
    }
    return size;
    }
//
// OMAP 3.2/3.3 erratum: sometimes 0 is returned if CSAC/CDAC is
// read before the DMA controller finished disabling the channel.
//
#[no_mangle]
unsafe extern "C" fn omap_dma_chan_read_3_3(c: *mut omap_chan, reg: unsigned) -> u32 {
    static uint32_t omap_dma_chan_read_3_3(struct omap_chan *c, unsigned reg)
    {
    struct omap_dmadev *od = to_omap_dma_dev(c.vc.chan.device);
    uint32_t val;
    val = omap_dma_chan_read(c, reg);
    if (val == 0 && od.plat.errata & DMA_ERRATA_3_3)
    val = omap_dma_chan_read(c, reg);
    return val;
    }
#[no_mangle]
unsafe extern "C" fn omap_dma_get_src_pos(c: *mut omap_chan) -> dma_addr_t {
    static dma_addr_t omap_dma_get_src_pos(struct omap_chan *c)
    {
    struct omap_dmadev *od = to_omap_dma_dev(c.vc.chan.device);
    dma_addr_t addr, cdac;
    if (__dma_omap15xx(od.plat.dma_attr)) {
    addr = omap_dma_chan_read(c, CPC);
    } else {
    addr = omap_dma_chan_read_3_3(c, CSAC);
    cdac = omap_dma_chan_read_3_3(c, CDAC);
//
// CDAC == 0 indicates that the DMA transfer on the channel has
// not been started (no data has been transferred so far).
// Return the programmed source start address in this case.
//
    if (cdac == 0)
    addr = omap_dma_chan_read(c, CSSA);
    }
    if (dma_omap1())
    addr |= omap_dma_chan_read(c, CSSA) & 0xffff0000;
    return addr;
    }
#[no_mangle]
unsafe extern "C" fn omap_dma_get_dst_pos(c: *mut omap_chan) -> dma_addr_t {
    static dma_addr_t omap_dma_get_dst_pos(struct omap_chan *c)
    {
    struct omap_dmadev *od = to_omap_dma_dev(c.vc.chan.device);
    dma_addr_t addr;
    if (__dma_omap15xx(od.plat.dma_attr)) {
    addr = omap_dma_chan_read(c, CPC);
    } else {
    addr = omap_dma_chan_read_3_3(c, CDAC);
//
// CDAC == 0 indicates that the DMA transfer on the channel
// has not been started (no data has been transferred so
// far).  Return the programmed destination start address in
// this case.
//
    if (addr == 0)
    addr = omap_dma_chan_read(c, CDSA);
    }
    if (dma_omap1())
    addr |= omap_dma_chan_read(c, CDSA) & 0xffff0000;
    return addr;
    }
    static enum dma_status omap_dma_tx_status(struct dma_chan *chan,
    dma_cookie_t cookie, struct dma_tx_state *txstate)
    {
    struct omap_chan *c = to_omap_dma_chan(chan);
    enum dma_status ret;
    unsigned long flags;
    struct omap_desc *d = core::ptr::null_mut();
    ret = dma_cookie_status(chan, cookie, txstate);
    if (ret == DMA_COMPLETE)
    return ret;
    spin_lock_irqsave(&c.vc.lock, flags);
    if (c.desc && c.desc.vd.tx.cookie == cookie)
    d = c.desc;
    if (!txstate)
    goto out;
    if (d) {
    dma_addr_t pos;
    if (d.dir == DMA_MEM_TO_DEV)
    pos = omap_dma_get_src_pos(c);
#[no_mangle]
pub unsafe extern "C" fn if(DMA_MEM_TO_MEM: d->dir == DMA_DEV_TO_MEM || d->dir ==) -> else {
    else if (d.dir == DMA_DEV_TO_MEM  || d.dir == DMA_MEM_TO_MEM)
    pos = omap_dma_get_dst_pos(c);
    else
    pos = 0;
    txstate.residue = omap_dma_desc_size_pos(d, pos);
    } else {
    struct virt_dma_desc *vd = vchan_find_desc(&c.vc, cookie);
    if (vd)
    txstate.residue = omap_dma_desc_size(
    to_omap_dma_desc(&vd.tx));
    else
    txstate.residue = 0;
    }
    out:
    if (ret == DMA_IN_PROGRESS && c.paused) {
    ret = DMA_PAUSED;
    } else if (d && d.polled && c.running) {
    let mut ccr: u32 = omap_dma_chan_read(c, CCR);
//
// The channel is no longer active, set the return value
// accordingly and mark it as completed
//
    if (!(ccr & CCR_ENABLE)) {
    ret = DMA_COMPLETE;
    omap_dma_start_desc(c);
    vchan_cookie_complete(&d.vd);
    }
    }
    spin_unlock_irqrestore(&c.vc.lock, flags);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn omap_dma_issue_pending(chan: *mut dma_chan) {
    static void omap_dma_issue_pending(struct dma_chan *chan)
    {
    struct omap_chan *c = to_omap_dma_chan(chan);
    unsigned long flags;
    spin_lock_irqsave(&c.vc.lock, flags);
    if (vchan_issue_pending(&c.vc) && !c.desc)
    omap_dma_start_desc(c);
    spin_unlock_irqrestore(&c.vc.lock, flags);
    }
    static struct dma_async_tx_descriptor *omap_dma_prep_slave_sg(
    struct dma_chan *chan, struct scatterlist *sgl, unsigned sglen,
    enum dma_transfer_direction dir, unsigned long tx_flags, void *context)
    {
    struct omap_dmadev *od = to_omap_dma_dev(chan.device);
    struct omap_chan *c = to_omap_dma_chan(chan);
    enum dma_slave_buswidth dev_width;
    struct scatterlist *sgent;
    struct omap_desc *d;
    dma_addr_t dev_addr;
    unsigned i, es, en, frame_bytes;
    let mut ll_failed: bool = false;
    u32 burst;
    u32 port_window, port_window_bytes;
    if (dir == DMA_DEV_TO_MEM) {
    dev_addr = c.cfg.src_addr;
    dev_width = c.cfg.src_addr_width;
    burst = c.cfg.src_maxburst;
    port_window = c.cfg.src_port_window_size;
    } else if (dir == DMA_MEM_TO_DEV) {
    dev_addr = c.cfg.dst_addr;
    dev_width = c.cfg.dst_addr_width;
    burst = c.cfg.dst_maxburst;
    port_window = c.cfg.dst_port_window_size;
    } else {
    dev_err(chan.device.dev, "%s: bad direction?\n", __func__);
    return core::ptr::null_mut();
    }
// Bus width translates to the element size (ES)
    switch (dev_width) {
    case DMA_SLAVE_BUSWIDTH_1_BYTE:
    es = CSDP_DATA_TYPE_8;
    break;
    case DMA_SLAVE_BUSWIDTH_2_BYTES:
    es = CSDP_DATA_TYPE_16;
    break;
    case DMA_SLAVE_BUSWIDTH_4_BYTES:
    es = CSDP_DATA_TYPE_32;
    break;
    default: /* not reached */
    return core::ptr::null_mut();
    }
// Now allocate and setup the descriptor.
    d = kzalloc_flex(*d, sg, sglen, GFP_ATOMIC);
    if (!d)
    return core::ptr::null_mut();
    d.sglen = sglen;
    d.dir = dir;
    d.dev_addr = dev_addr;
    d.es = es;
// When the port_window is used, one frame must cover the window
    if (port_window) {
    burst = port_window;
    port_window_bytes = port_window * es_bytes[es];
    d.ei = 1;
//
// One frame covers the port_window and by  configure
// the source frame index to be -1 * (port_window - 1)
// we instruct the sDMA that after a frame is processed
// it should move back to the start of the window.
//
    d.fi = -(port_window_bytes - 1);
    }
    d.ccr = c.ccr | CCR_SYNC_FRAME;
    if (dir == DMA_DEV_TO_MEM) {
    d.csdp = CSDP_DST_BURST_64 | CSDP_DST_PACKED;
    d.ccr |= CCR_DST_AMODE_POSTINC;
    if (port_window) {
    d.ccr |= CCR_SRC_AMODE_DBLIDX;
    if (port_window_bytes >= 64)
    d.csdp |= CSDP_SRC_BURST_64;
#[no_mangle]
pub unsafe extern "C" fn if(32: port_window_bytes >=) -> else {
    else if (port_window_bytes >= 32)
    d.csdp |= CSDP_SRC_BURST_32;
#[no_mangle]
pub unsafe extern "C" fn if(16: port_window_bytes >=) -> else {
    else if (port_window_bytes >= 16)
    d.csdp |= CSDP_SRC_BURST_16;
    } else {
    d.ccr |= CCR_SRC_AMODE_CONSTANT;
    }
    } else {
    d.csdp = CSDP_SRC_BURST_64 | CSDP_SRC_PACKED;
    d.ccr |= CCR_SRC_AMODE_POSTINC;
    if (port_window) {
    d.ccr |= CCR_DST_AMODE_DBLIDX;
    if (port_window_bytes >= 64)
    d.csdp |= CSDP_DST_BURST_64;
#[no_mangle]
pub unsafe extern "C" fn if(32: port_window_bytes >=) -> else {
    else if (port_window_bytes >= 32)
    d.csdp |= CSDP_DST_BURST_32;
#[no_mangle]
pub unsafe extern "C" fn if(16: port_window_bytes >=) -> else {
    else if (port_window_bytes >= 16)
    d.csdp |= CSDP_DST_BURST_16;
    } else {
    d.ccr |= CCR_DST_AMODE_CONSTANT;
    }
    }
    d.cicr = CICR_DROP_IE | CICR_BLOCK_IE;
    d.csdp |= es;
    if (dma_omap1()) {
    d.cicr |= CICR_TOUT_IE;
    if (dir == DMA_DEV_TO_MEM)
    d.csdp |= CSDP_DST_PORT_EMIFF | CSDP_SRC_PORT_TIPB;
    else
    d.csdp |= CSDP_DST_PORT_TIPB | CSDP_SRC_PORT_EMIFF;
    } else {
    if (dir == DMA_DEV_TO_MEM)
    d.ccr |= CCR_TRIGGER_SRC;
    d.cicr |= CICR_MISALIGNED_ERR_IE | CICR_TRANS_ERR_IE;
    if (port_window)
    d.csdp |= CSDP_WRITE_LAST_NON_POSTED;
    }
    if (od.plat.errata & DMA_ERRATA_PARALLEL_CHANNELS)
    d.clnk_ctrl = c.dma_ch;
//
// Build our scatterlist entries: each contains the address,
// the number of elements (EN) in each frame, and the number of
// frames (FN).  Number of bytes for this entry = ES * EN * FN.
//
// Burst size translates to number of elements with frame sync.
// Note: DMA engine defines burst to be the number of dev-width
// transfers.
//
    en = burst;
    frame_bytes = es_bytes[es] * en;
    if (sglen >= 2)
    d.using_ll = od.ll123_supported;
    for_each_sg(sgl, sgent, sglen, i) {
    struct omap_sg *osg = &d.sg[i];
    osg.addr = sg_dma_address(sgent);
    osg.en = en;
    osg.fn = sg_dma_len(sgent) / frame_bytes;
    if (d.using_ll) {
    osg.t2_desc = dma_pool_alloc(od.desc_pool, GFP_ATOMIC,
    &osg.t2_desc_paddr);
    if (!osg.t2_desc) {
    dev_err(chan.device.dev,
    "t2_desc[%d] allocation failed\n", i);
    ll_failed = true;
    d.using_ll = false;
    continue;
    }
    omap_dma_fill_type2_desc(d, i, dir, (i == sglen - 1));
    }
    }
// Release the dma_pool entries if one allocation failed
    if (ll_failed) {
    for (i = 0; i < d.sglen; i++) {
    struct omap_sg *osg = &d.sg[i];
    if (osg.t2_desc) {
    dma_pool_free(od.desc_pool, osg.t2_desc,
    osg.t2_desc_paddr);
    osg.t2_desc = core::ptr::null_mut();
    }
    }
    }
    return vchan_tx_prep(&c.vc, &d.vd, tx_flags);
    }
    static struct dma_async_tx_descriptor *omap_dma_prep_dma_cyclic(
    struct dma_chan *chan, dma_addr_t buf_addr, size_t buf_len,
    size_t period_len, enum dma_transfer_direction dir, unsigned long flags)
    {
    struct omap_dmadev *od = to_omap_dma_dev(chan.device);
    struct omap_chan *c = to_omap_dma_chan(chan);
    enum dma_slave_buswidth dev_width;
    struct omap_desc *d;
    dma_addr_t dev_addr;
    unsigned es;
    u32 burst;
    if (dir == DMA_DEV_TO_MEM) {
    dev_addr = c.cfg.src_addr;
    dev_width = c.cfg.src_addr_width;
    burst = c.cfg.src_maxburst;
    } else if (dir == DMA_MEM_TO_DEV) {
    dev_addr = c.cfg.dst_addr;
    dev_width = c.cfg.dst_addr_width;
    burst = c.cfg.dst_maxburst;
    } else {
    dev_err(chan.device.dev, "%s: bad direction?\n", __func__);
    return core::ptr::null_mut();
    }
// Bus width translates to the element size (ES)
    switch (dev_width) {
    case DMA_SLAVE_BUSWIDTH_1_BYTE:
    es = CSDP_DATA_TYPE_8;
    break;
    case DMA_SLAVE_BUSWIDTH_2_BYTES:
    es = CSDP_DATA_TYPE_16;
    break;
    case DMA_SLAVE_BUSWIDTH_4_BYTES:
    es = CSDP_DATA_TYPE_32;
    break;
    default: /* not reached */
    return core::ptr::null_mut();
    }
// Now allocate and setup the descriptor.
    d = kzalloc(sizeof(*d) + sizeof(d.sg[0]), GFP_ATOMIC);
    if (!d)
    return core::ptr::null_mut();
    d.dir = dir;
    d.dev_addr = dev_addr;
    d.fi = burst;
    d.es = es;
    d.sglen = 1;
    d.sg[0].addr = buf_addr;
    d.sg[0].en = period_len / es_bytes[es];
    d.sg[0].fn = buf_len / period_len;
    d.ccr = c.ccr;
    if (dir == DMA_DEV_TO_MEM)
    d.ccr |= CCR_DST_AMODE_POSTINC | CCR_SRC_AMODE_CONSTANT;
    else
    d.ccr |= CCR_DST_AMODE_CONSTANT | CCR_SRC_AMODE_POSTINC;
    d.cicr = CICR_DROP_IE;
    if (flags & DMA_PREP_INTERRUPT)
    d.cicr |= CICR_FRAME_IE;
    d.csdp = es;
    if (dma_omap1()) {
    d.cicr |= CICR_TOUT_IE;
    if (dir == DMA_DEV_TO_MEM)
    d.csdp |= CSDP_DST_PORT_EMIFF | CSDP_SRC_PORT_MPUI;
    else
    d.csdp |= CSDP_DST_PORT_MPUI | CSDP_SRC_PORT_EMIFF;
    } else {
    if (burst)
    d.ccr |= CCR_SYNC_PACKET;
    else
    d.ccr |= CCR_SYNC_ELEMENT;
    if (dir == DMA_DEV_TO_MEM) {
    d.ccr |= CCR_TRIGGER_SRC;
    d.csdp |= CSDP_DST_PACKED;
    } else {
    d.csdp |= CSDP_SRC_PACKED;
    }
    d.cicr |= CICR_MISALIGNED_ERR_IE | CICR_TRANS_ERR_IE;
    d.csdp |= CSDP_DST_BURST_64 | CSDP_SRC_BURST_64;
    }
    if (__dma_omap15xx(od.plat.dma_attr))
    d.ccr |= CCR_AUTO_INIT | CCR_REPEAT;
    else
    d.clnk_ctrl = c.dma_ch | CLNK_CTRL_ENABLE_LNK;
    c.cyclic = true;
    return vchan_tx_prep(&c.vc, &d.vd, flags);
    }
    static struct dma_async_tx_descriptor *omap_dma_prep_dma_memcpy(
    struct dma_chan *chan, dma_addr_t dest, dma_addr_t src,
    size_t len, unsigned long tx_flags)
    {
    struct omap_chan *c = to_omap_dma_chan(chan);
    struct omap_desc *d;
    uint8_t data_type;
    d = kzalloc(sizeof(*d) + sizeof(d.sg[0]), GFP_ATOMIC);
    if (!d)
    return core::ptr::null_mut();
    data_type = __ffs((src | dest | len));
    if (data_type > CSDP_DATA_TYPE_32)
    data_type = CSDP_DATA_TYPE_32;
    d.dir = DMA_MEM_TO_MEM;
    d.dev_addr = src;
    d.fi = 0;
    d.es = data_type;
    d.sglen = 1;
    d.sg[0].en = len / BIT(data_type);
    d.sg[0].fn = 1;
    d.sg[0].addr = dest;
    d.ccr = c.ccr;
    d.ccr |= CCR_DST_AMODE_POSTINC | CCR_SRC_AMODE_POSTINC;
    if (tx_flags & DMA_PREP_INTERRUPT)
    d.cicr |= CICR_FRAME_IE;
    else
    d.polled = true;
    d.csdp = data_type;
    if (dma_omap1()) {
    d.cicr |= CICR_TOUT_IE;
    d.csdp |= CSDP_DST_PORT_EMIFF | CSDP_SRC_PORT_EMIFF;
    } else {
    d.csdp |= CSDP_DST_PACKED | CSDP_SRC_PACKED;
    d.cicr |= CICR_MISALIGNED_ERR_IE | CICR_TRANS_ERR_IE;
    d.csdp |= CSDP_DST_BURST_64 | CSDP_SRC_BURST_64;
    }
    return vchan_tx_prep(&c.vc, &d.vd, tx_flags);
    }
    static struct dma_async_tx_descriptor *omap_dma_prep_dma_interleaved(
    struct dma_chan *chan, struct dma_interleaved_template *xt,
    unsigned long flags)
    {
    struct omap_chan *c = to_omap_dma_chan(chan);
    struct omap_desc *d;
    struct omap_sg *sg;
    uint8_t data_type;
    size_t src_icg, dst_icg;
// Slave mode is not supported
    if (is_slave_direction(xt.dir))
    return core::ptr::null_mut();
    if (xt.frame_size != 1 || xt.numf == 0)
    return core::ptr::null_mut();
    d = kzalloc(sizeof(*d) + sizeof(d.sg[0]), GFP_ATOMIC);
    if (!d)
    return core::ptr::null_mut();
    data_type = __ffs((xt.src_start | xt.dst_start | xt.sgl[0].size));
    if (data_type > CSDP_DATA_TYPE_32)
    data_type = CSDP_DATA_TYPE_32;
    d.sglen = 1;
    sg = &d.sg[0];
    d.dir = DMA_MEM_TO_MEM;
    d.dev_addr = xt.src_start;
    d.es = data_type;
    sg.en = xt.sgl[0].size / BIT(data_type);
    sg.fn = xt.numf;
    sg.addr = xt.dst_start;
    d.ccr = c.ccr;
    src_icg = dmaengine_get_src_icg(xt, &xt.sgl[0]);
    dst_icg = dmaengine_get_dst_icg(xt, &xt.sgl[0]);
    if (src_icg) {
    d.ccr |= CCR_SRC_AMODE_DBLIDX;
    d.ei = 1;
    d.fi = src_icg + 1;
    } else if (xt.src_inc) {
    d.ccr |= CCR_SRC_AMODE_POSTINC;
    d.fi = 0;
    } else {
    dev_err(chan.device.dev,
    "%s: SRC constant addressing is not supported\n",
    __func__);
    kfree(d);
    return core::ptr::null_mut();
    }
    if (dst_icg) {
    d.ccr |= CCR_DST_AMODE_DBLIDX;
    sg.ei = 1;
    sg.fi = dst_icg + 1;
    } else if (xt.dst_inc) {
    d.ccr |= CCR_DST_AMODE_POSTINC;
    sg.fi = 0;
    } else {
    dev_err(chan.device.dev,
    "%s: DST constant addressing is not supported\n",
    __func__);
    kfree(d);
    return core::ptr::null_mut();
    }
    d.cicr = CICR_DROP_IE | CICR_FRAME_IE;
    d.csdp = data_type;
    if (dma_omap1()) {
    d.cicr |= CICR_TOUT_IE;
    d.csdp |= CSDP_DST_PORT_EMIFF | CSDP_SRC_PORT_EMIFF;
    } else {
    d.csdp |= CSDP_DST_PACKED | CSDP_SRC_PACKED;
    d.cicr |= CICR_MISALIGNED_ERR_IE | CICR_TRANS_ERR_IE;
    d.csdp |= CSDP_DST_BURST_64 | CSDP_SRC_BURST_64;
    }
    return vchan_tx_prep(&c.vc, &d.vd, flags);
    }
#[no_mangle]
unsafe extern "C" fn omap_dma_slave_config(chan: *mut dma_chan, cfg: *mut dma_slave_config) -> c_int {
    static int omap_dma_slave_config(struct dma_chan *chan, struct dma_slave_config *cfg)
    {
    struct omap_chan *c = to_omap_dma_chan(chan);
    if (cfg.src_addr_width == DMA_SLAVE_BUSWIDTH_8_BYTES ||
    cfg.dst_addr_width == DMA_SLAVE_BUSWIDTH_8_BYTES)
    return -EINVAL;
    if (cfg.src_maxburst > chan.device.max_burst ||
    cfg.dst_maxburst > chan.device.max_burst)
    return -EINVAL;
    memcpy(&c.cfg, cfg, sizeof(c.cfg));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn omap_dma_terminate_all(chan: *mut dma_chan) -> c_int {
    static int omap_dma_terminate_all(struct dma_chan *chan)
    {
    struct omap_chan *c = to_omap_dma_chan(chan);
    unsigned long flags;
    LIST_HEAD(head);
    spin_lock_irqsave(&c.vc.lock, flags);
//
// Stop DMA activity: we assume the callback will not be called
// after omap_dma_stop() returns (even if it does, it will see
// c->desc is NULL and exit.)
//
    if (c.desc) {
    vchan_terminate_vdesc(&c.desc.vd);
    c.desc = core::ptr::null_mut();
// Avoid stopping the dma twice
    if (!c.paused)
    omap_dma_stop(c);
    }
    c.cyclic = false;
    c.paused = false;
    vchan_get_all_descriptors(&c.vc, &head);
    spin_unlock_irqrestore(&c.vc.lock, flags);
    vchan_dma_desc_free_list(&c.vc, &head);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn omap_dma_synchronize(chan: *mut dma_chan) {
    static void omap_dma_synchronize(struct dma_chan *chan)
    {
    struct omap_chan *c = to_omap_dma_chan(chan);
    vchan_synchronize(&c.vc);
    }
#[no_mangle]
unsafe extern "C" fn omap_dma_pause(chan: *mut dma_chan) -> c_int {
    static int omap_dma_pause(struct dma_chan *chan)
    {
    struct omap_chan *c = to_omap_dma_chan(chan);
    struct omap_dmadev *od = to_omap_dma_dev(chan.device);
    unsigned long flags;
    let mut ret: c_int = -EINVAL;
    let mut can_pause: bool = false;
    spin_lock_irqsave(&od.irq_lock, flags);
    if (!c.desc)
    goto out;
    if (c.cyclic)
    can_pause = true;
//
// We do not allow DMA_MEM_TO_DEV transfers to be paused.
// From the AM572x TRM, 16.1.4.18 Disabling a Channel During Transfer:
// "When a channel is disabled during a transfer, the channel undergoes
// an abort, unless it is hardware-source-synchronized …".
// A source-synchronised channel is one where the fetching of data is
// under control of the device. In other words, a device-to-memory
// transfer. So, a destination-synchronised channel (which would be a
// memory-to-device transfer) undergoes an abort if the CCR_ENABLE
// bit is cleared.
// From 16.1.4.20.4.6.2 Abort: "If an abort trigger occurs, the channel
// aborts immediately after completion of current read/write
// transactions and then the FIFO is cleaned up." The term "cleaned up"
// is not defined. TI recommends to check that RD_ACTIVE and WR_ACTIVE
// are both clear _before_ disabling the channel, otherwise data loss
// will occur.
// The problem is that if the channel is active, then device activity
// can result in DMA activity starting between reading those as both
// clear and the write to DMA_CCR to clear the enable bit hitting the
// hardware. If the DMA hardware can't drain the data in its FIFO to the
// destination, then data loss "might" occur (say if we write to an UART
// and the UART is not accepting any further data).
//
#[no_mangle]
pub unsafe extern "C" fn if(DMA_DEV_TO_MEM: c->desc->dir ==) -> else {
    else if (c.desc.dir == DMA_DEV_TO_MEM)
    can_pause = true;
    if (can_pause && !c.paused) {
    ret = omap_dma_stop(c);
    if (!ret)
    c.paused = true;
    }
    out:
    spin_unlock_irqrestore(&od.irq_lock, flags);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn omap_dma_resume(chan: *mut dma_chan) -> c_int {
    static int omap_dma_resume(struct dma_chan *chan)
    {
    struct omap_chan *c = to_omap_dma_chan(chan);
    struct omap_dmadev *od = to_omap_dma_dev(chan.device);
    unsigned long flags;
    let mut ret: c_int = -EINVAL;
    spin_lock_irqsave(&od.irq_lock, flags);
    if (c.paused && c.desc) {
    mb();
// Restore channel link register
    omap_dma_chan_write(c, CLNK_CTRL, c.desc.clnk_ctrl);
    omap_dma_start(c, c.desc);
    c.paused = false;
    ret = 0;
    }
    spin_unlock_irqrestore(&od.irq_lock, flags);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn omap_dma_chan_init(od: *mut omap_dmadev) -> c_int {
    static int omap_dma_chan_init(struct omap_dmadev *od)
    {
    struct omap_chan *c;
    c = kzalloc_obj(*c);
    if (!c)
    return -ENOMEM;
    c.reg_map = od.reg_map;
    c.vc.desc_free = omap_dma_desc_free;
    vchan_init(&c.vc, &od.ddev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn omap_dma_free(od: *mut omap_dmadev) {
    static void omap_dma_free(struct omap_dmadev *od)
    {
    while (!list_empty(&od.ddev.channels)) {
    struct omap_chan *c = list_first_entry(&od.ddev.channels,
    struct omap_chan, vc.chan.device_node);
    list_del(&c.vc.chan.device_node);
    tasklet_kill(&c.vc.task);
    kfree(c);
    }
    }
// Currently used by omap2 & 3 to block deeper SoC idle states
#[no_mangle]
unsafe extern "C" fn omap_dma_busy(od: *mut omap_dmadev) -> bool {
    static bool omap_dma_busy(struct omap_dmadev *od)
    {
    struct omap_chan *c;
    let mut lch: c_int = -1;
    while (1) {
    lch = find_next_bit(od.lch_bitmap, od.lch_count, lch + 1);
    if (lch >= od.lch_count)
    break;
    c = od.lch_map[lch];
    if (!c)
    continue;
    if (omap_dma_chan_read(c, CCR) & CCR_ENABLE)
    return true;
    }
    return false;
    }
// Currently only used for omap2. For omap1, also a check for lcd_dma is needed
    static int omap_dma_busy_notifier(struct notifier_block *nb,
    unsigned long cmd, void *v)
    {
    struct omap_dmadev *od;
    od = container_of(nb, struct omap_dmadev, nb);
    switch (cmd) {
    case CPU_CLUSTER_PM_ENTER:
    if (omap_dma_busy(od))
    return NOTIFY_BAD;
    break;
    case CPU_CLUSTER_PM_ENTER_FAILED:
    case CPU_CLUSTER_PM_EXIT:
    break;
    }
    return NOTIFY_OK;
    }
//
// We are using IRQENABLE_L1, and legacy DMA code was using IRQENABLE_L0.
// As the DSP may be using IRQENABLE_L2 and L3, let's not touch those for
// now. Context save seems to be only currently needed on omap3.
//
#[no_mangle]
unsafe extern "C" fn omap_dma_context_save(od: *mut omap_dmadev) {
    static void omap_dma_context_save(struct omap_dmadev *od)
    {
    od.context.irqenable_l0 = omap_dma_glbl_read(od, IRQENABLE_L0);
    od.context.irqenable_l1 = omap_dma_glbl_read(od, IRQENABLE_L1);
    od.context.ocp_sysconfig = omap_dma_glbl_read(od, OCP_SYSCONFIG);
    od.context.gcr = omap_dma_glbl_read(od, GCR);
    }
#[no_mangle]
unsafe extern "C" fn omap_dma_context_restore(od: *mut omap_dmadev) {
    static void omap_dma_context_restore(struct omap_dmadev *od)
    {
    int i;
    omap_dma_glbl_write(od, GCR, od.context.gcr);
    omap_dma_glbl_write(od, OCP_SYSCONFIG, od.context.ocp_sysconfig);
    omap_dma_glbl_write(od, IRQENABLE_L0, od.context.irqenable_l0);
    omap_dma_glbl_write(od, IRQENABLE_L1, od.context.irqenable_l1);
// Clear IRQSTATUS_L0 as legacy DMA code is no longer doing it
    if (od.plat.errata & DMA_ROMCODE_BUG)
    omap_dma_glbl_write(od, IRQSTATUS_L0, 0);
// Clear dma channels
    for (i = 0; i < od.lch_count; i++)
    omap_dma_clear_lch(od, i);
    }
// Currently only used for omap3
    static int omap_dma_context_notifier(struct notifier_block *nb,
    unsigned long cmd, void *v)
    {
    struct omap_dmadev *od;
    od = container_of(nb, struct omap_dmadev, nb);
    switch (cmd) {
    case CPU_CLUSTER_PM_ENTER:
    if (omap_dma_busy(od))
    return NOTIFY_BAD;
    omap_dma_context_save(od);
    break;
    case CPU_CLUSTER_PM_ENTER_FAILED:	/* No need to restore context */
    break;
    case CPU_CLUSTER_PM_EXIT:
    omap_dma_context_restore(od);
    break;
    }
    return NOTIFY_OK;
    }
    static void omap_dma_init_gcr(struct omap_dmadev *od, int arb_rate,
    int max_fifo_depth, int tparams)
    {
    u32 val;
// Set only for omap2430 and later
    if (!od.cfg.rw_priority)
    return;
    if (max_fifo_depth == 0)
    max_fifo_depth = 1;
    if (arb_rate == 0)
    arb_rate = 1;
    val = 0xff & max_fifo_depth;
    val |= (0x3 & tparams) << 12;
    val |= (arb_rate & 0xff) << 16;
    omap_dma_glbl_write(od, GCR, val);
    }

    BIT(DMA_SLAVE_BUSWIDTH_2_BYTES) | \
    BIT(DMA_SLAVE_BUSWIDTH_4_BYTES))
//
// No flags currently set for default configuration as omap1 is still
// using platform data.
//
    static const struct omap_dma_config default_cfg;
#[no_mangle]
unsafe extern "C" fn omap_dma_probe(pdev: *mut platform_device) -> c_int {
    static int omap_dma_probe(struct platform_device *pdev)
    {
    const struct omap_dma_config *conf;
    struct omap_dmadev *od;
    int rc, i, irq;
    u32 val;
    od = devm_kzalloc(&pdev.dev, sizeof(*od), GFP_KERNEL);
    if (!od)
    return -ENOMEM;
    od.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(od.base))
    return PTR_ERR(od.base);
    conf = of_device_get_match_data(&pdev.dev);
    if (conf) {
    od.cfg = conf;
    od.plat = dev_get_platdata(&pdev.dev);
    if (!od.plat) {
    dev_err(&pdev.dev, "omap_system_dma_plat_info is missing");
    return -ENODEV;
    }
    } else if (IS_ENABLED(CONFIG_ARCH_OMAP1)) {
    od.cfg = &default_cfg;
    od.plat = omap_get_plat_info();
    if (!od.plat)
    return -EPROBE_DEFER;
    } else {
    return -ENODEV;
    }
    od.reg_map = od.plat.reg_map;
    dma_cap_set(DMA_SLAVE, od.ddev.cap_mask);
    dma_cap_set(DMA_CYCLIC, od.ddev.cap_mask);
    dma_cap_set(DMA_MEMCPY, od.ddev.cap_mask);
    dma_cap_set(DMA_INTERLEAVE, od.ddev.cap_mask);
    od.ddev.device_alloc_chan_resources = omap_dma_alloc_chan_resources;
    od.ddev.device_free_chan_resources = omap_dma_free_chan_resources;
    od.ddev.device_tx_status = omap_dma_tx_status;
    od.ddev.device_issue_pending = omap_dma_issue_pending;
    od.ddev.device_prep_slave_sg = omap_dma_prep_slave_sg;
    od.ddev.device_prep_dma_cyclic = omap_dma_prep_dma_cyclic;
    od.ddev.device_prep_dma_memcpy = omap_dma_prep_dma_memcpy;
    od.ddev.device_prep_interleaved_dma = omap_dma_prep_dma_interleaved;
    od.ddev.device_config = omap_dma_slave_config;
    od.ddev.device_pause = omap_dma_pause;
    od.ddev.device_resume = omap_dma_resume;
    od.ddev.device_terminate_all = omap_dma_terminate_all;
    od.ddev.device_synchronize = omap_dma_synchronize;
    od.ddev.src_addr_widths = OMAP_DMA_BUSWIDTHS;
    od.ddev.dst_addr_widths = OMAP_DMA_BUSWIDTHS;
    od.ddev.directions = BIT(DMA_DEV_TO_MEM) | BIT(DMA_MEM_TO_DEV);
    if (__dma_omap15xx(od.plat.dma_attr))
    od.ddev.residue_granularity =
    DMA_RESIDUE_GRANULARITY_DESCRIPTOR;
    else
    od.ddev.residue_granularity = DMA_RESIDUE_GRANULARITY_BURST;
    od.ddev.max_burst = SZ_16M - 1; /* CCEN: 24bit unsigned */
    od.ddev.dev = &pdev.dev;
    INIT_LIST_HEAD(&od.ddev.channels);
    mutex_init(&od.lch_lock);
    spin_lock_init(&od.lock);
    spin_lock_init(&od.irq_lock);
// Number of DMA requests
    od.dma_requests = OMAP_SDMA_REQUESTS;
    if (pdev.dev.of_node && of_property_read_u32(pdev.dev.of_node,
    "dma-requests",
    &od.dma_requests)) {
    dev_info(&pdev.dev,
    "Missing dma-requests property, using %u.\n",
    OMAP_SDMA_REQUESTS);
    }
// Number of available logical channels
    if (!pdev.dev.of_node) {
    od.lch_count = od.plat.dma_attr.lch_count;
    if (unlikely(!od.lch_count))
    od.lch_count = OMAP_SDMA_CHANNELS;
    } else if (of_property_read_u32(pdev.dev.of_node, "dma-channels",
    &od.lch_count)) {
    dev_info(&pdev.dev,
    "Missing dma-channels property, using %u.\n",
    OMAP_SDMA_CHANNELS);
    od.lch_count = OMAP_SDMA_CHANNELS;
    }
// Mask of allowed logical channels
    if (pdev.dev.of_node && !of_property_read_u32(pdev.dev.of_node,
    "dma-channel-mask",
    &val)) {
// Tag channels not in mask as reserved
    val = ~val;
    bitmap_from_arr32(od.lch_bitmap, &val, od.lch_count);
    }
    if (od.plat.dma_attr.dev_caps & HS_CHANNELS_RESERVED)
    bitmap_set(od.lch_bitmap, 0, 2);
    od.lch_map = devm_kcalloc(&pdev.dev, od.lch_count,
    sizeof(*od.lch_map),
    GFP_KERNEL);
    if (!od.lch_map)
    return -ENOMEM;
    for (i = 0; i < od.dma_requests; i++) {
    rc = omap_dma_chan_init(od);
    if (rc) {
    omap_dma_free(od);
    return rc;
    }
    }
    irq = platform_get_irq(pdev, 1);
    if (irq <= 0) {
    dev_info(&pdev.dev, "failed to get L1 IRQ: %d\n", irq);
    od.legacy = true;
    } else {
// Disable all interrupts
    od.irq_enable_mask = 0;
    omap_dma_glbl_write(od, IRQENABLE_L1, 0);
    rc = devm_request_irq(&pdev.dev, irq, omap_dma_irq,
    IRQF_SHARED, "omap-dma-engine", od);
    if (rc) {
    omap_dma_free(od);
    return rc;
    }
    }
    if (omap_dma_glbl_read(od, CAPS_0) & CAPS_0_SUPPORT_LL123)
    od.ll123_supported = true;
    od.ddev.filter.map = od.plat.slave_map;
    od.ddev.filter.mapcnt = od.plat.slavecnt;
    od.ddev.filter.fn = omap_dma_filter_fn;
    if (od.ll123_supported) {
    od.desc_pool = dma_pool_create(dev_name(&pdev.dev),
    &pdev.dev,
    sizeof(struct omap_type2_desc),
    4, 0);
    if (!od.desc_pool) {
    dev_err(&pdev.dev,
    "unable to allocate descriptor pool\n");
    od.ll123_supported = false;
    }
    }
    rc = dma_async_device_register(&od.ddev);
    if (rc) {
    pr_warn("OMAP-DMA: failed to register slave DMA engine device: %d\n",
    rc);
    if (od.ll123_supported)
    dma_pool_destroy(od.desc_pool);
    omap_dma_free(od);
    return rc;
    }
    platform_set_drvdata(pdev, od);
    if (pdev.dev.of_node) {
    omap_dma_info.dma_cap = od.ddev.cap_mask;
// Device-tree DMA controller registration
    rc = of_dma_controller_register(pdev.dev.of_node,
    of_dma_simple_xlate, &omap_dma_info);
    if (rc) {
    pr_warn("OMAP-DMA: failed to register DMA controller\n");
    dma_async_device_unregister(&od.ddev);
    if (od.ll123_supported)
    dma_pool_destroy(od.desc_pool);
    omap_dma_free(od);
    }
    }
    omap_dma_init_gcr(od, DMA_DEFAULT_ARB_RATE, DMA_DEFAULT_FIFO_DEPTH, 0);
    if (od.cfg.needs_busy_check) {
    od.nb.notifier_call = omap_dma_busy_notifier;
    cpu_pm_register_notifier(&od.nb);
    } else if (od.cfg.may_lose_context) {
    od.nb.notifier_call = omap_dma_context_notifier;
    cpu_pm_register_notifier(&od.nb);
    }
    dev_info(&pdev.dev, "OMAP DMA engine driver%s\n",
    od.ll123_supported ? " (LinkedList1/2/3 supported)" : "");
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn omap_dma_remove(pdev: *mut platform_device) {
    static void omap_dma_remove(struct platform_device *pdev)
    {
    struct omap_dmadev *od = platform_get_drvdata(pdev);
    int irq;
    if (od.cfg.may_lose_context)
    cpu_pm_unregister_notifier(&od.nb);
    if (pdev.dev.of_node)
    of_dma_controller_free(pdev.dev.of_node);
    irq = platform_get_irq(pdev, 1);
    devm_free_irq(&pdev.dev, irq, od);
    dma_async_device_unregister(&od.ddev);
    if (!omap_dma_legacy(od)) {
// Disable all interrupts
    omap_dma_glbl_write(od, IRQENABLE_L0, 0);
    }
    if (od.ll123_supported)
    dma_pool_destroy(od.desc_pool);
    omap_dma_free(od);
    }
    static const struct omap_dma_config omap2420_data = {
    .lch_end = CCFN,
    .rw_priority = true,
    .needs_lch_clear = true,
    .needs_busy_check = true,
    };
    static const struct omap_dma_config omap2430_data = {
    .lch_end = CCFN,
    .rw_priority = true,
    .needs_lch_clear = true,
    };
    static const struct omap_dma_config omap3430_data = {
    .lch_end = CCFN,
    .rw_priority = true,
    .needs_lch_clear = true,
    .may_lose_context = true,
    };
    static const struct omap_dma_config omap3630_data = {
    .lch_end = CCDN,
    .rw_priority = true,
    .needs_lch_clear = true,
    .may_lose_context = true,
    };
    static const struct omap_dma_config omap4_data = {
    .lch_end = CCDN,
    .rw_priority = true,
    .needs_lch_clear = true,
    };
    static const struct of_device_id omap_dma_match[] = {
    { .compatible = "ti,omap2420-sdma", .data = &omap2420_data, },
    { .compatible = "ti,omap2430-sdma", .data = &omap2430_data, },
    { .compatible = "ti,omap3430-sdma", .data = &omap3430_data, },
    { .compatible = "ti,omap3630-sdma", .data = &omap3630_data, },
    { .compatible = "ti,omap4430-sdma", .data = &omap4_data, },
    {},
    };
    MODULE_DEVICE_TABLE(of, omap_dma_match);
    static struct platform_driver omap_dma_driver = {
    .probe	= omap_dma_probe,
    .remove = omap_dma_remove,
    .driver = {
    .name = "omap-dma-engine",
    .of_match_table = omap_dma_match,
    },
    };
#[no_mangle]
unsafe extern "C" fn omap_dma_filter_fn(chan: *mut dma_chan, param: *mut c_void) -> bool {
    static bool omap_dma_filter_fn(struct dma_chan *chan, void *param)
    {
    if (chan.device.dev.driver == &omap_dma_driver.driver) {
    struct omap_dmadev *od = to_omap_dma_dev(chan.device);
    struct omap_chan *c = to_omap_dma_chan(chan);
    let mut req: unsigned = *(unsigned *)param;
    if (req <= od.dma_requests) {
    c.dma_sig = req;
    return true;
    }
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn omap_dma_init() -> c_int {
    static int omap_dma_init(void)
    {
    return platform_driver_register(&omap_dma_driver);
    }
    subsys_initcall(omap_dma_init);
#[no_mangle]
unsafe extern "C" fn omap_dma_exit() -> void __exit {
    static void __exit omap_dma_exit(void)
    {
    platform_driver_unregister(&omap_dma_driver);
    }
    module_exit(omap_dma_exit);
    MODULE_AUTHOR("Russell King");
    MODULE_DESCRIPTION("Texas Instruments sDMA DMAengine support");
    MODULE_LICENSE("GPL");
