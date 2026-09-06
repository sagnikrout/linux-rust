//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-fsi.c
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
// Copyright (C) IBM Corporation 2020

pub const FSI_ENGID_SPI: c_uint = 0x23;
pub const FSI_MBOX_ROOT_CTRL_8: c_uint = 0x2860;
pub const FSI_MBOX_ROOT_CTRL_8_SPI_MUX: c_uint = 0xf0000000;
pub const FSI2SPI_DATA0: c_uint = 0x00;
pub const FSI2SPI_DATA1: c_uint = 0x04;
pub const FSI2SPI_CMD: c_uint = 0x08;

pub const FSI2SPI_RESET: c_uint = 0x18;
pub const FSI2SPI_STATUS: c_uint = 0x1c;

pub const FSI2SPI_IRQ: c_uint = 0x20;
pub const SPI_FSI_BASE: c_uint = 0x70000;
pub const SPI_FSI_TIMEOUT_MS: c_int = 1000;
pub const SPI_FSI_MAX_RX_SIZE: c_int = 8;
pub const SPI_FSI_MAX_TX_SIZE: c_int = 40;
pub const SPI_FSI_ERROR: c_uint = 0x0;
pub const SPI_FSI_COUNTER_CFG: c_uint = 0x1;
pub const SPI_FSI_CFG1: c_uint = 0x2;
pub const SPI_FSI_CLOCK_CFG: c_uint = 0x3;

pub const SPI_FSI_MMAP: c_uint = 0x4;
pub const SPI_FSI_DATA_TX: c_uint = 0x5;
pub const SPI_FSI_DATA_RX: c_uint = 0x6;
pub const SPI_FSI_SEQUENCE: c_uint = 0x7;
pub const SPI_FSI_SEQUENCE_STOP: c_uint = 0x00;

pub const SPI_FSI_SEQUENCE_COPY_DATA_TX: c_uint = 0xc0;

pub const SPI_FSI_STATUS: c_uint = 0x8;

    (GENMASK_ULL(31, 21) | GENMASK_ULL(15, 12))

    (SPI_FSI_STATUS_ERROR | \
    SPI_FSI_STATUS_TDR_OVERRUN | SPI_FSI_STATUS_RDR_UNDERRUN | \
    SPI_FSI_STATUS_RDR_OVERRUN)
