//! Automatically rewritten from C to Rust
//! Source: drivers/i2c/busses/i2c-xgene-slimpro.c
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
// X-Gene SLIMpro I2C Driver
//
// Copyright (c) 2014, Applied Micro Circuits Corporation
// Author: Feng Kan <fkan@apm.com>
// Author: Hieu Le <hnle@apm.com>
//
// This driver provides support for X-Gene SLIMpro I2C device access
// using the APM X-Gene SLIMpro mailbox driver.
//

pub const MAILBOX_I2C_INDEX: c_int = 0;

pub const SMBUS_CMD_LEN: c_int = 1;
pub const BYTE_DATA: c_int = 1;
pub const WORD_DATA: c_int = 2;
pub const BLOCK_DATA: c_int = 3;
pub const SLIMPRO_IIC_I2C_PROTOCOL: c_int = 0;
pub const SLIMPRO_IIC_SMB_PROTOCOL: c_int = 1;
pub const SLIMPRO_IIC_READ: c_int = 0;
pub const SLIMPRO_IIC_WRITE: c_int = 1;
pub const IIC_SMB_WITHOUT_DATA_LEN: c_int = 0;
pub const IIC_SMB_WITH_DATA_LEN: c_int = 1;
pub const SLIMPRO_DEBUG_MSG: c_int = 0;
pub const SLIMPRO_MSG_TYPE_SHIFT: c_int = 28;
pub const SLIMPRO_DBG_SUBTYPE_I2C1READ: c_int = 4;
pub const SLIMPRO_DBGMSG_TYPE_SHIFT: c_int = 24;
pub const SLIMPRO_DBGMSG_TYPE_MASK: c_uint = 0x0F000000U;
pub const SLIMPRO_IIC_DEV_SHIFT: c_int = 23;
pub const SLIMPRO_IIC_DEV_MASK: c_uint = 0x00800000U;
pub const SLIMPRO_IIC_DEVID_SHIFT: c_int = 13;
pub const SLIMPRO_IIC_DEVID_MASK: c_uint = 0x007FE000U;
pub const SLIMPRO_IIC_RW_SHIFT: c_int = 12;
pub const SLIMPRO_IIC_RW_MASK: c_uint = 0x00001000U;
pub const SLIMPRO_IIC_PROTO_SHIFT: c_int = 11;
pub const SLIMPRO_IIC_PROTO_MASK: c_uint = 0x00000800U;
pub const SLIMPRO_IIC_ADDRLEN_SHIFT: c_int = 8;
pub const SLIMPRO_IIC_ADDRLEN_MASK: c_uint = 0x00000700U;
pub const SLIMPRO_IIC_DATALEN_SHIFT: c_int = 0;
pub const SLIMPRO_IIC_DATALEN_MASK: c_uint = 0x000000FFU;
//
// SLIMpro I2C message encode
//
// dev		- Controller number (0-based)
// chip		- I2C chip address
// op		- SLIMPRO_IIC_READ or SLIMPRO_IIC_WRITE
// proto	- SLIMPRO_IIC_SMB_PROTOCOL or SLIMPRO_IIC_I2C_PROTOCOL
// addrlen	- Length of the address field
// datalen	- Length of the data field
//

    ((SLIMPRO_DEBUG_MSG << SLIMPRO_MSG_TYPE_SHIFT) | \
    ((SLIMPRO_DBG_SUBTYPE_I2C1READ << SLIMPRO_DBGMSG_TYPE_SHIFT) & \
    SLIMPRO_DBGMSG_TYPE_MASK) | \
    ((dev << SLIMPRO_IIC_DEV_SHIFT) & SLIMPRO_IIC_DEV_MASK) | \
    ((chip << SLIMPRO_IIC_DEVID_SHIFT) & SLIMPRO_IIC_DEVID_MASK) | \
    ((op << SLIMPRO_IIC_RW_SHIFT) & SLIMPRO_IIC_RW_MASK) | \
    ((proto << SLIMPRO_IIC_PROTO_SHIFT) & SLIMPRO_IIC_PROTO_MASK) | \
    ((addrlen << SLIMPRO_IIC_ADDRLEN_SHIFT) & SLIMPRO_IIC_ADDRLEN_MASK) | \
    ((datalen << SLIMPRO_IIC_DATALEN_SHIFT) & SLIMPRO_IIC_DATALEN_MASK))

