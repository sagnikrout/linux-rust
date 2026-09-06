//! Automatically rewritten from C to Rust
//! Source: drivers/char/ipmi/ipmi_si_ls2k.c
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
// Driver for Loongson-2K BMC IPMI interface
//
// Copyright (C) 2024-2025 Loongson Technology Corporation Limited.
//
// Authors:
// Chong Qiao <qiaochong@loongson.cn>
// Binbin Zhou <zhoubinbin@loongson.cn>
//

pub const LS2K_KCS_FIFO_IBFH: c_uint = 0x0;
pub const LS2K_KCS_FIFO_IBFT: c_uint = 0x1;
pub const LS2K_KCS_FIFO_OBFH: c_uint = 0x2;
pub const LS2K_KCS_FIFO_OBFT: c_uint = 0x3;
// KCS registers
pub const LS2K_KCS_REG_STS: c_uint = 0x4;
pub const LS2K_KCS_REG_DATA_OUT: c_uint = 0x5;
pub const LS2K_KCS_REG_DATA_IN: c_uint = 0x6;
pub const LS2K_KCS_REG_CMD: c_uint = 0x8;
pub const LS2K_KCS_CMD_DATA: c_uint = 0xa;
pub const LS2K_KCS_VERSION: c_uint = 0xb;
pub const LS2K_KCS_WR_REQ: c_uint = 0xc;
pub const LS2K_KCS_WR_ACK: c_uint = 0x10;

    static bool ls2k_registered;
