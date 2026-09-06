//! Automatically rewritten from C to Rust
//! Source: drivers/dma/xilinx/xilinx_dpdma.c
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
// Xilinx ZynqMP DPDMA Engine driver
//
// Copyright (C) 2015 - 2020 Xilinx, Inc.
//
// Author: Hyun Woo Kwon <hyun.kwon@xilinx.com>
//

// DPDMA registers
pub const XILINX_DPDMA_ERR_CTRL: c_uint = 0x000;
pub const XILINX_DPDMA_ISR: c_uint = 0x004;
pub const XILINX_DPDMA_IMR: c_uint = 0x008;
pub const XILINX_DPDMA_IEN: c_uint = 0x00c;
pub const XILINX_DPDMA_IDS: c_uint = 0x010;

pub const XILINX_DPDMA_INTR_CHAN_ERR_MASK: c_uint = 0x00041000;
pub const XILINX_DPDMA_INTR_CHAN_ERR: c_uint = 0x00fff000;
pub const XILINX_DPDMA_INTR_GLOBAL_ERR: c_uint = 0x07000000;
pub const XILINX_DPDMA_INTR_ERR_ALL: c_uint = 0x07fff000;
pub const XILINX_DPDMA_INTR_CHAN_MASK: c_uint = 0x00041041;
pub const XILINX_DPDMA_INTR_GLOBAL_MASK: c_uint = 0x0f000000;
pub const XILINX_DPDMA_INTR_ALL: c_uint = 0x0fffffff;
pub const XILINX_DPDMA_EISR: c_uint = 0x014;
pub const XILINX_DPDMA_EIMR: c_uint = 0x018;
pub const XILINX_DPDMA_EIEN: c_uint = 0x01c;
pub const XILINX_DPDMA_EIDS: c_uint = 0x020;

pub const XILINX_DPDMA_EINTR_CHAN_ERR_MASK: c_uint = 0x02082082;
pub const XILINX_DPDMA_EINTR_CHAN_ERR: c_uint = 0x7ffffffe;
pub const XILINX_DPDMA_EINTR_GLOBAL_ERR: c_uint = 0x80000001;
pub const XILINX_DPDMA_EINTR_ALL: c_uint = 0xffffffff;
pub const XILINX_DPDMA_CNTL: c_uint = 0x100;
pub const XILINX_DPDMA_GBL: c_uint = 0x104;

pub const XILINX_DPDMA_ALC0_CNTL: c_uint = 0x108;
pub const XILINX_DPDMA_ALC0_STATUS: c_uint = 0x10c;
pub const XILINX_DPDMA_ALC0_MAX: c_uint = 0x110;
pub const XILINX_DPDMA_ALC0_MIN: c_uint = 0x114;
pub const XILINX_DPDMA_ALC0_ACC: c_uint = 0x118;
pub const XILINX_DPDMA_ALC0_ACC_TRAN: c_uint = 0x11c;
pub const XILINX_DPDMA_ALC1_CNTL: c_uint = 0x120;
pub const XILINX_DPDMA_ALC1_STATUS: c_uint = 0x124;
pub const XILINX_DPDMA_ALC1_MAX: c_uint = 0x128;
pub const XILINX_DPDMA_ALC1_MIN: c_uint = 0x12c;
pub const XILINX_DPDMA_ALC1_ACC: c_uint = 0x130;
pub const XILINX_DPDMA_ALC1_ACC_TRAN: c_uint = 0x134;
// Channel register
pub const XILINX_DPDMA_CH_BASE: c_uint = 0x200;
pub const XILINX_DPDMA_CH_OFFSET: c_uint = 0x100;
pub const XILINX_DPDMA_CH_DESC_START_ADDRE: c_uint = 0x000;

pub const XILINX_DPDMA_CH_DESC_START_ADDR: c_uint = 0x004;
pub const XILINX_DPDMA_CH_DESC_NEXT_ADDRE: c_uint = 0x008;
pub const XILINX_DPDMA_CH_DESC_NEXT_ADDR: c_uint = 0x00c;
pub const XILINX_DPDMA_CH_PYLD_CUR_ADDRE: c_uint = 0x010;
pub const XILINX_DPDMA_CH_PYLD_CUR_ADDR: c_uint = 0x014;
pub const XILINX_DPDMA_CH_CNTL: c_uint = 0x018;

pub const XILINX_DPDMA_CH_CNTL_QOS_VID_CLASS: c_int = 11;
pub const XILINX_DPDMA_CH_STATUS: c_uint = 0x01c;

pub const XILINX_DPDMA_CH_VDO: c_uint = 0x020;
pub const XILINX_DPDMA_CH_PYLD_SZ: c_uint = 0x024;
pub const XILINX_DPDMA_CH_DESC_ID: c_uint = 0x028;

// DPDMA descriptor fields
pub const XILINX_DPDMA_DESC_CONTROL_PREEMBLE: c_uint = 0xa5;

pub const XILINX_DPDMA_ALIGN_BYTES: c_int = 256;
pub const XILINX_DPDMA_LINESIZE_ALIGN_BITS: c_int = 128;
pub const XILINX_DPDMA_NUM_CHAN: c_int = 6;
    struct xilinx_dpdma_chan;
//
// struct xilinx_dpdma_hw_desc - DPDMA hardware descriptor
// @control: control configuration field
// @desc_id: descriptor ID
// @xfer_size: transfer size
// @hsize_stride: horizontal size and stride
// @timestamp_lsb: LSB of time stamp
// @timestamp_msb: MSB of time stamp
// @addr_ext: upper 16 bit of 48 bit address (next_desc and src_addr)
// @next_desc: next descriptor 32 bit address
// @src_addr: payload source address (1st page, 32 LSB)
// @addr_ext_23: payload source address (2nd and 3rd pages, 16 LSBs)
// @addr_ext_45: payload source address (4th and 5th pages, 16 LSBs)
// @src_addr2: payload source address (2nd page, 32 LSB)
// @src_addr3: payload source address (3rd page, 32 LSB)
// @src_addr4: payload source address (4th page, 32 LSB)
// @src_addr5: payload source address (5th page, 32 LSB)
// @crc: descriptor CRC
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xilinx_dpdma_hw_desc {
    pub control: u32,
    pub desc_id: u32,
    pub xfer_size: u32,
    pub hsize_stride: u32,
    pub timestamp_lsb: u32,
    pub timestamp_msb: u32,
    pub addr_ext: u32,
    pub next_desc: u32,
    pub src_addr: u32,
    pub addr_ext_23: u32,
    pub addr_ext_45: u32,
    pub src_addr2: u32,
    pub src_addr3: u32,
    pub src_addr4: u32,
    pub src_addr5: u32,
    pub crc: u32,
    pub __aligned(XILINX_DPDMA_ALIGN_BYTES): },
//
// struct xilinx_dpdma_sw_desc - DPDMA software descriptor
// @hw: DPDMA hardware descriptor
// @node: list node for software descriptors
// @dma_addr: DMA address of the software descriptor
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xilinx_dpdma_sw_desc {
    pub hw: xilinx_dpdma_hw_desc,
    pub node: list_head,
    pub dma_addr: dma_addr_t,
}

//
// struct xilinx_dpdma_tx_desc - DPDMA transaction descriptor
// @vdesc: virtual DMA descriptor
// @chan: DMA channel
// @descriptors: list of software descriptors
// @error: an error has been detected with this descriptor
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xilinx_dpdma_tx_desc {
    pub vdesc: virt_dma_desc,
    pub chan: *mut xilinx_dpdma_chan,
    pub descriptors: list_head,
    pub error: bool,
}

    container_of(_desc, struct xilinx_dpdma_tx_desc, vdesc)
//
// struct xilinx_dpdma_chan - DPDMA channel
// @vchan: virtual DMA channel
// @reg: register base address
// @id: channel ID
// @wait_to_stop: queue to wait for outstanding transactions before stopping
// @running: true if the channel is running
// @first_frame: flag for the first frame of stream
// @video_group: flag if multi-channel operation is needed for video channels
// @lock: lock to access struct xilinx_dpdma_chan. Must be taken before
// @vchan.lock, if both are to be held.
// @desc_pool: descriptor allocation pool
// @err_task: error IRQ bottom half handler
// @desc: References to descriptors being processed
// @desc.pending: Descriptor schedule to the hardware, pending execution
// @desc.active: Descriptor being executed by the hardware
// @xdev: DPDMA device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xilinx_dpdma_chan {
    pub vchan: virt_dma_chan,
    pub reg: *mut void __iomem,
    pub id: c_uint,
    pub wait_to_stop: wait_queue_head_t,
    pub running: bool,
    pub first_frame: bool,
    pub video_group: bool,
    pub /: *mut *mut spinlock_t lock; / lock to access struct xilinx_dpdma_chan,
    pub desc_pool: *mut dma_pool,
    pub err_task: tasklet_struct,
    struct {
    pub pending: *mut xilinx_dpdma_tx_desc,
    pub active: *mut xilinx_dpdma_tx_desc,
    pub desc: },
    pub xdev: *mut xilinx_dpdma_device,
}

    container_of(_chan, struct xilinx_dpdma_chan, vchan.chan)
