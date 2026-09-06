//! Automatically rewritten from C to Rust
//! Source: drivers/nvmem/zynqmp_nvmem.c
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
// Copyright (C) 2019 Xilinx, Inc.
// Copyright (C) 2022 - 2023, Advanced Micro Devices, Inc.
//

pub const SILICON_REVISION_MASK: c_uint = 0xF;

pub const WORD_INBYTES: c_int = 4;
pub const SOC_VER_SIZE: c_uint = 0x4;
pub const EFUSE_MEMORY_SIZE: c_uint = 0x177;
pub const UNUSED_SPACE: c_uint = 0x8;

    EFUSE_MEMORY_SIZE)
pub const SOC_VERSION_OFFSET: c_uint = 0x0;
pub const EFUSE_START_OFFSET: c_uint = 0xC;
pub const EFUSE_END_OFFSET: c_uint = 0xFC;
pub const EFUSE_PUF_START_OFFSET: c_uint = 0x100;
pub const EFUSE_PUF_MID_OFFSET: c_uint = 0x140;
pub const EFUSE_PUF_END_OFFSET: c_uint = 0x17F;
pub const EFUSE_NOT_ENABLED: c_int = 29;
//
// efuse access type
//
    enum efuse_access {
    EFUSE_READ = 0,
    EFUSE_WRITE
    };