pub const SPI_FSI_PORT_CTRL: c_uint = 0x9;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsi2spi {
    pub /: *mut *mut *mut fsi_device fsi; / FSI2SPI CFAM engine device,
    pub /: *mut *mut mutex lock; / lock access to the device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsi_spi {
    pub /: *mut *mut *mut device dev; / SPI controller device,
    pub /: *mut *mut *mut fsi2spi bridge; / FSI2SPI device,
    pub base: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsi_spi_sequence {
    pub bit: c_int,
    pub data: u64,
}

#[no_mangle]
unsafe extern "C" fn fsi_spi_check_mux(fsi: *mut fsi_device, dev: *mut device) -> c_int {
    static int fsi_spi_check_mux(struct fsi_device *fsi, struct device *dev)
    {
    int rc;
    u32 root_ctrl_8;
    __be32 root_ctrl_8_be;
    rc = fsi_slave_read(fsi.slave, FSI_MBOX_ROOT_CTRL_8, &root_ctrl_8_be,
    sizeof(root_ctrl_8_be));
    if (rc)
    return rc;
    root_ctrl_8 = be32_to_cpu(root_ctrl_8_be);
    dev_dbg(dev, "Root control register 8: %08x\n", root_ctrl_8);
    if ((root_ctrl_8 & FSI_MBOX_ROOT_CTRL_8_SPI_MUX) ==
    FSI_MBOX_ROOT_CTRL_8_SPI_MUX)
    return 0;
    return -ENOLINK;
    }
#[no_mangle]
unsafe extern "C" fn fsi_spi_check_status(ctx: *mut fsi_spi) -> c_int {
    static int fsi_spi_check_status(struct fsi_spi *ctx)
    {
    int rc;
    u32 sts;
    __be32 sts_be;
    rc = fsi_device_read(ctx.bridge.fsi, FSI2SPI_STATUS, &sts_be,
    sizeof(sts_be));
    if (rc)
    return rc;
    sts = be32_to_cpu(sts_be);
    if (sts & FSI2SPI_STATUS_ANY_ERROR) {
    dev_err(ctx.dev, "Error with FSI2SPI interface: %08x.\n", sts);
    return -EIO;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fsi_spi_read_reg(ctx: *mut fsi_spi, offset: u32, value: *mut u64) -> c_int {
    static int fsi_spi_read_reg(struct fsi_spi *ctx, u32 offset, u64 *value)
    {
    let mut rc: c_int = 0;
    __be32 cmd_be;
    __be32 data_be;
    let mut cmd: u32 = offset + ctx.base;
    struct fsi2spi *bridge = ctx.bridge;
// value = 0ULL;
    if (cmd & FSI2SPI_CMD_WRITE)
    return -EINVAL;
    rc = mutex_lock_interruptible(&bridge.lock);
    if (rc)
    return rc;
    cmd_be = cpu_to_be32(cmd);
    rc = fsi_device_write(bridge.fsi, FSI2SPI_CMD, &cmd_be,
    sizeof(cmd_be));
    if (rc)
    goto unlock;
    rc = fsi_spi_check_status(ctx);
    if (rc)
    goto unlock;
    rc = fsi_device_read(bridge.fsi, FSI2SPI_DATA0, &data_be,
    sizeof(data_be));
    if (rc)
    goto unlock;
// value |= (u64)be32_to_cpu(data_be) << 32;
    rc = fsi_device_read(bridge.fsi, FSI2SPI_DATA1, &data_be,
    sizeof(data_be));
    if (rc)
    goto unlock;
// value |= (u64)be32_to_cpu(data_be);
    dev_dbg(ctx.dev, "Read %02x[%016llx].\n", offset, *value);
    unlock:
    mutex_unlock(&bridge.lock);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn fsi_spi_write_reg(ctx: *mut fsi_spi, offset: u32, value: u64) -> c_int {
    static int fsi_spi_write_reg(struct fsi_spi *ctx, u32 offset, u64 value)
    {
    let mut rc: c_int = 0;
    __be32 cmd_be;
    __be32 data_be;
    let mut cmd: u32 = offset + ctx.base;
    struct fsi2spi *bridge = ctx.bridge;
    if (cmd & FSI2SPI_CMD_WRITE)
    return -EINVAL;
    rc = mutex_lock_interruptible(&bridge.lock);
    if (rc)
    return rc;
    dev_dbg(ctx.dev, "Write %02x[%016llx].\n", offset, value);
    data_be = cpu_to_be32(upper_32_bits(value));
    rc = fsi_device_write(bridge.fsi, FSI2SPI_DATA0, &data_be,
    sizeof(data_be));
    if (rc)
    goto unlock;
    data_be = cpu_to_be32(lower_32_bits(value));
    rc = fsi_device_write(bridge.fsi, FSI2SPI_DATA1, &data_be,
    sizeof(data_be));
    if (rc)
    goto unlock;
    cmd_be = cpu_to_be32(cmd | FSI2SPI_CMD_WRITE);
    rc = fsi_device_write(bridge.fsi, FSI2SPI_CMD, &cmd_be,
    sizeof(cmd_be));
    if (rc)
    goto unlock;
    rc = fsi_spi_check_status(ctx);
    unlock:
    mutex_unlock(&bridge.lock);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn fsi_spi_data_in(in: u64, rx: *mut u8, len: c_int) -> c_int {
    static int fsi_spi_data_in(u64 in, u8 *rx, int len)
    {
    int i;
    let mut num_bytes: c_int = min(len, 8);
    for (i = 0; i < num_bytes; ++i)
    rx[i] = (u8)(in >> (8 * ((num_bytes - 1) - i)));
    return num_bytes;
    }
#[no_mangle]
unsafe extern "C" fn fsi_spi_data_out(out: *mut u64, tx: *const u8, len: c_int) -> c_int {
    static int fsi_spi_data_out(u64 *out, const u8 *tx, int len)
    {
    int i;
    let mut num_bytes: c_int = min(len, 8);
    u8 *out_bytes = (u8 *)out;
// Unused bytes of the tx data should be 0.
// out = 0ULL;
    for (i = 0; i < num_bytes; ++i)
    out_bytes[8 - (i + 1)] = tx[i];
    return num_bytes;
    }
#[no_mangle]
unsafe extern "C" fn fsi_spi_reset(ctx: *mut fsi_spi) -> c_int {
    static int fsi_spi_reset(struct fsi_spi *ctx)
    {
    int rc;
    dev_dbg(ctx.dev, "Resetting SPI controller.\n");
    rc = fsi_spi_write_reg(ctx, SPI_FSI_CLOCK_CFG,
    SPI_FSI_CLOCK_CFG_RESET1);
    if (rc)
    return rc;
    rc = fsi_spi_write_reg(ctx, SPI_FSI_CLOCK_CFG,
    SPI_FSI_CLOCK_CFG_RESET2);
    if (rc)
    return rc;
    return fsi_spi_write_reg(ctx, SPI_FSI_STATUS, 0ULL);
    }
#[no_mangle]
unsafe extern "C" fn fsi_spi_status(ctx: *mut fsi_spi, status: *mut u64, dir: *const c_char) -> c_int {
    static int fsi_spi_status(struct fsi_spi *ctx, u64 *status, const char *dir)
    {
    let mut rc: c_int = fsi_spi_read_reg(ctx, SPI_FSI_STATUS, status);
    if (rc)
    return rc;
    if (*status & SPI_FSI_STATUS_ANY_ERROR) {
    dev_err(ctx.dev, "%s error: %016llx\n", dir, *status);
    rc = fsi_spi_reset(ctx);
    if (rc)
    return rc;
    return -EREMOTEIO;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fsi_spi_sequence_add(seq: *mut fsi_spi_sequence, val: u8) {
    static void fsi_spi_sequence_add(struct fsi_spi_sequence *seq, u8 val)
    {
//
// Add the next byte of instruction to the 8-byte sequence register.
// Then decrement the counter so that the next instruction will go in
// the right place. Return the index of the slot we just filled in the
// sequence register.
//
    seq.data |= (u64)val << seq.bit;
    seq.bit -= 8;
    }
#[no_mangle]
unsafe extern "C" fn fsi_spi_sequence_init(seq: *mut fsi_spi_sequence) {
    static void fsi_spi_sequence_init(struct fsi_spi_sequence *seq)
    {
    seq.bit = 56;
    seq.data = 0ULL;
    }
    static int fsi_spi_transfer_data(struct fsi_spi *ctx,
    struct spi_transfer *transfer)
    {
    int loops;
    let mut rc: c_int = 0;
    unsigned long end;
    let mut status: u64 = 0ULL;
    if (transfer.tx_buf) {
    int nb;
    let mut sent: c_int = 0;
    let mut out: u64 = 0ULL;
    const u8 *tx = transfer.tx_buf;
    while (transfer.len > sent) {
    nb = fsi_spi_data_out(&out, &tx[sent],
    (int)transfer.len - sent);
    rc = fsi_spi_write_reg(ctx, SPI_FSI_DATA_TX, out);
    if (rc)
    return rc;
    loops = 0;
    end = jiffies + msecs_to_jiffies(SPI_FSI_TIMEOUT_MS);
    do {
    if (loops++ && time_after(jiffies, end))
    return -ETIMEDOUT;
    rc = fsi_spi_status(ctx, &status, "TX");
    if (rc)
    return rc;
    } while (status & SPI_FSI_STATUS_TDR_FULL);
    sent += nb;
    }
    } else if (transfer.rx_buf) {
    let mut recv: c_int = 0;
    let mut in: u64 = 0ULL;
    u8 *rx = transfer.rx_buf;
    while (transfer.len > recv) {
    loops = 0;
    end = jiffies + msecs_to_jiffies(SPI_FSI_TIMEOUT_MS);
    do {
    if (loops++ && time_after(jiffies, end))
    return -ETIMEDOUT;
    rc = fsi_spi_status(ctx, &status, "RX");
    if (rc)
    return rc;
    } while (!(status & SPI_FSI_STATUS_RDR_FULL));
    rc = fsi_spi_read_reg(ctx, SPI_FSI_DATA_RX, &in);
    if (rc)
    return rc;
    recv += fsi_spi_data_in(in, &rx[recv],
    (int)transfer.len - recv);
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fsi_spi_transfer_init(ctx: *mut fsi_spi) -> c_int {
    static int fsi_spi_transfer_init(struct fsi_spi *ctx)
    {
    let mut loops: c_int = 0;
    int rc;
    let mut reset: bool = false;
    unsigned long end;
    u64 seq_state;
    let mut clock_cfg: u64 = 0ULL;
    let mut status: u64 = 0ULL;
    u64 wanted_clock_cfg = SPI_FSI_CLOCK_CFG_ECC_DISABLE |
    SPI_FSI_CLOCK_CFG_SCK_NO_DEL |
    FIELD_PREP(SPI_FSI_CLOCK_CFG_SCK_DIV, 19);
    end = jiffies + msecs_to_jiffies(SPI_FSI_TIMEOUT_MS);
    do {
    if (loops++ && time_after(jiffies, end))
    return -ETIMEDOUT;
    rc = fsi_spi_read_reg(ctx, SPI_FSI_STATUS, &status);
    if (rc)
    return rc;
    seq_state = status & SPI_FSI_STATUS_SEQ_STATE;
    if (status & (SPI_FSI_STATUS_ANY_ERROR |
    SPI_FSI_STATUS_TDR_FULL |
    SPI_FSI_STATUS_RDR_FULL)) {
    if (reset) {
    dev_err(ctx.dev,
    "Initialization error: %08llx\n",
    status);
    return -EIO;
    }
    rc = fsi_spi_reset(ctx);
    if (rc)
    return rc;
    reset = true;
    continue;
    }
    } while (seq_state && (seq_state != SPI_FSI_STATUS_SEQ_STATE_IDLE));
    rc = fsi_spi_write_reg(ctx, SPI_FSI_COUNTER_CFG, 0ULL);
    if (rc)
    return rc;
    rc = fsi_spi_read_reg(ctx, SPI_FSI_CLOCK_CFG, &clock_cfg);
    if (rc)
    return rc;
    if ((clock_cfg & (SPI_FSI_CLOCK_CFG_MM_ENABLE |
    SPI_FSI_CLOCK_CFG_ECC_DISABLE |
    SPI_FSI_CLOCK_CFG_MODE |
    SPI_FSI_CLOCK_CFG_SCK_RECV_DEL |
    SPI_FSI_CLOCK_CFG_SCK_DIV)) != wanted_clock_cfg)
    rc = fsi_spi_write_reg(ctx, SPI_FSI_CLOCK_CFG,
    wanted_clock_cfg);
    return rc;
    }
    static int fsi_spi_transfer_one_message(struct spi_controller *ctlr,
    struct spi_message *mesg)
    {
    int rc;
    let mut seq_slave: u8 = SPI_FSI_SEQUENCE_SEL_SLAVE(spi_get_chipselect(mesg.spi, 0) + 1);
    unsigned int len;
    struct spi_transfer *transfer;
    struct fsi_spi *ctx = spi_controller_get_devdata(ctlr);
    rc = fsi_spi_check_mux(ctx.bridge.fsi, ctx.dev);
    if (rc)
    goto error;
    list_for_each_entry(transfer, &mesg.transfers, transfer_list) {
    struct fsi_spi_sequence seq;
    struct spi_transfer *next = core::ptr::null_mut();
// Sequencer must do shift out (tx) first.
    if (!transfer.tx_buf || transfer.len > SPI_FSI_MAX_TX_SIZE) {
    rc = -EINVAL;
    goto error;
    }
    dev_dbg(ctx.dev, "Start tx of %d bytes.\n", transfer.len);
    rc = fsi_spi_transfer_init(ctx);
    if (rc < 0)
    goto error;
    fsi_spi_sequence_init(&seq);
    fsi_spi_sequence_add(&seq, seq_slave);
    len = transfer.len;
    while (len > 8) {
    fsi_spi_sequence_add(&seq,
    SPI_FSI_SEQUENCE_SHIFT_OUT(8));
    len -= 8;
    }
    fsi_spi_sequence_add(&seq, SPI_FSI_SEQUENCE_SHIFT_OUT(len));
    if (!list_is_last(&transfer.transfer_list,
    &mesg.transfers)) {
    next = list_next_entry(transfer, transfer_list);
// Sequencer can only do shift in (rx) after tx.
    if (next.rx_buf) {
    u8 shift;
    if (next.len > SPI_FSI_MAX_RX_SIZE) {
    rc = -EINVAL;
    goto error;
    }
    dev_dbg(ctx.dev, "Sequence rx of %d bytes.\n",
    next.len);
    shift = SPI_FSI_SEQUENCE_SHIFT_IN(next.len);
    fsi_spi_sequence_add(&seq, shift);
    } else if (next.tx_buf) {
    if ((next.len + transfer.len) > (SPI_FSI_MAX_TX_SIZE + 8)) {
    rc = -EINVAL;
    goto error;
    }
    len = next.len;
    while (len > 8) {
    fsi_spi_sequence_add(&seq,
    SPI_FSI_SEQUENCE_SHIFT_OUT(8));
    len -= 8;
    }
    fsi_spi_sequence_add(&seq, SPI_FSI_SEQUENCE_SHIFT_OUT(len));
    } else {
    next = core::ptr::null_mut();
    }
    }
    fsi_spi_sequence_add(&seq, SPI_FSI_SEQUENCE_SEL_SLAVE(0));
    rc = fsi_spi_write_reg(ctx, SPI_FSI_SEQUENCE, seq.data);
    if (rc)
    goto error;
    rc = fsi_spi_transfer_data(ctx, transfer);
    if (rc)
    goto error;
    if (next) {
    rc = fsi_spi_transfer_data(ctx, next);
    if (rc)
    goto error;
    transfer = next;
    }
    }
    error:
    mesg.status = rc;
    spi_finalize_current_message(ctlr);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn fsi_spi_max_transfer_size(spi: *mut spi_device) -> usize {
    static size_t fsi_spi_max_transfer_size(struct spi_device *spi)
    {
    return SPI_FSI_MAX_RX_SIZE;
    }
#[no_mangle]
unsafe extern "C" fn fsi_spi_probe(fsi: *mut fsi_device) -> c_int {
    static int fsi_spi_probe(struct fsi_device *fsi)
    {
    int rc;
    let mut num_controllers_registered: c_int = 0;
    struct fsi2spi *bridge;
    struct device *dev = &fsi.dev;
    rc = fsi_spi_check_mux(fsi, dev);
    if (rc)
    return -ENODEV;
    bridge = devm_kzalloc(dev, sizeof(*bridge), GFP_KERNEL);
    if (!bridge)
    return -ENOMEM;
    bridge.fsi = fsi;
    mutex_init(&bridge.lock);
    for_each_available_child_of_node_scoped(dev.of_node, np) {
    u32 base;
    struct fsi_spi *ctx;
    struct spi_controller *ctlr;
    if (of_property_read_u32(np, "reg", &base))
    continue;
    ctlr = devm_spi_alloc_host(dev, sizeof(*ctx));
    if (!ctlr)
    break;
    ctlr.dev.of_node = np;
    ctlr.num_chipselect = of_get_available_child_count(np) ?: 1;
    ctlr.flags = SPI_CONTROLLER_HALF_DUPLEX;
    ctlr.max_transfer_size = fsi_spi_max_transfer_size;
    ctlr.transfer_one_message = fsi_spi_transfer_one_message;
    ctx = spi_controller_get_devdata(ctlr);
    ctx.dev = &ctlr.dev;
    ctx.bridge = bridge;
    ctx.base = base + SPI_FSI_BASE;
    rc = devm_spi_register_controller(dev, ctlr);
    if (rc)
    continue;
    num_controllers_registered++;
    }
    if (!num_controllers_registered)
    return -ENODEV;
    return 0;
    }
    static const struct fsi_device_id fsi_spi_ids[] = {
    { FSI_ENGID_SPI, FSI_VERSION_ANY },
    { }
    };
    MODULE_DEVICE_TABLE(fsi, fsi_spi_ids);
    static struct fsi_driver fsi_spi_driver = {
    .id_table = fsi_spi_ids,
    .probe = fsi_spi_probe,
    .drv = {
    .name = "spi-fsi",
    },
    };
    module_fsi_driver(fsi_spi_driver);
    MODULE_AUTHOR("Eddie James <eajames@linux.ibm.com>");
    MODULE_DESCRIPTION("FSI attached SPI controller");
    MODULE_LICENSE("GPL");