//
// struct xilinx_dpdma_device - DPDMA device
// @common: generic dma device structure
// @reg: register base address
// @dev: generic device structure
// @irq: the interrupt number
// @axi_clk: axi clock
// @chan: DPDMA channels
// @ext_addr: flag for 64 bit system (48 bit addressing)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xilinx_dpdma_device {
    pub common: dma_device,
    pub reg: *mut void __iomem,
    pub dev: *mut device,
    pub irq: c_int,
    pub axi_clk: *mut clk,
    pub chan: [*mut xilinx_dpdma_chan; XILINX_DPDMA_NUM_CHAN],
    pub ext_addr: bool,
}

// -----------------------------------------------------------------------------
// DebugFS
//
pub const XILINX_DPDMA_DEBUGFS_READ_MAX_SIZE: c_int = 32;

// Match xilinx_dpdma_testcases vs dpdma_debugfs_reqs[] entry
    enum xilinx_dpdma_testcases {
    DPDMA_TC_INTR_DONE,
    DPDMA_TC_NONE
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xilinx_dpdma_debugfs {
    pub testcase: enum xilinx_dpdma_testcases,
    pub xilinx_dpdma_irq_done_count: u16,
    pub chan_id: c_uint,
}

    static struct xilinx_dpdma_debugfs dpdma_debugfs;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xilinx_dpdma_debugfs_request {
    pub name: *const c_char,
    pub tc: enum xilinx_dpdma_testcases,
    pub buf): *mut *mut ssize_t (read)(char,
    pub args): *mut *mut int (write)(char,
}

#[no_mangle]
unsafe extern "C" fn xilinx_dpdma_debugfs_desc_done_irq(chan: *mut xilinx_dpdma_chan) {
    static void xilinx_dpdma_debugfs_desc_done_irq(struct xilinx_dpdma_chan *chan)
    {
    if (IS_ENABLED(CONFIG_DEBUG_FS) && chan.id == dpdma_debugfs.chan_id)
    dpdma_debugfs.xilinx_dpdma_irq_done_count++;
    }
#[no_mangle]
unsafe extern "C" fn xilinx_dpdma_debugfs_desc_done_irq_read(buf: *mut c_char) -> isize {
    static ssize_t xilinx_dpdma_debugfs_desc_done_irq_read(char *buf)
    {
    size_t out_str_len;
    dpdma_debugfs.testcase = DPDMA_TC_NONE;
    out_str_len = strlen(XILINX_DPDMA_DEBUGFS_UINT16_MAX_STR);
    out_str_len = min_t(size_t, XILINX_DPDMA_DEBUGFS_READ_MAX_SIZE,
    out_str_len + 1);
    snprintf(buf, out_str_len, "%d",
    dpdma_debugfs.xilinx_dpdma_irq_done_count);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xilinx_dpdma_debugfs_desc_done_irq_write(args: *mut c_char) -> c_int {
    static int xilinx_dpdma_debugfs_desc_done_irq_write(char *args)
    {
    char *arg;
    int ret;
    u32 id;
    arg = strsep(&args, " ");
    if (!arg || strncasecmp(arg, "start", 5))
    return -EINVAL;
    arg = strsep(&args, " ");
    if (!arg)
    return -EINVAL;
    ret = kstrtou32(arg, 0, &id);
    if (ret < 0)
    return ret;
    if (id < ZYNQMP_DPDMA_VIDEO0 || id > ZYNQMP_DPDMA_AUDIO1)
    return -EINVAL;
    dpdma_debugfs.testcase = DPDMA_TC_INTR_DONE;
    dpdma_debugfs.xilinx_dpdma_irq_done_count = 0;
    dpdma_debugfs.chan_id = id;
    return 0;
    }
// Match xilinx_dpdma_testcases vs dpdma_debugfs_reqs[] entry
    static struct xilinx_dpdma_debugfs_request dpdma_debugfs_reqs[] = {
    {
    .name = "DESCRIPTOR_DONE_INTR",
    .tc = DPDMA_TC_INTR_DONE,
    .read = xilinx_dpdma_debugfs_desc_done_irq_read,
    .write = xilinx_dpdma_debugfs_desc_done_irq_write,
    },
    };
    static ssize_t xilinx_dpdma_debugfs_read(struct file *f, char __user *buf,
    size_t size, loff_t *pos)
    {
    enum xilinx_dpdma_testcases testcase;
    char *kern_buff;
    let mut ret: c_int = 0;
    if (*pos != 0 || size <= 0)
    return -EINVAL;
    kern_buff = kzalloc(XILINX_DPDMA_DEBUGFS_READ_MAX_SIZE, GFP_KERNEL);
    if (!kern_buff) {
    dpdma_debugfs.testcase = DPDMA_TC_NONE;
    return -ENOMEM;
    }
    testcase = READ_ONCE(dpdma_debugfs.testcase);
    if (testcase != DPDMA_TC_NONE) {
    ret = dpdma_debugfs_reqs[testcase].read(kern_buff);
    if (ret < 0)
    goto done;
    } else {
    strscpy(kern_buff, "No testcase executed",
    XILINX_DPDMA_DEBUGFS_READ_MAX_SIZE);
    }
    size = min(size, strlen(kern_buff));
    if (copy_to_user(buf, kern_buff, size))
    ret = -EFAULT;
    done:
    kfree(kern_buff);
    if (ret)
    return ret;
// pos = size + 1;
    return size;
    }
    static ssize_t xilinx_dpdma_debugfs_write(struct file *f,
    const char __user *buf, size_t size,
    loff_t *pos)
    {
    char *kern_buff, *kern_buff_start;
    char *testcase;
    unsigned int i;
    int ret;
    if (*pos != 0 || size <= 0)
    return -EINVAL;
// Supporting single instance of test as of now.
    if (dpdma_debugfs.testcase != DPDMA_TC_NONE)
    return -EBUSY;
    kern_buff = kzalloc(size, GFP_KERNEL);
    if (!kern_buff)
    return -ENOMEM;
    kern_buff_start = kern_buff;
    ret = strncpy_from_user(kern_buff, buf, size);
    if (ret < 0)
    goto done;
// Read the testcase name from a user request.
    testcase = strsep(&kern_buff, " ");
    for (i = 0; i < ARRAY_SIZE(dpdma_debugfs_reqs); i++) {
    if (!strcasecmp(testcase, dpdma_debugfs_reqs[i].name))
    break;
    }
    if (i == ARRAY_SIZE(dpdma_debugfs_reqs)) {
    ret = -EINVAL;
    goto done;
    }
    ret = dpdma_debugfs_reqs[i].write(kern_buff);
    if (ret < 0)
    goto done;
    ret = size;
    done:
    kfree(kern_buff_start);
    return ret;
    }
    static const struct file_operations fops_xilinx_dpdma_dbgfs = {
    .owner = THIS_MODULE,
    .read = xilinx_dpdma_debugfs_read,
    .write = xilinx_dpdma_debugfs_write,
    };
#[no_mangle]
unsafe extern "C" fn xilinx_dpdma_debugfs_init(xdev: *mut xilinx_dpdma_device) {
    static void xilinx_dpdma_debugfs_init(struct xilinx_dpdma_device *xdev)
    {
    struct dentry *dent;
    dpdma_debugfs.testcase = DPDMA_TC_NONE;
    dent = debugfs_create_file("testcase", 0444, xdev.common.dbg_dev_root,
    core::ptr::null_mut(), &fops_xilinx_dpdma_dbgfs);
    if (IS_ERR(dent))
    dev_err(xdev.dev, "Failed to create debugfs testcase file\n");
    }
// -----------------------------------------------------------------------------
// I/O Accessors
//
#[no_mangle]
pub unsafe extern "C" fn dpdma_read(base: *mut void __iomem, offset: u32) -> u32 {
    static inline u32 dpdma_read(void __iomem *base, u32 offset)
    {
    return ioread32(base + offset);
    }
#[no_mangle]
pub unsafe extern "C" fn dpdma_write(base: *mut void __iomem, offset: u32, val: u32) {
    static inline void dpdma_write(void __iomem *base, u32 offset, u32 val)
    {
    iowrite32(val, base + offset);
    }
#[no_mangle]
pub unsafe extern "C" fn dpdma_clr(base: *mut void __iomem, offset: u32, clr: u32) {
    static inline void dpdma_clr(void __iomem *base, u32 offset, u32 clr)
    {
    dpdma_write(base, offset, dpdma_read(base, offset) & ~clr);
    }
#[no_mangle]
pub unsafe extern "C" fn dpdma_set(base: *mut void __iomem, offset: u32, set: u32) {
    static inline void dpdma_set(void __iomem *base, u32 offset, u32 set)
    {
    dpdma_write(base, offset, dpdma_read(base, offset) | set);
    }
// -----------------------------------------------------------------------------
// Descriptor Operations
//
// xilinx_dpdma_sw_desc_set_dma_addrs - Set DMA addresses in the descriptor
// @xdev: DPDMA device
// @sw_desc: The software descriptor in which to set DMA addresses
// @prev: The previous descriptor
// @dma_addr: array of dma addresses
// @num_src_addr: number of addresses in @dma_addr
//
// Set all the DMA addresses in the hardware descriptor corresponding to @dev
// from @dma_addr. If a previous descriptor is specified in @prev, its next
// descriptor DMA address is set to the DMA address of @sw_desc. @prev may be
// identical to @sw_desc for cyclic transfers.
//
    static void xilinx_dpdma_sw_desc_set_dma_addrs(struct xilinx_dpdma_device *xdev,
    struct xilinx_dpdma_sw_desc *sw_desc,
    struct xilinx_dpdma_sw_desc *prev,
    dma_addr_t dma_addr[],
    unsigned int num_src_addr)
    {
    struct xilinx_dpdma_hw_desc *hw_desc = &sw_desc.hw;
    unsigned int i;
    hw_desc.src_addr = lower_32_bits(dma_addr[0]);
    if (xdev.ext_addr)
    hw_desc.addr_ext |=
    FIELD_PREP(XILINX_DPDMA_DESC_ADDR_EXT_SRC_ADDR_MASK,
    upper_32_bits(dma_addr[0]));
    for (i = 1; i < num_src_addr; i++) {
    u32 *addr = &hw_desc.src_addr2;
    addr[i - 1] = lower_32_bits(dma_addr[i]);
    if (xdev.ext_addr) {
    u32 *addr_ext = &hw_desc.addr_ext_23;
    u32 addr_msb;
    addr_msb = upper_32_bits(dma_addr[i]) & GENMASK(15, 0);
    addr_msb <<= 16 * ((i - 1) % 2);
    addr_ext[(i - 1) / 2] |= addr_msb;
    }
    }
    if (!prev)
    return;
    prev.hw.next_desc = lower_32_bits(sw_desc.dma_addr);
    if (xdev.ext_addr)
    prev.hw.addr_ext |=
    FIELD_PREP(XILINX_DPDMA_DESC_ADDR_EXT_NEXT_ADDR_MASK,
    upper_32_bits(sw_desc.dma_addr));
    }
//
// xilinx_dpdma_chan_alloc_sw_desc - Allocate a software descriptor
// @chan: DPDMA channel
//
// Allocate a software descriptor from the channel's descriptor pool.
//
// Return: a software descriptor or NULL.
//
    static struct xilinx_dpdma_sw_desc *
    xilinx_dpdma_chan_alloc_sw_desc(struct xilinx_dpdma_chan *chan)
    {
    struct xilinx_dpdma_sw_desc *sw_desc;
    dma_addr_t dma_addr;
    sw_desc = dma_pool_zalloc(chan.desc_pool, GFP_ATOMIC, &dma_addr);
    if (!sw_desc)
    return core::ptr::null_mut();
    sw_desc.dma_addr = dma_addr;
    return sw_desc;
    }
//
// xilinx_dpdma_chan_free_sw_desc - Free a software descriptor
// @chan: DPDMA channel
// @sw_desc: software descriptor to free
//
// Free a software descriptor from the channel's descriptor pool.
//
    static void
    xilinx_dpdma_chan_free_sw_desc(struct xilinx_dpdma_chan *chan,
    struct xilinx_dpdma_sw_desc *sw_desc)
    {
    dma_pool_free(chan.desc_pool, sw_desc, sw_desc.dma_addr);
    }
//
// xilinx_dpdma_chan_dump_tx_desc - Dump a tx descriptor
// @chan: DPDMA channel
// @tx_desc: tx descriptor to dump
//
// Dump contents of a tx descriptor
//
    static void xilinx_dpdma_chan_dump_tx_desc(struct xilinx_dpdma_chan *chan,
    struct xilinx_dpdma_tx_desc *tx_desc)
    {
    struct xilinx_dpdma_sw_desc *sw_desc;
    struct device *dev = chan.xdev.dev;
    let mut i: c_uint = 0;
    dev_dbg(dev, "------- TX descriptor dump start -------\n");
    dev_dbg(dev, "------- channel ID = %d -------\n", chan.id);
    list_for_each_entry(sw_desc, &tx_desc.descriptors, node) {
    struct xilinx_dpdma_hw_desc *hw_desc = &sw_desc.hw;
    dev_dbg(dev, "------- HW descriptor %d -------\n", i++);
    dev_dbg(dev, "descriptor DMA addr: %pad\n", &sw_desc.dma_addr);
    dev_dbg(dev, "control: 0x%08x\n", hw_desc.control);
    dev_dbg(dev, "desc_id: 0x%08x\n", hw_desc.desc_id);
    dev_dbg(dev, "xfer_size: 0x%08x\n", hw_desc.xfer_size);
    dev_dbg(dev, "hsize_stride: 0x%08x\n", hw_desc.hsize_stride);
    dev_dbg(dev, "timestamp_lsb: 0x%08x\n", hw_desc.timestamp_lsb);
    dev_dbg(dev, "timestamp_msb: 0x%08x\n", hw_desc.timestamp_msb);
    dev_dbg(dev, "addr_ext: 0x%08x\n", hw_desc.addr_ext);
    dev_dbg(dev, "next_desc: 0x%08x\n", hw_desc.next_desc);
    dev_dbg(dev, "src_addr: 0x%08x\n", hw_desc.src_addr);
    dev_dbg(dev, "addr_ext_23: 0x%08x\n", hw_desc.addr_ext_23);
    dev_dbg(dev, "addr_ext_45: 0x%08x\n", hw_desc.addr_ext_45);
    dev_dbg(dev, "src_addr2: 0x%08x\n", hw_desc.src_addr2);
    dev_dbg(dev, "src_addr3: 0x%08x\n", hw_desc.src_addr3);
    dev_dbg(dev, "src_addr4: 0x%08x\n", hw_desc.src_addr4);
    dev_dbg(dev, "src_addr5: 0x%08x\n", hw_desc.src_addr5);
    dev_dbg(dev, "crc: 0x%08x\n", hw_desc.crc);
    }
    dev_dbg(dev, "------- TX descriptor dump end -------\n");
    }
//
// xilinx_dpdma_chan_alloc_tx_desc - Allocate a transaction descriptor
// @chan: DPDMA channel
//
// Allocate a tx descriptor.
//
// Return: a tx descriptor or NULL.
//
    static struct xilinx_dpdma_tx_desc *
    xilinx_dpdma_chan_alloc_tx_desc(struct xilinx_dpdma_chan *chan)
    {
    struct xilinx_dpdma_tx_desc *tx_desc;
    tx_desc = kzalloc_obj(*tx_desc, GFP_NOWAIT);
    if (!tx_desc)
    return core::ptr::null_mut();
    INIT_LIST_HEAD(&tx_desc.descriptors);
    tx_desc.chan = chan;
    tx_desc.error = false;
    return tx_desc;
    }
//
// xilinx_dpdma_chan_free_tx_desc - Free a virtual DMA descriptor
// @vdesc: virtual DMA descriptor
//
// Free the virtual DMA descriptor @vdesc including its software descriptors.
//
#[no_mangle]
unsafe extern "C" fn xilinx_dpdma_chan_free_tx_desc(vdesc: *mut virt_dma_desc) {
    static void xilinx_dpdma_chan_free_tx_desc(struct virt_dma_desc *vdesc)
    {
    struct xilinx_dpdma_sw_desc *sw_desc, *next;
    struct xilinx_dpdma_tx_desc *desc;
    if (!vdesc)
    return;
    desc = to_dpdma_tx_desc(vdesc);
    list_for_each_entry_safe(sw_desc, next, &desc.descriptors, node) {
    list_del(&sw_desc.node);
    xilinx_dpdma_chan_free_sw_desc(desc.chan, sw_desc);
    }
    kfree(desc);
    }
//
// xilinx_dpdma_chan_prep_cyclic - Prepare a cyclic dma descriptor
// @chan: DPDMA channel
// @buf_addr: buffer address
// @buf_len: buffer length
// @period_len: number of periods
// @flags: tx flags argument passed in to prepare function
//
// Prepare a tx descriptor incudling internal software/hardware descriptors
// for the given cyclic transaction.
//
// Return: A dma async tx descriptor on success, or NULL.
//
    static struct dma_async_tx_descriptor *
    xilinx_dpdma_chan_prep_cyclic(struct xilinx_dpdma_chan *chan,
    dma_addr_t buf_addr, size_t buf_len,
    size_t period_len, unsigned long flags)
    {
    struct xilinx_dpdma_tx_desc *tx_desc;
    struct xilinx_dpdma_sw_desc *sw_desc, *last = core::ptr::null_mut();
    let mut periods: c_uint = buf_len / period_len;
    unsigned int i;
    tx_desc = xilinx_dpdma_chan_alloc_tx_desc(chan);
    if (!tx_desc)
    return core::ptr::null_mut();
    for (i = 0; i < periods; i++) {
    struct xilinx_dpdma_hw_desc *hw_desc;
    if (!IS_ALIGNED(buf_addr, XILINX_DPDMA_ALIGN_BYTES)) {
    dev_err(chan.xdev.dev,
    "buffer should be aligned at %d B\n",
    XILINX_DPDMA_ALIGN_BYTES);
    goto error;
    }
    sw_desc = xilinx_dpdma_chan_alloc_sw_desc(chan);
    if (!sw_desc)
    goto error;
    xilinx_dpdma_sw_desc_set_dma_addrs(chan.xdev, sw_desc, last,
    &buf_addr, 1);
    hw_desc = &sw_desc.hw;
    hw_desc.xfer_size = period_len;
    hw_desc.hsize_stride =
    FIELD_PREP(XILINX_DPDMA_DESC_HSIZE_STRIDE_HSIZE_MASK,
    period_len) |
    FIELD_PREP(XILINX_DPDMA_DESC_HSIZE_STRIDE_STRIDE_MASK,
    period_len);
    hw_desc.control = XILINX_DPDMA_DESC_CONTROL_PREEMBLE |
    XILINX_DPDMA_DESC_CONTROL_IGNORE_DONE |
    XILINX_DPDMA_DESC_CONTROL_COMPLETE_INTR;
    list_add_tail(&sw_desc.node, &tx_desc.descriptors);
    buf_addr += period_len;
    last = sw_desc;
    }
    sw_desc = list_first_entry(&tx_desc.descriptors,
    struct xilinx_dpdma_sw_desc, node);
    last.hw.next_desc = lower_32_bits(sw_desc.dma_addr);
    if (chan.xdev.ext_addr)
    last.hw.addr_ext |=
    FIELD_PREP(XILINX_DPDMA_DESC_ADDR_EXT_NEXT_ADDR_MASK,
    upper_32_bits(sw_desc.dma_addr));
    last.hw.control |= XILINX_DPDMA_DESC_CONTROL_LAST_OF_FRAME;
    return vchan_tx_prep(&chan.vchan, &tx_desc.vdesc, flags);
    error:
    xilinx_dpdma_chan_free_tx_desc(&tx_desc.vdesc);
    return core::ptr::null_mut();
    }
//
// xilinx_dpdma_chan_prep_interleaved_dma - Prepare an interleaved dma
// descriptor
// @chan: DPDMA channel
// @xt: dma interleaved template
//
// Prepare a tx descriptor including internal software/hardware descriptors
// based on @xt.
//
// Return: A DPDMA TX descriptor on success, or NULL.
//
    static struct xilinx_dpdma_tx_desc *
    xilinx_dpdma_chan_prep_interleaved_dma(struct xilinx_dpdma_chan *chan,
    struct dma_interleaved_template *xt)
    {
    struct xilinx_dpdma_tx_desc *tx_desc;
    struct xilinx_dpdma_sw_desc *sw_desc;
    struct xilinx_dpdma_hw_desc *hw_desc;
    let mut hsize: usize = xt.sgl[0].size;
    let mut stride: usize = hsize + xt.sgl[0].icg;
    if (!IS_ALIGNED(xt.src_start, XILINX_DPDMA_ALIGN_BYTES)) {
    dev_err(chan.xdev.dev,
    "chan%u: buffer should be aligned at %d B\n",
    chan.id, XILINX_DPDMA_ALIGN_BYTES);
    return core::ptr::null_mut();
    }
    tx_desc = xilinx_dpdma_chan_alloc_tx_desc(chan);
    if (!tx_desc)
    return core::ptr::null_mut();
    sw_desc = xilinx_dpdma_chan_alloc_sw_desc(chan);
    if (!sw_desc) {
    xilinx_dpdma_chan_free_tx_desc(&tx_desc.vdesc);
    return core::ptr::null_mut();
    }
    xilinx_dpdma_sw_desc_set_dma_addrs(chan.xdev, sw_desc, sw_desc,
    &xt.src_start, 1);
    hw_desc = &sw_desc.hw;
    hsize = ALIGN(hsize, XILINX_DPDMA_LINESIZE_ALIGN_BITS / 8);
    hw_desc.xfer_size = hsize * xt.numf;
    hw_desc.hsize_stride =
    FIELD_PREP(XILINX_DPDMA_DESC_HSIZE_STRIDE_HSIZE_MASK, hsize) |
    FIELD_PREP(XILINX_DPDMA_DESC_HSIZE_STRIDE_STRIDE_MASK,
    stride / 16);
    hw_desc.control |= XILINX_DPDMA_DESC_CONTROL_PREEMBLE;
    hw_desc.control |= XILINX_DPDMA_DESC_CONTROL_COMPLETE_INTR;
    hw_desc.control |= XILINX_DPDMA_DESC_CONTROL_IGNORE_DONE;
    hw_desc.control |= XILINX_DPDMA_DESC_CONTROL_LAST_OF_FRAME;
    list_add_tail(&sw_desc.node, &tx_desc.descriptors);
    return tx_desc;
    }
// -----------------------------------------------------------------------------
// DPDMA Channel Operations
//
// xilinx_dpdma_chan_enable - Enable the channel
// @chan: DPDMA channel
//
// Enable the channel and its interrupts. Set the QoS values for video class.
//
#[no_mangle]
unsafe extern "C" fn xilinx_dpdma_chan_enable(chan: *mut xilinx_dpdma_chan) {
    static void xilinx_dpdma_chan_enable(struct xilinx_dpdma_chan *chan)
    {
    u32 reg;
    reg = (XILINX_DPDMA_INTR_CHAN_MASK << chan.id)
    | XILINX_DPDMA_INTR_GLOBAL_MASK;
    dpdma_write(chan.xdev.reg, XILINX_DPDMA_IEN, reg);
    reg = (XILINX_DPDMA_EINTR_CHAN_ERR_MASK << chan.id)
    | XILINX_DPDMA_INTR_GLOBAL_ERR;
    dpdma_write(chan.xdev.reg, XILINX_DPDMA_EIEN, reg);
    reg = XILINX_DPDMA_CH_CNTL_ENABLE
    | FIELD_PREP(XILINX_DPDMA_CH_CNTL_QOS_DSCR_WR_MASK,
    XILINX_DPDMA_CH_CNTL_QOS_VID_CLASS)
    | FIELD_PREP(XILINX_DPDMA_CH_CNTL_QOS_DSCR_RD_MASK,
    XILINX_DPDMA_CH_CNTL_QOS_VID_CLASS)
    | FIELD_PREP(XILINX_DPDMA_CH_CNTL_QOS_DATA_RD_MASK,
    XILINX_DPDMA_CH_CNTL_QOS_VID_CLASS);
    dpdma_set(chan.reg, XILINX_DPDMA_CH_CNTL, reg);
    }
//
// xilinx_dpdma_chan_disable - Disable the channel
// @chan: DPDMA channel
//
// Disable the channel and its interrupts.
//
#[no_mangle]
unsafe extern "C" fn xilinx_dpdma_chan_disable(chan: *mut xilinx_dpdma_chan) {
    static void xilinx_dpdma_chan_disable(struct xilinx_dpdma_chan *chan)
    {
    u32 reg;
    reg = XILINX_DPDMA_INTR_CHAN_MASK << chan.id;
    dpdma_write(chan.xdev.reg, XILINX_DPDMA_IEN, reg);
    reg = XILINX_DPDMA_EINTR_CHAN_ERR_MASK << chan.id;
    dpdma_write(chan.xdev.reg, XILINX_DPDMA_EIEN, reg);
    dpdma_clr(chan.reg, XILINX_DPDMA_CH_CNTL, XILINX_DPDMA_CH_CNTL_ENABLE);
    }
//
// xilinx_dpdma_chan_pause - Pause the channel
// @chan: DPDMA channel
//
// Pause the channel.
//
#[no_mangle]
unsafe extern "C" fn xilinx_dpdma_chan_pause(chan: *mut xilinx_dpdma_chan) {
    static void xilinx_dpdma_chan_pause(struct xilinx_dpdma_chan *chan)
    {
    dpdma_set(chan.reg, XILINX_DPDMA_CH_CNTL, XILINX_DPDMA_CH_CNTL_PAUSE);
    }
//
// xilinx_dpdma_chan_unpause - Unpause the channel
// @chan: DPDMA channel
//
// Unpause the channel.
//
#[no_mangle]
unsafe extern "C" fn xilinx_dpdma_chan_unpause(chan: *mut xilinx_dpdma_chan) {
    static void xilinx_dpdma_chan_unpause(struct xilinx_dpdma_chan *chan)
    {
    dpdma_clr(chan.reg, XILINX_DPDMA_CH_CNTL, XILINX_DPDMA_CH_CNTL_PAUSE);
    }
#[no_mangle]
unsafe extern "C" fn xilinx_dpdma_chan_video_group_ready(chan: *mut xilinx_dpdma_chan) -> u32 {
    static u32 xilinx_dpdma_chan_video_group_ready(struct xilinx_dpdma_chan *chan)
    {
    struct xilinx_dpdma_device *xdev = chan.xdev;
    let mut channels: u32 = 0;
    unsigned int i;
    for (i = ZYNQMP_DPDMA_VIDEO0; i <= ZYNQMP_DPDMA_VIDEO2; i++) {
    if (xdev.chan[i].video_group && !xdev.chan[i].running)
    return 0;
    if (xdev.chan[i].video_group)
    channels |= BIT(i);
    }
    return channels;
    }
//
// xilinx_dpdma_chan_queue_transfer - Queue the next transfer
// @chan: DPDMA channel
//
// Queue the next descriptor, if any, to the hardware. If the channel is
// stopped, start it first. Otherwise retrigger it with the next descriptor.
//
#[no_mangle]
unsafe extern "C" fn xilinx_dpdma_chan_queue_transfer(chan: *mut xilinx_dpdma_chan) {
    static void xilinx_dpdma_chan_queue_transfer(struct xilinx_dpdma_chan *chan)
    {
    struct xilinx_dpdma_device *xdev = chan.xdev;
    struct xilinx_dpdma_sw_desc *sw_desc;
    struct xilinx_dpdma_tx_desc *desc;
    struct virt_dma_desc *vdesc;
    u32 reg, channels;
    bool first_frame;
    lockdep_assert_held(&chan.lock);
    if (chan.desc.pending)
    return;
    if (!chan.running) {
    xilinx_dpdma_chan_unpause(chan);
    xilinx_dpdma_chan_enable(chan);
    chan.first_frame = true;
    chan.running = true;
    }
    vdesc = vchan_next_desc(&chan.vchan);
    if (!vdesc)
    return;
    desc = to_dpdma_tx_desc(vdesc);
    chan.desc.pending = desc;
    list_del(&desc.vdesc.node);
//
// Assign the cookie to descriptors in this transaction. Only 16 bit
// will be used, but it should be enough.
//
    list_for_each_entry(sw_desc, &desc.descriptors, node)
    sw_desc.hw.desc_id = desc.vdesc.tx.cookie
    & XILINX_DPDMA_CH_DESC_ID_MASK;
    sw_desc = list_first_entry(&desc.descriptors,
    struct xilinx_dpdma_sw_desc, node);
    dpdma_write(chan.reg, XILINX_DPDMA_CH_DESC_START_ADDR,
    lower_32_bits(sw_desc.dma_addr));
    if (xdev.ext_addr)
    dpdma_write(chan.reg, XILINX_DPDMA_CH_DESC_START_ADDRE,
    FIELD_PREP(XILINX_DPDMA_CH_DESC_START_ADDRE_MASK,
    upper_32_bits(sw_desc.dma_addr)));
    first_frame = chan.first_frame;
    chan.first_frame = false;
    if (chan.video_group) {
    channels = xilinx_dpdma_chan_video_group_ready(chan);
//
// Trigger the transfer only when all channels in the group are
// ready.
//
    if (!channels)
    return;
    } else {
    channels = BIT(chan.id);
    }
    if (first_frame)
    reg = XILINX_DPDMA_GBL_TRIG_MASK(channels);
    else
    reg = XILINX_DPDMA_GBL_RETRIG_MASK(channels);
    dpdma_write(xdev.reg, XILINX_DPDMA_GBL, reg);
    }
//
// xilinx_dpdma_chan_ostand - Number of outstanding transactions
// @chan: DPDMA channel
//
// Read and return the number of outstanding transactions from register.
//
// Return: Number of outstanding transactions from the status register.
//
#[no_mangle]
unsafe extern "C" fn xilinx_dpdma_chan_ostand(chan: *mut xilinx_dpdma_chan) -> u32 {
    static u32 xilinx_dpdma_chan_ostand(struct xilinx_dpdma_chan *chan)
    {
    return FIELD_GET(XILINX_DPDMA_CH_STATUS_OTRAN_CNT_MASK,
    dpdma_read(chan.reg, XILINX_DPDMA_CH_STATUS));
    }
//
// xilinx_dpdma_chan_notify_no_ostand - Notify no outstanding transaction event
// @chan: DPDMA channel
//
// Notify waiters for no outstanding event, so waiters can stop the channel
// safely. This function is supposed to be called when 'no outstanding'
// interrupt is generated. The 'no outstanding' interrupt is disabled and
// should be re-enabled when this event is handled. If the channel status
// register still shows some number of outstanding transactions, the interrupt
// remains enabled.
//
// Return: 0 on success. On failure, -EWOULDBLOCK if there's still outstanding
// transaction(s).
//
#[no_mangle]
unsafe extern "C" fn xilinx_dpdma_chan_notify_no_ostand(chan: *mut xilinx_dpdma_chan) -> c_int {
    static int xilinx_dpdma_chan_notify_no_ostand(struct xilinx_dpdma_chan *chan)
    {
    u32 cnt;
    cnt = xilinx_dpdma_chan_ostand(chan);
    if (cnt) {
    dev_dbg(chan.xdev.dev,
    "chan%u: %d outstanding transactions\n",
    chan.id, cnt);
    return -EWOULDBLOCK;
    }
// Disable 'no outstanding' interrupt
    dpdma_write(chan.xdev.reg, XILINX_DPDMA_IDS,
    XILINX_DPDMA_INTR_NO_OSTAND(chan.id));
    wake_up(&chan.wait_to_stop);
    return 0;
    }
//
// xilinx_dpdma_chan_wait_no_ostand - Wait for the no outstanding irq
// @chan: DPDMA channel
//
// Wait for the no outstanding transaction interrupt. This functions can sleep
// for 50ms.
//
// Return: 0 on success. On failure, -ETIMEOUT for time out, or the error code
// from wait_event_interruptible_timeout().
//
#[no_mangle]
unsafe extern "C" fn xilinx_dpdma_chan_wait_no_ostand(chan: *mut xilinx_dpdma_chan) -> c_int {
    static int xilinx_dpdma_chan_wait_no_ostand(struct xilinx_dpdma_chan *chan)
    {
    int ret;
// Wait for a no outstanding transaction interrupt upto 50msec
    ret = wait_event_interruptible_timeout(chan.wait_to_stop,
    !xilinx_dpdma_chan_ostand(chan),
    msecs_to_jiffies(50));
    if (ret > 0) {
    dpdma_write(chan.xdev.reg, XILINX_DPDMA_IEN,
    XILINX_DPDMA_INTR_NO_OSTAND(chan.id));
    return 0;
    }
    dev_err(chan.xdev.dev, "chan%u: not ready to stop: %d trans\n",
    chan.id, xilinx_dpdma_chan_ostand(chan));
    if (ret == 0)
    return -ETIMEDOUT;
    return ret;
    }
//
// xilinx_dpdma_chan_poll_no_ostand - Poll the outstanding transaction status
// @chan: DPDMA channel
//
// Poll the outstanding transaction status, and return when there's no
// outstanding transaction. This functions can be used in the interrupt context
// or where the atomicity is required. Calling thread may wait more than 50ms.
//
// Return: 0 on success, or -ETIMEDOUT.
//
#[no_mangle]
unsafe extern "C" fn xilinx_dpdma_chan_poll_no_ostand(chan: *mut xilinx_dpdma_chan) -> c_int {
    static int xilinx_dpdma_chan_poll_no_ostand(struct xilinx_dpdma_chan *chan)
    {
    u32 cnt, loop = 50000;
// Poll at least for 50ms (20 fps).
    do {
    cnt = xilinx_dpdma_chan_ostand(chan);
    udelay(1);
    } while (loop-- > 0 && cnt);
    if (loop) {
    dpdma_write(chan.xdev.reg, XILINX_DPDMA_IEN,
    XILINX_DPDMA_INTR_NO_OSTAND(chan.id));
    return 0;
    }
    dev_err(chan.xdev.dev, "chan%u: not ready to stop: %d trans\n",
    chan.id, xilinx_dpdma_chan_ostand(chan));
    return -ETIMEDOUT;
    }
//
// xilinx_dpdma_chan_stop - Stop the channel
// @chan: DPDMA channel
//
// Stop a previously paused channel by first waiting for completion of all
// outstanding transaction and then disabling the channel.
//
// Return: 0 on success, or -ETIMEDOUT if the channel failed to stop.
//
#[no_mangle]
unsafe extern "C" fn xilinx_dpdma_chan_stop(chan: *mut xilinx_dpdma_chan) -> c_int {
    static int xilinx_dpdma_chan_stop(struct xilinx_dpdma_chan *chan)
    {
    unsigned long flags;
    int ret;
    ret = xilinx_dpdma_chan_wait_no_ostand(chan);
    if (ret)
    return ret;
    spin_lock_irqsave(&chan.lock, flags);
    xilinx_dpdma_chan_disable(chan);
    chan.running = false;
    spin_unlock_irqrestore(&chan.lock, flags);
    return 0;
    }
//
// xilinx_dpdma_chan_done_irq - Handle hardware descriptor completion
// @chan: DPDMA channel
//
// Handle completion of the currently active descriptor (@chan->desc.active). As
// we currently support cyclic transfers only, this just invokes the cyclic
// callback. The descriptor will be completed at the VSYNC interrupt when a new
// descriptor replaces it.
//
#[no_mangle]
unsafe extern "C" fn xilinx_dpdma_chan_done_irq(chan: *mut xilinx_dpdma_chan) {
    static void xilinx_dpdma_chan_done_irq(struct xilinx_dpdma_chan *chan)
    {
    struct xilinx_dpdma_tx_desc *active;
    spin_lock(&chan.lock);
    xilinx_dpdma_debugfs_desc_done_irq(chan);
    active = chan.desc.active;
    if (active)
    vchan_cyclic_callback(&active.vdesc);
    else
    dev_warn(chan.xdev.dev,
    "chan%u: DONE IRQ with no active descriptor!\n",
    chan.id);
    spin_unlock(&chan.lock);
    }
//
// xilinx_dpdma_chan_vsync_irq - Handle hardware descriptor scheduling
// @chan: DPDMA channel
//
// At VSYNC the active descriptor may have been replaced by the pending
// descriptor. Detect this through the DESC_ID and perform appropriate
// bookkeeping.
//
#[no_mangle]
unsafe extern "C" fn xilinx_dpdma_chan_vsync_irq(chan: *mut xilinx_dpdma_chan) {
    static void xilinx_dpdma_chan_vsync_irq(struct  xilinx_dpdma_chan *chan)
    {
    struct xilinx_dpdma_tx_desc *pending;
    struct xilinx_dpdma_sw_desc *sw_desc;
    u32 desc_id;
    spin_lock(&chan.lock);
    pending = chan.desc.pending;
    if (!chan.running || !pending)
    goto out;
    desc_id = dpdma_read(chan.reg, XILINX_DPDMA_CH_DESC_ID)
    & XILINX_DPDMA_CH_DESC_ID_MASK;
// If the retrigger raced with vsync, retry at the next frame.
    sw_desc = list_first_entry(&pending.descriptors,
    struct xilinx_dpdma_sw_desc, node);
    if (sw_desc.hw.desc_id != desc_id) {
    dev_dbg(chan.xdev.dev,
    "chan%u: vsync race lost (%u != %u), retrying\n",
    chan.id, sw_desc.hw.desc_id, desc_id);
    goto out;
    }
//
// Complete the active descriptor, if any, promote the pending
// descriptor to active, and queue the next transfer, if any.
//
    spin_lock(&chan.vchan.lock);
    if (chan.desc.active)
    vchan_cookie_complete(&chan.desc.active.vdesc);
    chan.desc.active = pending;
    chan.desc.pending = core::ptr::null_mut();
    xilinx_dpdma_chan_queue_transfer(chan);
    spin_unlock(&chan.vchan.lock);
    out:
    spin_unlock(&chan.lock);
    }
//
// xilinx_dpdma_chan_err - Detect any channel error
// @chan: DPDMA channel
// @isr: masked Interrupt Status Register
// @eisr: Error Interrupt Status Register
//
// Return: true if any channel error occurs, or false otherwise.
//
    static bool
    xilinx_dpdma_chan_err(struct xilinx_dpdma_chan *chan, u32 isr, u32 eisr)
    {
    if (!chan)
    return false;
    if (chan.running &&
    ((isr & (XILINX_DPDMA_INTR_CHAN_ERR_MASK << chan.id)) ||
    (eisr & (XILINX_DPDMA_EINTR_CHAN_ERR_MASK << chan.id))))
    return true;
    return false;
    }
//
// xilinx_dpdma_chan_handle_err - DPDMA channel error handling
// @chan: DPDMA channel
//
// This function is called when any channel error or any global error occurs.
// The function disables the paused channel by errors and determines
// if the current active descriptor can be rescheduled depending on
// the descriptor status.
//
#[no_mangle]
unsafe extern "C" fn xilinx_dpdma_chan_handle_err(chan: *mut xilinx_dpdma_chan) {
    static void xilinx_dpdma_chan_handle_err(struct xilinx_dpdma_chan *chan)
    {
    struct xilinx_dpdma_device *xdev = chan.xdev;
    struct xilinx_dpdma_tx_desc *active;
    unsigned long flags;
    spin_lock_irqsave(&chan.lock, flags);
    dev_dbg(xdev.dev, "chan%u: cur desc addr = 0x%04x%08x\n",
    chan.id,
    dpdma_read(chan.reg, XILINX_DPDMA_CH_DESC_START_ADDRE),
    dpdma_read(chan.reg, XILINX_DPDMA_CH_DESC_START_ADDR));
    dev_dbg(xdev.dev, "chan%u: cur payload addr = 0x%04x%08x\n",
    chan.id,
    dpdma_read(chan.reg, XILINX_DPDMA_CH_PYLD_CUR_ADDRE),
    dpdma_read(chan.reg, XILINX_DPDMA_CH_PYLD_CUR_ADDR));
    xilinx_dpdma_chan_disable(chan);
    chan.running = false;
    if (!chan.desc.active)
    goto out_unlock;
    active = chan.desc.active;
    chan.desc.active = core::ptr::null_mut();
    xilinx_dpdma_chan_dump_tx_desc(chan, active);
    if (active.error)
    dev_dbg(xdev.dev, "chan%u: repeated error on desc\n",
    chan.id);
// Reschedule if there's no new descriptor
    if (!chan.desc.pending &&
    list_empty(&chan.vchan.desc_issued)) {
    active.error = true;
    list_add_tail(&active.vdesc.node,
    &chan.vchan.desc_issued);
    } else {
    xilinx_dpdma_chan_free_tx_desc(&active.vdesc);
    }
    out_unlock:
    spin_unlock_irqrestore(&chan.lock, flags);
    }
// -----------------------------------------------------------------------------
// DMA Engine Operations
//
    static struct dma_async_tx_descriptor *
    xilinx_dpdma_prep_dma_cyclic(struct dma_chan *dchan, dma_addr_t buf_addr,
    size_t buf_len, size_t period_len,
    enum dma_transfer_direction direction,
    unsigned long flags)
    {
    struct xilinx_dpdma_chan *chan = to_xilinx_chan(dchan);
    if (direction != DMA_MEM_TO_DEV)
    return core::ptr::null_mut();
    if (buf_len % period_len)
    return core::ptr::null_mut();
    return xilinx_dpdma_chan_prep_cyclic(chan, buf_addr, buf_len,
    period_len, flags);
    }
    static struct dma_async_tx_descriptor *
    xilinx_dpdma_prep_interleaved_dma(struct dma_chan *dchan,
    struct dma_interleaved_template *xt,
    unsigned long flags)
    {
    struct xilinx_dpdma_chan *chan = to_xilinx_chan(dchan);
    struct xilinx_dpdma_tx_desc *desc;
    if (xt.dir != DMA_MEM_TO_DEV)
    return core::ptr::null_mut();
    if (!xt.numf || !xt.sgl[0].size)
    return core::ptr::null_mut();
    if (!(flags & DMA_PREP_REPEAT) || !(flags & DMA_PREP_LOAD_EOT))
    return core::ptr::null_mut();
    desc = xilinx_dpdma_chan_prep_interleaved_dma(chan, xt);
    if (!desc)
    return core::ptr::null_mut();
    vchan_tx_prep(&chan.vchan, &desc.vdesc, flags | DMA_CTRL_ACK);
    return &desc.vdesc.tx;
    }
//
// xilinx_dpdma_alloc_chan_resources - Allocate resources for the channel
// @dchan: DMA channel
//
// Allocate a descriptor pool for the channel.
//
// Return: 0 on success, or -ENOMEM if failed to allocate a pool.
//
#[no_mangle]
unsafe extern "C" fn xilinx_dpdma_alloc_chan_resources(dchan: *mut dma_chan) -> c_int {
    static int xilinx_dpdma_alloc_chan_resources(struct dma_chan *dchan)
    {
    struct xilinx_dpdma_chan *chan = to_xilinx_chan(dchan);
    let mut align: usize = __alignof__(struct xilinx_dpdma_sw_desc);
    chan.desc_pool = dma_pool_create(dev_name(chan.xdev.dev),
    chan.xdev.dev,
    sizeof(struct xilinx_dpdma_sw_desc),
    align, 0);
    if (!chan.desc_pool) {
    dev_err(chan.xdev.dev,
    "chan%u: failed to allocate a descriptor pool\n",
    chan.id);
    return -ENOMEM;
    }
    return 0;
    }
//
// xilinx_dpdma_free_chan_resources - Free all resources for the channel
// @dchan: DMA channel
//
// Free resources associated with the virtual DMA channel, and destroy the
// descriptor pool.
//
#[no_mangle]
unsafe extern "C" fn xilinx_dpdma_free_chan_resources(dchan: *mut dma_chan) {
    static void xilinx_dpdma_free_chan_resources(struct dma_chan *dchan)
    {
    struct xilinx_dpdma_chan *chan = to_xilinx_chan(dchan);
    vchan_free_chan_resources(&chan.vchan);
    dma_pool_destroy(chan.desc_pool);
    chan.desc_pool = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn xilinx_dpdma_issue_pending(dchan: *mut dma_chan) {
    static void xilinx_dpdma_issue_pending(struct dma_chan *dchan)
    {
    struct xilinx_dpdma_chan *chan = to_xilinx_chan(dchan);
    unsigned long flags;
    spin_lock_irqsave(&chan.lock, flags);
    spin_lock(&chan.vchan.lock);
    if (vchan_issue_pending(&chan.vchan))
    xilinx_dpdma_chan_queue_transfer(chan);
    spin_unlock(&chan.vchan.lock);
    spin_unlock_irqrestore(&chan.lock, flags);
    }
    static int xilinx_dpdma_config(struct dma_chan *dchan,
    struct dma_slave_config *config)
    {
    struct xilinx_dpdma_chan *chan = to_xilinx_chan(dchan);
    struct xilinx_dpdma_peripheral_config *pconfig;
    unsigned long flags;
//
// The destination address doesn't need to be specified as the DPDMA is
// hardwired to the destination (the DP controller). The transfer
// width, burst size and port window size are thus meaningless, they're
// fixed both on the DPDMA side and on the DP controller side.
//
// Use the peripheral_config to indicate that the channel is part
// of a video group. This requires matching use of the custom
// structure in each driver.
//
    pconfig = config.peripheral_config;
    if (WARN_ON(pconfig && config.peripheral_size != sizeof(*pconfig)))
    return -EINVAL;
    spin_lock_irqsave(&chan.lock, flags);
    if (chan.id <= ZYNQMP_DPDMA_VIDEO2 && pconfig)
    chan.video_group = pconfig.video_group;
    spin_unlock_irqrestore(&chan.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xilinx_dpdma_pause(dchan: *mut dma_chan) -> c_int {
    static int xilinx_dpdma_pause(struct dma_chan *dchan)
    {
    xilinx_dpdma_chan_pause(to_xilinx_chan(dchan));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xilinx_dpdma_resume(dchan: *mut dma_chan) -> c_int {
    static int xilinx_dpdma_resume(struct dma_chan *dchan)
    {
    xilinx_dpdma_chan_unpause(to_xilinx_chan(dchan));
    return 0;
    }
//
// xilinx_dpdma_terminate_all - Terminate the channel and descriptors
// @dchan: DMA channel
//
// Pause the channel without waiting for ongoing transfers to complete. Waiting
// for completion is performed by xilinx_dpdma_synchronize() that will disable
// the channel to complete the stop.
//
// All the descriptors associated with the channel that are guaranteed not to
// be touched by the hardware. The pending and active descriptor are not
// touched, and will be freed either upon completion, or by
// xilinx_dpdma_synchronize().
//
// Return: 0 on success, or -ETIMEDOUT if the channel failed to stop.
//
#[no_mangle]
unsafe extern "C" fn xilinx_dpdma_terminate_all(dchan: *mut dma_chan) -> c_int {
    static int xilinx_dpdma_terminate_all(struct dma_chan *dchan)
    {
    struct xilinx_dpdma_chan *chan = to_xilinx_chan(dchan);
    struct xilinx_dpdma_device *xdev = chan.xdev;
    LIST_HEAD(descriptors);
    unsigned long flags;
    unsigned int i;
// Pause the channel (including the whole video group if applicable).
    if (chan.video_group) {
    for (i = ZYNQMP_DPDMA_VIDEO0; i <= ZYNQMP_DPDMA_VIDEO2; i++) {
    if (xdev.chan[i].video_group &&
    xdev.chan[i].running) {
    xilinx_dpdma_chan_pause(xdev.chan[i]);
    xdev.chan[i].video_group = false;
    }
    }
    } else {
    xilinx_dpdma_chan_pause(chan);
    }
// Gather all the descriptors we can free and free them.
    spin_lock_irqsave(&chan.vchan.lock, flags);
    vchan_get_all_descriptors(&chan.vchan, &descriptors);
    spin_unlock_irqrestore(&chan.vchan.lock, flags);
    vchan_dma_desc_free_list(&chan.vchan, &descriptors);
    return 0;
    }
//
// xilinx_dpdma_synchronize - Synchronize callback execution
// @dchan: DMA channel
//
// Synchronizing callback execution ensures that all previously issued
// transfers have completed and all associated callbacks have been called and
// have returned.
//
// This function waits for the DMA channel to stop. It assumes it has been
// paused by a previous call to dmaengine_terminate_async(), and that no new
// pending descriptors have been issued with dma_async_issue_pending(). The
// behaviour is undefined otherwise.
//
#[no_mangle]
unsafe extern "C" fn xilinx_dpdma_synchronize(dchan: *mut dma_chan) {
    static void xilinx_dpdma_synchronize(struct dma_chan *dchan)
    {
    struct xilinx_dpdma_chan *chan = to_xilinx_chan(dchan);
    unsigned long flags;
    xilinx_dpdma_chan_stop(chan);
    spin_lock_irqsave(&chan.vchan.lock, flags);
    if (chan.desc.pending) {
    vchan_terminate_vdesc(&chan.desc.pending.vdesc);
    chan.desc.pending = core::ptr::null_mut();
    }
    if (chan.desc.active) {
    vchan_terminate_vdesc(&chan.desc.active.vdesc);
    chan.desc.active = core::ptr::null_mut();
    }
    spin_unlock_irqrestore(&chan.vchan.lock, flags);
    vchan_synchronize(&chan.vchan);
    }
// -----------------------------------------------------------------------------
// Interrupt and Tasklet Handling
//
// xilinx_dpdma_err - Detect any global error
// @isr: Interrupt Status Register
// @eisr: Error Interrupt Status Register
//
// Return: True if any global error occurs, or false otherwise.
//
#[no_mangle]
unsafe extern "C" fn xilinx_dpdma_err(isr: u32, eisr: u32) -> bool {
    static bool xilinx_dpdma_err(u32 isr, u32 eisr)
    {
    if (isr & XILINX_DPDMA_INTR_GLOBAL_ERR ||
    eisr & XILINX_DPDMA_EINTR_GLOBAL_ERR)
    return true;
    return false;
    }
//
// xilinx_dpdma_handle_err_irq - Handle DPDMA error interrupt
// @xdev: DPDMA device
// @isr: masked Interrupt Status Register
// @eisr: Error Interrupt Status Register
//
// Handle if any error occurs based on @isr and @eisr. This function disables
// corresponding error interrupts, and those should be re-enabled once handling
// is done.
//
    static void xilinx_dpdma_handle_err_irq(struct xilinx_dpdma_device *xdev,
    u32 isr, u32 eisr)
    {
    let mut err: bool = xilinx_dpdma_err(isr, eisr);
    unsigned int i;
    dev_dbg_ratelimited(xdev.dev,
    "error irq: isr = 0x%08x, eisr = 0x%08x\n",
    isr, eisr);
// Disable channel error interrupts until errors are handled.
    dpdma_write(xdev.reg, XILINX_DPDMA_IDS,
    isr & ~XILINX_DPDMA_INTR_GLOBAL_ERR);
    dpdma_write(xdev.reg, XILINX_DPDMA_EIDS,
    eisr & ~XILINX_DPDMA_EINTR_GLOBAL_ERR);
    for (i = 0; i < ARRAY_SIZE(xdev.chan); i++)
    if (err || xilinx_dpdma_chan_err(xdev.chan[i], isr, eisr))
    tasklet_schedule(&xdev.chan[i].err_task);
    }
//
// xilinx_dpdma_enable_irq - Enable interrupts
// @xdev: DPDMA device
//
// Enable interrupts.
//
#[no_mangle]
unsafe extern "C" fn xilinx_dpdma_enable_irq(xdev: *mut xilinx_dpdma_device) {
    static void xilinx_dpdma_enable_irq(struct xilinx_dpdma_device *xdev)
    {
    dpdma_write(xdev.reg, XILINX_DPDMA_IEN, XILINX_DPDMA_INTR_ALL);
    dpdma_write(xdev.reg, XILINX_DPDMA_EIEN, XILINX_DPDMA_EINTR_ALL);
    }
//
// xilinx_dpdma_disable_irq - Disable interrupts
// @xdev: DPDMA device
//
// Disable interrupts.
//
#[no_mangle]
unsafe extern "C" fn xilinx_dpdma_disable_irq(xdev: *mut xilinx_dpdma_device) {
    static void xilinx_dpdma_disable_irq(struct xilinx_dpdma_device *xdev)
    {
    dpdma_write(xdev.reg, XILINX_DPDMA_IDS, XILINX_DPDMA_INTR_ALL);
    dpdma_write(xdev.reg, XILINX_DPDMA_EIDS, XILINX_DPDMA_EINTR_ALL);
    }
//
// xilinx_dpdma_chan_err_task - Per channel tasklet for error handling
// @t: pointer to the tasklet associated with this handler
//
// Per channel error handling tasklet. This function waits for the outstanding
// transaction to complete and triggers error handling. After error handling,
// re-enable channel error interrupts, and restart the channel if needed.
//
#[no_mangle]
unsafe extern "C" fn xilinx_dpdma_chan_err_task(t: *mut tasklet_struct) {
    static void xilinx_dpdma_chan_err_task(struct tasklet_struct *t)
    {
    struct xilinx_dpdma_chan *chan = from_tasklet(chan, t, err_task);
    struct xilinx_dpdma_device *xdev = chan.xdev;
    unsigned long flags;
// Proceed error handling even when polling fails.
    xilinx_dpdma_chan_poll_no_ostand(chan);
    xilinx_dpdma_chan_handle_err(chan);
    dpdma_write(xdev.reg, XILINX_DPDMA_IEN,
    XILINX_DPDMA_INTR_CHAN_ERR_MASK << chan.id);
    dpdma_write(xdev.reg, XILINX_DPDMA_EIEN,
    XILINX_DPDMA_EINTR_CHAN_ERR_MASK << chan.id);
    spin_lock_irqsave(&chan.lock, flags);
    spin_lock(&chan.vchan.lock);
    xilinx_dpdma_chan_queue_transfer(chan);
    spin_unlock(&chan.vchan.lock);
    spin_unlock_irqrestore(&chan.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn xilinx_dpdma_irq_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t xilinx_dpdma_irq_handler(int irq, void *data)
    {
    struct xilinx_dpdma_device *xdev = data;
    unsigned long mask;
    unsigned int i;
    u32 status;
    u32 error;
    status = dpdma_read(xdev.reg, XILINX_DPDMA_ISR);
    error = dpdma_read(xdev.reg, XILINX_DPDMA_EISR);
    if (!status && !error)
    return IRQ_NONE;
    dpdma_write(xdev.reg, XILINX_DPDMA_ISR, status);
    dpdma_write(xdev.reg, XILINX_DPDMA_EISR, error);
    if (status & XILINX_DPDMA_INTR_VSYNC) {
//
// There's a single VSYNC interrupt that needs to be processed
// by each running channel to update the active descriptor.
//
    for (i = 0; i < ARRAY_SIZE(xdev.chan); i++) {
    struct xilinx_dpdma_chan *chan = xdev.chan[i];
    if (chan)
    xilinx_dpdma_chan_vsync_irq(chan);
    }
    }
    mask = FIELD_GET(XILINX_DPDMA_INTR_DESC_DONE_MASK, status);
    if (mask) {
    for_each_set_bit(i, &mask, ARRAY_SIZE(xdev.chan))
    xilinx_dpdma_chan_done_irq(xdev.chan[i]);
    }
    mask = FIELD_GET(XILINX_DPDMA_INTR_NO_OSTAND_MASK, status);
    if (mask) {
    for_each_set_bit(i, &mask, ARRAY_SIZE(xdev.chan))
    xilinx_dpdma_chan_notify_no_ostand(xdev.chan[i]);
    }
    mask = status & XILINX_DPDMA_INTR_ERR_ALL;
    if (mask || error)
    xilinx_dpdma_handle_err_irq(xdev, mask, error);
    return IRQ_HANDLED;
    }
// -----------------------------------------------------------------------------
// Initialization & Cleanup
//
    static int xilinx_dpdma_chan_init(struct xilinx_dpdma_device *xdev,
    unsigned int chan_id)
    {
    struct xilinx_dpdma_chan *chan;
    chan = devm_kzalloc(xdev.dev, sizeof(*chan), GFP_KERNEL);
    if (!chan)
    return -ENOMEM;
    chan.id = chan_id;
    chan.reg = xdev.reg + XILINX_DPDMA_CH_BASE
    + XILINX_DPDMA_CH_OFFSET * chan.id;
    chan.running = false;
    chan.xdev = xdev;
    spin_lock_init(&chan.lock);
    init_waitqueue_head(&chan.wait_to_stop);
    tasklet_setup(&chan.err_task, xilinx_dpdma_chan_err_task);
    chan.vchan.desc_free = xilinx_dpdma_chan_free_tx_desc;
    vchan_init(&chan.vchan, &xdev.common);
    xdev.chan[chan.id] = chan;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xilinx_dpdma_chan_remove(chan: *mut xilinx_dpdma_chan) {
    static void xilinx_dpdma_chan_remove(struct xilinx_dpdma_chan *chan)
    {
    if (!chan)
    return;
    tasklet_kill(&chan.err_task);
    list_del(&chan.vchan.chan.device_node);
    }
    static struct dma_chan *of_dma_xilinx_xlate(struct of_phandle_args *dma_spec,
    struct of_dma *ofdma)
    {
    struct xilinx_dpdma_device *xdev = ofdma.of_dma_data;
    let mut chan_id: u32 = dma_spec.args[0];
    if (chan_id >= ARRAY_SIZE(xdev.chan))
    return core::ptr::null_mut();
    if (!xdev.chan[chan_id])
    return core::ptr::null_mut();
    return dma_get_slave_channel(&xdev.chan[chan_id].vchan.chan);
    }
#[no_mangle]
unsafe extern "C" fn dpdma_hw_init(xdev: *mut xilinx_dpdma_device) {
    static void dpdma_hw_init(struct xilinx_dpdma_device *xdev)
    {
    unsigned int i;
    void __iomem *reg;
// Disable all interrupts
    xilinx_dpdma_disable_irq(xdev);
// Stop all channels
    for (i = 0; i < ARRAY_SIZE(xdev.chan); i++) {
    reg = xdev.reg + XILINX_DPDMA_CH_BASE
    + XILINX_DPDMA_CH_OFFSET * i;
    dpdma_clr(reg, XILINX_DPDMA_CH_CNTL, XILINX_DPDMA_CH_CNTL_ENABLE);
    }
// Clear the interrupt status registers
    dpdma_write(xdev.reg, XILINX_DPDMA_ISR, XILINX_DPDMA_INTR_ALL);
    dpdma_write(xdev.reg, XILINX_DPDMA_EISR, XILINX_DPDMA_EINTR_ALL);
    }
#[no_mangle]
unsafe extern "C" fn xilinx_dpdma_probe(pdev: *mut platform_device) -> c_int {
    static int xilinx_dpdma_probe(struct platform_device *pdev)
    {
    struct xilinx_dpdma_device *xdev;
    struct dma_device *ddev;
    unsigned int i;
    int ret;
    xdev = devm_kzalloc(&pdev.dev, sizeof(*xdev), GFP_KERNEL);
    if (!xdev)
    return -ENOMEM;
    xdev.dev = &pdev.dev;
    xdev.ext_addr = sizeof(dma_addr_t) > 4;
    INIT_LIST_HEAD(&xdev.common.channels);
    platform_set_drvdata(pdev, xdev);
    xdev.axi_clk = devm_clk_get(xdev.dev, "axi_clk");
    if (IS_ERR(xdev.axi_clk))
    return PTR_ERR(xdev.axi_clk);
    xdev.reg = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(xdev.reg))
    return PTR_ERR(xdev.reg);
    dpdma_hw_init(xdev);
    xdev.irq = platform_get_irq(pdev, 0);
    if (xdev.irq < 0)
    return xdev.irq;
    ret = request_irq(xdev.irq, xilinx_dpdma_irq_handler, IRQF_SHARED,
    dev_name(xdev.dev), xdev);
    if (ret) {
    dev_err(xdev.dev, "failed to request IRQ\n");
    return ret;
    }
    ddev = &xdev.common;
    ddev.dev = &pdev.dev;
    dma_cap_set(DMA_SLAVE, ddev.cap_mask);
    dma_cap_set(DMA_PRIVATE, ddev.cap_mask);
    dma_cap_set(DMA_CYCLIC, ddev.cap_mask);
    dma_cap_set(DMA_INTERLEAVE, ddev.cap_mask);
    dma_cap_set(DMA_REPEAT, ddev.cap_mask);
    dma_cap_set(DMA_LOAD_EOT, ddev.cap_mask);
    ddev.copy_align = fls(XILINX_DPDMA_ALIGN_BYTES - 1);
    ddev.device_alloc_chan_resources = xilinx_dpdma_alloc_chan_resources;
    ddev.device_free_chan_resources = xilinx_dpdma_free_chan_resources;
    ddev.device_prep_dma_cyclic = xilinx_dpdma_prep_dma_cyclic;
    ddev.device_prep_interleaved_dma = xilinx_dpdma_prep_interleaved_dma;
// TODO: Can we achieve better granularity ?
    ddev.device_tx_status = dma_cookie_status;
    ddev.device_issue_pending = xilinx_dpdma_issue_pending;
    ddev.device_config = xilinx_dpdma_config;
    ddev.device_pause = xilinx_dpdma_pause;
    ddev.device_resume = xilinx_dpdma_resume;
    ddev.device_terminate_all = xilinx_dpdma_terminate_all;
    ddev.device_synchronize = xilinx_dpdma_synchronize;
    ddev.src_addr_widths = BIT(DMA_SLAVE_BUSWIDTH_UNDEFINED);
    ddev.directions = BIT(DMA_MEM_TO_DEV);
    ddev.residue_granularity = DMA_RESIDUE_GRANULARITY_DESCRIPTOR;
    for (i = 0; i < ARRAY_SIZE(xdev.chan); ++i) {
    ret = xilinx_dpdma_chan_init(xdev, i);
    if (ret < 0) {
    dev_err(xdev.dev, "failed to initialize channel %u\n",
    i);
    goto error;
    }
    }
    ret = clk_prepare_enable(xdev.axi_clk);
    if (ret) {
    dev_err(xdev.dev, "failed to enable the axi clock\n");
    goto error;
    }
    ret = dma_async_device_register(ddev);
    if (ret) {
    dev_err(xdev.dev, "failed to register the dma device\n");
    goto error_dma_async;
    }
    ret = of_dma_controller_register(xdev.dev.of_node,
    of_dma_xilinx_xlate, ddev);
    if (ret) {
    dev_err(xdev.dev, "failed to register DMA to DT DMA helper\n");
    goto error_of_dma;
    }
    xilinx_dpdma_enable_irq(xdev);
    xilinx_dpdma_debugfs_init(xdev);
    dev_info(&pdev.dev, "Xilinx DPDMA engine is probed\n");
    return 0;
    error_of_dma:
    dma_async_device_unregister(ddev);
    error_dma_async:
    clk_disable_unprepare(xdev.axi_clk);
    error:
    for (i = 0; i < ARRAY_SIZE(xdev.chan); i++)
    xilinx_dpdma_chan_remove(xdev.chan[i]);
    free_irq(xdev.irq, xdev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn xilinx_dpdma_remove(pdev: *mut platform_device) {
    static void xilinx_dpdma_remove(struct platform_device *pdev)
    {
    struct xilinx_dpdma_device *xdev = platform_get_drvdata(pdev);
    unsigned int i;
// Start by disabling the IRQ to avoid races during cleanup.
    free_irq(xdev.irq, xdev);
    xilinx_dpdma_disable_irq(xdev);
    of_dma_controller_free(pdev.dev.of_node);
    dma_async_device_unregister(&xdev.common);
    clk_disable_unprepare(xdev.axi_clk);
    for (i = 0; i < ARRAY_SIZE(xdev.chan); i++)
    xilinx_dpdma_chan_remove(xdev.chan[i]);
    }
    static const struct of_device_id xilinx_dpdma_of_match[] = {
    { .compatible = "xlnx,zynqmp-dpdma",},
    { /* end of table */ },
    };
    MODULE_DEVICE_TABLE(of, xilinx_dpdma_of_match);
    static struct platform_driver xilinx_dpdma_driver = {
    .probe			= xilinx_dpdma_probe,
    .remove			= xilinx_dpdma_remove,
    .driver			= {
    .name		= "xilinx-zynqmp-dpdma",
    .of_match_table	= xilinx_dpdma_of_match,
    },
    };
    module_platform_driver(xilinx_dpdma_driver);
    MODULE_AUTHOR("Xilinx, Inc.");
    MODULE_DESCRIPTION("Xilinx ZynqMP DPDMA driver");
    MODULE_LICENSE("GPL v2");