#[no_mangle]
unsafe extern "C" fn ls2k_mem_inb_v0(io: *const si_sm_io, offset: c_uint) -> c_uchar {
    static unsigned char ls2k_mem_inb_v0(const struct si_sm_io *io, unsigned int offset)
    {
    void __iomem *addr = io.addr;
    int reg_offset;
    if (offset & BIT(0)) {
    reg_offset = LS2K_KCS_REG_STS;
    } else {
    writeb(readb(addr + LS2K_KCS_REG_STS) & ~LS2K_KCS_STS_OBF, addr + LS2K_KCS_REG_STS);
    reg_offset = LS2K_KCS_REG_DATA_OUT;
    }
    return readb(addr + reg_offset);
    }
#[no_mangle]
unsafe extern "C" fn ls2k_mem_inb_v1(io: *const si_sm_io, offset: c_uint) -> c_uchar {
    static unsigned char ls2k_mem_inb_v1(const struct si_sm_io *io, unsigned int offset)
    {
    void __iomem *addr = io.addr;
    let mut inb: c_uchar = 0, cmd;
    bool obf, ibf;
    obf = readb(addr + LS2K_KCS_FIFO_OBFH) ^ readb(addr + LS2K_KCS_FIFO_OBFT);
    ibf = readb(addr + LS2K_KCS_FIFO_IBFH) ^ readb(addr + LS2K_KCS_FIFO_IBFT);
    cmd = readb(addr + LS2K_KCS_CMD_DATA);
    if (offset & BIT(0)) {
    inb = readb(addr + LS2K_KCS_REG_STS) & ~LS2K_KCS_DATA_MASK;
    inb |= FIELD_PREP(LS2K_KCS_STS_OBF, obf)
    | FIELD_PREP(LS2K_KCS_STS_IBF, ibf)
    | FIELD_PREP(LS2K_KCS_STS_CMD, cmd);
    } else {
    inb = readb(addr + LS2K_KCS_REG_DATA_OUT);
    writeb(readb(addr + LS2K_KCS_FIFO_OBFH), addr + LS2K_KCS_FIFO_OBFT);
    }
    return inb;
    }
    static void ls2k_mem_outb_v0(const struct si_sm_io *io, unsigned int offset,
    unsigned char val)
    {
    void __iomem *addr = io.addr;
    let mut sts: c_uchar = readb(addr + LS2K_KCS_REG_STS);
    int reg_offset;
    if (sts & LS2K_KCS_STS_IBF)
    return;
    if (offset & BIT(0)) {
    reg_offset = LS2K_KCS_REG_CMD;
    sts |= LS2K_KCS_STS_CMD;
    } else {
    reg_offset = LS2K_KCS_REG_DATA_IN;
    sts &= ~LS2K_KCS_STS_CMD;
    }
    writew(val, addr + reg_offset);
    writeb(sts | LS2K_KCS_STS_IBF, addr + LS2K_KCS_REG_STS);
    writel(readl(addr + LS2K_KCS_WR_REQ) + 1, addr + LS2K_KCS_WR_REQ);
    }
    static void ls2k_mem_outb_v1(const struct si_sm_io *io, unsigned int offset,
    unsigned char val)
    {
    void __iomem *addr = io.addr;
    unsigned char ibfh, ibft;
    int reg_offset;
    ibfh = readb(addr + LS2K_KCS_FIFO_IBFH);
    ibft = readb(addr + LS2K_KCS_FIFO_IBFT);
    if (ibfh ^ ibft)
    return;
    reg_offset = (offset & BIT(0)) ? LS2K_KCS_REG_CMD : LS2K_KCS_REG_DATA_IN;
    writew(val, addr + reg_offset);
    writeb(offset & BIT(0), addr + LS2K_KCS_CMD_DATA);
    writeb(!ibft, addr + LS2K_KCS_FIFO_IBFH);
    writel(readl(addr + LS2K_KCS_WR_REQ) + 1, addr + LS2K_KCS_WR_REQ);
    }
#[no_mangle]
unsafe extern "C" fn ls2k_mem_cleanup(io: *mut si_sm_io) {
    static void ls2k_mem_cleanup(struct si_sm_io *io)
    {
    if (io.addr)
    iounmap(io.addr);
    }
#[no_mangle]
unsafe extern "C" fn ipmi_ls2k_mem_setup(io: *mut si_sm_io) -> c_int {
    static int ipmi_ls2k_mem_setup(struct si_sm_io *io)
    {
    unsigned char version;
    io.addr = ioremap(io.addr_data, io.regspacing);
    if (!io.addr)
    return -EIO;
    version = readb(io.addr + LS2K_KCS_VERSION);
    io.inputb = version ? ls2k_mem_inb_v1 : ls2k_mem_inb_v0;
    io.outputb = version ? ls2k_mem_outb_v1 : ls2k_mem_outb_v0;
    io.io_cleanup = ls2k_mem_cleanup;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ipmi_ls2k_probe(pdev: *mut platform_device) -> c_int {
    static int ipmi_ls2k_probe(struct platform_device *pdev)
    {
    struct si_sm_io io;
    memset(&io, 0, sizeof(io));
    io.si_info	= &ipmi_kcs_si_info;
    io.io_setup	= ipmi_ls2k_mem_setup;
    io.addr_data	= pdev.resource[0].start;
    io.regspacing	= resource_size(&pdev.resource[0]);
    io.dev		= &pdev.dev;
    dev_dbg(&pdev.dev, "addr 0x%lx, spacing %d.\n", io.addr_data, io.regspacing);
    return ipmi_si_add_smi(&io);
    }
#[no_mangle]
unsafe extern "C" fn ipmi_ls2k_remove(pdev: *mut platform_device) {
    static void ipmi_ls2k_remove(struct platform_device *pdev)
    {
    ipmi_si_remove_by_dev(&pdev.dev);
    }
    static struct platform_driver ipmi_ls2k_platform_driver = {
    .driver = {
    .name = "ls2k-ipmi-si",
    },
    .probe	= ipmi_ls2k_probe,
    .remove	= ipmi_ls2k_remove,
    };
#[no_mangle]
pub unsafe extern "C" fn ipmi_si_ls2k_init() {
    void ipmi_si_ls2k_init(void)
    {
    platform_driver_register(&ipmi_ls2k_platform_driver);
    ls2k_registered = true;
    }
#[no_mangle]
pub unsafe extern "C" fn ipmi_si_ls2k_shutdown() {
    void ipmi_si_ls2k_shutdown(void)
    {
    if (ls2k_registered)
    platform_driver_unregister(&ipmi_ls2k_platform_driver);
    }
