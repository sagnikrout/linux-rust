//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-stm32-ospi.c
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
// Copyright (C) STMicroelectronics 2025 - All Rights Reserved
//

pub const OSPI_CR: c_uint = 0x00;

pub const CR_FTHRES_SHIFT: c_int = 8;

pub const OSPI_DCR1: c_uint = 0x08;

pub const DCR1_MTYP_MX_MODE: c_int = 1;
pub const DCR1_MTYP_HP_MEMMODE: c_int = 4;
pub const OSPI_DCR2: c_uint = 0x0c;

pub const OSPI_SR: c_uint = 0x20;

pub const OSPI_FCR: c_uint = 0x24;

pub const OSPI_DLR: c_uint = 0x40;
pub const OSPI_AR: c_uint = 0x48;
pub const OSPI_DR: c_uint = 0x50;
pub const OSPI_PSMKR: c_uint = 0x80;
pub const OSPI_PSMAR: c_uint = 0x88;
pub const OSPI_CCR: c_uint = 0x100;

pub const CCR_ADMODE_8LINES: c_int = 4;

pub const CCR_ADSIZE_32BITS: c_int = 3;

pub const CCR_DMODE_8LINES: c_int = 4;

pub const CCR_BUSWIDTH_0: c_uint = 0x0;
pub const CCR_BUSWIDTH_1: c_uint = 0x1;
pub const CCR_BUSWIDTH_2: c_uint = 0x2;
pub const CCR_BUSWIDTH_4: c_uint = 0x3;
pub const CCR_BUSWIDTH_8: c_uint = 0x4;
pub const OSPI_TCR: c_uint = 0x108;

pub const OSPI_IR: c_uint = 0x110;

pub const STM32_OSPI_MAX_NORCHIP: c_int = 2;
pub const STM32_FIFO_TIMEOUT_US: c_int = 30000;
pub const STM32_ABT_TIMEOUT_US: c_int = 100000;
pub const STM32_COMP_TIMEOUT_MS: c_int = 5000;
pub const STM32_BUSY_TIMEOUT_US: c_int = 100000;
pub const STM32_WAIT_CMD_TIMEOUT_US: c_int = 5000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32_ospi {
    pub dev: *mut device,
    pub ctrl: *mut spi_controller,
    pub clk: *mut clk,
    pub rstc: *mut reset_control,
    pub match_completion: completion,
    pub dma_chtx: *mut dma_chan,
    pub dma_chrx: *mut dma_chan,
    pub dma_completion: completion,
    pub regs_base: *mut void __iomem,
    pub mm_base: *mut void __iomem,
    pub regs_phys_base: phys_addr_t,
    pub mm_size: resource_size_t,
    pub clk_rate: u32,
    pub fmode: u32,
    pub cr_reg: u32,
    pub dcr_reg: u32,
    pub flash_presc: [u32; STM32_OSPI_MAX_NORCHIP],
    pub irq: c_int,
    pub status_timeout: c_ulong,
//
// To protect device configuration, could be different between
// 2 flash access
//
    pub lock: mutex,
}

