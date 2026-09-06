//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/imx/dcss/dcss-ctxld.c
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
// Copyright 2019 NXP.
//

pub const DCSS_CTXLD_CONTROL_STATUS: c_uint = 0x0;

pub const DCSS_CTXLD_DB_BASE_ADDR: c_uint = 0x10;
pub const DCSS_CTXLD_DB_COUNT: c_uint = 0x14;
pub const DCSS_CTXLD_SB_BASE_ADDR: c_uint = 0x18;
pub const DCSS_CTXLD_SB_COUNT: c_uint = 0x1C;
pub const SB_HP_COUNT_POS: c_int = 0;
pub const SB_HP_COUNT_MASK: c_uint = 0xffff;
pub const SB_LP_COUNT_POS: c_int = 16;
pub const SB_LP_COUNT_MASK: c_uint = 0xffff0000;
pub const DCSS_AHB_ERR_ADDR: c_uint = 0x20;

// The following sizes are in context loader entries, 8 bytes each.

    CTXLD_SB_HP_CTX_ENTRIES)
// Sizes, in entries, of the DB, SB_HP and SB_LP context regions.
    static u16 dcss_ctxld_ctx_size[3] = {
    CTXLD_DB_CTX_ENTRIES,
    CTXLD_SB_HP_CTX_ENTRIES,
    CTXLD_SB_LP_CTX_ENTRIES
    };