//
// struct xilinx_efuse - the basic structure
// @src:	address of the buffer to store the data to be write/read
// @size:	read/write word count
// @offset:	read/write offset
// @flag:	0 - represents efuse read and 1- represents efuse write
// @pufuserfuse:0 - represents non-puf efuses, offset is used for read/write
// 1 - represents puf user fuse row number.
//
// this structure stores all the required details to
// read/write efuse memory.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xilinx_efuse {
    pub src: u64,
    pub size: u32,
    pub offset: u32,
    pub flag: enum efuse_access,
    pub pufuserfuse: u32,
}

    static int zynqmp_efuse_access(void *context, unsigned int offset,
    void *val, size_t bytes, enum efuse_access flag,
    unsigned int pufflag)
    {
    struct device *dev = context;
    struct xilinx_efuse *efuse;
    dma_addr_t dma_addr;
    dma_addr_t dma_buf;
    let mut words: usize = bytes / WORD_INBYTES;
    int ret;
    unsigned int value;
    char *data;
    if (bytes % WORD_INBYTES != 0) {
    dev_err(dev, "Bytes requested should be word aligned\n");
    return -EOPNOTSUPP;
    }
    if (pufflag == 0 && offset % WORD_INBYTES) {
    dev_err(dev, "Offset requested should be word aligned\n");
    return -EOPNOTSUPP;
    }
    if (pufflag == 1 && flag == EFUSE_WRITE) {
    memcpy(&value, val, sizeof(value));
    if ((offset == EFUSE_PUF_START_OFFSET ||
    offset == EFUSE_PUF_MID_OFFSET) &&
    value & P_USER_0_64_UPPER_MASK) {
    dev_err(dev, "Only lower 4 bytes are allowed to be programmed in P_USER_0 & P_USER_64\n");
    return -EOPNOTSUPP;
    }
    if (offset == EFUSE_PUF_END_OFFSET &&
    (value & P_USER_127_LOWER_4_BIT_MASK)) {
    dev_err(dev, "Only MSB 28 bits are allowed to be programmed for P_USER_127\n");
    return -EOPNOTSUPP;
    }
    }
    efuse = dma_alloc_coherent(dev, sizeof(struct xilinx_efuse),
    &dma_addr, GFP_KERNEL);
    if (!efuse)
    return -ENOMEM;
    data = dma_alloc_coherent(dev, bytes,
    &dma_buf, GFP_KERNEL);
    if (!data) {
    ret = -ENOMEM;
    goto efuse_data_fail;
    }
    if (flag == EFUSE_WRITE) {
    memcpy(data, val, bytes);
    efuse.flag = EFUSE_WRITE;
    } else {
    efuse.flag = EFUSE_READ;
    }
    efuse.src = dma_buf;
    efuse.size = words;
    efuse.offset = offset;
    efuse.pufuserfuse = pufflag;
    zynqmp_pm_efuse_access(dma_addr, (u32 *)&ret);
    if (ret != 0) {
    if (ret == EFUSE_NOT_ENABLED) {
    dev_err(dev, "efuse access is not enabled\n");
    ret = -EOPNOTSUPP;
    } else {
    dev_err(dev, "Error in efuse read %x\n", ret);
    ret = -EPERM;
    }
    goto efuse_access_err;
    }
    if (flag == EFUSE_READ)
    memcpy(val, data, bytes);
    efuse_access_err:
    dma_free_coherent(dev, bytes,
    data, dma_buf);
    efuse_data_fail:
    dma_free_coherent(dev, sizeof(struct xilinx_efuse),
    efuse, dma_addr);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn zynqmp_nvmem_read(context: *mut c_void, offset: c_uint, val: *mut c_void, bytes: usize) -> c_int {
    static int zynqmp_nvmem_read(void *context, unsigned int offset, void *val, size_t bytes)
    {
    struct device *dev = context;
    int ret;
    let mut pufflag: c_int = 0;
    int idcode;
    int version;
    if (offset >= EFUSE_PUF_START_OFFSET && offset <= EFUSE_PUF_END_OFFSET)
    pufflag = 1;
    switch (offset) {
// Soc version offset is zero
    case SOC_VERSION_OFFSET:
    if (bytes != SOC_VER_SIZE)
    return -EOPNOTSUPP;
    ret = zynqmp_pm_get_chipid((u32 *)&idcode, (u32 *)&version);
    if (ret < 0)
    return ret;
    dev_dbg(dev, "Read chipid val %x %x\n", idcode, version);
// (int *)val = version & SILICON_REVISION_MASK;
    break;
// Efuse offset starts from 0xc
    case EFUSE_START_OFFSET ... EFUSE_END_OFFSET:
    case EFUSE_PUF_START_OFFSET ... EFUSE_PUF_END_OFFSET:
    ret = zynqmp_efuse_access(context, offset, val,
    bytes, EFUSE_READ, pufflag);
    break;
    default:
// (u32 *)val = 0xDEADBEEF;
    ret = 0;
    break;
    }
    return ret;
    }
    static int zynqmp_nvmem_write(void *context,
    unsigned int offset, void *val, size_t bytes)
    {
    let mut pufflag: c_int = 0;
    if (offset < EFUSE_START_OFFSET || offset > EFUSE_PUF_END_OFFSET)
    return -EOPNOTSUPP;
    if (offset >= EFUSE_PUF_START_OFFSET && offset <= EFUSE_PUF_END_OFFSET)
    pufflag = 1;
    return zynqmp_efuse_access(context, offset,
    val, bytes, EFUSE_WRITE, pufflag);
    }
    static const struct of_device_id zynqmp_nvmem_match[] = {
    { .compatible = "xlnx,zynqmp-nvmem-fw", },
    { /* sentinel */ },
    };
    MODULE_DEVICE_TABLE(of, zynqmp_nvmem_match);
#[no_mangle]
unsafe extern "C" fn zynqmp_nvmem_probe(pdev: *mut platform_device) -> c_int {
    static int zynqmp_nvmem_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    let mut econfig: nvmem_config = {};
    econfig.name = "zynqmp-nvmem";
    econfig.owner = THIS_MODULE;
    econfig.word_size = 1;
    econfig.size = ZYNQMP_NVMEM_SIZE;
    econfig.dev = dev;
    econfig.priv = dev;
    econfig.add_legacy_fixed_of_cells = true;
    econfig.reg_read = zynqmp_nvmem_read;
    econfig.reg_write = zynqmp_nvmem_write;
    return PTR_ERR_OR_ZERO(devm_nvmem_register(dev, &econfig));
    }
    static struct platform_driver zynqmp_nvmem_driver = {
    .probe = zynqmp_nvmem_probe,
    .driver = {
    .name = "zynqmp-nvmem",
    .of_match_table = zynqmp_nvmem_match,
    },
    };
    module_platform_driver(zynqmp_nvmem_driver);
    MODULE_AUTHOR("Michal Simek <michal.simek@amd.com>, Nava kishore Manne <nava.kishore.manne@amd.com>");
    MODULE_DESCRIPTION("ZynqMP NVMEM driver");
    MODULE_LICENSE("GPL");