#[no_mangle]
unsafe extern "C" fn stm32_ospi_read_fifo(val: *mut c_void, addr: *mut void __iomem, len: u8) {
    static void stm32_ospi_read_fifo(void *val, void __iomem *addr, u8 len)
    {
    switch (len) {
    case sizeof(u32):
// ((u32 *)val) = readl_relaxed(addr);
    break;
    case sizeof(u16):
// ((u16 *)val) = readw_relaxed(addr);
    break;
    case sizeof(u8):
// ((u8 *)val) = readb_relaxed(addr);
    }
    }
#[no_mangle]
unsafe extern "C" fn stm32_ospi_write_fifo(val: *mut c_void, addr: *mut void __iomem, len: u8) {
    static void stm32_ospi_write_fifo(void *val, void __iomem *addr, u8 len)
    {
    switch (len) {
    case sizeof(u32):
    writel_relaxed(*((u32 *)val), addr);
    break;
    case sizeof(u16):
    writew_relaxed(*((u16 *)val), addr);
    break;
    case sizeof(u8):
    writeb_relaxed(*((u8 *)val), addr);
    }
    }
#[no_mangle]
unsafe extern "C" fn stm32_ospi_abort(ospi: *mut stm32_ospi) -> c_int {
    static int stm32_ospi_abort(struct stm32_ospi *ospi)
    {
    void __iomem *regs_base = ospi.regs_base;
    u32 cr;
    int timeout;
    cr = readl_relaxed(regs_base + OSPI_CR) | CR_ABORT;
    writel_relaxed(cr, regs_base + OSPI_CR);
// wait clear of abort bit by hw
    timeout = readl_relaxed_poll_timeout_atomic(regs_base + OSPI_CR,
    cr, !(cr & CR_ABORT), 1,
    STM32_ABT_TIMEOUT_US);
    if (timeout)
    dev_err(ospi.dev, "%s abort timeout:%d\n", __func__, timeout);
    return timeout;
    }
#[no_mangle]
unsafe extern "C" fn stm32_ospi_poll(ospi: *mut stm32_ospi, buf: *mut c_void, len: u32, read: bool) -> c_int {
    static int stm32_ospi_poll(struct stm32_ospi *ospi, void *buf, u32 len, bool read)
    {
    void __iomem *regs_base = ospi.regs_base;
    void (*fifo)(void *val, void __iomem *addr, u8 len);
    u32 sr;
    int ret;
    u8 step;
    if (read)
    fifo = stm32_ospi_read_fifo;
    else
    fifo = stm32_ospi_write_fifo;
    while (len) {
    ret = readl_relaxed_poll_timeout_atomic(regs_base + OSPI_SR,
    sr, sr & SR_FTF, 1,
    STM32_FIFO_TIMEOUT_US);
    if (ret) {
    dev_err(ospi.dev, "fifo timeout (len:%d stat:%#x)\n",
    len, sr);
    return ret;
    }
    if (len >= sizeof(u32))
    step = sizeof(u32);
#[no_mangle]
pub unsafe extern "C" fn if(sizeof(u16): len >=) -> else {
    else if (len >= sizeof(u16))
    step = sizeof(u16);
    else
    step = sizeof(u8);
    fifo(buf, regs_base + OSPI_DR, step);
    len -= step;
    buf += step;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stm32_ospi_wait_nobusy(ospi: *mut stm32_ospi) -> c_int {
    static int stm32_ospi_wait_nobusy(struct stm32_ospi *ospi)
    {
    u32 sr;
    return readl_relaxed_poll_timeout_atomic(ospi.regs_base + OSPI_SR,
    sr, !(sr & SR_BUSY), 1,
    STM32_BUSY_TIMEOUT_US);
    }
#[no_mangle]
unsafe extern "C" fn stm32_ospi_wait_cmd(ospi: *mut stm32_ospi) -> c_int {
    static int stm32_ospi_wait_cmd(struct stm32_ospi *ospi)
    {
    void __iomem *regs_base = ospi.regs_base;
    u32 sr;
    let mut err: c_int = 0;
    if (ospi.fmode == CR_FMODE_APM)
    goto out;
    err = readl_relaxed_poll_timeout_atomic(ospi.regs_base + OSPI_SR, sr,
    (sr & (SR_TEF | SR_TCF)), 1,
    STM32_WAIT_CMD_TIMEOUT_US);
    if (sr & SR_TCF)
// avoid false timeout
    err = 0;
    if (sr & SR_TEF)
    err = -EIO;
    out:
// clear flags
    writel_relaxed(FCR_CTCF | FCR_CTEF, regs_base + OSPI_FCR);
    if (!err)
    err = stm32_ospi_wait_nobusy(ospi);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn stm32_ospi_dma_callback(arg: *mut c_void) {
    static void stm32_ospi_dma_callback(void *arg)
    {
    struct completion *dma_completion = arg;
    complete(dma_completion);
    }
#[no_mangle]
unsafe extern "C" fn stm32_ospi_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t stm32_ospi_irq(int irq, void *dev_id)
    {
    struct stm32_ospi *ospi = (struct stm32_ospi *)dev_id;
    void __iomem *regs_base = ospi.regs_base;
    u32 cr, sr;
    cr = readl_relaxed(regs_base + OSPI_CR);
    sr = readl_relaxed(regs_base + OSPI_SR);
    if (sr & SR_SMF) {
// disable irq
    cr &= ~CR_SMIE;
    writel_relaxed(cr, regs_base + OSPI_CR);
    complete(&ospi.match_completion);
    }
    return IRQ_HANDLED;
    }
    static int stm32_ospi_dma_setup(struct stm32_ospi *ospi,
    struct dma_slave_config *dma_cfg)
    {
    struct dma_slave_caps caps;
    let mut ret: c_int = 0;
    if (dma_cfg && ospi.dma_chrx) {
    ret = dma_get_slave_caps(ospi.dma_chrx, &caps);
    if (ret)
    return ret;
    dma_cfg.src_maxburst = caps.max_burst / dma_cfg.src_addr_width;
    if (dmaengine_slave_config(ospi.dma_chrx, dma_cfg)) {
    dev_err(ospi.dev, "dma rx config failed\n");
    dma_release_channel(ospi.dma_chrx);
    ospi.dma_chrx = core::ptr::null_mut();
    }
    }
    if (dma_cfg && ospi.dma_chtx) {
    ret = dma_get_slave_caps(ospi.dma_chtx, &caps);
    if (ret)
    return ret;
    dma_cfg.dst_maxburst = caps.max_burst / dma_cfg.dst_addr_width;
    if (dmaengine_slave_config(ospi.dma_chtx, dma_cfg)) {
    dev_err(ospi.dev, "dma tx config failed\n");
    dma_release_channel(ospi.dma_chtx);
    ospi.dma_chtx = core::ptr::null_mut();
    }
    }
    init_completion(&ospi.dma_completion);
    return ret;
    }
    static int stm32_ospi_tx_mm(struct stm32_ospi *ospi,
    const struct spi_mem_op *op)
    {
    memcpy_fromio(op.data.buf.in, ospi.mm_base + op.addr.val,
    op.data.nbytes);
    return 0;
    }
    static int stm32_ospi_tx_dma(struct stm32_ospi *ospi,
    const struct spi_mem_op *op)
    {
    struct dma_async_tx_descriptor *desc;
    void __iomem *regs_base = ospi.regs_base;
    enum dma_transfer_direction dma_dir;
    struct dma_chan *dma_ch;
    struct sg_table sgt;
    dma_cookie_t cookie;
    u32 cr, t_out;
    int err;
    if (op.data.dir == SPI_MEM_DATA_IN) {
    dma_dir = DMA_DEV_TO_MEM;
    dma_ch = ospi.dma_chrx;
    } else {
    dma_dir = DMA_MEM_TO_DEV;
    dma_ch = ospi.dma_chtx;
    }
//
// Spi_map_buf return -EINVAL if the buffer is not DMA-able
// (DMA-able: in vmalloc | kmap | virt_addr_valid)
//
    err = spi_controller_dma_map_mem_op_data(ospi.ctrl, op, &sgt);
    if (err)
    return err;
    desc = dmaengine_prep_slave_sg(dma_ch, sgt.sgl, sgt.nents,
    dma_dir, DMA_PREP_INTERRUPT);
    if (!desc) {
    err = -ENOMEM;
    goto out_unmap;
    }
    cr = readl_relaxed(regs_base + OSPI_CR);
    reinit_completion(&ospi.dma_completion);
    desc.callback = stm32_ospi_dma_callback;
    desc.callback_param = &ospi.dma_completion;
    cookie = dmaengine_submit(desc);
    err = dma_submit_error(cookie);
    if (err)
    goto out;
    dma_async_issue_pending(dma_ch);
    writel_relaxed(cr | CR_DMAEN, regs_base + OSPI_CR);
    t_out = sgt.nents * STM32_COMP_TIMEOUT_MS;
    if (!wait_for_completion_timeout(&ospi.dma_completion,
    msecs_to_jiffies(t_out)))
    err = -ETIMEDOUT;
    if (err)
    dmaengine_terminate_all(dma_ch);
    out:
    writel_relaxed(cr & ~CR_DMAEN, regs_base + OSPI_CR);
    out_unmap:
    spi_controller_dma_unmap_mem_op_data(ospi.ctrl, op, &sgt);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn stm32_ospi_xfer(ospi: *mut stm32_ospi, op: *const spi_mem_op) -> c_int {
    static int stm32_ospi_xfer(struct stm32_ospi *ospi, const struct spi_mem_op *op)
    {
    u8 *buf;
    if (!op.data.nbytes)
    return 0;
    if (ospi.fmode == CR_FMODE_MM)
    return stm32_ospi_tx_mm(ospi, op);
    else if (((op.data.dir == SPI_MEM_DATA_IN && ospi.dma_chrx) ||
    (op.data.dir == SPI_MEM_DATA_OUT && ospi.dma_chtx)) &&
    op.data.nbytes > 8)
    if (!stm32_ospi_tx_dma(ospi, op))
    return 0;
    if (op.data.dir == SPI_MEM_DATA_IN)
    buf = op.data.buf.in;
    else
    buf = (void *)op.data.buf.out;
    return stm32_ospi_poll(ospi, buf, op.data.nbytes,
    op.data.dir == SPI_MEM_DATA_IN);
    }
    static int stm32_ospi_wait_poll_status(struct stm32_ospi *ospi,
    const struct spi_mem_op *op)
    {
    void __iomem *regs_base = ospi.regs_base;
    u32 cr;
    reinit_completion(&ospi.match_completion);
    cr = readl_relaxed(regs_base + OSPI_CR);
    writel_relaxed(cr | CR_SMIE, regs_base + OSPI_CR);
    if (!wait_for_completion_timeout(&ospi.match_completion,
    msecs_to_jiffies(ospi.status_timeout))) {
    let mut sr: u32 = readl_relaxed(regs_base + OSPI_SR);
// Avoid false timeout
    if (!(sr & SR_SMF))
    return -ETIMEDOUT;
    }
    writel_relaxed(FCR_CSMF, regs_base + OSPI_FCR);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stm32_ospi_get_mode(buswidth: u8) -> c_int {
    static int stm32_ospi_get_mode(u8 buswidth)
    {
    switch (buswidth) {
    case 8:
    return CCR_BUSWIDTH_8;
    case 4:
    return CCR_BUSWIDTH_4;
    default:
    return buswidth;
    }
    }
#[no_mangle]
unsafe extern "C" fn stm32_ospi_send(spi: *mut spi_device, op: *const spi_mem_op) -> c_int {
    static int stm32_ospi_send(struct spi_device *spi, const struct spi_mem_op *op)
    {
    struct stm32_ospi *ospi = spi_controller_get_devdata(spi.controller);
    void __iomem *regs_base = ospi.regs_base;
    u32 ccr, cr, dcr2, tcr;
    int timeout, err = 0, err_poll_status = 0;
    let mut cs: u8 = spi.chip_select[ffs(spi.cs_index_mask) - 1];
    cr = readl_relaxed(ospi.regs_base + OSPI_CR);
    FIELD_MODIFY(CR_CSSEL, &cr, cs);
    FIELD_MODIFY(CR_FMODE_MASK, &cr, ospi.fmode);
    writel_relaxed(cr, regs_base + OSPI_CR);
    if (op.data.nbytes)
    writel_relaxed(op.data.nbytes - 1, regs_base + OSPI_DLR);
// set prescaler
    dcr2 = readl_relaxed(regs_base + OSPI_DCR2);
    dcr2 |= FIELD_PREP(DCR2_PRESC_MASK, ospi.flash_presc[cs]);
    writel_relaxed(dcr2, regs_base + OSPI_DCR2);
    ccr = FIELD_PREP(CCR_IMODE_MASK, stm32_ospi_get_mode(op.cmd.buswidth));
    if (op.addr.nbytes) {
    ccr |= FIELD_PREP(CCR_ADMODE_MASK,
    stm32_ospi_get_mode(op.addr.buswidth));
    ccr |= FIELD_PREP(CCR_ADSIZE_MASK, op.addr.nbytes - 1);
    }
    tcr = TCR_SSHIFT;
    if (op.dummy.buswidth && op.dummy.nbytes) {
    tcr |= FIELD_PREP(TCR_DCYC_MASK,
    op.dummy.nbytes * 8 / op.dummy.buswidth);
    }
    writel_relaxed(tcr, regs_base + OSPI_TCR);
    if (op.data.nbytes) {
    ccr |= FIELD_PREP(CCR_DMODE_MASK,
    stm32_ospi_get_mode(op.data.buswidth));
    }
    writel_relaxed(ccr, regs_base + OSPI_CCR);
// set instruction, must be set after ccr register update
    writel_relaxed(op.cmd.opcode, regs_base + OSPI_IR);
    if (op.addr.nbytes && ospi.fmode != CR_FMODE_MM)
    writel_relaxed(op.addr.val, regs_base + OSPI_AR);
    if (ospi.fmode == CR_FMODE_APM)
    err_poll_status = stm32_ospi_wait_poll_status(ospi, op);
    err = stm32_ospi_xfer(ospi, op);
//
// Abort in:
// -error case
// -read memory map: prefetching must be stopped if we read the last
// byte of device (device size - fifo size). like device size is not
// knows, the prefetching is always stop.
//
    if (err || err_poll_status || ospi.fmode == CR_FMODE_MM)
    goto abort;
// Wait end of tx in indirect mode
    err = stm32_ospi_wait_cmd(ospi);
    if (err)
    goto abort;
    return 0;
    abort:
    timeout = stm32_ospi_abort(ospi);
    writel_relaxed(FCR_CTCF | FCR_CSMF, regs_base + OSPI_FCR);
    if (err || err_poll_status || timeout)
    dev_err(ospi.dev, "%s err:%d err_poll_status:%d abort timeout:%d\n",
    __func__, err, err_poll_status, timeout);
    return err;
    }
    static int stm32_ospi_poll_status(struct spi_mem *mem,
    const struct spi_mem_op *op,
    u16 mask, u16 match,
    unsigned long initial_delay_us,
    unsigned long polling_rate_us,
    unsigned long timeout_ms)
    {
    struct stm32_ospi *ospi = spi_controller_get_devdata(mem.spi.controller);
    void __iomem *regs_base = ospi.regs_base;
    int ret;
    ret = pm_runtime_resume_and_get(ospi.dev);
    if (ret < 0)
    return ret;
    mutex_lock(&ospi.lock);
    writel_relaxed(mask, regs_base + OSPI_PSMKR);
    writel_relaxed(match, regs_base + OSPI_PSMAR);
    ospi.fmode = CR_FMODE_APM;
    ospi.status_timeout = timeout_ms;
    ret = stm32_ospi_send(mem.spi, op);
    mutex_unlock(&ospi.lock);
    pm_runtime_put_autosuspend(ospi.dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn stm32_ospi_exec_op(mem: *mut spi_mem, op: *const spi_mem_op) -> c_int {
    static int stm32_ospi_exec_op(struct spi_mem *mem, const struct spi_mem_op *op)
    {
    struct stm32_ospi *ospi = spi_controller_get_devdata(mem.spi.controller);
    int ret;
    ret = pm_runtime_resume_and_get(ospi.dev);
    if (ret < 0)
    return ret;
    mutex_lock(&ospi.lock);
    if (op.data.dir == SPI_MEM_DATA_IN && op.data.nbytes)
    ospi.fmode = CR_FMODE_INDR;
    else
    ospi.fmode = CR_FMODE_INDW;
    ret = stm32_ospi_send(mem.spi, op);
    mutex_unlock(&ospi.lock);
    pm_runtime_put_autosuspend(ospi.dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn stm32_ospi_dirmap_create(desc: *mut spi_mem_dirmap_desc) -> c_int {
    static int stm32_ospi_dirmap_create(struct spi_mem_dirmap_desc *desc)
    {
    struct stm32_ospi *ospi = spi_controller_get_devdata(desc.mem.spi.controller);
    if (desc.info.op_tmpl.data.dir == SPI_MEM_DATA_OUT)
    return -EOPNOTSUPP;
// Should never happen, as mm_base == null is an error probe exit condition
    if (!ospi.mm_base && desc.info.op_tmpl.data.dir == SPI_MEM_DATA_IN)
    return -EOPNOTSUPP;
    if (!ospi.mm_size)
    return -EOPNOTSUPP;
    return 0;
    }
    static ssize_t stm32_ospi_dirmap_read(struct spi_mem_dirmap_desc *desc,
    u64 offs, size_t len, void *buf)
    {
    struct stm32_ospi *ospi = spi_controller_get_devdata(desc.mem.spi.controller);
    struct spi_mem_op op;
    u32 addr_max;
    int ret;
    ret = pm_runtime_resume_and_get(ospi.dev);
    if (ret < 0)
    return ret;
    mutex_lock(&ospi.lock);
//
// Make a local copy of desc op_tmpl and complete dirmap rdesc
// spi_mem_op template with offs, len and *buf in  order to get
// all needed transfer information into struct spi_mem_op
//
    memcpy(&op, desc.info.op_tmpl, sizeof(struct spi_mem_op));
    dev_dbg(ospi.dev, "%s len = 0x%zx offs = 0x%llx buf = 0x%p\n", __func__, len, offs, buf);
    op.data.nbytes = len;
    op.addr.val = desc.info.offset + offs;
    op.data.buf.in = buf;
    addr_max = op.addr.val + op.data.nbytes + 1;
    if (addr_max < ospi.mm_size && op.addr.buswidth)
    ospi.fmode = CR_FMODE_MM;
    else
    ospi.fmode = CR_FMODE_INDR;
    ret = stm32_ospi_send(desc.mem.spi, &op);
    mutex_unlock(&ospi.lock);
    pm_runtime_put_autosuspend(ospi.dev);
    return ret ?: len;
    }
    static int stm32_ospi_transfer_one_message(struct spi_controller *ctrl,
    struct spi_message *msg)
    {
    struct stm32_ospi *ospi = spi_controller_get_devdata(ctrl);
    struct spi_transfer *transfer;
    struct spi_device *spi = msg.spi;
    struct spi_mem_op op;
    struct gpio_desc *cs_gpiod = spi.cs_gpiod[ffs(spi.cs_index_mask) - 1];
    let mut ret: c_int = 0;
    if (!cs_gpiod)
    return -EOPNOTSUPP;
    ret = pm_runtime_resume_and_get(ospi.dev);
    if (ret < 0)
    return ret;
    mutex_lock(&ospi.lock);
    gpiod_set_value_cansleep(cs_gpiod, true);
    list_for_each_entry(transfer, &msg.transfers, transfer_list) {
    let mut dummy_bytes: u8 = 0;
    memset(&op, 0, sizeof(op));
    dev_dbg(ospi.dev, "tx_buf:%p tx_nbits:%d rx_buf:%p rx_nbits:%d len:%d dummy_data:%d\n",
    transfer.tx_buf, transfer.tx_nbits,
    transfer.rx_buf, transfer.rx_nbits,
    transfer.len, transfer.dummy_data);
//
// OSPI hardware supports dummy bytes transfer.
// If current transfer is dummy byte, merge it with the next
// transfer in order to take into account OSPI block constraint
//
    if (transfer.dummy_data) {
    op.dummy.buswidth = transfer.tx_nbits;
    op.dummy.nbytes = transfer.len;
    dummy_bytes = transfer.len;
// If happens, means that message is not correctly built
    if (list_is_last(&transfer.transfer_list, &msg.transfers)) {
    ret = -EINVAL;
    goto end_of_transfer;
    }
    transfer = list_next_entry(transfer, transfer_list);
    }
    op.data.nbytes = transfer.len;
    if (transfer.rx_buf) {
    ospi.fmode = CR_FMODE_INDR;
    op.data.buswidth = transfer.rx_nbits;
    op.data.dir = SPI_MEM_DATA_IN;
    op.data.buf.in = transfer.rx_buf;
    } else {
    ospi.fmode = CR_FMODE_INDW;
    op.data.buswidth = transfer.tx_nbits;
    op.data.dir = SPI_MEM_DATA_OUT;
    op.data.buf.out = transfer.tx_buf;
    }
    ret = stm32_ospi_send(spi, &op);
    if (ret)
    goto end_of_transfer;
    msg.actual_length += transfer.len + dummy_bytes;
    }
    end_of_transfer:
    gpiod_set_value_cansleep(cs_gpiod, false);
    mutex_unlock(&ospi.lock);
    msg.status = ret;
    spi_finalize_current_message(ctrl);
    pm_runtime_put_autosuspend(ospi.dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn stm32_ospi_setup(spi: *mut spi_device) -> c_int {
    static int stm32_ospi_setup(struct spi_device *spi)
    {
    struct spi_controller *ctrl = spi.controller;
    struct stm32_ospi *ospi = spi_controller_get_devdata(ctrl);
    void __iomem *regs_base = ospi.regs_base;
    int ret;
    let mut cs: u8 = spi.chip_select[ffs(spi.cs_index_mask) - 1];
    if (ctrl.busy)
    return -EBUSY;
    if (!spi.max_speed_hz)
    return -EINVAL;
    ret = pm_runtime_resume_and_get(ospi.dev);
    if (ret < 0)
    return ret;
    ospi.flash_presc[cs] = DIV_ROUND_UP(ospi.clk_rate, spi.max_speed_hz) - 1;
    mutex_lock(&ospi.lock);
    ospi.cr_reg = CR_APMS | 3 << CR_FTHRES_SHIFT | CR_EN;
    writel_relaxed(ospi.cr_reg, regs_base + OSPI_CR);
// set dcr fsize to max address
    ospi.dcr_reg = DCR1_DEVSIZE_MASK | DCR1_DLYBYP;
    writel_relaxed(ospi.dcr_reg, regs_base + OSPI_DCR1);
    mutex_unlock(&ospi.lock);
    pm_runtime_put_autosuspend(ospi.dev);
    return 0;
    }
//
// No special host constraint, so use default spi_mem_default_supports_op
// to check supported mode.
//
    static const struct spi_controller_mem_ops stm32_ospi_mem_ops = {
    .exec_op	= stm32_ospi_exec_op,
    .dirmap_create	= stm32_ospi_dirmap_create,
    .dirmap_read	= stm32_ospi_dirmap_read,
    .poll_status	= stm32_ospi_poll_status,
    };
#[no_mangle]
unsafe extern "C" fn stm32_ospi_get_resources(pdev: *mut platform_device) -> c_int {
    static int stm32_ospi_get_resources(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct stm32_ospi *ospi = platform_get_drvdata(pdev);
    struct resource *res, _res;
    int ret;
    ospi.regs_base = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(ospi.regs_base))
    return PTR_ERR(ospi.regs_base);
    ospi.regs_phys_base = res.start;
    ospi.clk = devm_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(ospi.clk))
    return dev_err_probe(dev, PTR_ERR(ospi.clk),
    "Can't get clock\n");
    ospi.clk_rate = clk_get_rate(ospi.clk);
    if (!ospi.clk_rate) {
    dev_err(dev, "Invalid clock rate\n");
    return -EINVAL;
    }
    ospi.irq = platform_get_irq(pdev, 0);
    if (ospi.irq < 0)
    return ospi.irq;
    ret = devm_request_irq(dev, ospi.irq, stm32_ospi_irq, 0,
    dev_name(dev), ospi);
    if (ret) {
    dev_err(dev, "Failed to request irq\n");
    return ret;
    }
    ospi.rstc = devm_reset_control_array_get_exclusive_released(dev);
    if (IS_ERR(ospi.rstc))
    return dev_err_probe(dev, PTR_ERR(ospi.rstc),
    "Can't get reset\n");
    ospi.dma_chrx = dma_request_chan(dev, "rx");
    if (IS_ERR(ospi.dma_chrx)) {
    ret = PTR_ERR(ospi.dma_chrx);
    ospi.dma_chrx = core::ptr::null_mut();
    if (ret == -EPROBE_DEFER)
    goto err_dma;
    }
    ospi.dma_chtx = dma_request_chan(dev, "tx");
    if (IS_ERR(ospi.dma_chtx)) {
    ret = PTR_ERR(ospi.dma_chtx);
    ospi.dma_chtx = core::ptr::null_mut();
    if (ret == -EPROBE_DEFER)
    goto err_dma;
    }
    res = &_res;
    ret = of_reserved_mem_region_to_resource(dev.of_node, 0, res);
    if (!ret) {
    ospi.mm_size = resource_size(res);
    ospi.mm_base = devm_ioremap_resource(dev, res);
    if (IS_ERR(ospi.mm_base)) {
    dev_err(dev, "unable to map memory region: %pR\n", res);
    ret = PTR_ERR(ospi.mm_base);
    goto err_dma;
    }
    if (ospi.mm_size > STM32_OSPI_MAX_MMAP_SZ) {
    dev_err(dev, "Memory map size outsize bounds\n");
    ret = -EINVAL;
    goto err_dma;
    }
    } else {
    dev_info(dev, "No memory-map region found\n");
    }
    init_completion(&ospi.match_completion);
    return 0;
    err_dma:
    dev_info(dev, "Can't get all resources (%d)\n", ret);
    if (ospi.dma_chtx)
    dma_release_channel(ospi.dma_chtx);
    if (ospi.dma_chrx)
    dma_release_channel(ospi.dma_chrx);
    return ret;
    };
#[no_mangle]
unsafe extern "C" fn stm32_ospi_probe(pdev: *mut platform_device) -> c_int {
    static int stm32_ospi_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct spi_controller *ctrl;
    struct stm32_ospi *ospi;
    struct dma_slave_config dma_cfg;
    struct device_node *child;
    int ret;
    let mut spi_flash_count: u8 = 0;
//
// Flash subnodes sanity check:
// 1 or 2 spi-nand/spi-nor flashes		=> supported
// All other flash node configuration		=> not supported
//
    for_each_available_child_of_node(dev.of_node, child) {
    if (of_device_is_compatible(child, "jedec,spi-nor") ||
    of_device_is_compatible(child, "spi-nand"))
    spi_flash_count++;
    }
    if (spi_flash_count == 0 || spi_flash_count > 2) {
    dev_err(dev, "Incorrect DT flash node\n");
    return -ENODEV;
    }
    ctrl = devm_spi_alloc_host(dev, sizeof(*ospi));
    if (!ctrl)
    return -ENOMEM;
    ospi = spi_controller_get_devdata(ctrl);
    ospi.ctrl = ctrl;
    ospi.dev = &pdev.dev;
    platform_set_drvdata(pdev, ospi);
    ret = stm32_ospi_get_resources(pdev);
    if (ret)
    return ret;
    memset(&dma_cfg, 0, sizeof(dma_cfg));
    dma_cfg.src_addr_width = DMA_SLAVE_BUSWIDTH_1_BYTE;
    dma_cfg.dst_addr_width = DMA_SLAVE_BUSWIDTH_1_BYTE;
    dma_cfg.src_addr = ospi.regs_phys_base + OSPI_DR;
    dma_cfg.dst_addr = ospi.regs_phys_base + OSPI_DR;
    ret = stm32_ospi_dma_setup(ospi, &dma_cfg);
    if (ret)
    goto err_dma_free;
    mutex_init(&ospi.lock);
    ctrl.mode_bits = SPI_RX_DUAL | SPI_RX_QUAD |
    SPI_TX_DUAL | SPI_TX_QUAD |
    SPI_TX_OCTAL | SPI_RX_OCTAL;
    ctrl.flags = SPI_CONTROLLER_HALF_DUPLEX;
    ctrl.setup = stm32_ospi_setup;
    ctrl.bus_num = -1;
    ctrl.mem_ops = &stm32_ospi_mem_ops;
    ctrl.use_gpio_descriptors = true;
    ctrl.transfer_one_message = stm32_ospi_transfer_one_message;
    ctrl.num_chipselect = STM32_OSPI_MAX_NORCHIP;
    pm_runtime_enable(ospi.dev);
    pm_runtime_set_autosuspend_delay(ospi.dev, STM32_AUTOSUSPEND_DELAY);
    pm_runtime_use_autosuspend(ospi.dev);
    ret = pm_runtime_resume_and_get(ospi.dev);
    if (ret < 0)
    goto err_pm_enable;
    ret = reset_control_acquire(ospi.rstc);
    if (ret) {
    dev_err_probe(dev, ret, "Can not acquire reset %d\n", ret);
    goto err_pm_resume;
    }
    reset_control_assert(ospi.rstc);
    udelay(2);
    reset_control_deassert(ospi.rstc);
    ret = spi_register_controller(ctrl);
    if (ret) {
// Disable ospi
    writel_relaxed(0, ospi.regs_base + OSPI_CR);
    goto err_reset_control;
    }
    pm_runtime_put_autosuspend(ospi.dev);
    return 0;
    err_reset_control:
    reset_control_release(ospi.rstc);
    err_pm_resume:
    pm_runtime_put_sync_suspend(ospi.dev);
    err_pm_enable:
    pm_runtime_force_suspend(ospi.dev);
    mutex_destroy(&ospi.lock);
    err_dma_free:
    if (ospi.dma_chtx)
    dma_release_channel(ospi.dma_chtx);
    if (ospi.dma_chrx)
    dma_release_channel(ospi.dma_chrx);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn stm32_ospi_remove(pdev: *mut platform_device) {
    static void stm32_ospi_remove(struct platform_device *pdev)
    {
    struct stm32_ospi *ospi = platform_get_drvdata(pdev);
    pm_runtime_resume_and_get(ospi.dev);
    spi_unregister_controller(ospi.ctrl);
// Disable ospi
    writel_relaxed(0, ospi.regs_base + OSPI_CR);
    mutex_destroy(&ospi.lock);
    if (ospi.dma_chtx)
    dma_release_channel(ospi.dma_chtx);
    if (ospi.dma_chrx)
    dma_release_channel(ospi.dma_chrx);
    reset_control_release(ospi.rstc);
    pm_runtime_put_sync_suspend(ospi.dev);
    pm_runtime_force_suspend(ospi.dev);
    }
#[no_mangle]
unsafe extern "C" fn stm32_ospi_suspend(dev: *mut device) -> c_int {
    static int stm32_ospi_suspend(struct device *dev)
    {
    struct stm32_ospi *ospi = dev_get_drvdata(dev);
    pinctrl_pm_select_sleep_state(dev);
    reset_control_release(ospi.rstc);
    return pm_runtime_force_suspend(ospi.dev);
    }
#[no_mangle]
unsafe extern "C" fn stm32_ospi_resume(dev: *mut device) -> c_int {
    static int stm32_ospi_resume(struct device *dev)
    {
    struct stm32_ospi *ospi = dev_get_drvdata(dev);
    void __iomem *regs_base = ospi.regs_base;
    int ret;
    ret = pm_runtime_force_resume(ospi.dev);
    if (ret < 0)
    return ret;
    pinctrl_pm_select_default_state(dev);
    ret = pm_runtime_resume_and_get(ospi.dev);
    if (ret < 0)
    return ret;
    ret = reset_control_acquire(ospi.rstc);
    if (ret) {
    dev_err(dev, "Can not acquire reset\n");
    return ret;
    }
    writel_relaxed(ospi.cr_reg, regs_base + OSPI_CR);
    writel_relaxed(ospi.dcr_reg, regs_base + OSPI_DCR1);
    pm_runtime_put_autosuspend(ospi.dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stm32_ospi_runtime_suspend(dev: *mut device) -> c_int {
    static int stm32_ospi_runtime_suspend(struct device *dev)
    {
    struct stm32_ospi *ospi = dev_get_drvdata(dev);
    clk_disable_unprepare(ospi.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stm32_ospi_runtime_resume(dev: *mut device) -> c_int {
    static int stm32_ospi_runtime_resume(struct device *dev)
    {
    struct stm32_ospi *ospi = dev_get_drvdata(dev);
    return clk_prepare_enable(ospi.clk);
    }
    static const struct dev_pm_ops stm32_ospi_pm_ops = {
    SYSTEM_SLEEP_PM_OPS(stm32_ospi_suspend, stm32_ospi_resume)
    RUNTIME_PM_OPS(stm32_ospi_runtime_suspend, stm32_ospi_runtime_resume, core::ptr::null_mut())
    };
    static const struct of_device_id stm32_ospi_of_match[] = {
    { .compatible = "st,stm32mp25-ospi" },
    {},
    };
    MODULE_DEVICE_TABLE(of, stm32_ospi_of_match);
    static struct platform_driver stm32_ospi_driver = {
    .probe	= stm32_ospi_probe,
    .remove	= stm32_ospi_remove,
    .driver	= {
    .name = "stm32-ospi",
    .pm = pm_ptr(&stm32_ospi_pm_ops),
    .of_match_table = stm32_ospi_of_match,
    },
    };
    module_platform_driver(stm32_ospi_driver);
    MODULE_DESCRIPTION("STMicroelectronics STM32 OCTO SPI driver");
    MODULE_LICENSE("GPL");