// this represents an entry in the context loader map
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcss_ctxld_item {
    pub val: u32,
    pub ofs: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcss_ctxld {
    pub dev: *mut device,
    pub ctxld_reg: *mut void __iomem,
    pub irq: c_int,
    pub irq_en: bool,
    pub db: [*mut dcss_ctxld_item; 2],
    pub sb_hp: [*mut dcss_ctxld_item; 2],
    pub sb_lp: [*mut dcss_ctxld_item; 2],
    pub db_paddr: [dma_addr_t; 2],
    pub sb_paddr: [dma_addr_t; 2],
    pub /: *mut *mut u16 ctx_size[2][3]; / holds the sizes of DB, SB_HP and SB_LP ctx,
    pub current_ctx: u8,
    pub in_use: bool,
    pub armed: bool,
    pub /: *mut *mut spinlock_t lock; / protects concurent access to private data,
}

#[no_mangle]
unsafe extern "C" fn dcss_ctxld_irq_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t dcss_ctxld_irq_handler(int irq, void *data)
    {
    struct dcss_ctxld *ctxld = data;
    struct dcss_dev *dcss = dcss_drv_dev_to_dcss(ctxld.dev);
    u32 irq_status;
    irq_status = dcss_readl(ctxld.ctxld_reg + DCSS_CTXLD_CONTROL_STATUS);
    if (irq_status & CTXLD_IRQ_COMPLETION &&
    !(irq_status & CTXLD_ENABLE) && ctxld.in_use) {
    ctxld.in_use = false;
    if (dcss && dcss.disable_callback)
    dcss.disable_callback(dcss);
    } else if (irq_status & CTXLD_IRQ_ERROR) {
//
// Except for throwing an error message and clearing the status
// register, there's not much we can do here.
//
    dev_err(ctxld.dev, "ctxld: error encountered: %08x\n",
    irq_status);
    dev_err(ctxld.dev, "ctxld: db=%d, sb_hp=%d, sb_lp=%d\n",
    ctxld.ctx_size[ctxld.current_ctx ^ 1][CTX_DB],
    ctxld.ctx_size[ctxld.current_ctx ^ 1][CTX_SB_HP],
    ctxld.ctx_size[ctxld.current_ctx ^ 1][CTX_SB_LP]);
    }
    dcss_clr(irq_status & (CTXLD_IRQ_ERROR | CTXLD_IRQ_COMPLETION),
    ctxld.ctxld_reg + DCSS_CTXLD_CONTROL_STATUS);
    return IRQ_HANDLED;
    }
    static int dcss_ctxld_irq_config(struct dcss_ctxld *ctxld,
    struct platform_device *pdev)
    {
    int ret;
    ctxld.irq = platform_get_irq_byname(pdev, "ctxld");
    if (ctxld.irq < 0)
    return ctxld.irq;
    ret = request_irq(ctxld.irq, dcss_ctxld_irq_handler,
    0, "dcss_ctxld", ctxld);
    if (ret) {
    dev_err(ctxld.dev, "ctxld: irq request failed.\n");
    return ret;
    }
    ctxld.irq_en = true;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dcss_ctxld_hw_cfg(ctxld: *mut dcss_ctxld) {
    static void dcss_ctxld_hw_cfg(struct dcss_ctxld *ctxld)
    {
    dcss_writel(RD_ERR_EN | SB_HP_COMP_EN |
    DB_PEND_SB_REC_EN | AHB_ERR_EN | RD_ERR | AHB_ERR,
    ctxld.ctxld_reg + DCSS_CTXLD_CONTROL_STATUS);
    }
#[no_mangle]
unsafe extern "C" fn dcss_ctxld_free_ctx(ctxld: *mut dcss_ctxld) {
    static void dcss_ctxld_free_ctx(struct dcss_ctxld *ctxld)
    {
    struct dcss_ctxld_item *ctx;
    int i;
    for (i = 0; i < 2; i++) {
    if (ctxld.db[i]) {
    dma_free_coherent(ctxld.dev,
    CTXLD_DB_CTX_ENTRIES * sizeof(*ctx),
    ctxld.db[i], ctxld.db_paddr[i]);
    ctxld.db[i] = core::ptr::null_mut();
    ctxld.db_paddr[i] = 0;
    }
    if (ctxld.sb_hp[i]) {
    dma_free_coherent(ctxld.dev,
    CTXLD_SB_CTX_ENTRIES * sizeof(*ctx),
    ctxld.sb_hp[i], ctxld.sb_paddr[i]);
    ctxld.sb_hp[i] = core::ptr::null_mut();
    ctxld.sb_paddr[i] = 0;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn dcss_ctxld_alloc_ctx(ctxld: *mut dcss_ctxld) -> c_int {
    static int dcss_ctxld_alloc_ctx(struct dcss_ctxld *ctxld)
    {
    struct dcss_ctxld_item *ctx;
    int i;
    for (i = 0; i < 2; i++) {
    ctx = dma_alloc_coherent(ctxld.dev,
    CTXLD_DB_CTX_ENTRIES * sizeof(*ctx),
    &ctxld.db_paddr[i], GFP_KERNEL);
    if (!ctx)
    return -ENOMEM;
    ctxld.db[i] = ctx;
    ctx = dma_alloc_coherent(ctxld.dev,
    CTXLD_SB_CTX_ENTRIES * sizeof(*ctx),
    &ctxld.sb_paddr[i], GFP_KERNEL);
    if (!ctx)
    return -ENOMEM;
    ctxld.sb_hp[i] = ctx;
    ctxld.sb_lp[i] = ctx + CTXLD_SB_HP_CTX_ENTRIES;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn dcss_ctxld_init(dcss: *mut dcss_dev, ctxld_base: c_ulong) -> c_int {
    int dcss_ctxld_init(struct dcss_dev *dcss, unsigned long ctxld_base)
    {
    struct dcss_ctxld *ctxld;
    int ret;
    ctxld = devm_kzalloc(dcss.dev, sizeof(*ctxld), GFP_KERNEL);
    if (!ctxld)
    return -ENOMEM;
    dcss.ctxld = ctxld;
    ctxld.dev = dcss.dev;
    spin_lock_init(&ctxld.lock);
    ret = dcss_ctxld_alloc_ctx(ctxld);
    if (ret) {
    dev_err(dcss.dev, "ctxld: cannot allocate context memory.\n");
    goto err;
    }
    ctxld.ctxld_reg = devm_ioremap(dcss.dev, ctxld_base, SZ_4K);
    if (!ctxld.ctxld_reg) {
    dev_err(dcss.dev, "ctxld: unable to remap ctxld base\n");
    ret = -ENOMEM;
    goto err;
    }
    ret = dcss_ctxld_irq_config(ctxld, to_platform_device(dcss.dev));
    if (ret)
    goto err;
    dcss_ctxld_hw_cfg(ctxld);
    return 0;
    err:
    dcss_ctxld_free_ctx(ctxld);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn dcss_ctxld_exit(ctxld: *mut dcss_ctxld) {
    void dcss_ctxld_exit(struct dcss_ctxld *ctxld)
    {
    free_irq(ctxld.irq, ctxld);
    dcss_ctxld_free_ctx(ctxld);
    }
#[no_mangle]
unsafe extern "C" fn dcss_ctxld_enable_locked(ctxld: *mut dcss_ctxld) -> c_int {
    static int dcss_ctxld_enable_locked(struct dcss_ctxld *ctxld)
    {
    let mut curr_ctx: c_int = ctxld.current_ctx;
    u32 db_base, sb_base, sb_count;
    u32 sb_hp_cnt, sb_lp_cnt, db_cnt;
    struct dcss_dev *dcss = dcss_drv_dev_to_dcss(ctxld.dev);
    if (!dcss)
    return 0;
    dcss_dpr_write_sysctrl(dcss.dpr);
    dcss_scaler_write_sclctrl(dcss.scaler);
    sb_hp_cnt = ctxld.ctx_size[curr_ctx][CTX_SB_HP];
    sb_lp_cnt = ctxld.ctx_size[curr_ctx][CTX_SB_LP];
    db_cnt = ctxld.ctx_size[curr_ctx][CTX_DB];
// make sure SB_LP context area comes after SB_HP
    if (sb_lp_cnt &&
    ctxld.sb_lp[curr_ctx] != ctxld.sb_hp[curr_ctx] + sb_hp_cnt) {
    struct dcss_ctxld_item *sb_lp_adjusted;
    sb_lp_adjusted = ctxld.sb_hp[curr_ctx] + sb_hp_cnt;
    memcpy(sb_lp_adjusted, ctxld.sb_lp[curr_ctx],
    sb_lp_cnt * CTX_ITEM_SIZE);
    }
    db_base = db_cnt ? ctxld.db_paddr[curr_ctx] : 0;
    dcss_writel(db_base, ctxld.ctxld_reg + DCSS_CTXLD_DB_BASE_ADDR);
    dcss_writel(db_cnt, ctxld.ctxld_reg + DCSS_CTXLD_DB_COUNT);
    if (sb_hp_cnt)
    sb_count = ((sb_hp_cnt << SB_HP_COUNT_POS) & SB_HP_COUNT_MASK) |
    ((sb_lp_cnt << SB_LP_COUNT_POS) & SB_LP_COUNT_MASK);
    else
    sb_count = (sb_lp_cnt << SB_HP_COUNT_POS) & SB_HP_COUNT_MASK;
    sb_base = sb_count ? ctxld.sb_paddr[curr_ctx] : 0;
    dcss_writel(sb_base, ctxld.ctxld_reg + DCSS_CTXLD_SB_BASE_ADDR);
    dcss_writel(sb_count, ctxld.ctxld_reg + DCSS_CTXLD_SB_COUNT);
// enable the context loader
    dcss_set(CTXLD_ENABLE, ctxld.ctxld_reg + DCSS_CTXLD_CONTROL_STATUS);
    ctxld.in_use = true;
//
// Toggle the current context to the alternate one so that any updates
// in the modules' settings take place there.
//
    ctxld.current_ctx ^= 1;
    ctxld.ctx_size[ctxld.current_ctx][CTX_DB] = 0;
    ctxld.ctx_size[ctxld.current_ctx][CTX_SB_HP] = 0;
    ctxld.ctx_size[ctxld.current_ctx][CTX_SB_LP] = 0;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn dcss_ctxld_enable(ctxld: *mut dcss_ctxld) -> c_int {
    int dcss_ctxld_enable(struct dcss_ctxld *ctxld)
    {
    spin_lock_irq(&ctxld.lock);
    ctxld.armed = true;
    spin_unlock_irq(&ctxld.lock);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn dcss_ctxld_kick(ctxld: *mut dcss_ctxld) {
    void dcss_ctxld_kick(struct dcss_ctxld *ctxld)
    {
    unsigned long flags;
    spin_lock_irqsave(&ctxld.lock, flags);
    if (ctxld.armed && !ctxld.in_use) {
    ctxld.armed = false;
    dcss_ctxld_enable_locked(ctxld);
    }
    spin_unlock_irqrestore(&ctxld.lock, flags);
    }
    void dcss_ctxld_write_irqsafe(struct dcss_ctxld *ctxld, u32 ctx_id, u32 val,
    u32 reg_ofs)
    {
    let mut curr_ctx: c_int = ctxld.current_ctx;
    struct dcss_ctxld_item *ctx[] = {
    [CTX_DB] = ctxld.db[curr_ctx],
    [CTX_SB_HP] = ctxld.sb_hp[curr_ctx],
    [CTX_SB_LP] = ctxld.sb_lp[curr_ctx]
    };
    let mut item_idx: c_int = ctxld.ctx_size[curr_ctx][ctx_id];
    if (item_idx + 1 > dcss_ctxld_ctx_size[ctx_id]) {
    WARN_ON(1);
    return;
    }
    ctx[ctx_id][item_idx].val = val;
    ctx[ctx_id][item_idx].ofs = reg_ofs;
    ctxld.ctx_size[curr_ctx][ctx_id] += 1;
    }
    void dcss_ctxld_write(struct dcss_ctxld *ctxld, u32 ctx_id,
    u32 val, u32 reg_ofs)
    {
    spin_lock_irq(&ctxld.lock);
    dcss_ctxld_write_irqsafe(ctxld, ctx_id, val, reg_ofs);
    spin_unlock_irq(&ctxld.lock);
    }
#[no_mangle]
pub unsafe extern "C" fn dcss_ctxld_is_flushed(ctxld: *mut dcss_ctxld) -> bool {
    bool dcss_ctxld_is_flushed(struct dcss_ctxld *ctxld)
    {
    return ctxld.ctx_size[ctxld.current_ctx][CTX_DB] == 0 &&
    ctxld.ctx_size[ctxld.current_ctx][CTX_SB_HP] == 0 &&
    ctxld.ctx_size[ctxld.current_ctx][CTX_SB_LP] == 0;
    }
#[no_mangle]
pub unsafe extern "C" fn dcss_ctxld_resume(ctxld: *mut dcss_ctxld) -> c_int {
    int dcss_ctxld_resume(struct dcss_ctxld *ctxld)
    {
    dcss_ctxld_hw_cfg(ctxld);
    if (!ctxld.irq_en) {
    enable_irq(ctxld.irq);
    ctxld.irq_en = true;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn dcss_ctxld_suspend(ctxld: *mut dcss_ctxld) -> c_int {
    int dcss_ctxld_suspend(struct dcss_ctxld *ctxld)
    {
    let mut ret: c_int = 0;
    let mut timeout: c_ulong = jiffies + msecs_to_jiffies(500);
    if (!dcss_ctxld_is_flushed(ctxld)) {
    dcss_ctxld_kick(ctxld);
    while (!time_after(jiffies, timeout) && ctxld.in_use)
    msleep(20);
    if (time_after(jiffies, timeout))
    return -ETIMEDOUT;
    }
    spin_lock_irq(&ctxld.lock);
    if (ctxld.irq_en) {
    disable_irq_nosync(ctxld.irq);
    ctxld.irq_en = false;
    }
// reset context region and sizes
    ctxld.current_ctx = 0;
    ctxld.ctx_size[0][CTX_DB] = 0;
    ctxld.ctx_size[0][CTX_SB_HP] = 0;
    ctxld.ctx_size[0][CTX_SB_LP] = 0;
    spin_unlock_irq(&ctxld.lock);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn dcss_ctxld_assert_locked(ctxld: *mut dcss_ctxld) {
    void dcss_ctxld_assert_locked(struct dcss_ctxld *ctxld)
    {
    lockdep_assert_held(&ctxld.lock);
    }