//
// Encode for upper address for block data
//
pub const SLIMPRO_IIC_ENCODE_FLAG_BUFADDR: c_uint = 0x80000000;

    & 0x40000000))

    & 0x3FF00000))

pub const SLIMPRO_IIC_MSG_DWORD_COUNT: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct slimpro_i2c_dev {
    pub adapter: i2c_adapter,
    pub dev: *mut device,
    pub mbox_chan: *mut mbox_chan,
    pub pcc_chan: *mut pcc_mbox_chan,
    pub mbox_client: mbox_client,
    pub mbox_idx: c_int,
    pub rd_complete: completion,
    pub /: *mut *mut u8 dma_buffer[I2C_SMBUS_BLOCK_MAX + 1]; / dma_buffer[0] is used for length,
    pub resp_msg: *mut u32,
}

    container_of(cl, struct slimpro_i2c_dev, mbox_client)
    enum slimpro_i2c_version {
    XGENE_SLIMPRO_I2C_V1 = 0,
    XGENE_SLIMPRO_I2C_V2 = 1,
    };
//
// This function tests and clears a bitmask then returns its old value
//
#[no_mangle]
unsafe extern "C" fn xgene_word_tst_and_clr(addr: *mut u16, mask: u16) -> u16 {
    static u16 xgene_word_tst_and_clr(u16 *addr, u16 mask)
    {
    u16 ret, val;
    val = le16_to_cpu(READ_ONCE(*addr));
    ret = val & mask;
    val &= ~mask;
    WRITE_ONCE(*addr, cpu_to_le16(val));
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn slimpro_i2c_rx_cb(cl: *mut mbox_client, mssg: *mut c_void) {
    static void slimpro_i2c_rx_cb(struct mbox_client *cl, void *mssg)
    {
    struct slimpro_i2c_dev *ctx = to_slimpro_i2c_dev(cl);
//
// Response message format:
// mssg[0] is the return code of the operation
// mssg[1] is the first data word
// mssg[2] is NOT used
//
    if (ctx.resp_msg)
// ctx->resp_msg = ((u32 *)mssg)[1];
    if (ctx.mbox_client.tx_block)
    complete(&ctx.rd_complete);
    }
#[no_mangle]
unsafe extern "C" fn slimpro_i2c_pcc_rx_cb(cl: *mut mbox_client, msg: *mut c_void) {
    static void slimpro_i2c_pcc_rx_cb(struct mbox_client *cl, void *msg)
    {
    struct slimpro_i2c_dev *ctx = to_slimpro_i2c_dev(cl);
    struct acpi_pcct_shared_memory __iomem *generic_comm_base =
    ctx.pcc_chan.shmem;
// Check if platform sends interrupt
    if (!xgene_word_tst_and_clr(&generic_comm_base.status,
    PCC_STATUS_SCI_DOORBELL))
    return;
    if (xgene_word_tst_and_clr(&generic_comm_base.status,
    PCC_STATUS_CMD_COMPLETE)) {
    msg = generic_comm_base + 1;
// Response message msg[1] contains the return value.
    if (ctx.resp_msg)
// ctx->resp_msg = ((u32 *)msg)[1];
    complete(&ctx.rd_complete);
    }
    }
#[no_mangle]
unsafe extern "C" fn slimpro_i2c_pcc_tx_prepare(ctx: *mut slimpro_i2c_dev, msg: *mut u32) {
    static void slimpro_i2c_pcc_tx_prepare(struct slimpro_i2c_dev *ctx, u32 *msg)
    {
    struct acpi_pcct_shared_memory __iomem *generic_comm_base =
    ctx.pcc_chan.shmem;
    u32 *ptr = (void *)(generic_comm_base + 1);
    u16 status;
    int i;
    WRITE_ONCE(generic_comm_base.signature,
    cpu_to_le32(PCC_SIGNATURE | ctx.mbox_idx));
    WRITE_ONCE(generic_comm_base.command,
    cpu_to_le16(SLIMPRO_MSG_TYPE(msg[0]) | PCC_CMD_GENERATE_DB_INTR));
    status = le16_to_cpu(READ_ONCE(generic_comm_base.status));
    status &= ~PCC_STATUS_CMD_COMPLETE;
    WRITE_ONCE(generic_comm_base.status, cpu_to_le16(status));
// Copy the message to the PCC comm space
    for (i = 0; i < SLIMPRO_IIC_MSG_DWORD_COUNT; i++)
    WRITE_ONCE(ptr[i], cpu_to_le32(msg[i]));
    }
#[no_mangle]
unsafe extern "C" fn start_i2c_msg_xfer(ctx: *mut slimpro_i2c_dev) -> c_int {
    static int start_i2c_msg_xfer(struct slimpro_i2c_dev *ctx)
    {
    if (ctx.mbox_client.tx_block || !acpi_disabled) {
    if (!wait_for_completion_timeout(&ctx.rd_complete,
    msecs_to_jiffies(MAILBOX_OP_TIMEOUT)))
    return -ETIMEDOUT;
    }
// Check of invalid data or no device
    if (*ctx.resp_msg == 0xffffffff)
    return -ENODEV;
    return 0;
    }
    static int slimpro_i2c_send_msg(struct slimpro_i2c_dev *ctx,
    u32 *msg,
    u32 *data)
    {
    int rc;
    ctx.resp_msg = data;
    if (!acpi_disabled) {
    reinit_completion(&ctx.rd_complete);
    slimpro_i2c_pcc_tx_prepare(ctx, msg);
    }
    rc = mbox_send_message(ctx.mbox_chan, msg);
    if (rc < 0)
    goto err;
    rc = start_i2c_msg_xfer(ctx);
    err:
    if (!acpi_disabled)
    mbox_chan_txdone(ctx.mbox_chan, 0);
    ctx.resp_msg = core::ptr::null_mut();
    return rc;
    }
    static int slimpro_i2c_rd(struct slimpro_i2c_dev *ctx, u32 chip,
    u32 addr, u32 addrlen, u32 protocol,
    u32 readlen, u32 *data)
    {
    u32 msg[3];
    msg[0] = SLIMPRO_IIC_ENCODE_MSG(SLIMPRO_IIC_BUS, chip,
    SLIMPRO_IIC_READ, protocol, addrlen, readlen);
    msg[1] = SLIMPRO_IIC_ENCODE_ADDR(addr);
    msg[2] = 0;
    return slimpro_i2c_send_msg(ctx, msg, data);
    }
    static int slimpro_i2c_wr(struct slimpro_i2c_dev *ctx, u32 chip,
    u32 addr, u32 addrlen, u32 protocol, u32 writelen,
    u32 data)
    {
    u32 msg[3];
    msg[0] = SLIMPRO_IIC_ENCODE_MSG(SLIMPRO_IIC_BUS, chip,
    SLIMPRO_IIC_WRITE, protocol, addrlen, writelen);
    msg[1] = SLIMPRO_IIC_ENCODE_ADDR(addr);
    msg[2] = data;
    return slimpro_i2c_send_msg(ctx, msg, msg);
    }
    static int slimpro_i2c_blkrd(struct slimpro_i2c_dev *ctx, u32 chip, u32 addr,
    u32 addrlen, u32 protocol, u32 readlen,
    u32 with_data_len, void *data)
    {
    dma_addr_t paddr;
    u32 msg[3];
    int rc;
    paddr = dma_map_single(ctx.dev, ctx.dma_buffer, readlen, DMA_FROM_DEVICE);
    if (dma_mapping_error(ctx.dev, paddr)) {
    dev_err(&ctx.adapter.dev, "Error in mapping dma buffer %p\n",
    ctx.dma_buffer);
    return -ENOMEM;
    }
    msg[0] = SLIMPRO_IIC_ENCODE_MSG(SLIMPRO_IIC_BUS, chip, SLIMPRO_IIC_READ,
    protocol, addrlen, readlen);
    msg[1] = SLIMPRO_IIC_ENCODE_FLAG_BUFADDR |
    SLIMPRO_IIC_ENCODE_FLAG_WITH_DATA_LEN(with_data_len) |
    SLIMPRO_IIC_ENCODE_UPPER_BUFADDR(paddr) |
    SLIMPRO_IIC_ENCODE_ADDR(addr);
    msg[2] = (u32)paddr;
    rc = slimpro_i2c_send_msg(ctx, msg, msg);
// Copy to destination
    memcpy(data, ctx.dma_buffer, readlen);
    dma_unmap_single(ctx.dev, paddr, readlen, DMA_FROM_DEVICE);
    return rc;
    }
    static int slimpro_i2c_blkwr(struct slimpro_i2c_dev *ctx, u32 chip,
    u32 addr, u32 addrlen, u32 protocol, u32 writelen,
    void *data)
    {
    dma_addr_t paddr;
    u32 msg[3];
    int rc;
    if (writelen > I2C_SMBUS_BLOCK_MAX)
    return -EINVAL;
    memcpy(ctx.dma_buffer, data, writelen);
    paddr = dma_map_single(ctx.dev, ctx.dma_buffer, writelen,
    DMA_TO_DEVICE);
    if (dma_mapping_error(ctx.dev, paddr)) {
    dev_err(&ctx.adapter.dev, "Error in mapping dma buffer %p\n",
    ctx.dma_buffer);
    return -ENOMEM;
    }
    msg[0] = SLIMPRO_IIC_ENCODE_MSG(SLIMPRO_IIC_BUS, chip, SLIMPRO_IIC_WRITE,
    protocol, addrlen, writelen);
    msg[1] = SLIMPRO_IIC_ENCODE_FLAG_BUFADDR |
    SLIMPRO_IIC_ENCODE_UPPER_BUFADDR(paddr) |
    SLIMPRO_IIC_ENCODE_ADDR(addr);
    msg[2] = (u32)paddr;
    if (ctx.mbox_client.tx_block)
    reinit_completion(&ctx.rd_complete);
    rc = slimpro_i2c_send_msg(ctx, msg, msg);
    dma_unmap_single(ctx.dev, paddr, writelen, DMA_TO_DEVICE);
    return rc;
    }
    static int xgene_slimpro_i2c_xfer(struct i2c_adapter *adap, u16 addr,
    unsigned short flags, char read_write,
    u8 command, int size,
    union i2c_smbus_data *data)
    {
    struct slimpro_i2c_dev *ctx = i2c_get_adapdata(adap);
    let mut ret: c_int = -EOPNOTSUPP;
    u32 val;
    switch (size) {
    case I2C_SMBUS_BYTE:
    if (read_write == I2C_SMBUS_READ) {
    ret = slimpro_i2c_rd(ctx, addr, 0, 0,
    SLIMPRO_IIC_SMB_PROTOCOL,
    BYTE_DATA, &val);
    data.byte = val;
    } else {
    ret = slimpro_i2c_wr(ctx, addr, command, SMBUS_CMD_LEN,
    SLIMPRO_IIC_SMB_PROTOCOL,
    0, 0);
    }
    break;
    case I2C_SMBUS_BYTE_DATA:
    if (read_write == I2C_SMBUS_READ) {
    ret = slimpro_i2c_rd(ctx, addr, command, SMBUS_CMD_LEN,
    SLIMPRO_IIC_SMB_PROTOCOL,
    BYTE_DATA, &val);
    data.byte = val;
    } else {
    val = data.byte;
    ret = slimpro_i2c_wr(ctx, addr, command, SMBUS_CMD_LEN,
    SLIMPRO_IIC_SMB_PROTOCOL,
    BYTE_DATA, val);
    }
    break;
    case I2C_SMBUS_WORD_DATA:
    if (read_write == I2C_SMBUS_READ) {
    ret = slimpro_i2c_rd(ctx, addr, command, SMBUS_CMD_LEN,
    SLIMPRO_IIC_SMB_PROTOCOL,
    WORD_DATA, &val);
    data.word = val;
    } else {
    val = data.word;
    ret = slimpro_i2c_wr(ctx, addr, command, SMBUS_CMD_LEN,
    SLIMPRO_IIC_SMB_PROTOCOL,
    WORD_DATA, val);
    }
    break;
    case I2C_SMBUS_BLOCK_DATA:
    if (read_write == I2C_SMBUS_READ) {
    ret = slimpro_i2c_blkrd(ctx, addr, command,
    SMBUS_CMD_LEN,
    SLIMPRO_IIC_SMB_PROTOCOL,
    I2C_SMBUS_BLOCK_MAX + 1,
    IIC_SMB_WITH_DATA_LEN,
    &data.block[0]);
    } else {
    ret = slimpro_i2c_blkwr(ctx, addr, command,
    SMBUS_CMD_LEN,
    SLIMPRO_IIC_SMB_PROTOCOL,
    data.block[0] + 1,
    &data.block[0]);
    }
    break;
    case I2C_SMBUS_I2C_BLOCK_DATA:
    if (read_write == I2C_SMBUS_READ) {
    ret = slimpro_i2c_blkrd(ctx, addr,
    command,
    SMBUS_CMD_LEN,
    SLIMPRO_IIC_I2C_PROTOCOL,
    I2C_SMBUS_BLOCK_MAX,
    IIC_SMB_WITHOUT_DATA_LEN,
    &data.block[1]);
    } else {
    ret = slimpro_i2c_blkwr(ctx, addr, command,
    SMBUS_CMD_LEN,
    SLIMPRO_IIC_I2C_PROTOCOL,
    data.block[0],
    &data.block[1]);
    }
    break;
    default:
    break;
    }
    return ret;
    }
//
// Return list of supported functionality.
//
#[no_mangle]
unsafe extern "C" fn xgene_slimpro_i2c_func(adapter: *mut i2c_adapter) -> u32 {
    static u32 xgene_slimpro_i2c_func(struct i2c_adapter *adapter)
    {
    return I2C_FUNC_SMBUS_BYTE |
    I2C_FUNC_SMBUS_BYTE_DATA |
    I2C_FUNC_SMBUS_WORD_DATA |
    I2C_FUNC_SMBUS_BLOCK_DATA |
    I2C_FUNC_SMBUS_I2C_BLOCK;
    }
    static const struct i2c_algorithm xgene_slimpro_i2c_algorithm = {
    .smbus_xfer = xgene_slimpro_i2c_xfer,
    .functionality = xgene_slimpro_i2c_func,
    };
#[no_mangle]
unsafe extern "C" fn xgene_slimpro_i2c_probe(pdev: *mut platform_device) -> c_int {
    static int xgene_slimpro_i2c_probe(struct platform_device *pdev)
    {
    struct slimpro_i2c_dev *ctx;
    struct i2c_adapter *adapter;
    struct mbox_client *cl;
    int rc;
    ctx = devm_kzalloc(&pdev.dev, sizeof(*ctx), GFP_KERNEL);
    if (!ctx)
    return -ENOMEM;
    ctx.dev = &pdev.dev;
    platform_set_drvdata(pdev, ctx);
    cl = &ctx.mbox_client;
// Request mailbox channel
    cl.dev = &pdev.dev;
    init_completion(&ctx.rd_complete);
    cl.tx_tout = MAILBOX_OP_TIMEOUT;
    cl.knows_txdone = false;
    if (acpi_disabled) {
    cl.tx_block = true;
    cl.rx_callback = slimpro_i2c_rx_cb;
    ctx.mbox_chan = mbox_request_channel(cl, MAILBOX_I2C_INDEX);
    if (IS_ERR(ctx.mbox_chan))
    return dev_err_probe(&pdev.dev, PTR_ERR(ctx.mbox_chan),
    "i2c mailbox channel request failed\n");
    } else {
    struct pcc_mbox_chan *pcc_chan;
    const struct acpi_device_id *acpi_id;
    acpi_id = acpi_match_device(pdev.dev.driver.acpi_match_table,
    &pdev.dev);
    if (!acpi_id)
    return -EINVAL;
    if (device_property_read_u32(&pdev.dev, "pcc-channel",
    &ctx.mbox_idx))
    ctx.mbox_idx = MAILBOX_I2C_INDEX;
    cl.tx_block = false;
    cl.rx_callback = slimpro_i2c_pcc_rx_cb;
    pcc_chan = pcc_mbox_request_channel(cl, ctx.mbox_idx);
    if (IS_ERR(pcc_chan))
    return dev_err_probe(&pdev.dev, PTR_ERR(pcc_chan),
    "PCC mailbox channel request failed\n");
    ctx.pcc_chan = pcc_chan;
    ctx.mbox_chan = pcc_chan.mchan;
    if (!ctx.mbox_chan.mbox.txdone_irq) {
    rc = dev_err_probe(&pdev.dev, -ENOENT,
    "PCC IRQ not supported\n");
    goto mbox_err;
    }
    }
    rc = dma_set_mask_and_coherent(&pdev.dev, DMA_BIT_MASK(64));
    if (rc)
    dev_warn(&pdev.dev, "Unable to set dma mask\n");
// Setup I2C adapter
    adapter = &ctx.adapter;
    snprintf(adapter.name, sizeof(adapter.name), "MAILBOX I2C");
    adapter.algo = &xgene_slimpro_i2c_algorithm;
    adapter.class = I2C_CLASS_HWMON;
    adapter.dev.parent = &pdev.dev;
    adapter.dev.of_node = pdev.dev.of_node;
    ACPI_COMPANION_SET(&adapter.dev, ACPI_COMPANION(&pdev.dev));
    i2c_set_adapdata(adapter, ctx);
    rc = i2c_add_adapter(adapter);
    if (rc)
    goto mbox_err;
    dev_info(&pdev.dev, "Mailbox I2C Adapter registered\n");
    return 0;
    mbox_err:
    if (acpi_disabled)
    mbox_free_channel(ctx.mbox_chan);
    else
    pcc_mbox_free_channel(ctx.pcc_chan);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn xgene_slimpro_i2c_remove(pdev: *mut platform_device) {
    static void xgene_slimpro_i2c_remove(struct platform_device *pdev)
    {
    struct slimpro_i2c_dev *ctx = platform_get_drvdata(pdev);
    i2c_del_adapter(&ctx.adapter);
    if (acpi_disabled)
    mbox_free_channel(ctx.mbox_chan);
    else
    pcc_mbox_free_channel(ctx.pcc_chan);
    }
    static const struct of_device_id xgene_slimpro_i2c_dt_ids[] = {
    {.compatible = "apm,xgene-slimpro-i2c" },
    {},
    };
    MODULE_DEVICE_TABLE(of, xgene_slimpro_i2c_dt_ids);

    static const struct acpi_device_id xgene_slimpro_i2c_acpi_ids[] = {
    {"APMC0D40", XGENE_SLIMPRO_I2C_V1},
    {"APMC0D8B", XGENE_SLIMPRO_I2C_V2},
    {}
    };
    MODULE_DEVICE_TABLE(acpi, xgene_slimpro_i2c_acpi_ids);

    static struct platform_driver xgene_slimpro_i2c_driver = {
    .probe	= xgene_slimpro_i2c_probe,
    .remove = xgene_slimpro_i2c_remove,
    .driver	= {
    .name	= "xgene-slimpro-i2c",
    .of_match_table = of_match_ptr(xgene_slimpro_i2c_dt_ids),
    .acpi_match_table = ACPI_PTR(xgene_slimpro_i2c_acpi_ids)
    },
    };
    module_platform_driver(xgene_slimpro_i2c_driver);
    MODULE_DESCRIPTION("APM X-Gene SLIMpro I2C driver");
    MODULE_AUTHOR("Feng Kan <fkan@apm.com>");
    MODULE_AUTHOR("Hieu Le <hnle@apm.com>");
    MODULE_LICENSE("GPL");
