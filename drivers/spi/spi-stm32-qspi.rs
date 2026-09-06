//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-stm32-qspi.c
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
// Copyright (C) STMicroelectronics 2018 - All Rights Reserved
// Author: Ludovic Barre <ludovic.barre@st.com> for STMicroelectronics.
//

pub const QSPI_CR: c_uint = 0x00;

pub const CR_FTHRES_SHIFT: c_int = 8;

pub const QSPI_DCR: c_uint = 0x04;

pub const QSPI_SR: c_uint = 0x08;

pub const QSPI_FCR: c_uint = 0x0c;

pub const QSPI_DLR: c_uint = 0x10;
pub const QSPI_CCR: c_uint = 0x14;

pub const CCR_BUSWIDTH_0: c_uint = 0x0;
pub const CCR_BUSWIDTH_1: c_uint = 0x1;
pub const CCR_BUSWIDTH_2: c_uint = 0x2;
pub const CCR_BUSWIDTH_4: c_uint = 0x3;
pub const QSPI_AR: c_uint = 0x18;
pub const QSPI_ABR: c_uint = 0x1c;
pub const QSPI_DR: c_uint = 0x20;
pub const QSPI_PSMKR: c_uint = 0x24;
pub const QSPI_PSMAR: c_uint = 0x28;
pub const QSPI_PIR: c_uint = 0x2c;
pub const QSPI_LPTR: c_uint = 0x30;

pub const STM32_QSPI_MAX_NORCHIP: c_int = 2;
pub const STM32_FIFO_TIMEOUT_US: c_int = 30000;
pub const STM32_BUSY_TIMEOUT_US: c_int = 100000;
pub const STM32_ABT_TIMEOUT_US: c_int = 100000;
pub const STM32_WAIT_CMD_TIMEOUT_US: c_int = 5000;
pub const STM32_COMP_TIMEOUT_MS: c_int = 1000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32_qspi_flash {
    pub cs: u32,
    pub presc: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32_qspi {
    pub dev: *mut device,
    pub ctrl: *mut spi_controller,
    pub phys_base: phys_addr_t,
    pub io_base: *mut void __iomem,
    pub mm_base: *mut void __iomem,
    pub mm_size: resource_size_t,
    pub clk: *mut clk,
    pub clk_rate: u32,
    pub flash: [stm32_qspi_flash; STM32_QSPI_MAX_NORCHIP],
    pub match_completion: completion,
    pub fmode: u32,
    pub dma_chtx: *mut dma_chan,
    pub dma_chrx: *mut dma_chan,
    pub dma_completion: completion,
    pub cr_reg: u32,
    pub dcr_reg: u32,
    pub status_timeout: c_ulong,
//
// to protect device configuration, could be different between
// 2 flash access (bk1, bk2)
//
    pub lock: mutex,
}

#[no_mangle]
unsafe extern "C" fn stm32_qspi_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t stm32_qspi_irq(int irq, void *dev_id)
    {
    struct stm32_qspi *qspi = (struct stm32_qspi *)dev_id;
    u32 cr, sr;
    cr = readl_relaxed(qspi.io_base + QSPI_CR);
    sr = readl_relaxed(qspi.io_base + QSPI_SR);
    if (sr & SR_SMF) {
// disable irq
    cr &= ~CR_SMIE;
    writel_relaxed(cr, qspi.io_base + QSPI_CR);
    complete(&qspi.match_completion);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn stm32_qspi_read_fifo(val: *mut c_void, addr: *mut void __iomem, len: u8) {
    static void stm32_qspi_read_fifo(void *val, void __iomem *addr, u8 len)
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
unsafe extern "C" fn stm32_qspi_write_fifo(val: *mut c_void, addr: *mut void __iomem, len: u8) {
    static void stm32_qspi_write_fifo(void *val, void __iomem *addr, u8 len)
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
    static int stm32_qspi_tx_poll(struct stm32_qspi *qspi,
    const struct spi_mem_op *op)
    {
    void (*fifo)(void *val, void __iomem *addr, u8 len);
    let mut len: u32 = op.data.nbytes, sr;
    void *buf;
    int ret;
    u8 step;
    if (op.data.dir == SPI_MEM_DATA_IN) {
    fifo = stm32_qspi_read_fifo;
    buf = op.data.buf.in;
    } else {
    fifo = stm32_qspi_write_fifo;
    buf = (void *)op.data.buf.out;
    }
    while (len) {
    ret = readl_relaxed_poll_timeout_atomic(qspi.io_base + QSPI_SR,
    sr, (sr & SR_FTF), 1,
    STM32_FIFO_TIMEOUT_US);
    if (ret) {
    dev_err(qspi.dev, "fifo timeout (len:%d stat:%#x)\n",
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
    fifo(buf, qspi.io_base + QSPI_DR, step);
    len -= step;
    buf += step;
    }
    return 0;
    }
    static int stm32_qspi_tx_mm(struct stm32_qspi *qspi,
    const struct spi_mem_op *op)
    {
    memcpy_fromio(op.data.buf.in, qspi.mm_base + op.addr.val,
    op.data.nbytes);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stm32_qspi_dma_callback(arg: *mut c_void) {
    static void stm32_qspi_dma_callback(void *arg)
    {
    struct completion *dma_completion = arg;
    complete(dma_completion);
    }
    static int stm32_qspi_tx_dma(struct stm32_qspi *qspi,
    const struct spi_mem_op *op)
    {
    struct dma_async_tx_descriptor *desc;
    enum dma_transfer_direction dma_dir;
    struct dma_chan *dma_ch;
    struct sg_table sgt;
    dma_cookie_t cookie;
    u32 cr, t_out;
    int err;
    if (op.data.dir == SPI_MEM_DATA_IN) {
    dma_dir = DMA_DEV_TO_MEM;
    dma_ch = qspi.dma_chrx;
    } else {
    dma_dir = DMA_MEM_TO_DEV;
    dma_ch = qspi.dma_chtx;
    }
//
// spi_map_buf return -EINVAL if the buffer is not DMA-able
// (DMA-able: in vmalloc | kmap | virt_addr_valid)
//
    err = spi_controller_dma_map_mem_op_data(qspi.ctrl, op, &sgt);
    if (err)
    return err;
    desc = dmaengine_prep_slave_sg(dma_ch, sgt.sgl, sgt.nents,
    dma_dir, DMA_PREP_INTERRUPT);
    if (!desc) {
    err = -ENOMEM;
    goto out_unmap;
    }
    cr = readl_relaxed(qspi.io_base + QSPI_CR);
    reinit_completion(&qspi.dma_completion);
    desc.callback = stm32_qspi_dma_callback;
    desc.callback_param = &qspi.dma_completion;
    cookie = dmaengine_submit(desc);
    err = dma_submit_error(cookie);
    if (err)
    goto out;
    dma_async_issue_pending(dma_ch);
    writel_relaxed(cr | CR_DMAEN, qspi.io_base + QSPI_CR);
    t_out = sgt.nents * STM32_COMP_TIMEOUT_MS;
    if (!wait_for_completion_timeout(&qspi.dma_completion,
    msecs_to_jiffies(t_out)))
    err = -ETIMEDOUT;
    if (err)
    dmaengine_terminate_all(dma_ch);
    out:
    writel_relaxed(cr & ~CR_DMAEN, qspi.io_base + QSPI_CR);
    out_unmap:
    spi_controller_dma_unmap_mem_op_data(qspi.ctrl, op, &sgt);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn stm32_qspi_tx(qspi: *mut stm32_qspi, op: *const spi_mem_op) -> c_int {
    static int stm32_qspi_tx(struct stm32_qspi *qspi, const struct spi_mem_op *op)
    {
    if (!op.data.nbytes)
    return 0;
    if (qspi.fmode == CCR_FMODE_MM)
    return stm32_qspi_tx_mm(qspi, op);
    else if (((op.data.dir == SPI_MEM_DATA_IN && qspi.dma_chrx) ||
    (op.data.dir == SPI_MEM_DATA_OUT && qspi.dma_chtx)) &&
    op.data.nbytes > 4)
    if (!stm32_qspi_tx_dma(qspi, op))
    return 0;
    return stm32_qspi_tx_poll(qspi, op);
    }
#[no_mangle]
unsafe extern "C" fn stm32_qspi_wait_nobusy(qspi: *mut stm32_qspi) -> c_int {
    static int stm32_qspi_wait_nobusy(struct stm32_qspi *qspi)
    {
    u32 sr;
    return readl_relaxed_poll_timeout_atomic(qspi.io_base + QSPI_SR, sr,
    !(sr & SR_BUSY), 1,
    STM32_BUSY_TIMEOUT_US);
    }
#[no_mangle]
unsafe extern "C" fn stm32_qspi_wait_cmd(qspi: *mut stm32_qspi) -> c_int {
    static int stm32_qspi_wait_cmd(struct stm32_qspi *qspi)
    {
    u32 sr;
    let mut err: c_int = 0;
    if (qspi.fmode == CCR_FMODE_APM)
    goto out;
    err = readl_relaxed_poll_timeout_atomic(qspi.io_base + QSPI_SR, sr,
    (sr & (SR_TEF | SR_TCF)), 1,
    STM32_WAIT_CMD_TIMEOUT_US);
    if (sr & SR_TEF)
    err = -EIO;
    out:
// clear flags
    writel_relaxed(FCR_CTCF | FCR_CTEF, qspi.io_base + QSPI_FCR);
    if (!err)
    err = stm32_qspi_wait_nobusy(qspi);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn stm32_qspi_wait_poll_status(qspi: *mut stm32_qspi) -> c_int {
    static int stm32_qspi_wait_poll_status(struct stm32_qspi *qspi)
    {
    u32 cr;
    reinit_completion(&qspi.match_completion);
    cr = readl_relaxed(qspi.io_base + QSPI_CR);
    writel_relaxed(cr | CR_SMIE, qspi.io_base + QSPI_CR);
    if (!wait_for_completion_timeout(&qspi.match_completion,
    msecs_to_jiffies(qspi.status_timeout)))
    return -ETIMEDOUT;
    writel_relaxed(FCR_CSMF, qspi.io_base + QSPI_FCR);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stm32_qspi_get_mode(buswidth: u8) -> c_int {
    static int stm32_qspi_get_mode(u8 buswidth)
    {
    if (buswidth >= 4)
    return CCR_BUSWIDTH_4;
    return buswidth;
    }
#[no_mangle]
unsafe extern "C" fn stm32_qspi_send(spi: *mut spi_device, op: *const spi_mem_op) -> c_int {
    static int stm32_qspi_send(struct spi_device *spi, const struct spi_mem_op *op)
    {
    struct stm32_qspi *qspi = spi_controller_get_devdata(spi.controller);
    struct stm32_qspi_flash *flash = &qspi.flash[spi_get_chipselect(spi, 0)];
    u32 ccr, cr;
    int timeout, err = 0, err_poll_status = 0;
    cr = readl_relaxed(qspi.io_base + QSPI_CR);
    FIELD_MODIFY(CR_PRESC_MASK, &cr, flash.presc);
    FIELD_MODIFY(CR_FSEL, &cr, flash.cs);
    writel_relaxed(cr, qspi.io_base + QSPI_CR);
    if (op.data.nbytes)
    writel_relaxed(op.data.nbytes - 1,
    qspi.io_base + QSPI_DLR);
    ccr = qspi.fmode;
    ccr |= FIELD_PREP(CCR_INST_MASK, op.cmd.opcode);
    ccr |= FIELD_PREP(CCR_IMODE_MASK,
    stm32_qspi_get_mode(op.cmd.buswidth));
    if (op.addr.nbytes) {
    ccr |= FIELD_PREP(CCR_ADMODE_MASK,
    stm32_qspi_get_mode(op.addr.buswidth));
    ccr |= FIELD_PREP(CCR_ADSIZE_MASK, op.addr.nbytes - 1);
    }
    if (op.dummy.nbytes)
    ccr |= FIELD_PREP(CCR_DCYC_MASK,
    op.dummy.nbytes * 8 / op.dummy.buswidth);
    if (op.data.nbytes) {
    ccr |= FIELD_PREP(CCR_DMODE_MASK,
    stm32_qspi_get_mode(op.data.buswidth));
    }
    writel_relaxed(ccr, qspi.io_base + QSPI_CCR);
    if (op.addr.nbytes && qspi.fmode != CCR_FMODE_MM)
    writel_relaxed(op.addr.val, qspi.io_base + QSPI_AR);
    if (qspi.fmode == CCR_FMODE_APM)
    err_poll_status = stm32_qspi_wait_poll_status(qspi);
    err = stm32_qspi_tx(qspi, op);
//
// Abort in:
// -error case
// -read memory map: prefetching must be stopped if we read the last
// byte of device (device size - fifo size). like device size is not
// knows, the prefetching is always stop.
//
    if (err || err_poll_status || qspi.fmode == CCR_FMODE_MM)
    goto abort;
// wait end of tx in indirect mode
    err = stm32_qspi_wait_cmd(qspi);
    if (err)
    goto abort;
    return 0;
    abort:
    cr = readl_relaxed(qspi.io_base + QSPI_CR) | CR_ABORT;
    writel_relaxed(cr, qspi.io_base + QSPI_CR);
// wait clear of abort bit by hw
    timeout = readl_relaxed_poll_timeout_atomic(qspi.io_base + QSPI_CR,
    cr, !(cr & CR_ABORT), 1,
    STM32_ABT_TIMEOUT_US);
    writel_relaxed(FCR_CTCF | FCR_CSMF, qspi.io_base + QSPI_FCR);
    if (err || err_poll_status || timeout)
    dev_err(qspi.dev, "%s err:%d err_poll_status:%d abort timeout:%d\n",
    __func__, err, err_poll_status, timeout);
    return err;
    }
    static int stm32_qspi_poll_status(struct spi_mem *mem, const struct spi_mem_op *op,
    u16 mask, u16 match,
    unsigned long initial_delay_us,
    unsigned long polling_rate_us,
    unsigned long timeout_ms)
    {
    struct stm32_qspi *qspi = spi_controller_get_devdata(mem.spi.controller);
    int ret;
    if (!spi_mem_supports_op(mem, op))
    return -EOPNOTSUPP;
    ret = pm_runtime_resume_and_get(qspi.dev);
    if (ret < 0)
    return ret;
    mutex_lock(&qspi.lock);
    writel_relaxed(mask, qspi.io_base + QSPI_PSMKR);
    writel_relaxed(match, qspi.io_base + QSPI_PSMAR);
    qspi.fmode = CCR_FMODE_APM;
    qspi.status_timeout = timeout_ms;
    ret = stm32_qspi_send(mem.spi, op);
    mutex_unlock(&qspi.lock);
    pm_runtime_put_autosuspend(qspi.dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn stm32_qspi_exec_op(mem: *mut spi_mem, op: *const spi_mem_op) -> c_int {
    static int stm32_qspi_exec_op(struct spi_mem *mem, const struct spi_mem_op *op)
    {
    struct stm32_qspi *qspi = spi_controller_get_devdata(mem.spi.controller);
    int ret;
    ret = pm_runtime_resume_and_get(qspi.dev);
    if (ret < 0)
    return ret;
    mutex_lock(&qspi.lock);
    if (op.data.dir == SPI_MEM_DATA_IN && op.data.nbytes)
    qspi.fmode = CCR_FMODE_INDR;
    else
    qspi.fmode = CCR_FMODE_INDW;
    ret = stm32_qspi_send(mem.spi, op);
    mutex_unlock(&qspi.lock);
    pm_runtime_put_autosuspend(qspi.dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn stm32_qspi_dirmap_create(desc: *mut spi_mem_dirmap_desc) -> c_int {
    static int stm32_qspi_dirmap_create(struct spi_mem_dirmap_desc *desc)
    {
    struct stm32_qspi *qspi = spi_controller_get_devdata(desc.mem.spi.controller);
    if (desc.info.op_tmpl.data.dir == SPI_MEM_DATA_OUT)
    return -EOPNOTSUPP;
// should never happen, as mm_base == null is an error probe exit condition
    if (!qspi.mm_base && desc.info.op_tmpl.data.dir == SPI_MEM_DATA_IN)
    return -EOPNOTSUPP;
    if (!qspi.mm_size)
    return -EOPNOTSUPP;
    return 0;
    }
    static ssize_t stm32_qspi_dirmap_read(struct spi_mem_dirmap_desc *desc,
    u64 offs, size_t len, void *buf)
    {
    struct stm32_qspi *qspi = spi_controller_get_devdata(desc.mem.spi.controller);
    struct spi_mem_op op;
    u32 addr_max;
    int ret;
    ret = pm_runtime_resume_and_get(qspi.dev);
    if (ret < 0)
    return ret;
    mutex_lock(&qspi.lock);
// make a local copy of desc op_tmpl and complete dirmap rdesc
// spi_mem_op template with offs, len and *buf in  order to get
// all needed transfer information into struct spi_mem_op
//
    memcpy(&op, desc.info.op_tmpl, sizeof(struct spi_mem_op));
    dev_dbg(qspi.dev, "%s len = 0x%zx offs = 0x%llx buf = 0x%p\n", __func__, len, offs, buf);
    op.data.nbytes = len;
    op.addr.val = desc.info.offset + offs;
    op.data.buf.in = buf;
    addr_max = op.addr.val + op.data.nbytes + 1;
    if (addr_max < qspi.mm_size && op.addr.buswidth)
    qspi.fmode = CCR_FMODE_MM;
    else
    qspi.fmode = CCR_FMODE_INDR;
    ret = stm32_qspi_send(desc.mem.spi, &op);
    mutex_unlock(&qspi.lock);
    pm_runtime_put_autosuspend(qspi.dev);
    return ret ?: len;
    }
    static int stm32_qspi_transfer_one_message(struct spi_controller *ctrl,
    struct spi_message *msg)
    {
    struct stm32_qspi *qspi = spi_controller_get_devdata(ctrl);
    struct spi_transfer *transfer;
    struct spi_device *spi = msg.spi;
    struct spi_mem_op op;
    let mut ret: c_int = 0;
    if (!spi_get_csgpiod(spi, 0))
    return -EOPNOTSUPP;
    ret = pm_runtime_resume_and_get(qspi.dev);
    if (ret < 0)
    return ret;
    mutex_lock(&qspi.lock);
    gpiod_set_value_cansleep(spi_get_csgpiod(spi, 0), true);
    list_for_each_entry(transfer, &msg.transfers, transfer_list) {
    let mut dummy_bytes: u8 = 0;
    memset(&op, 0, sizeof(op));
    dev_dbg(qspi.dev, "tx_buf:%p tx_nbits:%d rx_buf:%p rx_nbits:%d len:%d dummy_data:%d\n",
    transfer.tx_buf, transfer.tx_nbits,
    transfer.rx_buf, transfer.rx_nbits,
    transfer.len, transfer.dummy_data);
//
// QSPI hardware supports dummy bytes transfer.
// If current transfer is dummy byte, merge it with the next
// transfer in order to take into account QSPI block constraint
//
    if (transfer.dummy_data) {
    op.dummy.buswidth = transfer.tx_nbits;
    op.dummy.nbytes = transfer.len;
    dummy_bytes = transfer.len;
// if happens, means that message is not correctly built
    if (list_is_last(&transfer.transfer_list, &msg.transfers)) {
    ret = -EINVAL;
    goto end_of_transfer;
    }
    transfer = list_next_entry(transfer, transfer_list);
    }
    op.data.nbytes = transfer.len;
    if (transfer.rx_buf) {
    qspi.fmode = CCR_FMODE_INDR;
    op.data.buswidth = transfer.rx_nbits;
    op.data.dir = SPI_MEM_DATA_IN;
    op.data.buf.in = transfer.rx_buf;
    } else {
    qspi.fmode = CCR_FMODE_INDW;
    op.data.buswidth = transfer.tx_nbits;
    op.data.dir = SPI_MEM_DATA_OUT;
    op.data.buf.out = transfer.tx_buf;
    }
    ret = stm32_qspi_send(spi, &op);
    if (ret)
    goto end_of_transfer;
    msg.actual_length += transfer.len + dummy_bytes;
    }
    end_of_transfer:
    gpiod_set_value_cansleep(spi_get_csgpiod(spi, 0), false);
    mutex_unlock(&qspi.lock);
    msg.status = ret;
    spi_finalize_current_message(ctrl);
    pm_runtime_put_autosuspend(qspi.dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn stm32_qspi_setup(spi: *mut spi_device) -> c_int {
    static int stm32_qspi_setup(struct spi_device *spi)
    {
    struct spi_controller *ctrl = spi.controller;
    struct stm32_qspi *qspi = spi_controller_get_devdata(ctrl);
    struct stm32_qspi_flash *flash;
    u32 presc, mode;
    int ret;
    if (ctrl.busy)
    return -EBUSY;
    if (!spi.max_speed_hz)
    return -EINVAL;
    mode = spi.mode & (SPI_TX_OCTAL | SPI_RX_OCTAL);
    if (mode && gpiod_count(qspi.dev, "cs") == -ENOENT) {
    dev_err(qspi.dev, "spi-rx-bus-width\\/spi-tx-bus-width\\/cs-gpios\n");
    dev_err(qspi.dev, "configuration not supported\n");
    return -EINVAL;
    }
    ret = pm_runtime_resume_and_get(qspi.dev);
    if (ret < 0)
    return ret;
    presc = DIV_ROUND_UP(qspi.clk_rate, spi.max_speed_hz) - 1;
    flash = &qspi.flash[spi_get_chipselect(spi, 0)];
    flash.cs = spi_get_chipselect(spi, 0);
    flash.presc = presc;
    mutex_lock(&qspi.lock);
    qspi.cr_reg = CR_APMS | 3 << CR_FTHRES_SHIFT | CR_SSHIFT | CR_EN;
//
// Dual flash mode is only enable in case SPI_TX_OCTAL or SPI_RX_OCTAL
// is set in spi->mode and "cs-gpios" properties is found in DT
//
    if (mode) {
    qspi.cr_reg |= CR_DFM;
    dev_dbg(qspi.dev, "Dual flash mode enable");
    }
    writel_relaxed(qspi.cr_reg, qspi.io_base + QSPI_CR);
// set dcr fsize to max address
    qspi.dcr_reg = DCR_FSIZE_MASK;
    writel_relaxed(qspi.dcr_reg, qspi.io_base + QSPI_DCR);
    mutex_unlock(&qspi.lock);
    pm_runtime_put_autosuspend(qspi.dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stm32_qspi_dma_setup(qspi: *mut stm32_qspi) -> c_int {
    static int stm32_qspi_dma_setup(struct stm32_qspi *qspi)
    {
    struct dma_slave_config dma_cfg;
    struct device *dev = qspi.dev;
    struct dma_slave_caps caps;
    let mut ret: c_int = 0;
    memset(&dma_cfg, 0, sizeof(dma_cfg));
    dma_cfg.src_addr_width = DMA_SLAVE_BUSWIDTH_1_BYTE;
    dma_cfg.dst_addr_width = DMA_SLAVE_BUSWIDTH_1_BYTE;
    dma_cfg.src_addr = qspi.phys_base + QSPI_DR;
    dma_cfg.dst_addr = qspi.phys_base + QSPI_DR;
    qspi.dma_chrx = dma_request_chan(dev, "rx");
    if (IS_ERR(qspi.dma_chrx)) {
    ret = PTR_ERR(qspi.dma_chrx);
    qspi.dma_chrx = core::ptr::null_mut();
    if (ret == -EPROBE_DEFER)
    goto out;
    } else {
    ret = dma_get_slave_caps(qspi.dma_chrx, &caps);
    if (ret)
    return ret;
    dma_cfg.src_maxburst = caps.max_burst / dma_cfg.src_addr_width;
    if (dmaengine_slave_config(qspi.dma_chrx, &dma_cfg)) {
    dev_err(dev, "dma rx config failed\n");
    dma_release_channel(qspi.dma_chrx);
    qspi.dma_chrx = core::ptr::null_mut();
    }
    }
    qspi.dma_chtx = dma_request_chan(dev, "tx");
    if (IS_ERR(qspi.dma_chtx)) {
    ret = PTR_ERR(qspi.dma_chtx);
    qspi.dma_chtx = core::ptr::null_mut();
    } else {
    ret = dma_get_slave_caps(qspi.dma_chtx, &caps);
    if (ret)
    return ret;
    dma_cfg.dst_maxburst = caps.max_burst / dma_cfg.dst_addr_width;
    if (dmaengine_slave_config(qspi.dma_chtx, &dma_cfg)) {
    dev_err(dev, "dma tx config failed\n");
    dma_release_channel(qspi.dma_chtx);
    qspi.dma_chtx = core::ptr::null_mut();
    }
    }
    out:
    init_completion(&qspi.dma_completion);
    if (ret != -EPROBE_DEFER)
    ret = 0;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn stm32_qspi_dma_free(qspi: *mut stm32_qspi) {
    static void stm32_qspi_dma_free(struct stm32_qspi *qspi)
    {
    if (qspi.dma_chtx)
    dma_release_channel(qspi.dma_chtx);
    if (qspi.dma_chrx)
    dma_release_channel(qspi.dma_chrx);
    }
//
// no special host constraint, so use default spi_mem_default_supports_op
// to check supported mode.
//
    static const struct spi_controller_mem_ops stm32_qspi_mem_ops = {
    .exec_op	= stm32_qspi_exec_op,
    .dirmap_create	= stm32_qspi_dirmap_create,
    .dirmap_read	= stm32_qspi_dirmap_read,
    .poll_status	= stm32_qspi_poll_status,
    };
#[no_mangle]
unsafe extern "C" fn stm32_qspi_probe(pdev: *mut platform_device) -> c_int {
    static int stm32_qspi_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct spi_controller *ctrl;
    struct reset_control *rstc;
    struct stm32_qspi *qspi;
    struct resource *res;
    int ret, irq;
    ctrl = devm_spi_alloc_host(dev, sizeof(*qspi));
    if (!ctrl)
    return -ENOMEM;
    qspi = spi_controller_get_devdata(ctrl);
    qspi.ctrl = ctrl;
    res = platform_get_resource_byname(pdev, IORESOURCE_MEM, "qspi");
    qspi.io_base = devm_ioremap_resource(dev, res);
    if (IS_ERR(qspi.io_base))
    return PTR_ERR(qspi.io_base);
    qspi.phys_base = res.start;
    res = platform_get_resource_byname(pdev, IORESOURCE_MEM, "qspi_mm");
    qspi.mm_base = devm_ioremap_resource(dev, res);
    if (IS_ERR(qspi.mm_base))
    return PTR_ERR(qspi.mm_base);
    qspi.mm_size = resource_size(res);
    if (qspi.mm_size > STM32_QSPI_MAX_MMAP_SZ)
    return -EINVAL;
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    ret = devm_request_irq(dev, irq, stm32_qspi_irq, 0,
    dev_name(dev), qspi);
    if (ret) {
    dev_err(dev, "failed to request irq\n");
    return ret;
    }
    init_completion(&qspi.match_completion);
    qspi.clk = devm_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(qspi.clk))
    return PTR_ERR(qspi.clk);
    qspi.clk_rate = clk_get_rate(qspi.clk);
    if (!qspi.clk_rate)
    return -EINVAL;
    ret = clk_prepare_enable(qspi.clk);
    if (ret) {
    dev_err(dev, "can not enable the clock\n");
    return ret;
    }
    rstc = devm_reset_control_get_exclusive(dev, core::ptr::null_mut());
    if (IS_ERR(rstc)) {
    ret = PTR_ERR(rstc);
    if (ret == -EPROBE_DEFER)
    goto err_clk_disable;
    } else {
    reset_control_assert(rstc);
    udelay(2);
    reset_control_deassert(rstc);
    }
    qspi.dev = dev;
    platform_set_drvdata(pdev, qspi);
    ret = stm32_qspi_dma_setup(qspi);
    if (ret)
    goto err_dma_free;
    mutex_init(&qspi.lock);
    ctrl.mode_bits = SPI_RX_DUAL | SPI_RX_QUAD | SPI_TX_OCTAL
    | SPI_TX_DUAL | SPI_TX_QUAD | SPI_RX_OCTAL;
    ctrl.setup = stm32_qspi_setup;
    ctrl.bus_num = -1;
    ctrl.mem_ops = &stm32_qspi_mem_ops;
    ctrl.use_gpio_descriptors = true;
    ctrl.transfer_one_message = stm32_qspi_transfer_one_message;
    ctrl.num_chipselect = STM32_QSPI_MAX_NORCHIP;
    pm_runtime_set_autosuspend_delay(dev, STM32_AUTOSUSPEND_DELAY);
    pm_runtime_use_autosuspend(dev);
    pm_runtime_set_active(dev);
    pm_runtime_enable(dev);
    pm_runtime_get_noresume(dev);
    ret = spi_register_controller(ctrl);
    if (ret)
    goto err_pm_runtime_free;
    pm_runtime_put_autosuspend(dev);
    return 0;
    err_pm_runtime_free:
    pm_runtime_get_sync(qspi.dev);
// disable qspi
    writel_relaxed(0, qspi.io_base + QSPI_CR);
    mutex_destroy(&qspi.lock);
    pm_runtime_put_noidle(qspi.dev);
    pm_runtime_disable(qspi.dev);
    pm_runtime_set_suspended(qspi.dev);
    pm_runtime_dont_use_autosuspend(qspi.dev);
    err_dma_free:
    stm32_qspi_dma_free(qspi);
    err_clk_disable:
    clk_disable_unprepare(qspi.clk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn stm32_qspi_remove(pdev: *mut platform_device) {
    static void stm32_qspi_remove(struct platform_device *pdev)
    {
    struct stm32_qspi *qspi = platform_get_drvdata(pdev);
    pm_runtime_get_sync(qspi.dev);
    spi_unregister_controller(qspi.ctrl);
// disable qspi
    writel_relaxed(0, qspi.io_base + QSPI_CR);
    stm32_qspi_dma_free(qspi);
    mutex_destroy(&qspi.lock);
    pm_runtime_put_noidle(qspi.dev);
    pm_runtime_disable(qspi.dev);
    pm_runtime_set_suspended(qspi.dev);
    pm_runtime_dont_use_autosuspend(qspi.dev);
    clk_disable_unprepare(qspi.clk);
    }
#[no_mangle]
unsafe extern "C" fn stm32_qspi_runtime_suspend(dev: *mut device) -> c_int {
    static int stm32_qspi_runtime_suspend(struct device *dev)
    {
    struct stm32_qspi *qspi = dev_get_drvdata(dev);
    clk_disable_unprepare(qspi.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stm32_qspi_runtime_resume(dev: *mut device) -> c_int {
    static int stm32_qspi_runtime_resume(struct device *dev)
    {
    struct stm32_qspi *qspi = dev_get_drvdata(dev);
    return clk_prepare_enable(qspi.clk);
    }
#[no_mangle]
unsafe extern "C" fn stm32_qspi_suspend(dev: *mut device) -> c_int {
    static int stm32_qspi_suspend(struct device *dev)
    {
    pinctrl_pm_select_sleep_state(dev);
    return pm_runtime_force_suspend(dev);
    }
#[no_mangle]
unsafe extern "C" fn stm32_qspi_resume(dev: *mut device) -> c_int {
    static int stm32_qspi_resume(struct device *dev)
    {
    struct stm32_qspi *qspi = dev_get_drvdata(dev);
    int ret;
    ret = pm_runtime_force_resume(dev);
    if (ret < 0)
    return ret;
    pinctrl_pm_select_default_state(dev);
    ret = pm_runtime_resume_and_get(dev);
    if (ret < 0)
    return ret;
    writel_relaxed(qspi.cr_reg, qspi.io_base + QSPI_CR);
    writel_relaxed(qspi.dcr_reg, qspi.io_base + QSPI_DCR);
    pm_runtime_put_autosuspend(dev);
    return 0;
    }
    static const struct dev_pm_ops stm32_qspi_pm_ops = {
    RUNTIME_PM_OPS(stm32_qspi_runtime_suspend, stm32_qspi_runtime_resume, core::ptr::null_mut())
    SYSTEM_SLEEP_PM_OPS(stm32_qspi_suspend, stm32_qspi_resume)
    };
    static const struct of_device_id stm32_qspi_match[] = {
    {.compatible = "st,stm32f469-qspi"},
    {}
    };
    MODULE_DEVICE_TABLE(of, stm32_qspi_match);
    static struct platform_driver stm32_qspi_driver = {
    .probe	= stm32_qspi_probe,
    .remove = stm32_qspi_remove,
    .driver	= {
    .name = "stm32-qspi",
    .of_match_table = stm32_qspi_match,
    .pm = pm_ptr(&stm32_qspi_pm_ops),
    },
    };
    module_platform_driver(stm32_qspi_driver);
    MODULE_AUTHOR("Ludovic Barre <ludovic.barre@st.com>");
    MODULE_DESCRIPTION("STMicroelectronics STM32 quad spi driver");
    MODULE_LICENSE("GPL v2");
